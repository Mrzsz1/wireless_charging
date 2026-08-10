# Lint 检查清单（Karpathy Health Check）

定期或在大批 ingest 后运行。产出 `logs/` 下报告并追加 `logs/log.md`。

## 检查项

### 1. Schema 完整性

- 页是否缺必填 frontmatter：`type` `title` `status`
- source 是否缺 `year` `source_type` `pdf_path`/`raw_md`
- source 是否缺 `acquisition_method`；auto 项是否缺 `discovered_via` / `discovery_run`
- source 是否缺 `paper_keywords` / `keyword_source`；关键词能否回溯到作者 Keywords / Index Terms
- `map-domain-keywords` 是否漏收已有 source 关键词，或把摘要推断伪装成作者关键词
- `triage_status`、`ingest_status` 是否混用或与所在层冲突
- 匹配字段是否出现 **未入库** 的 vocab id → 记入 proposals 或修正

### 2. 过期 / 漂移

- 最旧的 5–10 页是否被更新文献 superseded（只标记，不擅自改结论）
- `wiki/index.md` / `library-status` 是否与真实文件数一致；manual/auto canonical 与 pending/selected 候选计数是否一致
- `map-home` 链接是否失效

### 3. 覆盖缺口

- 正文反复出现但无独立页的 concept / method
- concept / method 是否仅被 1 个 source 使用且不属于真实问答所需的核心锚点
- system-model / objective / dataset-or-sim 是否只复述单篇 source、没有跨文献复用价值
- synthesis 是否至少覆盖 2 个 source，并含真实对照维度与 gap
- 只建议创建，**不**自动建 B 类页

### 3.1 研究档案详细度（2026-08-11 起）

- source 是否有：TL;DR、使用边界、模型/假设、目标/约束、算法、理论性质、实验/基线、定量结果、局限、证据定位
- method 是否有：输入输出、变量、算法步骤、复杂度/保证、适用/失效条件、来源锚点
- 原文未报告的字段是否明确标注，而不是省略后让回答模型猜测
- system-model / objective / dataset-or-sim 是否至少覆盖两个 source 或一个核心理论锚点
- raw 位置只写可信章节/行号；没有页映射时不得伪造 PDF page

### 4. 孤儿页

- 无入链的 wiki 页（除 maps/index 自身）
- 建议应从哪些 map/source 补链

### 5. 重复

- 近名/近标题 source 或 method
- **不删除**，列待用户确认合并

### 6. A/B 污染

- A 类页是否出现「我们将贡献」「我的 idea」
- idea 是否缺 `inspired_by` / 来源链接
- idea 的 `user_confirmed` 是否为 false 却当正式结论引用

### 7. 冲突表述

- 是否出现系统选边措辞（违反「并存不裁断」）

### 8. Graphify 一致性（若已建图）

- 运行 `graphify extract . --update` 后浏览 `GRAPH_REPORT.md`
- 记录 god nodes、surprising connections、建议问题
- 图与 wiki 严重不一致时以 **wiki 正文** 为准并修链

## 硬规则

- 不擅自删文件  
- 不擅自写 problem/idea  
- 可修无歧义 frontmatter  
- 追加 `## [日期] lint | …` 到 `logs/log.md`  

## 报告模板

```markdown
# Lint Report — YYYY-MM-DD

## Summary
🟢 / 🟡 / 🔴

## 1–8 分节
...

## Next Steps
1. 需用户确认：…
2. Agent 可自动修：…
```
