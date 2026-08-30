# P1-3 Diagnosis Record

## First real Research diagnosis

- Case: `real-research-improvement`
- Model / effort: `gpt-5.6-luna` / `low`
- Run count: exactly one
- Report: `evals/reports/qa-real-generator-e2e-planner-diagnostic.json`
- Report schema: `qa-real-generator-e2e-report-v5`
- Process result: executed-scope failure, exit code 2

### Planner and routing aggregates

| Field | Observed value |
| --- | --- |
| plannerAttempted | true |
| plannerUsed | false |
| plannerStatus | failed_fallback |
| plannerFallback | true |
| plannerFallbackReason | provider_exit |
| plannerLatencyMs | 115457 |
| plannerStageObserved | true |
| plannerBudgetRejected | false |
| routingLlmCallBudget | 4 |
| routingLlmCallsUsed | 3 |
| routingTokenCostUsed | 7968 |
| routingTokenCostCeiling | 18000 |
| routingBudgetRejections | [] |
| queryPlanVersion | qa-retrieval-contract-v2 |
| plannedFacetCount | 1 |
| plannedRequiredFacetCount | 1 |
| plannedSearchQueryCount | 1 |
| requestedKindCount | 3 |
| mustAttemptKindCount | 3 |
| retrievalRoundCount | 1 |
| retrievalStopReason | max_rounds |

The final answer remained persisted and grounded (`semanticStatus=succeeded`, two final factual claims, two supported, zero unsupported, citation coverage 1.0). The run failed only because strict Report v5 gates correctly rejected the Planner fallback.

## Unique branch selection

Selected branch: **F — Provider transient / unavailable**, specifically `provider_exit`.

- A excluded: the stable reason was not `output_schema_rejected`.
- B excluded: the stable reason was not `contract_json_invalid` or any `contract_*_invalid` value.
- C excluded: the stable reason was neither `idle_timeout` nor `total_timeout`.
- D excluded: no Planner budget rejection occurred and the reason was not `call_budget`.
- E excluded: the reason was not `token_budget`; the 18,000 ceiling remains unchanged.
- F is the only taskbook branch containing `provider_exit`.

The local adapter readiness check succeeded (`codex-cli 0.146.0`; `Logged in using ChatGPT`). A deterministic Planner stub reproduces the exact safe failure projection (`CODEX_EXIT_ERROR` -> `provider_exit`) and confirms raw error text is discarded. The existing Windows adapter fixture also proves a non-zero subprocess exit is safely reported. These tests do not prove an adapter defect or a valid success expectation for the same request, so there is no RED test that justifies a production change. Per the user and Branch F gate, no Provider Adapter, Planner Contract, timeout, input, or budget production behavior is changed.

## Deterministic protection

- `query_planner_provider_exit_is_auditable_and_redacted`: reproduces the observed failure category without a real Provider and proves fallback telemetry is exact and redacted.
- `exploratory_stub_planner_produces_a_usable_contract_without_fallback`: proves a valid stub RetrievalContract is accepted in Exploratory mode with `plannerStatus=succeeded`, no fallback, at least one facet, and at least one planned query.

