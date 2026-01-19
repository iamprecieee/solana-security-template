# Example 1: Duplicate Account Aliasing

## Vulnerability

When a program accepts two mutable accounts (e.g., source and destination), an attacker can pass the **same account** for both. If the program reads both balances into local variables before writing, the second write overwrites the first.

## The Bug

```rust
pub fn vulnerable_transfer(ctx: Context<VulnerableTransfer>, amount: u64) -> Result<()> {
    // CHECK: BUG - Caching both balances means second write overwrites first if aliased
    let source_balance = ctx.accounts.source.balance;
    let dest_balance = ctx.accounts.destination.balance;

    ctx.accounts.source.balance = source_balance.saturating_sub(amount);
    ctx.accounts.destination.balance = dest_balance.saturating_add(amount);

    Ok(())
}
```

If `source == destination` with balance 100 and amount 50:
- Expected: balance stays 100 (transfer to self)
- Actual: balance becomes 150 (tokens created from nothing)

## The Fix

Add a constraint that rejects aliased accounts:

```rust
#[derive(Accounts)]
pub struct SecureTransfer<'info> {
    #[account(mut)]
    pub source: Account<'info, Wallet>,
    #[account(
        mut,
        constraint = source.key() != destination.key() @ ErrorCode::DuplicateAccount
    )]
    pub destination: Account<'info, Wallet>,
    pub authority: Signer<'info>,
}
```

Anchor validates this constraint during deserialization, before the handler runs.

## Design Decision: Reject vs Allow

We reject aliased accounts rather than allowing them as no-ops because:
1. Self-transfers are usually client bugs - failing helps catch them
2. Failing early saves compute units
3. Explicit errors are easier to debug than silent no-ops

Both approaches are secure. This is a UX choice.

## Files

- [Vulnerable + Secure Implementation](../programs/solana-security-template/src/instructions/duplicate_account.rs)
- [Tests](../tests/src/test_duplicate_account/mod.rs)

## Reference

- [Sealevel Attacks: Duplicate Mutable Accounts](https://github.com/coral-xyz/sealevel-attacks/tree/master/programs/6-duplicate-mutable-accounts)
