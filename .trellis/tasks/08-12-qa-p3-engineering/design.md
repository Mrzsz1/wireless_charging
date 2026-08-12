# 技术设计

## 1. 模块边界

保留 `qa.rs` 作为公共 facade 和请求编排层，新增：

- `qa/graph.rs`：Graphify 解析、缓存、倒排索引、批量页面映射和可取消检索；
- `qa/session.rs`：会话 CRUD、cursor 编解码、列表/消息分页和批量证据组装；
- `qa/grounding.rs`：claim 分段、引用抽取、结构覆盖校验和零证据规范化；
- `qa/metrics.rs`：无敏感内容的检索计时、候选计数和排序评测纯函数。

子模块使用 `pub(super)` 暴露给 facade；对 `lib.rs` 仍由 `qa::*` re-export 稳定公共入口。

## 2. Graphify 索引

`GraphSearchIndex` 在 graph JSON 解析时持有预规范化的 node/relation/neighbor haystack，并建立 `HashMap<String, Vec<usize>>` token index。token 包含英文/数字词、完整短语以及中文 3–4 gram。

查询先按 query term 的 index keys 取节点并集，再用原有 substring 规则计算 node/relation/neighbor hits，保证排序语义不因倒排索引改变。若 term 无索引 key 命中则允许受控全扫回退，防止特殊符号查询失召回。

`pages` 表每问一次性读取为 `source_path -> (page_id,page_type,title)`，并仅为映射表条目检查文件存在。Graph loop 不执行 SQL。每 64 个候选检查一次 `AtomicBool`。

## 3. Cursor 分页

Cursor 使用 URL-safe base64-free 的 JSON-hex 或稳定分隔编码，内容为排序键，不包含正文：

- session cursor：`updated_at + id`；
- message cursor：`created_at + rowid`。

新增 DTO：

```text
ChatSessionPage { items, nextCursor }
ChatMessagePage { session, messages, nextCursor }
```

列表 query 在后端匹配 session title 或所属消息 content。消息首页按倒序取最近 N 条后恢复时间正序；后续通过 `before` cursor 加载更早消息。证据使用动态 IN 参数一次查询并按 message_id 分组。

旧接口作为兼容适配器继续返回原始数组/完整 detail。

## 4. 前端状态

AskView 保存 `sessionCursor/sessionHasMore/sessionLoadingMore` 和 `messageCursor/messageHasMore/messageLoadingMore`。搜索词 debounce 后从后端重新取第一页；“加载更多”追加且按 ID 去重。打开会话默认取最近消息页，“加载更早消息”前插并保持现有 evidence/waterline 选择。

## 5. 检索诊断

`RetrievalDiagnostics` 仅含：

- `totalMs`；
- `channels[] = {name,durationMs,candidateCount}`；
- `selectedCount`；
- `cancelCheckCount`。

不得包含 question、query terms、snippet、路径、API key 或 provider token。诊断随 `QuestionContext` 和完成结果返回，不单独写日志正文。

## 6. 排序评测

对每个 Gold case 记录期望 Wiki/source 的 rank，计算：

- Recall@5/10/20；
- MRR；
- binary-relevance NDCG@10；
- required kind coverage；
- selected paper 是否有对应 Wiki source 的 pair coverage。

测试输出聚合摘要并设置不低于当前基线的阈值。评测纯函数独立于数据库，便于扩展更多人工标注案例。

## 7. 兼容与回滚

- 分页本身基于现有索引和 rowid；后续准确率整改仅新增幂等 `run_manifest` 字段，并保持旧消息可读。
- 新 DTO 字段使用 serde default，旧持久化消息不受影响。
- 旧 Tauri commands 保留；前端仅切换到新增分页 commands。
- Graphify 倒排索引失败时降级为空/全扫，不影响 Wiki/paper/book。
- `entailmentChecked` 保持 false。

## 8. 科研上下文与 Prompt Envelope

新增 `qa/context.rs`，负责：

- 保守 token 估算；
- completed 消息按 request ID 组装完整 exchange；
- 最近完整 exchange 原文保留；
- 更旧用户问题/约束的确定性 extractive memory；
- 旧 `[E#]` 清理、history fingerprint 与 context budget；
- provider-neutral `PromptEnvelope`。

Prompt 使用 `research_contract/session_memory/recent_exchanges/current_query/evidence_bundle/answer_contract` 六层结构。历史和证据正文以 JSON quoted data 序列化，不能覆盖 research contract。

## 9. 可复现 Manifest

聊天 schema 升级并为 assistant message 保存 `run_manifest`。`QaRunManifest` 包含：

- prompt、answer schema、retriever、context schema 版本；
- provider、requested/resolved model、temperature、max output tokens；
- prompt SHA-256、index snapshot ID；
- recent/resolved history message IDs；
- evidence stable source ID 与 SHA-256；
- context token breakdown、受限修复记录和 answer completeness。

旧消息的 manifest 为 `None`。失败消息保持原失败语义；凭据、endpoint、问题全文和 provider payload 不进入 manifest。

## 10. Paper section 相关性

`linked_paper_candidates` 接收当前 query terms。对每个 Wiki source，先在该 page ID 的 `paper_sections_fts` 内执行 query-constrained BM25，只有无命中时才使用 Abstract/Problem/Model/Introduction fallback。候选 relation/retrieval reason 必须显式区分 query match 与 fallback。

## 11. Answer completeness 与受限修复

远程 provider 的有证据回答按意图检查必需 Markdown section 和最少信息 claim 数。结构不完整时 fail closed。受限修复仅删除“同一 claim 已有有效非图谱证据时”的未知 `[E#]` token；不猜测证据、不补事实、不修复无引用 claim。

## 12. 前端可见性

`retrieval_completed`、`AskResult` 和历史 assistant message 暴露无正文的 context budget / run manifest。证据栏显示输入预算、历史、memory、evidence、输出预留、free tokens、压缩轮次和 index snapshot。消息操作提供“复制审计包”，内容为问题、答案、证据和 manifest，不含凭据。
