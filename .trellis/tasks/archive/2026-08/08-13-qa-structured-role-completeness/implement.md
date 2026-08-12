# Implementation Plan

1. Define canonical role descriptors per intent in the context contract and serialize them as JSON in the prompt.
2. Extend structured claims with `role`; validate explicit roles and bounded legacy label aliases.
3. Return observed roles from structured parsing and feed them into completeness validation.
4. Replace Markdown exact-phrase element checks on structured answers with role-set checks; preserve section/count checks and audit DTO shape.
5. Update fixtures and add regressions for natural labels, missing roles, legacy aliases, and unknown roles.
6. Update QA code spec, run focused Rust tests/checks and diff review.
7. Commit implementation, run `npm run tauri build`, archive the task, and record the session.
