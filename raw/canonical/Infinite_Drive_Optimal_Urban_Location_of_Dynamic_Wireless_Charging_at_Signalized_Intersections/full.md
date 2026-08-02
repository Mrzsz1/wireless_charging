---
title: "Infinite Drive: Optimal Urban Location of Dynamic Wireless Charging at Signalized Intersections"
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
pdf_path: "raw/canonical/Infinite_Drive_Optimal_Urban_Location_of_Dynamic_Wireless_Charging_at_Signalized_Intersections/Infinite_Drive_Optimal_Urban_Location_of_Dynamic_Wireless_Charging_at_Signalized_Int.pdf"
raw_md: "raw/canonical/Infinite_Drive_Optimal_Urban_Location_of_Dynamic_Wireless_Charging_at_Signalized_Intersections/full.md"
---
# Infinite Drive: Optimal Urban Location of Dynamic Wireless Charging at Signalized Intersections

Yudai Honma<sup>a,\*</sup>, Daisuke Hasegawa<sup>b</sup>, Katsuhiro Hata<sup>c</sup>, Xuesong (Simon) Zhou<sup>d</sup>, Michael J. Kuby<sup>e</sup>, Takashi Oguchi<sup>a</sup>

a) Institute of Industrial Science, The University of Tokyo, Komaba 4-6-1, Meguro-ku, Tokyo 153-8505, Japan

b) Center for Real Estate Innovation, The University of Tokyo, Hongo 7-3-1, Bunkyo-ku, Tokyo 113-0033, Japan

c) College of Engineering, Shibaura Institute of Technology, Toyosu 3-7-5, Koto-ku, Tokyo 135-8548, Japan

d) School of Sustainable Engineering and the Built Environment, Arizona State University, Tempe, AZ 85281, USA

e) School of Geographical Sciences and Urban Planning, Arizona State University, Tempe, AZ 85281, USA

\* Corresponding author. Email: yudai@iis.u-tokyo.ac.jp

## Abstract

Dynamic Wireless Power Transfer (DWPT) could eliminate plug-in charging in cities, but optimal urban deployment is complex. This paper develops a mixed-integer programming model that optimizes DWPT location under signalized intersection dynamics—acceleration, deceleration, and queue-positiondependent dwell time—through probabilistic signal patterns and saturation headway-based modeling. A case study of Kawagoe City, Japan, shows that electrifying 1.5% of the road network is sufficient to sustain continuous urban EV operation without plug-in charging for the baseline scenario, and at most 2.9% suffices across all tested assumptions. Monte Carlo simulations of continuous trip chains averaging approximately 600 km and reaching up to approximately 800 km confirm that optimized 12 kWh-battery deployments sustain operation in all simulated runs, revealing an infrastructure–battery tradeoff corresponding to roughly 1.7–3.0 tonnes CO<sub>2</sub>e of avoided battery manufacturing emissions per vehicle relative to a conventional 40 kWh urban EV. These findings position DWPT deployment as an environmentally efficient pathway for sustainable urban mobility when deployed optimally.

Keywords: Dynamic wireless power transfer (DWPT); Charging infrastructure location; Signalized intersections; Mixed-integer programming; Battery sizing; Battery manufacturing emissions

## 1. Introduction

For over a century, automobiles have relied primarily on fossil fuels, contributing significantly to air pollution and climate change. In response, many governments have committed to ending sales of new fossil-fuel-powered cars and vans by 2040 or earlier, and by no later than 2035 in leading markets (UK Government, 2022). As a result, electric vehicles (EVs) are positioned as a central pillar in the decarbonization of transport systems. While EVs eliminate tailpipe emissions, their overall economic and environmental impacts depend critically on the availability of efficient, equitable, and low-carbon charging infrastructure.

Most current EV charging relies on stationary charging at plug-in stations, requiring vehicles to stop for extended periods. This introduces inefficiencies such as route detours, idle queuing, and spatial mismatches between infrastructure and demand. These issues are particularly problematic in dense urban environments, where travel flexibility, energy efficiency, and curbside space are limited. As a result, extensive research has explored the optimal location of charging stations (Coffman et al., 2017; Kchaou-Boujelben, 2021; Rahman et al., 2016), aiming to minimize detours, balance usage, and ensure networkwide accessibility.

To support this, a wide range of facility-location models have been proposed. Classical models such as the p-median problem minimize average travel distance to charging sites (ReVelle and Swain, 1970), while the flow-capturing location model (FCLM) identifies station locations that intercept en-route travel flows (Hodgson, 1990). These frameworks have been extended to accommodate path deviations, travel time uncertainty, and stochastic demand (Berman et al., 1995). For EV-specific applications, the flow-refueling location model (FRLM) introduced by Kuby and Lim (2005) explicitly accounts for limited driving range and the need for multiple en-route charging events. More recent mixed-integer programming (MIP) formulations incorporate features such as battery charge tracking, station capacity, and layered coverage (Capar et al., 2013; Kim and Kuby, 2012; Lim and Kuby, 2010; Wang and Lin, 2009). Recent data-driven approaches further leverage trip origin–destination data and urban informatics to estimate charging demand and optimize plug-in station placement (Yi et al., 2022).

These models have also highlighted systemic constraints. As EV adoption grows, charging station congestion, long waiting times, and energy delivery bottlenecks at the system level are becoming critical concerns (Bruglieri et al., 2019; Honma and Toriumi, 2017, 2014; Upchurch et al., 2009). Industry responses—such as ultra-fast charging or oversized onboard batteries—can mitigate range anxiety, but they impose substantial costs. High-power charging stresses local electricity grids, while larger batteries increase vehicle weight, reduce energy efficiency, and intensify reliance on scarce materials such as lithium and cobalt. Frequent high-voltage charging may also accelerate battery degradation, shortening battery life. These challenges underscore the need for infrastructure strategies that address urban EV energy supply in a system-level and resource-efficient manner.

Dynamic Wireless Power Transfer (DWPT) has emerged as a promising alternative. By enabling EVs to receive energy while in motion via inductive coils embedded in roadways, DWPT shifts the charging paradigm from stationary refueling to integrated, en-route energy delivery (Bi et al., 2016; Lukic and Pantic, 2013; Miller et al., 2015). This approach reduces range anxiety and queuing delays while offering broader system benefits. In particular, DWPT allows EVs to operate with smaller onboard batteries, reducing vehicle mass, improving energy efficiency, and lowering material demand (Bi et al., 2019).

Technical progress in DWPT has accelerated in recent years, accompanied by a growing number of real-world demonstration projects. Korea’s On-Line Electric Vehicle (OLEV) program provided early operational insights through full-scale deployment on public roads (Hwang et al., 2018; Jang et al., 2015; Ko and Jang, 2013). More recently, Electreon has introduced modular roll-out installation technologies and is conducting pilot projects across Europe (Electreon, 2025). In Japan, DWPT-equipped buses have been deployed as part of demonstration projects for Expo 2025 Osaka, Kansai (Japan Association for the 2025 World Exposition, 2025). Together, these demonstrations indicate that DWPT technology is approaching deployment readiness, shifting the research focus from feasibility toward planning and optimization.

Despite this progress, optimal location planning for DWPT—particularly in intersectiondominated urban environments—remains insufficiently understood. Compared to plug-in charging infrastructure, location modeling for DWPT is still relatively new, though rapidly expanding. A key stream of prior work formulates DWPT deployment as a network problem with multiple routes and congestion effects, often using equilibrium-based approaches. For example, Riemann et al. (2015) proposed a flow-capturing model under stochastic user equilibrium. Chen et al. (2016) proposed an optimization model for deploying DWPT charging lanes in transportation networks under user equilibrium. Manshadi et al. (2018) examined coupled electricity–transportation interactions. Liu et al. (2021) incorporated electricity prices into DWPT location decisions. More recently, Ngo et al. (2020) addressed optimal DWPT positioning across road networks for battery electric vehicles, and Tran et al. (2022) developed an urban DWPT lane location model incorporating dynamic route-choice behavior. Beyond equilibrium formulations, researchers have explored alternative modeling paradigms, including heuristic economic design (Ko et al., 2015), joint planning of stationary and dynamic charging (Chen et al., 2017), and robust optimization approaches (Alwesabi et al., 2022; Liu and Song, 2017).

In parallel, economic feasibility and implementation considerations have been emphasized as essential for real-world viability. Prior studies have examined DWPT deployment at metropolitan and corridor scales, addressing cost effectiveness and system design under realistic spatial constraints (Fuller, 2016; Honma et al., 2024; Mubarak et al., 2021; Trinko et al., 2022; Yan et al., 2022), while other work has focused on pricing and broader economic rationality (He et al., 2013; Jeong et al., 2015). These strands collectively underscore that DWPT planning requires both network-level optimization and careful attention to operational realism.

Most existing DWPT location models, however, are developed for highway corridors or represent urban networks at a coarse resolution, without explicitly capturing the intersection-dominated stop-and-go dynamics that shape charging opportunities in cities. While Zhang et al. (2021) examined eco-driving control for connected and automated electric vehicles at signalized intersections with wireless charging, their focus was on vehicle-side speed optimization rather than citywide DWPT location decisions. In urban contexts, traffic signals, queue formation, and acceleration–deceleration behavior jointly determine (i) the time vehicles spend over embedded coils and (ii) the corresponding energy consumption during approach, stopping, and discharge. DWPT installations near traffic signals may therefore be especially effective, but their benefits are highly sensitive to signal timing, queue-position-dependent dwell times, and approach speeds. Determining where and how much DWPT to install in cities thus requires a highresolution modeling framework that integrates traffic kinematics, queuing behavior, and energy transfer dynamics.

In this study, we refer to these combined effects of signal timing, queue formation, and acceleration–deceleration behavior near intersections as signalized intersection dynamics, which fundamentally govern both EV energy consumption and in-motion charging opportunities in urban networks. Fig. 1 conceptually illustrates how DWPT effectiveness can differ between highway and urban settings due to these intersection-driven dynamics (Japan Science and Technology Agency, 2025).

![](images/a2c29c3f8e4ca62de7a54e6e825af2d7e9a68739ccce1421707ef365ec890f4b.jpg)  
(a) DWPT in expressway scenario

![](images/9496b781dbfb6b55c1aa3a88794d1cb6796db1c4f83d72bf8b8fbbf0d6087a8e.jpg)  
(b) DWPT in urban-scale scenario

Fig. 1. Graphical image of DWPT location in various scenarios from Japan Science and Technology Agency (2025)

A critical foundation for DWPT location optimization lies in understanding how vehicle kinematics—particularly acceleration and deceleration near intersections—affect both energy consumption and charging opportunity. Table 1 summarizes the methodological streams that inform this study. The kinematic equations developed in this study (Eqs. 13–21) generate segment-level trajectory data—the shared foundation across all streams.

Table 1  
Methodological foundations linking vehicle kinematics, environmental assessment, and charging infrastructure

<table><tr><td>Research Stream</td><td>Key Contributions</td><td>Relevance To This Study</td></tr><tr><td>Queuing &amp; traffic flow at intersections</td><td>Lawson et al. (1997): input-output diagrams for queue dynamics at bottlenecks and saturation headway-based modeling</td><td>Foundation for dwell time calculation and segment-level trajectory near signals</td></tr><tr><td>Cross-resolution traffic-emission integration</td><td>Zhou et al. (2015): mesoscopic simulator with emission modelZhang and Qian (2023): large-scale networksJiang et al. (2017): urban expressways</td><td>Segment-level speed profiles serve as shared inputs for emission estimation and energy transfer calculation</td></tr><tr><td>Car-following &amp; emission estimation</td><td>Meng et al. (2021): Newell&#x27;s model with stochastic parameters for emissionsDas and Tanvir (2024): physics-informed LSTM for emission-aware dynamics</td><td>Acceleration variability in stop-and-go traffic determines both emission rates and DWPT charging duration</td></tr><tr><td>Environmental exposure &amp; accessibility</td><td>Vallamsundar et al. (2016); Zhong et al. (2023): population exposureSong et al. (2017): green accessibility metrics</td><td>Spatial distribution of infrastructure determines network-wide environmental outcomes</td></tr><tr><td>Eco-routing &amp; emission-sensitive operations</td><td>Pahwa and Jaller (2024): freight eco-routingKöster et al. (2018): emission-sensitive deliveryPoulhès and Proulhac (2021): low emission zones</td><td>Strategic infrastructure deployment yields benefits beyond immediate locations</td></tr><tr><td>Multi-layer optimization &amp; computational methods</td><td>Zhou et al. (2025): flow-through tensor architectureChen et al. (2024): joint trajectory-signal optimizationWang et al. (2018): DTA review</td><td>Unified frameworks for coupling traffic, energy, and infrastructure decisions</td></tr></table>

