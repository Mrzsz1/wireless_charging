---
title: "Directional WPT Charging for Routing-Asymmetric WRSNs with a Mobile Charger"
year: null
source_type: paper
why_relevant: ""
acquisition_method: auto_discovery
discovered_via: ["arxiv"]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260714-204713"
triage_status: promoted
selected_by_user: true
acquired_at: "2026-07-14T12:47:13+00:00"
canonicalized_at: 2026-07-14
ingest_status: ingested
pdf_path: "raw/canonical/Directional_WPT_Charging_for_Routing-Asymmetric_WRSNs_with_a_Mobile_Charger/Directional_WPT_Charging_for_Routing-Asymmetric_WRSNs_with_a_Mobile_Charger.pdf"
raw_md: "raw/canonical/Directional_WPT_Charging_for_Routing-Asymmetric_WRSNs_with_a_Mobile_Charger/full.md"
---
# Directional WPT Charging for Routing-Asymmetric WRSNs with a Mobile Charger

Zhenguo Gao\* Senior Member, IEEE, Qi Zhang, Qingyu Gao, Yunlong Zhao, Hsiao-Chun Wu\* Fellow, IEEE

Abstract—Mobile Charge Scheduling for wirelessly charging nodes in Wireless Rechargeable Sensor Networks (WRSNs) is a promising but still evolving research area. Existing research mostly assumes a symmetric environment, where the routing costs in opposite directions between two locations are considered identical. However, various factors such as terrain restrictions and wind or water flows may invalidate the routing-symmetric assumption in practical environments, thereby significantly limiting the performance of these solutions in routing-asymmetric WRSNs (RA-WRSNs). To address the routing-asymmetric challenges in mobile charge scheduling for WRSNs, this paper systematically investigates the underlying Asymmetric Directional Mobile Charger (DMC) Charge Scheduling (ADMCCS) problem, aiming to minimize energy loss while satisfying the charging demands of the network nodes. The DMC model is assumed because its results can be easily applied to the specialized case of an Omnidirectional Mobile Charger (OMC). To solve the ADMCCS problem, we propose a four-step framework. First, a minimum-size efficient charging position set is selected using our designed K-means-based Charging Position Generation (KCPG) algorithm, addressing the challenge of the unlimited charging position selection space. Next, minimum-size functionalequivalent direction sets at these positions are determined using an optimal algorithm, tackling the challenge of infinite charging directions. Subsequently, the optimal energy transmission time lengths for all directions at the positions are obtained by formulating and solving a Nonlinear Program (NLP) problem. Finally, the Lin-Kernighan Heuristic (LKH) algorithm for the Asymmetric Traveling Salesman Problem is adapted to obtain a highly probable optimal loop tour, addressing the routingasymmetric challenge. The combination of these steps results in our DMC Scheduling algorithm for RA-WRSNs (RA-DMCS). The properties of the ADMCCS problem and the proposed algorithms are analyzed, and experiments demonstrate that RA-DMCS considerably outperforms other typical algorithms.

Index Terms—Charging scheduling, wireless power transfer, directional mobile chargers, wireless rechargeable sensor networks, asymmetric path planning

## I. INTRODUCTION

applied in various industrial and everyday scenarios.

However, a major limitation hindering the rapid development of WSNs is the energy constraints of battery-powered nodes [1, 2]. This issue is particularly pronounced with the advancement of edge intelligence, which imposes high computational demands on the nodes and leads to increased energy consumption [3, 4]. To address this issue, researchers are working not only on improving battery storage capacity but also on advancing Wireless Power Transfer (WPT) technology for on-demand wireless charging of the nodes [5, 6].

With the ongoing development of WPT technology, researchers have extended its application to WSNs, leading to the concept of Wireless Rechargeable Sensor Networks (WRSNs) [7]. In WRSNs, the nodes are equipped with energy reception modules, while fixed or Mobile Chargers (MCs) equipped with WPT energy transmission modules are responsible for charging the nodes on demand. A WRSN also contains a Base Station (BS) where the MCs are replenished and remain stationed, awaiting node charging requests.

To efficiently and promptly meet the energy demands of the nodes in WRSNs, optimizing the MCs’ charging schedule involves designing their charging trajectory, scheduling their charging directions, and determining charging times along each direction. Executing a charging schedule with an MC incurs two types of energy consumption: movement energy consumption and charging energy consumption. The former is used for moving the MC, while the latter results from the energy transmission from the MC to the nodes. An MC’s charging schedule should aim to minimize overall energy consumption while ensuring that all nodes receive adequate energy [8]. Depending on the energy transmission module they are equipped with, MCs can be categorized as Omnidirectional MCs (OMCs) or Directional MCs (DMCs). OMCs radiate energy signals uniformly in all directions, while DMCs concentrate signal energy in a specific direction within a sectorlike region [9].

Initially constrained by limited charging distance, early work focused on a one-to-one (O2O) charging mode using OMCs, where an MC must approach very close to a node to charge it [10]. This simplifies the charging schedule problem to a trajectory design problem for the MC. With advancements in WPT enabling long-distance wireless charging over several meters, many studies have begun to explore oneto-many (O2M) and many-to-many (M2M) charging modes. These approaches allow multiple MCs to charge several nodes simultaneously. In these studies, the charging schedule involves both the trajectory design of the MCs and their energy transmission schedules. Some research further extends the MC charging scenario by assuming that nodes can also transmit energy, enabling explicit multi-hop WPT. These studies propose efficient scheduling algorithms that produce improved schedules [7, 11, 12]. DMC-based charging of WRSNs offers the advantage of higher energy transfer efficiency [13], but it introduces the challenge of determining an optimal charging direction set. As a result, DMC-based charging of WRSNs is increasingly attracting research attention [14].

Existing studies often assume routing-symmetric environments [15], where the energy consumption for movement between two positions is considered identical in both directions. However, this routing-symmetric assumption often does not hold in many real-world scenarios due to factors such as terrain and topography restrictions, altitude differences, wind forces (if deployed in the air), and water flow (if deployed on or in water). Routing asymmetry makes designs intended for routing-symmetric environments ineffective in practical routing-asymmetric scenarios. Although the asymmetry of updown paths in underwater sensor networks was addressed in [10], this work only considered routing asymmetry within a simple greedy trajectory design framework and focused on OMCs in O2O charging mode. Consequently, it did not explore the more energy-efficient M2M mode, resulting in less effective charging schedules. For simplicity, we will omit the term ”routing” when referring to ”symmetric” and ”asymmetric” in the following text.

To systematically address routing-asymmetric issues, we conducted an in-depth investigation into Directional WPT Charging of WRSNs using DMCs in such asymmetric environments. To avoid being limited to specific asymmetric scenarios and to generalize across various environments, we refer to these WRSNs as Routing-Asymmetric WRSNs (RA-WRSNs).

In this paper, we primarily investigate the DMC charge scheduling problem in RA-WRSNs, which we term the Asymmetric DMC Charge Scheduling (ADMCCS) problem. The objective is to develop a DMC charging schedule that ensures all nodes’ energy demands are met. Several challenges impede this task: the unlimited selection space for charging positions, the infinite number of possible charging directions, and the routing asymmetry of the WRSNs.

We show that the ADMCCS problem is NP-hard, making it impractical to design an optimal algorithm; hence, we focus on efficient approximation algorithms. To simplify the problem, we additionally assume that the DMC does not transmit energy signals while moving. Inspired by [16], we address the ADMCCS problem using a multi-step approach, as outlined in Fig. 1. First, we identify several charging positions within the network region. Next, we establish the charging directions and determine the time duration for transmitting energy along each direction at each position. Finally, we design a round tour to visit all charging positions in the RA-WRSNs.

The corresponding tasks in these steps are formulated and solved with specifically designed algorithms. Specifically, we propose a K-means Charging Position Generation (KCPG) algorithm to determine charging positions, addressing the challenge of the unlimited charging position selection space. We adopt the cMFRDS algorithm from [14] to identify a minimum set of functional representative directions as the final charging directions, thus addressing the challenge of infinite charging directions. We use CPlex software to solve a linear programming problem and determine the optimal charging times for all directions at the selected charging positions. We use the Lin-Kernighan Heuristic (LKH) algorithm [17], a stateof-the-art algorithm for solving the TSP problem, to find an optimal loop tour with minimal length in the RA-WRSNs to visit all charging positions, thereby tackling the routing asymmetry challenge. These algorithms are then integrated into our DMC Scheduling algorithm in RA-WRSNs (RA-DMCS) to solve the ADMCCS problem. Extensive simulations and test-bed experiments demonstrate the superiority of our RA-DMCS algorithm.

![](images/4b89e236333cae30fe071ee4e2014d5be83ce91c9681057e91812825d403aa99.jpg)  
Fig. 1: Outline of the RA-DMCS algorithm for the ADMCCS problem

The main contributions of this paper are summarized as follows:

1) To the best of the authors’ knowledge, this is the first work to abstractly construct the concept of routingasymmetric WRSNs and to systematically investigate the DMC-based directional wireless charging scheduling problem in RA-WRSNs.

2) We demonstrate that the ADMCCS problem for DMCbased wireless charging scheduling in RA-WRSNs is NPhard and apply state-of-the-art methods and results from the fields of the Traveling Salesman Problem (TSP) and Asymmetric TSP (ATSP) to address this problem.

3) We propose the RA-DMCS algorithm to solve the ADM-CCS problem. Extensive simulation experiments validate the effectiveness of our RA-DMCS algorithm.

The remaining sections of this paper are organized as follows: Section II provides a brief overview of previous research on WRSNs. Section III introduces the system and energy transmission models. Section IV details the ADM-CCS problem, its mathematical formulation, and proposes corresponding solutions through in-depth analysis. Section VI presents the RA-DMCS algorithm along with its analysis. Section VII evaluates the performance of the algorithms through simulations. Finally, Section VIII summarizes the key findings and concludes the paper.

## II. RELATED WORK

In this section, we summarize the current research on charging scheduling and classify it into two main categories: stationary charging and mobile charging. Additionally, we review related work on the study of asymmetric paths.

## A. Charging with Stationary Chargers

As the deployment of stationary chargers with omnidirectional WPT can easily be adapted from various node deployment solutions for covering a network in closely related realms, most related work has focused on the topic of charging WRSNs with stationary directional chargers. The main concern is to optimally determine the positions and charging directions for the chargers while considering various additional factors.

For example, for a given WRSN and a fixed number of stationary chargers, Dai et al. [18] proposed an approximate algorithm for optimizing the overall expected charging utility by jointly optimizing the positions and charging directions for the chargers. The approximation ratio of the algorithm was analyzed. For WRSNs where nodes may drift within a certain range, Wang et al. [19] aimed to maximize the overall expected charging utility by determining the charging directions of multiple directional stationary chargers with predetermined fixed positions. The authors subsequently extended their work to complex scenarios with obstacles and multi-type heterogeneous stationary chargers, where a piecewise constant function was used to approximate the nonlinear energy transmission power model. This body of work targets 2D WRSNs, where the solution space is a restricted region in a 2D plane.

Jiang et al. [20] proposed a 3D WRSN charging scheme in which nodes and chargers are deployed on different planes. By reasonably controlling the number and positions of chargers, the scheme aims to meet the charging requirements of the nodes while minimizing costs.

