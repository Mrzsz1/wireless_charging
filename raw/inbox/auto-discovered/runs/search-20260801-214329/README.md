# 论文自动发现候选报告

> **边界声明：** 本报告只是外部检索产生的 triage 候选，不是 `raw/canonical`，
> 未经人工确认与 A 编译不得作为 wiki 硬事实，也不代表完整的全球查新。

- 抓取时间（UTC）：`2026-08-01T13:43:29+00:00`
- 来源：`arxiv, openalex, tavily, serpapi`
- 原始命中：`373`；去重后：`40`
- 缓存命中：`0`
- 排序：标题/摘要词项命中 + 轻量时间加分；不是语义相关性或质量判定

## 检索主题

- wireless power transfer scheduling
- wireless rechargeable sensor networks
- mobile charger scheduling
- dynamic wireless charging
- RF energy harvesting scheduling

## 来源告警

- PDF / A novel priority-driven heap-based scheduling algorithm for mobile chargers in wireless rechargeable sensor networks: 下载内容不是 PDF
- PDF / Wireless rechargeable sensor networks: Energy provisioning technologies, charging scheduling schemes, and challenges: PDF 下载返回 HTTP 403
- PDF / Recent advances in wireless rechargeable sensor networks: A comprehensive review of energy management and charging strategies: PDF 下载返回 HTTP 403
- PDF / Efficient charging schedules in a rechargeable wireless sensor network with multiple chargers: S. Ghosh et al.: 下载内容不是 PDF
- PDF / DRL-Based Charging Strategy Optimization for IRS-Assisted UAV in Wireless Rechargeable Sensor Networks: PDF 下载返回 HTTP 403
- PDF / Quantum-Inspired Multi-Objective Optimization Framework for Dynamic Wireless Electric Vehicle Charging in Highway Networks Under Stochastic Traffic and Renewable Energy Variability: PDF 下载返回 HTTP 403
- PDF / Enhanced Evolutionary Multi-Objective Deep Reinforcement Learning for Reliable and Efficient Wireless Rechargeable Sensor Networks: PDF 下载返回 HTTP 404
- PDF / A Reinforcement Learning-Based Dynamic Clustering of Sleep Scheduling Algorithm (RLDCSSA-CDG) for Compressive Data Gathering in Wireless Sensor Networks: PDF 下载返回 HTTP 403
- PDF / Novel joint data collection and wireless charging algorithm for rechargeable wireless sensor networks: 下载内容不是 PDF
- PDF / Advances in Energy Harvesting for Sustainable Wireless Sensor Networks: Challenges and Opportunities: PDF 下载返回 HTTP 403
- triage PDF / A novel priority-driven heap-based scheduling algorithm for mobile chargers in wireless rechargeable sensor networks: 下载内容不是 PDF
- triage PDF / Wireless rechargeable sensor networks: Energy provisioning technologies, charging scheduling schemes, and challenges: PDF 下载返回 HTTP 403
- triage PDF / DRL-Based Charging Strategy Optimization for IRS-Assisted UAV in Wireless Rechargeable Sensor Networks: PDF 下载返回 HTTP 403

## 候选列表

### 1. A novel priority-driven heap-based scheduling algorithm for mobile chargers in wireless rechargeable sensor networks

- 作者：M Kamaruzzaman, A Chandra…
- 日期：2026
- 来源：serpapi, tavily
- 筛选状态：`selected`；人工选择：`true`
- DOI：10.1007/s12083-025-02117-2；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：21.00（标题命中：charger, mobile, rechargeable, scheduling, sensor；摘要命中：charging, dynamic；近年文献；多源交叉命中）
- 命中主题：mobile charger scheduling, wireless rechargeable sensor networks
- 页面：https://link.springer.com/article/10.1007/s12083-025-02117-2
- PDF：https://link.springer.com/content/pdf/10.1007/s12083-025-02117-2.pdf
- 本地 PDF：未下载

… rechargeable sensor networks (WRSNs). This research paper proposes an efficient scheduling scheme for mobile charging … dynamically adjusting charging routes and schedules. The …

### 2. Charging Scheduling of Clustered Wireless Rechargeable Sensor Networks Considering Dynamic Selection of Cluster Heads

- 作者：M Liu, H Yao
- 日期：2026
- 来源：serpapi
- 筛选状态：`promoted`；人工选择：`true`
- DOI：10.32604/cmc.2026.078181；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：20.75（标题命中：charging, dynamic, rechargeable, scheduling, sensor；摘要命中：charger, energy, mobile；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://www.sciopen.com/article/10.32604/cmc.2026.078181
- PDF：https://www.sciopen.com/local/article_pdf/10.32604/cmc.2026.078181.pdf
- 本地 PDF：raw/inbox/auto-discovered/papers/2026-Charging-Scheduling-of-Clustered-Wireless-Rechargeable-Sensor-Networks-0cb745f4/paper.pdf

… charging scheduling problem of mobile chargers in clustered wireless rechargeable sensor networks … neural networks, and energyaware routing for enhanced efficiency and longevity in …

### 3. Collaborative Charging Optimization for Wireless Rechargeable Sensor Networks via Heterogeneous Mobile Chargers

- 作者：Jianhang Yao, Hui Kang, Geng Sun, Jiahui Li, Hongjuan Li, Jiacheng Wang, Yinqiu Liu
- 日期：2025-11-16
- 来源：arxiv
- 筛选状态：`promoted`；人工选择：`true`
- DOI：10.1109/jiot.2026.3696627；arXiv：2511.12501
- 开放获取：True；许可：未提供
- 相关度分数：19.75（标题命中：charger, charging, mobile, rechargeable, sensor；摘要命中：dynamic, energy；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://arxiv.org/abs/2511.12501v2
- PDF：https://arxiv.org/pdf/2511.12501v2
- 本地 PDF：raw/inbox/auto-discovered/papers/2025-Collaborative-Charging-Optimization-for-Wireless-Rechargeable-Sensor-N-2c9e4ede/paper.pdf

