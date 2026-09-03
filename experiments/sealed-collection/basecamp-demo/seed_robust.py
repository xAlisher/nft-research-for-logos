#!/usr/bin/env python3
"""Robust Sealed-collection seeder: per-step verification + retry, then a final
verification loop that re-runs any missing piece until every NFT is present.

The fragile step is the shield (`token send --from <copy> --to <rcpt>`): it panics
'Invalid sender data' when print-nft isn't synced into the wallet's view yet. So we
poll `account get <copy>` until it actually holds the printed NFT before shielding.
"""
import os, subprocess, sys, time, json, re

WALLET = "/extra/tmp/sealed-wallet/wallet"
HOME = "/extra/tmp/sealed-wallet/home"
ENV = dict(os.environ,
           PATH="/extra/tmp/sealed-wallet:" + os.path.expanduser("~/.risc0/bin") + ":" +
                os.path.expanduser("~/.cargo/bin") + ":" + os.environ.get("PATH", ""),
           RISC0_DEV_MODE="1", LEE_WALLET_HOME_DIR=HOME)

PIECES = [
    ("I · Control of Money",     "01", "Executive Order 6102 — Gold Confiscation", "F. D. Roosevelt · 1933",
     "https://archive.org/details/pdfy-MHvlymfJYU05yELW",
     "By decree, holding your own gold became a crime. Money you 'own' can be recalled by the state at will."),
    ("I · Control of Money",     "02", "Nixon Closes the Gold Window", "White House · 1971",
     "https://archive.org/details/1971-president-nixon-address-to-the-nation-outlining-a-new-economic-policy-the-c",
     "The last tie between the dollar and gold, cut overnight. Money becomes worth whatever power says it is."),
    ("II · Surveillance State",  "03", "Church Committee Final Report", "US Senate · 1976",
     "https://archive.org/details/ChurchCommittee_FullReport",
     "The Senate's own record of intelligence agencies spying on, disrupting, and deceiving the citizens they served."),
    ("II · Surveillance State",  "04", "MKUltra Document 0000017748", "CIA · FOIA release",
     "https://archive.org/details/DOC_0000017748",
     "The CIA's experiments on unwitting people — surveillance and control turned inward, without consent."),
    ("III · Censored World",     "05", "Areopagitica", "John Milton · 1644",
     "https://archive.org/details/miltonareopagitica",
     "Milton's case against licensing the press, printed in defiance of the censors it condemns."),
]

def w(*args, timeout=180):
    """Run a wallet command; return (rc, stdout+stderr)."""
    try:
        p = subprocess.run([WALLET, *args], env=ENV, stdin=subprocess.DEVNULL,
                           capture_output=True, text=True, timeout=timeout)
        return p.returncode, (p.stdout + p.stderr)
    except subprocess.TimeoutExpired as e:
        return 124, (e.stdout or "") + (e.stderr or "") + "\n[TIMEOUT]"

def log(m): print(f"[{time.strftime('%H:%M:%S')}] {m}", flush=True)

def new_account(kind, label):
    rc, out = w("account", "new", kind, "--label", label)
    m = re.search(r'account_id (Public|Private)/([1-9A-HJ-NP-Za-km-z]+)', out)
    return (m.group(1) + "/" + m.group(2)) if m else None

def included(out):  # tx landed on-chain?
    return "included in block" in out.lower()

def present_names():
    rc, out = w("sealed-records")
    m = re.search(r'\[.*\]', out, re.S)
    if not m: return set()
    try: d = json.loads(m.group(0))
    except Exception: return set()
    return {r.get("name") for r in d if str(r.get("account", "")).startswith("Private/")}

def copy_has_nft(copy_label, tries=8):
    """Poll account get until the copy holds a printed NFT (forces sync)."""
    for _ in range(tries):
        rc, out = w("account", "get", "--account-id", copy_label)
        if re.search(r'NftPrintedCopy|NftMaster|printed', out):
            return True
        time.sleep(4)
    return False