Although these solutions can produce efficient deployment strategies for the chargers, the lack of mobility limits their adaptability to changes in the energy demands of the nodes, thus restricting the flexibility of such WRSNs.

## B. Charging with Mobile Chargers

For MCs with omnidirectional WPT, charging scheduling with mobile chargers involves trajectory design and charging time determination.

MCs with directional WPT introduce the selection of charging direction as a new design dimension, and the charging time at a position should be specified according to the charging directions.

For instance, Wu et al. [21] proposed flexible scheduling strategies to optimize charging routes, enhancing the charging utility of the MCs. Liu et al. [22] further refined the optimization objectives by focusing on energy utilization efficiency and the number of dead nodes. They proposed a request-based charging path system tailored to their model. Lin et al. [23] improved the path planning component by exploring the selection of charging positions while considering energy constraints and obstacles, thereby offering solutions for complex network environments. In [24], Dai improved upon previous work by incorporating directional charging into their scheme and studying the placement strategy of DMCs under mobility constraints. This strategy ensures effective energy delivery even when movement is restricted. Additionally, Abhinav et al. [25] introduced a heap-based energy replenishment scheme, which efficiently prioritizes nodes based on their energy requirements. Smriti et al. [26] considered on-demand charging schemes that account for heterogeneous energy consumption and partial charging, optimizing the use of available energy resources.

These studies collectively aim to achieve efficient energy utilization and extend the lifespan of WRSNs by addressing various challenges associated with mobile charging, such as asymmetric routing, path optimization, energy constraints, and dynamic network conditions. We have summarized the research details of the aforementioned papers, as shown in Table I.

## C. Mobile Charging in Asymmetric Environments

The above work assumes routing-symmetric environments, but this assumption may not hold in many real-world scenarios, which hinders the applicability of the proposed algorithms. However, to date, very few studies have considered the routing-asymmetry feature in charging scheduling for WRSNs. For charging WRSNs in underwater environments, Lin et al. [10] considered the asymmetry of underwater movement, but the routing-asymmetry was only addressed within a simple greedy trajectory design framework, failing to be systematically considered and tackled. Furthermore, their work only deals with OMCs in the O2O charging mode, without exploring the more energy-efficient M2M mode, resulting in a less effective charging schedule. As far as we know, this is the only work that considers routing-asymmetry in addressing the charging scheduling problem in WRSNs.

In fact, routing-asymmetry has long been an important issue in various versions of trajectory design problems, such as the TSP problem. To facilitate solving ATSP using the powerful algorithms designed for TSP, a method was proposed in Ref. [27] to transform an ATSP instance into a TSP instance. Better exploiting the state-of-the-art algorithms in the realms of TSP and ATSP can facilitate systematically addressing the charge scheduling problem in WRSNs with routing-asymmetry, making the solutions more applicable to real-world scenarios.

## III. PRELIMINARY MODELS

In this section, we present the preliminary models, including the system model, asymmetric routing model, DMC’s energy consumption model, WPT energy transfer model, and directional energy transfer coefficient model.

## A. System Model

We consider an RA-WRSN consisting of a Base Station (BS) located at position $l _ { 0 } .$ , a DMC, and N rechargeable nodes in the set $\mathcal { U } = \{ u _ { 1 } , u _ { 2 } , . . . , u _ { N } \}$ . For simplicity, each node $u _ { i } \in \mathcal { U }$ is also referred to by its position. The DMC is responsible for charging the nodes to ensure their normal operation. It typically recharges its battery fully at the BS and remains there until required to charge some nodes. Each node’s energy demand is referred to as a charging task. To address a set of charging tasks, the DMC departs from the

TABLE I: Summary of Main Related Work

<table><tr><td>Work</td><td>Optimization objective</td><td>One-to-many charging</td><td>Charging position optimization</td><td>Support directional charging</td><td>Support mobile charging</td><td>Address asymmetric routing</td></tr><tr><td>[18]</td><td>Charging utility</td><td>✓</td><td>✓</td><td>✓</td><td></td><td></td></tr><tr><td>[19]</td><td>Charging utility</td><td>✓</td><td>✓</td><td>✓</td><td></td><td></td></tr><tr><td>[20]</td><td>Costs</td><td>✓</td><td>✓</td><td>✓</td><td></td><td></td></tr><tr><td>[21]</td><td>Charging utility</td><td>✓</td><td>✓</td><td></td><td>✓</td><td></td></tr><tr><td>[22]</td><td>Energy efficiency and Number of Dead node</td><td>✓</td><td>✓</td><td></td><td>✓</td><td></td></tr><tr><td>[23]</td><td>Charging utility</td><td>✓</td><td>✓</td><td></td><td>✓</td><td></td></tr><tr><td>[24]</td><td>Charging utility</td><td>✓</td><td>✓</td><td>✓</td><td>✓</td><td></td></tr><tr><td>[25]</td><td>Charging utility</td><td>✓</td><td>✓</td><td>✓</td><td>✓</td><td></td></tr><tr><td>[26]</td><td>Charging Delay</td><td>✓</td><td>✓</td><td></td><td>✓</td><td></td></tr><tr><td>Our work</td><td>Energy loss and Time</td><td>✓</td><td>✓</td><td>✓</td><td>✓</td><td>✓</td></tr></table>

BS, follows a tour to charge the nearby nodes, and then returns to the BS. The charging tour must be meticulously designed and scheduled to fulfill the charging tasks efficiently. An illustrative example of the RA-WRSN is shown in Fig. 2.

Fig. 2: An example RA-WRSN  
![](images/3bae3f67f63bb6a59a5cb782b9248a959fada024c08b4f142dcbb37590ec9fa5.jpg)

Each node is equipped with a rechargeable battery and a wireless energy transceiver for transmitting and receiving energy. Node $u _ { i } \in \mathcal { U }$ has some parameters as follows: initial energy $e _ { \mathrm { B } } ^ { i } .$ , energy demand $e _ { \mathrm { D } } ^ { i } ,$ , and storage capacity $e _ { \mathrm { C } } ^ { i }$ . Here, $e _ { \mathrm { B } } ^ { i }$ denotes the initial energy stored in the battery of $u _ { i } ,$ , and $e _ { \mathrm { D } } ^ { i }$ represents the amount of energy required by node $u _ { i } .$ . To simplify notation, we compactly express the parameters of the nodes as column vectors. For example, we use $\mathbf { e _ { B } }$ to represent $\mathbf { e } _ { \mathrm { B } } { : = } [ e _ { \mathrm { B } } ^ { 1 } , e _ { \mathrm { B } } ^ { 2 } , \cdot \cdot \cdot , e _ { \mathrm { B } } ^ { N } ]$

The DMC has the following parameters: energy transmission power $p _ { 0 }$ , initial energy $e _ { \mathrm { B 0 } }$ , moving speed v, and energy consumption base rate $w _ { 0 }$ for moving one unit distance. Here we assume that the initial energy $e _ { \mathrm { B 0 } }$ is sufficient to meet the energy demands of all nodes. Otherwise, we can split the whole charging tour into several sub charging tours such that each sub-tour can be fully served by an DMC, as discussed in [12].

To simply the charge scheduling task, we additionally assume that the DMC does not transmit energy while moving. Therefore, a charging task set can be fulfilled in steps by firstly selecting charging positions in the network area, then determining a charging tour and charging schedule, to traverse these selected positions and staying their for a time to transmitting energy along some selected directions.

## B. Asymmetric Routing Model

The movement energy consumption incurred in moving the MC make up a main part of the energy consumption for fulfilling a charging schedule. Routing-asymmetry considerably affects the movement energy consumption of a charging tour. As introduced in Sec. I, routing-asymmetry results from various factors, such as terrain and topography restrictions, altitude difference, wind force if deployed in the air, and water flow if deployed on or in water. To abstract the heterogeneous underlying factors, we build an abstracting asymmetric routing model, which contains two parts: the asymmetric distance model and the asymmetric movement energy consumption rate model, where at the core are two factor coefficients for reflecting the effect of factors causing asymmetry.

For the asymmetric distance model, we define routingasymmetry (RA) distance coefficient $k _ { \mathrm { R A , D i s } } { : = } k _ { \mathrm { R A , D i s } } ( l _ { i } , l _ { j } |$ $m _ { \mathrm { R A } } , h _ { \mathrm { R A } } , f _ { \mathrm { R A } } )$ . The RA distance coefficient mimics the asymmetric effect on the length of the path segment from $l _ { i }$ to $l _ { j } .$ , determined by the factors of $m _ { \mathrm { R A } } , \ h _ { \mathrm { R A } }$ , and $f _ { \mathrm { R A } }$ Here $h _ { \mathrm { R A } }$ represents the height distance between $l _ { i }$ and $l _ { j } .$ , f represents the accumulated external force affecting the MC’s movement from $l _ { i }$ to $l _ { j } ,$ , and $m _ { \mathrm { R A } }$ represents the overall effect of other miscellaneous factors result from terrain and topography heterogeneity. Detailed formulation of $k _ { \mathrm { R A , D i s } }$ depends on the particular asymmetric environment, whereas the overall effect for all asymmetric environment finally lead to a real scale value. We abbreviate $k _ { \mathrm { R A , D i s } } ( l _ { i } , l _ { j } | m _ { \mathrm { R A } } , h _ { \mathrm { R A } } , f _ { \mathrm { R A } } )$ as $k _ { \mathrm { R A , D i s } } ( l _ { i } , l _ { j } )$ for simplicity. Similarly, for the asymmetric movement energy consumption rate model, we define RA movement energy consumption rate coefficient $k _ { \mathrm { R A , E g y } } { : = } k _ { \mathrm { R A , E g y } } ( l _ { i } , l _ { j } | m _ { \mathrm { R A } } , h _ { \mathrm { R A } } , f _ { \mathrm { R A } } )$ to account for the asymmetry on actual movement energy consumption rate per unit distance from $l _ { i }$ to $l _ { j }$ .

With the above defined coefficients, for any two positions $l _ { i } , \ l _ { j }$ in the area of the RA-WRSN, let $d _ { \mathrm { i d e a l } } ( l _ { i } , l _ { j } )$ denote the ideal euclidean distance between them, and let $d ( l _ { i } , l _ { i } )$ denote the routing-asymmetric real distance. To emphasize the context in routing-asymmetry, We call $d ( l _ { i } , l _ { i } )$ as the RA distance of the path segment from $l _ { i }$ to $l _ { j }$ . Similarly, we let $w ( l _ { i } , l _ { j } )$ denote the routing-asymmetric real movement energy consumption rate per unit distance from $l _ { i }$ to $l _ { j } .$ , naming it as RA energy consumption rate. Then the RA distance and RA movement energy consumption of the path segment from $l _ { i }$ to $l _ { j }$ are modeled as

$$
d (l _ {i}, l _ {j}) = k _ {\mathrm{RA,Dis}} (l _ {i}, l _ {j}) \cdot d _ {\text { Ideal }} (l _ {i}, l _ {j}),
$$

$$
w (l _ {i}, l _ {j}) = k _ {\mathrm{RA,Egy}} (l _ {i}, l _ {j}) \cdot w _ {0}.\tag{1}
$$

(2)

