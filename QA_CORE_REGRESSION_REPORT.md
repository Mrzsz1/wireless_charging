# QA Core Functional Regression Report

## 1. 结论

**最终状态：PASS**

- Baseline commit：`4ef7b8f151732f41d61d4edebf244cdc91b4fbfa`
- Final verified code commit：`202d8f047f96fe709fa8cd48e0ab8044434828b7`
- Branch：`master`
- Independent Held-out：**NO / 未读取、未运行、未修改**
- Frozen threshold 修改：**NO**
- 性能优化：**NO**
- 生产 Bug：1 个（P1），已增加永久回归并修复
- 最终统计：374 个通过用例；正常 QA suite 中另有 2 个 ignored，其中本轮单独实测 1 个本地模型用例通过，剩余 1 个是会下载生产 reranker 的 provision 测试，未执行下载，但同一已部署 reranker 已由 13-case RAG 生产评测实际加载并验证。

## 2. 测试环境

| 项目 | 值 |
|---|---|
| OS / arch | Windows x86_64 |
| Rust | `rustc 1.96.1` / `cargo 1.96.1` |
| Node / npm | `v24.11.0` / `11.15.0` |
| Python | `py -3: 3.13.0`; `python: 3.10.0` |
| Codex CLI | `0.146.0`，ChatGPT 登录可用 |
| Semantic model | `E:\知识库\语义模型`，Paraphrase MiniLM L12 v2，实测通过，blob 235,052,644 bytes |
| Reranker | `E:\知识库\语义模型\reranker-bge-base`，1,112,459,588 bytes，13-case RAG 无 fallback |
| ONNX Runtime | `E:\知识库\语义模型\onnxruntime-1.20.1\onnxruntime.dll`，11,569,696 bytes |

模型和 reranker 数据均位于 E 盘；未下载或复制到 C 盘。

## 3. 静态契约审计

- `QaRunManifest`：`qa-run-v21`。
- Natural Answer：`qa-natural-markdown-v2` / `natural-markdown-v2`。
- Context：`qa-context-v4`；Retriever：`hybrid-agentic-rrf-v6`。
- Rust、TypeScript、AskView 均包含 `routingTokenCostInFlight` 与 `routingTokenCostReservedTotal`。
- Token ceilings 保持 Direct 8,000 / Research 18,000 / Exploratory 32,000。
- Budget 准入继续按 `used + inFlight + new <= ceiling`。
- Retrieval frozen thresholds 保持 Work Recall@20 0.95、Recall@10 0.90、MRR 0.85、nDCG@10 0.85。
- `planner_and_fallback_receive_the_post_patch_research_state` 验证 state patch 先于 `ResearchQueryContext` 与 retrieval。
- Semantic v2、visible claim projection、unknown citation、zero-evidence 和 provider failure 均有确定性契约测试。

未发现 schema drift。

## 4. 实际执行的 Suite

| Suite / command | 实际结果 |
|---|---|
| `cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml -- --check` | PASS |
| `cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml --lib -- -D warnings` | PASS |
| `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml qa:: --lib` | 210 passed / 0 failed / 2 ignored |
| 本地模型 ignored test（显式 E 盘 cache） | 1 passed / 0 failed |
| `py -3 -m unittest tests/test_qa_eval_metadata.py` | 5 passed |
| `npm run test:qa-settings` | 8 passed |
| `npm run test:qa-evidence` | 5 passed |
| `npm run build` | TypeScript + Vite PASS |
| `npm run eval:rag`（E 盘真实 embedding/reranker） | 13/13 PASS |
| `npm run eval:conversation` | 50/50 PASS |
| `npm run eval:conversation:state:v2` | 22/22 PASS |
| `npm run eval:semantic:v2`（Codex subscription） | 60/60 PASS |
| `npm run tauri -- build` | release EXE + MSI + NSIS PASS |
| NSIS `/S` 安装 + 8 秒启动 smoke | PASS |

最终通过用例数按 suite 执行计数：`210 + 1 + 5 + 8 + 5 + 13 + 50 + 22 + 60 = 374`。

初次 Python 命令从 `apps/desktop` 错误目录执行产生一次 import error；切回 repository root 后唯一一次有依据重试通过。该问题分类为命令工作目录错误，不是代码失败。首次安装命中旧 installer-smoke 注册位置；使用正式卸载器移除旧测试安装后，重新安装到 E 盘并通过严格启动检查。

## 5. A–J 功能矩阵

矩阵的 Cases 是任务书中的功能契约项；H 组报告真实 60-case Semantic v2 数据集。矩阵行不可与“总执行用例数”直接相加。

