# 编译并安装最新 Windows 客户端

## Goal

将当前 `master` 最新提交编译为 Windows Release 安装包，覆盖本机旧版本并验证安装后的客户端可正常启动且包含最新侧栏界面。

## Requirements

1. 构建前确认工作区仅保留两个用户 Raw 运行目录未跟踪，不把它们加入构建提交。
2. 仅终止占用本项目构建产物或已安装客户端的相关进程。
3. 执行 Tauri Release 构建并生成 Windows 安装包。
4. 优先使用 NSIS 安装包静默覆盖安装；不存在时使用 MSI。
5. 安装后解析真实安装路径并启动已安装程序，而不是仅运行 `target/debug/app.exe`。
6. 验证进程存在、主窗口可见，并确认安装程序包含最新前端构建。
7. 记录安装包路径、已安装程序路径与验证结果。

## Acceptance Criteria

- [x] AC1：Release 构建成功，安装包存在且大小非零。
- [x] AC2：最新版客户端覆盖安装成功。
- [x] AC3：从安装目录启动客户端后进程与可见窗口存在。
- [x] AC4：两个 Raw 运行目录仍保持未跟踪且未修改。
- [x] AC5：发布任务按 Trellis 流程记录并提交。

## Verification

- Release：`apps/desktop/src-tauri/target/release/app.exe`，版本 `0.11.0`。
- NSIS：`apps/desktop/src-tauri/target/release/bundle/nsis/Wireless Charging Research Workbench_0.11.0_x64-setup.exe`，7,378,563 bytes。
- MSI：`apps/desktop/src-tauri/target/release/bundle/msi/Wireless Charging Research Workbench_0.11.0_x64_en-US.msi`，10,665,984 bytes。
- 安装目录：`C:/Users/qq155/AppData/Local/Wireless Charging Research Workbench/app.exe`。
- 桌面与开始菜单快捷方式均指向上述安装目录。
- 已安装程序严格 GUI E2E 通过，1366×768 与 1920×1080 无溢出。
- 已启动安装目录程序：版本 `0.11.0`，主窗口可见。
