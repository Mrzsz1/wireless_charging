# solve-mobile-known-trajectory

## 0. 库水位

基于当前知识库：16 篇 source，年份 2017–2026，上次 ingest：2026-08-01。

## 1. 直接可用

[[src-wu-charging-on-the-move]] / [[mtd-tunable-power-mobile-traj]] 最匹配：接收设备沿**已知轨迹**移动，静态充电器按轨迹分段选择**分段常数**功率档位。模型把各充电器贡献视为**功率可加**，并在**总功率预算**下最大化累计充电utility。

## 2. 适用前提

轨迹已知或可预测、时间离散、功率档位可执行且接收端能量模型与论文相近。若轨迹未知或有强波干涉，应转看在线朝向或干涉感知路线。

## 3. 对照

[[syn-mobility-online-service-scheduling]] 说明该方法不直接处理在线未知请求、服务收费或AoI。

证据水位：[[maps/library-status]]
