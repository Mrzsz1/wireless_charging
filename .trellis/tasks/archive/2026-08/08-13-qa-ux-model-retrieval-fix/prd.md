# 智能问答交互、Codex 模型与论文检索修复

## Goal

修复智能问答输入区在无错误横幅时占满主工作区、Codex 订阅模式不能识别本机可用模型与推理强度、已入库“波干扰”论文查询被引用门禁误判失败，并新增基于真实流水线状态的 Thinking 动画、阶段列表与实时耗时，使科研问答既可用又可审计。

## Background

- 截图 1 的输入区异常并非 textarea 内容撑高，而是 `.qa-chat` 固定声明四行网格、错误横幅条件渲染；无错误时自动布局把 composer 放进 `minmax(0,1fr)` 的第三行，导致输入框占据剩余高度。
- 当前 Codex 设置只有自由文本 `codexModel`。空值虽然文案称“跟随 Codex”，但执行参数同时使用 `--ignore-user-config`，因此实际不会读取本机 `~/.codex/config.toml` 的模型与 reasoning effort，也没有使用 `models_cache.json` 中 Codex 已提供的可选模型清单。
- 本机失败记录表明论文检索已经命中《Concurrent Charging with Wave Interference》等原文；失败原因不是“论文不存在”，而是模型输出 `[E1；E5]`、`[E1，原文位置]` 等复合标记，严格校验器只接受独立的 `[E#]`，最终把 35 条事实中的 34 条判为无同句有效引用。
- 当前生成态仅区分 retrieving/generating，既没有校验阶段事件，也没有耗时；用户看不到“问题解析 → 本地检索 → 证据组装 → Thinking → 生成 → 引用校验”的真实进度。

## Requirements

### R1 — 紧凑且稳定的问答输入区

- 无论错误横幅是否存在，标题、错误、消息区和 composer 必须落在固定语义网格行，消息区独占可伸缩空间。
- textarea 初始保持约 3 行，随输入内容在有限高度内自动增长，超过上限后内部滚动；生成期间不得扩张到工作区高度。
- 保留 Enter 发送、Shift+Enter 换行、停止按钮和窄屏布局；错误横幅出现/关闭不得引发布局跳位。

### R2 — Codex 模型与推理强度自动识别/选择

- 从本机 Codex home 的非敏感配置读取当前默认 `model` 与 `model_reasoning_effort`，从 `models_cache.json` 读取 `visibility=list` 的模型及其支持的 reasoning levels。
- 设置页提供“自动（跟随 Codex 当前默认）”以及已识别模型下拉列表；提供与所选模型能力一致的推理强度下拉列表，支持自动跟随与显式覆盖。
- 继续隔离用户 rules/instructions；自动模式应把识别出的模型和 reasoning effort 以显式受控参数传给 `codex exec`，而不是放开 `--ignore-user-config`。
- 缓存缺失、格式损坏、配置模型不在列表或 Codex 未安装时 fail-soft：保留自动选项、给出可诊断状态，不阻断其他 provider。
- 状态 DTO、日志、SQLite 和测试 fixture 不包含 token、cookie、API Key、认证文件内容或完整用户配置。
- 问答页显示本轮实际生效模型与 reasoning effort；run manifest 继续记录 requested/resolved model。

### R3 — “波干扰论文”检索与回答契约

- 把“波干扰”与“波干涉”视为同一领域表达，优先扩展到 `wave interference`、`concurrent charging` 等已有论文术语。
- 识别“有没有/有哪些……论文或文献”为 literature lookup 意图，输出适合文献发现的问题模板，避免把简单找论文问题强制扩写成 35 条求解型声明。
- 文献查找回答仍应足够完整：列出库内匹配论文、每篇与问题的关联、主要模型/方法、证据边界和可复现定位；不得只返回标题。
- Prompt 明确规定多个引用必须写成独立 ASCII token：`[E1] [E5]`；禁止 `[E1；E5]`、`[E1, E5]`、`[E1-E5]` 或把位置说明塞进引用括号。
- 在严格校验前仅对可证明等价的复合引用语法做确定性规范化：所有编号必须存在于本轮 evidence，引用外正文保持逐字不变，不补充新事实、不猜测引用、不放宽同句引用门禁。
- citation repair/run manifest 记录规范化次数或原始组，旧 manifest 缺少新字段时保持可读。
- 模型原始 Markdown 必须先经过统一解析管线，形成章节、事实、引用编号和来源定位的一致表示；完整性校验、引用校验、持久化与前端展示使用同一份规范化结果，避免各层重复解释原始输出。
- 统一管线遵循“格式上宽容、语义上严格、输出上一致”：可接受确定等价的引用标点变体，但不得自动补事实、猜测证据或为无引用陈述添加编号。
- 对真实问题“有没有关于波干扰的论文”应召回库内目标原文并完成回答，不再产生 `CITATION_VALIDATION_FAILED`；若模型仍给事实不配引用，继续 fail closed。

