---
title: "Minimizing the energy depletion in wireless rechargeable sensor networks using bi-level metaheuristic charging schemes"
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
pdf_path: "raw/canonical/Minimizing_the_energy_depletion_in_wireless_rechargeable_sensor_networks_using_bi-level_metaheur/Minimizing_the_energy_depletion_in_wireless_rechargeable_sensor_networks_using_bi-le.pdf"
raw_md: "raw/canonical/Minimizing_the_energy_depletion_in_wireless_rechargeable_sensor_networks_using_bi-level_metaheur/full.md"
---
# Minimizing the energy depletion in wireless rechargeable sensor networks using bi-level metaheuristic charging schemes

Huynh Thi Thanh Binh<sup>a,∗</sup>, Le Van Cuong<sup>a</sup>, Dang Hai Dang<sup>a</sup>, Le Trong Vinh<sup>b</sup>

<sup>a</sup>School of Information and Communication Technology, Hanoi University of Science and Technology, Vietnam

<sup>b</sup>University of Science, Vietnam National University, Hanoi

## Abstract

Recently, Wireless Rechargeable Sensor Networks (WRSNs) that leveraged the advantage of wireless energy transfer technology have opened a promising opportunity in solving the limited energy issue. However, an inefective charging strategy may reduce the charging performance. Although many practical charging algorithms have been introduced, these studies mainly focus on optimizing the charging path with a fully charging approach. This approach may lead to the death of a series of sensors due to their extended charging latency. This paper introduces a novel partial charging approach that follows a bi-level optimized scheme to minimize energy depletion in WRSNs. We aim at optimizing simultaneously two factors: the charging path and time. To accomplish this, we first formulate a mathematical model of the investigated problem. We then propose two approximate algorithms in which the optimization of the charging path and the charging time are considered as the upper and lower level, respectively. The first algorithm combines a Multi-start Local Search method and a Genetic Algorithm to find a solution. The second algorithm adopts a nested approach that utilizes the advantages of the Multitasking and Covariance Matrix Adaptation Evolutionary Strategies. Experimental validations on various network scenarios demonstrate that our proposed algorithms outperform the existing works.

Keywords: Wireless rechargeable sensor network, energy depletion, bi-level optimization, evolutionary strategy, multi-start local search, multitasking.

## 1. Introduction

A Wireless Sensor Network (WSN) consists of a collection of battery-powered sensor nodes deployed in a region of interest to monitor the physical environment and transfer the sensing information to the Base Station (BS) via multi-hop communication. [1, 2, 3]. Nowadays, WSNs play a nucleus role in the Internet of Things (IoTs) revolution due to their wide applications from civilian to military such as the smart metropolis, environment monitoring, intrusion detection, and battlefield surveillance [4, 5, 6]. However, limited energy issues remain as a major bottleneck phenomenon in WSNs. When a sensor’s battery is exhausted, the sensor becomes a dead node and loses its monitoring and communicating ability caus ing a series of negative impacts on the whole network performance [1, 7]. Therefore, one of the most critical conditions in continuously maintaining the network’s operation is to avoid the energy depletion of the sensor nodes. Energy-saving methods have been applied to prolong the sensor lifetime during the past decade [2, 8]. However, these approaches can only extend the network lifetime for a limited period. In recent years, a new approach that exploits the strength of wireless energy transfer technology has opened a promising solution for the energy issue in WSNs. Specifically, this technology allows energy transmitting from one or multiple Mobile Chargers (MCs) to the sensor nodes (equipped with a wireless energy receiver) without any wires or plugins. As a result, it has carried out a new network generation, named “Wireless Rechargeable Sensor Network (WRSN)”. In WRSN, the MC periodically travels around the network and charges sensors following either the on-demand charging strategy [9, 10, 11, 12, 13], or the periodic charging strategy [7, 14, 15]. Regarding the first strategy, the MC will travels and charges the requests receive from sensor node having its remaining energy below a predefined threshold. Consider the second strategy, the MC travels following a predetermined path to charge sensor nodes. In WRSNs, as the charging scheme plays a decisive role in prolonging the network lifetime, charging algorithm optimization thus has become a challenging problem.

Although many charging schemes have been proposed, most of the current works focus on optimizing the charging path of the MC (i.e., a sequence of the charging locations sorted by the visiting order of the MC). The studies [11, 16, 17, 18, 19, 20] aim at finding the optimal charging path with the fully charging method where the sensor’s battery is charged to maximum capacity. Moreover, they assumed that the MC energy is unlimited or suficient enough to charge all sensor nodes in each charging round. The full charging method may result in a long charging session of the MC if the number of sensors is large. It is only suitable for small networks with an insignificant number of sensor nodes. Besides, the charging time (i.e., the period that the MC stays and charges at each charging location) is also a primary factor in deciding the sensor’s lifetime. If the MC spends too much time at a charging location, the waiting time of other sensors will be increased; thus, uncharged sensors’ energy can be exhausted before being charged. On the other hand, if the MC allocates too little charging time for each sensor node, it will not have enough energy to operate until the next charging cycle.

Recently, the works in [21, 22, 23] attempted to solve the problem of optimizing charging time at each charging position to maximize the network lifetime. The authors [22] introduced a heuristic algorithm based on the sensor’s parameters to estimate the charging time interval at each location. The authors [23] addressed the problem of deciding the charging amount fo each sensor node to maximize the target covering time. The above-mentioned studies assume that the MC’s charging path is given and fixed in every charging round. The study [7] was the first to investigate the problem of minimizing the number of energy-depleted sensors by jointly optimizing both the charging path and the charging time. They decomposed the original problem into two sub-problems: charging path determining and charging time identifying, and then solved them separately. In addition, the previous periodic charging schemes assume that all sensors have the same energy consumption rate [24, 19, 15]. That is not realistic due to dynamic parameters in the network environment.

In this paper, we investigate a novel partial charging approach to minimize the number of dead sensor nodes under constraints in which the MC’s battery is limited, and sensors energy consumption rate is diverse. Specifically, we aim at optimizing both the charging path and charging time at each sensor node simultaneously. We named our investigated problem as Energy Depletion Minimization Problem (EDMP) in WRSNs. To solve the problem, we propose two approximate algorithms that leverage the advantage of the bilevel optimization approach, where determining the optimal charging path and charging time are considered the upper level and lower level optimization tasks, respectively. The first algorithm, named MLSGA based on a multi-restart mechanism to explore multiple feasible points in the enormous search space and escape the local optimum to find the global solution. A feasible solution of the charging scheme consists of a charging path and a charging time sequence correspondingly. Thus, at each iteration of MLSGA, an initial charging path is constructed by a greedy method, and the novel local search operators are adopted to improve the quality of the initial charging path at the upper level. Charging time optimization is performed at the lower level by leveraging a Genetic Algorithm. The second algorithm MTBCS based on a nested evolutionary strategy, where a hybrid of the Genetic Algorithm and the Local Search operators are adopted to speed up the charging path optimization at the upper level. Each feasible candidate of the upper level becomes an input for optimizing charging time at the lower level. However, instead of optimizing charging time for all charging paths at the upper level, we divided the charging paths into groups and only chose a representative candidate to identify an optimal charging time at the lower level. Multiple charging times linked to the chosen charging paths will be optimized simultaneously by the Multitasking-CMA-ES algorithm to significantly reduce the running time compared to the traditional nested algorithm.

The major contributions of this paper can be summarized as follows:

• Study the problem of minimizing energy depletion in WRSN based on a novel partial charging approach that optimizes both the charging path and charging time simultaneously.

• Provide network model, charging model, and a mathematical formulation of the investigated problem.

• Propose two approximate algorithms to solve the investigated problem based on a bi-level optimization approach. The first algorithm adopts a multi-restart mechanism to explore multiple feasible points in the enormous search space. An initial charging path is constructed by a greedy method and local search operator to enhance the quality of the charging path. The second algorithm leverages a nested evolutionary strategy, where multiple charging times linked to the chosen charging paths at the upper level will be optimized simultaneously by the Multitasking and Covariance Matrix

Adaptation Evolution Strategy.

• Perform statistic analysis and extensive experiments to compare the eficiency of the proposed algorithms to the most related works on various network scenarios.

The rest of the paper is organized as follows. In Section 2, we highlight related works. Section 3 presents the network model and problem formulation. Our proposed algorithms are described in Section 4 and Section 5. Statistic analysis and experimental results on benchmark datasets are discussed in Section 6. Finally, conclusions and future works are provided in Section 7.

## 2. Literature review

Since the limited energy constraint has shown to be a bottleneck phenomenon in traditional WSNs, many eforts have been devoted to solving the energy depletion avoidance problem. The literature can be classified into two primary methods to prolong the sensor lifetime, including energy-saving and energy replenishing [25]. The first method aims to minimize the sensor’s energy consumption using various techniques such as deploying relay nodes, relying on cross-layer design, or using mobile sinks [8]. Although these methods may extend the sensor lifetime for a period, sensor energy depletion is unavoidable. In recent times, Wireless Rechargeable Sensor Networks (WRSNs) that leverage the advantages of the wireless power transfer technology have emerged as a potential solution to the energy issue in traditional WSNs. In WRSNs, the sensor’s lifetime mainly depends on the charging scheme of the Mobile Charger (MC). Thus the most challenging problem in solving the WRSNs energy issue is to find an efective charging scheme for the MC to charge the sensor nodes. Most existing researches address this problem by focusing on two main approaches: charging path optimization and charging time optimization.

Regarding the first approach, the authors in [20, 24] studied the problem of maximizing the MC vacation time at the depot (i.e., the time the MC replenishes its battery). The authors proved that the optimal charging path of the MC is equivalent to the shortest Hamiltonian cycle. In the literature [20], Lyu et al. proposed an enhanced periodic charging scheme of [24] by taking into account the limited traveling energy of the MC and the imbalanced energy consumption among sensor nodes in the network. The authors addressed three possible network scenarios and applied a hybrid meta-heuristic algorithm based on Particle Swarm Optimization and Genetic Algorithm to determine the charging scheme for each scenario. Many researchers have worked on an on-demand charging strategy where the sensors send a charging request whenever their energy level drops below a threshold value. The MC charging decision will be made based on these requests. In [26] the authors constructed a proof-of-concept prototype of the system and attempted to solve the charging path planning problem by maximizing the network lifetime using a greedy algorithm depending on the lifetime of each sensor node. The studies of [10, 12] further complete the work in [26] by proposing a Starvation Avoidance Mobile Energy Replenishment scheme (SAMER) that adapt well to the high diversity of energy consumption but also fully consider the fairness of charging response [10] and an Invalid Node Minimized Algorithm (INMA) to optimize the waiting queue [12]. Aiming to find the optimal charging plan for the MC, Lin et al. in [27] studied the issues of both the fully charging strategy (the MC charges all the sensors to maximize their battery) and the partial charge plan (MC only charges a part of the sensor battery capacity). They proposed a Mixed Partial and Full charge plan (named MPF), including three specialized modules, i.e., the evaluation module, adjustment module, and selection module, to overcome the disadvantages of both charging plans. In order to enhance the charging utility, the authors in [14, 28] used a multi-nodes charging model that allows multiple sensors can be charged at the same time. Besides, They also studied how to find a collection of charging locations to maximize the MC charging power. These charging locations are determined by dividing the network charging field into smaller parts using a variety of techniques such as Smallest Enclosing Space [14] or the grid [28].

Some researchers have been working on the optimization of the charging time. Regarding this problem, Xu et al. in [21] proposed a novel idea to divide the charging energy and time into unit slots, and then the problem can be reduced to a matching problem between sensors and time slots. They then solve this problem by using the Maximum weighted matching in a bipartite graph to find the optimal charging schedule. Whereas the authors in [22] assumed that a charging path is predetermined and proposed a lightweight greedy algorithm to determine the optimal charging time at each charging location. Their ultimate goal is to maximize the network lifetime. The term “network lifetime” is defined by the interval from when the network starts until the first sensor node is dead (due to the exhausted energy). The authors in [23] handled the problem of deciding the amount of the charged energy for each sensor node to maximize the time of target monitoring.

An efective charging scheme should be considered both the charging path and charging time of the MC. The existing works, however, only deal with these factors separately. The authors of [7, 29] optimize both factors of the charging scheme by decomposing the problem into two sub-problems: finding the optimal charging path and determining the optimal charging time at each sensor. Then, each subproblem can be solved using an approximate approach based on the genetic algorithm [7], or a mixed-integer linear programming model [29]. This separation of the two phases could lead to the result of the first phase not being optimized for the final solution. In this paper, we propose the bi-level optimization approach to simultaneously optimize both the charging path and the charging time to minimize the number of the dead sensor nodes in the WRSNs.

