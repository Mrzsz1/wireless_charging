---
type: method
subtype: algorithm
title: DICCS动态非均匀聚类与混合优先级充电
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, battery, path_or_route]
constraints: [mobility, min_soc]
objectives: [max_completion_rate, min_latency, min_energy]
method_family: heuristic
problem_class: routing_with_charging
updated: 2026-08-01
---

# DICCS动态非均匀聚类与混合优先级充电

## 算法骨架

1. 根据节点位置、剩余能量和能耗率动态重聚类；
2. 以能量和簇内距离加权选择簇头；
3. 分别构造单节点簇与多节点簇停靠点；
4. 按距离、剩余能量和能耗率的混合优先级安排路线。

## 适用边界

- 适合能耗非均匀但拓扑变化速度仍允许重聚类的单车WRSN。
- 多车冲突、任务拆分和在线竞争尚需外层协调器。

## 来源

- [[../sources/src-tian2025-diccs-clustering]]
