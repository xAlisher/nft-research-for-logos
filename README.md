# NFT Research for Logos

> **Disclaimer.** This is an independent community project intended to demonstrate some of the capabilities and potential uses of the Logos technology stack. It has been developed independently by its contributor(s) and is not built for, on behalf of, or as part of the work of Logos or the Institute of Free Technology. It has not been reviewed, audited, approved, or endorsed by Logos or the Institute of Free Technology. The project, including its code, documentation, views, and functionality, is the sole responsibility of its contributor(s) and should not be attributed to Logos or the Institute of Free Technology.
>
> Exploratory R&D only — test / dev-mode, throwaway keys, a local testnet node. Do not use in production. Analysis of upstream code refers to the public [logos-blockchain/logos-execution-zone](https://github.com/logos-blockchain/logos-execution-zone).

Deep research into NFTs on Logos: comparative analysis (Ethereum, Solana, Bitcoin Ordinals), Logos' differentiator (**private-by-default ownership**), a buildable MVP, and hands-on dogfooding findings from actually running the stack. Companion PoC app: [xAlisher/sealed-keys-basecamp](https://github.com/xAlisher/sealed-keys-basecamp).

## The finding, in one line
Every incumbent (ETH/SOL/BTC) is public-by-construction on **who owns a token**. Logos already ships shielded, zk-proven accounts + a first-class `NonFungible` token program in-tree → the differentiator is **the first NFT you can own privately, transfer unlinkably, and reveal on your terms.**

## Contents
- **[`docs/analysis-and-strategy.md`](docs/analysis-and-strategy.md)** — comparative matrix, what each chain teaches, ranked differentiators, phased strategy, risks.
- **[`docs/mvp.md`](docs/mvp.md)** — the narrowed, buildable MVP (private ownership) → epics A–D → implementation-ready issues.
- **[`docs/verification.md`](docs/verification.md)** — adversarial source-check verdict (every claim confirmed at real file:line; MVP buildable, with the riskiest assumption flagged).
- **[`docs/museum-alignment.md`](docs/museum-alignment.md)** — Sealed as the on-chain wing of Logos's Museum of Civil Liberties (5-exhibit re-curation).
- **[`docs/collection-concepts.md`](docs/collection-concepts.md)** — testnet NFT collection concepts that showcase the differentiators (lead: "Sealed"). Each mapped to a primitive + issue.
- **`research/`** — evidence-tagged findings, source-linked (Sina discipline: `[CODE]`/`[CONFIRMED]`/`[H — X%]`):
  - [`media-storage.md`](research/media-storage.md) — NFT media storage options; [`logos-storage-discovery.md`](research/logos-storage-discovery.md) — Storage discovery/fetch (DHT works, host must stay online); [`eth-nfts.md`](research/eth-nfts.md) · [`sol-nfts.md`](research/sol-nfts.md) · [`btc-ordinals.md`](research/btc-ordinals.md) · [`logos-primitives.md`](research/logos-primitives.md)
- `docs/retro-log.md` — session retro log.

## Status
Research complete + synthesized + source-verified (2026-09-03). Every load-bearing claim confirmed against the actual LEZ source; an independent Codex pass was also launched. Epics/issues filed — see the repo's Issues tab. Ready for implementation prioritization.
