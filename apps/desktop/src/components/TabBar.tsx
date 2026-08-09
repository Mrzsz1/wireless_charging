import { useRef, type KeyboardEvent, type MouseEvent } from 'react'
import { X } from 'lucide-react'

export type WorkTab = { id: string; label: string; kind: string; resourceId?: string; nav?: string; repositoryPath?: string }

type TabBarProps = {
  tabs: WorkTab[]
  activeId: string
  onSelect: (id: string) => void
  onClose: (id: string) => void
}

export function TabBar({ tabs, activeId, onSelect, onClose }: TabBarProps) {
  const triggerRefs = useRef(new Map<string, HTMLButtonElement>())

  const selectAndFocus = (index: number) => {
    const tab = tabs[index]
    if (!tab) return
    onSelect(tab.id)
    window.requestAnimationFrame(() => triggerRefs.current.get(tab.id)?.focus())
  }

  const handleKeyDown = (event: KeyboardEvent<HTMLButtonElement>, index: number) => {
    if (!tabs.length) return
    if (event.key === 'ArrowLeft') {
      event.preventDefault()
      selectAndFocus((index - 1 + tabs.length) % tabs.length)
    } else if (event.key === 'ArrowRight') {
      event.preventDefault()
      selectAndFocus((index + 1) % tabs.length)
    } else if (event.key === 'Home') {
      event.preventDefault()
      selectAndFocus(0)
    } else if (event.key === 'End') {
      event.preventDefault()
      selectAndFocus(tabs.length - 1)
    }
  }

  const closeTab = (event: MouseEvent<HTMLButtonElement>, id: string) => {
    event.stopPropagation()
    onClose(id)
  }

  return (
    <div className="work-tabs" role="tablist" aria-label="打开的工作区" data-testid="work-tabs">
      {tabs.map((tab, index) => {
        const active = tab.id === activeId
        return <div className={`work-tab-shell ${active ? 'active' : ''}`} key={tab.id} role="presentation">
          <button
            ref={(element) => {
              if (element) triggerRefs.current.set(tab.id, element)
              else triggerRefs.current.delete(tab.id)
            }}
            className="work-tab-trigger"
            data-testid={active ? 'active-work-tab' : undefined}
            role="tab"
            aria-selected={active}
            tabIndex={active ? 0 : -1}
            title={tab.label}
            onClick={() => onSelect(tab.id)}
            onKeyDown={(event) => handleKeyDown(event, index)}
          >
            <span>{tab.label}</span>
          </button>
          {tabs.length > 1 && <button className="work-tab-close" onClick={(event) => closeTab(event, tab.id)} aria-label={`关闭${tab.label}`} title={`关闭${tab.label}`}><X size={13} /></button>}
        </div>
      })}
    </div>
  )
}
