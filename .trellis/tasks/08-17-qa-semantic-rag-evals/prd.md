# 语义 RAG 回归评估

## Goal

建立与具体 UI 问句解耦的语义改写/QueryPlan 回归集，验证未知中文表达经开放 Facet 和双语检索计划后仍能召回可定位的 Wiki + primary paper 证据。

## Requirements

- 用例问题不得复用生产硬编码短语或已知完整问题。
- 每例保存独立的 Provider 结构化计划夹具，而不是把映射写进生产检索代码。
- 覆盖物理干涉、移动轨迹、在线定向控制、付费协作服务和跨模型比较等不同语义面。
- 校验 answer profile、必需 Facet 覆盖、Wiki 命中、非 fallback primary paper 命中和行号定位。
- 评估集明确标记为开发回归，不声称端到端事实准确率。

## Acceptance Criteria

- [ ] 新增至少 5 条陌生表述的语义/QueryPlan 回归用例。
- [ ] 每条计划符合 `qa-query-plan-v1` 的边界。
- [ ] 每条用例召回预期 Wiki 与可核验 primary paper。
- [ ] 计划 Facet 在 RetrievalQuery 中记录为 covered。
- [ ] 全部 Rust 测试、前端构建和最终 Tauri 编译通过。
