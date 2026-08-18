import type { LiteratureCandidate, LiteratureCapability, ManualImportSession } from '../../types'

export type CandidateFilter = 'all' | 'pending' | 'selected' | 'rejected' | 'promoted' | 'eligible'

export function automationCapabilitiesReady(capabilities: LiteratureCapability[], automatic: boolean): boolean {
  const required: LiteratureCapability['id'][] = automatic ? ['full_ingest'] : ['discovery', 'download']
  return required.every((id) => capabilities.some((capability) => capability.id === id && capability.available))
}

export function localDateKey(date = new Date()): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

export function defaultSelectedManualFileIds(session: ManualImportSession | null): string[] {
  return session?.files.filter((file) => file.valid && file.selected && file.duplicateMatches.length === 0).map((file) => file.id) ?? []
}

export function filterCandidates(candidates: LiteratureCandidate[], query: string, filter: CandidateFilter): LiteratureCandidate[] {
  const needle = query.trim().toLocaleLowerCase('zh-CN')
  return candidates.filter((candidate) => {
    if (filter === 'eligible' && !candidate.qualification.eligible) return false
    if (filter !== 'all' && filter !== 'eligible' && candidate.triageStatus !== filter) return false
    if (!needle) return true
    return [candidate.title, candidate.abstract, candidate.venue, candidate.doi, candidate.arxivId, ...(candidate.authors ?? [])]
      .join(' ')
      .toLocaleLowerCase('zh-CN')
      .includes(needle)
  })
}

export function formatBytes(bytes: number): string {
  if (!Number.isFinite(bytes) || bytes < 0) return '—'
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 ** 2) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / 1024 ** 2).toFixed(1)} MB`
}

export function formatEpoch(value: string): string {
  if (!value) return '尚未运行'
  const numeric = Number(value)
  const date = Number.isFinite(numeric) ? new Date(numeric * 1000) : new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString('zh-CN', { hour12: false })
}
