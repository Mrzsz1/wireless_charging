# 科研 RAG 评测、迁移与灰度实施计划

## Phase 0 — 准备

- [ ] 确认子任务 1–4 的 feature flags、schema versions 和提交 SHA。
- [ ] Git 检查点；备份实际 SQLite 与语义缓存 manifest。
- [ ] 记录 legacy 问题 audit、会话/消息/证据计数和 build baseline。

## Phase 1 — 评测 harness

- [ ] 定义 case schema/parser 和重复 key/未知字段校验。
- [ ] 从真实 Markdown snapshot 构建隔离测试数据库。
- [ ] 添加文档/块级 Recall@k、MRR、channel attempt、locator integrity 指标。
- [ ] 添加 answer appendix/zero-evidence/unknown link 结构指标。

## Phase 2 — 回归用例

- [ ] 指定《近似算法》移动路径问题。
- [ ] 开放“文献或者哪本书”问题。
- [ ] 至少 3 个同义/双语改写。
- [ ] 至少 2 个新概念/新来源类型组合，确认代码无特判。
- [ ] 多轮指代、压缩、failed/unverified exclusion。
- [ ] graph/reference-only、true zero evidence 和 locator drift。

## Phase 3 — 故障注入

- [ ] Planner invalid/timeout。
- [ ] Local semantic model missing。
- [ ] Remote pgvector timeout/rate limit/auth failure（脱敏 fixture）。
- [ ] Reranker unavailable。
- [ ] Graph missing/corrupt。
- [ ] Cancel during round 1/2、repository switch 和 app navigation。

## Phase 4 — 双读与调优

- [ ] 运行 legacy/v2 retrieval comparison。
- [ ] 按 source/chunk/lexical/dense/fusion/rerank/coverage/locator 分类问题。
- [ ] 只根据跨问法 case 调整通用策略，拒绝关键词专用 patch。
- [ ] 冻结 v2 thresholds/limits/version。

## Phase 5 — 迁移和回滚演练

- [ ] 实际复制旧数据库执行 migration。
- [ ] 核对 chat sessions/messages/evidence 计数和抽样 payload。
- [ ] LUNAVEC1 迁移/重算演练。
- [ ] remote snapshot build/switch/cleanup。
- [ ] 按 feature flags 逐层回滚并再次打开旧会话。

## Phase 6 — 全量质量门

- [ ] `cargo fmt --check`。
- [ ] `cargo test --lib`。
- [ ] `py -3 -m unittest discover -s tests`。
- [ ] `py -3 tools/wiki_eval.py --answers-dir evals/answers`。
- [ ] `npm run test:qa-evidence`。
- [ ] `npm run test:qa-settings`。
- [ ] `npm run build`。
- [ ] `npm run verify:p3`。
- [ ] `npm run verify:p5:strict`。
- [ ] `cargo build --release`。
- [ ] `npm run tauri build`。
- [ ] GUI 冒烟：启动、指定书问答、开放问答、证据跳转、后台继续、取消、重启历史恢复。

## Phase 7 — 文档、提交和归档

- [ ] 更新 `prd.md` 问答架构决策和版本状态。
- [ ] 更新 `.trellis/spec/backend/qa-contract.md` 和必要 frontend 规范。
- [ ] 追加 `logs/log.md` 和详细验证记录。
- [ ] 记录 artifacts SHA-256、测试/编译退出码和剩余风险。
- [ ] 提交后逐项关闭父任务 AC，归档五个子任务和父任务。

## 发布阻断条件

- 指定来源未尝试却返回“没有”；
- unknown/越界 locator 可点击；
- 远程失败阻断本地 FTS；
- 旧会话/消息计数变化；
- Raw Markdown 被修改；
- 任一 release 编译失败。
