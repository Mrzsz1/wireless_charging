import assert from 'node:assert/strict'
import test from 'node:test'
import { normalizePage, paginate, visiblePageNumbers } from '../src/features/library/pagination.ts'

const records = Array.from({ length: 66 }, (_, index) => index + 1)

test('defaults can slice a 66-item catalog into seven ten-item pages', () => {
  const first = paginate(records, 1, 10)
  assert.deepEqual(first.items, records.slice(0, 10))
  assert.deepEqual({ page: first.page, pageCount: first.pageCount, start: first.start, end: first.end, total: first.total }, { page: 1, pageCount: 7, start: 1, end: 10, total: 66 })
  const last = paginate(records, 7, 10)
  assert.deepEqual(last.items, records.slice(60))
  assert.deepEqual({ start: last.start, end: last.end }, { start: 61, end: 66 })
})

test('empty and out-of-range pages converge to stable boundaries', () => {
  assert.deepEqual(paginate([], 9, 10), { items: [], page: 1, pageCount: 1, pageSize: 10, start: 0, end: 0, total: 0 })
  assert.equal(normalizePage(99, 12, 10), 2)
  assert.equal(normalizePage(-4, 12, 10), 1)
})

test('page sizes 10, 20 and 50 preserve ordering', () => {
  assert.deepEqual(paginate(records, 2, 20).items, records.slice(20, 40))
  assert.deepEqual(paginate(records, 2, 50).items, records.slice(50))
})

test('visible page buttons stay bounded around the current page', () => {
  assert.deepEqual(visiblePageNumbers(1, 12), [1, 2, 3, 4, 5])
  assert.deepEqual(visiblePageNumbers(6, 12), [4, 5, 6, 7, 8])
  assert.deepEqual(visiblePageNumbers(12, 12), [8, 9, 10, 11, 12])
  assert.deepEqual(visiblePageNumbers(1, 3), [1, 2, 3])
})
