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
