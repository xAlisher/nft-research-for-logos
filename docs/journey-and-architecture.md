# Sealed — end-to-end journey, architecture & gaps

_2026-09-03. How the collection actually gets minted, distributed, revealed — the components, the sequence, and the honest gaps before we build. Grounded in verified surfaces: the LEZ token program, the wallet CLI (Epics A/B/C, proven live), and the `wallet-ffi` C ABI (`lez/wallet-ffi/wallet_ffi.h`)._

## Components

| Component | What it is | Status |
|---|---|---|
| **LEZ node** | The chain the NFTs live on | ✅ live on Sneg (:3040); standalone works |
| **Token program** | define / print / transfer / shield / deshield NFTs | ✅ in-tree, proven live |
| **Wallet CLI** | `wallet` binary — mint, shield, reveal (`account get`), verify-disclosure | ✅ built (our branch: A1–A4 + C) |
| **`wallet-ffi`** | C ABI so a **module** can drive the wallet: `open`/`create_new`, `list_accounts`, `sync_to_block`, `get_account_private` (reveal), `transfer_shielded/deshielded/private_owned`, `get_private_account_keys`, `token_elf` + `send_generic_*_transaction` | ✅ exists; missing NFT convenience wrappers |
| **Sealed module** | The Basecamp app: gallery + login + declassify + copy | ❌ **to build** |
| **Encrypted payload store** | Where each piece's `{link, note}` ciphertext lives | ❌ **undefined — design gap** |

## The journey

### A. Mint & distribute (creator / admin, one-time)
1. **Define** the collection: `token new-nft "Sealed Genesis"` → one NFT definition + metadata + master (✅ proven).
2. **Print** the edition: `token print-nft` ×15 → 15 printed copies (✅ proven).
3. **Attach the payload:** for each piece, encrypt `{archive.org link, curatorial note}` under a key derived from the *recipient's* viewing key, and store the ciphertext (see the storage gap below).
4. **Distribute** — three options:
   - **Key-handoff (simplest, on-theme):** print each piece into a fresh private account and hand the recipient that account's mnemonic — a literal "sealed envelope." Good for 15.
   - **Shield-to-recipient:** recipient shares their npk/vpk; admin `transfer_shielded` a piece into their private account. Needs their keys first.
   - **Claim/drop (scalable, more build):** a claim flow where a user connects and pulls a piece — needs a distribution program or an admin-signed claim.

### B. Own & reveal (collector, in the module)
1. **Open the module** (Sealed) in Basecamp.
2. **Log in:** module calls `wallet_ffi_open` on the user's wallet (mnemonic / keystore) → it now holds their keys **in-process**.
3. **Sync:** `wallet_ffi_sync_to_block` → discovers the user's private NFT holdings (`view_tag` scan).
4. **Gallery:** module lists holdings; each sealed piece renders its **redaction art** (from the commitment). Public/others see only the seal.
5. **Declassify:** user taps a piece → module decrypts the `{link, note}` payload with the user's **viewing key** → shows the document link + curatorial note + **copy**. (Optionally `transfer_deshielded` for a *public* reveal.)
6. **Prove (optional):** share the viewing key / run the verifier so a chosen party confirms ownership without the wallet being exposed (Epic C, ✅).

## Sequence (activation order)
```
CREATOR:  node up → define → print×15 → encrypt payloads → distribute (keys/shield)
COLLECTOR: open Sealed module → wallet_ffi_open (login) → sync → see sealed gallery
           → tap "Declassify" → viewing-key decrypt → link+note → copy / open
           → (optional) deshield for public reveal, or export viewing key to prove
```

## Gaps (what's missing before this ships)

| # | Gap | Severity | Workaround now | Real fix |
|---|---|---|---|---|
| 1 | **Sealed module** doesn't exist | expected | — | Build it (QML/ui_qml module binding `wallet-ffi`) — the main deliverable |
| 2 | **Encrypted-payload storage** — the token NFT holding only carries `owned:bool`; the `{link,note}` ciphertext needs a home + key-wrapping to the owner's viewing key | **crypto PROVEN** (see [`../experiments/sealed-collection/ENCRYPT-THEN-SHIELD.md`](../experiments/sealed-collection/ENCRYPT-THEN-SHIELD.md)); transport TBD | encrypt-then-shield test passes: payload encrypted to recipient vpk (real ML-KEM+ChaCha) + NFT shielded to same recipient, one viewing key opens both | Only the ciphertext **transport** remains: token **metadata `uri`** blob vs **Codex/IPFS**. Crypto + key-wrapping settled |
| 3 | **"Login with Logos Wallet" (connect to the *running* official wallet) does not exist** | medium | `wallet_ffi_open` lets the module open the user's wallet keystore/mnemonic *in-process* — i.e. "log in by opening your wallet inside the app" | A true wallet-connect (module ↔ separate wallet app via IPC/session, WalletConnect-style) is **absent → a fork/build**: add a shared-keystore open or an IPC bridge to the wallet. For the demo, in-process open is enough |
| 4 | **NFT mint/print convenience absent in FFI** (has transfers, not `new_nft`/`print_nft`) | low | admin mints via the **CLI** (proven), or the module builds the instruction via `send_generic_public_transaction` + `token_elf` | Add `wallet_ffi_new_nft` / `print_nft` wrappers (small) |
| 5 | **Distribution mechanism** — no claim/drop program | low (for 15) | key-handoff or manual shield-transfer | A claim program / drop UX for scale |
| 6 | **Our A1–A4 + C wallet work is on a local branch, not upstream** | medium | build the module against our fork | Upstream the wallet NFT support (needs review) — a separate track |
| 7 | **Private metadata** — the metadata `uri` is public | low (payload is encrypted anyway) | encrypt the payload regardless | Native private-metadata (Phase 3) |

