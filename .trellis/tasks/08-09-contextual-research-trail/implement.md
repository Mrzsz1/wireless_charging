# 上下文相关研究脉络实施计划

## 阶段 0：冻结基线与建立证据

1. 记录当前 Git 状态、0.7.2 基线提交和现有发布产物。
2. 保存当前占位逻辑的代码锚点与 GUI 截图：`catalog.slice(0,5)`、method catalog 截断、两个无真实动作的 footer 按钮。
3. 执行现有基线门禁：`test:p1`、`test:p2`、build、verify、Rust test、Python 37、Wiki 10 问、两书 295 条。
4. 建立失败样本：打开不同页面时右侧五项完全相同；编号与相关度无关。
5. Git 检查点：`docs(trellis): plan contextual research trail`。

## 阶段 1：先定义契约与失败测试

1. 在 `types.ts` 增加 ResearchContextAnchor、ResearchTrailRequest/Response/Item/Relation 类型。
2. 在 Rust 建立对应 serde struct，全部使用 `rename_all = "camelCase"`。
3. 新增 Rust fixture，至少包含：
   - anchor 页面；
   - 一条出链、一条反链；
   - 一个 FTS source、一个 method；
   - 一个核心书籍章节；
   - 一个最小 Graphify 图与一条邻接边。
4. 先写失败测试：自排除、关系识别、跨通道去重、稳定平分、方法限定、graph 缺失降级。
5. 新增 TypeScript 失败测试：context key、请求序列、pins 损坏恢复、固定项与自动排名分离。
6. Git 检查点：`test(desktop): define research trail contracts`。

## 阶段 2：抽取共享检索基础

1. 审查 `qa.rs::query_terms`、`fts_query`、Wiki/book/graph candidates，确定可共享边界。
2. 优先新建 `retrieval.rs`；迁移中英词项扩展、FTS query escape 和必要候选 DTO。
3. 保持现有 `prepare_question` 排名与输出不变，先运行 P3 问答回归证明无漂移。
4. 禁止复制扩展词表；问答和研究脉络必须调用同一实现。
5. Git 检查点：`refactor(desktop): share local evidence retrieval`。

## 阶段 3：实现 Rust research_trail 模块

1. 实现请求验证和 anchor 加载。
2. 实现页面出链解析：复用现有 ID/stem 解析规则，不自行发明第二套路由。
3. 实现反向链接候选与 anchor 自排除。
4. 实现页面字段词项提取和 Wiki FTS 候选。
5. 实现核心书籍章节候选并保留 physical page 定位。
6. 实现 Graphify source_path→node 映射、一跳邻居和 graphPath。
7. 实现 method-only 候选与字段重合加分。
8. 实现多通道合并、理由合并、归一化、稳定排序、来源多样性与截断。
9. 记录 `degradedChannels`，区分 graph 缺失、books 缺失和基础 Wiki 错误。
10. 运行 Rust tests、fmt、Clippy。
11. Git 检查点：`feat(desktop): build contextual research trail`。

## 阶段 4：注册 Tauri 命令与前端服务

1. 在 `lib.rs` 注册 `prepare_research_trail`，通过当前 repository state 获取 root/connection。
2. 在 `services/desktop.ts` 增加类型安全 invoke wrapper。
3. 更新结构验证脚本，断言命令、服务函数和类型存在。
4. 增加命令级 Rust 测试：未开仓库、缺 pageId、页面不存在、正常响应。
5. 运行 TypeScript build 与 Rust tests。
6. Git 检查点：`feat(desktop): expose research trail command`。

## 阶段 5：实现前端上下文状态机

1. 新建 `researchTrailState.ts`：
   - normalize/question hash/context key；
   - request sequence claim；
   - cache key；
   - pins v1 parser；
   - 自动/固定项合并。
2. 新建 `useResearchTrail` hook；处理 idle/loading/ready/partial/empty/error。
3. 缓存键纳入 repository、context、repositoryGeneration、graphRefreshVersion。
4. 仓库变化时清空可见上下文与 cache；旧请求结果通过 sequence 拒绝。
5. 完成 Node 单元测试并接入 `test:p1` 或独立 `test:research-trail`。
6. Git 检查点：`feat(desktop): manage research trail state`。

## 阶段 6：接入页面、问答和搜索上下文

1. `openPage` 成功后发布 page anchor；页面失效时清理。
2. `AskView` 增加 `onResearchContextChange`：提交问题、打开历史会话、新会话分别发布 question/idle。
3. 文献库/方法库搜索词使用 350ms debounce 发布 search anchor；清空立即 idle。
4. 标签切换时依据目标 view 恢复正确锚点，不延续不可见旧页面。
5. 为 AskView callback 和 anchor 优先级增加纯状态测试。
6. Git 检查点：`feat(desktop): bind research context to active work`。

## 阶段 7：替换研究脉络 UI

