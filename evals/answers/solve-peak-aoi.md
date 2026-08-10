# solve-peak-aoi

## 0. 库水位

基于当前知识库：23 篇 source，年份 2017–2026，上次 ingest：2026-08-01。

## 1. 直接可用

参考 [[src-chen-peak-aoi-wpt]] 与 [[mtd-peak-aoi-joint-charge-tx]]。该路线做**充传联合**调度，在能量补给与数据传输之间分配时序和资源，目标是降低**最大峰值 AoI**，同时考虑**充电时延**和**带宽约束**。

## 2. 适用前提

需要无线供能边缘网络、数据流/采样过程和带宽模型；不能把单纯最大充电量的结果直接替代峰值AoI优化。

## 3. 关系

[[cpt-aoi-peak]] 和 [[syn-mobility-online-service-scheduling]] 可用于比较信息新鲜度目标与吞吐、成本目标的差异。

证据水位：[[maps/library-status]]
