import { createElement, useMemo, type ReactNode } from 'react'
import { convertFileSrc } from '@tauri-apps/api/core'
import { ExternalLink, FileImage, Link2 } from 'lucide-react'
import { isDesktopRuntime } from '../../services/desktop'

type MarkdownReaderProps = {
  body: string
  sourcePath: string
  onOpenLink: (target: string) => void
}

function safeLocalImagePath(sourcePath: string, target: string) {
  if (/^(https?:|data:|asset:)/i.test(target)) return target
  const cleanTarget = target.split('#')[0].trim()
  if (!cleanTarget || cleanTarget.includes('\0')) return ''
  const base = sourcePath.replace(/[\\/][^\\/]*$/, '').replace(/\\/g, '/')
  const segments = `${base}/${cleanTarget.replace(/\\/g, '/')}`.split('/')
  const safe: string[] = []
  for (const segment of segments) {
    if (!segment || segment === '.') continue
    if (segment === '..') return ''
    safe.push(segment)
  }
  const absolutePath = safe.join('/')
  return isDesktopRuntime() ? convertFileSrc(absolutePath) : ''
}

function inlineText(text: string, onOpenLink: (target: string) => void, sourcePath: string): ReactNode[] {
  const tokenPattern = /(\[\[[^\]]+\]\]|!\[[^\]]*\]\([^)]*\)|\[[^\]]+\]\([^)]*\)|`[^`]+`|\*\*[^*]+\*\*|\$[^$]+\$)/g
  const parts = text.split(tokenPattern)
  return parts.map((part, index) => {
    if (!part) return null
    const key = `${part}-${index}`
    if (part.startsWith('[[')) {
      const raw = part.slice(2, -2)
      const target = raw.split('|')[0].trim()
      const label = raw.split('|')[1]?.trim() || target
      return <button key={key} className="markdown-wikilink" onClick={() => onOpenLink(target)}><Link2 size={12} />{label}</button>
    }
    const image = part.match(/^!\[([^]]*)\]\(([^)]+)\)$/)
    if (image) {
      const [_, alt, target] = image
      const source = safeLocalImagePath(sourcePath, target)
      return source ? <img key={key} className="markdown-image" src={source} alt={alt || target} /> : <span key={key} className="markdown-image-placeholder"><FileImage size={14} />{target}</span>
    }
    const link = part.match(/^\[([^]]+)\]\(([^)]+)\)$/)
    if (link) {
      const [_, label, target] = link
      return <a key={key} className="markdown-link" href={target} target="_blank" rel="noreferrer">{label}<ExternalLink size={11} /></a>
    }
    if (part.startsWith('`')) return <code key={key} className="markdown-inline-code">{part.slice(1, -1)}</code>
    if (part.startsWith('$')) return <span key={key} className="markdown-inline-math" role="math">{part.slice(1, -1)}</span>
    if (part.startsWith('**')) return <strong key={key}>{part.slice(2, -2)}</strong>
    return <span key={key}>{part}</span>
  })
}

function isTableSeparator(line: string) {
  return /^\s*\|?\s*:?-{3,}:?\s*(\|\s*:?-{3,}:?\s*)+\|?\s*$/.test(line)
}

function tableCells(line: string) {
  return line.trim().replace(/^\|/, '').replace(/\|$/, '').split('|').map((cell) => cell.trim())
}