Despite the rapid proliferation of Internet of Things applications driving widespread wireless sensor network (WSN) deployment, traditional WSNs remain fundamentally constrained by persistent energy limitations that severely restrict network lifetime and operational sustainability. Wireless rechargeable sensor networks (WRSNs) integrated with wireless power transfer (WPT) technology emerge as a transformative paradigm, theoretically enabling unlimited operational lifetime. In this paper, we investigate a heterogeneous mobile charging architecture that strategically combines an automated aerial vehicle (AAV) and a ground smart vehicle (SV) in heterogeneous deployment scenarios to collaboratively exploit the superior mobility of the AAV and extended endurance of the SV for energy distribution. We formulate a multi-objective optimization problem that simultaneously addresses the dynamic balance of heterogeneous charger advantages, charging efficiency versus mobility energy consumption trade-offs, and real-time adaptive coordination under time-varying network conditions. This problem presents significant computational challenges due to its high-dimensional continuous action space, non-convex optimization landscape, and dynamic environmental constraints. To address these challenges, we propose the improved heterogeneous agent trust region policy optimization (IHATRPO) algorithm that integrates a self-attention mechanism for enhanced complex environmental state processing and employs a Beta sampling strategy to achieve unbiased gradient computation in continuous action spaces. Simulation results demonstrate that IHATRPO achieves a 51% performance improvement over the original HATRPO, significantly outperforming state-of-the-art baseline algorithms while substantially decreasing sensor node mortality rate and improving charging system efficiency.

### 4. Multi-antenna mobile charger scheduling optimization scheme for wireless rechargeable sensor networks

- 作者：J Li, Y Feng, N Liu, M Liu, Y Li
- 日期：2025
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：18.50（标题命中：charger, mobile, rechargeable, scheduling, sensor；摘要命中：charging；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://www.sciencedirect.com/science/article/pii/S0140366425003007
- PDF：—
- 本地 PDF：未下载

… in wireless rechargeable sensor networks. However, existing multi-antenna scheduling … Then, we utilize Double Deep Q-Network to plan MC’s charging path across clusters, which …

### 5. Wireless rechargeable sensor networks: Energy provisioning technologies, charging scheduling schemes, and challenges

- 作者：SA Aziz, X Wang, A Hawbani, B Qureshi…
- 日期：2025
- 来源：serpapi
- 筛选状态：`selected`；人工选择：`true`
- DOI：—；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：17.75（标题命中：charging, energy, rechargeable, scheduling, sensor；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/10918794/
- PDF：https://napier-repository.worktribe.com/OutputFile/4171963
- 本地 PDF：未下载

… charging modes, covering both offline and online modes. We present various charging scheduling … Qiao, “J-roc: A joint routing and charging scheme to prolong sensor network lifetime,” …

### 6. Soft Computing-Based Adaptive Energy Replenishment Via Mobile Charging in Wireless Sensor Networks with Fuzzy Logic and Genetic Algorithms

- 作者：Chatchai Punriboon, Nutthanon Leelathakul, Phet Aimtongkham, Pakarat Musikawan, Chakchai So–In
- 日期：2025-06-25
- 来源：openalex, serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1007/s42979-025-04108-9；arXiv：—
- 开放获取：False；许可：未提供
- 相关度分数：17.75（标题命中：charging, energy, mobile, sensor；摘要命中：charger, scheduling；近年文献；多源交叉命中）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://doi.org/10.1007/s42979-025-04108-9
- PDF：—
- 本地 PDF：未下载

… for the density of low-energy sensor nodes. Thus, this paper … the routes of a mobile charger and recharging sensor nodes. … a fuzzy routing stage, and a fuzzy charging scheduling stage. …

### 7. Revolutionizing wireless rechargeable sensor networks: speed optimization-based charging scheduling scheme (SOCSS) for efficient multi-node energy transfer

- 作者：R Goyal, A Tomar
- 日期：2025
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：17.75（标题命中：charging, energy, rechargeable, scheduling, sensor；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://www.sciencedirect.com/science/article/pii/S2210650225001191
- PDF：—
- 本地 PDF：未下载

… to charge sensor nodes can significantly prolong the lifetime of Wireless Rechargeable Sensor Networks … While previous studies have primarily focused on on-demand recharging within …

### 8. A New Ant Colony Optimization-Based Dynamic Path Planning and Energy Optimization Model in Wireless Sensor Networks for Mobile Sink by Using Mixed …

- 作者：F Chen, X Wu, Z Wang, W Qi, P Li
- 日期：2026
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：16.50（标题命中：dynamic, energy, mobile, sensor；摘要命中：charging, scheduling；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://www.mdpi.com/2313-7673/11/1/44
- PDF：—
- 本地 PDF：未下载

… transforms complex path planning and scheduling problems … introduce adjustable dwell times into trajectory planning: the sink “… the perspectives of optimizing charging station layout and …

### 9. Charging Scheduling Method for Wireless Rechargeable Sensor Networks Based on Energy Consumption Rate Prediction for Nodes

- 作者：未提供
- 日期：未提供
- 来源：tavily
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：16.50（标题命中：charging, energy, rechargeable, scheduling, sensor；摘要命中：dynamic, mobile）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://www.mdpi.com/1424-8220/24/18/5931
- PDF：—
- 本地 PDF：未下载

3. Chanak, P.; Banerjee, I. Congestion Free Routing Mechanism for IoT-Enabled Wireless Sensor Networks for Smart Healthcare Applications. IEEE Trans. Consum. Electron. 2020, 66, 223–232. [Google Scholar] [CrossRef] 4. Lyu, Z.; Wei, Z.; Pan, J.; Chen, H.; Xia, C.; Han, J.; Shi, L. Periodic charging planning for a mobile WCE in wireless rechargeable sensor networks based on hybrid PSO and GA algorithm. Appl. Soft Comput. 2019, 75, 388–403. [Google Scholar] [CrossRef] [...] route as the parent. ECBT–MEMR combines the ECBT and MEMR which is our proposed charging scheduling method. Similarly, NN–MEMR integrates the NN and MEMR as a benchmark method. To validate the advantages of ECBT–MEMR in minimizing the EMR, we choose RAND and LL as the baseline for comparison. In the RAND method, the MC charges all nodes in a random sequence during each T. The LL method takes into account the dynamic changes of the nodes’ energy consumption rate and uses short-term prediction [...] In this section, simulations are conducted to show the advantages of our proposed method. To verify the advantages of ECBT, we employed a routing algorithm that selects the nearest neighbor (NN) as the parent as a baseline for comparison. During the network topology construction phase of NN, all nodes are sorted in ascending order of their distance to the BS. Then, each node is traversed, selecting the nearest neighbor (i.e., the nodes within communication range) with an established route as

