# 页面类型规范

共 **9 类**。`method` 与 `algorithm` 合并为 `method`，用 `subtype` 区分。

## 总表

| type | 目录 | 文件前缀 | 阶段 | 职责 |
|------|------|----------|------|------|
| `source` | `wiki/sources/` | `src-` | A | 单篇论文/专利结构化卡 |
| `concept` | `wiki/concepts/` | `cpt-` | A | 术语与定义（中英对照） |
| `system-model` | `wiki/system-models/` | `sys-` | A | 场景、实体、系统假设 |
| `objective` | `wiki/objectives/` | `obj-` | A | 优化目标与约束族 |
| `method` | `wiki/methods/` | `mtd-` | A | 方法/算法骨架 |
| `dataset-or-sim` | `wiki/datasets-sims/` | `data-` | A | 数据、仿真、评测协议 |
| `synthesis` | `wiki/syntheses/` | `syn-` | A | 多源对照、并列冲突、gap |
| `problem` | `wiki/problems/` | `prob-` | **B** | research problem（须用户确认后写入） |
| `idea` | `wiki/ideas/` | `idea-` | **B** | 候选思路（须用户确认后写入） |

地图页：`wiki/maps/`，前缀 `map-`（或固定名如 `library-status.md`）。

## A 类页面准入规则

新页面必须提升复用或检索价值，不为“把目录填满”而拆页：

| 类型 | 独立建页条件 |
|------|--------------|
| `source` | 每篇 canonical 文献一页；这是唯一默认“一文一页”的类型 |
| `concept` / `method` | 至少被 2 个 source 使用；或虽为单源，但属于领域核心锚点且会被真实 `/solve`、`/novelty` 问题复用 |
| `system-model` / `objective` / `dataset-or-sim` | 能承载跨 source 对照、可复现实验协议或重复使用的模型/目标；仅复述单篇 source 时留在 source 页 |
| `synthesis` | 至少覆盖 2 个 source，并明确给出对照维度和 gap；不得只是链接列表 |

执行细则：

1. 暂不满足准入条件的内容保留在 source 的对应小节，并用 wikilink 指向已有公共页。
2. 单源页面不是自动删除对象；Lint 只标记“合并候选”，删除、合并和关键 claim 改写仍需用户确认。
3. 新 map、`problem`、`idea` 继续遵守人工确认闸门，准入规则不扩大 Agent 权限。
4. 每次批量 ingest 后优先增加跨文献 synthesis，而不是继续拆出低复用度原子页。

## 命名

- 英文 slug，小写，连字符：`src-zhang2023-online-wpt-scheduling.md`
- 中文放 H1/正文，**尽量不进文件名**
- raw 中 PDF 与 MinerU md **同名**；wiki source 通过 `pdf_path` / `raw_md` 关联

## 各类型正文建议结构

### source

Frontmatter 必须保留 `acquisition_method`、`triage_status`、`ingest_status`；自动发现项还要保留 `discovered_via` 与 `discovery_run`。作者关键词写入 `paper_keywords` 并标明 `keyword_source`，按 `schema/domain-keywords.md` 进入领域导航，不直接扩展正式词表。来源追踪只用于审计和筛选，不改变论文事实权重。

1. 一句话问题  
2. 系统设定 / 假设  
3. 方法要点  
4. 主要结果  
5. 局限  
6. 与本库其他页的链接  

禁止：写「我们将…」「本文贡献可改为…」类个人贡献句。

### concept

- 中文名 + 英文术语  
- 定义（可多源并列）  
- 易混概念  
- 相关 `[[links]]`

### system-model

- 适用 scenario（词表 id）  
- 实体与关系  
- 关键假设（可充电、移动性、信道等）  
- 来源 links  

### objective

- 优化目标（词表）  
- 常见约束  
- 与其他目标的权衡（并列，不裁断优劣）  

### method

- `subtype`: `method` | `algorithm`  
- `method_family`（词表）  
- 输入/输出与适用条件  
- 复杂度或在线/离线特性（若文献有）  
- 来源 sources  

### dataset-or-sim

- 公开数据或仿真设定  
- 评测指标  
- 可复现注意点  

### synthesis

- 对照表或分节并列  
- **冲突只并列不裁断**  
- Gaps（供 B 阶段使用，但不在此写 idea 正文）  

### problem / idea（B only）

- 必须字段：`inspired_by` 或正文中的来源链接（至少 1 个 source/synthesis/gap）  
- idea 不得假装已有实验结论  
- 写入前 **必须用户确认**
