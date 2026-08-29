# P1-1 Real Answer Grounding / Generator

## Goal

修正 Real Answer E2E 的 pre-persist/final 判分混淆，并收紧生产 Generator 的 claim-evidence 绑定契约，使 Direct/Research 真实回答尽量只生成当前 EvidenceItem 能完整支持的原子事实。同时为本轮涉及的 QA 生命周期建立不含原文的结构化日志，便于后续定位阶段与失败码。

## Requirements

### R1 — Runner 判分一致性

- 报告 schema 升级为 `qa-real-generator-e2e-report-v2`。
- 每个 case 显式分离 `prePersist` 和可选 `final`，并记录 `persisted`。
- `prePersist` 仅作 Generator/修复器诊断，不把错误永久塞入最终 `errors`。
- 持久化成功时，最终 PASS/FAIL 只使用 persisted `AskResult` 观测值；持久化失败时，case FAIL 并仅记录稳定持久化错误码。
- 报告不得出现未分阶段的 `citationValid=true` 与 `citation_validation_failed` 自相矛盾组合。

### R2 — Grounding Diagnostics

- pre/final 至少记录 grounding/citation/claim/completeness 聚合计数。
- 诊断包含 supported、partially-supported、contradicted、not-verifiable、research-suggestion、repaired 与 uncited-knowledge-fact 计数。
- 每个 claim 仅允记录 claim ID/type/status/evidence ID count/reason code/alignment score/`claimTextSha256`；禁止保存 claim 原文。

### R3 — Generator Grounding Contract

- 每个库内事实使用短句表达，一句一个可核验事实，并在同句紧邻位置附上能支持完整含义的 `[E#]`。
- 证据不足时必须明确说明未覆盖，不得借同段尾部引用支撑整段。
- 禁止将局部扩展到全局、相关性扩展到因果、平均表现扩展到最坏保证、仿真扩展到现实保证、特定参数扩展到任意参数、普通改善扩展为编造数字、单一方法扩展为唯一/最优、论文提出扩展为工业验证。
- 百分比、节点数、时间、距离、能耗、准确率、复杂度数字和参数值必须在 evidence 中逐字可支持。
- Grounded Body 仅放库内证据事实；一般知识/推测只能放入带固定未核验提示的 `## 模型补充（可能不准确）`，且不得带 `[E#]`。
- Research Suggestion 必须使用建议性措辞，不得写成证据已证明的事实。

### R4 — 可追踪日志

- 本轮修改的 prepare/generate/verify/audit/persist/E2E 功能边界均记录结构化事件码、阶段、脱敏 request ID hash、provider/model/mode、聚合数量与稳定错误码。
- 正式桌面端在 debug/release 均启用 `tauri-plugin-log`，文件写入 Tauri `AppLogDir`（软件数据目录下），并限制单文件大小与保留数量。
- 开发 E2E CLI 的追踪文件位于 `apps/desktop/logs/`，并被 Git 忽略；报告仍保持 metadata-only。
- 日志不得包含 question/answer/prompt/claim text/evidence snippet/repository 绝对路径/临时路径/credential/token/provider payload/chain-of-thought。

### R5 — 严格不变范围

- 不修改 Semantic call budget、`8k/18k/32k` ceiling、Planner 策略、Research State/alias、Zero-evidence completeness、Verifier/Atomic Claim 标准、Frozen Threshold 或 E2E case。
- 不读取、运行或修改 Independent Heldout。
- 不新增平行 Generator，不依赖 AnswerRepair 掩盖大量首轮错误。

## Acceptance Criteria

- [ ] AC1：Runner v2 报告明确区分 prePersist/final/persisted，最终错误不混入旧中间态。
- [ ] AC2：聚合与 claim-hash 诊断完整，不序列化原文或敏感路径。
- [ ] AC3：Natural v2 Prompt 明确覆盖原子事实、紧邻引用、范围/因果/数字限制、Supplement 隔离和 Suggestion 标记。
- [ ] AC4：确定性 tests 覆盖 Prompt 契约以及 supported/scope expansion/numeric hallucination/unsupported clause/supplement isolation。
- [ ] AC5：新增功能阶段有可追踪结构化日志，正式日志位于 AppLogDir，E2E 日志位于软件目录且不被 Git 追踪。
- [ ] AC6：Direct 真实重测给出 persisted/grounding/supported/contradicted/notVerifiable/repaired 数据。
- [ ] AC7：Direct 改善后再单独重测 Research，仅评估 Grounding，不处理 Planner fallback。
- [ ] AC8：fmt/clippy/QA tests/build 通过；每个阶段由 Git commit 保存。
- [ ] AC9：未触及 R5 列出的任何禁止项。

## Out of Scope

- P1-2 Semantic Verifier Call Budget、P1-3 Planner fallback、State alias、Zero-evidence completeness、性能优化、Heldout 与 release threshold 调整。