### 10. An efficient dynamic energy replenishment and data gathering strategy based on deep reinforcement learning in wireless rechargeable sensor networks

- 作者：M Zhao, J Li, X Chai, J Li
- 日期：2025
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：16.25（标题命中：dynamic, energy, rechargeable, sensor；摘要命中：charging, mobile；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://www.sciencedirect.com/science/article/pii/S111001682500376X
- PDF：—
- 本地 PDF：未下载

The advent of Wireless Rechargeable Sensor Networks (WRSNs) has brought about a new reality where sensor nodes can be recharged wirelessly via mobile charging vehicle. …

### 11. Metaheuristic-Based UAV Charging Scheduling for Wireless Rechargeable Mobile IoT Networks for Disaster Response Applications

- 作者：MA Abusirdaneh, Z Al Aghbari…
- 日期：2026
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：15.75（标题命中：charging, mobile, rechargeable, scheduling；摘要命中：sensor；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://ieeexplore.ieee.org/abstract/document/11565443/
- PDF：—
- 本地 PDF：未下载

… We formulate UAV charging scheduling as an offline trajectory … optimization for path planning in uav networks for long-… via mobile charging in wireless sensor networks with fuzzy …

### 12. Obstacles avoidance charging schedule for multiple mobile charging vehicles in wireless rechargeable sensor networks

- 作者：SMA Rahaman, M Azharuddin…
- 日期：2026
- 来源：serpapi
- 筛选状态：`promoted`；人工选择：`true`
- DOI：10.1504/ijcnds.2026.150924；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：15.50（标题命中：charging, mobile, rechargeable, sensor；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://www.inderscienceonline.com/doi/abs/10.1504/IJCNDS.2026.150924
- PDF：https://www.researchsquare.com/article/rs-3468314/latest.pdf
- 本地 PDF：raw/inbox/auto-discovered/papers/2026-Obstacles-avoidance-charging-schedule-for-multiple-mobile-charging-veh-58f24c70/paper.pdf

… learn and optimise their trajectory design and charging schedules in WRSNs without needing … (2020) ‘An efficient data collection path planning scheme for wireless sensor networks with …

### 13. Study on charging strategy of wireless rechargeable sensor networks based on dynamic inhomogeneous clustering

- 作者：P Tian, J Yang, H Pu, X Tian, J Tang, G Ran, L Peng
- 日期：2025
- 来源：serpapi
- 筛选状态：`promoted`；人工选择：`true`
- DOI：—；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：15.50（标题命中：charging, dynamic, rechargeable, sensor；摘要命中：scheduling；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://www.nature.com/articles/s41598-025-11569-8
- PDF：https://www.nature.com/articles/s41598-025-11569-8.pdf
- 本地 PDF：raw/inbox/auto-discovered/papers/2025-Study-on-charging-strategy-of-wireless-rechargeable-sensor-networks-ba-73049485/paper.pdf

… life cycle and stability, and provides an efficient solution for charging scheduling in … by uneven clustering in wireless sensor networks and proposes an improved routing algorithm for …

### 14. Recent advances in wireless rechargeable sensor networks: A comprehensive review of energy management and charging strategies

- 作者：EA Adjei, G Abdul-Salaam
- 日期：2025
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：15.50（标题命中：charging, energy, rechargeable, sensor；摘要命中：scheduling；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://www.preprints.org/frontend/manuscript/3a4de67c66b0b1bc15bcfcfb9debcbee/download_pub
- PDF：https://www.preprints.org/frontend/manuscript/3a4de67c66b0b1bc15bcfcfb9debcbee/download_pub
- 本地 PDF：未下载

… Charging scheduling adds another layer of complexity, requiring careful coordination to ensure prompt recharging without disrupting network … especially when inefficient routing leads to …

### 15. MCDM-FIS-based charging scheduling for wireless rechargeable sensor networks

- 作者：SA Aziz, X Wang, A Hawbani, F Miao…
- 日期：2025
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：15.50（标题命中：charging, rechargeable, scheduling, sensor；摘要命中：energy；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/10919042/
- PDF：—
- 本地 PDF：未下载

… a charging scheduling problem in each cluster to answer the question (6). Identifying which sensor … We aim to integrate our on-demand charging scheme with energy-efficient routing …

### 16. Efficient charging schedules in a rechargeable wireless sensor network with multiple chargers: S. Ghosh et al.

- 作者：S Ghosh, K Chakraborty, PB Khatua…
- 日期：2025
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1007/s11227-024-06804-4；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：15.25（标题命中：charger, charging, rechargeable, sensor；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://link.springer.com/article/10.1007/s11227-024-06804-4
- PDF：https://openurl.ebsco.com/fulltext/gcd:181721790?sid=ebsco:plink:crawler-gcd&id=ebsco:gcd:181721790&crl=f&jrnl=09208542
- 本地 PDF：未下载

… this updated network is coined as rechargeable wireless sensor network (RWSN) [9]. Recharging schedules for … A gradient-based routing for UAV is also put forward in [13] increasing its …

### 17. Deep Learning and Optimization Approaches in Deep Convolutional U-Shape Network with Jump Attention-Based Vision Transformer for Integrated Sequence Scheduling and Trajectory Planning with Obstacle Avoidance in Wireless Rechargeable Sensor Networks: A Rev

- 作者：Soraya Fernandes-Pereira
- 日期：2025-04-12
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.65521/ijeecs.v14i1.1941；arXiv：—
- 开放获取：True；许可：cc-by-nd
- 相关度分数：15.25（标题命中：rechargeable, scheduling, sensor；摘要命中：charger, dynamic, energy, mobile；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://doi.org/10.65521/ijeecs.v14i1.1941
- PDF：https://journals.mriindia.com/index.php/ijeecs/article/download/1941/1868
- 本地 PDF：raw/inbox/auto-discovered/runs/search-20260801-214329/papers/017-Deep-Learning-and-Optimization-Approaches-in-Deep-Convolutional-U-Shape-Network-with-Jump/paper.pdf

