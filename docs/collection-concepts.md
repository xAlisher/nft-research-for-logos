# Testnet NFT collection concepts — showcasing what a Logos NFT can do

_2026-09-03. Concepts for a testnet collection whose purpose is to **demonstrate the Logos NFT differentiators** — not art on a chain, but a collection whose mechanics are impossible or degraded on ETH/SOL/BTC. Grounded in primitives now proven live (see [`../experiments/`](../experiments)): public mint/print/transfer (A1–A4), shield → private-transfer → deshield (B1–B4), and the circuit-level private proof (B0). Selective disclosure (viewing keys) is Epic C._

## Design rule
Every concept must earn its place: it exercises a **specific differentiator** on a **specific primitive**, and it fails or degrades on a transparent chain. Ranked by (a) fit to the Logos ethos — anti-surveillance, self-sovereignty, "parallel society" — and (b) buildability on what's proven today.

## Concepts

| # | Collection | Differentiator | Primitive it exercises | Impossible/degraded elsewhere because | Buildable now? |
|---|---|---|---|---|---|
| 1 | **Sealed** — each piece mints hidden; the collector alone chooses when to publicly reveal it | Shield/deshield "reveal on your terms" | `send` public→private (shield) then private→public (deshield) — B1/B3 ✅ | ETH "reveals" are a centralized server flipping a URI; here the reveal is the owner's un-frontrunnable on-chain act | **Yes** |
| 2 | **Ghost Holders** — art whose value narrative is that no one can see who holds it | Private ownership | NFT in a shielded account (commitment + ML-KEM ciphertext) — B1 ✅ | `ownerOf`/token-account/holding-UTXO are public everywhere; the holder graph is the analytics product | **Yes** |
| 3 | **Whisper** — a private-gifting collection; recipients discover gifts by scanning | Unlinkable provenance | private→private `send` (no public sender→recipient edge) — B2 ✅ | every chain records "X sent Y this NFT" permanently | **Yes** |
| 4 | **Keyholder** — membership/access passes; prove you hold one (or a tier) to a single verifier without revealing which token or your wallet | Selective disclosure | viewing-key export + verifier — **Epic C (#3)** | ETH token-gating forces exposing your whole wallet to prove one holding | Needs Epic C |
| 5 | **Cabal** — relics co-owned by a guild via a Group Master Secret; prove membership-ownership without revealing the roster | Group-private ownership | shared private account (GMS) | no public chain has group-private co-ownership as a primitive | Later (GMS) |
| 6 | **Cicada** — an ARG/puzzle where clue inscriptions are encrypted on-chain, revealed selectively | Encrypted on-chain content + progressive reveal | private metadata / inscriptions + viewing keys | Bitcoin Ordinals are on-chain but plaintext forever; can't encrypt | Later (private metadata) |
| 7 | **Blind Editions** — editions printed straight into private accounts, so the distribution itself is hidden until reveal | Program-enforced scarcity + private editions | `print-nft` into a private account | edition distribution is fully public elsewhere | Mostly (print→private) |

## Recommendation for the testnet showcase

**Lead: "Sealed" (concepts 1+2 together).** Its *lifecycle is the demo* — mint public → shield (now a Ghost Holder, no public owner) → optionally private-transfer as a gift → deshield (the reveal moment). One collection walks a viewer through every privacy primitive we've proven, in a story legible to non-crypto audiences ("a sealed envelope only you can open, on-chain"). Buildable today; no Phase-2 work.

**Pair: a small "Keyholder" tier** to tease selective disclosure once Epic C lands — a handful of passes that gate a demo resource, proving ownership to a verifier without doxxing. This previews the marquee differentiator and gives a live "prove-without-revealing" beat for the Builder Meetup.

**Lore/positioning:** frame the pieces as *sovereign records* disclosed only under the holder's own authority — the privacy is the aesthetic and the argument for the stack, not a footnote. Fits the parallel-society / network-state theme directly.

## Suggested build path (testnet, no real value)
1. **Sealed MVP** on today's primitives: a small set (e.g. 10 pieces), each mint→shield; a demo of shield/gift/deshield; `account get` renders holdings. Reuses `walk.sh` / `walk-b.sh` patterns.
2. **Keyholder tier** after Epic C: viewing-key reveal to a verifier gating a resource.
3. Optional: **Blind Editions** via `print-nft` into private accounts.

Deliberately excludes: real marketplace, real value, and the Phase-2 zk-trait proof (own a rare trait without revealing which token) — those are follow-ons, noted so the demo stays honest about scope.

## Flagship "Sealed" — art direction & public-facing mechanics

Design problem: privacy means the public **can't** see ownership — so the mechanics must make the **absence** and the **reveal** tangible, not hide them. Every beat maps to a primitive proven live (see [`../experiments/`](../experiments)).

### Art direction — the sealed↔revealed duality is the aesthetic
Each piece has **two visual states**, both collectible-looking:
1. **The Seal (sealed state) — a generative sigil from the commitment hash.** While shielded, the public sees only a unique glyph (wax-seal / sigil / guilloché) deterministically generated from the on-chain commitment. Beautiful on its own, so "sealed" reads as intentional. Cheap + public **on-chain** (a small commitment-derived SVG), while the real art stays **encrypted off-chain** (Codex/IPFS — see [`../research/media-storage.md`](../research/media-storage.md)).
2. **Redacted-dossier aesthetic (recommended).** Pieces look like classified records (redaction bars, "CLASSIFIED / EYES ONLY", a network-state seal) that the holder **declassifies** (deshields) on their own authority. On-brand for the parallel-society / anti-surveillance ethos: privacy *is* the aesthetic.
3. **Fog of ownership.** A veiled frame; you can tell something is there, not what or whose. Only the owner's viewing key renders it clear.

The **reveal (deshield)** is the artistic climax: the seal dissolves into the real artwork, on-chain and un-frontrunnable.

### Public-facing mechanics (ranked)
1. **The Sealed Gallery + live reveal counter (hero).** Public site shows every piece as its seal, "N of M declassified." An owner deshields → their piece flips seal→art **live**, but the public never learns *who* held it. Aggregate visible; ownership private. → deshield (Epic B3).
2. **The "Prove It" wall — the C3 verifier as a public toy.** A widget where an owner supplies a viewing key (paste/QR); it confirms "this holder owns a Sealed piece" **without revealing wallet or which token.** Literally `wallet verify-disclosure` behind a web form (proven live). → Epic C.
3. **The absence exhibit (contrast-as-mechanic).** Side-by-side: "what a normal NFT explorer shows" (full holder graph, whale wallets) vs "what Logos shows" (nothing). The *missing surveillance* is the headline exhibit. → private ownership (Epic B1).
4. **Whisper (private gifting).** Shield-gift a piece to a friend; recipient discovers it by scanning. Public sees "N whispers sent," never sender→recipient. → unlinkable transfer (Epic B2).
5. **Progressive / selective reveal.** Owner reveals *one* trait publicly ("it's rare") while keeping the art sealed. Approximated today via viewing-key trait reveal; full "prove a rare trait without revealing which token" is the Phase-2 zk build.

### Hero combination for the demo
**Sealed Gallery (live reveals) + Prove-It wall + absence exhibit.** The flow *is* the argument: a wall of sealed records → owners declassify on their terms → here's what you (and no one) can see about who holds them → yet a holder can still prove ownership to whoever they choose.

### Technical mapping (so art ≠ vaporware)
- Sealed sigil = on-chain, derived from the commitment (public, cheap).
- Real art = encrypted media on Codex/IPFS; CID in the token `uri`; symmetric key wrapped to the owner's viewing/sealing key.
- Reveal = deshield (public) or viewing-key decode (selective); Prove-It = `verify-disclosure` (read-only viewing key).
- Caveat: Codex durability is best-effort on testnet today (mirror to Arweave/pinned-IPFS for permanence).

### Lore / positioning
Parallel-society framing: the pieces are **sovereign records** disclosed only under the holder's own authority. Declassification, not a "reveal button." The privacy is the story, and the collection is an argument for the stack.

## Honesty
This is a testnet showcase: dev-mode proving, no real market, art/metadata handling still off-chain-plaintext until private metadata lands (concept 6 dependency). The point is to **demonstrate the ownership/provenance/disclosure differentiators**, which are real and now proven, not to ship a market.
