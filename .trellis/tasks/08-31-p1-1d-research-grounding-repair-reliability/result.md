# P1-1D Final Result

## Status

**PARTIAL-BLOCKED** — all deterministic implementation and quality gates pass. The one permitted real Research run measured the first source-span implementation and exposed a second deterministic notice-boundary defect. That defect now has its own RED fixture and deterministic fix, but the taskbook forbids another live run, so the post-fix real close gate remains unmeasured.

## Commits

1. Baseline: `6ad4d767a656038aeab458e96be29fa9edbbf61f`.
2. Task plan / previous journal correction: `d519909`.
3. Legacy `replacen` RED: `ed5575e`.
4. Source-span reconstruction, invariants, diagnostics, logging, and R1-R12: `9f6b038`.
5. Notice-boundary RED after the single real run: `5f4eee9`.
6. Notice-aware factual projection, full failed-operation diagnostics, connector matrix, and spec: `bbce464`.
7. Single real Research report: `7d9377f` (the measured code under test was `9f6b038`).

## Modified Files

- `apps/desktop/src-tauri/src/qa/claim_verification.rs`
- `apps/desktop/src-tauri/src/qa.rs`
- `apps/desktop/src-tauri/src/qa/context.rs`
- `apps/desktop/src-tauri/src/qa/real_e2e.rs`
- `apps/desktop/src/types.ts`
- `.trellis/spec/backend/qa-contract.md`
- `evals/reports/qa-real-generator-e2e-report.json`
- This task's Trellis artifacts and the corrected previous journal entry.

## Root Cause and Repair

1. The legacy RED answer contained a supported long claim followed by an unsupported shorter claim whose text also appeared inside the long claim. `String::replacen(short, notice, 1)` replaced the substring inside the earlier supported claim and left the actual unsupported occurrence untouched.
2. Global claim repair no longer uses `replacen`. Draft claims are located from a monotonic cursor against the immutable source and bound to validated UTF-8 byte spans. Every span must be ordered, non-empty, non-overlapping, and byte-equal to its claim text.
3. The repaired answer is rebuilt once from untouched gaps plus preserved Supported/NotApplicable spans or the unchanged existing status notice. Duplicate text consumes successive source occurrences; substring collisions cannot target an earlier bound span.
4. Missing/invalid spans fail closed with `claim_span_not_found` / `claim_span_invalid`; there is no fallback to string replacement.
5. Immediate re-extraction requires every factual result to consume an exact Supported Draft key. `introduced_factual_claim` and `supported_claim_lost` stop the projection before Final Audit.
6. The live run showed that an inline fixed notice could share one final atomic segment with a neighboring supported clause. A second RED (`supported_left_clause_survives_a_repaired_right_clause`) reproduced this deterministically. Extraction now splits around exact backend notices before factual classification and trims only dangling connector commas, preserving the supported fragment's exact mapping.
7. Failed invariant audits retain safe attempted operation count/details and replacement count: Claim ID, byte span, source hash, replacement kind, body hashes, status, and stable error code; no answer or claim text is persisted in ordinary reports/logs.

## R1-R12

| Case | Result |
|---|---|
| R1 duplicate occurrence | PASS |
| R2 substring collision | RED on legacy code, PASS on source-span code |
| R3 same prefix / different evidence | PASS |
| R4 Markdown heading/list collision | PASS |
| R5 Draft 4 (`1` supported, `3` rejected) -> Final `1/1/0` | PASS |
| R6 missing span | PASS, fail closed |
| R7 overlap / invalid UTF-8 boundary | PASS, fail closed |
| R8 no supported claim | PASS, fixed no-supported notice |
| R9 notices excluded from factual denominator | PASS |
| R10 trusted history reads only fully supported visible final audit | PASS |
| R11 visible projection remains enforced | PASS |
| R12 unknown Evidence ID remains rejected | PASS |

Additional connector matrix: all 15 Chinese/English atomic connectors pass with the supported claim on either side of a repaired neighbor.

## Preserved Gates

- Final Grounding Audit: **YES, unchanged as the second gate**.
- Visible Projection Audit: **YES, unchanged as the third gate**.
- Semantic verdict/prompt: **NO modification**.
- Planner / Provider Schema / default 7890 proxy: **NO modification**.
- Retrieval / generator / budgets / citation thresholds: **NO modification**.
- Independent Heldout: **NOT RUN**; only its deterministic unit-test module ran.

## Deterministic Quality

- `cargo fmt --check`: PASS.
- `cargo clippy --lib --bins -- -D warnings`: PASS.
- `cargo test --lib claim_verification`: 39 PASS.
- `cargo test --lib production_core`: 3 PASS.
- `cargo test --lib real_e2e`: 19 PASS.
- `cargo test --lib heldout_runner`: 12 PASS (unit tests only).
- `cargo test --lib natural_answer`: 4 PASS.
- `py -3 -m unittest tests.test_qa_accuracy_eval`: 24 PASS.
- `npm run test:qa-evidence`: 6 PASS.
- `npm run build`: PASS.
- `git diff --check`: PASS.

## Single Real Research Result

Command executed exactly once with Shell proxy variables removed and project default proxy behavior intact. Exit code: `2`.

| Metric | Observed |
|---|---|
| plannerStatus / used / fallback | `succeeded / true / false` |
| semanticStatus | `succeeded` |
| draft claims | `6` |
| draft supported / partial / contradicted / not-verifiable | `1 / 0 / 1 / 4` |
| measured repairProjection | `failed: introduced_factual_claim` |
| measured final factual / supported / unsupported | `0 / 0 / 0` (fail-closed output) |
| citationCoverage | `0.0` |
| visibleProjectionValid | `false` |
| persisted | `false` |
| executedScopePassed | `false` |

The measured failure correctly remained blocked; Final Grounding and persistence were not weakened. The deterministic notice-boundary RED and fix were added afterward without a second live invocation.
