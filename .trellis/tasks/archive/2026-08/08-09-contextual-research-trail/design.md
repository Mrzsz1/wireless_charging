# 上下文相关研究脉络技术设计

## 1. 设计结论

新增一个只读、确定性的 `research_trail` 检索层，以当前页面或当前问题为锚点，合并 Wiki 链接关系、SQLite FTS5、核心书籍章节与 Graphify 邻接关系。前端把现有右侧 JSX 抽成独立组件，通过一个上下文状态机请求、缓存和呈现结果。

本阶段不引入 embedding 模型。原因是现有仓库已经具备可审计的 FTS5、页面链接、书籍索引和 Graphify；先建立“每个结果都能解释”的混合排序，比增加不可复核的远程语义依赖更符合论文严谨性边界。

## 2. 模块边界

```text
App / active tab / AskView
        │ ResearchContextAnchor
        ▼
useResearchTrail（请求序列、缓存、降级、固定项）
        │ invoke prepare_research_trail
        ▼
Rust research_trail.rs
  ├─ page anchor loader
  ├─ wikilink + backlink candidates
  ├─ Wiki FTS5 candidates
  ├─ method-only candidates
  ├─ qa.rs shared query terms / book candidates
  └─ Graphify node mapping + one-hop candidates
        │
        ▼
ResearchTrailPanel / EvidencePicker / GraphView focus
```

### 2.1 新文件

- `apps/desktop/src-tauri/src/research_trail.rs`：候选生成、归一化、去重、排序、响应组装。
- `apps/desktop/src/features/research-trail/ResearchTrailPanel.tsx`：面板 UI。
- `apps/desktop/src/features/research-trail/ResearchTrailPanel.css`：面板样式。
- `apps/desktop/src/features/research-trail/researchTrailState.ts`：上下文键、请求发布守卫、固定项解析/合并。
- `apps/desktop/tests/research-trail-state.test.ts`：纯状态回归。

如实现中发现 `qa.rs` 的候选函数适合共享，应将通用词项扩展与 FTS 查询提取为 `retrieval.rs`；禁止复制两套中英扩展词表。

## 3. 数据契约

### 3.1 前端锚点

```ts
type ResearchContextAnchor =
  | { kind: 'page'; key: string; pageId: string; title: string }
  | { kind: 'question'; key: string; text: string; title: string }
  | { kind: 'search'; key: string; text: string; title: string }
  | { kind: 'idle'; key: 'idle'; title: string }
```

`key` 必须稳定：

- 页面：`page:<pageId>`；
- 问题：`question:<sha256(normalized question)>`；
- 搜索：`search:<sha256(normalized query)>`；
- 空态：`idle`。

### 3.2 Tauri 请求

```ts
type ResearchTrailRequest = {
  kind: 'page' | 'question' | 'search'
  pageId?: string
  text?: string
  evidenceLimit: number // 1..10，UI 默认 5
  methodLimit: number   // 1..8，UI 默认 4
}
```

验证矩阵：

| 条件 | 行为 |
|---|---|
| `kind=page` 且缺 `pageId` | 返回参数错误 |
| `kind=question/search` 且文本少于 2 字符 | 返回参数错误 |
| limit 越界 | Rust clamp 到约定范围 |
| pageId 不存在 | 返回可诊断的“页面不存在”，不返回默认目录 |
| 仓库未打开 | 沿用“请先选择知识库目录”错误 |

### 3.3 响应

