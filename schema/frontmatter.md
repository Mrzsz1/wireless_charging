# Frontmatter 规范

所有 wiki 页 YAML frontmatter。匹配字段的值必须是 [[vocab.yaml]] 中的 **id**（列表字段为 id 数组）。

## 通用字段

```yaml
---
type: source | concept | system-model | objective | method | dataset-or-sim | synthesis | problem | idea
title: ""                    # 展示标题（可中文）
status: draft | active | needs_review
epistemic: high | medium | low
tags: []                     # 可选；自由标签，不替代受控词表
updated: YYYY-MM-DD
---
```

## 匹配核心（solve / novelty 用）

适用于：`source` `system-model` `objective` `method` `problem` `idea`（以及需要时的 `synthesis`）

```yaml
scenario: []                 # vocab: scenario
entities: []                 # vocab: entities
constraints: []              # vocab: constraints
objectives: []               # vocab: objectives
method_family: ""            # vocab: method_family；method/source 常用
problem_class: ""            # vocab: problem_class
```

空值：用 `[]` 或 `""`，不要编造 id。

## source 额外

```yaml
year: 2024
venue: ""
doi: ""                     # 可选；有明确 PDF/出版页证据时填写
source_type: paper           # vocab: source_type
authors: []
paper_keywords: []          # 作者 Keywords / Index Terms 的原词或明显 OCR 校正；自由元数据
keyword_source: index_terms # author_keywords | index_terms | not_found
acquisition_method: manual_upload | auto_discovery
discovered_via: []           # auto 时记录 arxiv/openalex/tavily/serpapi；manual 可为空
discovery_run: ""            # auto 时指向 search-* 运行目录；manual 可为空
triage_status: promoted      # pending | selected | rejected | promoted
selected_by_user: true
acquired_at: YYYY-MM-DD
canonicalized_at: YYYY-MM-DD
pdf_path: "raw/canonical/xxx.pdf"
raw_md: "raw/canonical/xxx.md"
why_relevant: ""
ingest_status: pending_convert | pending_ingest | ingested | convert_failed
```

### book（专著）可选字段

`source_type: book` 时保留以下书目字段；章节正文存放在 `raw/canonical/<book-id>/chapters/`，不为每章创建 wiki source 页。

```yaml
editors: []
edition: ""
publisher: ""
isbn: ""
page_count: 0
chapter_count: 0
core_reference: true
chapter_index: "raw/canonical/<book-id>/chapter-index.json"
quality_report: "raw/canonical/core-books-quality.json"
```

### 来源追踪与状态不得混用

- `acquisition_method` 是永久 provenance：晋升 canonical、完成 A 编译后仍不改变。
- `triage_status` 只表示人工筛选；进入 canonical 后为 `promoted`。
- `ingest_status` 只表示转换/编译进度。
- 以上是操作枚举，不是领域匹配词，不写入 `vocab.yaml`。
- 自动发现候选的 JSON 也使用同名字段；A 编译须原样传播到 source 页。

### 论文关键词不得冒充受控词表

- `paper_keywords` 是可溯源的作者元数据，不要求使用 `vocab.yaml` id。
- `keyword_source` 必须说明来自作者关键词、Index Terms 或原文未提供；不得把摘要推断默认为作者关键词。
- 规范化、别名合并与领域导航见 [[domain-keywords]]；只有匹配字段确实缺词时才进入 `vocab-proposals.md`。

## method 额外

```yaml
subtype: method | algorithm
method_family: ""            # 必填（若可知）
```

## problem / idea 额外（B）

```yaml
inspired_by: []              # wikilink 路径或源 id 列表，至少一个
supports: []                 # 可选
user_confirmed: false        # 写入正式页前须为 true
claimed_at: YYYY-MM-DD       # 可选
```

## synthesis 额外

```yaml
covers: []                   # 相关 source 链接
gaps: []                     # 短文本 gap 列表（非 idea）
```

## maps / library-status

见 `wiki/maps/library-status.md` 专用字段；地图页可仅用：

```yaml
type: map
title: ""
status: active
---
```
