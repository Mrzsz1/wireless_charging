import { Channel, invoke } from '@tauri-apps/api/core'
import type { AnswerStreamEvent, AskRequest, AskResult, Backlink, BookChapter, BookChapterDetail, BookSearchResult, BookSummary, ChatSessionDetail, ChatSessionSummary, ComparisonMatrix, CompileCapability, CompileRunDetail, CompileRunSummary, CompileStreamEvent, GraphFilters, GraphOverview, IndexStats, LinkResolution, LunaSettings, PageDetail, PageFilters, PageSummary, QuestionContext, RepositoryInfo, SearchResult, StartCompileRequest } from '../types'

type TauriWindow = Window & { __TAURI_INTERNALS__?: unknown }

export const isDesktopRuntime = () => Boolean((window as TauriWindow).__TAURI_INTERNALS__)

export async function chooseRepository(): Promise<RepositoryInfo> {
  return invoke<RepositoryInfo>('choose_repository')
}

export async function openRepository(path: string): Promise<RepositoryInfo> {
  return invoke<RepositoryInfo>('open_repository', { path })
}

export async function rebuildIndex(): Promise<IndexStats> {
  return invoke<IndexStats>('rebuild_index')
}

export async function repositoryInfo(): Promise<RepositoryInfo> {
  return invoke<RepositoryInfo>('repository_info')
}

export async function searchPages(query: string, limit = 30): Promise<SearchResult[]> {
  return invoke<SearchResult[]>('search_pages', { query, limit })
}

export async function listPages(filters: PageFilters = {}): Promise<PageSummary[]> {
  return invoke<PageSummary[]>('list_pages', { filters })
}

export async function getPage(pageId: string): Promise<PageDetail> {
  return invoke<PageDetail>('get_page', { pageId })
}

export async function resolveWikilink(target: string): Promise<LinkResolution> {
  return invoke<LinkResolution>('resolve_wikilink', { target })
}

export async function getBacklinks(pageId: string): Promise<Backlink[]> {
  return invoke<Backlink[]>('get_backlinks', { pageId })
}

export async function openLocalPath(path: string, reveal = false): Promise<string> {
  return invoke<string>('open_local_path', { path, reveal })
}

export async function listCoreBooks(): Promise<BookSummary[]> {
  return invoke<BookSummary[]>('list_core_books')
}

export async function listBookChapters(bookId: string): Promise<BookChapter[]> {
  return invoke<BookChapter[]>('list_book_chapters', { bookId })
}

export async function getBookChapter(bookId: string, chapterId: string): Promise<BookChapterDetail> {
  return invoke<BookChapterDetail>('get_book_chapter', { bookId, chapterId })
}

export async function searchBookChapters(query: string, bookId?: string, limit = 20): Promise<BookSearchResult[]> {
  return invoke<BookSearchResult[]>('search_book_chapters', { query, bookId, limit })
}

export async function graphOverview(filters: GraphFilters = {}): Promise<GraphOverview> {
  return invoke<GraphOverview>('graph_overview', { filters })
}

export async function graphNeighbors(nodeId: string, depth = 1, limit = 120): Promise<GraphOverview> {
  return invoke<GraphOverview>('graph_neighbors', { nodeId, depth, limit })
}

export async function graphPath(sourceId: string, targetId: string, maxDepth = 6): Promise<string[]> {
  return invoke<string[]>('graph_path', { sourceId, targetId, maxDepth })
}

export async function buildComparison(pageIds: string[]): Promise<ComparisonMatrix> {
  return invoke<ComparisonMatrix>('build_comparison', { pageIds })
}

export async function getLunaSettings(): Promise<LunaSettings> {
  return invoke<LunaSettings>('get_luna_settings')
}

export async function saveLunaSettings(settings: LunaSettings): Promise<LunaSettings> {
  return invoke<LunaSettings>('save_luna_settings', { settings })
}

export async function listChatSessions(limit = 100): Promise<ChatSessionSummary[]> {
  return invoke<ChatSessionSummary[]>('list_chat_sessions', { limit })
}

export async function getChatSession(sessionId: string): Promise<ChatSessionDetail> {
  return invoke<ChatSessionDetail>('get_chat_session', { sessionId })
}

export async function createChatSession(title?: string): Promise<ChatSessionSummary> {
  return invoke<ChatSessionSummary>('create_chat_session', { title })
}

export async function renameChatSession(sessionId: string, title: string): Promise<void> {
  return invoke('rename_chat_session', { sessionId, title })
}

export async function deleteChatSession(sessionId: string): Promise<void> {
  return invoke('delete_chat_session', { sessionId })
}

export async function prepareQuestion(question: string, limit = 14): Promise<QuestionContext> {
  return invoke<QuestionContext>('prepare_question', { question, limit })
}

export async function askLuna(request: AskRequest, onMessage: (event: AnswerStreamEvent) => void): Promise<AskResult> {
  const onEvent = new Channel<AnswerStreamEvent>()
  onEvent.onmessage = onMessage
  return invoke<AskResult>('ask_luna', { request, onEvent })
}

export async function cancelAnswer(requestId: string): Promise<void> {
  return invoke('cancel_answer', { requestId })
}

export async function getCompileCapabilities(): Promise<CompileCapability[]> {
  return invoke<CompileCapability[]>('get_compile_capabilities')
}

export async function listCompileRuns(limit = 100): Promise<CompileRunSummary[]> {
  return invoke<CompileRunSummary[]>('list_compile_runs', { limit })
}

export async function getCompileRun(runId: string): Promise<CompileRunDetail> {
  return invoke<CompileRunDetail>('get_compile_run', { runId })
}

export async function startCompileRun(request: StartCompileRequest, onMessage: (event: CompileStreamEvent) => void): Promise<CompileRunSummary> {
  const onEvent = new Channel<CompileStreamEvent>()
  onEvent.onmessage = onMessage
  return invoke<CompileRunSummary>('start_compile_run', { request, onEvent })
}

export async function retryCompileRun(runId: string, onMessage: (event: CompileStreamEvent) => void): Promise<CompileRunSummary> {
  const onEvent = new Channel<CompileStreamEvent>()
  onEvent.onmessage = onMessage
  return invoke<CompileRunSummary>('retry_compile_run', { runId, onEvent })
}

export async function cancelCompileRun(runId: string): Promise<void> {
  return invoke('cancel_compile_run', { runId })
}

export async function rollbackCompileRun(runId: string): Promise<string> {
  return invoke<string>('rollback_compile_run', { runId })
}
