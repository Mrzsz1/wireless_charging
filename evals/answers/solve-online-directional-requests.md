# solve-online-directional-requests

## 0. 库水位

当前为 **23 篇 source**，见 [[maps/library-status]]。

## 回答

参考 [[src-ma-tide-dynamic-power]] 与 [[mtd-tide-online-orientation]]。TIDE 面向固定传感器的**在线请求**，为每个定向充电器生成**候选朝向**，利用 neighbor set 表示朝向间的影响关系，再按当前请求和**波干涉**效用在线选择朝向。它是运行期控制，不是一次性部署算法；横向关系见 [[syn-interference-aware-concurrent-wpt]]。

## 原文证据

- 模型与在线请求：[[src-ma-tide-dynamic-power]]，原文第 62–128 行；方向选择算法：原文第 129–242 行。

## 边界

该方案依赖固定节点、已知候选朝向及可评估的干涉效用；移动节点、连续无界方向或无法观测的信道需另加估计与离散化。
