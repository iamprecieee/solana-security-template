# Example 3: Arbitrary CPI

Cross-Program Invocation (CPI) allows a program to call another program. If the target program ID is not validated, an attacker can substitute it with a malicious program to hijack execution.

---

## Vulnerability

When performing a CPI, the program must ensure it is calling the intended program. If the program ID is passed as an `UncheckedAccount`, an attacker can provide a malicious program that mimics the expected interface but performs unauthorized actions.

### Impact Summary

| Aspect | Description |
|--------|-------------|
| **Severity** | High |
| **Category** | CPI Security |
| **Exploit** | Malicious program execution with caller's authority |

---

## Bug

Using `UncheckedAccount` for program IDs bypasses Anchor's automatic verification, allowing any account to be treated as a program.

<details>
<summary><strong>View Vulnerable Code</strong></summary>

```rust
#[derive(Accounts)]
pub struct VulnerableCpi<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: SystemAccount<'info>,
    /// CHECK: BUG - Any program can be passed here.
    pub system_program: UncheckedAccount<'info>,
}

pub fn vulnerable_cpi(ctx: Context<VulnerableCpi>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer { from: ..., to: ... },
    );
    transfer(cpi_ctx, amount)?;
    Ok(())
}
```

</details>

> [!WARNING]
> An attacker can pass their own malicious program as `system_program`. When the caller invokes `transfer`, they are actually executing the attacker's code, which could steal funds or corrupt state.

---

## Fix

Use Anchor's `Program<'info, T>` wrapper. This ensures the account passed is a valid program and matches the expected program ID (e.g., the System Program).

<details open>
<summary><strong>View Secure Fix</strong></summary>

```rust
#[derive(Accounts)]
pub struct SecureCpi<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: SystemAccount<'info>,
    pub system_program: Program<'info, System>, // Validates program ID
}
```

</details>

> [!IMPORTANT]
> **Key Mitigation**: Always use the `Program<'info, T>` type for programs involved in CPI. This leverages Anchor's built-in validation to prevent calling arbitrary accounts.

---

## Project Files

| File | Description |
|------|-------------|
| [Implementation](../programs/solana-security-template/src/instructions/arbitrary_cpi.rs) | Rust logic for vulnerability and fix |
| [Tests](../tests/src/test_arbitrary_cpi/mod.rs) | Exploit verification tests |

---

## References

- [Sealevel Attacks: Arbitrary CPI](https://github.com/coral-xyz/sealevel-attacks/tree/master/programs/5-arbitrary-cpi)
- [Anchor CPI Documentation](https://www.anchor-lang.com/docs/cpi)
