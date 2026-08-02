import { spawnSync } from 'node:child_process'
const strict = process.argv.includes('--strict')
const checks = [
  ['node', ['scripts/verify-build.mjs'], 'structural build'],
  ['node', ['scripts/verify-config.mjs'], 'tauri config'],
  ['node', ['scripts/verify-updater-release.mjs'], 'signed updater configuration'],
  ['node', ['e2e/offline-smoke.mjs'], 'offline E2E'],
  ['node', ['e2e/gui-smoke.mjs', ...(strict ? ['--strict'] : [])], strict ? 'GUI E2E strict' : 'GUI E2E (SKIP when driver/app absent)'],
  ['node', ['scripts/smoke-installer.mjs', ...(strict ? ['--strict'] : [])], strict ? 'installer smoke strict' : 'installer smoke'],
]
for (const [cmd, args, label] of checks) {
  console.log(`\n=== ${label} ===`)
  const result = spawnSync(cmd, args, { cwd: process.cwd(), stdio: 'inherit', shell: false })
  if (result.error || result.status !== 0) process.exit(result.status ?? 1)
}
console.log('\nP5 verification passed.')
