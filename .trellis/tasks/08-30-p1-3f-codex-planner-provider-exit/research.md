# P1-3F Diagnosis Record

## Phase 0 baseline

- Baseline commit: `4c551e8e2d75425e33271248ffd4d71f16a673ae`.
- Codex executable: `codex.ps1`; external script / npm-wrapper source type.
- Codex CLI: `0.146.0`; login status ready.
- Executable SHA-256: `0c149db80ed0bf442c810146b0ad0163b74982fe4542d673f56c354d7b8229cb`.
- Windows: Windows 11 Pro `10.0.22631`.
- Probe/E2E model and effort: `gpt-5.6-luna` / `low`.

## J1 old-code RED

- Test: `codex_subscription::tests::j1_turn_failed_schema_event_is_preserved_and_redacted`.
- Fixture terminal event: `turn.failed` with a schema-class message, process exit code 1.
- Old observed error: `CODEX_EXIT_ERROR: Codex CLI 退出码 1`.
- Required error: `CODEX_JSONL_TURN_FAILED: schema_rejected`.
- Root function/condition: `codex_subscription::stream_answer_with` refreshed activity for any valid JSON, ignored `turn.failed`, and classified the final non-zero status only from stderr schema text or exit code.
- No real Provider call was made.

## JSONL adapter result

- `turn.failed` and top-level `error` are fatal typed observations.
- `item.completed` with `item.type=error` is a non-fatal warning and does not override a completed successful turn.
- Fatal events terminate the child process tree immediately and do not refresh idle timeout.
- Stable terminal categories and message SHA-256 are computed without returning raw messages.
- Planner fallback projection recognizes the new safe JSONL/stderr codes instead of collapsing them to `unknown`.
- J1–J7, the full `codex_subscription` module (18 tests), Provider Capabilities (4 tests), fmt, and library/binary Clippy pass.
- No real Provider call was made in this phase.
