# P1-2 Final Result

**Status: PASS**

1. **Baseline Commit**: `71836e89de8183e24917d8fdef4022dcc2f1dfa8`.

2. **Final Commit**: final work/spec commit `b6290af`; archive and journal delivery commits follow separately.

3. **修改文件列表**:
   - Budget/runtime: `apps/desktop/src-tauri/src/qa/adaptive_routing.rs`, `qa/claim_verification.rs`, `qa/production_core.rs`.
   - Deterministic contract tests: `apps/desktop/src-tauri/src/qa.rs`, `apps/desktop/src-tauri/src/heldout_runner.rs`.
   - Real report: `evals/reports/qa-real-generator-e2e-report.json`.
   - Spec/task records: `.trellis/spec/backend/qa-contract.md` and this Trellis task directory.

4. **adaptive-routing 版本**: `adaptive-routing-v2`.

5. **Call Budget**: Direct=3, Research=4, Exploratory=5.

6. **Token Ceiling 是否修改？** NO。仍为 Direct=8,000、Research=18,000、Exploratory=32,000。

7. **semanticVerifierCallReserve**: every online routing policy reserves exactly 1 call.

8. **非 Semantic 阶段如何防止消耗保留位？** Admission requires `nextCalls + remainingSemanticReserve <= llmCallBudget`. A rejected optional stage changes neither calls-used nor the Semantic-used counter.

9. **Semantic 调用失败后是否返还 Call Budget？** NO。`settle`, `release`, Drop, cancellation cleanup, and Provider failure release only in-flight token capacity; the admitted total call and Semantic reserve remain consumed.

10. **Direct 三阶段测试**: Understanding, Generator, and Semantic all admitted; `callsUsed=3`; no rejection.

11. **Research 四阶段测试**: Understanding, Planner, Generator, and Semantic all admitted after Direct-to-Research reconfiguration; `callsUsed=4`; no rejection.

12. **额外非 Semantic 调用**: rejected with `planner_retry:call_budget`; calls-used remains 2; the following Semantic call is still admitted.

13. **第二次 Semantic**: rejected with `semantic_verifier:call_budget`, including after release/provider failure; the reserve is non-refundable.

14. **Token 超限**: still rejected with `semantic_verifier:token_budget`; no Provider call occurs.

15. **无 eligible Claim**: status=`not_requested`, Provider invocation count=0, calls-used remains 0, Semantic reserve remains available.

16. **Unknown**: remains `NotVerifiable`, never becomes Supported.

17. **真实 Direct**:
    - routingLlmCallBudget=3; routingLlmCallsUsed=3; routingBudgetRejections=0.
    - semanticStatus=`succeeded`; semanticFallbackReason empty.
    - Draft claims=1, supported=1, Evidence ID count=1.
    - Final factual=1, supported=1, unsupported=0, citation coverage=1.0.
    - persisted=true; executedScopePassed=true; exit code=0.

18. **是否修改 Semantic Prompt / Decision？** NO。

19. **是否修改 Retrieval / Planner / Generator？** NO。Only the budget guard, shared stage identifier, aggregate telemetry usage, and tests changed.

20. **是否运行 Independent Heldout？** NO。Only synthetic heldout fixtures were executed; no formal question or answer was opened.

21. **是否修改 Frozen Threshold？** NO。

22. **质量结果**:
    - Rust: fmt PASS; Clippy lib/bins PASS; adaptive-routing 18 PASS; claim-verification 28 PASS; production-core 3 PASS; real-E2E 14 PASS; synthetic heldout 12 PASS.
    - Python evaluator: 24 PASS.
    - Frontend: `test:qa-evidence` 6 PASS; production TypeScript/Vite build PASS. `package.json` has no `test:qa`; the additional `test:qa-settings` run exposed one baseline-stale UI wording/cursor assertion, and baseline comparison proves neither file changed in P1-2, so it was not modified outside scope.
    - All-targets Clippy reaches only the taskbook-documented pre-existing `qa/evaluation.rs:1423 field_reassign_with_default`; there are no task-introduced warnings.

23. **最终状态**: **PASS**. Deterministic reserve enforcement, Direct/Research integration, production persistence, and the single real Direct run all passed; Semantic is no longer blocked by call budget.
