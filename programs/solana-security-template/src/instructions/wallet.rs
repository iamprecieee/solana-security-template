use anchor_lang::prelude::*;

use crate::Wallet;

#[derive(Accounts)]
pub struct CreateWallet<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 8)]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_wallet(ctx: Context<CreateWallet>, initial_balance: u64) -> Result<()> {
    ctx.accounts.wallet.owner = ctx.accounts.payer.key();
    ctx.accounts.wallet.balance = initial_balance;

    Ok(())
}
