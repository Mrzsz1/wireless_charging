# Design — P1-3S v2 Minimal Provider Schema Compatibility

## Boundaries

The full `RetrievalContract` JSON Schema remains the domain-facing source of truth. A second provider-facing projection is derived from it immediately before the schema crosses the Codex Structured Outputs boundary:

```text
retrieval_contract_schema()                full domain schema
        |
        +-> clone Value
              -> recursive evidence-driven compatibility transform
                    -> retrieval_contract_provider_schema()
                          -> query_plan_provider_schema()
                                -> production Planner / Probe B / Probe C
```

The first transform removes only `uniqueItems`. The full schema, Rust structs, Serde strictness, parser, and `normalize()` remain unchanged.

## Schema projection contract

`remove_provider_unsupported_keywords(&mut Value)` recursively visits objects and arrays. On every object it removes the exact key `uniqueItems`, then recurses into every remaining value. It does not maintain a broad speculative allowlist and does not remove any other keyword.

Tests compare a clone of the domain schema after applying the same one-key projection with the returned Provider schema. This proves all properties, required sets, nesting, enum values, and other constraints are structurally preserved.

## Wiring

`qa/query_plan.rs` exposes both:

- `query_plan_schema()` for the complete domain contract.
- `query_plan_provider_schema()` for Codex Structured Outputs.

Only Provider call sites change:

- `qa/production_core.rs`
- `production_heldout.rs` (wiring consistency only; no heldout data is opened or executed)
- `planner_probe.rs` Probe B and Probe C definitions

Prompt construction and `parse_query_plan()` remain tied to the same `RetrievalContract` domain model.

## Local validation

Provider schema relaxation does not relax acceptance. `parse_retrieval_contract()` still deserializes with `deny_unknown_fields` and runs `normalize()`. The existing `HashSet`-based kind normalization keeps first occurrence order; facet IDs remain fail-closed on duplicates. Bounds and relationship checks remain local and deterministic.

## Live-run gates

Live requests are strictly serial and one-shot:

```text
already-passed A
  -> temporary-proxy Probe B
       -> PASS only: temporary-proxy Probe C
            -> PASS only: temporary-proxy real-research-improvement
                 -> PASS only: default proxy implementation
                      -> shell proxy cleared
                           -> A -> B -> C -> real Research
```

Every safe report gets a unique non-overwriting path. A failure stops the chain. Raw diagnostics, when needed for a new rejected keyword, live outside the repository and are deleted after extracting only a safe classification/hash.

## Proxy resolution

After the temporary-proxy Research gate passes, a pure resolver chooses a child-process proxy policy:

1. Non-empty `WIRELESS_CODEX_PROXY_URL` wins. Case-insensitive `off`, `direct`, or `none` resolves to Direct.
2. Otherwise, if any standard proxy environment variable is already non-empty, preserve the inherited environment and do not overwrite it.
3. Otherwise, inject `http://127.0.0.1:7890` into the Codex child command only.

The resolver never mutates the parent environment. It returns a safe mode/source enum for tests; URLs and credentials are not logged. The command configurator applies the resolved values to all Codex child processes that need network access, without changing arguments, model, timeout, or output schema behavior.

## Observability

No new ad hoc log is introduced. Existing `qa_planner_probe_*`, Planner lifecycle, Codex diagnostics, and real E2E events already emit start/completion/failure with stable operation IDs and redacted error categories. Schema projection is a pure helper and proxy resolution contains no independently failing side effect; failures continue to surface at the existing Provider stage.

## Compatibility and rollback

- Domain schema consumers keep the existing function and behavior.
- Provider call sites opt into the explicit compatible projection.
- Parser acceptance remains at least as strict as before because local validation is unchanged.
- Removing the provider wiring commit restores old behavior cleanly; removing the proxy commit restores inherited/system networking behavior.
- No data migration, manifest version, report schema, budget, or user-visible answer change is required.
