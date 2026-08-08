# 全项目代码缺陷审查报告

审查日期：2026-08-09
基线提交：`d11a711`
审查模式：Trellis inline，只读审查；未修改业务源代码、`raw/`、`wiki/`、`schema/` 或 `graphify-out/`。

## 1. 结论摘要

- **P0：0 项**。没有发现可由当前代码直接证明的数据删除、不可启动、密钥泄漏、远程执行或主流程完全不可用问题。
- **P1：5 项已验证缺陷**，集中在增量索引、跨库状态隔离、图谱刷新、问答消息渲染和证据定位。
- **P2：3 项已验证缺陷/恢复性问题**，集中在搜索竞态、专著检索片段定位和回滚原子性。
- **待验证风险：2 项**，需要真实 Windows `notify` 事件、GUI E2E 或故障注入才能确认。
- Rust 单元测试、Clippy、前端构建、项目既有 verify 套件和 Python `unittest` 均通过；严格 GUI E2E 因缺少 `TAURI_APP_PATH`/`tauri-driver` 跳过并导致 strict 命令退出 1。

建议修复顺序：先处理 `IDX-001`、`STATE-001`、`QA-001`、`QA-002` 和 `GRAPH-001`，再补齐回归测试；随后处理 `SEARCH-001`、`BOOK-001` 与 `ROLLBACK-001`。

## 2. 审查方法与范围

### 2.1 导航与证据

1. 先读取 `prd.md`、Trellis backend/frontend 规范索引及项目 `AGENTS.md`。
2. 使用现有 `graphify-out/graph.json` 执行 Graphify 查询，定位索引、问答、编译中心、Graphify 和前端页面路径。
3. 使用 CodeGraph 查询符号及调用关系，再对即将作为证据的源代码做逐行复核。
4. 扫描 Rust/Tauri、React/TypeScript、Python 工具、测试和构建入口；所有结论均附源码位置或可复现命令。

### 2.2 严重级别定义

- **P0**：数据丢失、不可启动、远程执行/密钥泄漏或主流程完全不可用。
- **P1**：核心功能错误、跨层数据错配、稳定崩溃/死循环或重要回归。
- **P2**：边界条件、恢复性、维护性和测试缺口，不阻断主流程。

## 3. 已验证缺陷（按优先级）

### IDX-001 — Wiki 页面移出 `wiki/` 后残留旧索引

- **级别/置信度**：P1 / High
- **位置**：
  - `apps/desktop/src-tauri/src/repository_watcher.rs:26-50`
  - `apps/desktop/src-tauri/src/lib.rs:814-874`
- **证据**：重命名事件只根据新路径分类；新路径不是 `wiki/*.md` 时，`classify_event` 直接返回空列表（`repository_watcher.rs:39-47`）。增量处理只有收到 `IndexChange` 才会删除 `previous_path`（`lib.rs:839-852`）。
- **复现**：
  1. 打开并完成索引的知识库。
  2. 将 `wiki/sources/A.md` 移动到 `raw/`、`schema/` 或知识库外；在 Windows 资源管理器或 `Move-Item` 中执行。
  3. 等待客户端轮询 watcher。
  4. 调用文献库列表/搜索，旧的 `A` 仍可返回。
- **实际影响**：SQLite `pages`、`pages_fts` 和 `wikilinks` 保留已不存在的页面；问答和文献库会召回失效证据，删除/迁移不再反映到 UI。
- **根因**：删除旧路径依赖“新路径必须先被判定为 interesting”，没有把 rename-from 和 rename-to 分别处理。
- **现有覆盖**：`repository_watcher.rs:177-189` 只测试 `wiki/a.md -> wiki/b.md`；没有 wiki→非 wiki、移出仓库或单路径 rename 测试。
- **建议**：对 rename 事件始终保留仓库内的 `previous_path`，先删除旧 wiki ID，再按新路径决定是否 upsert；为 `wiki→raw`、`wiki→schema`、移出仓库和 Windows `RenameMode::{From,To,Both}` 增加回归测试。

### STATE-001 — 重启后切换知识库会复用上一仓库的 SQLite 索引

- **级别/置信度**：P1 / High
- **位置**：
  - `apps/desktop/src-tauri/src/lib.rs:706-745`
  - `apps/desktop/src-tauri/src/lib.rs:2462-2477`
- **证据**：`repository_db_path` 固定返回单个应用级 `knowledge.db`（`lib.rs:706-712`）；`open_repository_state` 只统计该 DB 的页面数并设置新 `state.root`，没有持久化或校验 repository root（`lib.rs:714-745`）。启动时仅在 `!info.indexed` 时重建（`lib.rs:2467`），只要旧库有页面就跳过重建。
- **复现**：
  1. 用仓库 A 启动客户端并建立索引。
  2. 让 `repository.json` 指向结构合法但内容不同的仓库 B（或在外部切换该文件后重启）。
  3. 重启客户端；启动流程打开 B，但因共享 DB `indexed_pages > 0` 不重建。
  4. 文献库/问答返回 A 的页面，而状态栏显示 B 路径。
