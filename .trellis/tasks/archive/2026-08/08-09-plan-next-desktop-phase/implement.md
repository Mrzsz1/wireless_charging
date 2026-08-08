# P5.4 详细实施计划

## 0. 启动前门禁与 Git 安全点

1. 用户审核本任务 `prd.md`、`design.md`、`implement.md` 后，再执行 `task.py start`；当前规划阶段不修改业务代码。
2. 运行 `python ./.trellis/scripts/get_context.py --mode phase --step 2.1`，加载 `trellis-before-dev` 和 `.trellis/spec/` 的前后端规范。
3. 确认 `git status --porcelain` 只包含本任务计划文件；记录 `git rev-parse HEAD`、0.7.0 版本文件、Rust/Node/Python 测试基线。
4. 从 Git 历史定位 `prd.md:887-896` 最后一个无乱码版本，生成只读对照，不在基线阶段改写。
5. 执行现有快速门：
   - `cd apps/desktop && npm run test:p1`
   - `cd apps/desktop && npm run build`
   - `cd apps/desktop/src-tauri && cargo test`
   - `python -m unittest discover -s tests -p "test_*.py" -v`
   - `python tools/core_book_eval.py`
6. 建立基线记录；若现有门失败，先判断是否与本阶段相关，不把预存失败误记为新回归。

**Git 检查点**：计划获批后提交 `docs(trellis): plan desktop correctness closure`；实施开始时工作树必须干净。

## 1. 先写失败回归测试

### 1.1 SEARCH-001

1. 新建可独立测试的 latest-request helper 与测试文件，但先只写暴露旧行为的契约测试。
2. 用 deferred Promise 模拟请求 A 先发、B 后发、B 先返回、A 后返回。
3. 覆盖旧请求失败、清空输入、连续三请求，断言只有最新 token 可提交。
4. 在 `package.json` 增加 `test:p2`，使新 Node 测试可单独执行。

### 1.2 BOOK-001 / PATH-RISK-001

1. 在 Rust 测试模块增加 snippet 纯函数契约：标题长于命中偏移、正文中后部命中、中文、emoji、空正文。
2. 用临时仓库和最小 `chapter-index.json` 增加路径边界测试。
3. 在 Windows 能创建 symlink/junction 时执行真实越界测试；权限不足时测试路径 components 和 canonical helper，不把发布门静默标为通过。

### 1.3 ROLLBACK-001

1. 构造 created/modified/deleted 混合的 3 文件成功基线。
2. 在第 2 个 artifact 制造应用失败，保存回滚前每个目标 hash，断言失败后完全一致。
3. 再构造补偿失败，断言不存在 `running`，状态为 `failed_partial` 且 result/event 包含失败路径。

### 1.4 WATCH-RISK-001

1. 为 watcher 批次状态写失败测试：begin 后未 ack 时再次 begin 仍返回同一批。
2. 对 apply helper 注入一次读取/事务错误，断言 fail 后 batch 保留，第二次成功后才清空。
3. 覆盖 rename previous/current 双路径、去重、blocked 和 full rebuild clear。

**完成门**：测试能够稳定复现审查问题；不能只做源码字符串断言。

**Git 检查点**：`test(desktop): reproduce remaining audit failures`。

## 2. 修复全局搜索竞态

1. 在 `apps/desktop/src/lib/` 新增 latest-request helper；API 只包含 next/current/invalidate 等最小操作。
2. `App.tsx` 用 `useRef` 持有 guard；`handleSearch` 的 success/error/empty 全部分支绑定 token。
3. 确认旧失败不会清空新结果或覆盖 notice。
4. 运行：
   - `npm run test:p1`
   - `npm run test:p2`
   - `npm run build`
5. 手动/GUI 验证快速输入、删除和 `Ctrl+K` 聚焦。

**Git 检查点**：`fix(desktop): ignore stale global search responses`。

## 3. 修复专著片段和路径边界

1. 在 `lib.rs` 提取 `resolve_repository_file`，让 `book_chapters` 返回已校验 canonical Markdown 路径。
2. 确保 list/get/search/rebuild 全部经由 `book_chapters` 使用同一安全路径。
3. 提取 `build_book_snippet`：标题与正文分开命中；正文按字符边界截取并折叠空白。
4. 保持 hits 排名、limit、章节 DTO、PDF physical pages 不变。
5. 运行 Rust 定向测试与全量测试。
6. 运行 `python tools/core_book_eval.py`，记录两书结果；任何一本低于 0.95 立即停止并回查排序/索引变化。
7. 打开两本书各抽查至少 3 个中后部命中，确认 UI 片段围绕命中且章节/PDF 跳转正常。

**Git 检查点**：`fix(desktop): secure and align core book snippets`。

## 4. 实现多文件回滚失败原子性

1. 在 `compile_center.rs` 定义内部 artifact plan、journal entry 和 rollback outcome 结构，不暴露任意文件路径给前端。
2. 复用/新增仓库相对路径边界校验；在任何写入前完成全部 hash、backup 和 operation 预检。
3. 创建受控 staging/compensation 目录和 manifest；对 staged 内容校验 `before_hash`。
4. 按 artifact 逐项应用并记录 journal；目标替换优先同卷 rename。
5. 实现逆序补偿函数，并让每个 `?` 失败出口统一进入 finalize-failure，避免漏标状态。
6. 在单个 SQLite transaction 中完成成功状态更新与 completed event。
7. 增加 `failed` / `failed_partial` 的 result_json 和 event；原 run 只有全成功才改为 `rolled_back`。
8. 增加异常 staging 清理/保留规则；保留人工诊断所需 manifest，不保留正文到日志。
9. 运行 compile_center 单元测试、全量 `cargo test`、`cargo clippy --all-targets --all-features -- -D warnings`。
10. 用本地 full-pipeline fixture 做一次真实多文件回滚成功和一次故障注入，不修改正式 Wiki 内容。

