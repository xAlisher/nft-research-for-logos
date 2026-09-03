# Encrypt-then-shield — prototype (gap #2, PROVEN)

_2026-09-03. The per-recipient distribution step for shield-to-recipient, proven end-to-end at the crypto + chain level. Test: [`nft_encrypt_then_shield.rs`](nft_encrypt_then_shield.rs)._

## Result
```
test encrypt_then_shield_binds_payload_and_nft_to_recipient_viewing_key ... ok
```
(integration_tests, on Sneg, `RISC0_DEV_MODE=1`.)

## What it proves
For one recipient, given their receive-key `(npk, vpk)`, the curator:
1. **Encrypts the `{link, note}` payload to the recipient's viewing key** — ML-KEM-768 `encapsulate(vpk)` → shared secret → ChaCha20 note encryption (`EncryptionScheme::encrypt`). The distributable blob = `{ciphertext, epk, commitment}`. This is the real on-chain note-encryption path, applied to the payload.
2. **Shields the NFT** (public sender → the recipient's private account) via the token program through the privacy circuit. After the transfer, the recipient privately owns an `NftPrintedCopy{owned:true}` (no public owner).

Then the recipient's **single viewing key** `(d, z)`:
- decapsulates + decrypts the payload → the exact `{link, note}` bytes, **and**
- is the identity that owns the shielded NFT.

A **wrong** viewing key recovers **neither** — it can't decrypt the payload, and it derives a different account id, so the shielded NFT isn't its. Payload and NFT are bound to the same receive-key.

## Transport = metadata.uri (chosen default, PROVEN)
`nft_metadata_uri_transport.rs` — `metadata_uri_carries_encrypted_payload_and_only_recipient_key_reveals_it ... ok`. The encrypted `{link,note}` is borsh-serialized + hex, carried **on-chain** in the NFT definition's `metadata.uri`:
- **Blob = 2818 bytes**, well under the 100 KiB `DATA_MAX_LENGTH` — fits with huge headroom.
- Round-trips through the real `TokenMetadata` ↔ `Data` encoding.
- Only the recipient's viewing key decrypts it (wrong key fails).
- On-chain + durable: **no external storage, no host node online required** (Logos Storage is the fallback only for large media that exceeds 100 KiB — see `../../research/logos-storage-discovery.md`).
- KDF salt is a deterministic commitment derived from the public `definition_id`, so the recipient reconstructs it without extra data.

## Why this closes gap #2
Gap #2 was: the NFT holding only stores `owned:bool`, so the `{link,note}` needs a home + key-wrapping to the owner. This shows the payload can be encrypted to the recipient's viewing key with the **same primitives** the chain already uses, and revealed with the **same key** that owns the NFT. Remaining productization: choose the ciphertext's transport (token `metadata.uri` blob vs Codex) — a storage-location choice, not a crypto question.

## How to run
From the `logos-execution-zone` checkout:
```
env RISC0_DEV_MODE=1 CARGO_TARGET_DIR=/extra/tmp/lez-nft-target cargo test -p integration_tests --test nft_encrypt_then_shield -- --nocapture
```

## Journey mapping (matches docs/journey-and-architecture.md)
- Recipient generates `(npk, vpk)` → shares as a `.keys` file. *(here: fixed test keys)*
- Curator: encrypt payload → vpk **(step 1)** + shield NFT → recipient **(step 2)**.
- Recipient: sync → owns the sealed NFT → viewing key decrypts payload → `{link, note}`.

## Honesty
Dev-mode proving; the payload here rides in an `Account.data` to reuse the exact `EncryptionScheme` (faithful). Production still needs the ciphertext transport decided (uri/Codex) and a `.keys` import/export in the module.
