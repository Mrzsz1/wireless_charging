# solve-online-directional-requests

## 0. 库水位

基于当前知识库：16 篇 source，年份 2017–2026，上次 ingest：2026-08-01。

## 1. 直接可用

参考 [[src-ma-tide-dynamic-power]] 与 [[mtd-tide-online-orientation]]。TIDE面向固定传感器的**在线请求**，从连续方向中抽取**候选朝向**，动态维护 neighbor set，并在**波干涉**下选择每个时隙的朝向/功率分布。

## 2. 适用前提

传感器位置基本固定、请求按时隙到达、定向充电器可以旋转，且干涉模型和请求队列假设可接受。[[syn-interference-aware-concurrent-wpt]] 将TIDE与CCSP、GAIN的决策阶段差异列出。

## 3. 库内未见

当前库未见同时覆盖移动接收端、未知未来请求和长期公平保证的完整算法。

证据水位：[[maps/library-status]]
