# P3 Luna 智能问答实施计划

> 目标版本：`0.5.0`  
> 阶段：P3  
> 前置：P0/P1/P2 已提供仓库选择、SQLite FTS5、只读 Wiki 阅读、两本核心书籍检索、Graphify 局部图谱和对比工作台。  
> 权威边界：`wiki/**/*.md` 是正文真相；SQLite 与 `graphify-out/` 只作为可重建检索派生物；默认不外搜。

## 1. 阶段目标

交付一个可审计的自然语言问答闭环：

```text
用户问题
  → 问题规范化与意图分类
  → Wiki / 核心书籍 / Graphify 多路召回
  → 去重、来源分层、可解释重排
  → 带证据编号的 Luna 上下文
  → 流式回答
  → 引用面板、原文跳转、会话历史
```

回答必须区分五类内容：

1. 库内直接证据；
2. 相似系统模型或问题设定；
3. 可迁移算法与使用前提；
4. 两本核心书籍中的理论基础；
5. 当前知识库尚未覆盖的部分。

每个回答都必须展示库水位，并且所有事实性结论能回到 Wiki 页面或核心书籍章节与物理页码。

## 2. 本阶段不做

- 不执行默认外网文献检索；
- 不自动写入 `wiki/`、`raw/`、`schema/`；
- 不生成或修改 problem/idea 页面；
- 不把 Graphify 派生关系当作正文事实；
- 不在客户端保存明文 API Key；
- 不提前接入 P4 的自动发现、下载、A 编译与 Lint 控制台。

## 3. 设计原则

### 3.1 离线优先

- 仓库搜索、核心书籍搜索、Graphify 查询和历史会话离线可用；
- Luna 未配置或请求失败时仍返回“证据包”，用户可以阅读、复制并逐项打开；
- 只有最终模型生成需要联网。

### 3.2 证据先于生成

- 模型只接收已经编号的证据；
- 回答引用格式固定为 `[E1]`、`[E2]`；
- 前端只把后端返回且已登记的证据编号渲染为可点击引用；
- 未引用内容不伪装成库内事实。

### 3.3 可重建与可审计

- 会话历史和用户消息属于客户端状态，可持久化；
- 检索证据保存当时的标题、路径、页码、分数和摘要快照；
- Wiki/书籍正文不复制进会话数据库，仅保存有限长度的证据片段；
- 日志只记录请求 ID、阶段、耗时、数量和错误类型，不记录 API Key。

## 4. 数据契约

### 4.1 会话

`ChatSession`

- `id`：稳定 UUID；
- `title`：首个问题自动截断生成，可由用户重命名；
- `created_at` / `updated_at`；
- `message_count`；
- `last_message_preview`。

### 4.2 消息

`ChatMessage`

- `id`、`session_id`；
- `role`：`user | assistant | system`；
- `content`；
- `status`：`pending | retrieving | generating | completed | failed | cancelled`；
- `created_at`；
- `error_code` / `error_message`；
- `waterline` 快照；
- `provider` / `model`；
- `request_id`。

### 4.3 证据

`EvidenceItem`

- `id`：回答内编号，例如 `E1`；
- `kind`：`wiki | book | graph`；
- `tier`：`direct | similar_model | transferable_method | theory | graph_hint`；
- `title`、`snippet`、`score`、`rank`；
- Wiki：`page_id`、`page_type`、`source_path`、`wikilink`；
- 书籍：`book_id`、`chapter_id`、`physical_page_start/end`、`markdown_path`、`pdf_path`；
- 图谱：`node_id`、`source_file`、`source_location`、`relation`；
- `retrieval_reason`：命中词、来源通道和排序理由。

### 4.4 库水位

`WaterlineSnapshot`

- `source_count`、`method_count`、`synthesis_count`、`chapter_count`；
- `year_min`、`year_max`；
- `last_ingest_at`；
- `repository_path`；
- `captured_at`。

### 4.5 流式事件

`AnswerStreamEvent`

- `started`：请求和会话已建立；
- `retrieval_started`；
- `retrieval_completed`：返回证据包与库水位；
- `token`：回答增量文本；
- `completed`：最终消息与引用统计；
- `failed`：结构化错误；
- `cancelled`。

