# 智能问答语义 RAG 与自适应上下文重构

## Goal

把当前关键词意图和固定历史轮数驱动的智能问答升级为：证据跨轮可见但严格隔离、历史仅受模型 token 窗口约束、本地混合语义检索、结构化 Query Plan、Facet 证据覆盖和可复现评估组成的科研问答 Agent。

## Requirements

- 上一轮证据在本轮检索期间继续可见，但明确标注且绝不参与本轮 Prompt、Schema、引用或审计。
- 移除 1–8 个最近问答轮数的产品限制，按模型窗口尽可能保留完整问答，超限后分层压缩并重新预算。
- 保留 SQLite FTS/BM25 和 Graphify，新增本地语义向量通道并做混合融合，不能用纯向量替代精确检索。
- 用开放式、多标签 Query Plan 和 Facet 证据要求替换四种关键词硬路由；无法规划时使用通用研究计划。
- 第一轮使用原始查询、确定性扩展和语义检索；Facet 覆盖不足时才请求 Codex 生成受控子查询。
- LLM 生成的查询只用于检索，永远不能作为回答证据。
- 保留结构化回答、Provider JSON Schema、引用检查、失败隔离、零证据和审计合同。

## Acceptance Criteria

- [x] 五个子任务均通过各自验收并归档。
- [x] 同一会话历史在数据库中无轮数上限，Prompt 选择由 token 预算决定。
- [x] 非词表中英文释义问题可通过语义通道召回相关论文/Wiki/书籍。
- [x] 复合问题产生多 Facet 计划，早停依据 Facet 覆盖而非固定候选数量。
- [x] 全量 Rust 测试、前端构建和 Tauri release 编译通过。

## Subtasks

1. `08-17-qa-evidence-panel-state`：证据侧栏轮次隔离。
2. `08-17-qa-adaptive-history-context`：Token 自适应历史与分层压缩。
3. `08-17-qa-hybrid-vector-retrieval`：本地混合向量检索。
4. `08-17-qa-query-planner-facets`：语义 Query Planner 与 Facet Agent。
5. `08-17-qa-semantic-rag-evals`：语义 RAG 回归评估。

## Ordering

按 1→2→3→4→5 实施；后续子任务建立在前序稳定合同上。

## Out of Scope

- 自动判断引用语义是否蕴含结论。
- 将知识库正文上传到第三方 embedding 服务。
- 删除现有 FTS、Graphify、审计或结构化回答链路。
