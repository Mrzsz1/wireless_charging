---
title: "Collaborative Charging Optimization for Wireless Rechargeable Sensor Networks via Heterogeneous Mobile Chargers"
year: null
source_type: paper
why_relevant: ""
acquisition_method: auto_discovery
discovered_via: ["arxiv"]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260801-214329"
triage_status: promoted
selected_by_user: true
acquired_at: "2026-08-01T13:43:29+00:00"
canonicalized_at: 2026-08-01
ingest_status: ingested
pdf_path: "raw/canonical/Collaborative_Charging_Optimization_for_Wireless_Rechargeable_Sensor_Networks_via_Heterogeneous/Collaborative_Charging_Optimization_for_Wireless_Rechargeable_Sensor_Networks_via_He.pdf"
raw_md: "raw/canonical/Collaborative_Charging_Optimization_for_Wireless_Rechargeable_Sensor_Networks_via_Heterogeneous/full.md"
---
# Collaborative Charging Optimization for Wireless Rechargeable Sensor Networks via Heterogeneous Mobile Chargers

Jianhang Yao, Hui Kang, Geng Sun, Senior Member, IEEE, Jiahui Li, Member, IEEE, Hongjuan Li, Jiacheng Wang, Yinqiu Liu

Abstract—Despite the rapid proliferation of Internet of Things applications driving widespread wireless sensor network (WSN) deployment, traditional WSNs remain fundamentally constrained by persistent energy limitations that severely restrict network lifetime and operational sustainability. Wireless rechargeable sensor networks (WRSNs) integrated with wireless power transfer (WPT) technology emerge as a transformative paradigm, theoretically enabling unlimited operational lifetime. In this paper, we investigate a heterogeneous mobile charging architecture that strategically combines an automated aerial vehicle (AAV) and a ground smart vehicle (SV) in heterogeneous deployment scenarios to collaboratively exploit the superior mobility of the AAV and extended endurance of the SV for energy distribution. We formulate a multi-objective optimization problem that simultaneously addresses the dynamic balance of heterogeneous charger advantages, charging efficiency versus mobility energy consumption trade-offs, and real-time adaptive coordination under time-varying network conditions. This problem presents significant computational challenges due to its highdimensional continuous action space, non-convex optimization landscape, and dynamic environmental constraints. To address these challenges, we propose the improved heterogeneous agent trust region policy optimization (IHATRPO) algorithm that integrates a self-attention mechanism for enhanced complex environmental state processing and employs a Beta sampling strategy to achieve unbiased gradient computation in continuous action spaces. Simulation results demonstrate that IHATRPO achieves a 51% performance improvement over the original HATRPO, significantly outperforming state-of-the-art baseline algorithms while substantially decreasing sensor node mortality rate and improving charging system efficiency.

Index Terms—Wireless rechargeable sensor network, collaborative charging optimization, heterogeneous mobile chargers, trust region policy optimization

## I. INTRODUCTION

With the rapid proliferation of Internet of Things (IoT) applications, wireless sensor networks (WSNs) have become fundamental infrastructures for environmental monitoring, smart cities, industrial automation, and precision agriculture [1], [2]. WSNs are self-organizing wireless networks that monitor physical phenomena such as temperature, sound, vibration, or pollutants [3]. Due to the small size, low power consumption, and autonomous network establishment capabilities of sensor nodes, conventional WSNs offer high flexibility, good adaptability, and low operational costs [1]. However, WSNs face a persistent challenge as the finite energy capacity of sensor nodes severely constrains network lifetime and operational sustainability. Specifically, sensor nodes typically rely on batteries that are difficult or impossible to replace in remote deployments, thereby leading to network degradation and eventual failure as nodes exhaust their energy reserves [4]. Recent research on extending the lifetime of WSNs has concentrated on energy conservation and energy provisioning approaches. While energy conservation techniques [5], [6] can significantly extend network lifetime, those methods cannot guarantee network stability since batteries will eventually be depleted. Energy provisioning through renewable energy harvesting offers a continuous energy supply, yet is constrained by unpredictable environmental conditions [6], [7].

To address these fundamental limitations, wireless rechargeable sensor networks (WRSNs) have emerged as a transformative paradigm. Specifically, WRSNs integrate wireless power transfer (WPT) technology with conventional sensing capabilities, theoretically providing indefinite operational lifetime [8]. Moreover, WRSNs employ dedicated charging infrastructure that can be categorized into static charging stations and mobile charging platforms. Static charging stations, while providing reliable power delivery, require extensive deployment due to the limited spatial range of WPT technology, resulting in prohibitively high infrastructure costs and reduced deployment flexibility [9]. Conversely, mobile charging platforms offer superior coverage adaptability and dynamic resource allocation capabilities. Among mobile charging solutions, automated aerial vehicles (AAVs) and ground smart vehicles (SVs) represent two complementary approaches with distinct operational characteristics. Specifically, AAVs excel in mobility, rapid deployment, and terrain independence but are constrained by limited energy capacity and weather sensitivity [10], [11], while SVs provide extended operational endurance and robust performance but are restricted by terrain accessibility and mobility limitations [12].

Current research on WRSNs focuses on single-type charging scenarios, which involve a trade-off between mobility and energy efficiency. However, single-type charging approaches, whether AAV-based or SV-based, cannot simultaneously optimize all critical performance metrics due to their individual limitations and the diverse requirements of WRSNs. Such fundamental limitations become particularly pronounced in complex deployment environments where sensor nodes exhibit varying energy demands, spatial distributions, and accessibility constraints that exceed the capabilities of any single charging platform. Motivated by these observations, we propose to combine AAV and SV platforms [13] and design a heterogeneous mobile charging architecture to overcome the inherent limitations of homogeneous charging approaches. This strategic coordination between heterogeneous chargers enables adaptive resource allocation that responds to varying sensor node energy demands and environmental constraints, potentially revolutionizing the efficiency and reliability of WRSNs.

However, implementing such heterogeneous mobile charging coordination introduces several significant technical challenges that existing solutions cannot adequately address. Firstly, the coordination problem between AAVs and SVs requires sophisticated collaborative decision-making mechanisms that can dynamically balance their respective advantages while accounting for different energy consumption patterns, mobility constraints, and charging capabilities in realtime operational conditions [13]. Secondly, the multi-objective optimization nature of the problem involves simultaneously maximizing charging efficiency, minimizing mobility energy consumption, and reducing sensor node mortality, then creating complex trade-offs that traditional optimization approaches cannot effectively resolve due to conflicting objectives and non-convex solution spaces [14]. Finally, sensor network conditions exhibit dynamic and time-varying characteristics, including fluctuating energy levels, changing environmental conditions, and evolving communication requirements [15], which necessitate adaptive strategies that can respond to these variations without compromising long-term performance objectives or system stability.

Accordingly, this paper proposes a novel deep reinforcement learning (DRL)-based approach for collaborative charging optimization in WRSNs employing heterogeneous mobile chargers. The main contributions of this paper are summarized as follows:

• Innovative Heterogeneous Air-Ground Collaborative

Charging System (HAGCCS) Model: We design a comprehensive system model that strategically integrates the AAV and SV as collaborative charging agents in WRSNs. This architecture is specifically tailored for complex deployment scenarios where single-charger solutions prove inadequate. To the best of our knowledge, this is the first work to systematically investigate the collaborative charging optimization problem for heterogeneous mobile chargers while considering their distinctive mobility characteristics, energy constraints, and charging capabilities.

• Multi-Objective Optimization Problem with Heterogeneous Charger Interdependencies: We formulate a multiobjective optimization problem that characterizes the complex interdependencies among charging efficiency maximization, mobility energy minimization, and sensor node mortality minimization in an environment with heterogeneous mobile chargers. This formulation enables the identification of fundamental trade-offs inherent in multi-objective optimization, where competing objectives generate a conflicting solution space, thus requiring collaborative coordination mechanisms. Moreover, this prob lem reveals distinctive coordination dynamics and complementary operational patterns in heterogeneous charger collaboration.

• DRL Solution with Heterogeneous Trust Region Strategy: To address the dynamic and multi-objective nature of the optimization challenge, we propose the improved heterogeneous agent trust region policy optimization (IHA-TRPO) algorithm. This approach incorporates two key innovations. First, the self-attention mechanism enables agents to process complex environmental information and inter-agent interactions more effectively. Second, the Beta sampling strategy ensures unbiased gradient computation for continuous action spaces with bounded constraints. These enhancements specifically address the challenges of decentralized decision-making in heterogeneous multiagent environments while ensuring convergence stability.

• Simulation and Performance Evaluation: Simulation results demonstrate that the proposed algorithm outperforms various baselines, e.g., proximal policy optimization (PPO), multi-agent deep deterministic policy gradient (MADDPG), heterogeneous-agent trust region policy optimization (HATRPO). Moreover, the heterogeneous charger coordination approach significantly enhances sensor network survivability while maintaining charging efficiency. In addition, it is also confirmed that collaborative AAV-SV deployment provides adaptive coverage capabilities that effectively respond to dynamic network conditions.

The rest of this paper is organized as follows. Section II reviews the related research activities in WRSNs. Section III presents the system models. Section IV formulates the optimization problem. Section V introduces the proposed IHATRPO algorithm. Section VI provides the comprehensive simulation results and performance analysis, and Section VII concludes the paper with discussions on future research directions.

## II. RELATED WORK

In this work, we aim to propose a collaborative charging optimization framework in WRSNs by using heterogeneous mobile chargers. This topic involves the charging system architecture in WRSNs, optimization objectives in WRSN charging systems, and optimization methods for WRSN charging. Thus, we briefly introduce the related works of these areas as follows.

## A. Charging System Architectures in WRSNs

Various charging system architectures have been designed to prolong the network lifetime in WRSNs. Traditional groundbased charging strategies have been extensively investigated, where mobile charging vehicles traverse the network to replenish sensor nodes. For example, the authors in [16] proposed a periodic charging and scheduling scheme aimed at optimizing the charging time and sensor selection of charging vehicles. Moreover, the authors in [17] proposed an on-demand charging strategy that incorporates spatial, temporal, and event domain characteristics of nodes, while utilizing an improved K-means algorithm for network partitioning with terrestrial wireless charging vehicles. Further building upon this ground-based mobile charger architecture, the authors in [18] focused on optimizing for network tasks by jointly selecting sensors and allocating energy.

With the advancement of AAV technology, aerial charging systems have emerged as promising alternatives for WRSN energy replenishment. For example, the authors in [19] proposed a joint scheduling and trajectory optimization problem for single-AAV based charging scenarios, thus improving charging efficiency by reducing repeated charging nodes while minimizing hovering points and flight distance. Furthermore, the authors in [17] investigated a multi-AAV deployment optimization problem and proposed an improved firefly algorithm to optimize charging efficiency, motion energy consumption, and sensor coverage. In [20], the authors proposed a cooperative air-ground architecture where one AAV charges sensors, and a ground-based vehicle provides battery replacement for the AAV, using a Deep Q-Network to optimize the strategy.

Recent studies have further explored advanced scheduling under mobility constraints and probabilistic approaches. Specifically, the authors in [21] proposed an integrated sensing and communication-assisted WRSN protocol that coordinates multiple mobile charging vehicles (MCVs) by incorporating probabilistic techniques to balance charging load and reduce travel cost. Moreover, the authors in [22] proposed a novel scheduling protocol, which models key node attributes as a probability distribution to guide on-demand MCV dispatch, improving charging delay and energy efficiency. Furthermore, the authors in [23] investigated periodic charging scheduling in AAV-based WRSNs with automatic landing pads, extending AAV service range through mid-flight energy replenishment.

