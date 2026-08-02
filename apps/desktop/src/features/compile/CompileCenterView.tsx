import { useEffect, useMemo, useRef, useState } from 'react'
import { AlertTriangle, ArchiveRestore, CheckCircle2, CircleStop, Clock3, CloudDownload, Code2, FileSearch, FileText, FolderOpen, LoaderCircle, Network, Play, RefreshCw, RotateCcw, ShieldCheck, Terminal } from 'lucide-react'
import { cancelCompileRun, getCompileCapabilities, getCompileRun, listCompileRuns, retryCompileRun, rollbackCompileRun, startCompileRun } from '../../services/desktop'
import type { CompileCapability, CompileRunDetail, CompileRunEvent, CompileRunSummary, CompileStreamEvent } from '../../types'
import './CompileCenterView.css'

type Props = {
  repositoryPath?: string
  onChooseRepository: () => void
  onOpenPath: (path: string, reveal?: boolean) => void
}

const statusLabels: Record<string, string> = {
  queued: '等待中', running: '运行中', succeeded: '已完成', failed: '失败', cancelled: '已取消', interrupted: '已中断', rolled_back: '已回滚',
}

const taskIcons = { lint: ShieldCheck, graphify_update: Network, discover: CloudDownload, parse: FileSearch, compile_a: Code2 }

function formatTime(value: string) {
  if (!value) return '—'
  const numeric = Number(value)
  const date = Number.isFinite(numeric) ? new Date(numeric * 1000) : new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString('zh-CN', { hour12: false })
}

