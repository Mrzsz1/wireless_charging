# Claim-level Verification 基线

- 状态：PASS
- verifier：`deterministic-claim-verifier-v1`
- Provider 抽象与失败 `unavailable`：PASS
- Supported：PASS
- Partially supported：PASS
- Contradicted：PASS
- Not verifiable / unsupported：PASS
- General knowledge：PASS
- Reasoned inference：PASS
- Research suggestion：PASS
- AnswerRepair：PASS
- 合法 Evidence ID 但语义不相关时不标记 verified/supported：PASS
- `QaRunManifest` telemetry：PASS

## 回归门禁

- Rust：183 PASS / 0 FAIL / 1 ignored（需本地 semantic model）
- 旧 citation / grounding / natural-answer 测试：PASS
- 冻结检索评测：保持独立运行，Claim Verifier 不改变 retrieval ranking。
