# Requirements

## Problem

Evidence-backed QA accepts `qa-structured-answer-v1`, but literature prompts serialize required Chinese headings with the same `、` character that occurs inside the canonical title `主题、模型与方法`. Models can therefore split one required section into `主题` and `模型与方法`. Strict title-array comparison rejects the otherwise valid answer, and the failure is converted into a synthetic unsupported claim, causing the UI to falsely report `CITATION_VALIDATION_FAILED: 1 / 1`.

## Requirements

1. Structured sections use stable machine-readable section IDs; display titles are backend-owned.
2. The prompt expresses the section contract as unambiguous JSON rather than a delimiter-joined Chinese list.
3. Existing `qa-structured-answer-v1` answers without section IDs remain accepted when their titles map unambiguously to the canonical sections.
4. The known legacy split `主题` + `模型与方法` is normalized into the canonical `主题、模型与方法` section without weakening evidence validation.
5. Invalid structured output reports a dedicated structured-answer validation error containing the actual parse/contract reason; it must not be labeled as a missing-citation failure.
6. Evidence IDs remain strictly validated; compatibility normalization must never invent, remove, or reassign citations.

## Acceptance Criteria

- A literature answer with canonical section IDs is accepted and rendered with backend-owned Chinese headings.
- The supplied audit answer with five legacy sections is normalized into four rendered sections and proceeds through ordinary claim/evidence validation.
- An invalid JSON or irreconcilable section contract returns `STRUCTURED_ANSWER_VALIDATION_FAILED` with a useful reason.
- A genuine unknown/missing/Graphify-only evidence reference still returns `CITATION_VALIDATION_FAILED`.
- Prompt text contains a JSON section contract and no ambiguous joined heading enumeration.
