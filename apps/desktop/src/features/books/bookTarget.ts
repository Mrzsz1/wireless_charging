import type { BookChapter } from '../../types'

export type BookTarget = {
  bookId: string
  chapterId: string
}

export function shortChapterId(chapterId: string): string {
  const separator = chapterId.indexOf(':')
  return separator >= 0 ? chapterId.slice(separator + 1) : chapterId
}

export function chapterLookupId(chapter: BookChapter): string {
  return shortChapterId(chapter.id)
}

export function matchesBookTarget(chapter: BookChapter, target?: BookTarget | null): boolean {
  if (!target || chapter.bookId !== target.bookId) return false
  return chapter.id === target.chapterId || shortChapterId(chapter.id) === shortChapterId(target.chapterId)
}
