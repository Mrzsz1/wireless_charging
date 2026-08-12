---
type: method
subtype: algorithm
title: DWPT设施—电池—充电调度联合规划
status: active
epistemic: high
scenario: [ev_dynamic_charging, fleet_charging_ops]
entities: [transmitter, receiver, battery, grid_or_source]
constraints: [mobility, deadline, qos]
objectives: [min_cost, min_energy]
method_family: ilp_milp
problem_class: demand_response_tariff
updated: 2026-08-12
---

# DWPT设施—电池—充电调度联合规划

## TL;DR

把来源论文的连续/组合决策压缩为可执行的输入—输出流程；使用前必须先核对物理模型、信息结构和目标口径。

## 何时使用 / 何时不使用

适用条件由来源论文的系统模型决定。

- 不满足来源论文的移动性、可加性、干涉、安全或在线信息假设时，不应只按算法名迁移。

## 输入 / 输出与变量

- **输入**：公交线路/班次、电价、能耗、设施和电池成本。
- **输出**：设施、电池容量与班次充电计划。
- 中间变量及符号沿用来源原文；实现时应保存随机种子、离散粒度、停止条件和不可行原因。

## 算法步骤

- 战略层选择设施和电池容量，战术层安排充电时段。
- 可将MINLP线性化为MILP，用商业求解器求解。
- 目标同时包含设施、电池和TOU充电成本。

执行顺序应保持“候选空间构造 → 可行性/约束处理 → 优化或排序 → 结果核验”，不能跳过候选空间与物理约束校验。

## 复杂度与理论保证

线性化 MILP；求解质量受实例规模与求解器限制。未由原文给出的复杂度、近似比或收敛性质均视为**原文未报告**。

## 实验验证与基线

采用来源论文的实例、基线和指标验证；百分比只对该实验口径有效。若用于新数据，应重跑消融、随机重复和敏感性分析。

## 失效边界

- 物理模型、请求到达方式或目标函数改变时，原保证可能失效。
- 元启发式/启发式的单次最好结果不能替代统计重复；离线算法不能自动声称在线保证。

## 证据与来源

- [[src-li2024-dwc-beb-integrated-planning]]

- 关键算法位置：原文第 132–501 行。