The connection across these streams is direct: the same acceleration and deceleration patterns that drive instantaneous emission rates also determine energy consumption and—for DWPT—the duration vehicles spend over charging coils. Our model builds on this foundation by developing segment-level kinematic equations that capture speed profiles shaped by signal timing and queue discharge—generating trajectory data structures essential for both infrastructure optimization and subsequent environmental assessment.

This study addresses this gap by developing a novel MIP model to optimize the location of DWPT infrastructure for urban EV mobility. Our work makes three primary contributions.

First, we introduce the concept of “Infinite Drive” as a system-level design principle for urban charging infrastructure. Rather than representing a deterministic guarantee at the individual vehicle level, Infinite Drive describes a planning objective under which urban EV operations are structurally energysufficient over repeated trip-making cycles, without reliance on any plug-in charging. Specifically, we solve for the number, locations, and lengths of DWPT segments that minimize the total installed DWPT length required to sustain such operations. While this goal may seem ambitious now, it is motivated by both medium- and long-term applications. In the medium term, it is especially valuable for drivers who cannot charge at home or work. In fact, DWPT offers the ultimate in convenience for the entire driving population. In the long-term, it leverages the advantage of continuously-on charging and aligns with future scenarios of autonomous-driving societies. In such a future, autonomous vehicles (AVs) could operate continuously, potentially serving logistics needs around the clock, maximizing asset utilization. This makes DWPT particularly valuable, as it allows for continuous charging without the need for AVs to pause their operation.

Second, the proposed modeling framework explicitly incorporates signalized intersection dynamics by accounting for EV acceleration, deceleration, and stopping behavior induced by traffic signal patterns. As vehicles approach signalized intersections, they tend to slow down or stop, increasing the time spent over embedded charging coils and thereby enhancing the amount of energy transferred via DWPT. These signal-induced acceleration–deceleration processes generate critical trade-offs, including whether DWPT should be placed on high-traffic or low-traffic roads and how far upstream from an intersection charging segments should extend. To reflect realistic queuing behavior and stop durations, the model integrates saturation headway-based queuing representations with segment-level kinematic equations. This formulation explicitly captures the interplay between traffic dynamics and charging opportunities, providing a nuanced and operationally realistic representation of DWPT effectiveness in urban networks.

Third, we demonstrate the practical applicability of the proposed framework through a detailed case study of Kawagoe City, a typical medium-sized Japanese city. Using calibrated traffic flow data, we determine the share of the urban road network required to achieve system-level energy sufficiency, examine the sensitivity of this requirement to alternative aggregation assumptions and safety margins, and validate the operational feasibility of the optimized deployment through Monte Carlo simulations of continuous trip chains under finite battery capacity. This analysis further enables explicit characterization of the infrastructure–battery tradeoff and provides a basis for joint decisions on DWPT location and onboard battery sizing. Together, these contributions position targeted DWPT deployment as a strategic component of sustainable urban EV infrastructure planning.

## 2. Conceptual framework

## 2.1. Concept of Infinite Drive

The core objective of this study is to formulate a suitable mathematical optimization model that determines the optimal location of DWPT infrastructure to enable “Infinite Drive”—a scenario in which EVs can meet all urban mobility needs without ever requiring plug-in charging.

Historically, EV infrastructure location models have focused on maximizing demand coverage under resource constraints, as exemplified by the flow-capturing location model (Hodgson, 1990), the flow-refueling location model (Kuby and Lim, 2005), and corridor-scale DWPT planning models (Honma et al., 2024). These models aim to intercept as much travel flow as possible given limited infrastructure budgets. Meanwhile, some station models—starting with Wang and Lin (2009)—take a stricter stance by requiring complete coverage and minimizing the number or cost of stations. These use the location set covering approach, which was first formulated by Toregas et al. (1971) for minimizing the number of facilities required for complete coverage of the urban population by essential emergency services such as fire stations and ambulance depots.

While the Infinite Drive concept is mathematically related to set-covering, it extends and reinterprets that logic for a dynamic, long-term application. Traditional set-covering for charging station networks asks, “Can all EV trips be completed?”—a question that is usually trivial in urban settings due to short trip lengths and access to home or workplace charging. In contrast, Infinite Drive asks: Can all EVs operate indefinitely in urban environments without plugging in at all? It imposes a perpetual energy sufficiency constraint, ensuring that every likely trip—now and in the future—is energetically feasible solely through en-route charging.

This distinction marks a shift from ensuring the feasibility of individual trips to enabling sustained operational cycles—making the Infinite Drive model notably more ambitious. For everyday urban mobility in all but the largest metropolitan areas, and for early adopters who typically have access to home or workplace charging, driving range is typically not a limiting factor. Infinite Drive becomes more important, however, at the megalopolitan scale, for residents of multifamily housing, and especially for high-utilization vehicles such as taxis, delivery fleets, ride-hailing services, and future autonomous vehicles (AVs) that operate continuously (Axsen and Pickrell-Barr, 2024; Kuby et al., 2025). For these users, Infinite Drive provides complete freedom from plug-in dependence, maximizing operational flexibility, energy access, and vehicle productivity.

In essence, while our model builds on the mathematical structure of the Set Covering Location Problem—locating the fewest facilities to cover all demands—it retools the concept for a dynamic wireless power network and a long-term urban mobility context. DWPT is not just a substitute for plug-in chargers, but a foundation for a fundamentally different energy delivery paradigm.

## 2.2. Critical tradeoff structures

A central consideration in optimizing DWPT location for urban mobility is the effect of vehicle acceleration and deceleration. These kinematic fluctuations significantly influence energy transfer rates and thus the effectiveness of DWPT infrastructure. It is almost intuitive to suggest that “DWPT should be placed near intersections,” since vehicles naturally slow down or stop in these areas, increasing dwell time over embedded coils. While this logic is compelling, the optimal location of DWPT is not as straightforward as it may appear. As illustrated in Fig. 2, even at intersections, nuanced tradeoffs arise that require rigorous modeling.

![](images/98d5d5f59f7d77f58378e3e5344422916b1e75cddb26a002eb1358d2d4ff59dd.jpg)  
(b) How long should the DWPT be?  
Fig. 2. Non-obvious tradeoffs at intersections

The first tradeoff (Fig. 2(a)) concerns which streets should be prioritized for DWPT installation. Roads with high traffic volumes appear attractive due to the large number of vehicles they serve, but these roads typically have longer green-light durations to manage throughput, meaning vehicles stop less

frequently and for shorter durations. This reduces the opportunities for efficient charging. On the other hand, lower-volume roads may feature longer red lights, allowing for longer stop times and potentially more effective energy transfer—but these roads see fewer vehicles overall. Thus, there is a tension between maximizing the number of charging opportunities and the quality of each opportunity.

The second tradeoff (Fig. 2(b)) involves how long the DWPT segment at a given intersection should be. Even though the common terminology for this technology is “Dynamic” WPT, it is most efficient to provide substantial power transfer when vehicles are stopped at red lights or at least slowing down for turns. Therefore, placing DWPT starting from the stop lines at traffic signals is ideal. However, determining how far back these systems should extend is challenging because the queue of EVs waiting at a red light gradually lengthens. Covering a longer segment would allow more EVs to benefit from the DWPT, but the queue will less frequently extend back that far and when it does the EVs will have shorter stop durations.

Together, these tradeoffs underscore the importance of detailed, segment-level modeling to determine where and how DWPT should be deployed. Any effective optimization framework must account for signal timing, traffic volumes, queuing behavior, and the kinematic states of vehicles to ensure that installations maximize real-world utility and efficiency, as shown in Fig. 3, where segmentlevel trajectory data serve as the shared foundation across all three domains.

![](images/0438f7e8998f06a368ee7f079255fcf89c253a45d925cffba715d102d0e3a370.jpg)  
Fig. 3. Conceptual framework linking car-following kinematics, energy/emission estimation, and DWPT infrastructure optimization.

## 3. Formulation

## 3.1. Model assumptions

The model is built on several key assumptions designed to balance realism and computational tractability. First, the road network, EV routes, and associated traffic volumes are treated as exogenous inputs. Each OD pair may have multiple pre-defined routes, but route choice behavior is fixed and does not respond to DWPT locations. This simplification not only reflects a scenario in which DWPT infrastructure is seamlessly integrated into everyday traffic patterns but also avoids the computational complexity of equilibrium traffic assignment while still capturing realistic route-level variations in flow volumes. The assumption is most appropriate when DWPT is sparsely deployed and not associated with explicit pricing incentives, so that the detour effect on route choice remains small; as DWPT density or pricing increases, equilibrium-based formulations such as those of Riemann et al. (2015) and Chen et al. (2016) provide the natural complementary perspective. The present framework thus occupies the fixedflow planning end of a methodological spectrum, prioritizing spatial resolution of signalized-intersection charging dynamics over equilibrium feedback effects.

Second, while signal-level vehicle trajectories are indeed critical to determining the optimal locations of DWPTs as noted earlier, the model accounts for these variations through probabilistic signal patterns. Because it is practically infeasible to collect highly granular data—such as the precise stopping position of each individual vehicle and the corresponding signal state at that moment—the model instead employs signal probability distributions to represent the likelihood of EVs stopping at traffic signals. For each intersection, a discrete set of patterns is used to capture the probabilities of stopping or passing through during a given signal cycle. These patterns encode the likelihood of an EV occupying a specific position in the queue, which directly affects its dwell time and, consequently, the amount of energy transferred via the DWPT.

Third, to reflect spatial granularity, the network is discretized into 7 m segments, approximately corresponding to the space occupied by a single stopped vehicle at a red light. This segmentation enables the model to explicitly capture acceleration, deceleration, and queue-position effects at intersections. It also allows DWPT location decisions to be expressed at the segment level.

Finally, the model’s objective is to minimize the total installed length of DWPT infrastructure required to achieve “Infinite Drive” within an urban network, subject to constraints on energy-sufficiency. Theoretically, these constraints could be imposed on each origin–destination route, but that would not only be computationally infeasible for larger applications but would also lead to substantial overbuilding of charging capacity, as we explain later. For now, suffice it to note here that the model aggregates routes into “energy balancing groups,” with a constraint requiring that the aggregate energy balance remains non-negative for each group at all times.

Relative to existing corridor-scale DWPT location models (Honma et al., 2024), the proposed framework introduces four key methodological extensions tailored to urban settings: (i) probabilistic signal patterns that capture stochastic stop-and-go behavior at signalized intersections; (ii) saturation headway-based queue modeling that links queue position to dwell time and charging duration; (iii) segment-level kinematic equations that explicitly model acceleration and deceleration around intersections; and (iv) operational validation through state-of-charge simulations with finite-capacity batteries (Section 6). Together, these extensions enable Infinite Drive analysis at fine spatial granularity in signal-controlled urban networks, where signal-induced variability dominates energy transfer opportunities.

