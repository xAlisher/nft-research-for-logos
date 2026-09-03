//! Epic C — selective disclosure of a private NFT via a viewing key (research issue C2).
//!
//! Claim under test: the owner of a privately-held NFT can hand a chosen verifier their *viewing
//! key* (the read capability, `d`/`z`), and that verifier can decode the holding and confirm
//! ownership/authenticity — WITHOUT the spend key, and WITHOUT anyone else (wrong key) learning
//! anything. This is the "reveal on your terms, to whom you choose" differentiator.
//!
//! Uses the real encryption primitives (`ViewingPublicKey` / `SharedSecretKey` ML-KEM-768 +
//! `EncryptionScheme` ChaCha20), exactly as the on-chain note encryption and wallet sync do.
//! Node-free (pure crypto), so it runs anywhere `RISC0_DEV_MODE=1 cargo test` runs.

use lee_core::{
    Commitment, EncryptionScheme, SharedSecretKey,
    account::{Account, AccountId, data::Data},
    encryption::ViewingPublicKey,
    program::PrivateAccountKind,
};
use token_core::TokenHolding;

#[test]
fn viewing_key_reveals_nft_holding_and_wrong_key_does_not() {
    // The owner's viewing key seed halves — the read capability, exported via `account show-keys
    // --viewing-secret`. Sharing (d, z) lets a verifier decode this account's holdings.
    let d = [7u8; 32];
    let z = [9u8; 32];
    let vpk = ViewingPublicKey::from_seed(&d, &z);

    // The privately-held NFT (an owned printed copy of a definition).
    let definition_id = AccountId::new([3; 32]);
    let account_id = AccountId::new([5; 32]);
    let account = Account {
        data: Data::from(&TokenHolding::NftPrintedCopy {
            definition_id,
            owned: true,
        }),
        ..Account::default()
    };
    let commitment = Commitment::new(&account_id, &account);
    let output_index = 0u32;
    let kind = PrivateAccountKind::Regular(0);

    // As on-chain: the note is encrypted toward the owner's viewing public key.
    let (shared_secret, epk) = SharedSecretKey::encapsulate(&vpk);
    let ciphertext =
        EncryptionScheme::encrypt(&account, &kind, &shared_secret, &commitment, output_index);

    // VERIFIER with the correct viewing key: decapsulate the shared secret, decrypt, read the NFT.
    let ss_ok = SharedSecretKey::decapsulate(&epk, &d, &z).expect("decapsulate with correct vsk");
    let (_kind, recovered) =
        EncryptionScheme::decrypt(&ciphertext, &ss_ok, &commitment, output_index)
            .expect("the correct viewing key must reveal the account");
    let holding = TokenHolding::try_from(&recovered.data).expect("token holding decodes");
    assert!(
        matches!(
            holding,
            TokenHolding::NftPrintedCopy {
                owned: true,
                definition_id: got,
            } if got == definition_id
        ),
        "verifier confirms ownership + authenticity of the NFT via the viewing key alone"
    );

    // WRONG viewing key: decapsulation yields a different shared secret, so decryption cannot
    // recover the holding — no disclosure to anyone without the owner's viewing key.
    let (dw, zw) = ([11u8; 32], [22u8; 32]);
    let ss_wrong =
        SharedSecretKey::decapsulate(&epk, &dw, &zw).expect("decapsulate returns some (wrong) secret");
    let wrong = EncryptionScheme::decrypt(&ciphertext, &ss_wrong, &commitment, output_index);
    assert!(
        wrong.map(|(_, a)| a) != Some(recovered),
        "a wrong viewing key must NOT reveal the NFT holding (selective disclosure holds)"
    );
}
