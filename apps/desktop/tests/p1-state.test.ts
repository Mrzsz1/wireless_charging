import test from 'node:test'
import assert from 'node:assert/strict'
import { claimCompletion, createCompletionLedger, mergeCompletedMessages } from '../src/features/qa/completionState.ts'
import { chapterLookupId, matchesBookTarget, shortChapterId } from '../src/features/books/bookTarget.ts'
import { nextGraphRefreshVersion, reconcileGraphPath, reconcileGraphSelection } from '../src/features/graph/refreshState.ts'
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
