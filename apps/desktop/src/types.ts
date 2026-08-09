export type RepositoryInfo = {
  path: string
  pageCount: number
  indexed: boolean
}

export type IndexStats = {
  path: string
  pageCount: number
  sourceCount: number
  methodCount: number
  synthesisCount: number
  chapterCount: number
}

export type CompileCapability = {
  taskKind: 'lint' | 'graphify_update' | 'discover' | 'parse' | 'compile_a' | 'full_pipeline' | 'literature_prepare' | 'literature_manual_ingest' | 'literature_candidate_download' | 'literature_candidate_ingest' | 'literature_auto_ingest'
  label: string
  description: string
  available: boolean
  reason: string
  writes: boolean
  network: boolean
  requiresInput: boolean
}

export type StartCompileRequest = {
  taskKind: CompileCapability['taskKind']
  inputPath?: string
  dryRun: boolean
  download: boolean
  force: boolean
  timeoutSeconds?: number
}

export type LiteratureIngestMode = 'prepare' | 'automatic' | 'manual' | 'download' | 'candidate'

export type LiteratureIngestSettings = {
  startupPromptEnabled: boolean
  autoPromoteEnabled: boolean
  minScore: number
  maxAutoIngest: number
  providers: string[]
  sinceYear?: number | null
  suppressedPromptDate: string
  lastAttemptAt: string
  lastSuccessAt: string
}

export type SearchProviderStatus = {
  id: string
  label: string
  description: string
  requiresKey: boolean
  configured: boolean
}

export type StartupPromptState = {
  shouldPrompt: boolean
  mode: 'prepare' | 'automatic'
  suppressedToday: boolean
  settings: LiteratureIngestSettings
}

export type DuplicateMatch = {
  kind: string
  value: string
  existingId: string
  existingPath: string
  title: string
}

export type ManualFilePreflight = {
  id: string
  path: string
  name: string
  size: number
  mtimeNs: number
  sha256: string
  valid: boolean
  selected: boolean
  errors: string[]
  duplicateMatches: DuplicateMatch[]
}

export type ManualImportSession = {
  id: string
  files: ManualFilePreflight[]
  createdAt: string
}

export type LiteratureCapability = {
  id: 'discovery' | 'download' | 'parse' | 'compile' | 'graphify'
  available: boolean
  reason: string
}

export type QualificationReason = {
  code: string
  passed: boolean
  message: string
}

export type LiteratureCandidate = {
  candidateId: string
  title: string
  authors: string[]
  year?: number | string
  abstract: string
  venue: string
  score: number
  doi: string
  arxivId: string
  pdfUrl: string
  sourceUrl: string
  provider: string
  triageStatus: 'pending' | 'selected' | 'rejected' | 'promoted'
  localPdf: string
  manualNote: string
  titleMatches: string[]
  abstractMatches: string[]
  matchedQueries: string[]
  manifestPath: string
  discoveryRuns: string[]
  duplicateMatches: DuplicateMatch[]
  qualification: {
    eligible: boolean
    score: number
    reasons: QualificationReason[]
    duplicates: DuplicateMatch[]
  }
}

export type StartLiteratureRunRequest = {
  mode: LiteratureIngestMode
  candidateIds?: string[]
  manualSessionId?: string
  selectedFileIds?: string[]
  forceDuplicates?: boolean
  timeoutSeconds?: number
}

export type CompileRunSummary = {
  id: string
  taskKind: string
  displayName: string
  status: 'queued' | 'running' | 'succeeded' | 'failed' | 'failed_partial' | 'cancelled' | 'interrupted' | 'rolled_back' | 'paused' | 'pause_requested' | 'resume_requested' | 'cancel_requested' | 'timed_out'
  currentStage: string
  createdAt: string
  startedAt: string
  finishedAt: string
  exitCode?: number | null
  failureReason: string
  retryOf: string
  timeoutSeconds: number
  currentStageIndex: number
  totalStages: number
  pauseRequested: boolean
  heartbeat: string
}

export type CompileRunEvent = {
  sequence: number
  eventKind: string
  stage: string
  message: string
  createdAt: string
}

