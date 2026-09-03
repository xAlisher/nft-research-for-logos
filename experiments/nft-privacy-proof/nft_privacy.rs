//! Proof of the NFT-through-privacy path (research issue B0).
//!
//! Claim under test: an NFT holding (`TokenHolding::NftPrintedCopy`, serialized into `account.data`)
//! can be owned privately and transferred privately through the SAME privacy-preserving execution
//! circuit that a fungible balance uses — because the privacy layer commits opaque `account.data` and
//! the circuit runs an arbitrary program. This was flagged in the research as "structurally sound but
//! unproven end-to-end" (no NFT ever went through the private path).
//!
//! This runs the REAL token program (`programs::token()`) with an `NftPrintedCopy` through
//! `execute_and_prove`, applies the resulting privacy-preserving transaction to `V03State`, and
//! asserts the private ownership transition landed as private commitments — no public owner, no edge.
//!
//! Mirrors the proven fungible cases (`lee` `transition_from_privacy_preserving_transaction_private`
//! and `integration_tests` `tps::build_privacy_transaction`), swapping the balance program/field for
//! the token program and an NFT holding in `data`. Run under `RISC0_DEV_MODE=1` like the rest of CI.

#![expect(
    clippy::tests_outside_test_module,
    clippy::unwrap_used,
    reason = "integration test"
)]

use lee::{
    Account, AccountId, PrivacyPreservingTransaction, V03State,
    privacy_preserving_transaction::{self as pptx, circuit},
    program::Program,
};
use lee_core::{
    Commitment, DUMMY_COMMITMENT_HASH, InputAccountIdentity, Nullifier, NullifierPublicKey,
    account::{AccountWithMetadata, Nonce, data::Data},
    encryption::ViewingPublicKey,
};
use token_core::{Instruction as TokenInstruction, TokenHolding};

fn nft(definition_id: AccountId, owned: bool) -> Data {
    Data::from(&TokenHolding::NftPrintedCopy {
        definition_id,
        owned,
    })
}

#[test]
fn private_nft_transfer_hides_owner_and_provenance() {
    let definition_id = AccountId::new([7; 32]);
    let token = programs::token();
    let token_id = token.id();

    // Sender keys (private account that owns the NFT).
    let sender_nsk = [1u8; 32];
    let sender_vpk = ViewingPublicKey::from_seed(&[99u8; 32], &[100u8; 32]);
    let sender_npk = NullifierPublicKey::from(&sender_nsk);
    let sender_account_id =
        AccountId::for_regular_private_account(&sender_npk, &sender_vpk, 0);

    // Recipient keys (fresh private account).
    let recipient_nsk = [2u8; 32];
    let recipient_vpk = ViewingPublicKey::from_seed(&[101u8; 32], &[102u8; 32]);
    let recipient_npk = NullifierPublicKey::from(&recipient_nsk);
    let recipient_account_id =
        AccountId::for_regular_private_account(&recipient_npk, &recipient_vpk, 0);

    // The sender privately owns the NFT (value lives in `data`, not `balance`).
    let sender_nonce = Nonce(0xdead_beef);
    let sender_account = Account {
        program_owner: token_id,
        balance: 0,
        nonce: sender_nonce,
        data: nft(definition_id, true),
    };
    let sender_commitment = Commitment::new(&sender_account_id, &sender_account);

    let mut state = V03State::new().with_private_accounts([(
        sender_commitment,
        Nullifier::for_account_initialization(&sender_account_id),
    )]);

    // Build the private -> private NFT transfer by running the real token program in the circuit.
    let membership_proof = state
        .get_proof_for_commitment(&sender_commitment)
        .expect("sender's commitment must be in state");

    let sender_pre =
        AccountWithMetadata::new(sender_account.clone(), true, sender_account_id);
    // Fresh (default) recipient: `token::transfer` zeroizes a copy from the sender (owned = false)
    // then flips it to owned = true, so no pre-initialization is required.
    let recipient_pre =
        AccountWithMetadata::new(Account::default(), true, recipient_account_id);

    // NFT printed-copy transfer requires amount_to_transfer == 1 (token transfer.rs).
    let instruction =
        Program::serialize_instruction(TokenInstruction::Transfer { amount_to_transfer: 1 })
            .unwrap();

    let (output, proof) = circuit::execute_and_prove(
        vec![sender_pre, recipient_pre],
        instruction,
        vec![
            InputAccountIdentity::PrivateAuthorizedUpdate {
                vpk: sender_vpk,
                random_seed: [0; 32],
                view_tag: 0,
                nsk: sender_nsk,
                membership_proof,
                identifier: 0,
            },
            InputAccountIdentity::PrivateForeignInit {
                vpk: recipient_vpk,
                random_seed: [0; 32],
                npk: recipient_npk,
                identifier: 0,
                commitment_root: DUMMY_COMMITMENT_HASH,
            },
        ],
        &token.into(),
    )
    .expect("the NFT private transfer must execute and prove in the privacy circuit");

    let message = pptx::message::Message::try_from_circuit_output(vec![], vec![], output).unwrap();
    let witness_set = pptx::witness_set::WitnessSet::for_message(&message, proof, &[]);
    let tx = PrivacyPreservingTransaction::new(message, witness_set);

    // Expected post-states: same NFT definition, ownership flipped.
    let expected_sender_commitment = Commitment::new(
        &sender_account_id,
        &Account {
            program_owner: token_id,
            nonce: sender_nonce.private_account_nonce_increment(&sender_nsk),
            balance: 0,
            data: nft(definition_id, false),
        },
    );
    let expected_recipient_commitment = Commitment::new(
        &recipient_account_id,
        &Account {
            program_owner: token_id,
            nonce: Nonce::private_account_nonce_init(&recipient_account_id),
            balance: 0,
            data: nft(definition_id, true),
        },
    );

    // Neither post-state exists before the transition.
    assert!(
        state
            .get_proof_for_commitment(&expected_sender_commitment)
            .is_none()
    );
    assert!(
        state
            .get_proof_for_commitment(&expected_recipient_commitment)
            .is_none()
    );

    // Apply the private transfer. `.unwrap()` here proves the circuit proof VERIFIED for an NFT.
    state
        .transition_from_privacy_preserving_transaction(&tx, 1, 0)
        .expect("the NFT private transfer proof must verify against state");

    // The ownership transition landed purely as private commitments:
    //  - sender now holds the NFT with owned = false,
    //  - recipient now holds the same NFT definition with owned = true,
    // with no public `ownerOf` and no on-chain sender->recipient edge.
    assert!(
        state
            .get_proof_for_commitment(&expected_sender_commitment)
            .is_some(),
        "sender's post-transfer (owned = false) NFT commitment must be in private state"
    );
    assert!(
        state
            .get_proof_for_commitment(&expected_recipient_commitment)
            .is_some(),
        "recipient's post-transfer (owned = true) NFT commitment must be in private state"
    );
}
