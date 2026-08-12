# novelty-joint-placement-online-interference

## 0. 库水位与范围

基于当前知识库的 **23 篇 source** 判断，见 [[maps/library-status]]。

## 判断

结论是**部分重叠**。[[src-ma-concurrent-gain]]覆盖利用波干涉的离线**部署阶段**，[[src-guo-concurrent-ccsp]]覆盖静态开关组合，[[src-ma-tide-dynamic-power]]覆盖在线请求下的朝向控制；[[syn-interference-aware-concurrent-wpt]]已并列三条路线。但“部署 + 在线开关与朝向 + **长期公平**效用”的统一跨时标模型尚未完整覆盖，因此不能判为已解决。

## 原文证据

- GAIN 部署模型与算法：原文第 69–365 行；TIDE 在线模型与算法：原文第 62–242 行；CCSP 开关调度：原文第 162–302 行。

## 边界

这是当前库内新颖性边界，不是全球文献结论；公平定义、在线反馈和跨阶段耦合方式仍需先形式化。
