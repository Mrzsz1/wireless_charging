# Implementation Plan — QA GitHub Review Remediation

## Phase A — Claim/Evidence fail-closed contract

- [x] Add failing tests for no-ID factual claims, cue-injection false facts, type/status separation, heuristic-vs-entailment telemetry and missing defaults.
- [x] Reorder audit pipeline so verifier consumes citation-preserving body before display rendering.
- [x] Persist per-claim audit and repair provenance; update Rust/TypeScript DTOs and UI wording.
- [x] Update QA contract and claim verification eval docs.
- [x] Run targeted + full gates.
- [x] Commit locally: `fix(qa): enforce claim evidence grounding`.

## Phase B — Cross-encoder reranking and calibrated fallback

- [x] Rename `SemanticResearchReranker` to `EmbeddingRescorer`; rename telemetry and tests.
- [x] Add `RerankProvider` and explicit local FastEmbed `TextRerank` loading without automatic query-time download.
- [x] Replace raw-score addition with rank fusion and preserve source/graph/reference protections.
- [x] Add unavailable/corrupt/cancel tests and non-tautological fixtures.
- [x] Extend eval report to distinguish cross-encoder from embedding/deterministic fallback.
- [x] Run retrieval metrics and compare frozen baseline.
- [x] Commit locally: `feat(qa): add cross-encoder rerank provider`.

## Phase C — Executable routing budgets

- [x] Add `LlmBudgetGuard` tests for reservation, usage, rejection and fallback.
- [x] Enforce `planner_enabled`; set DirectQA maximum retrieval rounds to one.
- [x] Route resolver/planner/generator/verifier calls through the request-scoped guard.
- [x] Persist actual calls/tokens and rejection diagnostics.
- [x] Run routing, latency and regression gates.
- [x] Commit locally: `fix(qa): enforce adaptive routing budgets`.

## Phase D — Provider capability symmetry

- [x] Define provider descriptor/capability traits and common planning boundary.
- [x] Adapt Codex planning to the common interface.
- [x] Implement Compatible API structured Understanding/QueryPlan calls with the existing secret boundary and fail-soft parsing.
- [x] Replace provider-name checks with capability checks.
- [x] Add parity, malformed output, missing key and budget tests.
- [x] Commit locally: `refactor(qa): unify planning provider capabilities`.

## Phase E — P1 understanding, profiles and neutral method discovery

- [x] Add deterministic routing confidence and confidence-based escalation.
- [x] Add four intent-specific answer/completeness profiles and tests.
- [x] Remove candidate method names from neutral first-round search terms.
- [x] Mark matcher outputs as hypotheses; discover methods from evidence before applicability matching.
- [x] Add corroboration/provenance telemetry and regression cases.
- [x] Commit locally: `feat(qa): improve research intent and method discovery`.

## Phase F — Exact parent context and final integration

- [x] Replace same-document longest-block expansion with exact active same-document `parent_block_id` lookup.
- [x] Add wrong-longest-sibling, missing-parent, cross-document and inactive-parent tests.
- [x] Run full gates: fmt, Rust tests/clippy, frontend tests/build, P3, RAG, question corpus, Wiki and release gates supported by the environment.
- [x] Update baselines without lowering thresholds.
- [x] Commit locally: `fix(qa): resolve exact evidence parent context`.

## Risky Files / Rollback Points

- `apps/desktop/src-tauri/src/qa.rs`: retrieval/audit/persistence boundary.
- `apps/desktop/src-tauri/src/lib.rs`: provider orchestration and streaming.
- `apps/desktop/src-tauri/src/qa/{natural_answer,claim_verification,grounding,reranker,evidence_manager}.rs`.
- `apps/desktop/src-tauri/src/qa/{context,understanding,problem_understanding,retrieval}.rs`.
- `apps/desktop/src/types.ts` and QA UI status text.
- `evals/**` and `.trellis/spec/backend/qa-contract.md`.

Rollback is phase-commit based; user-owned untracked files remain untouched.
