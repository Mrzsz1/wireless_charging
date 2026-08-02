---
title: "Joint Scheduling and Trajectory Optimization of Charging UAV in Wireless Rechargeable Sensor Networks"
year: null
source_type: paper
why_relevant: ""
acquisition_method: auto_discovery
discovered_via: ["openalex"]
discovery_run: "raw/inbox/auto-discovered/runs/search-20260714-214003"
triage_status: promoted
selected_by_user: true
acquired_at: "2026-07-14T13:40:03+00:00"
canonicalized_at: 2026-07-14
ingest_status: ingested
pdf_path: "raw/canonical/Joint_Scheduling_and_Trajectory_Optimization_of_Charging_UAV_in_Wireless_Rechargeable_Sensor_Net/Joint_Scheduling_and_Trajectory_Optimization_of_Charging_UAV_in_Wireless_Rechargeabl.pdf"
raw_md: "raw/canonical/Joint_Scheduling_and_Trajectory_Optimization_of_Charging_UAV_in_Wireless_Rechargeable_Sensor_Net/full.md"
---
# Joint Scheduling and Trajectory Optimization of Charging UAV in Wireless Rechargeable Sensor Networks

Yanheng Liu, Hongyang Pan, Geng Sun, Member, IEEE, Aimin Wang, Jiahui Li, and Shuang Liang

Abstract—Wireless rechargeable sensor networks with a charging unmanned aerial vehicle (CUAV) have the broad application prospects in the power supply of the rechargeable sensor nodes (SNs). However, how to schedule a CUAV and design the trajectory to improve the charging efficiency of the entire system is still a vital problem. In this paper, we formulate a joint-CUAV scheduling and trajectory optimization problem (JSTOP) to simultaneously minimize the hovering points of CUAV, the number of the repeatedly covered SNs and the flying distance of CUAV for charging all SNs. Due to the complexity of JSTOP, it is decomposed into two optimization subproblems that are CUAV scheduling optimization problem (CSOP) and CUAV trajectory optimization problem (CTOP). CSOP is a hybrid optimization problem that consists of the continuous and discrete solution space, and the solution dimension in CSOP is not fixed since it should be changed with the number of hovering points of CUAV. Moreover, CTOP is a completely discrete optimization problem. Thus, we propose a particle swarm optimization (PSO) with a flexible dimension mechanism, a K-means operator and a punishment-compensation mechanism (PSOFKP) and a PSO with a discretization factor, a 2-opt operator and a path crossover reduction mechanism (PSOD2P) to solve the converted CSOP and CTOP, respectively. Simulation results evaluate the benefits of PSOFKP and PSOD2P under different scales and settings of the network, and the stability of the proposed algorithms is verified.

Index Terms—Wireless rechargeable sensor networks, scheduling and trajectory optimization, unmanned aerial vehicle, particle swarm optimization.

## I. INTRODUCTION

Wireless sensor networks (WSNs) are practical techniques that are extensively utilized in Internet of Things (IoT), e.g., smart homes [1] [2] [3]. A WSN is composed of many sensor nodes (SNs) that are randomly deployed in certain areas, and once deployed, the locations of these SNs are usually fixed and the network can be used to conduct monitoring and communication [4] tasks. In order to ensure that SNs work safely and effectively, it is necessary to ensure the power supply demand of SNs [5]. However, SNs are usually deployed in unavailable areas, which means that it is impractical to replace the batteries frequently [6]. Moreover, a large number of discarded batteries will lead to the environmental pollutions. Thus, many approaches are dedicated to optimizing the network protocols [7] to prolong the lifetime of WSNs. Although these strategies can prolong the lifetime of WSNs [8] to some extent, the limited battery capacity is still a challenge [9] for a single SN.

With the rapid development of wireless power transfer (WPT) technology [10] [11] [12], wireless rechargeable sensor networks (WRSNs) that combine WSNs and WPT can largely solve the battery capacity problem [13]. Since the number of SNs in a WRSN is usually large, it is not economical to use a fixed charger for replenishing energy to each node [14] [15]. Thus, using mobile devices such as unmanned aerial vehicles (UAVs) instead of fixed chargers becomes a feasible and widely used solution [16] [17]. However, due to the limited energy of UAVs and the demand for charging timeliness, it is necessary to reasonably schedule UAVs to improve the charging efficiency.

The scheduling of charging UAV (CUAV) is a key point to improve the charging efficiency between a CUAV and SNs in WRSN since it is better to charge all SNs in a shorter time. However, a CUAV may face several challenges for achieving the abovementioned purpose. For example, in a large-scale WRSN with large number of SNs, the number of CUAV hovering points will be huge, causing the increasing of CUAV hovering energy consumption and the reduction of CUAV endurance time. Moreover, some of SNs may be repeatedly charged, such that affecting the total charging efficiency. In addition, the flight order and trajectory of CUAV needs to be considered since it will directly affect the flying overhead of CUAV.

To solve the abovementioned challenges, we aim to jointly consider the hovering points and flying distance of CUAV for improving the charging efficiency. The main contributions of this work are summarized as follows:

• Different from the previous works that usually aim to charge SNs by using the static charging piles [18] or mobile charging vehicles [19], we propose to use a

CUAV to charge all SNs in a WRSN so that improving the charging performance of the network. Specifically, we formulate a joint-CUAV scheduling and trajectory optimization problem (JSTOP) to simultaneously minimize the hovering points of CUAV, the number of the repeatedly covered SNs and the flying distance of CUAV for charging all SNs.

• JSTOP is proven as NP-hard, and it has both continuous and discrete solution spaces. Moreover, it cannot be directly solved by using conventional algorithm in one step since solving the third objective function needs to determine the hovering locations of CUAV in advance, which needs to be determined by solving the first and second objective functions. Thus, we divide the original formulated JSTOP into two sub-problems that are CUAV scheduling optimization problem (CSOP) and CUAV trajectory optimization problem (CTOP), respectively.

• CSOP is also a hybrid optimization problem that consists of the continuous and discrete solution space, and the solution dimension in CSOP is not fixed since it should be changed with the number of hovering points of CUAV. Moreover, CTOP is a completely discrete optimization problem. Thus, we propose two different improved versions of particle swarm optimization (PSO) algorithms, i.e., PSO with a flexible dimension mechanism, a Kmeans operator and a punishment-compensation mechanism (PSOFKP) and PSO with a discretization factor, a 2-opt operator and a path crossover reduction mechanism (PSOD2P), to solve converted CSOP and CTOP, respectively. The proposed charging approach as well as the algorithms can ensure that all SNs are effectively charged in the shortest time so that achieving the maximum energy utilization efficiency and charging efficiency of CUAV.

• Simulations are conducted to evaluate the effectiveness and performance of the proposed CUAV-based charging approach. Specifically, we show the benefits of the proposed PSOFKP and PSOD2P for solving the formulated CSOP and CTOP under different scales and settings of the network. Moreover, the stability of the proposed algorithms is verified.

The remainder of this paper is organized as follows. Section II reviews the related work. Section III shows the system model. Section IV formulates JSTOP and gives the corresponding analysis. Section V proposes the algorithms. Section VI shows the simulation results. Finally, the conclusion is given in Section VII.

## II. RELATED WORK

Many previous works have considered installing antennas on UAVs that can transmit RF waves from UAVs to SNs previously [20] [21] [22]. However, as a potential technology of WPT, ER technology is used more and more in real scenarios [23]. The former usually needs to satisfy a high requirement of air attitude of UAVs. In other words, once the transmitting antennas on UAVs are not directed at the receiving antenna on the ground, the transmission efficiency is greatly vulnerable. Instead, the latter is robust in tolerating a mite deviation from the predetermined angle [24]. Thus, this work focuses on the application of ER technology to UAV charging, and we introduce a summary of some related work to show the novelty of our work.

Several previous works study the problem of the coverage of the UAVs. For example, a tethered UAV coverage plan by building a UAV networking system is proposed in [25], and the authors design the network architecture and UAV platform of tethered UAV and proves that the plan can satisfy the requirements of the emergency communication. The authors in [26] propose a game-theoretic framework to perform coverage maximization and power control which both of which have Nash equilibrium points, then multi-UAV energy-efficient coverage deployment algorithm based on spatial adaptive play is adopted to guarantee optimal energy-efficient coverage deployment. Liu et al. [27] use the reinforcement learning method to build a novel UAV coverage model, which makes a UAV consume less energy by reducing UAV action times when covering the same size area. Moreover, they propose a reliable method to solve the huge problem of the dimension space of the value function. The authors in [28] propose a novel antenna array with parasitic elements and install it on UAVs. The results show that the method can improve the coverage range by approximately 156%. However, none of these works above considers the scheduling and trajectory of UAV.

Some previous works optimize the deployment and scheduling of CUAVs in WRSNs to enhance the charging performance. For instance, the authors in [23] propose a method to maximize the energy efficiency of a CUAV by adjusting its trajectory, and the proposed method has advantages in charging performance and running time. However, SNs may be charged repeatedly, which may cause unnecessary resource consumption. Najeeb et al. [29] propose a practical algorithm that can charge SNs without knowing the power level in advance. However, the flight energy consumption of CUAVs is not considered. Xu et al. [30] adopt a Q-learning model in which the agent constantly explores and optimizes the scheduling strategy of a UAV, and the method improves the stability of the system by a maximum of 78% compared with no charging scheme. However, the flight freedom of CUAVs is limited, which is impractical for the real scenarios. Chen et al. [31] use WPT technology to charge a CUAV, and the deployment of CUAV and the power of the charging station are optimized such that maximizing the overall power transmission efficiency. However, SNs are not considered. Yang et al. [32] propose a controllable and obstacle avoidance CUAV routing algorithm to minimize the number of dead SNs in the network and maximize the energy efficiency during charging process. However, the problem of repeated charging is not considered. Zhu et al. [33] investigate how to deploy a rechargeable directional sensor network using a mobile charger with the least number of nodes for perpetual target coverage subject to the limited sensing angles of directional sensors and limited energy capacity of the mobile charger. However, the motion energy of the mobile charger is not considered. Li et al. [6] use the joint deployment of multiple CUAVs to improve the number of SNs in the charging range, maximize the minimum value of charging efficiency and reduce the movement energy consumption of CUAVs. However, the trajectories of CUAVs are ignored. Accordingly, these works above only consider to optimize the positions of CUAVs but do not jointly change the positions and amount of hovering points. Moreover, the number of repeatedly covered SNs is also not considered.

Moreover, after the scheduling of CUAV, it is necessary to determine the trajectory to further reduce the flight energy consumption of CUAV. There are several existing works that consider the trajectory optimization of UAVs. For example, Salamat et al. [34] propose a trajectory generation algorithm to represent the trajectory of UAVs and form a tool to estimate the test location and state. However, they do not consider any background about communication or charging. Ding et al. [35] optimize the UAV trajectory based on graph theory in order to minimize the mission completion time of UAV and ensure the connectivity requirements simultaneously. Compared to the conventional method, the performance of this new method is similar, while the complexity is greatly reduced. However, how to properly distribute the mission to separate the flight areas of UAVs is still a problem. Ebrahimi et al. [36] adopt a novel reinforcement learning method to make UAV find its trajectory autonomously so as to improve the positioning accuracy of multiple objects and reduce energy consumption of UAV. However, the stability of the proposed method is ignored. Yh et al. [37] design a method that optimizes the flight path of UAV and adaptively controls the interference power on the suspicious link to enhance the average surveillance rate in a limited flight time. However, the algorithm only perform advantages while solving small scale problem. The authors [38] boost the security of a relay assisted wireless communication system by optimizing the flight trajectory of UAV. However, they ignore the complete coverage of SNs in WRSN. Wu et al. [39] propose a novel evolutionary algorithm to achieve wireless power and information transfer with SNs, then jointly maximize the energy utilization efficiency of UAVs, and minimize the communication delay by optimizing the trajectory. However, increasing the number of UAVs may cause increase the overhead.

