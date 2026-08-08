import { useEffect, useMemo, useRef, useState } from 'react'
import { BookOpen, ChevronDown, ChevronRight, FileText, Search, ShieldCheck } from 'lucide-react'
import { convertFileSrc } from '@tauri-apps/api/core'
import { getBookChapter, isDesktopRuntime, listBookChapters, listCoreBooks, searchBookChapters } from '../../services/desktop'
import type { BookChapter, BookChapterDetail, BookSearchResult, BookSummary } from '../../types'
import { MarkdownReader } from '../pages/MarkdownReader'
import { chapterLookupId, matchesBookTarget, type BookTarget } from './bookTarget'

type CoreBooksViewProps = {
  onOpenLink: (target: string) => void
  target?: BookTarget | null
}

function pageLabel(value?: number | null) { return value ? `PDF p.${value}` : 'PDF 页码未记录' }

export function CoreBooksView({ onOpenLink, target }: CoreBooksViewProps) {
  const [books, setBooks] = useState<BookSummary[]>([])
  const [chapters, setChapters] = useState<BookChapter[]>([])
  const [selectedBook, setSelectedBook] = useState('')
  const [selectedChapter, setSelectedChapter] = useState<BookChapterDetail | null>(null)
  const [query, setQuery] = useState('')
  const [results, setResults] = useState<BookSearchResult[]>([])
  const [loading, setLoading] = useState(true)
  const [notice, setNotice] = useState('')
  const chapterRequest = useRef(0)
  const chapterListRequest = useRef(0)
  const pendingChapterId = useRef('')
  const targetEnabled = useRef(Boolean(target?.bookId && target?.chapterId))

  const targetKey = target ? `${target.bookId}:${target.chapterId}` : ''
  useEffect(() => {
    targetEnabled.current = Boolean(targetKey)
  }, [targetKey])

  useEffect(() => {
    if (!isDesktopRuntime()) { setLoading(false); setNotice('核心专著目录需要在 Windows 桌面客户端中加载'); return }
    let active = true
    void listCoreBooks().then((items) => {
      if (!active) return
      setBooks(items)
      const targetBook = target?.bookId && items.some((item) => item.id === target.bookId) ? target.bookId : ''
      if (targetBook || items[0]) setSelectedBook(targetBook || items[0].id)
    }).catch((error) => { if (active) setNotice(`核心专著加载失败：${String(error)}`) }).finally(() => { if (active) setLoading(false) })
    return () => { active = false }
  }, [])

  useEffect(() => {
    if (!selectedBook || !isDesktopRuntime()) return
    const request = ++chapterListRequest.current
    chapterRequest.current += 1
    pendingChapterId.current = ''
    setSelectedChapter(null)
    setChapters([])
    void listBookChapters(selectedBook).then((items) => {
      if (request !== chapterListRequest.current) return
      setChapters(items)
      const targetChapter = targetEnabled.current && target?.bookId === selectedBook
        ? items.find((item) => matchesBookTarget(item, target))
        : undefined
      if (targetChapter) void openChapter(targetChapter, true)
      else if (items[0]) void openChapter(items[0])
    }).catch((error) => { if (request === chapterListRequest.current) setNotice(`章节目录加载失败：${String(error)}`) })
  }, [selectedBook])

  useEffect(() => {
    if (targetEnabled.current && target?.bookId && target.bookId !== selectedBook) setSelectedBook(target.bookId)
  }, [target?.bookId, selectedBook])

  useEffect(() => {
    if (!targetEnabled.current || !target?.chapterId || target.bookId !== selectedBook || !chapters.length) return
    const chapter = chapters.find((item) => matchesBookTarget(item, target))
    if (chapter && selectedChapter?.chapter.id !== chapter.id && pendingChapterId.current !== chapter.id) void openChapter(chapter, true)
  }, [chapters, selectedBook, selectedChapter?.chapter.id, target?.bookId, target?.chapterId])

  async function openChapter(chapter: BookChapter, preserveTarget = false) {
    if (!preserveTarget) targetEnabled.current = false
    const request = ++chapterRequest.current
    pendingChapterId.current = chapter.id
    try {
      const detail = await getBookChapter(chapter.bookId, chapterLookupId(chapter))
      if (request === chapterRequest.current) setSelectedChapter(detail)
    } catch (error) {
      if (request === chapterRequest.current) setNotice(`章节读取失败：${String(error)}`)
    } finally {
      if (request === chapterRequest.current && pendingChapterId.current === chapter.id) pendingChapterId.current = ''
    }
  }

  const runSearch = async (value: string) => {
    setQuery(value)
    if (!value.trim() || !isDesktopRuntime()) { setResults([]); return }
    try { setResults(await searchBookChapters(value, selectedBook || undefined, 30)) } catch (error) { setNotice(`专著搜索失败：${String(error)}`) }
  }

  const selectedBookMeta = useMemo(() => books.find((book) => book.id === selectedBook), [books, selectedBook])
  const pdfUrl = selectedChapter && isDesktopRuntime() ? `${convertFileSrc(selectedChapter.chapter.pdfPath)}#page=${selectedChapter.chapter.physicalPageStart ?? 1}` : ''

  if (loading) return <section className="page-loading" data-testid="books-view"><BookOpen size={22} className="spin" /><span>正在加载核心专著目录…</span></section>
  return <section className="books-view" data-testid="books-view">
    <div className="library-heading"><div><div className="eyebrow">CORE REFERENCE BOOKS</div><h1>核心书籍</h1><p>两本算法专著、61 个章节、1171 个 PDF 物理页。</p></div><div className="book-quality-badge"><ShieldCheck size={15} />检索质量 ≥95%</div></div>
    {notice && <div className="notice"><FileText size={15} /><span>{notice}</span></div>}
    <div className="books-toolbar"><label className="library-search"><Search size={16} /><input value={query} onChange={(event) => void runSearch(event.target.value)} placeholder="搜索章节、算法、模型或术语…" /></label><select data-testid="book-selector" value={selectedBook} onChange={(event) => { targetEnabled.current = false; setSelectedBook(event.target.value) }} aria-label="选择专著">{books.map((book) => <option key={book.id} value={book.id}>{book.title} · {book.chapterCount}章</option>)}</select></div>
    {query && <div className="book-search-results">{results.map((result) => <button className="book-search-result" key={result.chapter.id} onClick={() => void openChapter(result.chapter)}><span>{result.chapter.title}</span><small>{pageLabel(result.chapter.physicalPageStart)}–{result.chapter.physicalPageEnd ?? '?'} · {result.snippet}</small></button>)}{!results.length && <div className="library-empty">没有找到章节匹配。</div>}</div>}
    <div className="book-reader-layout">
      <aside className="chapter-sidebar"><div className="book-card"><BookOpen size={18} /><div><strong>{selectedBookMeta?.title ?? '核心专著'}</strong><small>{selectedBookMeta?.year} · {selectedBookMeta?.pageCount} 页</small></div></div><div className="chapter-count">章节目录 · {chapters.length}</div><div className="chapter-tree">{chapters.map((chapter) => <button data-testid={`book-chapter-${chapter.id}`} key={chapter.id} className={`chapter-item ${selectedChapter?.chapter.id === chapter.id ? 'active' : ''}`} onClick={() => void openChapter(chapter)}><span className="chapter-chevron">{chapter.chapterNumber === 0 ? <BookOpen size={13} /> : selectedChapter?.chapter.id === chapter.id ? <ChevronDown size={13} /> : <ChevronRight size={13} />}</span><span><strong>{chapter.chapterNumber === 0 ? '前置内容' : `Chapter ${chapter.chapterNumber}`}</strong><small>{chapter.title}</small></span></button>)}</div></aside>
      <main className="book-markdown-column" data-testid="book-chapter-content">{selectedChapter ? <><div className="book-chapter-heading"><div><div className="eyebrow">{selectedChapter.chapter.bookId} · Chapter {selectedChapter.chapter.chapterNumber}</div><h2>{selectedChapter.chapter.title}</h2><p>{pageLabel(selectedChapter.chapter.physicalPageStart)} – {selectedChapter.chapter.physicalPageEnd ?? '未记录'} · {selectedChapter.chapter.charCount.toLocaleString()} 字符</p></div></div><MarkdownReader body={selectedChapter.body} sourcePath={selectedChapter.chapter.markdownPath} onOpenLink={onOpenLink} /></> : <div className="book-empty"><BookOpen size={28} /><strong>选择一个章节开始阅读</strong></div>}</main>
      <aside className="book-pdf-column">{selectedChapter ? <><div className="pdf-heading"><span>PDF 定位</span><small>{pageLabel(selectedChapter.chapter.physicalPageStart)}</small></div>{pdfUrl ? <iframe title={`${selectedChapter.chapter.title} PDF`} src={pdfUrl} className="pdf-frame" /> : <div className="pdf-empty"><FileText size={26} /><span>PDF 预览仅在桌面客户端中可用</span><small>{selectedChapter.chapter.pdfPath}</small></div>}</> : <div className="pdf-empty"><FileText size={26} /><span>章节打开后显示 PDF</span></div>}</aside>
    </div>
  </section>
}
