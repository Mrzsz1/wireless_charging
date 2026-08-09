import { Radar, ShieldCheck, X } from 'lucide-react'
import type { StartupPromptState } from '../../types'

type Props = {
  prompt: StartupPromptState
  busy: boolean
  onRun: () => void
  onSuppressToday: () => void
  onCancel: () => void
}

export function StartupIngestPrompt({ prompt, busy, onRun, onSuppressToday, onCancel }: Props) {
  const fullIngest = prompt.mode === 'automatic'
  return <div className="ingest-modal-backdrop" role="presentation">
    <section className="ingest-modal" role="dialog" aria-modal="true" aria-labelledby="ingest-startup-title" data-testid="ingest-startup-prompt">
      <button className="ingest-modal-close" aria-label="关闭" disabled={busy} onClick={onCancel}><X size={17} /></button>
      <div className="ingest-modal-icon"><Radar size={25} /></div>
      <h2 id="ingest-startup-title">检查最新研究文献？</h2>
      <p>{fullIngest
        ? `将检索候选，并把最多 ${prompt.settings.maxAutoIngest} 篇满足全部规则的开放论文完整入库。`
        : '将检索并下载候选清单，不写入正式 Wiki；你可以随后逐篇确认添加。'}</p>
      <div className="ingest-modal-safety"><ShieldCheck size={15} /><span>{fullIngest ? '完整入库已在“设置 → 文献自动化”中显式启用' : '当前采用默认的自动准备模式'}</span></div>
      <div className="ingest-modal-actions">
        <button data-testid="ingest-prompt-suppress" className="ingest-button secondary" disabled={busy} onClick={onSuppressToday}>今天不再提醒</button>
        <button data-testid="ingest-prompt-cancel" className="ingest-button secondary" disabled={busy} onClick={onCancel}>取消</button>
        <button data-testid="ingest-prompt-run" className="ingest-button primary" disabled={busy} onClick={onRun}>{busy ? '正在启动…' : '本次运行'}</button>
      </div>
    </section>
  </div>
}