The above two equations make up the asymmetric routing model. The routing-asymmetry are ultimately manifested as RA distances and RA energy consumption rates of directional path segments between charging position pairs. For notation simplicity, We collect the RA values into two matrices of $\scriptstyle \mathbf { D } = [ d ( l _ { i } , l _ { j } ) ] _ { \{ l _ { i } , l _ { j } \in { \mathcal { L } } \} }$ and $\mathbf { W } { = } [ w ( l _ { i } , l _ { j } ) ] _ { \{ l _ { i } , l _ { j } \in \mathcal { L } \} }$ . Here L is the set of charging positions.

## C. Energy Consumption Model of the DMC

In fulfilling a charging schedule, the energy consumption of the DMC contains two parts: movement energy consumption for driving the DMC move, charging energy consumption resulting from charging the nodes.

For a path segment from point $l _ { i }$ to $l _ { j }$ with RA distance $d ( l _ { i } , l _ { j } )$ and movement energy consumption rate $w ( l _ { i } , l _ { j } )$ , the movement energy consumption $e _ { \mathrm { M o v e } } ^ { \mathrm { M C } } ( { \ r _ { l } } _ { i } , { \ r _ { l } } _ { j } )$ and the moving time $t _ { \mathrm { M o v e } } ^ { \mathrm { M C } } ( l _ { i } , l _ { j } )$ are respectively modeled as Eq. (3) and Eq. (4), where v is the constant moving speed of the DMC.

$$
e _ {\mathrm{Move}} ^ {\mathrm{MC}} (l _ {i}, l _ {j}) = d (l _ {i}, l _ {j}) w (l _ {i}, l _ {j}), \qquad l _ {i}, l _ {j} \in \mathcal {L}\tag{3}
$$

$$
t _ {\mathrm{Move}} ^ {\mathrm{MC}} (l _ {i}, l _ {j}) = d (l _ {i}, l _ {j}) / \overline {{v}}, \quad l _ {i}, l _ {j} \in \mathcal {L}.\tag{4}
$$

Let $\mathrm { r } ( \mathcal { L } ) { : = } [ l _ { 0 } , l _ { \pi _ { 1 } } , l _ { \pi _ { 2 } } , \ldots , l _ { \pi _ { \vert } \mathcal { L } \vert } , l _ { \pi _ { \left( \vert \mathcal { L } \vert + 1 \right) } = } l _ { 0 } ]$ represent a charging tour contains $| { \mathcal { L } } |$ intermediate positions, then the movement energy consumption and moving time of the DMC following the tour can be obtained as

$$
e _ {\text { Move }} ^ {\text { MC }} (\mathbf {r} (\mathcal {L})) = \sum_ {i = 0} ^ {| \mathcal {L} |} e _ {\text { Move }} ^ {\text { MC }} (l _ {\pi_ {i}}, l _ {\pi_ {(i + 1)}}),\tag{5}
$$

$$
t _ {\text { Move }} ^ {\text { MC }} (\mathbf {r} (\mathcal {L})) = \sum_ {i = 0} ^ {| \mathcal {L} |} t _ {\text { Move }} ^ {\text { MC }} (l _ {\pi_ {i}}, l _ {\pi_ {(i + 1)}}).\tag{6}
$$

