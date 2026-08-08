import { existsSync } from 'node:fs'
import { spawn, spawnSync } from 'node:child_process'
import { dirname, extname, resolve } from 'node:path'
import { assertProcessStaysAlive, terminateProcessTree } from './process-lifecycle.mjs'

const strict = process.argv.includes('--strict') || process.env.E2E_STRICT === '1'
const installerPath = process.env.INSTALLER_PATH
if (!installerPath) {
  const message = 'INSTALLER_PATH not set; packaging smoke skipped (build remains valid).'
  if (strict) { console.error(message); process.exit(2) }
  console.log(message)
  process.exit(0)
}
const full = resolve(installerPath)
if (!existsSync(full)) throw new Error(`installer not found: ${full}`)
console.log(`PASS installer artifact exists: ${full}`)
if (!strict) process.exit(0)
if (process.platform !== 'win32') throw new Error('strict installer smoke currently requires Windows')

const appPath = process.env.INSTALLER_APP_PATH ? resolve(process.env.INSTALLER_APP_PATH) : ''
const uninstallerPath = process.env.INSTALLER_UNINSTALLER_PATH ? resolve(process.env.INSTALLER_UNINSTALLER_PATH) : ''
if (!appPath || !uninstallerPath) {
  throw new Error('strict installer smoke requires INSTALLER_APP_PATH and INSTALLER_UNINSTALLER_PATH')
}

const extension = extname(full).toLowerCase()
const install = extension === '.msi'
  ? spawnSync('msiexec.exe', ['/i', full, '/qn', '/norestart'], { stdio: 'inherit', windowsHide: true })
  : spawnSync(full, ['/S', `/D=${dirname(appPath)}`], { stdio: 'inherit', windowsHide: true })
if (install.error || install.status !== 0) throw new Error(`installer failed with ${install.status ?? install.error}`)
if (!existsSync(appPath)) throw new Error(`installed application not found: ${appPath}`)
console.log(`PASS installed application exists: ${appPath}`)

const app = spawn(appPath, [], { stdio: 'ignore', windowsHide: true })
try {
  await assertProcessStaysAlive(app)
  console.log(`PASS installed application launched: PID ${app.pid}`)
} finally {
  await terminateProcessTree(app)
}
console.log('PASS installed application process exited')

if (!existsSync(uninstallerPath)) throw new Error(`uninstaller not found: ${uninstallerPath}`)
const uninstall = spawnSync(uninstallerPath, ['/S'], { stdio: 'inherit', windowsHide: true })
if (uninstall.error || uninstall.status !== 0) throw new Error(`uninstaller failed with ${uninstall.status ?? uninstall.error}`)
for (let attempt = 0; attempt < 60 && existsSync(appPath); attempt += 1) {
  await new Promise((resolveDelay) => setTimeout(resolveDelay, 250))
}
if (existsSync(appPath)) throw new Error(`application remains after uninstall: ${appPath}`)
console.log('PASS installer install/launch/uninstall smoke')