In the past decade, Covariance Matrix Adaptation Evolutionary Strategy (CMA-ES) has been shown to be a state-of-the-art algorithm for continuous optimization problems with fast convergence speed. As CMA-ES belongs to Evolution Strategy (ES), the algorithm consists of three essential operators: recombination, mutation, and selection. Furthermore, it also possesses appealing characteristics such as derivative-free, covariant, of-the-shelf, scalable. It is advantageous on problems that are non-convex, non-separable, ill-conditioned, multi-modal, and noisy evaluations [30]. Because of these advantages, we decided to adopt the CMA-ES for the charging time optimization for one of our proposed algorithms, and details could be found in section 5.1

## 3. Network system setting and problem formulation

Table 1  
List of main notations

<table><tr><td>Notation</td><td>Description</td></tr><tr><td> $V = \{1, ..., n\}$ </td><td>a set of static sensor nodes</td></tr><tr><td>0</td><td>the base station</td></tr><tr><td> $e^{max}, e^{min}$ </td><td>energy capacity and minimum energy level of sensor battery</td></tr><tr><td> $p_i$ </td><td>average energy consumption rate of sensor i</td></tr><tr><td> $e_{i}^{init}, e_{i}^{depot}$ </td><td>initial energy and residual energy of node i at the beginning and at the finishing of the scheduling period, respectively</td></tr><tr><td> $e_i$ </td><td>residual energy of sensor i when MC arrives</td></tr><tr><td> $d_{i,j}$ </td><td>euclidean distance between two sensors i and j</td></tr><tr><td> $a_i$ </td><td>the time that the mobile charger arrives at sensor node i</td></tr><tr><td> $z_i$ </td><td>a binary variable represents the active status of sensor i</td></tr><tr><td></td><td>energy reduction of sensor i in a charging cycle (e.g. the energy level gain of i between two timings: beginning and finishing time of the scheduling period T)</td></tr><tr><td> $\Delta_i$ </td><td></td></tr><tr><td> $E_{MC}$ </td><td>maximum energy level of the MC in a charging cycle</td></tr><tr><td> $E_{move}, E_{charge}$ </td><td>total consumed energy of MC for traveling and charging processes.</td></tr><tr><td> $P_M, U$ </td><td>the per-second energy consumption rate of MC when traveling and when charging</td></tr><tr><td>v</td><td>the MC&#x27;s velocity</td></tr><tr><td>T</td><td>the scheduling period of a charging cycle</td></tr><tr><td> $T_{travel}, T_{charge}$ </td><td>the total time for traveling and charging, respectively.</td></tr><tr><td> $\wp$ </td><td>the discrete vector representing a charging path of MC</td></tr><tr><td> $\mathcal{T}$ </td><td>the continuous vector representing a time charging sequence at all charging locations</td></tr></table>

## 3.1. Network Setting

We consider a Wireless Rechargeable Sensor Network deployed over a region of interest. The network model can be represented as a weighted graph $\mathbf { G } = ( \mathbf { V } , \mathbf { E } , \mathbf { D } )$ , where ${ \bf V } =$ $\{ 0 , 1 , 2 , . . . , n \}$ includes a set of static sensors (nodes $1 , 2 , . . . , \mathrm { { n } ) }$ that are equipped with a wireless receiver as well as a base station (node 0) responsible for gathering data from the sensor nodes. E represents a set of edges between any two nodes that are in the range of their communication. The set $\mathbf { D } \subseteq V \times V$ represents the travel distances following Euclidean formulation between sensor nodes. Each sensor consumes energy for three essential tasks including sensing, receiving, and transmitting, determined by the network topology. In this system, we leverage the energy consumption model presented in [31].

• The energy consumption of a sensor for receiving l bits data with distance d is calculated as follows:

$$
e r = l \epsilon_ {e l e c} (\mathrm{J/b}),\tag{1}
$$

• The energy consumption of a sensor for transmitting l bits data with distance d is defined as follows:

