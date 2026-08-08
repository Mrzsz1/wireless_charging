# GUI E2E 环境修复实施计划

## 阶段 1：配置解析

1. 新增 `apps/desktop/e2e/gui-config.mjs`，实现应用和驱动候选路径、显式覆盖、平台后缀和诊断结果。
2. 为配置解析增加 Node 内置测试，使用临时目录和注入环境，不依赖真实 GUI 或 WebDriver。
3. 在 `gui-smoke.mjs` 中复用配置解析，统一普通/严格模式的提示和退出码。

## 阶段 2：开发者入口

4. 增加 `test:e2e-config` npm script，并将其纳入 `verify:p5` 的 GUI 检查之前。
5. 更新 `apps/desktop/README.md`，记录自动发现、显式覆盖、驱动安装和严格运行命令。

## 阶段 3：验证与提交

6. 运行配置单测、已有 P1 单测、前端 build/verify、Rust 检查和 `git diff --check`。
7. 在当前 Windows 环境用仓库已有 `app.exe` 和 `tauri-driver` 运行 `npm run e2e:gui:strict`；若 GUI 驱动受桌面会话限制，记录精确错误而非把它标为代码失败。
8. 更新任务验收状态和 Trellis 规范/日志，提交一个聚焦 GUI E2E 的 Git commit。

## 回滚点

- 配置解析提交前：仅撤销新增 helper/test。
- GUI 运行验证失败：保留诊断输出，回退到显式环境变量命令，不修改产品代码。

## 实施记录

- [x] 配置发现 helper、Node 单测和 GUI smoke 集成完成。
- [x] release 优先选择，避免 debug 构建缺少 Vite dev server 导致白屏。
- [x] 缺少 Windows native Edge driver 时普通模式可诊断跳过，strict 模式返回 2。
- [x] 当前环境安装 `tauri-driver` 与匹配 `msedgedriver`，严格 GUI E2E 通过。
- [x] build、verify、Rust/Python/Node 回归和 wiki lint 通过。
