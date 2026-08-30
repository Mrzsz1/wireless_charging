# P1-3 Final Result

1. **Baseline Commit**: `4e8399589ca20a33980309fb057d4f125a1262bd`.
2. **Final work commit**: recorded by `test(qa): verify real research planner path`; archival/journal commits follow separately.
3. **Modified files**: `qa.rs`, `qa/provider_capabilities.rs`, `qa/real_e2e.rs`, `qa/trace.rs`, `.trellis/spec/backend/qa-contract.md`, two safe E2E reports, and this task's records.
4. **E2E Report version**: `qa-real-generator-e2e-report-v5`.
5. **QaRunManifest schema changed?** NO; it remains `qa-run-v22`.
6. **First real Research diagnosis**: attempted=true, used=false, status=`failed_fallback`, fallback=true, reason=`provider_exit`, latency=115457 ms, stage observed=true, budget rejected=false, calls used=3, rejections=[], QueryPlan v2, facets=1, queries=1, one retrieval round, stop=`max_rounds`.
7. **Selected branch**: F — Provider transient/unavailable (`provider_exit`).
8. **Why only F**: the observed stable enum is listed only under F; A–E require different exact reasons and were excluded.
9. **Pre-change failure-path test**: `query_planner_provider_exit_is_auditable_and_redacted`; the existing Windows Codex adapter fixture also exercises non-zero process exit.
10. **Exact root function/condition**: `codex_subscription::stream_answer_with` received a non-success Codex subprocess status and returned `CODEX_EXIT_ERROR`; the Planner boundary safely classified it as `provider_exit`. No deeper deterministic Adapter defect was proven.
11. **Modified production layer**: diagnostics/reporting only. The Provider Adapter, Planner Schema/Parser/Timeout/Input/Budget behavior was not modified.
12. **Other candidate roots modified?** NO.
13. **Final real Research**: status=`failed_fallback`, fallback=true, reason=`provider_exit`, latency=118301 ms, facets=1, queries=1, call budget/used=4/3, rejections=[], semantic=`succeeded`, final factual/supported/unsupported=3/3/0, citation coverage=1.0, persisted=true, executedScopePassed=false, exit code=2.
14. **Exploratory Stub Planner**: PASS; accepted a valid contract, used the Planner, produced facet/query counts, and did not fallback.
15. **Direct Schema changed?** NO.
16. **Semantic Prompt/Decision changed?** NO.
17. **Call Budget/Token Ceiling changed?** NO; 3/4/5 calls and 8,000/18,000/32,000 tokens remain frozen.
18. **Independent Heldout run/read?** NO.
19. **Frozen threshold changed?** NO.
20. **Quality**: focused Rust/Clippy, Python evaluator, QA evidence frontend test, and build pass. `npm run test:qa` does not exist; mapped QA settings has one pre-existing UI-text assertion failure. All-target Clippy has only the documented pre-existing `qa/evaluation.rs:1423` lint.
21. **Final status**: **FAIL** — the final real Planner still returned `failed_fallback`. No production patch was made because the required deterministic old-FAIL/new-PASS Adapter test was not established.
