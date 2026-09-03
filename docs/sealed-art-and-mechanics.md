# "Sealed" — art direction & public-facing mechanics

_2026-09-03. Art + interactive mechanics for the flagship testnet collection ([`collection-concepts.md`](collection-concepts.md) #1). Design problem: privacy means the public **can't** see ownership — so the mechanics must make the **absence** and the **reveal** tangible, not hide them. Every beat maps to a primitive proven live (Epics A/B/C; see [`../experiments/`](../experiments))._

## Art direction — the sealed↔revealed duality is the aesthetic

Each piece has **two visual states**, both collectible-looking:

1. **The Seal (sealed state) — a generative sigil from the commitment hash.** While shielded, the public sees only a unique glyph (wax-seal / sigil / guilloché) deterministically generated from the on-chain commitment. Beautiful on its own, so "sealed" reads as intentional, not a placeholder. Cheap + public **on-chain** (a small commitment-derived SVG), while the real art stays **encrypted off-chain** (Codex/IPFS — see [`../research/media-storage.md`](../research/media-storage.md)).
2. **Redacted-dossier aesthetic (recommended).** Pieces look like classified records — redaction bars, "CLASSIFIED / EYES ONLY", a network-state seal — that the holder **declassifies** (deshields) on their own authority. On-brand for the parallel-society / anti-surveillance ethos: privacy *is* the aesthetic.
3. **Fog of ownership.** A veiled/frosted frame; you can tell something is there, not what or whose. Only the owner's viewing key renders it clear.

The **reveal (deshield)** is the artistic climax: the seal dissolves into the real artwork, on-chain and un-frontrunnable.

## Public-facing mechanics (ranked)

1. **The Sealed Gallery + live reveal counter (hero).** Public site shows every minted piece as its seal, with "N of M declassified." When an owner deshields, their piece flips seal→art **live** — but the public never learns *who* held it. Aggregate visible; ownership private. → deshield (Epic B/B3).
2. **The "Prove It" wall — the C3 verifier as a public toy.** A widget where an owner supplies a viewing key (paste/QR) and it confirms "this holder owns a Sealed piece" **without revealing wallet or which token.** Literally `wallet verify-disclosure` behind a web form (proven live: correct key discloses, wrong key doesn't). → Epic C.
3. **The absence exhibit (contrast-as-mechanic).** Side-by-side: "what a normal NFT explorer shows" (full holder graph, whale wallets, floor-sweeping) vs "what Logos shows" (nothing). Make the *missing surveillance* the headline exhibit — the strongest argument for the stack. → private ownership (Epic B/B1).
4. **Whisper (private gifting).** Anyone shield-gifts a piece to a friend; recipient discovers it by scanning. Public sees "N whispers sent," never sender→recipient. → unlinkable transfer (Epic B/B2).
5. **Progressive / selective reveal.** Owner reveals *one* trait publicly ("it's rare") while keeping the art sealed — owner-controlled partial disclosure as a collecting game. Approximated today via viewing-key trait reveal; the full "prove a rare trait without revealing which token" is the Phase-2 zk build.

## Hero combination for the demo
**Sealed Gallery (live reveals) + Prove-It wall + absence exhibit.** The flow *is* the argument: a wall of sealed records → owners declassify on their terms → here's what you (and no one) can see about who holds them → yet a holder can still prove ownership to whoever they choose. The full "own privately, reveal on your terms, prove without revealing" story, interactive — every beat backed by a proven primitive.

## Technical mapping (so art ≠ vaporware)
- Sealed sigil = on-chain, derived from the commitment (public, cheap).
- Real art = encrypted media on Codex/IPFS; CID in the token `uri`; symmetric key wrapped to the owner's viewing/sealing key.
- Reveal = deshield (public) or viewing-key decode (selective).
- Prove-It = `verify-disclosure` (read-only viewing key).
- Honest caveat: Codex durability is best-effort on testnet today (mirror to Arweave/pinned-IPFS if permanence matters).

## Lore / positioning
Parallel-society framing: the pieces are **sovereign records** disclosed only under the holder's own authority. Declassification, not a "reveal button." The privacy is the story, and the collection is an argument for the stack.
