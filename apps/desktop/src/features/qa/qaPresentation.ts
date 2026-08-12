import type { ChatMessage, ChatSessionSummary, CitationValidation, EvidenceItem, WaterlineSnapshot } from '../../types'

export type QaPhase = 'idle' | 'retrieving' | 'generating'

export type EvidenceEmptyState = {
  title: string
  detail: string
  kind: 'waiting' | 'none' | 'initial'
}

export function evidenceEmptyState(phase: QaPhase, waterline: WaterlineSnapshot | null, evidenceCount: number): EvidenceEmptyState | null {
  if (evidenceCount > 0) return null
  if (phase === 'retrieving') return { title: '正在检索', detail: '正在查询 Wiki、论文原文、核心书籍和 Graphify。', kind: 'waiting' }
  if (waterline) return { title: '本轮未检索到参考来源', detail: '回答已标记为未验证，不会进入后续对话上下文。', kind: 'none' }
  return { title: '等待提问', detail: '提问后在这里核验引用、页码和排序理由。', kind: 'initial' }
}

export type CitationSummary = {
  tone: 'supported' | 'unverified' | 'invalid'
  label: string
  detail: string
}

export function citationSummary(validation?: CitationValidation | null): CitationSummary | null {
  if (!validation) return null
  if (validation.groundingStatus === 'unverified') {
    return { tone: 'unverified', label: '无参考来源 · 未验证', detail: '本轮内容不进入后续对话上下文。' }
  }
  if (!validation.supported) {
    const unsupportedClaims = validation.unsupportedClaims ?? []
    const graphOnlyClaims = validation.graphOnlyClaims ?? []
    return {
      tone: 'invalid',
      label: '引用校验未通过',
      detail: `${unsupportedClaims.length} 条事实缺少有效引用；${graphOnlyClaims.length} 条仅由图谱提示支撑。`,
    }
  }
  const claimCount = validation.claimCount ?? 0
  const citedClaimCount = validation.citedClaimCount ?? 0
  const coverage = validation.citationCoverage ?? 0
  return {
    tone: 'supported',
    label: claimCount > 0 ? `引用覆盖 ${Math.round(coverage * 100)}%` : '引用编号已校验',
    detail: claimCount > 0
      ? `${citedClaimCount}/${claimCount} 条事实已绑定本轮非图谱证据；语义未自动核验。`
      : '这是旧版校验记录；引用编号已校验，逐条覆盖与语义未自动核验。',
  }
}

export function linkEvidenceCitations(content: string): string {
  let result = ''
  let index = 0
  while (index < content.length) {
    const marker = content[index]
    // Preserve code spans/fences and math source verbatim. Citation-looking
    // tokens inside executable examples or formulas are content, not evidence.
    if ((marker === '`' || marker === '$') && content[index - 1] !== '\\') {
      let width = 1
      while (content[index + width] === marker) width += 1
      const delimiter = marker.repeat(width)
      const closing = content.indexOf(delimiter, index + width)
      if (closing >= 0) {
        result += content.slice(index, closing + width)
        index = closing + width
        continue
      }
    }
    if (marker === '[' && content[index - 1] !== '\\') {
      const match = content.slice(index).match(/^\[(E\d+)\]/)
      if (match) {
        const end = index + match[0].length
        // Do not create nested links when persisted Markdown already linked the
        // label explicitly.
        result += content[end] === '(' ? match[0] : `[${match[1]}](evidence:${match[1]})`
        index = end
        continue
      }
    }
    result += marker
    index += 1
  }
  return result
}

export function evidenceById(evidence: EvidenceItem[]): Map<string, EvidenceItem> {
  return new Map(evidence.map((item) => [item.id, item]))
}

export function appendUniqueSessions(current: ChatSessionSummary[], incoming: ChatSessionSummary[]): ChatSessionSummary[] {
  const seen = new Set(current.map((item) => item.id))
  return [...current, ...incoming.filter((item) => !seen.has(item.id))]
}

export function prependUniqueMessages(current: ChatMessage[], incoming: ChatMessage[]): ChatMessage[] {
  const seen = new Set(current.map((item) => item.id))
  return [...incoming.filter((item) => !seen.has(item.id)), ...current]
}

export function buildAuditBundle(question: string, message: ChatMessage): string {
  return JSON.stringify({
    schemaVersion: 'qa-audit-bundle-v1',
    question,
    answer: message.content,
    evidence: message.evidence,
    runManifest: message.runManifest ?? null,
  }, null, 2)
}
