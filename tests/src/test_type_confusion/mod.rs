pub mod helpers;
pub use helpers::*;

use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::common::setup_svm;

/// BUG: AccountInfo accepts any account type - attacker passes UserData as AdminConfig.
#[test]
fn test_vulnerable_accepts_wrong_account_type() {
    let mut svm = setup_svm();

    let attacker = Keypair::new();
    svm.airdrop(&attacker.pubkey(), 10_000_000_000).unwrap();

    let fake_admin = Keypair::new();
    let tx = Transaction::new_signed_with_payer(
        &[initialize_user_data_ix(
            fake_admin.pubkey(),
            attacker.pubkey(),
            attacker.pubkey(), // Attacker controls owner field at same byte offset as authority
        )],
        Some(&attacker.pubkey()),
        &[&attacker, &fake_admin],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();

    // Attacker calls vulnerable_admin_action with UserData account
    let tx = Transaction::new_signed_with_payer(
        &[vulnerable_admin_ix(fake_admin.pubkey(), attacker.pubkey())],
        Some(&attacker.pubkey()),
        &[&attacker],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx);
    assert!(
        result.is_ok(),
        "Exploit should succeed with wrong account type"
    );
}

/// FIX: Account<T> validates discriminator - rejects wrong account type.
#[test]
fn test_secure_rejects_wrong_account_type() {
    let mut svm = setup_svm();

    let attacker = Keypair::new();
    svm.airdrop(&attacker.pubkey(), 10_000_000_000).unwrap();

    let fake_admin = Keypair::new();
    let tx = Transaction::new_signed_with_payer(
        &[initialize_user_data_ix(
            fake_admin.pubkey(),
            attacker.pubkey(),
            attacker.pubkey(),
        )],
        Some(&attacker.pubkey()),
        &[&attacker, &fake_admin],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[secure_admin_ix(fake_admin.pubkey(), attacker.pubkey())],
        Some(&attacker.pubkey()),
        &[&attacker],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx);
    assert!(
        result.is_err(),
        "Secure version should reject wrong account type"
    );
}
