# Example 2: Type Confusion / Discriminator

Solana accounts are raw byte arrays. Without explicit type checking (discriminators), programs can be tricked into misinterpreting one account type as another.

---

## Vulnerability

If a program doesn't check the account type, an attacker can pass a different account type that happens to match the expected byte layout. This is often called "Type Cosplay".

### Impact Summary

| Aspect | Description |
|--------|-------------|
| **Severity** | Critical |
| **Category** | Account Validation |
| **Exploit** | Unauthorized access by spoofing higher-privileged account types |

---

## Bug

The vulnerability stems from using `AccountInfo` (which bypasses all Anchor checks) instead of typed accounts.

<details>
<summary><strong>View Vulnerable Code</strong></summary>

```rust
#[derive(Accounts)]
pub struct VulnerableAdmin<'info> {
    /// CHECK: BUG - Any account type can be passed here.
    #[account(mut)]
    pub admin_config: AccountInfo<'info>, 
    pub signer: Signer<'info>,
}

pub fn vulnerable_admin_action(ctx: Context<VulnerableAdmin>) -> Result<()> {
    // Blindly reading bytes 8-40 as "authority"
    let data = ctx.accounts.admin_config.try_borrow_data()?;
    let authority = Pubkey::try_from(&data[8..40]).map_err(|_| ErrorCode::InvalidData)?;
    
    require!(authority == ctx.accounts.signer.key(), ErrorCode::Unauthorized);
    Ok(())
}
```

</details>

> [!WARNING]
> An attacker can pass a `UserData` account. If the `owner` field in `UserData` is at the same offset as `authority` in `AdminConfig`, the program will accept the attacker's `UserData` as a valid `AdminConfig`.

---

## Fix

Use Anchor's typed `Account<'info, T>` wrapper, which automatically validates the 8-byte discriminator stored at the start of the account data.

<details open>
<summary><strong>View Secure Fix</strong></summary>

```rust
#[derive(Accounts)]
pub struct SecureAdmin<'info> {
    #[account(mut)]
    pub admin_config: Account<'info, AdminConfig>, // Validates discriminator
    pub signer: Signer<'info>,
}
```

</details>

> [!IMPORTANT]
> **Key Mitigation**: Never use `AccountInfo` or `UncheckedAccount` for accounts that have a known structure. Always use `Account<'info, T>` to leverage Anchor's automatic discriminator checks.

---

## Project Files

| File | Description |
|------|-------------|
| [Implementation](../programs/solana-security-template/src/instructions/type_confusion.rs) | Rust logic for vulnerability and fix |
| [Tests](../tests/src/test_type_confusion/mod.rs) | Exploit verification tests |

---

## References

- [Sealevel Attacks: Type Cosplay](https://github.com/coral-xyz/sealevel-attacks/tree/master/programs/3-type-cosplay)
- [Anchor Account Types](https://www.anchor-lang.com/docs/account-types)
