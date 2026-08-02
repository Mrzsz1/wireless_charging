---
title: "Charging Scheduling of Clustered Wireless Rechargeable Sensor Networks Considering Dynamic Selection of Cluster Heads"
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
pdf_path: "raw/canonical/Charging_Scheduling_of_Clustered_Wireless_Rechargeable_Sensor_Networks_Considering_Dynamic_Selec/Charging_Scheduling_of_Clustered_Wireless_Rechargeable_Sensor_Networks_Considering_D.pdf"
raw_md: "raw/canonical/Charging_Scheduling_of_Clustered_Wireless_Rechargeable_Sensor_Networks_Considering_Dynamic_Selec/full.md"
---
https://doi.org/10.32604/cmc.2026.078181

ARTICLE

![](images/73117d1f6da666060b750166dc42dd83869de3da6de196c329add9846ac9c2ce.jpg)

# Charging Scheduling of Clustered Wireless Rechargeable Sensor Networks Considering Dynamic Selection of Cluster Heads

Mengqi Liu and Haiqing Yao

Institute of Logistics Science and Engineering, Shanghai Maritime University, Shanghai, China

\*Corresponding Author: Haiqing Yao. Email: hqyao@shmtu.edu.cn

Received: 25 December 2025; Accepted: 19 March 2026; Published: 08 May 2026

ABSTRACT: For the wide-coverage application scenarios, wireless rechargeable sensor networks are normally divided into multiple clusters to support the diversity and flexibility for monitoring, and use the mobile charger (MC) to support the sustainable charging ofthe network. Many eforts focus on optimizing the cluster head selection and mobile charger scheduling to improve the network energy eficiency and reliability. However, the existing work tends to use fixed triggering mechanism for cluster head (CH) rotation, and may trigger the rotation either too early or too late. Besides, the existing charging triggering mechanisms cannot track the changes in network topology in real time. As a result, both the network energy eficiency and the node failure rate degenerate correspondingly. To solve these problems, this work proposes a dynamic cluster head selection algorithm (DCHSA), which evaluates potential candidate CH sets based on the energy consumption, remaining energy and topological structure, and then select a new CH within this set based on the CH rotation energy consumption and the candidate CH evaluation mechanism. Furthermore, an adaptive dual-threshold selection algorithm based on dynamic energy consumption (ADTSA-DEC) is proposed to determine the set of requiring charging nodes and the trigger time for charging scheduling. The particle swarm optimization is then employed to implement the charging scheduling. Finally, extensive simulations validate that the newly proposed algorithms have outstanding accuracy and robustness in improving overall network energy eficiency and node survivability compared with existing methods

KEYWORDS: Clustered wireless rechargeable sensor networks; cluster head rotation; adaptive dual-threshold; charging scheduling strategy; particle swarm optimization

## 1 Introduction

In recent years, wireless sensor networks (WSNs) have gradually become an important mean of perception due to its technical characteristics such as self-organization and easy deployment, and have been widely used in environment monitoring, intelligent building, industrial and agricultural monitoring and other fields [1–4]. To alleviate the limited energy supply in WSNs for long-term online operation, Wireless Power Transfer (WPT) technology with the advantages of low cost and convenience has been tested to be one of the efective means [5,6]. Specifically, the nodes in Wireless Rechargeable Sensor Networks (WRSN) are equipped with wireless energy receiving modules, and mobile charger (MC) that can be the unmanned vehicle or drone with wireless energy transmission modules can automatically recharge the nodes under the guidance of charging strategy. As a result, the charging scheduling of MC solves the problem of which nodes to be charge, when to charge, and in what order to charge [7,8].

Basically, the topology of a WRSN can be clustered or non-clustered. A non-clustered network is normally low capacity and small coverage, and thus its charging scheduling burden on MC is lightweight [9]. Correspondingly, in wide-coverage application scenarios, the clustered WRSN trends to be deployed in diferent clusters based on geographical location, energy consumption levels, sensor types, or other criteria. Then, the charging scheduling of MC have to be properly designed with the constraints of imbalance and uncertainty of node energy consumption, and the overall energy eficiency and reliability in network applications [10]. Specifically, the change in topology structure triggers the fluctuation of node energy consumption, which can be autonomous or passive. In the autonomous factors, the rotation mechanism of CH and dynamic routing mechanism attract lots of research interests for their outstanding performance in balancing node energy consumption and extending network survivability. However, existing research on charging scheduling has not yet considered the impact of the rotation mechanism of CH on the charging node set, resulting in shortcomings in optimizing the overall performance of the network.

In terms of CH selection mechanism, most previous studies focus on the criteria for CH selection and the mechanism of new CH rotation [11,12]. The rotation of CH improves the energy consumption balance of nodes in the cluster based on the change of network topology, and then improve the survival rate of nodes. The earliest research on CH rotation defines the candidate set for the new CH as all other nodes in the cluster, and randomly select the new CH [13]. Then, many eforts select the new CH based on the residual energy of nodes, the distance between nodes and the number of neighboring nodes [14–17]. In [18], the researcher utilize artificial neural networks and Bayesian regularization algorithms to optimize the CH selection and routing process of the LEACH-D algorithm, efectively reducing transmission overhead and idle listening, and extending the network lifetime while reducing network energy consumption When the edge nodes in the cluster are selected as new CH or facing the network with non-uniform distribution, these eforts can lead to the increase of energy consumption and energy imbalance of the whole cluster, and also lead to the increase of node failure rate, and thus are not applicable. In addition, some work chooses nodes with low residual energy as candidate CH at the cost ofthe charging frequency ofMC [19]. The edge nodes in the cluster and the sparse nodes in the non-uniform distribution network are all located at the end ofthe topology. All the above work inevitably choose such nodes as CH. To limit the range of candidate CHs, some relevant studies select nodes with higher residual energy in the cluster as candidate CH sets. However, these methods still face the problem of uneven energy consumption among nodes in the cluster [20]. Besides, to trigger the CH rotation, existing research predominantly employs predefined cycles or predetermined thresholds to initiate new CH rotations [19–21]. However, this fixed triggering mechanism is dificult to adapt to the random impact of interference factors such as wireless link reliability, sudden event sampling, and data retransmission on node energy consumption. Consequently, it may trigger CH rotations either too early or too late, and thus reduces the network energy eficiency and increases the node failure rate.

Furthermore, to appropriately trigger the MC for charging, the existing charging scheduling strategies can be divided into periodic and on-demand types. The periodic charging scheduling strategy is obviously unable to meet the charging requirement when the network topology changes, and the resulting overcharging or undercharging problems can lead to network energy waste and node failure [22]. Correspondingly, in order to meet the real-time energy consumption requirements of the network, various threshold based charging triggering strategies have been proposed [14–16,23]. However, considering the issue of time-varying network energy consumption caused by CH rotation in the clustered WRSN, the existing charging triggering mechanisms cannot track the changes in network topology in real time, which in turn cannot meet the realtime charging needs of the network, and can also cause problems such as energy waste and node failure, and thus become an urgent challenge to be solved.

To address these problems, this work first proposes the Dynamic Cluster Head Selection Algorithm (DCHSA). During the operation of the network, the CH nodes collect information such as the energy consumption, remaining energy, and topological structure of intra-cluster nodes, to evaluate potential candidate CH sets. Then, a new CH is selected from this set based on the CH rotation energy consumption and the candidate CH evaluation mechanism. Furthermore, an Adaptive Dual-Threshold Selection Algorithm Based on Dynamic Energy Consumption (ADTSA-DEC) is proposed, which integrates information such as energy consumption, residual energy, topological structure, and spatial distribution to calculate the Warning Threshold energy (WT) and Danger Threshold energy (DT) for each node to determine the set of nodes for energy replenishment and the trigger time for MC scheduling, separately. Particle Swarm Optimization is then used to control the charging process. Finally, extensive simulations have validated that the newly proposed algorithm has outstanding accuracy and robustness in improving overall network energy eficiency and node survivability. The innovative contributions of this work are detailed as follows:

(1) A dynamic cluster head selection algorithm (DCHSA) is proposed based on the energy consumption, remaining energy and topological structure of nodes within the cluster. It calculates the real-time variance of remaining energy and the maximum value of the variance of remaining energy of nodes, and evaluates the candidate CH set in combination with the spatial distribution within the cluster. Then, a new CH is obtained based on the CH rotation energy consumption and the candidate CH evaluation mechanism.

(2) An adaptive dual-threshold selection algorithm based on dynamic energy consumption (ADTSA-DEC) is proposed. By integrating the node energy consumption, residual energy, topological structure, and spatial distribution, the ADTSA-DEC calculates the WT in real time to determine the set of nodes requiring charging. Subsequently, it optimizes the DT to determine the triggering moment for MC scheduling.

(3) A charging scheduling strategy for clustered wireless rechargeable sensor networks with sparse deployment (CWRSN-CSS) is proposed, which incorporates the constraints of mobile energy consumption, charging energy consumption, and overall node failure rate. Based on the DCHSA and ADTSA-DEC, the CWRSN-CSS employs a particle swarm optimization algorithm to achieve charging scheduling for a single mobile charger. Extensive simulation analyses demonstrate that the algorithm exhibits outstanding scalability, accuracy, and robustness, and thus contributes to both theoretical advancement and practical applications of WRSNs.

The remainder of this work is structured as follows. Section 2 reviews related work. Section 3 proposes the network model and problem formulation. Section 4 introduces a dynamic cluster head selection algorithm and an adaptive dual-threshold algorithm, and thus proposes a charging scheduling strategy for WRSN that incorporates CH rotation and sparse node deployment. Section 5 presents simulation analyses and experimental validation. Finally, Section 6 concludes the work.

## 2 Related Work

## 2.1 CH Selection Mechanism

Firstly, the relevant eforts on the CH selection methods and the trigger strategy for CH rotation is reviewed. Fanian and Kuchaki Rafsanjani [20] establish a fuzzy rule table corresponding to candidate nodes, selects the node with the maximum fuzzy output as the CH. Chaurasia et al. [22] propose a metaheuristicbased optimization algorithm for CH selection to minimize both node energy consumption and data transmission latency. The CH selection process dynamically incorporates node density, residual energy, distance between cluster heads and the base station, and inter-cluster distances. Qamar et al. [24] schedule mobile sink to rendezvous points (rather than network CH nodes) in the cluster distribution space for information collection within the clusters, and used a weight function and k-means clustering to select rendezvous points, in order to improve energy eficiency and ensure network coverage efectiveness. Ismail et al. [25] proposed a rule-based energy-eficient routing protocol, in which the optimal CH is selected based on the residual energy, distance, node density, link quality, and load. Ri and Kim [26] propose a novel uneven cluster-based routing protocol. This protocol assigns relatively accurate weights to 7 multicriteria characterizing sensor nodes, and selects CH nodes based on these weights. Dong et al. [27] propose a spatiotemporal joint charging strategy, which trigger the CH rotation when the residual energy ofthe current CH falls below the average residual energy within the cluster. Choudhury et al. [28] propose a non-threshold cluster head rotation scheme, which elucidates that the energy consumption of cluster heads primarily arises from four aspects: data aggregation, data forwarding, maintenance ofthe cluster routing network, and cluster head rotation. Specifically, this scheme derives optimal rotation time and CH selection to extend the network lifetime. Mostarda et al. [29] employs the integer linear programming to derive an optimal rotation strategy. This strategy generates a scheduling table for each node during the cluster formation phase, and thus reduces the overhead associated with cluster formation and CH election.

