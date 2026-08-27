# Conversation State Benchmark v2 基线

- 数据集：`conversation_state_v2_cases.json`
- 版本：`conversation-state-v2.1.0`
- Cases：22（14 个核心状态用例 + 5 个参数安全用例 + 20/50/100 轮长对话）
- 报告：`reports/conversation-state-v2-report.json`

| 指标 | 结果 |
|---|---:|
| State Exact Match | 1.0000 |
| Objective Exact Match | 1.0000 |
| Constraint Exact Match | 1.0000 |
| Method Exact Match | 1.0000 |
| Parameter Exact Match | 1.0000 |
| Mixed Operation Exact Match | 1.0000 |
| Parameter Overwrite Exact Match | 1.0000 |
| Unexpected State Rate | 0.0000 |
| Destructive Mutation Error Rate | 0.0000 |
| Query Context Objective Recall | 1.0000 |
| Query Context Constraint Recall | 1.0000 |
| Query Context Parameter Recall | 1.0000 |
| Query Context Excluded Method Accuracy | 1.0000 |
| Reference Resolution Accuracy | 1.0000 |
| Parameter Implicit Reference Resolved | 3 |
| Parameter Implicit Reference Rejected | 3 |
| Unknown Parameter Name | 2 |
| Parameter State Corruption | 0 |

该基线是可见的 development/regression 数据，用于锁定状态归约和查询上下文契约，不替代独立
held-out 的端到端事实准确率评估。
