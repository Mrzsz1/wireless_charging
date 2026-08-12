# relation-gain-tide

## 0. 库水位

当前为 **23 篇 source**，见 [[maps/library-status]]。

## 对照

[[src-ma-concurrent-gain]]作用于离线**部署阶段**：主要可控变量是传感器位置/PoI 分配与充电器部署，用几何结构寻找建设性干涉区域。[[src-ma-tide-dynamic-power]]作用于**在线运行**：节点固定、请求到达后控制**充电器朝向**。两者都利用波干涉但决策时标不同，详见 [[syn-interference-aware-concurrent-wpt]]。

## 原文证据

- GAIN 模型、部署算法：原文第 69–365 行；TIDE 在线模型、朝向算法：原文第 62–242 行。

## 边界

不能把两篇各自的提升百分比直接横比；拓扑、请求分布、utility 和控制变量均不同。