The framework also positions itself within a broader methodological landscape for coupling traffic dynamics with energy costs. Alternative approaches, such as Eco-System Optimal (ECO-SO) flow assignment (Lu et al., 2016), recent dynamic user equilibrium formulations with non-monotonic emission costs (Tidswell et al., 2021), and the hyper-prism accessibility structure of Mahmoudi et al. (2019), endogenize driver responses to infrastructure conditions but become computationally prohibitive at the segment-level resolution required to capture signalized-intersection charging dynamics. The present framework therefore trades behavioral feedback for spatial fidelity, accepting fixed route flows in exchange for explicit segment-level modeling of signal-induced acceleration, deceleration, and queueposition-dependent dwell time.

## 3.2. Mathematical optimization problem

All notations for the model are defined below:

Indices:

?? Index of small (7 m) segments that divide road networks

?? Index of signal patterns, including vehicle stop positions at red lights

?? Index of routes

$k$ Index of energy balancing groups (mutually exclusive route subsets)

Sets:

?? Set of small segments that divide road networks

$I _ { r }$ Set of small segments that divide road networks for route <sub>??</sub>

$S$ Set of signal patterns, including vehicle stop positions at red lights

$R$ Set of all routes

$R _ { k }$ Set of routes in energy balancing group <sub>??</sub>

$K$ Set of energy balancing groups

Parameters:

$b _ { r }$ Expected energy balance for route <sub>??</sub> (kWh)

$c _ { i s } ^ { r }$ Required electric energy for route <sub>??</sub> to pass through segment <sub>??</sub> given signal pattern <sub>??</sub> (kWh)

$d _ { i }$ Length of segment <sub>??</sub> (m)

$f _ { r }$ Flow volume of route

$g _ { i s } ^ { r }$ Electric energy transfer for route <sub>??</sub> from DWPT at segment <sub>??</sub> given signal pattern <sub>??</sub> (kWh)

$p _ { i s } ^ { r }$ Probability of encountering signal pattern <sub>??</sub> at segment <sub>??</sub> (dimensionless)

$\varphi$ Safety coefficient for energy balance

Decision Variables:

$z _ { i }$ 1 if DWPT is installed on segment <sub>??</sub>, 0 otherwise

As outlined above, the model minimizes the length of DWPT installation necessary to enable all EVs to complete all their routes to achieve Infinite Drive within an urban area. Note that there is only one type of decision variab $\mathrm { l e } , z _ { i } ,$ representing whether a 7-m DWPT coil is embedded in the roadway on segment i or not. The second type of binary decision variable typically found in charging station location models based on the concept of max-covering—representing whether an OD pair is covered or not—is not needed in this formulation because all routes must be covered. In addition, no length variable is needed because the roads are divided into individual segments i. No route-used variable is needed because routes are given. Finally, no variables are included to explicitly keep track of the battery state-of-charge (SOC) on each segment of each route, because the model imposes an expected-value energy-balance constraint at the route and group level rather than tracking deterministic SOC trajectories—a formulation consistent with the probabilistic signal-pattern representation. Finite-battery operational feasibility is validated separately through the SOC simulations in Section 6.

The mathematical optimization problem proposed is as follows:

Min. $\textstyle \sum _ { i \in I } d _ { i } z _ { i }$

(1)

subject to:

$$
\sum_ {i \in I _ {r}} \sum_ {s \in S} p _ {i s} ^ {r} (g _ {i s} ^ {r} z _ {i} - \varphi c _ {i s} ^ {r}) = b _ {r}
$$

$$
\forall r \in R\tag{2}
$$

$$
\sum_ {r \in R _ {k}} f _ {r} b _ {r} \geq 0
$$

$$
\forall k \in K\tag{3}
$$

$$
z _ {i} \in \{0, 1 \}
$$

$$
\forall i \in I\tag{4}
$$

Eq. 1 minimizes the total length of DWPT installation across the target area. Constraint Eq. 2 represents the relationship for calculating the energy balance during EV travel along a specific route <sub>??</sub> in the designated area. In this study, the signal patterns are assumed to be given probabilistically. The term $p _ { i s } ^ { r }$ accounts for the probability of each signal pattern, including the specific position where an EV stops at a red light. Changes in signal patterns and the stopping position affect the timing of acceleration and deceleration, which in turn influences the amount of energy $g _ { i s } ^ { r }$ that can be transferred from the DWPT and the electric energy consumption $c _ { i s } ^ { r }$ of the EV (obtained by integrating instantaneous power over time). To embed a safety margin directly into the energy balance, the consumption term is scaled by a safety coefficient $\varphi \left( > 1 \right)$ , so that the pattern-specific balance on each segment is given by the transferable DWPT energy $g _ { i s } ^ { r } z _ { i }$ minus the scaled consumption <sub>φ</sub> $c _ { i s } ^ { r } ;$ the role of <sub>φ</sub> is discussed below. Consequently, the expected energy balance can be calculated by multiplying the probability of each signal pattern <sub>??</sub> by the corresponding energy balance for that pattern and then summing these products across all road segments used by route .

Constraint Eq. 3 addresses the energy balance condition necessary for achieving Infinite Drive, introducing two key concepts. The first concept is the "energy balancing group" k, which groups several routes $r \in R _ { k }$ . While it is possible to impose a constraint ensuring a positive energy balance for each individual route <sub>??</sub>, this approach is clearly an overestimate. In reality, EVs traverse various routes among different origins and destinations, and it suffices to ensure a positive overall energy balance as a whole. Thus, on the left-hand side, we calculate the expected total energy balance for each group <sub>??</sub> and constrain it to be non-negative. Flows $f _ { r }$ are used as weights to represent the relative frequency of route usage within a group and can be interpreted as probability weights for a representative vehicle’s repeated tripmaking cycle. The aggregation does not imply physical energy sharing across vehicles; rather, it captures expected energy balance over time for an individual vehicle following typical urban travel patterns. The concept of the groups will be discussed in more detail in the next sub-section. The second key concept is the safety coefficient $\varphi _ { : }$ , which enters the route-level energy balance in Eq. 2 by scaling the consumption term. Requiring the expected DWPT energy gain to exceed φ times the expected consumption—rather than merely to match it—imposes a deterministic safety margin (for example, $\varphi = 1 . 1$ requires a 10% energy surplus), which is precisely why a larger <sub>φ</sub> calls for more DWPT infrastructure to satisfy the constraint. While the formulation does not explicitly track battery state-of-charge dynamics, the safety coefficient $\varphi$ provides a deterministic-equivalent of a chance constraint, imposing a tolerance margin on the group-level expected-energy-balance guarantee. This formulation accommodates several practical sources of variability that are difficult to capture explicitly within the optimization, including heterogeneity in onboard battery capacity across the EV fleet by model and vintage, variability in DWPT coil spacing and energy transfer efficiency, road grade effects (hills) not captured by the segmentaveraged kinematic equations, and ambient temperature effects on motor and battery performance. By choosing $\varphi > 1$ as a standard engineering convention, planners introduce a deterministic safety margin against these stochastic and fleet-level uncertainties without expanding the dimensionality of the optimization problem. Finally, Constraint Eq. 4 requires the variables related to the installation of DWPT to be binary.

This mathematical optimization problem, while appearing straightforward, is not easily solvable due to several complexities. Firstly, the probabilistic patterns related to signals $p _ { i s } ^ { r }$ , the amount of energy that can be transferred from DWPT $g _ { i s } ^ { r }$ , and the energy consumption by $\mathrm { E V s } c _ { i s } ^ { r }$ must be accurately prepared. Furthermore, the problem involves partitioning the entire road network in the target area— comprising a total length of approximately 150 km, as considered in our numerical example—into $\mathrm { 7 } { \cdot } \mathrm { m }$ segments, each corresponding to the space occupied by a single vehicle. This segmentation leads to over 21,000 binary variables $z _ { i } ,$ making the optimization problem computationally demanding.

## 3.3. Definition of “Energy Balancing Groups”

This section provides further explanation regarding the aggregation of selected routes into "energy balancing groups," for which the expected energy balance must be non-negative. While it is theoretically possible to constrain each individual route <sub>??</sub> to have a positive expected energy balance, this approach would clearly be an overestimate of DWPT needed, and thus economically inefficient. For instance, if the city has numerous very short trips, such as visits to a nearby convenience store, enforcing a positive energy balance for each trip would necessitate the installation of DWPT almost everywhere in the city. This requirement is impractical, given that EVs are equipped with batteries of sufficient capacity to manage such short trips without constant recharging from DWPT.

Instead, we propose using a more natural aggregation scheme: routes aggregated by each location assumed to be an EV driver's home (Fig. 4(a)). This means collecting all routes that either start or end at any home locations, including multifamily dwellings. The intention behind this approach is to consider a scenario where drivers make multiple trips to various destinations and return home, ensuring that the overall expected energy balance is met across these trips. By achieving a positive energy balance for the group as a whole, we ensure that the concept of Infinite Drive is maintained. The specific spatial granularity of the "home location" can be tailored to the resolution of the available demand data— possible aggregation levels include individual buildings or households, road links serving as virtual origins, traffic analysis zones (TAZs), or census-defined neighborhoods. Finer aggregation tends to require more extensive DWPT deployment, while coarser aggregation permits greater energy balancing across routes (cf. Section 5.3). This approach allows for a more practical and realistic deployment of DWPT, focusing on aggregate rather than individual route-level energy balances.

Of course, alternative aggregation schemes can also be considered. For example, many optimal EV infrastructure location models allocate demand based on Origin-Destination pairs. Correspondingly, one could aggregate routes based on these OD pairs, encompassing various paths between the same two locations (Fig. 4(b)). This approach would be a finer aggregation than the location-based approach, likely requiring more extensive DWPT installations to meet the energy balance criteria.

Conversely, one could consider an even coarser aggregation, where it is sufficient for the entire target area's expected energy balance to be non-negative. This broader scope of aggregation applies when individual specific points or OD pairs do not need to maintain an energy balance independently. Instead, the overall expected energy balance across the entire region would suffice (Fig. 4(c)). This scenario represents a pattern that would require the least amount of DWPT installation, assuming an integrated approach to shared vehicle usage that optimizes energy distribution and minimizes infrastructure needs.

These three aggregation schemes naturally correspond to different vehicle classes encountered in real-world fleet operations. Anchor-location aggregation (Fig. 4(a)) is well-suited to privately owned EVs whose trips anchor at home, as well as to freight and delivery vehicles whose trip chains begin and end at centralized depots. OD-pair aggregation (Fig. 4(b)) suits operations dominated by specific recurring trip patterns. Region-based aggregation (Fig. 4(c)) is most appropriate for shared mobility services and future autonomous vehicles whose operations may span the network without a fixed anchor. Because the energy-balancing group set <sub>??</sub> is defined as a generic partition of the route set <sub>??</sub>, the proposed framework accommodates all such vehicle classes—and arbitrary mixtures thereof—without modifying the underlying optimization formulation.

![](images/7950c12411912f1d171ada8a8f8d427a8783ed9bb48159a89175954b13bb9223.jpg)  
(a) aggregated by each location (home)

![](images/52b61656fca066aeff098f762bbe1f67cc39a1f14b07751a6c960cc8f9de194f.jpg)  
(b) aggregated by each OD pair

![](images/b515f56a87f5e6ce97e49f97eedcd8e615828d1a0ef289cad6cfe644d70a492d.jpg)  
(c) aggregated across the entire region  
Fig. 4. Three approaches to grouping routes into “energy balancing groups.”

## 3.4. Considerations of signal patterns s

Next, we consider the probability distribution of signal patterns, because capturing or simulating stops at traffic signals and the precise timing of acceleration and braking is challenging. Therefore, we supplement the exogenous routing data through probabilistic calculations for detailed aspects such as redlight cycles and the vehicle position in the queue.

