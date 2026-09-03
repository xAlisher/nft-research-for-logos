# Bitcoin NFTs (Ordinals / Runes) — findings

_Research by Sina agent, 2026-09-03. Evidence tags: [CONFIRMED] primary source · [H — X%] hypothesis/inference · [? unknown]._

## Ordinal theory (sat numbering & identity)
- Ordinal theory assigns identities to individual satoshis, letting them be tracked, transferred, and imbued with meaning — the base layer for Bitcoin NFTs. [CONFIRMED] https://docs.ordinals.com/overview.html
- Sats numbered in mining order: first sat of the first block is 0, last is 4,999,999,999 (5B sats = 50 BTC subsidy). [CONFIRMED] https://docs.ordinals.com/faq.html
- Sats move input→output FIFO — a numbering/tracking convention, NOT a Bitcoin consensus rule. [CONFIRMED] https://docs.ordinals.com/overview.html
- Rarity tiers from block cadence: common / uncommon (first of block) / rare (first of difficulty period) / epic (first of halving epoch) / legendary / mythic (genesis). [CONFIRMED] https://docs.ordinals.com/overview.html
- Tracking requires an off-Bitcoin index: `ord` runs alongside Bitcoin Core; the protocol is an interpretation layer invisible to base consensus. [CONFIRMED] https://docs.ordinals.com/overview.html

## Inscriptions (arbitrary on-chain data)
- An inscription attaches arbitrary content (image/text/JSON/HTML/SVG) to a sat — "bitcoin-native digital artifacts, more commonly known as NFTs." [CONFIRMED] https://docs.ordinals.com/inscriptions.html
- Content stored **entirely on-chain** in taproot script-path spend scripts ("envelopes"). No IPFS/Arweave pointer — the bytes live in the chain. [CONFIRMED] https://docs.ordinals.com/inscriptions.html
- Enabled by SegWit (2017) + Taproot (2021): data in **witness** data; the witness discount makes it economical. [CONFIRMED] https://docs.ordinals.com/inscriptions.html
- Created via **commit/reveal** (two txs); inscription made on the first sat of the reveal tx's input. [CONFIRMED] https://docs.ordinals.com/inscriptions.html
- HTML/SVG inscriptions are **sandboxed to block off-chain references** — immutable + self-contained by design. [CONFIRMED] https://docs.ordinals.com/inscriptions.html
- Digital-artifact ethos = 5 properties: owned, **complete** (off-chain-pointer NFTs are "incomplete"), **permissionless** (royalty-gated NFT is disqualified), **uncensorable**, **immutable** (upgrade-key NFT is disqualified). [CONFIRMED] https://docs.ordinals.com/digital-artifacts.html

## Runes / BRC-20 (fungible-token layers)
- **BRC-20**: JSON text (deploy/mint/transfer) inscribed via Ordinals; balances computed by an **off-chain indexer**; multi-tx per op. [CONFIRMED] https://arxiv.org/pdf/2310.10652
- **Runes** (Casey Rodarmor, launched block 840,000 / 2024-04-20): UTXO-native, uses **OP_RETURN** for metadata, self-contained, does **not** rely on ordinals/inscriptions. [CONFIRMED] https://www.kraken.com/learn/bitcoin-runes-protocol
- Efficiency: Runes = single tx + compact OP_RETURN (~80 bytes) vs BRC-20's multi-tx + up-to-4MB witness — Runes put less bloat pressure. [CONFIRMED] https://rocknblock.medium.com/exploring-bitcoin-inscriptions-brc-20-and-runes-protocol-1b7f01f8572c
- Division of labor: inscriptions/Ordinals → non-fungible; Runes & BRC-20 → fungible. [CONFIRMED] https://pixelplex.io/blog/bitcoin-runes/

## Provenance & ownership
- Ownership = control of the UTXO holding the inscribed sat; transfer = moving that sat in a normal tx. [CONFIRMED] https://docs.ordinals.com/inscriptions.html
- Provenance fully auditable — content + history are on-chain and public forever. [CONFIRMED] https://www.unchained.com/blog/bitcoin-inscriptions-ordinals
- **Wallet gotcha:** a normal wallet treats sats as identical → can spend the inscribed UTXO as fee/change = permanent loss, no warning. Mitigation = sat-control / UTXO locking in ordinals-aware wallets. [CONFIRMED] https://docs.ordinals.com/guides/collecting.html