However, these works treat ground-based and aerial charging systems as independent solutions, thus overlooking the potential collaborative benefits of air-ground cooperative charging. Different from these methods, we design a heterogeneous charging system that simultaneously coordinates both the AAV and SV to achieve complementary operational advantages and compensate for individual limitations.

## B. Optimization Objectives in WRSN Charging Systems

The optimization objectives in WRSN charging systems have been primarily focused on network lifetime maximization and node mortality rate minimization. For instance, the authors in [24] proposed a hybrid approach targeting network longevity through optimized charging scheduling, where inner rings adopt single-node charging with flat topology while outer rings employ multi-node charging with cluster topology. Moreover, the authors in [25] proposed an energy-efficient adaptive directional charging algorithm that focuses on maximizing sensor node survival rates by adaptively selecting single-node or multi-node charging based on sensor node density.

Energy consumption optimization of mobile chargers represents another critical research direction. The authors in [26] proposed a DRL-based mobile safety policy intervention algorithm specifically targeting single mobile charger energy efficiency in an uncertain environment with mobile obstacles. Moreover, the authors in [27] combined SV deployment with recovery operations, jointly optimizing charging and recovery scheduling to minimize overall system energy consumption while handling increased charging requests.

Charging efficiency has also received considerable attention in recent studies. Specifically, the authors in [28] proposed efficient algorithms for increasing energy efficiency in WRSNs for cyber-physical systems through intelligent scheduling and sensor node prioritization without requiring prior knowledge of energy levels. Furthermore, trajectory optimization has emerged as a key goal for enhancing charging efficiency, where researchers focus on minimizing travel distances and optimizing charging paths to improve overall system performance.

Recent multi-objective studies have further advanced AAVassisted charging and IoT optimization. Specifically, the authors in [29] proposed a multi-objective DRL algorithm for an AAV-assisted wireless powered IoT network to jointly optimize data rate, harvested energy, and AAV energy consumption. Moreover, the authors in [30] jointly optimized average data rate, energy consumption, and coverage fairness for AAVassisted IoT networks via combined on-policy and off-policy RL. Furthermore, the authors in [31] formulated a multiobjective resource allocation problem for an AAV-assisted power IoT system enabling simultaneous data collection and wireless charging, solved by an RL-based dynamic algorithm.

However, these studies either optimize a single chargingrelated objective or combine WRSN charging with other tasks. Different from these approaches, our work focuses on the charging tasks while jointly optimizing the multi-objective problem composed of the mortality of sensor nodes, energy consumption of chargers, and charging efficiency.

## C. Optimization Methods for WRSN Charging

Conventional optimization methods have been widely employed for WRSN charging problems. For example, the graphbased optimization approaches have been extensively used, where the authors in [32] proposed comprehensive frameworks by using hexagonal decomposition and boustrophedon path planning for energy-aware coordination of one AAV in WRSN, thus addressing simultaneous period-area coverage, charging scheduling, and resource allocation challenges. Moreover, evolutionary computation methods have also demonstrated effectiveness, as shown in [4], which proposed an improved non-dominated sorting genetic algorithm-based solution for many-objective charging optimization in WRSNs. Additionally, heuristic optimization techniques have been applied in some works, where researchers employ greedy algorithms and local search methods to solve charging scheduling problems with polynomial time complexity.

Recent advances in DRL have introduced intelligent decision-making capabilities to WRSN charging systems. For instance, the authors in [33] proposed a novel DRL approach with a hybrid action space for mobile charging, specifically employing the deep deterministic policy gradient (DDPG) algorithm to determine optimal charging time allocation and achieve improved network lifetime through continuous action space control. Furthermore, the authors in [34] introduced an asynchronous and scalable multi-agent proximal policy optimization algorithm for cooperative charging, thus demonstrating enhanced charging coordination through distributed policy optimization with improved scalability for large-scale scenarios.

However, these DRL-based works primarily focus on homogeneous multi-agent systems without considering the coordination challenges inherent in heterogeneous agent environments. Current approaches lack the collaborative mechanisms required to handle heterogeneous agent coordination between the AAV and SV with fundamentally different operational characteristics. These limitations motivate us to propose a specialized multi-agent DRL algorithm capable of managing heterogeneous agent interactions.

## D. Motivation and Contributions of This Work

Different from these works, we consider a heterogeneous air-ground cooperative charging system by using both the AAV and SV. Moreover, we formulate a multi-objective optimization problem that jointly considers the mortality of sensor nodes, energy consumption of chargers, and charging efficiency. To solve it, we propose an innovative heterogeneous multi-agent DRL method specifically designed for coordinating agents with diverse operational characteristics and capabilities. In the following section, therefore, we present a detailed description of the system model under consideration.

## III. SYSTEM MODELS AND PRELIMINARIES

In this section, we introduce the models of the considered HAGCCS, including the network model, wireless charging model, and energy consumption models of the AAV and SV.

## A. Network Model

The HAGCCS under consideration is illustrated in Fig. 1, and it comprises the following elements:

![](images/561cee0a989f27e62841ecf264d017a0d64a43d9c7180ec0829a921afa513b91.jpg)  
Fig. 1. Architecture diagram of the HAGCCS for the WRSN. The AAV and SV travel within the WRSN to collaboratively provide energy for sensors through WPT waves.

![](images/789fc28ad4c1915f48460fce1bd202ea94f8907e744347ce4249f228fd6de481.jpg)  
Fig. 2. The time slot division model in HAGCCS.

• A set of sensor nodes $\mathcal { S } = \{ 1 , 2 , \ldots , N _ { S } \}$ . These sensor nodes are stationary and randomly distributed throughout the network, primarily tasked with data collection. Note that each sensor node can transmit data to a remote base station (BS) or receive commands from it [13]. Moreover, each sensor node is equipped with an energy harvesting unit and an energy storage unit, which means that it can receive and store wireless energy transferred by mobile chargers [35].

• A pair of heterogeneous mobile chargers. Specifically, the heterogeneous mobile chargers consist of an AAV and an SV. Note that both the AAV and SV are capable of processing data from sensor nodes, BSs, and other mobile chargers [36]. Moreover, the AAV and SV can travel freely within the network area to provide charging service for the sensor nodes within a specified radius [37], and their batteries power both of them.

• A remote BS that acts as a data fusion center. This BS is located at the edge of the region for data collection, and without loss of generality, we consider that the BS has no energy constraint since it has a sufficient energy supply [38].

In HAGCCS, the energy consumption of sensor nodes typically follows certain protocols and cycles to ensure efficient network operation and prolong network lifetime. In this case, we consider a discrete-time system evolving over the timeline $\mathcal { T } = \{ t | 1 , 2 , . . . , T \}$ . Specifically, each time slot t consists of two main phases that are the sensing phase and charging phase, as illustrated in Fig. 2. In the sensing phase, sensor nodes perform data collection, data processing, and data transmission. In the charging phase, the AAV and SV provide wireless energy transfer to the sensor nodes.

TABLE I SUMMARY OF MAIN NOTATIONS

<table><tr><td>Notation</td><td>Description</td></tr><tr><td> $\mu$ </td><td>Charging efficiency</td></tr><tr><td> $\lambda_1, \lambda_2, \lambda_3$ </td><td>Weighting coefficients for reward components</td></tr><tr><td> $\lambda$ </td><td>Wavelength of RF</td></tr><tr><td> $\rho$ </td><td>Air density</td></tr><tr><td> $\eta$ </td><td>Rectifier efficiency</td></tr><tr><td> $\delta$ </td><td>The KL threshold</td></tr><tr><td> $\theta_i$ </td><td>Heading angle of agent  $i$ </td></tr><tr><td> $\gamma$ </td><td>Discount factor</td></tr><tr><td> $\alpha_b, \beta_b$ </td><td>Shape parameters of Beta distribution</td></tr><tr><td> $a_t^i$ </td><td>Action of agent  $i$  at time slot  $t$ </td></tr><tr><td> $b_i$ </td><td>Binary death indicator of sensor node  $i$ </td></tr><tr><td> $d_{\text{max}}$ </td><td>Maximum charging radius of AAV/SV</td></tr><tr><td> $d_i$ </td><td>Distance between sensor node and agent  $i$ </td></tr><tr><td> $d$ </td><td>Distance between sensor node and AAV/SV</td></tr><tr><td> $f_1, f_2, f_3$ </td><td>Charging efficiency, travel distance, node mortality</td></tr><tr><td> $G_s, G_r$ </td><td>Antenna gain of transmitter and receiver</td></tr><tr><td> $\mathcal{N}$ </td><td>Agent set</td></tr><tr><td> $L_p$ </td><td>Polarization loss</td></tr><tr><td> $P_0$ </td><td>Transmit power of AAV/SV</td></tr><tr><td> $P_i$ </td><td>Received power at sensor node  $i$ </td></tr><tr><td> $P_{AAV}(v)$ </td><td>Motion energy consumption of AAV</td></tr><tr><td> $P_{SV}(v)$ </td><td>Motion energy consumption of SV</td></tr><tr><td> $q_i^t$ </td><td>Energy level of sensor node  $i$  at time  $t$ </td></tr><tr><td> $s_t$ </td><td>State space at time slot  $t$ </td></tr><tr><td> $\mathcal{S}$ </td><td>Set of sensor nodes</td></tr><tr><td> $\mathcal{T}$ </td><td>Set of time slots</td></tr><tr><td> $v$ </td><td>Flight/travel speed of AAV/SV</td></tr><tr><td> $X_{max}, Y_{max}$ </td><td>Maximum range of WRSN area</td></tr><tr><td> $Z_t$ </td><td>Decision variables at time slot  $t$ </td></tr></table>

Based on this, we consider that all the sensor nodes and SV are located within the same two-dimensional plane, while the AAV maintains a constant altitude when flying or hovering. As such, the locations of the i-th sensor node, AAV, SV, are denoted as $( x _ { i } ^ { S } , y _ { i } ^ { S } , 0 ) , ( x ^ { A A V } , y ^ { A A V } , h ) , ( x ^ { S V } , y ^ { S V } , 0 )$ respectively.

As such, during each time slot, the AAV and SV travel freely within the sensor network to charge nearby sensor nodes, which aims to improve the charging efficiency and extend the network lifetime. In the following, we model the wireless charging model and energy consumption model of the AAV and SV to identify the key decision variables for optimizing wireless energy transfer and its transmission efficiency.

## B. Wireless Charging Model

In WRSNs, WPT enables the transmission of electrical energy wirelessly from the transmitter to the receiver across the air gap. We consider a radio-frequency (RF) based omnidirectional WPT model [39], which utilizes RF waves at a specific frequency for energy transmission, thereby allowing energy to propagate in all directions.

As such, the charging efficiency $\mu$ of the AAV or SV for sensor nodes can be defined as follows:

$$
\mu = \frac {G _ {s} G _ {r} \eta}{L _ {p}} \left(\frac {\lambda}{4 \pi (d + \beta)}\right) ^ {2},\tag{1}
$$

where $G _ { s }$ denotes the antenna gain of the AAV or SV, $G _ { r }$ represents the antenna gain of the sensor nodes as the receiver, $\lambda$ is the wavelength of the RF signal, $\eta$ is the rectifier efficiency, $L _ { p }$ is the polarization loss, β is a tunable parameter in the Friis free-space equation, and d is the distance between the AAV or SV and the sensor node.

