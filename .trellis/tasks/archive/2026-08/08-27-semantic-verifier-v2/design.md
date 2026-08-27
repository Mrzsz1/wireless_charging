# Design — Semantic Verifier v2

## Boundaries

仅修改 `qa/claim_verification.rs`、`qa/semantic_benchmark.rs`、semantic benchmark CLI/数据/基线与对应 spec。Provider、model、temperature、batch size、retrieval、answer generation 和其他 QA 架构保持不变。

## Semantic decision flow

```text
mapped atomic claim + evidence
  -> deterministic mapping/hard checks
  -> semantic decision: all material parts supported?
       yes -> entailed
       no  -> explicit opposite or mutually exclusive?
               yes -> contradicted
               no  -> unknown
  -> merge: entailed=Supported; contradicted=Contradicted;
            unknown=NotVerifiable/PartiallySupported
  -> differentiated Answer Repair
```

Prompt 中只保留三个抽象反例：bounded scope expansion、direct negation、correlation-to-causation。Evidence 继续作为不可信 JSON data，输出继续使用 closed schema。

## Dataset contracts

- v1 保留原文件名，但版本升级并修正十个互斥 simulation/deployment Gold；重新计算 cases SHA-256。
- v2 使用 `qa-semantic-verifier-benchmark-v2`，允许且要求本轮 60 cases，冻结 20/20/20 分布。
- loader 同时兼容 v1 与 v2；v1 仍要求至少 100 cases，v2 要求恰好 60 cases和标签均衡。

## Report contracts

`qa-semantic-verifier-report-v2` 在现有字段上增加：

- `entailedPrecision/Recall`
- `contradictionPrecision/Recall`
- `unknownPrecision/Recall`
- `macroF1`
- `confusionMatrix[gold][predicted]`
- `categoryMetrics`
- `failedCases`
- 每题 `latencyMs/provider/fallback`

Batch latency 只能按 batch 中 case 数确定性分配为逐题审计值，同时保留真实 total latency；不得伪称每题独立网络调用耗时。

## Validation strategy

先完成静态转换与 unit tests，再运行 `cargo fmt/check`，最后对 v2 真实 Provider 运行一次。真实结果不用于继续调 Prompt。

## Rollback

代码、v1 Gold、v2 dataset/report/baseline 放在一个语义验证专用提交中，可独立回滚，不影响上一轮 QA 生产评测架构。
