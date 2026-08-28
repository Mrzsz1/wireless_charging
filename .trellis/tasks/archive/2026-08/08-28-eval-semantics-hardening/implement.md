# Implementation Plan

1. Metric core and schema v4
   - Add the single work identity helper and shared relevance-view metrics.
   - Add work/exact per-case Option metrics and eligibility flags.
   - Add aggregate denominators and zero-evidence confusion metrics.
   - Reject ambiguous empty expectedDocuments.
   - Add dataset fingerprint and report v4 identity fields.

2. Reporting and release integration
   - Update JSON/Markdown renderers and MRR diagnostics naming.
   - Keep legacy aliases explicitly mapped to work metrics.
   - Update production collector/release gate mappings.
   - Preserve frozen threshold numbers exactly.

3. Regression tests
   - Zero-evidence denominator exclusion.
   - Wiki/Paper same-work match versus exact miss.
   - Returned and expected duplicate work collapse.
   - Shared relevance-set consistency for Recall/MRR/nDCG.
   - Ambiguous fixture rejection, report fingerprint, JSON/Markdown consistency and threshold invariance.

4. Verification and baseline
   - Run focused Rust/Python tests, fmt/check/clippy.
   - Run the existing development/regression RAG suite only.
   - Generate JSON and Markdown from one report object, record corrected metrics.
   - Confirm no production Retrieval source behavior changed.
   - Commit locally by phase.

## Completion Record

- Metric/report schema implementation: complete.
- Release collector and frozen threshold field migration: complete; values remain 0.95/0.90/0.85/0.85.
- Focused Rust tests: 13 passed (9 evaluation + 4 metric); broader `qa::` suite: 197 passed, 2 ignored.
- Python release-gate tests: 8 passed.
- `cargo fmt --check` and `cargo clippy --lib -- -D warnings`: passed.
- Development/regression RAG run: PASS, 13/13; ranking eligible 12, zero-evidence 1.
- Corrected work metrics: Recall@5/10/20 1.000/1.000/1.000, MRR 0.958333, nDCG@10 0.969244.
- Production QA behavior: unchanged; only evaluation/reporting, tests, docs, and release metric mapping changed.