Since in Eq. (1), all parameters except for d and $\beta$ are constant values in a specific WRSN, the calculation for the charging efficiency $\mu$ can be simplified as $\mu = { \alpha } / ( d + \beta ) ^ { 2 }$ where α is a constant that encompasses the parameter values of $G _ { s } , G _ { r } , \eta , L _ { p } , \lambda$ , and others from Eq. (1). Let $P _ { 0 }$ represent the transmit power of the AAV or SV. Then, the received power $P _ { i }$ at the i-th sensor node $S _ { i }$ can be given by $P _ { i } = \mu _ { i } P _ { 0 }$

From Eq. (1), it can be observed that the received power at the sensor node primarily depends on the distance between the AAV or SV and the sensor node, as all parameters except for d can be considered constants. As such, we set the max charging distance $d _ { m a x }$ to assess the impact of distance on the received power. Specifically, when the distance between the AAV (or SV) and sensor node exceeds $d _ { m a x } .$ , the received power at the sensor node becomes too low for energy rectification, thus preventing effective charging. Therefore, $d _ { m a x }$ can be regarded as the effective charging radius. The received power $P _ { i }$ can then be further expressed as follows:

$$
P _ {i} = \left\{ \begin{array}{l l} \frac {\alpha P _ {0}}{(d _ {i} + \beta) ^ {2}} & d _ {i} \leq d _ {m a x} \\ 0 & d _ {i} > d _ {m a x} \end{array} \right..\tag{2}
$$

## C. Energy Consumption Model of AAV and SV

The total energy consumption of the AAV and SV consists of two main components. The first part is the energy consumed by the AAV and SV for charging sensor nodes. The second part is the energy consumed during the movement of the AAV and SV, including propulsion and hovering for the AAV, as well as the travel of the SV. Moreover, the energy consumption caused by communication among sensor nodes, mobile chargers, and BS is negligible compared to the movement energy consumption. Therefore, we focus on the wireless charging energy in Section III-B and motion energy consumption in this section. Based on this, we consider the use of rotary-wing AAV and SV equipped with DC motors, with their respective motion energy consumption models as follows:

For a rotary-wing AAV with a flight speed of v, its motion energy consumption [40] can be given by

$$
\begin{array}{c} P _ {A A V} (v) = P _ {B} \left(1 + \frac {3 v ^ {2}}{v _ {t i p} ^ {2}}\right) + \\ P _ {I} \left(\sqrt {1 + \frac {v ^ {4}}{4 v _ {0} ^ {4}}} - \frac {v ^ {2}}{2 v _ {0} ^ {2}}\right) ^ {1 / 2} + \frac {1}{2} d _ {0} \rho s A v ^ {3}, \end{array}\tag{3}
$$

where $P _ { B }$ and $P _ { I }$ represent the blade power and induced power of the AAV in a hovering state, respectively. Moreover, $v _ { t i p }$ denotes the tip speed of the rotor blades, while $v _ { 0 }$ represents the average induced rotor speed of the AAV in the hovering state. Additionally, $d _ { 0 }$ and $\rho$ are the body drag coefficient and air density, respectively. Meanwhile, s and

A represent the solidity and area of the rotor of the AAV, respectively.

For an SV with a travel speed of v and using a permanent magnet direct current (PMDC) motor model, its motion energy consumption [39] can be given by

$$
P _ {S V} (v) = k _ {1} v ^ {2} + k _ {2} v + k _ {3},\tag{4}
$$

where $k _ { 1 } , k _ { 2 }$ , and $k _ { 3 }$ are the respective control parameters.

Without loss of generality, we disregard the additional increase or decrease in energy consumption of the AAV and SV due to acceleration or deceleration during motion, as these account for only a small fraction of their total operating time.

## IV. PROBLEM FORMULATION AND ANALYSES

In this section, we analyze the collaborative charging problem of HAGCCS. First, we analyze several key factors involved in the charging phase. Second, we formulate and analyze the collaborative charging problem.

## A. Problem Statement

In this work, we focus on three optimization objectives, i.e., improving the charging efficiency of the AAV and SV, reducing the travel distance of the AAV and SV, and minimizing the mortality of the sensor nodes. These three optimization objectives involve inherent trade-offs. Specifically, if the AAV and SV are positioned closer to the sensor nodes, a larger number of nodes will fall within the charging range, thereby improving the charging efficiency. The location of the AAV and SV is directly related to their energy consumption, which means that if the positioning results in more frequent or longer travel of the AAV and SV, the energy consumption will increase accordingly. Moreover, improper positioning may lead to inadequate coverage of sensor nodes, thereby preventing some nodes from receiving sufficient charging support, which means that the node mortality increases.

As such, the corresponding decision variables are represented as $\begin{array} { r } { { \cal Z } _ { t } \ = \ \{ x _ { t } ^ { \hat { S V } } , y _ { t } ^ { S V } , x _ { t } ^ { A A V } , y _ { t } ^ { A A V } , h _ { t } ^ { A A V } \} } \end{array}$ , whose variables correspond to the coordinates of the AAV and SV.

In HAGCCS, we aim to enhance the charging efficiency of the AAV and SV to supply more energy to the sensor nodes, thereby extending the lifetime of WRSN. According to Eq. (2), the AAV or SV can charge all sensor nodes within the effective charging radius $d _ { m a x }$ . Therefore, the charging efficiency of the AAV or SV, which is the first optimization objective $f _ { 1 }$ , can be expressed as follows:

$$
f _ {1} = \sum_ {i = 1} ^ {N _ {S}} P _ {i}.\tag{5}
$$

By reducing the travel distance of the AAV and SV, the energy consumption caused by their travel distance is minimized. Therefore, more energy can be allocated for charging the sensor nodes, thereby effectively improving their energy utilization efficiency. Let $( x _ { \mathrm { i n i t } } , y _ { \mathrm { i n i t } } , z _ { \mathrm { i n i t } } )$ and $( x _ { \mathrm { t a r g e t } } , y _ { \mathrm { t a r g e t } } , z _ { \mathrm { t a r g e t } } )$ represent the initial and target positions of the AAV or SV, respectively, in a single movement, then the travel distance of the AAV or SV, which is the second optimization objective $f _ { 2 } ,$ can then be expressed as follows:

$$
f _ {2} = \sqrt {\left(x _ {\text {target}} - x _ {\text {init}}\right) ^ {2} + \left(y _ {\text {target}} - y _ {\text {init}}\right) ^ {2} + \left(z _ {\text {target}} - z _ {\text {init}}\right) ^ {2}}.\tag{6}
$$

The mortality of sensor nodes is a key indicator for evaluating the performance and efficiency of WRSNs. Specifically, an increase in sensor node mortality leads to deterioration in WRSN stability and reliability, while also reducing the integrity of collected data. As such, we consider minimizing the mortality of sensor nodes in this network as the third optimization objective. Specifically, the third objective $f _ { 3 } , i . e .$ the mortality of sensor nodes,

$$
f _ {3} = \frac {\sum_ {i = 1} ^ {N _ {S}} b _ {i}}{N _ {\mathcal {S}}},\tag{7}
$$

where $b _ { i }$ is a binary variable defined as follows:

$$
b _ {i} = \left\{ \begin{array}{l l} 0, & \text { if   sensor   node   } i \text {   is   alive } \\ 1, & \text { if   sensor   node   } i \text {   is   dead } \end{array} \right..\tag{8}
$$

Note that $f _ { 3 }$ is a shared network-level term applied to both the AAV and SV, whereas $f _ { 1 }$ and $f _ { 2 }$ are agent-specific terms that reflect the individual charging efficiency and travel distance of each agent. More importantly, $f _ { 3 }$ is designed to encourage cooperative coverage between the AAV and SV and to incentivize both agents to coordinate proactively so as to minimize coverage gaps and maximize the overall lifetime.

To improve the charging efficiency, the AAV and SV need to move frequently between sensor nodes that need to be charged, which results in an increase in their travel distance. However, as the travel distances of the AAV and SV increase, their energy consumption also rises, which means that they cannot charge more sensors. As a result, the mortality of sensor nodes will increase. Therefore, three optimization objectives have a conflicting relationship. Thus, we formulate this problem by using multi-objective optimization theory.

According to the three optimization sub-objectives above, our optimization problem can be formulated as follows:

$$
\text {(P1)}: \quad \max _ {\mathbf {Z} _ {t}} \sum_ {t = 1} ^ {\mathcal {T}} (f _ {1}, - f _ {2}, - f _ {3}),\tag{9a}
$$

$$
\mathrm{s.t.} \qquad 0 \leq x _ {t} ^ {A A V} \leq X _ {m a x}, \quad \forall t \in \mathcal {T}\tag{9b}
$$

$$
0 \leq y _ {t} ^ {A A V} \leq Y _ {m a x}, \quad \forall t \in \mathcal {T}\tag{9c}
$$

$$
0 \leq x _ {t} ^ {S V} \leq X _ {m a x}, \quad \forall t \in \mathcal {T}\tag{9d}
$$

$$
0 \leq y _ {t} ^ {S V} \leq Y _ {m a x}, \quad \forall t \in \mathcal {T}\tag{9e}
$$

where $X _ { m a x }$ and $Y _ { m a x }$ represent the maximum ranges of the WRSN area along the x-axis and y-axis, respectively. Moreover, the boundary constraints (9b)-(9c) and (9d)-(9e) ensure that both the AAV and SV operate within the WRSN boundaries, respectively.

## B. Problem Analyses

Based on the HAGCCS and the optimization sub-objectives, problem (P1) exhibits the following characteristics. Firstly, problem (P1) exhibits strong dynamic and stochastic characteristics. Specifically, the energy consumption magnitude of sensors varies randomly, thereby making the current overall network sensor energy consumption level unpredictable, which makes it challenging to dynamically capture critical state features, thus demonstrating strong dynamic properties. Moreover, given the limited energy budgets carried by the AAV and SV, the uncertainty in their travel distances makes their energy consumption stochastic, thereby reducing the energy available for charging the WRSN. Secondly, this problem involves both long-term and short-term optimization objectives. In particular, the long-term objective is to maximize the WRSN lifetime, while the short-term objective is to minimize the energy consumption of the AAV and SV within each time slot. Therefore, during the optimization process, we should consider both the current and long-term interests. Finally, since the AAV is an energy-sensitive system that requires real-time decisionmaking during flight operation, the solution used to solve this problem should satisfy real-time computational requirements.

Accordingly, the multi-objective optimization problem (P1) exhibits dynamic characteristics, long-term slot properties, and real-time decision-making requirements. Thus, conventional optimization methods or evolutionary computation algorithms are unsuitable for this problem. Specifically, conventional optimization methods typically rely on a known and fixed environment model [14]. Even if heuristic or evolutionary algorithms are used, they are often predefined or require a considerable amount of time to run, which prevents realtime adjustments in practical applications [41]. Moreover, these methods generally focus on immediate optimization and struggle to balance both current and long-term benefits. Though conventional methods may maximize short-term gains, they overlook the sustainability of long-term network performance and stability. Furthermore, due to the limited computing capability of their onboard devices and the constrained energy budgets of the AAV and SV, Pareto-based methods, which require high computational overhead to compute a set of nondominated solutions, are not suitable for solving this multi-objective problem. Therefore, a faster and more energyefficient approach that better aligns with the goal of extending the WRSN lifetime is needed to solve problem (P1).

Accordingly, we adopt the advantageous DRL to address the considered problem. Specifically, DRL enables adaptive decision-making in dynamic environments and optimizes longterm network performance by learning from real-time feedback, thereby making it well-suited for problem (P1) in HAGCCS.

## V. HETEROGENEOUS TRUST REGION STRATEGY OPTIMIZATION-BASED DECENTRALIZED SOLUTION

In this section, we propose a decentralized solution to address the collaborative charging problem (P1) in the WRSN. Firstly, we formulate the optimization problem as a Markov game (MG) [42] involving the AAV and SV agents. Secondly, we introduce the IHATRPO algorithm that integrates a selfattention mechanism and Beta sampling to enhance multiagent coordination. Finally, we analyze the computational and space complexity of the proposed algorithm.

## A. Markov Game Formulation

We first model Problem (P1) as a MG. Specifically, MG can be formally represented by the tuple $\langle \mathcal { N } , \{ \boldsymbol { S } _ { i } \} _ { i \in \mathcal { N } } , \{ \mathcal { A } _ { i } \} _ { i \in \mathcal { N } }$ $\mathcal { P } , \{ R _ { i } \} _ { i \in \mathcal { N } } , \gamma \rangle$ . The key elements of MG are given as follows:

1) Agent Set: The HAGCCS employs two agents that are assigned to control the AAV and SV, respectively, i.e.,

$$
\mathcal {N} = \{A ^ {A A V}, A ^ {S V} \}.\tag{10}
$$

At each time slot t, both agents independently observe the environmental state and execute actions, so as to maximize their respective expected total rewards.

2) State Space: Both agents share the same global state space, which can ensure complete environmental observability for decision-making. Specifically, the state space consists of the energy levels and positions of sensor nodes, which can be collected by BS via existing WRSN communication protocols, and positions of the AAV and SV, which can be acquired via the global positioning system (GPS). Moreover, the acquisition of such state information through the inherent communication protocols in WRSNs incurs no significant additional communication overhead. Therefore, the state space is defined as follows:

