# Design — Held-out Coverage Metrics Hardening

## Boundary

The frozen case owns expected coverage. The system run owns answer claims and evidence. Review artifacts bridge the two but may not redefine either expected set.

## Contracts

1. `answerClaims.dimension` remains optional compatibility metadata with default `factual`. It is used only for claim-category diagnostics such as reference support; it never feeds method recall or constraint preservation.
2. Each primary review contains exact one-to-one arrays:
   - `method_coverage`: `{method_family, verdict}` where verdict is `covered | not_covered`.
   - `constraint_coverage`: `{constraint, verdict}` where verdict is `preserved | not_preserved`.
3. Expected keys come directly from the frozen case arrays. Empty expected arrays require empty reviewer arrays and contribute zero denominator.
4. Adjudication uses the same three independent channels: `claims`, `method_coverage`, and `constraint_coverage`. It is required when any channel disagrees and contains exactly the union of disputed entries, no agreed entries.
5. `review_totals` accepts the frozen case object and computes method/constraint numerators from final coverage verdicts and denominators from frozen array lengths.

## Freeze provenance

`freeze_draft` loads a candidate pool path (default `evals/research_questions_v1.json`), verifies schema/status/count/splits and canonical `casesSha256`, extracts exactly 80 `split=heldout` candidates, then validates each curated case's ID, question and type against the candidate's `id`, `question` and `intent`. The frozen dataset records candidate source, pool hash and heldout count.

## Compatibility

- Public pending dataset remains valid with `cases=[]`.
- Existing run bundles without `answerClaims.dimension` remain valid and default to factual.
- Frozen review artifacts must follow the new coverage contract; old review artifacts fail closed instead of silently producing inflated metrics.

## Failure model

All malformed, incomplete, duplicate, unknown, mismatched or unadjudicated coverage data raises a stable evaluator/workflow error before metrics are emitted.
