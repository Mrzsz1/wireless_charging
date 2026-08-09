# 文献执行卡死与全局搜索失败：实施计划

## 1. 建立回归测试

- [x] 抽取可测试的全局搜索查询函数，先增加真实 FTS5 `curr` 命中与 `<mark>` 高亮测试。
- [x] 增加中文搜索与空结果回归测试。
- [x] 为共享后台命令配置增加 Python UTF-8 环境断言。
- [x] 为 `tools/literature_ingest.py` 增加 GBK 环境下中文 JSON 输出回归测试。

验证：

```powershell
cd apps/desktop/src-tauri
cargo test search_pages
cargo test process_support
cd ../../../
py -3 -m unittest tests.test_literature_ingest
```

## 2. 修复搜索 SQL

- [x] 将 FTS5 `snippet()` 补全为六参数并保留现有 BM25、limit、prefix query 与 LIKE fallback。
- [x] 搜索命令改为调用抽取函数，不改变前端 DTO。
- [x] 核对 `qa.rs` 和 `research_trail.rs` 的合法调用，无需顺带改写。

回滚点：仅搜索 helper 与 SQL；失败时可独立恢复。

## 3. 统一 Windows 后台进程配置

- [x] 新增共享进程辅助模块并在 `lib.rs` 注册。
- [x] Windows 使用 `CREATE_NO_WINDOW`；Python 增加 UTF-8 环境。
- [x] 应用于 `literature_ingest.rs` 的 Python/能力探测、`compile_center.rs` 的主任务与 `taskkill`。
- [x] 不更改命令 allowlist 和参数数组边界。

回滚点：辅助模块和调用点是独立机械改动。

## 4. 移出 Tauri 主处理线程

- [x] `execute_compile_request` 使用 `spawn_blocking` 包裹完整长期运行。
- [x] 三个启动/重试命令改为 async，保持取消令牌生命周期。
- [x] 候选列举、triage、能力检查改为先复制状态再异步运行，且不跨线程移动 rusqlite Connection。
- [x] 前端 API、Channel 事件和 UI 状态无需改签名。

验证：

```powershell
cd apps/desktop/src-tauri
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## 5. Python 输出兜底与错误展示

- [x] CLI stdout/stderr 入口设为 UTF-8。
- [x] 确认结构化 `LITERATURE_RESULT` 和候选 JSON 均可包含中文。
- [x] 错误仍通过 stderr 与编译中心事件返回，不吞 traceback 的关键原因。

## 6. 版本、文档与发布

- [x] 版本提升到 0.9.1，同步 package、Cargo、Tauri 和 updater fixtures。
- [x] 更新客户端 README、根 PRD 与当日日志，说明两项缺陷及修复。
- [x] 执行全量 Python、Rust、前端构建和 verify 门禁。
- [x] 重新构建 release、MSI、NSIS。
- [x] 严格 GUI E2E 验证搜索与文献页；NSIS 执行隔离安装生命周期。
- [x] 记录新产物 SHA-256。

## 7. Git 与 Trellis 收尾

- [x] `git diff --check`、密钥扫描、确认用户 raw 失败目录未暂存。
- [x] 按 `trellis-check` 复核跨层数据流和回归覆盖。
- [x] 提交代码、测试、文档和任务记录（`6286abe`）。
- [x] 归档 Trellis 任务并记录开发日志。

停止条件：任何测试继续出现 `snippet()` 参数错误、GBK traceback、可见控制台窗口、窗口无响应，或用户 raw 目录进入暂存区时，不进入发布与提交。

## 实施结果

- 搜索 SQL 已改为合法六参数 FTS5 `snippet`，严格 GUI 在真实 0.9.1 release 上执行 `curr` 搜索通过。
- 长期编译与文献任务经 `spawn_blocking` 执行；所有内部 Windows 子进程统一隐藏，Python 固定 UTF-8。
- 质量门：Rust 42/42、Python 45/45、Wiki Eval 10/10；两书 Recall@5 为 1.000 / 0.986667；前端构建、P1/P2/P3/P4/P5、严格 GUI 与 NSIS 生命周期全部通过。
- 产物：app 20,403,200 bytes，MSI 9,662,464 bytes，NSIS 6,747,702 bytes；SHA-256 见根 PRD 与发布日志。
- `raw/inbox/auto-discovered/runs/search-20260809-204315/` 保持为未跟踪用户数据，未修改、未暂存。