$$
e t = \left\{ \begin{array}{l l} l \epsilon_ {e l e c} + l \epsilon_ {f c} \times d ^ {2} \mathrm{if} d <   d _ {0} \\ l \epsilon_ {e l e c} + l \epsilon_ {m p} \times d ^ {4} \mathrm{if} d \geq d _ {0}, \end{array} \right.\tag{2}
$$

where the $\epsilon _ { e l e c }$ is the electronics energy expenditures per bit data to run the transmitter or receiver. $\epsilon _ { f c }$ and $\epsilon _ { m c }$ are the amplifier energy of transmitting one bit of data in free space $( d ^ { 2 } )$ and multi-path fading $( d ^ { 4 } )$ model, respectively. $d _ { 0 }$ represents the threshold distance between the receiver and transmitter, respectively. The parameters in the network model are set as: $\epsilon _ { e l e c } = 0 . 0 5 J / b i t , \epsilon _ { f c } = 0 . 0 1 J / b i t / m ^ { 2 } , \epsilon _ { m p } = 0 . 0 1 3 p J / b i t / m ^ { 4 } , d _ { 0 } = \sqrt { \frac { \epsilon _ { f c } } { \epsilon _ { m p } } }$

The sensors periodically send the data packets containing the sensory data, their resid ual energy, and the time stamp to the BS through the multi-hop transfer protocol. From the obtained information, the BS can estimate the average energy consumption $e _ { i }$ and the remaining energy $( e _ { i } ^ { i n i t } )$ of sensor i by using methods in [12]. Each sensor has the battery capacity $e ^ { m a x }$ and minimum operating energy threshold $e ^ { m i n }$ . When the energy of sensor is less than $e ^ { m i n }$ , it become a dead sensor and can not perform its tasks. Therefore, to avoid the energy depletion of sensors, a Mobile Charger (MC) is adopted. In such context, the MC departs at the BS (also plays a role as the depot) and periodically travels to the location of sensor nodes in the network to replenish their battery, in which for each sensor node $i ,$ the MC will spend an interval $t _ { i }$ to charge it. After finishing the charging process for all sensor nodes, the MC returns to the BS to fully recharge its battery and prepare for the next charging round. The network system is illustrated in Figure 1.

## 3.2. The problem formulation

Without loss of generality, a charging cycle period is set to $T ,$ and the MC replenished itself to its maximum battery capacity $E _ { M C }$ for beginning the charging round. In this work, we study how to determine the optimal charging path and time to minimize energy depletion of sensors in WRSNs. We name the investigated problem as Energy Depletion Minimization Problem (EDMP). Formally, a feasible solution to the EDMP consists of two factors: a charging path $\wp = \left( \pi _ { 0 } , \pi _ { 1 } , \pi _ { 2 } , . . . , \pi _ { n } , \pi _ { n + 1 } \right)$ and a charging time sequence $\mathcal { T } =$ $\left( \tau _ { \pi _ { 1 } } , \tau _ { \pi _ { 2 } } , \ldots , \tau _ { \pi _ { n } } \right)$ ; where $\{ \pi _ { 1 } , \pi _ { 2 } , . . . , \pi _ { n } \}$ is a permutation of the set $\{ 1 , 2 , . . . , n \}$ and $\pi _ { 0 } \equiv$ $\pi _ { n + 1 } \equiv 0 . { \tau } _ { \pi _ { i } }$ indicates the charging time at sensor $\pi _ { i }$ when the MC travels following the path ℘. To easy in the solution representation, we denote a charging path $\pi _ { o }  \pi _ { 1 }  \pi _ { 2 } $ $\pi _ { 3 } . . . \to \pi _ { n } \to \pi _ { n + 1 }$ as a vector $\left( \pi _ { 1 } , \pi _ { 2 } , \pi _ { 3 } , \pi _ { n } \right)$

Since the MC always expenses energy during the functioning process, MC’s total energy for traveling and charging processes must not exceed its maximum battery capacity. We have:

$$
T _ {t r a v e l} \times P _ {M} + \sum_ {i = 1} ^ {n} \tau_ {\pi_ {i}} \times U \leq E _ {M C}\tag{3}
$$

where, $P _ { M }$ and $U$ represent the per-second energy consumption rates of MC when traveling and charging, respectively. $T _ { t r a v e l } = \frac { \sum _ { j = 0 } ^ { n } d _ { \pi _ { j } \pi _ { j + 1 } } } { v }$ is the total traveling time of MC with velocity v.

![](images/cef9a039b5ec69a847faec58a5fdd347fa923021ad44ac4953b1bc30597a6940.jpg)  
Figure 1: A wireless rechargeable sensor network system.

In addition, the total traveling time and charging time does not also exceed the scheduling period T :

$$
T _ {t r a v e l} + T _ {c h a r g e} \leq T\tag{4}
$$

where, $\begin{array} { r } { T _ { c h a r g e } = \sum _ { i = 1 } ^ { n } \tau _ { \pi } } \end{array}$ is the total charging time that the MC spends to charge all sensors in one charging cycle.

During the cycle period T , we have observed that the energy fluctuation of $\pi _ { i }$ may be divided into three major phases: the pre-charging phase, the in-charging, and the after charging phase. For the first phase, since MC travels and charges the sensors $\left\{ \pi _ { 1 } , \pi _ { 2 } , \ldots , \pi _ { i - 1 } \right\}$ $\pi _ { i }$ has to wait a period \`o time. As a result, the energy of $\pi _ { i }$ decreases gradually. For the second phase, the energy of $\pi _ { i }$ is increased by receiving energy from the MC. Finally, in the third phase, when MC leaves $\pi _ { i }$ and travels for the other sensors, the energy of $\pi _ { i }$ again declines for performing its tasks. Fig. 2 illustrates the energy fluctuation of sensor $\pi _ { i }$ during the charging cycle.

Let $e _ { \pi _ { i } }$ and $e _ { \pi _ { i } } ^ { d e p o t }$ are the residual energy of sensor $\pi _ { i }$ at two timings: the time that MC arrives at sensor π<sub>i</sub> and the the end of period T , respectively. Accordingly, $e _ { \pi _ { i } }$ and $e _ { \pi _ { i } } ^ { d e p o t }$ are

![](images/ed3d7ba27691844c1db99f561e2e465b76fdfbb4af3a40400f6fa00a52cad70d.jpg)  
Figure 2: Energy fluctuation of a sensor in a charging cycle $T ,$ , where $t _ { 0 }$ is the timing that $M C$ starts at the depot. Energy of the sensor decreases in the first phase $( t _ { 0 }$ to $t _ { 1 } )$ and the third phase (t<sub>2</sub> to T ). The second phase $( t _ { 1 }$ to $t _ { 2 } )$ is the timing that the sensor’s battery is replenished.

calculated as follows:

$$
e _ {\pi_ {i}} = e _ {\pi_ {i}} ^ {i n i t} - \left(\sum_ {j = 0} ^ {i - 1} \frac {d _ {\pi_ {j} \pi_ {j + 1}}}{v} + \sum_ {j = 0} ^ {i - 1} t _ {\pi_ {j}}\right) \times p _ {\pi_ {i}}\tag{5}
$$

and

$$
e _ {\pi_ {i}} ^ {d e p o t} = e _ {\pi_ {i}} ^ {i n i t} - T \times p _ {\pi_ {i}}\tag{6}
$$

The first term $e _ { \pi _ { i } } ^ { i n i t }$ in formula (5) and (6) is the initial energy of sensor $\pi _ { i }$ at each charging cycle, while the second term in both formulas above indicates the energy that the sensor $\pi _ { i }$ expenses from when MC starts the charging cycle until it arrives at the location of $\pi _ { i }$ and when finishing the charging cycle at BS, respectively. Let $z _ { \pi _ { i } }$ be a binary variable which represents the status of sensor $\pi _ { i } .$ , where $z _ { \pi _ { i } } = 1$ if $\pi _ { i }$ dies in charging cycle, otherwise $z _ { \pi _ { i } } =$ 0. In Fig. 2, the remaining energy of sensor $\pi _ { i }$ attains the lowest value in a charging cycle at two timings, either when MC arrives at $\pi _ { i }$ or the end of the charging cycle. Thus, to determine whether sensor $\pi _ { i }$ dies in the charging cycle or not, we only consider the remaining energy of $\pi _ { i }$ at the above two timings. It means that:

$$
z _ {\pi_ {i}} = \left\{ \begin{array}{l} 1 \text {if} e _ {\pi_ {i}} <   e ^ {m i n} \text {or} e _ {\pi_ {i}} ^ {d e p o t} <   e ^ {m i n} \\ 0 \text {otherwise}, \end{array} \right.\tag{7}
$$

Intuitively, to avoid energy depletion, sensors should be recharged before they drain energy. Thus, we need to determine the optimal charging path sequence through all sensors and the corresponding charging time that MC spends at each sensor to minimize the number of sensor nodes depleted their energy (named dead sensor nodes). Moreover, our objective focuses on minimizing the number of dead sensor nodes at the current charging round and the next charging round. Thus, the problem objective is defined as follows:

$$
f = \alpha \frac {\sum_ {i = 1} ^ {n} z _ {\pi_ {i}}}{n} + (1 - \alpha) \frac {m a x _ {\pi_ {i} \in V} \{\Delta_ {\pi_ {i}} \}}{e ^ {m a x} - e ^ {m i n}} \rightarrow m i n,\tag{8}
$$

where, α is a control parameter and $\Delta _ { \pi _ { i } }$ is the energy diference of the sensor $\pi _ { i }$ between two timings when MC starts and finishes the charging cycle. The value of $\Delta _ { \pi _ { i } }$ is defined by:

$$
\Delta_ {\pi_ {i}} = \left\{ \begin{array}{l l} e _ {\pi_ {i}} ^ {i n i t} - e _ {\pi_ {i}} ^ {d e p o t}, & \text { if } z _ {\pi_ {i}} = 0 \text { and } e _ {\pi_ {i}} ^ {i n i t} > e _ {\pi_ {i}} ^ {d e p o t} \\ 0, & \text { otherwise. } \end{array} \right.\tag{9}
$$

The first component in the objective function 8 depicts the ratio of the dead sensor (due to energy depletion) over the total network number of sensors and favors the minimum when the number of the dead sensors at the current charging cycle is the smallest. While the second component tends to maximize the sensor’s remaining energy after each charging cycle toward minimizing the dead nodes in the next charging cycle. Table 1 describes the notations used in this paper.

The optimization problem EDMP can be represented as follows.

$$
M i n _ {\wp , t} \left(\alpha \frac {\sum_ {i = 1} ^ {n} z _ {\pi_ {i}}}{n} + (1 - \alpha) \frac {m a x _ {\pi_ {i} \in V} \{\Delta_ {\pi_ {i}} \}}{e ^ {m a x} - e ^ {m i n}}\right)\tag{10}
$$

s.t. Constraints (3), (4),

$$
\begin{array}{l l} e _ {\pi_ {i}} ^ {i n i t} + \tau_ {\pi_ {i}} \times (U - p _ {\pi_ {i}}) \leq e ^ {m a x} & \forall \pi_ {i} \in V \\ \tau_ {\pi_ {i}} \geq 0 & \forall \pi_ {i} \in V \end{array}\tag{11}
$$

(12)

Constraint (11) indicates that the total charged energy of each sensor does not exceed the sensor’s maximum battery capacity. Constraint (12) expresses the charging time at each sensor node must be not negative.

The EDMP is considered as a Bi-level Optimization Problem (BLOP) because EDMP consists of two optimization levels, including the charging path level and the charging time level, referred to as upper-level and lower-level optimization tasks, respectively. Moreover, the charging time optimization task is nested as a constraint of the charging path optimization task. Thus, EDMP is known as the NP-hard problem [32]. It is dificult to find an exact solution in large-scale scenarios. Therefore, we adopt approximate methods based on bi-level optimization approach to deal with the investigated problem.

We have an essential observation that the total charging time of the MC significantly afects the performance of a charging scheme. If the MC spend too much time for the charging process, the period of one charging cycle may substantially increase lead to the energy depletion of critical sensors before they are recharged in the next charging round. In addition, the energy of MC maybe not be enough for the MC to go back to the depot. However, too little charging time by the MC may lead to the sensors energy level too low and not enough to survive until being charge in the following cycle. So, before representing the proposed algorithm, in next section, we will determine the upper bound of the total charging time in one charging round.

## 3.3. Total charging time determination

We first construct the upper bound of the total charging time as follows. Suppose that a scheme with a charging tour $\wp = \pi _ { 0 } \to \pi _ { 1 } \to \pi _ { 2 } \dots \to \pi _ { n } \to \pi _ { n + 1 }$ is performed by MC in the charging round $k ^ { t h }$ . Assuming that all the sensors are still active after the charging round. That assumption can be formulated as:

$$
\sum_ {i = 1} ^ {n} e _ {\pi_ {i}} ^ {i n i t} + \sum_ {i = 1} ^ {n} \tau_ {\pi_ {i}} \times U - n \times e ^ {m i n} \geq (T _ {c h a r g e} + T _ {t r a v e l}) \times \sum_ {i = 1} ^ {n} p _ {\pi_ {i}}\tag{13}
$$

That means the sum of the initial energy of the network and the extra energy that the network receives from MC must be greater or equal to the total energy consumption by all the sensors. Otherwise, the sensors battery can be drained at the end of the charging round. Replace $T _ { c h a r g e }$ into the upper equation, the constraint (13) can be rewritten as follows:

$$
\sum_ {i = 1} ^ {n} e _ {\pi_ {i}} ^ {i n i t} - n \times e ^ {m i n} - T _ {t r a v e l} \times \sum_ {i = 1} ^ {n} p _ {\pi_ {i}} \geq T _ {c h a r g e} \times \left(\sum_ {i = 1} ^ {n} p _ {\pi_ {i}} - U\right)\tag{14}
$$

From (14), if $\textstyle U < \sum _ { i = 1 } ^ { n } p _ { \pi }$ we have:

$$
T _ {c h a r g e} \leq \frac {\sum_ {i = 1} ^ {n} e _ {\pi_ {i}} ^ {i n i t} - n \times e ^ {m i n} - T _ {t r a v e l} \times \sum_ {i = 1} ^ {n} p _ {\pi_ {i}}}{\sum_ {i = 1} ^ {n} p _ {\pi_ {i}} - U}\tag{15}
$$

Besides, because the energy of a sensor can not exceed its battery capacity, so we have

$$
\sum_ {i = 1} ^ {n} e _ {\pi_ {i}} ^ {i n i t} + \sum_ {i = 1} ^ {n} \tau_ {\pi_ {i}} \times U - (T _ {c h a r g e} + T _ {t r a v e l}) \times \sum_ {i = 1} ^ {n} p _ {\pi_ {i}} \leq n \times e ^ {m a x}\tag{16}
$$

Replace $\textstyle \sum _ { i = 1 } ^ { n } \tau _ { \pi _ { i } }$ by $T _ { c h a r g e }$ and rewrite the constraint (16), we have

$$
T _ {c h a r g e} \times (U - \sum_ {i = 1} ^ {n} p _ {\pi_ {i}}) \leq n \times e ^ {m a x} - \sum_ {i = 1} ^ {n} e _ {\pi_ {i}} ^ {i n i t} + T _ {t r a v e l} \times \sum_ {i = 1} ^ {n} p _ {\pi_ {i}}\tag{17}
$$

From constraint (17), if $\begin{array} { r } { U > \sum _ { i = 1 } ^ { n } p _ { \pi _ { i } } } \end{array}$ we have:

$$
T _ {c h a r g e} \leq \frac {n \times e ^ {m a x} - \sum_ {i = 1} ^ {n} e _ {\pi_ {i}} ^ {i n i t} + T _ {t r a v e l} \times \sum_ {i = 1} ^ {n} p _ {\pi_ {i}}}{U - \sum_ {i = 1} ^ {n} p _ {\pi_ {i}}},\tag{18}
$$

In the case $\begin{array} { r } { U = \sum _ { i = 1 } ^ { n } p _ { \pi _ { i } } } \end{array}$ , i.e. the energy consumed by the whole network is totally compensated by the charging mobile MC, the equations (15) and (18) are obviously satisfied. So, the upper bound of the charging time just needs to follow the constraint (3), which is rewritten as

$$
T _ {c h a r g e} \leq \frac {E _ {M C} - T _ {t r a v e l} \times P _ {M}}{U}\tag{19}
$$

From (3), (15), (18), and (19), we can calculate the upper bound of total charging time for the charging plan as follows:

$$
T _ {c h a r g e} \leq \left\{ \begin{array}{l l} m i n \left(\frac {E _ {M C} - T _ {t r a v e l} \times P _ {M}}{U}, \frac {\sum_ {i = 1} ^ {n} e _ {\pi_ {i}} ^ {i n i t} - n \times e ^ {m i n} - T _ {t r a v e l} \times \sum_ {i = 1} ^ {n} p _ {\pi_ {i}}}{\sum_ {i = 1} ^ {n} p _ {\pi_ {i}} - U}\right), & \text {if} U <   \sum_ {i = 1} ^ {n} p _ {\pi_ {i}} \\ m i n \left(\frac {E _ {M C} - T _ {t r a v e l} \times P _ {M}}{U}, \frac {n \times e ^ {m a x} - \sum_ {i = 1} ^ {n} e _ {\pi_ {i}} ^ {i n i t} + T _ {t r a v e l} \times \sum_ {i = 1} ^ {n} p _ {\pi_ {i}}}{U - \sum_ {i = 1} ^ {n} p _ {\pi_ {i}}}\right), & \text {if} U > \sum_ {i = 1} ^ {n} p _ {\pi_ {i}} \\ \frac {E _ {M C} - T _ {t r a v e l} \times P _ {M}}{U}, & \text {if} U = \sum_ {i = 1} ^ {n} p _ {\pi_ {i}} \end{array} \right.\tag{20}
$$

It is clear that to prolong sensors’ lifetime; they should be charged as much as possible. In other words, the energy that MC uses to charge for the network should be as much as possible. Based on the above observation and the equation (20), in this paper, we determine the total charging time for a given charging tour $\pi _ { 0 } \to \pi _ { 1 } \to \pi _ { 2 } \dots \to \pi _ { n } \to \pi _ { n + 1 }$ as follows.

$$
T _ {c h a r g e} = \left\{ \begin{array}{l l} m i n \left\{\frac {E _ {M C} - T _ {t r a v e l} \times P _ {M}}{U}, \frac {\sum_ {i = 1} ^ {n} e _ {\pi_ {i}} ^ {i n i t} - n \times e ^ {m i n} - T _ {t r a v e l} \times \sum_ {i = 1} ^ {n} p _ {\pi_ {i}}}{\sum_ {i = 1} ^ {n} p _ {\pi_ {i}} - U} \right\}, & \mathrm{if} U <   \sum_ {i = 1} ^ {n} p _ {\pi_ {i}} \\ m i n \left\{\frac {E _ {M C} - T _ {t r a v e l} \times P _ {M}}{U}, \frac {n \times e ^ {m a x} - \sum_ {i = 1} ^ {n} e _ {\pi_ {i}} ^ {i n i t} + T _ {t r a v e l} \times \sum_ {i = 1} ^ {n} p _ {\pi_ {i}}}{U - \sum_ {i = 1} ^ {n} p _ {\pi_ {i}}} \right\}, & \mathrm{if} U > \sum_ {i = 1} ^ {n} p _ {\pi_ {i}} \\ \frac {E _ {M C} - T _ {t r a v e l} \times P _ {M}}{U}, & \mathrm{if} U = \sum_ {i = 1} ^ {n} p _ {\pi_ {i}} \end{array} \right.\tag{21}
$$

## 4. Hybrid of multi-start local search and genetic algorithm

## 4.1. Motivation of proposed algorithm

The work in [7] proposes a two-phase algorithm in which the former phase tries to find an optimal charging path and the latter phase focuses on optimizing the charging time while assuming that MC follows the path obtained by the first phase. However, the result of the first phase may not be an optimal path, and thus the second phase may be misled into an incorrect assumption. From that observation, in this section, we propose an algorithm for constructing the charging schedule, in which several (local optimal) paths are considered to optimize the charging time.

Due to the ability to produce multiple locally optimal solutions by running a local search procedure from multiple starting points, the multi-start local search algorithm is chosen to optimize the charging path. In addition, the genetic algorithm, with exemplary performance in global search, is adopted for determining the charging time at each sensor. A novel local search operator is also proposed in this section to improve the quality of charging paths. Instead of determining the charging time for only one path as in [7], in each iteration of the proposed algorithm, we attempt to find an optimal path, and then the corresponding charging time for that path is determined. To simplify the presentation, the proposed algorithm in this section is named Hybrid of Multi-start Local Search and Genetic Algorithm (MLSGA).

## 4.2. MLSGA algorithm scheme

The outlines of our approach are presented in Algorithm 1. Each iteration of the algorithm consists of two steps. The first step aims to construct an optimal path for MC. The charging time for the path obtained after the first step is then determined in the second step by using a genetic algorithm (line 7). After two steps, the new charging schedule $( \wp , \mathcal { T } )$ will be compared with the best overall solution obtained so far to see if a new best is found. If this is the case, we successfully replace the best overall solution with the improved solution (lines 8-10). In the end, the best overall solution is returned as the final result.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 1: Hybrid Multi-start Local Search and Genetic Algorithm

Input: A wireless rechargeable sensor network consists of n sensors  $\{1,2,\ldots,n\}$  and a mobile charger.

Output: A charging path  $\wp^{*} = (\pi_{1}^{*}, \pi_{2}^{*} \ldots \pi_{n}^{*})$  and a charging time vector  $\mathcal{T}^{*} = (\tau_{\pi_{1}^{*}}, \tau_{\pi_{2}^{*}}, \ldots, \tau_{\pi_{n}^{*}})$  correspondingly.

1 begin

2  $\wp^{*} \leftarrow null;$ 

3  $T^{*} \leftarrow null;$ 

4 while terminate conditions are not satisfied do

5  $\wp_{0} \leftarrow construct an initial charging path;$  // see Subsect. 4.5

6  $\wp \leftarrow local search on \wp_{0} until a local optima is reached;$  // using evaluation method in 4.3 and local search operator in 4.4

7 Determine the optimal charging time vector T corresponding to the charging path  $\wp$  using Genetic Algorithm; // see in Subsect. 4.6

8 if  $f(\wp^{*}, T^{*}) &gt; f(\wp, T)$  then

9  $\wp^{*} \leftarrow \wp;$ 

10  $T^{*} \leftarrow T;$ 

11 return  $\{\wp^{*}, T^{*}\};$
</div>

Before presenting two steps of each iteration, we first introduce a greedy method for charging path evaluation and a local search operator for improving a given path.

## 4.3. Charging path evaluation method

To precisely evaluate a given charging path, the corresponding optimal charging time must be determined. Unfortunately, finding the optimal charging time is not straightforward itself. Thus, to evaluate all generated paths during the search process, we propose an evaluation method by greedy determining the charging time.

For a given charging path $\wp \equiv \pi _ { 0 }  \pi _ { 1 }  \pi _ { 2 } . . .  \pi _ { n }  \pi _ { 0 }$ , intuitively, the charging time at a sensor depends on its energy status and energy consumption rate, which means sensors with less residual energy and higher consumption rate should be charged more than the others. Based on that observation, the charging time is greedily determined as follows. First, we define for each sensor $\pi _ { i }$ a weight factor as

$$
w _ {\pi_ {i}} = m a x \left(0, \frac {p _ {\pi_ {i}}}{\sum_ {i = 1} ^ {n} p _ {\pi_ {i}}} - \frac {e _ {\pi_ {i}} ^ {i n i t}}{\sum_ {i = 1} ^ {n} e _ {\pi_ {i}} ^ {i n i t}}\right).\tag{22}
$$

After determining the weights of all sensors, the charging time at sensor $\pi _ { i }$ is assigned as follows

$$
\tau_ {\pi_ {i}} = T _ {c h a r g e} \times \frac {w _ {\pi_ {i}}}{\sum_ {i = 1} ^ {n} w _ {\pi_ {i}}},\tag{23}
$$

where $T _ { c h a r g e }$ is the total of charging time of MC in the charging round that is calculated by 21. The charging path $\wp$ is then evaluated with the above charging time under the objective function (10).

## 4.4. Local search operator for charging path improvement

In this subsection, we propose local search operators to improve the quality of a charging path. Given a schedule that consists of a path $\wp = \left( \pi _ { 1 } , \pi _ { 2 } \ldots . . . , \pi _ { n } \right)$ and charging time at each sensor $\mathcal { T } = ( \tau _ { \pi _ { 1 } } , \tau _ { \pi _ { 2 } } , \ldots , \tau _ { \pi _ { n } } )$ , following 3.2, there are two reasons that cause the dead of a sensor $\pi _ { i }$ as follows:

$\pi _ { i }$ is dead before being charged $( x _ { \pi _ { i } } = 1$ because $e _ { \pi _ { i } } < e _ { m i n } )$

$\pi _ { i }$ is dead after being charged, i.e., the total initial energy of $\pi _ { i }$ and the energy received from MC is not enough to keep it active to the end of the charging round $( y _ { \pi _ { i } } = 1$ cause of $e _ { \pi _ { i } } ^ { d e p o t } < e _ { m i n } )$

Let define $D _ { B }$ and $D _ { A }$ as two collections of any dead sensor caused by the first and second reasons. The basic idea of our local search operator is as follows.

If a sensor $\pi _ { i } ~ \in ~ D _ { B } , \ \pi _ { i }$ is charged too late. So, we move it forward to the front of the charging sequence. By doing that, the charged waiting time of $\pi _ { i }$ will be reduced. Conversely, if a sensor $\pi _ { i } \in D _ { A }$ , the charging time that the MC spends at the location of $\pi _ { i }$ should be increased. From Equations (5) and (11), the charging time at sensor $\pi _ { i }$ has an upper bound value as follows:

$$
\tau_ {\pi_ {i}} ^ {m a x} = \frac {e ^ {m a x} - e _ {\pi_ {i}} ^ {i n i t} + \left(\sum_ {j = 0} ^ {i - 1} \frac {d _ {\pi_ {j} , \pi_ {j + 1}}}{v} + \sum_ {j = 1} ^ {i - 1} \tau_ {\pi_ {j}}\right) \times p _ {\pi_ {i}}}{U - p _ {\pi_ {i}}}.\tag{24}
$$

From (24), to increase $\tau _ { \pi _ { i } } ^ { m a x }$ , we need to increase the term $\begin{array} { r } { \sum _ { j = 0 } ^ { i - 1 } \frac { d _ { \pi _ { j } , \pi _ { j + 1 } } } { v } + \sum _ { j = 1 } ^ { i - 1 } \tau _ { \pi _ { j } } } \end{array}$ , i.e., the sensor $\pi _ { i }$ should be charged later. Thus, we move $\pi _ { i }$ to a backward position of the charging queue ℘. Algorithm 2 represents the pseudo-code of the proposed local search operator.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 2: Local search operator

Input: A charging path $P = (\pi_1, \pi_2 \ldots, \pi_n)$ and charging time at each sensor
$\mathcal{T} = (\tau_{\pi_1}, \tau_{\pi_2}, \ldots, \tau_{\pi_n})$

Output: A neighbour charging path $\wp'$ of the charging path $\wp$.

1 begin
2    $NS1 \longleftarrow |D_B|$;
3    $NS2 \longleftarrow |D_A|$;
4    if $NS1 + NS2 = 0$ then
5    $P' \longleftarrow$ random move a sensor in $\wp$;
6    else if $D_A$ is empty or rand(0,1) $\leq \frac{NS1}{NS1+NS2}$ then
7    Choose a random sensor $\pi_i \in D_B$;
8    $j \longleftarrow argmax_{j \in [1,i-1]} \{a_{\pi_j} : a_{\pi_j} \leq l_{\pi_i}\}$;
9    $P' \longleftarrow$ move $\pi_i$ to a random position in range [1,j];
10    else
11    Choose a random sensor $\pi_i \in D_A$;
12    $j \longleftarrow argmax_{j \in [i+1,n]} \{a_{\pi_j} : a_{\pi_j} \leq l_{\pi_i}\}$;
13    $P' \longleftarrow$ move $\pi_i$ to a random position in range [i+1,j];
14    return $\wp'$;
</div>

where $l _ { \pi _ { i } }$ is the lifetime of sensor $\pi _ { i }$ without the replenishing energy,

$$
l _ {\pi_ {i}} = \frac {e _ {\pi_ {i}} ^ {i n i t} - e _ {m i n}}{p _ {\pi_ {i}}},\tag{25}
$$

and $a _ { \pi _ { j } }$ is the time that MC arrives at $\pi _ { j }$ and starts charging,

$$
a _ {\pi_ {j}} = \sum_ {j = 0} ^ {i - 1} \frac {d _ {\pi_ {j} , \pi_ {j + 1}}}{v} + \sum_ {j = 1} ^ {i - 1} \tau_ {\pi_ {j}}.\tag{26}
$$

In the above algorithm, if a sensor $\pi _ { i }$ is dead, we move it to a new position based on its lifetime $l _ { \pi _ { i } }$ (lines 9, 13) to ensure that it will not die after the move. Two well-known operators, namely, Two-exchange and Relocated, are adopted with the same probability to move $\pi _ { i }$ to another position.

## 4.5. Charging path construction

The charging path of the MC is designed based on the observation: a charging schedule needs to have a low traveling cost to maximize the charging energy for sensor nodes. Considering the energy aspect, since the the MC energy is limited, minimizing the traveling energy to increase the received energy by the sensor nodes. Regarding the time aspect, the more optimized the charging tour be, the less time the sensors have to wait. Therefore, a greedy approach is used at each iteration of the proposed algorithm to generate the charging tours with a low moving cost as a good starting point.

Specifically, we use the well-known k-nearest neighbor algorithm [33] to generate an initial charging path. The method starts with a partial solution that consists of the base station $\pi _ { 0 } ,$ and a list of L is composed of k unvisited nearest sensors evaluated by moving distance. Each subsequent step chooses a random sensor from the list L to add to the partial solution. This technique generates these diferent charging paths for each iteration. For the diversity of the starting points, we randomly select the value of k in the range [2, n] for each charging path, where n is the number of sensors. If k is small, the method generates an initial charging path with a low traveling cost. In contrast, If k is close to $n ,$ the method generates an individual with a higher degree of randomness.

The local search operator proposed in Subsect. 4.4 is then performed on the initial path until the local optima are reached.

## 4.6. Charging time optimization based on genetic algorithm

Suppose that $\wp \equiv \left( \pi _ { 1 } , \pi _ { 2 } \ldots . . . , \pi _ { n } \right)$ is the charging path obtained after the first step of an iteration. This subsection represents how we determine the charging time at each sensor $\pi _ { i }$ by using the genetic algorithm.

## 4.6.1. Individual representation

The pattern chromosome in this algorithm is an vector of n element $X = \{ \rho _ { \pi _ { 1 } } , \rho _ { \pi _ { 2 } } , . . . , \rho _ { \pi _ { n } } \}$ Each element $\rho _ { \pi _ { i } }$ is real number in range [0, 1] which represents the ratio of charging time at $\pi _ { i }$ to the total charging time of MC in the charging round. The charging time at $\pi _ { i }$ is then decoded from $\rho _ { \pi _ { i } }$ as follows.

$$
\tau_ {\pi_ {i}} = T _ {c h a r g e} \times \frac {\rho_ {\pi_ {i}}}{\sum_ {i = 1} ^ {n} \rho_ {\pi_ {i}}},\tag{27}
$$

where $T _ { c h a r g e }$ is the total charging time that is calculated by Eq. 21.

## 4.6.2. Genetic operators

The initial population consists of N individuals. For each individual X, the values of all the genes are assigned by a random number following the uniform distribution in the interval [0, 1]. Two well-known operators, namely Simulated Binary Crossover and Polynomial Mutation [34] are deployed to generate the ofspring in each generation. Then, N fittest individuals are selected from the current population and the ofspring to form the next generation population.

## 5. Multitasking approach-based bi-level charging scheme optimization

## 5.1. Motivation of proposed algorithm

The MLSGA algorithm has overcome limitations of the separated two-phase approach in the work [7] by utilizing the bi-level optimization approach. However, the obtained solution at each iteration of the MLSGA algorithm depends highly on the charging path construction. That means an inefective path may lead to a sub-optimal result. Therefore, instead of establishing only one good path at the upper level in each iteration to optimize the charging time for the lower level, multiple diverse paths will be formulated at the upper level to explore the search space. The charging paths will be optimized through the evolution process and linked to a lower level to optimize the charging time. As a result, the evolution process is carried out at both levels simultaneously, and the final optimal solution is obtained. Our second proposed algorithm is constructed based on this basic idea.

In this section, we represent our proposal named MTBCS that adopts the nested evolutionary strategy to assure the diversity of the lower-level search space while retaining a feasible searching time. Specifically, a hybrid of the Genetic Algorithm (GA) and the Lo cal Search (LS) operators is adopted to speed up the charging path optimization at the upper level. Each feasible candidate of the upper level becomes an input for optimizing the charging time at the lower level. However, instead of optimizing the charging time for all charging paths at the upper level, we divided the charging paths into groups and only chose a representative charging path for each group to identify an optimal charging time at the lower level. Multiple charging times associated with the chosen charging paths will be optimized simultaneously by the Multitasking Covariance Matrix Adaptation evolution strategy (M-CMAES) based on an explicit knowledge transfer mechanism.

The reason is as follows:

• Genetic Algorithm is strongly capable of identifying promising regions of the search space (exploration), but it often fails or takes a long time to refine the optimal local solution (exploitation), which can be eficiently achieved by a local search method. Therefore, incorporating Local Search into GA may help to obtain a better solution for the upper level.

• The individuals of the upper level (i.e., the input of the lower level) more or less have certain similarities in terms of the genotypes, while multitasking via knowledge transfer among optimization tasks, has been shown in such researches [35, 36] to be able to enhance the search performance. Moreover, CMA-ES is recognized as a state-ofthe-art stochastic algorithm to address black-box continuous optimization problems. Unlike GA, where the individual’s evolution is achieved explicitly using predefined crossover and mutation methods, CMA-ES individuals are self-adapting via the Co variance Matrix update and population step size. Because of that, CMA-ES evolution is accomplished implicitly, which has shown to be a more eficient way to improve the quality of the result with faster convergence speed compared to GA in more complex problems like ours since the fixed parameters of GA won’t be able to adapt to diferent changes happen in the evolution process. Thus, a synergy of the multitasking approach and CMA-ES algorithm could establish a robust optimizer for the lower level.

The second proposed algorithm is named Multitasking approach-based Bi-level Charging Scheme (MTBCS).

## 5.2. MTBCS algorithm scheme

Algorithm 3 represents the pseudo-code of our proposed algorithm MTBCS. After generating and evaluating the initial population (lines 2-3), each iteration of the main loop includes two stages corresponding to two levels of the problem. In the first stages (lines 6-15), the genetic operators and the local search procedure are performed to optimize the charging path. The greedy method proposed in Subsect. 4.3 is reused in this stage to evaluate all the generated paths. After that, in the second stage, several paths are selected from the upper level population and the proposed M-CMAES is invoked (lines 16-17) for optimizing the charging time.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 3: Multitasking approach-based bi-level charging scheme

Input: A list of sensors $s_1, s_2, \ldots, s_n$, a mobile charger MC.

Output: A charging schedule consists of a charging path $\wp^* = (\pi_1^*, \pi_2^* \ldots \pi_n^*)$ and the charging time at each sensor $\mathcal{T}^* = (\tau_{\pi_1^*}, \tau_{\pi_2^*}, \ldots, \tau_{\pi_n^*})$.

begin

2 Generate N initial charging paths to form the population $\wp_0$;

3 Evaluate all the charging path in $\wp_0$; // using method in Subsect. 4.3

4 t ← 0;

5 while terminate conditions are not satisfied do

6 Offspring C ← ∅;

7 while size of C &lt; N do

8 Select two parents $p_1$ and $p_2$ from $P_t$ using binary tournament selection;

9 Apply genetic operators on $p_1$ and $p_2$ to produce two offsprings;

10 for each offspring o ∈ c do

11 Evaluate offspring s; // using method in Subsect. 4.3

12 if f(o) &lt; f(p1) and f(s) &lt; f(p2) then

13 localSearch(o); // using local search operator in Subsect. 4.4

14 C ← C ∪ c;

15 Select N fittest individuals from $P_t \cup C$ to form $P_{t+1}$;

16 Cluster $P_{t+1}$ into k groups using a k-medoids algorithm; // see Subsect. 5.4.1

17 Optimize charging time for k best charging paths of k groups using M-CMAES algorithm;

18 // see Subsect. 5.4

19 t ← t + 1;

20 return bestSolution;
</div>

The next subsection 5.3 indicates how we employ genetic algorithm and local search at the first stage (upper level). The selecting input for the second stage and details about this stage (lower level) is represented in Subsection 5.4.

## 5.3. Genetic algorithm for upper level optimization

## 5.3.1. Individual representation

Each individual presents a charging path for MC, which is a sequence of all sensors in the network. Thus, the permutation encoding is used in this algorithm, where each individual is a string of n natural numbers in range [1, n]. An individual is feasible if all the elements are not overlapping with others.

## 5.3.2. Genetic operators

N individuals of the initial population of the upper level are greedily generated by the method in Subsection 4.5 with the same motivation. In each generation, we employ the Partial Mapped Crossover (PMX) and the Swap Mutation [37] to produce the ofspring. Every time a generated ofspring is better than all of its parents, the local search operator proposed in Subsect. 4.4 is performed on that ofspring until the local optima is reached. This way, all of the promising regions of the search space that were explored by the genetic operators will be exploited rapidly.

After that, N best individuals are selected from the combination of current population and the generated ofspring to form the next generation population.

## 5.4. Multitasking covariance matrix adaptation evolution strategy for lower level optimization

Determining the optimal charging time for a given path is a hard problem, so it is computationally very expensive to optimize the charging time for all generated paths (i.e., perform the lower-level optimization for all members of the upper level population). Therefore, only several paths are selected from the upper level to be the input of the lower level task. In this subsection, we first represent how to perform that selection, and then the proposed M-CMAES will be presented in detail.

## 5.4.1. K-medoids algorithm for selecting charging paths

In this work, we propose a novel selection mechanism that does not need to perform the lower-level task for every member of the upper level but still covers the whole space of the upper level’s population. In particular, we first leverage a k-medoids algorithm to cluster the population of the upper level into $k$ groups based on individuals’ similarity. Then, k best individuals of k groups (one best individual for each group) are selected as the input for the lower level.

The similarity of individuals in the upper level population is measured based on the correlation in the position of sensors in the decoded paths (phenotypes). Let $\epsilon _ { P , P ^ { \prime } }$ denotes the similarity between two charging paths $\wp$ and $\wp ^ { \prime } .$ . Algorithm 4 represents how to calculate the value of $\epsilon _ { P , P ^ { \prime } }$ . Consider all pairs of sensors $\langle \pi _ { i } , \pi _ { j } \rangle$ in $\wp ,$ $( 1 \leq i < j \leq n _ { \ Z }$ , without replacement). $\wp$ and $\wp ^ { \prime }$ are similar in the pair $\langle \pi _ { i } , \pi _ { j } \rangle$ if $\langle \pi _ { i } , \pi _ { j } \rangle$ also appears in $\wp ^ { \prime }$ . In that case, $\epsilon _ { P , P ^ { \prime } }$ is increased by 1. Thus, $\begin{array} { r } { \epsilon _ { P , P ^ { \prime } } = \frac { n ( n - 1 ) } { 2 } } \end{array}$ is the maximum when $\wp$ and $\wp ^ { \prime }$ are exactly the same and $\epsilon _ { P , P ^ { \prime } } = 0$ is the minimum when $\wp$ is an inverse sequence of $\wp ^ { \prime }$ . Fig. 3 illustrates an example of algorithm 4.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 4: Calculate the similarity between $\wp$ and $\wp'$

Input: Two charging paths $\wp$ and $\wp'$.

Output: The similarity value $\epsilon_{P,P'}$.

1 begin
2    $\epsilon_{P,P'} \leftarrow 0$;
3    for $i \leftarrow 1$ to $n - 1$ do
4    for $j \leftarrow i + 1$ to $n$ do
5    if the pair of sensors $\langle \pi_i, \pi_j \rangle$ in $\wp$ appears in $\wp'$ then
6    $\epsilon_{P,P'} \leftarrow \epsilon_{P,P'} + 1$;
7 Return $\epsilon_{P,P'}$;
</div>

After calculating the pairwise similarity between individuals, a k-medoids algorithm, such as PAM is invoked to cluster the population. Due to the limited space, the deployment of PAM algorithm is omitted. Details about this algorithm could be found in [38].

## 5.4.2. M-CMAES algorithm for the lower level optimization

In the CMA-ES [30], the multivariate normal distribution is built from its population:

$$
\mathcal {N} (m, \sigma^ {2} \mathcal {C}) \sim m + \sigma \mathcal {N} (0, \mathcal {C}),\tag{28}
$$

![](images/03a21a3d3f44954f902952f5d134a6f46b6173d06e914f755332dca139f6d331.jpg)  
Figure 3: An example of calculating similarity between two charging paths. There are 5 pair of sensors: ⟨1, 2⟩, ⟨1, 4⟩, ⟨3, 2⟩, ⟨3, 4⟩, and ⟨2, 4⟩ that appear in both P<sub>1</sub> and P<sub>2</sub>. Thus, $\epsilon _ { P _ { 1 } , P _ { 2 } } = 5 $

where C is the covariance matrix, σ is the step size, and m is the mean vector of the population. During the evolutionary process, CMA-ES selects top µ fittest individuals from the population of λ individuals for updating vector m. λ ofspring are then sampled from the new mean vector m, following the distribution ${ \mathcal { N } } ( m , \sigma ^ { 2 } { \mathcal { C } } )$

In our algorithm, optimizing the charging time for each selected path is considered as a single task in the multitask environment. M-CMAES employs k separative populations for k tasks that evolve simultaneously. During the evolutionary process, knowledge transfer explicitly occurs in each task by replacing several individuals of its current population with the best solutions found so far in other tasks. Outlines of M-CMAES are represented in the Algorithm 5.

```txt
Algorithm 5: M-CMAES

Input: k charging paths that were selected from the upper level population and corresponding k sequence of initial charging time.

Output: Optimal charging time for k paths.

1 begin
2 Generate k initial populations for k tasks and evaluate all individuals for their associated task only;
3 t ← 0;
4 while terminate conditions are not satisfied do
5    for each task i ← 1 to k do
6    Randomly replace k - 1 individuals in the population of task i by k - 1 so far best solutions found by the rest k - 1 tasks;
7    Apply standard steps of CMA-ES to generate new population for task i, i.e., selection, sampling, and distribution updating;
8    Evaluate all individuals in the new population for task i only;
```

The key diference of proposed algorithm from the standard CMA-ES is the benefit of knowledge transfer between tasks. M-CMAES randomly replaces k − 1 individuals of a task by the best solutions found so far by the rest k − 1 tasks. By doing that, each task is allowed to inherit the achievement or the good genetic material of other tasks. The search performance of the algorithm is therefore improved, compared to solving each task in a standalone way. It is notable that since the input of all tasks is selected from the population of the upper level, it has certain similarities between tasks, i.e., all the tasks are highly relevant. Thus, the individual replacement can be eficiently performed without the control of random mating probability as in the traditional multifactorial evolutionary algorithm [39].

It is also worth mentioning that for each task, we leverage the charging time calculated by the greedy method in Subsect. 4.3 (Eq. 23) as a good starting point for the evolutionary strategy. The value of λ is set as default as $\lambda = \left| 4 + 3 \mathrm { l n } ( n ) \right|$ , where n is the number of sensors [40]. Thus, the number of tasks in the $\mathrm { M \mathrm { - } C M A E S , i . e . }$ , the number of clusters in the k-medoids algorithm (Subsect. 5.4.1) is determined by $\begin{array} { r } { k = \left\lceil \frac { N } { \lambda } \right\rceil } \end{array}$ , where N is the population size of M-CMAES.

## 6. Evaluation Results

This section evaluates the performances of our two proposed algorithms: Hybrid of Multi-start Local Search and Genetic Algorithm (MLSGA) and Multitasking approachbased Bi-level Charging Scheme (MTBCS). Furthermore, we compare our proposals with the most relevant works including an on-demand charging strategy named INMA [12], a periodic charging strategy named GACS [7], and HPSOGA from [20].

INMA sends a charging request to the MC whenever a sensor energy level drops below a predefined threshold. Then, the next to-be-charged node in the charging path will be selected based on their residual energy and Euclidean distance to the current location of the MC. In detail, the set of charging sensor candidates is selected to minimize the number of other requesting nodes that may sufer from energy depletion. The GACS algorithm decomposed the task of determining the charging schedule into two sub-problems: finding the optimal charging path and determining the optimal charging time for each sensor. An approximate algorithm using the genetic approach is applied for each sub-problem to find an eficient solution. Then the final charging scheme is constructed using the results of the two sub-problems. In HPSOGA, periodic charging planning for a mobile WCE with limited traveling energy is proposed. With the optimization objective of maximizing the docking time ratio, this periodic charging planning ensures that the energy of the nodes in the WRSN varies periodically and that nodes perpetually fail to die.

## 6.1. Network instances

The sensors are deployed in the network with the sensor field size of 500m × 500m and the base station positioned at the center. A total of 120 network instances are divided into three diferent types of networks based on the distribution method of the sensors:

• Uniform distribution: Coordinate $( x , y )$ of each sensor node is generated by a random function, where the $x _ { c o o r d i n a t e }$ and y are real value in range [0, W ].

• Normal distribution: The location of sensor nodes is generated following the Gaussian distribution. The sensors generated outside the sensor field will be discarded and regenerated to be inside.

• Grid distribution method: We divide the network area with dimensions $W \times W$ into 100 square cells. Each square cell has a side equal to $W / 1 0$ . We then randomly generate the location of sensors on each square cell.

For each deployed method, 40 network instances are divided into four sets based on their number of sensors: 25, 50, 75, 100, respectively. For each set with these settings, ten diferent network instances are generated. Each instance is named according to the following format $^ { \circ \circ } T y p e .$ Num $. O r d ^ { \prime \prime }$ with the Type corresponding to the distribution type of the network: $^ { 6 \delta } { \boldsymbol { r } } ^ { \eta }$ for Random distribution, $^ { * } n ^ { * }$ for the Normal distribution and, $^ { \ast } g ^ { \ast }$ for the Grid distribution; Num is the number of sensors, and Ord is the order of the instance in its set.

## 6.2. Simulation settings

We use the MC parameters provided in the works [24, 20]. The average energy consumption rate of each sensor is estimated using the method in [12] where the base station collects the information related to the sensor’s energy. The detail of the charging model parameters is presented in table 2. The INMA threshold of sending charging request $e _ { t h r e d }$ is set to $0 . 4 \times$ battery capacity of the sensor. Based on the experiments, all the genetic operators and parameters of algorithms MLSGA, MTBCS, GACS are kept identical as in table 3.

Table 2  
Charging model parameters

<table><tr><td>Parameters</td><td>Value</td></tr><tr><td> $E_{MC}$ </td><td>108000 (J)</td></tr><tr><td>U</td><td>5 (J/s)</td></tr><tr><td> $P_M$ </td><td>1 (J/s)</td></tr><tr><td>V</td><td>5 (m/s)</td></tr><tr><td> $e^{max}$ </td><td>10800 (J)</td></tr><tr><td> $e^{min}$ </td><td>540 (J)</td></tr></table>

Table 3  
The proposed algorithms’ parameters

<table><tr><td>Parameter</td><td>Value</td></tr><tr><td>Number of charging path evaluations</td><td>25000</td></tr><tr><td>Number of charging time evaluations for one charging path</td><td>25000</td></tr><tr><td>Population size for optimizing the charging path</td><td>100</td></tr><tr><td>Population size for optimizing the charging time</td><td>100</td></tr><tr><td>Crossover rate</td><td>0.9</td></tr><tr><td>Mutation rate</td><td>0.05</td></tr><tr><td>SBX distribution index</td><td>2</td></tr><tr><td>Polynomial Mutation distribution index</td><td>5</td></tr></table>

All experiments are implemented in the JMetalPy framework and conducted on a computer with an Intel Core i7-6700HQ CPU and 8GB of RAM

## 6.3. Evaluating criteria

The algorithm eficiency will be evaluated by the dead node ratio of the network calculated by:

$$
\mathrm{nodefailureratio} (\%) = \frac {\text {Numberofdeadnodes}}{\text {Numberofsensors}} \times 1 0 0
$$

We focus on the dead node ratio criteria because this is one of the most critical measurements to evaluate the eficiency of a charging scheme in WRSNs.

## 6.4. Experimental Results

To study the efects of the proposed algorithms in solving the WRSNs energy depletion problem and experiment with their performances in comparison with existing works, we perform five experiments on the received results:

• Experiment 1: Evaluate the efect of our proposed algorithms in comparison with INMA, GACS, and HPSOGA using statistical tests.

• Experiment 2: Evaluate the impact of the sensors parameters (number of sensors, average energy consumption rate) on the node failure ratio of the network.

• Experiment 3: Evaluate the impact of the MC parameters ( charging rate, MC battery capacity) on the node failure ratio of the network.

• Experiment 4: Evaluate the MTBCS convergence trend when applying the greedy and random initialization approaches.

• Experiment 5: Evaluate the algorithms running time.

6.4.1. Non-parametric statistic for comparing results of proposed algorithms and existing algorithm

In recent years, the use of statistical tests to enhance the evaluation process of a new method’s performance has become a common technique in computational intelligence [11, 41]. Considering the complexity of the charging scheduling problem in WRSNs, the requirements of parametric tests (i.e., independence, normality, and homoscedasticity) will not be satisfied. Therefore, to examine the performances of five algorithms MTBCS, MLSGA, GACS, INMA, and HPSOGA, we use Non-parametric tests to analyze the received results.

This study includes three main steps:

• The first step is to use statistical tests named Friedman and Friedman Align to justify any significant diferences among the performances of the algorithms.

• The second step is to perform multiple comparisons with a control method to highlight the diferences in performance of the best algorithm and others.

• The final step is to perform the median contrast estimation to estimate the magnitudes of the diferences between the performance of algorithms.

Results of the Friedman and Align Friedman tests in table 4 show that there is a statistically significant diference in the performance of 5 algorithms with p-value = 0 and p-value = 2.2e<sup>−10</sup> for Friedman and Aligned Friedman tests, respectively. However, at this stage, we only know that there are diferences somewhere between the related groups, but both the Friedman and Aligned Friedman tests can not pinpoint which groups, in particular, difer from each other.

As a result, additional statistical procedures need to be applied to analyze the performance diferences between pairs of algorithms. In more detail, efective post-hoc procedures such as Hom, Holland, Rom and Finner are conducted and MTBCS is considered as control algorithm to demonstrate the diferences in performance against the remaining algorithms (MLSGA, INMA, GACS, HPSOGA).

Table 4  
Average Ranking of Algorithms

<table><tr><td>Algorithm</td><td>Friedman</td><td>Aligned Friedman</td></tr><tr><td>MTBCS</td><td>1.3101</td><td>953.2601</td></tr><tr><td>MLSGA</td><td>2.0863</td><td>1351.8886</td></tr><tr><td>INMA</td><td>2.7637</td><td>1719.2142</td></tr><tr><td>GACS</td><td>3.9768</td><td>2783.6119</td></tr><tr><td>HPSOGA</td><td>4.8630</td><td>3694.5249</td></tr><tr><td>statistic value</td><td>2745.6590</td><td>656.6239</td></tr><tr><td>p-value</td><td>0.0000</td><td>2.2019E-10</td></tr></table>

Table 5  
z-values and p-values of the Friedman test (MTBCS is the control algorithm)

<table><tr><td>Algorithm</td><td>z-value</td><td>p-value</td><td>Holm</td><td>Holland</td><td>Rom</td><td>Finner</td></tr><tr><td>MLSGA</td><td>10.0606</td><td>8.2512E-24</td><td>0.05</td><td>0.05</td><td>0.05</td><td>0.05</td></tr><tr><td>GACS</td><td>34.5639</td><td>8.7979E-262</td><td>0.0166</td><td>0.0169</td><td>0.0166</td><td>0.0253</td></tr><tr><td>INMA</td><td>18.8404</td><td>3.5203E-79</td><td>0.025</td><td>0.0253</td><td>0.025</td><td>0.0377</td></tr><tr><td>HPSOGA</td><td>46.0518</td><td>0.0000</td><td>0.0125</td><td>0.0127</td><td>0.0131</td><td>0.0127</td></tr></table>

Table 5 and 6 display the results of Hom, Holland, Rom and Finner post-hoc procedures when comparing MTBCS with other four algorithms. Additionally, Table 7 and 8 show the adjusted p values for each comparison. We can see that in both the Friedman and Aligned Friedman tests, all procedures reject the null hypothesis with a degree of significance α = 0.05 which means that a statistically significant improvement can be obtained by applying the MTBCS algorithm instead of the other four. Therefore, MTBCS is considered as the best algorithm among the five algorithms

In order to further demonstrate the magnitudes of the diferences between pairs of algorithms, we also conducted the median based contrast estimation among the algorithms.

Table 9 shows the contrast estimation among pairs of algorithms. As can be seen, focusing in the rows of the table, specifically at the performance of MTBCS, all its related estimators are negative; that is, it achieves a very low dead node ratio considering median estimators. Considering MLSGA performance, the only positive value in its row belongs to MTBCS, and it is still the best performing algorithm compared to the other four (INMA, GACS, and HPSOGA) in our experimental study.

## 6.4.2. Impact of the sensors parameters

6.4.2.1. The number of sensors.

Table 6  
z-values and p-values of the Aligned Friedman test (MTBCS is the control algorithm)

<table><tr><td>Algorithm</td><td>z-value</td><td>p-value</td><td>Holm</td><td>Holland</td><td>Rom</td><td>Finner</td></tr><tr><td>MLSGA</td><td>6.7372</td><td>1.6141E-11</td><td>0.05</td><td>0.05</td><td>0.05</td><td>0.05</td></tr><tr><td>GACS</td><td>30.9349</td><td>4.0545E-210</td><td>0.0166</td><td>0.0169</td><td>0.0166</td><td>0.0253</td></tr><tr><td>INMA</td><td>12.9454</td><td>2.493E-38</td><td>0.025</td><td>0.0253</td><td>0.025</td><td>0.0377</td></tr><tr><td>HPSOGA</td><td>46.3303</td><td>0.0000</td><td>0.0125</td><td>0.0127</td><td>0.0131</td><td>0.0127</td></tr></table>

## Table 7

Adjusted p-values for the Friedman test (MTBCS is the control algorithm)

<table><tr><td>Algorithm</td><td>unadjusted p</td><td>pHolm</td><td>pHolland</td><td>pRom</td><td>pFinner</td></tr><tr><td>MLSGA</td><td>8.2512E-24</td><td>8.2512E-24</td><td>0.0000</td><td>8.2512E-24</td><td>0.0000</td></tr><tr><td>GACS</td><td>8.7979E-262</td><td>2.6393E-261</td><td>0.0000</td><td>2.6393E-261</td><td>0.0000</td></tr><tr><td>INMA</td><td>3.5203E-79</td><td>7.0407E-79</td><td>0.0000</td><td>7.0407E-79</td><td>0.0000</td></tr><tr><td>HPSOGA</td><td>0.0000</td><td>0.0000</td><td>0.0000</td><td>0.0131</td><td>0.0000</td></tr></table>

![](images/20c5b82ad0c6862e23c1d6959f629b7b9b0ac1216435c65198d31e4a1db4137f.jpg)  
(a) Grid distribution network

![](images/486f8d11584cc49025344b447b06b728191150bb4aa797cc63f7c875c3b0caf6.jpg)  
(b) Normal distribution network

![](images/9d118df05090bbae63ed1626f755135f9cdb6c026c7a349f2de5ce5920434adf.jpg)  
(c) Uniform distribution network  
Figure 4: Comparison of the node failure ratio and the number of sensor nodes in diferent sensor distribution.

Fig 4 illustrates the dead node ratio in three diferent deployment methods when the number of sensors varies from 25 to 100 nodes. As Fig 4 shows, the dead node ratios of the whole network are proportional to the number of sensors in all three types of network distributions. The reason is that the more sensors there are, the more data packets containing the sensory data and sensor’s residual energy are created. As a result, the sensors have to handle a substantially more enormous workload. Eventually, sensors deplete energy more quickly, which results in the increment of the node failure ratio of the network.

Table 8  
Adjusted p-values for the Aligned Friedman test (MTBCS is the control algorithm)

<table><tr><td>Algorithm</td><td>unadjusted p</td><td>pHolm</td><td>pHolland</td><td>pRom</td><td>pFinner</td></tr><tr><td>MLSGA</td><td>1.6141E-11</td><td>1.6141E-11</td><td>1.6141E-11</td><td>1.6141E-11</td><td>1.6141E-11</td></tr><tr><td>GACS</td><td>4.0545E-210</td><td>1.2163E-209</td><td>0.0000</td><td>1.2163E-209</td><td>0.0000</td></tr><tr><td>INMA</td><td>2.4930E-38</td><td>4.9861E-38</td><td>0.0000</td><td>4.9861E-38</td><td>0.0000</td></tr><tr><td>HPSOGA</td><td>0.0000</td><td>0.0000</td><td>0.0000</td><td>0.0000</td><td>0.0000</td></tr></table>

Table 9 Contrast Estimation

<table><tr><td></td><td>MTBCS</td><td>MLSGA</td><td>INMA</td><td>GACS</td><td>HPSOGA</td></tr><tr><td>MTBCS</td><td>0.000</td><td>-2.490</td><td>-4.440</td><td>-10.770</td><td>-19.500</td></tr><tr><td>MLSGA</td><td>2.490</td><td>0.000</td><td>-1.950</td><td>-8.280</td><td>-17.010</td></tr><tr><td>INMA</td><td>4.440</td><td>1.950</td><td>0.000</td><td>-6.330</td><td>-15.060</td></tr><tr><td>GACS</td><td>10.770</td><td>8.280</td><td>6.330</td><td>0.000</td><td>-8.730</td></tr><tr><td>HPSOGA</td><td>19.500</td><td>17.01</td><td>15.060</td><td>8.730</td><td>0.000</td></tr></table>

Further comparisons among the results on three distribution methods, we observed that all algorithms perform better when the sensors are deployed following the grid distribution. This phenomenon can be explained by the fact that the grid distribution makes the sensors less concentrated when compared to both the uniform and the normal distribution. Therefore, the workload of all sensors will be reduced since they have to communicate with fewer surrounding sensors. Because of that, the average energy consumption rate of all sensors will reduce, which decreases the node failure ratio of the network.

Considering the comparison among algorithms results, we can see that MTBCS and MLSGA significantly outperform INMA, GACS, and HPSOGA in all test cases, which vary in the number of sensors. Specifically, MTBCS reduces the dead node ratio of INMA by 15.81%, 10.36%, 13.51%, and HPSOGA by 26.27%, 16.56%, 24.48%, and GACS by 5.54% 5.75% and 6.47% for u, g and n sets. The MLSGA does not perform as well as the MTBCS, but it still reduces the dead not ratio of INMA by 11.75%, 11.79%, 9.78%, HPSOGA by 22.22%, 13.99%, 20.75%, and GACS by 1.48%, 3.18%, and 2.74% on g, n, and u sets respec tively. We can also see that the gap between our two algorithms and INMA tends to widen when the number of sensors increases. This phenomenon could be explained by the fact that INMA only optimizes the sensors in the charging queue but not the rest of the sensors, which will become a problem when the number of sensors is high, leading to a significan number of sensors not being in the queue, hence, the number of not-optimized sensors will also become notable. Considering the GACS, even though it has overcome the limitations of the INMA but since the process of determining the charging path and the charging time are separated, the charging path found in the first phase is not guaranteed to benefit the ultimate goal of the algorithm, which is to minimize the dead node percentage of the net work. On the contrary, both MTBCS and MLSGA optimize the charging time for multiple charging paths with the only goal to minimize the dead node ratio. Regarding HPSOGA, the indirect goal to maximize the docking time (time the MC spends recharging itself at the base station) has been inefective in harsh network conditions where sensor deaths are inevitable. Hence the dead node ratio of HPSOGA increased rapidly with the number of sensors.

6.4.2.2. Impact of average energy consumption rate.

![](images/084d474abc832f9da8cf3ab36ced4b481807219668776a9c799eed763356d7da.jpg)  
(a) Grid distribution network

![](images/87fc1aa313fa4fd6221cf53c926fab0daf71ac0453a8b315df457502781288d8.jpg)  
(b) Normal distribution network

![](images/52d3857a273f798d3583973bcaa630a9e2d7f5c557cc8766bb03049154447523.jpg)  
(c) Uniform distribution network  
Figure 5: Comparison of node failure ratio and average energy consumption rate for diferent sensor distributions

Figure 5 demonstrates the node failure ratio of grid network, normal network, and uniform network when the average power consumption rates of sensors are set at various values from 0.8J/s to 2J/s. The line charts show the increasing trend of the dead node ratio when the average energy consumption rate of the sensors increases in all three types of distribution and all four algorithms. Obviously, the higher energy consumption rate results in the shorter sensor’s lifetime and the higher energy needed to charge them.

Considering the algorithm’s results, we can see that MTBCS and MLSGA significantly outperformed INMA and HPSOGA and showed a better result than GACS. Specifically, on average of MTBCS reduces the dead node ratios of INMA by up to 10.78%, 8.76%, 10.65%, of HPSOGA by 56.09%, 66.41%, 60.48%, and of GACS by 3.53%, 4.30%, and 3.08% with respect to u, g, and n sets. The MLSGA reduces the dead node ratios of INMA up to 8.50%, 6.14%, 8.97%, of HPSOGA by up to 56.75%, 62.91%, 62.29%, and of GACS by 1.26%, 1.69%, and 1.42% on u, n, g sets, respectively. We can also see that the gap between the periodic charging schemes MTBCS, MLSGA, GACS, and INMA is considerable. As the sensor’s consumption rate increases, many charging requests are sent, which increases the number of sensors waiting to be optimized. Whereas the periodic charging schemes, especially our two proposed algorithms, adapt better to the increase of the power consumption rates of the sensors as they optimize all sensors in the network simultaneously.

![](images/1e81a001ab9afe603ca32ad422f322219173f0f24644ed1d06270f18f42c7212.jpg)  
(a) Grid distribution network

![](images/591cddabcfad66676f06c8e2289324bc2da86652c61345dad78e54bd0aaecf61.jpg)  
(b) Normal distribution network

![](images/8d03f3f60238a3ef089afd403b15b531f81a60fb2d5d8fca9ec3cad78388cff8.jpg)  
(c) Uniform distribution network  
Figure 6: Comparison of the node failure ratio and the charging rate of the Mobile Charger for three distributions

Figure 6 shows the node failure ratio of grid network, normal network, and uniform network when the MC charging power is set at various values from 2J/s to 25J/s. From the three graphs, we can see that the network dead node ratios are inversely proportional to the charging rate of the MC. This is clear because the higher charging rates correspond to less time spent on charging the sensors, making the waiting to be charged time of all sensors lessened.

Further comparison between algorithms results showed that MTBCS and MLSGA both outperform INMA, GACS, and HPSOGA on all values of the charging ratio of MC. Specifically, MTBCS reduces the dead node ratio rates to 15.81%, 8.3%, 11% when compared to INMA, 56.55%, 38.5%, 48.9% when compared to HPSOGA, and 5.81%, 1.76%, 3.08% when compared to GACS on u, g, n distribution network respectively. MLSGA reduces the dead node ratio rates up to 12.46%, 7.72%, 10.07% when compared to INMA, 55.46%, 37.92%, 47.97% when compared to HPSOGA and 1.4%, 1.18%, 2.15% when compared to GACS.

We can also notice that the gaps between INMA and our two algorithms widen when the charging rate increases. For INMA, the charging request is only made when the sensor power drops below a fixed threshold. This approach will not be able to take full advantage of the high MC charging power because the bigger MC charging power should keep all sensors on a larger threshold value. On the other hand, our proposed algorithms can adjust both the charging path and charging time to utilize the high MC charging power.

Regarding the GACS, although it also adapts well to the high MC charging rate, because of the separation between its two phases, only the charging time is adapted to the high MC charging rate but not the charging path. Consequently, although it still performs worse than MTBCS and MLSGA, the gap is insignificant even when the charging rate reached 25J/s.

![](images/65c2e5f1ec6e76b087319be87434d1963dea252332a6eb079e4306a57cf74323.jpg)  
Energy capacity of the MC (J)  
(a) Grid distribution network

![](images/1c496cbc84ad25950a043b65d3eac3bc6a2a7805ccb767880f3d281f95c0bb98.jpg)  
Energy capacity of the MC (J)  
(b) Normal distribution network

![](images/7bd42186b056cf7fcc52b8fd07973f99f88ca7d1160b21aa1e7d8183c809d837.jpg)  
Energy capacity of the MC (J)  
(c) Uniform distribution network  
Figure 7: Comparison of the node failure ratio and the battery of Mobile Charger for three distributions

Fig 7 displays the node failure ratio of grid network, normal network, and uniform network when the MC battery capacity is set at various values from 13500J to 108000J. We can see that although the dead node percentage reduces when the MC energy capacity increases, the impact is small and become minuscule as the MC capacity reach a certain value. Specifically, when the MC energy capacity increases from 13500J to 2700J, the dead node ratio only declines 4.33%, 1.22%, 2.34% for the MTBCS algorithm and 5.65%, 1.67%, 3.24% for the MLSGA on u, g and n distribution network, respectively. Furthermore, when the MC capacity increases from 81000J to 108000J, the dead node ratio of MTBCS and HLSGA stop decreasing in all three network distributions.

Further examining the results among algorithms, we can also see that our algorithms significantly outperform INMA and GACS on all values of the MC capacity. Specifically, MTBCS reduces the dead node ratio of INMA by 15.81%, 10.36%, 13.51%, HPSOGA by 26.31%, 16.86%, 24.61% and GACS by 5.54%, 5.75%, 6.47%, 24.61% on u, g and n distribution network respectively. In addition, MLSGA reduces the dead node ratio of INMA by 11.8%, 7.69%, 9.89%, 20.79%, HPSOGA 22.72%, 14.95%, 20.99%, and GACS by 1.53%, 3.08%, 2.85% with respect to u, g and n distribution.

The above observations have shown that the MC energy capacity does not play a decisive role in the dead node ratio of the network. This can be explained because when the sensor’s parameters and the MC charging power are fixed, the MC only needs a fixed amount of energy to charge and move around the network. Hence, when the MC capacity goes pass this fixed value, it no longer has any impact on the dead node percentage of the network. Because of this reason, the following experiment is conducted to examine the impacts of both the MC charging power and energy capacity on the failure node ratio of the network.

![](images/4f83add3dae674a253bfdc41731b4f3735257fef9ac61fd582f2549f00887ac7.jpg)  
(a) Grid distribution network

![](images/8335dd5d1c5a770887062369715ae58ae43ffc939560c62616ed0c56fc2a46dd.jpg)  
(b) Normal distribution network

![](images/e769454f8e798516726521eb41744dccf7cca0f16298f1e62e43b660a638042c.jpg)  
(c) Uniform distribution network  
Figure 8: Impact of the charging rate and the MC’ battery on the node failure ratio

Figure 8 illustrates the node failure ratio when the MC battery capacity is set at various values from 13500J to 108000J on a grid network, a normal network, and uniform network. From Figure 8 we can see that the network dead node ratios are inversely proportional to the charging rate and the charging power of the MC. Unlike the only MC energy experiment, the addition of the charging power factor helps substantially reduce the dead node percentage of the network. This could be explained by the fact that it could fully utilize its high energy level when the MC has high charging power. With both high energy and charging power, the MC is capable of charging the sensors faster and replenishing the sensors to a higher level of energy; these stats resulted in a great decrease in the network replenish ratio.

Comparing the performances of all five algorithms, we can see that MTBCS outperforms INMA, GACS, and HPSOGA in all experiment varieties while the MLSGA outperforms INMA, HPSOGA and has similar results when compared to GACS. In detail, MTBCS reduces the dead node ratio of INMA up to 11.37%, 8.16%, 10.49%, HPSOGA up to 48.39%, 38.55%, 48.00% , and GACS up to 2.23%, 1.90%, 2.59% on u, g and n distribution network respectively. Furthermore, MLSGA reduces the dead node ratio of INMA by 9.69%, 6.75%, 8.91%, HPSOGA by 47.25%, 37.89%, 46.75%, GACS by 0.54%, 0.49%, 1.01% with respect to u, g, and n network.

![](images/c86f5f2b142d51cd07a500bb3cae5a8884da7b9902c50b40531e2f16bc9718ac.jpg)

![](images/f79a57427d83f8873a26751a2a20a508005efea0e5e158fced16038587c03986.jpg)

![](images/72eedb65c552da1c418424f3e91ba3f5d313d4df1f04993212471f6d161d5c5f.jpg)  
Figure 9: The convergence trend of MTBCS algorithm

Figure 9 illustrates the convergence trends of MTBCS when applying the greedy and the random initialization approach on six instances including two from each type of network: grid network, normal network and uniform network. According to the three line graphs, the greedy initialization method always provide a better starting solution for the searching process. Furthermore, the convergence speed when applying the greedy initialization is also consistently higher. Regarding the quality of the final solutions, it can be seen that the MTBCS with the greedy initialization always outperform the random initialization.

## 6.4.5. Algorithm’s run times analysis

Table 10  
Running time comparison between algorithms (seconds)

<table><tr><td>Number of sensors</td><td>GACS</td><td>MLSGA</td><td>MTBCS</td><td>HPSOGA</td></tr><tr><td>25</td><td>0.069</td><td>0.20</td><td>1.92</td><td>12.02</td></tr><tr><td>50</td><td>0.18</td><td>2.05</td><td>5.44</td><td>38.56</td></tr><tr><td>75</td><td>0.36</td><td>2.34</td><td>11.52</td><td>61.66</td></tr><tr><td>100</td><td>0.48</td><td>4.08</td><td>17.4</td><td>85.70</td></tr></table>

Table 10 shows the detailed running time of MTBCS, MLSGA, GACS, and HPSOGA algorithms. INMA is not included in this evaluation process because it is an on-demand charging algorithm; hence the results of INMA should be obtained at an instance to satisfy the real-time constraints of its charging model. As can be seen, regarding various sensors, GACS is always the fastest algorithm among the three, and HPSOGA is the slowest. MTBCS and MLSGA are the second and third slowest algorithms among the four. This trend can be explained by the fact that GACS only optimizes charging time for the best charging path obtained by the first phase, whereas the charging time optimization is performed once every iteration in MLSGA and ktimes per iteration in MTBCS. However, considering the proposed algorithms’ performance improvements to GACS combined with the static nature of the periodic charging approach, the running time is still acceptable, and the trade-of is worthy. On the other hand, HPSOGA is the slowest algorithm while having the worst result because of its indirect goal to maximize the docking time of the MC.

## 7. Conclusions

In this paper, we study the problem of energy depletion avoidance in WRSN. To achieve that ultimate goal, we focus on minimizing the number of dead sensor nodes after the charging process based on an optimized bi-level charging approach where the charging path and charging time are simultaneously solved. Since the search space is enormous and complex, we proposed two meta-heuristic algorithms with two novel search strategies to handle the investigated problem. The first algorithm MLSGA starts from multiple points to explore the search space and then exploits the feasible space by genetic algorithm. The second algorithm, MTBCS, leverages the superiority of the multitasking approach and covariance adaptation evolutionary strategy to optimize charging time at the lower level. Finally, we extensively performed experiments in network scenarios and the experimental results have demonstrated that our charging algorithms significantly reduce the number of dead nodes compared to the benchmark.

## Acknowledgment

Funding: This research is funded by Vietnam National Foundation for Science and Technology Development (NAFOSTED) under grant number 102.01-2019.304.

## References

[1] M. Kocakulak, I. Butun, An overview of wireless sensor networks towards internet of things, in: 2017 IEEE 7th annual computing and communication workshop and conference (CCWC), IEEE, 2017, pp. 1–6.

[2] P. Le Nguyen, Y. Ji, K. Le, T.-H. Nguyen, Load balanced and constant stretch routing in the vicinity of holes in wsns, in: 2018 15th IEEE annual consumer communications & networking conference (CCNC), IEEE, 2018, pp. 1–6.

[3] D. Kandris, C. Nakas, D. Vomvas, G. Koulouras, Applications of wireless sensor networks: an up-todate survey, Applied System Innovation 3 (1) (2020) 14.

[4] A. B. Noel, A. Abdaoui, T. Elfouly, M. H. Ahmed, A. Badawy, M. S. Shehata, Structural health monitoring using wireless sensor networks: A comprehensive survey, IEEE Communications Surveys & Tutorials 19 (3) (2017) 1403–1423.

[5] P. Le Nguyen, K. Nguyen, H. Vu, Y. Ji, Telpac: A time and energy eficient protocol for locating and patching coverage holes in wsns, Journal of Network and Computer Applications 147 (2019) 102439.

[6] J. Min, J. Kim, Y. Kwon, Y. Lee, Multi-channel mac protocol for real-time monitoring of weapon flight test in wireless sensor network, in: Proc. of the 6th Int’l Conf. on Sensor Technologies and Applications (SENSORCOMM 2012), 2012, pp. 83–88.

[7] T. T. Huong, P. Le Nguyen, H. T. T. Binh, K. Nguyenz, N. M. Hai, et al., Genetic algorithm-based periodic charging scheme for energy depletion avoidance in wrsns, in: 2020 IEEE Wireless Communications and Networking Conference (WCNC), IEEE, 2020, pp. 1–6.

[8] P. Le Nguyen, N. T. Hanh, N. T. Khuong, H. T. T. Binh, Y. Ji, Node placement for connected target coverage in wireless sensor networks with dynamic sinks, Pervasive and Mobile Computing 59 (2019) 101070.

[9] W. Xu, W. Liang, X. Jia, Z. Xu, Z. Li, Y. Liu, Maximizing sensor lifetime with the minimal service cost of a mobile charger in wireless sensor networks, IEEE Transactions on Mobile Computing 17 (11) (2018) 2564–2577.

[10] Y. Feng, N. Liu, F. Wang, Q. Qian, X. Li, Starvation avoidance mobile energy replenishment for wireless rechargeable sensor networks, in: 2016 IEEE International Conference on Communications (ICC), IEEE, 2016, pp. 1–6.

[11] A. Kaswan, A. Tomar, P. K. Jana, An eficient scheduling scheme for mobile charger in on-demand wireless rechargeable sensor networks, Journal of Network and Computer Applications 114 (2018) 123– 134.

[12] J. Zhu, Y. Feng, M. Liu, G. Chen, Y. Huang, Adaptive online mobile charging for node failure avoidance in wireless rechargeable sensor networks, Computer Communications 126 (2018) 28–37.

[13] N. T. Long, T. T. Huong, N. N. Bao, H. T. T. Binh, P. Le Nguyen, K. Nguyen, Q-learning-based distributed multi-charging algorithm for large-scale wrsns, Nonlinear Theory and Its Applications, IEICE 14 (1) (2023) 18–34.

[14] L. Fu, P. Cheng, Y. Gu, J. Chen, T. He, Optimal charging in wireless rechargeable sensor networks, IEEE Transactions on Vehicular Technology 65 (1) (2015) 278–291.

[15] W. Ouyang, X. Liu, M. Obaidat, C. Lin, H. Zhou, T. Liu, K.-F. Hsiao, Utility-aware charging scheduling for multiple mobile chargers in large-scale wireless rechargeable sensor networks, IEEE Transactions on Sustainable Computing (2020).

[16] L. Xie, Y. Shi, Y. T. Hou, W. Lou, H. D. Sherali, S. F. Midkif, Multi-node wireless energy charging in sensor networks, IEEE/ACM Transactions on Networking 23 (2) (2014) 437–450.

[17] L. Fu, L. He, P. Cheng, Y. Gu, J. Pan, J. Chen, Esync: Energy synchronized mobile charging in rechargeable wireless sensor networks, IEEE Transactions on vehicular technology 65 (9) (2015) 7415– 7431.

[18] X. Ye, W. Liang, Charging utility maximization in wireless rechargeable sensor networks, Wireless Networks 23 (7) (2017) 2069–2081.

[19] W. Xu, W. Liang, X. Lin, G. Mao, Eficient scheduling of multiple mobile chargers for wireless sensor networks, IEEE Transactions on Vehicular Technology 65 (9) (2015) 7670–7683.

[20] Z. Lyu, Z. Wei, J. Pan, H. Chen, C. Xia, J. Han, L. Shi, Periodic charging planning for a mobile wce in wireless rechargeable sensor networks based on hybrid pso and ga algorithm, Applied Soft Computing 75 (2019) 388–403.

[21] W. Xu, W. Liang, X. Jia, Z. Xu, Maximizing sensor lifetime in a rechargeable sensor network via partial energy charging on sensors, in: 2016 13th Annual IEEE International Conference on Sensing, Communication, and Networking (SECON), IEEE, 2016, pp. 1–9.

[22] T. H. Nguyen, P. Le Nguyen, et al., Extending network lifetime by exploiting wireless charging in wsn, in: 2020 RIVF International Conference on Computing and Communication Technologies (RIVF), IEEE, 2020, pp. 1–6.

[23] F. Carrabs, C. D’Ambrosio, A. Raiconi, Optimization of sensor battery charging to maximize lifetime in a wireless sensors network, Optimization Letters (2020) 1–14.

[24] Y. Shi, L. Xie, Y. T. Hou, H. D. Sherali, On renewable sensor networks with wireless energy transfer, in: 2011 Proceedings IEEE INFOCOM, IEEE, 2011, pp. 1350–1358.

[25] M. Tian, W. Jiao, J. Liu, The charging strategy of mobile charging vehicles in wireless rechargeable sensor networks with heterogeneous sensors, IEEE Access 8 (2020) 73096–73110.

[26] Y. Peng, Z. Li, W. Zhang, D. Qiao, Prolonging sensor network lifetime through wireless charging, in: 2010 31st IEEE Real-Time Systems Symposium, IEEE, 2010, pp. 129–139.

[27] C. Lin, Y. Zhou, H. Dai, J. Deng, G. Wu, Mpf: Prolonging network lifetime of wireless rechargeable sensor networks by mixing partial charge and full charge, in: 2018 15th Annual IEEE International Conference on Sensing, Communication, and Networking (SECON), IEEE, 2018, pp. 1–9.

[28] X. Yang, G. Han, L. Liu, A. Qian, W. Zhang, Igrc: An improved grid-based joint routing and charging algorithm for wireless rechargeable sensor networks, Future Generation Computer Systems 92 (2019) 837–845.

[29] T. T. Huong, H. T. T. Binh, P. Le Nguyen, V. D. An, et al., Optimizing charging locations and charging time for energy depletion avoidance in wireless rechargeable sensor networks, in: 2020 IEEE Congress on Computational Intelligence, (WCCI), IEEE, 2020

[30] A. Auger, N. Hansen, Tutorial cma-es: evolution strategies and covariance matrix adaptation, in: Proceedings of the 14th annual conference companion on Genetic and evolutionary computation, 2012, pp. 827–848.

[31] W. B. Heinzelman, A. P. Chandrakasan, H. Balakrishnan, An application-specific protocol architecture for wireless microsensor networks, IEEE Transactions on wireless communications 1 (4) (2002) 660–670.

[32] A. Sinha, P. Malo, K. Deb, A review on bilevel optimization: from classical to evolutionary approaches and applications, IEEE Transactions on Evolutionary Computation 22 (2) (2017) 276–295.

[33] G. Kizilate¸s, F. Nuriyeva, On the nearest neighbor algorithms for the traveling salesman problem, in: Advances in Computational Science, Engineering and Information Technology, Springer, 2013, pp. 111–118.

[34] K. Deb, D. Deb, Analysing mutation schemes for real-parameter genetic algorithms, International Journal of Artificial Intelligence and Soft Computing 4 (1) (2014) 1–28.

[35] T. T. B. Huynh, D. T. Pham, B. T. Tran, C. T. Le, M. H. P. Le, A. Swami, T. L. Bui, A multifactorial optimization paradigm for linkage tree genetic algorithm, Information Sciences 540 (2020) 325–344.

[36] T. B. Thang, N. B. Long, N. V. Hoang, H. T. T. Binh, Adaptive knowledge transfer in multifactorial evolutionary algorithm for the clustered minimum routing cost problem, Applied Soft Computing (2021) 107253.

[37] A. E. Eiben, J. E. Smith, et al., Introduction to evolutionary computing, Vol. 53, Springer, 2003.

[38] E. Schubert, P. J. Rousseeuw, Faster k-medoids clustering: improving the pam, clara, and clarans algorithms, in: International conference on similarity search and applications, Springer, 2019, pp. 171– 187.

[39] A. Gupta, Y.-S. Ong, L. Feng, Multifactorial evolution: toward evolutionary multitasking, IEEE Transactions on Evolutionary Computation 20 (3) (2015) 343–357.

[40] K. Nishida, Y. Akimoto, Psa-cma-es: Cma-es with population size adaptation, in: Proceedings of the Genetic and Evolutionary Computation Conference, 2018, pp. 865–872.

[41] N. Kumar, D. Dash, M. Kumar, An eficient on-demand charging schedule method in rechargeable sensor networks, Journal of Ambient Intelligence and Humanized Computing (2020) 1–18.
