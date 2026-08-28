# Design — Eval Semantics Hardening Audit

## Boundary

把当前 Retrieval evaluator 当作待审计对象，把 production retrieval pipeline 当作只读依赖。审计允许读取和测试 ranking projection，但不改变候选生成、召回、融合或重排行为。

## Evidence Matrix

为附件验收项建立四层证据：

1. **Contract**：report schema、field nullability、canonical identity 和 denominator 定义。
2. **Implementation**：`relevance_work_id`、`RankingRelevanceView`、per-case eligibility、aggregate filtering 和 release mapping。
3. **Regression**：zero-evidence、Wiki/Paper surface mismatch、returned/expected duplicate、report/Markdown、threshold invariance。
4. **Observed artifact**：现有 dev/regression report 的 schema、dataset SHA、eligible count 和真实指标。

只有某层缺失或行为与契约不一致时才进入补丁阶段。

## Canonical Metric Contract

- Work identity 仅合并 `wiki:sources/<id>` 与 `paper:sources/<id>` 为 `source:<id>`。
- Returned ranking 和 expected set 使用同一个 identity function 并分别去重。
- Work Recall、MRR、binary nDCG 共享一个 `RankingRelevanceView`。
- Exact-source view 使用原始 document ID；passage MRR 是 diagnostic-only。
- `expectedDocuments=[]` 只有 `zeroEvidenceExpected=true` 时合法；这些 case 的 ranking metrics 为 `None`。

## Verification Strategy

- 先做 source/diff/schema 静态复核。
- 再运行聚焦 evaluation/metrics Rust tests 与 Python release-gate tests。
- 运行 fmt/clippy 作为最终代码门禁。
- 仅当当前 report 缺失、数据集变化或补丁影响跨层输出时，重跑 13-case RAG suite。

## Compatibility and Rollback

- v4 legacy aliases继续精确映射到 Work 指标。
- 阈值字段迁移不改变数字。
- 若新补丁造成 production behavior diff，立即回滚该补丁并将问题记录为越界。

