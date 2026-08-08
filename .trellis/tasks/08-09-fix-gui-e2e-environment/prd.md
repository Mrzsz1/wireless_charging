# GUI E2E 环境可运行性修复

## 目标

让桌面端 GUI E2E 在仓库默认构建产物和常见 Rust 工具链环境下可以直接运行，不再因为没有手工设置 `TAURI_APP_PATH` 而误报环境缺失；同时保留显式环境变量覆盖和缺少依赖时的可诊断跳过行为。

## 背景

`apps/desktop/e2e/gui-smoke.mjs` 当前只读取 `TAURI_APP_PATH`，即使 `apps/desktop/src-tauri/target/debug/app.exe` 已存在也会跳过。驱动也只尝试 PATH 中的 `tauri-driver`，未覆盖 Cargo bin 的常见安装位置。这样普通 E2E 的结果无法区分“产物存在但未配置变量”和“真正没有运行条件”。

## 范围

- 修改 GUI E2E 启动配置解析：自动发现 release/debug 的 Tauri 可执行文件，优先使用包含前端资源的 release 产物。
- 支持 `TAURI_APP_PATH`、`TAURI_DRIVER` 显式覆盖，覆盖值优先且错误信息准确。
- 自动探测 Cargo home 下的 `tauri-driver(.exe)`，同时兼容 PATH 和 Windows/Linux/macOS。
- 将发现逻辑拆成可单测的纯函数，覆盖覆盖值、默认候选、缺失文件和驱动路径。
- 更新桌面端 E2E 文档/脚本提示，使严格模式可复现。

## 非范围

- 不修改产品业务逻辑、Tauri command 或知识库数据。
- 不在测试脚本中静默安装 Rust/Cargo 依赖，不引入网络下载。
- 不改变非严格模式“缺少外部环境时允许跳过”的既有契约。

## 验收标准

- [x] 未设置 `TAURI_APP_PATH` 时，若默认 release/debug `app.exe` 存在，GUI E2E 自动选择它，并优先选择 release 产物。
- [x] 设置有效 `TAURI_APP_PATH` 时使用该路径；设置无效路径时给出包含路径的明确错误/跳过信息，不回退到不可见的其他路径。
- [x] `TAURI_DRIVER` 有效时优先使用；未设置时可使用 PATH 或 `$CARGO_HOME/bin/tauri-driver(.exe)`。
- [x] 缺少 app 或 driver 时，普通模式仍退出 0 并输出可执行修复命令；严格模式退出非 0。
- [x] 配置发现单测通过，现有 `test:p1`、build、verify、strict GUI E2E（在本机依赖存在时）通过。
- [x] `git diff --check` 通过，变更有清晰 commit。