In recent years, there are several previous works that consider to use the evolutionary algorithms, e.g., PSO and its variants, to solve the different optimization problems [40] [41]. For example, Zhang et al. [42] present a novel PSO which can deal with the irregular and unpredictable noise in real-life applications. Although the algorithm has obvious advantages in most test functions, it is not adaptive to solve the discrete problem. Shao et al. [43] introduce a multi-objective hybrid PSO for solving flexible job-shop scheduling problem and this algorithm considers the discrete solution space. However, it ignores the hybrid solution condition which consists of both continuous and discrete solution space. Liang et al. [44] use a novel learning strategy whereby historical best information of all other particles is used to update the velocity of the particle. This method enables the diversity of the swarm to be preserved to discourage premature convergence. However, it does not consider a priori information, which may cause additional unnecessary cost. Moreover, Cao et al. [45] embed the local search strategy in conventional PSO for achieving higher convergence rate and accuracy. However, it may make the algorithm fall into local optima in some conditions. The authors [46] propose a parallel PSO with a special operator to obtain faster optimization. However, the improvement of algorithm performance is limited.

![](images/c7657b84141cd443c3eefd513d1b464fe6d57f6e61e80bde1176d07ba0980e20.jpg)  
Fig. 1. System model of a CUAV-enabled WRSN.

## III. SYSTEM MODELS

In this section, the network model, wireless charging model and energy consumption model of CUAV are introduced.

## A. Network model

As shown in Fig. 1, we consider a CUAV-enabled WRSN system that consists of a CUAV and n SNs recorded as $S _ { i } ( i \in \{ 1 , \ 2 , \ 3 , \ . . . , \ n \} )$ . The flight height of CUAV for charging SNs is fixed at h, which means that CUAV flying at the plane β. Moreover, SNs are randomly deployed on the plane α, and each of them can be powered by a CUAV. CUAV visits different hovering positions and it can simultaneously charge several SNs within the effective charging range such that all nodes can get power eventually. The 3D coordinates of SNs are defined as $S _ { i } = ( x _ { i } , y _ { i } , 0 )$ , and thus, the distance $D _ { u s }$ between CUAV $\boldsymbol { q } ( t ) = ( \boldsymbol { x } ( t ) , \boldsymbol { y } ( t ) , h )$ and $S _ { i }$ can be achieved as follows:

$$
\begin{array}{r l} & D _ {u s} = | | q (t) - S _ {i} | | _ {2} \\ & \qquad = \sqrt {(x (t) - x _ {i}) ^ {2} + (y (t) - y _ {i}) ^ {2} + h ^ {2}} \end{array}\tag{1}
$$

where $0 \leq t \leq T$ , and T represents the total flight time of CUAV.

## B. Wireless charging model

The far-field directional radio frequency (RF) wave [47] and electromagnetic resonance (ER) [48] are the most commonly used WPT technologies. Specifically, the former has a long transmission distance, while it has the strict requirements on the positions of transmitters. If the transmitting antenna and receiving antenna are not aligned in an RF system, the transmission efficiency of the system may be greatly affected. However, the transmission efficiency of the latter is less affected by the alignment and the transmission efficiency can reach more than 90% [23]. Thus, ER is regarded as a promising technology, and it is adopted in this work to supply energy from CUAV to SNs. Specifically, the charging efficiency $\eta$ of the ER technology is defined as follows [23]:

$$
\eta = \frac {a}{b D _ {u s} ^ {6} + c}\tag{2}
$$

where $^ { a , }$ b and c are constants that are related to the parameters of CUAV. Refer to [23], we set $a = b = c = 1$ in this work for simplicity. Thus, $D _ { u s }$ is the only variable, which is variated according to the positions of CUAV and SNs, and the specific variation range of $D _ { u s }$ will be given in Section VI.

## C. Energy consumption model of CUAV

In this paper, a multi-rotor aero-robot is adopted due to its hover function in the sky. Wu et al. [23] extends the energy consumption model of this type of CUAV into a CUAV-enabled WRSN. Specifically, the energy consumption of CUAV can be divided into propulsion energy consumption and charging energy consumption. We only need to consider the horizon propulsion energy consumption of the propulsion energy consumption because we assume that CUAV flies at the same altitude in this work. It is worth noting that the energy consumption caused by CUAV acceleration is usually ignored [23]. Accordingly, the propulsion energy consumption model of CUAV can be expressed as follows[49]:

$$
E _ {p r o} (T) = \int_ {0} ^ {T} P _ {p r o} (| | v (t) | |) d t\tag{3}
$$

where $T$ is the flight time, $v ( t )$ is the speed of CUAV, and $P _ { p r o }$ is the propulsion power which can be calculated as follows:

$$
\begin{array}{c} P _ {p r o} (v) = P _ {0} (1 + \frac {3 v ^ {2}}{U _ {t i p} ^ {2}}) + P _ {i} (\sqrt {1 + \frac {4 v ^ {4}}{4 v _ {0} ^ {4}}} - \frac {v ^ {2}}{2 v _ {0} ^ {2}}) ^ {1 / 2} \\ + \frac {1}{2} d _ {0} \rho s A v ^ {3} \end{array}\tag{4}
$$

where $P _ { 0 }$ and $P _ { i }$ are the two constant parameters for the blade profile power and induced power at the hovering status, respectively. $U _ { t i p }$ is the tip speed of the rotor blade, $v _ { 0 }$ is the mean rotor induced velocity when hovering, $d _ { 0 }$ and s represent the fuselage drag ratio and rotor solidity, respectively, and $\rho$ and A are the air density and rotor disc area, respectively. Assume that CUAV flight speed is fixed, then the propulsion power of CUAV can be divided into the moving power and hovering power, respectively. The moving power $P _ { m o v e }$ is expressed as follows:

$$
P _ {m o v e} = P _ {p r o} (v (t) = v _ {m o v e})\tag{5}
$$

where $v _ { m o v e }$ is the moving speed of CUAV. Moreover, the hovering power $P _ { h o v e r }$ is expressed as follows:

$$
P _ {h o v e r} = P _ {p r o} (v (t) = 0)\tag{6}
$$

If we assume that the total CUAV movement time and the total CUAV hovering time are $t _ { m }$ and $t _ { h }$ , respectively, then the total energy consumption of CUAV can be rewritten as follows:

$$
E _ {t o t a l} (t _ {m}, t _ {h}) = P _ {m o v e} t _ {m} + (P _ {h o v e r} + P _ {s}) t _ {h}\tag{7}
$$

where $P _ { s }$ represents the transmission power from CUAV to SNs.

## IV. PROBLEM STATEMENT

In this section, JSTOP for improving the performance of the WRSN system is formulated and analyzed.

## A. Problem formulation

In this work, we propose to use a CUAV to charge all SNs in WRSN. Specifically, CUAV flies at the fixed altitude and it is assumed the time for charging each SN is the same due to low energy storage capacity of each SN. Moreover, CUAV needs to hover at a point for charging a set of SNs within its effective charging range and then moves to the next position one by one until all SNs are charged, which can extend the lifetime of WRSN. Note that a CUAV can charge multi SNs at each hovering point if these SNs are within its effective charging range. However, there may be three challenges for CUAV in the abovementioned charging process. First, the number of the hovering points of CUAV will be huge, especially for a large-scale network with large number of SNs, which may increase the hovering energy consumption of CUAV so that further affecting the endurance time of CUAV. Second, some of SNs may be repeatedly charged, as shown in Fig. 1, which may waste some charging power so that decreasing the total charging efficiency. Finally, CUAV needs to fly over every prior determined hovering point to charging all SNs, and thus how to select flight order and trajectory will directly affect the flying distance as well as the propulsion energy consumption of CUAV.

Accordingly, in the considered scenario, the ultimate objective is to improve the charging performance between CUAV and SNs in the UAV-enabled WRSN. In other words, we aim to let CUAV charge all SNs with the least hovering points and the shortest flying distance. Note that improving the overall charging performance can be directly affected by three optimization objectives as follows.

Optimization objective 1: Minimizing the number of CUAV hovering points. According to Eq. (7), the total hovering energy consumption of CUAV is linear with the total hovering time of CUAV (i.e., the total charging time) since the charging time for each SN is assumed to be the same. Thus, CUAV should select the less hovering points while covering more SNs at each hovering point so that reducing the hovering energy and time consumptions of CUAV, thereby improving the charging efficiency to a certain extent. Note that the vital constraint of this optimization objective is that the determined hovering points of CUAV should cover all SNs, i.e., each SN must be charged in each hovering point of CUAV. Therefore, we think that it is necessary to optimize the number of CUAV hovering points, and the corresponding objective function is designed as follows:

$$
f _ {1} (X _ {u 1}, Y _ {u 1},..., X _ {u k}, Y _ {u k}, k) = k\tag{8}
$$

where $( X _ { u i } , \ Y _ { u i } ) , i = 1 , \ 2 , \ 3 , \ . . . ,$ k is the two-dimensional coordinates of CUAV.

Optimization objective 2: Minimizing the number of repeatedly covered SNs. As mentioned above, the number of SNs that within the charging range of CUAV at each hovering point should be maximized to improve the charging efficiency. However, this may lead some SNs to be repeatedly charged, especially when SNs are densely distributed, which may reduce the total charging efficiency. Thus, it is reasonable to reduce the number of repeatedly covered SNs in each hovering point of CUAV, and the corresponding objective function is designed as follows:

$$
f _ {2} (X _ {u 1}, Y _ {u 1}, \dots , X _ {u k}, Y _ {u k}, k) = S _ {r c}\tag{9}
$$

where $S _ { r c }$ represents the number of repeatedly covered SNs by the different locations of CUAV, and it is expressed as follows:

$$
S _ {r c} = \sum_ {j = 1} ^ {k} \sum_ {i = 1} ^ {n} u _ {i j} - n\tag{10}
$$

where $u _ { i j }$ is a binary variable, which is designed as follows:

$$
u _ {i j} = \left\{ \begin{array}{l l} 1 & \quad | | q _ {j} - S _ {i} | | \leq d _ {m a x} \\ 0 & \quad | | q _ {j} - S _ {i} | | > d _ {m a x} \end{array} \right.\tag{11}
$$

where $q _ { j }$ is the jth hovering location.

Optimization objective 3: Minimizing the flying distance of CUAV for charging all SNs. After determining the optimal hovering points for charging, CUAV needs to fly over each prior determined hovering point to charging all SNs. Thus, how to select the flight order and plan the trajectory becomes a vital problem to reduce the total flying distance of CUAV, which directly affects the total charging performance, and the corresponding objective function is designed as follows:

$$
f _ {3} = \sum_ {a, b = 1} ^ {k} c _ {a b} d _ {a b}\tag{12}
$$

where $d _ { a b }$ is the distance between the hovering locations $q _ { a }$ and $q _ { b }$ , wherein a = 1, 2, 3, ..., k and b = 1, 2, 3, ..., k. Moreover, $c _ { a b }$ is expressed as follows:

$$
c _ {a b} = \left\{ \begin{array}{l l} 1, & \text {   If   CUAV   visits   } q _ {b} \text {   after   visiting   } q _ {a} \\ 0, & \text {   otherwise   } \end{array} \right.\tag{13}
$$

Accordingly, the abovementioned three optimization objectives are all critical to the charging performance of UAVenabled WRSN, and they are jointly considered in this work. Thus, JSTOP can be formulated as follows:

$$
\text {(JSTOP)} \min _ {\{X _ {u}, Y _ {u} \}, \{q _ {j} \}, k} f = k + S _ {r c} + \sum_ {a, b = 1} ^ {k} c _ {a b} d _ {a b}\tag{14a}
$$

$$
\mathrm{s.t.} \qquad X _ {u m i n} \leq X _ {u} \leq X _ {u m a x}\tag{14b}
$$

$$
Y _ {u m i n} \leq Y _ {u} \leq Y _ {u m a x}\tag{14c}
$$

$$
k \in \mathbb {N} ^ {+}, 2 \leq k \leq n\tag{14d}
$$

$$
n _ {c} = n\tag{14e}
$$

$$
c _ {a b} \in \{0, 1 \}\tag{14f}
$$

$$
\sum_ {a = 1} ^ {k} c _ {a b} = 1\tag{14g}
$$

$$
\sum_ {b = 1} ^ {k} c _ {a b} = 1\tag{14h}
$$

where $n _ { c }$ represents the total number of SNs that can obtain power and S represents the set of SNs.

## B. Problem conversion

The formulated JSTOP is NP-hard, and it has both continuous and discrete solution spaces, which is difficult to be solved by using conventional algorithms. Moreover, JSTOP cannot be directly solved since solving the third objective function needs to determine the hovering locations of CUAV in advance, which needs to be determined by solving the first and second objective functions. Thus, we convert the original formulated JSTOP into two sub-problems that are CSOP and CTOP, respectively. Specifically, CSOP is formulated as follows:

(CSOP)

$$
\min _ {\{X _ {u}, Y _ {u} \}, k} f _ {C S O P} = f _ {1} + f _ {2}
$$

$$
\mathrm{s.t.} X _ {u m i n} \leq X _ {u} \leq X _ {u m a x}\tag{15a}
$$

$$
Y _ {u m i n} \leq Y _ {u} \leq Y _ {u m a x}
$$

$$
n _ {c} = n\tag{15b}
$$

(15c)

$$
k \in \mathbb {N} ^ {+}, 2 \leq k \leq n\tag{15d}
$$

(15e)

Then, CTOP is formulated as follows:

(CTOP)

$$
\min _ {\{q _ {j} \}} f _ {C T O P} = f _ {3}\tag{16a}
$$

$$
\begin{array}{l l} \text {s.t.} & c _ {a b} \in \{0, 1 \} \end{array}\tag{16b}
$$

$$
\sum_ {a = 1} ^ {k} c _ {a b} = 1\tag{16c}
$$

$$
\sum_ {b = 1} ^ {k} c _ {a b} = 1\tag{16d}
$$

By the conversions above, the original formulated JSTOP can be solved by using different algorithms, which will be detailed in the following section.

## C. Problem analysis

In this section, the NP-hardness of the converted CSOP and CTOP are analyzed.

For CSOP, as shown in Eq. (15), it is constructed by the linear weighting method of $f _ { 1 }$ and $f _ { 2 }$ . For ease of analysis and without loss of generality, we only consider $f _ { 1 }$ and thus the original formulated CSOP is simplified as follows:

$$
\min _ {\{X _ {u}, Y _ {u} \}, k} f _ {1} = k\tag{17a}
$$

$$
\mathrm{s.t.} \quad n _ {c} = n\tag{17b}
$$

Definition 1 (Minimum geometric unit disk cover problem (MGUDCP)): Assume that a set of points $P = p _ { 1 } , p _ { 2 } , . . . , p _ { n }$ and a set C on the plane, and the goal is that $\cup _ { C _ { i } \in C } C _ { i }$ covers all points P . Then, the goal is to select the minimum cardinality subset $C ^ { \prime } \subseteq C$ such that each of the points in P is covered by at least one disk in $C ^ { \prime }$

Thus, the optimization problem shown in Eq. (17) above can be reduced to MGUDCP [50], which is an NP-hard problem and it is transformed from 3SATm which is a classical NP-hard problem [51]. Since the formulated CSOP is more complex than the simplified problem in Eq. (17), then the original CSOP is NP-hard.

For CTOP, as shown in the Eq. (16), it can be reduced to a traveling salesman problem (TSP), which is NP-hard [52]. Thus, CTOP is also an NP-hard problem.

Accordingly, the original formulated JSTOP is an NP-hard problem since both of CSOP and CTOP are NP-hard.

## V. ALGORITHMS

The formulated optimization problem is proven as NP-hard, which means that an algorithm cannot find the optimal solution in polynomial time. Therefore, one effective strategy is to use the evolutionary algorithms to find a feasible solution in finite iterations, which is a practical method for dealing with the considered problem. Evolutionary algorithms have many advantages in solving NP-hard problems [53]. However, the conventional PSO cannot satisfy the solving requirements of CSOP and CTOP, since the discrete optimization is included in these solutions. Therefore, we propose two different version of improved PSO algorithms for dealing with these two converted optimization problems, respectively.

## A. Conventional PSO

PSO is originally proposed to solve the continuous optimization problems [54]. Compared to other evolutionary algorithms, PSO has several advantages, e.g., fewer parameters and ease of use [55]. Moreover, PSO has a memory function, which means that the best history position of the particle population can be recorded, and these solutions can share the information with other solutions in the population. In addition, its nature of interacting particles has proved to be effective in finding the optimum in the search space. Thus, these characteristics motivated us to choose PSO as the basic algorithm structure for solving the formulated optimization problems.

In conventional PSO, each member of the group constantly changes its velocity and position by learning from its own and other members’ experiences. Let $\nu _ { d } ^ { m } = ( \nu _ { 1 } ^ { m } , \nu _ { 2 } ^ { m } , . . . , \nu _ { D } ^ { m } )$ )

and $\chi _ { d } ^ { m } = ( \chi _ { 1 } ^ { m } , \chi _ { 2 } ^ { m } , . . . , \chi _ { D } ^ { m } )$ be the velocities and positions of the mth individual, respectively, where $m = 1 , 2 , 3 , . . . , U$ and U represents the population size, and suppose that ${ P _ { b e s t _ { d } } } ^ { m } \ = \ ( P _ { b e s t _ { 1 } } { } ^ { m } , P _ { b e s t _ { 2 } } { } ^ { m } , . . . , P _ { b e s t _ { D } } { } ^ { m } )$ and $\begin{array} { r l } { G _ { b e s t _ { d } } } & { { } = } \end{array}$ $( G _ { b e s t _ { 1 } } , G _ { b e s t _ { 2 } } , . . . , G _ { b e s t _ { D } } )$ denote the local optimal solution and the global optimal solution, respectively, where $d \_ =$ $1 , 2 , 3 , . . . , D$ and $D$ is the dimension of decision variables. Then, the population update process of conventional PSO can be expressed as follows [54]:

$$
\nu_ {d} ^ {m} = w * \nu_ {d} ^ {m} + c _ {1} * r _ {1} * (P _ {b e s t _ {d}} ^ {m} - \chi_ {d} ^ {m})
$$

$$
+ c _ {2} * r _ {2} * (G _ {b e s t _ {d}} - \chi_ {d} ^ {m})\tag{18a}
$$

$$
\chi_ {d} ^ {m} = \nu_ {d} ^ {m} + \chi_ {d} ^ {m}\tag{18b}
$$

where w is the inertia weight, $c _ { 1 }$ and $c _ { 2 }$ are the learning factors, and $r _ { 1 }$ and $r _ { 2 }$ denote two random numbers produced in the range from $( 0 , 1 )$ . The main steps of conventional PSO can be found in [54].

## B. Motivation for proposing the two PSO variants

As mentioned in Section IV, the formulated JSTOP is NP-hard, and it has both continuous and discrete solution spaces, which is difficult to be solved by using conventional PSO. Moreover, JSTOP cannot be directly solved by using conventional PSO in one step since solving the third objective function needs to determine the hovering locations of CUAV in advance, which needs to be determined by solving the first and second objective functions. Thus, in this work, the original formulated JSTOP are converted into CSOP and CTOP, respectively. However, there are still several challenges for conventional PSO to deal with the converted two subproblems.

• CSOP is also a hybrid optimization problem that consists of the continuous and discrete solution space, which becomes a challenging task for conventional PSO since it is proposed for the continuous optimization problems.

• The solution dimension in CSOP is not fixed since it should be changed with the number of hovering points of CUAV, and this cannot be handled by conventional PSO.

• CTOP is a completely discrete optimization problem, which means that conventional PSO cannot be used to solve it.

• The positions of CUAV, the flying path of CUAV and the number of hovering points of CUAV are the solution to the formulated optimization problem. Thus, we must optimize $( 2 ~ \times ~ k { + } 1 )$ solution dimensions. Moreover, the number of solution dimension increases in proportion to both of the network scale and number of SNs, which means that the converted CSOP may become a largescale optimization problem. For example, if we have 1000 SNs in a WRSN, the upper limit of the dimension of the solution can reach 2001 at most, which is difficult to be solved.

Accordingly, we tried to seek for some previous works that related to PSO for finding out the suitable advanced PSO variants to solve the formulated optimization problems, which are mainly presented in Section II. However, there is no readymade method that can be directly used to solve the converted problems, which motives us to propose PSOFKP and PSOD2P, and they are detailed as follows.

## C. PSOFKP

CSOP is actually a hybrid optimization problem since it has both continuous and discrete solution spaces, which means that it cannot be directly solved by using conventional PSO. Thus, we propose a PSOFKP with three improved factors to solve the formulated CSOP, and the details are as follows.

1) Flexible dimension mechanism: In CSOP, the number of CUAV hovering points and the corresponding hovering positions are regarded as the solutions. However, the former is not preset, which means that the dimension of a solution cannot be determined. Thus, we introduce a flexible dimension mechanism to dealing with the dimension length change condition, which is detailed as follows.

Fig. 2(a) shows the basic principle of the flexible dimension mechanism. Specifically, this mechanism adds the auxiliary variables to the particles with nonmaximum dimensions to make all the particles reach the maximum dimension. These auxiliary variables are randomly generated in the search space to ensure that the algorithm works properly and do not participate in the calculation of the objective function. The description of the auxiliary variables production process is shown in Fig. 2(b).

2) K-means operator: In conventional PSO, the initial solutions are usually randomly generated. However, a high quality heuristic function can improve the search efficiency and performance of the algorithm [56]. Moreover, we find that when using the update process of the particle position shown in Eq. (18), the convergence becomes weaker as the iteration progress. Thus, the K-means operator that considers the locations of SNs is applied to initialize the solutions and the iteration process in the algorithm. Specifically, suppose that k is an integer that generated randomly from [2, n] and the decision variable can be written as $( X _ { u 1 } , Y _ { u 1 } , . . . , X _ { u k } , Y _ { u k } , k ) .$ wherein n is the number of SNs. Moreover, the K-means clustering is used to determine g centers from a group of points after minimizing the mean squared distance between each given point and the nearest center. The main steps of the K-means operator are described in [57].

In this work, the locations of SNs play roles as the given points and k is given as the number of centers. After applying the K-means operator, we can obtain a set of hovering positions, and the process is shown in Algorithm 1. Thus, the initialization process may be more efficient which can improve the performance of the algorithm, and the algorithm can obtain a better convergence rate.

3) Punishment-compensation mechanism: Due to the uncertainty of k, the algorithm may easily become trapped in the local optima, causing it to be unable to guarantee that all SNs are charged. Thus, we propose a punishment-compensation mechanism to overcome the shortcomings above. Specifically, the value of k is changed iteratively, and the process follows the two following principles:

```txt
Algorithm 1: K-means operator for determining the hovering positions of CUAV

1 Obtain the coordinates of SNs;
2 Taking the sensor coordinates as the given points, the coordinates of k cluster centers are obtained according to [57];
3 The coordinates of k cluster centers are injected into the population.
```

(a) Punishment: If k of the local optimal solution of the particle is greater than that of the global optimal solution, k decreases by $\frac { n } { 1 0 0 }$ . Otherwise, k of the global optimal solution is assigned to the particle.

(b) Compensation: If k of the local optimal solution of the particle is less than that of the global optimal solution, k increases by $\frac { n } { 1 0 0 }$ . Otherwise, k of the global optimal solution is assigned to the particle. Moreover, in order to ensure that the search step size is not too large, we provide a usage probability $\rho$ for this mechanism.

The main steps of the proposed punishment-compensation mechanism are presented in Algorithm 2. Note that rand is a random number that generated from [0, 1], k<sup>i</sup> is the k value of particle $i , k ^ { P _ { b e s t _ { i } } }$ is the k value of the local optimal solution of the particle i and $k ^ { G _ { b e s t } }$ is the k value of the global optimal solution.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 2: Punishment-compensation mechanism
1 Set the related parameters $\rho$;
2 if all SNs can obtain energy then
3    if rand &lt; $\rho$ then
4    if $k^{P_{best_i}} \geq k^{G_{best}}$ then
5    $k^i = k^i - n/100$;
6    else
7    $k^i = k^{G_{best}}$;
8    end
9    if $k^i \leq 1$ then
10    $k^i = k^i + n/100$;
11    end
12    end
13 else
14    if rand &lt; $\rho$ then
15    if $k^{P_{best_i}} \leq k^{G_{best}}$ then
16    $k^i = k^i + n/100$;
17    else
18    $k^i = k^{G_{best}}$;
19    end
20    if $k^i \geq n$ then
21    $k^i = k^i - n/100$;
22    end
23    end
24 end
</div>

Accordingly, proposed PSOFKP is described in Algorithm 3.

It is worth noting that the “<” in Algorithm 3 is different from the general meaning. Specifically, it judges whether the two results meet the same prerequisite, that is, whether they can charge all nodes or not, and then compares the corresponding values. If one result can charge all nodes and the other can not, then even if the value of the former is greater than that of the latter, the former is still less than the latter.

![](images/6882fde69ddc65e27ca564593c00661755e4b7118ec83cbf5a8d2a61ff03fcc9.jpg)  
(a)

