# relation-additive-vs-interference

## 0. 库水位

基于当前知识库：23 篇 source，年份 2017–2026，上次 ingest：2026-08-01。

## 1. 模型差异

[[src-wang-hipo-obstacles]] 与 [[src-dai-wanda-multi-antenna]] 重点是几何覆盖、异构设备或多天线放置，通常采用可加功率/覆盖近似；[[src-guo-concurrent-ccsp]] 与 [[src-ma-tide-dynamic-power]] 则显式处理**波干涉**、相位或叠加效应。

## 2. 为什么不能直接替代

- HIPO/WANDA优化的是几何覆盖或部署结构，变量和目标不包含CCSP/TIDE的运行时开关、在线朝向。
- 在功率可加模型中增加覆盖通常带来可预测收益；干涉模型中增加充电器可能发生抵消，结论不同。
- 因此两类结果的**模型边界**、传播假设和动作时间尺度不同，不能把放置结论直接替代干涉感知调度。

## 3. 综合证据

[[syn-interference-aware-concurrent-wpt]] 给出开关、部署和在线朝向三条路线的并列关系。

证据水位：[[maps/library-status]]
