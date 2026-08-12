# Requirements

## Problem

Structured QA answers are validated and rendered correctly, but completeness is then re-evaluated by searching the rendered Markdown for exact Chinese phrases. Semantically complete answers fail when labels use natural variants such as `模型与方法` instead of `模型或方法`, or `模型边界` instead of `证据边界`.

## Requirements

1. Remove exact required-element phrase searching from structured-answer completeness validation.
2. Add a stable machine-readable `role` to each structured claim.
3. Define intent-specific required role contracts owned by the backend and include them unambiguously in the model prompt.
4. Validate completeness from structured claim roles before Markdown rendering; labels and prose remain presentation text.
5. Preserve section, minimum-claim, evidence-ID, non-Graphify, supplement, audit-bundle, and failure-persistence checks.
6. Preserve backward compatibility for already generated v1 answers without roles only when deterministic legacy labels can be mapped; do not infer roles from arbitrary prose.
7. Keep non-structured/offline compatibility behavior without requiring structured roles.

## Acceptance Criteria

- Literature answers require `paper_title`, `question_relevance`, `model_or_method`, `evidence_boundary`, and `source_location` roles.
- Natural labels such as `求解方法`, `模型边界`, and `复现定位` pass when their explicit role is correct.
- Repeating irrelevant claims inside correctly named sections fails with missing roles.
- Rendered Markdown is not searched for exact required-element phrases on the structured path.
- `answerCompleteness.requiredElements/missingElements` continue to expose user-readable names derived from role validation.
- Focused tests and `cargo check` pass, then a Tauri release build produces MSI and NSIS installers.
