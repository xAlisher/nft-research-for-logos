# Logos NFT primitives — findings

_Research by Sina agent, 2026-09-03, grounded in Logos docs + repo code under `~/basecamp/refs/`. Evidence tags: [CODE — X%] seen in source/docs · [H — X%] inference · [? unknown]._

_Scope: what the Logos stack (Bedrock/Mantle L1 + Logos Execution Zone) offers for building NFTs today, and what could make a Logos NFT structurally different from ETH/SOL/BTC._

## TL;DR
- A working **Metaplex-style NFT model already exists in the LEZ token program** — non-fungible definitions, a master/printed-copy edition system (`PrintNft`), a metadata account (standard + off-chain URI + creators). [CODE — 95%] `logos-execution-zone/lez/programs/token/core/src/lib.rs`
- Any token holding — fungible or NFT — can live in a **private (shielded) account**: commitment + nullifier + a post-quantum-encrypted note, ownership transitions proven in zk (RISC0/STARK). [CODE — 90%]
- The genuine differentiator is **private-by-default ownership + unlinkable transfer history + viewing-key/zk selective disclosure** — a class of NFT ETH/SOL cannot express and BTC ordinals cannot approach. [H — 80%]
- **Not shippable as a product NFT yet:** wallet/ATA layer returns "unsupported token type" for NFTs, explorer/indexer can't render private holdings, no trait schema, no royalty/marketplace, no selective-disclosure API. [CODE — 90%]

## What exists today — the asset primitives
- **A full token program is built into the LEZ**: `Transfer`, `NewFungibleDefinition`, `NewDefinitionWithMetadata`, `InitializeAccount`, `Burn`, `Mint`, **`PrintNft`**. [CODE — 98%] `token/core/src/lib.rs` (enum `Instruction`)
- **Non-fungible tokens are a first-class definition type**: `NewTokenDefinition::NonFungible { name, printable_supply }` and `TokenDefinition::NonFungible { name, printable_supply, metadata_id }` — an NFT definition *requires* a metadata account (fungible makes it `Option`). [CODE — 98%] same file
- **An NFT is an account holding a `TokenHolding`** in two roles: `NftMaster { definition_id, print_balance }` (mintable master) and `NftPrintedCopy { definition_id, owned: bool }` (an owned copy). One definition ID per holding account. [CODE — 98%] `token/core/src/lib.rs`; docs https://docs.logos.co/lez/transfer-tokens/create-and-transfer-custom-tokens-on-the-logos-execution-zone
- **Edition printing = Metaplex master/print editions**: `print_nft` requires an authorized master, decrements `print_balance`, writes a new `NftPrintedCopy{owned:true}` (asserts `print_balance > 1`, 1 reserved for master). [CODE — 95%] `token/src/print_nft.rs`
- **NFT supply cannot be minted further** — `mint` panics on `NonFungible`, enforcing scarcity at program level; supply fixed to `printable_supply` at definition. [CODE — 95%] `token/src/mint.rs`
- **Metadata standard exists but is thin**: `TokenMetadata { definition_id, standard, uri, creators, primary_sale_date }`; `standard` = `MetadataStandard::{Simple, Expanded}`, `uri` = pointer to off-chain metadata; `primary_sale_date` stubbed (`0`, `TODO #261`). [CODE — 95%] `token/src/new_definition.rs:108-109`
- **Notes are the L1/Mantle value unit, NOT the NFT unit** — "Bedrock-native fungible tokens," UTXO-based, for gas + bridging. Inherently fungible; an NFT is *not* "a unique note." [CODE — 90%] https://docs.logos.co/blockchain/concepts/about-mantle
- **Inscriptions are the Ordinals-analog**: "Inscription operations write arbitrary data to the Logos Blockchain… a permanent, decentralised record"; Zones "post data as on-chain inscriptions via Logos channels." [CODE — 95%] https://docs.logos.co/blockchain/zone-sdk/inscribe-data-on-chain-using-zone-sdk
- **The team already shipped an ordinals-style collection** — "The Exit," 1,000 ordinals inscribed on Bitcoin. Two NFT construction paths exist: (a) LEZ token-program NFT (state, transferable, editions), (b) raw inscription (immutable data record). [CODE — 85%] https://blog.logos.co/article/2024-roundup
- **Three ways to represent an NFT, by fit:** (1) LEZ token-program `NonFungible` + `PrintNft` (intended, structured); (2) raw Zone-channel inscription (immutable, Ordinals-like, no transfer semantics); (3) bespoke LEZ program with custom account data. [H — 75%]

