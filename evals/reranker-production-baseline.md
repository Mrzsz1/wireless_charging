# Cross-Encoder Production Deployment 基线

- 检查日期：2026-08-26
- 目标模型：`BAAI/bge-reranker-base`
- 模型版本：`fastembed-4.9.1-bge-reranker-base`
- 默认部署目录：`%LOCALAPPDATA%/LunaWiki/fastembed/reranker-bge-base`
- 初始状态：`missing`
- 显式 production provision 实测：`FAIL`（`RERANKER_DEPLOYMENT_FAILED: download_or_initialization`）
- Query-time download：禁用；缺失时保持 EmbeddingRescorer → Deterministic fallback。

## 当前真实 RAG 运行

- 用例：13/13 regression PASS
- Document Recall@5/10/20：0.808 / 0.885 / 1.000
- MRR / nDCG@10：0.811 / 0.798
- Reranker fallback：12/13（0.923）
- 平均 reranker latency：13.8 ms

## Production Gate 判定

`FAIL`。真实 Cross-Encoder 尚未成功部署，fallback rate 0.923 高于冻结上限 0.05；当前指标属于 fallback 路径，不能作为 Cross-Encoder before/after 质量证明。

代码已提供离线 status/health、显式 provision/repair、Hugging Face snapshot 发现、模型版本与 UI 入口。修复外部下载/初始化环境后，必须重新运行 ignored real-model health test 与 RAG benchmark，再生成 before/after artifact。