$$
\mathcal {S} = \left\{s _ {t} \mid s _ {t} = \left(\mathcal {S} _ {t}, A A V _ {t}, S V _ {t}\right), \forall t \in \mathcal {T} \right\},\tag{11}
$$

where ${ \mathcal S } _ { t } \ = \ \{ x _ { t } ^ { 1 } , x _ { t } ^ { 2 } , . . . , x _ { t } ^ { N _ { S } } , y _ { t } ^ { 1 } , y _ { t } ^ { 2 } , . . . , y _ { t } ^ { N _ { S } } , q _ { t } ^ { 1 } , q _ { t } ^ { 2 } , . . . , q _ { t } ^ { N _ { S } } \}$ represents the set of coordinates, and current energy levels of each sensor node at the beginning of time slot t. Meanwhile, $A A V _ { t } ~ = ~ \{ x _ { t } ^ { A A V } , y _ { t } ^ { A A V } , \bar { h } ^ { A A V } \}$ and $S V _ { t } ~ = ~ \{ x _ { t } ^ { S V } , y _ { t } ^ { S V } \}$ denote the coordinates of the AAV and SV, respectively, at the start of time slot $t .$

3) Action Space: Each agent operates within its own action space, representing distinct decision variables for controlling vehicle motion parameters. Both the AAV and SV agents follow the same mathematical formulation while maintaining independent control over their respective vehicles. Based on environmental observations, each agent governs two critical motion parameters, which are the heading angle θ and travel distance d. Note that these two parameters can correspond to the decision variable $\scriptstyle { Z _ { t } }$ of problem (P1). Consequently, the action space for each agent is defined as:

$$
\mathcal {A} _ {i} = \{a _ {t} ^ {i} | a _ {t} ^ {i} = (\theta_ {t} ^ {i}, d _ {t} ^ {i}), \forall t \in \mathcal {T}, i \in \mathcal {N} \}.\tag{12}
$$

4) Reward Function: The reward mechanism is designed to motivate both agents to optimize their respective contributions to the HAGCCS performance. Each agent receives individual rewards based on its performance, with both agents sharing the same mathematical reward structure to ensure consistency and fairness in the learning process. Specifically, we combine three optimization sub-objectives, $i . e . ,$ , charging efficiency, energy consumption represented by travel distance, and network sustainability measured by node mortality, into a scalarized reward function through a weighted sum, which enables faster solution finding in WRSN charging scenarios with limited computing resources and energy budgets. The reward function is defined as follows:

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 1: IHATRPO

Input: Number of heterogeneous agents n, Max training episodes max_episodes, max time slots max_time_slots

Output: Optimized policy network parameters  $\{\theta_{i}\}_{i=1}^{n}$ 

/* Initialization: */

1 for agent  $i \in [1, n]$  do

2 Initialize Actor network parameters  $\theta_{i}$  and Critic network parameters  $\omega_{i}$ 

3 end

4 for episode = 1 to max_episodes do

5 Reset sensor nodes, initialize power levels for AAV/SV, initialize trajectory buffer D

6 for t = 1 to max_time_slots do

7 for agent  $i \in [1, n]$  do

8 Agent i constructs Beta distribution from state  $s_{t}$ , samples action  $a_{i}^{t}$ 

9 Execute action  $a_{i}^{t}$ , receive reward  $r_{i}^{t}$ 

10 end

11 Update environment state  $s_{t} \rightarrow s_{t+1}$ 

12 for agent  $i \in [1, n]$  do

13 Store transition ( $s_{t}, a_{i}^{t}, s_{t+1}, r_{i}^{t}$ ) in trajectory buffer D

14 end

15 if  $E_{AAV} \leq 0$  and  $E_{SV} \leq 0$  then

16 break

17 end

18 end

/* Policy Update: */

19 for agent  $i \in [1, n]$  do

20 Compute generalized advantage estimation (GAE) advantages  $\hat{A}^{\pi_{\theta_{i}}}$  from D and normalize

21 Update  $\omega_{i}$ 

22 Update  $\theta_{i}$  via TRPO using Eq. (17)

23 end

24 end

25 Return  $\theta = \{\theta_{1}, \ldots, \theta_{n}\}$ .
</div>

$$
\mathcal {R} _ {i} = \left\{r _ {t} ^ {i} \mid r _ {t} ^ {i} = \lambda_ {1} f _ {1, t} ^ {i} - \lambda_ {2} f _ {2, t} ^ {i} - \lambda_ {3} f _ {3, t}, \forall t \in \mathcal {T}, i \in \mathcal {N} \right\},\tag{13}
$$

where $f _ { 1 , t } ^ { i } , \ f _ { 2 , t } ^ { i }$ , and $f _ { 3 , t }$ correspond to Eq. (5), Eq. (6), and Eq. (7) during time slot t. The weighting coefficients $\lambda _ { 1 }$ $\lambda _ { 2 } ,$ , and $\lambda _ { 3 }$ serve as balancing factors that ensure appropriate emphasis on the relative importance of each reward component in the overall system performance. Furthermore, we normalize the three sub-objectives during the weighted sum process using $\lambda _ { 1 } , \lambda _ { 2 } .$ , and $\lambda _ { 3 } ,$ , respectively, whose values will be further analyzed in Section VI-B1.

## B. IHATRPO Algorithm

In this section, we handle the MG through the IHATRPO algorithm, where the AAV and SV are each treated as an agent. In the following, we first introduce the conventional HATRPO. Subsequently, we present two improvement measures, namely a self-attention mechanism and Beta sampling, to enhance the ability of HATRPO to handle the MG.

1) Preliminaries of HATRPO: HATRPO integrates the multi-agent framework with trust region policy optimization to enhance multi-agent DRL (MADRL), thus achieving monotonic improvement.

In an N-agent MG, the joint policy $\pi ~ = ~ \left( \pi _ { 1 } , \ldots , \pi _ { N } \right)$ represents collective decision-making of agents. Specifically, at time slot t, given state $s ^ { t } .$ , each agent takes an action $a _ { i } ^ { t }$ according to its policy. Subsequently, the environment computes the reward $\pmb { r } ^ { t } = ( r _ { 1 } ^ { t } , \ldots , t _ { N } ^ { t } )$ based on the joint action $\textbf { \em a } ^ { t } ~ = ~ ( a _ { 1 } ^ { t } , \ldots , a _ { N } ^ { t } )$ and updates the state to $s ^ { t + 1 }$ The optimization goal is to maximize expected cumulative reward by updating policy parameters from $\theta _ { i }$ to $\theta _ { i } ^ { \prime } ,$ where the objective function difference from the policy update is given by

$$
J (\theta_ {i} ^ {\prime}) - J (\theta_ {i}) = \mathbb {E} _ {\tau \sim \pi} \left[ \sum_ {t = 0} ^ {\infty} \gamma^ {t} A ^ {\pi_ {\theta_ {i}}} (s _ {i} ^ {t}, a _ {i} ^ {t}) \right],\tag{14}
$$

where τ is the trajectory, $\gamma \in ( 0 , 1 )$ is the discount factor, and $A ^ { \pi _ { \theta _ { \zeta } } }$ is the advantage function under policy $\pi _ { \boldsymbol { \theta } _ { i } }$ . However, since the updated policy $\pi _ { \theta _ { i } ^ { \prime } }$ cannot be computed directly, we approximate the objective function using the state distribution of the pre-update policy $\pi _ { \boldsymbol { \theta } _ { i } }$ and apply importance sampling to correct the action distribution. The objective is then given by

$$
L \left(\theta_ {i} ^ {\prime} \mid \theta_ {i}\right) = \mathbb {E} _ {s _ {i} \sim \nu^ {\pi}} \mathbb {E} _ {a _ {i} \sim \pi_ {\theta_ {i}} (\cdot | s _ {i})} \left[ \frac {\pi_ {\theta_ {i} ^ {\prime}} (a _ {i} \mid s _ {i})}{\pi_ {\theta_ {i}} (a _ {i} \mid s _ {i})} A ^ {\pi_ {\theta_ {i}}} (s _ {i}, a _ {i}) \right]. \tag {1}\tag{15}
$$

To maintain proximity between the updated and original policies, we adopt the Kullback-Leibler (KL) divergence within the trust region policy optimization framework [43]. Specifically, the divergence between the pre-update policy $\pi _ { \boldsymbol { \theta } _ { i } }$ and post-update policy $\pi _ { \theta _ { i } ^ { \prime } }$ is denoted by $D _ { K L } ( \pi _ { \theta } | | \pi _ { \theta _ { i } ^ { \prime } } )$ . By setting δ as the update step size threshold, we formulate the optimization problem as:

$$
\begin{array}{l} \max _ {\theta_ {i} ^ {\prime}} L (\theta_ {i} ^ {\prime} | \theta_ {i}) \\ \text {s.t.} \mathbb {E} _ {s _ {i} \sim \nu^ {\pi}} \left[ D _ {K L} (\pi_ {\theta_ {i}} | | \pi_ {\theta_ {i} ^ {\prime}}) \right] \leq \delta . \end{array}\tag{16}
$$

To simplify the computation, we apply linear and quadratic approximations to the objective function and KL constraint, respectively, thereby yielding the closed-form update as follows:

$$
\theta_ {i} ^ {k + 1} = \theta_ {i} ^ {k} + \alpha^ {j} \sqrt {\frac {2 \delta}{(g _ {i}) ^ {T} (H _ {i}) ^ {- 1} g _ {i}}} H _ {i} ^ {- 1} g _ {i},\tag{17}
$$

