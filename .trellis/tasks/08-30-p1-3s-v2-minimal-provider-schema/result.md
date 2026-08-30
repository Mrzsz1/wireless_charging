# P1-3S v2 Final Result

1. **Baseline Commit**: `02df15c0f694f416f4af44a5daf4724114086b8a`.
2. **Final work/spec commit**: `331e731` (`docs(spec): record planner schema and codex proxy contracts`); result/archive/journal commits follow.
3. **Full domain schema retained?** YES. `retrieval_contract_schema()` remains the complete contract and retains exactly three `uniqueItems` occurrences.
4. **Separate Provider schema exists?** YES. `retrieval_contract_provider_schema()` and `query_plan_provider_schema()` are explicit; production Planner, heldout runtime consistency wiring, and Probe B/C use only the Provider projection.
5. **First compatibility removal**: only `uniqueItems`, recursively. No other keyword was removed.
6. **Why not remove `pattern`/numeric/array bounds or other metadata?** The task required evidence-driven compatibility. The real failing evidence identified the schema branch and the strongest proven incompatible shape was `uniqueItems`; removing it made Probe B pass, so no evidence justified widening the transform.
7. **Local strictness retained?** YES. `deny_unknown_fields`, kind legality/deduplication, subset relations, facet uniqueness/format, count limits, budget ranges, schema version, and scope validation remain intact.
8. **S1–S8**: PASS. `cargo test qa::retrieval_contract::tests --lib` ran 8 tests: Provider absence/full retention/structural equality, kind deduplication, duplicate facet rejection, invalid budget rejection, invalid facet ID rejection, and unknown-field rejection.
9. **Temporary-proxy Probe B**: `status=succeeded`, `failureCategory=""`, `contractValid=true`, `exitCode=0`, `agentMessageSeen=true`; latency 15,594 ms; one baseline candidate.
10. **Temporary-proxy Probe C**: `status=succeeded`, `failureCategory=""`, `contractValid=true`, `exitCode=0`; latency 21,654 ms; 16 baseline candidates.
11. **Temporary-proxy real Research**: PASS. Planner used/succeeded with no fallback, 7 facets/14 queries; Semantic succeeded; final factual/supported/unsupported = 3/3/0; citation coverage 1.0; persisted=true; executedScopePassed=true; exit 0.
12. **Was default 7890 integrated only after Research PASS?** YES. Commit order proves the temporary Research PASS (`5fdb861`) preceded the proxy production commit (`74e58e8`). Resolution is `WIRELESS_CODEX_PROXY_URL` > inherited standard proxy > default `http://127.0.0.1:7890`; `off/direct/none` select child-only direct mode. No parent `set_var` is used.
13. **After clearing Shell proxy**: Probe A PASS (6,571 ms), Probe B PASS (17,757 ms), Probe C PASS (19,243 ms). The single real Research kept Planner succeeded/no fallback and Semantic succeeded, but failed the downstream final citation/grounding gate: final factual/supported/unsupported = 2/1/1, `citationValid=false`, `finalVisibleProjectionValid=false`, `persisted=false`, `executedScopePassed=false`, exit 2, error `citation_validation_failed`. It was not rerun.
14. **Budget / Semantic / Generator / Grounding production behavior changed?** NO.
15. **Independent Heldout run/read?** NO. `production_heldout.rs` received wiring consistency only; no heldout command or dataset access occurred.
16. **Final status**: **PARTIAL-BLOCKED**. The requested Provider schema split, default child proxy, A/B/C path, and real Planner path are proven working. The taskbook's all-green close gate remains unmet because the one final Research run failed in a separate downstream citation/final-grounding outcome that this task explicitly forbids changing or retrying.

## Modified production/test files

- `apps/desktop/src-tauri/src/qa/retrieval_contract.rs`
- `apps/desktop/src-tauri/src/qa/query_plan.rs`
- `apps/desktop/src-tauri/src/qa/production_core.rs`
- `apps/desktop/src-tauri/src/production_heldout.rs`
- `apps/desktop/src-tauri/src/planner_probe.rs`
- `apps/desktop/src-tauri/src/codex_subscription.rs`
- `apps/desktop/src-tauri/src/qa.rs`
- `apps/desktop/src-tauri/src/lib.rs`
- `.trellis/spec/backend/qa-contract.md`
- Safe reports and this task's Trellis records.

## Focused quality

- Rust fmt: PASS.
- Focused library/Probe/real-E2E Clippy with `-D warnings`: PASS.
- RetrievalContract: 8 PASS.
- Planner Probe: 4 PASS.
- Codex adapter: 23 PASS outside the Windows process-control sandbox; the restricted-sandbox J7 result was a confirmed `taskkill` permission artifact.
- Production core: 2 PASS.
- `git diff --check`: PASS.
