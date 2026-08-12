# relation-additive-vs-interference

## 0. 库水位

当前为 **23 篇 source**，见 [[maps/library-status]]。

## 对照

[[src-wang-hipo-obstacles]]与[[src-dai-wanda-multi-antenna]]解决的是位置、朝向、障碍和天线配置的**几何覆盖**，组合收益建立在多源**功率可加**或子模近似上。[[src-guo-concurrent-ccsp]]与[[src-ma-tide-dynamic-power]]显式处理相位相关的**波干涉**，同一组发射器可能建设或破坏性叠加，因此需开关/朝向调度；综述见 [[syn-interference-aware-concurrent-wpt]]。

## 原文证据

- HIPO 模型：原文第 63–109 行；WANDA 模型：原文第 68–129 行；CCSP 干涉调度：原文第 77–302 行；TIDE：原文第 62–242 行。

## 边界

模型边界不同：几何覆盖近似比不能移植到波相位效用，干涉调度也不能替代障碍与连续位置部署。
