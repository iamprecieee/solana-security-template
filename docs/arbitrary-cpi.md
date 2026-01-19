# Example 3: Arbitrary CPI

## Vulnerability

Cross-Program Invocation (CPI) lets one program call another. If the target program isn't validated, an attacker can substitute a malicious program.

## The Bug

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
    // Invokes whatever program is passed - attacker controls execution
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer { from: ..., to: ... },
    );
    transfer(cpi_ctx, amount)?;
    Ok(())
}
```

Using `UncheckedAccount` for the program means Anchor won't verify it's the real System Program.

## The Fix

Use `Program<'info, System>` which validates the account is the actual System Program.

```rust
#[derive(Accounts)]
pub struct SecureCpi<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: SystemAccount<'info>,
    pub system_program: Program<'info, System>, // Anchor validates program ID
}
```

## Files

- [Vulnerable + Secure Implementation](../programs/solana-security-template/src/instructions/arbitrary_cpi.rs)
- [Tests](../tests/src/test_arbitrary_cpi/mod.rs)

## Reference

- [Sealevel Attacks: Arbitrary CPI](https://github.com/coral-xyz/sealevel-attacks/tree/master/programs/5-arbitrary-cpi)
