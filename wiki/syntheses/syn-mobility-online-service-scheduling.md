---
type: synthesis
title: 移动性、在线请求与服务目标下的无线充电调度
status: active
epistemic: high
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, time_slot, user_or_request, path_or_route]
constraints: [mobility, causality_online, qos, peak_power]
objectives: [max_throughput, min_cost, min_latency]
method_family: ""
problem_class: online_scheduling
covers:
  - "[[src-wu-charging-on-the-move]]"
  - "[[src-xu-cooperative-ccs]]"
  - "[[src-ma-tide-dynamic-power]]"
  - "[[src-chen-peak-aoi-wpt]]"
  - "[[src-qaisar2026-isac-uav-charging]]"
gaps:
  - "现有文献分别固定轨迹、价格或请求模型，尚无统一的移动请求与在线资源预算框架"
  - "成本、充电 utility 与信息新鲜度使用不同目标，当前没有共同评测协议"
  - "公平、截止时间和部分充电在本组核心文献中覆盖不足"
updated: 2026-08-12
---

# 移动性、在线请求与服务目标下的无线充电调度

这组文献都把“谁在何时获得能量”作为调度问题，但移动对象、未来信息和最终效用不同。使用时应先匹配系统设定，再选择算法骨架。

## 场景与决策对照

| 文献 | 谁移动/变化 | 未来信息 | 决策 | 目标 |
|------|-------------|----------|------|------|
| [[src-wu-charging-on-the-move]] | 接收设备沿已知轨迹移动 | 轨迹已知或可预测 | 静态充电器的离散功率档位 | 累计充电 utility |
| [[src-xu-cooperative-ccs]] | 设备移动到固定服务点 | 需求与价格作为离线输入 | 设备到充电器的分组/分配与成本分摊 | 充电费 + 往返移动成本 |
| [[src-ma-tide-dynamic-power]] | 请求集合随时隙变化 | 不依赖完整未来请求 | 定向充电器朝向 | 干涉下总体充电 utility |
| [[src-chen-peak-aoi-wpt]] | 数据状态与充传过程随时间推进 | 按论文采样/带宽设定调度 | 充电朝向/时序与数据传输 | 最大峰值 AoI |
| [[src-qaisar2026-isac-uav-charging]] | UAV 状态与请求队列在线变化 | 只使用当前估计和请求 | 优先级、轨迹与部分充电时间 | 效率、距离与时延 |

## 可迁移部分

- [[src-wu-charging-on-the-move]] 的轨迹分段与子模近似适合“轨迹可预测、功率可加、总预算固定”的问题；它不直接处理在线未知请求或波干涉。
- [[src-xu-cooperative-ccs]] 的合作分组与成本分摊适合存在服务价格、设备可主动移动且共享时段能够降成本的场景；其经济目标不能直接替换为 AoI。
- [[src-ma-tide-dynamic-power]] 的候选朝向和邻域更新适合固定节点、在线请求、定向充电与干涉并存的场景；若接收端持续移动，邻域更新条件需要重新建模。
- [[src-chen-peak-aoi-wpt]] 把能量补给与信息传输共同纳入调度，适合“能量只是完成数据服务的中间资源”的问题；其目标与纯充电量最大化不同。
- [[src-qaisar2026-isac-uav-charging]] 提供在线队列与部分充电，但没有覆盖未知移动接收端的逐请求 deadline 保证。

## 不能直接混用的假设

1. [[src-wu-charging-on-the-move]]、[[src-xu-cooperative-ccs]] 的 source 卡采用功率可加模型；[[src-ma-tide-dynamic-power]] 明确建模波干涉。
2. 已知轨迹的离线功率分配不等价于未知请求的在线调度。
3. 最小综合成本、最大充电 utility、最小峰值 AoI 是不同目标；当前库没有统一标尺将各论文百分比提升直接比较。
4. [[src-chen-peak-aoi-wpt]] 的“在线”含义来自时序与采样/传输过程，不应自动等同于任意请求到达模型。

## Gaps

1. 同时存在移动接收端、在线到达请求与发射总功率预算时，当前库没有统一调度框架。
2. 缺少把部分充电、公平/截止时间和信息新鲜度纳入同一服务质量模型的证据。
3. 缺少跨论文共同数据、轨迹和指标设置，难以判断方法差异来自算法还是实验设定。

## 相关页

- 总览：[[syn-wrsn-scheduling-placement]]
- 方法：[[mtd-tunable-power-mobile-traj]] · [[mtd-ccs-cooperative-service]] · [[mtd-tide-online-orientation]] · [[mtd-peak-aoi-joint-charge-tx]]
- 研究问题：[[prob-joint-deployment-online-interference]]
- 地图：[[map-online-scheduling]] · [[map-power-allocation]] · [[map-models-and-objectives]]
