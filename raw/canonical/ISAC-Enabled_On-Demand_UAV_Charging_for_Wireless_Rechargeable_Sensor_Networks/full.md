---
title: "ISAC-Enabled On-Demand UAV Charging for Wireless Rechargeable Sensor Networks"
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
pdf_path: "raw/canonical/ISAC-Enabled_On-Demand_UAV_Charging_for_Wireless_Rechargeable_Sensor_Networks/ISAC-Enabled_On-Demand_UAV_Charging_for_Wireless_Rechargeable_Sensor_Networks.pdf"
raw_md: "raw/canonical/ISAC-Enabled_On-Demand_UAV_Charging_for_Wireless_Rechargeable_Sensor_Networks/full.md"
---
# ISAC-Enabled On-Demand UAV Charging for Wireless Rechargeable Sensor Networks

Muhammad Umar Farooq Qaisar, Lin Zhang, Paolo Bellavista, Shehzad Ashraf Chaudhry, Shamsher Ullah, and Chang Liu

Abstract—Unmanned aerial vehicles (UAVs) equipped with wireless power transfer (WPT) extend the lifetime of wireless rechargeable sensor networks (WRSNs) by delivering energy on demand. This article presents an integrated sensing and communication (ISAC)-enabled on-demand UAV charging framework coordinated by a central base station. A prioritized charging queue captures node urgency and service cost through residual energy, traffic load, estimated UAV travel time, and flightdirection alignment. This bidirectional coupling ensures that scheduling decisions shape the UAV trajectory, while updated mobility estimates from ISAC dynamically reorder the queue. ISAC-assisted estimation of UAV distance, speed, and position updates travel-time predictions under mobility uncertainty. A time-allocated partial charging policy distributes limited hover time across queued nodes according to criticality. Simulations show gains in energy usage efficiency, travel distance, and charging delay compared with representative baselines. We discuss deployment considerations, including computational overhead, scalability, and parameter selection, to aid practitioners evaluating the framework for IoT scenarios.

Index Terms—Wireless rechargeable sensor networks, UAV charging, wireless power transfer, integrated sensing and communication, on-demand scheduling, partial charging.

## I. INTRODUCTION

Wireless sensor networks (WSNs) form the core sensing substrate for the Internet of Things (IoT), enabling continuous monitoring in infrastructure health, precision agriculture, industrial automation, and environmental surveillance. Despite advances in low-power hardware and energy-aware protocol design, most deployments remain fundamentally energylimited because sensing, computation, and multi-hop communications draw from finite on-board batteries [1]. Energy depletion at a small subset of nodes can have a disproportionate impact: a failed relay may disconnect an entire region, while the loss of critical sensing points reduces coverage and degrades inference quality [2]. These failure modes motivate wireless rechargeable sensor networks (WRSNs), where wireless power transfer (WPT) replenishes node energy and enables long-term operation [3].

In WRSNs the primary bottleneck shifts from node energy capacity to charging scheduling and route planning under limited mission time and energy. A mobile charger has to perform three tasks: which nodes to serve, in what order, and for what duration, while working online as requests keep coming in. The scheduler tries to balance urgency (avoiding outages) with service cost (the time and energy to get to and charge a node), and it must do so online as requests come in [4]. The challenges amplified by using unmanned aerial vehicles (UAVs) as mobile chargers are: UAVs are able to bypass the obstacles on the ground. They would reach the dispersed nodes quickly. Nevertheless, the limited flight endurance and expensive hovering tightly couples routing and scheduling. Every additional detour increases propulsion expenditure besides causing a rise in the mission time. This reduces the charging time available for other nodes. Besides, it raises the risk of charging deadlines being missed [5]. Fig. 1 illustrates the considered UAV-assisted WRSN scenario, where a base station coordinates a WPT-equipped UAV that travels to low-energy sensors and hovers to recharge them.

![](images/9295f992d54b3da3db3ee37193ef8ebae20bb3b5a6f3e04c33951314922f39a3.jpg)  
Fig. 1: UAV-assisted wireless rechargeable sensor network (WRSN) scenario: the base station (BS) dispatches a WPTequipped UAV to hover and recharge sensor nodes within an effective charging range.

