import { useMemo } from 'react'
import ReactMarkdown from 'react-markdown'
import rehypeKatex from 'rehype-katex'
import remarkGfm from 'remark-gfm'
import remarkMath from 'remark-math'
import type { EvidenceItem } from '../../types'
import { evidenceById, linkEvidenceCitations } from './qaPresentation'
import 'katex/dist/katex.min.css'

type MarkdownMessageProps = {
  content: string
  evidence: EvidenceItem[]
  onCitation: (item: EvidenceItem) => void
}

export default function MarkdownMessage({ content, evidence, onCitation }: MarkdownMessageProps) {
  const byId = useMemo(() => evidenceById(evidence), [evidence])
  const markdown = useMemo(() => linkEvidenceCitations(content), [content])
  return <ReactMarkdown
    remarkPlugins={[remarkGfm, remarkMath]}
    rehypePlugins={[rehypeKatex]}
    urlTransform={(url) => url.startsWith('evidence:') ? url : /^https?:\/\//i.test(url) ? url : ''}
    components={{
      a: ({ href, children }) => {
        if (href?.startsWith('evidence:')) {
          const id = href.slice('evidence:'.length)
          const item = byId.get(id)
          return item
            ? <button type="button" className="qa-inline-citation" onClick={() => onCitation(item)}>[{id}]</button>
            : <span className="qa-invalid-citation" title="回答引用了未登记的证据">[{id}]</span>
        }
        return href ? <a href={href} target="_blank" rel="noreferrer noopener">{children}</a> : <span>{children}</span>
      },
      img: ({ alt }) => <span className="qa-image-placeholder">[图片：{alt || '未命名'}]</span>,
    }}
  >{markdown}</ReactMarkdown>
}
