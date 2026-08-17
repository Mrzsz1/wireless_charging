# Design — 语义 RAG 回归评估

## Fixture

`evals/semantic_rag_questions.json` 保存：问题、严格 QueryPlan、期望 Wiki ID、期望 primary paper page ID。计划模拟 Codex 已通过 Provider schema 返回的结果，使普通单元测试不联网、不下载模型，也不依赖登录状态。

## Runner

Rust 生产索引测试读取夹具，对每条用例调用带 planner hook 的真实检索准备函数，断言：

1. answer profile 与计划一致；
2. plannerUsed=true；
3. 所有 required facet 都在 coveredFacetIds；
4. 至少一个期望 Wiki 命中；
5. 每个期望 paper source 有非 `wiki_source_to_primary_fallback` 的原文证据和行号。

该测试覆盖 QueryPlan → FTS/Graph/语义通道融合 → 去重/多样性 → Wiki-primary 配对的跨层路径。

## Boundary

这不是 held-out 人工事实评审，也不替代 `heldout_questions.json` 的双盲审计；只证明结构化计划和检索回归。
