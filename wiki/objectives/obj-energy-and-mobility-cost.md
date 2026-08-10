---
type: objective
title: 充电能效与移动成本权衡
status: active
epistemic: high
scenario: [sensor_rf_energy, uav_wpt, fleet_charging_ops]
entities: [transmitter, receiver, battery, path_or_route]
constraints: [mobility]
objectives: [max_efficiency, min_energy, min_cost]
method_family: ""
problem_class: routing_with_charging
updated: 2026-08-11
---

# 充电能效与移动成本权衡

## TL;DR

移动充电系统不能只最大化传给节点的能量；悬停、飞行、地面行驶和发射本身都消耗能量。目标需要明确分子分母或加权项，否则不同论文的“效率”不可直接比较。

## 常见表达

- 最大化交付能量 / 移动与发射总能耗；
- 最小化路径长度或移动能耗，同时满足所有充电需求；
- 以停靠点数、重复覆盖数和路径距离组成分层/多目标代理；
- 以总运营成本统一移动、充电和等待费用。

## 适用与误用

- **适用**：CUAV、MCV、空地协同、车队充电运营。
- **误用**：只报告路径变短，却不确认服务覆盖和节点能量约束仍满足；把二维距离直接当作 UAV 真实能耗；把加权 reward 数值当作物理效率。

## 证据

- [[src-liu2021-joint-cuav-scheduling-trajectory]]：系统能耗模型 raw 行 86–119，JSTOP 三个目标 raw 行 120–205。
- [[src-yao2026-ihatrpo-heterogeneous-chargers]]：AAV/SV 能耗与 reward，raw 行 137–160、261–329。
- [[src-xu-cooperative-ccs]]：合作服务的综合成本视角。

## 相关页面

- [[sys-mobile-uav-routing-scheduling]] · [[obj-multi-objective-survivability]]

