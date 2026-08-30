# Logging Guidelines

## 1. Scope / Trigger

- Use this contract whenever a desktop backend feature adds lifecycle, provider, evaluation, persistence, repair, download, or failure telemetry.
- Logging is diagnostic infrastructure. It must never alter QA answers, thresholds, budgets, persistence decisions, or user-visible error semantics.

## 2. Signatures and Storage

- Desktop logger: `tauri_plugin_log::Builder` is installed for debug and release with `level(Info)`, a 10 MiB file limit, and `RotationStrategy::KeepSome(5)`.
- Tauri's `LogDir` target owns the physical path. On Windows this resolves under the application's OS-managed data/log directory; code must not hardcode a user profile or installation path.
- QA structured events: `qa::trace::emit(&QaTraceEvent)` with schema `qa-trace-v1` and log target `qa_trace`.
- Development Real Answer E2E additionally writes the same event projection to `apps/desktop/logs/qa-real-e2e.jsonl`. The directory is Git-ignored and the file is truncated after 5 MiB before appending new JSON lines.

## 3. Contracts

- Every event contains `schemaVersion`, Unix-millisecond timestamp, stable `event`, `stage`, and `status`.
- Optional fields are typed and allowlisted: hashed request identity, public case ID, execution mode, provider/model identity, aggregate evidence/claim counts, persisted state, and stable error code.
- Request IDs are SHA-256 hashed and shortened before logging. Error messages are reduced to the stable prefix before the first colon.
- QA lifecycle event families are `qa_prepare_*`, `qa_generate_*`, `qa_semantic_*`, `qa_audit_*`, `qa_persist_*`, and `qa_e2e_*`.
- `info` records normal lifecycle boundaries and measured degraded states. `warn` records logger serialization/lock/write failures and recoverable startup/deployment issues. `error` is reserved for process-level failures that cannot be represented through an existing result/error contract. Debug/trace must not carry content payloads.
- Logger write failure is fail-soft for production QA. E2E startup may fail closed when its required local trace directory cannot be configured, because the runner explicitly promises a trace artifact.

## 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| Event serialization fails | Emit only `event=qa_trace_serialize_failed`; continue the product operation |
| Desktop AppLogDir is unavailable | Tauri plugin initialization reports startup failure; never fall back to the repository or an arbitrary absolute path |
| E2E trace directory cannot be created | Stable `QA_TRACE_DIRECTORY_CREATE_FAILED`; do not run an untraceable diagnostic |
| E2E JSONL reaches 5 MiB | Truncate the local development trace, then append the next complete JSON line |
| Provider/semantic/persistence fails | Log stage/status/stable code and aggregate counts only; preserve the original caller-visible error |
| A proposed field contains content or a path | Do not add it to `QaTraceEvent`; use a hash, count, enum, or stable code instead |

## 5. Good / Base / Bad Cases

- Good: `qa_persist_failed` records request hash, mode, evidence count, `persisted=false`, and `citation_validation_failed` without answer text.
- Base: semantic verification is unavailable; `qa_semantic_completed` records `status=unavailable` and `errorCode=llm_budget_exceeded` while QA applies its existing fallback.
- Bad: log a question, answer, prompt, claim text, evidence snippet, repository/temp path, credential, token, provider payload, chain-of-thought, or raw error message.

## 6. Tests Required

- Unit tests assert request hashing, stable error-code projection, allowed JSON keys, one-record-per-line append behavior, and bounded desktop rotation configuration.
- Real E2E verification independently parses every JSONL line, rejects keys outside the allowlist, and confirms the log path is Git-ignored.
- Report/log redaction checks reject absolute Windows/UNC paths and raw content fields.

## 7. Wrong vs Correct

### Wrong

```text
INFO question=<full text> answer=<full text> error=<provider body>
```

### Correct

```json
{"schemaVersion":"qa-trace-v1","event":"qa_persist_failed","stage":"persistence","status":"failed","requestIdHash":"16-hex","evidenceCount":2,"persisted":false,"errorCode":"citation_validation_failed"}
```