1. 把 `App.tsx:595-599` 内联 JSX 提取为 `ResearchTrailPanel`。
2. 删除研究脉络对 `recentPages` / `relatedMethods` 的依赖；Dashboard 最近页面仍可继续使用 `recentPages`。
3. 实现锚点副标题、刷新、收起、loading skeleton、partial、empty、error/retry。
4. 实现证据卡：rank、kind、relation、score、reason、snippet、定位与图谱核验标记。
5. 实现相关方法卡：method family、关系、score、summary。
6. 保持天蓝色主题，验证长中英文标题、长路径和 125%/150% DPI。
7. Git 检查点：`feat(desktop): render auditable research trail`。

## 阶段 8：实现真实“添加证据”

1. 新建 EvidencePicker modal，Wiki/核心书籍并行搜索。
2. 搜索输入至少两字符，使用序列守卫避免乱序。
3. 选择项写入 `desktop.research-trail.pins.v1`；按 repository + contextKey 隔离。
4. 固定项在独立区域显示，支持打开与取消固定；不改自动 rank。
5. 加载时验证资源仍存在，无效项清理。
6. 测试损坏 JSON、重复固定、切库隔离、删除资源。
7. Git 检查点：`feat(desktop): pin local research evidence`。

## 阶段 9：图谱聚焦与来源打开

1. 扩展 `GraphView` props：`targetNodeId` / `targetPath`。
2. “查看完整脉络图”携带 anchor node；Graphify 证据点击携带 item node/path。
3. Wiki 证据调用 `openPage`；book 证据设置 `BookTarget`；graph 映射到 Wiki 时提供正文优先入口。
4. 目标不存在时显示提示并保留可用总图。
5. 添加导航与状态恢复测试。
6. Git 检查点：`feat(desktop): focus graph from research trail`。

## 阶段 10：自动化验收

### 10.1 Rust

```powershell
cd apps/desktop/src-tauri
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

断言：出链、反链、FTS、book、graph、method、去重、稳定排序、自排除、降级矩阵。

### 10.2 前端

```powershell
cd apps/desktop
npm run test:p1
npm run test:p2
npm run test:research-trail
npm run build
npm run verify
npm run verify:p3
npm run verify:p4
npm run verify:p5
```

### 10.3 GUI strict

扩展 `e2e/gui-smoke.mjs`：

1. 打开两个不同 fixture 页面，断言 anchor 与 evidence ID 随页面变化。
2. 断言不再出现固定目录前五项。
3. 提交离线问题，断言面板切换 question anchor。
4. 打开 Wiki/书籍/图谱三类证据。
5. 添加并取消固定证据；重启验证同仓库持久化。
6. 验证 1366×768 与 1920×1080 无横向溢出。

### 10.4 仓库门禁

```powershell
cd ../../..
python -m unittest discover -s tests -p "test_*.py" -v
python -m compileall -q tools tests
python tools/wiki_eval.py
python tools/core_book_eval.py
python tools/wiki_lint.py --strict-graphify
git diff --check
```

## 阶段 11：版本、文档与发布

1. 统一 package/Cargo/Tauri/fallback/verify 版本为 0.8.0。
2. 更新根 `prd.md`，记录 P5.6/研究脉络真实检索决策与验收结果。
3. 更新桌面 README，说明锚点、排名、Graphify 提示、固定证据和离线边界。
4. 新增 `logs/2026-08-09-contextual-research-trail.md`。
5. `npm run tauri build` 生成 EXE、MSI、NSIS；严格安装/启动/退出/卸载。
6. 记录三个产物 SHA-256，并保留 0.7.2 回滚产物。
7. 更新 Graphify 派生图，不覆盖 `wiki/`。

## 阶段 12：Trellis 收口

1. 用 `trellis-check` 对 R1-R7 / AC1-AC13 做逐项映射。
2. 更新 `.trellis/spec/frontend/state-management.md`：上下文优先级、request sequence、cache/pins 契约。
3. 更新后端质量 spec：混合检索排序、Graphify 降级和稳定 tie-break 契约。
4. 按模块提交 Git；派生图单独提交。
5. 归档任务并记录 session journal。
6. 最终要求 `git status --porcelain` 为空。

## 关键风险与回滚点

| 风险 | 预防 | 回滚点 |
|---|---|---|
| 复用 qa.rs 导致问答召回漂移 | 先冻结 P3 fixture，抽取后立即跑回归 | 阶段 2 commit |
| Graphify source_path 无法映射 Wiki | 规范化分隔符、stem fallback、明确 graph-only | 阶段 3 commit |
| 快速切换造成旧结果覆盖 | request sequence 单测 + GUI 快速切换 | 阶段 5 commit |
| pins 跨仓库污染 | repository identity 进入 key | 阶段 8 commit |
| App.tsx 继续膨胀 | 面板、hook、state 全部独立 feature | 阶段 7 commit |
| GUI 小屏不可用 | 面板独立滚动、footer sticky、两分辨率 strict | 阶段 10 gate |
