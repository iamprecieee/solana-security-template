use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

pub mod helpers;
pub use helpers::*;

use crate::common::{create_funded_payer, setup_svm};

/// BUG: Same account passed as source and destination inflates balance.
#[test]
fn test_vulnerable_allows_same_account() {
    let mut svm = setup_svm();
    let payer = create_funded_payer(&mut svm);
    let wallet = Keypair::new();

    let wallet_creation_tx = Transaction::new_signed_with_payer(
        &[create_wallet_ix(wallet.pubkey(), payer.pubkey(), 100)],
        Some(&payer.pubkey()),
        &[&payer, &wallet],
        svm.latest_blockhash(),
    );
    svm.send_transaction(wallet_creation_tx).unwrap();

    let vulnerable_transfer_tx = Transaction::new_signed_with_payer(
        &[transfer_ix(
            wallet.pubkey(),
            wallet.pubkey(),
            payer.pubkey(),
            50,
            false,
        )],
        Some(&payer.pubkey()),
        &[&payer, &wallet],
        svm.latest_blockhash(),
    );
    svm.send_transaction(vulnerable_transfer_tx).unwrap();

    let account = svm.get_account(&wallet.pubkey()).unwrap();
    let balance = u64::from_le_bytes(account.data[40..48].try_into().unwrap());
    // Started with 100, transferred 50 to self, ended with 150 (bug!)
    assert_eq!(balance, 150);
}

/// FIX: Constraint rejects transaction when source == destination.
#[test]
fn test_secure_rejects_same_account() {
    let mut svm = setup_svm();
    let payer = create_funded_payer(&mut svm);
    let wallet = Keypair::new();

    let wallet_creation_tx = Transaction::new_signed_with_payer(
        &[create_wallet_ix(wallet.pubkey(), payer.pubkey(), 100)],
        Some(&payer.pubkey()),
        &[&payer, &wallet],
        svm.latest_blockhash(),
    );
    svm.send_transaction(wallet_creation_tx).unwrap();

    let secure_transfer_tx = Transaction::new_signed_with_payer(
        &[transfer_ix(
            wallet.pubkey(),
            wallet.pubkey(),
            payer.pubkey(),
            50,
            true,
        )],
        Some(&payer.pubkey()),
        &[&payer, &wallet],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(secure_transfer_tx);
    assert!(
        result.is_err(),
        "secure_transfer should reject aliased accounts"
    );

    let account = svm.get_account(&wallet.pubkey()).unwrap();
    let balance = u64::from_le_bytes(account.data[40..48].try_into().unwrap());
    // Balance unchanged - transaction was rejected
    assert_eq!(balance, 100);
}
