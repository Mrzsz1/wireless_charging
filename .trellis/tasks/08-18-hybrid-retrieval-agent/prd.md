# 混合检索、重排与有限 Agentic 补查

## Goal

用开放的 RetrievalContract、来源解析、独立混合检索通道、RRF/reranker 和软覆盖控制替换当前固定 answer profile、截断查询词和 `minimumEvidence` 停止逻辑，使科研问题在换词、跨来源和隐式概念情况下仍能得到高召回、可审计的证据候选。

## Requirements

### H1 RetrievalContract

- Provider JSON Schema 表达 scope、explicit sources、requested/must-attempt kinds、concepts、aliases、related problems、facets 和 budget。
- 不包含固定 `solve/novelty/relationship/literature` answer profile。
- 不包含硬性的 `minimumEvidence`。
- Planner 失败时保留原问题完整文本和全部可能请求来源，不能只生成首部中文 n-gram。

### H2 来源解析

- 先用 title/alias 索引解析显式书名/论文名。
- source-constrained 查询把 document IDs 下推到 FTS/vector 过滤。
- open 查询按 requestedKinds 分别尝试 paper/book/wiki。
- 未解析显式来源必须记录 gap，不能用其他来源结果冒充完成。

### H3 独立通道

- title/alias、FTS/BM25、dense、metadata-filtered 和 graph 每轮独立记录。
- 通道状态区分 not requested、attempted zero hit、succeeded with hits、degraded 和 failed。
- Graph 只能扩展候选，最终正文证据必须映射到 active ContentBlock。

### H4 融合与重排

- 通道内分数不直接跨量纲相加；使用 RRF。
- reranker 输入问题、来源标题、heading path、role 和 content。
- 显式来源强命中受到保护；reference-only 和 graph-only 降权。
- 使用 document diversity 和 semantic similarity 去重，允许同一关键来源保留互补块。

### H5 软覆盖与补查

- Coverage 评估通道尝试、显式来源、facets、冲突、gaps 和 new gain。
- 不输出事实真假的“充分/不充分”二值结论。
- Agent 最多补查两轮，每轮查询有数量/长度/来源边界。
- 连续无新增、预算耗尽、取消或所有请求面已尝试时停止并记录原因。

### H6 审计与性能

- 记录每轮查询、通道耗时/错误、候选排序变化、reranker 和停止原因。
- 首轮证据可先返回 UI，后续补查更新进度。
- 所有阻塞检索继续在 worker 中运行并响应取消。

## Acceptance Criteria

- [ ] AC1：新 planner schema 不包含 answerProfile/minimumEvidence，且有完整 JSON 示例和 Provider 原生约束。
- [ ] AC2：原问题的尾部核心概念不会因 n-gram `.take()` 或总 term truncate 静默消失。
- [ ] AC3：指定《近似算法》的问题只在目标书范围检索正文，同时可以使用别名扩展 TSP/path planning。
- [ ] AC4：开放“文献或者哪本书”问题分别记录 paper/book attempted 状态。
- [ ] AC5：仅 Wiki 返回 4 条弱结果不会使未尝试 book/paper 的检索提前停止。
- [ ] AC6：Graph-only、reference-only 和 query-unmatched fallback 不会成为高置信正文证据。
- [ ] AC7：未见过的新术语通过 dense + LLM expansion 工作，不修改代码词表。
- [ ] AC8：Planner、reranker、semantic 或 Graph 任一失败均有明确降级且不阻断其他通道。
- [ ] AC9：最多首轮加两轮补查，取消/超时有效，审计 stopReason 正确。
- [ ] AC10：Rust 检索回归和 release build 通过。

## Out of Scope

- 最终回答固定结构、逐 claim 引用或语义蕴含。
- 默认外网检索。
- Markdown 解析、向量生成和证据 UI。
