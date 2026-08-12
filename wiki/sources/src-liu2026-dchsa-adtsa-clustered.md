---
type: source
title: "Charging Scheduling of Clustered Wireless Rechargeable Sensor Networks Considering Dynamic Selection of Cluster Heads"
status: active
epistemic: high
year: 2026
venue: "Computers, Materials & Continua"
doi: "10.32604/cmc.2026.078181"
source_type: paper
acquisition_method: auto_discovery
discovered_via: [serpapi]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260801-214329"
triage_status: promoted
selected_by_user: true
acquired_at: 2026-08-01
canonicalized_at: 2026-08-01
authors: ["Mengqi Liu", "Haiqing Yao"]
paper_keywords: ["Clustered wireless rechargeable sensor networks", "cluster head rotation", "adaptive dual-threshold", "charging scheduling strategy", "particle swarm optimization"]
keyword_source: author_keywords
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, battery, path_or_route]
constraints: [mobility, min_soc]
objectives: [max_completion_rate, min_energy]
method_family: heuristic
problem_class: routing_with_charging
pdf_path: "raw/canonical/Charging_Scheduling_of_Clustered_Wireless_Rechargeable_Sensor_Networks_Considering_Dynamic_Selec/Charging_Scheduling_of_Clustered_Wireless_Rechargeable_Sensor_Networks_Considering_D.pdf"
raw_md: "raw/canonical/Charging_Scheduling_of_Clustered_Wireless_Rechargeable_Sensor_Networks_Considering_Dynamic_Selec/full.md"
why_relevant: "把簇头轮换造成的动态能耗显式反馈到移动充电触发阈值和调度。"
ingest_status: ingested
updated: 2026-08-12
---

# 动态簇头选择与自适应双阈值充电

## TL;DR

固定簇头轮换和固定充电触发阈值不能跟踪网络状态时，如何动态协调簇头选择与移动充电器调度。

## 何时使用 / 何时不使用

- **使用**：簇头轮换显著改变节点能耗，充电触发阈值必须随网络角色和动态耗能更新。
- **不使用**：无聚类路由、多充电器实时协作或需要严格 deadline/最优性保证时。

## 系统模型与假设

- 宽覆盖WRSN被划分为多个簇，由移动充电器持续服务。
- 簇头轮换改变数据聚合、维护和节点能耗，从而改变充电请求触发条件。

## 变量、目标与约束

- **变量/状态**：候选簇头、剩余能量方差、轮换/维护能耗、上下充电阈值、请求集合和移动充电路径。
- **目标与约束**：通过动态簇头选择和自适应双阈值降低耗能与死亡风险，再由调度策略处理产生的请求。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- DCHSA根据簇内剩余能量方差形成候选簇头集，再结合剩余能量及轮换/维护能耗选簇头。
- ADTSA-DEC依据簇头轮换引起的动态能耗调整双阈值。
- 调度策略以PSO等启发式方式协调请求与移动路径。

## 理论性质与复杂度

DCHSA、ADTSA-DEC 与后续 PSO/启发式调度未给出近似比；证据为模型推导和仿真实验。

## 实验设置与基线

报告仿真参数、关键参数敏感性、两个子算法、补充对照及现实场景数值实验。

## 定量结果

- 论文仿真报告其在能耗和网络可靠性方面优于所选基线，并对规模、簇数、权重和时间尺度进行了敏感性分析。

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- 主要证据来自仿真；密集部署、低时延接入和多充电器协作留待后续。

## 证据定位

- Raw：`raw/canonical/Charging_Scheduling_of_Clustered_Wireless_Rechargeable_Sensor_Networks_Considering_Dynamic_Selec/full.md`
- 模型：§3，第 77–127 行；DCHSA：第 132–194 行；ADTSA-DEC：第 195–298 行；总体调度：第 299–364 行；实验：第 365–509 行。

## 相关页面

- 方法：[[../methods/mtd-dchsa-adtsa-dec]]
- 综合：[[../syntheses/syn-adaptive-mobile-charger-coordination]] · [[../syntheses/syn-wrsn-scheduling-placement]]