```ts
type ResearchTrailRelation =
  | 'outgoing_link'
  | 'backlink'
  | 'wiki_fts'
  | 'book_fts'
  | 'graph_neighbor'
  | 'field_overlap'
  | 'manual'

type ResearchTrailItem = {
  id: string
  kind: 'wiki' | 'book' | 'graph'
  rank: number
  title: string
  snippet: string
  score: number          // 0..1，显示用
  relation: ResearchTrailRelation
  retrievalReason: string
  pageId: string
  pageType: string
  sourcePath: string
  wikilink: string
  bookId: string
  chapterId: string
  physicalPageStart?: number | null
  physicalPageEnd?: number | null
  markdownPath: string
  pdfPath: string
  nodeId: string
  sourceLocation: string
  graphPath: string[]
}

type ResearchTrailResponse = {
  contextKey: string
  anchor: {
    kind: 'page' | 'question' | 'search'
    title: string
    pageId: string
    nodeId: string
  }
  evidence: ResearchTrailItem[]
  methods: ResearchTrailItem[]
  degradedChannels: Array<'wiki' | 'books' | 'graph'>
  generatedAt: string
}
```

Rust 与 TypeScript 全部使用 camelCase 序列化；不允许前端直接 cast 未验证 JSON。

## 4. 候选生成

### 4.1 页面锚点

1. 从 `pages` 读取锚点页面的 title、summary、body、frontmatter、source_path。
2. 从 `wikilinks` 读取出链，并通过既有 ID/stem 解析规则定位目标页面。
3. 反向查询所有指向锚点 ID/stem 的页面。
4. 从标题、摘要、frontmatter 的 keywords/scenario/objectives/constraints/method_family 提取最多 12 个去重词项；正文仅取标题区和前 1,200 字作为补充，不把整页塞入查询。
5. 用词项查询 Wiki FTS5 和核心书籍 FTS5。
6. 通过 `source_path` / `source_file` 把锚点映射到 Graphify 节点；读取一跳边并映射邻居回 Wiki 页面。

### 4.2 问题与搜索锚点

复用 `qa.rs` 的 `query_terms`、Wiki/书籍候选生成和 Graphify 词项命中。为避免规则漂移，公共检索函数变为 `pub(crate)` 或提取到 `retrieval.rs`，由问答与研究脉络共同调用。

### 4.3 方法候选

方法候选使用独立池：

- Wiki FTS 查询限定 `p.page_type='method'`；
- 页面直接链接到 method 时获得结构关系加分；
- scenario/objectives/constraints/method_family 字段词项重合加分；
- Graphify 一跳映射到 method 页面时加分。

方法不能从普通 catalog 截断获得。

## 5. 排序与去重

### 5.1 原始信号

| 信号 | 基础权重 |
|---|---:|
| 页面出链 | 1.00 |
| 页面反向链接 | 0.88 |
| Graphify 直接邻接 | 0.72 |
| 标题 FTS 命中 | 0.68 |
| 正文/摘要 FTS 命中 | 0.52 |
| 核心书籍章节命中 | 0.58 |
| 方法字段重合 | 每字段 +0.08，上限 +0.24 |
| source/synthesis 页面质量加分 | +0.08 |
| Graphify-only 证据折减 | ×0.65 |

具体数值写为 Rust 常量，并通过 fixture 固定排序。SQLite `bm25` 先转为单调相似分，再与结构信号合并；最终分数 clamp 到 `0..1`。

### 5.2 去重键

- Wiki：`wiki:<pageId>`；
- Book：`book:<bookId>:<chapterId>`；
- Graph：`graph:<nodeId>`。

同一候选被多个通道命中时保留最高分，并合并最多三条检索理由和最短 graphPath。自动排名只针对自动候选；手工固定项显示在单独区域，不参与 rank 重排。

### 5.3 稳定顺序

主排序：`score DESC`；平分依次按关系优先级、`kind`、规范化 title、ID。禁止依赖 HashMap/JSON 遍历顺序。

## 6. 前端状态机

```text
idle ── anchor valid ──> loading ── success ──> ready/empty/partial
                           │
                           └─ failure ──> error ── retry ──> loading
```

`useResearchTrail` 维护：

- `requestSequence`：每次请求递增；只有当前 sequence 可发布；
- `cache`：内存 Map，键包含 repository identity、context key、repository generation、graph refresh version；
- `pins`：localStorage v1，按 repository + context key 隔离；
- `status`：idle/loading/ready/partial/empty/error；
- `refresh()`：跳过缓存重新请求。

