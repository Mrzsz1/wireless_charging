# PRD：自生长 LLM Wiki（无线充电调度）

> 状态：已达成共享理解（Grill Session 确认）；**骨架、论文发现与来源追踪已落地**（2026-07-14）  
> 用途：避免后续实现与讨论产生冲突；**本文件为架构与规则的权威摘要**  
> 管理工具：Obsidian + Claudian  
> 范式：[Karpathy LLM Wiki](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f)（编译型持久 wiki）  
> 图查询：[Graphify](https://github.com/Graphify-Labs/graphify)（CLI/Skill，非向量库）  
> 入口：`HOME.md` · 总图：`wiki/maps/map-home.md` · Agent：`AGENTS.md`

---

## 1. 背景与目标

### 1.1 问题

科研场景下，文献与想法分散，难以：

1. 针对已有问题，快速找到**可用解法**或**可迁移解法**；
2. 判断某个 **idea 是否新颖 / 是否已被解决**（在可审计的证据链上）。

传统「每次问 LLM + 临时检索」不会留下可复利的结构化资产。Karpathy LLM Wiki 的核心是：LLM **增量编译并维护**一套持久、可交叉引用的 Markdown wiki；知识编译一次并保持更新，而不是每个问题重新推导。

### 1.2 产品目标

搭建一个**自生长**的结构化 LLM Wiki，领域聚焦**无线充电调度**（导师研究方向）：

- **Obsidian** 管理与阅读  
- **Claudian** 日常 `/solve`、`/novelty`  
- **外部 Agent** 执行 Ingest / Lint（`AGENTS.md`）  
- **Graphify** 对 vault 建可查询知识图（`query` / `path` / `explain`）  
- **MinerU** 负责 PDF→同名 md  
- **Windows 桌面客户端**作为统一可视化入口：本地阅读、检索、图谱、书籍、对比、Luna 问答与编译任务中心  

权威参考摘要：`schema/references/karpathy-llm-wiki.md`、`schema/references/graphify.md`。

### 1.3 非目标（第一期明确不做）

| 非目标 | 说明 |
|--------|------|
| 以写 paper 为主流程 | 近期不优化 related work 成稿、投稿流水线 |
| 万能个人第二大脑 | 不收任意生活/无关领域知识 |
| 默认全网查新 | `/novelty` 不默认外搜（见 §7） |
| 网页 / blog / PPT 作为源 | 禁止进入 raw（见 §4） |
| 文件夹热监听全自动编译 | 不做静默 ingest（见 §5） |
| 公网知识库网站 | 第一版只交付本地 Windows 客户端，不部署网站或公网服务 |

### 1.4 成功标准（用户可感知）

在 Obsidian + Claudian 中能够稳定完成：

1. **`/solve`**：给定研究问题 → 输出直接可用 / 可改可用 / 未见 的方法列表，并带 `[[wikilink]]` 与适用前提；
2. **`/novelty`**：给定 idea → 基于**当前知识库**给出已覆盖 / 部分重叠 / 未见，并带证据链接与**库水位**时间表述。

---

## 2. 用户与场景

### 2.1 主要用户

- 研究者本人（研究生向）：文献编译、问题求解、idea 检验、与导师方向对齐。

### 2.2 核心场景（双核）

| 场景 | 说明 | 系统侧重 |
|------|------|----------|
| **A. 文献编译与综述** | 读 paper → 结构化 → 对照、矛盾并列、gap | ingest、source/method/synthesis、maps |
| **B. 问题 / idea 锻造** | 从编译结果提炼 problem、评估 idea | problem/idea 页、显式触发、人工确认 |

二者关系见 §3（串行 + 轻量隔离），**不是**二选一。

### 2.3 典型问法

- 「这个问题有没有现成解法？有没有可能可迁移的解法？」
- 「我的 idea 新吗？是不是已经被人做过了？」

---

## 3. 工作流架构

### 3.1 A 串行 + 轻量 B 隔离（已锁定）

```text
自动发现 ─→ raw/inbox/auto-discovered ─┐
                                      ├→ 人工 triage → raw/canonical → MinerU → [A 编译] → wiki
手动投放 ─→ raw/inbox/manual-drop ─────┘                                      ↓
                                                                  显式 [B 锻造] → problem / idea（需确认）
```

| 规则 | 内容 |
|------|------|
| 默认循环 | **A**：canonical 文献结构化写入 wiki |
| B 触发 | 显式动作（synthesize ideas / open problems），**非**后台乱长 |
| 隔离 | 文献类页面禁止写「我们将贡献…」；贡献性表述只进 `problem` / `idea` |
| 链接硬规则 | idea/problem 必须 `inspired_by` / `supports`（或等价字段）链回 source、synthesis 或 gap |

### 3.2 三层分层（Karpathy 式）

| 层 | 路径 | 职责 |
|----|------|------|
| Raw | `raw/` | 原始文献与转换稿；不与 wiki 混放 |
| Wiki | `wiki/` | 持久编译产物；问答主读对象 |
| Schema | `schema/` + `AGENTS.md` | 类型契约、词表、提示词、写作红线（人 + agent 宪法） |
| Index / Log | `wiki/index.md`、`logs/log.md` | Karpathy 内容目录 + 时间线 |
| Graph（派生） | `graphify-out/` | Graphify 输出；可重建；**非**正文真相 |

---

## 4. 源材料策略

### 4.1 两个正交维度：采集来源 × 生命周期（2026-07-14 修订）

`manual/auto` 只回答“从哪里来”，`inbox/canonical/ingested` 只回答“处理到哪一步”。二者不得互相代替。

| 维度 | 取值 | 说明 |
|------|------|------|
| 采集来源 | `manual_upload` / `auto_discovery` | 手动投放或自动发现；晋升 canonical 后仍永久保留 |
| 筛选状态 | `pending` / `selected` / `rejected` / `promoted` | 候选是否被人工选择及是否已晋升 |
| 编译状态 | `pending_convert` / `pending_ingest` / `ingested` / `convert_failed` | PDF/Markdown/A 编译进度 |

| 区域 | 路径 | 行为 |
|------|------|------|
| 自动发现候选 | `raw/inbox/auto-discovered/runs/` | 每次检索的审计快照；默认 `pending` |
| 自动发现已选 | `raw/inbox/auto-discovered/papers/` | 人工选中的候选元数据/PDF，仍不是 canonical |
| 手动投放 | `raw/inbox/manual-drop/` | 用户放入的正式文献，待确认与解析 |
| Canonical | `raw/canonical/` | 已确认相关；**A 编译只读取这里**，来源元数据必须保留 |

硬规则：自动发现默认只进入 inbox；只有用户显式开启“自动完整入库”且候选通过主题、分数、标题、标识、开放 PDF、去重和单次上限全部规则后，才可进入受控 canonical → MinerU → A 编译流水线。手动投放也不得绕过 canonical 的可读性与状态检查。自动流程始终禁止创建 B 类页、新 Map、修改正式词表或改写关键 claim。

### 4.2 白名单 / 黑名单

| 允许 | 禁止 |
|------|------|
| **论文为主** | 网页 |
| 专利等正式辅助文献 | Blog |
| （同属正式文献、你认定的技术报告等可进，须标 epistemic） | PPT |

灰文献 / 专利：可进 canonical，但 frontmatter 强制 `source_type` + `epistemic`；B 阶段生成 idea 时降权。

### 4.3 PDF → Markdown（已锁定）

| 项 | 决定 |
|----|------|
| 主形态 | PDF |
| 转换工具 | **MinerU**（用户执行） |
| 产出 | Markdown + 图片资源；**一文一夹**（见下） |
| 质量目标 | 结构保留中等（章节/摘要/表格尽量保留；图随 md 相对路径保留） |
| 失败 | 标记 `convert_failed`，**禁止**静默进入 A 编译 |

#### 目录约定（解决 MinerU `images/`）

MinerU 常见产出：`xxx.md` + `images/*`。多篇论文若都丢在同一层，**图片会撞名/断链**。  
采用 **一文一夹**（推荐，已写入 raw 说明）：

```text
raw/canonical/
  Zhang2023_OnlineWPT_Scheduling/
    Zhang2023_OnlineWPT_Scheduling.pdf   # 可选归档
    Zhang2023_OnlineWPT_Scheduling.md    # MinerU 正文（A 编译主输入）
    images/                              # MinerU 导出图，路径与 md 内引用一致
    Zhang2023_OnlineWPT_Scheduling.html  # 可选：仅人读，不作为 A 编译主源
```

| 资产 | 角色 |
|------|------|
| `.md` + `images/` | **主工作副本**：Obsidian 可预览图；A 编译以文本为主，关键图可按路径打开 |
| `.pdf` | 溯源与印刷级图；争议时以 PDF 为准 |
| `.html` | **可选人读**；**不**当 ingest 主源（链、结构对 agent 差，且易与 wiki 混淆） |

原则：

1. **不要**把多篇的 `images/` 合并成一个全局 images。  
2. md 内图片用**相对路径**（如 `images/fig1.jpg`），保持 MinerU 默认即可。  
3. A 编译：正文与公式以 md 为准；需要读图时 agent 打开同目录 `images/` 或回看 PDF。  
4. Graphify / 问答仍以 md 为主；大图库可在 `.graphifyignore` 排除 `**/images/**` 以免噪音（图本身不进概念图）。

- 编译状态机：`pending_convert` → `pending_ingest` → `ingested`（或 `convert_failed`）  
- 来源追踪：`acquisition_method` / `discovered_via` / `discovery_run` / `triage_status` / `selected_by_user` / `acquired_at` / `canonicalized_at`  
- 文献字段：`title` / `year` / `source_type` / `ingest_status` / `pdf_path` / `raw_md`；`why_relevant` 建议补一句

---

## 5. 自主程度与角色分工

### 5.1 半自动（已锁定）

| 动作 | 是否可自动写入 |
|------|----------------|
| A：canonical ingest → 更新 source / concept / model / method / synthesis 等 | **是** |
| 已有 map **补链接** | **是** |
| **新建** map 主题 | **否**（需用户确认） |
| B：problem / idea | **否**（需用户确认） |
| 删除、合并、改写关键 claim | **否**（需用户确认） |
| 通用外搜 / novelty 实时查新 | **否**（必须先询问用户） |
| 用户已配置的论文自动发现任务 | **两级自动化**：默认只写 inbox 候选；用户显式开启后，仅满足严格资格的开放论文可自动晋升并执行 A 流水线；不得形成 novelty 结论或写 B 类页 |

### 5.2 分工（已锁定）

| 角色 | 职责 |
|------|------|
| **用户** | 导入 PDF、跑 MinerU、确认 B、确认新 map、确认删并/改 claim、批准外搜 |
| **外部 agent**（Cursor / Claude Code 等） | 批量 A 编译、词表提案、结构维护、`logs/`、更新 library-status |
| **Claudian**（Obsidian 插件） | 日常 `/solve`、`/novelty`、轻量补链 |
| **共同宪法** | `schema/`（类型说明、vocab、Claudian 提示词模板） |

### 5.3 端到端日常流

1. 客户端启动后询问是否运行配置流程，或用户在“文献入库”手动触发；论文进入 `auto-discovered` 或 `manual-drop`，写明采集来源。
2. 默认由用户 triage；显式开启自动完整入库时，只有通过全部资格规则且位于单次上限内的候选可自动确认。选择项晋升 `raw/canonical/`，保留 provenance 与决策理由。
3. MinerU：PDF → `full.md + images`，标记 `pending_ingest`。  
4. 外部 agent：A 编译 → wiki + maps 补链 + logs + library-status，并把 raw 标记为 `ingested`。  
5. 用户：Obsidian + Claudian 跑 `/solve`、`/novelty`。  
6. B 阶段：任一方可起草 problem/idea，**写入前用户确认**。

---

## 6. Vault 结构与页面类型

### 6.1 目录布局（类型为骨 + 主题 MOC；第一期即建 maps）

```text
vault/
  raw/
    canonical/
    inbox/
      auto-discovered/
        runs/
        papers/
      manual-drop/
  wiki/
    sources/
    concepts/
    system-models/
    objectives/
    methods/              # method + algorithm（subtype 区分）
    datasets-sims/
    syntheses/
    problems/             # 仅 B 阶段
    ideas/                # 仅 B 阶段
    maps/                 # 主题 MOC、总索引、library-status
  schema/
    vocab.yaml            # 受控词表唯一权威
    vocab-proposals.md    # 新词提案
    # 类型定义、写作规范、Claudian 模板等
  logs/
  prd.md                  # 本文件
```

### 6.2 页面类型（调度特化 9 类）

| 阶段 | type | 职责 |
|------|------|------|
| A | `source` | 单篇论文/专利结构化卡 |
| A | `concept` | 术语与定义（中英对照） |
| A | `system-model` | 场景与实体（设备、充电器、时间槽等） |
| A | `objective` | 优化目标与约束族 |
| A | `method` | 方法/算法骨架（`subtype`: method \| algorithm） |
| A | `dataset-or-sim` | 数据、仿真设定、评测协议 |
| A | `synthesis` | 多源对照、并列冲突、gap 汇总 |
| B | `problem` | 从 gap 提炼的 research problem |
| B | `idea` | 候选思路；必须链回文献/problem |

第一期**不扩展** author/venue/theorem 等重型类型；满 3 个月再用再拆。

### 6.3 文件命名（已锁定）

| 位置 | 规则 | 示例 |
|------|------|------|
| raw | 用户方便即可；PDF 与 md **同名** | `Zhang2023_OnlineWPT_Scheduling.pdf/.md` |
| wiki | 稳定 ID：类型短前缀 + 英文 slug | `src-zhang2023-online-wpt-scheduling.md` |
| wiki 标题 | 中文可出现在 H1/正文；**尽量不进文件名** | — |
| 关联 | source 用 frontmatter `pdf_path` / `raw_md` 指向 raw | 不要求 wiki 名 = PDF 名 |

前缀建议：`src-` `cpt-` `sys-` `obj-` `mtd-` `data-` `syn-` `prob-` `idea-` `map-`

---

## 7. 问答与检索

### 7.1 架构（已锁定）

**（可选 Graphify 收窄）→ 结构化字段 / index 过滤 → 精读少数 wiki 页 → 作答并回链。**

- 当前入口：**Obsidian 插件（Claudian）**；桌面客户端完成后，客户端成为面向用户的统一入口，Obsidian 保留为维护与审校工具  
- 外部 agent：Ingest / Lint / 调 Graphify CLI  
- **Graphify**：对 `wiki/` + `raw/**/*.md` 建图；`graphify query/path/explain` 辅助检索；`graph.html` 供人浏览  
- Graphify **不是**第二套 wiki：禁止用其 `--wiki` 覆盖本库 `wiki/` 结构；`graphify-out` 可删可重建  
- 传统「只扫 raw PDF 的 RAG」**不作为主路径**；wiki 不足且用户批准时再下钻 raw 或外搜  

### 7.1.1 Karpathy 三操作（本库映射）

| 操作 | 本库 |
|------|------|
| Ingest | A 编译 + 更新 index/log/library-status + Graphify `--update` |
| Query | Claudian `/solve` `/novelty`；Agent 可先 Graphify 再读页 |
| Lint | `schema/lint-checklist.md` |

### 7.2 标准问法模板

| 命令 | 输入 | 输出要点 |
|------|------|----------|
| `/solve` | 问题描述 | 直接可用 / 可改可用 / 未见；match 强度；前提与边界；`[[links]]` |
| `/novelty` | idea 描述 | 重叠 source；已解决程度；剩余 gap；`[[links]]`；**库水位**；置信度 |

### 7.3 `/novelty` 范围（已锁定）

| 规则 | 内容 |
|------|------|
| 默认 | **仅本 wiki**（已编译 source/synthesis/method 等） |
| 话术 | 必须声明「基于当前知识库…」，禁止伪全球「从未有人做过」 |
| 外搜 | **必须先询问用户**；因**日期**关键，外搜结果须带 `retrieved_at` + 文献 `year` |
| 外搜结果落盘 | 不得直接写成 wiki 事实；可进报告或 `raw/inbox`，升 canonical 并编译后才进正文 |

### 7.4 时间纪律（已锁定）

| 规则 | 内容 |
|------|------|
| source | 必填 `year` |
| 库水位 | 维护 `wiki/maps/library-status.md`：已编译 source、manual/auto canonical、pending/selected 候选、年份与最近 ingest |
| 问答 | `/solve` 与 `/novelty` **必须引用库水位** |
| 排序 | 可按 `year` 排序；**新 ≠ 自动更好** |

### 7.5 冲突策略（已锁定）

**并存不裁断（A）**：

- synthesis 等只并列：谁、在何种设定下、主张什么；
- 系统**不选边、不判对错**；
- `/solve` 并列候选解法与前提，由用户裁决；
- 第一期不强制独立 conflict 页。

### 7.6 语言（已锁定）

| 项 | 决定 |
|----|------|
| 策略 | 分类型务实双语（D） |
| 主写作语言 | **中文** |
| concept | 中文名 + 英文标准术语（必填） |
| source | 结构化叙述可用中文；标题/venue/专有名词保留英文 |
| synthesis / problem / idea | 中文为主 |

---

## 8. Frontmatter 与受控词表

### 8.1 调度匹配核心字段（已锁定）

参与 `/solve`、`/novelty` 匹配的类型至少包含：

```yaml
type: source | concept | system-model | objective | method | dataset-or-sim | synthesis | problem | idea
title: ""
status: draft | active | needs_review
epistemic: high | medium | low

# 匹配核心（取值必须来自受控词表 id）
scenario: []          # 或单值，实现时统一
entities: []
constraints: []
objectives: []
method_family: ""
problem_class: ""

# source 额外
year: 2024
venue: ""
source_type: paper | patent | ...   # 词表
paper_keywords: []                  # 作者 Keywords / Index Terms，自由元数据
keyword_source: index_terms         # author_keywords | index_terms | not_found
acquisition_method: manual_upload | auto_discovery
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: YYYY-MM-DD
canonicalized_at: YYYY-MM-DD
pdf_path: ""
raw_md: ""
why_relevant: ""
```

第一期**不要求**数学形式、超参、完整 baseline 矩阵等重型字段。

### 8.2 受控词表治理（已锁定）

| 项 | 决定 |
|----|------|
| 唯一权威 | `schema/vocab.yaml` |
| 新词流程 | 写入 `schema/vocab-proposals.md` → **用户确认** → 合并进正式表 |
| LLM 禁令 | **禁止**擅自向正式词表或 frontmatter 写入未入库 id |
| 页面写法 | frontmatter **只写 id**；正文可用中文 label |
| 种子 | 按「无线充电调度」起草短表，用户审阅后冻结；宁可短、准 |

第一期维度：`scenario` · `entities` · `constraints` · `objectives` · `method_family` · `problem_class` · `source_type`

每项建议结构：`id`（英文 snake_case）+ `label_zh` + `label_en` + 可选 `aliases`

### 8.3 论文关键词三层治理（2026-07-14 锁定）

```text
source.paper_keywords（作者原词 + 来源）
  → map-domain-keywords（规范别名 + source 证据）
  → vocab-proposals（仅匹配字段确有缺口时）
  → 用户确认后进入 vocab.yaml
```

- 论文关键词用于扩大领域导航与发现面，不直接成为受控 id。
- 优先使用作者 `Keywords` / `Index Terms`；没有则明确 `not_found`，不把摘要推断伪装为作者关键词。
- 同一词族被至少 2 篇 source 支撑，或成为正式 problem / 高频问答的必要入口时，可提高导航优先级；仍须遵守 A 类页面准入和词表提案闸门。
- 执行规范见 `schema/domain-keywords.md`；领域入口为 `wiki/maps/map-domain-keywords.md`。

---

## 9. MVP（第一期）

### 9.1 完成定义（已锁定：小而真）

1. Vault 目录与 `schema/`（类型规范、写作红线、Claudian 模板）就位；  
2. `vocab.yaml` 种子经用户批注/确认一版；  
3. `wiki/maps/`：总图 + library-status + 2–4 个主题 MOC（第一期即建，可先空壳后补链）；  
4. 用户**自行导入**第一批论文 PDF，并用 MinerU 转为同名 md；  
5. 外部 agent 完成 A 编译（建议量级约 8–12 篇，**以用户实际导入为准**）；  
6. maps 具备真实链接；library-status 反映水位；  
7. Claudian 上用**一个真实研究问题**跑通 `/solve`，用**一个真实 idea 草稿**跑通 `/novelty`；  
8. 演示一次 B：problem/idea 起草 → 用户确认后写入。

### 9.2 明确不做（MVP）

- 批量全自动文件夹监视  
- 默认外搜查新  
- 高保真版面还原管道  
- 多主题「第二大脑」架构  
- 重型 ontology（12+ 页面类型）

---

## 10. 风险与约束

| 风险 | 缓解 |
|------|------|
| LLM 幻觉写入 wiki | 半自动闸门；无 canonical 锚点不写硬事实；logs 可追溯 |
| 伪全球新颖性 | 默认仅库内 + 强制水位话术 |
| 词表爆炸/同义碎片 | 提案制；禁止自动扩正式表 |
| 论文关键词污染受控词表 | 原词、领域导航、正式 vocab 三层隔离；保留 source 证据与人工晋升闸门 |
| 源污染 / 来源丢失 | 禁网页/blog/PPT；采集来源与生命周期分离；canonical/source 永久保留 provenance |
| A/B 内容互相污染 | 目录隔离 + 文风红线 |
| 插件上下文限制 | 编译走外部 agent；Claudian 做检索式问答 |
| MinerU 转换质量 | 中等保真；失败不编译；关键处以 PDF 为准 |

---

## 11. 决策日志（Grill 锁定摘要）

| # | 议题 | 决定 |
|---|------|------|
| 1 | 边界 | B：无线充电为主 + 有限相邻域；科研；调度核心 |
| 2 | 场景 | A 文献编译 + B idea 锻造（双核） |
| 3 | A/B 关系 | 串行 A→B + 轻量隔离 |
| 4 | Raw | 双桶 D；论文主、专利辅；禁网页/blog/PPT |
| 5 | 页面类型 | 调度特化 9 类 |
| 6 | 目录 | 类型目录 + 主题 MOC；maps 第一期就建 |
| 7 | 自主度 | 半自动 B |
| 8 | 语言 | D，主中文 |
| 9 | 问答 | 结构化检索 C；入口 Claudian |
| 10 | 字段 | 调度匹配核心 B + **受控词表** |
| 11 | 词表治理 | 单表 + 提案制 B |
| 12 | MVP | 小而真 B；论文用户自导入 |
| 13–14 | 导入 | PDF + MinerU → 同名 md；结构中等 |
| 15 | 编译执行方 | 外部 agent 编译 / Claudian 问答 |
| 15b | Agent 偏好 | 主用 **Codex CLI + Grok CLI**（非仅 Cursor） |
| 16 | Novelty | 仅库内 A；外搜须询问（日期敏感） |
| 17 | 时间 | year + library-status 水位 B |
| 18 | 冲突 | 并存不裁断 A |
| 19 | 命名 | 稳定 ID 前缀 B |
| 20 | 共识 | 确认成立 |
| 21 | 参考源 | 对齐 Karpathy gist；采用 Graphify 为图查询层 |
| 22 | Karpathy 文件 | `AGENTS.md`、`wiki/index.md`、`logs/log.md`、Lint |
| 23 | Graphify | `.graphifyignore`；ingest 后 `--update`；不覆盖 wiki 正文 |
| 24 | MinerU 图 | **一文一夹** md+images；HTML 可选人读；编译主源仍是 md |
| 25 | 采集与生命周期（2026-07-14） | `manual_upload/auto_discovery` 与 `inbox/canonical/ingested` 正交建模 |
| 26 | 自动发现授权（2026-07-14） | 已配置的定时论文搜索可写 inbox；晋升、编译、novelty 结论仍需人工闸门 |
| 27 | 来源追踪（2026-07-14） | provenance 从候选贯穿 raw/canonical 与 wiki/source；库水位分渠道统计 |
| 28 | 论文关键词治理（2026-07-14） | 作者原词 → 领域关键词地图 → vocab 提案；不得按词频自动写入正式词表 |
| 29 | 首轮 triage 与 B 试运行（2026-07-14） | 用户授权 Agent 裁决候选并正式化一个 problem；selected 仍不自动晋升，未验证算法不建 idea 页 |
| 30 | Windows 客户端（2026-08-01） | 新增本地优先的 Windows 桌面客户端；不建设公网网站；交付 `.exe` 与 `.msi`/安装程序 |
| 31 | 客户端技术路线（2026-08-01） | Tauri 2 + React/TypeScript + Rust；SQLite FTS5 为可重建检索缓存；Graphify 与 wiki 分工不变 |
| 32 | 客户端写入边界（2026-08-01） | 第一阶段只读展示与查询；编译中心只调用既有受控流程；problem/idea 与关键 claim 继续保留人工确认闸门 |
| 33 | 客户端下一阶段 2（2026-08-01） | 0.3.0 聚焦文献详情、方法库与只读 Markdown 阅读器；先完成可追溯阅读闭环，再进入书籍、图谱和 Luna 问答 |
| 34 | 下一阶段 2 实施完成（2026-08-01） | 客户端 0.3.0 已接入页面列表/详情、Markdown、wikilink、反向链接、来源定位和方法库；继续保持 Raw/Wiki 只读 |
| 35 | 客户端 P3 Luna 问答（2026-08-02） | 0.5.0 采用证据优先链路：Wiki/核心书籍/Graphify 多路召回、可解释重排、Luna 流式生成、引用面板、会话历史与离线证据降级；API Key 只从环境变量读取 |
| 36 | 客户端 P5.3 发布硬化（2026-08-02） | 目标版本 0.7.0；完成生产签名更新、严格 GUI/安装 E2E、真实完整流水线验收、页面状态恢复、真正增量索引、Graphify 语义覆盖与文档收口 |
| 37 | 客户端窗口可见性恢复（2026-08-09） | 0.7.2 统一用物理像素持久化窗口矩形；按当前显示器工作区校验、迁移 v2 状态，并禁止安装 smoke 遗留应用进程 |
| 38 | 客户端文献入库（2026-08-09） | 新增手动 PDF、待确认候选和自动添加三入口；默认自动准备，显式开启后才允许严格合格候选自动完整入库；启动仅询问，不安装服务或计划任务 |

---

## 12. 后续实现顺序

### 已完成

1. 目录骨架、`schema/`、词表种子、maps、Claudian 模板  
2. Karpathy / Graphify 参考与 `AGENTS.md`、index、log、lint、ignore  
3. 38 条候选首轮 triage，并推进为 7 promoted / 3 selected / 14 rejected / 14 pending  
4. 首个 B problem 与论文关键词三层治理  

### 当前发布收口

1. P5.4 桌面端正确性与恢复可靠性已在 0.7.1 完成。
2. P5.5 窗口可见性恢复与安装 smoke 进程清理已在 0.7.2 完成。
3. 继续保持 Raw/Wiki 正文只读、两书 295 条固定评测集与 95% Recall@5 门禁。

---

## 13. Windows 桌面客户端（2026-08-01 锁定）

### 13.1 产品定位

客户端是知识库的**本地科研工作台**，不是 Markdown 文件浏览器，也不是网站壳。它统一承载阅读、检索、问答、比较、知识关系探索和编译任务观察，同时保持现有 Raw / Wiki / Schema / Graph 分层不变。

### 13.2 技术架构

```text
Tauri 2 Windows Shell
├─ React + TypeScript：桌面交互界面
├─ Rust Commands：文件系统、进程、SQLite、系统集成
├─ SQLite FTS5：可删除、可重建的本地搜索缓存
├─ Python sidecar：复用现有发现、解析、检索与编译工具
├─ Graphify CLI / graph.json：关系检索与图谱
└─ Luna：基于检索证据组织自然语言回答
```

约束：

1. 最终交付 `.exe` 与 `.msi` 或 `setup.exe`，不要求启动独立 Web 服务。
2. `wiki/**/*.md` 是正文真相；SQLite 与 `graphify-out/` 均为可重建派生物。
3. 客户端不得用 Graphify `--wiki` 覆盖现有 Wiki。
4. 第一阶段保持 `raw/` 与 `wiki/` 只读；后续写入仍遵守 A/B 闸门。
5. 默认离线可阅读、搜索、对比和浏览图谱；只有 Luna 远端调用、自动发现或下载任务按配置联网。

### 13.3 信息架构

| 模块 | 主要能力 |
|------|----------|
| 工作台 | 库水位、最近编译、研究主题、待处理任务、快捷提问 |
| 智能问答 | 自然语言输入、检索过程、流式回答、证据与引用、会话历史 |
| 文献库 | source 列表、全文搜索、字段过滤、详情、PDF、相关方法 |
| 文献入库 | 本地 PDF 预检与完整入库、候选确认/仅下载/拒绝、启动询问与两级自动化 |
| 方法库 | 方法骨架、适用前提、理论保证、来源论文、可迁移关系 |
| 核心书籍 | 两本算法专著的目录树、章节 Markdown、全书搜索、物理页码与 PDF 定位 |
| 知识图谱 | 社区视图、邻居展开、路径查询、类型过滤、Wiki 回链 |
| 对比工作台 | 2–5 篇论文或方法的模型、目标、约束、算法、保证、指标与局限并列 |
| 编译中心 | 最新文献发现、下载、解析、A 编译、Lint、Graphify 更新、日志与失败重试 |
| 设置 | 知识库路径、Luna 配置、索引重建、主题、字体、缓存与日志目录 |

### 13.4 主要交互框架

采用科研工具式三栏结构：

```text
左侧：全局导航与知识树
中部：当前工作区，可多标签打开问答、论文、书籍或图谱
右侧：证据、引用、反向链接、相关方法和页码定位
```

全局行为：`Ctrl+K` 搜索、前进/后退、多标签页、深浅主题、窗口状态保存、原始 PDF 打开、wikilink 跳转、文件变化后的增量索引提示。

### 13.5 Luna 问答链路

```text
问题意图识别
→ Wiki / SQLite FTS5 召回
→ 两本核心书籍章节检索
→ Graphify query/path/explain 收窄关系
→ 证据重排与去重
→ Luna 生成回答
→ 展示 [[wikilink]]、论文、章节、物理页码与库水位
```

回答必须区分：库内直接证据、相似模型、可迁移算法、核心书籍理论基础、库内尚未覆盖部分。默认不外搜；外搜仍按 §7.3 取得用户批准。

### 13.6 UI 设计方向

- 参考飞书的清晰导航、克制分隔和高信息密度；参考 Zotero 的三栏研究资料组织；参考 Obsidian 的双向链接与局部图谱。
- 只借鉴信息架构与交互规律，不复制品牌、Logo、图标或逐像素界面。
- 风格关键词：安静、理性、精确、轻量、论文友好；浅色暖灰底，深墨文字，低饱和蓝青为主强调色，琥珀色只用于待处理/风险状态。
- 避免渐变大背景、玻璃拟态、过度圆角、卡片套卡片和营销式大标题。
- 优先适配 1366×768 与 1920×1080；数据表、公式、引用、中文与英文标题必须清晰可读。

### 13.7 实施阶段

| 阶段 | 交付 |
|------|------|
| P0 | Tauri 工程、目录选择、数据契约、SQLite FTS5、索引一致性测试 |
| P1 | 工作台、文献/方法浏览、Markdown、wikilink、全局检索、反向链接 |
| P2 | 核心书籍目录、章节阅读、PDF 页码定位、图谱、对比工作台 |
| P3 | Luna 问答、证据重排、引用面板、会话历史 |
| P4 | 编译中心、实时日志、失败重试、任务结果与回滚入口 |
| P5 | Windows 安装包、自动更新、端到端测试、使用说明 |

### 13.8 验收标准

1. `library-status` 中的 source / method / synthesis 数量与客户端完全一致。
2. 两本书全部已索引章节均可搜索、阅读，并定位到物理页码。
3. 标准问题集的 Top-5 检索召回率达到 95% 以上；答案证据覆盖率单独评估，不以生成文风代替召回指标。
4. 每个回答均可展开证据，至少包含 Wiki 来源；书籍结论包含书名、章节与页码。
5. 图谱节点可回到 Wiki 正文，且不把派生关系伪装成正文事实。
6. 无网络时仍可完成阅读、搜索、对比与图谱浏览。
7. 文件变更后能够增量更新缓存，也能一键完整重建。
8. 编译中心完整保留命令、开始/结束时间、退出码、日志和失败原因。
9. 第一阶段客户端操作不会修改 `raw/` 正文或绕过 B 类人工确认。

### 13.9 下一阶段 2：可追溯阅读闭环（目标版本 0.3.0）

#### 13.9.1 阶段目标

在现有目录选择、SQLite FTS5 索引、全局搜索和多标签框架之上，完成“找到页面 → 阅读正文 → 查看关系 → 回到来源”的只读闭环。本阶段只覆盖 **文献详情、方法库、Markdown 阅读器**，不提前混入核心书籍双栏阅读、完整图谱、Luna 问答或编译中心。

#### 13.9.2 数据与命令层

| 能力 | Rust/Tauri 命令 | 结果要求 |
|------|-----------------|----------|
| 页面列表 | `list_pages(page_type, filters, sort)` | 支持 `source`、`method`，按标题、年份、状态和方法族过滤/排序 |
| 页面详情 | `get_page(page_id)` | 返回 frontmatter、Markdown 正文、源文件路径、更新时间和稳定页面 ID |
| Wiki 链接解析 | `resolve_wikilink(target, source_path)` | 处理别名、相对目标和未解析链接；不得静默跳错页 |
| 反向链接 | `get_backlinks(page_id)` | 返回引用页、所在段落摘要和可跳转目标 |
| 本地来源打开 | `open_local_path(path)` | 仅允许打开当前知识库根目录内的文件，并校验规范化路径 |
| 索引升级 | schema migration | 对现有 AppData SQLite 做版本迁移；失败时允许完整重建，不修改 Wiki 正文 |

数据契约应将 `type`、`title`、`year`、`status`、`epistemic`、`method_family`、`scenario`、`objectives`、`constraints`、`pdf_path`、`raw_md` 与 wikilink 关系规范化。解析失败必须携带文件路径与原因，界面不得以空白页吞掉错误。

#### 13.9.3 文献库与文献详情

1. 文献库展示全部 source，支持关键词、年份、状态、场景、方法族筛选以及标题/年份排序。
2. 点击搜索结果或文献行，在现有标签栏中打开唯一详情标签；重复打开同一页面时激活原标签。
3. 详情页分为概览、系统模型、目标与约束、方法、实验、局限、原始 Markdown 七个区域；缺失字段显示“本页未记录”，不得生成补写内容。
4. 右侧研究脉络展示出链、反向链接、相关方法和来源文件入口；每项均可回到对应 Wiki 页。
5. `pdf_path` / `raw_md` 存在时提供系统打开按钮；路径失效时显示可诊断错误和实际记录路径。

#### 13.9.4 方法库与方法详情

1. 方法库按 `method_family`、`scenario`、`objectives` 和 `constraints` 过滤，并支持全文搜索。
2. 方法详情展示算法骨架、适用前提、目标与约束、理论保证、复杂度、局限和来源文献；仅呈现 Wiki 已记录事实。
3. 来源论文、相关方法和核心概念全部使用可解析 wikilink；未解析链接显示“断链”状态并进入可导出的审校列表。
4. 文献详情与方法详情可互相跳转，且保留返回历史、当前标签和滚动位置。

#### 13.9.5 只读 Markdown 阅读器

1. 支持标题、段落、列表、引用、表格、代码块、行内/块级公式、图片和内部 wikilink。
2. 提供页面目录、页内搜索、标题锚点、复制引用和“在文件资源管理器中显示”。
3. 本地图片通过安全资源协议加载，只允许知识库根目录内路径；缺图显示原始相对路径。
4. 外部链接必须明确标识并交给系统浏览器；Markdown 内 HTML 默认不执行脚本。
5. 阅读器保持正文只读，不提供绕过 A/B 闸门的直接编辑入口。

#### 13.9.6 客户端集成与状态

1. 复用现有多标签栏，支持文献、方法和 Markdown 页面标签；关闭当前标签时回到最近访问标签。
2. 左侧导航由现有图标栏原位展开为完整导航，不再渲染第二个并列侧栏；折叠宽度 56px，展开宽度约 224px。
3. 保存窗口尺寸、最大化状态、侧栏折叠状态、右侧脉络面板状态和最近标签；仓库切换后清理失效标签。
4. 所有异步页面具备加载、空结果和错误状态；错误信息包含可执行的重试或重建索引入口。

#### 13.9.7 实施顺序

1. 定义页面详情、关系和过滤器 TypeScript/Rust 数据契约，并加入 SQLite schema migration。
2. 实现 `list_pages`、`get_page`、`resolve_wikilink`、`get_backlinks`、`open_local_path` 及 Rust 单元测试。
3. 完成通用只读 Markdown 阅读器和安全本地资源解析。
4. 完成文献库筛选、文献详情及搜索结果跳转。
5. 完成方法库、方法详情和文献—方法双向跳转。
6. 接入多标签、访问历史、滚动位置与界面状态持久化。
7. 建立断链、缺失路径、异常 frontmatter、大文件与中文查询回归样本。
8. 完成 1366×768、1920×1080 视觉验收，执行前端构建、Rust 测试和 Windows 安装包冒烟测试。

#### 13.9.8 阶段验收

1. 当前库 23 个 source 与 20 个 method 均可从列表进入详情，数量与 `library-status` 一致。
2. 任一搜索结果可在两次交互内进入正文；同一页面不会产生重复标签。
3. 已解析 wikilink 的跳转成功率为 100%；未解析项全部显式标记且可定位源文件。
4. 文献与方法详情中的字段均可追溯到具体 Wiki 路径，不把 SQLite 或 Graphify 派生内容当正文事实。
5. Markdown 标题、表格、公式、代码块、图片和中文内容在两种目标分辨率下可读。
6. 路径越界、缺图、损坏 frontmatter 和索引版本不一致均不会导致白屏。
7. 本阶段所有客户端操作不修改 `raw/` 或 `wiki/` 正文。
8. `npm run build`、前端回归校验、`cargo test`、Tauri release 构建和安装后启动测试全部通过。

#### 13.9.9 本阶段明确不做

- 两本核心书籍的目录—章节—PDF 页码双栏阅读（进入 P2）。
- 完整 Graphify 交互图、路径可视化和多文献对比（进入 P2）。
- Luna 自然语言问答、证据重排和会话历史（进入 P3）。
- 自动发现、下载、A 编译、Lint 和失败重试控制台（进入 P4）。

#### 13.9.10 实施状态（2026-08-01）

本阶段已在客户端 `0.3.0` 落地：SQLite 页面详情/过滤/链接解析/反向链接/安全本地文件打开命令、文献库与方法库筛选列表、只读 Markdown 阅读器、文献/方法详情页和多标签跳转均已接入。Raw/Wiki 正文仍只读，未改变 A/B 人工确认边界。自动化验收包含前端构建与结构检查、3 个 Rust 单元测试、Tauri release 打包和桌面启动冒烟测试。

### 13.10 P3：Luna 证据优先智能问答（目标版本 0.5.0）

#### 13.10.1 阶段目标

在 P2 的核心书籍、Graphify 局部图谱和对比工作台之上，完成“自然语言提问 → 多路检索 → 证据重排 → Luna 流式回答 → 引用核验 → 历史恢复”的问答闭环。回答必须区分库内直接证据、相似模型、可迁移算法、核心书籍理论基础和库内尚未覆盖部分，并展示当前库水位。

完整执行设计、数据契约、步骤与验收门禁见 `design/p3-luna-qa-plan.md`。

#### 13.10.2 数据与命令层

1. SQLite 新增 `chat_sessions`、`chat_messages`、`chat_evidence` 与仅保存非秘密参数的 `app_settings`；使用 `PRAGMA user_version` 管理 migration。
2. 会话按知识库路径隔离；重建 Wiki/书籍索引不得删除历史消息和证据快照。
3. 新增 Luna 设置、会话 CRUD、`prepare_question`、`ask_luna`、`cancel_answer` 等 Tauri 命令。
4. 流式事件包括 started、retrieval、token、completed、failed 与 cancelled；错误携带请求 ID 和可重试属性。

#### 13.10.3 检索与证据

1. Wiki 使用 SQLite FTS5，`source`、`method`、`synthesis` 分层加权；
2. 两本核心书籍使用 `book_chapters_fts`，命中必须带书名、章节和 PDF physical pages；
3. Graphify 只提供关系候选，必须带 `source_file/source_location`，不得单独支撑事实；
4. 全局排序后保留来源多样性，存在书籍命中时证据包至少保留一个书籍章节；
5. 引用编号固定为 `[E#]`，前端只激活后端已登记的证据编号。

#### 13.10.4 Luna 与秘密边界

1. endpoint 使用 OpenAI-compatible Chat Completions；默认模型名 `gpt-5.6-luna`；
2. API Key 默认从 `LUNA_API_KEY` 环境变量读取，不写 SQLite、日志和前端持久状态；
3. 支持 SSE 流式响应、超时和取消；网络或配置异常时回退到确定性离线证据包；
4. 默认不外搜，Luna 只能依据本轮编号证据组织回答。

#### 13.10.5 客户端交互

1. “智能问答”从占位导航升级为真实工作区；
2. 左列展示当前知识库的会话历史，中列展示问题与流式回答，右列展示证据、库水位、页码和排序理由；
3. 支持停止、重试、复制、重命名与删除会话；
4. Wiki 引用可打开页面，书籍引用可打开本地 PDF，未登记引用显示为无效引用；
5. 未选择仓库、未建索引、Luna 未配置、请求失败和取消均有独立状态，不产生白屏。

#### 13.10.6 验收

1. 10 条 Wiki 固定问题均至少召回一个预期 Wiki 证据并返回库水位；
2. 两本核心书籍 Recall@5 分别保持 ≥95%，书籍证据 physical-page 锚点率为 100%；
3. 会话按仓库隔离，索引重建后历史仍存在；
4. API Key 不出现在 SQLite、日志、错误和前端状态；
5. Luna 未配置或断网时仍可完成检索、证据阅读、复制与历史保存；
6. `npm run build`、`npm run verify`、`cargo test`、Wiki/Core-book 评测、Tauri release 构建与安装后启动冒烟全部通过。

#### 13.10.7 实施状态（2026-08-02）

客户端 `0.5.0` 已完成 P3 主链路：问答 schema migration、会话历史、多路证据召回与来源多样性、库水位快照、Luna SSE 流式适配、环境变量密钥边界、离线证据降级、智能问答三列界面和动态引用面板。自动验收通过 12 个 Rust 测试（含 10/10 固定问题预期证据召回与真实仓库证据测试）、前端构建/结构校验、Wiki 10/10 契约、两书 295 条检索评测和 Tauri release 构建；MSI、NSIS 安装包及 release 可执行文件均已生成，隐藏启动 8 秒冒烟通过。

### 13.11 P4：编译中心与工作区可靠性（目标版本 0.6.0）

#### 13.11.1 阶段目标

修复仓库选择后的列表刷新与“我的空间”树展开问题，并完成“受控任务 → 实时日志 → 结果审计 → 失败重试”的本地编译闭环。详细方案与安全边界见 `design/p4-compile-center-plan.md`。

#### 13.11.2 已实现能力

1. 仓库恢复遇到空索引时自动重建；前端使用仓库就绪代次触发文献、方法与对比数据重新加载。
2. “我的空间”使用稳定节点 ID、独立展开状态、真实子导航、键盘按钮行为及 `aria-expanded`。
3. SQLite 新增 `compile_runs`、`compile_run_events`、`compile_artifacts`；遗留运行任务在重启后标记为 `interrupted`。
4. 编译中心固定允许 `lint`、`graphify_update`、`discover`、`parse`、`compile_a`，不接受前端命令字符串。
5. 子进程 stdout/stderr 实时写入事件表并通过 Tauri Channel 推送；支持取消、相同参数重试、仓库隔离和生成物登记。
6. parse 输入必须位于当前仓库；日志对 API Key、Token、Authorization、Bearer 与签名参数脱敏。
7. 三栏界面展示任务目录、历史/实时日志、参数/失败原因/生成物/回滚入口。

#### 13.11.3 验收状态（2026-08-02）

客户端 `0.6.0` 已通过前端构建、P4 结构门禁、16 个 Rust 测试、31 个 Python 工具链测试、Clippy 零警告、Wiki 10/10 契约及两书 295 条 Recall@5 评测。Graphify 更新为 1471 nodes / 2225 edges / 130 communities；MSI、NSIS 与 release 可执行文件构建完成，隐藏启动 8 秒冒烟通过。

### 13.12 P5.3：发布硬化、真实增量与状态恢复（目标版本 0.7.0）

#### 13.12.1 阶段目标

在 `0.6.0` 已具备阅读、检索、书籍、图谱、Luna 问答和编译中心主链路的基础上，完成“可长期使用、可重复发布、可严格验收”的收口。该阶段不新增研究页面类型，不扩张 A/B 写入权限；重点消除跳过式测试、离线占位更新、伪增量索引、重启状态丢失和 Graphify 语义覆盖缺口。

完成后应形成以下闭环：

```text
文件变化 → 局部索引更新 → 页面/图谱可见
客户端重启 → 仓库/标签/页面/滚动位置恢复
完整流水线 → 阶段审计 → 失败恢复/重试/回滚 → Graphify/Lint/评测
版本发布 → 签名产物 → 更新清单 → 客户端检查/下载/安装/重启
安装包 → 静默安装 → 启动导航验收 → 卸载 → 残留检查
```

#### 13.12.2 范围与硬边界

1. `wiki/**/*.md` 仍是正文真相；SQLite 与 `graphify-out/` 继续是可重建派生物。
2. 文件监听只更新索引，不自动触发 A 编译，不自动晋升发现候选，不修改 `raw/` 正文。
3. 完整流水线只调用固定 allowlist；前端不得提交可执行文件名、任意参数或 shell 字符串。
4. `problem` / `idea`、新 map、删除合并和关键 claim 修改继续保留人工确认闸门。
5. 自动更新必须验证签名；私钥只来自构建环境，不进入 Git、应用配置、日志或前端状态。
6. GUI/安装 E2E 缺少运行依赖时必须明确失败；日常快速验证可单独提供允许 skip 的 smoke 命令，但发布门禁不得 skip。
7. Graphify 语义补图不得使用 `--wiki` 覆盖本库 `wiki/`，也不得把派生关系写成正文事实。

#### 13.12.3 工作包

##### P5.3-00：基线、计划与 Git 安全点

1. 确认工作树干净，记录当前提交、版本、测试数量、知识库水位、Graphify 节点/边数与两书 Recall@5。
2. 将本计划写入 PRD；核对 `design/p3-luna-qa-plan.md`、`design/p4-compile-center-plan.md` 与实际完成状态。
3. 每个独立工作包完成后单独提交 Git；提交前执行 `git diff --check`，不得把密钥、AppData、`target/`、安装后的系统文件或临时更新配置加入版本库。

##### P5.3-01：页面、标签、滚动与快捷键恢复

1. 为标签持久化增加 schema 版本和仓库标识；仓库切换时删除不属于当前仓库的页面标签。
2. 客户端启动恢复 `activeTab` 时，若为 Wiki 页面，自动调用 `get_page` 与 `get_backlinks`；目标缺失时关闭失效标签并回到安全默认页，不显示空白占位。
3. 按 `repositoryPath + resourceId` 保存页面滚动位置；页面卸载前记录，重新激活后在正文渲染完成后恢复。
4. 记录窗口尺寸、最大化状态与最近有效标签；异常 JSON 或旧 schema 自动回退默认值。
5. 实现全局 `Ctrl+K` / `Cmd+K`：聚焦当前工作区的搜索框；输入框、对话框和组合输入法场景不得重复触发。
6. 增加恢复、失效标签、仓库切换、快捷键和滚动位置的前端回归测试。

##### P5.3-02：真正的增量索引

1. 将现有“目录变化 → `rebuild_connection`”拆分为 `upsert_page`、`delete_page`、`refresh_page_links` 和 `refresh_book_or_schema`。
2. 单个 `wiki/**/*.md` 创建/修改时，只重新解析该页，更新 pages/FTS/wikilinks，并局部重算受影响反向链接。
3. 单页删除或重命名时，删除旧 ID、FTS 与出链；重命名事件必须同时处理旧路径和新路径。
4. `schema/`、核心书籍章节索引、SQLite schema 版本变化继续触发完整重建；`graphify-out/graph.json` 变化只刷新图缓存，不重建 Wiki FTS。
5. 监听器进行稳定去重与静默期防抖；同一批变更只产生一次前端通知，避免流水线批量写入时反复全量重建。
6. 增量事务失败时回滚并返回可诊断错误；界面提供“一键完整重建”兜底。
7. 使用临时仓库测试 create/modify/delete/rename、断链变有效、有效链接变断链、批量事件与 schema 全量回退；增量结果必须与完整重建结果等价。

##### P5.3-03：完整流水线与 Graphify 语义覆盖

1. 为 full pipeline 建立无网络 fixture：发现、可选解析、A 编译、Lint、Graphify、快照和 verify 均使用可控本地命令替身，验证阶段顺序、日志、超时、取消、安全边界暂停、继续和失败短路。
2. 在真实仓库执行一次非 Dry-run 受控验收；若不希望新增候选，使用固定本地 fixture，禁止把测试候选晋升 canonical。
3. 逐阶段验证生成物 hash、退出码、失败原因、retry_of、回滚材料和重启后的 `interrupted` 恢复。
4. Graphify 阶段区分代码结构更新与 Markdown 语义更新；新增/修改 Wiki 页必须进入语义提取流程或形成明确待处理队列，不得以裸 `graphify update` 成功冒充语义覆盖完成。
5. 重新运行 Wiki Lint；Graphify 未覆盖 Wiki 页警告必须归零，或生成包含精确页面路径和原因的阻塞报告。
6. 保持两本核心书籍 295 条评测：Algorithmic Game Theory 与 Approximation Algorithms 的 Recall@5 均不得低于 95%，physical-page 锚点率保持 100%。

##### P5.3-04：生产签名自动更新

1. 保留默认离线构建；生产发布使用临时 Tauri config overlay 注入 HTTPS endpoint 与正式公钥，并生成 updater artifacts。
2. 构建脚本强制检查 `TAURI_UPDATER_ENDPOINT`、`TAURI_UPDATER_PUBKEY`、`TAURI_SIGNING_PRIVATE_KEY`，私钥缺失或 endpoint 非 HTTPS 时立即失败。
3. 提供可本地启动的更新清单 fixture/server，用旧版本客户端验证：检查版本 → 下载 → minisign 校验 → 安装 → 重启 → 版本更新。
4. 验证清单损坏、签名错误、下载中断、无网络、无新版本和用户取消；失败不得损坏当前安装。
5. 设置页显示当前版本、更新通道、检查结果和下载进度；日志不得包含私钥或完整认证 header。
6. 更新发布说明，明确密钥轮换、endpoint 切换、失败回滚与旧版本兼容策略。

##### P5.3-05：严格 GUI、安装与卸载 E2E

1. 将 `gui-smoke.mjs` 分为可跳过的开发 smoke 和发布必跑的 strict 套件；strict 模式自动启动/停止 `tauri-driver`，任一必需元素缺失即失败。
2. GUI 路径至少覆盖：启动、窗口最小化/最大化/关闭、侧栏原位展开、“我的空间”树、选择/恢复仓库、文献库加载、问答输入、核心书籍、图谱、编译中心和设置。
3. 增加页面状态恢复、`Ctrl+K`、文件变化增量刷新、full pipeline fixture、更新失败状态的 GUI 用例。
4. 安装测试使用隔离用户目录：NSIS/MSI 安装 → 启动 → 读取本地 fixture → 关闭 → 卸载；检查进程、开始菜单/快捷方式和预期残留。
5. 发布门禁要求显式提供应用路径与安装包路径；缺少 driver、WebView2 或安装器时返回非零状态，不以 `SKIP` 算通过。
6. 对 1366×768 和 1920×1080 各执行一次关键界面截图验收，确保无白屏、遮挡、不可达按钮和横向溢出。

##### P5.3-06：使用说明、状态文档与发布收口

1. 更新 `apps/desktop/README.md`：开发、离线模式、Luna、完整流水线、Graphify、严格 E2E、安装包、签名更新与故障诊断。
2. 将 P3/P4 计划文档标记为历史已实施方案，并链接实际版本、提交和最终测试结果；不得删除原设计决策。
3. 更新 PRD 的实际测试数量、Graphify 水位、安装包路径与 P5.3 完成状态；追加 `logs/YYYY-MM-DD-*.md`。
4. 更新 `wiki/maps/library-status.md` 仅限真实水位变化；不得因客户端代码变更伪造文献水位。
5. 执行 Graphify 更新并确认 `wiki/index.md` 未被派生 wiki 覆盖。

#### 13.12.4 逐步实施顺序

1. 创建 Git 基线提交或确认现有干净提交，记录版本与验收快照。
2. 编写状态恢复和增量索引的失败回归测试，先复现当前 active page 无法恢复和单页变化触发全量重建。
3. 实现标签 schema、仓库隔离、页面恢复、滚动恢复与 `Ctrl+K`，完成前端构建和回归。
4. 提取 Rust 单页解析/写入函数，实现 create/modify/delete/rename 增量事务及完整重建兜底。
5. 集成 watcher 防抖与前端通知，验证增量结果和全量结果一致。
6. 建立 full pipeline 本地 fixture，覆盖成功、阶段失败、超时、取消、暂停/继续、重试和回滚。
7. 修正 Graphify 阶段的 Markdown 语义更新，补齐当前未覆盖 Wiki 页面并运行 Lint。
8. 加固签名发布脚本，建立本地 updater fixture，验证成功升级与全部失败分支。
9. 加固 GUI strict 与安装/卸载 E2E，在两种分辨率执行关键路径。
10. 运行完整质量门禁：前端构建、P5 验证、Rust fmt/clippy/test、Python 工具测试、Wiki 契约、核心书籍评测、Graphify/Lint、Tauri debug/release、安装后启动。
11. 更新 README、P3/P4 状态、PRD 实施状态与日志；检查所有结果可由命令和产物复核。
12. 按工作包提交 Git，最后确认 `git status --porcelain` 为空，并记录提交哈希与安装包路径。

#### 13.12.5 验收门禁

1. 重启客户端后恢复最后有效仓库、标签、active page、页面正文、反向链接和滚动位置；失效资源自动降级而非白屏。
2. `Ctrl+K` 在全局与文献库中行为一致，键盘和鼠标均可完成主要导航。
3. 单 Wiki 页 create/modify/delete/rename 不调用全量重建；增量 SQLite 内容与完整重建逐表等价。
4. schema/core-books 变化触发完整重建，Graphify-only 变化不重建 Wiki FTS。
5. full pipeline fixture 的成功与六类失败/控制分支全部自动化通过；真实受控运行保留完整审计记录。
6. Wiki Lint 为 0 errors；Graphify 语义覆盖警告归零，B 类页面警告仅可在用户确认后修复，不作为擅自改写理由。
7. 两书 Recall@5 均 ≥95%，书籍证据 physical-page 锚点率 100%，Wiki 固定问题保持 10/10。
8. 生产 updater 使用正式签名完成一次旧版到新版升级；错误签名、断网和损坏清单均保持当前版本可启动。
9. GUI strict、NSIS/MSI 安装启动和卸载测试均真实执行，发布报告中不存在关键步骤 `SKIP`。
10. 1366×768 与 1920×1080 无白屏、关键按钮不可达、侧栏重复展开或主体横向溢出。
11. README、PRD、设计状态、日志与实际命令一致；密钥扫描无泄漏，Git 工作树干净。

#### 13.12.6 子代理分工与交付协议

1. 子代理只负责宽范围探索、检索和独立核验，不修改代码、不决定方案、不执行最终发布。
2. 并行拆分为：状态恢复/快捷键、增量索引、流水线/Graphify、更新器、GUI/安装、文档/验收六个探查任务。
3. 每个子代理必须返回 `file:line`、符号名、当前行为、缺口、建议测试点和未覆盖范围；事实与推断分开。
4. 主线程根据证据亲自阅读即将修改的确切代码，负责实现、集成、测试、Graphify 和 Git 提交。


#### 13.12.7 P5.3 实施记录（历史）

P5.3 的代码、测试和 0.7.0 发布产物已由提交 `8eb272b` 及后续 GUI E2E 环境提交完成。仓库历史中的该小节曾出现编码损坏；本次不把损坏文本当作事实来源，实际数字以可复核命令和 Git 产物为准：Rust、Python、前端构建和严格 GUI E2E 均在本阶段重新执行，核心专著评测继续使用 295 条固定问题。

#### 13.13 P5.4：桌面端正确性与恢复可靠性收口（已完成 2026-08-09，版本 0.7.1）

P5.4 关闭代码审查遗留的 `SEARCH-001`、`BOOK-001`、`ROLLBACK-001`、`WATCH-RISK-001` 与 `PATH-RISK-001`，形成以下闭环：

```text
快速输入 → 仅最新搜索响应可见
章节索引 → 仓库边界校验 → 字符安全片段 → PDF 页码定位
目录事件 → in-flight 批次 → 事务成功确认 / 失败退避重试 / blocked
编译回滚 → 全量预检 → staging → 逆序补偿 → failed/failed_partial/succeeded 终态
```

范围与硬边界：

1. 全局搜索使用请求 generation 丢弃乱序成功/失败响应；`Ctrl+K` 和既有搜索 DTO 保持兼容。
2. 专著章节路径拒绝绝对路径、Windows 前缀、`..` 和符号链接越界；片段按字符边界生成，不改排名和评测集。
3. 多文件回滚在文件系统操作前完成 hash/备份预检，使用 staging 与补偿日志；只有全部恢复并落账才将原 run 标记 `rolled_back`。
4. watcher 事件在 SQLite 成功前保持 in-flight；失败保留批次并退避，达到上限显示 blocked，完整重建成功后清理已覆盖批次。
5. 不修改 `raw/`、`wiki/` 正文，不自动晋升候选，不新增 B 类页面，不执行外部搜索。

验收门：

- 全局搜索乱序和旧失败回归通过；
- 章节片段 Unicode/标题偏移/空正文测试通过；
- 章节路径越界和 symlink/junction 边界测试通过；
- created/modified/deleted 混合回滚及第二个 artifact 故障注入通过，失败 run 不留 `running`；
- watcher rename、去重、重试、blocked 和完整重建清理通过；
- Algorithmic Game Theory Recall@5 ≥95%，Approximation Algorithms Recall@5 ≥95%，physical-page 锚点保持 100%；
- 版本文件统一为 0.7.1，严格 GUI/安装门禁无关键 `SKIP`，Git 工作树干净。

实施结果（2026-08-09）：

- Rust `cargo fmt --check`、Clippy（`-D warnings`）与 32 个测试通过；前端 `test:p1` 4/4、`test:p2` 3/3、构建与 `verify`、P3/P4/P5 门禁通过。
- Python 工具链 37/37、Wiki 10/10、Wiki Lint 0 errors/1 warning、两书 295 条评测通过；Algorithmic Game Theory Recall@5=1.000，Approximation Algorithms Recall@5=0.986667。
- 发布构建成功：`apps/desktop/src-tauri/target/release/app.exe`、MSI、NSIS；严格 GUI（1366×768、1920×1080）与 NSIS 安装/启动/卸载 smoke 均通过，无关键 `SKIP`。
- Graphify 已增量更新至 2589 nodes / 4364 edges / 200 communities；`wiki/`、`raw/` 正文未改动。
- 产物 SHA-256：app `63E048760E42DBDCCE3B27C8AF2AAD1D95777B90EDC06468B3497890456B2626`；MSI `3EF3931E81525B63E365BE12D89CF2FDA488AF747121F7683F63A347F707B8FA`；NSIS `C1EBA0AF177BBD564690432D55C5856328F2F754DE19FBB5B01C1035E40D1B29`。

#### 13.14 P5.5：Windows 窗口可见性恢复（已完成 2026-08-09，版本 0.7.2）

问题根因由现场窗口矩形确认：单显示器工作区为 `{x:0,y:0,width:2048,height:1104}`，故障进程窗口位于 `{left:-2858,top:381,right:-708,bottom:1589}`，窗口仍存活但与当前工作区无交集。旧实现又将物理像素保存值作为逻辑像素恢复，使高 DPI 与显示器拓扑变化进一步放大越界风险。

本阶段完成以下闭环：

1. 窗口 v3 状态统一保存物理像素；兼容读取 v2，并用 `PhysicalSize` / `PhysicalPosition` 恢复。
2. 启动时读取当前显示器工作区：合法负坐标副屏位置继续保留，越界矩形被限制到相交工作区，完全离屏时回到主显示器中央。
3. 最小化不覆盖正常矩形，最大化只更新状态标志；恢复后执行 `unminimize`、`show` 与 `setFocus`，Tauri 初始窗口同时启用居中兜底。
4. NSIS smoke 改为显式管理子进程树，卸载前确认应用退出，避免测试目录中的残留进程被后续启动误认。
5. Node 回归覆盖故障坐标、负坐标双屏、分辨率缩小、损坏状态与 DPI fallback；严格 GUI E2E 在导航前断言窗口与显示器工作区相交。

验收结果：前端 `test:p1` 8/8、`test:p2` 3/3、安装生命周期 2/2，Rust 32/32，Python 37/37；严格 GUI 与 NSIS 安装/启动/退出/卸载通过且无关键 `SKIP`。两书 Recall@5 分别为 1.000 与 0.986667，未修改 `raw/`、`wiki/` 正文。

0.7.2 发布产物 SHA-256：app `B4595386B5ACC1F35CAE64295704BBD2295FEDF89928353927291273FF6D672A`；MSI `0C8AE406B6A62DDA250410F86C6F5F1581C201C9980C4D0565200F7F6748D88B`；NSIS `A368FE9789F6C1A5808CFE4EB655210916269B071BCEA9714CAC227AE4CA55C4`。

#### 13.15 P5.6：上下文相关研究脉络（已实现 2026-08-09，版本 0.8.0）

右侧“研究脉络”不再把目录前五项伪装为排名结果，而由当前 Wiki 页面、已提交问题或文献库搜索词驱动：

1. 页面上下文融合正文出链、反向链接、Wiki FTS5、两本核心书籍和 Graphify 一跳关系，并排除锚点自身；问题与搜索上下文复用智能问答的同一套中英检索扩展。
2. 证据项显示来源类型、关系、归一化分数、检索理由与可定位字段；相关方法只允许 `type: method` 页面。
3. Graphify 或书籍索引缺失时显示 `degradedChannels`，不退回固定目录顺序；请求序列守卫阻止快速切换时旧响应覆盖新上下文。
4. “添加证据”并行搜索本地 Wiki 与核心书籍，固定项按 repository + contextKey 隔离存入本地状态，不修改知识库正文；支持取消固定。
5. Wiki、书籍和 Graphify 证据分别打开页面、章节或聚焦图节点；问答仅在提交问题/打开历史时更新锚点，不跟随未提交输入。
6. 硬边界保持不变：不修改 `raw/`、`wiki/`、`schema/` 正文，不自动晋升候选，不进行默认外搜。

验收基线：Rust 35/35，前端 P1 8/8、P2 3/3、研究脉络状态 3/3；P3/P4/P5、Wiki 10 问、两书 295 条评测和 1366×768/1920×1080 GUI strict 通过；GUI strict 已对真实 0.8.0 release EXE 验证搜索上下文产生研究脉络锚点与证据卡，NSIS 已在隔离目录完成安装、启动、进程退出与卸载。Algorithmic Game Theory Recall@5=1.000，Approximation Algorithms Recall@5=0.986667。

0.8.0 发布产物 SHA-256：app `1F4031EF30225BD181D230191E0A4BCF970152539EEDA6CF89705A692F9059F3`；MSI `EC9AEB4A687CA13B6958EFE9CE5B9EA653C2AFAE91CF012B04481FD16A83CE33`；NSIS `1D50526FA46ACCDD99E871ACE0A2373E5CFE7876A9B3F4FFFC53C9B1A53CA6B9`。

#### 13.16 客户端文献入库（2026-08-09）

客户端新增一级入口“文献入库”，复用编译中心的固定命令、仓库写锁、事件流与任务历史：

1. **手动添加**：系统选择器支持多 PDF；确认前只执行 PDF 头、200MB、大小/修改时间、SHA-256 与正式库重复预检，默认排除无效和重复项。确认添加代表完整入库，不是简单复制。
2. **待确认**：聚合历史 discovery manifest，以稳定候选 ID 支持搜索、状态/来源筛选、排序、备注、稍后、拒绝、仅下载与确认添加；仅下载保持 inbox 身份。
3. **自动添加**：默认“自动准备候选”；只有用户显式打开“允许自动完整入库”，候选同时通过主题、分数、标题、DOI/arXiv、开放 PDF、去重规则且未超过单次上限时，才进入正式流水线。
4. **触发方式**：客户端打开知识库后弹窗提供“本次运行 / 今天不再提醒 / 取消”；另有“立即运行”按钮。不创建 Windows 服务、计划任务或客户端关闭后的后台进程。
5. **治理边界**：自动与人工确认的正式入库只执行 A 类编译，禁止 problem/idea、新 Map、正式词表、删除/合并和关键 claim 改写；候选元数据与人工重复覆盖决定保留在受控清单和任务日志中。

0.9.0 验收：Python 44/44、Rust 40/40、Clippy、前端状态/构建/P3/P4/P5、真实 release strict GUI 与 NSIS 隔离安装/启动/卸载通过；两书 Recall@5 保持 1.000 / 0.986667。发布 SHA-256：app `806C7C48542B55D7E9E4A8652048DE8164999B97C4C343E762DBD5A779FB5F09`；MSI `7CFEE878F1B569BD2551FB252A39A6F0881024A9E0F94B7B889F47BC478F9887`；NSIS `88DA4A0FC307AE6FF337A88BF5B12E1EECFA3DF801A46042AD43EE0D0D8B59B0`。

#### 13.17 客户端运行时与搜索修复（已完成 2026-08-09，版本 0.9.1）

0.9.1 修复两个真实安装版 P0 缺陷：点击文献自动检索后界面无响应、弹出 `py.exe` 并因 GBK 中文输出失败；全局搜索因 SQLite FTS5 `snippet()` 参数数量错误而完全失败。

1. 长期编译和文献任务移入 Tauri 阻塞线程池，候选列举、triage 与能力检查也在释放状态锁后后台执行，保持 IPC 和界面响应。
2. 新增共享进程配置：Windows 内部子进程使用 `CREATE_NO_WINDOW`；Python 同时设置 `PYTHONUTF8=1`、`PYTHONIOENCODING=utf-8`，CLI stdout/stderr 再做 UTF-8 兜底。
3. 全局搜索使用六参数 `snippet(pages_fts, 2, '<mark>', '</mark>', ' … ', 24)`，保留 BM25、前缀查询、结果上限与 LIKE fallback。
4. 回归测试实际建立内存 FTS，覆盖 `curr` 高亮、中文查询和空结果；Python 测试在父环境声明 GBK 时验证中文 JSON 仍为 UTF-8。
5. 用户失败运行目录 `raw/inbox/auto-discovered/runs/search-20260809-204315/` 保持原状，不纳入发布提交。

0.9.1 验收：Rust 42/42、Python 45/45、Wiki Eval 10/10、两书 Recall@5 1.000 / 0.986667；前端构建与全部阶段门禁、真实 release strict GUI（含 `curr` 搜索）和 NSIS 隔离安装/启动/退出/卸载全部通过。发布产物：app 20,403,200 bytes，SHA-256 `D095044DEF94BD5FEBAA6A0DD88ADC5258D65610191A46D039EC692DCE3FE0DE`；MSI 9,662,464 bytes，SHA-256 `44BA19C8940AA654DEBE1A77FFA1C5EEB83E06B50DBCBC30A3C336C0F50B4369`；NSIS 6,747,702 bytes，SHA-256 `9823D2D4891BA728693B486ED4F928B07DF50A93743E8E1B44E068C889B6F4C6`。

---

## 14. 变更规则

- 与本 PRD 冲突的实现或临时约定，**以本文件为准**，除非用户显式修订本文件。  
- 修订时：更新对应章节 + 在 §11 决策日志追加一行（日期 + 变更说明）。  
- 词表具体 id 列表、maps 主题名等执行细节可在不违反本 PRD 原则的前提下迭代；**原则性冲突必须先改 PRD**。
