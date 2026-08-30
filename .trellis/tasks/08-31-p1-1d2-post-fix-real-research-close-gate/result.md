# P1-1D2 Final Result

## Status

**PASS** — the post-`bbce464` real Research close gate passed on its first and only invocation. P1-1D's downstream Research grounding blocker is closed.

## Commits

- Baseline: `02c1bb8de1a9a628b1e324f4be0145c55af06410`.
- Plan: `86545e2`.
- Activated measurement baseline: `60e92ae565e1a5bb50eee3ec7ca5c108841f4793`.
- Real close-gate report: `4dc3db5`.

## Scope

- No production code changed in this task.
- Planner, Provider Schema, proxy configuration, Retrieval, Generator, budgets, Semantic verdict/prompt, citation thresholds, Final Grounding, and frozen data were untouched.
- Independent Heldout was not executed; only deterministic `heldout_runner` unit tests ran.

## Deterministic Preflight

- `cargo fmt --check`: PASS.
- `cargo clippy --lib --bins -- -D warnings`: PASS.
- `cargo test --lib claim_verification`: 39 PASS.
- `cargo test --lib production_core`: 3 PASS.
- `cargo test --lib real_e2e`: 19 PASS.
- `cargo test --lib heldout_runner`: 12 PASS.
- `cargo test --lib natural_answer`: 4 PASS.
- `py -3 -m unittest tests.test_qa_accuracy_eval`: 24 PASS.
- `npm run test:qa-evidence`: 6 PASS.
- `npm run build`: PASS.
- Production claim repair contains no `replacen` call.

## Single Post-Fix Real Research

The task removed Shell proxy variables and used the product's existing child-process proxy resolution. Exactly one invocation ran:

```text
QA_REAL_E2E_CASE_ID=real-research-improvement npm run eval:qa-real-e2e
```

Result: exit code `0`, `Real Answer Generator E2E PASS`.

| Metric | Observed |
|---|---|
| Provider / model | `codex-subscription / gpt-5.6-luna` |
| Planner attempted / used / status / fallback | `true / true / succeeded / false` |
| Planner facets / queries | `4 / 8` |
| Semantic status | `succeeded` |
| Evidence selected | `14` |
| Draft factual / supported / not-verifiable | `6 / 3 / 3` |
| Repair Projection | `succeeded`, empty error code |
| Repair operations / replacements | `6 / 3` |
| Final factual / supported / unsupported | `3 / 3 / 0` |
| Final cited / unknown | `3 / 0` |
| Final citation coverage | `1.0` |
| Visible projection | `true` |
| Citation valid | `true` |
| Persisted | `true` |
| Executed scope passed | `true` |
| Errors | none |

`scope=single_case`, so `fullSuiteEvaluated=false` and `releaseEligible=false` remain correct; this task closes the selected P1-1D Research gate rather than claiming a full five-case release evaluation.

## Artifacts

- `evals/reports/qa-real-generator-e2e-report.json`
- Git-ignored structured trace: `apps/desktop/logs/qa-real-e2e.jsonl`
