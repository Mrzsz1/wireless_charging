# RED Baseline — Provider Schema `uniqueItems`

- Baseline source commit: `02df15c0f694f416f4af44a5daf4724114086b8a`.
- Planning commit: `601f23a`.
- Production behavior was unchanged when this observation was recorded.
- Test: `planner_probe::tests::current_provider_schema_contains_unique_items_before_fix`.
- Command: `cargo test current_provider_schema_contains_unique_items_before_fix --lib`.
- Result: PASS as an observation of the defect: the schema passed to Probe B contained `uniqueItems:true` at:
  - `/properties/requestedKinds/uniqueItems`
  - `/properties/mustAttemptKinds/uniqueItems`
  - `/properties/facets/items/properties/preferredKinds/uniqueItems`
- This establishes the evidence for the first and only initial Provider compatibility transform.
