import { existsSync } from 'node:fs'
import { spawn, spawnSync } from 'node:child_process'
import { createConnection } from 'node:net'
import { remote } from 'webdriverio'
import { formatCandidates, resolveAppPath, resolveDriver } from './gui-config.mjs'

const strict = process.argv.includes('--strict') || process.env.E2E_STRICT === '1'
const nativeDriver = process.env.TAURI_NATIVE_DRIVER
const directNative = Boolean(nativeDriver)
const appResolution = resolveAppPath()
const app = appResolution.path
const driverResolution = resolveDriver({ explicit: directNative ? nativeDriver : null })
const driver = driverResolution.executable

const finishUnavailable = (message) => {
  if (strict) { console.error(message); process.exit(2) }
  console.log(message); process.exit(0)
}

if (!app) {
  const detail = appResolution.explicit
    ? `TAURI_APP_PATH does not point to a file: ${appResolution.requested}`
    : `no app.exe found in the default debug/release targets:\n${formatCandidates(appResolution.candidates)}`
  const msg = `GUI E2E SKIP: ${detail}\nBuild the app or set TAURI_APP_PATH to an existing executable.`
  finishUnavailable(msg)
}

const probe = spawnSync(driver, [directNative ? '--version' : '--help'], { stdio: 'ignore', shell: false })
if (probe.error || probe.status !== 0) {
  const label = directNative ? 'TAURI_NATIVE_DRIVER' : 'tauri-driver'
  const detail = driverResolution.explicit
    ? `${label} is not executable: ${driver}`
    : `tauri-driver was not found on PATH or Cargo bin (${formatCandidates(driverResolution.candidates)})`
  const msg = `GUI E2E SKIP: ${detail}\nInstall with cargo install tauri-driver --locked or set TAURI_DRIVER.`
  finishUnavailable(msg)
}

if (!app || !existsSync(app)) {
  // The resolver already checks this. Keep the guard close to the WebDriver
  // setup so a concurrently removed build fails with the same contract.
  const msg = `GUI E2E SKIP: application disappeared before launch: ${app}`
  finishUnavailable(msg)
}

