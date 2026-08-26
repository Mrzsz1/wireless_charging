# Unified Production Eval Harness 基线

- 检查日期：2026-08-26
- Source commit：`0b28fd0eeccf89535821b3d2c40244c5c8c6e41c`
- Harness：`tools/qa_production_eval.py`
- Output：`evals/releases/<git-sha>/`
- Release Gate：15/30 PASS，最终 `FAIL`

## 已自动生成并通过

- Retrieval：Recall@20 1.000、Recall@10 0.962、Canonical Document MRR 0.962、nDCG@10 0.851。
- Conversation：50 cases；reference/constraint/objective = 1.000/1.000/1.000。
- Reliability：planner/provider/DB contracts PASS；crash 0、handled/fallback 1.000、invalid verified 0。
- Reranker：真实 `BAAI/bge-reranker-base`，fallback 0。
- Semantic Verifier：真实 Codex `gpt-5.6-luna`，100/100 completed，invalid verified 0。

## 仍保持 FAIL

- Canonical independent held-out 尚未提供，因此 heldout/grounding/open-research 工件缺失。
- Target performance profile 尚未 sealed，cold/warm P95 尚未测量。

所有现有 artifact 数值由 machine reports 解析并通过 `qa-eval-metadata-v1` 包装；collector
拒绝 raw question/answer/prompt/credential 字段和非有限数，不允许人工填入生产通过值。
