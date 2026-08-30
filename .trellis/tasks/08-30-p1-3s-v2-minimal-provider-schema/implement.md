# Implementation Plan — P1-3S v2 Minimal Provider Schema Compatibility

## Phase 0 — Plan and RED

- [x] Record clean baseline `02df15c0f694f416f4af44a5daf4724114086b8a` and commit planning artifacts.
- [x] Add a deterministic pre-fix test proving the current Provider path receives a schema containing `uniqueItems` at all three kind-array positions.
- [x] Run the RED observation before any production change and commit it separately.

## Phase 1 — Minimal schema split

- [x] Add the recursive one-key `uniqueItems` removal helper and `retrieval_contract_provider_schema()`.
- [x] Export `query_plan_provider_schema()` while retaining `query_plan_schema()` as the full contract.
- [x] Wire production Planner, `production_heldout` consistency path, and Probe B/C to the Provider schema only.
- [x] Add S1–S3 and the evidence-driven compatibility comment; run focused schema tests and commit.

## Phase 1b — Strict local validation

- [x] Add/confirm S4 duplicate kind normalization.
- [x] Add/confirm S5 duplicate facet ID rejection.
- [x] Add/confirm S6 invalid budget rejection.
- [x] Add/confirm S7 invalid facet ID rejection.
- [x] Add/confirm S8 unknown-field rejection.
- [x] Run focused RetrievalContract/QueryPlan tests and commit.

## Phase 2 — Temporary-proxy Probe B

- [x] Build the current `qa-planner-probe` binary.
- [x] Explicitly set uppercase and lowercase HTTP/HTTPS/ALL proxy variables to `http://127.0.0.1:7890` for the child command only.
- [x] Run Probe B exactly once to a new safe report; do not run A.
- [x] Probe B passed; commit the safe aggregate report/evidence without broadening the compatibility transform.

## Phase 3 — Temporary-proxy Probe C

- [x] Only after B passes, run Probe C exactly once with the same executable/model/effort/proxy.
- [x] Validate success, contract validity, and positive baseline count; commit safe evidence.
- [x] Probe C passed, so the actual-failure stop branch was not entered.

## Phase 4 — Temporary-proxy real Research

- [x] Only after C passes, build/use `qa-real-e2e` and select only `QA_REAL_E2E_CASE_ID=real-research-improvement`.
- [x] Run once with temporary proxy and a unique report.
- [x] Verify Planner, Semantic, final factual/support, citation, persistence, executed-scope, and exit-code gates; commit safe evidence.
- [x] Do not run Independent Heldout.

## Phase 5 — Conditional default proxy integration

- [x] Only after Phase 4 passes, add pure proxy resolution and child `Command::env(...)` injection in `codex_subscription.rs`.
- [x] Cover explicit URL, `off/direct/none`, inherited standard proxy, and default localhost:7890 behavior without parent `set_var`.
- [x] Preserve existing structured lifecycle diagnostics and ensure no URL/credential is logged.
- [x] Run focused Codex adapter tests, fmt, and Clippy; commit.

## Phase 6 — Final validation without Shell proxy

- [x] Clear uppercase/lowercase HTTP/HTTPS/ALL proxy variables in the verification process.
- [x] Run Probe A, B, and C exactly once each, in order, to unique safe reports; all passed.
- [x] Run only `real-research-improvement` exactly once after all probes passed.
- [x] Validate every taskbook field and record the final `citation_validation_failed` gate without a live rerun or out-of-scope production change.

## Phase 7 — Quality and delivery

- [x] Run focused Rust tests, `cargo fmt --check`, relevant Clippy, and `git diff --check`; avoid full/heldout regression.
- [x] Update `.trellis/spec/backend/qa-contract.md` with the domain/provider schema boundary and proven proxy contract.
- [ ] Write the final 16-item result report.
- [ ] Commit verified work, archive the Trellis task, record journal, and normally push `master` to `origin` without force.

## Rollback and stop points

- Before each live stage, verify the previous report passed; never bypass B/C/Research gates.
- Never overwrite a prior probe/report path.
- Never broaden the compatibility transform without a real `schema_rejected` observation.
- Never integrate the default proxy before temporary-proxy Research passes.
- If push fails, retain all local commits and retry only a normal push.