## 5. SQLite migration

引入显式 `PRAGMA user_version`，P3 migration 新增：

1. `chat_sessions`；
2. `chat_messages`；
3. `chat_evidence`；
4. `app_settings`：只保存 endpoint、model、timeout 等非秘密配置；
5. 必要索引：session 更新时间、message session/time、evidence message/rank；
6. `repository_id` 或仓库路径哈希，用于隔离不同知识库的会话和缓存。

全量重建知识索引时不得删除会话表；切换仓库后只展示当前仓库会话。

## 6. 检索编排

### 6.1 Wiki 通道

- FTS5 查询标题、正文和 keywords；
- page type 加权：`source`、`method`、`synthesis` 优先；
- 召回上限默认 20；
- 对过短、重复或同一页面片段去重。

### 6.2 核心书籍通道

- 使用已建立的 `book_chapters_fts`，不在每次问答时重新扫描 61 个文件；
- 默认两书同时检索，召回上限 10；
- 必须返回书名、章节、physical pages 和章节路径；
- 保持现有 Recall@5 ≥ 95% 门禁。

### 6.3 Graphify 通道

- 只作为关系收敛和候选扩展；
- 缺失 `graph.json` 时跳过，不阻断问答；
- 图节点必须携带 `source_file/source_location` 才能进入最终证据包；
- `graph_hint` 不单独支撑事实性结论。

### 6.4 去重与重排

初版采用可解释确定性重排：

```text
final_score = lexical_score
            + title_bonus
            + page_type_bonus
            + exact_term_bonus
            + cross_channel_bonus
            - duplicate_penalty
```

按来源分层保底：至少保留 Wiki 证据；书籍命中时至少保留一个章节；Graphify 只占少量配额。P3 不依赖不可审计的远程 reranker 才能工作。

## 7. Luna 适配层

### 7.1 配置

- `endpoint`：OpenAI-compatible Chat Completions 地址；
- `model`：默认显示 `gpt-5.6-luna`，允许配置；
- `api_key_env`：默认 `LUNA_API_KEY`；
- `timeout_seconds`、`max_output_tokens`、`temperature`；
- 客户端只显示“已配置/未配置”和掩码，不读取完整密钥；
- 密钥只从进程环境读取，不写 SQLite、日志或前端状态。

### 7.2 请求

- system prompt 固定知识边界、引用格式和库水位要求；
- user payload 包含原问题、会话最近消息和编号证据；
- 默认 `temperature = 0.1`；
- 请求带唯一 `request_id`；
- 支持超时、一次可控重试、取消；
- HTTP 错误只记录状态码和 request ID，不记录授权头。

### 7.3 降级

Luna 未配置、离线或返回异常时：

- 保存用户消息与检索证据；
- 生成确定性的证据摘要页，不冒充 Luna 回答；
- 提供“配置 Luna”“重试生成”“复制证据包”。

## 8. Rust/Tauri 命令

计划新增：

- `get_luna_settings`；
- `save_luna_settings`；
- `list_chat_sessions`；
- `get_chat_session`；
- `create_chat_session`；
- `rename_chat_session`；
- `delete_chat_session`；
- `prepare_question`：只做检索并返回证据包；
- `ask_luna`：检索、生成、持久化和流式事件；
- `cancel_answer`；
- `retry_answer`。

命令错误统一包含 `code/message/retryable/request_id`，避免前端解析任意字符串。

## 9. 前端交互

### 9.1 智能问答页

- 左列：会话历史、新建、重命名、删除；
- 中列：消息流、问题输入、停止、重试、复制；
- 右列：本轮证据、来源筛选、库水位、检索耗时；
- 首屏提供针对当前库的示例问题；
- 相同会话使用唯一 Tab，不重复打开。

### 9.2 引用交互

- `[E#]` 显示为可点击引用；
- Wiki 证据调用现有 `openPage`；
- 书籍证据打开核心书籍对应章节并显示 PDF 页码；
- Graphify 证据打开对应节点或 Wiki 来源；
- 引用面板显示原文片段、来源类型、排序理由和定位信息。

