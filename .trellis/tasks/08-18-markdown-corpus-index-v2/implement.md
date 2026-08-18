# Markdown 多粒度语料索引实施计划

## Phase 0 — 前置与 Git 检查点

- [ ] 运行 `git status --short`，确认用户未跟踪文件不纳入任务。
- [ ] 阅读父任务三份计划、`.trellis/spec/backend/qa-contract.md`、`schema/frontmatter.md`、`schema/page-types.md`、`schema/writing-rules.md`。
- [ ] 提交或记录开始前 Git SHA；开始编码后保持小提交。

## Phase 1 — 冻结类型和迁移

- [ ] 新增 `DocumentKind`、`DocumentRecord`、`ContentGranularity`、`ContentRole`、`ContentBlock`、`SourceLocator`。
- [ ] 在 `lib.rs` 增加 v2 schema 和非破坏 migration；不删除旧表。
- [ ] 增加 schema version/index snapshot metadata。
- [ ] 为 Rust serde 与 TypeScript camelCase 写契约 fixture。

## Phase 2 — Markdown 解析器

- [ ] 实现 frontmatter 与正文分离。
- [ ] 实现标题树、块边界、行号、父子关系。
- [ ] 实现结构安全切块和 token 估算。
- [ ] 实现角色初始分类。
- [ ] 单元测试覆盖中英文标题、重复标题、无 H1、表格、公式、代码块、长列表、空小节和 CRLF。

## Phase 3 — 文档发现、别名和去重

- [ ] 为 Wiki、论文 Markdown、专著 chapters 分别实现 discovery adapter。
- [ ] 复用现有 source/raw provenance，禁止凭文件名盲猜同一论文。
- [ ] 建立 alias 来源字段与规范化索引。
- [ ] 增加重复 full/exported Markdown 的真实 fixture 回归。

## Phase 4 — FTS 与 snapshot

- [ ] 写入 documents/aliases/blocks/FTS。
- [ ] 未变 hash 跳过；变化文档局部替换；失败保留旧 snapshot。
- [ ] 在 repository watcher/index rebuild 路径接入 v2。
- [ ] 记录 blocks per granularity、roles、duplicates、failed documents。

## Phase 5 — Locator API

- [ ] 实现路径边界校验和 `resolve_source_locator`。
- [ ] 实现 block -> heading -> line -> document fallback。
- [ ] 添加 Markdown detail/open 命令所需的定位返回值；本子任务不做 UI。

## Phase 6 — 验证与交接

- [ ] 运行目标模块单元测试。
- [ ] 用真实仓库临时 SQLite 构建一次，核对文档数、块数、重复诊断和两本书章节。
- [ ] 连续重建两次验证增量命中；修改临时 fixture 验证局部更新。
- [ ] `cargo fmt --check`。
- [ ] `cargo test --lib corpus`、`cargo test --lib locator`（按实际模块名调整并记录）。
- [ ] `cargo build --release`。
- [ ] 提交代码并在任务 notes 记录 schema version、迁移和下游可用类型。

## 风险文件与回滚点

- `apps/desktop/src-tauri/src/lib.rs`：索引事务和 schema；先新增 v2，避免原地破坏。
- `apps/desktop/src-tauri/src/qa.rs`：只接 typed adapter，避免同时重写检索。
- `apps/desktop/src/types.ts`：新增可选契约，旧 UI 暂不删除字段。
- 回滚：关闭 `rag_index_v2` 并继续读旧表；v2 表可保留作为未使用派生物。
