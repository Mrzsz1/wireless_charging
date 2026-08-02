
## [2026-08-01] ingest | 两本核心算法专著章节化入库

- 用户指定 `Algorithmic game theory-book.pdf` 与 `Approximation Algorithms-book.pdf`；inbox 中的 `PDF_B.pdf` 通过版权页/书签确认是 Algorithmic Game Theory，`PDF_A.pdf` 确认是 Approximation Algorithms。
- 原始 PDF 保留不改；在 `work/core-books/inputs/` 做解密/工作副本，在 `work/core-books/*/parts/` 按章节拆成不超过 180 页的 MinerU 请求。
- MinerU 精确解析完成 61 个 parts（近似算法 31 个、算法博弈 30 个）；章节合并到 `raw/canonical/<book-id>/chapters/`，同时保留 `mineru/` 语义层。
- 为解决数学符号和 born-digital 文本的页级准确性，检索正文采用 Poppler page-faithful Markdown，MinerU 结果作为语义/图表复核层；未改 raw PDF。
- 质量门禁：两书 61 个章节均覆盖；最小 token recall 1.000；最小 token precision 0.956444（Approximation Algorithms）和 0.986173（Algorithmic Game Theory），均超过 95%。报告：`raw/canonical/core-books-quality.json`。
- 新增 `source_type: book`、书目 frontmatter、两张 source 页、核心专著综合页、检索注册表和 `tools/core_reference_search.py`；domain keyword 统计明确排除 book source，不影响 16 篇论文指标。
- 新增 3 个核心书测试；全套 unittest **30/30 通过**。
- 检索仍为确定性词法首轮；`evals/core-book-retrieval.json` 已放入 5 条种子查询，达到每本书 100 条人工复核后再宣称 Recall@5 ≥95%。
