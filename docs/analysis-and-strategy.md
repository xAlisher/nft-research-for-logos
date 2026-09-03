# NFTs on Logos — comparative analysis & strategy

_Synthesis of the four research streams in [`../research/`](../research). Every load-bearing claim traces to a tagged finding there ([CODE]/[CONFIRMED]/[H]). 2026-09-03._

---

## 1. Executive summary

Every major NFT ecosystem — Ethereum, Solana, Bitcoin Ordinals — is **public by construction on the one axis that matters most for a collectible: who owns it.** `ownerOf` on Ethereum, the token account on Solana, the holding UTXO on Bitcoin — all openly queryable, all permanently linkable to a real identity once a single address is deanonymized. The industry's privacy work (EIP-5564 stealth addresses, Solana Token-2022 confidential transfers) hides *recipients* or *amounts* but **never the ownership link itself**, and none of it applies to NFTs in practice.

Logos already ships the missing piece: a **shielded, zk-proven account model** where any token holding — including a first-class `NonFungible` token — can live in a private account whose owner, balance, and history are commitments + post-quantum ciphertext, never plaintext on-chain. The non-fungible token program (`NewTokenDefinition::NonFungible`, `PrintNft` editions, fixed scarcity) is **already in-tree**.

**The differentiator, in one line:** *the first NFT you can own privately by default, transfer without leaving a public provenance graph, and prove you own only when and to whom you choose.*

The cryptographic foundations exist. What is missing is entirely **product layer** — wallet NFT UX, a trait schema, private metadata, a selective-disclosure flow. That is the buildable frontier, and the MVP ([`mvp.md`](mvp.md)) takes the sharpest, best-supported slice of it.

---

## 2. Comparative matrix

| Axis | Ethereum (ERC-721/1155) | Solana (Metaplex / cNFT) | Bitcoin (Ordinals) | **Logos (LEZ)** |
|---|---|---|---|---|
| Ownership | Public `ownerOf` [CONFIRMED] | Public token account [CONFIRMED] | Public holding UTXO [CONFIRMED] | **Private by default (shielded account)** [CODE — 88%] |
| Transfer history | Fully public graph [CONFIRMED] | Fully public [CONFIRMED] | Fully public [CONFIRMED] | **Unlinkable (nullifier↔commitment + Blend)** [CODE — 82%] |
| Metadata | Off-chain (IPFS/central); "rug" risk [CONFIRMED] | Off-chain + on-chain PDA [CONFIRMED] | **Fully on-chain, immutable** [CONFIRMED] | Off-chain URI today; private-metadata is a gap [CODE — 80%] |
| Scarcity guarantee | Contract-enforced [CONFIRMED] | Program-enforced (Master Edition) [CONFIRMED] | Sat-level; indexer-tracked [CONFIRMED] | **Program-enforced (`mint` panics on NFT)** [CODE — 95%] |
| Editions | ERC-1155 copies [CONFIRMED] | Master/Print editions [CONFIRMED] | 1-of-1 per sat [CONFIRMED] | **`PrintNft` master/printed-copy** [CODE — 95%] |
| Royalties | Signal-only; enforcement collapsed [CONFIRMED] | Enforced via pNFT rule sets [CONFIRMED] | Impossible (by design) [CONFIRMED] | None yet; enforceable-in-transfer is a build [CODE — 85%] |
| Selective disclosure | None | Auditor-decrypts-all (fungible) [CONFIRMED] | None | **Per-asset viewing key (`vpk`)** [CODE — 85%] |
| Cost model | Gas-variable; L2s ~$0.01–0.5 [H] | ~0.0029 SOL; cNFT ~290x cheaper [CONFIRMED] | High/volatile; block-size pressure [H] | zk local-proving latency (minutes) [CODE — 80%] |
| PQ-safety | No | No | No | **ML-KEM-768 note encryption** [CODE — 90%] |

**The through-line:** on the three incumbents, transparency is the default and cannot be opted out of. On Logos, privacy is the default and *disclosure* is the opt-in — the inverse posture, and the whole point.

---

## 3. What each incumbent teaches us

