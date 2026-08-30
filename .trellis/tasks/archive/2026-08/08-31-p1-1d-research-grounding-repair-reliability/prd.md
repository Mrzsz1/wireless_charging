# P1-1D Research Grounding Repair Reliability

## Goal

Make final grounding repair deterministic: prove the current global `String::replacen()` collision with a synthetic RED fixture, replace it with fail-closed source-span reconstruction, and ensure repair never introduces a factual claim that is absent from the Supported Draft set.

## Background

- Baseline is `6ad4d767a656038aeab458e96be29fa9edbbf61f` with a clean worktree.
- Planner, Provider Schema, default `127.0.0.1:7890` proxy, Probe A/B/C, and Semantic Verifier are already proven functional.
- The blocking real Research run had Draft `4` claims (`1` supported, `1` partial, `2` not verifiable) but Final `2/1/1`; the extra final factual claim did not map to any Supported Draft claim.
- The current repair loop mutates the answer while locating each claim by the first matching string occurrence.
- The previous P1-3S result is `PARTIAL-BLOCKED`; its journal entry must use the same status and point to this task.

## Requirements

1. Add safe repair-projection diagnostics containing only schema/status, source and repaired hashes, claim IDs, byte spans, original-text hashes, replacement kinds/counts, and a stable error code. Do not persist raw claim/answer/evidence text.
2. Before production behavior changes, add and execute a synthetic test that demonstrates or excludes a wrong-occurrence replacement by the legacy `replacen` loop.
3. Bind each Draft Claim to an ordered, non-overlapping UTF-8 source span in the original answer. Missing, invalid, out-of-order, or overlapping spans fail closed; there is no fallback to global string replacement.
4. Reconstruct the repaired answer once from the immutable original answer and the validated spans. Preserve all existing status-to-notice mappings and the no-supported-claims notice.
5. Immediately re-extract repaired claims and reject the projection if any factual claim is neither an exact Supported Draft mapping nor a recognized grounding system notice.
6. Retain `audit_repaired_answer()`, `natural_answer::render()`, and `audit_rendered_visible_answer()` as independent downstream gates.
7. Emit structured `qa_repair_projection_started/completed/failed` events through the existing QA trace sink with the shared request hash, aggregate counts, and stable error code only.
8. Cover R1-R12 from the taskbook with deterministic focused tests, without running Independent Heldout.
9. After deterministic checks pass, run exactly one `real-research-improvement` E2E invocation without Shell proxy variables.

## Acceptance Criteria

- [x] Legacy `replacen` is proven RED by a committed synthetic collision fixture.
- [x] Production claim repair contains no global `replacen` call.
- [x] Duplicate text, substring collision, same-prefix/different-evidence, and Markdown/list repetition replace only their bound source occurrences.
- [x] Four Draft claims (`1` supported, `3` non-supported) project to Final `1/1/0` with no introduced factual claim.
- [x] Missing or invalid spans return a stable `REPAIR_PROJECTION_INVALID` classification and fail closed.
- [x] No-supported output remains `NO_SUPPORTED_CLAIMS_NOTICE`; system notices stay outside the factual denominator.
- [x] Trusted history remains empty unless the final audit is fully supported and visible-projection valid.
- [x] Visible projection and unknown-evidence rejection remain enforced.
- [x] Repair lifecycle logging tests cover success and representative failure with no content/path leakage.
- [x] Required Rust, Python, npm, build, and diff checks pass; Independent Heldout is not run.
- [x] The single real Research run is reported exactly as observed; no retry is used to manufacture a pass.
- [x] Planner, Provider Schema, proxy, budgets, Semantic verdicts/prompts, citation thresholds, and final grounding gates are unchanged.
- [ ] The post-notice-boundary-fix commit is proven by a new real Research PASS; the taskbook's one-run limit intentionally leaves this close gate unmeasured.

## Out of Scope

- Planner, Provider Schema, proxy, call-budget, or token-ceiling changes.
- Semantic prompt/verdict changes or weakening any citation/final-grounding threshold.
- Frozen thresholds, frozen datasets, Independent Heldout, or P1-4 work.
- Repeated real Research runs.
