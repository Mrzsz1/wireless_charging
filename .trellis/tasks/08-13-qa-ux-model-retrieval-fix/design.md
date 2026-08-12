# 技术设计：智能问答交互、Codex 模型与论文检索修复

## 1. 根因与设计原则

### 1.1 Composer 高度

当前 `.qa-chat` 固定四行：`auto auto minmax(0,1fr) auto`，但 `.qa-error` 是条件节点。错误为空时 DOM 只有 heading/messages/composer 三个直接子节点，CSS 自动放置把 composer 分配到第三个 `1fr` 行，故截图出现整屏输入区。修复采用显式 grid-row/area，不再依赖条件节点数量。

### 1.2 Codex 自动模型

`codexModel=''` 当前没有真正跟随 Codex，因为 `build_exec_args()` 强制 `--ignore-user-config`。设计保持该隔离开关，仅读取两个允许字段并显式下发：

```text
CODEX_HOME/config.toml -> model, model_reasoning_effort
CODEX_HOME/models_cache.json -> visibility=list 模型及 supported_reasoning_levels
```

认证材料、rules、MCP 和其它配置不进入 DTO。配置解析和执行选择分离，便于 fail-soft 和纯单元测试。

### 1.3 论文回答失败

运行库已经召回正确论文。模型把编号与中文分隔符/位置文字合并在一个方括号中，而 lexer 只认精确 `[E数字]`，于是引用覆盖率接近零。修复不降低门禁，而是同时收紧生成契约、增加安全的等价语法 canonicalization，并为 literature lookup 使用更合适的回答 schema。

### 1.4 Thinking

Thinking 面板由 request lifecycle 事件驱动，展示系统可验证的阶段，不依赖模型自报推理。计时是 UI 单调时钟；最终科研审计仍由 evidence、citation validation 和 run manifest 承担。

## 2. Codex 模型发现与选择

### 2.1 数据结构

新增序列化结构：

```rust
CodexModelOption {
  id: String,
  display_name: String,
  default_reasoning_effort: String,
  supported_reasoning_efforts: Vec<String>,
}

CodexSubscriptionStatus {
  ...existing,
  configured_model: String,
  configured_reasoning_effort: String,
  available_models: Vec<CodexModelOption>,
  model_catalog_status: "detected" | "missing" | "invalid",
}
```

`QaSettings` 增加 `codexReasoningEffort`，空字符串表示自动。旧库读取不到该 key 时默认空。

### 2.2 发现算法

1. Codex home 优先 `CODEX_HOME`，否则 `$HOME/.codex`。
2. 使用 TOML parser 读取 `config.toml`，只投影顶层字符串 `model`、`model_reasoning_effort`；不序列化原始文档。
3. 使用 serde JSON 读取 cache，仅保留 `visibility == "list"`、合法 slug、display name、default/supported reasoning levels；按 `priority` 和文件顺序稳定排序、slug 去重。
4. cache 缺失时，如果配置模型合法，则构造单项 fallback；损坏时状态标为 invalid，但 Codex 登录 readiness 独立保留。
5. Status cache 的 TTL 继续存在；设置页显式刷新重新读取目录。

### 2.3 有效选择

```text
effectiveModel = settings.codexModel || status.configuredModel || ""
effectiveEffort = settings.codexReasoningEffort
               || status.configuredReasoningEffort
               || selectedModel.defaultReasoningEffort
               || ""
```

- 手动模型不在 catalog：设置页显示“已保存但当前未发现”，执行仍可显式传递，避免破坏旧配置。
- reasoning effort 不受该模型支持：保存时拒绝或清空为自动，不把无效值传给 CLI。
- `build_exec_args` 保留 `--ignore-user-config --ignore-rules --sandbox read-only`，增加 `--model effectiveModel` 和安全的 `-c model_reasoning_effort=...`；参数值只接受 allowlist effort。
- provider metadata 的 requested model 使用 effective model；resolved model优先 CLI JSONL 实际报告。

