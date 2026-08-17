# Implement — 语义 Query Planner 与 Facet Agent

1. 新建 query plan DTO、严格 schema、prompt、解析/归一化和覆盖判定。
2. 将检索准备拆为首轮 baseline → planner hook → 未覆盖 Facet 扩展。
3. 在 `ask_luna` 的 Codex 路径接入短时限 Provider-native planner，失败降级。
4. 删除固定 intent 关键词路由与领域硬编码 query expansion。
5. 将计划版本、Facet 覆盖摘要加入可审计运行信息或检索查询 DTO。
6. 添加结构化计划、无效输出、fallback、Facet 扩展与未知问法回归测试。
7. 更新 QA contract，运行 Rust 全测与前端构建。
8. Git 提交并归档子任务。
