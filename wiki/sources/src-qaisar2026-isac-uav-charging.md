---
type: source
title: "ISAC-Enabled On-Demand UAV Charging for Wireless Rechargeable Sensor Networks"
status: active
epistemic: medium
year: 2026
venue: "arXiv preprint"
doi: ""
source_type: preprint
acquisition_method: auto_discovery
discovered_via: [arxiv]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260801-214329"
triage_status: promoted
selected_by_user: true
acquired_at: 2026-08-01
canonicalized_at: 2026-08-01
authors: ["Muhammad Umar Farooq Qaisar", "Lin Zhang", "Paolo Bellavista", "Shehzad Ashraf Chaudhry", "Shamsher Ullah", "Chang Liu"]
paper_keywords: ["Wireless rechargeable sensor networks", "UAV charging", "wireless power transfer", "integrated sensing and communication", "on-demand scheduling", "partial charging"]
keyword_source: index_terms
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, battery, path_or_route, user_or_request]
constraints: [mobility, min_soc, causality_online]
objectives: [max_efficiency, min_latency, max_completion_rate]
method_family: online_algorithm
problem_class: routing_with_charging
pdf_path: "raw/canonical/ISAC-Enabled_On-Demand_UAV_Charging_for_Wireless_Rechargeable_Sensor_Networks/ISAC-Enabled_On-Demand_UAV_Charging_for_Wireless_Rechargeable_Sensor_Networks.pdf"
raw_md: "raw/canonical/ISAC-Enabled_On-Demand_UAV_Charging_for_Wireless_Rechargeable_Sensor_Networks/full.md"
why_relevant: "把ISAC状态估计、在线优先队列、返航安全和按紧迫度部分充电闭环耦合。"
ingest_status: ingested
updated: 2026-08-12
---

# ISAC辅助的按需UAV充电

## TL;DR

在请求持续到达且UAV位置和飞行状态存在不确定性时，如何联动状态估计、服务排序、飞行轨迹与充电时长。

## 何时使用 / 何时不使用

- **使用**：单 UAV 面对持续到达的低能量请求，需要状态估计、优先队列、部分充电和返航安全闭环。
- **不使用**：多 UAV 冲突、未知接收端移动轨迹的 deadline 保证、禁飞区或抗欺骗是核心时。

## 系统模型与假设

- 基站维护低能量节点请求队列，并作为WPT-UAV的出发和返回仓位。
- ISAC估计UAV距离、速度和位置，周期性更新旅行时间。
- UAV续航、速度、加速度、悬停时间和返航能量受限。

## 变量、目标与约束

- **变量/状态**：请求优先级、估计位置/速度、服务顺序、旅行与悬停时间、部分充电份额和返航能量。
- **目标与约束**：在 UAV 续航与返航约束下提高能量使用效率、缩短路径和充电延迟，并按紧迫度分配有限充电时间。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- 以剩余能量、业务负载、预计旅行时间和飞行方向一致性构成透明优先级。
- 调度结果影响轨迹，新的ISAC状态反过来重排队列。
- 用紧迫度加权的时间分配执行部分充电，并在队列构造中加入返航约束。

## 理论性质与复杂度

属于在线透明评分与状态反馈启发式；原文未报告竞争比、最优性或多 UAV 收敛保证。

## 实验设置与基线

仿真报告能量效率、旅行距离、充电延迟与优先级权重敏感性；没有真实飞行测试床。

## 定量结果

- 论文仿真报告相对代表性基线提高能量使用效率、缩短轨迹并降低充电时延。

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- 单UAV、仿真和简化WPT/飞行模型；多UAV冲突、禁飞区、安全攻击及大规模分布式实现尚未验证。

## 证据定位

- Raw：`raw/canonical/ISAC-Enabled_On-Demand_UAV_Charging_for_Wireless_Rechargeable_Sensor_Networks/full.md`
- 模型：§III，第 52–85 行；队列/状态估计/部分充电：§IV，第 86–150 行；实验：第 151–192 行。

## 相关页面

- 方法：[[../methods/mtd-isac-uav-priority-partial-charging]]
- 综合：[[../syntheses/syn-adaptive-mobile-charger-coordination]] · [[../syntheses/syn-mobile-uav-directional-scheduling]]
