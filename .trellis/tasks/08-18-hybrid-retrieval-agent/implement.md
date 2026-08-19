# 混合检索与有限 Agentic 实施计划

## Phase 0 — 基线与所有权

- [x] 确认子任务 1/2 的契约和 feature flags 可用。
- [x] Git 检查点；记录现有两个移动路径问题的 audit bundle 作为失败基线。
- [x] 给 `qa.rs` 指定单一 owner，先抽模块再允许并行开发。

## Phase 1 — RetrievalContract

- [x] 新增 Rust structs、deny_unknown_fields parser、JSON Schema 和完整示例。
- [x] Codex planner 使用原生 output schema；compatible provider 继续按既有 prompt-contract 降级，等待其能力探测子任务。
- [x] 实现不丢完整问题的 deterministic fallback。
- [x] 测试未知字段、数量边界、Unicode、显式来源和 open scope。

## Phase 2 — SourceResolver 与 query builder

- [x] 使用 documents/aliases 解析标题和中英文别名。
- [x] 构建保留完整 concept 的 lexical terms；删除前缀窗口截断依赖。
- [x] 把 document IDs/kinds/roles/granularities 下推各通道。

## Phase 3 — 独立 channel adapters

- [x] title/alias channel。
- [x] unified ContentBlock FTS channel。
- [x] VectorStore dense channel。
- [x] Graph hint channel，并强制映射 active ContentBlock。
- [x] 每通道返回 attempt status 和 typed errors。

## Phase 4 — Fusion/diversity/reranker

- [x] 通道内排名标准化和 RRF。
- [x] 稳定 block/document 去重。
- [x] 显式来源、exact title、reference/graph/fallback 保护/降权规则。
- [x] Reranker interface、fallback 和测试。

## Phase 5 — Coverage 和 rounds

- [x] 新增 RetrievalCoverage 和 stop reasons。
- [x] 删除 v2 主路径的 `evidence_sufficient/minimumEvidence`。
- [x] Provider 在 RetrievalContract 中生成 gap/facet 查询，Coverage 按需释放，最多两轮补查。
- [x] 串接全局 timeout/cancel；后续轮次共享总查询预算且不得超过三轮。

## Phase 6 — 审计和前端进度契约

- [x] run manifest 写 planner/retriever/reranker/round/channel/gap/stop version。
- [ ] 每轮首批证据的增量流事件由子任务 4 与回答/证据 UI 一并接入；本子任务已提供 round/channel/status 契约。
- [x] 检索继续运行在现有后台 QA worker，页面切换不取消共享 request 状态。

## Phase 7 — 验证与编译

- [x] 回归：指定书籍、开放 paper+book、新术语、planner failure、dense degraded、graph missing、cancel。
- [x] 断言所有 requested channels attempted，不能只断言最终有若干候选。
- [x] 检查 RRF 与 reranker 的排序保护/降权行为。
- [x] `cargo fmt --check`。
- [x] `cargo test --lib`（含迁移期 query_plan 与新 retrieval 模块测试）。
- [x] `cargo test semantic_query_plan_regressions_recall_auditable_primary_sources --lib`。
- [x] `cargo build --release`。
- [x] 记录 legacy/v2 feature flag、open dual-read 与删除旧逻辑的后续条件；提交在 Trellis 3.4 完成。

## 回滚

- `rag_retriever_v2=false` 切回 legacy retriever。
- 新 audit 表保留，不影响旧会话。
- 不删除旧 query plan 代码，直到 rollout 子任务通过双读评测。
