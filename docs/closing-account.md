# Example 4: Closing Account Refund Theft

When an account is closed on Solana, its lamports must be transferred to another account. If the destination account is not validated, an attacker can redirect the refund to their own address.

---

## Vulnerability

Closing an account involves reducing its lamports to zero. The runtime then removes the account from the state. If the program allows the user to specify an unvalidated `destination` for these lamports, a malicious actor can steal the funds.

### Impact Summary

| Aspect | Description |
|--------|-------------|
| **Severity** | High |
| **Category** | Account Closure |
| **Exploit** | Theft of rent-exempt lamports during account closure |

---

## Bug

The vulnerability occurs when a program manually handles the lamport transfer and zeroing of data without verifying the recipient's identity.

<details>
<summary><strong>View Vulnerable Code</strong></summary>

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

pub fn vulnerable_close(ctx: Context<VulnerableClose>) -> Result<()> {
    let data_account = &ctx.accounts.data_account.to_account_info();
    let dest = &ctx.accounts.destination;

    // Manual transfer without destination validation
    **dest.try_borrow_mut_lamports()? += data_account.lamports();
    **data_account.try_borrow_mut_lamports()? = 0;

    Ok(())
}
```

</details>

> [!WARNING]
> A malicious authority can specify an arbitrary `destination` address. While they might be the authority for the `data_account`, the system should often enforce that the refund goes back to the original payer or a specific treasury.

---

## Fix

Use Anchor's `close` constraint. This declarative approach ensures that Anchor handles the transfer securely, typically sending the lamports to a validated signer.

<details open>
<summary><strong>View Secure Fix</strong></summary>

```rust
#[derive(Accounts)]
pub struct SecureClose<'info> {
    #[account(
        mut, 
        has_one = authority,
        close = authority  // Anchor securely handles refund to authority
    )]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
}
```

</details>

> [!IMPORTANT]
> **Key Mitigation**: Always use the `close = <target>` constraint in Anchor. This avoids manual lamport manipulation and ensures the refund destination is properly validated at the framework level.

---

## Project Files

| File | Description |
|------|-------------|
| [Implementation](../programs/solana-security-template/src/instructions/closing_account.rs) | Rust logic for vulnerability and fix |
| [Tests](../tests/src/test_closing_account/mod.rs) | Exploit verification tests |

---

## References

- [Sealevel Attacks: Closing Accounts](https://github.com/coral-xyz/sealevel-attacks/tree/master/programs/9-closing-accounts)
- [Anchor Close Constraint](https://www.anchor-lang.com/docs/constraints#close)
