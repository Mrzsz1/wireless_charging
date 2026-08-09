import test from 'node:test'
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import { createLatestRequestGuard } from '../src/lib/latestRequest.ts'

const appSource = readFileSync(new URL('../src/App.tsx', import.meta.url), 'utf8')
const stylesSource = readFileSync(new URL('../src/styles.css', import.meta.url), 'utf8')

test('only the newest request can publish after responses arrive out of order', () => {
  const guard = createLatestRequestGuard()
  const first = guard.next()
  const second = guard.next()
  assert.equal(guard.isCurrent(first), false)
  assert.equal(guard.isCurrent(second), true)
})

test('clearing the query invalidates an in-flight request', () => {
  const guard = createLatestRequestGuard()
  const token = guard.next()
  guard.invalidate()
  assert.equal(guard.isCurrent(token), false)
})

test('a stale failure is ignored just like a stale success', () => {
  const guard = createLatestRequestGuard()
  const oldToken = guard.next()
  const currentToken = guard.next()
  assert.equal(guard.isCurrent(oldToken), false)
  assert.equal(guard.isCurrent(currentToken), true)
})

test('global search waits for an explicit form submission', () => {
  assert.match(appSource, /<form className="global-search" role="search" onSubmit=/)
  assert.match(appSource, /data-testid="global-search" value=\{searchDraft\} onChange=\{\(event\) => setSearchDraft\(event\.target\.value\)\}/)
  assert.match(appSource, /type="submit" className="global-search-submit" data-testid="global-search-submit"/)
  assert.match(appSource, /if \(searchBusy \|\| !searchDraft\.trim\(\)\) return/)
})

test('global search can be cleared without publishing stale results', () => {
  assert.match(appSource, /const clearSearch = \(\) => \{\s*searchRequests\.invalidate\(\)/)
  assert.match(appSource, /setSearchDraft\(''\)[\s\S]*setQuery\(''\)[\s\S]*setResults\(\[\]\)/)
  assert.match(appSource, /data-testid="global-search-clear" aria-label="清空搜索" onClick=\{clearSearch\}/)
})

test('toolbar keeps the search and command actions left aligned', () => {
  const toolbarActions = stylesSource.match(/\.toolbar-actions \{[^}]+\}/)?.[0] ?? ''
  assert.match(stylesSource, /\.workspace-toolbar \{[^}]*justify-content: flex-start;/)
  assert.match(toolbarActions, /margin-left: 0;/)
  assert.doesNotMatch(toolbarActions, /margin-left: auto;/)
})
