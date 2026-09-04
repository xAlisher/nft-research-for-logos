# NFT on Logos: overview and dogfooding findings

> **Disclaimer.** This is an independent community R&D / proof-of-concept, developed to demonstrate capabilities of the Logos technology stack. It is not built for, on behalf of, or as part of the work of Logos or the Institute of Free Technology, and has not been reviewed, audited, approved, or endorsed by Logos or IFT. Dev-mode / testnet only.

**Goal of this work:** dogfooding, i.e. actually using the Logos stack to build NFTs and report findings and insights.

**Repos:**
- Research + findings (this repo): https://github.com/xAlisher/nft-research-for-logos
- PoC Basecamp app ("Sealed records"): https://github.com/xAlisher/sealed-keys-basecamp

## What we explored and analysed
- Can Logos do NFTs? How well? What is missing?
- Compared Logos with Ethereum, Solana, and Bitcoin Ordinals.
- Read the Logos code, the official docs, and ran the stack live.

## What Logos already has
- The blockchain already knows what an NFT is.
- The token program supports one-of-a-kind tokens (create, numbered copies, transfer).
- Logos also has private, shielded accounts built in.
- So the hard cryptographic part is done. Code: https://github.com/logos-blockchain/logos-execution-zone (token program under `lez/programs/token/`).

## Where the gaps are
- **The wallet.** The LEZ CLI wallet could not drive NFTs.
  - It errored ("unsupported token type") when you owned one (`lez/wallet/src/cli/programs/ata.rs`).
  - No commands to create, print, or trade NFTs.
  - Trading was broken: a transfer got rejected by the receiver.
- **The docs.** There are no official NFT docs at all. There is a guide for fungible tokens; nothing for NFTs.

## What we forked and added (R&D fork, not upstreamed)
- Forked the LEZ wallet and added the missing commands: show an NFT you own, create an NFT, print copies, and a fix for the broken transfer. Plus commands to seal, unseal, and discover private NFTs.
- Proved each step works live on a local Logos node.

## What we built on top
- A PoC Basecamp app: **Sealed records**.
- A themed test collection: the **Museum of Civil Liberties** (declassified documents).
- Each NFT looks like a sealed, redacted paper. Owner is hidden. You click to unseal with your private key and the real document appears.
- Ran end to end inside an isolated Basecamp, live against a testnet node.

## Private NFT ownership on Logos
On other chains the public ledger says "wallet 0xABC owns NFT #5" and everyone can read it. On Logos, the ledger only stores a scrambled fingerprint of your NFT, a locked box. It does not say who owns it, or what is inside. Only you hold the key.

- **Visible on-chain:** that a private transaction happened, plus scrambled fingerprints ("commitments"). Meaningless to outsiders.
- **Hidden:** who owns it (no wallet, no name), what it is (title, content, link are encrypted), and the trail of who gave it to whom (transfers do not link up).
- **What the owner can prove, and how:** the owner holds a viewing key. They can produce a proof that says "I own this NFT" to one specific verifier, without exposing their wallet or their other holdings. They can reveal one item to one person and keep the rest sealed. Proving it to you does not broadcast it to the world.

Evidence in this repo:
- Private ownership proven at the circuit level: [`experiments/nft-privacy-proof/PROOF.md`](../experiments/nft-privacy-proof/PROOF.md)
- Selective disclosure (prove/reveal with a viewing key): [`experiments/nft-selective-disclosure/`](../experiments/nft-selective-disclosure/)
- Why it works: Logos hashes opaque account data, so an NFT shields exactly like a private balance (`lez/state_machine/core/src/commitment.rs` upstream).

## What makes NFT on Logos different
- On every other major chain, NFT ownership is public. Anyone sees who owns what.
- On Logos, you can own an NFT privately: own it privately, transfer it without a trace, reveal on your terms.
- No other major chain offers private-by-default NFT ownership.

## When someone would pick Logos over other chains
- When they do not want ownership to be public.
- Private membership or access passes.
- Sensitive credentials or certificates.
- Collections where the holder wants to stay anonymous.
- Archival or evidence records revealed on demand.
- Any case where "who owns this" should be a secret until the owner decides.

## Insight: this de-risks Lambda Prize LP-0001
- LP-0001 is "Private NFT Ownership Proof": prove you own some NFT in a collection without revealing which one (private token-gating). https://github.com/logos-co/lambda-prize/blob/master/prizes/LP-0001.md
- Its status is draft, "pending NFT Program readiness". It has been blocked on exactly the readiness question tested here.
- This dogfooding shows the substrate is ready: the NFT program works, private ownership works, and viewing-key disclosure works.
- Delta to LP-0001's goal: we proved "I own this NFT, revealed to you, wallet hidden." LP-0001 wants one level up, "I own one of these, but not which", a set-membership proof over the collection plus a nullifier to stop replay.
- LP-0001 also asks for a module/SDK and a Basecamp app GUI. The PoC app is a working reference for that shape.

## Status
Research and PoC complete; demonstrated live on a local testnet node. Tracking issue: https://github.com/logos-co/ecosystem/issues/232
