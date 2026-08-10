# 深化 LLM Wiki 结构与检索 — 技术设计

## 1. 边界

本任务修改四个层面：产品/审查文档、Wiki/Schema、桌面端派生索引与问答召回、质量评测。`raw/` 仅作为只读输入；SQLite 与 `graphify-out/` 都是可重建派生物。

## 2. 目标检索架构

```text
问题
  ├─ L1 Wiki / Map / Graphify：发现候选模型、方法与关系
  ├─ L2 paper_sections_fts：召回论文原文章节和行号
  └─ L2 book_chapters_fts：召回核心书籍章节和 physical pages
          ↓
统一 Candidate 排序 → 按 kind 去重并保留渠道多样性 → EvidenceItem → 回答模型
```

### 2.1 SQLite 表

`paper_sections`：

- `id`：`<page_id>#<chunk_index>`，由相对路径和顺序稳定生成；
- `page_id`、`title`、`section_title`；
- `source_path`、`pdf_path`；
- `line_start`、`line_end`；
- `body`。

`paper_sections_fts`：`section_id` 为 UNINDEXED，索引 `title`、`section_title`、`body`。

### 2.2 分块

1. 按 Markdown H1–H4 标题开始新 section；frontmatter 不作为正文。
2. 在章节超过约 6,000 字符时，按空行段落累计切块；单个超长段落按字符安全边界切分。
3. 每块保留标题路径和原始起止行号，不复制或修改 raw 文件。
4. 找不到 `raw_md`、路径越界、目录或 book source 时跳过；Wiki 摘要仍可用。

### 2.3 检索与排序

- `paper_candidates` 使用和 Wiki/书籍相同的查询词及 FTS 前缀表达式。
- 论文原文候选 `kind=paper`、`tier=primary_source`，snippet 上限高于摘要页，用于具体约束、公式和实验事实。
- 去重键包含渠道；`wiki:<page_id>` 与 `paper:<section_id>` 可以同时存在。
- 最终包在有候选时至少保留 Wiki、paper、book 各一条；Graphify 不强制占位。
- `source_location` 使用“章节标题 · raw lines x–y”，`wikilink` 链回 source 页。

## 3. Wiki 结构

新增目录沿用现有 `wiki/system-models`、`wiki/objectives`、`wiki/datasets`。第一批页面不是论文摘要副本，而是多来源的复用节点。

页面采用共同阅读顺序：

1. TL;DR；
2. 何时使用 / 何时不使用；
3. 形式化定义；
4. 方法或比较；
5. 证据与原文定位；
6. 相关页面。

## 4. 一致性策略

- `library-status.md` 是水位权威；`index.md` 与 `map-home.md` 本轮同步到实际文件计数。
- Lint 增加详细度结构检查与 map 水位漂移检查，避免下次回退。
- Graphify 通过正式 CLI 更新，不手改派生 JSON。

## 5. 兼容与迁移

- `CREATE TABLE IF NOT EXISTS` 支持旧数据库原地升级。
- 完整重建先清除 paper 派生表；聊天、设置、任务历史不受影响。
- repository identity 逻辑保持不变；无需更改前端 DTO，复用现有 `EvidenceItem.sourceLocation`。

## 6. 风险与回滚

- 大 raw Markdown 可能增加首次重建时间：只索引 source frontmatter 明确引用的 21 篇论文，并按章节切块。
- MinerU 标题质量不一：无标题块使用论文标题/“正文”，不猜页码。
- 无 PDF 页映射时只报告 raw 行号，不把 Markdown 行号冒充 PDF 页码。
- 回滚时可移除新表和 `paper_candidates` 调用；Wiki/Schema 修改独立保留。
