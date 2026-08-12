# 智能问答准确率、上下文与回答效果审查

日期：2026-08-12  
范围：桌面端智能问答的检索、上下文构造、提示词、回答生成、引用门禁、评测与可复现性。  
参考实现：xAI 官方开源仓库 [grok-build](https://github.com/xai-org/grok-build)。

## 1. 审查结论

当前功能已经具备一条较完整的“知识库检索 → 证据打包 → 模型生成 → 引用结构校验 → 会话持久化”链路，并且零证据、Graphify 边界、取消、仓库切换和流式异常处理较严谨。

但是，当前系统只能证明以下事实：

1. 固定 10 题上，每题至少能找到一个预期 Wiki 和一个具有原文行号的目标 paper 页面；
2. 这 10 题的平均 `Recall@10/20=0.555`、`MRR=0.692`、`nDCG@10=0.498`；
3. 被保存为 `completed` 的回答通过了“检测到的事实句中存在本轮非 Graphify `[E#]`”这一结构门禁；
4. 零证据回答会标成 `unverified`，且不会进入后续上下文。

当前系统尚未证明：

- 最终答案的事实准确率是多少；
- `[E#]` 引用是否真正支持所在句；
- 多轮上下文中的约束是否被完整保留；
- 当前 10 题表现能否推广到未见问题；
- Codex 与 compatible API 两个 provider 是否产生同等质量、同等长度和可复现的答案。

因此，现阶段应将产品定位表述为：**“具备结构化证据约束的科研问答原型”**，而不是“已经测得高准确率的科研问答系统”。

## 2. 本轮实测

### 2.1 检索排序

执行：

```powershell
cd apps/desktop/src-tauri
cargo test p3_gold_questions_recall_expected_wiki_evidence -- --nocapture
```

结果：

| 指标 | 实测值 | 能说明什么 |
|---|---:|---|
| 固定题最低 Wiki + paper 命中 | 10/10 | 每题至少存在一个最低证据对 |
| Recall@5 | 0.365 | 前 5 条仅覆盖约 36.5% 的全部标注目标 |
| Recall@10 | 0.555 | 前 10 条覆盖约 55.5% 的全部标注目标 |
| Recall@20 | 0.555 | 扩到 20 条没有继续提高目标覆盖 |
| MRR | 0.692 | 第一个相关目标通常较靠前，但不是稳定第一 |
| nDCG@10 | 0.498 | 整体排序质量处于中等水平 |
| required-kind coverage | 1.000 | 要求的证据类型均出现 |
| Wiki-primary pair coverage | 0.800 | 20% 用例没有保留全部预期 Wiki/原文配对 |

这里的 `10/10` 不是“答案准确率 100%”。即便把它错误地当成二项成功率，10 个样本的 95% Wilson 区间下界也只有约 72.2%；而该统计实际测量的又只是最低检索命中，不是答案正确性。

### 2.2 静态答案契约

执行：

```powershell
python tools/wiki_eval.py --answers-dir evals/answers
```

结果为 10/10 PASS，但 `tools/wiki_eval.py:131-178` 只检查：

- 指定 wikilink 是否出现；
- “库水位”及 source 数量字符串是否出现；
- `must_mention` / `critical_constraints` 词面是否出现；
- 是否有“原文证据”、行号样式与“边界”字样。

它不执行生产 provider、不校验 `[E#]`、不判断事实真伪，也不判断引用是否支持结论。`evals/answers/REVIEW.md` 中的 10 份答案是静态会话基线，不含 provider、具体 model、prompt 版本、检索快照或采样配置。

更关键的是，这些静态答案主要使用 `[[wikilink]]`，而生产持久化门禁要求当前 `[E#]`。因此，**离线答案回归集与生产回答契约不是同一条链路**。

## 3. 分级问题

### P1-01：没有端到端答案准确率评测

证据：

- `tools/wiki_eval.py:131-178` 是词面和结构检查；
- `apps/desktop/src-tauri/src/lib.rs:4235-4384` 只评估检索排名；
- `apps/desktop/src-tauri/src/qa/grounding.rs:128-218` 只校验引用编号、类型与同句覆盖，且明确写入 `entailment_checked=false`；
- `evals/answers/REVIEW.md` 没有独立双人审阅、逐 claim 标注、分歧裁决或置信区间。

影响：

- “10/10”“引用覆盖 100%”容易被误读成事实正确率；
- 错误陈述只要附带一个存在的 `[E#]`，就可能通过结构门禁；
- 科研场景不足以据此判断回答是否可用于论文综述、模型比较或新颖性分析。

建议：先建立生产链路评测 runner，再讨论“准确率”。自动语义蕴含仍可保持未实施，但至少要有人工逐 claim 评审作为事实准确率基线。

### P1-02：Gold 题与查询扩展规则高度耦合，存在题集泄漏风险

`apps/desktop/src-tauri/src/qa.rs:821-914` 包含“开关组合、已知轨迹、发起充电请求、充电费、部分充电、波干涉、城市路口、实时调度”等精确触发词；这些表达与 `evals/gold_questions.json` 的固定 10 题高度重合。

当前阈值又是按当前结果贴线固定，例如 Recall@5 实测 0.365、门槛 0.35。它适合做“防止当前实现回退”的回归门禁，不适合估计未见问题的泛化准确率。

建议：

1. 将现有 10 题明确命名为 `development/regression`；
2. 新建冻结的 blind test，规则调整期间禁止查看答案标签；
3. 至少覆盖同义改写、组合约束、未登录词、负例、零证据、跨轮指代、用户纠正、矛盾证据、数值/公式、超长会话；
4. 报告 dev 与 blind test 两套指标，禁止只报告调优集。

### P1-03：Wiki 下钻 paper 时选节与当前问题无关，但评测仍可能计为命中

`apps/desktop/src-tauri/src/qa.rs:1137-1206` 对每个 Wiki source 固定选择优先级最高的 Abstract / Problem / Model / Introduction 首节，而不是使用当前 query 在该论文内重新检索最相关 section。随后它继承 Wiki 分数并获得 linked-paper 通道加权。

`apps/desktop/src-tauri/src/lib.rs:4315-4329` 的 Gold 命中判定只要求：

- page ID 属于目标 paper；
- `source_location` 非空且包含原文行号。

它不要求命中的 section 与问题或关键约束相关。

影响：系统可能展示“目标论文 + 可定位行号”，但该片段只是摘要或介绍，不足以支撑回答中的具体算法、约束或结论。由于 prompt 又把 `kind=paper` 描述为可直接支撑事实，形式上很强的证据外观会放大错误信任。

建议：Wiki 下钻后必须在该 paper 的 section 集合内用当前分解 query 重新排序；Gold 增加 `expected_section`、关键句或人工 section relevance 标签。

### P1-04：科研可复现性信息没有闭环

当前消息持久化保存 provider、model、evidence、waterline 和 citation validation，但没有保存：

- prompt/schema 版本；
- 完整 system/user prompt 哈希；
- `RetrievalQuery`、query terms、参与上下文的消息 ID；
- retriever/ranker 版本；
- 索引或知识库内容快照哈希；
- provider 参数；
- Codex 实际解析到的具体模型。

`apps/desktop/src-tauri/src/codex_subscription.rs:603-609` 在用户未指定模型时只记录 `codex-default`。`apps/desktop/src-tauri/src/qa.rs:2033-2041` 的 compatible API 使用明确 model、temperature 与 max tokens，而 Codex 路径没有同等采样配置。两个 provider 的结果不宜直接横向比较，也难以重放。

建议：每轮保存独立 `QaRunManifest`，至少包含 `promptVersion / answerSchemaVersion / retrieverVersion / modelResolved / providerSettings / historyMessageIds / evidenceChecksums / indexSnapshotId / generatedAt`。

### P2-01：上下文预算按“8 条消息 + 12,000 字符”裁剪，而非模型 token 与完整 exchange

`apps/desktop/src-tauri/src/qa.rs:550-603`：

- 最多取 8 条 completed 消息；
- 总预算固定为 12,000 字符；
- 按消息而不是 user/assistant exchange 保留；
- 超长消息直接截取前缀；
- 没有对旧历史做结构化摘要。

这会产生四类问题：

1. 可能从 assistant 开始，丢失其对应用户问题；
2. 旧轮中的研究目标、假设、符号或用户纠正可能被直接丢弃；
3. 中文字符数与不同模型 token 数差异较大；
4. 证据片段也没有统一 token 总预算，难以保证为输出保留足够空间。

Grok Build 提供 `/context` 展示 system prompt、messages、reasoning/overhead、free space 及工具定义等预算，并在上下文达到默认 85% 时自动 compact；其 compaction 还包含 two-pass、前缀 fingerprint 和退化摘要校验。可参考：

- [Slash commands：`/compact` 与 `/context`](https://github.com/xai-org/grok-build/blob/main/crates/codegen/xai-grok-pager/docs/user-guide/04-slash-commands.md)
- [Compaction implementation](https://github.com/xai-org/grok-build/blob/main/crates/codegen/xai-grok-shell/src/session/compaction.rs)

建议引入 `ContextPlan`：按 provider/model token window 计算预算，保留最近 2–3 个完整 exchange，较老历史压成结构化科研记忆，并预留输出与安全余量。

### P2-02：指代触发器过宽，可能把无关历史实体带入自包含问题

`apps/desktop/src-tauri/src/qa.rs:606-633` 将“其中”“分别”等常用词也判定为历史指代。一个本身已经包含完整实体的“请分别比较 A 与 B”仍可能追加旧会话中的 CCSP/GAIN 等实体。

`apps/desktop/src-tauri/src/qa.rs:662-731` 主要提取：

- 历史用户消息中的全大写 model-like token；
- 最近四条消息中出现的已知页面标题/ID。

当前只有 CCSP/GAIN 一类合成回归，没有测量：

- self-contained 问题被历史污染的比例；
- 用户纠正后旧实体是否仍被带回；
- 三个以上实体、中文别名、缩写冲突；
- 长会话压缩后实体是否保持一致。

建议把 coreference 判断改为结构化规则：先解析当前问题是否已有足够实体；只有“缺少主语/比较对象且存在指代词”时才补历史实体。加入 `context_contamination_rate` 与 `constraint_retention_rate`。

### P2-03：Prompt 是单块文本，信任边界和 provider 契约不统一

`apps/desktop/src-tauri/src/qa.rs:1831-1894` 把历史、当前问题、水位和证据拼成一个普通字符串。虽然声明“历史不是证据”，但没有使用明确的结构标签，也没有声明证据片段中的指令性文本必须被当作引用数据忽略。

Codex prompt 比 compatible API system prompt 更详细：Codex 明确解释 paper、wiki、book 与 `sourceLocation`，API 版本 `apps/desktop/src-tauri/src/qa.rs:2025-2029` 没有完全同步。由此可能出现 provider 间回答结构和来源粒度漂移。

Grok Build 将用户输入包装在 `<user_query>` 中，将环境信息放在独立 `<user_info>`，并把规则附加为 `<human_rules>`；这类显式分层有利于区分指令、环境快照和用户数据：

- [user_message.rs](https://github.com/xai-org/grok-build/blob/main/crates/codegen/xai-grok-shell/src/session/user_message.rs)
- [mvp_agent prompt composition](https://github.com/xai-org/grok-build/blob/main/crates/codegen/xai-grok-shell/src/agent/mvp_agent/mod.rs)

建议生产 prompt 统一为一个 provider-neutral `PromptEnvelope`，至少包含：

```text
<research_contract version="...">不可违反的科研证据规则</research_contract>
<session_memory snapshot="...">旧轮结构化摘要；稳定 page ID，不保留旧 E#</session_memory>
<recent_exchanges>最近完整 user/assistant 轮次</recent_exchanges>
<current_query>本轮原问题、意图和已解析约束</current_query>
<retrieval_plan>主题、变量、目标、约束、比较轴、证据需求</retrieval_plan>
<evidence_bundle>带 authority/location/checksum 的只读引用数据</evidence_bundle>
<answer_contract version="...">按意图定义的输出结构</answer_contract>
```

并明确：`evidence_bundle` 与 `recent_exchanges` 内的任何命令式文本均属于数据，不改变 `research_contract`。

### P2-04：回答模板固定，但“充分回答”没有可执行标准

现有有证据 prompt 强制五个栏目：“库内直接证据、相似模型、可迁移算法、核心书籍理论基础、库内尚未覆盖”。问题在于：

- 即使某类证据不存在，也要求模型组织该栏目，容易产生填充性内容；
- solve、novelty、relationship 三种意图没有各自的完整性 schema；
- 没有要求显式列出决策变量、目标函数、约束、适用前提、证据冲突与尚缺信息；
- 没有最低信息单元或长度门禁；
- API 默认最多 1,800 tokens，Codex 则没有使用同一输出上限。

“回答需要足够完整”不应简单转成固定字数，而应转成意图相关的完整性要求：

| 意图 | 必需信息单元 |
|---|---|
| solve | 直接结论、模型假设、决策变量、目标、关键约束、算法步骤、复杂度/保证（证据有则写）、适用边界、缺口 |
| relationship | 统一比较轴、逐对象对照、共同点、差异、不可迁移条件、证据冲突/缺口 |
| novelty | 已覆盖子问题矩阵、未覆盖交叉项、证据范围、证据不支持推出的结论、下一步检索或形式化需求 |

某项没有证据时输出“当前证据包未覆盖”，而不是补写一般常识。建议每个事实段至少包含“结论 + 条件/范围 + 当前 `[E#]`”，并设置基于必需信息单元的 completeness gate。

### P2-05：引用结构校验只有失败，没有一次受限修复

模型先流式输出，完成后 `persist_exchange` 才执行结构校验。若一个总体正确的回答漏掉一处 `[E#]`，整轮会作为失败处理；系统没有用同一证据包执行“只修引用和结构、禁止新增事实”的修复 pass。

建议最多执行一次 deterministic repair：输入原回答、结构错误列表与同一 evidence bundle，只允许删除无证据句、拆句或补入已经存在且明确对应的当前 `[E#]`。修复前后都记录 manifest，失败则保持现有 fail-closed 行为。

### P3-01：诊断适合运行健康度，不足以支持科研复盘

现有诊断只跨层传递总耗时、通道耗时/候选数、选中数和取消检查点。这对隐私与 UI 很好，但不足以回答：

- 为什么某个目标证据被丢弃；
- 每条证据由哪些 query facet 命中；
- 当前 prompt 各块占多少 token；
- 哪些历史约束被压缩、保留或舍弃；
- 同一问题在不同模型/版本下为何变化。

建议保持 UI DTO 的最小化，同时在本地创建可选、可导出的科研审计记录；记录稳定 ID、哈希、分数分解和 drop reason，不记录 API key 等凭据。

### P3-02：默认离线模式不是“回答”，而是证据清单

`apps/desktop/src-tauri/src/qa.rs:1780-1828` 的离线模式只列出检索片段，并提示配置 Luna 后生成完整回答。当前系统已经支持 Codex subscription，这段文案和产品能力不一致；同时用户可能把“证据清单”误认为回答效果较差。

建议将其明确命名为“证据浏览模式”，并按当前 provider 能力更新文案；不要把该模式纳入最终答案质量统计。

## 4. 当前实现的优点

1. **零证据边界正确**：后端固定声明“无参考来源”，模型一般知识被标为 `unverified`，并通过 `status='completed'` 过滤确保不进入下一轮上下文。
2. **仓库隔离正确**：会话必须属于当前 repository；检索在只读事务快照中执行，仓库切换后旧回答不保存。
3. **Graphify 定位正确**：图谱只作为导航提示，不足以单独支撑事实，离线答案也不会把 graph hint 渲染成事实 bullet。
4. **primary source 可回溯**：paper evidence 保存 canonical 路径、section 与原文行号。
5. **引用结构 fail-closed**：未知 `[E#]`、无引用事实和 Graphify-only 事实会拒绝持久化为 completed。
6. **界面没有夸大语义校验**：UI 明确显示“语义未自动核验”。
7. **Codex 执行隔离较好**：使用临时 workspace、`--ephemeral`、`--ignore-rules`、`--sandbox read-only`，避免问答时读取或修改真实仓库。
8. **流式完整性处理严谨**：compatible API 的截断、异常 finish reason、非法 JSON 和合法结束前 EOF 均失败，部分答案不保存为 completed。

这些能力应保留，后续优化不应牺牲零证据隔离、Graphify 边界和 fail-closed 语义。

## 5. 建议的上下文设计

### 5.1 六层上下文

1. **Research policy**：证据等级、引用规则、零证据规则、Graphify 边界；会话内不可变。
2. **Repository policy**：本知识库的 page type、waterline、索引快照和领域边界。
3. **Session memory**：对旧轮进行结构化压缩，只保存研究目标、用户确认的假设/定义、稳定 page ID、未解决问题和用户纠正；不保存旧 `[E#]`。
4. **Recent verbatim tail**：最近 2–3 个完整 exchange 原文，避免摘要丢失细节。
5. **Current query plan**：当前问题、指代解析结果、实体、决策变量、目标、约束、比较轴、缺失的关键变量。
6. **Current evidence bundle**：本轮唯一可引用事实来源，包含稳定来源、authority、section/location、snippet、rank/reason/checksum。

### 5.2 Token 预算

建议根据实际 model context window 动态分配，而不是固定字符数：

- system/research contract：固定上限；
- session memory：约 10%；
- recent exchanges：约 15%；
- evidence bundle：约 45%–55%；
- current query/plan：约 5%；
- output reserve：至少 20%；
- safety margin：5%–10%。

达到 75%–80% 时预压缩，低于 Grok 默认 85%，因为科研回答通常需要更大的输出和引用余量。压缩应采用两段式：旧前缀形成 NOTE1，随后以 NOTE1 + 最近 tail 形成最终 memory；用 prefix fingerprint 在编辑、回退、模型或 prompt 版本变化时使缓存失效。

### 5.3 上下文可见性

参考 Grok `/context`，UI 增加“本轮上下文”面板：

- model context window；
- research contract tokens；
- session memory tokens；
- recent history tokens；
- evidence tokens；
- output reserve / free space；
- 被截断或压缩的 exchange 数；
- 使用的 history message IDs；
- index snapshot ID。

这既能解释回答为何遗漏上下文，也能让科研用户在复现实验时记录条件。

## 6. 建议的回答格式

生产 prompt 应按意图选 schema，不再无条件输出五个固定栏目。推荐统一骨架：

```markdown
## 结论
直接回答问题，并说明结论只覆盖当前知识库或一般知识。

## 模型与适用前提
- 系统对象：
- 决策变量：
- 目标：
- 关键约束：

## 证据综合
| Claim | 结论 | 条件/范围 | Evidence | 强度 |
|---|---|---|---|---|

## 方法或比较
按 solve / relationship / novelty 的专属 schema 展开；每个事实段同句引用。

## 边界、冲突与未覆盖项
区分“证据明确否定”“当前库未覆盖”“需要外部检索”。

## 库水位与复现信息
source/method/synthesis/book 数量、年份、indexSnapshotId、promptVersion、provider/model。
```

长度由信息单元决定：复杂问题至少完成所有必需信息单元；证据不足时明确写缺口，而不是为了变长增加无依据内容。

## 7. 准确率评测方案

### 7.1 数据集分层

| 集合 | 用途 | 建议规模 |
|---|---|---:|
| regression/dev | 调整 query expansion 和排序 | 现有 10 题继续保留 |
| blind retrieval | 估计未见问题检索表现 | ≥60 题 |
| answer factuality | 逐 claim 事实与引用人工评审 | ≥40 题，每题双人 |
| multi-turn context | 指代、纠正、约束累积、长会话 | ≥30 个会话 |
| zero-evidence | 无证据声明、一般知识边界、后续隔离 | ≥20 题 |
| adversarial/robustness | 同义改写、否定、矛盾来源、片段指令、数值公式 | ≥30 题 |

### 7.2 指标分层

不要只给一个综合“准确率”，至少分别报告：

1. **Retrieval**：Recall@k、nDCG@k、MRR、primary-section relevance、Wiki-primary pair completeness。
2. **Context**：reference resolution accuracy、constraint retention、context contamination、correction override accuracy、summary faithfulness。
3. **Answer**：claim factual accuracy、citation correctness、citation coverage、required-unit completeness、boundary correctness、contradiction handling。
4. **Calibration**：有证据时回答率、零证据 abstention/标注正确率、错误高置信表达比例。
5. **Reproducibility**：相同 manifest 重跑的一致率、不同 provider 的 schema pass rate。

建议人工 claim 标注至少包含 `correct / partially-correct / unsupported / contradicted / unverifiable`，两名评审独立标注并报告 Cohen's kappa 或 Krippendorff's alpha；有分歧时裁决。所有比例报告样本数与 95% 置信区间。

## 8. 优化顺序与验收

### 第一阶段：先修测量闭环

1. 将现有 10 题降格为 dev/regression；冻结 blind set；
2. 增加从生产 `prepare_question → build prompt → provider fixture → persist validation` 跑通的 eval runner；
3. 统一静态答案与生产 `[E#]` / answer schema 契约；
4. 保存 `QaRunManifest`；
5. 报告“未测”而不是用 10/10 替代答案准确率。

验收：任意评测答案都能追溯到 provider、具体 model、promptVersion、indexSnapshotId、history IDs 和 evidence checksums。

### 第二阶段：修检索假阳性

1. Wiki 下钻后按当前 query 在目标 paper 内选 section；
2. Gold 标注 section relevance，不再只看 page ID + 行号；
3. query expansion 只在 dev 上调整，在 blind set 上一次性验收；
4. 记录候选 drop reason 与 facet coverage。

验收：blind set 的 primary-section relevance 与 pair completeness 达到预先冻结的阈值，且不靠新增测试题原句触发器实现。

### 第三阶段：上下文管理

1. 引入 `ContextPlan`、token 估算和 output reserve；
2. 保留完整 exchange；
3. 实现结构化 session memory + recent tail；
4. 增加 prefix fingerprint、失效规则和 `/context` 类可视化；
5. 修窄 self-contained 问题的 coreference 触发。

验收：多轮集分别报告 constraint retention ≥预设阈值、contamination ≤预设阈值，并证明 zero-evidence turn 仍为 0 注入。

### 第四阶段：回答效果与 provider 一致性

1. 建立统一 `PromptEnvelope`；
2. 为三类意图建立可执行 completeness schema；
3. 统一 Codex/API 的来源说明、长度与元数据；
4. 增加一次受限引用/结构 repair；
5. 运行双人 claim-level 人工评审。

验收：回答不以字数为目标，而以必需信息单元通过率、事实正确率、引用正确率、边界正确率和 provider schema pass rate 为准。

## 9. 最终判断

当前实现的安全边界和证据结构已经明显优于普通“把检索片段直接塞给模型”的 RAG，但科研准确率的核心短板仍是：**评测链路没有覆盖生产答案，检索用例存在调优耦合，paper 命中不等于 section 相关，上下文与 provider 缺少可复现预算和版本记录。**

下一步最有价值的工作不是继续添加 prompt 形容词，而是先完成：

1. 生产链路 eval runner；
2. 冻结 blind set；
3. query-relevant paper section；
4. `QaRunManifest`；
5. token-aware `ContextPlan`；
6. 三类意图的 completeness schema。

完成这些之后，系统才具备严谨报告“检索准确率、上下文正确率、回答事实准确率”的基础。
