# Markdown 科研混合 Agentic RAG 总体设计

## 1. 设计边界

本任务只改造桌面端智能问答及其可重建索引。Markdown 正文继续由现有 Raw/Wiki 治理规则管理；问答系统只读正文，不写回知识内容。SQLite 会话数据继续本地持久化。远程 pgvector 是可配置的向量能力，不是正文存储，也不能成为离线词法检索的单点故障。

## 2. 目标数据流

```text
Markdown files
  -> MarkdownCorpusIndexer
     -> DocumentRecord
     -> ContentBlock(document/section/subsection/semantic)
     -> aliases + roles + stable locators
  -> FTS5 derived index
  -> EmbeddingPipeline
     -> LocalVectorStore (offline/cache)
     -> PgVectorStore (configured remote)

User question + trusted conversation memory
  -> RetrievalContractPlanner (provider JSON Schema)
  -> SourceResolver / AliasResolver
  -> RetrievalOrchestrator
       lexical(title/alias/FTS)
       dense(vector)
       metadata-filtered source channels
       graph hints
  -> RRF Fusion -> Diversity -> Reranker
  -> CoverageController
       stop OR bounded follow-up queries (max 2)
  -> ContextBudgeter
  -> AnswerProvider (natural Markdown)
  -> EvidenceAppendixBuilder (deterministic)
  -> persistence + UI deep links + audit
```

## 3. 统一核心契约

### 3.1 DocumentRecord

```text
id, kind, canonicalTitle, aliases[], authors[], year, tags[],
markdownPath, contentHash, provenance, updatedAt
```

`kind` 至少支持 `wiki | paper | book`。Graphify 不进入 DocumentRecord 正文集合，只保留独立 graph hint 类型。

### 3.2 ContentBlock

```text
id, documentId, parentBlockId?, granularity,
heading, headingPath[], role, ordinal,
lineStart?, lineEnd?, markdownPath,
content, contentHash, embeddingText, active
```

稳定 ID 由 `documentId + normalized heading path + local ordinal/content fingerprint` 生成。不能只使用行号，因为 Markdown 编辑会导致行号漂移。

### 3.3 SourceLocator

```text
documentId, blockId, headingPath[], markdownPath,
lineStart?, lineEnd?, contentHash, snapshotId
```

打开顺序：block ID -> heading path -> line range -> 显示“原定位已变化”的文档级降级页。所有路径必须经过仓库根路径边界校验。

### 3.4 RetrievalContract

```text
schemaVersion
scope { mode, explicitSources[] }
requestedKinds[]
mustAttemptKinds[]
concepts[]
aliases[]
relatedProblems[]
facets[] { id, label, required, searchQueries[], preferredKinds[] }
budget { maxRounds, maxQueries, maxCandidates }
```

这里没有固定 answerProfile，也没有 `minimumEvidence`。Provider 负责语义规划，后端负责允许值、数量、长度、来源边界和 fail-soft fallback。

### 3.5 RetrievalCoverage

```text
rounds[]
channelAttempts[] { kind, attempted, succeeded, hitCount, latency, error? }
requestedKindsCovered[]
explicitSourcesResolved[]
facetSignals[]
newCandidateGain
conflicts[]
gaps[]
stopReason
```

Coverage 是继续检索的软控制信号，不是对答案真假的硬判定。唯一硬约束是：显式请求的通道必须尝试；证据必须存在且可定位；Graph hint 不得冒充正文证据。

## 4. 存储布局

### 4.1 本地 SQLite

保存统一文档/块元数据、FTS、别名、会话、审计索引和远程同步状态。建议新增版本化表而不是一次性破坏旧表：

- `documents_v2`
- `document_aliases_v2`
- `content_blocks_v2`
- `content_blocks_fts_v2`
- `embedding_records_v2`
- `retrieval_runs_v2`
- `retrieval_rounds_v2`

迁移期保留旧 `pages`、`paper_sections`、`books`、`book_chapters` 的只读适配，直到新回归通过。

### 4.2 向量存储

定义 `VectorStore` 能力：`health`、`upsert`、`delete_snapshot`、`query`、`stats`。首个远程实现为 PostgreSQL + pgvector；连接配置与凭据隔离。离线实现继续支持本地持久化，但升级为按块键、模型和内容哈希存储，不再将整个仓库绑成单个不可解释二进制快照。

远程不可用时：

1. 优先查询已存在的本地向量；
2. 若本地向量也不可用，继续标题/别名/FTS/Graph 通道；
3. UI 和审计显示 semantic degraded，而不是宣称“知识库无结果”。

## 5. 检索与排序

每种来源类型维护独立 channel attempt。标题/别名先完成来源解析；显式来源转成 document ID filter。每个通道返回统一 `RetrievalCandidate`，包含原始分数、通道、轮次、匹配原因和 locator。

融合顺序：通道内归一化 -> RRF 融合 -> 稳定去重 -> 来源/文档多样性控制 -> reranker -> 上下文选择。Reranker 必须有 fail-soft 路径；任何远程 reranker 都不能成为回答的必需依赖。

## 6. Agentic 控制

Agent 只做受限动作：生成补充查询、选择尚未尝试的来源类型、根据缺口改写查询。它不能写知识库、不能外搜、不能扩大到未请求的来源，也不能自行宣布来源存在。

默认配置：首轮 + 最多两轮补查、每轮每 facet 最多 2 个查询、总查询数上限、全局超时和取消令牌。每轮之后计算新增唯一块数和覆盖变化；连续无新增即停止。

## 7. 回答与证据

最终 Provider 只输出 Markdown 正文。旧 `qa-structured-answer-v1` 不再作为用户答案主契约，旧固定章节/claim completeness 不再阻断回答。后端将实际选中的 ContentBlock 转为短证据条目并追加 `参考证据`。前端把 E1 映射到结构化 SourceLocator，而不是信任 Markdown 中的原始路径。

## 8. 上下文

保留 completed/trusted-only 历史原则。预算算法遍历尽可能多的最近完整轮次；超预算时压缩更早轮次为来源剥离后的可信摘要，再重新计算 query、evidence、history 和 output reserve。压缩状态和消息 ID 写入 run manifest。

## 9. 兼容、发布与回滚

- 使用 schema/retriever/prompt 版本区分旧会话和新会话。
- 双读阶段可在审计模式并行运行 legacy 与 v2 检索，但只向用户展示一个结果。
- 开关：`rag_index_v2`、`rag_retriever_v2`、`rag_answer_v2`、`remote_vector_enabled`。
- 回滚只切回旧读路径并重建派生索引；不删除 Markdown、会话或旧证据快照。
- 远程向量数据按 repository ID + snapshot ID 隔离，删除远程索引不影响本地正文。

## 10. 关键风险

1. Markdown 标题变化导致 locator 漂移：使用 block fingerprint 和多级 fallback。
2. 重复 MinerU/完整稿造成重复：按 provenance、正文 hash 和 canonical document ID 去重。
3. 向量成本与延迟：增量 hash、批处理、局部查询和本地降级。
4. LLM 扩展漂移：来源过滤、数量限制、查询审计和最大轮次。
5. Reranker 误杀：保留通道强命中和显式来源保护，评测比较排序前后。
6. 远程免费额度/休眠：健康检查、离线缓存、错误可见和非阻断降级。