Wireless Rechargeable Sensor Networks (WRSNs) have emerged as an effective solution to address the energy limitations of traditional Wireless Sensor Networks by incorporating mobile chargers and intelligent energy management strategies. However, efficient sequence scheduling and trajectory planning for mobile chargers remain significant challenges, especially in dynamic environments with obstacles. Recent advancements in deep learning and optimization techniques have provided promising solutions to these problems. This review highlights deep convolutional U-shape networks (U-Net) integrated with jump attention-based Vision Transformers (ViTs) for optimizing scheduling and trajectory planning in WRSNs. U-Net models are effective for spatial feature extraction due to their encoder–decoder structure, while Vision Transformers enhance global context modelling through self-attention mechanisms. The integration of convolutional and transformer-based approaches improves both local and global feature learning, leading to better performance in path planning and obstacle avoidance. Additionally, attention-based spatial–temporal models improve trajectory prediction by capturing complex interactions in dynamic environments. Despite these advancements, challenges such as energy constraints, computational complexity, scalability, and real-time deployment persist, indicating the need for efficient and adaptive solutions.

### 18. Deep Learning and Optimization Approaches for Sequence Scheduling and Trajectory Planning in Wireless Rechargeable Sensor Networks

- 作者：Haleema Fernandes-Pereira
- 日期：2025-05-25
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.65521/itsi-teee.v14i1.2813；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：15.25（标题命中：rechargeable, scheduling, sensor；摘要命中：charger, dynamic, energy, mobile；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://doi.org/10.65521/itsi-teee.v14i1.2813
- PDF：https://journals.mriindia.com/index.php/itsiteee/article/download/2813/2694
- 本地 PDF：raw/inbox/auto-discovered/runs/search-20260801-214329/papers/018-Deep-Learning-and-Optimization-Approaches-for-Sequence-Scheduling-and-Trajectory-Planning/paper.pdf

Wireless Rechargeable Sensor Networks (WRSNs) have emerged as an effective solution for addressing the energy limitations of traditional Wireless Sensor Networks by integrating mobile chargers and intelligent energy management mechanisms. However, efficient sequence scheduling and trajectory planning for mobile chargers remain major challenges, especially in dynamic environments containing obstacles and varying network conditions. Recently, deep learning and optimization-based techniques have shown significant potential for solving these complex problems. In particular, deep convolutional U-shape networks (U-Net) integrated with jump attention-based Vision Transformers (ViTs) have attracted considerable attention for trajectory planning and obstacle avoidance in WRSNs. U-Net architectures effectively capture spatial features through encoder–decoder structures, while Vision Transformers utilize self-attention mechanisms to model long-range dependencies and global contextual relationships. The combination of convolutional operations with transformer architectures improves local feature extraction and global dependency learning, resulting in more accurate path prediction and efficient mobile charger scheduling. Recent studies indicate that attention-based spatial–temporal models significantly enhance trajectory optimization, obstacle avoidance, and navigation efficiency in dynamic WRSN environments. Furthermore, hybrid deep learning and optimization frameworks improve energy efficiency, scalability, and scheduling accuracy. However, challenges such as computational complexity, real-time implementation, and resource constraints continue to motivate future research in intelligent WRSN systems.

### 19. UAV Joint Scheduling for Optimizing Charging and Data Collection in Wireless Rechargeable Sensor Networks

- 作者：W Lai, C Sha, J Wang, S Xia…
- 日期：2026
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：15.00（标题命中：charging, rechargeable, scheduling, sensor；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/11408735/
- PDF：—
- 本地 PDF：未下载

… rechargeable sensor networks (WRSNs), this article proposes a joint scheduling scheme for unmanned … Douzi, “Adaptive routing protocol for lifetime maximization in multi-constraint …

### 20. Study on charging strategy of wireless rechargeable sensor networks based on dynamic inhomogeneous clustering | Scientific Reports

- 作者：未提供
- 日期：未提供
- 来源：tavily
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：15.00（标题命中：charging, dynamic, rechargeable, sensor；摘要命中：charger, energy, mobile, scheduling）
- 命中主题：mobile charger scheduling
- 页面：https://www.nature.com/articles/s41598-025-11569-8
- PDF：—
- 本地 PDF：未下载

1 (2020).") proposes a new WRSN on-demand billing scheduling scheme. First, an efficient network partitioning method is proposed for allocating MCs in order to evenly balance their workloads. Next, fuzzy logic, which mixes various network attributes is employed to determine the charging schedule of MCs. We also develop an expression to determine the charging threshold of a node, which varies according to its energy consumption rate. However, in large-scale networks, MC’s mobility path planning [...] location and energy consumption rate to dynamically adjust the cluster structure, and introduces a weight function (combining the initial energy, remaining energy and average distance within the cluster) to elect the cluster head to balance the energy consumption, and (2) designing a mobile charging vehicle (MC) charging path planning strategy that determines the stopping points for single-node and multi-node clusters, respectively, and scheduling them through hybrid priority (distance, [...] demand, and location of each sensor node in the network, and plans reasonable charging paths and scheduling schemes for the mobile chargers, prioritizing the charging demand of nodes that are about to run out of energy and have high charging waiting time requirements, so as to ensure a balanced supply of energy in the whole network, and to effectively avoid node failures caused by energy starvation.) strategy is a classical online charging strategy, while the FCFS (First-Come, First-Served,

### 21. ISAC-Enabled On-Demand UAV Charging for Wireless Rechargeable Sensor Networks

- 作者：Muhammad Umar Farooq Qaisar, Lin Zhang, Paolo Bellavista, Shehzad Ashraf Chaudhry, Shamsher Ullah, Chang Liu
- 日期：2026-07-26
- 来源：arxiv
- 筛选状态：`promoted`；人工选择：`true`
- DOI：—；arXiv：2607.23572
- 开放获取：True；许可：未提供
- 相关度分数：14.75（标题命中：charging, rechargeable, sensor；摘要命中：dynamic, energy, scheduling；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://arxiv.org/abs/2607.23572v1
- PDF：https://arxiv.org/pdf/2607.23572v1
- 本地 PDF：raw/inbox/auto-discovered/papers/2026-ISAC-Enabled-On-Demand-UAV-Charging-for-Wireless-Rechargeable-Sensor-N-0be40014/paper.pdf

