import { Channel, invoke } from '@tauri-apps/api/core'
import type { AnswerStreamEvent, AskRequest, AskResult, Backlink, BookChapter, BookChapterDetail, BookSearchResult, BookSummary, ChatMessagePage, ChatSessionDetail, ChatSessionPage, ChatSessionSummary, CodexSubscriptionStatus, ComparisonMatrix, CompileCapability, CompileRunDetail, CompileRunSummary, CompileStreamEvent, GraphFilters, GraphOverview, IndexStats, LinkResolution, LiteratureCandidate, LiteratureCapability, LiteratureIngestSettings, LunaSettings, ManualImportSession, PageDetail, PageFilters, PageSummary, QaSettings, ResolvedSourceLocation, SemanticDeploymentStatus, SemanticDownloadProgress, SemanticModelSettings, SourceLocator, QuestionContext, RepositoryInfo, RepositoryWatchStatus, ResearchTrailRequest, ResearchTrailResponse, SearchProviderStatus, SearchResult, StartCompileRequest, StartLiteratureRunRequest, StartupPromptState } from '../types'

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

export async function resolveSourceLocator(locator: SourceLocator): Promise<ResolvedSourceLocation> {
  return invoke<ResolvedSourceLocation>('resolve_source_locator', { locator })
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

export async function getQaSettings(): Promise<QaSettings> {
  return invoke<QaSettings>('get_qa_settings')
}

export async function saveQaSettings(settings: QaSettings): Promise<QaSettings> {
  return invoke<QaSettings>('save_qa_settings', { settings })
}

export async function getSemanticModelSettings(): Promise<SemanticModelSettings> {
  return invoke<SemanticModelSettings>('get_semantic_model_settings')
}

export async function chooseSemanticModelCacheDirectory(): Promise<string> {
  return invoke<string>('choose_semantic_model_cache_directory')
}

export async function saveSemanticModelSettings(cacheDir: string): Promise<SemanticModelSettings> {
  return invoke<SemanticModelSettings>('save_semantic_model_settings', { settings: { cacheDir } })
}

export async function checkSemanticModelDeployment(): Promise<SemanticDeploymentStatus> {
  return invoke<SemanticDeploymentStatus>('check_semantic_model_deployment')
}

export async function repairSemanticModelDeployment(onProgress?: (progress: SemanticDownloadProgress) => void): Promise<SemanticDeploymentStatus> {
  const onEvent = new Channel<SemanticDownloadProgress>()
  onEvent.onmessage = (progress) => onProgress?.(progress)
  return invoke<SemanticDeploymentStatus>('repair_semantic_model_deployment', { onEvent })
}

export async function copySemanticModelCacheAndSwitch(targetDir: string): Promise<SemanticModelSettings> {
  return invoke<SemanticModelSettings>('copy_semantic_model_cache_and_switch', { targetDir })
}

export async function openSemanticModelCacheDirectory(): Promise<void> {
  return invoke<void>('open_semantic_model_cache_directory')
}

export async function getCodexSubscriptionStatus(): Promise<CodexSubscriptionStatus> {
  return invoke<CodexSubscriptionStatus>('get_codex_subscription_status')
}

export async function startCodexLogin(): Promise<string> {
  return invoke<string>('start_codex_login')
}

export async function listChatSessions(limit = 100): Promise<ChatSessionSummary[]> {
  return invoke<ChatSessionSummary[]>('list_chat_sessions', { limit })
}

export async function listChatSessionsPage(cursor?: string, query?: string, limit = 40): Promise<ChatSessionPage> {
  return invoke<ChatSessionPage>('list_chat_sessions_page', { cursor, query, limit })
}

export async function getChatSession(sessionId: string): Promise<ChatSessionDetail> {
  return invoke<ChatSessionDetail>('get_chat_session', { sessionId })
}

export async function getChatSessionPage(sessionId: string, before?: string, limit = 60): Promise<ChatMessagePage> {
  return invoke<ChatMessagePage>('get_chat_session_page', { sessionId, before, limit })
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

export async function prepareResearchTrail(request: ResearchTrailRequest): Promise<ResearchTrailResponse> {
  return invoke<ResearchTrailResponse>('prepare_research_trail', { request })
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

export async function pauseCompileRun(runId: string): Promise<void> { return invoke('pause_compile_run', { runId }) }
export async function resumeCompileRun(runId: string): Promise<void> { return invoke('resume_compile_run', { runId }) }

export async function getRepositoryWatchStatus(): Promise<RepositoryWatchStatus> { return invoke('get_repository_watch_status') }
export async function processRepositoryChanges(): Promise<RepositoryWatchStatus> { return invoke('process_repository_changes') }

export async function rollbackCompileRun(runId: string): Promise<string> {
  return invoke<string>('rollback_compile_run', { runId })
}

export async function getLiteratureCapabilities(): Promise<LiteratureCapability[]> {
  return invoke<LiteratureCapability[]>('get_literature_capabilities')
}

export async function getLiteratureSettings(): Promise<LiteratureIngestSettings> {
  return invoke<LiteratureIngestSettings>('get_literature_settings')
}

export async function saveLiteratureSettings(settings: LiteratureIngestSettings): Promise<LiteratureIngestSettings> {
  return invoke<LiteratureIngestSettings>('save_literature_settings', { settings })
}

export async function listSearchProviderStatuses(): Promise<SearchProviderStatus[]> {
  return invoke<SearchProviderStatus[]>('list_search_provider_statuses')
}

export async function saveSearchProviderKey(provider: string, apiKey: string): Promise<SearchProviderStatus> {
  return invoke<SearchProviderStatus>('save_search_provider_key', { provider, apiKey })
}

export async function deleteSearchProviderKey(provider: string): Promise<SearchProviderStatus> {
  return invoke<SearchProviderStatus>('delete_search_provider_key', { provider })
}

export async function testSearchProvider(provider: string): Promise<string> {
  return invoke<string>('test_search_provider', { provider })
}

export async function getIngestStartupPrompt(localDate: string): Promise<StartupPromptState> {
  return invoke<StartupPromptState>('get_ingest_startup_prompt', { localDate })
}

export async function suppressIngestPromptToday(localDate: string): Promise<void> {
  return invoke('suppress_ingest_prompt_today', { localDate })
}

export async function chooseManualPdfs(): Promise<ManualImportSession | null> {
  return invoke<ManualImportSession | null>('choose_manual_pdfs')
}

export async function discardManualImportSession(sessionId: string): Promise<void> {
  return invoke('discard_manual_import_session', { sessionId })
}

export async function listLiteratureCandidates(): Promise<LiteratureCandidate[]> {
  return invoke<LiteratureCandidate[]>('list_literature_candidates')
}

export async function updateCandidateTriage(candidateIds: string[], status: LiteratureCandidate['triageStatus'], note?: string): Promise<number> {
  return invoke<number>('update_candidate_triage', { candidateIds, status, note })
}

export async function startLiteratureRun(request: StartLiteratureRunRequest, onMessage: (event: CompileStreamEvent) => void): Promise<CompileRunSummary> {
  const onEvent = new Channel<CompileStreamEvent>()
  onEvent.onmessage = onMessage
  return invoke<CompileRunSummary>('start_literature_run', { request, onEvent })
}
