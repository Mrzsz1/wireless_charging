---
title: "Placing Wireless Chargers With Multiple Antennas"
year: 2024
venue: "IEEE Transactions on Mobile Computing"
doi: "10.1109/TMC.2023.3338563"
source_type: paper
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: 2026-07-10
canonicalized_at: 2026-07-10
ingest_status: ingested
---

# Placing Wireless Chargers With Multiple Antennas

Haipeng Dai , Senior Member, IEEE, Yikang Zhang , Weijun Wang , Member, IEEE, Rong Gu , Member, IEEE, Yuben Qu , Chi Lin , Senior Member, IEEE, Lijie Xu , Member, IEEE, Jiaqi Zheng , Senior Member, IEEE, Wanchun Dou , Member, IEEE, and Guihai Chen , Fellow, IEEE

Abstract—Charger placement is an important problem in improving the quality of service in wireless rechargeable sensor networks. This paper studies the problem of Wireless ChArger PlacemeNt with Multiple (Directional) Antennas (WANDA). The problem is described as follows: given a set of wireless chargers equipped with multiple directional antennas and a set of wireless rechargeable sensors, determine the chargers’ positions and orientations to maximize the overall charging utility. According to the relative positional relationship between the antennas, the problem is classified into Relative Orientation Fixed (WANDA-ROF) and Relative Orientation Unfixed (WANDA-ROU) situations. To address WANDA, we present a piecewise constant function to approximate the nonlinearity of charging power and propose an area discretization technique to reduce the infinite solution space to a limited one without performance loss. Then, we prove the monotonic submodularity of WANDA, and present a $\frac { \mathbf { 1 } } { \mathbf { 2 } } - \epsilon$ approximation algorithm for the ROF situation and a $\frac { \mathbf { 1 } } { \mathbf { 6 } } - \epsilon$ approximation algorithm for the ROU situation, all run in polynomial time. Finally, we conduct extensive simulation and experiments to show that our algorithms outperform comparison algorithms by at least 16% for ROF situation and 12% for ROU situation.

Index Terms—Combinatorial optimization, directional wireless charging network, multiple antennas.

## I. INTRODUCTION

W <sup>IRELESS</sup> <sup>Power</sup> <sup>Transfer</sup> <sup>(WPT)</sup> <sup>technology,</sup> <sup>which</sup>was first proposed by Nikola Tesla in the 19th was first proposed by Nikola Tesla in the 19th

Manuscript received 27 December 2022; revised 9 September 2023; accepted 15 November 2023. Date of publication 4 December 2023; date of current version 7 May 2024. This work was supported in part by the National Natural Science Foundation of China under Grants 62272223, U22A2031, 61872178, and 61832005, in part by the Collaborative Innovation Center of Novel Software Technology and Industrialization, Nanjing University, and in part by Jiangsu High-level Innovation and Entrepreneurship (Shuangchuang) Program. This work was supported in part by the National Natural Science Foundation of China under Grants 62072254, 62072303, 62172069, and 62172206. Recommended for acceptance by K. R. Chowdhury. (Corresponding authors: Haipeng Dai; Rong Gu; Jiaqi Zheng; Guihai Chen.)

This article has supplementary downloadable material available at https://doi.org/10.1109/TMC.2023.3338563, provided by the authors.

Digital Object Identifier 10.1109/TMC.2023.3338563 century [1], has been widely considered as one of the most promising techniques to deliver energy for end devices in recent years. Benefited from its notable features of portability and low cost, WPT technology has been successfully applied in many practical scenarios, such as mobile services [2], [3], electric vehicles [4], [5], [6], [7], [8], electronic portable devices [9], [10], [11], RFID [12], [13], [14], and satellite communications [15], [16], [17]. Network lifetime is an important issue in Wireless Sensor Networks (WSNs) [18], [19], [20], and the optimization of sensor placement can prolong it [21], [22]. It’s reported that in 2030, shipments of wireless charging transmitters and receivers are forecast to grow to 4.7 billion units [23].

![](images/6d3eab1fb077b3b83d1556a67b95f4b5c0c4c59fc27daa438101c014dc9c63a2.jpg)  
Fig. 1. Multi-antenna charging model.

As a representative of far-field WPT applications [24], Wireless Rechargeable Sensor Networks (WRSNs) provide a promising way to address the energy limitation problem of traditional WSNs. In WRSNs, one or many wireless chargers are deployed in the networks [25] and provide energy to surrounding sensors by emitting electromagnetic waves. There are mainly two types of wireless charging: omnidirectional charging and directional charging. In omnidirectional charging, chargers transfer power equally to all directions whose model is always built as a disk. In directional charging, chargers transfer power directionally whose model is always built as a sector. Directional charging concentrates electromagnetic power to a specific sector region which enhances charging efficiency. However, it provides zero power when sensors are deployed out of this region.

In order to trade off the efficiency and equity of wireless charging, some wireless chargers are equipped with multiple antennas emitting electromagnetic waves to different directions simultaneously [26]. In multi-antenna wireless charger networks, each charger is equipped with multiple directional antennas emitting electromagnetic waves to different directions simultaneously, as depicted in Fig. 1. To realize effective charging, the multiantenna charger and the sensor should be located in the coverage area of each other and face directly with each other. In Fig. 1,

Chi Lin is with the School of Software, Dalian University of Technology, Dalian, Liaoning 116024, China (e-mail: c.lin@dlut.edu.cn).

only sensor $o _ { 1 }$ can receive nonzero charging power from charger $s _ { 1 }$ since its position and orientation can match with $s _ { 1 }$

In this paper, we study the problem of the Wireless ChArger PlacemeNt with Multiple (Directional) Antennas (WANDA) [27], that is, given a set of wireless rechargeable sensors, and a set of multi-antenna wireless chargers, whose Relative Orientations of antennas can be Fixed (WANDA-ROF) or Unfixed (WANDA-ROU), determining the position and multiple orientations of each charger such that the overall charging utility is maximized. In WANDA-ROF, the angle between any two adjacent orientations of the charger is constant, and once the position and one of the orientations are fixed, all the other orientations are fixed, while WANDA-ROU is the opposite. To the best of our knowledge, we are the first to consider the placement problem of wireless chargers with multiple (directional) antennas. Though some existing works consider the placement problem of directional chargers [28], [29], [30], [31], all of them focus on single-directional chargers. Besides, while some existing works concern the multi-antenna chargers [32], [33], [34], [35], none of them considers the placement problem. These prior works cannot be applied to our problem.

Much evidence motivates us to concentrate on wireless charging with multiple antennas: (1) The multi-directional antennas are more efficient than omnidirectional antennas. As mentioned in [36], by concentrating power toward IoT devices, the energy efficiency of the mobile chargers increases when using a directional antenna instead of an omnidirectional counterpart. (2) The single-directional antennas have a rather narrow beamwidth, while integrating multiple antennas can alleviate it. (3) Rotating these antennas will make the charging progress more configurable and flexible, especially when chargers are mobile. For example, using multiple-directional antennas can charge multiple IoT devices simultaneously while maintaining their energy efficiency [36]. (4) There exists an increasing number of research, products, and applications with multiple antennas/beams. These emerging devices and applications require effective deployment and schedule strategies. For instance, WattUp NF-230 [37] employs a transmitter that splits feeds for multiple antennas. It consists of multiple small antennas (tens to hundreds of them) arranged in a two-dimensional array. In addition, DA2223 [38] is a power receiver chip that can utilize multiple antennas to increase received power for applications with a larger area. In [39], the authors argued that by combining the emitting strength of multiple antennas in a triangulation-like mechanism, one could create a relatively small spherical bubble of high enough power density without exceeding any radiation limits and without “contaminating” the spectrum outside the bubble.

There are mainly three technical challenges to address our problem. First, the charging power function in our model is nonlinear and the positions of all the chargers are continuous, which result in infinite searching space. Second, the coverage interrelationship between a multi-antenna charger and a sensor is very complicated, since the orientations of multiple antennas of each charger must be taken into consideration simultaneously. Third, even if the searching space could be limited, we also need to check all the combinations of each charger’s position and orientations, which results in high computational complexity.

To address the first challenge, we propose a piecewise constant function to approximate the nonlinear relationship between charging power and charging distance, by which we can partition the plane into multiple subareas to place chargers. To address the second challenge, we propose the notion of Maximum Coverage Set (MCS) to limit the possible positions and orientations of a charger. To address the third challenge, we prove that our problem falls into the realm of maximizing a monotone submodular function subject to a partition matroid constraint, which can be solved by approximation greedy algorithms. Therefore, our proposed algorithms WANDA-ROF and WANDA-ROU achieve an approximation ratio of nearly $\begin{array} { l } { { \frac { 1 } { 2 } } } \end{array}$ and nearly $\frac { 1 } { 6 }$ , respectively.

## II. RELATED WORK

Many studies have been conducted on wireless charging, focusing on various objectives such as network models, Wireless Power Transfer (WPT) techniques, system design issues, and performance metrics [40], [41], [42]. In terms of charger mobility, current works can be categorized into dynamic de ployment [43], [44], [45], [46], [47], [48] and static deployment [49], [50], [51]. Regarding static deployment, the study in [49] proposes to deploy a set of wireless chargers (Powercast [52]) in a square area and optimized the transmission strategies to maximize the energy harvested by stationary energy harvesters. Besides, for dynamic deployment, the Powercast wireless charger is mounted on a moving robot in the work [43]. This approach allows the mobile wireless charger to adjust the transmission patterns of stationary sensors while in motion. In the study [47], the authors explored a scenario where a charger is combined with a self-propelled vehicle to achieve flexible charger deployment. They addressed the problem of dynamic charger path selection and designed four meta-heuristic algorithms for Internet-of-Things (IoTs) applications. In the context of Wireless Rechargeable Sensor Networks (WRSNs) based on industrial IoTs, the study in [48] investigates the joint sensor activation and mobile charging vehicle scheduling. Furthermore, some researchers have proposed the use of Unmanned Aerial Vehicles (UAVs) for wireless power transfer [53], [54]. In terms of adopted methodologies, current works can be categorized into two main categories: rule-based solutions [43], [50], [55] and meta-heuristic-based solutions [47], [56], [57], [58]. For example, Dai et al. [50] designed a polynomial-time algorithm to detect whether a target area achieves omnidirectional charging. Kaushik et al. [56] presented a PSO-based (Particle Swarm Op timization) algorithm to address the deterministic deployment issue for point coverage in WSNs. In the work [57], a constrained Pareto-based multi-objective evolutionary method is proposed as a solution to the deterministic deployment problem in WSNs. This approach models the connectivity requirement as a constraint to ensure full connectivity between each sensor node and the base station. Additionally, the work [58] employs a genetic algorithm to guarantee the coverage and connectivity of a given collection of targets.

Next, we will provide a detailed introduction to the literatures most relevant to our work, concerning the directional wireless charger networks, the wireless charger placement, and the multiantenna wireless networks.

Directional Wireless Charger Networks: Many works about wireless charging have been done by adopting directional charging model. In this model, the coverage area of charger is modeled as a sector. However, all the existing works suppose that each charger is only equipped with one directional antenna, which are different from ours. Lin et al. adopted an anisotropic model and aimed to minimize the charging delay by adjusting the direction of the mobile charger in mobile charging scenarios [43], [59]. Dai et al. proposed the notion of omnidirectional charging for directional charger networks and investigated how to detect whether a given area satisfies omnidirectional charging [50]. Besides, Dai et al. also considered the task scheduling problem of directional wireless charger networks and studied how to optimize the charging utility by scheduling the orientations of chargers [60].

Wireless Charger Placement: Placement of wireless chargers is a fundamental problem in wireless charger networks and has been widely researched. Dai et al. aimed to optimize the overall expected charging utility by determining the positions and orientations of all the chargers simultaneously [28], [29]. They also considered the case where the chargers have some limited mobility [61] and the case where the probabilistic electromagnetic radiation safety needs to be guaranteed [62]. Wang et al. studied the deployment problem of heterogeneous wireless chargers and their objective is maximizing the charging utility [30]. The problem settings in these two papers are close to ours, while they only considered chargers with one directional antenna, which makes their solutions unable to be adapted to address our problem. In [63], Yu et al. studied the placement problem with chargers under connectivity constraints, that is, the placed chargers are all connected. Moreover, Zhang et al. dedicated to maximize the charging quality and investigated the charger placement and power allocation problem simultaneously [64]. Lee et al. proposed an energy-efficient adaptive directional charging algorithm, which can adaptively choose single- or multi-charging by considering the density of sensors [65]. Yu et al. studied a directional charger scheduling problem in mobile WRSNs, and whether the sensor can receive the energy from the charger is determined by the position and orientation of the charger, which is different from our model [66]. He et al. investigated how to deploy readers in a network to ensure that the WISP tags can harvest sufficient energy for continuous operation, including two forms of the problem, named point provisioning and path provisioning, respectively [67]. To sum up, all existing works regarding charger placement are not suitable for our problem due to the setting of multiple directional antennas.

Multi-Antenna Wireless Networks: Though there are some existing works considering the wireless charger with multiple directional antennas, to the best of our knowledge, the placement problem has not been studied yet. Lee et al. [36] tried to prolong the battery lifetime of Internet-of-Things devices by using mobile chargers (MCs). They proposed a multi-directional MC scheme that exploits multi-directional beams to reduce the charging delay while maintaining the advantages of the directional antenna with a higher charging efficiency. They also studied the problem of multi-charging [65], which is interpreted as wireless chargers emitting power for multiple nodes simultaneously.

Zhang et al. investigated a MIMO broadcast problem in SWIPT systems where all the transmitters and receivers are equipped with multiple directional antennas and they aimed to maximize the information rate and received power rate for receivers [32]. In [33] and [34], Liu et al. and Yang et al. dedicated to maximize the throughput of SWIPT systems by determining the time allocation of both uplink and downlink of energy beamforming. All of them supposed that the wireless chargers have fixed positions, and are not suitable for our problem. Besides, there are some existing works regarding the connectivity problem in wireless sensor networks with each sensor equipped with multiple directional antennas. In [68], Tran et al. solved both the antenna orientation problem and the power assignment problem with a symmetric connectivity constraint with constant-factor approximation ratios. In [69] and [70], Bhattacharya et al. and Dobrev et al. both studied the strong connectivity problem of wireless networks of sensors using multiple directional antennas. Khodamoradi et al. studied the downlink transmission problem in massive MIMO with the support of SWIPT system, which is based on power-splitting scheme and their objective is to maximize the energy efficiency of the system by joint system-level optimization [71]. In their models, the number of directional antennas of sensor is limited and the proposed solution is mainly based on geometric analysis, which are very different from ours. To sum up, all existing works are not suitable for our problem since the placement of multi-antenna chargers has not been considered.

## III. MODEL AND PROBLEM STATEMENT

## A. Network Model

Suppose that there are <sup>N</sup> heterogeneous multi-antenna wireless chargers $S = \{ s _ { 1 } , . . . , s _ { N } \}$ on a <sup>2D</sup> plane <sup>Ω</sup>. Each charger has multiple directional antennas to launch electromagnetic waves towards different directions. We denote the number of types of chargers as $R \left( R \leq N \right)$ , and the number of chargers in the <sup>r</sup>-th type as $N _ { r }$ . Also, there are <sup>M</sup> heterogeneous directional wireless rechargeable sensors $O = \{ o _ { 1 } , . . . , o _ { M } \}$ . Each sensor has a fixed position and orientation and needs to be powered by wireless chargers continuously in order to maintain normal operations. By abuse of notation, $s _ { i }$ and $o _ { j }$ also represent the positions of the charger $s _ { i }$ and the sensor $o _ { j } ,$ , respectively. Some of the notations frequently used in this paper are listed in Table I.

## B. Multi-Antenna Charging Model

