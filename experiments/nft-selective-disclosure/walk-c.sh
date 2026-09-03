#!/usr/bin/env bash
# Epic C3: live selective-disclosure verifier demo vs the LEZ node.
set -uo pipefail
export PATH="$HOME/.risc0/bin:$HOME/.cargo/bin:$PATH"
export RISC0_DEV_MODE=1
W="$HOME/nft-build/target/release/wallet"
HD="$HOME/nft-build/wallet-home-c3-$(date +%s)"; mkdir -p "$HD"
cp -f "$HOME/nft-build/lez/lez/wallet/configs/debug/wallet_config.json" "$HD/wallet_config.json"
export LEE_WALLET_HOME_DIR="$HD"

echo "== setup: accounts =="
$W account new public  --label cdef    >/dev/null
$W account new public  --label cmaster >/dev/null
$W account new public  --label cmeta   >/dev/null
$W account new public  --label ccopy   >/dev/null
$W account new private --label cowner  >/dev/null

echo "== define + print, then SHIELD the NFT into the private owner =="
$W token new-nft --definition-account-id cdef --master-account-id cmaster --metadata-account-id cmeta --name "Sealed #1" --printable-supply 3 --uri "logos-storage://demo-cid" --creators "alisher" >/dev/null
$W token print-nft --master-account-id cmaster --printed-account-id ccopy >/dev/null
$W token send --from ccopy --to cowner --amount 1 >/dev/null
echo "owner (private) holding:"
$W account get --account-id cowner | grep -E "Nft|owned" || true

echo "== owner exports viewing key (the read capability) =="
KEYS=$($W account show-keys --account-id cowner --viewing-secret)
NPK=$(printf '%s\n' "$KEYS" | sed -n '1p')
VSKD=$(printf '%s\n' "$KEYS" | awk '/^vsk_d/{print $2}')
VSKZ=$(printf '%s\n' "$KEYS" | awk '/^vsk_z/{print $2}')
echo "npk=$NPK"
echo "vsk_d=$VSKD"
echo "vsk_z=$VSKZ"

echo
echo "===== VERIFIER with the CORRECT viewing key (should disclose the NFT) ====="
$W verify-disclosure --npk "$NPK" --vsk-d "$VSKD" --vsk-z "$VSKZ"

echo
echo "===== VERIFIER with a WRONG viewing key (should disclose nothing) ====="
$W verify-disclosure --npk "$NPK" \
  --vsk-d 0000000000000000000000000000000000000000000000000000000000000000 \
  --vsk-z 1111111111111111111111111111111111111111111111111111111111111111

echo
echo "===== C3 DEMO COMPLETE ====="