A further practical challenge is that effective scheduling depends on accurate, timely knowledge of the UAV state and the evolving service cost to each candidate node. Travel time is not static, it varies with the UAV’s current position, instantaneous speed, and the geometry of the partially executed tour. Consequently, route-planning and scheduling strategies based on low-fidelity or delayed mobility state estimates may incur route oscillations, unnecessary trajectory adjustments, and degraded prioritization. This motivates a sensingassisted control loop in which UAV state estimation and queue management are coupled [6]. In particular, integrated sensing and communication (ISAC) [7] enables the network to jointly support data exchange and obtain updated estimates of UAV distance, speed, and position [8]. These estimates are used to refine access-time predictions and charging priorities, improving responsiveness and trajectory efficiency [9]

This article develops OD-UCS, an ISAC-enabled ondemand UAV charging framework for WRSNs coordinated by a central base station. The framework helps in enhancing the decision quality in uncertain mobilities in constrained mission time. The design is distinct for 3 features. A queuing mechanism for the charging of nodes is built that gives priority to service cost and node urgency using residual energy, traffic load, estimated UAV travel time, and compatibility with flight direction. Moreover, It uses the ISAC-assisted estimation of UAV distance, speed, and position to continuously update travel-time predictions on the fly to enhance the reliability of scheduling decisions as UAV moves. Third, it introduces a time allocation-based partial charging policy, which distributes a limited hover time among the queued nodes based on their criticality to cover more nodes while missing fewer deadlines. Since the queue explicitly considers travel time and alignment of the travelling direction, both determined by the UAV’s current position and heading, any scheduling decision implicitly shapes the trajectory, and vice versa, trajectory updates prompted via ISAC estimation trigger a reordering of priority. The design contains a close bidirectional coupling.

The rest of this article is structured as follows. Section II presents a review of related work. It positions our contribution with respect to the literature. Section III presents the system model encompassing the network, charging and mobility components. The proposed charging model with priority queue, ISAC-aided state estimation, time-allocated partial charging is detailed in Section IV. Section V evaluates and discusses the performance. The article is concluded in Section VI and future research directions are outlined.

## II. RELATED WORK

UAV-enabled WRSNs have evolved from basic tourplanning formulations to integrated service frameworks that must balance mobility limits, heterogeneous node depletion, and timely response under realistic deployment constraints. Rather than surveying each contribution in isolation, we organize the discussion along three design dimensions, architecture and coordination, trajectory and energy management, and charging scheduling intelligence, and identify where prior work leaves gaps that our framework addresses.

Architecture and coordination. Early work focused on hybrid vehicle-drone architectures. Chen et al. [5] proposed a collaborative model that combines wireless charging vehicles (WCVs), WCV-carried drones, and independent drones, using a ring-based partition to coordinate responsibilities and mitigate the respective limitations of ground vehicles and drones in large WRSNs. Zhao et al. [6] investigated UAV dispatch planning for bridge-monitoring WRSNs, emphasizing feasible routing under obstacle constraints. These works demonstrate multi-platform coordination but do not exploit ISAC for realtime state estimation within the scheduling loop.

Trajectory and energy management. Lin et al. [9] formulated a period-area coverage perspective in which a UAV must both recharge sensors and provide periodic sensing coverage. Chen et al. [10] presented CGDA-Q for agricultural IoT, combining adaptive charging cells and Q-learning for path planning. Liu et al. [11] studied an IRS-assisted UAV charging architecture with a two-stage trajectory and phase-optimization approach. Ma et al. [12] explored laser-powered UAV far-field wireless charging coupled with data backhauling. These approaches advance physical-layer energy delivery but generally assume accurate UAV state and do not couple state estimation with online priority updates.

Charging scheduling intelligence. Liu et al. [13] adopted deep reinforcement learning (DQN) for UAV-assisted charging to reduce sensor downtime. Betalo et al. [14] used multiagent deep reinforcement learning to coordinate charging and path planning across multiple UAVs. Gupta et al. [15] modeled UAV-enabled charging under a non-cooperative pricing framework among charging providers. While these learningbased approaches improve adaptability, they require extensive training and offer limited interpretability.

Positioning of this work. Prior UAV-enabled WRSN charging studies improve routing and scheduling under energy and time constraints, yet they generally assume accurate mobility information and do not tightly couple state estimation with online priority updates. They also lack principled, time-allocated partial-charging policies. Our framework integrates ISACassisted estimation into the scheduling loop, uses transparent priority scheduling that needs no training, and applies urgencyweighted partial charging to improve responsiveness under constrained endurance.

