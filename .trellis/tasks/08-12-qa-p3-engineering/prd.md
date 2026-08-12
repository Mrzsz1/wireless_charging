# 智能问答 P3 工程优化

## Goal

在不改变现有 P1/P2 可信度与持久化语义的前提下，完成 Graphify 检索扩展性、会话分页/搜索、QA 模块边界、排序评测和隐私安全可观测性优化。语义蕴含自动核验明确不在本任务范围内。

## Background

P1/P2 修复已使智能问答在当前小库上正确工作，但复审确认仍有五类工程扩展项：Graphify 每问全图扫描并逐节点查库、会话只加载最近 100 条且详情全量加载、`qa.rs` 约 3700 行职责集中、排序只有 10 题布尔召回回归、运行时缺少不含正文/密钥的通道指标。

## Requirements

1. Graphify 缓存必须预计算规范化检索字段和倒排 token index；页面映射必须批量读取，禁止逐节点 SQLite 查询。
2. Graphify 候选扫描必须支持通道内部取消检查；缺失或损坏 graph 继续降级为空通道。
3. 新增 cursor-based 会话列表分页和后端 query 搜索；搜索不得只过滤前端已加载的 100 条。
4. 新增消息 cursor 分页，默认加载最近一页；证据按页面消息 ID 批量读取，消除逐消息 N+1 查询。
5. 前端支持会话搜索、加载更多会话和加载更早消息；切库、取消、重试和消息合并语义不回归。
6. 将 Graphify、会话存储和引用校验从 `qa.rs` 拆到职责明确的子模块，同时保持 Tauri command 和已有 DTO 向后兼容。
7. 检索结果生成不包含问题正文/凭据的诊断指标：总耗时、各通道耗时、候选数、最终证据数和取消状态；前端可查看本轮诊断摘要。
8. Gold retrieval 增加排序质量指标，至少报告 Recall@K、MRR、NDCG@K、required-kind coverage 和 Wiki/paper pair coverage，并设置可执行阈值。
9. 不引入在线 embedding/reranker，不发起真实 provider 请求，不修改 Raw/Wiki/Graphify 正文。
10. 保持 `entailmentChecked=false`；本任务不实现 claim—evidence 语义蕴含。
11. 将固定 10 题明确标记为 development/regression，不再把最低检索命中表述为最终答案准确率；新增生产契约 fixture runner 与独立 held-out 数据入口。
12. Wiki source 下钻论文时必须优先选择与当前 query 相关的 section；generic section 仅作为显式 fallback，评测必须区分两者。
13. 每轮回答必须生成并持久化 `QaRunManifest`，记录 prompt/retriever/context/schema 版本、provider/model、非敏感采样参数、索引快照、历史消息 ID、证据 checksum 和 prompt hash。
14. 历史上下文必须按完整 exchange 与 token 预算规划；旧轮仅生成不含旧 `[E#]` 的确定性用户约束摘要，零证据/失败轮继续排除。
15. Codex 与 compatible API 必须共享同一 `PromptEnvelope`、信任边界和意图化 answer schema；证据与历史中的命令式文本一律视为引用数据。
16. 生产 provider 的有证据回答必须通过意图化完整性检查；只允许执行不新增事实的受限引用修复。
17. 前端必须显示本轮 context token 分解、索引快照和回答 schema 状态，并支持复制不含凭据的科研审计包。

## Acceptance Criteria

- [x] Graphify 查询使用倒排候选集和一次 pages 批量映射；单元测试证明结果等价、缓存失效和内部取消。
- [x] 会话列表接口返回 `{items,nextCursor}`，支持稳定 cursor 和后端 query；超过首屏的数据仍可搜索。
- [x] 消息接口返回分页结果并通过单次批量证据查询组装页面；前端可加载更早消息。
- [x] 旧 `list_chat_sessions/get_chat_session` 命令继续可用，新增分页命令由 AskView 使用。
- [x] `qa.rs` 至少拆出 graph、session、grounding 三个模块，跨模块类型和可见性最小化。
- [x] `QuestionContext/AskResult/retrieval_completed` 暴露无敏感内容的 retrieval diagnostics，UI 显示总耗时和通道候选数。
- [x] Gold 10 题继续 10/10，并新增 Recall@K、MRR、NDCG@K、类型覆盖和配对覆盖断言/报告。
- [x] Rust fmt、Clippy `-D warnings`、完整 Rust tests、相关 Node tests、前端 build、Wiki eval、Trellis validate 和 `git diff --check` 全部通过。
- [x] 生产 Prompt/上下文/manifest 数据流在 Rust、SQLite、Tauri DTO、TypeScript 和 UI 间完整往返。
- [x] query-relevant linked-paper、完整 exchange、上下文污染、Prompt provider 一致性、manifest migration/round-trip、answer completeness 和审计包均有回归测试。
- [x] development 与 held-out/人工评审入口有明确分层，自动报告不会把结构通过率命名为事实准确率。

## Out of Scope

- 自动语义蕴含、事实真实性或 contradiction 模型。
- 在线 embedding、cross-encoder 或付费 reranker。
- Raw/Wiki/Graphify 内容编辑。
- 本轮提交与发布；仍遵循 Trellis Phase 3.4 的单次提交确认。
