# 2026-08-24 科研 RAG 评测、迁移与发布验收

## 范围

- 子任务：`.trellis/tasks/08-18-rag-evaluation-rollout`
- 父任务：`.trellis/tasks/08-18-md-hybrid-agentic-rag`
- 实现提交：`6fbe58f`（评测器与检索修正）、`40b26a2`（GUI 冷启动冒烟预算）。

## 可执行评测

- 新增 `qa-rag-evaluation-cases-v1` 严格用例契约和 Rust `rag-eval` CLI，使用真实 Markdown 在内存 SQLite 中重建索引。
- 13 个用例覆盖指定书、开放 paper+book、中英改写、新概念、多轮指代、reference-only 边界和真零证据。
- `npm run eval:rag`：13/13 PASS；source resolution 1.000；channel attempt 1.000；文档 Recall@5/10/20 = 0.808/0.885/1.000；heading Recall@20 = 1.000；MRR/nDCG@10 = 0.811/0.798；locator validity = 1.000；zero-evidence FN/FP = 0/0；平均 59.1 ms。
- 双读报告逐例列出 legacy/v2 改善和退化；当前没有用单一百分比宣称总体事实准确率。

## 通用检索修正

- 显式来源无法解析时以 `unresolved_explicit_source` 失败关闭，不再回退到开放范围返回无关文档。
- 指定来源的正文查询移除已解析标题和仅用于指代的历史实体，避免标题 token 淹没目标小节。
- 同文档同小节的 section/semantic 命中按稳定身份去重。
- reranker 失效时保留 RRF 融合候选并记录降级；双语重排使用契约 concepts/aliases/related problems/facets，未增加“移动路径”或“波干扰”专用特判。

## 迁移和回滚证据

- 旧库备份位于本机临时目录 `wc-rag-rollout-20260824-221550`，同时保存语义缓存 manifest；它们不进入 Git。
- 备份抽样计数：chat sessions/messages/evidence = 6/14/55；documents/content blocks = 98/6330。
- 迁移回归在重建前后比较 session/message/evidence 计数并保留旧 evidence payload。知识索引与用户会话表独立重建。
- 本地多语模型实际探针通过；旧 LUNAVEC1 与语义模型目录保留为回滚副本。pgvector snapshot 切换/清理通过脱敏 adapter 回归，未使用真实凭据。

## 质量门和编译

- Rust：`cargo fmt --check` PASS；`cargo clippy --all-targets -- -D warnings` PASS；`cargo test --lib` 167 PASS / 1 ignored；部署模型 ignored probe 单独 1 PASS。
- Python：`py -3 -m unittest discover -s tests` 63 PASS；`wiki_eval.py` PASS。
- 前端：QA evidence 5 PASS；QA settings 8 PASS；`npm run build` PASS；`npm run verify:p3` PASS。
- 发布：`cargo build --release` PASS；`npm run tauri build` PASS，产生 MSI 和 NSIS；`npm run verify:p5:strict` 在显式脱敏临时安装路径下 PASS，覆盖 GUI 和 install/launch/uninstall。
- GUI 冷启动研究脉络在语义初始化时可超过 15 秒；冒烟测试改为先等待 FTS 完成，再在 180 秒冷启动预算内等待成功或明确错误，不再把慢初始化误判为功能失败。

## 发布产物 SHA-256

- `app.exe`：`A09510842B47E2EC0ECD7638D4922A1D6D41A7D28CE0B235BE0AA5ADB5C553EA`
- MSI：`36A5AB1E703FF87996AB2701C891695A8C1FD74C09E7DFB30DA4924021E38246`
- NSIS：`CF379F612BCBBCF18F056C8A67202631CD0194A18CFD9F10C2D3AA38BBCBFF03`
- 评测 baseline：`353E1BF5C170AF5BDD6F008FCEBA29F729A9F9212B29463031928FC3B847997A`

## 剩余边界

- 开发集中 `obstacle-anchor-projection` 和 `dwpt-beb-planning` 的 Recall@5 仍为 0，但均在 Recall@20 命中且 locator 有效；这是下一轮排序优化点，不应通过专用关键词修补。
- 实际库已有 v2 文档/块索引，但 `embedding_records_v2` 当前为 0；功能和增量复用已通过测试，真实库的首次 v2 向量同步仍需用户在应用中执行。
- 本评测不包含默认外搜、语义蕴含评分或公开 benchmark 结论。
