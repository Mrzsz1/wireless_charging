# Wiki 问答回归集

`gold_questions.json` 固定 10 个真实使用问题，用来防止 wiki 改动后 `/solve`、`/novelty` 和跨文献关系回答退化。它明确属于 **development/regression** split：题目及期望结果对开发者可见，因此只证明回归契约，不代表生产端到端事实准确率。

## 用法

只检查题集结构、链接目标和类型配额：

```powershell
py -3 tools/wiki_eval.py
```

若已把 Claudian/LLM 的答案保存为 `evals/answers/<case-id>.md`，可进一步检查每个答案是否包含预期 wikilink、库水位和题集 `must_mention` 必提概念：

```powershell
py -3 tools/wiki_eval.py --answers-dir evals/answers
```

脚本做确定性链接、水位和必提概念契约检查，不替代人工判断答案是否真正理解了方法边界，也不输出“准确率”结论。每次修改核心 synthesis、问答模板或导航结构后运行一次。

当前答案基线与维护者初审见 `evals/answers/REVIEW.md`；切换问答模型（例如 Luna）后应保留旧基线并重新运行评测。

## Semantic RAG / QueryPlan 回归

`semantic_rag_questions.json` 保存陌生中文表述及其经 Provider JSON Schema 约束后的
`qa-query-plan-v1` 夹具。它验证开放 Facet、双语扩展查询、必需证据类型、Wiki-primary
配对和原文行号定位，不把问题映射写入生产检索代码。

Rust 回归会建立真实仓库索引并走完整 QueryPlan 检索路径；测试时注入冻结计划，因此不登录
Codex、不联网、也不下载本地 embedding 模型：

```powershell
cd apps/desktop/src-tauri
cargo test semantic_query_plan_regressions_recall_auditable_primary_sources --lib
```

该文件同样属于 development regression，只能证明结构化计划与召回契约，不能作为端到端
事实准确率声明。事实准确率仍以独立冻结 held-out、完整审计包和双盲人工 claim 评审为准。

## Markdown 科研 RAG v2 双读评测

`rag_retrieval_cases.json` 覆盖指定书籍、开放论文+书籍、双语改写、新概念、跨文档比较、
多轮指代和真实零证据。它只保存问题、冻结的 planner 语义扩展、预期文档/标题、通道和
locator 边界，不要求最终回答出现固定中文短语。

运行时会在内存 SQLite 中从当前 Markdown 重建索引，分别执行 legacy 单轮候选和 v2
RetrievalContract，生成 JSON 与 Markdown 差异报告；不会调用 Answer Provider、外网或下载模型：

```powershell
cd apps/desktop
npm run eval:rag
```

默认报告位于 `evals/reports/rag-evaluation-latest.{json,md}`，包含 source resolution、
requested channel attempt、文档 Recall@5/10/20、heading Recall、MRR、nDCG@10、locator
validity、零证据 FN/FP、轮数、耗时以及逐题 legacy/v2 改善和退化。关键用例与 Top20、
locator、通道、来源解析属于硬门禁；平均排序指标用于诊断，不能单独掩盖关键来源失败。

## Conversation Understanding 回归

`conversation_understanding_cases.json` 冻结 10 个真实库内对象与 5 类 follow-up
表达的笛卡尔积，共 50 个多轮问题，另含两个序数指代边界用例。它覆盖普通指代、方法改进、
模型来源、方案比较、可迁移解法，以及“第二种/第三个方案”。

运行：

```powershell
cd apps/desktop/src-tauri
cargo test frozen_follow_up_matrix_covers_fifty_resolution_and_routing_cases --lib
```

测试要求 standalone question 带回正确实体、记录实际使用的历史消息 ID，并得到冻结的
`ResearchIntent` 与 `ExecutionMode`。该题集属于 development/regression，不替代独立 held-out。
当前冻结结果见 `conversation-understanding-baseline.md`。

## Cross-Encoder Reranker 回归

`semantic_reranker_cases.json` 提供 10 个模型无关的重排回归用例，用冻结的交叉编码器分数验证
`CrossEncoderResearchReranker` 的加权 RRF、候选预算和指标门禁，不在测试中下载模型：

```powershell
cd apps/desktop/src-tauri
cargo test cross_encoder_reranker_improves_recall_at_five_mrr_and_ndcg_without_losing_recall_at_twenty --lib
```

端到端 `npm run eval:rag` 另外记录实际 reranker provider、状态、耗时与 fallback。Cross-encoder
未部署时依次回落到 `EmbeddingRescorer`、`DeterministicResearchReranker`，并保持 Recall@20 门禁。结果见
`semantic-reranker-baseline.md`。

## Evidence Manager 回归

`EvidenceManager` 在最终 evidence 编号前执行稳定去重、来源权威轻量加权、文档/类型多样性、
父上下文扩展与 token 预算估算（最终裁剪仍由 `ContextPlan` 负责）：

```powershell
cd apps/desktop/src-tauri
cargo test evidence_manager --lib
```

