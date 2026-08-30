# P1-3S v2 Planner Provider Schema 最小兼容修复

## Goal

以可证明的最小修改修复 Codex Provider 对 Query Planner `RetrievalContract` 输出 Schema 的拒绝，同时保留完整领域 Schema 与 Rust 本地业务校验。只有临时 `127.0.0.1:7890` 代理下 Probe B、Probe C 和指定真实 Research 依次通过后，才把该代理作为 Codex 子进程的可覆盖默认值接入软件，并在清空 Shell 代理后完成最终验证。

## Background

- 基线提交：`02df15c0f694f416f4af44a5daf4724114086b8a`。
- 同一 `127.0.0.1:7890` 代理、Desktop Codex CLI 和 `gpt-5.6-luna/low` 下，Probe A 已通过。
- Probe B 在引入完整 `query_plan_schema()` 后以 `schema_rejected` 失败；Prompt 约 667 tokens，Schema 2528 bytes，baseline candidate 为 1，因此当前进入 Schema 分支。
- `kind_array_schema()` 在 `requestedKinds`、`mustAttemptKinds` 和 `facets[].preferredKinds` 三处复用并写入 `uniqueItems:true`。
- 完整 Schema 还包含 `$schema`、`examples`、`const`、`pattern`、长度/数量/数值范围约束；第一轮没有证据支持批量删除这些约束。
- Probe C 和 `real-research-improvement` 尚未运行；它们受前序 PASS 门禁约束。

## Requirements

### R1 — Evidence-driven RED

- 生产修改前新增并运行确定性测试，证明旧 Provider 路径使用的完整 Schema 含有 `uniqueItems`。
- 测试或审查必须证明同一 `kind_array_schema()` 覆盖上述三个字段位置。
- 保留 RED 证据提交，之后再修改生产行为。

### R2 — Split domain and Provider schemas

- `retrieval_contract_schema()` 继续表示完整领域契约并保留 `uniqueItems`。
- 新增语义清晰的 `retrieval_contract_provider_schema()`；它从完整 Schema 派生并递归删除且只删除 `uniqueItems`。
- 兼容转换必须是共享纯函数，并注明规则由真实失败证据驱动；没有新的 `schema_rejected` 证据不得扩展 denylist。
- Query Planner 公开层同时暴露完整 Schema 与 Provider Schema，名称必须避免后续误用。
- 真实 Planner、Probe B 和 Probe C 只向 Codex `--output-schema` 传递 Provider Schema；领域测试和文档仍可使用完整 Schema。

### R3 — Preserve local contract enforcement

- 不修改 `RetrievalContract` 字段、`deny_unknown_fields`、Prompt 业务语义、解析语义、排序、预算、Token Ceiling、Semantic、Generator 或 Grounding。
- 本地解析/归一化继续保证 kind 合法和去重、`mustAttemptKinds ⊆ requestedKinds`、facet ID 唯一与字符规则、facet/query 数量、budget 范围、unknown field、schemaVersion 和 scope 校验。
- duplicate kind 可确定性去重；duplicate facet ID 必须失败。

### R4 — Ordered live verification

- 先在显式临时 `HTTP_PROXY/HTTPS_PROXY/ALL_PROXY=http://127.0.0.1:7890` 下只运行一次 Probe B，不重跑 A。
- Probe B PASS 后只运行一次 Probe C；若 B 仍为 `schema_rejected`，立即停止后续 live run，读取仓库外真实诊断并只为下一实际 rejected keyword 建立一个 RED 与一个转换。
- Probe C PASS 后只运行一次 `real-research-improvement`；不得运行其他真实 case 或 Independent Heldout。
- 每个 live report 使用不覆盖的唯一输出路径，保留安全报告，临时原始诊断仅可位于仓库外并在检查后删除。

### R5 — Conditional default proxy integration

- 仅当临时代理下真实 Research 满足任务书门禁后，修改 `codex_subscription.rs`。
- 优先级必须为：`WIRELESS_CODEX_PROXY_URL` > 已有 `HTTP_PROXY/HTTPS_PROXY/ALL_PROXY` > 默认 `http://127.0.0.1:7890`。
- `WIRELESS_CODEX_PROXY_URL=off|direct|none` 表示不向 Codex 子进程注入代理。
- 代理只通过 `Command::env(...)` 注入 Codex 子进程；禁止调用 `std::env::set_var` 污染父进程。
- 不记录代理凭据或完整 URL；既有 Codex/Planner/Probe 生命周期日志继续承担开始、成功、失败和稳定错误分类定位。

### R6 — Final verification without Shell proxy

- 正式代理接入后清空当前 PowerShell 的大小写 proxy 环境变量。
- 依次只运行一次 Probe A、Probe B、Probe C 和 `real-research-improvement`。
- 全部通过后才关闭 P1-3S；任一门禁失败即按实际分类停止，不通过重复 live run 获取偶然成功。

### R7 — Delivery

- 每个可独立验证阶段形成普通 Git commit，不 amend、不改写历史。
- 完成 focused quality gate、规格更新、Trellis 归档和 journal 后，普通推送当前分支到 `origin`，禁止 force push。

## Acceptance Criteria

- [ ] S1：Provider Schema 递归不含 `uniqueItems`。
- [ ] S2：完整领域 Schema 仍包含 `uniqueItems`。
- [ ] S3：两份 Schema 的 properties、required、嵌套对象结构和 enum 值一致，差异只允许为 `uniqueItems`。
- [ ] S4：`["paper","paper","book"]` 归一化为 `["paper","book"]`。
- [ ] S5：duplicate facet ID 返回 `RETRIEVAL_CONTRACT_INVALID`。
- [ ] S6：`maxRounds=99` 返回 `RETRIEVAL_CONTRACT_INVALID`。
- [ ] S7：facet ID `A bad id!` 返回 `RETRIEVAL_CONTRACT_INVALID`。
- [ ] S8：unknown field 继续由 `deny_unknown_fields` 拒绝。
- [ ] 临时 7890 下 Probe B：`status=succeeded`、空 `failureCategory`、`contractValid=true`、`exitCode=0`、`agentMessageSeen=true`。
- [ ] Probe C：`status=succeeded`、空 `failureCategory`、`contractValid=true`、`baselineCandidateCount>0`。
- [ ] 临时代理真实 Research：Planner、Semantic、Final Grounding、citation、persistence 和 executed scope 均满足任务书阈值，exit code 0。
- [ ] 默认代理仅在上述 Research PASS 后接入，并通过覆盖/继承/default/direct 四类确定性测试。
- [ ] 清空 Shell proxy 后 Probe A/B/C 和真实 Research 全部通过。
- [ ] 相关日志仍使用统一设施、共享 operation ID 和稳定错误分类；无正文、证据、密钥、代理凭据或绝对路径进入日志。
- [ ] Budget / Semantic / Generator / Grounding 行为未修改，Independent Heldout 未运行。
- [ ] `cargo fmt`、focused Rust tests、Clippy、`git diff --check` 通过，工作树无无关生成物。
- [ ] 完成提交、Trellis 归档、journal，并普通推送到 GitHub。

## Out of Scope

- 修改 Planner Prompt、Retrieval 排序、Call Budget、Token Ceiling、Semantic Verifier、Generator、Grounding 或 frozen/heldout 数据。
- 一次性删除 `pattern`、`minimum/maximum`、`minItems/maxItems`、`minLength/maxLength`、`const`、`$schema` 或 `examples`。
- Probe B 失败后继续运行 Probe C；Probe C 失败后继续运行真实 Research。
- Independent Heldout、全量回归或重复 live canary。
