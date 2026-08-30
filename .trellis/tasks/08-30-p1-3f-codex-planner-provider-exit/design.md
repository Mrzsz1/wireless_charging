# Design — P1-3F Codex Planner Provider Exit Diagnosis

## Shared adapter event model

`codex_subscription::stream_answer_with` remains the single subprocess adapter. JSONL parsing returns typed internal observations instead of only optional text:

```text
JSONL line
  -> typed observation
     -> activity/model/agent content
     -> turn completed
     -> item warning (non-fatal)
     -> terminal failure (fatal)
  -> subprocess lifecycle + fixed safe error
```

A `CodexTerminalFailure` holds only `event_type`, fixed `category`, and message SHA-256. Raw text may exist only in the explicitly enabled repository-external diagnostic artifact and is deleted after probe inspection.

## Failure precedence

The adapter owns one deterministic precedence: cancellation; local idle timeout; local total timeout; fatal stdout JSONL; classifiable stderr; generic exit. Fatal stdout is handled immediately, terminates the process tree, joins readers, and returns a stable `CODEX_JSONL_TURN_FAILED` or `CODEX_JSONL_ERROR` code.

## Probe isolation

A development-only probe binary/helper reuses `stream_answer`, the selected model/effort, structured-output transport, and temporary workspace:

- A: tiny Boolean schema and minimal prompt.
- B: current RetrievalContract schema and minimal valid input.
- C: current real Planner prompt/public Development input, only if B passes.

Each result contains only status, terminal event type, fixed category, exit code, latency, safe counts/hashes, executable source type/version, and no raw prompt/output/path.

## Branch isolation

- A fail: base CLI/login/model/Provider/structured-output environment; no Planner change.
- A pass/B fail: schema branch only.
- A+B pass/C fail: Planner input branch only when numeric evidence proves abnormal size.
- A+B+C pass/E2E fail: integration branch only.
- Precise auth/usage/rate/overload/transport: external Provider block, no Planner change.
- Generic exit: collect the required safe process/event aggregates; no retry or unrelated change.

## Compatibility

Final-answer generation and other structured callers use the same improved fatal-event semantics. Public outputs, `qa-run-v22`, Report v5, routing budgets, Planner contract strictness, and all answer safety gates stay unchanged.
