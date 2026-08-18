# 科研 RAG 评测、迁移与灰度设计

## 1. 评测矩阵

每个 case 使用最小契约：

```text
id, question, conversation?, scopeExpectation,
mustAttemptKinds[], expectedDocuments[], expectedBlocksOrHeadings[],
forbiddenEvidenceKinds[], locatorRequired, zeroEvidenceExpected?, notes
```

不要求固定答案中文短语，不把模型自然措辞作为召回正确性的代理。答案层只检查状态、证据附录、未核验边界和已知危险输出。

## 2. Case 族

1. source-constrained book：中文别名 + 隐式 TSP。
2. open literature：paper/book 同时请求。
3. paper source + model/method facet。
4. bilingual paraphrase。
5. multi-turn reference resolution。
6. graph hint only 不得冒充正文。
7. reference-only 降权。
8. true zero evidence 与 dense degraded 区分。
9. remote vector unavailable/local fallback。
10. locator drift and legacy session。

## 3. 双读

`RagComparisonRun` 同时保存 legacy/v2 的候选列表和诊断，但只把配置选中的版本发送给 answer provider。比较工具生成 Markdown/JSON 报告，按错误类别聚合。双读不得双倍调用付费 Answer Provider；只比较检索，必要时对固定 evidence 使用离线 answer fixture。

## 4. 门禁

硬门禁只针对可验证不变量：请求通道尝试、来源/块命中、locator 有效、未知证据拒绝、无越界路径、迁移计数、降级状态和构建。排序指标设置“不得显著低于 baseline + 关键 case 必须通过”，避免用单一平均值掩盖指定来源失败。

## 5. 性能

记录冷/热索引、embedding 增量、首轮 retrieval、每轮补查、reranker、首 token 和总回答。远程免费实例冷启动单独记录，不作为本地词法通道失败原因。性能目标在真实仓库规模上确定，并把硬件/模型/存储状态写报告。

## 6. 迁移/回滚

1. 备份 SQLite/设置/本地向量 manifest。
2. 构建 v2 snapshot，不覆盖旧 active index。
3. 校验数量、hash、locators 和向量状态。
4. 开启 index v2，retriever 先双读。
5. 开启 answer v2。
6. 失败时按 answer -> retriever -> index 逆序关闭 flags。

## 7. 发布证据

最终任务 notes 写测试命令、退出码、耗时、报告路径、release artifact 和 SHA-256。项目 PRD 只记录已锁定架构与实际验收，不复制整份测试日志。
