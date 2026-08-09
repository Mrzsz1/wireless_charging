import { useCallback, useEffect, useMemo, useState } from 'react'
import { AlertTriangle, Check, CheckCircle2, ChevronRight, CircleX, CloudDownload, ExternalLink, FileCheck2, FilePlus2, FolderOpen, LoaderCircle, Play, Radar, RefreshCw, Save, Search, Settings2, ShieldCheck, Trash2, UploadCloud } from 'lucide-react'
import { chooseManualPdfs, discardManualImportSession, getLiteratureCapabilities, getLiteratureSettings, listLiteratureCandidates, saveLiteratureSettings, startLiteratureRun, updateCandidateTriage } from '../../services/desktop'
import type { CompileStreamEvent, LiteratureCandidate, LiteratureCapability, LiteratureIngestMode, LiteratureIngestSettings, ManualImportSession } from '../../types'
import { defaultSelectedManualFileIds, filterCandidates, formatBytes, formatEpoch, type CandidateFilter } from './ingestState'
import './LiteratureIngestView.css'

type Props = {
  repositoryPath: string
  autoStartRequest?: { version: number; mode: 'prepare' | 'automatic' }
  onChooseRepository: () => void
  onCompleted: (message: string) => void
  onOpenCompileCenter: () => void
  onOpenPath: (path: string, reveal?: boolean) => void
}

type Tab = 'manual' | 'candidates' | 'automatic'

const defaultSettings: LiteratureIngestSettings = {
  startupPromptEnabled: true, autoPromoteEnabled: false, minScore: 8, maxAutoIngest: 3,
  providers: ['arxiv', 'openalex'], sinceYear: 2015, suppressedPromptDate: '', lastAttemptAt: '', lastSuccessAt: '',
}

const filterLabels: Record<CandidateFilter, string> = { all: '全部', pending: '待确认', selected: '已选择', rejected: '已忽略', promoted: '已入库', eligible: '满足自动规则' }

function eventMessage(event: CompileStreamEvent) {
  return `[${event.stage || event.type}] ${event.message}`
}

