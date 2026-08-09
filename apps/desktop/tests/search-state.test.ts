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
  assert.match(appSource, /<form className="global-search sidebar-global-search" role="search" onSubmit=/)
  assert.match(appSource, /data-testid="global-search"[^>]+value=\{searchDraft\} onChange=\{\(event\) => setSearchDraft\(event\.target\.value\)\}/)
  assert.match(appSource, /type="submit" className="global-search-submit" data-testid="global-search-submit"/)
  assert.match(appSource, /if \(searchBusy \|\| !searchDraft\.trim\(\)\) return/)
})

test('global search can be cleared without publishing stale results', () => {
  assert.match(appSource, /const clearSearch = \(\) => \{\s*searchRequests\.invalidate\(\)/)
  assert.match(appSource, /setSearchDraft\(''\)[\s\S]*setQuery\(''\)[\s\S]*setResults\(\[\]\)/)
  assert.match(appSource, /data-testid="global-search-clear" aria-label="清空搜索" onClick=\{clearSearch\}/)
})

test('search and new-question commands live in the sidebar without the redundant workspace toolbar', () => {
  const sidebarStart = appSource.indexOf('<aside className={`app-sidebar')
  const sidebarEnd = appSource.indexOf('</aside>', sidebarStart)
  const searchForm = appSource.indexOf('data-testid="global-search"')
  const newQuestion = appSource.indexOf('data-testid="sidebar-new-qa"')
  assert.ok(sidebarStart >= 0 && sidebarEnd > sidebarStart)
  assert.ok(searchForm > sidebarStart && searchForm < sidebarEnd)
  assert.ok(newQuestion > sidebarStart && newQuestion < sidebarEnd)
  assert.doesNotMatch(appSource, /workspace-toolbar/)
  assert.doesNotMatch(appSource, /data-testid="context-toggle"/)
  assert.doesNotMatch(appSource, /title="刷新知识库快照"/)
  assert.match(stylesSource, /\.sidebar-brand-search \{[^}]*margin-left: auto;/)
  assert.doesNotMatch(stylesSource, /\.workspace-toolbar \{/)
})

test('Ctrl+K expands the sidebar before focusing the relocated search input', () => {
  assert.match(appSource, /const focusGlobalSearch = useCallback\(\(\) => \{\s*setNavCollapsed\(false\)\s*setSidebarSearchOpen\(true\)/)
  assert.match(appSource, /event\.preventDefault\(\)\s*focusGlobalSearch\(\)/)
})
