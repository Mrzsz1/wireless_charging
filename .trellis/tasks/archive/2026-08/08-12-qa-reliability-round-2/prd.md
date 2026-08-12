# 智能问答第二轮可靠性修复

## Goal

修复交接复审确认的 5 个智能问答 P1 缺陷，使多轮检索、零证据处理、请求取消、失败恢复和 Graphify 关系召回形成一致、可测试、可发布的跨层契约，同时保持现有仓库隔离、引用编号、原文定位、书籍页码和凭据边界。

## Background

- 当前客户端版本为 `0.12.2`，上一阶段已完成受限会话历史、引用编号校验、意图加权、repository generation 隔离、completed 幂等、按需 Codex 探测和 Graphify Wiki 回链过滤。
- 复审确认以下缺陷仍存在：
  - `QA-P1-01`：`apps/desktop/src-tauri/src/lib.rs:2586` 在读取历史前调用检索，`apps/desktop/src-tauri/src/qa.rs:1334-1351` 只接收当前问题，多轮指代不会参与召回。
  - `QA-P1-02`：`apps/desktop/src-tauri/src/qa.rs:1556` 在证据为空时允许无引用普通回答通过。
  - `QA-P1-03`：`apps/desktop/src-tauri/src/lib.rs:2568-2609` 在 Tauri async 命令内持有 repository mutex 执行同步检索；`apps/desktop/src-tauri/src/lib.rs:2654-2668` 检索和 Codex 探测后才登记取消；`apps/desktop/src/features/qa/AskView.tsx:306` 在收到后端 request ID 前禁用停止。
  - `QA-P1-04`：`apps/desktop/src-tauri/src/qa.rs:1891-1944` 只向既有会话保存 failed assistant；首轮失败无记录，`apps/desktop/src/features/qa/completionState.ts:38-43` 也无法从失败 assistant 找到原问题。
  - `QA-P1-05`：`apps/desktop/src-tauri/src/qa.rs:1212-1218` 只以中心节点 label/source_file 产生初始命中，边关系、邻居、community 和 source_location 仅用于命中后的展示。
- 当前自动化基线：前端 P1 11/11、QA Settings 4/4、Rust QA 13/13、P3 Rust 63/63、Wiki/论文契约 10/10、两书 Recall@5 为 1.000000 / 0.986667。

## Requirements

### R1 — 历史感知检索

- 在检索前加载同一 repository、同一 session 的受限 completed 历史。
- 构造显式 `RetrievalQuery`，至少保留原问题、解析后问题、实体、意图和使用到的历史消息标识。
- 采用确定性、可测试的指代补全；不得把完整 assistant 历史直接拼入 FTS。
- 历史仅用于 query rewrite 和 Prompt 指代理解，不得成为本轮 evidence，历史 `[E#]` 不得沿用。

### R2 — 零证据无来源回答语义

- 当 `evidence=[]` 时，Codex 或兼容 API 可以继续生成基于模型一般知识的回答，但必须在回答首部和 UI 状态中明确说明“当前知识库没有参考来源，以下内容未由本库证据支持”。
- 零证据回答不得伪造 `[E#]`、wikilink、论文位置或书籍页码，不得显示为“有证据支持”的成功回答。
- 零证据模型回答保存为独立的 `unverified` 状态，可重新打开和重试，但从 `conversation_history()`、后续 query rewrite 和后续 Prompt 中排除。
- 用户选择离线证据模式时没有生成模型，只返回确定性的“当前知识库没有参考来源”提示；Codex 与兼容 API才生成无来源回答。
- `validate_citations` 必须区分“有证据且引用有效”“零证据且已明确标注无来源”和“零证据却伪装为有依据回答”。

### R3 — 可立即取消的非阻塞请求生命周期

- 前端在提交前生成非空 request ID，随 `AskRequest` 发送；后端拒绝空 ID和重复 active ID。
- 提交后立即进入可停止状态；后端进入命令后立即登记 cancel flag 并发送 `started`，随后才执行历史、检索和 Codex 状态探测。
- SQLite 检索和 Graphify 解析进入 blocking worker，不长期占用 Tauri async executor 或 repository mutex。
- 检索通道之间、Codex 状态探测前后、生成前后和持久化前检查取消与 repository identity。
- Graphify 使用按 repository + 文件版本失效的内存缓存；Codex ask-time 状态使用短 TTL 缓存，设置页显式刷新仍可绕过缓存。
- retrieval 中取消不生成、不落库、不保留幽灵消息；切换知识库能取消尚未生成的请求。