Since the multiple antennas of a charger are relatively independent, the multi-antenna charger can be regarded as multiple single-directional chargers located at the same position. For each orientation, we adopt the practical directional charging model proposed in [28], [29], [50], whose power supply area of charger and power receiving area of rechargeable sensor are both approximated as sectors. We denote the $B _ { i }$ orientations of charger $s _ { i }$ as $\theta _ { i } = \{ \theta _ { i } ^ { 1 } , . . . , \theta _ { i } ^ { B _ { i } } \}$ , the corresponding $B _ { i }$ charging angles as $A _ { s _ { i } } = \{ A _ { s _ { i } } ^ { 1 } , . . . , A _ { s _ { i } } ^ { B _ { i } } \}$ and the corresponding $B _ { i }$ charging distances as $D _ { i } = \{ D _ { i } ^ { 1 } , . . . , D _ { i } ^ { B _ { i } } \}$ . A three-directional charger $s _ { 1 }$ is shown in Fig. 1, whose orientations, charging angles, and charging distances are respectively $\theta _ { 1 } ^ { 1 } , \theta _ { 1 } ^ { 2 } , \theta _ { 1 } ^ { 3 } , \bar { A } _ { s _ { 1 } } ^ { \bar { 1 } } , \bar { A } _ { s _ { 1 } } ^ { 2 } , A _ { s _ { 1 } } ^ { 3 }$ $D _ { s _ { 1 } } ^ { 1 } , \bar { D _ { s _ { 1 } } ^ { 2 } }$ and $D _ { s _ { 1 } } ^ { 3 }$ . The power receiving area of the sensor $o _ { j }$ is also modeled as a sector and we denote the orientation of $o _ { j }$ as $\phi _ { j }$ and the receiving angle of $o _ { j }$ as $A _ { o _ { j } }$ . Special to note that the coverage areas of the same charger can be overlapped.

TABLE I NOTATIONS

<table><tr><td>Symbol</td><td>Description</td></tr><tr><td>M</td><td>Number of wireless rechargeable sensors</td></tr><tr><td>N</td><td>Number of wireless chargers</td></tr><tr><td>R</td><td>Number of types of chargers</td></tr><tr><td> $N_r$ </td><td>Number of the r-th type of chargers</td></tr><tr><td> $s_i$ </td><td>Wireless charger i, or its position</td></tr><tr><td> $o_j$ </td><td>Wireless rechargeable sensor j, or its position</td></tr><tr><td> $B_i$ </td><td>Number of directional antennas for charger  $s_i$ </td></tr><tr><td> $A_{s_i}^b$ </td><td>Charging angle of b-th orientation for charger  $s_i$ </td></tr><tr><td> $D_i^b$ </td><td>Charging distance of b-th orientation for charger  $s_i$ </td></tr><tr><td> $A_{o_j}$ </td><td>Receiving angle of sensor  $o_j$ </td></tr><tr><td> $\theta_i^b$ </td><td>b-th orientation of charger  $s_i$ </td></tr><tr><td> $\phi_j$ </td><td>Orientation of sensor  $o_j$ </td></tr><tr><td>P(·)</td><td>Charging power function</td></tr><tr><td>U(·)</td><td>Charging utility function</td></tr><tr><td> $P_{th}$ </td><td>Threshold for charging utility function</td></tr><tr><td> $P_j$ </td><td>Total received power of sensor  $o_j$ </td></tr><tr><td> $U_j(\cdot)$ </td><td>Charging utility function of sensor  $o_j$ </td></tr><tr><td> $\alpha_{ij},\beta_{ij}$ </td><td>Constants in the charging model</td></tr><tr><td> $\widetilde{P}(.)$ </td><td>Approximated charging power function</td></tr><tr><td> $(s_i,\theta_i)$ </td><td>Strategy of charger  $s_i$ </td></tr><tr><td> $\mathcal{O}_i$ </td><td>Omnidirectional coverage set of charger  $s_i$ </td></tr><tr><td> $h_X(\cdot)$ </td><td>Charging utility function with given position set X</td></tr></table>

The relative orientations of a charger can be either fixed or unfixed, namely, the angle between two orientations $\langle \theta _ { i } ^ { b _ { 1 } } , \theta _ { i } ^ { b _ { 2 } } \rangle$ for charger $s _ { i }$ is a constant or a variable where $b _ { 1 } , b _ { 2 } \in$ $\{ 1 , 2 , . . . , B _ { i } \}$ , named as Relative Orientation Fixed situation of WANDA (WANDA-ROF) and Relative Orientation Unfixed situation of WANDA (WANDA-ROU).

The sensor receives nonzero power from a charger if and only if they are located in the coverage area of each other and face directly with each other. As depicted in Fig. 1, the sensor $o _ { 1 }$ can receive nonzero charging power from the charger $s _ { 1 }$ while $s _ { 2 }$ cannot. The charging power of sensor $o _ { j }$ from the <sup>b</sup>-th orientation of the charger $s _ { i }$ is

