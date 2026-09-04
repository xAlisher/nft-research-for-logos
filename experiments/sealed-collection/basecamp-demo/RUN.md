# Sealed collection — live in an isolated Basecamp (5-piece wall)

The `sealed_keys` module (xAlisher/sealed-keys-basecamp) shown end-to-end in an ISOLATED
Basecamp against the our LEZ test node LEZ node: 5 sealed NFTs across 3 exhibit halls, discovered + unsealed.

## Reliable seeding — `seed_robust.py`
Per-step verification + retry, then passes until all present. The fragile step is the shield
(`token send --from <copy> --to <rcpt>`): it panics `Invalid sender data` (transfer.rs:16) when
`print-nft` isn't synced into the wallet's view yet. Fix = **poll `account get <copy>` until it
actually holds the printed NFT, THEN shield** (retry on race). Public name = `<exhibit>||<nn>`
(structure public); title/meta/note/url sealed in the payload.

Result: `RESULT ALL_PRESENT` — 5/5, every shield on attempt 1.
  I·Control of Money 01 (EO 6102), 02 (Nixon Gold Window)
  II·Surveillance State 03 (Church Committee), 04 (MKUltra)
  III·Censored World 05 (Areopagitica)

## Bugs the wetware run caught (all fixed in the module)
- ui_qml is a SEPARATE lgx (mkLogosQmlModule) + needs a 256² icon + root `assets/icon.png` staged.
- `font.pixelSize: 12.5` (float) aborts QML load — must be int.
- QML→core bridge wraps returns as `{success,value,error}` — unwrap to `value`.
- show-keys needs `--account-id`; keys print as two UNLABELED hex lines.
- **std::regex `[.*]` / `{[^\n]*}` on ~KB CLI output stack-overflows (SIGSEGV) at 2+ records**
  — replaced with plain find/rfind.
- AppImage resets PATH → honour `LEE_WALLET_BIN`.
- Node: 2s block time raced tx finalization (use 15s); restart needs the `~/.risc0/bin/r0vm` symlink.
