# Epic B — private NFT ownership walk (run-log)

_2026-09-03. Ran the wallet against the live standalone LEZ node on Sneg (`:3040`, `RISC0_DEV_MODE=1`). Trimmed output: [`run-log-b.txt`](run-log-b.txt) (the raw log is ~500KB — the privacy-preserving txs dump full encrypted state; trimmed here to the signal). Script: [`walk-b.sh`](walk-b.sh)._

## Result — GREEN, no code change needed

The private paths (`token send` routing Public↔Private) handled the NFT holding out of the box; the public path's A4 claim-auth fix was **not** needed here (the shielded/private/deshielded routing authorizes the recipient via foreign-init/owned handling).

| Leg | Command | Block | Verification |
|---|---|---|---|
| setup | `new-nft` + `print-nft` → public holder | 90, 92 | `bpubcopy = NftPrintedCopy owned:true` |
| **B1 shield** | `send bpubcopy → bpriv1 --amount 1` (public→**private**) | 94 (`PrivacyPreserving` tx) | `bpubcopy owned:false` · `bpriv1 owned:true` (private, local) |
| **B2 private→private** | `send bpriv1 → bpriv2 --amount 1` | 96 | `bpriv1 owned:false` · `bpriv2 owned:true` |
| **B3 deshield** | `send bpriv2 → bpubfinal --amount 1` (private→public) | 98 | `bpriv2 owned:false` · `bpubfinal owned:true` (public) |

All steps `exit 0`; no `ClaimedUnauthorizedAccount`, no failed execution checks on the node.

## What this proves (the differentiator, live)

The **full private NFT lifecycle** works end-to-end on a live LEZ node: an NFT can be **shielded** into a private account (no public owner — only a commitment + ML-KEM ciphertext on-chain), **transferred privately** (no public sender→recipient edge), and **deshielded** back to public on the owner's terms. This is exactly the "own privately, transfer unlinkably, reveal on your terms" differentiator (docs/analysis-and-strategy.md), now demonstrated against real block production — not just the circuit proof (B0, #10).

- **B4 (private-holdings display):** `account get --account-id <private label>` renders the `NftPrintedCopy` from local state (A1's `describe_holding`), confirmed on `bpriv1`/`bpriv2`.

## Scope / honesty
- Dev-mode proving (`RISC0_DEV_MODE=1`) — executes guests for real, skips STARK generation (CI's own setting). A real-proof run is heavier but the logic path is identical.
- Selective disclosure via viewing key (Epic C: export-scoped-key + verifier) is the next differentiator layer and is **not** exercised here.

## Reproduce
Node up on Sneg (issue #9). Then: `bash ~/nft-build/walk-b.sh` (fresh wallet home each run; `sequencer_addr=http://127.0.0.1:3040`).
