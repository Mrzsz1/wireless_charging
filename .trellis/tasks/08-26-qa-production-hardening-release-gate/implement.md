# Implementation Plan — QA Production Hardening & Final Release Gate

## Phase A — Atomic Claim Extraction

- [x] 新增 AtomicClaim、Unverified/Unavailable 状态与 extractor/mapping/typing 分层。
- [x] 实现 citation-aware guarded clause segmentation，保持 Markdown 与 Evidence ID 契约。
- [x] 新增 `evals/atomic_claim_cases.json`，不少于 50 个 regression/adversarial cases。
- [x] 更新 grounding/claim spec 与 manifest schema。
- [x] 运行 targeted Rust tests、fmt、clippy。
- [x] 本地提交：`fix(qa): extract atomic evidence claims`。

## Phase B — Semantic VerificationProvider

- [x] 为 Provider capability 增加 semantic verification，共用 Codex/API structured transport。
- [x] 实现 batch prompt/schema/parser、SemanticEntailment、合并矩阵与逐 Claim fallback。
- [x] 将 verifier call 接入 request-scoped LlmBudgetGuard，Provider 调用位于 repository write lock 之前且 audit 不会重复调用 Provider。
- [x] 增加 timeout/invalid JSON/budget/unavailable/contradiction/scope/causality regression。
- [x] 扩展 manifest/UI-safe telemetry，semantic 与 heuristic checked 分离。
- [x] 本地提交：`feat(qa): add semantic claim verification`。

## Phase C — Cross-Encoder Production Lifecycle

- [ ] 固定模型 manifest/version/file integrity，增加 offline status/health DTO。
- [ ] 增加显式 provision/repair/cancel/progress 命令；query-time 保持 no-download。
- [ ] 增加 missing/partial/corrupt/ready/repeated/offline/repair tests。
- [ ] 扩展 reranker telemetry 与真实模型 benchmark/report schema。
- [ ] 在当前主机尝试真实 provision + benchmark；无环境时保存明确未满足结果。
- [ ] 本地提交：`feat(qa): provision production reranker model`。

## Phase D — Adversarial, Failure, Stress and Metadata

- [ ] 增加 prompt-injection/untrusted-evidence contract 与 regression。
- [ ] 补齐 planner/embedding/reranker/verifier/graph/DB failure injection。
- [ ] 增加 20/50/100 轮 ResearchSessionState stress 和 provider matrix tests。
- [ ] 增加统一 eval metadata envelope、canonical dataset/runtime hashes 与安全字段校验。
- [ ] 增加 deterministic PR CI 与 real-model RC workflow。
- [ ] 本地提交：`test(qa): harden production failure paths`。

## Phase E — Held-out Harness, Release Gate and Report

- [ ] 扩展 held-out schema/metrics：partial/unsupported/not-applicable、citation completeness、reference/method/constraint 指标。
- [ ] 保留双 reviewer、第三人 adjudication、checksum/Wilson CI fail-closed contract。
- [ ] 新增冻结 release thresholds 与 `check_qa_release_gate.py`，输出逐项 PASS/FAIL reason。
- [ ] 新增 artifact collector 与 `QA_PRODUCTION_RELEASE_REPORT.md` generator。
- [ ] contract tests 覆盖当前真实 FAIL、完整合格 fixture PASS、核心可靠性不允许 conditional。
- [ ] 本地提交：`feat(qa): automate production release gate`。

## Phase F — Full Integration and Final Audit

- [ ] `cargo fmt --check`。
- [ ] 全量 Rust tests + clippy `-D warnings`。
- [ ] frontend typecheck/tests/build、P3、RAG、360 questions、Wiki/core-book gates。
- [ ] atomic/semantic/reranker/held-out/release-gate 全部专项验证。
- [ ] 生成当前 commit 对应的生产 release report；缺失外部生产证据时保持 FAIL 并列出原因。
- [ ] 更新 `.trellis/spec/backend/qa-contract.md`、eval baselines 和任务验收清单，不降低阈值。
- [ ] 本地提交：`docs(qa): record production release decision`。

## Risky Files / Rollback Points

- `apps/desktop/src-tauri/src/qa/{claim_verification,grounding,provider_capabilities,semantic,reranker,context}.rs`
- `apps/desktop/src-tauri/src/{qa.rs,lib.rs}`
- `apps/desktop/src/types.ts` 与 QA audit UI（仅 telemetry projection，不做 UI 重构）
- `tools/qa_accuracy_eval.py`、新增 release/eval scripts、`evals/**`
- `.github/workflows/**`、`.trellis/spec/backend/qa-contract.md`

## Validation Invariants

- 任何 heuristic-only 路径：`entailmentChecked=false`。
- 任何无 Evidence factual Claim：不得 Supported。
- 任何真实模型缺失：release gate 失败，不以 fixture 替代。
- 任何未独立冻结 held-out：release gate 失败，不生成虚假精度/Wilson CI。
- 所有用户已有未跟踪文件保持未修改、未暂存、未提交。