### R4 — 可审计 Thinking 阶段与耗时

- 生成中的回答卡显示 `Thinking · Ns`，计时从提交开始，终态或取消时停止。
- 展示基于真实系统事件的处理链：理解问题、本地检索、整理证据、模型 Thinking、生成回答、引用与完整性校验；每项具有 waiting/active/completed 状态和动画。
- 后端在进入 grounding/completeness/persistence 前发送显式 validation event，前端增加 validating 阶段；不得把推测步骤标成已完成。
- 列表展示的是可审计处理阶段和检索结果摘要，不保存或展示模型隐藏推理草稿；流式回答仍按 token 正常显示。
- 失败、取消、重试、切换知识库和重新打开历史时，阶段状态、计时器与旧请求事件必须正确清理并保持 repository/request ID 隔离。

### R5 — 兼容、质量与交付

- Rust、TypeScript、Tauri Channel、settings SQLite、run manifest 和 UI DTO 同步；新增字段使用 serde/default 或前端默认值兼容旧数据。
- 不修改 `raw/`、`wiki/`、`schema/vocab.yaml`、Graphify 派生正文或 B 类页面；不默认外搜。
- 保持零证据语义、严格引用覆盖、Markdown 引用投影、completed 幂等、失败交换、取消与 repository 隔离等既有契约。
- 完成定向回归、前端构建、Rust fmt/clippy/test、P3/P5、Wiki lint 与差异审查；成功后提交 Git 并编译可运行客户端。

## Acceptance Criteria

- [ ] 无错误横幅和有错误横幅两种状态下，composer 均保持紧凑，消息区占据剩余高度；3 行初始高度和有限自动增长可用。
- [ ] 设置页自动发现本机 `gpt-5.6-sol`、`gpt-5.6-terra`、`gpt-5.6-luna` 等 `visibility=list` 模型，并仅显示所选模型支持的推理强度。
- [ ] 自动模式能识别本机 Codex 默认模型与 `xhigh` 等 reasoning effort，并在保留 `--ignore-user-config/--ignore-rules` 时显式传给执行命令。
- [ ] 缓存/配置缺失或损坏有单元测试覆盖，不泄露认证信息，也不让离线/API provider 退化。
- [ ] “有没有关于波干扰的论文”检索命中《Concurrent Charging with Wave Interference》和相关库内论文，回答包含关系说明、方法/边界和原文定位。
- [ ] `[E1；E5]`、`[E1, E5]` 与合法的 `[E1，位置说明]` fixture 被规范化为独立已知 `[E#]` token；未知 ID、范围写法、无来源事实仍被拒绝。
- [ ] 校验、数据库保存和前端引用按钮消费同一规范化 Markdown；重新打开会话不会恢复原始复合引用格式。
- [ ] 文献查找意图使用专用回答结构，事实逐句引用且回答不过度膨胀；现有 solve/novelty/relationship 结构不退化。
- [ ] 生成中显示阶段链、`Thinking` 和每秒更新的耗时；后端 validation event 能驱动“引用与完整性校验”步骤。
- [ ] 停止、失败、完成、重试和切库均清理 timer/phase；旧请求事件不会更新新仓库 UI。
- [ ] 前端定向测试、QA Settings、`npm run build`、Rust 定向/全量测试、fmt、Clippy、P3/P5、Wiki lint 与 `git diff --check` 通过。
- [ ] 变更提交到 Git；编译产物启动验证通过，未跟踪交接文档不被误提交。

## Out of Scope

- 展示或持久化模型内部隐藏推理文本。
- 新增外部论文搜索或自动下载未入库论文。
- 放宽逐事实同句引用、引入 claim-level NLI 或自动为无引用事实猜测证据。
- 重做整个智能问答视觉系统、会话数据库或检索架构。
- 修改 Codex 认证流程、读取认证令牌或复制 Codex 完整用户配置。
