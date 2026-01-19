//! Integer Overflow - See docs/integer-overflow.md for full explanation.

use anchor_lang::prelude::*;

use crate::error::ErrorCode;

#[account]
pub struct Counter {
    pub value: u64,
}

/// VULNERABLE: Uses unchecked arithmetic that can overflow.
#[derive(Accounts)]
pub struct VulnerableAdd<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

pub fn vulnerable_add(ctx: Context<VulnerableAdd>, amount: u64) -> Result<()> {
    // CHECK: BUG - Unchecked addition can wrap around on overflow
    ctx.accounts.counter.value = ctx.accounts.counter.value.wrapping_add(amount);
    msg!("Counter: {} (vulnerable)", ctx.accounts.counter.value);
    Ok(())
}

/// SECURE: Uses checked arithmetic that returns error on overflow.
#[derive(Accounts)]
pub struct SecureAdd<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

pub fn secure_add(ctx: Context<SecureAdd>, amount: u64) -> Result<()> {
    ctx.accounts.counter.value = ctx
        .accounts
        .counter
        .value
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;
    msg!("Counter: {} (secure)", ctx.accounts.counter.value);
    Ok(())
}

/// Initialize counter for testing.
#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(init, payer = payer, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_counter(ctx: Context<InitializeCounter>, initial_value: u64) -> Result<()> {
    ctx.accounts.counter.value = initial_value;
    Ok(())
}
