# novelty-mobile-partial-deadline

## 0. 库水位与范围

基于当前知识库的 **23 篇 source** 判断，见 [[maps/library-status]]。

## 判断

当前**未完整覆盖**。[[src-wu-charging-on-the-move]]提供的是**已知轨迹**下的离线可调功率调度；[[src-qaisar2026-isac-uav-charging]]具有在线队列和**部分充电**，但服务端是单 UAV，未给出未知移动接收端每请求**截止时间**保证。相关边界见 [[syn-mobility-online-service-scheduling]] 与 [[syn-wrsn-scheduling-placement]]。

## 原文证据

- 已知轨迹模型：原文第 60–135 行；ISAC 在线队列与部分充电：原文第 86–150 行。

## 边界

“未知轨迹 + 在线 + 截止时间保证”仍是缺口；已有部分充电不等于 deadline-feasible，也不构成全球新颖性结论。
