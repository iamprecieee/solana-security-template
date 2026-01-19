//! Arbitrary CPI - See docs/arbitrary-cpi.md for full explanation.

use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

/// VULNERABLE: UncheckedAccount bypasses program ID validation.
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
        Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
        },
    );
    transfer(cpi_ctx, amount)?;

    msg!("Transfer executed (vulnerable)");
    Ok(())
}

/// SECURE: Program<System> enforces correct program ID at deserialization.
#[derive(Accounts)]
pub struct SecureCpi<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn secure_cpi(ctx: Context<SecureCpi>, amount: u64) -> Result<()> {
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
        },
    );
    transfer(cpi_ctx, amount)?;

    msg!("Transfer executed (secure)");
    Ok(())
}
