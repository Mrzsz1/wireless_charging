# P1-1D Implementation Plan

1. Correct the previous P1-3S journal status to `PARTIAL-BLOCKED` and point its next step to P1-1D.
2. Commit this task plan on baseline `6ad4d767`.
3. Add a synthetic substring/occurrence collision test whose expected safe projection fails on the legacy `String::replacen()` implementation; run only that focused test and preserve the RED output.
4. Add repair projection audit types and a pure ordered source-span locator/rebuilder with explicit validation and stable errors.
5. Replace the production mutation loop with one-shot source reconstruction and post-repair factual-claim invariant validation; keep all existing status mappings and zero-supported behavior.
6. Wire safe audit metadata into the run manifest/types and add lifecycle trace events at the QA orchestration boundary.
7. Add R1-R12 deterministic regressions, including fail-closed span errors, notices, trusted history, visible projection, and unknown evidence.
8. Run focused checks while iterating:
   - `cargo test --lib claim_verification`
   - focused QA orchestration/trace tests
9. Run the taskbook quality matrix from `apps/desktop/src-tauri` / `apps/desktop` as appropriate:
   - `cargo fmt --check`
   - `cargo clippy --lib --bins -- -D warnings`
   - `cargo test --lib claim_verification`
   - `cargo test --lib production_core`
   - `cargo test --lib real_e2e`
   - `cargo test --lib heldout_runner` (unit tests only; no Independent Heldout execution)
   - `cargo test --lib natural_answer`
   - `py -3 -m unittest tests.test_qa_accuracy_eval`
   - `npm run test:qa-evidence`
   - `npm run build`
   - `git diff --check`
10. After all deterministic checks pass, clear Shell proxy variables and run `QA_REAL_E2E_CASE_ID=real-research-improvement npm run eval:qa-real-e2e` exactly once.
11. Record exact real-run metrics, commit verified code/spec/task results, archive the Trellis task, record the journal, and normally push `master` to `origin`.

## Rollback Points

- RED fixture commit: tests only, intentionally failing until the next commit.
- Source-span commit: isolated production behavior change.
- Diagnostics/tests commits: schema-compatible and independently reversible.
