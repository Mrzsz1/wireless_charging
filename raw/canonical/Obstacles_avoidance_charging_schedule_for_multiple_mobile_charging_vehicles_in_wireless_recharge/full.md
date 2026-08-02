---
title: "Obstacles avoidance charging schedule for multiple mobile charging vehicles in wireless rechargeable sensor networks"
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
pdf_path: "raw/canonical/Obstacles_avoidance_charging_schedule_for_multiple_mobile_charging_vehicles_in_wireless_recharge/Obstacles_avoidance_charging_schedule_for_multiple_mobile_charging_vehicles_in_wirel.pdf"
raw_md: "raw/canonical/Obstacles_avoidance_charging_schedule_for_multiple_mobile_charging_vehicles_in_wireless_recharge/full.md"
---
# Obstacles Avoidance Charging Schedule for Multiple Mobile Charging Vehicles in Wireless Rechargeable Sensor Networks

Sk Md Abidar Rahaman Aliah University Md Azharuddin Aliah University

Pratyay Kuila  (  pratyay\_kuila@yahoo.com ) National Institute of Technology Sikkim

## Research Article

Keywords: Wireless rechargeable sensor networks, charging scheduling, Mobile charging vehicles, Obstacles, Joint charging preference

Posted Date: October 26th, 2023

DOI: https://doi.org/10.21203/rs.3.rs-3468314/v1

License:   This work is licensed under a Creative Commons Attribution 4.0 International License. Read Full License

Additional Declarations: No competing interests reported.

# Obstacles Avoidance Charging Schedule for Multiple Mobile Charging Vehicles in Wireless Rechargeable Sensor Networks

Sk Md Abidar Rahaman<sup>a</sup>, Md Azharuddin<sup>a</sup>, Pratyay Kuila<sup>b,∗</sup>

Department of Computer Science & Engineering

<sup>a</sup>Aliah University, Kolkata 700156, India <sup>b</sup>National Institute of Technology Sikkim-737139, India

## Abstract

The integration of wireless power transfer technology into mobile charging vehicles (MCVs) opens up new possibilities for wirelessly recharging the batteries of sensor nodes (SNs). Thereby, it extends the network’s operational lifespan for wireless rechargeable sensor networks (WRSNs). However, the task of devising optimal charging schedules with MCVs is a complex one, especially when these MCVs encounter obstacles necessitating detours. These detours can impact the overall energy consumption and longevity of WRSNs. In this article, we introduce a strategic approach that harnesses the capabilities of multiple MCVs to enhance the eficiency of charging operations within WRSNs. The proposed method considers several key factors, such as the temporal and spatial aspects of charging requirements, the presence of multiple MCVs, and the challenges posed by obstacles. By amalgamating these elements into a unified metric, our strategy determines charging sequences based on collective charging preferences. The allocation of charging partitions is facilitated by clustering, leveraging the locations of SNs and their energy consumption rates, with each MCV tending to a designated partition. Furthermore, we employ an obstacle avoidance algorithm grounded in the use of anchor points and projection points to address scheduling issues when obstacles are encountered. Overall, this strategic framework ofers an eficient and efective solution for SN charging, contributing to improved longevity and performance within WRSNs. Extensive simulations are conducted to validate the proposed methodology, and performance comparisons with existing systems are provided. An analysis of variance (ANOVA) is performed, followed by a post-hoc analysis.

Keywords: Wireless rechargeable sensor networks, charging scheduling, Mobile charging vehicles, Obstacles, Joint charging preference.

## 1. Introduction

Wireless sensor networks (WSNs) employ numerous tiny sensor nodes (SNs) distributed haphazardly throughout a geographical region to gather and oversee the physical characteristics of a designated target. These embedded SNs are equipped with energy-eficient, compactscale computing, sensing, and wireless communication capabilities, allowing them to identify, process, and transmit data to sink nodes [1, 2]. Beyond military surveillance, the utility of WSNs extends to aviation, preempting explosions, disseminating disaster alerts, and environmental monitoring [3, 4, 5]. To optimize the limited power resources of SNs, researchers have introduced clustering algorithms and routing protocols, seeking to mitigate the traditional power supply constraints that have constrained the network’s lifespan. Furthermore, novel approaches such as harnessing environmental energy sources like solar, vibration, and wind power have been proposed by other researchers to extend network longevity [6, 7, 8]. However, the reliance on these methods can lead to fluctuating energy provision due to their susceptibility to environmental variables. In response to the energy challenge in WSNs, the concept of wireless power transfer (WPT) technology has emerged. Networks of this kind are referred to as Wireless Rechargeable Sensor Networks (WRSNs) [9]. Within WRSNs, a wireless charging vehicle (MCV) traverses the network, recharging SNs as it moves. The implementation of wireless charging technology ensures a dependable and controlled energy supply for WRSNs.

WRSNs exhibit the potential for efective energy supply, yet they grapple with several formidable challenges. Given the distinct missions of SNs, their energy consumption rates and recharging requirements vary significantly. This necessitates the prompt replenishment of energy for all SNs by MCVs to ensure uninterrupted network functionality, as articulated in the study by Deng et al. [10]. Moreover, a solitary MCV, equipped with limited energy storage, is insuficient for recharging all the SNs within a large-scale WRSN. Hence, the operation of multiple MCVs in tandem is imperative to achieve perpetual network operation, an aspect emphasized by [11, 12]. However, it is crucial to acknowledge that the deployment of MCVs significantly impacts the overall cost of WRSNs. Enhancing the eficiency of a WRSN involves the judicious allocation of MCVs, catering precisely to the needs of the requesting SNs. Consequently, the optimization of charging scheduling strategies has become a pertinent concern, as discussed in the works [13, 14, 15].

Sensing fields often encounter impediments in real-world physical settings [16]. Consequently, the primary task becomes identifying the most eficient path while circumventing these obstacles. In our approach, which involves cluster-based partitioning, we designate an MCV for each partition. Nevertheless, the presence of diverse obstacles within the sensing field adds complexity to the coordination of these MCVs. It’s important to note that MCVs can access any location within the field except for obstacle sites. Thus, the central research challenge revolves around efectively dispatching the MCVs to determine the shortest viable routes while avoiding obstacles. To address these aforementioned challenges, we introduce a novel strategy that leverages multiple MCVs. Under this framework, MCVs can intelligently assess how best to fulfill a request, considering its spatial, temporal, and sensory requirements. Our contributions can be succinctly summarized as follows:

• We proposed an eficient clustering method to partition the whole network into multiple regions. After that, a particular MCV is assigned to each of the partitions.

• We have considered multiple critical metrics like the residual energy of SNs, the energy consumption rate of the SNs, and the distance from MCV to SNs when making charging requests for SNs. By combining these factors into one metric, the proposed strategy determines charge sequences based on the joint charging preference allocation. We have also considered the maximum energy capacity constraint of the MCV and the maximum time constraint during the charging scheduling design.

• In a real-world scenario, to solve the scheduling problem with obstacles, we employed an obstacle avoidance algorithm based on the anchor point and projection point of an obstacle with an SN. As the MCV moves through the network with obstacles, it will search for the shortest route that avoids all obstacles.

• To ensure that the proposed method is efective, it is simulated and compared with two existing works [17] and [18]. Thereafter, we performed a statistical test, an analysis of variance (ANOVA), and a post hoc analysis.

Following is an outline of the rest of the article. The section 2 reviews the literature related to WRSNs. The section 3 provides a variety of models, terminologies, and problem formulations. A phase-by-phase breakdown of the proposed works is shown in section 4. Through extensive experiments, the section 5 evaluates the performance of the proposed algorithm. The paper concludes with future directions in section 6.

## 2. Related works

There has been a significant advancement in the study of WRSNs that use MCVs to extend network lifetime. The key areas of study included maintaining sensors in WRSNs and improving MCV charging eficiency. According to these studies, there are two categories of charging scheduling methods for WRSNs: fixed charging strategies [17, 19, 20] and realtime [21, 22, 23, 24] charging techniques. According to the existing studies in WRSNs, most MCVs have a fixed charging schedule and follow static charging paths. According to its predetermined charging schedule, the MCV traverses SNs and delivers energy. Charging strategies are generally formulated as traveling salesman problems (TSP) based on SNs’ energy consumption and distribution models. An optimal traveling path for MCVs is designed with a Hamiltonian cycle. In the TSP field, the ant colony algorithm (ACO) has become widely used. Distributed computing is used, providing strong global search capabilities and positive feedback on information [17]. A fixed charging strategy can be classified into single-node [25, 19, 26] and multi-node charging schemes [27, 20]. A path planning technique based on SN remaining energy levels was proposed to address the energy eficiency issue [26]. Based on lifetime prediction, a fixed charging strategy was proposed in the study [25]. As a result of the single-node charging strategy, an MCV can only replenish energy for one sensor at a time, causing low charging eficiency. Multiple neighboring nodes are simultaneously charged using the multi-node charging strategy[27]. The efect of charging multiple nodes significantly improved charging eficiency[20]. It is important that fixed charging strategies forecast residual energy and SN status. As a result of complex environmental changes, the energy consumption rate of the SNs in WRSNs varies dynamically. The fixed charge strategy’s computational expense is greatly raised as a result.

