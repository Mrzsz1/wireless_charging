# 智能问答生产加固与最终发布门禁

## Goal

在不推翻现有 Research QA Pipeline、不新增无关 Agent/Retriever/Intent、也不降低冻结阈值的前提下，完成审阅文档要求的 P0 Production Hardening：原子 Claim、真实语义验证接口与回退、Cross-Encoder 生产部署验证、对抗与失败测试、独立 held-out 评测工具链、自动 Release Gate 和可审计发布报告。

## Background / Confirmed Facts

- 审阅基线是 GitHub `master@d5e9598b728c71b5aa195d6371ef7c15684e37ec`，与当前本地已推送基线一致。
- `claim_verification.rs` 当前以 `grounding::claim_segments` 的句界结果为 Claim；一个含“建议 + 因为 + 事实”的复合句会被整体分类，存在 claim smuggling 风险。
- `ClaimType` 与 `VerificationStatus` 已分离，但 `VerificationStatus` 尚无 `Unavailable`，生产仅运行 deterministic lexical heuristic；`entailmentChecked` 正确保持 false。
- `VerificationProvider` 当前是同步 heuristic 接口，Provider 失败会丢弃整份 claim report，而不是逐 Claim fail-soft 后回退。
- Cross-Encoder 已使用本地 FastEmbed `TextRerank`，但仅消费预部署目录；没有普通安装环境的显式 provisioning/status/health 流程，因此当前 RAG 基线可以长期 fallback。
- `qa_accuracy_eval.py` 已具备 evidence checksum、双 reviewer、第三人裁决和 Wilson 95% CI 校验；`heldout_questions.json` 仍处于 `awaiting_independent_curation` 且 cases 为空。
- 独立 held-out 题目与人工评审属于外部输入。本任务实现 schema、harness、报告和 gate，但不伪造数据，不以开发者自建题证明 Production Ready。
- 用户已有持续偏好：每个实施阶段必须本地 Git 保存；未明确要求前不再次 push GitHub。

## Requirements

### R1 — Atomic Claim Extraction

- 新增明确的 `AtomicClaim`，包含稳定 ID、文本、`ClaimType`、Evidence IDs、`VerificationStatus` 与可选 confidence。
- 将 segmentation、typing、evidence mapping、verification、repair 分离为可独立测试的步骤。
- deterministic segmenter 必须保留 Markdown/citation 边界，并处理建议+原因、事实+建议、因果、转折、并列与枚举；不得仅按连接词无条件切割。
- Evidence ID 必须随原子 proposition 保留；用户输入或无效 `[E#]` 不得变成系统 Evidence。
- 至少 50 个冻结 atomic/adversarial regression cases。

### R2 — Semantic VerificationProvider

- 在 deterministic verifier 之外新增语义验证结果 `Entailed | Contradicted | Unknown` 和统一 Provider contract。
- Codex 与 Compatible API 通过同一 capability/结构化输出路径运行语义验证；offline 保持 deterministic fallback。
- Provider timeout、无效 JSON、预算拒绝、不可用与取消必须有稳定状态、逐 Claim 回退和 telemetry；heuristic 结果不得设置 semantic entailment checked。
- Evidence 缺失/未知/graph-only 时 fail closed，不调用语义 Provider。
- 最终合并策略必须覆盖数量级、范围扩大、因果扩大、相关性转因果、否定冲突和 unknown。
- Answer Repair 最多一次，且 semantic verifier/repair 的模型调用计入同一个 request-scoped `LlmBudgetGuard`。

### R3 — Cross-Encoder Production Validation

- 提供机器级、显式、可检查的 reranker 模型生命周期：missing/partial/invalid/ready/error、模型版本、完整性校验、首次 provision/repair、重复运行、离线启动、损坏恢复与 fallback。
- 普通 query-time retrieval 不联网；只有显式 provisioning 命令允许下载。
- Telemetry 必须记录 requested/provider/model/available/fallback/reason/candidate count/latency。
- 提供真实模型 benchmark 入口与 before/after/fallback 报告；未部署真实模型时门禁必须 FAIL/PENDING，不能把 fixture 分数当生产证明。

### R4 — Adversarial / Failure / Stress Validation

- 覆盖 claim smuggling、citation laundering、scope/causality expansion、numerical hallucination、citation ID injection、knowledge prompt injection。
- Evidence prompt 必须标记为 untrusted data，知识正文不得改变系统指令。
- 覆盖 planner timeout、embedding unavailable、cross-encoder missing、semantic verifier timeout、Graphify unavailable、DB locked，不 panic、不 corrupt，并保留明确 fallback telemetry。
- 增加 20/50/100 轮 ResearchSessionState stress regression，验证替换/删除/目标变更使用最新状态。
- Provider matrix 覆盖 Codex、Compatible API、local-only 的 structured output、planning、generation、timeout、invalid JSON、rate limit、budget。

