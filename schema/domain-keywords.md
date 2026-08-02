# 论文关键词与领域关键词治理

论文关键词用于扩展领域导航，但不替代 `schema/vocab.yaml`。本库采用三层结构：

```text
论文作者关键词（paper_keywords）
  → 规范化领域关键词（wiki/maps/map-domain-keywords.md）
  → 匹配字段缺词时的 vocab-proposals.md
  → 用户确认后才可进入 vocab.yaml
```

## 1. 论文原始层

- `source.paper_keywords` 保存论文 `Keywords` / `Index Terms` 的原词或仅做明显 OCR 校正的近原词。
- `source.keyword_source` 优先取 `author_keywords` 或 `index_terms`；原文未提供则写 `not_found`，不要根据摘要默默猜测。
- 原始关键词允许自由文本、缩写和大小写，不要求是受控词表 id。
- 每个关键词必须能通过 source 的 `raw_md` 回溯；边界文献的关键词不因进入地图而自动成为核心领域词。

## 2. 领域导航层

`wiki/maps/map-domain-keywords.md` 将作者用词归并为可浏览的规范概念，并保留：

1. 规范中文/英文名称；
2. 作者原词与常见缩写；
3. 支撑它的 `[[src-...]]`；
4. 出现文献数和范围标签（核心 / 邻域 / 边界）。

规范化只合并拼写、单复数、缩写和明确同义词。不得把语义不同的词强行合并，例如 `wireless charging`、`wireless power transfer` 与 `wireless-powered network` 可以关联，但仍保留各自原词。

## 3. 受控匹配层

- 只有 frontmatter 的 `scenario`、`entities`、`constraints`、`objectives`、`method_family`、`problem_class`、`source_type` 使用 `vocab.yaml` id。
- 关键词高频不等于必须进入受控词表。只有当真实 `/solve`、`/novelty` 或页面匹配字段缺少表达能力时，才写 `vocab-proposals.md`。
- 提案必须说明 dimension、来源论文、别名和具体检索需求；用户确认前不得写入 `vocab.yaml` 或匹配字段。

## 4. 增量更新规则

每次 A 编译：

1. 从 canonical 正文抽取作者关键词并写入 source；
2. 运行 `py -3 tools/domain_keywords.py --check` 检查字段与地图覆盖；
3. 将新词补入领域关键词地图，保留原词和 source 链；
4. 仅在匹配字段确实缺词时追加 vocab proposal；
5. 更新 `library-status` 的关键词覆盖计数并写日志。

推荐关注两类增长信号：同一规范词被至少 2 篇 source 支撑，或单篇词已经成为正式 problem / 高频问答的必要检索入口。前者可提升为核心导航词，后者可保留为问题驱动词；两者都不自动获得“领域共识”地位。
