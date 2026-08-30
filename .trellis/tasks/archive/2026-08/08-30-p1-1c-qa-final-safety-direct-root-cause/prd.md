# P1-1C QA Final Safety and Direct Root Cause

## Goal

Make `qa-run-v22` final-grounding semantics consistent across heldout export, trusted conversation history, the production UI, real-E2E reporting, and post-render integrity. Then run one auditable Direct diagnostic, change only the proven failing layer, and rerun Direct once.

## Background

- Actual baseline for this task is `abd5e02a74dfdb3e639c87c62f97a2ee59a5d47e` on `master`. The taskbook's older `65d10eb` reference is not a reset target; intervening commits are the completed P1-1B final-grounding work and project delivery/logging rules.
- Phase 0 worktree was clean before task creation.
- Research already demonstrated Draft `3 supported + 2 not-verifiable`, Final `3/3 supported`, and `persisted=true`.
- Direct `real-direct-rose` currently reaches evidence-backed generation but ends with no final supported factual claim; its failing layer must be proven, not guessed.

## Requirements

### R1 — Strict scope and sequence

- Execute deterministic Phases 1–5 before any real Provider call.
- Permit exactly one Direct diagnostic in Phase 6 and exactly one Direct verification rerun in Phase 8.
- Never run, read, inspect, or continue an Independent Heldout formal run; never modify a frozen/heldout dataset.
- Do not change token ceilings, LLM call budgets, semantic-verifier decision strategy, grounding/citation/final-audit thresholds, query planner/fallback, retrieval/reranker/embedding unless the Direct diagnosis proves taskbook branch A, zero-evidence completeness, retries, or performance behavior.

### R2 — Heldout export uses final supported claims

- For `qa-run-v22`, `answerClaims` must come from `runManifest.finalGroundingAudit.claims` and include only `Supported` final claims.
- Claim text must be deterministically projected to final visible text; `citedEvidenceIds` remain structured provenance.
- Missing/invalid final audit, unsupported/non-factual claims, unknown evidence IDs, duplicate IDs, empty visible text, or projection mismatch fail closed.
- Python evaluator independently validates audit schema/status/counts/source mapping/evidence IDs rather than trusting the Rust runner.
- Backward compatibility is explicit for pre-v22 fixtures only; formal current runs cannot silently fall back to Draft claims.

### R3 — Trusted history uses final supported facts only

- Build trusted assistant context from ordered, deduplicated Final Supported claims after citation-token removal.
- Exclude research suggestions, not-applicable/not-verifiable/contradicted claims, system notices, model supplements, and evidence appendix text.
- If Final supported count is zero, trusted context is empty.

### R4 — Only finalized answers reach the production UI

- Generator output remains internal until semantic verification, repair, final audit, render/post-render checks, and persistence succeed.
- Production UI receives no raw `Token` content. It receives only terminal `Completed` final content, or `Failed`/`Cancelled` without draft content.
- The UI may show phase/progress text but must not render, store, or expose copy controls for raw draft text.

### R5 — Single-case E2E semantics are unambiguous

- Add `scope`, `executedScopePassed`, `fullSuiteEvaluated`, and `releaseEligible`.
- `passed` and CLI exit `0/2` reflect only the executed scope; environment/provider/file failures remain exit `1`.
- A passing single case has `releaseEligible=false`; only a passing full five-case suite is release eligible.

### R6 — Final claim provenance and visible-body integrity

- Canonicalize claim mapping by normalized text plus sorted unique evidence IDs.
- Preserve duplicate Draft claims with an ordered queue instead of overwriting by text.
- Each final supported claim records its source Draft claim ID and bounded provenance metadata.
- After natural rendering, deterministically validate that every audited supported claim exists in the visible body and that no new visible factual claim was introduced. Ignore only canonical renderer exclusions such as appendix, system notices, model supplement, link targets, code, and math.
- Projection failure is fail closed and blocks persistence.

### R7 — Direct root-cause branch is evidence-driven

- Diagnostic report must answer: `evidenceAnswersQuestion`, `rawGeneratorContainsEvidenceId`, `parserExtractedEvidenceId`, and `finalProjectionPreservedEvidenceBinding`.
- Select exactly one taskbook branch: A retrieval/evidence selection, B Direct generator structured binding, or C citation/parser/render/projection preservation.
- Add a public deterministic regression for the selected branch before the minimal production fix.
- If Direct reaches semantic verification and is blocked by `semantic_verifier:call_budget`, stop without changing budgets and report the independent P1-2 blocker.

### R8 — Diagnostic logging

- Every modified lifecycle emits structured stage start/completion/failure events through the existing application log sink with shared operation/request identity and stable error codes.
- Logs contain aggregate counts/statuses only; no question, answer, claim/evidence text, provider payload, secret, or absolute path.
- Tests assert representative success and failure event sequences for newly instrumented behavior.

## Acceptance Criteria

- [ ] `qa-run-v22` heldout bundle exports only Final Supported claims; Draft 5 / Final 3 fixture exports exactly 3.
- [ ] Python evaluator independently rejects missing/invalid final audit and unknown cited evidence.
- [ ] Trusted context contains only ordered Final Supported factual claims and is empty when none are supported.
- [ ] Production `Token` event count is zero for success, failure, and cancellation; only `Completed` carries the final answer.
- [ ] Single-case pass exits 0 with `releaseEligible=false`; full-suite eligibility is separate.
- [ ] Duplicate/evidence-order final claims map deterministically to source Draft IDs.
- [ ] Post-render visible projection accepts canonical renderer transformations and rejects renderer-introduced facts.
- [ ] Direct diagnosis records all four booleans and only the proven A/B/C layer changes.
- [ ] Direct verification has at least one Final Supported factual claim, zero unsupported/unknown citations, coverage `1.0`, and persistence, or is explicitly `PARTIAL-BLOCKED` only by semantic call budget.
- [ ] Required Rust, Python, frontend, build, formatting, and Clippy checks pass; only documented pre-existing unrelated lint may remain.
- [ ] No Independent Heldout content is run or read; budgets, planner, frozen thresholds/datasets, reranker, embedding, and zero-evidence policy remain unchanged.
- [ ] Stage-level structured logs make the failing lifecycle step directly identifiable.

## Out of Scope

- Independent Heldout execution or curation.
- Query Planner and `planner failed_fallback` repairs.
- Exploratory state aliases and zero-evidence completeness changes.
- Token/LLM budget changes, semantic-verifier policy changes, retry inflation, and performance optimization.
- Verified incremental streaming; this task intentionally uses final-only answer delivery.
