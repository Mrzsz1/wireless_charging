# Windows 客户端 P3 Luna 智能问答实施记录

- 日期：2026-08-02
- 版本：0.5.0
- 计划：`design/p3-luna-qa-plan.md`

## 子代理核验

按用户级子代理规则并行派出 3 个 `default` 探子，仅做只读探索：

1. 前端：确认“智能问答”原为占位页，定位 `App.tsx`、TabBar、右侧证据面板和 Tauri service 集成点；
2. 后端：确认 SQLite/FTS5、核心书籍、Graphify、命令注册、路径边界和测试缺口；
3. 质量：确认 Wiki 10 题契约、核心书籍 95% Recall 门禁、库水位/引用规则和构建脚本缺口。

方案取舍、代码修改和最终验收由主代理完成。

## 实现

### Rust / SQLite

- 新增 `src-tauri/src/qa.rs`；
- 使用 `PRAGMA user_version = 3`；
- 新增 `chat_sessions`、`chat_messages`、`chat_evidence`、`app_settings`；
- 会话按规范化仓库路径隔离，知识索引重建不删除聊天历史；
- 新增 Luna 设置、会话 CRUD、`prepare_question`、`ask_luna`、`cancel_answer` 命令；
- Wiki 与核心书籍使用 FTS5，Graphify 只作为关系提示；
- 全局排序后保证 Wiki/书籍来源多样性；
- Luna 使用 OpenAI-compatible Chat Completions SSE，支持超时、取消和离线证据降级；
- API Key 只从设置指定的环境变量读取，默认 `LUNA_API_KEY`。

### React / TypeScript

- 新增 `features/qa/AskView.tsx` 与 `AskView.css`；
- 接通“智能问答”导航和工作区 Tab；
- 实现会话历史、问题输入、检索状态、流式回答、停止/重试/复制；
- 实现右侧库水位、证据卡片、`[E#]` 引用校验与来源打开；
- 实现 Luna endpoint/model/环境变量名设置，前端不接触完整 API Key；
- Luna 未配置时明确显示“离线证据模式”。

## 自动验收

| 项目 | 结果 |
|------|------|
| `npm run build` | PASS |
| `npm run verify` | PASS，含 8 项 P3 结构/安全检查 |
| `cargo test` | PASS，12/12 |
| 真实仓库多路证据 | PASS，23 source / 20 method / 61 chapters |
| Wiki 固定问题召回 | PASS，10/10 每题至少一个预期 Wiki 证据 |
| `wiki_eval.py --answers-dir evals/answers` | PASS，10/10 |
| 核心书籍评测 | PASS，295 queries；AGT 1.000，Approximation 0.986667 |
| Tauri release | PASS，MSI + NSIS |
| release 启动冒烟 | PASS，进程稳定运行 8 秒 |

## 交付物

```text
apps/desktop/src-tauri/target/release/app.exe
apps/desktop/src-tauri/target/release/bundle/msi/Wireless Charging Research Workbench_0.5.0_x64_en-US.msi
apps/desktop/src-tauri/target/release/bundle/nsis/Wireless Charging Research Workbench_0.5.0_x64-setup.exe
```

SHA-256：

- MSI：`4E8BA8DD5BE9BF6D49FCE6DEE62C4709956E41B1BEE2C77ACF94F3696802308E`（6,201,344 bytes）
- NSIS：`E85A895ACBC1B2782EA558DD41794B4C19A792C0C662E73A17232346192F21BD`（4,441,154 bytes）

## 边界

- `raw/` 与 `wiki/` 正文未被客户端修改；
- Graphify 与 SQLite 仍是派生检索层；
- 默认不外搜；
- problem/idea 人工确认闸门保持不变；
- 编译中心仍进入 P4。