Let $S _ { \mathrm { D i r } } ( i ) { : = } \{ \psi _ { 1 } , \psi _ { 2 } , \ldots , \psi _ { k _ { i } } \}$ denote the charging direction set at position $l _ { i } .$ . We use a tuple $( l _ { i } , \psi _ { j } )$ to completely define a pair of a direction and its associated position. Without loss of generality, let assume that the DMC does not transmit energy at the BS, i.e., the number of charging directions at $l _ { 0 }$ is $k _ { 0 } = 0$ . Let $S _ { \mathrm { P o s D i r } } ( l _ { i } ) { : = } \{ ( l _ { i } , \psi _ { j } ) | i { \in } \{ 1 , 2 , \ldots , k _ { i } \} $ , then $S _ { \mathrm { P o s D i r } } \mathrm { : = \cup _ { \mathrm { } i = 1 } ^ { | \mathcal { L } | } } S _ { \mathrm { P o s D i r } } ( \mathit { l } _ { i } )$ contains all position-direction (abbreviated as Pos-Dir) pairs. Let us sort the pairs in $\mathcal { S } _ { \mathrm { P o s D i r } }$ into a list firstly on position and then on direction value, and with a slight abuse of symbols, we re-use $\mathcal { S } _ { \mathrm { P o s D i r } }$ to denote the sorted list. We further assume that the charge time list (or column vector) corresponding to the Pos-Dir pairs in $\mathcal { S } _ { \mathrm { P o s D i r } }$ as $\mathbf { t } _ { \mathrm { T r a n } } ^ { \mathrm { M C } } { = } [ t _ { 1 } ^ { \mathrm { T r a n } } , t _ { 2 } ^ { \mathrm { T r a n } } , \cdot \cdot \cdot , t _ { K } ^ { \mathrm { t r a n } } ] ^ { \mathrm { T } }$ with $\begin{array} { r } { K : = \sum _ { i = 1 } ^ { M } k _ { i } } \end{array}$ denotes the length of list $\mathbf { t } _ { \mathrm { T r a n } } ^ { \mathrm { M C } }$ . With the assumption that the DMC always transmit energy with power $p _ { 0 } ,$ the charging energy consumption of the DMC for transmitting energy can be obtained as

$$
e _ {\text {Tran}} ^ {\text {MC}} := p _ {0} \cdot \sum_ {i = 1} ^ {K} t _ {i} ^ {\text {Tran}} = p _ {0} \mathbb {1} ^ {1 \times K} \mathbf {t} _ {\text {Tran}} ^ {\text {MC}}.\tag{7}
$$

The total energy consumption and the final energy of the DMC after fulfilling a charging schedule can be obtained as Eq. (8) and Eq. (9), respectively.

$$
e _ {\text { Total }} ^ {\text { MC }} = e _ {\text { Tran }} ^ {\text { MC }} + e _ {\text { Move }} ^ {\text { MC }} (\mathbf {r} (\mathcal {L})).\tag{8}
$$

$$
e _ {\mathrm{F0}} = e _ {\mathrm{B0}} - e _ {\text { Total }} ^ {\mathrm{MC}} = e _ {\mathrm{B0}} - e _ {\text { Tran }} ^ {\mathrm{MC}} - e _ {\text { Move }} ^ {\mathrm{MC}} (\mathbf {r} (\mathcal {L})).\tag{9}
$$

## D. WPT Energy Transfer Model

Let $c ( i , j )$ denote the energy transfer coefficient from Pos-Dir pair i in $\mathcal { S } _ { \mathrm { P o s D i r } }$ to node $u _ { j }$ . If the DMC transmits energy at power $p _ { 0 }$ , the energy power received by node $u _ { j }$ is expressed as $p ( k ) { : = } c ( i , j ) p _ { 0 }$ . Let construct energy transfer coefficient matrix $\mathbf { C } { : = } [ c ( i , j ) ] _ { \{ i \in \{ 1 , 2 , \cdots , K \} , j \in \{ 1 , 2 , \cdots , | \mathcal { L } | \} \} }$ , then given the energy transmission time duration list $\mathbf { t } _ { \mathrm { T r a n } } ^ { \mathrm { M C } ^ { \prime } } { = } [ t _ { 1 } , t _ { 2 } , \dots , t _ { K } ] ^ { \mathrm { T } }$ corresponding to the Pos-Dir pairs in $\mathcal { S } _ { \mathrm { P o s D i r } } .$ , the list of energy received by all nodes ${ \bf e } _ { \mathrm { R } } : = [ e _ { 1 } ^ { \mathrm { R } } , e _ { 2 } ^ { \mathrm { R } } , \cdot \cdot \cdot , e _ { M } ^ { \mathrm { R } } ] ^ { \mathrm { T } }$ can be compactly expressed as

$$
\mathbf {e} _ {\mathrm{R}} = p _ {0} \mathbf {C t} _ {\text { Tran }} ^ {\mathrm{MC}}.\tag{10}
$$

Considering the capacity limits of the nodes, final energy stored in the batteries of the nodes, denoted as e<sub>F</sub>, can be expressed as Eq. (11), where min means element-wise minimization.

$$
\mathbf {e} _ {\mathrm{F}} = \min \left\{\mathbf {e} _ {\mathrm{B}} + \mathbf {e} _ {\mathrm{R}}, \mathbf {e} _ {\mathrm{C}} \right\}.\tag{11}
$$

## E. Directional Energy Transfer Coefficient Model

Energy transfer coefficient model is used to determine the energy transfer coefficient used in Sec. III-D. Here we adopt a widely used directional energy transfer coefficient model as expressed in Eq. (12), as outlined in [28]. Here the region covered by the energy transmission signal of a DMC is a sector rooted at the DMC with radius equals charge distance D and sector angle $\varphi ,$ as depicted in Fig. 3. The center-line of the coverage sector is referred to as the corresponding charging direction, denoted as ψ. With the additional parameter including charging direction angle ψ, node direction angle θ, and the distance d from the DMC to the node concerned, Eq. (12) determines the energy transfer coefficient from the DMC to the node. All direction angles in the model are measured relative to a certain reference direction with positive in counterclockwise. We assume that the DMC’s sector angle $\varphi$ remains constant, whereas $\psi$ is freely adjustable.

$$
c (\psi , \varphi , \theta , d) = \left\{ \begin{array}{l l} \frac {\delta}{(\alpha + d) ^ {\beta}}, & d \leq D, \theta \in [ \psi - \varphi / 2, \psi + \varphi / 2 ], \\ 0, & \text { otherwise }. \end{array} \right.\tag{12}
$$

The energy radiation leads to a charge sector is called a charging beam. As described in [29], DMCs may be capable of generating multiple charging beams simultaneously. So they can correspondingly be classified as single-beam DMC and multi-beam DMC. We focus on single-beam DMC in this paper, whereas the multi-beam DMC case can be addressed following the idea in [14], where normal symmetrical WRSNs are assumed.

![](images/2df3f98f1bacb3e9760428c4718928cbd7d869fc9543481ca9b7ebfce8400910.jpg)  
Fig. 3: Coverage sector of a DMC

For ease of reference, we compiled the main symbols used in this paper in Table II.

## IV. THE ADMCCS PROBLEM

In this section, we first provide the structure of the charging schedule list, then define the ADMCCS problem, present its formulation, and prove that it is NP-hard.

## A. Structure of Charging Schedule List

A charging schedule for fulfilling a charging task set involves the DMC’s charging trajectory design, the charging tour time scheduling, and the determination of charging direction and charging time. We represent a complete charging schedule using an ordered list named the DMC’s Operation Schedule (DOS). Here, we only consider schedules that, by default, satisfy all nodes’ energy demands.

Let $\mathbf { s } _ { \mathrm { M C } } { = } [ s _ { 1 } ^ { E } , s _ { 2 } ^ { E } , \ldots , s _ { m _ { 1 } } ^ { E } ]$ represent a DOS list with $m _ { 1 }$ items, where each item is a tuple $s _ { i } ^ { E } { : = } ( s t a t e , l , \psi , t )$ . These items are classified into two categories: movement schedule items for arranging travel between charging positions, and energy transmission schedule items for arranging the energy transmission operation along directions at certain positions. state∈0, 1 is a binary variable indicating the category of the item. An item with state=0 means that the DMC should move toward position l for a time duration t, and the field $\psi$ is not used. An item with $s t a t e { = } 1$ means that the DMC should transmit energy at position l along direction ψ for a time duration t. For example, a schedule item (1, 3, 45, 5) means that the DMC, which is at position $l _ { 3 } ,$ should transmit energy along the $4 5 ^ { \circ }$ direction for 5 time units. A schedule item (0, 4, 90, 6) means that the DMC should move toward position $l _ { 4 }$ for 6 time units.

Let $S _ { \mathrm { M o v e } } ^ { \mathrm { D O S } }$ and $S _ { \mathrm { T r a n } } ^ { \mathrm { D O S } }$ denote the sets of movement schedule items and charging schedule items, respectively. Then, $\mathcal { S } _ { \mathrm { M o v e } } ^ { \mathrm { D O S } } { = } \{ s _ { i } ^ { E } | s _ { i } ^ { E } . s t a t e { = } 0 \}$ and $S _ { \mathrm { T r a n } } ^ { \mathrm { D O S } } { = } \{ s _ { i } ^ { E } | s _ { i } ^ { E } . s t a t e { = } \dot { 1 } \}$

For an item $s _ { i } ^ { E } { \in } \mathbf { s } _ { \mathrm { M C } }$ , we use s<sup>E</sup>.state to access the state value of the item. This rule applies to other fields of the schedule items. As a list, the items in s<sub>MC</sub> are executed sequentially to fulfill the schedule. Let $\textstyle { \tau _ { i } ^ { \mathrm { E } } : = \sum _ { j = 1 } ^ { i } s _ { j } ^ { \mathrm { E } } . t }$ , then item $s _ { i } ^ { E }$ starts at time $\tau _ { i - 1 } ^ { \mathrm { E } }$ and ends at time $\tau _ { i } ^ { \mathrm { E } }$

TABLE II: SYMBOL DEFINITIONS

<table><tr><td>Symbols</td><td>Description</td></tr><tr><td> $\mathcal{U}, u_i, N$ </td><td> $\mathcal{U}:=\{u_1, u_2, \ldots, u_N\}$  denotes the set of nodes with node number  $M$ , where  $u_i$  is the  $i$ -th node.</td></tr><tr><td> $\mathcal{L}, l_i, L$ </td><td> $\mathcal{L}:=\{l_1, l_2, \ldots, l_L\}$  denotes the set of charging positions with position number  $L$ , where  $l_i$  is the  $i$ -th position  $l_i$ .</td></tr><tr><td> $\mathbf{r}(\mathcal{L})$ </td><td>A charging tour on  $\mathcal{L}$ , i.e., a tour traverses all positions in  $\mathcal{L}$ .</td></tr><tr><td> $e_B, e_D, e_C$ </td><td> $e_B:=\{e_1^B, \ldots, e_N^B\}$  is the list of initial energy of all nodes.  $e_D$  and  $e_C$  are the energy demanding list and storage capacity list.</td></tr><tr><td> $e_F, e_R$ </td><td>The lists of final energy and received energy of all nodes.</td></tr><tr><td> $e_{B0}, e_{F0}$ </td><td>DMC&#x27;s initial energy and final energy.</td></tr><tr><td> $p_0, \overline{v}, w$ </td><td>Energy transmission power, moving speed, energy consumption base rate of the DMC.</td></tr><tr><td> $\psi, \varphi, D, \theta$ </td><td>Charging direction angle, charge sector angle, charge distance, and node direction angle.</td></tr><tr><td> $e_{\text{Tran}}^{\text{MC}}(s), e_{\text{Move}}^{\text{MC}}(s), e_{\text{Loss}}^{\text{MC}}(s)$ </td><td>Charging energy consumption, movement energy consumption, and total energy loss of the DMC for fulfilling a charging schedule  $s$ .</td></tr><tr><td> $e_{\text{Loss}}^{\text{Total}}(s), e_{\text{Rcv}}^{\text{Nodes}}(s)$ </td><td>Total energy loss and total energy received by all nodes for fulfilling a charging schedule  $s$ .</td></tr><tr><td> $e_{\text{Tran}}^{\text{MC}}(\mathbf{t}), e_{\text{Rcv}}^{\text{Nodes}}(\mathbf{t}), e_{\text{Loss}}^{\text{WPT}}(\mathbf{t})$ </td><td>Total energy loss, total energy received by all nodes, and the charging energy loss for fulfilling a charging schedule with energy transmission time list  $\mathbf{t}$ .</td></tr><tr><td> $t_{\text{Tran}}^{\text{MC}}(s), t_{\text{Move}}^{\text{MC}}(s)$ </td><td>Energy transmission time list and movement time list of the DMC in a charging schedule  $s$ .</td></tr><tr><td> $\mathbf{C}, \mathbf{D}, \mathbf{W}$ </td><td>Energy transfer coefficient matrix, RA distance matrix, RA movement energy consumption matrix.</td></tr><tr><td> $\mathbf{s}_{\text{MC}}$ </td><td>A charging schedule consists of schedule items with structure ( $state, l, \psi, t$ ).</td></tr><tr><td> $d(l_i, l_j), w(l_i, l_j)$ </td><td>Routing-asymmetric distance and routing-asymmetric energy consumption of the path segment from position  $l_i$  to position  $l_j$ .</td></tr><tr><td> $\mathcal{S}_{\text{Dir}}(l_i), \mathcal{S}_{\text{PosDir}}$ </td><td> $\mathcal{S}_{\text{Dir}}(l_i)$  is the set of charging directions at position  $l_i$ ,  $\mathcal{S}_{\text{PosDir}} := \cup_{l_i \in \mathcal{L}} \mathcal{S}_{\text{Dir}}(l_i)$ .</td></tr></table>

## B. The ADMCCS Problem

For an instance of the ADMCCS problem, the total initial energy $e _ { \mathrm { T B } } { : = } e _ { \mathrm { B 0 } } { + } \mathbb { 1 } ^ { 1 \times N } \mathbf { e } _ { \mathrm { B } }$ is a constant. For a DOS charging schedule s, the total final energy of all nodes and the DMC is $e _ { \mathrm { T F } } ( \mathbf { s } ) { : = } e _ { \mathrm { F 0 } } { + } \mathbb { 1 } ^ { 1 \times N } \mathbf { e } _ { \mathrm { F } }$ . Therefore, the total energy loss for fulfilling schedule s is $e _ { \mathrm { L o s s } } ^ { \mathrm { T o t a l } } ( \mathbf { s } ) { : = } e _ { \mathrm { T B } } { - } e _ { \mathrm { T F } } ( \mathbf { s } )$ , and the total energy consumption of the DMC after completing the schedule s is $e _ { \mathrm { L o s s } } ^ { \mathrm { M C } } ( \mathbf { s } ) { : = } e _ { \mathrm { B 0 } } { - } e _ { \mathrm { F 0 } }$ . We further define $e _ { \mathrm { R c v } } ^ { \mathrm { N o d e s } } ( \mathbf { s } ) { : = } \mathbb { 1 } ^ { 1 \times N } ( \mathbf { e } _ { \mathrm { F } } { - } \mathbf { e } _ { \mathrm { B } } )$ . Thus, we have the total energy loss $e _ { \mathrm { L o s s } } ^ { \mathrm { \bar { T o t a l } } } ( \mathbf { s } ) { = } e _ { \mathrm { L o s s } } ^ { \mathrm { M C } } ( \mathbf { s } ) { - } e _ { \mathrm { R c v } } ^ { \mathrm { N o d e s } } ( \mathbf { s } )$ $\mathbf { A s } ~ e _ { \mathrm { L o s s } } ^ { \mathbf { M C } } ( \mathbf { s } )$ and $e _ { \mathrm { R c v } } ^ { \mathrm { N o d e s } } ( \mathbf { s } )$ are both completely determined by the energy transmission time list t embedded in schedule s, they can also be expressed as $e _ { \mathrm { L o s s } } ^ { \mathrm { M C } } ( \mathbf { t } )$ and $e _ { \mathrm { R c v } } ^ { \mathrm { N o d e s } } ( \mathbf { t } )$ . Since $e _ { \mathrm { L o s s } } ^ { \mathrm { M C } } ( \mathbf { t } ) { - } e _ { \mathrm { R c v } } ^ { \mathrm { N o d e s } } ( \mathbf { t } )$ represents the energy loss resulting from energy transmission, we call it charging energy loss and define it as in Eq. (13).

$$
e _ {\text { Loss }} ^ {\text { WPT }} (\mathbf {t}) := e _ {\text { Loss }} ^ {\text { MC }} (\mathbf {t}) - e _ {\text { Rcv }} ^ {\text { Nodes }} (\mathbf {t}).\tag{13}
$$

The goal here is to minimize the total energy loss while satisfying the energy demands of all nodes. With this objective, our targeted problem in this paper, named the Asymmetric DMC Charging Schedule (ADMCCS) problem, can be formally stated as follows:

ADMCCS Problem: Given an RA-WRSN consisting of a DMC, N nodes, and a BS at position $l _ { 0 } ,$ where the nodes have parameters including initial energy list $\mathbf { e _ { B } } ,$ energy demand list $\mathbf { e _ { D } } ,$ and battery capacity list $\mathbf { e _ { C } } ,$ and the DMC has parameters including energy transmission sector angle $\varphi ,$ transmission power p<sub>0</sub> (corresponding to charge distance D), and move speed v (m/s), the task is to find a charging schedule $\mathbf { s _ { C S } }$ with minimum energy loss $\mathbf { e _ { L o s s } ^ { T o t a l } }$ while ensuring that all charging demands are satisfied.

The ADMCCS problem can be formulated as in Eq. (14).

(P1)

$$
\begin{array}{l l} \min _ {\mathcal {L},   \mathbf {r} (\mathcal {L}),   \mathcal {S} _ {\varphi} (\mathcal {L}),   \mathbf {t} _ {\text {Tran}} ^ {\text {MC}}} & e _ {\text {Loss}} ^ {\text {Total}} (\mathbf {r} (\mathcal {L}), \mathcal {S} _ {\varphi} (\mathcal {L}), \mathbf {t} _ {\text {Tran}} ^ {\text {MC}}), \\ \text {s.t.} & C 1: \mathbf {e} _ {\mathbf {F}} \geq \mathbf {e} _ {\mathbf {B}} + \mathbf {e} _ {\mathbf {D}}; \\ & C 2: \mathbf {t} _ {\text {Tran}} ^ {\text {MC}} \geq \mathbf {0}; \\ & C 3: \mathcal {L} \in \Omega ,   \mathbf {r} (\mathcal {L}) \in \Omega_ {\mathcal {L}},   \mathcal {S} _ {\varphi} \in [ 0, 2 \pi); \end{array}\tag{14}
$$

In Eq. (14), Ω denotes the solution space of charging positions, i.e., the region accessible for charging, and $\Omega _ { \mathrm { r } } ( \mathcal { L } )$ denotes the solution space of valid charging tours traversing all positions in $\mathcal { L } .$ The interval [0, 2π) represents the space of charging directions. C1 ensures that all nodes’ energy demands are satisfied. Here, for two vectors $\mathbf { a } { : = } [ a _ { 1 } , a _ { 2 } , \ldots , a _ { n } ]$ and $\mathbf { b } { \mathrm { : = } } [ b _ { 1 } , b _ { 2 } , \ldots , b _ { n } ]$ , a≥b means $a _ { i } \geq b _ { i }$ for all $i { \in } \{ 1 , 2 , \ldots , n \}$

In the objective function in Eq. (14), the dependencies of $e _ { \mathrm { L o s s } } ^ { \mathrm { T o t a l } }$ on $\mathbf { r } ( \mathcal { L } ) , S _ { \varphi } ( \mathcal { L } )$ , and $\mathbf { t } _ { \mathrm { T r a n } } ^ { \mathrm { M C } }$ are explicitly emphasized.

## Theorem 1. The ADMCCS problem is NP-hard.

Proof. We prove this by showing that a restricted version of ADMCCS is indeed the Asymmetric Traveling Salesman Problem (ATSP), which is known to be NP-hard [30].

To this end, consider the ADMCCS problem with the following additional constraints: The charging distance D of the DMC is very small, i.e., D=0, such that the DMC can only charge a node exactly at the node’s location. This constraint prohibits the M2M charging mode and allows only the O2O charging mode. Thus, the DMC has to charge the nodes one by one, providing exactly the amount of energy that satisfies their energy demands—no more, no less. In this situation, the charging order makes no difference in terms of charging energy consumption and the total energy received by the nodes. As a consequence, the quality of a charging schedule is solely determined by its movement energy consumption, which is completely determined by the charging trajectory of the DMC.

Therefore, finding an optimal charging schedule with minimum energy loss for the restricted version of the ADMCCS problem corresponds to the task of finding an optimal charging trajectory of the DMC with minimum movement energy consumption, which is indeed the ATSP problem. □

## V. SOLVE THE ADMCCS PROBLEM

As the ADMCSS problem is NP-hard, we focus on designing efficient approximation algorithms. To achieve this, and inspired by [16], we address the problem in four steps, as outlined in Fig. 1 in Sec. I.

Step 1: Select a minimum-size set of charging positions that ensures all nodes can be charged by the DMC from some of these positions.

Step 2: Select a minimum-size set of functional-equivalent charging directions at the charging positions.

Step 3: Determine the energy transmission time duration along the directions at all the charging positions.

Step 4: Find an energy-efficient loop tour to traverse the charging positions in RA-WRSNs.

## A. Select Minimum-Size Charging Position Set

Considering the charge distance D, we define a node as being covered by a charging position if the distance between the node and the charging position is not greater than D. A charging position set that ensures all nodes are covered by at least one position in the set is called a valid charging position set.

In this step, our goal is to determine a valid charging position set with the minimum size and the smallest total sum of a distance metric. Let L denote a charging position set, and let Ω denote the solution space of charging position sets. The task in this step can be formulated as in Eq. (15).

$$
\begin{array}{ll}\mathcal{L}^{*} = & \arg \min_{\mathcal{L}}|\mathcal{L}| + \lambda \sum_{l_{i}\in \mathcal{L}}d_{i}^{\max},\\ \text{s.t.} & C4:d_{i}^{\max} = \min_{l_{j}\in \mathcal{L}}d(u_{i},l_{j}),u_{i}\in \mathcal{U}.\\ & C5:d_{i}^{\min}\leq D,u_{i}\in \mathcal{U}.\\ & C6:d_{j}^{\max} = \max_{\substack{u_{i}\in \{u_{i}|u_{i}\in \mathcal{U},\\ d(u_{i},l_{j})\leq D\}}}d(u_{i},l_{j}),l_{j}\in \mathcal{L};\\ & C7:\mathcal{L}\in \Omega_{L}. \end{array} \tag{P2}\tag{15}
$$

In Eq. (15), the objective is a combination of the size of the charging position set and the sum of the distances between a charging position and the farthest node covered by it. The coefficient λ adjusts the relative importance of the distance metric and the number of charging positions $| { \mathcal { L } } |$ . Constraint C4 defines $d _ { i } ^ { \operatorname* { m a x } }$ as the smallest distance between node $u _ { i }$ and a charging position in ${ \mathcal { L } } ,$ and constraint C5 requires that each node must be covered by at least one charging position in ${ \mathcal { L } } .$ Constraint C6 defines $d _ { j } ^ { \operatorname* { m a x } }$ as the maximum distance from a node covered by a charging position $l _ { j } \in \mathcal { L }$ to the position itself, and constraint C7 restricts the solution space.

## Theorem 2. The problem P2 is NP-complete.

Proof. By removing the sector term in the objective function of P1, problem P1 is reduced to a new problem, which is essentially the Unit Disk Cover (UDC) problem [31]. The UDC problem is stated as follows: In the Euclidean plane, given a set $P { = } \{ p _ { 1 } , p _ { 2 } , \cdots , p _ { n } \}$ of n points and a set of m unit disks with centers in a set $L { = } \{ l _ { 1 } , l _ { 2 } , \cdots , l _ { m } \}$ , the task is to determine a minimum-size set $L ^ { * } \subseteq L$ such that all points in $P$ are covered by the union of the unit disks with centers in $L ^ { * }$ The UDC problem is NP-complete [31]; therefore, problem P2 is NP-complete. □

Since P2 is NP-hard, we solve it in two steps guided by its objective function: (1) find a valid charging position set with minimum size; (2) refine the charging positions in the set to minimize $\textstyle \sum _ { l _ { i } \in { \mathcal { L } } ^ { * } } d _ { i } ^ { \mathrm { m a x } }$

The node set covered by a charging position can be regarded as a cluster, with the charging position serving as the cluster center. Thus, clustering methods can be employed to address the first step. We propose a K-means Charging Position Generation (KCPG) algorithm to return a valid charging position set in polynomial time. This algorithm automatically adjusts the position set size and ultimately obtains the clusters and their center points. The specific code is provided later in Section VI.

The second step involves determining the smallest enclosing circle that covers each node cluster, returning the center point and the diameter of the smallest circle. This can be solved using the well-known Welzl algorithm [32] or by using mature software packages such as MATLAB or SciPy.

## B. Determine Minimum-Size Functional-Equivalent charging direction Set

At each charging position $l _ { i } { \in } { \mathcal { L } } ^ { * }$ , the DMC should transmit energy along selected directions. The DMC can take any direction within the infinite continuous range [0, 2π). Given that the charge distance D and the charge sector angle $\varphi$ of the DMC are constants, we can use the Create Minimum Functional Representative Direction Set (cMFRAS) algorithm from [14] to obtain a minimum-size discrete direction set that is functionally equivalent to the set [0, 2π). These directions are then used as the charging directions. The optimality of cMFRAS was established in [14].

In this paper, following the process outlined in [14], we determine the functionally equivalent direction set for each position and collect them into the set of Pos-Dir pairs $\mathcal { S } _ { \mathrm { P o s D i r } }$ Let us assume $\begin{array} { r } { | S _ { \mathrm { P o s D i r } } | { = } K } \end{array}$ . Each Pos-Dir pair is then regarded as a Virtual MC (VMC). As described in Sec. III-D, the energy transfer coefficients from the VMCs to the nodes are collected into an energy transfer coefficient matrix C, where each row corresponds to a VMC, and each column corresponds to a node.

## C. Determine Time Duration for all charging directions

Once the set $\mathcal { S } _ { \mathrm { P o s D i r } }$ of Pos-Dir pairs has been determined, we need to determine the energy transmission times of the DMC along the specified charging directions at the corresponding charging positions in each Pos-Dir pair, or, for short, along the Pos-Dir pairs. The complete set of energy transmission times determines the amount of energy that the nodes in the RA-WRSN can receive.

Recall that $\mathbf { t } { : = } [ t _ { 1 } , t _ { 2 } , \ldots , t _ { K } ] ^ { \mathrm { T } }$ denotes the column vector of energy transmission times along the Pos-Dir pairs in a charging schedule s. Since the movement energy consumption of the

DMC is not affected by t, minimizing the total energy loss $e _ { \mathrm { L o s s } } ^ { \mathrm { T o t a l } } ( \mathbf { s } )$ is equivalent to minimizing $e _ { \mathrm { T r a n } } ^ { \mathrm { M C } } ( \mathbf { t } ) { - } e _ { \mathrm { R c v } } ^ { \mathrm { N o d e s } } ( \mathbf { t } )$ . As the DMC transmits energy with a constant power $p _ { 0 } ,$ , minimizing the charging energy consumption of the DMC is identical to minimizing the sum of the elements in t. Therefore, the transmission times can be determined by solving the problem presented in Eq. (16).

$$
\begin{array}{l l} \text {(P3)} & \mathbf {t} ^ {*} = \underset {\mathbf {t}} {\arg \min} \mathbb {1} ^ {1 \times K} \mathbf {t}, \\ & s. t. \quad C 1, C 2; \\ & \quad C 8: \mathbf {t} \geq \mathbf {0}; \\ & \quad C 9: E q. (1 3). \end{array}\tag{16}
$$

We use Cplex to solve this problem, and denote the solution as $\mathbf { t } ^ { * }$ . By combining $\mathbf { t } ^ { * }$ with the Pos-Dir pairs in the set $\mathcal { S } _ { \mathrm { P o s D i r } } .$ we can construct the energy transmission schedule item set $S _ { \mathrm { T r a n } } ^ { \mathrm { D O S } }$

## D. Asymmetric Path Planning in RA-WRSN

The objective of this step is to determine the minimum movement energy consumption loop tour r that traverses all charging positions in $\mathcal { L } ^ { \ast }$ , which were determined in the first step. In RA-WRSNs, the RA distances and energy consumption rates of the bidirectional path segments between points determine the travel time and energy consumption of the loop path. Routing asymmetry is the main challenge in this step. Recall that the effects of the routing asymmetry factors are ultimately manifested in the RA distance matrix D and the RA movement energy consumption matrix W.

Let $\mathbf { r } _ { \pi } { : = } [ l _ { \pi _ { 0 } } { = } l _ { 0 } , l _ { \pi _ { 1 } } , \ldots , l _ { \pi _ { L } } , l _ { 0 } ]$ represent a charge tour consisting of $L { + 2 }$ points. Let $\{ \mathbf { r } _ { \pi } \}$ denote the position set contained in $\mathbf { r } _ { \pi }$ , and let $\left| \mathbf { r } _ { \pi } \right|$ denote the number of points in $\mathbf { r } _ { \pi } , \mathrm { i . e . , } | \mathbf { r } _ { \pi } | { = } L { + } 2$ . The movement energy consumption of the tour $\mathbf { r } _ { \pi }$ is $e _ { \mathrm { M o v e } } ^ { \mathrm { M C } } ( { \bf r } _ { \pi } )$ . The task of asymmetric path planning for RA-WRSNs in this step can be formulated as follows:

$$
\begin{array}{l l} \text {(P4)} & \mathbf {r} _ {\pi} ^ {*} = \quad \arg \min _ {\mathbf {r} _ {\pi}} e _ {\text {Move}} ^ {\text {MC}} (\mathbf {r} _ {\pi}) \\ & s. t. \quad C 1 0: \{\mathbf {r} _ {\pi} \} = \mathcal {L} ^ {*} \cup \{l _ {0} \}, \\ & C 1 1: | \mathbf {r} _ {\pi} | \geq | \mathcal {L} ^ {*} | + 1, \end{array}\tag{17}
$$

In Eq. (17), constraint C10 ensures that all positions in ${ \mathcal { L } } ^ { * } \cup \{ l _ { 0 } \}$ are traversed by the charge tour $\mathbf { r } _ { \pi } .$ . Constraint C11 ensures that the tour $\mathbf { r } _ { \pi }$ visits each point in $\mathcal { L } ^ { \ast }$ at least once.

In a symmetric Euclidean routing environment, due to the triangle inequality, a tour usually passes through each node exactly once. However, in an RA-WRSN, the triangle inequality does not necessarily hold, so sometimes a tour with sub-loops may be more efficient than a non-sub-loop tour. Therefore, C10 allows tours with sub-loops, which makes problem P4 different from the ATSP.

## Theorem 3. Problem P4 is NP-hard.

Proof. By applying an additional restriction that does not allow sub-tours, we obtain a restricted version of Problem P4. This restricted version is simply the Asymmetric Traveling Salesman Problem (ATSP). Since the ATSP is NP-hard [30], Problem P4 is also NP-hard. □

To solve P4, we use the most efficient and powerful heuristic algorithm, known as LKH, which achieves state-of-the-art performance in both solution quality and running speed. LKH performs exceptionally well in solving the ATSP.

Instead of solving the ATSP directly, another typical approach is to first transform the ATSP into a TSP, which can be done using the method proposed in [27], and then solve it using traditional TSP algorithms. However, this transformation doubles the number of points to be visited, which significantly challenges TSP algorithms and greatly hinders their performance. This drawback is validated by the experiments in Sec. VII-E.

Let $\mathbf { r } _ { \pi } ^ { * }$ denote the energy-minimizing path obtained using LKH. Based on $\mathbf { r } _ { \pi } ^ { * }$ , we can derive the movement time duration list $\pmb { \mathrm { t } } _ { \mathrm { M o v e } } ^ { M C } = [ d \ddot { l } _ { \pi _ { 0 } } , l _ { \pi _ { 1 } } ) / \overline { { v } } , d ( l _ { \pi _ { 1 } } , l _ { \pi _ { 2 } } ) / \overline { { v } } , \ldots , d ( l _ { \pi _ { L } } , l _ { 0 } ) / \overline { { v } } ]$ thereby determining all movement schedule items $S _ { \mathrm { M o v e } } ^ { \mathrm { D O S } }$

Additionally, based on $\mathbf { r } _ { \pi } ^ { * } .$ , we arrange the energy transmission schedule items $S _ { \mathrm { T r a n } } ^ { \mathrm { D O S } }$ . For the schedule items corresponding to the directions of a single charging position, the items are sorted in ascending order of direction angles.

By combining the items in $S _ { \mathrm { M o v e } } ^ { \mathrm { D O S } }$ and $S _ { \mathrm { T r a n } } ^ { \mathrm { D O S } }$ , we can easily construct a charging schedule s as the final solution to the ADMCCS problem.

## VI. THE RA-DMCS ALGORITHM

Based on the analyses in Sec. IV, we propose a heuristic algorithm, named RA-DMCS, to approximately solve the ADMCCS problem. As shown in the pseudocode in Alg. 1, RA-DMCS accepts inputs such as e<sub>B</sub>, e<sub>C</sub>, and generates a charging schedule s<sub>MC</sub>. As outlined in Fig. 1, RA-DMCS consists of four steps. It selects charging positions in step 1 (code line 2), and determines charging directions in step 2 (code line 3). In step 3, it formulates the problem following Eq. (16) (code line 4), determines charging times for all Pos-Dir pairs (code line 5), and constructs the schedule item set $S _ { T r a r } ^ { \mathrm { M C } }$ (code line 6). In step 4, it generates a charging tour $\mathbf { r } ^ { * }$ using the LKH algorithm (code line 7) and constructs the schedule item set $\mathcal { S } _ { M o v e } ^ { \breve { \mathbf { M } } \mathrm { C } }$ (code line 8). Finally, it constructs s and returns the result.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 1 The RA-DMCS algorithm
Input: U,  $e_{B}$ ,  $e_{D}$ ,  $e_{C}$ ,  $p_{0}$ ,  $e_{B0}$ , D,  $\varphi$ ,  $\overline{v}$ ;
Output:  $s_{MC}$ ;
1: Initial: C,  $S_{dir}$ ;
2:  $\mathcal{L}^{*}=KCPG(U,D)$ ;  $\triangleright$  Determine a charging position set, detailed in Alg. 1
3: Construct Pos-Dir pair set  $S_{PosDir}$  following the process in [14] by using the cMFRDS algorithm;
4: Construct the problem following Eq. (16);
5: Obtain  $t^{*}$  using CPLEX;
6: Construct  $S_{Tran}^{MC}$  based on  $S_{PosDir}$  and  $t^{*}$ ;
7:  $r^{*}=LKH(\mathcal{L}^{*},D,W)$ ;
8: Construct  $S_{Move}^{MC}$  based on  $r^{*}$  as introduced in Sec. IV;
9: Construct  $s_{MC}$  by combing  $S_{Move}^{MC}$  and  $S_{Tran}^{MC}$ );
10: return  $s_{MC}$ ;
</div>

