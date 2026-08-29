# P1-1 Real Answer Grounding / Generator 技术设计

## Data Flow

```text
QuestionContext
  -> build_prompt_envelope (hardened grounding contract)
  -> production generator
  -> semantic + deterministic claim verification
  -> prePersist AnswerAudit
  -> production persistence gate
       success -> final AskResult observation
       failure -> persisted=false + stable persistence error
  -> metadata-only report v2
```

`prePersist` 和 `final` 使用同一个纯投影函数，但两者语义不同：前者诊断生成/修复阶段，后者代表持久化门禁接受后用户最终结果。

## Runner v2 Schema

- `RealE2eCaseResult.persisted: bool`
- `prePersist: GroundingObservation`
- `final: Option<GroundingObservation>`
- `GroundingObservation` 含 provider/model/mode/evidence/citation/grounding/completeness/claim 聚合计数、budget/planner/reranker telemetry 和不含原文的 `claims[]`。
- `ClaimDiagnostic` 仅包含 `claimId/claimType/verificationStatus/evidenceIdCount/reasonCode/alignmentScore/claimTextSha256`。
- 持久化失败时 `final=null`，`errors=[stable persistence code]`，prePersist 字段仍可用于定位，但不重复生成最终错误。

## Final Verdict

1. prepare/generate 无法完成：case FAIL，稳定运行错误码。
2. persistence 失败：case FAIL，`persisted=false`，`final=null`。
3. persistence 成功：对 `final` 执行 provider/model/generator/budget/citation/semantic/manifest/mode/state 契约验证。
4. multi-turn 只在最后轮完成后验证最终 state。

## Prompt Boundary

Grounding 约束继续属于 provider-neutral `answer_contract`，Codex 和 Compatible API 共享，不在 provider adapter 中复制。不改 verifier 或 repair 判定，只减少 Generator 首轮生成超出 evidence 语义的 claim。

## Structured Logging

- 新增单一 `qa::trace` owner，定义允记录的 typed metadata，统一输出 `qa_trace` 结构化 JSON 事件。
- request ID 仅记录 SHA-256 短 hash；错误仅记录 `safe_error_code`。
- Desktop 由 `tauri-plugin-log` 在所有 build 中写入 `AppLogDir`，旋转策略为有界保留。
- E2E CLI 为同一 typed event 投影配置 `apps/desktop/logs/qa-real-e2e.jsonl`；目录仅保存运行日志并被 Git 忽略。
- 不记录 prompt/answer/question/claim/evidence 原文，也不记录仓库或临时路径。

## Compatibility and Rollback

Tauri command signature、UI event order、QA answer rendering、persistence schema、budgets、planner/state/verifier 保持不变。Runner report 是开发评测 schema 升级，不与 heldout/release report 混用。