const driverArgs = directNative ? ['--port=4444'] : []
const driverProcess = spawn(driver, driverArgs, { shell: false, stdio: process.env.E2E_DRIVER_LOG === '1' ? 'inherit' : 'ignore' })
const stopDriver = async () => {
  if (driverProcess.exitCode === null) {
    if (process.platform === 'win32') {
      spawnSync('taskkill', ['/PID', String(driverProcess.pid), '/T', '/F'], { stdio: 'ignore' })
    } else {
      driverProcess.kill('SIGTERM')
    }
  }
  await new Promise((resolve) => driverProcess.exitCode !== null ? resolve() : driverProcess.once('exit', resolve))
}
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
try {
  await waitForPort()
} catch (error) {
  await stopDriver()
  const detail = error instanceof Error ? error.message : String(error)
  const nativeHint = process.platform === 'win32' && !directNative
    ? ' On Windows, install a matching msedgedriver.exe and add it to PATH, or set TAURI_NATIVE_DRIVER.'
    : ''
  finishUnavailable(`GUI E2E SKIP: driver failed to start: ${detail}.${nativeHint}`)
}
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
  const verifyWindowVisible = async () => {
    const geometry = await browser.execute(() => ({
      window: { x: window.screenX, y: window.screenY, width: window.outerWidth, height: window.outerHeight },
      workArea: { x: window.screen.availLeft, y: window.screen.availTop, width: window.screen.availWidth, height: window.screen.availHeight },
    }))
    const overlapWidth = Math.max(0, Math.min(geometry.window.x + geometry.window.width, geometry.workArea.x + geometry.workArea.width) - Math.max(geometry.window.x, geometry.workArea.x))
    const overlapHeight = Math.max(0, Math.min(geometry.window.y + geometry.window.height, geometry.workArea.y + geometry.workArea.height) - Math.max(geometry.window.y, geometry.workArea.y))
    if (overlapWidth * overlapHeight <= 0) throw new Error(`window is outside the active monitor work area: ${JSON.stringify(geometry)}`)
    console.log(`PASS window intersects active monitor: ${JSON.stringify(geometry)}`)
  }
  await verifyWindowVisible()
  await verifyViewport(1366, 768)
  await requireElement('[data-testid="sidebar"]')
  await browser.pause(600)
  const startupPrompt = await browser.$('[data-testid="ingest-startup-prompt"]')
  if (await startupPrompt.isExisting()) {
    await (await requireElement('[data-testid="ingest-prompt-cancel"]')).click()
    console.log('PASS ingest startup prompt cancel')
  }
  const contextToggle = await requireElement('[data-testid="context-toggle"]')
  let researchTrail = await browser.$('[data-testid="research-trail-panel"]')
  if (!await researchTrail.isExisting()) {
    await contextToggle.click()
    researchTrail = await browser.$('[data-testid="research-trail-panel"]')
  }
  await researchTrail.waitForExist({ timeout: 5000, timeoutMsg: 'Research trail panel did not open' })
  console.log('PASS [data-testid="research-trail-panel"]')
  const search = await requireElement('[data-testid="global-search"]')
  await browser.keys(['\uE009', 'k', '\uE000'])
  if (!await search.isFocused()) throw new Error('Ctrl+K did not focus global search')
  await search.setValue('curr')
  await (await requireElement('[data-testid="nav-library"]')).click()
  const searchStatus = await requireElement('[data-testid="library-search-status"]')
  await browser.waitUntil(async () => !(await searchStatus.getText()).includes('正在搜索'), { timeout: 15000, timeoutMsg: 'Global search did not complete' })
  const searchNotice = await browser.$('[data-testid="app-notice"]')
  if (await searchNotice.isExisting() && (await searchNotice.getText()).includes('搜索失败')) {
    throw new Error(`Global search failed: ${await searchNotice.getText()}`)
  }
  console.log('PASS global FTS search query')
  const space = await requireElement('[data-testid="space-toggle"]')
  const before = await space.getAttribute('aria-expanded')
  await space.click()
  const after = await space.getAttribute('aria-expanded')
  if (before === after) throw new Error('Workspace tree did not toggle')
  const librarySearch = await requireElement('[data-testid="library-search"]')
  await librarySearch.setValue('无线充电调度')
  const trailAnchor = await browser.$('.trail-anchor')
  await trailAnchor.waitForExist({ timeout: 15000, timeoutMsg: 'Research trail did not follow the library search context' })
  const trailCard = await browser.$('.trail-card-main')
  await trailCard.waitForExist({ timeout: 15000, timeoutMsg: 'Research trail search returned no auditable item' })
  console.log('PASS contextual research trail search')
  await librarySearch.click()
  await browser.keys(['\uE009', 'a', '\uE000', '\uE003'])
  await browser.waitUntil(async () => (await (await browser.$('[data-testid="library-search-status"]')).getText()).includes('已加载'), { timeout: 15000, timeoutMsg: 'Library catalog did not return after clearing search' })
  await requireElement('[data-testid="library-pagination"]')
  const pageStatus = await requireElement('[data-testid="library-page-status"]')
  if (!(await pageStatus.getText()).includes('第 1 /')) throw new Error(`Unexpected first page status: ${await pageStatus.getText()}`)
  await (await requireElement('button[aria-label="下一页"]')).click()
  if (!(await pageStatus.getText()).includes('第 2 /')) throw new Error(`Pagination did not advance: ${await pageStatus.getText()}`)
  console.log('PASS library pagination')
  await (await requireElement('[data-testid="nav-ingest"]')).click()
  await requireElement('[data-testid="literature-ingest"]')
  await requireElement('[data-testid="ingest-tab-manual"]')
  await (await requireElement('[data-testid="ingest-tab-candidates"]')).click()
  await (await requireElement('[data-testid="ingest-tab-automatic"]')).click()
  await requireElement('[data-testid="automation-settings-link"]')
  if (await (await browser.$('.automation-settings')).isExisting()) throw new Error('Automatic settings form still rendered on ingest page')
  console.log('PASS literature ingest navigation and tabs')
  await (await requireElement('[data-testid="nav-books"]')).click()
  await requireElement('[data-testid="books-view"]')
  await requireElement('[data-testid="book-selector"]')
  await requireElement('[data-testid="book-chapter-content"]')
  await (await requireElement('[data-testid="nav-graph"]')).click()
  await requireElement('[data-testid="graph-view"]')
  await requireElement('[data-refresh-version]')
  await (await requireElement('[data-testid="nav-qa"]')).click()
  await requireElement('[data-testid="qa-input"]')
  await (await requireElement('[data-testid="nav-compile"]')).click()
  await requireElement('[data-testid="compile-center"]')
  await (await requireElement('[data-testid="settings"]')).click()
  await requireElement('[data-testid="updater-settings"]')
  await requireElement('[data-testid="literature-automation-settings"]')
  await requireElement('[data-testid="search-api-settings"]')
  await requireElement('[data-testid="provider-arxiv"]')
  await requireElement('[data-testid="provider-openalex"]')
  await requireElement('[data-testid="provider-tavily"]')
  await requireElement('[data-testid="provider-serpapi"]')
  await verifyViewport(1920, 1080)
  console.log('PASS GUI E2E launch/navigation probe')
} finally {
  await browser.deleteSession()
  await stopDriver()
}
