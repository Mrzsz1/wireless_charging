import { useEffect, useMemo, useRef, useState } from 'react'
import { BookOpen, ChevronLeft, ChevronRight, FileText, GitBranch, LoaderCircle, Network, Pin, PinOff, Plus, RefreshCw, Search, Star } from 'lucide-react'
import { isDesktopRuntime, prepareResearchTrail, searchBookChapters, searchPages } from '../../services/desktop'
import type { ResearchTrailItem, ResearchTrailRequest, ResearchTrailResponse } from '../../types'
import { createLatestRequestGuard } from '../../lib/latestRequest'
import { mergePinnedItems, parseResearchTrailPins, pinsForContext, RESEARCH_TRAIL_PINS_KEY, toggleResearchTrailPin } from './researchTrailState'

type Props = {
  open: boolean
  tab: 'evidence' | 'methods'
  request: ResearchTrailRequest | null
  repositoryPath: string
  refreshVersion: number
  onClose: () => void
  onOpen: () => void
  onTabChange: (tab: 'evidence' | 'methods') => void
  onOpenPage: (pageId: string) => void
  onOpenBook: (bookId: string, chapterId: string) => void
  onOpenPath: (path: string) => void
  onOpenGraph: (nodeId?: string) => void
  onShowMethods: (query: string) => void
}

const relationLabels: Record<string, string> = {
  outgoing_link: '正文引用', backlink: '反向引用', graph_neighbor: '图谱相邻', wiki_fts: 'Wiki 命中',
  book_fts: '核心书籍', field_overlap: '字段相似', graph_hint: '图谱提示',
  manual: '手工固定',
}

const responseCache = new Map<string, ResearchTrailResponse>()

function itemIcon(kind: ResearchTrailItem['kind']) {
  return kind === 'book' ? <BookOpen size={13} /> : kind === 'graph' ? <GitBranch size={13} /> : <FileText size={13} />
}

function openItem(item: ResearchTrailItem, props: Props) {
  if (item.kind === 'wiki' && item.pageId) props.onOpenPage(item.pageId)
  else if (item.kind === 'book' && item.bookId && item.chapterId) props.onOpenBook(item.bookId, item.chapterId)
  else if (item.kind === 'graph' && item.nodeId) props.onOpenGraph(item.nodeId)
  else if (item.sourcePath) props.onOpenPath(item.sourcePath)
}

function asWikiItem(id: string, title: string, snippet: string, pageType: string, sourcePath: string): ResearchTrailItem {
  return { id, kind: 'wiki', rank: 0, title, snippet, score: 1, relation: 'manual', retrievalReason: '手动添加到当前研究上下文', pageId: id, pageType, sourcePath, wikilink: `[[${id.replace(/\.md$/, '')}]]`, bookId: '', chapterId: '', markdownPath: '', pdfPath: '', nodeId: '', sourceLocation: sourcePath, graphPath: [] }
}

