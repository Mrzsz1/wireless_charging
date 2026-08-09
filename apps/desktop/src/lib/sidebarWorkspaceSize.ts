export const SIDEBAR_WORKSPACE_SIZE_KEY = 'desktop.sidebar-workspace-height.v1'
export const SIDEBAR_WORKSPACE_SIZE_VERSION = 1
export const SIDEBAR_WORKSPACE_DEFAULT = 280
export const SIDEBAR_WORKSPACE_MIN = 132
export const SIDEBAR_WORKSPACE_STEP = 16

type PersistedSidebarWorkspaceSize = {
  version: number
  height: number
}

export function parseSidebarWorkspaceHeight(value: unknown): number | null {
  if (!value || typeof value !== 'object') return null
  const candidate = value as Partial<PersistedSidebarWorkspaceSize>
  if (candidate.version !== SIDEBAR_WORKSPACE_SIZE_VERSION) return null
  if (typeof candidate.height !== 'number' || !Number.isFinite(candidate.height) || candidate.height < 0) return null
  return Math.round(candidate.height)
}

export function clampSidebarWorkspaceHeight(value: number, min: number, max: number): number {
  const safeMin = Number.isFinite(min) ? Math.max(0, Math.round(min)) : 0
  const safeMax = Number.isFinite(max) ? Math.max(safeMin, Math.round(max)) : safeMin
  const safeValue = Number.isFinite(value) ? Math.round(value) : safeMin
  return Math.min(safeMax, Math.max(safeMin, safeValue))
}

export function resizeSidebarWorkspaceByKey(
  key: string,
  current: number,
  min: number,
  max: number,
): number | null {
  if (key === 'ArrowUp') return clampSidebarWorkspaceHeight(current - SIDEBAR_WORKSPACE_STEP, min, max)
  if (key === 'ArrowDown') return clampSidebarWorkspaceHeight(current + SIDEBAR_WORKSPACE_STEP, min, max)
  if (key === 'Home') return clampSidebarWorkspaceHeight(min, min, max)
  if (key === 'End') return clampSidebarWorkspaceHeight(max, min, max)
  return null
}

export function serializeSidebarWorkspaceHeight(height: number): string {
  return JSON.stringify({ version: SIDEBAR_WORKSPACE_SIZE_VERSION, height: Math.round(height) })
}
