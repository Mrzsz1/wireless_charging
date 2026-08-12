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
updated: 2026-08-12
---

# 有障碍多移动充电车调度

## TL;DR

多个移动充电车在障碍环境中如何分区服务、排序请求并生成可绕行的充电路线。

## 何时使用 / 何时不使用

- **使用**：二维静态障碍环境中，多辆 MCV 可先分区，再按时空事件偏好服务并绕障。
- **不使用**：动态障碍、三维路径、跨区抢占或硬实时 deadline；预印本与后续期刊版本差异尚未核验。

## 系统模型与假设

- WRSN被划分为多个子区域，每个区域分配一辆移动充电车。
- 障碍物迫使路线绕行，并改变旅行能耗、响应时间和可服务节点数量。

## 变量、目标与约束

- **变量/状态**：网络分区与 MCV 分配、请求优先级、访问序列、障碍锚点/投影点和绕行路径。
- **目标与约束**：兼顾吞吐、能量利用、响应时间、死亡节点和移动距离，并满足充电与绕障时间约束。
- 公式与原文符号以 canonical Markdown 对应章节为准；本页不把 OCR 不稳定公式重排为新定义。

## 算法流程

- 用时间、空间和事件偏好排序充电请求。
- 先聚类划分服务区域，再按joint charging preference构造充电计划。
- 用锚点和投影点生成障碍绕行路径。

## 理论性质与复杂度

三阶段聚类—排序—绕障为启发式，原文未给出全局最优或近似比；统计检验只支撑所测实例。

## 实验设置与基线

比较多个运行时长/网络规模/障碍数量指标，并用 ANOVA/LSD 做统计验证。

## 定量结果

- 论文用仿真及ANOVA/LSD检验与两个基线比较，报告在吞吐、能量利用、响应时间、死亡节点和旅行距离等指标上改善。

所有百分比和排序只在论文自己的模型、参数与 baseline 下成立，不跨论文直接横比。

## 局限与失效条件

- canonical全文是2023 Research Square预印本；2026期刊记录的最终排版全文未取得，二者版本差异待复核。
- 障碍为二维几何设定，未纳入三维、动态障碍和数据传输deadline。

## 证据定位

- Raw：`raw/canonical/Obstacles_avoidance_charging_schedule_for_multiple_mobile_charging_vehicles_in_wireless_recharge/full.md`
- 模型：§3，第 87–192 行；三阶段方案：§4，第 193–348 行；仿真与统计：第 349–479 行。

## 相关页面

- 方法：[[../methods/mtd-obstacle-aware-multi-mcv]]
- 综合：[[../syntheses/syn-adaptive-mobile-charger-coordination]] · [[../syntheses/syn-mobile-uav-directional-scheduling]]
