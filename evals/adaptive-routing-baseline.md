# Adaptive Query Routing 基线

| 路径 | Planner 默认 | 最大检索轮数 | 最大查询数 | 最大候选数 | LLM call budget | token ceiling |
|---|---:|---:|---:|---:|---:|---:|
| DirectQA | 否 | 2 | 4 | 40 | 1 | 8,000 |
| ResearchQA | 是 | 2 | 12 | 80 | 3 | 18,000 |
| ExploratoryResearch | 是 | 3 | 20 | 120 | 5 | 32,000 |

## 验收

- 简单路径 query/candidate/LLM/token 预算最低：PASS
- 复杂路径允许 coverage-driven multi-round retrieval：PASS
- Exploratory 禁止超过三轮：PASS
- 显式 legacy planner callback 保持兼容：PASS
- quality：冻结 RAG evaluation PASS
- latency：以 `retrievalDiagnostics.totalMs` 实测，以本表 ceiling 控制最坏路径
- LLM calls / token cost：`QaRunManifest` 记录预算上限
- Rust：186 PASS / 0 FAIL / 1 ignored（需本地 semantic model）
