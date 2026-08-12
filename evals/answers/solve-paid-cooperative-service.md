# solve-paid-cooperative-service

## 0. 库水位

当前为 **23 篇 source**，见 [[maps/library-status]]。

## 回答

使用 [[src-xu-cooperative-ccs]] 与 [[mtd-ccs-cooperative-service]]：先把设备分配到付费充电器，再以比例法或 Shapley value 做组内**成本分摊**，同时计入**充电成本**和**移动成本**。小规模可用 CCSA 近似优化；大规模可用联盟形成博弈 CCSGA，并收敛到纯 **Nash** 均衡。关系见 [[syn-mobility-online-service-scheduling]]。

## 原文证据

- 服务与成本模型：[[src-xu-cooperative-ccs]]，原文第 84–149 行；成本分摊：原文第 150–187 行；优化与博弈算法：原文第 188–474 行。

## 边界

付费服务、固定充电距离和论文的价格/移动成本模型是必要边界；均衡不等于全局社会成本最优，也不直接覆盖波干涉。
