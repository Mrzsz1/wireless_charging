# Implementation Plan — P1-2 Semantic Verifier Call Budget

## Phase 0 — Baseline and planning

- [x] Confirm clean `master` baseline `71836e89de8183e24917d8fdef4022dcc2f1dfa8`.
- [x] Record Rust 1.96.1, Node 24.11.0, npm 11.15.0, and Python 3.13.0.
- [x] Create this Trellis task and write PRD/design/implementation artifacts.
- [x] Commit `chore(task): plan p1-2 semantic verifier call budget` after validating these artifacts.

## Phase 1 — Deterministic red tests first

- [x] Add B1 Direct and B2 worst-case Research legal-chain tests without changing production policy/guard logic.
- [x] Add B3 reserve protection, B4 one-shot Semantic, B5 token ceiling, B6 reconfigure, and B7 failure/non-refund tests.
- [x] Run `cargo test --lib adaptive_routing` against baseline logic and record the expected failures.
- [x] Only after the red result is recorded, proceed to production implementation.

## Phase 2 — Reserve implementation

- [x] Add `adaptive-routing-v2`, budgets 3/4/5, reserve field=1, shared stage constant, and guard reserve algorithm.
- [x] Use the shared stage constant from claim verification and Semantic telemetry without changing Semantic prompt/decision behavior.
- [x] Run 18 adaptive-routing tests, fmt check, and Clippy lib gate successfully.
- [x] Commit `fix(qa): reserve semantic verifier call capacity` after the green gate.

## Phase 3 — Semantic integration

- [ ] Add S1 succeeded-after-prior-calls fixture test.
- [ ] Add S2 genuine token-budget unavailable test.
- [ ] Add S3 no-eligible-claim/no-consumption test.
- [ ] Add S4 unknown-remains-NotVerifiable test.
- [ ] Commit `test(qa): cover reserved semantic verification calls`.

## Phase 4 — Production Core synthetic regression

- [ ] Exercise Direct structured binding through Semantic entailed, Final Supported=1, and persistence with the real guard.
- [ ] Assert policy version/budget/usage/rejections/Semantic/Final telemetry.
- [ ] Commit deterministic production-path coverage with the taskbook test commit phase.

## Phase 5 — Contextual Research protection

- [ ] Use stubs only to force Understanding + Planner + Generator + Semantic.
- [ ] Assert Research budget=4 and all four calls are admitted.
- [ ] Confirm no business logic outside the budget contract changed.

## Phase 6 — One real Direct E2E

- [ ] After all deterministic gates pass, run only `real-direct-rose` exactly once.
- [ ] Record budget, Semantic, Draft/Final, citation, persistence, scope, and exit aggregates.
- [ ] Classify PASS/PARTIAL-BLOCKED/FAIL exactly as specified; do not rerun or widen budgets.
- [ ] Commit `test(qa): verify direct semantic production path`.

## Phase 7 — Full deterministic quality and delivery

- [ ] Run required focused Rust, Python evaluator, frontend QA, build, fmt, and Clippy commands.
- [ ] Run all-targets Clippy and record only the documented pre-existing warning if unchanged.
- [ ] Update QA code-specs with the v2 reserve contract.
- [ ] Write `result.md`, archive the task, record journal, push `master` normally, and verify remote SHA.

## Guardrails

- No token-ceiling, Semantic decision/prompt, grounding threshold, Retrieval, Planner logic, Generator, reranker, embedding, zero-evidence, retry, frozen-data/threshold, schema, or performance changes.
- No formal Independent Heldout execution or content inspection.
- No reset, clean, stash, amend, history rewrite, or force push.
- No real Provider call before Phase 6 and no second real Direct rerun.
