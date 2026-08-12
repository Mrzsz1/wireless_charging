---
type: synthesis
title: 干涉感知的并发 WPT：从开关调度到部署与在线朝向
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, time_slot]
constraints: [interference, peak_power]
objectives: [min_latency, max_throughput]
method_family: ""
problem_class: multi_tx_coordination
covers:
  - "[[src-guo-concurrent-ccsp]]"
  - "[[src-ma-concurrent-gain]]"
  - "[[src-ma-tide-dynamic-power]]"
  - "[[src-dai-wanda-multi-antenna]]"
  - "[[src-wang-hipo-obstacles]]"
gaps:
  - "缺少在同一硬件与几何条件下对非线性干涉模型和可加功率模型的受控比较"
  - "现有三条干涉感知路线分别固定了部署、请求或动作维度，尚未覆盖联合部署与在线调度"
  - "公平、截止时间和安全辐射约束尚未与波干涉在线控制同时出现"
updated: 2026-08-12
---

# 干涉感知的并发 WPT：从开关调度到部署与在线朝向

本页只并列不同设定下的解法与边界，不判断哪一种模型“正确”。核心区别是：多发射端到同一接收端的功率是否允许直接相加。

## 三条干涉感知路线

| 路线 | 文献 | 固定项 | 决策变量 | 主要目标 |
|------|------|--------|----------|----------|
| 并发集合的时间调度 | [[src-guo-concurrent-ccsp]] | 充电器和传感器位置 | 每个充电周期的充电器开关组合 | 缩短全节点充满时间 |
| 利用增强区的联合部署 | [[src-ma-concurrent-gain]] | 充电阶段不再调整 | 充电器位置、传感器在局部区域内的落点 | 最大化充电 utility |
| 在线改变干涉图样 | [[src-ma-tide-dynamic-power]] | 充电器位置、传感器位置 | 每时隙定向充电器朝向 | 响应动态请求并最大化总体 utility |

三者都以 [[cpt-wave-interference|波干涉]] 解释“打开更多充电器未必增加接收功率”，但它们消除不利干涉的动作不同：CCSP 关掉部分发射端，GAIN 选择空间位置，TIDE 改变定向朝向。

## 与可加功率放置模型的边界

[[src-dai-wanda-multi-antenna]] 和 [[src-wang-hipo-obstacles]] 也处理多方向覆盖与充电器放置，但其 source 卡采用几何覆盖/功率可加视角。它们分别突出多天线相对朝向、异构充电器、扇环近区盲区和障碍阻挡；当前库内证据没有把这些因素与 CCSP/GAIN/TIDE 的相位干涉模型合并。

因此，方法迁移时至少要先检查：

1. 发射端是否同频相干，接收功率能否近似可加；
2. 系统可控制的是开关、位置还是朝向；
3. 请求是预先已知、批量充满，还是在线到达；
4. 传感器是否允许精确移动到干涉增强区。

## 证据强弱与不可直接比较项

- [[src-guo-concurrent-ccsp]]：仿真与实体 testbed；目标是完成时间。
- [[src-ma-concurrent-gain]]：仿真与现场实验；目标是部署后的 utility。
- [[src-ma-tide-dynamic-power]]：仿真与现场实验；目标是在线请求下的 utility。
- 文献报告的百分比提升基于各自基线、规模和请求模型，不能横向当作统一排行榜。

## Gaps

1. 在同一硬件、频率、几何与接收电路上，对“可加近似”与“相位干涉模型”进行受控比较。
2. 把部署阶段的空间选择与运行阶段的开关/朝向调度放入同一问题，同时保留可计算性。
3. 在波干涉在线控制中加入公平、截止时间、部分充电或安全辐射约束。

## 相关页

- 总览：[[syn-wrsn-scheduling-placement]]
- 方法：[[mtd-ccsp-greedy-scheduling]] · [[mtd-gain-placement-interference]] · [[mtd-tide-online-orientation]]
- 研究问题：[[prob-joint-deployment-online-interference]]
- 地图：[[map-power-allocation]] · [[map-multi-device-wpt]] · [[map-online-scheduling]]
