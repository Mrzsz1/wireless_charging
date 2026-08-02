import { existsSync } from 'node:fs'
import { spawn, spawnSync } from 'node:child_process'
import { createConnection } from 'node:net'
import { remote } from 'webdriverio'

const strict = process.argv.includes('--strict') || process.env.E2E_STRICT === '1'
const app = process.env.TAURI_APP_PATH
const driver = process.env.TAURI_DRIVER || 'tauri-driver'
const nativeDriver = process.env.TAURI_NATIVE_DRIVER
const directNative = Boolean(nativeDriver)
if (!app || !existsSync(app)) {
  const msg = 'GUI E2E SKIP: set TAURI_APP_PATH to the built .exe and install tauri-driver (cargo install tauri-driver --locked).'
  if (strict) { console.error(msg); process.exit(2) }
  console.log(msg); process.exit(0)
}
const driverExecutable = directNative ? nativeDriver : driver
const probe = spawnSync(driverExecutable, [directNative ? '--version' : '--help'], { stdio: 'ignore', shell: false })
if (probe.status !== 0) {
  const msg = 'GUI E2E SKIP: tauri-driver unavailable; install with cargo install tauri-driver --locked.'
  if (strict) { console.error(msg); process.exit(2) }
  console.log(msg); process.exit(0)
}
const driverArgs = directNative ? ['--port=4444'] : []
const driverProcess = spawn(driverExecutable, driverArgs, { shell: false, stdio: process.env.E2E_DRIVER_LOG === '1' ? 'inherit' : 'ignore' })
const waitForPort = async (timeoutMs = 15000) => {
  const deadline = Date.now() + timeoutMs
  while (Date.now() < deadline) {
    const ready = await new Promise((resolve) => {
      const socket = createConnection({ host: '127.0.0.1', port: 4444 })
      socket.once('connect', () => { socket.destroy(); resolve(true) })
      socket.once('error', () => resolve(false))
      socket.setTimeout(500, () => { socket.destroy(); resolve(false) })
    })
    if (ready) return
    if (driverProcess.exitCode !== null) throw new Error(`tauri-driver exited with ${driverProcess.exitCode}`)
    await new Promise((resolve) => setTimeout(resolve, 200))
  }
  throw new Error('tauri-driver did not become ready on port 4444')
}
await waitForPort()
const capabilities = directNative
  ? { browserName: 'webview2', 'ms:edgeOptions': { binary: app, args: [] }, 'wdio:enforceWebDriverClassic': true }
  : { 'tauri:options': { application: app }, browserName: 'wry', 'wdio:enforceWebDriverClassic': true }
const browser = await remote({ hostname: '127.0.0.1', port: 4444, capabilities })
try {
  await browser.waitUntil(async () => {
    for (const handle of await browser.getWindowHandles()) {
      await browser.switchToWindow(handle)
      if (await (await browser.$('[data-testid="sidebar"]')).isExisting()) return true
    }
    return false
  }, {
    timeout: 20000,
    interval: 250,
    timeoutMsg: 'application WebView did not expose the workspace shell',
  })
  const requireElement = async (selector) => {
    const el = await browser.$(selector)
    await el.waitForExist({ timeout: 15000, timeoutMsg: `Missing required GUI element: ${selector}` })
    console.log(`PASS ${selector}`)
    return el
  }
  const verifyViewport = async (width, height) => {
    await browser.setWindowSize(width, height)
    const metrics = await browser.execute(() => ({
      clientWidth: document.documentElement.clientWidth,
      scrollWidth: document.documentElement.scrollWidth,
      bodyWidth: document.body.getBoundingClientRect().width,
    }))
    if (metrics.scrollWidth > metrics.clientWidth + 1 || metrics.bodyWidth > metrics.clientWidth + 1) {
      throw new Error(`document overflows at ${width}x${height}: ${JSON.stringify(metrics)}`)
    }
    console.log(`PASS viewport ${width}x${height}`)
  }
  await verifyViewport(1366, 768)
  await requireElement('[data-testid="sidebar"]')
  const search = await requireElement('[data-testid="global-search"]')
  await browser.keys(['\uE009', 'k', '\uE000'])
  if (!await search.isFocused()) throw new Error('Ctrl+K did not focus global search')
  const space = await requireElement('[data-testid="space-toggle"]')
  const before = await space.getAttribute('aria-expanded')
  await space.click()
  const after = await space.getAttribute('aria-expanded')
  if (before === after) throw new Error('Workspace tree did not toggle')
  await (await requireElement('[data-testid="nav-library"]')).click()
  await requireElement('[data-testid="library-search"]')
  await (await requireElement('[data-testid="nav-books"]')).click()
  await requireElement('[data-testid="books-view"]')
  await (await requireElement('[data-testid="nav-graph"]')).click()
  await requireElement('[data-testid="graph-view"]')
  await (await requireElement('[data-testid="nav-qa"]')).click()
  await requireElement('[data-testid="qa-input"]')
  await (await requireElement('[data-testid="nav-compile"]')).click()
  await requireElement('[data-testid="compile-center"]')
  await (await requireElement('[data-testid="settings"]')).click()
  await requireElement('[data-testid="updater-settings"]')
  await verifyViewport(1920, 1080)
  console.log('PASS GUI E2E launch/navigation probe')
} finally {
  await browser.deleteSession()
  if (process.platform === 'win32') {
    spawnSync('taskkill', ['/PID', String(driverProcess.pid), '/T', '/F'], { stdio: 'ignore' })
  } else {
    driverProcess.kill('SIGTERM')
  }
  await new Promise((resolve) => driverProcess.exitCode !== null ? resolve() : driverProcess.once('exit', resolve))
}