仓库变化时清空可见状态和内存缓存；Graphify 或索引 generation 变化只使相关缓存键失效。

## 7. 上下文来源集成

### 7.1 页面

`openPage` 成功后设置 page anchor；加载失败或关闭失效页时清理。切到非 page 标签不能继续把旧页面伪装成当前上下文。

### 7.2 问答

`AskView` 增加 `onResearchContextChange`：

- 提交问题时发布 question anchor；
- 打开历史会话时发布最后一个有效用户问题；
- 新会话且无消息时发布 idle；
- 不在 textarea 每次 change 时发布。

### 7.3 文献/方法搜索

沿用现有 query，但用 350ms debounce 生成 search anchor；查询清空立即恢复 idle。请求序列守卫覆盖乱序响应。

## 8. UI 设计

### 8.1 面板头部

- 标题“研究脉络”；
- 上下文副标题“基于：……”；
- 刷新按钮、收起按钮；
- partial 状态显示“部分来源不可用”。

### 8.2 证据链卡片

- 自动证据显示 rank 圆点；固定项显示 pin 图标，不占 rank；
- 关系标签：直接引用、反向引用、全文命中、书籍章节、图谱邻接；
- 相关度百分比只表示本地排序分数，不表述为学术置信度；
- 展示 `retrievalReason`、片段和来源定位；
- Graphify-only 项显示“图谱提示·需核验”。

### 8.3 添加证据选择器

对话框包含 Wiki 与核心书籍两个结果组；输入至少两字符后并行调用 `searchPages` / `searchBookChapters`，使用独立序列守卫。选择后写入 pins；无效路径在下次加载时清理。

### 8.4 图谱聚焦

`GraphView` 增加可选 `targetNodeId`/`targetPath`；打开完整脉络时优先选择锚点节点，点击图谱证据时选择该证据节点。目标不存在时显示可诊断提示并保留总图。

## 9. 错误与降级矩阵

| 场景 | 响应/UI |
|---|---|
| 仓库未选择 | idle 引导，不发请求 |
| 页面不存在 | error + 重新加载页面入口 |
| Wiki 查询失败 | 整体失败；Wiki 是基础真相层 |
| book 表为空/章节索引缺失 | partial，保留 Wiki/Graphify |
| graph.json 不存在或损坏 | partial，保留 Wiki/Books |
| 所有通道无命中 | empty，显示当前锚点与方法库搜索入口 |
| 旧请求后返回 | 丢弃，不改变 UI |
| pins JSON 损坏 | 清空该存储键并继续自动证据 |
| 固定资源已删除 | 忽略该项并回写清理后的 pins |

## 10. 兼容与迁移

1. 不迁移 SQLite schema；检索使用现有 pages/pages_fts/wikilinks/books/book_chapters 表。
2. 新增 localStorage `desktop.research-trail.pins.v1`，解析失败安全回退为空。
3. 现有 `EvidenceItem` 保持问答兼容；研究脉络定义新类型，避免强迫聊天历史迁移。
4. `ResearchTrailPanel` 替换原内联 JSX，但保留 `desktop.context-open` 和当前面板宽度/视觉。

## 11. 性能目标

- 2,700 节点以内的当前 Graphify 图上，本地首次检索目标 P95 ≤ 800ms；缓存命中 ≤ 50ms。
- 单次自动候选上限：Wiki 30、Book 16、Graph 30、Method 20；排序后才截断。
- 搜索锚点 debounce 350ms；不得在 question textarea 输入期间检索。
- 不在 React render 中同步解析 graph.json 或扫描文件系统。

## 12. 回滚

1. 新功能集中在独立 Rust 模块、前端 feature 目录和 App 接线。
2. 若后端命令出现发布阻塞，可回滚到 0.7.2；不得恢复“目录前五项是假证据”的行为，回滚 UI 应显示明确空态。
3. localStorage pins 是可删除派生状态；回滚无需迁移知识库文件。
