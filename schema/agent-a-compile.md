# 外部 Agent：A 编译操作规程

供 Cursor / Claude Code 等执行。问答请用户用 Claudian + `claudian-*.md` 模板。
总宪法：`AGENTS.md`。范式：`schema/references/karpathy-llm-wiki.md`。

## 开工前必读

1. `AGENTS.md`
2. `prd.md`
3. `schema/writing-rules.md`
4. `schema/page-types.md`
5. `schema/frontmatter.md`
6. `schema/vocab.yaml`
7. `schema/ingest-checklist.md`

## 输入

- 用户指定的 `raw/canonical/*.md`（MinerU 已转换，`pending_ingest`）
- 或：「编译所有 pending_ingest」

## 步骤

1. **列出待处理文件**，只读 canonical；跳过 `convert_failed` 与已 `ingested`（除非用户要求重编译）。
2. **逐篇**：读 raw md → 写/更新 `wiki/sources/src-<slug>.md`。新 source 与重点重编译页执行 `page-types.md` 的研究档案结构：模型、变量、目标/约束、算法、理论、实验、数值、局限与证据位置缺一项时须解释“原文未报告”，不能只写摘要卡。
3. **抽取实体页**：仅当内容足够且可链回该 source；复用已有 concept/method，避免重复页。跨两篇以上 source 复用的模型、目标和实验协议优先写入 `system-models/`、`objectives/`、`datasets-sims/`，单源细节留在 source。
4. **论文关键词**：优先抽取作者 `Keywords` / `Index Terms` 到 `paper_keywords`，写明 `keyword_source`；原文没有则写 `[]` / `not_found`，不得默猜。按 `schema/domain-keywords.md` 更新既有 `map-domain-keywords`。
5. **词表**：只用已有 id；缺词 → `vocab-proposals.md`，正文可用自然语言 +「待入库」。论文关键词不得自动晋升受控 id。
6. **synthesis**：相关主题追加并列；不裁断。method 不能只是单篇 source 的三行摘录；应写清输入输出、算法步骤、保证/复杂度、失效边界与 raw 锚点。
7. **maps**：只给**已存在**的 map 补 `[[links]]`；新主题列入待确认，不新建文件。
8. **library-status.md**：更新计数、年份、关键词覆盖、last_ingest_at、最近 ingest 列表。
9. **wiki/index.md**：为每个新/改 source 与主要实体页追加一行摘要。
10. **来源追踪**：把 raw 的 `acquisition_method`、`discovered_via`、`discovery_run`、`triage_status`、时间字段原样传播到 source 页；缺失时标 `needs_review`，不得猜成 auto。
11. **raw md**：`ingest_status: ingested`；不改变 `acquisition_method`，canonical 的 `triage_status` 为 `promoted`。
12. **logs/log.md**：追加 `## [日期] ingest | 标题`；可选 `logs/YYYY-MM-DD-*.md` 详情。
13. **桌面端原文索引**：source 的 `raw_md` 必须指向 canonical 内可读文件；桌面端重建索引会只读生成章节块。不得把 Markdown 行号冒充 PDF 页码。
14. **Graphify**（若已安装 CLI）：在 vault 根执行
    `graphify extract . --update`
    并在 log 记一笔 `graphify`。失败则注明「待用户本机重建」，不阻塞编译交付。
15. **停止线**：不写 `wiki/problems`、`wiki/ideas`，除非用户本轮明确要求并确认写入。
    **禁止** `graphify … --wiki` 覆盖本库 `wiki/` 目录结构。

## 输出给用户的摘要格式

```text
## A 编译完成
- 处理 raw：…
- 新建 wiki：…
- 更新 wiki：…
- index / library-status / log：已更新
- Graphify：已 --update / 跳过（原因）
- 词表提案：…
- 待你确认：…（新 map / 重命名 / 合并等）
- 日志：logs/…
```

## 禁止

- 网页/blog/PPT 源
- 自动外搜写进 wiki
- 无确认写 B 类页
- 擅自改 `vocab.yaml`
- 删除用户文件（除非明确要求）
- 手改 `graphify-out` 冒充知识正文