### R4 — 失败交换持久化与精确重试

- 首轮和既有会话失败都保存成对记录：原始 user 问题与 failed assistant 错误，两者携带同一 request ID。
- failed/cancelled 消息不得进入 `conversation_history()`。
- 当前 UI 与重新打开的历史都能看到失败问题和错误；失败 assistant 的重试必须使用对应的确切 user 内容。
- 取消不保存失败交换；repository 已切换的旧请求不得写入新仓库。

### R5 — Graphify 关系召回

- Graphify 初始搜索文档至少覆盖节点 label/description、来源页标题、source_location、community/community_name、边 relation 和一跳邻居 label。
- 分别计算 node hit、relation hit 和 neighbor hit，使 relation-only 与 neighbor-only 查询可产生候选和可解释理由。
- 候选必须继续映射到真实且已索引的 `wiki/**/*.md` 页面；无 Wiki 回链节点不得进入最终 evidence。
- Graphify 始终为 `graph_hint`，不能单独支撑事实结论。

### R6 — 兼容与发布

- Rust、TypeScript、Tauri IPC、流事件、SQLite request ID 与失败 DTO 保持一致。
- 保持 completed event / invoke result 幂等、repository 三段隔离、论文 `sourceLocation`、书籍 physical pages、provider 失败不伪装离线、Codex/API 凭据不读取不持久化。
- 不修改 `raw/`、`wiki/`、`schema/vocab.yaml` 正文，不新增 `wiki/problems` 或 `wiki/ideas`，不默认外搜。
- 以 patch 版本 `0.12.3` 完成交付：实现、测试、Git 提交、Release 构建、安装和启动验证。

## Acceptance Criteria

- [ ] `CCSP/GAIN` 首问后追问“它们的约束有什么区别”能同时召回两者的预期 Wiki 与 primary paper；query rewrite 可观察且受历史预算限制。
- [ ] 旧 `[E#]` 不进入新 evidence，failed/cancelled 历史不进入 rewrite 或 Prompt。
- [ ] `evidence=[]` 时 Codex/API 输出带固定无来源声明的模型一般知识回答，离线模式输出确定性无来源提示；三者均无伪造引用且不显示为有证据支持。
- [ ] 零证据回答可在历史中重新打开，但不进入下一轮 query rewrite、Prompt 或 conversation history。
- [ ] 发送后立即可停止；retrieval 中取消不生成、不保存、不留下 UI 幽灵消息。
- [ ] 大型 graph fixture 检索期间，轻量 Tauri command 不被 repository mutex 长时间阻塞；Graphify 缓存按文件变化失效。
- [ ] 空 request ID 和重复 active request ID 被拒绝；repository switch 仍隔离旧事件、旧结果和旧写入。
- [ ] 首轮失败可创建可恢复会话；历史显示成对失败交换；重试使用确切原问题。
- [ ] relation-only、neighbor-only 和 community/source-location 查询能召回正确 Graphify 中心节点；无 Wiki 回链节点仍被过滤。
- [ ] 现有 Wiki/paper 10/10 契约、两书 Recall@5 ≥ 0.95、前端 P1/QA settings 和全部 Rust 回归不退化。
- [ ] `npm run build`、`npm run verify:p3`、`npm run verify:p5`、`cargo fmt --check`、`cargo clippy --all-targets -- -D warnings`、`cargo test`、`py -3 tools/wiki_lint.py` 与 `git diff --check` 通过。
- [ ] 0.12.3 的版本声明、更新 fixture 和验证脚本同步；MSI/NSIS 构建成功，NSIS 安装后的注册表版本、可执行文件版本和主窗口启动探针通过。

## Out of Scope

- embedding/BM25 混合检索、score normalization、MMR、动态 evidence budget。
- Markdown/KaTeX/表格/代码块渲染和引用面板重设计。
- claim-level NLI、引用覆盖率仪表盘、nDCG/延迟仪表盘。
- 会话服务端分页/导出、更多 provider adapter、外部论文搜索接入问答。
- 重构整个客户端或修改知识库内容治理。
