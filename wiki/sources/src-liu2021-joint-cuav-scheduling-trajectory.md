---
type: source
title: "Joint Scheduling and Trajectory Optimization of Charging UAV in Wireless Rechargeable Sensor Networks"
status: active
epistemic: high
year: 2021
venue: "IEEE Internet of Things Journal"
doi: "10.1109/JIOT.2021.3132015"
source_type: paper
acquisition_method: auto_discovery
discovered_via: [openalex]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260714-214003"
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-14
canonicalized_at: 2026-07-14
authors: ["Yanheng Liu", "Hongyang Pan", "Geng Sun", "Aimin Wang", "Jiahui Li", "Shuang Liang"]
paper_keywords: ["Wireless rechargeable sensor networks", "scheduling and trajectory optimization", "unmanned aerial vehicle", "particle swarm optimization"]
keyword_source: index_terms
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, path_or_route]
constraints: [mobility]
objectives: [max_efficiency, min_energy, max_completion_rate]
method_family: metaheuristic
problem_class: routing_with_charging
pdf_path: "raw/canonical/Joint_Scheduling_and_Trajectory_Optimization_of_Charging_UAV_in_Wireless_Rechargeable_Sensor_Net/Joint_Scheduling_and_Trajectory_Optimization_of_Charging_UAV_in_Wireless_Rechargeabl.pdf"
raw_md: "raw/canonical/Joint_Scheduling_and_Trajectory_Optimization_of_Charging_UAV_in_Wireless_Rechargeable_Sensor_Net/full.md"
why_relevant: "将充电UAV悬停点调度与飞行轨迹联合优化，补充移动WRSN的空中充电场景。"
ingest_status: ingested
updated: 2026-08-01
---

# 充电UAV的调度与轨迹联合优化

## 一句话问题

如何同时选择CUAV悬停点、减少重复覆盖并缩短飞行距离，从而为WRSN节点提供更高的整体充电效率。

## 系统设定与假设

- CUAV为空中移动充电器，传感器节点分布在二维网络区域。
- 决策同时包含连续/离散的悬停点选择和离散的访问顺序。
- 目标是为所有节点完成充电，实验采用不同规模和网络设置。

## 方法要点

- 将JSTOP拆分为CUAV调度优化CSOP和轨迹优化CTOP。
- PSOFKP使用可变维度、K-means算子和惩罚-补偿机制求解CSOP。
- PSOD2P使用离散化因子、2-opt和路径交叉缩减求解CTOP。

## 主要结果

- 仿真显示两种改进PSO在多个规模和设置下优于对比算法，稳定性实验支持所引入算子有效。

## 局限

- 结论依赖PSO超参数、二维部署和实验基准；未来需扩展不同网络结构与飞行高度。
- 论文将联合问题拆解后求解，不能直接视为全局最优联合规划。

## 链接

- 概念：[[cpt-directional-charging]]
- 方法：[[mtd-uav-joint-scheduling-trajectory-pso]]
- 综合：[[syn-mobile-uav-directional-scheduling]] · [[syn-mobility-online-service-scheduling]]

