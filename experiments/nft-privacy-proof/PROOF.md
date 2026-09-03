# B0 — NFT-through-privacy path: PROVEN

_2026-09-03. Resolves the one assumption the verification flagged as "structurally sound but unproven end-to-end" (issue #10 / [`../../docs/verification.md`](../../docs/verification.md))._

## Result

```
test private_nft_transfer_hides_owner_and_provenance ... ok
test result: ok. 1 passed; 0 failed; ... finished in 0.16s
```

A private NFT transfer **executed, proved, and verified** against `V03State` using the **real token program** and a real `NftPrintedCopy` holding — the first time an NFT has gone through the Logos privacy circuit.

## What the test does (`nft_privacy.rs`)

1. Builds a **private (shielded)** sender account whose value is an NFT — `TokenHolding::NftPrintedCopy { definition_id, owned: true }` serialized into `account.data`, `program_owner = programs::token().id()` (the real token program ELF).
2. Runs a token `Instruction::Transfer { amount_to_transfer: 1 }` **private → private** through `lee`'s `execute_and_prove` (the privacy-preserving execution circuit), with the sender as `PrivateAuthorizedUpdate` and a fresh recipient as `PrivateForeignInit` — identical identity setup to the proven fungible path.
3. Applies the resulting `PrivacyPreservingTransaction` via `state.transition_from_privacy_preserving_transaction(...)`. **This call succeeding is the core proof: the circuit proof VERIFIED for an NFT holding.**
4. Asserts (via the public `get_proof_for_commitment`) that after the transfer the private commitment set contains exactly:
   - the **sender's** new commitment (same NFT, now `owned: false`),
   - the **recipient's** new commitment (same NFT definition, now `owned: true`),
   both absent beforehand — i.e. the ownership change is recorded purely as **private commitments**, with no public `ownerOf` and no on-chain sender→recipient edge.

## Why this is the decisive proof

The privacy layer commits opaque `account.data` (`commitment.rs`) and the circuit runs an arbitrary program (`execute_and_prove`), so the open question was only whether a **`data`-carrying** holding (NFT) round-trips the private path like a **`balance`-carrying** one (fungible). It does — with the real token program, not a replica.

## How to reproduce

The test lives at `integration_tests/tests/nft_privacy.rs` in the `logos-execution-zone` checkout (mirrored here). From that repo root, dev-mode like CI:

```
env RISC0_DEV_MODE=1 CARGO_TARGET_DIR=/extra/tmp/lez-nft-target cargo test -p integration_tests --test nft_privacy -- --nocapture
```

## Scope / honesty

- **Level proven:** circuit + state-machine (the privacy engine). This is the layer the risk lived in.
- **Not covered here (as scoped):** the wallet/CLI NFT surface (Epic A — the wallet still returns "unsupported token type"; the wallet-driven integration harness can't mint/print NFTs yet), and a real (non-dev-mode) STARK proof. Dev mode executes the guest for real but skips STARK generation — same setting CI uses for these tests.
- **Consequence for the MVP:** Epic B is de-risked. The remaining B1–B4 work is wallet plumbing + UX over a privacy path now proven to accept NFTs, not new cryptography.