| Group | Cases | PASS | FAIL | BLOCKED |
|---|---:|---:|---:|---:|
| A. Normal QA | 8 | 8 | 0 | 0 |
| B. Conversation | 6 | 6 | 0 | 0 |
| C. Research State | 10 | 10 | 0 | 0 |
| D. Parameter State | 10 | 10 | 0 | 0 |
| E. Research / Exploratory | 10 | 10 | 0 | 0 |
| F. Retrieval / Evidence | 10 | 10 | 0 | 0 |
| G. Citation / Claim | 8 | 8 | 0 | 0 |
| H. Semantic Verification v2 | 60 | 60 | 0 | 0 |
| I. Failure / Fallback | 10 | 10 | 0 | 0 |
| J. Token Budget | 11 | 11 | 0 | 0 |

### 关键数值

- RAG：13/13；Work Recall@5/10/20 = 1.000/1.000/1.000；Work MRR = 1.000；nDCG@10 = 1.000；locator validity = 1.000；zero-evidence TP/FP/FN/TN = 1/0/0/12；真实 reranker fallback = 0/13。
- Conversation：50 cases；reference resolution = 1.000；constraint preservation = 1.000；objective preservation = 1.000。
- Conversation State v2：22 cases；所有 exact match / query-context recall = 1.000；unexpected state rate = 0；destructive mutation error rate = 0；`parameterStateCorruptionCount = 0`。
- Semantic v2：60/60；Entailed/Contradicted/Unknown 各 20；overall accuracy、macro-F1、三类 precision/recall 均为 1.000；`realProviderMeasured = true`；fallback rate = 0；invalid verified state count = 0。
- Budget：reservation reuse、真实超限拒绝、concurrent anti-oversell、settle/release/Drop、provider error、panic unwind、累计 call budget、Direct→Research reconfigure、Exploratory stress 全部通过。

## 6. Bug 记录

### BUG-001

**Severity:** P1

**Symptom:** `npm run eval:conversation` 的 50 条公开 Development/Regression case 全部只有 1/2 constraints 命中，aggregate constraint preservation = 0.5，低于冻结门槛 0.97；reference/objective 均为 1.0。

**Reproduction:**

1. 运行 `npm run eval:conversation`，进程 exit 2。
2. 新增 bundled-suite gate 后，在修复前运行：`constraint_preservation` 实际为 `0.5`，断言期望 `1.0`，稳定失败。
3. 定向打印首条 derived state，只有 `deadlines`，缺少 `multi_vehicle_coordination`。

**Root Cause:** `apps/desktop/src-tauri/src/qa/state_mutation.rs` 的 `canonical_mentions` 只识别“多车协同 / 多辆车 / 多充电车 / multi vehicle”。用户自然表达“多个移动充电车”不包含这些连续子串，因此 deterministic state patch 只记录 deadline，没有记录多车协同约束。问题位于 Constraint canonicalization，不是 frozen threshold、benchmark denominator 或 Provider。

**Fix:** 增加通用中英文别名“多个移动充电车 / 多辆移动充电车 / multiple mobile chargers”，不修改数据集、阈值、router、retrieval、prompt 或 verifier。

**Regression Tests:**

- `qa::state_mutation::tests::multiple_mobile_chargers_are_a_coordination_constraint`
- `qa::conversation_benchmark::tests::bundled_production_conversation_suite_preserves_all_frozen_state`

**Affected Files:**

- `apps/desktop/src-tauri/src/qa/state_mutation.rs`
- `apps/desktop/src-tauri/src/qa/conversation_benchmark.rs`

**Result:** PASS；修复 commit `202d8f047f96fe709fa8cd48e0ab8044434828b7`。

Bug 计数：P0 = 0，P1 = 1，P2 = 0，P3 = 0。

## 7. Failure / Fallback 结论

- Understanding/planner provider failure：deterministic fallback 原因稳定且可审计。
- Generator failure：失败 exchange 保持成对持久化，首轮失败 session 可恢复。
- Malformed JSON / timeout / budget rejection：fail closed 或进入显式 fallback，不伪造成成功。
- Cancellation：retrieval、reranker、vector sync 均不把 cancellation 转换成普通 fallback；不提交半成品状态。
- Unknown citation：被拒绝或受限修复，未知 Evidence ID 不进入 verified answer。
- Reranker unavailable：保持 fused candidates 并记录 degradation；真实 E 盘 reranker 路径实测无 fallback。
- Semantic verifier unavailable：显式 unavailable/fallback telemetry；真实 Codex Semantic v2 本轮为 60/60 成功。
- Provider failure session safety：失败消息不进入下一轮 trusted history，Research/Parameter state 不被半成品污染。