- **实际影响**：跨库污染检索、证据与页面详情；用户可能把 A 的研究结论误认为 B 的内容。
- **根因**：缓存 DB 没有 repository identity/manifest，启动恢复条件只看“是否有任意页面”。
- **现有覆盖**：`lib.rs` 测试覆盖同一 root 的 rebuild 和会话保留；`compile_center.rs` 的 repository isolation 只覆盖编译运行记录，不覆盖页面索引；没有重启/跨 root 测试。
- **建议**：在 DB metadata 中保存 canonical root（或为每个 repository 使用独立 DB）；打开时比较 identity，不一致时原子清空并重建。启动重建失败应显式报告，不要静默保留旧索引。

### QA-001 — 每次成功问答在 UI 中追加两遍消息

- **级别/置信度**：P1 / High
- **位置**：
  - `apps/desktop/src-tauri/src/lib.rs:2236-2243`
  - `apps/desktop/src/features/qa/AskView.tsx:123-130,149,173-174`
- **证据**：后端在持久化成功后发送 `AnswerStreamEvent::Completed`（`lib.rs:2236-2239`）并随后返回同一个 `AskResult`（`lib.rs:2243`）。前端 `handleEvent` 对 `completed` 调用 `applyCompleted`（`AskView.tsx:149`），`submitQuestion` 等待 `askLuna` 返回后再次调用 `applyCompleted(result)`（`AskView.tsx:173-174`）。`applyCompleted` 只移除 `local-*` 消息，然后无条件追加 user/assistant 两条，因此第二次调用必然重复。
- **复现**：
  1. 选择知识库并进入“智能问答”。
  2. 提交任意问题，使用离线证据模式即可触发。
  3. 等待回答完成；同一问题和回答在当前会话中各显示两次。
  4. 重新打开会话历史可看到数据库实际只保存一份，说明是前端渲染重复。
- **实际影响**：问答记录显示重复、滚动和重试上下文混乱；用户可能误以为模型生成了两次答案。
- **根因**：事件流和 invoke 返回值同时被当作 UI 最终提交信号，没有按 `requestId` 去重或选择唯一提交点。
- **现有覆盖**：Rust QA 测试覆盖会话和持久化；没有包含 `Completed` 事件到 React 状态的前端测试，也没有 GUI E2E 断言消息条数。
- **建议**：只保留事件或返回值中的一个作为最终提交；若两者都保留，用 `requestId`/message ID 做幂等替换，并新增离线与 Luna 流式完成的消息计数回归测试。

### QA-002 — 问答中的书籍证据无法打开到引用章节

- **级别/置信度**：P1 / High
- **位置**：
  - `apps/desktop/src/features/qa/AskView.tsx:187-192`
  - `apps/desktop/src/App.tsx:472`
  - `apps/desktop/src/features/books/CoreBooksView.tsx:30-44`
- **证据**：`AskView.openEvidence` 正确传出 `onOpenBook(item.bookId, item.chapterId)`（`AskView.tsx:189`），但 App 传入的回调是 `onOpenBook={() => activateView('books')}`，丢弃两个参数（`App.tsx:472`）。`CoreBooksView` 的章节定位逻辑依赖 `target.bookId/target.chapterId`（`CoreBooksView.tsx:36-44`），而 App 始终渲染 `target={null}`（同一行上方的 books 分支）。
- **复现**：
  1. 在智能问答中提问，得到 `kind=book` 证据。
  2. 点击证据详情的“打开来源”。
  3. 客户端只切到“核心书籍”页面，显示默认书籍/默认章节，不是被引用的章节。
- **实际影响**：核心“回答→书籍页码/章节”可审计链路断裂；用户必须手动搜索章节，严谨引用场景容易误读。
- **根因**：跨组件 callback 签名已定义为 `(bookId, chapterId)`，但 App 没有把参数转换成 `CoreBooksView.target`。
- **现有覆盖**：书籍 Rust 命令和章节解析有测试；没有前端 evidence-to-chapter 导航测试，GUI E2E 也没有覆盖该交互。
- **建议**：在 App 保存 `bookTarget` 状态，回调设置目标并激活 books；将 target 传给 `CoreBooksView`，并用 book/chapter ID 回归测试验证最终选中章节。

### GRAPH-001 — Graphify 文件变化被检测到，但已挂载图谱视图不刷新

