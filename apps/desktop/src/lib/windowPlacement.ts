export const WINDOW_STATE_VERSION = 3
export const WINDOW_STATE_KEY = 'desktop.window-state.v3'
export const LEGACY_WINDOW_STATE_KEY = 'desktop.window-state.v2'

const DEFAULT_LOGICAL_WIDTH = 1366
const DEFAULT_LOGICAL_HEIGHT = 768
const MIN_LOGICAL_WIDTH = 1180
const MIN_LOGICAL_HEIGHT = 720

export type PhysicalRect = {
  x: number
  y: number
  width: number
  height: number
}

export type MonitorWorkArea = PhysicalRect & {
  scaleFactor: number
  primary: boolean
}

export type PersistedWindowState = PhysicalRect & {
  version: typeof WINDOW_STATE_VERSION
  maximized: boolean
}

export type ResolvedWindowPlacement = {
  state: PersistedWindowState
  recovered: boolean
}

function finiteNumber(value: unknown): value is number {
  return typeof value === 'number' && Number.isFinite(value)
}

function positiveDimension(value: unknown): value is number {
  return finiteNumber(value) && value >= 320
}

function sanitizeMonitor(monitor: MonitorWorkArea): MonitorWorkArea | null {
  if (!finiteNumber(monitor.x) || !finiteNumber(monitor.y) || !finiteNumber(monitor.width) || !finiteNumber(monitor.height)) return null
  if (monitor.width <= 0 || monitor.height <= 0) return null
  return {
    x: Math.round(monitor.x),
    y: Math.round(monitor.y),
    width: Math.round(monitor.width),
    height: Math.round(monitor.height),
    scaleFactor: finiteNumber(monitor.scaleFactor) && monitor.scaleFactor > 0 ? monitor.scaleFactor : 1,
    primary: monitor.primary === true,
  }
}

export function parsePersistedWindowState(value: unknown): PersistedWindowState | null {
  if (!value || typeof value !== 'object') return null
  const candidate = value as Record<string, unknown>
  if (candidate.version !== 2 && candidate.version !== WINDOW_STATE_VERSION) return null
  if (!positiveDimension(candidate.width) || !positiveDimension(candidate.height)) return null
  if (!finiteNumber(candidate.x) || !finiteNumber(candidate.y) || typeof candidate.maximized !== 'boolean') return null
  return {
    version: WINDOW_STATE_VERSION,
    width: Math.round(candidate.width),
    height: Math.round(candidate.height),
    x: Math.round(candidate.x),
    y: Math.round(candidate.y),
    maximized: candidate.maximized,
  }
}

export function intersectionArea(rect: PhysicalRect, area: PhysicalRect): number {
  const width = Math.max(0, Math.min(rect.x + rect.width, area.x + area.width) - Math.max(rect.x, area.x))
  const height = Math.max(0, Math.min(rect.y + rect.height, area.y + area.height) - Math.max(rect.y, area.y))
  return width * height
}

function clamp(value: number, minimum: number, maximum: number): number {
  return Math.min(Math.max(value, minimum), maximum)
}

function dimensionsForArea(rect: Pick<PhysicalRect, 'width' | 'height'>, area: MonitorWorkArea) {
  const minimumWidth = Math.min(area.width, Math.round(MIN_LOGICAL_WIDTH * area.scaleFactor))
  const minimumHeight = Math.min(area.height, Math.round(MIN_LOGICAL_HEIGHT * area.scaleFactor))
  return {
    width: clamp(Math.round(rect.width), minimumWidth, area.width),
    height: clamp(Math.round(rect.height), minimumHeight, area.height),
  }
}

export function fitRectToWorkArea(rect: PhysicalRect, area: MonitorWorkArea): PhysicalRect {
  const { width, height } = dimensionsForArea(rect, area)
  return {
    x: clamp(Math.round(rect.x), area.x, area.x + area.width - width),
    y: clamp(Math.round(rect.y), area.y, area.y + area.height - height),
    width,
    height,
  }
}

function centeredRect(rect: Pick<PhysicalRect, 'width' | 'height'>, area: MonitorWorkArea): PhysicalRect {
  const { width, height } = dimensionsForArea(rect, area)
  return {
    x: area.x + Math.round((area.width - width) / 2),
    y: area.y + Math.round((area.height - height) / 2),
    width,
    height,
  }
}

function defaultRect(area: MonitorWorkArea): PhysicalRect {
  return centeredRect({
    width: Math.round(DEFAULT_LOGICAL_WIDTH * area.scaleFactor),
    height: Math.round(DEFAULT_LOGICAL_HEIGHT * area.scaleFactor),
  }, area)
}

export function resolveWindowPlacement(
  stored: PersistedWindowState | null,
  monitorCandidates: MonitorWorkArea[],
): ResolvedWindowPlacement | null {
  const monitors = monitorCandidates.map(sanitizeMonitor).filter((monitor): monitor is MonitorWorkArea => monitor !== null)
  if (!monitors.length) return null
  const primary = monitors.find((monitor) => monitor.primary) ?? monitors[0]

  if (!stored) {
    return {
      state: { version: WINDOW_STATE_VERSION, ...defaultRect(primary), maximized: false },
      recovered: true,
    }
  }

  const ranked = monitors
    .map((monitor) => ({ monitor, intersection: intersectionArea(stored, monitor) }))
    .sort((left, right) => right.intersection - left.intersection)
  const target = ranked[0].intersection > 0 ? ranked[0].monitor : primary
  const rect = ranked[0].intersection > 0 ? fitRectToWorkArea(stored, target) : centeredRect(stored, target)
  const recovered = rect.x !== stored.x || rect.y !== stored.y || rect.width !== stored.width || rect.height !== stored.height
  return {
    state: { version: WINDOW_STATE_VERSION, ...rect, maximized: stored.maximized },
    recovered,
  }
}

export function createPersistedWindowState(rect: PhysicalRect, maximized: boolean): PersistedWindowState {
  return {
    version: WINDOW_STATE_VERSION,
    x: Math.round(rect.x),
    y: Math.round(rect.y),
    width: Math.round(rect.width),
    height: Math.round(rect.height),
    maximized,
  }
}
