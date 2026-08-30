# P1-1D2 Post-Fix Real Research Close Gate

## Goal

Verify the committed source-span and notice-boundary repair with deterministic gates and exactly one post-fix `real-research-improvement` run; preserve all Planner, Provider, proxy, Semantic, and Final Grounding contracts.

## Background

- Baseline: `02c1bb8de1a9a628b1e324f4be0145c55af06410`, clean `master`, already pushed to `origin/master`.
- P1-1D proved the legacy global `replacen` collision and replaced it with ordered source-span reconstruction.
- Its single live run exposed `introduced_factual_claim`; `5f4eee9` reproduced the inline notice-boundary drift and `bbce464` fixed it deterministically.
- The post-`bbce464` real close gate has not been measured.

## Requirements

1. Re-run the deterministic P1-1D quality gates before any live traffic.
2. Confirm claim repair contains no global `replacen` and the connector/notice-boundary regression remains green.
3. Clear Shell proxy variables and rely on the product's existing default `127.0.0.1:7890` child-proxy behavior.
4. Execute exactly one `QA_REAL_E2E_CASE_ID=real-research-improvement` run.
5. Do not modify Planner, Provider Schema, proxy, budgets, retrieval, generator, Semantic verdict/prompt, citation thresholds, Final Grounding Gate, or frozen/heldout assets.
6. If the run passes, close the P1-1D real Research gate. If it fails, preserve the report, classify the exact first failing gate, and do not retry live traffic in this task.

## Acceptance Criteria

- [ ] Required deterministic Rust/Python/frontend/build checks pass.
- [ ] Independent Heldout is not executed.
- [ ] Exactly one post-fix real Research invocation is recorded.
- [ ] Planner is attempted/used/succeeded with no fallback.
- [ ] Semantic status is succeeded.
- [ ] Repair Projection is succeeded with no error.
- [ ] Final factual count is positive, supported equals factual, unsupported is zero, citation coverage is `1.0`, and visible projection is valid.
- [ ] Persistence succeeds, executed scope passes, and process exit code is `0`.
- [ ] All commits are archived/journaled and normally pushed to GitHub.

## Out of Scope

- Any second live run.
- Independent Heldout or P1-4.
- Weakening or bypassing a failed gate.
