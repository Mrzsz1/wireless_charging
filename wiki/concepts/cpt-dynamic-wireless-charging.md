---
type: concept
title: 动态无线充电 Dynamic Wireless Charging
status: active
epistemic: high
scenario: [ev_dynamic_charging]
entities: [transmitter, receiver, battery, path_or_route]
constraints: [mobility, deadline]
objectives: [min_cost, min_energy]
method_family: ""
problem_class: charger_placement
updated: 2026-08-01
---

# 动态无线充电 Dynamic Wireless Charging

动态无线充电（DWPT/DWC）允许车辆在行驶、减速或信号交叉口排队过程中获得能量。调度问题通常同时涉及道路设施位置、车辆电池容量、车辆轨迹和充电时机。

## 与本库文献的关系

- [[src-honma2026-infinite-drive-dwpt]]：信号交叉口、排队停留和连续城市运行。
- [[src-li2024-dwc-beb-integrated-planning]]：电动公交设施部署、电池容量和TOU充电调度联合规划。

## 边界

DWPT设施部署不等价于在线充电请求调度；交通网络、信号、车队时刻表和电池状态必须明确后才能比较方法。