export function MarkdownReader({ body, sourcePath, onOpenLink }: MarkdownReaderProps) {
  const blocks = useMemo(() => {
    const lines = body.replace(/\r\n/g, '\n').split('\n')
    const result: React.ReactNode[] = []
    let index = 0
    let blockKey = 0
    while (index < lines.length) {
      const line = lines[index]
      const key = `markdown-block-${blockKey++}`
      if (!line.trim()) { index += 1; continue }
      if (line.trim().startsWith('```')) {
        const language = line.trim().slice(3).trim()
        const code: string[] = []
        index += 1
        while (index < lines.length && !lines[index].trim().startsWith('```')) { code.push(lines[index]); index += 1 }
        index += 1
        result.push(<pre key={key} className="markdown-code"><code data-language={language || undefined}>{code.join('\n')}</code></pre>)
        continue
      }
      if (line.trim() === '$$') {
        const formula: string[] = []
        index += 1
        while (index < lines.length && lines[index].trim() !== '$$') { formula.push(lines[index]); index += 1 }
        index += 1
        result.push(<div key={key} className="markdown-math" role="math">{formula.join('\n')}</div>)
        continue
      }
      const heading = line.match(/^(#{1,6})\s+(.+)$/)
      if (heading) {
        const level = heading[1].length
        result.push(createElement(`h${level}`, { key, className: 'markdown-heading' }, inlineText(heading[2], onOpenLink, sourcePath)))
        index += 1
        continue
      }
      if (line.includes('|') && index + 1 < lines.length && isTableSeparator(lines[index + 1])) {
        const header = tableCells(line)
        const rows: string[][] = []
        index += 2
        while (index < lines.length && lines[index].includes('|') && lines[index].trim()) { rows.push(tableCells(lines[index])); index += 1 }
        result.push(<div key={key} className="markdown-table-wrap"><table className="markdown-table"><thead><tr>{header.map((cell, cellIndex) => <th key={`${key}-h-${cellIndex}`}>{inlineText(cell, onOpenLink, sourcePath)}</th>)}</tr></thead><tbody>{rows.map((row, rowIndex) => <tr key={`${key}-r-${rowIndex}`}>{header.map((_, cellIndex) => <td key={`${key}-c-${cellIndex}`}>{inlineText(row[cellIndex] || '', onOpenLink, sourcePath)}</td>)}</tr>)}</tbody></table></div>)
        continue
      }
      if (/^\s*[-*+]\s+/.test(line)) {
        const items: string[] = []
        while (index < lines.length && /^\s*[-*+]\s+/.test(lines[index])) { items.push(lines[index].replace(/^\s*[-*+]\s+/, '')); index += 1 }
        result.push(<ul key={key} className="markdown-list">{items.map((item, itemIndex) => <li key={`${key}-${itemIndex}`}>{inlineText(item, onOpenLink, sourcePath)}</li>)}</ul>)
        continue
      }
      if (/^\s*\d+[.)]\s+/.test(line)) {
        const items: string[] = []
        while (index < lines.length && /^\s*\d+[.)]\s+/.test(lines[index])) { items.push(lines[index].replace(/^\s*\d+[.)]\s+/, '')); index += 1 }
        result.push(<ol key={key} className="markdown-list">{items.map((item, itemIndex) => <li key={`${key}-${itemIndex}`}>{inlineText(item, onOpenLink, sourcePath)}</li>)}</ol>)
        continue
      }
      if (/^\s*>/.test(line)) {
        const quote: string[] = []
        while (index < lines.length && /^\s*>/.test(lines[index])) { quote.push(lines[index].replace(/^\s*>\s?/, '')); index += 1 }
        result.push(<blockquote key={key} className="markdown-quote">{quote.map((item, itemIndex) => <div key={`${key}-${itemIndex}`}>{inlineText(item, onOpenLink, sourcePath)}</div>)}</blockquote>)
        continue
      }
      const paragraph: string[] = [line]
      index += 1
      while (index < lines.length && lines[index].trim() && !/^(#{1,6})\s+/.test(lines[index]) && !lines[index].trim().startsWith('```') && !/^\s*[-*+]\s+/.test(lines[index]) && !/^\s*\d+[.)]\s+/.test(lines[index])) { paragraph.push(lines[index]); index += 1 }
      result.push(<p key={key} className="markdown-paragraph">{paragraph.flatMap((item, itemIndex) => [itemIndex > 0 && <br key={`${key}-br-${itemIndex}`} />, inlineText(item, onOpenLink, sourcePath)])}</p>)
    }
    return result
  }, [body, onOpenLink, sourcePath])

  return <article className="markdown-reader">{blocks.length ? blocks : <div className="markdown-empty">本页没有可展示的正文。</div>}</article>
}
