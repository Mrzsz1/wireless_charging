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

(To be filled by the team)

## Contextual Mixed Retrieval Contract

- Tauri research-trail DTOs use `#[serde(rename_all = "camelCase")]` and preserve Wiki, book physical-page, and Graphify location fields end to end.
- Question/search retrieval reuses `qa::prepare_question` and its shared term/FTS helpers; do not copy the bilingual expansion table.
- Page retrieval ranks explicit outgoing links and backlinks ahead of FTS/graph hints, excludes the anchor, merges reasons across duplicate identities, uses deterministic tie-breaks, and preserves Wiki/book diversity when available.
- Related-method output is restricted to `page_type = 'method'`; it must never fall back to catalog order.
- Missing Graphify or core-book indexes are reported through `degradedChannels`; a missing optional channel does not turn valid Wiki evidence into an error.
- Rust tests must cover stable context keys, deduplication/reason merging, direct-link priority, method restriction, and degraded channels. Run fmt, Clippy with `-D warnings`, and the complete Rust suite before commit.
