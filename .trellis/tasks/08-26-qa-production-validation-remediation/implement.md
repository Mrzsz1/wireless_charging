# Implementation Plan — QA Production Validation Remediation

## Phase A — MRR Diagnostics and Ranking

- [x] 扩展 RAG evaluator，生成安全的 per-query `mrr_diagnostics.json`。
- [x] 明确 document/passage MRR，并分析当前最差排名原因。
- [x] 检查 clean reranker query、exact passage input、rerank 前去重和 parent expansion 顺序。
- [x] 仅做通用 score fusion/diversification修复；诊断确认根因是 Wiki-primary pair 指标身份，未盲调 Retriever/权重或增加题目特判。
- [x] 运行真实 Cross-Encoder A/B，满足全部冻结 retrieval 阈值。
- [x] 本地提交：`fix(qa): improve document-level reranking`。

## Phase B — Real Semantic Verification

- [x] 建立 100 条 frozen claim-evidence benchmark 与 schema/hash 校验。
- [x] 新增真实 Codex/Compatible API benchmark CLI，复用生产 structured transport。
- [x] 输出真实 provider/model/config 与 accuracy/failure/fallback 指标。
- [x] 无 Provider、timeout、invalid JSON 与取消保持 fail closed。
- [x] 本地提交：`feat(qa): benchmark real semantic verification`。

## Phase C — Unified Production Eval Harness

- [x] 实现 `qa-production-eval` orchestrator 和同一 Git SHA 输出目录。
- [x] 实现 canonical 50-case conversation evaluator 与三项指标工件。
- [x] 自动收集 retrieval/reliability/reranker/semantic/performance，不允许人工填数。
- [x] 接入 release gate/report，验证缺失外部输入时诚实 FAIL（15/30 PASS）。
- [x] 本地提交：`feat(qa): generate production evaluation artifacts`。

## Phase D — Independent Held-out Tooling

- [x] 增加 50 题独立 curator 模板和 canonical method/constraint schema。
- [x] 实现 freeze seal、同一 RC run bundle 和 blind reviewer export。
- [x] 实现 A/B review 与 C adjudication 导入校验。
- [x] 从同一 held-out 派生 heldout/grounding/open-research 工件。
- [x] 外部人员输入未到位时保留 pending/FAIL。
- [x] 本地提交：`feat(qa): add independent heldout review workflow`。

## Phase E — Performance Profile and Benchmark

- [x] 分离 model load/input prepare/inference telemetry。
- [x] 验证模型 session 复用、dedup/candidate cap/batch 顺序。
- [x] 新增 sealed target profile，按 ExecutionMode 定义 SLO。
- [x] 输出 cold 与 warm P50/P95/P99 `performance.json`。
- [x] 重跑统一 gate；外部 held-out 未完成前不得宣告 PASS（19/31）。
- [x] 本地提交：`perf(qa): measure production reranker latency`（`7dba584`）。

## Final Verification

- [x] Rust fmt/test/clippy（234 passed，2 个需外部模型的 ignored tests）。
- [x] Python evaluator/release tests（83 passed）。
- [x] Frontend typecheck/tests/build/P3。
- [x] 真实 RAG、360 questions、Wiki/core-book。
- [x] 更新 QA spec、baselines、release report 与任务 AC。
- [x] 本地提交：`docs(qa): record remediation release status`。

## Invariants

- 不降低 `evals/qa_release_thresholds.json`。
- 不使用 independent held-out 调参。
- 不把 fixture/fake provider 标记成 production measurement。
- 不修改 raw 文献或用户已有未跟踪文件。
- 不 push GitHub。
