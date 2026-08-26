# Cross-Encoder Production Deployment 基线

- 检查日期：2026-08-26
- 目标模型：`BAAI/bge-reranker-base`
- 模型版本：`fastembed-4.9.1-bge-reranker-base`
- 默认部署目录：`%LOCALAPPDATA%/LunaWiki/fastembed/reranker-bge-base`
- 当前机器配置目录：`E:/知识库/语义模型/reranker-bge-base`（模型与向量数据均位于非系统盘）
- 初始状态：`missing`；首次直连 Hugging Face 下载因 `os error 10060` 超时失败
- 重新 provision：`PASS`；通过当前系统代理续传并完成离线落盘
- ONNX SHA-256：`15b9a8c3da82eddf263df571281166e00e9308fe19d077084b642ebfcaf06d2b`
- 真实模型 health probe：`PASS`（ignored production health test 1/1）
- Query-time download：禁用；缺失时保持 EmbeddingRescorer → Deterministic fallback。

## 当前真实 RAG 运行

- 用例：12/13 PASS，整体状态 `REVIEW`
- Document Recall@5/10/20：0.885 / 0.885 / 0.962
- MRR / nDCG@10：0.789 / 0.782
- Reranker fallback：0/13（0.000）
- 平均 reranker latency：38101.1 ms
- 剩余失败：`dwpt-beb-planning` 的 Top20 未覆盖全部 expected documents

## Production Gate 判定

Cross-Encoder 部署与 fallback 子门禁为 `PASS`：真实模型成功加载并完成 12 个实际重排用例，fallback rate 0.000 低于冻结上限 0.05。

整体 Production Gate 仍为 `FAIL/REVIEW`：RAG 回归仅 12/13 通过，平均重排耗时约 38.1 秒，且目标机器 performance 阈值尚未冻结。后续需要修复 `dwpt-beb-planning` 排序退化、优化 Cross-Encoder 候选批量与延迟，并补齐长任务取消、真实进度和 failure injection。
