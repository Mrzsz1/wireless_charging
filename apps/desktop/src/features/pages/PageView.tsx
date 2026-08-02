import { useMemo } from 'react'
import { BookOpen, ExternalLink, FileText, FolderOpen, Link2, RefreshCw, RotateCcw } from 'lucide-react'
import type { Backlink, PageDetail } from '../../types'
import { MarkdownReader } from './MarkdownReader'

type PageViewProps = {
  page: PageDetail
  backlinks: Backlink[]
  backlinksLoading: boolean
  onOpenLink: (target: string) => void
  onOpenPath: (path: string, reveal?: boolean) => void
  onReload: () => void
}

const fieldLabels: Record<string, string> = {
  scenario: '场景',
  entities: '实体',
  constraints: '约束',
  objectives: '目标',
  method_family: '方法族',
  problem_class: '问题类别',
  epistemic: '证据等级',
  status: '状态',
}

function displayField(value: string) {
  const trimmed = value.trim()
  if (!trimmed) return ''
  if (trimmed.startsWith('[') || trimmed.startsWith('{')) {
    try {
      const parsed = JSON.parse(trimmed) as unknown
      return Array.isArray(parsed) ? parsed.join('、') : JSON.stringify(parsed)
    } catch { return trimmed }
  }
  return trimmed.replace(/^\[|\]$/g, '').replace(/['"]/g, '').replace(/,/g, '、')
}

export function PageView({ page, backlinks, backlinksLoading, onOpenLink, onOpenPath, onReload }: PageViewProps) {
  const metadata = useMemo(() => Object.entries(page.frontmatter).filter(([key, value]) => fieldLabels[key] && displayField(value)), [page.frontmatter])
  const pdfPath = page.frontmatter.pdf_path || ''
  const rawMd = page.frontmatter.raw_md || ''
  return <section className="page-view">
    <div className="page-heading page-detail-heading">
      <div className="page-title-block"><div className="eyebrow">{page.pageType.toUpperCase()} · {page.year || '年份未记录'}</div><h1>{page.title}</h1><p>{page.summary || '本页尚未记录摘要。'}</p></div>
      <div className="heading-actions"><button className="refresh-button" onClick={onReload}><RefreshCw size={15} />重新读取</button>{pdfPath && <button className="refresh-button" onClick={() => onOpenPath(pdfPath)}><BookOpen size={15} />打开 PDF</button>}{rawMd && <button className="refresh-button" onClick={() => onOpenPath(rawMd)}><FileText size={15} />打开原始 Markdown</button>}</div>
    </div>
    <div className="page-meta-strip"><span><FileText size={14} />{page.sourcePath}</span><span>{page.modifiedAt ? `更新于 ${page.modifiedAt}` : '更新时间未记录'}</span></div>
    <div className="page-layout">
      <div className="page-main-column">
        {metadata.length > 0 && <section className="page-section page-facts"><div className="section-header"><h2>结构化字段</h2><span className="page-section-note">来自 frontmatter</span></div><div className="fact-grid">{metadata.map(([key, value]) => <div className="fact-item" key={key}><span>{fieldLabels[key]}</span><strong>{displayField(value)}</strong></div>)}</div></section>}
        <section className="page-section"><div className="section-header"><h2>正文</h2><button className="link-button" onClick={() => onOpenPath(page.sourcePath, true)}><FolderOpen size={14} />在文件夹中显示</button></div><MarkdownReader body={page.body} sourcePath={page.sourcePath} onOpenLink={onOpenLink} /></section>
        <section className="page-section page-links-section"><div className="section-header"><h2>出链</h2><span className="page-section-note">{page.links.length} 条内部链接</span></div>{page.links.length ? <div className="page-link-list">{page.links.map((link) => <button className="page-link-chip" key={link} onClick={() => onOpenLink(link)}><Link2 size={13} />{link}</button>)}</div> : <p className="page-muted">本页没有记录内部链接。</p>}</section>
      </div>
      <aside className="page-side-column">
        <section className="page-side-section"><div className="section-header"><h2>反向链接</h2>{backlinksLoading && <RotateCcw size={14} className="spin" />}</div>{backlinks.length ? <div className="backlink-list">{backlinks.map((item) => <button className="backlink-item" key={`${item.source.id}-${item.target}`} onClick={() => onOpenLink(item.source.id)}><span className="backlink-type">{item.source.pageType}</span><strong>{item.source.title}</strong><small>引用 {item.target}</small></button>)}</div> : <p className="page-muted">没有发现反向链接。</p>}</section>
        <section className="page-side-section"><div className="section-header"><h2>页面信息</h2></div><dl className="page-info-list"><dt>页面 ID</dt><dd>{page.id}</dd><dt>类型</dt><dd>{page.pageType}</dd><dt>来源文件</dt><dd title={page.sourcePath}>{page.sourcePath.split(/[\\/]/).pop()}</dd></dl></section>
        <section className="page-side-section page-readonly-note"><ExternalLink size={15} /><span>正文来自 Wiki，只读展示。修改请回到受控编译流程。</span></section>
      </aside>
    </div>
  </section>
}
