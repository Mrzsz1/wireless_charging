# GUI E2E 配置发现技术设计

## 设计边界

配置解析仅负责将环境变量、仓库路径和工具链路径转换为可验证的启动配置；WebDriver 会话、窗口交互和断言继续由 `gui-smoke.mjs` 负责。解析层不执行安装、不启动应用、不修改工作区。

## 配置优先级

### 应用路径

1. `TAURI_APP_PATH`：非空时视为显式配置。路径存在且为文件才可启动；显式路径无效时保留失败原因，不静默改用其他产物。
2. `apps/desktop/src-tauri/target/release/app.exe`（优先选择，包含生产构建的前端资源）。
3. `apps/desktop/src-tauri/target/debug/app.exe`（仅在 release 产物不存在时使用；debug 构建通常需要同时运行 Vite dev server）。

候选路径根据 E2E 模块位置和当前工作目录生成，避免从仓库根目录或 `apps/desktop` 目录执行时行为不同。非 Windows 平台使用对应的 `app` 文件名。

### 驱动路径

1. `TAURI_DRIVER`：显式路径或命令名。
2. PATH 中的 `tauri-driver`。
3. `$CARGO_HOME/bin/tauri-driver(.exe)`；未设置 `CARGO_HOME` 时使用用户 home 下的 `.cargo`。

最终仍通过 `spawnSync(..., --help/--version)` 验证可执行性。显式驱动路径无效时不悄悄换用另一驱动。

## 数据流

`resolveGuiE2eConfig` → `gui-smoke.mjs` 环境检查 → 启动 tauri-driver → WebDriver capabilities → 现有 GUI 断言。解析结果包含 `appPath`、`driverExecutable`、来源和诊断候选列表，错误信息只展示本地路径和安装命令。

## 兼容性与回滚

- 保持现有 `TAURI_APP_PATH`、`TAURI_DRIVER`、`TAURI_NATIVE_DRIVER` 行为和 strict/non-strict 退出码。
- 默认候选只增加自动发现，不覆盖用户显式配置。
- 回滚只需恢复 `e2e/gui-config.mjs`、`e2e/gui-smoke.mjs` 及对应测试/文档。

## 启动失败处理

驱动可执行文件探针通过并不代表 `tauri-driver` 能启动；Windows 下还可能缺少 `msedgedriver.exe`。等待 4444 端口失败时，脚本先清理子进程，再沿用 normal=0 / strict=2 的环境契约，并输出 native driver 安装提示，避免留下孤儿进程或只显示未定位的堆栈。
