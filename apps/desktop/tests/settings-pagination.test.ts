import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'

const read = (path: string) => readFileSync(new URL(path, import.meta.url), 'utf8')

test('automation editing lives in SettingsView rather than the automatic run page', () => {
  const settings = read('../src/features/settings/SettingsView.tsx')
  const ingest = read('../src/features/ingest/LiteratureIngestView.tsx')
  assert.match(settings, /data-testid="literature-automation-settings"/)
  assert.match(settings, /data-testid="search-api-settings"/)
  assert.match(settings, /saveLiteratureSettings/)
  assert.doesNotMatch(ingest, /saveLiteratureSettings|className="automation-settings"/)
  assert.match(ingest, /onOpenSettings/)
})

test('search provider cards never render a persisted key value', () => {
  const settings = read('../src/features/settings/SettingsView.tsx')
  const types = read('../src/types.ts')
  assert.match(settings, /已保存值不会回显/)
  assert.match(settings, /type=\{visibleKeys\[provider\.id\] \? 'text' : 'password'\}/)
  const statusType = types.slice(types.indexOf('export type SearchProviderStatus'), types.indexOf('export type StartupPromptState'))
  assert.doesNotMatch(statusType, /apiKey|secret|value/)
})

test('library list renders bounded pagination controls', () => {
  const library = read('../src/features/library/LibraryView.tsx')
  assert.match(library, /data-testid="library-pagination"/)
  assert.match(library, /PAGE_SIZE_OPTIONS/)
  assert.match(library, /pagination\.items\.map/)
})
