#!/usr/bin/env python3
"""Sealed collection — encrypted-payload reveal mechanic.

Each NFT carries an encrypted payload (an Internet Archive link + a short curatorial note). It is unreadable
until REVEAL: the owner re-derives a per-piece key from their viewing secret and
decrypts. Wrong key -> authentication fails -> nothing is revealed.

This is a dependency-free, faithful stand-in for the on-chain path already proven
in the LEZ (ML-KEM-768 key agreement + ChaCha20 note encryption, decoded via the
viewing key -- see experiments/nft-selective-disclosure). Here we use a SHA256-CTR
keystream + HMAC-SHA256 tag so it runs anywhere with just the stdlib; the shape is
identical: key-from-viewing-secret -> stream cipher -> authenticated ciphertext.
"""
from __future__ import annotations
import base64, hashlib, hmac, os, json, sys

DOMAIN = b"SEALED/v1/"

def derive_key(viewing_secret_hex: str, piece_id: str) -> bytes:
    """Per-piece 32-byte key bound to the owner's viewing secret and the piece id."""
    vsk = bytes.fromhex(viewing_secret_hex)
    return hashlib.sha256(DOMAIN + vsk + b"|" + piece_id.encode()).digest()

def _keystream(key: bytes, nonce: bytes, n: int) -> bytes:
    out = bytearray()
    counter = 0
    while len(out) < n:
        out += hashlib.sha256(key + nonce + counter.to_bytes(8, "big")).digest()
        counter += 1
    return bytes(out[:n])

def seal(url: str, key: bytes) -> str:
    """Encrypt a URL -> opaque base64 blob (nonce | ciphertext | tag)."""
    nonce = os.urandom(16)
    pt = url.encode()
    ct = bytes(a ^ b for a, b in zip(pt, _keystream(key, nonce, len(pt))))
    tag = hmac.new(key, nonce + ct, hashlib.sha256).digest()[:16]
    return base64.b64encode(nonce + ct + tag).decode()

def reveal(blob_b64: str, key: bytes) -> str | None:
    """Decrypt with the right key; return None if the key is wrong (tag mismatch)."""
    raw = base64.b64decode(blob_b64)
    nonce, ct, tag = raw[:16], raw[16:-16], raw[-16:]
    if not hmac.compare_digest(tag, hmac.new(key, nonce + ct, hashlib.sha256).digest()[:16]):
        return None
    pt = bytes(a ^ b for a, b in zip(ct, _keystream(key, nonce, len(ct))))
    return pt.decode(errors="replace")

def redacted(blob_b64: str, width: int = 44) -> str:
    """How the sealed payload looks to the public: a redaction bar."""
    return "█" * width  # full-block bar

# ---- demo ----
if __name__ == "__main__":
    # A stand-in owner viewing secret (hex). In the real flow this is the vsk from
    # `wallet account show-keys --viewing-secret`.
    OWNER_VSK = "f66fc630c2803e4b40c8b10496b00b19424ee922278d9e7a0a78ca80f9a784ea"
    WRONG_VSK = "00" * 32

    # Each payload = the Internet Archive link + a short curatorial note ("why it's in the
    # Museum"). BOTH are encrypted; the whole payload is revealed together on decrypt.
    pieces = [
        ("piece-01", {"url": "https://archive.org/details/pdfy-MHvlymfJYU05yELW",
                      "note": "By decree, holding your own gold became a crime."}),
        ("piece-11", {"url": "https://archive.org/details/FBI-COINTELPRO-BLACK",
                      "note": "The FBI's covert program to 'expose, disrupt, and neutralize' dissent."}),
        ("piece-14", {"url": "https://web.archive.org/web/20161106014250/https://www.epa.gov/climatechange",
                      "note": "A government science page, deleted at a change of power."}),
    ]

    print("== SEAL (mint time) ==")
    sealed = []
    for pid, payload in pieces:
        k = derive_key(OWNER_VSK, pid)
        blob = seal(json.dumps(payload), k)
        sealed.append((pid, blob))
        print(f"  {pid}: SEALED -> {redacted(blob)}")
        print(f"           blob = {blob[:56]}...")

    print("\n== REVEAL with the CORRECT viewing key (owner) ==")
    for pid, blob in sealed:
        k = derive_key(OWNER_VSK, pid)
        p = json.loads(reveal(blob, k))
        print(f"  {pid}: {p['url']}")
        print(f"           note: {p['note']}")

    print("\n== REVEAL with a WRONG viewing key (anyone else) ==")
    for pid, blob in sealed:
        k = derive_key(WRONG_VSK, pid)
        r = reveal(blob, k)
        print(f"  {pid}: {'<cannot reveal>' if r is None else r}")

    print("\nMechanic OK: the link + curatorial note are unreadable until revealed with the owner's viewing key.")
