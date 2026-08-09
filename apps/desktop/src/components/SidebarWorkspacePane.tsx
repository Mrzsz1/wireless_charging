import {
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
  type KeyboardEvent,
  type PointerEvent,
  type ReactNode,
} from 'react'
import {
  SIDEBAR_WORKSPACE_DEFAULT,
  SIDEBAR_WORKSPACE_MIN,
  SIDEBAR_WORKSPACE_SIZE_KEY,
  clampSidebarWorkspaceHeight,
  parseSidebarWorkspaceHeight,
  resizeSidebarWorkspaceByKey,
  serializeSidebarWorkspaceHeight,
} from '../lib/sidebarWorkspaceSize'

type SidebarWorkspacePaneProps = {
  children: ReactNode
}

type SizeLimits = {
  min: number
  max: number
}

type DragState = SizeLimits & {
  pointerId: number
  startHeight: number
  startY: number
}

const INITIAL_LIMITS: SizeLimits = { min: SIDEBAR_WORKSPACE_MIN, max: 1200 }

function readPersistedHeight(): number {
  try {
    const value = localStorage.getItem(SIDEBAR_WORKSPACE_SIZE_KEY)
    if (!value) return SIDEBAR_WORKSPACE_DEFAULT
    return parseSidebarWorkspaceHeight(JSON.parse(value)) ?? SIDEBAR_WORKSPACE_DEFAULT
  } catch {
    return SIDEBAR_WORKSPACE_DEFAULT
  }
}

export function SidebarWorkspacePane({ children }: SidebarWorkspacePaneProps) {
  const panelRef = useRef<HTMLDivElement>(null)
  const resizerRef = useRef<HTMLDivElement>(null)
  const [desiredHeight, setDesiredHeight] = useState(readPersistedHeight)
  const desiredHeightRef = useRef(desiredHeight)
  const dragRef = useRef<DragState | null>(null)
  const [limits, setLimits] = useState<SizeLimits>(INITIAL_LIMITS)
  const [dragging, setDragging] = useState(false)

  const measureLimits = (): SizeLimits => {
    const panel = panelRef.current
    const resizer = resizerRef.current
    const sidebar = panel?.closest<HTMLElement>('.app-sidebar')
    const footer = sidebar?.querySelector<HTMLElement>('.sidebar-footer')
    const spacer = sidebar?.querySelector<HTMLElement>('.sidebar-spacer')
    if (!panel || !resizer || !sidebar || !footer) return limits

    const sidebarRect = sidebar.getBoundingClientRect()
    const panelRect = panel.getBoundingClientRect()
    const footerRect = footer.getBoundingClientRect()
    const resizerRect = resizer.getBoundingClientRect()
    const spacerMinHeight = spacer ? Number.parseFloat(getComputedStyle(spacer).minHeight) || 0 : 0
    const available = Math.max(
      0,
      sidebarRect.bottom - footerRect.height - panelRect.top - resizerRect.height - spacerMinHeight,
    )
    const max = Math.floor(available)
    return { min: Math.min(SIDEBAR_WORKSPACE_MIN, max), max }
  }

  const updateLimits = () => {
    const next = measureLimits()
    setLimits((current) => current.min === next.min && current.max === next.max ? current : next)
    return next
  }

  const setHeight = (height: number, nextLimits: SizeLimits, persist = false) => {
    const next = clampSidebarWorkspaceHeight(height, nextLimits.min, nextLimits.max)
    desiredHeightRef.current = next
    setDesiredHeight(next)
    if (persist) {
      try {
        localStorage.setItem(SIDEBAR_WORKSPACE_SIZE_KEY, serializeSidebarWorkspaceHeight(next))
      } catch {
        // Local storage is optional; resizing must remain usable when it is unavailable.
      }
    }
  }

  useLayoutEffect(() => {
    const panel = panelRef.current
    const sidebar = panel?.closest<HTMLElement>('.app-sidebar')
    if (!sidebar) return
    updateLimits()
    const observer = new ResizeObserver(() => updateLimits())
    observer.observe(sidebar)
    return () => observer.disconnect()
  }, [])

  useEffect(() => () => {
    document.body.classList.remove('sidebar-workspace-resizing')
  }, [])

  const finishDrag = (event: PointerEvent<HTMLDivElement>) => {
    const drag = dragRef.current
    if (!drag || drag.pointerId !== event.pointerId) return
    dragRef.current = null
    if (event.currentTarget.hasPointerCapture(event.pointerId)) event.currentTarget.releasePointerCapture(event.pointerId)
    setDragging(false)
    document.body.classList.remove('sidebar-workspace-resizing')
    setHeight(desiredHeightRef.current, { min: drag.min, max: drag.max }, true)
  }

  const handlePointerDown = (event: PointerEvent<HTMLDivElement>) => {
    if (event.button !== 0) return
    const panel = panelRef.current
    if (!panel) return
    event.preventDefault()
    const nextLimits = updateLimits()
    dragRef.current = {
      pointerId: event.pointerId,
      startHeight: panel.getBoundingClientRect().height,
      startY: event.clientY,
      ...nextLimits,
    }
    try {
      event.currentTarget.setPointerCapture(event.pointerId)
    } catch {
      // Synthetic pointer events used by GUI verification do not own an active pointer.
    }
    setDragging(true)
    document.body.classList.add('sidebar-workspace-resizing')
  }

  const handlePointerMove = (event: PointerEvent<HTMLDivElement>) => {
    const drag = dragRef.current
    if (!drag || drag.pointerId !== event.pointerId) return
    setHeight(drag.startHeight + event.clientY - drag.startY, { min: drag.min, max: drag.max })
  }

  const handleKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    const nextLimits = updateLimits()
    const current = panelRef.current?.getBoundingClientRect().height ?? desiredHeightRef.current
    const next = resizeSidebarWorkspaceByKey(event.key, current, nextLimits.min, nextLimits.max)
    if (next === null) return
    event.preventDefault()
    setHeight(next, nextLimits, true)
  }

  const handleDoubleClick = () => {
    const nextLimits = updateLimits()
    setHeight(SIDEBAR_WORKSPACE_DEFAULT, nextLimits, true)
  }

  const effectiveHeight = clampSidebarWorkspaceHeight(desiredHeight, limits.min, limits.max)

  return (
    <>
      <div
        ref={panelRef}
        className="sidebar-expanded-content"
        data-testid="sidebar-workspace-pane"
        style={{ height: effectiveHeight }}
      >
        {children}
      </div>
      <div
        ref={resizerRef}
        className={`sidebar-workspace-resizer ${dragging ? 'dragging' : ''}`}
        data-testid="sidebar-workspace-resizer"
        role="separator"
        tabIndex={0}
        aria-label="调整我的空间高度"
        aria-orientation="horizontal"
        aria-valuemin={limits.min}
        aria-valuemax={limits.max}
        aria-valuenow={effectiveHeight}
        title="上下拖动调整高度，双击恢复默认"
        onPointerDown={handlePointerDown}
        onPointerMove={handlePointerMove}
        onPointerUp={finishDrag}
        onPointerCancel={finishDrag}
        onLostPointerCapture={finishDrag}
        onKeyDown={handleKeyDown}
        onDoubleClick={handleDoubleClick}
      />
    </>
  )
}
