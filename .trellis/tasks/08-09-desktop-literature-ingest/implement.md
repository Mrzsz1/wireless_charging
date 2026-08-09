# 客户端文献添加与自动入库：实施计划

## 0. 执行原则

- 当前任务在用户审阅规划后才能由 `task.py start` 激活。
- 主线程按 inline 模式直接实施和检查，不派发实现/检查子代理。
- 每个阶段先补失败测试，再修改实现；任何写入正式 Wiki 的测试只能使用临时 fixture。
- 保持 Git 工作区可回滚：阶段性检查通过后再进入下一阶段，最终统一提交。

## 1. 基线与契约冻结

- [ ] 记录 `git status --short`、当前版本和现有测试基线。
- [ ] 阅读即将修改的完整文件：`App.tsx`、`types.ts`、`desktop.ts`、`compile_center.rs`、`lib.rs`、`paper_search.py`、`paper_triage.py`、`mineru_to_md.py`、`full_pipeline.py`。
- [ ] 阅读相关 Trellis frontend/backend guideline；若仍为空模板，以现有代码约定为准，不在本任务顺带重写 guideline。
- [ ] 在根 `prd.md` 标记待更新位置，但实施完成前不提前改变权威行为描述。
- [ ] 建立 fixture：旧/新 discovery manifest、重复 Wiki source、canonical PDF、无效/超限/变更后的手动文件。

验证：

```powershell
git status --short
cd apps/desktop
npm run test:p1
npm run build
cd src-tauri
cargo test
cd ../../../..
py -3 -m unittest discover -s tests -p "test_*.py"
```

回滚点：此阶段只新增测试 fixture/测试骨架，不改运行行为。

## 2. 领域元数据与候选聚合

- [ ] 扩展 `tools/paper_search.py` 的 Paper JSON：`candidate_id`、`title_matches`、`abstract_matches`，保持旧调用兼容。
- [ ] 新增 `tools/literature_ingest.py` 的稳定 ID、manifest 迁移、跨 run 聚合、状态冲突处理、资格判断和 JSON 输出。
- [ ] 重复检测实现 DOI、arXiv、规范化标题；PDF 可用时补 SHA-256。
- [ ] 所有 manifest 修改采用临时文件 + replace；首次迁移保留审计备份或明确 migration event。
- [ ] 资格结果返回逐条 reason code，不解析展示文案。
- [ ] 为 score=8 边界、无标题命中、无标识、无 PDF、重复、上限 3、旧 manifest 缺字段增加 Python 单测。
- [ ] 保持 `paper_triage.py` 现有 CLI 可用，并让新字段随 materialize sidecar 保留。

验证：

```powershell
py -3 -m unittest tests.test_paper_search
py -3 -m unittest tests.test_paper_triage
py -3 -m unittest tests.test_literature_ingest
py -3 tools/literature_ingest.py list-candidates --repository . --json
```

回滚点：移除新增工具和 Paper 可选字段即可恢复；旧 manifest 不应被测试直接改写。

## 3. 精确范围入库编排器

- [ ] 在 `tools/literature_ingest.py` 实现 `stage-manual`、`download-candidates`、`ingest`、`auto-run` 固定子命令。
- [ ] 手动清单执行 size/mtime/SHA-256 二次校验，安全复制到批次目录并写 `manual_upload` provenance。
- [ ] 候选清单必须按 stable ID 回查 owning manifest；拒绝前端伪造 URL、路径或元数据。
- [ ] “仅下载”更新 local PDF 与 sidecar，但不写 promoted/canonical/Wiki。
- [ ] 正式入库逐篇执行，A 编译提示只允许本篇 canonical 目录；禁止 B 类、schema、vocab 和删除操作。
- [ ] 输出标准 pipeline/item 事件；结果 JSON 记录 completed/failed/skipped 及 source page。
- [ ] 重试复用安全完成阶段，失败不阻塞下一篇。
- [ ] 增加 fixture 模式，模拟 discovery、download、parse、compile、lint、graphify、snapshot 的成功/单篇失败/取消。

验证：

```powershell
py -3 -m unittest tests.test_literature_ingest_pipeline
py -3 tools/literature_ingest.py auto-run --repository . --dry-run --json
```

回滚点：新编排器尚未接入客户端，不影响旧 `full_pipeline.py`。

## 4. Tauri 数据层与命令

