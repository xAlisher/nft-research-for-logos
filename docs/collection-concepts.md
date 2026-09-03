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

## Honesty
This is a testnet showcase: dev-mode proving, no real market, art/metadata handling still off-chain-plaintext until private metadata lands (concept 6 dependency). The point is to **demonstrate the ownership/provenance/disclosure differentiators**, which are real and now proven, not to ship a market.
