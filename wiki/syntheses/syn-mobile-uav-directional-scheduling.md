---
type: synthesis
title: 移动、UAV与三维定向WPT调度对照
status: active
epistemic: high
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, path_or_route, time_slot]
constraints: [mobility, min_soc]
objectives: [min_energy, max_efficiency, max_completion_rate]
method_family: ""
problem_class: routing_with_charging
covers:
  - "[[src-wu-charging-on-the-move]]"
  - "[[src-gao2024-ra-dmcs-asymmetric-directional]]"
  - "[[src-liu2021-joint-cuav-scheduling-trajectory]]"
  - "[[src-binh2025-bilevel-metaheuristic-charging]]"
  - "[[src-gao2025-felkh-3d-uav]]"
  - "[[../sources/src-yao2026-ihatrpo-heterogeneous-chargers]]"
  - "[[../sources/src-qaisar2026-isac-uav-charging]]"
  - "[[../sources/src-rahaman2023-obstacle-mcv]]"
gaps:
  - "二维移动充电、UAV悬停点和三维方向集分别处理，缺少统一的空地协同基准。"
  - "路径、方向、传输时长和在线请求尚未在同一模型中同时出现。"
  - "元启发式与LKH结果缺少跨论文统一的能耗、服务完成率和运行时间协议。"
updated: 2026-08-01
---

# 移动、UAV与三维定向WPT调度对照

这些文献都把移动充电器的空间动作纳入调度，但移动主体、维度、方向模型和评价指标不同，不能仅按“轨迹优化”标签直接互换。

## 决策维度对照

| 文献 | 移动主体 | 空间 | 主要决策 | 方法骨架 |
|---|---|---|---|---|
| [[src-wu-charging-on-the-move]] | 设备/静态充电器相对运动 | 2D | 离散功率与轨迹段 | 子模/启发式 |
| [[src-gao2024-ra-dmcs-asymmetric-directional]] | 定向移动充电器 | 2D、有向路径 | 位置、方向、时长、闭环路线 | KCPG+LP+LKH |
| [[src-liu2021-joint-cuav-scheduling-trajectory]] | CUAV | 2D | 悬停点和访问轨迹 | PSOFKP+PSOD2P |
| [[src-binh2025-bilevel-metaheuristic-charging]] | 移动充电器 | WRSN区域 | 路径和充电时间 | 双层元启发式 |
| [[src-gao2025-felkh-3d-uav]] | UAV | 3D | 球面方向和充电tour | cMFEDS+LKH |
| [[../sources/src-yao2026-ihatrpo-heterogeneous-chargers]] | AAV + 地面SV | 2D/空地异构 | 连续移动与协同服务 | IHATRPO |
| [[../sources/src-qaisar2026-isac-uav-charging]] | UAV | 2D区域上空 | 在线队列、状态估计、部分充电 | ISAC闭环优先调度 |
| [[../sources/src-rahaman2023-obstacle-mcv]] | 多MCV | 2D有障碍 | 分区、排序、绕行 | 时空事件偏好+锚点/投影点 |

## 可迁移与不可直接混用

- RA-DMCS适合有向移动代价；其ATSP步骤不能直接替代对称二维路线。
- CUAV论文把调度和轨迹拆成两个PSO子问题；与RA-DMCS的方向集和LP时间分配不同。
- FELKH-3D解决三维方向离散化，但尚未处理在线请求、障碍物和飞行安全。
- 双层元启发式以死亡节点为目标，不能把其结果直接和最大充电utility或AoI百分比比较。

## Gaps

1. 缺少移动充电器、UAV、在线请求和干涉同时存在时的统一模型。
2. 缺少公开的2D/3D、对称/非对称、静态/动态请求统一基准。
3. 能耗、服务完成率、时延和算法运行时间尚未形成共同评测协议。

## 相关页

- 方法：[[mtd-ra-dmcs-asymmetric-mobile]] · [[mtd-uav-joint-scheduling-trajectory-pso]] · [[mtd-bilevel-metaheuristic-charging]] · [[mtd-felkh-3d-directional-uav]]
- 地图：[[map-multi-device-wpt]] · [[map-online-scheduling]] · [[map-models-and-objectives]]
