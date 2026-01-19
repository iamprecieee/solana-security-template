# Example 2: Type Confusion / Discriminator

## Vulnerability

Solana accounts are just byte arrays. If a program doesn't check the account type (discriminator), an attacker can pass a different account type that happens to match the byte layout.

## The Bug

The vulnerability stems from using `AccountInfo` (unchecked) instead of `Account<T>` (checked).

```rust
#[derive(Accounts)]
pub struct VulnerableAdmin<'info> {
    /// CHECK: BUG - Any account type can be passed here.
    #[account(mut)]
    pub admin_config: AccountInfo<'info>, 
    pub signer: Signer<'info>,
}

pub fn vulnerable_admin_action(ctx: Context<VulnerableAdmin>) -> Result<()> {
    // Program blindly reads bytes 8-40 as "authority", trusting the account is AdminConfig
    let data = ctx.accounts.admin_config.try_borrow_data()?;
    let authority = Pubkey::try_from(&data[8..40]); 
    
    // ...
}
```

Because `AccountInfo` bypasses Anchor's discriminator check, an attacker can pass a `UserData` account. Since `UserData` has the same byte layout (Pubkey at offset 8), the program misinterprets `UserData.owner` as `AdminConfig.authority`.

## The Fix

Use Anchor's typed `Account<'info, T>` wrapper, which automatically checks the 8-byte discriminator.

```rust
pub struct SecureAdmin<'info> {
    #[account(mut)]
    pub admin_config: Account<'info, AdminConfig>, // Checks discriminator matches AdminConfig
    pub signer: Signer<'info>,
}
```

If an attacker passes a `UserData` account, Anchor rejects it because `UserData`'s discriminator doesn't match `AdminConfig`.

## Files

- [Vulnerable + Secure Implementation](../programs/solana-security-template/src/instructions/type_confusion.rs)
- [Tests](../tests/src/test_type_confusion/mod.rs)

## Reference

- [Sealevel Attacks: Type Cosplay](https://github.com/coral-xyz/sealevel-attacks/tree/master/programs/3-type-cosplay)