export type CompileArtifact = {
  id: string
  artifactKind: string
  relativePath: string
  operation: string
  rollbackEligible: boolean
  beforeHash?: string
  afterHash?: string
}

export type CompileRunDetail = {
  summary: CompileRunSummary
  request: StartCompileRequest
  events: CompileRunEvent[]
  artifacts: CompileArtifact[]
}

export type CompileStreamEvent = {
  type: 'accepted' | 'stage_started' | 'stage_completed' | 'progress' | 'stdout' | 'stderr' | 'paused' | 'resumed' | 'completed' | 'failed' | 'cancelled' | 'timed_out'
  runId: string
  sequence: number
  stage: string
  message: string
  timestamp: string
}

export type RepositoryWatchStatus = {
  active: boolean
  root?: string | null
  processedChanges: number
  fullRebuild: boolean
  graphRefresh: boolean
  pendingChanges: number
  retryAttempt: number
  blocked: boolean
  lastError?: string | null
}

export type SearchResult = {
  id: string
  pageType: string
  title: string
  year: string
  summary: string
  sourcePath: string
  snippet: string
  score: number
}

export type PageSummary = {
  id: string
  pageType: string
  title: string
  year: string
  summary: string
  sourcePath: string
  modifiedAt: string
  status: string
  epistemic: string
  methodFamily: string
}

export type PageDetail = PageSummary & {
  body: string
  frontmatter: Record<string, string>
  links: string[]
}

export type PageFilters = {
  pageType?: string
  query?: string
  year?: string
  status?: string
  methodFamily?: string
  sort?: 'title' | 'year_asc' | 'modified'
  limit?: number
}

export type LinkResolution = {
  target: string
  anchor: string
  resolved: boolean
  page?: PageSummary | null
}

export type Backlink = {
  source: PageSummary
  target: string
}

export type BookSummary = {
  id: string
  title: string
  year: string
  pageCount: number
  chapterCount: number
  sourcePath: string
  pdfPath: string
  qualityStatus: string
}

export type BookChapter = {
  id: string
  bookId: string
  chapterNumber: number
  title: string
  markdownPath: string
  pdfPath: string
  physicalPageStart?: number | null
  physicalPageEnd?: number | null
  printedPageStart?: number | null
  printedPageEnd?: number | null
  charCount: number
  ingestStatus: string
}

export type BookChapterDetail = {
  chapter: BookChapter
  body: string
}

export type BookSearchResult = {
  chapter: BookChapter
  snippet: string
  score: number
}

export type GraphNode = {
  id: string
  label: string
  nodeType: string
  sourceFile: string
  sourceLocation: string
  community?: number | null
  origin: string
}

export type GraphEdge = {
  source: string
  target: string
  relation: string
  confidence: string
  weight: number
}

export type GraphOverview = {
  nodes: GraphNode[]
  edges: GraphEdge[]
  nodeCount: number
  edgeCount: number
  communityCount: number
}

export type GraphFilters = {
  query?: string
  nodeType?: string
  community?: number
  limit?: number
}

export type ComparisonCell = {
  value: string
  sourcePath: string
  field: string
}

export type ComparisonColumn = {
  id: string
  title: string
  pageType: string
  cells: Record<string, ComparisonCell>
}

export type ComparisonMatrix = {
  fields: string[]
  columns: ComparisonColumn[]
}

export type LunaSettings = {
  endpoint: string
  model: string
  apiKeyEnv: string
  timeoutSeconds: number
  maxOutputTokens: number
  temperature: number
  apiKeyConfigured: boolean
}

export type WaterlineSnapshot = {
  sourceCount: number
  methodCount: number
  synthesisCount: number
  chapterCount: number
  yearMin: string
  yearMax: string
  lastIngestAt: string
  repositoryPath: string
  capturedAt: string
}

export type EvidenceItem = {
  id: string
  kind: 'wiki' | 'book' | 'graph'
  tier: 'direct' | 'similar_model' | 'transferable_method' | 'theory' | 'graph_hint'
  title: string
  snippet: string
  score: number
  rank: number
  pageId: string
  pageType: string
  sourcePath: string
  wikilink: string
  bookId: string
  chapterId: string
  physicalPageStart?: number | null
  physicalPageEnd?: number | null
  markdownPath: string
  pdfPath: string
  nodeId: string
  sourceLocation: string
  relation: string
  retrievalReason: string
}