## A. Using LKH to Solve P2

We adapted LKH for the ATSP problem to solve P4. LKH [17] was originally designed for solving standard TSP problems and was later adapted to the ATSP problem. According to [33], although LKH is heuristically approximate, computational experiments have demonstrated its high efficiency. As stated in [33], optimal solutions are produced with an impressively high frequency.

LKH typically starts with a heuristic initial solution, then tries to converge quickly to a high-quality solution by repeatedly applying k-opt sub-tour swaps. A k-opt sub-tour swap in LKH adjusts the tour by replacing k path segments in the tour with k new ones. Each iteration involves searching for the best k-opt sub-tour swap that most effectively reduces the total tour length. A crucial component of LKH is its effective swap quality evaluation method, which efficiently assesses the potential improvement of a k-opt sub-tour swap before it is applied, prioritizing the swaps with higher values to ensure significant reductions in tour length with each swap. LKH dynamically adjusts the value of k, allowing it to explore a larger solution space and escape local optima. It also dynamically adjusts the type of k-opt sub-tour swaps to perform, makes instancespecific adaptations, and thus effectively balances exploration and exploitation.

## B. KCPG Algorithm

We propose a K-means Charging Position Generation (KCPG) algorithm to solve Eq. (15). The pseudocode for KCPG is shown in Alg. 2. The algorithm uses j to denote the number of clusters, initializing it as 1. The K-means clustering algorithm is utilized to group the nodes into j clusters (code line 3). For each cluster, the center of the minimum circle covering the node set in the cluster is obtained (code line 4). More clusters are constructed unless all minimum circles have radii smaller than the charge distance D. Finally, the centers of the clusters are returned as $\mathcal { L } .$

