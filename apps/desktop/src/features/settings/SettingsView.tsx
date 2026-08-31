import { useCallback, useEffect, useState } from 'react'
import { Bot, CheckCircle2, CloudDownload, Copy, Eye, EyeOff, FolderOpen, HardDrive, KeyRound, LoaderCircle, LogIn, RefreshCw, RotateCcw, Save, Settings2, ShieldCheck, Trash2 } from 'lucide-react'
import { cancelRerankerModelDeployment, cancelSemanticVectorSync, checkRerankerModelDeployment, checkSemanticModelDeployment, chooseSemanticModelCacheDirectory, copySemanticModelCacheAndSwitch, deleteSearchProviderKey, deleteSemanticVectorKey, getCodexSubscriptionStatus, getLiteratureSettings, getQaSettings, getSemanticModelSettings, getSemanticVectorStatus, listSearchProviderStatuses, openSemanticModelCacheDirectory, repairRerankerModelDeployment, repairSemanticModelDeployment, saveLiteratureSettings, saveQaSettings, saveSearchProviderKey, saveSemanticModelSettings, saveSemanticVectorSettings, startCodexLogin, syncSemanticVectors, testSearchProvider } from '../../services/desktop'
import type { CodexSubscriptionStatus, LiteratureIngestSettings, QaSettings, RerankerDeploymentStatus, SearchProviderStatus, SemanticDeploymentStatus, SemanticDownloadProgress, SemanticModelSettings, SemanticVectorStatus, VectorSyncProgress } from '../../types'
import { DelayedHelp } from '../../components/DelayedHelp'
import { formatBytes } from '../ingest/ingestState'
import './SettingsView.css'
import { ResearchStateVocabularySettings } from './ResearchStateVocabularySettings'

type Theme = 'light' | 'dark' | 'system'

type Props = {
  repositoryPath: string
  theme: Theme
  fontSize: number
  releaseInfo: { version: string; channel: string }
  updateBusy: boolean
  desktopRuntime: boolean
  focusSection?: string
  onChooseRepository: () => void
  onRebuild: () => void
  onThemeChange: (theme: Theme) => void
  onFontSizeChange: (size: number) => void
  onUpdate: () => void
}

const defaultSettings: LiteratureIngestSettings = {
  startupPromptEnabled: true,
  autoPromoteEnabled: false,
  minScore: 8,
  maxAutoIngest: 3,
  providers: ['arxiv', 'openalex'],
  sinceYear: 2015,
  suppressedPromptDate: '',
  lastAttemptAt: '',
  lastSuccessAt: '',
}

