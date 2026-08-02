---
title: "Study on charging strategy of wireless rechargeable sensor networks based on dynamic inhomogeneous clustering"
year: null
source_type: paper
why_relevant: ""
acquisition_method: auto_discovery
discovered_via: ["serpapi"]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260801-214329"
triage_status: promoted
selected_by_user: true
acquired_at: "2026-08-01T13:43:29+00:00"
canonicalized_at: 2026-08-01
ingest_status: ingested
pdf_path: "raw/canonical/Study_on_charging_strategy_of_wireless_rechargeable_sensor_networks_based_on_dynamic_inhomogeneo/Study_on_charging_strategy_of_wireless_rechargeable_sensor_networks_based_on_dynamic.pdf"
raw_md: "raw/canonical/Study_on_charging_strategy_of_wireless_rechargeable_sensor_networks_based_on_dynamic_inhomogeneo/full.md"
---
OPEN

Check for updates

# Study on charging strategy of wireless rechargeable sensor networks based on dynamic inhomogeneous clustering

Peng Tian<sup>1</sup>, JiaYang<sup>1</sup>, Hongyu Pu<sup>2</sup>, Xin Tian<sup>1</sup>, Jiale Tang<sup>1</sup>, Guozheng Ran<sup>1</sup> & Liang Peng<sup>1</sup>

In wireless rechargeable sensor networks (WRSNs), high node mortality rate severely constrains the network performance. To address this problem, this paper proposes an innovative charging strategy based on dynamic inhomogeneous clustering (DICCS). The core of this strategy lies in dynamically adjusting the network clustering structure, which combines the dynamic changes of node energy, position and energy consumption rate to achieve the optimal division of clusters. Firstly, the improved k-means algorithm is used to perform dynamic inhomogeneous clustering of the network, determine the optimal number of clusters through iterative optimization, and introduce a weight function to synthesize the node’s initial energy, residual energy, and the average intra-cluster distance to select the cluster head in order to balance the energy consumption. On this basis, DICCS plans eficient charging paths for mobile charging carts (MCs), designs charging dwell point selection mechanisms for single-node and multi-node clusters respectively, and dynamically adjusts the charging sequence based on the mixed priorities (distance, residual energy, and energy consumption rate). Simulation experiments show that DICCS significantly reduces the node mortality rate (only 4.3%) and charging waiting time, while optimizing the mobility cost of MCs, compared to strategies such as SAMER, VTMT, and FCFS. Its dynamic inhomogeneous clustering mechanism efectively mitigates the energy consumption imbalance problem, improves the network life cycle and stability, and provides an eficient solution for charging scheduling in heterogeneous dynamic WRSNs.

Keywords Dynamic uneven clustering, Node mortality, Path planning, Wireless rechargeable sensor network

Wireless Sensor Networks (WSNs, WSN is a self-organizing network formed by a large number of distributed sensor nodes through wireless communication. Its core objective is to sense, collect, and transmit information from the physical world, such as temperature, humidity, vibration, etc., and send the data to base stations or user terminals)<sup>1</sup> consist of numerous sensor nodes capable of monitoring, sensing, and collecting data from their surrounding environment. These nodes are integral in various applications, including military surveillance<sup>2</sup>, disaster warning systems<sup>3</sup>, medical monitoring<sup>4</sup>, and smart markets<sup>5</sup>. However, the significant energy limitations of sensor nodes prevent continuous operation, posing a major challenge to the long-term functionality of WSNs. The advent of wireless charging technology has alleviated the energy challenges faced by WSNs, leading to the rapid development of Wireless Rechargeable Sensor Networks (WRSNs, WRSN is an extension of WSN that addresses the energy limitations of traditional WSNs by introducing mobile charging devices such as mobile charging vehicles, drones, or fixed charging infrastructure to provide wireless energy replenishment to sensor nodes. Its core concept is to replace “battery replacement” with “charging” to achieve long-term sustainable network operation)<sup>6,7</sup>. In this context, wireless power transfer (WPT)<sup>8</sup> has emerged as a crucial area of research, ofering a more efective solution compared to energy harvesting and node energy-saving techniques.

Within the realm of wireless charging for sensor nodes, two primary charging methodologies have emerged: static charging by a mobile charging cart (MC) and dynamic charging by a mobile charging cart (MC).

## Static charging

Static charging<sup>9</sup>schemes typically involve the use of an MC to charge sensor nodes at fixed locations. A hybrid variable optimization algorithm based on the fireworks algorithm, proposed in Ref. 10, experimentall demonstrates the feasibility of static charging planning. However, in large-scale networks, it becomes challenging for the MC to visit all nodes within a given timeframe, leading to some nodes depleting their energy reserves and failing to operate. Another approach, outlined in Ref. 11, proposes a path-planning method for the MC that divides the sensor network into multiple cells. The MC periodically charges and collects data from the cells containing sensor nodes, optimizing the amount of data collected per unit of energy and ensuring continuous network operation. However, this strategy lacks an efective mechanism for adaptive adjustments in response to significant changes in the network topology or node states, which may necessitate re-planning anchor point and increasing the algorithm’s computational complexity. Additionally, the Mayfly Algorithm (MA) introduced in Ref. 12. addresses energy management by notifying the base station (BS) when the first node’s energy drops below a threshold. The BS then dispatches the MC to recharge all nodes within the same cycle. While this strategy ensures full recharging, it does not explore the potential benefits of partial charging, which might be more eficient in certain scenarios where energy utilization eficiency is a priority.

## Dynamic charging

Dynamic charging<sup>13</sup>strategies aim to enhance the efectiveness of static charging schemes. For example, the strategy proposed in Ref. 14 deploys a single wireless charging device that charges nodes one by one in planar WRSNs. The scheme introduces an energy consumption rate prediction mechanism for each node, along with a dynamic threshold to minimize the charging device’s movement and reduce charging delays.However, this approach’s cluster head rotation mechanism primarily considers the residual energy and the distance to the cluster center, neglecting other critical factors such as communication and processing capabilities of the nodes. In contrast, the dynamic charging method proposed in Ref. 15 uses a non-static charging strategy based on data rate variation. The method sets an attraction function for each node based on residual energy and distance, ensuring that the wireless charging device (WCE) always charges the node with the highest attraction value. While this strategy demonstrates its dynamic charging path selection, its adaptability and eficiency in largescale WRSNs with complex node distributions require further verification. Additionally, the approach in Ref. 16 combines a non-dominated sorting genetic algorithm with multi-attribute decision-making to coordinate multiple MCs, charging nodes based on proximity. However, this strategy does not fully address the fairness of the charging response.

## Fairness in charging response

Fairness in charging response is crucial in WRSNs. Many existing studies assume uniformity in node structure and energy consumption, overlooking the inherent heterogeneity and dynamic energy consumption rates in WRSNs. Such assumptions significantly deviate from practical implementations and complicate the optimization of charging schedules<sup>2,17–22</sup>. This lack of consideration for dynamic factors, such as varying energy consumption rates and time constraints, can reduce the charging eficiency of the MC, leading to imbalanced energy distribution and an increased node mortality rate.

## Proposed DICCS strategy

To address the energy challenges in WRSNs, this paper introduces the Dynamic Inhomogeneous Clustering Charging Strategy (DICCS), which incorporates several key innovations:

1. Dynamic Uneven Clustering: The strategy dynamically clusters the sensor network, allowing multiple nodes to be charged simultaneously. This reduces the mobile charging cart’s energy consumption and minimizes node waiting times during the charging process.

2. Energy Consumption and Location-Aware Charging: The DICCS strategy takes into account diferences in the energy consumption rates of nodes and their spatial locations when determining the MC’s charging stopping points. This ensures more balanced energy distribution across the network.

3. Double Thresholds for Charging Prioritization: The strategy sets double thresholds that consider factors such as node distance, residual energy, and energy consumption rate. This prioritization ensures that high-priority nodes are charged first, thereby enhancing the fairness of the charging process.

By considering these factors, DICCS aims to address the fundamental energy dilemma of WRSNs, improve charging scheduling eficiency, and significantly reduce node mortality, thus extending the overall network lifespan and enhancing operational stability.

## Related work

Sensor nodes have very limited energy and premature energy depletion of some nodes can cause the network to fail to operate continuously. It has become a bottleneck in the field of wireless sensor networks. Literature<sup>23</sup> proposes a charging path planning based on DQN and a dynamic dwell time adjustment algorithm with LSTM-TD3, but both DQN and LSTM-TD3 are complex deep reinforcement learning models, which require a large amount of computational resources and training time, and are dificult to be directly deployed in resourceconstrained sensor nodes or embedded devices. Literature<sup>24</sup> proposes a charging strategy for wireless sensor networks with a single energy-constrained MCD that minimizes the network outage time and maximizes the charging eficiency. However, the strategy is based on a single MCD and does not consider multiple MCDs coscheduling scenarios (e.g., path conflict, task assignment), which may be ineficient in real large-scalenetworks. Literature<sup>25</sup> proposes a greedy heuristic approximation solution for deploying sensors using the aggregation efect (GHDSAE). Then, a greedy heuristic (GH) solution and a particle swarm optimization (PSO) solution are proposed for P2. It is more cost-efective to deploy sensors and chargers jointly using the GHDSAE solution and the PSO solution.The PSO algorithm has a time complexity of 1 and may face high computational overhead in large-scale networks, afecting real-time performance. Literature<sup>26</sup> proposes a Deep Reinforcement Learning approach for JCSCT with Hybrid Action Space (DRLH-JCSCT), which utilizes Deep Q Networks (DQNs) to generate charging sequences and Deep Deterministic Policy Gradients (DDPGs) to determine the charging time. The proposed method improves the charging performance with longer network lifetime and fewer fault sensors. However, the parameter settings are specific to the network characteristics. The number of neurons and layers in the neural network must be balanced with the reward value and training time. The learning rat significantly afects the convergence of the neural network training process. If the learning rate is too high, th network may converge to a local optimum. If it is too low, it may take a long time to train.

In literature<sup>27</sup>, Cheng Pengjiang et al. proposed an improved deep Q network approach for CSCE (IDQN-CSCE), where MC acts as an agent to explore the WRSN and determine the charging strategy based on the charging demand of the sensors.IDQN-CSCE uses Q learning and deep Q networks (DQN) to train both Q tables and DQN networks. Literature<sup>28</sup> proposes a new model-free deep reinforcement learning algorithm, Multi-stage Exploratory Deep Q Network (MEDQN), where MC is designed as an agent to explore the online charging schedule through a new multi-stage exploration strategy to maximize the network QSC based on the real-time network state. however, the computational complexity is high. Literature<sup>29</sup> proposes a QoS-based ondemand charging scheduling (or QOCS) model in which a charging vehicle carries multiple removable batterypowered chargers. In the novel QoS-based billing model, the billing scheduling problem of requesting nodes is investigated to ensure the integrity of network data collection and to maximize the satisfaction of billing services

