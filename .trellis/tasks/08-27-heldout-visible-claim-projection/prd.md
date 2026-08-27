# 修复 Held-out 可见 claim 投影基础设施

## Goal

修复 Independent Held-out Runner 在 `natural-markdown-v2` 最终渲染后错误使用 pre-render claim 文本导致的确定性 `HELDOUT_AUDIT_INVALID: claim_projection`，确保导出的 `answerClaims` 精确代表最终用户可见文本，同时保留结构化 provenance。

## Requirements

- 修改范围仅限 Independent Held-out audit/export infrastructure 与其共享的纯文本投影函数、Python 独立审计校验、测试和契约文档。
- 禁止改变 Retrieval、Query Planner、Prompt、State、Reranker、Embedding、Answer Generation、Semantic Verifier 判断策略、`natural_answer_v2` 用户可见输出和 frozen dataset。
- 从 `runManifest.claimVerifications` 的 pre-render claim 确定性投影出最终可见 claim；不得调用第二个 LLM、重新生成 claim 或修改答案。
- 可见投影必须复用 natural answer renderer 的同一 canonical transformation，覆盖 evidence token 移除、unsafe Markdown link sanitation、Windows absolute-path redaction、appendix boundary 和 trim。
- `answerClaims.text` 必须非空且逐字存在于最终 `audit.answer`。
- `answerClaims.citedEvidenceIds` 原样保留 manifest 的 `evidenceIds`，不要求最终 claim 文本含 `[E#]`。
- Rust runner 与 Python evaluator 均须独立校验所有 cited evidence ID 存在于当前完整 EvidenceItem 集合；不得只信任另一侧。
- heldout run schema 升级为 `qa-heldout-run-v2`，明确结构化 provenance 与可见文本分离。
- 旧 `.part` run 作为 `invalid_harness_run` 保留，不读取其中答案、不覆盖、不删除、不视为成功 run。
- 修复后使用未改动的同一 frozen 40-case dataset 完整重跑，不得只重跑失败 case、重新 curator 或重新选题。

## Acceptance Criteria

- [x] `Synthetic claim [E1]` 投影为 `Synthetic claim`，最终回答包含该文本，导出 `citedEvidenceIds=["E1"]` 并通过。
- [x] projected claim 不存在最终 answer 时失败。
- [x] unknown `E99`、duplicate claim ID、empty projected claim、evidence checksum tampering 均失败。
- [x] Python evaluator 独立拒绝 unknown citedEvidenceId，同时仍要求 claim text 逐字存在于最终 answer。
- [x] natural answer renderer 与 heldout projection 的链接清理、Windows 路径隐藏和 evidence-token 移除使用同一纯函数；现有最终用户可见输出回归保持不变。
- [x] 聚焦 Rust/Python 测试全部通过，公开 schema/spec 已同步。
- [ ] 旧失败 `.part` 被登记为 `invalid_harness_run`，新的 40-case run 从零开始并完整结束。

## Notes

- frozen heldout dataset 为只读输入，本任务不修改其任何字段或 seal。
- 各阶段使用本地 Git commit 保存；未收到上传指令前不推送 GitHub。
- 完整运行已从第 1 题重新开始，visible-claim audit 连续通过 20 个 case；随后由独立生产门禁 `LLM_BUDGET_EXCEEDED: generator:token_budget` 终止。该 partial 已原样保存为 `incomplete_runtime_failure`，未读取答案、未删除、未提升为成功 run。完成 40/40 需要用户另行允许处理被本任务明确禁止修改的生产生成预算。
