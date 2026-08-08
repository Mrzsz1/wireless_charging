# P5.4 桌面端正确性与恢复可靠性

## 触发原因

代码审查发现全局搜索乱序响应、核心专著片段偏移、章节路径越界、跨文件回滚中途失败和 watcher 事件丢失风险。本阶段目标版本为 0.7.1，范围限定在桌面客户端、测试、发布元数据和 Graphify 派生图，不修改 `raw/` 或 `wiki/` 正文。

## 实施记录

- Trellis 任务：`.trellis/tasks/08-09-plan-next-desktop-phase/`
- 计划提交：`b480093 docs(trellis): plan desktop correctness closure`
- 已完成提交：`057e875`（搜索 generation guard）、`c5cd0c4`（专著片段与路径边界）、`b22ed16`（多文件回滚 staging/补偿）、`d8d6aa9`（watcher in-flight/ack/retry/blocked）。
- 版本与文档收口：0.7.1、README、PRD、本日志、Graphify 派生图。

## 质量证据

- Rust：`cargo fmt --check`、`cargo clippy --all-targets --all-features -- -D warnings`、32/32 tests 通过。
- 前端：`test:p1` 4/4、`test:p2` 3/3、TypeScript/Vite build、`verify`、`verify:p3`、`verify:p4`、`verify:p5` 通过。
- Python/知识库：unittest 37/37、Wiki eval 10/10、Wiki Lint 0 errors/1 warning；两书 295 条评测中 Algorithmic Game Theory Recall@5=1.000，Approximation Algorithms Recall@5=0.986667，`passes_95_book_recall=true`。
- 严格发布门禁：release `app.exe` 构建成功；GUI strict 覆盖 1366×768 和 1920×1080；NSIS 安装、启动、卸载 smoke 通过；无关键 `SKIP`。
- Graphify：已执行 `graphify update .`，当前 2589 nodes / 4364 edges / 200 communities；`wiki/index.md` 未被覆盖。

## 发布产物

```text
apps/desktop/src-tauri/target/release/app.exe
apps/desktop/src-tauri/target/release/bundle/msi/Wireless Charging Research Workbench_0.7.1_x64_en-US.msi
apps/desktop/src-tauri/target/release/bundle/nsis/Wireless Charging Research Workbench_0.7.1_x64-setup.exe
```

SHA-256：

- app.exe：`63E048760E42DBDCCE3B27C8AF2AAD1D95777B90EDC06468B3497890456B2626`
- MSI：`3EF3931E81525B63E365BE12D89CF2FDA488AF747121F7683F63A347F707B8FA`
- NSIS：`C1EBA0AF177BBD564690432D55C5856328F2F754DE19FBB5B01C1035E40D1B29`

## 已知基线警告

Wiki Lint 仍有 1 条既有 B 类页面 `wiki/problems/prob-joint-deployment-online-interference.md` 缺少 `inspired_by`。按照仓库规则，本阶段未在未获用户确认时改写 B 类正文；该警告不影响 P5.4 代码、书籍召回或发布门禁。
