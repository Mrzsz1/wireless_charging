# 科研 RAG 检索评测

- 状态：PASS
- 用例：13/13
- Source resolution accuracy：1.000
- Channel attempt rate：1.000
- Document Recall@5/10/20：0.808 / 0.885 / 1.000
- Heading Recall@20：1.000
- MRR / nDCG@10：0.811 / 0.798
- Locator validity：1.000
- Zero-evidence FN/FP：0 / 0
- 平均检索耗时：59.9 ms
- 平均轮数：1.00

## 用例

### book-mobile-path-source · 在《近似算法》这本书里面有没有关于移动路径规划的算法？

- 状态：PASS
- 通道：book, mixed, source
- Stop：all_requested_surfaces_attempted
- Recall@5/20：1.000 / 1.000
- Locator：1.000
- v2 改善：book:approximation-algorithms

### open-paper-book-mobile-path · 有没有文献或者哪本书里面涉及到移动路径规划的相关内容？

- 状态：PASS
- 通道：book, mixed, paper, source
- Stop：all_requested_surfaces_attempted
- Recall@5/20：1.000 / 1.000
- Locator：1.000
- v2 改善：book:approximation-algorithms, paper:sources/src-liu2021-joint-cuav-scheduling-trajectory

### bilingual-uav-trajectory · Which paper jointly optimizes charging scheduling and UAV trajectory with particle swarm optimization?

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Recall@5/20：1.000 / 1.000
- Locator：1.000

### rose-source-multipath · 《ROSE: Robustly Safe Charging for Wireless Power Transfer》怎样建模多径叠加与建设性、破坏性干涉？

- 状态：PASS
- 通道：mixed, paper, source
- Stop：all_requested_surfaces_attempted
- Recall@5/20：1.000 / 1.000
- Locator：1.000

### wave-interference-literature · 哪些论文研究了波干扰下的并发充电或定向功率控制？

- 状态：PASS
- 通道：mixed, paper, source
- Stop：all_requested_surfaces_attempted
- Recall@5/20：0.500 / 1.000
- Locator：1.000
- v2 改善：paper:sources/src-ma-concurrent-gain, paper:sources/src-ma-tide-dynamic-power

### obstacle-anchor-projection · 哪项研究使用时空事件偏好以及锚点或投影点，让多辆移动充电车绕开障碍？

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Recall@5/20：0.000 / 1.000
- Locator：1.000

### heterogeneous-ihatrpo · Which work applies IHATRPO to coordinate heterogeneous aerial and ground charging vehicles?

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Recall@5/20：1.000 / 1.000
- Locator：1.000

### isac-partial-charging · 哪篇工作把 ISAC 状态估计、在线请求队列和部分充电放进同一个 UAV 调度闭环？

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Recall@5/20：1.000 / 1.000
- Locator：1.000

### dwpt-beb-planning · 动态无线充电基础设施选址与电动公交电池容量联合规划有哪些研究？

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Recall@5/20：0.000 / 1.000
- Locator：1.000
- v2 改善：paper:sources/src-li2024-dwc-beb-integrated-planning

### peak-aoi-directional · 定向充电与数据传输联合优化时，哪篇论文最小化最大峰值信息年龄？

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Recall@5/20：1.000 / 1.000
- Locator：1.000

### wanda-multiple-antennas · 哪项工作研究了带多天线的无线充电器放置，并区分相对朝向？

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Recall@5/20：1.000 / 1.000
- Locator：1.000

### followup-rose-reference · 它怎样处理多径传播造成的干涉？

- 状态：PASS
- 通道：mixed, paper, source
- Stop：all_requested_surfaces_attempted
- Recall@5/20：1.000 / 1.000
- Locator：1.000

### true-zero-evidence · 当前库中是否存在《QTC-9 量子拓扑结晶协议》这篇潮汐引力无线充电调度论文？

- 状态：PASS
- 通道：source
- Stop：unresolved_explicit_source
- Recall@5/20：1.000 / 1.000
- Locator：1.000

