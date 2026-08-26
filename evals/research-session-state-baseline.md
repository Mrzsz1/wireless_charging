# Research Session State 基线

- 状态：PASS
- 20 / 50 / 100 条 ConversationTurn 压力回归：PASS
- active problem：PASS
- objective latest-wins：PASS
- constraint add/remove：PASS
- method replacement：PASS
- assumptions / papers / hypotheses / open questions：PASS
- source message trace：PASS
- ContextPlan structured injection：PASS
- state token budget accounting：PASS
- `QaRunManifest` version/revision/count telemetry：PASS

## 回归门禁

- Rust：230 PASS / 0 FAIL / 2 ignored（真实 reranker / semantic model 分离到 RC gate）
- 旧 context budget / long-history tests：PASS
- RAG evaluation：PASS
- Provider capability/failure matrix：PASS（Codex / Compatible API / local-only；timeout / invalid response / rate limit / budget）
- Planner、embedding/reranker、semantic verifier、Graphify 与 DB lock failure injection：PASS
