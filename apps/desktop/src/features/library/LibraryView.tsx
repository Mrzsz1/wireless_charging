import { BookOpen, ExternalLink, FileText, Gauge, Search, SlidersHorizontal } from 'lucide-react'
import type { PageFilters, PageSummary, SearchResult } from '../../types'

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

const typeLabels: Record<string, string> = { source: '文献', method: '方法', synthesis: '综述', concept: '概念', page: '页面' }

export function LibraryView({ query, results, catalog, pageType = 'source', filters, loading, onQueryChange, onFiltersChange, onOpenResult }: LibraryViewProps) {
  const typeName = pageType === 'method' ? '方法库' : '文献库'
  const catalogResults: SearchResult[] = catalog.map((page) => ({ id: page.id, pageType: page.pageType, title: page.title, year: page.year, summary: page.summary, sourcePath: page.sourcePath, snippet: page.summary, score: 0 }))
  const visibleResults = query.trim() ? results : catalogResults
  const years = [...new Set(catalog.map((page) => page.year).filter(Boolean))].sort((a, b) => Number(b) - Number(a))
  return <section className="library-view">
    <div className="library-heading"><div><div className="eyebrow">LOCAL KNOWLEDGE INDEX</div><h1>{typeName}</h1><p>{pageType === 'method' ? '浏览方法骨架、适用前提和来源文献。' : '浏览当前知识库中的论文、综述和结构化来源页面。'}</p></div><button className="refresh-button" onClick={() => onFiltersChange({ ...filters, sort: filters.sort === 'title' ? undefined : 'title' })}><SlidersHorizontal size={16} />{filters.sort === 'title' ? '按默认排序' : '按标题排序'}</button></div>
    <div className="library-search"><Search size={17} /><input autoFocus value={query} onChange={(event) => onQueryChange(event.target.value)} placeholder="搜索标题、模型、方法或关键词…" /><kbd>Ctrl K</kbd></div>
    <div className="library-filters"><select aria-label="年份筛选" value={filters.year ?? ''} onChange={(event) => onFiltersChange({ ...filters, year: event.target.value || undefined })}><option value="">全部年份</option>{years.map((year) => <option key={year} value={year}>{year}</option>)}</select><select aria-label="状态筛选" value={filters.status ?? ''} onChange={(event) => onFiltersChange({ ...filters, status: event.target.value || undefined })}><option value="">全部状态</option><option value="active">active</option><option value="draft">draft</option><option value="needs_review">needs_review</option></select><span className="library-filter-count">{loading ? '正在加载…' : `${visibleResults.length} 个页面`}</span></div>
    <div className="library-meta"><span>{loading ? '正在搜索…' : query ? `找到 ${results.length} 条结果` : `已加载 ${catalog.length} 个页面`}</span><span>仅当前知识库</span></div>
    <div className="library-results">{visibleResults.map((result) => <button className="library-result" key={result.id} onClick={() => onOpenResult(result)}><div className="result-type">{result.pageType === 'method' ? <Gauge size={15} /> : result.pageType === 'synthesis' ? <BookOpen size={15} /> : <FileText size={15} />}<span>{typeLabels[result.pageType] ?? result.pageType}</span></div><div className="result-main"><strong>{result.title}</strong><span>{result.year || '年份未记录'} · {result.sourcePath}</span><p>{result.snippet || result.summary}</p></div><ExternalLink size={15} className="result-open" /></button>)}{!loading && !visibleResults.length && <div className="library-empty"><FileText size={30} /><strong>{query ? '没有找到匹配页面' : '当前筛选没有页面'}</strong><span>{query ? '尝试更换关键词，或先在设置中选择知识库目录。' : '调整年份或状态筛选后重试。'}</span></div>}</div>
  </section>
}
