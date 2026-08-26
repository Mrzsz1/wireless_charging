# Implementation Plan — Cross-Encoder 批量重排与可靠部署

## Phase 1 — Batched inference and score fusion

- [x] 在真实模型上比较现有 batch 16 与单批 batch 80，确认 FastEmbed `par_chunks` 与 ONNX 全核执行的过度订阅风险。
- [x] 引入明确的 batch config/telemetry，复用单一 Cross-Encoder session，默认单批覆盖最多 80 个候选。
- [x] 冻结 base score/rank，并实现稳定归一化后的 base + Cross-Encoder score fusion。
- [x] 保持 explicit source 与 relation adjustments；增加统一 document-repeat score penalty，不缩减候选数量、不按用例特判。
- [x] reranker unit tests 与现有 13 项 RAG suite 通过；未新增 `dwpt-beb-planning` 专项回归用例。平均 reranker latency 35553.7 ms，fallback 0/13。
- [x] 本地 Git commit：`perf(qa): batch and fuse cross-encoder reranking`。

## Phase 2 — Cancellable progress-aware provisioning

- [ ] 新增固定 revision/file/size/SHA-256 artifact manifest。
- [ ] 用可控流式 downloader 替换 reranker 的 FastEmbed 隐式下载，保留底层分类错误。
- [ ] 实现 `.part → size/SHA-256 → flush/sync → atomic rename`。
- [ ] 增加 request-scoped cancellation token、active-run 互斥和 cancel Tauri command。
- [ ] 增加 typed progress Channel：bytes/total/speed/percent/phase/file/status/message。
- [ ] 设置页显示真实进度与取消按钮；query-time no-download 保持不变。
- [ ] 本地 Git commit：`feat(qa): make reranker provisioning cancellable`。

## Phase 3 — Failure injection and production verification

- [ ] 增加 disk-full、network interruption、cancel、corrupt payload/final、partial resume 测试。
- [ ] 增加 repeated repair/idempotency 与 concurrent repair rejection 测试。
- [ ] 验证旧 ready snapshot 在所有失败路径保持可用。
- [ ] 运行真实 health probe、现有 RAG benchmark，记录 before/after latency、fallback 与排名指标。
- [ ] 更新 `evals/reranker-production-baseline.md` 与父任务阶段 C 状态。
- [ ] 运行 fmt、targeted tests、full Rust tests、clippy 与 frontend typecheck/tests/build。
- [ ] 本地 Git commit：`test(qa): verify reranker deployment failures`。

## Validation commands

```powershell
cd apps/desktop/src-tauri
cargo fmt --check
cargo test qa::reranker
cargo test qa::semantic
cargo test provisions_and_health_checks_the_real_production_reranker -- --ignored --nocapture
cargo run --bin rag-eval -- "E:\知识库\wireless_charging"
cargo test
cargo clippy --all-targets --all-features -- -D warnings

cd ..
npm run typecheck
npm test
npm run build
```

真实模型命令运行前通过应用设置或测试环境指向用户配置的非系统盘 `${SEMANTIC_CACHE_DIR}`；不得把模型复制进仓库或 C 盘默认目录。

## Risky files

- `apps/desktop/src-tauri/src/qa/semantic.rs`
- `apps/desktop/src-tauri/src/qa/reranker.rs`
- `apps/desktop/src-tauri/src/qa/retrieval.rs`
- `apps/desktop/src-tauri/src/qa.rs`
- `apps/desktop/src-tauri/src/lib.rs`
- `apps/desktop/src/services/desktop.ts`
- `apps/desktop/src/types.ts`
- `apps/desktop/src/features/settings/SettingsView.tsx`
- `apps/desktop/src/features/settings/SettingsView.css`
- `evals/reranker-production-baseline.md`

## Review gates

- [ ] 没有为单一 benchmark case 写路径、标题或 ID 特判。
- [ ] 没有减少候选数量或降低父任务阈值。
- [ ] progress 字节单调且来自实际读写；未知总量不伪造百分比。
- [ ] cancellation 不变成 fallback/success，且不破坏旧模型。
- [ ] valid ready 的重复 repair 不发起大文件下载。
- [ ] 模型、`.part` 和 failure fixture 大文件均不进入 Git。
- [ ] 用户已有未跟踪文件不暂存、不提交。
