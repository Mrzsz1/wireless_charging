# Markdown 多粒度语料索引 v2

- 触发原因：修复科研问答对书籍、论文和 Wiki 的统一召回基础，避免整章单向量和 PDF 页码依赖。
- 正文边界：只读取 `wiki/**/*.md`、source `raw_md` 和核心专著 chapters；未修改任何 Raw Markdown 正文。
- 新增派生表：`documents_v2`、`document_aliases_v2`、`content_blocks_v2`、`content_blocks_fts_v2`。
- 新增粒度：document、section、semantic；保存角色、标题路径、相对 Markdown 路径、内容哈希和 SourceLocator。
- 稳定定位：block → heading path → line range → document，全部经过仓库路径边界检查。
- 增量行为：内容哈希未变则复用；变化文档局部替换；移除文档停用；旧知识表与聊天表保留。
- 别名：核心专著 source frontmatter 增加 `近似算法`、`算法博弈论`，索引同时采集 title/frontmatter/wikilink alias 来源。
- 验证：Rust 141/141；真实仓库 Wiki/paper/book/Chapter 3/Chapter 11/locator 回归通过；前端 production build 通过；Rust release build 通过。
- 待后续：子任务 `08-18-embedding-pgvector-store` 使用 `content_blocks_v2.embedding_text` 和 content hash 生成多粒度向量。
