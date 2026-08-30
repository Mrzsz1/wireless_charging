# Provider Schema Split Verification

- `retrieval_contract_schema()` remains the full domain schema and contains exactly three `uniqueItems` keys.
- `retrieval_contract_provider_schema()` clones the domain schema and recursively removes only `uniqueItems`.
- `query_plan_schema()` exposes the full domain schema; `query_plan_provider_schema()` is used only at Provider boundaries.
- Provider wiring updated in production Planner, heldout runtime consistency path, and Probe B/C. No heldout data was opened or run.
- S1/S2/S3 test: `qa::retrieval_contract::tests::provider_schema_removes_only_unique_items_from_domain_schema` — PASS.
- Probe wiring test: `planner_probe::tests::provider_probe_schema_excludes_unique_items_after_projection` — PASS.
- `cargo check --lib` — PASS without warnings.
- No Prompt, parser, Retrieval ranking, budget, Semantic, Generator, Grounding, or user-visible answer behavior changed.
