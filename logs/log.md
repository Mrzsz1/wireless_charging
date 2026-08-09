# 知识库时间线 log

> Karpathy 式 append-only 日志。每条以统一前缀开头，便于 `grep`。  
> 格式：`## [YYYY-MM-DD] <kind> | <title>`  
> kind：`scaffold` | `ingest` | `query` | `lint` | `graphify` | `schema` | `vocab` | `note`

## [2026-07-10] scaffold | Vault 骨架落地

- 建立 raw / wiki / schema / templates / logs
- 写入 prd、词表种子、Claudian 模板、主题 maps
- 对齐 Karpathy gist 与 Graphify 规程（AGENTS.md、index、本 log）

## [2026-07-10] schema | 引入参考源

- Karpathy: https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f
- Graphify: https://github.com/Graphify-Labs/graphify
- 本地：`schema/references/karpathy-llm-wiki.md`、`schema/references/graphify.md`

## [2026-07-10] graphify | 安装与项目注册

- CLI：`graphifyy` 0.9.11（`uv tool install`）
- `graphify install --project` → `.claude/skills/graphify/`
- `graphify cursor install` → `.cursor/rules/graphify.mdc`（可选）
- 尝试 `graphify extract .`：无 API key 失败（预期）

## [2026-07-10] graphify | 对齐用户工具 Codex + Grok

- 用户主用 **Codex CLI** 与 **Grok CLI**，不强制 Cursor
- `graphify install --platform codex --project` → `.codex/skills/graphify/`
- `graphify install --platform agents --project` → `.agents/skills/graphify/`
- 文档默认入口改为：Codex `$graphify .` / Grok 走 Agent Skills

## [2026-07-10] graphify | 用户确认首图成功

- `graphify-out/graph.json` / `graph.html` / `GRAPH_REPORT.md` 已生成
- 下一阶段：导入 PDF + MinerU + A 编译（canonical 尚无论文）

## [2026-07-10] schema | MinerU 图片：一文一夹

- 问题：MinerU md 配 `images/`，平铺会撞名断链
- 决定：`raw/canonical/<slug>/{.md,images/,.pdf?,.html?}`；A 编译主源仍是 md
- HTML 可选人读，不替代 md；graphify 忽略 `**/images/**` 与 html

## [2026-07-10] note | 写入使用说明.md

- 日常问库 / 养库 / 锻 idea / 工具分工 / 成功标准
- 入口：`使用说明.md`；`HOME.md` 已链入

## [2026-07-10] ingest | 首批 9 篇 A 编译

- raw：`raw/canonical` 下 9 夹，主读 `full.md`
- 新建 sources×9、concepts×5、methods×8、synthesis×1
- 更新 index、library-status、主题 maps
- 边界：`src-alzenad-uav-bs-qos`（UAV-BS 非 WPT）标 needs_review
- Graphify：待用户 `$graphify . --update`
- 详情：`logs/2026-07-10-ingest-batch1.md`

## [2026-07-14] note | 补充系统架构文档

- 新增 `ARCHITECTURE.md`：分层、权威顺序、组件、数据契约、Ingest/Query/B/Lint 流程与当前水位
- `HOME.md`、`使用说明.md` 增加架构入口
- `.graphifyignore` 排除架构/使用说明及多平台 skill 注册目录，避免工具元数据继续污染领域图
- 记录当前漂移：Graphify 旧图含 skill 节点；raw 主转换稿未检出 `ingest_status`
- 详情：`logs/2026-07-14-architecture.md`

## [2026-07-14] note | 接入 MinerU API 自动解析

- 新增 `tools/mineru_to_md.py` 与 PowerShell 入口：批量上传本地 PDF、轮询、下载并安全解压 MinerU 结果
- 默认读取 `E:\知识库\aoikey.txt`；token 不写入项目、日志或 OSS 上传请求
- 输出遵循 `raw/canonical` 一文一夹，并给 `full.md` 添加 `pending_ingest` frontmatter
- 新增无网络单元测试与使用说明；只读鉴权探测通过，不会自动写 wiki
- 详情：`logs/2026-07-14-mineru-api.md`

## [2026-07-14] note | 增加 arXiv / OpenAlex 论文自动发现

