---
type: method
subtype: algorithm
title: ISAC辅助优先队列与部分充电UAV调度
status: active
epistemic: medium
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, battery, path_or_route, user_or_request]
constraints: [mobility, min_soc, causality_online]
objectives: [max_efficiency, min_latency, max_completion_rate]
method_family: online_algorithm
problem_class: routing_with_charging
updated: 2026-08-01
---

# ISAC辅助优先队列与部分充电UAV调度

## 算法骨架

1. 由剩余能量、流量、旅行时间和飞行方向一致性计算请求优先级；
2. 用ISAC状态估计持续更新UAV位置、速度与旅行时间；
3. 状态变化触发队列重排和轨迹调整；
4. 按紧迫度分配有限悬停时间实施部分充电；
5. 在接纳请求时预留返航能量。

## 适用边界

- 适合在线请求和移动状态估计共同变化的单UAV场景。
- 多UAV、欺骗/干扰、禁飞区和精细飞行动力学需要额外模块。

## 来源

- [[../sources/src-qaisar2026-isac-uav-charging]]
