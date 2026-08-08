# 修复代码审查 P1 缺陷并补齐回归测试

## 目标

修复归档审查报告确认的 5 个 P1 缺陷，使桌面客户端在知识库文件变更、知识库切换、问答完成、书籍证据跳转和 Graphify 图更新时保持数据、状态与界面一致，并建立可重复执行的回归测试。

审查依据：`.trellis/tasks/archive/2026-08/08-09-code-audit-20260809/review-report.md`。

## 用户价值

- 文献库和智能问答只使用当前知识库中真实存在的页面。
- 切换或恢复知识库后不会混入其他知识库的索引结果。
- 每次问答只显示一组问题与回答。
- 点击书籍证据后直接定位到被引用的书籍章节。
- Graphify 文件更新后，已打开的图谱视图自动显示最新节点与关系。

## 已确认事实

- 当前桌面客户端使用 React/TypeScript + Tauri/Rust + SQLite。
- 页面索引、聊天记录、编译记录与模型设置共用应用数据目录中的 `knowledge.db`。
- `chat_sessions.repository_id` 和 `compile_runs.repository_path` 已具备仓库隔离字段；`pages`、`books`、`book_chapters` 等派生索引表没有仓库字段。
- watcher 已识别 `wiki/`、核心书籍、`schema/` 与 `graphify-out/graph.json`，但 rename 只依据目标路径决定是否发出变更。
- `ask_luna` 同时通过 Channel 发送 `Completed` 事件并返回 `AskResult`；前端对两条完成通道都会追加消息。
- `AskView` 已传出 `bookId/chapterId`，`CoreBooksView` 也支持 `target`，但 App 没有保存和传递该目标。
- Graphify 变化已通过 `RepositoryWatchStatus.graphRefresh` 返回，GraphView 只在首次挂载时加载。
- 当前没有独立的前端单元测试框架；Node 24 可使用内置 `node:test` 与 TypeScript type stripping，无需新增 npm 依赖。

## 需求

### R1 — 修复 Wiki rename/out-of-scope 索引一致性（IDX-001）

- rename 事件的旧路径和新路径必须分别分类。
- 旧路径位于 `wiki/*.md` 时，无论新路径是否仍属于可索引范围，都必须删除旧页面、FTS 和 wikilink 索引。
- 新路径位于 `wiki/*.md` 且文件存在时必须 upsert 新页面。
- 必须覆盖 `RenameMode::Both` 以及只有单个路径的 rename-from/rename-to 事件。
- `wiki→wiki`、`wiki→raw/schema/仓库外`、`raw→wiki` 行为必须可测试。

### R2 — 修复知识库缓存身份隔离（STATE-001）

- SQLite 必须保存当前页面索引对应的 canonical repository identity。
- 打开知识库时，如果 identity 缺失、与目标 root 不同或属于旧版缓存，必须在暴露 RepositoryInfo 前重建派生索引。
- identity 相同且索引有效时允许复用缓存。
- 重建只影响派生知识表：`pages`、`pages_fts`、`wikilinks`、`books`、`book_chapters`、`book_chapters_fts`。
- 聊天历史、编译历史、Luna 设置及其他应用状态必须保留。
- 重建或 identity 写入失败时，不得切换 `RepositoryState.root`，不得把失败知识库写入 `repository.json`。

### R3 — 问答完成必须幂等（QA-001）

- 同一个 `requestId` 的 Channel `completed` 事件和 invoke 返回结果只能在 UI 中产生一组 user/assistant 消息。
- 完成处理必须移除临时 `local-*` 消息，并按持久化 message ID 去重。
- 会话刷新、phase 清理和 evidence/waterline 更新等副作用每个 request 只执行一次。
- 离线证据模式和 Luna 流式模式共用同一完成逻辑。

### R4 — 书籍证据必须定位到目标章节（QA-002）

- App 必须接收并保存 `bookId/chapterId`。
- 激活核心书籍视图时，必须把目标传给 `CoreBooksView`。
- 目标 chapter ID 为完整 ID 或短 ID 时均能匹配。
- 切换书籍、异步加载章节目录后，最终选中章节必须与证据一致。
- 手动进入核心书籍页面时继续使用现有默认书籍/首章行为。

### R5 — Graphify 变化必须刷新已挂载视图（GRAPH-001）

- App 必须消费 `RepositoryWatchStatus.graphRefresh`，维护单独的图谱刷新版本。
- 图谱文件变化时，已挂载 GraphView 必须重新调用 `graphOverview`。
- 普通 wiki 页面变化不得触发不必要的图谱重载。
- 手动“重置视图”与搜索、邻居展开等现有交互保持有效。

### R6 — 回归测试与质量门

- Rust 单元测试覆盖 watcher rename 边界和 repository identity/legacy migration。
- 前端状态逻辑使用可测试的纯函数，并通过 Node 内置测试覆盖完成幂等、书籍目标匹配和 graph refresh 版本更新。
- GUI E2E 在具备 `TAURI_APP_PATH` 与 `tauri-driver` 时验证主导航和新增关键 test ID；缺少运行环境时必须明确记录跳过原因。
- 现有 Rust、Python、构建、verify:p3/p4/p5 套件不得回归。

### R7 — 兼容性与治理边界

- 不修改 `raw/`、`wiki/`、`schema/` 正文或 `graphify-out/` 派生图。
- 不新增网络请求、模型调用或下载行为。
- 不新增 npm 运行时依赖；优先使用 Node 24 内置测试能力。
- Tauri command 名称和现有 TypeScript 公共 API 保持兼容。
- 所有错误继续以用户可读字符串返回，不静默吞掉索引身份迁移失败。

## 验收标准

- [ ] AC1：`wiki/a.md→raw/a.md` 后旧 page/FTS/wikilink 均被删除；`raw/a.md→wiki/a.md` 后新页面被索引；`wiki/a.md→wiki/b.md` 不残留旧 ID。
- [ ] AC2：带有仓库 A 索引的共享 DB 打开仓库 B 时自动重建为 B；A 的聊天记录、编译记录和 Luna 设置仍可读取。
- [ ] AC3：同一 `AskResult` 先后经 Channel 与 invoke 返回处理，最终消息列表只有一个 user 和一个 assistant，完成副作用只执行一次。
- [ ] AC4：点击 book evidence 后，核心书籍视图选中的 `bookId/chapterId` 与 evidence 完全一致。
- [ ] AC5：`graphRefresh=true` 使 GraphView 重新加载一次；普通 `processedChanges>0 && graphRefresh=false` 不增加图谱刷新版本。
- [ ] AC6：新增回归测试全部通过，并且 `cargo test`、`cargo clippy -D warnings`、`npm run build`、`npm run verify`、`verify:p3`、`verify:p4`、`verify:p5`、Python unittest 全部通过。
- [ ] AC7：`git diff --check` 通过；工作区只包含本任务文件；业务改动以清晰 Git commit 保存。
- [ ] AC8：修复后审查报告中的 IDX-001、STATE-001、QA-001、QA-002、GRAPH-001 均有对应代码、回归测试和验证记录。

## 非目标

- 本任务不处理 P2：SEARCH-001、BOOK-001、ROLLBACK-001。
- 本任务不处理 WATCH-RISK-001、PATH-RISK-001，除非实现 P1 时发现其会阻断验收。
- 不更换 SQLite、Tauri、React 或 Graphify 技术栈。
- 不重新设计知识库 UI、问答提示词或检索排序算法。
- 不为每个知识库迁移成独立数据库文件；本阶段采用共享 DB + 派生索引 identity 的最小兼容方案。