where $\theta _ { i } ^ { k }$ represents the policy parameters after the k-th iteration of the i-th agent, $\alpha ^ { j } ~ \in ~ ( 0 , 1 )$ is the backtracking line search coefficient, which ensures that $\theta _ { i } ^ { \prime }$ is superior to $\theta _ { i } ^ { k }$ and satisfies the KL divergence constraint. Moreover, $g _ { i } =$ $\begin{array} { r } { \check { \nabla } _ { \theta _ { i } ^ { \prime } } \mathbb { E } _ { s _ { i } \sim \nu ^ { \pi } } \mathbb { E } _ { a _ { i } \sim \pi _ { \theta ^ { k } } ( \cdot \vert s _ { i } ) } \big [ \pi _ { \theta _ { i } ^ { \prime } } \big ( a _ { i } \vert s _ { i } \big ) / \pi _ { \theta _ { i } ^ { k } } \big ( a _ { i } \vert s _ { i } \big ) A ^ { \pi _ { \theta _ { i } ^ { k } } } \big ( s _ { i } , a _ { i } \big ) \big ] } \end{array}$ is the gradient of the optimization objective, and $\begin{array} { r l } { H _ { i } } & { { } = } \end{array}$ $\mathcal { H } \left[ \bar { \mathbb { E } _ { s _ { i } \sim \nu ^ { \pi } } } \left[ D _ { K L } \left( \pi _ { \theta _ { i } } \middle | \middle | \bar { \pi _ { \theta _ { i } ^ { \prime } } } \right) \right] \right]$ represents the Hessian matrix derived from the KL divergence.

2) Self-Attention Mechanism: In HAGCCS, heterogeneous charging agents must simultaneously process multidimensional state information, including their own states, distributed sensor node conditions, and inter-agent coordination requirements within a non-stationary environment. Conventional MADRL approaches treat all state information equally through conventional feature extraction, thereby failing to capture varying feature importance and dynamic relationships between the AAV and SV and sensor nodes, which leads to suboptimal decision-making.

The self-attention mechanism addresses these limitations by dynamically assigning importance weights to input elements based on contextual relevance. Different from traditional approaches, the self-attention mechanism captures complex dependencies through parallel processing while adaptively focusing on critical information for decision-making. In our IHATRPO, we integrate the self-attention mechanism into the actor networks of both AAV and SV agents for heterogeneous multi-agent coordination. Specifically, the self-attention mechanism [44] computes context-aware representations by measuring similarity between input elements using Query (Q), Key (K), and Value (V) vectors, which can be given by

$$
A (Q, K, V) = \sigma \left(\frac {Q K ^ {T}}{\sqrt {d _ {k}}}\right) V,\tag{18}
$$

where $d _ { k }$ represents the dimension of K. By using the self-attention mechanism integration, heterogeneous agents dynamically prioritize relevant information based on context and achieve a deep understanding of state interdependencies for informed decision-making. Note that since the policy is trained offline and deployed in a fixed network topology, this complexity does not constitute a significant bottleneck during deployment.

![](images/41bd4fa4707f305b09f1cc5590819bed145b5261abe69e08a62e6e25cc4ff475.jpg)  
Fig. 3. Boundary effects on Gaussian distribution bias. The shaded areas represent probability mass falling outside the valid action range, which must be truncated during sampling.

3) Beta Sampling: The HAGCCS requires continuous action control for the AAV and SV travel within bounded action spaces constrained by the finite distribution range of the WRSN. However, conventional continuous control methods utilize the Gaussian distribution for action sampling, whose unbounded nature conflicts with the bounded action spaces in our work, thereby resulting in boundary effects and distributional bias that compromise gradient computation accuracy, as demonstrated in Fig. 3.

In this case, the Beta distribution addresses these limitations through its inherent bounded property on [0, 1], thereby ensuring all sampled actions remain within valid ranges without truncation. Unlike the Gaussian distribution that requires clipping or rescaling, thereby resulting in computation bias, Beta distributions naturally maintain unbiased gradient computation while respecting action space constraints [45]. The probability density function of the Beta distribution is given by

$$
f (x; \alpha_ {b}, \beta_ {b}) = \frac {\Gamma (\alpha_ {b} + \beta_ {b})}{\Gamma (\alpha_ {b}) \Gamma (\beta_ {b})} x ^ {\alpha_ {b} - 1} (1 - x) ^ {\beta_ {b} - 1},\tag{19}
$$

where $\Gamma ( \cdot )$ is the Gamma function, and $\alpha _ { b }$ and $\beta _ { b }$ serve as shape parameters that collectively determine the distribution shape. We adopt $\pi _ { \theta } ( a | s ) = f ( c \cdot a ; \alpha _ { b } , \beta _ { b } )$ to characterize the stochastic policy, which is referred to as the Beta sampling strategy. The parameters $\boldsymbol { \alpha } _ { b } ~ = ~ \boldsymbol { \alpha } _ { b , \theta } ( \boldsymbol { s } )$ and $\beta _ { b } ~ = ~ \beta _ { b , \theta } ( s )$ are modeled by a neural network parameterized by θ. The parameter c is determined based on the value ranges of travel direction and distance for the AAV or SV in the action space, thereby ensuring that action outputs satisfy their respective action space constraints.

Through Beta sampling implementation, the agents maintain unbiased gradient computation within bounded action spaces, eliminate boundary effects that degrade policy performance, and ensure natural action space compliance without additional constraints or post-processing steps.

## C. Main Steps and Complexity Analysis of IHATRPO

Based on the methods introduced above, we propose the IHATRPO framework to address the multi-objective problem (P1). The overall structure of IHATRPO is depicted in Fig. 4, and the detailed procedure is outlined in Algorithm 1.

As shown in Fig. 4, IHATRPO treats the AAV and SV as two independent agents that are trained and executed separately. Specifically, in each time slot, the AAV and SV agents each receive the global state of the environment and extract key features through the self-attention mechanism. Based on these features, each agent samples a bounded continuous action from the Beta sampling strategy to determine its movement direction and travel distance for the next step. Following this sequential decision process, rewards generated for the AAV and SV agents from the environment and state transitions are stored in the trajectory buffer. At the end of each episode, the collected trajectories are used to update the critic networks and optimize the actor networks through trust-region constrained policy updates for the AAV and SV agents. By repeating this process, IHATRPO gradually optimizes the AAV and SV as heterogeneous agents that collaboratively provide charging services for the WRSN.

More notably, the IHATRPO framework follows the centralized training and centralized execution paradigm because both the AAV and SV agents require the global state information, including the energy levels and positions of all sensor nodes as well as positions of the AAV and SV, so as to coordinate effectively in providing wireless charging services for the WRSN.

We analyze the computational and space complexity of the IHATRPO algorithm during both the training and execution phases. The computational complexity of IHATRPO during the training phase is $\mathcal { O } ( N _ { A } ( | \pmb { \theta } | + | \pmb { \omega } | + N _ { E } ( T ( V + N ^ { 2 } h ) +$ $\left| \omega \right| + N _ { T } ( N _ { K } + N _ { M } N ^ { 2 } h ) + \left| \pmb { \theta } \right| ( N _ { K } + N _ { M } N ^ { 2 } h ) ) )$ , which can be summarized as follows:

• Network Initialization: This phase involves the initial ization of network parameters of the AAV and SV.

![](images/0c0472a83ba498d8e796ac84380e08c8d47b2eb8c0884339c55fd9d54d45abdd.jpg)  
Fig. 4. Framework of IHATRPO for heterogeneous air-ground collaborative charging in the WRSN. The algorithm integrates Beta distribution-based action sampling, self-attention mechanism-enhanced state processing in actor networks for the AAV and SV to optimize multi-objective charging strategies.

Specifically, the computational complexity is expressed as $\mathcal { O } ( N _ { A } ( | \pmb { \theta } | + | \pmb { \omega } | ) )$ , where $N _ { A }$ is the number of agents, the | · | operation represents the number of parameters in the networks.

• Action Selection: This phase entails selecting actions according to feature extraction of the self-attention mechanism and the output scores of the actor network, and corresponding complexity is $\mathcal { O } ( N _ { A } N _ { E } T N ^ { 2 } h )$ , where N is the number of sensor nodes and h is the embedding dimension of the self-attention mechanism. Moreover, $N _ { E }$ denotes the number of training episodes, and $T$ is the number of steps per episode.

• Reward Calculation and State Transitions: The computational complexity of reward calculation and state transitions is $\mathcal { O } ( N _ { A } N _ { E } T V )$ , where V represents the complexity of interacting with the environment.

• Network Update: The updating phase consists of two main parts that are the updates of the critic networks, as well as the updates of the actor networks. First, the advantage function is calculated, and the critic network parameters are updated subsequently. This part has the complexity of $\mathcal { O } ( N _ { A } N _ { E } ( | \omega | + N _ { T } ) )$ , where $N _ { T }$ is the length of the sampled training data. Second, the actor network is updated by calculating the target value of the surrogate function, calculating the conjugate gradient, and linearly searching for parameters that meet the conditions. Therefore, the corresponding complexity is $\mathcal { O } ( N _ { A } N _ { E } ( N _ { T } ( N _ { K } + N _ { M } N ^ { 2 } h ) + \vert \pmb { \theta } \vert ( N _ { K } + N _ { M } N ^ { 2 } h ) ) )$ where $N _ { K }$ is the number of iterations of the conjugate gradient and $N _ { M }$ is the number of iterations of the linear search. Thus, the complexity of this phase is calculated as $\mathcal { O } ( N _ { A } N _ { E } ( | \omega | + \bar { N } _ { T } ( \bar { N _ { K } } + N _ { M } \bar { N } ^ { 2 } h ) + | \theta | ( N _ { K } +$

$$
N _ {M} N ^ {2} h)))).
$$

Note that, as for the computational complexity, the network update phase dominates the computational complexity during training, particularly the processes of conjugate gradient computation and linear search combined with the attention mechanism for optimizing the actor network parameters.

Besides, the space complexity of IHATRPO during the training phase is $\mathcal { O } ( N _ { A } ( | \pmb { \theta } | + | \pmb { \omega } | ) + | \mathcal { D } | ( | \pmb { s } | + \pmb { a } )$ , where |D| denotes the size of the trajectory buffer. As such, the space complexity is mainly for storing neural network parameters and sampled trajectories.

During the evaluation phase, the computational complexity of IHATRPO is $\mathcal { O } ( N _ { A } N _ { E } N ^ { 2 } h )$ , which can be attributed to action selection and transition according to the current state using the feature and actor network. Moreover, the space complexity during the execution phase is $N _ { A } | \pmb \theta |$ since the feature and actor network parameters need to be stored in memory for action selection.

## VI. SIMULATIONS AND ANALYSES

In this section, we first introduce the simulation settings and baselines. Subsequently, we present the optimization results, the comparison analyses with state-of-the-art baselines, and the analysis of agent spatial movement patterns.

## A. Simulation Setups

1) Scenario and Algorithm Setups: In the simulations, we consider the scenario that the AAV and SV provide wireless charging to a sensor network. The primary parameters are shown in Table II. Additionally, following the methodology in [46], we set the charging efficiency parameters α and $\beta$ in

Eq. (2) to 36 and 30, respectively. The energy consumption rate of sensor nodes per round is randomly generated within the range of 0.025 J to 0.04 J.

In the proposed IHATRPO, the algorithm parameters are shown in Table II. Both the policy network and value network are configured with two hidden layers, each containing 256 neurons. Meanwhile, in the self-attention mechanism, the number of heads is set to 4, and the embedding dimension is set to 256. Additionally, we set the number of training iterations to $6 . 5 \times 1 0 ^ { 5 }$ and employ the Adam optimizer for neural network updates. Note that these algorithm parameters are determined by careful tuning to ensure performance and convergence. We consider the heterogeneity between the AAV and SV by assigning different reward weight coefficients, which are determined experimentally and shown in Table II.

TABLE II  
SIMULATION SETTINGS

