import { lazy, Suspense, useEffect, useRef, useState } from 'react'
import { BookOpen, Bot, Check, CheckCircle2, ChevronRight, CircleStop, Clipboard, FileText, GitBranch, LoaderCircle, MessageSquarePlus, MoreHorizontal, Plus, RefreshCw, Search, Send, Settings, ShieldCheck, Trash2, X } from 'lucide-react'
import { askLuna, cancelAnswer, deleteChatSession, getChatSessionPage, getCodexSubscriptionStatus, getQaSettings, isDesktopRuntime, listChatSessionsPage, renameChatSession, saveQaSettings } from '../../services/desktop'
import type { AnswerProvider, AnswerStreamEvent, AskResult, ChatMessage, ChatSessionSummary, CodexSubscriptionStatus, ContextBudget, EvidenceItem, QaRunManifest, QaSettings, RetrievalDiagnostics, SourceLocator, WaterlineSnapshot } from '../../types'
import { claimCompletion, createCompletionLedger, mergeCompletedMessages, mergeFailedMessages, repositoryIdentity, retryQuestionFor, rollbackOptimisticMessages } from './completionState'
import { appendUniqueSessions, buildAuditBundle, citationSummary, evidenceEmptyState, evidencePanelOwnership, prependUniqueMessages } from './qaPresentation'
import './AskView.css'

const MarkdownMessage = lazy(() => import('./MarkdownMessage'))
const SESSION_PAGE_SIZE = 40
const MESSAGE_PAGE_SIZE = 60

type AskViewProps = {
  repositoryPath?: string
  onOpenSettings: () => void
  onResearchContextChange: (question: string | null) => void
  onOpenPage: (pageId: string, title?: string) => void
  onOpenBook: (bookId: string, chapterId: string) => void
  onOpenPath: (path: string, reveal?: boolean) => void
  onOpenSource: (locator: SourceLocator, label: string) => void
}

const emptySettings: QaSettings = {
  answerProvider: 'offline-evidence',
  codexModel: '',
  codexReasoningEffort: '',
  endpoint: '',
  model: 'gpt-5.6-luna',
  apiKeyEnv: 'LUNA_API_KEY',
  timeoutSeconds: 180,
  maxOutputTokens: 1800,
  contextWindowTokens: 32768,
  temperature: 0.1,
  apiKeyConfigured: false,
}

const emptyCodexStatus: CodexSubscriptionStatus = { installed: false, version: '', authenticated: false, ready: false, statusLabel: '尚未检测', diagnostic: '', configuredModel: '', configuredReasoningEffort: '', availableModels: [], modelCatalogStatus: 'missing' }

function providerLabel(provider: string) {
  if (provider === 'codex-subscription') return 'Codex 订阅'
  if (provider === 'luna' || provider === 'compatible-api') return '兼容 API'
  return '证据浏览'
}

function providerReady(provider: AnswerProvider, settings: QaSettings, codex: CodexSubscriptionStatus) {
  if (provider === 'codex-subscription') return codex.ready
  if (provider === 'compatible-api') return settings.apiKeyConfigured && Boolean(settings.endpoint)
  return true
}

const effortLabels: Record<string, string> = { none: '无', low: '低', medium: '中', high: '高', xhigh: '极高', max: '最大', ultra: 'Ultra' }

const suggestions = [
  '有没有适合在线无线充电请求调度的解决办法？',
  '核心书籍中有哪些可迁移到无线充电调度的近似算法？',
  '库内有哪些干涉感知并发充电模型，它们的约束有什么区别？',
  '移动充电器路径规划与服务调度之间有哪些相似模型？',
]

