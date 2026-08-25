# 根据 GitHub 审查修复智能问答

## Goal

把当前 Research QA 从“模块齐全但部分代理实现会产生错误安全感”修复为能力与命名一致、默认 fail-closed、预算可执行、Provider 能力对称的实现；优先解决审查列出的 4 个 P0，再处理可独立验证的 P1，不新增无关 Agent 或 Retriever。

## Background

审查基于 GitHub `master`，指出当前实现已具备主要研究问答模块，但以下能力仍不满足生产门禁：

- `natural_answer::render` 在 verifier 前删除 `[E#]`，随后 appendix 存在即标记 supported。
- 无显式映射的 factual claim 会自动绑定全部 Evidence。
- `ClaimStatus` 混合 claim 类型与验证结果；lexical cue 可绕过证据检查；heuristic 被记作 entailment。
- `SemanticResearchReranker` 实际是 embedding cosine rescore，并把未校准 bonus 直接叠加 RRF。
- Adaptive Routing 记录 policy，但 planner 与 call/token budget 没有被完整执行。
- Planner/Understanding 仅按 provider 名称为 Codex 启用，Compatible API 能力不对称。
- Problem Matcher 的候选方法直接进入检索词，形成 confirmation bias。
- 索引已有 `parent_block_id`，但 Evidence Manager 未消费它，而是在同文档中选择最长块。
- 多个 ResearchIntent 在生成端折叠为 `solve`。
- 缺失 `groundingStatus` 时默认 `supported`，不符合 fail-closed。

生产 held-out 仍须独立研究者冻结和双人逐 claim 复核；本任务不得弱化该门禁或宣称生产准确率已达标。

## Requirements

### R1 — Claim → Evidence 契约重做（P0）

- Generator 的 claim-to-Evidence 映射必须保留到 verifier 完成；显示层可以隐藏 `[E#]`，验证层不得提前删除。
- factual claim 没有显式 Evidence ID 时必须为 `not_verifiable`，不得自动绑定全部 Evidence。
- 将 `ClaimType` 与 `VerificationStatus` 分为两个独立字段：
  - ClaimType：knowledge_fact / general_knowledge / reasoned_inference / research_suggestion。
  - VerificationStatus：supported / partially_supported / contradicted / not_verifiable / not_applicable。
- General knowledge、inference、suggestion 的关键词仅决定 ClaimType，不能先于事实核验而产生“已支持”结论。
- deterministic lexical 检查只能记录 `heuristicVerificationChecked=true`；`entailmentChecked` 在没有真正 NLI/LLM verifier 时保持 false。
- 缺失 grounding 字段默认 `unverified`。
- Run manifest / Rust DTO / TypeScript DTO / UI 文案保持语义一致，并能审计逐 claim 映射与修复。

### R2 — 真正的 RerankProvider 与分数校准（P0）

- 将现有 embedding cosine 实现重命名为 `EmbeddingRescorer`，明确为 fallback。
- 增加可替换的 `RerankProvider`；主实现使用本地 FastEmbed `TextRerank` cross-encoder，在已部署模型存在时联合编码 query/document。
- 普通问答不得自动下载模型；缺失、损坏或推理失败时回落到 EmbeddingRescorer，再回落到 deterministic。
- 不再把 RRF raw score、cosine bonus、lexical bonus直接相加；采用稳定 rank fusion 或 provider rank 作为独立排序阶段，并保留 explicit-source / graph / reference fail-closed 保护。
- 遥测必须区分 cross-encoder、embedding fallback、deterministic fallback，不能把 embedding rescore 报告为 semantic reranker 成功。
- 冻结测试不得用“知道正确答案”的 tautological embedder 作为质量证明；真实质量门禁使用独立真实检索集和明确的 model-deployment 状态。

### R3 — Adaptive Routing 与 LLM/Token Budget 强制执行（P0）

- Planner 调用必须服从 `routing_policy.planner_enabled`；DirectQA 默认最大 retrieval round=1。
- 引入统一 `LlmBudgetGuard`，Resolver、Query Planner、Reranker/Verifier（如调用 LLM）和 Answer Generator 都通过同一个预算对象记账。
- 超出 call budget 或 token ceiling 时必须执行稳定 fallback/停止，不得继续调用后只记录“超预算”。
- manifest 同时记录 budget、实际 calls、估算 input/output tokens、拒绝/降级原因。
- 兼容旧接口的 override 必须显式、可审计且不能成为默认路径。

### R4 — Provider Capability 对称（P0）

