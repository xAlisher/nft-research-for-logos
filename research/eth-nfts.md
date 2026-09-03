# Ethereum NFTs — findings

_Research by Sina agent, 2026-09-03. Evidence tags: [CONFIRMED] primary source · [H — X%] hypothesis/inference · [? unknown]._

## Token standards

- **ERC-721** is the canonical non-fungible token standard (EIP-721, Jan 2018); each token has a unique `tokenId` and a distinct owner. [CONFIRMED] https://eips.ethereum.org/EIPS/eip-721
- ERC-721 mandates `ownerOf`, `balanceOf`, `transferFrom`/`safeTransferFrom`, `approve`/`setApprovalForAll` + the `Transfer` event; this shared interface lets any wallet/marketplace handle any NFT. [CONFIRMED] https://eips.ethereum.org/EIPS/eip-721
- **ERC-1155 (multi-token / semi-fungible)** lets one contract hold fungible + non-fungible + semi-fungible types, each a unique integer ID with per-holder balances; `safeBatchTransferFrom` batches mint/transfer. [CONFIRMED] https://eips.ethereum.org/EIPS/eip-1155
- "Semi-fungible" = tokens interchangeable while identical (e.g. same-tier tickets) that become unique on redemption. [CONFIRMED] https://ethereum.org/developers/docs/standards/tokens/erc-1155/
- **ERC-6551 (token-bound accounts)** gives every existing ERC-721 NFT its own smart-contract wallet via a registry + deterministic address; the NFT can own ERC-20s, other NFTs, and call contracts. [CONFIRMED] https://eips.ethereum.org/EIPS/eip-6551
- **EIP-2981 (royalty standard)** is a `royaltyInfo(tokenId, salePrice)` interface that *signals* a royalty; payment is **voluntary**, not enforced on-chain. [CONFIRMED] https://eips.ethereum.org/EIPS/eip-2981

## Metadata & storage

- `tokenURI(tokenId)` returns a URI (usually JSON: name/image/attributes); the standard does not constrain where it points or guarantee it won't change. [CONFIRMED] https://eips.ethereum.org/EIPS/eip-721
- Storage splits: **centralized** (link breaks/swaps), **IPFS** (content-addressed, needs pinning), **Arweave** (pay-once permanent), **on-chain** (permanent, gas-expensive). [CONFIRMED] https://chainscorelabs.com/guides/non-fungible-tokens-nfts-digital-art-and-collectibles/nft-standards-and-protocols/setting-up-a-protocol-for-nft-data-storage-strategies
- The **"metadata rug"**: if the contract lets the owner change `tokenURI` or points at mutable/centralized hosting, the image/traits can be altered or vanish after sale. [CONFIRMED] https://rameerez.com/problems-and-technical-nuances-of-nft-immutability-and-ipfs/
- Scale of the problem: ~38.8% of sampled NFTs used IPFS, ~31.7% centralized (AWS/GCP) — a large minority rug-exposed by design. [CONFIRMED] https://arxiv.org/pdf/2408.13281
- Mitigation is opt-in: ERC-3569 (sealed metadata) signals frozen metadata. [CONFIRMED] https://eips.ethereum.org/EIPS/eip-3569

## Provenance & ownership

- Current owner via `ownerOf(tokenId)`; full provenance = replaying the on-chain `Transfer` log (from/to/tokenId, incl. mint from `0x0` and burn to `0x0`). [CONFIRMED] https://eips.ethereum.org/EIPS/eip-721
- History is permanent + append-only — provenance can't be forged or erased (the genuine value prop). [CONFIRMED] https://eips.ethereum.org/EIPS/eip-721
- Same property = ownership + complete transfer history are **fully public**; anyone can trace the chain of custody permissionlessly. [CONFIRMED] https://eips.ethereum.org/EIPS/eip-721

## Marketplaces & royalties

- EIP-2981 royalties are unenforceable at protocol level. OpenSea shipped an **Operator Filter** (blocklist of non-royalty marketplaces). [CONFIRMED] https://eips.ethereum.org/EIPS/eip-2981
- **Blur** overtook OpenSea by making royalties optional, bypassing the filter via Seaport. [CONFIRMED] https://ambcrypto.com/opensea-to-suspend-operator-filter-as-blurs-ascendency-gains-momentum/
- OpenSea capitulated (Aug 2023): sunset the Operator Filter, moved to **optional** creator fees — the collapse of on-marketplace royalty enforcement. [CONFIRMED] https://cointelegraph.com/news/opensea-disable-on-chain-royalty-enforcement-tool
- Root cause: royalties are a social convention on a permissionless transfer primitive; any marketplace ignoring them wins on price. [H — 90%] https://roverx.io/blog/nft-creators-suffer-as-opensea-and-blur-fight-over-creator-royalties/

