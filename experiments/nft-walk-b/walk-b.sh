#!/usr/bin/env bash
# Epic B: private NFT ownership walk (shield -> private transfer -> deshield) vs the live LEZ node.
set -uo pipefail
export PATH="$HOME/.risc0/bin:$HOME/.cargo/bin:$PATH"
export RISC0_DEV_MODE=1
W="$HOME/nft-build/target/release/wallet"
HD="$HOME/nft-build/wallet-home-b-$(date +%s)"
mkdir -p "$HD"
cp -f "$HOME/nft-build/lez/lez/wallet/configs/debug/wallet_config.json" "$HD/wallet_config.json"
export LEE_WALLET_HOME_DIR="$HD"
run() { echo; echo "### \$ wallet $*"; "$W" "$@"; echo "   (exit $?)"; }

echo "===== accounts ====="
run account new public  --label bdef
run account new public  --label bmaster
run account new public  --label bmeta
run account new public  --label bpubcopy
run account new private --label bpriv1
run account new private --label bpriv2
run account new public  --label bpubfinal

echo; echo "===== public setup: define NFT + print into a public holder ====="
run token new-nft --definition-account-id bdef --master-account-id bmaster --metadata-account-id bmeta --name "Private NFT Demo" --printable-supply 5 --uri "ipfs://demo" --creators "alisher"
run token print-nft --master-account-id bmaster --printed-account-id bpubcopy
run account get --account-id bpubcopy

echo; echo "===== B1 SHIELD: public -> private (bpubcopy -> bpriv1) ====="
run token send --from bpubcopy --to bpriv1 --amount 1
run account get --account-id bpubcopy
run account get --account-id bpriv1

echo; echo "===== B2 PRIVATE -> PRIVATE (bpriv1 -> bpriv2) ====="
run token send --from bpriv1 --to bpriv2 --amount 1
run account get --account-id bpriv1
run account get --account-id bpriv2

echo; echo "===== B3 DESHIELD: private -> public (bpriv2 -> bpubfinal) ====="
run token send --from bpriv2 --to bpubfinal --amount 1
run account get --account-id bpriv2
run account get --account-id bpubfinal

echo; echo "===== EPIC B WALK COMPLETE ====="
