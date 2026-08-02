---
title: "Charging on the Move: Scheduling Static Chargers with Tunable Power for Mobile Devices"
year: 2021
venue: "IEEE/ACM International Symposium on Quality of Service (IWQoS)"
doi: "10.1109/IWQOS52092.2021.9521299"
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

# Charging on the Move: Scheduling Static Chargers with Tunable Power for Mobile Devices

Tao Wu∗, Panlong Yang†, Haipeng Dai‡

∗Electronic Engineering Institute, National University of Defense Technology †School of Computer Science and Technology, University of Science and Technology of China ‡State Key Laboratory for Novel Software Technology, Nanjing University, Nanjing, Jiangsu 210024, China Email: terence.taowu@gmail.com, plyang@ustc.edu.cn, haipengdai@nju.edu.cn

Abstract—The breakthrough of Wireless Power Transfer (W-PT) technique provides a promising paradigm to tackle the energy limitation problem for end-devices when replenishing energy wirelessly without the need of replacing battery. Existing works seldom consider the mobility of rechargeable devices like miniature sensors on-body or implanted medical devices which may induce great gap between practical energy supply and demand. In this paper, we study the novel issue of Charging on the Move (CM) to optimize the scheduling of transmitting power of static chargers for mobile devices. Unfortunately, solving this problem is non-trivial, because it involves nonlinearity due to time-varying distances caused by movement. Besides, charging scheduling with tunable power level is a variant of budgeted maximum coverage problem, which is NP-hard. To address CM, we approximate the variational charging power as piecewise constant power, and divide the movement trajectories with approximated charging utility. Then, we first consider our problem with fixed power level, where each charger can be scheduled off or on at a fixed power level. We prove the submodularity of the objective function and design $\textbf { \scriptsize { a } } \frac { 1 - 1 / e } { 2 }$ approximation algorithm. On this basis, we further bound the performance loss during the problem reformulation, and finally propose a $\frac { 1 - 1 / e } { 2 ( 1 + \varepsilon ) T }$ approximation algorithm for tunable scheduling strategy, where T is the maximum power level. Extensive simulations and tracedriven evaluations are conducted to evaluate the performance of our proposed algorithm.

## I. INTRODUCTION

Nowadays, the breakthrough of Wireless Power Transfer (WPT) technique has drawn significant attention from both academia and industry community [1]. Energy can be wirelessly transmitted from power chargers to rechargeable devices such as WISP and RFID tags, which provides a promising paradigm to tackle the energy limitation problem for end-devices. Due to its advantages such as high reliability and efficiency of continuous power supply, exploiting WPT technique can easily prolong the network lifetime and construct a rechargeable wireless sensor network without the need of replacing battery that requires extensive human efforts [2].

The aforementioned benefits of WPT motivate many studies regarding the issues of charging optimization in rechargeable wireless sensor networks. A large body of works has studied scheduling static chargers [3]–[7] in terms of position placement, direction scheduling, power allocation and switch on/off of chargers, etc. The other part of works [8]–[18] considers mobile chargers to visit sensor devices for energy replenishment, where the vehicle is equipped with an energy replenishment facility. They exploit the high mobility and flexibility of chargers to optimize the vacation time [8], maximize the on-demand charging requests [10] [12], minimize the charging delay [13], maximize the charging reward [17], etc. However, these schemes seldom take the mobility of rechargeable devices into account thus may induce great gap between practical energy supply and demand.

![](images/be8eb2ea1dc718abc16df9cc8b91013690801e0d4ef746a967f6ce90e68b0e9b.jpg)  
Fig. 1: Charging on the move: People with implanted/carry-on rechargeable devices travel along city roads where static chargers with tunable power (e.g., Level 1-Level 4) are deployed nearby to provide wireless charging service.

Generally, rechargeable devices are probably in a state of motion. For example, embedding miniature sensors such as onbody or implanted medical devices endow the mobility feature that are tightly coupled with human movement behaviors. These special tiny devices can help in measuring biomarkers (like glucose, cholesterol and sodium), modifying a body function (e.g., pacemaker), etc. To satisfy continuous energy supply of these devices, Xiaoran et al. [19] presented a RFbased flexible far-field charging system, In-N-Out, to avoid the risk of battery replacement by surgery [20]. They leveraged the beamforming technique to combine signals coherently at the medical implants and prototyped the In-N-Out system on 21 USRP software defined radios which could achieve 0.37W average charging power when the implant is 2m away. Besides, Powercast Corporation also developed an energy harvest RFID Temperature Scanning System for employees to carry on for battling Covid-19. The system was consisted of a fob small enough to take along and could be quickly charged when held near an RFID reader [21].

Therefore, in this paper we are concerned with the novel problem of Charging on the Move (CM). A promising scenario is proposed in Fig. 1, people with implanted/carry-on rechargeable devices travel along city roads where static chargers are deployed nearby to provide wireless charging service. Then, we propose a reliable and supplementary scheme of power supply to lengthen devices’ lifetime, that is, given a determined wireless chargers topology, with the tunable power level of any charger in an appropriate range, mobile rechargeable devices would harvest non-negligible charging power from multiple deployed chargers along its moving trajectory, which leads to the accumulated energy achievement called charging utility. Although there have emerged some wireless charger scheduling works, none of them adopts the tunable charging strategy and quantify the charging utility to give a reliable supply of energy for mobile devices. To the best of our knowledge, only [22] and [23] consider charger deployment for mobile sensors. However, they do not consider the tunable power level which is directly relevant to charging efficiency and safety. In addition, though there are few works [6] [24] considering tunable power allocation policy for chargers, which is close to ours, they do not take the mobility of rechargeable devices into account which cannot be adapted to address our problem.

Our proposed problem has two main challenges. First, the problem is essentially nonlinear because the received charging power is nonlinear with distance in the charging model. The mobility characteristic incurs time-varing received power due to dynamic positions. Calculating the accumulated charging utility on the move incurs much complexity when mobile devices travel within the charging region. Second, scheduling the transmitting power of chargers tunably is similar to a variant of the budgeted maximum coverage problem which is NP-hard [25]. Generally, various power levels bring about various charging utilities and jointly considering these two challenges would make our problem more complicated.

To tackle aforementioned challenges, we approximate the variational charging power as the piecewise constant power, and divide the moving trajectories with bounded performance loss. The objective function thus becomes almost linear and our problem becomes a combinatorial optimization problem. Next, we consider our problem when each charger can only be scheduled off or on at a fixed power level. We prove the submodularity of the objective function and transform it into the scope of maximizing a submodular function. On this basis, we further bound the performance loss when chargers can be scheduled tunably and finally propose an approximation algorithm for tunable scheduling strategy. Our contributions can be summarized as follows:

<sub>•</sub> We present a novel scheme of charging mobile devices with tunable power level of chargers and quantify the received energy to enhance the overall charging utility.

We separate trajectories according to the piecewise constant power with ε approximation of received power. We transform the problem into the scope of maximizing a submodular function, which allows $\frac { 1 ^ { \bullet } - 1 / e } { 2 }$ and $\frac { 1 - 1 / e } { 2 ( 1 + \varepsilon ) T }$ approximation algorithms for fixed and tunable power level, respectively, where T is the maximum power level.

Extensive numerical evaluations and trace-driven evaluations have been carried out to validate the performance of our propose algorithm.

TABLE I: Definition of notations

<table><tr><td>Notation</td><td>Definition</td></tr><tr><td> $c_{i}, C$ </td><td>A charger, charger set</td></tr><tr><td> $M$ </td><td>Number of chargers</td></tr><tr><td> $s_{j}, S$ </td><td>A rechargeable device, device set</td></tr><tr><td> $L_{j}, \mathcal{L}$ </td><td>Route for device  $s_{j}$ , Route set</td></tr><tr><td> $N$ </td><td>Number of devices/routes</td></tr><tr><td> $h_{i}$ </td><td>Power level of charger  $c_{i}$ </td></tr><tr><td> $p_{i}$ </td><td>Power of charger  $c_{i}$ </td></tr><tr><td> $p_{min}$ </td><td>Minimum power of a charger</td></tr><tr><td> $P_{th}$ </td><td>Threshold of negligible power</td></tr><tr><td> $T$ </td><td>Maximum power level of a charger</td></tr><tr><td> $\mathcal{T}$ </td><td>Power level of a charger</td></tr><tr><td> $H$ </td><td>A power allocation, i.e., ( $h_{1}, h_{2}, ..., h_{N}$ )</td></tr><tr><td> $D(h_{i})$ </td><td>Farthest coverage distance with respect to  $h_{i}$ </td></tr><tr><td> $d(s_{i}, o_{j})$ </td><td>Distance between  $s_{i}$  and  $o_{j}$ </td></tr><tr><td> $P_{r}(.)$ </td><td>Received charging power function</td></tr><tr><td> $v_{j}$ </td><td>Speed of mobile device  $s_{j}$ </td></tr><tr><td> $E_{j}$ </td><td>Energy capacity of device  $s_{j}$ </td></tr><tr><td> $E_{r}(.)$ </td><td>Received energy function</td></tr><tr><td> $U(.)$ </td><td>Charging utility function</td></tr><tr><td> $B$ </td><td>Power budget</td></tr><tr><td> $\alpha, \beta$ </td><td>Constant parameters</td></tr></table>

The rest of the paper is organized as follows. We review related works in Sec. II. Then, we present the system model and formulate the problem in Sec. III. The specific solution is presented in Sec. IV. Extensive numerical and real-world trace-driven evaluations are conducted in Sec. V and Sec. VI, respectively. We finally conclude the work in Sec. VII.

## II. RELATED WORKS

Many existing works focus on the charging optimization using static chargers. Zhang et al. considered the charger placement and power allocation to maximize the charging quality [6]. Dai et al. studied scheduling on/off for static chargers [5] and addressed the charger placement issue to guarantee the charging EMR safety [26]. Furthermore, they extended to jointly consider the positions and orientations in [3]. Peng et al. considered the nonlinear superposition charging effect from the radio interference [27].

There are also many works scheduling mobile chargers for power supply. In [9] [11], authors designed mobile charging scheduling schemes to decide the charging time and schedule the sensors activation. Dai et al. studied minimizing the number of mobile chargers [16]. Lin et al. focused on finding the charging path maximizing the number of nodes charged [14]. Liang et al. studied the charging utility maximization problem while sensors can be full or partial charged [17]. Tao et al. investigated the tasks-driven mobile charging problem respecting the energy requirement diversity among nodes [18]. They also considered the sensor placement for future mobile charging in [28]. Fu et al. studied to determine the mobile charger stop locations and durations to minimize the total charging delay [13]. Chiu et al. exploited the mobility nature of sensor nodes to improve charger deployment [22]. Li et al. also considered the power non-outage probability requirement of mobile sensors and tried to find a charger placement strategy [23]. However, none of them considered the received charging energy of mobile sensors.