## III. SYSTEM MODEL

## A. Network Model and Assumptions

We consider a WRSN deployed over a $5 0 0 \times 5 0 0 \mathrm { m ^ { 2 } }$ . A set of rechargeable sensor devices is randomly scattered within the area. Each sensor node is equipped with sensing, processing, and communication capabilities, as well as a rechargeable battery.

The base station is located at the center of the monitoring area. It acts as the sink for data traffic, serves as the controller for charging operations and a depot for the UAV. It maintains state information about sensor nodes, including their estimated positions, residual energy levels, and traffic statistics. This information is obtained through periodic status reports and, when necessary, through on-demand queries.

An unmanned aerial vehicle acts as an aerial mobile charger. The UAV is equipped with a WPT module for wirelessly transferring energy to sensor nodes and with communication interfaces for exchanging control and telemetry data. A UAV departs from the base station with enough energy to serve a subset of nodes and safely return to the base. Its movement is limited by the maximum speed limit, acceleration limits and flight time limit.

The energy consumption profile of sensor nodes involves sensing, local processing and communication. Their energy consumption depends on their sensing duty cycle, traffic generation rate and their forwarding role in the network topology. Nodes send their remaining energy and traffic load to base station. They can communicate through many hops if they are not in direct range.

When a node’s residual energy falls below a predefined threshold (30 %), it generates a charging request that includes the node’s identifier, its estimated position, current residual energy, and recent traffic statistics. The request is forwarded to the base station, which maintains a global queue of nodes awaiting charging. Nodes that exhaust their battery before being served enter a dormant state, they resume operation once recharged in a subsequent mission. This dormancy mechanism bounds the impact of missed deadlines and motivates the urgency-weighted design of the priority queue.

The base station manages the flight and charging operation of the UAV. The base station calculates the charging schedule, including which nodes the UAV should visit, when to visit them, and how long to charge each node. It updates this schedule as new charging requests arrive, and ISAC provides status updates on the UAV. The base station ensures that the UAV has sufficient energy for a safe return to the depot.

## B. Charging Model

The WPT system of the UAV operates when the UAV hovers above the sensor node in a particular proximity. It is assumed that as long as the UAV is within an acceptable vertical and horizontal distance from a node, a near-constant charging rate can be achieved via alignment and coupling conditions. This simplifies the charging model while capturing a key dependence on dwell time.

Each operation of charging consists of a travel phase and a dwell phase. In the travel phase, the UAV moves to the neighborhood of the target node from its present position. The energy used for travelling in UAV is speed dependent. It changes altitude and make some diversion in order to avoid obstacles. During the dwell phase, the UAV remains stationary over the node and employs WPT to transfer energy. Hovering uses a lot of energy, which needs to be factored into mission budget.

We assume that either the energy consumption profile of the UAV is known or can be accurately estimated from previous calibration. The profile is used by the base station to decide on the number of nodes to be served in a single mission and to save some energy for return to the depot. The outcome is an efficient charging time budget for every mission, representing the total dwell time available among nodes.

This article abstracts out all the physical-layer details into charging rate parameter and focusses on the scheduling framework for allocating charging time.

## C. UAV Mobility Model

The UAV follows a controlled stop-and-charge operation coordinated by the base station (BS). It departs from the BS, travels in straight-line point-to-point flight between scheduled service locations at a constant speed $v _ { \mathrm { m a x } } .$ , and hovers to recharge the selected sensor node. The BS updates the service sequence online as new requests arrive.

Before moving to the next target, the UAV verifies that its remaining onboard energy can support the flight to the candidate node, the planned hovering and charging duration at that node, and the subsequent return trip to the base station including a safety margin. Return-to-depot safety constraint. The UAV may proceed to a candidate node only if its remaining energy exceeds the total of the propulsion energy needed to reach that node, the hovering energy consumed during the allocated charging time at that node, and the energy required to fly back to the base station together with a safety reserve. This constraint is applied before each node is admitted into the active charging queue, so that the return-to-depot guarantee is integrated into the scheduling policy rather than checked only at the end of a tour. If the remaining energy is insufficient, the node is deferred to the next mission and the UAV returns to the base station for energy replenishment.

