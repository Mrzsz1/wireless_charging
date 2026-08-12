import { useCallback, useEffect, useState } from 'react'
import { Bot, CheckCircle2, CloudDownload, Eye, EyeOff, FolderOpen, KeyRound, LoaderCircle, LogIn, RefreshCw, Save, Settings2, ShieldCheck, Trash2 } from 'lucide-react'
import { deleteSearchProviderKey, getCodexSubscriptionStatus, getLiteratureSettings, getQaSettings, listSearchProviderStatuses, saveLiteratureSettings, saveQaSettings, saveSearchProviderKey, startCodexLogin, testSearchProvider } from '../../services/desktop'
import type { CodexSubscriptionStatus, LiteratureIngestSettings, QaSettings, SearchProviderStatus } from '../../types'
import { DelayedHelp } from '../../components/DelayedHelp'
import './SettingsView.css'

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
  timeoutSeconds: 90,
  maxOutputTokens: 1800,
  contextWindowTokens: 32768,
  recentExchangeLimit: 3,
  temperature: 0.1,
  apiKeyConfigured: false,
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

const reasoningLabels: Record<string, string> = { low: '低', medium: '中', high: '高', xhigh: '极高', max: '最大', ultra: 'Ultra' }

function replaceStatus(statuses: SearchProviderStatus[], next: SearchProviderStatus) {
  return statuses.map((item) => item.id === next.id ? next : item)
}

