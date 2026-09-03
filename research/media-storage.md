# NFT media storage on Logos — findings

_Research by Sina agent, 2026-09-03. Evidence tags: [CODE — X%] Logos docs/source · [CONFIRMED — X%] authoritative external · [H — X%] inference · [? unknown]._

## 1. On-chain (inscriptions & LEZ state)
- Logos supports **inscription operations** — "write arbitrary data to the Logos Blockchain … a permanent, decentralised record"; Zones post data on-chain via channels. [CODE — 96%] https://docs.logos.co/blockchain/zone-sdk/inscribe-data-on-chain-using-zone-sdk
- **Per-inscription size cap:** the Zone-SDK tutorial handles `Inscription::try_from` failing with "Message is too large to fit in an inscription." [CODE — 95%] same
- **LEZ on-chain data bounded by `max_block_size`** (test config 1 MiB); >1 MiB → `TransactionTooLarge`. [CODE — 97%] `lez/sequencer/service/src/service.rs:47-66`, `integration_tests/tests/block_size_limit.rs`
- **Media fully on-chain?** Only tiny (few-KB SVG/thumbnail); image-sized media won't fit one inscription + is uneconomical to chunk — like Bitcoin Ordinals (bounded + public). [H — 85%]
- On-chain inscriptions are **public-only** (plaintext permanent record; no encryption at that layer). [H — 88%]

## 2. Codex = Logos Storage (the native option)
- **Codex is the old name for the Logos Storage component** (repo `logos-storage/logos-storage-nim`, "formerly nim-codex"). [CODE — 98%] https://docs.logos.co/get-started/glossary#codex
- Upstream Codex = a Decentralized Durability Engine: erasure coding + ZK storage proofs + lazy repair. [CONFIRMED — 93%] https://docs.codex.storage/learn/whitepaper
- **CRITICAL maturity caveat:** the Logos fork **narrowed scope to file-sharing and removed the marketplace + proving logic** (Storage v0.3.0). So on Logos today: **best-effort peer file-sharing, NOT contract-backed persistence** — no storage proofs, no durability marketplace. [CODE — 92%] https://blog.logos.co/article/developer-update-jan-2026
- **Content-addressed by CID** + a manifest (filename/mimetype/size); upload returns CID, others fetch by CID. Module C ABI: `storage_upload_*`, `storage_fetch(cid)`, `storage_download_*`, `storage_download_manifest(cid)`, `storage_delete/remove`, `storage_exists`; async (events). [CODE — 95%] `logos-storage-module` `libstorage.h`, https://docs.logos.co/get-started/glossary#cid
- **Persistence is operator-dependent** — data lives in the hosting node's data-dir; availability depends on some node continuing to host it; no rent/SLA keeping it alive. [CODE — 88%] https://docs.logos.co/storage/get-started/faq
- **Testnet availability:** Logos Storage runs on **testnet v0.2** (Basecamp Package Manager / Nix). Usable for dogfooding as file-sharing without durability guarantees. [CODE — 90%]
- **No built-in NFT↔Storage link:** the token `uri` is free-form; an NFT would just put a Codex CID in it — pinning/resolution is app-layer, not chain-enforced. No storage/CID import in `logos-execution-zone`. [CODE — 90%]

## 3. Off-chain conventional (current token `uri`)
- The live token program stores an **off-chain pointer** (ETH/Metaplex pattern): `TokenMetadata { …, uri: String /* pointer to off-chain metadata */, … }`, `MetadataStandard { Simple, Expanded }`. Chain does not validate/resolve it. [CODE — 98%] `lez/programs/token/core/src/lib.rs:195-224`
- Tradeoffs: HTTP = centralized/"metadata rug"; IPFS = content-addressed but needs pinning; Arweave = pay-once permanence (external to Logos); Logos Storage CID = native but best-effort durability today. [CONFIRMED/CODE — 85-90%]

## 4. Privacy-preserving media (the differentiator)
- **Established pattern (off-Logos):** encrypt media → store ciphertext on decentralized storage → gate the decryption key by token ownership. Reference: Lit Protocol + IPFS/Pinata, Lens gated content. [CONFIRMED — 90%] https://pinata.cloud/blog/how-to-encrypt-and-decrypt-files-on-ipfs-using-lit-protocol-and-pinata/
- **No turnkey Logos equivalent yet** — the Storage module exposes no encrypt/decrypt; encryption is app-side before upload. [CODE — 90%]
- **Native primitives to build it:** LEZ private accounts + viewing/nullifier keys (private ownership); **ML-KEM-768 sealing keys** for shared private accounts (wrap a content key to the owner's key); Waku payload encryption (deliver wrapped keys); key-agreement moving to Kyber-768 (PQ). [CODE — 82-92%]
- **Assembled pattern (buildable, not off-the-shelf):** encrypt media w/ random symmetric key → ciphertext to Logos Storage (CID) → CID in token `uri` → wrap the symmetric key to owner's sealing/viewing pubkey → on selective disclosure, owner unwraps + decrypts. Private NFT → private media. [H — 80%]
- **Honest gap:** encryption solves confidentiality, not persistence — same "still hosted in 6 months?" risk (marketplace removed). [CODE — 85%]

## Recommendation matrix (testnet collection)
| Use-case | Option | Why / caveat |
|---|---|---|
| Tiny public media (icon/SVG) | On-chain (LEZ state / inscription) | Fits ~1 MiB cap; durable + tamper-proof; public-only |
| Standard/large public media | `uri` → Logos Storage CID (fall back Arweave/pinned-IPFS for guaranteed permanence) | Native + content-addressed; Logos durability best-effort today |
| Private NFT, small secret media | Encrypt + Storage CID, key wrapped to owner key | On-chain is plaintext → must go off-chain encrypted |
| Private NFT, large media (differentiator) | Ciphertext on Storage/IPFS + symmetric key KEM-wrapped to owner viewing/sealing key (via Waku); ownership in a private LEZ account | Composes native privacy + Storage; app-layer build; mirrors Lit/Lens |
| Guaranteed permanence now | Arweave / well-pinned IPFS in `uri` | Logos Storage marketplace/proofs not active yet |

**Bottom line:** dogfood a **Logos Storage CID** in the token `uri`; keep on-chain only for tiny icons; for private NFTs build the encrypt-then-CID + key-wrap pattern (Logos has the privacy primitives, no packaged flow); use Arweave/pinned-IPFS when permanence must be guaranteed (Codex durability engine exists upstream but is removed from the Logos fork today).

## Sources
docs.logos.co (glossary #codex/#cid/#viewing-keys, about-mantle, zone-sdk/inscribe, storage/*, lez, set-up-shared-private-lez-account, messaging/security) · code: token core lib.rs:195-224, new_definition.rs:107, sequencer service.rs:47-66, block_size_limit.rs, logos-storage-module libstorage.h · blog.logos.co jan-2026 (Storage v0.3.0), may-2026 (Kyber-768) · https://docs.codex.storage/learn/whitepaper · Pinata/Lit, Lens gated-content, Messari Lit overview
