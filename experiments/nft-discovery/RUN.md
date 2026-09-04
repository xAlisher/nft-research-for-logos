# 0.1.2 — on-chain Sealed record discovery (live on our LEZ test node :3040)

`wallet sealed-records` walks every account the wallet holds, keeps the NFT holdings
(NftMaster / NftPrintedCopy), and resolves each:

    holding.definition_id -> TokenDefinition::NonFungible.metadata_id -> TokenMetadata.uri

emitting the records whose uri is a Sealed payload (`sealed:v1:…`) as JSON
`[{account, definitionId, name, metadataUri}]`. This is what the module gallery calls to
auto-populate — the recipient never pastes an id or uri.

## Walk (walk-d.sh, live node)
1. create public accounts (dnftdef/master/meta/copy)
2. `token new-nft … --uri "sealed:v1:<blob>"`  → included in block 1178
3. `token print-nft` (owned copy)              → included in block 1180
4. `wallet sealed-records`                      → JSON below

## Result (uri hex elided)
```json
[{"account":"Public/2GtLYP6oXPxAsUjSBD92w29YrgEE1b6xMMvHZm2sQYyT",
  "definitionId":"3cd7JVhtvji54uby4QjLtcnajSt9vvnxHPg8mBvx2cZn",
  "metadataUri":"sealed:v1:f7000000…356f84",
  "name":"Sealed Record 001"},
 {"account":"Public/GDqQo4epS7p3yKivN5WMgxQfLXuASfafNCqHoZtHCZwB",   // the NftMaster the wallet also holds
  "definitionId":"3cd7JVhtvji54uby4QjLtcnajSt9vvnxHPg8mBvx2cZn",
  "metadataUri":"sealed:v1:f7000000…356f84",
  "name":"Sealed Record 001"}]
```
Two records because this wallet both defined (holds the master) and printed (holds the copy)
the NFT; a real recipient holds only the printed copy. Resolution chain verified end-to-end.

Command: lez-work branch nft/epic-a-wallet, commit 4728627f (handle_sealed_records).