Real-time charging procedures were suggested as a way to reduce the impact of instability issues for WRSNs. In the real-time charging approach, the node starts an energy request for charging when its residual energy falls below a specific threshold. The requesting sensor is promptly recharged by the MCV in accordance with the charging schedule, relying on certain pre-fixed rules. By their arrival time, charging requests are fulfilled. The first-come, firstserved (FCFS) approach was introduced in [28]. In FCFS, temporal priorities are weighed over spatial ones when determining the charging schedule. The MCV chooses the sensor for charging only based on the distance to the nearest job next with the preemption (NJNP) strategy [18]. NJNP has a few obvious drawbacks. Firstly, in WRSNs, sensors are dispersed at random, and charging requests can come from anywhere at any time, resulting in lengthy charging delays and lengthy moving distances. Secondly, if the requesting sensor is far from the MCV, it cannot be charged in time, using up all of its power.

Similar distinctions can be made between single-MCV charging strategies [21, 22, 23, 18] and multi-MCV [24, 29] [4] collaborative charging strategies when describing real-time charging techniques. To improve scheduling performance, a charging system that combines arrival time and distance was suggested [21]. Using residual energy, distance to MCV, and critical node density, the authors developed a charging scheme in [22]. As a means of maximizing the quality of monitoring for stochastic events, Dai et al. [23] designed a joint charging and scheduling scheme. An algorithm based on queueing theory was used to solve the problem. In large-scale WRSNs, thousands of SNs may exist, but only one MCV can address all of them. A multi-MCV charging strategy was proposed by some researchers for this reason. [24] presented a genetic algorithm-based charging schedule for multi-MCVs. According to Lin et al., [29], multiple MCVs can be cooperatively charged by using a cooperative charging model and a Nash equilibrium point. A multiple MCVs ondemand charging strategy has been proposed by Jiang et al. [4] for maximizing the coverage utility of MCVs. They did not include the survival ratio or the charging delay, though. In [30] the author addresses the energy redistribution-assisted MCV charging scheduling problem in WRSNs. A greedy algorithm is proposed to minimize energy loss and the time duration of charging. In the paper [31], the author selects an optimal sensor set based on combining reinforcement learning and approximation algorithms and schedules the MCV to minimize system energy consumption. In conclusion, the real-time charging technique can increase the network’s overall lifespan. To enhance the charging performance, however, several network settings for scheduling the sensors should be improved. Prior research has also ignored the possibility that the sensing field may contain a range of obstacles. An obstacle-containing sensing field has a more complex route for MCV than one without.

Currently, most of the charging strategies only consider residual energy, location, and sensor stochastic charging requests when deciding the charging order. All of these factors interact with one another and are complex. Additionally, it is important to consider how much energy the node consumes. Energy-consuming nodes should be charged earlier than those with lower consumption rates.

Further, the charging request is the result of stochastic energy consumption accruing over time. The node’s energy will be rapidly depleted if its charging request is not handled in time. A WRSN must be able to respond to unpredictable events at any time and anywhere, especially if the sensors are deployed in a large area. Event-detection failure can result when a node exhausts its energy. To address such real-time scheduling challenges, a charging strategy is proposed.

However, the challenge is also to find the shortest route for MCVs to prolong the network lifetime. When obstacles of any shape or size are present in the sensing field, deploying MCVs becomes more complicated. A solution to this problem is also presented in this paper.

Due to these limitations, it is necessary to look at the SNs’ scheduling issues more thoroughly. In Table 1, comparisons of the proposed and existing systems in terms of five key attributes are shown.

Table 1: Summary of the related works

<table><tr><td>Paper</td><td>Multiple MCV</td><td>Obstacle avoidance of MCVs</td><td>Joint charging priority</td><td>MCVs travel and recharge energy</td></tr><tr><td>[15]</td><td>Yes</td><td>No</td><td>No</td><td>No</td></tr><tr><td>[27]</td><td>No</td><td>No</td><td>No</td><td>No</td></tr><tr><td>[17]</td><td>No</td><td>No</td><td>No</td><td>No</td></tr><tr><td>[19]</td><td>No</td><td>No</td><td>No</td><td>No</td></tr><tr><td>[20]</td><td>No</td><td>No</td><td>No</td><td>No</td></tr><tr><td>[22]</td><td>No</td><td>No</td><td>No</td><td>No</td></tr><tr><td>[24]</td><td>Yes</td><td>No</td><td>No</td><td>No</td></tr><tr><td>[18]</td><td>No</td><td>No</td><td>No</td><td>No</td></tr><tr><td>[26]</td><td>No</td><td>No</td><td>No</td><td>No</td></tr><tr><td>[30]</td><td>No</td><td>No</td><td>No</td><td>No</td></tr><tr><td>[31]</td><td>No</td><td>No</td><td>No</td><td>No</td></tr><tr><td>Proposed</td><td>Yes</td><td>Yes</td><td>Yes</td><td>Yes</td></tr></table>

## 3. System model and problem formulation

## 3.1. Network model

Wireless rechargeable sensor networks (WRSNs) comprise multiple stationary SNs, numerous mobile charging vehicles (MCVs), and a single maintenance station. These SNs are assumed to have identical battery capacities and are randomly distributed across a designated area. Both MCVs and SNs are equipped with limited-capacity batteries, along with GPS modules or other localization tools. When an SN’s energy falls below a certain threshold, it sends a charging request. The operational MCVs are capable of traversing the entire monitored area to replenish the energy of the SNs that have requested it. The maintenance station is assumed to have an unlimited energy supply and can charge all MCVs as needed.

The SN’s energy diminishes over time while it operates continuously. When the remaining energy of an SN falls below a certain threshold, the SN initiates a notification and transmits a charging request. This request is added to the MCV’s request queue, where it will be processed at a later time. The MCV utilizes information such as the residual energy, position, and energy consumption rate of the charging request nodes to determine and establish an optimal charging path. The MCV expedites the charging process by relocating itself to the location of the SN requesting energy replenishment. This action ensures that each SN receives a full recharge. Consequently, the energy-receiving rate determines how long it takes for each SN to be charged. Whenever the MCV’s remaining energy level falls below a predetermined threshold, it will return to the maintenance station for recharging.

## 3.2. System model

WRSNs with n number of requested rechargeable SNs were considered, in which 0 represents the maintenance station. SNs are assumed to have fixed transmission power and that their energy can be recharged by MCVs. As a result, each SN can communicate within a stable range. We assume that each SN’s communication range is fixed and that its location and obstacles are known [32]. Each SN sends charging requests to the maintenance station after its energy falls below the threshold. After designing the charging schedule, the maintenance station sends a particular MCV assigned for that region to recharge the SNs according to the schedule. Suppose that $\mathbb { M } = \{ m _ { 1 } , m _ { 2 } , . . . , m _ { k } \}$ is the set of MCVs that charge the SNs in the network. The $m _ { j }$ is associated with velocity (v) and travel time $( T _ { l } ( j ) )$ during which it must finish recharging all the assigned SNs and return to the maintenance station. Assume, $A = \{ A ^ { 1 } , A ^ { 2 } , . . . , A ^ { y } \}$ is the set of obstacles in the environment and the positions of the obstacles are known [32]. Generally, the MCVs have to travel across the environment to recharge the SNs. Thereby, the MCVs require a detour to reach the next SN to avoid the obstacles between the SNs. All the notations are described in Table 2.

Table 2: Notations

<table><tr><td>Notation</td><td>Description</td></tr><tr><td> $S = \{s_1, s_2, \ldots, s_n\}$ </td><td>Set of requested SNs.</td></tr><tr><td> $\mathbb{M} = \{m_1, m_2, \ldots, m_k\}$ </td><td>Set of MCVs.</td></tr><tr><td> $A = \{A^1, A^2, \ldots, A^y\}$ </td><td>Set of obstacles.</td></tr><tr><td> $E_t$ </td><td>Energy transmitted by MCV per unit time</td></tr><tr><td> $t_i$ </td><td>The charging time of  $s_i$ </td></tr><tr><td> $T_l(j)$ </td><td>Travel time of  $m_j$ </td></tr><tr><td> $B_{max}(j)$ </td><td>The energy capacity of the  $m_j$ </td></tr><tr><td> $E_c(j)$ </td><td>Total energy consumption of  $m_j$ </td></tr><tr><td> $E_{travel}(j)$ </td><td>Total travel energy consumption of  $m_j$ </td></tr><tr><td> $E_{recharge}(j)$ </td><td>Total energy consumption of  $m_j$  to recharge the SNs</td></tr><tr><td>K</td><td>Number of partitions</td></tr></table>