Unmanned aerial vehicles (UAVs) equipped with wireless power transfer (WPT) extend the lifetime of wireless rechargeable sensor networks (WRSNs) by delivering energy on demand. This article presents an integrated sensing and communication (ISAC)-enabled on-demand UAV charging framework coordinated by a central base station. A prioritized charging queue captures node urgency and service cost through residual energy, traffic load, estimated UAV travel time, and flight-direction alignment. This bidirectional coupling ensures that scheduling decisions shape the UAV trajectory, while updated mobility estimates from ISAC dynamically reorder the queue. ISAC-assisted estimation of UAV distance, speed, and position updates travel-time predictions under mobility uncertainty. A time-allocated partial charging policy distributes limited hover time across queued nodes according to criticality. Simulations show gains in energy usage efficiency, travel distance, and charging delay compared with representative baselines. We discuss deployment considerations, including computational overhead, scalability, and parameter selection, to aid practitioners evaluating the framework for IoT scenarios.

### 22. Secure Charging Scheduling in Wireless Rechargeable Sensor Networks

- 作者：W Yang, C Lin, J Deng, H Dai, L Chen…
- 日期：2025
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：14.75（标题命中：charging, rechargeable, scheduling, sensor；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/11146916/
- PDF：—
- 本地 PDF：未下载

… of sensors and MC. Furthermore, to deal with the second challenge, we design a near-optimal charging scheduling … routes determined by our near-optimal charging scheduling scheme. …

### 23. Modelled Energy Cost Minimization Solution for Wireless Rechargeable Sensor Networks

- 作者：M Ahmed, DS NYITAMEN…
- 日期：2025
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：14.75（标题命中：energy, rechargeable, sensor；摘要命中：charger, charging, mobile, scheduling；近年文献）
- 命中主题：mobile charger scheduling
- 页面：http://journals.abuad.edu.ng/ajerd/article/view/965
- PDF：https://journals.abuad.edu.ng/ajerd/article/download/965/677
- 本地 PDF：raw/inbox/auto-discovered/runs/search-20260801-214329/papers/023-Modelled-Energy-Cost-Minimization-Solution-for-Wireless-Rechargeable-Sensor-Networks/paper.pdf

… Sensor nodes (SNs) in wireless sensor networks (WSNs) are … ’ utility energies, planning an on-demand charging trajectory to … -aware charging scheduling for multiple mobile chargers in …

### 24. DRL-Based Charging Strategy Optimization for IRS-Assisted UAV in Wireless Rechargeable Sensor Networks

- 作者：X Liu, C Zhao, S Chen, T Wang, F Chen
- 日期：2026
- 来源：serpapi
- 筛选状态：`selected`；人工选择：`true`
- DOI：10.1145/3789206；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：14.00（标题命中：charging, rechargeable, sensor；摘要命中：charger, energy；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://dl.acm.org/doi/abs/10.1145/3789206
- PDF：https://dl.acm.org/doi/pdf/10.1145/3789206
- 本地 PDF：未下载

… chargers in wireless sensor networks through sensor energy management, … to the subproblem of UAVs’ trajectory planning. In this section, DRL is employed for path planning, where the …

### 25. Synergistic Charger Deployment and Charging Scheduling for Heterogeneous WRSNs

- 作者：CH Lin, HY Huang, YC Sung…
- 日期：2025
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：14.00（标题命中：charger, charging, scheduling；摘要命中：dynamic, rechargeable, sensor；近年文献）
- 命中主题：mobile charger scheduling, wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/11236455/
- PDF：—
- 本地 PDF：未下载

… sensors and dynamic sensors with unpredictable trajectories. … for wireless sensor networks with multiple portable chargers,” … Wu, “Wireless rechargeable sensor networks with separable …

### 26. Reinforcement Learning for Hybrid Charging Stations Planning and Operation Considering Fixed and Mobile Chargers

- 作者：Yanchen Zhu, Honghui Zou, Chufan Liu, Yuyu Luo, Yuankai Wu, Yuxuan Liang
- 日期：2025-06-20
- 来源：arxiv
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：2506.16764
- 开放获取：True；许可：未提供
- 相关度分数：13.75（标题命中：charger, charging, mobile；摘要命中：dynamic, scheduling；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://arxiv.org/abs/2506.16764v2
- PDF：https://arxiv.org/pdf/2506.16764v2
- 本地 PDF：raw/inbox/auto-discovered/runs/search-20260801-214329/papers/026-Reinforcement-Learning-for-Hybrid-Charging-Stations-Planning-and-Operation-Considering-Fix/paper.pdf

The success of vehicle electrification relies on efficient and adaptable charging infrastructure. Fixed-location charging stations often suffer from underutilization or congestion due to fluctuating demand, while mobile chargers offer flexibility by relocating as needed. This paper studies the optimal planning and operation of hybrid charging infrastructures that combine both fixed and mobile chargers within urban road networks. We formulate the Hybrid Charging Station Planning and Operation (HCSPO) problem, jointly optimizing the placement of fixed stations and the scheduling of mobile chargers. A charging demand prediction model based on Model Predictive Control (MPC) supports dynamic decision-making. To solve the HCSPO problem, we propose a deep reinforcement learning approach enhanced with heuristic scheduling. Experiments on real-world urban scenarios show that our method improves infrastructure availability - achieving up to 244.4% increase in coverage - and reduces user inconvenience with up to 79.8% shorter waiting times, compared to existing solutions.

### 27. Resilient Sector Scheduling Scheme for Wireless Rechargeable Sensor Networks

- 作者：CH Lin, YT Lin, YC Sung…
- 日期：2026
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：13.50（标题命中：rechargeable, scheduling, sensor；摘要命中：charger, charging；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/11526730/
- PDF：—
- 本地 PDF：未下载

… Traditional approaches often address node clustering, charging strategies, and charger routing independently, frequently failing to adapt to real-time demand variations and …

### 28. Energy Optimization in UAV-Based Wireless Rechargeable Sensor Networks

