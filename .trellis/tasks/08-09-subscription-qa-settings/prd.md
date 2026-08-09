# 设置页订阅问答模式

## Goal

让只有 ChatGPT/Codex 订阅、没有独立 OpenAI API Key 的用户，也能在 Windows 客户端中使用“智能问答”。所有回答引擎的选择、登录状态和连接参数统一放在左侧“设置”页面；“智能问答”页面只负责提问、证据检索、生成、取消和历史浏览。

## Background

- 当前问答仅有 OpenAI-compatible Luna HTTP 流和离线证据回退。`LunaSettings` 按知识库保存在 SQLite，API Key 只能从进程环境变量读取。
- `AskView` 仍内置“Luna 设置”弹窗，而 0.10.0 的全局设置页只有一个“前往智能问答”提示卡，不符合“配置全部放在设置”的产品方向。
- 本机已安装 `codex-cli 0.146.0`，`codex login status` 返回 `Logged in using ChatGPT`。用户已具备 ChatGPT 订阅登录，不需要手工导入 API Key。
- ChatGPT 订阅与普通 API 计费分离；本功能复用 Codex CLI 的 ChatGPT 登录状态，不读取、不复制、不回显 Codex 本地凭据。
- 现有 Wiki/Core-book/Graphify 证据召回、`[E#]` 引用、库水位、会话历史和取消入口必须继续成立。

## Requirements

### R1. 所有问答配置集中到设置页

- “设置”新增完整“AI 回答引擎”区域，代替现有“Luna 与模型”跳转卡。
- 区域提供三个互斥模式：`Codex 订阅`、`兼容 API`、`仅离线证据`。
- Codex 订阅为没有传统 API 配置时的推荐模式；兼容 API 保留现有 endpoint/model/API Key 环境变量/超时/输出长度/temperature 配置。
- “智能问答”页面删除 Luna 配置弹窗。页头只显示当前引擎、可用状态和“前往设置”按钮。
- 未选择知识库时可查看本机 Codex 状态，但回答引擎偏好与兼容 API 参数必须显示为不可保存，因为它们按知识库隔离。

### R2. Codex 订阅状态与登录入口

- 设置页显示 Codex CLI 是否安装、版本、ChatGPT 登录状态、当前是否可用于问答，以及最近检测错误的安全摘要。
- 提供“刷新状态”和“登录 ChatGPT”按钮；登录按钮启动 Codex 官方浏览器登录流程，客户端不采集账号、密码、cookie、access token 或 API Key。
- 已登录状态不得暴露凭据文件路径、token、组织密钥或认证响应正文。
- Codex CLI 缺失、版本命令失败、未登录或登录流程启动失败时，界面必须给出可执行诊断，不能伪装成功。
- 本期不在客户端执行全局 `codex logout`，避免误伤用户的 Codex/Codex App 会话。

### R3. 订阅问答执行

- 继续先运行本地 `prepare_question`，把 Wiki、核心专著和 Graphify 召回证据及库水位传给回答引擎。
- Codex 模式使用本机登录态启动非交互 `codex exec`；不生成、不读取传统 OpenAI API Key。
- 每次问答使用一次性会话与受控空工作目录、`read-only` sandbox、`never` approval、ephemeral 模式，禁止写知识库并避免继承项目任务上下文。
- 只通过 stdin 传入问题、系统约束、库水位和编号证据。回答仍必须使用 `[E#]`，Graphify 只能作为关系提示，库内未见不得表述为全球不存在。
- 解析 Codex JSONL：可用增量消息则实时发送 token；只有最终消息时以稳定分块回放到现有 UI。忽略推理、工具调用和非回答事件。
- 最终会话记录保存 `provider=codex-subscription` 和实际/可识别模型；失败时保留本地离线证据回退并显示 Codex 原因的脱敏摘要。

### R4. 取消、超时与进程可靠性

