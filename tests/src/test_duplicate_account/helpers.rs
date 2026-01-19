use solana_sdk::{
    message::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::common::{discriminator, program_id};

pub fn create_wallet_ix(wallet_pubkey: Pubkey, payer_pubkey: Pubkey, balance: u64) -> Instruction {
    let mut data = discriminator("create_wallet").to_vec();
    data.extend_from_slice(&balance.to_le_bytes());

    Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(wallet_pubkey, true),
            AccountMeta::new(payer_pubkey, true),
            AccountMeta::new_readonly(Pubkey::default(), false),
        ],
        data,
    }
}

pub fn transfer_ix(
    source: Pubkey,
    dest: Pubkey,
    authority: Pubkey,
    amount: u64,
    secure: bool,
) -> Instruction {
    let name = if secure {
        "secure_transfer"
    } else {
        "vulnerable_transfer"
    };
    let mut data = discriminator(name).to_vec();
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(source, true),
            AccountMeta::new(dest, true),
            AccountMeta::new(authority, true),
        ],
        data,
    }
}
