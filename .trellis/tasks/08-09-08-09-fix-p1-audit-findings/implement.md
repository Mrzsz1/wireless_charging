# 实施计划：P1 缺陷修复

## 阶段 0：规划评审与任务激活

1. 用户评审 `prd.md`、`design.md`、`implement.md`。
2. 获得实施确认后执行：

   ```powershell
   python ./.trellis/scripts/task.py start 08-09-08-09-fix-p1-audit-findings
   ```

3. 读取 `trellis-before-dev`，重新加载 backend/frontend 规范和即将修改的完整源文件。
4. 记录基线：

   ```powershell
   git status --short
   git rev-parse --short HEAD
   cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
   npm run build --prefix apps/desktop
   ```

5. 若基线出现与审查报告不一致的失败，先记录并停止对应改动，避免把旧失败误归因于本任务。

## 阶段 1：修复 IDX-001（watcher rename）

### 1.1 修改事件分类

文件：`apps/desktop/src-tauri/src/repository_watcher.rs`

1. 保留 `ModifyKind::Name(RenameMode)`，不要只折叠成通用 `ChangeKind::Rename`。
2. 为旧路径和新路径分别执行 root/excluded/classify 检查。
3. `RenameMode::Both` 在任一端 interesting 时发出一条 change，聚合 `full_rebuild/graph_refresh`。
4. `RenameMode::From` 使用 Remove 语义；`RenameMode::To` 使用 Create 语义。
5. 对平台给出的 Any/Other 单路径 rename 保留存在性兜底。

### 1.2 提取增量索引 helper

文件：`apps/desktop/src-tauri/src/lib.rs`

1. 从 `process_repository_changes` 提取数据库应用逻辑。
2. helper 在单个 SQLite transaction 中处理：
   - Remove：删除 wiki ID、FTS、wikilink。
   - Rename：先删除 previous wiki，再按 current 是否存在决定 upsert/delete。
   - Create/Modify：文件存在时 upsert。
   - graph-only：跳过知识表写入。
3. Tauri command 保留 event emit 和 RepositoryState 统计更新。

### 1.3 添加 Rust 回归测试

1. `wiki/a.md→wiki/b.md`：旧 ID 不存在，新 ID 存在。
2. `wiki/a.md→raw/a.md`：旧 ID/FTS/wikilink 全部删除。
3. `raw/a.md→wiki/a.md`：新页面写入。
4. 单路径 Rename From：文件不存在时删除旧索引。
5. schema/core 路径 rename 仍正确触发 full rebuild。