![](images/55008a960ae440753986d055e236c91af1687be9a67211299c49ab77751d2357.jpg)  
Fig. 2. (a) The entire population after the flexible dimension mechanism. (b) The entire population with auxiliary variables.

## D. PSOD2P

Although several versions of discrete PSO algorithms have been proposed to solve the discrete optimization problems, they may be not suitable to be directly used for the formulated CTOP, and the reasons are discussed in Section V-B. Thus, a PSOD2P with three improved factors is proposed and the details are as follows.

1) Discretization improvement factor: We redefine the velocity and position, and the calculation method between them in conventional PSO as follows.

Definition 2 (Positions and velocity): Let the access sequence of UAV hovering locations be the solution of CTOP, i.e. the position in PSOD2P. The exchange pair W (A, B) is defined as the exchange of two hovering positions A and B in the solution sequence, and the ordered queue of the set of exchange pairs is the velocity. For example, assume that the solution sequence is $Q = q _ { 1 } , \ q _ { 2 } , . . . , \ q _ { i } , \ q _ { j } , \ q _ { t } , \ . . . , \ q _ { k }$ where k is the total amount of hovering. Then, $Q ^ { * } =$ $Q + W ( q _ { i } , \ q _ { j } ) \ = \ q _ { 1 } , \ q _ { 2 } , . . . , \ q _ { j } , \ q _ { i } , \ q _ { t } , \ . . . , \ q _ { k }$ . Assume that the velocity is $L ~ = ~ ( W _ { 1 } ( q _ { i } , ~ q _ { j } ) , ~ W _ { 2 } ( q _ { j } , ~ q _ { t } ) ) )$ , then $Q ^ { * } = Q + L = q _ { 1 } , \ q _ { 2 } , . . . , \ q _ { t } , \ q _ { i } , \ q _ { j } , \ . . . , \ q _ { k } .$

Definition 3 (Equivalent velocity): Different velocities acting on the same solution may produce the same new solution. These velocities are called equivalent velocity.

Definition 4 (Velocity merging): Several velocities can be combined to obtain a new velocity, and ⃝+ is defined as the merging operator of two velocities.

Definition 5 (Simplest velocity): Among all equivalent velocities, the velocity with the fewer exchange pairs is called the simplest velocity. For example, $Q ^ { * } - Q \ = \ W ( q _ { i } , \ q _ { j } )$ is a simplest velocity.

Overall, Eq. (18) can be rewritten as follows:

$$
\begin{array}{c} \nu^ {m \prime} = w * \nu^ {m \prime} \textcircled {+} r _ {1} * (P _ {b e s t} ^ {m \prime} - \chi^ {m \prime}) \\ \textcircled {+} r _ {2} * (G _ {b e s t} - \chi^ {m \prime}) \\ \chi^ {m \prime} = \nu^ {m \prime} + \chi^ {m \prime} \end{array}\tag{19a}
$$

(19b)

where w is a constant, $r _ { 1 }$ and $r _ { 2 }$ are two random numbers that are generated from $[ 0 , 1 ]$ , w ∗ ν represents all exchange pairs in simplest velocity ν reserved by probability w and $r _ { 1 } * \left( P _ { b e s t } { } ^ { m \prime } - \chi ^ { m \prime } \right)$ represents all exchange pairs in simplest velocity $( \boldsymbol { P _ { b e s t } } ^ { m \prime } - \chi ^ { m \prime } )$ reserved by probability $r _ { 1 }$ . In the same way, $r _ { 2 } * ( G _ { b e s t } - \chi ^ { m \prime } )$ represents all exchange pairs in simplest velocity $( G _ { b e s t } - \chi ^ { m \prime } )$ reserved by probability $r _ { 2 } .$ Obviously, the larger the parameters $w , r _ { 1 }$ and $r _ { 2 }$ are set, the more exchange pairs that are retained in $\nu , ( { P _ { b e s t } } ^ { m \prime } - { \chi } ^ { m \prime } )$ and $( G _ { b e s t } - \chi ^ { m \prime } )$ , respectively.

2) 2-opt operator: The 2-opt algorithm is a classic local search method that can accelerate the convergence of the algorithm and improve the accuracy of the solution, and it is widely used in the graph optimization problems [58]. The idea of the 2-opt operator is to remove two edges from a route and reconnect these edges in order to obtain a new route with a shorter total path length. Fig. 3 shows the basic principle of 2-opt operator. Specifically, the original access sequence is $A , B , C , D , E , A .$ , and after the 2-opt process, we reconnect the edge B, C and D, E. Then, the access sequence is changed to A, B, D, C, E, A. Moreover, this removal and reconnection process should be continued until no 2-opt improvement is found. The process of 2-opt operator is shown in Algorithm 4, and the details are described as follows.

In the algorithm, B is a legitimate access sequence, and $B _ { j + 1 }$ and $B _ { h }$ represent the $( j + 1 ) _ { t h }$ and $h _ { t h }$ hovering positions of CUAV, respectively. Moreover, $E [ B _ { j , j + 1 } ]$ is the

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 3: PSOFKP
1 Define the fitness function:
 $f_{CSOP}(x), x = [x^{1}, x^{2}, ..., x^{U}]$ ;
2 Set the related parameters.
3 (1) The learning factors:  $c_{1}$  and  $c_{2}$ ;
4 (2) The inertia weight: w;
5 (3) The maximum number of iterations:  $G_{max}$ ; for m=1 to U do
6 Initialize the velocity  $\nu^{m}$  and position  $\chi^{m}$  with Algorithm 1;
7 Compute  $f_{CSOP}(\chi^{m})$  and set  $P_{best}^{m} = \chi^{m}$ ;
8 end
9  $G_{best} = \min(P_{best}^{m})$ ;
10 for it=1 to  $G_{max}$  do
11 for m=1 to U do
12 Update the value of k with Algorithm 2;
13 Generate additional population position  $\Lambda^{m}$  according to the newly generated k with Algorithm 1;
14 Compute  $f_{CSOP}(\Lambda^{m})$ ;
15 Update the velocity  $\nu^{m}$  and position  $\chi^{m}$  according to Eq. (18);
16 Compute  $f_{CSOP}(\chi^{m})$ ;
17 end
18 In order to maintain the population size, the redundant particles were eliminated at position  $\Lambda$  and position  $\chi$ ;
19 for m=1 to U do
20 if  $f_{CSOP}(\chi^{m}) &lt; f_{CSOP}(P_{best}^{m})$  then
21  $P_{best}^{m} = \chi^{m}$ ;
22 end
23 if  $f_{CSOP}(P_{best}^{m}) &lt; f_{CSOP}(G_{best})$  then
24  $G_{best} = P_{best}^{m}$ ;
25 end
26 end
27 end
28 Output  $G_{best}$ .
</div>

![](images/c87ddd474b1e4e2c70cd5be942e475e67e728c8b8c75d4a26c170b05c8fc5de9.jpg)

Fig. 3. The access sequence before and after the 2-opt process.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 4: 2-opt operator
1 for j=1 to n-3 do
2    for h=j+2 to n-1 do
3    if  $E[B_{j,j+1}] + E[B_{h,h+1}] &gt;$ $E[B_{j,h}] + E[B_{j+1,h+1}]$  then
4    Swap the access sequence of  $B_{j+1}$  and  $B_{h}$ ;
5    end
6    end
7 end
</div>

![](images/03b4048678599524ad666c5535334ccd52b753817433c7d72e2a00201bdfafad.jpg)  
Fig. 4. (a) The origin path before the using of path crossover reduction mechanism. (b) The path after using the path crossover reduction mechanism.

Euclidean distance between $B _ { j }$ and $B _ { j + 1 } , \ E [ B _ { h , h + 1 } ]$ is the Euclidean distance between $B _ { h }$ and $B _ { h + 1 } , \ E [ B _ { j , h } ]$ is the Euclidean distance between $B _ { j }$ and $B _ { h }$ , and $E [ B _ { j + 1 , h + 1 } ]$ is the Euclidean distance between $B _ { j + 1 }$ and $B _ { h + 1 }$

3) Path crossover reduction mechanism: Using 2-opt operator can produce new crossover paths. Besides, it may also slow down the convergence rate of the algorithm. Thus, in this section, we introduce a path crossover reduction mechanism to solve the issues above. Specifically, path $p _ { j }$ is defined as the path composed of vertex j and vertex $j + 1$ , which is shown in Fig. 4(a). Similarly, path $p _ { h }$ is defined as the path composed of vertex h and vertex $h + 1$ , and there is an intersection between $p _ { j }$ and $p _ { h }$ , and the trajectory of CUAV optimized by reducing the crossover is shown in Fig. 4(b). According to the principle that the sum length of two sides of a triangle in a plane is greater than the third side length, it is easy to prove $p _ { j } + p _ { h } > n p _ { j } + n p _ { h }$ . Moreover, we define a function iscro whose function is to determine whether two line paths in a plane is intersected. If they do, 1 is returned; otherwise, 0 is returned. Then, the path crossover reduction mechanism can be described is Algorithm 5.

```matlab
Algorithm 5: Path crossover reduction mechanism
1 for j=1 to n-2 do
2    h = j + 2;
3    while h=j+2≤n do
4    if iscro(pj, ph) then
5    Reverse the sequence from node j + 1 to node h;
6    h = j + 2;
7    end
8    h = h + 1;
9    end
10 end
```

In the algorithm, n is the amount of hovering CUAV. Then, PSOD2P can be described in Algorithm 6.

## E. Convergence and complexity of the proposed algorithms

In this section, the convergence and complexity of the proposed PSOFKP and PSOD2P are analyzed.

1) Convergences of the proposed algorithms: Evolutionary algorithms are effective methods to solve the NP-hard problems since they are able to find a feasible solution under the limited time and resources without considering the specific form of the problem. Specifically, according to the principle, the evolutionary algorithms can find a stable suboptimal solution by utilizing continuous iterations. Moreover, the authors in [59] analyze the convergence of the evolutionary algorithms and give two lemmas on the probability of locating in the promising area and the different distance from the initial solution to the optimal solution, which proves that the evolutionary algorithm will be sure to be convergent for any tolerance whatever for unimodal or multimodal functions. Moreover, the convergence of the proposed algorithms can be further observed and verified by the simulation results in the next section.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 6: PSOD2P
1 Define the fitness function:
 $f_{CTOP}(x), x = [x^{1}, x^{2}, ..., x^{U}]$ ;
