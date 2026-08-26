# Technical Design — Cross-Encoder 批量重排与可靠部署

## Scope and boundaries

本任务只修改 Cross-Encoder inference、ranking fusion 与 reranker provisioning。Retriever、RRF 通道召回、候选预算、语义 embedding 模型和 release threshold 均保持原契约。

主要边界：

- `qa/retrieval.rs`：继续构造 `HybridResearchReranker`，只传递 request cancellation 与 telemetry。
- `qa/reranker.rs`：拥有基础分数与 Cross-Encoder 分数融合，不做下载。
- `qa/semantic.rs`：拥有模型 session、批量推理、artifact manifest、流式下载、校验、原子提交与 deployment state。
- `lib.rs`：拥有 Tauri request 生命周期、取消 token、Channel 转发和并发任务互斥。
- `desktop.ts` / `SettingsView.tsx`：typed progress、取消按钮与部署状态展示。

## Batched inference

当前 FastEmbed 已按 `batch_size=16` 使用 `par_chunks`，而 ONNX session 又配置为使用可用并行度。实现阶段先运行 batch matrix，确认 8/16/32/全候选在当前机器的耗时，避免外层 batch 并发与 ONNX intra-op 线程过度订阅。

生产路径维持一个缓存 session。最终 batch 配置由真实测量选定并以常量/配置结构表达；每次运行记录：

```text
candidate_count
batch_size
batch_count
model_max_length
inference_latency_ms
```

不得为了降低耗时减少候选数。若 FastEmbed 内部并行策略仍导致过度订阅，可在应用层串行控制 batch，或让单个较大 batch 进入一次 session run；选择以同机 benchmark 为准。

## Score fusion

Cross-Encoder 输入前冻结每个候选的 base score 与 base rank。Cross-Encoder 返回 score 后，对有限数值做稳定归一化，再计算：

```text
fused = base_weight * normalized_base
      + cross_weight * normalized_cross_encoder
      + existing_source_and_relation_adjustments
```

归一化必须处理全相等、极端 logit、NaN/Inf 和未进入 Cross-Encoder 的尾部候选。权重是统一配置，不按 query/case 特判。retrieval reason 同时记录 base score/rank、Cross-Encoder score/rank、权重和 fused score。

现有 13 项 suite 是唯一质量验证入口；不新增 `dwpt-beb-planning` 专项用例。

## Artifact manifest

固定 revision `2cfc18c9415c912f9d8155881c133215df768a70`，manifest 至少包含：

| Path | Bytes | SHA-256 |
|---|---:|---|
| `onnx/model.onnx` | 1112459588 | `15b9a8c3da82eddf263df571281166e00e9308fe19d077084b642ebfcaf06d2b` |
| `tokenizer.json` | 17098107 | `9eb652ac4e40cc093272bbbe0f55d521cf67570060227109b5cdc20945a4489e` |
| `config.json` | 799 | `289adf7ada1eb6b4afa7589a48a032d45a076cf2e46dcdb3b4cabc33be14f708` |
| `special_tokens_map.json` | 279 | `d5469a60db23249c7f8945013d78df30b44b6bf686c6bb4740f4223f77b1b535` |
| `tokenizer_config.json` | 443 | `a1d6bc8734a6f635dc158508bef000f8e2e5a759c7d92f984b2c86e5ff53425b` |

## Provisioning state machine

```text
idle/missing
  -> downloading(.part)
  -> verifying(size + SHA-256)
  -> committing(atomic rename)
  -> initializing
  -> probing
  -> ready

downloading -> cancelled/partial
downloading -> network_error/partial
verifying   -> invalid/quarantine
any write   -> disk_full/error
```

每个 artifact 在目标目录写 `<name>.part`。下载循环按 chunk 更新 SHA/bytes、发进度并检查 cancellation。成功后 flush/sync、复核 size/hash，再在同一文件系统内 rename。已有 ready 文件先验证；有效则跳过，无效则 quarantine 后重新下载。旧 ready snapshot 在新 snapshot 完整通过前保持可用。

网络中断与取消保留 `.part`。再次 repair 根据服务器 Range 支持和 manifest size 决定续传；不支持安全续传时从零重建 `.part`。任何路径都不允许 query-time 发起网络请求。

## Cancellation and concurrency

`AppState` 增加 reranker provisioning cancellation/active-run 状态。repair command 创建唯一 token，并通过 typed Channel 传递进度；cancel command 只设置 token。重复 repair 在已有任务运行时返回稳定 busy error，不共享可变 writer。

取消错误使用 `RERANKER_DEPLOYMENT_CANCELLED`，不进入 generic download failure。任务结束后必须清除 active token，包含 success/error/panic join 分支。

## Failure injection

下载与文件提交通过窄接口隔离，测试替身可以：

- 在第 N 字节返回 network error；
- 在第 N 字节返回 disk-full writer error；
- 返回错误 payload/hash；
- 提供已有 valid/invalid final 与 partial；
- 连续执行两次 repair 并统计网络调用次数。

测试验证状态、文件系统结果、进度单调性、取消语义和旧 snapshot 保留，不依赖真实网络。

## Compatibility and rollback

- 保持 `RerankerDeploymentStatus` 旧字段可反序列化；新增字段使用默认值。
- query-time fallback 语义不变。
- 每阶段独立 commit，可分别回滚 batching/fusion、provisioning、failure tests。
- 模型文件位于外部缓存目录，不进入 Git。
