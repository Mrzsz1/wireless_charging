# 智能问答准确率与上下文整改实施报告

## 已实现

1. **检索相关性**
   - Wiki source 下钻论文时，在目标论文内部按当前 query 做 FTS/BM25 section 选择。
   - generic Abstract/Problem/Model/Introduction 标记为 `wiki_source_to_primary_fallback`，只承担回源导航，不计入 query-matched primary contract。
   - Gold 排序继续执行 Recall@5/10/20、MRR、nDCG@10、证据类型覆盖和 Wiki/paper 配对覆盖阈值。

2. **Token-aware 上下文**
   - 新增 `qa/context.rs`、`ContextPlan` 和 `ContextBudget`。
   - 历史只接收 completed 状态，按 request ID 组成完整 user/assistant exchange；孤儿消息、failed、cancelled、unverified 排除。
   - 最近 1–8 个完整轮次原文保留；更旧轮次压缩为确定性的用户问题/约束 memory，并清除旧 `[E#]`。
   - 记录 context window、输入预算、研究契约、memory、近期历史、问题、证据、序列化开销、输出预留、安全余量和 free tokens。
   - `其中`、`分别` 不再单独触发指代扩写；当前问题显式命名两个实体时不会混入历史实体。

3. **统一 Prompt Envelope**
   - Codex 与 compatible API 共用 `research_contract/session_memory/recent_exchanges/current_query/evidence_bundle/answer_contract` 六层结构。
   - 历史、问题和证据以 JSON data 进入 prompt，`< > &` 进行转义，不能闭合或覆盖控制层。
   - 三类意图分别要求 solve 要素、relationship 比较轴、novelty 覆盖矩阵与当前库边界。
   - 远程有证据回答要求六个固定二级标题和最低事实信息量；不完整回答以 `ANSWER_COMPLETENESS_FAILED` 失败闭合。

4. **运行可复现性**
   - assistant message 新增并持久化 `run_manifest`；迁移通过幂等字段检测完成，不覆盖共享数据库的全局 `PRAGMA user_version`。
- Manifest 包含 prompt/answer/retriever/context schema 版本、provider、requested/resolved model、temperature、token limits、prompt hash、index snapshot、近期/压缩/指代解析 history IDs、evidence checksums、context budget、引用修复和完整性结果。
   - Codex 未上报默认模型时记录 `provider-default-unreported`，不虚构模型名；compatible API 从 SSE `model` 字段记录 resolved model。
   - full history 和 paginated history 均可还原 manifest；旧消息兼容为 `None`。

5. **Grounding 与受限修复**
   - 引用校验继续是结构覆盖，不声称语义蕴含，`entailmentChecked=false`。
   - 只有当同一 claim 已含有效非图谱引用时，才允许删除其中未知 `[E#]`；不会添加引用、补事实或修复无引用 claim。
   - 引用扫描与前端投影统一排除 fenced/inline code、数学区、转义文本和既有 Markdown 链接，代码字面量不会伪装成证据。

6. **终审补强**
   - 生成后校验失败的回答也持久化证据、引用校验和 `run_manifest`，保留 prompt hash、证据 checksum、修复及完整性失败信息。
   - 最近完整轮次严格按时间优先；最新轮次过大时不再跳过它而混入更旧短轮次，并由总输入预算门禁失败闭合。
   - Manifest 新增压缩记忆来源消息 ID；Gold 排序与 Wiki/paper 配对指标排除 generic paper fallback。
   - Held-out 评测强制双人独立盲审、分歧第三人裁决、完整 claim 覆盖，以及 evidence canonical SHA-256/stable-source 复算。

7. **科研审计 UI**
   - 证据侧栏显示 context token 分解、近期/压缩轮数、snapshot、prompt/answer schema 和完整性状态。
   - assistant message 支持复制 JSON 审计包：问题、回答、证据、manifest。
   - `offline-evidence` 统一显示为“证据浏览模式”，避免被误读为完整生成回答。

8. **评测边界**
   - `gold_questions.json` 明确为 `development_regression/development`，Wiki evaluator 输出 `CONTRACT PASS`，不再称为事实准确率。
   - 新增 `heldout_questions.json` 和 `qa_accuracy_eval.py`：仅在独立冻结题集和人工逐 claim 评审存在后报告 factual precision、Wilson 95% 区间、claim support rate、not-verifiable rate、引用 ID 精度与结构完整率。
   - 当前 held-out 状态为 `awaiting_independent_curation`，因此不会生成虚假的生产准确率数字。

## 验证结果

- Rust full tests：90 passed。
- Clippy：`--all-targets --all-features -- -D warnings` passed。
- TypeScript + Vite production build：passed。
- Node suites：49 passed（P1/P2、research trail、ingest、pagination、library、settings、QA provider、E2E config、process lifecycle）。
- Wiki development/regression contract：10/10 `CONTRACT PASS`。
- Held-out runner：pending 状态符合预期。
- Python tests：60 passed（含 held-out 审计回归 11 项）。
- Trellis context validate：passed。
- `git diff --check`：passed。
- 独立终审复核：原 6 项补充发现全部闭合，无阻塞问题。

## 当前科研结论边界

本次实现把“回归契约、引用结构、上下文预算、回答结构、运行复现”变成可执行门禁。生产事实准确率需要独立冻结的 held-out 题集与人工逐 claim 复核后才能得到；自动语义蕴含仍按任务范围保持关闭。
