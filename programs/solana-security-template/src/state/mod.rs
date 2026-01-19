use anchor_lang::prelude::*;

#[account]
pub struct Wallet {
    pub owner: Pubkey,
    pub balance: u64,
}

impl Wallet {
    pub const SIZE: usize = 8 + 32 + 8; // discriminator + pubkey + u64
}
