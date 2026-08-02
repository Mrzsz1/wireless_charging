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
updated: 2026-08-01
---

# 双层元启发式充电与能量耗尽规避

## 一句话问题

如何联合确定移动充电路径和充电时间，以减少充电过程后的死亡传感器节点数量。

## 系统设定与假设

- 面向WRSN的移动充电场景，节点能量耗尽是主要失效指标。
- 充电路径和时间形成双层搜索问题，解空间大且复杂。

## 方法要点

- MLSGA用多起点搜索探索空间，再以遗传算法利用可行区域。
- MTBCS以多任务框架和协方差自适应进化策略优化低层充电时间。
- 通过元启发式联合处理路径与时间，而不是将两者固定为单一阶段。

## 主要结果

- 网络场景实验显示两种算法相较基准显著减少死亡节点；具体优势依赖实验规模和参数。

## 局限

- 这是预印本算法评估，尚缺统一公开基准和真实硬件验证。
- 元启发式结果依赖初始化、超参数和停止条件，不能直接替代可证明的最优性保证。

## 链接

- 方法：[[mtd-bilevel-metaheuristic-charging]]
- 综合：[[syn-mobile-uav-directional-scheduling]] · [[syn-mobility-online-service-scheduling]]

