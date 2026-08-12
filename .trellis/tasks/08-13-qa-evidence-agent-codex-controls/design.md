# 技术设计：多轮证据分级 Agent 与 Codex 聊天模型控制

## 1. 总体边界

```text
Chat composer selection
  -> AskRequest(model/effort snapshot)
  -> bounded QueryPlan (1..3 passes)
  -> fused evidence package
  -> shared PromptEnvelope
  -> Codex/API stream with activity timeout
  -> normalized Markdown
  -> structural grounding classification
  -> trusted-context projection + persistence
  -> message/evidence/audit UI
```

语义蕴含保持在边界外。所有“已验证”仅表示引用编号真实、来源允许且结构上与声明同句绑定。

## 2. 查询规划器

### 2.1 数据结构

新增内部结构：

```rust
RetrievalPass {
  ordinal: usize,
  strategy: "initial" | "index_expansion" | "fragment_fallback",
  terms: Vec<String>,
}

RetrievalLoopDiagnostics {
  pass_count: usize,
  stop_reason: "sufficient" | "no_novel_terms" | "low_gain" | "max_passes",
  candidate_gain: Vec<usize>,
}
```

跨层诊断只输出序号、策略枚举、计数、耗时和停止原因，不输出 terms。

### 2.2 可组合术语

将现有整句 alias 表替换为概念级中英映射，例如“干扰→interference”“并发→concurrent”“轨迹→trajectory”“请求→request”“收费→pricing”。原问题的英文/数字 token 与有界中文 3–4 字片段始终作为候选，但按优先级和全局上限去重。

### 2.3 多轮执行

1. Pass 1：组合概念词 + 原始词。
2. 对 Pass 1 的高分 Wiki/论文/书籍候选标题进行安全分词；只保留与原始 terms 至少存在一个交集的标题词，过滤停用词、过短词和证据编号，生成 novel terms。
3. 若意图需要的证据类型齐备且有效候选达到阈值，则停止；否则执行 Pass 2。
4. 若无 novel terms，直接使用此前未加入的中文片段执行 fallback；无新增时停止。
5. Pass 2/3 的候选进入同一 RRF 融合池，以稳定 source key 去重；新增唯一候选低于阈值时停止。
6. 最多三轮，之后执行现有 diversity/retention/pair repair。

## 3. 证据分级回答

### 3.1 输出契约

所有 intent 的末尾允许可选章节：

```markdown
## 模型补充（可能不准确）
> 以下内容来自模型一般知识，未由当前知识库证据核验，可能不准确。
...
```

该章节内禁止 `[E#]`、wikilink 和位置伪装；章节外事实保持原有严格门禁。

### 3.2 Structural validation

`grounding.rs` 在已有 Markdown mask/claim splitter 上增加 heading-aware claim projection：

- verified claims：模型补充章节外的事实；必须有当前非 Graphify 引用。
- model claims：模型补充章节内的事实；必须没有任何 `[E#]`，并由固定提示行标识。
- unknown/graph-only/uncited outside section：invalid。
- only verified claims：supported。
- verified + model claims：mixed。
- zero evidence + fixed notice：unverified。

`CitationValidation` 新增 `modelSupplementClaims`、`modelSupplementClaimCount`；`unsupportedClaims` 只保存真正违规声明。`entailmentChecked` 继续为 false。

### 3.3 可信历史投影

`chat_messages.content` 继续保存完整 Markdown。新增 `trusted_context` 列：

- supported：完整规范化答案去除旧 `[E#]` 后的文本。
- mixed：移除“模型补充”标题、提示和该章节正文，仅保留验证章节，并移除旧引用编号。
- unverified/failed：空。

`conversation_history` 对 assistant 使用 `trusted_context`，对 user 仅选择 status=completed/mixed 且与具有非空 trusted assistant 的同 request 完整配对；因此 mixed 的用户问题可延续，但模型推测不会进入历史。旧记录无 trusted_context 时，completed 回退现有 content，其他状态为空。

## 4. Codex 模型能力与请求快照

### 4.1 数据来源

- `config.toml`：只投影顶层 `model`、`model_reasoning_effort`。
- `models_cache.json`：只投影 `visibility=list`、slug、display_name、priority、default_reasoning_level、supported_reasoning_levels[].effort。
- 目录以本机 Codex 缓存为主，不静态编造 GPT 模型；官方文档仅用于 reasoning effort 语义与 fallback 合法值校验。

目录解析按 priority/file order 排序、slug 去重，并保留 `none/low/medium/high/xhigh/max/ultra` 中模型实际报告的集合。未知字段忽略，缓存损坏 fail-soft。

### 4.2 AskRequest

新增可选：

```text
codexModel
codexReasoningEffort
```

仅 Codex provider 消费。请求字段“缺失”兼容旧客户端并回退 repository saved default；“显式空字符串”表示 composer 选择了“自动跟随 Codex”。后端再按 Codex configured default -> model default 解析，并校验 effort 是否属于模型能力集合。run manifest 记录最终 requested/resolved model。

### 4.3 前端状态

AskView 加载 settings/status 后维护 composer selection。选择变化：

1. 立即更新本地 UI。
2. 异步保存 repository default；失败时保留本轮选择并显示轻量错误。
3. submit 把当前快照放入 AskRequest，避免保存竞态。
4. 模型改变时把无效 effort 归零为 auto。

SettingsView 删除重复选择字段，但不删除底层设置兼容键。

## 5. 超时状态机

Codex 生成维护：

```text
started_at
last_activity_at
idle_timeout = configured timeout (default 180s)
hard_timeout = clamp(max(idle_timeout * 4, 600s), 600s..1800s)
```

任意成功读取的 stdout JSONL 行刷新 `last_activity_at`；文本 delta 继续流式发送。循环依次检查 cancel、hard timeout、idle timeout、进程退出。错误码：

- `CODEX_IDLE_TIMEOUT`
- `CODEX_TOTAL_TIMEOUT`
- `CODEX_CANCELLED`

为避免老设置 90 秒继续频繁误杀，读取时把未显式迁移的默认 90 提升到 180；设置允许 30–600 秒。API provider继续使用原 `reqwest::Client.timeout`。

## 6. UI 设计

Composer footer 左侧从提示文字改为：

```text
[provider状态]  [模型 ▾]  [推理强度 ▾]
                                      [Stop/Send]
```

控件使用透明背景、现有 `#6b8192` 文本、1px hover/focus 边界、4px 圆角、10px 字号；不增加卡片嵌套或强阴影。标题右侧仅保留短 readiness 与设置按钮。窄屏下左侧横向滚动/截断。

Thinking 链根据检索 pass 事件或 retrieval diagnostics 显示“扩展查询/再次检索”，只呈现可审计系统阶段。

## 7. 兼容、迁移与回滚

- SQLite 新列使用幂等 `column_exists` 迁移，不修改全局 `user_version`。
- DTO 新字段 serde/default；前端给旧后端响应提供空数组/默认值。
- 旧 completed 记录继续按 content 进入历史；新 mixed/unverified 必须依赖 trusted_context 隔离。
- 若多轮检索导致回归，可在规划器内退回单 pass，不影响 grounding/persistence。
- 若 composer catalog 不可用，显示“Codex 默认/模型默认”，仍允许发送。
- 未验证/混合状态不改变原始 answer/audit evidence，可完整复现。
