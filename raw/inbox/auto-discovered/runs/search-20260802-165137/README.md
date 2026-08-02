# 论文自动发现候选报告

> **边界声明：** 本报告只是外部检索产生的 triage 候选，不是 `raw/canonical`，
> 未经人工确认与 A 编译不得作为 wiki 硬事实，也不代表完整的全球查新。

- 抓取时间（UTC）：`2026-08-02T08:51:37+00:00`
- 来源：`arxiv, openalex, tavily, serpapi`
- 原始命中：`335`；去重后：`50`
- 缓存命中：`10`
- 排序：标题/摘要词项命中 + 轻量时间加分；不是语义相关性或质量判定

## 检索主题

- wireless power transfer scheduling
- wireless rechargeable sensor networks
- mobile charger scheduling
- dynamic wireless charging
- RF energy harvesting scheduling

## 候选列表

### 1. Attention-Shared Multi-Agent Actor–Critic-Based Deep Reinforcement Learning Approach for Mobile Charging Dynamic Scheduling in Wireless Rechargeable Sensor Networks

- 作者：Chengpeng Jiang, Ziyang Wang, Shuai Chen, Jinglin Li, Haoran Wang, Xiang Jinwei, Wendong Xiao
- 日期：2022-07-12
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.3390/e24070965；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：22.00（标题命中：charging, dynamic, mobile, rechargeable, scheduling, sensor；摘要命中：charger, energy；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://doi.org/10.3390/e24070965
- PDF：https://www.mdpi.com/1099-4300/24/7/965/pdf?version=1657697991
- 本地 PDF：未下载

The breakthrough of wireless energy transmission (WET) technology has greatly promoted the wireless rechargeable sensor networks (WRSNs). A promising method to overcome the energy constraint problem in WRSNs is mobile charging by employing a mobile charger to charge sensors via WET. Recently, more and more studies have been conducted for mobile charging scheduling under dynamic charging environments, ignoring the consideration of the joint charging sequence scheduling and charging ratio control (JSSRC) optimal design. This paper will propose a novel attention-shared multi-agent actor-critic-based deep reinforcement learning approach for JSSRC (AMADRL-JSSRC). In AMADRL-JSSRC, we employ two heterogeneous agents named charging sequence scheduler and charging ratio controller with an independent actor network and critic network. Meanwhile, we design the reward function for them, respectively, by considering the tour length and the number of dead sensors. The AMADRL-JSSRC trains decentralized policies in multi-agent environments, using a centralized computing critic network to share an attention mechanism, and it selects relevant policy information for each agent at every charging decision. Simulation results demonstrate that the proposed AMADRL-JSSRC can efficiently prolong the lifetime of the network and reduce the number of death sensors compared with the baseline algorithms.

### 2. Optimal and dynamic scheduling using multiple mobile chargers in rechargeable sensor networks: An madm-based approach

- 作者：R Goyal, A Tomar
- 日期：2022
- 来源：serpapi, tavily
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1145/3549206.3549313；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：21.50（标题命中：charger, dynamic, mobile, rechargeable, scheduling, sensor；近年文献；多源交叉命中）
- 命中主题：mobile charger scheduling
- 页面：https://dl.acm.org/doi/fullHtml/10.1145/3549206.3549313
- PDF：https://dl.acm.org/doi/pdf/10.1145/3549206.3549313
- 本地 PDF：未下载

by R Goyal · 2022 · Cited by 7 — The trajectory for mobile charger is determined using one of the MADM technique where MADM is the way to choose the best alternative based on some criteria [12]

### 3. Optimized Charging Scheduling with Single Mobile Charger for Wireless Rechargeable Sensor Networks

- 作者：Qihua Wang, Fanzhi Kong, Meng Wang, Huaqun Wang
- 日期：2017-11-21
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.3390/sym9110285；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：20.75（标题命中：charger, charging, mobile, rechargeable, scheduling, sensor；摘要命中：dynamic, energy）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://doi.org/10.3390/sym9110285
- PDF：https://www.mdpi.com/2073-8994/9/11/285/pdf?version=1511279489
- 本地 PDF：未下载

Due to the rapid development of wireless charging technology, the recharging issue in wireless rechargeable sensor network (WRSN) has been a popular research problem in the past few years. The weakness of previous work is that charging route planning is not reasonable. In this work, a dynamic optimal scheduling scheme aiming to maximize the vacation time ratio of a single mobile changer for WRSN is proposed. In the proposed scheme, the wireless sensor network is divided into several sub-networks according to the initial topology of deployed sensor networks. After comprehensive analysis of energy states, working state and constraints for different sensor nodes in WRSN, we transform the optimized charging path problem of the whole network into the local optimization problem of the sub networks. The optimized charging path with respect to dynamic network topology in each sub-network is obtained by solving an optimization problem, and the lifetime of the deployed wireless sensor network can be prolonged. Simulation results show that the proposed scheme has good and reliable performance for a small wireless rechargeable sensor network.

### 4. Near Optimal Charging Scheduling for 3-D Wireless Rechargeable Sensor Networks with Energy Constraints

- 作者：Chi Lin, Chunyang Guo, Haipeng Dai, Lei Wang, Guowei Wu
- 日期：2019-07-01
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1109/icdcs.2019.00068；arXiv：—
- 开放获取：False；许可：未提供
- 相关度分数：18.25（标题命中：charging, energy, rechargeable, scheduling, sensor；摘要命中：charger, mobile）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://doi.org/10.1109/icdcs.2019.00068
- PDF：—
- 本地 PDF：未下载

Wireless Rechargeable Sensor Network (WRSN) becomes a hot research issue in recent years owing to the breakthrough of wireless power transfer technology. Most prior arts concentrate on developing scheduling schemes in 2-D networks where mobile chargers are placed on the ground. However, few of them are suitable for 3-D scenarios, making it difficult or even impossible to popularize in practical applications. In this paper, we focus on the problem of charging a 3-D WRSN with an Unmanned Aerial Vehicle (UAV) to maximize charged energy within energy constraints. To deal with the problem, we propose a spatial discretization scheme to obtain a finite feasible charging spot set for UAV in 3-D environment and a temporal discretization scheme to determine charging duration for each charging spot. Then, we transform the problem into a submodular maximization problem with routing constraints, and present a cost-efficient approximation algorithm with a provable approximation ratio of e-1/4e(1-ε) to solve it. Lastly, extensive simulations and test-bed experiments show the superior performance of our algorithm.

### 5. Data Collecting and Energy Charging Oriented Mobile Path Design for Rechargeable Wireless Sensor Networks

- 作者：Meiyan Zhang, Wenyu Cai
- 日期：2022-04-08
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1155/2022/5004507；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：17.50（标题命中：charging, energy, mobile, rechargeable, sensor；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://doi.org/10.1155/2022/5004507
- PDF：https://downloads.hindawi.com/journals/js/2022/5004507.pdf
- 本地 PDF：未下载