- **Ethereum** — the standards playbook (ERC-721/1155/6551, EIP-2981) is the interoperability baseline every wallet/marketplace expects; match its *interface shape* so Logos NFTs feel familiar. Its royalty collapse (OpenSea→Blur) proves that **enforcement must live in the transfer primitive, not marketplace goodwill** — a privacy zone that mediates transfer can do what transparent chains can't. [`../research/eth-nfts.md`]
- **Solana** — compressed NFTs (on-chain root + off-chain data via DAS) prove the ecosystem tolerates a **commitment-on-chain / data-off-chain** pattern at scale — the same shape as Logos's commitment + encrypted note, but Logos's off-chain part is *encrypted*, not merely off-chain. Also the cautionary tale: cNFTs created a hard RPC/indexer centralization dependency — **avoid re-creating that trust surface.** [`../research/sol-nfts.md`]
- **Bitcoin Ordinals** — the purest "data IS on-chain" model and the strongest permanence guarantee; also the starkest privacy failure (everything public + immutable *forever*, incl. accidental PII). Logos has its own inscription primitive (Zone channels) and the team already shipped an ordinals collection ("The Exit"). The divergence Logos can own: **encrypted/committed inscriptions with ZK-verifiable provenance** — Ordinals' permanence without its permanent exposure. [`../research/btc-ordinals.md`]

---

## 4. The Logos differentiators (ranked by buildability × uniqueness)

From [`../research/logos-primitives.md`], ranked:

1. **Private ownership — no public holder.** [CODE — 88%] Near-shippable on existing primitives. *Strongest and best-supported.*
2. **Unlinkable transfer history.** [CODE — 82%] Rides on #1 (every private transfer is inherently unlinkable) + Blend at the network layer. Near-shippable.
3. **Private-by-default provenance with opt-in reveal (shield/deshield toggle).** [CODE — 75%] The disclosure UX is the build.
4. **zk / viewing-key selective disclosure of traits** ("prove I own a rare trait without revealing which token or who"). [H — 65%] The marquee demo, but needs a new trait schema + circuit — *not* MVP.
5. **Group-private ownership + PQ confidentiality.** [CODE — 70%] Niche, genuinely unique, later.
6. **Program-enforced scarcity with private editions.** [CODE — 78%] Comes for free with #1 + `PrintNft`.

**Strategic read:** #1 + #2 + #3 form one coherent, near-shippable story — *own privately, transfer unlinkably, reveal on your terms*. #4 is the headline that earns attention but is a second-epic build. #5/#6 are follow-ons. The strategy is to **ship the coherent near-term story first (it is already a world-first), then extend to the marquee zk-trait proof.**

---

## 5. Broad strategy (phased)

- **Phase 0 — dogfood the existing NFT program (unblock the surface).** The `NonFungible` program + `PrintNft` exist but the wallet ATA layer returns `"unsupported token type"` [CODE — 90%]. First move: make an NFT actually mintable/printable/transferable from the wallet, publicly, end-to-end. This is pure dogfooding and directly serves Franck's assignment (Journey logos-docs#454). *Prerequisite for everything.*
- **Phase 1 — the private-ownership MVP (the differentiator).** Mint an NFT into a shielded account, transfer it privately (unlinkable), and produce a viewing-key reveal so a chosen party — or the public, via deshield — can verify ownership/authenticity. This is the world-first and the [`mvp.md`](mvp.md) scope.
- **Phase 2 — selective trait disclosure (the marquee).** Add a circuit-legible trait schema + a zk proof-of-trait ("I own a token with trait T, without revealing which/who"). This is differentiator #4 and needs new circuit work.
- **Phase 3 — enforcement + ecosystem.** Royalty/primary-sale logic in the transfer primitive (the thing transparent chains failed at), private-metadata storage, explorer viewing-key decoding, marketplace/atomic-swap-for-NFT.

Sequencing rationale (🐲 attack the unprepared point; secure the ground before advancing): incumbents are undefended on private ownership and Logos is already armed there → strike Phase 1 first. But Phase 0 is the ground you must hold before you can advance — an NFT you can't mint from the wallet can't be owned privately.

---

## 6. Risks & honest caveats

- **Latency:** private transfers take minutes (local zk proving) [CODE — 80%] — a real UX cost; frame the demo around it, don't hide it.
- **Metadata leak:** even a privately-held NFT leaks its image/traits via the plaintext off-chain metadata account unless metadata is itself made private [CODE — 80% / H — 60%] — Phase 1 must at minimum acknowledge this; full private metadata is Phase 3.
- **Indexer/explorer blindness:** private holdings can't be displayed without client-side viewing-key decoding [CODE — 82%] — the MVP's "reveal" step is partly a workaround for this and partly the product.
- **Trait-proof is not free:** differentiator #4 is [H — 65%], a genuine build — do not promise it as near-term.
- **This is testnet:** no real value, no real marketplace — the deliverable is a *proof + narrative*, not a shipped market.

---

_Next: [`mvp.md`](mvp.md) — the narrowed, buildable MVP highlighting differentiator #1 (private ownership)._