### 1.4 阶段验证

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml -- --check
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml repository_watcher
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml incremental
```

回滚点：仅还原 `repository_watcher.rs` 和本阶段 `lib.rs` helper；测试夹具不得写真实 `wiki/`。

## 阶段 2：修复 STATE-001（repository identity）

### 2.1 数据库迁移

文件：`apps/desktop/src-tauri/src/lib.rs`

1. 在 `db_schema` 创建 `repository_metadata`。
2. 增加 canonical identity helper、metadata read/write helper。
3. identity 的字符串规范与 QA repository ID 规则保持一致，避免同一路径因分隔符或大小写产生两个身份。

### 2.2 调整打开顺序

1. `open_repository_state` 先验证并 canonicalize root。
2. 打开 DB 并运行 schema migration。
3. 读取 identity：
   - 相同：读取当前统计。
   - 缺失/不同：调用 `rebuild_connection`，成功后写 identity。
4. 全部成功后才替换 `state.root/state.db/indexed_pages` 并写 `repository.json`。
5. 删除启动阶段仅以 `page_count>0` 判断仓库正确性的分支；启动复用由 identity 结果决定。
6. 避免 `choose_repository` 在 identity mismatch 已重建后重复做无意义重建；保留用户显式“重建索引”的能力。

### 2.3 添加数据库测试

1. A DB + A identity 打开 A：复用正确页面。
2. A DB + A identity 打开 B：页面变为 B。
3. legacy DB 有页面但无 identity：重建为目标 root。
4. 重建前写入 chat session、compile run、app setting；重建后逐表断言仍存在。
5. rebuild 失败时 identity 不更新，旧 RepositoryState 不被替换。

### 2.4 阶段验证

```powershell
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml -- --check
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml repository_identity
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
```

### 2.5 Git 保存点

```powershell
git add apps/desktop/src-tauri/src/repository_watcher.rs apps/desktop/src-tauri/src/lib.rs
git commit -m "fix(desktop): keep repository index identity consistent"
```

回滚点：该 commit 同时回滚 watcher 与 identity 迁移；metadata 表残留对旧代码无影响。

## 阶段 3：建立前端纯状态测试入口

1. 新增 feature-local 纯函数：
   - `src/features/qa/completionState.ts`
   - `src/features/books/bookTarget.ts`
   - `src/features/graph/refreshState.ts`
2. 新增 `apps/desktop/tests/p1-state.test.ts`，使用 `node:test` 与 `node:assert/strict`。
3. `package.json` 新增：

   ```json
   "test:p1": "node --experimental-strip-types --test tests/p1-state.test.ts"
   ```

4. 测试先按缺陷复现编写并确认旧实现语义会失败，再接入组件实现。

验证：

```powershell
npm run test:p1 --prefix apps/desktop
```

## 阶段 4：修复 QA-001（完成幂等）

文件：

- `apps/desktop/src/features/qa/completionState.ts`
- `apps/desktop/src/features/qa/AskView.tsx`

步骤：

1. 实现按 message ID 去重并移除 local 临时消息的纯函数。
2. `AskView.applyCompleted` 统一调用该函数。
3. 增加 request ID guard，阻止第二条完成通道重复执行 evidence/session/refreshSessions 等副作用。
4. repositoryPath 变化时清理完成 guard。
5. 保持错误、取消、离线回答和 Luna 流式 token 行为不变。
6. 增加测试：同一结果应用两次、不同 request 连续完成、临时消息替换、已有历史消息保持顺序。

阶段验证：

```powershell
npm run test:p1 --prefix apps/desktop
npm run build --prefix apps/desktop
```

## 阶段 5：修复 QA-002（书籍目标导航）

文件：

- `apps/desktop/src/App.tsx`
- `apps/desktop/src/features/books/bookTarget.ts`
- `apps/desktop/src/features/books/CoreBooksView.tsx`

步骤：

1. 定义 `BookTarget` 与目标章节匹配函数。
2. App 保存 `bookTarget`。
3. `AskView.onOpenBook(bookId, chapterId)` 设置 target 并激活 books。
4. `CoreBooksView` 接收 target，在目标书籍章节目录加载后调用一次 `openChapter`。
5. 防止默认首章异步请求晚于目标章节返回后覆盖目标章节：使用 selected book/target token 或请求序列校验。
6. 给当前书籍/章节增加稳定 `data-testid`，供 GUI E2E 使用。
7. 测试完整 ID、短 ID、错误 book ID 和 target=null。

阶段验证：

```powershell
npm run test:p1 --prefix apps/desktop
npm run build --prefix apps/desktop
```

## 阶段 6：修复 GRAPH-001（图谱自动刷新）

文件：

- `apps/desktop/src/App.tsx`
- `apps/desktop/src/features/graph/refreshState.ts`
- `apps/desktop/src/features/graph/GraphView.tsx`

步骤：

1. App 新增 `graphRefreshVersion`。
2. polling 结果仅在 `status.graphRefresh` 时增加版本。
3. GraphView 增加 `refreshVersion` prop。
4. 将 `load` 包装为稳定 callback；effect 在刷新版本变化时读取 `graphOverview`。
5. 处理刷新时当前 query/selected/path 状态：保留 query 语义，清理已不存在的 selected/path，避免显示旧节点详情。
6. 测试 graphRefresh true/false 版本投影；GUI 环境可用时验证 graph-view 的刷新标记或节点计数变化。

阶段验证：

```powershell
npm run test:p1 --prefix apps/desktop
npm run build --prefix apps/desktop
```

## 阶段 7：前端集成与 GUI 回归

1. 扩展 `apps/desktop/e2e/gui-smoke.mjs`：
   - QA 完成后同一 message ID 只出现一次（fixture/离线模式可用时）。
   - book evidence 跳转后目标章节 test ID 正确。
   - graph refresh 后刷新版本或节点计数变化。
2. 无 GUI 前置条件时运行非 strict 并记录 SKIP；具备条件时运行 strict。

```powershell
npm run e2e:gui --prefix apps/desktop
# 具备 TAURI_APP_PATH、TAURI_DRIVER 后：
npm run e2e:gui:strict --prefix apps/desktop
```

3. Git 保存点：

```powershell
git add apps/desktop/src apps/desktop/tests apps/desktop/e2e/gui-smoke.mjs apps/desktop/package.json
git commit -m "fix(desktop): make research state updates idempotent"
```

回滚点：前端 commit 可独立回滚，不影响 SQLite migration。

## 阶段 8：Trellis 质量检查

读取并执行 `trellis-check`。完整门禁：

```powershell
python -m compileall -q tools tests
python -m unittest discover -s tests -p "test_*.py" -v
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml -- --check
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
npm run test:p1 --prefix apps/desktop
npm run build --prefix apps/desktop
npm run verify --prefix apps/desktop
npm run verify:p3 --prefix apps/desktop
npm run verify:p4 --prefix apps/desktop
npm run verify:p5 --prefix apps/desktop
git diff --check
git status --short
```

检查重点：

- Rust metadata migration 是否只清理派生表。
- rename 的旧路径/new path 是否覆盖所有事件模式。
- Channel + invoke 完成顺序交换后仍幂等。
- CoreBooksView 的默认首章请求是否会覆盖 target 请求。
- GraphView refresh 是否只由 graphRefresh 驱动。
- TypeScript command payload、camelCase 字段和 props 签名保持一致。

## 阶段 9：规范更新、最终提交与收尾

1. 使用 `trellis-update-spec` 记录两条可执行约定：
   - 派生 SQLite 索引必须带 repository identity，identity 切换后先重建再暴露状态。
   - 多通道异步完成事件必须按 request/message ID 幂等投影。
2. 更新本任务验收清单与实现记录。
3. 提交规范和任务文档：

   ```powershell
   git add .trellis/spec .trellis/tasks/08-09-08-09-fix-p1-audit-findings
   git commit -m "docs(trellis): record desktop state consistency contracts"
   ```

4. 确认工作区干净、提交日志完整。
5. 归档任务并记录 journal：

   ```powershell
   python ./.trellis/scripts/task.py archive 08-09-08-09-fix-p1-audit-findings
   python ./.trellis/scripts/add_session.py --title "修复桌面客户端 P1 状态一致性问题" --commit "COMMIT_HASHES" --summary "完成五项 P1 修复及回归测试"
   ```

## 实施停止条件

- 任何测试写入真实 `raw/`、`wiki/` 或 `schema/` 正文。
- repository identity 方案要求删除聊天/编译历史才能通过。
- 需要新增网络依赖或改变 Tauri command 公共契约。
- GUI strict 的失败来自真实断言而不是缺少环境前置条件。
- Git 出现本任务范围外的未提交修改。

出现停止条件时回到规划阶段更新 PRD/design，不以临时绕过继续实施。

## Execution record

- Task activated with `task.py start`; no raw/wiki/schema/graphify source files
  were modified.
- Baseline: Rust 23 tests and the desktop production build passed.
- IDX-001: `RenameMode::{From,To,Both,Any,Other}` classification now preserves
  old/new endpoints and applies remove/create/upsert semantics in one SQLite
  transaction. Added Rust rename regression tests.
- STATE-001: added `repository_metadata` and canonical repository identity;
  mismatched or legacy caches rebuild derived tables before active state is
  replaced. Added preservation assertions for chat and app settings.
- QA-001: completion events are claimed by request ID and merged by persisted
  message ID before side effects.
- QA-002: `BookTarget` now crosses `AskView -> App -> CoreBooksView`, supports
  full/short chapter IDs, and guards stale chapter requests.
- GRAPH-001: `graphRefreshVersion` is driven only by `graphRefresh`; GraphView
  reloads the current query and reconciles stale selection/path state.
- Verification: Python compileall/unittest, Rust fmt/test/clippy, Node P1
  tests, desktop build, verify/verify:p3/verify:p4/verify:p5, offline GUI smoke,
  and `git diff --check` passed. GUI strict was skipped because the environment
  has no `TAURI_APP_PATH`/`tauri-driver`.
