---
type: synthesis
title: 自适应移动充电器协调方法对照
status: active
epistemic: high
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, battery, path_or_route, user_or_request]
constraints: [mobility, min_soc, causality_online]
objectives: [max_efficiency, min_energy, min_latency, max_completion_rate, multi_objective]
method_family: ""
problem_class: routing_with_charging
covers:
  - "[[../sources/src-yao2026-ihatrpo-heterogeneous-chargers]]"
  - "[[../sources/src-tian2025-diccs-clustering]]"
  - "[[../sources/src-liu2026-dchsa-adtsa-clustered]]"
  - "[[../sources/src-qaisar2026-isac-uav-charging]]"
  - "[[../sources/src-rahaman2023-obstacle-mcv]]"
gaps:
  - "异构多智能体、动态聚类、簇头轮换、ISAC在线估计和障碍绕行仍是彼此分离的模型。"
  - "五篇文献缺少共同实例、统一能耗模型和同一组时延/死亡率/旅行成本指标。"
  - "学习方法缺少确定性保证，启发式方法缺少动态分布外适应性对照。"
updated: 2026-08-01
---

# 自适应移动充电器协调方法对照

五篇文献都在静态路线之外引入状态自适应，但自适应对象不同：充电器能力、聚类结构、簇头角色、UAV状态估计或障碍几何。

## 对照

| 来源 | 自适应对象 | 调度骨架 | 主要目标 | 关键边界 |
|---|---|---|---|---|
| [[../sources/src-yao2026-ihatrpo-heterogeneous-chargers]] | AAV/SV异构能力与动态节点状态 | IHATRPO多智能体强化学习 | 效率、移动能耗、死亡率 | 无障碍、需训练 |
| [[../sources/src-tian2025-diccs-clustering]] | 分簇、簇头、停靠点和优先级 | 动态k-means与混合优先级 | 死亡率、等待时间、移动成本 | 单车 |
| [[../sources/src-liu2026-dchsa-adtsa-clustered]] | 簇头轮换和请求阈值 | DCHSA + ADTSA-DEC | 可靠性与能耗 | 多车协作未覆盖 |
| [[../sources/src-qaisar2026-isac-uav-charging]] | UAV位置/速度估计和在线请求 | ISAC闭环队列 + 部分充电 | 效率、轨迹、时延 | 单UAV、简化飞行模型 |
| [[../sources/src-rahaman2023-obstacle-mcv]] | 障碍绕行和多车分区 | 时空事件偏好 + 几何绕行 | 吞吐、能耗、响应时间 | 静态二维障碍 |

## 迁移关系

- DICCS与DCHSA都调整聚类层，但前者联合停靠点和访问优先级，后者强调簇头轮换对请求阈值的反馈。
- ISAC队列可以作为障碍路径或异构多车策略的状态输入，但原文未验证这种组合。
- IHATRPO能表达异构协作，却不提供障碍几何的硬可行性；障碍绕行算法可作为动作投影或路径后处理候选。
- 部分充电提高单次任务覆盖面，但与簇头轮换、多车任务冲突之间的耦合仍未统一。

## 相关方法

- [[../methods/mtd-ihatrpo-heterogeneous-charging]]
- [[../methods/mtd-diccs-dynamic-clustering]]
- [[../methods/mtd-dchsa-adtsa-dec]]
- [[../methods/mtd-isac-uav-priority-partial-charging]]
- [[../methods/mtd-obstacle-aware-multi-mcv]]
