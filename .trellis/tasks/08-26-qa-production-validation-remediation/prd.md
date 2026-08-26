# 智能问答生产验证剩余问题整改

## Goal

按最新审查指示把当前 `FAIL — 10/30` 的根因收敛为五条工作流：提升真实 RAG 的 document-level MRR、运行真实 Semantic VerificationProvider、自动生成统一生产评测工件、提供独立 held-out 冻结/盲审/裁决导入流程、冻结并测量目标机器性能。保持现有 Research QA 架构、Retriever、Graphify、Intent 和门禁阈值不变。

## Requirements

### R1 — MRR diagnostics and ranking correction

- 先生成每题 `mrr_diagnostics.json`，记录 document-level 与 passage-level 首个 relevant rank、Top10 stable source/document identity、检索通道、RRF/base/Cross-Encoder/final score 和重复文档位置。
- 明确 Production Gate 使用 document-level MRR；同文档重复 passage 不得虚假扩大 rank 分母，同时保留 passage MRR 供诊断。
- Cross-Encoder 输入只包含 resolved question、关键 objective/constraints、title、heading 与 exact passage；parent context 只能在 rerank 后扩展。
- Rerank query 不拼接完整 QueryPlan、全部 aliases/related problems/subqueries。
- 在 rerank 前稳定去重，在 rerank 后施加有界 same-document duplicate penalty；不禁止同一论文的多个真正高相关 section。
- 不增加 Retriever、Agent、Intent、Graph 通道或题目/路径特判；不降低阈值。

### R2 — Real Semantic Verifier benchmark

- 使用真实 Codex 或 Compatible API 运行独立的 frozen claim-evidence benchmark，不能把 fake provider contract 标记为真实测量。
- Benchmark 至少覆盖 entailed/contradicted/unknown、数字扩大、范围扩大、因果扩大、否定反转、条件缺失与多证据组合。
- Verifier 只依据给定 Evidence，不使用世界知识补充；输出必须满足 closed structured schema。
- 记录 provider/model/config、accuracy、contradiction recall、unknown precision、timeout、invalid JSON 与 fallback rate；只有完整真实运行才设置 `realProviderMeasured=true`。
- deterministic verifier 始终保留为 fail-soft fallback。

### R3 — Unified production artifact generator

- 新增一个 `qa-production-eval` 入口，统一冻结 metadata 并生成 retrieval/conversation/reranker/semantic/performance 工件。
- Conversation evaluator 使用 canonical entities/constraints/objectives，不做自然语言全文 exact match；指标定义固定为 reference resolution、critical constraint preservation 和 active objective preservation。
- 所有工件使用 `qa-eval-metadata-v1`，通过 `.part` 原子写入，不含凭据、原始 Provider payload 或绝对路径。
- 缺失人工 held-out 时仍能生成诚实的 pending/FAIL 状态，不能人工填入通过数值。

### R4 — Canonical independent held-out workflow

- Grounding、Open Research 与 Held-out 必须来自同一批 Canonical Production Held-out 运行，不建立三套互相不一致的数据。
- 提供 50 题模板、canonical method/constraint ID schema、freeze seal、run bundle、blind reviewer export/import 和 adjudication 校验。
- Reviewer 输入不包含系统自己的 verification verdict；两名 primary reviewer 独立，分歧由第三名 reviewer 裁决。
- 独立题目和人工 verdict 仍由真实外部人员提供；仓库工具不得自行伪造生产真值。

### R5 — Performance profile and benchmark

- Reranker telemetry 分离 model load、input prepare 和 inference，并记录 candidate/batch/token 数。
- 确认模型 session 进程内复用；先去重再 candidate cap/batch rerank，parent expansion 在 rerank 后。
- 性能 profile 在测量前冻结，分别定义 Direct/Research/Exploratory SLO，并分别记录 cold start 与 warm P50/P95/P99。
- 模型和评测数据继续位于非系统盘；最终生成 `performance.json`，不根据结果反向放宽阈值。

## Acceptance Criteria

- [x] AC1：`mrr_diagnostics.json` 可定位每题 document/passage 首个 relevant rank 与完整安全评分分解。
- [x] AC2：真实 RAG MRR ≥ 0.85，同时 Recall@20 ≥ 0.95、Recall@10 ≥ 0.90、nDCG@10 ≥ 0.85、fallback ≤ 0.05。
- [x] AC3：真实 Semantic Provider benchmark 生成有效工件，且 invalid verified state = 0；无真实 Provider 时保持 FAIL。
- [x] AC4：统一 harness 自动生成 conversation/reranker/semantic/performance 工件并执行 release gate。
- [x] AC5：Conversation 三项指标来自 canonical gold 与实际输出，不来自手填数值。
- [ ] AC6：held-out tooling 对 seal、独立双审、第三人裁决、blind export 和同一 RC 派生工件 fail closed。
- [ ] AC7：目标机器 profile 在性能测量前冻结；cold 与各 ExecutionMode warm P95 可审计。
- [ ] AC8：全量 Rust/Python/frontend/RAG/Wiki/core-book 门禁通过，不引入新 Retriever/Agent/Intent 或阈值降低。
- [ ] AC9：每阶段独立本地 Git commit，不 push GitHub，不包含用户已有未跟踪文件。

## External Completion Boundary

- 独立 held-out 的问题、两名 reviewer verdict 和分歧裁决属于外部生产证据。技术实现完成后，若这些输入尚未提供，Release Gate 必须继续 FAIL，并明确列出待人工完成项。
