# Logos Storage — discovery & fetch model (fallback transport)

_Research by Sina agent, 2026-09-03. For when a payload exceeds the on-chain 100 KiB cap (large media) and must live off-chain. Evidence tags: [CODE — X%]/[CONFIRMED — X%]/[H — X%]._

## Summary verdict
**Cross-node fetch by CID WORKS on testnet — discovery is real (DHT), but it is NOT store-and-forget.** A recipient does **not** need the host's address in advance; given a CID and a shared bootstrap network, their node locates + pulls the blob via the DHT. **BUT** the curator's hosting node must stay **online and internet-reachable** until all recipients have pulled — there is no guaranteed replication (block TTL ~30 days, no active marketplace). [CODE/H — 85%]

## Findings
- **Codex = Logos Storage** (module `logos-storage-module` over engine `logos-storage-nim`, formerly nim-codex). [CODE — 99%] docs glossary
- **Upload → CID:** `uploadUrl(path)` (bg, 64 KiB chunks) → `storageUploadDone` → CID from the manifest. Blocks in local repo (`data-dir`, 20 GiB quota), `block-ttl ≈ 30d`. [CODE — 95%] `storage_module_interface.h`, docs run-logos-storage-node
- **Discovery IS present + kept from Codex:** a DHT on `disc-port` (UDP 8090) does "discovering peers (finding who is out there and where content lives) and transferring data"; nodes advertise provider records ("the peerId has to be advertised in the DHT"). [CODE — 97%] docs storage/concepts/connectivity, interface.h connect()
- **Network fetch:** `fetch(cid)` = "fetch content identified by a CID from the network"; `downloadToUrl(cid, …, local=false)` goes through the network if not local. [CODE — 98%] interface.h
- **Join = bootstrap:** use `network: logos.test` (testnet) preset (carries bootstrap SPRs); then peers self-discover. [CODE — 97%] docs connectivity
- **Critical constraint — host reachability:** the blob lives only on whoever holds the blocks. If that node is offline or NAT-unreachable, peers' fetch **times out**. FAQ: "I can download files, but nobody can download from me" = "your node is unreachable… incoming connections are blocked"; "downloads time out from a different machine" = "the publishing node is not reachable." [CODE — 98%] docs FAQ
- **Reachability:** fixed `listen-port`(TCP)+`disc-port`(UDP) + port-forward + `nat:extip:<IP>`, or a relay fallback with hole-punching (lower perf). Automatic port-mapping "currently being tested, not available for now." [CODE — 92%] docs connectivity
- **Resilience improves after first pull:** once a recipient downloads, their node also serves those blocks (multi-source) — but the *first* fetch depends on the curator. [H — 70%]

## Implication for Sealed
- **Default = metadata.uri (on-chain, 100 KiB cap).** The `{link, note}` blob (~2 KB) fits on-chain, is durable, and needs **no** curator node online — strictly better than Storage for this payload. **Confirmed the right default.**
- **Storage is the fallback ONLY for large media** (actual document scans/images that exceed 100 KiB). Then: uri holds the CID; run the **curator node on a VPS/public IP** (fixed forwarded ports, `nat:extip`) and keep it up until recipients pull; do NOT assume go-dark-after-upload; mirror to Arweave/pinned-IPFS if permanence matters.
- A home/NAT curator relying on the (still-in-testing) relay is the fragile case ("nobody can download from me").

## Sources
docs.logos.co: storage/concepts/connectivity · storage/get-started/faq · storage/get-started/run-logos-storage-node · get-started/glossary. Repo: `~/basecamp/refs/logos-storage-module/src/storage_module_interface.h`, `README.md`. Engine (referenced, not checked out): github.com/logos-storage/logos-storage-nim.
