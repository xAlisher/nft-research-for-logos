# Solana NFTs — findings

_Research by Sina agent, 2026-09-03. Evidence tags: [CONFIRMED] primary source · [H — X%] hypothesis/inference · [? unknown]._

## Standards / programs
- An NFT on Solana is an SPL token mint with **supply 1, 0 decimals, mint authority null** (supply can never change). [CONFIRMED] https://github.com/solana-foundation/developer-content/blob/main/content/courses/tokens-and-nfts/nfts-with-metaplex.md
- **Metaplex Token Metadata** attaches a metadata account (PDA) to a mint, storing on-chain name/symbol/URI/royalties/verification flags. [CONFIRMED] https://developers.metaplex.com/token-metadata
- Token Standard enum (set by program): 0 NonFungible, 1 FungibleAsset (semi-fungible), 2 Fungible, 3 NonFungibleEdition, 4 ProgrammableNonFungible. [CONFIRMED] https://metaplex.com/docs/token-metadata/token-standard
- A **Master Edition** marks a mint non-fungible + controls printing of Editions. [CONFIRMED] https://metaplex.com/docs/token-metadata/token-standard
- **Programmable NFTs (pNFTs)** are frozen at all times so all transfer/delegate calls route through Token Metadata, validated against creator **rule sets** (allow/deny-list) — the enforced-royalties mechanism. [CONFIRMED] https://metaplex.com/docs/token-metadata/token-standard
- **Metaplex Core** is now the recommended standard: single-account asset design, enforced royalties, plugin system. Token Metadata is labeled legacy. [CONFIRMED] https://www.metaplex.com/docs/smart-contracts/core
- **Token-2022 (Token Extensions)** adds mint-level features incl. a metadata extension; primarily the fungible track. [H — 80%] https://solana.com/docs/tokens/extensions

## Compressed NFTs (cNFTs)
- cNFTs (Metaplex **Bubblegum V2**) store each asset as a **hashed leaf in an on-chain Merkle tree**; the tree holds only root/hashes, not asset data. [CONFIRMED] https://www.metaplex.com/docs/smart-contracts/bubblegum-v2
- Actual NFT data lives **off-chain in Solana transaction history** (changelog); only the Merkle root is on-chain. [CONFIRMED] https://www.metaplex.com/docs/smart-contracts/bubblegum-v2
- **State compression** uses a **Concurrent Merkle Tree** (changelog of recent roots + proof paths) enabling parallel writes per block. [CONFIRMED] https://www.metaplex.com/docs/smart-contracts/bubblegum-v2/concurrent-merkle-trees
- Retrieval depends on the **Metaplex DAS API**; **not all RPCs support DAS**. [CONFIRMED] https://www.metaplex.com/docs/smart-contracts/bubblegum-v2
- Cost: ~0.00001 SOL per cNFT in large trees vs ~0.0029 SOL per Core NFT (~290x); ~1M-cNFT tree ≈ 8.5 SOL rent. Independent: 1M collection ≈ 12,000 SOL traditional vs ≈ 5 SOL compressed. [CONFIRMED] https://www.quicknode.com/guides/solana-development/nfts/mint-compressed-nft
- **Trade-offs:** hard dependency on DAS-capable RPC/indexer; ecosystem lag (V1→V2 wallet/marketplace support). [CONFIRMED] https://www.metaplex.com/docs/smart-contracts/bubblegum-v2

## Metadata & storage
- Split: **on-chain** = name/symbol/URI/royalties/flags (metadata PDA); **off-chain JSON** = description/image/attributes. [CONFIRMED] https://www.quicknode.com/guides/solana-development/nfts/solana-nft-metadata-deep-dive
- Off-chain data typically on Arweave/IPFS/Shadow Drive (a plain web URI is also valid). [CONFIRMED] https://www.quicknode.com/guides/solana-development/nfts/solana-nft-metadata-deep-dive
- **Collection model:** Metaplex Certified Collections group NFTs under a collection NFT with an on-chain **verified** flag. [CONFIRMED] https://www.quicknode.com/guides/solana-development/nfts/solana-nft-metadata-deep-dive

## Provenance & ownership
- Ownership fully **public**: holder = wallet whose token account holds the supply-1 mint, readable by anyone. [CONFIRMED] https://github.com/solana-foundation/developer-content/blob/main/content/courses/tokens-and-nfts/nfts-with-metaplex.md
- Full transfer history + provenance publicly auditable; cNFTs persist each update in tx history. [CONFIRMED] https://www.metaplex.com/docs/smart-contracts/bubblegum-v2

