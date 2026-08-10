---
type: system-model
title: 在线请求下的定向充电控制模型
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, time_slot, user_or_request]
constraints: [interference, causality_online]
objectives: [max_throughput, max_completion_rate]
method_family: online_algorithm
problem_class: online_scheduling
updated: 2026-08-11
---

# 在线请求下的定向充电控制模型

## TL;DR

节点在运行中按剩余能量产生请求，控制器只能使用当前队列和能量状态，逐时隙选择定向充电器朝向。连续角度通常先压缩成有限候选覆盖集合，再处理相邻充电器之间的干涉耦合。

## 何时使用 / 不使用

- **使用**：请求动态到达、传感器位置固定、充电器可旋转且朝向切换快于决策时隙。
- **不使用**：未来请求完全已知时应对比离线调度；朝向切换耗时不可忽略时必须把切换成本加入状态与约束。

## 状态、动作与约束

- 状态：请求集合/队列、节点剩余能量、候选方向覆盖关系、邻接充电器集合；
- 动作：每个时隙为各充电器选择一个候选朝向；
- 约束：不使用未来请求，且相邻充电器的动作共同决定干涉后的功率；
- 输出：方向组合序列及对应请求服务效用。

TIDE 的 dominant sensor set 用于从连续旋转空间抽取不会丢失代表性功率分布的候选方向；neighbor set 再限制联合枚举范围。该模型不自带竞争比，实际性能依赖候选抽取、时隙和请求过程。

## 跨来源关系

- [[src-ma-tide-dynamic-power]] 是直接模型。
- [[src-guo-concurrent-ccsp]] 提供相同非加性干涉下的离线开关基线。
- [[src-qaisar2026-isac-uav-charging]] 展示动态状态估计、在线队列与部分充电，但动作主体为移动 UAV。

## 证据位置

- TIDE：§II-D 与 §III，raw 行 117–242；算法输入来自在线请求队列。
- ISAC-UAV：其 source 页链接的 raw 中“Problem Formulation / Algorithm”章节。

## 相关页面

- [[obj-aggregate-charging-utility]] · [[mtd-tide-online-orientation]]
- [[sys-interference-aware-concurrent-static]] · [[syn-mobility-online-service-scheduling]]

