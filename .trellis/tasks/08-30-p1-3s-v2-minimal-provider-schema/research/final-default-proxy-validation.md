# Final Validation — Shell Proxy Cleared

The verification process explicitly removed `WIRELESS_CODEX_PROXY_URL`, uppercase/lowercase HTTP, HTTPS, ALL proxy variables, and NO_PROXY variables. The rebuilt application code therefore selected its default child proxy `http://127.0.0.1:7890`.

Each stage was run exactly once and used a unique non-overwriting safe report.

## Probes

| Stage | Status | Category | Contract | Exit | Latency | Baseline |
| --- | --- | --- | --- | --- | --- | --- |
| Probe A | succeeded | empty | valid | 0 | 6,571 ms | 0 |
| Probe B | succeeded | empty | valid | 0 | 17,757 ms | 1 |
| Probe C | succeeded | empty | valid | 0 | 19,243 ms | 16 |

Safe reports:

- `evals/reports/qa-codex-planner-probe-a-p1-3s-v2-default-proxy.json`
- `evals/reports/qa-codex-planner-probe-b-p1-3s-v2-default-proxy.json`
- `evals/reports/qa-codex-planner-probe-c-p1-3s-v2-default-proxy.json`

## Real Research

- Selected case: only `real-research-improvement`.
- Run count: exactly one.
- Planner transport/schema path passed:
  - `plannerAttempted=true`
  - `plannerUsed=true`
  - `plannerStatus=succeeded`
  - `plannerFallback=false`
  - `plannerFallbackReason=""`
  - `plannedFacetCount=5`
  - `plannedSearchQueryCount=10`
- Semantic verifier completed: `semanticStatus=succeeded`.
- Final gate failed outside this task's permitted production layers:
  - `citationValid=false`
  - `finalVisibleProjectionValid=false`
  - final factual / supported / unsupported = 2 / 1 / 1
  - `persisted=false`
  - `executedScopePassed=false`
  - error `citation_validation_failed`
  - process exit code 2
- Safe report: `evals/reports/qa-real-generator-e2e-p1-3s-v2-default-proxy.json`.

Per the taskbook, the real run was not repeated to seek an accidental pass. No Prompt, budget, Semantic, Generator, Grounding, persistence, or answer-generation behavior was changed. The result proves the default proxy and Planner schema path work after Shell proxy removal, while the task's all-green close gate remains unmet because of a separate final-grounding/citation outcome.