The mobility constraints, maximum speed, endurance limit, and the return-to-depot guarantee, directly shape the scheduling decisions in Section IV. Because the priority queue accounts for estimated travel time and direction alignment (both functions of the UAV’s current state), every reordering of the queue implicitly adjusts the trajectory. Conversely, when ISAC updates revise the UAV’s estimated position or speed, the travel-time and direction terms in the priority formula change, which may trigger a different node ordering and hence a different planned path. This bidirectional coupling between scheduling and trajectory is a central design feature: it ensures that the UAV follows a route that is both urgency aware and mobility efficient, adapting in real time as new information arrives.

## IV. PROPOSED CHARGING MODEL

The proposed ISAC-enabled charging framework consists of three interrelated components: an ISAC-assisted prioritized charging queue, an ISAC-driven UAV state estimation mechanism, and a time-allocated partial charging model. The base station orchestrates these components to plan and continuously refine UAV missions in response to evolving network conditions and updated mobility estimates. Fig. 2 provides a highlevel view of the system model and the closed-loop interaction among queue-based scheduling, ISAC-assisted UAV state estimation, and time-allocated partial charging.

## A. ISAC-Assisted Prioritized Charging Queue

The charging queue is designed for demand and cost representing the urgency of charging and costs of serving. The base station maintains four key attributes for every node in queue, namely residual energy, traffic load, estimated UAV travel time and direction alignment.

![](images/7bcc37b60bb179ed82908c9f9e46862e9cffdf8caa6117a63af285eb314d43ee.jpg)  
Fig. 2: Overview of the BS-orchestrated ISAC-enabled on-demand UAV charging framework for WRSNs: (left) system model showing status reporting, charging requests, and the UAV stop-and-charge operation with WPT within an effective range; (right) proposed charging model integrating (1) a prioritized charging queue, (2) ISAC-driven UAV state estimation with BSside fusion, and (3) time-allocated partial charging with feedback from UAV state reports.

1) Residual energy: Residual energy is the most direct representation of charging urgency, as it quantifies how close a sensor is to energy depletion. Nodes that have extremely low remaining energy will fail before the UAV can get there. This leads to a deterioration in sensing coverage and, in multi-hop topologies, connectivity loss.

The base station also monitors the depletion rate of energy of each node. If one of them faces elevated consumption, their priorities may differ even if they have equal residual energy. As residual energy decreases, the urgency score increases, with a steeper response below a safety threshold. Consequently, the near-critical nodes move to the head of the queue.

2) Traffic load: Traffic load is the extent to which a node participates in communication. In WRSNs, a node having high traffic consumes energy much faster than a leaf node since it has to receive, process and forward packets. Protecting the relay availability of high-load nodes is crucial for ensuring end-to-end data delivery. Base station estimates load through periodic report, which gets mapped into a load-related score.

3) Estimated UAV travel time: The estimated travel time of the UAV from its current location to a candidate node reflects the service cost in terms of delay and propulsion energy. Prolonged travel time causes the node to require more energy to reach and delays charging of other waiting nodes. The base station determines this estimate from the up-to-date UAV state and the node’s location. When other attributes are similar, shorter travel times are preferred, but the scheduler specifically weighs urgency against mobility cost.

4) Flight direction alignment: The flight direction alignment determines whether a candidate node falls in the forward direction of the current UAV motion. When the scheduler selects nodes along the current heading, the UAV prevent frequent sharp turns or backtracking, thus limiting travel time and propulsion-energy expenditure.

The base station determines the alignment by using the angle between the UAV heading vector and the displacement vector to each node. When multiple candidates have similar urgency, the alignment score serves as a cost-sensitive tiebreaker.

cost-aware tie-breaker.

5) Composite priority and queue maintenance: The base station combines the four attribute scores into a single composite priority value for each node. The urgency score derived from residual energy and the traffic-load score both increase the priority, because nodes that are closer to depletion or carry more traffic should be served sooner. The estimated travel time decreases the priority, because longer flights consume more energy and delay service to other nodes. The directionalignment score increases the priority, because nodes that lie along the current flight path reduce the need for sharp turns and backtracking. A separate weight controls the relative influence of each factor, and all scores are normalized to a common range before combination.