## 3.3. Charging model

Based on Friis’s free space equations, here we assume the same charging model as in [20, 22, 33]. The MCV consumes energy to travel and recharge the SNs. The total energy consumption of $m _ { j }$ can be estimated as:

$$
E _ {C} (j) = E _ {t r a v e l} (j) + E _ {r e c h a r g e} (j)\tag{1}
$$

where, $E _ { t r a v e l } ( j )$ and $E _ { r e c h a r g e } ( j )$ are the travelling and recharge energy of $m _ { j }$ respectively. Let $E _ { u } ( j )$ be the traveling energy consumption of $m _ { j }$ per unit of time. If $m _ { j }$ travels for $T _ { l } ( j )$ unit of time then the total traveling energy consumption is calculated as:

$$
E _ {t r a v e l} (j) = E _ {u} (j) \times T _ {l} (j)\tag{2}
$$

Let $q ( j )$ be the total recharge energy (circuit and transmission energy) consumption of $m _ { j }$ while recharging $s _ { i }$ for $t _ { i }$ unit of time. Let $X _ { i , j }$ be a Boolean variable whose value is 1 if the $s _ { i }$ is charged by $m _ { j }$ , otherwise 0. Therefore, $E _ { r e c h a r g e } ( j )$ can be calculated as:

$$
E _ {r e c h a r g e} (j) = q (j) \times \sum_ {i = 1} ^ {n} [ t _ {i} \times X _ {i, j} ]\tag{3}
$$

## 3.4. Time constraints

Each MCV is required to return to the BS to restore its batteries after all charging requests have been fulfilled or its energy reaches below the required level. In constraint (4), the MCV traveling time is limited based on the predetermined charging period T. It is assumed that all the MCVs are supposed to finish their journey in the allotted amount of time T.

$$
\{T _ {l} (j) + \sum_ {i = 1} ^ {n} t _ {i} \times X _ {i, j} \} \leq T\tag{4}
$$

## 3.5. Problem formulation

## 3.5.1. Charging problem

Assuming that there are n numbers of charging requests with corresponding deadlines and multiple MCVs as the charging device. SNs are to be recharged by the predetermined MCV at a specific time, and each MCV has a limited energy capacity, with the objective of maximizing SN recharge energy and minimizing the travel energy of all MCVs. Following is a formulation of the problem:

$$
\text { Minimize } P _ {1} = \sum_ {j = 1} ^ {m} (E _ {u} (j) \times T _ {l} (j))\tag{5}
$$

$$
\text { Maximize } P _ {2} = \sum_ {j = 1} ^ {m} \left\{q (j) \times \sum_ {i = 1} ^ {n} [ t _ {i} \times X _ {i, j} ] \right\}\tag{6}
$$

Subject to,

$$
\sum_ {\forall m _ {j} \in \mathbb {M}} X _ {i, j} = 1, \forall i, 1 \leq i \leq n, \forall j, 1 \leq j \leq m\tag{7}
$$

$$
T _ {l} (j) + \sum_ {i = 1} ^ {n} t _ {i} \times X _ {i, j} \leq T, \forall m _ {j} \in \mathbb {M}\tag{8}
$$

$$
q (j) \times \sum_ {i = 1} ^ {n} t _ {i} \times X _ {i, j} + (E _ {u} (j) \times T _ {l} (j)) \leq B _ {\max} (j), \forall m _ {j} \in \mathbb {M}\tag{9}
$$

Equations (5) and (6) describe the scheduling objectives. The constraints are represented using equations $( 7 ) \AA - ( 9 )$ Constraint (7) ensures that a particular $\mathrm { M C V } ( m _ { j } )$ charges only one SN $\left( { { s _ { i } } } \right)$ at a time in its region. Constraint (8) ensures that the total charging time of $m _ { j }$ , including travel time $( T _ { l } ( j ) )$ , is lower than the maximum allowed time T. According to constraint (9), the total energy consumed by the $m _ { j }$ during SN charging and travel should not exceed its maximum battery capacity $\left( B _ { m a x } ( j ) \right)$ .

## 3.5.2. Obstacle avoidance problem

Here, we have assumed that there are multiple obstacles in the WRSNs. If there is an obstacle in the path of the MCV between two SNs, then the MCV needs to avoid that obstacle to reach the next SNs for charging. Here, the problem is formulated in the form of integer linear programming (ILP). Let the nodes $s _ { i }$ and $s _ { i + 1 }$ be separated by an obstacle in a particular partition, and let $d i s _ { a r c } ( s _ { i } , s _ { i + 1 } )$ represent the distance between $s _ { i }$ and $s _ { i + 1 }$ over the obstacle. $d i s ( s _ { i } , s _ { i + 1 } )$ represents the distance between $s _ { i }$ and $s _ { i + 1 }$ without an obstacle in the same partition. Let $Z _ { s _ { i } s _ { i + 1 } }$ be a Boolean variable represented using equation (10).