<table><tr><td>Parameters</td><td>Values</td></tr><tr><td>Network area</td><td> $100 \times 100 \, m^{2}$ </td></tr><tr><td>Number of sensor nodes</td><td>100</td></tr><tr><td>Transmit power of AAV and SV</td><td>3 W [47]</td></tr><tr><td>Reception threshold of the sensor node</td><td>5 mW</td></tr><tr><td>The max energy of the sensor node</td><td>2 J</td></tr><tr><td>The charging radius of AAV and SV</td><td>6 m</td></tr><tr><td>Learning rate of neural network</td><td> $5 \times 10^{-5}$ </td></tr><tr><td>KL threshold</td><td> $5 \times 10^{-5}$ </td></tr><tr><td>Linear search step</td><td>0.5</td></tr><tr><td>GAE scaling factor  $\lambda$ </td><td>0.98</td></tr><tr><td>Entropy coefficient</td><td>0.01</td></tr><tr><td>Discount factor</td><td>0.96</td></tr><tr><td>Time step of each episode</td><td>200</td></tr><tr><td> $\lambda_{1}, \lambda_{2}, \text{and } \lambda_{3} \text{ for AAV}$ </td><td>1, 0.001, and 1</td></tr><tr><td> $\lambda_{1}, \lambda_{2}, \text{and } \lambda_{3} \text{ for SV}$ </td><td>1, 0.03, and 0.1</td></tr></table>

2) Baselines: To demonstrate the superiority of the proposed IHATRPO, we introduce the following comparative baselines. Note that these baselines adopt the same parameters as mentioned above and integrate the schedule policy of the AAV and SV.

• PPO: PPO is a policy gradient method that improves training stability through clipped surrogate objectives [48]. As a single-agent baseline, PPO treats the multi-agent environment as a stationary single-agent MDP by training each agent independently, therefore ignoring the non-stationary nature caused by other learning agents.

• DDPG: DDPG is an actor-critic method designed for continuous control tasks that combines policy gradient methods with Q-learning [49]. When applied to multiagent settings, each agent is trained independently using DDPG and treats other agents as part of the environment dynamics without explicit coordination mechanisms.

• MADDPG: MADDPG is a DDPG-based classical MADRL approach based on the centralized training and decentralized execution architecture [50]. This baseline allows agents to access global information during training while maintaining individual policies and shows effectiveness in multi-agent continuous control tasks.

• Heterogeneous-Agent PPO (HAPPO): HAPPO adapts PPO for heterogeneous multi-agent settings where agents have different observation and action spaces [51]. This method serves as a baseline given its capability to handle heterogeneous AAV-SV coordination.

• HATRPO: HATRPO extends TRPO to a heterogeneous multi-agent environment by maintaining individual trust regions for each agent [51]. Furthermore, the details of this approach are elaborated in Section V-B1. The implementation ensures stable policy updates through KL divergence constraints across diverse agents.

As such, the comparisons with PPO and DDPG demonstrate the necessity of multi-agent coordination mechanisms, the comparison with MADDPG shows the effectiveness of handling different types of agents, the comparison with HAPPO illustrates the superiority of the HATRPO-based framework in handling heterogeneous multi-agent scenarios, and the comparison with HATRPO can assess the effectiveness of two improvement measures of IHATRPO. In the following analyses, we first present the performance of multiple optimization sub-objectives under the IHATRPO, and then conduct a comparative analysis of convergence performance and total reward feedback between these baselines and IHATRPO, and the following analysis of agent trajectories.

## B. Performance Evaluation

1) Optimization Results: Fig. 5(a) shows the respective cumulative reward of the AAV and SV, Fig. 5(b), Fig. 5(c), and Fig. 5(d) illustrate optimization of objectives in terms of the charging efficiency (f<sub>1</sub>), travel distance $( f _ { 2 } )$ of the AAV and SV, and the mortality rate $( 1 0 0 \times f _ { 3 } \% )$ of sensor nodes. As can be seen, the AAV and SV agents exhibit similar convergence trends and converge after approximately 200k iterations in Fig. 5(a), which demonstrates that IHATRPO, which employs the heterogeneous optimization framework, achieves good optimization performance for heterogeneous agents. Moreover, each objective achieves good optimization results with increasing training episodes in Figs. 5(a), (b), and (c), which demonstrates that the proposed reward function in Eq. (13) can better balance the relationship between the AAV and SV. Moreover, it is noteworthy that a significant reduction in sensor node mortality from an initial rate exceeding 90% to below 10% in Fig. 5(d), which indicates that through the scheduling of the AAV and SV, the sensor node mortality can be reduced and HAGCCS achieves better energy efficiency.

Furthermore, to validate the heterogeneous weight assignment, we compare three configurations in Fig. 7. As shown in Fig. 7, the proposed heterogeneous assignment achieves more stable convergence and optimization results on the travel distance and the mortality rate of sensor nodes, thereby confirming the necessity of agent-specific weight configuration.

2) Comparison Results: Fig. 6 illustrates the cumulative rewards for each episode of IHATRPO in comparison to other benchmark algorithms. As can be seen, IHATRPO achieves faster convergence speed and the highest reward. This can be explained by several factors. First, the selfattention mechanism enables IHATRPO to dynamically prioritize relevant information and capture complex dependencies among multi-dimensional states. Second, the Beta sampling provides naturally bounded action sampling complying with the HAGCCS characteristics for IHATRPO. While HATRPO demonstrates the fastest convergence performance in the initial phase, this algorithm achieves lower cumulative rewards after convergence due to its limited capability in processing highdimensional information and coordinating between heterogeneous agents. Among the single-agent baselines, PPO shows better convergence than DDPG, but both struggle with the multi-agent coordination challenges for convergence. MAD-DPG fails to account for the heterogeneity between the AAV and SV, thereby resulting in slower convergence and lower reward. HAPPO and HATRPO exhibit slower convergence due to action boundary violations caused by Gaussian sampling.

![](images/15676a28b768b0538a9ae2389fc5a89b13ef744655f1f390e229de64258f9283.jpg)  
(a)

![](images/491cc19e1cb4cf02550b5b3843afabe29ee09ab80fe640ac2a2386ec70269c1b.jpg)  
(b)

![](images/a02e516cfccb639ff1c2ae3d59e5688ea8c127e77873b913587a5d4e8f0364cc.jpg)  
(c)

![](images/2770f6dc45ececc6672ee7180d3306d9020488f66ee9d45b38fbafad452fc427.jpg)  
(d)  
Fig. 5. Visualization results obtained by IHATRPO. (a) The total reward of the AAV and SV, respectively. (b) The charging efficiency of the AAV and SV. (c) The travel distance of the AAV and SV. (d) The mortality of sensor nodes (100 × f<sub>3</sub>%).

![](images/1946806c1a43fa2c80950a19317779f3b8e165a054b0978ae3aaf704cd107935.jpg)  
Fig. 6. Convergence performance comparison of PPO, DDPG, MADDPG, HAPPO, HATRPO, IHATRPO.

The performance improvement over the original HATRPO particularly validates that the integration of the self-attention mechanism and Beta sampling strategy effectively enhances policy optimization capability, thereby achieving superior learning performance in the heterogeneous multi-agent collaborative charging scenario.

3) Sensitivity Analysis: To further evaluate the robustness of IHATRPO, we conduct sensitivity analyses from multiple dimensions, across the density of sensor nodes, settings of the AAV and SV, and the settings in energy and position of sensor nodes. The details are as follows:

Firstly, we set the number of sensor nodes $N ~ \in ~ \{ 5 0 , 1 0 0 , 2 0 0 \}$ and the side length of WRSN area {100, 150, 200} m separately, thereby testing the impact of densities of sensor nodes. As shown in Figs. 8 and 9, an appropriate sensor node density is beneficial for IHATRPO to reduce the mortality rate of sensor nodes. On the one hand, when the density is too high, although the charging efficiency of the AAV and SV improves, the AAV and SV face challenges in providing sufficient energy or timely charging services to all sensor nodes. On the other hand, when the density is too low, the AAV and SV are required to travel greater distances, thereby resulting in higher energy consumption for mobility while making it increasingly difficult to meet the charging demands of distant sensor nodes in time. However, IHATRPO achieves good optimization results in the mortality rate of sensor nodes across all settings.

Secondly, we set the charging radius {4, 6, 8} m and the energy budget {50%, 75%, 100%} of the default energy budget of the AAV and SV separately, thereby testing the impact of settings of the AAV and SV. As shown in Figs. 10 and 12, a small charging radius and insufficient energy budget both degrade charging coverage and increase node mortality, whereas radius 6 m and enough energy budget yield satisfactory optimization performance.

Thirdly, we set the initial energy configuration, i.e., Gaussian distribution, uniform random distribution, and fixed 1.5 J, and the position configuration, i.e., random, uniform, and clustered deployment scenarios of sensor nodes separately, thereby testing the impact of settings of sensor nodes. As shown in Fig. 11, IHATRPO demonstrates robustness to energy heterogeneity across all settings. As for the position distribution of sensor nodes, as shown in Fig. 13(c), when sensor nodes form only a small number of clusters within the WRSN area, the SV is unable to move between these clusters in time to respond to charging demands, and consequently tends to remain stationary, consistent with the observations in Fig. 8(c) and Fig. 9(c). However, IHATRPO achieves good optimization results in the mortality rate of sensor nodes across all position settings of sensor nodes.

Moreover, we set different seeds {0, 1, 3, 42} and report the 95% confidence interval of the averaged total reward, as shown in Figs. 14 and 15, thereby confirming the strong reproducibility of IHATRPO.

4) Ablation Analysis: Fig. 16 presents the contribution of each proposed component in IHATRPO. Specifically, we examine the effects of removing the self-attention mechanism and the Beta sampling strategy, respectively. As can be seen, the complete IHATRPO achieves the highest total reward value and demonstrates stable convergence, thereby highlighting the synergistic effect of its components. When the self-attention mechanism is removed, slower convergence suggests that the integration of the self-attention enhances the capability to extract and integrate critical state information from the complex environment. Similarly, the removal of the

![](images/eedbb12e5b4ae77ebf3962f62e65a4ebb61ff3d7f260a278af7788ccab7a53c4.jpg)  
(a)

![](images/842b0c6e639b67460e4873175ad366f7b9205592a85fa7accd554d921bd5d5d3.jpg)  
(b)

![](images/1c72379cb4b16da425b9040ed82b2dce06c1d389227fb62fc0048e7c5333e1bb.jpg)  
(c)

![](images/29b96fe064bef4e76271ea7d4a3b88e59e4a512cca286baeb25f213223172884.jpg)  
(d)

![](images/899c0d4015b8bbfd1b07e846684feb9729e6d1d4b7eefa44328dd9aa57b5e889.jpg)  
(e)  
Fig. 7. Results of IHATRPO under different weights of the reward function. (a) The charging efficiency for SV. (b) The charging efficiency for AAV. (c) The travel distance for SV. (d) The travel distance for AAV. (e) The mortality of sensor nodes (100 × f<sub>3</sub>%).

![](images/770331491ec380a1b4487db7a6cd1dfbbde7db4efc5f6648320d3cb77c938a28.jpg)  
(a)

![](images/dde8e99cd1f73bbb31cc73e50a833100760ecd6c76557fa3d518accdc62cf431.jpg)  
(b)

![](images/640b1e278c5618578fcab7b063b56747638e17f96e71f59efca5170311e068fe.jpg)  
(c)

![](images/5d09cb3b568066a767fc3e45f67f81ab3c3e8001e19147973fc062b6fe7b7f5b.jpg)  
(d)

