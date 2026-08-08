import test from 'node:test'
import assert from 'node:assert/strict'
import { claimCompletion, createCompletionLedger, mergeCompletedMessages } from '../src/features/qa/completionState.ts'
import { chapterLookupId, matchesBookTarget, shortChapterId } from '../src/features/books/bookTarget.ts'
import { nextGraphRefreshVersion, reconcileGraphPath, reconcileGraphSelection } from '../src/features/graph/refreshState.ts'
import { intersectionArea, parsePersistedWindowState, resolveWindowPlacement, type MonitorWorkArea } from '../src/lib/windowPlacement.ts'
import type { AskResult, BookChapter, ChatMessage, GraphOverview } from '../src/types.ts'

const message = (id: string, role: ChatMessage['role']): ChatMessage => ({
  id,
  sessionId: 'session',
  role,
  content: id,
  status: 'completed',
  createdAt: id,
  errorCode: '',
  errorMessage: '',
  provider: 'test',
  model: 'test',
  requestId: 'request-1',
  evidence: [],
})

const result: AskResult = {
  requestId: 'request-1',
  sessionId: 'session',
  userMessage: message('user-1', 'user'),
  assistantMessage: message('assistant-1', 'assistant'),
  evidence: [],
  waterline: { sourceCount: 0, methodCount: 0, synthesisCount: 0, chapterCount: 0, yearMin: '', yearMax: '', lastIngestAt: '', repositoryPath: 'repo-a', capturedAt: '' },
  offline: true,
}

test('completion claim is idempotent and resets when repository changes', () => {
  const ledger = createCompletionLedger('repo-a')
  assert.equal(claimCompletion(ledger, 'repo-a', 'request-1'), true)
  assert.equal(claimCompletion(ledger, 'repo-a', 'request-1'), false)
  assert.equal(claimCompletion(ledger, 'repo-b', 'request-1'), true)
})

test('completed messages replace local placeholders without duplicating history', () => {
  const messages = [message('history', 'assistant'), message('local-user', 'user'), message('local-assistant', 'assistant'), result.userMessage, result.assistantMessage]
  const merged = mergeCompletedMessages(messages, result)
  assert.deepEqual(merged.map((item) => item.id), ['history', 'user-1', 'assistant-1'])
})

const chapter = (id: string, bookId = 'algorithmic-game-theory'): BookChapter => ({
  id,
  bookId,
  chapterNumber: 1,
  title: 'Chapter',
  markdownPath: 'chapter.md',
  pdfPath: 'book.pdf',
  physicalPageStart: 1,
  physicalPageEnd: 2,
  printedPageStart: 1,
  printedPageEnd: 2,
  charCount: 10,
  ingestStatus: 'done',
})

test('book target matches full and short chapter IDs', () => {
  const item = chapter('algorithmic-game-theory:ch01')
  assert.equal(shortChapterId(item.id), 'ch01')
  assert.equal(chapterLookupId(item), 'ch01')
  assert.equal(matchesBookTarget(item, { bookId: item.bookId, chapterId: 'algorithmic-game-theory:ch01' }), true)
  assert.equal(matchesBookTarget(item, { bookId: item.bookId, chapterId: 'ch01' }), true)
  assert.equal(matchesBookTarget(item, { bookId: 'approximation-algorithms', chapterId: 'ch01' }), false)
})

const graph: GraphOverview = {
  nodes: [{ id: 'a', label: 'A', nodeType: 'wiki', sourceFile: '', sourceLocation: '', community: null, origin: 'test' }],
  edges: [],
  nodeCount: 1,
  edgeCount: 0,
  communityCount: 1,
}

test('graph refresh version and stale selection/path reconciliation are deterministic', () => {
  assert.equal(nextGraphRefreshVersion(2, false), 2)
  assert.equal(nextGraphRefreshVersion(2, true), 3)
  assert.equal(reconcileGraphSelection(graph, graph.nodes[0])?.id, 'a')
  assert.equal(reconcileGraphSelection(graph, { ...graph.nodes[0], id: 'missing' }), null)
  assert.deepEqual(reconcileGraphPath(graph, ['a']), ['a'])
  assert.deepEqual(reconcileGraphPath(graph, ['a', 'missing']), [])
})

const primaryMonitor: MonitorWorkArea = { x: 0, y: 0, width: 2048, height: 1104, scaleFactor: 1, primary: true }

test('off-screen legacy window state is centered inside the primary work area', () => {
  const stored = parsePersistedWindowState({ version: 2, x: -2858, y: 381, width: 2150, height: 1208, maximized: false })
  const resolved = resolveWindowPlacement(stored, [primaryMonitor])
  assert.ok(resolved)
  assert.equal(resolved.recovered, true)
  assert.ok(intersectionArea(resolved.state, primaryMonitor) > 0)
  assert.ok(resolved.state.x >= primaryMonitor.x)
  assert.ok(resolved.state.y >= primaryMonitor.y)
  assert.ok(resolved.state.x + resolved.state.width <= primaryMonitor.x + primaryMonitor.width)
  assert.ok(resolved.state.y + resolved.state.height <= primaryMonitor.y + primaryMonitor.height)
})

test('a valid negative coordinate is retained when a left monitor still exists', () => {
  const left: MonitorWorkArea = { x: -2560, y: 0, width: 2560, height: 1400, scaleFactor: 1, primary: false }
  const stored = parsePersistedWindowState({ version: 3, x: -2200, y: 100, width: 1400, height: 900, maximized: true })
  const resolved = resolveWindowPlacement(stored, [primaryMonitor, left])
  assert.ok(resolved)
  assert.equal(resolved.recovered, false)
  assert.equal(resolved.state.x, -2200)
  assert.equal(resolved.state.maximized, true)
})

test('window size and position are fitted after the display resolution shrinks', () => {
  const monitor: MonitorWorkArea = { x: 0, y: 0, width: 1920, height: 1040, scaleFactor: 1, primary: true }
  const stored = parsePersistedWindowState({ version: 3, x: 1800, y: 900, width: 3200, height: 1800, maximized: false })
  const resolved = resolveWindowPlacement(stored, [monitor])
  assert.ok(resolved)
  assert.deepEqual(
    { x: resolved.state.x, y: resolved.state.y, width: resolved.state.width, height: resolved.state.height },
    { x: monitor.x, y: monitor.y, width: monitor.width, height: monitor.height },
  )
})

test('invalid persisted values fall back to a centered DPI-scaled default', () => {
  assert.equal(parsePersistedWindowState({ version: 3, x: Number.NaN, y: 0, width: 1366, height: 768, maximized: false }), null)
  assert.equal(parsePersistedWindowState({ version: 3, x: 0, y: 0, width: 200, height: 100, maximized: false }), null)
  const monitor: MonitorWorkArea = { x: 0, y: 0, width: 3840, height: 2080, scaleFactor: 2, primary: true }
  const resolved = resolveWindowPlacement(null, [monitor])
  assert.ok(resolved)
  assert.equal(resolved.state.width, 2732)
  assert.equal(resolved.state.height, 1536)
  assert.equal(resolved.state.x, 554)
  assert.equal(resolved.state.y, 272)
})
