# Direct Verification Rerun

## Run control

- Case: public regression `real-direct-rose`.
- Phase 8 real Provider executions: exactly 1.
- Phase 6 + Phase 8 total real Provider executions: exactly 2.
- No raw diagnostic export was enabled, and no answer text is retained here.

## Aggregates

| Metric | Result |
|---|---:|
| Provider / model | `codex-subscription` / `gpt-5.6-luna` |
| Evidence count | 2 |
| Generator fallback / budget rejection | 0 / 0 |
| Draft claim count | 1 |
| Draft cited Evidence ID count | 1 |
| Draft supported / not-verifiable | 0 / 1 |
| Draft alignment score | 0.05 |
| Final factual / supported | 0 / 0 |
| Final citation coverage | 0.0 |
| Final visible projection valid | true |
| Semantic status | `unavailable` |
| Semantic fallback reason | `llm_budget_exceeded` |
| Persisted | false |
| Scope / executedScopePassed | `single_case` / false |
| releaseEligible | false |
| Process exit | 2 |

## Verdict

The Branch B fix worked at its intended boundary: the previously uncited Direct Draft claim now has one current-run Evidence ID, so Generator-to-parser evidence binding is present. The deterministic lexical verifier rated the claim not verifiable, and the subsequent semantic-verifier call was rejected by the independent request call budget. Per the taskbook, P1-1C stops as **PARTIAL-BLOCKED** and does not change budgets, verification thresholds, planner behavior, retrieval, or semantic decision policy. The remaining blocker belongs to P1-2.