The weight selection reflects the design rationale. The urgency weight is set high because preventing outages is the primary objective. The travel-time weight is set to discourage long detours. The traffic-load and direction-alignment weights serve as secondary modifiers. Practitioners can tune these weights: increasing the traffic-load weight protects relayheavy topologies, increasing the direction-alignment weight favors compact trajectories. Weight tuning guideline: urgency weight 0.4, traffic-load weight 0.2, travel-time weight 0.25, and direction-alignment weight 0.15 (summing to one after normalization). Sensitivity is discussed in Section V-E.

After computing composite priorities, the base station constructs the charging queue. The queue is updated online to reflect time-varying node states and UAV motion: a node is removed once it has been served, and newly reported demands are inserted as they arrive. The base station recomputes priorities and reorders the queue whenever updated attributes alter the service order.

Return-to-depot integration. The feasibility check is applied before a node is admitted into the active queue. Specifically, when evaluating whether to enqueue a new candidate, the base station first estimates the energy that would be required to fly from that candidate back to the base station using the current UAV position and the planned return path, and then verifies that the UAV’s remaining energy is sufficient to reach the candidate, hover and charge there, and return safely. If not, the candidate is placed in a deferred list for the next mission. This ensures that the return-to-depot guarantee is not an afterthought but an integral part of the queue construction process.

## B. ISAC-Driven UAV State Estimation

Online charging scheduling requires timely and accurate knowledge of the UAV state (position and velocity), since travel-time and direction-related priority terms depend on the current UAV motion. Although the UAV can provide onboard state reports, these may be degraded by sensor errors and reporting latency. ISAC introduces a complementary, networkside sensing modality that can support UAV tracking using communication waveforms and sensing processing.

When the UAV enters a region with pending charging demands, the base station schedules an ISAC sensing round and selects a subset of sensor nodes with favorable geometry and reliable links. The selected nodes transmit predefined reference waveforms and, in a time-separated sensing interval, listen to the returned signals to capture echoes associated with the UAV. Each node then extracts compact observables (e.g., delayand Doppler-related features) via lightweight correlation-based processing and forwards them to the base station.

At the base station, delay-related observables are mapped to range information, and Doppler-related observables provide information about radial motion. The base station uses geometric localization to estimate the UAV’s position and refines the velocity estimate based on the temporal evolution of measurements from multiple nodes.

The resulting ISAC-based estimates are fused with the UAVreported state using a Kalman-type filter. In essence, the filter maintains a predicted UAV state based on a dynamic motion model, when a new ISAC-derived measurement arrives, the filter computes a Kalman gain that balances trust between the prediction and the measurement, producing a corrected state estimate. This fusion mitigates two kinds of errors: sensor noise and bias in the UAV’s onboard inertial navigation, which can accumulate as drift over time and outliers or inaccuracies in individual ISAC measurements, which are smoothed by the filter’s prediction step. The updated state is then fed back to the queue-maintenance procedure to refresh the travel-time and direction-compatibility terms, which may reorder composite priorities as the UAV progresses along its route.

Overhead and scalability. Node-side correlation takes milliseconds. Observables are a few bytes per round. BS-side fusion is $O ( n )$ with $n \ = \ 3 { \mathrm { - } } 5 .$ , negligible against secondsscale travel. Queue reordering is $O ( K \log K )$ and remains tractable for several hundred nodes. End-to-end latency is tens to hundreds of milliseconds, small compared with travel times. For large networks, ISAC uses a geometry-selected subset, updates are event-driven, and the area can be zoned. Centralized control suits networks up to a few hundred nodes, distributed extensions are left for future work.

Robustness. The Kalman filter handles moderate dynamics and degrades gracefully when measurement quality drops. Independent safeguards, the return-to-depot constraint and the urgency score, keep the queue functional even if estimates lag. Performance trends remain consistent across 100 − −500 nodes, indicating graceful degradation.

## C. Partial Charging with Time Allocation

We adopt a partial-charging policy in which the UAV distributes hover time among queued nodes rather than fully recharging one at a time. Full recharge maximizes per-node delivery but risks missing deadlines for others. Partial charging serves more nodes per mission, improves coverage, and reduces outage probability.

A risk of partial charging is repeated near-depletion cycles. We mitigate this by defining charging demand to push nodes above a safety margin, not merely to the threshold. The steep urgency score also prevents indefinite postponement of nearcritical nodes.

Under the adopted WPT abstraction, once the UAV is positioned within an effective charging region above a node, the delivered energy increases approximately in proportion to the time spent hovering.

