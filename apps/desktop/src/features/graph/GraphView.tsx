import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { CircleDot, Filter, GitBranch, Network, Search } from 'lucide-react'
import { graphNeighbors, graphOverview, graphPath, isDesktopRuntime } from '../../services/desktop'
import type { GraphFilters, GraphNode, GraphOverview } from '../../types'
import { reconcileGraphPath, reconcileGraphSelection } from './refreshState'

type GraphViewProps = { onOpenPage: (sourceFile: string) => void; refreshVersion?: number }

function layoutNodes(nodes: GraphNode[]) {
  return nodes.map((node, index) => {
    const angle = (index / Math.max(nodes.length, 1)) * Math.PI * 2
    const radius = 30 + (index % 5) * 7
    return { node, x: 50 + Math.cos(angle) * radius, y: 50 + Math.sin(angle) * radius * .72 }
  })
}

export function GraphView({ onOpenPage, refreshVersion = 0 }: GraphViewProps) {
  const [graph, setGraph] = useState<GraphOverview | null>(null)
  const [query, setQuery] = useState('')
  const [selected, setSelected] = useState<GraphNode | null>(null)
  const [notice, setNotice] = useState('')
  const [pathTarget, setPathTarget] = useState('')
  const [path, setPath] = useState<string[]>([])
  const [loading, setLoading] = useState(true)
  const loadRequest = useRef(0)

  const load = useCallback(async (filters: GraphFilters = {}) => {
    if (!isDesktopRuntime()) { setNotice('知识图谱需要在 Windows 桌面客户端中读取 graphify-out/graph.json'); setLoading(false); return }
    const request = ++loadRequest.current
    setLoading(true)
    try {
      const next = await graphOverview({ ...filters, limit: 120 })
      if (request !== loadRequest.current) return
      setGraph(next)
      setSelected((current) => reconcileGraphSelection(next, current))
      setPath((current) => reconcileGraphPath(next, current))
    } catch (error) {
      if (request === loadRequest.current) setNotice(`图谱加载失败：${String(error)}`)
    } finally {
      if (request === loadRequest.current) setLoading(false)
    }
  }, [])
  useEffect(() => { void load(query.trim() ? { query } : {}) }, [load, query, refreshVersion])
  const positioned = useMemo(() => layoutNodes(graph?.nodes ?? []), [graph])
  const nodeMap = useMemo(() => new Map(positioned.map((item) => [item.node.id, item])), [positioned])

  const search = (value: string) => { setQuery(value) }
  const expand = async (node: GraphNode) => { setSelected(node); try { setGraph(await graphNeighbors(node.id, 1, 120)) } catch (error) { setNotice(`邻居加载失败：${String(error)}`) } }
  const findPath = async () => { if (!selected || !pathTarget) return; try { setPath(await graphPath(selected.id, pathTarget, 8)) } catch (error) { setNotice(`路径查询失败：${String(error)}`) } }
  const openNode = (node: GraphNode) => { if (node.sourceFile.includes('wiki/')) onOpenPage(node.sourceFile.replace(/^.*wiki[\\/]?/, '').replace(/\\/g, '/').replace(/\.md$/, '')) }

  return <section className="graph-view" data-testid="graph-view" data-refresh-version={refreshVersion}>
    <div className="library-heading"><div><div className="eyebrow">GRAPHIFY KNOWLEDGE MAP</div><h1>知识图谱</h1><p>从社区概览逐步展开局部关系，所有连线均标注为派生关系。</p></div><div className="graph-counts"><span>{graph?.nodeCount ?? 0} 节点</span><span>{graph?.edgeCount ?? 0} 边</span><span>{graph?.communityCount ?? 0} 社区</span></div></div>
    <div className="graph-toolbar"><label className="library-search"><Search size={16} /><input value={query} onChange={(event) => void search(event.target.value)} placeholder="搜索节点或来源文件…" /></label><button className="refresh-button" onClick={() => void load()}><Filter size={15} />重置视图</button></div>
    {notice && <div className="notice"><Network size={15} /><span>{notice}</span></div>}
    <div className="graph-layout"><div className="graph-canvas-wrap">{loading ? <div className="page-loading"><CircleDot size={22} className="spin" /><span>正在读取图谱…</span></div> : graph && graph.nodes.length ? <svg className="graph-canvas" viewBox="0 0 100 100" role="img" aria-label="Graphify 局部图谱">{graph.edges.map((edge) => { const source = nodeMap.get(edge.source); const target = nodeMap.get(edge.target); return source && target ? <line key={`${edge.source}-${edge.target}-${edge.relation}`} x1={source.x} y1={source.y} x2={target.x} y2={target.y} className="graph-line" /> : null })}{positioned.map(({ node, x, y }) => <g key={node.id} className={`graph-svg-node ${selected?.id === node.id ? 'selected' : ''}`} transform={`translate(${x} ${y})`} onClick={() => void expand(node)}><circle r={node.id === selected?.id ? 3.5 : 2.5} /><text y="6" textAnchor="middle">{node.label.slice(0, 22)}</text></g>)}</svg> : <div className="graph-empty"><Network size={28} /><strong>没有可展示的图谱节点</strong><span>请先运行 graphify update .</span></div>}</div>
      <aside className="graph-side-panel">{selected ? <><div className="graph-node-title"><span className="eyebrow">NODE</span><h2>{selected.label}</h2><small>{selected.nodeType} · community {selected.community ?? '—'}</small></div><dl className="page-info-list"><dt>节点 ID</dt><dd>{selected.id}</dd><dt>来源</dt><dd>{selected.sourceFile || '未记录'}</dd><dt>位置</dt><dd>{selected.sourceLocation || '未记录'}</dd><dt>关系</dt><dd>Graphify 派生</dd></dl><button className="refresh-button graph-open-button" onClick={() => openNode(selected)}><GitBranch size={14} />打开 Wiki 来源</button><div className="graph-path-form"><h3>路径查询</h3><input value={pathTarget} onChange={(event) => setPathTarget(event.target.value)} placeholder="输入目标节点 ID" /><button className="refresh-button" onClick={() => void findPath()}>查询路径</button>{path.length > 0 && <div className="path-result">{path.map((id, index) => <span key={id}>{index > 0 && ' → '}{id}</span>)}</div>}{pathTarget && !path.length && <small>尚未找到路径</small>}</div></> : <div className="graph-empty"><Network size={24} /><span>点击节点查看详情和邻居。</span></div>}</aside></div>
  </section>
}
