# 技术设计：智能问答第二轮可靠性修复

## 1. 设计目标

本阶段在不重做问答 UI、不改变知识库治理边界的前提下，修复五个跨层可靠性缺陷。核心原则是：

1. **请求身份先于工作**：客户端 request ID、取消登记和 `started` 事件必须发生在历史读取、检索、Codex 探测和生成之前。
2. **检索与生成分离**：历史可帮助构造检索查询，但只有本轮召回结果是 evidence。
3. **有来源与无来源显式分型**：有证据回答保持严格 `[E#]` 契约；零证据模型回答使用 `unverified` 状态和服务器所有的固定声明。
4. **持久化按交换成对**：completed、unverified、failed 都保存成对 user/assistant；cancelled 和 repository-changed 不落库。
5. **阻塞工作离开 async executor**：SQLite/FTS、Graphify 读取解析和 Codex CLI 探测运行在 blocking worker，repository mutex 只用于短快照和最终写入。

## 2. 跨层数据契约

### 2.1 AskRequest

TypeScript 与 Rust 的 `AskRequest` 增加必填 `requestId`：

```text
requestId: UUID string
question: string
sessionId?: string
evidenceLimit?: number
repositoryId: normalized repository identity
```

- 前端使用 `crypto.randomUUID()` 在提交前创建 ID，并立即写入 `requestId` state 与 `activeRequestId` ref。
- 后端拒绝空白、非 UUID、过长或已处于 active 状态的 request ID。
- 同一 request ID 贯穿事件、SQLite 两条消息、completion ledger 和取消表。

### 2.2 RetrievalQuery

新增内部结构：

```rust
struct RetrievalQuery {
    original_question: String,
    resolved_question: String,
    entities: Vec<String>,
    intent: String,
    used_history_message_ids: Vec<String>,
}
```

`QuestionContext` 保存该结构或其中等价的可序列化字段，便于测试和诊断；Prompt 仍分别接收 original question、受限历史和本轮 evidence。

### 2.3 ConversationTurn

`ConversationTurn` 增加 `id` 和 `requestId`。SQL 只读取：

```text
status='completed' AND role IN ('user','assistant')
```

因此 `failed`、`cancelled`、`unverified` 均不进入后续 query rewrite 或 Prompt。历史上限继续保持 8 条、12,000 字符。

### 2.4 CitationValidation / grounding

保留现有字段并增加：

```text
groundingStatus: supported | unverified | invalid
zeroEvidence: boolean
```

规则：

| evidence | 回答 | groundingStatus | supported | 持久化状态 |
|---|---|---|---|---|
| 非空 | 仅已登记 `[E#]` 且至少一条引用 | supported | true | completed |
| 非空 | 缺引用或含未知编号 | invalid | false | failed 交换 |
| 空 | 服务器固定无来源声明 + 无 `[E#]` | unverified | false | unverified 交换 |
| 空 | 声称有本库依据或伪造 `[E#]` | invalid | false | failed 交换 |

旧 SQLite JSON 缺少新字段时用 serde default 兼容读取。

### 2.5 FailedExchange

失败事件扩展为可携带已持久化交换：

```text
requestId
code
message
retryable
exchange?: { sessionId, userMessage, assistantMessage }
```

如果 repository 已切换或失败发生在创建请求身份之前，`exchange` 为空且不落库。

## 3. 多轮 query rewrite

### 3.1 顺序

```text
读取受限 completed 历史
  → 识别当前 intent / 指代标记
  → 确定性抽取近期实体
  → 构造 RetrievalQuery
  → 多通道检索
  → Prompt 使用原问题 + 受限历史 + 本轮 evidence
```

### 3.2 确定性实体解析

仅在当前问题包含明确指代或短比较追问时启用历史补全，例如：`它`、`它们`、`二者`、`这些`、`上述`、`前者/后者`、`they/them/these/those/both`。

实体来源按优先级：

1. 最近一条 completed user 消息中的大写缩写、字母数字模型名、引号内短语；
2. 当前 pages 索引中能在最近 user/assistant 文本中精确出现的页面标题/稳定 ID；
3. 最近 assistant 消息中的同类实体。

约束：实体去重，过滤 `[E#]`，最多 8 个、总长度最多 256 字符；优先最近 user，避免完整 assistant 回答污染 FTS。`resolved_question` 仅以结构化后缀加入实体，不改写用户原文。

CCSP/GAIN 用例应得到等价于：

```text
original: 它们的约束有什么区别？
entities: [CCSP, GAIN]
resolved: 它们的约束有什么区别？ 相关实体：CCSP；GAIN
```

## 4. 请求生命周期与取消

### 4.1 新时序

```text
前端生成 requestId、显示 optimistic user、立即启用停止
  ↓
后端校验 requestId/repositoryId，生成或复用 sessionId
  ↓
登记 cancellation，发送 started + retrieval_started
  ↓
spawn_blocking：打开独立 SQLite connection，读取设置/历史并检索
  ↓（每通道检查 cancel/repository generation）
发送 retrieval_completed
  ↓
按需读取短 TTL Codex 状态 → provider 生成
  ↓
取消/仓库检查 → 校验 grounding → 事务持久化 → completed
```

### 4.2 repository 锁边界

复用编译中心的 `root + db_path` 快照模式：

- async 主线程持锁只复制 `root`、`Connection::path()` 和 authoritative repository ID；
- blocking worker 用 db path 打开独立连接执行只读查询；
- 最终事务仍在当前 repository connection 上执行，并再次核验 repository ID；
- 检索期间切库由 cancel + 后置 identity check 丢弃旧结果。

