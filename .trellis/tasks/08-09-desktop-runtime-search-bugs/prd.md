# 修复文献执行卡死与全局搜索失败

## Goal

修复 Windows 客户端中两个已由真实安装版复现的 P0 运行时缺陷：文献自动检索阻塞界面并弹出控制台，随后因 GBK 输出失败；全局搜索因 SQLite FTS5 `snippet()` 调用参数错误而完全不可用。修复后客户端应保持响应、后台任务不显示控制台，并能稳定处理中文输出和中英文搜索。

## Background

- 用户截图显示点击“立即检索最新文献”后窗口标题出现“未响应”，同时弹出 `C:\WINDOWS\py.exe` 控制台。
- 任务随后在 `tools/literature_ingest.py:602` 输出含中文的 JSON 时触发 `UnicodeEncodeError: 'gbk' codec can't encode character`。
- `apps/desktop/src-tauri/src/literature_ingest.rs:515-526` 使用同步 `Command::output()` 启动 `py -3`，未声明 UTF-8 环境，也未在 Windows 隐藏控制台。
- `apps/desktop/src-tauri/src/compile_center.rs:1206-1212` 启动长期 Python 编译任务时同样未设置隐藏窗口或 UTF-8 环境。
- `apps/desktop/src-tauri/src/lib.rs:1112` 调用 `snippet(pages_fts,2,'<mark>','</mark>')`，只有 4 个参数；SQLite FTS5 要求 6 个参数，因此搜索返回 `wrong number of arguments to function snippet()`。
- 工作区内 `raw/inbox/auto-discovered/runs/search-20260809-204315/` 是本次失败运行留下的用户数据，修复过程中保持不动且不纳入提交。

## Requirements

### R1. 文献任务不阻塞 UI

- 长期编译/文献任务必须在 Tauri 阻塞线程池执行，IPC 主处理线程只负责参数验证、任务登记和等待异步结果。
- 任务运行期间窗口必须保持可拖动、可切换页面，并能持续接收日志事件。
- 取消、超时、仓库写互斥、任务历史和部分失败语义保持不变。

### R2. Windows 后台进程不可见

- 文献候选读取、能力检查、编译任务以及取消用辅助命令均不得弹出 `py.exe`、`cmd.exe` 或其他控制台窗口。
- Windows 使用后台进程创建标志；非 Windows 平台保持现有行为。
- 进程配置应集中复用，避免文献模块和编译中心再次漂移。

### R3. Python 输出固定为 UTF-8

- Rust 启动 Python 时显式设置 `PYTHONUTF8=1` 与 `PYTHONIOENCODING=utf-8`。
- `tools/literature_ingest.py` 的 CLI 入口对 stdout/stderr 做 UTF-8 兜底，中文标题、路径、错误和 JSON 均可输出。
- Rust 继续按 UTF-8 解析结构化 stdout；失败 stderr 应以可读文本进入 UI，而不是编码 traceback。

### R4. 全局搜索恢复

- 全局 Wiki 搜索使用合法的 FTS5 `snippet(table,column,start,end,ellipsis,tokens)` 六参数调用。
- 保留 `<mark>` 高亮、BM25 排序、结果上限和无 FTS 命中时的 LIKE fallback。
- 英文前缀查询 `curr` 与中文查询均应返回结果或正常空列表，不产生 SQL 函数错误。

### R5. 回归与发布

- 增加 Rust 搜索回归测试，真实建立内存 FTS 索引并执行查询。
- 增加后台 Python UTF-8/进程配置测试，以及 Python CLI 中文 JSON 回归测试。
- 执行 Python、Rust、前端构建、严格 GUI E2E 和 NSIS 安装生命周期验证。
- 版本提升到 0.9.1，重新生成 EXE、MSI、NSIS，记录路径、大小和 SHA-256，并提交 Git。

## Acceptance Criteria

- [x] AC1 / R1：长期任务已移入阻塞线程池；严格 GUI 验证期间窗口导航与交互保持响应。
- [x] AC2 / R2：自动检索、候选刷新、能力检查和取消辅助命令统一使用 Windows 无窗口进程配置。
- [x] AC3 / R3：GBK 父环境下含中文的 CLI JSON 回归测试通过，不再出现 `UnicodeEncodeError`。
- [x] AC4 / R4：真实 0.9.1 release 严格 GUI 搜索 `curr` 通过，内存 FTS 测试确认 `<mark>` 高亮。
- [x] AC5 / R4：Rust 回归覆盖中文搜索和正常空结果；既有 LIKE fallback 保留。
- [x] AC6 / R1-R3：Rust 42/42、Python 45/45、前端与 P3/P4/P5 门禁均通过。
- [x] AC7 / R5：0.9.1 release 严格 GUI 与 NSIS 安装/启动/退出/卸载通过。
- [x] AC8：暂存前检查明确排除失败运行目录；该用户数据保持原状。

## Out of Scope

- 删除或重新执行用户本次失败产生的候选目录。
- 修改自动检索评分、来源、关键词或入库资格规则。
- 重构全部 SQLite 查询或编译中心任务模型。