Literature<sup>30</sup> proposes a Wireless Mobile Charger Ofset Optimization (WMCEO) algorithm that aims to optimize the movement trajectory and charging time of the WMC at each stay location to achieve higher energy eficiency and network lifetime. A Wireless Mobile Charger Ofset Optimization (WMCEO) algorithm designed to optimize the movement trajectory and charging time of the WMC at each stay position to achieve higher energy eficiency and network lifetime. Literature<sup>31</sup> proposes a new WRSN on-demand billing scheduling scheme. First, an eficient network partitioning method is proposed for allocating MCs in order to evenly balance their workloads. Next, fuzzy logic, which mixes various network attributes is employed to determine the charging schedule of MCs. We also develop an expression to determine the charging threshold of a node, which varies according to its energy consumption rate. However, in large-scale networks, MC’s mobility path planning may face the combinatorial explosion problem, and the trade-of between charging delay and energy consumption needs to be addressed. Reference 32 addresses the problem of finding the global optimum in clustering algorithms for wireless sensor networks and the issue of uneven energy consumption in non-uniform clustering algorithms, proposing a new clustering algorithm. The algorithm first employs the flood tree algorithm to determine the network’s optimal value and uses it to calculate the competitive radius of nodes. It then applies the concept of non-uniform clustering to construct clusters of varying sizes. This algorithm achieves significant improvements in energy consumption and energy balance, ultimately extending the network’s lifespan. However, the cluster structure remains relatively fixed once established, without considering dynamic changes in node energy, position shifts, or network topology evolution. When there are significant diferences in node energy consumption rates, the static cluster structure may lead to premature death of some cluster heads. To address this, this paper designs a dynamic clustering mechanism.

Reference 33 addresses the issue of limited energy in wireless sensor networks by proposing an energyeficient non-uniform clustering routing algorithm. The algorithm first elects transmission nodes within “hot zones,” efectively resolving the issue of load imbalance within these zones. This approach reduces energy consumption, improves sensor network performance, and extends network lifetime. However, the algorithm elects cluster heads based on fixed competition radii and remaining energy, and the cluster structure does not adjust after establishment in response to changes in node energy, position shifts, or topological evolution (such as node failure or addition). The DICCS proposed in this paper improves the k-means algorithm to iteratively optimize cluster partitioning in real-time based on node energy, position, and energy consumption rates Reference 34 proposes a non-uniform cluster-based dual cluster head algorithm for wireless sensor network (UDCH). This algorithm first comprehensively considers various node information (such as remaining node energy, distance from the node to the base station, and the parity of the number of operation cycles) to elect cluster heads, dividing the entire network into clusters of unequal sizes. Simulation results show that the UDCH routing algorithm performs excellently in balancing node energy consumption in WSNs. However, the election of sub-cluster heads is limited to a single dimension, and there are constraints associated with the switching of odd and even rounds.

Reference 35 addresses the shortcomings of traditional wireless sensor routing algorithms and improves node energy utilization by proposing an energy-balanced wireless sensor clustering routing algorithm. First, the artificial fish school algorithm is employed to solve for the optimal network partitioning line, ensuring the uniformity of the entire network clustering. Then, the threshold for selecting cluster head nodes is improved to reduce the probability of low-remaining-energy nodes becoming cluster head nodes. Additionally, an adaptive dynamic data communication method is adopted to reduce communication energy consumption. Experimenta results show that compared to current classical wireless sensor network routing algorithms, the proposed algorithm not only makes cluster head node selection more reasonable but also improves node energy utilization eficiency and ensures network energy consumption balance. However, for uniform clustering, non-uniform clustering can better adapt to the uneven energy consumption of sensor nodes. Therefore, the proposed non uniform clustering strategy is designed to adapt to the uneven energy consumption of sensor nodes.

Reference 36 addresses the issue of energy holes caused by uneven clustering in wireless sensor networks and proposes an improved routing algorithm for wireless sensor networks with uneven clustering. The algorithm first elects cluster heads based on factors such as remaining node energy, distance from the node to the base station, node “degree,” and distance from the node to the cluster head. Simulation results indicate that this routing algorithm can efectively conserve energy and balance node energy consumption, thereby extending the network’s lifespan. However, the cluster head election cycle is fixed and does not dynamically adjust based on node energy fluctuations, and the algorithm has a relatively high complexity.

## Network model

The wireless rechargeable sensor network model is shown in Fig.  1, where the sensor nodes are randomly scattered in a $M \times M$ within the rectangular region of the Sensor Node Collection ${ \cal S } = \{ S _ { 1 } , S _ { 2 } , \cdot \cdot \cdot S _ { N } \}$ A mobile charging cart (MC, In wireless rechargeable sensor networks, mobile charging vehicles are devices used to replenish energy for sensor nodes), and a base station (BS, In wireless rechargeable sensor networks, base stations are key hub nodes connecting sensor networks to external networks, such as the Internet and cloud computing centers. They typically have stronger computing power, storage capacity, and communication capabilities, and are responsible for collecting and processing data from sensor nodes and transmitting it to remote servers or user terminals) located at the center of the sensor network are arranged in the sensor network, and the mobile charging cart uses magnetically coupled resonant<sup>37–39</sup> charging to charge the sensor nodes in the network.

The MC as well as the sensor nodes are equipped with repeaters, and the sensor nodes are able to transmit the monitored data, such as temperature, humidity, air pressure, light intensity and other relevant data of the field environment to the base station, which transmits the received data monitored by the sensors to the terminal. The initial energy of the sensor node is $E _ { i n } ( i )$ , Wireless rechargeable sensor networks consist of a large number of sensor nodes ${ { S } _ { i } } \left( i = 1 , 2 , \cdots N \right)$ , Mobile charging trolley and base station together.

During the network operation, the sensor nodes continuously monitor their residual energy. Once the residual energy of a node drops below the charging request threshold, it immediately transmits its relevant information to the cluster head node. Subsequently, the cluster head node is responsible for summarizing and integrating the information of these nodes that need to be recharged. The information data aggregated by the cluster head node contains $( C O S _ { i } , P _ { i } , \mu _ { s t a y } , T _ { i } , E _ { r e i } )$ , where is the location information of the sensor node i.e., the coordinates of the sensor node $i  ( x _ { i } , y _ { i } )$ , P is the energy consumption rate of the sensor node, $\mu { _ { s t a y } }$ is the coordinate information of the location of the charging stopping point within the cluster (where the MC travels to the charging cluster to stop), $T _ { i }$ is the timestamp of the node when the node sends a request for a charging message, and $\boldsymbol { \mathbf { \check { E } } } _ { r e i } ^ { \check { \mathbf { \theta } } }$ is the remaining energy at the time of the node sending the request for a charging messag.

![](images/744cb73816463906960e3503b466201b000a36f4a9c69a6e42cde7be34ddcdd3.jpg)  
Figure 1. Network model.

## Energy model

## Node energy consumption model

The energy consumption of wireless sensor networks mainly consists of three parts: sensing energy, computing energy and communication energy, sensing energy and computing energy is much lower than the communication energy, so in this paper, only the communication energy is considered.

The free space model and multipath fading model are set according to the communication distance between the sending and receiving nodes.The energy consumed by a node to send l bits of data is shown in Eq. (1).

$$
E _ {T X} (l, d) = \left\{ \begin{array}{c} l E _ {e l e c} + l \varepsilon_ {f s} d ^ {2}, d <   d _ {0} \\ l E _ {e l e c} + l \varepsilon_ {m p} d ^ {4}, d.. d _ {0} \end{array} \right.\tag{1}
$$

where, $E _ { T X } ( l , d )$ )denotes the energy consumed by the transmitting node i to send l bits of data to the receiving node j which is at a distance of d from the transmitting node; $l \varepsilon _ { f s } d ^ { \widetilde { 2 } }$ is the energy lost by the transmitting circuit; $\varepsilon _ { f s }$ and $\varepsilon _ { m p }$ are the power amplification energy losses in the free space model and multipath fading model respectively, and $d _ { 0 } = \surd \varepsilon _ { f s } / \varepsilon _ { m p }$ is the distance threshold.

The energy consumed by the node to receive l bit of data is shown in Eq. (2).

$$
E _ {R X} = l E _ {e l e c}\tag{2}
$$

where, $E _ { R X }$ is the energy consumed by the node to receive l bits of data and $E _ { e l e c }$ is the energy consumed per unit of bit by the node to send or receive (here it is assumed that the energy consumed for sending and receiving is the same).

The cluster head node is mainly responsible for collecting the data information from the nodes in the cluster to send to the relay nodes, then the energy consumption of the cluster head node is:

$$
E _ {C N} = \gamma l (\delta_ {i} - 1)\tag{3}
$$

where: δ<sub>i</sub>-Number of nodes in the cluster,γ-Energy consumed by cluster head node to collect l bit data.

## Node charging model

In the wireless rechargeable sensor network system, the energy loss of sensor nodes mainly originates from the data collection and data transmission activities between nodes. When the mobile charging trolley travels to the designated charging stops in the cluster and implements charging operation on the sensor nodes, the derivation based on Friis’ Equations<sup>40,41</sup> leads to the expression of the received power of the sensor nodes as shown in Eq. (4):

$$
P _ {r} = P _ {t} \frac {G _ {t} G _ {r} \eta}{L _ {P}} \left(\frac {\lambda}{4 \pi (d + \beta)}\right) ^ {2}\tag{4}
$$

where: P -MC’s charging power; d-The distance between the node and the MC; $G _ { r ^ { - } } \mathrm { A }$ ntenna gain at the receiving end; λ-Carrier wavelength; η-Rectification eficiency; β-Adjusting the parameters of the free-space equation for short-distance transmission; L -polarization loss, is a parameter and all others are constants.

## Sensor network cluster

Uneven energy consumption among sensor nodes is a critical factor affecting the lifetime of Wireless Rechargeable Sensor Networks (WRSNs). A well-designed clustering mechanism plays a pivotal role in balancing the energy consumption across the network and improving its overall performance and longevity.

