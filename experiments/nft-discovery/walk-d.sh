#!/usr/bin/env bash
# 0.1.2 discovery walk against the local LEZ node (:3040) on Sneg.
# Define an NFT whose metadata.uri is a sealed:v1: blob, print it to a wallet-owned
# account, then prove `wallet sealed-records` auto-discovers + resolves it.
set -uo pipefail
export PATH="$HOME/.risc0/bin:$HOME/.cargo/bin:$PATH"
export RISC0_DEV_MODE=1
W="$HOME/nft-build/target/debug/wallet"      # debug: carries the new sealed-records/unseal cmds
HD="$HOME/nft-build/wallet-home-disc-$(date +%s)"
mkdir -p "$HD"
cp -f "$HOME/nft-build/lez/lez/wallet/configs/debug/wallet_config.json" "$HD/wallet_config.json"
export LEE_WALLET_HOME_DIR="$HD"

URI=$(grep FIXTURE_URI /tmp/fixture.txt | cut -d= -f2-)
echo "sealed uri: ${URI:0:32}… (${#URI} chars)"

run() { echo; echo "### \$ wallet $*"; "$W" "$@"; echo "   (exit $?)"; }

echo "===== 1. accounts ====="
run account new public --label dnftdef
run account new public --label dnftmaster
run account new public --label dnftmeta
run account new public --label dnftcopy

echo "===== 2. define the NFT with a SEALED metadata.uri ====="
run token new-nft --definition-account-id dnftdef --master-account-id dnftmaster \
    --metadata-account-id dnftmeta --name "Sealed Record 001" --printable-supply 5 \
    --uri "$URI" --creators "Logos EcoDev"

echo "===== 3. print a copy the wallet owns ====="
run token print-nft --master-account-id dnftmaster --printed-account-id dnftcopy

echo "===== 4. DISCOVER: wallet sealed-records (auto-resolve definition -> metadata -> uri) ====="
run sealed-records