```txt
Algorithm 2 The KCPG algorithm
Input: Node set U, charging distance D;
Output: The charging position set L.
1: Initialize j=1;
2: while true do
3: G= Kmeans(U, j);
4: {L, RM}= Optimiz(G);
5: return L if r≤D ∀r∈RM otherwise j=j+1;
6: end while
```

Theorem 4. The time complexity of the KCPG algorithm is $O ( n ^ { 3 } )$

Proof. KCPG iteratively calls the K-means clustering algorithm with an increasing number of clusters initialized as 1. In each iteration for a certain cluster number, K-means is used to cluster the nodes, and then the minimum enclosing circles for covering the nodes in each cluster are obtained using the Welzl algorithm [32], and the results are checked to determine if the results are acceptable, otherwise a new iteration with the number of clusters increased by 1 is conducted.

According to [34], K-means itself has a time complexity of $O ( t k n )$ , where t is the number of iterations, k is the number of clusters, n is the number of data points, and $m { = } 2$ is the dimension of the data points. The welzl algorithm [32] has time complexity $O ( n / k )$ , where $n / k$ is the size of a cluster. Running welzl for k clusters requires time $O ( k \cdot n / k ) =$ $O ( n )$ . Thus, an iteration with k clusters has time complexity $O ( n t k ) { + } O ( n )$

In the worst-case, cluster number will eventually increase to n. Given that t and $t _ { m }$ are constants, the time complexity of KCPG can be obtained as

$$
\begin{array}{r l} {T} & {= O (n) \cdot (O (n t k) + O (n)) = O (n ^ {2} t k + n ^ {2})} \\ & {= O (n ^ {2} k + n ^ {2}) = O (n ^ {3}).} \end{array}\tag{18}
$$

## VII. PERFORMANCE EVALUATION

In this section, we comparatively evaluate the performance of RA-DMCS against some representative algorithms through experiments. The experiments are coded with python and conducted on a computer with AMD R7-6800HS CPU, 16GB RAM, and Ubuntu 20.04.5 LTS OS.

## A. Algorithms for Comparison

The algorithms in [10] and in [35] are selected for comparison. They are designated as One-to-One Greedy (O2OGre) and Greedy Grouping with Fixed-Direction Ant Colony (GFDA), respectively.

O2OGre[10] adopts a one-to-one charging approach, not exploiting the opportunity of multiple node simultaneous energy reception from the same energy signal. In O2OGre, the MC’s charge tour decision problem is regarded as a symmetric TSP problem and solved using greedy algorithm.

GFDA [35] also adopts a grouped directional charging approach similar to us, but it is devoted to routing symmetric WRSNs. It begins by grouping the nodes using a greedy algorithm prioritized with energy demanding amount. Within each group, the center of the circle is selected as the charging position, and a certain number of charging directions are selected uniformly from the [0, 2π) range. The MC’s charge tour decision problem is also regarded as a symmetric TSP problem, yet solved using an ant colony algorithm.

## B. Performance Metrics and Simulation Setup

Four main performance metrics are employed: total energy loss, tour distance, and time span. The energy loss metric represents the total energy loss consisting of charging energy loss and movement energy consumption. The tour distance metric represents the total distance of the charging tour in a charging schedule. The time span metric of a charging schedule is the sum of the time field of all schedule items.

Main simulation parameters and their default values are shown in Table III. A particular set of the values of the parameters is referred as a simulation setup. The impacts of the parameters on algorithms are investigated by experiments with various simulation setups, which differ only in the value of the parameter being inspected.

TABLE III: Simulation Parameters

<table><tr><td>Symbols</td><td>Values</td><td>Symbols</td><td>Values</td></tr><tr><td>N</td><td>200</td><td> $l_{BS}$ </td><td>(100, 100)</td></tr><tr><td>δ</td><td>4000</td><td> $\overline{v}$ </td><td>1 m/s</td></tr><tr><td> $e_{C}$ </td><td>60 ~ 90 J</td><td> $k_{RA,Dis}$ </td><td>0.5 ~ 1.5</td></tr><tr><td> $e_{B}$ </td><td>6 ~ 36 J</td><td>w</td><td>4 J/m</td></tr><tr><td> $e_{D}$ </td><td>18~75 J</td><td>D</td><td>20 m</td></tr><tr><td> $p_{0}$ </td><td>4  $W_{0}$ </td><td>φ</td><td>π/4</td></tr><tr><td>α</td><td>100</td><td>L</td><td>200 m</td></tr><tr><td>β</td><td>2</td><td> $k_{RA,Egy}$ </td><td>1</td></tr></table>