function formatTime(value: string) {
  const timestamp = Number(value)
  if (!Number.isFinite(timestamp)) return ''
  return new Date(timestamp).toLocaleString('zh-CN', { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' })
}

function localMessage(role: 'user' | 'assistant', content: string, status: ChatMessage['status'], requestId = ''): ChatMessage {
  return { id: `local-${requestId || Date.now()}-${role}`, sessionId: '', role, content, status, createdAt: String(Date.now()), errorCode: '', errorMessage: '', provider: role === 'assistant' ? 'pending' : 'local', model: '', requestId, evidence: [], citationValidation: null }
}

function tierLabel(tier: EvidenceItem['tier']) {
  return tier === 'primary_source' ? '论文原文' : tier === 'direct' ? '直接证据' : tier === 'similar_model' ? '相似模型' : tier === 'transferable_method' ? '可迁移算法' : tier === 'theory' ? '理论基础' : '图谱提示'
}

function kindIcon(kind: EvidenceItem['kind']) {
  return kind === 'book' ? <BookOpen size={15} /> : kind === 'graph' ? <GitBranch size={15} /> : <FileText size={15} />
}

function evidenceLocationLabel(item: EvidenceItem) {
  const heading = item.locator?.headingPath?.at(-1)
  if (heading) return heading
  if (item.sourceLocation) return item.sourceLocation
  if (item.kind === 'book' && item.physicalPageStart) return `PDF p.${item.physicalPageStart}–${item.physicalPageEnd ?? '?'}`
  return item.locator ? 'Markdown 来源' : '旧版来源记录'
}

function verificationStatusLabel(status: string) {
  return status === 'supported' ? '支持'
    : status === 'partially_supported' ? '部分支持'
      : status === 'contradicted' ? '冲突'
        : status === 'not_verifiable' ? '不可核验'
          : '不适用'
}

function MessageContent({ content, evidence, onCitation }: { content: string; evidence: EvidenceItem[]; onCitation: (item: EvidenceItem) => void }) {
  return <div className="qa-message-content"><Suspense fallback={<span className="qa-markdown-loading">{content}</span>}><MarkdownMessage content={content} evidence={evidence} onCitation={onCitation} /></Suspense></div>
}

function CitationStatus({ message }: { message: ChatMessage }) {
  const summary = citationSummary(message.citationValidation, message.runManifest?.answerFormat)
  if (!summary) return null
  return <div className={`qa-citation-status ${summary.tone}`}><strong>{summary.label}</strong><span>{summary.detail}</span></div>
}

export function AskView({ repositoryPath, onOpenSettings, onResearchContextChange, onOpenPage, onOpenBook, onOpenPath, onOpenSource }: AskViewProps) {
  const [sessions, setSessions] = useState<ChatSessionSummary[]>([])
  const [sessionQuery, setSessionQuery] = useState('')
  const [activeSessionId, setActiveSessionId] = useState('')
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const [question, setQuestion] = useState('')
  const [phase, setPhase] = useState<'idle' | 'retrieving' | 'generating' | 'validating'>('idle')
  const [elapsedSeconds, setElapsedSeconds] = useState(0)
  const [hasFirstToken, setHasFirstToken] = useState(false)
  const generationStartedAt = useRef(0)
  const composerRef = useRef<HTMLTextAreaElement>(null)
  const [streamingText, setStreamingText] = useState('')
  const [requestId, setRequestId] = useState('')
  const [evidence, setEvidence] = useState<EvidenceItem[]>([])
  const [evidenceRequestId, setEvidenceRequestId] = useState('')
  const [selectedEvidence, setSelectedEvidence] = useState<EvidenceItem | null>(null)
  const [waterline, setWaterline] = useState<WaterlineSnapshot | null>(null)
  const [retrievalDiagnostics, setRetrievalDiagnostics] = useState<RetrievalDiagnostics | null>(null)
  const [contextBudget, setContextBudget] = useState<ContextBudget | null>(null)
  const [runManifest, setRunManifest] = useState<QaRunManifest | null>(null)
  const [settings, setSettings] = useState<QaSettings>(emptySettings)
  const [codexStatus, setCodexStatus] = useState<CodexSubscriptionStatus>(emptyCodexStatus)
  const [loadingHistory, setLoadingHistory] = useState(false)
  const [sessionCursor, setSessionCursor] = useState('')
  const [sessionHasMore, setSessionHasMore] = useState(false)
  const [loadingMoreSessions, setLoadingMoreSessions] = useState(false)
  const [messageCursor, setMessageCursor] = useState('')
  const [messageHasMore, setMessageHasMore] = useState(false)
  const [loadingOlderMessages, setLoadingOlderMessages] = useState(false)
  const [error, setError] = useState('')
  const endRef = useRef<HTMLDivElement>(null)
  const messagesRef = useRef<HTMLDivElement>(null)
  const previousScrollHeight = useRef<number | null>(null)
  const completionLedger = useRef(createCompletionLedger())
  const repositoryGeneration = useRef(0)
  const activeRequestId = useRef('')
  const mountedRepositoryPath = useRef<string | null | undefined>(null)
  const sessionListGeneration = useRef(0)
  const sessionOpenGeneration = useRef(0)

  useEffect(() => {
    if (phase === 'idle' || !generationStartedAt.current) return
    const update = () => setElapsedSeconds(Math.max(0, Math.floor((performance.now() - generationStartedAt.current) / 1000)))
    update()
    const timer = window.setInterval(update, 1000)
    return () => window.clearInterval(timer)
  }, [phase])

  useEffect(() => {
    const textarea = composerRef.current
    if (!textarea) return
    textarea.style.height = 'auto'
    textarea.style.height = `${Math.min(textarea.scrollHeight, 148)}px`
    textarea.style.overflowY = textarea.scrollHeight > 148 ? 'auto' : 'hidden'
  }, [question])

  const resetGenerationState = () => {
    generationStartedAt.current = 0
    setElapsedSeconds(0)
    setHasFirstToken(false)
    setPhase('idle')
  }

  const replaceSessionPage = async (query = sessionQuery) => {
    if (!isDesktopRuntime() || !repositoryPath) return
    const requestGeneration = ++sessionListGeneration.current
    const page = await listChatSessionsPage(undefined, query.trim() || undefined, SESSION_PAGE_SIZE)
    if (requestGeneration !== sessionListGeneration.current) return
    setSessions(page.items)
    setSessionCursor(page.nextCursor ?? '')
    setSessionHasMore(Boolean(page.nextCursor))
  }

  const refreshSessions = async () => {
    if (!isDesktopRuntime() || !repositoryPath) return
    try { await replaceSessionPage() } catch (cause) { setError(`会话列表加载失败：${String(cause)}`) }
  }

  const loadMoreSessions = async () => {
    if (!sessionCursor || loadingMoreSessions || !repositoryPath) return
    setLoadingMoreSessions(true)
    const requestGeneration = sessionListGeneration.current
    try {
      const page = await listChatSessionsPage(sessionCursor, sessionQuery.trim() || undefined, SESSION_PAGE_SIZE)
      if (requestGeneration !== sessionListGeneration.current) return
      setSessions((current) => appendUniqueSessions(current, page.items))
      setSessionCursor(page.nextCursor ?? '')
      setSessionHasMore(Boolean(page.nextCursor))
    } catch (cause) { setError(`加载更多会话失败：${String(cause)}`) } finally { setLoadingMoreSessions(false) }
  }

  useEffect(() => {
    if (mountedRepositoryPath.current === repositoryPath) return
    mountedRepositoryPath.current = repositoryPath
    repositoryGeneration.current += 1
    sessionListGeneration.current += 1
    sessionOpenGeneration.current += 1
    const staleRequest = activeRequestId.current
    activeRequestId.current = ''
    if (staleRequest && isDesktopRuntime()) void cancelAnswer(staleRequest).catch(() => undefined)
    generationStartedAt.current = 0
    setElapsedSeconds(0)
    setHasFirstToken(false)
    setPhase('idle')
    completionLedger.current = createCompletionLedger(repositoryPath ?? '')
    setActiveSessionId('')
    setMessages([])
    setEvidence([])
    setEvidenceRequestId('')
    setRetrievalDiagnostics(null)
    setWaterline(null)
    setSessionCursor('')
    setSessionHasMore(false)
    setMessageCursor('')
    setMessageHasMore(false)
    onResearchContextChange(null)
    if (!isDesktopRuntime() || !repositoryPath) return
    const generation = repositoryGeneration.current
    void getQaSettings().then(async (qa) => {
      const codex = qa.answerProvider === 'codex-subscription' ? await getCodexSubscriptionStatus() : emptyCodexStatus
      if (generation !== repositoryGeneration.current) return
      setSettings(qa)
      setCodexStatus(codex)
    }).catch((cause) => setError(`问答工作区初始化失败：${String(cause)}`))
  }, [repositoryPath])

  const effectiveCodexModel = settings.codexModel || codexStatus.configuredModel
  const selectedCodexOption = codexStatus.availableModels.find((item) => item.id === effectiveCodexModel)
  const supportedEfforts = selectedCodexOption?.supportedReasoningEfforts ?? []
  const configuredEffortSupported = effectiveCodexModel === codexStatus.configuredModel
    && (!supportedEfforts.length || supportedEfforts.includes(codexStatus.configuredReasoningEffort))
  const automaticEffort = configuredEffortSupported
    ? codexStatus.configuredReasoningEffort || selectedCodexOption?.defaultReasoningEffort || ''
    : selectedCodexOption?.defaultReasoningEffort || ''

  const persistComposerSelection = (next: QaSettings) => {
    setSettings(next)
    if (!repositoryPath || !isDesktopRuntime()) return
    void saveQaSettings(next).catch((cause) => setError(`模型选择保存失败：${String(cause)}`))
  }

  useEffect(() => {
    if (!repositoryPath || !isDesktopRuntime()) return
    sessionListGeneration.current += 1
    const timer = window.setTimeout(() => {
      void replaceSessionPage(sessionQuery).catch((cause) => setError(`会话列表加载失败：${String(cause)}`))
    }, 180)
    return () => window.clearTimeout(timer)
  }, [repositoryPath, sessionQuery])

  useEffect(() => {
    const previousHeight = previousScrollHeight.current
    if (previousHeight !== null && messagesRef.current) {
      const container = messagesRef.current
      previousScrollHeight.current = null
      window.requestAnimationFrame(() => { container.scrollTop += container.scrollHeight - previousHeight })
      return
    }
    endRef.current?.scrollIntoView({ behavior: 'smooth', block: 'end' })
  }, [messages, streamingText, phase])

  const openSession = async (sessionId: string) => {
    if (phase !== 'idle') return
    const requestGeneration = ++sessionOpenGeneration.current
    const repositoryRequestGeneration = repositoryGeneration.current
    setLoadingHistory(true)
    setError('')
    try {
      const detail = await getChatSessionPage(sessionId, undefined, MESSAGE_PAGE_SIZE)
      if (requestGeneration !== sessionOpenGeneration.current || repositoryRequestGeneration !== repositoryGeneration.current) return
      setActiveSessionId(sessionId)
      setMessages(detail.messages)
      setMessageCursor(detail.nextCursor ?? '')
      setMessageHasMore(Boolean(detail.nextCursor))
      const latestAssistant = [...detail.messages].reverse().find((message) => message.role === 'assistant')
      const latestQuestion = [...detail.messages].reverse().find((message) => message.role === 'user')?.content ?? null
      onResearchContextChange(latestQuestion)
      setEvidence(latestAssistant?.evidence ?? [])
      setEvidenceRequestId(latestAssistant?.requestId ?? '')
      setRetrievalDiagnostics(null)
      setContextBudget(latestAssistant?.runManifest?.contextBudget ?? null)
      setRunManifest(latestAssistant?.runManifest ?? null)
      setWaterline(latestAssistant?.waterline ?? null)
      setSelectedEvidence(latestAssistant?.evidence?.[0] ?? null)
    } catch (cause) {
      if (requestGeneration === sessionOpenGeneration.current) setError(`会话打开失败：${String(cause)}`)
    } finally {
      if (requestGeneration === sessionOpenGeneration.current) setLoadingHistory(false)
    }
  }

  const loadOlderMessages = async () => {
    if (!activeSessionId || !messageCursor || loadingOlderMessages) return
    const requestGeneration = sessionOpenGeneration.current
    const requestedSessionId = activeSessionId
    setLoadingOlderMessages(true)
    try {
      const detail = await getChatSessionPage(requestedSessionId, messageCursor, MESSAGE_PAGE_SIZE)
      if (requestGeneration !== sessionOpenGeneration.current) return
      previousScrollHeight.current = messagesRef.current?.scrollHeight ?? null
      setMessages((current) => prependUniqueMessages(current, detail.messages))
      setMessageCursor(detail.nextCursor ?? '')
      setMessageHasMore(Boolean(detail.nextCursor))
    } catch (cause) { setError(`加载更早消息失败：${String(cause)}`) } finally { setLoadingOlderMessages(false) }
  }

  const newSession = () => {
    if (phase !== 'idle') return
    sessionOpenGeneration.current += 1
    setActiveSessionId('')
    setMessages([])
    setEvidence([])
    setEvidenceRequestId('')
    setRetrievalDiagnostics(null)
    setContextBudget(null)
    setRunManifest(null)
    setSelectedEvidence(null)
    setWaterline(null)
    setMessageCursor('')
    setMessageHasMore(false)
    setError('')
    onResearchContextChange(null)
  }

  const applyCompleted = (result: AskResult, generation = repositoryGeneration.current) => {
    if (generation !== repositoryGeneration.current) return
    if (!claimCompletion(completionLedger.current, repositoryPath ?? '', result.requestId)) return
    setActiveSessionId(result.sessionId)
    setMessages((current) => mergeCompletedMessages(current, result))
    setEvidence(result.evidence)
    setEvidenceRequestId(result.requestId)
    setRetrievalDiagnostics(result.retrievalDiagnostics)
    setContextBudget(result.contextBudget)
    setRunManifest(result.runManifest)
    setSelectedEvidence(result.evidence[0] ?? null)
    setWaterline(result.waterline)
    setStreamingText('')
    setRequestId('')
    activeRequestId.current = ''
    resetGenerationState()
    void refreshSessions()
  }

  const handleEvent = (event: AnswerStreamEvent, generation: number, optimisticId: string, originalSessionId: string) => {
    if (generation !== repositoryGeneration.current) return
    if (event.type === 'started') {
      setRequestId(event.payload.requestId)
      activeRequestId.current = event.payload.requestId
      setActiveSessionId(event.payload.sessionId)
    } else if (event.type === 'retrieval_started') {
      setPhase('retrieving')
    } else if (event.type === 'retrieval_completed') {
      setEvidence(event.payload.evidence)
      setEvidenceRequestId(event.payload.requestId)
      setRetrievalDiagnostics(event.payload.retrievalDiagnostics)
      setContextBudget(event.payload.contextBudget)
      setSelectedEvidence(event.payload.evidence[0] ?? null)
      setWaterline(event.payload.waterline)
      setPhase('generating')
    } else if (event.type === 'token') {
      setHasFirstToken(true)
      setStreamingText((current) => current + event.payload.content)
    } else if (event.type === 'validation_started') {
      setPhase('validating')
    } else if (event.type === 'completed') {
      applyCompleted(event.payload.result, generation)
    } else if (event.type === 'failed') {
      setError(`${event.payload.code}：${event.payload.message}`)
      if (event.payload.exchange) {
        setMessages((current) => mergeFailedMessages(current, optimisticId, event.payload.exchange!))
        setActiveSessionId(event.payload.exchange.sessionId)
      } else {
        setMessages((current) => rollbackOptimisticMessages(current, optimisticId))
        setActiveSessionId(originalSessionId)
      }
      setContextBudget(null)
      setRunManifest(null)
      resetGenerationState()
      setStreamingText('')
      setRequestId('')
      activeRequestId.current = ''
      void refreshSessions()
    } else if (event.type === 'cancelled') {
      setError('本轮问答已停止，未写入会话历史。')
      setMessages((current) => rollbackOptimisticMessages(current, optimisticId))
      setContextBudget(null)
      setRunManifest(null)
      resetGenerationState()
      setStreamingText('')
      setRequestId('')
      activeRequestId.current = ''
      setActiveSessionId(originalSessionId)
    }
  }

  const submitQuestion = async (preset?: string) => {
    const value = (preset ?? question).trim()
    if (!value || phase !== 'idle') return
    if (!isDesktopRuntime()) { setError('智能问答需要在 Windows 桌面客户端中运行。'); return }
    if (!repositoryPath) { setError('请先选择知识库并建立索引。'); return }
    setQuestion('')
    setError('')
    setStreamingText('')
    generationStartedAt.current = performance.now()
    setElapsedSeconds(0)
    setHasFirstToken(false)
    setPhase('retrieving')
    const generation = repositoryGeneration.current
    const originalSessionId = activeSessionId
    const clientRequestId = crypto.randomUUID()
    setRequestId(clientRequestId)
    activeRequestId.current = clientRequestId
    const optimistic = localMessage('user', value, 'retrieving', clientRequestId)
    let terminalEventHandled = false
    setMessages((current) => [...current, optimistic])
    onResearchContextChange(value)
    try {
      const result = await askLuna({ requestId: clientRequestId, question: value, sessionId: originalSessionId || undefined, evidenceLimit: 14, repositoryId: repositoryIdentity(repositoryPath), codexModel: settings.codexModel, codexReasoningEffort: settings.codexReasoningEffort }, (event) => {
        if (event.type === 'failed' || event.type === 'cancelled') terminalEventHandled = true
        handleEvent(event, generation, optimistic.id, originalSessionId)
      })
      applyCompleted(result, generation)
    } catch (cause) {
      if (generation !== repositoryGeneration.current) return
      if (terminalEventHandled) return
      if (!terminalEventHandled && !String(cause).includes('已取消')) setError(`问答执行失败：${String(cause)}`)
      setMessages((current) => rollbackOptimisticMessages(current, optimistic.id))
      setStreamingText('')
      resetGenerationState()
      setRequestId('')
      activeRequestId.current = ''
      setActiveSessionId(originalSessionId)
    }
  }

  const stopAnswer = async () => {
    if (!requestId) return
    try { await cancelAnswer(requestId) } catch (cause) { setError(`停止失败：${String(cause)}`) }
  }

  const openEvidence = (item: EvidenceItem) => {
    setSelectedEvidence(item)
    if (item.locator) onOpenSource(item.locator, item.title)
    else if ((item.kind === 'wiki' || item.kind === 'graph') && item.pageId) onOpenPage(item.pageId, item.title)
    else if (item.kind === 'paper' && item.sourcePath) onOpenPath(item.sourcePath)
    else if (item.kind === 'book' && item.bookId && item.chapterId) onOpenBook(item.bookId, item.chapterId)
    else if (item.kind === 'book' && item.pdfPath) onOpenPath(item.pdfPath)
    else if (item.sourcePath) onOpenPage(item.sourcePath, item.title)
  }

  const removeSession = async (session: ChatSessionSummary) => {
    if (!window.confirm(`删除会话“${session.title}”？`)) return
    try {
      await deleteChatSession(session.id)
      if (activeSessionId === session.id) newSession()
      await refreshSessions()
    } catch (cause) { setError(`删除会话失败：${String(cause)}`) }
  }

  const editSessionTitle = async (session: ChatSessionSummary) => {
    const title = window.prompt('会话标题', session.title)?.trim()
    if (!title || title === session.title) return
    try { await renameChatSession(session.id, title); await refreshSessions() } catch (cause) { setError(`重命名失败：${String(cause)}`) }
  }

  const emptyEvidence = evidenceEmptyState(phase, waterline, evidence.length)
  const evidenceOwnership = evidencePanelOwnership(phase, evidenceRequestId, requestId, evidence.length)
  const activeThinkingStep = phase === 'retrieving'
    ? 1
    : phase === 'generating' && !hasFirstToken
      ? 3
      : phase === 'generating'
        ? 4
        : phase === 'validating'
          ? 5
          : -1
  const thinkingSteps = [
    '理解问题',
    '检索本地知识库',
    '整理证据上下文',
    'Thinking',
    '生成回答',
    '回答与证据校验',
  ].map((label, index) => ({
    label,
    state: index < activeThinkingStep ? 'done' : index === activeThinkingStep ? 'active' : 'waiting',
  }))
  const thinkingStage = phase === 'retrieving'
    ? { title: '正在检索知识库', detail: '正在匹配 Wiki、论文原文、专著与知识图谱。' }
    : phase === 'generating' && !hasFirstToken
      ? { title: '模型正在组织回答', detail: '正在基于本轮证据组织自然科研回答。' }
      : phase === 'generating'
        ? { title: '正在生成回答', detail: '正在接收自然 Markdown 回答。' }
        : { title: '正在检查回答与证据', detail: '正在清理不受信链接并追加真实证据。' }

  return <section className="qa-view">
    <aside className="qa-sessions">
      <div className="qa-session-heading"><div><strong>研究会话</strong></div><button className="qa-icon-button" onClick={newSession} title="新建会话"><MessageSquarePlus size={17} /></button></div>
      <label className="qa-session-search"><Search size={14} /><input value={sessionQuery} onChange={(event) => setSessionQuery(event.target.value)} placeholder="搜索会话…" />{sessionQuery && <button title="清除搜索" onClick={() => setSessionQuery('')}><X size={13} /></button>}</label>
      <div className="qa-session-list">
        {sessions.map((session) => <div className={`qa-session-item ${activeSessionId === session.id ? 'active' : ''}`} key={session.id}>
          <button className="qa-session-main" onClick={() => void openSession(session.id)}><strong>{session.title}</strong><span>{session.lastMessagePreview || '空会话'}</span><small>{formatTime(session.updatedAt)} · {session.messageCount} 条</small></button>
          <button className="qa-session-more" onClick={() => void editSessionTitle(session)} title="重命名"><MoreHorizontal size={14} /></button>
          <button className="qa-session-delete" onClick={() => void removeSession(session)} title="删除"><Trash2 size={13} /></button>
        </div>)}
        {!sessions.length && !sessionQuery.trim() && <div className="qa-empty-history">尚无历史会话。提出第一个问题后会自动保存。</div>}
        {!sessions.length && !!sessionQuery.trim() && <div className="qa-empty-history">没有匹配的历史会话。</div>}
        {sessionHasMore && <button className="qa-load-more" onClick={() => void loadMoreSessions()} disabled={loadingMoreSessions}>{loadingMoreSessions ? '加载中…' : '加载更多会话'}</button>}
      </div>
    </aside>

    <main className="qa-chat">
      <div className="qa-chat-heading"><div><h1>智能问答</h1></div><div className="qa-model-state"><span className={providerReady(settings.answerProvider, settings, codexStatus) ? 'ready' : 'offline'}>{providerReady(settings.answerProvider, settings, codexStatus) ? <Check size={13} /> : <ShieldCheck size={13} />}{providerLabel(settings.answerProvider)} · {providerReady(settings.answerProvider, settings, codexStatus) ? '已就绪' : '未就绪'}</span><button className="qa-icon-button" data-testid="qa-open-settings" onClick={onOpenSettings} title="前往设置"><Settings size={17} /></button></div></div>
      {error && <div className="qa-error"><span>{error}</span><button onClick={() => setError('')}><X size={14} /></button></div>}
      <div className="qa-messages" ref={messagesRef}>
        {loadingHistory && <div className="qa-loading"><LoaderCircle size={18} className="spin" />加载会话历史…</div>}
        {messageHasMore && <button className="qa-load-older" onClick={() => void loadOlderMessages()} disabled={loadingOlderMessages}>{loadingOlderMessages ? '加载中…' : '加载更早消息'}</button>}
        {!messages.length && phase === 'idle' && <div className="qa-welcome"><div className="qa-orb"><Bot size={28} /></div><h2>先检索，再回答</h2><p>每次提问都会检索 Wiki、两本核心书籍和 Graphify，并把回答绑定到可定位证据。</p><div className="qa-suggestions">{suggestions.map((item) => <button key={item} onClick={() => void submitQuestion(item)}><Plus size={14} /><span>{item}</span><ChevronRight size={14} /></button>)}</div></div>}
          {messages.map((message, index) => { const retryQuestion = message.role === 'assistant' ? retryQuestionFor(messages, index) : ''; return <article data-testid={`qa-message-${message.id}`} className={`qa-message ${message.role} ${message.status}`} key={message.id}><div className="qa-avatar">{message.role === 'assistant' ? <Bot size={16} /> : '你'}</div><div className="qa-bubble"><div className="qa-message-meta"><strong>{message.role === 'assistant' ? providerLabel(message.provider) : '研究问题'}</strong><span>{message.status === 'failed' ? '失败' : message.status === 'unverified' ? '无参考来源 · 未验证' : message.status === 'mixed' ? '含模型补充' : formatTime(message.createdAt)}</span></div>{message.role === 'assistant' && message.status === 'failed' ? <div className="qa-message-content">{message.errorCode}：{message.errorMessage || '本轮回答生成失败'}</div> : message.role === 'assistant' ? <MessageContent content={message.content} evidence={message.evidence} onCitation={openEvidence} /> : <div className="qa-message-content">{message.content}</div>}{message.role === 'assistant' && message.status !== 'failed' && <CitationStatus message={message} />}{message.role === 'assistant' && <div className="qa-message-actions"><button onClick={() => void navigator.clipboard.writeText(message.content)}><Clipboard size={13} />复制回答</button><button onClick={() => void navigator.clipboard.writeText(buildAuditBundle(retryQuestion, message))}><Clipboard size={13} />复制审计包</button><button onClick={() => void submitQuestion(retryQuestion)} disabled={!retryQuestion || phase !== 'idle'}><RefreshCw size={13} />重试</button></div>}</div></article> })}
        {phase !== 'idle' && (
          <article className="qa-message assistant streaming">
            <div className="qa-avatar"><Bot size={16} /></div>
            <div className="qa-bubble">
              <div className="qa-message-meta">
                <strong>Thinking · {elapsedSeconds}s</strong>
                <span><LoaderCircle size={13} className="spin" aria-hidden="true" /></span>
              </div>
              <div className="qa-thinking-loader">
                <div className="qa-thinking-signal" aria-hidden="true"><LoaderCircle size={24} className="qa-thinking-spinner" /></div>
                <div className="qa-thinking-current" role="status" aria-live="polite" aria-atomic="true">
                  <strong>{thinkingStage.title}</strong>
                  <span className="qa-thinking-dots" aria-hidden="true"><i /><i /><i /></span>
                  <small>{thinkingStage.detail}</small>
                </div>
                <time aria-hidden="true">{elapsedSeconds}s</time>
                <div className="qa-thinking-flow" aria-hidden="true"><i /></div>
              </div>
              <div className="qa-thinking-chain" aria-label="回答处理进度">
                {thinkingSteps.map((step) => (
                  <div className={step.state} data-state={step.state} key={step.label}>
                    {step.state === 'done'
                      ? <CheckCircle2 size={13} />
                      : step.state === 'active'
                        ? <LoaderCircle size={14} className="qa-thinking-spinner compact" aria-hidden="true" />
                        : <span className="qa-step-dot" aria-hidden="true" />}
                    <span>{step.label}</span>
                  </div>
                ))}
              </div>
              {streamingText && (
                <div className="qa-natural-stream"><MessageContent content={streamingText} evidence={evidence} onCitation={openEvidence} /><span className="qa-stream-cursor" aria-hidden="true" /></div>
              )}
            </div>
          </article>
        )}
        <div ref={endRef} />
      </div>
      <div className="qa-composer"><textarea ref={composerRef} data-testid="qa-input" value={question} onChange={(event) => setQuestion(event.target.value)} onKeyDown={(event) => { if (event.key === 'Enter' && !event.shiftKey) { event.preventDefault(); void submitQuestion() } }} placeholder="询问模型、约束、算法、解决办法或新颖性…" rows={3} disabled={phase !== 'idle'} /><div className="qa-composer-footer"><div className="qa-composer-controls"><span className={providerReady(settings.answerProvider, settings, codexStatus) ? 'ready' : 'offline'}><ShieldCheck size={12} />{providerLabel(settings.answerProvider)}</span>{settings.answerProvider === 'codex-subscription' && <><label title="选择本轮 Codex 模型"><span className="sr-only">Codex 模型</span><select aria-label="Codex 模型" disabled={phase !== 'idle'} value={settings.codexModel} onChange={(event) => persistComposerSelection({ ...settings, codexModel: event.target.value, codexReasoningEffort: '' })}><option value="">自动 · {codexStatus.configuredModel || 'Codex 默认'}</option>{codexStatus.availableModels.map((model) => <option key={model.id} value={model.id}>{model.displayName}</option>)}</select></label><label title="选择本轮推理强度"><span className="sr-only">推理强度</span><select aria-label="推理强度" disabled={phase !== 'idle'} value={settings.codexReasoningEffort} onChange={(event) => persistComposerSelection({ ...settings, codexReasoningEffort: event.target.value })}><option value="">自动 · {effortLabels[automaticEffort] || automaticEffort || '模型默认'}</option>{supportedEfforts.map((effort) => <option key={effort} value={effort}>{effortLabels[effort] || effort}</option>)}</select></label></>}</div>{phase === 'idle' ? <button className="qa-send" onClick={() => void submitQuestion()} disabled={!question.trim()}><Send size={15} />发送</button> : <button className="qa-stop" onClick={() => void stopAnswer()} disabled={!requestId}><CircleStop size={15} />停止</button>}</div></div>
    </main>

    <aside className="qa-evidence-panel">
      <div className="qa-evidence-heading"><div><strong>{evidenceOwnership === 'previous-during-retrieval' ? '上一轮证据' : '本轮证据'}</strong>{evidenceOwnership === 'previous-during-retrieval' && <small>仅供查看 · 不参与本轮回答</small>}</div><span>{evidence.length}</span></div>
      {waterline && <div className="qa-waterline"><strong>库水位</strong><div><span>{waterline.sourceCount}<small>source</small></span><span>{waterline.methodCount}<small>method</small></span><span>{waterline.synthesisCount}<small>synthesis</small></span><span>{waterline.chapterCount}<small>chapters</small></span></div><p>{waterline.yearMin || '未知'}–{waterline.yearMax || '未知'} · 当前仓库</p></div>}
      {retrievalDiagnostics && <div className="qa-retrieval-diagnostics"><div><strong>检索诊断</strong><span>{retrievalDiagnostics.totalMs} ms · {retrievalDiagnostics.passCount || 1} 轮 · 选中 {retrievalDiagnostics.selectedCount}</span></div><p>{retrievalDiagnostics.channels.map((channel) => `${channel.name} ${channel.candidateCount}/${channel.durationMs}ms`).join(' · ')}</p><small>停止：{retrievalDiagnostics.stopReason || '单轮完成'} · 增益 {(retrievalDiagnostics.candidateGains ?? []).join('/')} · 取消检查点 {retrievalDiagnostics.cancelCheckCount}</small></div>}
      {contextBudget && <div className="qa-context-budget"><div><strong>上下文预算</strong><span>{contextBudget.estimatedTotalTokens}/{contextBudget.inputBudgetTokens}</span></div><p>契约 {contextBudget.researchContractTokens} · 记忆 {contextBudget.sessionMemoryTokens} · 近期 {contextBudget.recentHistoryTokens} · 问题 {contextBudget.currentQueryTokens} · 证据 {contextBudget.evidenceTokens} · 序列化 {contextBudget.serializationOverheadTokens}</p><small>输出预留 {contextBudget.outputReserveTokens} · 空余 {contextBudget.freeTokens} · 最近 {contextBudget.recentExchangeCount} 轮 · 压缩 {contextBudget.compactedMessageCount} 条{contextBudget.truncated ? ' · 已裁剪' : ''}</small>{runManifest && <><small>快照 {runManifest.indexSnapshotId.slice(0, 19)}… · {runManifest.promptVersion}/{runManifest.answerSchemaVersion} · 回答{runManifest.answerCompleteness.complete ? '通过' : '未通过'}</small><small>规划 {runManifest.planningProvider || 'deterministic'} · {(runManifest.providerCapabilities ?? []).join(', ') || '无模型规划能力'} · 路由置信度 {runManifest.routingConfidence || 'unknown'}{runManifest.resolverEscalated ? ' · 已升级理解' : ''}</small><small>方法：证据发现 {(runManifest.discoveredMethods ?? []).length} · 规则假设 {(runManifest.methodHypotheses ?? []).length} · 已佐证 {(runManifest.corroboratedMethodHypotheses ?? []).length}</small><small>LLM 调用 {runManifest.routingLlmCallsUsed ?? 0}/{runManifest.routingLlmCallBudget ?? 0} · token 成本 {runManifest.routingTokenCostUsed ?? 0}/{runManifest.routingTokenCostCeiling ?? 0}{runManifest.routingBudgetRejections?.length ? ` · 拒绝 ${runManifest.routingBudgetRejections.join(', ')}` : ''}</small></>}</div>}
      {Boolean(runManifest?.claimVerifications?.length) && <details className="qa-claim-audit"><summary>逐条证据审计 <span>{runManifest?.claimVerifications?.length}</span></summary><div>{runManifest?.claimVerifications?.map((claim) => <article key={claim.id} className={claim.verificationStatus}><div><strong>{claim.id}</strong><span>{verificationStatusLabel(claim.verificationStatus)}</span></div><p>{claim.text}</p><small>{claim.claimType} · {claim.evidenceIds.length ? claim.evidenceIds.join(', ') : '无显式证据'} · {claim.verificationMethod}</small></article>)}</div><footer>启发式核验记录不等同于模型语义蕴含判断。</footer></details>}
      <div className="qa-evidence-list">{evidence.map((item) => <button className={`qa-evidence-card ${selectedEvidence?.id === item.id ? 'selected' : ''}`} key={item.id} onClick={() => setSelectedEvidence(item)}><span className="qa-evidence-id">{item.id}</span><div><div className="qa-evidence-type">{kindIcon(item.kind)}<span>{tierLabel(item.tier)}</span></div><strong>{item.title}</strong><p>{item.snippet}</p><small>{evidenceLocationLabel(item)}</small></div></button>)}</div>
      {emptyEvidence && <div className={`qa-empty-evidence ${emptyEvidence.kind}`}><FileText size={23} /><strong>{emptyEvidence.title}</strong><span>{emptyEvidence.detail}</span></div>}
      {selectedEvidence && <div className="qa-evidence-detail" id={`evidence-${selectedEvidence.id}`}><div><strong>{selectedEvidence.id} · 定位信息</strong><button onClick={() => setSelectedEvidence(null)}><X size={13} /></button></div><p>{selectedEvidence.retrievalReason}</p><a href={`#open-${selectedEvidence.id}`} className="qa-open-source" onClick={(event) => { event.preventDefault(); openEvidence(selectedEvidence) }}>{kindIcon(selectedEvidence.kind)}打开{selectedEvidence.kind === 'paper' ? '论文原文' : selectedEvidence.kind === 'book' ? '书籍来源' : '知识库来源'}</a></div>}
    </aside>

  </section>
}