In the WRSN architecture, clustering is typically performed to group sensor nodes based on certain criteria, with each group being managed by a cluster head node (CN, In wireless sensor networks, cluster head nodes are key nodes elected in each cluster through clustering algorithms. They are responsible for managing ordinary nodes within the cluster and data fusion processing, and are a core component of the clustered network architecture)<sup>42–44</sup>. The primary responsibility of the cluster head is to aggregate the data collected from all nodes within its cluster and transmit the fused data to the base station (BS). By utilizing data fusion techniques, the cluster head can synthesize information from multiple nodes, which enhances both the accuracy and reliability of the transmitted data. For instance, in a target tracking scenario, multiple sensor nodes might independently measure the target’s position. The cluster head node can combine these measurements using data fusion algorithms to derive a more accurate estimate of the target is location, providing a reliable data foundation for subsequent decision-making processes.

However, as the number of nodes within a cluster increases, the amount of data that the cluster head needs to collect and transmit also rises. This leads to higher energy consumption for data aggregation and transmission. To address this challenge, this paper proposes an uneven clustering strategy, where the energy consumption of cluster heads is dynamically managed based on their proximity to the base station (BS). Specifically, clusters closer to the BS have larger coverage areas and accommodate more sensor nodes. As a result, the energy consumption of the cluster head increases due to the larger volume of data it needs to handle. In contrast, clusters situated farther from the BS tend to have smaller coverage areas with fewer nodes, which reduces the cluster head’s energy consumption for data collection.

![](images/8ef5d0bf84f2543d74f566facdd2560698275fe8d87e6e373eaff403de4ce6a5.jpg)  
Fig. 2. Node distribution and MC charging path map.

This dynamic partitioning strategy is reflected in the MC charging paths within the WRSN, where the charging decisions are tailored based on the energy needs of the nodes and the spatial distribution of the clusters. As shown in Fig. 2, the dynamic partitioning and MC charging paths are designed to optimize energy distribution, ensuring that nodes receive adequate energy replenishment while minimizing energy consumption, particularly for cluster heads that manage larger amounts of data.

By balancing the energy consumption of cluster heads based on their location within the network, this uneven clustering strategy not only enhances the network’s eficiency but also helps to extend the operationa lifetime of the WRSN, ensuring more sustainable performance over time.

According to the sensor network partitioning model, in the network layout, the proportion of nodes in the area closer to the base station is relatively large. In the dynamic partitioning process, if the number of nodes near the base station is lower than the number of nodes far away from the base station, the necessary conditions for cluster partitioning cannot be met. In this case, it is necessary to adjust the loop through several iterations unti the number of nodes in the cluster near the base station exceeds the number of nodes in the cluster far away from the base station, so as to achieve the requirements of uneven clustering and ensure the rationality and efectiveness of the network structure. When clustering the wireless rechargeable sensor network, based on the k-means algorithm $4 5 { - } 4 8$ , the network is initially clustered by considering the energy and distance factors of the sensor nodes, and then the clustering efect is judged by the decision functionϕ(i), which is shown in Eq. (5):

$$
\phi (i) = \frac {\tau (i) - \theta (i)}{\max \{\theta (i) , \tau (i) \}}\tag{5}
$$

where:θ(i)-Degrees of cohesion(Cohesion measures the closeness of samples within the same cluster, $\mathrm { i . e . , }$ , the similarity between sample points and the cluster to which they belong. The smaller the $\theta ( i )$ value, the closer sample i is to other samples in the cluster, and the stronger the cohesion of the cluster) $\mathfrak { i } \tau ( i )$ -Degree of separation (Separation measures the degree of separation between diferent clusters, i.e., the diference between sample points and other clusters. The larger the $\tau ( i )$ value, the greater the distance between sample i and other clusters, and the stronger the separation between clusters); $\phi ( i ) \bar { \in } [ - 1 , 1 ]$

The k-means clustering algorithm pseudo-code is shown below as well as the corresponding flowchart in Fig. 3.

## Algorithm1  K-mean

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Require:  $X = \{x_{1}, x_{2}, \cdots, x_{N}\}$ , T
Ensure:  $\phi_{k}$ 

1:  $K = \{2, 3, \cdots, K_{\max}\}$ 

2:  $\phi^{*} \to -\infty$ 

3: for  $k \in K$  do

4:  $k \quad \mu_{1}^{(0)}, \mu_{2}^{(0)}, \cdots, \mu_{k}^{(0)}$ 

5: for  $t = 1 \to T$  do

6:  $x_{i}$ 

7:  $C_{j}^{(t)} \leftarrow \{x_{i} : \| x_{i} - \mu_{j}^{(t-1)} \|^{2} \leq \| x_{i} - \mu_{l}^{(t-1)} \|^{2}, \forall l \neq j\}$ 

8:  $\mu_{j}^{(t)} \leftarrow \frac{1}{|C_{j}^{(t)}|} \sum_{x \in C_{j}^{(t)}} x$ 

9: if  $(\|\mu_{j}^{(t)} - \mu_{j}^{(t-1)}\| &lt; \partial, \forall j)$  then

10: break

11: end if

12: end for

13:  $\phi_{k}$ 

14: for  $x_{i} \in X$  do

15:  $\theta(i) = \frac{1}{|C_{j}| - 1} \sum_{x \in C_{j}, x \neq x_{i}} \| x_{i} - x\|$ 

16:  $\tau(i) = \min_{l \neq j} \frac{1}{|C_{l}|} \sum_{x \in C_{l}} \| x_{i} - x\|$ 

17:  $\phi(i) = \frac{\tau(i) - \theta(i)}{\max\{\theta(i), \tau(i)\}}$ 

18: end for

19:  $\phi_{k} \leftarrow \frac{1}{N} \sum_{i=1}^{N} \phi(i)$ 

20: if  $\phi_{k} &gt; \phi^{*}$  then

21:  $k^{*} \leftarrow k, \phi^{*} \leftarrow \phi_{k}$ 

22:  $C \leftarrow (C_{1}^{(t)}, ..., C_{k}^{(t)})$ 

23: end if

24: end for

25:  $\forall C_{j} \in C$ 

26:  $CH_{j} = \arg\max_{x_{i} \in C_{j}} Y$ 

27: Return  $C, k^{*}, \{CH_{1}, ..., CH_{k^{*}}\}$
</div>

where $k ^ { * }$ is the optimal number of clusters, $\phi ^ { * }$ is the optimal profile coeficient, and T is the maximum number of iterations of the k-mean clustering algorithm.

Through the decision function, the number of clusters in the wireless rechargeable sensor network can be determined through a number of iterative calculations. k. In this process, a weight function is introduced for the selection of cluster head nodes.

This weight function considers the initial energy of the sensor nodes $E _ { i n } ( i )$ , surplus energy $E _ { r e i }$ and the average distance of the sensor node from other sensor nodes in the cluster $d _ { a v } ,$ The average distance $d _ { a v }$ is shown in Eq. (6):

$$
d _ {a v} = \frac {1}{\delta_ {j} - 1} \sum_ {i = 1, i \neq j} ^ {\delta_ {j} - 1} d (S _ {i}, S _ {o r})\tag{6}
$$

where: $d ( S _ { i } , S _ { o r } )$ is the distance between a sensor node $S _ { i }$ and any member node $S _ { o r }$ in the cluster.

Stability S can be measured by the energy fluctuation of a node over time, let $E _ { i } ( t )$ denote the energy of a node at a moment in time. $T _ { S }$ is the length of the statistical time period, $\overline { { E _ { i } } }$ is the average energy of the node over the time period, then the expression for stability S is shown in Eq. (7):

$$
S = 1 - \frac {\sum_ {t = 1} ^ {T _ {S}} | E _ {i} (t) - \overline {{E _ {i}}} |}{T _ {S} \times \overline {{E _ {i}}}}\tag{7}
$$

Location importance L can be determined based on the distance of the node from the base station or critical area, the closer the distance or the node that is on the critical path may be more important, the expression is as follows:

$$
L = \frac {1}{1 + e ^ {- \theta (Y - Y _ {0})}}\tag{8}
$$

where is the θ adjustment factor with a range of $5 \sim 1 0 ,$ , and $Y _ { 0 }$ is the weighting threshold with a range of $0 . 6 \sim 0 . 8 .$ The weighting function Y is introduced as shown in Eq. (9):

$$
Y = \alpha_ {1} \frac {E _ {i n} (i)}{E _ {r e i}} + \alpha_ {2} \frac {1}{d _ {a v}} + \lambda_ {1} L + \lambda_ {2} S\tag{9}
$$

![](images/91c5e75f62e6c6485efe744f16a20fcdb4f3e29413b010c1cef20b2fff5b8075.jpg)  
Fig. 3. k-mean clustering algorithm flowchart.

where: $\alpha _ { 1 } + \alpha _ { 2 } + \lambda _ { 1 } + \lambda _ { 2 } = 1$ , The weight function is to calculate the weight of the sensor node campaigning for the cluster head node within each cluster, if the sensor node $S _ { i } \mathsf { ^ { c } } _ { s } E _ { r e i } , \breve { L } ,$ S The higher and $d _ { a v }$ smaller the weight Y of the sensor node $S _ { i : }$ , the higher the probability of being a cluster head node, and vice versa, the lower the probability of being a cluster head node. The pseudo-code of the weight function calculation algorithm is as follows, and the corresponding flowchart is shown in Fig. 4.

Algorithm2  Algorithm for the computation of the weight function Y

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Input: weighting factor $\alpha_{1},\alpha_{2},\lambda_{1},\lambda_{2},E_{in}(i),E_{rei}$ Coordinates of node $i\to (x_i,y_i)$, Set of nodes in the cluster $C$  
1: for $j\in C,j\neq i$ do  
2: $d(S_i,S_{or})\gets \sqrt{(x_i - x_j)^2 + (y_i - y_j)^2}$  
3: end for  
4: $d_{av}\gets (\delta_j - 1)^{-1}\cdot \sum_{i = 1,i\neq j}^{\delta_j - 1}d(S_i,S_{or})\quad \mathrm{eq(6)}$  
5: $S\gets 1 - \sum_{t = 1}^{T_S}|E_i(t) - \overline{E_i}|\cdot (T_S\times \overline{E_i})^{-1}\quad \mathrm{eq(7)}$  
6: $L\gets (1 + e^{-\theta (Y - Y_0)})^{-1}\quad \mathrm{eq(8)}$  
7: $Y\gets \alpha_{1}(E_{in}(i) / E_{rei}) + \alpha_{2}(d_{av})^{-1} + \lambda_{1}L + \lambda_{2}S\quad \mathrm{eq(9)}$  
8: Return $Y$
</div>

