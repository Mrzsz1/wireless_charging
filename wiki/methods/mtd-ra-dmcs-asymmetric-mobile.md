---
type: method
subtype: algorithm
title: RA-DMCS 路由非对称定向移动充电
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, path_or_route, time_slot]
constraints: [mobility]
objectives: [min_energy, max_completion_rate]
method_family: heuristic
problem_class: routing_with_charging
updated: 2026-08-12
---

# RA-DMCS 路由非对称定向移动充电

## TL;DR

把来源论文的连续/组合决策压缩为可执行的输入—输出流程；使用前必须先核对物理模型、信息结构和目标口径。

## 何时使用 / 何时不使用

- 定向移动充电器、WRSN节点和有向移动代价。
- 需要同时决定位置、方向、传输时间和访问闭环。

- 不满足来源论文的移动性、可加性、干涉、安全或在线信息假设时，不应只按算法名迁移。

## 输入 / 输出与变量

- **输入**：节点需求、定向传输、有向移动代价。
- **输出**：位置、方向、时长和 ATSP 闭环。
- 中间变量及符号沿用来源原文；实现时应保存随机种子、离散粒度、停止条件和不可行原因。

## 算法步骤

- KCPG缩减连续位置空间；cMFRDS缩减方向空间。
- 线性规划分配传输时长；LKH处理非对称路径。

执行顺序应保持“候选空间构造 → 可行性/约束处理 → 优化或排序 → 结果核验”，不能跳过候选空间与物理约束校验。

## 复杂度与理论保证

多个子问题 NP-hard/NP-complete；整体为 KCPG+cMFRDS+LP+LKH。未由原文给出的复杂度、近似比或收敛性质均视为**原文未报告**。

## 实验验证与基线

采用来源论文的实例、基线和指标验证；百分比只对该实验口径有效。若用于新数据，应重跑消融、随机重复和敏感性分析。

## 失效边界

- 物理模型、请求到达方式或目标函数改变时，原保证可能失效。
- 元启发式/启发式的单次最好结果不能替代统计重复；离线算法不能自动声称在线保证。

## 证据与来源

- [[src-gao2024-ra-dmcs-asymmetric-directional]]

- 关键算法位置：原文第 202–387 行。