### 2.4 UI

- 模型 `<select>` 第一项为“自动（当前：DISPLAY_NAME）”，后续为 catalog 列表。
- 推理强度 `<select>` 第一项为“自动（当前：极高等）”，选项随有效模型变化。
- 问答页状态胶囊显示 `Codex · DISPLAY_NAME · 极高`，状态不可用时回退现有 readiness 文案。

## 3. 引用语法规范化与 literature intent

### 3.0 统一输出解析管线

模型回答不再由完整性校验器、引用校验器和前端分别解析。后端先生成统一的 `ParsedAnswer`：

```text
ParsedAnswer
├── normalized_markdown
├── sections[] { heading, body }
├── claims[] { text, citation_ids, source_location, section }
└── citation_repair
```

处理顺序固定为：Markdown 隐藏区域识别 → 引用语法规范化 → 章节解析 → 事实切分与引用绑定 → 完整性/grounding 校验 → 保存 `normalized_markdown` → 前端渲染。规范化先于事实切分，避免复合引用内部的中文分号被当成新的事实边界。首阶段可以复用现有 claim splitter 和 completeness API，由 `ParsedAnswer` 统一编排而不进行无关的大规模重构。

### 3.1 Canonicalization

在 Markdown mask 之后、未知引用修复之前，扫描普通文本中的方括号区域。只处理满足以下语法的组：

```text
[E<digits> (separator E<digits>)+ (optional source-location text)]
separator := ; | ； | , | ， | 、
```

处理规则：

- 至少提取一个编号；每个编号都必须存在于本轮 evidence。
- 若含位置说明，只允许它跟随首个或单个编号并移到引用 token 后作为普通文字；不得把说明误识别成 ID。
- `[E1；E5]` -> `[E1] [E5]`。
- `[E1，II. PRELIMINARIES · 原文第 69–70 行]` -> `[E1]（II. PRELIMINARIES · 原文第 69–70 行）`。
- 范围 `[E1-E5]`、未知 `[E99]`、代码/数学/Markdown link label/target 内内容不规范化。
- 正文事实字符保持不变；只替换引用容器的标点与括号。之后仍运行同句 claim coverage 校验。

`CitationRepair` 增加带默认值的 `normalizedCitationGroups` 与可选规范化摘要（只含 evidence ID，不存答案正文），`applied` 在删除未知 ID 或规范化发生时为 true。

最终 `AnswerAudit.answer`、`chat_messages.content`、stream completed result 和 Markdown renderer 都使用 `normalized_markdown`；模型原始文本只在当前请求内短暂存在，不作为第二套持久化真相。

### 3.2 Prompt 契约

- research contract 与 answer contract 增加精确例子：多个来源写 `[E1] [E5]`。
- 明确 sourceLocation 写在引用 token 外，例如 `原文第 69–70 行 [E1]`。
- Prompt/answer schema 版本递增，run manifest 可区分修复前后运行。

### 3.3 Literature lookup

新增 `INTENT_LITERATURE`，在 novelty 判定之前识别：

```text
论文、文献、paper、papers、literature、有没有关于、有哪些关于
```

其中包含“研究空白/创新/有没有人做”等强 novelty 标记时仍归 novelty，避免语义冲突。

literature 回答结构：

```text
## 结论
## 库内相关论文
## 主题、模型与方法
## 边界与复现信息
```

必填元素：论文标题、与问题的关系、模型或方法、适用/证据边界、来源定位。每篇一条，事实逐句绑定独立 `[E#]`，最低事实数为 2。其它 intent 继续使用原六标题和最低 3 条事实。

### 3.4 检索词

把 curated alias 的 needle 扩展为“波干涉/波干扰”，保持 domain terms 在长中文 raw term 之前，增加 fixture 断言目标论文位于前列。检索仍只使用当前 SQLite/Wiki/Graphify，不外搜。

