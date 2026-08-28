# Design — Retrieval Evaluation Semantics v4

## Scope boundary

All changes remain inside evaluation/reporting and release metric mapping. Retrieval execution is treated as an opaque producer of a ranked `RankedEvidence` list. No production ranking, query, prompt, model, budget or state code is changed.

## Relevance views

One deterministic helper `relevance_work_id(document_id)` owns work identity:

- `wiki:sources/<source>` → `source:<source>`
- `paper:sources/<source>` → `source:<source>`
- every other ID → unchanged

A `RankingRelevanceView` is built once per unit from both expected and returned IDs. It owns a deduplicated ordered ranking and deduplicated expected set. Recall, reciprocal rank and binary nDCG consume this same view, so their relevance unit cannot drift.

Two views are produced:

1. work view using `relevance_work_id` — production gate;
2. exact-source view using identity — diagnostic representation fidelity.

## Eligibility and nullability

`rankingMetricsEligible = !expectedDocuments.is_empty()`. Dataset validation guarantees an empty expected set means `zeroEvidenceExpected=true`. Per-case ranking metrics are `Option<f64>` and serialize as `null` when ineligible. Aggregate ranking metrics divide only by `rankingEligibleCaseCount`.

Passage MRR remains diagnostic-only and follows the same eligibility rule. Legacy ranking fields remain Option-valued aliases of the work metrics in report v4.

## Zero-evidence classification

Truth is `zeroEvidenceExpected`; prediction is `zeroEvidenceObserved`.

- TP: expected and observed zero evidence
- FP: positive expected but observed zero evidence
- FN: zero expected but evidence observed
- TN: positive expected and evidence observed

The aggregate exposes counts plus precision `TP/(TP+FP)`, recall `TP/(TP+FN)` and specificity `TN/(TN+FP)`. Undefined ratios serialize as null.

## Report identity

`EvaluationCase` and conversation DTOs gain deterministic serialization. `caseDatasetSha256` hashes the serialized normalized cases array. Report v4 includes this hash plus suite name/count, index snapshot, retriever version and schema version.

## Markdown and release mapping

The Markdown renderer consumes the in-memory v4 report only. Snapshot tests compare displayed values to serialized report fields. Release gates point to `workRecallAt10/20`, `workMrr`, and `workNdcgAt10`; numeric thresholds are unchanged.

## Compatibility

For one v4 transition:

- `documentRecallAt5/10/20 = workRecallAt5/10/20`
- `documentMrr = workMrr`
- `mrr = workMrr`
- `ndcgAt10 = workNdcgAt10`

The schema and README label these as legacy aliases. Consumers should migrate to explicit work fields.
