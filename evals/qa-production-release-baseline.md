# QA Production Release Gate 基线

- 检查日期：2026-08-26
- 源提交：`66c5bba3f087a2851553fdfb0d24c2747ae62bd0`
- 冻结门禁：30 项，10 PASS / 20 FAIL
- 最终决策：`FAIL`

## 已验证

- Rust：230 passed / 0 failed / 2 ignored；fmt 与 clippy `-D warnings` PASS。
- Frontend：type-check、全部 57 个 Node contract tests、production build、P3 PASS。
- 数据与库：360 题 seal PASS；Wiki 75 页 lint 0 errors / 1 warning；core-book 295 queries，book Recall@5 门禁 PASS。
- 真实 Cross-Encoder RAG：13/13 PASS，Recall@10/20 = 0.962/1.000，nDCG@10 = 0.851，fallback = 0/13，平均 reranker latency = 15738.0 ms。
- 对抗/可靠性：prompt injection、Provider matrix、planner/embedding/reranker/verifier/Graphify/DB failure injection、20/50/100 message state stress PASS。
- Held-out/release contract：27 个 Python tests PASS；pending held-out 与缺失工件均 fail closed；完整合格 fixture PASS；核心可靠性失败不会 conditional。

## 当前失败原因

- 真实 RAG MRR = 0.821，低于冻结阈值 0.85。
- 独立 held-out 尚未冻结：0/30，不能报告生产 factual precision/Wilson 结论。
- 尚无真实 semantic verifier 生产评测；当前只有 deterministic fallback 与 fake-provider contract。
- 目标机器 performance profile、P95 阈值与对应测量尚未冻结。
- conversation、grounding、open-research 尚无统一生产 metadata envelope 工件。

完整逐项判定见仓库根目录 `QA_PRODUCTION_RELEASE_REPORT.md`。该 FAIL 是外部生产证据与冻结指标未满足的诚实结果，不代表工具链执行失败，也不以 development/regression fixture 替代发布证明。
