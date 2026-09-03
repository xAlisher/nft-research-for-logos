//! Sealed collection — encrypt-then-shield prototype (research gap #2), as a runnable journey test.
//!
//! Distribution = shield-to-recipient. Per recipient, the curator:
//!   1. encrypts the {link, note} payload to the recipient's VIEWING KEY (ML-KEM-768 + ChaCha20 —
//!      the real note-encryption path), producing the "payload store" blob (ciphertext + epk); and
//!   2. shields the NFT (public -> the recipient's private account) so only they own it.
//! The recipient then opens BOTH with their viewing key; a wrong key opens neither. This binds the
//! payload and the NFT to the same receive-key, which is exactly what the Sealed module needs.
//!
//! Faithful: uses the LEZ token program, the privacy circuit, and the real encryption primitives
//! (see experiments/nft-privacy-proof and nft-selective-disclosure). Runs under RISC0_DEV_MODE=1.

use lee::{
    Account, AccountId, PrivacyPreservingTransaction, PrivateKey, PublicKey, V03State,
    privacy_preserving_transaction::{self as pptx, circuit},
    program::Program,
};
use lee_core::{
    Commitment, DUMMY_COMMITMENT_HASH, EncryptionScheme, InputAccountIdentity, SharedSecretKey,
    NullifierPublicKey,
    account::{AccountWithMetadata, Nonce, data::Data},
    encryption::ViewingPublicKey,
    program::PrivateAccountKind,
};
use token_core::{Instruction as TokenInstruction, TokenHolding};

#[test]
fn encrypt_then_shield_binds_payload_and_nft_to_recipient_viewing_key() {
    let token_id = programs::token().id();
    let definition_id = AccountId::new([7; 32]);

    // ---- RECIPIENT's receive-key: they share (npk, vpk); they keep the viewing secret (d, z). ----
    let d = [3u8; 32];
    let z = [4u8; 32];
    let vpk = ViewingPublicKey::from_seed(&d, &z);
    let recipient_nsk = [5u8; 32];
    let recipient_npk = NullifierPublicKey::from(&recipient_nsk);
    let recipient_id = AccountId::for_regular_private_account(&recipient_npk, &vpk, 0);

    // =========================================================================================
    // CURATOR — step 1: encrypt the {link, note} payload to the recipient's viewing key.
    // The payload rides in an Account's `data`, encrypted exactly as an on-chain note would be.
    // =========================================================================================
    let payload = br#"{"url":"https://archive.org/details/ChurchCommittee_FullReport","note":"The Senate's own record of intelligence agencies spying on the citizens they served."}"#.to_vec();
    let payload_acc = Account {
        data: Data::try_from(payload.clone()).expect("payload fits in Data"),
        ..Account::default()
    };
    let payload_id = AccountId::new([9; 32]);
    let payload_commit = Commitment::new(&payload_id, &payload_acc);
    let (payload_ss, payload_epk) = SharedSecretKey::encapsulate(&vpk);
    let payload_ciphertext =
        EncryptionScheme::encrypt(&payload_acc, &PrivateAccountKind::Regular(0), &payload_ss, &payload_commit, 0);
    // The distributable blob for this piece = { payload_ciphertext, payload_epk, payload_commit }.

    // =========================================================================================
    // CURATOR — step 2: shield the NFT (public sender -> recipient's private account).
    // =========================================================================================
    let sender_sk = PrivateKey::try_new([37; 32]).unwrap();
    let sender_id = AccountId::from(&PublicKey::new_from_private_key(&sender_sk));
    let sender_nonce = Nonce(1);
    let sender_acc = Account {
        program_owner: token_id,
        balance: 0,
        nonce: sender_nonce,
        data: Data::from(&TokenHolding::NftPrintedCopy { definition_id, owned: true }),
    };
    let mut state = V03State::new().with_public_accounts([(sender_id, sender_acc.clone())]);

    let sender_pre = AccountWithMetadata::new(sender_acc.clone(), true, sender_id);
    let recipient_pre = AccountWithMetadata::new(Account::default(), true, recipient_id);
    let (output, proof) = circuit::execute_and_prove(
        vec![sender_pre, recipient_pre],
        Program::serialize_instruction(TokenInstruction::Transfer { amount_to_transfer: 1 }).unwrap(),
        vec![
            InputAccountIdentity::Public,
            InputAccountIdentity::PrivateForeignInit {
                vpk,
                random_seed: [0; 32],
                npk: recipient_npk,
                identifier: 0,
                commitment_root: DUMMY_COMMITMENT_HASH,
            },
        ],
        &programs::token().into(),
    )
    .expect("shield transfer must execute + prove");
    let message = pptx::message::Message::try_from_circuit_output(vec![sender_id], vec![sender_nonce], output).unwrap();
    let witness_set = pptx::witness_set::WitnessSet::for_message(&message, proof, &[&sender_sk]);
    let tx = PrivacyPreservingTransaction::new(message, witness_set);

    // Recipient's private NFT commitment after the shield (owned by them, owned:true).
    let expected_recipient_nft = Commitment::new(
        &recipient_id,
        &Account {
            program_owner: token_id,
            nonce: Nonce::private_account_nonce_init(&recipient_id),
            balance: 0,
            data: Data::from(&TokenHolding::NftPrintedCopy { definition_id, owned: true }),
        },
    );
    assert!(state.get_proof_for_commitment(&expected_recipient_nft).is_none());
    state.transition_from_privacy_preserving_transaction(&tx, 1, 0).expect("shield must verify");
    assert!(
        state.get_proof_for_commitment(&expected_recipient_nft).is_some(),
        "recipient must privately own the shielded NFT"
    );

    // =========================================================================================
    // RECIPIENT — reveal: the SAME viewing key opens both the NFT and the payload.
    // =========================================================================================
    // (a) payload: decapsulate with (d, z) -> decrypt -> the {link, note} bytes.
    let ss_ok = SharedSecretKey::decapsulate(&payload_epk, &d, &z).expect("decapsulate with recipient vsk");
    let (_kind, recovered) =
        EncryptionScheme::decrypt(&payload_ciphertext, &ss_ok, &payload_commit, 0).expect("payload must decrypt");
    assert_eq!(recovered.data.as_ref(), payload.as_slice(), "recipient recovers the exact payload");

    // (b) wrong viewing key opens NEITHER the payload...
    let (dw, zw) = ([11u8; 32], [22u8; 32]);
    let ss_wrong = SharedSecretKey::decapsulate(&payload_epk, &dw, &zw).expect("some (wrong) secret");
    let wrong = EncryptionScheme::decrypt(&payload_ciphertext, &ss_wrong, &payload_commit, 0);
    assert!(
        wrong.map(|(_, a)| a.data.as_ref().to_vec()) != Some(payload.clone()),
        "a wrong viewing key must NOT recover the payload"
    );
    // ...nor the NFT: the wrong key derives a different account id, so the recipient's commitment isn't theirs.
    let wrong_vpk = ViewingPublicKey::from_seed(&dw, &zw);
    let wrong_npk = NullifierPublicKey::from(&[99u8; 32]);
    let wrong_id = AccountId::for_regular_private_account(&wrong_npk, &wrong_vpk, 0);
    assert_ne!(wrong_id, recipient_id, "a different viewing key = a different account identity");
}
