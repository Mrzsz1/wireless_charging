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
updated: 2026-08-01
---

# 有障碍多MCV时空事件协同调度

## 算法骨架

1. 聚类分区并为每个子区分配移动充电车；
2. 以时间、空间和事件偏好形成联合充电优先级；
3. 构造分区内充电顺序；
4. 遇到障碍时利用锚点和投影点生成绕行路径。

## 适用边界

- 适合已知二维静态障碍和多车分区服务。
- 动态障碍、跨区任务迁移、三维路径及实时deadline需要扩展。

## 来源

- [[../sources/src-rahaman2023-obstacle-mcv]]