## III. SYSTEM MODEL AND PROBLEM FORMULATION A. Network Model

We consider a set of M chargers deployed in a large 2D region, denoted as $C = \{ c _ { 1 } , c _ { 2 } , . . . c _ { M } \}$ . By abusing of notation, we still use $c _ { i }$ <sup>=</sup>to denote the location of a charger. There is a set of N rechargeable devices, denoted as $\boldsymbol { S } = \left\{ s _ { 1 } , s _ { 2 } , . . . s _ { N } \right\}$ These devices are usually attached to moving objects such as human bodies, of which the mobility patterns have certain degree of regularity. For example, people usually travel along the streets between their houses and offices regularly. We can take advantage of the mobility patterns to form moving trajectories using machine learning technique [29]. Then, we can have the straightforward but reasonable assumption that each of mobile devices ‘moves’ along a known fixed route. Let $\mathcal { L }$ be the set of N trajectories, each trajectory $L _ { j }$ is a set of points generated from a mobile rechargeable devices $s _ { j }$ We list the notations in Table I.

For any device $s _ { j } .$ , we use $( x [ s _ { j } ] , y [ s _ { j } ] )$ to denote the location coordinate of $s _ { j }$ <sup>( [ ] [ ])</sup>at any point on its mobile trajectory. The location of charger $c _ { i }$ is $( x [ c _ { i } ] , y [ c _ { i } ] )$ . We consider the <sup>( [ ] [ ])</sup>Euclidean distance as a metric and the distance between charger $c _ { i }$ and device $s _ { j }$ is defined as

$$
\begin{array}{c} d (c _ {i}, s _ {j}) = \sqrt {(x [ c _ {i} ] - x [ s _ {j} ]) ^ {2} + (y [ c _ {i} ] - y [ s _ {j} ]) ^ {2}}. \\ \text { Charging   Model } \end{array}
$$

## B. Charging Model

We assume the charger can adjust its transmitting power flexibly. The power of charger $c _ { i }$ is $p _ { i }$ and each charger can be operated at $T + 1$ different power levels. Referring to the spirit in [6], we define

$$
p _ {i} = p \left(h _ {i}\right) = p _ {\mathrm{min}} \cdot h _ {i},
$$

where $h _ { i } \subseteq \{ 0 , 1 , 2 , . . . , T \}$ is the power level of $c _ { i }$ and $p _ { m i n }$ is the minimum power of a charger. Specifically, we denote the power level set $\mathcal { H } = \{ 1 , 2 , . . . T \} . \ h _ { i } = 0$ means the charger is powered off and $h _ { i } \in \mathcal { H }$ means powered on. Admittedly, this kind of power discretization is for simplicity and the proposed method is valid as long as the allowable power level is limited. Then, a charging scheduling strategy of power for M chargers can be denoted by a vector $H = \left( h _ { 1 } , h _ { 2 } , \ldots , h _ { M } \right)$

To define the received power by a device $s _ { j }$ from charger $c _ { i }$ , we use the empirical charging model [30] as follows:

$$
P _ {r} \left(c _ {i}, s _ {j}\right) = \left\{ \begin{array}{l l} \frac {\alpha p (h _ {i})}{(d (c _ {i} , s _ {j}) + \beta) ^ {2}}, & d \left(c _ {i}, s _ {j}\right) \leq D \left(h _ {i}\right) \\ 0, & d \left(c _ {i}, s _ {j}\right) > D \left(h _ {i}\right), \end{array} \right.\tag{1}
$$

where α and $\beta$ are the constant determined by the inter hardware and surrounding environment. When a device moves far away from a charger, the device would receive negligible power that cannot be rectified to useful electrical energy. We denote the threshold of this negligible power by $P _ { t h }$ . By letting

$$
\frac {\alpha \cdot p (h _ {i})}{(D (h _ {i}) + \beta) ^ {2}} = P _ {t h},
$$

we have

$$
D \left(h _ {i}\right) = \sqrt {\frac {\alpha}{P _ {t h}} p \left(h _ {i}\right)} - \beta .
$$

$D ( h _ { i } )$ is the maximum charging distance with the power level $h _ { i }$ that could be received. In other words, the power level of charger $c _ { i }$ decides the maximum charging radius. Thus, a mobile device would receive effective power while moving into the charging circle and cannot receive while moving out.

For simplifying our problem, we assume devices move along their predefined routes at a constant speed. Let $v _ { j }$ denote the travelling rate for device $s _ { j }$ in route $L _ { j }$ and $| L _ { j } | / v _ { j }$ represents the travelling time. Then, the harvested energy $E _ { r } ( c _ { i } , s _ { j } )$ received by device $s _ { j }$ from charger $c _ { i }$ can be quantified as

$$
E _ {r} (c _ {i}, s _ {j}) = \int_ {0} ^ {| L _ {j} | / v _ {j}} P _ {r} (c _ {i}, s _ {j}) d t = \frac {1}{v _ {j}} \int_ {0} ^ {L _ {j}} P _ {r} (c _ {i}, s _ {j}) d l.
$$

Considering the received power of one device from multiple chargers is additive [31], then, given a group of chargers $C ^ { \prime } \subseteq C$ which are scheduled on, the total power received by device $s _ { j }$ is $P _ { r } ( C ^ { \prime } , s _ { j } ) = \sum _ { c . i \in C ^ { \prime } } { P ( c _ { i } , s _ { j } ) }$ . Thus, the ac-<sup>i</sup>cumulated harvested energy of device $s _ { j }$ charged by multiple chargers concurrently can be denoted as

$$
\begin{array}{c} E _ {r} (C ^ {\prime}, s _ {j}) = \frac {1}{v _ {j}} \int_ {0} ^ {L _ {j}} P _ {r} (C ^ {\prime}, s _ {j}) d l \\ = \frac {1}{v _ {j}} \int_ {0} ^ {L _ {j}} \sum_ {c _ {i} \in C ^ {\prime}} P _ {r} (c _ {i}, s _ {j}) d l \\ = \sum_ {c _ {i} \in C ^ {\prime}} E _ {r} (c _ {i}, s _ {j}). \end{array}
$$

## C. Problem Formulation

Since each device has a maximum battery capacity for energy storage, the achieved energy of device $s _ { j }$ has an upper limit value $E _ { j }$ . If the total energy $E _ { r } ( C ^ { \prime } , s _ { j } )$ received by device $s _ { j }$ is larger than $E _ { j }$ <sup>( )</sup>, the over-received power, $i . e . ,$ $E _ { r } ( C ^ { \prime } , s _ { j } ) - E _ { j }$ , would be useless. Therefore, we define the <sup>( )</sup>received charging energy $U ( C ^ { \prime } , s _ { j } )$ of on device $s _ { j }$ as

$$
U (C ^ {\prime}, s _ {j}) = \min \{E _ {r} (C ^ {\prime}, s _ {j}), E _ {j} \}.\tag{2}
$$

Then, given a set of scheduled-on chargers $C ^ { \prime } \subseteq C$ and a charging scheduling strategy H, we can define the overall charging utility on all mobile devices S along their trajectories as

$$
U (C ^ {\prime}, H) = \sum_ {s _ {j} \in S} U (C ^ {\prime}, s _ {j}).\tag{3}
$$

Generally, the power transmission network has a limited supporting capacity at any time for deployed chargers and the transmit power of chargers directly involves the economic cost. Thereby, we can assume there is a power budget constraint $B ,$ and our objective is to schedule the transmitting power for all chargers to maximize the overall charging utility. We formulate the problem as

$$
\begin{array}{c} P 1: \max U (C ^ {\prime}, H) \\ s. t \sum_ {c _ {i} \in C ^ {\prime}} p (h _ {i}) \leq B. \end{array}
$$

## D. Hardness Analysis

Lemma 1: The formulated problem P1 is NP-hard. The proof of Lemma 1 is based on the reduction from the budget maximum coverage problem [25], which is omitted due to the space limitation.

## IV. SOLUTION

Based on the NP-hardness analysis above, we first solve the problem when chargers can only be scheduled into two states, power off or on at a fixed power level $h _ { i } \in { \mathcal { H } }$ , that is, $h _ { i }$ is constant for all scheduled-on chargers. The approximation algorithms designed here will serve as basics of the method for tunable charging proposed in the next subsection. We show the overview in Fig. 2.

![](images/412bfa9157c441f5a4e180eb585e0a8303d779316a1220c27db4c36cec0d82aa.jpg)  
Fig. 2: Overview of solution.

## A. Piecewise Constant Approximation of Charging Power

Let $P _ { r } ( d )$ denote the received power of a mobile device from a charger with distance d. Due to the mobility, the device would receive various amount of power when locating at different positions, which brings much complexity to calculate harvested energy. Thus, we adopt the charging discretization method to approximate the received power $P _ { r }$ with a bounded performance error.

Based on Equation (1), we use multiple piecewise constant segments $\widehat { P } _ { r } ( \bar { d } )$ to approximate $P _ { r } ( d )$ . Fig. 3 illustrates the key idea of the charging approximation. Let $l ( 0 ) , l ( 1 ) . . . , l ( K )$ be the end points of $K$ <sup>(0) (1) ( )</sup>constant segments in an increasing sequence. Obviously, a larger K would reduce the approximation error but increase more computational overhead.

Definition 1: Setting $l ( 0 ) = 0$ and $l ( K ) = D$ , the piecewise constant function $\widehat { P } _ { r } ( d )$ can be defined as

$$
\widehat {P} _ {r} (d) = \left\{ \begin{array}{l} P _ {r} (l (1)), d = l (0) \\ P _ {r} (l (k)), l (k - 1) <   d \leq l (k) (k = 1,..., K) \cdot \\ 0, d > l (K) \end{array} \right.
$$

To bound the approximation error of piecewise constant approximation, we set up a small positive parameter ε. We derive the sufficient condition to ensure the approximation error is no more than ε in the following lemma.

Lemma 1. Setting $l ( 0 ) ~ = ~ 0 , ~ l ( K ) ~ = ~ D$ and $l ( k ) \ =$ $\begin{array} { r } { \beta ( ( 1 + \varepsilon ) ^ { \frac { k } { 2 } } - 1 ) , \mathtt {  { k } } = 1 , . . . , K - 1 , \mathtt {  { K } } = \left\lceil \frac { \ln { ( ( D + \beta ) / \beta ) ^ { 2 } } } { \ln ( 1 + \varepsilon ) } \right\rceil , } \end{array}$ we have the approximation error as

$$
1 \leq \frac {P _ {r} (d)}{\widehat {P} _ {r} (d)} \leq 1 + \varepsilon , (d \leq D).\tag{4}
$$

<sup>( )</sup>Proof: Without loss of generality, suppose we have $l ( k -$ $1 ) < d \leq l ( k )$ for a given distance $d ,$ we derive that

$$
\frac {P _ {r} (d)}{\widehat {P} _ {r} (d)} = \frac {P _ {r} (d)}{P _ {r} (l (k))} \geq 1
$$

and

$$
\begin{array}{l} \frac {P _ {r} (d)}{\widehat {P} _ {r} (d)} \leq \frac {P _ {r} (l (k - 1))}{P _ {r} (l (k))} = \frac {(l (k) + \beta) ^ {2}}{(l (k - 1) + \beta) ^ {2}} \\ = \frac {\left(\beta ((1 + \varepsilon) ^ {k / 2} - 1) + \beta\right) ^ {2}}{\left(\beta ((1 + \varepsilon) ^ {(k - 1) / 2} - 1) + \beta\right) ^ {2}} = 1 + \varepsilon . \end{array}
$$

Then, the result follows.

## B. Trajectory Discretization

Based on the constant approximation of received charging power, we divide the mobile trajectory of a device to ease the problem. As shown in Fig. 4, a device $s _ { j }$ travels across the charging range of charger $c _ { i }$ along its route $L _ { j }$ . The trajectory within the charging range can be divided into multiple segments according to the approximated charging power received.

![](images/14607e86beb0f0775e482277fddbf734126d529c1d9c9ca009662cb0844b87bf.jpg)  
Fig. 3: Charging power approximation. Fig. 4: Trajectory discretization.

Respecting the piecewise constant charging power, we draw multiple concentric charging circles centered in $c _ { i }$ with radius $l ( 1 ) , l ( 2 ) , . . . , l ( K _ { i } )$ , respectively. $l _ { 0 } ^ { i }$ is the point closest to charger $c _ { i }$ in route $L _ { j }$ . Considering the symmetry of line segments in the charging circle, we analyze the half trajectory $\overline { { l _ { 0 } ^ { i } l _ { g } ^ { i } } }$ specifically. Let $\{ l _ { 1 } ^ { i } , l _ { 2 } ^ { i } , . . . , l _ { g } ^ { i } \}$ be the $g$ intersection points in an increasing sequence separated by these concentric circles. Then, we have $d ( c _ { i } , l _ { g } ^ { i } ) = l ( K _ { i } ) , \overline { { l _ { k - 1 } ^ { i } l _ { k } ^ { i } } } = \Delta l _ { k }$ , and the divided segments are $G _ { i } = \{ l _ { k - 1 } ^ { i } l _ { k } ^ { i } | k = 1 , 2 , . . . g \}$

Thereby, we can calculate the charging utility for a separate segment as

$$
E _ {r} (c _ {i}, \Delta l _ {k}) = \frac {1}{v _ {j}} \int_ {0} ^ {\Delta l _ {k}} P _ {r} (c _ {i}, s _ {j}) d l.
$$

Due to the symmetry, we can achieve $2 \times G _ { i }$ separated segments in all after such trajectory discretization progress. Thus, the overall received charging energy can be transformed to a weighted sum, i.e.,

$$
E _ {r} (c _ {i}, s _ {j}) = 2 \sum_ {\Delta l _ {k} \in G _ {i}} E _ {r} (c _ {i}, \Delta l _ {k}).
$$

## C. Charging Utility Approximation

We first consider the case where a mobile device $s _ { j }$ is charged by only one charger $c _ { i } .$ For any segment $\overline { { l _ { k - 1 } ^ { i } l _ { k } ^ { i } } }$ in the trajectory, a device can receive an approximated charging power $\widehat { P } _ { r } ( c _ { i } , s _ { j } ) = P _ { r } ( d ( c _ { i } , l _ { k } ^ { i } ) )$ . On this basis, we replace $P _ { r } ( c _ { i } , s _ { j } )$ with $\deg _ { r } ( c _ { i } , s _ { j } )$ and achieve the approximated charging energy $\widehat { E } _ { r } ( c _ { i } , \Delta l _ { k } )$ and $\widehat { E } _ { r } ( c _ { i } , s _ { j } )$ , respectively.

We note that though such approximation incurs performance loss, we show in the proof to Theorem 3 that such loss can be bounded. First, we have the following theorem when single device harvests energy from single charger.

Theorem 1. Let $\widehat { E } _ { r } ( c _ { i } , s _ { j } )$ be the approximated received charging energy of device $s _ { j }$ from charger $c _ { i }$ when the trajectory within the charging range is divided into $2 \times G _ { i }$ segments according to the approximated charging power received, namely, $\begin{array} { r } { \widehat { E } _ { r } ( c _ { i } , s _ { j } ) = \hat { 2 } { } \sum _ { \substack { \Lambda l , \mathscr { c } C } } \frac { 1 } { v _ { j } } \int _ { 0 } ^ { \Delta l _ { k } } \widetilde { P } _ { r } ( c _ { i } , s _ { j } ) d l } \end{array}$ . We have Δl G <sup>k</sup>the approximation error as

$$
1 \leq \frac {E _ {r} (c _ {i} , s _ {j})}{\widehat {E} _ {r} (c _ {i} , s _ {j})} \leq 1 + \varepsilon .\tag{5}
$$

Proof: In the light of Equation (4), the approximation error for a separated segment satisfies

$$
\begin{array}{r l} & E _ {r} (c _ {i}, \Delta l _ {k}) = \int_ {0} ^ {\Delta l _ {k}} \frac {P _ {r} (c _ {i} , s _ {j})}{v _ {j}} d l \\ & \qquad \leq \int_ {0} ^ {\Delta l _ {k}} \frac {(1 + \varepsilon) \widehat {P} _ {r} (c _ {i} , s _ {j})}{v _ {j}} d l \\ & \qquad = (1 + \varepsilon) \widehat {E} _ {r} (c _ {i}, \Delta l _ {k}). \end{array}
$$

Then, we have

$$
\frac {E _ {r} (c _ {i} , s _ {j})}{\widehat {E} _ {r} (c _ {i} , s _ {j})} = \frac {\sum_ {\Delta l _ {k} \in G _ {i}} 2 E _ {r} (c _ {i} , \Delta l _ {k})}{\sum_ {\Delta l _ {k} \in G _ {i}} 2 \widehat {E} _ {r} (c _ {i} , \Delta l _ {k})} \leq 1 + \varepsilon
$$

as well as

$$
\frac {E _ {r} (c _ {i} , s _ {j})}{\widehat {E} _ {r} (c _ {i} , s _ {j})} \geq 1.
$$

Then, the result follows.

If one device receives energy by multiple chargers concurrently, dividing trajectory would be complex but still maintains a bounded performance error. We have the following theorem when single device harvests energy from multiple chargers.

Theorem 2. Let $\widehat { E } _ { r } ( C ^ { \prime } , s _ { j } )$ be the approximated received charging energy of device $s _ { j }$ from multiple chargers $C ^ { \prime }$ of which the charging range are overlapped. The trajectory is divided into<sup>-</sup> $G _ { C ^ { \prime } }$ segments according to the $a p \cdot$ proximated charging power received, namely, $\widehat { E } _ { r } ( C ^ { \prime } , s _ { j } ) =$ $\sum _ { \Delta l _ { k } \in G _ { C ^ { \prime } } } \int _ { 0 } ^ { \Delta l _ { k } } \frac { \widehat { P } _ { r } ( C ^ { \prime } , s _ { j } ) } { v _ { j } } d l .$

<sup>C</sup>We have the approximation error as

$$
1 \leq \frac {E _ {r} (C ^ {\prime} , s _ {j})}{\widehat {E} _ {r} (C ^ {\prime} , s _ {j})} \leq 1 + \varepsilon .\tag{6}
$$

Proof: In this part, we specify how to divide the trajectory in the overlapped region and achieve the separated segments $G _ { C ^ { \prime } }$ . Without loss of generality, we assume set $C ^ { \prime }$ has two chargers $c _ { 1 }$ and $c _ { 2 }$ with corresponding power level $h _ { 1 }$ and $h _ { 2 } .$ . As shown in $\operatorname { F i g } . 5$ , the wireless charging range of these two chargers are overlapped. Device $s _ { j }$ moves across both two chargers through the overlapped region. We denote the intersection points of two charging circles as $l _ { g _ { 1 } } ^ { 1 }$ and $l _ { g _ { 2 } } ^ { 2 }$ respectively. After the mobile charging discretization process in Section IV-B, multiple separated segments are obtained where the split points can be expressed as $\{ l _ { 0 } ^ { 1 } , l _ { 1 } ^ { 1 } , . . . , l _ { g _ { 1 } } ^ { 1 } \}$ and $\{ l _ { 0 } ^ { 2 } , l _ { 1 } ^ { 2 } , . . . , l _ { g _ { 2 } } ^ { \bar { 2 } } \}$ , respectively.

For segments out of the overlapped region, we can achieve the approximated charging energy based on Theorem 1. For segments in the overlapped region, taking $\overline { { l _ { g _ { 1 } - 2 } ^ { 1 } l _ { g _ { 2 } - 2 } ^ { 2 } } }$ as an example, we present how to calculate the approximated charging energy and analyze the approximated error case by case.

(i) If the original segment is divided by the left intersection point $l _ { g _ { 2 } } ^ { 2 } , i . e .$ , segment $\overline { { l _ { g _ { 1 } - 2 } ^ { 1 } l _ { g _ { 1 } - 1 } ^ { 1 } } }$ is divided, then we have two extra separated segments $\overline { { l _ { g _ { 1 } - 2 } ^ { 1 } l _ { g _ { 2 } } ^ { 2 } } }$ and $\overline { { l _ { g _ { 2 } } ^ { 2 } l _ { g _ { 1 } - 1 } ^ { 1 } } }$ , respectively.

For segment $\overline { { l _ { g _ { 1 } - 2 } ^ { 1 } l _ { g _ { 2 } } ^ { 2 } } }$ , since the device moving along this separated trajectory only receives energy from charger $c _ { 1 }$ , the accumulated received energy is $E _ { r } ( C ^ { \prime } , \overline { { l _ { g _ { 1 } - 2 } ^ { 1 } l _ { g _ { 2 } } ^ { 2 } } } ) =$ $E _ { r } ( c _ { 1 } , \overline { { l _ { g _ { 1 } - 2 } ^ { 1 } l _ { g _ { 2 } } ^ { 2 } } } )$ . We have

$$
\begin{array}{l} E _ {r} (C ^ {\prime}, \overline {{l _ {g _ {1} - 2} ^ {1} l _ {g _ {2}} ^ {2}}}) \\ = \frac {1}{v _ {j}} \int_ {0} ^ {\overline {{l _ {g _ {1} - 2} ^ {1} l _ {g _ {2}} ^ {2}}}} P _ {r} (c _ {1}, s _ {j}) d l \\ \leq \frac {1}{v _ {j}} \int_ {0} ^ {\overline {{l _ {g _ {1} - 2} ^ {1} l _ {g _ {2}} ^ {2}}}} (1 + \varepsilon) \widehat {P} _ {r} (c _ {1}, s _ {j}) d l \\ = \frac {1}{v _ {j}} \int_ {0} ^ {\overline {{l _ {g _ {1} - 2} ^ {1} l _ {g _ {2}} ^ {2}}}} (1 + \varepsilon) P _ {r} (c _ {1}, l _ {g _ {1} - 1} ^ {1}) d l \\ = (1 + \varepsilon) \frac {1}{v _ {j}} P _ {r} (c _ {1}, l _ {g _ {1} - 1} ^ {1}) \overline {{l _ {g _ {1} - 2} ^ {1} l _ {g _ {2}} ^ {2}}} \\ = (1 + \varepsilon) \widehat {E} _ {r} (c _ {1}, \overline {{l _ {g _ {1} - 2} ^ {1} l _ {g _ {2}} ^ {2}}}). \end{array}
$$

For segment $\overline { { l _ { g _ { 2 } } ^ { 2 } l _ { g _ { 1 } - 1 } ^ { 1 } } }$ , device moving along this separated trajectory receives energy from both chargers $c _ { 1 }$ and $c _ { 2 } ,$ then the accumulated received energy is $E _ { r } ( C ^ { \prime } , \overline { { l _ { g _ { 2 } } ^ { 2 } l _ { g _ { 1 } - 1 } ^ { 1 } } } ) =$ $E _ { r } ( c _ { 1 } , \overline { { l _ { g _ { 2 } } ^ { 2 } l _ { g _ { 1 } - 1 } ^ { 1 } } } ) + E _ { r } ( c _ { 2 } , \overline { { l _ { g _ { 2 } } ^ { 2 } l _ { g _ { 1 } - 1 } ^ { 1 } } } )$ . We have

$$
\begin{array}{l} E _ {r} (C ^ {\prime}, \overline {{l _ {g _ {2}} ^ {2} l _ {g _ {1} - 1} ^ {1}}}) \\ = \frac {1}{v _ {j}} \int_ {0} ^ {\overline {{l _ {g _ {2}} ^ {2} l _ {g _ {1} - 1} ^ {1}}}} (P _ {r} (c _ {1}, s _ {j}) + P _ {r} (c _ {2}, s _ {j})) d l \\ \leq \frac {1}{v _ {j}} \int_ {0} ^ {\overline {{l _ {g _ {2}} ^ {2} l _ {g _ {1} - 1} ^ {1}}}} (1 + \varepsilon) (\widehat {P} _ {r} (c _ {1}, s _ {j}) + \widehat {P} _ {r} (c _ {2}, s _ {j})) d l \\ = \frac {1 + \varepsilon}{v _ {j}} (P _ {r} (c _ {1}, l _ {g _ {1} - 1} ^ {1}) \overline {{l _ {g _ {2}} ^ {2} l _ {g _ {1} - 1} ^ {1}}} + P _ {r} (c _ {2}, l _ {g _ {2}} ^ {2}) \overline {{l _ {g _ {2}} ^ {2} l _ {g _ {1} - 1} ^ {1}}}) \\ = (1 + \varepsilon) (\widehat {E} _ {r} (c _ {1}, \overline {{l _ {g _ {2}} ^ {2} l _ {g _ {1} - 1} ^ {1}}}) + \widehat {E} _ {r} (c _ {2}, \overline {{l _ {g _ {2}} ^ {2} l _ {g _ {1} - 1} ^ {1}}})) \end{array}
$$

(ii) If the segment is fully located in the overlapped region, $i . e . , \overline { { l _ { g _ { 1 } - 1 } ^ { 1 } l _ { g _ { 2 } - 1 } ^ { 2 } } }$ , device moving along this separated trajectory could also receive energy from chargers $c _ { 1 }$ and $^ { c _ { 2 } , }$ then the accumulated received energy is $E _ { r } ( C ^ { \prime } , \overline { { { l _ { g _ { 1 } - 1 } ^ { 1 } l _ { g _ { 2 } - 1 } ^ { 2 } } } } ) =$ $\overline { { E _ { r } ( c _ { 1 } , \overline { { l _ { g _ { 1 } - 1 } ^ { 1 } l _ { g _ { 2 } - 1 } ^ { 2 } } } ) } } + E _ { r } ( c _ { 2 } , \overline { { l _ { g _ { 1 } - 1 } ^ { 1 } l _ { g _ { 2 } - 1 } ^ { 2 } } } )$ . We have

$$
\begin{array}{l} E _ {r} (C ^ {\prime}, \overline {{l _ {g _ {1} - 1} ^ {1} l _ {g _ {2} - 1} ^ {2}}}) \\ = \frac {1}{v _ {j}} \int_ {0} ^ {\overline {{l _ {g _ {1} - 1} ^ {1} l _ {g _ {2} - 1} ^ {2}}}} (P _ {r} (c _ {1}, s _ {j}) + P _ {r} (c _ {2}, s _ {j})) d l \\ \leq \frac {1}{v _ {j}} \int_ {0} ^ {\overline {{l _ {g _ {1} - 1} ^ {1} l _ {g _ {2} - 1} ^ {2}}}} (1 + \varepsilon) (\widehat {P} _ {r} (c _ {1}, s _ {j}) + \widehat {P} _ {r} (c _ {2}, s _ {j})) d l \\ = \frac {(1 + \varepsilon)}{v _ {j}} (P _ {r} (c _ {1}, l _ {g _ {1}} ^ {1}) \overline {{l _ {g _ {1} - 1} ^ {1} l _ {g _ {2} - 1} ^ {2}}} + P _ {r} (c _ {2}, l _ {g _ {2}} ^ {2}) \overline {{l _ {g _ {1} - 1} ^ {1} l _ {g _ {2} - 1} ^ {2}}}) \\ = (1 + \varepsilon) (\widehat {E} _ {r} (c _ {1}, \overline {{l _ {g _ {1} - 1} ^ {1} l _ {g _ {2} - 1} ^ {2}}}) + \widehat {E} _ {r} (c _ {2}, \overline {{l _ {g _ {1} - 1} ^ {1} l _ {g _ {2} - 1} ^ {2}}})) \end{array}
$$

(iii) If the original segment is divided by the right intersection point $l _ { g _ { 1 } } ^ { 1 } , i . e . , \overline { { l _ { g _ { 2 } - 1 } ^ { 2 } l _ { g _ { 2 } - 2 } ^ { 2 } } }$ is divided, then we have two extra separated segments $\overline { { l _ { g _ { 2 } - 1 } ^ { 2 } l _ { g _ { 1 } } ^ { 1 } } }$ and $\overline { { l _ { g _ { 1 } } ^ { 1 } l _ { g _ { 2 } - 1 } ^ { 2 } } } .$ , respectively.

$$
\overline {{l _ {g _ {2} - 1} ^ {2} l _ {g _ {1}} ^ {1}}}.
$$

$$
c _ {2}
$$

$$
c _ {1}
$$

$$
E _ {r} (C ^ {\prime}, \overline {{l _ {g _ {2} - 1} ^ {2} l _ {g _ {1}} ^ {1}}}) =
$$

$$
E _ {r} (c _ {1}, \overline {{l _ {g _ {2} - 1} ^ {2} l _ {g _ {1}} ^ {1}}}) + E _ {r} (c _ {2}, \overline {{l _ {g _ {2} - 1} ^ {2} l _ {g _ {1}} ^ {1}}})
$$

$$
\begin{array}{r l} & E _ {r} (C ^ {\prime}, \overline {{l _ {g _ {2} - 1} ^ {2} l _ {g _ {1}} ^ {1}}}) \\ & = \frac {1}{v _ {j}} \int_ {0} ^ {\overline {{l _ {g _ {2} - 1} ^ {2} l _ {g _ {1}} ^ {1}}}} (P _ {r} (c _ {1}, s _ {j}) + P _ {r} (c _ {2}, s _ {j})) d l \\ & \leq \frac {1}{v _ {j}} \int_ {0} ^ {\overline {{l _ {g _ {2} - 1} ^ {2} l _ {g _ {1}} ^ {1}}}} (1 + \varepsilon) (\widehat {P} _ {r} (c _ {1}, s _ {j}) + \widehat {P} _ {r} (c _ {2}, s _ {j})) d l \\ & = \frac {(1 + \varepsilon)}{v _ {j}} (P _ {r} (c _ {1}, l _ {g _ {1}} ^ {1}) \overline {{l _ {g _ {2} - 1} ^ {2} l _ {g _ {1}} ^ {1}}} + P _ {r} (c _ {2}, l _ {g _ {2} - 1} ^ {2}) \overline {{l _ {g _ {2} - 1} ^ {2} l _ {g _ {1}} ^ {1}}}) \\ & = (1 + \varepsilon) (\widehat {E} _ {r} (c _ {1}, \overline {{l _ {g _ {2} - 1} ^ {2} l _ {g _ {1}} ^ {1}}}) + \widehat {E} _ {r} (c _ {2}, \overline {{l _ {g _ {2} - 1} ^ {2} l _ {g _ {1}} ^ {1}}})) \end{array}
$$

For segment $\overline { { l _ { g _ { 1 } } ^ { 1 } l _ { g _ { 2 } - 1 } ^ { 2 } } }$ , since a device moving along this separated trajectory only receives energy from charger $c _ { 2 } .$ , then the accumulated received energy is $E _ { r } ( C ^ { \prime } , \overline { { { l _ { g _ { 1 } } ^ { 1 } l _ { g _ { 2 } - 1 } ^ { 2 } } } } ) =$ $E _ { r } ( c _ { 2 } , \overline { { l _ { g _ { 1 } } ^ { 1 } l _ { g _ { 2 } - 1 } ^ { 2 } } } )$ . We have

![](images/0caf8d4987ab4e7c2e13c5cd2420b90d1427562d6912736c32e19588b5cda648.jpg)  
Fig. 5: Charging utility approximation.

$$
\begin{array}{l} E _ {r} (C ^ {\prime}, \overline {{l _ {g _ {1}} ^ {1} l _ {g _ {2} - 1} ^ {2}}}) \\ = \frac {1}{v _ {j}} \int_ {0} ^ {\overline {{l _ {g _ {1}} ^ {1} l _ {g _ {2} - 1} ^ {2}}}} P _ {r} (c _ {2}, s _ {j}) d l \\ \leq \frac {1}{v _ {j}} \int_ {0} ^ {\overline {{l _ {g _ {1}} ^ {1} l _ {g _ {2} - 1} ^ {2}}}} (1 + \varepsilon) \widehat {P} _ {r} (c _ {2}, s _ {j}) d l \\ = \frac {(1 + \varepsilon)}{v _ {j}} P _ {r} (c _ {2}, l _ {g _ {2} - 1} ^ {2}) \overline {{l _ {g _ {1}} ^ {1} l _ {g _ {2} - 1} ^ {2}}} \\ = (1 + \varepsilon) \widehat {E} _ {r} (c _ {2}, \overline {{l _ {g _ {1}} ^ {1} l _ {g _ {2} - 1} ^ {2}}}). \end{array}
$$

Therefore, we can achieve the approximated received charging energy for any trajectories divided by chargers $C ^ { \prime } =$ $\{ c _ { 1 } , c _ { 2 } \}$ . If the amount of overall separated segments is $G _ { C ^ { \prime } }$ then we have the accumulated energy received by device $s _ { j }$ as follows:

$$
\begin{array}{l} E _ {r} (C ^ {\prime}, s _ {j}) = \int_ {0} ^ {L _ {j}} \frac {P _ {r} (C , s _ {j})}{v _ {j}} d l \\ = \sum_ {\Delta l _ {k} \in G _ {C ^ {\prime}}} \frac {1}{v _ {j}} \int_ {0} ^ {\Delta l _ {k}} P _ {r} (C ^ {\prime}, s _ {j}) d l \\ = \sum_ {\Delta l _ {k} \in G _ {C ^ {\prime}}} E _ {r} (C ^ {\prime}, \Delta l _ {k}) \\ \leq (1 + \varepsilon) \sum_ {\Delta l _ {k} \in G _ {C ^ {\prime}}} \widehat {E} _ {r} (C ^ {\prime}, \Delta l _ {k}) \\ = (1 + \varepsilon) \widehat {E} _ {r} (C ^ {\prime}, s _ {j}). \end{array}
$$

Meanwhile, it is easy to see $E _ { r } ( C ^ { \prime } , s _ { j } ) \geq \widehat { E } _ { r } ( C ^ { \prime } , s _ { j } )$ . Thus, the proof is completed. ■

## D. Problem Reformulation and Solution

In this subsection, we elaborate on how to schedule the chargers on/off to maximize the overall charging utility for all mobile devices. Specifically, we first reformulate the problem, then prove its submodularity, and thereby propose an effective algorithm to address the problem.

Noticeably, scheduling all the chargers on/off is equivalent to select a subset of chargers to transfer power. For any charger subset $C ^ { \prime } \subseteq C$ deployed in the region which is selected, we can compute the approximated charging energy received by each mobile device correspondingly. Thus, we have $\begin{array} { r } { \widehat { U } ( \boldsymbol { C } ^ { j } ) = \boldsymbol { \Sigma } } \end{array}$ min $\{ \widehat { E } _ { r } ( C ^ { \prime } , s _ { j } ) , \bar { E _ { j } } \}$ and formulate the s <sub>∈</sub>S

<sup>j</sup>following problem as

$$
\begin{array}{c} P 2: m a x \widehat {U} (C ^ {\prime}) \\ s. t \sum_ {c _ {i} \in C ^ {\prime}} p (h _ {i}) \leq B, \\ C ^ {\prime} \subseteq C. \end{array}
$$

Now, P2 becomes a combinational optimization problem to select the chargers $C ^ { \prime }$ scheduled on from C. Before addressing

P2, we first give the following definitions.

Definition 2: (Nonnegativity, Monotonicity, and Submodularity) Given a finite ground set , a real-valued set function defined as $f : 2 ^ { \mathcal { U } } \to R ,$ f is called nonnegative, monotone (nondecreasing), and submodular if and only if it satisfies following conditions, respectively.

$f ( \varnothing ) = 0$ and $f ( X ) \geq 0$ for all $X \subseteq { \mathcal { U } }$ (nonnegative);

$f ( X ) ~ \leq ~ f ( Y )$ <sup>( )</sup>for all $X \subseteq Y \subseteq \mathcal { U }$ or equivalently: $f ( X \cup \{ e \} ) - f ( X ) > 0$ for all $X \subseteq { \mathcal { U } }$ and $e \in \mathcal { U } \backslash X$ <sup>( )</sup>(monotone);

$f ( X ) + f ( Y ) \geq f ( X \cup Y ) + f ( X \cap Y )$ , for any $X , Y \subseteq { \mathcal { U } }$ or equivalently: $f ( X \cup \{ e \} ) - f ( X ) \geq f ( Y \cup \{ e \} ) -$ $f ( Y ) , X \subseteq Y \subseteq \mathcal { U } , e \in \mathcal { U } \backslash Y$ (submodular);

Then, we have the following lemma:

Lemma 2. The objective function in P2 is nonnegative, monotone and submodular.

Proof: Based on the definition of the objective function in P2, we can check whether $\widehat { U } ( C ^ { \prime } )$ satisfies the three listed properties. First, clearly $\widehat { U } ( C ^ { \prime } )$ is nonnegative that $\widehat { U } ( C ^ { \prime } ) \geq 0$ for all $C ^ { \prime } \subseteq C$

Second, For charger set $C ^ { \prime } \subseteq C$ and new opened charger $c _ { i } \subseteq C \backslash C ^ { \prime }$ , we have $\widehat { U } ( C ^ { \prime } \cup \{ c _ { i } \} ) - \widehat { U } ( C ^ { \prime } ) \leq \mathrm { 0 }$ because the received energy of devicew is accumulated when travelling across the charging range of multiple chargers. Then the objective function is monotone.

Third, let $C ^ { \prime }$ and $C ^ { \prime \prime }$ be two set such that $C ^ { \prime } \subseteq C ^ { \prime \prime } \subseteq C$ and charger $c _ { i } \subseteq C \backslash C ^ { \prime \prime }$ . Here, we give the specific analysis that $\widehat { U } ( \bar { C ^ { \prime } } )$ is submodular by proving

$$
\widehat {U} (C ^ {\prime} \cup \{c _ {i} \}) - \widehat {U} (C ^ {\prime}) \geq \widehat {U} (C ^ {\prime \prime} \cup \{c _ {i} \}) - \widehat {U} (C ^ {\prime \prime}).
$$

It is equal to prove

$$
\widehat {U} (C ^ {\prime} \cup \{c _ {i} \}, s _ {j}) - \widehat {U} (C ^ {\prime}, s _ {j}) \geq \widehat {U} (C ^ {\prime \prime} \cup \{c _ {i} \}, s) - \widehat {U} (C ^ {\prime \prime}, s _ {j}).
$$

Case 1: If $\begin{array} { r l r } { E _ { j } } & { { } \le } & { \widehat { E } _ { r } ( C ^ { \prime } , L _ { j } ) } \end{array}$ , we have $\begin{array} { r l } { \widehat { U } ( C ^ { \prime } , s _ { j } ) } & { { } = } \end{array}$ min $\{ \widehat { E } _ { r } ( C ^ { \prime } , L _ { j } ) , \widehat { E } _ { j } \} = E _ { j }$ . Similarly, $\widehat { U } ( C ^ { \prime \prime } , s _ { j } ) = E _ { j }$ . Then, <sup>( )</sup>we have the marginal charging utility

$$
\widehat {U} (C ^ {\prime} \cup \{c _ {i} \}, s _ {j}) - \widehat {U} (C ^ {\prime}, s _ {j}) \geq \widehat {U} (C ^ {\prime \prime} \cup \{c _ {i} \}, s) - \widehat {U} (C ^ {\prime \prime}, s _ {j}).
$$

Case 2: If $\widehat { E } _ { r } ( C ^ { \prime } , L _ { j } ) \ \leq \ E _ { j } \ \leq \ \widehat { E } _ { r } ( C ^ { \prime \prime } , L _ { j } )$ , we have $\widehat { U } ( C ^ { \prime \prime } , s _ { j } ) = E _ { j }$ . Meanwhile, we can derive that

$$
\begin{array}{l} \widehat {U} (C ^ {\prime} \cup \{c _ {i} \}, s) - \widehat {U} (C ^ {\prime}, s) \\ = m i n \{\widehat {E} _ {r} (C ^ {\prime} \cup \{c _ {i} \}, L _ {j}), E _ {j} \} - m i n \{\widehat {E} _ {r} (C ^ {\prime}, L _ {j}), E _ {j} \} \\ = m i n \{\widehat {E} _ {r} (C ^ {\prime} \cup \{c _ {i} \}, L _ {j}), E _ {j} \} - \widehat {E} _ {r} (C ^ {\prime}, L _ {j}) \\ = m i n \{\widehat {E} _ {r} (\{c _ {i} \}, L _ {j}), E _ {j} - \widehat {E} _ {r} (C ^ {\prime}, L _ {j}) \} \geq 0. \end{array}
$$

Case 3: If $\widehat { E } _ { r } ( C ^ { \prime \prime } , L _ { j } ) \leq E _ { j }$ , in this case, we have

$$
\begin{array}{r l} & {\widehat {U} (C ^ {\prime} \cup \{c _ {i} \}, s _ {j}) - \widehat {U} (C ^ {\prime}, s _ {j})} \\ & {= m i n \{\widehat {E} _ {r} (C ^ {\prime} \cup \{c _ {i} \}, L _ {j}), E _ {j} \} - m i n \{\widehat {E} _ {r} (C ^ {\prime}, L _ {j}), E _ {j} \}} \\ & {= m i n \{\widehat {E} _ {r} (C ^ {\prime} \cup \{c _ {i} \}, L _ {j}), E _ {j} \} - \widehat {E} _ {r} (C ^ {\prime}, L _ {j})} \\ & {= m i n \{\widehat {E} _ {r} (\{c _ {i} \}, L _ {j}), E _ {j} - \widehat {E} _ {r} (C ^ {\prime}, L _ {j}) \}} \end{array}
$$

and

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 1: Utility-Cost ratio (UC-ratio)

Input: Charger set C with corresponding power level H, the route set L for devices S, power budget B.

Output:  $C'$ .

1 Initialization.  $C_{1}' = C_{2}' = \emptyset$ .

2  $C_{1}' \leftarrow \arg\max_{c_{i} \in C} \widehat{U}(c_{i}), p(h_{i}) \leq B$ .

3 while  $C \neq \emptyset$  do

4  $c \leftarrow \arg\max_{c_{i} \in C \setminus C_{2}'} \frac{\widehat{U}(C_{2}' \cup \{c_{i}\}) - \widehat{U}(C_{2}')}{p_{i}}$ .

5 if  $\sum_{c_{i} \in C_{2}' \cup \{c\}} p(h_{i}) \leq B$  then

6  $C_{2}' = C_{2}' \cup \{c\}$ .

7  $C = C \setminus \{c\}$ .

8  $C' = \arg\max_{C' \in \{C_{1}', C_{2}'\}} \widehat{U}(C')$ .

 $\widehat{U}(C'' \cup \{c_{i}\}, s_{j}) - \widehat{U}(C'', s_{j}) = min\{\widehat{E}_{r}(C'' \cup \{c_{i}\}, L_{j}), E_{j}\} - min\{\widehat{E}_{r}(C'', L_{j}), E_{j}\} = min\{\widehat{E}_{r}(C'' \cup \{c_{i}\}, L_{j}), E_{j}\} - \widehat{E}_{r}(C'', L_{j}) = min\{\widehat{E}_{r}(c_{i}\}, L_{j}), E_{j} - \widehat{E}_{r}(C'', L_{j})\}$ .

For i.  $\widehat{E}_{r}(\{c_{i}\}, L_{j}) \leq E_{j} - \widehat{E}_{r}(C'', L_{j})$ , we have

 $min\{\widehat{E}_{r}(\{c_{i}\}, L_{j}), E_{j} - \widehat{E}_{r}(C', L_{j})\} = \widehat{E}_{r}(\{c_{i}\}, L_{j}) = min\{\widehat{E}_{r}(\{c_{i}\}, L_{j}), E_{j} - \widehat{E}_{r}(C'', L_{j})\}$ . Then, we have

 $\widehat{U}(C' \cup \{c_{i}\}, s_{j}) - \widehat{U}(C', s_{j}) = \widehat{U}(C'' \cup \{c_{i}\}, s_{j}) - \widehat{U}(C'', s_{j})$ .

For ii.  $E_{j} - \widehat{E}_{r}(C'', L_{j}) \leq \widehat{E}_{r}(\{c_{i}\}, L_{j}) \leq E_{j} - \widehat{E}_{r}(C', L_{j})$ , we have

 $min\{\widehat{E}_{r}(\{c_{i}\}, L_{j}), E_{j} - \widehat{E}_{r}(C', L_{j})\} = \widehat{E}_{r}(\{c_{i}\}, L_{j}) \geq E_{j} - \widehat{E}_{r}(C'', L_{j}) = min\{\widehat{E}_{r}(\{c_{i}\}, L_{j}), E_{j} - \widehat{E}_{r}(C'', L_{j})\}$ .

For iii.  $\widehat{E}_{r}(C', L_{j}) \leq \widehat{E}_{r}(\{c_{i}\}, L_{j})$ , then we have

 $min\{\widehat{E}_{r}(\{c_{i}\}, L_{j}), E_{j} - \widehat{E}_{r}(C', L_{j})\} = E_{j} - \widehat{E}_{r}(C', L_{j}) \geq E_{j} - \widehat{E}_{r}(C'', L_{j}) = min\{\widehat{E}_{r}(\{c_{i}\}, L_{j}), E_{j} - \widehat{E}_{r}(C'', L_{j})\}$ .

Therefore, we have

 $\widehat{U}(C' \cup \{c_{i}\}, s_{j}) - \widehat{U}(C', s_{j}) \geq \widehat{U}(C'' \cup \{c_{i}\}, s_{j}) - \widehat{U}(C'', s_{j})$ . and prove  $\widehat{U}(C')$  is submodular.

Therefore, our reformulated problem falls into the scope of the problem of maximizing a monotone submodular function subject to the budget constraint. Then, we can devise a simple but efficient approximation algorithm. The core idea in this algorithm is to greedily select the charger c to schedule on at each step. Initially,  $C' = C' = \emptyset$ , we iteratively select the charger  $c_i$  into  $C_2'$  to schedule on which has the largest utility-cost ratio, i.e.,  $\widehat{U}(C' + |s_c|) - \widehat{U}(C')$ .
</div>

$$
\begin{array}{c} r a t i o,   i. e., \\ c \leftarrow \underset {c _ {i} \in C \setminus C _ {2} ^ {\prime}} {\arg \max} \frac {\widehat {U} (C _ {2} ^ {\prime} \cup \{c _ {i} \}) - \widehat {U} (C _ {2} ^ {\prime})}{p _ {i}}. \end{array}
$$

Utilizing this iterative method, we can add more chargers continuously until violating the power constraint. We then find the device set $C _ { 2 } ^ { \prime } ,$ satisfying $\sum _ { c _ { i } \in C _ { 2 } ^ { \prime } } p ( h _ { i } ) \ \leq \ B$ . Finally we would compare $\widehat { U } ( C _ { 2 } ^ { \prime } )$ with $\widehat { U } ( C _ { 1 } ^ { \prime } )$ to select the maximal one. Therefore, we will get an approximation ratio of $\frac { 1 - 1 / e } { 2 }$ referring to [25] and the time complexity is $O ( M ^ { 2 } )$

## E. Approximation Algorithm for Tunable Charging Scheduling

When the power level of a charger can be scheduled flexibly, the power level of charger $c _ { i }$ can be or vary in . Considering the approximated charging energy received by the devices, we can replace the objective function $U ( C ^ { \prime } , H )$ in P1 with $\widehat { U } ( C ^ { \prime } , H )$ to formulate the problem P3 as follows:

$$
\begin{array}{c} P 3: m a x \widehat {U} (C ^ {\prime}, H) \\ s. t \sum_ {c _ {i} \in C ^ {\prime}} p (h _ {i}) \leq B. \end{array}
$$

Before addressing this problem, we first consider a variant of the problem P3: In the location $c _ { i } ,$ , there are $T$ chargers and the power levels of these chargers are $1 , 2 , . . . , T _ { \mathrm { { ; } } }$ , respectively. We use $\left( { { c } _ { i } } , { { h } _ { k } } \right)$ to denote the charger with the power level k and then have $M \cdot T$ chargers in total. Given a power budget B, how to schedule these $M \cdot T$ chargers on/off that maximize the overall received energy.

Let $Z$ be the Cartesian product of C and , which denotes the set of all chargers. If charger $\left( { { c } _ { i } } , { { h } _ { k } } \right)$ is scheduled on, the power $P _ { r } ( c _ { i } , h _ { k } , s _ { j } )$ received by device $s _ { j }$ from $c _ { i }$ with power level $h _ { k }$ is

$$
P _ {r} \left(c _ {i}, h _ {k}, s _ {j}\right) = \left\{ \begin{array}{l l} \frac {\alpha p (h _ {k})}{(d (c _ {i} , s _ {j}) + \beta) ^ {2}}, & d \left(c _ {i}, s _ {j}\right) \leq D \left(h _ {k}\right), \\ 0, & d \left(c _ {i}, s _ {j}\right) > D \left(h _ {k}\right). \end{array} \right.
$$

Correspondingly, given the charger set $Z ^ { \prime }$ , the total power $P _ { r } \left( Z ^ { \prime } , s _ { j } \right)$ received by device $s _ { j }$ is

$$
P _ {r} \left(Z ^ {\prime}, s _ {j}\right) = \sum_ {\left(c _ {i}, h _ {k}\right) \in Z ^ {\prime}} P _ {r} \left(c _ {i}, h _ {k}, s _ {j}\right).
$$

Similarly, we can utilize the charging discretization method to divide all trajectories of the mobile devices. Let $G _ { Z ^ { \prime } }$ denote the divided segment set for sensor $s _ { j }$ and the approximated power received is $\begin{array} { r l } { \widehat { P } _ { r } \left( Z ^ { \prime } , s _ { j } \right) = \ } & { { } \sum \ } \end{array} \ \widehat { P } _ { r } \left( c _ { i } , h _ { k } , s _ { j } \right)$ . Then, we achieve the $( c _ { i } , \overline { { h _ { k } } } ) \in Z ^ { \prime }$ <sup>i k</sup>accumulated approximated energy received along its trajectory $\hat { E } _ { r } ( Z ^ { \prime } , s _ { j } ) = \sum _ { \Delta l _ { k } \in G _ { \tau } , \atop \Delta l _ { k } \in G _ { \tau } } 2 \hat { E } _ { r } ( Z ^ { \prime } , \overbrace { \Delta l _ { k } } ^ { \Delta l _ { k } } ) = \sum _ { \Delta l _ { k } \in G _ { \tau ^ { \prime } } } \frac { 2 \hat { P } _ { r } ( Z ^ { \prime } , s _ { j } ) \Delta l _ { k } ^ { - } } { v _ { j } }$ <sup>k Z k Z</sup>and the approximated charging utility for sensor $s _ { j }$ is

$$
\widehat {U} _ {Z ^ {\prime}} (s _ {j}) = \min \{\sum_ {\Delta l _ {k} \in G _ {Z ^ {\prime}}} \hat {E} _ {r} (Z ^ {\prime}, \Delta l _ {k}), E _ {j} \}.
$$

Thereby, the objective function is

$$
\widehat {U} (Z ^ {\prime}) = \sum_ {s _ {j} \in S} \hat {U} _ {Z ^ {\prime}} (s _ {j}).
$$

<sup>j</sup>We formulate the problem P4 similar to P2 as follows:

$$
\widehat {U} (Z ^ {\prime})
$$

$$
s.t\sum_{\substack{(c_{i},h_{k})\in Z^{\prime}\\ Z^{\prime}\subseteq Z.}}p(h_{k})\leq B,
$$

On this basis, we can solve problem P3 by devising a Two-Step Algorithm (TSA) to decide all the power levels of chargers flexibly. Denote by $I _ { i }$ a row vector where the $i \textrm { -- } t h$ element is 1 and all other elements are zeros, i.e., ${ \cal I } _ { i } ~ = ~ ( 0 , 0 , . . . , 1 , . . . , 0 ) . ~ H [ c _ { i } ]$ is used to store the selected maximal power level of charger $c _ { i }$ . The main idea follows two steps.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 2: Two-Step Algorithm (TSA)

Input: Charger set C, device set S with associated route set L, power budget B.

Output: Scheduled-on charger set C', Power scheduling strategy H.

1 Initially.  $Z = C \times H$ ,  $Z_{1} = Z_{2} = C_{1}' = C_{2}' = \emptyset$ .

 $H_{1} = H_{2} = 0$ .

2  $(c, h) \leftarrow \arg\max_{(c_{i}, h_{k}) \in Z} \hat{U}(c_{i}, h_{k}), p(h_{k}) \leq B$ .

3  $Z_{1} = (c, h)$ ,  $C_{1}' = \{c\}$ ,  $H_{1}[c] = h$ .

4 while  $Z \neq \emptyset$  do

5    $(c, h) \leftarrow \arg\max_{(c_{i}, h_{k}) \in Z} \frac{\hat{U}(Z_{2} \cup \{(c_{i}, h_{k})\}) - \hat{U}(Z_{2})}{p(h_{k})}$ .

6    if  $\sum_{(c_{i}, h_{k}) \in Z_{2} \cup \{(c, h)\}} p(h_{k}) \leq B$  then

7    $Z_{2} = Z_{2} \cup \{(c, h)\}$ .

8    if  $H_{2}[c] &lt; h$  then

9    $H_{2}[c] \leftarrow h$ .

10    $Z = Z \setminus \{(c, h)\}$ .

11  $(C', H_{2}) \leftarrow RemoveDuplicationAndUtilize(C, S, B, H_{2})$ .

12  $(C', H) \leftarrow \arg\max_{(C', H) \in \{(C_{1}', H_{1}), (C_{2}', H_{2})\}} \hat{U}(C', H)$ .

13 Sub-procedure: RemoveDuplicationAndUtilize

Input: C, S, B, H.

Output:  $C'$ , H.

14  $C' \leftarrow \emptyset$ ,  $B' \leftarrow 0$ 

15 foreach  $H[c_{i}] &gt; 0$  do

16    $C' \leftarrow C' \cup \{c_{i}\}$ ,  $B' \leftarrow B' + p(H[c_{i}])$ .

17 while  $B - B' \geq p_{min}$  do

18    $c_{i} \leftarrow \arg\max_{H[c_{i}] + 1 \leq T} \hat{U}(C' \cup \{c_{i}\}, H + I_{i}) - \hat{U}(C', H)$ .

19    $C' \leftarrow C' \cup \{c_{i}\}$ ,  $H[c_{i}] \leftarrow H[c_{i}] + 1$ ,  $B' \leftarrow B' + p_{min}$ .

20 return  $(C', H)$ .
</div>

First, referring to the previous method in Algorithm 1, we use the greedy strategy to solve the problem P4 and obtain the power-on chargers set $Z ^ { \prime }$ (Line 5). We use $H _ { 2 } [ c _ { i } ]$ to store the maximum power level $h _ { k }$ for all selected charger $( c _ { i } , h _ { k } ) \subseteq Z ^ { \prime }$ <sup>( )</sup>(Lines 6-9). Second, considering multiple chargers at the same location $c _ { i }$ can be selected, we invoke RemoveDuplicationAndUtilize sub-procedure to select chargers with the maximal power level (Lines 14-16). If $H [ c _ { i } ] > 0$ , we add the corresponding charger into $C ^ { \prime }$ . Thus, we retain at most one charger for each location to transfer $Z ^ { \prime }$ into $C ^ { \prime }$ . Considering there may be some unused energy budget for $C ^ { \prime } { . }$ , then we try to reuse the residual budget $B - B ^ { \prime }$ (Lines 17-19) to improve the received energy. Specifically, we allocate a fixed power $p _ { m i n }$ to the charger that maximizes the increasement of charging utility. Finally, we compare $C _ { 1 } ^ { \prime } , H _ { 1 }$ with $C _ { 2 } ^ { \prime } , H _ { 2 }$ to achieve the better one. Therefore, we have the following theorem.

Theorem 3. Setting $l ( k ) \ = \ \beta ( ( 1 + \varepsilon ) ^ { \frac { k } { 2 } } \ - \ 1 )$ , Algorithm 2 achieves an approximation ratio of $\frac { 1 - 1 / e } { 2 ( 1 + \varepsilon ) T }$ and its time complexity is $O ( M ^ { 2 } T ^ { 3 } )$ .

Proof: Let $( C ^ { * } , H ^ { * } )$ denote the set of strategies of all M chargers under the optimal solution to problem P1 or P3. $C ^ { * }$ represents the chargers that scheduled on and $H ^ { * }$ represents the corresponding power level.

First, combining Equation $( 2 ) , ( 3 ) , ( 5 ) , ( 6 ) .$ , we can achieve

$$
\widehat {U} (C ^ {*}, H ^ {*}) \geq U (C ^ {*}, H ^ {*}) / (1 + \varepsilon).\tag{7}
$$

Second, let $Z ^ { * }$ be the optimal solution of P4, and $Z ^ { \prime } =$ $\arg \operatorname* { m a x } _ { Z ^ { \prime } \in \{ Z _ { 1 } , Z _ { 2 } \} } \widehat { U } ( Z ^ { \prime } )$ , where $Z _ { 1 }$ and $Z _ { 2 }$ <sup>=</sup>are the solution generated by Line 3 and 7, respectively. According the results in Algorithm 2 in Section IV-D, we know

$$
\widehat {U} (Z ^ {\prime}) \geq \widehat {U} (Z ^ {*}) (1 - 1 / e) / 2.\tag{8}
$$

Third, if we restrict that only one charger with its fixed power level can be scheduled on at each charging location, the problem P4 is equal to P3. In this point, we have

$$
\widehat {U} (Z ^ {*}) \geq \widehat {U} (C ^ {*}, H ^ {*}).\tag{9}
$$

Fourth, since we can schedule at most $T$ chargers at one location in $Z ^ { \prime }$ and always retain the charger that has the maximal power level when transferring $Z ^ { \prime }$ into $C ^ { \prime }$ , we have

$$
\widehat {U} (C ^ {\prime}) \geq \widehat {U} (Z ^ {\prime}) / T.\tag{10}
$$

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Combine above equations together, we have
 $\widehat{U}(C', H) = \max\{\widehat{U}(C_{1}', H_{1}), \widehat{U}(C_{2}', H_{2})\}$ $\geq \max\{\widehat{U}(Z_{1}), \widehat{U}(Z_{2})\}/T$  (refer to Equation (10))
 $= \widehat{U}(Z') / T \geq \frac{1 - 1/e}{2T} \widehat{U}(Z^{*})$  (refer to Equation (8))
 $\geq \frac{1 - 1/e}{2T} \widehat{U}(C^{*}, H^{*})$  (refer to Equation (9))
 $\geq \frac{1 - 1/e}{2(1 + \varepsilon)T} U(C^{*}, H^{*})$ . (refer to Equation (7))
Thus, we prove the approximation ratio of the TSA algorithm.
We omit the time complexity analysis to save space.
</div>

## V. EVALUATION

In this section, we conduct extensive numerical simulations to verify the performance of the proposed algorithm in terms of the charger number M, device number N, charging discretization parameter ε and budget B.

## A. Evaluation Setup

In our simulation, the sensing region is a m m square area, chargers are uniformly distributed in this region. The trajectories of mobile devices are randomly generated that travelling across some chargers or not. If no otherwise stated, we set $M = 1 5 , N = 2 5 , p _ { m i n } = 1 0 0 , T = 1 0 , \alpha = 1 0 , \beta =$ . The speed of mobile devices is uniformly generated between $2 m / s$ and $3 m / s$ . The threshold $p _ { t h }$ of negligible power is . and the default power budget is $B = 4 0 0 0$

## B. Baseline Setup

As there are no algorithms available for our formulated problem, we devised 4 algorithms named random algorithm (RAN) and fixed power level algorithms with local optimal level (FPL-OPT), maximal level (FPL-MAX) and minimal level (FPL-MIN) for comparison.

RAN first sets a power level $h ( i )$ in $\tau$ for all chargers with the same probability and randomly schedules chargers on while not violating the budget constraint. If exceeding the budget, RAN randomly schedules another charger on. Both FPL algorithms consist of two phases. The first phase is to decide the power level of each charger in advance. The second phase is to invoke UR-ratio algorithm to generate a charger scheduling strategy with fixed power level. Specifically, FPL-OPT has the power level calculated by

![](images/774ab5131c87e3fe7e97232205bda25bb48c5980a7977fceae9616462206c698.jpg)  
Fig. 6: Utility vs. <sup>M</sup>

![](images/f6cc60985a9f0dd58f141a2065948ae75e0b6bd2971e7133a9d675a91bf2a906.jpg)  
Fig. 7: Utility vs. <sup>N</sup>

$$
h _ {i} = \arg \max _ {h _ {i} \in \{1, \dots , T \}} \sum_ {s _ {j} \in S} \widehat {U} (c _ {i}, s _ {j}) / (p (h _ {i})).
$$

## C. Evaluation Results and Analysis

1) Impact of the number of chargers M. Our proposed algorithm outperforms RAN, FLP-OPT, FLP-MAX, FLP-MIN on average by . , . , . , . , respectively, as the number of chargers increases from 11 to 20. From Fig. 6 we can see that the results of most algorithms except RAN have an increasing trend. This is because the increasement of chargers gives more chances of charging scheduling since we can schedule M T chargers on/off in all. Some newly deployed chargers may cover more mobile devices with closer distance. Thus, we can always select the charger which has the maximum charging utility at each iteration using the greedy strategy. Naturally, RAN has little variance due to the random selection under the budget constraint.

2) Impact of the number of mobile devices N. Our proposed algorithm outperforms RAN, FLP-OPT, FLP-MAX, FLP-MIN on average by . , . , . , . , respectively, as the number of mobile devices rises from 20 to 65. Fig. 7 shows that the charging utility of the proposed algorithm and other algorithms all grow up nearly linearly. Obviously, more devices enter the charging region make more chances of wireless power transfer. Then, the increasement of the number of mobile devices gives more improving space of the accumulated charging utility of all devices.

3) Impact of parameter ε. Our proposed algorithm outperforms RAN, FLP-OPT, FLP-MAX, FLP-MIN on average by . , . , . , . , respectively, as the parameter ε increases from 0.1 to 0.9. As shown in Fig. 8, all these algorithms have a slight diminishing trend. The reason is that the approximated charging power is inverse proportion to parameter ε according to Equation (6). Furthermore, rising the value of parameter ε would directly improve the approximated charging distance according to the equation $d ( k ) = \beta ( ( 1 + \varepsilon ) ^ { \frac { k } { 2 } } - 1 )$ . Thus, the received charging power of piecewise constant approximation decreases and the overall approximated energy obtained from scheduled chargers diminishes. Admittedly, reducing the value of parameter ε would elevate the obtained charging utility but also lead to much computational overhead. Therefore, to reduce the running time of our algorithm, we set ε . .

4) Impact of power budget B. Our proposed algorithm outperforms RAN, FLP-OPT, FLP-MAX, FLP-MIN on average by . , . , . , . , as the power budget B increases from 1000 to 7000. Obviously, as we improve the power budget, the performance of all algorithms has an increasing trend. As depicted in Fig. 9, our algorithm increases the charging utility by . when the budget is improved from 1000 to 5000. However, it then becomes relatively stable when the budget is more than 5000. What accounts for this stability is the influence of the device capacity of battery. Increasing the budget would not improve the charging utility when devices are charged to saturated status.

![](images/dde57be35487bdbc3e88091a9d7ecad13ed40b8a892559630fcbd937a5a46fcb.jpg)  
Fig. 8: Utility vs. <sup>ε</sup>

![](images/9672057cf090b9a98e7b32d616b8f2cb12f22779ab1845816b3b2f814f6661d3.jpg)  
Fig. 9: Utility vs. <sup>B</sup>

![](images/c2f874941b37f7b28aa6b6e0e269827a995d85b069c81f02e274ec414e0fa8ab.jpg)  
Fig. 10: A scheduling strategy visualization.

For easy understanding, we visualize the tunable charging scheduling strategy for mobile devices as shown in Fig. 10. There are 10 chargers and 8 moving trajectories of devices in the m m region. Different lengths of radius means the maximum charging range when setting different charging power. By executing our proposed algorithm, we only need to schedule 5 chargers powered-on with different charging power to achieve a good charging utility.

## VI. REAL WORLD TRACE EVALUATION

To further evaluate our proposed algorithm, we conduct realworld trace-driven evaluations based on the GPS trajectory dataset of users in Beijing, China [32]. We select 25 moving routes and simulate 411 chargers deployment with uniform distribution in km km region as shown in Fig. 11. Other parameter settings are the same in Section V-A.

We show a representative result in Fig. 12 when the power budget B increases from 200000 to 290000. Our proposed algorithm also has the best utility and outperforms RAN, FLP-OPT, FLP-MAX, FLP-MIN on average by . , . , <sup>25 71% 16 1%</sup>. , . , respectively. As we improve the power budget, all the algorithms have an increasing trend until up to the device capacity of battery. Specifically, our algorithm increases the charging utility by . when the budget is improved from 200000 to 290000. Therefore, our proposed algorithm presents the good effect to support the scheme of charging on the move.

![](images/b1e5793274c27c285513fc7e57c7a4f429a7a65418c4aad7f887010712bd8f4f.jpg)  
Fig. 11: The 25 user trajectories in Beijing.

![](images/c874bb9cbb81a7129f56594d1b69a277070add3ee0f8379c1aa87fc186e2eab9.jpg)  
Fig. 12: Utility vs. <sup>B</sup>

## VII. CONCLUSION

This paper represents the first effort towards tunable charger scheduling for mobile devices to maximize the overall charging utility. We approximate the charging power into piecewise constant power and partition the moving trajectory of devices. We present specific theoretical analysis to bound the discrete performance loss and prove the submodularity of the reformulated objective function. $\mathrm { ~ A ~ } \frac { 1 - 1 / e } { 2 }$ approximation algorithm is proposed to schedule charging on/off with fixed power level and a $\frac { 1 - 1 / e } { 2 ( 1 + \varepsilon ) T }$ approximation algorithm is devised to decide the tunable power level, respectively. Finally, we evaluate the performance of the proposed algorithm against the fixed power level and our proposed algorithm outperforms well according to the results in the simulation and real-world trace-driven experiments.

## ACKNOWLEDGMENTS

This research is partially supported by the National Natural Science Foundation of China under Grant No. 62002377, 62072424, 61772546, 61625205, 61632010, 61751211, 61772488, 61520106007, in part by the Fundamental Research Funds for the Central Universities under Grant 14380059, in part by the Key Research Program of Frontier Sciences, CAS, No. QYZDY-SSW- JSC002 and NSF ECCS-1247944, and NSF CNS 1526638, in part by the National key research and development plan No. 2017YFB0801702, 2018YFB1004704.

## REFERENCES

[1] Kurs et al., “Wireless power transfer via strongly coupled magnetic resonances.” Science, 2007.

[2] Xie et al., “Wireless power transfer and applications to sensor networks,” IEEE Wireless Communications, vol. 20, no. 4, pp. 140–145, 2013.

[3] H. Dai et al., “Optimizing wireless charger placement for directional charging,” in IEEE INFOCOM, 2017, pp. 1–9.

[4] X. Wang et al., “Practical heterogeneous wireless charger placement with obstacles,” IEEE Transactions on Mobile Computing, vol. 19, no. 8, pp. 1910–1927, 2020.

[5] H. Dai et al., “Safe charging for wireless power transfer,” IEEE/ACM Transactions on Networking, vol. 25, no. 6, pp. 3531–3544, 2017.

[6] Zhang et al., “P3: Joint optimization of charger placement and power allocation for wireless power transfer,” in IEEE INFOCOM, 2015, pp. 2344–2352.

[7] H. Dai et al., “Charging task scheduling for directional wireless charger networks,” IEEE Transactions on Mobile Computing, no. 99, pp. 1–1, 2020.

[8] Y. Shi et al., “On renewable sensor networks with wireless energy transfer,” in IEEE INFOCOM, 2011, pp. 1350–1358.

[9] H. Dai et al., “Near optimal charging and scheduling scheme for stochastic event capture with rechargeable sensors,” in IEEE MASS, 2013, pp. 10–18.

[10] C. Lin et al., “Tadp: Enabling temporal and distantial priority scheduling for on-demand charging architecture in wireless rechargeable sensor networks,” Journal of Systems Architecture, vol. 70, pp. 26–38, 2016.

[11] H. Dai et al., “Chase: Charging and scheduling scheme for stochastic event capture in wireless rechargeable sensor networks,” IEEE Transactions on Mobile Computing, vol. 19, no. 1, pp. 44–59, 2020.

[12] L. Jiang et al., “Effective on-demand mobile charger scheduling for maximizing coverage in wireless rechargeable sensor networks,” Mobile Networks and Applications, vol. 19, no. 4, pp. 543–551, 2014.

[13] L. Fu et al., “Minimizing charging delay in wireless rechargeable sensor networks,” in IEEE INFOCOM, 2013, pp. 2922–2930.

[14] L. Chen et al., “Charge me if you can: Charging path optimization and scheduling in mobile networks,” in ACM MobiHoc, 2016, pp. 101–110.

[15] L. C. et al., “Tsca: A temporal-spatial real-time charging scheduling algorithm for on-demand architecture in wireless rechargeable sensor networks,” IEEE Transactions on Mobile Computing, vol. 17, no. 1, pp. 211–224, 2018.

[16] H. Dai et al., “Minimizing the number of mobile chargers for large-scale wireless rechargeable sensor networks,” Computer Communications, vol. 46, pp. 54–65, 2014.

[17] W. Liang et al., “Approximation algorithms for charging reward maximization in rechargeable sensor networks via a mobile charger,” IEEE/ACM Transactions on Networking, vol. 25, no. 5, pp. 3161–3174, 2017.

[18] T. Wu et al., “Collaborated tasks-driven mobile charging and scheduling: A near optimal result,” in IEEE INFOCOM, 2019, pp. 1810–1818.

[19] X. Fan et al., “Towards flexible wireless charging for medical implants using distributed antenna system,” in ACM MobiCom, 2020, pp. 22:1– 22:15.

[20] “Pacemaker battery replacement causes cardiac arrest,” https://www.expertinstitute.com/resources/case-studies/ pacemaker-battery-replacement-causes-cardiac-arrest/.

[21] “Rfid journal,” https://www.rfidjournal.com/ rfid-ir-provide-temperature-screening-and-location-system/.

[22] T. C. Chiu et al., “Mobility-aware charger deployment for wireless rechargeable sensor networks,” in APNOMS, 2012.

[23] Y. Li et al., “Charging while moving: Deploying wireless chargers for powering wearable devices,” IEEE Transactions on Vehicular Technology, vol. 67, pp. 11 575–11 586, 2018.

[24] H. Dai et al., “Scape: Safe charging with adjustable power,” IEEE/ACM Transactions on Networking, no. 99, pp. 1–14, 2018.

[25] S. Khuller et al., “The budgeted maximum coverage problem,” Information Processing Letters, vol. 70, no. 1, pp. 39–45, 1999.

[26] H. Dai et al., “Radiation constrained wireless charger placement,” in IEEE INFOCOM, 2016, pp. 1–9.

[27] P. Guo et al., “Concurrently wireless charging sensor networks with efficient scheduling,” IEEE Transactions on Mobile Computing, vol. 16, no. 9, pp. 2450–2463, 2017.

[28] T. Wu et al., “Charging oriented sensor placement and flexible scheduling in rechargeable wsns,” in IEEE INFOCOM, 2019, pp. 73–81.

[29] X. Fan et al., “Buildsensys: Reusing building sensing data for traffic prediction with cross-domain learning,” IEEE Transactions on Mobile Computing, 2020.

[30] C. Zhu et al., “A survey on coverage and connectivity issues in wireless sensor networks,” Journal of Network & Computer Applications, vol. 35, no. 2, pp. 619–632, 2012.

[31] H. S et al., “Energy provisioning in wireless rechargeable sensor networks,” IEEE Transactions on Mobile Computing, vol. 12, no. 10, pp. 1931–1942, 2012.

[32] Y. Zheng et al., “Geolife: A collaborative social networking service among user, location and trajectory.” vol. 33, 2010, pp. 32–39.
