---
type: system-model
title: 移动与 UAV 充电的路径—调度联合模型
status: active
epistemic: high
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, battery, path_or_route]
constraints: [mobility, min_soc]
objectives: [max_efficiency, min_energy, max_completion_rate]
method_family: metaheuristic
problem_class: routing_with_charging
updated: 2026-08-11
---

# 移动与 UAV 充电的路径—调度联合模型

## TL;DR

移动充电器必须同时决定“在哪里服务、服务谁、服务多久、按什么顺序移动”。停靠点改变覆盖和充电时间，访问顺序又改变移动能耗，因此路径与调度不能简单独立优化。

## 适用边界

- **适用**：移动充电车或 CUAV；位置、停靠点、服务集合和访问顺序共同影响目标。
- **不适用**：充电器固定且只有开关/功率分配；飞行高度、障碍和动力学占主导但模型只使用二维欧氏距离。

## 统一变量

- $H$：候选/连续悬停点集合；
- $x_h$：是否选择停靠点；
- $a_{hj}$：停靠点 $h$ 是否服务节点 $j$；
- $\pi$：访问顺序；
- $t_h$：停靠充电时间；
- 路径距离与悬停时间共同决定移动充电器能耗。

## 三种求解形态

1. **分解型元启发式**：CUAV JSTOP 分解为停靠点调度 CSOP 与离散路径 CTOP，再用 PSOFKP/PSOD2P 求解；不等同于全局联合最优。
2. **几何/聚类型**：先生成停靠点或簇，再求访问与服务顺序。
3. **序贯决策型**：把节点能量和移动充电器状态放入 Markov game/MDP，用策略学习连续动作。

## 证据

- [[src-liu2021-joint-cuav-scheduling-trajectory]]：系统模型 raw 行 62–119，JSTOP raw 行 120–280。
- [[src-yao2026-ihatrpo-heterogeneous-chargers]]：空地异构连续决策，raw 行 87–230。
- [[src-gao2025-felkh-3d-uav]]：三维方向与 UAV 位置耦合。
- [[src-rahaman2023-obstacle-mcv]]：有障碍多 MCV 的区域与绕行约束。

## 相关页面

- [[obj-energy-and-mobility-cost]] · [[obj-multi-objective-survivability]]
- [[mtd-uav-joint-scheduling-trajectory-pso]] · [[mtd-ihatrpo-heterogeneous-charging]]
- [[syn-mobile-uav-directional-scheduling]]

