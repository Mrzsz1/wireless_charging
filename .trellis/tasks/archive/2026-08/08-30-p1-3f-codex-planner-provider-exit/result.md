# P1-3F Final Result

1. **Baseline Commit**: `4c551e8e2d75425e33271248ffd4d71f16a673ae`.
2. **Final work commit**: the commit containing this result/spec update; archive and journal commits follow.
3. **Modified files**: shared Codex adapter, Planner failure projection, probe module/binary/export, QA re-export, safe probe reports, QA code-spec, and P1-3F task records.
4. **Codex CLI / executable**: shell baseline `codex.ps1` npm wrapper `0.146.0`; application probe selected desktop binary `0.151.0-alpha.7.1`.
5. **Parses `turn.failed`?** YES; fatal typed observation with fixed category and message SHA-256.
6. **Parses top-level `error`?** YES; fatal typed observation with fixed category and message SHA-256.
7. **`item.type=error` handling**: non-fatal warning only; a completed successful turn remains successful.
8. **Can JSONL Fatal still collapse to generic `provider_exit`?** NO; only a non-zero exit with no fatal event/classifiable stderr remains generic.
9. **J1–J7**: PASS. J1's old implementation returned `CODEX_EXIT_ERROR`; the fixed adapter returns `CODEX_JSONL_TURN_FAILED: schema_rejected`. J7 terminates within its bounded elapsed assertion.
10. **Probe A**: failed; terminal event `error`; corrected fixed category `transport`; exit `-1`; latency 47,830 ms; desktop CLI `0.151.0-alpha.7.1`; message hash `61cbbf0ae9181725d0cf3779e1e940b5e4bf65475d3c2cc3a59c1964ad8f7723`.
11. **Probe B**: not run because Probe A failed.
12. **Probe C**: not run because Probe B was ineligible.
13. **Selected branch**: Provider external transport (taskbook branch C).
14. **Why only this branch**: the minimal structured-output control failed before RetrievalContract Schema or real Planner input was introduced; Schema/Input/Integration conclusions are therefore unsupported and excluded.
15. **RED tests**: J1 proved fatal JSONL loss; the timeout-classification fixture proved the observed safe class changed from `unknown` to `transport`.
16. **Exact root**: `stream_answer_with` ignored fatal JSONL before P1-3F; after preservation, `classify_codex_terminal_message` initially lacked the request-timeout transport marker.
17. **Modified layer**: shared JSONL Adapter and diagnostic/probe infrastructure only.
18. **Other root-cause layers modified?** NO.
19. **Final real Research**: not run. Failed Probe A is a mandatory stop condition, so no Planner/Semantic/Grounding/persistence aggregates or exit code were fabricated.
20. **Call Budget / Token Ceiling changed?** NO; 3/4/5 and 8,000/18,000/32,000 remain unchanged.
21. **Semantic / Generator / Grounding changed?** NO.
22. **Independent Heldout run/read?** NO. Only the named synthetic `heldout_runner` unit-test subset required by the taskbook was executed.
23. **Frozen threshold changed?** NO.
24. **Quality**: fmt PASS; library/binary Clippy PASS; Rust subsets PASS (Codex 19, Provider 4, RetrievalContract 3, QueryPlan 5, Production Core 3, Real E2E 18, Adaptive Routing 18, Claim Verification 28, synthetic Heldout Runner 12); Python evaluator 24 PASS; `test:qa-evidence` 6 PASS; frontend build PASS.
25. **Final status**: **PARTIAL-BLOCKED** — terminal error adaptation and exact classification are complete, while the base Codex Provider transport path blocks the real Planner objective. P1-4 remains prohibited.