During the process of electing a cluster head node, a special situation may arise where multiple sensor nodes within the same cluster have the same weight value. In such cases, a secondary criterion is applied to resolve the tie: the residual energy of the sensor nodes with identical weight values is compared. The node with the highest residual energy is selected as the cluster head, as it is more likely to have a longer operational lifespan, ensuring stable and reliable management of the cluster. All nodes in the network participate in the election process, and the node with the highest weight value is ultimately chosen as the cluster head. This weight value is typically a combination of factors such as node energy, distance to the base station (BS), and communication capabilities, which together reflect the node is ability to manage data aggregation and transmission eficiently. Once the cluster head election is completed, the base station (BS) broadcasts the information regarding the elected cluster heads to the member nodes within each cluster. This ensures that all nodes within a cluster are aware of thei respective cluster head, facilitating smooth communication and coordination.

Simultaneously, the BS aggregates the cluster head nodes of all clusters into a set denoted as A, which is used for further network management tasks, such as coordinating energy distribution and optimizing communication routes across the WRSN. This approach ensures that the cluster head election is both fair and energy-eficient, with an emphasis on selecting nodes that can sustain high levels of performance throughout the network’s operation. It also facilitates the proper dissemination of cluster information, promoting eficient dat aggregation and communication within the network.

## Node energy threshold segmentation

Threshold division of sensor node energy, by setting diferent energy thresholds<sup>49–51</sup>, sensor nodes in the network can be made to undertake diferent tasks or work modes according to their remaining energy. For example, nodes with higher energy are assigned more data collection and transmission tasks, while nodes with lower energy are allowed to reduce their activities or enter into a dormant state, so as to equalize the energy consumption of the whole network, avoid certain nodes from prematurely exhausting their energy and dying, and prolong the overall lifetime of the network. The energy threshold can also be used as an important indicator to predict when a node may fail. When the remaining energy of a node is close to or lower than a certain threshold, the network manager can take corresponding measures in advance, such as adjusting routing paths, reassigning tasks, or arranging for node replacement, in order to ensure the normal operation of the network and to minimize the impact of sudden node failure on the performance of the network.

![](images/ea29b9f3b3ee051f23a9ce3e438846d0d45f5e38fd5ba665ff788ebdcd4124e4.jpg)  
Fig. 4. Flowchart of weight function calculation.

The sensor nodes within each cluster report their remaining energy to the cluster head node in real time. The charging strategy proposed in this paper takes into account the diferences in node energy consumption and optimizes the threshold range in real time based on dynamic parameters such as node energy consumption rate, network load, and intra-cluster density, as shown in Eq. (10):

