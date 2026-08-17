import test from 'node:test'
import assert from 'node:assert/strict'
import { evidencePanelOwnership } from '../src/features/qa/qaPresentation.ts'

test('keeps previous evidence visibly isolated until current retrieval completes', () => {
  assert.equal(
    evidencePanelOwnership('retrieving', 'request-old', 'request-new', 4),
    'previous-during-retrieval',
  )
  assert.equal(
    evidencePanelOwnership('retrieving', 'request-new', 'request-new', 4),
    'current',
  )
})

test('never labels empty or settled evidence as previous retrieval state', () => {
  assert.equal(evidencePanelOwnership('retrieving', 'request-old', 'request-new', 0), 'current')
  assert.equal(evidencePanelOwnership('generating', 'request-old', 'request-new', 4), 'current')
  assert.equal(evidencePanelOwnership('idle', 'request-old', '', 4), 'current')
})
