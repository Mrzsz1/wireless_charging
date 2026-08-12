---
type: method
subtype: algorithm
title: ISAC辅助优先队列与部分充电UAV调度
status: active
epistemic: medium
scenario: [sensor_rf_energy, uav_wpt]
entities: [transmitter, receiver, battery, path_or_route, user_or_request]
constraints: [mobility, min_soc, causality_online]
objectives: [max_efficiency, min_latency, max_completion_rate]
method_family: online_algorithm
problem_class: routing_with_charging
updated: 2026-08-12
---

# ISAC辅助优先队列与部分充电UAV调度

## TL;DR

把来源论文的连续/组合决策压缩为可执行的输入—输出流程；使用前必须先核对物理模型、信息结构和目标口径。

## 何时使用 / 何时不使用

- 适合在线请求和移动状态估计共同变化的单UAV场景。
- 多UAV、欺骗/干扰、禁飞区和精细飞行动力学需要额外模块。

- 不满足来源论文的移动性、可加性、干涉、安全或在线信息假设时，不应只按算法名迁移。

## 输入 / 输出与变量

- **输入**：在线请求、节点能量/流量、UAV 状态与续航。
- **输出**：优先队列、轨迹、悬停时长与部分充电份额。
- 中间变量及符号沿用来源原文；实现时应保存随机种子、离散粒度、停止条件和不可行原因。

## 算法步骤

1. 由剩余能量、流量、旅行时间和飞行方向一致性计算请求优先级；
2. 用ISAC状态估计持续更新UAV位置、速度与旅行时间；
3. 状态变化触发队列重排和轨迹调整；
4. 按紧迫度分配有限悬停时间实施部分充电；
5. 在接纳请求时预留返航能量。

执行顺序应保持“候选空间构造 → 可行性/约束处理 → 优化或排序 → 结果核验”，不能跳过候选空间与物理约束校验。

## 复杂度与理论保证

在线启发式；无竞争比或多 UAV 收敛保证。未由原文给出的复杂度、近似比或收敛性质均视为**原文未报告**。

## 实验验证与基线

采用来源论文的实例、基线和指标验证；百分比只对该实验口径有效。若用于新数据，应重跑消融、随机重复和敏感性分析。

## 失效边界

- 物理模型、请求到达方式或目标函数改变时，原保证可能失效。
- 元启发式/启发式的单次最好结果不能替代统计重复；离线算法不能自动声称在线保证。

## 证据与来源

- [[../sources/src-qaisar2026-isac-uav-charging]]

- 关键算法位置：原文第 86–150 行。