These studies evaluate the selection ofnew CH by considering factors such as residual energy and spatial distribution of nodes, and propose fixed update cycles or rotation factors to trigger CH rotation. However, few studies have considered imposing reasonable constraints on candidate CH nodes, and triggered the CH rotation based solely on the states of these candidate nodes. As a result, these studies fails to account for the dynamic changes in network topology under CH rotation, and can easily lead to biases in CH selection and cause rotation to occur either prematurely or with delay.

## 2.2 The Trigger Mechanism for MC Charging

The charging trigger mechanism is mainly about when to trigger MC charging. Li et al. [23] adopt a periodic charging mode, where a charging operation is inserted at the last time in each data aggregation cycle. Lee et al. [30] use a fixed-threshold charging mode based on the residual energy of nodes. Shang et al. [31] manage the node’s residual energy and calculate an adaptive single charging threshold for each node in a distributed manner. Jiang et al. [32] employ convolutional neural networks to extract environmental state features and gate recurrent units to predict charging decisions, in which MC continuously executes charging operation. As a preliminary work for this article, we investigate the real-time optimization of node charging thresholds and MC charging paths to maximize MC energy eficiency and minimize node data loss in [33]. An adaptive dual-threshold algorithm based on a static routing network to compute node thresholds is proposed, where MC charging is triggered when the remaining energy of a node falls below the warning threshold. Bian et al. [34] propose an equilibrium allocation strategy for charging requests based on a dynamic dualthreshold mode, where the MC carries out the charging task when a certain number of nodes are below the dual-threshold. In summary, the existing charging triggering mechanisms ignore the time-varying network energy consumption caused by CH rotation in the clustered WRSN, and thus can cause problems such as energy waste and node failure.

Table 1 summarizes the related work. Regarding the metrics for CH selection, “Dynamic CH” indicates whether CH rotation is implemented in clustered network, while “Candidate CH” denotes whether constraints are imposed on candidate CH nodes during the selection process. For the trigger mechanism ofMC, it refers to the triggering method for MC to start executing energy replenishment tasks for nodes.

It is clear that the existing studies tend to ignore the dynamic CH selection or has limitations in adapting to intra-cluster topology variations, CH selection criteria and rotation trigger. Besides, few scholars address the selection of candidate CH sets, and the existing studies with candidate CH nodes rarely considered the energy consumption imbalance in the cluster. In the charging trigger mechanisms, the impact of dynamic node energy consumption on threshold values has not been addressed. To bridge this gap, this work introduces a dynamic CH selection algorithm and an adaptive dual-threshold strategy, and then sequentially address the challenges of charging scheduling of MC in clustered WRSN.

Table 1: Summary of related work.

<table><tr><td rowspan="2">Paper</td><td colspan="2">CH Selection</td><td rowspan="2">The Trigger Mechanism for MC Charging</td></tr><tr><td>Dynamic CH</td><td>Candidate CH</td></tr><tr><td>[20]</td><td>√</td><td>Nodes with above average residual energy</td><td>Adaptive single threshold</td></tr><tr><td>[22]</td><td>×</td><td>×</td><td>Periodic charging</td></tr><tr><td>[24]</td><td>√</td><td>×</td><td>Periodic charging</td></tr><tr><td>[25]</td><td>×</td><td>×</td><td>Charging scheduling is not involved</td></tr><tr><td>[26]</td><td>×</td><td>×</td><td>Charging scheduling is not involved</td></tr><tr><td>[27]</td><td>√</td><td>×</td><td>Fixed-threshold charging</td></tr><tr><td>[28]</td><td>√</td><td>×</td><td>Charging scheduling is not involved</td></tr><tr><td>[29]</td><td>√</td><td>×</td><td>Charging scheduling is not involved</td></tr><tr><td>[30]</td><td>×</td><td>×</td><td>Fixed-threshold charging</td></tr><tr><td>[31]</td><td>×</td><td>×</td><td>Adaptive single threshold</td></tr><tr><td>[32]</td><td>×</td><td>×</td><td>Fixed-threshold charging</td></tr><tr><td>[33]</td><td>×</td><td>×</td><td>Adaptive dual-threshold</td></tr><tr><td>[34]</td><td>×</td><td>×</td><td>Adaptive dual-threshold</td></tr><tr><td>Our</td><td>√</td><td>Candidate CH Algorithm for node energy balancing</td><td>Adaptive dual-threshold based on energy consumption dynamics</td></tr></table>

## 3 Problem Model and Definition

## 3.1 Network Model

As illustrated in Fig. 1, a two-dimensional square region ψ with side length L is used, where the MC adopts one-to-one charging mode for nodes to simulate sparse deployment of clustered WRSN [34,35]. The region ψ is partitioned into multiple square subregions with side length $L _ { c }$ . The homogeneous wireless sensor nodes $S = \left\{ s _ { 1 } , s _ { 2 } , \ldots \ldots , s _ { n } \right\}$ are evenly divided into $N _ { c }$ clusters $C = \{ C _ { 1 } , C _ { 2 } , \ldots , C _ { N _ { c } } \}$ and randomly deployed within $N _ { c }$ subregions to simulate the uneven deployment of networks. A node s $\left( 1 \leq i \leq n \right)$ is fixed after deployment with the known location $\left( x _ { i } , y _ { i } \right)$ , and is equipped with a wireless energy harvesting module. A base station (BS) is positioned at the center of ψ to replenish energy for the MC, aggregate data collected by the MC, and schedule the MC to recharge nodes. Considering that the coverage area of such networks is in the range of kilometers, each cluster first aggregates data by the CH, and then the BS aggregate data with the help of MC. This approach can achieve outstanding network energy eficiency and reliability at the cost of data transmission eficiency. Consequently, it is suitable for applications such as environmental monitoring, smart agriculture, and other sparse and large-area scenarios.

At the beginning of network operation, all nodes are equipped with fully charged batteries with the capacity of $E _ { s }$ . The residual energy of node $s _ { i }$ is denoted as $\ b { p } _ { i }$ . Two thresholds, namely WT and DT, are established for each node. At the commencement of each charging cycle, the MC departs from the BS and sequentially replenishes the energy of nodes in the charging queue according to the charging sequence planned by the BS, until each node reaches its full energy capacity. Upon completing energy replenishment for all nodes in the charging set, the MC returns to the BS to recharge itself and uploads the collected node information, which serves as a reference for the BS to plan the subsequent charging task, thereby marking the completion of the current charging cycle. Table 2 lists the parameters utilized in this work along with their corresponding definitions.

![](images/f511206e9853892b05f660ce8021ccb2d50456469ea34b1b68f5637959c0995e.jpg)  
Figure 1: Network model diagram.

Table 2: Summary of related parameters.

<table><tr><td>Parameters</td><td>Explanation</td><td>Parameters</td><td>Explanation</td></tr><tr><td> $\psi$ </td><td>The two-dimensional area where the problem model is located</td><td> $n_{perce}$ </td><td>The data collection rate of node  $s_i$ </td></tr><tr><td>L</td><td>The side length of the  $\psi$ </td><td> $e_{perce}$ </td><td>The energy consumption per the unit data sensed by the node</td></tr><tr><td>n</td><td>The number of sensor nodes in  $\psi$ </td><td> $b_i^{trans}$ </td><td>The data transmission rate of node  $s_i$  per second</td></tr><tr><td>N</td><td>The number of sub-regions in  $\psi$ </td><td> $b_i^{rece}$ </td><td>The data reception rate of node  $s_i$  per second</td></tr><tr><td> $L_c$ </td><td>The side length of the sub-region in  $\psi$ </td><td> $\nu$ </td><td>The movement speed of the MC</td></tr><tr><td> $N_c$ </td><td>The number of clusters in the network, and  $1 \leq N_c \leq N$ </td><td> $E_i^e$ </td><td>The energy consumption rate of node  $s_i$ </td></tr><tr><td>T</td><td>The operation time of network</td><td> $E_c$ </td><td>The total charging energy consumption of MC within T</td></tr><tr><td> $s_i$ </td><td>The sensor node in the network, and  $1 \leq i \leq n$ </td><td> $E_m$ </td><td>The total mobile energy consumption of MC within T</td></tr></table>

(Continued)

<table><tr><td colspan="4">Table 2 (continued)</td></tr><tr><td>Parameters</td><td>Explanation</td><td>Parameters</td><td>Explanation</td></tr><tr><td> $B$ </td><td>The battery capacity of the MC</td><td> $\theta$ </td><td>The probability weight factor for a node to become CH</td></tr><tr><td> $p_i$ </td><td>The remaining energy of the node  $s_i$ </td><td></td><td></td></tr></table>

## 3.2 Energy Consumption Model

Generally, the energy consumption rate $E _ { i } ^ { e }$ of nodes $s _ { i }$ primarily consists of perception consumption rate $E _ { P e r c e }$ , transmission consumption rate $E _ { t r a n s }$ , and reception consumption rate $E _ { r e c e }$ [33]:

$$
\left\{ \begin{array}{l} E _ {p e r c e} = e _ {p e r c e} * n _ {p e r c e} \\ E _ {t r a n s} = (\beta_ {1} + \beta_ {2} * d _ {\max} ^ {\alpha}) * b _ {i} ^ {t r a n s} \\ E _ {r e c e} = \gamma * b _ {i} ^ {r e c e} \end{array} \right.\tag{1}
$$

where $d _ { \mathrm { m a x } }$ represents the maximum communication distance between nodes, α denotes the path loss exponent for data transmission, $\beta _ { 1 }$ and $\beta _ { 2 }$ are the energy consumption coeficients for data transmission, while γ signifies the energy consumption coeficient for data reception.

Then, we have $E _ { i } ^ { e } = E _ { p e r c e } + E _ { t r a n s } + E _ { r e c e } .$

## 3.3 Charging Model

In the one-to-one charging mode, the MC can recharge the nodes with a constant power when stationary, and its charging time $\tau _ { s _ { i } }$ for node $s _ { i }$ is given by [36]:

$$
\tau_ {s _ {i}} = \frac {E _ {s} - p _ {i}}{E _ {c h a r g e r} * \lambda}\tag{2}
$$

where $E _ { c h \tt a r g e r }$ denotes the charging power of the MC, and $\lambda \in [ 0 , 1 ]$ represents the decline coeficient for wireless charging.

## 3.4 Problem Model

This work aims to optimize the highly concerned energy consumption and reliability of the network, namely maximizing the network reliability at the cost of minimal overall energy consumption. The energy consumption of network encompasses both the mobility energy consumption and the charging energy consumption of the MC, while network reliability is measured by the failure rate of all nodes. To balance these three optimization objectives, normalization is applied to each metric. In summary, as the network runs for T time, the optimization objective Target\_value is formulated as:

$$
\left\{ \begin{array}{l} M i n \text { Target\_value } = \varepsilon_ {l} * \frac {N _ {\text { loss\_node }}}{N _ {\text { ch   arg   ing\_node }}} + \varepsilon_ {c} * \frac {E _ {c}}{T _ {m} * E _ {\text { ch   arg   er }} * \lambda} + \varepsilon_ {m} * \frac {E _ {m}}{T _ {m} * e _ {m} * v} \\ S. t. E _ {c} + E _ {m} \leq B \end{array} \right.\tag{3}
$$

where $\varepsilon _ { l } , \varepsilon _ { c }$ and $\varepsilon _ { m }$ are weight factors and $\varepsilon _ { l } , \varepsilon _ { c } , \varepsilon _ { m } > 0 , T _ { m }$ represent the scheduling time for mobile and charging of MC in T, $N _ { l o s s \_ n o d e }$ denotes the total number of failed nodes in $T _ { m } , N _ { c h a r g i n g \_ n o d e }$ signifies the total charging number for nodes by the MC in $T _ { m } , E _ { c }$ represents the total charging energy consumption ofthe MC in $T _ { m } , E _ { m }$ denotes the total mobility energy consumption of the MC in $T _ { m }$ , the constraint specifies that the total energy consumption during charging scheduling must not exceed the battery capacity of the MC.

## 4 Solutions

To address the problem of uneven energy consumption in clusters, a CH rotation strategy to balance energy consumption within the cluster is proposed, which gradually evaluates the candidate CH set and selects the new CH. Then, to track the changes in node energy consumption caused by dynamic topology, an adaptive dual-threshold algorithm is proposed to determine the charging node set and the triggering moment for MC. Finally, particle swarm optimization is employed to plan the MC’s charging path with the limit of energy eficient and network reliability.

## 4.1 Dynamic Cluster Head Selection Algorithm

After deployment, the location of the centroid of a cluster $C _ { j } ( 1 \leq j \leq N _ { c } )$ is calculated as follows:

$$
\left\{ \begin{array}{l} x _ {c} = \frac {1}{n _ {c} ^ {j}} \sum_ {i = 1} ^ {n _ {c} ^ {j}} x _ {i} \\ y _ {c} = \frac {1}{n _ {c} ^ {j}} \sum_ {i = 1} ^ {n _ {c} ^ {j}} y _ {i} \end{array} \right.\tag{4}
$$

where $n _ { c } ^ { j }$ represents the number of nodes within the $C _ { j } .$

The node closest to the centroid is chosen as the initial CH. Then, a practical routing protocol can be used to construct the intra-cluster topology, such as the Ad hoc On-demand Distance Vector (AODV) [37]. The energy consumption rate $E _ { C H } ^ { e }$ ofthe CH can be calculated by Eq. (1) with the network topology information, and the operational duration $t _ { C H }$ of the CH under the current network topology is $t _ { C H } = \frac { E _ { s } } { E _ { C H } ^ { e } }$

To balance the energy consumption rate of nodes within the cluster and alleviate the problem of node failure caused by hotspot efects, nodes near the centroid become a reasonable choice as subsequent candidate CHs. Therefore, a reasonable small area close to the centroid needs to be estimated. Firstly, when the CH node fails, the residual energy of all nodes in $C _ { j } { \mathrm { \scriptsize ~ i s } } p _ { i } = E _ { s } - t _ { C H } \ * \ E _ { i } ^ { e } , i = 1 , 2 , \dots , n _ { c } ^ { j } .$

Consequently, the average residual energy $\overline { { \boldsymbol { p } } }$ and residual energy variance $\sigma _ { s i m }$ of nodes in $C _ { j }$ is:

$$
\left\{ \begin{array}{l} \overline {{p}} = \frac {1}{n _ {c} ^ {j}} \sum_ {i = 1} ^ {n _ {c} ^ {j}} p _ {i} \\ \sigma_ {s i m} = \frac {1}{n _ {c} ^ {j}} \sum_ {i = 1} ^ {n _ {c} ^ {j}} (p _ {i} - \overline {{p}}) ^ {2} \end{array} \right.\tag{5}
$$

The CH manages the data aggregation and network topology in $C _ { j } ,$ and thus it has the highest energy consumption rate in $C _ { j }$ . Therefore, when the energy of the CH is exhausted, the residual energy variance $\sigma _ { s i m }$ of all nodes in $C _ { j }$ is used as a benchmark $\sigma _ { s i m } ^ { b }$ . Consequently, the dynamic selection radius r for the candidate CH set can be calculated as follows:

$$
r = \left\{ \begin{array}{l} \frac {R _ {m a x} * \sigma_ {s i m}}{\sigma_ {s i m} ^ {b}}, \sigma_ {s i m} <   \sigma_ {s i m} ^ {b} \\ R _ {m a x}, \sigma_ {s i m} \geq \sigma_ {s i m} ^ {b} \end{array} \right.\tag{6}
$$

where $R _ { m a x }$ is the maximum distance from the centroid to the nodes in $C _ { j }$

Finally, all candidate CH sets are located within a circular centered at the centroid with radius r. The candidate CH node set $C _ { C C H }$ and the scale $n _ { C C H } \left( n _ { C C H } \leq n \right)$ of $C _ { C C H }$ is obtained. Normally, the energy consumption of the CH is primarily from aggregating data from its intra-cluster nodes, and maintaining the network (such as synchronization, duty cycling, and control overhead). This work assumes a uniform data sensing rate $n _ { \mathrm { p r e c e } }$ for all nodes. Then, the energy consumption rate of the CH for data aggregation includes the energy consumption rate for data collection and data forwarding, and can be expressed as:

$$
E _ {a g g} = n _ {s} ^ {j} * n _ {\mathrm{prece}} * E _ {\mathrm{trans}} + \sum_ {i = 1} ^ {n _ {s} ^ {j}} h _ {i} * n _ {\mathrm{prece}} * E _ {\mathrm{trans}}\tag{7}
$$

where $n _ { s } ^ { j }$ represents the number of nodes with data collection in $C _ { j } , h _ { i }$ denotes the number of routing hops from a node $s _ { i }$ (with data collection function) to the CH.

Besides, the total energy consumption $E _ { C H - r o t } ( E _ { C H - r o t } = n _ { s } ^ { j } \ * \ e _ { C H - r o t } )$ for the CH rotation and the energy consumption rate $E _ { c o n }$ for network maintenance are mainly used for broadcast notifications and topology construction, and the energy consumption $e _ { C H - r o t }$ for the CH rotation of one node is assumed to be fixed [28]. Then, to select an optimal candidate CH from the $C _ { C C H }$ , the survival time for a candidate node $s _ { i }$ in $C _ { C C H }$ is estimated as follows:

$$
\varsigma_ {i} = \frac {p _ {i} - E _ {C H - r o t}}{E _ {a g g} ^ {C H} + E _ {c o n}}\tag{8}
$$

where $\ b { p } _ { i }$ denotes the residual energy of the candidate node $s _ { i } , E _ { a g g } ^ { C H }$ represents the energy consumption for data aggregation when serving as the CH.

As a result, the current CH can dynamically calculate the candidate CH node set $C _ { C C H }$ and the $\varsigma _ { i }$ of each candidate node in each data aggregation cycle. When a candidate node exhibits a lifetime longer than that of the current CH, it is selected as the new CH, and the CH rotation is triggered. The execution process of the proposed DCHSA algorithm is detailed in Algorithm 1 and its time complexity is ${ \mathrm { O } } ( { \mathfrak { n } } )$

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 1: Dynamic cluster head selection algorithm (DCHSA) for a cluster $C_j$ ($1 \leq j \leq N_c$)
Input: The $\sigma_{sim}^{max}$, $R_{max}$, $p_i = \{p_1, p_2, \ldots, p_{n_c^j}\}$
Output: The next CH in $C_j$.
1. calculate $\sigma_{sim}$ based on (4) and (5);
2. calculate $r$ based on (6), obtain $C_{CCH}$ and $n_{CCH}$;
3. for $i = 1$: $n_{CCH}$ do
4. calculate $\varsigma_i$ based on (7) and (8);
5. end
6. find the maximum $\varsigma_i$;
7. if ($Lifetime_{CH} \leq Lifetime_i$) then
8. $CH_{next} = s_i$;
9. else ($Lifetime_{CH} &gt; Lifetime_i$)
10. $CH_{next} = CH_{current}$;
11. return $CH_{next}$
</div>

## 4.2 Adaptive Dual-Threshold Selection Algorithm Based on Dynamic Energy Consumption

Basically, the dual-threshold mechanism for nodes calculates an upper warning threshold (WT) and a lower danger threshold (DT) according to the node’s residual energy $\ b { p } _ { i }$ . When $p _ { i } < W T _ { i }$ , node $s _ { i }$ is involved into the charging queue $C _ { w c n } .$ . Subsequently, when the residual energy $\boldsymbol { p } _ { j }$ of any node $s _ { j }$ in $C _ { w c n }$ is less than $D T _ { j }$ for the first time, the MC initiates a charging operation for all nodes in $C _ { w c n }$ . With the DCHSA algorithm, any node in cluster $C _ { j } \left( 1 \leq j \leq N _ { c } \right)$ can serve as the cluster head, and thus the energy consumption rate of nodes in $C _ { j }$ dynamically change. Therefore, an expected energy consumption rate $E _ { i } ^ { e e }$ for node $s _ { i } \big ( s _ { i } \in C _ { j } \big )$ is introduced when calculating $W T _ { i }$ and $D T _ { i }$ , which involves a distance probability (DP) and a aggregation probability (AP).

Firstly, when the node close to the centroid is selected as the CH, the overall energy consumption rate within the cluster can decreases. Besides, with the design of the proposed DCHSA algorithm, the node close to the centroid is selected as the CH with priority. Therefore, the DP of the node $s _ { i }$ is designed to increase with the decrease of distance between the node $s _ { i }$ and the centroid of $C _ { j } ,$ , and is calculated as follows:

$$
D P _ {i} = \frac {\frac {1}{\varepsilon + d _ {i}}}{\sum_ {k = 1} ^ {n _ {s} ^ {j}} \frac {1}{\varepsilon + d _ {k}}}\tag{9}
$$

where ε represents a minimal positive value and is set to 1E 6 to ensure the computability, $d _ { i } \left( d _ { k } \right)$ denotes the distance from node $s _ { i } \left( s _ { k } \right)$ ) to the centroid of $C _ { j } , n _ { s } ^ { j } = \vert C _ { j } \vert , s _ { i } \in C _ { j }$ and $s _ { k } \in C _ { j }$

Based on the Eq. (7), the aggregation probability of the node $s _ { i }$ in $C _ { j }$ is calculated as follows:

$$
A P _ {i} = \frac {E _ {a g g} ^ {i}}{\sum_ {k = 1} ^ {n _ {s} ^ {j}} E _ {a g g} ^ {k}}\tag{10}
$$

Consequently, the energy consumption probability of the node $s _ { i }$ becoming the CH of $C _ { j }$ is:

$$
P _ {i, C H} = \theta * D P _ {i} + (1 - \theta) * A P _ {i}\tag{11}
$$

where θ is the weighting factor, and $\theta \in \left[ 0 , 1 \right]$

When θ is close to 1, the distance probability of the $s _ { i }$ dominates the energy consumption probability, and the nodes close to the centroid are prioritized as cluster heads. However, when θ is close to $^ { 0 , }$ the aggregation probability of the $s _ { i }$ dominates the energy consumption probability, and the nodes away from to the centroid are prioritized as cluster heads.

Finally, when node $s _ { k }$ serves as the CH of $C _ { j } { \mathrm { : } }$ , the energy consumption rate of node $s _ { i } \ ( i \neq k )$ is denoted as $E _ { k , i } ^ { e } ,$ and the expected energy consumption rate of $s _ { i }$ is $\begin{array} { r } { E _ { i } ^ { e e } = \sum _ { k = 1 } ^ { n _ { s } ^ { j } } \bigl ( P _ { k , C H } \ * \ E _ { k , i } ^ { e } \bigr ) } \end{array}$

According to Eq. (1), the node with the highest energy consumption rate in the network is the CH. Then, when the CH remains unchanged, the operational duration of the cluster is given by $\begin{array} { r } { T _ { W T } = \frac { E _ { s } } { E _ { C H } ^ { e } } } \end{array}$ . The $E _ { C H } ^ { e }$ denotes the energy consumption rate of the CH, and includes the energy consumption rate for data acceptance and the energy consumption rate for network maintenance, and $E _ { C H } ^ { e } = n _ { s } ^ { j } * E _ { r e c e } + E _ { c o n } \ ( n _ { s } ^ { j } =$ $| C _ { j } | )$

Subsequently, the preliminary WT of the node $s _ { i }$ can be computed as follows $W T _ { 0 } ^ { i } = T _ { W T } * E _ { i } ^ { e }$ . To trigger charging appropriately, the DT of node $s _ { i }$ is calculated according to the sum of the average moving delay and the average charging delay of MC. Firstly, the time to failure of node $s _ { i }$ is computed based on its current residual energy and energy consumption rate, as $\begin{array} { r } { t _ { i } = \frac { p _ { i } } { E _ { i } ^ { e } } } \end{array}$ . Then, the minimum failure time $t _ { m i n }$ of all nodes can be calculated, and the remaining energy of all nodes after $t _ { m i n }$ is ${ \boldsymbol { p } ^ { \prime } } _ { i } = p _ { i } - t _ { m i n } * E _ { i } ^ { e } , 1 \leq i \leq n$

The preliminary charging queue $C _ { w c n } ^ { 0 }$ can be form by:

$$
C _ {w c n} ^ {0} = \left\{s _ {i} \mid \forall p _ {i} ^ {\prime} \leq W T _ {0} ^ {i}, 1 \leq i \leq n \right\}\tag{12}
$$

The distance matrix $D _ { w c n } ^ { 0 }$ of $C _ { w c n } ^ { 0 }$ is as follows:

$$
D _ {w c n} ^ {0} = \left[ \begin{array}{c c c c} d _ {s _ {1} s _ {1}} & d _ {s _ {1} s _ {2}} & \dots & d _ {s _ {1} s _ {m}} \\ d _ {s _ {2} s _ {1}} & d _ {s _ {2} s _ {2}} & \dots & d _ {s _ {2} s _ {m}} \\ \vdots & \vdots & \ddots & \vdots \\ d _ {s _ {m} s _ {1}} & d _ {s _ {m} s _ {2}} & \dots & d _ {s _ {m} s _ {m}} \end{array} \right]\tag{13}
$$

where $d _ { s _ { i } s _ { j } }$ denotes the Euclidean distance between $s _ { i }$ and $s _ { j } ( s _ { i } , s _ { j } \in C _ { w c n } ^ { 0 } )$ , and m is the number ofnodes in $C _ { w c n } ^ { 0 } .$

Hence, the mean distance $d _ { m }$ of $D _ { w c n } ^ { 0 }$ is $\begin{array} { r } { d _ { m } = \frac { 1 } { m ^ { 2 } } \sum _ { i = 1 } ^ { m } \sum _ { j = 1 } ^ { m } d _ { s _ { i } s _ { j } } } \end{array}$ . The average movement time $t i m e _ { m }$ for MC to visit any node in $C _ { w c n } ^ { 0 }$ can be calculated as:

$$
\text {time} _ {m} = \frac {\left[ \frac {d _ {m}}{\nu} + \frac {2 d _ {m}}{\nu} + \cdots + \frac {m d _ {m}}{\nu} \right]}{m} = \frac {(m + 1) \sum_ {i = 1} ^ {m} \sum_ {j = 1} ^ {m} d _ {s _ {i} s _ {j}}}{2 \nu m ^ {2}}\tag{14}
$$

Based on the Eq. (2), the average charging time $\tau _ { a \nu e }$ for each node $s _ { i }$ in $C _ { w c n } ^ { 0 }$ is $\tau _ { a \nu e } =$ average $( \tau _ { s _ { 1 } } , \dots , \tau _ { s _ { i } } , \dots , \tau _ { s _ { m } } ) , s _ { i } \varepsilon C _ { w c n } ^ { 0 }$ . Subsequently, the average waiting time for MC to charge any node in $C _ { w c n } ^ { 0 }$ is calculated as:

$$
t i m e _ {c} = \frac {\left[ \tau_ {a v e} + 2 \tau_ {a v e} + \cdots + m \tau_ {a v e} \right]}{m} = \frac {(1 + m) \tau_ {a v e}}{2}\tag{15}
$$

The average of overall waiting time for MC to visit and charge any node in $C _ { w c n } ^ { 0 }$ is tim $e _ { t } = t i m e _ { c } +$ $t i m e _ { m }$ . Then, the final WT and DT are calculated based on the $E _ { i } ^ { e }$ and $E _ { i } ^ { e e }$

When $E _ { i } ^ { e } \leq E _ { i } ^ { e e }$ , the DT of node $s _ { i }$ is $D T _ { 1 } ^ { i } = t i m e _ { t } * E _ { i } ^ { e e }$ . Besides, the time $T _ { W T } ^ { i }$ for node $s _ { i }$ from WT status to exhaustion can be estimated by the sum of the lifetime of the CH (from full capacity to exhaustion) and the average of overall waiting time time , as $\begin{array} { r } { T _ { W T } ^ { i } = \frac { E _ { s } } { E _ { C H } ^ { e } } \ : + } \end{array}$ time . As a result, the WT for node $s _ { i }$ is $W T _ { 1 } ^ { i } = T _ { W T } ^ { i } * E _ { i } ^ { e e }$

Conversely, when $E _ { i } ^ { e } > E _ { i } ^ { e e }$ , the DT ofnode $s _ { i }$ is $D T _ { 1 } ^ { i } = t i m e _ { t } * E _ { i } ^ { e }$ . Similarly, the time $T _ { W T } ^ { i }$ ofnode $s _ { i }$ can by calculated by $\begin{array} { r } { T _ { W T } ^ { i } = \frac { \dot { E _ { s } } } { E _ { C H } ^ { e } } + t i m e _ { t } } \end{array}$ . The WT is obtained as $W T _ { 1 } ^ { i } = T _ { W T } ^ { i } \ * \ E _ { i } ^ { e }$

Finally, the final charging queue $C _ { w c n } ^ { 1 }$ can be determined by $C _ { w c n } ^ { 1 } = \left\{ s _ { i } \big | \forall p _ { i } \le W T _ { 1 } ^ { i } , 1 \le i \le n \right\}$

In summary, the pseudocode for the proposed ADTSA-DEC is presented in Algorithm 2 and its time complexity is ${ \mathrm { O } } ( n )$

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 2: Adaptive dual-threshold selection algorithm based on dynamic energy consumption (ADTSA-DEC)

Input: The coordinates, residual energy, topological structure of all nodes in S

Output: The WT and DT of all nodes, the final charging queue  $C_{wcn}^{1}$ 

1. calculate the  $\{E_{1}^{ee}, E_{2}^{ee}, \ldots, E_{n}^{ee}\}$  based on (9)–(11);

2. calculate the  $\{t_{1}, t_{2}, \ldots, t_{n}\}$  and obtain the  $t_{min}$ ;

3. calculate the remaining energy  $\{p_{1}^{'}, p_{2}^{'}, \ldots, p_{n}^{'}\}$ ;

4. obtain the preliminary charging queue  $C_{wcn}^{0}$  based on (12);

5. calculate the average of overall waiting time  $time_{t}$  for MC based on (13)–(15);

6. for i = 1:n do

7. if  $E_{i}^{e} \leq E_{i}^{ee}$  then

8. calculate  $DT_{1}^{i}$  and  $WT_{1}^{i}$ ;

9. else  $E_{i}^{e} &gt; E_{i}^{ee}$ 

10. calculate  $DT_{1}^{i}$  and  $WT_{1}^{i}$ ;

11. end

12. obtain the final charging queue by  $C_{wcn}^{1} = \{s_{i} | \forall p_{i} \leq WT_{1}^{i}, 1 \leq i \leq n\}$ ;

13. return  $C_{wcn}^{1}, \{WT_{1}^{1}, WT_{1}^{2}, \ldots, WT_{1}^{n}\}$  and  $\{DT_{1}^{1}, DT_{1}^{2}, \ldots, DT_{1}^{n}\}$ .
</div>

## 4.3 Charging Scheduling Strategyfor Clustered Wireless Rechargeable Sensor Networks

Based on the result of the DCHSA and ADTSA-DEC algorithms, the Particle Swarm Optimization (PSO) is employed to address the following charging scheduling for all nodes in $C _ { w c n } ^ { 1 } .$ In PSO, each particle represents a potential charging route for the MC. Then, the dimensionality $N _ { d }$ of each particle equals to the number ofnodes in $C _ { w c n } ^ { 1 } { : }$ , namely $N _ { d } = \left| C _ { w c n } ^ { 1 } \right| \leq n ,$ , and each dimension ofthe particle corresponds uniquely to a node in $C _ { w c n } ^ { 1 } ,$ and the value of that dimension is the charging ranking of that node in the entire queue. To mitigate issues such as premature convergence, local optima entrapment, and slow convergence rates, an improved PSO algorithm proposed by [38] is adopted, which incorporates an adaptive fractional-order velocity strategy and introduces perturbation based on the evolutionary state for the particle swarm. The velocity and value update functions for particles are defined as follows:

$$
\left\{ \begin{array}{l} v _ {i} ^ {k + 1} = \omega v _ {i} ^ {k} + \frac {1}{2} \omega (1 - \omega) v _ {i} ^ {k - 1} + \frac {1}{6} \omega (1 - \omega) (2 - \omega) v _ {i} ^ {k - 2} \\ + \frac {1}{2 4} \omega (1 - \omega) (2 - \omega) (3 - \omega) v _ {i} ^ {k - 3} + c _ {1} r _ {1} (x _ {i b} ^ {k} - x _ {i} ^ {k}) + c _ {2} r _ {2} (x _ {g b} ^ {k} - x _ {i} ^ {k}) \\ x _ {i} ^ {k + 1} = x _ {i} ^ {k} + v _ {i} ^ {k + 1} \end{array} \right.\tag{16}
$$

where $x _ { i } ^ { k }$ and $\nu _ { i } ^ { k }$ represent the value and velocity of the i-th particle at the k-th iteration, respectively; $x _ { i b } ^ { k }$ and $x _ { g b } ^ { k }$ denote the best-known value of the i-th particle and the global best value of the entire swarm up to the k-th iteration, respectively; $c _ { 1 }$ and $c _ { 2 }$ are the acceleration coeficients of the particle; $r _ { 1 }$ and $r _ { 2 }$ are two random values uniformly distributed in [0, 1]; ω is the velocity inertia weight.

The calculation of $c _ { 1 }$ and $c _ { 2 }$ is as follows:

$$
\left\{ \begin{array}{l} c _ {1} = \left(c _ {1 i} - c _ {1 f}\right) * \frac {N _ {p s o} - k}{N _ {p s o}} + c _ {1 f} \\ c _ {2} = \left(c _ {2 i} - c _ {2 f}\right) * \frac {N _ {p s o} - k}{N _ {p s o}} + c _ {2 f} \end{array} \right.\tag{17}
$$

where $c _ { 1 i } \ \left( c _ { 2 i } \right)$ and $c _ { 1 f } \ ( c _ { 2 f } )$ denote the initial and final values of acceleration coeficients $c _ { 1 }$ and $c _ { 2 } ,$ respectively; $N _ { p s o }$ represents the number of iterations;

The ω can be adaptively adjusted by the evolutionary state of the particle swarm, as follows:

$$
\omega = 0. 9 - \frac {1}{1 + e ^ {- e _ {f} ^ {k}}} * \frac {k}{N _ {p s o}}\tag{18}
$$

where the evolutionary factor $e _ { f } ^ { k }$ represents the evolutionary state of the particle swarm at the k-th iteration, and is calculated as follows:

$$
e _ {f} ^ {k} = \frac {d _ {g b} ^ {k} - d _ {m i n} ^ {k}}{d _ {m a x} ^ {k} - d _ {m i n} ^ {k}}\tag{19}
$$

where $d _ { m a x } ^ { k }$ and $d _ { m i n } ^ { k }$ represent the maximum and minimum values ofthe average distance between particles, respectively; $d _ { g b } ^ { k }$ denotes the mean distance of $g _ { b e s t } ;$ the average distance $d _ { i } ^ { k }$ at the k-th iteration can be calculated as:

$$
d _ {i} ^ {k} = \frac {1}{N _ {S} - 1} \sum_ {j = 1, j \neq i} ^ {N _ {S}} \sqrt {\sum_ {k = 1} ^ {N _ {d}} (x _ {i} ^ {k} - \frac {k}{j}) ^ {2}}\tag{20}
$$

where $N _ { S }$ denotes the population size of the particle swarms.

The fitness function is as follows:

$$
M i n f i t n e s s \_ v a l u e = \frac {n _ {l o s s _ {n o d e}}}{N _ {d}} + \frac {E _ {m} ^ {s}}{E _ {m} ^ {H}}\tag{21}
$$

where $n _ { l o s s \_ n o d e }$ represents the number of nodes that fail due to energy depletion during the charging schedule, $N _ { d }$ represents the number of nodes in $C _ { w c n } ^ { 1 } , E _ { m } ^ { s }$ is the mobility energy consumption of MC when it moves along a testing charging route (corresponding to a testing particle), $E _ { m } ^ { H }$ is the mobility energy consumption of MC when it moves along the minimum Hamiltonian distance between nodes $C _ { w c n } ^ { 1 }$

Finally, the proposed CWRSN-CSS is detailed in Algorithm 3. It executes the DCHSA and ADTSA-DEC algorithm with the time complexity of ${ \mathrm { O } } ( { \mathrm { n } } )$ , and calculates thefitness\_value with the time complexity of O(n), and thus estimates the optimal particle with the time complexity of $\mathrm { O } ( N _ { S } { } ^ { * } { \mathfrak { n } } )$ , and finally outputs the optimal charging route with the time complexity of $\mathrm { O } ( N _ { p s o } { } ^ { * } N _ { S } { } ^ { * } { \bf n } )$ . With the continuous operation of the network, the Target\_value formulated by $\operatorname { E q . } \left( 3 \right)$ can be updated.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 3: Charging scheduling strategy for clustered WRSN (CWRSN-CSS)

Input:  $S = \{s_{1}, s_{2}, \ldots, s_{n}\}$ ,  $C = \{C_{1}, C_{2}, \ldots, C_{N_{c}}\}$ , topology and energy parameters of a deployed WRSN

Output: Charging route P

1. execute the DCHSA and ADTSA-DEC algorithm;
2. initialize the velocity and value of all  $N_{S}$  particles;
3. for k = 1:  $N_{pso}$  do
4. for i = 1:  $N_{S}$  do
5. calculate the fitness_value for each particle based on (21);
6. end
7. update  $p_{best}$  and  $g_{best}$ , the velocity and value of each particle based on (16)–(20);
8. end
9. return P.
</div>

## 5 Experimental Procedures and Results Analysis

## 5.1 Simulation Environment and Parameter Configuration

To validate the robustness and accuracy ofthe proposed algorithm, extensive simulations are conducted. Nodes ranging from 100 to 500 are randomly distributed in ψ (1000 m  1000 m). Key parameters employed in the simulations are detailed in Table 3. The $\varepsilon _ { l } , \varepsilon _ { c } , \varepsilon _ { m } = 1$ is used to equalize the weights of mobile energy consumption, charging energy consumption and node failure in optimization. The energy consumption parameters $( n _ { p r e c e } , e _ { p r e c e } , \alpha , \gamma , \beta _ { 1 } , \beta _ { 2 } , \lambda )$ and the charging model are based on the latest references [7,22]. To mitigate the impact of stochastic variations, each parameter configuration is simulated for 15 independent runs, and the average result is used. All simulations are executed on a computer with an AMD R9 7940H CPU, using Python version 3.10.11.

Table 3: Simulation parameters.

<table><tr><td>Parameter</td><td>Value</td><td>Parameter</td><td>Parameter</td></tr><tr><td> $\nu$ </td><td>5 m/s</td><td> $L_c$ </td><td>100 m</td></tr><tr><td> $E_s$ </td><td>1000 J</td><td>R</td><td>50 m</td></tr><tr><td> $n_{\text{prece}}$ </td><td>100 bit/s</td><td> $E_{\text{ch arg er}}$ </td><td>5 J/s</td></tr><tr><td> $e_{\text{prece}}$ </td><td>1E-6 J/bit</td><td> $\beta_1$ </td><td>6E-6 J/s</td></tr><tr><td> $\alpha$ </td><td>2</td><td> $\beta_2$ </td><td>1E-9 J/bit/m2</td></tr><tr><td> $\gamma$ </td><td>1E-9 J/bit</td><td> $\lambda$ </td><td>0.8</td></tr><tr><td> $E_{\text{con}}$ </td><td>5E-4 J/s</td><td> $e_{CH-rot}$ </td><td>2E-4 J/s</td></tr></table>

## 5.2 Numerical Simulation Analysis ofKey Parameters

The impact of n (problem scale), $N _ { c }$ (number of clusters), θ (weighting factor for distance and energy consumption in Cluster Head selection), and T (time scale) on the robustness and accuracy of the proposed algorithm is firstly analyzed. Specifically, $N _ { c } \in [ 2 , 2 0 ] , \theta \in [ 0 , 1 ] , T \in [ 1 \mathrm { E } 6 , 5 \mathrm { E } 6 ]$

In Fig. 2, the calculated target energy eficiency values exhibit an increasing trend with the growth of n under virtually all simulated conditions. This progression demonstrates an approximately linear characteristic, thereby permitting the estimation of energy eficiency through a nearly linear function of n. Fig. 2a presents the Target\_value of the CWRSN-CSS under the conditions where $n = 1 0 0 { - } 3 0 0 , N _ { c } =$ $2 - 2 0 , \theta = 0 . 6$ , and $T = 1 { \mathrm { E } } 6$ . The results indicate that, with diferent $n ,$ the Target\_value initially decreases and subsequently increases as $N _ { c }$ rises. This is because when the $N _ { c }$ is small $( \mathrm { i . e . , }$ the number of intracluster nodes is large), the energy consumption ofCH nodes increases, leading to a high frequency ofmobile MC charging scheduling and consequently a high movement energy consumption, which in turn raises the Target\_value. Correspondingly, when the $N _ { c }$ is large (i.e., the number of intra-cluster nodes is small), the MC charging frequency decreases. However, the number of nodes requiring charging per scheduling cycle increases, similarly amplifying movement energy consumption and thereby increasing the Target\_value. Therefore, during the network deployment phase, iterative methods such as the trial-and-error approach can be employed to progressively determine the optimal $N _ { c }$

Fig. 2b illustrates the impact of the network operation time on algorithmic accuracy under the conditions of $n = 1 0 0 { - } 3 0 0 , \theta = 0 . 6 ,$ , the number of intra-cluster nodes is 25 $( N _ { c } = 4 , 8 , 1 2$ , respectively), and $T =$ $1 \mathrm { E } 6 { - } 5 \mathrm { E } 6 s .$ . The results indicate that as the network operation time increases, the Target\_value exhibits growth and gradually stabilizes. This is because, at the initial stage ofnetwork operation, all nodes possess full energy levels, resulting in a relatively low node failure rate. Subsequently, as the network continues to operate, the energy consumption for charging and mobility, and the node failure rate gradually stabilize, resulting in the convergence of the Target\_value to a steady state.

![](images/ee2d2d597b007512d225e8d45dedab77b73119f7850c7af8e0915b0e5e9eeeed.jpg)  
(a)

![](images/84cdb58271e3601f5d49251813fa9581f98e6b2dcaeb444373d1cfdc5f9192ca.jpg)  
(b)

![](images/74b4f0463c3d9e5033f719f31888a22bed47d8ba34fbd0d9b55ea6e214f66b5c.jpg)  
(c)  
Figure 2: Target\_value of key parameters: (a) Target\_value of $N _ { c } ; ( { \mathbf { b } } )$ Target\_value of $T ;$ (c) Target\_value of θ.

Fig. 2c presents the Target\_value of the CWRSN-CSS under the conditions where $n = 1 0 0 { - } 3 0 0 .$ , the number of intra-cluster nodes is $2 5 ( N _ { c } = 4 , 8 , 1 2$ , respectively), $\theta = 0 { - } 1$ , and $T = 1 \mathrm { { E } } 6$ . It can be observed that the algorithm achieves optimal results when θ approaches 0.6. According to Eq. (11), this indicates that within the weighting scheme balancing distance factors and energy consumption costs for cluster head selection, the distance factor plays a more critical role, and thus contributes more significantly to achieving superior network energy eficiency. Based on this analysis, the parameter $\theta$ is set to 0.6 in subsequent simulations. In summary, the proposed algorithms have robust solution quality with increasing network scale and tempora dimensions, and the optimization strategies for the $N _ { c }$ and $\theta$ are empirically validated.

Furthermore, Fig. 3 represents the impact of the network scale and clusters on the separate network energy eficiency and node survivability at $T = 3 \mathrm { { E } } { + } 6$ . Basically, when $\varepsilon _ { l } , \varepsilon _ { c } , \varepsilon _ { m } = 1$ , The proportion of node failure rate is the smallest, while the proportion of charging energy consumption rate is the largest. Therefore, subsequent research can adjust the weight coeficients according to the optimization objectives to highlight diferent scheduling optimization strategies. Besides, in Fig. 3a, the node failure rate increases with the growth of network scale n when the number of clusters is fixed, and it decreases with the growth of number of clusters when the n is fixed. However, in Fig. 3b, the charging energy consumption rate decreases with the growth of network scale n when the number of clusters is fixed, and it also decreases with the growth of number of clusters when the n is fixed. This indicates that the sensitivity of charging energy consumption and node failure rate to key network parameters is mutually exclusive. Finally, in Fig. 3c, the mobile energy consumption rate increases with the growth of network scale n when the number of clusters is fixed, and it also increases with the growth of number of clusters when the n is fixed. This indicates that the sensitivity of charging energy consumption and mobile energy consumption rate to key network parameters is completely opposite. These two energy consumption targets cannot be simultaneously optimized, and can only be achieved through a trade-of.

(nb)

(c)  
![](images/bbd37d84773a2a4e10da2f1dddc1c05f1e3d01ddd4ec9b0a4ca9c17c62815480.jpg)

![](images/6ddd211ab12fb0bcd8e4862012979fd9b18faa6287f6140fccc82c3c65b78bc3.jpg)

![](images/8407d80b5aa7842463678b42febc91d81d5e9ad01f891a94cd35ed21222a2784.jpg)  
Figure 3: The separate network energy eficiency and node survivability under diferent network scale and clusters: (a) Failure rate of nodes; (b) Charging energy comsumption rate; (c) Mobile energy comsumption rate.

## 5.3 Results and Analysis ofDCHSA

To evaluate the impact of DCHSA on the precision of the CWRSN-CSS, this work conducts a comparative analysis with two state-of-the-art cluster head selection algorithms: NCHR [28] and ORS [29]. The comparison is estimated by the metrics of Target\_value, number of failure nodes, and energy consumption.

Fig. 4 illustrates the variation of Target\_value over $T = 1 \mathrm { E } 6  – 5 \mathrm { E } 6$ and $n = 1 0 0 { - } 5 0 0$ when employing the newly proposed DCHSA algorithm vs. two comparative algorithms, ORS and NCHR. Overall, the charging scheduling strategy utilizing the proposed DCHSA algorithm consistently achieves optimal energy eficiency gains across diferent network scales as operation time progresses. Correspondingly, the charging scheduling strategies employing the two comparative algorithms, ORS and NCHR, exhibit fluctuations during the initial phase of network operation. However, the CWRSN-CSS charging scheduling strategy based on the DCHSA algorithm consistently maintains stable and optimal target values. As the n increases, the node density within the network rises, which leads to a marginal increase in the Target\_value across all three algorithms. Nevertheless, the Target\_value for each algorithm demonstrates minimal fluctuation. This phenomenon can be attributed to the increased number of nodes, which results in a slight elevation of the number of node failures. Specifically, in Fig. 4a–c, the Target\_value of the CWRSN-CSS algorithm based on DCHSA consistently remains lower than that of the charging scheduling algorithms using ORS and NCHR. The average reductions are 17.74%, 5.07%, 5.39%, 5.97%, and 5.52% ((4.09%, 1.68%, 0.14%, 0.48%, 0.44%) and (2.76%, 3.65%, 2.36%, 2.73%, 3.16%)) for the first comparison set, 4.64%, 4.58%, 3.10%, 1.00%, and 1.51% ((1.13%, 0.25%, 0.95%, 0.76%, 0.96%) and (2.14%, 2.08%, 1.71%, 1.21%, 1.16%)) for the second comparison set.

![](images/2cee97c5d3bce1fb41e084cda1243c2ea19f808e80e4a44640a9d6fc9a08505b.jpg)

![](images/55c07f10089052f0ea8e46261f651213a475a0679a880823a890d539bd16f6f2.jpg)

![](images/c43da81f51086000a15fedfc13854d5bdb5f64b4252b2f64903ab6bf40746b9e.jpg)  
Figure 4: Target\_value of the three CH selection algorithms: (a) $n = 1 0 0 ;$ (b) $n = 3 0 0 ;$ (c) $n = 5 0 0 .$

Furthermore, Fig. 5 presents a comparative analysis of the number of failed nodes among these three algorithms under network scales ranging from $n = 1 0 0 { - } 5 0 0$ . Two distinct observations can be drawn: (a) across varying network sizes, as operational time accumulates, the proposed DCHSA algorithm and the existing ORS algorithm achieve fewer node failures compared to the NCHR algorithm. Although the proposed DCHSA algorithm exhibits a marginal increase in failed nodes relative to the ORS algorithm, the diference is not substantial. (b) Comparative analysis of the three subfigures reveals that while the absolute number of node failures increases with larger network scales n, the node failure ratio remains stable. This indicates that the DCHSA-based charging scheduling strategy CWRSN-CSS exhibits robust performance in maintaining consistent node failure rates.

![](images/90522d2e565cd9655beddde7bbd346bc6fd376a5a459349203e312eb5182c220.jpg)

![](images/585d89167030634d4eff8d94c992e859db50add39137e13631ce0fc77fb7c316.jpg)

![](images/411c5330010581b3612a854da8ad71d16d82cff326288cc3032c4b90852d6cca.jpg)  
Figure 5: The number of failed nodes of the three CH selection algorithms: (a) $n = 1 0 0 ;$ (b) $n = 3 0 0 ;$ (c) $n = 5 0 0 .$

Finally, Fig. 6 presents a comparative analysis of energy consumption among these three algorithms under node scales ranging from $n = 1 0 0 { - } 5 0 0$ . The figure demonstrates that: (a) Both the proposed DCHSA and NCHR algorithm achieve lower energy consumption per unit time compared to that of the ORS algorithm. Although the proposed DCHSA exhibits a marginal increase in energy consumption per unit time relative to NCHR, the diference is negligible; (b) Across all simulations, energy consumption per unit time tend to be stable when increasing node scales, which suggests strong robustness of the proposed algorithm.

![](images/3431f4129b1e775e7211490612d1cf1c25e7fe38e30bbd3833cef6588f593bc7.jpg)

![](images/66c53dc4a6fe3d22ee11d8b730567b6f4707cd743e19bca757101c9bcca37ce1.jpg)

![](images/18a964df7462c519930b5391b3fe7f883011ba55de258908cfe5ebe8b679c455.jpg)  
Figure 6: Energy consumption time ratio of the three CH selection algorithms: (a) n 100; (b) $n = 3 0 0 ;$ (c) $n = 5 0 0 .$

In summary, although the DCHSA algorithm exhibits a marginally higher node failure count compared to the ORS algorithm and a marginally higher network energy consumption per unit time compared to the NCHR algorithm, it achieves superior objective values compared to the benchmark algorithms ORS and NCHR, and thus it has overall balanced performance.

## 5.4 Results and Analysis ofADTSA-DEC

To evaluate the performance of the proposed ADTSA-DEC algorithm, a comparative analysis is conducted against two advanced algorithms: the fixed dual-threshold method in the charging threshold direction [39] and the ADT-OSA algorithm [33]. For the fixed dual-threshold method, five distinct sets of fixed WT and DT are employed to ensure a comprehensive and generalizable comparison. In the simulation, the Target\_value and the number of failed nodes of the algorithms are compared.

Fig. 7 presents the Target\_value ofcharging scheduling strategies based on the newly proposed ADTSA-DEC algorithm, the existing ADT-OSA algorithm, and five fixed-threshold algorithms under conditions of $n = 1 0 0 { - } 5 0 0$ and $T = 1 \mathrm { E } 6 - 5 \mathrm { E } 6$ . The following conclusions can be drawn: (a) As the node scale increases, the CWRSN-CSS algorithm based on ADTSA-DEC consistently achieves the optimal target value, with average reductions of 0.77%, 10.67%, 8.99%, 4.62%, 6.35%, and 6.10% compared to the target values of the ADT OSA-based algorithm and the five fixed-threshold algorithms, respectively. (b) With the growth ofboth node scale and network operation time, the target value of the CWRSN-CSS algorithm increases, and the growth is slight. (c) Although the fixed-threshold-based charging scheduling strategies can improve target values through threshold adjustment, the results is still worse than the target value obtained by the ADTSA-DEC based CWRSN-CSS strategy. This indicates that the fixed-threshold method cannot adapt to maximize the energy eficiency of networks with time-varying energy consumption.

![](images/2a5856c02d26f9abd1b815b64e23e7d42dc4ab981db0846a148c3b2951b04820.jpg)  
Ta)

![](images/48c3f869604383a58ddeb1eabcf8208ef839c15007c448ad401720f9c82bf39d.jpg)  
(b)

![](images/4c064bd0f5607a7a26cc72265a641716610fe879132678f7e73abe48501da31b.jpg)  
Figure 7: Target\_value of the three threshold algorithms: (a) n 100; (b) $n = 3 0 0 ;$ (c) n 500.

Fig. 8 presents the number of node failures under the charging scheduling strategies based on the ADTSA-DEC algorithm and the six comparative algorithms with $n = 1 0 0 { - } 5 0 0$ and $T = 1 \mathrm { E } 6  – 5 \mathrm { E } 6 .$ . The following conclusions can be drawn: (a) As the number of nodes and the operational duration increase, the ADTSA-DEC-based CWRSN-CSS charging scheduling strategy consistently outperforms the other algorithms in terms of failure nodes, with average reductions of 17.18%, 324.18%, 246.21%, 111.75%, 165.59%, and 175.59%, respectively. (b) The number of node failures under the ADTSA-DEC-based CWRSN-CSS strategy increases with prolonged network operation, but the growth trend remains gradual, thereby its robustness over time can be validated. (c) As both network scale and operational time increase, the optimal number of node failures of the fixed-threshold-based charging strategies are achieved under diferent fixed thresholds. This further indicates that the fixed-threshold-based strategy is not suitable for maximizing energy eficiency in networks with time-varying energy consumption, which verifies the superiority of ADTSA-DEC. (d) Compared to the ADT-OSA, the newly proposed ADTSA-DEC algorithm has superior performance in reducing the node failure rate. This is attributed to the fact that the ADT-OSA algorithm does not account for abrupt changes in node energy consumption caused by CH rotation. Ultimately, it can be concluded that the ADTSA-DEC algorithm demonstrates superior performance in addressing energy consumption variations among nodes caused by CH changes.

(b)  
![](images/fd216a999c0184ac40411d38ec5e2e08aba4734fe459bfa046b63496f47f266c.jpg)  
(a)

![](images/257c0596466199bba43bbfe92ff7e6fcd3b418bb849b241f7a4adf4742e1a044.jpg)  
(b)

![](images/fa7e97581b74b157dd7fb0fbebe8bc9a7e059889630479b0fdda83c842c9ae36.jpg)  
(c)  
Figure 8: The number of failed nodes of the three threshold algorithms: (a) n  100; (b) $n = 3 0 0 ;$ (c) $n = 5 0 0$

## 5.5 Supplementary Comparative Test

To further validate the efectiveness of the scheduling strategy based on PSO algorithm, in Fig. 9a, the comparison results of four representative heuristic MC scheduling algorithms (IR-IGA [33], IABC [39], EUP-ACS [40], MOCFA [41]), and the improved PSO algorithm are presented at diferent node scales and $\mathrm { T } = 3 \mathrm { E } { + } 6 $ . Basically, these algorithms share the charging queue and charging trigger time by the DCHSA and ADTSA-DEC. It can be seen that the proposed CWRSN-CSS can almost always achieve the minimal Target\_value, and its slight decline appears in small-scale networks.

![](images/afdd75db2bfd598c31ab930239f55ec0e04b0aaed6589c316075b78481bea195.jpg)

![](images/eed6752274ab9e7db69653b39d3a2bad6d7fd7f36a32c4f4da5921100135c832.jpg)

![](images/552f30db6dbb319c40401f371d12b75108028a7cd6ae3225a880c0ad783ed519.jpg)  
(c)  
Figure 9: Supplementary comparative tests: (a) Comparison of scheduling algorithms; (b) CH selection algorithm under emergency events; (c) Charging threshold algorithm under emergency events

Besides, in real network, nodes may generates additional data collection to encounter the occurrence of emergency events, which can be described as the following probability process: when a node detects an emergency event $X ,$ it triggers emergency sampling, and the node can trigger emergency sampling with a duration of $t _ { e g }$ and a sensing rate of $b _ { e g } ~ * ~ n _ { \mathit { p e r c e } } \ : ( b _ { e g } > )$ 1); assuming that emergency events are independent in time and space, their occurrence probability follows a Poisson distribution; specifically, during the network operation time T, the occurrence ofemergency event X at a certain node follows a Poisson distribution, with a probability P following the parameter $\rho , X \sim \pi \left( \rho \right)$ , and the probability density function is calculated as: $\begin{array} { r } { P \left( X \left( T \right) = k \right) = \frac { \left( \rho T \right) ^ { k } } { k ! } e ^ { - \rho T } } \end{array}$ , where $\rho \big ( \rho = 2 0 - 1 0 0 \big )$ is the parameter of the Poisson distribution. Therefore, for node $s _ { i }$ in the network, the total number of data packets sensed within T time is calculated as: $\eta _ { i } =$ $T \ * \ n _ { p e r c e } + X _ { i } ( T ) \ * \ t _ { e g } \ * \ b _ { e g } \ * \ n _ { p e r c e } ,$ where $X _ { i } ( T )$ represents the number of emergency events $s _ { i }$ occurring at the node within T time.

Fig. 9 also illustrates the comparison between the proposed DCHSA and ADTSA-DEC algorithms in the presence of emergency events. Fig. 9b represents the results of DCHSA and two comparative algorithms

(b)

in the presence of emergency events, with $n = 3 0 0$ and $T = 1 \mathrm { E } { + } 6 { - } 5 \mathrm { E } { + } 6 .$ . It can be seen that even under sudden sampling, the results of DCHSA are still superior to the comparative algorithms. Fig. 9c represents the comparative results of ADTSA-DEC, ADT-OSA and five fixed trigger threshold methods in the presence of emergency events, with $n = 3 0 0$ and $T = 1 \mathrm { E } { + } 6 { - } 5 \mathrm { E } { + } 6$ . It can be seen that the proposed ADTSA-DEC algorithm maintains superior results during the continuous operation of the network.

To simulate the impact of network distribution diversity on the newly proposed method in reality, in Fig. 10, the Target\_value is presented under diferent network distributed scenarios with $T = 3 \mathrm { { E } } { + } 6$ and $n = 1 0 0 { - } 5 0 0 .$ . Fig. 10a illustrates the scenario of central clustering, where most nodes are clustered at the center of ψ. Compared to the uniform clustering scenario in Fig. 10b, since most nodes within the cluster are close to the center of $\psi ,$ the uneven energy consumption caused by frequent cluster head rotation, hotspot efect, and bottleneck efect increases, which leads to the increase of charging energy consumption rate, and thus increases the Target\_value. Fig. 10c shows the scenario where most nodes are dispersed at the edge of ψ. Compared to the uniform distribution in Fig. 10b, the increase in the proportion of MC mobile energy consumption and the resulting increase in node failure rate led to an overall increase in the Target\_value. As a result, the Target\_value based on the proposed DCHSA can always be optimal, and thus it is robust to network distribution diversity.

![](images/c8e203aec33f9e6c61935849a92af0c4f9718f38191a58311aa29206be19036f.jpg)

![](images/a70f9fe353fb527f379aea7a569045025d9b38f8f54eef66683b548713d9af76.jpg)

![](images/a88c2d2955693fb9b4e9f4a2dc0f8fa23561946c3b9ddae59d85df0bdc9e7257.jpg)  
Figure 10: Multi network distributed scenarios: (a) Central clustering; (b) Uniform clustering; (c) Terrain-constrained clustering.

## 5.6 Numerical Results in Real-Life Scenario

In real-life scenarios, charging scheduling may encounter significant challenges such as path blockage for MC. To further validate the accuracy and robustness ofthe proposed algorithm in practical environments, Shanghai Wildlife Park is selected as the experimental site, as illustrated in Fig. 11. This scenario serves as a typical application of WRSN in environmental monitoring, where sensors can be deployed anywhere except lakes and wooded areas, while the movement paths of MC are constrained by obstacles such as lakes and trees. In this work, the charging scheduling strategy based on CWRSN-CSS with three distinct CH selection algorithms is first employed to determine the starting, intermediate, and ending points of the path. Subsequently, the two-dimensional environment is modeled as a grid map (with a grid size of 1 m 1 m). The A\* algorithm is applied for segment-by-segment path planning, and the sub-paths are concatenated to obtain the complete charging route.

As shown in Table 4, the results of the three charging scheduling strategies indicate that the objective values of all methods have increased compared to that in the ideal simulation scenario. This is attributed to path obstruction issues in real world, which lead to a slight rise in the mobile energy consumption of MC and the node failure rate. Nevertheless, the proposed algorithm consistently maintains the optimal objective value. Thus, the performance and robustness ofthe proposed algorithm are validated under real-life scenario.

![](images/316dcf50b279c0287c216ba953fce413757d1f6f228ce969b81e6fd9f0e4487a.jpg)  
Figure 11: Shanghai wild animal park.

Table 4: Target\_value of three charging scheduling strategies in real-life scenario simulation.

<table><tr><td rowspan="2">Network Scale</td><td colspan="3">CH Selection Algorithm + Charging Threshold Algorithm</td></tr><tr><td>DCHSA + ADTSA-DEC</td><td>ORS + ADTSA-DEC</td><td>NCHR + ADTSA-DEC</td></tr><tr><td>n = 100</td><td>1.178818851</td><td>1.3654849</td><td>1.227797966</td></tr><tr><td>n = 200</td><td>1.186901041</td><td>1.229411834</td><td>1.220782393</td></tr><tr><td>n = 300</td><td>1.216099547</td><td>1.242163296</td><td>1.235931561</td></tr></table>

## 6 Conclusion

This work investigates the charging scheduling problem of mobile chargers in clustered wireless rechargeable sensor networks by proposing a dynamic cluster head selection algorithm (DCHSA) and an adaptive dual-threshold selection algorithm (ADTSA-DEC). These algorithms jointly consider network energy consumption and reliability constraints. In DCHSA, cluster head (CH) nodes dynamically determine the candidate CH set by computing the real-time variance of residual energy within clusters. The optimal CH is then selected based on multiple metrics, including residual energy and the energy consumption associated with data aggregation, network maintenance, and CH rotation. In ADTSA-DEC, adaptive threshold calculation is introduced while accounting for dynamic variations in node energy consumption caused by CH rotation. Extensive simulation results demonstrate that the proposed algorithms significantly outperform existing approaches in maintaining network reliability while achieving low energy consumption. Furthermore, the robustness and scalability of the proposed methods are validated through comprehensive analyses of key parameters, including the problem scale, number of clusters, weighting factors for distance and energy consumption in CH selection, and time scale, as well as simulations under real-world scenarios.

Future work will focus on addressing challenges arising in more complex scenarios, such as dense deployment, low-latency access, and multi-charger collaborative scheduling. To tackle these challenges, heuristic strategies integrated with machine learning and neural networks will be further explored to improve iterative capability and adaptability in complex environments.

Acknowledgement: Not applicable.

Funding Statement: The author received no specific funding for this study.

Author Contributions: The authors confirm contribution to the paper as follows: study conception, design and draft manuscript preparation: Mengqi Liu; writing—reviewing & editing: Haiqing Yao. All authors reviewed and approved the final version of the manuscript.

Availability of Data and Materials: The data that support the findings of this study are available from the author, Haiqing Yao, upon reasonable request.

Ethics Approval: Not applicable.

Conflicts of Interest: The author declares no conflicts of interest.

## Nomenclature

WRSN Wireless Rechargeble Sensor Networks

WSN Wireless Sensor Network

MC Mobile Chargers

## References

1. Zhang C, Zhang L, Gan H, Chen H, Li Z. An energy optimization algorithm for WRSN nodes based on regional partitioning and inter-layer routing. Comput Mater Contin. 2025;84(2):3125–48. doi:10.32604/cmc.2025.064499.

2. Chen J, Yi C, Wang R, Zhu K, Cai J. Learning aided joint sensor activation and mobile charging vehicle scheduling for energy-eficient WRSN-based industrial IoT. IEEE Trans Veh Technol. 2023;72(4):5064–78. doi:10.1109/TVT. 2022.3224443.

3. Das R, Dash D. Joint on-demand data gathering and recharging by multiple mobile vehicles in delay sensitive WRSN using variable length GA. Comput Commun. 2023;204(3):130–46. doi:10.1016/j.comcom.2023.03.022.

4. Ahmadi H, Bouallegue R. Exploiting machine learning strategies and RSSI for localization in wireless sensor networks: a survey. In: Proceedings of the 2017 13th International Wireless Communications and Mobile Computing Conference (IWCMC); 2017 Jun 26–30; Valencia, Spain. New York, NY, USA: IEEE; 2017. p. 1150–4. doi:10.1109/ IWCMC.2017.7986447.

5. Gao Z, Zhang Q, Gao Q, Zhao Y, Wu HC. Directional WPT charging for routing-asymmetric WRSNs with a mobile charger. IEEE Trans Veh Technol. 2026;75(1):1370–85. doi:10.1109/tvt.2025.3594160.

6. Ma X, Liu X, Ansari N, Ding J. Enhancing WRSN sustainability through on-demand directional wireless charging with multiple green-powered mobile vehicles. IEEE Internet Things J. 2025;12(11):18019–30. doi:10.1109/JIOT.2025. 3540170.

7. Chen J, Li X, Ding Y, Cai B, He J, Zhao M. Charging eficiency optimization based on swarm reinforcement learning under dynamic energy consumption for WRSN. IEEE Sens J. 2024;24(20):33427–41. doi:10.1109/JSEN. 2024.3407748

8. Ri MG, Kim IG, Pak SH, Jong NJ, Kim SJ. An integrated MCDM-based charging scheduling in a WRSN with multiple MCs. Peer Peer Netw Appl. 2024;17(5):3286–303. doi:10.1007/s12083-024-01705-y.

9. Jiang C, Chen W, Chen X, Zhang S, Xiao W. Deep reinforcement learning approach with hybrid action space for mobile charging in wireless rechargeable sensor networks. Expert Syst Appl. 2024;249(4):123752. doi:10.1016/j. eswa.2024.123752.

10. Neamatollahi P, Naghibzadeh M, Abrishami S, Yaghmaee MH. Distributed clustering-task scheduling for wireless sensor networks using dynamic hyper round policy. IEEE Trans Mob Comput. 2018;17(2):334–47. doi:10.1109 TMC.2017.2710050.

11. Hada RPS, Srivastava A. Dynamic cluster head selection in WSN. ACM Trans Embed Comput Syst. 2024;23(4):1–27. doi:10.1145/3665867.

12. Gupta D, Ramesh JVN, Kumar MK, Alghayadh FY, Dodda SB, Ahanger TA, et al. Optimizing cluster head selection for E-commerce-enabled wireless sensor networks. IEEE Trans Consumer Electron. 2024;70(1):1640–7. doi:10. 1109/tce.2024.3360513.

13. Heinzelman WR, Chandrakasan A, Balakrishnan H. Energy-eficient communication protocol for wireless microsensor networks. In: Proceedings of the 33rd Annual Hawaii International Conference on System Sciences; 2000 Jan 7: Maui, HI, USA. New York, NY, USA: IEEE: 2000, p. 926982, doi:10.1109/HICSS,2000.926982

14. Wedaj FT, Hawbani A, Wang X, Qaisar MUF, Othman W, Alsamhi SH, et al. RECO: on-demand recharging and data collection for wireless rechargeable sensor networks. IEEE Trans Green Commun Netw. 2023;7(4):1863–76. doi:10.1109/tgcn.2023.3305026.

15. Han G, Yang X, Liu L, Zhang W. A joint energy replenishment and data collection algorithm in wireless rechargeable sensor networks. IEEE Internet Things J. 2018;5(4):2596–604. doi:10.1109/JIOT.2017.2784478.

16. Han G, Wu J, Wang H, Guizani M, Ansere JA, Zhang W. A multicharger cooperative energy provision algorithm based on density clustering in the industrial Internet of Things. IEEE Internet Things J. 2019;6(5):9165–74. doi:10. 1109/JIOT.2019.2928557.

17. Prakash V, Singh D, Pandey S, Singh S, Singh PK. Energy-optimization route and cluster head selection using M-PSO and GA in wireless sensor networks. Wirel Pers Commun. 2024;107(2):1–26. doi:10.1007/s11277-024-11 096-1.

18. Qamar MS, Munir MF. A hybrid framework integrating deterministic clustering, neural networks, and energyaware routing for enhanced eficiency and longevity in wireless sensor network. Comput Mater Contin. 2025;84(3):5463–85. doi:10.32604/cmc.2025.064442.

19. Liu K, Peng J, He L, Pan J, Li S, Ling M, et al. An active mobile charging and data collection scheme for clustered sensor networks. IEEE Trans Veh Technol. 2019;68(5):5100–13. doi:10.1109/tvt.2019.2906234.

20. Fanian F, Kuchaki Rafsanjani M. CFMCRS: calibration fuzzy-metaheuristic clustering routing scheme simultaneous in on-demand WRSNs for sustainable smart city. Expert Syst Appl. 2023;211(14):118619. doi:10.1016/j.eswa. 2022.118619.

21. Amuthan A, Arulmurugan A. Semi-Markov inspired hybrid trust prediction scheme for prolonging lifetime through reliable cluster head selection in WSNs. J King Saud Univ Comput Inf Sci. 2021;33(8):936–46. doi:10.1016/ j.jksuci.2018.07.006.

22. Chaurasia S, Kumar K, Kumar N. MOCRAW: a meta-heuristic optimized cluster head selection based routing algorithm for WSNs. Ad Hoc Netw. 2023;141(4):103079. doi:10.1016/j.adhoc.2022.103079.

23. Li J, Sun G, Wang A, Lei M, Liang S, Kang H, et al. A many-objective optimization charging scheme for wireless rechargeable sensor networks via mobile charging vehicles. Comput Netw. 2022;215(6):109196. doi:10.1016/ j.comnet.2022.109196.

24. Qamar MS, Haq IU, Daraz A, Alamri AM, AlQahtani SA, Munir MF. A novel approach to energy optimization: eficient path selection in wireless sensor networks with hybrid ANN. Comput Mater Contin. 2024;79(2):2945–70. doi:10.32604/cmc.2024.050168.

25. Ismail AS, Hawbani A, Wang X, Abdel Aziz S, Alsamhi SH, Zhao L, et al. RBEER: rule-based energy-eficient routing protocol for large-scale UWSNs. IEEE Trans Green Commun Netw. 2024;8(3):1168–81. doi:10.1109/tgcn. 2024.3364776.

26. Ri MG, Kim CH. An uneven cluster-based routing protocol for WSNs using an integrated MCDM and EPO. In: Proceedings of the 2024 9th International Conference on Computer and Communication Systems (ICCCS); 2024 Apr 19–22: Xian, China. New York, NY, USA: IJEEE: 2024, p. 678–83, doi:10.21203/rs,3,rs-5280783/vl.

27. Dong Y, Li S, Bao G, Wang C. An eficient combined charging strategy for large-scale wireless rechargeable sensor networks. IEEE Sens J. 2020;20(17):10306–15. doi:10.1109/JSEN.2020.2990641.

28. Choudhury N, Matam R, Mukherjee M, Lloret J, Kalaimannan E. NCHR: a nonthreshold-based cluster-head rotation scheme for IEEE 802.15.4 cluster-tree networks. IEEE Internet Things J. 2021;8(1):168–78. doi:10.1109/JIOT. 2020.3003320

29. Mostarda L, Navarra A, De Leone R. Optimal vs. rotation heuristics in the role of cluster-head for routing in IoT constrained devices. Internet Things. 2023;22(23):100757. doi:10.1016/j.iot.2023.100757.

30. Lee C, Na W, Jang G, Lee C, Cho S. Energy-eficient and delay-minimizing charging method with a multiple directional mobile charger. IEEE Internet Things J. 2021;8(10):8291–303. doi:10.1109/JIOT.2020.3044684.

31. Shang C, Chang CY, Liao WH, Roy DS. RLR: joint reinforcement learning and attraction reward for mobile charger in wireless rechargeable sensor networks. IEEE Internet Things J. 2023;10(18):16107–20. doi:10.1109/JIOT. 2023.3267242.

32. Jiang C, Chen W, Wang Z, Xiao W. Deep reinforcement learning-based joint sequence scheduling and trajectory planning in wireless rechargeable sensor networks. IEEE Sens J. 2024;24(8):13699–711. doi:10.1109/JSEN.2024. 3373664.

33. Yao H, Xiao C, Yang Y, Postolache O. Directional mobile charger scheduling strategy based on adaptive dual threshold. IEEE Sens J. 2024;24(11):18467–78. doi:10.1109/jsen.2024.3387445.

34. Bian X, Sha C, Malekian R, Zhao C, Wang R. Balanced distribution strategy for the number of recharging requests based on dynamic dual thresholds in WRSNs. IEEE Internet Things J. 2024;11(19):30551–70. doi:10.1109/JIOT.2024. 3413079.

35. Kumar R, Mukherjee JC. On-demand vehicle-assisted charging in wireless rechargeable sensor networks. Ad Hoc Netw. 2021;112(5834):102389. doi:10.1016/j.adhoc.2020.102389.

36. Makanda K, Hawbani A, Wang X, Naji A, Al-Dubai A, Zhao L, et al. Adaptive mobile chargers scheduling scheme based on AHP-MCDM for WRSN. IEEE Trans Sustain Comput. 2025;10(1):57–69. doi:10.1109/tsusc.2024.3391316.

37. Chakeres ID, Belding-Royer EM. AODV routing protocol implementation design. In: Proceedings of the 24th International Conference on Distributed Computing Systems Workshops; 2004 Mar 23–24; Tokyo, Japan. New York, NY, USA: IEEE; 2004. p. 698–703. doi:10.1109/ICDCSW.2004.1284108.

38. Song B, Wang Z, Zou L. An improved PSO algorithm for smooth path planning of mobile robots using continuous high-degree Bezier curve. Appl Soft Comput. 2021;100(1):106960. doi:10.1016/j.asoc.2020.106960.

39. Zhao C, Zhu P, Zhang N, Chen S, Shao X, Wang Y. Directional charging-based scheduling strategy for multiple mobile chargers in wireless rechargeable sensor networks. Ad Hoc Netw. 2023;149(3):103251. doi:10.1016/j.adhoc. 2023.103251.

40. Lei Y, Yu J, Cao D, Feng Z, Chan S. An energy urgency priority based mobile charging scheme in Wireless Rechargeable Sensor Network. Ad Hoc Netw. 2023;140(6):103067. doi:10.1016/j.adhoc.2022.103067.

41. Wang X, Lyu Z, Wei Z, Wang L, Lu Y, Shi L. Multi-objective path planning algorithm for mobile charger in wireless rechargeable sensor networks. Wirel Netw. 2023;29(1):267–83. doi:10.1007/s11276-022-03126-2.