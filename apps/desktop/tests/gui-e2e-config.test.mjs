import assert from 'node:assert/strict'
import { mkdtempSync, mkdirSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { test } from 'node:test'
import { appCandidates, resolveAppPath, resolveDriver } from '../e2e/gui-config.mjs'

function desktopFixture() {
  const root = mkdtempSync(join(tmpdir(), 'wireless-gui-e2e-'))
  mkdirSync(join(root, 'src-tauri', 'target', 'debug'), { recursive: true })
  mkdirSync(join(root, 'src-tauri', 'target', 'release'), { recursive: true })
  return root
}

test('auto-discovers the release app before a debug app from the desktop root', () => {
  const desktopRoot = desktopFixture()
  const debugApp = join(desktopRoot, 'src-tauri', 'target', 'debug', 'app.exe')
  const releaseApp = join(desktopRoot, 'src-tauri', 'target', 'release', 'app.exe')
  writeFileSync(debugApp, 'fixture')
  writeFileSync(releaseApp, 'fixture')

  const result = resolveAppPath({ env: {}, cwd: desktopRoot, desktopRoot, platform: 'win32' })

  assert.equal(result.path, releaseApp)
  assert.equal(result.source, 'auto')
  assert.equal(result.explicit, false)
})

test('falls back to release when debug is absent', () => {
  const desktopRoot = desktopFixture()
  const releaseApp = join(desktopRoot, 'src-tauri', 'target', 'release', 'app.exe')
  writeFileSync(releaseApp, 'fixture')

  const result = resolveAppPath({ env: {}, cwd: desktopRoot, desktopRoot, platform: 'win32' })

  assert.equal(result.path, releaseApp)
  assert.equal(appCandidates({ cwd: desktopRoot, desktopRoot, platform: 'win32' })[0], releaseApp)
})

test('does not silently fall back when TAURI_APP_PATH is invalid', () => {
  const desktopRoot = desktopFixture()
  const debugApp = join(desktopRoot, 'src-tauri', 'target', 'debug', 'app.exe')
  writeFileSync(debugApp, 'fixture')
  const requested = join(desktopRoot, 'missing.exe')

  const result = resolveAppPath({ env: { TAURI_APP_PATH: requested }, cwd: desktopRoot, desktopRoot, platform: 'win32' })

  assert.equal(result.path, null)
  assert.equal(result.explicit, true)
  assert.equal(result.requested, requested)
})

test('prefers an explicit driver command', () => {
  const result = resolveDriver({ env: {}, explicit: 'C:\\tools\\tauri-driver.exe', platform: 'win32' })

  assert.equal(result.executable, 'C:\\tools\\tauri-driver.exe')
  assert.equal(result.explicit, true)
})

test('discovers tauri-driver in CARGO_HOME', () => {
  const cargoHome = mkdtempSync(join(tmpdir(), 'wireless-cargo-'))
  const driver = join(cargoHome, 'bin', 'tauri-driver.exe')
  mkdirSync(join(cargoHome, 'bin'), { recursive: true })
  writeFileSync(driver, 'fixture')

  const result = resolveDriver({ env: { CARGO_HOME: cargoHome }, platform: 'win32', home: cargoHome })

  assert.equal(result.executable, driver)
  assert.equal(result.source, 'CARGO_HOME')
})