- 核心 QA 能力由 capability 决定，不由 `answer_provider == PROVIDER_CODEX` 决定。
- 定义 provider-neutral capability/PlanningProvider 边界，至少描述 structured output、understanding、planning、streaming。
- Codex 与 Compatible API 在 capability 可用时都能运行 Understanding / Query Planning；任一失败都回落 deterministic。
- Compatible API 的规划调用使用同一闭合 JSON Schema、同一验证器和同一 budget guard；密钥不进入 manifest/log/error。

### R5 — Confidence-based Understanding Escalation（P1）

- Deterministic resolver/router 输出置信度与原因。
- 明显自包含且高置信问题不升级。
- 低置信、开放问题、目标/约束/方法状态变化，即使没有固定指代词，也可升级到 PlanningProvider。
- 回归集覆盖审查中的自然 paraphrase。

### R6 — 专用 ResearchIntent Answer Profiles（P1）

- 至少新增 method_improvement、solution_search、problem_modeling、exploratory profile。
- 每个 profile 有独立 answer/completeness contract，不再全部折叠为 solve。
- SolutionSearch 必须覆盖 problem classification、candidate methods、适用原因、不兼容约束、证据边界和下一步。

### R7 — Neutral Retrieval 与 Method Hypothesis 隔离（P1）

- 初始检索只使用 problem class、objective、constraints 等中性表示。
- 规则 MethodMatcher 输出标记为 hypothesis，不直接混入初始 search terms。
- 方法候选先从知识库证据发现，再做 applicability matching；hypothesis expansion 只能作为独立后续通道并要求 corroboration。
- manifest 区分 discovered method 与 hypothesis method。

### R8 — Exact Parent Context（P1）

- 使用现有 `content_blocks_v2.parent_block_id` 从 semantic child 精确读取 parent section。
- parent 不存在、非 active、跨文档或读取失败时不扩展；禁止同文档 longest-block heuristic。
- 测试必须证明 sibling/最长错误 section 不会污染 evidence。

### R9 — Evaluation 与发布纪律

- 每个修复阶段增加正向、反向、fallback、telemetry 测试，并执行独立本地 Git commit；最终由用户决定是否 push。
- 保持已有 13 题 RAG、50 follow-up、50 problem、20-turn memory、360 question corpus 和 Wiki/Core-book 门禁。
- 不降低阈值，不填充独立 production held-out 真值，不把 fallback 结果表述为真实 cross-encoder/NLI 质量。

## Acceptance Criteria

- [ ] AC1：自然回答验证前保留 claim Evidence IDs；无映射 factual claim 为 not_verifiable，且不自动绑定全包 Evidence。
- [ ] AC2：ClaimType 与 VerificationStatus 分离，错误的“通用知识/建议/推断”事实不能绕过验证。
- [ ] AC3：heuristic 检查后 `entailmentChecked=false`，并独立记录 heuristic verification。
- [ ] AC4：缺失 grounding 状态反序列化为 unverified。
- [ ] AC5：cross-encoder、EmbeddingRescorer fallback、deterministic fallback 三层状态可区分；排序不再 raw-score 相加。
- [ ] AC6：DirectQA 默认不调用 planner、最大一轮 retrieval；budget guard 能阻止超额调用并记录实际消耗。
- [ ] AC7：Compatible API 与 Codex 根据 capability 运行同一 Understanding/Planning contract。
- [ ] AC8：低置信无显式指代的科研 follow-up 会升级，高置信自包含问题不升级。
- [ ] AC9：四个新增 ResearchIntent profile 的 completeness contract 通过测试。
- [ ] AC10：方法 hypothesis 不进入中性首轮检索，只有独立 corroborated expansion 才进入最终方法建议。
- [ ] AC11：semantic block 只扩展 `parent_block_id` 指向的 active 同文档 parent。
- [ ] AC12：全量 Rust tests/clippy、frontend tests/build、P3 verification、RAG eval、Wiki eval、core-book gate全部通过；冻结指标不下降。
- [ ] AC13：每一实施阶段有独立本地 Git commit，用户未要求前不 push。

## Out of Scope

- 伪造或代替独立研究者 production held-out 评审。
- 为了通过测试降低 Recall/MRR/nDCG/claim precision 门槛。
- 新增与审查无关的 Agent、Retriever 或 Wiki 内容。
- 自动下载 cross-encoder/NLI 模型作为普通问答的隐式副作用。

## Technical Notes

- 仓库已经存储 `parent_block_id`，无需重新发明 parent schema；问题在候选/扩展链路未消费。
- FastEmbed 提供 `TextRerank` cross-encoder API；本项目现有 fastembed/ORT 运行时可复用，但必须沿用“显式部署、普通查询不下载”的原则。
- 现有 QA contract 仍包含 natural v2 不按 claim coverage gate、appendix integrity 代表 supported 的旧约定；实施时必须同步修订。