- 新增 `tools/paper_search.py`、PowerShell 入口、无线充电调度主题预设与可选每日定时任务
- 输出严格停在 `raw/inbox/search-*`：带抓取时间、跨源去重、透明排序和 candidate 边界，不自动进入 canonical/wiki/Graphify
- 默认 arXiv；配置独立 OpenAlex key 后自动启用 OpenAlex；开放 PDF 下载需显式开启
- 真实 arXiv 验证得到 25 条原始命中、23 条去重候选；增量模式后续识别 0 条未见项
- Python 测试 11/11、脚本编译与 PowerShell 语法检查通过
- 详情：`logs/2026-07-14-paper-search.md`

## [2026-07-14] note | 论文发现扩展为四源

- 自动读取共享配置中的 OpenAlex、Tavily、SerpApi Key；Google Scholar 通过 SerpApi `google_scholar` 引擎接入
- Tavily 只检索学术域名白名单；四源结果统一去重、排序并停在 `raw/inbox`
- MinerU 兼容共享多 Key 文件，旧版首行 Token 仍可读取
- 四源真实检索：OpenAlex 5、Tavily 5、Google Scholar 5，0 错误；arXiv 的默认主题检索此前已验证
- 测试 15/15；报告与缓存 Key 泄漏扫描为 0
- 详情：`logs/2026-07-14-paper-search-multisource.md`

## [2026-07-14] architecture | LLM Wiki 来源追踪与双入口优化

- 将 `manual/auto`（采集来源）与 `inbox/canonical/ingested`（生命周期）拆为正交维度
- 新目录：`raw/inbox/auto-discovered/{runs,papers}` 与 `raw/inbox/manual-drop`
- 3 次历史检索报告已迁移，38 条候选逐条补齐 provenance，当前全部 `pending`
- 新增 `paper_triage.py`：可记录 selected/rejected，并建立已选候选元数据队列；不自动晋升
- MinerU 可从路径/sidecar 推断并传播 `manual_upload` / `auto_discovery`
- 9 篇现有 raw 主稿与 9 个 source 已补齐 `manual_upload` provenance；raw 状态统一为 `ingested`
- 更新 PRD、schema、templates、index、library-status、架构与使用文档；测试 19/19
- Graphify 代码更新不适合本 Markdown 主库，已从自动备份恢复 174 nodes / 248 edges；语义图待 LLM backend 全量重建
- 详情：`logs/2026-07-14-llm-wiki-provenance-architecture.md`

## [2026-07-14] tooling | Windows 桌面快捷启动

- 新增 `tools/launch-wiki.ps1`：优先用 Obsidian 打开 `HOME.md`，否则回退默认 Markdown 程序
- 新增 `tools/create-wiki-shortcut.ps1`：生成“无线充电 LLM Wiki”桌面快捷方式
- `HOME.md` 与 `使用说明.md` 已补充快捷启动入口
- 已创建并核验：`C:\Users\qq155\OneDrive\Desktop\无线充电 LLM Wiki.lnk`

## [2026-07-14] ingest | 内容密度与元数据闭环

- 本地 PDF 核验 9/9 source 的 year、venue、DOI；raw 只改 frontmatter，不改正文
- synthesis 从 1 增至 3：新增干涉感知路线、移动/在线服务路线
- 更新 index、library-status、4 个既有 map 与 source 反链
- 详情：`logs/2026-07-14-llm-wiki-content-loop.md`

## [2026-07-14] lint | 页面准入与 10 条问答回归契约

- 增加 A 类页面准入规则，限制单源低复用拆页
- 新建 5 solve + 3 novelty + 2 relationship 回归用例及确定性校验工具
- 候选初筛只产出建议，不改变 38 pending / 0 selected 的人工状态
- 详情：`evals/README.md`、`logs/2026-07-14-auto-candidate-triage-recommendations.md`

## [2026-07-14] note | A→B 联合部署与在线干涉草案

- 基于 CCSP、GAIN、TIDE 与两篇 synthesis 形成 problem/idea 审阅稿
- 未写入正式 `wiki/problems` / `wiki/ideas`，等待用户确认具体草案
- 草案：`logs/2026-07-14-ab-pilot-review-draft.md`

## [2026-07-14] lint | 内容闭环验收