In this study, the discrete index encodes the joint state of (i) whether the signal is green or red, and (ii) the queue position of the vehicle if the signal is red. Specifically, $s \in \{ 0 , 1 , 2 , \ldots , L _ { \mathrm { r e d } } \}$ , where $s = 0$ denotes a green-light passage, and $s = 1 , 2 , \ldots , L _ { \mathrm { r e d } }$ denotes a red-light stop at the -th position in the queue. This single-variable encoding captures both the signal phase and the queue dynamics in a unified formulation. The probabilities of encountering green and red lights can be expressed as follows:

Probability of red light:

$$
q _ {\mathrm{red}}\tag{5}
$$

Probability of green light:

$$
1 - q _ {\mathrm{red}}\tag{6}
$$

Once the probabilities of signal patterns are determined, we estimate how many vehicles queue up during a red-light cycle:

Time of red light:

$$
W _ {\mathrm{red}} = c \times q _ {\mathrm{red}}\tag{7}
$$

Queue length during red light:

$$
L _ {\mathrm{red}} = \frac {c}{3 6 0 0} \times q _ {\mathrm{red}} \times f\tag{8}
$$

Here, <sub>??</sub> represents the number of EVs per hour per link, and c is the cycle time, calibrated per intersection from observed signal data. For roads with multiple lanes, the value of <sub>??</sub> is divided by the number of lanes.

To more accurately reflect queuing behavior, we incorporate the concept of saturation headway (Bonneson, 1992; Kimber et al., 1986), the average time interval between successive vehicles as they depart once the signal turns green. The probability of being the <sub>??</sub>-th vehicle in the queue and the corresponding waiting time are given by:

Probability of being the <sub>??</sub>-th vehicle:

(9)

Waiting time of being the <sub>??</sub>-th vehicle:

$$
\frac {W _ {\mathrm{red}}}{L _ {\mathrm{red}}} \times (L _ {\mathrm{red}} - s + 1) + h _ {s} \times (s - 1)\tag{10}
$$

where $h _ { s }$ is the saturation headway (typically 2.0–2.5 seconds). This formulation better captures realworld departure delays, particularly for vehicles farther back in the queue. This uniform approximation reflects the assumption that vehicle arrival times are evenly distributed within the signal cycle, which is a common and tractable representation when detailed arrival profiles are unavailable. In coordinated-signal corridors where platoon arrivals predominate, $p _ { i s } ^ { r }$ would deviate from uniform, and the formulation can be readily recalibrated with the corresponding non-uniform arrival profile to preserve transferability to such settings.

Given that the domain of $s \in \{ 0 , 1 , 2 , \ldots , L _ { \mathrm { r e d } } \}$ , the probability distribution $p _ { i s } ^ { r }$ for signal pattern <sub>??</sub> is defined as:

$$
p _ {i s} ^ {r} = \left\{ \begin{array}{c c} 1 - q _ {\mathrm{red}} & \text {if} s = 0 (\text {Green light}) \\ \frac {q _ {\mathrm{red}}}{L _ {\mathrm{red}}} & \text {if} s = 1, 2, \ldots , L _ {\mathrm{red}} (\text {Red light}) \end{array} \right.\tag{11}
$$

Additionally, the specific waiting time $t _ { i s } ^ { r , \mathrm { r e d } }$ for stopping at a traffic signal on segment <sub>??</sub> during a red light is:

$$
t _ {i s} ^ {r, \text {red}} = \left\{ \begin{array}{c c} \frac {W _ {\text {red}}}{L _ {\text {red}}} \times (L _ {\text {red}} - s + 1) + h _ {s} \times (s - 1) & \text {if the stop occurs on segment i} \\ 0 & \text {otherwise} \end{array} \right.\tag{12}
$$

This model does not yet account for dedicated lanes for right or left turns. Consequently, the values of $\dot { p } _ { i s } ^ { r }$ and $t _ { i s } ^ { r , \mathrm { r e d } }$ do not depend on route . For future modeling work, however, where such dependencies may be considered, the notation <sub>??</sub> has been retained in the formulation for generality. Additionally, while the notation does not explicitly include indices for specific intersections or times of day, these factors were individually set per intersection and time slot in this study.

## 3.5. Considerations of acceleration and braking

The specific behaviors of acceleration and braking vary based on whether a vehicle stops at a traffic signal and, if so, its position in the queue. How these variations influence both energy consumption and the amount of time an EV is positioned over DWPT depends on two key elements. The first element is the signal pattern and the associated stopping position. If the light is green, vehicles can pass through the intersection without stopping (Fig. 5(a)). However, if the light is red, vehicles must stop at an appropriate position before the intersection and then accelerate again when the light changes (Fig. 5(b)).

The second element concerns the vehicle's maneuver at the intersection—whether it goes straight or turns left or right. Since the routes are exogenously determined in this study, we can specify the maneuvers for each vehicle. While vehicles going straight can generally continue without stopping (assuming they have a green light), those making left or right turns must slow significantly or completely stop at the intersection, even if they have a green light, as a safety precaution (Fig. 5(c,d)). The conservative assumption to make here is that a full stop is necessary at each turn and the stop is assumed to occur in the central part of the intersection where DWPT is not present. Thus the stop time does not

contribute to energy transfer calculations. For vehicles proceeding straight, normal acceleration and braking are considered based on the speed limits of each link.

![](images/4f359869c53edbd1e1b9e8f8051f32654a5e88269a07a651b47274b7bbc4d8b6.jpg)  
(a) green signal, straight

![](images/7a5f64a4539f0307bd91d3e36723939b37172258d3af7951def03c2c025ba239.jpg)  
(b) red signal, straight

![](images/470dda4227f6079cffe3adce37dbed1499a67e85e04f386d1fbcc668291f0d3a.jpg)  
(c) green signal, left/right

![](images/e9e1b25d270c93181212ad1c11babfdf03f1e6e6c67babcc51d5e4a99e9aa550.jpg)  
(d) red signal, left/right  
Fig. 5. Mathematical scenario to reflect signal pattern and vehicle’s maneuver

It is crucial to note that these specific acceleration and braking behaviors are dictated by the probability distribution of signal pattern <sub>??</sub>, as these probabilities will govern the vehicle's interactions with the signal and, consequently, their energy consumption and the duration over the DWPT.

The following equations determine the position, speed, and transit time on each segment due to acceleration and braking. Starting from position $x = 0$ and velocity $v = 0$ with constant acceleration $a ,$ the relationships between position <sub>??</sub>, velocity $v ,$ and time <sub>??</sub> are given by:

$$
\begin{array}{l} \text {[When time t is given:]} \\ v (t) = a t \\ x (t) = \frac {1}{2} a t ^ {2} \end{array}\tag{13}
$$

(14)

[When velocity <sub>??</sub> is given:]

$$
t (v) = \frac {v}{a}
$$

$$
x (v) = \frac {v ^ {2}}{2 a}\tag{15}
$$

(16)

[When position <sub>??</sub> is given:]

$$
\begin{array}{l} {t (x) = \sqrt {\frac {2 x}{a}}} \\ {v (x) = \sqrt {2 a x}} \end{array}\tag{17}
$$

(18)

These equations apply to both acceleration and deceleration scenarios. For example, if a vehicle is traveling at a constant speed $v _ { c o n s t }$ and needs to stop at position $x _ { \tt s t o p } .$ , we can calculate the required braking distance using Eq. 16 as $x _ { \mathrm { s t o p } } - x ( v _ { c o n s t } )$ . The velocity reduction from $x _ { \mathrm { s t o p } } - x ( v _ { c o n s t } )$ to $x _ { s \tt t o p }$ can then be determined using $\mathbf { E q } .$ 18 for <sub>??(??)</sub>. Similarly, if a vehicle decelerates from a constant speed $v _ { c o n s t } ^ { 1 }$ to $v _ { c o n s t } ^ { 2 }$ at position $x _ { 2 }$ , the transition can be calculated as $x _ { 2 } - x ( v _ { c o n s t } ^ { 1 } ) + x ( v _ { c o n s t } ^ { 2 } )$

The critical component, the transit time $t _ { i s } ^ { r , \mathrm { m o v e } }$ over each segment <sub>??</sub>, with segment boundaries $\left[ x _ { i } ^ { 1 } , x _ { i } ^ { 2 } \right]$ , can be obtained by integrating the reciprocal of the speed over the segment:

$$
t _ {i s} ^ {r, \mathrm{move}} = \int_ {x _ {i} ^ {1}} ^ {x _ {i} ^ {2}} \frac {1}{v (x | s)} \mathrm{d} x\tag{19}
$$

Combining the transit time during movement $t _ { i } ^ { r , \mathrm { m o v e } } ( s )$ with the waiting time at red lights $t _ { i s } ^ { r , \mathrm { r e d } }$ from Eq. 12, we get:

$$
t _ {i s} ^ {r} = t _ {i s} ^ {r, \mathrm{move}} + t _ {i s} ^ {r, \mathrm{red}}\tag{20}
$$

Finally, let $g _ { \mathrm { t r a n s } }$ denote the effective DWPT transfer power (kW) (set to 18.7 kW in this study; see Section 4.1). Then the energy received from DWPT during the segment can be calculated as:

$$
g _ {i s} ^ {r} = g _ {\mathrm{trans}} * t _ {i s} ^ {r}\tag{21}
$$

Note that the transit time is converted from seconds to hours when computing energy in kWh. These equations collectively provide the basis for determining the dynamic power transfer and consumption behaviors of EVs as they move through segments with varying speeds and stopping patterns.

![](images/f9411b3050ce26f1c6eb9173ccfba314aaf17318bdada46b472c86f877b90c2a.jpg)

## Fig. 6. Space–time diagram of vehicle trajectories near intersections

To supplement our modeling framework, Fig. 6 presents a space–time diagram of vehicle trajectories near intersections. This figure visually demonstrates that dwell times depend on queue position, thereby supporting the formulation used in this study. It also illustrates the deceleration and acceleration phases that arise through interactions with traffic signals. The pink-shaded area represents stoppage due to red lights, while the yellow-shaded area indicates travel on low-speed segments or deceleration when approaching a red light. The green-shaded area reflects acceleration after the signal turns green.

By depicting these speed fluctuations and differences in stop durations across signal phases, the diagram shows that the vehicle dynamics considered in this study are not exogenously imposed but are endogenously determined by signal control, queue position, and intersection behavior. Moreover, by embedding speed variation into segment-level kinematic equations and signal-pattern probabilities, the figure helps address potential concerns regarding the realism of our velocity modeling. Although our model does not implement a fully time-dependent traffic assignment, the deterministic segment-level kinematics—reflecting the impacts of signals and turning movements—appropriately capture the speed variations necessary for this study. This approach offers an effective balance between computational efficiency and behavioral realism for urban-scale DWPT planning.

## 4. Data

## 4.1. Power consumption equation

First, Eq. 22 computes the power consumption in kilowatts (kW), based on the previous studies (Fiori et al., 2016; The Engineering ToolBox, 2004; Wu et al., 2015):

$$
P (v, a, \theta) = \frac {1}{\eta} v \left(m a + m g \cos \theta f _ {r l} + \frac {1}{2} \rho A _ {f} C _ {D} v ^ {2} + m g \sin \theta\right)\tag{22}
$$

<sub>??</sub> represents the EV's speed in meters per second (m/s), <sub>??</sub> the acceleration in meters per second squared $( \mathbf { m } / \mathbf { s } ^ { 2 } )$ , and <sub>??</sub> the road gradient in degrees (°) (Table 2). Regarding the effective DWPT transfer power $g _ { \mathrm { t r a n s } }$ from DWPT, existing studies suggest an output of 20–25 kW (Hata et al., 2019). In addition, when a vehicle is stationary, a wireless power transfer, such as that of WPT4, has been proposed (SAE J2954 standard, 2020). We assume a transfer capacity of 22 kW with an efficiency of 85%, resulting in an effective output of 18.7 kW.

Table 2  
Parameters for calculating motor power

<table><tr><td>Parameters</td><td>Values</td></tr><tr><td>Efficiency of electric motor η (%)</td><td>90</td></tr><tr><td>Vehicle weight (including driver) m (kg)</td><td>1,640</td></tr><tr><td>Gravitational acceleration g (m/s2)</td><td>9.8066</td></tr><tr><td>Rolling resistance coefficient  $f_{rl}$ </td><td>0.015</td></tr><tr><td>Air mass density ρ (kg/m3)</td><td>1.2256</td></tr><tr><td>Frontal area of vehicle  $A_f$  (m2)</td><td>2.34</td></tr><tr><td>Aerodynamic drag coefficient  $C_D$ </td><td>0.32</td></tr></table>

## 4.2. Traffic flow data for Kawagoe City

The case study focuses on Kawagoe City, a typical medium-sized urban area in Saitama Prefecture, Japan. The traffic flow data used in this study were developed through detailed traffic simulations conducted as part of joint research with Kawagoe City. The dataset integrates road geometry, travel demand, and signal operations to represent realistic urban traffic conditions.

Intersection topology and lane configurations were obtained from OpenStreetMap, while origin– destination (OD) travel patterns (Fig. 7(a)) were estimated using a combination of building-use data and local traffic volume surveys conducted by Kawagoe City. Traffic assignment was performed using a user equilibrium model, run separately for each hour from 7:00 to 19:00 on both weekdays and weekends. This procedure generated 57,249 distinct routes, each associated with a flow volume.

To reflect realistic operating conditions at signalized intersections, signal patterns were calibrated based on observed traffic volumes and empirical signal timing ratios provided by the city. These data were used to construct probabilistic signal pattern distributions for each route, capturing the likelihood of stopping and queue-position-dependent dwell times. In addition, free-flow link speeds were adjusted to match observed travel times from the equilibrium assignment, thereby accounting for delays induced by traffic signals.

The resulting traffic flow distribution is shown in Fig. 7(b). High traffic volumes are observed along the eastern bypass connecting to the highway interchange, while downtown streets—although also heavily used—are characterized by narrow widths and correspondingly lower operating speeds. Fig. 8 presents the distributions of trip distances and energy consumption across all routes. The average trip distance is 2,967 m, and the average energy consumption is 307.3 Wh, values that are consistent with empirical measurements reported for urban EV operations.

![](images/ab222719041bab3a1497e32272537c9eb4e7fe7df969fc3550b13dc3f76dfabd.jpg)  
(a) Distribution of OD pairs

![](images/37aaeae50440284d0dac5fff7ea75315d6ffa5eef62ec161d85eeb3eb3eeadfe.jpg)  
(b) Distribution of traffic volume  
Fig. 7. Traffic distribution in Kawagoe City

![](images/8f350bcb704006315a9d86b475f8f2b99666825b03b17d90154740a9f2e114e7.jpg)  
(a) Distribution of travel distance

![](images/020a69914ca745dcb6b711117210d220e96f1bf2e70221982286585cbd657e2c.jpg)  
(b) Distribution of energy consumption  
Fig. 8. Distributions of travel distance and energy consumption

## 5. Results

## 5.1. Problem size and computational environment

The Kawagoe road network covers a total length of 147,868 m. Discretizing the network into 7-m segments yields 21,124 binary decision variables $z _ { i } ,$ each indicating whether a DWPT coil is installed on a candidate segment. All segments along the network are treated as candidate locations, including midblock segments away from signalized intersections, so that the optimization itself confirms, rather than presupposes, where DWPT should be placed.

On the demand side, the dataset includes 57,249 routes obtained from hour-by-hour traffic assignment. Unless otherwise stated, these routes are aggregated into home-based groups (Section 3.3), which serve as the baseline definition of “groups” for the energy-balance constraints. Specifically, the Kawagoe OD data are generated at the road-link level: each link in the network acts as a virtual origin representing the residential population served by that segment, and all routes originating from a given link are grouped into one energy-balancing group. This link-level realization is finer than typical TAZ-based aggregation, while remaining coarser than individual building or household resolution. This combined scale is non-trivial for binary-location problems but remains tractable owing to the sparsity of the segment–route coupling, in which each variable $z _ { i }$ appears in the energy balance only of routes traversing segment <sub>??</sub>. The model was solved using Gurobi 12.0.3 on a PC equipped with eight 2.2 GHz cores and 32 GB RAM. For each run, we imposed a computation time limit of 7,200 s. The reported solutions correspond to the best solutions found within the time limit. Across the nine scenarios reported in Section 5.3, the solver-reported optimality gaps remain small (0–2.47%), indicating that the solutions are nearoptimal for the purpose of planning-level inference.

## 5.2. Baseline optimal location under home-based groups

Fig. 9 shows the optimized DWPT location for the baseline setting (home-based groups; safety coefficient <sub>?? =</sub> 1.1). The solution confirms that signalized intersections are the most effective urban locations for DWPT, where deceleration, stopping, and queue discharge increase the time vehicles spend over embedded coils. Under this baseline case, the model equips 56 intersections with a total DWPT length of 2,233 m, corresponding to 1.51% of the total road network.

Consistent with Japan’s left-hand traffic, many selected segments are placed on approach lanes that capture the dominant stopping and queuing behavior upstream of intersections. The deployment pattern is highly non-uniform across the city, reflecting heterogeneity in stop probability, queue formation, and signal timing. Even within the same corridor, the optimized solution allocates longer DWPT segments to approaches with higher red-encounter likelihood and longer expected dwell times, while assigning shorter segments where vehicles are more likely to pass during green phases. At Intersection A, for example, where a bypass road intersects with a major arterial, the installation length is relatively long due to the frequent occurrence of queues and extended stop durations. In contrast, other intersections along the same bypass, such as Intersection B, have shorter installations, reflecting longer green phases that reduce the likelihood of vehicles stopping. Downtown intersections, such as Intersection C, also receive DWPT installation because narrow street geometries, short block lengths, and recurrent congestion create conditions where vehicles spend more time idling and therefore present favorable opportunities for energy transfer.

![](images/94c104588330a870611ab4585c33a05786807b2e4554f1f5892e5491b4ec16e2.jpg)  
Fig. 9. Optimal location of DWPT in Kawagoe City (home-based groups; safety coefficient $\varphi =$ ??. ??<sup>)</sup>  
5.3. Sensitivity to aggregation schemes and safety coefficients

To assess robustness to planning assumptions, we conduct a two-dimensional sensitivity analysis along (i) the aggregation scheme used in the energy-balance constraints and (ii) the safety coefficient <sub>??</sub>. We consider three aggregation schemes: (a) entire region, (b) each home location (baseline), and (c) each OD pair, combined with three safety coefficient values <sub>?? =</sub> 1.1, 1.3, 1.5, resulting in nine scenarios.

Table 3 summarizes the required DWPT length, the corresponding share of the road network, and the number of equipped intersections. Two consistent patterns emerge. First, for a fixed aggregation scheme, increasing <sub>??</sub> increases the required DWPT length. For example, under the baseline home-based aggregation, the required DWPT length rises from 2,233 m at <sub>??</sub> <sub>=</sub> 1.1 to 3,346 m at <sub>??</sub> <sub>=</sub> 1.5 (1.51% to 2.26% of total road length). Similar increases are observed for the region-based and OD-pair-based settings.

Second, for a fixed safety coefficient, more granular aggregation leads to larger infrastructure requirements. At <sub>?? =</sub> 1.1, the required DWPT length increases from 1,596 m (entire region) to 2,233 m (home-based) and 2,744 m (OD pair). At <sub>?? =</sub> 1.5, it increases from 2,387 m (entire region) to 3,346 m (home-based) and 4,291 m (OD pair). This reflects the intuitive fact that stricter, more granular energybalance requirements reduce the degree to which surplus energy on some routes can offset deficits on others.

Importantly, even under the most demanding tested condition—OD-pair aggregation with 1.5—the required DWPT length remains 4,291 m, corresponding to 2.90% of the urban road network. This indicates that the infrastructure requirement remains modest even when adopting conservative buffers and stringent aggregation assumptions.

Table 3  
Required DWPT length under different aggregation schemes and safety coefficients

<table><tr><td>Scheme</td><td> $\varphi$ </td><td>DWPT length (m)</td><td>Road share (%)</td><td># Intersections</td><td>Optimality gap (%)</td></tr><tr><td>Region</td><td>1.1</td><td>1,596</td><td>1.08</td><td>51</td><td>0.00</td></tr><tr><td>Region</td><td>1.3</td><td>1,981</td><td>1.34</td><td>55</td><td>0.00</td></tr><tr><td>Region</td><td>1.5</td><td>2,387</td><td>1.61</td><td>59</td><td>0.00</td></tr><tr><td>Home (base)</td><td>1.1</td><td>2,233</td><td>1.51</td><td>56</td><td>1.54</td></tr><tr><td>Home (base)</td><td>1.3</td><td>2,744</td><td>1.86</td><td>62</td><td>0.97</td></tr><tr><td>Home (base)</td><td>1.5</td><td>3,346</td><td>2.26</td><td>59</td><td>1.22</td></tr><tr><td>OD pair</td><td>1.1</td><td>2,744</td><td>1.86</td><td>63</td><td>2.47</td></tr><tr><td>OD pair</td><td>1.3</td><td>3,465</td><td>2.34</td><td>65</td><td>2.01</td></tr><tr><td>OD pair</td><td>1.5</td><td>4,291</td><td>2.90</td><td>68</td><td>1.89</td></tr></table>

(Note: <sub>??</sub> denotes the safety coefficient; DWPT length is the total installed length across the network.)

## 6. Operational validation and infrastructure–battery tradeoffs

The optimization model guarantees a non-negative expected energy balance at the group level but does not explicitly track the underlying battery state-of-charge (SOC) dynamics, since doing so would substantially increase computational complexity on the urban scale. With finite onboard battery capacity, however, short-term stochastic fluctuations may still lead to temporary depletion events even when the expected balance is satisfied. This section presents a simulation-based validation that addresses this gap by examining both the feasibility of the Infinite Drive concept under realistic SOC dynamics and the tradeoff between DWPT infrastructure investment and required onboard battery capacity.

## 6.1. Simulation design and evaluation metric

We perform Monte Carlo simulations of continuous urban operations by generating long trip chains, each comprising 100 round trips between a randomly selected home location and destinations drawn from the calibrated urban travel demand. These trip chains, with cumulative travel distance

averaging approximately 600 km per run and reaching up to approximately 800 km, represent extended, high-utilization operations—such as taxi services, delivery fleets, ride-hailing vehicles, or future shared autonomous vehicles—that far exceed typical daily urban travel and therefore provide a conservative stress test for the Infinite Drive concept.

Each simulation run begins with the EV initially fully charged at its home location, after which no plug-in recharging occurs throughout the entire 100-round-trip sequence. Routes and departure times for each round trip are sampled from the calibrated demand, determining the signal-pattern probabilities along each path. Along each road segment, the SOC is updated by subtracting the segment-level energy consumption and adding energy transferred via DWPT when and where the segment is equipped and the corresponding signal pattern allows charging.

A run is defined as successful if the SOC never reaches zero throughout the entire 100-round-trip sequence. For each scenario, 1,000 independent simulation runs are conducted, and the success rate is calculated as the proportion of runs completed without battery depletion. Empirical success rates are reported together with 95% Clopper–Pearson exact confidence intervals (Clopper and Pearson, 1934), which remain well-defined at boundary cases such as 100% empirical success, where asymptotic approximations would degenerate.

## 6.2. Optimized versus uniform DWPT locations

We first compare the optimized DWPT location derived from the MIP model (home-based groups; safety coefficient <sub>??</sub> <sub>=</sub> 1.1; total DWPT length = 2,233 m) with a simple uniform heuristic that installs 7 m of DWPT at every candidate signalized approach (total DWPT length = 3,584 m). Although the uniform heuristic requires substantially more DWPT infrastructure, it does not account for heterogeneity in queue formation, dwell-time distributions, or signal timing.

As shown in Fig. 10, for a 12 kWh battery, the optimized deployment achieves a success rate of 100.0% (95% CI: 99.63%–100.0%), whereas the uniform 7-m deployment achieves only 93.5% (91.79%–94.95%). For a 6 kWh battery, the optimized deployment achieves a success rate of 97.2% (95.98%–98.13%), compared to 84.1% (81.68%–86.31%) under the uniform deployment. The nonoverlapping 95% confidence intervals between optimized and uniform deployments confirm that these differences are statistically significant under both battery configurations. Representative SOC trajectories are shown in Fig. 10, illustrating that failures under the uniform deployment occur more frequently and at earlier stages of continuous operation, despite the larger total DWPT length.

These results indicate that DWPT effectiveness depends not only on total installed length, but also on precise spatial alignment with signal-induced stopping behavior. Uniform deployment fails to exploit intersections where vehicles experience long dwell times and favorable queue positions, leading to missed charging opportunities even when more infrastructure is installed.

![](images/55da72c87da731834d6da81d5451f636711a17549d12c7065ad784ae1f1a1355.jpg)  
(a) Optimized location, Battery Capacity = 12 kWh ( Completion rate =100.0% )

![](images/c5ddd4d10f9e5e295ed00cc1e16d7641669ddb146024d72d5e09bd3c25dfcd34.jpg)  
(b) Uniform location, Battery Capacity = 12 kWh ( Completion rate =93.5% )

![](images/3e755c9e69fe14681e6e8ac287941673e1993c609a589f0a33a39c81de32566a.jpg)  
(c) Optimized location, Battery Capacity = 6 kWh ( Completion rate =97.2% )

![](images/548d787ee719df78ed14ce2424c23904041ea9ec8e73bc95517137cf297572d9.jpg)  
(d) Uniform location, Battery Capacity = 6 kWh ( Completion rate =84.1% )  
Fig. 10. Comparison of battery state-of-charge (SOC) trajectories under optimized and uniform DWPT locations

## 6.3. Infrastructure–battery tradeoffs under optimized deployment

We next examine the tradeoff between DWPT infrastructure length and EV battery capacity under optimized deployment strategies. Fig. 11(a) summarizes success rates as a function of battery capacity for optimized DWPT deployments under safety coefficients of 1.1, 1.3, and 1.5, corresponding to total DWPT lengths of 2,233 m, 2,744 m, and 3,346 m, respectively.

As the safety coefficient increases, additional DWPT segments are installed to provide greater robustness against stochastic charging losses and finite battery constraints. This increase in infrastructure investment systematically improves reliability for a given battery size. Near-perfect performance can be achieved with progressively smaller batteries: approximately 12 kWh for $\varphi = 1 . 1$ , 8 kWh for $\varphi = 1 . 3$ and 6 kWh for $\varphi = 1 . 5$ . These results confirm that the safety coefficient in the optimization model provides a meaningful and interpretable mechanism for trading off infrastructure investment against vehicle-side energy storage.

For comparison, Fig. 11(b) presents analogous success-rate curves for uniform DWPT deployments of 7 m, 14 m, and 21 m per candidate approach. Achieving reliability comparable to the optimized solutions requires substantially larger DWPT investments—up to more than 7.3% of the total road length—highlighting the inefficiency of uniform location strategies. Importantly, the observed tradeoff between DWPT length and battery capacity represents an environmental outcome in itself. By enabling reliable operations with substantially smaller onboard batteries and a limited electrified road

footprint, the optimized solutions demonstrate system-level potential for reducing vehicle mass, material demand, and infrastructure-related impacts.

To illustrate the potential scale of the vehicle-side environmental co-benefit, we draw on the lifecycle assessment literature on Li-ion EV battery manufacturing. Recent cradle-to-gate (i.e., manufacturing-stage) emission factors for commercial-scale NMC-chemistry Li-ion batteries are manufacturing-stage) emission factors for commercial-scale NMC-chemistry Li-ion batteries are

estimated at 61–106 kg CO<sub>2</sub>e/kWh depending on the electricity mix used in cell production (Emilsson and Dahllöf, 2019); earlier estimates of 140–172 kg CO<sub>2</sub>e/kWh reflected less efficient cell manufacturing and higher-carbon electricity inputs (Ellingsen et al., 2014; Kim et al., 2016). Using the most recent range, the 12 kWh battery sufficient for Infinite Drive operation at $\varphi = 1 .$ 1 corresponds to approximately 1.7–3.0 tonnes $\mathrm { C O } _ { 2 } \mathrm { e }$ of avoided battery manufacturing per vehicle relative to a conventional 40 kWh urban EV. The magnitude of this vehicle-side benefit is broadly consistent with the DWPT life-cycle assessment of Bi et al. (2019), who showed that selective electrification of roughly 3% of road lane-miles can enable substantial battery downsizing while reducing fleet-wide greenhouse gas emissions. This estimate does not include the embodied emissions of the DWPT road infrastructure itself, which remain poorly characterized in the literature and lie beyond the scope of this study; a full comparative life-cycle assessment is therefore left for future work. The figure should be interpreted as an upper bound on the vehicle-side co-benefit rather than as a net environmental accounting. Joint optimization of charger location and battery size in electric-bus systems has also been studied as a precursor to the present codesign framing (Liu et al., 2017), while Liu et al. (2024) recently extended this co-design perspective to passenger EVs by jointly configuring DWPT facilities and battery capacity in a road-network setting. Comparable life-cycle analyses of DWPT in electric bus transit systems have similarly highlighted that infrastructure–battery tradeoffs and environmental performance depend critically on operational context, including service frequency and infrastructure sharing across multiple lines (Pei et al., 2024).

![](images/ef7bc33d1af0cad356596c75ae580e96a8f6e2e12c4fad6d642afea6832228e2.jpg)  
(a) Optimized DWPT locations

![](images/b7fd8e2eef59bbfcf900478e906b4ded5955ac6de1d8c5c5bd998a77f187c3e2.jpg)  
(b) Uniform DWPT locations  
Fig. 11. Tradeoff between DWPT infrastructure length and required battery capacity

## 6.4. Implications for battery sizing and infrastructure planning

Taken together, these results provide an operational validation of the Infinite Drive concept and demonstrate that the proposed framework can be used not only to minimize DWPT length, but also to inform joint decisions on infrastructure deployment and battery sizing. Rather than treating battery capacity as an exogenous design choice, planners can use the framework to evaluate how targeted DWPT deployment enables smaller batteries without compromising operational reliability.

From a sustainability perspective, this tradeoff has important implications. Smaller onboard batteries imply lower vehicle mass, reduced material demand, and potentially lower upstream manufacturing impacts, while optimized DWPT deployment minimizes the physical footprint and embodied impacts of road electrification. In fact, the cost savings from reducing battery capacity can help pay for the DWPT infrastructure. By shifting part of the energy sufficiency burden from vehicle-side

overdesign to system-level infrastructure planning, optimized DWPT deployment offers a pathway toward more cost-effective, resource-efficient, and environmentally sustainable urban EV systems.

## 7. Discussion

## 7.1. Comparison with prior DWPT location work

The methodology and findings presented in this study can be situated relative to several streams of prior DWPT location research. Studies of DWPT deployment at metropolitan and corridor scales (Fuller, 2016; Honma et al., 2024; Mubarak et al., 2021; Trinko et al., 2022; Yan et al., 2022) have established the feasibility and economic rationale of large-scale DWPT deployment, but typically operate at coarser spatial resolution that does not explicitly capture intersection-driven stop-and-go dynamics. The present framework complements these by introducing intersection-level resolution through 7 m segments combined with probabilistic signal patterns and saturation headway-based queue modeling, capturing both queue-position-dependent dwell times and signal-induced acceleration/deceleration that shape charging opportunities in dense urban networks.

A parallel stream of equilibrium-based urban DWPT location models (Chen et al., 2016; Liu et al., 2021; Riemann et al., 2015; Tran et al., 2022) endogenizes driver route choice in response to deployment. The present framework occupies a complementary methodological position: by accepting fixed route flows as exogenous input, it can devote computational resources to substantially finer spatial resolution, while remaining naturally extensible to equilibrium-based assignment once DWPT deployment density grows high enough to materially alter routing behavior.

A more recent stream emphasizes joint optimization of DWPT infrastructure and battery capacity, co-determining charger locations and battery sizes for electric-bus systems (Li et al., 2024; Liu et al., 2017) and, most recently, for passenger EVs operating on road networks (Liu et al., 2024). The infrastructure–battery tradeoff demonstrated through the SOC simulations in Section 6 contributes to this line of work by adding Monte Carlo–based operational validation of long trip chains, rather than relying solely on expected-value optimization guarantees. Taken together, three features collectively distinguish the present contribution from prior streams: (i) the introduction of "Infinite Drive" as a complete-coverage design principle for urban EV infrastructure; (ii) explicit modeling of signalized intersection dynamics through probabilistic signal patterns and saturation headway-based queue representation; and (iii) operational validation through SOC simulations that complements expected-value optimization with finite-battery feasibility analysis.

## 7.2. Generalizability beyond Kawagoe

The 1.51% baseline result obtained for Kawagoe reflects the characteristics of a medium-sized Japanese city with dense signalized intersections and Japan-typical OD demand patterns. The qualitative finding—that targeted DWPT location at signalized intersections enables Infinite Drive with minimal infrastructure—is expected to hold across urban contexts, but the specific share of the road network requiring electrification will depend on local intersection density, arterial-versus-grid network topology, and trip-distance distributions. Cities with sparser intersection density may require relatively more DWPT coverage because the queue-position-dependent dwell-time mechanism that drives our efficiency results is most effective at signal-rich nodes. Conversely, cities with shorter typical trips may require less coverage. The framework itself is directly applicable to any signalized urban network with comparable input data, and future cross-city comparative studies would help establish quantitative scaling relationships between urban form characteristics and required DWPT share.

## 7.3. Limitations and assumptions revisited

Several assumptions underlying our framework warrant explicit acknowledgment. First, routes and traffic flows are treated as exogenous (Section 3.1), which is reasonable for low-to-moderate DWPT density but becomes a first-order limitation if deployment is dense enough to materially alter driver routing decisions. Second, the signal-pattern probabilities assume uniform vehicle arrival distributions within each cycle (Section 3.4), an appropriate approximation for the Kawagoe context but one that would overestimate back-of-queue dwell times in coordinated-arterial networks where platoon arrivals predominate. Third, the vehicle parameters in Table 2 represent a passenger sedan; freight, transit, and other multi-class fleets would require parameter sweeps falling outside this study's single-class focus. Fourth, the operational validation in Section 6 employs a single home-based trip-chain structure consisting of 100 round trips per run; fleet operations involving depot-based dispatch or shared autonomous vehicles would warrant separately tailored validation scenarios. These limitations define natural directions for the future research discussed below.

## 7.4. Future research directions

## Methodological extensions

Building on the limitations identified above, five methodological directions merit future investigation: (1) integration with dynamic traffic assignment to capture time-varying congestion effects (Wang et al., 2018); (2) application of multi-layer tensor architectures (Zhou et al., 2025) for joint optimization of DWPT locations, grid connections, and routing; (3) chance-constrained formulations to address battery capacity limitations more rigorously than the current safety coefficient approach; (4) computational scaling to larger metropolitan networks by restricting candidate segments to intersectionvicinity locations; and (5) multi-class extension to heterogeneous fleets, requiring integration of vehicle class-specific energy parameters with heterogeneous queue and segment representations to capture vehicle-length effects on the 7-m discretization, queue-position counting, and saturation headway.

## Environmental assessment extensions

The segment-level kinematics developed in this study provide direct inputs for environmental impact quantification. Following the cross-resolution approach (Zhou et al., 2015), future work can couple these trajectory outputs with emission rate functions to assess mixed-fleet impacts during the EV transition period. The stochastic parameters introduced by Meng et al. (2021) could extend our signal pattern formulation to capture driver variability. Additionally, a full comparative life-cycle assessment— extending the preliminary battery-side estimate presented in Section 6.3 to include DWPT road infrastructure, installation, and use-phase emissions—would quantify the net embodied carbon and material benefits. Such an assessment would also support a marginal-abatement-cost analysis of urban DWPT deployment in policy-relevant cost-per-tonne $\mathrm { C O } _ { 2 } \mathrm { e }$ terms, connecting the framework to the climate-policy literature (IPCC, 2022). Importantly, the operational validation presented in Section 6 demonstrates that such battery downsizing scenarios are not merely theoretical, but operationally feasible under realistic urban travel conditions.

## Economic considerations

Although a detailed cost analysis is beyond the scope of this study, the infrastructure–battery tradeoff demonstrated here carries direct economic implications. Smaller onboard batteries reduce vehicle purchase costs, with current Li-ion battery pack prices on the order of \$100–150 per kWh (BloombergNEF, 2024; Ziegler and Trancik, 2021) implying potential per-vehicle savings of several thousand US dollars when battery capacity is reduced from 40 kWh to 12 kWh. These savings must be weighed against DWPT deployment costs, which include infrastructure installation, maintenance, and— importantly—repair costs, since damaged or malfunctioning coils embedded in road surfaces can be more difficult and costly to access than stationary chargers. Conversely, DWPT eliminates the downtime associated with plug-in charging, which can be substantial for high-utilization fleets and represents a significant economic value beyond direct energy delivery. A full system-level cost-benefit analysis integrating vehicle-side savings, DWPT capital and operational expenditures, and downtime valuation is left as a direction for future work, drawing on existing economic feasibility studies (Honma et al., 2024; Trinko et al., 2022).

## 8. Conclusions and policy implications

This study developed a new MIP model for optimally locating DWPT infrastructure in urban areas, with the aim of realizing the concept of Infinite Drive—a scenario in which EVs can complete continuous urban operations without ever requiring plug-in charging. Application to Kawagoe City, Japan, demonstrated that Infinite Drive can be achieved with a remarkably modest amount of infrastructure. Under the baseline home-based aggregation and a safety coefficient of <sub>??</sub> <sub>=</sub> 1.1, only 2,233 m of DWPT, or 1.51% of the road network, distributed across 56 intersections was sufficient to sustain all EV operations in expectation; even under the most demanding tested condition—OD-pair aggregation with <sub>??</sub> <sub>=</sub> 1.5—the required installation length remained 4,291 m, corresponding to 2.90% of the total road network. Monte Carlo simulations of continuous trip chains further confirmed that the optimized deployment can sustain long-duration operations with finite batteries, with near-perfect performance achievable with approximately 12 kWh, 8 kWh, and 6 kWh batteries when total DWPT length is 2,233 m, 2,744 m, and 3,346 m, respectively.

Beyond demonstrating technical feasibility, the findings highlight the practical relevance of DWPT as a component of sustainable urban mobility systems. The benefits are especially pronounced for high-utilization vehicles, such as taxis, delivery fleets, ride-hailing services, and future autonomous vehicles (AVs), which operate continuously and often lack convenient access to fixed charging infrastructure. For these users, Infinite Drive provides independence from plug-in stations, increasing vehicle productivity and asset utilization.

The concept also carries important environmental and policy implications. Continuous in-motion charging reduces the need for oversized batteries, lowering vehicle weight, improving energy efficiency, and reducing reliance on scarce materials. As illustrated in Section 6.3, the corresponding vehicle-side reduction in battery manufacturing emissions is on the order of 1.7–3.0 tonnes CO<sub>2</sub>e per vehicle relative to a conventional 40 kWh urban EV, representing a front-loaded co-benefit that does not depend on future grid decarbonization. It enables more direct use of surplus renewable energy, particularly midday solar generation, while reducing idling and queuing at stationary charging facilities. While the realized magnitude of this synergy varies by local climate and renewable-energy mix, the continuous nature of DWPT provides the temporal flexibility needed to align EV demand with whichever renewable generation pattern characterizes a given urban context. As a result, DWPT can help integrate EVs more efficiently into urban energy systems and align transport policy with broader goals of carbon reduction and sustainable urban mobility.

For city planners considering DWPT deployment, three concrete recommendations emerge from this analysis. First, DWPT segments should be located at signalized intersections with frequent red-ligh encounters and substantial queue formation, rather than uniformly distributed across all approaches; as shown in Section 6.2, uniform deployment requires substantially more infrastructure while delivering lower operational reliability. Second, the choice of aggregation scheme in the optimization should be matched to the dominant vehicle class served: anchor-based aggregation for home- or depot-anchored fleets, OD-pair aggregation for recurring commute patterns, and region-based aggregation for shared mobility and autonomous services. Third, DWPT deployment decisions should be co-designed with target battery capacity rather than treating the two as independent design variables, since modest increases in installed DWPT length can enable substantial reductions in required onboard battery capacity, yielding combined cost and environmental benefits. Taken together, these findings position targeted DWPT deployment not as supplemental charging infrastructure, but as a strategic enabler of plug-free, lowcarbon, and resource-efficient urban mobility systems.

## Acknowledgments

This work was supported by the Japan Society for the Promotion of Science (JSPS), Grant-in-Aid for Scientific Research (B) 24K01109.

## References

Alwesabi, Y., Avishan, F., Yanıkoğlu, İ., Liu, Z., Wang, Y., 2022. Robust strategic planning of dynamic wireless charging infrastructure for electric buses. Appl. Energy 307, 118243.

Axsen, J., Pickrell-Barr, J., 2024. What drives fleets? Organizations’ perceived barriers and motivators for alternative-fuel vehicles. Transp. Res. D Transp. Environ. 132, 104220.

Berman, O., Bertsimas, D., Larson, R.C., 1995. Locating Discretionary Service Facilities, II: Maximizing market size, minimizing inconvenience. Oper. Res. 43, 623–632.

Bi, Z., Kan, T., Mi, C.C., Zhang, Y., Zhao, Z., Keoleian, G.A., 2016. A review of wireless power transfer for electric vehicles: Prospects to enhance sustainable mobility. Appl. Energy 179, 413–425.

Bi, Z., Keoleian, G.A., Lin, Z., Moore, M.R., Chen, K., Song, L., Zhao, Z., 2019. Life cycle assessment and tempo-spatial optimization of deploying dynamic wireless charging technology for electric cars. Transp. Res. Part C Emerg. Technol. 100, 53–67.

BloombergNEF, 2024. Lithium-Ion Battery Pack Prices See Largest Drop Since 2017, Falling to \$115 per Kilowatt-Hour [WWW Document]. BloombergNEF. URL https://about.bnef.com/insights/commodities/lithium-ion-battery-pack-prices-see-largest-dropsince-2017-falling-to-115-per-kilowatt-hour-bloombergnef/ (accessed 6.2.26).

Bonneson, J., 1992. Modeling queued driver behavior at signalized junctions. Transportation Research Record 1365, 99–107.

Bruglieri, M., Mancini, S., Pisacane, O., 2019. The green vehicle routing problem with capacitated alternative fuel stations. Comput. Oper. Res. 112, 104759.

Capar, I., Kuby, M., Leon, V.J., Tsai, Y.-J., 2013. An arc cover–path-cover formulation and strategic analysis of alternative-fuel station locations. Eur. J. Oper. Res. 227, 142–151.

Chen, P., Wei, L., Wang, T., Yu, G., 2024. Joint optimisation of vehicle trajectory and signal control at intersections mixed with connected automated vehicles: a departure sequence estimation-based approach. Transp. B Transp. Dyn. 12. https://doi.org/10.1080/21680566.2024.2303055

Chen, Z., He, F., Yin, Y., 2016. Optimal deployment of charging lanes for electric vehicles in transportation networks. Trans. Res. Part B: Methodol. 91, 344–365.

Chen, Z., Liu, W., Yin, Y., 2017. Deployment of stationary and dynamic charging infrastructure for electric vehicles along traffic corridors. Transp. Res. Part C Emerg. Technol. 77, 185–206.

Clopper, C.J., Pearson, E.S., 1934. The use of confidence or fiducial limits illustrated in the case of the binomial. Biometrika 26, 404.

Coffman, M., Bernstein, P., Wee, S., 2017. Electric vehicles revisited: a review of factors that affect adoption. Transp. Rev. 37, 79–93.

Das, T., Tanvir, S., 2024. Emissions aware car following model: A physics informed LSTM application, in: 2024 Forum for Innovative Sustainable Transportation Systems (FISTS). Presented at the 2024 Forum for Innovative Sustainable Transportation Systems (FISTS), IEEE, pp. 1–5.

Electreon, 2025. Wireless charging road technology and pilot projects [WWW Document]. Electreon. URL https://www.electreon.com/ (accessed 12.1.25).

Ellingsen, L.A.-W., Majeau-Bettez, G., Singh, B., Srivastava, A.K., Valøen, L.O., Strømman, A.H., 2014. Life cycle assessment of a lithium‐ion battery vehicle pack: LCA of a Li-ion battery vehicle pack. J. Ind. Ecol. 18, 113–124.

Emilsson, E., Dahllöf, L., 2019. Lithium-ion vehicle battery production: Status 2019 on energy use, CO2 emissions, use of metals, products’ environmental footprint, and recycling (No. C444). IVL Swedish Environmental Research Institute.

Fiori, C., Ahn, K., Rakha, H.A., 2016. Power-based electric vehicle energy consumption model: Model development and validation. Appl. Energy 168, 257–268.

Fuller, M., 2016. Wireless charging in California: Range, recharge, and vehicle electrification. Transp. Res. Part C Emerg. Technol. 67, 343–356.

Hata, K., Imura, T., Fujimoto, H., Hori, Y., Gunji, D., 2019. Charging infrastructure design for in-motion WPT based on sensorless vehicle detection system, in: 2019 IEEE PELS Workshop on Emerging

Technologies: Wireless Power Transfer (WoW). Presented at the 2019 IEEE PELS Workshop on Emerging Technologies: Wireless Power Transfer (WoW), IEEE, pp. 205–208.

He, F., Yin, Y., Zhou, J., 2013. Integrated pricing of roads and electricity enabled by wireless power transfer. Transp. Res. Part C Emerg. Technol. 34, 1–15.

Hodgson, M.J., 1990. A flow‐capturing location‐allocation model. Geogr. Anal. 22, 270–279.

Honma, Y., Hasegawa, D., Hata, K., Oguchi, T., 2024. Locational analysis of in-motion wireless power transfer system for long-distance trips by electric vehicles: Optimal locations and economic rationality in Japanese expressway network. Netw. Spat. Econ. 24, 261–290.

Honma, Y., Toriumi, S., 2017. Mathematical analysis of electric vehicle movement with respect to multiple charging stops. J. Energy Eng. 143, F4016007.

Honma, Y., Toriumi, S., 2014. Model analysis of electric vehicle charging infrastructure development on highways—an approximation of the required scale of electric vehicle charging facilities—. Forma 29, 41–50.

Hwang, I., Jang, Y.J., Ko, Y.D., Lee, M.S., 2018. System optimization for dynamic wireless charging electric vehicles operating in a multiple-route environment. IEEE Trans. Intell. Transp. Syst. 19, 1709–1726.

IPCC (Ed.), 2022. Climate change 2022 - mitigation of climate change. https://doi.org/10.1017/9781009157926

Jang, Y.J., Jeong, S., Ko, Y.D., 2015. System optimization of the On-Line Electric Vehicle operating in a closed environment. Comput. Ind. Eng. 80, 222–235.

Japan Association for the 2025 World Exposition, 2025. Demonstration of wireless power transfer buses at Expo 2025 Osaka, Kansai [WWW Document]. URL https://www.expo2025.or.jp/ (accessed 12.1.25).

Japan Science and Technology Agency, 2025. Development of Low-Carbon Society Technologies [WWW Document]. URL https://www.jst.go.jp/mirai/en/program/lowcarbon/JPMJMI21E2.html (accessed 12.1.25).

Jeong, S., Jang, Y.J., Kum, D., 2015. Economic analysis of the dynamic charging electric vehicle. IEEE Trans. Power Electron. 30, 6368–6377.

Jiang, Z., Chen, X., Ouyang, Y., 2017. Traffic state and emission estimation for urban expressways based on heterogeneous data. Transp. Res. D Transp. Environ. 53, 440–453.

Kchaou-Boujelben, M., 2021. Charging station location problem: A comprehensive review on models and solution approaches. Transp. Res. Part C Emerg. Technol. 132, 103376.

Kim, H.C., Wallington, T.J., Arsenault, R., Bae, C., Ahn, S., Lee, J., 2016. Cradle-to-gate emissions from a commercial electric vehicle Li-ion battery: A comparative analysis. Environ. Sci. Technol. 50, 7715–7722.

Kim, J.-G., Kuby, M., 2012. The deviation-flow refueling location model for optimizing a network of refueling stations. Int. J. Hydrogen Energy 37, 5406–5420.

Kimber, R., McDonald, M., Hounsell, N., 1986. The Prediction of Saturation Flow for Road Junctions Controlled by Traffic Signals. Transportation and Road Research Laboratory, Department of Transport.

Ko, Y.D., Jang, Y.J., 2013. The optimal system design of the online electric vehicle utilizing wireless power transmission technology. IEEE Trans. Intell. Transp. Syst. 14, 1255–1265.

Ko, Y.D., Jang, Y.J., Lee, M.S., 2015. The optimal economic design of the wireless powered intelligent transportation system using genetic algorithm considering nonlinear cost function. Computers & Industrial Engineering 89, 67–79.

Köster, F., Ulmer, M.W., Mattfeld, D.C., Hasle, G., 2018. Anticipating emission-sensitive traffic management strategies for dynamic delivery routing. Transp. Res. D Transp. Environ. 62, 345– 361.

Kuby, M., Cordova-Cruzatty, A., Parker, N.C., King, D.A., 2025. EV charging for multifamily housing: Review of evidence, methods, barriers, and opportunities. Renew. Sustain. Energy Rev. 210, 115253.

Kuby, M., Lim, S., 2005. The flow-refueling location problem for alternative-fuel vehicles. Socioecon. Plann. Sci. 39, 125–145.

Lawson, T.W., Lovell, D.J., Daganzo, C.F., 1997. Using input-output diagram to determine spatial and temporal extents of a queue upstream of a bottleneck. Transp. Res. Rec. 1572, 140–147.

Li, W., He, Y., Hu, S., He, Z., Ratti, C., 2024. Planning dynamic wireless charging infrastructure for battery electric bus systems with the joint optimization of charging scheduling. Transp. Res. Part C Emerg. Technol. 159, 104469.

Lim, S., Kuby, M., 2010. Heuristic algorithms for siting alternative-fuel stations using the Flow-Refueling Location Model. Eur. J. Oper. Res. 204, 51–61.

Liu, H., Zou, Y., Chen, Y., Long, J., 2021. Optimal locations and electricity prices for dynamic wireless charging links of electric vehicles for sustainable transportation. Transp. Res. Part E: Logist. Trans. Rev. 152, 102187.

Liu, S., Wang, D.Z.W., Tian, Q., Lin, Y.H., 2024. Optimal configuration of dynamic wireless charging facilities considering electric vehicle battery capacity. Transp. Res. Part E: Logist. Trans. Rev. 181, 103376.

Liu, Z., Song, Z., 2017. Robust planning of dynamic wireless charging infrastructure for battery electric buses. Transp. Res. Part C Emerg. Technol. 83, 77–103.

Liu, Z., Song, Z., He, Y., 2017. Optimal deployment of dynamic wireless charging facilities for an electric bus system. Transp. Res. Rec. 2647, 100–108.

Lu, C.-C., Liu, J., Qu, Y., Peeta, S., Rouphail, N.M., Zhou, X., 2016. Eco-system optimal time-dependent flow assignment in a congested network. Trans. Res. Part B: Methodol. 94, 217–239.

Lukic, S., Pantic, Z., 2013. Cutting the cord: Static and dynamic inductive wireless charging of electric vehicles. IEEE Electrification Mag. 1, 57–64.

Mahmoudi, M., Song, Y., Miller, H.J., Zhou, X., 2019. Accessibility with time and resource constraints: Computing hyper-prisms for sustainable transportation planning. Comput. Environ. Urban Syst. 73, 171–183.

Manshadi, S.D., Khodayar, M.E., Abdelghany, K., Uster, H., 2018. Wireless charging of electric vehicles in electricity and transportation networks. IEEE Trans. Smart Grid 9, 4503–4512.

Meng, D., Song, G., Wu, Y., Zhai, Z., Yu, L., Zhang, J., 2021. Modification of Newell’s car-following model incorporating multidimensional stochastic parameters for emission estimation. Transp. Res. D Transp. Environ. 91, 102692.

Miller, J.M., Onar, O.C., Chinthavali, M., 2015. Primary-side power flow control of wireless power transfer for electric vehicle charging. IEEE J. Emerg. Sel. Top. Power Electron. 3, 147–162.

Mubarak, M., Üster, H., Abdelghany, K., Khodayar, M., 2021. Strategic network design and analysis for in-motion wireless charging of electric vehicles. Transp. Res. Part E: Logist. Trans. Rev. 145, 102179.

Ngo, H., Kumar, A., Mishra, S., 2020. Optimal positioning of dynamic wireless charging infrastructure in a road network for battery electric vehicles. Transp. Res. D Transp. Environ. 85, 102385.

Pahwa, A., Jaller, M., 2024. Evaluating private and system-wide impacts of freight eco-routing. Transp. Res. D Transp. Environ. 130, 104170.

Pei, M., Hu, Y., Han, W., Qu, X., Zou, C., 2024. Life-Cycle analysis of economic and environmental effects for electric bus transit systems. Transp. Res. D Transp. Environ. 131, 104205.

Poulhès, A., Proulhac, L., 2021. The Paris Region low emission zone, a benefit shared with residents outside the zone. Transp. Res. D Transp. Environ. 98, 102977.

Rahman, I., Vasant, P.M., Singh, B.S.M., Abdullah-Al-Wadud, M., Adnan, N., 2016. Review of recent trends in optimization techniques for plug-in hybrid, and electric vehicle charging infrastructures. Renew. Sustain. Energy Rev. 58, 1039–1047.

ReVelle, C.S., Swain, R.W., 1970. Central facilities location. Geogr. Anal. 2, 30–42.

Riemann, R., Wang, D.Z.W., Busch, F., 2015. Optimal location of wireless charging facilities for electric vehicles: Flow-capturing location model with stochastic user equilibrium. Transp. Res. Part C Emerg. Technol. 58, 1–12.

SAE J2954 standard, 2020. Wireless Power Transfer for Light-Duty Plug-in/Electric Vehicles and Alignment Methodology [WWW Document]. URL https://www.sae.org/standards/content/j2954\_202010 (accessed 12.1.25).

Song, Y., Miller, H.J., Stempihar, J., Zhou, X., 2017. Green accessibility: Estimating the environmental costs of network-time prisms for sustainable transportation planning. J. Transp. Geogr. 64, 109– 119.

The Engineering ToolBox, 2004. Drag Coefficient [WWW Document]. URL https://www.engineeringtoolbox.com/drag-coefficient-d\_627.html (accessed 12.1.25).

Tidswell, J., Downward, A., Thielen, C., Raith, A., 2021. Minimising emissions in traffic assignment with non-monotonic arc costs. Trans. Res. Part B: Methodol. 153, 70–90.

Toregas, C., Swain, R., ReVelle, C., Bergman, L., 1971. The location of emergency service facilities. Oper. Res. 19, 1363–1373.

Tran, C.Q., Keyvan-Ekbatani, M., Ngoduy, D., Watling, D., 2022. Dynamic wireless charging lanes location model in urban networks considering route choices. Transp. Res. Part C Emerg. Technol. 139, 103652.

Trinko, D., Horesh, N., Zane, R., Song, Z., Kamineni, A., Konstantinou, T., Gkritza, K., Quinn, C., Bradley, T.H., Quinn, J.C., 2022. Economic feasibility of in-motion wireless power transfer in a high-density traffic corridor. eTransportation 11, 100154.

UK Government, 2022. COP26 declaration on accelerating the transition to 100% zero emission cars and vans [WWW Document]. Gov.uk. URL https://www.gov.uk/government/publications/cop26- declaration-zero-emission-cars-and-vans/cop26-declaration-on-accelerating-the-transition-to-100- zero-emission-cars-and-vans (accessed 12.1.25).

Upchurch, C., Kuby, M., Lim, S., 2009. A model for location of capacitated alternative‐fuel stations. Geogr. Anal. 41, 85–106.

Vallamsundar, S., Lin, J., Konduri, K., Zhou, X., Pendyala, R.M., 2016. A comprehensive modeling framework for transportation-induced population exposure assessment. Transp. Res. D Transp. Environ. 46, 94–113.

Wang, Y., Szeto, W.Y., Han, K., Friesz, T.L., 2018. Dynamic traffic assignment: A review of the methodological advances for environmentally sustainable road transportation applications. Trans. Res. Part B: Methodol. 111, 370–394.

Wang, Y.-W., Lin, C.-C., 2009. Locating road-vehicle refueling stations. Transp. Res. Part E: Logist. Trans. Rev. 45, 821–829.

Wu, X., Freese, D., Cabrera, A., Kitch, W.A., 2015. Electric vehicles’ energy consumption measurement and estimation. Transp. Res. D Transp. Environ. 34, 52–67.

Yan, L., Shen, H., Zhao, J., Xu, C., Luo, F., Qiu, C., Zhang, Z., Mahmud, S., 2022. CatCharger: Deploying in-motion wireless chargers in a metropolitan road network via categorization and clustering of vehicle traffic. IEEE Internet Things J. 9, 9525–9541.

Yi, Z., Liu, X.C., Wei, R., 2022. Electric vehicle demand estimation and charging station allocation using urban informatics. Transp. Res. D Transp. Environ. 106, 103264.

Zhang, J., Tang, T.-Q., Yan, Y., Qu, X., 2021. Eco-driving control for connected and automated electric vehicles at signalized intersections with wireless charging. Appl. Energy 282, 116215.

Zhang, P., Qian, S., 2023. Estimating environmental impacts of large-scale transportation networks with vehicle registration data. Transp. Res. D Transp. Environ. 123, 103901.

Zhong, H., Xu, R., Lu, H., Liu, Y., Zhu, M., 2023. Dynamic assessment of population exposure to trafficoriginated PM2.5 based on multisource geo-spatial data. Transp. Res. D Transp. Environ. 124, 103923.

Zhou, X., Kim, T., Ameli, M., Zhu, H., Honma, Y., Pendyala, R.M., 2025. Flow-through tensors: A unified computational graph architecture for multi-layer transportation network optimization. Artificial Intelligence for Transportation 1, 100006.

Zhou, X., Tanvir, S., Lei, H., Taylor, J., Liu, B., Rouphail, N.M., Frey, H.C., 2015. Integrating a simplified emission estimation model and mesoscopic dynamic traffic simulator to efficiently evaluate emission impacts of traffic management strategies. Transp. Res. D Transp. Environ. 37, 123–136.

Ziegler, M.S., Trancik, J.E., 2021. Re-examining rates of lithium-ion battery technology improvement and cost decline. Energy Environ. Sci. 14, 1635–1651.
