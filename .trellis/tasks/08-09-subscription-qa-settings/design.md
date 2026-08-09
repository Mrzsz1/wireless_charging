# 设置页订阅问答模式：技术设计

## 1. 当前调用链

```text
AskView
  -> askLuna(request, Channel)
  -> Tauri ask_luna
  -> qa::prepare_question(SQLite + Wiki + core books + Graphify)
  -> qa::stream_luna(OpenAI-compatible SSE) OR qa::offline_answer
  -> qa::persist_exchange
  -> AnswerStreamEvent -> AskView
```

当前 `LunaSettings` 同时承担“选择引擎”和“兼容 API 参数”，且编辑 UI 位于 `AskView`。本次拆为通用问答设置、引擎适配器和设置页呈现，但保留既有检索、事件与持久化主干。

## 2. 目标架构

```text
SettingsView
  ├─ get/saveQaSettings --------------------------┐
  ├─ getCodexSubscriptionStatus ------------------┤ Tauri
  └─ startCodexLogin -----------------------------┘

AskView -> askQuestion(request, Channel)
               |
               +-> prepare_question (unchanged)
               |
               +-> answer_router(provider)
                    ├─ CodexSubscriptionAdapter
                    ├─ CompatibleApiAdapter (existing Luna stream)
                    └─ OfflineEvidenceAdapter
               |
               +-> persist_exchange (provider/model recorded)
```

保留 Tauri 命令 `ask_luna` 作为兼容别名或直接平滑改名为 `ask_question`，前端只使用新语义名称。若改名，结构门禁同时防止旧调用残留。

## 3. 数据契约

### 3.1 QaSettings

```ts
type AnswerProvider = 'codex-subscription' | 'compatible-api' | 'offline-evidence'

type QaSettings = {
  answerProvider: AnswerProvider
  codexModel: string          // 空值 = 跟随 Codex 订阅默认
  endpoint: string
  model: string
  apiKeyEnv: string
  timeoutSeconds: number
  maxOutputTokens: number
  temperature: number
  apiKeyConfigured: boolean
}
```

SQLite 新键只保存非秘密配置，例如 `qa.answer_provider`、`qa.codex_model`；现有 `luna.*` 键原样保留。`apiKeyConfigured` 继续是运行时派生值。

### 3.2 CodexSubscriptionStatus

```ts
type CodexSubscriptionStatus = {
  installed: boolean
  version: string
  authenticated: boolean
  ready: boolean
  statusLabel: string
  diagnostic: string
}
```

DTO 不包含 token、credential path、用户邮箱、组织、完整 stdout/stderr。状态命令只允许固定 `codex --version` 与 `codex login status`。

## 4. Codex CLI 适配器

### 4.1 固定命令形状

```text
codex -a never exec
  --json
  --ephemeral
  --skip-git-repo-check
  --ignore-user-config
  --ignore-rules
  --sandbox read-only
  --cd <APP_TEMP_EMPTY_DIR>
  [--model <OPTIONAL_CODEX_MODEL>]
  -
```

- Prompt 通过 stdin，绝不放进命令行参数或任务日志。
- `--ignore-user-config` 仍使用 `CODEX_HOME` 的官方认证，但隔离用户模型、hooks、MCP 与代理配置，避免问答触发外围工具。
- 空目录阻断项目 AGENTS/技能和知识库文件读取；知识只来自已召回的编号证据。
- `process_support` 统一设置 `CREATE_NO_WINDOW` 和 UTF-8 环境。

### 4.2 Prompt

复用 `build_prompt` 的库水位与证据格式，并增加回答契约：仅依据证据、每个事实带 `[E#]`、Graphify 不能单独支撑事实、禁止调用工具/读取文件、禁止生成补丁。问题与证据只存在于子进程 stdin 和内存，不写临时 prompt 文件。

### 4.3 JSONL 解析

新建强类型/宽容解析器：

- 接受已知 agent-message 增量事件并发送 `AnswerStreamEvent::Token`。
- 接受最终 agent-message/turn completed，提取最终文本。
- 忽略 reasoning、command/tool、usage 和未知版本事件。
- 若只有最终文本且之前未产生 token，则按 Unicode 字符边界分块回放。
- malformed JSON 行计数并脱敏，若最终文本有效则不因外围事件失败；无最终文本才返回稳定错误码。

解析器完全由本地 fixture 驱动，避免绑定单一 CLI 小版本。

## 5. 进程生命周期

- `spawn_blocking` 启动 Codex 子进程，stdin/stdout/stderr 全管道化。
- 为 requestId 保存取消 flag 与可终止 PID；取消时 Windows 使用隐藏 `taskkill /PID <pid> /T /F`，其他平台回退 `Child::kill`。
- stdout reader 与 stderr reader 并行，stderr 只保留长度受限的脱敏诊断。
- 进程退出、取消、超时、解析失败均走单一 cleanup guard：等待/终止、删除临时目录、移除 request 注册。
- 临时目录位于应用缓存或系统 temp 的固定前缀下，每次 UUID 隔离，不位于当前知识库。

## 6. 设置页与 AskView

### SettingsView

- 将现有“Luna 与模型”卡替换为“AI 回答引擎”。
- 顶部三段式 provider 选择；Codex pane 显示版本/登录/刷新/登录按钮/模型覆盖；API pane 显示原 Luna 字段；offline pane 解释只返回证据包。
- 保存按钮统一保存 provider 与相应非秘密参数。Codex 状态刷新独立于仓库选择。

### AskView

- 删除 `settingsOpen/settingsDraft` 与完整 modal JSX/CSS。
- 从 `QaSettings` 与 `CodexSubscriptionStatus` 计算状态 badge。
- 齿轮按钮通过 App 回调切换到 Settings，并聚焦 `#qa-engine-settings`。
- 历史 assistant 元数据显示 `Codex 订阅`、`Luna` 或`离线证据`。

## 7. 迁移矩阵

| 旧状态 | 无新 provider 键时的选择 |
|---|---|
| endpoint 非空且 API Key 环境变量有效 | `compatible-api` |
| 否则 Codex installed + authenticated | `codex-subscription` |
| 其余 | `offline-evidence` |

推导默认只在读取时执行；用户保存后写入 `qa.answer_provider`，以后不因环境变化自动切换。Codex 后续掉线时当前 provider 不被改写，执行时降级离线并提示修复入口。

## 8. 测试策略

- Rust：provider 迁移、命令 allowlist、状态脱敏、JSONL 增量/最终/未知/畸形、UTF-8、超时、取消进程树、离线回退、历史 provider。
- 前端：设置页拥有所有字段、AskView 无 modal、状态 badge、导航、provider 切换与仓库禁用矩阵。
- E2E：只使用 fixture Codex executable，严禁在自动测试中消耗真实订阅或改登录态。
- 手工 smoke：发布前单独运行一次合成 `[E1]` 问答，结果不落入知识库或正式会话库。

## 9. 回滚

- 设置 provider 切回 `compatible-api` 或 `offline-evidence` 即可停用新路径。
- 数据库仅增加 app_settings 键，无 destructive migration；旧客户端会忽略它们。
- 若 Codex 适配器异常，删除新模块/命令并恢复 AskView 设置跳转即可，历史消息不需迁移。
