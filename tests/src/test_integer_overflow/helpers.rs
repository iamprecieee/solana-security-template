use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::common::{discriminator, program_id, SYSTEM_PROGRAM_ID};

pub fn initialize_counter_ix(counter: Pubkey, payer: Pubkey, initial_value: u64) -> Instruction {
    let mut data = discriminator("initialize_counter").to_vec();
    data.extend_from_slice(&initial_value.to_le_bytes());

    Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(counter, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ],
        data,
    }
}

pub fn vulnerable_add_ix(counter: Pubkey, amount: u64) -> Instruction {
    let mut data = discriminator("vulnerable_add").to_vec();
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: program_id(),
        accounts: vec![AccountMeta::new(counter, false)],
        data,
    }
}

pub fn secure_add_ix(counter: Pubkey, amount: u64) -> Instruction {
    let mut data = discriminator("secure_add").to_vec();
    data.extend_from_slice(&amount.to_le_bytes());

    Instruction {
        program_id: program_id(),
        accounts: vec![AccountMeta::new(counter, false)],
        data,
    }
}