![](images/ea3a1609a6559a6be93f12adca1ac3061e4a67ac06621565bfaf96407fa10409.jpg)  
(e)  
Fig. 8. Results of IHATRPO under different numbers of sensor nodes. (a) The charging efficiency for SV. (b) The charging efficiency for AAV. (c) The travel distance for SV. (d) The travel distance for AAV. (e) The mortality of sensor nodes (100 × f<sub>3</sub>%).

![](images/97b04c20cf0445fcb8c40085f3f6b3f80f82c4323166920d7f19cf89ab6634c9.jpg)  
(a)

![](images/91f0a834508568dacabe02a6869881d8f5c19570a62bd7f8417c1655f2578568.jpg)  
(b)

![](images/30009311802dcf918c4d380649f88d0d92c1de8fcafd34695c4c8bd481bf18df.jpg)  
(c)

![](images/efe2287ea104cf948069985c88ad3b61b9d6ea97c30c1aff2b52ddc91e50cb3a.jpg)  
(d)

![](images/3835feeb9646d56727e4ec39d8226200ab98808d45af221209adac4e0603c849.jpg)  
(e)  
Fig. 9. Results of IHATRPO under different lengths of WRSN areas. (a) The charging efficiency for SV. (b) The charging efficiency for AAV. (c) The travel distance for SV. (d) The travel distance for AAV. (e) The mortality of sensor nodes (100 × f<sub>3</sub>%).

![](images/409a1e6711e5f39a36e4a3ad9edbf3eb3d61e7fa7d2a69a41346f6750a9a90aa.jpg)  
(a)

![](images/0121c5696fee613b6d643805496a7f0255fefde8e630d26501184d8b53f58611.jpg)  
(b)

![](images/ae5b10d436b18bfb24c383e1d86023ddd7838f825acdcdb0280fc15dd47776b4.jpg)  
(c)

![](images/2d963b4373f8b6760965c58a41113911d6977c35c8e7e93560601b7e4aebdf22.jpg)  
(d)

![](images/916ff5f652efbcf266fcd4eb133deed6fe663c043234f7cb938704c31be8fe2b.jpg)  
(e)  
Fig. 10. Results of IHATRPO under different charging radii of mobile chargers AAV/SV. (a) The charging efficiency for SV. (b) The charging efficiency for AAV. (c) The travel distance for SV. (d) The travel distance for AAV. (e) The mortality of sensor nodes (100 × f<sub>3</sub>%).

Beta sampling causes a noticeable decline in the final reward value, which indicates that the integration of the Beta sampling strategy helps the policy explore the bounded action space more effectively.

Quantitatively, the integration of the self-attention mechanism and Beta sampling strategy yields an overall reward value improvement of approximately 51% compared with the original HATRPO algorithm. This result in Fig. 16 confirms that both components contribute significantly to enhance the learning performance and overall reward of IHATRPO.

![](images/65ee6a2b263f87311b6dd39dfb754c0a0cf8f1307bdb872d9000fc183d2ff5d0.jpg)  
(a)

![](images/4fdc4130979a00e49d5f01c3accfea1d88a34892b62d897cfb46c81e9a320709.jpg)  
(b)

![](images/12514e035b01958f94cc808d1627715b286fbb5ebee37c6fd3fe1d9fcb7254d4.jpg)  
(c)

![](images/3745e43c0ea914273f3035b8732f3524e551cbd5d41a418ae86e9b3d2d2d4586.jpg)  
(d)

![](images/4e8a97a5f1f777f01d6861e8c3d3c07747dc28c180e7070ef0406d12cf18a1b3.jpg)  
(e)  
Fig. 11. Results of IHATRPO under different initial energy levels of sensor nodes. (a) The charging efficiency for SV. (b) The charging efficiency for AAV. (c) The travel distance for SV. (d) The travel distance for AAV. (e) The mortality of sensor nodes (100 × f %).

![](images/613b48e70abff849ec3c95128b2f8f56f0789336d80cd8df42b2f5b1f3f39447.jpg)  
(a)

![](images/a6474382c911cbcb77d018401e63782dfeb8cbbbc38d613b32ebb0a4e07a9d4f.jpg)  
(b)

![](images/0f88280c0e9915d321cdf100d4e776776496316c3610ed6782ab4748f4caf7f8.jpg)  
(c)

![](images/d39b5ec84cdbf3a8d8c5dc49b72a7afbdda8bf38504e42916ae1da7905e54982.jpg)  
(d)

![](images/c8cd5aabff2a97f6557d7a111b12aa1fa2150ff7a77e654aaef7c40482ab87ac.jpg)  
(e)  
Fig. 12. Results of IHATRPO under different total energy budgets of AAV/SV. (a) The charging efficiency for SV. (b) The charging efficiency for AAV. (c) The travel distance for SV. (d) The travel distance for AAV. (e) The mortality of sensor nodes (100 × f %).

![](images/b4a3e69f4f67517fd05caca570a96b07e03c8988b88a739696cc7f6c3dcdcf84.jpg)  
(a)

![](images/568d613323e7d467d3c5366354e0f234dc853e4bc27efd24e724382409542e62.jpg)  
(b)

![](images/5a01f0f479da00363715f9acb8d10f5a4cb97c1118e1e6346cf8533ff04405d2.jpg)  
(c)

![](images/34023a00e76e2f91e56eac1abf5dfc9becf122908ff7f67823902d46b2b73928.jpg)  
(d)

![](images/c8b3929f73fb566e85c74e7f1dd2299f2f2c7afeda2cab14cb91df922120aa0c.jpg)  
(e)  
Fig. 13. Results of IHATRPO under different position distributions of sensor nodes. (a) The charging efficiency for SV. (b) The charging efficiency for AAV. (c) The travel distance for SV. (d) The travel distance for AAV. (e) The mortality of sensor nodes (100 × f<sub>3</sub>%).

![](images/7848f5812dca509821d37692de558b62731b5109f3c4596a82863c0bf23930e6.jpg)  
(a)

![](images/f2b914ec9ae604c031a60c259d14199b17a19116954c39e5903a4c4ecfaf4859.jpg)  
(b)

![](images/0180b13f5060a320be7f3cf88e961485043ca97a6f6a38c81044785527f6ac70.jpg)  
(c)

![](images/e18268bf1bfa8c297cd351edc22ff7bb65e5d304d0e6eb4cf4f5b5b8fe623cf7.jpg)  
(d)

![](images/56efcc0a27d0d03808f3dee6f94fa30e487c911745939c05d20942756aa66807.jpg)  
(e)  
Fig. 14. Results of IHATRPO under different seeds. (a) The charging efficiency for SV. (b) The charging efficiency for AAV. (c) The travel distance for SV. (d) The travel distance for AAV. (e) The mortality of sensor nodes (100 × f<sub>3</sub>%).

5) Spatial Movement Patterns Analysis: Fig. 17 shows the trajectory patterns and spatial distribution of the AAV and SV in the WRSN obtained through IHATRPO optimization. As observed in the trajectory visualization, the AAV primarily operates in the lower region of the sensor network, while the SV predominantly covers the upper region. The middle area demonstrates overlapping coverage where sensor nodes may be served by either agent, with the actual charging responsibility determined dynamically based on real-time charging requirements and spatial proximity.

This territorial division emerges naturally from the embedded coordination mechanism in IHATRPO. The self-attention mechanism enables each agent to dynamically assess charging priorities and spatial distribution based on the current WRSN conditions, which results in an efficient labor division that minimizes redundant coverage. Moreover, the Beta sampling enables agents to discover optimal territorial boundaries that balance workload distribution and service efficiency. This territorial coordination demonstrates the effectiveness of IHATRPO in achieving intelligent spatial resource allocation without explicit territorial assignment protocols or centralized

![](images/8e91c2c50748b72ab7345621380af30a390ed00c43a2fada5c5e9e562bcfe2c5.jpg)

Fig. 15. Results of IHATRPO averaged across all seeds, with the shaded region indicating the 95% confidence interval (CI).  
![](images/a3541a019bf3dc729aa779a041becbd259e0db69e1b3e0d97332c067e945ab20.jpg)  
Fig. 16. Effectiveness of different techniques (the self-attention mechanism and Beta sampling strategy).

![](images/8e210e7eea44228b58fd5bf82446e65764f73bbf1da283b95ef6ac2f74132ac6.jpg)  
Fig. 17. The trajectory of the AAV and SV obtained by IHATRPO.

coordination mechanisms.

## VII. CONCLUSION

This paper has investigated a collaborative charging optimization problem for WRSNs using heterogeneous mobile chargers in heterogeneous deployment scenarios. Following this, we have formulated a multi-objective optimization problem to simultaneously maximize charging efficiency, minimize mobility energy consumption, and reduce sensor node mortality by coordinating the AAV and SV. The problem has proved highly challenging due to its dynamic nature with real-time adaptation requirements and complex trade-offs between competing objectives in heterogeneous multi-agent environments. To address these challenges, we have proposed the novel IHATRPO algorithm that incorporates the selfattention mechanism for enhanced environmental processing and the Beta sampling strategy for unbiased gradient computation in continuous action spaces. Simulation results have demonstrated that the proposed IHATRPO algorithm achieves faster convergence and superior performance compared to baselines, with sensor node mortality dramatically reduced from over 90% to below 10%. Spatial movement patterns analysis shows that the AAV and SV naturally develop complementary coverage patterns through the embedded coordination mechanism, with each agent specializing in different network regions to achieve efficient spatial division of labor. However, the current simulation settings are still based on an obstaclefree WRSN area. In more complex physical environments or larger-scale WRSNs, IHATRPO faces more challenges for the AAV and SV to provide timely charging services for all nodes. Therefore, future work may focus on more complex simulation environments, large-scale WRSNs, and coordination among more charging agents.

## ACKNOWLEDGMENT

The authors would like to thank Prof. Dusit Niyato for his valuable advisory support and insightful suggestions during the early stage of this work.

## REFERENCES

[1] D. Kandris, C. Nakas, D. Vomvas, and G. Koulouras, “Applications of wireless sensor networks: an up-to-date survey,” Appl. Syst. Innov., vol. 3, no. 1, 2020.

[2] W. W. Greenwood, J. P. Lynch, and D. Zekkos, “Applications of UAVs in civil infrastructure,” J. Infrastruct. Syst., vol. 25, no. 2, p. 04019002, 2019.

[3] I. Akyildiz, W. Su, Y. Sankarasubramaniam, and E. Cayirci, “A survey on sensor networks,” IEEE Commun. Mag., vol. 40, no. 8, pp. 102–114, 2002.

[4] J. Li, G. Sun, A. Wang, M. Lei, S. Liang, H. Kang, and Y. Liu, “A many-objective optimization charging scheme for wireless rechargeable sensor networks via mobile charging vehicles,” Comput. Netw., vol. 215, p. 109196, 2022.

[5] D. Dhabliya, R. Soundararajan, P. Selvarasu, M. S. Balasubramaniam, A. S. Rajawat, S. B. Goyal, M. S. Raboaca, T. C. Mihaltan, C. Verma, and G. Suciu, “Energy-efficient network protocols and resilient data transmission schemes for wireless sensor networks—an experimental survey,” Energies, vol. 15, no. 23, 2022.

[6] M. Y. A. Khan, M. Hussain, M. Halim, S. Ibrahim, and A. Haque, “A comprehensive review on techniques and challenges of energy harvesting from distributed renewable energy sources for wireless sensor networks,” Control Syst. Optim. Lett., vol. 2, pp. 15–22, 2024.

[7] B. Y. Leon´ Avila, C. A. Garc <sup>´</sup> ´ıa Vazquez, O. P ´ erez Baluja, D. T. Cotfas,´ and P. A. Cotfas, “Energy harvesting techniques for wireless sensor networks: A systematic literature review,” Energy Strategy Rev., vol. 57, p. 101617, 2025.