## Recommended path
1. **Resolve gap #2 first** (payload storage + key-wrapping) — everything else depends on it. Prototype: ciphertext in `metadata.uri`, key wrapped to the owner's viewing key, reveal via `get_account_private` + our decrypt.
2. **Build the Sealed module** (gap #1) binding `wallet-ffi`: login (`open`) → sync → gallery → declassify → copy.
3. Use **key-handoff distribution** for the 15-piece demo (gap #5 deferred).
4. Treat **wallet-connect** (gap #3) as a stretch — in-process `open` is fine for the demo; a real "Login with Logos Wallet" across apps is a fork worth scoping separately (and broadly useful beyond this collection).

## Key collection (shield-to-recipient) & the receiver journey

Chosen distribution: **shield-to-recipient** — you collect each recipient's receive-key first, then shield a piece to them. All supported by `wallet-ffi`:

- **Generate (receiver side):** `wallet_ffi_create_private_accounts_key` → `FfiPrivateAccountKeys { nullifier_public_key (npk, 32B), viewing_public_key (vpk, 1184B ML-KEM) }`.
- **Shield (your side):** `wallet_ffi_transfer_shielded(from, to_keys = {npk, vpk}, to_identifier, amount=1, …)` — takes exactly the recipient's npk+vpk. The recipient discovers the incoming piece by running `sync` (the identifier need not be shared; the scan finds it).

**Can collection be done through a Basecamp module? Yes — generation + display can.** The module calls `create_private_accounts_key` and shows the npk+vpk as a copyable **"receive key"** (best: an **exported `.keys` file** — vpk is 1184 bytes, too long to paste and bulky as a QR). What a module can't do alone is *centrally collect* many keys — there's no on-chain "register to receive." So the transport is **out-of-band**: recipients export their `.keys` and send it to you (Discord/form/upload), you keep the list and batch-shield. (A submission endpoint could automate collection later; not needed for a 15-recipient demo.)

### Receiver journey (step by step)
1. **Install** the Sealed module in Basecamp (from the catalog).
2. **Wallet:** on first run, create a wallet (`wallet_ffi_create_new` → shows a mnemonic to back up) or open an existing one (`wallet_ffi_open`). This is their identity + keys.
3. **Get receive-key:** tap "Get my claim key" → module runs `create_private_accounts_key` → shows npk + vpk and offers **Export `.keys`**.
4. **Send it to the curator** (out-of-band). ← *this is the key-collection step*
5. *(You shield a piece to their key — off-stage: `transfer_shielded`.)*
6. **Sync:** receiver taps Sync → `wallet_ffi_sync_to_block` → the incoming private NFT is discovered.
7. **Sealed gallery:** their piece appears as a redaction sheet. No one else can see they hold it (private ownership).
8. **Declassify:** tap the piece → viewing key decrypts the `{link, note}` payload → archive.org link + curatorial note + **copy / open**.
9. **Optional:** `deshield` for a public reveal, or export the viewing key so a chosen party can verify ownership (Epic C) without exposing the wallet.

### Extra gaps this surfaces
- **Central key collection** = out-of-band for now (no register-to-receive); a submission form/endpoint is a later nicety.
- **`.keys` transport UX** — module should export/import a `.keys` file (npk line 1, vpk line 2), matching the CLI `--to-keys` convention; QR is impractical at 1184 bytes.
- Ties back to **gap #2 (payload storage)**: the `{link,note}` ciphertext must be attached at shield time and keyed to the recipient's viewing key — so you encrypt per-recipient once you have their vpk.

## Governance
Module + Museum branding are outward-facing → run past Franck/leadership (and Eric for content) before anything official. Groundwork (module, mint, mechanic) proceeds internally regardless.
