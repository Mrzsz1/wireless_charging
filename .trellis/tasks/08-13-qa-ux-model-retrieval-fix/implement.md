# 实施计划：智能问答交互、Codex 模型与论文检索修复

## 0. 执行方式与门禁

本任务使用 Codex inline 模式，由主会话直接实现与检查，不创建 implement/check 子代理。用户批准本计划后才运行 `task.py start`。每组先增加失败回归，再修改实现；严格引用、repository 隔离、取消或凭据边界任一退化时停止后续编译。

## 1. 读取 Trellis 开发规范并建立基线

- [ ] 加载 frontend/backend/qa-contract、cross-layer、code-reuse 与 quality guides。
- [ ] 记录 `npm run test:qa-settings`、QA Rust定向测试、`npm run build` 当前基线。
- [ ] 确认用户未跟踪的 `智能问答交接文档-2026-08-12.md` 不纳入本任务。

## 2. Codex 模型目录与配置投影

- [ ] 在 `codex_subscription.rs` 定义 secret-free model option/config projection。
- [ ] 解析 `CODEX_HOME/config.toml` 的 allowlist 字段和 `models_cache.json` 的 list-visible 模型，处理缺失/损坏/fallback。
- [ ] 扩展 status DTO；补模型目录、reasoning effort、排序去重和敏感字段测试。
- [ ] 保持现有登录探测和状态 TTL，不让目录失败改变 authenticated/ready。

## 3. 设置持久化、受控执行与 UI 选择

- [ ] `QaSettings/LunaSettings` 增加 `codexReasoningEffort`，读取/校验/保存 SQLite key 并保持旧库默认。
- [ ] 计算 effective model/effort；扩展 Codex stream API和 exec args，保留 ignore-user-config/rules/read-only。
- [ ] SettingsView 把自由文本升级为 auto + 动态模型 select、auto + 能力约束 effort select；保留未发现旧 override。
- [ ] AskView 状态胶囊显示实际自动选择；更新 types/default fixtures和 QA settings tests。

主要文件：

```text
apps/desktop/src-tauri/src/codex_subscription.rs
apps/desktop/src-tauri/src/qa.rs
apps/desktop/src-tauri/src/lib.rs
apps/desktop/src/types.ts
apps/desktop/src/features/settings/SettingsView.tsx
apps/desktop/src/features/qa/AskView.tsx
apps/desktop/tests/qa-provider-settings.test.ts
```

## 4. 引用 canonicalization

- [ ] 增加统一 `ParsedAnswer`/等价内部结果，由同一入口顺序执行 Markdown mask、引用规范化、章节/事实解析、完整性和 grounding 校验。
- [ ] 在 grounding Markdown mask 边界内实现复合引用解析，仅接受本轮已知 ID。
- [ ] 规范化中文/英文分隔符和单 ID 位置说明；拒绝范围、未知 ID及隐藏 Markdown 区域。
- [ ] 扩展 `CitationRepair` 与 TS manifest类型，旧 JSON字段缺失用默认值。
- [ ] 将规范化接入未知引用删除之前，验证后仍执行逐事实同句门禁。
- [ ] 确保 audit、SQLite、completed event 和前端渲染只消费规范化 Markdown，补重新打开历史的一致性回归。
- [ ] 补真实失败样式 fixture和所有负向测试。

## 5. Literature intent 与波干扰召回

- [ ] 新增 literature 分类及与 novelty/relationship 的优先级测试。
- [ ] 为 literature 定义四段回答 schema、必填元素和最低事实数；升级 prompt/answer schema版本。
- [ ] Prompt 明确独立 `[E#]` 及位置写法，避免复合方括号。
- [ ] curated query alias 同时覆盖“波干扰/波干涉”，添加目标论文排名回归。
- [ ] 使用本地应用数据库或等价导出 fixture验证“有没有关于波干扰的论文”命中并通过最终审计。

主要文件：

```text
apps/desktop/src-tauri/src/qa.rs
apps/desktop/src-tauri/src/qa/context.rs
apps/desktop/src-tauri/src/qa/grounding.rs
apps/desktop/src/types.ts
```

