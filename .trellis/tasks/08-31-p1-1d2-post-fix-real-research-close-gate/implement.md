# P1-1D2 Implementation Plan

1. Record baseline and confirm clean worktree.
2. Run `cargo fmt --check`, Clippy, focused Rust suites, Python evaluator tests, QA evidence tests, frontend build, and `git diff --check`.
3. Confirm no production `replacen` remains in claim repair.
4. Commit the task plan and activate the task.
5. With Shell proxy variables removed, run exactly once:
   `QA_REAL_E2E_CASE_ID=real-research-improvement npm run eval:qa-real-e2e`.
6. Inspect and commit the metadata-only report and exact trace outcome.
7. Write `result.md`, update the QA spec only if a new executable contract was learned, archive, journal, and push normally.
