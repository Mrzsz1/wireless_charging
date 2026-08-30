# P1-1C Final Result

**Status: PARTIAL-BLOCKED**

1. **Baseline Commit**: `abd5e02a74dfdb3e639c87c62f97a2ee59a5d47e` (the actual clean repository baseline; newer than the taskbook reference).

2. **Final Commit**: final work commit `d85cec981c1548293dbbeef6a7698f7c9033cddf`; archive/journal delivery commits follow separately.

3. **修改文件列表**:
   - Production/evaluation: `apps/desktop/src-tauri/src/heldout_runner.rs`, `apps/desktop/src-tauri/src/lib.rs`, `apps/desktop/src-tauri/src/qa.rs`, `apps/desktop/src-tauri/src/qa/{claim_verification,context,direct_answer,grounding,production_core,real_e2e}.rs`, `apps/desktop/src/features/qa/AskView.tsx`, `apps/desktop/src/types.ts`.
   - Tests/tools/report: `tests/test_qa_accuracy_eval.py`, `tools/qa_accuracy_eval.py`, `apps/desktop/tests/qa-evidence-state.test.ts`, `evals/reports/qa-real-generator-e2e-report.json`.
   - Contracts/task records: `.trellis/spec/backend/qa-contract.md`, `.trellis/spec/frontend/quality-guidelines.md`, and this Trellis task plus its parent child-link metadata.

4. **是否运行或读取 Independent Heldout？** NO. Neither formal questions nor formal run answers were opened or executed.

5. **Heldout answerClaims 现在来自哪里？** Only `runManifest.finalGroundingAudit.claims` whose Final status is `supported`, after canonical visible-text projection; Draft `claimVerifications` are not exported as answers.

6. **Draft 5 / Final 3 fixture 的结果？** PASS: exactly 3 Final Supported `answerClaims` are exported.

7. **Python evaluator 如何独立校验 Final Audit？** It independently checks v2 Final Audit status/counts/coverage, exact visible hashes/source mappings, stable claim IDs, evidence checksums, and requires every `citedEvidenceId` to be a current complete EvidenceItem ID.

8. **trusted_context 现在如何构造？** Ordered Final Supported claims only, projected through the shared natural visible-text transform, with stable de-duplication.

9. **Research Suggestion 是否会进入 trusted history？** NO. Suggestions, system notices, model supplements, appendices, and every non-supported claim are excluded.

10. **原始 Generator Token 是否还会发给 UI？** NO. Production has no token event variant; the UI receives progress and one persisted Final `completed` result.

11. **Final FAIL 时用户是否可能看见原始草稿？** NO. Raw Provider text stays backend-internal and failure/cancellation does not emit draft content.

12. **Single-case PASS 的退出码是多少？** 0. Single-case failure is 2; infrastructure/provider/file errors are 1.

13. **executedScopePassed / releaseEligible 分别是什么意思？** `executedScopePassed` reports whether every case actually selected for this invocation passed. `releaseEligible` is true only after the complete frozen five-category public suite was evaluated and passed.

14. **Final Claim 如何处理重复文字？** A canonical `(visible text, sorted unique Evidence IDs)` key maps to a FIFO queue of Draft claims, so duplicates are consumed in order rather than overwritten.

15. **Evidence ID 顺序是否归一化？** YES. IDs are sorted and deduplicated before set/key comparison.

16. **是否记录 sourceDraftClaimId？** YES. Every Final Supported claim records its source Draft claim ID and provenance metadata.

17. **渲染后 visible projection 如何校验？** The audited body and final natural-rendered body use the shared canonical visible-text transformation, SHA-256 hashes, ordered visible-claim equality, and source-mapping count/order checks; a renderer-added fact fails closed.

18. **Direct 四项诊断**: `evidenceAnswersQuestion=true`; `rawGeneratorContainsEvidenceId=false`; `parserExtractedEvidenceId=false`; `finalProjectionPreservedEvidenceBinding=false` because no binding existed before the fix.

19. **Direct 最终选择了哪个分支？** Branch B. Retrieval selected answerable ROSE evidence, while the Provider omitted all IDs. Direct now uses strict internal `qa-direct-grounded-answer-v1`; the backend validates current non-Graph evidence bindings and deterministically renders natural citation-bearing text. Research/Exploratory behavior was not moved to this schema.

20. **Direct 真实重跑**: Evidence=2; Draft claims=1 (now with one Evidence ID); Final factual=0; Final supported=0; Final unsupported=0; citation coverage=0.0; semantic status=`unavailable` / `llm_budget_exceeded`; persisted=false; `executedScopePassed=false`; exit=2. This proves evidence binding is fixed and the next blocker is the independent semantic call budget.

21. **是否修改 Token Budget？** NO.

22. **是否修改 Planner？** NO.

23. **是否修改 Frozen Threshold？** NO.

24. **测试结果**:
   - Rust fmt PASS; Clippy lib/bins PASS.
   - Rust focused: heldout 12 PASS; `final_grounding` literal filter matched 0, so the actual `final_` filter ran 13 PASS; trusted-context 3 PASS; real-E2E 14 PASS; claim-verification 24 PASS; Direct schema/parser 3 PASS.
   - Python evaluator 24 PASS; frontend QA 6 PASS; TypeScript/Vite production build PASS.
   - All-targets Clippy reaches only the taskbook-documented pre-existing `apps/desktop/src-tauri/src/qa/evaluation.rs:1423 field_reassign_with_default`; no task-introduced warning exists.

25. **未解决问题和下一阻塞项**: P1-2 must address `semantic_verifier:call_budget`. P1-1C does not change the budget, semantic decision policy, verification thresholds, retrieval, planner, reranker, embedding, frozen data, or zero-evidence completeness to bypass it.
