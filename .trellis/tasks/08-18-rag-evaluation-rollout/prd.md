# 科研 RAG 评测、迁移与灰度发布

## Goal

用真实 Markdown 语料和多类科研问法验证 RAG v2 的召回、来源覆盖、定位、降级、多轮上下文和回答呈现；完成旧索引/会话迁移、双读对比、性能门禁、回滚演练和最终编译发布。

## Requirements

### V1 回归类型

- 指定书/论文、开放 paper+book、Wiki 方法、隐式概念、中英文别名、跨文档比较、零证据和冲突证据。
- 问法变化不能只替换一个关键词；需要同义、语序、上下文指代和新术语样例。
- 不新增面向用户的数据集说明；fixtures 只保存问题、预期来源/通道/定位和边界。

### V2 评测指标

- source resolution accuracy。
- requested channel attempt rate。
- Recall@k / MRR / nDCG（按文档和块分别）。
- locator validity 和 evidence appendix integrity。
- zero-evidence false-negative / false-positive 分类。
- latency、round count、embedding reuse、remote fallback rate。
- 不宣称通过小型开发题集得到总体事实准确率。

### V3 双读与错误分析

- Legacy/v2 对同一问题运行，记录候选、通道、停止原因和最终证据差异。
- 错误至少分类为 source resolution、chunking、lexical、dense、fusion、reranker、coverage、answer 和 locator。
- v2 失败时能通过 feature flag 回滚。

### V4 数据迁移

- 旧知识索引可重建为 v2，旧会话/消息/证据不丢失。
- 旧 LUNAVEC1 可迁移或重算；失败不删除旧缓存。
- 远程 pgvector snapshot 与本地 repository ID 对齐。

### V5 发布质量

- 全部局部测试、前端 build、cargo release、Tauri bundle 和 GUI 冒烟通过。
- 设置、模型部署、后台任务、取消/超时、页面切换持续运行不回归。
- 更新 PRD、Trellis spec、日志和版本记录。

## Acceptance Criteria

- [ ] AC1：两个移动路径问题均命中真实《近似算法》Markdown 章节；开放问法同时尝试 book/paper。
- [ ] AC2：至少包含一个从未写入检索代码的新概念回归，证明无专用特判。
- [ ] AC3：所有 expected evidence 的 locator 可打开；PDF 缺失样例同样通过。
- [ ] AC4：planner/reranker/semantic/remote DB/Graph 分别故障时降级路径通过。
- [ ] AC5：长会话压缩后仍能解析指代且不带入 failed/unverified 回答。
- [ ] AC6：legacy 会话数、消息数和证据数迁移前后相同；知识索引可独立重建。
- [ ] AC7：双读报告说明 v2 改善、退化和剩余风险，不只给单一百分比。
- [ ] AC8：无真实密钥进入 fixture、日志、SQLite 或审计导出。
- [ ] AC9：父任务 AC1–AC13 有逐项验收证据。
- [ ] AC10：最终 Rust、Python、前端、Tauri release 编译全部成功并记录退出码。

## Out of Scope

- 建立公开 benchmark 或正式科研数据集说明。
- 默认外网检索准确率评测。
- 语义蕴含自动评分。
