# P1-2 Semantic Verifier Call Budget

## Goal

Guarantee one non-consumable Semantic Verifier call slot for every online QA routing mode while retaining a finite total LLM-call budget and the existing token ceilings. Prove the Direct and worst-case contextual Research failures with deterministic red tests before changing production budget logic.

## Background

- Baseline is clean `71836e89de8183e24917d8fdef4022dcc2f1dfa8` on `master`.
- The P1-1C Direct rerun used Understanding and Generator, then rejected Semantic at `semantic_verifier:call_budget` with two calls used and ample token headroom.
- A simple Direct `2 -> 3` change is insufficient because contextual Research legally needs Understanding + Planner + Generator + Semantic, and optional stages must not consume the verifier's final slot.

## Requirements

### R1 — Test-first staged execution

- Before changing production policy/guard behavior, add deterministic B1–B7 tests that fail against the baseline for Direct, Research, reserve protection, one-shot Semantic, token enforcement, reconfiguration, and failed-provider accounting.
- Record the failing command and failure reasons without committing a deliberately red production branch.
- Do not make a real Provider call until every deterministic phase passes.

### R2 — Versioned finite policy

- Upgrade `ROUTING_POLICY_VERSION` from `adaptive-routing-v1` to `adaptive-routing-v2`.
- Set total call budgets to Direct=3, Research=4, Exploratory=5.
- Keep token ceilings exactly Direct=8,000, Research=18,000, Exploratory=32,000.
- Add `RoutingPolicy.semantic_verifier_call_reserve = 1` for every online mode.

### R3 — Non-consumable Semantic reserve

- Define one shared `SEMANTIC_VERIFIER_STAGE` constant and use it in the guard and claim-verification caller.
- Track successful Semantic reservations separately. Semantic may acquire at most the configured reserve.
- A non-Semantic reservation is admitted only when `next_calls + remaining_semantic_reserve <= llm_call_budget`.
- A rejected reservation changes neither calls-used nor semantic-used state.
- `settle`, `release`, Drop, and Provider failure release token in-flight capacity but never refund call attempts or the Semantic reserve.
- Token admission remains exactly `used + in_flight + requested <= token_ceiling` and fails closed independently of call availability.
- Reconfiguration preserves all prior usage, including whether the Semantic reserve has already been consumed.

### R4 — Integration behavior

- Fixture Semantic tests must call the real `run_semantic_verification` path after prior legal stage reservations.
- Eligible Direct claims can reach Semantic after Understanding + Generator.
- Eligible contextual Research can reach Semantic after Understanding + Planner + Generator.
- No eligible claim means `not_requested` and consumes no Semantic slot.
- Semantic `unknown` remains `NotVerifiable`; no decision/grounding threshold changes.
- A synthetic production chain proves policy/manifest telemetry, Final Supported count, and persistence without mocking `LlmBudgetGuard`.

### R5 — Scope and safety

- Modify only call-budget policy, reserve enforcement, shared stage naming, deterministic tests, existing aggregate telemetry assertions, specs, and task records.
- Do not modify token ceilings, Semantic prompt/decision logic, grounding/citation thresholds, retrieval, reranker, embedding, query-planner behavior, Generator prompt/schema, zero-evidence rules, retries, frozen thresholds/data, or manifest schema.
- Do not run or inspect Independent Heldout formal questions/answers.
- Keep `qa-run-v22` and `qa-real-generator-e2e-report-v4` schemas unchanged.

### R6 — Diagnostics and delivery

- Existing `routingLlmCallsUsed`, `routingBudgetRejections`, `routingLlmStages`, and Semantic lifecycle trace events must identify reserve success/failure without recording question, answer, evidence, prompt, or Provider payload content.
- After deterministic gates pass, run `real-direct-rose` exactly once and do not loop on an unfavorable result.
- Commit phases separately, archive/journal the Trellis task, and push normally to GitHub `origin/master` without force.

## Acceptance Criteria

- [ ] B1 Direct allows Understanding + Generator + Semantic, `callsUsed=3`, no rejection.
- [ ] B2 Research allows Understanding + Planner + Generator + Semantic, `callsUsed=4`, no rejection.
- [ ] B3 an extra Direct non-Semantic call is rejected while the subsequent Semantic call remains available.
- [ ] B4 a second Semantic reservation is rejected.
- [ ] B5 a genuine Semantic token overrun remains `semantic_verifier:token_budget`.
- [ ] B6 Direct-to-Research reconfiguration preserves prior usage and allows the legal four-stage chain.
- [ ] B7 release/Drop/Provider failure does not refund a consumed Semantic call.
- [ ] Fixture integration covers semantic succeeded, token-budget unavailable, no eligible claim, and unknown-not-supported.
- [ ] Synthetic Direct production path records `adaptive-routing-v2`, budget 3, calls used 3, no rejections, Semantic succeeded/checked, Final Supported=1, and successful persistence.
- [ ] Contextual Research deterministic path records budget 4 and admits all four legal stages.
- [ ] Synthetic heldout contract parsing accepts `qa-run-v22` with `adaptive-routing-v2`; no formal heldout content is opened or run.
- [ ] Required Rust/Python/frontend/build/Clippy checks pass, except only the documented unrelated all-targets Clippy warning if still present.
- [ ] The one real Direct rerun no longer reports `semantic_verifier:call_budget`; outcome is classified strictly by the taskbook.

## Out of Scope

- Semantic token-reservation sizing, answer/evidence semantic-alignment repair, Provider retries, Planner fallback repair, complete five-case real E2E, Independent Heldout, and performance work.
