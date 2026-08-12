# Quality Guidelines

> Code quality standards for backend development.

---

## Overview

<!--
Document your project's quality standards here.

Questions to answer:
- What patterns are forbidden?
- What linting rules do you enforce?
- What are your testing requirements?
- What code review standards apply?
-->

(To be filled by the team)

---

## Forbidden Patterns

<!-- Patterns that should never be used and why -->

(To be filled by the team)

---

## Required Patterns

<!-- Patterns that must always be used -->

(To be filled by the team)

---

## Testing Requirements

<!-- What level of testing is expected -->

(To be filled by the team)

---

## Code Review Checklist

<!-- What reviewers should check -->

## Scenario: Windows Codex CLI discovery for GUI processes

### 1. Scope / Trigger

This contract applies whenever the Tauri backend detects, launches, or streams from the Codex CLI on Windows. GUI processes may inherit a stale or reduced `PATH`, and npm installs may expose only `.cmd`/`.bat` shims.

### 2. Signatures

- `explicit_executable() -> Option<String>`
- `discovered_executables() -> Vec<PathBuf>`
- `append_windows_path_candidates(candidates, seen, path, native_only)`
- `append_codex_desktop_binaries(candidates, seen, local_app_data)`
- `executable() -> String`
- Optional environment overrides: `CODEX_CLI_PATH`, then legacy `WIRELESS_CODEX_BIN`.

### 3. Contracts

- An explicit non-empty override is authoritative.
- Automatic discovery prefers native Codex desktop binaries under `%LOCALAPPDATA%\OpenAI\Codex\bin\*\codex.exe`, then native PATH candidates, then script shims.
- Automatic discovery reads both the current process PATH and persisted HKCU/HKLM PATH values so installing Codex does not require a Windows sign-out before the app can refresh.
- Candidate paths are file-checked, case-insensitively deduplicated on Windows, and accepted only when `--version` returns an allowlisted `codex-cli ...` line.
- Status, login, and answer streaming must use the same resolver. The public status DTO must never expose token, cookie, credential path, or API key fields.

### 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| `CODEX_CLI_PATH` is non-empty | Use it without replacing it with auto-discovery |
| Process PATH lacks Codex but desktop binary exists | Detect the desktop binary and report its version |
| PATH contains duplicate entries | Probe each normalized path at most once |
| Candidate exists but `--version` fails or is unsafe | Continue to the next automatic candidate |
| No candidate succeeds | Fall back to `codex` and return the existing unavailable diagnostic |
| Version succeeds but `login status` fails | Report installed=true, authenticated=false, and a login-status diagnostic |

### 5. Good/Base/Bad Cases

- **Good**: Explorer launches the installed client with an old PATH; `%LOCALAPPDATA%\OpenAI\Codex\bin\<release>\codex.exe` is found and ChatGPT login status is reported.
- **Base**: A terminal-launched client resolves `codex.exe` or `codex.cmd` from PATH and behaves as before.
- **Bad**: Call only `Command::new("codex")`; a valid CLI then appears as “未安装” because the GUI process cannot resolve the command name.

### 6. Tests Required

- Rust tests create a spaced local-app-data path and custom Node directory, then assert desktop binary discovery, shim discovery, ordering, and deduplication.
- The Windows fixture exercises `--version`, `login status`, interactive login, JSONL streaming, failure redaction, timeout, and cancellation.
- Structural frontend tests assert the status DTO remains secret-free and all Windows candidate types remain wired.

### 7. Wrong vs Correct

#### Wrong

```rust
Command::new("codex").arg("--version").spawn()?;
```

#### Correct

```rust
let executable = executable();
run_fixed_with(&executable, &["--version"], STATUS_TIMEOUT)?;
```

## Desktop Process and Search Reliability Contract

The Windows desktop backend has three non-negotiable runtime contracts:

1. Long-running work invoked by a Tauri command must not run directly on the async command executor. Move blocking filesystem, SQLite, network, or child-process work into `tauri::async_runtime::spawn_blocking`, and never hold a Tauri state mutex guard across `.await`.
2. Every Windows child process that is internal to the application must use the shared background-process configurator. Python commands must additionally set `PYTHONUTF8=1` and `PYTHONIOENCODING=utf-8`; direct Python entry points that emit JSON must reconfigure stdout/stderr to UTF-8 as a second line of defense.
3. SQLite FTS5 auxiliary functions must be verified by executing a real in-memory FTS query. In particular, `snippet` always uses all six arguments: table, column, start marker, end marker, ellipsis, and token count.

Do not duplicate Windows creation flags or Python encoding variables at individual call sites. Reuse `apps/desktop/src-tauri/src/process_support.rs` so capability checks, candidate operations, compile tasks, cancellation helpers, and future background commands remain consistent.

