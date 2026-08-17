# 语义 Query Planner 与 Facet Agent

## Goal

移除按固定中文关键词判断四类问题和维护领域问句映射的路由方式，改为 Codex 原生 JSON Schema 约束的开放式 Query Plan、多标签 Facet 覆盖与按需扩展检索。

## Requirements

- 问题类型、检索子问题、实体和证据覆盖面由 Codex 结构化规划，不由固定短语命中决定。
- 首轮始终使用原问题、通用分词和本地语义向量执行确定性基线召回。
- Query Planner 只能输出严格 schema 中的字段；无效、超时或不可用时安全降级，不阻断基线回答。
- LLM 生成的扩展查询只在首轮证据/Facet 覆盖不足时进入后续检索。
- Facet 是开放 ID/标签/查询集合，可表达论文、模型、方法、边界、来源定位以及未来未知维度，不限定四种意图。
- 每个必需 Facet、必需证据类型和最低证据数都参与停止条件；禁止仅用候选总数宣称充分。
- Planner 输入不含密钥、端点、完整历史或思维链；只包含 resolved question 和有界首轮候选摘要。

## Acceptance Criteria

- [ ] 删除基于问题中文/英文固定短语打分的 `intent(question)`。
- [ ] 删除领域问句到论文/术语的硬编码扩展，保留通用字词基线。
- [ ] Codex Query Plan 使用 `--output-schema`，能解析和约束开放 Facet。
- [ ] 首轮充分时不执行 LLM 扩展查询；不足时按未覆盖 Facet 执行有界检索。
- [ ] Planner 失败时仍能基于原问题完成检索。
- [ ] 诊断和运行清单可审计所用计划版本、Facet 覆盖和停止原因，但不暴露原始 Planner 响应。
- [ ] Rust 测试和前端构建通过。

## Notes

- 现有四种 answer profile 暂保留为回答结构模板；由结构化计划选择，不再由关键词路由选择。
