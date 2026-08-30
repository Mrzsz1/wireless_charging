# P1-3F Codex Planner Provider Exit Diagnosis

## Goal

Continue P1-3 by preserving and precisely classifying fatal Codex `exec --json` terminal events, then use isolated Probe A/B/C calls to identify exactly one evidence-backed Planner failure layer. The public Research case must ultimately use a real, valid RetrievalContract without weakening Semantic, Grounding, persistence, routing budgets, or E2E gates.

## Requirements

### R1 — Baseline and sequence

- Start from clean `4c551e8e2d75425e33271248ffd4d71f16a673ae` or later commits belonging only to P1-3F.
- Execute the taskbook phases in order. A failed phase stops later real-provider phases.
- Do not enter P1-4 while P1-3 remains incomplete.

### R2 — JSONL terminal semantics

- The shared Codex subprocess adapter must distinguish ordinary activity, model metadata, agent deltas/completion, `turn.completed`, fatal `turn.failed`, fatal top-level `error`, and non-fatal `item.type=error` warnings.
- Fatal observations retain only event type, fixed category, message SHA-256, and exit code in safe errors/telemetry. Raw messages never enter ordinary logs, manifests, reports, or Git.
- Fatal JSONL stops the process promptly and cannot continue refreshing idle timeout.
- Success requires exit code 0, no fatal event, and a non-empty final agent message. An item warning alone must not override `turn.completed` plus a successful exit.

### R3 — Precise categories

- `classify_codex_terminal_message` returns only fixed categories covering schema/request/context/auth/usage/rate/provider transport/model/cancellation/unknown failures.
- Error precedence is cancellation, local idle timeout, local total timeout, fatal stdout JSONL, safely classified stderr, then generic exit code.
- Generic `provider_exit` remains only when no fatal JSONL or classifiable stderr reason exists.

### R4 — Tests before behavior change

- J1–J7 are deterministic and include an old-code RED proving `turn.failed` was collapsed to `CODEX_EXIT_ERROR`.
- Existing Generator, Direct, Semantic, and Codex adapter behavior must not regress.
- No Planner Schema/input/timeout/budget change is permitted without a later unique-branch RED test.

### R5 — Development-only diagnostics and probes

- Optional `QA_CODEX_EXEC_DIAGNOSTIC_DIR` accepts only an absolute directory outside the repository, defaults off, and is enabled only for Development E2E/probes.
- Raw stdout/stderr diagnostic artifacts are temporary and deleted after inspection; Git retains only safe categories, event types, exit codes, and hashes.
- Run Probe A once. Run B only if A passes. Run C only if B passes. Use the same adapter, `gpt-5.6-luna`, `low`, and isolated workspace. At most three probe calls.
- Select exactly one Schema/Input/Provider/Generic-exit/Integration branch from the measured matrix.

### R6 — Guardrails

- Do not run/read Independent Heldout, alter frozen data/thresholds, or continue partial heldout runs.
- Do not change the public Research question, Planner success semantics, E2E fallback gate, 3/4/5 call budgets, 8,000/18,000/32,000 token ceilings, Direct schema, Semantic/Generator/Grounding/Final Audit, Retrieval/Reranker/Embedding, or unrelated UI.
- Do not add speculative retries, 300-second timeouts, force pushes, or raw Provider data to Git.

## Acceptance Criteria

- [ ] J1–J7 pass and prove raw fatal messages are redacted.
- [ ] `turn.failed` and top-level `error` can no longer collapse into generic `provider_exit`.
- [ ] `item.type=error` remains non-fatal when the turn completes successfully.
- [ ] Probe A/B/C are run only as allowed and produce safe reports.
- [ ] Exactly one root-cause branch is selected from measured evidence.
- [ ] A branch-specific old-code RED exists before any Planner Schema/input/timeout/budget/integration change.
- [ ] Required deterministic Rust/Python/frontend/build gates pass, aside from documented pre-existing issues.
- [ ] One final real Research E2E is run only after deterministic gates.
- [ ] Final status is reported strictly as PASS, PARTIAL-BLOCKED, or FAIL.

## Notes

- Taskbook: `C:/Users/qq155/Downloads/p1_3f_codex_planner_provider_exit_taskbook.md`.
- Baseline executable metadata: `codex.ps1`, external script / npm wrapper, Codex CLI `0.146.0`, SHA-256 `0c149db80ed0bf442c810146b0ad0163b74982fe4542d673f56c354d7b8229cb`.
- Runtime: Windows 11 Pro `10.0.22631`; model/effort `gpt-5.6-luna` / `low`.
