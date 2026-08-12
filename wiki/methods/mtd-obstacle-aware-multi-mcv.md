---
type: method
subtype: algorithm
title: 有障碍多MCV时空事件协同调度
status: active
epistemic: medium
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, path_or_route, user_or_request]
constraints: [mobility, deadline]
objectives: [max_completion_rate, min_energy, min_latency]
method_family: heuristic
problem_class: routing_with_charging
updated: 2026-08-12
---

# 有障碍多MCV时空事件协同调度

## TL;DR

把来源论文的连续/组合决策压缩为可执行的输入—输出流程；使用前必须先核对物理模型、信息结构和目标口径。

## 何时使用 / 何时不使用

- 适合已知二维静态障碍和多车分区服务。
- 动态障碍、跨区任务迁移、三维路径及实时deadline需要扩展。

- 不满足来源论文的移动性、可加性、干涉、安全或在线信息假设时，不应只按算法名迁移。

## 输入 / 输出与变量

- **输入**：二维障碍、节点请求、MCV 数量与能耗。
- **输出**：分区、请求顺序和无碰撞绕行路径。
- 中间变量及符号沿用来源原文；实现时应保存随机种子、离散粒度、停止条件和不可行原因。

## 算法步骤

1. 聚类分区并为每个子区分配移动充电车；
2. 以时间、空间和事件偏好形成联合充电优先级；
3. 构造分区内充电顺序；
4. 遇到障碍时利用锚点和投影点生成绕行路径。

执行顺序应保持“候选空间构造 → 可行性/约束处理 → 优化或排序 → 结果核验”，不能跳过候选空间与物理约束校验。

## 复杂度与理论保证

三阶段启发式；统计检验不构成全局最优保证。未由原文给出的复杂度、近似比或收敛性质均视为**原文未报告**。

## 实验验证与基线

采用来源论文的实例、基线和指标验证；百分比只对该实验口径有效。若用于新数据，应重跑消融、随机重复和敏感性分析。

## 失效边界

- 物理模型、请求到达方式或目标函数改变时，原保证可能失效。
- 元启发式/启发式的单次最好结果不能替代统计重复；离线算法不能自动声称在线保证。

## 证据与来源

- [[../sources/src-rahaman2023-obstacle-mcv]]

- 关键算法位置：原文第 193–348 行。
