# Design — QA Production Hardening & Final Release Gate

## Architecture Boundaries

保持现有流水线，只替换/补强三个边界：

1. `answer -> AtomicClaimExtractor -> EvidenceMapper`：在现有 citation parser 之后、verification 之前形成原子 proposition。
2. `DeterministicVerifier -> SemanticVerificationProvider -> FinalStatusMerger -> AnswerRepair`：semantic 是真实 Provider 结果，heuristic 是可审计 fallback。
3. `eval artifacts -> release gate -> release report`：所有生产声明只来自机器可读、带版本和完整性校验的 artifact。

Retrieval/RRF/EvidenceManager 保持结构不变。Cross-Encoder 工作仅补齐模型生命周期、健康检查和真实 benchmark，不增加 Retriever。

## Atomic Claim Data Flow

```text
Markdown answer
  -> citation-aware sentence/list/table projection
  -> deterministic clause candidates
  -> guarded atomic split (connector + two informative clauses)
  -> ClaimType classification per clause
  -> citation inheritance limited to the original local sentence/citation suffix
  -> AtomicClaim { id,text,type,evidenceIds,status=Unverified,confidence=None }
```

切分规则必须保持文本可定位，AnswerRepair 使用原始原文逐次替换。建议短语与原因拆开；单独的不确定推断不因“但是”被错误切成无意义碎片。Markdown code/math/link/citation token 保持现有 mask 语义。

## Verification Contracts

```rust
enum SemanticEntailment { Entailed, Contradicted, Unknown }

struct SemanticVerificationResult {
    claim_id: String,
    status: SemanticEntailment,
    confidence: Option<f32>,
    reason: Option<String>,
}
```

`VerificationProvider` 接受一批可验证 Atomic Claims 与各自已映射 Evidence，使用一次 bounded structured-output call，避免每 Claim 消耗一次 LLM budget。Provider schema 关闭额外字段、限制枚举/数量，并要求结果恰好覆盖输入 Claim IDs。

合并矩阵：

| Mapping | Semantic | Heuristic | Final |
|---|---|---|---|
| missing/unknown/graph-only | not called | optional | `NotVerifiable` |
| valid | Entailed | any non-contradiction | `Supported` |
| valid | Contradicted | any | `Contradicted` |
| valid | Unknown | partial | `PartiallySupported` |
| valid | Unknown | other | `NotVerifiable` |
| valid | unavailable | completed | heuristic status；semanticChecked=false；fallback=true |

Provider call 在 answer generation 后、DB lock 前执行；与 generator 共用 `LlmBudgetGuard`，阶段名 `semantic_verifier`。Audit/persist 消费已经冻结的 verification batch，禁止在持有 repository DB mutex 时进行网络/模型调用，也避免当前重复 audit 触发重复 Provider 请求。

## Provider Capability

`ProviderCapabilities` 新增 `semantic_verification`。Codex 与 Compatible API 为 true，offline/unknown 为 false。复用同一个 structured provider transport，但 verification prompt/schema/timeout/telemetry 独立命名，避免把 planning 语义泄漏到 verifier。

## Cross-Encoder Lifecycle

- 机器级目录：显式环境变量优先，其次 semantic cache 下固定版本目录。
- manifest 固定 repo/revision/files/sha256/model ID；下载到 `.part`，校验后原子 rename。
- `inspect` 只离线；`provision/repair` 是唯一联网入口；损坏文件 quarantine，不静默删除旧可用 snapshot。
- query-time 仅 `ready -> load -> infer`；其他状态立即稳定 fallback。
- benchmark 记录 pre-rerank 与 post-rerank 指标以及 provider/model/fallback rate；fixture scorer 只属于 regression，不可满足 production gate。

## Evaluation Artifacts

统一目录：`evals/runs/<timestamp>_<git-sha>/`，包含 `config.json`、`retrieval.json`、`reranker.json`、`grounding.json`、`conversation.json`、`heldout.json`、`performance.json`、`telemetry.json`、`summary.md`。

所有 artifact 使用同一 metadata envelope；dataset hash 按 canonical JSON 计算，runtime config 只记录非敏感配置哈希。API key、完整对话、论文正文和绝对路径不进入 metrics artifact。

## Release Decision

Gate 读取冻结 threshold 文件和 artifact schema，缺失、版本不匹配、非有限数、伪造 provider 状态均作为失败原因。只有全部 P0 核心 gate 满足才 PASS。CONDITIONAL PASS 仅允许已冻结的非核心性能偏差；当前实现默认不自动给 CONDITIONAL PASS。

当前仓库没有独立 held-out 和真实模型运行 artifact，因此实现完成后的诚实预期结果是 `FAIL`（工具链完成、生产证据未完成），不是 Production Ready。

## Compatibility / Migration

- `QaRunManifest` schema bump；新增字段全部 `#[serde(default)]`，历史消息继续可读。
- 旧 `VerifiedClaim` JSON 通过默认值迁移到 AtomicClaim-compatible projection。
- 现有 deterministic verifier 保留为 fallback，旧 API 测试可通过 adapter 继续运行。
- 现有 query-time model no-download contract 保持不变。

## Rollback

- 每阶段独立 commit；Atomic/semantic/reranker/eval gate 可按 commit 回滚。
- 新模型 provisioning 写入版本目录和临时文件，不覆盖旧模型。
- Release gate 只读评测 artifact，不修改生产数据或阈值。
