# Technical Design

## Contract

Extend `StructuredClaim` with optional `role`. The prompt requires explicit roles and emits an intent-specific JSON role contract. Keep `qa-structured-answer-v1` for additive compatibility and bump the prompt version.

Backend owns stable role IDs and display names. Literature roles are `paper_title`, `question_relevance`, `model_or_method`, `evidence_boundary`, and `source_location`. Solve/relationship/novelty retain their existing human requirements through corresponding stable role IDs.

## Validation flow

`parse_validate_render` validates role IDs while iterating claims, records the unique roles present, and returns them with the rendered Markdown and citation validation. Explicit unknown roles fail structurally. Role presence is independent from labels, text, or evidence validity.

`audit_generated_answer` passes structured roles to completeness validation. Structured completeness derives missing elements from role IDs; it does not scan Markdown. Non-structured compatibility retains section/claim checks but does not impose role requirements.

## Legacy compatibility

Role-less v1 claims map only through an explicit, bounded label alias table. This supports stored/replayed payloads and known prior prompt labels without searching arbitrary claim prose. New prompt output must include role.

## Audit and errors

Keep `AnswerCompletenessValidation` fields unchanged. `requiredElements` and `missingElements` remain Chinese display strings so the UI and audit bundle do not need a DTO migration. Missing roles continue to produce `ANSWER_COMPLETENESS_FAILED` with accurate display names.
