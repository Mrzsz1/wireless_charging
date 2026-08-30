# P1-3 Query Planner Provider Reliability

## Goal

Prove the real Research Query Planner failure layer using a redacted Report v5 and stable failure taxonomy, then select exactly one evidence-backed A–F branch, reproduce it deterministically, and apply only that branch's minimal production fix. A successful fallback answer must never masquerade as a successful Planner execution.

## Background

- Clean baseline: `4e8399589ca20a33980309fb057d4f125a1262bd` on `master`.
- Runtime: Rust 1.96.1, Node 24.11.0, npm 11.15.0, Python 3.13.0, Codex CLI 0.146.0.
- Real E2E defaults: `gpt-5.6-luna`, reasoning effort `low`.
- Historical Research reports showed `plannerStatus=failed_fallback`, but lacked enough stable detail to distinguish budget, Provider schema, timeout, protocol, parser, or contract-validation failures.

## Requirements

### R1 — Diagnostic-first sequence

- Complete Report v5, precise Planner failure classification, lifecycle logging, and strict Research/Exploratory gates before any real Provider call.
- Run `real-research-improvement` exactly once for diagnosis, using a separate report path so the latest Direct PASS report remains intact.
- Select exactly one A–F branch only from the reported `plannerFallbackReason` and aggregate Planner telemetry.
- Before changing production behavior for that branch, add a minimal deterministic test and prove it fails on the diagnosed implementation.
- After the fix and deterministic regressions, rerun the same real Research case exactly once; never loop on an unfavorable result.

### R2 — Report v5

- Upgrade only `qa-real-generator-e2e-report-v4` to v5; keep `qa-run-v22` unchanged.
- Project safe Planner fields from existing manifest/retrieval/budget telemetry: attempted/used/status/fallback/reason/latency, stage observed, budget rejected, QueryPlan version, facet/query/kind counts, and retrieval stop reason.
- Never serialize question, prompt, raw output, search-query text, snippet, stderr, Provider payload, or local path.

### R3 — Exact failure classification

- Add `stable_planner_failure_kind(error) -> &'static str` with the taskbook's fixed enum covering call/token budgets, output-schema rejection, idle/total timeouts, rate/exit/protocol/unavailable Provider failures, RetrievalContract JSON/schema/scope/kind/facet/budget/general validation, cancellation, and unknown.
- Manifest/report/logging receive only the stable enum, never the raw error string.

### R4 — Planner lifecycle diagnostics

- Emit `qa_planner_started`, `qa_planner_completed`, or `qa_planner_failed` at the actual Planner boundary using the existing QA trace sink and shared request identity.
- Record only provider/model, execution mode, baseline candidate count, latency, safe counts, stable fallback reason, and aggregate budget rejection code.
- A diagnostic/logging failure must not alter Planner, fallback, retrieval, or answer behavior.

### R5 — Strict E2E gates

- Research/Exploratory require attempted=true, used=true, succeeded status, no fallback/reason, observed Planner budget stage without rejection, retrieval-contract v2, at least one facet, and at least one planned search query.
- `failed_fallback` yields `planner_failed_fallback`; succeeded-without-plan yields `planner_success_without_plan`.
- Direct continues to accept `policy_disabled` and must not require Planner success.

### R6 — Evidence-selected minimal fix

- Select only one branch: A Provider Schema, B contract JSON/validation alignment, C Planner input or timeout, D duplicate budget-stage consumption, E token sizing/input accounting, or F Provider adapter/transient handling.
- Do not simultaneously modify Provider Schema, Parser, Timeout, Planner Input, Budget, Retrieval, or Adapter layers.
- Preserve parser/normalization strictness and all existing safety gates.

### R7 — Prohibited changes

- Do not run/read Independent Heldout, continue partial heldout runs, modify frozen data/thresholds, or change the real question.
- Do not modify P1-2 call budgets 3/4/5 or token ceilings 8,000/18,000/32,000.
- Do not modify Direct Answer Schema, Generator Prompt, Semantic Prompt/decision/thresholds, Grounding/Final Audit standards, Retrieval/Reranker/Embedding, zero-evidence behavior, or performance behavior.
- Do not relabel fallback as success, weaken E2E gates, add unbounded retries, rewrite Git history, or force push.

## Acceptance Criteria

- [ ] Report v5 contains all safe Planner diagnostics and passes redaction tests.
- [ ] D1–D5 cover failed fallback, true success, Direct policy-disabled, every stable failure prefix, and report redaction.
- [ ] Phase 1 makes no Planner behavior change and makes no real Provider call.
- [ ] First real Research diagnostic is executed exactly once and records every required aggregate.
- [ ] Exactly one A–F branch is selected and justified from `plannerFallbackReason`.
- [ ] The diagnosed branch has a recorded RED test before any production fix.
- [ ] Only the selected layer changes and its focused tests pass.
- [ ] Exploratory Stub Planner succeeds without a real Exploratory call.
- [ ] Final real Research Planner succeeds without fallback, produces at least one facet/query, has no budget rejection, and the final answer remains safely grounded/persisted—or is classified strictly as the taskbook's external blocker.
- [ ] Call/token budgets, `qa-run-v22`, Direct Schema, Generator/Semantic/Grounding behavior, frozen thresholds/data, and formal heldout remain unchanged.
- [ ] Required Rust/Python/frontend/build/Clippy gates complete with only documented unrelated baseline issues.

## Out of Scope

- Real Exploratory execution, P1-4 state-alias work, forcing a second retrieval round, Planner performance optimization, and unrelated frontend cleanup.