回归结果及当前 13 题端到端门禁见 `evidence-manager-baseline.md`。

## Claim-level Verification 回归

生成后的自然 Markdown 与结构化回答都会经过独立 claim extraction、evidence alignment、
deterministic verifier fallback 和 answer repair。合法 Evidence ID 不再自动等于语义支持：

```powershell
cd apps/desktop/src-tauri
cargo test claim_verification --lib
cargo test obvious_unsupported_claim_is_repaired_and_never_reported_as_verified --lib
```

冻结状态矩阵见 `claim-verification-cases.json`，门禁记录见
`claim-verification-baseline.md`。

## Problem Understanding / Method Matcher 回归

冻结的 50 个真实问题描述覆盖 domain、objective、constraints、related problem type 与
candidate methods。门禁同时检查约束不丢失、相关问题召回和方法召回：

```powershell
cd apps/desktop/src-tauri
cargo test problem_understanding --lib
```

数据见 `problem-understanding-cases.json`，结果见
`problem-understanding-baseline.md`。

## Research Session State 回归

20 轮科研聊天门禁覆盖目标、约束、方法、假设的新增、替换和删除，后续问题必须读取最新状态：

```powershell
cd apps/desktop/src-tauri
cargo test research_memory --lib
```

结果见 `research-session-state-baseline.md`。

## Adaptive Query Routing 回归

DirectQA / ResearchQA / ExploratoryResearch 使用不同的检索、LLM call 与 token ceiling，
并保留显式 legacy planner callback 兼容入口：

```powershell
cd apps/desktop/src-tauri
cargo test adaptive_routing --lib
```

quality / latency budget / LLM calls / token cost 对比见
`adaptive-routing-baseline.md`。

## 360 题冻结科研问题集

`research_questions_v1.json` 保存 360 个科研问题，覆盖 12 个无线充电相邻场景与 10 种
ResearchIntent，并预先拆分为 development 160、regression 120、heldout 80。heldout 不含
期望答案、期望文档或 must-mention，禁止据此逐题调规则。

```powershell
cd apps/desktop
npm run eval:questions
```

验证器检查总量、唯一性、字段、split 配额、domain/intent 水位、heldout 泄漏和 canonical
SHA-256 seal。结果见 `research-question-dataset-baseline.md`。生产事实准确率仍执行下方更严格的
独立研究者冻结与双评审流程。

## Production held-out 准确率

`heldout_questions.json` 是独立冻结入口。当前状态为 `awaiting_independent_curation`，不预填模型自行构造的“真值”。冻结要求：

1. 由未参与检索器、提示词和回归集开发的研究者独立抽样，至少 30 题；
2. 冻结后才生成 `evals/heldout-runs/<case-id>.json` 审计包；
3. 匿名双人逐 claim 复核，分歧交第三人裁决，保存到 `evals/heldout-reviews/<case-id>.json`；
4. 运行 `py -3 tools/qa_accuracy_eval.py`，报告事实 precision、Wilson 95% 区间、partial/unsupported/not-applicable、引用正确率/完整率以及 reference/method/constraint 指标；
5. 自动语义蕴含保持未启用，结果中固定为 `semanticEntailmentChecked=false`。

### Held-out run 的可核验 schema

每个 `heldout-runs/<case-id>.json` 必须直接来自同一轮审计包，并包含：

```json
{
  "question": "冻结题目原文",
  "answer": "最终回答正文，含 [E1]",
  "answerClaims": [
    {"claimId": "C1", "text": "回答中逐字存在的 claim [E1]", "citedEvidenceIds": ["E1"]}
  ],
  "evidence": ["完整 EvidenceItem 对象"],
  "runManifest": {
    "evidenceChecksums": [
      {"evidenceId": "E1", "stableSourceId": "...", "sha256": "64 位小写十六进制"}
    ],
    "answerCompleteness": {"claimCount": 1, "complete": true}
  }
}
```

`answerClaims` 是人工评审的冻结 claim 清单。其长度必须严格等于
`runManifest.answerCompleteness.claimCount`，每个 `text` 必须逐字出现在
`answer` 中，每个 `citedEvidenceIds` 也必须以 `[E#]` 出现在该 claim 文本中。这样评审不能用
“manifest 声明 99 个 claim、实际只提交 1 个 verdict”的方式缩小分母。

证据数组及 checksum 数组必须非空，并保留 Rust `EvidenceItem` 的全部字段。评测器按该结构体的固定字段顺序生成
UTF-8 compact JSON（与 `serde_json::to_vec(EvidenceItem)` 一致），从当前 `evidence`
内容重新计算 SHA-256，同时校验 `stableSourceId`。只对比 evidence ID、缺失 checksum、
伪造 checksum 或 checksum 生成后篡改 snippet/路径/位置等任一证据字段都会 fail closed。

### 独立双评审与分歧裁决 schema

每个 `heldout-reviews/<case-id>.json` 使用以下结构：

