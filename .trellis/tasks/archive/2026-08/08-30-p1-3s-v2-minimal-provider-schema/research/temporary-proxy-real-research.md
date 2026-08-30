# Temporary-Proxy Real Research

- Eligibility: Probe B and Probe C both passed.
- Selected case: only `real-research-improvement` via `QA_REAL_E2E_CASE_ID`.
- Run count: exactly one.
- Proxy: explicit uppercase/lowercase HTTP, HTTPS, and ALL proxy variables set to `http://127.0.0.1:7890`.
- Model / effort: `gpt-5.6-luna` / `low`.
- Safe report: `evals/reports/qa-real-generator-e2e-p1-3s-v2-proxy7890.json`.
- Process exit code: 0; report `passed=true`; `executedScopePassed=true`; one of one cases passed.

## Required gates

| Field | Result |
| --- | --- |
| plannerAttempted | true |
| plannerUsed | true |
| plannerStatus | succeeded |
| plannerFallback | false |
| plannerFallbackReason | empty |
| plannedFacetCount | 7 |
| plannedSearchQueryCount | 14 |
| semanticStatus | succeeded |
| final factual / supported / unsupported | 3 / 3 / 0 |
| finalCitationCoverage | 1.0 |
| persisted | true |
| executedScopePassed | true |
| exitCode | 0 |

- Planner latency: 23,855 ms.
- Reranker: `cross-encoder-research-v1`, no fallback.
- Routing calls: 3 of 4; token cost: 9,694 of 18,000; no budget rejection.
- No other E2E case and no Independent Heldout was run.
- This PASS unlocks the taskbook's conditional default proxy integration phase.
