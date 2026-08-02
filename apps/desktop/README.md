# Wireless Charging Research Workbench 0.6.x

Windows desktop client for the local wireless-charging research wiki. The app runs fully offline with the checked-in `public/data/library.json` snapshot; Luna and updater integrations are optional.

## Prerequisites
- Node.js 20+
- Rust stable + Tauri CLI (only for native packaging)
- Python 3.10+ for data export and wiki checks

## Development (offline)
```powershell
cd apps/desktop
npm ci
npm run data:build
npm run dev
```

## Verification
```powershell
npm run build
npm run verify:p5
```
`verify:p5` runs structural checks, config validation, and the offline E2E smoke test. No network or API credentials are required.

## Windows packaging
```powershell
npm run tauri build
npm run smoke:installer   # set INSTALLER_PATH to override auto-discovery
```
Artifacts are written under `src-tauri/target/release/bundle/`.

## Optional Luna configuration
Set `LUNA_API_KEY` (or the env name configured in the app) and an OpenAI-compatible endpoint only when online Q&A is desired. Missing values keep the app in offline evidence mode.

## Optional updater contract
The updater is intentionally configuration-only until a signed release endpoint is provisioned. Set:
- `TAURI_UPDATER_ENDPOINT`: HTTPS JSON update manifest endpoint
- `TAURI_UPDATER_PUBKEY`: Tauri updater public key

Builds without either variable remain valid and offline. Before enabling production updates, wire the endpoint and key into `src-tauri/tauri.conf.json` and add the Tauri updater plugin in the Rust crate.
