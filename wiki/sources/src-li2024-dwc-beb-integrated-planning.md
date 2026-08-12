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
updated: 2026-08-12
---

# 电动公交动态无线充电设施与调度联合规划

## TL;DR

如何同时部署DWC设施、确定BEB电池容量，并在分时电价下安排充电，使设施、电池和充电总成本最小。

## 何时使用 / 何时不使用

- **使用**：电动公交的 DWPT 设施、电池容量与既定班次充电调度需要战略—战术一体化规划。
- **不使用**：车辆到达随机、需要实时改班或交通反馈控制时；案例百分比不能直接迁移到其他城市。

## 系统模型与假设

- 研究对象是具有固定或预设运行时刻表的电动公交系统。
- 战略层决定设施位置和电池容量，战术层决定充电调度。
- 采用真实公交网络案例并比较完整模型与受限模型。

## 变量、目标与约束

- **变量/状态**：DWC 设施位置/容量、公交电池容量、班次上的充电时段与能量、电价时段决策。
- **目标与约束**：最小化设施、电池和充电运营总成本，同时满足公交能量、线路、时刻表和设施容量约束。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- 构建战略部署与战术调度的双层模型。
- 原始MINLP经线性化改写为MILP，由商业求解器求解。
- 将设施成本、电池成本、充电成本和TOU电价耦合。

## 理论性质与复杂度

战略与战术模型合并为集成规划，非线性项经线性化形成 MILP；最优性依赖商业求解器和实例规模。

## 实验设置与基线

以实际公交系统参数进行数值研究，并对成本、电池、设施和充电参数做敏感性分析。

## 定量结果

- 北京公交网络实验报告总成本降低10.12%，充电成本降低23.29%。
- 结果支持将设施选址、电池大小和调度联合设计，而非分别优化。

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- 预设时刻表未覆盖拥堵、临时发车调整、载客量和坡度引起的能耗不确定性。
- 结果受单一公交网络和电价设定影响。

## 证据定位

- Raw：`raw/canonical/Planning_dynamic_wireless_charging_infrastructure_for_battery_electric_bus_systems_with_the_join/full.md`
- 问题：§2，第 85–131 行；战略层：第 132–353 行；战术层：第 354–423 行；集成/线性化：第 424–501 行；实验：第 502–603 行。

## 相关页面

- 概念：[[cpt-dynamic-wireless-charging]]
- 方法：[[mtd-integrated-dwpt-battery-scheduling]]
- 综合：[[syn-dynamic-roadway-wpt-infrastructure]]