Energy efficiency is one of the most important concerns in wireless sensor networks (WSNs). As far as we know, almost all energy efficiency researches of WSNs focus on energy conservation in some respects such as wireless data transmission and minimal data collection. Recently, wireless energy transfer has been a promising technology to prolong the lifetime of microsensor nodes, and so the traditional WSNs can be extended to rechargeable WSNs. Rechargeable WSNs is a new type of wireless sensor networks, where each sensor node can replenish energy through wireless charging. For rechargeable WSNs, it is powered by reusable energy or harvested energy, so the energy efficiency problem can be completely solved. Furthermore, mobile data collection has been well recognized to have significant advantages over sensory data collection manner using static sinks. In this paper, by employing one or multiple recharging sinks to replenish energy for sensor nodes and collect sensory data concurrently, we propose a novel wireless charging and mobile data collecting method based on self-organizing map (SOM) unsupervised learning for rechargeable WSNs. In other words, the sink mobility and energy replenishment are jointly considered in this paper. Finally, we evaluate the performance of the proposed algorithms through software simulation. Extensive results verify that the performance of the proposed algorithm can reduce the travel cost of mobile sink and improve the residual energy level for sensor nodes. As a results, it is very promising in the field of data acquisition in wireless sensor networks.

### 6. An efficient charging scheme using battery constrained mobile charger in wireless rechargeable sensor networks: R. Das et al.

- 作者：R Das, D Dash, CBK Yadav
- 日期：2022
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1007/s11235-022-00951-w；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：17.50（标题命中：charger, charging, mobile, rechargeable, sensor；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://link.springer.com/article/10.1007/s11235-022-00951-w
- PDF：—
- 本地 PDF：未下载

… The objective of the work is to optimize charging path, flow routing, and sojourn time. … is to optimize charging schedule and thereby minimizing the charging time and increase the …

### 7. Design of self-sustainable wireless sensor networks with energy harvesting and wireless charging

- 作者：P Zhou, C Wang, Y Yang
- 日期：2021
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1145/3459081；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：15.75（标题命中：charging, energy, harvesting, sensor；摘要命中：charger, mobile）
- 命中主题：mobile charger scheduling
- 页面：https://dl.acm.org/doi/abs/10.1145/3459081
- PDF：https://dl.acm.org/doi/pdf/10.1145/3459081
- 本地 PDF：未下载

… , wireless charging is a … the planning problems of sensors with various types and distributed energy storage powered by environmental energy. Then we schedule the Mobile Chargers (…

### 8. MobiCharger: Optimal scheduling for cooperative EV-to-EV dynamic wireless charging

- 作者：L Yan, H Shen, L Kang, J Zhao…
- 日期：2022
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：15.50（标题命中：charger, charging, dynamic, scheduling；摘要命中：mobile, sensor；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://ieeexplore.ieee.org/iel7/7755/4358975/09864082.pdf
- PDF：https://liyan2015.github.io/papers/conference/mobicharge_poster.pdf
- 本地 PDF：未下载

… trajectory of each EV, current road traffic state and charging station availability in the route planning … Zhu, “Evaluating the ondemand mobile charging in wireless sensor networks,” IEEE …

### 9. Wireless mobile charger excursion optimization algorithm in wireless rechargeable sensor networks

- 作者：S Malebary
- 日期：2020
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：15.00（标题命中：charger, mobile, rechargeable, sensor；摘要命中：charging, scheduling）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/9123876/
- PDF：—
- 本地 PDF：未下载

… Moreover, a new routing protocol has been designed based on the charging … -time charging scheduling algorithm for ondemand architecture in wireless rechargeable sensor networks,” …

### 10. Mobile charging strategy for wireless rechargeable sensor networks

- 作者：TS Chen, JJ Chen, XY Gao, TC Chen
- 日期：2022
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：14.75（标题命中：charging, mobile, rechargeable, sensor；摘要命中：scheduling；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://www.mdpi.com/1424-8220/22/1/359
- PDF：—
- 本地 PDF：未下载

… Wang, Li, Ye, and Yang solved the recharge scheduling problem with considering the … routes, finally, the MR can move to designated routes to wirelessly charge the sensor along …

### 11. DMCP: A distributed mobile charging protocol in wireless rechargeable sensor networks

- 作者：A Kaswan, PK Jana, M Dash, A Kumar…
- 日期：2022
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1145/3526090；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：14.50（标题命中：charging, mobile, rechargeable, sensor；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://dl.acm.org/doi/abs/10.1145/3526090
- PDF：https://dl.acm.org/doi/pdf/10.1145/3526090
- 本地 PDF：未下载

… in large-scale wireless rechargeable sensor networks (WRSNs). … (MCs) to schedule charging collaboratively and efficiently … its charging request directly to the BS via multi-hop routing if it …

### 12. Charging Oriented Sensor Placement and Flexible Scheduling in Rechargeable WSNs

- 作者：Tao Wu, Panlong Yang, Haipeng Dai, Wanru Xu, Mingxue Xu
- 日期：2019-04-01
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1109/infocom.2019.8737502；arXiv：—
- 开放获取：False；许可：未提供
- 相关度分数：14.50（标题命中：charging, rechargeable, scheduling, sensor；摘要命中：energy）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://doi.org/10.1109/infocom.2019.8737502
- PDF：—
- 本地 PDF：未下载

The recent breakthrough in Wireless Power Transfer (WPT) provides a promising way to support rechargeable sensors to enrich a series of energy-consuming applications. Unfortunately, two major design restrictions hinder the applicability of rechargeable sensor networks. First, most of the sensor placement schemes are focusing on the sensing tasks instead of the charging utility, which leaves a considerably high performance gap towards the optimal result. Second, the charging scheduling is non-flexible, where full or nothing charging policy suffers from the relatively low charging coverage as well as efficiency. In this paper, we focus on how to efficiently improve the charging utility when introducing charging oriented sensor placement and flexible scheduling policy. To this end, we jointly consider optimizing node positions and charging allocations. In particular, we formulate a general convex optimization problem under a general routing constraint, which generates great difficulty. We utilize area partition and charging discretization methods to reformulate a submodular function maximization problem. Thus a constant approximation algorithm is delivered to construct a near optimal charging tour. To this end, we analyze the performance loss from the discretization to guarantee that the output of the proposed algorithm has more than $(1 -\varepsilon)/4 (1 - 1 /e)$ of the optimal solution, where $\varepsilon$ is an arbitrarily small positive parameter $(0 \leq \varepsilon \leq 1)$. Both simulations and field experiments are conducted to evaluate the performance of our proposed algorithm.

### 13. Maintaining Large-Scale Rechargeable Sensor Networks Perpetually via Multiple Mobile Charging Vehicles

- 作者：Weifa Liang, Wenzheng Xu, Xiaojiang Ren, Xiaohua Jia, Xiaola Lin
- 日期：2016-05-02
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1145/2898357；arXiv：—
- 开放获取：False；许可：未提供
- 相关度分数：14.50（标题命中：charging, mobile, rechargeable, sensor；摘要命中：energy, scheduling）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://doi.org/10.1145/2898357
- PDF：—
- 本地 PDF：未下载

Wireless energy transfer technology based on magnetic resonant coupling has been emerging as a promising technology for wireless sensor networks (WSNs) by providing controllable yet perpetual energy to sensors. In this article, we study the deployment of the minimum number of mobile charging vehicles to charge sensors in a large-scale WSN so that none of the sensors will run out of energy, for which we first advocate a flexible on-demand charging paradigm that decouples sensor energy charging scheduling from the design of sensing data routing protocols. We then formulate a novel optimization problem of scheduling mobile charging vehicles to charge life-critical sensors in the network with an objective to minimize the number of mobile charging vehicles deployed, subject to the energy capacity constraint on each mobile charging vehicle. As the problem is NP-hard, we instead propose an approximation algorithm with a provable performance guarantee if the energy consumption of each sensor during each charging tour is negligible. Otherwise, we devise a heuristic algorithm by modifying the proposed approximation algorithm. We finally evaluate the performance of the proposed algorithms through experimental simulations. Experimental results demonstrate that the proposed algorithms are very promising, and the solutions obtained are fractional of the optimal ones. To the best of our knowledge, this is the first approximation algorithm with a nontrivial approximation ratio for a novel scheduling problem of multiple mobile charging vehicles for charging sensors.

### 14. Joint charging tour planning and depot positioning for wireless sensor networks using mobile chargers

- 作者：G Jiang, SK Lam, Y Sun, L Tu…
- 日期：2017
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：14.25（标题命中：charger, charging, mobile, sensor；摘要命中：rechargeable, scheduling）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/7889006/
- PDF：https://www.tum-create.edu.sg/sites/default/files/files/jiang2017jointSMAT.pdf
- 本地 PDF：未下载

… in order to maximize network utility for staticrouting rechargeable sensor networks. The work … ) scheduling mechanism for rechargeable sensor networks using a MC with given charging …

