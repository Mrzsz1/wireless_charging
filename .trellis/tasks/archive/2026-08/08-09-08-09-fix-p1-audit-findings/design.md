# 技术设计：P1 状态一致性修复

## 1. 设计原则

1. **源文件是真相，SQLite 页面表是派生缓存**：仓库 identity 不一致时重建派生表，不迁移旧页面。
2. **事件必须幂等**：同一 request/变更通过多个通道到达时，状态投影只能应用一次。
3. **组件边界传递完整标识符**：book evidence 的 `bookId/chapterId` 不在中间层丢失。
4. **刷新信号最小化**：Graphify 只响应 graph 文件变化，不绑定所有知识库变化。
5. **优先扩展现有结构**：保留单个 `knowledge.db`、现有 Tauri command、现有 watcher polling 和现有 CoreBooksView target 机制。

## 2. 影响范围

### Rust/Tauri

- `apps/desktop/src-tauri/src/repository_watcher.rs`
- `apps/desktop/src-tauri/src/lib.rs`
- 相关 `#[cfg(test)]` 单元测试

### React/TypeScript

- `apps/desktop/src/App.tsx`
- `apps/desktop/src/features/qa/AskView.tsx`
- `apps/desktop/src/features/books/CoreBooksView.tsx`
- `apps/desktop/src/features/graph/GraphView.tsx`
- 新增少量与各 feature 同目录的纯状态 helper
- `apps/desktop/tests/p1-state.test.ts`
- `apps/desktop/package.json`
- 必要时扩展 `apps/desktop/e2e/gui-smoke.mjs`

## 3. IDX-001：rename 双端分类

### 3.1 事件分类

对 rename 的两个端点分别得到：

```text
previous: inside root? → excluded? → PathClassification
current : inside root? → excluded? → PathClassification
```

只要任一端点 interesting，就生成 `IndexChange`：

- `path`：当前路径；若只有 rename-from，则使用旧路径。
- `previous_path`：`RenameMode::Both` 时保存旧路径。
- `full_rebuild`：旧端或新端需要 full rebuild 时为 true。
- `graph_refresh`：旧端或新端指向 graph.json 时为 true。

单路径事件处理：

- `RenameMode::From` 映射为 Remove 语义。
- `RenameMode::To` 映射为 Create 语义。
- backend 增量处理对无 `previous_path` 的 Rename 使用文件是否存在兜底：存在则 upsert，不存在则删除旧 ID。

### 3.2 索引应用

把增量索引循环提取为可单测 helper（例如 `apply_index_changes`），输入 `Connection + root + &[IndexChange]`，输出当前 `IndexStats`。Tauri command 继续负责 watcher polling 和 event emit；helper 只负责数据库事务。

该分层允许直接用 temp repository + in-memory SQLite 验证：

- wiki→wiki：delete old + upsert new。
- wiki→raw：delete old，不 upsert new。
- raw→wiki：只 upsert new。
- rename-from 单路径：删除旧 wiki。

## 4. STATE-001：共享 DB 的派生索引 identity

### 4.1 数据库结构

在 `db_schema` 新增轻量 metadata 表：

```sql
CREATE TABLE IF NOT EXISTS repository_metadata (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
```

使用固定 key：`knowledge_index_repository_id`。值为 canonical root 的稳定 Windows identity：

1. `fs::canonicalize(root)`；
2. `\` 转 `/`；
3. Windows 路径按大小写不敏感规则规范化。

### 4.2 打开流程

```text
validate root
  → canonicalize root
  → open knowledge.db + migrate schema
  → read repository identity + page count
  → identity matches: reuse derived cache
  → identity missing/mismatch: rebuild derived tables from target root
  → write new identity
  → only now update RepositoryState and repository.json
```

兼容旧数据库：metadata key 缺失且页面表非空时，视为 identity 未知，执行一次重建；重建成功后写入 key。

### 4.3 保留数据

`rebuild_connection` 继续只清理知识派生表，不触碰：

- `chat_sessions` / `chat_messages` / `chat_evidence`
- `compile_runs` / `compile_run_events` / `compile_artifacts`
- `app_settings`
- `repository_metadata` 中非目标 key

identity 只在重建成功后更新。如果重建失败，`open_repository_state` 返回错误，并保持旧的内存 RepositoryState 与 `repository.json` 不变。

## 5. QA-001：完成事件幂等投影

### 5.1 状态 helper

在 QA feature 下新增纯函数，按持久化 message ID 合并完成结果：

```text
current messages
  → 移除 local-* 临时消息
  → 移除与 result.userMessage/result.assistantMessage 相同 ID 的旧条目
  → 追加一组持久化消息
