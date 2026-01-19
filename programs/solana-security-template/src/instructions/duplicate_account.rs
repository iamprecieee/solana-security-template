//! Duplicate Account Aliasing - See docs/duplicate-account.md for full explanation.

use anchor_lang::prelude::*;

use crate::{error::ErrorCode, Wallet};

/// VULNERABLE: No check that source != destination.
#[derive(Accounts)]
pub struct VulnerableTransfer<'info> {
    #[account(mut)]
    pub source: Account<'info, Wallet>,
    #[account(mut)]
    pub destination: Account<'info, Wallet>,
    pub authority: Signer<'info>,
}

pub fn vulnerable_transfer(ctx: Context<VulnerableTransfer>, amount: u64) -> Result<()> {
    // CHECK: BUG - Caching both balances means second write overwrites first if aliased
    let source_balance = ctx.accounts.source.balance;
    let dest_balance = ctx.accounts.destination.balance;

    ctx.accounts.source.balance = source_balance.saturating_sub(amount);
    ctx.accounts.destination.balance = dest_balance.saturating_add(amount);

    msg!("Transferred {} (vulnerable)", amount);
    Ok(())
}

/// SECURE: Constraint enforces source != destination at deserialization.
#[derive(Accounts)]
pub struct SecureTransfer<'info> {
    #[account(mut)]
    pub source: Account<'info, Wallet>,

    #[account(mut, constraint = source.key() != destination.key() @ ErrorCode::DuplicateAccount)]
    pub destination: Account<'info, Wallet>,
    pub authority: Signer<'info>,
}

pub fn secure_transfer(ctx: Context<SecureTransfer>, amount: u64) -> Result<()> {
    let source_balance = ctx.accounts.source.balance;
    let dest_balance = ctx.accounts.destination.balance;

    ctx.accounts.source.balance = source_balance.saturating_sub(amount);
    ctx.accounts.destination.balance = dest_balance.saturating_add(amount);

    msg!("Transferred {} (secure)", amount);
    Ok(())
}
