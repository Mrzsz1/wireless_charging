import { X } from 'lucide-react'

export type WorkTab = { id: string; label: string; kind: string; resourceId?: string; nav?: string }

type TabBarProps = {
  tabs: WorkTab[]
  activeId: string
  onSelect: (id: string) => void
  onClose: (id: string) => void
}

export function TabBar({ tabs, activeId, onSelect, onClose }: TabBarProps) {
  return (
    <div className="work-tabs" role="tablist" aria-label="打开的工作区">
      {tabs.map((tab) => <div className={`work-tab ${tab.id === activeId ? 'active' : ''}`} key={tab.id} role="tab" aria-selected={tab.id === activeId}>
        <button onClick={() => onSelect(tab.id)}>{tab.label}</button>
        {tabs.length > 1 && <button className="work-tab-close" onClick={() => onClose(tab.id)} aria-label={`关闭${tab.label}`}><X size={13} /></button>}
      </div>)}
    </div>
  )
}
