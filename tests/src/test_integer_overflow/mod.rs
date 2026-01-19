pub mod helpers;
pub use helpers::*;

use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::common::{create_funded_payer, setup_svm};

/// BUG: wrapping_add silently overflows, wrapping u64::MAX + 1 to 0.
#[test]
fn test_vulnerable_allows_overflow() {
    let mut svm = setup_svm();
    let payer = create_funded_payer(&mut svm);
    let counter = Keypair::new();

    // Create counter near max value
    let tx = Transaction::new_signed_with_payer(
        &[initialize_counter_ix(
            counter.pubkey(),
            payer.pubkey(),
            u64::MAX - 10,
        )],
        Some(&payer.pubkey()),
        &[&payer, &counter],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();

    // Add amount that causes overflow
    let tx = Transaction::new_signed_with_payer(
        &[vulnerable_add_ix(counter.pubkey(), 100)],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    // Succeeds but value wraps around (bug!)
    let result = svm.send_transaction(tx);
    assert!(result.is_ok());
}

/// FIX: checked_add returns error on overflow.
#[test]
fn test_secure_rejects_overflow() {
    let mut svm = setup_svm();
    let payer = create_funded_payer(&mut svm);
    let counter = Keypair::new();

    // Create counter near max value
    let tx = Transaction::new_signed_with_payer(
        &[initialize_counter_ix(
            counter.pubkey(),
            payer.pubkey(),
            u64::MAX - 10,
        )],
        Some(&payer.pubkey()),
        &[&payer, &counter],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();

    // Add amount that would overflow
    let tx = Transaction::new_signed_with_payer(
        &[secure_add_ix(counter.pubkey(), 100)],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );

    // Fails with Overflow error
    let result = svm.send_transaction(tx);
    assert!(result.is_err());
}
