# 复杂对话状态与 State-aware Research Query 整改

## Goal

按 complex_conversation_state_query_solution.md 实现逐对象 ResearchStatePatch、确定性 StateReducer、ResearchSessionState v2、State-aware ResearchQueryContext、最新状态先应用再规划检索，并新增 Conversation State Benchmark v2。

## Requirements

### R1 — Per-object state mutation

- Replace sentence-wide remove/replace booleans with ordered `ResearchStatePatch.operations` over objective, constraint, assumption, method, and parameter fields.
- Support `add/remove/keep/replace/set/set_all/clear`; last explicit operation on the same target wins by ordered reduction.
- Low-confidence destructive operations, ambiguous references, and replace with a missing source fail closed without modifying state.
- Reducer is deterministic and never calls an LLM.

### R2 — ResearchSessionState v2

- Preserve objectives/constraints/assumptions/methods/papers/hypotheses/open questions.
- Add typed parameters, excluded methods, state version, last patch ID, source message/turn tracking, and bounded telemetry.
- Active state must remain distinct from historical mentions; removed methods may be retained only in `excludedMethods`.

### R3 — Mutation extraction

- Deterministic parsing handles clear single/multi-operation Chinese/English mutations, parameter overwrite, set-all, clear, replacement, and same-turn self-correction without algorithm-specific case branches.
- Existing structured understanding call may return only an ordered patch—not a final state—and is used for complex/ambiguous cases; all output is validated before deterministic reduction.
- Unknown/low-confidence destructive actions are rejected.

### R4 — State-aware query context

- Build `ResearchQueryContext` from the post-patch state and resolved references before planner/retrieval.
- Intent-aware selection includes current objectives, critical constraints, relevant parameters, assumptions, active methods, and excluded methods without appending unlimited history or one oversized query string.
- Planner input and deterministic fallback contract consume the structured context; excluded methods lower recommendation eligibility but do not hard-filter useful comparison evidence.
- Required order is Extract Patch → Apply Patch → Build Context → Plan → Retrieve.

### R5 — Benchmark and telemetry

- Add Conversation State Benchmark v2 with core state, parameter, open question, ambiguous mutation, combined mutation+question, and 20/50/100-turn cases.
- Measure exact state/objective/constraint/method/parameter match, unexpected state rate, destructive mutation error rate, query-context recalls, excluded-method accuracy, and reference resolution.
- Record patch operation/low-confidence/rejected counts, changed/warning counts, and query-context field counts.

### R6 — Boundaries

- Reuse current Conversation Resolver, ResearchIntent, Retrieval, Reranker, Method Matcher, and Semantic Verifier; do not add a new Agent or rewrite QA architecture.
- No case-ID, fixed-algorithm-pair, or sentence-wide keyword switches that clear entire state.
- Preserve current semantic/retrieval/reranker contracts and limit validation to relevant unit/integration plus required regression filters.

## Acceptance Criteria

- [x] AC1：mixed operation、parameter overwrite 与 self-correction exact match 为 100%。
- [x] AC2：ambiguous destructive mutation 与 replace-source-missing 均 fail closed；destructive mutation error rate = 0。
- [x] AC3：State/Objectives/Constraints/Methods/Parameters exact match ≥ 0.98，unexpected state rate ≤ 0.01。
- [x] AC4：Query Context objective/constraint/parameter recall 与 excluded-method accuracy ≥ 0.97。
- [x] AC5：Planner 输入和 fallback contract 可审计地使用 post-patch state；组合 case 不再使用旧参数或已排除方法作为主要推荐目标。
- [x] AC6：20/50/100-turn benchmark 使用 canonical state，而非无限历史 Prompt。
- [x] AC7：相关 Rust fmt/check/tests 与必要 Retrieval/Semantic/Reranker regression 通过；不运行 GUI/安装包无关测试。
- [x] AC8：规范、基线、报告、本地 Git 提交与 Trellis 归档完成；用户未跟踪文件保持不变。

## Production boundary

- Benchmark is development-visible and validates state/query contracts; it does not replace independent factual held-out review.
