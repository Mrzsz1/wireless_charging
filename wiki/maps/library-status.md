---
type: map
title: 库水位 Library Status
status: active
source_count: 23
paper_source_count: 21
book_source_count: 2
core_reference_book_count: 2
book_chapter_count: 61
book_page_count: 1171
synthesis_count: 7
method_count: 20
concept_count: 7
system_model_count: 4
objective_count: 4
dataset_or_sim_count: 1
source_research_profile_count: 21
method_research_profile_count: 20
source_metadata_complete_count: 23
source_with_paper_keywords_count: 20
paper_keyword_occurrence_count: 90
distinct_literal_keyword_count: 74
normalized_domain_keyword_count: 74
eval_case_count: 10
problem_count: 1
idea_count: 0
canonical_manual_count: 9
canonical_auto_count: 12
canonical_auto_pending_ingest_count: 0
compiled_auto_source_count: 12
auto_discovery_run_count: 6
auto_candidate_pending_count: 46
auto_candidate_selected_count: 6
auto_candidate_rejected_count: 14
auto_candidate_promoted_count: 12
manual_drop_pending_count: 0
year_min: 2017
year_max: 2026
last_ingest_at: 2026-08-01
last_content_update_at: 2026-08-12
updated: 2026-08-12
---

# 库水位

> `/solve` 与 `/novelty` **必须**引用本节。

## 摘要

| 指标 | 值 |
|------|-----|
| source 页数量 | **23**（21篇论文/预印本 + 2本核心专著） |
| synthesis 页数量 | **7** |
| method / concept | **20 / 7** |
| system-model / objective / dataset-or-sim | **4 / 4 / 1** |
| source 元数据完整度 | **23/23** 已有 year、venue、来源与raw路径；论文、预印本与专著分开标注 |
| 论文关键词覆盖 | **20/21** 有作者 Keywords / Index Terms；90 次出现、74 个大小写归一原词 |
| canonical 来源 | 手动 **11**（9篇论文 + 2本专著）· 自动发现 **12**（已完成 A 编译） |
| 自动发现候选 | **46 pending** · **6 selected/全文受限** · **14 rejected** · **12 promoted/已编译** |
| 手动投放待处理 | **0** |
| 文献年份跨度 | **2017–2026**（正式卷期与预印本年份分开解释） |
| 上次 ingest | **2026-08-01** |
| 最近转换 | **2026-08-01**（最新5篇公开 PDF 由 MinerU 转换，0失败） |
| 内容层最近更新 | **2026-08-12**（P2 完成 21 篇论文 source 与 20 个 method 的研究档案化，并升级原文证据评测） |
| 核心 WRSN 充电 | 8 篇 active |
| 边界文献 | 1 篇 UAV-BS QoS（needs_review） |
| vocab 版本 | `2026-07-10-seed-v1`（draft_seed） |

## 采集渠道与生命周期

| 采集渠道 | Inbox pending | Inbox selected | Canonical / 已编译 source |
|----------|---------------|----------------|---------------------------|
| `manual_upload` | 0 | — | **11 / 11**（9篇论文 + 2本专著） |
| `auto_discovery` | **46** | **6** | **12 / 12** |

`manual/auto` 是来源，`pending/selected/promoted` 是筛选状态，`pending_ingest/ingested` 是编译状态。自动发现候选只有晋升 canonical 并完成 A 编译后，才计入 `source_count`。

机器辅助初筛见 [[../../logs/2026-07-14-auto-candidate-triage-recommendations|早期审阅报告]] 与 [[../../logs/2026-08-01-2143-latest-literature-ingest|最新增量报告]]。当前自动发现累计12篇已晋升并完成A编译；仍有6项selected等待开放全文，46项pending待后续筛选。

## 本轮已完成 A 编译（自动发现，7）

