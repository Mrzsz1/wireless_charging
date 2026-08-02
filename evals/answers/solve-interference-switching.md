# solve-interference-switching

## 0. 库水位

基于当前知识库：16 篇 source，年份 2017–2026，上次 ingest：2026-08-01；答案仅代表当前库水位。

## 1. 结论

优先参考 [[src-guo-concurrent-ccsp]] 与 [[mtd-ccsp-greedy-scheduling]]。CCSP在多个同频静态充电器同时工作时，将充电器选择写成**开关组合**调度，并显式处理干涉导致的**非线性叠加**；目标是缩短全网**完成时间**。其适用前提是静态节点/充电器、论文给定的传播与干涉模型，以及可执行的离散开关动作。

## 2. 可迁移边界

[[syn-interference-aware-concurrent-wpt]] 将CCSP与部署和在线朝向路线并列。若开关动作还要和部署一起优化，可参考GAIN，但需重新建模时间尺度；不能把几何覆盖结果直接当成干涉调度结果。

## 3. 库内未见

当前库未见同时提供统一实时请求、长期公平和开关切换代价的完整方案。

证据水位：[[maps/library-status]]
