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
- Provisioning：固定 revision/size/SHA-256 manifest；支持 `.part` 续传、真实字节进度、取消、原子提交与重复 repair 离线跳过。
- Inference：单 session 进程内复用；稳定去重后按 ExecutionMode 截断候选并单批执行，parent context 只在 rerank 后扩展；manifest 记录 candidate/batch/max length、model load/input prepare/inference 与平均输入 token。
- Query-time download：禁用；缺失时保持 EmbeddingRescorer → Deterministic fallback。

## 当前真实 RAG 运行

- 用例：13/13 PASS
- Document Recall@5/10/20：1.000 / 1.000 / 1.000
- Canonical Document MRR / Passage MRR / nDCG@10：1.000 / 1.000 / 0.955
- Reranker fallback：0/13（0.000）
- 平均 reranker latency：8239.2 ms；平均 model load/input prepare/inference：221.0 / 1.2 / 8014.6 ms；平均输入 176.1 tokens。
- `dwpt-beb-planning`：通过统一 score fusion 与 document-repeat penalty 恢复覆盖；没有新增专项用例或路径/标题特判。

## Production Gate 判定

Cross-Encoder 部署、完整性、可靠性、检索质量与 fallback 子门禁为 `PASS`：真实模型成功加载，13/13 RAG 用例通过，Canonical Document MRR 1.000 高于冻结下限 0.85，fallback rate 0.000 低于冻结上限 0.05。

目标机器 performance profile 与各模式 SLO 已在测量前冻结并全部通过，真实 semantic verifier 也已完成。整体 Production Gate 仍不据此宣告 Production Ready：独立 held-out、Grounding 与 Open Research 人工证据尚未提供，统一 release gate 因此保持 FAIL。