$$
\begin{array}{l} P (s _ {i}, \theta_ {i} ^ {b}, o _ {j}, \phi_ {j}) \\ = \left\{ \begin{array}{l l} \frac {\alpha_ {i j}}{(| | s _ {i} o _ {j} | | + \beta_ {i j}) ^ {2}}, & 0 \leq | | s _ {i} o _ {j} | | \leq D _ {i} ^ {b}, \\ \overrightarrow {o _ {j} s _ {i}} \cdot \overrightarrow {r _ {\phi_ {j}}} - \| o _ {j} s _ {i} \| c o s (A _ {o _ {j}} / 2) \geq 0, \\ \text { and } \overrightarrow {s _ {i} o _ {j}} \cdot \overrightarrow {r _ {\theta_ {i} ^ {b}}} - \| s _ {i} o _ {j} \| c o s (A _ {s _ {i}} ^ {b} / 2) \geq 0, \\ 0, & \text { otherwise }, \end{array} \right. \end{array}\tag{1}
$$

where $\alpha _ { i j }$ and $\beta _ { i j }$ are two known constants depending on the magnetic environment and the charger’s hardware parameters, and $| | s _ { i } o _ { j } | |$ is the distance between $s _ { i }$ and $o _ { j }$ . Although there are other electronic and signal parameters such as antenna gain, frequency, wireless channel propagation factor, environmental exponential coefficient, super additive effect, or cancellation effects [72], [73], [74], we can reasonably simplify these parameters and give an empirical power formula, and the derivation can be referred to the work in [67]. In addition, we assume that the charging power is additive [75]. More specifically, when a device $s _ { j }$ is charged by multiple chargers/antennas, the received power of $s _ { j }$ is the sum of the received power from all the chargers. The property of direct addition of power can be referred to [67], which is derived from the experiments in Section IV-B of [67]. In the experiments, two readers (power transmitters) are placed facing each other and a WISP tag (power receiver) is put in the middle between them. The distance between the tag and either reader varies from 0.6 m to 1.2 m in increments of 0.1 m. The authors found that the relative error between the sum of the individual recharge power and the joint recharge power is negligible, especially when the distance from the power transmitters is not too small. Thus, they concluded that the wireless recharge power received by a WISP tag from multiple readers is additive, which can be also applied to our wireless charging model. The above theory is also proved in Section III in [41], where Friis’s free space equation-based omnidirectional WPT model is simplified to a form just like (1). Although there are many other charging models, such as vector model [76], [77], [78], we still use the model in (1) as it already describes our charging system well.

Based on (1), the total received power of sensor $o _ { j }$ is

$$
P _ {j} = \sum_ {i = 1} ^ {N} \sum_ {b = 1} ^ {B _ {i}} P (s _ {i}, \theta_ {i} ^ {b}, o _ {j}, \phi_ {j}).\tag{2}
$$

## C. Charging Utility Model

In practice, the charging power cannot exceed the capacity of sensors. Thus, the charging utility of a sensor is first proportional to its received power and then stays constant [28], [29], [50], [60]. The charging utility function of sensor $o _ { j }$ is obtained as

$$
U _ {j} (x) = \left\{ \begin{array}{l l} \frac {1}{P _ {t h}} \cdot x, & x \leq P _ {t h}, \\ 1, & x > P _ {t h}, \end{array} \right.\tag{3}
$$

where $P _ { t h }$ is a given threshold determined by hardware design. Then, we define the overall charging utility as the normalized sum of the charging utility of all the sensors, which is

$$
U = \frac {1}{M} \sum_ {j = 1} ^ {M} U _ {j} \left(\sum_ {i = 1} ^ {N} \sum_ {b = 1} ^ {B _ {i}} P (s _ {i}, \theta_ {i} ^ {b}, o _ {j}, \phi_ {j})\right).\tag{4}
$$

## D. Problem Formulation

Given a set of sensors, our objective is to determine the chargers’ positions and the antennas’ orientations to maximize the overall charging utility. Formally, the Wireless ChArger PlacemeNt with Multiple (Directional) Antennas (WANDA)

problem is defined as follows.

$$
\begin{array}{l l} \text {(P1)} & \max \quad \frac {1}{M} \sum_ {j = 1} ^ {M} U _ {j} \left(\sum_ {i = 1} ^ {N} \sum_ {b = 1} ^ {B _ {i}} P (s _ {i}, \theta_ {i} ^ {b}, o _ {j}, \phi_ {j})\right), \\ & s. t. \quad s _ {i} \in \Omega , 0 \leq \theta_ {i} ^ {b} <   2 \pi , \\ & b \in \{1, 2, \ldots , B _ {i} \}, i \in \{1, 2, \ldots , N \}, \end{array}
$$

where the decision variables are the position $s _ { i }$ and the orientations $\theta _ { i } = \{ \theta _ { i } ^ { 1 } , . . . , \theta _ { i } ^ { B _ { i } } \}$ , the combination of which we name as the strategy of charger, denoted by $( s _ { i } , \theta _ { i } )$

We have the following theorem for WANDA.

Theorem 1: The WANDA problem is NP-hard.

Please refer to our supplemental material for the proof, where we relate WANDA to a classic NP-hard problem [79]. Note that we do not consider other metrics regarding charging networks or sensor networks, such as energy loss factors [80], end-to-end energy transfer efficiency [81], and throughput of wireless sensor networks [82]. Besides, we can adjust the chargers to a specific frequency (e.g:, 915 MHz) to make the charging process coexist with the wireless sensor networks without conflict.

## IV. SOLUTION FOR WANDA-ROF

In this section, we present the solution for WANDA-ROF. Since the angle between different orientations of charger is constant, once one orientation is fixed and then all the remaining orientations are fixed. To address this problem, we first propose an area discretization technique by adopting a piecewise constant function to approximate the nonlinear relationship between charging power and charging distance. By doing this, we partition the plane into multiple subareas and limit the number of positions for each type of chargers. Then, we introduce the notion of Maximum Coverage Set (MCS) to explore the coverage relationship between charger and sensor. After that, we propose a MCS extraction method which transforms WANDA-ROF to the problem of maximizing a monotone submodular function subject to a partition matroid constraint. Finally, we prove that our solution for WANDA-ROF achieves an approximation ratio of $\frac { 1 } { 2 } - \epsilon$

## A. Area Discretization

In this subsection, we aim to limit the number of positions of all the chargers. Let $\begin{array} { r } { P ^ { i j } ( d ) = \frac { \alpha _ { i j } } { ( d + \beta _ { i j } ) ^ { 2 } } } \end{array}$ denote the charging power in terms of the distance <sup>d</sup> between charger $s _ { i }$ and sensor $o _ { j }$ . We present a piecewise constant function to approximate the original charging power function as

$$
\widetilde {P ^ {i j}} (d) = \left\{ \begin{array}{l l} P ^ {i j} (l (1)), & d = l (0), \\ P ^ {i j} (l (k)), & l (k - 1) <   d \leq l (k) (k = 1, \ldots K ^ {i j}), \\ 0, & d > l (K ^ {i j}), \end{array} \right.\tag{5}
$$

![](images/c6f5df3638dd27a7c0dbf419210e255e3ba9e43e63fce23c3bf0636a112ee7b8.jpg)  
Fig. 2. Area discretization.

where $l ( 0 ) = 0 , ~ l ( K ^ { i j } ) = \operatorname* { m a x } \{ D _ { i } ^ { b } \mid b = 1 , 2 , . . . , B _ { i } \}$ . To bound its error, we set $l ( k ) = \beta _ { i j } ( ( 1 + \epsilon _ { 1 } ) ^ { k / 2 } - 1 ) ( k =$ $1 , . . . , K ^ { i j } - 1 )$ , then have the following lemmas.

Lemma 1: For the charging power, the approximation error with the piecewise constant function is

$$
1 \leq \frac {P ^ {i j} (d)}{\widetilde {P ^ {i j}} (d)} \leq 1 + \epsilon_ {1}.\tag{6}
$$

Lemma $2 \cdot$ The number of sectors partitioned by <sup>M</sup> sensors is at most $O ( M ^ { 2 } \epsilon _ { 1 } ^ { - 2 } )$ ).

Lemma 3: For the charging utility for the sensor $o _ { j }$ , the approximation error is

$$
1 \leq \frac {U _ {j} (\sum_ {i = 1} ^ {N} P ^ {i j} (d))}{U _ {j} (\sum_ {i = 1} ^ {N} \widetilde {P ^ {i j}} (d))} \leq 1 + \epsilon_ {1}.\tag{7}
$$

The proofs of the above lemmas are given by [83]. Then, for each type of chargers, we partition the area by drawing multiple concentric sectors about each sensor according to the piecewise constant function and the sensor’s orientation. Taking Fig. 2 as an example, the area has been partitioned into 25 subareas.

## B. Maximum Coverage Set Extraction

After area discretization, each sensor always receives constant charging power from its surrounding chargers in the same subarea if these chargers cover the sensor. In this subsection, we aim to analyze the coverage relationship between chargers and sensors. For further analysis, we introduce the following notions.

Definition 1. (Coverage Set): For a charger $s _ { i }$ with a given position and orientations, the coverage set $O _ { i }$ is defined as a set consisting of the tuple $( o _ { j } , t _ { o _ { j } } )$ , where $t _ { o _ { i } }$ is the number of orientations that can cover $o _ { j }$ . For $j \in \{ 1 , 2 , . . . , M \} , ( o _ { j } , t _ { o _ { j } } ) \in$ $O _ { i }$ if and only if ${ { t } _ { { { o } _ { i } } } } > 0$

Definition 2. (Maximum Coverage Set): For a charger $s _ { i }$ and its coverage set $O _ { i } ^ { 1 }$ , if there isn’t other coverage set $O _ { i } ^ { 2 } .$ , such that for each tuple $( o _ { j } , t _ { o _ { i } } ^ { 1 } )$ in $O _ { i } ^ { 1 }$ , we have $( o _ { j } , t _ { o _ { i } } ^ { 2 } ) \in \mathrm { \overrightarrow { O } } _ { i } ^ { 2 }$ , where $0 < t _ { o _ { i } } ^ { 1 } \le t _ { o _ { i } } ^ { 2 }$ and $O _ { i } ^ { \bar { 1 } } \neq O _ { i } ^ { 2 }$ , then we say $O _ { i } ^ { 1 }$ is a maximum coverage set of $s _ { i }$ .

Definition 3. (Omnidirectional Coverage Set): Suppose that the charger $s _ { i }$ is omnidirectional, for a given position, we say the set consisting of its covered sensors is a omnidirectional coverage set for $s _ { i } ,$ denoted by $\mathcal { O } _ { i }$

Obviously, $\mathcal { O } _ { i }$ contains all the sensors can be covered by charger $s _ { i } ,$ , and the MCS of $s _ { i }$ is always a subset of $\mathcal { O } _ { i }$ due to the directionality of charger. According to the definition of MCS, we only need to select the strategies for the charger from the MCS set to maximize the overall charging utility. Then, our task is transformed to extract all the MCSs for each charger.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 1: MCS Extraction for WANDA-ROF.

Input: the r-th type of chargers and the subarea H
Output: the MCS set of the r-th type of chargers in the subarea H

1 Compute the omnidirectional coverage set  $O_{r}$  of the r-th type of chargers in the subarea H.

2 for each three sensors  $o_{j}, o_{k},$  and  $o_{l}$  in  $O_{r}$  do

3 Draw arcs passing  $o_{j}$  and  $o_{k}$  with all the possible circle angles of  $A_{s_{r}}^{b}, |\theta_{s_{r}}^{b_{1}}, \theta_{s_{r}}^{b_{2}}| \pm \frac{1}{2} A_{s_{r}}^{b_{1}} \pm \frac{1}{2} A_{s_{r}}^{b_{2}}|$ .

4 Draw arcs passing  $o_{k}$  and  $o_{l}$ , with all the possible circle angles of  $A_{s_{r}}^{b}, |\theta_{s_{r}}^{b_{1}}, \theta_{s_{r}}^{b_{2}}| \pm \frac{1}{2} A_{s_{r}}^{b_{1}} \pm \frac{1}{2} A_{s_{r}}^{b_{2}}|$ .

5 Locate the charger at the intersections accordingly and add the coverage sets to the candidate MCS set.

6 Draw arcs passing  $o_{j}$  and  $o_{k}$  with all the possible circle angles of  $A_{s_{r}}^{b}, |\theta_{s_{r}}^{b_{1}}, \theta_{s_{r}}^{b_{2}}| \pm \frac{1}{2} A_{s_{r}}^{b_{1}} \pm \frac{1}{2} A_{s_{r0}}^{b_{2}}|$ .

7 Draw a straight line passing  $o_{k}$  and  $o_{l}$ .

8 Locate the charger at the intersections accordingly and add the coverage sets to the candidate MCS set.

9 for each pair of sensors  $o_{j}, o_{k}$  in  $O_{r}$  do

10 Draw arcs passing  $o_{j}$  and  $o_{k}$  with all the possible circle angles of  $A_{s_{r}}^{b}, |\theta_{s_{r}}^{b_{1}}, \theta_{s_{r}}^{b_{2}}| \pm \frac{1}{2} A_{s_{r}}^{b_{1}} \pm \frac{1}{2} A_{s_{r}0}^{b_{2}}|$  and obtain the intersecting lines of the subarea and the arcs.

11 Randomly select a point on each intersecting line and locate the charger at this point, find all the MCSs of this point as candidate MCSs.

12 Randomly select a point in the subarea and locate the charger at this point, find all the MCSs of this point as candidate MCSs.

13 Delete the coverage sets in the candidate MCS set that are not MCSs.
</div>

When the subarea is a single point, all MCSs can be easily obtained by exploiting the relationship between the coverage area and the positions of sensors. For a normal area, let $\Pi _ { r } ^ { H }$ be the strategy set for the <sup>r</sup>-th type of chargers in the subarea <sup>H</sup> and <sup>Π</sup>r be the union set of the strategy set for the <sup>r</sup>-th type of chargers of all the subareas. Then, we know that

Theorem 2: For any coverage set of the <sup>r</sup>-th type of chargers in the subarea <sup>H</sup>, it is a MCS only if it is included in $\Pi _ { r } ^ { H }$

Proof: To facilitate derivation, we denote all MCSs of the <sup>r</sup>-th type of chargers in the subarea <sup>H</sup> as $\Gamma _ { r } ^ { H }$ . What we want to prove is $\Gamma _ { r } ^ { H } \subset \overline { { \mathbf { I } } } _ { r } ^ { H }$ , thus we can reduce our solution space to finite without omission. For any $\mathbf { M C S } \gamma \sin \Gamma _ { r } ^ { H }$ , we move it in the subarea while adjust its orientations simultaneously to guarantee that there is no sensor going to leave its coverage areas. During the process we can maintain the MCS property of <sup>Γ</sup> because it does not lose any coverage sensor. We note that it is impossible for a sensor to touch the sector boundaries from the outside of $\gamma ,$ because this contradicts with its MCS property. We try to make as many sensors as possible touch the sector boundaries of $\gamma .$ Then, we must encounter one of the following conditions, called “critical coverage conditions”.

Case 1: At most one sensor appears on the boundary of a sector. This case is illustrated in Fig. 3(a), corresponding to Line 12 in Algorithm 1.

Case 2: At most two sensors appear on the boundaries of sectors. This case is illustrated in Fig. 3(b1)–(b3), corresponding to Line 9 to Line 11 in Algorithm 1.

Case 3: At most three sensors appear on at least two boundaries of sectors. This case is illustrated in Fig. 3(c1)–(c4), corresponding to Line 2 to Line 8 in Algorithm 1.

Therefore, all the MCSs must be included in the set of coverage sets generated from these cases. Then, we can obtain the MCS set by deleting the coverage sets that are not MCSs according to Def. 2.

## C. Greedy Algorithm

After the above area discretization and MCS extraction, we obtain all strategies for chargers. Then, WANDA-ROF can be reformulated as

$$
\begin{array}{l l} \text {(P2)} & \max \frac {1}{M} \sum_ {j = 1} ^ {M} U _ {j} \left(\sum_ {(s _ {i}, \theta_ {i}) \in \Pi} x _ {i} \sum_ {b = 1} ^ {B _ {i}} P (s _ {i}, \theta_ {i} ^ {b}, o _ {j}, \phi_ {j})\right), \\ & s. t. \sum_ {(s _ {i}, \theta_ {i}) \in \Pi_ {r}} x _ {i} = N _ {r}, x _ {i} \in \{0, 1 \}, \end{array}
$$

where $x _ { i }$ is a binary indicator denoting whether the strategy $( s _ { i } , \theta _ { i } )$ is selected or not.

For further discussion, we give the following definitions.

Definition 4. ([84] Monotone Submodular Set Function): Let <sup>S</sup> be a finite ground set. A real-valued set function $f : 2 ^ { S } $ R is normalized, monotonic, and submodular if and only if it satisfies the following conditions, respectively: $( 1 ) \ f ( \varnothing ) = 0 ;$ (2) $f ( A \cup \{ e \} ) - f ( A ) \geq 0$ for any $A \subseteq S$ and $e \in S \backslash A ; ( 3 )$ $f ( A \cup \{ e \} ) - f ( A ) \geq f ( B \cup \{ e \} ) - f ( B )$ for any $A \subseteq B \subseteq$ $S$ and $e \in S / B$

Definition 5. ([84] Matroid): A matroid M is a strategy $\mathcal { M } =$ <sup>(S, L)</sup> where <sup>S</sup> is a finite ground set, $L \subseteq 2 ^ { S }$ is a collection of independent sets, such that: (1) $\mathcal { O } \in L ; ( 2 )$ if $X \subseteq Y \in L$ then $X \in L ; ( 3 )$ if X, $Y \in L$ , and $| X | < | Y |$ , then $\exists y \in Y / X$ $X \cup \{ y \} \in L$

Definition 6. ([84] Partition Matroid): Given $\textstyle S = \bigcup _ { i = 1 } ^ { k } S _ { i } ^ { \prime }$ is the disjoint union of <sup>k</sup> sets, $l _ { 1 } , l _ { 2 } , . . . . , l _ { k }$ are positive integers, a partition matroid $\mathcal { M } = ( S , L )$ is a matroid where $L = \{ X \subset$ $S ^ { ' } : | X \cap S _ { i } ^ { \prime } | \leq l _ { i } f o r i \in [ k ] \} ,$

By constructing a partition matroid $\mathcal { M } = ( \Pi , L )$ , where $L =$ $\{ \mathcal { X } \subset \Pi : | \mathcal { X } \cap \Pi _ { r } | \leq N _ { r } \ f o r \ r \in [ R ] \}$ , WANDA-ROF can be reformulated as

$$
\text {(P3)} \max f (\mathcal {X}) = \frac {1}{M} \sum_ {j = 1} ^ {M} U _ {j} \left(\sum_ {(s _ {i}, \theta_ {i}) \in \mathcal {X}} \sum_ {b = 1} ^ {B _ {i}} P (s _ {i}, \theta_ {i} ^ {b}, o _ {j}, \phi_ {j})\right),
$$

$$
s. t. \quad \mathcal {X} \in L,\tag{8}
$$

![](images/933d005cc1a8cdda719c85a9f5cee865545562c834ead48a3dbd0d61b4a1848d.jpg)  
Fig. 3. Critical coverage conditions for WANDA-ROF. (a) At most one sensor appears on the boundary of a sector. (b1)–(b3) At most two sensors appear on the boundaries of sectors. (c1)–(c4) At most three sensors appear on at least two boundaries of sectors.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 2: Greedy Algorithm to Select Strategies for WANDA-ROF.

Input: the candidate strategy set $\Pi$
Output: the positions and orientations for all the chargers
1 for all $r \in [R]$ do
2    while $|\mathcal{X}_r| \leq N_r$ do
3    $\mathcal{X} = \bigcup_{r=1}^{R} \mathcal{X}_r$.
4    $e^* = \text{argmax}_{e \in \Pi_r \setminus \mathcal{X}_r} f(\mathcal{X} \cup \{e\}) - f(\mathcal{X})$.
5    $\mathcal{X}_r = \mathcal{X}_r \cup \{e^*\}$.
</div>

```txt
Algorithm 3: Scheme for WANDA-ROF.

Input: the parameters of all the chargers and sensors
Output: the positions and orientations for all the chargers

1 for all types of chargers do

2 Divide the area into multiple subareas by drawing multiple concentric sectors about each sensor according to the piecewise constant function and the sensor's orientation.

3 for all the subareas do

4 Perform Alg. 1 to extract the MCSs in the subarea and add them to the candidate strategy set.

5 Perform Alg. 2 to select strategies for all the chargers.
```

where the decision variable is X (the selected strategies for all chargers). For P3, we have the following lemma.

Lemma 4: The objective function <sup>f(</sup>X <sup>)</sup> in P3 is a monotone submodular function and the constraint is a partition matroid constraint.

Please refer to our supplemental material for the proof.

Consequently, P3 can be addressed by a $\begin{array} { l } { { \frac { 1 } { 2 } } } \end{array}$ approximation greedy algorithm proposed in [84]. We present our algorithm in Algorithm 2.

In summary, our scheme for WANDA-ROF is presented in Algorithm 3.

Theorem 3: Our solution for WANDA-ROF achieves an approximation ratio of $\frac { 1 } { 2 } - \epsilon$ (setting $\begin{array} { r } { \epsilon _ { 1 } = \frac { 2 \epsilon } { 1 - 2 \epsilon } ) } \end{array}$ and its time complexity is $O ( N M ^ { 5 } \epsilon ^ { - 2 } B ^ { 4 } )$ (setting $B = \operatorname* { m a x } \{ B _ { r } \mid r =$ $1 , 2 , . . . , R \} )$ ).

Proof: By [84], the approximation ratio of Algorithm 2 is $\frac { 1 } { 2 }$ . Combining it with the approximation error of area discretization $\frac { 1 } { 1 + \epsilon _ { 1 } }$ , the total approximation ratio of WANDA-ROF is $\begin{array} { r } { \frac { 1 } { 2 ( 1 + \epsilon _ { 1 } ) } = \frac { 1 } { 2 } - \epsilon } \end{array}$ where $\begin{array} { r } { \epsilon _ { 1 } = \frac { 2 \epsilon } { 1 - 2 \epsilon } } \end{array}$ . Then, we analyze the time complexity of our solution for ROF situation. For each type of chargers, the number of subareas generated by area discretization is $O ( M ^ { 2 } \epsilon _ { 1 } ^ { - 2 } )$ . In Algorithm 1, the size of omnidirectional coverage set for each type of chargers is $O ( M )$ . In the first for loop, we first enumerate each three sensors in $\mathcal { O } _ { r }$ and draw arcs and lines with all the possible combinations. The time complexity of this loop is $O ( M ^ { 3 } B _ { r } ^ { 4 } )$ . Then, in the second for loop, we enumerate each two sensors in $\mathcal { O } _ { r }$ and draw arcs with different circle angles.

After randomly selecting points on all the intersecting lines, we determine all the MCSs of these points. Thus, the time complexity of this step is $O ( M ^ { 3 } B _ { r } ^ { 2 } )$ . Then, we deal with the last cases with time complexity of $O ( M )$ and delete all the coverage sets that are not MCSs with time complexity of $O ( M ^ { 2 } )$ Thus, the total time complexity of Algorithm 1 is $O ( M ^ { 3 } B _ { r } ^ { 4 } )$ . To generate all the candidate strategies for each type of chargers, we need to perform Algorithm 1 in all the subareas partitioned by the <sup>r</sup>-th type of chargers. Since the number of subareas is $\dot { O } ( M ^ { 2 } \epsilon _ { 1 } ^ { - 2 } )$ , the number of strategies for each type of chargers is $O ( M ^ { 5 } \epsilon ^ { - 2 } B ^ { 4 } )$ by setting $\boldsymbol { B } = \operatorname* { m a x } \{ B _ { r } \mid r = 1 , 2 , . . . , R \}$ and $\textstyle \epsilon _ { 1 } = { \frac { 2 \epsilon } { 1 - 2 \epsilon } }$ . Then, we perform Algorithm 2 to select the strategies for all the chargers. Algorithm 2 requires <sup>N</sup> iterations and in each iterations the algorithm needs to go through all the candidate strategies of each type of chargers. Thus, the total time complexity of Algorithm 2 is ${ \cal O } ( N M ^ { \mathrm { 5 } } \epsilon ^ { - 2 } B ^ { 4 } )$ <sup>)</sup>. Therefore, the total time complexity of our scheme is $O ( N M ^ { 5 } \epsilon ^ { - 2 } B ^ { 4 } )$ . -

## V. SOLUTION FOR WANDA-ROU

In this section, we propose a two-level submodular optimization scheme for WANDA-ROU. The preliminary of this scheme includes area discretization and candidate position set construction. The area discretization part is the same as that in Section IV-A, so we omit here to avoid repetition. After area discretization, we construct a candidate position set for each type of chargers, which actually includes all the possible positions that can generate MCSs. Then, we divide the decision variables, that is, the position of charger and the orientation of charger, in our problem into two levels by noting that the two kinds of variables are tightly-coupled, while the position and orientation of charger are in the upper-level and lower-level, respectively. Then, the problem transforms into a two-level optimization problem, that is, first selecting positions in the candidate position set for each charger in the upper-level and then selecting orientations for each selected position in the lower-level, which both fall into the realm of maximizing a monotone submodular function subject to a partition matroid constraint and we adopt a greedy algorithm to solve this problem. Then, we prove that our scheme achieves an approximation ratio of $\begin{array} { r l r } {  { \frac { 1 } { 6 } - \epsilon . } } \end{array}$

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 4: Candidate Position Set Construction for WANDA-ROU.

Input: the r-th type of chargers and the subarea H
Output: the candidate position set for the r-th type of chargers in the subarea H

1 Compute the omnidirectional coverage set  $O_{r}$  of the r-th type of chargers in the subarea H.

2 for each two pairs of sensors  $o_{j}$ ,  $o_{k}$  and  $o_{g}$ ,  $o_{h}$  in  $O_{r}$  do

3 Draw arcs passing  $o_{j}$  and  $o_{k}$ ,  $o_{g}$  and  $o_{h}$ , with different circle angles of  $A_{s_{r}}^{b_{1}}$ ,  $A_{s_{r}}^{b_{2}}$ , respectively, for  $b_{1}, b_{2} \in \{1, 2, ..., B_{r}\}$ . Add the intersections to the candidate position set and record the corresponding fixed orientations.

4 Draw a straight line passing  $o_{g}$  and  $o_{h}$  and arcs passing  $o_{j}$  and  $o_{k}$  with circle angles of  $A_{s_{r}}^{b}$ , for  $b \in \{1, 2, ..., B_{r}\}$ . Add the intersections to the candidate position set and record the corresponding fixed orientations.

5 Draw a straight line passing  $o_{j}$  and  $o_{k}$  and a straight line passing  $o_{g}$  and  $o_{h}$ . Add the intersection to the candidate position set and record the corresponding fixed orientations.

6 for each three sensors  $o_{j}$ ,  $o_{k}$  and  $o_{g}$  in  $O_{r}$  do

7 Draw a straight line passing  $o_{j}$  and  $o_{k}$  and arcs passing  $o_{k}$  and  $o_{h}$  with circle angles of  $A_{s_{r}}^{b}$ , for  $b \in \{1, 2, ..., B_{r}\}$ . Add the intersections to the candidate position set and record the corresponding fixed orientation.

8 for each pair of sensors  $o_{j}$ ,  $o_{k}$  in  $O_{r}$  do

9 Draw arcs passing  $o_{j}$  and  $o_{k}$  with circle angles of  $A_{s_{r}}^{b}$ , for  $b \in \{1, 2, ..., B_{r}\}$  and obtain the intersecting lines of the subarea and the arcs. Randomly select a point on each intersecting line and add the point to the candidate position set.

10 Draw a straight line passing  $o_{j}$  and  $o_{k}$  and obtain the intersecting line of the subarea and the straight line. Randomly select a point on the intersecting line and add the point to the candidate position set.

11 Randomly select a point in the subarea and add the point to the candidate position set.
</div>

## A. Candidate Position Set Construction

For WANDA-ROU, since the time complexity to determine all the MCSs of a given position achieves $\dot { O ( M ^ { B ^ { \prime } ) } }$ , where $B ^ { \prime }$ is the number of the orientations need to be determined, directly adopting the same idea in WANDA-ROF is infeasible. For further analysis, we present the following notion.

Definition $7 ;$ (Candidate Position): For a charger $s _ { i }$ and the position $c _ { i } ,$ if at least one MCS can be obtained by adjusting the orientations of $s _ { i }$ at $c _ { i }$ , then we say $c _ { i }$ is a candidate position of $s _ { i }$ .

Then, we propose Algorithm 4 to determine the candidate position set for each type of chargers. Generally, Algorithm 4 generates all the candidate positions and holds the following theorem. Note that $C _ { r } ^ { H }$ denotes the candidate position set of the <sup>r</sup>-th type of chargers in the subarea $H ,$ , and $C _ { r }$ denotes the union set of the candidate position set of the <sup>r</sup>-th type of chargers of all the subareas.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 5: Greedy Algorithm to Select Orientations with Given Position Set.

Input: the candidate orientation set $\Phi_i^b$ for $b$-th orientation of the charger $s_i, b \in \{1, 2, ..., B_i\}$, $i \in \{1, 2, ..., N\}$

Output: the orientation set $P = \{p_i^b | b \in \{1, 2, ..., B_i\}, i \in \{1, 2, ..., N\}\}$

for all $b \in [B_i]$ and $i \in [N]$ do

if the $b$-th orientation of the charger $s_i$ is not determined then

$p_i^b = \arg\max_{e \in \Phi_i^b} h_X(P \cup \{e\}) - h_X(P)$.

$P = P \cup \{p_i^b\}$.
</div>

Theorem 4: $C _ { r } ^ { H }$ includes all the positions that can possibly generate MCSs for the <sup>r</sup>-th type of chargers in the subarea <sup>H</sup>.

Proof: As per Definition 7, it is obvious that all the possible positions that can generate MCSs are included in the candidate position set for each type of chargers. Similar to the proof of Theorem 2, we move a MCS strategy in the subarea and adjust its orientations and guarantee that no sensor leaves its coverage areas. We summarize the critical coverage conditions for WANDA-ROU as follows. After this, we can finish our proof by imitating the proof of Theorem 2.

Case 1: At most one sensor appears on the boundary of the sector. This case is shown in Fig. 4(a), corresponding to Line 11 in Algorithm 4.

Case 2: At most two sensors appear on the boundaries of the same sector. This case is shown in Fig. 4(b)–(b2), corresponding to Line 8 to Line 10 in Algorithm 4.

Case 3: At most three sensors appear on both boundaries of the same sector. This case is shown in Fig. 4(d), corresponding to Line 6 to Line 7 in Algorithm 4.

Case 4: At most two pairs of sensors appear on the boundaries of two sectors, respectively. This case is shown in Fig. 4(c1)– (c3), corresponding to Line 2 to Line 5 in Algorithm 4. -

We note that 0 to 2 orientations can be fixed in the process of determining candidate positions due to the geometric relationships between sensors and coverage areas, which is the reason why we need to record the corresponding fixed orientations for each position in Algorithm 4.

## B. Orientations Determination for Given Position Set

In this subsection, we consider the lower-level problem, namely, determining the orientations for the positions in a given position set. First, we construct function $h _ { X } ( P )$ as the charging utility function with given position set <sup>X</sup>, where <sup>P</sup> is the strategy set for <sup>X</sup>. We have $\begin{array} { r } { \bar { h } _ { X } ( \bar { P ) } = \sum _ { i = 1 } ^ { N } h _ { \{ x _ { i } \} } ( P _ { i } ) } \end{array}$ , where $x _ { i }$ is the position of the charger $s _ { i }$ in $X$ and $P _ { i }$ is the orientation set of $s _ { i } .$ . For the <sup>b</sup>-th orientation of the charger $s _ { i } ,$ we first obtain the candidate orientation set $\Phi _ { i } ^ { b }$ , which includes all the orientations that can generate MCSs. It can be easily obtained by rotating the charger. Then, let $\begin{array} { r } { \Phi = \bigcup _ { 1 } ^ { N } \bigcup _ { 1 } ^ { B _ { i } } \Phi _ { i } ^ { b } } \end{array}$ to be the union set and we can construct a partition matroid $\mathcal { M } = ( \Phi , I )$ , where <sup>I</sup> is defined as follows

![](images/f5ecf968ab7a6e446170c5aa6134f8a9d9c90b723616a84b6d5618aebca40fd0.jpg)  
Fig. 4. Critical coverage conditions for WANDA-ROU. (a) At most one sensor appears on the boundary of the sector. (b1)–(b2) At most two sensors appear on the boundaries of the same sector. (c1)–(c3) At most two pairs of sensors appear on the boundaries of two sectors, respectively. (d) At most three sensors appear on both boundaries of the same sector.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 6: Greedy Algorithm to Select Strategies for WANDA-ROU.

Input: the candidate position set  $C_{r}$  for the r-th type of chargers,  $r \in \{1, 2, ..., R\}$ 

Output: the positions and orientations for all the chargers

1 for all  $r \in [R]$  do

2 while  $|X_{r}| \leq N_{r}$  do

3  $X = \bigcup_{r=1}^{R} X_{r}$ .

4  $e^{*} = \arg\max_{e \in C_{r} \setminus X_{r}} \mathcal{U}(X \cup \{e\}) - \mathcal{U}(X)$ 

5 (perform Alg. 5 to determine the orientations of chargers under position set  $X \cup \{e\}$  and X, and compute the corresponding overall charging utility).

5  $X_{r} = X_{r} \cup \{e^{*}\}$ .
</div>

$$
I = \{P \subset \Phi : | P \cap \Phi_ {i} ^ {b} | \leq h _ {i} ^ {b} f o r b \in [ B _ {i} ] \text {   and   } i \in [ N ] \},\tag{9}
$$

where $h _ { i } ^ { b }$ is a binary indicator that $h _ { i } ^ { b } = 1$ if and only if the <sup>b</sup>-th orientation of the charger $s _ { i }$ is undetermined in Algorithm 4. Then, we can formulate this problem as follows.

$$
\begin{array}{r l} \text {(P4)} & \max h _ {X} (P), \\ & s. t. P \in I. \end{array}
$$

For the objective function of P4, we have the following lemma to reveal its characteristics.

Lemma 5: The objective function $h _ { X } ( P )$ in P4 is a monotone submodular function and the constraint is a partition matroid constraint.

Please refer to our supplemental material for the proof.

Then, we can address P4 by a greedy algorithm with approximation ratio of $\textstyle { \frac { 1 } { 2 } }$ [84], which is shown in Algorithm 5.

## C. Greedy Algorithm

For the upper-level problem, we need to determine the positions for all the chargers in the candidate position set $C .$ We construct function $\mathcal { U } ( X )$ as the overall charging utility of selected position set X whose orientations of each position are selected by Algorithm 5. Similarly, since the orientations of chargers can be calculated as presented in Section V-B, we limit the orientation variables and use $\mathcal { U } ( X )$ to represent the overall charging utility, which is only determined by the position set <sup>X</sup>. The constraint of this problem can also be formulated as a partition matroid $\mathcal { M } = ( X , I )$ , where <sup>I</sup> is defined as

```txt
Algorithm 7: Scheme for WANDA-ROU.

Input: the parameters of all the chargers and sensors
Output: the positions and orientations for all the chargers

1 for all types of chargers do

2 Divide the area into multiple subareas by drawing multiple concentric sectors about each sensor according to the piecewise constant function and the sensor's orientation.

3 for all the subareas do

4 Perform Alg. 4 to determine the candidate positions that can lead to MCSs and add them to the candidate position set.

5 Perform Alg. 6 to select strategies for all the chargers.
```

$$
I = \{X \subset C: | X \cap C _ {r} | \leq N _ {r} f o r r \in [ R ] \}.\tag{10}
$$

Then, our problem can be reformulated as

$$
\begin{array}{r l} \text {(P5)} & \max \mathcal {U} (X), \\ & s. t. X \in I. \end{array}
$$

For U<sup>(X)</sup>, we have the following lemma.

Lemma 6: The objective function U<sup>(X)</sup> in P5 is a monotone submodular function and the constraint is a partition matroid constraint.

Please refer to our supplemental material for the proof.

Then, P5 can be solved by the greedy algorithm in [84], as is shown in Algorithm 6.

In summary, the details of our scheme for WANDA-ROU is shown in Algorithm 7.

Theorem 5: Our solution for WANDA-ROU achieves an approximation ratio of $\frac { 1 } { 6 } - \epsilon$ (setting $\begin{array} { r } { \epsilon _ { 1 } = \frac { 6 \epsilon } { 1 - 6 \epsilon } ) } \end{array}$ and its time complexity is $O ( N ^ { 2 } M ^ { \mathsf { ^ { 2 } } } \epsilon ^ { - 2 } B ^ { 3 } )$ (setting $B = \operatorname* { m a x } \{ B _ { r } \mid r =$ $1 , 2 , . . . , R \} )$ .

Proof: Our algorithm adopts two greedy algorithms to determine the strategies of chargers, i.e., greedily selecting positions and greedily evaluating each position when selecting positions. We denote $X = \{ x _ { 1 } , x _ { 2 } , . . . , x _ { N } \}$ as the strategy set generated by our algorithm and the element $x _ { i }$ is the position for $s _ { i }$ . Then, we denote $Y = \{ y _ { 1 } , y _ { 2 } , . . . , y _ { N } \}$ as the strategy set generated by greedily selecting positions and precisely evaluating positions by enumeration. Besides, we denote $Z = \{ z _ { 1 } , z _ { 2 } , . . . , z _ { N } \}$ as the optimal solution. Besides, we denote the marginal utility increment by the strategies $x _ { i }$ under strategy set <sup>X</sup> as $\Delta { \mathcal { U } } _ { ( X ) } ( x _ { i } )$

![](images/c1eae6fd98f17417b08f8b9d5603e6e3177ed0b29ebba32dede99fd518c01f68.jpg)  
Fig. 5. Auxiliary figure for the proof of Theorem 5.

and the set of the first $i - 1$ elements in $X$ as $X _ { i - 1 }$ . We construct strategy set $X \& Y = \{ x _ { 1 } , x _ { 2 } , . . . , x _ { N } , y _ { 1 } , y _ { 2 } , . . . , y _ { N } \}$ by concatenating the elements in <sup>X</sup> and $Y$ , as is shown in Fig. 5. We denote $a ^ { \prime }$ as greedily selecting orientations and $a ^ { * }$ as selecting optimal orientations for the position <sup>a</sup>. Since we always choose the element with the maximal charging utility increment in each iteration of the greedy algorithm, for two positions $x _ { i }$ and $y _ { i }$ of the charger $s _ { i }$ , we have

$$
\Delta \mathcal {U} _ {(X _ {i - 1})} (x _ {i} ^ {\prime}) \geq \Delta \mathcal {U} _ {(X _ {i - 1})} (y _ {i} ^ {\prime}).\tag{11}
$$

Since $\mathcal { U } ( . )$ is a monotone submodular function, we have

$$
\Delta \mathcal {U} _ {(X _ {i - 1})} (y _ {i} ^ {\prime}) \geq \Delta \mathcal {U} _ {(X \& Y _ {i - 1})} (y _ {i} ^ {\prime}).\tag{12}
$$

Note that the greedy algorithm for maximizing the monotone submodular function subject to a partition matroid constraint achieves $\frac { 1 } { 2 }$ approximation ratio as per [84], we thus have

$$
\Delta \mathcal {U} _ {(X \& Y _ {i - 1})} (y _ {i} ^ {\prime}) > \frac {1}{2} \Delta \mathcal {U} _ {(X \& Y _ {i - 1})} (y _ {i} ^ {*}).\tag{13}
$$

Combining Inequalities (11), (12) and (13), we have

$$
\Delta \mathcal {U} _ {(X _ {i - 1})} (x _ {i} ^ {\prime}) > \frac {1}{2} \Delta \mathcal {U} _ {(X \& Y _ {i - 1})} (y _ {i} ^ {*}).\tag{14}
$$

Since

$$
\mathcal {U} (X) = \sum_ {i = 1} ^ {N} \Delta \mathcal {U} _ {\left(X _ {i - 1}\right)} \left(x _ {i} ^ {\prime}\right),
$$

$$
\mathcal {U} (X \& Y) = \sum_ {i = 1} ^ {N} \Delta \mathcal {U} _ {(X _ {i - 1})} (x _ {i} ^ {\prime}) + \sum_ {i = 1} ^ {N} \Delta \mathcal {U} _ {(X \& Y _ {i - 1})} (y _ {i} ^ {*}),\tag{15}
$$

(note that $X _ { 0 } = Y _ { 0 } = \emptyset )$ , we have

$$
\mathcal {U} (X) > \frac {1}{3} \mathcal {U} (X \& Y).\tag{16}
$$

Moreover, the greedy algorithm for maximizing the monotone submodular function subject to a partition matroid constraint achieves an approximation ratio of $\frac { 1 } { 2 }$ as per [84]. Thus, we have

$$
\mathcal {U} (Y) > \frac {1}{2} \mathcal {U} (Z).\tag{17}
$$

Combining Inequalities (16) and (17), we have

$$
\mathcal {U} (X) > \frac {1}{3} \mathcal {U} (X \& Y) > \frac {1}{3} \mathcal {U} (Y) > \frac {1}{6} \mathcal {U} (Z).\tag{18}
$$

Therefore, the approximation ratio of our scheme is $\frac { 1 } { 6 }$ . Since the approximation error of area discretization is $\frac { 1 } { 1 + \epsilon _ { 1 } }$ , the total approximation ratio is $\begin{array} { r } { \frac { 1 } { 6 ( 1 + \epsilon _ { 1 } ) } = \frac { 1 } { 6 } - \epsilon } \end{array}$ by setting $\begin{array} { r } { \epsilon _ { 1 } = \frac { 6 \epsilon } { 1 - 6 \epsilon } } \end{array}$

For time complexity analysis, in Algorithm 4, we enumerate each four sensors in $\mathcal { O } _ { r }$ and draw arcs and lines with all the possible cases in the first for loop. The time complexity of this for loop is $O ( M ^ { 4 } B _ { r } ^ { 2 } )$ . Next, in the second for loop, we enumerate each three sensors in $\mathcal { O } _ { r }$ , and the time complexity is $O ( M ^ { 3 } B _ { r } )$ In the third for loop, we enumerate each two sensors and obtain the candidate positions by randomly selecting points on the intersecting lines. The time complexity of is $O ( M ^ { 2 } B _ { r } )$ . After that, we randomly select a point in the subarea as a candidate position, and the time complexity is $O ( 1 )$ . Thus, the total time complexity for Algorithm 4 is $O ( M ^ { 4 } B _ { r } ^ { 2 } )$ . Since we need to perform Algorithm 4 for $O ( M ^ { 2 } \epsilon _ { 1 } ^ { - 2 } )$ times to generate the candidate positions in all the subareas, the number of candidate positions of each type of chargers achieves ${ \cal O } ( M ^ { 6 } \epsilon ^ { - 2 } B ^ { 2 } )$ by setting $B = \operatorname* { m a x } \{ B _ { r } \mid r = 1 , 2 , . . . , R \}$ and $\begin{array} { r } { \epsilon _ { 1 } = \frac { 2 \epsilon } { 1 - 2 \epsilon } } \end{array}$ . Then, in each iteration of Algorithm 6, we compute $\mathcal { U } ( . )$ for all the candidate positions. Since we need to generate the corresponding orientations from the position set, the time complexity of each iteration is $O ( N M B \cdot M ^ { 6 } \epsilon ^ { - 2 } B ^ { 2 } )$ . As the number of iterations is $O ( N )$ the total time complexity is $O ( N ^ { 2 } M ^ { 7 } \epsilon ^ { - 2 } B ^ { 3 } )$ <sup>)</sup>. Therefore, the total time complexity of our scheme is $O ( N ^ { 2 } M ^ { 7 } \epsilon ^ { - 2 } B ^ { 3 } )$ -

## VI. SIMULATION EXPERIMENTS

In this section, we conduct simulations to validate the performance of our proposed algorithms (Algorithm 3 for WANDA-ROF and Algorithm 7 for WANDA-ROU). The wireless charging model and our formalized problem are described in Section III. To the best of our knowledge, there are no new schemes for the problem of maximizing the charging utility given a certain number of chargers with multiple directional antennas. Most of the existing studies on wireless charger networks focus on placing chargers as few as possible, placing chargers to achieve full coverage, scheduling the movement/energy of chargers, charging safety, etc. Despite that there are no such direct relative literatures regarding our problem of placement of multi-antenna chargers for maximizing utility, we have designed 3 placement algorithms and adapted 8 placement algorithms to place multi-antenna chargers. Specifically, We compare our proposed algorithms with 3 self-designed algorithms in terms of error threshold $\epsilon _ { 1 }$ , power threshold $P _ { \mathit { t h } }$ charging distance $D ,$ , and charging constant $\alpha ;$ and with 8 related adapted algorithms in terms of average charging utility, minimum charging utility, power waste, and execution time.

## A. Baseline Setup

We conduct simulations in a <sup>40</sup> × <sup>40</sup> square area where 6 chargers need to be deployed and 10 sensors are uniformly distributed with random positions and random orientations. There are 3 types of chargers and 4 types of sensors in our simulations. The numbers of different types of chargers and sensors are 1, 2, 3 and 1, 2, 3, 4, respectively. The receiving angles of sensors are set to ${ \begin{array} { l } { { \frac { \pi } { 5 } } , { \frac { \pi } { 4 } } , { \frac { \pi } { 3 } } , { \mathrm { a n d ~ } } { \frac { \pi } { 2 } } } \end{array} }$ . The numbers of antennas of chargers are set to 2, 3, and 4, whose charging distances are 5, 6, and 7, respectively. For the first type of chargers, the charging angles are set to $\frac { \pi } { 4 }$ and $\frac { \pi } { 3 }$ ; and the relative orientation between adjacent antennas is set to <sup>π</sup> in WANDA-ROF. For the second type of chargers, the charging angles are set to ${ \mathit { \frac { \pi } { 5 } } } , \ { \mathit { \frac { \pi } { 4 } } }$ , and ${ \frac { \pi } { 3 } } ;$ and the relative orientations between adjacent antennas are set to <sup>π</sup> and $\frac { \pi } { 3 }$ in WANDA-ROF. For the third type of chargers, the charging angles are set to ${ \frac { \pi } { 6 } } , { \frac { \pi } { 5 } } , { \frac { \pi } { 4 } }$ , and ${ \frac { \pi } { 3 } } ;$ and the relative orientations between adjacent antennas are set to ${ \frac { \pi } { 3 } } , { \frac { \pi } { 2 } }$ , and $\frac { \pi } { 2 }$ in WANDA-ROF. Besides, unless otherwise stated, we set $\bar { P _ { t h } = 0 . 0 5 W }$ and $\epsilon _ { 1 } = 0 . 2$

Since there are no existing algorithms concerning the placement problem of multi-antenna chargers, we propose 3 algorithms for the comparison of different setups: Random Positions and Random Orientations (RPRO), Limited Random Positions and Random Orientations (LRPRO), and Limited Random Positions and Greedy Local Utility (LRPGLU). RPPO randomly generates the positions and orientations of all the chargers. LRPRO limits the choices of positions in RPRO by guaranteeing that each charger must be located at one of the coverage areas of sensors. LRPGLU improves LRPRO by greedily selecting the MCSs for each generated position in LRPRO that can maximize the local charging utility.

Moreover, we make some adaptions to 8 related algorithms so as to make them suitable for deploying multi-antenna chargers: IPSCD [86], GPSOCD [87], GACD [88], ICS [89], SMRU [90], ESC-DW [91], NB-GCS [92], and PB-GCS [92]. IPSCD [86] uses the particle swarm optimization (PSO) concept to optimize WRSN charger deployment. The method is divided into two steps: estimating the charging efficiency according to the distance and angle between chargers and sensor nodes, and utilizing the local optimal result and the global optimal result to adjust locations and antenna orientations of chargers to maximize the average utility on the basis of PSO. GPSOCD [87] uses the genetic algorithm (GA) to encode the positions and orientations of the chargers and leverage the PSO algorithm to find the best setup of the chargers. GACD [88] is a GA-based charger deployment algorithm aimed at minimizing the number of deployment chargers. We adopt its coding format and revise its crossover and mutation operations to maintain the validity of solutions. The fitness function is naturally defined as the average charging utility. ICS [89] (which is also a heuristic swarm intelligence algorithm) solves the joint problem of maximizing the received power of the sensor nodes and minimizing the number of charger nodes. We adjust the target function to the average utility to fit our problem. SMRU [90] is a heuristic method that tries to arrange the chargers’ placement to maximize the charging capacity. It leverages a remove strategy that removes the excess chargers and redeploys them to achieve power balance. ESC-DW [91], inspired by the shifting, expansion or shrinkage function of Daubechies wavelet, enhances the sensor coverage with the Daubechies wavelet algorithm to identify the optimal position for the wireless chargers. NB-GCS [92] and PB-GCS [92] are both greedy heuristic algorithms. First, they randomly generate chargers whose apexes are located at grid points. NB-GCS adjusts the direction of each charger iteratively to cover more sensor nodes by tuning its orientation close to another charger, while PB-GCS does the same thing on paired chargers.

Note that all the above 11 algorithms have an ROF (Relative Orientations of antennas are Fixed) edition and ROU (Relative Orientations of antennas are Unfixed) edition for ROF situation and ROU situation, respectively.

![](images/261bf3d9c8831b9189cde23e25b3453078c55f4a628c9531bba7b61c0ce9e396.jpg)  
Fig. 6. Average utility versus -<sub>1</sub> (ROF).

We mainly consider four metrics for evaluation: (average) charging utility, minimum charging utility, power waste, and execution time. Specifically, the average charging utility is defined in (4), which is the average of all utility of the sensors; the minimum utility is defined as the minimum utility of the sensors, that is <sup>min</sup> $U _ { j } ,$ where $1 \leq j \leq M ;$ ; the power waste describes the excessive power received by sensors, which is formalized as $\begin{array} { r } { P _ { t h } \cdot \sum _ { j = 1 } ^ { M } ( \bar { U _ { j } } - 1 ) } \end{array}$ for all $U _ { j } >$ <sup>1</sup>; and the execution time measures the time taken by different algorithms to place all chargers. In the evaluation of each comparison algorithm, we randomly generate 100 scenarios with different positions and/or orientations of sensors, and average the output results to eliminate the impact of input data randomness and algorithm preference. Further, if the algorithm itself has random steps, $e . g .$ , RPRO, we repeat the algorithm for 100 times under each scenario to record its average performance. In the evaluation regarding the execution time, we set the number of sensors to 1000. We implement our algorithms and comparison algorithms in ${ \bf \boldsymbol { \mathsf { C } } } + + { \bf \boldsymbol { \mathsf { \Sigma } } }$ and compile them with g++. All simulations are conducted on a desktop with 16 GB of memory and an Intel i7-8700 processor.

## B. Performance Results

The performance results are as follows.

Impact of error threshold $\epsilon _ { 1 } .$ Our simulation results show that on average, WANDA respectively outperforms RPRO, LRPRO, and LRPGLU by 236%, 199%, and 86% for ROF situation and by 206%, 170%, and 64% for ROU situation in terms of $\epsilon _ { 1 }$ Figs. 6 and 10 show that the error threshold $\epsilon _ { 1 }$ has no significant impact on the charging utility of WANDA. Since the number of sensors and the charging distance of chargers are sufficiently large, the charging utility of RPRO and LRPRO are close. Moreover, LRPGLU outperforms LRPRO because it adopts the idea of MCS in terms of selecting orientations of chargers.

Impact of power threshold for charging utility function $P _ { \mathit { t h } } .$ Our simulation results show that on average, WANDA respectively outperforms RPRO, LRPRO, and LRPGLU by 291%, 219%, and 91% for ROF situation and by 331%, 276%, and 54% for ROU situation in terms of $P _ { t h }$ . Figs. 7 and 11 show that the charging utility of WANDA decreases monotonically with $P _ { t h }$ since less utility can be gained with a larger $P _ { t h }$ under the same amount of received power of sensors.

Impact of charging distance of chargers $D ;$ Our simulation results show that on average, WANDA respectively outperforms RPRO, LRPRO, and LRPGLU by 237%, 188%, and 83% for

![](images/b58d222b584fe252b2c32d5310c8e9b1b5a29fa508fcd816620b40a39872ba66.jpg)  
Fig. 7. Average utility versus $P _ { t h }$ (ROF).

![](images/770398c238e4419b02f2eb199023980b04c9b1a5a7039f18545dafb7b744d58f.jpg)  
Fig. 11. Average utility versus $P _ { t h }$ (ROU).

![](images/710e76fbfc11c921fb1fdf71adc7ca91b65be319cc3ac66e2a2e4d75e9c7dcaa.jpg)

![](images/68f59fda70ac64ed9f6171c47586ba31427e81f393f361fd80cd60ae86a83b6b.jpg)

Fig. 8. Average utility versus D (ROF).  
![](images/36814a58e910de0bcca147604b69219ba9c671f1a2a56bad4c41691431e8d7b5.jpg)  
Fig. 9. Average utility versus α (ROF).

Fig. 12. Average utility versus D (ROU).  
![](images/e36d14954b4c1e2709e9ac4d190bfa0c21771164d59c1162e94487abd46137d8.jpg)

![](images/0d385ea1814f2bed387006af154966c3558cdd87771fe525df1e95492924b9c2.jpg)  
Fig. 10. Average utility versus $\epsilon _ { 1 }$ (ROU).

Fig. 13. Average utility versus α (ROU).  
![](images/756032189745e5d98c0c8dc6e6042d3a386c74b14bd34bb0221594bb1b86c692.jpg)  
Fig. 14. Average utility versus charger-to-sensor-ratio (ROF).

ROF situation and by 217%, 179%, and 67% for ROU situation in terms of <sup>D</sup>. With a longer charging distance, more choices can be provided to chargers to further improve the charging utility of WANDA. Figs. 8 and 12 show that the charging utility of WANDA increases monotonically with <sup>D</sup> increases while the charging utility of RPRO, LRPRO, and LRPGLU change not too much. The increase of charging utility of WANDA becomes slow when the charging distance increases to 1.5 times of initial setting.

Impact of constants in the charging model <sup>α</sup>: Our simulation results show that on average, WANDA respectively outperforms

RPRO, LRPRO, and LRPGLU by 241%, 203%, and 87% for ROF situation and by 219%, 180%, and 63% for ROU situation in terms of <sup>α</sup>. Figs. 9 and 13 show that the charging utility of four algorithms increase at a fast speed and then keep constant when <sup>α</sup> changes to 1.25 times of initial setting.

Average utility: Our simulation results show that on average, WANDA respectively outperforms IPSCD, GPSOCD, GACD, ICS, SMRU, ESC-DW, NB-GCS, and PB-GCS by 32%, 23%, 31%, 89%, 16%, 29%, 102%, and 42% for ROF situation and by 32%, 16%, 24%, 93%, 12%, 36%, 124%, and 80% for ROU situation in terms of the average utility of all sensors. Figs. 14 and 18 show that the average charging utility of each comparison algorithm jitters more violently than that of WANDA with the increase of chargers. This is in line with our expectation since some of these algorithms are heuristic methods, which provide unstable solutions based on searching, and the others are local optimization algorithms, which easily drop into local optima and thus result in an uncertain global result.

![](images/119cd9d1b28b97023e1b3fd70fa23332bf7d45357907473c67ad0f9aa2498472.jpg)  
Fig. 15. Minimum utility versus charger-to-sensor-ratio (ROF).

![](images/786f419e088a93c4c1b8859979e3188637b099ff00319154387dd4d8d66f070a.jpg)  
Fig. 16. Power waste versus charger-to-sensor-ratio (ROF).

![](images/9511f24fcefa5d23cd439f388ca63878c5d66a5cd25cfe41d1cde3deb853e3c0.jpg)  
Fig. 17. Execution time versus charger-to-sensor-ratio (ROF).

![](images/f7a578d9053ba9e1a4e0b2fcbab9370928f3b3c1f19a19124fe548fb11200a34.jpg)  
Fig. 18. Average utility versus charger-to-sensor-ratio (ROU).

Minimum utility: Our simulation results show that on average, WANDA respectively outperforms IPSCD, GPSOCD, GACD, ICS, SMRU, ESC-DW, NB-GCS, and PB-GCS by 279%, 76%, 125%, 700%, 53%, 380%, 800%, and 260% for ROF situation and by 184%, 19%, 103%, 714%, 16%, 235%, 783%, and 469% for ROU situation in terms of the minimum utility of all sensors. Figs. 15 and 19 show that the minimum charging utility of each comparison algorithm is smaller than that of WANDA. This means that our algorithm can achieve better fairness of charging.

![](images/8ab466bb3c404d53624e34098e353d4bfb8e17536b326cc94f229508a2248bb7.jpg)  
Fig. 19. Minimum utility versus charger-to-sensor-ratio (ROU).

![](images/cfaad30fd938e8658e027c826225bbcb82f0b6811bbe1598afd5c15894776f18.jpg)  
Fig. 20. Power waste versus charger-to-sensor-ratio (ROU).

![](images/4c9458735b36c706f50a3705898928935c692f270f560136ef538c759771030e.jpg)  
Fig. 21. Execution time versus charger-to-sensor-ratio (ROU).

Power waste: Our simulation results show that on average, WANDA respectively outperforms IPSCD, GPSOCD, GACD, ICS, SMRU, ESC-DW, NB-GCS, and PB-GCS by 50%, 6%, 46%, 22%, 38%, 4%, 43%, and 51% for ROF situation and by 67%, 58%, 67%, 42%, 59%, 55%, 71%, and 34% for ROU situation in temrs of the overflowing energy of all sensors. Figs. 16 and 20 show that the power waste of each comparison algorithm is larger than that of WANDA. This means that our algorithm can alleviate the problem of overcharging, and thus makes the charging process more safe and energy-saving.

Execution time: Our simulation results show that on average, WANDA respectively outperforms IPSCD, GPSOCD, GACD, ICS, SMRU, ESC-DW, NB-GCS, and PB-GCS by <sup>138</sup>×, <sup>153</sup>×, <sup>1277</sup>×, <sup>215</sup>×, <sup>30</sup>×, <sup>61</sup>×, <sup>45</sup>×, and <sup>48</sup>× for ROF situation and by <sup>69</sup>×, <sup>75</sup>×, <sup>619</sup>×, <sup>103</sup>×, <sup>17</sup>×, <sup>33</sup>×, <sup>24</sup>×, and <sup>29</sup>× for ROU situation in terms of the execution time of placing chargers. Figs. 17 and 21 show that the execution time of WANDA is more stable than that of the other algorithms. It can be attributed to our rule-based placement process.

![](images/779a17e56fd8f71c9f24b2f67c7cdba1fbc86eb536fb03bee37c235bf4151e79.jpg)  
Fig. 22. Insights (ROF).

![](images/4c733255630b1619b97e88d6ced169f015fc141a975cb949e5e46e91eb4e8987.jpg)  
Fig. 23. Insights (ROU).

![](images/6d47614205e6fcac13f34c0a36cf46f3134833ad631066ebe7f2a38695aa8c87.jpg)  
Fig. 24. Utility of different algorithms (ROF).

![](images/f6b49d5b3607964af78e3953a18c84f2dcf08361924d5441d40d12559c1c66cc.jpg)  
Fig. 25. Utility of different algorithms (ROU).

## C. Performance Comparison

The results of the various experiments provide strong support for our rule-based methods, leading to several valuable conclusions. First, meta-heuristic approaches are not necessarily superior to our rule-based methods, and the performance diagrams concerning utility can illustrate this point. Second, these metaheuristic methods have no performance guarantee, which means that they could provide a quite poor solution unpredictably. This is illustrated by the utility distribution in Figs. 24 and 25, where we draw the utility of different algorithms with 10 sensors and

6 chargers under 100 random sensor network topologies. The result shows that our proposed WANDA has distribution concentration of utility, which means that our algorithm is more robust in dealing with various scenarios. No performance guarantee is not only a theoretical flaw, but can also lead to disasters in real life. For instance, the nodes in surveillance wireless sensor networks need enough power (i.e., utility) to detect unauthorized intrusions [85], and a bad deployment strategy will cause the system to fail. Third, these meta-heuristic methods usually have no time complexity bound (due to their heuristic search characteristic), and consume too much time to search for an acceptable result. Indeed, the uncertainties in performance and running time motivate us to propose the performance-guaranteed and time-bounded scheme WANDA in this paper.

## D. Insights

In this subsection, we study the impact of the uniformity of sensor’s position distribution on the performance of WANDA-ROF and WANDA-ROU. We randomly generate the position for each sensor with both <sup>x</sup>-coordinate and <sup>y</sup>-coordinate following a Gaussian distribution with $\mu = 2 0$ . We vary the standard deviation $\sigma _ { x }$ and $\sigma _ { y }$ both from 4 to 19 and conduct experiments of each set for 100 times. Figs. 22 and 23 show that the performance of both WANDA-ROF and WANDA-ROU decreases monotonically when $\sigma _ { x }$ or $\sigma _ { y }$ increases. This is because with a larger standard deviation, the distribution of positions of sensors is more likely to be sparser, and chargers are harder to find orientations that cover more sensors.

## VII. FIELD EXPERIMENTS

In this section, we conduct field experiments to validate the performance of our proposed algorithms.

## A. Testbed

In the experiments, we use three types of chargers for trasmitting power, two types of rechargeable sensors for receiving power, and one wireless AP connecting to a laptop to receive message from sensors. The type of chargers is Powercast TX91501 [86] with single embedded directional antennas. As no multi-antenna chargers are available, we use three types of single-directional chargers to achieve the same effect. We place them in the desired location with appropriate directions and plug the DC connector of the power supply into the connectors on the chargers. When the green LED illuminates, the chargers are active and transmitting radio frequency (RF) energy. The power receiver is a TB-Powercast rechargeable wireless sensor node [87], which can independently collect RF energy in space environment and convert it into DC power for nodes without power supply. Besides, it has the ability to perceive environment parameters, process data, and communicate as common sensor nodes. Finally, we use an wireless AP named MRF24J40 from Mircochip [88] to collect the charging power information from the sensor node and report it to a host machine (a laptop) through a data connection wire. The devices used in our experiments are shown in Fig. 26. There are 3 types of multi-antenna chargers, and the numbers of them are 1, 2, and 3. The numbers of the 2 types of sensors are 3 and 4, which are randomly located in a <sup>230</sup> <sup>cm</sup> × <sup>230</sup> <sup>cm</sup> square area. The <sup>x</sup>-coordinate, <sup>y</sup>-coordinate, and orientation of sensors, are (0.85,2.15,69.59), (2.71,2.20,172.75), (1.66,2.91,296.22), (2.57,1.25,309.22), (2.48,2.03,109.43), (0.88,1.06,131.20) and (1.19,1.23,355.87). To effectively utilize our devices, we limit the distance between charger and sensor with <sup>60 cm</sup> to <sup>160 cm</sup>.

(a)  
(d)  
![](images/98e061755b67872ab9ecc86a97d9aa04f1415b583eb8b4e4bed908c33891f2da.jpg)

![](images/173f2c88bbd41f41373a66888ca1408e7be91164ad8c8f35ded5caf33e516191.jpg)  
(b)

![](images/50e947648e0a3fabff921915fecaa499bc8f33ffade61dbb03dbfee58360e92e.jpg)  
(c)

![](images/09ba075659a15fc1ae6edc26caf5c9210818ba9e6cf16b09bdef930136bc82a2.jpg)

Fig. 26. Testbed: (a) charger type 1&2; (b) charger type 3; (c) sensor type 1; (d) sensor type 2; and (e) an AP connected to a laptop.  
![](images/af9e6cbc6c7d9126a739226d364222eb78cc897ac8e12e20aa5de1ab1057d5da.jpg)

![](images/c317e69607a6012d682a12bf298f32298fcca8148ee2504f370eb94ec12d455d.jpg)  
Fig. 27. Strategies (ROF).

![](images/27617c1db60bb9875a4d6503167b456ec2709c085b388530c9ac98b423cb8255.jpg)  
Fig. 28. Strategies (ROU).

## B. Experimental Results

![](images/4726c850143d28c9bc128a3c66bc854ed4d1d4d8b6cd3c01a8db1a3b90e16eae.jpg)

![](images/25ed97f32dd17c2e33203b086424424b268490a89e2528bfa056ffa936a43e59.jpg)  
Fig. 29. Utility (ROF).

As illustrated in Figs. 27 and 28, the strategies for WANDA, RPRO, and LRPGLU for ROF and ROU situation are shown in the red, blue, and green arrows, respectively. Despite that the selected positions of three algorithms are close to each other, WANDA can accurately select the most advantageous positions and orientations for chargers simultaneously by making a balance among the charging utility of different sensors, as shown in Figs. 29 and 31. Besides, Figs. 30 and 32 show the charging power CDF of sensors for the three algorithms, and WANDA respectively outperforms RPRO and LRPGLU by 63% and 22% for ROF situation and by 68% and 30% for ROU situation.

Fig. 30. CDF (ROF).  
![](images/2c49869b3b2df5d275500c53c7031a2fbe5b2d67c7ede350c6ff2350ffad1b27.jpg)

Fig. 31. Utility (ROU).  
![](images/26f2c8b50d4ec4c07ce105f9b944e5f8347a03cf148aef8f0b22a4386d5c2963.jpg)  
Fig. 32. CDF (ROU).

## VIII. DISCUSSION

## A. Charger Redeployment

In this subsection, we discuss the charger redeployment problem. The problem is described as follows: when the topology of devices dynamically changes, how to schedule chargers so that the cost of switching their positions and orientations from the previous strategies to the new ones is minimized. A simple way is performing our solution for WANDA two times for the original topology and the new one, and then finding the relationship between them. To begin with, we clarify the concept of mapping.

Definition 8. (Mapping): Given a set of chargers and its two charger schemes $\mathcal { X } _ { 1 }$ and $\mathcal { X } _ { 2 } .$ strategy $s _ { i } \in \mathcal { X } _ { 1 }$ can be mapped to strategy $s _ { j } \in \mathcal { X } _ { 2 }$ if and only if we can place $s _ { i }$ at $s _ { j }$ and rotate <sup>s</sup>i to provide power not less than $s _ { j } ;$ scheme $\mathcal { X } _ { 1 }$ can be mapped to $\mathcal { X } _ { 2 }$ if and only if there is a bijection from $\mathcal { X } _ { 1 }$ to $\mathcal { X } _ { 2 }$

![](images/917d2ddbe5749d0e66aa1acb6745ab2b9b357770755dce04e0229986d99a114b.jpg)  
Fig. 33. Redeployment for WANDA (ROF and ROU). (a) Map a charger with two antennas to a charger with a single antenna. (b) For ROF, we rotate the charger and detect whether the orientation satisfies the charging constraint. (c) For ROU, we rotate each antenna and detect whether the orientation satisfies the charging constraint.

As per the definition of mapping, it is clear that a strategy of a charger can be mapped to another one of a charger of different type. For example, in Fig. 33(a), we map a charger with two antennas to a charger with a single antenna, and provide not less power for the sensor. Obviously, this mapping will not reduce the utility.

Formally, the charger redeployment problem can be formulated into a weighted bipartite graph perfect matching problem. Give a complete bipartite graph $G ( U , V , E )$ where $| U | = | V | =$ <sup>N</sup> and there is a weight $w _ { i j }$ for the edge connecting the <sup>i</sup>-th vertex in <sup>U</sup> and the $j \mathrm { - t h }$ vertex in <sup>V</sup> . Here, <sup>N</sup> is the number of chargers, $U$ and $V$ are the original strategy set and the new one, respectively. Our task is to find a perfect matching for all the vertices such that the total value of weights of the selected edges is minimized. First, we measure the switching cost between any two chargers. After that, the matching problem can be then easily addressed by the well-known Hungarian algorithm [89], [90].

1) Redeployment $f o r$ WANDA-ROF: We assume that $s _ { i }$ is mapped to $s _ { x } ,$ the position is changed to $s _ { i } ^ { \prime } ,$ and the orientation needs to change from $\theta _ { i }$ to $\theta _ { i } ^ { \prime }$ to provide enough power for surrounding sensors. For WANDA-ROF, the switching cost can be formulated as