```json
{
  "case_id": "case-id",
  "primary_reviews": [
    {
      "reviewer_id_hash": "64 位小写 SHA-256",
      "blinded": true,
      "independent": true,
      "claims": [
        {"claim_id": "C1", "claim": "与 answerClaims 完全一致 [E1]", "verdict": "supported"}
      ]
    },
    {
      "reviewer_id_hash": "另一个 64 位小写 SHA-256",
      "blinded": true,
      "independent": true,
      "claims": [
        {"claim_id": "C1", "claim": "与 answerClaims 完全一致 [E1]", "verdict": "contradicted"}
      ]
    }
  ],
  "adjudication": {
    "reviewer_id_hash": "第三个 64 位小写 SHA-256",
    "blinded": true,
    "independent": true,
    "claims": [
      {"claim_id": "C1", "claim": "与 answerClaims 完全一致 [E1]", "verdict": "not_verifiable"}
    ]
  }
}
```

两份 primary review 必须来自不同 reviewer，且各自恰好覆盖全部 `answerClaims` 一次。
无分歧时省略 `adjudication`；存在分歧时必须由不同于两名 primary reviewer 的第三人
裁决全部且仅裁决分歧 claim。聚合统计每个 claim 只计一次：一致 verdict 直接采用，
分歧 verdict 采用第三人裁决。

在独立题集冻结前可验证入口状态：

```powershell
py -3 tools/qa_accuracy_eval.py --allow-pending
```

智能问答中的“复制审计包”会导出问题、最终回答、本轮证据和 `QaRunManifest`；不包含凭据或 provider 原始 payload。

### 独立 curator 与盲审工作流

仓库提供 50-case 空白模板，不预填 Codex 生成的问题或真值：

```powershell
py -3 tools/qa_heldout_workflow.py template --cases 50 --output evals/heldout_curator_template.json
```

独立 curator 填写问题、canonical critical-constraint ID 与 acceptable method-family ID，完成
独立性声明和匿名 SHA-256 后冻结：

```powershell
py -3 tools/qa_heldout_workflow.py freeze --draft CURATOR_DRAFT.json --frozen-at UTC_TIMESTAMP --output FROZEN_HELDOUT.json
```

冻结后运行同一 Release Candidate，再导出不含 system verification verdict/run manifest 的盲审包：

```powershell
py -3 tools/qa_heldout_workflow.py export-review --dataset FROZEN_HELDOUT.json --runs HELDOUT_RUNS --output BLIND_REVIEW_BUNDLES
```

A/B reviewer 和 C adjudicator 完成后，从同一 dataset/run seal 派生全部三个工件：

```powershell
py -3 tools/qa_heldout_workflow.py derive --dataset FROZEN_HELDOUT.json --runs HELDOUT_RUNS --reviews HELDOUT_REVIEWS --output evals/heldout-derived-latest
```

`qa-production-eval` 自动读取该目录。缺 run/review、重复 reviewer、非独立 reviewer、漏 claim、
缺 adjudication、checksum 篡改或三个工件 sourceRun 不一致都会 fail closed。

冻结数据集还必须提供 `curation.independent=true`、匿名 curator hash、冻结时间和
canonical `cases_sha256`。每个 `answerClaims` 可用 `dimension` 标注
`factual/reference/method/constraint`；旧审计包缺省为 `factual`。

## Production Release Gate

冻结阈值位于 `qa_release_thresholds.json`。门禁只读取带统一 metadata envelope 的
`evals/runs/<run-id>/` 工件；缺工件、缺字段、非有限数、非真实 reranker/semantic provider、
未独立冻结 held-out 或未冻结目标机器性能配置均 fail closed：

```powershell
py -3 tools/check_qa_release_gate.py --artifacts evals/runs/<run-id> --report QA_PRODUCTION_RELEASE_REPORT.md
```

完整合格工件输出 `PASS`。默认不启用 `CONDITIONAL PASS`，事实可靠性、Grounding、真实模型
与独立 held-out 等核心门禁在任何情况下都不得条件通过。PR CI 仅跑确定性 contract；真实模型
Release Candidate gate 通过手动或定时 workflow 运行。

### 统一 Production Eval Harness

`tools/qa_production_eval.py` 是单一生产评测入口。默认顺序运行真实 RAG、50-case
Conversation evaluator、真实 Semantic Verifier benchmark 和 reliability contracts，再在
`evals/releases/<git-sha>/` 原子生成 manifest、retrieval/conversation/reliability/reranker/
semantic/performance 工件、release gate 和报告：

```powershell
cd apps/desktop
npm run eval:production
```

已单独完成昂贵模型运行时，可复用同一工作树中的 latest machine reports：

```powershell
py -3 tools/qa_production_eval.py --use-existing --allow-fail
```

Conversation 数据集使用 canonical constraint/objective ID，不做自然语言全文 exact match。
当前 50/50 case 的 reference resolution、constraint preservation、objective preservation 均为
1.000。外部 held-out 和 sealed performance profile 缺失时，harness 仍写出可审计工件并保持
最终 FAIL，不手填通过值。
