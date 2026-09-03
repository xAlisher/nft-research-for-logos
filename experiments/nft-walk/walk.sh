#!/usr/bin/env bash
# A5 NFT dogfooding walk against the local LEZ node (:3040) on Sneg.
set -uo pipefail
export PATH="$HOME/.risc0/bin:$HOME/.cargo/bin:$PATH"
export RISC0_DEV_MODE=1
W="$HOME/nft-build/target/release/wallet"
HD="$HOME/nft-build/wallet-home"
mkdir -p "$HD"
cp -f "$HOME/nft-build/lez/lez/wallet/configs/debug/wallet_config.json" "$HD/wallet_config.json"
export LEE_WALLET_HOME_DIR="$HD"

run() { echo; echo "### \$ wallet $*"; "$W" "$@"; echo "   (exit $?)"; }

echo "===== node liveness ====="
tail -n 2 "$HOME/nft-build/sequencer.log" 2>/dev/null || echo "(no seq log)"

echo; echo "===== 1. create labeled public accounts ====="
run account new public --label nftdef
run account new public --label nftmaster
run account new public --label nftmeta
run account new public --label nftcopy
run account new public --label nftrecipient

echo; echo "===== 2. define the NFT (new-nft: definition + master + metadata) ====="
run token new-nft --definition-account-id nftdef --master-account-id nftmaster --metadata-account-id nftmeta --name "Genesis Private NFT" --printable-supply 5 --uri "ipfs://demo-cid" --creators "alisher"

echo; echo "===== 3. verify master holding (expect NftMaster) ====="
run account get --account-id nftmaster

echo; echo "===== 4. print an NFT copy (master -> nftcopy) ====="
run token print-nft --master-account-id nftmaster --printed-account-id nftcopy

echo; echo "===== 5. verify printed copy (expect owned) ====="
run account get --account-id nftcopy

echo; echo "===== 6. transfer the NFT copy (amount 1: nftcopy -> nftrecipient) ====="
run token send --from nftcopy --to nftrecipient --amount 1

echo; echo "===== 7. verify: sender no longer owns, recipient owns ====="
run account get --account-id nftcopy
run account get --account-id nftrecipient

echo; echo "===== WALK COMPLETE ====="
