# Logging Guidelines

## 1. Scope / Trigger

- Use this contract for every new or modified product feature and every behavior-changing bug fix. Instrument the user-visible workflow and each material stage that can fail independently; trivial pure helpers do not need one log per call.
- A feature is incomplete when a failure can only be located from a returned message or stack trace. Its logs must identify the feature, operation, stage, outcome, and stable failure class.
- Cross-layer features must emit authoritative lifecycle events at the backend orchestration boundary. Frontend-only flows must use or add a shared bridge to the application log sink; browser-console-only diagnostics do not satisfy this contract.
- Logging is diagnostic infrastructure. It must never alter QA answers, thresholds, budgets, persistence decisions, or user-visible error semantics.

## 2. Signatures and Storage

- Desktop logger: `tauri_plugin_log::Builder` is installed for debug and release with `level(Info)`, a 10 MiB file limit, and `RotationStrategy::KeepSome(5)`.
- Tauri's `LogDir` target owns the physical path. On Windows this resolves under the application's OS-managed data/log directory; code must not hardcode a user profile or installation path.
- QA structured events: `qa::trace::emit(&QaTraceEvent)` with schema `qa-trace-v1` and log target `qa_trace`.
- Development Real Answer E2E additionally writes the same event projection to `apps/desktop/logs/qa-real-e2e.jsonl`. The directory is Git-ignored and the file is truncated after 5 MiB before appending new JSON lines.
- New feature event families use stable snake-case names: `<feature>_started`, `<feature>_<stage>_started`, `<feature>_<stage>_completed`, `<feature>_<stage>_failed`, `<feature>_completed`, and `<feature>_failed`.

## 3. Contracts

- Every event contains `schemaVersion`, Unix-millisecond timestamp, stable `event`, `feature`, `operationId` (or a safe stable hash), `stage`, and `status`.
- Stage completion/failure records include `durationMs` when measurable. Retryable work also records `attempt`; failure records include a stable `errorCode` rather than raw error text.
- Optional fields are typed and allowlisted: public case ID, execution mode, provider/model identity, aggregate counts, persisted state, and stable error code. Field names and enum values are part of the diagnostic contract and must not be changed ad hoc between stages.
- Request IDs are SHA-256 hashed and shortened before logging. Error messages are reduced to the stable prefix before the first colon.
- QA lifecycle event families are `qa_prepare_*`, `qa_generate_*`, `qa_semantic_*`, `qa_audit_*`, `qa_persist_*`, and `qa_e2e_*`.
- `info` records normal lifecycle boundaries and measured degraded states. `warn` records logger serialization/lock/write failures and recoverable startup/deployment issues. `error` is reserved for process-level failures that cannot be represented through an existing result/error contract. Debug/trace must not carry content payloads.
- Logger write failure is fail-soft for production QA. E2E startup may fail closed when its required local trace directory cannot be configured, because the runner explicitly promises a trace artifact.
- File-backed production logs must use the application-owned `LogDir` and bounded rotation. Do not create feature-specific files in the repository, source tree, download directory, current working directory, or a hardcoded absolute path.

## 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| Event serialization fails | Emit only `event=qa_trace_serialize_failed`; continue the product operation |
| Desktop AppLogDir is unavailable | Tauri plugin initialization reports startup failure; never fall back to the repository or an arbitrary absolute path |
| E2E trace directory cannot be created | Stable `QA_TRACE_DIRECTORY_CREATE_FAILED`; do not run an untraceable diagnostic |
| E2E JSONL reaches 5 MiB | Truncate the local development trace, then append the next complete JSON line |
| Provider/semantic/persistence fails | Log stage/status/stable code and aggregate counts only; preserve the original caller-visible error |
| A proposed field contains content or a path | Do not add it to `QaTraceEvent`; use a hash, count, enum, or stable code instead |
| A material stage starts | Emit one `*_started` event with the shared operation ID before side effects begin |
| A material stage succeeds | Emit the matching `*_completed` event with duration/counts after its postcondition is true |
| A material stage fails or is cancelled | Emit the matching `*_failed` event with the same operation ID and stable `errorCode`; preserve cancellation semantics |
| A frontend-only feature has no persistent log bridge | Add/reuse the shared application-log bridge before considering the feature complete; do not scatter `console.log` calls |
| A feature changes without lifecycle-log tests | Quality gate fails; do not commit the feature as complete and do not push it |

## 5. Good / Base / Bad Cases

- Good: one operation emits `download_started` -> `download_transfer_started` -> `download_transfer_completed` -> `download_checksum_completed` -> `download_completed`; a checksum error instead ends at `download_checksum_failed` with the same operation ID and `errorCode=checksum_mismatch`.
- Base: `qa_persist_failed` records request hash, mode, evidence count, `persisted=false`, and `citation_validation_failed` without answer text.
- Bad: emit only `feature_failed`, forcing a maintainer to guess which step failed; scatter unrelated console lines; or log a question, answer, prompt, claim text, evidence snippet, repository/temp path, credential, token, provider payload, chain-of-thought, or raw error message.

## 6. Tests Required

- Every feature test suite asserts the success-stage sequence and at least one representative failure-stage sequence, including shared operation ID, exact failed stage, and stable error code.
- Unit tests assert request hashing, stable error-code projection, allowed JSON keys, one-record-per-line append behavior, and bounded desktop rotation configuration.
- Real E2E verification independently parses every JSONL line, rejects keys outside the allowlist, and confirms the log path is Git-ignored.
- Report/log redaction checks reject absolute Windows/UNC paths and raw content fields.
- Review must fail when a new side-effecting stage has no start/success/failure instrumentation or writes directly to an ad hoc log file.

## 7. Wrong vs Correct

### Wrong

```text
INFO feature failed: error=<provider body>
console.log("step failed", rawPayload)
```

### Correct

```json
{"schemaVersion":"feature-trace-v1","event":"model_download_checksum_failed","feature":"model_download","operationId":"16-hex","stage":"checksum","status":"failed","durationMs":842,"attempt":1,"errorCode":"checksum_mismatch"}
```
