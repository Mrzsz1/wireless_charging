import { useEffect, useMemo, useRef, useState } from 'react'
import { BookOpen, Bot, Check, ChevronRight, CircleStop, Clipboard, FileText, GitBranch, LoaderCircle, MessageSquarePlus, MoreHorizontal, Plus, RefreshCw, Search, Send, Settings, ShieldCheck, Trash2, X } from 'lucide-react'
import { askLuna, cancelAnswer, deleteChatSession, getChatSession, getLunaSettings, isDesktopRuntime, listChatSessions, renameChatSession, saveLunaSettings } from '../../services/desktop'
import type { AnswerStreamEvent, AskResult, ChatMessage, ChatSessionSummary, EvidenceItem, LunaSettings, WaterlineSnapshot } from '../../types'
import { claimCompletion, createCompletionLedger, mergeCompletedMessages } from './completionState'
import './AskView.css'

type AskViewProps = {
  repositoryPath?: string
  onOpenPage: (pageId: string, title?: string) => void
  onOpenBook: (bookId: string, chapterId: string) => void
  onOpenPath: (path: string, reveal?: boolean) => void
}

const emptySettings: LunaSettings = {
  endpoint: '',
  model: 'gpt-5.6-luna',
  apiKeyEnv: 'LUNA_API_KEY',
  timeoutSeconds: 90,
  maxOutputTokens: 1800,
  temperature: 0.1,
  apiKeyConfigured: false,
}

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

function localMessage(role: 'user' | 'assistant', content: string, status: ChatMessage['status']): ChatMessage {
  return { id: `local-${Date.now()}-${role}`, sessionId: '', role, content, status, createdAt: String(Date.now()), errorCode: '', errorMessage: '', provider: role === 'assistant' ? 'pending' : 'local', model: '', requestId: '', evidence: [] }
}

function tierLabel(tier: EvidenceItem['tier']) {
  return tier === 'direct' ? '直接证据' : tier === 'similar_model' ? '相似模型' : tier === 'transferable_method' ? '可迁移算法' : tier === 'theory' ? '理论基础' : '图谱提示'
}

function kindIcon(kind: EvidenceItem['kind']) {
  return kind === 'book' ? <BookOpen size={15} /> : kind === 'graph' ? <GitBranch size={15} /> : <FileText size={15} />
}

function MessageContent({ content, evidence, onCitation }: { content: string; evidence: EvidenceItem[]; onCitation: (item: EvidenceItem) => void }) {
  const byId = useMemo(() => new Map(evidence.map((item) => [item.id, item])), [evidence])
  return <div className="qa-message-content">{content.split(/(\[E\d+\])/g).map((part, index) => {
    const match = /^\[(E\d+)\]$/.exec(part)
    if (!match) return <span key={`${index}-${part.slice(0, 8)}`}>{part}</span>
    const item = byId.get(match[1])
    return item ? <button key={`${index}-${part}`} className="qa-inline-citation" onClick={() => onCitation(item)}>{part}</button> : <span key={`${index}-${part}`} className="qa-invalid-citation" title="回答引用了未登记的证据">{part}</span>
  })}</div>
}