### 9.3 状态

- `retrieving`：展示三路召回进度；
- `generating`：流式光标与停止按钮；
- `failed`：显示可诊断原因及重试入口；
- `offline-evidence`：明确为证据包，不显示为模型回答；
- 空仓库、未建索引和 Luna 未配置均有独立状态。

## 10. 实施步骤

### Step 1：基线冻结

1. 运行 `npm run build`、`npm run verify`、`cargo test`、核心书籍评测；
2. 记录 P2 release 构建和安装冒烟状态；
3. 修复会阻断 P3 的现有构建问题；
4. 保存基线输出。

### Step 2：数据契约与 migration

1. 增加 Rust/TypeScript 对应类型；
2. 引入 `user_version`；
3. 创建会话、消息、证据、设置表；
4. 增加 migration、仓库隔离和“重建索引不删历史”测试。

### Step 3：离线证据包

1. 实现 Wiki FTS、书籍 FTS 和 Graphify 候选转换；
2. 实现去重、分层和确定性重排；
3. 实现库水位读取；
4. 暴露 `prepare_question`；
5. 固定问题集验证证据召回与引用锚点。

### Step 4：会话历史

1. 实现 CRUD 命令；
2. 问题、回答、失败状态和证据入库；
3. 实现仓库切换隔离；
4. 实现会话列表分页与更新时间排序。

### Step 5：Luna 客户端

1. 实现配置读写和环境密钥检查；
2. 实现请求构建、SSE/流式解析、超时和取消；
3. 实现引用完整性校验；
4. 实现离线证据降级；
5. 使用本地 fixture server 测试，不依赖真实密钥。

### Step 6：智能问答 UI

1. 接通导航、Tab 和 `AskView`；
2. 实现会话列、消息流和输入框；
3. 实现检索进度与流式回答；
4. 实现动态引用面板和证据跳转；
5. 实现停止、重试、复制和错误状态；
6. 保存最近会话及界面状态。

### Step 7：质量门禁

1. Rust：migration、检索、排序、会话、取消、错误脱敏、路径边界；
2. 前端：导航、消息状态机、引用解析、证据点击、非 Tauri fallback；
3. Fixture：正常流、断流、超时、401、429、500、无引用回答；
4. 评测：Wiki 10 题、核心书籍 Recall@5、证据覆盖率和引用锚点率；
5. 性能：本库规模下首批证据目标 1 秒内，UI 不阻塞。

### Step 8：集成和交付

1. `npm run build`；
2. `npm run verify`；
3. `cargo test`；
4. `py -3 tools/wiki_eval.py --answers-dir evals/answers`；
5. `py -3 tools/core_book_eval.py`；
6. Tauri release 构建；
7. 安装后启动、提问、引用跳转、重启后历史恢复冒烟；
8. 更新版本、README、PRD 状态、日志和 Graphify。

## 11. 验收标准

1. 10 条 Wiki 固定问题均返回库水位和至少一个有效 Wiki 引用；
2. 两本书 Recall@5 分别保持 ≥ 95%，书籍证据锚点率为 100%；
3. 每个 `[E#]` 都能在右侧面板找到且可定位来源；
4. 回答未引用的证据编号不会被渲染成有效引用；
5. Luna 未配置或断网时仍能完成检索、阅读、证据复制和历史保存；
6. 切换仓库不会混用会话或证据；
7. 重建知识索引不会删除会话历史；
8. API Key 不出现在 SQLite、日志、错误信息或前端持久状态；
9. 路径越界、损坏图文件、损坏 frontmatter、模型错误均不产生白屏；
10. `npm run build`、`npm run verify`、`cargo test`、P3 评测和 Tauri release 构建全部通过。

## 12. 子代理核验分工记录

- 前端探子：核验导航、Tab、证据面板、Tauri service 和 P2 遗留；
- 后端探子：核验 SQLite、FTS5、书籍、Graphify、命令注册、路径安全与测试；
- 质量探子：核验 Query 模板、固定题集、核心书籍 95% Recall 门禁、联网边界与构建脚本。

子代理结果只作为定位线索；具体代码修改、方案取舍和最终验收由主代理完成。
