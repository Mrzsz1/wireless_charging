---
type: source
title: "Obstacles Avoidance Charging Schedule for Multiple Mobile Charging Vehicles in Wireless Rechargeable Sensor Networks"
status: active
epistemic: medium
year: 2023
venue: "Research Square preprint; IJCNDS journal record discovered in 2026"
doi: "10.21203/rs.3.rs-3468314/v1"
source_type: preprint
acquisition_method: auto_discovery
discovered_via: [serpapi]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260801-214329"
triage_status: promoted
selected_by_user: true
acquired_at: 2026-08-01
canonicalized_at: 2026-08-01
authors: ["Sk Md Abidar Rahaman", "Md Azharuddin", "Pratyay Kuila"]
paper_keywords: ["Wireless rechargeable sensor networks", "charging scheduling", "Mobile charging vehicles", "Obstacles", "Joint charging preference"]
keyword_source: author_keywords
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, path_or_route, user_or_request]
constraints: [mobility, deadline]
objectives: [max_completion_rate, min_energy, min_latency]
method_family: heuristic
problem_class: routing_with_charging
pdf_path: "raw/canonical/Obstacles_avoidance_charging_schedule_for_multiple_mobile_charging_vehicles_in_wireless_recharge/Obstacles_avoidance_charging_schedule_for_multiple_mobile_charging_vehicles_in_wirel.pdf"
raw_md: "raw/canonical/Obstacles_avoidance_charging_schedule_for_multiple_mobile_charging_vehicles_in_wireless_recharge/full.md"
why_relevant: "在多移动充电车调度中显式处理障碍绕行、空间分区和时空事件偏好。"
ingest_status: ingested
updated: 2026-08-01
---

# 有障碍多移动充电车调度

## 一句话问题

多个移动充电车在障碍环境中如何分区服务、排序请求并生成可绕行的充电路线。

## 系统设定与假设

- WRSN被划分为多个子区域，每个区域分配一辆移动充电车。
- 障碍物迫使路线绕行，并改变旅行能耗、响应时间和可服务节点数量。

## 方法要点

- 用时间、空间和事件偏好排序充电请求。
- 先聚类划分服务区域，再按joint charging preference构造充电计划。
- 用锚点和投影点生成障碍绕行路径。

## 主要结果

- 论文用仿真及ANOVA/LSD检验与两个基线比较，报告在吞吐、能量利用、响应时间、死亡节点和旅行距离等指标上改善。

## 局限

- canonical全文是2023 Research Square预印本；2026期刊记录的最终排版全文未取得，二者版本差异待复核。
- 障碍为二维几何设定，未纳入三维、动态障碍和数据传输deadline。

## 链接

- 方法：[[../methods/mtd-obstacle-aware-multi-mcv]]
- 综合：[[../syntheses/syn-adaptive-mobile-charger-coordination]] · [[../syntheses/syn-mobile-uav-directional-scheduling]]
