import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'
import { automationCapabilitiesReady, defaultSelectedManualFileIds, filterCandidates, formatBytes, localDateKey } from '../src/features/ingest/ingestState.ts'
import type { LiteratureCandidate, LiteratureCapability, ManualImportSession } from '../src/types.ts'

const candidate = (overrides: Partial<LiteratureCandidate> = {}): LiteratureCandidate => ({
  candidateId: 'candidate-1', title: 'Wireless charging scheduling', authors: ['Alice'], year: 2026,
  abstract: 'A robust optimization model', venue: 'T-ITS', score: 9, doi: '10.1/demo', arxivId: '',
  pdfUrl: 'https://example.test/a.pdf', sourceUrl: '', provider: 'openalex', triageStatus: 'pending',
  localPdf: '', manualNote: '', titleMatches: ['charging'], abstractMatches: ['scheduling'], matchedQueries: ['wireless'],
  manifestPath: 'raw/inbox/run/manifest.json', discoveryRuns: ['run'], duplicateMatches: [],
  qualification: { eligible: true, score: 9, reasons: [], duplicates: [] }, ...overrides,
})

test('localDateKey uses local calendar fields', () => {
  assert.equal(localDateKey(new Date(2026, 7, 9, 23, 30)), '2026-08-09')
})

test('manual defaults exclude invalid and duplicate PDFs', () => {
  const session: ManualImportSession = {
    id: 's', createdAt: '1', files: [
      { id: 'ok', path: '', name: 'ok.pdf', size: 10, mtimeNs: 1, sha256: 'a', valid: true, selected: true, errors: [], duplicateMatches: [] },
      { id: 'duplicate', path: '', name: 'd.pdf', size: 10, mtimeNs: 1, sha256: 'b', valid: true, selected: true, errors: [], duplicateMatches: [{ kind: 'sha256', value: 'b', existingId: 'x', existingPath: 'raw/x.pdf', title: 'x' }] },
      { id: 'invalid', path: '', name: 'bad.pdf', size: 0, mtimeNs: 1, sha256: '', valid: false, selected: false, errors: ['空'], duplicateMatches: [] },
    ],
  }
  assert.deepEqual(defaultSelectedManualFileIds(session), ['ok'])
})

test('candidate filtering combines status, qualification and text', () => {
  const rejected = candidate({ candidateId: 'candidate-2', title: 'Road traffic', triageStatus: 'rejected', qualification: { eligible: false, score: 2, reasons: [], duplicates: [] } })
  assert.deepEqual(filterCandidates([candidate(), rejected], 'Alice', 'pending').map((item) => item.candidateId), ['candidate-1'])
  assert.deepEqual(filterCandidates([candidate(), rejected], '', 'eligible').map((item) => item.candidateId), ['candidate-1'])
})

test('formatBytes returns compact values', () => {
  assert.equal(formatBytes(2 * 1024 * 1024), '2.0 MB')
})

test('automation readiness checks only dependencies required by the selected mode', () => {
  const capabilities: LiteratureCapability[] = [
    { id: 'discovery', available: true, reason: '可用' },
    { id: 'download', available: true, reason: '可用' },
    { id: 'compile', available: false, reason: '缺少 Codex' },
    { id: 'full_ingest', available: false, reason: '依赖不完整' },
  ]
  assert.equal(automationCapabilitiesReady(capabilities, false), true)
  assert.equal(automationCapabilitiesReady(capabilities, true), false)
  assert.equal(automationCapabilitiesReady([], false), false)
})

test('automatic literature UI owns a visible spinner and readable running strip', () => {
  const view = readFileSync(new URL('../src/features/ingest/LiteratureIngestView.tsx', import.meta.url), 'utf8')
  const css = readFileSync(new URL('../src/features/ingest/LiteratureIngestView.css', import.meta.url), 'utf8')
  assert.match(view, /className="ingest-action-spinner" aria-hidden="true"/)
  assert.match(view, /aria-busy=\{busy\}/)
  assert.match(view, /role="status" aria-live="polite"/)
  assert.match(css, /@keyframes ingest-action-spin/)
  assert.match(css, /\.ingest-run-strip pre \{[^}]*font-size: 11px/)
  assert.match(css, /prefers-reduced-motion: reduce/)
})
