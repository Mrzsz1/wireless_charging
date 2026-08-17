# Design — 语义 Query Planner 与 Facet Agent

## Data contracts

新增 `qa/query_plan.rs`：

- `QueryPlan`: schemaVersion、answerProfile、restatedQuestion、facets、requiredKinds、minimumEvidence。
- `QueryFacet`: 开放稳定 ID、显示标签、若干 searchQueries、required、preferredKinds。
- `QueryPlanningInput`: resolved question 与首轮候选的有界、无秘密摘要。

所有字符串、数组长度、证据类型、最低数和总查询数在后端再次校验；Provider schema 不是唯一防线。

## Provider flow

1. 在只读快照内建立历史与 resolved question。
2. 用原问题的通用词项 + semantic channel 完成首轮召回。
3. Codex Provider 用独立、短时限、无流式 UI token 的 `codex exec --output-schema` 生成 Query Plan。
4. Planner 失败时使用无扩展的通用 fallback plan。
5. 根据首轮候选检查最低证据、requiredKinds 和 required Facets。
6. 仅对未覆盖 Facet 使用 LLM searchQueries 运行后续检索，最多仍受三轮上限约束。

最终回答继续使用用户选定模型/思考强度；Planner 不改变模型设置。

## Baseline terms

`query_terms` 只做通用 Unicode token、ASCII 词和有界中文片段提取。删除无线充电领域关键词表、组合规则和已知论文别名，避免换问题就失败。跨语言和语义改写由本地 E5 与 Planner 查询承担。

## Coverage

- 首轮用 Facet 查询与候选标题/摘要的通用 token overlap 发现已覆盖面。
- 未覆盖 Facet 的专门检索结果由后端直接标记为该 Facet 覆盖，不依赖再次解析文本。
- Graphify-only 候选不满足事实 Facet或 requiredKinds。
- 停止原因扩展为 facet sufficient / fallback / max passes 等稳定枚举。

## Failure / rollback

- Planner 超时、Codex 未登录、schema 拒绝、解析失败全部降级到 baseline plan。
- Planner 不能把完整历史引用或任意输出字段注入回答 prompt。
- 回滚时移除 planner hook 和模块即可恢复原检索循环；结构化回答/引用验证不变。