Regression coverage for changes in these areas must include:

- a real FTS prefix query with visible highlight markers;
- a Unicode query and a normal empty-result path;
- Python JSON output containing non-ASCII text while the parent environment requests a legacy Windows code page;
- assertions that the shared Python command configurator installs both UTF-8 environment variables;
- the full Rust suite, Clippy with `-D warnings`, frontend build, strict GUI search smoke, and strict installer lifecycle smoke before release.

Reviewers must reject synchronous `Command::output`/`wait` calls in Tauri command handlers, four-argument FTS5 `snippet` calls, and Windows Python launches that bypass the shared configurator.

(To be filled by the team)

## Contextual Mixed Retrieval Contract

- Tauri research-trail DTOs use `#[serde(rename_all = "camelCase")]` and preserve Wiki, book physical-page, and Graphify location fields end to end.
- Question/search retrieval reuses `qa::prepare_question` and its shared term/FTS helpers; do not copy the bilingual expansion table.
- Page retrieval ranks explicit outgoing links and backlinks ahead of FTS/graph hints, excludes the anchor, merges reasons across duplicate identities, uses deterministic tie-breaks, and preserves Wiki/book diversity when available.
- Related-method output is restricted to `page_type = 'method'`; it must never fall back to catalog order.
- Missing Graphify or core-book indexes are reported through `degradedChannels`; a missing optional channel does not turn valid Wiki evidence into an error.
- Rust tests must cover stable context keys, deduplication/reason merging, direct-link priority, method restriction, and degraded channels. Run fmt, Clippy with `-D warnings`, and the complete Rust suite before commit.

## Scenario: Auditable Wiki and Primary-Paper Evidence Pairing

### 1. Scope / Trigger

Apply this contract whenever `qa::prepare_question` ranking, bilingual expansion, paper-section indexing, or evidence limits change. It prevents a strong paper boost from evicting the Wiki source that explains the claim, while retaining the independent core-book channel.

### 2. Signatures

- `query_terms(question: &str) -> Vec<String>`
- `linked_paper_candidates(connection: &Connection, wiki: &[Candidate]) -> Result<Vec<Candidate>, String>`
- `prepare_question(connection, root, question, limit) -> Result<QuestionContext, String>`
- Paired paper marker: `relation = "wiki_source_to_primary"`.

### 3. Contracts

- Put bilingual domain expansions before unsplit Chinese clauses and deduplicate before applying the term cap.
- When a recalled Wiki candidate has `page_type = "source"`, down-drill to one non-reference canonical paper section with a real line range.
- A paired paper exposes the same `page_id`, `kind = "paper"`, `tier = "primary_source"`, and `source_location` containing `原文第 x–y 行`.
- Final evidence preserves at least one Wiki, paper, and book result when each channel produced a candidate. Adding Wiki/paper pairs may remove duplicate-channel or graph candidates, never the sole book result.

### 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| Wiki source has an indexed canonical section | Return the Wiki item and its paired paper when the evidence limit permits |
| Wiki source has no paper section | Keep the Wiki result; do not fabricate a location |
| Direct paper FTS misses a Chinese question | Use the matched Wiki source to down-drill its canonical section |
| Pair insertion reaches the limit | Evict graph or a duplicate-channel item; preserve sole Wiki/paper/book representatives |
| Section is References/Acknowledgements | Exclude it from pair selection |

### 5. Good/Base/Bad Cases

- **Good**: a Chinese scheduling query returns `[[src-...]]`, the corresponding canonical section with raw line range, and a core-book chapter with physical pages.
- **Base**: only Wiki evidence exists; the answer remains usable and clearly lacks primary-paper location.
- **Bad**: sorting all candidates and truncating once, which can return many boosted paper sections but lose the Wiki provenance or sole book result.

### 6. Tests Required

- Real-repository Gold Contract regression: all ten cases recall an expected Wiki page and an allowed paper source with `原文第` and `行`.
- Real-repository mixed-channel regression: a generic scheduling query retains Wiki and book evidence; every book result has a physical page.
- Run Rust fmt, Clippy `-D warnings`, complete Rust tests, frontend build, and release GUI E2E before packaging.

### 7. Wrong vs Correct

#### Wrong

```rust
candidates.sort_by(score);
let evidence = candidates.into_iter().take(limit).collect();
```

#### Correct

```rust
let wiki = wiki_candidates(connection, &terms)?;
candidates.extend(linked_paper_candidates(connection, &wiki)?);
// Rank, deduplicate, preserve channel diversity, then preserve Wiki/paper pairs.
```

## Desktop Search Credential Contract

Search-provider credentials are desktop secrets, not repository configuration.