- [ ] 新增 `apps/desktop/src-tauri/src/literature_ingest.rs`。
- [ ] 新增 SQLite settings/manual session 表及幂等 migration。
- [ ] 实现 repository-scoped settings，默认 startup prompt=true、auto promote=false、min score=8、max=3。
- [ ] 实现自然日 prompt 判定：“运行”不抑制、“今天不再提醒”写当天、“取消”不写。
- [ ] 实现后端多 PDF 选择、PDF 头/大小/路径/重解析点/哈希预检、重复匹配和临时 session。
- [ ] session 启动时二次校验，消费/过期清理，禁止前端直接传外部路径。
- [ ] 实现候选 list/detail/triage commands；manifest 继续作为权威。
- [ ] 在 `lib.rs` 注册命令，并确保仓库未选择/切换时返回一致错误或清理旧 session。
- [ ] Rust 单测覆盖设置隔离、日期语义、旧 manifest、重复原因、session 篡改、路径边界和过期清理。

验证：

```powershell
cd apps/desktop/src-tauri
cargo fmt --check
cargo test literature_ingest
cargo test
```

回滚点：新 module/table/commands 独立，未接入 UI；SQLite 表可保留而不影响旧版本。

## 5. 编译中心受控执行接入

- [ ] 扩展 `StartCompileRequest` 内部可信参数与五个 literature task kind。
- [ ] `start_literature_run` 解析 session/candidate IDs，生成应用本地 run manifest，再调用同一 compile runner。
- [ ] `build_task` 仅映射固定 Python 工具和固定 mode；不得接受命令字符串、任意脚本或任意 manifest 路径。
- [ ] 扩展 stage plan、artifact scopes、事件解析和 result JSON，支持单篇 item 事件。
- [ ] 保持仓库写锁、日志脱敏、取消/暂停、timeout、interrupted、retry 和 rollback。
- [ ] 编译中心任务目录显示新增任务或至少能打开其历史详情；不要求普通用户从编译中心手填 literature 参数。
- [ ] Rust 测试覆盖 allowlist、可信 manifest、仓库隔离、单篇部分失败、重试和日志脱敏。

验证：

```powershell
cd apps/desktop/src-tauri
cargo fmt --check
cargo test compile_center
cargo test
```

回滚点：保留旧任务种类不变；移除新增映射即可退回旧编译中心。

## 6. TypeScript 契约与纯状态逻辑

- [ ] 在 `types.ts` 增加 settings、candidate、duplicate、manual session、prompt、literature run DTO。
- [ ] 在 `services/desktop.ts` 增加逐一对应的 invoke/Channel 封装；前端不得直接 cast 原始 payload。
- [ ] 新增 `features/ingest/ingestState.ts`，实现筛选、排序、批量选择、默认排除重复/无效、资格摘要、prompt 日期逻辑和最新请求守卫。
- [ ] 增加 Node TypeScript 测试，覆盖仓库切换、迟到响应、候选去重、批量选择、prompt 三动作和事件归并。
- [ ] 在 `package.json` 增加 `test:ingest`，纳入 build/verify 前质量门。

验证：

```powershell
cd apps/desktop
npm run test:ingest
npm run test:p1
npm run test:p2
npm run test:research-trail
```

回滚点：类型和 service 封装尚未渲染，不影响现有导航。

## 7. 文献入库 UI

- [ ] 新增 `LiteratureIngestView.tsx/.css` 与三个标签组件。
- [ ] 手动标签：系统选择器、预检表、默认排除、重复覆盖、执行摘要、启动/取消、逐篇结果。
- [ ] 待确认标签：搜索、筛选、排序、分页或虚拟化阈值、详情、批量选择、确认添加、仅下载、拒绝、稍后、备注。
- [ ] 自动标签：自动准备/自动入库说明、显式开关、来源、阈值、上限、资格预览、立即运行、最近任务。
- [ ] 所有写操作使用应用内确认摘要，不使用不可测试的散落 `window.confirm`。
- [ ] 关联 run 可跳转编译中心并选中对应任务；成功 source 可通过 `openPage` 打开。
- [ ] 缺依赖按动作展示原因；加载、空、错误、降级、运行和部分失败状态完整。
- [ ] 键盘、焦点、aria、窄窗口和现有天蓝色视觉规范通过检查。

验证：

```powershell
cd apps/desktop
npm run test:ingest
npm run build
```

回滚点：组件独立，尚未接入一级导航时可整体移除。

## 8. 导航、启动提示与跨视图刷新