$$
c _ {R O F} \left(s _ {i}, s _ {x}\right) = f _ {d} \left(\| s _ {i} s _ {i} ^ {\prime} \|\right) + f _ {\theta} \left(<   \theta_ {i}, \theta_ {i} ^ {\prime} >\right),\tag{19}
$$

where $\| s _ { i } s _ { i } ^ { \prime } \|$ and $< \theta _ { i } , \theta _ { i } ^ { \prime } >$ denote the traveling distance and the rotating angle, respectively; $f _ { d } ( \cdot )$ and $f _ { \theta } ( \cdot )$ are the monotone increasing functions of the traveling distance and the rotating angle, respectively. Thus, the problem can be formulated as follows.

$$
\begin{array}{l l} \min & c _ {R O F} \left(s _ {i}, s _ {x}\right), \\ s. t. & \sum_ {b = 1} ^ {B _ {i}} P \left(s _ {i} ^ {\prime}, \theta_ {i} ^ {\prime b}, o _ {j}, \phi_ {j}\right) \geq \sum_ {b = 1} ^ {B _ {x}} P \left(s _ {x}, \theta_ {x} ^ {b}, o _ {j}, \phi_ {j}\right), \\ & j \in \{1, 2, \ldots , M \}. \end{array}
$$

Here the decision variable is $\theta _ { i } ^ { \prime } = \{ \theta _ { i } ^ { \prime 1 } , . . . , \theta _ { i } ^ { \prime B _ { i } } \}$ , i.e., the orientation strategy of $s _ { i }$ after being placed at $s _ { i } ^ { \prime } .$ . Note that the solution occurs only if some charging sector edge touches a sensor. Therefore, we can rotate the charger anticlockwise and clockwise (until <sup>π</sup>) and detect whether the orientation satisfies the charging constraint, as shown in Fig. 33(b). The time complexity is proportional to the number of sensors charged by $s _ { x }$ and the number of mapping, i.e., $O ( { \frac { M } { N } } \cdot N ^ { 2 } )$ . When there is no $\theta _ { i } ^ { \prime }$ satisfies the charging constraint, we say that there is no feasible mapping and we record the cost as infinity (∞).

