# 实施步骤

## A. 契约与纯逻辑

- [x] 扩展 Rust/TypeScript DTO：repository ID、conversation history、citation validation、失败状态。
- [x] 提取前端重试问题绑定、repository generation 和乐观消息回滚纯函数并补测试。
- [x] 实现后端历史截断、引用解析/校验和意图配置纯函数并补测试。

## B. 后端问答链路

- [x] 调整 prepare/prompt：先识别意图、意图感知召回、注入受限会话历史。
- [x] 增强 Graphify 一跳关系、community 和安全来源映射；过滤无来源节点。
- [x] 调整 `ask_luna`：仓库三段校验、Codex 按需单次探测、provider 失败不伪装离线。
- [x] 成功前执行引用校验；失败持久化结构化状态，成功持久化 validation。

## C. 前端交互

- [x] 仓库变化主动取消旧请求并用 generation 隔离事件/结果。
- [x] 每条 assistant 重试绑定相邻 user 消息。
- [x] 取消/失败回滚乐观消息和流式状态；展示持久化 failed 状态。
- [x] Codex 状态只在 Codex provider 下加载，设置入口继续支持显式刷新。
- [x] Graphify 来源按规范化 page ID 打开。

## D. 验证与交付

- [x] Rust QA/IPC/Graphify/仓库竞态测试。
- [x] 前端 QA 状态测试、TypeScript build、P3/P5 验证。
- [x] `cargo fmt --check`、Clippy `-D warnings`、Rust 全套测试。
- [x] 运行秘密扫描与 `git diff --check`，更新任务结果/规范。
- [x] Git 提交、Trellis 归档、journal、推送私有 `master`。

## 验证命令

```powershell
cd apps/desktop
npm run test:qa-settings
npm run test:p1
npm run build
npm run verify:p3
npm run verify:p5

cd src-tauri
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
