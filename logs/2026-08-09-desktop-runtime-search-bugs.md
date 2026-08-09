# 0.9.1 文献运行时与全局搜索修复

## 用户现场

Windows 安装版点击“立即检索最新文献”后窗口显示“未响应”，出现独立 `C:\WINDOWS\py.exe` 控制台，随后 `tools/literature_ingest.py` 在输出中文候选 JSON 时抛出 GBK `UnicodeEncodeError`。全局搜索输入 `curr` 则返回 `wrong number of arguments to function snippet()`。

## 根因

1. Tauri command 在异步命令执行器上同步等待完整编译/文献子进程，造成界面命令链阻塞。
2. Windows 子进程没有统一的无窗口创建策略，Python 又继承系统旧代码页。
3. 全局搜索把 SQLite FTS5 的固定六参数 `snippet` 当成四参数函数调用；此前测试只覆盖前端请求时序，未执行真实 FTS SQL。

## 修复

- 新增 `apps/desktop/src-tauri/src/process_support.rs`，统一配置 Windows 后台进程与 Python UTF-8 环境。
- 编译中心、文献候选操作和长期运行通过 `tauri::async_runtime::spawn_blocking` 执行，不跨 `.await` 持有状态锁；取消、事件、历史和 allowlist 契约不变。
- `tools/literature_ingest.py` CLI 入口将 stdout/stderr 重配为 UTF-8，支持直接运行与 Rust 调用。
- 抽出可测试的页面查询函数，将 FTS5 `snippet` 改为六参数；GUI E2E 增加真实 `curr` 搜索断言。
- 版本统一提升到 0.9.1，更新 updater 无更新/有更新 fixture。

## 防复发约束

- 分类：跨层契约缺失、隐式 Windows 编码假设与真实 SQL 覆盖缺口。
- 结构预防：所有桌面后台进程复用共享配置模块，长期阻塞工作统一进入阻塞线程池。
- 测试预防：FTS 辅助函数必须在真实内存 FTS 上执行；Python CLI 必须在遗留代码页父环境下验证非 ASCII JSON。
- 规范已写入 `.trellis/spec/backend/quality-guidelines.md`。

## 验证

- Rust：`cargo fmt --check`、Clippy `-D warnings`、42/42 tests。
- Python：45/45 tests；Wiki Eval 10/10；Wiki Lint 0 errors / 2 既有 warnings。
- 核心书籍：Algorithmic Game Theory Recall@5 1.000；Approximation Algorithms Recall@5 0.986667。
- 前端：P1/P2/研究脉络/文献入库/安装生命周期、build、verify、P3/P4/P5 全部通过。
- GUI：真实 0.9.1 release strict E2E 通过，包含 `curr` 搜索；1366×768 与 1920×1080 路径通过。
- 安装：NSIS 在 `work/installer-smoke-0.9.1-final` 完成安装、启动、退出、卸载，无关键 `SKIP`。

## 发布产物

| 产物 | 字节 | SHA-256 |
|---|---:|---|
| `apps/desktop/src-tauri/target/release/app.exe` | 20,403,200 | `D095044DEF94BD5FEBAA6A0DD88ADC5258D65610191A46D039EC692DCE3FE0DE` |
| `apps/desktop/src-tauri/target/release/bundle/msi/Wireless Charging Research Workbench_0.9.1_x64_en-US.msi` | 9,662,464 | `44BA19C8940AA654DEBE1A77FFA1C5EEB83E06B50DBCBC30A3C336C0F50B4369` |
| `apps/desktop/src-tauri/target/release/bundle/nsis/Wireless Charging Research Workbench_0.9.1_x64-setup.exe` | 6,747,702 | `9823D2D4891BA728693B486ED4F928B07DF50A93743E8E1B44E068C889B6F4C6` |

## 数据边界

失败运行生成的 `raw/inbox/auto-discovered/runs/search-20260809-204315/` 保持原样且不纳入提交。未修改 Wiki/Raw 正文、候选规则或 A/B 写入闸门。