- 0 断链、0 非导航孤儿页、0 A/B 贡献句命中
- 10 条问答契约通过；单元测试 22/22；Python 编译通过
- Graphify 语义图因无 LLM backend 未重建，wiki 正文保持权威
- 报告：`logs/2026-07-14-lint-content-loop.md`

## [2026-07-14] triage | 自动发现候选首轮正式裁决

- 用户授权 Agent 代为决定；38 条候选变为 14 pending / 10 selected / 14 rejected
- selected 只物化 10 份 metadata，未下载 PDF、晋升 canonical 或进入 wiki 证据层
- 详情：`logs/2026-07-14-triage-b-problem-domain-keywords.md`

## [2026-07-14] schema | 论文关键词三层治理

- 9 个 source 增加 `paper_keywords` / `keyword_source`；8/9 有 Index Terms，32 次出现
- 新建 `map-domain-keywords`、`schema/domain-keywords.md` 与只读检查工具
- 关键词不直接写 `vocab.yaml`；本轮无新 vocab proposal

## [2026-07-14] note | 首个正式 B research problem

- 用户授权将联合部署与在线干涉调度草案正式化为 problem
- 算法 idea 因硬件动作与公平定义未锁定而暂缓
- 页面：`wiki/problems/prob-joint-deployment-online-interference.md`

## [2026-07-14] lint | 候选、关键词与 B problem 验收

- 候选统计一致；关键词检查通过；Wiki 断链 0；A 类贡献句 0
- Query 契约 10/10；单元测试 24/24
- Graphify 仍为旧语义快照，待文档 backend 后受控全量刷新
- 报告：`logs/2026-07-14-lint-triage-keywords-b.md`

## [2026-07-14] ingest | 自动发现 PDF 下载与 MinerU 7/7

- 10 条首选候选中取得 7 篇合法公开 PDF；签名与首页标题校验通过
- MinerU batch 完成 7、失败 0；canonical 状态为 auto_discovery + promoted + pending_ingest
- 其余 3 篇经多源复核仍为 closed access，未绕过付费墙
- 修复 selected `paper.pdf` 导致 canonical 目录名退化及重复运行误建后缀目录的问题；26/26 测试通过
- 详情：`logs/2026-07-14-auto-download-mineru.md`

## [2026-08-01] ingest | 自动发现 7 篇文献完成 A 编译

- 触发原因：完成 P0 文献编译闭环
- 读取 raw：7 个 `raw/canonical/*/full.md`，均为 `promoted + pending_ingest`，转换失败数为 0
- 新建 source：7 个 `wiki/sources/src-*.md`
- 新建 A 类页：2 concepts、7 methods、2 syntheses
- 更新：`wiki/index.md`、`wiki/maps/library-status.md`、5 个主题/关键词地图
- raw：7 个主稿 frontmatter 的 `ingest_status` 更新为 `ingested`；正文未改
- 关键词：15/16 source 有作者 Keywords / Index Terms，66 次出现
- 问答：保存 10 份回归答案；`wiki_eval --answers-dir` 通过
- 待用户确认：`vocab.yaml` 仍为 `draft_seed`，未自动冻结；无 B 类页面写入
- 详情：`logs/2026-08-01-p0-ingest.md`

## [2026-08-01] graphify | P0 文档图重建

- 使用当前 `.graphifyignore`，排除 `.agents/.codex/.claude/.cursor/.codegraph/tools/tests` 等元数据和工具目录。
- 以 `wiki/**/*.md` 与 `raw/canonical/*/full.md` 做结构化Markdown抽取并运行 `graphify cluster-only`。
- 结果：798 nodes / 820 links / 63 communities；工具噪声节点 0。
- 典型领域查询已命中新增的 RA-DMCS、CUAV、DWPT 和三维UAV页面。
- 当前图为结构化 EXTRACTED 边（无 LLM semantic backend，0% INFERRED）；后续配置Graphify文档backend后可补充语义边。
- 旧快照保留于 `graphify-out/graph-pre-p0-20260801.json`，Graphify自动备份位于 `graphify-out/2026-08-01/`。

## [2026-08-01] ingest | 两本核心算法专著章节化入库

