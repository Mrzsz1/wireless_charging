# Semantic VerificationProvider 基线

- Contract：`semantic-claim-verifier-v2`
- Provider parity：Codex / Compatible API advertise `semantic_verification`; offline does not。
- Batch schema：closed object；exact Claim ID coverage；`entailed | contradicted | unknown`；confidence `0..1 | null`。
- Mapping gate：missing / unknown / graph-only Evidence 不调用 Provider。
- Budget：一次 batch call，阶段 `semantic_verifier`，与 generator 共用 request-scoped `LlmBudgetGuard`。
- 三态边界：先判断全部关键内容是否被支持；否则仅在 Evidence 明确陈述相反命题、包含互斥事实或真实排他语义时判 `contradicted`；其余证据不足均为 `unknown`。
- Regression：semantic success、contradiction、unknown scope/causality expansion、invalid JSON、Provider timeout、budget rejection、deterministic fallback 均 PASS。
- Telemetry：semantic checked、heuristic checked、provider、model、status、latency、fallback reason 分离。

Contract/failure semantics 继续使用 fake structured Provider 做最小回归；真实 Provider 的 60-case v2 结果见 `semantic-verifier-v2-baseline.md`。该开发可见 benchmark 不构成 Independent Production Held-out 或 Production Ready 证明。