const defaultQaSettings: QaSettings = {
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

const defaultSemanticSettings: SemanticModelSettings = {
  cacheDir: '',
  effectiveCacheDir: '',
  defaultCacheDir: '',
  usingDefault: true,
  modelName: 'Qdrant/paraphrase-multilingual-MiniLM-L12-v2-onnx-Q',
  remoteVectorEnabled: false,
  remoteVectorEndpoint: '',
  remoteVectorKeyConfigured: false,
}

const defaultVectorStatus: SemanticVectorStatus = {
  schemaVersion: 'rag-vector-store-v2', modelName: defaultSemanticSettings.modelName, dimension: 384,
  activeSnapshot: '', local: { store: 'local-sqlite', ready: false, vectorCount: 0, documentCount: 0, pendingSyncCount: 0, modelId: '', dimension: 384, lastError: '' },
  remote: { store: 'pgvector', ready: false, vectorCount: 0, documentCount: 0, pendingSyncCount: 0, modelId: '', dimension: 384, lastError: '' },
  remoteEnabled: false, remoteKeyConfigured: false, countsByGranularity: {}, lastSyncAt: '', lastError: '',
}

const defaultSemanticStatus: SemanticDeploymentStatus = {
  state: 'missing',
  modelName: defaultSemanticSettings.modelName,
  cacheDir: '',
  defaultCacheDir: '',
  runtimeReady: false,
  modelFilesReady: false,
  tokenizerReady: false,
  partialDownloadCount: 0,
  totalBytes: 0,
  probeDimension: 0,
  checkedAt: '',
  diagnostic: '尚未检查本地语义模型部署状态。',
}

const defaultRerankerStatus: RerankerDeploymentStatus = {
  state: 'missing', modelName: 'BAAI/bge-reranker-base', modelVersion: '', modelDir: '',
  runtimeReady: false, modelFilesReady: false, tokenizerReady: false, healthChecked: false,
  checkedAt: '', diagnostic: '尚未检查交叉编码器部署状态。',
}

const semanticStateLabel: Record<SemanticDeploymentStatus['state'], string> = {
  ready: '已部署',
  partial: '下载未完成',
  invalid: '部署损坏',
  missing: '尚未部署',
  error: '检查失败',
}

const semanticPhaseLabel: Record<SemanticDownloadProgress['phase'], string> = {
  runtime: 'ONNX Runtime',
  model: '量化模型',
  tokenizer: 'Tokenizer',
  inference: '推理验证',
}

const emptyCodexStatus: CodexSubscriptionStatus = {
  installed: false,
  version: '',
  authenticated: false,
  ready: false,
  statusLabel: '尚未检测',
  diagnostic: '正在读取本机 Codex 状态。',
  configuredModel: '',
  configuredReasoningEffort: '',
  availableModels: [],
  modelCatalogStatus: 'missing',
}

function replaceStatus(statuses: SearchProviderStatus[], next: SearchProviderStatus) {
  return statuses.map((item) => item.id === next.id ? next : item)
}

function formatSemanticCheckedAt(value: string) {
  const timestamp = Number(value)
  return Number.isFinite(timestamp) && timestamp > 0 ? new Date(timestamp).toLocaleString() : '尚未检查'
}

export function SettingsView({ repositoryPath, theme, fontSize, releaseInfo, updateBusy, desktopRuntime, focusSection, onChooseRepository, onRebuild, onThemeChange, onFontSizeChange, onUpdate }: Props) {
  const [settings, setSettings] = useState(defaultSettings)
  const [qaSettings, setQaSettings] = useState(defaultQaSettings)
  const [semanticSettings, setSemanticSettings] = useState(defaultSemanticSettings)
  const [semanticStatus, setSemanticStatus] = useState(defaultSemanticStatus)
  const [rerankerStatus, setRerankerStatus] = useState(defaultRerankerStatus)
  const [semanticProgress, setSemanticProgress] = useState<SemanticDownloadProgress | null>(null)
  const [rerankerProgress, setRerankerProgress] = useState<SemanticDownloadProgress | null>(null)
  const [vectorStatus, setVectorStatus] = useState(defaultVectorStatus)
  const [vectorProgress, setVectorProgress] = useState<VectorSyncProgress | null>(null)
  const [vectorEndpointDraft, setVectorEndpointDraft] = useState('')
  const [vectorKeyDraft, setVectorKeyDraft] = useState('')
  const [vectorEnabledDraft, setVectorEnabledDraft] = useState(false)
  const [semanticCacheDraft, setSemanticCacheDraft] = useState('')
  const [codexStatus, setCodexStatus] = useState(emptyCodexStatus)
  const [providerStatuses, setProviderStatuses] = useState<SearchProviderStatus[]>([])
  const [keyDrafts, setKeyDrafts] = useState<Record<string, string>>({})
  const [visibleKeys, setVisibleKeys] = useState<Record<string, boolean>>({})
  const [busyAction, setBusyAction] = useState('load')
  const [message, setMessage] = useState('')
  const [error, setError] = useState('')
  const load = useCallback(async () => {
    setBusyAction('load'); setError('')
    try {
      const [statuses, literature, qa, codex, semantic, deployment, reranker, vectors] = await Promise.all([
        listSearchProviderStatuses(),
        repositoryPath ? getLiteratureSettings() : Promise.resolve(defaultSettings),
        repositoryPath ? getQaSettings() : Promise.resolve(defaultQaSettings),
        getCodexSubscriptionStatus(),
        getSemanticModelSettings(),
        checkSemanticModelDeployment(),
        checkRerankerModelDeployment(),
        repositoryPath ? getSemanticVectorStatus().catch(() => defaultVectorStatus) : Promise.resolve(defaultVectorStatus),
      ])
      setProviderStatuses(statuses)
      setSettings(literature)
      setQaSettings(qa)
      setCodexStatus(codex)
      setSemanticSettings(semantic)
      setSemanticCacheDraft(semantic.cacheDir)
      setSemanticStatus(deployment)
      setRerankerStatus(reranker)
      setVectorStatus(vectors)
      setVectorEndpointDraft(semantic.remoteVectorEndpoint)
      setVectorEnabledDraft(semantic.remoteVectorEnabled)
    } catch (reason) { setError(`读取设置失败：${String(reason)}`) }
    finally { setBusyAction('') }
  }, [repositoryPath])

  useEffect(() => { void load() }, [load])
  useEffect(() => {
    if (!focusSection) return
    const timer = window.setTimeout(() => document.getElementById(focusSection)?.scrollIntoView({ behavior: 'smooth', block: 'start' }), 80)
    return () => window.clearTimeout(timer)
  }, [focusSection])

  const persistLiterature = async () => {
    if (!repositoryPath) return
    setBusyAction('literature'); setError(''); setMessage('')
    try {
      setSettings(await saveLiteratureSettings(settings))
      setMessage('文献自动化设置已保存')
    } catch (reason) { setError(`保存失败：${String(reason)}`) }
    finally { setBusyAction('') }
  }

  const persistQa = async () => {
    if (!repositoryPath) return
    setBusyAction('qa'); setError(''); setMessage('')
    try {
      setQaSettings(await saveQaSettings(qaSettings))
      setMessage('AI 回答引擎设置已保存')
    } catch (reason) { setError(`保存回答引擎失败：${String(reason)}`) }
    finally { setBusyAction('') }
  }

  const chooseSemanticCache = async () => {
    setError(''); setMessage('')
    try { setSemanticCacheDraft(await chooseSemanticModelCacheDirectory()) }
    catch (reason) {
      if (!String(reason).includes('用户取消')) setError(`选择缓存目录失败：${String(reason)}`)
    }
  }

  const refreshSemanticDeployment = async () => {
    setBusyAction('semantic-check'); setError(''); setMessage('')
    try {
      const next = await checkSemanticModelDeployment()
      setSemanticStatus(next)
      setMessage(`语义模型检查完成：${semanticStateLabel[next.state]}`)
    } catch (reason) { setError(`语义模型检查失败：${String(reason)}`) }
    finally { setBusyAction('') }
  }

  const repairSemanticDeployment = async () => {
    setBusyAction('semantic-repair'); setError(''); setMessage('')
    setSemanticProgress(null)
    try {
      const next = await repairSemanticModelDeployment(setSemanticProgress)
      setSemanticStatus(next)
      setMessage('语义模型下载、初始化与探针检查已完成')
    } catch (reason) {
      setSemanticProgress((current) => current ? { ...current, status: 'failed', message: String(reason) } : current)
      setError(`语义模型部署失败：${String(reason)}`)
    }
    finally { setBusyAction('') }
  }

  const repairRerankerDeployment = async () => {
    setBusyAction('reranker-repair'); setError(''); setMessage('')
    setRerankerProgress(null)
    try {
      const next = await repairRerankerModelDeployment(setRerankerProgress)
      setRerankerStatus(next)
      setMessage('交叉编码器下载、初始化与健康检查已完成')
    } catch (reason) {
      const cancelled = String(reason).includes('RERANKER_DEPLOYMENT_CANCELLED')
      setRerankerProgress((current) => current ? { ...current, status: cancelled ? 'cancelled' : 'failed', message: cancelled ? '部署已停止，可稍后继续下载' : String(reason) } : current)
      if (cancelled) setMessage('交叉编码器部署已停止')
      else setError(`交叉编码器部署失败：${String(reason)}`)
    }
    finally { setBusyAction('') }
  }

  const stopRerankerDeployment = async () => {
    try { await cancelRerankerModelDeployment(); setMessage('正在停止交叉编码器部署…') }
    catch (reason) { setError(`停止交叉编码器部署失败：${String(reason)}`) }
  }

  const switchAndRedeploySemantic = async () => {
    setBusyAction('semantic-switch-repair'); setError(''); setMessage('')
    setSemanticProgress(null)
    try {
      const saved = await saveSemanticModelSettings(semanticCacheDraft)
      setSemanticSettings(saved)
      setSemanticCacheDraft(saved.cacheDir)
      const next = await repairSemanticModelDeployment(setSemanticProgress)
      setSemanticStatus(next)
      setMessage('已切换缓存目录并完成语义模型部署')
    } catch (reason) {
      setSemanticProgress((current) => current ? { ...current, status: 'failed', message: String(reason) } : current)
      setError(`切换并部署失败：${String(reason)}`)
    }
    finally { setBusyAction('') }
  }

  const copyAndSwitchSemantic = async () => {
    setBusyAction('semantic-copy'); setError(''); setMessage('')
    try {
      const saved = await copySemanticModelCacheAndSwitch(semanticCacheDraft)
      setSemanticSettings(saved)
      setSemanticCacheDraft(saved.cacheDir)
      const next = await checkSemanticModelDeployment()
      setSemanticStatus(next)
      setMessage('已复制现有缓存并切换目录；旧目录仍保留')
    } catch (reason) { setError(`复制缓存失败：${String(reason)}`) }
    finally { setBusyAction('') }
  }

  const openSemanticCache = async () => {
    setError(''); setMessage('')
    try { await openSemanticModelCacheDirectory() }
    catch (reason) { setError(`打开缓存目录失败：${String(reason)}`) }
  }

  const persistVectorSettings = async () => {
    setBusyAction('semantic-vector-settings'); setError(''); setMessage('')
    try {
      const saved = await saveSemanticVectorSettings(vectorEnabledDraft, vectorEndpointDraft, vectorKeyDraft)
      setSemanticSettings(saved)
      setVectorEndpointDraft(saved.remoteVectorEndpoint)
      setVectorEnabledDraft(saved.remoteVectorEnabled)
      setVectorKeyDraft('')
      if (repositoryPath) setVectorStatus(await getSemanticVectorStatus())
      setMessage('向量存储设置已保存；凭据只保存在系统凭据管理器')
    } catch (reason) { setError(`保存向量存储设置失败：${String(reason)}`) }
    finally { setBusyAction('') }
  }

  const clearVectorKey = async () => {
    setBusyAction('semantic-vector-key'); setError(''); setMessage('')
    try {
      const saved = await deleteSemanticVectorKey()
      setSemanticSettings(saved)
      setVectorKeyDraft('')
      setMessage('远程向量凭据已清除')
    } catch (reason) { setError(`清除远程向量凭据失败：${String(reason)}`) }
    finally { setBusyAction('') }
  }

  const runVectorSync = async () => {
    if (!repositoryPath) return
    setBusyAction('semantic-vector-sync'); setError(''); setMessage(''); setVectorProgress(null)
    try {
      const status = await syncSemanticVectors(setVectorProgress)
      setVectorStatus(status)
      setMessage(status.lastError ? '本地向量完成，远程存储已降级' : '多粒度向量同步完成')
    } catch (reason) { setError(`向量同步失败：${String(reason)}`) }
    finally { setBusyAction('') }
  }

  const stopVectorSync = async () => {
    try { await cancelSemanticVectorSync() }
    catch (reason) { setError(`停止向量同步失败：${String(reason)}`) }
  }

  const beginCodexLogin = async () => {
    setBusyAction('codex-login'); setError(''); setMessage('')
    try { setMessage(await startCodexLogin()) }
    catch (reason) { setError(`启动 ChatGPT 登录失败：${String(reason)}`) }
    finally { setBusyAction('') }
  }

  const persistKey = async (provider: string) => {
    const apiKey = keyDrafts[provider]?.trim() ?? ''
    if (!apiKey) { setError('请输入新的 API Key'); return }
    setBusyAction(`save:${provider}`); setError(''); setMessage('')
    try {
      const status = await saveSearchProviderKey(provider, apiKey)
      setProviderStatuses((current) => replaceStatus(current, status))
      setKeyDrafts((current) => ({ ...current, [provider]: '' }))
      setVisibleKeys((current) => ({ ...current, [provider]: false }))
      setMessage(`${status.label} API Key 已安全保存`)
    } catch (reason) { setError(`保存 Key 失败：${String(reason)}`) }
    finally { setBusyAction('') }
  }

  const clearKey = async (provider: string) => {
    setBusyAction(`clear:${provider}`); setError(''); setMessage('')
    try {
      const status = await deleteSearchProviderKey(provider)
      setProviderStatuses((current) => replaceStatus(current, status))
      setKeyDrafts((current) => ({ ...current, [provider]: '' }))
      setMessage(`${status.label} API Key 已清除`)
    } catch (reason) { setError(`清除 Key 失败：${String(reason)}`) }
    finally { setBusyAction('') }
  }

  const checkProvider = async (provider: string) => {
    setBusyAction(`test:${provider}`); setError(''); setMessage('')
    try { setMessage(await testSearchProvider(provider)) }
    catch (reason) { setError(`连接测试失败：${String(reason)}`) }
    finally { setBusyAction('') }
  }

  const toggleProvider = (provider: string, enabled: boolean) => setSettings((current) => ({
    ...current,
    providers: enabled ? [...new Set([...current.providers, provider])] : current.providers.filter((item) => item !== provider),
  }))

  const semanticPathChanged = semanticCacheDraft.trim() !== semanticSettings.cacheDir
  const semanticBusy = busyAction.startsWith('semantic-')

  return <section className="settings-view" data-testid="settings-view">
    <header className="settings-heading"><div><div className="settings-title-row"><h1>设置</h1><DelayedHelp testId="settings-page-help" label="集中管理知识库、论文检索、自动入库和客户端偏好。" /></div></div><button className="refresh-button" disabled={busyAction === 'load'} onClick={() => void load()}><RefreshCw className={busyAction === 'load' ? 'spin' : ''} size={14} />刷新状态</button></header>
    {error && <div className="settings-alert error">{error}<button onClick={() => setError('')}>关闭</button></div>}
    {message && <div className="settings-alert success"><CheckCircle2 size={15} />{message}<button onClick={() => setMessage('')}>关闭</button></div>}

    <div className="settings-sections">
      <section className="settings-card" data-testid="repository-settings"><div className="settings-card-title"><FolderOpen size={18} /><div className="settings-title-row"><h2>知识库与外观</h2><DelayedHelp testId="repository-settings-help" label="选择正文仓库并调整本机显示偏好。" /></div></div><div className="settings-repository"><strong>{repositoryPath || '尚未选择本地知识库'}</strong><div><button className="refresh-button" onClick={onChooseRepository}><FolderOpen size={14} />选择目录</button><button className="refresh-button" disabled={!repositoryPath} onClick={onRebuild}><RefreshCw size={14} />重建索引</button></div></div><div className="settings-inline-fields"><label><span>主题</span><select value={theme} onChange={(event) => onThemeChange(event.target.value as Theme)}><option value="light">浅色</option><option value="dark">深色</option><option value="system">跟随系统</option></select></label><label><span>字号 {fontSize}px</span><input type="range" min="12" max="18" value={fontSize} onChange={(event) => onFontSizeChange(Number(event.target.value))} /></label></div></section>

      <section className="settings-card" data-testid="literature-automation-settings"><div className="settings-card-title"><Settings2 size={18} /><div className="settings-title-row"><h2>文献自动化</h2><DelayedHelp testId="literature-settings-help" label="此处配置“文献入库 → 自动添加”的启动与筛选规则。" /></div><button className="settings-save" disabled={!repositoryPath || !!busyAction} onClick={() => void persistLiterature()}>{busyAction === 'literature' ? <LoaderCircle className="spin" size={14} /> : <Save size={14} />}保存</button></div>{!repositoryPath ? <div className="settings-disabled">选择知识库后可编辑自动化设置。</div> : <><div className="settings-checkboxes"><label><input type="checkbox" checked={settings.startupPromptEnabled} onChange={(event) => setSettings((current) => ({ ...current, startupPromptEnabled: event.target.checked }))} /><span><strong>启动时询问是否运行</strong><small>弹窗提供“本次运行 / 今天不再提醒 / 取消”</small></span></label><label><input type="checkbox" checked={settings.autoPromoteEnabled} onChange={(event) => setSettings((current) => ({ ...current, autoPromoteEnabled: event.target.checked }))} /><span><strong>允许自动完整入库</strong><small>关闭时只准备候选，不写正式 Wiki</small></span></label></div><div className="settings-number-grid"><label><span>相关度阈值</span><input type="number" min="0" max="100" step="0.5" value={settings.minScore} onChange={(event) => setSettings((current) => ({ ...current, minScore: Number(event.target.value) }))} /></label><label><span>单次最大入库数</span><input type="number" min="1" max="20" value={settings.maxAutoIngest} onChange={(event) => setSettings((current) => ({ ...current, maxAutoIngest: Number(event.target.value) }))} /></label><label><span>起始年份</span><input type="number" min="1990" max="2100" value={settings.sinceYear ?? ''} onChange={(event) => setSettings((current) => ({ ...current, sinceYear: event.target.value ? Number(event.target.value) : null }))} /></label></div><fieldset className="settings-provider-toggles"><legend>启用的检索来源</legend>{providerStatuses.map((provider) => <label key={provider.id}><input type="checkbox" checked={settings.providers.includes(provider.id)} onChange={(event) => toggleProvider(provider.id, event.target.checked)} /><span>{provider.label}</span></label>)}</fieldset></>}</section>

      <section className="settings-card" data-testid="search-api-settings"><div className="settings-card-title"><KeyRound size={18} /><div className="settings-title-row"><h2>论文搜索服务</h2><DelayedHelp testId="search-api-settings-help" label="API Key 保存到 Windows 凭据管理器，不写入知识库、SQLite 或日志。" /></div></div><div className="provider-credential-list">{providerStatuses.map((provider) => <article className="provider-credential" key={provider.id} data-testid={`provider-${provider.id}`}><div className="provider-copy"><div><strong>{provider.label}</strong><span className={provider.configured ? 'configured' : 'missing'}>{provider.requiresKey ? provider.configured ? '已安全配置' : '尚未配置' : '无需 Key'}</span></div><p>{provider.description}</p></div>{provider.requiresKey && <div className="provider-key-editor"><label><span className="sr-only">{provider.label} API Key</span><input aria-label={`${provider.label} API Key`} type={visibleKeys[provider.id] ? 'text' : 'password'} autoComplete="off" value={keyDrafts[provider.id] ?? ''} onChange={(event) => setKeyDrafts((current) => ({ ...current, [provider.id]: event.target.value }))} placeholder={provider.configured ? '输入新 Key 可替换（已保存值不会回显）' : '输入 API Key'} /><button aria-label={visibleKeys[provider.id] ? '隐藏本次输入' : '显示本次输入'} onClick={() => setVisibleKeys((current) => ({ ...current, [provider.id]: !current[provider.id] }))}>{visibleKeys[provider.id] ? <EyeOff size={15} /> : <Eye size={15} />}</button></label><button disabled={!keyDrafts[provider.id]?.trim() || !!busyAction} onClick={() => void persistKey(provider.id)}><Save size={14} />保存</button><button disabled={!provider.configured || !!busyAction} onClick={() => void clearKey(provider.id)}><Trash2 size={14} />清除</button></div>}<button className="provider-test" disabled={(provider.requiresKey && !provider.configured) || !!busyAction} onClick={() => void checkProvider(provider.id)}>{busyAction === `test:${provider.id}` ? <LoaderCircle className="spin" size={14} /> : <ShieldCheck size={14} />}测试连接</button></article>)}</div></section>

      <section id="qa-engine-settings" className="settings-card qa-engine-settings" data-testid="qa-engine-settings" data-loaded={busyAction === 'load' ? 'false' : 'true'}>
        <div className="settings-card-title"><Bot size={18} /><div className="settings-title-row"><h2>AI 回答引擎</h2><DelayedHelp testId="qa-settings-help" label="订阅、兼容 API 与离线模式统一在这里管理；智能问答页面不再保存连接参数。" /></div><button className="settings-save" disabled={!repositoryPath || !!busyAction} onClick={() => void persistQa()}>{busyAction === 'qa' ? <LoaderCircle className="spin" size={14} /> : <Save size={14} />}保存</button></div>
        <div className="qa-provider-tabs" role="radiogroup" aria-label="AI 回答引擎">
          <button disabled={busyAction === 'load'} data-testid="qa-provider-tab-codex" role="radio" aria-checked={qaSettings.answerProvider === 'codex-subscription'} className={qaSettings.answerProvider === 'codex-subscription' ? 'active' : ''} onClick={() => setQaSettings((current) => ({ ...current, answerProvider: 'codex-subscription' }))}><ShieldCheck size={15} /><span><strong>Codex 订阅</strong><small>推荐 · 无需 API Key</small></span></button>
          <button disabled={busyAction === 'load'} data-testid="qa-provider-tab-api" role="radio" aria-checked={qaSettings.answerProvider === 'compatible-api'} className={qaSettings.answerProvider === 'compatible-api' ? 'active' : ''} onClick={() => setQaSettings((current) => ({ ...current, answerProvider: 'compatible-api' }))}><KeyRound size={15} /><span><strong>兼容 API</strong><small>保留 Luna/OpenAI-compatible</small></span></button>
          <button disabled={busyAction === 'load'} data-testid="qa-provider-tab-offline" role="radio" aria-checked={qaSettings.answerProvider === 'offline-evidence'} className={qaSettings.answerProvider === 'offline-evidence' ? 'active' : ''} onClick={() => setQaSettings((current) => ({ ...current, answerProvider: 'offline-evidence' }))}><FolderOpen size={15} /><span><strong>证据浏览模式</strong><small>确定性证据包</small></span></button>
        </div>
        {!repositoryPath && <div className="settings-disabled">选择知识库后才能保存回答引擎；本机 Codex 登录状态仍可查看。</div>}
        <div className="settings-number-grid"><label><span>上下文窗口 Token</span><input type="number" min="8192" max="1000000" step="1024" disabled={!repositoryPath} value={qaSettings.contextWindowTokens} onChange={(event) => setQaSettings((current) => ({ ...current, contextWindowTokens: Number(event.target.value) }))} /></label></div><p className="qa-provider-note">历史轮数不设固定上限。系统按模型窗口尽可能保留完整问答，达到 Token 上限后压缩较老历史并重新计算上下文。</p>
        {qaSettings.answerProvider === 'codex-subscription' && <div className="qa-provider-pane" data-testid="qa-provider-codex">
          <div className={`codex-status-card ${codexStatus.ready ? 'ready' : 'missing'}`}><div className="codex-status-icon">{codexStatus.ready ? <CheckCircle2 size={20} /> : <ShieldCheck size={20} />}</div><div><strong>{codexStatus.statusLabel}</strong><span>{codexStatus.version || '未检测到版本'}</span><p>{codexStatus.diagnostic}</p></div><div className="codex-status-actions"><button disabled={busyAction === 'load'} onClick={() => void load()}><RefreshCw className={busyAction === 'load' ? 'spin' : ''} size={14} />刷新状态</button>{!codexStatus.authenticated && <button className="primary" disabled={!codexStatus.installed || !!busyAction} onClick={() => void beginCodexLogin()}>{busyAction === 'codex-login' ? <LoaderCircle className="spin" size={14} /> : <LogIn size={14} />}登录 ChatGPT</button>}</div></div>
          <small className="qa-provider-note">已从本机 Codex 识别 {codexStatus.availableModels.length} 个可选模型。每次对话的模型与推理强度在智能问答输入框中选择；客户端只读取模型元数据，不读取或复制 token、cookie、API Key。</small>
        </div>}
        {qaSettings.answerProvider === 'compatible-api' && <div className="qa-provider-pane" data-testid="qa-provider-api">
          <p className="qa-provider-note">兼容现有 Chat Completions SSE 服务。API Key 仍只从进程环境变量读取，不写入 SQLite 或日志。</p>
          <div className="qa-api-fields"><label><span>Chat Completions endpoint</span><input disabled={!repositoryPath} value={qaSettings.endpoint} onChange={(event) => setQaSettings((current) => ({ ...current, endpoint: event.target.value }))} placeholder="https://HOST/v1/chat/completions" /></label><label><span>模型</span><input disabled={!repositoryPath} value={qaSettings.model} onChange={(event) => setQaSettings((current) => ({ ...current, model: event.target.value }))} /></label><label><span>API Key 环境变量</span><input disabled={!repositoryPath} value={qaSettings.apiKeyEnv} onChange={(event) => setQaSettings((current) => ({ ...current, apiKeyEnv: event.target.value.toUpperCase() }))} /></label></div>
          <div className="settings-number-grid"><label><span>超时（秒）</span><input type="number" min="10" max="300" disabled={!repositoryPath} value={qaSettings.timeoutSeconds} onChange={(event) => setQaSettings((current) => ({ ...current, timeoutSeconds: Number(event.target.value) }))} /></label><label><span>最大输出 Token</span><input type="number" min="256" max="8000" disabled={!repositoryPath} value={qaSettings.maxOutputTokens} onChange={(event) => setQaSettings((current) => ({ ...current, maxOutputTokens: Number(event.target.value) }))} /></label><label><span>Temperature</span><input type="number" min="0" max="1" step="0.1" disabled={!repositoryPath} value={qaSettings.temperature} onChange={(event) => setQaSettings((current) => ({ ...current, temperature: Number(event.target.value) }))} /></label></div>
          <div className="qa-api-state"><ShieldCheck size={15} /><span>{qaSettings.apiKeyConfigured ? `${qaSettings.apiKeyEnv} 已检测到` : `${qaSettings.apiKeyEnv} 尚未检测到；运行时将降级为离线证据`}</span></div>
        </div>}
        {qaSettings.answerProvider === 'offline-evidence' && <div className="qa-provider-pane offline" data-testid="qa-provider-offline"><FolderOpen size={22} /><div><strong>本地证据浏览模式</strong><p>检索 Wiki、论文原文、两本核心专著与 Graphify，并展示可审计证据包；不调用在线回答模型。</p></div></div>}
      </section>
      <ResearchStateVocabularySettings repositoryPath={repositoryPath} />
      <section id="semantic-model-settings" className="settings-card semantic-model-settings" data-testid="semantic-model-settings">
        <div className="settings-card-title"><HardDrive size={18} /><div className="settings-title-row"><h2>本地语义模型</h2><DelayedHelp testId="semantic-model-settings-help" label="为所有知识库共享的本机 embedding 模型。部署检查严格离线，只有下载/修复会访问网络。" /></div></div>
        <div className={`semantic-deployment-status ${semanticStatus.state}`}>
          <div className="semantic-status-main"><span className="semantic-status-icon">{semanticStatus.state === 'ready' ? <CheckCircle2 size={19} /> : semanticStatus.state === 'partial' ? <CloudDownload size={19} /> : <HardDrive size={19} />}</span><div><strong>{semanticStateLabel[semanticStatus.state]}</strong><small title={semanticStatus.modelName}>{semanticStatus.modelName}</small><p>{semanticStatus.diagnostic}</p></div></div>
          <div className="semantic-component-grid"><span data-ready={semanticStatus.runtimeReady}>ONNX Runtime<strong>{semanticStatus.runtimeReady ? '就绪' : '缺失'}</strong></span><span data-ready={semanticStatus.modelFilesReady}>量化模型<strong>{semanticStatus.modelFilesReady ? '完整' : '缺失'}</strong></span><span data-ready={semanticStatus.tokenizerReady}>Tokenizer<strong>{semanticStatus.tokenizerReady ? '完整' : '缺失'}</strong></span><span data-ready={semanticStatus.probeDimension === 384}>推理探针<strong>{semanticStatus.probeDimension ? `${semanticStatus.probeDimension} 维` : '未通过'}</strong></span></div>
          <div className="semantic-status-meta"><span>占用 {formatBytes(semanticStatus.totalBytes)}</span><span>未完成文件 {semanticStatus.partialDownloadCount}</span><span>{semanticSettings.usingDefault ? '默认目录' : '自定义目录'}</span><span>检查时间 {formatSemanticCheckedAt(semanticStatus.checkedAt)}</span></div>
        </div>
        <div className="semantic-path-editor">
          <label><span>模型与向量缓存目录</span><input value={semanticCacheDraft} onChange={(event) => setSemanticCacheDraft(event.target.value)} placeholder={semanticSettings.defaultCacheDir || '使用默认目录'} disabled={semanticBusy} /></label>
          <button disabled={semanticBusy} onClick={() => void chooseSemanticCache()}><FolderOpen size={14} />选择目录</button>
          <button disabled={semanticBusy || !semanticCacheDraft} onClick={() => setSemanticCacheDraft('')}><RotateCcw size={14} />恢复默认</button>
        </div>
        <div className="semantic-current-path" title={semanticSettings.effectiveCacheDir}><HardDrive size={13} /><span>{semanticSettings.effectiveCacheDir || semanticStatus.cacheDir || '尚未解析缓存目录'}</span></div>
        <div className="semantic-actions">
          <button disabled={semanticBusy || busyAction === 'load'} onClick={() => void refreshSemanticDeployment()}>{busyAction === 'semantic-check' ? <LoaderCircle className="spin" size={14} /> : <ShieldCheck size={14} />}检查部署</button>
          <button disabled={semanticBusy} onClick={() => void openSemanticCache()}><FolderOpen size={14} />打开目录</button>
          {semanticPathChanged ? <><button disabled={semanticBusy || semanticStatus.totalBytes === 0 || !semanticCacheDraft.trim()} onClick={() => void copyAndSwitchSemantic()}>{busyAction === 'semantic-copy' ? <LoaderCircle className="spin" size={14} /> : <Copy size={14} />}复制现有缓存并切换</button><button className="primary" disabled={semanticBusy} onClick={() => void switchAndRedeploySemantic()}>{busyAction === 'semantic-switch-repair' ? <LoaderCircle className="spin" size={14} /> : <CloudDownload size={14} />}切换并重新部署</button></> : <button className="primary" disabled={semanticBusy} onClick={() => void repairSemanticDeployment()}>{busyAction === 'semantic-repair' ? <LoaderCircle className="spin" size={14} /> : <CloudDownload size={14} />}{semanticStatus.state === 'ready' ? '重新检查并修复' : '下载/修复'}</button>}
          {semanticProgress && <div className={`semantic-download-progress ${semanticProgress.status}`} data-testid="semantic-download-progress" role="status" aria-live="polite"><div><strong>{semanticPhaseLabel[semanticProgress.phase]}</strong><span>{semanticProgress.totalBytes > 0 ? `${Math.round(semanticProgress.percent)}%` : semanticProgress.message}</span></div><small title={semanticProgress.fileName}>{semanticProgress.fileName}{semanticProgress.totalBytes > 0 ? ` · ${formatBytes(semanticProgress.downloadedBytes)} / ${formatBytes(semanticProgress.totalBytes)} · ${formatBytes(semanticProgress.bytesPerSecond)}/s` : ''}</small><div className={`semantic-progress-track ${semanticProgress.totalBytes > 0 ? '' : 'indeterminate'}`}><i style={semanticProgress.totalBytes > 0 ? { width: `${Math.min(100, semanticProgress.percent)}%` } : undefined} /></div></div>}
        </div>
        <div className={`semantic-deployment-status ${rerankerStatus.state}`} data-testid="reranker-deployment-status">
          <div className="semantic-status-main"><span className="semantic-status-icon">{rerankerStatus.state === 'ready' ? <CheckCircle2 size={19} /> : <CloudDownload size={19} />}</span><div><strong>Cross-Encoder · {semanticStateLabel[rerankerStatus.state]}</strong><small>{rerankerStatus.modelName} · {rerankerStatus.modelVersion || '版本待检查'}</small><p>{rerankerStatus.diagnostic}</p></div></div>
          <div className="semantic-component-grid"><span data-ready={rerankerStatus.runtimeReady}>ONNX Runtime<strong>{rerankerStatus.runtimeReady ? '就绪' : '缺失'}</strong></span><span data-ready={rerankerStatus.modelFilesReady}>Reranker 模型<strong>{rerankerStatus.modelFilesReady ? '完整' : '缺失'}</strong></span><span data-ready={rerankerStatus.tokenizerReady}>Tokenizer<strong>{rerankerStatus.tokenizerReady ? '完整' : '缺失'}</strong></span><span data-ready={rerankerStatus.healthChecked}>健康探针<strong>{rerankerStatus.healthChecked ? '通过' : '未通过'}</strong></span></div>
          <div className="semantic-actions"><button className="primary" disabled={semanticBusy} onClick={() => void repairRerankerDeployment()}>{busyAction === 'reranker-repair' ? <LoaderCircle className="spin" size={14} /> : <CloudDownload size={14} />}{rerankerStatus.state === 'ready' ? '重新检查并修复' : '下载/修复 Cross-Encoder'}</button>{busyAction === 'reranker-repair' && <button onClick={() => void stopRerankerDeployment()}>停止</button>}{rerankerProgress && <div className={`semantic-download-progress ${rerankerProgress.status}`} data-testid="reranker-download-progress" role="status" aria-live="polite"><div><strong>{semanticPhaseLabel[rerankerProgress.phase]}</strong><span>{rerankerProgress.totalBytes > 0 ? `${Math.round(rerankerProgress.percent)}%` : rerankerProgress.message}</span></div><small title={rerankerProgress.fileName}>{rerankerProgress.fileName}{rerankerProgress.totalBytes > 0 ? ` · ${formatBytes(rerankerProgress.downloadedBytes)} / ${formatBytes(rerankerProgress.totalBytes)} · ${formatBytes(rerankerProgress.bytesPerSecond)}/s` : ''}</small><div className={`semantic-progress-track ${rerankerProgress.totalBytes > 0 ? '' : 'indeterminate'}`}><i style={rerankerProgress.totalBytes > 0 ? { width: `${Math.min(100, rerankerProgress.percent)}%` } : undefined} /></div></div>}</div>
        </div>
        <p className="qa-provider-note">查询过程不会下载模型；只有上述显式下载/修复操作允许联网。复制操作保留旧目录作为回滚副本。</p>
        <div className="semantic-vector-panel" data-testid="semantic-vector-panel">
          <div className="semantic-vector-heading"><div><strong>多粒度向量索引</strong><small>文档 {vectorStatus.countsByGranularity.document ?? 0} · 章节 {vectorStatus.countsByGranularity.section ?? 0} · 语义块 {vectorStatus.countsByGranularity.semantic ?? 0}</small></div><span data-ready={vectorStatus.local.ready && vectorStatus.local.vectorCount > 0}>{vectorStatus.local.vectorCount > 0 ? `${vectorStatus.local.vectorCount} 条本地向量` : '尚未构建'}</span></div>
          <div className="semantic-vector-stats"><span>本地：{vectorStatus.local.ready ? '就绪' : '未就绪'}</span><span>远程：{vectorEnabledDraft ? vectorStatus.remote.ready ? '已连接' : vectorStatus.remote.lastError || '待连接' : '未启用'}</span><span>待同步：{vectorStatus.local.pendingSyncCount}</span><span>最近同步：{formatSemanticCheckedAt(vectorStatus.lastSyncAt)}</span></div>
          <div className="semantic-vector-remote"><label className="semantic-vector-toggle"><input type="checkbox" checked={vectorEnabledDraft} onChange={(event) => setVectorEnabledDraft(event.target.checked)} disabled={semanticBusy} /><span>启用 PostgreSQL + pgvector</span></label><input aria-label="pgvector endpoint" value={vectorEndpointDraft} onChange={(event) => setVectorEndpointDraft(event.target.value)} placeholder="https://PROJECT.supabase.co" disabled={semanticBusy} /><input aria-label="pgvector API Key" type="password" autoComplete="off" value={vectorKeyDraft} onChange={(event) => setVectorKeyDraft(event.target.value)} placeholder={semanticSettings.remoteVectorKeyConfigured ? '已安全配置；留空保持不变' : '输入 API Key'} disabled={semanticBusy} /><button disabled={semanticBusy} onClick={() => void persistVectorSettings()}><Save size={14} />保存</button><button disabled={semanticBusy || !semanticSettings.remoteVectorKeyConfigured} onClick={() => void clearVectorKey()}><Trash2 size={14} />清除 Key</button></div>
          <div className="semantic-vector-actions"><button className="primary" disabled={semanticBusy || !repositoryPath || semanticStatus.state !== 'ready'} onClick={() => void runVectorSync()}>{busyAction === 'semantic-vector-sync' ? <LoaderCircle className="spin" size={14} /> : <RefreshCw size={14} />}构建/同步向量</button>{busyAction === 'semantic-vector-sync' && <button onClick={() => void stopVectorSync()}>停止</button>}{vectorProgress && <div className="semantic-vector-progress" data-testid="semantic-vector-progress" role="status"><div><strong>{vectorProgress.message}</strong><span>{Math.round(vectorProgress.percent)}%</span></div><small>已完成 {vectorProgress.completedBlocks}/{vectorProgress.totalBlocks} · 新计算 {vectorProgress.computedBlocks} · 复用 {vectorProgress.reusedBlocks} · 远程 {vectorProgress.remoteSyncedBlocks}</small><div><i style={{ width: `${Math.min(100, vectorProgress.percent)}%` }} /></div></div>}</div>
        </div>
      </section>
      <section className="settings-card compact" data-testid="updater-settings"><div className="settings-card-title"><CloudDownload size={18} /><div className="settings-title-row"><h2>客户端更新</h2><DelayedHelp testId="updater-settings-help" label={`当前版本 ${releaseInfo.version} · ${releaseInfo.channel} 通道。`} /></div><button className="refresh-button" disabled={!desktopRuntime || updateBusy} onClick={onUpdate}>{updateBusy ? <RefreshCw className="spin" size={14} /> : <CloudDownload size={14} />}{updateBusy ? '正在检查' : '检查更新'}</button></div></section>
    </div>
  </section>
}
