import { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState, type ComponentType } from 'react'
import { PhysicalPosition, PhysicalSize } from '@tauri-apps/api/dpi'
import { availableMonitors, getCurrentWindow, primaryMonitor } from '@tauri-apps/api/window'
import {
  Archive,
  BookOpen,
  Bot,
  ChevronDown,
  ChevronRight,
  CircleHelp,
  Code2,
  FileText,
  FilePlus2,
  Folder,
  Gauge,
  GitBranch,
  Home,
  LibraryBig,
  Menu,
  Network,
  Plus,
  RefreshCw,
  Search,
  SquarePen,
  Star,
  X,
} from 'lucide-react'
import { AppToast } from './components/AppToast'
import { SidebarWorkspacePane } from './components/SidebarWorkspacePane'
import { createLatestRequestGuard } from './lib/latestRequest'
import { createPersistedWindowState, LEGACY_WINDOW_STATE_KEY, parsePersistedWindowState, resolveWindowPlacement, WINDOW_STATE_KEY, type MonitorWorkArea, type PersistedWindowState } from './lib/windowPlacement'
import { CoreBooksView } from './features/books/CoreBooksView'
import type { BookTarget } from './features/books/bookTarget'
import { ComparisonView } from './features/comparison/ComparisonView'
import { CompileCenterView } from './features/compile/CompileCenterView'
import { GraphView } from './features/graph/GraphView'
import { nextGraphRefreshVersion } from './features/graph/refreshState'
import { LiteratureIngestView } from './features/ingest/LiteratureIngestView'
import { StartupIngestPrompt } from './features/ingest/StartupIngestPrompt'
import { localDateKey } from './features/ingest/ingestState'
import { LibraryView } from './features/library/LibraryView'
import { PageView } from './features/pages/PageView'
import { AskView } from './features/qa/AskView'
import { ResearchTrailPanel } from './features/research-trail/ResearchTrailPanel'
import { SettingsView } from './features/settings/SettingsView'
import {
  chooseRepository,
  getIngestStartupPrompt,
  getBacklinks,
  getPage,
  isDesktopRuntime,
  listPages,
  openLocalPath,
  processRepositoryChanges,
  rebuildIndex,
  repositoryInfo,
  searchPages,
  suppressIngestPromptToday,
} from './services/desktop'
import { checkAndInstallUpdate, getAppReleaseInfo } from './services/updater'
import type { Backlink, PageDetail, PageFilters, PageSummary, RepositoryInfo, ResearchTrailRequest, SearchResult, StartupPromptState } from './types'

type Icon = ComponentType<{ size?: number; strokeWidth?: number; className?: string }>
type MainView = 'dashboard' | 'qa' | 'library' | 'ingest' | 'methods' | 'books' | 'graph' | 'comparison' | 'compile' | 'settings' | 'help' | 'page'
type Theme = 'light' | 'dark' | 'system'

type NavigationItem = { label: string; view: MainView; icon: Icon }
type WorkspaceItem = { id: string; label: string; view?: MainView; star?: boolean; children?: WorkspaceItem[] }
type WorkTab = { id: string; label: string; kind: string; resourceId?: string; nav?: string; repositoryPath?: string }

const navigation: NavigationItem[] = [
  { label: '工作台', view: 'dashboard', icon: Home },
  { label: '智能问答', view: 'qa', icon: Bot },
  { label: '文献库', view: 'library', icon: FileText },
  { label: '文献入库', view: 'ingest', icon: FilePlus2 },
  { label: '方法库', view: 'methods', icon: Gauge },
  { label: '核心书籍', view: 'books', icon: BookOpen },
  { label: '知识图谱', view: 'graph', icon: Network },
  { label: '对比', view: 'comparison', icon: GitBranch },
  { label: '编译中心', view: 'compile', icon: Code2 },
]

const workspaceItems: WorkspaceItem[] = [
  { id: 'wireless-charging', label: '无线充电调度研究', view: 'dashboard', star: true },
  {
    id: 'vehicle-road',
    label: '车路协同',
    children: [
      { id: 'vehicle-road-library', label: '相关文献', view: 'library' },
      { id: 'vehicle-road-graph', label: '关系图谱', view: 'graph' },
    ],
  },
  {
    id: 'scheduling',
    label: '调度算法',
    children: [
      { id: 'scheduling-methods', label: '方法模型', view: 'methods' },
      { id: 'scheduling-books', label: '核心书籍', view: 'books' },
      { id: 'scheduling-compare', label: '方法对比', view: 'comparison' },
    ],
  },
  {
    id: 'experiments',
    label: '实验数据',
    children: [
      { id: 'experiments-evidence', label: '证据问答', view: 'qa' },
      { id: 'experiments-compile', label: '编译任务', view: 'compile' },
    ],
  },
]

