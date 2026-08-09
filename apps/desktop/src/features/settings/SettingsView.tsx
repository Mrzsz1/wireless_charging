import { useCallback, useEffect, useState } from 'react'
import { CheckCircle2, CloudDownload, Eye, EyeOff, FolderOpen, KeyRound, LoaderCircle, RefreshCw, Save, Settings2, ShieldCheck, Trash2 } from 'lucide-react'
import { deleteSearchProviderKey, getLiteratureSettings, listSearchProviderStatuses, saveLiteratureSettings, saveSearchProviderKey, testSearchProvider } from '../../services/desktop'
import type { LiteratureIngestSettings, SearchProviderStatus } from '../../types'
import './SettingsView.css'

type Theme = 'light' | 'dark' | 'system'

type Props = {
  repositoryPath: string
  theme: Theme
  fontSize: number
  releaseInfo: { version: string; channel: string }
  updateBusy: boolean
  desktopRuntime: boolean
  onChooseRepository: () => void
  onRebuild: () => void
  onThemeChange: (theme: Theme) => void
  onFontSizeChange: (size: number) => void
  onUpdate: () => void
  onOpenQa: () => void
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

function replaceStatus(statuses: SearchProviderStatus[], next: SearchProviderStatus) {
  return statuses.map((item) => item.id === next.id ? next : item)
}

export function SettingsView({ repositoryPath, theme, fontSize, releaseInfo, updateBusy, desktopRuntime, onChooseRepository, onRebuild, onThemeChange, onFontSizeChange, onUpdate, onOpenQa }: Props) {
  const [settings, setSettings] = useState(defaultSettings)
  const [providerStatuses, setProviderStatuses] = useState<SearchProviderStatus[]>([])
  const [keyDrafts, setKeyDrafts] = useState<Record<string, string>>({})
  const [visibleKeys, setVisibleKeys] = useState<Record<string, boolean>>({})
  const [busyAction, setBusyAction] = useState('')
  const [message, setMessage] = useState('')
  const [error, setError] = useState('')

  const load = useCallback(async () => {
    setBusyAction('load'); setError('')
    try {
      const [statuses, literature] = await Promise.all([
        listSearchProviderStatuses(),
        repositoryPath ? getLiteratureSettings() : Promise.resolve(defaultSettings),
      ])
      setProviderStatuses(statuses)
      setSettings(literature)
    } catch (reason) { setError(`读取设置失败：${String(reason)}`) }
    finally { setBusyAction('') }
  }, [repositoryPath])

  useEffect(() => { void load() }, [load])

  const persistLiterature = async () => {
    if (!repositoryPath) return
    setBusyAction('literature'); setError(''); setMessage('')
    try {
      setSettings(await saveLiteratureSettings(settings))
      setMessage('文献自动化设置已保存')
    } catch (reason) { setError(`保存失败：${String(reason)}`) }
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
    <header className="settings-heading"><div><div className="eyebrow">APPLICATION PREFERENCES</div><h1>设置</h1><p>集中管理知识库、论文检索、自动入库和客户端偏好。</p></div><button className="refresh-button" disabled={busyAction === 'load'} onClick={() => void load()}><RefreshCw className={busyAction === 'load' ? 'spin' : ''} size={14} />刷新状态</button></header>
    {error && <div className="settings-alert error">{error}<button onClick={() => setError('')}>关闭</button></div>}
    {message && <div className="settings-alert success"><CheckCircle2 size={15} />{message}<button onClick={() => setMessage('')}>关闭</button></div>}

    <div className="settings-sections">
      <section className="settings-card" data-testid="repository-settings"><div className="settings-card-title"><FolderOpen size={18} /><div><h2>知识库与外观</h2><p>选择正文仓库并调整本机显示偏好。</p></div></div><div className="settings-repository"><strong>{repositoryPath || '尚未选择本地知识库'}</strong><div><button className="refresh-button" onClick={onChooseRepository}><FolderOpen size={14} />选择目录</button><button className="refresh-button" disabled={!repositoryPath} onClick={onRebuild}><RefreshCw size={14} />重建索引</button></div></div><div className="settings-inline-fields"><label><span>主题</span><select value={theme} onChange={(event) => onThemeChange(event.target.value as Theme)}><option value="light">浅色</option><option value="dark">深色</option><option value="system">跟随系统</option></select></label><label><span>字号 {fontSize}px</span><input type="range" min="12" max="18" value={fontSize} onChange={(event) => onFontSizeChange(Number(event.target.value))} /></label></div></section>

      <section className="settings-card" data-testid="literature-automation-settings"><div className="settings-card-title"><Settings2 size={18} /><div><h2>文献自动化</h2><p>此处配置“文献入库 → 自动添加”的启动与筛选规则。</p></div><button className="settings-save" disabled={!repositoryPath || !!busyAction} onClick={() => void persistLiterature()}>{busyAction === 'literature' ? <LoaderCircle className="spin" size={14} /> : <Save size={14} />}保存</button></div>{!repositoryPath ? <div className="settings-disabled">选择知识库后可编辑自动化设置。</div> : <><div className="settings-checkboxes"><label><input type="checkbox" checked={settings.startupPromptEnabled} onChange={(event) => setSettings((current) => ({ ...current, startupPromptEnabled: event.target.checked }))} /><span><strong>启动时询问是否运行</strong><small>弹窗提供“本次运行 / 今天不再提醒 / 取消”</small></span></label><label><input type="checkbox" checked={settings.autoPromoteEnabled} onChange={(event) => setSettings((current) => ({ ...current, autoPromoteEnabled: event.target.checked }))} /><span><strong>允许自动完整入库</strong><small>关闭时只准备候选，不写正式 Wiki</small></span></label></div><div className="settings-number-grid"><label><span>相关度阈值</span><input type="number" min="0" max="100" step="0.5" value={settings.minScore} onChange={(event) => setSettings((current) => ({ ...current, minScore: Number(event.target.value) }))} /></label><label><span>单次最大入库数</span><input type="number" min="1" max="20" value={settings.maxAutoIngest} onChange={(event) => setSettings((current) => ({ ...current, maxAutoIngest: Number(event.target.value) }))} /></label><label><span>起始年份</span><input type="number" min="1990" max="2100" value={settings.sinceYear ?? ''} onChange={(event) => setSettings((current) => ({ ...current, sinceYear: event.target.value ? Number(event.target.value) : null }))} /></label></div><fieldset className="settings-provider-toggles"><legend>启用的检索来源</legend>{providerStatuses.map((provider) => <label key={provider.id}><input type="checkbox" checked={settings.providers.includes(provider.id)} onChange={(event) => toggleProvider(provider.id, event.target.checked)} /><span>{provider.label}</span></label>)}</fieldset></>}</section>

      <section className="settings-card" data-testid="search-api-settings"><div className="settings-card-title"><KeyRound size={18} /><div><h2>论文搜索服务</h2><p>API Key 保存到 Windows 凭据管理器，不写入知识库、SQLite 或日志。</p></div></div><div className="provider-credential-list">{providerStatuses.map((provider) => <article className="provider-credential" key={provider.id} data-testid={`provider-${provider.id}`}><div className="provider-copy"><div><strong>{provider.label}</strong><span className={provider.configured ? 'configured' : 'missing'}>{provider.requiresKey ? provider.configured ? '已安全配置' : '尚未配置' : '无需 Key'}</span></div><p>{provider.description}</p></div>{provider.requiresKey && <div className="provider-key-editor"><label><span className="sr-only">{provider.label} API Key</span><input aria-label={`${provider.label} API Key`} type={visibleKeys[provider.id] ? 'text' : 'password'} autoComplete="off" value={keyDrafts[provider.id] ?? ''} onChange={(event) => setKeyDrafts((current) => ({ ...current, [provider.id]: event.target.value }))} placeholder={provider.configured ? '输入新 Key 可替换（已保存值不会回显）' : '输入 API Key'} /><button aria-label={visibleKeys[provider.id] ? '隐藏本次输入' : '显示本次输入'} onClick={() => setVisibleKeys((current) => ({ ...current, [provider.id]: !current[provider.id] }))}>{visibleKeys[provider.id] ? <EyeOff size={15} /> : <Eye size={15} />}</button></label><button disabled={!keyDrafts[provider.id]?.trim() || !!busyAction} onClick={() => void persistKey(provider.id)}><Save size={14} />保存</button><button disabled={!provider.configured || !!busyAction} onClick={() => void clearKey(provider.id)}><Trash2 size={14} />清除</button></div>}<button className="provider-test" disabled={(provider.requiresKey && !provider.configured) || !!busyAction} onClick={() => void checkProvider(provider.id)}>{busyAction === `test:${provider.id}` ? <LoaderCircle className="spin" size={14} /> : <ShieldCheck size={14} />}测试连接</button></article>)}</div></section>

      <section className="settings-card compact"><div className="settings-card-title"><ShieldCheck size={18} /><div><h2>Luna 与模型</h2><p>模型地址、模型名和密钥环境变量仍在智能问答设置面板中管理。</p></div><button className="link-button" onClick={onOpenQa}>前往智能问答</button></div></section>
      <section className="settings-card compact" data-testid="updater-settings"><div className="settings-card-title"><CloudDownload size={18} /><div><h2>客户端更新</h2><p>当前版本 {releaseInfo.version} · {releaseInfo.channel} 通道。</p></div><button className="refresh-button" disabled={!desktopRuntime || updateBusy} onClick={onUpdate}>{updateBusy ? <RefreshCw className="spin" size={14} /> : <CloudDownload size={14} />}{updateBusy ? '正在检查' : '检查更新'}</button></div></section>
    </div>
  </section>
}
