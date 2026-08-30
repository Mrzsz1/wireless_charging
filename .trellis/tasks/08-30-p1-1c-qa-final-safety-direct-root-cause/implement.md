# Implementation Plan — P1-1C

## Phase 0 — Baseline and planning

- [x] Record clean pre-task baseline `abd5e02`, branch, Rust, Node/npm, and Python versions.
- [x] Create this child Trellis task under `08-29-p1-1-real-answer-grounding-generator`.
- [x] Commit planning artifacts and parent child-link metadata.

## Phase 1 — Heldout runner Final claims

- [x] Inspect `heldout_runner.rs`, bundle schema/tests, and `qa_accuracy_eval.py` without opening formal heldout questions or run answers.
- [x] Make v22 bundle creation require Final Supported claims and canonical visible projection.
- [x] Add Draft 5 / Final 3, missing final audit, unsupported claim, unknown evidence, duplicate/empty/projection-failure fixtures.
- [x] Add independent Python final-audit/evidence validation and focused tests.
- [x] Run focused Rust/Python tests and commit `fix(eval): export final grounded claims to heldout bundles`.

## Phase 2 — Trusted history

- [x] Replace whole-answer trust projection with ordered Final Supported claim projection.
- [x] Exclude suggestions, notices, supplements, appendix, and non-supported statuses.
- [x] Add five trusted-context/multi-turn pollution regressions and logging assertions.
- [x] Commit `fix(qa): build trusted history from final supported claims`.

## Phase 3 — Final-only UI delivery

- [x] Stop production adapter, zero-evidence path, and offline path from emitting raw Token content.
- [x] Keep progress phases and deliver final content only through `Completed` after persistence.
- [x] Remove the frontend Token payload contract and all draft copy/display state.
- [x] Add success/failure/cancel backend event tests, frontend state tests, and log-sequence assertions.
- [x] Commit `fix(qa): expose only finalized answers to the ui`.

## Phase 4 — E2E scope semantics

- [x] Add scope/executed/full-suite/release fields and exact exit semantics.
- [x] Test single pass/fail and full-suite pass/fail without real Provider calls.
- [x] Commit `fix(eval): separate executed scope pass from full-suite eligibility`.

## Phase 5 — Final provenance and visible integrity

- [x] Replace text-only map with canonical key + FIFO queue and additive source mapping.
- [x] Add post-render body hashes/validity and fail-closed no-new-fact validation using shared canonical projection.
- [x] Add F1–F6 regressions plus stage logging assertions.
- [x] Commit `fix(qa): harden final claim provenance and visible projection`.

## Phase 6 — One Direct diagnosis

- [ ] Configure `E:\qa-direct-diagnostic-p1-1c` and `real-direct-rose`.
- [ ] Run real E2E exactly once.
- [ ] Inspect local diagnostic only; record the four booleans and delete the local artifacts.

## Phase 7 — One proven root-cause fix

- [ ] Select exactly one A/B/C branch.
- [ ] Add a public deterministic failing regression, apply the minimal single-layer fix, and run focused tests.
- [ ] Commit the taskbook-defined branch-specific fix message.

## Phase 8 — One Direct verification rerun

- [ ] Run `real-direct-rose` exactly once.
- [ ] Record evidence/Draft/Final/citation/semantic/persistence/scope/exit aggregates.
- [ ] Stop as `PARTIAL-BLOCKED` if the independent semantic call-budget blocker appears; do not alter budgets.
- [ ] Commit `test(qa): verify p1-1c direct production path`.

## Phase 9 — Full deterministic quality and delivery

- [ ] Run `cargo fmt --check`.
- [ ] Run `cargo clippy --lib --bins -- -D warnings` and `cargo clippy --all-targets -- -D warnings`.
- [ ] Run focused heldout/final-grounding/trusted-context/real-E2E/claim-verification Rust tests.
- [ ] Run `python -m unittest tests.test_qa_accuracy_eval`.
- [ ] Run frontend QA tests and `npm run build`.
- [ ] Update `.trellis/spec/backend/qa-contract.md` and related specs.
- [ ] Write `result.md`, archive task, record journal, and normally push all commits to GitHub `origin/master`.

## Guardrails

- Do not run or inspect Independent Heldout formal data.
- Do not reset, clean, stash, amend, force push, or include unrelated files.
- Do not change budgets, planner, frozen thresholds/data, semantic decision policy, reranker/embedding, zero-evidence policy, or retries.
- Do not proceed to the next phase until the current focused quality gate is green.
