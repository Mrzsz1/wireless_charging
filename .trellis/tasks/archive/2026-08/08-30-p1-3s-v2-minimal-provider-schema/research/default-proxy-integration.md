# Default Codex Proxy Integration

Precondition: temporary-proxy Probe B, Probe C, and `real-research-improvement` all passed before this production change.

## Contract

The Codex child-process resolver applies this priority:

1. `WIRELESS_CODEX_PROXY_URL` non-empty override.
2. Existing non-empty uppercase/lowercase HTTP, HTTPS, or ALL proxy environment.
3. Default `http://127.0.0.1:7890`.

`WIRELESS_CODEX_PROXY_URL=off|direct|none` removes proxy variables and sets child-only `NO_PROXY=*`. All changes are applied through `Command::env`, `env_remove`, or inherited command state. No parent-process `set_var` is used.

The configurator is applied to Codex version/login status, interactive login, and `codex exec` child commands. It is not applied to unrelated subprocesses such as registry discovery.

## Deterministic verification

- Explicit override wins over an inherited proxy — PASS.
- Existing standard proxy remains inherited without overwrite — PASS.
- Empty configuration injects localhost:7890 — PASS.
- `off`, `direct`, and `none` select child-only direct mode — PASS.
- Full `codex_subscription::tests` — 23 passed, 0 failed.
- `cargo fmt --all -- --check` — PASS.
- Focused library / Probe / real-E2E Clippy with `-D warnings` — PASS.

No proxy URL, credentials, prompt, answer, or evidence is added to logs or reports. Existing Provider lifecycle events and fixed failure categories remain the diagnostic boundary.
