//! Sealed collection — metadata.uri transport prototype (research gap #2, chosen default).
//!
//! The encrypted {link, note} payload is carried ON-CHAIN in the NFT definition's `metadata.uri`
//! field (DATA_MAX_LENGTH = 100 KiB, so the ~2 KB blob fits comfortably). This proves:
//!   1. the encrypted blob (real ML-KEM-768 + ChaCha20, borsh-serialized) fits in the uri, and
//!   2. it round-trips through the real `TokenMetadata` <-> `Data` encoding, and
//!   3. only the recipient's viewing key decrypts it (wrong key fails).
//! No external storage, no host node online: the payload lives on-chain, durable. (Logos Storage is
//! the fallback for large media only — see research/logos-storage-discovery.md.)

use lee_core::{
    Commitment, EncryptionScheme, NullifierPublicKey, SharedSecretKey,
    account::{Account, AccountId, data::{Data, DATA_MAX_LENGTH}},
    encryption::{EncryptedAccountData, ViewingPublicKey},
    program::PrivateAccountKind,
};
use token_core::{MetadataStandard, TokenMetadata};

const URI_PREFIX: &str = "sealed:v1:";

#[test]
fn metadata_uri_carries_encrypted_payload_and_only_recipient_key_reveals_it() {
    // Public, known to the recipient (it's the definition of the NFT they own).
    let definition_id = AccountId::new([7; 32]);
    // KDF salt both sides derive from the public definition id (a fixed account).
    let kdf_commit = Commitment::new(&definition_id, &Account::default());

    // ---- Recipient's receive-key: shares (npk, vpk); keeps the viewing secret (d, z). ----
    let d = [3u8; 32];
    let z = [4u8; 32];
    let vpk = ViewingPublicKey::from_seed(&d, &z);
    let recipient_nsk = [5u8; 32];
    let recipient_npk = NullifierPublicKey::from(&recipient_nsk);

    // ---- CURATOR: encrypt {link, note} to the recipient's viewing key, pack it into a uri. ----
    let payload = br#"{"url":"https://archive.org/details/ChurchCommittee_FullReport","note":"The Senate's own record of intelligence agencies spying on the citizens they served."}"#.to_vec();
    let payload_acc = Account { data: Data::try_from(payload.clone()).unwrap(), ..Account::default() };
    let (ss, epk) = SharedSecretKey::encapsulate(&vpk);
    let ciphertext = EncryptionScheme::encrypt(&payload_acc, &PrivateAccountKind::Regular(0), &ss, &kdf_commit, 0);
    let ead = EncryptedAccountData::new(ciphertext, &recipient_npk, &vpk, epk);
    let blob = borsh::to_vec(&ead).expect("serialize encrypted payload");
    let uri = format!("{URI_PREFIX}{}", hex::encode(&blob));

    // (1) fits on-chain
    let cap = usize::try_from(DATA_MAX_LENGTH.as_u64()).unwrap();
    assert!(uri.len() < cap, "uri ({} bytes) must fit DATA_MAX_LENGTH ({cap})", uri.len());

    // ---- Store it in the NFT metadata; confirm it survives the real TokenMetadata<->Data encoding. ----
    let meta = TokenMetadata {
        definition_id,
        standard: MetadataStandard::Simple,
        uri: uri.clone(),
        creators: "Logos EcoDev".to_owned(),
        primary_sale_date: 0,
    };
    let meta_data = Data::from(&meta);
    assert!(meta_data.as_ref().len() < cap, "metadata account data must fit on-chain");
    let meta_back = TokenMetadata::try_from(&meta_data).expect("metadata round-trips");
    assert_eq!(meta_back.uri, uri, "uri survives the metadata encoding");

    // ---- RECIPIENT: read uri from the metadata, decrypt with the viewing key. ----
    let hex_part = meta_back.uri.strip_prefix(URI_PREFIX).expect("uri prefix");
    let raw = hex::decode(hex_part).expect("hex");
    let ead_back: EncryptedAccountData = borsh::from_slice(&raw).expect("deserialize payload");

    let ss_ok = SharedSecretKey::decapsulate(&ead_back.epk, &d, &z).expect("decapsulate");
    let (_kind, recovered) = EncryptionScheme::decrypt(&ead_back.ciphertext, &ss_ok, &kdf_commit, 0)
        .expect("recipient viewing key must decrypt the metadata payload");
    assert_eq!(recovered.data.as_ref(), payload.as_slice(), "recipient recovers the exact {{link, note}}");

    // ---- WRONG key: cannot read the on-chain payload. ----
    let (dw, zw) = ([11u8; 32], [22u8; 32]);
    let ss_wrong = SharedSecretKey::decapsulate(&ead_back.epk, &dw, &zw).expect("some (wrong) secret");
    let wrong = EncryptionScheme::decrypt(&ead_back.ciphertext, &ss_wrong, &kdf_commit, 0);
    assert!(
        wrong.map(|(_, a)| a.data.as_ref().to_vec()) != Some(payload.clone()),
        "a wrong viewing key must NOT recover the payload from the public metadata"
    );

    println!("metadata.uri transport OK: blob {} bytes (< {cap}); only the recipient viewing key reveals it", uri.len());
}