[8] B. Qureshi, S. A. Aziz, X. Wang, A. Hawbani, S. H. Alsamhi, T. Qureshi, and A. Naji, “A state-of-the-art survey on wireless rechargeable sensor networks: Perspectives and challenges,” Wirel. Netw., vol. 28, no. 7, pp. 3019–3043, 2022.

[9] A. Kaswan, P. K. Jana, and S. K. Das, “A survey on mobile charging techniques in wireless rechargeable sensor networks,” IEEE Commun. Surv. Tutorials, vol. 24, no. 3, pp. 1750–1779, 2022.

[10] G. Sun, L. Zhang, J. Li, J. Wu, J. Wang, Z. Sun, C. Zhao, and V. C. M. Leung, “Age of information optimization in laser-charged UAV-assisted IoT networks: A multi-agent deep reinforcement learning method,” IEEE Trans. Netw. Sci. Eng., vol. 13, pp. 1436–1457, 2026.

[11] X. Mou, D. Gladwin, J. Jiang, K. Li, and Z. Yang, “Near-field wireless power transfer technology for unmanned aerial vehicles: a systematical review,” IEEE J. Emerg. Sel. Topics Ind. Electron., vol. 4, no. 1, pp. 147–158, 2023.

[12] C. Lin, F. Gao, H. Dai, J. Ren, L. Wang, and G. Wu, “Maximizing charging utility with obstacles through Fresnel diffraction model,” in Proc. IEEE INFOCOM, 2020, pp. 2046–2055.

[13] N. Liu, C. Luo, J. Cao, Y. Hong, and Z. Chen, “Trajectory optimization of laser-charged UAVs for charging wireless rechargeable sensor networks,” Sensors, vol. 22, no. 23, p. 9215, 2022.

[14] S. He, J. Chen, F. Jiang, D. K. Y. Yau, G. Xing, and Y. Sun, “Energy provisioning in wireless rechargeable sensor networks,” IEEE Trans. Mob. Comput., vol. 12, no. 10, pp. 1931–1942, 2013.

[15] C. Lin, Z. Wang, D. Han, Y. Wu, C. Yu, and G. Wu, “TADP: enabling temporal and distantial priority scheduling for on-demand charging architecture in wireless rechargeable sensor networks,” J. Syst. Archit., vol. 70, pp. 26–38, 2016.

[16] H. Dai, Q. Ma, X. Wu, G. Chen, D. K. Y. Yau, S. Tang, X. Li, and C. Tian, “CHASE: Charging and scheduling scheme for stochastic event capture in wireless rechargeable sensor networks,” IEEE Trans. Mob. Comput., vol. 19, no. 1, pp. 44–59, 2020.

[17] S. Liang, Z. Fang, G. Sun, C. Lin, J. Li, S. Li, and A. Wang, “Charging UAV deployment for improving charging performance of wireless rechargeable sensor networks via joint optimization approach,” Comput. Netw., vol. 201, p. 108573, 2021.

[18] T. Wu, P. Yang, H. Dai, C. Xiang, X. Rao, J. Huang, and T. Ma, “Joint sensor selection and energy allocation for tasks-driven mobile charging in wireless rechargeable sensor networks,” IEEE Internet Things J., vol. 7, no. 12, pp. 11 505–11 523, 2020.

[19] Y. Liu, H. Pan, G. Sun, A. Wang, J. Li, and S. Liang, “Joint scheduling and trajectory optimization of charging UAV in wireless rechargeable sensor networks,” IEEE Internet Things J., vol. 9, no. 14, pp. 11 796– 11 813, 2022.

[20] N. Liu, J. Zhang, C. Luo, J. Cao, Y. Hong, Z. Chen, and T. Chen, “Dynamic charging strategy optimization for UAV-assisted wireless rechargeable sensor networks based on deep Q-network,” IEEE Internet Things J., vol. 11, no. 12, pp. 21 125–21 134, 2024.

[21] M. U. F. Qaisar, W. Yuan, P. Bellavista, G. Han, and A. Ahmed, “ISACassisted wireless rechargeable sensor networks with multiple mobile charging vehicles,” IEEE Internet Things Mag., vol. 7, no. 6, pp. 80–86, 2024.

[22] M. U. F. Qaisar, W. Yuan, P. Bellavista, F. Liu, G. Han, R. S. Zakariyya, and A. Ahmed, “Poised: probabilistic on-demand charging scheduling for ISAC-assisted WRSNs with multiple mobile charging vehicles,” IEEE Trans. Mob. Comput., vol. 23, no. 12, pp. 10 818–10 834, 2024.

[23] Z. Zhao, T. Deng, Y. Liu, and F. Lin, “Charging between PADs: Periodic charging scheduling in the UAV-based WRSN with PADs,” Int. J. Distrib. Sens. Netw., vol. 2024, no. 1, p. 8851835, 2024.

[24] Y. Yang, X. Liu, K. Tang, W. Che, and Q. Xue, “Multi-type charging scheduling based on area requirement difference for wireless rechargeable sensor networks,” IEEE Trans. Sustain. Comput., vol. 9, no. 2, pp. 182–196, 2024.

[25] D. Lee, C. Lee, G. Jang, W. Na, and S. Cho, “Energy-efficient directional charging strategy for wireless rechargeable sensor networks,” IEEE Internet Things J., vol. 9, no. 19, pp. 19 034–19 048, 2022.

[26] X. Zhang, R. Jia, Q. Yin, Z. Zheng, and M. Li, “Intelligent trajectory design and charging scheduling in wireless rechargeable sensor networks with obstacles,” IEEE Trans. Mob. Comput., vol. 23, no. 9, pp. 8664– 8679, 2024.

[27] L. Li, Y. Feng, N. Liu, Y. Li, and J. Zhang, “Deep reinforcement learning-based dynamic charging–recycling scheme for wireless

rechargeable sensor networks,” IEEE Sensors J., vol. 24, no. 9, pp. 15 457–15 471, 2024.

[28] E. F. Orumwense and K. Abo-Al-Ez, “On increasing the energy efficiency of wireless rechargeable sensor networks for cyber-physical systems,” Energies, vol. 15, no. 3, 2022.

[29] Y. Yu, J. Tang, J. Huang, X. Zhang, D. K. C. So, and K. Wong, “Multiobjective optimization for UAV-assisted wireless powered IoT networks based on extended DDPG algorithm,” IEEE Trans. Commun., vol. 69, no. 9, pp. 6361–6374, 2021.

[30] L. Zhang, A. Celik, S. Dang, and B. Shihada, “Energy-efficient trajectory optimization for UAV-assisted IoT networks,” IEEE Trans. Mob. Comput., vol. 21, no. 12, pp. 4323–4337, 2022.

[31] T. Lyu, J. An, M. Li, F. Liu, and H. Xu, “UAV-assisted wireless charging and data processing of power IoT devices,” Computing, vol. 106, no. 3, pp. 789–819, 2024.

[32] C. Lin, S. Hao, W. Yang, P. Wang, L. Wang, G. Wu, and Q. Zhang, “Maximizing energy efficiency of period-area coverage with a UAV for wireless rechargeable sensor networks,” IEEE/ACM Trans. Netw., vol. 31, no. 4, pp. 1657–1673, 2023.

[33] C. Jiang, W. Chen, X. Chen, S. Zhang, and W. Xiao, “Deep reinforcement learning approach with hybrid action space for mobile charging in wireless rechargeable sensor networks,” Expert Syst. Appl., vol. 249, p. 123752, 2024.

[34] Y. Liang, H. Wu, and H. Wang, “ASM-PPO: asynchronous and scalable multi-agent PPO for cooperative charging,” in Proc. AAMAS, 2022, pp. 798–806.

[35] Z. Ning, H. Ji, X. Wang, E. C. H. Ngai, L. Guo, and J. Liu, “Joint optimization of data acquisition and trajectory planning for UAV-assisted wireless powered internet of things,” IEEE Trans. Mob. Comput., vol. 24, no. 2, pp. 1016–1030, 2025.

[36] T. Chen, J. Chen, X. Gao, and T. Chen, “Mobile charging strategy for wireless rechargeable sensor networks,” Sensors, vol. 22, no. 1, p. 359, 2022.

[37] L. Xie, Y. Shi, Y. T. Hou, and H. D. Sherali, “Making sensor networks immortal: An energy-renewal approach with wireless power transfer,” IEEE/ACM Trans. Netw., vol. 20, no. 6, pp. 1748–1761, 2012.

[38] J. Yi and I. Yoon, “Efficient energy supply using mobile charger for solar-powered wireless sensor networks,” Sensors, vol. 19, no. 12, p. 2679, 2019.

[39] Y. Mei, Y. Lu, Y. C. Hu, and C. S. G. Lee, “Energy-efficient motion planning for mobile robots,” in Proc. IEEE ICRA, 2004, pp. 4344–4349.

[40] Y. Zeng, J. Xu, and R. Zhang, “Energy minimization for wireless communication with rotary-wing UAV,” IEEE Trans. Wirel. Commun., vol. 18, no. 4, pp. 2329–2345, 2019.

[41] Y. Shu, H. Yousefi, P. Cheng, J. Chen, Y. J. Gu, T. He, and K. G. Shin, “Near-optimal velocity control for mobile charging in wireless rechargeable sensor networks,” IEEE Trans. Mob. Comput., vol. 15, no. 7, pp. 1699–1713, 2016.

[42] S. Gronauer and K. Diepold, “Multi-agent deep reinforcement learning: A survey,” Artif. Intell. Rev., vol. 55, no. 2, pp. 895–943, 2022.

[43] J. Schulman, S. Levine, P. Abbeel, M. I. Jordan, and P. Moritz, “Trust region policy optimization,” in Proc. ICML, vol. 37, 2015, pp. 1889– 1897.

[44] A. Vaswani, N. Shazeer, N. Parmar, J. Uszkoreit, L. Jones, A. N. Gomez, L. u. Kaiser, and I. Polosukhin, “Attention is all you need,” in Adv. Neural Inf. Process. Syst., vol. 30, 2017.

[45] P.-W. Chou, “The Beta policy for continuous control reinforcement learning,” Master’s thesis, Carnegie Mellon University, 2017.

[46] L. Fu, P. Cheng, Y. Gu, J. Chen, and T. He, “Optimal charging in wireless rechargeable sensor networks,” IEEE Trans. Veh. Technol., vol. 65, no. 1, pp. 278–291, 2016.

[47] C. Hou and Q. Huang, “Energy supply control of wireless powered piecewise linear neural network,” IEEE Trans. Autom. Sci. Eng., vol. 21, no. 4, pp. 6892–6907, 2024.

[48] J. Schulman, F. Wolski, P. Dhariwal, A. Radford, and O. Klimov, “Proximal policy optimization algorithms,” CoRR, vol. abs/1707.06347, 2017.

[49] T. P. Lillicrap, J. J. Hunt, A. Pritzel, N. Heess, T. Erez, Y. Tassa, D. Silver, and D. Wierstra, “Continuous control with deep reinforcement learning,” in Proc. ICLR, 2016.

[50] R. Lowe, Y. Wu, A. Tamar, J. Harb, P. Abbeel, and I. Mordatch, “Multiagent actor-critic for mixed cooperative-competitive environments,” in Adv. Neural Inf. Process. Syst., 2017, pp. 6379–6390.

[51] J. G. Kuba, R. Chen, M. Wen, Y. Wen, F. Sun, J. Wang, and Y. Yang, “Trust region policy optimisation in multi-agent reinforcement learning,” in Proc. ICLR, 2022.