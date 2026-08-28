# 科研 RAG 检索评测

- Schema：qa-rag-evaluation-report-v4
- Case dataset SHA-256：4a0c2d2af08a5fd0faa24749261735dd3a9e8a2b778bf8b60bc77fc04ac405cc
- 状态：PASS
- 用例：13/13
- Ranking eligible / zero-evidence：12 / 1
- Source resolution accuracy：1.000
- Channel attempt rate：1.000
- Work Recall@5/10/20：1.000 / 1.000 / 1.000
- Work MRR / nDCG@10：0.958 / 0.969
- Exact-source Recall@5/10/20：1.000 / 1.000 / 1.000
- Exact-source MRR / nDCG@10：0.854 / 0.896
- Heading Recall@20（eligible=3）：1.000
- Passage MRR（diagnostic-only）：0.958
- Locator validity：1.000
- Zero-evidence TP/FP/FN/TN：1 / 0 / 0 / 12
- Zero-evidence precision/recall/specificity：1.000 / 1.000 / 1.000
- 平均检索耗时：61.1 ms
- 平均轮数：1.00
- Reranker fallback：12 / 13 (0.923)
- 平均 reranker 耗时：6.9 ms
- Reranker load/prepare/inference/input tokens：0.0 / 0.0 / 0.0 ms / 0.0

## 用例

### book-mobile-path-source · 在《近似算法》这本书里面有没有关于移动路径规划的算法？

- 状态：PASS
- 通道：book, mixed, source
- Stop：all_requested_surfaces_attempted
- Work Recall@5/20：1.000 / 1.000
- Exact-source Recall@5/20：1.000 / 1.000
- Work/Exact-source/Passage MRR：1.000 / 1.000 / 1.000
- Locator：1.000
- Reranker：deterministic-research-v2 / degraded / 6 ms / fallback=cross_encoder_unavailable+reranker_unavailable
- v2 改善：book:approximation-algorithms

### open-paper-book-mobile-path · 有没有文献或者哪本书里面涉及到移动路径规划的相关内容？

- 状态：PASS
- 通道：book, mixed, paper, source
- Stop：all_requested_surfaces_attempted
- Work Recall@5/20：1.000 / 1.000
- Exact-source Recall@5/20：1.000 / 1.000
- Work/Exact-source/Passage MRR：1.000 / 1.000 / 1.000
- Locator：1.000
- Reranker：deterministic-research-v2 / degraded / 11 ms / fallback=cross_encoder_unavailable+reranker_unavailable
- v2 改善：book:approximation-algorithms, paper:sources/src-liu2021-joint-cuav-scheduling-trajectory

### bilingual-uav-trajectory · Which paper jointly optimizes charging scheduling and UAV trajectory with particle swarm optimization?

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Work Recall@5/20：1.000 / 1.000
- Exact-source Recall@5/20：1.000 / 1.000
- Work/Exact-source/Passage MRR：1.000 / 1.000 / 1.000
- Locator：1.000
- Reranker：deterministic-research-v2 / degraded / 9 ms / fallback=cross_encoder_unavailable+reranker_unavailable

### rose-source-multipath · 《ROSE: Robustly Safe Charging for Wireless Power Transfer》怎样建模多径叠加与建设性、破坏性干涉？

- 状态：PASS
- 通道：mixed, paper, source
- Stop：all_requested_surfaces_attempted
- Work Recall@5/20：1.000 / 1.000
- Exact-source Recall@5/20：1.000 / 1.000
- Work/Exact-source/Passage MRR：1.000 / 1.000 / 1.000
- Locator：1.000
- Reranker：deterministic-research-v2 / degraded / 4 ms / fallback=cross_encoder_unavailable+reranker_unavailable

### wave-interference-literature · 哪些论文研究了波干扰下的并发充电或定向功率控制？

- 状态：PASS
- 通道：mixed, paper, source
- Stop：all_requested_surfaces_attempted
- Work Recall@5/20：1.000 / 1.000
- Exact-source Recall@5/20：1.000 / 1.000
- Work/Exact-source/Passage MRR：1.000 / 1.000 / 1.000
- Locator：1.000
- Reranker：deterministic-research-v2 / degraded / 13 ms / fallback=cross_encoder_unavailable+reranker_unavailable
- v2 改善：paper:sources/src-ma-concurrent-gain, paper:sources/src-ma-tide-dynamic-power

