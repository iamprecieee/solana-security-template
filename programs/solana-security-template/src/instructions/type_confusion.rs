//! Type Confusion / Discriminator - See docs/type-confusion.md for full explanation.

use anchor_lang::prelude::*;

use crate::error::ErrorCode;

#[account]
pub struct AdminConfig {
    pub authority: Pubkey,
    pub settings: u64,
}

/// Same byte layout as AdminConfig but different discriminator.
#[account]
pub struct UserData {
    pub owner: Pubkey,
    pub data: u64,
}

#[derive(Accounts)]
pub struct InitializeAdminConfig<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 8)]
    pub config: Account<'info, AdminConfig>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_admin_config(
    ctx: Context<InitializeAdminConfig>,
    authority: Pubkey,
) -> Result<()> {
    ctx.accounts.config.authority = authority;
    ctx.accounts.config.settings = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeUserData<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 8)]
    pub user_data: Account<'info, UserData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_user_data(ctx: Context<InitializeUserData>, owner: Pubkey) -> Result<()> {
    ctx.accounts.user_data.owner = owner;
    ctx.accounts.user_data.data = 0;
    Ok(())
}

/// VULNERABLE: AccountInfo bypasses discriminator check.
#[derive(Accounts)]
pub struct VulnerableAdmin<'info> {
    /// CHECK: BUG - Any account type can be passed here.
    #[account(mut)]
    pub admin_config: AccountInfo<'info>,
    pub signer: Signer<'info>,
}

pub fn vulnerable_admin_action(ctx: Context<VulnerableAdmin>) -> Result<()> {
    let data = ctx.accounts.admin_config.try_borrow_data()?;
    let authority = Pubkey::try_from(&data[8..40]).map_err(|_| ErrorCode::InvalidData)?;
    require!(
        authority == ctx.accounts.signer.key(),
        ErrorCode::Unauthorized
    );

    msg!("Admin action executed (vulnerable)");
    Ok(())
}

/// SECURE: Account<T> validates 8-byte discriminator at deserialization.
#[derive(Accounts)]
pub struct SecureAdmin<'info> {
    #[account(mut)]
    pub admin_config: Account<'info, AdminConfig>,
    pub signer: Signer<'info>,
}

pub fn secure_admin_action(ctx: Context<SecureAdmin>) -> Result<()> {
    require!(
        ctx.accounts.admin_config.authority == ctx.accounts.signer.key(),
        ErrorCode::Unauthorized
    );

    msg!("Admin action executed (secure)");
    Ok(())
}