To compute the time allocation, the base station first uses each node’s residual energy to derive its charging demand. Both residual energy and traffic load contribute to the urgency/criticality weight: a node with low residual energy and high traffic load receives a higher weight than a node with the same residual energy but lower traffic demand, because its failure would disrupt more network traffic. Specifically, the base station computes an urgency weight for each node by adding its residual-energy urgency score to a scaled version of its traffic-load score, where a tunable parameter controls how strongly traffic load influences the result. This per-node sum is then divided by the corresponding total across all nodes currently in the charging queue, yielding a normalized urgency weight. The charging time assigned to a node is finally obtained by multiplying its urgency weight by the total hover time budget available for the current mission.

TABLE I: Simulation parameters.

<table><tr><td>Parameter</td><td>Value</td></tr><tr><td>Number of sensors</td><td>100–500</td></tr><tr><td>Sensor communication radius</td><td>50 m</td></tr><tr><td>Sensor sensing radius</td><td>25 m</td></tr><tr><td>Sensor battery capacity</td><td>10 J</td></tr><tr><td>Sensor energy consumption rate (sensing + processing + communication)</td><td>0.01 J/s</td></tr><tr><td>Threshold of charging request</td><td>30% residual energy</td></tr><tr><td>UAV speed</td><td>20 m/s</td></tr><tr><td>UAV propulsion power (flight)</td><td>150 W</td></tr><tr><td>UAV hovering power</td><td>200 W</td></tr><tr><td>UAV initial energy</td><td>500 kJ</td></tr><tr><td>UAV RF emission power (WPT)</td><td>200 W</td></tr><tr><td>Effective WPT charging rate</td><td>5 W</td></tr><tr><td>Monitoring area</td><td> $500 \times 500 \text{ m}^{2}$ </td></tr></table>

Reallocation timing. The aggregated charging time and per-node allocations are recomputed each time the charging queue changes, that is, whenever a new node joins the queue, a node is served and removed, or ISAC updates trigger a priority reordering. In practice, this means that time splits are recomputed before each hover, not on a fixed periodic schedule. This event-driven reallocation ensures that the time allocation always reflects the most current urgency distribution and UAV energy budget.

When a new queue node joins, the base station recomputes aggregate time and per-node allocations to keep the partialcharging plan consistent.

## V. PERFORMANCE EVALUATION AND DISCUSSION

## A. Simulation Configuration

To evaluate the effectiveness of the proposed framework, we consider a simulated WRSN environment in a square monitoring area with numerous sensor nodes. The nodes are randomly distributed to capture spatial heterogeneity. The base station is located in the central part of the area and operates as a data sink and UAV depot. The summery of simulation parameters is given in Table I.

The sensor battery capacity and energy consumption rate are selected to represent a low-power sensor model in the simulator. Under the assumed consumption rate of 0.01 J/s, the nominal battery lifetime is approximately 1,000 s in continuous active operation. The UAV is initialized with 500 kJ of energy, which is sufficient to support multiple flight and hovering operations during a mission. The UAV propulsion and hovering powers are chosen to reflect typical values for small to medium multirotor platforms. The effective 5 W WPT charging rate represents the received charging power after accounting for propagation and conversion losses.

Sensor nodes have identical battery capacities but may experience different traffic loads. Some nodes generate more data or participate in more forwarding, which leads to heterogeneous energy depletion across the network. Nodes consume energy for sensing, processing, and communication, and their residual energy decreases accordingly. When the residual energy of a node falls below the predefined threshold, it issues a charging request to the base station.

The UAV begins each mission at the base station with a fixed energy budget. Its speed and propulsion power are set to representative values for a small to medium multirotor UAV. Before the first charging decision is made, the UAV remains at the base station; once dispatched, it flies in a straight line to the first scheduled service location. Thereafter, the UAV trajectory is determined by the charging schedule.

The proposed ISAC-enabled framework (OD-UCS) is compared with two representative baselines. The first baseline is CGDA-Q [10], which constructs charging cells to determine UAV hover locations, dynamically assigns charging tasks among multiple UAVs to balance load, and applies Qlearning for charging-path planning. The second baseline is MA-DDQN [14], which jointly optimizes dynamic charging decisions and UAV trajectories to reduce task completion time and improve sensor survivability.

