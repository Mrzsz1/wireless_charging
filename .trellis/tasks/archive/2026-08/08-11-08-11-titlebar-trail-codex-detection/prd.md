# 标题栏研究脉络控制与 Codex CLI 检测修复

## Goal

将研究脉络折叠开关移动到标题栏窗口控制区前，并修复 Windows 已安装 Codex CLI 仍被误判未安装的问题。

## Requirements

### 1. 研究脉络开关位置

- 研究脉络的折叠/展开必须由标题栏中的单一开关控制。
- 开关固定放在 Windows 原生最小化、最大化、关闭按钮之前，即截图红框位置。
- 研究脉络展开时显示“收起”语义图标，折叠时显示“展开”语义图标；悬浮提示和无障碍标签必须同步变化。
- 删除右侧内容区边缘的竖向折叠轨道和中间悬浮按钮，折叠后主内容区直接占满释放的宽度。
- 研究脉络面板头部不再保留第二个折叠入口，避免重复控制。

### 2. Codex CLI 检测

- Windows 客户端必须识别 PATH 中的 `codex.exe`、`codex.cmd`、`codex.bat` 和无扩展名启动器。
- 必须覆盖 npm/Node 全局安装目录、自带 Codex 桌面应用资源目录及常见用户级安装目录。
- 检测逻辑不能假设 Tauri GUI 进程继承了启动终端的完整 PATH。
- `.cmd`/`.bat` 启动器必须用 Windows 兼容方式执行，不能因 `Command::new("codex")` 无法直接启动脚本而误判未安装。
- 设置页刷新状态后应显示实际版本；登录状态检测继续沿用现有协议，不读取或持久化 token、cookie、API Key。
- 支持显式 `CODEX_CLI_PATH` 覆盖，便于特殊安装位置和诊断。

### 3. 交付与约束

- 保持现有 React/Tauri 架构，不新增 UI 依赖。
- 更新静态契约、E2E 和后端路径解析测试；每类测试只运行一次，不重复复测。
- 完成 release 构建、安装并启动新客户端。
- 仅提交本任务文件，不纳入现有 `raw/inbox/auto-discovered/runs/` 未跟踪目录。

## Acceptance Criteria

- [x] 标题栏开关位于拖拽区域之后、原生窗口按钮之前，展开/折叠状态图标与提示正确。
- [x] 折叠研究脉络后不再出现 42px 竖向轨道或内容区悬浮开关，主区域扩展到窗口右缘。
- [x] 研究脉络展开后仍可刷新，证据链/相关方法交互不回归。
- [x] 在本机 `F:\nodejs\codex.cmd`/`codex` 及 Codex 桌面资源 `codex.exe` 存在的情况下，设置页不再显示“Codex CLI 未安装”。
- [x] Windows 路径候选去重、优先级和脚本执行方式具有自动化测试覆盖。
- [x] 前端类型检查、静态验证、单元测试、Rust 测试、release 构建通过。
- [x] NSIS 安装成功且新版本客户端可启动。

## Notes

- Keep `prd.md` focused on requirements, constraints, and acceptance criteria.
- Lightweight tasks can remain PRD-only.
- For complex tasks, add `design.md` for technical design and `implement.md` for execution planning before `task.py start`.

## Delivery Record

- Rust: `cargo test ... codex_subscription`，7 passed。
- Frontend: `test:qa-settings`，4 passed；`verify` 全部通过。
- GUI: `e2e:gui:strict` 通过 1366×768、1920×1080、标题栏折叠布局与本机 Codex 检测。
- Release: Tauri 0.11.0 MSI/NSIS 构建成功；NSIS 静默安装成功；安装版进程 PID 29800 已启动。
