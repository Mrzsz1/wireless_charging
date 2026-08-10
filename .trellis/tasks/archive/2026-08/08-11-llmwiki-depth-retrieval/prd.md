# 深化 LLM Wiki 结构与检索

## Goal

把当前“目录 + 摘要卡”形态的 LLM Wiki 升级为可供研究者阅读、可供桌面端按章节召回、并能回到原文核验的研究知识库。先关闭会直接造成遗漏或错误水位的 P0，再完成可复用系统模型、目标与实验协议的 P1 知识层。

## Background

- 2026-08-11 审查确认：当前 66 个 Wiki 页面中有 23 个 source、20 个 method、7 个 concept、7 个 synthesis、8 个 map、1 个 problem，但 `system-model`、`objective`、`dataset-or-sim` 均为 0。
- method 正文中位数约 167 字符、source 正文中位数约 550 字符；大部分页面缺少变量、约束、算法步骤、复杂度、理论保证、实验设置和原文位置。
- 21 篇论文 source 摘要正文总字符约为对应 raw Markdown 的 0.81%；桌面端只索引 Wiki 正文和两本核心书籍，论文 `raw_md` 不在正常问答检索链中。
- `wiki/maps/map-home.md` 仍显示 16 sources / 8 methods / 5 syntheses，实际水位为 23 / 20 / 7；评测答案也保留旧的 16-source 水位。
- Wiki Lint 为 0 errors / 2 warnings；Graphify 缺少 8 个现有 source 页面，关系召回不完整。
- 现有 10 问评测主要检查 wikilink、水位和关键词，不验证原文位置、必要约束覆盖与无证据陈述。

## Requirements

### R1 审查与产品文档

- 把上述事实、影响、目标架构和优先级写入独立审查报告与根 `prd.md`。
- 报告必须区分“结构合规”“内容深度”“检索覆盖”“人工可读性”“评测有效性”。

### R2 P0 数据一致性

- `wiki/index.md`、`wiki/maps/map-home.md`、`wiki/maps/library-status.md` 的实际页面计数、最近更新时间与导航必须一致。
- 主索引应集中列出全部 21 篇论文/预印本、2 本专著、20 个方法和 7 个 synthesis，不再把最新批次拆成难以发现的孤立附录。
- 修正仍引用旧水位的固定评测答案；不得用更新文本掩盖真实页面计数。
- 重新生成 Graphify，使现有 23 个 source 均可在派生图中找到；Graphify 失败必须留下可复核日志，不得手编 `graphify-out`。

### R3 P0 论文章节级证据检索

- 桌面端 SQLite 派生索引新增论文 section/chunk 索引；数据从 source 页的 `raw_md` 只读生成。
- chunk 至少保留：稳定 ID、source page ID、论文标题、章节标题、raw Markdown 路径、PDF 路径、起止行号和正文。
- 按 Markdown 标题分节；超长章节再按段落切块，避免一个超大章节吞掉检索，同时不得写回 `raw/`。
- 问答候选同时覆盖 Wiki、论文原文章节、核心书籍和 Graphify；论文证据必须展示章节/行号来源并能链回 source 页面。
- 同一论文的 Wiki 摘要和原文章节不得因错误去重互相覆盖；Graphify 仍只能作关系提示。
- 仓库切换或重建索引时清理并重建论文派生表，不影响聊天、设置或编译历史。

### R4 P1 知识表达契约

- 升级 `schema/page-types.md`、A 编译规程和 Lint 规则：以必需研究字段而不是机械字数作为详细度门禁。
- source 页应覆盖研究问题、形式化模型、目标与约束、算法流程、理论性质、实验设置、定量结果、局限及证据定位；原文没有的信息明确写“原文未报告”，不得补造。
- method 页应成为跨文献可复用方法档案，至少覆盖输入输出、变量、算法步骤、复杂度/理论保证、适用条件、失效边界和来源锚点。

### R5 P1 可复用知识层

- 新建有跨 source 复用价值的 `system-model`、`objective`、`dataset-or-sim` 页面，不为填目录而创建单源复述页。
- 第一批至少覆盖：干涉感知并发充电、在线定向请求、移动/UAV 路径—调度、异构移动充电器；完成时间、充电效用、能效/移动成本、多目标生存性；通用 WRSN 仿真与证据报告协议。
- 页面必须链回至少两个 source 或一个明确的核心理论锚点；冲突结论只并列，不裁断。
- 深化五组高价值 source/method：CCSP、GAIN、TIDE、CUAV 联合调度轨迹、IHATRPO。

### R6 人工阅读路径

- `map-models-and-objectives` 改成按“系统模型 → 目标 → 方法 → 证据”的任务导向入口。
- 页面采用渐进披露：开头提供 TL;DR、适用/不适用，再进入公式、算法、实验与证据锚点。
- 所有新增和深化页面保持中文正文、英文术语并列及有效 `[[wikilink]]`。

## Constraints

- `raw/` 正文只读，不删除或重写 PDF、转换稿及核心书籍章节。
- 不擅自修改 `schema/vocab.yaml`；缺词只进入提案。本阶段优先复用现有词表。
- 不新增或改写 `wiki/problems`、`wiki/ideas`。
- 不默认外搜；所有知识内容只来自当前 canonical 文献、核心书籍和既有 Wiki。
- 不使用 Graphify `--wiki` 覆盖本库 `wiki/`。
- 保留两个用户失败运行目录为未跟踪文件，不纳入提交。

## Acceptance Criteria

- [x] AC1：存在 `logs/2026-08-11-llmwiki-structure-audit.md`，包含量化证据、影响、P0/P1 决策和当前库水位；根 `prd.md` 有对应阶段条目。
- [x] AC2：Wiki Lint 0 errors；索引、总图和库水位均报告 23 source、20 method、7 synthesis，并正确展示新增类型数量。
- [x] AC3：SQLite 重建后为全部非 book source 的可用 `raw_md` 生成论文 section；测试证明可按英文和中文词命中 raw 证据并返回章节与行号。
- [x] AC4：问答证据包可同时保留同一 source 的 Wiki 摘要和论文原文章节，且 Codex prompt 明确论文证据可直接支持事实、Graphify 只作提示。
- [x] AC5：新增不少于 4 个 system-model、4 个 objective、1 个 dataset-or-sim 页面，全部满足准入与双向导航要求。
- [x] AC6：CCSP、GAIN、TIDE、CUAV、IHATRPO 的 5 个 source 和 5 个 method 页面按新结构深化，关键陈述带 raw 章节/行号锚点或明确缺失声明。
- [x] AC7：Graphify 增量重建后不再报告此前缺失的 8 个 source；若工具因环境失败，日志包含命令、错误和未完成状态。
- [x] AC8：Rust format、Clippy、完整 Rust 测试、Wiki Lint、Wiki Eval、核心书籍检索评测和前端构建各执行一次并通过。
- [x] AC9：Git 提交不包含 `raw/inbox/auto-discovered/runs/search-20260809-204315/` 与 `search-20260809-211516/`。

## Out of Scope

- 对全部 21 篇论文一次性做同等深度的人工重编译；本阶段建立契约、检索底座并完成五组高价值样板。
- 新增外部论文、重新下载 PDF、修改受控词表、创建新的 B 类研究想法。
- 改造桌面端页面视觉样式或生成新的安装包。
