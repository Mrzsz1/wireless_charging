# P1-1B Final Grounding Gate — Result

## Outcome

- 已将 claim verification 拆成 Draft Audit 与 deterministic Final Audit。
- persistence、`CitationValidation.supported`、grounding status 与 trusted history 只读取 Final Audit。
- Draft 失败计数保留在 manifest/E2E 诊断中，不再污染修复后的最终 gate。
- Research 真实 case 已达到 `final 3/3 supported`、unsupported=0、unknown=0、coverage=1.0、persisted=true。
- Direct 真实 case 正确返回 `insufficient_supported_claims` 并拒绝持久化；本轮生成器只输出一条无 `[E#]` 的证据不足陈述，因此没有 eligible semantic claim，`semanticStatus=not_requested`。未修改 Direct LLM budget。

## Files changed

- `apps/desktop/src-tauri/src/qa/claim_verification.rs`
- `apps/desktop/src-tauri/src/qa/grounding.rs`
- `apps/desktop/src-tauri/src/qa.rs`
- `apps/desktop/src-tauri/src/qa/context.rs`
- `apps/desktop/src-tauri/src/qa/real_e2e.rs`
- `apps/desktop/src/types.ts`
- `evals/reports/qa-real-generator-e2e-research-report.json`
- `evals/reports/qa-real-generator-e2e-direct-report.json`
- `.trellis/spec/backend/qa-contract.md`

## Final projection rules

1. Repair keeps only Draft `Supported` and `ResearchSuggestion/NotApplicable` claims.
2. Contradicted/NotVerifiable/Unavailable claims become fixed system notices; PartiallySupported becomes the fixed “未支持部分已省略” notice and never retains the full original claim.
3. Final factual claims are re-extracted without another LLM.
4. A final fact must map to an exact Draft `Supported` claim after whitespace and repair-boundary leading-punctuation normalization, with the identical evidence-ID list.
5. Every evidence ID must exist in the current evidence set and at least one cited item must be non-Graphify.
6. New/unmatched final facts fail closed. Fixed system notices are not factual and require no citation.
7. No supported fact produces the fixed insufficiency answer, `insufficient_supported_claims`, no persistence, and no trusted history.

## Schema and logs

- `QaRunManifest`: `qa-run-v22`, with `finalGroundingAudit` beside Draft fields.
- Real E2E report: `qa-real-generator-e2e-report-v3`, with explicit `draft*` and `final*` metrics.
- Added safe `qa_final_grounding_audit_completed` aggregate trace event.
- Optional raw diagnosis uses `QA_REAL_E2E_GROUNDING_DIAGNOSTIC_DIR`; it requires an absolute directory outside the repository and redacts local paths/unsafe links.
- The temporary `E:/qa-grounding-diagnostics-p1-1b` JSON artifacts were inspected and deleted; no raw claim/evidence diagnostic is committed.

## Focused verification

- `cargo fmt --check`: PASS.
- Claim verification tests: 20 PASS.
- Grounding tests: 11 PASS.
- Context/contract tests: 11 PASS.
- Real E2E runner tests: 12 PASS.
- Persistence/trusted-history integration tests: 3 PASS.
- `cargo clippy --lib -- -D warnings`: PASS.
- `npm run build`: PASS.
- `cargo clippy --all-targets -- -D warnings`: repository-existing test-only failure at `qa/evaluation.rs:1423` (`field_reassign_with_default`); P1-1B's analogous test warning was fixed. No unrelated evaluation code was changed.

## Real runs

### Research

- Report: `evals/reports/qa-real-generator-e2e-research-report.json`
- Draft: 5 claims; supported=3; not-verifiable=2; repaired=2.
- Final: factual=3; supported=3; unsupported=0; unknown=0; coverage=1.0.
- Persistence: true; case errors: none; case passed: true.
- Single-case runner aggregate stays `passed=false` by the existing full-suite-only rule.

### Direct

- Report: `evals/reports/qa-real-generator-e2e-direct-report.json`
- Draft: 1 uncited NotVerifiable claim; repaired=1.
- Final: factual=0; status=`insufficient_supported_claims`.
- Persistence: false; stable error=`citation_validation_failed`.
- This verifies Final Audit correctness; the remaining failure is generator evidence mapping, not stale Draft counters.

## Production behavior boundary

- Modified: AnswerRepair safety projection, final grounding/persistence/trusted-history gate, answer contract priority, manifest/report/logging.
- Unmodified: retrieval, query planner, state, reranker, embedding, semantic verifier decision strategy, LLM budgets/thresholds, zero-evidence policy, frozen/heldout datasets, and natural Markdown renderer behavior.
