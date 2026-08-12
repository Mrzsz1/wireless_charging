---
type: method
subtype: algorithm
title: FELKH-3D 三维定向UAV充电
status: active
epistemic: medium
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, path_or_route]
constraints: [mobility]
objectives: [max_efficiency, min_energy]
method_family: heuristic
problem_class: routing_with_charging
updated: 2026-08-12
---

# FELKH-3D 三维定向UAV充电

## TL;DR

把来源论文的连续/组合决策压缩为可执行的输入—输出流程；使用前必须先核对物理模型、信息结构和目标口径。

## 何时使用 / 何时不使用

适用条件由来源论文的系统模型决定。

- 不满足来源论文的移动性、可加性、干涉、安全或在线信息假设时，不应只按算法名迁移。

## 输入 / 输出与变量

- **输入**：三维节点、定向 WPT、UAV 能耗与需求。
- **输出**：代表方向集、时长与 UAV tour。
- 中间变量及符号沿用来源原文；实现时应保存随机种子、离散粒度、停止条件和不可行原因。

## 算法步骤

- cMFEDS从三维球面方向中生成最小功能等价方向集。
- LKH规划UAV充电tour，避免直接搜索无限方向空间。
- 主要证据来自三维WRSN仿真，需区分方向集最优性和整体启发式路径质量。

执行顺序应保持“候选空间构造 → 可行性/约束处理 → 优化或排序 → 结果核验”，不能跳过候选空间与物理约束校验。

## 复杂度与理论保证

cMFEDS 方向集具结构最小性；LKH tour 为启发式。未由原文给出的复杂度、近似比或收敛性质均视为**原文未报告**。

## 实验验证与基线

采用来源论文的实例、基线和指标验证；百分比只对该实验口径有效。若用于新数据，应重跑消融、随机重复和敏感性分析。

## 失效边界

- 物理模型、请求到达方式或目标函数改变时，原保证可能失效。
- 元启发式/启发式的单次最好结果不能替代统计重复；离线算法不能自动声称在线保证。

## 证据与来源

- [[src-gao2025-felkh-3d-uav]]

- 关键算法位置：原文第 194–523 行。
