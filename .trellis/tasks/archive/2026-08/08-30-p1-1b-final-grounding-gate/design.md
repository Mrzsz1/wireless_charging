# P1-1B Final Grounding Gate — Design

## Current defect

`verify_and_repair_with_semantic` 对草稿 claim 完成语义验证并生成 repaired answer，
但后续 `apply_claim_verification` 仍把草稿的 unsupported/repaired/uncited 计数写入最终
`CitationValidation`。因此即使 repair 已删除不可靠陈述，持久化 gate 仍按错误对象失败。

## Target flow

1. **Draft Audit**：保留现有语义 claim verification 结果，仅用于生成器质量诊断和 run manifest。
2. **Deterministic Repair**：按 draft verdict 保留、删除或替换 claim；PartiallySupported 使用固定提示，不能携带完整原 claim。
3. **Final Audit**：对 repaired answer 再提取最终 claim；不调用 LLM。
4. **Deterministic mapping**：最终事实 claim 仅可精确映射到 draft 中 `Supported` 的规范化文本与引用集合；否则 unsupported。
5. **Evidence validation**：独立校验最终 cited evidence ID 属于当前 evidence set；unknown citation fail closed。
6. **Gate**：最终事实 claim 全 supported、coverage=1、unknown=0 且 completeness=true 才可持久化。

## Shared notice classification

在 grounding 共享层定义 `is_grounding_system_notice(text)`，识别四条固定提示。
`is_factual_claim`、Final Audit、trusted-history 过滤和测试复用同一函数，避免分类漂移。

## No-supported outcome

若 repair 后没有可保留的 supported factual claim，返回固定安全答案：

> 当前检索到了相关资料，但本轮生成内容没有形成可被证据可靠支持的结论。

Final grounding 状态为 `insufficient_supported_claims`，不允许持久化为 grounded answer，
也不进入 trusted history。可保留给用户显示，但真实 Grounded PASS 必须保持失败/阻塞。

## Answer contract

将 contract 输入扩展为 intent、execution mode、evidence coverage、has evidence：

- Direct：优先 1–3 条直接结论及证据边界，不要求完整 Research profile。
- Research/Exploratory：只要求当前证据覆盖的要素；未覆盖要素省略或使用系统提示，禁止自由补事实。

## Diagnostics and logging

- 产品日志仅记录 request hash、阶段、计数、状态、耗时和稳定错误码。
- 原始 claim 与证据片段诊断使用显式环境开关写入仓库外临时目录。
- 诊断内容只含相对/逻辑 ID，不含凭据或绝对路径；真实复跑核对后删除。

## Compatibility

- 尽量以向后兼容的可选/default 字段扩充 run manifest/report。
- 不修改生成预算、semantic verifier 策略、planner/state、retrieval/reranker/embedding、zero-evidence 或 frozen eval 数据。
