use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::common::{discriminator, program_id, SYSTEM_PROGRAM_ID};

pub fn initialize_data_account_ix(
    data_account: Pubkey,
    payer: Pubkey,
    authority: Pubkey,
) -> Instruction {
    let mut data = discriminator("initialize_data_account").to_vec();
    data.extend_from_slice(authority.as_ref());

    Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(data_account, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
        ],
        data,
    }
}

pub fn vulnerable_close_ix(
    data_account: Pubkey,
    authority: Pubkey,
    destination: Pubkey,
) -> Instruction {
    Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(data_account, false),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(destination, false),
        ],
        data: discriminator("vulnerable_close").to_vec(),
    }
}

pub fn secure_close_ix(data_account: Pubkey, authority: Pubkey) -> Instruction {
    Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(data_account, false),
            AccountMeta::new(authority, true),
        ],
        data: discriminator("secure_close").to_vec(),
    }
}
