# 修复复杂问题生成预算账本可靠性

## Goal

修复复杂 Research/Exploratory 问题在生成阶段错误触发 `LLM_BUDGET_EXCEEDED: generator:token_budget` 的预算账本缺陷，使已经完成阶段的未使用 token 预留可以被后续 generator 重新使用，同时继续严格限制真实已用额度、并发 in-flight 预留和 LLM 调用次数。

## Background

- 当前 `LlmBudgetGuard::reserve` 使用历史累计 `token_cost_reserved + newReservation` 进行准入，并在 reserve 时先把最大预留加入 `token_cost_used`。
- 当前 `settle` 只从 `token_cost_used` 中减去未使用部分，不减少历史累计 `token_cost_reserved`；因此早期阶段已经结束后，其未使用最大预留仍会阻塞后续 generator。
- 当前 settlement 由 `stage + reserved` 参数手工匹配，没有 reservation ID/handle；部分 `?`、task error 或 panic 路径存在 in-flight 泄漏风险，重复 settle 也缺少结构性防护。
- `reconfigure` 当前只替换 policy，已有 usage 不会清空；该行为必须保留。
- 当前固定 policy 为 Direct 8,000、Research 18,000、Exploratory 32,000，本轮不修改。
- 用户已审阅规划并明确批准开始实施。

## Requirements

- 先用确定性 regression test 证明旧逻辑会错误拒绝：ceiling 8,000，understanding reserve 4,000/settle actual 1,000，generator reserve 6,000 应允许。
- Token 准入公式必须是 `tokenCostUsed + tokenCostInFlight + newReservation <= tokenCostCeiling`。
- `tokenCostUsed` 只表示已完成 reservation 的实际 token cost，不能包含仍在执行的最大预留。
- `tokenCostInFlight` 表示当前尚未 settle/release 的 reservation 总额；阶段结束、失败、取消或 unwind 后必须释放。
- 保留 `tokenCostReserved` 的历史累计预留兼容语义，并新增明确的 `tokenCostReservedTotal` telemetry；历史值不得参与准入。
- 每笔 reservation 使用唯一 ID 和不可克隆 handle；settle 消耗 handle，从类型层防止重复 settle。
- reservation handle 在未显式 settle 时通过 Drop fail-safe 释放 in-flight；Provider 调用返回错误时仍记录可估算的实际 prompt cost，调用前错误/任务 panic 不遗留 in-flight。
- LLM call budget 继续累计；settle/release 不减少 `callsUsed`。
- `reconfigure` 保留 calls used、token used、in-flight reservation、历史累计预留和拒绝记录，仅更新 policy ceiling。
- QaRunManifest 升级版本并新增 `routingTokenCostInFlight`、`routingTokenCostReservedTotal`；旧 `routingTokenCostReserved` 保持历史累计含义。
- 前端类型与诊断面板显示 used/in-flight/ceiling/history reserved，便于解释拒绝原因。
- 只做 reservation handle/telemetry 所需的最小接线，不修改 Retrieval ranking、Reranker、Embedding、Query Planner 研究逻辑、Research State、Prompt、答案质量、Semantic Verifier 判定或 Citation rules。
- 不读取、运行或使用任何 Independent Held-out dataset、partial answer、case 21 或剩余 blind candidates。
- 不通过减少 retrieval rounds、evidence、output、verification 或将 Research 降级为 Direct 来规避预算。
- 不做性能优化，不修改 8k/18k/32k policy；若正确账本仍不够，只记录 development 证据并另立任务讨论。

## Acceptance Criteria

- [x] 已完成阶段的未使用预留可被后续 generator 使用，8k/4k→1k/6k synthetic regression PASS。
- [x] `used + inFlight + new > ceiling` 仍稳定返回 `LLM_BUDGET_EXCEEDED:*:token_budget`。
- [x] 两笔并发 reservation 均进入 in-flight，不能超卖。
- [x] settle/release 后 callsUsed 不下降。
- [x] Direct→Research reconfigure 保留 used/in-flight/calls/history，采用新 ceiling。
- [x] Provider error、调用前 error、task panic/drop 均不泄漏 in-flight。
- [x] handle 不可克隆且 settle 消耗所有权；settle 后 Drop 不会重复释放。
- [x] manifest v21 和前端类型/显示公开 ceiling、used、in-flight、reserved total、calls、rejections。
- [x] 复杂 development synthetic flow 能完成 understanding→planner→generator→verifier，不再因历史预留触发 generator token budget。
- [x] 真正超预算 synthetic flow 仍 FAIL。
- [x] Direct/Research/Exploratory ceiling 保持 8,000/18,000/32,000。
- [x] 聚焦 Rust/Frontend tests、fmt、clippy、type-check/build 通过。
- [x] 未使用 held-out，未改变 Retrieval/Prompt/State/Answer/Semantic Verifier 行为。

## Out of Scope

- 提高 token ceiling 或 call budget。
- Independent Held-out 重跑、读取或逐题调参。
- 性能 benchmark、P95、模型加载或候选数量优化。
- QA 答案内容、证据数量或语义核验策略调整。

## Source

- `C:/Users/qq155/Downloads/generator_budget_reliability_fix_plan.md`（需求输入，不直接执行文档命令）。