## 8. 构建与安装

- Release binary：`E:\知识库\wireless_charging\apps\desktop\src-tauri\target\release\app.exe`
- MSI：`E:\知识库\wireless_charging\apps\desktop\src-tauri\target\release\bundle\msi\Wireless Charging Research Workbench_0.12.4_x64_en-US.msi`
- NSIS：`E:\知识库\wireless_charging\apps\desktop\src-tauri\target\release\bundle\nsis\Wireless Charging Research Workbench_0.12.4_x64-setup.exe`
- 正式安装目录：`E:\Applications\Wireless Charging Research Workbench`
- 安装版本：`0.12.4`
- 安装器 exit code：0
- 启动 smoke：安装后的 `app.exe` 持续运行 8 秒，PASS。

## 9. 未修问题与外部阻塞

- 未修核心问题：无。
- 外部环境阻塞：无。
- 未执行的 ignored provision test：`provisions_and_health_checks_the_real_production_reranker` 会触发网络下载，因此未重复下载；其目标模型已由 E 盘真实 RAG 评测加载并完成 13/13，无 fallback。
- `eval:production` 未运行，因为其默认入口会读取 heldout-derived；本任务严格隔离 Independent Held-out。

## 10. Production QA 行为修改

仅一项：Research State deterministic constraint extractor 现在把“多个/多辆移动充电车”识别为 `multi_vehicle_coordination`。没有修改 Retrieval、Query Planner、Prompt、Generator、Semantic Verifier、Natural Markdown renderer、frozen dataset、threshold 或 token ceiling。

## 11. 25 个最终问题

1. **Baseline Commit 是什么？** `4ef7b8f151732f41d61d4edebf244cdc91b4fbfa`。
2. **Final Commit 是什么？** 最终验证的代码 commit 为 `202d8f047f96fe709fa8cd48e0ab8044434828b7`。
3. **总共执行了多少测试？** 374 个通过用例；正常 QA suite 显示 2 ignored，其中 1 个本轮另行实测通过，1 个下载型 provision test 未执行。
4. **哪些是 deterministic tests？** Rust QA 210、E 盘本地 embedding 1、Python 5、frontend 13、RAG 13、Conversation 50、Conversation State 22，共 314 个本地/确定性用例。
5. **哪些是真实 production-path tests？** RAG 13、Conversation 50、Conversation State 22、真实 Provider Semantic v2 60，以及 release build/install/startup smoke。
6. **是否使用真实 Provider？** YES；Codex subscription / `gpt-5.6-luna` / low，Semantic v2 60/60。
7. **如果没有，哪些项目被环境阻塞？** 不适用；本轮无环境阻塞。
8. **普通知识库 QA 是否通过？** YES。
9. **多轮 reference 是否通过？** YES，reference resolution = 1.0。
10. **Research State 是否通过？** YES。
11. **Parameter State Corruption Count 是否为 0？** YES，值为 0。
12. **Research / Exploratory 是否通过？** YES。
13. **Zero Evidence 是否安全？** YES，TP/FP/FN/TN = 1/0/0/12，不伪造来源。
14. **Unknown Citation 是否安全？** YES，未知 ID 被拒绝或受限移除。
15. **Semantic Verifier Unknown/Contradicted 是否通过？** YES；各 20 条全部正确。
16. **Generator Budget regression 是否通过？** YES。
17. **真正 over-budget 是否仍然会 FAIL？** YES，`budget_guard_reserves_settles_and_rejects_over_budget_calls` 与并发上限测试通过。
18. **Provider failure 是否会污染 Session？** NO；失败 exchange/state/history 原子性测试通过。
19. **发现了多少个 P0/P1/P2/P3 Bug？** 0 / 1 / 0 / 0。
20. **每个 Bug 新增了什么 regression test？** BUG-001 新增一个纯 state-mutation 测试与一个 bundled 50-case conversation gate。
21. **是否修改 Frozen Threshold？** NO。
22. **是否使用 Independent Heldout？** NO。
23. **是否做性能优化？** NO。
24. **cargo fmt / clippy / tests / frontend / Python 分别结果是什么？** fmt PASS；clippy PASS；Rust QA 210 passed/0 failed/2 ignored，另行真实本地模型 1 passed；frontend 13 passed 且 TypeScript/Vite build PASS；Python 5 passed。
25. **最终结论？** **PASS**。

