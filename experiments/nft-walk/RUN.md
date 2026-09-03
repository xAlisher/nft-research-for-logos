# A5 — NFT dogfooding walk against a live LEZ node (run-log)

_2026-09-03. Ran the wallet against the standalone LEZ node on Sneg (`:3040`, `RISC0_DEV_MODE=1`)._

> **✅ UPDATE — full walk GREEN after the A4 fix.** The transfer now lands (block 77): `nftcopy → owned:false`, `nftrecipient → owned:true`. Green run: [`run-log-green.txt`](run-log-green.txt). The first run (below) surfaced the A4 bug; kept for the record. First run: [`run-log.txt`](run-log.txt); script: [`walk.sh`](walk.sh).

## Result summary

| Step | Command | Outcome |
|---|---|---|
| 1 | `account new public` ×5 (labeled) | ✅ accounts created |
| 2 | `token new-nft` (A2) | ✅ **included in block 33** — NFT defined |
| 3 | `account get nftmaster` (A1 render) | ✅ `{"NftMaster":{"definition_id":"8eNX…","print_balance":5}}` |
| 4 | `token print-nft` (A3) | ✅ **included in block 35** |
| 5 | `account get nftcopy` | ✅ `{"NftPrintedCopy":{"definition_id":"8eNX…","owned":true}}` |
| 6 | `token send --amount 1` (A4) | ❌ **rejected** — see finding below |
| 7 | post-state check | ✅ consistent: copy still `owned:true`, recipient `Uninitialized` (rejected tx did not apply) |

**A1, A2, A3 are proven end-to-end on a live node.** The public NFT create → render → print → own lifecycle works.

## Finding — A4 does NOT "just work" via the existing `Send`

The transfer tx was **rejected by the sequencer**:

```
ERROR sequencer_core] Transaction e10f42cf… failed execution check with error:
  InvalidProgramBehavior(ClaimedUnauthorizedAccount { account_id: ENv6t8DindQh5rGicx4RFqnv4LUKYnzwRwYEXXu16yyb })
```

**Root cause.** `token::transfer` claims a fresh (default) recipient with `Claim::Authorized` (`transfer.rs`: `new_claimed_if_default(recipient_post, Claim::Authorized)`), but the public token `Send` handler passes the recipient as **`PublicNoSign`** (`handle_transfer_token` → `into_public_identity(recipient_id, false)`). Claiming an account as *Authorized* requires that account to be authorized (signed) in the transaction — so the claim is rejected.

**This corrects the earlier A4 assessment** (issue #8), which was based on a code read ("`send_transfer_transaction` has no fungible-specific logic, so it just works"). Reading the path is not running it — the live run disproves the assumption. (📍 verify-before-claiming.)

**Fix directions (for A4):**
1. **Pre-initialize the recipient holding** before transfer — an `InitializeAccount` / `ata create` for that definition so the recipient is no longer default (then the transfer updates, not claims). This likely also affects fungible token transfers to fresh recipients.
2. **or** have the token `Send` sign/authorize the recipient when it will be claimed (pass the recipient authorized when the wallet holds its key), i.e. `into_public_identity(recipient_id, true)` for the claim case.
3. Same constraint applies to the **shield** step in Epic B (mint/transfer an NFT *into* a fresh private account) — worth confirming against B0's setup (B0 used `PrivateForeignInit`, which handles the claim differently, so B0 is unaffected).

Next: implement fix (1) or (2), re-run this walk to green, then proceed to Epic B against the live node.

## Reproduce
Node up on Sneg (see issue #9). Then: `bash ~/nft-build/walk.sh` (uses the release wallet, `LEE_WALLET_HOME_DIR` with `sequencer_addr=http://127.0.0.1:3040`).