- **级别/置信度**：P1 / High
- **位置**：
  - `apps/desktop/src-tauri/src/repository_watcher.rs:88-96`
  - `apps/desktop/src-tauri/src/lib.rs:814-915`
  - `apps/desktop/src/App.tsx:198-207`
  - `apps/desktop/src/features/graph/GraphView.tsx:25-30`
- **证据**：watcher 将 `graphify-out/graph.json` 标记为 `graph_refresh`（`repository_watcher.rs:88-96`），后端在 `index_update_completed` 中只报告 `graphRefresh`（`lib.rs:903-915`）。App 收到变化只增加 `repositoryGeneration`（`App.tsx:198-207`）；GraphView 的 `load` effect 依赖数组是空数组，只在挂载时执行（`GraphView.tsx:30`），且 App 没有把 generation 传给它。
- **复现**：
  1. 启动客户端并打开“知识图谱”，保持该视图不切换。
  2. 在知识库外部运行 `graphify update .` 或替换 `graphify-out/graph.json`。
  3. 等待自动 watcher 轮询完成。
  4. 图谱仍显示旧节点/边；离开页面再回来才重新读取。
- **实际影响**：用户看到的派生关系图与当前 Graphify 文件不一致，刷新提示已出现但视图内容过期。
- **根因**：刷新信号没有贯通到 GraphView；`repositoryGeneration` 只刷新主目录数据。
- **现有覆盖**：Rust 只测试图 JSON 过滤；前端结构校验未断言 watcher 变化后 GraphView 重新调用 `graphOverview`，GUI E2E strict 因环境缺少驱动未运行。
- **建议**：把 generation/version 作为 GraphView prop 或订阅 `index_update_completed`，在 `graphRefresh` 时重新加载；新增图 JSON 变更后的组件回归测试。

## 4. 已验证的 P2 问题

### SEARCH-001 — 全局搜索存在响应乱序竞态

- **级别/置信度**：P2 / High
- **位置**：`apps/desktop/src/App.tsx:401-408`
- **证据**：每次输入都直接启动 `searchPages(value, 30)`，响应返回后无条件 `setResults`；没有 request sequence、AbortController 或当前 query 校验。
- **复现**：快速输入 `algorithm` 后立即改为 `game`，让第一请求延迟到第二请求之后；旧的 `algorithm` 结果会覆盖当前 `game` 查询。
- **影响/根因**：搜索结果与输入框不一致；异步状态没有按 query 绑定。
- **现有覆盖**：无前端搜索竞态测试；仅有 Rust FTS 单元/结构测试。
- **建议**：维护递增 request token，返回时仅接受最新 token；或取消上一请求并在 UI 层按 query 丢弃过期结果。

### BOOK-001 — 专著搜索片段使用了错误的字符串偏移

- **级别/置信度**：P2 / High
- **位置**：`apps/desktop/src-tauri/src/lib.rs:1474-1489`
- **证据**：代码对 `format!("{} {}", chapter.title, body)` 做 `find`（`lib.rs:1474,1485`），再把包含标题前缀的 `index` 直接用于 `body.get(start..end)`（`lib.rs:1486-1489`）。命中位置应减去标题及空格的字节长度；当前实现会整体偏移，标题较长时还可能切片失败并回退到正文开头。
- **复现**：选一个标题较长、正文中命中词在中后部的章节，搜索该词；观察片段不围绕命中词或显示错误位置。
- **影响/根因**：搜索结果仍能返回章节，但摘要不能准确定位命中上下文。
- **现有覆盖**：无 snippet 偏移测试；现有书籍搜索测试只验证结果排序/数量。
- **建议**：分别在 title/body 上计算命中，或使用 body 的独立索引；按字符边界而非混合字符串字节偏移生成 snippet。

### ROLLBACK-001 — 回滚多文件时出现中途错误会留下部分恢复结果

- **级别/置信度**：P2 / High
- **位置**：`apps/desktop/src-tauri/src/compile_center.rs:1283-1368`
- **证据**：回滚前只做 hash 预检（`compile_center.rs:1311-1325`），随后逐个执行删除/复制（`compile_center.rs:1330-1348`）。任意中途 `remove_file`、备份缺失或 `fs::copy` 失败都会直接返回错误；已完成的前序文件不会恢复，数据库中已插入的 rollback run 也不会被标记失败或清理。
- **复现**：构造包含两个可回滚 artifact 的成功任务，在第二个 artifact 删除备份或制造权限错误，然后调用 `rollback_compile_run`。
- **影响/根因**：知识库只恢复一部分，run 状态停留在 `running`，后续重试/审计无法判断真实文件状态。
- **现有覆盖**：`compile_center.rs:1398-1435` 只覆盖单个 modified 文件成功回滚，没有多文件故障注入或原子性断言。
- **建议**：先验证所有备份可读，再把恢复复制到临时目录并原子替换；或维护已恢复清单并在失败时反向补偿，同时把 rollback run 标记为 `failed_partial`。

