---
type: map
title: 主题 · 系统模型与目标
status: active
updated: 2026-08-11
---

# 系统模型与目标：从问题到证据

> 使用顺序：先选择最接近的系统模型，再确认优化目标和不可违反的约束，最后进入方法与 source。不要只因算法名称相似就迁移。

## 1. 选择系统模型

| 你的问题 | 系统模型 | 关键边界 |
|---|---|---|
| 多个静态同频充电器，功率不可加 | [[sys-interference-aware-concurrent-static]] | 组合开关、部署或朝向都受波干涉影响 |
| 固定节点动态发起请求，充电器可旋转 | [[sys-online-directional-request]] | 只用当前队列，方向组合有邻域耦合 |
| 移动充电车/UAV 同时决定停靠和路径 | [[sys-mobile-uav-routing-scheduling]] | 覆盖、停留、访问顺序和移动能耗耦合 |
| AAV、SV 或多种 MCV 能力不同 | [[sys-heterogeneous-mobile-charger-coordination]] | 动作/能耗/可达域与信息结构必须分别定义 |

## 2. 选择优化目标

| 目标 | 页面 | 易混点 |
|---|---|---|
| 所有节点尽快达到能量阈值 | [[obj-full-charge-completion-time]] | 不等于总效用最大或能耗最小 |
| 当前部署/请求下总体收能收益最高 | [[obj-aggregate-charging-utility]] | 必须说明饱和、阈值与公平性 |
| 兼顾服务效果与移动/发射成本 | [[obj-energy-and-mobility-cost]] | 路径短不代表需求全部满足 |
| 同时优化效率、距离和节点生存 | [[obj-multi-objective-survivability]] | reward 提升不等于各物理目标同比改善 |

## 3. 方法与原文证据

| 模型 → 目标 | 可用方法 | 主要 source |
|---|---|---|
| 静态干涉 → 完成时间 | [[mtd-ccsp-greedy-scheduling]] | [[src-guo-concurrent-ccsp]] |
| 静态干涉 → 总体效用 | [[mtd-gain-placement-interference]] | [[src-ma-concurrent-gain]] |
| 在线定向 → 总体效用 | [[mtd-tide-online-orientation]] | [[src-ma-tide-dynamic-power]] |
| UAV 路径—调度 → 能耗 | [[mtd-uav-joint-scheduling-trajectory-pso]] | [[src-liu2021-joint-cuav-scheduling-trajectory]] |
| 异构协同 → 多目标生存性 | [[mtd-ihatrpo-heterogeneous-charging]] | [[src-yao2026-ihatrpo-heterogeneous-chargers]] |

## 4. 实验与复核

- [[data-wrsn-simulation-evidence-protocol]]：跨论文比较前先核对场景、功率模型、随机性、baseline 与证据位置。
- [[syn-interference-aware-concurrent-wpt]]：CCSP / GAIN / TIDE 的决策阶段差异。
- [[syn-adaptive-mobile-charger-coordination]]：异构、聚类、部分充电和障碍路线。
- [[syn-core-books-atlas]]：近似算法、博弈与机制的理论入口。

## 相关地图

- [[map-home]] · [[map-online-scheduling]] · [[map-power-allocation]] · [[map-multi-device-wpt]]
