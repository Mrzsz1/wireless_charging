import assert from 'node:assert/strict'
import test from 'node:test'
import { readFileSync, readdirSync } from 'node:fs'
import { resolve } from 'node:path'
import { filterLibraryItems, LIBRARY_CATEGORIES, libraryCategoryCounts, libraryTypeLabel } from '../src/features/library/categories.ts'

const repositoryRoot = resolve(import.meta.dirname, '../../..')
const libraryViewSource = readFileSync(resolve(repositoryRoot, 'apps/desktop/src/features/library/LibraryView.tsx'), 'utf8')

function repositoryPageTypes(): string[] {
  const wikiRoot = resolve(repositoryRoot, 'wiki')
  return readdirSync(wikiRoot, { recursive: true, withFileTypes: true })
    .filter((entry) => entry.isFile() && entry.name.endsWith('.md'))
    .map((entry) => readFileSync(resolve(entry.parentPath, entry.name), 'utf8').match(/^type:\s*['"]?([^'"\r\n]+)/m)?.[1].trim() ?? '')
    .filter(Boolean)
}

test('category definitions cover every governed Wiki page type with Chinese labels', () => {
  assert.deepEqual(LIBRARY_CATEGORIES.map((item) => item.label), ['全部', '论文文献', '方法', '综述', '概念', '系统模型', '优化目标', '数据与实验', '研究问题', '知识地图'])
  assert.equal(libraryTypeLabel('map'), '知识地图')
  assert.equal(libraryTypeLabel('unknown-future-type'), '其他页面')
})

test('repository baseline is classified into the current 75-page waterline', () => {
  const items = repositoryPageTypes().map((pageType) => ({ pageType }))
  assert.deepEqual(libraryCategoryCounts(items), {
    all: 75,
    source: 23,
    method: 20,
    synthesis: 7,
    concept: 7,
    'system-model': 4,
    objective: 4,
    'dataset-or-sim': 1,
    problem: 1,
    map: 8,
  })
})

test('filtering keeps order, isolates map pages and preserves unknown types in all', () => {
  const items = [{ id: 'map-1', pageType: 'map' }, { id: 'source-1', pageType: 'source' }, { id: 'future-1', pageType: 'future' }, { id: 'map-2', pageType: 'map' }]
  assert.deepEqual(filterLibraryItems(items, 'map').map((item) => item.id), ['map-1', 'map-2'])
  assert.deepEqual(filterLibraryItems(items, 'all').map((item) => item.id), ['map-1', 'source-1', 'future-1', 'map-2'])
})

test('LibraryView exposes accessible category controls and forces the method-only view', () => {
  assert.match(libraryViewSource, /role="group" aria-label="内容分类" data-testid="library-categories"/)
  assert.match(libraryViewSource, /data-testid=\{`library-category-\$\{item\.id\}`\} aria-pressed=\{category === item\.id\}/)
  assert.match(libraryViewSource, /pageType === 'method' \? 'method' : category/)
  assert.match(libraryViewSource, /data-page-type=\{result\.pageType\}/)
})
