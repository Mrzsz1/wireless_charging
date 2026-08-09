export const PAGE_SIZE_OPTIONS = [10, 20, 50] as const

export type PageSlice<T> = {
  items: T[]
  page: number
  pageCount: number
  pageSize: number
  start: number
  end: number
  total: number
}

export function normalizePage(page: number, total: number, pageSize: number): number {
  const safeSize = Math.max(1, Math.floor(pageSize) || PAGE_SIZE_OPTIONS[0])
  const pageCount = Math.max(1, Math.ceil(Math.max(0, total) / safeSize))
  return Math.min(pageCount, Math.max(1, Math.floor(page) || 1))
}

export function paginate<T>(items: readonly T[], page: number, pageSize: number): PageSlice<T> {
  const safeSize = Math.max(1, Math.floor(pageSize) || PAGE_SIZE_OPTIONS[0])
  const current = normalizePage(page, items.length, safeSize)
  const offset = (current - 1) * safeSize
  const visible = items.slice(offset, offset + safeSize)
  return {
    items: visible,
    page: current,
    pageCount: Math.max(1, Math.ceil(items.length / safeSize)),
    pageSize: safeSize,
    start: items.length ? offset + 1 : 0,
    end: items.length ? offset + visible.length : 0,
    total: items.length,
  }
}

export function visiblePageNumbers(page: number, pageCount: number, maxButtons = 5): number[] {
  const total = Math.max(1, Math.floor(pageCount) || 1)
  const count = Math.min(total, Math.max(1, Math.floor(maxButtons) || 1))
  const current = normalizePage(page, total, 1)
  let start = Math.max(1, current - Math.floor(count / 2))
  start = Math.min(start, total - count + 1)
  return Array.from({ length: count }, (_, index) => start + index)
}
