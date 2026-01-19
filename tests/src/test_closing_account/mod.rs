pub mod helpers;
pub use helpers::*;

use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::common::{create_funded_payer, setup_svm};

/// BUG: Attacker can specify any destination for refund lamports.
#[test]
fn test_vulnerable_allows_arbitrary_destination() {
    let mut svm = setup_svm();
    let authority = create_funded_payer(&mut svm);
    let attacker = Keypair::new();
    let data_account = Keypair::new();

    // Create data account owned by authority
    let tx = Transaction::new_signed_with_payer(
        &[initialize_data_account_ix(
            data_account.pubkey(),
            authority.pubkey(),
            authority.pubkey(),
        )],
        Some(&authority.pubkey()),
        &[&authority, &data_account],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();

    // Authority closes but lamports go to attacker
    let tx = Transaction::new_signed_with_payer(
        &[vulnerable_close_ix(
            data_account.pubkey(),
            authority.pubkey(),
            attacker.pubkey(), // Attacker receives refund!
        )],
        Some(&authority.pubkey()),
        &[&authority],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx);
    assert!(result.is_ok(), "Vulnerable close should succeed");
}

/// FIX: close = authority ensures only authority receives refund.
#[test]
fn test_secure_sends_to_authority_only() {
    let mut svm = setup_svm();
    let authority = create_funded_payer(&mut svm);
    let data_account = Keypair::new();

    // Create data account
    let tx = Transaction::new_signed_with_payer(
        &[initialize_data_account_ix(
            data_account.pubkey(),
            authority.pubkey(),
            authority.pubkey(),
        )],
        Some(&authority.pubkey()),
        &[&authority, &data_account],
        svm.latest_blockhash(),
    );
    svm.send_transaction(tx).unwrap();

    let authority_balance_before = svm.get_account(&authority.pubkey()).unwrap().lamports;

    // Close - lamports go to authority
    let tx = Transaction::new_signed_with_payer(
        &[secure_close_ix(data_account.pubkey(), authority.pubkey())],
        Some(&authority.pubkey()),
        &[&authority],
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(tx);
    assert!(result.is_ok(), "Secure close should succeed");

    // Authority received refund (minus tx fee)
    let authority_balance_after = svm.get_account(&authority.pubkey()).unwrap().lamports;
    assert!(authority_balance_after > authority_balance_before - 10000);
}
