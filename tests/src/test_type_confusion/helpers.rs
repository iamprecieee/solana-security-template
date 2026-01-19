use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use crate::common::{discriminator, program_id};

pub fn initialize_admin_config_ix(config: Pubkey, payer: Pubkey, authority: Pubkey) -> Instruction {
    let mut data = discriminator("initialize_admin_config").to_vec();
    data.extend_from_slice(authority.as_ref());

    Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(config, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(Pubkey::default(), false),
        ],
        data,
    }
}

pub fn initialize_user_data_ix(user_data: Pubkey, payer: Pubkey, owner: Pubkey) -> Instruction {
    let mut data = discriminator("initialize_user_data").to_vec();
    data.extend_from_slice(owner.as_ref());

    Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(user_data, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(Pubkey::default(), false),
        ],
        data,
    }
}

pub fn vulnerable_admin_ix(config: Pubkey, signer: Pubkey) -> Instruction {
    let data = discriminator("vulnerable_admin_action").to_vec();

    Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(config, false),
            AccountMeta::new_readonly(signer, true),
        ],
        data,
    }
}

pub fn secure_admin_ix(config: Pubkey, signer: Pubkey) -> Instruction {
    let data = discriminator("secure_admin_action").to_vec();

    Instruction {
        program_id: program_id(),
        accounts: vec![
            AccountMeta::new(config, false),
            AccountMeta::new_readonly(signer, true),
        ],
        data,
    }
}
