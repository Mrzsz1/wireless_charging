# P1-1B Final Grounding Gate

## Goal

修复真实答案 grounding gate 审计对象错误：保留生成器草稿审计用于诊断，
但在 deterministic repair 之后重新审计最终答案，并且只允许最终审计结果控制
持久化、grounding 状态与可信历史资格。

## Requirements

- 将 grounding 流程显式拆分为 Draft Audit 与 Final Audit；禁止用草稿失败计数判定已修复答案。
- Final Audit 不调用第二个 LLM，不重新生成 claim，只做确定性投影、映射和证据校验。
- 最终事实 claim 必须逐条精确映射到草稿中已验证为 `Supported` 的 claim；新增或无法映射的事实 claim fail closed。
- `Supported` 与 `ResearchSuggestion/NotApplicable` 可保留；`Contradicted`、`NotVerifiable` 必须移除或替换为系统提示；`PartiallySupported` 不得保留完整原 claim。
- 提供共享 `is_grounding_system_notice` 分类，系统提示不进入事实分母、不要求引用、不得进入 trusted history。
- 有证据时，最终通过条件为：事实 claim 数大于 0、全部 supported、unsupported 为 0、unknown citation 为 0、citation coverage 为 1.0、completeness 为 true。
- 没有 supported claim 时输出固定安全结果，状态为 `insufficient_supported_claims`，且 `trustedHistoryEligible=false`；不得伪造 Grounded PASS。
- `answer_contract` 按 intent、execution mode、evidence coverage 和 has evidence 生成：Direct 优先最小直接回答与证据边界，不强制完整 Research profile；Research/Exploratory 只要求证据实际覆盖的元素。
- 持久化、运行清单和真实 E2E 报告必须区分草稿与最终 grounding 指标，并写入安全结构化日志。
- 原始 claim/证据诊断只允许写到 Git 之外的本地临时目录；不含凭据和绝对路径；任务结束删除。
- 只修复本文确认的 Final Grounding Gate 缺口，不调整 semantic budget、阈值、planner、state、zero-evidence、测试 case 或 heldout 数据。

## Acceptance Criteria

- [ ] Draft invalid、repair 后 Final valid 时允许持久化。
- [ ] Final 中新增事实 claim 时 Final Audit 失败。
- [ ] Grounding system notice 不作为事实 claim 且不要求 citation。
- [ ] Final Audit 重复执行结果一致。
- [ ] `PartiallySupported` repair 后不保留完整原 claim。
- [ ] 无 supported claim 时返回固定 insufficiency 答案、不 grounded、不进入 trusted history。
- [ ] Direct contract 不再强制完整 5–7 项 Research profile。
- [ ] Research 真实链路先复跑；目标为最终 supported 等于事实总数、unsupported=0、coverage=1、persisted=true。
- [ ] Direct 真实链路后复跑并准确归因残余 semantic/cross-language 阻塞，不修改预算。
- [ ] `cargo fmt --check`、目标 Rust 测试、clippy 与必要的前端构建通过；避免无关的全量过度测试。
- [ ] 每个实施阶段均创建本地 Git commit；不在本任务中自动推送 GitHub。

## Notes

- 任务来源：`C:/Users/qq155/Downloads/p1_1b_final_grounding_gate_fix.md`。
- 基线 commit：`61ce0f2` 之后的本地 master（包含 P1-1 诊断与日志提交）。
- 不改变 natural answer 的用户可见渲染设计，也不通过清零错误计数绕过 gate。
