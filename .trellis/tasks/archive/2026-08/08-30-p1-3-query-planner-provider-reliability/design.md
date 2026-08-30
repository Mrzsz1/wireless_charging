# Design — P1-3 Query Planner Provider Reliability

## Diagnostic boundary

Phase 1 changes observation and diagnostics only:

```text
Planner request boundary
  -> safe lifecycle event
  -> existing Planner/fallback behavior
  -> QaRunManifest v22 safe fields
  -> GroundingObservation v5 projection
  -> strict Research/Exploratory verdict
```

No Planner schema, prompt, parser, timeout, input, budget, or adapter behavior changes before the first real diagnostic.

## Report v5 projection

`GroundingObservation` adds only booleans, enums, counts, and milliseconds derived from current run state. `plannerAttempted` comes from status/stage evidence; `plannerStageObserved` comes from the canonical `planner` stage in `routingLlmStages`; budget rejection is derived from stable rejection codes. Query/facet/kind counts are computed before serialization, never by retaining raw queries.

Research/Exploratory verdicts use the projected values as a conjunction. Direct has a separate legal policy-disabled contract.

## Stable failure taxonomy

One pure classifier owns prefix-to-enum mapping. Classification occurs where Planner errors are converted into deterministic fallback metadata, so manifest, report, and logs share the same redacted reason. Tests cover every enum plus unknown and ensure the raw suffix never propagates.

## Lifecycle logging

The orchestration boundary emits started before reservation/Provider work, then completed only after a validated QueryPlan is accepted, or failed when deterministic fallback is selected. The existing `qa::trace` schema is reused; no raw diagnostic file or ad hoc logger is introduced.

## Branch isolation

After the one real diagnostic:

- A changes Provider schema construction/wiring only.
- B changes exactly one proven prompt/schema/parser/normalization mismatch.
- C changes Planner input sizing or the Planner-only timeout, based on measured aggregates.
- D changes duplicate/error stage accounting only.
- E changes only anomalous Planner input/reservation accounting; a genuinely valid >18k chain stops.
- F changes only a reproducible Provider adapter defect; transient/unavailable state produces no code change.

Each code branch requires a RED fixture against the current diagnosed implementation before its production patch.

## Real-run isolation

The first diagnostic writes `evals/reports/qa-real-generator-e2e-planner-diagnostic.json`; the verification writes a separate Planner verification report. Both use the public development case and never open formal heldout data. Model and effort remain `gpt-5.6-luna` / `low`.

## Compatibility

- `qa-run-v22` and all stored manifests remain unchanged.
- E2E report v5 is a development-report schema upgrade.
- Existing safe fallback continues to work, but strict Research/Exploratory E2E treats it as failure.
