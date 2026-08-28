# Design — Generator Budget Ledger Reliability

## Root Cause

Current admission is based on historical maximum reservations:

```text
historicalReservedTotal + newReservation <= ceiling
```

`settle` reduces the provisional value stored in `tokenCostUsed`, but historical `tokenCostReserved` never decreases. A completed 4,000-token reservation that actually used 1,000 therefore still consumes 4,000 in every later admission decision. This is the confirmed mechanism behind false `generator:token_budget` rejection.

## Ledger Model

`LlmBudgetUsage` will expose four independent counters:

- `token_cost_used`: actual cost committed by completed reservations.
- `token_cost_in_flight`: maximum token cost of currently active reservations.
- `token_cost_reserved`: compatibility field; historical cumulative maximum reserved.
- `token_cost_reserved_total`: explicit historical cumulative maximum reserved; equal to the compatibility field.

Admission uses only:

```text
token_cost_used + token_cost_in_flight + new_reservation <= policy.token_cost_ceiling
```

Historical reservation totals remain audit-only.

## Reservation Ownership

`reserve(stage, ceiling)` returns an owned `LlmReservation` instead of `()`.

```rust
pub struct LlmReservation {
    id: u64,
    stage: String,
    guard: LlmBudgetGuard,
    closed: bool,
}
```

`LlmBudgetState` owns `next_reservation_id` and a map of active ID → `{stage, reserved}`. The handle is not `Clone`.

- `settle(self, actual)` removes exactly its ID, subtracts its reservation from in-flight, and adds actual cost to used.
- `release(self)` removes exactly its ID and subtracts in-flight without adding used.
- `Drop` performs best-effort release when a handle leaves scope unclosed, covering early `?`, provider setup errors, task panic and cancellation unwind.
- Because settle consumes the non-clone handle, safe Rust cannot settle one reservation twice. `closed` prevents Drop from releasing after successful settlement.

## Call and Token Semantics

- `calls_used` increments once when reserve succeeds and never decreases.
- Token reservation release does not refund a call.
- A provider result, including a provider error after invocation, settles with the measurable prompt/response estimate.
- An error before provider invocation drops/releases the handle, leaving actual used unchanged.
- Actual cost is recorded as observed rather than silently capped to reserved; any overage is visible and prevents later admission.

## Reconfigure

`reconfigure` only replaces `RoutingPolicy`. It does not mutate usage counters, active reservation map, next ID, stages or rejections. Existing in-flight reservations therefore remain charged under the new ceiling until they close.

## Telemetry and Compatibility

- Keep `routingTokenCostReserved` as historical cumulative reserved.
- Add `routingTokenCostInFlight` and `routingTokenCostReservedTotal` to `RetrievalQuery`, `QaRunManifest`, TypeScript DTOs and the evidence-side diagnostics.
- Bump `RUN_MANIFEST_SCHEMA_VERSION` from `qa-run-v20` to `qa-run-v21` because the manifest contract gains fields.
- Old manifests deserialize with zero defaults for new fields.

## Production Wiring

Update only the existing budget boundaries:

- understanding
- query planner
- Codex/API generator
- semantic verifier
- production heldout executor for compile compatibility only; do not run or inspect heldout data

No prompt, retrieval, answer or verification decision logic changes.

## Rollback

- Budget core/tests are one commit boundary.
- Production/manifest/frontend wiring is a second boundary if practical.
- Reverting the feature restores v20 fields and the old API; no stored database migration is involved.

