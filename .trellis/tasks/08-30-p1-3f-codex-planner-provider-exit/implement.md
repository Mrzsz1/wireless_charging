# Implementation Plan — P1-3F Codex Planner Provider Exit Diagnosis

## Phase 0 — Baseline and planning

- [x] Confirm clean baseline `4c551e8e2d75425e33271248ffd4d71f16a673ae`.
- [x] Record Codex executable safe metadata, version/login state, Windows, model, and effort.
- [x] Create PRD/design/implementation artifacts.
- [x] Commit `chore(task): plan p1-3f codex planner exit diagnosis`.

## Phase 1/2 — RED fixtures and terminal-event adapter

- [x] Add J1–J7 deterministic fixtures before changing terminal-event behavior.
- [x] Run at least J1 on the old implementation and record exact RED output/root function.
- [x] Commit `test(qa): reproduce codex jsonl terminal failures`.
- [x] Implement typed JSONL observations, fixed classification, redacted failure state, and precedence in the shared Codex adapter.
- [x] Make Fatal terminate promptly; keep item error non-fatal; require non-empty final agent message for success.
- [x] Pass J1–J7 plus existing Codex adapter and Planner failure-projection tests.
- [x] Commit `fix(qa): preserve codex jsonl terminal failures`.

## Phase 3 — Repository-external diagnostic support

- [x] Add guarded `QA_CODEX_EXEC_DIAGNOSTIC_DIR` support for Development E2E/probes only.
- [x] Validate absolute outside-repository destination and default-off behavior.
- [x] Add redaction/path/cleanup tests; never commit raw artifacts.

## Phase 4 — Probe A/B/C matrix

- [x] Add the development-only probe entry point and safe report schema.
- [x] Run deterministic probe wiring tests and commit `test(qa): add isolated planner provider probes`.
- [x] Run Probe A exactly once; it failed with precise category `transport`.
- [x] Do not run Probe B because Probe A did not pass.
- [x] Do not run Probe C because Probe B was not eligible.
- [x] Delete the repository-external raw diagnostic directory.
- [x] Commit safe aggregate report with `test(qa): record exact planner provider failure`.

## Phase 5/6/7 — Unique branch, RED, minimal fix

- [x] Select only Provider external `transport` and exclude Schema/Input/Generic-exit/Integration branches.
- [x] Add a synthetic timeout-classification RED: old `unknown`, new `transport`.
- [x] Do not modify Planner Schema/input/timeout/budget/integration.
- [x] Record the precise external Provider blocker.
- [x] Do not create a Planner production-fix commit because the external branch forbids one.

## Phase 8 — Deterministic regression

- [x] Run fmt and library/binary Clippy.
- [x] Run Codex Subscription, Provider Capabilities, Retrieval Contract, QueryPlan, Production Core, Real E2E, Adaptive Routing, Claim Verification, and Heldout Runner synthetic test subsets without opening Independent Heldout data.
- [x] Run Python QA evaluator, `test:qa-evidence`, and frontend build.
- [x] Record actual QA script mapping; `test:qa-evidence` is the existing relevant script.

## Phase 9 — One final real Research

- [x] Do not run final `real-research-improvement` because failed Probe A forbids later real stages.
- [x] Record final Research as not run due `probe_a_failed` rather than fabricate aggregates.
- [x] Classify `PARTIAL-BLOCKED` from the precise external `transport` category without rerun.
- [x] Omit `test(qa): verify real query planner path` because no final Research call was eligible.

## Phase 10 — Delivery

- [x] Update QA spec with terminal-event and proven branch contracts.
- [x] Write the 25-item result report.
- [ ] Archive task, record journal, push successful delivery to GitHub without force, and verify remote SHA.

## Rollback points

- Before probes: J1–J7 and existing shared-adapter tests must be green.
- After each probe: stop if its prerequisite failed; never run downstream probes.
- Before any branch patch: require a deterministic RED.
- Before final E2E: require deterministic quality gates.