To simulate routing-asymmetry, the RA distance coefficients and RA movement energy consumption rate coefficients defined in Sec. III-D are determined randomly in range [0.5,1.5].

To reduce randomness in results, simulations for each simulation setup are repeated for 200 problem instances. The nodes in these instances are randomly distributed within a 200m×200m area. The performance metrics are averaged over the 200 simulations to obtain the final results, and the 95% confidence intervals are also calculated.

## C. Charge Tour Comparison for a Toy Network

We visually illustrate the differences between the algorithms by applying them to a toy network with 10 nodes in a 100×100 area and a BS at position [50,50]. For space limitation, only the RA movement energy consumption rate coefficients for this network are shown in Table IV.

TABLE IV: RA movement energy consumption rate coefficients for the toy network

<table><tr><td></td><td>BS</td><td>A</td><td>B</td><td>C</td><td>D</td></tr><tr><td>BS</td><td>0.00</td><td>1.11</td><td>1.10</td><td>1.14</td><td>1.35</td></tr><tr><td>A</td><td>0.89</td><td>0.00</td><td>1.03</td><td>1.31</td><td>1.43</td></tr><tr><td>B</td><td>0.90</td><td>0.97</td><td>0.00</td><td>1.33</td><td>1.15</td></tr><tr><td>C</td><td>0.86</td><td>0.69</td><td>0.67</td><td>0.00</td><td>1.11</td></tr><tr><td>D</td><td>0.65</td><td>0.57</td><td>0.85</td><td>0.89</td><td>0.00</td></tr></table>

The charge tours generated by the algorithms are shown in Fig. 4, with sub-figures (a), (b), and (c) show the tours determined by RA-DMCS, GFDA, and O2OGre, respectively. Fig. 4(b) provides the legends. In the sub-figures, the black dashed lines represent the DMC’s trajectory, with the number on each path indicating the corresponding movement energy consumption. A green poly-lines indicates the direction of an energy transmission, with the attached number represents the corresponding energy received by the nodes in that direction. The four positions designated as $\mathrm { A } \tilde { \mathrm { D } }$ are the selected charging positions. RA-DMCS exploits the routing-asymmetry property and generates a path outperforms the others. Comparatively, both O2OGre and GFDA could not deal with the routing-asymmetry characteristics well as designed for routing-symmetric environment.

![](images/a77a9bb27938f9f043be5fd9cad4ec03d06a8e3e7da284224977fdcf2d9bdd41.jpg)  
Fig. 4: Charging tours determined by the algorithms

Table V presents the detailed performance metrics of the algorithms for the network. Notably, RA-DMCS demonstrates lower energy consumption, higher charging efficiency, and shorter time span.

TABLE V: Performance of the algorithms for the toy network

<table><tr><td>Performance Metric</td><td>O2OGre [10]</td><td>RA-DMCS (Our)</td><td>GFDA [35]</td></tr><tr><td>Energy loss (J)</td><td>1844.82</td><td>1583.87</td><td>1912.02</td></tr><tr><td>Time span (s)</td><td>579.43</td><td>524.63</td><td>596.23</td></tr><tr><td>Charging energy loss (J)</td><td>709.38</td><td>768.51</td><td>847.93</td></tr><tr><td>Movement energy consumption (J)</td><td>1135.44</td><td>815.36</td><td>1064.09</td></tr><tr><td>Charging time (s)</td><td>295.57</td><td>312.78</td><td>330.21</td></tr><tr><td>Moving time (s)</td><td>283.86</td><td>211.85</td><td>266.02</td></tr></table>

## D. Simulation Results and Analyses

We conducted simulation experiments to examine the effects of key parameters on the performance of the algorithms. In these experiments, we make N increase from 50 to 450 with an increment of 50, whereas the other parameters take their default values as in Table III. The results are shown in Fig. 5.

Only the results showing the effects of the number of nodes are provided here for space limitation.

The results in Fig. 5(a) shows that, as the number of nodes increases, the energy loss of the three algorithms also increases, yet RA-DMCS consistently maintains a lower energy consumption level than the other two algorithms. As the number of nodes increases, energy consumption of GFDA and RA-DMCS both tend to stabilize. This comes from two fold effects of increasing node number. On the one hand, compared to the large increase in node number, the number of clusters does not increase as much, which in turn makes the movement energy consumption does not increase much. On the other hand, as the number of nodes increases, each cluster will contain more nodes, and more energy will be harvested by them from the DMC’s energy transmission signal, thus leading reduced energy loss. As an integrated effect, the energy consumption of the algorithms become more stable.

As shown in Fig. 5(b), as the number of nodes increases, the time span of the solutions returned by the algorithms all increase. Compared with O2OGre, time spans of GFDA and RA-DMCS are much lower, as they both exploit M2M charging mode. Compared to O2OGre, Exploiting M2M not only reduces the number of charging positions, which leads to reduced move time and movement energy consumption, but also allows multiple nodes to be charged simultaneously, significantly shortening charging time. Compared with GFDA, our RA-DMCS employs an optimal direction selection algorithm and a better position selection method, making it outperforms GFDA.

Fig. 5(c) comparatively show the tour lengths of solutions obtained by the three algorithms. O2OGre utilizes the less efficient O2O charging mode, causing the DMC to visit all nodes to complete the entire charge task set, thus leading to a much longer tour length to complete the entire scheduling task. RA-DMCS also outperforms GFDA in term of this metric. Thus, RA-DMCS demonstrates the best performance among them in all the three performance metrics.

To better understand the performance of the algorithms, we further inspect the performance results in some detailed metrics, as shown in Fig. 6.

Fig. 6(a) provides the results of energy consumption related metrics including total energy loss, charging energy loss, movement energy consumption, with an additional metric of tour distance for facilitating the inspection. As the travel distance metric scales differently from the other metrics, its y-axis is separately placed on the right side of the figure chart. The results show that RA-DMCS outperforms the others in the terms of these energy related metrics.

Fig. 6(b) shows the results for time-related detail metrics including time span, charging time, moving time and algorithm runtime. RA-DMCS consumes the least time in terms of charging and moving times. RA-DMCS significantly outperforms GFDA and O2OGre. The results validate the superiority of RA-DMCS over others.

![](images/b9050cb8566375975a6723f641c5c01f662d41ec954c37cdff7732f5d04aeca3.jpg)  
(a) Energy loss

![](images/d8ac06d4e21d26144dd07ac65346af183cde3b87509ed1acf5ffe2cc4d2e855b.jpg)  
(b) Time span

![](images/5b939588e74be1d1a3352fa26e43f2a3748c0281a80fbe95c21c3586bd42ab47.jpg)  
(c) Tour distance

Fig. 5: Effects of the number of nodes  
![](images/a44e042c815929c6a3546b2c1092fd16d893fad2e7d952e278a777c2829fe16a.jpg)  
(a) Energy loss

![](images/989e065c3464c5542b472e749122e8cc6b41ea70823cdac7e83aa96ff6c8140a.jpg)  
(b) Time  
Fig. 6: Results of detail metrics of the algorithms

## E. Comparison of Different Approaches in solving Charging tour decision problem P4

Charging tour decision problem P4 in Eq. (17) for routingasymmetric WRSNs in the fourth step in Sec. V-D is a main challenge that distinguish our work from the literature. To concentrate on the performance of different approaches in solving the the charging tour decision problem, we select other five representative algorithms and test there performance in solving ATSP problem instances.

For representing heuristic algorithms, we select an ant colony based algorithm and an greedy algorithm, which are designated as ATSP Ant and ATSP Greedy, respectively. For the approach of transforming ATSP to TSP and then solve it using TSP algorithms, here we reuse LKH, ant colony, and greedy algorithm to solve the transformed TSP problem. We denote the corresponding entire method as TSP LKH, TSP Ant, and TSP Greedy, respectively. For the original LKH algorithm that directly applies to ATSP, we denote it as ATSP LKH for discrimination.

Simulation experiments similar to those in the previous section are conducted. We just provide the results for inspecting the effects of node number. The results of these six algorithms are shown in Fig. 7.

Fig. 7 shows that ATSP LKH demonstrates optimal overall performance in term of movement energy consumption. Although solving ATSP by converting it to an TSP problem is feasible, performance of algorithms using this approach are consistently inferior to the comparators of directly solving ATSP. Fig. 7 also shows that, while ATSP Ant performs well in solving ATSP, its comparator TCP Ant exhibits much poorer performance. This is because that, converting ATSP to TSP doubles the points to be visited, making the ant colony algorithm more likely to fall into local optima.

The results in Fig. 7(b) show that, although ATSP LKH performs much better than TSP LKH in term of movement energy consumption, as they differ only in the form of input data, they have similar performance in term of moving time.

Fig. 7(c) reveals that ATSP Greedy has the shortest runtime, but at the expense of slight performance degradation. ATSP LKH ranks second in this metric. Overall, ATSP LKH is the most preferable one.

## VIII. CONCLUSION

In this paper, we investigates the DMC scheduling problem for node recharging in routing-asymmetric WRSNs, termed as ADMCCS, aiming to determine the minimal energy loss charging strategy. We first prove that ADMCCS is an NPhard problem, and subsequently decompose it into four key components: charging position generation, determination of charging directions, optimal energy transmission time length and asymmetric path planning. Regarding charging position generation, we prove its NP-hard problem and propose an adaptive KCPG algorithm to minimize the number of charging positions while covering all nodes. For charging direction determination, we employ the cMFRDS algorithm to select a minimal functional representative direction set for each position. In third part, we formulate a linear programming optimization problem to minimize the total transmission time, solved using Cplex, to determine the optimal charging times for each direction. In the asymmetric path planning phase, we utilize the LKH algorithm to find paths with minimal energy consumption based on the charging position set. Finally, we present the RA-DMCS algorithm as a comprehensive solution to the ADMCCS problem, integrating the methodologies from the four aforementioned parts. Currently, our approach still has room for improvement, and we plan to further expand our work based on ADMCCS. We did not consider issues include the presence of obstacles in the network, energy allocation among nodes, dynamic considerations of changes in node distribution, and the collaborative operation of omnidirectional and directional charging vehicles. We aim to address these problems in future work.

![](images/f4ebe08123847bbb4d6ef74d9331476198ff36df571c0c1b582e57a2b6e5d63f.jpg)  
(a) Movement energy consumption

![](images/33e585ecfbd75d4659a080dd6f63a663fadea3920bd3676bd834169a7176d02e.jpg)  
(b) Moving time

![](images/bbd8594ed6d8e0915081dd245b7bd275711651448a378619b93e14f1fa012ebe.jpg)  
(c) Runtime  
Fig. 7: Comparison of different approaches to solve ATSP

## REFERENCES

[1] Y. Wang, H. T. Luan, Z. Su, N. Zhang, and A. Benslimane, “A secure and efficient wireless charging scheme for electric vehicles in vehicular energy networks,” IEEE Transactions on Vehicular Technology, vol. 71, no. 2, pp. 1491–1508, Feb. 2022. [Online]. Available: https://ieeexplore.ieee.org/document/9632356/