export function ResearchTrailPanel(props: Props) {
  const [data, setData] = useState<ResearchTrailResponse | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  const [pickerOpen, setPickerOpen] = useState(false)
  const [pickerQuery, setPickerQuery] = useState('')
  const [pickerResults, setPickerResults] = useState<ResearchTrailItem[]>([])
  const [refreshNonce, setRefreshNonce] = useState(0)
  const [pinStore, setPinStore] = useState(() => parseResearchTrailPins(localStorage.getItem(RESEARCH_TRAIL_PINS_KEY)))
  const requests = useRef(createLatestRequestGuard()).current
  const pickerRequests = useRef(createLatestRequestGuard()).current

  useEffect(() => { localStorage.setItem(RESEARCH_TRAIL_PINS_KEY, JSON.stringify(pinStore)) }, [pinStore])

  useEffect(() => {
    if (!props.request || !props.repositoryPath || !isDesktopRuntime()) {
      requests.invalidate(); setData(null); setLoading(false); setError(''); return
    }
    const cacheKey = JSON.stringify([props.repositoryPath, props.request, props.refreshVersion, refreshNonce])
    const cached = responseCache.get(cacheKey)
    if (cached) { requests.invalidate(); setData(cached); setLoading(false); setError(''); return }
    const token = requests.next()
    setLoading(true)
    setData(null)
    setError('')
    void prepareResearchTrail(props.request).then((next) => {
      if (requests.isCurrent(token)) { responseCache.set(cacheKey, next); setData(next) }
    }).catch((cause) => {
      if (requests.isCurrent(token)) { setData(null); setError(String(cause)) }
    }).finally(() => { if (requests.isCurrent(token)) setLoading(false) })
  }, [props.request, props.repositoryPath, props.refreshVersion, refreshNonce, requests])

  useEffect(() => {
    if (!pickerOpen || pickerQuery.trim().length < 2) { pickerRequests.invalidate(); setPickerResults([]); return }
    const token = pickerRequests.next()
    const timer = window.setTimeout(() => {
      const query = pickerQuery.trim()
      void Promise.all([searchPages(query, 12), searchBookChapters(query, undefined, 8)]).then(([pages, books]) => {
        if (!pickerRequests.isCurrent(token)) return
        setPickerResults([
          ...pages.map((item) => asWikiItem(item.id, item.title, item.snippet || item.summary, item.pageType, item.sourcePath)),
          ...books.map((item) => ({ id: item.chapter.id, kind: 'book' as const, rank: 0, title: item.chapter.title, snippet: item.snippet, score: 1, relation: 'manual', retrievalReason: '手动添加到当前研究上下文', pageId: '', pageType: 'book_chapter', sourcePath: item.chapter.markdownPath, wikilink: '', bookId: item.chapter.bookId, chapterId: item.chapter.id, physicalPageStart: item.chapter.physicalPageStart, physicalPageEnd: item.chapter.physicalPageEnd, markdownPath: item.chapter.markdownPath, pdfPath: item.chapter.pdfPath, nodeId: '', sourceLocation: item.chapter.markdownPath, graphPath: [] })),
        ])
      }).catch((cause) => { if (pickerRequests.isCurrent(token)) setError(`证据搜索失败：${String(cause)}`) })
    }, 300)
    return () => window.clearTimeout(timer)
  }, [pickerOpen, pickerQuery, pickerRequests])

  const contextKey = data?.anchor.contextKey ?? ''
  const pinned = useMemo(() => contextKey ? pinsForContext(pinStore, props.repositoryPath, contextKey) : [], [contextKey, pinStore, props.repositoryPath])
  const visible = useMemo(() => mergePinnedItems(pinned, props.tab === 'methods' ? data?.methods ?? [] : data?.evidence ?? []), [data, pinned, props.tab])
  const pinnedKeys = useMemo(() => new Set(pinned.map((item) => `${item.kind}:${item.id}`)), [pinned])

  const togglePin = (item: ResearchTrailItem) => {
    if (!contextKey) return
    const next = toggleResearchTrailPin(pinStore, props.repositoryPath, contextKey, item)
    setPinStore(next)
  }

  if (!props.open) return <button className="context-reopen" aria-label="展开研究脉络" onClick={props.onOpen}><ChevronLeft size={16} /></button>
  return <aside className="context-panel research-trail-panel" data-testid="research-trail-panel">
    <div className="context-heading"><div><h2>研究脉络</h2>{data?.anchor ? <small>基于：{data.anchor.title}</small> : <small>随页面、提问或搜索切换</small>}</div><div className="trail-heading-actions"><button className="icon-button subtle" title="刷新研究脉络" onClick={() => setRefreshNonce((value) => value + 1)}><RefreshCw size={14} /></button><button className="icon-button subtle" title="收起研究脉络" onClick={props.onClose}><ChevronRight size={17} /></button></div></div>
    <div className="context-tabs"><button className={props.tab === 'evidence' ? 'active' : ''} onClick={() => props.onTabChange('evidence')}>证据链</button><button className={props.tab === 'methods' ? 'active' : ''} onClick={() => props.onTabChange('methods')}>相关方法</button></div>
    {data?.anchor.subtitle && <div className="trail-anchor"><Network size={13} /><span>{data.anchor.subtitle}</span></div>}
    {loading && <div className="trail-state"><LoaderCircle className="spin" size={18} />正在检索当前上下文…</div>}
    {!loading && error && <div className="trail-error"><span>研究脉络加载失败：{error}</span><button onClick={() => setRefreshNonce((value) => value + 1)}><RefreshCw size={13} />重试</button></div>}
    {!loading && !error && !props.request && <div className="trail-state">打开 Wiki 页面、提交研究问题或输入搜索词后显示相关证据。</div>}
    {!loading && !error && props.request && !visible.length && <div className="trail-state">当前上下文没有检索到可审计条目。</div>}
    {!!data?.degradedChannels.length && <div className="trail-degraded">部分通道未就绪：{data.degradedChannels.join('、')}</div>}
    {!loading && !!visible.length && <div className={props.tab === 'methods' ? 'method-list' : 'evidence-list'}>{visible.map((item) => {
      const isPinned = pinnedKeys.has(`${item.kind}:${item.id}`)
      return props.tab === 'methods' ? <div className="method-item trail-method" key={`${item.kind}-${item.id}`}><button className="trail-card-main" onClick={() => openItem(item, props)}><span><strong>{item.title}</strong><span className="method-tags"><span>{relationLabels[item.relation] ?? item.relation}</span><span>{Math.round(item.score * 100)}%</span></span><small>{item.retrievalReason}</small></span></button><button className="trail-pin" title={isPinned ? '取消固定' : '固定到当前上下文'} onClick={() => togglePin(item)}>{isPinned ? <PinOff size={14} /> : <Star size={14} />}</button></div>
        : <div className="evidence-item trail-evidence" key={`${item.kind}-${item.id}`}><span className={`evidence-rank ${isPinned ? 'pinned' : ''}`}>{isPinned ? <Pin size={10} /> : item.rank}</span><button className="trail-card-main" onClick={() => openItem(item, props)}><span><strong>{item.title}</strong><span className="evidence-meta">{itemIcon(item.kind)} {isPinned ? '手工固定' : (relationLabels[item.relation] ?? item.relation)} · {Math.round(item.score * 100)}%{item.kind === 'graph' ? ' · 需回到 Wiki/原文核验' : ''}</span><p>{item.retrievalReason}{item.snippet ? `：${item.snippet}` : ''}</p></span></button><button className="trail-pin" title={isPinned ? '取消固定' : '固定到当前上下文'} onClick={() => togglePin(item)}>{isPinned ? <PinOff size={14} /> : <Pin size={14} />}</button></div>
    })}</div>}
    <div className="evidence-footer"><button className="link-button" disabled={!contextKey} onClick={() => setPickerOpen((value) => !value)}><Plus size={12} />添加证据</button>{props.tab === 'methods' ? <button className="link-button" onClick={() => props.onShowMethods(data?.anchor.title ?? '')}>查看更多方法</button> : <button className="link-button" onClick={() => props.onOpenGraph(data?.anchor.graphNodeId)}>查看脉络图</button>}</div>
    {pickerOpen && <div className="trail-picker"><label><Search size={13} /><input autoFocus value={pickerQuery} onChange={(event) => setPickerQuery(event.target.value)} placeholder="搜索 Wiki 或核心书籍…" /></label><div>{pickerResults.map((item) => <button key={`${item.kind}-${item.id}`} onClick={() => { togglePin(item); setPickerOpen(false); setPickerQuery('') }}>{itemIcon(item.kind)}<span><strong>{item.title}</strong><small>{item.snippet}</small></span><Plus size={13} /></button>)}</div></div>}
  </aside>
}