def seed_piece(i, piece):
    ex, nn, title, meta, url, note = piece
    tag = f"p{i}_{nn}"
    log(f"── piece {nn} · {ex} ──")
    rcpt = new_account("private", f"rcpt_{tag}")
    dfn  = new_account("public",  f"def_{tag}")
    mst  = new_account("public",  f"master_{tag}")
    mta  = new_account("public",  f"meta_{tag}")
    cpy  = new_account("public",  f"copy_{tag}")
    if not all([rcpt, dfn, mst, mta, cpy]):
        log("  account creation failed"); return False
    def_id = dfn.split("/", 1)[1]
    # recipient viewing key (two unlabeled hex lines)
    rc, keys = w("account", "show-keys", "--account-id", f"rcpt_{tag}")
    npks = [l.strip() for l in keys.splitlines() if re.fullmatch(r'[0-9a-f]{64}', l.strip())]
    vpks = [l.strip() for l in keys.splitlines() if re.fullmatch(r'[0-9a-f]{2000,}', l.strip())]
    if not npks or not vpks:
        log("  show-keys parse failed"); return False
    rc, seal = w("seal", "--to-vpk", vpks[0], "--to-npk", npks[0], "--definition-id", def_id,
                 "--url", url, "--title", title, "--meta", meta, "--exhibit", ex, "--note", note)
    um = re.search(r'sealed:v1:[0-9a-f]+', seal)
    if not um: log("  seal failed"); return False
    uri = um.group(0)
    rc, out = w("token", "new-nft", "--definition-account-id", f"def_{tag}",
                "--master-account-id", f"master_{tag}", "--metadata-account-id", f"meta_{tag}",
                "--name", f"{ex}||{nn}", "--printable-supply", "5", "--uri", uri, "--creators", "Logos EcoDev")
    if not included(out): log(f"  new-nft not included: {out[-120:]}"); return False
    log("  minted")
    rc, out = w("token", "print-nft", "--master-account-id", f"master_{tag}", "--printed-account-id", f"copy_{tag}")
    if not included(out): log(f"  print-nft not included: {out[-120:]}"); return False
    log("  printed; waiting for copy to hold the NFT…")
    if not copy_has_nft(f"copy_{tag}"):
        log("  copy never showed the NFT — skipping shield"); return False
    # shield with retry
    for attempt in range(1, 4):
        rc, out = w("token", "send", "--from", f"copy_{tag}", "--to", f"rcpt_{tag}", "--amount", "1")
        if included(out): log(f"  shielded (attempt {attempt})"); break
        if "Invalid sender data" in out or "panic" in out:
            log(f"  shield attempt {attempt} raced (sender empty); re-syncing…")
            w("account", "sync-private"); w("account", "get", "--account-id", f"copy_{tag}"); time.sleep(4)
        else:
            log(f"  shield failed: {out[-120:]}"); time.sleep(4)
    else:
        log("  shield failed after retries"); return False
    w("account", "sync-private")
    return True

def main():
    for f in ("storage.json", "statistics.json"):
        try: os.remove(os.path.join(HOME, f))
        except FileNotFoundError: pass
    want = {f"{ex}||{nn}" for (ex, nn, *_ ) in PIECES}
    # up to 3 passes: each pass seeds any still-missing piece
    for pass_no in range(1, 4):
        have = present_names()
        missing = [(i, p) for i, p in enumerate(PIECES) if f"{p[0]}||{p[1]}" not in have]
        log(f"=== pass {pass_no}: have {len(have)}/{len(PIECES)}; missing {[p[1] for _,p in missing]} ===")
        if not missing: break
        for i, p in missing:
            ok = seed_piece(i, p)
            log(f"  piece {p[1]} -> {'OK' if ok else 'FAILED'}")
    w("account", "sync-private")
    have = present_names()
    log(f"=== FINAL: {len(have)}/{len(PIECES)} present: {sorted(have)} ===")
    print("RESULT " + ("ALL_PRESENT" if have >= want else "MISSING:" + ",".join(sorted(want - have))))

if __name__ == "__main__":
    main()
