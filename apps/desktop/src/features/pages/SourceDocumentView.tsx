import { FileText, FolderOpen, Info } from 'lucide-react'
import type { ResolvedSourceDocument } from '../../types'
import { MarkdownReader } from './MarkdownReader'

type SourceDocumentViewProps = {
  document: ResolvedSourceDocument
  onOpenLink: (target: string) => void
  onOpenPath: (path: string, reveal?: boolean) => void
}

export function SourceDocumentView({ document, onOpenLink, onOpenPath }: SourceDocumentViewProps) {
  const { location } = document
  return <section className="page-view" data-testid="source-document-view">
    <div className="page-heading page-detail-heading">
      <div className="page-title-block">
        <h1>{document.title}</h1>
        <p>{location.headingPath.length ? location.headingPath.join(' › ') : 'Markdown 来源文档'}</p>
      </div>
      <div className="heading-actions">
        <button className="refresh-button" onClick={() => onOpenPath(location.markdownPath, true)}><FolderOpen size={14} />在文件夹中显示</button>
      </div>
    </div>
    {location.degradedReason && <div className="notice"><Info size={15} /><span>{location.degradedReason}</span></div>}
    <div className="page-meta-strip">
      <span><FileText size={14} />{location.matchedBy === 'block' ? '稳定内容块' : `降级定位：${location.matchedBy}`}</span>
      <span>{location.lineStart ? `Markdown 第 ${location.lineStart}${location.lineEnd ? `–${location.lineEnd}` : ''} 行` : '文档定位'}</span>
    </div>
    <section className="page-section">
      <div className="section-header"><h2>来源正文</h2><span className="page-section-note">只读 · 已定位</span></div>
      <MarkdownReader
        body={document.body}
        sourcePath={location.markdownPath}
        onOpenLink={onOpenLink}
        focus={{ headingPath: location.headingPath, lineStart: location.lineStart, matchedBy: location.matchedBy }}
      />
    </section>
  </section>
}
