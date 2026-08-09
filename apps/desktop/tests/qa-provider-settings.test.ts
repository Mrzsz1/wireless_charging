import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'

const read = (path: string) => readFileSync(new URL(path, import.meta.url), 'utf8')

test('all answer engine editing lives in SettingsView', () => {
  const settings = read('../src/features/settings/SettingsView.tsx')
  const ask = read('../src/features/qa/AskView.tsx')
  assert.match(settings, /data-testid="qa-engine-settings"/)
  assert.match(settings, /Codex 订阅/)
  assert.match(settings, /兼容 API/)
  assert.match(settings, /仅离线证据/)
  assert.match(settings, /getCodexSubscriptionStatus/)
  assert.match(ask, /data-testid="qa-open-settings"/)
  assert.doesNotMatch(ask, /Luna 设置|qa-settings-dialog|saveLunaSettings|settingsDraft/)
})

test('Codex status DTO and settings expose no authentication secret', () => {
  const types = read('../src/types.ts')
  const start = types.indexOf('export type CodexSubscriptionStatus')
  const end = types.indexOf('export type WaterlineSnapshot')
  const status = types.slice(start, end)
  assert.match(status, /authenticated: boolean/)
  assert.match(status, /ready: boolean/)
  assert.doesNotMatch(status, /token|cookie|apiKey|credentialPath/)
})

test('App routes the AskView settings action to the global settings section', () => {
  const app = read('../src/App.tsx')
  assert.match(app, /onOpenSettings=\{\(\) => openSettings\('qa-engine-settings'\)\}/)
  assert.match(app, /focusSection=\{settingsFocusSection\}/)
})