Both baselines are evaluated under identical network topologies, sensor placements, and traffic patterns. Although CGDA-Q was originally designed for multi-UAV operation, it is configured with a single UAV in this study to ensure a fair comparison under the same system scale.

## B. Energy Usage Efficiency

Fig. 3a shows that the proposed scheme achieves consistently higher energy usage efficiency than CGDA-Q and MA-DDQN across all node densities, meaning that a larger fraction of the UAV’s onboard energy is converted into effective charging rather than propulsion and hovering overhead. This gain becomes more pronounced as the network becomes denser, indicating that the priority design scales gracefully.

## C. Travel Distance

As shown in Fig. 3b, the proposed scheme achieves a shorter average UAV travel distance per tour than the two baselines, resulting in more compact and directionally coherent trajectories. The reduced path length directly reduces the propulsion energy consumption and provides more of the limited energy budget of the UAV to wireless power transfer.

## D. Charging Delay

As shown in Fig. 3c, the proposed scheme reduces the average charging delay much more than CGDA-Q and MA-DDQN, and can serve more charging requests in the same operation time. This indicates that the policy not only improves the service speed of energy-critical nodes, but also the temporal fairness in general.

## E. Sensitivity to Priority Weights

To assess the impact of weight selection on performance, we varied the urgency weight α from 0.2 to 0.6 (adjusting $\gamma$ inversely, with $\beta$ and $\delta$ held constant). The results show that energy usage efficiency varies by less than 5% across this range, and the relative ranking against both baselines is preserved. This insensitivity indicates that the framework is robust to moderate weight perturbations. The largest performance drop occurs at very low $\alpha ~ ( \leq ~ 0 . 2 )$ , where urgency is under-weighted and critical nodes are deprioritized, this boundary case reinforces the design rationale for setting α relatively high.

![](images/8c206953555a529dabf7cb637de8b57c857cd5eb290990a7cfcfb736b1627025.jpg)  
(a) Energy Usage Efficiency

![](images/65a974699e136ad3e7ed2ff19ad0a0bf71e2b6715719d49d17dc5c0693c2cfe6.jpg)  
(b) Travel Distance  
Fig. 3: Performance over sensor nodes.

![](images/c29ff067bae505c9bbdb80c671f3153d950f73c37aea0cb69253673d84885e7f.jpg)  
(c) Charging Delay

## VI. CONCLUSION AND FUTURE DIRECTIONS

This article has presented an ISAC-enabled, on-demand UAV charging framework for wireless rechargeable sensor networks that tightly couples queue-based charging decisions, mobility-aware service costs, and ISAC-assisted UAV state awareness under time-varying traffic and energy conditions. By jointly considering node urgency (residual energy and traffic load), UAV mobility cost (travel time and flightdirection compatibility), and partial-charging control, the proposed design improves the utilization of the UAV’s limited onboard energy and enhances the temporal reliability of energy replenishment. Simulation results demonstrate that the framework consistently attains higher energy usage efficiency, shorter and more compact UAV trajectories, and significantly reduced charging delay compared with representative baseline schemes.

Key design insights. First, coupling ISAC state estimation with online priority updates improves scheduling quality compared with approaches that rely on static information. Second, partial charging with urgency-weighted allocation serves more nodes per mission without creating near-depletion cycles, provided a safety margin is built into the demand calculation. Third, integrating the return-to-depot constraint into queue construction guarantees safe tours by design.

Open challenges and future directions. Several promising research directions emerge from this work:

• Multi-UAV coordination: Distributed scheduling, conflictfree trajectory design, and energy-aware task allocation.

• Realistic charging andflight models: Detailed WPT models, flight-energy models, and operational constraints such as no-fly zones.

• Scalability: Distributed or edge-assisted architectures for networks exceeding several hundred nodes.

• Heterogeneous UAV platforms: Fleets with different endurance, speed, and charging capabilities.

• Security: Integrity of charging requests and ISAC measurements against spoofing or jamming.

## REFERENCES

[1] M. U. Farooq, X. Wang, A. Hawbani, L. Zhao, A. Al-Dubai, and O. Busaileh, “Sdorp: Sdn based opportunistic routing for asynchronous

wireless sensor networks,” IEEE Transactions on Mobile Computing, vol. 22, no. 8, pp. 4912–4929, 2022.

