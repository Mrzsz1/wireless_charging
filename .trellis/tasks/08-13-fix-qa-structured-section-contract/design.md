# Technical Design

## Canonical contract

Define one backend-owned section descriptor per intent with `id` and `title`. For literature the canonical IDs are `conclusion`, `related_papers`, `topic_methods`, and `boundary_reproduction`. Other intents receive equally stable IDs for their existing headings.

The model payload section gains an optional `id`. New prompts require it. Optionality preserves parsing of already-produced v1 audit bundles.

## Normalization

`parse_validate_render` normalizes model sections before validation:

1. If IDs are present, validate exact canonical ID order and reject unknown, duplicate, mixed, or mismatched ID/title data.
2. If IDs are absent, map exact legacy titles to canonical descriptors.
3. For literature only, merge adjacent legacy sections titled `主题` and `模型与方法` into `topic_methods`, preserving group and claim order.
4. Compare normalized IDs to the expected canonical order.
5. Render canonical backend titles, ignoring model-authored presentation wording after validation.

No claim or `evidenceIds` mutation occurs.

## Error flow

Extend the internal `AnswerAudit` with an optional structured validation error. `audit_generated_answer` retains invalid validation metadata for audit persistence but also records the real structural error. `persist_exchange_with_metadata` checks this field before citation failure formatting and returns `STRUCTURED_ANSWER_VALIDATION_FAILED: <reason>`. Genuine citation failures retain the existing error code.

## Compatibility and rollback

The schema version remains `qa-structured-answer-v1` because the new `id` field is additive and legacy title-only payloads remain supported. Rollback is isolated to the structured parser, prompt builder, and internal error projection.
