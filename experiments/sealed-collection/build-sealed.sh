#!/usr/bin/env bash
# Build the "Sealed" testnet NFT collection on the live LEZ node.
# Mints one definition, prints an edition, shields some pieces (sealed) and leaves
# others public (revealed). Emits a manifest the gallery consumes.
set -uo pipefail
export PATH="$HOME/.risc0/bin:$HOME/.cargo/bin:$PATH"
export RISC0_DEV_MODE=1
W="$HOME/nft-build/target/release/wallet"
HD="$HOME/nft-build/sealed-home"; rm -rf "$HD"; mkdir -p "$HD"
cp -f "$HOME/nft-build/lez/lez/wallet/configs/debug/wallet_config.json" "$HD/wallet_config.json"
export LEE_WALLET_HOME_DIR="$HD"
OUT="$HOME/nft-build/sealed-manifest.txt"; : > "$OUT"

B58='[1-9A-HJ-NP-Za-km-z]'
newpub()  { $W account new public  --label "$1" 2>/dev/null | grep -oE "Public/${B58}+"  | head -1 | cut -d/ -f2; }
newpriv() { $W account new private --label "$1" 2>/dev/null | grep -oE "Private/${B58}+" | head -1 | cut -d/ -f2; }

echo "== define the collection =="
DEF=$(newpub sdef); MASTER=$(newpub smaster); META=$(newpub smeta)
$W token new-nft --definition-account-id sdef --master-account-id smaster --metadata-account-id smeta \
  --name "Sealed Genesis" --printable-supply 12 \
  --uri "logos-storage://sealed-genesis" --creators "Logos EcoDev" >/dev/null
echo "definition|$DEF" >> "$OUT"
echo "  definition=$DEF"

N=6
echo "== print $N pieces =="
for i in $(seq 1 $N); do
  PID=$(newpub "piece$i")
  $W token print-nft --master-account-id smaster --printed-account-id "piece$i" >/dev/null
  echo "print piece$i -> $PID"
  # pieces 1-3 = SEALED (shield into a private owner); 4-6 = REVEALED (public)
  if [ "$i" -le 3 ]; then
    OWNER=$(newpriv "owner$i")
    $W token send --from "piece$i" --to "owner$i" --amount 1 >/dev/null
    KEYS=$($W account show-keys --account-id "owner$i" --viewing-secret 2>/dev/null)
    NPK=$(printf '%s\n' "$KEYS" | sed -n '1p')
    VD=$(printf '%s\n' "$KEYS" | awk '/^vsk_d/{print $2}')
    VZ=$(printf '%s\n' "$KEYS" | awk '/^vsk_z/{print $2}')
    echo "piece|$i|sealed|$PID|$OWNER|$NPK|$VD|$VZ" >> "$OUT"
    echo "  piece$i SEALED -> owner$i ($OWNER)"
  else
    echo "piece|$i|revealed|$PID|||" >> "$OUT"
    echo "  piece$i REVEALED (public $PID)"
  fi
done

echo
echo "== manifest ($OUT) =="
cat "$OUT"
echo "== SEALED COLLECTION BUILT =="
