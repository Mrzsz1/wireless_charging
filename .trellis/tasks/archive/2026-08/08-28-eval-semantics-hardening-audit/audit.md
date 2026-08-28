# Eval Semantics Hardening Audit

## Conclusion

当前 `master` 已实现附件的全部功能契约。复核确认一个测试覆盖缺口：zero-evidence 的关键回归原先主要由 `qa/metrics.rs` 的独立测试 helper 证明，没有直接锁定 production report 使用的 `average_optional` 分母路径，也没有逐字段断言真实 report JSON 把 ineligible work/exact metrics 序列化为 `null`。

本任务只修复该确认缺口：production 聚合现在委托给可直接测试的 `average_present`，并在真实 13-case development/regression report 测试中断言 zero-evidence 的全部 work/exact ranking 字段为 JSON `null`。计算结果与 production QA 行为均未改变。

## Acceptance Evidence Matrix

| Attachment requirement | Evidence | Result |
|---|---|---|
| Zero-evidence 不进入 ranking denominator | `evaluation.rs:870-888` 的 `average_optional -> average_present`；`evaluation.rs:1468-1471` 直接测试 `[Some(0), None] -> Some(0)` | PASS |
| Per-case 不适用指标为 `Option/null` | `evaluation.rs:99-118`；`lib.rs:5243-5284` 对真实 zero-evidence case 和 JSON 字段逐项断言 | PASS |
| `rankingEligibleCaseCount`/`zeroEvidenceCaseCount` 公开 | `evaluation.rs:151-153, 946-963`；`lib.rs:5230-5234` | PASS |
| Ambiguous empty expected 被拒绝 | `evaluation.rs:326-330`；strict case contract regression | PASS |
| 唯一 deterministic Work identity | `evaluation.rs:520-526` 的 `relevance_work_id` | PASS |
| Recall/MRR/nDCG 共享 relevance view | `evaluation.rs:529-590, 699-721` 的 `RankingRelevanceView` | PASS |
| Wiki/Paper 同 work、exact-source miss | `evaluation.rs:1369-1380` | PASS |
| Returned duplicate work 不重复占位 | `evaluation.rs:1398-1418` 的 work ranking `source:a/source:b` 断言 | PASS |
| Expected duplicate representation 去重 | `evaluation.rs:1398-1418` 的 work denominator=2、exact denominator=3 断言 | PASS |
| Work/exact 指标并存 | `evaluation.rs:101-118, 156-173` | PASS |
| Legacy aliases 使用 Work 语义 | `evaluation.rs:822-829, 978-986` | PASS |
| Passage MRR diagnostic-only | `evaluation.rs:1116-1120` 的 MRR diagnostic report 标志；Markdown 与 QA contract 同样明确 | PASS |
| Zero-evidence confusion metrics | `evaluation.rs:175-181, 904-945, 988-994`；real-suite assertions `lib.rs:5236-5242` | PASS |
| Report schema v4 与 dataset identity | `evaluation.rs:18, 199-209, 891-894, 1020-1023`；hash regression | PASS |
| JSON/Markdown 同源 | `evaluation.rs:1141-1163`；renderer regression `evaluation.rs:1421-1444` | PASS |
| Work-level release mapping | `tools/qa_production_eval.py:111-114` | PASS |
| Threshold 数值不降低 | `evals/qa_release_thresholds.json:11-14`；`tests/test_qa_release_gate.py:199-214` | PASS |
| 不修改 production Retrieval | 本任务代码差异仅在 evaluation aggregation helper 与 test-only integration assertions | PASS |
| 不使用 held-out | 本任务未读取、运行或修改任何 held-out dataset/run/answer | PASS |

## Validation

- `qa::evaluation::tests`: 10 passed.
- `qa::metrics::tests`: 4 passed.
- Real Markdown repository RAG suite test: 1 passed (13-case suite, no external held-out).
- Python release-gate tests: 8 passed.
- `cargo fmt --check`: passed.
- `cargo clippy --lib -- -D warnings`: passed.

## Current Corrected Benchmark

Tracked development/regression baseline remains unchanged because the patch is behavior-preserving:

- Cases: 13/13 PASS
- Ranking eligible: 12
- Zero-evidence cases: 1
- Work Recall@10: 1.000
- Work Recall@20: 1.000
- Work MRR: 0.958333
- Work nDCG@10: 0.969244
- Zero-evidence precision: 1.000
- Zero-evidence recall: 1.000

## Scope Confirmation

- Production QA behavior modified: no.
- Production Retrieval behavior modified: no.
- Held-out used: no.
- 13-case report regenerated: no; the report contract and metric values did not change, and the existing real-repository suite was executed directly.

