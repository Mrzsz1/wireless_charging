# Implementation Plan — P1-3 Query Planner Provider Reliability

## Phase 0 — Baseline and planning

- [x] Confirm clean baseline `4e8399589ca20a33980309fb057d4f125a1262bd` on `master`.
- [x] Record Rust/Node/npm/Python/Codex CLI and E2E model/effort.
- [x] Create PRD/design/implementation artifacts.
- [x] Commit `chore(task): plan p1-3 query planner provider reliability` after validation.

## Phase 1 — Report v5 and exact diagnostics only

- [x] Add all safe Planner observation fields and bump only the E2E report to v5.
- [x] Add strict Research/Exploratory gates while preserving Direct `policy_disabled`.
- [x] Add stable Planner failure classifier and use it at fallback projection.
- [x] Add redacted Planner lifecycle events through `qa::trace`.
- [x] Add D1–D5 and logging/field-count tests; prove no Planner behavior changed.
- [x] Run focused fmt/Clippy/real-E2E deterministic tests.
- [x] Commit `test(qa): expose and gate real planner execution`.

## Phase 2 — One real Research diagnosis

- [x] Run public `real-research-improvement` exactly once with `gpt-5.6-luna` / `low`.
- [x] Write a dedicated Planner diagnostic report without overwriting the Direct PASS report.
- [x] Record every required Planner/budget/plan/retrieval aggregate.
- [x] Do not rerun and do not change production code in this phase.

## Phase 3 — Select exactly one branch

- [x] Select A/B/C/D/E/F only from the observed stable reason (`provider_exit` -> F).
- [x] Document why other branches are excluded.
- [x] Limit any potential production change to the Provider Adapter; no defect is yet proven.

## Phase 4 — RED reproduction before fix

- [x] Add the smallest deterministic fixture for the chosen branch's observed failure path.
- [x] Reproduce `CODEX_EXIT_ERROR` -> `provider_exit` at the Planner boundary and verify redaction.
- [x] Commit `test(qa): reproduce real planner failure category` while keeping production behavior unchanged.

## Phase 5 — Minimal single-layer fix

- [x] Do not modify production behavior because no RED success expectation proves an Adapter defect.
- [x] Preserve strict parser/normalization, budgets, retries, safety gates, and unrelated QA behavior.
- [x] Record Branch F's evidence-based no-fix outcome instead of manufacturing a speculative patch.

## Phase 6 — Deterministic regression

- [x] Run retrieval-contract, query-plan, adaptive-routing, production-core, real-E2E, and claim-verification Rust tests.
- [x] Add and pass the Exploratory Stub Planner success test without real Exploratory execution.
- [x] Run Python evaluator, relevant frontend QA, build, fmt, and Clippy gates.
- [x] Record only documented unrelated all-targets/baseline frontend issues.

## Phase 7 — One real Research verification

- [x] Rerun `real-research-improvement` exactly once using a separate verification report.
- [x] Record Planner, plan, budget, Semantic, Final Grounding, persistence, scope, and exit aggregates.
- [x] Classify final status as FAIL because Planner remained `failed_fallback`; never rerun.
- [x] Commit `test(qa): verify real research planner path`.

## Phase 8 — Spec, archive, journal, delivery

- [x] Update QA code-specs with Report v5, stable Planner diagnostics, strict gates, and the proven branch contract.
- [x] Write the 21-item `result.md`.
- [ ] Archive task, record journal, normally push `master`, and verify remote SHA.

## Guardrails

- No formal Independent Heldout access or execution.
- No call/token budget, Direct Schema, Generator, Semantic, Grounding, Retrieval/Reranker/Embedding, frozen-data/threshold, zero-evidence, or performance changes.
- No production fix before the first real diagnostic and the chosen-branch RED test.
- No reset, clean, stash, amend, history rewrite, or force push.