只读检索连接使用一致的 SQLite 读事务，避免索引更新时混合两个快照。

### 4.3 早到取消与重复 ID

取消表支持短期 early-cancel tombstone：若 `cancel_answer` 比 `ask_luna` 的登记更早到达，记录已取消标记；`ask_luna` 登记时复用该标记并立即走 cancelled。过期 tombstone 在登记/取消时清理，防止未知 ID 永久占用。

已登记且未取消的相同 request ID 返回 `REQUEST_ID_ACTIVE`，不得覆盖原 flag。所有终态通过统一 cleanup guard 移除 active entry。

### 4.4 取消检查点

- 历史读取前后；
- Wiki、paper、linked paper、book、Graphify 各通道之间；
- Graphify 缓存构建前后；
- Codex 状态探测前后；
- provider 启动前、流式回调中和生成后；
- grounding 校验前和持久化前。

## 5. Graphify 检索索引与缓存

### 5.1 缓存

`AppState` 增加独立 `Arc<Mutex<GraphSearchCache>>`。缓存键包含：

```text
repositoryId + graph path + file length + modified timestamp
```

缓存值只保存 Graphify 派生拓扑和规范化搜索文档，不保存 Wiki 事实。文件版本变化或 repository 切换自动 miss；解析失败返回 degraded 空通道，不影响 Wiki/paper/book。

缓存锁不与 repository mutex 共用，命中时只复制 `Arc<GraphSearchIndex>`；首次解析发生在 blocking worker。

### 5.2 搜索文档

每个中心节点的文档包含：

- label/name、description/summary；
- source_file、source_location；
- community 数字和 community_name；
- 一跳边 relation；
- 一跳 neighbor label/name。

来源页 title 从 SQLite `pages` 映射时加入匹配。命中分别计为 `nodeHits`、`relationHits`、`neighborHits`，并进入 score 与 retrievalReason。即使 node label 未命中，只要 relation 或 neighbor 命中也可产生候选。

最终安全闸门保持：中心节点 source path 必须规范化到存在的 `wiki/**/*.md`，且能映射到 `pages` 的 page ID/page type；否则过滤。

## 6. 零证据生成与展示

### 6.1 生成

- Codex/API：使用专门的 no-evidence prompt，允许模型基于一般知识回答，明确禁止声称来自本库和禁止输出 `[E#]`。
- 服务器在模型正文前强制添加固定声明，声明不依赖模型遵循：

```text
当前知识库没有检索到参考来源。以下内容来自模型的一般知识，未经本库证据核验。
```

- 离线 provider：无生成模型，只返回固定声明和建议用户补充关键词/文献。
- 最终规范化阶段移除或替换零证据正文中的 `[E数字]` 形式，避免前端产生伪引用按钮；validation 记录原始违规并可测试。

### 6.2 持久化/UI

- user 与 assistant 均保存为 `status='unverified'`，确保 SQL completed-only 历史不会只吸收单边 user。
- UI 为 assistant 显示“无参考来源 · 未验证”状态，不显示证据支持标识；证据面板保持 0 条。
- `retryQuestionFor` 优先按相同 request ID/相邻交换寻找 `completed|failed|unverified` user，兼容旧消息的相邻 completed fallback。

## 7. 失败交换

`persist_failure_exchange` 接收 root、reserved session ID、原问题、request ID、错误码/消息和 provider：

1. 复用属于当前 repository 的现有 session；否则用 reserved ID 创建新 session；
2. 在单一事务内插入 failed user 与 failed assistant；
3. assistant 保存脱敏 errorCode/errorMessage；
4. 返回完整 `FailedExchange` 给事件层；
5. conversation history 继续只读取 completed，因此失败交换不会污染下一轮。

前端收到带 exchange 的 failed 事件后，用后端消息替换 optimistic user，切换到持久化 session 并刷新会话列表；无 exchange 时才执行纯回滚。

## 8. Codex 状态缓存

`AppState` 增加 ask-time 状态缓存，保存状态与采样时间。建议 TTL 30 秒：

- `ask_luna` 命中有效缓存直接使用；过期时在 blocking worker 探测并更新；
- 设置页 `get_codex_subscription_status` 视为显式刷新，绕过 TTL 并更新缓存；
- offline/API 路径不探测 Codex；
- 探测前后检查取消。

## 9. 兼容、迁移与回滚

- `chat_messages.status` 为 TEXT，无需破坏性迁移即可增加 `unverified`。
- `citation_validation` JSON 新字段使用默认值兼容旧记录；如 schema version 需要提升，只做可逆的非破坏迁移。
- 旧 completed/failed 消息继续可读；重试逻辑保留 legacy fallback。
- Graphify 缓存可整体清空，真相仍是 Wiki/SQLite 与当前 graph 文件。
- 若 query rewrite 误召回，可关闭指代触发器而不影响单轮检索。
- 若状态缓存异常，可回退到按问探测，不影响 provider/凭据边界。
- 版本按 `0.12.3` 同步 package/Cargo/Tauri、更新 fixture 和配置校验；生成产物不提交 Git。

## 10. 不变量

- completed event 与 invoke result 仍以 request ID 幂等合并。
- repository ID 在准备、生成后和写入前核验；旧库结果不写入新库。
- 只有本轮 evidence 能形成 `[E#]`；历史引用不可复用。
- Graphify 只作关系提示且必须回链 Wiki。
- 论文证据保留 sourceLocation，书籍证据保留 physical pages。
- provider 失败不转换为离线完成；API Key、Codex token/cookie 不进入 SQLite、日志或事件。
- 不修改 Raw、Wiki、正式词表和 B 类页面。
