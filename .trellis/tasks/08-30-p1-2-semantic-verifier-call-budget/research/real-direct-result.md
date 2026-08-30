# Phase 6 Real Direct Result

## Run control

- Public case: `real-direct-rose`.
- Real Provider executions in P1-2: exactly 1.
- Provider/model: `codex-subscription` / `gpt-5.6-luna`.
- No raw grounding diagnostic export was enabled and no answer text is retained in this task record.
- Process exit: 0.

## Aggregates

| Metric | Result |
|---|---:|
| Evidence count / selected | 2 / 2 |
| Generator fallback / budget rejection | 0 / 0 |
| Routing LLM call budget / used | 3 / 3 |
| Routing budget rejection count | 0 |
| Token used / ceiling | 3,767 / 8,000 |
| Semantic succeeded / unavailable | 1 / 0 |
| Semantic status / fallback reason | `succeeded` / empty |
| Draft claims / supported / evidence IDs | 1 / 1 / 1 |
| Final factual / supported / unsupported | 1 / 1 / 0 |
| Final cited / unknown | 1 / 0 |
| Final citation coverage | 1.0 |
| Visible projection valid | true |
| Persisted | true |
| executedScopePassed | true |
| fullSuiteEvaluated / releaseEligible | false / false |

## Verdict

**PASS.** The Semantic Verifier no longer encounters `call_budget`; it executes exactly once in the reserved slot, returns a successful verdict, and the final supported Direct answer passes citation, visible-projection, and persistence gates. Single-case success does not claim full-suite release eligibility.
