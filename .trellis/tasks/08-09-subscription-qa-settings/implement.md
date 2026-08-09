# 设置页订阅问答模式：实施计划

## 1. 先建立回归与 fixture

- [x] 新增 CodexSubscriptionStatus、AnswerProvider、QaSettings 的类型/迁移单测。
- [x] 建立 fake Codex executable fixture，覆盖 version、login status、login launch、JSONL delta/final/malformed、stderr、hang 和 process-tree child。
- [x] 增加前端结构门禁：失败基线应证明 AskView 仍有 Luna modal、SettingsView 尚无完整回答引擎卡。

验证：定向 Node/Rust 测试先失败，再进入实现。

## 2. 提取通用问答设置

- [x] 将 `LunaSettings` 扩展/迁移为 `QaSettings`，增加 `answerProvider` 与可选 `codexModel`。
- [x] 保留 `luna.*` SQLite 键，新增 `qa.answer_provider`、`qa.codex_model`，实现迁移矩阵和严格枚举校验。
- [x] Tauri/TypeScript service 使用 `get_qa_settings`、`save_qa_settings`；必要时保留旧命令兼容测试。
- [x] 更新历史 provider label，不修改旧记录。

## 3. 实现 Codex 状态与登录命令

- [x] 新建 `codex_subscription.rs`，固定执行 `codex --version`、`codex login status`，限制超时并脱敏输出。
- [x] 注册 `get_codex_subscription_status` 与 `start_codex_login`；所有阻塞操作进入 `spawn_blocking`。
- [x] 登录使用官方浏览器流程，不读取/返回凭据；状态刷新可确认登录完成。
- [x] Windows 使用共享后台进程配置，避免控制台弹窗。

回滚点：移除模块和两个命令不影响现有 Luna/离线问答。

## 4. 实现订阅回答适配器

- [x] 提取共享 system/prompt builder，确保三种 provider 使用同一证据契约。
- [x] 构建固定 Codex 命令与空临时工作目录，prompt 仅写 stdin。
- [x] 实现 JSONL 宽容解析和 token/final 统一输出；错误码统一为 `CODEX_*`。
- [x] `ask_luna` 重构为通用路由，兼容 API 流保持原行为，Codex 失败和未就绪按规则降级离线。
- [x] 持久化 provider/model/offline 与现有完成幂等逻辑保持一致。

## 5. 加固取消与清理

- [x] 扩展 request registry，使取消 flag 能终止活动 Codex PID/process tree。
- [x] 抽取 Rust 跨平台进程树终止 helper，复用 Windows `taskkill` 与 `CREATE_NO_WINDOW`。
- [x] 为取消、超时、异常退出建立 cleanup guard，验证无孤儿进程、无临时目录、无半完成会话。
- [x] 限制和脱敏 stderr；密钥/token 模式扫描覆盖所有新错误路径。

## 6. 所有配置迁入 SettingsView

- [x] 用“AI 回答引擎”替换 SettingsView 的 Luna 跳转卡，并固定 `data-testid="qa-engine-settings"`。
- [x] 实现三个 provider 页签、Codex 状态/登录/刷新、可选模型覆盖、兼容 API 原字段和离线说明。
- [x] 区分全局 Codex 状态与仓库级 provider 保存；未选库时提供明确禁用说明。
- [x] `AskView` 删除 Luna modal/state/CSS，齿轮改为“前往设置”，provider badge 使用真实状态。
- [x] App 增加设置定位意图；进入设置后滚动/聚焦问答引擎区域。

## 7. 自动化验证

- [x] Node：P1/P2/research-trail/ingest/pagination/settings、新 QA/provider 状态测试。
- [x] Rust：fmt、Clippy `-D warnings`、完整 tests；fixture 不读取真实 Codex 凭据、不联网。
- [x] Python 45 tests、Wiki Eval 10/10、两书 Recall@5、Wiki Lint；不修改 Wiki/Raw/Graphify。
- [x] strict GUI 覆盖设置三模式、Codex ready fixture、AskView 跳转、provider 历史标签、取消和两个目标视口。
- [x] secret scan：SQLite、manifest、日志、错误、Git diff 中无 token/API Key；两个用户 raw 目录仍未跟踪。

## 8. 发布与 Git

- [x] 用户批准实施后将任务 `start`，主线程按 `trellis-before-dev` 实现并按 `trellis-check` 审查。
- [x] 版本提升到 0.11.0，同步 package/Cargo/Tauri/updater fixtures/README/root PRD/log/spec。
- [x] 构建 app/MSI/NSIS，执行 strict GUI 与隔离安装/启动/退出/卸载，记录 bytes/SHA-256。
- [x] 若用户同意真实 smoke，使用一条合成证据验证订阅回答；不保存到正式数据库。（本次未获独立批准，按边界未执行真实订阅请求。）
- [x] 显式暂存本任务文件，排除 `raw/inbox/auto-discovered/runs/`，提交、归档并记录 Trellis journal。

## 停止条件

- 任一实现需要读取/导出 ChatGPT cookie、Codex token 或生成通用 API Key。
- Codex 子进程可写知识库、继承用户 hooks/MCP/代理配置或在取消后残留。
- 自动测试消耗真实订阅额度、改变真实登录态或将真实回答写入正式会话。
- 设置仍分散在 AskView，或 provider 选择无法按仓库持久化。


## 实施结果

- fake Codex fixture 已覆盖 version、ChatGPT status、login launch、JSONL delta/final、非零失败脱敏、timeout 与 cancel；真实服务未被调用。
- 设置页加载竞态在 P5 首轮复合验证中复现并修复：provider 页签等待 `data-loaded=true`，异步旧快照不再覆盖用户点击。
- Rust 53/53、Python 45/45、前端全部状态测试、P1–P5、最终 release strict GUI 与 NSIS 安装生命周期通过。
- 0.11.0 app/MSI/NSIS 已构建并记录 bytes/SHA-256；两个用户 raw 运行目录未纳入任务改动。