- [[../sources/src-dai2022-rose-robust-safe-charging]]：ROSE鲁棒安全充电
- [[../sources/src-liu2021-joint-cuav-scheduling-trajectory]]：充电UAV调度与轨迹
- [[../sources/src-gao2024-ra-dmcs-asymmetric-directional]]：路由非对称定向移动充电
- [[../sources/src-li2024-dwc-beb-integrated-planning]]：BEB动态充电基础设施规划
- [[../sources/src-binh2025-bilevel-metaheuristic-charging]]：双层元启发式充电
- [[../sources/src-gao2025-felkh-3d-uav]]：三维定向UAV充电
- [[../sources/src-honma2026-infinite-drive-dwpt]]：信号交叉口DWPT部署

## 领域关键词水位

- 入口：[[map-domain-keywords|领域关键词地图]]；治理：[[../../schema/domain-keywords|三层关键词规则]]。
- 20/21 篇论文/预印本 source 提供明确 Keywords / Index Terms；[[../sources/src-wu-charging-on-the-move]] 未检出作者关键词，不做摘要推断。
- 当前没有因为词频直接修改 `vocab.yaml`；受控词仍遵循 proposal → 用户确认闸门。

## 最近 ingest

| 日期 | 说明 | source 页 |
|------|------|-----------|
| 2026-07-10 | 首批 9 篇 raw/canonical 全文 A 编译 | 见 [[../index|index]] |
| 2026-08-01 | 自动发现 7 篇 canonical 完成 A 编译 | 见 [[../index|index]] |
| 2026-08-01 | 最新自动发现5篇完成下载、MinerU和A编译 | [[../../logs/2026-08-01-2143-latest-literature-ingest|详情]] |

## 内容健康度

| 指标 | 当前值 | 说明 |
|------|--------|------|
| source : synthesis | 23 : 7 | 最新自动发现批次已纳入跨文献比较 |
| 问答回归用例 | 10 | Gold Contract v2：5 solve · 3 novelty · 2 relationship；逐题要求 Wiki + paper、允许 source ID、原文章节/行号、关键约束与边界 |
| 可复用知识层 | 4 system-model · 4 objective · 1 dataset-or-sim | 从模型/目标进入，不依赖先知道论文名 |
| 研究档案详细度 | 21/21 论文 source · 20/20 method | source 覆盖模型、变量、目标、算法、理论、实验、定量结果、局限与 raw 行号；method 覆盖输入输出、步骤、保证与失效边界 |
| 正式 B 页 | 1 problem / 0 idea | [[../problems/prob-joint-deployment-online-interference]] 已获用户授权；算法 idea 因硬件动作假设未锁定而暂缓 |

## 领域重心（本批）

- 传感器网 **RF 无线可充电（WRSN）** 放置 / 并发 / 定向 / 在线请求  
- 较少：EV 动态无线充电、纯磁耦合近场调度  

## 核心专著水位

- 专著：**2**（`Algorithmic Game Theory`、`Approximation Algorithms`）
- 章节：**61**（含 front matter；正文章节 59）
- PDF 页数：**1171**
- 质量门禁：`raw/canonical/core-books-quality.json`，两本书最小 token recall **1.000**，最小 token precision 分别 **0.956444 / 0.986173**，均通过 95% 阈值。
- 检索：运行 `py -3 tools/core_reference_search.py "<问题>" --limit 8`，结果必须携带章节和 PDF physical pages。
- 检索回归：`evals/core-book-retrieval-report.json` 共 295 条章节种子查询；Algorithmic Game Theory Recall@5 **1.000**，Approximation Algorithms Recall@5 **0.986667**，均通过 95%。

## 2026-08-01 最新文献自动增量

- 检索：5个主题、4个来源；原始命中373，去重并按2025年以后过滤后269，历史去重后新增269；保存Top 40。
- 本轮候选：32 pending / 3 selected（开放全文下载失败）/ 5 promoted并已编译。
- 新增source：5；当前总水位：**23 source = 21 papers/preprints + 2 books**。
- 新增方法：IHATRPO、DICCS、DCHSA+ADTSA-DEC、ISAC部分充电队列、有障碍多MCV调度。
- 新增综合：[[../syntheses/syn-adaptive-mobile-charger-coordination]]。
- 论文关键词：**20/21** source有作者关键词或Index Terms，共90次、74个大小写归一原词。
- 最新全文锚点：[[../sources/src-qaisar2026-isac-uav-charging]]，arXiv v1日期为2026-07-26。
