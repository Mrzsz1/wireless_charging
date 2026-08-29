# P1-1 Real Answer Grounding / Generator 实施计划

## Phase A — Runner v2 与日志基础

- [ ] 先写 Runner 回归：prePersist 失败不污染 final errors，persistence failure 使 `persisted=false/final=null`。
- [ ] 实现 v2 `GroundingObservation`/`ClaimDiagnostic` 投影与聚合计数。
- [ ] 新增 typed `qa::trace` 事件、E2E 软件目录日志 sink 与脱敏/清理 tests。
- [ ] 将 Desktop log plugin 改为 debug/release 常驻b的 AppLogDir 有界旋转配置。
- [ ] 执行 targeted tests，Git 提交 Phase A。

## Phase B — Generator Prompt Grounding

- [ ] 先扩展 Prompt contract test，不做整段 snapshot。
- [ ] 收紧 provider-neutral natural v2 `answer_contract`，覆盖原子事实、紧邻引用、范围/因果/数字扩张、证据不足、Supplement 隔离、Suggestion 标记。
- [ ] 新增 5 类 deterministic grounding fixture tests，不改 verifier threshold。
- [ ] 在 prepare/generate/verify/audit/persist 关键边界接入 safe structured trace。
- [ ] 执行 targeted tests，Git 提交 Phase B。

## Phase C — 真实重测

- [ ] 只运行 `QA_REAL_E2E_CASE_ID=real-direct-rose` 一次，审计 persisted/grounding/claim 计数与日志脱敏。
- [ ] Direct 明显改善后，只运行 `real-research-improvement` 一次；保留 Planner fallback 现状。
- [ ] 若 Direct 未改善，仅做一次有证据的 Prompt 修正后重试，不扩展到其他 P1 问题。
- [ ] 提交 metadata-only v2 报告，Git 保存 Phase C。

## Phase D — Quality and Finish

- [ ] `cargo fmt --check`。
- [ ] `cargo clippy --lib --bins -- -D warnings`。
- [ ] Rust QA/Runner/trace tests。
- [ ] `npm run build`。
- [ ] 检查日志与报告不含 question/answer/prompt/claim/path/secret。
- [ ] 更新 QA/logging spec、验收项、Git 提交、归档并记录 journal。

## Guardrails

- 不运行 heldout。
- 不修改 Semantic call budget、Token Ceiling、Planner、Research State/alias、Zero-evidence completeness、Verifier threshold 或 Frozen Threshold。
- 不修改现有 5-case fixture 的问题或选题。
- 真实模型每个指定 case 一次，只在有明确根因时最多重试一次。