### R5 — Eval Metadata / Held-out Harness

- 所有生产 eval artifact 能记录 git commit、dataset version/hash、answer/embedding/reranker/verification provider/model、runtime config hash、timestamp、platform、CPU、memory。
- held-out harness 严格校验至少 30 个独立冻结问题、完整 run bundle、双 reviewer、分歧裁决、evidence integrity，并输出核心指标与 Wilson 95% CI。
- 不改变 `awaiting_independent_curation` 的事实；外部数据缺失时输出明确 PENDING/FAIL 原因。

### R6 — Automated Release Gate / Report / CI

- 新增冻结、机器可读的 release threshold 配置；最终 held-out 运行前不得自动调低。
- Release Gate 聚合 retrieval、conversation、grounding、open research、reliability、performance、真实 reranker/semantic verifier、held-out 数据；输出 PASS / CONDITIONAL PASS / FAIL 及逐项原因。
- 事实可靠性、Grounding、大量 Cross-Encoder fallback、缺失独立 held-out 或缺失真实 semantic verifier 不得得到 CONDITIONAL PASS。
- 生成 `QA_PRODUCTION_RELEASE_REPORT.md`，包含 commit/build/dataset/models/providers/metrics/performance/fallback/limitations/decision。
- 增加 PR deterministic CI 与可手动/定时触发的真实模型 Release Candidate gate；昂贵真实模型不强制每个 PR 运行。

## Fixed Release Thresholds

- Retrieval: Document Recall@20 ≥ 0.95；Recall@10 ≥ 0.90；MRR ≥ 0.85；nDCG@10 ≥ 0.85。
- Conversation: reference resolution ≥ 0.95；constraint preservation ≥ 0.97；objective preservation ≥ 0.97。
- Grounding: factual claim precision ≥ 0.97；unsupported factual claim rate ≤ 0.02；contradicted claim rate ≤ 0.01；citation correctness ≥ 0.98。
- Open research: relevant method recall ≥ 0.90；critical constraint preservation ≥ 0.97。
- Reliability: crash count = 0；provider failure handled = 1.00；fallback success ≥ 0.99；invalid verified state = 0。
- Cross-Encoder production fallback rate ≤ 0.05。
- Performance 阈值必须由目标机器配置文件显式冻结；缺失配置或测量即 FAIL，不猜测。

## Acceptance Criteria

- [ ] AC1：复合“建议+事实”不再因建议词而绕过 verification，50+ atomic cases 全部通过。
- [ ] AC2：每个 AtomicClaim 只有一个主要 proposition，并保留准确 Evidence ID 映射。
- [ ] AC3：语义 Provider 成功时真实记录 semantic checked/provider/model/confidence；heuristic fallback 保持 semantic checked=false。
- [ ] AC4：semantic timeout/invalid/budget/unavailable 逐 Claim fail-soft，deterministic fallback 完成且状态不伪装。
- [ ] AC5：Cross-Encoder 有显式 status/provision/repair/health，query-time 保持离线；真实模型 gate 能区分 enabled 与 fallback。
- [ ] AC6：对抗 Grounding、知识 prompt injection、失败注入、Provider matrix、20/50/100 轮 stress regression 通过。
- [ ] AC7：held-out evaluator 对不独立、少于 30、缺 run/review、reviewer 重复、缺 adjudication、checksum 篡改全部 fail closed。
- [ ] AC8：release gate 在当前缺失独立 held-out/真实模型实测时给出诚实 FAIL 与具体原因；完整合格 fixture 可 PASS。
- [ ] AC9：发布报告包含审阅要求的全部元数据、指标、fallback、限制和最终决策。
- [ ] AC10：CI 分离 deterministic PR gate 与真实模型 RC gate。
- [ ] AC11：全量 Rust fmt/test/clippy、frontend typecheck/tests/build、P3、RAG、question corpus、Wiki、held-out harness 与 release gate contract tests全部通过。
- [ ] AC12：每阶段独立本地 Git commit；不包含用户已有未跟踪文件；未再次请求前不 push。

## Out of Scope

- 不重写整个 `qa/`，不新增 Agent Framework、Retriever、ResearchIntent、知识图谱方案或 UI 大改。
- 不针对 held-out 单题添加特殊规则、alias 或 prompt。
- 不由 Codex 生成并审核最终独立 held-out 数据，也不伪造真实模型/真实用户机器结果。
- 不在事实可靠性核心 gate 未通过时宣布 Production Ready。
