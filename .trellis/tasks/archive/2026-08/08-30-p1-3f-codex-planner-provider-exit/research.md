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

## Probe infrastructure

- Added `qa-planner-probe` with one probe per invocation and non-overwritable atomic safe reports.
- Probe A uses the tiny Boolean schema; Probe B uses the current RetrievalContract schema and one bounded candidate; Probe C deterministically captures the real public Research Planner input without a Provider call before invoking the shared adapter.
- `QA_CODEX_EXEC_DIAGNOSTIC_DIR` is default-off, absolute-only, and rejected inside the repository. Raw stdout/stderr live only under the explicit external probe directory; committed reports contain hashes/counts/enums only.
- Probe lifecycle uses `qa_planner_probe_started/completed/failed` with one hashed operation ID and safe aggregate fields.
- Probe infrastructure tests (3), Codex Subscription tests (19), binary compilation, fmt, and library/binary Clippy pass without a real Provider call.

## Probe A result and branch decision

- Probe A was invoked exactly once with the tiny Boolean schema and minimal prompt.
- Safe report: `evals/reports/qa-codex-planner-probe-a.json`; SHA-256 `3cf339db28802d9b9bd02b3f27be37a2d150f0a3f22b341fa4779d5702397eab`.
- Measured status: failed; terminal event type `error`; exit code `-1` because the adapter terminated immediately after the fatal event; latency 47,830 ms; three JSONL events; no agent message or completed turn; stderr was empty at the fatal boundary.
- The initial classifier returned `unknown`. The repository-external raw artifact was inspected once, reduced to the stable category `transport`, and then deleted. No raw message, JSONL, stderr, prompt, or external path is retained in Git.
- Old-code RED: a synthetic request-timeout terminal message classified as `unknown`; after the adapter-only classifier patch it is `transport`.
- Probe A failure means the base CLI/model/Provider structured-output control path is externally blocked. Probe B and Probe C were not run, as required by the prerequisite matrix. Total real probe calls: one.
- Unique branch: Provider external `transport` (taskbook branch C). Schema, Planner input, timeout, budget, parser, Retrieval, and integration branches are excluded because Probe A did not pass.
- Codex CLI selected by the application probe was the desktop binary, version `0.151.0-alpha.7.1`; this differs from the shell npm wrapper `0.146.0` recorded at baseline.
- Status: `PARTIAL-BLOCKED-BY-PROVIDER`. No Planner production patch or final Research E2E is permitted after the failed prerequisite.
