import { useCallback, useEffect, useRef, useState } from 'react'
import { Sparkles, X } from 'lucide-react'

const TOAST_HOLD_MS = 3600
const TOAST_EXIT_MS = 450

type AppToastProps = {
  message: string
  contextOpen: boolean
  onDismiss: () => void
}

export function AppToast({ message, contextOpen, onDismiss }: AppToastProps) {
  const [exiting, setExiting] = useState(false)
  const holdTimer = useRef<number | null>(null)
  const exitTimer = useRef<number | null>(null)
  const dismissRef = useRef(onDismiss)

  useEffect(() => { dismissRef.current = onDismiss }, [onDismiss])

  const clearHoldTimer = useCallback(() => {
    if (holdTimer.current === null) return
    window.clearTimeout(holdTimer.current)
    holdTimer.current = null
  }, [])

  const beginExit = useCallback(() => {
    clearHoldTimer()
    setExiting(true)
    if (exitTimer.current !== null) window.clearTimeout(exitTimer.current)
    exitTimer.current = window.setTimeout(() => dismissRef.current(), TOAST_EXIT_MS)
  }, [clearHoldTimer])

  const scheduleExit = useCallback(() => {
    clearHoldTimer()
    holdTimer.current = window.setTimeout(beginExit, TOAST_HOLD_MS)
  }, [beginExit, clearHoldTimer])

  useEffect(() => {
    scheduleExit()
    return () => {
      clearHoldTimer()
      if (exitTimer.current !== null) window.clearTimeout(exitTimer.current)
    }
  }, [clearHoldTimer, scheduleExit])

  return <div
    className={`app-toast ${contextOpen ? 'context-open' : ''} ${exiting ? 'exiting' : ''}`}
    data-testid="app-notice"
    role="status"
    aria-live="polite"
    aria-atomic="true"
    onMouseEnter={clearHoldTimer}
    onMouseLeave={() => { if (!exiting) scheduleExit() }}
  >
    <span className="app-toast-icon"><Sparkles size={15} /></span>
    <span className="app-toast-message">{message}</span>
    <button className="app-toast-close" aria-label="关闭通知" title="关闭通知" onClick={beginExit}><X size={14} /></button>
  </div>
}