- 作者：L Zhu, J Li, J Wu, R Jia, C Li, M Li
- 日期：2026
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：13.50（标题命中：energy, rechargeable, sensor；摘要命中：charging, scheduling；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/11516501/
- PDF：—
- 本地 PDF：未下载

… design and charging scheduling algorithms that not only maximize the charging utility of … Elghitani [19] introduced a Lyapunov optimization-based UAV routing and task scheduling …

### 29. Learning-driven charging trajectories in WRSNs: a sector-based MST approach for energy efficiency

- 作者：A Singh, A Tomar, S Agrawal, A Achar
- 日期：2026
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1007/s11276-025-04038-7；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：13.25（标题命中：charging, energy；摘要命中：charger, dynamic, mobile, scheduling, sensor；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://link.springer.com/article/10.1007/s11276-025-04038-7
- PDF：—
- 本地 PDF：未下载

… sustainable operations in Wireless Sensor Networks (WSNs) … and scheduling of Mobile Chargers (MCs) in such networks … Dynamic charging scheduling and path planning scheme for …

### 30. Wireless Rechargeable Sensor Networks: A Comprehensive Review of Static Sensor Charging Techniques

- 作者：S Deb, S Chatterjee, A Khan, R Das…
- 日期：2025
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：13.25（标题命中：charging, rechargeable, sensor；摘要命中：charger, mobile；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/11365664/
- PDF：—
- 本地 PDF：未下载

… Mobile chargers adhere to predetermined routes and schedules in a systematic … charging protocol in wireless rechargeable sensor networks,” ACM Transactions on Sensor Networks, …

### 31. Quantum-Inspired Multi-Objective Optimization Framework for Dynamic Wireless Electric Vehicle Charging in Highway Networks Under Stochastic Traffic and Renewable Energy Variability

- 作者：Dong Hua, Chenzhang Chang, Suisheng Liu, Yiqing Liu, Daolin Ma, Hua Hua
- 日期：2025-04-07
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.3390/wevj16040221；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：13.00（标题命中：charging, dynamic, energy；摘要命中：scheduling；近年文献）
- 命中主题：dynamic wireless charging
- 页面：https://doi.org/10.3390/wevj16040221
- PDF：https://www.mdpi.com/2032-6653/16/4/221/pdf?version=1744077227
- 本地 PDF：未下载

The rapid adoption of electric vehicles (EVs) and the increasing reliance on renewable energy sources necessitate innovative charging infrastructure solutions to address key challenges in energy efficiency, grid stability, and sustainable transportation. Dynamic wireless charging systems, which enable EVs to charge while in motion, offer a transformative approach to mitigating range anxiety and optimizing energy utilization. However, these systems face significant operational challenges, including dynamic traffic conditions, uncertain EV arrival patterns, energy transfer efficiency variations, and renewable energy intermittency. This paper proposes a novel quantum computing-assisted optimization framework for the modeling, operation, and control of wireless dynamic charging infrastructure in urban highway networks. Specifically, we leverage Variational Quantum Algorithms (VQAs) to address the high-dimensional, multi-objective optimization problem associated with real-time energy dispatch, charging pad utilization, and traffic flow coordination. The mathematical modeling framework captures critical aspects of the system, including power balance constraints, state-of-charge (SOC) dynamics, stochastic vehicle arrivals, and charging efficiency degradation due to vehicle misalignment and speed variations. The proposed methodology integrates quantum-inspired optimization techniques with classical distributionally robust optimization (DRO) principles, ensuring adaptability to system uncertainties while maintaining computational efficiency. A comprehensive case study is conducted on a 50 km urban highway network equipped with 20 charging pad segments, supporting an average traffic flow of 10,000 EVs per day. The results demonstrate that the proposed quantum-assisted approach significantly enhances energy efficiency, reducing energy losses by up to 18% compared to classical optimization methods. Moreover, traffic-aware adaptive charging strategies improve SOC recovery by 25% during peak congestion periods while ensuring equitable energy allocation among different vehicle types. The framework also facilitates a 30% increase in renewable energy utilization, aligning energy dispatch with periods of high solar and wind generation. Key insights from the case study highlight the critical impact of vehicle alignment, speed variations, and congestion on wireless charging performance, emphasizing the need for intelligent scheduling and real-time optimization. The findings contribute to advancing the integration of quantum computing into sustainable transportation planning, offering a scalable and robust solution for next-generation EV charging infrastructure.

### 32. Enhanced Evolutionary Multi-Objective Deep Reinforcement Learning for Reliable and Efficient Wireless Rechargeable Sensor Networks

- 作者：Bowei Tong, Hui Kang, Jiahui Li, Geng Sun, Jiacheng Wang, Yaoqi Yang, Bo Xu, Dusit Niyato
- 日期：2025-10-24
- 来源：arxiv
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：2510.21127
- 开放获取：True；许可：未提供
- 相关度分数：13.00（标题命中：rechargeable, sensor；摘要命中：charger, charging, dynamic, energy, mobile；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://arxiv.org/abs/2510.21127v2
- PDF：https://arxiv.org/pdf/2510.21127v2
- 本地 PDF：未下载

Despite rapid advancements in sensor networks, conventional battery-powered sensor networks suffer from limited operational lifespans and frequent maintenance requirements that severely constrain their deployment in remote and inaccessible environments. As such, wireless rechargeable sensor networks (WRSNs) with mobile charging capabilities offer a promising solution to extend network lifetime. However, WRSNs face critical challenges from the inherent trade-off between maximizing the node survival rates and maximizing charging energy efficiency under dynamic operational conditions. In this paper, we investigate a typical scenario where mobile chargers move and charge the sensor, thereby maintaining the network connectivity while minimizing the energy waste. Specifically, we formulate a multi-objective optimization problem that simultaneously maximizes the network node survival rate and mobile charger energy usage efficiency across multiple time slots, which presents NP-hard computational complexity with long-term temporal dependencies that make traditional optimization approaches ineffective. To address these challenges, we propose an enhanced evolutionary multi-objective deep reinforcement learning algorithm, which integrates a long short-term memory (LSTM)-based policy network for temporal pattern recognition, a multilayer perceptron-based prospective increment model for future state prediction, and a time-varying Pareto policy evaluation method for dynamic preference adaptation. Extensive simulation results demonstrate that the proposed algorithm significantly outperforms existing approaches in balancing node survival rate and energy efficiency while generating diverse Pareto-optimal solutions. Moreover, the LSTM-enhanced policy network converges 25% faster than conventional networks, with the time-varying evaluation method effectively adapting to dynamic conditions.

