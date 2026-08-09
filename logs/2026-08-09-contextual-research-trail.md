# 2026-08-09 上下文相关研究脉络

## 目标

把 Windows 客户端右侧“研究脉络”从 `catalog.slice(0, 5)` / 方法目录截断占位，升级为随当前页面、研究问题和搜索词变化的可审计本地混合检索。

## 实施

- 新增 Rust `research_trail` 模块与 `prepare_research_trail` Tauri 命令。
- 页面锚点聚合出链、反链、QA 共享检索与 Graphify 一跳；问题/搜索锚点复用 `qa::prepare_question`。
- 相关方法限定 `page_type=method`，证据与方法统一稳定排序、跨通道去重、自排除和降级通道标记。
- 新增 `ResearchTrailPanel`、请求时序守卫、问答/搜索上下文接入、GraphView 节点聚焦。
- “添加证据”检索 Wiki 与核心书籍；固定项按知识库和上下文键保存于 `desktop.research-trail.pins.v1`。
- 版本统一升级到 0.8.0；README、PRD 与结构验证脚本同步。

## 验收

- Rust：35/35；fmt、Clippy `-D warnings` 通过。
- 前端：P1 8/8、P2 3/3、research-trail 3/3，TypeScript/Vite build 通过。
- `verify`、P3、P4、P5 通过；真实 0.8.0 release EXE 的 GUI strict 覆盖 1366×768、1920×1080及搜索上下文研究脉络返回。
- 0.8.0 NSIS 在隔离临时目录完成静默安装、应用启动、进程树退出与静默卸载。
- Wiki 固定问题 10/10；核心书籍 295 条评测通过，Recall@5 为 1.000 / 0.986667。
- Wiki Lint 0 errors；保留既有警告，不修改 `raw/`、`wiki/`、`schema/` 正文。

## 发布产物

- `apps/desktop/src-tauri/target/release/app.exe` — `1F4031EF30225BD181D230191E0A4BCF970152539EEDA6CF89705A692F9059F3`
- `apps/desktop/src-tauri/target/release/bundle/msi/Wireless Charging Research Workbench_0.8.0_x64_en-US.msi` — `EC9AEB4A687CA13B6958EFE9CE5B9EA653C2AFAE91CF012B04481FD16A83CE33`
- `apps/desktop/src-tauri/target/release/bundle/nsis/Wireless Charging Research Workbench_0.8.0_x64-setup.exe` — `1D50526FA46ACCDD99E871ACE0A2373E5CFE7876A9B3F4FFFC53C9B1A53CA6B9`
