use litesvm::LiteSVM;
use sha2::{Digest, Sha256};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

pub const SYSTEM_PROGRAM_ID: Pubkey = solana_sdk::pubkey!("11111111111111111111111111111111");

pub fn discriminator(name: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(format!("global:{}", name));

    let result = hasher.finalize();
    result[..8].try_into().unwrap()
}

pub fn program_id() -> Pubkey {
    solana_security_template::ID.to_bytes().into()
}

pub fn setup_svm() -> LiteSVM {
    let mut svm = LiteSVM::new();
    svm.add_program_from_file(
        solana_security_template::ID.to_bytes(),
        "../target/deploy/solana_security_template.so",
    )
    .unwrap();
    svm
}

pub fn create_funded_payer(svm: &mut LiteSVM) -> Keypair {
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 10_000_000).unwrap();
    payer
}
