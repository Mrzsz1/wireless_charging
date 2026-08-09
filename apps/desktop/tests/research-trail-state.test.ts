import test from 'node:test'
import assert from 'node:assert/strict'
import type { ResearchTrailItem } from '../src/types.ts'
import { mergePinnedItems, parseResearchTrailPins, toggleResearchTrailPin } from '../src/features/research-trail/researchTrailState.ts'

const item = (id: string): ResearchTrailItem => ({ id, kind: 'wiki', rank: 1, title: id, snippet: '', score: .8, relation: 'wiki_fts', retrievalReason: '', pageId: id, pageType: 'source', sourcePath: '', wikilink: '', bookId: '', chapterId: '', markdownPath: '', pdfPath: '', nodeId: '', sourceLocation: '', graphPath: [] })

test('invalid persisted pin data is discarded', () => {
  assert.deepEqual(parseResearchTrailPins('{bad'), { version: 1, repositories: {} })
})

test('pins are isolated by repository and context', () => {
  let store = parseResearchTrailPins(null)
  store = toggleResearchTrailPin(store, 'repo-a', 'page:a', item('a'))
  assert.equal(store.repositories['repo-a']['page:a'].length, 1)
  assert.equal(store.repositories['repo-a']['page:b'], undefined)
  assert.equal(store.repositories['repo-b'], undefined)
})

test('pinned items lead and do not duplicate ranked items', () => {
  assert.deepEqual(mergePinnedItems([item('b')], [item('a'), item('b')]).map((value) => value.id), ['b', 'a'])
})
