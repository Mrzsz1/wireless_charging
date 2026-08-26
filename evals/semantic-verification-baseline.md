# Semantic VerificationProvider 基线

- Contract：`semantic-claim-verifier-v1`
- Provider parity：Codex / Compatible API advertise `semantic_verification`; offline does not。
- Batch schema：closed object；exact Claim ID coverage；`entailed | contradicted | unknown`；confidence `0..1 | null`。
- Mapping gate：missing / unknown / graph-only Evidence 不调用 Provider。
- Budget：一次 batch call，阶段 `semantic_verifier`，与 generator 共用 request-scoped `LlmBudgetGuard`。
- Regression：semantic success、contradiction、unknown scope expansion、invalid JSON、Provider timeout、budget rejection、deterministic fallback 均 PASS。
- Telemetry：semantic checked、heuristic checked、provider、model、status、latency、fallback reason 分离。

当前测试使用 fake structured Provider 验证 contract 与 failure semantics；它不构成真实 Provider factual-precision 或 Production Ready 证明。真实 Provider/held-out 结果由最终 release gate 单独要求。