$$
\mathrm{X} = \left\{ \begin{array}{c} E _ {d a n g e r} ^ {i} = E _ {b a s e} \cdot \left(1 - \alpha \cdot \frac {\gamma_ {i}}{\gamma_ {m a x}}\right) \cdot \left(1 + \partial \cdot \frac {d _ {i}}{d _ {m a x}}\right) \\ E _ {e m e r g e n c y} ^ {i} = E _ {d a n g e r} ^ {i} \cdot \left(0. 4 + 0. 6 \cdot e ^ {- \Re \cdot L _ {c l u s t e r}}\right) \\ L _ {c l u s t e r} = \frac {N _ {p a c k e t}}{N _ {m a x}} \\ \gamma_ {i} ^ {n e w} = \ell \cdot \gamma_ {i} ^ {o l d} + (1 - \ell) \cdot \frac {E _ {c o n s u m e d}}{\Delta t} \end{array} \right.\tag{10}
$$

where $E _ { \mathrm { b a s e } } = 4 0 \% E \mathrm { m a x } ( E \mathrm { m a x }$ is the maximum energy of the sensor node), is the adjustable base charging threshold; $\alpha \in [ 0 , 1 ]$ is the energy consumption rate weight (high energy consuming nodes lower the danger threshold); $\partial \in \mathsf { [ 0 , 0 . 5 ] }$ is the distance weight (away from the cluster head node raises the threshold); $\Re > 0$ is the load sensitivity factor (emergency threshold is adaptively lowered in case of high loads); γ is the energy consumption rate of node i (energy consumption per unit of time); $L _ { \mathrm { c l u s t e r } }$ is the current intra-cluster load (frequency of packet transmissions); and $d _ { i }$ is the node i’s to cluster head distance; $\ell \in [ 0 , 1 ]$ is the smoothing factor $( \ell  1 )$ , the new energy consumption rate is almost completely dependent on historical data and ignores current measurements. The system is highly stable but slow in response. $\bar { \ell } \to 0$ , The new energy consumption rate is completely determined by current measurements. The system responds quickly but is susceptible to short term fluctuations, $E _ { \mathrm { c o n s u m e d } }$ is the energy consumed by node i in time interval $( \Delta t ) ; \dot { N } _ { \mathrm { p a c k e t } }$ is the total number of intra-cluster packet transmissions per unit time; and $N _ { \mathrm { m a x } }$ is the preset maximum amount of packets to be carried in the cluster. To avoid the increase in energy consumption due to frequent calculations, the thresholds are updated every 60 s.

1. When $\mathrm { X } = E _ { d a n g e r } ^ { i }$ , the remaining energy of the sensor node is in the warning interval, the node sends a request for charging request and stores it in the set G.

2. When the residual energy of the sensor node drops to the danger zone, the node will send out an emergency charging signal and add itself to the emergency charging aggregate Q.

When the Base Station (BS) receives a charging request from a node, it forwards this information to the Mobile Charger (MC). In this case, if MC receives the emergency charging signal, it will prioritize the charging of nodes in set Q to prevent these nodes from running out of energy and becoming dead nodes. After completing the emergency charging, if there is no new emergency charging signal, the MC will plan the charging of the nodes in the set G according to the priority. Once the charging of the nodes in collection G is completed, the MC will then charge the nodes in the collection H of dead nodes according to the distance priority principle.

## Mobile charging trolley path planning

## Mobile charging trolley charging stops identified

In a network, uneven energy consumption of sensor nodes within each cluster may lead to premature death of individual nodes. Therefore, it is necessary to consider the distance of nodes to the cluster head and their energy consumption rate comprehensively in the charging strategy. Nodes with high consumption rate should be close to the charging dwell point, while nodes with low consumption rate can be a little farther away. These two factors are used to ensure fair and eficient charging of nodes within the cluster when choosing the location of the mobile charger (MC) dwell point, thus guaranteeing the normal operation of the network and the eficient utilization and saving of energy. The following scheme is proposed in this paper for the distribution of nodes in the cluster:

1. When there is only one node in the cluster, there is no need to calculate the charging dwell point and the MC moves directly to that node position for charging.

2. When there are two and more nodes in the cluster, the coordinates of the charging dwell points in the cluster are determined by the coordinate information $C O S _ { i }$ of the sensor nodes and the energy consumption rates of the sensor nodes in diferent states, as shown in Eqs. (11, 12):

$$
\left\{ \begin{array}{l l} X _ {s t a y} = \sum_ {i = 1} ^ {\delta_ {j}} (\xi_ {i} \cdot x _ {i}) \\ Y _ {s t a y} = \sum_ {i = 1} ^ {\delta_ {j}} (\xi_ {i} \cdot y _ {i}) \end{array} \right.\tag{11}
$$

$$
\zeta_ {i} = \omega_ {s} \frac {P _ {s i}}{\sum_ {i = 1} ^ {\delta_ {j}} P _ {s i}} + \omega_ {r} \frac {P _ {r i}}{\sum_ {i = 1} ^ {\delta_ {j}} P _ {r i}} + \omega_ {d} \frac {P _ {d i}}{\sum_ {i = 1} ^ {\delta_ {j}} P _ {d i}}\tag{12}
$$

where: $\xi _ { i }$ is the ratio of the energy consumption rate of node $S _ { i }$ in a given cluster in diferent states to the energy consumption rate of all nodes in the cluster. $P _ { s i } , P _ { r i } , P _ { d i }$ denote the energy consumption rate of node $S _ { i }$ in the data sending, receiving and sleeping states, respectively. $\omega _ { s } , \omega _ { r } ,$ ω are the sensor node data sending energy consumption rate weights, data receiving energy consumption rate weights, and data dormant energy consumption rate weights, respectively. $x _ { i } , y _ { i }$ are the horizontal and vertical coordinates of the sensor node S .When determining the location of the charging point, the charging time of each sensor node in the cluster can be balanced more efectively by introducing the weight factor of the node’s energy consumption power. This method can reasonably allocate the charging resources according to the actual energy demand of the nodes, thus improving the charging eficiency and prolonging the overall lifetime of the network.

## Prioritization of clusters

In this paper, a non-uniform clustering strategy is used, so there is a diference in the energy consumption of diferent clusters. For clusters with more nodes, the energy consumption of the cluster head node is inevitably higher than that of clusters with fewer nodes. These diferences need to be taken into account while formulating the charging priority. The charging cluster priority includes the evaluation of distance priority, which is defined as the ratio of the distance from the MC charging dwell point to the current position of the MC in the current cluster to the total distance from the charging dwell point to the MC in all remaining clusters. This weighting metric enables efective optimization of the MC’s charging path and energy utilization eficiency.

$$
d = \frac {d _ {M C - \mu_ {s t a y}}}{\sum_ {i = 1} ^ {k} d _ {M C - \mu_ {s t a y}}}\tag{13}
$$

$$
d _ {M C - \mu_ {s t a y}} = \sqrt {\left(X _ {M C} - X _ {s t a y}\right) ^ {2} + \left(Y _ {M C} - Y _ {s t a y}\right) ^ {2}}\tag{14}
$$

where: The position coordinates of the MC are $\left( X _ { M C } , Y _ { M C } \right)$ , and the position coordinates of the charging dwell point are $( \bar { X } _ { s t a y } , Y _ { s t a y } )$

The priority of residual energy is the ratio of the average value of residual energy of all sensor nodes in the cluster to the sum of the average value of residual energy in all uncharged clusters. This priority is used to measure the overall residual energy level of the nodes in the cluster, so as to rationally arrange the charging order and prioritize the allocation of charging resources for the clusters with lower residual energy to ensure the continuity of network operation and the fairness of energy allocation. As shown in Eq. (15):

$$
E = \frac {\sum_ {i = 1} ^ {\delta_ {j}} E _ {r e i} / \delta_ {j}}{\sum_ {j = 1} ^ {k - 1} \sum_ {i = 1} ^ {\delta_ {j}} E _ {r e i}}\tag{15}
$$

The priority of energy consumption rate is defined as the ratio of the average value of the energy consumption rate of all nodes in a given cluster to the sum of the total energy consumption rate of all nodes, which is calculated as shown in Eq. (16).

This priority is used to measure the average energy consumption of the nodes within a cluster in order to prioritize the scheduling of charging for clusters with higher energy consumption rates, thus reducing the risk of premature death of the nodes and optimizing the eficiency of energy allocation and utilization across the network:

$$
P = \sum_ {i = 1} ^ {\delta_ {j}} (P _ {i} / \delta_ {j}) / \sum_ {i = 1} ^ {N} P _ {i}\tag{16}
$$

The hybrid priority of a cluster is obtained by weighted summation of the distance priority, residual energy priority and energy consumption rate priority of the cluster according to certain weights. Through the result of weight calculation of hybrid priority, the priority level of each cluster can be determined. Specifically, the priority level reflects the degree of urgency of each cluster after considering the distance, remaining energy and energy consumption rate, so as to provide reasonable charging sequence planning for MCs to achieve eficient energ utilization and fair network operation.

$$
T = \sigma d + \varepsilon E + \zeta P\tag{17}
$$

where: σ is the weight of distance priority, ε is the weight of residual energy priority, ζ is the weight of energy consumption rate priority, and the three weights satisfy $\sigma + \varepsilon + \zeta = 1$

Based on the value of T obtained each time, the cluster with the smallest value of T is selected as the next charging cluster and charged. When the ratio of the weight value of the distance and the energy consumption rate is larger, the cluster with low residual energy is prioritized and the nodes in the cluster are charged. The weight value of the corresponding parameter is calculated for each cluster T, and the charging sequence is arranged based on the size of the final weight value.

## Simulation analysis Parameter settings

In this paper, MATLAB2024a is used to conduct simulation experiments to construct a square area with a side length of 400 m in the network and randomly deploy base stations and a varying number of sensor nodes in the area. The initial position of the MC is set to be the location of the base station, the generation of node data information follows the law of Poisson distribution with an average time interval of 60 s, and the bandwidth of the network is set to be 10kbps.The whole simulation running time is 72,000 s, and the specific simulation parameters are shown in Table 1.

In this paper, the algorithm is used to compare with VTMT<sup>52</sup>, SAMER, and FCFS<sup>53</sup> strategies, where the VTMT(Vehicle-to-Microgrid Technology, charging strategy is an intelligent charging strategy that takes into account the interaction between the vehicle and the microgrid. The VTMT charging strategy is designed to optimize the charging process of electric vehicles (EVs) while taking into account the operating conditions and demands of the microgrid. It dynamically adjusts the charging power and time of EVs through real-time monitoring and analysis of the microgrid’s parameters such as voltage, frequency, and power balance, as well as the EV’s charging demand, battery status, and other information, in order to achieve the coordinated optimization of vehicle charging and microgrid operation.) strategy is a research strategy that can provide charging support for dead nodes, and the performance of the strategies can be evaluated by comparing the node mortality rate. The SAMER(The SAMER charging algorithm, i.e., Starvation Avoidance Mobile Energy Replenishment (SAMER) algorithm, is proposed to address the problem of energy constraints in wireless rechargeable sensor networks. The SAMER algorithm avoids energy shortfalls by calculating and taking into account the maximum tolerable waiting time for each charging request. shortage. It comprehensively evaluates the energy status, charging demand, and location of each sensor node in the network, and plans reasonable charging paths and scheduling schemes for the mobile chargers, prioritizing the charging demand of nodes that are about to run out of energy and have high charging waiting time requirements, so as to ensure a balanced supply of energy in the whole network, and to efectively avoid node failures caused by energy starvation.) strategy is a classical online charging strategy, while the FCFS (First-Come, First-Served, charging strategy is a charging management strategy based on the first-come, first-served principle, e.g., if sensor node i sends a charging request before sensor node j, the MC prioritizes charging sensor node i, even though sensor node i has a higher residual energy than sensor node j.)strategy has a lower charging performance, and is usually used to compare the performance of DICCS strategy. The performance metrics analyzed in this paper for comparison are as follows:

## Cost of charging

It refers to the total distance traveled by the MC in the process of completing a round of charging tasks. This metric reflects the eficiency of charging path planning, the shorter distance indicates the lower charging cost and the better strategy performance.

## Nodal mortality rate

It refers to the ratio of the number of nodes dying due to energy depletion to the total number of nodes. Node death rate is an important indicator for evaluating the charging strategy, the smaller the value, the better the strategy is in guaranteeing the continuous operation of the network.

## Average waiting time for node charging

The average time that the sensor node to be charged waits from sending the charging request message to the MC charging it. By comparing and analyzing these metrics, the advantages and disadvantages of diferent charging strategies can be more comprehensively evaluated

## Impact of changes in parameters on indicators

In this paper, we vary the MC movement speed, MC charging rate, number of nodes, topology of the network, and node energy consumption rate parameters to observe the impact on the above performance metrics.

## Impact of MC movement speed on metrics

In this part of the experiment, we focus on the efect of the change of MC movement speed on the performance index of each charging strategy. The performance of the four strategies at diferent speeds is observed in detail by gradually changing the MC’s moving speed so that it grows linearly in the range 1 \~ 8 m/s.

As shown in Fig. 5(a), a clear trend emerges where the node mortality rates of all four charging strategies decrease gradually as the MC’s moving speed increases. However, the DICCS strategy outperforms the other three strategies. It is evident from the results that when the MC’s speed reaches a certain threshold, the node mortality rate of the DICCS, VTMT, and SAMER strategies approaches zero. This indicates that at higher speeds, these strategies efectively maintain the operational status of the sensor nodes and prevent node failure due to energy depletion. Notably, the DICCS strategy exhibits a significantly lower node mortality rate compared to the other three strategies within this speed interval.

The superior performance of DICCS can be attributed to its unique approach, which incorporates a dua energy threshold for sensor nodes and prioritizes the charging of nodes with lower residual energy. Additionally, the MC continuously collects real-time data about the sensor nodes, allowing it to dynamically adjust its charging schedule based on the current energy status of the nodes. This dynamic mechanism ensures that the DICCS strategy efectively reduces the mortality rate of sensor nodes while ensuring fairness and eficiency in the charging process. In contrast, the VTMT and SAMER strategies exhibit similar node mortality rates, both showing some charging eficacy. However, the FCFS strategy has a notably higher mortality rate compared to the other strategies.

This is mainly due to the FCFS strategy’s failure to fully consider the fairness of charging and the dynamic energy consumption of nodes during the charging process, resulting in suboptimal charging path planning. This ineficiency increases the MC’s travel distance, and certain sensor nodes may fail to receive timely charging, causing them to deplete their energy and die prematurely. Consequently, the FCFS strategy demonstrates poorer charging performance, as reflected in its high node mortality rate.

In summary, the DICCS strategy achieves the best results in terms of node mortality and charging fairness, making it more suitable for wireless sensor networks characterized by high dynamics and uneven energy consumption distributions.

As shown in Fig. 5(b), the average waiting time for sensor node charging decreases as the MC’s speed increases. At higher MC speeds, the average waiting time for the DICCS, VTMT, and SAMER strategies approaches zero. However, the average waiting time for the FCFS strategy is approximately 3.1 times longer than that of the DICCS, VTMT, and SAMER strategies when the MC moves at a high speed. This highlights the ineficiency of the FCFS strategy in reducing waiting times. The reason for this ineficiency is that the FCFS strategy does not consider the geographic location of the sensor nodes when determining the charging sequence. As a result, the MC moves in a disorganized, back-and-forth manner, increasing its travel overhead. This unnecessary movement leads to longer waiting times for nodes to receive charging services. Although the waiting time for the FCFS strategy decreases with an increase in MC speed, it still lags behind the DICCS, VTMT, and SAMER strategies.

<table><tr><td>Symbolic</td><td>Clarification</td><td>Numerical value</td></tr><tr><td> $M^{*}M$ </td><td>Simulation area</td><td>400 m*400 m</td></tr><tr><td>N</td><td>Number of nodes</td><td>25~200</td></tr><tr><td> $E_{in}(i)$ </td><td>Node initial energy</td><td>50 J</td></tr><tr><td> $E_{mc}$ </td><td>MC initial energy</td><td>8000 J</td></tr><tr><td> $E_{rate}$ </td><td>MC Charge Rate</td><td>100 mJ/s</td></tr><tr><td> $V_{mc}$ </td><td>MC Movement Speed</td><td>2 m/s</td></tr><tr><td> $E_{m}$ </td><td>Demagnetizing factor</td><td>3 mJ</td></tr></table>

Table 1. Parameter settings.

![](images/84fb6e94ba3d4c5dee7bfa6bc30893677ae438b8af8ca5193fdceaf61f3ce01e.jpg)  
(a) Nodal mortality rate

![](images/dfc7e6df415d06a37334a127469c5f0f9fa494664340e44fdec61820a9db14bb.jpg)  
(b) Average waiting time

![](images/386cfa7d5dd1108309cdeb001ec9c0b75226907f4d496a2a2b8fbca1890bcb72.jpg)  
(c) Cost of charging  
Fig. 5. Efect of MC movement speed on metrics.

As shown in Fig. 5(c), the MC charging cost for all four strategies increases as the MC’s speed rises. This increase is due to the fact that a higher MC speed enables it to serve more nodes per unit of time, thereby increasing the total distance it needs to cover within the network. Among the strategies, the DICCS and FCFS strategies have lower charging costs compared to VTMT and SAMER. For example, the charging cost of the MC in the SAMER strategy is approximately 1.27 times higher than that of the VTMT strategy, 1.91 times higher than that of the FCFS strategy, and 2.30 times higher than that of the DICCS strategy when the MC moves at a high speed.

Despite the rising charging costs with increased MC speed, the DICCS strategy maintains relatively optimal overall performance. This is because DICCS carefully considers factors such as node distribution, energy states, and charging eficiency when designing the charging process. As a result, the MC can select the most eficient charging path and prioritize the charging of sensor nodes in a more rational manner, thereby mitigating the impact of increased charging costs caused by the higher MC speed. This comprehensive approach allows the DICCS strategy to strike a balance between charging eficiency, cost, and overall network performance.

## Impact of MC charging rate on metrics

This set of experiments observes the performance of diferent charging strategies by adjusting the charging rate of the MC to the nodes (the range is set to 75 mJ/s −250 mJ/s). The experimental results are shown in Fig. 6:

As shown in Fig. 6(a), as the MC charging rate increases, the number of sensor nodes served by all four charging strategies (FCFS, DICCS, SAMER, and VTMT) also increases. This is because a higher charging rate allows the MC to transmit more energy to the sensor nodes per unit of time, enabling more nodes to receive energy replenishment in a timely manner. This ultimately leads to a reduction in the node mortality rate. The DICCS charging strategy again demonstrates its superior performance. Specifically, when the MC charging speed reaches a certain point, the node mortality rate of the DICCS strategy essentially drops to zero. At this point, the node mortality rate in the DICCS strategy stabilizes, while the mortality rates of the FCFS, SAMER, and VTMT strategies remain non-zero, even at the same charging speed. For instance, when the charging speed reaches a specific value, the FCFS strategy’s node mortality rate is about 1.65 times that of the SAMER strategy and 2.32 times that of the VTMT strategy. This is mainly because the FCFS strategy only charges sensor nodes based on the sequence in which the nodes request charging, without considering the energy state of the nodes or the overall network condition. As a result, the FCFS strategy demonstrates the worst performance and th highest node mortality rate.

As shown in Fig. 6(b), the average charging waiting time for sensor nodes decreases with the increase in the MC charging speed for all four strategies. This reduction in waiting time reflects improved charging eficiency, as the increased speed allows the MC to serve more nodes within a given time period.

For example, when the MC charging speed reaches a specific point, the average charging waiting time for the FCFS strategy is 1.36 times longer than that of the VTMT strategy, 1.62 times longer than that of the SAMER strategy, and 2.0 times longer than that of the DICCS strategy. The DICCS strategy excels in reducing waiting times due to its ability to charge multiple nodes simultaneously, leveraging parallel charging. This parallel charging mechanism ensures that the charging resources are used more eficiently. In contrast, the FCFS strategy is serial in nature, meaning nodes are charged one after the other. This serial approach fails to maximize the use of available charging resources, resulting in longer waiting times for sensor nodes to be charged.

As shown in Fig. 6(c), the number of sensor nodes served by the MC increases as the MC’s charging rate accelerates. However, the charging cost for the FCFS strategy is significantly higher than that for the DICCS, SAMER, and VTMT strategies, mainly because the FCFS strategy involves unnecessary back-and-forth movements of the MC in the sensor network. Additionally, the FCFS strategy does not adequately take into account the relationship between charging eficiency and cost during the design process. For instance, when the MC charging speed reaches a certain point, the charging cost of the FCFS strategy is about 1.52 times that of the VTMT strategy, 2.0 times that of the SAMER strategy, and 2.92 times that of the DICCS strategy. The DICCS strategy exhibits the lowest charging cost due to its more eficient selection of sensor nodes based on their energy consumption dynamics, ensuring that the MC charges the nodes with the most pressing needs first. This eficient scheduling leads to a lower node mortality rate while efectively reducing the overall charging cost. As a result, the DICCS strategy delivers higher overall performance compared to the FCFS, SAMER, and VTMT strategies, even as the MC’s charging speed increases.

![](images/0a1e875d6b7ea056d83e1955737a3b7be94e1f551205939176771a349b8b6866.jpg)  
(a) Nodal mortality rate

![](images/cbcf25927f438841ad15ec38c51fc7c4893dc33ff43425b889fc676b67d21076.jpg)  
(b) Average waiting time

![](images/cfef66a6e08d1ba9012ec87b30aebba1a16e0f9da7464341540c5f3d513adfa0.jpg)  
(c) Cost of charging  
Fig. 6. Efect of MC charging rate speed on metrics.

In summary, the DICCS strategy consistently outperforms the other strategies in terms of node mortality rate, charging waiting time, and charging cost due to its intelligent, dynamic charging mechanism that takes into account both the energy consumption of individual nodes and the overall network dynamics.

## Impact of the number of nodes on indicators

This set of experiments analyzes the efect of the change in the number of nodes on each performance metric by gradually increasing the number of nodes at intervals of 25 (ranging from 25 to 200). The experimental results are shown in Fig. 7.

As shown in Fig.  7(a), when the number of sensor nodes is small, such as 50 nodes, the node mortality rates for all four charging strategies (FCFS, SAMER, VTMT, and DICCS) are relatively low. This is because, in smaller networks, the energy resources are more abundant, allowing each sensor node to obtain enough energy easily, thus maintaining normal operation. However, as the number of nodes increases, competition for energy intensifies, leading to higher node mortality rates across all strategies. Among these, the FCFS, SAMER, and VTMT strategies show a faster increase in node mortality, while the DICCS strategy demonstrates significantly lower mortality rates compared to the others. For instance, when the number of sensor nodes reaches 150, the node mortality rate of the FCFS strategy is 1.21 times that of the VTMT strategy, 1.6 times that of the SAMER strategy, and 6.8 times that of the DICCS strategy. This significant improvement in the DICCS strategy is primarily due to its dual threshold mechanism, which prioritizes the charging of nodes with critically low energy. Additionally, the DICCS strategy continuously gathers real-time node data, enabling it to make accurate decisions regarding which nodes should receive charging first, efectively balancing energy distribution across the network and minimizing node mortality.

As shown in Fig. 7(b), the average waiting time for charging of all four strategies increases with the growing number of sensor nodes. This is because an increasing number of nodes leads to more charging requests, while the MC’s charging capacity is limited, requiring nodes to wait longer for charging services. Among the strategies, the DICCS and VTMT strategies experience a slower increase in waiting time, while the FCFS and SAMER strategies show a faster rise in waiting time.

For example, when the number of sensor nodes reaches 175, the average waiting time for FCFS charging is 1.91 times that of the SAMER strategy, 2.16 times that of the VTMT strategy, and 3.25 times that of the DICCS strategy. The FCFS strategy lacks dynamic assessment of the nodes’ energy urgency and their geographical distribution. This blind charging selection leads to poorly planned MC charging paths, resulting in longer traveling distances for the MC and a delayed charging process for many nodes. In contrast, the DICCS strategy optimizes the path planning based on energy urgency and node location, leading to more eficient charging and shorter waiting times.

As shown in Fig.  7(c), the charging cost of all four strategies initially increases and then decreases. This behavior is primarily related to the changing node density and the optimization of the charging paths. When the number of nodes is small, such as below 75 nodes, the MC has to travel larger distances to serve nodes due to their sparse distribution, leading to higher charging costs. However, as node density increases, charging path optimization becomes more eficient, and the MC’s travel distance decreases, lowering the charging cost. In this process, the DICCS strategy stands out by accounting for dynamic changes in node energy consumption, prioritizing charging, and adjusting the charging path in real-time. This approach ensures more eficient allocation of charging resources. By considering factors like node residual energy, energy consumption rates, and distances, the DICCS strategy optimizes the MC is charging path and charging sequence, minimizing unnecessary movement and efectively reducing the charging cost.

In summary, the DICCS strategy consistently outperforms the other strategies in terms of node mortality rate, waiting time, and charging cost by dynamically adjusting to changes in the network and eficiently managing the MC’s charging resources. This makes the DICCS strategy more efective in large and dynamic sensor networks.

![](images/7919129edabc329279f0f9773b538603b49e2357c107787555e12d091e563500.jpg)  
(a) Nodal mortality rate

![](images/ef51555658f0fe75dc2eb385cfb5cbe69120c3f1e3f87fbc56a5c50699f9370d.jpg)  
(b) Average waiting time

![](images/a1fc26b30ac36de6d706356a471f5f49fcaad58cb994dfc48595f26e3b9230f2.jpg)  
(c) Cost of charging  
Fig. 7. Efect of number of nodes on metrics.

Impact of the network’s topology on metrics

The network topology is very important for diferent network metrics (e.g., node mortality, average waiting time, charging cost, transmission eficiency, reliability, etc.). Each topology exhibits diferent performance characteristics under diferent strategies, afecting the eficiency, stability, and cost of the network. Network topology can significantly afect several metrics under diferent application scenarios and management strategies. In this paper, we demonstrate the impact of four diferent policies (FCFS, VTMT, SAMER, DICCS) on the metrics parameters under eight diferent network topologies (Circular, Cellular, Triangular, Mesh, Rectangular, Tree, Random, Star). The experimental results are shown in Fig. 8.

As shown in Fig. 8(a), the impact of diferent network topologies on Node Mortality (NM) is presented for four charging strategies (FCFS, VTMT, SAMER, and DICCS). The FCFS strategy shows relatively consistent performance across most topologies. However, the node mortality rate increases as the network topolog changes, with a notable increase under triangular and rectangular topologies.

The VTMT strategy performs better than FCFS, with a lower node mortality rate, especially under random and star topologies, where the node mortality rate remains relatively low. The SAMER strategy also shows better performance across all topologies, maintaining a low node mortality rate, particularly in star and random topologies. In contrast, the DICCS strategy exhibits the highest node mortality rate, particularly under circular and star topologies. Interestingly, cellular network topology consistently demonstrates the lowest node mortality rate, suggesting that it is the most eficient topology in terms of maintaining node survival.

In Fig. 8(b), the efect of network topologies on average waiting time under the four strategies is shown. FCFS generally results in high waiting times across most topologies, with the circular and triangular topologies showing exceptionally high waiting times (close to 800  s). This suggests that certain topologies may cause resource contention when network loads are high, leading to longer waiting times for charging. Compared to FCFS, VTMT generally maintains a lower average waiting time, especially under tree and star topologies, where the waiting time is relatively low (around 300  s). This indicates that these topologies support more eficient resource scheduling, which in turn reduces waiting times. On the other hand, circular and triangular topologies tend to have longer waiting times, potentially due to longer data paths and less eficient node connectivity.

The SAMER strategy demonstrates a consistently low average waiting time for all topologies, performing particularly well in star and tree topologies, with waiting times under 300 s. This highlights SAMER’s ability to adapt well to network changes, reducing resource contention and enhancing charging eficiency. The DICCS strategy has the shortest waiting times in most topologies, especially in the star topology, where the waiting time is minimized to around 300 s. This can be attributed to the centralized nature of the star topology, which allows for more eficient scheduling and resource allocation. The cellular topology also performs well, though it shows slightly higher waiting times than the star topology.

As shown in Fig. 8(c), the efect of network topologies on charging cost is demonstrated. FCFS generally experiences rising charging costs across most topologies, with particularly high charging costs under circular and triangular topologies. However, the star topology stands out with a relatively low charging cost, indicating its eficiency in resource management, which helps reduce the overall charging-related costs. VTMT exhibits relatively low charging costs across all topologies, particularly under star and tree topologies, where charging costs remain low. This suggests that these topologies allow for better management of battery and charging resources, thus reducing the charging cost. On the other hand, SAMER tends to have higher charging costs, especially in cellular and rectangular topologies, where significant resource allocation ineficiencies lead to increased costs. DICCS generally shows the highest charging costs across most topologies, particularly in treeshaped and rectangular topologies. This may be due to the distributed concurrent communication strategy, which increases communication and charging overheads in these topologies. However, the star topology performs better in charging cost under DICCS, suggesting that this topology is better suited to reduce resource consumption in concurrent communication management.

In summary, diferent network topologies significantly influence the key parameters of node mortality rate, waiting time, and charging cost. By selecting the appropriate topology based on network size, load, and performance objectives, it is possible to optimize resource management, reduce costs, improve transmission eficiency, and enhance the reliability of the network. Star topology appears to be the most eficient for reducing waiting time and charging costs, while cellular topology is the most efective in minimizing node mortality. DICCS performs well in most topologies but benefits the most from star topology for eficient resource management.

![](images/3ca6e98fc88d8f83dd102e32934591816b9a15f59773f8f099be43d89c2736d9.jpg)  
(a) Nodal mortality rate

![](images/be1df364327feb04ba380fa0e64ce905651164e2b616a52ef151b42dc03b5d5d.jpg)  
(b) Average waiting time

![](images/665623f326cd0dc74d91af1d9665548d8de3772ca948f704fdce3910cfbe2d2f.jpg)  
(c) Cost of charging  
Fig. 8. Impact of the topology of the network on the metrics.

## Impact of nodal energy consumption rates on indicators

This set of experiments analyzes the efect of the change in the energy consumption rate of the nodes on each performance index by gradually increasing the energy consumption rate of the nodes in intervals of 0.25mJ/ s(range of 0.5mJ/s\~2.25mJ/s). The experimental results are shown in Fig. 9.

As shown in Fig.  9(a), when the energy consumption rate of sensor nodes gradually increases, the node mortality rate of all four charging strategies (FCFS, VTMT, SAMER, and DICCS) also increases. The increase in mortality is especially significant for the FCFS strategy. This happens because when the energy consumption rate rises, the sensor nodes consume energy more rapidly, and if their energy is not replenished in time the nodes die. This leads to an increase in node mortality within the network. Specifically, when the energy consumption rate reaches a certain level, the FCFS strategy shows a node mortality rate that is 1.4 times that of SAMER, 1.94 times that of VTMT, and 2 times that of DICCS. Despite the increase in node mortality with higher energy consumption rates, the DICCS strategy continues to exhibit a lower mortality rate compared to the other three strategies. This superior performance is attributed to the DICCS strategy’s use of a dual threshold mechanism (emergency charging set Q and warning set G), which prioritizes charging for nodes with high energy consumption.

Additionally, dynamic clustering is used to optimize the charging path, ensuring that high-energy-consuming nodes receive timely energy replenishment, thereby reducing their mortality rate.

As seen in Fig.  9(b), when the energy consumption rate of sensor nodes increases, the average charging waiting time for all four strategies also increases. However, the FCFS strategy has a significantly higher charging waiting time compared to VTMT, SAMER, and DICCS strategies. The increased energy consumption rate leads to more frequent charging requests, and without eficient charging scheduling, the waiting time for nodes to get charged increases. The FCFS policy processes requests sequentially, without considering the energy consumption of the nodes, leading to a sharp rise in waiting times. For instance, when the node energy consumption rate increases, the FCFS policy’s average charging waiting time is about 2.75 times that of SAMER, 5 times that of VTMT, and 15 times that of DICCS. Although the waiting times for VTMT, SAMER, and DICCS are similar, the DICCS strategy results in the shortest waiting time. This is because DICCS uses dynamic clustering and parallel charging mechanisms (such as multi-node dwell point optimization) to reduce unnecessary movement between clusters and shortens the waiting time for high-energy-consuming nodes through priority scheduling. As a result, DICCS efectively reduces the average charging waiting time, thereby extending the lifespan of the sensor network.

In Fig. 9(c), when the energy consumption rate of the sensor nodes increases, the charging cost for the FCFS strategy remains the highest compared to the VTMT, SAMER, and DICCS strategies. This is because the mobile charger (MC) needs to move more frequently to meet the increased demand for charging caused by the higher energy consumption rate, leading to an increase in the total distance traveled. The high charging cost for FCFS is primarily due to ineficient path planning, such as round-trip movements that increase the distance covered. On the other hand, the DICCS strategy has the lowest charging cost, as it optimizes the charging path by considering factors such as distance, remaining energy, and energy consumption rate. By doing so, it minimizes redundant movements and centralizes the service for multiple nodes using the clustering mechanism, reducing the cost of each charging trip. Although DICCS’s charging cost increases as the energy consumption rate rises, the increase is smaller than that of the other strategies. The VTMT and SAMER strategies show a decreasing trend in charging costs when the energy consumption rate increases. This reduction is due to changes in the distribution of nodes or network topology as some nodes fail due to rapid energy depletion, which leads to the formation of denser clusters of remaining nodes. In this case, the MC can serve most of the nodes by simply moving between these clusters, reducing the travel distance compared to when nodes were scattered across the network.

In summary, as the energy consumption rate of sensor nodes increases, the FCFS strategy experiences a significant rise in both node mortality and charging waiting time, as well as a higher charging cost due to ineficient scheduling and movement. The DICCS strategy, however, efectively mitigates these issues by prioritizing high-energy-consuming nodes, optimizing charging paths, and using dynamic clustering, resulting in a lower node mortality rate, shorter waiting times, and lower charging costs compared to the other strategies. VTMT and SAMER also perform better than FCFS, but DICCS remains the most eficient strategy, especially under high energy consumption conditions.

![](images/2c031c26b150ffd641969fae36dc7b37471967fbbcdb3968d3b7c5fa6ca75ac2.jpg)  
(a) Nodal mortality rate

![](images/bf52d16148f802c39ce842f401524a8a82c3103f436d89f2399f3986e6235f9c.jpg)  
(b) Average waiting time

![](images/0b4cb9ba2662ee6e24933f38ab713b54a338a16acf1784ae5d0118e6b0922590.jpg)  
(c) Cost of charging  
Fig. 9. Impact of node energy consumption rate on metrics.

## Real-time and Robustness Testing.

The real-time test is designed to verify the ability of the DICCS strategy to respond quickly to dynamic network changes, including charging request processing, and scheduling delays of mobile charging carts (MC).

## Charge request processing time.

A burst scenario where nodes send charging requests at diferent energy thresholds is simulated (e.g., 50% of nodes enter the warning threshold at the same time), and the time from the issuance of the request to the completion of the message processing at the base station (BS) is recorded. The results show that the DICCS strategy has an average request processing time of 0.8 s, which is significantly lower than that of SAMER (1.5 s) and FCFS (2.2 s) through the dynamic clustering and double thresholding mechanism.

## MC Response Time.

In a network with node densities from 25 to 200, the average time for MCs to arrive at the first charging dwel point from receiving a charging command is counted. The average response time of MCs in DICCS is 12 s, which outperforms that of VTMT (18 s) and FCFS (28 s). Thanks to the dynamic cluster optimized path planning, the MC movement path is reduced by 23%.

The robustness test verifies the stability and fault tolerance of the DICCS policy in abnormal environments, including scenarios such as node failure, and MC failure.

## Node Random Failure.

Randomly shut down 30% of the nodes and observe the charging eficiency and network survival rate of the remaining nodes. The results show that DICCS by dynamically adjusting the cluster head weights and charging priorities, the mortality rate of the remaining nodes increases to only 5.1%, which is significantly lower than that of SAMER (8.7%) and FCFS (12.3%).

## Dynamic energy consumption rate fluctuations.

The node energy consumption rate is randomly adjusted (± 50%) to test the system’s adaptability to sudden changes in energy consumption. The results show that DICCS has a node mortality rate fluctuation range of less than 1.2% by updating the energy rate weights in real time, while FCFS has a fluctuation range of 4.5%.

The DICCS strategy shows eficient dynamic response capability in real-time, and its charging request processing, MC scheduling and topology reconfiguration time are better than the comparison algorithms; in the robustness test, DICCS efectively copes with abnormal scenarios such as node failure and communication interference through redundancy mechanism, dynamic priority adjustment and path optimization, and the network mortality rate and charging eficiency remain stable. Experiments show that DICCS is suitable for largescale WRSN in dynamic and complex environments, and has strong engineering practical value.

## Conclusion

Aiming at the network performance limitation problem caused by high node mortality in wireless rechargeable sensor networks (WRSNs), a charging strategy based on dynamic inhomogeneous clustering (DICCS) is proposed. Through the dynamic clustering mechanism with hybrid priority scheduling, the energy allocation eficiency is significantly optimized and the node mortality rate is reduced. Specific methods include (1) an improved k-means algorithm combines node energy, location and energy consumption rate to dynamically adjust the cluster structure, and introduces a weight function (combining the initial energy, remaining energy and average distance within the cluster) to elect the cluster head to balance the energy consumption, and (2) designing a mobile charging vehicle (MC) charging path planning strategy that determines the stopping points for single-node and multi-node clusters, respectively, and scheduling them through hybrid priority (distance, remaining energy, energy consumption rate) to dynamically adjust the charging sequence.

Simulation experiments show that DICCS outperforms SAMER, VTMT, and FCFS strategies in terms of node mortality rate, charging waiting time, and MC mobility cost.DICCS prioritizes and guarantees the charging of high-energy-consumption nodes through dual dynamic charging thresholds, which improves the network life cycle and stability. In future research, we will extend the multi-MC cooperative charging mechanism to solve the path conflict and task allocation problems in large-scale networks; optimize the complexity of the algorithm to enhance the real-time performance and robustness to cope with the challenges of dynamic topology and node mobility; and explore the application of deep learning and reinforcement learning in charging scheduling to further improve the resource allocation eficiency.

## Data availability

Data is available, we can provide all data if required, please contact 15882970488@163.com.

Received: 16 April 2025; Accepted: 10 July 2025

Published online: 30 September 2025

## References

1. Kaur, A., Bansal, M. & Kumar, D. Mobile agent and ACO based data aggregation for wireless sensor network. SN Comput. Sci. 6 (1), 77–77 (2025).

2. Li, J. et al. A deep reinforcement learning approach for online mobile charging scheduling with optimal quality of sensing coverage in wireless rechargeable sensor networks. Ad Hoc Netw. 156, 103431 (2024).

3. Kumar, B. C. & Dinesh, Y. An energy eficient periodic data gathering and charging schedule using MVs in wireless rechargeable sensor networks. Computing 105 (11), 2563–2593 (2023).

4. Fanian, F., Rafsanjani, K. M. & Shokouhifar, M. Combined fuzzy-metaheuristic framework for Bridge health monitoring using UAV-enabled rechargeable wireless sensor networks. Appl. Soft Comput. 167 (PC), 112429–112429 (2024).

5. Tomar, A. et al. Towards intelligent decision making for charging scheduling in rechargeable wireless sensor networks. J. Netw. Syst. Manage. 32 (4), 86–86 (2024).

6. Hsu, F. L. WRSN energy replenishment strategy based on charging eficiency. Comput. Application Res. 39 (02), 521–525 (2022).

7. Banimelhem, O., & Hamad, B.S. A Proactive Charging Approach for Extending the Lifetime of Sensor Nodes in Wireless Rechargeable Sensor Networks. J Sens Actuat Netw 14(2), 26–26 (2025).

8. Hai, Z.S., Yuan, X.C., & Cai, Z.Q. Modelling of mutual inductance between superconducting pancake coils used in wireless power transfer (WPT) systems. IEEE Trans Appl Supercond 29(2), 1–4 (2019).

9. Chen Hua, W. et al. Cyclic charging planning for wireless charging devices with energy constrained WRSNs. J. Electron. Meas. Instrum. 31 (07), 1031–1039 (2017).

10. Wang, Z. et al. Periodic charging path planning based on PE-FWA algorithm in WRSNs. J. Electron. Meas. Instrum. 33 (05), 118–124 (2019).

11. Chengkai, X. Research on WCV Periodic Charging and Data Collection Planning in Wireless Rechargeable Sensor Networks, Hefei University of Technology (2020)

12. Sandrine, M. & Kewen, X. Multi-Objective optimization with mayfly algorithm for periodic charging in wireless rechargeable sensor networks. World Electr. Veh. J. 13 (7), 120–120 (2022).

13. Yang Jia, K. & Dongshan, Y. Research on multi-node on-demand charging strategy based on WRSN. Comput. Application Res. 40 (09), 2736–2742 (2023).

14. Wang, Q. Research on on-demand Charging Strategy for Wireless Rechargeable Sensor Networks, Wuhan University of Ligh Industry (2024).

15. Lei Lei, C. Charging strategy for wireless rechargeable sensor networks based on firefly algorithm. J. Guilin Univ. Electron. Sci. Technol. 41 (06), 477–483 (2021).

16. Smriti, P. & Abhinav, T. An eficient partial charging scheme using multiple mobile chargers in wireless rechargeable sensor networks. Ad Hoc Netw. 113 (prepublish), 102407 (2021).

17. Zhang, N. et al. Research on hybrid mobile charging scheduling for wireless rechargeable sensor networks. J. Sens. Technol. 34 (09), 1237–1249 (2021).

18. Wang Yang, Z. et al. A directed charging scheduling scheme for wireless rechargeable sensor networks based on utility maximization. J. Electron. Inform. 43 (05), 1331–1338 (2021).

19. Dong Ying, C. et al. Charging scheduling for clustered rechargeable wireless sensor networks based on energy prediction. J. Jilin Univ. (Engineering Edition). 48 (04), 1265–1273 (2018).

20. Xiaoguo, Y. Mobile charging scheduling algorithm in wireless rechargeable sensor networks. Comput. Application. 37 (S1), 13–17 (2017).

21. Qin, H. et al. Petri-Net-Based charging scheduling optimization in rechargeable sensor networks. Sensors 24 (19), 6316–6316 (2024).

22. Huang, S. et al. Charging scheduling method for wireless rechargeable sensor networks based on energy consumption rate prediction for nodes. Sens. (Basel Switzerland). 24 (18), 5931–5931 (2024).

23. Zhao, M. et al. An eficient dynamic energy replenishment and data gathering strategy based on deep reinforcement learning in wireless rechargeable sensor networks. Alexandria Eng. J. 123, 170–179 (2025).

24. Liu, G., Chen, Y. & ,Jiao, W. Maximize Lifetime of Wireless Rechargeable Sensor Networks with Mobile Energy-Limited Charging Device, Sensors 23(18) (2023)

25. Lian, J. & Yao, H. Joint deployment of sensors and chargers in wireless. Rechargeable Sens. Networks Energies. 17 (13), 3130–3130 (2024).

26. Jiang, C. et al. Deep reinforcement learning approach with hybrid action space for mobile charging in wireless rechargeable sensor networks. Expert Syst. Appl. 249 (PC), 123752 (2024).

27. Jiang, C. et al. An improved deep Q-network approach for charging sequence scheduling with optimal mobile charging cost and charging eficiency in wireless rechargeable sensor networks. Ad Hoc Netw. 157, 103458 (2024).

28. Li, J. et al. A deep reinforcement learning approach for online mobile charging scheduling with optimal quality of sensing coverage in wireless rechargeable sensor networks. Ad Hoc Networks 156, 103431 (2024).

29. ZhanSheng, C., Hui, T. & Hong, S. Improve the quality of charging services for rechargeable wireless sensor networks by deploying a mobile vehicle with multiple removable chargers. Wireless Networks 28(7), 2805–2819 (2022).

30. Malebary, S. Wireless mobile charger excursion optimization algorithm in wireless rechargeable sensor networks. IEEE Sens. J. PP(99), 1 (2020).

31. Tomar, A., Muduli, L. & P Jana, K. A fuzzy Logic-based On-demand charging algorithm for wireless rechargeable sensor networks with multiple chargers. IEEE Trans. Mob. Comput. PP(99), 1 (2020).

32. He Wei, X. Improved non-uniform clustering wireless sensor network routing algorithm. Comput. Eng. Appl. 53(24), 136–141 (2017).

33. Pin, Z., Jiajia, W. & Meng, Z. Energy-eficient non-uniform clustering routing algorithm for wireless sensor networks. J. Sens. Technol. 29(12), 1919–1923 (2016).

34. Wu Zhengjiang, Z. & Deng Yiqin, M. Deng yiqin. A new Non-uniform clustering dual cluster head algorithm for wireless sensor networks: the UDCH algorithm . Small Micro Comput. Syst. 37(12), 2612–2616 (2016).

35. Zhu, F., Wei, J. An energy-eficient unequal clustering routing protocol for wireless sensor networks[J]. Int J Distrib Sens Netw 15(9), 1550147719879384 (2019). https://doi.org/10.1177/1550147719879384.

36. Wenmei, Z. & Fubao, L. Improved non-uniform clustering routing algorithm for wireless sensor networks. J. Sens. Technol. 28(05), 739–743 (2017).

37. Tang, J. et al. Magnetic coupling resonant ultra-high voltage electric vehicle wireless charging system based on parity‐time symmetry. Int. J. Circuit Theory Appl. 52 (4), 2027–2037 (2024).

38. Benalia, N. et al. Enhancing electric vehicle charging performance through series-series topology resonance-coupled wireless power transfer. PloS One. 19 (3), e0300550 (2024).

39. Niu, B. et al. Research on Magnetic Coupled Resonant Wireless Charging Technology Applied To Intelligent Patrol Robots. Journal of Physics: Conference Series 1650(2), 022095 (2020)

40. Cho, H. Y. & Byun, J. W. Generalized Friis Transmission Equation for Orbital Angular Momentum Radios. IEEE Trans. Antennas Propag. 67(4P2), 2423–2429 (2019).

41. Thiele, A. G. Friis transmission over a ground plane: Understanding the efects of Nonfree-Space conditions. IEEE Antennas Propag. Mag. 61 (1), 72–76 (2019)

42. Wu, Y. H., Shi, Y. B. & Huang, Y. Y. LEACH optimization algorithm based on energy balanced selection of cluster head nodes. J. Liaoning Univ. Petrochemical Technol. 43 (06), 82–88 (2023).

43. Alshammri, H.G. Enhancing wireless sensor network lifespan and eficiency through improved cluster head selection using improved squirrel search algorithm[J]. Artif Intell Rev 58(3), 79 (2025). https://doi.org/10.1007/S10462-024-11088-4.

44. Xuehui, Z., Jun, W. & You, G. A novel cluster heads election method based on group sparse TSK fuzzy system for energy-eficient wireless sensor network. J. Intell. Fuzzy Syst. 46 (2), 5299–5311 (2024).

45. Zhang, Y. et al. Application of K-means clustering and spectroscopic analysis for rapid sorting of inner Shell colors in freshwater pearl mussels Hyriopsis schlegelii. Aquaculture, 599742128 (2025).

46. Qi, M., Zhao, J. & Feng, Y. An optimized public opinion communication system in social media networks based on K-means cluster analysis. Heliyon 10 (24), e40033 (2024).

47. Boyan T., Jun W., Xiaoyu D., et al., Memorial K-means clustering for cooperative spectrum sensing in cognitive wireless sensor networks at low SNR regimes. Sensor Review 45(3), 443–452 (2025).

48. Dugyala, R. et al. Secure cloud computing: leveraging GNN and leader K-means for intrusion detection optimization. Sci. Rep. 14(1), 30906 (2024).

49. Li & Lingle Ren xiuli. Wormhole attack detection algorithm for wireless sensor networks based on double constraint mechanism J. Sens. Technol. 37(09), 1629–1637 (2024).

50. Gaurang, R., & Madhuri, B. Improving energy Estimation based clustering with energy threshold for wireless sensor networks. Int. J. Comput. Appl. 113 (19), 41–47 (2015)

51. Kung, W. et al. WSN ant colony routing algorithm based on adaptive residual energy threshold. J. Northwest. Polytechnical Univ. 40(02), 442–449 (2022).

52. Hu, C. et al. From waste to wealth: current advances in recycling technologies for metal recovery from Vanadium-Titanium magnetite tailings. J. Sustainable Metall. 10(3), 1007–1035 (2024).

53. Ma, J., Tang, C., Xu, W., et al. An algorithm for train delay propagation on double-track railway lines under FCFS management. Front Eng Manag 11(4), 1–13 (2024).

## Author contributions

TP.YJ and TX wrote the main manuscript text, TJL.RGZ and PL prepared the manuscript centerfold, and all authors reviewed it.

## Declarations

## Competing interests

The authors declare no competing interests.

## Additional information

Correspondence and requests for materials should be addressed to J.Y.

Reprints and permissions information is available at www.nature.com/reprints.

Publisher’s note Springer Nature remains neutral with regard to jurisdictional claims in published maps and institutional afiliations.

Open Access This article is licensed under a Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License, which permits any non-commercial use, sharing, distribution and reproduction in any medium or format, as long as you give appropriate credit to the original author(s) and the source, provide a link to the Creative Commons licence, and indicate if you modified the licensed material. You do not have permission under this licence to share adapted material derived from this article or parts of it. The images or other third party material in this article are included in the article’s Creative Commons licence, unless indicated otherwise in a credit line to the material. If material is not included in the article’s Creative Commons licence and your intended use is not permitted by statutory regulation or exceeds the permitted use, you will need to obtain permission directly from the copyright holder. To view a copy of this licence, visit http://creativecommo ns.org/licenses/by-nc-nd/4.0/.

© The Author(s) 2025