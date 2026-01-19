# Example 4: Closing Account Refund Theft

## Vulnerability

When closing an account, the remaining lamports must be transferred somewhere. If the destination isn't validated, an attacker can redirect the refund.

## The Bug

```rust
#[derive(Accounts)]
pub struct VulnerableClose<'info> {
    #[account(mut, has_one = authority)]
    pub data_account: Account<'info, DataAccount>,
    pub authority: Signer<'info>,
    /// CHECK: BUG - Any account can receive the refund.
    #[account(mut)]
    pub destination: AccountInfo<'info>,
}
```

The `destination` account is not validated. A malicious authority (or front-end) can specify any address to receive the lamports.

## The Fix

Use Anchor's `close` constraint to send lamports only to a validated account:

```rust
#[derive(Accounts)]
pub struct SecureClose<'info> {
    #[account(
        mut, 
        has_one = authority,
        close = authority  // Lamports go to authority only
    )]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
}
```

## Files

- [Vulnerable + Secure Implementation](../programs/solana-security-template/src/instructions/closing_account.rs)
- [Tests](../tests/src/test_closing_account/mod.rs)

## Reference

- [Sealevel Attacks: Closing Accounts](https://github.com/coral-xyz/sealevel-attacks/tree/master/programs/9-closing-accounts)
