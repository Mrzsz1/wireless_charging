export const LIBRARY_CATEGORIES = [
  { id: 'all', label: '全部' },
  { id: 'source', label: '论文文献' },
  { id: 'method', label: '方法' },
  { id: 'synthesis', label: '综述' },
  { id: 'concept', label: '概念' },
  { id: 'system-model', label: '系统模型' },
  { id: 'objective', label: '优化目标' },
  { id: 'dataset-or-sim', label: '数据与实验' },
  { id: 'problem', label: '研究问题' },
  { id: 'map', label: '知识地图' },
] as const

export type LibraryCategory = (typeof LIBRARY_CATEGORIES)[number]['id']

const TYPE_LABELS = new Map<string, string>(
  LIBRARY_CATEGORIES
    .filter((category) => category.id !== 'all')
    .map((category) => [category.id, category.label]),
)

export function libraryTypeLabel(pageType: string): string {
  return TYPE_LABELS.get(pageType) ?? '其他页面'
}

export function libraryCategoryCounts(items: ReadonlyArray<{ pageType: string }>): Record<LibraryCategory, number> {
  const counts = Object.fromEntries(LIBRARY_CATEGORIES.map((category) => [category.id, 0])) as Record<LibraryCategory, number>
  counts.all = items.length
  for (const item of items) {
    if (TYPE_LABELS.has(item.pageType)) counts[item.pageType as Exclude<LibraryCategory, 'all'>] += 1
  }
  return counts
}

export function filterLibraryItems<T extends { pageType: string }>(items: readonly T[], category: LibraryCategory): T[] {
  if (category === 'all') return [...items]
  return items.filter((item) => item.pageType === category)
}