- 用户指定 `Algorithmic game theory-book.pdf` 与 `Approximation Algorithms-book.pdf`；inbox 中的 `PDF_B.pdf` 通过版权页/书签确认是 Algorithmic Game Theory，`PDF_A.pdf` 确认是 Approximation Algorithms。
- 原始 PDF 保留不改；在 `work/core-books/inputs/` 做解密/工作副本，在 `work/core-books/*/parts/` 按章节拆成不超过 180 页的 MinerU 请求。
- MinerU 精确解析完成 61 个 parts（近似算法 31 个、算法博弈 30 个）；章节合并到 `raw/canonical/<book-id>/chapters/`，同时保留 `mineru/` 语义层。
- 为解决数学符号和 born-digital 文本的页级准确性，检索正文采用 Poppler page-faithful Markdown，MinerU 结果作为语义/图表复核层；未改 raw PDF。
- 质量门禁：两书 61 个章节均覆盖；最小 token recall 1.000；最小 token precision 0.956444（Approximation Algorithms）和 0.986173（Algorithmic Game Theory），均超过 95%。报告：`raw/canonical/core-books-quality.json`。
- 新增 `source_type: book`、书目 frontmatter、两张 source 页、核心专著综合页、检索注册表和 `tools/core_reference_search.py`；domain keyword 统计明确排除 book source，不影响 16 篇论文指标。
- 新增 3 个核心书测试；全套 unittest **30/30 通过**。
- 检索仍为确定性词法首轮；`evals/core-book-retrieval.json` 已放入 5 条种子查询，达到每本书 100 条人工复核后再宣称 Recall@5 ≥95%。

## [2026-08-01] graphify | 核心专著独立图

- `tools/build_core_book_graph.py` 生成 `graphify-out/core-books/graph.json`：2 books、61 chapters、75 nodes、376 links。
- 主 WPT graph 的 `.graphifyignore` 排除核心书章节和 MinerU staging，避免 61 个章节污染调度图；主图重新更新为 651 nodes / 624 edges / 78 communities。
- 查询专著正文使用 `tools/core_reference_search.py`；图仅用于章节关系导航。
- 检索回归：`evals/core-book-retrieval-report.json` 共 295 条章节种子查询；Algorithmic Game Theory Recall@5 **1.000**，Approximation Algorithms Recall@5 **0.986667**，均通过 95%。

- 最终验证：`py -3 -m unittest discover -s tests -p 'test_*.py'` **31/31 OK**；`domain_keywords.py` 15/16、66 occurrences；`wiki_eval.py` 10/10。

## [2026-08-01] ingest | 最新文献自动发现、下载与A编译

- 5主题×4来源：373原始结果，去重及2025+过滤后269；Top 40进入本轮报告。
- 选择8项；5项开放PDF完成MinerU（5/5）和A编译，3项下载受站点限制保持selected。
- 新增5 source、5 method、1 synthesis；总水位23 source（21 papers/preprints + 2 books）。
- 最新全文：ISAC-Enabled On-Demand UAV Charging，arXiv v1 2026-07-26。
- 验证：关键词20/21、wiki eval 10/10、unittest 31/31。
- 详情：`logs/2026-08-01-2143-latest-literature-ingest.md`

## [2026-08-01] graphify | 最新文献增量图更新

- 执行 `graphify update . --force`。
- 主图更新为805 nodes / 794 edges / 77 communities。
- 查询已命中DICCS、DCHSA/ADTSA、IHATRPO、ISAC部分充电与有障碍多MCV的新source/method/synthesis。

## [2026-08-02] desktop | P3 Luna 证据优先智能问答 0.5.0

- 详细计划写入 `design/p3-luna-qa-plan.md`，并由 3 个默认子代理分别核验前端、Rust/SQLite 与质量门禁现状。
- 新增问答 SQLite migration、按仓库隔离的会话/消息/证据历史、Wiki/核心书籍/Graphify 多路召回、来源多样性与库水位快照。
- 新增 OpenAI-compatible Luna SSE 流式适配；API Key 只读取环境变量，未配置或联网失败时返回可审计离线证据包。
- “智能问答”导航升级为三列真实工作区：会话历史、流式消息、动态 `[E#]` 引用与来源定位。
- 验收：`npm run build`、`npm run verify`、`cargo test` 12/12、Wiki Eval 10/10、核心书籍 295 条评测均通过；Tauri 0.5.0 MSI/NSIS 构建及 release 隐藏启动冒烟通过。
- 详情：`logs/2026-08-02-desktop-p3-luna.md`

