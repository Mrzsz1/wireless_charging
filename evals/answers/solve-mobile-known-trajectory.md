# solve-mobile-known-trajectory

## 0. 库水位

当前为 **23 篇 source**，见 [[maps/library-status]]。

## 回答

可直接参考 [[src-wu-charging-on-the-move]] 与 [[mtd-tunable-power-mobile-traj]]。该方法假设**轨迹已知**、发射**功率可调**，将沿轨迹变化的接收功率做**分段常数**近似，在充电器离散功率档位和**总功率预算**下最大化饱和充电效用；其物理模型采用多充电器**功率可加**。相关移动模型对照见 [[syn-mobility-online-service-scheduling]]。

## 原文证据

- 系统模型与问题：[[src-wu-charging-on-the-move]]，原文第 60–135 行；轨迹离散与功率近似：原文第 136–306 行；近似算法：原文第 307–562 行。

## 边界

轨迹未知、速度强随机或存在相干波干涉时，原方法的离线输入和可加模型均不成立，需要在线预测或干涉感知外层。
