# Implementation Plan — Conversation State + Query v2

## Phase A — State model and reducer

- [ ] Add patch/action/field/value/confidence types and deterministic reducer.
- [ ] Upgrade ResearchSessionState v2 with parameters, excluded methods, version/source fields.
- [ ] Implement add/remove/keep/replace/set/set_all/clear, ordered conflicts and fail-closed reports.
- [ ] Add focused reducer/parameter/self-correction tests.

## Phase B — Mutation extraction and state reconstruction

- [ ] Replace sentence-wide remove/replace behavior with clause-local ordered operations.
- [ ] Extend structured understanding schema/input with compact current state and patch-only output.
- [ ] Reconstruct history through patches; current turn uses resolved/provider patch when valid.
- [ ] Add ambiguous destructive and multi-operation tests.

## Phase C — State-aware query planning

- [ ] Add deterministic ResearchQueryContext builder and intent-aware selection.
- [ ] Build context only after current patch application.
- [ ] Extend RetrievalPlanningInput/fallback contract and initial terms with bounded current state.
- [ ] Project telemetry/state into RetrievalQuery/ContextPlan and update TypeScript types if serialized.
- [ ] Lock mutation → context → planner order with integration tests.

## Phase D — Conversation State Benchmark v2

- [ ] Create sealed conversation_state_v2_cases.json with 14 core + 20/50/100-turn cases.
- [ ] Implement evaluator/report and npm entry point.
- [ ] Generate metrics/baseline and verify production thresholds.

## Phase E — Limited regression and finish

- [ ] Rust fmt/check and state/query related tests.
- [ ] Required existing ResearchMemory/Understanding/Planner/Retrieval/Semantic/Reranker regression filters only.
- [ ] Update QA spec, task AC, local Git commit, Trellis archive/journal.

## Explicitly skipped

- Full GUI, installer, frontend build, full RAG/production gate, unrelated test suites.
- New Agents, database migrations, unlimited-history prompts, algorithm/case-specific patches.
