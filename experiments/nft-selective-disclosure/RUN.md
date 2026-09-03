# Epic C — viewing-key selective disclosure (proof + export)

_2026-09-03. The "reveal on your terms, to whom you choose" differentiator._

## C2 — selective disclosure PROVEN (node-free)

[`nft_selective_disclosure.rs`](nft_selective_disclosure.rs) — `test viewing_key_reveals_nft_holding_and_wrong_key_does_not ... ok` (integration_tests, on Sneg).

Using the **real** on-chain encryption primitives (ML-KEM-768 `ViewingPublicKey`/`SharedSecretKey` + ChaCha20 `EncryptionScheme`):
1. A private NFT holding (`TokenHolding::NftPrintedCopy { owned: true }`) is encrypted toward an owner's viewing public key, exactly as the chain stores it.
2. A **verifier holding the owner's viewing secret key** (`d`,`z`) decapsulates + decrypts and recovers the holding — confirming ownership + authenticity **without the spend key**.
3. A **wrong viewing key** cannot recover the holding — no disclosure to anyone else.

That is selective disclosure: the owner reveals one asset to one chosen verifier, and no one else learns anything.

## C1 — viewing-key export (verified live)

`wallet account show-keys --account-id <private> --viewing-secret` now prints, in addition to the default `npk`/`vpk` (kept intact for the `--to-keys` receive flow):
```
vsk_d <hex>
vsk_z <hex>
```
This is the read capability the owner hands a verifier. Verified live on Sneg against a fresh private account.

## C3 — remaining (packaged verifier + recording)

The capability is proven (C2) and the key export exists (C1). What remains for a polished demo:
- a **standalone verifier CLI** that takes `(vsk_d, vsk_z)` + an account reference, fetches the account's encrypted state from the node, and prints the decoded holding (a read-only wrapper over the sync/decode path the wallet already uses at `lib.rs` `EncryptionScheme::decrypt` + `SharedSecretKey::decapsulate`);
- a short recording of owner-exports-key → verifier-confirms-ownership.

This unlocks the **"Keyholder"** collection tier (docs/collection-concepts.md #4): access passes proven to a verifier without revealing which token or the holder's wallet.

## Honesty
Dev-mode proving for the surrounding stack; the disclosure crypto itself is the production path (ML-KEM-768 + ChaCha20), exercised directly. The zk "prove a rare trait without revealing which token" variant is a separate Phase-2 build, not this.
