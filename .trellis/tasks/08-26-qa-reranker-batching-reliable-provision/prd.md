# Cross-Encoder 批量重排、分数融合与可取消可靠部署

## Goal

完成阶段 C 的剩余生产化工作：在不更换 `BAAI/bge-reranker-base`、不通过缩减候选集掩盖性能问题的前提下，优化真实 Cross-Encoder 批量推理；使用基础检索分数与 Cross-Encoder 分数融合避免排序退化；将 reranker 下载/修复改造成可取消、可观测、校验后原子提交且可故障恢复的显式部署流程。

## Background

- 真实模型已在非系统盘部署并通过 health probe，ONNX SHA-256 为 `15b9a8c3da82eddf263df571281166e00e9308fe19d077084b642ebfcaf06d2b`。
- 当前真实 RAG benchmark 为 12/13 PASS、fallback 0/13，平均 reranker latency 38101.1 ms；`dwpt-beb-planning` Top20 未覆盖全部 expected documents。
- `rerank_texts` 已向 FastEmbed 传入 `Some(16)`；FastEmbed 4.9.1 内部使用 `par_chunks(batch_size)`。本任务必须基于真实测量优化 batch 策略，不能把已有行为重新包装成“新增批处理”。
- 当前 `fuse_rankings` 使用基础 rank 与 Cross-Encoder rank 的 RRF，但 Cross-Encoder 权重为 1.5，并覆盖原始候选分数；需要改为可审计的分数融合，降低单一模型对基础检索结果的过度改写。
- 当前 reranker repair 直接调用 FastEmbed 自动下载，没有取消入口、下载回调、固定文件 manifest、`.part` 生命周期或底层错误保留。

## Requirements

### R1 — Measured batched inference

- 保持单个已加载 `TextRerank` session 复用，不为每个候选或每个 batch 重新初始化模型。
- 明确 batch 配置与边界，避免 Rayon 外层并行 batch 与 ONNX Runtime 全核 intra-op 造成线程过度订阅。
- 用当前真实模型在同一机器比较候选规模与 batch size，选择有测量依据的默认值；记录 candidate count、batch size、batch count 与 inference latency。
- 不通过减少 `SEMANTIC_RERANK_LIMIT` 或删除候选来获得表面性能改善。

### R2 — Base/Cross-Encoder score fusion

- 保留基础检索/确定性重排分数，并与 Cross-Encoder 分数经过稳定归一化后融合；不得让 Cross-Encoder 原始 logit 直接覆盖基础排序。
- 显式来源、reference/graph/primary-fallback 等既有保护语义继续生效，融合公式与权重写入 telemetry/retrieval reason。
- 不为 `dwpt-beb-planning` 增加专门回归用例或硬编码；使用现有 13 项 RAG suite 验证融合结果。

### R3 — Cancellable reranker provisioning

- reranker 下载/修复具有 request-scoped cancellation token 和独立取消命令。
- 取消在文件块边界检查，返回稳定 `RERANKER_DEPLOYMENT_CANCELLED`，不标记为损坏、不覆盖旧的 ready 模型。
- 设置页在部署期间显示取消入口；重复点击修复不能创建并发写入同一目标的任务。

### R4 — Real download progress

- 下载回调报告真实 `downloadedBytes`、`totalBytes`、`bytesPerSecond` 与 `percent`，并携带 phase/file/status/message。
- 前端消费 typed Channel 事件；未知总量时不得伪造百分比，完成事件必须与最终文件大小一致。

### R5 — Verified atomic artifact commit

- 固定 repo revision、相对路径、文件大小与 SHA-256 manifest，至少覆盖 ONNX 与四个 tokenizer/config 文件。
- 每个文件写入同目录 `.part`，流式计算/最终复核 SHA-256；仅校验通过后原子 rename 到最终路径。
- 校验失败保留稳定诊断并隔离损坏 final；不得以未校验文件进入 `ready`。
- query-time 保持严格 no-download，模型及向量数据继续使用用户配置的非系统盘缓存目录。

### R6 — Failure and idempotency coverage

- 覆盖磁盘空间不足、网络中断、损坏下载、损坏既有文件、取消以及重复 repair。
- 网络中断/取消留下可识别 partial 状态；再次 repair 可以安全续传或重新下载。
- ready 状态重复 repair 只执行完整性与 health 检查，不重复下载大文件。
- 所有失败均不得 panic、不得破坏旧 ready snapshot，并保留可审计错误类别。

## Acceptance Criteria

- [x] AC1：真实模型路径使用单 session 批量推理，并输出 batch/candidate/latency telemetry；同机平均 latency 37105.6 ms，低于 38101.1 ms 基线。
- [x] AC2：现有 13 项 RAG suite 达到 13/13 PASS，Cross-Encoder fallback rate 保持 0.000；未新增 `dwpt-beb-planning` 专项回归用例。
- [x] AC3：下载期间可取消，取消结果稳定、UI 可见，旧 ready 模型不受影响。
- [x] AC4：进度事件的字节数、总量、速度和百分比来自实际流式下载，完成值与落盘文件一致。
- [x] AC5：所有 manifest 文件遵循 `.part → SHA-256/size 校验 → 原子 rename`，损坏文件不能进入 ready。
- [x] AC6：磁盘不足、网络中断、损坏文件与重复 repair 测试全部通过，没有 panic、并发覆盖或旧模型破坏。
- [x] AC7：query-time no-download 与非系统盘缓存配置保持不变。
- [x] AC8：每个实施阶段均单独本地 Git commit；未提交模型文件、未修改用户已有未跟踪文件、未上传 GitHub。

## Out of Scope

- 更换或量化 Cross-Encoder 模型。
- 为性能缩减候选数量或降低检索覆盖面。
- 新增 `dwpt-beb-planning` 专项回归数据或针对该用例硬编码。
- 降低父任务冻结的 release thresholds。
- 上传 GitHub。
