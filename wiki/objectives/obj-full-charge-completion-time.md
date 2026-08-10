---
type: objective
title: 全充满完成时间最小化
status: active
epistemic: high
scenario: [sensor_rf_energy, multi_device_wpt]
entities: [transmitter, receiver, time_slot]
constraints: [interference, peak_power]
objectives: [min_latency, max_completion_rate]
method_family: ""
problem_class: offline_scheduling
updated: 2026-08-11
---

# 全充满完成时间最小化

## TL;DR

目标是让所有指定节点达到能量阈值所需的总时段最少。它强调最慢节点和可行覆盖，不等同于最大化总收能。

## 形式化定义

CCSP 选择活跃充电器集合 $S_k$ 及整数时段数 $\gamma_k$：

$$\min \sum_k\gamma_k,\qquad
\text{s.t. }\sum_k\gamma_k u_j^k\ge E,\ \forall j.$$

其中 $u_j^k$ 是组合 $S_k$ 在节点 $j$ 的非加性充电效用。若节点有不同需求，应把统一 $E$ 改为 $E_j^{\mathrm{need}}$；若允许部分完成，则目标应转为完成率或带惩罚的多目标，而不是继续称为全充满。

## 权衡

- 与 [[obj-aggregate-charging-utility]]：总效用高可能仍饿死弱节点。
- 与 [[obj-energy-and-mobility-cost]]：最短完成时间可能增加发射或移动能耗。
- 与 [[obj-multi-objective-survivability]]：全充满是批处理目标，动态网络更关心死亡率和持续服务。

## 证据

- [[src-guo-concurrent-ccsp]] raw §4.1，行 174–188。
- [[src-liu2021-joint-cuav-scheduling-trajectory]] 把停靠点数量、重复覆盖和轨迹距离作为完成全网充电的代理目标。