## Privacy posture

- Public by default: owner address, every NFT an address holds, the full transfer graph, and (via marketplace data) sale prices are openly queryable. [CONFIRMED] https://eips.ethereum.org/EIPS/eip-721
- Addresses are pseudonymous but persistent — one deanonymizing link (ENS, KYC'd withdrawal, public wallet) exposes all holdings + history. [H — 90%] https://eips.ethereum.org/EIPS/eip-5564
- **EIP-5564 (stealth addresses)** = the main native privacy attempt: sender derives a fresh one-time address for the recipient from a published meta-address. [CONFIRMED] https://eips.ethereum.org/EIPS/eip-5564
- Pairs with **ERC-6538 (stealth meta-address registry)**; recipients scan announcements via view tags. [CONFIRMED] https://eips.ethereum.org/EIPS/eip-6538
- Stealth addresses hide only the **recipient**; sender, timing, and fact/amount stay public, and gas-funding can re-link — partial, not zk-grade shielding. [H — 80%] https://arxiv.org/pdf/2308.01703

## Limitations & costs

- Mainnet mint/transfer = a full contract tx; historically a few dollars to $50+ (higher at peak). [CONFIRMED] https://www.nadcab.com/blog/gas-optimization-technique-nft
- ERC721A cuts batch-mint gas up to ~80% by deferring per-token bookkeeping. [CONFIRMED] https://www.alchemy.com/blog/erc721-vs-erc721a-batch-minting-nfts
- Scaling pushed to **L2s**: optimistic ~$0.10–0.50, ZK-rollups ~$0.01–0.20, Polygon PoS ~$0.001–0.05 — 90%+ cheaper. [H — 85%] https://cryptoexplained.substack.com/p/what-is-layer-2-in-crypto
- Dencun (Mar 2024, EIP-4844 blobs) cut L2 data costs 50–90%. [CONFIRMED] https://dev.to/raji_moshood_ee3a4c2638f6/gas-fees-explained-why-ethereum-is-expensive-and-how-layer-2s-solve-it-49oc

## Privacy gaps (relevant to Logos)

- **Total transparency is the default and cannot opt out:** `ownerOf`, holdings-per-address, and the transfer graph are public primitives baked into ERC-721/1155 — a privacy-first stack must treat these as hidden-by-default to differentiate. [CONFIRMED] https://eips.ethereum.org/EIPS/eip-721
- **Price/trade visibility:** sales are public → wealth, taste, trading behavior of any linked address exposed. [H — 90%] https://eips.ethereum.org/EIPS/eip-5564
- **Ethereum privacy tooling is bolt-on + partial:** stealth addresses hide only the recipient, low-adoption, don't shield amounts/senders/graph — room for native sender/recipient/amount confidentiality + zk provenance proofs. [H — 80%] https://eips.ethereum.org/EIPS/eip-5564
- **Metadata privacy unaddressed:** even "private" schemes leave metadata on public IPFS/HTTP; Logos could keep asset content confidential/access-gated. [H — 75%] https://arxiv.org/pdf/2408.13281
- **Royalty enforcement solved nowhere** on transparent chains; a privacy stack could enforce in the transfer primitive, not by marketplace goodwill. [H — 70%] https://cointelegraph.com/news/opensea-disable-on-chain-royalty-enforcement-tool

## Sources
ERC-721 https://eips.ethereum.org/EIPS/eip-721 · ERC-1155 https://eips.ethereum.org/EIPS/eip-1155 · ERC-6551 https://eips.ethereum.org/EIPS/eip-6551 · EIP-2981 https://eips.ethereum.org/EIPS/eip-2981 · ERC-3569 https://eips.ethereum.org/EIPS/eip-3569 · EIP-5564 https://eips.ethereum.org/EIPS/eip-5564 · ERC-6538 https://eips.ethereum.org/EIPS/eip-6538 · metadata study https://arxiv.org/pdf/2408.13281 · IPFS immutability https://rameerez.com/problems-and-technical-nuances-of-nft-immutability-and-ipfs/ · OpenSea royalties https://cointelegraph.com/news/opensea-disable-on-chain-royalty-enforcement-tool · Blur/OpenSea https://ambcrypto.com/opensea-to-suspend-operator-filter-as-blurs-ascendency-gains-momentum/ · stealth-address analysis https://arxiv.org/pdf/2308.01703 · ERC721A gas https://www.alchemy.com/blog/erc721-vs-erc721a-batch-minting-nfts