## [2026-08-02] desktop | P4 编译中心与工作区可靠性 0.6.0

- 修复选择知识库后文献库不刷新及“我的空间”目录不可展开。
- 接入固定允许列表任务、SQLite 任务历史、实时 stdout/stderr、取消、重试、仓库隔离、日志脱敏与生成物入口。
- 验收：`npm run verify:p4`、Rust 16/16、Python 31/31、Wiki 10/10、两书 Recall@5、Clippy、Tauri release 与隐藏启动冒烟全部通过。
- 详情：`logs/2026-08-02-desktop-p4-compile-center.md`

## [2026-08-09] desktop | 文献入库与两级自动化 0.9.0

- 新增“手动添加 / 待确认 / 自动添加”客户端入口；确认添加执行完整入库，仅下载保持 inbox 候选身份。
- 启动询问支持本次运行、今天不再提醒、取消；默认自动准备，显式开启后才允许严格合格候选自动正式入库。
- 新增可信运行清单、PDF 哈希预检、候选稳定 ID/资格原因、SQLite 设置与 session、5 类可审计编译任务。
- Rust 39/39、前端构建与门禁、strict GUI、NSIS 安装生命周期通过；发布 app/MSI/NSIS 0.9.0。
- 详情：`logs/2026-08-09-desktop-literature-ingest.md`

## [2026-08-09] desktop | 文献运行时与全局搜索修复 0.9.1

- 修复文献自动检索阻塞 Tauri 命令线程、弹出 `py.exe` 以及中文 JSON 在 GBK 控制台触发 `UnicodeEncodeError`。
- 长任务与候选操作改为阻塞线程池；内部 Windows 子进程统一隐藏，Python 固定 UTF-8。
- 修复 FTS5 `snippet()` 四参数误用；真实 release GUI 已执行 `curr` 搜索并通过。
- 验收：Rust 42/42、Python 45/45、Wiki 10/10、两书 Recall@5 1.000 / 0.986667，strict GUI 与 NSIS 生命周期通过。
- 详情：`logs/2026-08-09-desktop-runtime-search-bugs.md`

## [2026-08-09] desktop | 设置集中管理、搜索凭据与分页 0.10.0

- 文献自动化配置迁入统一设置页，自动添加页保留运行状态和“前往设置”入口。
- 新增 arXiv、OpenAlex、Tavily、Google Scholar（SerpApi）配置；Key 使用 Windows Credential Manager，前端不回显，只有受控检索子进程接收。
- 文献库、方法库与搜索结果新增 10/20/50 分页，并覆盖筛选重置、越界收敛和空结果。
- 验收：Rust 45/45、Python 45/45、Wiki 10/10、两书 Recall@5 1.000 / 0.986667，P5 strict GUI 与 NSIS 生命周期通过。
- 详情：`logs/2026-08-09-desktop-settings-pagination.md`

## [2026-08-09] desktop | ChatGPT/Codex 订阅问答与设置收口 0.11.0

- 设置页统一提供 Codex 订阅、兼容 API、离线证据三种回答引擎；AskView 删除连接参数弹窗并定位到全局设置。
- Codex 订阅复用官方 ChatGPT 登录状态；证据经 stdin 进入只读、一次性、隔离的 `codex exec`，取消/超时终止完整进程树。
- fake Codex fixture 覆盖状态、登录启动、JSONL、错误脱敏、超时与取消；自动验证未消耗真实订阅额度。
- 验收：Rust 53/53、Python 45/45、Wiki 10/10、两书 Recall@5 1.000 / 0.986667、P5、最终 release strict GUI 与 NSIS 安装生命周期通过。
- 详情：`logs/2026-08-09-desktop-subscription-qa.md`
## [2026-08-02] desktop | P5.3 ????????? 0.7.0

- ?????/??/??/????? `Ctrl+K`??? Wiki ???? upsert/delete/rename?
- ?? full pipeline fixture?Graphify ??????????????? updater ??????? manifest fixture?
- `verify:p5:strict` ???? GUI ???? NSIS ??/??/???Python 37/37?Rust 23/23?Wiki 10/10??? Recall@5 ??? 95%?
- Graphify 1633 nodes / 2525 links / 146 communities????`logs/2026-08-02-desktop-p53-release-hardening.md`?
