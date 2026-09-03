# MVP — the Private NFT ("own it privately, reveal it on your terms")

_Narrowed from [`analysis-and-strategy.md`](analysis-and-strategy.md). Highlights **differentiator #1: private ownership** (with #2 unlinkable transfer and #3 opt-in reveal riding along). 2026-09-03._

---

## 1. The one-liner

**The first NFT you can own privately.** Mint a 1-of-1 (or a small edition) into a *shielded* LEZ account: the chain records a commitment + post-quantum ciphertext, never "address X owns NFT #7." Transfer it privately — no public sender→recipient edge. Then **prove you own it, to exactly whom you choose**, via a viewing key, or reveal it publicly by deshielding at sale.

Nothing on Ethereum, Solana, or Bitcoin can do this. On all three, ownership is a public primitive. [CONFIRMED across `../research/*`]

## 2. Why this slice (and not the marquee zk-trait proof)

- It sits on primitives that **already exist in-tree**: the `NonFungible` token program + `PrintNft` + shielded accounts + viewing keys + ML-KEM note encryption. [CODE 88–95%, see `../research/logos-primitives.md`]
- The marquee "prove a rare trait without revealing which token" is [H — 65%] — needs a new trait schema + circuit. That is **Phase 2**, deliberately out of MVP scope.
- It directly dogfoods Franck's NFT/token-program assignment and Journey logos-docs#454 (mint & transfer NFTs), turning a docs walk-through into a differentiated demo.

## 3. Scope — what the MVP does (and does not) do

**In scope (the demo path):**
1. Define an NFT (`NewDefinitionWithMetadata` → `NonFungible`) with a real name + metadata URI + creator.
2. Mint/print it (`PrintNft`) into a **private account** (shield or mint-to-private).
3. Transfer it **private→private** at least once (demonstrating an unlinkable hop).
4. **Reveal ownership two ways:** (a) share the account viewing key with one chosen verifier who confirms "this account holds printed copy of definition D"; (b) **deshield** to make ownership publicly verifiable (the "list for sale" moment).
5. A thin **viewing-key verifier** (CLI or minimal UI) that, given a viewing key, decodes and displays the privately-held NFT — the "private gallery, revealed to you" surface.

**Explicitly out of scope (later phases):** zk trait proofs (Phase 2); royalty/primary-sale enforcement (Phase 3); private metadata storage (Phase 3, but *acknowledged* — see risks); a marketplace/atomic-swap; the mac/mobile wallet surface.

## 4. The blocking gap the MVP must clear first

The wallet ATA layer returns `"unsupported token type"` for `NftMaster`/`NftPrintedCopy` (`lez/wallet/src/cli/programs/ata.rs:208-209`) [CONFIRMED, see [`verification.md`](verification.md)]. **Before any privacy story, an NFT must be mintable/printable/transferable end-to-end from the wallet.** This is Epic A below and is a prerequisite; it is also, by itself, exactly the dogfooding deliverable.

**Good news from verification:** the NFT *transfer* logic already exists in the program — `token/src/transfer.rs:72-94` performs the `NftPrintedCopy` ownership flip (`sender_owned=false; recipient_owned=true`), `NftMaster` at :50-54. So Epic A is **wallet plumbing over existing program semantics**, not new program logic. And the privacy layer is account-data-agnostic (commits opaque `account.data`), so an NFT holding shields/transfers privately through the same path as a fungible one — Epic B is de-risked. Full evidence in [`verification.md`](verification.md).

## 5. Epics → issues (implementation-ready)

### Epic A — NFT support in the LEZ wallet (Phase 0, prerequisite)
- **A1.** Handle `NftMaster` / `NftPrintedCopy` in `ata.rs` (remove `"unsupported token type"`); create/inspect NFT holding accounts. _Accept: `wallet ata` shows an NFT holding without error._
- **A2.** Wallet command to define an NFT (`NewDefinitionWithMetadata`, `NonFungible`, name+uri+creators). _Accept: a new NFT definition appears on-chain with a metadata account._
- **A3.** Wallet command to `print_nft` from the master into a target account; enforce `print_balance` semantics. _Accept: a printed copy lands in a public account; `mint` on the NFT still panics (scarcity holds)._
- **A4.** Public→public NFT transfer via the wallet. _Accept: printed copy moves between two public accounts; explorer shows it._
- **A5.** End-to-end dogfooding walk following Journey logos-docs#454, capturing every gap as a doc/issue. _Accept: a written run-log + filed doc-fixes._

### Epic B — Private ownership (Phase 1, the differentiator)
- **B0. (trivial-experiment-first — do this before scoping the rest of B).** Shield ONE printed-copy NFT into a private account and private-transfer it exactly once, headlessly. This proves the NFT-through-privacy path end-to-end (structurally sound but unproven end-to-end — the riskiest assumption, see [`verification.md`](verification.md)). _Accept: one NFT shielded + one private hop, chain shows no owner/edge. If it passes, the rest of B follows with high confidence._
- **B1.** Mint/shield an NFT **into a private account** (public→private "shielded" transfer of a printed copy). _Accept: chain shows only a commitment + ciphertext + nullifier for the holding; no public `ownerOf` equivalent._
- **B2.** Private→private NFT transfer (unlinkable hop). _Accept: no on-chain edge links sender to recipient; recipient's wallet recovers the NFT by scanning `view_tag`-filtered blobs._
- **B3.** Deshield an NFT (private→public) — the "reveal for sale" path. _Accept: ownership becomes publicly verifiable; provenance before the reveal stays unlinkable._
- **B4.** Wallet UX for the shield/deshield toggle + a "private holdings" list decoded with the local viewing key. _Accept: owner sees their private NFT; a third party without the key cannot._

### Epic C — Selective disclosure (Phase 1 tail, the "reveal on your terms")
- **C1.** Export a per-account viewing key scoped to a single NFT holding. _Accept: sharing it lets exactly one verifier read that holding, nothing else._
- **C2.** A minimal **viewing-key verifier** (CLI or thin UI): input a viewing key → decode + display "account holds printed copy of definition D (name, creator)." _Accept: verifier confirms authenticity/ownership from the key alone, without the owner's spend key._
- **C3.** Demo script + recording: mint → shield → private transfer → selective reveal → deshield. _Accept: a reproducible end-to-end narrative for the Builder Meetup / weekly update._

### Epic D — Honest-caveats hardening (cross-cutting, right-sized)
- **D1.** Document + measure the private-transfer proving latency; surface it in the UX rather than hiding it. _Accept: a measured number + a "generating proof…" state._
- **D2.** Acknowledge the metadata-leak caveat in the demo + file the private-metadata follow-up (Phase 3). _Accept: the demo states what is and isn't hidden; a Phase-3 issue exists._

## 6. Success criteria (definition of done for the MVP)
- A user can mint an NFT, move it into a shielded account, transfer it privately at least once, and the chain reveals **no owner and no transfer edge** for the private legs. [core claim]
- A chosen verifier, given only a viewing key, can confirm ownership/authenticity of that one NFT. [selective disclosure]
- Deshielding makes ownership publicly verifiable on demand. [opt-in reveal]
- The whole path is captured as a reproducible demo + a written dogfooding run-log feeding Journey logos-docs#454 and the weekly update.

## 7. What this proves
That Logos is not "another NFT chain" but the **only one where ownership is private by default and disclosure is the deliberate act** — the exact inverse of ETH/SOL/BTC, built on primitives Logos already has. It is a world-first framed honestly (testnet, no real market, measured latency), and it is the on-ramp to the marquee zk-trait proof in Phase 2.