### 15. Energy-saving traffic scheduling in hybrid software defined wireless rechargeable sensor networks

- 作者：Y Wei, X Ma, N Yang, Y Chen
- 日期：2017
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：14.25（标题命中：energy, rechargeable, scheduling, sensor；摘要命中：charger, charging）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://www.mdpi.com/1424-8220/17/9/2126
- PDF：—
- 本地 PDF：未下载

… joint Power COntrol and Routing (PCOR) scheme for rechargeable sensor networks. Liu et al. [… in normal wireless sensor networks, in the effective charging scope of wireless chargers, …

### 16. TADP: Enabling temporal and distantial priority scheduling for on-demand charging architecture in wireless rechargeable sensor networks

- 作者：C Lin, Z Wang, D Han, Y Wu, CW Yu, G Wu
- 日期：2016
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：14.00（标题命中：charging, rechargeable, scheduling, sensor；摘要命中：charger, mobile）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://www.sciencedirect.com/science/article/pii/S1383762116300285
- PDF：https://www.researchgate.net/profile/Chi-Lin-4/publication/309731485_1-s20-S1383762116300285-main/links/58203cd508ae40da2cb4d7f0/1-s20-S1383762116300285-main.pdf
- 本地 PDF：未下载

… charging scheduling algorithm (TADP), which takes both the distance between nodes and the mobile charger and the arrival time of charging … use a rechargeable awareness routing …

### 17. Energy-efficient dynamic offloading and resource scheduling in mobile cloud computing

- 作者：Songtao Guo, Bin Xiao, Yuanyuan Yang, Yang Yang
- 日期：2016-04-01
- 来源：openalex, serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1109/infocom.2016.7524497；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：14.00（标题命中：dynamic, energy, mobile, scheduling；多源交叉命中）
- 命中主题：dynamic wireless charging, wireless power transfer scheduling
- 页面：http://hdl.handle.net/10397/105714
- PDF：http://ira.lib.polyu.edu.hk/bitstream/10397/105714/1/Xiao_Energy-Efficient_Dynamic_Offloading.pdf
- 本地 PDF：未下载

Mobile cloud computing (MCC) as an emerging and prospective computing paradigm, can significantly enhance computation capability and save energy of smart mobile devices (SMDs) by offloading computation-intensive tasks from resource-constrained SMDs onto the resource-rich cloud. However, how to achieve energy-efficient computation offloading under the hard constraint for application completion time remains a challenge issue. To address such a challenge, in this paper, we provide an energy-efficient dynamic offloading and resource scheduling (eDors) policy to reduce energy consumption and shorten application completion time. We first formulate the eDors problem into the energy-efficiency cost (EEC) minimization problem while satisfying the task-dependency requirements and the completion time deadline constraint. To solve the optimization problem, we then propose a distributed eDors algorithm consisting of three subalgorithms of computation offloading selection, clock frequency control and transmission power allocation. More importantly, we find that the computation offloading selection depends on not only the computing workload of a task, but also the maximum completion time of its immediate predecessors and the clock frequency and transmission power of the mobile device. Finally, our experimental results in a real testbed demonstrate that the eDors algorithm can effectively reduce the EEC by optimally adjusting the CPU clock frequency of SMDs based on the dynamic voltage and frequency scaling (DVFS) technique in local computing, and adapting the transmission power for the wireless channel conditions in cloud computing.

### 18. Energy-efficient dynamic task offloading for energy harvesting mobile cloud computing

- 作者：Y Zhang, J He, S Guo
- 日期：2018
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：13.75（标题命中：dynamic, energy, harvesting, mobile；摘要命中：scheduling）
- 命中主题：dynamic wireless charging
- 页面：https://ieeexplore.ieee.org/abstract/document/8515736/
- PDF：—
- 本地 PDF：未下载

… simultaneous wireless information and power transfer (… the dynamic offloading and resource scheduling optimization … -convex optimization problem to an AO optimization problem. …

### 19. Towards sustainable economic zones with uav-assisted real-time energy replenishment in wireless rechargeable sensor networks

- 作者：R Goyal, A Tomar, G Singal
- 日期：2023
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：13.50（标题命中：energy, rechargeable, sensor；摘要命中：charger, charging, mobile；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://ieeexplore.ieee.org/abstract/document/10469062/
- PDF：—
- 本地 PDF：未下载

… of charging station deployment on minimizing UAV trajectories … design of mobile chargers in wireless sensor networks,” Wireless … path planning algorithm for mobile charger in wireless …

### 20. Reducing sensor failure and ensuring scheduling fairness for online charging in heterogeneous rechargeable sensor networks

- 作者：J Wu, S Li, Q Huang
- 日期：2020
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：13.50（标题命中：charging, rechargeable, scheduling, sensor）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/9219569/
- PDF：—
- 本地 PDF：未下载

… wireless rechargeable sensor networks as a triple (N, BS, MC), as shown in Fig. 1. N is the set of sensor … Qiao: J-RoC: A joint routing and charging scheme to prolong sensor network …

### 21. Fair Energy Division Scheme to Permanentize the Network Operation for Wireless Rechargeable Sensor Networks

- 作者：Alaa Omran Almagrabi
- 日期：2020-01-01
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1109/access.2020.3027615；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：13.25（标题命中：energy, rechargeable, sensor；摘要命中：charger, charging, mobile）
- 命中主题：mobile charger scheduling
- 页面：https://doi.org/10.1109/access.2020.3027615
- PDF：https://ieeexplore.ieee.org/ielx7/6287639/8948470/09208649.pdf
- 本地 PDF：未下载

In the past years, the energy constraint problem is known as a design issue of Wireless Sensor Networks (WSNs) due to equipping the sensor nodes with limited power supplies. During the last few years, Wireless Rechargeable Sensor Networks (WRSNs) have gained researchers attention. In a WRSN, the sensor nodes are equipped with RF circuits, which enables them to receive energy from Wireless Mobile Chargers (WMC). However, most of the existing wireless charging algorithms consider the unlimited power budget for WMCs, which is the opposite of feasibility of a real network environment. Likewise, most of the previous works fail to take full advantage of WMC as it starts recharging the nodes when their energy level reaches a threshold, which leads to increasing the inactive time of WMC. Moreover, although previous works employed WMCs, the network lifetime is limited. However, optimal division of the energy of WMC among nodes can guarantee the perpetual network operation. Therefore, proposing an efficient method that jointly solves these challenges is required. In this paper, a new Fair Energy Division Scheme (FEDS) is presented, which undertakes the permanent network operation by optimizing the energy division at the beginning of each cycle. Simulation results exhibit that FEDS achieves perpetual network lifetime. In addition, the proposed scheme improves energy efficiency, (25%) compared to Uneven Cluster-based Mobile Charging (UCMC) and (75%) compared to Nearest-Job-Next Preemption NJNP; travelling time of WMC, (50%) compared to UCMC and (75%) compared to NJNP. In conclusion, the proposed protocol significantly improves network.

### 22. Scheduling status updates to minimize age of information with an energy harvesting sensor

- 作者：BT Bacinoglu, E Uysal-Biyikoglu
- 日期：2017
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：1701.08354
- 开放获取：True；许可：未提供
- 相关度分数：13.25（标题命中：energy, harvesting, scheduling, sensor）
- 命中主题：RF energy harvesting scheduling
- 页面：https://ieeexplore.ieee.org/abstract/document/8006703/
- PDF：https://arxiv.org/pdf/1701.08354
- 本地 PDF：未下载

… As suggested in [15], we consider update policies of thresholdtype which are optimal in continuous time for the problem of minimizing average age under stationary energy harvesting …

### 23. A novel approach for path plan of mobile chargers in wireless rechargeable sensor networks

- 作者：F Chen, Z Zhao, G Min, Y Wu
- 日期：2016
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：13.25（标题命中：charger, mobile, rechargeable, sensor；摘要命中：charging）
- 命中主题：mobile charger scheduling
- 页面：https://ieeexplore.ieee.org/abstract/document/7950212/
- PDF：—
- 本地 PDF：未下载

… R-MQCSP [11] jointly schedules the charging time at each node and the node … of path plan for mobile chargers in wireless rechargeable sensor networks. Aiming to optimize the …

### 24. Powering rechargeable sensor networks: Sustainable energy solution for smart healthcare monitoring

- 作者：R Goyal, A Tomar
- 日期：2024
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：13.00（标题命中：energy, rechargeable, sensor；摘要命中：charging, scheduling；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/10882076/
- PDF：—
- 本地 PDF：未下载

… A comparative study of wireless sensor networks and their routing … charging scheduling and path planning scheme for multiple mcenabled on-demand wireless rechargeable sensor net…

### 25. Trajectory Optimization of Laser-Charged UAVs for Charging Wireless Rechargeable Sensor Networks

- 作者：Ning Liu, Chuanwen Luo, Jia Cao, Yi Hong, Zhibo Chen
- 日期：2022-11-27
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.3390/s22239215；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：13.00（标题命中：charging, rechargeable, sensor；摘要命中：energy, mobile；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://doi.org/10.3390/s22239215
- PDF：https://www.mdpi.com/1424-8220/22/23/9215/pdf?version=1669794761
- 本地 PDF：未下载

This paper considers a laser-powered unmanned aerial vehicle (UAV)-enabled wireless power transfer (WPT) system. In the system, a UAV is dispatched as an energy transmitter to replenish energy for battery-limited sensors in a wireless rechargeable sensor network (WRSN) by transferring radio frequency (RF) signals, and a mobile unmanned vehicle (MUV)-loaded laser transmitter travels on a fixed path to charge the on-board energy-limited UAV when it arrives just below the UAV. Based on the system, we investigate the trajectory optimization of laser-charged UAVs for charging WRSNs (TOLC problem), which aims to optimize the flight trajectories of a UAV and the travel plans of an MUV cooperatively to minimize the total working time of the UAV so that the energy of every sensor is greater than or equal to the threshold. Then, we prove that the problem is NP-hard. To solve the TOLC problem, we first propose the weighted centered minimum coverage (WCMC) algorithm to cluster the sensors and compute the weighted center of each cluster. Based on the WCMC algorithm, we propose the TOLC algorithm (TOLCA) to design the detailed flight trajectory of a UAV and the travel plans of an MUV, which consists of the flight trajectory of a UAV, the hovering points of a UAV with the corresponding hovering times used for the charging sensors, the hovering points of a UAV with the corresponding hovering times used for replenishing energy itself, and the hovering times of a UAV waiting for an MUV. Numerical results are provided to verify that the suggested strategy provides an effective method for supplying wireless rechargeable sensor networks with sustainable energy.

### 26. Efficient Energy Harvesting in Wireless Sensor Networks of Smart Grid

- 作者：Uthman Baroudi, Ahmad Shawahna, Md. Enamul Haque
- 日期：2019-11-02
- 来源：arxiv
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：1911.07621
- 开放获取：True；许可：未提供
- 相关度分数：13.00（标题命中：energy, harvesting, sensor；摘要命中：charger, charging, mobile）
- 命中主题：mobile charger scheduling
- 页面：https://arxiv.org/abs/1911.07621v1
- PDF：https://arxiv.org/pdf/1911.07621v1
- 本地 PDF：未下载

Smart grids are becoming ubiquitous in recent time. With the progress of automation in this arena, it needs to be diagnosed for better performance and less failures. There are several options for doing that but we have seen from the past research that using Wireless Sensor Network (WSN) as the diagnosis framework would be the most promising option due to its diverse benefits. Several challenges such as effect of noise, lower speed, selective node replacement, complexity of logistics, and limited battery lifetime arise while using WSN as the framework. Limited battery lifetime has become one of the most significant issues to focus on to get rid of it. This article provides a model for replenishing the battery charge of the sensor nodes of wireless sensor network. We will use the model for sensor battery recharging in an efficient way so that no nodes become out of service after a while. We will be using mobile charger for this purpose. So, there may be some scope for improving the recharge interval for the mobile charger as well. This will be satisfied using optimum path calculation for each time the charger travels to the nodes. Our main objectives are to maximize the nodes battery utilization, distribute power effectively from the energy harvester, and minimize the distance between power source and cluster head. The simulation results show that the proposed approach successfully maximizes the utilization of the nodes battery while minimizes the waiting time for the sensor nodes to get recharged from the energy harvester.

### 27. RCSS: A real-time on-demand charging scheduling scheme for wireless rechargeable sensor networks

- 作者：P Zhong, Y Zhang, S Ma, X Kui, J Gao
- 日期：2018
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：13.00（标题命中：charging, rechargeable, scheduling, sensor）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://www.mdpi.com/1424-8220/18/5/1601
- PDF：—
- 本地 PDF：未下载

… on-demand charging scheduling works decided the node charging … deployment and routing. In Proceedings of the IEEE 30th … station in a rechargeable sensor network. In Proceedings of …

### 28. An optimal cluster formation based energy efficient dynamic scheduling hybrid MAC protocol for heavy traffic load in wireless sensor networks

- 作者：V Sundararaj, S Muthukumar, RS Kumar
- 日期：2018
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：13.00（标题命中：dynamic, energy, scheduling, sensor）
- 命中主题：dynamic wireless charging
- 页面：https://www.sciencedirect.com/science/article/pii/S0167404818303754
- PDF：—
- 本地 PDF：未下载

… Energy effective dynamic arranging procedure for traffic adaptive wireless sensor networks. At … -aware clusters by optimal selection of cluster heads. We utilize a dynamic time slot task …

### 29. Optimal charging scheduling using nature inspired metaheuristic algorithm in wireless sensor networks

- 作者：CBK Yadav, A Alam, AK Singh
- 日期：2026
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1201/9781003654483-99；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：12.50（标题命中：charging, scheduling, sensor；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://api.taylorfrancis.com/content/chapters/edit/download?identifierName=doi&identifierValue=10.1201/9781003654483-99&type=chapterpdf
- PDF：—
- 本地 PDF：未下载

… This simple but powerful technique helps us squeeze extra efficiency out of our charging routes after the main algorithm has done its work. In this work, we use some predefined limit …

### 30. TLFW: A Three-layer Framework in Wireless Rechargeable Sensor Network with a Mobile Base Station

- 作者：Anwen Wang, Xianjia Meng, Lvju Wang, Xiang Ji, Hao Chen, Baoying Liu, Feng Chen, Yajuan Du, Guangcheng Yin
- 日期：2020-02-21
- 来源：arxiv
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：2002.11047
- 开放获取：True；许可：未提供
- 相关度分数：12.50（标题命中：mobile, rechargeable, sensor；摘要命中：energy, scheduling）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://arxiv.org/abs/2002.11047v1
- PDF：https://arxiv.org/pdf/2002.11047v1
- 本地 PDF：未下载

Wireless sensor networks as the base support for the Internet of things has been a large number of popularity and application. Such as intelligent agriculture, we have to use the sensor network to obtain the growth environmental data of crops, etc.. However, the difficulty of power supply of wireless nodes has seriously hindered the application and development of Internet of things. In order to solve this problem, people use low-power, sleep scheduling and other energy-saving methods on the nodes. Although these methods can prolong the working time of nodes, they will eventually become invalid because of the exhaustion of energy. The use of solar energy, wind energy, and wireless signals in the environment to obtain energy is another way to solve the energy problem of nodes. However, these methods are affected by weather, environment and other factors, and are unstable. Thus, the discontinuity work of the node is caused. In recent years, the development of wireless power transfer (WPT) has brought another solution to this problem. In this paper, a three-layer framework is proposed for mobile station data collection in rechargeable wireless sensor networks to keep the node running forever, named TLFW which includes the sensor layer, cluster head layer, and mobile station layer. And the framework can minimize the total energy consumption of the system. The simulation results show that the scheme can reduce the energy consumption of the entire system, compared with a Mobile Station in a Rechargeable Sensor Network(MSiRSN).

### 31. Efficient Wireless Charging Pad Deployment in Wireless Rechargeable Sensor Networks

- 作者：Jingjing Chen, Chang Wu Yu, Wen Ouyang
- 日期：2020-01-01
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1109/access.2020.2975635；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：12.50（标题命中：charging, rechargeable, sensor；摘要命中：energy, scheduling）
- 命中主题：mobile charger scheduling, wireless rechargeable sensor networks
- 页面：https://doi.org/10.1109/access.2020.2975635
- PDF：https://ieeexplore.ieee.org/ielx7/6287639/8948470/09006885.pdf
- 本地 PDF：未下载

The rapid development of wireless power transfer technology brings forth innovative vehicle energy solutions and breakthroughs utilizing wireless sensor networks (WSNs). In most existing schemes, wireless rechargeable sensor networks (WRSNs) are generally equipped with one or more wireless charging vehicles (vehicles) to serve sensor nodes (SNs). These schemes solve the energy issue to some extent; however, due to off-road and speed limitations of vehicles, some SNs still cannot be charged in time, negatively affecting the lifetime of the networks. Our work proposes a new WRSN model equipped with one wireless charging drone (drone) with a constrained flight distance coupled with several wireless charging pads (pads) deployed to charge the drone when it cannot reach the subsequent stop. Our model solves this charging issues effectively and overcomes energy capacity limitations of the drone. Thus, a wireless charging pad deployment problem is formulated, which aims to apply the minimum number of pads so that at least one feasible routing path can be established for the drone to reach every SN in a given WRSN. Four feasible heuristics, three based on graph theory and one on geometry, are proposed for this problem. In addition, a novel drone scheduling algorithm, the shortest multi-hop path algorithm, is developed for the drone to serve charging requests with the assistance of pads. We examine the proposed schemes through extensive simulations. The results compare and demonstrate the effectiveness of the proposed schemes in terms of network density, region size and maximum flight distance.

### 32. Practical Mission Planning for Optimized UAV-Sensor Wireless Recharging

- 作者：Qiuchen Qian, James O'Keeffe, Yanran Wang, David Boyle
- 日期：2022-03-09
- 来源：arxiv
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：2203.04595
- 开放获取：True；许可：未提供
- 相关度分数：12.25（标题命中：charging, sensor；摘要命中：charger, energy, mobile, rechargeable, scheduling；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://arxiv.org/abs/2203.04595v2
- PDF：https://arxiv.org/pdf/2203.04595v2
- 本地 PDF：未下载

Optimal maintenance of sensor nodes in a Wireless Rechargeable Sensor Network (WRSN) requires effective scheduling of power delivery vehicles by solving the Charging Scheduling Problem (CSP). Deploying Unmanned Aerial Vehicles (UAVs) as mobile chargers has emerged as a promising solution due to their mobility and flexibility. The CSP can be formulated as a Mixed-Integer Non-Linear Programming problem whose optimization objective is maximizing the recharged energy of sensor nodes within the UAV battery constraint. While many studies have demonstrated satisfactory performance of heuristic algorithms in addressing specific routing problems, few studies explore online updating (i.e., mission re-planning `on the fly') in the CSP context. Here we present a new offline and online mission planner leveraging a first-principles power consumption model that uses real-time state information and environmental information. The planner, namely Rapid Online Metaheuristic-based Planner (ROMP), supplements solutions from a Guided Local Search (GLS) with our Context-aware Black Hole Algorithm. Our results demonstrate that ROMP outperforms GLS in most cases tested. We developed and proposed FastROMP to speed up the online mission (re-)planning algorithm by introducing a new online adjustment operator that uses the latest state information as input, eliminating the need for re-initialization. FastROMP not only provides a better quality route, but it also significantly reduces computational time. The reduction ranges from 39.57% in sparse deployment to 93.3% in denser deployments.

### 33. Minimizing Age of Information in Multihop Energy-Harvesting Wireless Sensor Network

- 作者：Kunyi Chen, Fatma Benkhelifa, Hong Gao, Julie A. McCann, Jianzhong Li
- 日期：2022-08-09
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1109/jiot.2022.3197428；arXiv：—
- 开放获取：True；许可：other-oa
- 相关度分数：12.25（标题命中：energy, harvesting, sensor；摘要命中：scheduling；近年文献）
- 命中主题：RF energy harvesting scheduling
- 页面：https://pureportal.coventry.ac.uk/en/publications/minimizing-age-of-information-in-multihop-energyharvesting-wireless-sensor-network(428e9435-e838-482f-9383-6d67ab96ea19).html
- PDF：https://pure.coventry.ac.uk/ws/files/56402495/Binder13.pdf
- 本地 PDF：未下载

Age of Information (AoI), a metric measuring the information freshness, has drawn increased attention due to its importance in monitoring applications in which nodes send timestamped status updates to interested recipients, and timely updates about phenomena are important. In this work, we consider the AoI minimization scheduling problem in multihop energy harvesting (EH) wireless sensor networks (WSNs). We design the generation time of updates for nodes and develop transmission schedules under both protocol and physical interference models, aiming at achieving minimum peak AoI and average AoI among all nodes for a given time duration. We prove that it is an NP-Hard problem and propose an energy-adaptive, distributed algorithm called the minimizing AoI scheduling algorithm for general network (MAoIG). We derive its theoretical upper bounds for the peak and average AoI and a lower bound for peak AoI. The numerical results validate that MAoIG outperforms all of the baseline schemes in all scenarios and that the experimental results tightly track the theoretical upper bound optimal solutions while the lower bound tightness decreases with the number of nodes.

### 34. Charging strategy and scheduling algorithm for directional wireless power transfer in WRSNs

- 作者：Yang Yu, Cheng Qin
- 日期：2022-02-14
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1016/j.aej.2022.01.060；arXiv：—
- 开放获取：True；许可：cc-by-nc-nd
- 相关度分数：12.25（标题命中：charging, scheduling；摘要命中：charger, energy, mobile, rechargeable, sensor；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://doi.org/10.1016/j.aej.2022.01.060
- PDF：—
- 本地 PDF：未下载

Wireless Power Transfer (WPT) technology plays a significant role to prolong the lifetime of wireless rechargeable sensor networks (WRSNs). To achieve stable and reliable energy supplements via wireless charging, the optimization of the trajectory of mobile chargers is crucial. In this paper, a charging strategy and scheduling algorithm for directional wireless power transfer in WRSNs is proposed. Firstly, the charging demand degree is defined to determine the priority of charging requests. Then, to avoid the occurrence of node's energy exhausted, the charger's orientation angle selection algorithm based on charging priority is designed. Finally, we formulate the problem of directional charger's deployment into discrete unit disk cover problem and propose a moving path planning scheme based on improved Genetic Algorithm to optimize the energy charging efficiency. Simulation results illustrate the benefit of our proposed scheme over the benchmark.

### 35. An adaptive charging scheme for large-scale wireless rechargeable sensor networks inspired by deep Q-network

- 作者：AD Vuong, HT Tran, HNQ Pham, QM Bui…
- 日期：2024
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1007/s00521-024-09658-2；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：12.00（标题命中：charging, rechargeable, sensor；近年文献）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://link.springer.com/article/10.1007/s00521-024-09658-2
- PDF：—
- 本地 PDF：未下载

… model for the charging schedule optimization problem. … ’s charging schedule can be represented as a sequence of locations (CL or BS). Thus, we can split up the charging schedule into …

### 36. UAV-Assisted Cooperative Charging and Data Collection Strategy for Heterogeneous Wireless Sensor Networks

- 作者：Y Xin, L Li, Y Ning, Y Yang, P Shi
- 日期：2025
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：11.75（标题命中：charging, sensor；摘要命中：charger, energy, mobile, scheduling；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://www.mdpi.com/2504-446X/9/12/859
- PDF：—
- 本地 PDF：未下载

… studies on mobile chargers primarily addressed path planning and scheduling issues. … and energy constraints, paper [29] proposed an optimized UAV trajectory design to overcome …

### 37. Energy-Efficient Control with Harvesting Predictions for Solar-Powered Wireless Sensor Networks

- 作者：Tengyue Zou, Shouying Lin, Qijie Feng, Yanlian Chen
- 日期：2016-01-04
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.3390/s16010053；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：11.50（标题命中：energy, harvesting, sensor；摘要命中：rechargeable, scheduling）
- 命中主题：RF energy harvesting scheduling
- 页面：https://doi.org/10.3390/s16010053
- PDF：https://www.mdpi.com/1424-8220/16/1/53/pdf?version=1451982309
- 本地 PDF：未下载

Wireless sensor networks equipped with rechargeable batteries are useful for outdoor environmental monitoring. However, the severe energy constraints of the sensor nodes present major challenges for long-term applications. To achieve sustainability, solar cells can be used to acquire energy from the environment. Unfortunately, the energy supplied by the harvesting system is generally intermittent and considerably influenced by the weather. To improve the energy efficiency and extend the lifetime of the networks, we propose algorithms for harvested energy prediction using environmental shadow detection. Thus, the sensor nodes can adjust their scheduling plans accordingly to best suit their energy production and residual battery levels. Furthermore, we introduce clustering and routing selection methods to optimize the data transmission, and a Bayesian network is used for warning notifications of bottlenecks along the path. The entire system is implemented on a real-time Texas Instruments CC2530 embedded platform, and the experimental results indicate that these mechanisms sustain the networks' activities in an uninterrupted and efficient manner.

### 38. A novel energy optimization framework to enhance the performance of sensor nodes in Industry 4.0

- 作者：S. Sivakumar, J. Logeshwaran, Raju Kannadasan, Muhammad Faheem, Dhanasekar Ravikumar
- 日期：2024-01-02
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1002/ese3.1657；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：11.25（标题命中：energy, sensor；摘要命中：dynamic, harvesting, scheduling；近年文献）
- 命中主题：RF energy harvesting scheduling
- 页面：https://doi.org/10.1002/ese3.1657
- PDF：https://onlinelibrary.wiley.com/doi/pdfdirect/10.1002/ese3.1657
- 本地 PDF：未下载

Abstract Industry 4.0 is a term used to refer to the fourth industrial revolution, characterized by the introduction of new technologies, such as the Internet of Things, Big Data, and artificial intelligence (AI). As the number of connected devices in industrial settings grows, energy optimization of such sensors becomes increasingly essential. This paper proposes an energy optimization framework for sensor nodes in Industry 4.0. The framework is based on energy efficiency, energy conservation, and energy harvesting principles. It is designed to optimize the energy consumption of sensor nodes while maintaining their performance. The framework includes dynamic power management, scheduling, and harvesting techniques to reduce energy consumption while maintaining performance. In addition, the framework provides a comprehensive approach to energy optimization, including advanced analytics and AI to predict energy consumption and optimize energy use. The proposed model reached 96.93% sensitivity, 91.36% false discovery rate, 11.28% false omission rate, 90.12% prevalence threshold, and 91.24% threat score. The proposed framework is expected to improve the performance of sensor nodes in Industry 4.0, enabling increased efficiency and cost savings.

### 39. Survey of Energy Harvesting Technologies for Wireless Sensor Networks

- 作者：Alexander Williams, Matheus F. Torquato, Ian Cameron, Ashraf Fahmy, Johann Sienz
- 日期：2021-01-01
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1109/access.2021.3083697；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：11.25（标题命中：energy, harvesting, sensor）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://doi.org/10.1109/access.2021.3083697
- PDF：https://ieeexplore.ieee.org/ielx7/6287639/9312710/09440395.pdf
- 本地 PDF：未下载

Energy harvesting (EH) technologies could lead to self-sustaining wireless sensor networks (WSNs) which are set to be a key technology in Industry 4.0. There are numerous methods for small-scale EH but these methods differ greatly in their environmental applicability, energy conversion characteristics, and physical form which makes choosing a suitable EH method for a particular WSN application challenging due to the specific application-dependency. Furthermore, the choice of EH technology is intrinsically linked to non-trivial decisions on energy storage technologies and combinatorial architectures for a given WSN application. In this paper we survey the current state of EH technology for small-scale WSNs in terms of EH methods, energy storage technologies, and EH system architectures for combining methods and storage including multi-source and multi-storage architectures, as well as highlighting a number of other optimisation considerations. This work is intended to provide an introduction to EH technologies in terms of their general working principle, application potential, and other implementation considerations with the aim of accelerating the development of sustainable WSN applications in industry.

### 40. An efficient combined charging strategy for large-scale wireless rechargeable sensor networks

- 作者：Y Dong, S Li, G Bao, C Wang
- 日期：2020
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：11.25（标题命中：charging, rechargeable, sensor；摘要命中：scheduling）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/9079536/
- PDF：—
- 本地 PDF：未下载

… scheduling scheme for wireless rechargeable sensor … and data gathering in wireless rechargeable sensor networks,” in … joint routing and charging scheme to prolong sensor network …

### 41. Deadline-Driven Multi-node Mobile Charging

- 作者：Xunpeng Rao, Panlong Yang, Haipeng Dai, Tao Wu, Hao Zhou, Jing Zhao, Linlin Chen, Peng-Jun Wan
- 日期：2018-10-29
- 来源：arxiv
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：1810.12385
- 开放获取：True；许可：未提供
- 相关度分数：11.25（标题命中：charging, mobile；摘要命中：charger, energy, rechargeable, scheduling, sensor）
- 命中主题：mobile charger scheduling, wireless rechargeable sensor networks
- 页面：https://arxiv.org/abs/1810.12385v1
- PDF：https://arxiv.org/pdf/1810.12385v1
- 本地 PDF：未下载

Due to the merit without requiring charging cable, wireless power transfer technologies have drawn rising attention as a new method to replenish energy to Wireless Rechargeable Sensor Networks (WRSNs). In this paper, we study mobile charger scheduling problem for multi-node recharging with deadline-series. Our target is to maximize the overall effective charging utility, and minimize the traveling time as well. Instead of charging only once over a scheduling cycle, we incorporate the multiple charging strategy for multi-node charging with deadline constraint, where charging spots and tour are jointly optimized. Specifically, we formulate the effective charging utility maximization problem as to maximize a monotone submodular function subject to a partition matroid constraint, and propose a simple but effective 1/2 -approximation greedy algorithm. After that, we present the grid-based skip-substitute operation further to save the traveling time, which can increase the charging utility. Finally, we conduct the evaluation for the performance of our scheduling scheme. Comparing to the Early Deadline First scheme, the simulation and field experiment results show that our algorithm outperform EDF by 37.5% and 37.9%, respectively.

### 42. Minimizing Age of Information in Energy Harvesting Wireless Sensor Networks

- 作者：Naoya Hirosawa, Hiroki Iimori, Koji Ishibashi, Giuseppe Thadeu Freitas de Abreu
- 日期：2020-01-01
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1109/access.2020.3038954；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：11.00（标题命中：energy, harvesting, sensor）
- 命中主题：RF energy harvesting scheduling
- 页面：https://doi.org/10.1109/access.2020.3038954
- PDF：https://ieeexplore.ieee.org/ielx7/6287639/8948470/09262853.pdf
- 本地 PDF：未下载

We consider the uplink of an energy harvesting (EH) wireless sensor network (WSN) where N single-antenna sensors communicate with a common fusion center (FC) with the aim of cooperatively minimizing the overall average age of information (AoI). Specifically, we propose new resource allocation algorithms to minimize the average AoI in an EH-WSNs employing common multiple-access schemes, in particular time-division multiple access (TDMA) and frequency-division multiple access (FDMA). To this end, we take advantage of the convexity of the derived AoI, enabling an optimal resource block assignment, implemented as a greedy algorithm for TDMA systems and in the form of an alternating direction method of multipliers (ADMM) scheme for FDMA systems. The optimality of the greedy resource allocation scheme derived for the TDMA case is obtained by design, whereas that of the ADMM-based method derived for the FDMA case is demonstrated numerically. Simulation results indicate that the choice between TDMA or FDMA depends on the available resources, size of the data packet, and the time of packet observation in the system.

### 43. A General Framework for Charger Scheduling Optimization Problems

- 作者：Xuan Li, Miao Jin
- 日期：2020-09-28
- 来源：arxiv
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：2009.14428
- 开放获取：True；许可：未提供
- 相关度分数：11.00（标题命中：charger, scheduling；摘要命中：charging, mobile, rechargeable, sensor）
- 命中主题：mobile charger scheduling, wireless rechargeable sensor networks
- 页面：https://arxiv.org/abs/2009.14428v1
- PDF：https://arxiv.org/pdf/2009.14428v1
- 本地 PDF：未下载

This paper presents a general framework to tackle a diverse range of NP-hard charger scheduling problems, optimizing the trajectory of mobile chargers to prolong the life of Wireless Rechargeable Sensor Network (WRSN), a system consisting of sensors with rechargeable batteries and mobile chargers. Existing solutions to charger scheduling problems require problem-specific design and a trade-off between the solution quality and computing time. Instead, we observe that instances of the same type of charger scheduling problem are solved repeatedly with similar combinatorial structure but different data. We consider searching an optimal charger scheduling as a trial and error process, and the objective function of a charging optimization problem as reward, a scalar feedback signal for each search. We propose a deep reinforcement learning-based charger scheduling optimization framework. The biggest advantage of the framework is that a diverse range of domain-specific charger scheduling strategy can be learned automatically from previous experiences. A framework also simplifies the complexity of algorithm design for individual charger scheduling optimization problem. We pick three representative charger scheduling optimization problems, design algorithms based on the proposed deep reinforcement learning framework, implement them, and compare them with existing ones. Extensive simulation results show that our algorithms based on the proposed framework outperform all existing ones.

### 44. Optimize the Age of Useful Information in Edge-assisted Energy-harvesting Sensor Networks

- 作者：未提供
- 日期：未提供
- 来源：tavily
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1145/3640342；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：11.00（标题命中：energy, harvesting, sensor；摘要命中：mobile, scheduling）
- 命中主题：RF energy harvesting scheduling
- 页面：https://dl.acm.org/doi/10.1145/3640342
- PDF：—
- 本地 PDF：未下载

Google Scholar Wen-Zhan Song, Fenghua Yuan, and Richard LaHusen. 2006. Time-optimum packet scheduling for many-to-one routing in wireless sensor networks. In _IEEE International Conference on Mobile Ad Hoc and Sensor Systems_. IEEE, 81–90. Crossref Google Scholar Yin Sun, Igor Kadota, Rajat Talak, and Eytan Modiano. 2022. _Age of Information: A New Metric for Information Freshness_. Springer Nature. Google Scholar [...] Google Scholar Rajat Talak, Sertac Karaman, and Eytan Modiano. 2018. Optimizing age of information in wireless networks with perfect channel state information. In _16th International Symposium on Modeling and Optimization in Mobile, Ad Hoc, and Wireless Networks (WiOpt’18)_. IEEE, 1–8. Crossref Google Scholar [...] The first condition implies that each EH node in the interference set \(I(i,t)\) should be able to work at time slot _t_. The second condition means that if _i_ will transmit at time slot _t_, then the EH nodes in \(I(i,t)\) cannot transmit at _t_. The interference set records the EH nodes that are interfered by the transmission of _i_ at time slot _t_. Therefore, when scheduling an EH node to transmit, besides the representativeness of the transmitted sensory data, we should also consider the

### 45. Data-driven Spatial Classification using Multi-Arm Bandits for Monitoring with Energy-Constrained Mobile Robots

- 作者：Xiaoshan Lin, Siddharth Nayak, Stefano Di Cairano, Abraham P. Vinod
- 日期：2025-01-14
- 来源：arxiv
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：2501.08222
- 开放获取：True；许可：未提供
- 相关度分数：10.75（标题命中：energy, mobile；摘要命中：charging, sensor；近年文献）
- 命中主题：mobile charger scheduling
- 页面：https://arxiv.org/abs/2501.08222v2
- PDF：https://arxiv.org/pdf/2501.08222v2
- 本地 PDF：未下载

We consider the spatial classification problem for monitoring using data collected by a coordinated team of mobile robots. Such classification problems arise in several applications including search-and-rescue and precision agriculture. Specifically, we want to classify the regions of a search environment into interesting and uninteresting as quickly as possible using a team of mobile sensors and mobile charging stations. We develop a data-driven strategy that accommodates the noise in sensed data and the limited energy capacity of the sensors, and generates collision-free motion plans for the team. We propose a bi-level approach, where a high-level planner leverages a multi-armed bandit framework to determine the potential regions of interest for the drones to visit next based on the data collected online. Then, a low-level path planner based on integer programming coordinates the paths for the team to visit the determined regions subject to the physical constraints. We characterize several theoretical properties of the proposed approach, including anytime guarantees and task completion time. We show the efficacy of our approach in simulation, and further validate these observations in physical experiments using mobile robots.

### 46. Electric-vehicle energy management and charging scheduling system in sustainable cities and society

- 作者：KN Qureshi, A Alhudhaif, G Jeon
- 日期：2021
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：未知；许可：未提供
- 相关度分数：10.75（标题命中：charging, energy, scheduling）
- 命中主题：dynamic wireless charging
- 页面：https://www.sciencedirect.com/science/article/pii/S2210670721002766
- PDF：—
- 本地 PDF：未下载

… , are used in wireless power transfer to address the existing challenges. Wireless Power Transfer (WPT) technologies are introduced where the users wirelessly transferring energy to …

### 47. Capacity over capacitance for reliable energy harvesting sensors

- 作者：Neal Jackson, Joshua Adkins, Prabal Dutta
- 日期：2019-04-04
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1145/3302506.3310400；arXiv：—
- 开放获取：True；许可：cc-by
- 相关度分数：10.75（标题命中：energy, harvesting, sensor）
- 命中主题：RF energy harvesting scheduling
- 页面：https://doi.org/10.1145/3302506.3310400
- PDF：https://dl.acm.org/doi/pdf/10.1145/3302506.3310400
- 本地 PDF：未下载

Today, most sensors that harvest energy from indoor solar, ambient RF, or thermal gradients buffer small amounts of energy in capacitors as they intermittently work through a sensing task. While the utilization of capacitors for energy storage affords these systems indefinite lifetimes, their low energy capacity necessitates complex intermittent programming models for state retention and energy management. However, recent advances in battery technology lead us to reevaluate the impact that increased energy storage capacity may have on the necessity of these programming models and the reliability of energy harvesting sensors.

### 48. Periodic charging for wireless sensor networks with multiple portable chargers

- 作者：M Hu, Z Chen, K Peng, X Ma, P Zhou, J Liu
- 日期：2018
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：10.75（标题命中：charger, charging, sensor；摘要命中：scheduling）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://ieeexplore.ieee.org/abstract/document/8571229/
- PDF：https://ieeexplore.ieee.org/iel7/6287639/8600701/08571229.pdf
- 本地 PDF：未下载

… In this paper, we study both periodic charging time scheduling and charging path planning … We assume the network topology is comparable stable and the routing paths (or routing table…

### 49. Optimal power control, scheduling, and energy harvesting for wireless networked control systems

- 作者：G Karadag, MS Iqbal, S Coleri
- 日期：2020
- 来源：serpapi
- 筛选状态：`pending`；人工选择：`false`
- DOI：—；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：10.50（标题命中：energy, harvesting, scheduling）
- 命中主题：dynamic wireless charging
- 页面：https://ieeexplore.ieee.org/abstract/document/9284493/
- PDF：https://www.researchgate.net/profile/Muhammad-Shahid-Iqbal-4/publication/346785185_Optimal_Power_Control_Scheduling_and_Energy_Harvesting_for_Wireless_Networked_Control_Systems/links/654b19beb86a1d521bc2886b/Optimal-Power-Control-Scheduling-and-Energy-Harvesting-for-Wireless-Networked-Control-Systems.pdf
- 本地 PDF：未下载

… the optimal power control, energy harvesting and scheduling … power allocation problem is separable from the scheduling … provide the exact expression for optimal power control. The …

### 50. Energy-Harvesting Wireless Sensor Networks (EH-WSNs)

- 作者：Kofi Sarpong Adu‐Manu, Nadir Adam, Cristiano Tapparello, Hoda Ayatollahi, Wendi Heinzelman
- 日期：2018-04-27
- 来源：openalex
- 筛选状态：`pending`；人工选择：`false`
- DOI：10.1145/3183338；arXiv：—
- 开放获取：True；许可：未提供
- 相关度分数：10.50（标题命中：energy, harvesting, sensor）
- 命中主题：wireless rechargeable sensor networks
- 页面：https://doi.org/10.1145/3183338
- PDF：https://dl.acm.org/doi/pdf/10.1145/3183338
- 本地 PDF：未下载

Wireless Sensor Networks (WSNs) are crucial in supporting continuous environmental monitoring, where sensor nodes are deployed and must remain operational to collect and transfer data from the environment to a base-station. However, sensor nodes have limited energy in their primary power storage unit, and this energy may be quickly drained if the sensor node remains operational over long periods of time. Therefore, the idea of harvesting ambient energy from the immediate surroundings of the deployed sensors, to recharge the batteries and to directly power the sensor nodes, has recently been proposed. The deployment of energy harvesting in environmental field systems eliminates the dependency of sensor nodes on battery power, drastically reducing the maintenance costs required to replace batteries. In this article, we review the state-of-the-art in energy-harvesting WSNs for environmental monitoring applications, including Animal Tracking, Air Quality Monitoring, Water Quality Monitoring, and Disaster Monitoring to improve the ecosystem and human life. In addition to presenting the technologies for harvesting energy from ambient sources and the protocols that can take advantage of the harvested energy, we present challenges that must be addressed to further advance energy-harvesting-based WSNs, along with some future work directions to address these challenges.

## 下一步

1. 用 `tools/paper-triage.ps1 <results.json> --select <序号>` 标记真正相关的候选；
2. 从 `raw/inbox/auto-discovered/papers/` 选择项晋升 `raw/canonical/`；
3. 用 MinerU 生成 Markdown，并保留 provenance；
4. 按 `schema/agent-a-compile.md` 执行 A 编译，再更新 Graphify。
