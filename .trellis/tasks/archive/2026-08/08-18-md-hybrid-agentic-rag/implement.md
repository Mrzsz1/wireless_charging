# Markdown 科研混合 Agentic RAG 总实施计划

## 0. 交接规则

- 本父任务保持 `planning`，不直接承担代码实现；按顺序启动拥有交付物的子任务。
- 每个 Agent 开始前先运行 `python ./.trellis/scripts/task.py current --source`，确认目标子任务，再加载该子任务的 `prd.md`、`design.md`、`implement.md`。
- 修改代码前必须检查 `git status --short` 并建立 Git 检查点；只提交任务所属文件。现有未跟踪的自动检索运行目录和 `智能问答交接文档-2026-08-12.md` 属于用户资产，不得纳入提交或删除。
- 不改 `raw/**/*.md` 正文；索引、测试和配置可以读取它。
- 每个子任务完成后先执行局部测试，再执行其 `implement.md` 指定的编译命令并提交；最终由第 5 个子任务完成全量编译。

## 1. 执行顺序

### Gate A — 冻结契约

- [ ] 启动 `08-18-markdown-corpus-index-v2`。
- [ ] 冻结 `DocumentRecord`、`ContentBlock`、`SourceLocator` 和索引版本。
- [ ] 确认论文、书籍、Wiki 都使用 Markdown 路径且 PDF 可空。
- [ ] 输出可供后续任务复用的 Rust/TypeScript 类型和迁移说明。

### Gate B — 建立多粒度语义能力

- [ ] 启动 `08-18-embedding-pgvector-store`。
- [ ] 使用 Gate A 的 block ID/content hash 构建增量 embedding。
- [ ] 完成 VectorStore 抽象、远程 pgvector 适配和本地降级。
- [ ] 提供部署/健康/同步统计，不在每次对话下载模型。

### Gate C — 替换检索编排

- [ ] 启动 `08-18-hybrid-retrieval-agent`。
- [ ] 新 RetrievalContract 不包含固定 answer profile/minimumEvidence。
- [ ] 标题/别名、FTS、dense、metadata、graph 独立执行。
- [ ] 完成 RRF、reranker、多样性和软 coverage controller。
- [ ] 完成最多两轮的受限补查与可取消状态机。

### Gate D — 替换回答和链接

- [ ] 启动 `08-18-answer-evidence-links`。
- [ ] Provider 改为自然 Markdown 输出，不再要求复杂 answer JSON。
- [ ] 后端确定性追加证据附录。
- [ ] 前端用 SourceLocator 打开 Markdown 并定位 block/heading/line。
- [ ] 删除固定中文章节/claim count 对最终回答的阻断路径。

### Gate E — 评测和发布

- [ ] 启动 `08-18-rag-evaluation-rollout`。
- [ ] 建立来源受限、开放多来源、别名、隐式概念、零证据、降级和长会话回归。
- [ ] 双读对比 legacy 与 v2，并记录错误类型而不是只记录总命中率。
- [ ] 完成迁移、回滚、性能、构建和 release 验证。

## 2. 可并行工作

- Gate A 冻结类型后，Gate B 的远程 VectorStore 实现可与 Gate C 的 RetrievalContract/parser 并行。
- Gate D 的前端 EvidenceAppendix 视觉组件可提前开发，但深链行为必须等待 Gate A 的 SourceLocator 冻结。
- Gate E 可在 Gate A 后先编写 fixtures/评测框架，最终断言等待 Gate B–D 接入。
- 禁止多个 Agent 同时修改 `apps/desktop/src-tauri/src/qa.rs`；应先拆模块或由单一 owner 集成。

## 3. 预计模块拆分

```text
qa/corpus.rs
qa/locator.rs
qa/vector_store.rs
qa/retrieval_contract.rs
qa/retrieval.rs
qa/fusion.rs
qa/coverage.rs
qa/answer.rs
qa/audit.rs
```

现有 `qa/context.rs`、`qa/semantic.rs`、`qa/query_plan.rs`、`qa/grounding.rs` 在迁移期保留；新模块通过小提交逐步替代，最后再删除失去调用方的旧逻辑。

## 4. 最终质量门

```powershell
cd E:\知识库\wireless_charging\apps\desktop\src-tauri
cargo test --lib
cargo build --release

cd E:\知识库\wireless_charging\apps\desktop
npm run test:qa-evidence
npm run test:qa-settings
npm run build
npm run verify:p3
npm run verify:p5:strict
npm run tauri build

cd E:\知识库\wireless_charging
py -3 -m unittest discover -s tests
py -3 tools/wiki_eval.py --answers-dir evals/answers
```

## 5. 完成定义

- 五个子任务均有独立提交和验证记录。
- 父任务 AC1–AC13 有逐项证据。
- 新旧索引迁移与回滚均做过实际演练。
- Release 编译成功；问答目标回归由真实 Markdown 索引运行。
- 更新项目 PRD 的问答架构决策、Trellis backend/frontend 规范和开发日志后，再归档父任务。