### obstacle-anchor-projection · 哪项研究使用时空事件偏好以及锚点或投影点，让多辆移动充电车绕开障碍？

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Work Recall@5/20：1.000 / 1.000
- Exact-source Recall@5/20：1.000 / 1.000
- Work/Exact-source/Passage MRR：0.500 / 0.250 / 0.500
- Locator：1.000
- Reranker：deterministic-research-v2 / degraded / 6 ms / fallback=cross_encoder_unavailable+reranker_unavailable

### heterogeneous-ihatrpo · Which work applies IHATRPO to coordinate heterogeneous aerial and ground charging vehicles?

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Work Recall@5/20：1.000 / 1.000
- Exact-source Recall@5/20：1.000 / 1.000
- Work/Exact-source/Passage MRR：1.000 / 1.000 / 1.000
- Locator：1.000
- Reranker：deterministic-research-v2 / degraded / 3 ms / fallback=cross_encoder_unavailable+reranker_unavailable

### isac-partial-charging · 哪篇工作把 ISAC 状态估计、在线请求队列和部分充电放进同一个 UAV 调度闭环？

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Work Recall@5/20：1.000 / 1.000
- Exact-source Recall@5/20：1.000 / 1.000
- Work/Exact-source/Passage MRR：1.000 / 0.500 / 1.000
- Locator：1.000
- Reranker：deterministic-research-v2 / degraded / 4 ms / fallback=cross_encoder_unavailable+reranker_unavailable
- v2 退化：paper:sources/src-qaisar2026-isac-uav-charging

### dwpt-beb-planning · 动态无线充电基础设施选址与电动公交电池容量联合规划有哪些研究？

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Work Recall@5/20：1.000 / 1.000
- Exact-source Recall@5/20：1.000 / 1.000
- Work/Exact-source/Passage MRR：1.000 / 0.500 / 1.000
- Locator：1.000
- Reranker：deterministic-research-v2 / degraded / 4 ms / fallback=cross_encoder_unavailable+reranker_unavailable
- v2 改善：paper:sources/src-li2024-dwc-beb-integrated-planning

### peak-aoi-directional · 定向充电与数据传输联合优化时，哪篇论文最小化最大峰值信息年龄？

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Work Recall@5/20：1.000 / 1.000
- Exact-source Recall@5/20：1.000 / 1.000
- Work/Exact-source/Passage MRR：1.000 / 1.000 / 1.000
- Locator：1.000
- Reranker：deterministic-research-v2 / degraded / 13 ms / fallback=cross_encoder_unavailable+reranker_unavailable

### wanda-multiple-antennas · 哪项工作研究了带多天线的无线充电器放置，并区分相对朝向？

- 状态：PASS
- 通道：mixed, paper, source, wiki
- Stop：all_requested_surfaces_attempted
- Work Recall@5/20：1.000 / 1.000
- Exact-source Recall@5/20：1.000 / 1.000
- Work/Exact-source/Passage MRR：1.000 / 1.000 / 1.000
- Locator：1.000
- Reranker：deterministic-research-v2 / degraded / 13 ms / fallback=cross_encoder_unavailable+reranker_unavailable

### followup-rose-reference · 它怎样处理多径传播造成的干涉？

- 状态：PASS
- 通道：mixed, paper, source
- Stop：all_requested_surfaces_attempted
- Work Recall@5/20：1.000 / 1.000
- Exact-source Recall@5/20：1.000 / 1.000
- Work/Exact-source/Passage MRR：1.000 / 1.000 / 1.000
- Locator：1.000
- Reranker：deterministic-research-v2 / degraded / 4 ms / fallback=cross_encoder_unavailable+reranker_unavailable

### true-zero-evidence · 当前库中是否存在《QTC-9 量子拓扑结晶协议》这篇潮汐引力无线充电调度论文？

- 状态：PASS
- 通道：source
- Stop：unresolved_explicit_source
- Work Recall@5/20：N/A / N/A
- Exact-source Recall@5/20：N/A / N/A
- Work/Exact-source/Passage MRR：N/A / N/A / N/A
- Locator：1.000
- Reranker：hybrid-cross-encoder-research-v2 / not_run / 0 ms
