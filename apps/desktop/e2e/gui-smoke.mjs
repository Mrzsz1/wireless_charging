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
const hostHasCodex = process.platform === 'win32' && spawnSync('where.exe', ['codex'], { stdio: 'ignore', shell: false }).status === 0

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
  await requireElement('.titlebar-app-menu')
  const topSettings = await requireElement('[data-testid="settings"]')
  const topHelp = await requireElement('[data-testid="help"]')
  const menuPlacement = await browser.execute(() => ({
    settingsInSidebar: Boolean(document.querySelector('[data-testid="sidebar"] [data-testid="settings"]')),
    helpInSidebar: Boolean(document.querySelector('[data-testid="sidebar"] [data-testid="help"]')),
    footerExists: Boolean(document.querySelector('.sidebar-footer')),
  }))
  if (menuPlacement.settingsInSidebar || menuPlacement.helpInSidebar || menuPlacement.footerExists) {
    throw new Error(`Settings/help were not fully moved to the titlebar: ${JSON.stringify(menuPlacement)}`)
  }
  console.log('PASS settings and help titlebar placement')
  const toast = await requireElement('[data-testid="app-notice"]')
  const toastLayout = await browser.execute(() => {
    const item = document.querySelector('[data-testid="app-notice"]')
    const main = document.querySelector('.main-workspace')
    if (!(item instanceof HTMLElement) || !(main instanceof HTMLElement)) return null
    const rect = item.getBoundingClientRect()
    const style = getComputedStyle(item)
    return {
      position: style.position,
      top: rect.top,
      rightGap: document.documentElement.clientWidth - rect.right,
      insideMain: main.contains(item),
      role: item.getAttribute('role'),
      live: item.getAttribute('aria-live'),
    }
  })
  if (!toastLayout || toastLayout.position !== 'fixed' || toastLayout.top < 40 || toastLayout.rightGap < 10 || toastLayout.insideMain || toastLayout.role !== 'status' || toastLayout.live !== 'polite') {
    throw new Error(`Immersive toast layout/accessibility contract failed: ${JSON.stringify(toastLayout)}`)
  }
  await browser.execute(() => document.querySelector('[data-testid="app-notice"]')?.dispatchEvent(new MouseEvent('mouseover', { bubbles: true })))
  await browser.pause(3800)
  if (!await toast.isExisting()) throw new Error('Toast disappeared while hovered')
  await browser.execute(() => document.querySelector('[data-testid="app-notice"]')?.dispatchEvent(new MouseEvent('mouseout', { bubbles: true })))
  await toast.waitForExist({ reverse: true, timeout: 5500, timeoutMsg: 'Toast did not fade out after hover ended' })
  console.log('PASS fixed toast placement, hover pause and automatic fade')
  await browser.pause(600)
  const startupPrompt = await browser.$('[data-testid="ingest-startup-prompt"]')
  if (await startupPrompt.isExisting()) {
    await (await requireElement('[data-testid="ingest-prompt-cancel"]')).click()
    console.log('PASS ingest startup prompt cancel')
  }
  if (await (await browser.$('.workspace-toolbar')).isExisting()) throw new Error('Redundant workspace toolbar is still visible')
  if (await (await browser.$('[data-testid="context-toggle"]')).isExisting()) throw new Error('Redundant context toggle is still visible')
  const trailToggle = await requireElement('[data-testid="trail-toggle"]')
  const titlebarToggleLayout = await browser.execute(() => {
    const toggle = document.querySelector('[data-testid="trail-toggle"]')
    const actions = document.querySelector('.window-actions')
    const drag = document.querySelector('.titlebar-drag-region')
    if (![toggle, actions, drag].every((item) => item instanceof HTMLElement)) return null
    const toggleRect = toggle.getBoundingClientRect()
    const actionsRect = actions.getBoundingClientRect()
    const dragRect = drag.getBoundingClientRect()
    return { toggleLeft: toggleRect.left, toggleRight: toggleRect.right, actionsLeft: actionsRect.left, dragRight: dragRect.right }
  })
  if (!titlebarToggleLayout || titlebarToggleLayout.dragRight > titlebarToggleLayout.toggleLeft + 2 || titlebarToggleLayout.toggleRight > titlebarToggleLayout.actionsLeft + 2) {
    throw new Error(`Research trail toggle is not between titlebar drag region and window controls: ${JSON.stringify(titlebarToggleLayout)}`)
  }
  let researchTrail = await browser.$('[data-testid="research-trail-panel"]')
  if (!await researchTrail.isExisting()) {
    await trailToggle.click()
    researchTrail = await browser.$('[data-testid="research-trail-panel"]')
  }
  await researchTrail.waitForExist({ timeout: 5000, timeoutMsg: 'Research trail panel did not open' })
  console.log('PASS [data-testid="research-trail-panel"]')
  const trailRefresh = await requireElement('[data-testid="trail-refresh"]')
  if (!(await trailRefresh.getText()).includes('刷新')) throw new Error('Research trail refresh control is missing its text label')
  await trailToggle.click()
  await researchTrail.waitForExist({ reverse: true, timeout: 5000, timeoutMsg: 'Research trail panel did not collapse' })
  const collapsedLayout = await browser.execute(() => {
    const body = document.querySelector('.app-body')
    const main = document.querySelector('.main-workspace')
    const toggle = document.querySelector('[data-testid="trail-toggle"]')
    if (![body, main, toggle].every((item) => item instanceof HTMLElement)) return null
    const bodyRect = body.getBoundingClientRect()
    const mainRect = main.getBoundingClientRect()
    return {
      rightDelta: Math.abs(bodyRect.right - mainRect.right),
      pressed: toggle.getAttribute('aria-pressed'),
      legacyRail: Boolean(document.querySelector('[data-testid="research-trail-rail"], [data-testid="trail-reopen"]')),
    }
  })
  if (!collapsedLayout || collapsedLayout.rightDelta > 2 || collapsedLayout.pressed !== 'false' || collapsedLayout.legacyRail) {
    throw new Error(`Collapsed research trail still reserves a rail or has stale state: ${JSON.stringify(collapsedLayout)}`)
  }
  await trailToggle.click()
  researchTrail = await browser.$('[data-testid="research-trail-panel"]')
  await researchTrail.waitForExist({ timeout: 5000, timeoutMsg: 'Research trail panel did not reopen' })
  console.log('PASS titlebar research trail placement, refresh, collapse and reopen')
  const sidebar = await requireElement('[data-testid="sidebar"]')

  if (await (await browser.$('[data-testid="work-tabs"]')).isExisting()) throw new Error('Removed work tab bar is still visible')
  if (await (await browser.$('.eyebrow')).isExisting()) throw new Error('English eyebrow heading is still visible')
  console.log('PASS single-view navigation and Chinese-only headings')

  await browser.setWindowSize(1600, 900)
  const workspacePane = await requireElement('[data-testid="sidebar-workspace-pane"]')
  const workspaceResizer = await requireElement('[data-testid="sidebar-workspace-resizer"]')
  if (await workspaceResizer.getAttribute('role') !== 'separator' || await workspaceResizer.getAttribute('aria-orientation') !== 'horizontal') {
    throw new Error('Workspace resizer does not expose the separator contract')
  }
  const paneHeightBeforeDrag = (await workspacePane.getSize()).height
  await browser.execute(() => {
    const handle = document.querySelector('[data-testid="sidebar-workspace-resizer"]')
    if (!(handle instanceof HTMLElement)) throw new Error('Workspace resizer missing')
    const y = handle.getBoundingClientRect().top + 5
    const eventOptions = { bubbles: true, pointerId: 41, pointerType: 'mouse', button: 0, buttons: 1, clientX: handle.getBoundingClientRect().left + 30 }
    handle.dispatchEvent(new PointerEvent('pointerdown', { ...eventOptions, clientY: y }))
    handle.dispatchEvent(new PointerEvent('pointermove', { ...eventOptions, clientY: y - 40 }))
    handle.dispatchEvent(new PointerEvent('pointerup', { ...eventOptions, buttons: 0, clientY: y - 40 }))
  })
  await browser.waitUntil(async () => (await workspacePane.getSize()).height < paneHeightBeforeDrag - 10, {
    timeout: 5000,
    timeoutMsg: 'Dragging upward did not shrink the workspace pane',
  })
  const paneHeightAfterDrag = (await workspacePane.getSize()).height
  await workspaceResizer.click()
  await browser.execute(() => {
    const handle = document.querySelector('[data-testid="sidebar-workspace-resizer"]')
    if (!(handle instanceof HTMLElement)) throw new Error('Workspace resizer missing')
    handle.dispatchEvent(new KeyboardEvent('keydown', { key: 'ArrowDown', bubbles: true }))
  })
  await browser.waitUntil(async () => (await workspacePane.getSize()).height > paneHeightAfterDrag, {
    timeout: 5000,
    timeoutMsg: 'ArrowDown did not expand the focused workspace pane',
  })
  const persistedWorkspaceHeight = await browser.execute(() => localStorage.getItem('desktop.sidebar-workspace-height.v1'))
  if (!persistedWorkspaceHeight) throw new Error('Workspace pane height was not persisted')
  console.log('PASS resizable and keyboard-accessible workspace pane')

  if (!(await sidebar.getAttribute('class')).includes('collapsed')) {
    await (await requireElement('button[aria-label="展开或收起侧边栏"]')).click()
    await browser.waitUntil(async () => (await sidebar.getAttribute('class')).includes('collapsed'), { timeout: 5000, timeoutMsg: 'Sidebar did not collapse' })
  }
  if (await (await browser.$('[data-testid="sidebar-workspace-resizer"]')).isExisting()) throw new Error('Workspace resizer stayed visible after sidebar collapse')
  await requireElement('[data-testid="sidebar-search-trigger"]')
  await requireElement('[data-testid="sidebar-new-qa"]')
  await browser.keys(['\uE009', 'k', '\uE000'])
  await browser.waitUntil(async () => (await sidebar.getAttribute('class')).includes('expanded'), { timeout: 5000, timeoutMsg: 'Ctrl+K did not expand the sidebar' })
  await requireElement('[data-testid="sidebar-workspace-resizer"]')
  await verifyViewport(1366, 768)
  const search = await requireElement('[data-testid="global-search"]')
  if (!await search.isFocused()) throw new Error('Ctrl+K did not focus global search')
  await search.setValue('curr')
  const searchSubmit = await requireElement('[data-testid="global-search-submit"]')
  if (!await searchSubmit.isEnabled()) throw new Error('Global search submit button stayed disabled with a query')
  await searchSubmit.click()
  const searchStatus = await requireElement('[data-testid="library-search-status"]')
  await browser.waitUntil(async () => !(await searchStatus.getText()).includes('正在搜索'), { timeout: 15000, timeoutMsg: 'Global search did not complete' })
  const searchNotice = await browser.$('[data-testid="app-notice"]')
  if (await searchNotice.isExisting() && (await searchNotice.getText()).includes('搜索失败')) {
    throw new Error(`Global search failed: ${await searchNotice.getText()}`)
  }
  await search.setValue('game')
  await browser.keys('\uE007')
  await browser.waitUntil(async () => !(await searchStatus.getText()).includes('正在搜索'), { timeout: 15000, timeoutMsg: 'Enter did not submit global search' })
  console.log('PASS global FTS search button and Enter submission')
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
  const qaLayout = await browser.execute(() => {
    const body = document.querySelector('.app-body')
    const main = document.querySelector('.main-workspace.qa-active')
    const qa = document.querySelector('.qa-view')
    const evidence = document.querySelector('.qa-evidence-panel')
    const toggle = document.querySelector('[data-testid="trail-toggle"]')
    if (![body, main, qa, evidence, toggle].every((item) => item instanceof HTMLElement)) return null
    const rect = (item) => {
      const value = item.getBoundingClientRect()
      return { left: value.left, right: value.right, width: value.width }
    }
    return { body: rect(body), main: rect(main), qa: rect(qa), evidence: rect(evidence), pressed: toggle.getAttribute('aria-pressed'), legacyRail: Boolean(document.querySelector('[data-testid="research-trail-rail"]')) }
  })
  if (!qaLayout || Math.abs(qaLayout.main.right - qaLayout.body.right) > 2 || Math.abs(qaLayout.qa.right - qaLayout.body.right) > 2 || qaLayout.evidence.right > qaLayout.qa.right + 2 || qaLayout.pressed !== 'false' || qaLayout.legacyRail) {
    throw new Error(`QA layout is clipped or the collapsed trail still reserves space: ${JSON.stringify(qaLayout)}`)
  }
  await (await requireElement('[data-testid="trail-toggle"]')).click()
  await requireElement('[data-testid="research-trail-panel"]')
  const openQaLayout = await browser.execute(() => ({
    contextVisible: document.querySelector('.main-workspace.qa-active')?.classList.contains('context-visible') ?? false,
    evidenceDisplay: getComputedStyle(document.querySelector('.qa-evidence-panel')).display,
  }))
  if (!openQaLayout.contextVisible || openQaLayout.evidenceDisplay !== 'none') throw new Error(`QA did not reserve space for the open research trail: ${JSON.stringify(openQaLayout)}`)
  await (await requireElement('[data-testid="trail-toggle"]')).click()
  await (await browser.$('[data-testid="research-trail-panel"]')).waitForExist({ reverse: true, timeout: 5000 })
  console.log('PASS QA width, automatic trail collapse and titlebar-only control')
  await (await requireElement('[data-testid="qa-open-settings"]')).click()
  const qaSettings = await requireElement('[data-testid="qa-engine-settings"]')
  await browser.waitUntil(async () => (await qaSettings.getAttribute('data-loaded')) === 'true', { timeout: 15000, timeoutMsg: 'QA settings did not finish loading' })
  await (await requireElement('[data-testid="qa-provider-tab-codex"]')).click()
  await requireElement('[data-testid="qa-provider-codex"]')
  if (hostHasCodex) {
    const statusCard = await requireElement('.codex-status-card')
    await browser.waitUntil(async () => !(await statusCard.getText()).includes('正在读取本机 Codex 状态'), { timeout: 15000, timeoutMsg: 'Codex status did not finish loading' })
    const statusText = await statusCard.getText()
    if (statusText.includes('Codex CLI 未安装') || statusText.includes('未检测到版本')) {
      throw new Error(`Installed Codex CLI was not detected by the GUI process: ${statusText}`)
    }
    console.log('PASS installed Codex CLI detection from GUI environment')
  }
  await (await requireElement('[data-testid="qa-provider-tab-api"]')).click()
  await requireElement('[data-testid="qa-provider-api"]')
  await (await requireElement('[data-testid="qa-provider-tab-offline"]')).click()
  await requireElement('[data-testid="qa-provider-offline"]')
  console.log('PASS QA settings are centralized with three providers')
  await (await requireElement('[data-testid="nav-compile"]')).click()
  await requireElement('[data-testid="compile-center"]')
  await topSettings.click()
  await requireElement('[data-testid="updater-settings"]')
  await requireElement('[data-testid="literature-automation-settings"]')
  await requireElement('[data-testid="search-api-settings"]')
  await requireElement('[data-testid="qa-engine-settings"]')
  await requireElement('[data-testid="provider-arxiv"]')
  await requireElement('[data-testid="provider-openalex"]')
  await requireElement('[data-testid="provider-tavily"]')
  await requireElement('[data-testid="provider-serpapi"]')
  if (await topSettings.getAttribute('aria-current') !== 'page') throw new Error('Titlebar settings entry has no active-page state')
  const settingsHelp = await requireElement('[data-testid="settings-page-help"]')
  await browser.execute((selector) => {
    const trigger = document.querySelector(selector)
    if (!(trigger instanceof HTMLElement)) throw new Error('Settings help trigger missing')
    trigger.dispatchEvent(new PointerEvent('pointerover', { bubbles: true, pointerType: 'mouse' }))
  }, '[data-testid="settings-page-help"]')
  await browser.pause(450)
  if (await (await browser.$('[role="tooltip"]')).isExisting()) throw new Error('Settings help tooltip opened before the one-second delay')
  await browser.pause(700)
  const helpTooltip = await requireElement('[role="tooltip"]')
  if (!(await helpTooltip.getText()).includes('集中管理知识库')) throw new Error('Settings help tooltip did not preserve the explanatory copy')
  await browser.execute((selector) => {
    const trigger = document.querySelector(selector)
    if (!(trigger instanceof HTMLElement)) return
    trigger.dispatchEvent(new PointerEvent('pointerout', { bubbles: true, pointerType: 'mouse' }))
  }, '[data-testid="settings-page-help"]')
  await browser.waitUntil(async () => !await (await browser.$('[role="tooltip"]')).isExisting(), { timeout: 3000, timeoutMsg: 'Settings help tooltip did not close on pointer leave' })
  console.log('PASS delayed settings help tooltip')
  await topHelp.click()
  const helpHeading = await requireElement('.placeholder-view h1')
  if ((await helpHeading.getText()).trim() !== '帮助') throw new Error('Titlebar help entry did not open the help view')
  if (await topHelp.getAttribute('aria-current') !== 'page') throw new Error('Titlebar help entry has no active-page state')
  console.log('PASS titlebar help navigation')
  await verifyViewport(1920, 1080)
  console.log('PASS GUI E2E launch/navigation probe')
} finally {
  await browser.deleteSession()
  await stopDriver()
}
