# 写作与操作红线

与 [[../prd|prd.md]] 一致；违反则不得写入 wiki。

## A / B 隔离

| 区域 | 允许 | 禁止 |
|------|------|------|
| A 页（source…synthesis） | 忠实编译、对照、并列主张、标 gap | 「我们将贡献」「我的 idea 是」「本文新颖之处在于（指用户）」 |
| B 页（problem/idea） | 研究问题、候选思路、与文献重叠分析 | 写成已验证实验结论；无来源锚点 |

## 事实与锚点

1. 无 `raw/canonical` 已 ingest 锚点，不得把内容写成高置信硬事实。  
2. 关键句尽量能回溯到 `[[src-...]]` 或 raw md 章节。  
3. 专利等：`epistemic: medium`（或词表约定），B 阶段降权。

## 冲突

- **并存不裁断**：只写「文献 A 在设定 X 下主张…；文献 B 在设定 Y 下主张…」。  
- 系统与 Claudian **不选边**。  
- `/solve` 并列多解法与前提，由用户裁决。

## 词表

- frontmatter 匹配字段 **只写** `schema/vocab.yaml` 中已有 **id**。  
- 缺词 → `vocab-proposals.md`，用户确认前 **不得**当正式 id 使用（可用正文自然语言描述，并注明「待入库」）。
- source 的 `paper_keywords` 是作者元数据，可保留自由文本；它只能扩展 [[domain-keywords|领域关键词地图]]，不能绕过提案流程直接扩写 `vocab.yaml`。

## 外搜

- 通用网页搜索与 `/novelty` 实时查新默认禁止；用户明确批准后方可。  
- 用户已配置并授权的 `tools/paper_search.py` 可周期运行，但只可写 `raw/inbox/auto-discovered/` 候选。  
- 所有外搜结果带 `retrieved_at` + 文献 `year`。  
- 外搜内容不得直接升格为 wiki 事实；先 inbox 或问答报告。

## 采集来源与生命周期

- `manual_upload` / `auto_discovery` 是来源；`pending` / `selected` / `promoted` 与 `pending_ingest` / `ingested` 是状态。
- 自动发现与手动投放共用 canonical → A 编译链，但 provenance 不得在移动或编译时丢失。
- 自动发现候选即使下载了 PDF，也仍不是 canonical；必须由用户选择。

## 自动 vs 确认

| 可自动 | 需用户确认 |
|--------|------------|
| A 编译写入/更新 A 类页 | problem / idea 写入 |
| 已有 map 补链接 | 新建 map 主题 |
| 更新 library-status | 删除、合并、改关键 claim |
| 写 logs | 通用外搜 / novelty 实时查新 |
| 已授权论文发现任务写 inbox | 自动晋升搜索候选 |

## 日志

每次外部 agent 批量运行应在 `logs/` 增加 `YYYY-MM-DD-HHMM-简述.md`，列出：

- 触发原因  
- 读取的 raw 文件  
- 新建/修改的 wiki 路径  
- 词表提案（如有）  
- 待用户确认项  

## 语言

- 正文主语言：**中文**  
- concept：中文名 + 英文术语  
- 专有名词、论文标题、venue 保留英文  
