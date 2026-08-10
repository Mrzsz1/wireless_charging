---
type: system-model
title: 异构移动充电器协同模型
status: active
epistemic: medium
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, battery, path_or_route]
constraints: [mobility, min_soc]
objectives: [max_efficiency, min_energy, max_completion_rate, multi_objective]
method_family: rl
problem_class: routing_with_charging
updated: 2026-08-11
---

# 异构移动充电器协同模型

## TL;DR

当空中 AAV 与地面 SV、或多种 MCV 的速度、能耗、可达区域不同，不能用“同质 agent 复制”描述。模型必须保留各 agent 的动作边界、能量预算和服务能力，同时定义共享状态与协同目标。

## 使用条件

- 至少两类移动充电器，能力或可达域不同；
- 任务需要显式分工而非简单增加同类车辆；
- 状态随节点能量消耗持续变化。

## 形式化骨架

异构 agent 集合 $A$ 中，每个 agent $i$ 有动作空间 $\mathcal A_i$、移动/能耗模型和策略 $\pi_i$。IHATRPO 使用全局状态（节点能量与位置、AAV/SV 位置），每个 agent 输出有界连续移动动作，并以充电效率、移动距离和节点死亡率的加权奖励协同。

这类模型需要区分：

- **模型异构性**：动作、速度、能耗或地形约束不同；
- **目标共享程度**：共享奖励不等于执行时必然协同；
- **信息结构**：IHATRPO 当前是全局状态下的集中训练和集中执行，不应误称为分散执行；
- **泛化边界**：无障碍仿真中的区域分工不能直接外推到真实地形。

## 证据

- [[src-yao2026-ihatrpo-heterogeneous-chargers]]：直接空地异构模型，raw 行 87–160、235–329、388–450。
- [[src-rahaman2023-obstacle-mcv]]：多 MCV 与障碍带来的异质可达性对照。
- [[src-gao2024-ra-dmcs-asymmetric-directional]]：路由非对称性和定向移动服务对照。

## 相关页面

- [[sys-mobile-uav-routing-scheduling]]
- [[obj-multi-objective-survivability]] · [[mtd-ihatrpo-heterogeneous-charging]]
- [[syn-adaptive-mobile-charger-coordination]]