export type QuestionContext = {
  requestId: string
  question: string
  intent: 'solve' | 'novelty' | 'relationship'
  evidence: EvidenceItem[]
  waterline: WaterlineSnapshot
  generatedAt: string
}

export type ResearchContextAnchor = {
  kind: 'page' | 'question' | 'search' | 'idle'
  contextKey: string
  title: string
  subtitle: string
  pageId: string
  graphNodeId: string
}

export type ResearchTrailRequest = {
  kind: 'page' | 'question' | 'search'
  pageId?: string
  text?: string
  evidenceLimit?: number
  methodLimit?: number
}

export type ResearchTrailItem = {
  id: string
  kind: 'wiki' | 'book' | 'graph'
  rank: number
  title: string
  snippet: string
  score: number
  relation: string
  retrievalReason: string
  pageId: string
  pageType: string
  sourcePath: string
  wikilink: string
  bookId: string
  chapterId: string
  physicalPageStart?: number | null
  physicalPageEnd?: number | null
  markdownPath: string
  pdfPath: string
  nodeId: string
  sourceLocation: string
  graphPath: string[]
}

export type ResearchTrailResponse = {
  anchor: ResearchContextAnchor
  evidence: ResearchTrailItem[]
  methods: ResearchTrailItem[]
  degradedChannels: string[]
  generatedAt: string
}

export type ChatSessionSummary = {
  id: string
  title: string
  createdAt: string
  updatedAt: string
  messageCount: number
  lastMessagePreview: string
}

export type ChatMessage = {
  id: string
  sessionId: string
  role: 'user' | 'assistant' | 'system'
  content: string
  status: 'pending' | 'retrieving' | 'generating' | 'completed' | 'failed' | 'cancelled'
  createdAt: string
  errorCode: string
  errorMessage: string
  provider: string
  model: string
  requestId: string
  evidence: EvidenceItem[]
  waterline?: WaterlineSnapshot | null
}

export type ChatSessionDetail = {
  session: ChatSessionSummary
  messages: ChatMessage[]
}

export type AskRequest = {
  question: string
  sessionId?: string
  evidenceLimit?: number
}

export type AskResult = {
  requestId: string
  sessionId: string
  userMessage: ChatMessage
  assistantMessage: ChatMessage
  evidence: EvidenceItem[]
  waterline: WaterlineSnapshot
  offline: boolean
}

export type AnswerStreamEvent =
  | { type: 'started'; payload: { requestId: string; sessionId: string } }
  | { type: 'retrieval_started'; payload: { requestId: string } }
  | { type: 'retrieval_completed'; payload: { requestId: string; evidence: EvidenceItem[]; waterline: WaterlineSnapshot } }
  | { type: 'token'; payload: { requestId: string; content: string } }
  | { type: 'completed'; payload: { requestId: string; result: AskResult } }
  | { type: 'failed'; payload: { requestId: string; code: string; message: string; retryable: boolean } }
  | { type: 'cancelled'; payload: { requestId: string } }

export type DashboardData = {
  generatedAt: string
  waterline: { sources: number; methods: number; syntheses: number; chapters: number; updatedAt: string }
  graph: { nodes: number; edges: number; communities: number }
  recent: { kind: string; title: string; meta: string; date: string; state: string }[]
  trends: { label: string; wireless: number; road: number; rl: number }[]
  topics: { label: string; x: number; y: number; size: number; tone: string }[]
  evidence: { rank: number; title: string; meta: string; quote: string }[]
  methods: { title: string; tags: string[]; meta: string; favorite: boolean }[]
  compileTask: { title: string; phase: string; progress: number; inputs: number; output: string }
}

export const fallbackData: DashboardData = {
  generatedAt: '',
  waterline: { sources: 23, methods: 20, syntheses: 7, chapters: 61, updatedAt: '2026-08-01' },
  graph: { nodes: 811, edges: 799, communities: 80 },
  recent: [],
  trends: [],
  topics: [],
  evidence: [],
  methods: [],
  compileTask: { title: '无线充电调度研究文献编译', phase: '等待数据', progress: 0, inputs: 0, output: '' },
}
