# Implementation Plan — P1-3F Codex Planner Provider Exit Diagnosis

## Phase 0 — Baseline and planning

- [x] Confirm clean baseline `4c551e8e2d75425e33271248ffd4d71f16a673ae`.
- [x] Record Codex executable safe metadata, version/login state, Windows, model, and effort.
- [x] Create PRD/design/implementation artifacts.
- [ ] Commit `chore(task): plan p1-3f codex planner exit diagnosis`.

## Phase 1/2 — RED fixtures and terminal-event adapter

- [ ] Add J1–J7 deterministic fixtures before changing terminal-event behavior.
- [ ] Run at least J1 on the old implementation and record exact RED output/root function.
- [ ] Commit `test(qa): reproduce codex jsonl terminal failures`.
- [ ] Implement typed JSONL observations, fixed classification, redacted failure state, and precedence in `codex_subscription.rs` only.
- [ ] Make Fatal terminate promptly; keep item error non-fatal; require non-empty final agent message for success.
- [ ] Pass J1–J7 plus existing Codex/Generator/Direct/Semantic tests.
- [ ] Commit `fix(qa): preserve codex jsonl terminal failures`.

## Phase 3 — Repository-external diagnostic support

- [ ] Add guarded `QA_CODEX_EXEC_DIAGNOSTIC_DIR` support for Development E2E/probes only.
- [ ] Validate absolute outside-repository destination and default-off behavior.
- [ ] Add redaction/path/cleanup tests; never commit raw artifacts.

## Phase 4 — Probe A/B/C matrix

- [ ] Add the development-only probe entry point and safe report schema.
- [ ] Run deterministic probe wiring tests and commit `test(qa): add isolated planner provider probes`.
- [ ] Run Probe A exactly once.
- [ ] Run Probe B exactly once only if A passes.
- [ ] Run Probe C exactly once only if B passes.
- [ ] Delete the repository-external raw diagnostic directory.
- [ ] Commit safe aggregate report with `test(qa): record exact planner provider failure`.

## Phase 5/6/7 — Unique branch, RED, minimal fix

- [ ] Select exactly one branch from safe probe results and exclude all others in writing.
- [ ] Add the smallest branch-specific old-code RED and record error/root function.
- [ ] Do not modify Planner Schema/input/timeout/budget/integration without RED.
- [ ] Apply a minimal single-layer patch or record a precise external blocker.
- [ ] Commit only the selected branch's exact fix when a production patch is justified.

## Phase 8 — Deterministic regression

- [ ] Run fmt and library/binary Clippy.
- [ ] Run Codex Subscription, Provider Capabilities, Retrieval Contract, QueryPlan, Production Core, Real E2E, Adaptive Routing, Claim Verification, and Heldout Runner test subsets without opening heldout data.
- [ ] Run Python QA evaluator, `test:qa-evidence`, and frontend build.
- [ ] Record actual QA script mapping and pre-existing issues only.

## Phase 9 — One final real Research

- [ ] Run `real-research-improvement` exactly once after deterministic gates.
- [ ] Record Planner, budget, QueryPlan, Semantic, Final Grounding, persistence, scope, and exit aggregates.
- [ ] Classify PASS/PARTIAL-BLOCKED/FAIL without rerun.
- [ ] Commit `test(qa): verify real query planner path`.

## Phase 10 — Delivery

- [ ] Update QA spec with terminal-event and proven branch contracts.
- [ ] Write the 25-item result report.
- [ ] Archive task, record journal, push successful delivery to GitHub without force, and verify remote SHA.

## Rollback points

- Before probes: J1–J7 and existing shared-adapter tests must be green.
- After each probe: stop if its prerequisite failed; never run downstream probes.
- Before any branch patch: require a deterministic RED.
- Before final E2E: require deterministic quality gates.
