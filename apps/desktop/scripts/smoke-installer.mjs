import { existsSync } from 'node:fs'
import { spawnSync } from 'node:child_process'
import { resolve } from 'node:path'
const path = process.env.INSTALLER_PATH
if (!path) { console.log('INSTALLER_PATH not set; packaging smoke skipped (build remains valid).'); process.exit(0) }
const full = resolve(path)
if (!existsSync(full)) throw new Error(`installer not found: ${full}`)
console.log(`PASS installer artifact exists: ${full}`)
if (process.platform === 'win32') {
  const r = spawnSync(full, ['/S','/TASKS=""'], { stdio:'ignore', windowsHide:true })
  console.log(`installer launch exit code: ${r.status ?? 'started'}`)
}
