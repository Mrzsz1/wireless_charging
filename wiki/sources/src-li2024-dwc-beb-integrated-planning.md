---
type: source
title: "Planning dynamic wireless charging infrastructure for battery electric bus systems with the joint optimization of charging scheduling"
status: active
epistemic: high
year: 2024
venue: "Transportation Research Part C"
doi: "10.1016/j.trc.2023.104469"
source_type: paper
acquisition_method: auto_discovery
discovered_via: [openalex]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260714-214003"
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-14
canonicalized_at: 2026-07-14
authors: ["Wenlong Li", "Yi He", "Songhua Hu", "Zhengbing He", "Carlo Ratti"]
paper_keywords: ["Infrastructure planning", "Charging scheduling", "Integrated optimization", "Dynamic wireless charging", "Battery electric bus"]
keyword_source: author_keywords
scenario: [ev_dynamic_charging, fleet_charging_ops]
entities: [transmitter, receiver, battery, grid_or_source, path_or_route]
constraints: [mobility, deadline, qos]
objectives: [min_cost, min_energy, max_completion_rate]
method_family: ilp_milp
problem_class: demand_response_tariff
pdf_path: "raw/canonical/Planning_dynamic_wireless_charging_infrastructure_for_battery_electric_bus_systems_with_the_join/Planning_dynamic_wireless_charging_infrastructure_for_battery_electric_bus_systems_w.pdf"
raw_md: "raw/canonical/Planning_dynamic_wireless_charging_infrastructure_for_battery_electric_bus_systems_with_the_join/full.md"
why_relevant: "将公交DWC设施部署、电池容量、TOU电价下充电调度纳入一个两层规划模型。"
ingest_status: ingested
updated: 2026-08-01
---

# 电动公交动态无线充电设施与调度联合规划

## 一句话问题

如何同时部署DWC设施、确定BEB电池容量，并在分时电价下安排充电，使设施、电池和充电总成本最小。

## 系统设定与假设

- 研究对象是具有固定或预设运行时刻表的电动公交系统。
- 战略层决定设施位置和电池容量，战术层决定充电调度。
- 采用真实公交网络案例并比较完整模型与受限模型。

## 方法要点

- 构建战略部署与战术调度的双层模型。
- 原始MINLP经线性化改写为MILP，由商业求解器求解。
- 将设施成本、电池成本、充电成本和TOU电价耦合。

## 主要结果

- 北京公交网络实验报告总成本降低10.12%，充电成本降低23.29%。
- 结果支持将设施选址、电池大小和调度联合设计，而非分别优化。

## 局限

- 预设时刻表未覆盖拥堵、临时发车调整、载客量和坡度引起的能耗不确定性。
- 结果受单一公交网络和电价设定影响。

## 链接

- 概念：[[cpt-dynamic-wireless-charging]]
- 方法：[[mtd-integrated-dwpt-battery-scheduling]]
- 综合：[[syn-dynamic-roadway-wpt-infrastructure]]

