# Verification verdict — source-checked

_Adversarial verification of [`analysis-and-strategy.md`](analysis-and-strategy.md) + [`mvp.md`](mvp.md) against the actual LEZ source at `~/basecamp/refs/logos-execution-zone`. 2026-09-03. (An independent Codex/"Senti" pass was also launched — task `task-mtl8aesa-auj1ld` — pull it with `/codex:status` to cross-check; findings below are the in-house source read.)_

| # | Claim | Verdict | Evidence (real path:line) |
|---|---|---|---|
| 1 | Non-fungible token program exists (`PrintNft`, `NonFungible`, `NftMaster`/`NftPrintedCopy`, `TokenMetadata`) | **CONFIRMED** | `lez/programs/token/core/src/lib.rs`: `Instruction::PrintNft` :62 · `NonFungible` :71,:84 · `NftMaster` :117 · `NftPrintedCopy` :122 · `TokenMetadata` :206 (`standard` :210, `primary_sale_date` :216) · `MetadataStandard` :221 |
| 2 | `print_nft` decrements supply + asserts `>1`; `mint` panics on NFT (scarcity) | **CONFIRMED** | `token/src/print_nft.rs:36-40` (`assert!(*print_balance > 1)`, `checked_sub(1)`, `owned:true` :48) · `token/src/mint.rs:57` (`panic!("Cannot mint additional supply for Non-Fungible Tokens")`) |
| 3 | Shielded accounts: commitment/nullifier + ML-KEM-768 note encryption + viewing key | **CONFIRMED** | `lee/state_machine/core/src/commitment.rs:40,:57` · `encryption/mod.rs:14` (`ML_KEM_768_CIPHERTEXT_LEN=1088`), :2-3 (ChaCha20), :75-83 (`EncryptedAccountData`, `compute_view_tag(npk,vpk)`) · `nullifier.rs:40-41` (`for_regular_private_account(npk, vpk, identifier)`) |
| 4 | Wallet ATA layer returns "unsupported token type" for NFTs (the blocking gap) | **CONFIRMED** | `lez/wallet/src/cli/programs/ata.rs:208-209` (`NftMaster{..}|NftPrintedCopy{..} => println!("… unsupported token type")`) |
| 5 | The MVP is buildable on today's primitives | **CONFIRMED — and stronger than drafted** | see corrections below |
| 6 | Differentiator (private ownership / unlinkable provenance / selective disclosure) is genuinely unique + Logos-enabled | **CONFIRMED** | incumbents public-by-construction (`../research/*`); Logos privacy is account-level + data-agnostic (below) |

## Corrections the docs needed (applied)

- **NFT transfer logic ALREADY EXISTS in the program** — not just for fungibles. `token/src/transfer.rs:72-94` handles `NftPrintedCopy`: asserts `*sender_owned`, asserts `!*recipient_owned`, then `*sender_owned=false; *recipient_owned=true`; `NftMaster` at :50-54. **Consequence:** Epic A's transfer work is *wallet plumbing*, not program logic — smaller than drafted. The original MVP under-claimed here.
- **The privacy layer is account-data-agnostic** — `Commitment::new(account_id, account)` commits `SHA256(Comm_DS ‖ account_id ‖ program_owner ‖ balance ‖ nonce ‖ SHA256(account.data))` (`commitment.rs:11-25`); the privacy circuit operates on generic `Account`/`AccountWithMetadata` (`privacy_preserving_circuit/src/execution_state.rs:8,:20`), never on fungible-specific types. An NFT holding is just serialized `TokenHolding::NftPrintedCopy` inside `account.data`, so it shields / private-transfers / deshields through the **same** path as a fungible holding. This upgrades differentiators #1/#2 from [H] toward [CODE-supported] and de-risks Epic B.

## Buildability verdict

**The MVP is genuinely ready to scope into issues.** The cryptographic foundations (shielded accounts, ML-KEM, viewing keys, generic-account commitments) and the NFT program semantics (define, print, transfer-with-ownership-flip, scarcity) all exist in-tree.

**Riskiest remaining assumption (honest):** while the *program* transfer logic and the *privacy* commitment path both exist, there is no evidence the **wallet + privacy-transaction builder has been exercised end-to-end for an NFT holding specifically** (the fungible private-transfer path is what the docs/tests demonstrate). The NFT-through-privacy path is structurally sound but **unproven end-to-end** — so the very first build step (Epic A / early Epic B) must be a *trivial-experiment-first*: shield one printed-copy NFT and private-transfer it once, before scoping the fuller UX. If that smallest experiment passes, the rest of the MVP follows with high confidence. Two smaller risks stay as drafted: metadata leaks via the plaintext off-chain metadata account (Phase 3), and explorer/indexer can't render private holdings without client-side viewing-key decoding (part of Epic C).
