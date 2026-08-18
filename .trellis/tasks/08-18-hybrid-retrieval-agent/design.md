# 混合检索与有限 Agentic 技术设计

## 1. 当前缺陷

- `query_plan.rs` 把问题限制为四种 answer profile，并要求 2–12 条 minimumEvidence。
- `query_terms` 对总词数截断，中文 fragment/bigram 从长串前部 `.take()`，会丢失尾部核心概念。
- `evidence_sufficient` 将候选数量、required kinds 和关键词 facet 匹配当成停止条件。
- `retrieve_pass` 虽有多通道，但 book/paper/wiki 使用不同旧表，无法共享显式来源过滤和统一块定位。
- 当前 fusion 叠加手工 bonus，解释性存在但跨通道量纲不稳。

## 2. 模块

```text
qa/retrieval_contract.rs
qa/source_resolver.rs
qa/retrieval.rs
qa/fusion.rs
qa/reranker.rs
qa/coverage.rs
qa/retrieval_audit.rs
```

`qa.rs` 只保留 orchestration facade。旧 `query_plan.rs` 作为 feature flag fallback，最终 rollout 后删除。

## 3. Planner fallback

Provider 成功时返回严格 schema。失败时 deterministic fallback：

- `scope.mode=open`；
- `concepts=[完整原问题]`；
- 从显式别名索引解析 sources；
- requestedKinds 默认 `[wiki,paper,book]`，但仅当 UI/问题明确限制时缩小；
- maxRounds=1（无 LLM 补查），不能生成 minimum evidence。

fallback 必须保留完整 Unicode 问题；词法 query builder 可后续分词，但不得先截掉后半句。

## 4. Round pipeline

```text
contract + round queries
 -> resolve explicit document IDs
 -> run requested channels independently
 -> normalize within channel
 -> RRF by stable block ID
 -> protect explicit-source and exact-title hits
 -> semantic/document diversity
 -> rerank top-N
 -> coverage snapshot
 -> stop or plan next round
```

每个 round 生成 immutable audit record。第二轮结果不能仅因“新一轮 RRF”自动压过首轮强命中；融合保留 origin round 和 exact match 保护。

## 5. Reranker

定义 `Reranker` 接口。首选本地 cross-encoder 或可配置 Provider；不可用时使用可解释 deterministic features。Reranker 只重排 top-N，不从零召回。输出记录 model/version/score；显式来源 exact hit 只在明显无正文相关性时降低，但不能被其他文档完全挤出。

## 6. Coverage controller

Coverage controller 只回答“是否值得继续检索”：

- mustAttemptKinds 是否全部 attempted；
- explicitSources 是否解析并检索；
- required facets 是否存在正向 signal；
- 当前证据是否全为 reference/graph/fallback；
- 是否有冲突；
- 本轮新增唯一文档/块；
- 剩余 time/token/query budget。

它返回 action：`stop(reason)` 或 `continue(gaps, allowedKinds, remainingBudget)`，不返回事实置信度。

## 7. Query expansion

LLM 根据 gap 生成中英表达、相关标准问题名和别名候选。代码：去重、最大字符数、每 facet 上限、总查询上限、禁止外搜、保持 document filters。索引候选标题可提供词汇，但不能把首轮噪声无条件扩成新主题。

## 8. UI events

流事件增加 round/channel/progress/stop reason。UI 可显示“正在检索书籍/论文/语义索引”，但不暴露原始内部思维链，只显示操作和耗时。
