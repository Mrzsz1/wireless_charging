---
type: source
title: "Collaborative Charging Optimization for Wireless Rechargeable Sensor Networks via Heterogeneous Mobile Chargers"
status: active
epistemic: medium
year: 2025
venue: "arXiv preprint; IEEE Internet of Things Journal early-access DOI (2026)"
doi: "10.1109/JIOT.2026.3696627"
source_type: preprint
acquisition_method: auto_discovery
discovered_via: [arxiv]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260801-214329"
triage_status: promoted
selected_by_user: true
acquired_at: 2026-08-01
canonicalized_at: 2026-08-01
authors: ["Jianhang Yao", "Hui Kang", "Geng Sun", "Jiahui Li", "Hongjuan Li", "Jiacheng Wang", "Yinqiu Liu"]
paper_keywords: ["Wireless rechargeable sensor network", "collaborative charging optimization", "heterogeneous mobile chargers", "trust region policy optimization"]
keyword_source: index_terms
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, battery, path_or_route]
constraints: [mobility, min_soc]
objectives: [max_efficiency, min_energy, max_completion_rate, multi_objective]
method_family: rl
problem_class: routing_with_charging
pdf_path: "raw/canonical/Collaborative_Charging_Optimization_for_Wireless_Rechargeable_Sensor_Networks_via_Heterogeneous/Collaborative_Charging_Optimization_for_Wireless_Rechargeable_Sensor_Networks_via_He.pdf"
raw_md: "raw/canonical/Collaborative_Charging_Optimization_for_Wireless_Rechargeable_Sensor_Networks_via_Heterogeneous/full.md"
why_relevant: "以空中AAV和地面SV组成异构移动充电器，联合优化充电效率、移动能耗和节点死亡率。"
ingest_status: ingested
updated: 2026-08-01
---

# 异构移动充电器协同优化

## 一句话问题

如何让续航和机动性不同的空中充电器与地面充电器在动态WRSN中形成互补分工。

## 系统设定与假设

- AAV与地面智能车SV共同服务传感器节点，动作空间连续且多智能体异构。
- 同时考虑充电效率、移动能耗和节点死亡率，环境状态随节点能量变化。
- 当前实验区域不含障碍物。

## 方法要点

- 将协同调度表述为Markov game。
- IHATRPO在异构agent trust-region策略优化中加入自注意力状态处理和Beta采样。
- 由学习到的策略形成空地充电器的区域分工。

## 主要结果

- 论文报告IHATRPO相对原始HATRPO总体性能提高51%。
- 在其实验设置中，节点死亡率由超过90%降至10%以下，并观察到AAV与SV的互补覆盖模式。

## 局限

- 结论来自仿真，且未纳入障碍、超大规模网络和更多异构充电代理。
- 多目标reward及结果依赖训练分布，不能直接等同于确定性近似保证。

## 链接

- 方法：[[../methods/mtd-ihatrpo-heterogeneous-charging]]
- 综合：[[../syntheses/syn-adaptive-mobile-charger-coordination]] · [[../syntheses/syn-mobile-uav-directional-scheduling]]
