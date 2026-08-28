# Implementation Plan

1. Prove the root cause
   - Add a failing regression for ceiling 8,000: reserve 4,000, settle 1,000, then reserve generator 6,000.
   - Record the old failure as historical-reserved admission drift.

2. Replace the ledger core
   - Add unique active reservation IDs/map and in-flight telemetry.
   - Make `reserve` return non-clone `LlmReservation`.
   - Implement consuming `settle`, explicit `release`, and Drop release fallback.
   - Change admission to used + in-flight + new; preserve cumulative calls/history.

3. Cover ledger invariants
   - True token overage rejection.
   - Concurrent reservation anti-oversell.
   - Calls remain cumulative after settle/release.
   - Reconfigure preserves used/in-flight/calls/history.
   - Drop/provider failure releases in-flight.
   - Settle closes once and Drop has no second effect.
   - Development synthetic understanding→planner→generator→verifier flow and true-overage control.

4. Wire production boundaries
   - Move reservation handles through understanding/planner/generator/semantic-verifier call sites.
   - Ensure every provider result settles and every pre-call/abnormal exit releases through ownership.
   - Update production heldout source only for API compatibility; do not run heldout.

5. Add auditable telemetry
   - Add in-flight/reserved-total fields to RetrievalQuery and usage projection.
   - Bump QaRunManifest v20→v21 and map new fields.
   - Update TypeScript run-manifest/retrieval types and QA diagnostic display.
   - Keep existing reserved field semantics unchanged.

6. Verify without over-testing
   - `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml -- --check`
   - Focused adaptive-routing, provider-failure, manifest and production-fixture Rust tests.
   - `cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml --lib -- -D warnings`
   - Frontend focused tests plus `npm run build`/type-check.
   - Do not run Independent Held-out; do not modify ceilings.

7. Finish
   - Update `.trellis/spec/backend/qa-contract.md` with the ledger formula, handle lifecycle, telemetry and test matrix.
   - Commit each completed phase locally.
   - After all checks pass, push `master` to the confirmed GitHub origin as explicitly requested.

## Risk and Rollback Points

- Highest risk: a call site drops a handle before provider invocation or forgets to settle after a real call. The RAII default must fail safe and tests must cover both cases.
- Manifest field additions require Rust/TypeScript synchronization and v21 versioning.
- No policy-number, prompt, retrieval, state, evidence or verifier-judgment changes are permitted.

## Completion Record

- Root-cause regression failed before the fix and passes after the ledger rewrite.
- Budget core now uses unique RAII reservation handles and used + in-flight admission.
- Production understanding/planner/generator/verifier boundaries use owned handles.
- Manifest v21 and frontend diagnostics expose used/in-flight/ceiling/history.
- Rust QA: 208 passed, 2 ignored; frontend tests/build, fmt and clippy passed.
- No held-out data was used and 8k/18k/32k ceilings were unchanged.
