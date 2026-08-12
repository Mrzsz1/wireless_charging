import { useEffect, useMemo, useState } from 'react'
import { BookOpen, Boxes, ChevronLeft, ChevronRight, CircleHelp, Database, ExternalLink, FileText, Gauge, Lightbulb, Map, Search, SlidersHorizontal, Target } from 'lucide-react'
import type { PageFilters, PageSummary, SearchResult } from '../../types'
import { filterLibraryItems, LIBRARY_CATEGORIES, libraryCategoryCounts, libraryTypeLabel, type LibraryCategory } from './categories'
import { PAGE_SIZE_OPTIONS, paginate, visiblePageNumbers } from './pagination'

type LibraryViewProps = {
  query: string
  results: SearchResult[]
  catalog: PageSummary[]
  pageType?: 'source' | 'method'
  filters: PageFilters
  loading: boolean
  onQueryChange: (query: string) => void
  onFiltersChange: (filters: PageFilters) => void
  onOpenResult: (result: SearchResult) => void
}

function resultTypeIcon(pageType: string) {
  if (pageType === 'method') return <Gauge size={15} />
  if (pageType === 'synthesis') return <BookOpen size={15} />
  if (pageType === 'map') return <Map size={15} />
  if (pageType === 'system-model') return <Boxes size={15} />
  if (pageType === 'objective') return <Target size={15} />
  if (pageType === 'dataset-or-sim') return <Database size={15} />
  if (pageType === 'concept') return <Lightbulb size={15} />
  if (pageType === 'problem') return <CircleHelp size={15} />
  return <FileText size={15} />
}

export function LibraryView({ query, results, catalog, pageType = 'source', filters, loading, onQueryChange, onFiltersChange, onOpenResult }: LibraryViewProps) {
  const [page, setPage] = useState(1)
  const [pageSize, setPageSize] = useState<number>(PAGE_SIZE_OPTIONS[0])
  const [category, setCategory] = useState<LibraryCategory>('all')
  const typeName = pageType === 'method' ? '方法库' : '文献库'
  const catalogResults: SearchResult[] = useMemo(() => catalog.map((item) => ({ id: item.id, pageType: item.pageType, title: item.title, year: item.year, summary: item.summary, sourcePath: item.sourcePath, snippet: item.summary, score: 0 })), [catalog])
  const baseResults = query.trim() ? results : catalogResults
  const activeCategory: LibraryCategory = pageType === 'method' ? 'method' : category
  const categoryCounts = useMemo(() => libraryCategoryCounts(baseResults), [baseResults])
  const visibleResults = useMemo(() => filterLibraryItems(baseResults, activeCategory), [activeCategory, baseResults])
  const years = [...new Set(catalog.map((page) => page.year).filter(Boolean))].sort((a, b) => Number(b) - Number(a))
  const pagination = useMemo(() => paginate(visibleResults, page, pageSize), [page, pageSize, visibleResults])
  useEffect(() => { setPage(1) }, [query, pageType, category, filters.year, filters.status, filters.sort])
  useEffect(() => { if (page !== pagination.page) setPage(pagination.page) }, [page, pagination.page])
  return <section className="library-view">
    <div className="library-heading"><div><h1>{typeName}</h1><p>{pageType === 'method' ? '浏览方法骨架、适用前提和来源文献。' : '浏览当前知识库中的论文、综述和结构化来源页面。'}</p></div><button className="refresh-button" onClick={() => onFiltersChange({ ...filters, sort: filters.sort === 'title' ? undefined : 'title' })}><SlidersHorizontal size={16} />{filters.sort === 'title' ? '按默认排序' : '按标题排序'}</button></div>
    <div className="library-search"><Search size={17} /><input data-testid="library-search" autoFocus value={query} onChange={(event) => onQueryChange(event.target.value)} placeholder="搜索标题、模型、方法或关键词…" /><kbd>Ctrl K</kbd></div>
    {pageType !== 'method' && <div className="library-categories" role="group" aria-label="内容分类" data-testid="library-categories">
      {LIBRARY_CATEGORIES.map((item) => <button type="button" key={item.id} data-testid={`library-category-${item.id}`} aria-pressed={category === item.id} className={category === item.id ? 'active' : ''} onClick={() => setCategory(item.id)}><span>{item.label}</span><strong>{categoryCounts[item.id]}</strong></button>)}
    </div>}
    <div className="library-filters"><select aria-label="年份筛选" value={filters.year ?? ''} onChange={(event) => onFiltersChange({ ...filters, year: event.target.value || undefined })}><option value="">全部年份</option>{years.map((year) => <option key={year} value={year}>{year}</option>)}</select><select aria-label="状态筛选" value={filters.status ?? ''} onChange={(event) => onFiltersChange({ ...filters, status: event.target.value || undefined })}><option value="">全部状态</option><option value="active">active</option><option value="draft">draft</option><option value="needs_review">needs_review</option></select><span className="library-filter-count">{loading ? '正在加载…' : `${visibleResults.length} 个页面`}</span></div>
    <div className="library-meta" data-testid="library-search-status"><span>{loading ? '正在搜索…' : query ? `找到 ${visibleResults.length} 条结果` : `已加载 ${visibleResults.length} 个页面`}</span><span>{pagination.total ? `显示 ${pagination.start}–${pagination.end}` : '暂无结果'} · {activeCategory === 'all' ? '全部分类' : libraryTypeLabel(activeCategory)} · 仅当前知识库</span></div>
    <div className="library-results" data-testid="library-search-results">{pagination.items.map((result) => <button className="library-result" data-page-type={result.pageType} key={result.id} onClick={() => onOpenResult(result)}><div className="result-type">{resultTypeIcon(result.pageType)}<span>{libraryTypeLabel(result.pageType)}</span></div><div className="result-main"><strong>{result.title}</strong><span>{result.year || '年份未记录'} · {result.sourcePath}</span><p>{result.snippet || result.summary}</p></div><ExternalLink size={15} className="result-open" /></button>)}{!loading && !visibleResults.length && <div className="library-empty"><FileText size={30} /><strong>{query ? '当前分类没有匹配页面' : '当前筛选没有页面'}</strong><span>{query ? '尝试切换分类或更换关键词。' : '调整内容分类、年份或状态后重试。'}</span></div>}</div>
    {!!pagination.total && <nav className="library-pagination" data-testid="library-pagination" aria-label={`${typeName}分页`}><div className="pagination-size"><span>每页</span><select aria-label="每页显示数量" value={pageSize} onChange={(event) => { setPageSize(Number(event.target.value)); setPage(1) }}>{PAGE_SIZE_OPTIONS.map((size) => <option value={size} key={size}>{size} 条</option>)}</select></div><div className="pagination-controls"><button aria-label="上一页" disabled={pagination.page === 1} onClick={() => setPage((current) => current - 1)}><ChevronLeft size={14} />上一页</button>{visiblePageNumbers(pagination.page, pagination.pageCount).map((number) => <button aria-label={`第 ${number} 页`} aria-current={number === pagination.page ? 'page' : undefined} className={number === pagination.page ? 'active' : ''} onClick={() => setPage(number)} key={number}>{number}</button>)}<button aria-label="下一页" disabled={pagination.page === pagination.pageCount} onClick={() => setPage((current) => current + 1)}>下一页<ChevronRight size={14} /></button></div><span data-testid="library-page-status">第 {pagination.page} / {pagination.pageCount} 页 · 共 {pagination.total} 条</span></nav>}
  </section>
}