## 4. Thinking 状态机

### 4.1 后端事件

`AnswerStreamEvent` 增加：

```text
validation_started { requestId }
```

发送点位于 provider 输出结束、确定性引用规范化/完整性/grounding 审计及持久化之前。若 provider 在首 token 前等待，前端处于 generating + no token，即模型 Thinking。

### 4.2 前端状态

Phase 扩展为：

```text
idle | retrieving | generating | validating
```

请求局部时间状态：

```text
startedAt: performance.now()
elapsedSeconds: integer
hasFirstToken: boolean
```

阶段投影：

| 阶段 | 状态来源 |
|---|---|
| 理解问题 | submit/started 后 completed |
| 本地检索 | retrieval_started active；retrieval_completed completed |
| 整理证据 | retrieval_completed 后 completed |
| Thinking | generating 且无首 token 时 active；首 token 后 completed |
| 生成回答 | 首 token后 active；validation_started 后 completed |
| 引用与完整性校验 | validation_started active；completed 后 completed |

完成时卡片由 persisted assistant 替换，不额外保存时间；失败/取消调用统一 `resetGenerationState()` 清理 interval、phase 和临时步骤。

### 4.3 视觉与无障碍

- 卡片顶部 `Thinking · 12s`，active 行使用 pulse/dot/line 动画，completed 使用 check icon，waiting 使用低对比样式。
- 使用 `aria-live="polite"`，动画尊重 `prefers-reduced-motion`。
- 流式正文存在时，阶段摘要收拢到标题下方，不遮挡回答。

## 5. Composer 布局

采用显式行：

```css
.qa-chat { grid-template-rows:auto auto minmax(0,1fr) auto; }
.qa-chat-heading { grid-row:1; }
.qa-error { grid-row:2; }
.qa-messages { grid-row:3; }
.qa-composer { grid-row:4; }
```

空的第二行自然为 0；条件节点不再改变后续位置。textarea 基于 `scrollHeight` 在 3 行最小高度和约 8 行最大高度之间调整，提交/清空后复位，超过上限 `overflow-y:auto`。

## 6. 测试策略

### Rust

- config/cache parser：正常、缺失、损坏、hidden model、重复、非法 effort、secret-free DTO。
- exec args：auto/override model 与 effort，保留隔离/sandbox 参数。
- citation canonicalization：中文/英文分隔符、位置说明、未知 ID、范围、代码/数学/Markdown link 边界。
- 一致性：audit、SQLite round-trip、completed DTO 与前端 citation projection 得到完全相同的规范化 Markdown。
- intent/prompt/completeness：literature 与 novelty 冲突优先级，专用标题与最低事实数。
- `波干扰` query terms/真实 SQLite fixture 检索回归；不把 Graphify 单独当事实证据。
- validation event 序列和现有失败/取消终态。

### TypeScript/UI

- status/QaSettings DTO defaults、模型/effort 下拉联动、auto label。
- phase projection、timer reset、first-token/validation event、stale repository event。
- composer row contract和有限自动增长。

### 全量门禁

保留现有 P1/P3/P5、QA settings、gold contract、两书 Recall@5、Rust全量、TypeScript build、fmt/clippy、Wiki lint 和秘密扫描。

## 7. 兼容与回滚

- 新 settings key 缺失默认自动；旧 `codexModel` 文本覆盖继续有效。
- Status DTO 新字段由前端默认值兼容短暂的旧后端响应。
- `CitationRepair` 新字段使用 serde default，旧 SQLite manifest 可读。
- Prompt/answer schema 升级只影响新运行，旧会话按已存 JSON 展示。
- 若 model cache 不可读，自动执行仍可使用 config 模型；两者都缺失时沿用 provider default 并明确标为未报告。
- 规范化器只处理已知 ID 的可证明等价语法，可单独回滚而不改变严格 validator。
