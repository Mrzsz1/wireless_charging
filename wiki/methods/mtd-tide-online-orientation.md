---
type: method
subtype: algorithm
title: TIDE 在线朝向/功率分布控制
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, time_slot]
constraints: [interference, causality_online]
objectives: [max_throughput]
method_family: online_algorithm
problem_class: online_scheduling
updated: 2026-08-11
---

# TIDE 在线定向控制

## TL;DR

TIDE 用 dominant sensor sets 把连续朝向压成有限候选，再用 neighbor sets 局部联合选择方向，使在线请求节点在干涉下获得较高总效用。

## 何时使用 / 适用条件

- 定向可旋转充电器、固定传感器、在线请求、组合功率不可加。
- 不适合需要同时优化充电器部署或移动路径的场景。

## 输入 / 输出与变量

- 输入：请求集合、节点状态、每个充电器的候选 sensor set/朝向、邻接耦合关系。
- 输出：每次队列更新后的充电器朝向组合。
- 决策单元：neighbor set；效用只针对当前请求节点计算。

## 算法步骤

1. 连续扫描方向并记录 dominant sensor sets。
2. 加入其子集，得到候选 sensor sets 与代表朝向。
3. 按覆盖/干涉关系构造 neighbor sets。
4. 在局部组内评估方向组合；组过大时继续拆分。
5. 选择总体请求 utility 最大的局部组合并执行，队列变化后重复。

## 复杂度与理论保证

- 原始朝向组合问题 NP-hard。
- 组合复杂度对 neighbor set 内候选方向数呈乘积/指数增长，候选抽取和分组用于降低它。
- 原文未报告竞争比或近似比，因此不能称为最坏情形有保证的在线算法。

## 失效边界

- 分组切断跨组干涉时可能损失全局最优性。
- 机械切换延迟、能耗和方向误差未纳入会高估收益。
- 请求到达分布改变时必须重新评测。

## 证据与来源

- [[src-ma-tide-dynamic-power]]
- Raw §III，行 129–242；候选抽取 Algorithm 1 从行 151 开始；NP-hard 见行 181–195；实验行 243–300。
- 模型：[[sys-online-directional-request]]；目标：[[obj-aggregate-charging-utility]]。
