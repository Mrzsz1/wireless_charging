---
type: method
subtype: algorithm
title: 信号交叉口DWPT设施选址模型
status: active
epistemic: medium
scenario: [ev_dynamic_charging]
entities: [transmitter, receiver, battery, path_or_route]
constraints: [mobility, min_soc, deadline]
objectives: [min_cost, min_energy]
method_family: ilp_milp
problem_class: charger_placement
updated: 2026-08-01
---

# 信号交叉口DWPT设施选址模型

## 适用条件

- 车辆在信号交叉口存在排队、减速或停留时间。
- 设施选址与电池容量、连续行程需要联合评估。

## 要点

- 用混合整数规划表达路段/交叉口部署和车辆能量可行性。
- 将信号模式、排队停留和车辆聚合方式纳入部署决策。

## 来源

- [[src-honma2026-infinite-drive-dwpt]]
