---
type: source
title: "Study on charging strategy of wireless rechargeable sensor networks based on dynamic inhomogeneous clustering"
status: active
epistemic: high
year: 2025
venue: "Scientific Reports"
doi: "10.1038/s41598-025-11569-8"
source_type: paper
acquisition_method: auto_discovery
discovered_via: [serpapi]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260801-214329"
triage_status: promoted
selected_by_user: true
acquired_at: 2026-08-01
canonicalized_at: 2026-08-01
authors: ["Peng Tian", "Jia Yang", "Hongyu Pu", "Xin Tian", "Jiale Tang", "Guozheng Ran", "Liang Peng"]
paper_keywords: ["Dynamic uneven clustering", "Node mortality", "Path planning", "Wireless rechargeable sensor network"]
keyword_source: author_keywords
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, battery, path_or_route]
constraints: [mobility, min_soc]
objectives: [max_completion_rate, min_latency, min_energy]
method_family: heuristic
problem_class: routing_with_charging
pdf_path: "raw/canonical/Study_on_charging_strategy_of_wireless_rechargeable_sensor_networks_based_on_dynamic_inhomogeneo/Study_on_charging_strategy_of_wireless_rechargeable_sensor_networks_based_on_dynamic.pdf"
raw_md: "raw/canonical/Study_on_charging_strategy_of_wireless_rechargeable_sensor_networks_based_on_dynamic_inhomogeneo/full.md"
why_relevant: "把动态非均匀聚类、停靠点、路径与混合优先级统一到移动充电车调度。"
ingest_status: ingested
updated: 2026-08-01
---

# DICCS动态非均匀聚类充电

## 一句话问题

节点能耗和空间分布动态变化时，如何联合调整聚类结构、充电停靠点和访问顺序以减少节点死亡。

## 系统设定与假设

- 单个移动充电车服务按动态非均匀方式划分的WRSN。
- 节点位置、剩余能量和能耗率共同影响分簇及优先级。

## 方法要点

- 改进k-means迭代确定簇数，以初始能量、剩余能量和簇内距离的权重选择簇头。
- 对单节点簇和多节点簇分别确定充电停靠点。
- 用距离、剩余能量和能耗率构成混合优先级，动态调整充电顺序。

## 主要结果

- 论文仿真报告节点死亡率为4.3%，并在等待时间和移动成本上优于SAMER、VTMT与FCFS基线。

## 局限

- 当前是单移动充电车与仿真环境；多车任务分配、路径冲突和节点移动尚未处理。
- 聚类和权重参数对结果敏感，缺少公开统一基准。

## 链接

- 方法：[[../methods/mtd-diccs-dynamic-clustering]]
- 综合：[[../syntheses/syn-adaptive-mobile-charger-coordination]] · [[../syntheses/syn-wrsn-scheduling-placement]]
