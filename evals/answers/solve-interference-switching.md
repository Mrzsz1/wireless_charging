# solve-interference-switching

## 0. 库水位

当前为 **23 篇 source**（21 篇论文/预印本、2 本核心专著），见 [[maps/library-status]]。

## 回答

直接采用 [[src-guo-concurrent-ccsp]] 与 [[mtd-ccsp-greedy-scheduling]]：把每个同频静态充电器的**开关组合**当作一个候选动作，按节点剩余能量计算非线性叠加下的边际收能，依次选择动作，最小化批量**全充满**的完成时间。SSCP 贪心有近似保证；弱节点可能拖尾时可用 balanced 版本。更大的方法关系见 [[syn-interference-aware-concurrent-wpt]]。

## 原文证据

- CCSP 模型、NP-hard 与调度目标：[[src-guo-concurrent-ccsp]]，原文第 162–212 行；算法与近似比：原文第 218–302 行。

## 边界

适用前提是同频、静态节点、组合效用可预计算且目标为全充满；未知在线请求、移动节点或正交信道不能直接套用。
