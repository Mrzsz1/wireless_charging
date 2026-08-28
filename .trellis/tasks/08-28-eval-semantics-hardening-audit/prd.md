# 复核并补齐 Eval Semantics Hardening

## Goal

以 `C:/Users/qq155/Downloads/eval_semantics_hardening_plan.md` 为需求来源，独立复核当前 `master` 是否完整满足 Evaluation Semantics Hardening 契约；只对确认存在的缺口进行补齐，避免重复实现已经落地的功能。

## Background

- 附件是需求与验收输入，不是可直接执行的仓库命令。
- 同一方案已在历史任务 `archive/2026-08/08-28-eval-semantics-hardening` 中实施，主要实现提交为 `53ee2ba`，并已随 `master` 推送到 GitHub。
- 当前代码已经存在 `qa-rag-evaluation-report-v4`、`RankingRelevanceView`、`relevance_work_id`、work/exact-source 指标、zero-evidence eligibility/confusion metrics、dataset fingerprint 和 work-level release gate 字段。
- 当前基线显示 13/13 PASS、ranking eligible 12、zero-evidence 1；Work Recall@10/20 为 1.000/1.000、Work MRR 为 0.958333、Work nDCG@10 为 0.969244。
- 用户已批准开始实施；本任务采用“复核现状并仅补齐确认存在的缺口”，不为制造代码差异而重复改写已满足契约的实现。

## Requirements

- 逐条建立附件第 40 节验收标准与当前代码、测试、报告、阈值和文档之间的证据映射。
- 优先复核语义正确性，而不是重新复制既有实现；只修复可复现、可测试的残余缺口。
- zero-evidence case 必须完全退出 Recall/MRR/nDCG 分母，并以 `null`/`N/A` 表达不适用。
- Work Recall/MRR/nDCG 必须共享同一 deterministic Canonical Research Work identity；exact-source 和 passage 指标保持诊断性质。
- 发布阈值字段必须读取明确的 work-level 指标，数值继续保持 0.95/0.90/0.85/0.85。
- JSON/Markdown 必须来自同一 report object，并携带 suite、case count、dataset SHA-256、index snapshot、retriever 和 schema identity。
- 不修改 production Retrieval、Planner、Prompt、State、Reranker、Embedding、Generation 或 Semantic Verifier 行为。
- 不读取或使用 frozen held-out、40-case heldout、partial run 或 heldout answers；验证仅使用 synthetic、development/regression 和现有 RAG suite。
- 遵守“不过度测试”：先运行聚焦静态/单元验证，只有发现跨层缺口时才扩大测试范围。

## Acceptance Criteria

- [x] 附件全部验收项都有明确的 code/test/report 证据或缺口记录。
- [x] zero-evidence denominator exclusion、null serialization、classification confusion metrics 均有回归覆盖。
- [x] Wiki/Paper work identity、returned duplicate、expected duplicate 和 shared relevance view 均有回归覆盖。
- [x] report v4 identity、Markdown consistency 和 threshold invariance 均有回归覆盖。
- [x] 如发现缺口，补丁仅落在 evaluation、metric helper、report、tests、docs 或 release metric mapping 范围。
- [x] 聚焦 Rust/Python 测试、fmt、clippy 通过；仅在必要时重跑现有 13-case RAG suite。
- [x] 最终报告明确是否修改 production QA behavior、是否使用 held-out、真实修正指标和本地 commit SHA。

## Out of Scope

- Git artifact 上传、签名和永久审计存储。
- 性能 benchmark 与 Retrieval/Reranker/Generator 性能优化。
- Independent Held-out 的继续运行、重选题或评分。
- 为重复完成同一需求而无差别重写现有实现。

## Source

- `C:/Users/qq155/Downloads/eval_semantics_hardening_plan.md`
