# Example 1: Duplicate Account Aliasing

In Solana, an attacker can pass the same account for multiple mutable parameters. If not handled, this can lead to logic errors and fund inflation.

---

## Vulnerability

When a program accepts two mutable accounts (e.g., source and destination), an attacker can pass the **same account** for both. If the program reads both balances into local variables before writing, the second write overwrites the first.

### Impact Summary

| Aspect | Description |
|--------|-------------|
| **Severity** | High |
| **Category** | Account Validation |
| **Exploit** | Balance inflation via stale data writes |

---

## Bug

The program caches initial balances and writes them back independently, unaware that they refer to the same underlying account.

<details>
<summary><strong>View Vulnerable Code</strong></summary>

```rust
pub fn vulnerable_transfer(ctx: Context<VulnerableTransfer>, amount: u64) -> Result<()> {
    // BUG: Caching both balances means second write overwrites first if aliased
    let source_balance = ctx.accounts.source.balance;
    let dest_balance = ctx.accounts.destination.balance;

    ctx.accounts.source.balance = source_balance.saturating_sub(amount);
    ctx.accounts.destination.balance = dest_balance.saturating_add(amount);

    Ok(())
}
```

</details>

> [!WARNING]
> If `source == destination` with balance 100 and amount 50:
> - **Expected**: balance remains 100 (self-transfer).
> - **Actual**: balance becomes 150 (tokens created from nothing).

---

## Fix

Use Anchor constraints to ensure account uniqueness during deserialization.

<details open>
<summary><strong>View Secure Fix</strong></summary>

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

</details>

> [!IMPORTANT]
> **Key Mitigation**: Always validate that distinct mutable accounts have unique public keys using `constraint = a.key() != b.key()`.

---

## Design Decision: Reject vs Allow

We reject aliased accounts rather than allowing them as no-ops because:
1. Self-transfers are usually client bugs - failing helps catch them
2. Failing early saves compute units
3. Explicit errors are easier to debug than silent no-ops

Both approaches are secure. This is a UX choice.

---

## Project Files

| File | Description |
|------|-------------|
| [Implementation](../programs/solana-security-template/src/instructions/duplicate_account.rs) | Rust logic for vulnerability and fix |
| [Tests](../tests/src/test_duplicate_account/mod.rs) | Exploit verification tests |

---

## References

- [Sealevel Attacks: Duplicate Mutable Accounts](https://github.com/coral-xyz/sealevel-attacks/tree/master/programs/6-duplicate-mutable-accounts)
- [Anchor Constraints Documentation](https://www.anchor-lang.com/docs/constraints)