[2] B. Zhou, C. Wu, Q. Yang, Y. Qian, and Y. Nie, “Resilient sensor data dissemination to mitigate link faults in iot networks with long-haul optical wires for power transmission grids,” IEEE Internet of Things Journal, vol. 11, no. 9, pp. 15 919–15 939, 2024.

[3] M. U. F. Qaisar, W. Yuan, P. Bellavista, F. Liu, G. Han, R. S. Zakariyya, and A. Ahmed, “Poised: Probabilistic on-demand charging scheduling for isac-assisted wrsns with multiple mobile charging vehicles,” IEEE Transactions on Mobile Computing, vol. 23, no. 12, pp. 10 818–10 834, 2024.

[4] S. A. Aziz, X. Wang, A. Hawbani, B. Qureshi, S. H. Alsamhi, A. Alabsi, L. Zhao, A. Al-Dubai, and A. Ismail, “Wireless rechargeable sensor networks: Energy provisioning technologies, charging scheduling schemes, and challenges,” IEEE Transactions on Sustainable Computing, vol. 10, no. 5, pp. 873–890, 2025.

[5] J. Chen, C. W. Yu, and R.-H. Cheng, “Collaborative hybrid charging scheduling in wireless rechargeable sensor networks,” IEEE Transactions on Vehicular Technology, vol. 71, no. 8, pp. 8994–9010, 2022.

[6] C. Zhao, Y. Wang, X. Zhang, S. Chen, C. Wu, and K. L. Teo, “Uav dispatch planning for a wireless rechargeable sensor network for bridge monitoring,” IEEE Transactions on Sustainable Computing, vol. 8, no. 2, pp. 293–309, 2022.

[7] M. U. F. Qaisar, W. Yuan, P. Bellavista, G. Han, and A. Ahmed, “Isacassisted wireless rechargeable sensor networks with multiple mobile charging vehicles,” IEEE Internet of Things Magazine, vol. 7, no. 6, pp. 80–86, 2024.

[8] K. Meng, Q. Wu, J. Xu, W. Chen, Z. Feng, R. Schober, and A. L. Swindlehurst, “Uav-enabled integrated sensing and communication: Opportunities and challenges,” IEEE Wireless Communications, vol. 31, no. 2, pp. 97–104, 2023.

[9] C. Lin, S. Hao, W. Yang, P. Wang, L. Wang, G. Wu, and Q. Zhang, “Maximizing energy efficiency of period-area coverage with a uav for wireless rechargeable sensor networks,” IEEE/ACM Transactions on Networking, vol. 31, no. 4, pp. 1657–1673, 2022.

[10] J. Chen, X. Li, B. Cai, J. He, Y. Ma, and J. Liu, “A reinforcement learning-based energy charging strategy for agricultural internet of things with multi-uav-assisted wrsn,” IEEE Internet of Things Journal, vol. 12, no. 23, pp. 49 022–49 035, 2025.

[11] X. Liu, C. Zhao, S. Chen, T. Wang, and F. Chen, “Drl-based charging strategy optimization for irs-assisted uav in wireless rechargeable sensor networks,” ACM Transactions on Sensor Networks, vol. 22, no. 3, pp. 1–38, 2026.

[12] X. Ma, X. Liu, and N. Ansari, “Green laser-powered uav far-field wireless charging and data backhauling for a large-scale sensor network,” IEEE Internet of Things Journal, vol. 11, no. 19, pp. 31 932–31 946, 2024.

[13] N. Liu, J. Zhang, C. Luo, J. Cao, Y. Hong, Z. Chen, and T. Chen, “Dynamic charging strategy optimization for uav-assisted wireless rechargeable sensor networks based on deep q-network,” IEEE Internet of Things Journal, vol. 11, no. 12, pp. 21 125–21 134, 2023.

[14] M. L. Betalo, S. Leng, A. M. Seid, H. N. Abishu, A. Erbad, and X. Bai, “Dynamic charging and path planning for uav-powered rechargeable wsns using multi-agent deep reinforcement learning,” IEEE Transactions on Automation Science and Engineering, vol. 22, pp. 15 610–15 626, 2025.

[15] A. K. Gupta and M. R. Bhatnagar, “A non-cooperative pricing strategy for uav-enabled charging of wireless sensor network,” IEEE Transactions on Green Communications and Networking, vol. 9, no. 2, pp. 459–470, 2024.