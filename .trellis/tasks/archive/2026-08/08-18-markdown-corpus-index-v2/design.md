# Markdown 多粒度语料索引技术设计

## 1. 当前问题锚点

- `apps/desktop/src-tauri/src/lib.rs` 当前分离维护 `pages`、`paper_sections`、`books` 和 `book_chapters`，字段与检索粒度不一致。
- `book_chapters_fts` 只包含章节 title/body，书名和别名不参与 FTS。
- 论文证据依赖 line range；书籍证据依赖 PDF/physical pages；两者缺少统一稳定 locator。
- `qa/semantic.rs` 当前按页面/章节构造语义文档，无法精确定位长章节内部内容。

## 2. 新模块

建议新增：

```text
apps/desktop/src-tauri/src/qa/corpus.rs
apps/desktop/src-tauri/src/qa/markdown_parser.rs
apps/desktop/src-tauri/src/qa/locator.rs
```

`lib.rs` 只负责仓库级索引事务和命令注册，解析/ID/locator 逻辑不得继续堆入 `qa.rs`。

## 3. SQLite v2

```sql
documents_v2(
  id PRIMARY KEY, kind, canonical_title, markdown_path,
  authors_json, year, tags_json, provenance_json,
  content_hash, snapshot_id, active, updated_at
)

document_aliases_v2(
  document_id, alias, normalized_alias, language, source,
  PRIMARY KEY(document_id, normalized_alias)
)

content_blocks_v2(
  id PRIMARY KEY, document_id, parent_block_id,
  granularity, heading, heading_path_json, role, ordinal,
  line_start, line_end, markdown_path, content, content_hash,
  embedding_text, snapshot_id, active
)

content_blocks_fts_v2(
  block_id UNINDEXED, document_id UNINDEXED,
  canonical_title, aliases, heading_path, role, content
)
```

FTS 写入由单一函数从 typed records 生成，避免 SQL 分支之间字段漂移。

## 4. 文档发现与去重

1. `wiki/**/*.md`：每文件一文档，frontmatter `id/type/title` 优先。
2. `raw/canonical/**/chapters/*.md`：使用父目录/manifest 建 book document，章节文件成为 section roots。
3. `raw/canonical/**/full.md` 和论文导出稿：使用现有 source 页 `raw_md`/canonical provenance 优先选主稿；同 hash 文件合并；无法判定的重复项写诊断但只选择一个 active document。
4. 所有绝对路径只在运行时解析；数据库保存仓库相对 POSIX 路径。

## 5. ID 与定位

- document ID 优先 frontmatter/现有稳定 ID；否则由 canonical provenance 生成稳定 slug/hash。
- heading key 为规范化标题路径；同标题重复时追加同级 occurrence。
- semantic block ID 为 document ID + heading key + local ordinal + 短 content fingerprint。
- locator 打开时先查 active block；hash 不同则尝试同 document + heading path + fingerprint；再退到 line range。

## 6. 角色分类

先用标题词典和结构规则分类，例如 Abstract、Introduction/Background、Motivation、Objective、Model、Method/Algorithm、Experiment/Evaluation、Result、Limitation、Conclusion、References。规则独立于问答关键词，不决定是否召回，只作为元数据与排序信号。

## 7. 事务与 snapshot

- 在临时 snapshot 中解析并写入全部变化文档。
- 单文档失败记录错误，旧 active 版本保留。
- snapshot 完整校验后切换 repository metadata 的 active snapshot。
- 会话/聊天表不参与知识索引清理事务。

## 8. TypeScript 边界

向前端暴露 `SourceLocator` 和可选字段；旧 `EvidenceItem` 的 PDF/physical page 字段在兼容期保留，但不再 required。所有新增 Rust/TS 字段遵循 camelCase 序列化并由共享 fixture 验证。
