import { readFileSync, writeFileSync, rmSync } from 'node:fs'
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

const base = JSON.parse(readFileSync(resolve(desktop, 'src-tauri', 'tauri.conf.json'), 'utf8'))
base.bundle.createUpdaterArtifacts = true
base.plugins = {
  ...(base.plugins ?? {}),
  updater: { active: true, endpoints: [endpoint], pubkey },
}
writeFileSync(temporaryConfig, `${JSON.stringify(base, null, 2)}\n`)

try {
  const result = spawnSync('npx', ['tauri', 'build', '--config', temporaryConfig], {
    cwd: desktop,
    env: process.env,
    shell: true,
    stdio: 'inherit',
  })
  process.exitCode = result.status ?? 1
} finally {
  rmSync(temporaryConfig, { force: true })
}