const defaultTab: WorkTab = { id: 'nav-dashboard', label: '研究工作台', kind: 'dashboard', nav: 'dashboard' }
const WORKSPACE_STATE_VERSION = 2

type PersistedWorkspaceState = {
  version: number
  repositoryPath: string
  tabs: WorkTab[]
  activeTab: string
  view: MainView
}

function readStored<T>(key: string, fallback: T): T {
  try {
    const value = localStorage.getItem(key)
    return value ? (JSON.parse(value) as T) : fallback
  } catch {
    return fallback
  }
}

function readWorkspaceState(): PersistedWorkspaceState {
  const fallback: PersistedWorkspaceState = { version: WORKSPACE_STATE_VERSION, repositoryPath: '', tabs: [defaultTab], activeTab: defaultTab.id, view: 'dashboard' }
  const stored = readStored<PersistedWorkspaceState>('desktop.workspace-state.v2', fallback)
  if (stored.version !== WORKSPACE_STATE_VERSION || !Array.isArray(stored.tabs) || !stored.tabs.length || typeof stored.activeTab !== 'string') return fallback
  const activeExists = stored.tabs.some((tab) => tab.id === stored.activeTab)
  return { ...stored, activeTab: activeExists ? stored.activeTab : stored.tabs[0].id, view: activeExists ? stored.view : (stored.tabs[0].kind as MainView) }
}

function viewLabel(view: MainView) {
  return navigation.find((item) => item.view === view)?.label ?? (view === 'settings' ? '设置' : view === 'help' ? '帮助' : '页面')
}

