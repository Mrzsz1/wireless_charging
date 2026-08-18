# Markdown 多粒度语料索引实施计划

## Phase 0 — 前置与 Git 检查点

- [x] 运行 `git status --short`，确认用户未跟踪文件不纳入任务。
- [x] 阅读父任务三份计划、`.trellis/spec/backend/qa-contract.md`、`schema/frontmatter.md`、`schema/page-types.md`、`schema/writing-rules.md`。
- [x] 开始前 Git SHA 为 `4400893`。

## Phase 1 — 冻结类型和迁移

- [x] 新增 `DocumentKind`、`DocumentRecord`、`ContentGranularity`、`ContentRole`、`ContentBlock`、`SourceLocator`。
- [x] 在 `lib.rs` 增加 v2 schema 和非破坏 migration；不删除旧表。
- [x] 增加 schema version/index snapshot metadata。
- [x] Rust serde 与 TypeScript camelCase 契约已接入并通过编译。

## Phase 2 — Markdown 解析器

- [x] 复用已索引 frontmatter，并对正文执行独立结构解析。
- [x] 实现 ATX/Setext 标题树、块边界、行号和父子关系。
- [x] 实现段落/围栏安全切块、长段分割和重叠窗口。
- [x] 实现 17 类角色加 `general_content` 初始分类。
- [x] 单元测试覆盖标题树、Setext、围栏代码、长段、角色和行号稳定性；既有 frontmatter/CRLF/索引测试保持通过。

## Phase 3 — 文档发现、别名和去重

- [x] 为 Wiki、source `raw_md` 论文和专著 chapters 实现 discovery adapter。
- [x] 复用 source/raw provenance，并按 raw content hash 去重论文正文。
- [x] 建立 title/frontmatter/wikilink alias 来源与规范化索引。
- [x] 真实仓库回归验证统一发现；重复论文 path/hash 计入构建诊断。

## Phase 4 — FTS 与 snapshot

- [x] 写入 documents/aliases/blocks/FTS。
- [x] 未变 hash 跳过；变化文档局部替换；移除文档停用；构建失败时外层事务保留旧 snapshot。
- [x] 在 repository watcher/index rebuild 路径接入 v2。
- [x] CorpusBuildStats 记录文档/块、更新/复用/停用和重复论文数。

## Phase 5 — Locator API

- [x] 实现路径边界校验和 `resolve_source_locator` Tauri command。
- [x] 实现 block -> heading -> line -> document fallback。
- [x] 添加 Rust/TypeScript `ResolvedSourceLocation` 定位返回值；本子任务未改 UI。

## Phase 6 — 验证与交接

- [x] 运行 parser/corpus/locator 目标测试。
- [x] 用真实仓库临时 SQLite 构建，核对 Wiki/paper/book、两书章节、语义块、别名和 locator。
- [x] 连续重建、局部修改和删除 fixture 验证增量行为。
- [x] `cargo fmt --check`。
- [x] `cargo test --lib`：141/141。
- [x] `cargo build --release`；同时 `npm run build` 成功。
- [x] 提交代码并在任务 notes 记录 schema version、迁移和下游可用类型。

## 风险文件与回滚点

- `apps/desktop/src-tauri/src/lib.rs`：索引事务和 schema；先新增 v2，避免原地破坏。
- `apps/desktop/src-tauri/src/qa.rs`：只接 typed adapter，避免同时重写检索。
- `apps/desktop/src/types.ts`：新增可选契约，旧 UI 暂不删除字段。
- 回滚：关闭 `rag_index_v2` 并继续读旧表；v2 表可保留作为未使用派生物。
