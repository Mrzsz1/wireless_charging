import { existsSync } from 'node:fs'
import { spawn, spawnSync } from 'node:child_process'
import { remote } from 'webdriverio'

const strict = process.argv.includes('--strict') || process.env.E2E_STRICT === '1'
const app = process.env.TAURI_APP_PATH
const driver = process.env.TAURI_DRIVER || 'tauri-driver'
if (!app || !existsSync(app)) {
  const msg = 'GUI E2E SKIP: set TAURI_APP_PATH to the built .exe and install tauri-driver (cargo install tauri-driver --locked).'
  if (strict) { console.error(msg); process.exit(2) }
  console.log(msg); process.exit(0)
}
const probe = spawnSync(driver, ['--version'], { stdio: 'ignore', shell: true })
if (probe.status !== 0) {
  const msg = 'GUI E2E SKIP: tauri-driver unavailable; install with cargo install tauri-driver --locked.'
  if (strict) { console.error(msg); process.exit(2) }
  console.log(msg); process.exit(0)
}
const driverProcess = spawn(driver, [], { shell: true, stdio: 'ignore' })
await new Promise((resolve) => setTimeout(resolve, 1200))
const browser = await remote({ hostname: '127.0.0.1', port: 4444, capabilities: { 'tauri:options': { application: app }, browserName: 'wry' } })
try {
  await browser.pause(1500)
  const requireElement = async (selector) => {
    const el = await browser.$(selector)
    if (!await el.isExisting()) throw new Error(`Missing required GUI element: ${selector}`)
    console.log(`PASS ${selector}`)
    return el
  }
  await requireElement('[data-testid="sidebar"]')
  const space = await requireElement('[data-testid="space-toggle"]')
  const before = await space.getAttribute('aria-expanded')
  await space.click()
  const after = await space.getAttribute('aria-expanded')
  if (before === after) throw new Error('Workspace tree did not toggle')
  await (await requireElement('[data-testid="nav-library"]')).click()
  await requireElement('[data-testid="library-search"]')
  await (await requireElement('[data-testid="nav-qa"]')).click()
  await requireElement('[data-testid="qa-input"]')
  await (await requireElement('[data-testid="nav-compile"]')).click()
  await requireElement('[data-testid="compile-center"]')
  await (await requireElement('[data-testid="settings"]')).click()
  console.log('PASS GUI E2E launch/navigation probe')
} finally {
  await browser.deleteSession()
  driverProcess.kill()
}