### 33. A Reinforcement Learning-Based Dynamic Clustering of Sleep Scheduling Algorithm (RLDCSSA-CDG) for Compressive Data Gathering in Wireless Sensor Networks

- 作者：Alaa N. El-Shenhabi, Ehab H. Abdelhay, Mohamed A. Mohamed, Ibrahim F. Moawad
- 日期：2025-01-08
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.3390/technologies13010025；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：13.00（标题命中：dynamic, scheduling, sensor；摘要命中：energy；近年文献）
- 命中主题：dynamic wireless charging
- 页面：https://doi.org/10.3390/technologies13010025
- PDF：https://www.mdpi.com/2227-7080/13/1/25/pdf?version=1736346425
- 本地 PDF：未下载

Energy plays a major role in wireless sensor networks (WSNs), and measurements demonstrate that transmission consumes more energy than processing. Hence, organizing the transmission process and managing energy usage throughout the network are the main goals for maximizing the network’s lifetime. This paper proposes an algorithm called RLDCSSA-CDG, which is processed through the 3F phases: foundation, formation, and forwarding phases. Firstly, the network architecture is founded, and the cluster heads (CHs) are determined in the foundation phase. Secondly, sensor nodes are dynamically gathered into clusters for better communication in the formation phase. Finally, the transmitting process will be adequately organized based on an adaptive wake-up/sleep scheduling algorithm to transmit the data at the “right time” in the forwarding phase. The MATLAB platform was utilized to conduct simulation studies to validate the proposed RLDCSSA-CDG’s effectiveness. Compared to a very recent work called RLSSA and RLDCA for CDG, the proposed RLDCSSA-CDG reduces total data transmissions by 22.7% and 63.3% and energy consumption by 8.93% and 38.8%, respectively. It also achieves the lowest latency compared to the two contrastive algorithms. Furthermore, the proposed algorithm increases the whole network lifetime by 77.3% and promotes data recovery accuracy by 91.1% relative to the compared algorithms.

### 34. A Proactive Charging Approach for Extending the Lifetime of Sensor Nodes in Wireless Rechargeable Sensor Networks

- 作者：O Banimelhem, S Bani Hamad
- 日期：2025
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：12.50（标题命中：charging, rechargeable, sensor；摘要命中：scheduling；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://www.mdpi.com/2224-2708/14/2/26
- PDF：—
- 本地 PDF：未下载

… The multi-hop routing in wireless sensor networks refers to the process of transmitting data … scheduling method used in wireless rechargeable sensor networks. In this approach, …

### 35. Novel joint data collection and wireless charging algorithm for rechargeable wireless sensor networks

- 作者：P Chandra, S Soni
- 日期：2025
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1007/s12083-024-01870-0；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：12.25（标题命中：charging, rechargeable, sensor；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://link.springer.com/article/10.1007/s12083-024-01870-0
- PDF：https://link.springer.com/content/pdf/10.1007/s12083-024-01870-0.pdf
- 本地 PDF：未下载

… in a table of routes and to schedule visits … a wireless rechargeable sensor network (WRSN) that is positioned throughout a target area. Thousands of sensor nodes make up the network, …

### 36. Impact and optimization of vehicle charging scheduling on regional clean energy power supply network management

- 作者：Penghui Xu, Xiaobo Wang, Zhichao Li
- 日期：2025-01-28
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1186/s42162-025-00476-x；arXiv：—
- 开放获取：True；许可：cc-by-nc-nd
- 相关度分数：12.25（标题命中：charging, energy, scheduling；近年文献）
- 命中主题：dynamic wireless charging, wireless power transfer scheduling, wireless rechargeable sensor networks
- 页面：https://doi.org/10.1186/s42162-025-00476-x
- PDF：https://energyinformatics.springeropen.com/counter/pdf/10.1186/s42162-025-00476-x
- 本地 PDF：raw/inbox/auto-discovered/runs/search-20260801-214329/papers/036-Impact-and-optimization-of-vehicle-charging-scheduling-on-regional-clean-energy-power-supp/paper.pdf

Driven by the global energy transition, the widespread use of electric vehicles has profoundly reshaped the transportation landscape and thrown many problems to the power system, and coordinating their charging needs with renewable energy generation has become a key part of ensuring the stable operation of regional clean energy power supply networks. This study focuses on the problem of vehicle charging dispatch to make a breakthrough, deeply analyzes the effect and efficiency of the clean energy grid, and then proposes a series of targeted measures to effectively improve the operational efficiency and reliability of the energy system. The comprehensive model integrates electric vehicle charging stations, distributed photovoltaic power generation systems, wind farms, and battery energy storage devices and enables the charging process to be accurately controlled with real-time monitoring and intelligent algorithms. In particular, the demand forecasting model based on machine learning effectively solves the dilemma of matching the charging load with a clean energy supply. Experimental data strongly confirms that the optimization strategy has led to a 15% reduction in peak load on the grid, a 23% increase in the proportion of clean energy consumption, and a 10% reduction in total electricity consumption. For policymakers, these achievements can be used as a guide to help formulate energy policies and build a framework for adapting to the development of new energy. For practitioners, they serve as a guide to energy planning, grid dispatch, and technology research and development to improve effectiveness. The research promotes the growth of green energy, optimizes the energy structure, lays the foundation for a low-carbon and environmentally friendly society, affects the economy, environment, culture, and other fields, and becomes a key force driving sustainable development.

### 37. Advances in Energy Harvesting for Sustainable Wireless Sensor Networks: Challenges and Opportunities

- 作者：Muhammad Umer Mushtaq, Hein S. Venter, Avinash Singh, Muhammad Owais
- 日期：2025-02-20
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.3390/hardware3010001；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：12.25（标题命中：energy, harvesting, sensor；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://doi.org/10.3390/hardware3010001
- PDF：https://www.mdpi.com/2813-6640/3/1/1/pdf?version=1740038076
- 本地 PDF：未下载

