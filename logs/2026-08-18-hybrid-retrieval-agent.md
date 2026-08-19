# 混合检索、重排与有限 Agentic 补查

## 目标

将智能问答检索从固定 answer profile、前缀词截断和候选数量阈值，迁移为面向 Markdown 科研语料的开放检索契约、来源解析、独立混合通道、RRF/reranker 与软覆盖控制。

## 已实现

- 新增 `qa-retrieval-contract-v2`。Codex 通过原生 JSON Schema 生成 scope、显式来源、概念、别名、相关问题、facets、请求来源类型和最多三轮预算；Schema 拒绝 `answerProfile`、`minimumEvidence` 与未知字段。
- Planner 失败时保留完整 Unicode 问题、书名号中的显式来源和开放 `wiki/paper/book` 范围，不再从问题首部截取固定 n-gram。
- 通过 `documents_v2` 与 `document_aliases_v2` 解析书籍/论文身份。显式来源把 document ID 下推到 ContentBlock FTS 和 dense 过滤；未解析来源记录 gap，其他文档不能冒充完成。
- title/alias、ContentBlock FTS、metadata-filtered FTS、dense 和 graph-mapped ContentBlock 独立运行，记录 round、status、耗时、候选数、脱敏错误类型和 SHA-256 round fingerprint。
- 使用 stable block key 的 Reciprocal Rank Fusion，随后执行可替换 reranker；保护显式来源与正文命中，降低 reference-only、graph-only、fallback 和后续弱轮次。
- Coverage 只决定是否继续检索，依据 requested/must-attempt kinds、显式来源 gap、required facets、新增候选和共享查询预算；不输出事实“充分/不充分”。Provider facet 查询仅在需要补查时释放，最多首轮加两轮。
- Run manifest 升级到 `qa-run-v4`，记录 retriever/reranker 版本、round/channel/gap/stop 状态，不保存原问题、原始查询、路径、snippet 或 Provider payload。

## 迁移与回滚

- 生产默认启用 `hybrid-agentic-rrf-v6`。
- `LUNAWIKI_RAG_RETRIEVER_V2=false`（亦支持 `0/off/no`）立即切回 legacy retriever。
- open scope 在最终评测前把 legacy 与 v2 分别视为独立排名通道，再用 stable-ID RRF 融合；显式来源已解析时只使用 v2 文档过滤，防止跨文档泄漏。
- 旧 `query_plan.rs` 暂作兼容 facade；最终评测子任务确认指标后再删除 legacy 逻辑。
- 每轮首批证据的增量 UI 事件由 `08-18-answer-evidence-links` 与自然回答/证据界面一起接入；本子任务已提供 round/channel/status 类型契约。

## 验证

- Rust 全量：160 passed，1 ignored（本地语义模型部署探针需显式 cache 环境；此前已在已部署模型上通过）。
- 前端 QA 契约：8/8。
- TypeScript/Vite production build：通过。
- 真实仓库回归：指定《近似算法》仅返回目标书 ContentBlock；开放“文献或者哪本书”分别记录 paper/book attempted；目标书召回 Euclidean TSP。
- Rust release build：通过。

## 边界

- 本任务不改变最终自然回答、后端证据附录或 Markdown 深链渲染，它们属于下一子任务。
- Compatible API 尚未声明原生 JSON Schema 能力，继续使用既有 prompt contract；Codex planner 已使用 `--output-schema`。
- 不修改 `raw/` 正文、正式词表或用户未跟踪的自动检索结果。
