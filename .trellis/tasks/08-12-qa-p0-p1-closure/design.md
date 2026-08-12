# 技术设计

## 请求身份与仓库隔离

`AskRequest` 新增 `repositoryId`。前端从当前知识库路径派生稳定 generation token 并随请求发送；后端以当前 `RepositoryState.root` 计算 authoritative repository ID。请求在三处校验：准备上下文前、远程生成后、写入事务前。仓库切换时前端先调用现有取消命令，再重置 completion ledger、消息、证据和流式状态；事件处理闭包同时核验 generation。

## 多轮上下文

后端在确定/创建 session 后，从 `chat_messages` 读取当前问题之前最近 8 条 `completed` user/assistant 消息，总字符预算 12,000。历史形成独立 `ConversationTurn` DTO，仅用于 Prompt；当前 user 问题不重复。系统 Prompt 明确历史不属于证据，历史中的 `[E#]` 不得沿用，当前回答只能引用本轮 evidence。

## 引用校验

新增 `CitationValidation`：`citedIds`、`unknownIds`、`citationPrecision`、`hasCitations`、`supported`。验证器解析回答中的 `[E数字]`：

- unknown 非空 → 失败；
- 当前 evidence 非空且远程回答没有任何有效引用 → 失败；
- 离线模板由确定性生成器保证引用；
- 成功结果和 assistant message 携带验证摘要，便于 UI/后续评测。

本阶段不做自然语言 claim-level NLI；“事实覆盖率”采用可执行的最低契约：非空证据回答必须至少一个有效引用，所有出现的引用必须已登记。

## 意图感知检索

先计算 intent，再取候选。每个 candidate 增加 intent bonus；排序与配额按意图执行：

- `solve`：method/source/paper/book 优先，保证方法或原文证据配额；
- `novelty`：source/paper/synthesis 优先，强调年份、重叠与库水位，Graphify 只补关系；
- `relationship`：Wiki + 可回链 Graphify 优先，使用一跳边/邻居/社区信息；
- general：保持当前均衡行为。

去重和 hard tier 规则保持，Graphify 不能替代 primary evidence。

## Graphify 可引用性

解析 `graph.json` 的 nodes 和 links/edges。节点候选必须能从 `source`/`file`/`path`/location 映射到仓库内 `wiki/**/*.md` 或 canonical Markdown；Wiki 路径规范化为相对路径并去掉 `wiki/` 前缀与 `.md`，作为可打开 page ID。命中节点的一跳邻居标题、关系类型和 community 写入 retrieval reason；无法安全映射来源的节点直接过滤。

## Provider 与状态语义

`get_qa_settings` 不再探测 Codex，只读取保存设置。`ask_luna` 先读取 provider：仅 Codex provider 调用一次 `get_status`，并把 ready 传入设置/执行分支。API/Codex 失败直接返回结构化 failed，不调用 `offline_answer`；只有 provider 明确为 `offline-evidence` 才生成 completed 离线回答。

失败时后端在已创建 session 的情况下持久化 assistant `failed` 消息（脱敏错误码/用户消息），但不持久化未成功的 user 消息；前端回滚乐观 user 消息并重新加载会话，避免双写和幽灵状态。

## 回滚

- DB 迁移仅新增字段，旧客户端可忽略。
- 若引用门过严，可仅回滚“无引用失败”条件，保留 unknown 引用拒绝与结构化摘要。
- 若 Graphify schema 存在变体，解析器按多字段兼容，无法映射时降级为不返回 graph evidence，不影响 Wiki/论文/书籍召回。