- 点击“停止”必须停止生成、终止 Codex 子进程树、清理受控临时目录，并且不写入半完成会话。
- Codex 进程使用 Windows 隐藏后台配置，不弹出终端窗口；stdin/stdout/stderr 固定 UTF-8。
- 设置超时后先请求终止，再强制清理进程树；任何路径都必须移除取消注册项和临时材料。
- stderr、JSONL 解析错误和状态检测错误不得包含凭据、Authorization、access token 或完整认证载荷。
- 并发仍遵守现有“一次问答一个 requestId”的状态机和完成幂等契约。

### R5. 迁移与兼容

- 新增按仓库保存的 `answerProvider`；现有 Luna 参数和历史消息 schema 保持兼容。
- 首次迁移规则：已配置可用 Luna endpoint + Key 环境变量时保持 `compatible-api`；否则本机 Codex 已登录时选择 `codex-subscription`；其余选择 `offline-evidence`。
- 兼容 API 继续使用现有 HTTP SSE 行为；离线模式继续生成确定性证据包。
- 旧历史消息照常显示；新历史根据 `provider` 显示“Codex 订阅 / Luna / 离线证据”。

### R6. 质量、发布与文档

- 增加纯 fixture 的 Codex 状态/JSONL/失败/超时/取消测试，自动化测试不得消耗真实订阅额度或修改真实 Codex 登录态。
- 严格 GUI 覆盖设置页三个模式、已登录状态、问答页设置跳转、Codex provider 标识以及 1366×768/1920×1080 布局。
- 完成 Rust fmt/clippy/test、前端状态测试/build、P1–P5、Python/Wiki/Core-book 门禁和安装生命周期。
- 版本目标为 0.11.0；更新根 PRD、桌面 README、Trellis spec、日志、发布产物哈希并用 Git 保存。
- 发布前可执行一次用户明确批准范围内的真实订阅 smoke：只传合成证据、要求一句含 `[E1]` 的回答，不读取或修改知识库。

## Acceptance Criteria

- [x] AC1 / R1：所有回答引擎配置只能在“设置”编辑；AskView 不再包含 Luna 配置弹窗，设置按钮能定位到对应设置区域。
- [x] AC2 / R1：设置页可选择 Codex 订阅、兼容 API、离线证据，保存后重新进入仍保持，仓库切换互相隔离。
- [x] AC3 / R2：真实安装版能显示 Codex CLI 版本和 ChatGPT 登录状态；未登录时可启动浏览器登录流程，客户端从不接触认证秘密。
- [x] AC4 / R3：Codex 模式先完成现有混合召回，再将受控编号证据交给 `codex exec`，回答中的 `[E#]` 可打开原来源。
- [x] AC5 / R3：JSONL 增量/最终消息均能显示；最终历史记录 provider 为 `codex-subscription`，兼容 API/离线 provider 不回归。
- [x] AC6 / R4：取消与超时会终止完整 Codex 进程树、清理临时目录、不保存半完成对话，且 Windows 不弹出控制台。
- [x] AC7 / R5：已有 Luna 配置、旧会话和无 Codex 环境都按迁移矩阵正常工作，不要求用户重新设置。
- [x] AC8 / R6：fixture 测试不调用真实 Codex 服务；完整门禁、严格 GUI、0.11.0 release/MSI/NSIS 和安装生命周期通过。
- [x] AC9 / R6：日志、SQLite、错误、manifest、Git diff 与前端状态中不存在 Codex token/API Key；两个用户 raw 运行目录继续保持未跟踪。

## Out of Scope

- 绕过 ChatGPT/Codex 套餐限制、复用浏览器 cookie、导出 Codex token 或把订阅伪装成通用 OpenAI API。
- 在客户端执行 Codex logout、切换组织、购买额度或管理订阅账单。
- 本阶段接入 Ollama/LM Studio；接口设计保留未来 provider 扩展位。
- 让 Codex 自由修改知识库、执行编译任务或替代现有本地证据检索。
