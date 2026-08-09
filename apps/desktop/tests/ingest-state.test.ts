import assert from 'node:assert/strict'
import test from 'node:test'
import { defaultSelectedManualFileIds, filterCandidates, formatBytes, localDateKey } from '../src/features/ingest/ingestState.ts'
import type { LiteratureCandidate, ManualImportSession } from '../src/types.ts'

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
