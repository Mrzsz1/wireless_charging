---
type: method
subtype: algorithm
title: DCHSA与ADTSA-DEC动态簇头双阈值调度
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, battery, path_or_route]
constraints: [mobility, min_soc]
objectives: [max_completion_rate, min_energy]
method_family: heuristic
problem_class: routing_with_charging
updated: 2026-08-01
---

# DCHSA与ADTSA-DEC动态簇头双阈值调度

## 算法骨架

1. 以簇内剩余能量方差触发候选簇头集更新；
2. 结合剩余能量、聚合、维护和轮换能耗选择新簇头；
3. 根据簇头轮换引起的实时能耗变化调整充电双阈值；
4. 将新请求状态交给移动充电调度策略。

## 适用边界

- 适合聚类网络中“路由角色变化影响充电需求”的闭环调度。
- 多充电器协作和低时延密集网络尚未覆盖。

## 来源

- [[../sources/src-liu2026-dchsa-adtsa-clustered]]
