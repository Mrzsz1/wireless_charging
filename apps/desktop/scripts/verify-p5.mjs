import { spawnSync } from 'node:child_process'
const checks = [
  ['node',['scripts/verify-build.mjs'],'structural build'],
  ['node',['scripts/verify-config.mjs'],'tauri config'],
  ['node',['e2e/offline-smoke.mjs'],'offline E2E'],
  ['node',['e2e/gui-smoke.mjs'],'GUI E2E (SKIP when driver/app absent)'],
  ['node',['scripts/smoke-installer.mjs'],'installer smoke'],
]
for (const [cmd,args,label] of checks) {
  console.log(`\n=== ${label} ===`)
  const r = spawnSync(cmd,args,{cwd:process.cwd(),stdio:'inherit',shell:false})
  if (r.error || r.status !== 0) process.exit(r.status ?? 1)
}
console.log('\nP5 verification passed.')
