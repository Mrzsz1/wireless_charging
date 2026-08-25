import test from 'node:test'
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import { citationSummary, evidencePanelOwnership, linkEvidenceCitations } from '../src/features/qa/qaPresentation.ts'
import type { CitationValidation } from '../src/types.ts'

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

test('natural answers report per-claim grounding rather than appendix presence', () => {
  const validation: CitationValidation = {
    citedIds: ['E1'], unknownIds: [], citationPrecision: 1, hasCitations: true,
    supported: true, groundingStatus: 'supported', zeroEvidence: false,
    claimCount: 0, citedClaimCount: 0, citationCoverage: 0, unsupportedClaims: [],
    graphOnlyClaims: [], syntaxValid: true, coverageValid: true, entailmentChecked: false,
    heuristicVerificationChecked: true,
    modelSupplementClaimCount: 0, modelSupplementClaims: [], appendixIntegrity: true,
    appendixEvidenceIds: ['E1'],
  }
  const summary = citationSummary(validation, 'natural-markdown-v2')
  assert.equal(summary?.label, '逐条证据核验通过')
  assert.match(summary?.detail ?? '', /显式绑定本轮证据/)
})

test('backend-owned short evidence links remain intact during inline citation projection', () => {
  const markdown = '## 参考证据\n\n- [书籍 · Euclidean TSP](evidence:E1)'
  assert.equal(linkEvidenceCitations(markdown), markdown)
})

test('natural answer and Markdown locator contracts are wired across backend and UI', () => {
  const context = readFileSync(new URL('../src-tauri/src/qa/context.rs', import.meta.url), 'utf8')
  const qa = readFileSync(new URL('../src-tauri/src/qa.rs', import.meta.url), 'utf8')
  const app = readFileSync(new URL('../src/App.tsx', import.meta.url), 'utf8')
  const ask = readFileSync(new URL('../src/features/qa/AskView.tsx', import.meta.url), 'utf8')
  assert.match(context, /qa-natural-markdown-v2/)
  assert.match(context, /每条库内事实陈述必须.*\[E#\]/)
  assert.match(qa, /natural_answer::render/)
  assert.match(qa, /LUNAWIKI_RAG_ANSWER_V2/)
  assert.match(app, /readSourceLocator/)
  assert.match(app, /SourceDocumentView/)
  assert.match(app, /const tabId = 'source-current'/)
  assert.match(app, /tab\.kind !== 'source'/)
  assert.match(ask, /item\.locator/)
  assert.match(ask, /逐条证据审计/)
  assert.match(ask, /claimVerifications/)
  assert.doesNotMatch(ask, /正在接收并组织结构化回答/)
})

test('never labels empty or settled evidence as previous retrieval state', () => {
  assert.equal(evidencePanelOwnership('retrieving', 'request-old', 'request-new', 0), 'current')
  assert.equal(evidencePanelOwnership('generating', 'request-old', 'request-new', 4), 'current')
  assert.equal(evidencePanelOwnership('idle', 'request-old', '', 4), 'current')
})
