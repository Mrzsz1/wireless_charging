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
updated: 2026-08-01
---

# DWPT设施—电池—充电调度联合规划

## 要点

- 战略层选择设施和电池容量，战术层安排充电时段。
- 可将MINLP线性化为MILP，用商业求解器求解。
- 目标同时包含设施、电池和TOU充电成本。

## 来源

- [[src-li2024-dwc-beb-integrated-planning]]
