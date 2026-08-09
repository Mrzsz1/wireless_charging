# Desktop 0.11.0：ChatGPT/Codex 订阅问答与设置收口

## 范围

- 回答引擎集中到“设置 → AI 回答引擎”。
- 新增 Codex 订阅、兼容 API、离线证据三种仓库级模式。
- 保留原有本地证据召回、`[E#]`、库水位、会话历史和离线回退。
- 不改 `raw/`、`wiki/`、`graphify-out/` 正文，不提交真实订阅问题。

## 实现

- Rust 新增 `codex_subscription.rs`：安全状态 DTO、官方浏览器登录启动、隔离的 `codex exec --json`、JSONL 宽容解析和 Windows 进程树清理。
- Codex 命令固定为 never approval、ephemeral、read-only sandbox、空临时目录、忽略用户配置与项目规则；prompt 只从 stdin 输入。
- `QaSettings` 新增 `answerProvider`、`codexModel`，按知识库保存；旧 Luna 键、兼容 API SSE 与历史 schema 保持兼容。
- SettingsView 成为唯一配置入口；AskView 只显示状态并跳转。设置加载期间禁用 provider 页签，避免异步快照覆盖新点击。
- fake Windows Codex executable 覆盖 version、ChatGPT status、login launch、delta/final、stderr failure、timeout 和 cancellation。

## 验证

- Rust：`cargo fmt --check`、Clippy `-D warnings`、53/53 tests。
- Node/前端：P1 8/8、P2 3/3、research trail 3/3、ingest 4/4、pagination 4/4、settings 3/3、QA provider 3/3、E2E config 5/5、installer lifecycle 2/2；TypeScript/Vite build 与结构门禁通过。
- Wiki/工具：Python 45/45、Wiki Eval 10/10、Algorithmic Game Theory Recall@5=1.000、Approximation Algorithms Recall@5=0.986667。
- Wiki Lint：0 errors、2 个既有 warnings；未以客户端改动处理 B 类页面或派生图覆盖警告。
- P3/P4/P5 通过；最终 0.11.0 release 严格 GUI 覆盖 1366×768 与 1920×1080，NSIS 在隔离目录完成安装、启动、完整进程退出和卸载。
- 真实 Codex 仅执行本地版本/登录状态检查；没有提交订阅回答请求。

## 发布产物

| 产物 | Bytes | SHA-256 |
|---|---:|---|
| `app.exe` | 21,342,720 | `F5A27BD6373B79A9F834927C97CA6EA67A5AF11E6D6EB8D9D41E67B5187E1D19` |
| MSI | 10,358,784 | `EBAD51C89A10373E536F71D95B14B04A811DC1842D5BCA3C09039F96F6CD54E8` |
| NSIS | 7,212,562 | `6C542249F5C683EA53C2E1E51B7C2A08D086247838DAE882C2BDF2239E4D874C` |

## 保留边界

- 两个用户运行目录继续未跟踪：`search-20260809-204315/`、`search-20260809-211516/`。
- Codex token、cookie、API Key、凭据路径和认证响应不进入 WebView、SQLite、日志、manifest 或 Git。