Energy harvesting wireless sensor networks (EH-WSNs) appear as the fundamental backbone of research that attempts to expand the lifespan and efficiency of sensor networks positioned in resource-constrained environments. This review paper provides a in-depth examination of latest developments in this area, highlighting the important components comprising routing protocols, energy management plans, cognitive radio applications, physical layer security (PLS), and EH approaches. Across a well-ordered investigation of these features, the article clarifies the notable developments in technology, highlights recent barriers, and inquires avenues for future revolution. The article starts by furnishing a detailed analysis of different energy harvesting methodologies, incorporating solar, thermal, kinetic, and radio frequency (RF) energy, and their respective efficacy in non-identical operational circumstances. It also inspects state-of-the-art energy management techniques aimed at optimizing energy consumption and storage to guarantee network operability. Moreover, the integration of cognitive radio into EH-WSNs is acutely assessed, highlighting its capacity to improve spectrum efficiency and tackle associated technological problems. The present work investigates ground-breaking methodologies in Physical Layer Security (PLS) that uses energy harvesting measures to improve the data security. In this review article, these techniques are explored with respect to classical encryption and discussed its as well the network security points of view. The assessment furthers criticizes traditional routing protocols and their significance in Energy Harvesting Wireless Sensor Networks (EH-WSNs) as well as the balance that has long been sought between energy efficiency and security in this space. The paper closes with the importance of continuous research to tackle existing challenges and to leverage newly available means as highlighted in this document. In order to adequately serve the increasingly changing requirements of EH-WSNs, future research will and should, be geared towards incorporating AI techniques with some advanced energy storage solutions. This paper discusses the integration of novel methodologies and interdisciplinary advancements for better performance, security and sustainability for WSNs.

### 38. Adaptive Ergodic Search with Energy-Aware Scheduling for Persistent Multi-Robot Missions

- 作者：Kaleb Ben Naveed, Devansh R. Agrawal, Rahul Kumar, Dimitra Panagou
- 日期：2025-05-16
- 来源：arxiv
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：2505.11663
- 开放获取：True；许可：未提供
- 相关度分数：12.25（标题命中：energy, scheduling；摘要命中：charging, dynamic, mobile, rechargeable；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://arxiv.org/abs/2505.11663v1
- PDF：https://arxiv.org/pdf/2505.11663v1
- 本地 PDF：raw/inbox/auto-discovered/runs/search-20260801-214329/papers/038-Adaptive-Ergodic-Search-with-Energy-Aware-Scheduling-for-Persistent-Multi-Robot-Missions/paper.pdf

Autonomous robots are increasingly deployed for long-term information-gathering tasks, which pose two key challenges: planning informative trajectories in environments that evolve across space and time, and ensuring persistent operation under energy constraints. This paper presents a unified framework, mEclares, that addresses both challenges through adaptive ergodic search and energy-aware scheduling in multi-robot systems. Our contributions are two-fold: (1) we model real-world variability using stochastic spatiotemporal environments, where the underlying information evolves unpredictably due to process uncertainty. To guide exploration, we construct a target information spatial distribution (TISD) based on clarity, a metric that captures the decay of information in the absence of observations and highlights regions of high uncertainty; and (2) we introduce Robustmesch (Rmesch), an online scheduling method that enables persistent operation by coordinating rechargeable robots sharing a single mobile charging station. Unlike prior work, our approach avoids reliance on preplanned schedules, static or dedicated charging stations, and simplified robot dynamics. Instead, the scheduler supports general nonlinear models, accounts for uncertainty in the estimated position of the charging station, and handles central node failures. The proposed framework is validated through real-world hardware experiments, and feasibility guarantees are provided under specific assumptions.

### 39. Bio-Inspired Energy-Efficient Routing for Wireless Sensor Networks Based on Honeybee Foraging Behavior and MDP-Driven Adaptive Scheduling

- 作者：F Chen, X Wu, W Qi, Z Wang, Z Wang, P Li
- 日期：2026
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：12.00（标题命中：energy, scheduling, sensor；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://www.mdpi.com/2313-7673/11/5/311
- PDF：—
- 本地 PDF：未下载

… the seamless integration of global path planning and local routing adaptation in highly non-stationary WSNs. Many existing approaches tend to treat sink trajectory selection and real-…

### 40. Deep reinforcement learning for AoI-aware UAV-assisted networks with RF energy harvesting

- 作者：GK Pandey, DS Gurjar, S Yadav…
- 日期：2025-03-13
- 来源：openalex, serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1109/lnet.2025.3550931；arXiv：—
- 开放获取：False；许可：未提供
- 相关度分数：11.75（标题命中：energy, harvesting；摘要命中：dynamic, scheduling；近年文献；多源交叉命中）
- 命中主题：RF energy harvesting scheduling, dynamic wireless charging
- 页面：https://ieeexplore.ieee.org/abstract/document/10925373/
- PDF：—
- 本地 PDF：未下载

This letter considers UAV-assisted data collection from energy-constrained Internet of Things (IoT) devices. Herein, a UAV utilizes radio frequency-based wireless power transfer technique to charge multiple IoT devices or schedules one IoT device to transmit its sensed data. Using the harvested energy, the IoT devices share the collected data with the UAV as per their schedule. For this setup, we aim to minimize IoT devices’ average Age of Information (AoI) by optimally controlling the UAV’s trajectory and scheduling of IoT devices while adhering to the energy consumption limitations of UAV and IoT devices. Considering the dynamic scenario for the considered network, the optimization problem is modeled as a Markov Decision Process and solved through dueling double deep Q-networks (D3QN) algorithm. The simulation results show that the proposed framework outperforms the baseline methods in reducing the average AoI of the IoT devices.

## 下一步

1. 用 `tools/paper-triage.ps1 <results.json> --select <序号>` 标记真正相关的候选；
2. 从 `raw/inbox/auto-discovered/papers/` 选择项晋升 `raw/canonical/`；
3. 用 MinerU 生成 Markdown，并保留 provenance；
4. 按 `schema/agent-a-compile.md` 执行 A 编译，再更新 Graphify。
