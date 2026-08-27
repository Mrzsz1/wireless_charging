# Semantic Verifier v2 真实 Provider 基线

- 运行日期：2026-08-27
- Contract：`semantic-claim-verifier-v2`
- Dataset：`semantic-v2-2026-08-27`
- Dataset role：development / regression / release semantic benchmark；不是 Independent Production Held-out
- Cases：60（20 entailed / 20 contradicted / 20 unknown）
- Cases SHA-256：`afdb2e85957fb288b388f483ffbdfea4f668cf2bd2276cd80c1f0340f04e5f09`
- Provider / model / reasoning effort：`codex-subscription` / `gpt-5.6-luna` / `low`
- Batch size：20；真实 Provider 共 3 个 batch call
- `realProviderMeasured=true`；60/60 completed；fallback=0

## Metrics

| Metric | Result |
|---|---:|
| Overall Accuracy | 1.000 |
| Entailed Precision / Recall | 1.000 / 1.000 |
| Contradiction Precision / Recall | 1.000 / 1.000 |
| Unknown Precision / Recall | 1.000 / 1.000 |
| Macro F1 | 1.000 |
| Timeout / Invalid JSON / Fallback | 0 / 0 / 0 |
| Invalid verified state | 0 |
| Total provider latency | 64,507 ms |

## Confusion matrix

| Gold \\ Predicted | Entailed | Contradicted | Unknown |
|---|---:|---:|---:|
| Entailed | 20 | 0 | 0 |
| Contradicted | 0 | 20 | 0 |
| Unknown | 0 | 0 | 20 |

- Failed cases：无。
- Category failures：无。
- 每题 `latencyMs` 是对应 batch 总延迟的确定性均分，60 条之和严格等于 `totalLatencyMs`；它不是 60 次独立 Provider 调用。
- 完整机器报告：`evals/reports/semantic-verifier-v2-report.json`。

## v1 Gold audit

v1 的十个 `missing_condition` case——`sem-009/019/029/039/049/059/069/079/089/099`——由 `unknown` 修正为 `contradicted`。这些 Evidence 都明确使用 `only in simulation` 排除了 Claim 所声称的真实部署验证。v1 dataset 升级为 `2026-08-27-semantic-v1.1`，重新密封 SHA-256 为 `e01861b4d673dc2d3c009b1901a2d88c5ba08987894cd95e38f98a0eb5bd3222`；没有把修标签后的旧预测当成 v2 结果。

## Boundary

本轮只证明新的三态定义在开发可见的不同表达方式上稳定。整体事实可靠性仍必须由独立 held-out、真实 QA Answer 与双 Reviewer 证明，本结果不改变当前 Production Release Gate 状态。
