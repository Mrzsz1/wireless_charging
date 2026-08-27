# Independent Held-out Phase 1 Runner 与冻结契约

## Goal

在正式盲测题目冻结前，完成与题目内容无关的通用 Held-out Runner、统一 canonical ResearchIntent 类型契约，并保证真实运行只调用当前生产 QA 核心路径、导出同轮完整审计包且全程 fail closed。

## Requirements

### R1 — Scope boundary

- 只修改 held-out runner、冻结数据集契约、`qa_accuracy_eval.py` 类型/验证共享、评测基础设施测试、README/scripts 和必要的 audit export plumbing。
- 不修改 Retrieval、Query Planner/Prompt、Conversation/State、Reranker、Embedding、Method Matcher、Answer Prompt、Semantic/Claim Verifier 行为。
- 仓库内 `evals/heldout_questions.json` 保持模板状态且 `cases=[]`；测试不得读取或复制 `research_questions_v1.json` 的 heldout 内容。

### R2 — Canonical type contract

- 新冻结 case 只允许：`direct_factual`, `literature_search`, `comparison`, `origin_derivation`, `method_improvement`, `solution_search`, `problem_modeling`, `related_problem`, `counterfactual`, `novelty`。
- `evals/heldout_questions.json`、Rust Runner、Python evaluator 和测试使用同一 canonical contract；不得额外映射新冻结数据。

### R3 — Frozen dataset validation

- Runner 支持外部 `--dataset`, `--output-dir`, `--repository`，以及可选 provider/model/reasoning effort。
- 启动时验证 role/split/status、independent curator、小写 SHA-256 curator ID、frozen_at、canonical cases SHA-256、至少 30 cases、唯一 ID、合法 type 与非空 question。
- Python evaluator 与 Runner 对同一数据集给出一致接受/拒绝结果。

### R4 — Reproducible official run

- 记录 Git commit；存在任何未提交/未跟踪工作区内容时 fail closed。
- 一个 dataset SHA + Git commit + runtime config 对应唯一 run identity；已有 official run 不覆盖、不选择性重跑。
- 每个 case 使用独立空会话上下文；case 内未来可扩展多轮，但不同 case 不共享状态。

### R5 — Production execution and audit bundle

- 默认 executor 使用桌面端当前生产 retrieval/planning/generation/semantic verification/audit functions；不使用 offline fixture、mock retrieval、冻结 planner 或额外美化 LLM。
- 每题保存同轮真实 `answer`, 完整 `EvidenceItem[]`, 生产 `QaRunManifest` 和从该 manifest 的 atomic claim 审计投影出的 `answerClaims`。
- 写成功前验证 claim 文本逐字存在于 answer、claim 数与 `answerCompleteness.claimCount` 一致、Evidence IDs/manifest checksums 一一匹配。
- `.json.part` 写入、flush/sync 后原子 rename；任一 case 失败终止整轮且不伪装成功。

### R6 — Tests and release boundary

- 只用 synthetic/development fixture，覆盖未冻结/非独立/hash 错误/少于 30/重复 ID/非法 type/dirty tree/既有 run、session 隔离、claim/evidence/checksum/atomic write/runtime metadata。
- 不降低现有 release threshold，不正式冻结题目，不执行真实 blind run。

## Acceptance Criteria

- [ ] AC1：Runner CLI、外部 dataset/output/repository 参数和 npm entry point 完成。
- [ ] AC2：Rust/Python/template canonical allowed types 完全一致。
- [ ] AC3：所有冻结/curation/hash/count/case 校验均 fail closed。
- [ ] AC4：默认 executor 使用生产 QA 核心函数，case 会话隔离且不读取 heldout 候选内容。
- [ ] AC5：审计包直接保存同轮 answer/evidence/runManifest/claim projection，并通过 claim 与 checksum 自校验。
- [ ] AC6：run identity 不覆盖、Git dirty fail closed、输出原子写入。
- [ ] AC7：synthetic runner tests 与必要现有 QA/retrieval/conversation/semantic tests 通过。
- [ ] AC8：规范、README、本地 Git 提交、Trellis 归档与 journal 完成。

## Production boundary

本任务只完成 Phase 1 基础设施。正式 dataset 由独立 curator 在新代码 commit 上冻结，之后停止 QA 开发再运行 blind test。
