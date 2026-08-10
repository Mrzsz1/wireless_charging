# LLM Wiki P0/P1 实施与验收报告

## 范围

本轮实施对应 `prd.md` 13.20 和 Trellis 任务
`08-11-llmwiki-depth-retrieval`，目标是修正知识库水位漂移，增加论文原文章节召回，
并建立可复用的系统模型、目标与仿真证据层。

## P0 完成项

1. 统一 `wiki/index.md`、`wiki/maps/map-home.md` 与
   `wiki/maps/library-status.md` 的实际水位：23 source、20 method、7 synthesis。
2. 修正 10 份回归答案的旧水位；`wiki_eval.py` 改为从
   `library-status.md` 动态校验 source 数。
3. SQLite 派生索引新增 `paper_sections` 和 `paper_sections_fts`，
   仅读取 source 页声明的 `raw_md`，严格限定在 `raw/canonical/`。
4. 原文按 Markdown 标题分节，超长节按 6000 字符确定性分块，每块保留
   source page ID、章节名、raw/PDF 路径和起止行号。
5. 问答新增 `paper / primary_source` 证据类型，Wiki 摘要与同一论文原文
   可并存；前端显示“论文原文”和可核验的章节/行号。
6. 索引 schema 版本升级会重建派生知识表，但保留聊天、设置和编译历史。

## P1 完成项

- 新增 4 个 `system-model`、4 个 `objective` 和 1 个 `dataset-or-sim` 页面。
- 深化 CCSP、GAIN、TIDE、CUAV 联合调度轨迹、IHATRPO 五组 source/method，
  补全系统模型、变量、目标、算法、理论/复杂度、实验、定量结果、边界与 raw 行号。
- 更新 page type、A 编译和 Lint 契约，以研究字段完整性代替机械字数。
- `map-models-and-objectives` 改为“系统模型 → 目标 → 方法 → 证据”的任务路径。

## Graphify

- 直接执行 `graphify extract . --update` 时，headless CLI 因本机未配置其支持的
  LLM API key 而在写图前退出；未覆盖 Wiki 或原图。
- 按 Graphify skill 的 host-agent 语义抽取路径，对缺失的 17 个 Wiki 文档生成
  98 nodes、122 edges、3 hyperedges，与原图增量合并并重新聚类。
- 新图：2828 nodes、4822 edges、215 communities；严格 Lint 已确认 75 个 Wiki 页全部纳入。

## 验收结果

| 门禁 | 结果 |
|---|---|
| Wiki Lint strict Graphify | 75 pages，0 errors，1 个既有 B 类 `inspired_by` warning，0 断链，0 孤页 |
| Wiki Eval | 10/10 PASS |
| 核心书籍检索 | 295 queries；AGT Recall@5 1.000，AA Recall@5 0.986667，通过 95% |
| Python unittest | 46/46 PASS |
| Rust fmt / Clippy | PASS；Clippy `-D warnings` |
| Rust tests | 56/56 PASS；另外的中英文论文原文定向回归 1/1 PASS |
| Frontend build | TypeScript + Vite PASS，1823 modules |
| canonical raw 覆盖 | 21/21 非 book source 的 `raw_md` 存在 |

## 留存事项

- `wiki/problems/prob-joint-deployment-online-interference.md` 缺少 `inspired_by`。该页属 B 类，
  本轮遵守“未经用户确认不改 B 类正文”约束，故保留为唯一 warning。
- 本轮只深化五组高价值样板，其余 source/method 按新 schema 在后续批次逐页编译。