**Git 检查点**：`fix(desktop): make multi-file rollback failure-safe`。

## 5. 实现 watcher 事件确认与重试

1. 在 `repository_watcher.rs` 增加 ChangeBatch、in-flight、attempt、next_retry_at 和 blocked 状态。
2. 把 `poll()` 的“读取即消费”改为 begin/ack/fail 合同；所有新事件先进入 pending 并稳定去重。
3. 在 `lib.rs` 提取 `apply_repository_changes`，保持现有 full rebuild、wiki upsert/delete/rename 和 graph-only 语义。
4. 重写 Tauri command 编排：
   - begin batch；
   - emit started；
   - apply；
   - 成功 ack + completed；
   - 失败 fail + failed，返回可诊断错误。
5. `rebuild_index` 成功后调用 clear-after-full-rebuild；失败不清理 pending。
6. 扩展 RepositoryWatchStatus 和前端状态提示，blocked 时展示“重试/完整重建”而非静默停止。
7. 运行 watcher 单测、临时仓库增量/全量等价测试和 P1 状态回归。
8. 在真实 Windows 目录中执行：写入锁定 → 修改事件 → 首次失败 → 解锁 → 自动/手动重试成功；记录日志与最终页面数。

**Git 检查点**：`fix(desktop): retain repository changes until indexed`。

## 6. 集成、版本与文档

1. 将版本更新为 0.7.1：
   - `apps/desktop/package.json`
   - `apps/desktop/package-lock.json`
   - `apps/desktop/src-tauri/Cargo.toml`
   - `apps/desktop/src-tauri/tauri.conf.json`
2. 更新根 `prd.md`：
   - 把 §12 “下一阶段”改为真实 P5.4 状态；
   - 新增 P5.4 工作包和验收结果；
   - 从 Git 最后正常版本恢复 §13.12.7 乱码，不改动有证据的数字。
3. 更新 `apps/desktop/README.md`：watcher blocked、failed_partial、完整重建、P2 测试与严格 GUI 命令。
4. 追加 `logs/2026-08-09-p5-4-desktop-correctness.md`，记录触发原因、修改文件、测试结果、安装包路径和回退提交。
5. 不修改 `wiki/maps/library-status.md`，除非测试证明确有知识库水位变化；本阶段正常情况下不应变化。
6. 运行 Graphify 增量更新仅同步项目文档/代码图；确认 `wiki/index.md` 未被覆盖。

**Git 检查点**：`docs(desktop): record 0.7.1 reliability closure`。

## 7. 完整质量门

按顺序执行并保存输出：

1. `git diff --check`
2. `cd apps/desktop && npm run test:p1`
3. `cd apps/desktop && npm run test:p2`
4. `cd apps/desktop && npm run build`
5. `cd apps/desktop && npm run verify`
6. `cd apps/desktop && npm run verify:p3`
7. `cd apps/desktop && npm run verify:p4`
8. `cd apps/desktop && npm run verify:p5`
9. `cd apps/desktop/src-tauri && cargo fmt --check`
10. `cd apps/desktop/src-tauri && cargo clippy --all-targets --all-features -- -D warnings`
11. `cd apps/desktop/src-tauri && cargo test`
12. `python -m unittest discover -s tests -p "test_*.py" -v`
13. `python -m compileall -q tools tests`
14. `python tools/wiki_eval.py`
15. `python tools/core_book_eval.py`
16. `python tools/wiki_lint.py --strict-graphify`
17. `cd apps/desktop && npm run tauri build`
18. 设置 `TAURI_APP_PATH`/安装包路径后执行 `npm run verify:p5:strict`；报告中关键 GUI/installer 步骤不得出现 `SKIP`。
19. 在 1366×768 与 1920×1080 检查：搜索、核心书籍、编译回滚、watcher 错误提示、窗口按钮、侧栏和知识库目录。

## 8. 最终审查与交付

1. 使用 `trellis-check` 做规格映射：逐项核对 R1-R6 与 AC1-AC10。
2. 检查 `git diff --name-only`：不得包含 `raw/` 正文、Wiki 正文、密钥、AppData、`target/`、`dist/`、安装后的系统文件或临时 staging。
3. 检查所有 rollback run 都有终态，watcher blocked 批次可见，路径错误不泄露外部内容。
4. 生成 0.7.1 release/MSI/NSIS 路径和 hash；保留 0.7.0 回退产物。
5. 最终提交前再次运行 `git diff --check`，提交建议：`release(desktop): ship 0.7.1 correctness closure`。
6. `git status --porcelain` 必须为空；记录最终 commit hash。
7. 完成 Trellis task，归档任务并更新开发日志。

## 9. 停止条件

出现以下任一情况立即停止发布并回到对应工作包：

- 任一本书 Recall@5 < 0.95 或 physical-page 锚点下降；
- 故障注入后任一文件 hash 与回滚前不一致且未进入 `failed_partial` 明示；
- watcher 失败批次被清空但索引未成功；
- 路径越界测试读到仓库外内容；
- strict GUI/installer 关键步骤出现 `SKIP`；
- Git 差异包含 Raw/Wiki 正文、密钥或构建垃圾。