export function CompileCenterView({ repositoryPath, onChooseRepository, onOpenPath }: Props) {
  const [capabilities, setCapabilities] = useState<CompileCapability[]>([])
  const [runs, setRuns] = useState<CompileRunSummary[]>([])
  const [selectedKind, setSelectedKind] = useState<CompileCapability['taskKind']>('lint')
  const [selectedRunId, setSelectedRunId] = useState('')
  const [detail, setDetail] = useState<CompileRunDetail | null>(null)
  const [inputPath, setInputPath] = useState('raw/inbox')
  const [dryRun, setDryRun] = useState(true)
  const [download, setDownload] = useState(false)
  const [force, setForce] = useState(false)
  const [liveEvents, setLiveEvents] = useState<CompileRunEvent[]>([])
  const [runningId, setRunningId] = useState('')
  const [busy, setBusy] = useState(false)
  const [error, setError] = useState('')
  const logRef = useRef<HTMLDivElement>(null)

  const selectedCapability = useMemo(() => capabilities.find((item) => item.taskKind === selectedKind), [capabilities, selectedKind])
  const displayedEvents = runningId ? liveEvents : detail?.events ?? []

  const refresh = async () => {
    if (!repositoryPath) return
    try {
      const [nextCapabilities, nextRuns] = await Promise.all([getCompileCapabilities(), listCompileRuns()])
      setCapabilities(nextCapabilities)
      setRuns(nextRuns)
      if (!nextCapabilities.some((item) => item.taskKind === selectedKind)) setSelectedKind(nextCapabilities[0]?.taskKind ?? 'lint')
      if (!selectedRunId && nextRuns[0]) setSelectedRunId(nextRuns[0].id)
      setError('')
    } catch (reason) { setError(String(reason)) }
  }

  useEffect(() => { setCapabilities([]); setRuns([]); setDetail(null); setSelectedRunId(''); if (repositoryPath) void refresh() }, [repositoryPath])
  useEffect(() => { if (!selectedRunId || runningId) return; void getCompileRun(selectedRunId).then(setDetail).catch((reason) => setError(String(reason))) }, [runningId, selectedRunId])
  useEffect(() => { logRef.current?.scrollTo({ top: logRef.current.scrollHeight }) }, [displayedEvents.length])

  const acceptEvent = (event: CompileStreamEvent) => {
    if (event.type === 'accepted') setRunningId(event.runId)
    setLiveEvents((current) => current.some((item) => item.sequence === event.sequence) ? current : [...current, { sequence: event.sequence, eventKind: event.type, stage: event.stage, message: event.message, createdAt: event.timestamp }].sort((a, b) => a.sequence - b.sequence))
  }

  const run = async (retryId?: string) => {
    if (!repositoryPath || busy || !selectedCapability?.available) return
    if (selectedCapability.writes && !dryRun && !window.confirm(`运行“${selectedCapability.label}”会写入当前知识库的允许区域。继续吗？`)) return
    setBusy(true); setError(''); setLiveEvents([]); setDetail(null); setRunningId('pending')
    try {
      const result = retryId
        ? await retryCompileRun(retryId, acceptEvent)
        : await startCompileRun({ taskKind: selectedKind, inputPath: selectedKind === 'parse' ? inputPath : undefined, dryRun, download, force }, acceptEvent)
      setRunningId('')
      setSelectedRunId(result.id)
      await refresh()
      setDetail(await getCompileRun(result.id))
    } catch (reason) { setError(String(reason)); setRunningId(''); await refresh() }
    finally { setBusy(false) }
  }

  const stop = async () => {
    if (!runningId || runningId === 'pending') return
    try { await cancelCompileRun(runningId) } catch (reason) { setError(String(reason)) }
  }

  const rollback = async () => {
    if (!detail) return
    try { setError(await rollbackCompileRun(detail.summary.id)) } catch (reason) { setError(String(reason)) }
  }

  if (!repositoryPath) return <section className="compile-empty"><Code2 size={34} /><h1>编译中心</h1><p>选择知识库后才能运行发现、解析、Lint、A 编译与 Graphify 更新任务。</p><button className="refresh-button" onClick={onChooseRepository}><FolderOpen size={16} />选择知识库</button></section>

  return <section className="compile-center-view">
    <header className="compile-center-heading"><div><div className="eyebrow">AUDITABLE LOCAL PIPELINE</div><h1>编译中心</h1><p>受控运行知识库任务，保留命令阶段、退出码、日志、失败原因和生成物。</p></div><button className="refresh-button" onClick={() => void refresh()}><RefreshCw size={15} />刷新</button></header>
    {error && <div className="compile-alert"><AlertTriangle size={15} /><span>{error}</span><button onClick={() => setError('')}>关闭</button></div>}
    <div className="compile-center-layout">
      <aside className="compile-task-catalog">
        <div className="compile-column-title"><span>任务目录</span><small>{capabilities.filter((item) => item.available).length}/{capabilities.length} 可用</small></div>
        {capabilities.map((capability) => {
          const Icon = taskIcons[capability.taskKind]
          return <button key={capability.taskKind} className={`compile-task-card ${selectedKind === capability.taskKind ? 'selected' : ''}`} onClick={() => setSelectedKind(capability.taskKind)}>
            <Icon size={17} /><span><strong>{capability.label}</strong><small>{capability.description}</small><em className={capability.available ? 'available' : 'unavailable'}>{capability.reason}</em></span>
          </button>
        })}
        <div className="compile-boundary"><ShieldCheck size={15} /><span>固定命令允许列表<br />Raw/Wiki 治理边界保持不变</span></div>
      </aside>

      <main className="compile-run-column">
        <div className="compile-column-title"><span>任务运行</span><small>{runs.length} 条历史</small></div>
        <div className="compile-run-list">
          {runs.map((item) => <button key={item.id} className={`compile-run-item ${selectedRunId === item.id && !runningId ? 'selected' : ''}`} onClick={() => { if (!runningId) setSelectedRunId(item.id) }}>
            <span className={`run-status-dot ${item.status}`} />
            <span><strong>{item.displayName}</strong><small>{formatTime(item.createdAt)} · {statusLabels[item.status] ?? item.status}</small></span>
            {item.status === 'succeeded' ? <CheckCircle2 size={15} /> : item.status === 'running' ? <LoaderCircle size={15} className="spin" /> : <Clock3 size={15} />}
          </button>)}
          {!runs.length && !busy && <div className="compile-list-empty"><Terminal size={22} /><span>还没有任务记录</span></div>}
        </div>
        <div className="compile-log-toolbar"><span><Terminal size={14} />实时日志</span>{busy && <button onClick={() => void stop()}><CircleStop size={14} />停止</button>}</div>
        <div className="compile-log" ref={logRef} role="log" aria-live="polite">
          {displayedEvents.map((event) => <div className={`compile-log-line ${event.eventKind}`} key={event.sequence}><time>{event.sequence.toString().padStart(3, '0')}</time><b>{event.stage}</b><span>{event.message}</span></div>)}
          {!displayedEvents.length && <div className="compile-log-empty">选择历史任务或开始运行以查看日志。</div>}
        </div>
      </main>

      <aside className="compile-inspector">
        <div className="compile-column-title"><span>{runningId ? '当前运行' : detail ? '任务详情' : '运行参数'}</span></div>
        {!runningId && <div className="compile-parameters">
          <label><span>任务</span><strong>{selectedCapability?.label ?? '—'}</strong></label>
          {selectedCapability?.requiresInput && <label><span>知识库内 PDF 或目录</span><input value={inputPath} onChange={(event) => setInputPath(event.target.value)} placeholder="raw/inbox/manual-drop" /></label>}
          <label className="compile-check"><input type="checkbox" checked={dryRun} onChange={(event) => setDryRun(event.target.checked)} /><span>Dry run（不联网/不写正文）</span></label>
          {selectedKind === 'discover' && <label className="compile-check"><input type="checkbox" checked={download} onChange={(event) => setDownload(event.target.checked)} /><span>下载开放 PDF</span></label>}
          {selectedKind === 'parse' && <label className="compile-check"><input type="checkbox" checked={force} onChange={(event) => setForce(event.target.checked)} /><span>覆盖同名解析结果</span></label>}
          <button className="compile-primary" disabled={busy || !selectedCapability?.available} onClick={() => void run()}><Play size={15} />开始运行</button>
        </div>}
        {detail && !runningId && <div className="compile-detail">
          <dl><dt>状态</dt><dd>{statusLabels[detail.summary.status] ?? detail.summary.status}</dd><dt>阶段</dt><dd>{detail.summary.currentStage}</dd><dt>开始</dt><dd>{formatTime(detail.summary.startedAt)}</dd><dt>结束</dt><dd>{formatTime(detail.summary.finishedAt)}</dd><dt>退出码</dt><dd>{detail.summary.exitCode ?? '—'}</dd></dl>
          {detail.summary.failureReason && <p className="compile-failure">{detail.summary.failureReason}</p>}
          <div className="compile-detail-actions"><button onClick={() => void run(detail.summary.id)}><RotateCcw size={14} />相同参数重试</button><button onClick={() => void rollback()}><ArchiveRestore size={14} />回滚入口</button></div>
          <h3>生成物</h3>
          {detail.artifacts.map((artifact) => <button className="compile-artifact" key={artifact.id} onClick={() => onOpenPath(artifact.relativePath, true)}><FileText size={14} /><span>{artifact.relativePath}</span><small>{artifact.operation}</small></button>)}
          {!detail.artifacts.length && <p className="compile-muted">未登记生成物。</p>}
        </div>}
        {runningId && <div className="compile-running"><LoaderCircle size={26} className="spin" /><strong>任务正在运行</strong><span>关闭页面不会删除任务记录；可在日志区停止。</span></div>}
      </aside>
    </div>
  </section>
}