2 Set the related parameters.
3 (1) The learning factors:  $c_{1}$  and  $c_{2}$ ;
4 (2) The inertia weight: w;
5 (3) The maximum number of iterations:  $G_{max}$ ; for m=1 to U do
6 Initialize the velocity  $\nu^{m'}$  and position  $\chi^{m'}$ ;
7 Compute  $f_{CTOP}(\chi^{m'})$  and set  $P_{best}^{m'} = \chi^{m'}$ ;
8 end
9  $G_{best}' = \min(P_{best}^{m'})$ ;
10 for it=1 to  $G_{max}$  do
11 for m=1 to U do
12 Compute the simplest velocity ( $P_{best}^{m'} - x^{m}$ ) and  $G_{best}' - x^{m}$ );
13 Compute the velocity  $\nu^{m'}$  according to Eq. (19a);
14 Compute the position  $\chi^{m'}$  according to Eq. (19b) and compute  $f_{CTOP}(\chi^{m'})$ ;
15 if (mod(it,20)==0) then
16 Compute the new position  $\chi^{m''}$  according to Algorithm 4;
17 if  $f_{CTOP}(\chi^{m'}) &lt; f_{CTOP}(\chi^{m'})$  then
18  $\chi^{m'} = \chi^{m''}$ 
19 end
20 end
21 if (mod(it,20)==0) then
22 Update the position  $\chi^{m'}$  according to Algorithm 5;
23 if  $f_{CTOP}(\chi^{m'}) &lt; f_{CTOP}(P_{best}^{m'})$  then
24  $P_{best}^{m} = \chi^{m}$ 
25 end
26 if  $f_{CTOP}(P_{best}^{m'}) &lt; f_{CTOP}(G_{best})$  then
27  $G_{best}' = P_{best}^{m'}$ 
28 end
29 end
30 Update the velocity  $\nu^{m'}$  to an simplest velocity;
31 end
32 end
33 Output  $G_{best}'$ .
</div>

2) Complexities of the proposed algorithms: For PSOFKP, we assume that the maximum iteration, the number of SNs, the number of CUAV hovering points and the population size are $t , n ,$ k and $U ,$ respectively, then the computation of K-means has $O ( t \cdot n \cdot k \cdot I )$ complexity, as mentioned in [60], where I is the processing time for calculating the distance between two hovering positions. Note that I can be ignored with the increasing of the computing power, hence the computation complexity of K-means can be regarded as $O ( t \cdot n \cdot k )$ . Due to the uncertainty of k of each solution, the worst overall computation complexity of PSOFKP is $O ( U \cdot t \cdot n ^ { 2 } )$

For PSOD2P, the complexity of PSOD2P is primarily derived by the comparison of the 2-opt operator and the path crossover reduction mechanism. We assume that the solution dimension and population size are $n$ and $U ,$ respectively, then the comparison of in 2-opt operator has $O ( U \cdot n ^ { 2 } )$ computational complexity. Moreover, the path crossover reduction mechanism also has $O ( U \cdot n ^ { 2 } )$ computational complexity. Thus, the overall complexity of proposed PSOD2P is $O ( U \cdot n ^ { 2 } )$

## VI. SIMULATION RESULTS

In this section, simulations are conducted to evaluate the performance of the proposed approaches by using Matlab. We consider three simulation cases with different numbers of SNs (100 SNs for Case 1, 500 SNs for Case 2 and 1000 SNs for Case 3), and the region of WRSN is set as 500 m × 500 m. Moreover, the flight height h is 10 m, and the maximum transmission distance $d _ { m a x }$ between CUAV and SNs is $1 0 { \sqrt { 2 } }$ m. In addition, the population size and the maximum number of iterations of PSOFKP and PSOD2P are 20 and 200, respectively. Note that the simulations are repeated independently for 30 times to avoid the random bias, and the statistical optimization results are presented.

## A. Optimization of CSOP

Figs. 5(a), 5(b) and 5(c) show the scheduling results of CUAV for charging the all SNs obtained by the proposed PSOFKP in different simulation cases, while Figs. 5(d), 5(e) and 5(f) give the random scheduling results of CUAV. Note that the random scheduling means that the hovering points of CUAV are randomly determined. As can be seen, the random scheduling method cannot satisfy the constraint that all nodes should be charged.

Then, we introduce the firefly algorithm (FA) [6] and cuckoo search (CS) [61] to solve CSOP for comparison, and use the uniform scheduling method as the comparison approach. The parameters of the abovementioned comparison algorithms are given in Table I. Note that the uniform scheduling method means that the number of CUAV hovering points is fixed as 1301, and the hovering points are uniformly distributed in the area. Table II shows the corresponding numerical results of the proposed PSOFKP and the abovementioned comparison approaches. Note that “Mean”, “Std.”, “Maximum” and “Minimum” represent the mean value, standard deviation, maximum value and minimum value of the 30 independent tests, respectively. Moreover, the statistical results of the running time of

![](images/b48c3dd7ee36cf8a3050e87a20190b401d73f5639caf19dbe354ca4910b52cda.jpg)  
(a)

![](images/4ebc045f718df5312269279d497271029000fe309d319b367b6ec4bd2b969044.jpg)  
(b)

![](images/f844ec597aae96c7cf7a37893ff8a62b3615e9816c785a655ade1e4197fc3071.jpg)  
(c)

![](images/9ed2c6cd958f327934a85014be6dc878ff78af48539f39fd49bd892d86312b71.jpg)  
(d)

![](images/1b2da97b6d4a7c2a40916bc058cec1a662f1b12aa1e22bc144fc36307115cadc.jpg)  
(e)

![](images/463f0338c3056500e4fee40c558c239f618e1b5358866fa1ab5a36800b57ebe9.jpg)  
(f)  
Coverage range of different hovering positions ·SNs

Fig. 5. The optimization results of CUAV hovering positions obtained by PSOFKP and random scheduling. (a) PSOFKP for case 1. (b) PSOFKP for case 2. (c) PSOFKP for case 3. (d) Random scheduling for case 1. (e) Random scheduling for case 2. (f) Random scheduling for case 3.  
TABLE I  
PARAMETER SETUPS OF DIFFERENT ALGORITHMS

<table><tr><td>Algorithm</td><td>Values of parameters</td></tr><tr><td>FA</td><td> $\alpha = 0.5, \beta = 0.2, \gamma = 1$ </td></tr><tr><td>CS</td><td> $\beta = 3$ </td></tr><tr><td>PSOFKP</td><td> $c_1 = 2, c_2 = 2, w = 0.73, kt = 20$ </td></tr></table>

FA, CS and the proposed PSOFKP are shown in Table III. It can be seen from these tables that PSOFKP achieves the better results compared to other algorithms, while it takes the longer CPU time. This is because that the introduced improved factors of PSOFKP will undoubtedly takes some extra calculations, which may consume more time. However, the gap of running time between proposed PSOFKP and other algorithms is not very large, and the proposed CUAV-based charging method is usually performed off-line. Thus, we may say that PSOFKP has the overall best performance for dealing with converted CSOP.

Fig. 6 shows the convergence rates of PSOFKP, FA and CS. As can be seen, PSOFKP has the fastest convergence rate compared to the benchmark algorithms. The reason may be that the introduced K-means method is able to enhance the performance of conventional PSO. Specifically, this operator is embedded into the initialization process of conventional PSO to make the initial solutions to be more reasonably distributed by utilizing their prior knowledge. Moreover, this operator is also embedded into the solution update process to improve the exploitation ability of the algorithm. In addition, the punishment-compensation mechanism is helpful to update the discrete part of the solutions and enhance the diversity of the population. Thus, these operators may accelerate the convergence rate of the algorithm.

TABLE II  
NUMERICAL STATISTICAL RESULTS OBTAINED BY PSOFKP AND OTHER CONTRAST ALGORITHMS FOR SOLVING CSOP

<table><tr><td></td><td></td><td></td><td>Mean</td><td>Std.</td><td>Maximum</td><td>Minimum</td></tr><tr><td rowspan="8">Case 1</td><td rowspan="2">Uniform</td><td> $f_1$ </td><td>1301.00</td><td>0.00</td><td>1301.00</td><td>1301.00</td></tr><tr><td> $f_2$ </td><td>61.00</td><td>0.00</td><td>0.00</td><td>0.00</td></tr><tr><td rowspan="2">FA</td><td> $f_1$ </td><td>93.00</td><td>3.07</td><td>99.00</td><td>87.00</td></tr><tr><td> $f_2$ </td><td>5.27</td><td>3.67</td><td>15.00</td><td>0.00</td></tr><tr><td rowspan="2">CS</td><td> $f_1$ </td><td>92.87</td><td>2.54</td><td>98.00</td><td>88.00</td></tr><tr><td> $f_2$ </td><td>4.63</td><td>3.10</td><td>13.00</td><td>0.00</td></tr><tr><td rowspan="2">PSOFKP</td><td> $f_1$ </td><td>87.43</td><td>1.72</td><td>91.00</td><td>84.00</td></tr><tr><td> $f_2$ </td><td>0.00</td><td>0.00</td><td>0.00</td><td>0.00</td></tr><tr><td rowspan="8">Case 2</td><td rowspan="2">Uniform</td><td> $f_1$ </td><td>1301.00</td><td>0.00</td><td>1301.00</td><td>1301.00</td></tr><tr><td> $f_2$ </td><td>286.00</td><td>0.00</td><td>0.00</td><td>0.00</td></tr><tr><td rowspan="2">FA</td><td> $f_1$ </td><td>428.07</td><td>23.90</td><td>471.00</td><td>387.00</td></tr><tr><td> $f_2$ </td><td>134.70</td><td>40.62</td><td>220.00</td><td>77.00</td></tr><tr><td rowspan="2">CS</td><td> $f_1$ </td><td>435.13</td><td>21.18</td><td>476.00</td><td>382.00</td></tr><tr><td> $f_2$ </td><td>143.80</td><td>36.43</td><td>230.00</td><td>72.00</td></tr><tr><td rowspan="2">PSOFKP</td><td> $f_1$ </td><td>381.97</td><td>9.82</td><td>408.00</td><td>366.00</td></tr><tr><td> $f_2$ </td><td>67.93</td><td>8.96</td><td>93.00</td><td>45.00</td></tr><tr><td rowspan="8">Case 3</td><td rowspan="2">Uniform</td><td> $f_1$ </td><td>1301.00</td><td>0.00</td><td>1301.00</td><td>1301.00</td></tr><tr><td> $f_2$ </td><td>579.00</td><td>0.00</td><td>0.00</td><td>0.00</td></tr><tr><td rowspan="2">FA</td><td> $f_1$ </td><td>787.10</td><td>57.61</td><td>892.00</td><td>703.00</td></tr><tr><td> $f_2$ </td><td>659.30</td><td>149.37</td><td>950.00</td><td>449.00</td></tr><tr><td rowspan="2">CS</td><td> $f_1$ </td><td>789.47</td><td>41.72</td><td>895.00</td><td>708.00</td></tr><tr><td> $f_2$ </td><td>664.67</td><td>108.55</td><td>949.00</td><td>538.00</td></tr><tr><td rowspan="2">PSOFKP</td><td> $f_1$ </td><td>674.23</td><td>15.19</td><td>696.00</td><td>643.00</td></tr><tr><td> $f_2$ </td><td>389.73</td><td>24.71</td><td>428.00</td><td>346.00</td></tr></table>

TABLE III  
NUMERICAL STATISTICAL RESULTS OF CPU TIMES (S) OBTAINED BY DIFFERENT ALGORITHMS FOR SOLVING CSOP

<table><tr><td></td><td></td><td>Mean</td><td>Std.</td><td>Maximum</td><td>Minimum</td></tr><tr><td rowspan="3">Case 1</td><td>FA</td><td>4.05</td><td>0.33</td><td>5.343</td><td>3.58</td></tr><tr><td>CS</td><td>12.82</td><td>0.84</td><td>14.81</td><td>11.44</td></tr><tr><td>PSOFKP</td><td>92.35</td><td>19.91</td><td>114.59</td><td>31.67</td></tr><tr><td rowspan="3">Case 2</td><td>FA</td><td>36.93</td><td>0.10</td><td>39.38</td><td>34.78</td></tr><tr><td>CS</td><td>193.31</td><td>9.66</td><td>219.42</td><td>175.13</td></tr><tr><td>PSOFKP</td><td>608.37</td><td>147.97</td><td>798.91</td><td>120.72</td></tr><tr><td rowspan="3">Case 3</td><td>FA</td><td>193.94</td><td>2.93</td><td>201.31</td><td>188.70</td></tr><tr><td>CS</td><td>996.28</td><td>75.34</td><td>1148.53</td><td>896.39</td></tr><tr><td>PSOFKP</td><td>1630.90</td><td>158.88</td><td>1988.51</td><td>1340.73</td></tr></table>

## B. Optimization of CTOP

In this section, the comparison algorithms for solving the formulated CTOP are introduced in detail, and the optimization results are presented.

1) Introduction to comparison algorithms: The artificial bee colony (ABC) [52] [62], genetic algorithm (GA) [63] [41], differential evolution (DE) [64] [65], simulated annealing (SA) [66] [67] and tabu search (TS) [68] [69] are introduced as the comparison algorithms with PSOD2P for solving the converted CTOP. Specifically, how are these abovementioned algorithms applied to solve the investigated problems are presented to make a clearer comparison.

All of these algorithms are evolutionary algorithms, and thus they have the similar algorithm structure. In CTOP, the solutions are the trajectory of CUAV for approaching each hovering point, which means that the solution space is with discrete values. Note that conventional ABC, GA, DE, SA and TS may solve the discrete optimization problems by using the mapping mechanism, and the details are as follows.

ABC: ABC algorithm is inspired by the foraging behavior of bees. In ABC algorithm, the colony of artificial bees is divided into three groups: employed bees, onlookers and scouts. Half of the colony consists of the employed bees, and another half consists of the onlookers. The position of a food source corresponds to a possible solution to the optimization problem, and the nectar amount of each food source represents their quality (fitness) of the associated solution. The number of the employed bees equals to the number of food sources. When a food source has been abandoned by bees, the abandoned employed bee would become a scout, and the main steps of ABC for solving formulated CTOP are as follows. First, ABC needs to generate the initial solutions and the population randomly, and then evaluates the objective function values of these solutions, wherein the initial solutions are the access sequences of CUAV hovering positions. After the initialization, the population of solutions is subject to repeated cycles of the search courses of employed bees, onlookers and scouts. Second, the algorithm enters the employed bee phase. Specifically, for each food source, the algorithm exchanges two access sequences of CUAV hovering positions in the food source randomly to generate new food source and retain the better food source according to the greedy principle. When all employed bees complete the search process, they will share the information about nectar amounts and positions of food sources with onlookers. An onlooker evaluates the nectar information which is owned by all employed bees, and then chooses a food source with a probability which is related to the nectar amount. As in the case of the employed bee, the onlooker can produce a modification on the position in its memory and check the nectar amount of the candidate source. If the nectar amount is more than that of the previous one, the bee would memorize the new position and forgets the previous one. An onlooker chooses a food source completely depending on the probability value associated with the food source $\tau _ { i } ,$ which is calculated as follows:

$$
\tau_ {i} = \xi_ {i} / \sum_ {j = 1} ^ {U} \xi_ {j}\tag{20}
$$

where $\xi _ { i }$ denotes the fitness value of the ith solution which is proportional to the nectar amount of the food source in the ith position and U denotes the population size. Third, the food source which is abandoned by the bees would be replaced with a new food source that found by the scouts. In ABC algorithm, the foraging behavior is simulated by randomly producing a food source and replacing the abandoned one with a new one. If a position cannot be further improved through a predetermined number of cycles, the food source should be abandoned. The predetermined number of cycles is an important control parameter in ABC algorithm, which is called limit for abandonment. The generation process of new food source can be described as exchanging two access sequences of CUAV hovering positions in the old food source randomly. If the current iteration time is less than the maximum iteration, the algorithm executes from the second step to the third step for a loop.

GA: A solution to the formulated optimization problem will be regarded as a chromosome of GA, and the main steps of GA for solving formulated CTOP are as follows. First, GA needs to randomly generate the initial solutions and population, and then evaluates the objective function values of these solutions. Similar to ABC, the solutions of GA are regarded as the access sequences of CUAV hovering positions. Second, for each chromosome, the algorithm should execute the follow operators: (a) Crossover: Two parent individuals are selected to crossover by roulette wheel which are recorded as $Z _ { 1 }$ and $Z _ { 2 } ,$ , respectively. If the generated random number is less than the crossover probability, the selected gene fragment from $Z _ { 1 }$ will be retained in the offspring chromosome, and the length of the gene fragment is s, where $s ~ = ~ l e n g t h ( Z ) / 2 ~ - ~ 1$ and $l e n g t h ( Z )$ represents the length of the chromosome. The starting position of the gene fragment is randomly generated in the first half of the chromosome of the offspring chromosome, and the rest gene of the offspring chromosome is generated according to $Z _ { 2 }$ . Specifically, the offspring chromosome starts from the position after the end of the gene fragment and puts the gene corresponding to $Z _ { 2 }$ if the gene is not appeared in offspring chromosome until the length of the offspring chromosome is equal to that of the parent chromosomes. It is worth noted that the chromosome is a cycle sequence, which means that the next gene will become the first one if the gene is the last one in its chromosome. (b) Mutation: If the generated random number is less than the crossover probability, the algorithm needs to exchange two genes in chromosome randomly to generate a new chromosome. It is worth noted that every node of the access sequences of CUAV hovering positions is regarded as gene in GA. If the current iteration time is not satisfied with the requirement of the maximum iteration, then the algorithm should execute the crossover and mutation operator for every chromosome iteratively.

![](images/c97f09ddfd6071d692cc455386be9b96862b5c9668e9ba41292c559faee29c7b.jpg)  
(a)

![](images/2941bffe01dd4406f511d4b9772c34e2fdf2796e320c235f2e3b3a21ef645c24.jpg)  
(b)

![](images/0784fbd957c64410e4d1aaa6ec886c1319448295fc1521bfd758ee2645433582.jpg)  
(c)  
Fig. 6. Convergence rates obtained by PSOFKP. (a) Case 1. (b) Case 2. (c) Case 3.

DE: Based on the natural selection idea of survival of the fittest, DE has the characteristics of self-learning and selfadaptive to a certain extent. At the start of the evolution, the perturbations are large since parent populations are far away. As the evolutionary process matures, the population converges to a small region and the perturbations adaptively become small. An individual in DE represents a solution to the formulated optimization problem and the main steps of DE for solving formulated CTOP are as follows. First, DE needs to generate the initial solutions and the population randomly, and then evaluates the objective function values of these solutions. However, the solution of conventional DE is the real numbers in search space in the initialization, while the solution to the formulated optimization problem is the access sequence of the hovering positions. Thus we transform the real numbers to the sequence by sorting in order to calculate the objective function. The order of magnitude of real numbers in an individual is the access order of CUAV. Second, do mutation, crossover, selection as conventional DE recurrently until the current iteration reaches the maximum iteration. It is worth noted that we compute the objective function values by the method that transform the real numbers to the sequence by sorting. The main steps of DE is shown in [64].

SA: A solution to the formulated optimization problem will be regarded as a particle of SA, and the main steps of SA for solving CTOP are as follows. First, SA needs to generate U initial solution, set the search times in every temperature as L and the cooling rate as $q ,$ then evaluates the objective function value, wherein U is the population size. The method of generating the new solutions in SA is to swap the access order of two CUAV hovering positions. Then, for each individual in the population, SA needs to generate L new solutions according this method. Third, the algorithm calculates the objective function values of the new generated solutions and the corresponding increments between new solutions and initial solution. If the new solutions are better than the current optimal solution, then the new solutions should be retained, or retain the new solution according to the probabilities [67]. Meanwhile, the temperature reduction process is always accompanied by the algorithm, just like conventional SA.

TS: TS is a memory-based search strategy that allows the local search process to proceed beyond local optima. This is achieved by allowing the objective function to deteriorate when the current solution is a local optimum, and by keeping track of recent moves or solutions in a so-called tabu list. Whenever the algorithm attempts to move to a solution or to perform a move recorded in the tabu list, the move is banned. This rule prevents cycling and forces other solutions to be explored. The main steps of TS for solving CTOP are as follows. First, the TS algorithm needs to generate one initial solution randomly, set the tabu list to empty set, and give the maximum number of candidate solutions and the tabu length as $U$ and Ψ, respectively, where the solution is the access sequence of CUAV hovering positions. Second, for each candidate solution, the algorithm exchanges two access sequences of the initial solution to generate U new candidate solutions. Third, TS updates the current optimal solution, the tabu list and the solution to the next iteration process. To avoid falling into local optimum in the search process, this movements will be recorded in the tabu list. Specifically, the tabu list records the access sequence that has been exchanged and the tabu length that limits the length of exchanging action. If the coming exchanging movement has been recorded in tabu list, the algorithm will choose the optimal movement which has not been record in tabu list. Finally, if the algorithm does not meet the termination condition, execute the second and the

TABLE IV  
PARAMETER SETUPS OF DIFFERENT ALGORITHMS

<table><tr><td>Algorithm</td><td>Values of parameters</td></tr><tr><td>ABC</td><td>Limit = 200</td></tr><tr><td>GA</td><td>pc = 0.9, pm = 0.9</td></tr><tr><td>DE</td><td>F = 0.6, CR = 0.5</td></tr><tr><td>SA</td><td>T0 = 500, L = 200, q = 0.98</td></tr><tr><td>TS</td><td>ψ = 200</td></tr><tr><td>PSOD2P</td><td>c1 = 0.8, c2 = 0.8, w = 1</td></tr></table>

third step iteratively.

Moreover, the key parameter settings of the abovementioned comparison algorithms are given in Table IV.

2) Optimization results of CTOP: Figs. 7(a), 7(b) and 7(c) show the randomly generated trajectories for CUAV for case 1, case 2 and case 3, respectively, while Figs. 7(d), 7(e) and 7(f) show the trajectories of CUAV optimized by using proposed PSOD2P. Obviously, the flight distance of CUAV is significantly reduced by PSOD2P. Moreover, the numerical statistical results and the running time obtained by different algorithms for solving CTOP are shown in Tables V and VI, respectively, and the definitions of the “Mean”, “Std.”, “Maximum” and “Minimum” have the same with meanings with CSOP case. Similar to CSOP case, the running time of PSOD2P increases more than that of other comparison algorithms as the dimension increasing, and the reason may be that PSOD2P has more cyclic search in decision variables. In addition, Fig. 8 reveals the convergence rates of different algorithms for solving CTOP. As can be seen, the proposed PSOD2P achieves the best performance in terms of the convergence rate and accuracy.

TABLE V  
NUMERICAL STATISTICAL RESULTS OBTAINED BY PSOD2P AND OTHER COMPARISON ALGORITHMS FOR SOLVING CTOP

<table><tr><td></td><td></td><td>Mean</td><td>Std.</td><td>Maximum</td><td>Minimum</td></tr><tr><td rowspan="6">Case 1</td><td>ABC</td><td>12376.32</td><td>425.57</td><td>13181.90</td><td>11498.25</td></tr><tr><td>GA</td><td>9091.24</td><td>483.78</td><td>10109.88</td><td>8096.96</td></tr><tr><td>DE</td><td>18326.82</td><td>440.96</td><td>19244.45</td><td>17424.32</td></tr><tr><td>SA</td><td>5723.19</td><td>288.88</td><td>6476.33</td><td>5122.58</td></tr><tr><td>TS</td><td>8944.10</td><td>321.54</td><td>9522.97</td><td>8287.20</td></tr><tr><td>PSOD2P</td><td>4608.16</td><td>67.07</td><td>4733.52</td><td>4427.93</td></tr><tr><td rowspan="6">Case 2</td><td>ABC</td><td>69275.63</td><td>818.78</td><td>70452.06</td><td>67569.59</td></tr><tr><td>GA</td><td>55795.94</td><td>1131.89</td><td>58408.71</td><td>53144.74</td></tr><tr><td>DE</td><td>85934.94</td><td>960.91</td><td>88387.79</td><td>83188.24</td></tr><tr><td>SA</td><td>28090.84</td><td>1161.00</td><td>31100.04</td><td>25921.37</td></tr><tr><td>TS</td><td>50733.53</td><td>1085.51</td><td>53364.74</td><td>48715.45</td></tr><tr><td>PSOD2P</td><td>10758.48</td><td>124.50</td><td>10917.75</td><td>10427.79</td></tr><tr><td rowspan="6">Case 3</td><td>ABC</td><td>136892.35</td><td>1235.45</td><td>138836.39</td><td>132516.02</td></tr><tr><td>GA</td><td>112420.97</td><td>2069.67</td><td>116193.32</td><td>107805.58</td></tr><tr><td>DE</td><td>157734.59</td><td>1407.61</td><td>160045.53</td><td>154587.73</td></tr><tr><td>SA</td><td>56525.65</td><td>1036.81</td><td>58599.25</td><td>54665.08</td></tr><tr><td>TS</td><td>106623.92</td><td>1638.90</td><td>109542.78</td><td>103468.69</td></tr><tr><td>PSOD2P</td><td>15317.24</td><td>116.32</td><td>15557.45</td><td>15116.52</td></tr></table>

We further conduct tests to verify the effectiveness of the introduced improved factors of PSOD2P. Since CTOP is a discrete problem, the discretization improvement factor is necessary for this problem. Thus, in this test, the optimization results, i.e., the total flight distances of CUAV obtained by PSO with a discretization improvement factor and a 2-opt operator (PSOD2), PSO with a discretization improvement factor and a path crossover reduction mechanism (PSODP) and PSOD2P are presented, respectively. Note that these tests are also independently run for 30 times in each case to prevent the random bias. Fig. 9(a), 9(b) and 9(c) show the corresponding test results for cases 1, 2 and 3, respectively. As can be seen, compared to conventional PSO, the discretization improvement factor provides a prerequisite for the solution of CTOP, and the 2-opt operator improves the convergence rate of the algorithm. Moreover, the path crossover reduction mechanism also helps to improve the convergence rate of the algorithm. Accordingly, the combination of these three improved factors can improve the performance of the algorithm, and the reasons are as follows. The generation method of new solutions in traditional methods is random, which may cause some newly generated solutions are worse than the original ones due to the extensive solution space, and the proposed path crossover reduction mechanism can provide a reliable method to enhance the quality of the new solutions. Specifically, there are two cases that need to be considered. First, if the original path does not have any crossover, the quality of the new solution is the same as that of the original one. Second, if the original path contains at least one crossover, the quality of the new solution is better than the original one. However, this mechanism may fall into premature convergence. Thus, the proposed 2-opt operator is able to make the algorithm to jump out from local optima. Accordingly, these two operators may make PSOD2P to be superior for deal with CTOP.

TABLE VI  
NUMERICAL STATISTICAL RESULTS OF RUNNING TIMES (S) OBTAINED BY PSOD2P AND OTHER COMPARISON ALGORITHMS FOR SOLVING CTOP

<table><tr><td></td><td></td><td>Mean</td><td>Std.</td><td>Maximum</td><td>Minimum</td></tr><tr><td rowspan="6">Case 1</td><td>ABC</td><td>1.58</td><td>0.11</td><td>2.05</td><td>1.50</td></tr><tr><td>GA</td><td>0.19</td><td>0.08</td><td>0.56</td><td>0.13</td></tr><tr><td>DE</td><td>0.48</td><td>0.05</td><td>0.68</td><td>0.42</td></tr><tr><td>SA</td><td>8.35</td><td>0.05</td><td>8.47</td><td>8.27</td></tr><tr><td>TS</td><td>0.60</td><td>0.12</td><td>1.20</td><td>0.53</td></tr><tr><td>PSOD2P</td><td>3.83</td><td>0.12</td><td>4.25</td><td>3.60</td></tr><tr><td rowspan="5">Case 2</td><td>ABC</td><td>56.62</td><td>0.31</td><td>57.34</td><td>56.00</td></tr><tr><td>GA</td><td>0.81</td><td>0.16</td><td>1.61</td><td>0.72</td></tr><tr><td>DE</td><td>1.36</td><td>0.04</td><td>1.48</td><td>1.29</td></tr><tr><td>SA</td><td>14.39</td><td>0.06</td><td>14.54</td><td>14.32</td></tr><tr><td>PSOD2P</td><td>117.65</td><td>1.37</td><td>122.50</td><td>115.83</td></tr><tr><td rowspan="6">Case 3</td><td>ABC</td><td>262.25</td><td>4.72</td><td>281.11</td><td>258.89</td></tr><tr><td>GA</td><td>1.90</td><td>0.27</td><td>3.28</td><td>1.75</td></tr><tr><td>DE</td><td>1.89</td><td>0.09</td><td>2.12</td><td>1.68</td></tr><tr><td>SA</td><td>20.65</td><td>0.09</td><td>20.94</td><td>20.48</td></tr><tr><td>TS</td><td>3.79</td><td>0.33</td><td>4.58</td><td>3.48</td></tr><tr><td>PSOD2P</td><td>516.58</td><td>1.90</td><td>521.36</td><td>512.58</td></tr></table>

## C. Stability tests of the proposed algorithms

In this section, trials are conducted to evaluate the stability of the proposed algorithm. Specifically, we use PSOFKP and PSOD2P and other comparison algorithms to solve the converted CSOP and CTOP independently for 30 times, respectively, and the results are shown in Figs. 10 and 11. As can be seen, both of the proposed two algorithms can achieve the overall best stability performance on solving the corresponding optimization problems, and this can be also reflected in Tables II and V, respectively.

![](images/bd965b6d36a83b578936e32f7d43251b908eea0cceab75e34c6916fbe5bcadff.jpg)  
(a)

![](images/28ac7b59650c843e3a01584ce09b09e08682c9f434f899fbf28e55c4acaffc37.jpg)  
(b)

![](images/585442b12e87dc3a63e6d7fe73f745c2d36f7193b719145107673d1cfdb61401.jpg)  
(c)

![](images/4d027a8f77d1a860f5b1092761e359f1ae81dbad7ed88a75997d63233183828e.jpg)  
(d)

![](images/c04467ba8705c299c7c5cb28ec02ef2dc605b215b98d046992d898b737747e12.jpg)  
(e)

![](images/e0caa5e91dd212d723ac7fdaf1a8d04e344d6436a229ccb6f2b3834dcef624bc.jpg)  
(f)  
Paths Hovering positions

Fig. 7. The original path and the path optimized by PSOD2P. (a) The original path for case 1. (b) The original path for case 2. (c) The original path for case 3. (d) The path optimized by PSOD2P for case 1. (e) The path optimized by PSOD2P for case 2. (f) The path optimized by PSOD2P for case 3.  
![](images/eb065177640353ff8942c58d518208158394aa19edb5bf26669ff739e02c8cbb.jpg)  
(a)

![](images/fbc48de04b83b953f55ac92e0baf3b6c45adb234986ee00c62f2dc2f74991f87.jpg)  
(b)

![](images/d7148c3853315c10386fb987b14c0a4a2dfde2bb569b070a297bdb601947f365.jpg)  
(c)  
Fig. 8. Convergence rates of different algorithms for solving CTOP in different cases. (a) Case 1. (b) Case 2. (c) Case 3.

## VII. CONCLUSION

In this paper, the joint scheduling and trajectory optimization of CUAV for improving the charging efficiency is investigated. First, we consider to use a CUAV to charge all SNs in WRSN. Specifically, we formulate a JSTOP to jointly minimize the hovering points of CUAV, the number of repeatedly covered SNs and the flying distance of CUAV for charging all SNs. Then, due to the NP-hardness and complex hybrid solution space of JSTOP, we divided this problem into CSOP and CTOP, respectively, and propose a PSOFKP algorithm and a PSOD2P algorithm to solve the converted subproblems. Simulation results demonstrate the superiority of the proposed joint optimization approach. Specifically, PSOFKP and PSOD2P have the overall best performance in comparison with some other benchmark algorithms, and the effectiveness of the introduced improved factors are evaluated. Moreover, it is demonstrated that both of the proposed two PSO-based algorithms are stable for solving the converted optimization problems. The results of this paper can be further extended by considering more network structure and different CUAV flight altitude, which will be investigated as the future work.

![](images/a09527ca74e37f61e4a0dca4d70aedc622bf6d1d59b657c359318f0db64c40fe.jpg)  
(a)

![](images/60ead15d591751fe37d2e36dda1ba4c67a52bed55429726763d8ceed803a1ffa.jpg)  
(b)

![](images/1f449ef373c52d7a39a8a8bbbce8524179c0348277cc99fafc08a0d3f17a2c4e.jpg)  
(c)

Fig. 9. Effectiveness verification for the improved factors of PSOD2P in different cases. (a) Case 1. (b) Case 2. (c) Case 3.  
![](images/c4a31a064434eaca4cb754c687bb924cf81818d273a67782a2ea7c6f7d8cb0eb.jpg)  
(a)

![](images/0722b077ebf462e2a4275272de88bb02b2ab2c1d1f7dd798ba6643cbe02fb9c4.jpg)  
(b)

![](images/9f1f40757ffd962bf89c946d71285fc41b003dbae7c45dbda4d035f4fcb76471.jpg)  
(c)

Fig. 10. The stability of different algorithms for solving CSOP. (a) Case 1. (b) Case 2. (c) Case 3.  
![](images/a00099966ccf3a1960bb8497c82e6472dda9d99061df51e9d7928caa542e3f96.jpg)  
(a)

![](images/d484d014b3b85a9a4c02cccd196fe9b1465051b7a44a9e6c05808823a3f772a4.jpg)  
(b)

![](images/a22700d3e4a6c0664af6be9618395d284d00100ec69cd5175d0a663fd4fb7b4e.jpg)  
(c)  
Fig. 11. Stability tests of different algorithms for solving CTOP in different cases. (a) Case 1. (b) Case 2. (c) Case 3.

## REFERENCES

[1] J. Baek, S. I. Han, and Y. Han, “Energy-efficient UAV routing for wireless sensor networks,” IEEE Transactions on Vehicular Technology, vol. 69, no. 2, pp. 1741–1750, 2020.

[2] Q. Ju, G. Sun, H. Li, and Y. Zhang, “Collaborative in-network processing for internet of battery-less things,” IEEE Internet of Things Journal, vol. 6, no. 3, pp. 5184–5195, 2019.

[3] G. Sun, Y. Liu, S. Liang, Z. Chen, A. Wang, Q. Ju, and Y. Zhang, “A sidelobe and energy optimization array node selection algorithm for collaborative beamforming in wireless sensor networks,” IEEE Access, vol. 6, pp. 2515–2530, 2018.

[4] J. Yick, B. Mukherjee, and D. Ghosal, “Wireless sensor network survey,” Computer Networks, vol. 52, no. 12, pp. 2292–2330, 2008.

[5] Q. Ju and Y. Zhang, “Charge redistribution-aware power management for supercapacitor-operated wireless sensor networks,” IEEE Sensors Journal, vol. 16, no. 7, pp. 2046–2054, 2016.

[6] S. Li, A. Wang, G. Sun, and L. Liu, “Improving charging performance for wireless rechargeable sensor networks based on charging UAVs: a

joint optimization approach,” in 2020 IEEE Symposium on Computers and Communications (ISCC), 2020.

[7] K. B. Amir, I. J. Tani, S. Sarwar, K. M. A. Salam, and H. A. Rahman, “A scheduling method for multiple target coverage to prolong lifetime of wireless sensor networks,” in 2013 2nd International Conference on Advances in Electrical Engineering (ICAEE), 2013, pp. 176–180.

[8] C. Lin, Z. Shang, W. Du, J. Ren, L. Wang, and G. Wu, “Codoc: A novel attack for wireless rechargeable sensor networks through denial of charge,” in IEEE INFOCOM 2019 - IEEE Conference on Computer Communications, 2019, pp. 856–864.

[9] Q. Ju and Y. Zhang, “Clustered data collection for internet of batteryless things,” IEEE Internet of Things Journal, vol. 4, no. 6, pp. 2275–2285, 2017.

[10] C. Lin, Z. Yang, H. Dai, L. Cui, L. Wang, and G. Wu, “Minimizing charging delay for directional charging,” IEEE/ACM Transactions on Networking, pp. 1–16, 2021, early access, doi: 10.1109/TNET.2021.3095280.

[11] C. Lin, F. Gao, H. Dai, J. Ren, L. Wang, and G. Wu, “Maximizing

charging utility with obstacles through fresnel diffraction model,” in IEEE INFOCOM 2020 - IEEE Conference on Computer Communications, 2020, pp. 2046–2055.

[12] C. Lin, Y. Zhou, F. Ma, J. Deng, and G. Wu, “Minimizing charging delay for directional charging in wireless rechargeable sensor networks,” in IEEE INFOCOM 2019 - IEEE Conference on Computer Communications, 2019.

[13] P. Cheng, S. He, F. Jiang, Y. Gu, and J. Chen, “Optimal scheduling for quality of monitoring in wireless rechargeable sensor networks,” IEEE Transactions on Wireless Communications, vol. 12, no. 6, pp. 3072– 3084, 2013.

[14] Y. Zeng, R. Zhang, and T. J. Lim, “Wireless communications with unmanned aerial vehicles: Opportunities and challenges,” IEEE Communications Magazine, vol. 54, no. 5, pp. 36–42, 2016.

[15] Q. Ju and Y. Zhang, “Predictive power management for internet of battery-less things,” IEEE Transactions on Power Electronics, vol. 33, no. 1, pp. 299–312, 2018.

[16] Q. Ju, B. Kim, C.-W. Lin, and S. Shiraishi, “Edge-assisted data transmission for connected vehicles,” Jan. 26 2021, US Patent 10,904,891.

[17] C. Lin, Z. Wang, J. Deng, L. Wang, and G. Wu, “mTS: Temporal-and spatial-collaborative charging for wireless rechargeable sensor networks with multiple vehicles,” in IEEE INFOCOM 2018 - IEEE Conference on Computer Communications, 2018.

[18] S. Geng, Y. Liu, Y. Meng, A. Wang, and Z. Ying, “Charging nodes deployment optimization in wireless rechargeable sensor network,” in Globecom IEEE Global Communications Conference, 2018.

[19] C. Lin, J. Zhou, C. Guo, H. Song, G. Wu, and M. S. Obaidat, “TSCA: A temporal-spatial real-time charging scheduling algorithm for ondemand architecture in wireless rechargeable sensor networks,” IEEE Transactions on Mobile Computing, vol. 17, no. 1, pp. 211–224, 2018.

[20] I. Salah, T. Hafid, B. Abdelmajid, E. A. Ahmed, and F. Jaouad, “Design of a new antenna structure for an RF energy recovery system on a UAV,” in 2019 7th Mediterranean Congress of Telecommunications (CMT), 2019, pp. 1–4.

[21] Y. Spyridis, T. Lagkas, P. Sarigiannidis, and J. Zhang, “Rule-based autonomous tracking of RF transmitter using a UAV swarm,” in 2020 12th International Symposium on Communication Systems, Networks and Digital Signal Processing (CSNDSP), 2020, pp. 1–3.

[22] S. Saab, A. Eid, N. Kouzayha, J. Costantine, Z. Dawy, G. Virone, and F. Paonessa, “UAV-enabled RF sensor wake-up,” in 2018 IEEE Conference on Antenna Measurements Applications (CAMA), 2018, pp. 1–3.

[23] P. Wu, F. Xiao, C. Sha, H. Huang, and L. Sun, “Trajectory optimization for UAVs’ efficient charging in wireless rechargeable sensor networks,” IEEE Transactions on Vehicular Technology, vol. 69, no. 4, pp. 4207– 4220, 2020.

[24] Q. Ju, H. Li, and Y. Zhang, “Power management for kinetic energy harvesting IoT,” IEEE Sensors Journal, pp. 1–1, 2018.

[25] L. Liu, “A downlink coverage scheme of tethered UAV,” in 2020 International Wireless Communications and Mobile Computing (IWCMC), 2020, pp. 685–691.

[26] L. Ruan, J. Wang, J. Chen, Y. Xu, Y. Yang, H. Jiang, Y. Zhang, and Y. Xu, “Energy-efficient multi-UAV coverage deployment in UAV networks: A game-theoretic framework,” China Communications, vol. 15, no. 10, pp. 194–209, 2018.

[27] B. Liu, Y. Zhang, S. Fu, and X. Liu, “Reduce UAV coverage energy consumption through actor-critic algorithm,” in 2019 15th International Conference on Mobile Ad-Hoc and Sensor Networks (MSN), 2019, pp. 332–337.

[28] D.-G. Seo, C.-H. Jeong, Y.-S. Choi, J.-S. Park, Y.-Y. Jeong, and W.-S. Lee, “Wide beam coverage dipole antenna array with parasitic elements for UAV communication,” in 2019 IEEE International Symposium on Antennas and Propagation and USNC-URSI Radio Science Meeting, 2019, pp. 1147–1148.

[29] N. W. Najeeb and C. Detweiler, “UAV based wireless charging of sensor networks without prior knowledge,” in 2018 IEEE/RSJ International Conference on Intelligent Robots and Systems (IROS), 2018, pp. 3151– 3158.

[30] J. Xu, K. Zhu, and R. Wang, “RF aerially charging scheduling for UAV fleet : A Q-learning approach,” in 2019 15th International Conference on Mobile Ad-Hoc and Sensor Networks (MSN), 2019, pp. 194–199.

[31] W. Chen, S. Zhao, Q. Shi, and R. Zhang, “Resonant beam chargingpowered UAV-assisted sensing data collection,” IEEE Transactions on Vehicular Technology, vol. 69, no. 1, pp. 1086–1090, 2020.

[32] L. Yang, Z. Su, H. Yang, Z. Na, and F. Yan, “An efficient charging algorithm for UAV-aided wireless sensor networks,” in 2020 IEEE 6th

International Conference on Computer and Communications (ICCC), 2020, pp. 834–838.

[33] X. Zhu, J. Li, and M. Zhou, “Target coverage-oriented deployment of rechargeable directional sensor networks with a mobile charger,” IEEE Internet of Things Journal, vol. 6, no. 3, pp. 5196–5208, 2019.

[34] B. Salamat and A. M. Tonello, “A modelling approach to generate representative UAV trajectories using PSO,” in 2019 27th European Signal Processing Conference (EUSIPCO), 2019, pp. 1–5.

[35] D. Yang, Q. Dan, L. Xiao, C. Liu, and L. Cuthbert, “An efficient trajectory planning for cellular-connected UAV under the connectivity constraint,” China Communications, vol. 18, no. 2, pp. 136–151, 2021.

[36] D. Ebrahimi, S. Sharafeddine, P.-H. Ho, and C. Assi, “Autonomous UAV trajectory for localizing ground objects: A reinforcement learning approach,” IEEE Transactions on Mobile Computing, vol. 20, no. 4, pp. 1312–1324, 2021.

[37] A. Yh, A. Sz, L. A. Ye, and B. Rg, “Trajectory design and power control in legitimate unmanned aerial vehicle monitoring networks,” Physical Communication, 2021, early access, doi: https://doi.org/10.1016/j.phycom.2021.101281.

[38] L. Shen, N. Wang, and X. Mu, “Iterative UAV trajectory optimization for physical layer secure mobile relaying,” in 2018 International Conference on Cyber-Enabled Distributed Computing and Knowledge Discovery (CyberC), 2018.

[39] P. Wu, F. Xiao, H. Huang, and R. Wang, “Load balance and trajectory design in multi-UAV aided large-scale wireless rechargeable networks,” IEEE Transactions on Vehicular Technology, vol. 69, no. 11, pp. 13 756– 13 767, 2020.

[40] K. Gao, Z. Cao, L. Zhang, Z. Chen, Y. Han, and Q. Pan, “A review on swarm intelligence and evolutionary algorithms for solving flexible job shop scheduling problems,” IEEE/CAA Journal of Automatica Sinica, vol. 6, no. 4, pp. 904–916, 2019.

[41] J. Tang, G. Liu, and Q. Pan, “A review on representative swarm intelligence algorithms for solving optimization problems: Applications and trends,” IEEE/CAA Journal of Automatica Sinica, vol. 8, no. 10, pp. 1627–1643, 2021.

[42] J. Zhang, X. Zhu, Y. Wang, and M. C. Zhou, “Dual-environmental particle swarm optimizer in noisy and noise-free environments,” IEEE Transactions on Cybernetics, pp. 1–11, 2018.

[43] X. Shao, W. Liu, Q. Liu, and C. Zhang, “Hybrid discrete particle swarm optimization for multi-objective flexible job-shop scheduling problem,” International Journal of Advanced Manufacturing Technology, vol. 67, no. 9-12, pp. 2885–2901, 2013.

[44] J. J. Liang, A. K. Qin, P. N. Suganthan, and S. Baskar, “Comprehensive learning particle swarm optimizer for global optimization of multimodal functions,” IEEE Transactions on Evolutionary Computation, vol. 10, no. 3, pp. 281–295, 2006.

[45] Y. Cao, H. Zhang, W. Li, M. Zhou, Y. Zhang, and W. A. Chaovalitwongse, “Comprehensive learning particle swarm optimization algorithm with local search for multimodal functions,” IEEE Transactions on Evolutionary Computation, vol. 23, no. 4, pp. 718–731, 2019.

[46] S. Cui and D. S. Weile, “Application of a parallel particle swarm optimization scheme to the design of electromagnetic absorbers,” IEEE Transactions on Antennas & Propagation, vol. 53, no. 11, pp. 3616– 3624, 2005.

[47] A. Pacini, F. Benassi, D. Masotti, and A. Costanzo, “Design of a miniaturized omni-directional RF-to-DC IR-WPT,” in 2018 IEEE Wireless Power Transfer Conference (WPTC), 2018, pp. 1–4.

[48] A. Torrisi and D. Brunelli, “Magnetic resonant coupling wireless power transfer for lightweight batteryless UAVs,” in 2020 International Symposium on Power Electronics, Electrical Drives, Automation and Motion (SPEEDAM), 2020, pp. 751–756.

[49] Y. Zeng, J. Xu, and R. Zhang, “Energy minimization for wireless communication with rotary-wing UAV,” IEEE Transactions on Wireless Communications, vol. 18, no. 4, pp. 2329–2345, 2018.

[50] G. K. Das, R. Fraser, A. Loopez-Ortiz, and B. G. Nickerson, “On the´ discrete unit disk cover problem.” International Journal of Computational Geometry & Applications, vol. 22, no. 05, pp. 1 250 009–, 2012.

[51] D. S. Johnson, “The NP-completeness column: An ongoing guide,” Journal of Algorithms, vol. 8, no. 3, pp. 438–448, 1987.

[52] N. Pathak, M. Mishra, and S. P. S. Kushwah, “Improved local search based modified ABC algorithm for TSP problem,” in 2017 4th International Conference on Electronics and Communication Systems (ICECS), 2017, pp. 173–178.

[53] T. Zheng, Y. Liu, G. Sun, S. Liang, J. Han, Q. Ju, and S. Li, “Joint sidelobe suppression and nulls control of large-scale linear antenna array using particle swarm optimization with global search and population

mutation,” International Journal of Numerical Modelling: Electronic Networks, Devices and Fields, vol. 33, no. 4, p. e2710, 2020.

[54] J. Kennedy and R. Eberhart, “Particle swarm optimization,” in Proceedings of ICNN’95 - International Conference on Neural Networks, vol. 4, 1995, pp. 1942–1948 vol.4.

[55] M. Thida, H. L. Eng, D. N. Monekosso, and P. Remagnino, “A particle swarm optimisation algorithm with interactive swarms for tracking multiple targets,” Applied Soft Computing, vol. 13, no. 6, pp. 3106– 3117, 2013.

[56] Y. F. Yiu, J. Du, and R. Mahapatra, “Evolutionary heuristic a\* search: Heuristic function optimization via genetic algorithm,” in 2018 IEEE First International Conference on Artificial Intelligence and Knowledge Engineering (AIKE), 2018, pp. 25–32.

[57] T. Kanungo, D. M. Mount, N. S. Netanyahu, C. D. Piatko, R. Silverman, and A. Y. Wu, “An efficient K-means clustering algorithm: analysis and implementation,” IEEE Transactions on Pattern Analysis and Machine Intelligence, vol. 24, no. 7, pp. 881–892, 2002.

[58] X. Chen, Y. Zhou, Z. Tang, and Q. Luo, “A hybrid algorithm combining glowworm swarm optimization and complete 2-opt algorithm for spherical travelling salesman problems,” Applied Soft Computing, p. S1568494617302399, 2017.

[59] X. Zhao, “Convergent analysis on evolutionary algorithm with nonuniform mutation,” in Evolutionary Computation, 2008.

[60] D. Stiawan, M. E. Suryani, Susanto, M. Y. Idris, M. N. Aldalaien, N. Alsharif, and R. Budiarto, “Ping flood attack pattern recognition using a K-means algorithm in an internet of things (IoT) network,” IEEE Access, vol. 9, pp. 116 475–116 484, 2021.

[61] S. Liang, Z. Fang, G. Sun, and J. Zhang, “A physical layer security approach based on optical beamforming for indoor visible light communication,” IEEE Communications Letters, vol. 24, no. 10, pp. 2109– 2113, 2020.

[62] G. Tian, Y. Ren, Y. Feng, M. Zhou, H. Zhang, and J. Tan, “Modeling and planning for dual-objective selective disassembly using and/or graph and discrete artificial bee colony,” IEEE Transactions on Industrial Informatics, vol. 15, no. 4, pp. 2456–2468, 2018.

[63] Z. Tao, “TSP problem solution based on improved genetic algorithm,” in 2008 Fourth International Conference on Natural Computation, vol. 1, 2008, pp. 686–690.

[64] T. J. Choi and W. A. Chang, “An adaptive cauchy differential evolution algorithm with bias strategy adaptation mechanism for global numerical optimization,” in The 6th International Conference on Computer Research and Development.

[65] Y. Yang, S. Gao, Y. Wang, and Y. Todo, “Global optimum-based search differential evolution,” IEEE/CAA Journal of Automatica Sinica, vol. 6, no. 02, pp. 46–61, 2019.

[66] J. Bi, H. Yuan, S. Duanmu, M. Zhou, and A. Abusorrah, “Energyoptimized partial computation offloading in mobile-edge computing with genetic simulated-annealing-based particle swarm optimization,” IEEE Internet of Things Journal, vol. 8, no. 5, pp. 3774–3785, 2021.

[67] M. M. Keikha, “Improved simulated annealing using momentum terms,” in International Conference on Intelligent Systems, 2011.

[68] N. Yang, X. Ma, and P. Li, “An improved angle-based crossover tabu search for the larger-scale traveling salesman problem,” in 2009 WRI Global Congress on Intelligent Systems, vol. 1, 2009, pp. 584–587.

[69] X. Zuo, B. Li, X. Huang, M. Zhou, C. Cheng, X. Zhao, and Z. Liu, “Optimizing hospital emergency department layout via multiobjective tabu search,” IEEE Transactions on Automation Science and Engineering, vol. 16, no. 3, pp. 1137–1147, 2019.
