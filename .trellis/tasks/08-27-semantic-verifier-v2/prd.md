# Semantic Verifier v2 语义边界与评测升级

## Goal

按 semantic_verifier_v2_solution_plan.md 和 semantic_benchmark_v2.md 修正三态边界、导入 60 题 v2、补齐分类指标并运行一次真实 Provider；限制为最小必要测试，不改其他 QA 架构。

## Requirements

### R1 — 三态语义边界

- Semantic Verifier 必须按“先支持、再明确冲突、否则 unknown”的顺序判定。
- 证据不足、范围/因果/时间/条件外推默认不能被当成 contradiction；只有显式相反事实、排他语义或不可同时为真时才是 contradicted。
- Prompt 使用少量通用反例，不包含 case ID、category 分支或逐题规则。
- `Semantic Unknown` 只能合并为 `NotVerifiable` 或 `PartiallySupported`，Answer Repair 使用“证据不足”而不是“证据证明错误”。

### R2 — v1 Gold 审核

- 审核 v1 十个 `missing_condition` case：`sem-009/019/029/039/049/059/069/079/089/099`。
- Evidence 含 `only in simulation` 且 Claim 声称真实部署时，修正为 `contradicted`，同时更新 dataset version 与 canonical SHA-256。
- 不将修标签后的 v1 指标当成 v2 运行结果。

### R3 — v2 冻结数据集

- 将 `evals/reports/semantic_benchmark_v2.md` 的 60 个不同语义 case 转为 `semantic_verification_v2_cases.json`。
- 数据集固定为 20 entailed / 20 contradicted / 20 unknown，保存 dataset role、version、caseCount 与 cases SHA-256。
- v2 明确属于 development/regression/release semantic benchmark，不得标记为 Independent Production Held-out。

### R4 — v2 指标与报告

- 报告输出 overall accuracy、三类 precision/recall、macro F1、3×3 confusion matrix、category metrics、失败 case、timeout/invalid/fallback、invalid verified state、逐题 latency/provider/fallback。
- 指标必须从逐题预测计算，零分母行为确定且可测试。
- `realProviderMeasured=true` 仍要求真实 Provider 完整跑完全部 case。

### R5 — 有限验证

- 只运行 Semantic Verifier 相关 Rust 测试、`cargo fmt/check` 和一次真实 60-case Provider benchmark。
- 不运行全量 Rust、前端、GUI、安装包、RAG 或 Production Gate。
- 不通过反复运行 benchmark 调 Prompt，不修改 provider/model/temperature/batch/retrieval/answer generation。

## Acceptance Criteria

- [ ] AC1：三态 Prompt 与版本升级，包含严格决策树和最小通用反例，无 benchmark 特判。
- [ ] AC2：v1 十个错误 Gold、版本和 SHA-256 被一致修正。
- [ ] AC3：60-case v2 JSON 通过 schema/hash/20-20-20 分布校验。
- [ ] AC4：v2 报告包含全部要求的总体、逐类、混淆矩阵、失败 case 与逐题运行字段。
- [ ] AC5：相关 Rust 测试、fmt/check 通过；真实 Provider 仅运行一次且报告完整。
- [ ] AC6：未改动其他 QA 架构，未宣称 Production Ready，用户原有未跟踪文件保持不变。

## External Boundary

- v2 benchmark 已被开发流程看到，只能证明 Semantic Verifier 的回归稳定性。
- Production Ready 仍依赖独立 held-out、真实 QA answer 与双 reviewer。