export function SettingsView({ repositoryPath, theme, fontSize, releaseInfo, updateBusy, desktopRuntime, focusSection, onChooseRepository, onRebuild, onThemeChange, onFontSizeChange, onUpdate }: Props) {
  const [settings, setSettings] = useState(defaultSettings)
  const [qaSettings, setQaSettings] = useState(defaultQaSettings)
  const [codexStatus, setCodexStatus] = useState(emptyCodexStatus)
  const [providerStatuses, setProviderStatuses] = useState<SearchProviderStatus[]>([])
  const [keyDrafts, setKeyDrafts] = useState<Record<string, string>>({})
  const [visibleKeys, setVisibleKeys] = useState<Record<string, boolean>>({})
  const [busyAction, setBusyAction] = useState('load')
  const [message, setMessage] = useState('')
  const [error, setError] = useState('')
  const effectiveCodexModel = qaSettings.codexModel || codexStatus.configuredModel
  const selectedCodexOption = codexStatus.availableModels.find((item) => item.id === effectiveCodexModel)
  const supportedReasoningEfforts = selectedCodexOption?.supportedReasoningEfforts ?? []
  const configuredEffortSupported = !supportedReasoningEfforts.length || supportedReasoningEfforts.includes(codexStatus.configuredReasoningEffort)
  const automaticReasoningEffort = configuredEffortSupported ? codexStatus.configuredReasoningEffort || selectedCodexOption?.defaultReasoningEffort || '' : selectedCodexOption?.defaultReasoningEffort || ''

  const load = useCallback(async () => {
    setBusyAction('load'); setError('')
    try {
      const [statuses, literature, qa, codex] = await Promise.all([
        listSearchProviderStatuses(),
        repositoryPath ? getLiteratureSettings() : Promise.resolve(defaultSettings),
        repositoryPath ? getQaSettings() : Promise.resolve(defaultQaSettings),
        getCodexSubscriptionStatus(),
      ])
      setProviderStatuses(statuses)
      setSettings(literature)
      setQaSettings(qa)
      setCodexStatus(codex)
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
        <div className="settings-number-grid"><label><span>上下文窗口 Token</span><input type="number" min="8192" max="1000000" step="1024" disabled={!repositoryPath} value={qaSettings.contextWindowTokens} onChange={(event) => setQaSettings((current) => ({ ...current, contextWindowTokens: Number(event.target.value) }))} /></label><label><span>保留近期完整轮次</span><input type="number" min="1" max="8" step="1" disabled={!repositoryPath} value={qaSettings.recentExchangeLimit} onChange={(event) => setQaSettings((current) => ({ ...current, recentExchangeLimit: Number(event.target.value) }))} /></label></div><p className="qa-provider-note">研究契约、会话记忆、近期完整轮次、本轮问题和证据包采用显式预算，同时预留最大输出 Token 与安全余量。</p>
        {qaSettings.answerProvider === 'codex-subscription' && <div className="qa-provider-pane" data-testid="qa-provider-codex">
          <div className={`codex-status-card ${codexStatus.ready ? 'ready' : 'missing'}`}><div className="codex-status-icon">{codexStatus.ready ? <CheckCircle2 size={20} /> : <ShieldCheck size={20} />}</div><div><strong>{codexStatus.statusLabel}</strong><span>{codexStatus.version || '未检测到版本'}</span><p>{codexStatus.diagnostic}</p></div><div className="codex-status-actions"><button disabled={busyAction === 'load'} onClick={() => void load()}><RefreshCw className={busyAction === 'load' ? 'spin' : ''} size={14} />刷新状态</button>{!codexStatus.authenticated && <button className="primary" disabled={!codexStatus.installed || !!busyAction} onClick={() => void beginCodexLogin()}>{busyAction === 'codex-login' ? <LoaderCircle className="spin" size={14} /> : <LogIn size={14} />}登录 ChatGPT</button>}</div></div>
          <div className="qa-api-fields qa-codex-selection">
            <label><span>Codex 模型</span><select disabled={!repositoryPath} value={qaSettings.codexModel} onChange={(event) => setQaSettings((current) => ({ ...current, codexModel: event.target.value, codexReasoningEffort: '' }))}><option value="">自动（{codexStatus.configuredModel || 'Codex 默认'}）</option>{codexStatus.availableModels.map((model) => <option key={model.id} value={model.id}>{model.displayName}</option>)}{qaSettings.codexModel && !codexStatus.availableModels.some((model) => model.id === qaSettings.codexModel) && <option value={qaSettings.codexModel}>{qaSettings.codexModel}（当前未发现）</option>}</select></label>
            <label><span>推理强度</span><select disabled={!repositoryPath} value={qaSettings.codexReasoningEffort} onChange={(event) => setQaSettings((current) => ({ ...current, codexReasoningEffort: event.target.value }))}><option value="">自动（{reasoningLabels[automaticReasoningEffort] || automaticReasoningEffort || '模型默认'}）</option>{supportedReasoningEfforts.map((effort) => <option key={effort} value={effort}>{reasoningLabels[effort] || effort}</option>)}</select></label>
          </div><small className="qa-provider-note">已从本机 Codex 识别 {codexStatus.availableModels.length} 个可选模型；客户端只读取模型元数据，不读取或复制 token、cookie、API Key。</small>
        </div>}
        {qaSettings.answerProvider === 'compatible-api' && <div className="qa-provider-pane" data-testid="qa-provider-api">
          <p className="qa-provider-note">兼容现有 Chat Completions SSE 服务。API Key 仍只从进程环境变量读取，不写入 SQLite 或日志。</p>
          <div className="qa-api-fields"><label><span>Chat Completions endpoint</span><input disabled={!repositoryPath} value={qaSettings.endpoint} onChange={(event) => setQaSettings((current) => ({ ...current, endpoint: event.target.value }))} placeholder="https://HOST/v1/chat/completions" /></label><label><span>模型</span><input disabled={!repositoryPath} value={qaSettings.model} onChange={(event) => setQaSettings((current) => ({ ...current, model: event.target.value }))} /></label><label><span>API Key 环境变量</span><input disabled={!repositoryPath} value={qaSettings.apiKeyEnv} onChange={(event) => setQaSettings((current) => ({ ...current, apiKeyEnv: event.target.value.toUpperCase() }))} /></label></div>
          <div className="settings-number-grid"><label><span>超时（秒）</span><input type="number" min="10" max="300" disabled={!repositoryPath} value={qaSettings.timeoutSeconds} onChange={(event) => setQaSettings((current) => ({ ...current, timeoutSeconds: Number(event.target.value) }))} /></label><label><span>最大输出 Token</span><input type="number" min="256" max="8000" disabled={!repositoryPath} value={qaSettings.maxOutputTokens} onChange={(event) => setQaSettings((current) => ({ ...current, maxOutputTokens: Number(event.target.value) }))} /></label><label><span>Temperature</span><input type="number" min="0" max="1" step="0.1" disabled={!repositoryPath} value={qaSettings.temperature} onChange={(event) => setQaSettings((current) => ({ ...current, temperature: Number(event.target.value) }))} /></label></div>
          <div className="qa-api-state"><ShieldCheck size={15} /><span>{qaSettings.apiKeyConfigured ? `${qaSettings.apiKeyEnv} 已检测到` : `${qaSettings.apiKeyEnv} 尚未检测到；运行时将降级为离线证据`}</span></div>
        </div>}
        {qaSettings.answerProvider === 'offline-evidence' && <div className="qa-provider-pane offline" data-testid="qa-provider-offline"><FolderOpen size={22} /><div><strong>本地证据浏览模式</strong><p>检索 Wiki、论文原文、两本核心专著与 Graphify，并展示可审计证据包；不调用在线回答模型。</p></div></div>}
      </section>
      <section className="settings-card compact" data-testid="updater-settings"><div className="settings-card-title"><CloudDownload size={18} /><div className="settings-title-row"><h2>客户端更新</h2><DelayedHelp testId="updater-settings-help" label={`当前版本 ${releaseInfo.version} · ${releaseInfo.channel} 通道。`} /></div><button className="refresh-button" disabled={!desktopRuntime || updateBusy} onClick={onUpdate}>{updateBusy ? <RefreshCw className="spin" size={14} /> : <CloudDownload size={14} />}{updateBusy ? '正在检查' : '检查更新'}</button></div></section>
    </div>
  </section>
}
