---
type: map
title: 主题 · 多设备 WPT
status: active
scenario: [multi_device_wpt, sensor_rf_energy]
updated: 2026-07-14
---

# 主题地图：多设备 / 多充电器 WRSN

> 本批文献重心为传感器网 RF 可充电，对应 multi-charger / multi-device 并发与放置。

## 核心概念

- [[cpt-concurrent-charging]] · [[cpt-wave-interference]] · [[cpt-charging-utility]] · [[cpt-directional-charging]]

## 方法

- [[mtd-ccsp-greedy-scheduling]] · [[mtd-gain-placement-interference]] · [[mtd-ccs-cooperative-service]] · [[mtd-wanda-multi-antenna-placement]] · [[mtd-hipo-placement-obstacles]] · [[mtd-rose-robust-safe-power-scheduling]]

## 文献

- [[src-guo-concurrent-ccsp]] · [[src-ma-concurrent-gain]] · [[src-xu-cooperative-ccs]] · [[src-dai-wanda-multi-antenna]] · [[src-wang-hipo-obstacles]] · [[src-wu-charging-on-the-move]] · [[src-dai2022-rose-robust-safe-charging]] · [[src-liu2021-joint-cuav-scheduling-trajectory]] · [[src-gao2025-felkh-3d-uav]]

## 综合与 Gap

- [[syn-wrsn-scheduling-placement]] · [[syn-interference-aware-concurrent-wpt]] · [[syn-mobility-online-service-scheduling]] · [[syn-mobile-uav-directional-scheduling]]

## 相关主题

- [[map-power-allocation]]
- [[map-online-scheduling]]
- [[map-home]]

## 2026-08-01 自动增量：自适应移动充电

- [[../sources/src-yao2026-ihatrpo-heterogeneous-chargers|异构AAV/SV协同充电]] → [[../methods/mtd-ihatrpo-heterogeneous-charging|IHATRPO]]
- [[../sources/src-tian2025-diccs-clustering|DICCS动态非均匀聚类]] → [[../methods/mtd-diccs-dynamic-clustering|动态聚类与混合优先级]]
- [[../sources/src-liu2026-dchsa-adtsa-clustered|动态簇头与双阈值]] → [[../methods/mtd-dchsa-adtsa-dec|DCHSA+ADTSA-DEC]]
- [[../sources/src-qaisar2026-isac-uav-charging|ISAC按需UAV充电]] → [[../methods/mtd-isac-uav-priority-partial-charging|在线队列与部分充电]]
- [[../sources/src-rahaman2023-obstacle-mcv|有障碍多MCV调度]] → [[../methods/mtd-obstacle-aware-multi-mcv|时空事件协同绕行]]
- 对照：[[../syntheses/syn-adaptive-mobile-charger-coordination]]
