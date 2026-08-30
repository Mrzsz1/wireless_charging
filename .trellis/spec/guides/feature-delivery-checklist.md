# Feature Delivery Checklist

Use this checklist for every new feature and behavior-changing bug fix.

## Before implementation

- [ ] Identify the user-visible feature boundary and every material stage that can fail independently.
- [ ] Choose one stable operation ID that follows the workflow across frontend/backend boundaries.
- [ ] Read [`../backend/logging-guidelines.md`](../backend/logging-guidelines.md) and include log assertions in the task acceptance criteria.

## During implementation

- [ ] Emit feature start and feature completion/failure.
- [ ] Emit start/completion/failure for each material stage using stable event names, stage names, status values, and error codes.
- [ ] Send persistent diagnostics through the application-owned log sink; do not create ad hoc log files or rely on console-only output.
- [ ] Keep raw user content, answers, evidence, secrets, provider payloads, and absolute paths out of logs.

## Quality gate

- [ ] Relevant tests, lint, type checks, and builds pass.
- [ ] Tests assert the success-stage sequence and at least one representative failure-stage sequence.
- [ ] Logs identify the exact failed stage from `feature + operationId + stage + errorCode` without reading sensitive content.
- [ ] `git diff --check` passes and the working tree contains no unrelated generated artifacts.

## Delivery gate

- [ ] Commit verified work without amending or rewriting history.
- [ ] Complete Trellis archive and journal bookkeeping when a Trellis task exists.
- [ ] Unless the user explicitly requests local-only work, run a normal push of the current branch to GitHub `origin` after all task commits exist.
- [ ] Verify the remote branch contains the final local commit. Never use `--force` or `--force-with-lease`.
- [ ] If push fails, retain the local commits, report the branch/SHA/error, and retry only a normal push.
