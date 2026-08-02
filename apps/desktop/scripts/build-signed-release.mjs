import { existsSync, readFileSync, readdirSync, writeFileSync, unlinkSync } from 'node:fs'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { dirname, resolve } from 'node:path'

const here = dirname(fileURLToPath(import.meta.url))
const desktop = resolve(here, '..')
const temporaryConfig = resolve(desktop, 'src-tauri', '.updater-release.json')
const endpoint = process.env.TAURI_UPDATER_ENDPOINT?.trim()
const pubkey = process.env.TAURI_UPDATER_PUBKEY?.trim()
const privateKey = process.env.TAURI_SIGNING_PRIVATE_KEY?.trim()

if (!endpoint || !pubkey || !privateKey) {
  console.error('Set TAURI_UPDATER_ENDPOINT, TAURI_UPDATER_PUBKEY and TAURI_SIGNING_PRIVATE_KEY before a signed release build.')
  process.exit(2)
}

let endpointUrl
try {
  endpointUrl = new URL(endpoint)
} catch {
  console.error('TAURI_UPDATER_ENDPOINT must be a valid HTTPS URL.')
  process.exit(2)
}
if (endpointUrl.protocol !== 'https:') {
  console.error('TAURI_UPDATER_ENDPOINT must use HTTPS for release builds.')
  process.exit(2)
}
if (!/^[A-Za-z0-9+/=\r\n]+$/.test(pubkey) || pubkey.replace(/\s/g, '').length < 32) {
  console.error('TAURI_UPDATER_PUBKEY must be a non-empty base64-encoded minisign public key.')
  process.exit(2)
}

const base = JSON.parse(readFileSync(resolve(desktop, 'src-tauri', 'tauri.conf.json'), 'utf8'))
base.bundle.createUpdaterArtifacts = true
base.plugins = {
  ...(base.plugins ?? {}),
  updater: { active: true, endpoints: [endpoint], pubkey },
}
writeFileSync(temporaryConfig, `${JSON.stringify(base, null, 2)}\n`)

try {
  if (process.argv.includes('--validate-only')) {
    console.log('PASS signed updater release configuration')
    process.exitCode = 0
  } else {
    const executable = process.platform === 'win32' ? 'npx.cmd' : 'npx'
    const result = spawnSync(executable, ['tauri', 'build', '--config', temporaryConfig], {
      cwd: desktop,
      env: process.env,
      shell: false,
      stdio: 'inherit',
    })
    if (result.error || result.status !== 0) {
      process.exitCode = result.status ?? 1
    } else {
      const bundleRoot = resolve(desktop, 'src-tauri', 'target', 'release', 'bundle')
      const files = []
      const visit = (directory) => {
        if (!existsSync(directory)) return
        for (const entry of readdirSync(directory, { withFileTypes: true })) {
          const path = resolve(directory, entry.name)
          if (entry.isDirectory()) visit(path)
          else files.push(path)
        }
      }
      visit(bundleRoot)
      const signatures = files.filter((path) => path.endsWith('.sig'))
      if (!signatures.length) {
        console.error(`Signed release completed without updater signature artifacts under ${bundleRoot}.`)
        process.exitCode = 3
      } else {
        console.log(`PASS ${signatures.length} updater signature artifact(s) generated.`)
      }
    }
  }
} finally {
  if (existsSync(temporaryConfig)) unlinkSync(temporaryConfig)
}