## 6. Thinking 事件、计时与阶段 UI

- [ ] 增加 `validation_started` Rust/TS Channel事件并在 grounding/persist 前发送。
- [ ] 抽取可测试的 phase-to-steps 投影和统一 generation reset；覆盖 first token、完成、失败、取消、重试、切库和 stale event。
- [ ] AskView 增加单调计时器、`Thinking · Ns`、处理链和 validation 阶段。
- [ ] 增加 pulse/check/waiting 动画、`aria-live` 与 reduced-motion。
- [ ] 保持流式 Markdown 与 citation projection不变。

## 7. Composer 紧凑布局与自动增长

- [ ] 用显式 grid row固定 heading/error/messages/composer。
- [ ] textarea 初始 3 行、最大约 8 行；输入时按 scrollHeight增长，清空/提交复位，溢出内部滚动。
- [ ] 验证无错误、有错误、生成停止、窄屏和长问题输入。
- [ ] 添加源契约或 DOM 测试，防止再次用条件子节点依赖自动 grid placement。

## 8. Trellis 契约更新

- [ ] 更新 `.trellis/spec/backend/qa-contract.md`：Codex自动模型投影、独立引用语法、literature schema、validation event。
- [ ] 在 frontend spec记录条件错误行与 composer显式布局、Thinking阶段事件契约。
- [ ] 仅记录稳定约定，不修改 Wiki/Raw/Graphify正文。

## 9. 定向验证

在 `apps/desktop`：

```powershell
npm run test:qa-settings
npm run test:p1
npm run build
cargo test codex_subscription::tests --manifest-path src-tauri/Cargo.toml
cargo test grounding::tests --manifest-path src-tauri/Cargo.toml
cargo test qa::tests --manifest-path src-tauri/Cargo.toml
```

重点验收：模型目录与effort、exec隔离参数、复合引用规范化、literature意图、波干扰论文命中、validation事件、timer/reset及composer四行布局。

## 10. 全量质量门禁

```powershell
cd apps/desktop
npm run verify:p3
npm run verify:p5

cd src-tauri
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test

cd ../../..
py -3 tools/wiki_lint.py
git diff --check
git status --short
```

保留 Wiki/paper gold contract、两书 Recall@5 ≥ 0.95、准确率 eval、Markdown引用边界、repository isolation、cancel/failed exchange回归。

## 11. 差异、秘密与真实场景审查

- [ ] 审查完整 diff和 staged列表，确认未触碰 raw/wiki/vocab/B类页面。
- [ ] 扫描 token/cookie/API key/authorization/password/secret；确认 status/manifest/log仅含模型元数据。
- [ ] 用本机 Codex 配置验证自动识别列表和 xhigh显示，不输出完整配置。
- [ ] 用“有没有关于波干扰的论文”进行真实端到端问答，核对答案、引用按钮、证据面板和阶段计时。

## 12. Git、编译与启动验证

- [ ] 所有门禁通过后提交 Git；不提交生成产物和用户交接文档。
- [ ] 编译客户端可运行产物；如现有发布流程要求 patch版本，则同步为 `0.12.4` 后构建。
- [ ] 启动验证模型/effort选择、紧凑composer、Thinking计时和波干扰查询。
- [ ] 记录提交哈希、产物路径与验证结果，完成 journal/任务归档。

## 13. 当前执行结果

- [x] 统一引用 canonicalization、literature schema 与波干扰真实仓库召回回归完成。
- [x] Codex 模型目录、自动/显式模型和 reasoning effort 选择完成；受控 exec 参数与 secret-free DTO 测试通过。
- [x] Thinking 阶段事件、实时计时、显式 validation 阶段与紧凑 composer 完成。
- [x] Trellis backend/frontend 稳定契约已同步。
- [x] 前端 build、QA Settings 5/5、P1 17/17、Rust 96/96、fmt、Clippy、P3/P5、GUI 探针、Wiki lint 和准确率审计测试通过。
- [ ] Git 提交、0.12.4 release 构建和启动验证待最后执行。