[2] Y.-N. Ma, Y.-J. Gong, C.-F. Xiao, Y. Gao, and J. Zhang, “Path planning for autonomous underwater vehicles: An ant colony algorithm incorporating alarm pheromone,” IEEE Transactions on Vehicular Technology, vol. 68, no. 1, pp. 141–154, jan 2019. [Online]. Available: https://ieeexplore.ieee.org/document/8540402/

[3] Z. Meng, H. Xu, M. Chen, Y. Xu, Y. Zhao, and C. Qiao, “Learning-driven decentralized machine learning in resource-constrained wireless edge computing,” in IEEE

INFOCOM 2021 - IEEE Conference on Computer Communications. Vancouver, BC, Canada: IEEE, may 2021, pp. 1–10. [Online]. Available: https: //ieeexplore.ieee.org/document/9488817/

[4] L. Fu, P. Cheng, Y. Gu, J. Chen, and T. He, “Optimal charging in wireless rechargeable sensor networks,” IEEE Transactions on Vehicular Technology, vol. 65, no. 1, pp. 278–291, jan 2016. [Online]. Available: http://ieeexplore.ieee.org/document/7006710/

[5] Z. Fan, Z. Jie, and Q. YuJie, “A survey on wireless power transfer based charging scheduling schemes in wireless rechargeable sensor networks,” in 2018 IEEE 4th International Conference on Control Science and Systems Engineering (ICCSSE). Wuhan, China: IEEE, aug 2018, pp. 194–198. [Online]. Available: https://ieeexplore.ieee.org/document/8724809/

[6] L. Xie, Y. Shi, Y. T. Hou, and A. Lou, “Wireless power transfer and applications to sensor networks,” IEEE Wireless Communications, vol. 20, no. 4, pp. 140–145, aug 2013. [Online]. Available: http://ieeexplore.ieee.org/ document/6590061/

[7] Z. Gao, Y. Chen, L. Fan, H. Wang, H. C.-H. Scott, and H.-C. Wu, “Joint energy loss and time span minimization for energy redistribution assisted charging of WRSNs with a mobile charger,” IEEE Transactions on Green Communications and Networking, vol. 10, no. 5, pp. 4636–4651, 2023. [Online]. Available: https://doi.org/10.1109/JIOT.2022.3219061

[8] M. Dorigo, M. Birattari, and T. Stutzle, “Ant colony optimization,” IEEE Computational Intelligence Magazine, vol. 1, no. 4, pp. 28–39, nov 2006. [Online]. Available: http://ieeexplore.ieee.org/document/4129846/

[9] R. George and T. A. J. Mary, “Review on directional antenna for wireless sensor network applications,” IET Communications, vol. 14, no. 5, pp. 715–722, mar 2020. [Online]. Available: https://onlinelibrary.wiley.com/doi/ 10.1049/iet-com.2019.0859

[10] C. Lin, K. Wang, Z. Chu, K. Wan, D. Jing, and G. Wu, “Hybrid charging scheduling schemes for threedimensional underwater wireless rechargeable sensor networks,” The Journal of Systems and Software,

vol. 146, pp. 42–58, dec 2018. [Online]. Available: https://doi.org/10.1016/j.jss.2018.09.002

[11] Z. Gao, D. Chen, and H.-C. Wu, “Graph coloring inspired approximate algorithm for wireless energy redistribution in WSNs,” IEEE Transactions on Green Communications and Networking, vol. 4, no. 1, pp. 42–58, 2020. [Online]. Available: https://doi.org/10.1109/TGCN.2019.2947172

[12] Y. Chen, H. Wang, D. Chen, Y. Jiang, Z. Gao, and J. Cao, “Energy redistribution assisted charging of wrsns with multiple mobile chargers having multiple base stations,” IEEE Transactions on Green Communications and Networking, no. 148, pp. 1–15, 2023. [Online]. Available: https://doi.org/10.1016/j.adhoc.2023.103213

[13] X. Liu, P. Lin, T. Liu, T. Wang, A. Liu, and W. Xu, “Objective-variable tour planning for mobile data collection in partitioned sensor networks,” IEEE Transactions on Mobile Computing, pp. 1–1, 2020. [Online]. Available: https://ieeexplore.ieee.org/document/ 9119834/

[14] Z. Gao, C. Liu, and Y. Chen, “Scheduling of ERD-Assisted charging of a WRSN using a directional mobile charger,” IEEE Transactions on Mobile Computing, vol. 23, no. 6, pp. 6681–6696, 2024. [Online]. Available: https://ieeexplore.ieee.org/document/10285040/

[15] T. Wu, P. Yang, H. Dai, W. Xu, and M. Xu, “Collaborated tasks-driven mobile charging and scheduling: A near optimal result,” in IEEE INFOCOM 2019 - IEEE Conference on Computer Communications. Paris, France: IEEE, apr 2019, pp. 1810–1818. [Online]. Available: https://ieeexplore.ieee.org/document/8737509/

[16] Z. Gao, D. Chen, and H.-C. Wu, “Energy loss minimization for wireless power transfer based energy redistribution in WSNs,” IEEE Transactions on Vehicular Technology, vol. 68, no. 12, pp. 12 271–12 285, dec 2019. [Online]. Available: https://ieeexplore.ieee.org/ document/8864040/

[17] Eric D. Taillard and K. Helsgaun, “POPMUSIC for<sup>´</sup> the travelling salesman problem,” European Journal of Operational Research, vol. 272, no. 2, pp. 420–429, jan 2019. [Online]. Available: https://doi.org/10.1016/j.ejor. 2018.06.039

[18] H. Dai, X. Wang, A. X. Liu, H. Ma, and G. Chen, “Optimizing wireless charger placement for directional charging,” in IEEE INFOCOM 2017 - IEEE Conference on Computer Communications. Atlanta, GA, USA: IEEE, May 2017, pp. 1–9. [Online]. Available: http: //ieeexplore.ieee.org/document/8057017/

[19] X. Wang, H. Dai, H. Huang, Y. Liu, G. Chen, and W. Dou, “Robust scheduling for wireless charger networks,” in IEEE INFOCOM 2019 - IEEE Conference on Computer Communications. Paris, France: IEEE, apr 2019, pp. 2323–2331. [Online]. Available: https: //ieeexplore.ieee.org/document/8737628/

[20] J.-R. Jiang and J.-H. Liao, “Efficient Wireless Charger Deployment for Wireless Rechargeable Sensor Networks,” Energies, 2016.

[21] T. Wu, P. Yang, H. Dai, W. Xu, and M. Xu, “Charging Oriented Sensor Placement and Flexible Scheduling

in Rechargeable WSNs,” in IEEE INFOCOM 2019 - IEEE Conference on Computer Communications. Paris, France: IEEE, Apr. 2019, pp. 73–81. [Online]. Available: https://ieeexplore.ieee.org/document/8737502/

[22] T. Liu, B. Wu, S. Zhang, J. Peng, and W. Xu, “An Effective Multi-node Charging Scheme for Wireless Rechargeable Sensor Networks,” in IEEE INFOCOM 2020 - IEEE Conference on Computer Communications. Toronto, ON, Canada: IEEE, Jul. 2020, pp. 2026– 2035. [Online]. Available: https://ieeexplore.ieee.org/ document/9155262/

[23] C. Lin, F. Gao, H. Dai, J. Ren, L. Wang, and G. Wu, “Maximizing Charging Utility with Obstacles through Fresnel Diffraction Model,” in IEEE INFOCOM 2020 - IEEE Conference on Computer Communications. Toronto, ON, Canada: IEEE, Jul. 2020, pp. 2046– 2055. [Online]. Available: https://ieeexplore.ieee.org/ document/9155274/

[24] H. Dai, C. Wu, X. Wang, W. Dou, and Y. Liu, “Placing wireless chargers with limited mobility,” in IEEE INFOCOM 2020 - IEEE Conference on Computer Communications. Toronto, ON, Canada: IEEE, jul 2020, pp. 2056–2065. [Online]. Available: https://ieeexplore.ieee.org/document/9155356/

[25] A. Tomar, R. Anwit, and P. K. Jana, “An efficient scheme for on-demand energy replenishment in wireless rechargeable sensor networks,” in 2017 International Conference on Advances in Computing, Communications and Informatics (ICACCI). Udupi: IEEE, sep 2017, pp. 125–130. [Online]. Available: http://ieeexplore.ieee.org/ document/8125828/

[26] S. Priyadarshani, A. Tomar, and P. K. Jana, “An efficient partial charging scheme using multiple mobile chargers in wireless rechargeable sensor networks,” Ad Hoc Networks, vol. 113, p. 102407, mar 2021. [Online]. Available: https://linkinghub.elsevier. com/retrieve/pii/S1570870520307344

[27] R. Jonker and T. Volgenant, “Transforming asymmetric into symmetric traveling salesman problems,” Operations Research Letters, vol. 2, no. 4, pp. 161– 163, Nov. 1983, publisher: North-Holland. [Online]. Available: http://www-sciencedirect-com-s.w.hqu.edu. cn:8118/science/article/abs/pii/0167637783900482

[28] H. Dai, H. Ma, and A. X. Liu, “Radiation constrained scheduling of wireless charging tasks,” in Proceedings of the 18th ACM International Symposium on Mobile Ad Hoc Networking and Computing. Chennai India: ACM, jul 2017, pp. 1–10. [Online]. Available: https: //dl.acm.org/doi/10.1145/3084041.3084060

[29] C. Lee, W. Na, G. Jang, C. Lee, and S. Cho, “Energy-efficient and delay-minimizing charging method with a multiple directional mobile charger,” IEEE Internet of Things Journal, vol. 8, no. 10, pp. 8291–8303, may 2021. [Online]. Available: https: //ieeexplore.ieee.org/document/9295334/

[30] C. H. Papadimitriou and S. Vempala, “On the approximability of the traveling salesman problem,” Combinatorica, vol. 26, no. 1, pp. 101–120, 2006,

iD: Papadimitriou\*2006. [Online]. Available: https: //doi.org/10.1007/s00493-006-0008-z

[31] R. J. Fowler, M. S. Paterson, and S. L. Tanimoto, “Optimal packing and covering in the plane are np-complete,” Information Processing Letters, vol. 12, no. 3, pp. 133–137, 1981. [Online]. Available: https: //doi.org/10.1016/j.adhoc.2023.103213

[32] E. Welzl, “Smallest enclosing disks (balls and ellipsoids),” vol. 555 LNCS, Graz, Austria, 1991, pp. 359 370. [Online]. Available: http://dx.doi.org/10.1007/BFb0038202

[33] “Lkh version 2.0.10 (november 2022).” [Online]. Available: http://akira.ruc.dk/<sup>∼</sup>keld/research/LKH/

[34] J. MacQueen, “Some methods for classification and analysis of multivariate observations,” in Proceedings of the fifth Berkeley symposium on mathematical statistics and probability, vol. 1. Oakland, CA, USA, 1967, pp. 281–297.

[35] Y. Liang, M. Yin, Y. Zhang, W. Wang, W. Jia, and T. Wang, “Grouping reduces energy cost in directionally rechargeable wireless vehicular and sensor networks,” IEEE Transactions on Vehicular Technology, vol. 72, no. 8, pp. 10 840–10 851, aug 2023. [Online]. Available: https://ieeexplore.ieee.org/document/10081057/
