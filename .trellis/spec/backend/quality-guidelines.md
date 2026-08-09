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
