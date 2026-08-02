---
type: problem
title: 多发射端 WPT 的联合部署与在线干涉调度
status: active
epistemic: medium
scenario: [sensor_rf_energy]
entities: [transmitter, receiver, time_slot, user_or_request, power_pool]
constraints: [interference, causality_online, fairness, peak_power]
objectives: [max_throughput, max_fairness, multi_objective]
method_family: ""
problem_class: multi_tx_coordination
inspired_by:
  - "[[src-guo-concurrent-ccsp]]"
  - "[[src-ma-concurrent-gain]]"
  - "[[src-ma-tide-dynamic-power]]"
  - "[[syn-interference-aware-concurrent-wpt]]"
  - "[[syn-mobility-online-service-scheduling]]"
supports: []
user_confirmed: true
claimed_at: 2026-07-14
updated: 2026-07-14
---

# 多发射端 WPT 的联合部署与在线干涉调度

> 证据边界：本问题由当前 **9 篇 source** 的库内缺口凝练而来；“库内尚未完整覆盖”不等于全球首次提出，也不构成新颖性结论。

## 问题陈述

考虑多发射端同频无线功率传输（WPT）网络：部署能力需要长期固定，接收端请求在线到达，多发射端之间存在建设性或破坏性波干涉。系统需要联合决定：

1. 慢时间尺度的充电器位置、数量与可用朝向集合；
2. 快时间尺度每个时隙的并发开关集合和定向朝向；
3. 请求的服务次序与部分充电量；

目标是在总功率和峰值功率约束下，同时提高长期充电效用与请求间公平性。若硬件不支持独立开关、在线旋转或低成本切换，相应动作必须从模型中删除并重新定义问题。

## 证据链

| 已有路线 | 已覆盖 | 留给本问题的缺口 |
|----------|--------|------------------|
| [[src-guo-concurrent-ccsp]] | 固定位置下选择并发开关组合，降低干涉并缩短批量充满时间 | 在线请求、部分服务与长期公平未被统一建模 |
| [[src-ma-concurrent-gain]] | 联合选择充电器位置与传感器落点，利用建设性干涉 | 运行阶段的在线动作和请求服务次序固定 |
| [[src-ma-tide-dynamic-power]] | 固定位置下根据动态请求在线选择定向朝向 | 部署能力、并发开关与公平目标未联合优化 |

综合证据见 [[syn-interference-aware-concurrent-wpt]] 与 [[syn-mobility-online-service-scheduling]]。当前判断只是：各组成部分分别已有覆盖，但库内尚未出现同一设定下的完整联合问题。

## 需要先锁定的建模问题

1. 充电器能否独立开关、同时旋转；动作切换的时间和能量成本是否可忽略？
2. 接收端固定还是移动；系统能观测当前位置、未来轨迹或仅观测当前请求？
3. 公平采用 max-min、比例公平、等待时间还是截止时间满足率？
4. 部分充电是否改变后续请求到达、设备耗能和电池饱和模型？
5. 干涉模型是否需要相位级校准，还是允许使用可加功率近似作为对照？

## 建议评测轴

- 长期充电效用、最差用户效用与请求完成率；
- 平均/尾部等待时间、截止时间违约率；
- 总发射能量、峰值功率与动作切换次数；
- 在相同几何和硬件下比较：可加功率、非线性干涉、仅开关、仅朝向、联合动作；
- 离线已知请求上界、滚动时域基线和真正在线算法之间的差距。

## 非主张

- 本页没有给出已验证算法，也不声称某个双时间尺度方案可行。
- 本页不声称全球文献中没有联合模型；形成新颖性判断前仍需用户批准专项外搜。
- 当前不单独建立 idea 页；候选算法骨架仍保留在 [[../../logs/2026-07-14-ab-pilot-review-draft|A→B 审阅草案]]。

## 相关页

- 概念：[[cpt-wave-interference]] · [[cpt-concurrent-charging]] · [[cpt-directional-charging]]
- 方法：[[mtd-ccsp-greedy-scheduling]] · [[mtd-gain-placement-interference]] · [[mtd-tide-online-orientation]]
- 地图：[[map-domain-keywords]] · [[map-online-scheduling]] · [[map-multi-device-wpt]]
