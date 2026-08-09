import type { ResearchTrailItem } from '../../types'

export const RESEARCH_TRAIL_PINS_KEY = 'desktop.research-trail.pins.v1'

export type ResearchTrailPinStore = {
  version: 1
  repositories: Record<string, Record<string, ResearchTrailItem[]>>
}

const emptyStore = (): ResearchTrailPinStore => ({ version: 1, repositories: {} })

function isItem(value: unknown): value is ResearchTrailItem {
  if (!value || typeof value !== 'object') return false
  const item = value as Partial<ResearchTrailItem>
  return typeof item.id === 'string'
    && (item.kind === 'wiki' || item.kind === 'book' || item.kind === 'graph')
    && typeof item.title === 'string'
    && typeof item.score === 'number'
    && Number.isFinite(item.score)
    && typeof item.relation === 'string'
    && typeof item.retrievalReason === 'string'
}

export function parseResearchTrailPins(raw: string | null): ResearchTrailPinStore {
  if (!raw) return emptyStore()
  try {
    const value = JSON.parse(raw) as Partial<ResearchTrailPinStore>
    if (value.version !== 1 || !value.repositories || typeof value.repositories !== 'object') return emptyStore()
    const repositories: ResearchTrailPinStore['repositories'] = {}
    for (const [repository, contexts] of Object.entries(value.repositories)) {
      if (!contexts || typeof contexts !== 'object') continue
      repositories[repository] = {}
      for (const [context, items] of Object.entries(contexts)) {
        repositories[repository][context] = Array.isArray(items) ? items.filter(isItem).slice(0, 30) : []
      }
    }
    return { version: 1, repositories }
  } catch {
    return emptyStore()
  }
}

export function pinsForContext(store: ResearchTrailPinStore, repository: string, contextKey: string) {
  return store.repositories[repository]?.[contextKey] ?? []
}

export function toggleResearchTrailPin(store: ResearchTrailPinStore, repository: string, contextKey: string, item: ResearchTrailItem) {
  const current = pinsForContext(store, repository, contextKey)
  const exists = current.some((candidate) => candidate.kind === item.kind && candidate.id === item.id)
  const next = exists ? current.filter((candidate) => candidate.kind !== item.kind || candidate.id !== item.id) : [...current, item]
  return {
    ...store,
    repositories: {
      ...store.repositories,
      [repository]: { ...store.repositories[repository], [contextKey]: next },
    },
  }
}

export function mergePinnedItems(pinned: ResearchTrailItem[], ranked: ResearchTrailItem[]) {
  const keys = new Set(pinned.map((item) => `${item.kind}:${item.id}`))
  return [...pinned, ...ranked.filter((item) => !keys.has(`${item.kind}:${item.id}`))]
}