- [ ] `App.tsx` 增加 `ingest` MainView、侧栏导航、标签页和 workspace class。
- [ ] “文献入库”置于“文献库”和“方法库”之间。
- [ ] 仓库 ready/generation 变化后读取 prompt state；每次仓库代次只弹一次，避免 React effect 重复。
- [ ] 新增 `StartupIngestPrompt` 三动作：“本次运行”“今天不再提醒”“取消”。
- [ ] 本次运行明确展示 configured mode；运行后关联 literature view/compile run。
- [ ] 正式入库成功后触发 repository index rebuild、catalog refresh、Graph refresh 和 research trail refresh；忽略旧仓库迟到响应。
- [ ] 增加 dashboard 快捷入口与帮助说明。
- [ ] GUI E2E fixture 覆盖导航、三个标签、启动 prompt 三动作和一次模拟成功入库。

验证：

```powershell
cd apps/desktop
npm run test:ingest
npm run build
npm run verify
npm run verify:p3
npm run verify:p4
npm run verify:p5
npm run e2e:gui:strict
```

回滚点：可隐藏 nav/prompt，保留后端和编译能力供后续恢复。

## 9. 文档与治理同步

- [ ] 更新根 `prd.md` §1.3、§5.1、§5.3：记录两级自动化、显式启用、资格阈值、启动询问/手动按钮，并保留 B 类和 schema 禁令。
- [ ] 更新 `apps/desktop/README.md`：新入口、三类流程、依赖、自动边界、故障处理。
- [ ] 如新增 manifest 字段属于长期契约，更新相应 `schema/` 说明；不直接修改 `vocab.yaml`。
- [ ] 更新 `logs/log.md` 与当日详细日志。
- [ ] 更新版本号和发布说明。

验证：

```powershell
py -3 tools/wiki_lint.py --strict-graphify
git diff --check
```

## 10. 全量质量门

- [ ] Python：

```powershell
py -3 -m unittest discover -s tests -p "test_*.py"
py -3 tools/wiki_eval.py
py -3 tools/core_book_eval.py
py -3 tools/wiki_lint.py --strict-graphify
```

- [ ] 前端：

```powershell
cd apps/desktop
npm run test:p1
npm run test:p2
npm run test:research-trail
npm run test:installer-lifecycle
npm run test:ingest
npm run build
npm run verify
npm run verify:p3
npm run verify:p4
npm run verify:p5
```

- [ ] Rust：

```powershell
cd apps/desktop/src-tauri
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```

- [ ] 图谱和差异：

```powershell
cd E:\知识库\wireless_charging
graphify update . --force
git diff --check
git status --short
```

停止条件：任一回归失败、真实知识库被测试写入、自动资格出现无理由通过、单篇失败生成伪成功 source、仓库外路径可由前端伪造时，不进入发布阶段。

## 11. Windows 构建与发布验证

- [ ] 构建 Tauri release 和 NSIS。
- [ ] 严格 GUI 冒烟验证窗口显示、仓库恢复、启动 prompt、文献入库导航、手动选择器入口和模拟任务。
- [ ] 确认安装/升级/卸载不遗留后台服务或计划任务。
- [ ] 记录安装包绝对路径、版本、大小和验证结果。

```powershell
cd apps/desktop
npm run tauri build
npm run test:e2e-config
npm run e2e:gui:strict
```

## 12. 最终审查、提交与收尾

- [ ] 按 `trellis-check` 做 spec、跨层数据流、复用、类型、测试和文档检查。
- [ ] 检查 PRD 的每个 AC 均有实现和测试证据。
- [ ] 检查 Git diff 不包含 API Key、下载测试 PDF、临时 run manifest、数据库或构建缓存。
- [ ] 更新 Trellis spec 中值得长期保留的约定。
- [ ] 提交代码、测试、文档和任务记录，提交信息建议：`feat(desktop): add governed literature ingestion`。
- [ ] 记录 journal，归档任务并向用户报告安装包与使用方法。

## 13. 实施结果（2026-08-09）

- 已完成独立“文献入库”入口、手动添加、待确认、自动添加和启动三选项提示。
- 已完成可信手动会话、PDF 二次指纹校验、DOI/arXiv/标题/SHA-256 去重、候选资格理由和单次上限。
- 已复用编译中心受控任务、仓库写互斥、事件流、部分失败、重试、日志和生成物审计。
- Python 44/44、Rust 40/40、前端入库状态 4/4、TypeScript/Vite、P3/P4/P5、严格 GUI 与 NSIS 安装生命周期均通过。
- 两本核心书 Recall@5 为 1.000 / 0.986667；Wiki Lint 为 0 error、2 个既有 warning，其中 B 类页面需用户确认后补 `inspired_by`，派生图仍缺 8 个 Wiki source 节点。
- Windows 0.9.0 release、MSI、NSIS 已重新生成并记录 SHA-256。