export default function App() {
  const bootWorkspace = useRef(readWorkspaceState()).current
  const [view, setView] = useState<MainView>(bootWorkspace.view)
  const [navCollapsed, setNavCollapsed] = useState(() => readStored('desktop.nav-collapsed', false))
  const [sidebarSearchOpen, setSidebarSearchOpen] = useState(false)
  const [contextOpen, setContextOpen] = useState(() => bootWorkspace.view === 'qa' ? false : readStored('desktop.context-open', true))
  const [contextTab, setContextTab] = useState<'evidence' | 'methods'>('evidence')
  const [expandedWorkspaceNodes, setExpandedWorkspaceNodes] = useState<string[]>(() => readStored('desktop.workspace-expanded', ['scheduling']))
  const [tabs, setTabs] = useState<WorkTab[]>(bootWorkspace.tabs)
  const [activeTab, setActiveTab] = useState(bootWorkspace.activeTab)
  const [repository, setRepository] = useState<RepositoryInfo | null>(null)
  const [repositoryGeneration, setRepositoryGeneration] = useState(0)
  const [bookTarget, setBookTarget] = useState<BookTarget | null>(null)
  const [graphRefreshVersion, setGraphRefreshVersion] = useState(0)
  const [graphFocusNodeId, setGraphFocusNodeId] = useState('')
  const [researchRequest, setResearchRequest] = useState<ResearchTrailRequest | null>(null)
  const [catalog, setCatalog] = useState<PageSummary[]>([])
  const [searchDraft, setSearchDraft] = useState('')
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<SearchResult[]>([])
  const [searchBusy, setSearchBusy] = useState(false)
  const [filters, setFilters] = useState<PageFilters>({ limit: 200, sort: 'modified' })
  const [page, setPage] = useState<PageDetail | null>(null)
  const [backlinks, setBacklinks] = useState<Backlink[]>([])
  const [loading, setLoading] = useState(true)
  const [notice, setNoticeState] = useState({ id: 0, message: '' })
  const [theme, setTheme] = useState<Theme>(() => readStored('desktop.theme', 'light'))
  const [fontSize, setFontSize] = useState(() => readStored('desktop.font-size', 14))
  const [updateBusy, setUpdateBusy] = useState(false)
  const [startupIngestPrompt, setStartupIngestPrompt] = useState<StartupPromptState | null>(null)
  const [startupPromptBusy, setStartupPromptBusy] = useState(false)
  const [autoStartRequest, setAutoStartRequest] = useState<{ version: number; mode: 'prepare' | 'automatic' }>({ version: 0, mode: 'prepare' })
  const [releaseInfo, setReleaseInfo] = useState({ version: '0.11.0', channel: 'stable' })
  const [settingsFocusSection, setSettingsFocusSection] = useState('')
  const globalSearchRef = useRef<HTMLInputElement>(null)
  const workspaceRef = useRef<HTMLElement>(null)
  const currentScrollKey = useRef('')
  const restoredRepository = useRef('')
  const promptedRepositories = useRef(new Set<string>())
  const searchRequests = useRef(createLatestRequestGuard()).current

  const setNotice = useCallback((message: string) => {
    setNoticeState((current) => ({ id: current.id + 1, message }))
  }, [])

  const focusGlobalSearch = useCallback(() => {
    setNavCollapsed(false)
    setSidebarSearchOpen(true)
    window.setTimeout(() => {
      globalSearchRef.current?.focus()
      globalSearchRef.current?.select()
    }, 0)
  }, [])

  useEffect(() => {
    if (!isDesktopRuntime()) return
    void getAppReleaseInfo().then(setReleaseInfo).catch(() => undefined)
  }, [])

  const refreshRepository = useCallback(async () => {
    setLoading(true)
    try {
      const [info, pages] = await Promise.all([
        isDesktopRuntime() ? repositoryInfo() : Promise.resolve<RepositoryInfo | null>(null),
        listPages({ ...filters, limit: 200 }),
      ])
      setRepository(info)
      setCatalog(pages)
      setNotice(info ? `已加载 ${pages.length} 个页面` : `预览模式已加载 ${pages.length} 个页面`)
    } catch (error) {
      setCatalog([])
      setNotice(`页面列表加载失败：${String(error)}`)
    } finally {
      setLoading(false)
    }
  }, [filters, repositoryGeneration])

  useEffect(() => { void refreshRepository() }, [refreshRepository])
  useEffect(() => {
    if (!repository?.path || !isDesktopRuntime() || promptedRepositories.current.has(repository.path)) return
    promptedRepositories.current.add(repository.path)
    void getIngestStartupPrompt(localDateKey()).then((prompt) => {
      if (prompt.shouldPrompt) setStartupIngestPrompt(prompt)
    }).catch((error) => setNotice(`启动文献检查读取失败：${String(error)}`))
  }, [repository?.path])
  useEffect(() => {
    if (!repository || !isDesktopRuntime()) return
    let active = true
    let polling = false
    const timer = window.setInterval(async () => {
      if (!active || polling) return
      polling = true
      try {
        const status = await processRepositoryChanges()
        if (active && status.blocked) {
          setNotice(`自动索引已暂停：${status.lastError ?? '请执行完整重建后重试'}`)
        } else if (active && status.processedChanges > 0) {
          setNotice(`检测到 ${status.processedChanges} 项知识库变更，索引已自动更新`)
          setRepositoryGeneration((value) => value + 1)
          setGraphRefreshVersion((value) => nextGraphRefreshVersion(value, status.graphRefresh))
        }
      } catch (error) {
        if (active) setNotice(`自动索引更新失败：${String(error)}`)
      } finally {
        polling = false
      }
    }, 1500)
    return () => { active = false; window.clearInterval(timer) }
  }, [repository?.path])
  useEffect(() => {
    const handleShortcut = (event: KeyboardEvent) => {
      if (!(event.ctrlKey || event.metaKey) || event.key.toLowerCase() !== 'k' || event.repeat || event.isComposing) return
      const target = event.target as HTMLElement | null
      if (target?.closest('[role="dialog"]')) return
      event.preventDefault()
      focusGlobalSearch()
    }
    window.addEventListener('keydown', handleShortcut)
    return () => window.removeEventListener('keydown', handleShortcut)
  }, [focusGlobalSearch])
  useEffect(() => {
    if (!isDesktopRuntime()) return
    const appWindow = getCurrentWindow()
    const stored = parsePersistedWindowState(readStored<unknown>(WINDOW_STATE_KEY, null))
      ?? parsePersistedWindowState(readStored<unknown>(LEGACY_WINDOW_STATE_KEY, null))
    let disposed = false
    let saveTimer: number | null = null
    let lastNormalState: PersistedWindowState | null = stored ? { ...stored, maximized: false } : null
    const unlisten: Array<() => void> = []
    const persistWindowState = (state: PersistedWindowState) => {
      localStorage.setItem(WINDOW_STATE_KEY, JSON.stringify(state))
    }
    const saveWindow = async () => {
      try {
        const [minimized, maximized] = await Promise.all([appWindow.isMinimized(), appWindow.isMaximized()])
        if (minimized) return
        if (maximized) {
          if (lastNormalState) persistWindowState({ ...lastNormalState, maximized: true })
          return
        }
        const [size, position] = await Promise.all([appWindow.innerSize(), appWindow.outerPosition()])
        lastNormalState = createPersistedWindowState({ width: size.width, height: size.height, x: position.x, y: position.y }, false)
        persistWindowState(lastNormalState)
      } catch { /* keep the current window state when the platform rejects persistence */ }
    }
    const scheduleSave = () => {
      if (saveTimer !== null) window.clearTimeout(saveTimer)
      saveTimer = window.setTimeout(() => {
        saveTimer = null
        void saveWindow()
      }, 180)
    }
    const ensureVisible = async () => {
      try { if (await appWindow.isMinimized()) await appWindow.unminimize() } catch { /* best effort */ }
      try { await appWindow.show() } catch { /* best effort */ }
      try { await appWindow.setFocus() } catch { /* best effort */ }
    }
    void (async () => {
      try {
        const [monitors, primary] = await Promise.all([availableMonitors(), primaryMonitor()])
        const workAreas: MonitorWorkArea[] = monitors.map((monitor) => ({
          x: monitor.workArea.position.x,
          y: monitor.workArea.position.y,
          width: monitor.workArea.size.width,
          height: monitor.workArea.size.height,
          scaleFactor: monitor.scaleFactor,
          primary: primary !== null
            && monitor.position.x === primary.position.x
            && monitor.position.y === primary.position.y
            && monitor.size.width === primary.size.width
            && monitor.size.height === primary.size.height,
        }))
        const placement = resolveWindowPlacement(stored, workAreas)
        if (placement) {
          await appWindow.unmaximize()
          await appWindow.setSize(new PhysicalSize(placement.state.width, placement.state.height))
          await appWindow.setPosition(new PhysicalPosition(placement.state.x, placement.state.y))
          lastNormalState = { ...placement.state, maximized: false }
          persistWindowState(placement.state)
          if (placement.state.maximized) await appWindow.maximize()
        } else {
          await appWindow.unmaximize()
          await appWindow.center()
        }
      } catch {
        try { await appWindow.unmaximize() } catch { /* best effort */ }
        try { await appWindow.center() } catch { /* best effort */ }
      }
      await ensureVisible()
      try {
        const offResize = await appWindow.onResized(scheduleSave)
        const offMove = await appWindow.onMoved(scheduleSave)
        if (disposed) { offResize(); offMove() } else unlisten.push(offResize, offMove)
      } catch { /* window persistence is optional outside the native runtime */ }
    })()
    return () => {
      disposed = true
      if (saveTimer !== null) window.clearTimeout(saveTimer)
      unlisten.forEach((off) => off())
      void saveWindow()
    }
  }, [])
  useEffect(() => { localStorage.setItem('desktop.nav-collapsed', JSON.stringify(navCollapsed)) }, [navCollapsed])
  useEffect(() => { localStorage.setItem('desktop.context-open', JSON.stringify(contextOpen)) }, [contextOpen])
  useEffect(() => {
    if (view === 'qa') setContextOpen(false)
  }, [view])
  useEffect(() => { localStorage.setItem('desktop.workspace-expanded', JSON.stringify(expandedWorkspaceNodes)) }, [expandedWorkspaceNodes])
  useEffect(() => {
    localStorage.setItem('desktop.workspace-state.v2', JSON.stringify({
      version: WORKSPACE_STATE_VERSION,
      repositoryPath: repository?.path ?? bootWorkspace.repositoryPath,
      tabs,
      activeTab,
      view,
    } satisfies PersistedWorkspaceState))
  }, [activeTab, bootWorkspace.repositoryPath, repository?.path, tabs, view])
  useEffect(() => {
    localStorage.setItem('desktop.theme', JSON.stringify(theme))
    document.documentElement.dataset.theme = theme
  }, [theme])
  useEffect(() => {
    localStorage.setItem('desktop.font-size', JSON.stringify(fontSize))
    document.documentElement.style.fontSize = `${fontSize}px`
  }, [fontSize])
  useLayoutEffect(() => {
    const element = workspaceRef.current
    if (!element) return
    const stored = readStored<{ version: number; positions: Record<string, number> }>('desktop.scroll-state.v2', { version: WORKSPACE_STATE_VERSION, positions: {} })
    const positions = stored.version === WORKSPACE_STATE_VERSION ? stored.positions : {}
    if (currentScrollKey.current) positions[currentScrollKey.current] = element.scrollTop
    const nextKey = `${repository?.path ?? 'preview'}:${activeTab}`
    currentScrollKey.current = nextKey
    localStorage.setItem('desktop.scroll-state.v2', JSON.stringify({ version: WORKSPACE_STATE_VERSION, positions }))
    const frame = window.requestAnimationFrame(() => { element.scrollTop = positions[nextKey] ?? 0 })
    return () => window.cancelAnimationFrame(frame)
  }, [activeTab, page?.id, repository?.path, view])

  const activateView = useCallback((nextView: MainView, label = viewLabel(nextView)) => {
    const tabId = `nav-${nextView}`
    setView(nextView)
    setActiveTab(tabId)
    setTabs((current) => current.some((tab) => tab.id === tabId)
      ? current
      : [...current, { id: tabId, label, kind: nextView, nav: nextView }])
    if (!['page', 'qa', 'library', 'methods', 'graph'].includes(nextView)) setResearchRequest(null)
  }, [])

  const openPage = useCallback(async (pageId: string) => {
    setLoading(true)
    try {
      const [detail, linked] = await Promise.all([getPage(pageId), getBacklinks(pageId)])
      setPage(detail)
      setBacklinks(linked)
      setResearchRequest({ kind: 'page', pageId: detail.id, evidenceLimit: 5, methodLimit: 4 })
      setView('page')
      setActiveTab(pageId)
      setTabs((current) => current.some((tab) => tab.id === pageId)
        ? current.map((tab) => tab.id === pageId ? { ...tab, label: detail.title, repositoryPath: repository?.path } : tab)
        : [...current, { id: pageId, label: detail.title, kind: 'page', resourceId: pageId, repositoryPath: repository?.path }])
      return true
    } catch (error) {
      setNotice(`页面打开失败：${String(error)}`)
      return false
    } finally {
      setLoading(false)
    }
  }, [repository?.path])

  useEffect(() => {
    const repositoryPath = repository?.path
    if (!repositoryPath || restoredRepository.current === repositoryPath) return
    restoredRepository.current = repositoryPath
    const repositoryChanged = Boolean(bootWorkspace.repositoryPath && bootWorkspace.repositoryPath !== repositoryPath)
    const validTabs = repositoryChanged
      ? tabs.filter((tab) => tab.kind !== 'page' || tab.repositoryPath === repositoryPath)
      : tabs.filter((tab) => tab.kind !== 'page' || !tab.repositoryPath || tab.repositoryPath === repositoryPath)
    const safeTabs = validTabs.length ? validTabs : [defaultTab]
    if (safeTabs.length !== tabs.length) setTabs(safeTabs)
    const target = safeTabs.find((tab) => tab.id === activeTab) ?? safeTabs[safeTabs.length - 1]
    if (target.kind === 'page' && target.resourceId) {
      void openPage(target.resourceId).then((opened) => {
        if (opened) return
        const fallback = safeTabs.find((tab) => tab.kind !== 'page') ?? defaultTab
        setTabs((current) => current.filter((tab) => tab.id !== target.id))
        setActiveTab(fallback.id)
        setView(fallback.kind as MainView)
      })
    } else {
      setActiveTab(target.id)
      setView(target.kind as MainView)
    }
  }, [activeTab, bootWorkspace.repositoryPath, openPage, repository?.path, tabs])

  const toggleWorkspaceNode = (id: string) => {
    setExpandedWorkspaceNodes((current) => current.includes(id)
      ? current.filter((item) => item !== id)
      : [...current, id])
  }

  const handleChooseRepository = async () => {
    try {
      const selected = await chooseRepository()
      setRepository(selected)
      setNotice(`知识库已加载索引：${selected.pageCount} 个页面`)
      setRepositoryGeneration((value) => value + 1)
    } catch (error) {
      setNotice(`知识库选择未完成：${String(error)}`)
    }
  }

  const handleRebuild = async () => {
    try {
      const stats = await rebuildIndex()
      setNotice(`索引已重建：${stats.pageCount} 个页面`)
      setRepositoryGeneration((value) => value + 1)
    } catch (error) {
      setNotice(`索引重建失败：${String(error)}`)
    }
  }

  const handleUpdate = async () => {
    if (!isDesktopRuntime() || updateBusy) return
    setUpdateBusy(true)
    try {
      await checkAndInstallUpdate((state) => setNotice(state.message))
    } catch (error) {
      setNotice(`更新检查失败：${String(error)}`)
    } finally {
      setUpdateBusy(false)
    }
  }

  const clearSearch = () => {
    searchRequests.invalidate()
    setSearchDraft('')
    setQuery('')
    setResults([])
    setSearchBusy(false)
    setResearchRequest((current) => current?.kind === 'search' ? null : current)
  }

  const handleSearch = async (value: string, navigateToLibrary = false) => {
    setQuery(value)
    const normalized = value.trim()
    if (!normalized) {
      clearSearch()
      return
    }
    const token = searchRequests.next()
    setSearchBusy(true)
    if (navigateToLibrary) activateView('library')
    try {
      const nextResults = await searchPages(normalized, 30)
      if (searchRequests.isCurrent(token)) {
        setResults(nextResults)
        setSearchBusy(false)
      }
    } catch (error) {
      if (searchRequests.isCurrent(token)) {
        setResults([])
        setSearchBusy(false)
        setNotice(`搜索失败：${String(error)}`)
      }
    }
  }

  const submitGlobalSearch = () => {
    if (searchBusy || !searchDraft.trim()) return
    void handleSearch(searchDraft, true)
  }

  const handleLibrarySearch = (value: string) => {
    setSearchDraft(value)
    void handleSearch(value)
  }

  useEffect(() => {
    if (view !== 'library' && view !== 'methods') return
    const value = query.trim()
    if (value.length < 2) { setResearchRequest(null); return }
    const timer = window.setTimeout(() => setResearchRequest({ kind: 'search', text: value, evidenceLimit: 5, methodLimit: 4 }), 350)
    return () => window.clearTimeout(timer)
  }, [query, view])

  const recentPages = useMemo(() => catalog.slice(0, 5), [catalog])
  const renderDashboard = () => (
    <>
      <div className="page-heading">
        <div><h1>研究工作台</h1></div>
        <button className="refresh-button" onClick={() => void refreshRepository()}><RefreshCw size={14} />刷新快照</button>
      </div>
      <div className="metrics-grid">
        <div className="metric"><div className="metric-icon"><FileText size={22} /></div><div><div className="metric-value">{catalog.filter((item) => item.pageType === 'source').length}</div><div className="metric-label">文献来源</div></div></div>
        <div className="metric"><div className="metric-icon"><Gauge size={22} /></div><div><div className="metric-value">{catalog.filter((item) => item.pageType === 'method').length}</div><div className="metric-label">方法模型</div></div></div>
        <div className="metric"><div className="metric-icon"><BookOpen size={22} /></div><div><div className="metric-value">61</div><div className="metric-label">书籍章节</div></div></div>
        <div className="metric"><div className="metric-icon"><Network size={22} /></div><div><div className="metric-value">{repository?.pageCount ?? catalog.length}</div><div className="metric-label">已索引页面</div></div></div>
      </div>
      <section className="panel recent-panel">
        <div className="section-header"><h2>最近更新</h2><button className="link-button" onClick={() => activateView('library')}>查看文献库</button></div>
        <div className="recent-table">
          <div className="table-head"><span>类型</span><span>标题</span><span>状态</span><span>年份</span><span /></div>
          {recentPages.length ? recentPages.map((item) => (
            <button className="research-row" key={item.id} onClick={() => void openPage(item.id)}>
              <span><span className={`type-icon ${item.pageType === 'method' ? 'method' : ''}`}><FileText size={13} /></span>{item.pageType}</span>
              <strong>{item.title}</strong><span className="muted-text">{item.status || '已收录'}</span><span>{item.year || '—'}</span><ChevronRight size={14} />
            </button>
          )) : <div className="empty-state">{loading ? '正在加载知识库…' : '当前没有可展示的页面'}</div>}
        </div>
      </section>
      <div className="dashboard-split">
        <section className="panel"><div className="section-header"><h2>快速入口</h2></div><div className="compile-row"><button className="toolbar-button" onClick={() => activateView('qa')}><Bot size={17} />询问知识库</button><button className="toolbar-button" onClick={() => activateView('ingest')}><FilePlus2 size={17} />添加文献</button><button className="toolbar-button" onClick={() => activateView('comparison')}><GitBranch size={17} />方法对比</button><button className="toolbar-button" onClick={() => activateView('graph')}><Network size={17} />查看图谱</button></div></section>
        <section className="panel"><div className="section-header"><h2>编译状态</h2><button className="link-button" onClick={() => activateView('compile')}>打开编译中心</button></div><div className="compile-row"><div className="compile-icon"><Archive size={21} /></div><div className="compile-title"><strong>知识库编译流水线</strong><span>任务状态以编译中心记录为准</span></div></div></section>
      </div>
    </>
  )

  const openSettings = (section = '') => {
    setSettingsFocusSection(section)
    activateView('settings')
  }

  const renderSettings = () => <SettingsView repositoryPath={repository?.path ?? ''} theme={theme} fontSize={fontSize} releaseInfo={releaseInfo} updateBusy={updateBusy} desktopRuntime={isDesktopRuntime()} focusSection={settingsFocusSection} onChooseRepository={() => void handleChooseRepository()} onRebuild={() => void handleRebuild()} onThemeChange={setTheme} onFontSizeChange={setFontSize} onUpdate={() => void handleUpdate()} />

  const renderContent = () => {
    if (loading && view === 'page') return <div className="page-loading"><RefreshCw className="spin" />正在加载页面…</div>
    if (view === 'dashboard') return renderDashboard()
    if (view === 'library' || view === 'methods') return <LibraryView query={query} results={results} catalog={catalog} pageType={view === 'methods' ? 'method' : 'source'} filters={filters} loading={loading} onQueryChange={handleLibrarySearch} onFiltersChange={setFilters} onOpenResult={(item) => void openPage(item.id)} />
    if (view === 'page' && page) return <PageView page={page} backlinks={backlinks} backlinksLoading={loading} onOpenLink={(id) => void openPage(id)} onOpenPath={(path, reveal) => void openLocalPath(path, reveal)} onReload={() => void openPage(page.id)} />
    if (view === 'books') return <CoreBooksView onOpenLink={(id) => void openPage(id)} target={bookTarget} />
    if (view === 'graph') return <GraphView onOpenPage={(id) => void openPage(id)} refreshVersion={graphRefreshVersion} targetNodeId={graphFocusNodeId} />
    if (view === 'comparison') return <ComparisonView candidates={catalog} onOpenPage={(id) => void openPage(id)} />
    if (view === 'ingest') return <LiteratureIngestView repositoryPath={repository?.path ?? ''} autoStartRequest={autoStartRequest} onChooseRepository={() => void handleChooseRepository()} onCompleted={(message) => { setNotice(message); setRepositoryGeneration((value) => value + 1) }} onOpenCompileCenter={() => activateView('compile')} onOpenSettings={() => openSettings('literature-automation-settings')} onOpenPath={(path, reveal) => void openLocalPath(path, reveal)} />
    if (view === 'qa') return <AskView repositoryPath={repository?.path ?? ''} onOpenSettings={() => openSettings('qa-engine-settings')} onResearchContextChange={(question) => setResearchRequest(question ? { kind: 'question', text: question, evidenceLimit: 5, methodLimit: 4 } : null)} onOpenPage={(id) => void openPage(id)} onOpenBook={(bookId, chapterId) => { setBookTarget({ bookId, chapterId }); activateView('books') }} onOpenPath={(path) => void openLocalPath(path)} />
    if (view === 'compile') return <CompileCenterView repositoryPath={repository?.path ?? ''} onChooseRepository={() => void handleChooseRepository()} onOpenPath={(path) => void openLocalPath(path)} />
    if (view === 'settings') return renderSettings()
    if (view === 'help') return <div className="placeholder-view"><div className="placeholder-icon"><CircleHelp size={28} /></div><h1>帮助</h1><p>“文献入库”用于手动添加 PDF、确认自动发现候选，以及运行启动时询问的自动检索；确认添加会执行完整入库，仅下载不会成为正式证据。编译中心用于查看日志、失败原因、生成物和回滚记录。</p><button className="refresh-button" onClick={() => activateView('dashboard')}>返回工作台</button></div>
    return <div className="placeholder-view"><div className="placeholder-icon"><FileText size={28} /></div><h1>页面未加载</h1><p>请重新从文献库打开该页面。</p></div>
  }

  return (
    <div className="app-shell">
      <header className="titlebar">
        <button className="titlebar-menu" aria-label="展开或收起侧边栏" onClick={() => setNavCollapsed((value) => !value)}><Menu size={18} /></button>
        <div className="titlebar-product" data-tauri-drag-region><span className="window-title">研究工作台</span></div>
        <nav className="titlebar-app-menu" aria-label="应用菜单">
          <button data-testid="settings" className={view === 'settings' ? 'active' : ''} aria-current={view === 'settings' ? 'page' : undefined} onClick={() => openSettings()}>设置</button>
          <button data-testid="help" className={view === 'help' ? 'active' : ''} aria-current={view === 'help' ? 'page' : undefined} onClick={() => activateView('help')}>帮助</button>
        </nav>
        <div className="titlebar-drag-region" data-tauri-drag-region />
        <div className="window-actions">
          <button aria-label="最小化" onClick={() => void getCurrentWindow().minimize()}>−</button>
          <button aria-label="最大化或还原" onClick={() => void getCurrentWindow().toggleMaximize()}>□</button>
          <button aria-label="关闭" onClick={() => void getCurrentWindow().close()}>×</button>
        </div>
      </header>

      <div className="app-body">
        <aside className={`app-sidebar ${navCollapsed ? 'collapsed' : 'expanded'}`} data-testid="sidebar">
          <div className="sidebar-brand">
            <span className="sidebar-icon"><LibraryBig size={19} /></span>
            {!navCollapsed && <><span className="sidebar-brand-label">工作台</span><button className={`sidebar-brand-search ${sidebarSearchOpen ? 'active' : ''}`} data-testid="sidebar-search-trigger" aria-label="搜索知识库" title="搜索知识库（Ctrl K）" onClick={focusGlobalSearch}><Search size={16} /></button></>}
          </div>
          <div className="sidebar-command-area">
            {navCollapsed && <button className="sidebar-nav-item sidebar-command" data-testid="sidebar-search-trigger" aria-label="搜索知识库" title="搜索知识库（Ctrl K）" onClick={focusGlobalSearch}><span className="sidebar-icon"><Search size={18} /></span></button>}
            {!navCollapsed && sidebarSearchOpen && <form className="global-search sidebar-global-search" role="search" onSubmit={(event) => { event.preventDefault(); submitGlobalSearch() }}>
              <Search size={15} />
              <input ref={globalSearchRef} data-testid="global-search" aria-label="搜索论文、方法、模型或问题" value={searchDraft} onChange={(event) => setSearchDraft(event.target.value)} placeholder="搜索知识库…" />
              {searchDraft && <button type="button" className="clear-search" data-testid="global-search-clear" aria-label="清空搜索" onClick={clearSearch}><X size={13} /></button>}
              <button type="submit" className="global-search-submit" data-testid="global-search-submit" disabled={searchBusy || !searchDraft.trim()} aria-label="提交搜索" title="搜索">
                {searchBusy ? <RefreshCw className="spin" size={13} /> : <Search size={13} />}
              </button>
            </form>}
            <button className={`sidebar-nav-item sidebar-command ${view === 'qa' ? 'selected' : ''}`} data-testid="sidebar-new-qa" onClick={() => activateView('qa')} title="新建问答">
              <span className="sidebar-icon"><SquarePen size={18} /></span>{!navCollapsed && <span className="sidebar-label">新建问答</span>}
            </button>
          </div>
          <nav className="primary-nav">
            {navigation.map(({ label, view: itemView, icon: NavIcon }) => (
              <button key={itemView} data-testid={`nav-${itemView}`} className={`sidebar-nav-item ${view === itemView ? 'selected' : ''}`} onClick={() => activateView(itemView, label)} title={label}>
                <span className="sidebar-icon"><NavIcon size={18} /></span>{!navCollapsed && <span className="sidebar-label">{label}</span>}
              </button>
            ))}
          </nav>

          {!navCollapsed && <SidebarWorkspacePane>
            <div className="nav-divider" />
            <div className="nav-section-title"><span>我的空间</span><Plus size={14} /></div>
            <div className="workspace-tree">
              {workspaceItems.map((item) => {
                const expanded = expandedWorkspaceNodes.includes(item.id)
                return <div className="workspace-node" key={item.id}>
                  <button data-testid={item.id === 'scheduling' ? 'space-toggle' : undefined} className={`tree-item ${expanded ? 'expanded' : ''}`} aria-expanded={item.children ? expanded : undefined} onClick={() => item.children ? toggleWorkspaceNode(item.id) : item.view && activateView(item.view, item.label)}>
                    <span className="tree-leading">{item.children ? (expanded ? <ChevronDown size={14} /> : <ChevronRight size={14} />) : <Folder size={14} />}</span><span>{item.label}</span>{item.star && <Star className="star-fill" size={13} />}
                  </button>
                  {item.children && expanded && <div className="workspace-children">{item.children.map((child) => <button className="tree-item tree-child" key={child.id} onClick={() => child.view && activateView(child.view, child.label)}><span className="tree-leading"><ChevronRight size={12} /></span>{child.label}</button>)}</div>}
                </div>
              })}
            </div>
          </SidebarWorkspacePane>}

          <div className="sidebar-spacer" />
        </aside>

        <main ref={workspaceRef} className={`main-workspace ${view === 'qa' ? 'qa-active' : ''} ${view === 'qa' && contextOpen ? 'context-visible' : ''} ${view === 'compile' ? 'compile-active' : ''}`}>
          {renderContent()}
        </main>

        <ResearchTrailPanel open={contextOpen} tab={contextTab} request={researchRequest} repositoryPath={repository?.path ?? ''} refreshVersion={repositoryGeneration + graphRefreshVersion} onClose={() => setContextOpen(false)} onOpen={() => setContextOpen(true)} onTabChange={setContextTab} onOpenPage={(id) => void openPage(id)} onOpenBook={(bookId, chapterId) => { setBookTarget({ bookId, chapterId }); activateView('books') }} onOpenPath={(path) => void openLocalPath(path)} onOpenGraph={(nodeId) => { setGraphFocusNodeId(nodeId ?? ''); activateView('graph') }} onShowMethods={(value) => { activateView('methods'); if (value.trim()) handleLibrarySearch(value) }} />
      </div>

      <footer className="statusbar"><span><i className="status-dot" />{repository?.indexed ? '已同步' : '等待索引'}</span><span>{repository?.path || '尚未选择本地知识库'}</span><span>页面 {repository?.pageCount ?? catalog.length}</span><span className="status-graph">Graphify 派生图</span></footer>
      {notice.message && <AppToast key={notice.id} message={notice.message} contextOpen={contextOpen} onDismiss={() => setNoticeState((current) => current.id === notice.id ? { ...current, message: '' } : current)} />}
      {startupIngestPrompt && <StartupIngestPrompt prompt={startupIngestPrompt} busy={startupPromptBusy} onCancel={() => setStartupIngestPrompt(null)} onSuppressToday={() => {
        setStartupPromptBusy(true)
        void suppressIngestPromptToday(localDateKey()).then(() => { setStartupIngestPrompt(null); setNotice('今天不再提醒自动文献检查') }).catch((error) => setNotice(`设置提醒失败：${String(error)}`)).finally(() => setStartupPromptBusy(false))
      }} onRun={() => {
        const mode = startupIngestPrompt.mode
        setStartupIngestPrompt(null)
        activateView('ingest')
        setAutoStartRequest((current) => ({ version: current.version + 1, mode }))
      }} />}
    </div>
  )
}
