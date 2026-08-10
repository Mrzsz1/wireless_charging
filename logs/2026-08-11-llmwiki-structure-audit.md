---
type: audit
title: LLM Wiki 结构、内容深度与检索覆盖审查
date: 2026-08-11
status: active
---

# LLM Wiki 结构、内容深度与检索覆盖审查

## 结论

当前库已经具备 Raw / Wiki / Schema / Graphify 的治理骨架，但内容仍偏“目录 + 摘要卡”。结构合规不等于答案充分：论文原文没有进入桌面端常规问答证据链，方法页又过短，模型容易漏掉原文已有的公式、约束、理论保证和实验条件；人工阅读时也缺少从系统模型、目标到方法和证据的渐进路径。

## 量化水位

| 类型 | 数量 | 正文中位字符数 | 主要问题 |
|---|---:|---:|---|
| source | 23 | 约 550 | 多数停留在摘要级，没有页码/章节锚点 |
| method | 20 | 约 167 | 输入输出、变量、复杂度、保证和失败边界不完整 |
| concept | 7 | 约 272 | 可导航，但定义和易混概念偏薄 |
| synthesis | 7 | 约 514 | 部分只有来源并列，比较维度不足 |
| system-model | 0 | — | 无法从问题模型进入知识 |
| objective | 0 | — | 目标、约束和权衡没有复用节点 |
| dataset-or-sim | 0 | — | 实验复现参数无法跨文献对照 |

21 篇论文 source 摘要正文合计约为对应 canonical Markdown 正文的 **0.81%**。该比例不是单独的质量判据，但在 raw 论文没有进入主要问答检索链时，会直接形成证据损失。

## 五个审查维度

### 1. 结构合规：较好

- `AGENTS.md`、`schema/` 和 A/B 写入边界明确。
- source provenance、canonical 生命周期和受控词表治理可审计。
- Raw 正文、Wiki 编译产物和 Graphify 派生图分层正确。

### 2. 内容深度：不足

23 个 source 页面中，公式、复杂度、实验设置、变量定义和精确证据位置的覆盖都很低；20 个 method 页面中几乎没有系统化的复杂度、理论保证和实验条件。典型的 `mtd-hipo-placement-obstacles` 正文不足百字，只能回答“用了什么”，不能回答“为什么适用、怎么实现、保证是什么”。

### 3. 检索覆盖：P0

桌面端 `pages_fts` 只索引 Wiki，`book_chapters_fts` 只索引两本核心书籍。论文 `raw_md` 不参与 `prepare_question`，而 Wiki 命中只返回短 snippet。结果是：原文有答案、Wiki 摘要没摘录时，回答仍会显示“库内未覆盖”。

### 4. 人工可读性：不足

- `map-home` 仍写 16 sources / 8 methods / 5 syntheses，实际为 23 / 20 / 7。
- `index` 将最新五篇拆在后置增量区，主表不完整。
- 缺少 system-model、objective、dataset-or-sim，研究者必须先知道论文或算法名才能进入。
- 页面缺少“何时使用 / 何时不使用 / 原文在哪里”的快速阅读层。

### 5. 评测有效性：不足

当前 10 问回归主要验证预期 wikilink、水位文本和 must-mention 词；不能证明科学事实准确、关键约束完整或引用位置正确。部分已通过答案仍保留 16-source 旧水位，证明测试通过与内容新鲜度并不等价。

## 已发现的一致性问题

1. `wiki/maps/map-home.md` 水位过期。
2. `wiki/index.md` 主来源表、方法表和 synthesis 表不完整。
3. 固定评测答案仍引用 16 sources。
4. Graphify 缺少 8 个已存在 source：Binh 2025、Gao 2024、Gao 2025、Honma 2026、Liu 2021、Qaisar 2026、Rahaman 2023、Yao 2026。
5. problem 页缺 `inspired_by`，属于既有 B 类告警；本任务不在未获新确认的情况下改写 B 类正文。
6. 核心书籍目录存在 legacy/重复章节文件与状态显示差异；正式检索已按 chapter-index 的 61 个条目工作。本任务不改 Raw，继续以派生索引隔离重复文件。

## P0 决策

1. 同步 index、map-home、library-status、评测水位。
2. 新增只读 `paper_sections` 派生索引，按标题和段落切分 canonical 论文 Markdown。
3. 回答证据包同时召回 Wiki 摘要、论文原文章节、核心书籍章节和 Graphify 关系提示。
4. 论文证据携带 raw 路径、章节和行号；无可靠映射时不伪造 PDF 页码。
5. 重建 Graphify 并复核缺失 source。

## P1 决策

1. 升级 source/method 详细度契约，以研究字段完整性而非机械字数为门禁。
2. 建立 4 类 system model、4 类 objective 和 1 个通用仿真/证据协议页。
3. 深化 CCSP、GAIN、TIDE、CUAV、IHATRPO 五组 source/method 作为后续 A 编译样板。
4. 将模型与目标地图改为“模型 → 目标 → 方法 → 证据”的任务式入口。

## 成功判据

- 查询具体约束或算法细节时，可以召回论文原文章节并显示行号。
- 同一论文的摘要和原文证据可以并存，不互相错误去重。
- 人可以从模型或优化目标出发，不依赖记住论文标题。
- Wiki 水位只有一个事实版本，Lint 能发现后续漂移。
- 评测除了链接，还检查原文章节证据渠道和当前真实水位。

## 当前库水位

截至 2026-08-11：`[[../wiki/maps/library-status]]` 为 23 个 source（21 篇论文/预印本 + 2 本专著）、20 个 method、7 个 concept、7 个 synthesis、61 个核心书籍索引条目；年份覆盖 2017–2026。
