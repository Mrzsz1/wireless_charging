# 真实回答生成 E2E Runner

## Goal

新增一个小规模、可重复执行的 Real Answer Generator E2E 入口，使用公开 Development/Regression/Synthetic case、真实知识库、真实本地 retrieval/reranker、真实 Codex subscription generator 与 Semantic Verifier，证明桌面 UI 使用的生产 QA Core 能生成最终可审计回答，而不是只验证 retrieval 排名。

## Requirements

### R1 — 生产链复用

- Runner 必须复用 UI `ask_luna` 使用的真实 prepare → generate → verify → audit 核心。
- Generator 必须最终调用同一 `codex_subscription::stream_answer`，并使用同一 `LlmBudgetGuard` reserve/settle 路径。
- 不得新增 `codex exec` 旁路、mock generator 或另一套问答算法。
- 如需抽取公共函数，只允许行为保持型最小重构；Tauri Channel、AppState、窗口事件与 UI persistence 继续留在适配层。

### R2 — 五类公开用例

- Direct 普通问答。
- Research 方法改进。
- Exploratory 多移动充电器 + deadline 的多类方法研究。
- Multi-turn，验证 Objective/Constraint/Parameter/Excluded Method。
- Zero-evidence，验证不伪造知识库证据。
- Case 只能来自 Development/Regression/Synthetic；禁止 Independent Held-out。

### R3 — 隔离

- 使用真实 repository 内容和临时 SQLite 测试数据库。
- 临时数据库内允许持久化多轮会话，以复用 production persistence/history；运行结束后由 RAII 删除。
- 不向正式 App 会话数据库写入 TEST session。
- 报告不得包含完整问题答案、prompt、chain-of-thought、绝对临时路径或凭据。

### R4 — 自动验证

- 最终 Natural Markdown answer 非空，但不做全文一致断言。
- provider 必须为 `codex-subscription`；modelResolved 非空且不是 `deterministic`。
- `routingLlmStages` 必须包含 `generator`；不得出现 generator token-budget rejection。
- 所有结构化引用必须属于当前 EvidenceItem；unknown citation 数为 0。
- Citation Validation 必须满足对应 evidence/zero-evidence 契约。
- Semantic Verifier 必须真实成功，或明确报告合法 unavailable 状态与非空原因；不得伪装成功。
- QaRunManifest 必须完整，Research/Exploratory 需校验 mode、planner、round/evidence telemetry。
- Multi-turn 需校验 state 目标、约束、参数和 excluded method。

### R5 — 命令与报告

- 新增 `npm run eval:qa-real-e2e`。
- 默认运行 5 个逻辑 case；multi-turn 可包含多次真实 generator invocation。
- 生成 `evals/reports/qa-real-generator-e2e-report.json`，只记录 case ID、PASS/FAIL、mode、provider/model、evidence count、citation/semantic/budget 状态和聚合结果。
- 失败进程返回非零；报告使用 `.part` 后原子替换。

### R6 — 不可变约束

- 不修改 QA 算法、Prompt、Frozen Threshold、8k/18k/32k Token Ceiling、性能配置或 natural answer 用户可见行为。
- 不读取、运行、分析或修改 Independent Held-out。

## Acceptance Criteria

- [x] AC1：`eval:qa-real-e2e` 使用 UI 同源 QA Core 和真实 `codex-subscription`。
- [x] AC2：Direct/Research/Exploratory/Multi-turn/Zero-evidence 五类 case 全部执行。
- [x] AC3：正式 App 数据库无测试会话污染，临时数据库运行后删除。
- [x] AC4：报告不含完整答案或敏感数据，且以稳定 schema 输出。
- [x] AC5：provider/model/generator stage/budget/citation/semantic/manifest 契约全部自动验证。
- [x] AC6：针对 runner schema、报告脱敏、临时数据库隔离和 UI/core 同源接线增加 deterministic regression tests。
- [x] AC7：Rust QA tests、fmt、clippy、frontend script wiring/build 通过。
- [x] AC8：真实 5-case E2E 给出明确 PASS/FAIL；provider 环境问题不得伪装为代码 PASS。
- [x] AC9：未使用 Independent Held-out，未修改 Frozen Threshold、Token Ceiling、Prompt 或 QA 算法。

## Measured Result

- 2026-08-29 完整执行 5 个公开 case，真实 Provider 为 `codex-subscription`，模型为 `gpt-5.6-luna`，真实 generator 调用 5 次。
- 结果为明确 `FAIL`（0/5）；该结果是 QA 生产门禁失败，不是 Provider 未运行或 Runner 伪 PASS。
- 结构化引用 ID 无 unknown，generator fallback=0，generator budget rejection=0，真实 reranker 无 fallback。
- 已确认的后续产品缺口是 grounding/claim verification 失败、Direct/Multi-turn 语义验证预算不足、Exploratory 完整性/状态保留失败、Zero-evidence 完整性失败。这些修复超出本 Runner 任务的不可变范围。

## Out of Scope

- 大批量 benchmark、性能优化、heldout、UI 改造、回答全文快照、发布门槛调整。