## 5. 待验证风险

### WATCH-RISK-001 — watcher 轮询后索引失败可能丢失事件

`process_repository_changes` 先调用 `RepositoryWatcher::poll` 消费并清空 channel，再在 SQLite 事务中读取 Markdown（`lib.rs:814-874`）。若文件正处于写入锁定/半写状态导致 `upsert_wiki_page_index` 报错，事务会回滚，但已消费的事件不会重新入队；下一轮可能没有事件，索引长期落后。需要 Windows 文件锁和写入中断的故障注入确认，因此不计入已验证缺陷。

### PATH-RISK-001 — `chapter-index.json` 中的 Markdown 路径未做仓库边界校验

`book_chapters` 将 JSON 的 `path` 直接 `root.join`（`lib.rs:1290-1305`），`get_book_chapter` 随后直接读取该路径（`lib.rs:1410-1420` 附近）。当前仓库生成器是受控的，因此暂列待验证风险；若用户打开恶意/损坏知识库，可能把仓库外文件内容读入客户端。建议在修复书籍链路时一并 canonicalize 并验证 `starts_with(root)`。

## 6. 测试、构建与工具结果

| 检查 | 结果 | 备注 |
|---|---|---|
| `python -m pytest -q` | 未执行 | 当前解释器缺少 `pytest` 模块：`No module named pytest`。 |
| `python -m unittest discover -s tests -p "test_*.py" -v` | **PASS** | 37 个测试通过。 |
| `python -m compileall -q tools tests` | **PASS** | Python 语法编译通过。 |
| `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` | **PASS** | 23 个 Rust 单元测试通过。 |
| `cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml --all-targets --all-features -- -D warnings` | **PASS** | 无 Clippy 警告。 |
| `npm run build`（`apps/desktop`） | **PASS** | TypeScript 与 Vite 构建通过；生成的 `public/data/library.json` 为既有忽略产物。 |
| `npm run verify` | **PASS** | 结构、数据和契约检查通过。 |
| `npm run verify:p3` | **PASS** | Rust 23 tests；核心书籍 Recall@5 与章节召回均达到要求。 |
| `npm run verify:p4` | **PASS** | Wiki lint 66 pages，errors 0，1 条既有 `inspired_by` warning。 |
| `npm run verify:p5` | **PASS** | structural/config/updater/offline 检查通过；GUI/installer 按环境跳过。 |
| `npm run verify:p5:strict` | **FAIL（环境前置）** | 唯一失败为 GUI E2E：缺少 `TAURI_APP_PATH` 和 `tauri-driver`；不是源码断言失败。 |
| `ruff` / `mypy` / `pyright` | 未配置/不可用 | 当前环境 PATH 未找到这些命令。 |
| `git diff --check` | **PASS** | 无空白错误。 |

## 7. 未发现的高风险类别

- 未发现生产路径中无界 `unwrap`/`expect` 导致的已证实崩溃；命中主要位于测试或固定注册 ID。
- 未发现 API key 被写入 SQLite、前端持久状态或普通任务日志的证据；Luna 与检索工具已有环境变量/日志脱敏路径。
- `open_local_path`、编译 parse 输入和 MinerU ZIP 解压均存在边界校验；本次未将其列为缺陷。
- 未发现可由当前命令复现的构建、Rust 类型、Clippy 或 Python 语法回归。

## 8. 修复与回归测试优先级

1. **P1 第一批**：`IDX-001`（rename 双路径处理）、`STATE-001`（DB identity/独立 DB）、`QA-001`（完成事件幂等）、`QA-002`（book target 贯通）、`GRAPH-001`（generation/事件刷新）。
2. **P1 回归门**：增加 Rust watcher、跨 root startup、QA event contract、GraphView refresh 和 evidence-to-chapter 的自动化测试；配置 `TAURI_APP_PATH` 与 `tauri-driver` 后再跑 strict GUI E2E。
3. **P2 第二批**：`SEARCH-001`、`BOOK-001`、`ROLLBACK-001`，补充异步竞态、Unicode/标题偏移、多文件故障注入测试。
4. **风险收敛**：对 watcher 失败重试/事件重放和 chapter path canonicalize 增加诊断日志与边界测试。

本报告仅记录问题，未在本任务内修复业务代码。任何修复应按 Trellis `trellis-before-dev` → 实施 → `trellis-check` 流程另开任务，并在修复后重新执行本报告中的质量门。
