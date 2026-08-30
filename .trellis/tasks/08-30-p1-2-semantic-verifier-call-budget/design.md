# Design — P1-2 Semantic Verifier Call Budget

## Boundary

The request-scoped guard remains the single admission authority:

```text
Understanding / Planner / Generator
  -> non-Semantic admission must leave remaining Semantic reserve
Semantic Verifier
  -> may consume only the dedicated reserve
All stages
  -> still obey total calls and token ceiling
```

No caller may manually emulate the reserve.

## Policy contract

`RoutingPolicy` gains `semantic_verifier_call_reserve: usize`. Version v2 policies are:

| Mode | Calls | Semantic reserve | Token ceiling |
|---|---:|---:|---:|
| Direct | 3 | 1 | 8,000 |
| Research | 4 | 1 | 18,000 |
| Exploratory | 5 | 1 | 32,000 |

`qa-run-v22` persists the existing policy version and runtime budget fields; no new manifest field is required.

## Guard algorithm

`LlmBudgetState` owns `semantic_verifier_calls_used`. `reserve()` performs all checks before mutating state:

1. Classify with shared `SEMANTIC_VERIFIER_STAGE`.
2. Reject Semantic when its reserve is exhausted.
3. Compute `next_calls` and the reserve remaining after the candidate reservation.
4. Reject when `next_calls + remaining_reserve` exceeds the policy total.
5. Apply the unchanged token equation.
6. Only after every check succeeds, allocate the reservation, increment calls-used, and increment semantic-used for a Semantic reservation.

Closing a reservation changes only token in-flight/actual accounting and stage telemetry. It does not reverse call counters.

## Reconfiguration

`reconfigure()` changes only the policy, preserving total calls, token accounting, active reservations, stage/rejection telemetry, and the Semantic-used count. Upgrading Direct to Research therefore grants only capacity justified by the new finite policy.

## Test seams

- Phase 1 places B1–B7 beside the budget owner and records the baseline red result before implementation.
- Phase 3 uses the existing fixture completion seam in claim verification or the smallest test-only equivalent, while exercising real claim eligibility, budget reservation, parsing, and merge behavior.
- Phase 4 uses synthetic production/persistence infrastructure and the real guard; it must not invoke a real Provider.
- Phase 5 reserves the exact contextual Research stage sequence with stubs only.

## Telemetry

Existing `LlmBudgetUsage` remains the aggregate contract. Accepted stages append the canonical stage name; close appends `:settled` or `:released`; rejected attempts append `<stage>:call_budget|token_budget`. Existing QA trace events expose Semantic completion/failure at orchestration boundaries. No content logging or manifest schema expansion is introduced.

## Compatibility and rollback

- `qa-run-v22` and E2E report v4 remain unchanged.
- Old serialized policy objects are not persisted as user data; current constructors provide the new field.
- Each taskbook commit is independently reversible. No token or verification behavior is coupled to the call-reserve change.
