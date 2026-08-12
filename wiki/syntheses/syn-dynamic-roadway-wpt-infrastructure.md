---
type: synthesis
title: 道路动态无线充电基础设施与调度
status: active
epistemic: high
scenario: [ev_dynamic_charging, fleet_charging_ops]
entities: [transmitter, receiver, battery, path_or_route, grid_or_source]
constraints: [mobility, deadline, qos]
objectives: [min_cost, min_energy, max_completion_rate]
method_family: ilp_milp
problem_class: charger_placement
covers:
  - "[[src-honma2026-infinite-drive-dwpt]]"
  - "[[src-li2024-dwc-beb-integrated-planning]]"
gaps:
  - "城市EV连续行程与公交车队调度使用不同网络、时刻表和需求聚合方式。"
  - "交通拥堵、载客量、道路坡度、实时电价和在线车辆请求尚未统一。"
  - "基础设施部署与在线功率分配之间缺少跨层验证。"
updated: 2026-08-12
---

# 道路动态无线充电基础设施与调度

两篇文献都把DWPT从单一充电动作提升为交通系统规划问题，但服务对象、网络规模和目标函数不同。

## 对照

| 文献 | 服务对象 | 规划层 | 调度因素 | 目标 |
|---|---|---|---|---|
| [[src-honma2026-infinite-drive-dwpt]] | 城市EV连续行程 | 交叉口/路段部署与电池容量 | 信号、排队、OD聚合、行程链 | 基础设施与电池权衡 |
| [[src-li2024-dwc-beb-integrated-planning]] | 电动公交车队 | 设施、电池与战术调度 | 时刻表、TOU电价、车队运行 | 总成本 |

## 边界

两者都不是典型的在线请求调度：Honma 侧重路口设施位置与车辆聚合能量平衡，Li 侧重预设公交班次下的战略—战术联合规划。若研究“城市路口部署 + 车辆实时调度”，已有证据只构成部分重叠；仍需重新定义随机车辆到达、拥堵反馈、能量接收曲线、实时电价和道路容量。
