---
type: source
title: "Minimizing the energy depletion in wireless rechargeable sensor networks using bi-level metaheuristic charging schemes"
status: active
epistemic: medium
year: 2025
venue: "arXiv preprint"
doi: ""
source_type: preprint
acquisition_method: auto_discovery
discovered_via: [arxiv]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260714-204713"
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-14
canonicalized_at: 2026-07-14
authors: ["Huynh Thi Thanh Binh", "Le Van Cuong", "Dang Hai Dang", "Le Trong Vinh"]
paper_keywords: ["Wireless rechargeable sensor network", "energy depletion", "bi-level optimization", "evolutionary strategy", "multi-start local search", "multitasking"]
keyword_source: author_keywords
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, path_or_route, time_slot]
constraints: [mobility, min_soc]
objectives: [max_completion_rate, min_energy]
method_family: metaheuristic
problem_class: routing_with_charging
pdf_path: "raw/canonical/Minimizing_the_energy_depletion_in_wireless_rechargeable_sensor_networks_using_bi-level_metaheur/Minimizing_the_energy_depletion_in_wireless_rechargeable_sensor_networks_using_bi-le.pdf"
raw_md: "raw/canonical/Minimizing_the_energy_depletion_in_wireless_rechargeable_sensor_networks_using_bi-level_metaheur/full.md"
why_relevant: "以死亡节点数量为核心指标，联合优化移动充电路径与充电时间。"
ingest_status: ingested
updated: 2026-08-12
---

# 双层元启发式充电与能量耗尽规避

## TL;DR

如何联合确定移动充电路径和充电时间，以减少充电过程后的死亡传感器节点数量。

## 何时使用 / 何时不使用

- **使用**：移动充电路径和停留时间强耦合，且主要目标是减少死亡/耗尽节点，允许用元启发式换取可扩展搜索。
- **不使用**：需要确定性最优、竞争比或实时 deadline 保证时；预印本的随机算法结果不能当作严格保证。

## 系统模型与假设

- 面向WRSN的移动充电场景，节点能量耗尽是主要失效指标。
- 充电路径和时间形成双层搜索问题，解空间大且复杂。

## 变量、目标与约束

- **变量/状态**：上层路径/访问序列、候选充电路径集合；下层各停留点充电时间。节点能量、耗能率、移动时间与充电时间共同决定死亡节点数。
- **目标与约束**：首要减少能量耗尽节点，并兼顾充电路径和时间可行性；总充电时间的推导与双层目标见原文问题定义。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- MLSGA用多起点搜索探索空间，再以遗传算法利用可行区域。
- MTBCS以多任务框架和协方差自适应进化策略优化低层充电时间。
- 通过元启发式联合处理路径与时间，而不是将两者固定为单一阶段。

## 理论性质与复杂度

MLSGA 与 MTBCS 都是随机元启发式；原文报告运行时间分析，但未给出全局最优、近似比或确定性收敛保证。

## 实验设置与基线

多组网络实例，列出算法参数、评价指标、节点参数敏感性和运行时间；应同时报告随机重复与停止条件。

## 定量结果

- 网络场景实验显示两种算法相较基准显著减少死亡节点；具体优势依赖实验规模和参数。

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- 这是预印本算法评估，尚缺统一公开基准和真实硬件验证。
- 元启发式结果依赖初始化、超参数和停止条件，不能直接替代可证明的最优性保证。

## 证据定位

- Raw：`raw/canonical/Minimizing_the_energy_depletion_in_wireless_rechargeable_sensor_networks_using_bi-level_metaheur/full.md`
- 模型与公式：§3，原文第 67–231 行；MLSGA：§4，第 232–375 行；MTBCS：§5，第 376–521 行；实验：§6，第 522–736 行。

## 相关页面

- 方法：[[mtd-bilevel-metaheuristic-charging]]
- 综合：[[syn-mobile-uav-directional-scheduling]] · [[syn-mobility-online-service-scheduling]]
