pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;

#[allow(ambiguous_glob_reexports)]
pub use instructions::*;

pub use state::*;

declare_id!("FsNmrbd8ux15qx7sKxFFw6aJQVnpyFkRggX1f16jEVt1");

#[program]
pub mod solana_security_template {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn create_wallet(ctx: Context<CreateWallet>, initial_balance: u64) -> Result<()> {
        wallet::create_wallet(ctx, initial_balance)
    }

    pub fn vulnerable_transfer(ctx: Context<VulnerableTransfer>, amount: u64) -> Result<()> {
        duplicate_account::vulnerable_transfer(ctx, amount)
    }

    pub fn secure_transfer(ctx: Context<SecureTransfer>, amount: u64) -> Result<()> {
        duplicate_account::secure_transfer(ctx, amount)
    }

    pub fn initialize_admin_config(
        ctx: Context<InitializeAdminConfig>,
        authority: Pubkey,
    ) -> Result<()> {
        type_confusion::initialize_admin_config(ctx, authority)
    }

    pub fn initialize_user_data(ctx: Context<InitializeUserData>, owner: Pubkey) -> Result<()> {
        type_confusion::initialize_user_data(ctx, owner)
    }

    pub fn vulnerable_admin_action(ctx: Context<VulnerableAdmin>) -> Result<()> {
        type_confusion::vulnerable_admin_action(ctx)
    }

    pub fn secure_admin_action(ctx: Context<SecureAdmin>) -> Result<()> {
        type_confusion::secure_admin_action(ctx)
    }

    pub fn vulnerable_cpi(ctx: Context<VulnerableCpi>, amount: u64) -> Result<()> {
        arbitrary_cpi::vulnerable_cpi(ctx, amount)
    }

    pub fn secure_cpi(ctx: Context<SecureCpi>, amount: u64) -> Result<()> {
        arbitrary_cpi::secure_cpi(ctx, amount)
    }

    pub fn initialize_data_account(
        ctx: Context<InitializeDataAccount>,
        authority: Pubkey,
    ) -> Result<()> {
        closing_account::initialize_data_account(ctx, authority)
    }

    pub fn vulnerable_close(ctx: Context<VulnerableClose>) -> Result<()> {
        closing_account::vulnerable_close(ctx)
    }

    pub fn secure_close(ctx: Context<SecureClose>) -> Result<()> {
        closing_account::secure_close(ctx)
    }

    pub fn initialize_counter(ctx: Context<InitializeCounter>, initial_value: u64) -> Result<()> {
        integer_overflow::initialize_counter(ctx, initial_value)
    }

    pub fn vulnerable_add(ctx: Context<VulnerableAdd>, amount: u64) -> Result<()> {
        integer_overflow::vulnerable_add(ctx, amount)
    }

    pub fn secure_add(ctx: Context<SecureAdd>, amount: u64) -> Result<()> {
        integer_overflow::secure_add(ctx, amount)
    }
}