## Privacy posture
- Public by default: metadata, current owner, full history all on-chain + indexable. No native NFT privacy layer. [CONFIRMED] https://github.com/solana-foundation/developer-content/blob/main/content/courses/tokens-and-nfts/nfts-with-metaplex.md
- Only shipped on-chain privacy primitive: **Token-2022 Confidential Transfer** (homomorphic encryption + ZK proofs). [CONFIRMED] https://solana.com/docs/tokens/extensions/confidential-transfer
- Provides only **partial privacy: hides transfer amounts + balances; token account addresses (ownership linkage) stay public**. [CONFIRMED] https://solana.com/docs/tokens/extensions/confidential-transfer
- Targets **fungibles** — no NFT support documented, and hiding an amount is meaningless for a supply-1 asset (the real secret = who owns which token, stays public). [H — 85%] https://www.helius.dev/blog/confidential-balances
- Optional **auditor** (ElGamal key on mint) can decrypt all amounts — privacy is revocable/gated by design. [CONFIRMED] https://www.helius.dev/blog/confidential-balances
- Whether any project built NFT-specific confidentiality on Solana. [? unknown]

## Cost & scale
- Standard/Core mint ≈ 0.0029 SOL (rent-dominated). [CONFIRMED] https://www.metaplex.com/docs/smart-contracts/bubblegum-v2
- Transfers ≈ base fee (~0.000005 SOL) + optional priority; cNFT transfers need a Merkle proof (larger tx). [H — 80%] https://solana.com/docs/core/fees
- cNFT economics: near-zero marginal mint after one-time tree rent → million-item collections viable. [CONFIRMED] https://www.quicknode.com/guides/solana-development/nfts/mint-compressed-nft

## Privacy gaps (relevant to Logos)
- **Ownership is never hidden.** Even with confidential transfers, token account addresses stay public → who-owns-what always exposed. The core gap Logos can differentiate on. [CONFIRMED] https://solana.com/docs/tokens/extensions/confidential-transfer
- **No confidential ownership/transfer privacy for NFTs** exists today; the one ZK primitive is fungible-amount-oriented. [H — 85%] https://www.helius.dev/blog/confidential-balances
- **Full public provenance:** every prior owner + sale price permanently visible — Logos could expose provenance selectively (ZK proof of authenticity/collection membership without revealing owner history). [CONFIRMED] https://www.metaplex.com/docs/smart-contracts/bubblegum-v2
- **Metadata public:** off-chain JSON discoverable via public URI; no native encrypted/gated-content standard. [H — 75%] https://www.quicknode.com/guides/solana-development/nfts/solana-nft-metadata-deep-dive
- **Auditor-backdoor precedent:** Solana privacy is opt-in + auditor-decryptable — a design choice Logos should consciously accept or reject (selective disclosure vs mandatory auditability). [CONFIRMED] https://www.helius.dev/blog/confidential-balances
- **Indexer/trust surface:** cNFTs prove Solana tolerates off-chain data + on-chain roots (DAS). A privacy design could reuse the pattern (encrypted off-chain data + on-chain commitment) but must avoid the RPC-centralization dependency. [CONFIRMED] https://www.metaplex.com/docs/smart-contracts/bubblegum-v2

## Sources
NFTs-with-Metaplex course https://github.com/solana-foundation/developer-content/blob/main/content/courses/tokens-and-nfts/nfts-with-metaplex.md · Token Standard https://metaplex.com/docs/token-metadata/token-standard · Token Metadata https://developers.metaplex.com/token-metadata · Bubblegum V2 https://www.metaplex.com/docs/smart-contracts/bubblegum-v2 · Concurrent Merkle Trees https://www.metaplex.com/docs/smart-contracts/bubblegum-v2/concurrent-merkle-trees · Core https://www.metaplex.com/docs/smart-contracts/core · Mint cNFT https://www.quicknode.com/guides/solana-development/nfts/mint-compressed-nft · Metadata deep dive https://www.quicknode.com/guides/solana-development/nfts/solana-nft-metadata-deep-dive · Enforceable royalties https://decrypt.co/112595/solana-enforceable-nft-royalties-new-metaplex-standard · Confidential Transfer https://solana.com/docs/tokens/extensions/confidential-transfer · Confidential Balances https://www.helius.dev/blog/confidential-balances · Fees https://solana.com/docs/core/fees
