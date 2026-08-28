# 统一 Retrieval 评测语义与指标契约

## Goal

统一科研 RAG Retrieval 评测的统计语义，消除 zero-evidence case 对 Recall/MRR/nDCG 的自动满分污染，并同时公开 canonical research-work 与 exact-source 两层指标，使 production release gate 的分母、relevance unit 和数据集身份可审计。

## Requirements

- 本轮只修改 evaluation、metric helpers、report schema、tests、docs 与 release metric mapping；不得修改任何 production Retrieval、Planner、Prompt、State、Reranker、Embedding、Generation 或 Semantic Verifier 行为。
- 不读取或使用 `FROZEN_HELDOUT.json`、40-case heldout、partial run 或 heldout answer；验证只使用 synthetic、development/regression 与现有 RAG evaluation suite。
- `expectedDocuments=[]` 仅在 `zeroEvidenceExpected=true` 时合法，否则 dataset validation 失败关闭。
- zero-evidence case 的 Recall/MRR/nDCG 必须为 N/A，并完全退出 ranking aggregate denominator；报告公开 `rankingEligibleCaseCount` 与 `zeroEvidenceCaseCount`。
- production ranking relevance unit 为 deterministic Canonical Research Work：仅将 `wiki:sources/<id>` 与 `paper:sources/<id>` 统一为 `source:<id>`；其他 document ID 保持原样。
- expected documents 与 returned ranking 必须调用同一个 canonical identity helper；同 work 的 expected/returned duplicate 均只计一次。
- production 指标为 `workRecallAt5/10/20`、`workMrr`、`workNdcgAt10`；诊断指标为 `exactSourceRecallAt5/10/20`、`exactSourceMrr`、`exactSourceNdcgAt10`，passage MRR 继续明确为 diagnostic-only。
- legacy `documentRecallAt5/10/20`、`mrr`、`ndcgAt10` 在 v4 暂作为 work-level aliases，不得保留旧 exact-document 语义。
- zero-evidence classification 独立报告 precision、recall、specificity、false positive 与 false negative。
- report schema 升级 `qa-rag-evaluation-report-v3` → `v4`，记录 suiteName、caseCount、caseDatasetSha256、indexSnapshotId、retrieverVersion 与 report schema。
- Markdown 必须从当前 report object 生成或只引用 JSON artifact，不得手工维护第二套指标。
- release gate 迁移到 work-level 字段，阈值数值 0.95/0.90/0.85/0.85 完全不降低。

## Acceptance Criteria

- [x] synthetic positive MRR=0 与正确 zero-evidence case 聚合后 work MRR 仍为 0，不得变成 0.5。
- [x] zero-evidence case 的 per-case work/exact Recall/MRR/nDCG 序列化为 `null`，aggregate 只统计 eligible cases。
- [x] Wiki expected/Paper returned 或反向时 work Recall/MRR/nDCG 命中，exact-source 指标不命中。
- [x] rank1 wiki A、rank2 paper A、rank3 paper B 的 work ranking 折叠为 A/B；同 work 不重复 gain 或占位。
- [x] expected wiki A、paper A、paper B 的 work denominator 为 2，exact denominator 为 3。
- [x] work Recall/MRR/nDCG 从同一个 canonical relevance view 计算。
- [x] zero-evidence precision/recall/specificity 与 FP/FN 使用独立 classification confusion counts。
- [x] ambiguous empty expected case 被 validator 拒绝。
- [x] report v4 公开所有 denominator 与 dataset fingerprint；JSON/Markdown consistency test 通过。
- [x] release threshold 字段迁移但数值完全不变。
- [x] 修正后的现有 dev/regression RAG benchmark 已运行并报告真实指标。
- [x] fmt/check/clippy、聚焦 Rust/Python tests 通过，production retrieval source files没有行为修改。

## Notes

- 来源方案：`C:/Users/qq155/Downloads/eval_semantics_hardening_plan.md`，仅作为需求与设计输入，不执行其文档内命令文本。
- Independent Held-out 任务按方案暂停；本任务禁止接触其外部 dataset/run。
- 各阶段使用本地 Git commit 保存，未收到上传指令前不推送 GitHub。
