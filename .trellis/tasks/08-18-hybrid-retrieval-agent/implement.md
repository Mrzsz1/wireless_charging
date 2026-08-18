# 混合检索与有限 Agentic 实施计划

## Phase 0 — 基线与所有权

- [ ] 确认子任务 1/2 的契约和 feature flags 可用。
- [ ] Git 检查点；记录现有两个移动路径问题的 audit bundle 作为失败基线。
- [ ] 给 `qa.rs` 指定单一 owner，先抽模块再允许并行开发。

## Phase 1 — RetrievalContract

- [ ] 新增 Rust structs、deny_unknown_fields parser、JSON Schema 和完整示例。
- [ ] 更新 Codex/compatible provider planner 原生 schema 调用。
- [ ] 实现不丢完整问题的 deterministic fallback。
- [ ] 测试未知字段、数量边界、Unicode、显式来源和 open scope。

## Phase 2 — SourceResolver 与 query builder

- [ ] 使用 documents/aliases 解析标题和中英文别名。
- [ ] 构建保留完整 concept 的 lexical terms；删除前缀窗口截断依赖。
- [ ] 把 document IDs/kinds/roles/granularities 下推各通道。

## Phase 3 — 独立 channel adapters

- [ ] title/alias channel。
- [ ] unified ContentBlock FTS channel。
- [ ] VectorStore dense channel。
- [ ] Graph hint channel，并强制映射 active ContentBlock。
- [ ] 每通道返回 attempt status 和 typed errors。

## Phase 4 — Fusion/diversity/reranker

- [ ] 通道内排名标准化和 RRF。
- [ ] 稳定 block/document 去重。
- [ ] 显式来源、exact title、reference/graph/fallback 保护/降权规则。
- [ ] Reranker interface、fallback 和测试。

## Phase 5 — Coverage 和 rounds

- [ ] 新增 RetrievalCoverage 和 stop reasons。
- [ ] 删除主路径的 `evidence_sufficient/minimumEvidence`。
- [ ] 实现 gap prompt 和最多两轮补查。
- [ ] 串接全局 timeout/cancel；后续轮次不得重置总预算。

## Phase 6 — 审计和前端进度契约

- [ ] run manifest 写 planner/retriever/reranker/round/channel/gap/stop version。
- [ ] 流事件携带可展示的步骤，不包含模型隐藏思维。
- [ ] 保持页面切换后后台任务状态恢复。

## Phase 7 — 验证与编译

- [ ] 回归：指定书籍、开放 paper+book、新术语、planner failure、dense degraded、graph missing、cancel。
- [ ] 断言所有 requested channels attempted，不能只断言最终有若干候选。
- [ ] 检查 RRF 前后和 reranker 前后 top-k。
- [ ] `cargo fmt --check`。
- [ ] `cargo test query_plan --lib`（迁移期）与新 retrieval 模块测试。
- [ ] `cargo test semantic_query_plan_regressions_recall_auditable_primary_sources --lib`。
- [ ] `cargo build --release`。
- [ ] 提交并记录 legacy/v2 feature flag 与删除旧逻辑的后续条件。

## 回滚

- `rag_retriever_v2=false` 切回 legacy retriever。
- 新 audit 表保留，不影响旧会话。
- 不删除旧 query plan 代码，直到 rollout 子任务通过双读评测。
