//! Closing Account Refund Theft - See docs/closing-account.md for full explanation.

use anchor_lang::prelude::*;

#[account]
pub struct DataAccount {
    pub authority: Pubkey,
    pub data: u64,
}

/// VULNERABLE: close destination not validated.
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
    // Transfer lamports to destination
    let data_account = &ctx.accounts.data_account.to_account_info();
    let dest = &ctx.accounts.destination;

    **dest.try_borrow_mut_lamports()? += data_account.lamports();
    **data_account.try_borrow_mut_lamports()? = 0;

    // Zero out data
    let mut data = data_account.try_borrow_mut_data()?;
    data.fill(0);

    msg!("Account closed (vulnerable)");
    Ok(())
}

/// SECURE: Destination must be the authority.
#[derive(Accounts)]
pub struct SecureClose<'info> {
    #[account(
        mut,
        has_one = authority,
        close = authority
    )]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn secure_close(_ctx: Context<SecureClose>) -> Result<()> {
    // Anchor's `close = authority` handles everything
    msg!("Account closed (secure)");
    Ok(())
}

/// Initialize a data account for testing.
#[derive(Accounts)]
pub struct InitializeDataAccount<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 8)]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_data_account(
    ctx: Context<InitializeDataAccount>,
    authority: Pubkey,
) -> Result<()> {
    ctx.accounts.data_account.authority = authority;
    ctx.accounts.data_account.data = 0;
    Ok(())
}
