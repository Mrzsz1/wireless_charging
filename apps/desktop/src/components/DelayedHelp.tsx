import { useEffect, useId, useRef, useState } from 'react'
import { CircleHelp } from 'lucide-react'
import './DelayedHelp.css'

type Props = {
  label: string
  testId?: string
  delayMs?: number
}

export function DelayedHelp({ label, testId, delayMs = 1000 }: Props) {
  const tooltipId = useId()
  const timerRef = useRef<number | null>(null)
  const [open, setOpen] = useState(false)

  const clearTimer = () => {
    if (timerRef.current === null) return
    window.clearTimeout(timerRef.current)
    timerRef.current = null
  }

  const scheduleOpen = () => {
    clearTimer()
    timerRef.current = window.setTimeout(() => {
      timerRef.current = null
      setOpen(true)
    }, delayMs)
  }

  const close = () => {
    clearTimer()
    setOpen(false)
  }

  useEffect(() => () => clearTimer(), [])

  return <span className="delayed-help">
    <button
      type="button"
      className="delayed-help-trigger"
      data-testid={testId}
      aria-label={`说明：${label}`}
      aria-describedby={open ? tooltipId : undefined}
      aria-expanded={open}
      onPointerEnter={scheduleOpen}
      onPointerLeave={close}
      onFocus={() => { clearTimer(); setOpen(true) }}
      onBlur={close}
      onClick={() => { clearTimer(); setOpen(true) }}
    >
      <CircleHelp size={14} strokeWidth={1.8} />
    </button>
    {open && <span id={tooltipId} className="delayed-help-tooltip" role="tooltip">{label}</span>}
  </span>
}