```

### 5.2 副作用 guard

`AskView` 使用 request ID guard 记录最近已完成请求：

- 第一次完成：应用 messages/evidence/waterline/session/phase，并刷新会话列表。
- 相同 request 再次完成：直接返回，不重复刷新或追加。
- repositoryPath 变化时清理 guard 与本地 QA 状态。

保留 Channel `Completed` 与 invoke 返回两条通道，形成实时完成 + 返回值兜底，但通过同一个 `applyCompleted` 保证幂等。

## 6. QA-002：book target 贯通

### 6.1 App 状态

新增：

```ts
type BookTarget = { bookId: string; chapterId: string }
const [bookTarget, setBookTarget] = useState<BookTarget | null>(null)
```

`AskView.onOpenBook` 接收完整参数，设置 target 后激活 books。`renderContent` 把 target 传给 `CoreBooksView`。

### 6.2 章节匹配

将完整 ID/短 ID 的匹配提取到 books feature 的纯函数：

- `item.id === target.chapterId`
- `item.id === ${target.bookId}:${target.chapterId}`
- `item.id.endsWith(:${target.chapterId})`

CoreBooksView 先切换目标书籍，等待章节加载，再选择目标章节。手动导航时 target 为 null，保留默认首章逻辑。

## 7. GRAPH-001：独立刷新版本

### 7.1 App 投影

增加 `graphRefreshVersion`。每次 polling 返回：

- `graphRefresh=true`：版本 +1。
- 普通 wiki 变化：版本不变。

已有 `repositoryGeneration` 继续刷新页面目录，两者职责分离。

### 7.2 GraphView

新增 `refreshVersion` prop。`load` 使用 `useCallback`，effect 依赖 `load` 和 `refreshVersion`；版本变化时重新读取 `graphOverview`。手动重置和查询仍调用同一个 `load`。

## 8. 测试设计

### 8.1 Rust 测试

- watcher：Both/From/To、wiki→raw、raw→wiki、wiki→wiki。
- 增量应用：旧 page/FTS/wikilink 删除与新 page upsert。
- identity：same root 复用、A→B 重建、legacy 无 metadata 重建。
- 数据保留：重建后聊天、编译、设置记录仍存在。
- 失败路径：目标仓库无效或 rebuild 失败时 metadata/root 不推进。

### 8.2 前端纯状态测试

使用 Node 24：

```text
node --experimental-strip-types --test tests/p1-state.test.ts
```

测试：

- 同一 AskResult 应用两次仍只有一组消息。
- book target 能匹配完整与短 chapter ID。
- graphRefresh=true 才增加刷新版本。

不新增 Vitest/jsdom 等依赖。

### 8.3 GUI E2E

在现有 `gui-smoke.mjs` 中保留启动/导航测试，并为书籍选中章节、GraphView refresh 增加稳定 test ID。真实 GUI 断言仅在 `TAURI_APP_PATH` 和 WebDriver 前置条件具备时执行；其他环境由纯状态测试和构建门兜底。

## 9. 兼容性、迁移与回滚

- metadata 表使用 `CREATE TABLE IF NOT EXISTS`，旧数据库原地升级。
- 首次升级可能执行一次完整索引重建；聊天、编译记录和设置保持不变。
- 不改变 Tauri invoke command 或序列化字段，现有前端调用兼容。
- 任一阶段回滚可按 Git commit 粒度恢复：backend identity/watcher 与 frontend state/navigation 分为独立提交。
- 若 identity 迁移出现问题，可回滚相关提交；旧 DB 新增 metadata 表不会影响旧版本读取。

## 10. 主要取舍

| 方案 | 结论 | 原因 |
|---|---|---|
| 每仓库独立 SQLite 文件 | 本阶段不采用 | 会改变聊天、设置和编译历史布局，迁移范围过大。 |
| 共享 DB + repository identity | 采用 | 最小修改即可保护派生索引，同时保留现有历史表。 |
| 只移除 Channel 或 invoke 的一个完成通道 | 不采用 | 单通道更简单，但失去实时事件或返回值兜底。 |
| 双通道 + request ID 幂等 | 采用 | 保留现有 API，修复重复副作用。 |
| 所有知识库变化都刷新 GraphView | 不采用 | 产生无关图文件 I/O。 |
| 独立 graph refresh version | 采用 | 准确消费现有 `graphRefresh` 契约。 |
| 新增 Vitest/Testing Library | 本阶段不采用 | Node 24 内置测试足以验证纯状态逻辑，无需扩大依赖。 |