$$
Z _ {s _ {i} s _ {i + 1}} = \left\{ \begin{array}{l l} 1, & \text { If   there   is   an   obstacle   between   } s _ {i} \text {   and   } s _ {i + 1} \text {   in   same   partition } \\ 0, & \text { Otherwise. } \end{array} \right.\tag{10}
$$

Let $R _ { s _ { i } s _ { i + 1 } } ^ { j }$ be another Boolean variable and its value is 1 if there is an arc between $s _ { i }$ and $s _ { i + 1 }$ on the path of $m _ { j }$ in the same partition, otherwise, 0. It can be represented using equation (11).

$$
R _ {s _ {i} s _ {i + 1}} ^ {j} = \left\{ \begin{array}{l l} 1, & \text { If   } m _ {j} \text { 's   path   passes   through   an   arc   } (s _ {i}, s _ {i + 1}) \text {   in   same   partition } \\ 0, & \text { Otherwise. } \end{array} \right.\tag{11}
$$

The problem can be formulated as:

$$
\text { Minimize } F _ {1} = \sum_ {i = 1} ^ {n - 1} \sum_ {j = 1} ^ {m} (d i s (s _ {i} s _ {i + 1}) (1 - Z _ {s _ {i} s _ {i + 1}}) + d i s _ {a r c} (s _ {i} s _ {i + 1}) Z _ {s _ {i} s _ {i + 1}}) R _ {s _ {i} s _ {i + 1}} ^ {j}\tag{12}
$$

Subject to,

$$
\sum_ {j = 1} ^ {m} R _ {s _ {i} s _ {i + 1}} ^ {j} = 1, \forall i, 1 \leq i \leq n - 1\tag{13}
$$

$$
\sum_ {j = 1} ^ {m} X _ {i j} = 1, \forall i, 1 \leq i \leq n\tag{14}
$$

The obstacle avoidance problem in equation (12) minimizes the total traveling distance of all MCVs considering the obstacles in their corresponding partition environment. In constraint (13), each MCV visits a requested SN in a particular partition only once. The constraint (14) enforces that each MCV charges only one SN in its partition.

## 4. Proposed scheme

In this section, three methods are presented for the charging problem. First, the partition of the network for deploying multiple MCVs. Secondly, the charging schedule is designed for the MCVs. Finally, the obstacle-avoiding method is presented for the MCVs.

## 4.1. Phase 1: Partitioning of WRSNs

In this section, we divide the network into multiple partitions and assign an MCV in each partition. For partitioning the network, the binning-based silhouette method [34] is used to obtain the optimal number of partitions using K-means clustering. In this case, the best K value is selected using the Silhouette score method. The optimal K value is selected based on the highest Silhouette score. The Silhouette score is a measure of how well an SN fits within a cluster. It is calculated using the following formula:

$$
S i l h o u e t t e S c o r e s = (b - a) / m a x (a, b)\tag{15}
$$

where, a is the average of the intra-cluster distance for all the clusters, and b is the average of the inter-cluster distance for all the clusters. The intra-cluster distance for a cluster is calculated as the sum of the distance from the centroid of the cluster to all the member SNs and then divided by the sum distance by the number of member SNs. Note that the minimum intra-cluster distance means the MCV has to travel less distance to recharge the SNs inside a cluster. On the other hand, the inter-cluster distance means the average distance from the centroid of a cluster to its nearest cluster for all the clusters in the network. Also, note that minimizing the average inter-cluster distance means finding the optimum number of clusters to accommodate all the requested SNs.

Now, we calculate the Silhouette score as follows: First, we cluster the requested SNs into K number of clusters. After that, we calculate the intra-cluster distance for each cluster, and then the average of these intra-cluster distances is used to calculate the value of a. Next, we calculate the value of b, which is done by taking the average of the nearest cluster distance for each cluster in the network. Finally, we can calculate the value of Silhouette-score using the equation (15) for a specific value of K. This whole process is repeated up to a certain K value (say, up to 10). The optimal number of clusters is selected based on the value of K for which the silhouette score is highest. After selecting an optimal number of clusters, we consider it a partition and assign an MCV to each partition. The corresponding partitioning algorithm is shown in Algorithm 1.

![](images/100b45ff358c4342da90345b857d88d8c697cfb63fb1adfda00b81e43c140cd0.jpg)  
Fig. 1: Partitioning WRSNs based on Silhouette scores

Illustration 4.1. In Fig. 1, we have assumed three clusters (Cluster 1, Cluster 2, and Cluster 3). The mean intra-cluster distance (a) and the mean nearest-cluster distance (b)

for each SN are used to determine the silhouette coeficient. For that, we have to calculate the average intra-cluster distance for Cluster 1. Cluster 2 is the nearest neighbor to Cluster 1. So, we have to calculate the average distance between Cluster 2 and Cluster 1 SNs. We can calculate the silhouette scores using equation (15).

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 1: Partitioning Algorithm

Input: $S = \{s_1, s_2, \ldots, s_n\}$: Set of requested SNs.
$K_{max}$: Maximum value of $K$ to be considered for clustering.
Output: $K_{optimal}$: Optimal number of clusters based on the Silhouette score

1 $K_{optimal} \leftarrow 0$; // Initialize optimal number of clusters
2 $SilhouetteScore_{optimal} \leftarrow -1$; // Initialize optimal Silhouette score
3 for $K = 2$ to $K_{max}$ do

4    $\sigma = \{\sigma_1, \sigma_2, \ldots, \sigma_K\} \leftarrow PerformKMeans(S, K)$; // Perform K-Means with K clusters, where $\sigma$ is the set of clusters
5    $SilhouetteScores \leftarrow []$; // Initialize an empty list for Silhouette Scores
6    $a \leftarrow 0$; // Initialize a as zero
7    foreach $\sigma_k \in \sigma$ do
8    $a_{avg}(\sigma_k) \leftarrow AverageIntraClusterDistance(s_i, \sigma_k)$; // Calculate average intra-cluster distance.
9    $a \leftarrow a + a_{avg}(\sigma_k)$; // Update a
10    $a \leftarrow \frac{a}{K}$; // Average intra-cluster distance of all clusters
11    $b \leftarrow 0$; // Initialize b as zero
12    foreach $\sigma_k \in \sigma$ do
13    $b_{min}(\sigma_k) \leftarrow CalculateMinDistanceToNearestCluster(\sigma_k, \sigma)$; // Calculate minimum distance to the nearest cluster (distance from the centroid of the cluster)
14    $b \leftarrow b + b_{min}(\sigma_k)$; // Update b
15    $b \leftarrow \frac{b}{K}$; // Average inter-cluster distance
16    $SilhouetteScores(K) \leftarrow \frac{b-a}{\max(a,b)}$; // Calculate Silhouette Score for K if $SilhouetteScores(K) &gt; SilhouetteScore_{optimal}$ then
17    $SilhouetteScore_{optimal} \leftarrow SilhouetteScores(K)$; // Update optimal Silhouette Score
18    $K_{optimal} \leftarrow K$; // Update optimal number of clusters
19    return $K_{optimal}$; // Return optimal K
</div>

Lemma 4.1. The worst case time complexity of the Partitioning Algorithm 1 is $O ( ( n +$ 255 $K _ { m a x } \times n + K _ { m a x } ^ { 2 } ) \times K _ { m a x } )$

Proof. Lines 1 and 2 initialization takes O(1) time. Line no 3 to 19 contains the main computation within the loop. The loop of line no. 3 iterates $( K _ { m a x } - 1 )$ times. The K-Means clustering on Line 4 takes $O ( n )$ time, where n is the number of requested SNs in S. Line 7 to 9 calculates the average intra-cluster distance and takes $O ( K _ { m a x } \times n )$ time. Line no 12 to 14 calculates the minimum distance to the nearest cluster it takes $O ( K _ { m a x } ^ { 2 } )$ time.

Therefore, from lines no 3 to 16 takes $O ( ( n + K _ { m a x } \times n + K _ { m a x } ^ { 2 } ) \times K _ { m a x } )$ time. The update of optimal values on Lines 17 to 19 takes $O ( 1 )$ time. Therefore, the total time complexity for this algorithm is $O ( ( n + K _ { m a x } \times n + K _ { m a x } ^ { 2 } ) \times K _ { m a x } )$ □

## 4.2. Phase 2: Determining the charging schedule

The BS stores the locations of requested SNs and their deadline times. The deadline time $\left( H ( s _ { i } ) \right)$ of a node $s _ { i }$ is obtained by dividing the residual energy $\left( r ( s _ { i } ) \right)$ of the node by its energy consumption rate $\left( g ( s _ { i } ) \right)$ , i.e., $\begin{array} { r } { H ( s _ { i } ) = \frac { r ( s _ { i } ) } { g ( s _ { i } ) } } \end{array}$ . Based on the battery capacity of the MCV, each MCV will charge the requested SNs at a particular partition, which we obtained from Algorithm 1. Suppose the $d i s ( s _ { i } , m _ { k } )$ indicates the distance between SN $s _ { i }$ and MCV $m _ { k }$ The joint charging priority $\left( \rho ( s _ { i } ) \right)$ is obtained by combining the deadline time and distance between $s _ { i }$ and $\begin{array} { r } { m _ { k } , \mathrm { i . e . , } \rho ( s _ { i } ) = \frac { H ( s _ { i } ) } { d i s ( s _ { i } , m _ { k } ) } } \end{array}$ . The MCV will charge the requested SNs according to the sorted order of the joint charging priority, and an SN $s _ { i }$ with the maximum value of the $\rho ( s _ { i } )$ will be served first. The formula indicates that the $\rho ( s _ { i } )$ is inversely proportional to the distance between the SN and the MCV. In other words, SNs that are closer to the MCV will have higher charging priorities. In order to minimize the distance traveled during the charging process, the joint charging priority needs to be updated after the charging of each requested SN. Moreover, the constraints (8) and (9) must be satisfied to recharge the requested SNs. In Algorithm 2, we have developed the schedule for charging the SNs in a specific cluster or partition. In the same way, the charging schedule for other MCVs in their corresponding cluster or partition is designed.

![](images/f45f0ce1ef21d4e491fb23687311e94e51f0d1624338faa7042139ad4c0b0478.jpg)

![](images/47946000f28803e4c3ad371bdd646e85da3a9616b8084797345cf5ec87c0b2d0.jpg)  
Fig. 2: Charging scheduling design

Illustration 4.2. In Fig. 2 for cluster 1, the deadline time $\left( H ( s _ { i } ) \right)$ for each SN is calculated. After calculating the deadline time of S1, we have to calculate the distance from MCV 1 to S1. We can get a joint charging priority $\left( \rho ( s _ { i } ) \right)$ of S1 by dividing the deadline time of S1 by the distance from MCV 1 to S1. Similarly, the $\rho ( s _ { i } )$ value is calculated for all SNs in cluster 1, and the SNs that have the maximum joint charging priority are selected for charging. It follows the same process until it charges all the requested nodes. In this way, MCV 2 also charges the SNs in its partition.

Lemma 4.2. The worst case time complexity of the Charging Schedule() is $O ( n ^ { 2 } \times | \mathbb { M } | )$

Proof. The outer loop in lines 1 to 17 iterates over the set of MCVs, which has a size of |M|. The inner loop in lines no 5 to 6, iterates over SNs in cluster $\sigma _ { k }$ . This loop iterates until all requested SNs are scheduled. In the worst case, there will be n number of SNs in the cluster $\sigma _ { k }$ . So, it takes $O ( n )$ time. similarly, another loop in lines no 7-9 takes $O ( n )$ time. In line 10, the selection of the maximum $\rho ( s _ { i } )$ takes $O ( n )$ time. In lines 4 to 13, in the worst case, the while loop will run for $O ( n )$ time. So lines 4 to 13 takes $O ( ( n + n + n ) \times n ) \ \mathrm { O r } , O ( n ^ { 2 } )$ Therefore, the total time complexity (lines 1 to 18) of the algorithm 2 is $O ( n ^ { 2 } \times | \mathbb { M } | )$ 口

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 2: Charging_Schedule()

Input: Set of requested SNs: S, Set of MCV: M, Residual Energy of an SN:  $r(s_i)$ , Energy Consumption Rate:  $g(s_i)$ , Set of Cluster:  $\sigma$ 

Output: Charging order of the requested SNs:  $\pi_{c_k}$ 

1 foreach  $m_k \in M$  do

2    $\beta = \{s_i : s_i \in \sigma_k\}$ ; // Initialize an empty list of requested SNs

3    $\pi_{\sigma_k} = \varnothing$ ; // Initialize the empty list of charging order

4 while  $\beta \neq \varnothing$  do

5    foreach  $s_i \in \sigma_k$  do

6    Calculate dis( $s_i, m_k$ );

7    foreach  $s_i \in \sigma_k$  do

8    $H(s_i) = r(s_i)/g(s_i)$ ; // Calculate the deadline time for each SN

9    Calculate  $\rho(s_i) = H(s_i)/dis(s_i, m_k)$ ; // calculate joint charging priority of  $s_i$ 

10    Select the SN  $s_i \in A$  having maximum joint charging priority ( $\rho(s_i)$ );

11    $\pi_{\sigma_k} = \pi_{\sigma_k} + s_i$ ;

12    $\beta = \beta - s_i$ ;

13    Now the current position of  $m_k$  is the position of  $s_i$ ;

14 if constraints (8)-(9) satisfied or lower bound is reached then

15    $m_k$  will charge according to the sorted order of  $\pi_{\sigma_k}$ ;

16 else

17    $m_k$  will go to charging station;

18 return  $\pi_{\sigma_k}$  for each  $m_k$
</div>

## 4.3. Phase 3: Determining the charging schedule to avoid obstacles

In this section, we present a charging schedule that avoids the obstacles present in the charging path of an MCV. In Fig. 3, the circle represents the SNs deployed in WRSNs, and the square represents the obstacle. We have assumed that each obstacle is square; however, in reality, there can be other shapes as well. The shape of the obstacles and their corresponding anchor points can be calculated using existing techniques [32]. We have also considered that each obstacle has four anchor points (say, $A _ { 1 } ^ { i } , A _ { 2 } ^ { i } , A _ { 3 } ^ { i }$ , and $A _ { 4 } ^ { i } )$ . The set of anchor points of the obstacle $A ^ { i }$ is employed to address the obstacle problem.

Suppose there is an obstacle $A ^ { i }$ between $s _ { i }$ and $s _ { i + 1 }$ of the schedule path $\pi _ { \sigma _ { k } }$ of an MCV $m _ { k }$ in the partition $\sigma _ { k }$ . Then the Obstacle Avoidance algorithm is executed to find the shortest path between $s _ { i }$ and $s _ { i + 1 }$ , avoiding the obstacle. Let $C ^ { i } ( x _ { 0 } , y _ { 0 } )$ be the center point of the obstacle $A ^ { i }$ and $( P _ { x } ^ { i } , P _ { y } ^ { i } )$ be the projection point from the center to the path that leads from $s _ { i }$ to $s _ { i + 1 }$ . After that, we select the anchor point of $A ^ { i }$ that is closest to $( P _ { x } ^ { i } , P _ { y } ^ { i } )$ . In this way, the selected path for the schedule of MCV in the presence of an obstacle is the shortest path. It is shown in Fig. 4.

<table><tr><td> $S_{109}$ </td><td> $S_{295}$ </td><td> $S_{72}$ </td><td> $S_{198}$ </td><td> $S_{66}$ </td><td> $S_{227}$ </td><td> $S_{114}$ </td><td> $S_{48}$ </td><td> $S_{56}$ </td><td> $S_{219}$ </td><td> $S_{16}$ </td><td> $S_{203}$ </td><td> $S_{247}$ </td><td> $S_{189}$ </td><td> $S_{291}$ </td><td> $S_{89}$ </td><td> $S_{228}$ </td><td> $S_{120}$ </td><td> $S_{36}$ </td><td> $S_{134}$ </td></tr></table>

![](images/59bcc03e3bc373015aeeef5dcbc79d629c24dee2d145f13ca3afef78befed20c.jpg)  
Fig. 3: Sensor nodes and obstacles are deployed in WRSNs. The circle represents the sensor node, and the square represents an obstacle.

![](images/a16193f2cb8651d47fcffe9499925cf2334478974514e4afe1c6a8156c28f28f.jpg)  
Fig. 4: Example of a traveling path of an MCV with an obstacle

The obstacle avoidance algorithm provides the temporary result of the initialization process. In Fig. 4, the anchor point $A _ { 4 } ^ { i }$ is selected as the distance from $P ^ { i }$ to $A _ { 4 } ^ { i }$ is minimum. The MCV will choose the $A _ { 4 } ^ { i }$ point of the $i ^ { t h }$ obstacle to visit SN $s _ { i }$ to $s _ { i + 1 }$ . The obstacle avoidance algorithm is developed in Algorithm 3.

Lemma 4.3. The worst case time complexity of the obstacle avoidance Algorithm 3 is $O ( m a x | \sigma _ { k } | \times K _ { o p t i m a l } )$

Proof. Since in Algorithm 1, we have divided the network into $K _ { o p t i m a l }$ partitions and assigned an MCV to each partition, therefore the for loop in line no. 1 will run for $K _ { o p t i m a l }$ times. Next in line 2, the for loop will run for $m a x | \sigma _ { k } |$ (the partition having the maximum number of SNs) times. Lines 3 to 5 will run in constant time. Line number 6 will run four times since each obstacle has four anchor points. Line numbers 7–12 will run for a constant time. So, the total time complexity is $O ( m a x | \sigma _ { k } | \times K _ { o p t i m a l } )$ □

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 3: Obstacle avoidance algorithm
Input: Path $\pi_{\sigma_k}$ for each MCV
Output: Final path of each MCV in $\pi_{\sigma_k}'$ avoiding obstacle
1 foreach MCV do
2    for $k = 1$ to $|\sigma_k| - 1$ do
3    $\pi_{\sigma_k}' = \emptyset$; // New schedule initialized to null
4    if there is an obstacle $A^i$ between $s_i$ and $s_{i+1}$ then
5    Find the projection point $(P_x^i, P_y^i)$;
6    Find the anchor point $A_j^i$ of the obstacle $A_i$ having minimum distance from $(P_x^i, P_y^i)$;
7    Add $A_j^i$ to the schedule i.e., $\pi_{\sigma_k}' = \pi_{\sigma_k}' \cup \{s_i, A_j^i, s_{i+1}\}$;
8    else
9    $\pi_{\sigma_k}' = \pi_{\sigma_k}' \cup \{s_i, s_{i+1}\}$;
10    end
11    end
12    return $\pi_{\sigma_k}'$;
13 end
</div>

## 5. Simulation analysis

The simulation is performed using a computer with an Intel i7 processor running at 3.40 GHz and 8 GB of RAM under the MS Windows 10 OS and MATLAB R2018b. Our proposed algorithm is evaluated using several network topologies. SNs are scattered randomly throughout the zone, and the base station is placed in its corner. The MCV’s battery can be recharged or replaced at the charging station placed in the plot’s left corner. Each period, the MCV replenishes the batteries of the SNs making charging requests. Table 3 shows the network parameters and MCV configuration. The proposed scheme has been compared to and evaluated against the charging schemes presented in NJNP [18] and ACO [17]. To assess the efectiveness of the suggested approach, we primarily consider the following performance metrics: average response time, average sleeping time, total traveling distance, energy utilization eficiency, charging throughput, total received energy, charging utility, number of replenished nodes, etc. In order to conduct a fair comparison, we ran the suggested algorithm along with NJNP and ACO 30 times before averaging the results.

## 5.1. Comparison of Working Time(hours) of MCV and Charging Throughput

The charging throughput of MCV is a crucial evaluation metric for the on-demand charging approach. The number of charging requests that are successfully fulfilled in a given time unit is the charging throughput for the mobile charger. We noticed that the charging throughput of the proposed strategy increases with the number of nodes, as shown in Fig.

Table 3: Simulation parameters

<table><tr><td>Parameter description</td><td>Value of parameter</td></tr><tr><td>Sensor nodes number</td><td>100 – 300</td></tr><tr><td>Area</td><td>1000m × 1000m</td></tr><tr><td>Speed of the MCV</td><td>V = 4m/s</td></tr><tr><td>Initial energy SN</td><td>10kJ</td></tr><tr><td>MCV&#x27;s battery capacity</td><td> $10^6 J$ </td></tr><tr><td>Sleeping energy consumption rate</td><td>0.004%/s</td></tr><tr><td>Normal energy consumption rate</td><td>0.02%/s</td></tr><tr><td>Charging threshold</td><td>10%</td></tr><tr><td>Efficiency of charging (η)</td><td>0.1%/s</td></tr><tr><td>Threshold of minimum energy of sensors( $B_{min}$ )</td><td>40%</td></tr><tr><td>MCVs energy consumption in travelling</td><td>0.04kJ/s</td></tr></table>

5(a). This is because when there are fewer nodes in the network, the charging task is not as heavy. However, the charging throughput of the proposed scheme also makes significant progress when there are more nodes in the network. This is because the proposed scheme can more eficiently allocate its MCVs to charging tasks when there are more SNs. The number of nodes in the network is not constant. Nodes may die in the network over time, so the charging throughput of the scheme will fluctuate as the number of nodes in the network changes.

## 5.2. Comparison of Working Time(hours) of MCV and Energy Utilization Eficiency(%)

The percentage of energy used for charging the nodes by the MCVs is referred to as the energy-utilization eficiency of the charging strategy. The MCV is either charging the node or traveling to the node; thus, the MCV’s energy utilization eficiency is:

$$
e _ {u} = \frac {1 0 0 n _ {e c} . d _ {c}}{n _ {e c} . d _ {c} + d _ {t} . n _ {t}}\tag{16}
$$

where $n _ { e c }$ represents the rate of energy consumption while the node is operating normally. $d _ { c }$ is the duration of the charging state by the MCV, $d _ { t }$ is the rate of energy consumption while moving, and $n _ { t }$ is the MCV’s tour time. We have deployed 300 SNs in this scenario. As depicted in Fig. 5(b), the energy utilization eficiency of the proposed algorithm is much better than ACO but slightly lower than NJNP. ACO has a much lower energy utilization eficiency than the other two techniques. since selecting the quickest charging path does not result in an increase in energy utilization eficiency. Therefore, adopting the proper charging schedule for MCV is crucial to increasing the eficiency of energy utilization. First, the proposed technique chooses the far-of nodes as the charging target in order to guarantee the nodes’ survival. As a result, the MCV travels a long distance and uses more energy. That is why the energy utilization eficiency is better than the ACO. However, the energyutilization eficiency of the proposed algorithm during the first phase is somewhat lower than the NJNP approach because, at this stage, all the nodes are still alive in the proposed algorithm. In contrast, the NJNP method chooses nearby charging nodes, which reduces the travel distance of MCV. The energy utilization eficiency starts to decline when the MCV has to go farther for NJNP. However, the proposed scheme does its best to ensure nodes’ survival in comparison to NJNP and ACO; therefore, the energy utilization eficiency starts to surpass that of NJNP. Note that, in WRSNs, some nodes eventually yield to a lack of energy and die out. The suggested system, however, is unable to guarantee the survival of every node. Since the MCV needs to travel farther, the energy utilization eficiency will gradually decline.

![](images/726b319cc6077b41701b841b00c00159b9fb3f7389f8d23c56cf11eb32bc6d97.jpg)

![](images/c69088e98542187bb6babe6137c22728266eed5dc1d54abe62c2c747ea0210ab.jpg)  
Fig. 5: Comparisons, (a) working time(hours) vs. charging throughput, (b) working time (hours) vs. energy utilization eficiency(%)

## 5.3. Comparison of Working Time(hours) of MCV and Average Response Time (minutes)

The average response time is calculated based on the time between the beginning of the charging request and the receipt of the confirmation feedback. It can be observed from Fig. 6(a). that the average response time of the proposed algorithm is much lower than the NJNP and ACO. This is because, in the proposed algorithm, the charging sub-queue length is based on a more eficient clustering of the SNs and joint charging priority than existing approaches. On the other hand, the requesting nodes are continuously altering for NJNP and ACO approaches. Moreover, they are only responsible for the MCV charging schedule and cannot ensure that the nodes remain operational for a long time. In the case of NJNP and ACO, more SNs will eventually run out of energy and die because MCV will only be able to recharge the surviving nodes. Thus, the average response time will get shorter and shorter. In the proposed scheme, fewer nodes fail due to a lack of timely energy replenishment, and the length of the charging queue will also remain relatively stable. This means that the average response time of the network will be more consistent.

## 5.4. Comparison of Working Time(hours) and Number of Dead SNs

It is crucial that WRSNs have permanent and reliable nodes. Sensing in WRSNs is based on the active nodes. Sensing becomes less eficient when a large number of nodes die.

In Fig. 6(b), the NJNP and ACO techniques have a significantly higher number of dead nodes than the suggested strategy. Because the NJNP and ACO techniques do not use a particular design to ensure the nodes survive, the number of dead nodes rises dramatically as the network operates for a longer time. ACO only determines the MCV’s shorter charging route; hence, the proportion of dead nodes is the largest. Finding and serving the nearest charging node in NJNP helps reduce the number of dead nodes. In the proposed work, the remaining energy of SNs, the distance between MCV and charging node, and the energy consumption rate of SNs—these factors jointly determine the charging priority of SNs, which keeps SNs alive for a long period of time. So the number of dead SNs is the lowest in this case.

![](images/9f4bf78892a6a4ff605913c40cef5f20c9768530789a34439ef55224b00225c8.jpg)

![](images/6c8edd3aa86686a2a6a3fbd1fcb311b9116c3abcd7a338ec014f2bb673413649.jpg)  
Fig. 6: (a) Working time(hours) vs. Average response time(minutes), (b) Working time(hours) vs. Number of dead SNs.

## 5.5. Comparison of Working time(hours) of MCV and Average sleeping time(minutes)

When a node’s remaining energy falls below a threshold in the proposed strategy, it goes to sleep to conserve energy. However, nodes that are sleeping are unable to communicate with one another, and they can only sense the environment. As a result, the number of sleeping nodes and the duration of their sleep in WRSNs have a significant impact on service quality. In Fig. 7(a), both the average sleeping times for ACO and NJNP are lower than those in the suggested scheme. The MCV must determine the quickest charging route for the ACO strategy. As a result, the ACO method has the shortest average sleep duration. The MCV recharges the nearest requesting node while using the NJNP approach, and the average sleep time is greater than when using the ACO technique. However, the suggested charging technique attempts to ensure that every node is alive. The proposed charging strategy prioritizes the charging of nodes that are most likely to run out of energy first. It also takes into account the sleeping time of nodes, which means that nodes can still survive even if they have to sleep for a long period of time. However, the proposed strategy can still guarantee

Comparison of the number of clusters and the total travelling distance of MCV (m)

its survival until the MCV reaches the nodes because of the low energy consumption rate of the nodes while they are sleeping.

## 5.6. Comparison of Number of clusters and total traveling distance of MCV (m)

Another performance requirement for the charging approach is the entire distance traveled by MCVs. Fig. 7(b) compares the simulation utilizing the suggested approach with the ACO and NJNP strategies. The total distance traveled by MCVs under the proposed strategy is greater than the ACO and NJNP approaches. This is because, in ACO and NJNP techniques, the MCVs determine the shortest charging path. As a typical TSP algorithm, the ACO approach has the shortest overall travel distance. Similarly, in NJNP, the MCV chooses to charge the closest requesting node. In order to ensure nodes survive, the suggested strategy calls for the maintenance station to assign one MCV to work for each cluster whenever a node needs to be charged; thus, the MCV has the longest travel distance as a result. Moreover, the number of alive SNs in the proposed algorithm is high, as depicted in Fig. 6(b), therefore the MCV has to travel more distance to guarantee the survival of as many nodes as possible.

![](images/081c0cd372c423ac7838e96fdc2b0d9623d7cf1c7564e53ad38e8fea3fd69fa8.jpg)

![](images/96f1f5409efb763cf02af05c7eadab2852612fb8c3c7ae1f63546f440a1388f9.jpg)  
Fig. 7: (a) Working time(hours) vs. Average sleeping time(minutes), (b) Number of clusters vs. Total traveling distance of MCV (m).

## 5.7. Comparison of working time(hours) and the number of dead SNs

As shown in Fig. 8(a), the number of dead nodes in the network increases over time for various numbers of nodes in the proposed algorithm. When the number of SNs is increased, the scale of the network also rises, and the presence of obstacles also increases. So, a network with more nodes will also sufer more node deaths. When the number of nodes in the network is low, charging requests can be processed faster. However, the charging task for the MCV becomes more dificult as additional nodes with more obstacles are added to the network, so the number of dead SNs will increase.

## 5.8. Comparison of the number of nodes vs. Total received energy(J)

Fig. 8(b) depicts the total energy that the nodes received for diferent algorithms. ACO determines only the shorter route for MCVs; hence, there are more dead nodes, so less total energy is received. In NJNP, dead nodes are reduced by finding and serving the nearest charging node. However, in NJNP, the number of dead SNs is less than in ACO but more than in the proposed algorithm, as shown in Fig. 6(b). Thus, the total energy received in the NJNP is greater than ACO. The number of dead SNs in the proposed algorithm is less than in the other two schemes. Hence, MCV charges more SNs than the other two schemes. Therefore, the total energy received is highest as the number of SNs rises in the proposed work.

![](images/e36de00c29123dd273d5b1822145b308c6795401c24318b421f9780e4918f19e.jpg)

![](images/178872a554b60e53b3a2ed10d058bc429585baebb99e142b90e54acb8c440ec7.jpg)  
Fig. 8: (a) Working time(hours) vs. The number of dead SNs, (b) Number of nodes vs. Total received energy(J).

## 5.9. Comparison of number of nodes and Charging utility

The energy relationship between the overall energy acquired by SNs and the energy absorbed by the MCV is presented by the charging utility. A comparison of the charging utility results for these three strategies is shown in Fig. 9(a) The proposed method performs better than the other algorithms in terms of charging utility. The NJNP algorithm solely takes charging demand and location into account, ignoring costs associated with vehicle movement. Due to the higher total energy gain of the proposed algorithm than the other algorithms, more SNs can be recharged with the same amount of energy. Due to minor details in how the battery charging function works, the proposed technique enables nodes to achieve higher energy utility. As a result, the proposed method has a powerful charging utility.

![](images/08c131e589fff52b39f46ba89f3f2a7442dab8c1e7ac362956377840c7e035b0.jpg)

![](images/23fac699d8b812426fa630bc1b4e7f5e203751916a9490c891e70dbe420ab064.jpg)  
Fig. 9: (a) Number of nodes vs. Charging utility, (b) Number of nodes vs. Number of replenished nodes.

## 5.10. Comparison of number of nodes and number of replenished nodes

Fig. 9(b). displays the results for the number of SNs and the number of replenished SNs for various conditions. The suggested strategy simultaneously optimizes scheduling and charging time, allowing for faster replenishment of more SNs while reducing the charging time of lower-utility SNs. The comparison algorithm takes more time and efort to recharge those SNs. Both of these factors are therefore unavailable to replenish fresh SNs.

![](images/8ea6876d7f8119f6b681b9e403a753a82b17709e4be4c23da89478c5f5c850ce.jpg)

![](images/2703b3faeac0e108a74ee04eb9e4383e63ec129e8b339f79f258d4f67673a4f0.jpg)  
Fig. 10: (a) Total energy consumption(Kj) vs. Number of obstacles, (b) Total traveling distance of MCV(m) vs. Number of obstacles.

## 5.11. Comparison of the number of obstacles and total energy consumption of MCV

Fig. 10(a) displays the results of the number of obstacles with the total energy consumption of MCV with various numbers of SNs. The total energy consumption of MCV increases with the increase in the number of obstacles in the network. This is because when there are more obstacles in the network MCV needs to travel more and it consumes more energy. If more SNs are present in the network then MCV needs to travel more to fulfill the charging requests of the SNs. So, with the increase in the number of SNs, the total energy consumption of MCV increases.

## 5.12. Comparison of the total traveling distance of MCV and number of obstacles

Fig. 10(b) depicts the results of the total traveling distance of MCV and the number of obstacles with various numbers of SNs. If the number of obstacles increases the total traveling distance of MCV also increases because MCV needs to travel more to overcome those obstacles and charge the SNs. When the number of SNs increases from 100 to 300 the traveling distance of MCV also increases because MCV needs to charge more SNs so it needs to travel more.

## 5.13. Statistical Validation

In order to statistically validate the simulation’s results, we conduct a one-way ANOVA test [35] in this phase. Specifically, the following two cases are taken into consideration:

1. Null hypothesis $\left( H _ { 0 } \right)$ : Diferent groups are assumed to have equal means. Following is the null hypothesis:

$$
H _ {0}: \mu_ {N J N P} = \mu_ {A C O} = \mu_ {P r o p o s e d}\tag{17}
$$

2. Alternate hypothesis $\left( H _ { 1 } \right)$ : In general, it is assumed that the means of diferent groups are diferent. Following is the null hypothesis:

$$
H _ {1}: \mu_ {N J N P} \neq \mu_ {A C O} \neq \mu_ {P r o p o s e d}\tag{18}
$$

ANOVA test-specific parameters are the p-value, F-statistic, and F-critical. The diferentiation is concluded by rejecting $H _ { 0 }$ . In the absence of significant level( $\alpha = 0 . 0 5 )$ p-values and Fstatistics over the F-critical value, $H _ { 0 }$ must be rejected. The data input summary is given in table 4. The test result is given in Table 5. Moreover, we have included charging utility as a test parameter in our ANOVA. Among the parameters, this one plays a critical role in the sustainability of the proposed scheme. The F-Critical value is 3.40 for the inputs. We can reject $H _ { 0 }$ since the value of F-Stat > F-Critical. An alternative hypothesis $\left( H _ { 1 } \right)$ thus becomes accepted. Therefore, we discover that the charging utility for NJNP, ACO, and the proposed system is very diferent. The 95% confidence interval of the means for the stated parameter is displayed in Fig. 11.

Table 4: Data Input Summary

<table><tr><td colspan="5">Data Summary</td></tr><tr><td>Groups</td><td>N</td><td>Mean</td><td>Std. Dev.</td><td>Std. Error</td></tr><tr><td>Group 1</td><td>9</td><td>9.0111</td><td>2.5678</td><td>0.8559</td></tr><tr><td>Group 2</td><td>9</td><td>5.2444</td><td>1.5962</td><td>0.5321</td></tr><tr><td>Group 3</td><td>9</td><td>2.3556</td><td>0.6064</td><td>0.2021</td></tr></table>

Table 5: Output of ANOVA test

<table><tr><td colspan="6">ANOVA Summary</td></tr><tr><td>Source</td><td>Degree of freedom (DF)</td><td>Sum of squares (SS)</td><td>Mean square (MS)</td><td>F-stat</td><td>p-value</td></tr><tr><td>Between Groups</td><td>2</td><td>200.4866</td><td>100.2433</td><td>32.6252</td><td>0</td></tr><tr><td>Within Groups</td><td>24</td><td>76.0734</td><td>3.1697</td><td>-</td><td>-</td></tr><tr><td>Total</td><td>26</td><td>276.56</td><td>-</td><td>-</td><td>-</td></tr></table>

![](images/72a87bec24bc10e14ffe6db286ca7c6c76118367e981ecb47fd22cc86e31496f.jpg)  
Fig. 11: Comparison of 95% confidence interval for mean

As a result, statistically significant diferences exist between the three algorithms’ charging utility means. We are unable to identify how diferent the groups are from one another. This is shown by performing a least significant diference (LSD) post hoc analysis [36] on the ANOVA test results. In this research, we calculate the 95% confidence interval for mean diferences. It should be observed that if this interval does not include zero, then the means of the two groups under consideration diverge. In Table 6, the outcomes of the LSD post hoc analysis are shown. The charging utility of the suggested algorithm is thus statistically considerably higher than that of ACO and NJNP, according to the LSD post hoc analysis of the ANOVA test.

## 6. Conclusion

This paper has proposed a spatial and event-collaborative charging strategy for WRSNs with obstacles involving multiple MCVs. The charging requests are sorted based on the temporal, spatial, and event preferences presented in this strategy. The network is divided into sub-areas using clustering algorithms, and each cluster is assigned an MCV. In order to solve the scheduling problem with obstacles, an obstacle avoidance algorithm based on anchor points and projection points is employed. The proposed algorithm is extensively simulated and compared with two other existing ones. For the purpose of demonstrating the significance of the proposed study, an ANOVA statistical analysis is carried out. The performance of WRSNs can be significantly improved by the proposed scheme. However, machine-learningbased techniques will be considered in future work, which includes reinforcement learning, so that MCVs can further reduce their energy consumption. As part of the 3D modeling, we will also consider the deadline for sensor data delivery and obstacles.

Table 6: Analysis summary of LSD post hoc

<table><tr><td>Between Groups</td><td>LSD Difference</td><td>Lower Bound</td><td>Upper Bound</td><td>p-value</td></tr><tr><td>Proposed ACO</td><td>2.0345</td><td>3.7667</td><td>5.4988</td><td>1.5270e-04</td></tr><tr><td>Proposed NJNP</td><td>4.9234</td><td>6.6556</td><td>8.3877</td><td>3.6849e-08</td></tr></table>

## 520 Declarations

Ethical Approval: Not applicable.

Availability of data and materials: Not applicable.

Funding: This research received no external funding.

Conflict of interest: The authors declare no conflict of interest.

## 525 References

[1] Jiming Chen, Kang Hu, Qi Wang, Yuyi Sun, Zhiguo Shi, and Shibo He. Narrowband internet of things: Implementations and applications. IEEE Internet of Things Journal, 4(6):2309–2314, 2017.

[2] Shibo He, Dong-Hoon Shin, Junshan Zhang, Jiming Chen, and Youxian Sun. Fullview area coverage in camera sensor networks: Dimension reduction and near-optimal solutions. IEEE Transactions on Vehicular Technology, 65(9):7448–7461, 2015.

[3] Milica Pejanovi´c Duriˇsi´c, Zhilbert Tafa, Goran Dimi´c, and Veljko Milutinovi´c. A survey of military applications of wireless sensor networks. In 2012 Mediterranean conference on embedded computing (MECO), pages 196–199. IEEE, 2012.

[4] Lintong Jiang, Xiaobing Wu, Guihai Chen, and Yuling Li. Efective on-demand mobile charger scheduling for maximizing coverage in wireless rechargeable sensor networks. Mobile Networks and Applications, 19(4):543–551, 2014.

[5] B Majone, F Viani, E Filippi, A Bellin, A Massa, G Toller, F Robol, and M Salucci. Wireless sensor network deployment for monitoring soil moisture dynamics at the field scale. Procedia Environmental Sciences, 19:426–435, 2013.

[6] Ahmad H Dehwah, Jef S Shamma, and Christian G Claudel. A distributed routing scheme for energy management in solar powered sensor networks. Ad Hoc Networks, 67:11–23, 2017.

[7] Vinod R Challa, MG Prasad, and Frank T Fisher. Towards an autonomous self-tuning vibration energy harvesting device for wireless sensor network applications. Smart Materials and Structures, 20(2):025004, 2011.

[8] Hyun Jun Jung, Yooseob Song, Seong Kwang Hong, Chan Ho Yang, Sung Joo Hwang, Se Yeong Jeong, and Tae Hyun Sung. Design and optimization of piezoelectric impactbased micro wind energy harvester for wireless sensor network. Sensors and Actuators A: Physical, 222:314–321, 2015.

[9] Liguang Xie, Yi Shi, Y Thomas Hou, and Andwenjing Lou. Wireless power transfer and applications to sensor networks. IEEE Wireless Communications, 20(4):140–145, 2013.

[10] Ruilong Deng, Yongmin Zhang, Shibo He, Jiming Chen, and Xuemin Shen. Maximizing network utility of rechargeable sensor networks with spatiotemporally coupled constraints. IEEE Journal on Selected Areas in Communications, 34(5):1307– 1319, 2016.

[11] Xiaoguo Ye and Weifa Liang. Charging utility maximization in wireless rechargeable sensor networks. Wireless Networks, 23(7):2069–2081, 2017.

[12] Abhinav Tomar, Lalatendu Muduli, and Prasanta K Jana. A fuzzy logic-based ondemand charging algorithm for wireless rechargeable sensor networks with multiple chargers. IEEE Transactions on Mobile Computing, 20(9):2715–2727, 2020.

[13] Yuhou Wang, Ying Dong, Shiyuan Li, Hao Wu, and Mengyao Cui. Crcm: A new combined data gathering and energy charging model for wrsn. Symmetry, 10(8):319, 2018.

[14] Ying Dong, Yuhou Wang, Shiyuan Li, Mengyao Cui, and Hao Wu. Demand-based charging strategy for wireless rechargeable sensor networks. ETRI Journal, 41(3):326– 336, 2019.

[15] Ying Dong, Guangjiu Bao, Yuhong Liu, Ming Wei, Yuxin Huo, Zhiyuan Lou, Yong Wang, and Chunyue Wang. Instant on-demand charging strategy with multiple chargers in wireless rechargeable sensor networks. Ad Hoc Networks, 136:102964, 2022.

[16] Jau-Yang Chang, Jin-Tsong Jeng, Yung-Hoh Sheu, Z Jian, Wei-Yeh Chang, et al. An eficient data collection path planning scheme for wireless sensor networks with mobile sinks. EURASIP Journal on Wireless Communications and Networking, 2020(1):1–23, 2020.

[17] Zhenchun Wei, Chengkai Xia, Xiaohui Yuan, Renhao Sun, Zengwei Lyu, Lei Shi, and Jianjun Ji. The path planning scheme for joint charging and data collection in wrsns: A

multi-objective optimization method. Journal of Network and Computer Applications, 156:102565, 2020.

[18] Liang He, Linghe Kong, Yu Gu, Jianping Pan, and Ting Zhu. Evaluating the on-demand mobile charging in wireless sensor networks. IEEE Transactions on Mobile Computing, 14(9):1861–1875, 2014.

[19] Niayesh Gharaei, Yasser D Al-Otaibi, Suhail Ashfaq Butt, Sharaf Jameel Malebary, Sabit Rahim, and Gul Sahar. Energy-eficient tour optimization of wireless mobile chargers for rechargeable sensor networks. IEEE Systems Journal, 15(1):27–36, 2020.

[20] Abhinav Tomar, Kumar Nitesh, and Prasanta K Jana. An eficient scheme for trajectory design of mobile chargers in wireless sensor networks. Wireless Networks, 26(2):897–912, 2020.

[21] Chi Lin, Zhiyuan Wang, Ding Han, Youkun Wu, Chang Wu Yu, and Guowei Wu. Tadp: Enabling temporal and distantial priority scheduling for on-demand charging architecture in wireless rechargeable sensor networks. Journal of Systems Architecture, 70:26–38, 2016.

[22] Abhinav Tomar, Lalatendu Muduli, and Prasanta K Jana. An eficient scheduling scheme for on-demand mobile charging in wireless rechargeable sensor networks. Pervasive and Mobile Computing, 59:101074, 2019.

[23] Haipeng Dai, Qiufang Ma, Xiaobing Wu, Guihai Chen, David KY Yau, Shaojie Tang, Xiang-Yang Li, and Chen Tian. Chase: Charging and scheduling scheme for stochastic event capture in wireless rechargeable sensor networks. IEEE Transactions on Mobile Computing, 19(1):44–59, 2018.

[24] Zhenchun Wei, Meng Li, Qing Zhao, Zengwei Lyu, Siwei Zhu, and Zhen Wei. Multi-mc charging schedule algorithm with time windows in wireless rechargeable sensor networks. IEEE Access, 7:156217–156227, 2019.

[25] Yang Yang, He Li, Xuesong Qiu, Shaoyong Guo, XiaoXiao Zeng, Kang Zhao, and Haoran Xin. Research on lifetime prediction-based recharging scheme in rechargeable wsns. In NOMS 2018-2018 IEEE/IFIP Network Operations and Management Symposium, pages 1–4. IEEE, 2018.

[26] Yuhou Wang, Ying Dong, Shiyuan Li, Ruoyu Huang, and Yuhao Shang. A new ondemand recharging strategy based on cycle-limitation in a wrsn. Symmetry, 11(8):1028, 2019.

[27] Yu Ma, Weifa Liang, and Wenzheng Xu. Charging utility maximization in wireless rechargeable sensor networks by charging multiple sensors simultaneously. IEEE/ACM Transactions on Networking, 26(4):1591–1604, 2018.

[28] Liang He, Yanyan Zhuang, Jianping Pan, and Jingdong Xu. Evaluating on-demand data collection with mobile elements in wireless sensor networks. In 2010 IEEE 72nd Vehicular Technology Conference-Fall, pages 1–5. IEEE, 2010.

[29] Chi Lin, Youkun Wu, Zhicheng Liu, Mohammad S Obaidat, Chang Wu Yu, and Guowei Wu. Gtcharge: A game theoretical collaborative charging scheme for wireless rechargeable sensor networks. Journal of Systems and Software, 121:88–104, 2016.

[30] Zhenguo Gao, Yan Chen, Liling Fan, Haijun Wang, Scott Chih-Hao Huang, and Hsiao-Chun Wu. Joint energy loss and time span minimization for energy-redistributionassisted charging of wrsns with a mobile charger. IEEE Internet of Things Journal, 10(5):4636–4651, 2023.

[31] Jiayuan Chen, Changyan Yi, Ran Wang, Kun Zhu, and Jun Cai. Learning aided joint sensor activation and mobile charging vehicle scheduling for energy-eficient wrsn-based industrial iot. IEEE Transactions on Vehicular Technology, 2022.

[32] Hongseok Cheon and Byung Kook Kim. Online bidirectional trajectory planning for mobile robots in state-time space. IEEE Transactions on Industrial Electronics, 66(6):4555–4565, 2018.

[33] Shibo He, Jiming Chen, Fachang Jiang, David KY Yau, Guoliang Xing, and Youxian Sun. Energy provisioning in wireless rechargeable sensor networks. IEEE Transactions on Mobile Computing, 12(10):1931–1942, 2013.

[34] Akash Punhani, Neetu Faujdar, Krishna Kumar Mishra, and Manoharan Subramanian. Binning-based silhouette approach to find the optimal cluster using k-means. IEEE Access, 10:115025–115032, 2022.

[35] Keith E Muller and Bethel A Fetterman. Regression and ANOVA: an integrated approach using SAS software. John Wiley & Sons, Inc., 2003.

[36] Janez Demˇsar. Statistical comparisons of classifiers over multiple data sets. The Journal of Machine learning research, 7:1–30, 2006.