export function AskView({ repositoryPath, onOpenPage, onOpenBook, onOpenPath }: AskViewProps) {
  const [sessions, setSessions] = useState<ChatSessionSummary[]>([])
  const [sessionQuery, setSessionQuery] = useState('')
  const [activeSessionId, setActiveSessionId] = useState('')
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const [question, setQuestion] = useState('')
  const [phase, setPhase] = useState<'idle' | 'retrieving' | 'generating'>('idle')
  const [streamingText, setStreamingText] = useState('')
  const [requestId, setRequestId] = useState('')
  const [evidence, setEvidence] = useState<EvidenceItem[]>([])
  const [selectedEvidence, setSelectedEvidence] = useState<EvidenceItem | null>(null)
  const [waterline, setWaterline] = useState<WaterlineSnapshot | null>(null)
  const [settings, setSettings] = useState<LunaSettings>(emptySettings)
  const [settingsDraft, setSettingsDraft] = useState<LunaSettings>(emptySettings)
  const [settingsOpen, setSettingsOpen] = useState(false)
  const [loadingHistory, setLoadingHistory] = useState(false)
  const [error, setError] = useState('')
  const endRef = useRef<HTMLDivElement>(null)
  const completionLedger = useRef(createCompletionLedger())

  const refreshSessions = async () => {
    if (!isDesktopRuntime() || !repositoryPath) return
    try { setSessions(await listChatSessions()) } catch (cause) { setError(`会话列表加载失败：${String(cause)}`) }
  }

  useEffect(() => {
    completionLedger.current = createCompletionLedger(repositoryPath ?? '')
    setActiveSessionId('')
    setMessages([])
    setEvidence([])
    setWaterline(null)
    if (!isDesktopRuntime() || !repositoryPath) return
    void Promise.all([listChatSessions(), getLunaSettings()]).then(([history, luna]) => {
      setSessions(history)
      setSettings(luna)
      setSettingsDraft(luna)
    }).catch((cause) => setError(`问答工作区初始化失败：${String(cause)}`))
  }, [repositoryPath])

  useEffect(() => { endRef.current?.scrollIntoView({ behavior: 'smooth', block: 'end' }) }, [messages, streamingText, phase])

  const openSession = async (sessionId: string) => {
    if (phase !== 'idle') return
    setLoadingHistory(true)
    setError('')
    try {
      const detail = await getChatSession(sessionId)
      setActiveSessionId(sessionId)
      setMessages(detail.messages)
      const latestAssistant = [...detail.messages].reverse().find((message) => message.role === 'assistant')
      setEvidence(latestAssistant?.evidence ?? [])
      setWaterline(latestAssistant?.waterline ?? null)
      setSelectedEvidence(latestAssistant?.evidence?.[0] ?? null)
    } catch (cause) { setError(`会话打开失败：${String(cause)}`) } finally { setLoadingHistory(false) }
  }

  const newSession = () => {
    if (phase !== 'idle') return
    setActiveSessionId('')
    setMessages([])
    setEvidence([])
    setSelectedEvidence(null)
    setWaterline(null)
    setError('')
  }

  const applyCompleted = (result: AskResult) => {
    if (!claimCompletion(completionLedger.current, repositoryPath ?? '', result.requestId)) return
    setActiveSessionId(result.sessionId)
    setMessages((current) => mergeCompletedMessages(current, result))
    setEvidence(result.evidence)
    setSelectedEvidence(result.evidence[0] ?? null)
    setWaterline(result.waterline)
    setStreamingText('')
    setRequestId('')
    setPhase('idle')
    void refreshSessions()
  }

  const handleEvent = (event: AnswerStreamEvent) => {
    if (event.type === 'started') {
      setRequestId(event.payload.requestId)
      setActiveSessionId(event.payload.sessionId)
    } else if (event.type === 'retrieval_started') {
      setPhase('retrieving')
    } else if (event.type === 'retrieval_completed') {
      setEvidence(event.payload.evidence)
      setSelectedEvidence(event.payload.evidence[0] ?? null)
      setWaterline(event.payload.waterline)
      setPhase('generating')
    } else if (event.type === 'token') {
      setStreamingText((current) => current + event.payload.content)
    } else if (event.type === 'completed') {
      applyCompleted(event.payload.result)
    } else if (event.type === 'failed') {
      setError(`${event.payload.code}：${event.payload.message}`)
      setPhase('idle')
    } else if (event.type === 'cancelled') {
      setError('本轮问答已停止，未写入会话历史。')
      setPhase('idle')
      setRequestId('')
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
    setEvidence([])
    setSelectedEvidence(null)
    setPhase('retrieving')
    setMessages((current) => [...current, localMessage('user', value, 'retrieving')])
    try {
      const result = await askLuna({ question: value, sessionId: activeSessionId || undefined, evidenceLimit: 14 }, handleEvent)
      applyCompleted(result)
    } catch (cause) {
      if (!String(cause).includes('已取消')) setError(`问答执行失败：${String(cause)}`)
      setPhase('idle')
      setRequestId('')
    }
  }

  const stopAnswer = async () => {
    if (!requestId) return
    try { await cancelAnswer(requestId) } catch (cause) { setError(`停止失败：${String(cause)}`) }
  }

  const openEvidence = (item: EvidenceItem) => {
    setSelectedEvidence(item)
    if (item.kind === 'wiki' && item.pageId) onOpenPage(item.pageId, item.title)
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

  const persistSettings = async () => {
    try {
      const saved = await saveLunaSettings(settingsDraft)
      setSettings(saved)
      setSettingsDraft(saved)
      setSettingsOpen(false)
      setError('')
    } catch (cause) { setError(`设置保存失败：${String(cause)}`) }
  }

  const lastUserQuestion = [...messages].reverse().find((message) => message.role === 'user')?.content ?? ''
  const filteredSessions = useMemo(() => {
    const keyword = sessionQuery.trim().toLocaleLowerCase('zh-CN')
    if (!keyword) return sessions
    return sessions.filter((session) => `${session.title}\n${session.lastMessagePreview}`.toLocaleLowerCase('zh-CN').includes(keyword))
  }, [sessionQuery, sessions])

  return <section className="qa-view">
    <aside className="qa-sessions">
      <div className="qa-session-heading"><div><div className="eyebrow">CONVERSATIONS</div><strong>研究会话</strong></div><button className="qa-icon-button" onClick={newSession} title="新建会话"><MessageSquarePlus size={17} /></button></div>
      <label className="qa-session-search"><Search size={14} /><input value={sessionQuery} onChange={(event) => setSessionQuery(event.target.value)} placeholder="搜索会话…" />{sessionQuery && <button title="清除搜索" onClick={() => setSessionQuery('')}><X size={13} /></button>}</label>
      <div className="qa-session-list">
        {filteredSessions.map((session) => <div className={`qa-session-item ${activeSessionId === session.id ? 'active' : ''}`} key={session.id}>
          <button className="qa-session-main" onClick={() => void openSession(session.id)}><strong>{session.title}</strong><span>{session.lastMessagePreview || '空会话'}</span><small>{formatTime(session.updatedAt)} · {session.messageCount} 条</small></button>
          <button className="qa-session-more" onClick={() => void editSessionTitle(session)} title="重命名"><MoreHorizontal size={14} /></button>
          <button className="qa-session-delete" onClick={() => void removeSession(session)} title="删除"><Trash2 size={13} /></button>
        </div>)}
        {!sessions.length && <div className="qa-empty-history">尚无历史会话。提出第一个问题后会自动保存。</div>}
        {!!sessions.length && !filteredSessions.length && <div className="qa-empty-history">没有匹配的历史会话。</div>}
      </div>
    </aside>

    <main className="qa-chat">
      <div className="qa-chat-heading"><div><div className="eyebrow">LUNA · EVIDENCE FIRST</div><h1>智能问答</h1></div><div className="qa-model-state"><span className={settings.apiKeyConfigured && settings.endpoint ? 'ready' : 'offline'}>{settings.apiKeyConfigured && settings.endpoint ? <Check size={13} /> : <ShieldCheck size={13} />}{settings.apiKeyConfigured && settings.endpoint ? `${settings.model} 已配置` : '离线证据模式'}</span><button className="qa-icon-button" onClick={() => setSettingsOpen(true)} title="Luna 设置"><Settings size={17} /></button></div></div>
      {error && <div className="qa-error"><span>{error}</span><button onClick={() => setError('')}><X size={14} /></button></div>}
      <div className="qa-messages">
        {loadingHistory && <div className="qa-loading"><LoaderCircle size={18} className="spin" />加载会话历史…</div>}
        {!messages.length && phase === 'idle' && <div className="qa-welcome"><div className="qa-orb"><Bot size={28} /></div><h2>先检索，再回答</h2><p>每次提问都会检索 Wiki、两本核心书籍和 Graphify，并把回答绑定到可定位证据。</p><div className="qa-suggestions">{suggestions.map((item) => <button key={item} onClick={() => void submitQuestion(item)}><Plus size={14} /><span>{item}</span><ChevronRight size={14} /></button>)}</div></div>}
        {messages.map((message) => <article data-testid={`qa-message-${message.id}`} className={`qa-message ${message.role}`} key={message.id}><div className="qa-avatar">{message.role === 'assistant' ? <Bot size={16} /> : '你'}</div><div className="qa-bubble"><div className="qa-message-meta"><strong>{message.role === 'assistant' ? (message.provider === 'offline-evidence' ? '离线证据' : 'Luna') : '研究问题'}</strong><span>{formatTime(message.createdAt)}</span></div>{message.role === 'assistant' ? <MessageContent content={message.content} evidence={message.evidence} onCitation={setSelectedEvidence} /> : <div className="qa-message-content">{message.content}</div>}{message.role === 'assistant' && <div className="qa-message-actions"><button onClick={() => void navigator.clipboard.writeText(message.content)}><Clipboard size={13} />复制</button><button onClick={() => void submitQuestion(lastUserQuestion)} disabled={!lastUserQuestion || phase !== 'idle'}><RefreshCw size={13} />重试</button></div>}</div></article>)}
        {phase !== 'idle' && <article className="qa-message assistant streaming"><div className="qa-avatar"><Bot size={16} /></div><div className="qa-bubble"><div className="qa-message-meta"><strong>{phase === 'retrieving' ? '正在检索证据' : '正在组织回答'}</strong><span><LoaderCircle size={13} className="spin" /></span></div>{streamingText ? <MessageContent content={streamingText} evidence={evidence} onCitation={setSelectedEvidence} /> : <div className="qa-retrieval-steps"><span className="active">Wiki FTS5</span><span className={phase === 'generating' ? 'active' : ''}>核心书籍</span><span className={phase === 'generating' ? 'active' : ''}>Graphify</span></div>}</div></article>}
        <div ref={endRef} />
      </div>
      <div className="qa-composer"><textarea data-testid="qa-input" value={question} onChange={(event) => setQuestion(event.target.value)} onKeyDown={(event) => { if (event.key === 'Enter' && !event.shiftKey) { event.preventDefault(); void submitQuestion() } }} placeholder="询问模型、约束、算法、解决办法或新颖性…" rows={3} disabled={phase !== 'idle'} /><div className="qa-composer-footer"><span>Enter 发送 · Shift+Enter 换行 · 默认不外搜</span>{phase === 'idle' ? <button className="qa-send" onClick={() => void submitQuestion()} disabled={!question.trim()}><Send size={15} />发送</button> : <button className="qa-stop" onClick={() => void stopAnswer()} disabled={!requestId}><CircleStop size={15} />停止</button>}</div></div>
    </main>

    <aside className="qa-evidence-panel">
      <div className="qa-evidence-heading"><div><div className="eyebrow">AUDIT TRAIL</div><strong>本轮证据</strong></div><span>{evidence.length}</span></div>
      {waterline && <div className="qa-waterline"><strong>库水位</strong><div><span>{waterline.sourceCount}<small>source</small></span><span>{waterline.methodCount}<small>method</small></span><span>{waterline.synthesisCount}<small>synthesis</small></span><span>{waterline.chapterCount}<small>chapters</small></span></div><p>{waterline.yearMin || '未知'}–{waterline.yearMax || '未知'} · 当前仓库</p></div>}
      <div className="qa-evidence-list">{evidence.map((item) => <button className={`qa-evidence-card ${selectedEvidence?.id === item.id ? 'selected' : ''}`} key={item.id} onClick={() => setSelectedEvidence(item)}><span className="qa-evidence-id">{item.id}</span><div><div className="qa-evidence-type">{kindIcon(item.kind)}<span>{tierLabel(item.tier)}</span></div><strong>{item.title}</strong><p>{item.snippet}</p><small>{item.kind === 'book' ? `PDF p.${item.physicalPageStart ?? '?'}–${item.physicalPageEnd ?? '?'}` : item.wikilink || item.sourceLocation || item.sourcePath}</small></div></button>)}</div>
      {!evidence.length && <div className="qa-empty-evidence"><FileText size={23} /><strong>等待检索</strong><span>提问后在这里核验引用、页码和排序理由。</span></div>}
      {selectedEvidence && <div className="qa-evidence-detail"><div><strong>{selectedEvidence.id} · 定位信息</strong><button onClick={() => setSelectedEvidence(null)}><X size={13} /></button></div><p>{selectedEvidence.retrievalReason}</p><code>{selectedEvidence.kind === 'book' ? selectedEvidence.markdownPath : selectedEvidence.sourcePath}</code><button className="qa-open-source" onClick={() => openEvidence(selectedEvidence)}>{kindIcon(selectedEvidence.kind)}打开来源</button></div>}
    </aside>

    {settingsOpen && <div className="qa-settings-backdrop" onMouseDown={(event) => { if (event.target === event.currentTarget) setSettingsOpen(false) }}><div className="qa-settings-dialog"><div className="qa-settings-title"><div><div className="eyebrow">MODEL CONNECTION</div><h2>Luna 设置</h2></div><button onClick={() => setSettingsOpen(false)}><X size={17} /></button></div><p className="qa-settings-note">API Key 只从进程环境变量读取，不写入 SQLite、日志或前端持久状态。</p><label>Chat Completions endpoint<input value={settingsDraft.endpoint} onChange={(event) => setSettingsDraft({ ...settingsDraft, endpoint: event.target.value })} placeholder="https://HOST/v1/chat/completions" /></label><label>模型<input value={settingsDraft.model} onChange={(event) => setSettingsDraft({ ...settingsDraft, model: event.target.value })} /></label><label>API Key 环境变量<input value={settingsDraft.apiKeyEnv} onChange={(event) => setSettingsDraft({ ...settingsDraft, apiKeyEnv: event.target.value.toUpperCase() })} /></label><div className="qa-settings-grid"><label>超时（秒）<input type="number" min="10" max="300" value={settingsDraft.timeoutSeconds} onChange={(event) => setSettingsDraft({ ...settingsDraft, timeoutSeconds: Number(event.target.value) })} /></label><label>最大输出 Token<input type="number" min="256" max="8000" value={settingsDraft.maxOutputTokens} onChange={(event) => setSettingsDraft({ ...settingsDraft, maxOutputTokens: Number(event.target.value) })} /></label><label>Temperature<input type="number" min="0" max="1" step="0.1" value={settingsDraft.temperature} onChange={(event) => setSettingsDraft({ ...settingsDraft, temperature: Number(event.target.value) })} /></label></div><div className="qa-key-state"><ShieldCheck size={15} /><span>{settings.apiKeyConfigured ? `${settings.apiKeyEnv} 已检测到` : `${settings.apiKeyEnv} 尚未检测到；保存后仍可使用离线证据模式`}</span></div><div className="qa-settings-actions"><button onClick={() => setSettingsOpen(false)}>取消</button><button className="primary" onClick={() => void persistSettings()}>保存设置</button></div></div></div>}
  </section>
}
