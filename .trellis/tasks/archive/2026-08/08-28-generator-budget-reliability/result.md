# Generator Budget Reliability Result

## Root Cause Proof

Before the fix, the new regression `completed_stage_unused_reservation_is_reusable_by_generator` failed with:

```text
ceiling=8000
understanding reserve=4000, settle actual=1000
generator reserve=6000
old result: LLM_BUDGET_EXCEEDED: generator:token_budget
```

The old guard admitted against historical `token_cost_reserved` (4,000 + 6,000) even though only 1,000 was actually used and no reservation remained in flight. This deterministically proves the historical maximum reservation was the false-rejection cause.

## New Ledger Contract

Admission:

```text
tokenCostUsed + tokenCostInFlight + newReservation <= tokenCostCeiling
```

- `tokenCostUsed`: actual estimated cost committed by completed reservations.
- `tokenCostInFlight`: maximum cost held by active reservation handles.
- `tokenCostReserved`: compatibility field retaining historical cumulative maximum reservations.
- `tokenCostReservedTotal`: explicit historical cumulative maximum reservations; equal to the compatibility field.
- `callsUsed`: cumulative successful reservations; never reduced by settle/release.

## Reservation Lifecycle

- `reserve` allocates a unique ID and returns a non-clone, must-use `LlmReservation`.
- `settle(self, actual)` removes that exact ID, releases in-flight, records actual used, and consumes ownership.
- `release(self)` removes that exact ID without adding used.
- Unclosed handles release through `Drop`, covering early return, `?`, provider setup failure and unwind/panic.
- A second internal close returns `LLM_BUDGET_STATE_ERROR: reservation_closed` and leaves usage unchanged.

## Compatibility and Telemetry

- `QaRunManifest`: `qa-run-v20` → `qa-run-v21`.
- Added `routingTokenCostInFlight` and `routingTokenCostReservedTotal` to Rust/TypeScript contracts.
- Existing `routingTokenCostReserved` remains historical cumulative reserved.
- Generator failures project current usage/rejections before persisting the failed audit.
- QA sidebar displays calls, used, in-flight, ceiling, historical reserved and rejections separately.
- Direct/Research/Exploratory ceilings remain exactly 8,000/18,000/32,000.

## Tests

New/expanded deterministic tests cover:

1. completed unused reservation reuse by generator;
2. true used + in-flight overage rejection;
3. concurrent reservation anti-oversell and restored capacity after release;
4. calls remain cumulative after settle/release/drop;
5. Direct→Research reconfigure preserves usage/in-flight/calls/history;
6. provider error/early return releases through Drop;
7. duplicate internal close has no side effects;
8. Exploratory development stress reaches understanding→planner→generator→semantic verifier;
9. true policy ceilings remain 8k/18k/32k;
10. v21 manifest round-trip preserves budget telemetry.

## Validation

- Focused adaptive-routing tests: 11 passed.
- QA Rust suite: 208 passed, 2 ignored (only model-download/local-model tests).
- Clippy (`--lib -- -D warnings`): passed.
- Rust fmt/check: passed.
- Frontend QA evidence tests: 5 passed.
- TypeScript/Vite production build: passed.
- Independent Held-out: not read, not run, not modified.

## Scope Confirmation

- Retrieval ranking/Reranker/Embedding: unchanged.
- Query Planner research logic: unchanged; only reservation ownership wiring changed.
- Research State/parameter reducer: unchanged.
- Prompt/answer content/evidence/output limits: unchanged.
- Semantic Verifier judgment rules: unchanged; only reservation ownership wiring changed.
- Policy ceilings: unchanged.
- Performance optimization: not performed.