![](images/fa993fb54fc44e4d8940b44c60d244610b3aebfc35a2744a22754c44e8c2be58.jpg)  
Fig. 34. Bipartile graph for two schemes of chargers.

After measuring the cost between each vertex in <sup>U</sup> and $V .$ , we can conduct the Hungarian algorithm to find the perfect matching with the minimum cost. The solution always exists provided that we establish a mapping between chargers of the same type. For example, in Fig. 34, $[ s _ { 1 } \to s _ { 1 } ^ { \prime } , s _ { 2 } \to s _ { 2 } ^ { \prime } , s _ { 3 } \to s _ { 3 } ^ { \prime } ]$ is a valid mapping, and $[ s _ { 1 }  s _ { 2 } ^ { \prime } , s _ { 2 }  s _ { 3 } ^ { \prime } , s _ { 3 }  s _ { 1 } ^ { \prime } ]$ is another one. The time complexity of the Hungarian algorithm is $O ( N ^ { 3 } )$ in the worst case, and the total time complexity is $O ( M N + \dot { N } ^ { 3 } )$

2) Redeployment for WANDA-ROU: In this case, we assume that $s _ { i }$ is mapped to $s _ { x }$ as the same as WANDA-ROF. The switching cost can be formulated as

$$
c _ {R O U} \left(s _ {i}, s _ {x}\right) = f _ {d} \left(\| s _ {i} s _ {i} ^ {\prime} \|\right) + \sum_ {b = 1} ^ {B _ {i}} f _ {\theta} \left(<   \theta_ {i} ^ {b}, \theta_ {i} ^ {\prime b} >\right),\tag{20}
$$