## Privacy primitives — and what each buys an NFT
- **Every account is public OR private**, sharing one program/state model ("cleanly separates public and private state while keeping them fully interoperable"). An NFT is just a holding account → **can be minted into / transferred to a private account with no special support** — privacy is a property of the account, not the asset. [CODE — 90%] https://docs.logos.co/lez
- **A private account is a shielded note**: `Commitment = SHA256(prefix ‖ account_id ‖ program_owner ‖ balance ‖ nonce ‖ SHA256(data))`, spent via a `Nullifier`, plaintext never on-chain. [CODE — 92%] `lee/state_machine/core/src/commitment.rs`, `nullifier.rs`
- **Account contents encrypted per-output + post-quantum**: ChaCha20 under a shared secret via **ML-KEM-768** (`EncryptedAccountData { ciphertext, epk, view_tag }`). [CODE — 90%] `lee/state_machine/core/src/encryption/mod.rs`
- **A viewing key (`vpk`) governs who can decrypt** — private accounts derive from `(npk, vpk, identifier)`. This is the built-in hook for **selective disclosure**: share your viewing key → someone can read that account's history; withhold → they cannot. [CODE — 85%] `lee/state_machine/core/src/nullifier.rs`
- **State transitions are zk-proven, not re-executed publicly** — private updates run a `privacy_preserving_circuit` outputting `{public_pre/post_states, encrypted_private_post_states, new_commitments, new_nullifiers}`; the chain sees only ciphertext + commitments + nullifiers. [CODE — 90%] `lee/privacy_preserving_circuit/src/output.rs`, https://docs.logos.co/get-started/glossary
- **Four transfer modes**: public→public, private→private, **shielded** (public→private), **deshielded** (private→public) — an NFT can move in/out of the shielded set. [CODE — 90%] https://docs.logos.co/lez/get-started/run-lez-wallet-ui-and-initiate-native-token-transfers
- **Group-owned private accounts**: from one Group Master Secret each member derives the same keys → an NFT jointly + privately controlled by a group. [CODE — 80%] https://docs.logos.co/lez/accounts/set-up-shared-private-lez-account
- **Blend unlinks who-did-what at the network layer** — hides the block-proposer↔proposal link via layered encryption + random-delay paths + cover traffic, atop Cryptarchia (Private PoS). Defends the network-metadata deanonymization vector on-chain privacy alone leaves open. [CODE — 90%] https://docs.logos.co/blockchain/concepts/about-the-blend-network
- **Wallets recover private assets by scanning**: the indexer publishes `encrypted_private_post_states` + `new_commitments` + `new_nullifiers`; `view_tag` lets a wallet cheaply filter blobs addressed to it → private NFTs recoverable without a server knowing you own them. [CODE — 85%] `lez/indexer/service/protocol/src/lib.rs:234-236`

## The Logos differentiator (hypothesis, ranked)
What a "Logos NFT" could do that ETH/SOL (transparent) and BTC ordinals (public inscriptions) **cannot**:

1. **Private ownership — no public holder.** NFT lives in a shielded account; chain stores a commitment + ciphertext, never the owner's address nor "address X owns NFT #7." Strongest, best-supported claim. [CODE — 88%]
2. **Unlinkable transfer history.** Each private transfer nullifies the old commitment + emits a fresh one; no on-chain edge connecting sender→recipient, and Blend removes the network edge. ETH/SOL/BTC provenance is a fully public graph. [CODE — 82%]
3. **zk / viewing-key selective disclosure of authenticity or traits.** With circuit work: prove "I hold a printed copy of definition D" or "I own a token with trait T" without revealing which account/copy/who — grant one auditor a viewing key for one asset only. Primitives (vpk, commitments, RISC0) exist; trait-proof circuits do NOT yet. [H — 65%] — **a build, not a shipped feature.**
4. **Private-by-default provenance with opt-in reveal.** Default shielded; deshield to make an item publicly verifiable at sale, or shielded-transfer to keep a gift private. No other chain offers this toggle natively. [CODE — 75%]
5. **Group-private ownership + post-quantum confidentiality.** A collection co-owned privately by a DAO via GMS; notes already ML-KEM-768 (PQ-safe). [CODE — 70%]
6. **Fixed, program-enforced scarcity with private editions.** `PrintNft` enforced in-circuit, distributed into private accounts → verifiable scarcity without a public holder registry. [CODE — 78%]

## Gaps to build
- **Wallet/ATA layer doesn't handle NFTs** — associated-token-account command prints `"unsupported token type"` for `NftMaster` and `NftPrintedCopy`. NFT mint/print/transfer UX absent from the wallet surface. [CODE — 90%] `lez/wallet/src/cli/programs/ata.rs:208`
- **No trait/attribute standard for zk proofs** — `MetadataStandard` is a bare `{Simple, Expanded}` enum, no JSON schema, `uri` points off-chain. Differentiator #3 needs an on-chain, circuit-legible trait representation that doesn't exist. [CODE — 85%]
- **Metadata is off-chain + unprivate** — `uri`/`creators` sit in a plaintext metadata account; even a privately-held NFT leaks its image/traits via public metadata unless the metadata account itself is made private (untested). [CODE — 80% / H — 60%]
- **Explorer/indexer can't display private holdings** — indexer exposes only ciphertext + commitments + nullifiers for private state; explorer operates on public accounts. A "private NFT gallery" needs client-side viewing-key decoding UX (unbuilt). [CODE — 82%]
- **No marketplace / royalty / primary-sale logic** — `primary_sale_date` stubbed (`TODO #261`); no royalty field, no bid/ask, no escrow/atomic-swap-for-NFT program found. [CODE — 85%]
- **No selective-disclosure API** — viewing keys exist as key material, but no documented "reveal this one asset to this auditor" flow or proof-of-trait endpoint. [H — 55%]
- **Private transfers are slow (local proving)** — docs warn private transfers "may take a few minutes" for local proof generation. [CODE — 80%]
- **Inscription path lacks NFT semantics** — raw inscriptions give immutable data records but no transfer/ownership/edition logic; bridging inscriptions to token-program ownership is unbuilt. [H — 65%]

## Bottom line
Logos is unusually far along on the *hard* part — a shielded, zk-proven account model with a real non-fungible token program and post-quantum note encryption already in-tree. The defensible, buildable differentiator is the **private-ownership + unlinkable-provenance + viewing-key selective-disclosure NFT**; #1 and #2 are near-shippable on existing primitives, while the marquee "prove a rare trait in zk without revealing which token" (#3) needs new trait-schema + circuit work. The missing 20% is all *product* layer (wallet NFT UX, trait standard, private metadata, marketplace, explorer decoding), not cryptographic foundations.

_Key source files: `logos-execution-zone/lez/programs/token/core/src/lib.rs`, `token/src/print_nft.rs`, `token/src/new_definition.rs`, `token/src/mint.rs`, `lee/state_machine/core/src/{commitment.rs,nullifier.rs,account.rs,encryption/mod.rs}`, `lee/privacy_preserving_circuit/src/output.rs`, `lez/indexer/service/protocol/src/lib.rs`, `lez/wallet/src/cli/programs/ata.rs`._
