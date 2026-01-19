pub mod helpers;
pub use helpers::*;

use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::common::{create_funded_payer, setup_svm};

/// BUG: UncheckedAccount accepts any program - Anchor doesn't validate.
#[test]
fn test_vulnerable_accepts_fake_program_at_anchor_level() {
    let mut svm = setup_svm();
    let payer = create_funded_payer(&mut svm);
    let destination = Keypair::new();

    let fake_program = Keypair::new().pubkey();
    let tx = Transaction::new_signed_with_payer(
        &[vulnerable_cpi_ix(
            payer.pubkey(),
            destination.pubkey(),
            fake_program,
            1_000,
        )],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    // Fails at invoke (program not found), not at Anchor validation
    let result = svm.send_transaction(tx);
    assert!(result.is_err());
}

/// FIX: Program<System> validates program ID at deserialization.
#[test]
fn test_secure_rejects_fake_program_at_anchor_level() {
    let mut svm = setup_svm();
    let payer = create_funded_payer(&mut svm);
    let destination = Keypair::new();

    let fake_program = Keypair::new().pubkey();
    let tx = Transaction::new_signed_with_payer(
        &[secure_cpi_ix(
            payer.pubkey(),
            destination.pubkey(),
            fake_program,
            1_000,
        )],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    // Fails at Anchor validation - wrong program ID
    let result = svm.send_transaction(tx);
    assert!(result.is_err());
}
