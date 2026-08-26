# Real Semantic VerificationProvider 基线

- 检查日期：2026-08-26
- Dataset：`2026-08-26-semantic-v1`
- Cases：100 frozen claim-evidence pairs
- Dataset SHA-256：`3b4993b4acdf7ca0b155948ca63d97525411f3a1be26c31534d6bbe7db88414c`
- Provider：`codex-subscription`
- Model：`gpt-5.6-luna`
- Reasoning effort：`low`
- `realProviderMeasured`：`true`
- Completed：100/100
- Accuracy：0.820
- Contradiction recall：1.000
- Unknown precision：1.000
- Timeout / invalid JSON / fallback：0 / 0 / 0
- Invalid verified state：0
- Total provider latency：572401 ms

该 benchmark 直接复用生产 `VerificationProvider -> PlanningProvider -> Codex structured
output` 路径，原始 Provider payload 不落盘。Fixture 覆盖 direct/paraphrase/multi-evidence
entailment、numeric/negation/condition contradiction，以及 scope/causality/missing-condition/
multi-evidence insufficient 情况。

0.820 accuracy 被如实保留，不针对真实 Provider 输出回改 frozen v1 gold 或 prompt。主要分歧集中在
`unknown` 与 `contradicted` 边界：证据仅覆盖 simulation 时，“已在真实部署验证”被模型判为
contradicted，而 fixture v1 标为 unknown；部分 scope/causality expansion 也被判为 contradicted。
这不阻止 `realProviderMeasured=true` 的部署事实，但在专家复核并冻结 v2 标签政策前，不把该结果
解释为 82% 的最终用户回答准确率，也不替代独立 held-out 人工评审。

运行入口：

```powershell
cd apps/desktop
npm run eval:semantic
```
