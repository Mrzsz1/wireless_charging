import type { GraphNode, GraphOverview } from '../../types'

export function nextGraphRefreshVersion(version: number, graphRefresh: boolean): number {
  return graphRefresh ? version + 1 : version
}

export function reconcileGraphSelection(graph: GraphOverview, selected: GraphNode | null): GraphNode | null {
  if (!selected) return null
  return graph.nodes.find((node) => node.id === selected.id) ?? null
}

export function reconcileGraphPath(graph: GraphOverview, path: string[]): string[] {
  const nodeIds = new Set(graph.nodes.map((node) => node.id))
  return path.length > 0 && path.every((id) => nodeIds.has(id)) ? path : []
}