## Strengths vs limitations
- **Strength:** true on-chain permanence + no external hosting to rot/censor. [CONFIRMED] https://docs.ordinals.com/digital-artifacts.html
- **Strength:** no smart-contract complexity — "Inscriptions do not require writing or understanding smart contracts." [CONFIRMED] https://docs.ordinals.com/faq.html
- **Limitation:** no programmability — anything dynamic lives in off-chain indexers/marketplaces. [CONFIRMED] https://docs.ordinals.com/faq.html
- **Limitation:** no enforceable royalties — "technically infeasible." [CONFIRMED] https://docs.ordinals.com/faq.html
- **Limitation:** cost + block-size pressure; high/volatile fees; lasting chain bloat. [H — 85%] https://arxiv.org/pdf/2310.10652
- **Limitation:** clunky UX (commit/reveal, indexer dependence, accidental-spend hazard). [H — 90%] https://docs.ordinals.com/guides/collecting.html

## Privacy posture
- **No privacy by design.** Inscription content stored on-chain in the clear; viewable by anyone forever. [CONFIRMED] https://docs.ordinals.com/inscriptions.html
- Ownership + full transfer history transparent via the public UTXO graph + sat tracking. [CONFIRMED] https://docs.ordinals.com/overview.html
- The docs treat public visibility as a **feature** (uncensorability) and do not address privacy at all — no confidentiality primitive exists. [CONFIRMED] https://docs.ordinals.com/digital-artifacts.html

## Privacy gaps (relevant to Logos)
- **Content is public forever:** exact NFT bytes on-chain in plaintext — no encryption/selective disclosure/redaction. Logos could inscribe **commitments/encrypted blobs** with viewing-key or ZK-gated reveal. [H — 90%] https://docs.ordinals.com/inscriptions.html
- **Ownership graph fully linkable:** transfers expose the address/UTXO trail. Logos could break the link with shielded transfers / nullifier-based ownership while still proving provenance. [H — 85%] https://docs.ordinals.com/overview.html
- **No enforceable creator terms:** royalties impossible on Bitcoin (deliberately). A programmable-yet-private zone could enforce policy without leaking holders/traders. [CONFIRMED (BTC side)] https://docs.ordinals.com/faq.html
- **Immutability without privacy = permanent exposure:** anything inscribed (incl. accidental PII) is unremovable + world-readable. Privacy-first default should be an encrypted/committed artifact with owner-controlled disclosure. [H — 90%] https://docs.ordinals.com/digital-artifacts.html

## Inscription parallel to Logos
- **Same core idea:** Ordinals inscriptions = arbitrary data written fully on-chain, immutable, self-contained (no off-chain pointers). Logos inscriptions post data on-chain via **Zone channels** — the same "the data IS on-chain, not a hash of it" property → strongest permanence/completeness guarantee. [CONFIRMED for Ordinals] https://docs.ordinals.com/inscriptions.html
- **Key divergence Logos can own — privacy.** Bitcoin's inscription model is public-by-construction with zero confidentiality; Logos (privacy-first) can offer what Ordinals structurally cannot: **encrypted/committed inscriptions with ZK-verifiable provenance, unlinkable ownership transfer, selective disclosure** — matching permanence while removing "everything public forever." [H — 85%]
- **Also differentiable:** Zone-level programmability could enable enforceable creator terms (infeasible on Bitcoin) and avoid the base-chain fee/block-size pressure + accidental-sat-spend UX trap. [H — 80%] https://docs.ordinals.com/faq.html

## Sources
Overview https://docs.ordinals.com/overview.html · Inscriptions https://docs.ordinals.com/inscriptions.html · Digital Artifacts https://docs.ordinals.com/digital-artifacts.html · FAQ https://docs.ordinals.com/faq.html · Collecting/Wallet https://docs.ordinals.com/guides/collecting.html · Unchained https://www.unchained.com/blog/bitcoin-inscriptions-ordinals · Kraken Runes https://www.kraken.com/learn/bitcoin-runes-protocol · Spark Runes https://www.spark.money/research/runes-protocol-fungible-tokens-bitcoin · Rock'n'Block https://rocknblock.medium.com/exploring-bitcoin-inscriptions-brc-20-and-runes-protocol-1b7f01f8572c · PixelPlex https://pixelplex.io/blog/bitcoin-runes/ · arXiv BRC-20 https://arxiv.org/pdf/2310.10652
