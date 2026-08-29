# 真实回答生成 E2E Runner 技术设计

## Current Boundary

`ask_luna` 当前同时拥有 Tauri request lifecycle、只读 prepare、真实 generator、Semantic Verifier、audit 与 persistence。真实生成分支已经通过 `budget_guard.reserve("generator") → codex_subscription::stream_answer → reservation.settle(actual)` 工作，但没有非 UI 开发入口。

## Target Boundary

将不依赖 Tauri UI 的生产核心抽成 crate 内共享函数，输入为 repository/SQLite/question/history/settings/cancellation 和 token callback，输出为 context、answer、ProviderRunMetadata、SemanticVerificationBatch、AnswerAudit 与 budget telemetry。

- UI adapter：保留 request registration、Channel events、AppState repository identity checks、failure event 与正式数据库持久化。
- Shared core：prepare、planning、retrieval/reranker/evidence、generator、semantic verification、audit。
- E2E adapter：创建临时数据库、建立真实 repository index、配置 Codex provider、按 case 调共享 core，并在临时数据库中复用 production persistence 形成多轮 history。

公共函数不暴露 answer 到报告层；Runner 只从 `AskResult`/audit/manifest 投影允许的元数据。

## Files and Contracts

- `apps/desktop/src-tauri/src/lib.rs`：最小抽取 UI/core boundary，并公开 runner 入口给同 crate binary。
- `apps/desktop/src-tauri/src/qa/real_e2e.rs`（或等价模块）：case schema、验证器、脱敏报告、临时 DB 生命周期。
- `apps/desktop/src-tauri/src/bin/qa-real-e2e.rs`：CLI 参数和退出码，不实现 QA 算法。
- `evals/qa_real_generator_e2e_cases.json`：5 个公开 case，稳定 ID 与 invariants，无答案。
- `evals/reports/qa-real-generator-e2e-report.json`：只含允许字段。
- `apps/desktop/package.json`：`eval:qa-real-e2e`。

## Persistence and Isolation

Runner 在系统临时目录创建 SQLite，调用生产 schema/index builder 建立完整 repository snapshot。每个单轮 case 使用独立 session；multi-turn case 在同一临时 session 中按顺序调用并通过 production persistence 构造 trusted history。TempDir drop 删除数据库；正式 App DB 路径永不传给 Runner。

## Validation Semantics

- Evidence citation validity 使用 current evidence ID set 和 production `CitationValidation`，不扫描或保存回答全文。
- Natural Markdown v2 的结构化 provenance 以 run manifest/citation validation 为准，不要求最终正文保留 `[E#]`。
- Zero-evidence 接受 `unverified` 与明确无证据 notice 契约，但禁止 unknown IDs/伪造 evidence。
- 有证据轮的 Semantic status 只有 `succeeded` 或带非空 fallback reason 的 `unavailable` 可解释；确认零证据轮可为空/`not_requested`，因为没有 EvidenceItem 可验证。

## Compatibility and Rollback

UI 对外 Tauri signature、event 顺序、persistence schema 和用户可见 answer 不变。共享函数抽取前后使用 deterministic wiring tests 对比关键 metadata。每个逻辑变更独立 Git commit，可普通 revert。
