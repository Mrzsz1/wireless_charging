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
  id: 'discovery' | 'download' | 'parse' | 'compile' | 'graph' | 'full_ingest'
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

export type SourceLocator = {
  documentId: string
  blockId: string
  headingPath: string[]
  markdownPath: string
  lineStart?: number | null
  lineEnd?: number | null
  contentHash: string
  snapshotId: string
}

export type ResolvedSourceLocation = {
  documentId: string
  blockId: string
  markdownPath: string
  headingPath: string[]
  lineStart?: number | null
  lineEnd?: number | null
  matchedBy: 'block' | 'heading' | 'line' | 'document'
  contentHashMatches: boolean
  degradedReason: string
}

export type ResolvedSourceDocument = {
  title: string
  body: string
  location: ResolvedSourceLocation
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

export type AnswerProvider = 'codex-subscription' | 'compatible-api' | 'offline-evidence'

export type QaSettings = {
  answerProvider: AnswerProvider
  codexModel: string
  codexReasoningEffort: string
  endpoint: string
  model: string
  apiKeyEnv: string
  timeoutSeconds: number
  maxOutputTokens: number
  contextWindowTokens: number
  temperature: number
  apiKeyConfigured: boolean
}

export type LunaSettings = QaSettings

export type VocabularyKind = 'objective' | 'constraint' | 'assumption' | 'method' | 'parameter'
export type ParameterValueKind = 'integer' | 'float' | 'boolean' | 'text' | 'enum'

export type StateParameterSpec = {
  valueKind: ParameterValueKind
  unit?: string | null
  minimum?: number | null
  maximum?: number | null
  enumValues: string[]
}

export type StateFieldDefinition = {
  id: string
  kind: VocabularyKind
  label: string
  description: string
  aliases: string[]
  examples: string[]
  parameterSpec?: StateParameterSpec | null
  origin: 'built_in' | 'custom'
  enabled: boolean
}

export type StateVocabularyRegistry = {
  schemaVersion: string
  revision: number
  fields: StateFieldDefinition[]
}

export type CustomStateFieldInput = {
  kind: VocabularyKind
  label: string
  description: string
  aliases: string[]
  examples: string[]
  parameterSpec?: StateParameterSpec | null
}

export type StateVocabularyMappingDryRun = {
  dryRun: boolean
  vocabularyRevision: number
  vocabularyHash: string
  semanticMappingAttempted: boolean
  semanticMappingUsed: boolean
  mappedFields: Array<{
    fieldId: string
    label: string
    kind: string
    confidence: string
    action: string
  }>
}

export type SemanticModelSettings = {
  cacheDir: string
  effectiveCacheDir: string
  defaultCacheDir: string
  usingDefault: boolean
  modelName: string
  remoteVectorEnabled: boolean
  remoteVectorEndpoint: string
  remoteVectorKeyConfigured: boolean
}

export type VectorStoreStats = {
  store: string
  ready: boolean
  vectorCount: number
  documentCount: number
  pendingSyncCount: number
  modelId: string
  dimension: number
  lastError: string
}

export type SemanticVectorStatus = {
  schemaVersion: string
  modelName: string
  dimension: number
  activeSnapshot: string
  local: VectorStoreStats
  remote: VectorStoreStats
  remoteEnabled: boolean
  remoteKeyConfigured: boolean
  countsByGranularity: Record<string, number>
  lastSyncAt: string
  lastError: string
}

export type VectorSyncProgress = {
  phase: string
  status: string
  totalBlocks: number
  completedBlocks: number
  computedBlocks: number
  reusedBlocks: number
  remoteSyncedBlocks: number
  percent: number
  message: string
}

export type SemanticDeploymentState = 'missing' | 'partial' | 'invalid' | 'ready' | 'error'

export type SemanticDeploymentStatus = {
  state: SemanticDeploymentState
  modelName: string
  cacheDir: string
  defaultCacheDir: string
  runtimeReady: boolean
  modelFilesReady: boolean
  tokenizerReady: boolean
  partialDownloadCount: number
  totalBytes: number
  probeDimension: number
  checkedAt: string
  diagnostic: string
}

export type RerankerDeploymentStatus = {
  state: SemanticDeploymentState
  modelName: string
  modelVersion: string
  modelDir: string
  runtimeReady: boolean
  modelFilesReady: boolean
  tokenizerReady: boolean
  healthChecked: boolean
  checkedAt: string
  diagnostic: string
}

export type SemanticDownloadProgress = {
  status: 'starting' | 'downloading' | 'verifying' | 'complete' | 'skipped' | 'failed' | 'cancelled'
  phase: 'runtime' | 'model' | 'tokenizer' | 'inference'
  fileName: string
  downloadedBytes: number
  totalBytes: number
  percent: number
  bytesPerSecond: number
  message: string
}

export type CodexSubscriptionStatus = {
  installed: boolean
  version: string
  authenticated: boolean
  ready: boolean
  statusLabel: string
  diagnostic: string
  configuredModel: string
  configuredReasoningEffort: string
  availableModels: CodexModelOption[]
  modelCatalogStatus: 'detected' | 'missing' | 'invalid'
}

export type CodexModelOption = {
  id: string
  displayName: string
  defaultReasoningEffort: string
  supportedReasoningEfforts: string[]
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
  indexSnapshotId: string
}

export type ContextBudget = {
  contextWindowTokens: number
  inputBudgetTokens: number
  researchContractTokens: number
  sessionMemoryTokens: number
  recentHistoryTokens: number
  currentQueryTokens: number
  evidenceTokens: number
  serializationOverheadTokens: number
  outputReserveTokens: number
  safetyMarginTokens: number
  estimatedTotalTokens: number
  freeTokens: number
  recentExchangeCount: number
  compactedMessageCount: number
  truncated: boolean
}

export type ContextPlan = {
  schemaVersion: string
  sessionMemory: string
  researchState?: ResearchSessionState
  recentMessageIds: string[]
  compactedMessageIds: string[]
  fingerprint: string
  budget: ContextBudget
}

export type ResearchParameterValue =
  | { type: 'integer'; value: number }
  | { type: 'float'; value: number }
  | { type: 'boolean'; value: boolean }
  | { type: 'text'; value: string }

export type ResearchParameter = {
  key: string
  value: ResearchParameterValue
  unit?: string | null
  sourceMessageId?: string | null
  updatedAtTurn: number
}

export type ResearchSessionState = {
  schemaVersion: string
  stateVersion: string
  revision: number
  activeProblem: string
  objectives: string[]
  constraints: string[]
  assumptions: string[]
  methods: string[]
  excludedMethods: string[]
  parameters: Record<string, ResearchParameter>
  papers: string[]
  hypotheses: string[]
  openQuestions: string[]
  sourceMessageIds: string[]
  lastPatchId: string
}

export type ResearchQueryContext = {
  schemaVersion: string
  currentQuestion: string
  researchIntent: string
  objectives: string[]
  constraints: string[]
  assumptions: string[]
  parameters: Record<string, ResearchParameter>
  activeMethods: string[]
  excludedMethods: string[]
  resolvedReferences: string[]
  sourceStateRevision: number
  activeVocabularyFields: ActiveVocabularyField[]
}

export type ActiveVocabularyField = {
  id: string
  kind: string
  label: string
  description: string
  searchTerms: string[]
}

export type EvidenceChecksum = {
  evidenceId: string
  stableSourceId: string
  sha256: string
}

export type CitationRepair = {
  applied: boolean
  removedUnknownIds: string[]
  normalizedCitationGroups: number
}

export type AnswerCompletenessValidation = {
  applicable: boolean
  requiredSections: string[]
  missingSections: string[]
  requiredElements: string[]
  missingElements: string[]
  claimCount: number
  minimumClaimCount: number
  complete: boolean
}

export type EvidenceAvailabilityMode = 'grounded' | 'partial_coverage' | 'zero_usable_evidence'

export type ZeroEvidenceAudit = {
  schemaVersion: string
  applicable: boolean
  status: 'not_applicable' | 'succeeded' | 'failed' | string
  availabilityMode: EvidenceAvailabilityMode | string
  reason: string
  rawEvidenceCount: number
  supportEligibleEvidenceCount: number
  graphOnlyEvidenceCount: number
  noticePresent: boolean
  noticeCount: number
  visibleBodyNonEmpty: boolean
  epistemicBoundaryPresent: boolean
  epistemicStatus: 'unverified_general_knowledge' | string
  evidenceSupportApplicable: boolean
  citationTokenCount: number
  unknownCitationCount: number
  referenceAppendixPresent: boolean
  evidenceLinkPresent: boolean
  forbiddenKbAttributionCount: number
  trustedContextEmpty: boolean
  fallbackApplied: boolean
  fallbackReason: string
  complete: boolean
  errorCodes: string[]
}

export type ClaimType = 'knowledge_fact' | 'general_knowledge' | 'reasoned_inference' | 'research_suggestion'

export type VerificationStatus =
  | 'unverified'
  | 'supported'
  | 'partially_supported'
  | 'contradicted'
  | 'not_verifiable'
  | 'not_applicable'
  | 'unavailable'

export type AtomicClaim = {
  id: string
  text: string
  evidenceIds: string[]
  claimType: ClaimType
  verificationStatus: VerificationStatus
  confidence?: number | null
  verificationMethod: string
  alignmentScore: number
  reason: string
}

export type VerifiedClaim = AtomicClaim

export type RepairProjectionOperation = {
  claimId: string
  sourceStart: number
  sourceEnd: number
  originalTextSha256: string
  replacementKind: string
}

export type RepairProjectionAudit = {
  schemaVersion: string
  status: 'not_run' | 'succeeded' | 'failed' | string
  errorCode: string
  sourceBodySha256: string
  repairedBodySha256: string
  operationCount: number
  operations: RepairProjectionOperation[]
}

export type QaRunManifest = {
  schemaVersion: string
  promptVersion: string
  answerSchemaVersion: string
  retrieverVersion: string
  contextSchemaVersion: string
  provider: string
  structuredOutputMode: string
  answerFormat?: 'natural-markdown-v2' | 'structured-v1' | 'legacy-markdown' | string
  modelRequested: string
  modelResolved: string
  temperature?: number | null
  maxOutputTokens: number
  contextWindowTokens: number
  promptSha256: string
  indexSnapshotId: string
  recentHistoryMessageIds: string[]
  compactedHistoryMessageIds: string[]
  resolvedHistoryMessageIds: string[]
  evidenceChecksums: EvidenceChecksum[]
  contextBudget: ContextBudget
  citationRepair: CitationRepair
  answerCompleteness: AnswerCompletenessValidation
  evidenceAvailabilityMode?: EvidenceAvailabilityMode | string
  supportEligibleEvidenceCount?: number
  graphOnlyEvidenceCount?: number
  zeroEvidenceReason?: string
  zeroEvidenceAudit?: ZeroEvidenceAudit
  queryPlanVersion?: string
  plannerStatus?: string
  plannerLatencyMs?: number
  plannerFallback?: boolean
  plannerFallbackReason?: string
  planningProvider?: string
  providerCapabilities?: string[]
  resolverUsed?: string
  resolverStatus?: string
  resolverLatencyMs?: number
  resolverFallback?: boolean
  resolverFallbackReason?: string
  routingConfidence?: 'high' | 'medium' | 'low' | string
  resolverEscalated?: boolean
  researchIntent?: string
  executionMode?: 'direct' | 'research' | 'exploratory' | string
  routingReason?: string
  routerUsed?: string
  routerStatus?: string
  routerLatencyMs?: number
  routerFallback?: boolean
  plannedFacetIds?: string[]
  coveredFacetIds?: string[]
  rerankerVersion?: string
  rerankerRequested?: boolean
  rerankerProvider?: string
  rerankerModel?: string
  rerankerAvailable?: boolean
  rerankerStatus?: string
  rerankerLatencyMs?: number
  rerankerCandidateCount?: number
  rerankerBatchSize?: number
  rerankerBatchCount?: number
  rerankerModelMaxLength?: number
  rerankerModelLoadMs?: number
  rerankerInputPrepareMs?: number
  rerankerInferenceMs?: number
  rerankerAverageInputTokens?: number
  rerankerFallback?: boolean
  rerankerFallbackReason?: string
  evidenceManagerVersion?: string
  evidenceInputCount?: number
  evidenceDeduplicatedCount?: number
  evidenceSelectedCount?: number
  evidenceDocumentCount?: number
  evidenceParentExpansionCount?: number
  evidenceEstimatedTokens?: number
  claimExtractorVersion?: string
  claimVerifierVersion?: string
  verificationStatus?: 'not_run' | 'succeeded' | 'unavailable' | string
  verificationFallback?: boolean
  semanticVerificationChecked?: boolean
  heuristicVerificationChecked?: boolean
  verificationProvider?: string
  verificationModel?: string
  semanticVerificationStatus?: 'not_run' | 'not_requested' | 'succeeded' | 'unavailable' | string
  semanticVerificationLatencyMs?: number
  semanticVerificationFallbackReason?: string
  verifiedClaimCount?: number
  partiallySupportedClaimCount?: number
  contradictedClaimCount?: number
  notVerifiableClaimCount?: number
  notApplicableClaimCount?: number
  unverifiedClaimCount?: number
  unavailableClaimCount?: number
  repairedClaimCount?: number
  repairProjectionAudit?: RepairProjectionAudit
  claimVerifications?: VerifiedClaim[]
  finalGroundingAudit?: FinalGroundingAudit
  problemParserVersion?: string
  methodMatcherVersion?: string
  problemUnderstandingStatus?: string
  problemDomain?: string
  problemObjectives?: string[]
  problemConstraints?: string[]
  relatedProblemTypes?: string[]
  candidateMethods?: string[]
  methodHypotheses?: string[]
  discoveredMethods?: string[]
  corroboratedMethodHypotheses?: string[]
  methodEvidenceProvenance?: string[]
  researchStateVersion?: string
  researchStateRevision?: number
  researchStateObjectiveCount?: number
  researchStateConstraintCount?: number
  statePatchOperationCount?: number
  statePatchLowConfidenceCount?: number
  statePatchRejectedCount?: number
  parameterImplicitReferenceResolvedCount?: number
  parameterImplicitReferenceRejectedCount?: number
  parameterUnknownNameCount?: number
  parameterStateCorruptionCount?: number
  stateChanged?: boolean
  stateWarningCount?: number
  queryContextObjectiveCount?: number
  queryContextConstraintCount?: number
  queryContextParameterCount?: number
  queryContextExcludedMethodCount?: number
  routingPolicyVersion?: string
  routingMaxRounds?: number
  routingMaxQueries?: number
  routingMaxCandidates?: number
  routingLlmCallBudget?: number
  routingTokenCostCeiling?: number
  routingLlmCallsUsed?: number
  routingTokenCostUsed?: number
  routingTokenCostInFlight?: number
  routingTokenCostReserved?: number
  routingTokenCostReservedTotal?: number
  routingBudgetRejections?: string[]
  routingLlmStages?: string[]
  retrievalStopReason?: string
  retrievalRoundCount?: number
  requestedKinds?: string[]
  attemptedKinds?: string[]
  sourceGapCount?: number
  retrievalChannelStatuses?: string[]
  retrievalRoundFingerprints?: string[]
  generatedAt: string
}

export type FinalGroundingAudit = {
  schemaVersion: string
  auditStatus: 'not_run' | 'succeeded' | string
  groundingStatus: 'supported' | 'insufficient_supported_claims' | 'invalid' | 'unverified' | string
  factualClaimCount: number
  supportedCount: number
  unsupportedCount: number
  notApplicableCount: number
  citedClaimCount: number
  citedEvidenceIds: string[]
  unknownEvidenceIds: string[]
  citationPrecision: number
  citationCoverage: number
  claims: VerifiedClaim[]
  claimSources?: FinalClaimSource[]
  visibleProjectionValid?: boolean
  auditedBodySha256?: string
  visibleBodySha256?: string
}

export type FinalClaimSource = {
  finalClaimId: string
  sourceDraftClaimId: string
  textSha256: string
  evidenceIds: string[]
  draftVerificationMethod: string
  draftAlignmentScore: number
  draftConfidence?: number | null
}

export type EvidenceItem = {
  id: string
  kind: 'wiki' | 'paper' | 'book' | 'graph'
  tier: 'primary_source' | 'direct' | 'similar_model' | 'transferable_method' | 'theory' | 'graph_hint'
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
  locator?: SourceLocator | null
}

export type RetrievalChannelDiagnostic = {
  name: string
  durationMs: number
  candidateCount: number
  round: number
  status: 'not_requested' | 'attempted_zero_hit' | 'succeeded_with_hits' | 'degraded' | 'failed' | string
  errorKind: string
  roundFingerprint: string
}

export type RetrievalDiagnostics = {
  totalMs: number
  channels: RetrievalChannelDiagnostic[]
  selectedCount: number
  cancelCheckCount: number
  passCount: number
  stopReason: string
  candidateGains: number[]
}

export type QuestionContext = {
  requestId: string
  question: string
  intent: 'solve' | 'novelty' | 'relationship' | 'literature'
  retrievalQuery: {
    originalQuestion: string
    resolvedQuestion: string
    entities: string[]
    intent: string
    usedHistoryMessageIds: string[]
    usedReferenceHistoryMessageIds?: string[]
    researchIntent?: string
    executionMode?: 'direct' | 'research' | 'exploratory' | string
    routingReason?: string
    resolverUsed?: string
    resolverStatus?: string
    resolverLatencyMs?: number
    resolverFallback?: boolean
    resolverFallbackReason?: string
    routingConfidence?: 'high' | 'medium' | 'low' | string
    resolverEscalated?: boolean
    routerUsed?: string
    routerStatus?: string
    routerLatencyMs?: number
    routerFallback?: boolean
    queryPlanVersion?: string
    facetIds?: string[]
    plannedRequiredFacetIds?: string[]
    coveredFacetIds?: string[]
    plannerUsed?: boolean
    plannerStatus?: string
    plannerLatencyMs?: number
    plannerFallback?: boolean
    plannerFallbackReason?: string
    planningProvider?: string
    providerCapabilities?: string[]
    rerankerVersion?: string
    rerankerStatus?: string
    rerankerLatencyMs?: number
    rerankerCandidateCount?: number
    rerankerBatchSize?: number
    rerankerBatchCount?: number
    rerankerModelMaxLength?: number
    rerankerModelLoadMs?: number
    rerankerInputPrepareMs?: number
    rerankerInferenceMs?: number
    rerankerAverageInputTokens?: number
    rerankerFallback?: boolean
    rerankerFallbackReason?: string
    evidenceManagerVersion?: string
    evidenceInputCount?: number
    evidenceDeduplicatedCount?: number
    evidenceSelectedCount?: number
    evidenceDocumentCount?: number
    evidenceParentExpansionCount?: number
    evidenceEstimatedTokens?: number
    evidenceAvailabilityMode?: EvidenceAvailabilityMode | string
    supportEligibleEvidenceCount?: number
    graphOnlyEvidenceCount?: number
    zeroEvidenceReason?: string
    problemParserVersion?: string
    methodMatcherVersion?: string
    problemUnderstandingStatus?: string
    problemDomain?: string
    problemObjectives?: string[]
    problemConstraints?: string[]
    relatedProblemTypes?: string[]
    candidateMethods?: string[]
    methodHypotheses?: string[]
    discoveredMethods?: string[]
    corroboratedMethodHypotheses?: string[]
    methodEvidenceProvenance?: string[]
    problemSearchTerms?: string[]
    routingPolicyVersion?: string
    routingMaxRounds?: number
    routingMaxQueries?: number
    routingMaxCandidates?: number
    routingLlmCallBudget?: number
    routingTokenCostCeiling?: number
    routingLlmCallsUsed?: number
    routingTokenCostUsed?: number
    routingTokenCostInFlight?: number
    routingTokenCostReserved?: number
    routingTokenCostReservedTotal?: number
    routingBudgetRejections?: string[]
    routingLlmStages?: string[]
    requestedKinds?: string[]
    attemptedKinds?: string[]
    sourceGaps?: string[]
    researchQueryContext?: ResearchQueryContext
    researchStateVersion?: string
    statePatchOperationCount?: number
    statePatchLowConfidenceCount?: number
    statePatchRejectedCount?: number
    parameterImplicitReferenceResolvedCount?: number
    parameterImplicitReferenceRejectedCount?: number
    parameterUnknownNameCount?: number
    parameterStateCorruptionCount?: number
    stateChanged?: boolean
    stateWarningCount?: number
    queryContextObjectiveCount?: number
    queryContextConstraintCount?: number
    queryContextParameterCount?: number
    queryContextExcludedMethodCount?: number
  }
  conversation: ConversationTurn[]
  evidence: EvidenceItem[]
  retrievalDiagnostics: RetrievalDiagnostics
  contextPlan: ContextPlan
  waterline: WaterlineSnapshot
  generatedAt: string
}

export type ConversationTurn = { id: string; role: string; content: string; requestId: string }

export type CitationValidation = {
  citedIds: string[]
  unknownIds: string[]
  citationPrecision: number
  hasCitations: boolean
  supported: boolean
  groundingStatus: 'supported' | 'partially_supported' | 'mixed' | 'unverified' | 'invalid' | 'insufficient_supported_claims'
  zeroEvidence: boolean
  claimCount: number
  citedClaimCount: number
  citationCoverage: number
  unsupportedClaims: string[]
  graphOnlyClaims: string[]
  syntaxValid: boolean
  coverageValid: boolean
  entailmentChecked: boolean
  heuristicVerificationChecked?: boolean
  modelSupplementClaimCount: number
  modelSupplementClaims: string[]
  appendixIntegrity?: boolean
  appendixEvidenceIds?: string[]
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
  status: 'pending' | 'retrieving' | 'generating' | 'completed' | 'mixed' | 'unverified' | 'failed' | 'cancelled'
  createdAt: string
  errorCode: string
  errorMessage: string
  provider: string
  model: string
  requestId: string
  evidence: EvidenceItem[]
  waterline?: WaterlineSnapshot | null
  citationValidation?: CitationValidation | null
  runManifest?: QaRunManifest | null
}

export type ChatSessionDetail = {
  session: ChatSessionSummary
  messages: ChatMessage[]
}

export type ChatSessionPage = {
  items: ChatSessionSummary[]
  nextCursor?: string | null
}

export type ChatMessagePage = {
  session: ChatSessionSummary
  messages: ChatMessage[]
  nextCursor?: string | null
}

export type AskRequest = {
  requestId: string
  question: string
  sessionId?: string
  evidenceLimit?: number
  repositoryId: string
  codexModel?: string
  codexReasoningEffort?: string
}

export type AskResult = {
  requestId: string
  sessionId: string
  userMessage: ChatMessage
  assistantMessage: ChatMessage
  evidence: EvidenceItem[]
  retrievalDiagnostics: RetrievalDiagnostics
  contextBudget: ContextBudget
  runManifest: QaRunManifest
  waterline: WaterlineSnapshot
  offline: boolean
  citationValidation: CitationValidation
}

export type AnswerStreamEvent =
  | { type: 'started'; payload: { requestId: string; sessionId: string } }
  | { type: 'retrieval_started'; payload: { requestId: string } }
  | { type: 'retrieval_completed'; payload: { requestId: string; evidence: EvidenceItem[]; retrievalDiagnostics: RetrievalDiagnostics; contextBudget: ContextBudget; waterline: WaterlineSnapshot } }
  | { type: 'validation_started'; payload: { requestId: string } }
  | { type: 'completed'; payload: { requestId: string; result: AskResult } }
  | { type: 'failed'; payload: { requestId: string; code: string; message: string; retryable: boolean; exchange?: { sessionId: string; userMessage: ChatMessage; assistantMessage: ChatMessage } | null } }
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
