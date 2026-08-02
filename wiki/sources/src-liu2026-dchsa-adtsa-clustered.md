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
updated: 2026-08-01
---

# 动态簇头选择与自适应双阈值充电

## 一句话问题

固定簇头轮换和固定充电触发阈值不能跟踪网络状态时，如何动态协调簇头选择与移动充电器调度。

## 系统设定与假设

- 宽覆盖WRSN被划分为多个簇，由移动充电器持续服务。
- 簇头轮换改变数据聚合、维护和节点能耗，从而改变充电请求触发条件。

## 方法要点

- DCHSA根据簇内剩余能量方差形成候选簇头集，再结合剩余能量及轮换/维护能耗选簇头。
- ADTSA-DEC依据簇头轮换引起的动态能耗调整双阈值。
- 调度策略以PSO等启发式方式协调请求与移动路径。

## 主要结果

- 论文仿真报告其在能耗和网络可靠性方面优于所选基线，并对规模、簇数、权重和时间尺度进行了敏感性分析。

## 局限

- 主要证据来自仿真；密集部署、低时延接入和多充电器协作留待后续。

## 链接

- 方法：[[../methods/mtd-dchsa-adtsa-dec]]
- 综合：[[../syntheses/syn-adaptive-mobile-charger-coordination]] · [[../syntheses/syn-wrsn-scheduling-placement]]
