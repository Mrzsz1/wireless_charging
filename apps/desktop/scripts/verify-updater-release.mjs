import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { manifestFor } from './updater-fixture-server.mjs'

const script = fileURLToPath(new URL('./build-signed-release.mjs', import.meta.url))
const common = {
  ...process.env,
  TAURI_UPDATER_PUBKEY: 'QUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFB',
  TAURI_SIGNING_PRIVATE_KEY: 'LOCAL_FIXTURE_PRIVATE_KEY',
}

const run = (endpoint) => spawnSync(process.execPath, [script, '--validate-only'], {
  cwd: process.cwd(),
  env: { ...common, TAURI_UPDATER_ENDPOINT: endpoint },
  encoding: 'utf8',
  shell: false,
})

const insecure = run('http://127.0.0.1:4179/latest.json')
if (insecure.status === 0 || !insecure.stderr.includes('must use HTTPS')) {
  throw new Error(`insecure updater endpoint was not rejected: ${insecure.stdout}${insecure.stderr}`)
}
const secure = run('https://updates.example.test/{{target}}/{{arch}}/{{current_version}}')
if (secure.status !== 0 || !secure.stdout.includes('PASS signed updater release configuration')) {
  throw new Error(`valid signed updater configuration was rejected: ${secure.stdout}${secure.stderr}`)
}
console.log('PASS updater release configuration validation')

const origin = 'http://127.0.0.1:4179'
const noUpdate = manifestFor('no-update', origin)
const update = manifestFor('update', origin)
const tampered = manifestFor('tampered', origin)
if (noUpdate.version !== '0.7.0' || update.version !== '0.7.1') throw new Error('updater fixture versions are invalid')
if (update.platforms['windows-x86_64'].signature === tampered.platforms['windows-x86_64'].signature) {
  throw new Error('tampered updater fixture must change the signature')
}
console.log('PASS updater no-update/update/tampered fixture manifests')