where $\| s _ { i } s _ { i } ^ { \prime } \|$ and $< \theta _ { i } ^ { b } , \theta _ { i } ^ { \prime b } >$ denote the traveling distance and the rotating angle, respectively; $f _ { d } ( \cdot )$ and $f _ { \theta } ( \cdot )$ are the same as that in ROF. Thus, our optimization problem is

$$
\begin{array}{l l} \min & c _ {R O U} \left(s _ {i}, s _ {x}\right), \\ s. t. & \sum_ {b = 1} ^ {B _ {i}} P \left(s _ {i} ^ {\prime}, \theta_ {i} ^ {\prime b}, o _ {j}, \phi_ {j}\right) \geq \sum_ {b = 1} ^ {B _ {x}} P \left(s _ {x}, \theta_ {x} ^ {b}, o _ {j}, \phi_ {j}\right), \\ & j \in \{1, 2, \ldots , M \}. \end{array}
$$

Here the decision variable is $\theta _ { i } ^ { \prime } = \{ \theta _ { i } ^ { \prime 1 } , . . . , \theta _ { i } ^ { \prime B _ { i } } \}$ . Also, the solution occurs only if each charging sector edge touches a sensor. Therefore, we can rotate each antenna of the charger anticlockwise and clockwise (until $\pi )$ and detect whether the orientation satisfies charging constraint, as shown in Fig. 33(c). The time complexity is proportional to the number of sensors charged by $s _ { x } ,$ , the number of antennas, and the number of mapping. Therefore the time complexity is $\begin{array} { r l r } { \mathrm { ~ } } & { { } } & { O \big ( \frac { M } { N } \cdot N ^ { 2 } \cdot B \big ) } \end{array}$ where $B = \operatorname* { m a x } \{ B _ { i } \mid i = 1 , 2 , \ldots , N \}$ . The process for the situation that there is no $\theta _ { i } ^ { \prime }$ satisfies the charging constraint is the same as that for ROF. Finally, the total time complexity is $O ( M N B + N ^ { 3 } )$ .

## B. Charger Deployment With Obstacles

In some situations, the line-of-sight transmitting power from chargers can be blocked by obstacles of any shape without reflection. Therefore, we need to consider the problem of placing wireless chargers with multiple antennas in the environment of obstacles. In [83], Wang et al. considered the problem of practical heterogeneous wireless charger placement with obstacles. Similar to their work, we suppose there exist $T$ static obstacles $H = \{ H _ { 1 } , H _ { 2 } , \dots , H _ { T } \}$ . These obstacles prevent the placement of any device or charger inside them, and charging power cannot pass through or reflect off their surface. Thus, the charging power of sensor $o _ { j }$ from the <sup>b</sup>-th orientation of the charger $s _ { i }$ considering obstacles is reformulated as

![](images/ac4e286704ed6900fa130f95e1aca10451dd2d693c57696ccc2a4078983b1754.jpg)  
Fig. 35. Feasible areas with obstacles.

