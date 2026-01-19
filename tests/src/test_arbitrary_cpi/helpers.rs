use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::common::{discriminator, program_id};

pub fn vulnerable_cpi_ix(
    from: Pubkey,
    to: Pubkey,
    target_program: Pubkey,
    amount: u64,
) -> Instruction {
    let mut data = discriminator("vulnerable_cpi").to_vec();
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(from, true),
            AccountMeta::new(to, false),
            AccountMeta::new_readonly(target_program, false),
        ],
        data,
    }
}

pub fn secure_cpi_ix(from: Pubkey, to: Pubkey, program: Pubkey, amount: u64) -> Instruction {
    let mut data = discriminator("secure_cpi").to_vec();
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(from, true),
            AccountMeta::new(to, false),
            AccountMeta::new_readonly(program, false),
        ],
        data,
    }
}