export function LiteratureIngestView({ repositoryPath, autoStartRequest, onChooseRepository, onCompleted, onOpenCompileCenter, onOpenPath }: Props) {
  const [tab, setTab] = useState<Tab>('manual')
  const [settings, setSettings] = useState<LiteratureIngestSettings>(defaultSettings)
  const [capabilities, setCapabilities] = useState<LiteratureCapability[]>([])
  const [candidates, setCandidates] = useState<LiteratureCandidate[]>([])
  const [manualSession, setManualSession] = useState<ManualImportSession | null>(null)
  const [selectedFileIds, setSelectedFileIds] = useState<string[]>([])
  const [forceDuplicates, setForceDuplicates] = useState(false)
  const [selectedCandidateIds, setSelectedCandidateIds] = useState<string[]>([])
  const [candidateQuery, setCandidateQuery] = useState('')
  const [candidateFilter, setCandidateFilter] = useState<CandidateFilter>('pending')
  const [providerFilter, setProviderFilter] = useState('all')
  const [candidateSort, setCandidateSort] = useState<'score' | 'year' | 'title'>('score')
  const [candidateNote, setCandidateNote] = useState('')
  const [selectedCandidateId, setSelectedCandidateId] = useState('')
  const [busy, setBusy] = useState(false)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  const [runLog, setRunLog] = useState<string[]>([])
  const [runId, setRunId] = useState('')

  const loadCandidates = useCallback(async () => {
    if (!repositoryPath) return
    setLoading(true)
    try {
      const next = await listLiteratureCandidates()
      setCandidates(next)
      setSelectedCandidateIds((current) => current.filter((id) => next.some((item) => item.candidateId === id)))
      setSelectedCandidateId((current) => next.some((item) => item.candidateId === current) ? current : next[0]?.candidateId ?? '')
      setError('')
    } catch (reason) { setError(String(reason)) }
    finally { setLoading(false) }
  }, [repositoryPath])

  const load = useCallback(async () => {
    if (!repositoryPath) return
    setLoading(true)
    try {
      const [nextSettings, nextCapabilities] = await Promise.all([getLiteratureSettings(), getLiteratureCapabilities()])
      setSettings(nextSettings)
      setCapabilities(nextCapabilities)
      await loadCandidates()
      setError('')
    } catch (reason) { setError(String(reason)) }
    finally { setLoading(false) }
  }, [loadCandidates, repositoryPath])

  useEffect(() => { setManualSession(null); setSelectedFileIds([]); if (repositoryPath) void load() }, [repositoryPath])

  const acceptEvent = (event: CompileStreamEvent) => {
    if (event.type === 'accepted') setRunId(event.runId)
    setRunLog((current) => [...current.slice(-119), eventMessage(event)])
  }

  const run = useCallback(async (mode: LiteratureIngestMode, extra: Partial<{ candidateIds: string[]; manualSessionId: string; selectedFileIds: string[]; forceDuplicates: boolean }> = {}) => {
    if (!repositoryPath || busy) return
    setBusy(true); setError(''); setRunLog([]); setRunId('pending')
    try {
      const result = await startLiteratureRun({ mode, timeoutSeconds: 7200, ...extra }, acceptEvent)
      setRunId(result.id)
      const message = result.status === 'succeeded' ? `文献任务已完成：${result.displayName}` : `文献任务结束：${result.status}`
      onCompleted(message)
      if (mode === 'manual') { setManualSession(null); setSelectedFileIds([]) }
      await Promise.all([loadCandidates(), getLiteratureSettings().then(setSettings)])
    } catch (reason) { setError(String(reason)); onCompleted(`文献任务失败：${String(reason)}`) }
    finally { setBusy(false) }
  }, [busy, loadCandidates, onCompleted, repositoryPath])

  useEffect(() => {
    if (!autoStartRequest?.version || !repositoryPath) return
    setTab('automatic')
    void run(autoStartRequest.mode)
  }, [autoStartRequest?.version])

  const visibleCandidates = useMemo(() => filterCandidates(candidates, candidateQuery, candidateFilter)
    .filter((candidate) => providerFilter === 'all' || candidate.provider === providerFilter)
    .sort((left, right) => candidateSort === 'year'
      ? Number(right.year || 0) - Number(left.year || 0)
      : candidateSort === 'title'
        ? left.title.localeCompare(right.title, 'zh-CN')
        : right.score - left.score), [candidateFilter, candidateQuery, candidateSort, candidates, providerFilter])
  const selectedCandidate = candidates.find((item) => item.candidateId === selectedCandidateId) ?? null
  const eligibleCount = candidates.filter((item) => item.qualification.eligible && item.triageStatus !== 'promoted').length
  const allCapabilitiesReady = capabilities.every((item) => item.available)

  const pickManual = async () => {
    setError('')
    try {
      if (manualSession) await discardManualImportSession(manualSession.id)
      const session = await chooseManualPdfs()
      if (!session) return
      setManualSession(session)
      setSelectedFileIds(defaultSelectedManualFileIds(session))
      setForceDuplicates(false)
    } catch (reason) { setError(String(reason)) }
  }

  const discardManual = async () => {
    if (!manualSession) return
    try { await discardManualImportSession(manualSession.id) } catch { /* a stale preflight can still be cleared locally */ }
    setManualSession(null); setSelectedFileIds([]); setForceDuplicates(false)
  }

  const toggleCandidate = (id: string) => setSelectedCandidateIds((current) => current.includes(id) ? current.filter((item) => item !== id) : [...current, id])

  const triage = async (status: LiteratureCandidate['triageStatus'], note = '') => {
    if (!selectedCandidateIds.length || busy) return
    setBusy(true)
    try { await updateCandidateTriage(selectedCandidateIds, status, note || undefined); setSelectedCandidateIds([]); await loadCandidates() }
    catch (reason) { setError(String(reason)) }
    finally { setBusy(false) }
  }

  const saveSettings = async () => {
    setBusy(true); setError('')
    try { setSettings(await saveLiteratureSettings(settings)); onCompleted('自动入库设置已保存') }
    catch (reason) { setError(String(reason)) }
    finally { setBusy(false) }
  }

  if (!repositoryPath) return <section className="ingest-empty"><FilePlus2 size={36} /><h1>文献入库</h1><p>先选择知识库目录，再添加本地 PDF、确认候选或运行自动检索。</p><button className="ingest-button primary" onClick={onChooseRepository}><FolderOpen size={16} />选择知识库</button></section>

  return <section className="ingest-view" data-testid="literature-ingest">
    <header className="ingest-heading">
      <div><div className="eyebrow">GOVERNED LITERATURE PIPELINE</div><h1>文献入库</h1><p>把本地 PDF 与最新开放论文送入同一条可审计流水线：预检、去重、解析、A 编译、Lint、Graphify。</p></div>
      <div className="ingest-heading-actions"><button className="ingest-button secondary" disabled={loading || busy} onClick={() => void load()}><RefreshCw className={loading ? 'spin' : ''} size={15} />刷新</button><button className="ingest-button secondary" onClick={onOpenCompileCenter}>任务记录<ChevronRight size={15} /></button></div>
    </header>

    {error && <div className="ingest-alert error"><AlertTriangle size={15} /><span>{error}</span><button onClick={() => setError('')}>关闭</button></div>}
    <nav className="ingest-tabs" aria-label="文献入库方式">
      <button data-testid="ingest-tab-manual" className={tab === 'manual' ? 'active' : ''} onClick={() => setTab('manual')}><FilePlus2 size={16} />手动添加</button>
      <button data-testid="ingest-tab-candidates" className={tab === 'candidates' ? 'active' : ''} onClick={() => setTab('candidates')}><FileCheck2 size={16} />待确认<span>{candidates.filter((item) => item.triageStatus === 'pending').length}</span></button>
      <button data-testid="ingest-tab-automatic" className={tab === 'automatic' ? 'active' : ''} onClick={() => setTab('automatic')}><Radar size={16} />自动添加</button>
    </nav>

    {tab === 'manual' && <div className="ingest-manual-layout">
      <section className="ingest-panel manual-drop-panel">
        <div className="ingest-panel-heading"><div><h2>选择本地 PDF</h2><p>单次可选择多篇；文件会先做格式、大小、哈希和正式库重复检查。</p></div><button className="ingest-button primary" disabled={busy} onClick={() => void pickManual()}><UploadCloud size={16} />选择 PDF</button></div>
        {!manualSession && <button className="manual-dropzone" onClick={() => void pickManual()}><FilePlus2 size={28} /><strong>点击选择要入库的 PDF</strong><span>支持多选，单文件不超过 200 MB</span></button>}
        {manualSession && <div className="manual-files">
          {manualSession.files.map((file) => {
            const duplicate = file.duplicateMatches.length > 0
            const checked = selectedFileIds.includes(file.id)
            return <label className={`manual-file ${!file.valid ? 'invalid' : duplicate ? 'duplicate' : ''}`} key={file.id}>
              <input type="checkbox" disabled={!file.valid || (duplicate && !forceDuplicates)} checked={checked} onChange={() => setSelectedFileIds((current) => checked ? current.filter((id) => id !== file.id) : [...current, file.id])} />
              {file.valid ? duplicate ? <AlertTriangle size={17} /> : <CheckCircle2 size={17} /> : <CircleX size={17} />}
              <span><strong>{file.name}</strong><small>{formatBytes(file.size)} · SHA256 {file.sha256 ? file.sha256.slice(0, 12) : '—'}</small>{file.errors.map((item) => <em key={item}>{item}</em>)}{file.duplicateMatches.map((item) => <em key={item.existingPath}>重复：{item.existingPath}</em>)}</span>
            </label>
          })}
        </div>}
      </section>
      <aside className="ingest-panel ingest-confirm-panel">
        <h2>确认完整入库</h2><p>确认后依次执行 PDF 落盘、章节解析、A 编译、索引更新、Lint 与 Graphify。不是简单复制文件。</p>
        <dl><dt>已选择</dt><dd>{selectedFileIds.length} 篇</dd><dt>重复项</dt><dd>{manualSession?.files.filter((file) => file.duplicateMatches.length).length ?? 0} 篇</dd><dt>运行记录</dt><dd>编译中心</dd></dl>
        {!!manualSession?.files.some((file) => file.duplicateMatches.length) && <label className="ingest-checkbox danger"><input type="checkbox" checked={forceDuplicates} onChange={(event) => { setForceDuplicates(event.target.checked); if (!event.target.checked) setSelectedFileIds((current) => current.filter((id) => !manualSession.files.find((file) => file.id === id)?.duplicateMatches.length)) }} /><span>明确允许选择重复 PDF</span></label>}
        <button className="ingest-button primary wide" disabled={!manualSession || !selectedFileIds.length || busy} onClick={() => manualSession && void run('manual', { manualSessionId: manualSession.id, selectedFileIds, forceDuplicates })}>{busy ? <LoaderCircle className="spin" size={16} /> : <Check size={16} />}确认添加并完整入库</button>
        {manualSession && <button className="ingest-button danger wide" disabled={busy} onClick={() => void discardManual()}><Trash2 size={15} />清空本批次</button>}
      </aside>
    </div>}

    {tab === 'candidates' && <div className="candidate-workspace">
      <section className="ingest-panel candidate-list-panel">
        <div className="candidate-toolbar">
          <label><Search size={14} /><input value={candidateQuery} onChange={(event) => setCandidateQuery(event.target.value)} placeholder="搜索标题、作者、摘要、DOI…" /></label>
          <select aria-label="候选状态" value={candidateFilter} onChange={(event) => setCandidateFilter(event.target.value as CandidateFilter)}>{Object.entries(filterLabels).map(([value, label]) => <option value={value} key={value}>{label}</option>)}</select>
          <select aria-label="候选来源" value={providerFilter} onChange={(event) => setProviderFilter(event.target.value)}><option value="all">全部来源</option>{[...new Set(candidates.map((item) => item.provider).filter(Boolean))].sort().map((provider) => <option value={provider} key={provider}>{provider}</option>)}</select>
          <select aria-label="候选排序" value={candidateSort} onChange={(event) => setCandidateSort(event.target.value as typeof candidateSort)}><option value="score">按分数</option><option value="year">按年份</option><option value="title">按标题</option></select>
        </div>
        <div className="candidate-batch-actions"><span>已选 {selectedCandidateIds.length} 篇</span><button disabled={!selectedCandidateIds.length || busy} onClick={() => void run('download', { candidateIds: selectedCandidateIds })}><CloudDownload size={14} />仅下载</button><button disabled={!selectedCandidateIds.length || busy} onClick={() => void run('candidate', { candidateIds: selectedCandidateIds })}><FileCheck2 size={14} />确认添加</button><button disabled={!selectedCandidateIds.length || busy} onClick={() => void triage('pending', candidateNote)}><RefreshCw size={14} />稍后处理</button><button disabled={!selectedCandidateIds.length || busy} onClick={() => void triage('rejected', candidateNote)}><Trash2 size={14} />忽略</button></div>
        <div className="candidate-list-scroll">
          {visibleCandidates.map((candidate) => <article className={`candidate-row ${selectedCandidateId === candidate.candidateId ? 'selected' : ''}`} key={candidate.candidateId} onClick={() => { setSelectedCandidateId(candidate.candidateId); setCandidateNote(candidate.manualNote ?? '') }}>
            <input aria-label={`选择 ${candidate.title}`} type="checkbox" checked={selectedCandidateIds.includes(candidate.candidateId)} onClick={(event) => event.stopPropagation()} onChange={() => toggleCandidate(candidate.candidateId)} />
            <div><strong>{candidate.title}</strong><small>{candidate.authors?.join(', ') || '作者未知'} · {candidate.year || '年份未知'} · {candidate.provider}</small><p>{candidate.abstract || '暂无摘要'}</p><span className={`candidate-state ${candidate.triageStatus}`}>{filterLabels[candidate.triageStatus]}</span>{candidate.localPdf && <span className="candidate-state downloaded">已下载</span>}</div>
            <b className={candidate.qualification.eligible ? 'eligible' : ''}>{candidate.score.toFixed(1)}</b>
          </article>)}
          {!visibleCandidates.length && <div className="ingest-list-empty">{loading ? '正在加载候选…' : '当前筛选条件下没有候选文献'}</div>}
        </div>
      </section>
      <aside className="ingest-panel candidate-detail">
        {selectedCandidate ? <><div className="eyebrow">CANDIDATE DETAIL</div><h2>{selectedCandidate.title}</h2><p>{selectedCandidate.abstract || '暂无摘要'}</p><dl><dt>相关度</dt><dd>{selectedCandidate.score.toFixed(2)}</dd><dt>DOI</dt><dd>{selectedCandidate.doi || '—'}</dd><dt>arXiv</dt><dd>{selectedCandidate.arxivId || '—'}</dd><dt>清单</dt><dd title={selectedCandidate.manifestPath}>{selectedCandidate.manifestPath}</dd></dl>
          <h3>自动资格解释</h3><div className="qualification-list">{selectedCandidate.qualification.reasons.map((reason) => <div className={reason.passed ? 'passed' : 'failed'} key={reason.code}>{reason.passed ? <Check size={13} /> : <CircleX size={13} />}<span>{reason.message}</span></div>)}</div>
          <label className="candidate-note"><span>人工备注（用于“稍后处理”或“忽略”）</span><textarea value={candidateNote} onChange={(event) => setCandidateNote(event.target.value)} placeholder={selectedCandidate.manualNote || '记录筛选理由…'} /></label>
          <div className="candidate-detail-actions"><button disabled={!selectedCandidate.pdfUrl} onClick={() => void run('download', { candidateIds: [selectedCandidate.candidateId] })}><CloudDownload size={14} />仅下载</button><button onClick={() => void run('candidate', { candidateIds: [selectedCandidate.candidateId] })}><FileCheck2 size={14} />确认添加</button>{selectedCandidate.localPdf && <button onClick={() => onOpenPath(selectedCandidate.localPdf, true)}><ExternalLink size={14} />查看 PDF</button>}</div></> : <div className="ingest-list-empty">选择候选查看资格证据</div>}
      </aside>
    </div>}

    {tab === 'automatic' && <div className="automatic-layout">
      <section className="ingest-panel automatic-main">
        <div className="ingest-panel-heading"><div><h2>自动检索与添加</h2><p>启动时只询问一次。手动按钮随时可运行同一受控流程。</p></div><span className={`automation-mode ${settings.autoPromoteEnabled ? 'promote' : ''}`}>{settings.autoPromoteEnabled ? '自动完整入库' : '自动准备候选'}</span></div>
        <div className="automation-summary"><div><Radar size={20} /><span><strong>{eligibleCount}</strong><small>当前满足规则</small></span></div><div><FileCheck2 size={20} /><span><strong>{candidates.filter((item) => item.triageStatus === 'pending').length}</strong><small>等待人工确认</small></span></div><div><ShieldCheck size={20} /><span><strong>{allCapabilitiesReady ? '正常' : '受限'}</strong><small>流水线能力</small></span></div></div>
        <button className="ingest-button primary run-automation" disabled={busy || !capabilities.find((item) => item.id === 'discovery')?.available} onClick={() => void run(settings.autoPromoteEnabled ? 'automatic' : 'prepare')}>{busy ? <LoaderCircle className="spin" size={17} /> : <Play size={17} />}{settings.autoPromoteEnabled ? '立即检索并自动入库' : '立即检索最新文献'}</button>
        <p className="automation-boundary">自动完整入库只处理同时满足：主题命中、分数阈值、标题关键词、DOI/arXiv、开放 PDF、无重复的候选，单次最多 {settings.maxAutoIngest} 篇。</p>
        <h3>依赖检查</h3><div className="capability-grid">{capabilities.map((item) => <div key={item.id} className={item.available ? 'available' : 'unavailable'}>{item.available ? <CheckCircle2 size={15} /> : <AlertTriangle size={15} />}<span><strong>{item.id}</strong><small>{item.reason}</small></span></div>)}</div>
      </section>
      <aside className="ingest-panel automation-settings">
        <div className="ingest-panel-heading"><div><h2><Settings2 size={17} />自动设置</h2></div><button className="ingest-button secondary" disabled={busy} onClick={() => void saveSettings()}><Save size={14} />保存</button></div>
        <label className="ingest-checkbox"><input type="checkbox" checked={settings.startupPromptEnabled} onChange={(event) => setSettings((current) => ({ ...current, startupPromptEnabled: event.target.checked }))} /><span><strong>启动时询问是否运行</strong><small>弹窗提供“本次运行 / 今天不再提醒 / 取消”</small></span></label>
        <label className="ingest-checkbox"><input type="checkbox" checked={settings.autoPromoteEnabled} onChange={(event) => setSettings((current) => ({ ...current, autoPromoteEnabled: event.target.checked }))} /><span><strong>允许自动完整入库</strong><small>关闭时只生成候选清单，不写正式 Wiki</small></span></label>
        <label className="setting-field"><span>相关度阈值</span><input type="number" min={0} max={100} step={0.5} value={settings.minScore} onChange={(event) => setSettings((current) => ({ ...current, minScore: Number(event.target.value) }))} /></label>
        <label className="setting-field"><span>单次最大入库数</span><input type="number" min={1} max={20} value={settings.maxAutoIngest} onChange={(event) => setSettings((current) => ({ ...current, maxAutoIngest: Number(event.target.value) }))} /></label>
        <label className="setting-field"><span>起始年份</span><input type="number" min={1990} max={2100} value={settings.sinceYear ?? ''} onChange={(event) => setSettings((current) => ({ ...current, sinceYear: event.target.value ? Number(event.target.value) : null }))} /></label>
        <fieldset><legend>检索来源</legend>{['arxiv', 'openalex', 'tavily', 'serpapi'].map((provider) => <label className="provider-option" key={provider}><input type="checkbox" checked={settings.providers.includes(provider)} onChange={(event) => setSettings((current) => ({ ...current, providers: event.target.checked ? [...current.providers, provider] : current.providers.filter((item) => item !== provider) }))} />{provider}</label>)}</fieldset>
        <div className="automation-history"><span>上次尝试：{formatEpoch(settings.lastAttemptAt)}</span><span>上次成功：{formatEpoch(settings.lastSuccessAt)}</span></div>
      </aside>
    </div>}

    {(busy || runLog.length > 0) && <section className="ingest-run-strip"><div><span>{busy ? <LoaderCircle className="spin" size={15} /> : <CheckCircle2 size={15} />}{busy ? '任务运行中' : '最近任务输出'}</span><button onClick={onOpenCompileCenter}>在编译中心查看 · {runId || '等待任务编号'} <ChevronRight size={14} /></button></div><pre>{runLog.slice(-8).join('\n') || '正在等待任务事件…'}</pre></section>}
  </section>
}
