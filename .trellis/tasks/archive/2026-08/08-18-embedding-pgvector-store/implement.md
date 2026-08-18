# 多粒度 Embedding 与 pgvector 实施计划

## Phase 0 — 前置

- [x] 确认子任务 1 已冻结 ContentBlock/SourceLocator schema。
- [x] Git 检查点，排除用户未跟踪文件。
- [x] 记录当前语义部署状态、向量数、模型维度和现有回归基线。

## Phase 1 — 抽取模型与存储接口

- [x] 从 `qa/semantic.rs` 抽取 model session/batch embed，不改变下载修复 UI 行为。
- [x] 定义 VectorStore、VectorRecord、VectorQuery、VectorHit 和脱敏错误。
- [x] 为内存 fake store 写契约测试，后续本地/远程共用。

## Phase 2 — 多粒度 embedding pipeline

- [x] 从 active ContentBlock 生成 embeddingText。
- [x] 按 model/dimension/hash 建增量计划。
- [x] 批量推理、进度、取消、失败恢复。
- [x] 验证 document/section/semantic 三粒度数量与引用键。

## Phase 3 — 本地 v2 store

- [x] 实现按 block key 的持久化和余弦查询。
- [x] 只读识别 LUNAVEC1，设计复用/重算迁移。
- [x] 校验损坏、部分写入、模型变化和 cache directory switch。

## Phase 4 — pgvector adapter

- [x] 添加远程迁移/schema 文件和 health probe。
- [x] 实现幂等 upsert、过滤查询、snapshot cleanup。
- [x] 实现密钥安全存储、错误脱敏、timeout/rate-limit retry。
- [x] 用本地 fake HTTP/DB fixture 或隔离测试验证，不把真实凭据提交仓库。

## Phase 5 — 设置与状态

- [x] 扩展 Tauri settings/commands/types/service。
- [x] 设置页展示 storage、counts、pending、health、last sync。
- [x] 远程配置未完成时提供明确状态，不影响本地模型检查。

## Phase 6 — 验证和编译

- [x] Rust：store contract、incremental plan、local roundtrip、fallback、cancel、secret redaction。
- [x] 前端：semantic settings 状态和下载进度回归。
- [x] `cargo fmt --check`。
- [x] `cargo test semantic --lib` 和新 vector_store 测试。
- [x] `npm run test:qa-settings`。
- [x] `npm run build`。
- [x] `cargo build --release`。
- [x] 提交并记录远程 schema 版本、配置方法和离线行为。

## 回滚

- `remote_vector_enabled=false` 立即关闭远程。
- 本地 v2 异常时退回 LUNAVEC1 只读查询或 lexical-only。
- 不删除旧 cache，直到最终 rollout 子任务完成迁移验证。