$$
\begin{array}{l} P (s _ {i}, \theta_ {i} ^ {b}, o _ {j}, \phi_ {j}) \\ = \left\{ \begin{array}{c} \frac {\alpha_ {i j}}{(| | s _ {i} o _ {j} | | + \beta_ {i j}) ^ {2}}, \qquad 0 \leq \| s _ {i} o _ {j} \| \leq D _ {i} ^ {b}, \\ \overrightarrow {o _ {j} s _ {i}} \cdot \overrightarrow {r _ {\phi_ {j}}} - \| o _ {j} s _ {i} \| c o s (A _ {o _ {j}} / 2) \geq 0, \\ \overrightarrow {s _ {i} o _ {j}} \cdot \overrightarrow {r _ {\theta_ {i} ^ {b}}} - \| s _ {i} o _ {j} \| c o s (A _ {s _ {i}} ^ {b} / 2) \geq 0, \\ \text { and } s _ {i} o _ {j} \cap h _ {k} = \emptyset , \forall k \in \{1, 2, \ldots , T \}, \\ 0, \qquad \text { otherwise }. \end{array} \right. \end{array}\tag{21}
$$

Note that the condition $s _ { i } o _ { j } \cap h _ { k } = \emptyset$ reflects the requirement that at any time the line connecting $s _ { i }$ and $o _ { j }$ should not cross any obstacle. In the process of area discretization, we divide the area according to the position of obstacles, and ignore impossible parts. For example, in Fig. 35, the dark parts of the sector is “dead” areas, where chargers in these parts can never provide power for the sensor $o _ { j }$ . Therefore, we can safely ignore these parts when considering placing chargers for charging $o _ { j }$

## C. Charger Deployment With Cost

For the problem of charger deployment in a target area, we can often pre-determine a set of candidate locations for deploying wireless chargers, as Algorithm 1 for WANDA-ROF and Algorithm 4 for WANDA-ROU do. However, in some cases we have to pay for placing chargers at specific locations. The cost may come from many aspects, such as the expense to reach the location, the rental cost, or the maintenance cost. Undoubtedly, such cost should be taken into consideration in the wireless charger placement problem, and thus how to maximize charging utility with limited deployment cost budget becomes a realistic and important problem.

Suppose the cost of placing a charger at its candidate position $p _ { i }$ is $c ( p _ { i } )$ , and the cost budget is <sup>C</sup>. The utility maximum problem with limited cost for WANDA can be formalized as

$$
\begin{array}{l l} \max & \frac {1}{M} \sum_ {j = 1} ^ {M} U _ {j} \left(\sum_ {(s _ {i}, \theta_ {i}) \in \Pi} x _ {i} \sum_ {b = 1} ^ {B _ {i}} P (s _ {i}, \theta_ {i} ^ {b}, o _ {j}, \phi_ {j})\right), \\ s. t. & \sum_ {(s _ {i}, \theta_ {i}) \in \Pi} x _ {i} c (s _ {i}) \leq C, \sum_ {(s _ {i}, \theta_ {i}) \in \Pi_ {r}} x _ {i} = N _ {r}, x _ {i} \in \{0, 1 \}, \end{array}
$$

where <sup>Π</sup> is the candidate position set output by Algorithm 1 for WANDA-ROF or Algorithm 4 for WANDA-ROU.

Thus, we transform this problem into an LPD problem proposed in [31], which can be solved by an enumeration-based greedy algorithm.

## IX. CONCLUSION

In this paper, we have studied the placement problem of multiantenna chargers. Our key contributions are first considering the deployment of multi-antenna chargers and proposing algorithms for both ROF and ROU situation. To address this problem, we first presented a piecewise constant function to approximate the nonlinear relationship between charging power and charging distance, based on which an area discretization technique was proposed. Next, we presented algorithms for both WANDA-ROF and WANDA-ROU, respectively. For WANDA-ROF, we proposed a MCS extraction method and transformed WANDA to the problem of maximizing a monotone submodular function subject to a partition matroid constraint and presented a $\frac { 1 } { 2 } - \epsilon$ approximation algorithm. For WANDA-ROU, we constructed a candidate position set for each type of chargers and then proposed a novel two-level submodular optimization scheme to address it, which achieves an approximation ratio of $\begin{array} { r } { \frac { 1 } { 6 } - \epsilon . } \end{array}$ Simulation and experimental results show that our algorithms outperform comparison algorithms by at least 16% for ROF situation and 12% for ROU situation.

Our proposed algorithms try to arrange the chargers’ positions and orientations to maximize the overall utility. Meanwhile, they can be adapted to addressing the problem of minimizing the number of chargers while meeting given charging utility. This can be achieved by continuously increasing the number of chargers and executing our algorithm until the conditions are met. In addition, there are some future work worth exploring. For example, we can dynamically allocate the charging power in different directions to save energy. $\mathrm { O r } ,$ we can adjust the parameters of chargers to change the charging characteristics to adapt to the actual situation. In addition, outside the charging model we used in Section III, there are other charging models such as vector models [76], [77], [78] which can capture superadditive and cancellation effects or MIMO models [32], [34], [71] which can be combined with the beamforming technique to improve communication and energy transmission efficiency. The charging utility is also a research point under those models or scenarios. Other topics such as charger moving or charger communication are valuable directions waiting for us to dive in.

## REFERENCES

[1] Brown and C. William, “The history of power transmission by radio waves,” IEEE Trans. Microw. Theory Techn., vol. 32, no. 9, pp. 1230–1242, 1984.

[2] H. Jiang, P. Zhao, and C. Wang, “RobLoP: Towards robust privacy preserving against location dependent attacks in continuous LBS queries,” IEEE/ACM Trans. Netw., vol. 26, no. 2, pp. 1018–1032, Apr. 2018.

[3] X. Ma, H. Wang, H. Li, J. Liu, and H. Jiang, “Exploring sharing patterns for video recommendation on YouTube-like social media,” Multimedia Syst., vol. 20, no. 6, pp. 675–691, 2014.

[4] T. Imura, H. Okabe, and Y. Hori, “Basic experimental study on helical antennas of wireless power transfer for electric vehicles by using magnetic resonant couplings,” in Proc. Veh. Power Propulsion Conf., 2009, pp. 936–940.

[5] S. Li and C. C. Mi, “Wireless power transfer for electric vehicle applications,” J. Emerg. Sel. Top. Power Electron., vol. 3, no. 1, pp. 4–17, 2014.

[6] A. Ahmad, M. S. Alam, and R. Chabaan, “A comprehensive review of wireless charging technologies for electric vehicles,” IEEE Trans. Transp. Electrific., vol. 4, no. 1, pp. 38–63, Mar. 2018.

[7] Y. Huang, Z. Xiao, D. Wang, H. Jiang, and D. Wu, “Exploring individua travel patterns across private car trajectory data,” IEEE Trans. Intell. Transp. Syst., vol. 21, no. 12, pp. 5036–5050, Dec. 2020.

[8] Y. Wang, Z. Su, N. Zhang, and R. Li, “Mobile wireless rechargeable UAV networks: Challenges and solutions,” Commun. Mag., vol. 60, no. 3, pp. 33–39, 2022.

[9] J. Park et al., “A resonant reactive shielding for planar wireless power transfer system in smartphone application,” IEEE Trans. Electromagn. Compat., vol. 59, no. 2, pp. 695–703, Apr. 2017.

[10] J.-W. Teng and H.-H. Fu, “Electronic wrist watch having wireless charging function,” US Patent 8,908,479, 2014.

[11] S. Hui, “Planar wireless charging technology for portable electronic products and Qi,” Proc. IEEE, vol. 101, no. 6, pp. 1290–1301, Jun. 2013.

[12] M. Kiani and M. Ghovanloo, “An RFID-based closed-loop wireless power transmission system for biomedical applications,” IEEE Trans. Circuits Syst. II, Express Briefs, vol. 57, no. 4, pp. 260–264, Apr. 2010.

[13] D. Arnitz and M. S. Reynolds, “Multitransmitter wireless power transfer optimization for backscatter RFID transponders,” Antennas Wirel. Propag. Lett., vol. 12, no. 1, pp. 849–852, 2013.

[14] R. W. Boss, “An overview of RFID,” Library Technol. Rep., vol. 39, no. 6, pp. 7–17, 2009.

[15] C. Bergsrud and J. Straub, “A space-to-space microwave wireless power transmission experiential mission using small satellites,” Acta Astronautica, vol. 103, no. 10, pp. 193–203, 2014.

[16] D. Shi, L. Zhang, H. Ma, Z. Wang, Y. Wang, and Z. Cui, “Research on wireless power transmission system between satellites,” in Proc. Wirel. Power Transfer Conf., 2016, pp. 1–4.

[17] A. Tomar and S. Gupta, “Wireless power transmission: Applications and components,” Int. J. Eng., vol. 1, no. 5, pp. 1–8, 2012.

[18] X. Deng, B. Wang, W. Liu, and L. T. Yang, “Sensor scheduling for multimodal confident information coverage in sensor networks,” IEEE Trans. Parallel Distrib. Syst., vol. 26, no. 3, pp. 902–913, Mar. 2015.

[19] H. Jiang, J. Cheng, D. Wang, C. Wang, and G. Tan, “Continuous multidimensional top-k query processing in sensor networks,” in Proc. Int. Conf. Comput. Commun., 2011, pp. 793–801.

[20] S. Wang et al., “Energy efficient broadcasting using network coding aware protocol in wireless ad hoc network,” in Proc. Int. Conf. Commun., 2011, pp. 1–5.

[21] X. Deng, Y. Jiang, L. T. Yang, M. Lin, L. Yi, and M. Wang, “Data fusion based coverage optimization in heterogeneous sensor networks: A survey,” Inf. Fusion, vol. 52, no. 1, pp. 90–105, 2019.

[22] H. Jiang, A. Iyengar, E. Nahum, W. Segmuller, A. Tantawi, and C. P. Wright, “Load balancing for sip server clusters,” in Proc. Int. Conf. Comput. Commun., 2009, pp. 2286–2294.

[23] F. Laricchia, “Global unit shipments of wireless charging transmitters & receivers 2020–2030,” 2023. [Online]. Available: https://www.statista. com/statistics/681403/wireless-charging-transmitter-receiver-units/

[24] X. Lu, P. Wang, D. Niyato, D. I. Kim, and Z. Han, “Wireless charging technologies: Fundamentals, standards, and network applications,” Commun. Surv. Tut., vol. 18, no. 2, pp. 1413–1452, 2015.

[25] S. He, K. Shi, C. Liu, B. Guo, J. Chen, and Z. Shi, “Collaborative sensing in Internet of Things: A comprehensive survey,” Commun. Surv. Tut., vol. 24, no. 3, pp. 1435–1474, 2022.

[26] L. Liu, R. Zhang, and K.-C. Chua, “Secrecy wireless information and power transfer with MISO beamforming,” IEEE Trans. Signal Process., vol. 62, no. 7, pp. 1850–1863, Apr. 2014.

[27] H. Dai et al., “Placing wireless chargers with multiple antennas,” in Proc. Int. Conf. Sens. Commun. Netw., 2022, pp. 479–487.

[28] H. Dai, X. Wang, A. X. Liu, H. Ma, and G. Chen, “Optimizing wireless charger placement for directional charging,” in Proc. Int. Conf. Comput. Commun., 2017, pp. 1–9.

[29] H. Dai, X. Wang, A. X. Liu, H. Ma, G. Chen, and W. Dou, “Wireless charger placement for directional charging,” IEEE Trans. Netw., vol. 26, no. 4, pp. 1865–1878, Aug. 2018.

[30] X. Wang et al., “Heterogeneous wireless charger placement with obstacles,” in Proc. Int. Conf. Parallel Process., 2018, pp. 1–10.

[31] X. Ding et al., “Optimal charger placement for wireless power transfer,” Comput. Netw., vol. 170, no. 1, pp. 107–123, 2020.

[32] R. Zhang and C. K. Ho, “MIMO broadcasting for simultaneous wireless information and power transfer,” IEEE Trans. Wirel. Commun., vol. 12, no. 5, pp. 1989–2001, May 2013.

[33] L. Liu, R. Zhang, and K.-C. Chua, “Multi-antenna wireless powered communication with energy beamforming,” IEEE Trans. Commun., vol. 62, no. 12, pp. 4349–4361, Dec. 2014.

[34] G. Yang, C. K. Ho, R. Zhang, and Y. L. Guan, “Throughput optimization for massive MIMO systems powered by wireless energy transfer,” J. Sel. Areas Commun., vol. 33, no. 8, pp. 1640–1650, 2015.

[35] X. Lu, P. Wang, D. Niyato, D. I. Kim, and Z. Han, “Wireless networks with RF energy harvesting: A contemporary survey,” Commun. Surv. Tut., vol. 17, no. 2, pp. 757–789, 2014.

[36] C. Lee, W. Na, G. Jang, C. Lee, and S. Cho, “Energy-efficient and delayminimizing charging method with a multiple directional mobile charger,” Internet Things J., vol. 8, no. 10, pp. 8291–8303, 2020.

[37] Wattup, “Wattup smart glasses developer kit,” 2021. [Online]. Available: https://energous.com/files/documents/DeveloperKit-Briefs/WattUp-SmartGlassesDeveloper-Kit-Briefs\_Aug30--2021.pdf

[38] E. Products, “Receiver chip targets wireless charging of small electronic devices,” 2019. [Online]. Available: https://www.electronicproducts.com/ receiver-chip-targets-wireless-charging-of-small-electronic-devices/

[39] H. J. Visser, “Triangulation-like approach to wireless charging,” 2019. [Online]. Available: https://www.imec-int.com/en/imec-magazine/imecmagazine-june-2019/triangulation-like-approach-to-wireless-charging

[40] M. Y. Naderi, K. R. Chowdhury, and S. Basagni, “Wireless sensor networks with RF energy harvesting: Energy models and analysis,” in Proc. Wirel. Commun. Netw. Conf., 2015, pp. 1494–1499.

[41] A. Kaswan, P. K. Jana, and S. K. Das, “A survey on mobile charging techniques in wireless rechargeable sensor networks,” Commun. Surv. Tut., vol. 24, no. 3, pp. 1750–1779, 2022.

[42] H. Feng, R. Tavakoli, O. C. Onar, and Z. Pantic, “Advances in high-power wireless charging systems: Overview and design considerations,” IEEE Trans. Transp. Electrific., vol. 6, no. 3, pp. 886–919, Sep. 2020.

[43] C. Lin, Y. Zhou, F. Ma, J. Deng, L. Wang, and G. Wu, “Minimizing charging delay for directional charging in wireless rechargeable sensor networks,” in Proc. Int. Conf. Comput. Commun., 2019, pp. 1819–1827.

[44] L. Fu, P. Cheng, Y. Gu, J. Chen, and T. He, “Optimal charging in wireless rechargeable sensor networks,” IEEE Trans. on Veh. Technol., vol. 65, no. 1, pp. 278–291, Jan. 2016.

[45] C. Lin, J. Zhou, C. Guo, H. Song, G. Wu, and M. S. Obaidat, “TSCA: A temporal-spatial real-time charging scheduling algorithm for on-demand architecture in wireless rechargeable sensor networks,” IEEE Trans. Mobile Comput., vol. 17, no. 1, pp. 211–224, Jan. 2018.

[46] C. Lin, Z. Wang, J. Deng, L. Wang, J. Ren, and G. Wu, “mTS: Temporaland spatial-collaborative charging for wireless rechargeable sensor networks with multiple vehicles,” in Proc. Conf. Comput. Commun., 2018, pp. 99–107.

[47] H.-H. Cho, H.-T. Wu, C.-F. Lai, T. K. Shih, and F.-H. Tseng, “Intelligent charging path planning for iot network over blockchain-based edge architecture,” Internet Things J., vol. 8, no. 4, pp. 2379–2394, 2020.

[48] J. Chen, C. Yi, R. Wang, K. Zhu, and J. Cai, “Learning aided joint sensor activation and mobile charging vehicle scheduling for energy-efficient WRSN-based industrial IoT,” IEEE Trans. Veh. Technol., vol. 72, no. 4, pp. 5064–5078, Apr. 2023.

[49] S. Nikoletseas, T. P. Raptis, A. Souroulagkas, and D. Tsolovos, “Wireless power transfer protocols in sensor networks: Experiments and simulations,” J. Sensor Actuator Netw., vol. 6, no. 2, pp. 1–13, 2017.

[50] H. Dai, X. Wang, A. X. Liu, F. Zhang, Y. Zhao, and G. Chen, “Omnidirectional chargability with directional antennas,” in Proc. Int. Conf. Netw. Protoc., 2016, pp. 1–10.

[51] S. He, X. Gong, J. Zhang, J. Chen, and Y. Sun, “Curve-based deployment for barrier coverage in wireless sensor networks,” IEEE Trans. Wirel. Commun., vol. 13, no. 2, pp. 724–735, Feb. 2014.

[52] P. Corporation, “Powercast the biggest name in wireless power,” 2023. [Online]. Available: www.powercastco.com/

[53] Y. Liu, K. Xiong, Y. Lu, Q. Ni, P. Fan, and K. B. Letaief, “UAV-aided wireless power transfer and data collection in Rician fading,” J. Sel. Areas Commun., vol. 39, no. 10, pp. 3097–3113, 2021.

[54] H. Yan, Y. Chen, and S.-H. Yang, “UAV-enabled wireless power transfer with base station charging and UAV power consumption,” IEEE Trans. Veh. Technol., vol. 69, no. 11, pp. 12 883–12 896, Nov. 2020.

[55] Y. Wang and G. Cao, “Barrier coverage in camera sensor networks,” in Proc. Int. Symp. Mobile Ad Hoc Netw. Comput., 2011, pp. 1–10.

[56] A. Kaushik, M. Goswami, M. Manuja, S. Indu, and D. Gupta, “A binary PSO approach for improving the performance of wireless sensor networks,” Wirel. Pers. Commun., vol. 113, no. 1, pp. 263–297, 2020.

[57] M. A. Hameed and R. C. Muniyandi, “Computationally effective and practically aware Pareto-based multi-objective evolutionary approach for wireless sensor network deployment,” J. Eng. Appl. Sci., vol. 13, no. 13, pp. 4993–5003, 2018.

[58] C. Naik and D. P. Shetty, “Differential evolution metaheuristic scheme for k-coverage and m-connected optimal node placement in wireless sensor networks,” Int. J. Comput. Inf. Syst. Ind. Manage. Appl., vol. 11, no. 1, pp. 132–141, 2019.

[59] C. Lin, Z. Yang, H. Dai, L. Cui, L. Wang, and G. Wu, “Minimizing charging delay for directional charging,” IEEE Trans. Netw., vol. 29, no. 6, pp. 2478–2493, Dec. 2021.

[60] H. Dai, K. Sun, A. X. Liu, L. Zhang, J. Zheng, and G. Chen, “Charging task scheduling for directional wireless charger networks,” IEEE Trans. Mobile Comput., vol. 20, no. 11, pp. 3163–3180, Nov. 2021.

[61] H. Dai et al., “Placing wireless chargers with limited mobility,” IEEE Trans. Mobile Comput., vol. 22, no. 6, pp. 3589–3603, Jun. 2023.

[62] H. Dai et al., “ROSE: Robustly safe charging for wireless power transfer,” IEEE Trans. Mobile Comput., vol. 21, no. 6, pp. 2180–2197, Jun. 2022.

[63] N. Yu, H. Dai, A. X. Liu, and B. Tian, “Placement of connected wireless chargers,” in Proc. Int. Conf. Comput. Commun., 2018, pp. 387–395.

[64] S. Zhang, Z. Qian, F. Kong, J. Wu, and S. Lu, “P3: Joint optimization of charger placement and power allocation for wireless power transfer,” in Proc. Int. Conf. Comput. Commun., 2015, pp. 2344–2352.

[65] D. Lee, C. Lee, G. Jang, W. Na, and S. Cho, “Energy-efficient directional charging strategy for wireless rechargeable sensor networks,” Internet Things J., vol. 9, no. 19, pp. 19 034–19 048, 2022.

[66] Y. Yu and Q. Cheng, “Charging strategy and scheduling algorithm for directional wireless power transfer in WRSNs,” Alexandria Eng. J., vol. 61, no. 10, pp. 8315–8324, 2022.

[67] S. He, J. Chen, F. Jiang, D. K. Yau, G. Xing, and Y. Sun, “Energy provisioning in wireless rechargeable sensor networks,” IEEE Trans. Mobile Comput., vol. 12, no. 10, pp. 1931–1942, Oct. 2013.

[68] T. Tran and D. T. Huynh, “Symmetric connectivity algotirthms in multiple directional antennas wireless sensor networks,” in Proc. Int. Conf. Comput. Commun., 2018, pp. 333–341.

[69] B. Bhattacharya, Y. Hu, Q. Shi, E. Kranakis, and D. Krizanc, “Sensor network connectivity with multiple directional antennae of a given angular sum,” in Proc. Int. Parallel Distrib. Process. Symp., 2009, pp. 1–11.

[70] S. Dobrev, E. Kranakis, D. Krizanc, J. Opatrny, and L. Stacho, “Strong connectivity in sensor networks with given number of directional antennae of bounded angle,” Discrete Math. Algorithms Appl., vol. 4, no. 3, pp. 72–86, 2012.

[71] V. Khodamoradi, A. Sali, O. Messadi, A. Khalili, and B. M. Ali, “Energyefficient massive MIMO SWIPT-enabled systems,” IEEE Trans. Veh. Technol., vol. 71, no. 5, pp. 5111–5127, May 2022.

[72] A. H. Coarasa, P. Nintanavongsa, S. Sanyal, and K. R. Chowdhury, “Impact of mobile transmitter sources on radio frequency wireless energy harvesting,” in Proc. Int. Conf. Comput. Netw. Commun., 2013, pp. 573–577.

[73] R. G. Cid-Fuentes, M. Y. Naderi, R. Doost-Mohammady, K. R. Chowdhury, A. Cabellos-Aparicio, and E. Alarcón, “Leveraging deliberately generated interferences for multi-sensor wireless RF power transmission,” in Proc. Glob. Commun. Conf., 2015, pp. 1–6.

[74] M. Y. Naderi, K. R. Chowdhury, S. Basagni, W. Heinzelman, S. De, and S. Jana, “Surviving wireless energy interference in RF-harvesting sensor networks: An empirical study,” in Proc. Int. Conf. Sens. Commun. Netw. Workshops, 2014, pp. 39–44.

[75] H. Dai et al., “Safe charging for wireless power transfer,” IEEE Trans. Netw., vol. 25, no. 6, pp. 3531–3544, Dec. 2017.

[76] I. Katsidimas, S. Nikoletseas, T. P. Raptis, and C. Raptopoulos, “An algorithmic study in the vector model for wireless power transfer maximization,” Pervasive Mobile Comput., vol. 42, no. 1, pp. 108–123, 2017.

[77] I. Katsidimas, E. Kerimakis, and S. Nikoletseas, “Placement optimization in wireless charging systems under the vector model,” in Proc. Int. Conf. Distrib. Comput. Sensor Syst., 2019, pp. 473–480.

[78] I. Katsidimas, S. Nikoletseas, and C. Raptopoulos, “Power efficient algorithms for wireless charging under phase shift in the vector model,” in Proc. Int. Conf. Distrib. Comput. Sensor Syst., 2019, pp. 131–138.

[79] M. De Berg, S. Cabello, and S. Har-Peled, “Covering many or few points with unit disks,” in Proc. Int. Workshop Approximation Online Algorithms, 2006, pp. 55–68.

[80] Z. Gao, Y. Chen, L. Fan, H. Wang, S. C.-H. Huang, and H.-C. Wu, “Joint energy loss and time span minimization for energy-redistribution-assisted charging of WRSNs with a mobile charger,” Internet Things J., vol. 10, no. 5, pp. 4636–4651, 2022.

[81] H. Yu, Y. Zhang, S. Guo, Y. Yang, and L. Ji, “Energy efficiency maximization for WSNs with simultaneous wireless information and power transfer,” Sensors, vol. 17, no. 8, 2017, Art. no. 1906.

[82] H. Ju and R. Zhang, “Throughput maximization in wireless powered communication networks,” IEEE Trans. Wirel. Commun., vol. 13, no. 1, pp. 418–428, Jan. 2014.

[83] X. Wang et al., “Practical heterogeneous wireless charger placement with obstacles,” IEEE Trans. Mobile Comput., vol. 19, no. 8, pp. 1910–1927, Aug. 2020.

[84] S. Fujishige, Submodular Functions and Optimization. Amsterdam, The Netherlands: Elsevier, 2005.

[85] N. Gharaei, Y. D. Al-Otaibi, S. Rahim, H. J. Alyamani, N. A. K. K. Khani, and S. J. Malebary, “Broker-based nodes recharging scheme for surveillance wireless rechargeable sensor networks,” Sensors J., vol. 21, no. 7, pp. 9242–9249, 2021.

[86] P. Corporation, “TX91501–915mhz powercaster transmitter,” 2010. [Online]. Available: https://fliphtml5.com/miyz/dkxz/basic

[87] T. Technology, “Terabits,” 2023. [Online]. Available: http://www.terabits. cn/product

[88] M. Technology, “MRF24J40 data sheet,” 2023. [Online]. Available: https: //ww1.microchip.com/downloads/en/devicedoc/39776c.pdf

[89] H. W. Kuhn, “The Hungarian method for the assignment problem,” Nav. Res. Logistics Quart., vol. 2, no. 1, pp. 83–97, 1955.

[90] R. Jonker and T. Volgenant, “Improving the Hungarian assignment algorithm,” Operations Res. Lett., vol. 5, no. 4, pp. 171–175, 1986.

![](images/514cdae876bccd1e285ff025c6ce5e2bc601bd60cf74384a2e5b5963642eb5ee.jpg)

Haipeng Dai (Senior Member, IEEE) received the BS degree from the Department of Electronic Engineering from Shanghai Jiao Tong University, Shanghai, China, in 2010, and the PhD degree from the Department of Computer Science and Technology in Nanjing University, Nanjing, China, in 2014. His research interests are mainly in the areas of Internet of Things and mobile computing. He is an associate professor with the Department of Computer Science and Technology in Nanjing University. His research papers have been published in many prestigious conferences and journals such as ACM MobiSys, ACM MobiHoc, ACM UbiComp, IEEE IN-FOCOM, ACM SIGMETRICS, IEEE ICDCS, IEEE ICNP, IEEE Transactions on Mobile Computing, IEEE Journal on Selected Areas in Communications, IEEE/ACM Transactions on Networking, and IEEE Transactions on Parallel and Distributed Systems. He serves/ed as TPC Chair of the IEEE ISPA’22, TPC Vice-Chair of the IEEE HPCC’21, Poster Chair of the IEEE ICNP’14, Track Chair of the ICCCN’19 and the ICPADS’21, TPC member of ACM MobiHoc’20–22 and IEEE INFOCOM’20-23. He received Best Paper Award from IEEE ICNP’15, Best Paper Award Runner-up from IEEE SECON’18, and Best Paper Award Candidate from IEEE INFOCOM’17.

![](images/b77faa71b704b91c355fa5f33ed0e5250c1cf20cf752c99b5d02b210644258fe.jpg)

Yikang Zhang received the BS degree from the School of Computer Science and Technology from Soochow University, Suzhou, Jiangsu, China, in 2022. He is currently working toward the first-year graduate degree in Nanjing University.

![](images/77a3f902cbf7902d0ac94f81debf9ea923bca7cf1e3f6a2aac009cfc9073858c.jpg)

Weijun Wang (Member, IEEE) received the BS degree from the Department of Computer and Software, Nanjing University of Post and Telecommunication, Nanjing, China, in 2014, and the ME degree in computer technology from the PLA University of Science and Technology, Nanjing, in 2017. He is working towards the PhD degree with the Department of Computer Science and Technology, Nanjing University. His research interests include the UAV monitoring, MAC protocols in UAV networks and ad hoc networks.

![](images/11ea26f770f0c98cd186c33c011be8f5b3708184ea2114f6c626256516ec7fef.jpg)

and distributed computing, and graph theory algorithms.

![](images/6f5814d0f5a8d0dbc7fb40470539d4991d8bf3fd166c20dcebe5612e1904cfc1.jpg)

Lijie Xu (Member, IEEE) received the PhD degree from the Department of Computer Science and Technology, Nanjing University, Nanjing, in 2014. He was a research assistant with the Department of Computing, The Hong Kong Polytechnic University, Hong Kong, from 2011 to 2012. He is currently an Associate Professor with the Jiangsu Key Laboratory of Big Data Security and Intelligent Processing, Nanjing University of Posts and Telecommunications, Nanjing. His research interests are mainly in the areas of wireless sensor networks, ad-hoc networks, mobile

Rong Gu (Member, IEEE) received the PhD degree from Nanjing University, Nanjing, China, in 2016. He is an associate research professor in Nanjing University. His research interests include parallel and distributed computing, Big Data systems. His research papers have been published in many conference and journals, including IEEE Transactions on Parallel and Distributed Systems, IEEE ICDE, IEEE IPDPS, IEEE ICPP, Journal of Systems Architecture, Parallel Computing, Journal of Parallel and Distributed Computing and SPE.

![](images/fb79d7abd889c328778801fb821c56d469086c70247917d37a326bbed1f93976.jpg)

Yuben Qu received the BS degree in mathematics and applied mathematics from Nanjing University in 2009 and the MS degree in communication and information systems and the PhD degree in computer science and technology from the Nanjing Institute of Communications, China, in 2012 and 2016, respectively. From October 2015 to January 2016, he was a visiting research associate with the School of Computer Science and Engineering, The University of Aizu, Japan. He is currently a postdoctoral with the Department of Computer Science and Engineering,

Chi Lin (Senior Member, IEEE) received the BE and PhD degrees from the Dalian University of Technology, Dalian, China, in 2008 and 2013, respectively. He has been an assistant professor with the School of Software, Dalian University of Technology, since 2014. Since 2017, he has been an associate professor with the School of Software, Dalian University of Technology. He has authored more than 50 scientific papers in several journals and conferences, including MobiCom, INFOCOM, SECON, ICDCS, ICNP, ICPP, IEEE/ACM Transactions on Networking, IEEE

Shanghai Jiao Tong University, China. His research interests include mobile edge computing, air-ground integrated networks, D2D communications, and crowdsensing.

Transactions on Mobile Computing, ACM Transactions on Embedded Computing Systems, ACM Transactions on Sensor Networks, IEEE Transactions on Vehicular Technology, and special issue of Science. His research interests include pervasive computing and wireless sensor networks. In 2015, he was a Recipient of the ACM Academic Rising Star.

![](images/53e701e84d39269414b282fc7a57b336e1a4975a1cd2a3bd7ffb21ef63742fb9.jpg)

![](images/18960fac4b0bed063c419f2950e643c5b0810868c83f46fa3cf54618fdd5ea03.jpg)

Jiaqi Zheng (Senior Member, IEEE) received the PhD degree from Nanjing University, in 2017. He is currently an assistant researcher with the Department of Computer Science and Technology, Nanjing University, China. His research area is computer networking, particularly data center networks, SDN, and NFV. He was an assistant researcher with the City University of Hong Kong in 2015, and a visiting scholar with Temple University in 2016. He received the best paper award from IEEE ICNP 2015 and Doctorial Dissertation Award from ACM SIGCOMM China 2018. He is a member of the ACM.

![](images/f5b4e163fdbefc108792160d8933595f642794578b83bdf0721447c06fce2c80.jpg)

Wanchun Dou (Member, IEEE) received the PhD degree in mechanical and electronic engineering from the Nanjing University of Science and Technology, China, in 2001. He is currently a full professor of the State Key Laboratory for Novel Software Technology, Nanjing University. From April 2005 to June 2005 and from November 2008 to February 2009, he respectively visited the Department of Computer Science and Engineering, Hong Kong University of Science and Technology, Hong Kong, as a visiting scholar. Up to now, he has chaired three Nationa

Natural Science Foundation of China projects and published more than 100 research papers in international journals and international conferences. His research interests include workflow, cloud computing, and service computing.

![](images/22384a86d91a95091ee2dcbe3181b63d1603701cb129cb0418c51228b04dca81.jpg)

Guihai Chen (Fellow, IEEE) received the BS degree in computer software from Nanjing University in 1984, the ME degree in computer applications from Southeast University in 1987, and the PhD degree in computer science from the University of Hong Kong in 1997. He is a professor and deputy chair with the Department of Computer Science, Nanjing University, China. He had been invited as a visiting professor by many foreign universities including Kyushu Institute of Technology, Japan in 1998, University of Queensland, Australia in 2000, and Wayne State Uni-

versity, USA during 2001 to 2003. He has a wide range of research interests with focus on sensor networks, peer-to-peer computing, high-performance computer architecture, and combinatorics.
