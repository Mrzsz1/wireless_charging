# Design — Conversation State + State-aware Research Query v2

## Existing flow and change boundary

Keep the current understanding, routing, planner, retrieval, reranker, method discovery, generation, and verifier modules. Add three bounded owners:

```text
state_mutation.rs          natural-language operation extraction + patch schema
state_reducer.rs           deterministic patch validation/application
research_query_context.rs  intent-aware projection into planner/retrieval
```

`research_memory.rs` owns canonical state reconstruction from trusted user turns and upgrades to `research-session-state-v2`.

## Data flow

```text
history user turns -> apply historical ordered patches -> canonical pre-turn state
current message + resolved references + optional structured patch
  -> validated current patch
  -> deterministic reducer
  -> post-patch ResearchSessionState v2
  -> ResearchQueryContext
  -> initial retrieval terms + RetrievalPlanningInput
  -> planner or state-aware fallback RetrievalContract
  -> Retrieval/Rerank/Method discovery
```

The `RetrievalQuery` audit projection records patch/context telemetry and keeps the canonical state as a Rust-only field for the downstream `ContextPlan`; the frontend receives only bounded structured summaries.

## Mutation model

- `StateValue` is a closed enum for text or typed parameter values.
- Operations are ordered and normalized; destructive low-confidence operations are rejected.
- `Keep` is a no-op with warning when absent; it never adds.
- `Replace` requires `previousValue` to exist.
- `SetAll` is field-scoped and never inferred from generic remove/keep wording.
- Method remove moves the canonical value from active to excluded; add/replace removes the new value from excluded.

The deterministic extractor uses clause-local action detection and canonical vocab maps, not message-wide booleans. Complex structured understanding may supply an ordered patch, but never a final state.

## Query context

`ResearchQueryContext` includes current question/intent, selected objectives/constraints/assumptions/parameters, active/excluded methods, and resolved references. Selection is deterministic by intent:

- solution/exploratory: objective + constraints + parameters + assumptions + excluded methods
- method improvement: active method + objective + constraints
- comparison: resolved/active methods + objective + constraints
- direct/explanation: relevant constraint/entity plus compact model state

The fallback `RetrievalContract` gains bounded state facets and concepts; initial retrieval terms include canonical state tokens. Excluded methods are an audit/recommendation signal, not a retrieval blacklist.

## Compatibility

- All new serialized fields use serde defaults.
- Existing `build_retrieval_query` and `build_context_plan` entry points remain wrappers.
- Existing UnderstandingPlan payloads may omit `statePatch` in Rust fixtures, while the production closed schema requires it.
- Old persisted manifests hydrate with empty/default v2 fields.

## Benchmark

`conversation_state_v2_cases.json` stores full user-turn sequences plus exact final state/query context expectations and forbidden values. The evaluator uses the same deterministic state/reducer/context code, emits `qa-conversation-state-report-v2`, and includes 14 core cases plus 20/50/100-turn variants.

## Rollback

State model/reducer, query integration, and benchmark are committed as one isolated P0 change. Reverting restores v1 state derivation without changing database schema or persisted chat rows.