- Store them in the current Windows user's Credential Manager under one fixed application service name and a provider allowlist. Never persist them in SQLite, Wiki files, manifests, task arguments, logs, error strings, local storage, or Git.
- Tauri status DTOs expose provider metadata and a `configured` boolean only. A saved credential is never returned to the WebView, even in masked form.
- Blank input is a no-op. Deletion is an explicit command. Unknown provider IDs are rejected before any credential-store or network operation.
- Read credentials only immediately before an approved discovery/literature child process starts, and inject only configured provider variables into that child environment. Tasks outside the search/literature allowlist receive no credential variables.
- Credential-manager and connection-test failures use generic messages. Never include credential-bearing request URLs, headers, response bodies, or secret values in errors.
- Preserve the existing environment-variable and external-key-file behavior by omitting missing vault values instead of injecting empty environment variables.

Regression tests must use an in-memory credential store and assert that status DTOs contain no secret field, only configured variables are projected, and unknown providers fail closed. Do not save test keys into the developer's real Windows Credential Manager.

## Scenario: Codex Subscription Answer Process

### 1. Scope / Trigger

This contract applies when a desktop answer provider reuses the user's Codex CLI ChatGPT login instead of a conventional API key. It prevents authentication leakage, project-context inheritance, writable agent execution, orphan processes, and subscription-consuming tests.

### 2. Signatures

- `get_codex_subscription_status() -> CodexSubscriptionStatus`
- `start_codex_login() -> Result<String, String>`
- `get_qa_settings() -> QaSettings`
- `save_qa_settings(settings: QaSettings) -> QaSettings`
- `stream_answer(prompt, model, timeout, cancelled, on_token) -> Result<(answer, model), String>`
- `CodexSubscriptionStatus = { installed, version, authenticated, ready, statusLabel, diagnostic }`
- `QaSettings.answerProvider = "codex-subscription" | "compatible-api" | "offline-evidence"`

### 3. Contracts

- Status executes only `codex --version` and `codex login status`; authentication is true only for a successful status containing both `logged in` and `chatgpt`.
- The WebView receives no token, cookie, credential path, API key, organization identifier, or raw authentication payload.
- Answer execution uses `codex -a never exec --json --ephemeral --skip-git-repo-check --ignore-user-config --ignore-rules --sandbox read-only --cd <empty-temp> -`. The prompt is stdin-only.
- The evidence prompt comes from the existing local retrieval pipeline and preserves `[E#]`, waterline, and Graphify limitation language.
- Windows child processes use `configure_background_command`; cancellation, timeout, stdin/output failure, and stream-callback failure terminate the process tree. A temporary workspace is RAII-cleaned.
- Settings keys are repository-scoped. API-key values remain runtime-only and are never persisted.

### 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| CLI missing/version command fails | `ready=false` with a generic install/check diagnostic |
| Logged in by a mechanism other than ChatGPT | Do not classify it as subscription-ready |
| Unknown provider or invalid model/settings value | Reject before execution/persistence |
| Malformed/reasoning/tool JSONL event | Ignore it; never expose reasoning or tool payloads |
| Non-zero exit | Return `CODEX_EXIT_ERROR` without stderr/auth payload |
| Timeout/cancel | Kill the process tree, clean temp files, and do not save a partial assistant message |
| Codex unavailable | Preserve the deterministic offline evidence path and safe status summary |

### 5. Good/Base/Bad Cases

- **Good**: Local retrieval returns numbered evidence, a fake Codex emits delta/final JSONL, and the stored message records `provider=codex-subscription` with auditable citations.
- **Base**: Codex is absent or not logged in; settings show the exact readiness state and the question still has an offline evidence path.
- **Bad**: Passing the prompt as a command argument, inheriting repository rules/hooks, returning stderr verbatim, or relying on dropping `Child` to stop a process.

### 6. Tests Required

- Pure status DTO and version allowlist tests assert that no secret-shaped field crosses IPC.
- A fake Windows Codex executable covers version, ChatGPT login status, login launch, delta/final JSONL, non-zero stderr, hang/timeout, and cancellation without contacting a service.
- Command-shape tests assert read-only/ephemeral/never/isolation flags and stdin `-`.
- Run Rust fmt, Clippy `-D warnings`, the complete Rust suite, frontend provider tests, strict GUI, and isolated installer lifecycle before release.

### 7. Wrong vs Correct

#### Wrong

```rust
Command::new("codex").args(["exec", prompt]).output()?;
```

#### Correct

```rust
let mut child = Command::new("codex")
    .args(build_exec_args(&empty_workspace, model))
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;
child.stdin.take().unwrap().write_all(prompt.as_bytes())?;
```
