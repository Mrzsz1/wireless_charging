---
title: "Practical Heterogeneous Wireless Charger Placement with Obstacles"
year: 2020
venue: "IEEE Transactions on Mobile Computing"
doi: "10.1109/TMC.2019.2916384"
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

# Practical Heterogeneous Wireless Charger Placement with Obstacles

Xiaoyu Wang , Student Member, IEEE, Haipeng Dai , Member, IEEE, Weijun Wang Jiaqi Zheng , Member, IEEE, Nan Yu , Guihai Chen , Member, IEEE, Wanchun Dou , Member, IEEE, and Xiaobing Wu, Member, IEEE

Abstract—This paper considers the problem of practical Heterogeneous wIreless charger Placement with Obstacles (HIPO), i.e., given a number of heterogeneous rechargeable devices distributed on a 2D plane where obstacles of arbitrary shapes exist, deployin heterogeneous chargers with a given cardinality of each type. j.e., determining their positions and orientations, the combination of which we name as strategies, on the plane such that the rechargeable devices achieve maximized charging utility. After presenting our practical directional charging model, we first propose to use a piecewise constant function to approximate the nonlinear charging power, and divide the whole area into multi-feasible geometric areas in which a certain type of chargers have constant approximated charging power. Next, we propose the Practical Dominating Coverage Set extraction algorithm to reduce the unlimited solution space to a limited one by exacting a finite set of candidate strategies for all multi-feasible geometric areas. Finally, we prove the problem falls in the realm of maximizing a monotone submodular function subject to a partition matroid constraint, which allows a greedy algorithm to solve with approximation ratio of <sup>1</sup>  . We conduct experiments to evaluate the performance. Results show that our algorithm outperforms the comparison algorithms by at least 33.49 percent on average.

Index Terms—Charger placement, heterogeneity, obstacles

## 1 INTRODUCTION

nience such as no-wiring, no-contact, and reliability. Wireless Power Consortium, which aims to promote the standardization of WPT, has grown to include 275 companies including Apple and Huawei in 2017. By a recent report, there are more than 300 million commercial products based on WPT technology in use [1].

In a WPT system, devices are typically equipped with directional antennas to achieve high energy transfer efficiency by focusing the energy in narrow energy beams. Motivated by this fact, [2], [3] propose the directional charging model for which wireless chargers (or rechargeable devices) can only provide (or receive) non-zero power in a sector area called (power) charging area (or (power) receiving area). We argue that, however, this model is not sufficient to fully capture the charging characteristics in practice. To be specific, although it is a common sense that a device too far away from a charger receives negligible charging power, a device too close to a charger may also receive negligible or zero power for the following practical reasons. First, a charger may cease to work once it detects that a device is in close proximity for reasons like security or efficiency. For instance, by our filed test results, the commodity off-the-shelf TX91501 wireless charger produced by Powercast [4] transmits charging power if and only if a device is at least 17 cm away under typical settings, and it has an LED status indicator to show the working status. Second, in practical deployment, a wireless charger is usually elevated off the flat surface, where rechargeable devices locate at, for practical concerns such as electrical safety or larger coverage area. Thus, despite that we can project the 3D charging area of chargers to the flat surface for analysis, some devices below chargers may not be covered because of the directional charging pattern of chargers. Last but not least, a wireless charger together with its accessories, such as a platform it is mounted on, occupies considerable space in reality. Therefore, after abstracting a wireless charger as a point in theoretical analysis, it would be an appropriate (not perfect) choice to assume that a device receives zero power when it is too close to a charger.

Consequently, we propose our practical directional charging model as shown in Fig. 1, which generalizes the traditional directional charging model [2], [3]. Specifically, the charging area of chargers which are of the same type of charger $s _ { i }$ is modeled as a sector ring with distance ranges between $d _ { m i n } ^ { i }$ and $d _ { m a x } ^ { i }$ . Due to geometric symmetry, the power receiving area of devices is also modeled as a sector ring within range between $d _ { m i n } ^ { i }$ and $d _ { m a x } ^ { i } .$ . Besides, we for the first time take into consideration heterogeneity of chargers and devices as well as obstacles in wireless charger networks. Chargers/devices can have different parameter settings such as different types of antennas, which leads to different charging/receiving power and charging/receiving areas. Obstacles can be of arbitrary shapes and can block the line-of-sight transmitting power without reflection. For example, as shown in Fig. 1, o<sub>j</sub> and $o _ { k }$ denote two devices of different types, and they have different power receiving angles due to their distinct hardware parameters. Moreover, in Fig. 2, an obstacle lies in the receiving area of device $o _ { i }$ makes chargers placed in the two shaded areas (we call that holes) unable to charge o .

![](images/1eb46eab966f25098ebf5038366a05cc3611cd60cc9f6e4a0741388e065a2e42.jpg)  
Fig. 1. Charging model with heterogeneity.

In this paper, we consider the problem of practical Heterogeneous wIreless charger Placement with Obstacles (HIPO). Formally, given a number of heterogeneous rechargeable devices with fixed positions and orientations distributed on a 2D plane where obstacles of arbitrary shapes exist, deploying heterogeneous chargers with a given cardinality of each type, i.e., determining their positions and orientations, the combinations of which we name as strategies, on the plane such that the devices achieve maximized charging utility.

The related work of this paper mainly involves wireless charger placement, wireless sensor placement with obstacles, and heterogeneous wireless sensor placement networks; but none of their solutions can be adapted to address our problem. Specifically, the first adopts charging models less practical than ours, and it does not consider obstacles or heterogeneity of chargers and devices. The second considers the omnidirectional sensing model and its presented algorithms are heuristic. The third substantially differs from ours due to its different problem settings, which makes its solutions not applicable to our problem.

Generally, there are four main challenges in our problem. First, the charging power is nonlinear with distance and, therefore, the problem cannot be regarded as a simple geometric coverage problem. Second, obstacles are assumed to be in arbitrary shapes, which raises challenges in evaluating not only the occupation of solution space of chargers by obstacles, but also the caused blockage effect on transmitting power of chargers. Third, there are infinite candidate strategies for chargers to choose, which means the solution space is unlimited. Moreover, the non-convexity of the sector ring charging area leads to further difficulty in solution space analysis. Fourth, the heterogeneity of the chargers and devices makes the problem more complicated. We need to enumerate all possible combinations of chargers and devices due to their distinct charging parameters, and jointly consider placing all types of chargers towards an optimal solution.

To address these challenges, for the first one, we use a piecewise constant function to approximate the nonlinear charging power, and accordingly divide the whole area into several so-called geometric areas, such that a charger placed anywhere in a geometric area provides the same constant approximated charging power to a given device. For the second challenge, we further divide the geometric areas into feasible geometric areas by the boundaries of obstacles and holes corresponding to devices, and thereby, a charger anywhere in the feasible geometric areas with respect to a device can charge the device with non-zero power. For the third challenge, we propose the so-called Practical Dominating Coverage Set (PDCS) extraction algorithm to reduce the unlimited solution space to a limited one in each feasible geometric area without performance loss. In particular, the PDCS extraction algorithm leverages the rotational symmetry of the sector ring charging area to deal with its non-convexity. For the last challenge, the whole area is divided for multiple times into several versions of feasible geometric areas (multi-feasible geometric areas) corresponding to different types of chargers independently. Then, we perform the PDCS extraction algorithm in each feasible geometric area and obtain candidate strategies for chargers. Afterwards, the whole problem is modeled as maximizing a monotone submodular function subject to a partition matroid constraint, which allows a greedy algorithm to solve with <sup>1</sup>   approximation ratio.

![](images/953775c472c886a9f4444ee1917559e9d00509715a9f41c0ba7d70e27d2fd8d6.jpg)  
Fig. 2. Obstacles on the plane.

We evaluate our algorithm with simulations and field experiments. The results show that our algorithm outperforms the comparison algorithms by at least 33.49 percent.

## 2 RELATED WORK

Wireless Charger Networks. To the best of our knowledge, all existing works regarding wireless charger networks never consider the sector ring charging model of chargers, obstacles, or heterogeneity of chargers and/or devices, and none of them can be applied to address our problem. Generally, there are two commonly used charging models. First, some existing related works adopt the omnidirectional charging model for which both of the charging area of chargers and the power receiving area of devices are disks, i.e., regardless of the charging and receiving direction. Some works aim to optimize the charging quality, prolong the network lifetime, or consider fault tolerance [5], [6], [7], [8], [9], [10], [11], [12], [13], [14], [15]. In addition, some studies consider the wireless charger placement problem with low electromagnetic radiation constraints for the sake of human health. Nikoletseas et al.first proposed the concept of low radiation efficient charging [16], and Dai et al. proposed a charger placement scheme that guarantees radiation safety for every location on the plane [17]. The authors in [18], [19], [20], [21], [22], [23] studied the safe charging problem of scheduling power of chargers so that the radiation anywhere will never exceed a September 11,2024 at 08:07:32 UTC from IEEE Xplore. Restrictions apply.

TABLE 1 Notations

<table><tr><td>Symbol</td><td>Meaning</td></tr><tr><td> $s_{i}$ </td><td> $i$ th wireless charger, or its position</td></tr><tr><td> $o_{i}$ </td><td> $i$ th wireless rechargeable device, or its position</td></tr><tr><td> $h_{i}$ </td><td> $i$ th obstacle, or the set of points inside it</td></tr><tr><td> $N_{s}$ </td><td>Number of wireless chargers to be deployed</td></tr><tr><td> $N_{s}^{q}$ </td><td>Number of  $q$ th type wireless chargers to be deployed</td></tr><tr><td> $N_{o}$ </td><td>Number of rechargeable devices to charge</td></tr><tr><td> $N_{h}$ </td><td>Number of obstacles</td></tr><tr><td> $\alpha_{s}^{i}$ </td><td>Charging angle of charger  $s_{i}$ </td></tr><tr><td> $\alpha_{o}^{i}$ </td><td>Receiving angle of device  $o_{i}$ </td></tr><tr><td> $\vec{r}_{s_{i}}$ </td><td>Unit vector of the orientation of charger  $s_{i}$ </td></tr><tr><td> $\vec{r}_{o_{i}}$ </td><td>Unit vector of the orientation of device  $o_{i}$ </td></tr><tr><td> $\phi_{s}^{i}$ </td><td>Orientation of charger  $s_{i}$ </td></tr><tr><td> $\phi_{o}^{i}$ </td><td>Orientation of device  $o_{i}$ </td></tr><tr><td> $P_{w}(.)$ </td><td>Charging power function</td></tr><tr><td> $P_{th}^{j}$ </td><td>Power threshold of  $o_{j}$  for charging utility function</td></tr><tr><td> $a_{ij}, b_{ij}$ </td><td>Constants in the charging model for  $s_{i}$  and  $o_{j}$ </td></tr><tr><td> $d_{min}^{i}$ </td><td>Nearest distance charger  $s_{i}$  can reach</td></tr><tr><td> $d_{max}^{i}$ </td><td>Farthest distance charger  $s_{i}$  can reach</td></tr><tr><td> $\mathcal{U}_{j}(.)$ </td><td>Charging utility function for device  $o_{j}$ </td></tr></table>

given threshold on the considered field. Second, the other works adopt the directional charging model for which the charging area of chargers and/or the power receiving area of devices are sectors. Dai et al. investigated the problem of detecting omnidirectional charging as well as the omnidirectional charging probability for randomly placed chargers [2]. They also considered the charger deployment problem for obstacle-free areas [3], [24] as well as the charging task scheduling problem [25]. Moreover, we launched the first study on heterogeneous wireless charger placement problem considering sector ring charging model and obstacles in the conference version of this paper [26].

Wireless Sensor Placement with Obstacles. problem of wireless sensor placement with obstacles is essentially linear and geometric, and thus fundamentally differs from ours that is nonlinear. Further, most literatures regarding this problem adopt omnidirectional sensing model and have no performance guarantee, and therefore cannot be adapted to address our problem with performance requirement. Agarwal et al. studied the problem of covering a 2D spatial region with some occluders using sensors, and gave a randomized algorithm [27]. In [28], [29], the authors explored the area coverage problem with polygon obstacles. Chang et al. considered arbitrary shaped obstacles and employed a simple grid division method for placement [30]. Saeed et al. developed a system that provides visual coverage of wide and oriented targets using camera-mounted drones with obstacles on the plane [31]. In particular, this system assumes a sector ring sensing model for drones rather than sector model adopted by other works.

Heterogeneous Wireless Sensor Networks. There are a bunch of related works regarding heterogeneous wireless sensor networks, but none of them are applicable to our problem due to their different problem settings compared with ours. Zhang et al. studied two-layered heterogeneous sensor networks which have better scalability and lower overall cost than homogeneous sensor networks [32]. Liang et al. proposed a heterogeneous and hierarchical wireless sensor network architecture [33]. Wang et al. investigated the coverage and energy consumption control issues in mobile heterogeneous wireless sensor networks using omnidirectional sensing model [34]. Gupta et al. derived probabilistic expressions to optimize the cost of random deployment adopting the 3D heterogeneous and directional sensing model for sensors [35]. Guo et al. proposed the necessary condition of the optimal sensor deployment and studied the dynamic sensor deployment in both homogeneous and heterogeneous wireless sensor networks with limited communication range for sensor nodes [36].

## 3 PROBLEM FORMULATION

## 3.1 Network Model and Charging Model with Obstacles

Suppose there are $N _ { o }$ heterogeneous directional rechargeable devices with fixed positions and orientations $O = \{ o _ { 1 }$ $O _ { 2 } , \ldots , O _ { N _ { o } } \}$ g on the 2D plane $\gamma .$ . Moreover, we have $N _ { s }$ hetero-<sup>g</sup>geneous directional chargers ${ \cal { S } } = \{ s _ { 1 } , s _ { 2 } , \ldots , s _ { N _ { s } } \}$ to be placed on the plane $\gamma .$ . Without confusion, we also use $o _ { i }$ and $s _ { i }$ <sup>g</sup>to represent their positions, respectively. Suppose there exist $N _ { h }$ static obstacles $H = \{ h _ { 1 } , h _ { 2 } , . . . , h _ { N _ { h } } \}$ that can be in arbitrary shapes on $\gamma .$ . Any charger or device cannot be <sup>g</sup>placed inside them, and charging power cannot penetrate these obstacles or reflect from the obstacles’ surface. We also use $h _ { i }$ to denote the set of points inside the ith obstacle. Some of the notations in this paper are shown in Table 1.

We establish our charging model based on our empirical studies and observations of real scenarios. According to work [2], [3], the charging and power receiving area can be modeled as sectors. However, the practical scenarios show that when a device is too close to a charger, the charger will stop to emit power for security reasons. For example, the commodity offthe-shelf TX91501 wireless charger produced by Powercast [4] can only provide charging power with distance at least 17 cm away when it is put at the height of 14 cm from the ground. Consequently, the (power) charging and receiving areas of chargers and devices can be modeled as sector rings in this study as shown in Fig. 1. In this figure, the charging area of charger $s _ { i }$ with unit orientation vector $\vec { r } _ { s _ { i } }$ is in the shape of a sector ring with a near radius of $d _ { m i n } ^ { i }$ and a far radius of $d _ { m a x } ^ { i }$ as well as a central angle of $\alpha _ { s } ^ { i } .$ . Similarly, the device $o _ { j }$ with unit orientation vector $\vec { r } _ { o _ { j } }$ <sup>a</sup>has a power receiving area in the shape of a sector ring with a near (far) radius of $d _ { m i n } ^ { i } ( d _ { m a x } ^ { i } )$ due to geometric symmetry and a central angle of $\alpha _ { o } ^ { \mathcal { I } }$ . Note that the above parameters may vary across different <sup>a</sup>chargers and different devices due to their heterogeneity, e.g., $\alpha _ { o } ^ { k }$ for device $o _ { k }$ is different from $\alpha _ { o } ^ { j }$ for device $o _ { j }$ in Fig. 1. By <sup>a a</sup>incorporating the widely accepted empirical charging model proposed in [2], [3] and following our experimental results, the charging power from charger $s _ { i }$ to device $o _ { j }$ considering obstacles is given by

$$
\begin{array}{l} P _ {w} (s _ {i}, \phi_ {s} ^ {i}, o _ {j}, \phi_ {o} ^ {j}) \\ = \left\{ \begin{array}{l l} \frac {a _ {i j}}{(\| s _ {i} o _ {j} \| + b _ {i j}) ^ {2}}, d _ {m i n} ^ {i} \leq \| s _ {i} o _ {j} \| \leq d _ {m a x} ^ {i}, \\ \overrightarrow {s _ {i} o _ {j}} \cdot \vec {r} _ {s _ {i}} - \| s _ {i} o _ {j} \| \cos (\alpha_ {s} ^ {i} / 2) \geq 0, \\ \overrightarrow {o _ {j} s _ {i}} \cdot \vec {r} _ {o _ {j}} - \| o _ {j} s _ {i} \| \cos (\alpha_ {o} ^ {j} / 2) \geq 0, \\ \text { and } \quad s _ {i} o _ {j} \cap h _ {k} = \emptyset , \forall k \in \{1, 2, \ldots , N _ {h} \}, \\ 0, \quad \text { otherwise }, \end{array} \right. \end{array}\tag{1}
$$

![](images/e605efbfb2f65eb2fa6f442b7299f86de1718febd9ae38f5858aacd7f00204cf.jpg)  
Fig. 3. Piecewise constant function approximation.

where $\phi _ { s } ^ { i }$ and $\phi _ { o } ^ { j }$ are orientations of $s _ { i }$ and $o _ { j } ,$ respectively, $a _ { i j }$ and $b _ { i j }$ <sup>f</sup>are two constants decided by charger/device hardware and surrounding environment, $\displaystyle | | s _ { i } o _ { j } | |$ denotes the distance between $s _ { i }$ and $\bar { o _ { j } } , \alpha _ { s } ^ { i }$ and $\alpha _ { o } ^ { j }$ denote the charging and receiving angles, ${ \vec { r } } _ { s _ { i } }$ and $\vec { r } _ { o _ { j } }$ <sup>a</sup>are the unit vectors denoting the orientations of $s _ { i }$ and $o _ { j } ,$ respectively. Note that the condition $s _ { i } o _ { j } \cap h _ { k } = \emptyset$ reflects the requirement that the line connecting s and $o _ { j }$ should not cross any obstacle, because otherwise the charging power from $s _ { i }$ could be blocked by some obstacle and no power is received by $o _ { j }$

When a device is charged by multiple chargers, we assume that the charging power is additive [2], [3], i.e.,

$$
P _ {w} (o _ {j}) = \sum_ {i = 1} ^ {N _ {s}} P _ {w} (s _ {i}, \phi_ {s} ^ {i}, o _ {j}, \phi_ {o} ^ {j}).\tag{2}
$$

## 3.2 Charging Utility Model

As any device has a power saturated state, we assume that there is a power threshold $P _ { t h } ^ { j }$ for device $o _ { j } ,$ , i.e., the harvest power by device $o _ { j }$ must be no more than $P _ { t h } ^ { j }$ regardless of the charging power of chargers. Accordingly, we define the charging utility model for device $o _ { j }$ as follows.

$$
\mathcal {U} _ {j} (x) = \left\{ \begin{array}{l l} \frac {1}{P _ {t h} ^ {j}} \cdot x, & x \leq P _ {t h} ^ {j}, \\ 1, & x > P _ {t h} ^ {j}, \end{array} \right.\tag{3}
$$

where x denotes the received charging power by device $o _ { j }$

## 3.3 Problem Formulation

Our target is to decide the strategies of chargers to maximize the overall charging utility for all devices on the plane . By assigning a uniform weight $\frac { 1 } {  { N _ { o } } }$ to the utility of each <sup>g</sup> device for normalization, HIPO can be formalized as the following P1 problem.

$$
\begin{array}{l} \text {(P1)} \max _ {s _ {i}, \phi_ {s} ^ {i}} \frac {1}{N _ {o}} \sum_ {j = 1} ^ {N _ {o}} \mathcal {U} _ {j} (\sum_ {i = 1} ^ {N _ {s}} P _ {w} (s _ {i}, \phi_ {s} ^ {i}, o _ {j}, \phi_ {o} ^ {j})), \\ \text {s.t.} s _ {i} \in \gamma \text {and} \phi_ {s} ^ {i} \in [ 0, 2 \pi). \end{array}\tag{4}
$$

We have the following theorem to indicate the hardness of our problem HIPO.

Theorem 3.1. The HIPO problem is NP-hard.

all chargers and devices as well as there is no obstacle, so that each charger has a disk-shaped charging area with a constant radius D. Moreover, we suppose that once the device is covered by a charger, i.e., the device falls in the charging disk of a charger, the charging utility for the device becomes 1. Therefore, each device can be seen as a point, and HIPO changes to the problem of covering most points for $N _ { o }$ points in the area by $N _ { s }$ disks with the same radius D. This problem is exactly the partial disk covering problem which is proved to be NP-complete [37]. In general, we can prove the NP-hardness of HIPO by reducing from the partial disk covering problem. tu

## 4 SOLUTION

In this section, we propose a $\frac { 1 } { 2 } - \epsilon$ approximation algorithm to address HIPO, which mainly contains three steps. First, as the charging power is nonlinear with distance, blocked by obstacles, and varies for heterogenous chargers or devices, given a type of chargers, we approximate the charging power of chargers with respect to a device by a piecewise constant function, and divide the whole area into several feasible geometric areas by considering the blockage effect of obstacles. By doing so, the approximated power at any point in a feasible geometric area is constant. Further, by enumerating all types of chargers, we divide the area for multiple times, and obtain the so-called multi-feasible geometric areas. Second, we propose a Practical Dominating Coverage Set (PDCS) extraction method to confine the continuous solution space for strategies in each feasible geometric area, so that the number of candidate strategies becomes limited. Third, we reformulate the problem into maximizing a monotone submodular optimization problem subject to a partition matroid, which allows a greedy algorithm to solve with performance guarantee.

## 4.1 Area Discretization with Obstacles and Heterogeneity of Chargers and Devices

## 4.1.1 Piecewise Constant Function Approximation

Let $P _ { w } ^ { i j } ( d )$ denote the charging power from $s _ { i }$ to $o _ { j }$ at distance d. We use a piecewise constant function to approximate the power as follows.

$$
\widetilde {P _ {w} ^ {i j}} (d) = \left\{ \begin{array}{l} P _ {w} ^ {i j} (l (k)), l (k - 1) <   d \leq l (k) \\ \quad (k = k _ {0} ^ {i j}, k _ {0} ^ {i j} + 1, \ldots , K ^ {i j}), \\ 0, \qquad d <   d _ {m i n} ^ {i} \text {or} d > d _ {m a x} ^ {i}, \end{array} \right.\tag{5}
$$

where $l ( K ^ { i j } ) = d _ { m a x } ^ { i }$ and $k _ { 0 } ^ { i j }$ is a positive integer such that $l ( k _ { 0 } ^ { i j } - 1 ) < d _ { m i n } ^ { i } \leq l ( k _ { 0 } ^ { i j } )$

We have the following lemma to bound its error.

Lemma 4.1. For charger $s _ { i }$ and device $o _ { j } ,$ , setting $l ( K ^ { i j } ) = d _ { m a x } ^ { i } ,$ $l ( k ) = b _ { i j } ( ( 1 + \epsilon _ { 1 } ) ^ { k / 2 } - 1 )$ Þ, where $k = k _ { 0 } ^ { i j } , k _ { 0 } ^ { i j } + 1 , . . . , K ^ { i j } - 1$ (therefore $\begin{array} { r } { K ^ { i j } = \lceil \frac { \ln ( a _ { i j } / ( b _ { i j } ^ { 2 } P _ { w } ( d _ { m a x } ^ { i } ) ) ) } { \ln ( 1 + \epsilon _ { 1 } ) } \rceil } \end{array}$ , and $\begin{array} { r } { k _ { 0 } ^ { i j } = \lceil \frac { 2 \ln ( d _ { m i n } ^ { i } / b _ { i j } + 1 ) } { \ln ( 1 + \epsilon _ { 1 } ) } \rceil ) , } \end{array}$ we have the approximation error as

$$
1 \leq \frac {P _ {w} ^ {i j} (d)}{\widetilde {P _ {w} ^ {i j}} (d)} \leq 1 + \epsilon_ {1}, d _ {m i n} ^ {i} \leq d \leq d _ {m a x} ^ {i}.\tag{6}
$$

Proof. Consider the special case in HIPO where $\alpha _ { s } ^ { i } =$ As shown in Fig. 3, the charging area of charger $s _ { i }$ is $\alpha _ { o } ^ { i } = 2 \pi , d _ { m i n } ^ { i } = 0 , d _ { m a x } ^ { i } = D ( D$ <sup>a</sup>is a positive constant) for divided into three subareas, each in which the charging <sup>o</sup> p <sup>min</sup> <sup>max</sup> Authorized licensed use limited to: SHANGHAI UNIVERSITY. Downloaded on September 11,2024 at 08:07:32 UTC from IEEE Xplore. Restrictions apply.

![](images/472aa84d194ee0c8e5dc2a7b5d1270fa86aaf49c37d55eb91cfb04463ce5b93b.jpg)  
Fig. 4. Area discretization.

power is approximated as a constant as the horizontal segments show.

## 4.1.2 Area Discretizion

In this subsection, we show how to divide the whole 2D area into multi-feasible geometric areas. We first introduce geometric area discretization and feasible geometric area discretization to assist understanding.

Geometric Area Discretization. First, we discuss the case of only a single type of chargers to be placed on the plane without obstacles. Due to geometric symmetry, if a device is facing a charger at distance $d ,$ then the charger facing the device is also at distance d. Thus, we can divide the area with the power receiving area of devices by $l ( k _ { 0 } ^ { i j } ) , \dots , l ( K ^ { i j } )$ , and each subarea is called a geometric area. For example, as shown in Fig. 4a, the power receiving area is divided into 12 geometric areas for the charger type of $s _ { 1 } \ \left( s _ { 1 } \right.$ is not drawn in this subfigure).

Feasible Geometric Area Discretization. Then we further consider the area including obstacles. If the obstacles are positioned in the power receiving area, the area is further divided by the obstacles and corresponding holes of devices, that is, there are two more cases which are infeasible to place chargers for the specified device: the area inside the obstacles and the area in which the placed chargers cannot cover the device. We define the feasible geometric area for device $o _ { i }$ as the area in which the placed chargers can provide device $o _ { i }$ with constant non-zero charging power. We say the area is feasible (or infeasible) for $o _ { i }$ to describe the fact that the placed chargers in that area can (or cannot) charge $o _ { i }$ with non-zero power for simple. Moreover, it should be noticed that the infeasible area for $o _ { i }$ may be feasible for $o _ { j } ,$ thus the feasible geometric area discretizing for the whole area also requires the boundaries of holes and obstacles. As shown in Fig. 4b, the power receiving area of $o _ { 1 }$ and $O _ { 2 }$ is further divided by the boundaries of $h _ { 1 }$ and holes of the devices based on Fig. 4a according to the charger type of $s _ { 1 }$ . Since $h _ { 1 }$ completely shields the further power receiving area of $O 1$ and $O _ { 2 } ,$ , these geometric areas can be ignored. Thus, we get only six feasible geometric areas in total for simple. Moreover, the feasible geometric area 3 is a hole of device $o _ { 1 }$ . It should be considered as there may exist power receiving areas of other devices cover it. Charger $s _ { 1 }$ can provide device $o _ { 1 }$ with power $P _ { w } ( l ( k _ { 0 } ^ { 1 1 } + 1 ) ,$ Þ.

Multi-Feasible Geometric area Discretization. Next, we consider the heterogeneity of chargers. Clearly, the area discretization varies for different types of chargers. Since the charging power is linearly additive, we can divide the area into feasible geometric area for several times by different charging parameters, and consider the strategies in each feasible geometric area independently. Thus, multi-feasible geometric area discretization is to discretize the area to get feasible geometric areas for several versions based on different parameters of heterogeneous chargers. Fig. 4c shows another division of receiving area for the charger type of $s _ { 2 }$ different from 4(b). In this case, s<sub>2</sub> can provide $O _ { 1 }$ with power $P _ { w } ( l ( k _ { 0 } ^ { 2 1 } ) )$ Þ.

We have the following lemmas for multi-feasible geometric area discretization.

Lemma 4.2. Let $\widetilde { P _ { w } } ( o _ { j } )$ denote the approximated charging power received by device $o _ { j }$ in its multi-feasible geometric areas. Then, $\widetilde { P _ { w } } ( o _ { j } ) = \stackrel { \cdot } { 0 } i f { P _ { w } } ( o _ { j } ) = 0 ;$ otherwise, the approximation error is

$$
1 \leq \frac {P _ {w} (o _ {j})}{\widetilde {P _ {w}} (o _ {j})} \leq 1 + \epsilon_ {1}.\tag{7}
$$

Proof. According to Lemma 4.1, $\begin{array} { r } { 1 \le \frac { P _ { w } ( o _ { j } ) } { \sim } = } \end{array}$ $\begin{array} { r } { \sum _ { i = 1 } ^ { N _ { s } } P _ { w } ( s _ { i } , \phi _ { s } ^ { i } , o _ { j } , \phi _ { o } ^ { j } ) } \\ { \sum _ { i = 1 } ^ { N _ { s } } \widetilde { P } _ { w } ( s _ { i } , \phi _ { s } ^ { i } , o _ { j } , \phi _ { o } ^ { j } ) \leq \frac { ( 1 + \epsilon _ { 1 } ) \sum _ { i = 1 } ^ { N _ { s } } \widetilde { P } _ { w } ( s _ { i } , \phi _ { s } ^ { i } , o _ { j } , \phi _ { o } ^ { j } ) } { \sum _ { i = 1 } ^ { N _ { s } } \widetilde { P } _ { w } ( s _ { i } , \phi _ { s } ^ { i } , o _ { j } , \phi _ { o } ^ { j } ) } \leq 1 + \epsilon _ { 1 } } \end{array}$ tu

Lemma 4.3. Let $\mathcal { U } _ { j } ( x )$ denote the utility function for device $o _ { j }$ as Equation (3) shows. Then, $\mathcal { U } _ { j } ( \widetilde { P _ { w } } ( o _ { j } ) ) \dot { = } 0 \ i f \mathcal { \bar { U } } _ { j } ( P _ { w } ( o _ { j } ) ) \stackrel {  } { = } 0 ;$ otherwise, the approximation error is

$$
1 \leq \frac {\mathcal {U} _ {j} (P _ {w} (o _ {j}))}{\mathcal {U} _ {j} (\widetilde {P _ {w}} (o _ {j}))} \leq 1 + \epsilon_ {1}.\tag{8}
$$

Proof. Since $P _ { w } ( o _ { j } ) \ge \widetilde { P _ { w } } ( o _ { j } )$ Þ, there are only three cases to be considered:

1) $\widetilde { P _ { w } } ( o _ { j } ) \leq P _ { w } ( o _ { j } ) \leq P _ { t h } ^ { j } ;$

2) $\widetilde { P _ { w } } ( o _ { j } ) \leq P _ { t h } ^ { j } \leq P _ { w } ( o _ { j } ) ;$

3) $P _ { t h } ^ { j } \le \widetilde { P _ { w } } ( o _ { j } ) \le P _ { w } ( o _ { j } ) .$

For Case $1 ) , \mathcal { U } _ { j } ( P _ { w } ( o _ { j } ) ) = P _ { w } ( o _ { j } ) / P _ { t h } ^ { j }$ and $\mathcal { U } _ { j } ( \widetilde { P _ { w } } ( o _ { j } ) ) =$ $\widetilde { P _ { w } } ( o _ { j } ) / { P _ { t h } ^ { j } }$ . It is obvious that the conclusion stands according to Lemma 4.2. For Case 2), $\mathcal { U } _ { j } ( \widetilde { P _ { w } } ( o _ { j } ) ) =$ $\widetilde { P _ { w } } ( o _ { j } ) / \widetilde { P _ { t h } ^ { j } }$ and $\mathcal { U } _ { j } ( P _ { w } ( o _ { j } ) ) = 1 \leq P _ { w } ( o _ { j } ) / P _ { t h } ^ { j } ,$ , thus, the conclusion still stands. For Case 3), $\mathcal { U } _ { j } ( \tilde { P _ { w } } ( o _ { j } ) ) =$ $\mathcal { U } _ { j } ( P _ { w } ( o _ { j } ) ) = 1$ , so the conclusion also comes. In all, the conclusion stands. tu

Suppose that each obstacle can be expressed by a poly gon with no more than c edges and we have the following lemma to describe the number of feasible geometric areas.

Lemma 4.4. The number of feasible geometric areas for each type of chargers is $O ( N _ { o } { } ^ { 2 } \epsilon _ { 1 } ^ { - 2 } \dot { N _ { h } } ^ { 2 } c ^ { 2 } )$ , where c is the maximum number of edges of these obstacles.

Proof. We consider a relaxed bound which also includes the number of divided infeasible areas. In general, there are $O ( \epsilon _ { 1 } ^ { - 1 } )$ disjoint sector rings in the power receiving area of one device. Suppose the worst case that all the obstacles are in the power receiving area of one device, and it generates many holes. Since the holes are generated by connecting the device and the vertices of the obstacles, we connect the device with all the vertices and extend the line to intersect with the farthest boundary of the power receiving area. Thus, the power receiving area is divided into $O ( \epsilon _ { 1 } ^ { - 1 } ( 1 + N _ { h } c ) )$ Þ sector rings. Moreover, there are $N _ { o }$ devices, which generates $O ( N _ { o } \bar { \epsilon } _ { 1 } ^ { - 1 } ( 1 + N _ { h } c ) )$ areas in total. Plus considering $N _ { h }$ obstacles, the total number of shapes to intersect is $O ( N _ { o } \epsilon _ { 1 } ^ { - 1 } ( 1 + N _ { h } c ) + N _ { h } )$ Therefore, the plerePestrictions

<sup>o 1</sup>   <sup>h</sup>  <sup>h</sup> Authorized licensed use limited to: SHANGHAI UNIVERSITY. Downloaded on September 11,2024 at 08:07:32 UTC from IEEE Xplore. Restrictions apply.

![](images/8c7e0d25a6dc8741a5dc3f6f200041a3e7d6a619f92bbbc16c4ca9ec0b927b92.jpg)

![](images/7bba7f710266e018a92351b0ae0f8ab727fb9e27b65ecf1f6d343f46c6aa3f02.jpg)  
Fig. 5. A toy example of point case.

number of the divided subareas is squared, i.e., $O ( ( N _ { o } \epsilon _ { 1 } ^ { - 1 } ( 1 + N _ { h } c ) + N _ { h } ) ^ { 2 } )$ , and this can be simplified as $O ( N _ { o } { } ^ { 2 } \epsilon _ { 1 } ^ { - 2 } N _ { h } { } ^ { 2 } c ^ { 2 } )$ tu

Note that if the boundaries of obstacles are continuous curves rather than segments, our algorithm still works and its achieved approximation ratio remains, but its time complexity is no longer bounded. As this is out of the focus of this paper, we omit it here to save space.

## 4.2 Practical Dominating Coverage Set (PDCS) Extraction

After area discretization, the whole 2D area is divided into multi-feasible geometric areas where the placed chargers emit constant approximated charging power to devices anywhere in each feasible geometric area. Thus, we only need to consider strategies in each feasible geometric area. In this subsection, we propose the Practical Dominating Coverage Set (PDCS) extraction algorithm to extract strategies in each feasible geometric area. Essentially, the PDCS extraction algorithm exploits the rotational symmetry of the sector ring charging area to address its non-convexity. Note that we perform the algorithm multiple times for multi-feasible geometric areas.

## 4.2.1 Preliminaries

We first give some definitions.

Definition 4.1 (Dominance). Suppose there are two strategies $\langle s _ { i } , \phi _ { s } ^ { i } \rangle$ and $\langle s _ { j } , \phi _ { s } ^ { j } \rangle$ of the same type of chargers, and their cor-<sup>f f</sup>responding covered device sets are $O _ { i }$ and $O _ { j } ,$ respectively. If $O _ { i } \subset O _ { j } ,$ we say $\langle s _ { j } , \phi _ { s } ^ { j } \rangle$ dominates $\langle s _ { i } , \phi _ { s } ^ { i } \rangle$

Definition 4.2 (Practical Dominating Coverage Set). Suppose there is a strategy $\langle s _ { i } , \phi _ { s } ^ { i } \rangle$ with covered device set $O _ { i }$ <sup>f</sup>If there doesn’t exist a covered set $O _ { j }$ with strategy $\langle s _ { j } , \phi _ { s } ^ { j } \rangle$ such that $\langle s _ { j } , \phi _ { s } ^ { j } \rangle$ dominates $\langle s _ { i } , \phi _ { s } ^ { i } \rangle$ , then we say $O _ { i }$ <sup>f</sup>is a Practi-<sup>f f</sup>cal Dominating Coverage Set (PDCS).

Definition 4.3 (Candidate Covered Set of Devices). The devices in the Candidate Covered Set of Devices O<sup>^</sup> of subarea $\gamma _ { i }$ are those devices which can be charged by chargers <sup>g</sup>located in $\gamma _ { i }$ with non-zero power.

![](images/da19cb32d4091c37f7ec4d4ce614327dc9be5e57e9766c742171350a6e72bc59.jpg)

![](images/f6c517403df47689ca38aa3d4143470a36d1008748fc6511638f577d0a667ba7.jpg)

![](images/ffa5c94cdbdc4cd48fa1e10969e6b1d2309e5aa260a24840ff3550f4ff06fd3e.jpg)

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 1. PDCS Extraction for Point Case
Input: Reduced point  $\gamma_{i}$  and its candidate covered set of devices  $\hat{O}$ 
Output: PDCSs and their corresponding strategies
1: Place a charger at the point  $\gamma_{i}$  and compute the angle between the line connecting the charger and each candidate device and  $0^{\circ}$  orientation. Sort the devices by their angles.
2: Initialize the orientation of the charger  $\theta = 0^{\circ}$.;
3: while  $\theta &lt; 360^{\circ}$  do
4: Rotate the charger anticlockwise until a device is going to fall out of the charging area, and add the PDCS and corresponding strategy into the candidate solution set.;
5: Rotate the charger anticlockwise until a device is added to the charging area.
</div>

## 4.2.2 PDCS Extraction for Point Case

First, we consider a special case where a feasible geometric area $\gamma _ { i }$ is reduced to a point which is still denoted by $\gamma _ { i } .$ <sup>g g</sup>Algorithm 1 gives the details of the algorithm, and its basic idea is to rotate the charger at point $\gamma _ { i }$ for 360<sup></sup> and extract <sup>g</sup>the PDCSs. Fig. 5 shows a toy example for Algorithm 1. First, the charger is initialized at the position $\gamma _ { i }$ with orientation $0 ^ { \circ }$ <sup>g</sup>. We rotate the charger anticlockwise and add $O _ { 2 } ,$ ; o<sub>3</sub> in the charging area with $O _ { 1 }$ going to fall out, and obtain the PDCS $\left\{ o _ { 1 } , o _ { 2 } , o _ { 3 } \right\}$ as shown in Fig. 5b. Next, continue to rotate the charger and add new devices into the charging area. Rotate it until a device, say $O _ { 3 } ,$ is going to fall out, and obtain the PDCS $\{ o _ { 3 } , o _ { 4 } \}$ as shown in Fig. 5c. Repeat the above operations and we can get the covered device set $\left\{ o _ { 5 } , o _ { 6 } \right\}$ in Fig. 5d as a PDCS. When the charger has rotated for 360<sup></sup>, this operation terminates.

## 4.2.3 PDCS Extraction for Area Case

Next, we discuss the general area case, and show the algorithm in Algorithm 2. See Fig. 6, suppose there are six devices in the candidate covered set of devices of feasible geometric area $\gamma _ { i } ,$ which are classified into three types: $\{ o _ { 1 } , o _ { 2 } \} , \{ o _ { 3 } , o _ { 4 } \}$ and $\left\{ o _ { 5 } , o _ { 6 } \right\}$ . First, we draw a straight line through each pair of devices, say $o _ { 1 }$ and $O _ { 2 } ,$ , and put a charger at the intersection points of the feasible geometric area boundaries with the charger’s clockwise boundary crossing the two devices as shown in Fig. 6b, and thus obtain two candidate PDCS $\{ o _ { 1 } , o _ { 2 } , o _ { 3 } , o _ { 6 } \}$ and $\left\{ o _ { 1 } , o _ { 2 } , o _ { 3 } , o _ { 4 } , o _ { 5 } , o _ { 6 } \right\}$ as well as their corresponding strategies $\langle s _ { 1 } , \phi _ { s } ^ { 1 } \rangle$ i and $\langle s _ { 2 } , \phi _ { s } ^ { 2 } \rangle$ i. Second, we draw f f  September 11,2024 at 08:07:32 UTC from IEEE Xplore. Restrictions apply.

![](images/436694e12b5a65ef7ac61072e7ea90f53b7b79af0b30462d8ac8137a5a136ce2.jpg)  
Fig. 7. Critical conditions.

arcs through each pair of devices, say $o _ { 3 }$ and $O _ { 4 } ,$ , with circumferential angle being the charging angle of the current type of chargers, say $\alpha _ { s } ^ { k } ,$ , and put a charger at the intersection points <sup>a</sup>of the feasible geometric area boundaries with the charger’s two line boundaries crossing the two devices, respectively, as shown in Fig. 6c. We then obtain candidate PDCS $\{ o _ { 3 } , o _ { 4 } , o _ { 5 } , o _ { 6 } \}$ and strategies $\left. s _ { 3 } , \phi _ { s } ^ { 3 } \right.$ and $\langle s _ { 4 } , \phi _ { s } ^ { 4 } \rangle$ . Note that the <sup>f f</sup>gray charging area in all the subfigures in Fig. 6 is only for the current discussed type of devices, thus there may exist devices in other types like $o _ { 5 }$ and $o _ { 6 }$ which are not included in the gray charging area in Fig. 6c but can still be charged. Third, we randomly select a point on the boundary of the feasible geometric area and perform PDCS extraction algorithm for point case, as shown in Fig. 6d. Finally, we check all the obtained candidate PDCSs and their corresponding strategies, and only retain the true PDCSs by comparing them and their corresponding strategies. In this example, we reserve $\{ o _ { 1 } , o _ { 2 } , o _ { 3 } , o _ { 4 } , o _ { 5 } , o _ { 6 } \}$

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 2. PDCS Extraction for Area Case
Input: Feasible geometric area $\gamma_{i}$ and its candidate covered set of devices $\hat{O}$
Output: PDCSs and their corresponding strategies
1: for all pairs of devices in $\hat{O}$, say $o_{i}$ and $o_{j}$ do
2: Draw a straight line crossing $o_{i}$ and $o_{j}$, and intersect the boundaries of the feasible geometric area.
3: Put the charger at the intersection point, and let the clockwise boundary cross $o_{i}$ and $o_{j}$.
4: Add the PDCS and the corresponding strategy under this setting into the solution set.
5: Draw arcs crossing $o_{i}$ and $o_{j}$ with circumferential angle $\alpha_{s}^{k}$, and intersect the boundaries of the feasible geometric area.
6: Put the charger at the intersection point, and let the two line segment boundaries cross $o_{i}$ and $o_{j}$.
7: Add the PDCS and the corresponding strategy under this setting into the solution set.
8: Select a point on the boundaries of the feasible geometric area randomly and perform PDCS extraction algorithm for point case in Algorithm 1.
9: Filter the PDCSs and remove the subsets and their corresponding strategies.
</div>

Next, we define a transformation of the strategy.

Definition 4.4 (Projection). Keep the orientation of a strategy $\langle s _ { i } , \phi _ { s } ^ { i } \rangle$ fixed and move the strategy’s position along the reverse <sup>f</sup>direction of its orientation until it reaches the boundary of the current feasible geometric area.

The projection operation is shown in Fig. 7a. We can see that after projection, the strategy can cover not only $o _ { 1 }$ but $O _ { 2 }$ and $o _ { 3 }$ as well. Generally, we have the following lemma.

Lemma 4.5. If $\langle s _ { j } , \phi _ { s } ^ { j } \rangle$ is the projection of $\langle s _ { i } , \phi _ { s } ^ { i } \rangle$ , then $\langle s _ { j } , \phi _ { s } ^ { j } \rangle$ <sup>f</sup>must either dominate or be equivalent to $\langle s _ { i } , \phi _ { s } ^ { i } \rangle$

By Lemma 4.5, we have the following corollary.

Corollary 4.1. Considering PDCSs with corresponding strategies on the boundaries of a feasible geometric area is equivalent to considering that in the whole area.

Further, let G denote the output set of strategies of Algorithm 2. We have the following theorem.

Theorem 4.1. Given any strategy $\langle s _ { i } , \phi _ { s } ^ { i } \rangle$ , there must exist a strategy $\langle s _ { j } , \phi _ { s } ^ { j } \rangle \in \Gamma$ such that $\langle s _ { j } , \phi _ { s } ^ { j } \rangle$ either dominates or is equivalent to $\langle s _ { i } , \phi _ { s } ^ { i } \rangle$

Proof. For an arbitrary strategy $\langle s _ { i } , \phi _ { s } ^ { i } \rangle$ in a feasible geomet-<sup>f</sup>ric area, we do the following three operations and obtain a new strategy $\langle s _ { j } , \phi _ { s } ^ { j } \rangle$ :

1) Perform the projection transformation until the strategy’s position reaches the boundary of the area.

2) Keep the strategy’s position fixed and rotate its orientation anticlockwise until a device is going to fall out of the clockwise boundary of the strategy’s charging area.

3) Keep the clockwise boundary of the charger crossing the device which is going to fall out and move the charger along the feasible geometric area’s boundaries until another device is going to fall out of the charging area. If no other device is going to fall out, then stop the operation.

It is obvious that $\langle s _ { j } , \phi _ { s } ^ { j } \rangle$ either dominates or is equivalent to $\langle s _ { i } , \phi _ { s } ^ { i } \rangle$ <sup>f</sup>according to Corollary 4.1.

<sup>f</sup>Next, we will prove that the solution set obtained by Algorithm 2 corresponds to the set after the above three operations. The cases of the covered device set and corresponding strategy include five critical conditions after those three operations:

1) Another device touches the clockwise boundary of the charging area (Fig. 7b).

2) Another device touches the anticlockwise boundary of the charging area (Fig. 7c).

3) Another device touches the arc with distance $d _ { m i n } ^ { i }$ to the charger (Fig. 7d).

4) Another device touches the arc with distance $d _ { m a x } ^ { i }$ to the charger (Fig. 7e).

5) None of the other devices touches the boundary of the charging area (Fig. 7f).

Cases 1) and 2) are those cases that the device is going to fall out of the charging area, while the device in the left three cases will never fall out. Cases 3) and 4) are not critical conditions that the device is going to fall out, ptember 11,2024 at 08:07:32 UTC from IEEE Xplore. Restrictions apply.

![](images/3e2733300bbafc3f1c294623297cd2290f807fe9fc777b539bb1a28610e4ce9c.jpg)  
Fig. 8. Arc explanation.

since if the device falls out, the feasible geometric area must be further divided into more feasible geometric areas. As shown in Figs. 8a and 8b, the devices $o _ { 3 }$ and $O _ { 4 }$ fall out of the charging area by the nearest arc boundary and the farthest arc boundary, respectively, when the charger moves on the boundaries of feasible geometric area $\gamma _ { i }$ . We show that the feasible geometric area $\gamma _ { i }$ can <sup>g g</sup>be divided into two feasible geometric areas by the arc centering at $o _ { 3 }$ and $O _ { 4 }$ with radius $d _ { m i n } ^ { i }$ and $d _ { m a x } ^ { i } ,$ say $A _ { 1 }$ and $A _ { 2 } ,$ , respectively.

We can see that Step 2-4 and 5-7 in Algorithm 2 correspond to the first and the second cases, respectively. And for the last three cases, arbitrary positions on the boundaries of the feasible geometric area are equivalent, thus, leads to Step 8 of the PDCS extraction algorithm for point case in Algorithm 2. Therefore, the set of strategies obtained by Algorithm 2 is just the set of $\langle s _ { j } , \phi _ { s } ^ { j } \rangle$ . Since $\langle s _ { j } , \phi _ { s } ^ { j } \rangle$ either dominates or is equivalent to $\langle s _ { i } , \phi _ { s } ^ { i } \rangle$ , the <sup>f</sup>result follows. tu

## 4.3 Problem Reformulation

After performing Algorithm 2 for several times for multifeasible geometric areas, we obtain the PDCSs and their corresponding strategies. For each strategy, we can compute the charging power and charging utility for each device. Let G denote the strategy set, $\Gamma _ { q }$ denote the strategy set of qth type of chargers, and $x _ { i }$ be the indicator which denotes whether the ith strategy is selected. The problem P1 in Section 3.3 can be reformulated as the combinatorial problem as follows:

$$
\begin{array}{l} \text {(P2)} \max \frac {1}{N _ {o}} \sum_ {j = 1} ^ {N _ {o}} \mathcal {U} _ {j} \Bigg (\sum_ {\langle s _ {i}, \phi_ {s} ^ {i} \rangle \in \Gamma} x _ {i} \widetilde {P _ {w}} (s _ {i}, \phi_ {s} ^ {i}, o _ {j}, \phi_ {o} ^ {j}) \Bigg), \\ \text {s.t.} \sum_ {\langle s _ {i}, \phi_ {s} ^ {i} \rangle \in \Gamma_ {q}} x _ {i} = N _ {s} ^ {q}, q = 1, \ldots , \mathcal {Q}, x _ {i} \in \{0, 1 \}. \end{array}\tag{9}
$$

We will further reformulate the problem and obtain the final solution with a constant performance guarantee. First, we give some definitions.

Definition 4.5([38] Monotone submodular set function). Let S be a finite ground set. A real-valued set function $f : 2 ^ { S } $ R is normalized, monotonic and submodular if and only if it satisfies the following conditions, respectively: (1) $f ( { \bar { \varnothing } } ) { \stackrel { . } { = } } 0 ; ~ ( 2 ) ~ { \bar { f } } ( A \cup \{ e \} ) - f ( A ) \geq 0$ for any $A \subseteq { \bar { S } }$ and $e \in S \backslash A ; ( 3 ) ~ f ( A \cup \{ e \} ) - f ( A ) \geq f ( B \cup \{ e \} ) - f ( B )$ for any $A \subseteq B \subseteq S$ and $e \in S \backslash B$

Definition 4.6 ([38] Matroid). A Matroid M is a strategy $\mathcal { M } = ( S , L )$ where S is a finite ground set, $L \subseteq 2 ^ { S } i s ^ { \overline { { \alpha } } }$ Authorized licensed use limited to: SHANGHAI UNIVERSITY. Downloaded collection of independent sets, such that: (1) $\varnothing \in L ;$ (2) if $X \subseteq Y \in L ,$ then $X \in L ; \ ( 3 )$ if $X , Y \in L ,$ , and $| X | < | Y | ,$ then $\exists y \in Y \backslash X , X \cup \{ y \} \in L$

Definition 4.7 ([38] Partition matroid). Given $\textstyle S = \bigcup _ { i = 1 } ^ { k } S _ { i } ^ { \prime }$ is the disjoint union of k sets, $l _ { 1 } , l _ { 2 } , \ldots , l _ { k }$ are positive integers, a partition matroid $\mathcal { M } = ( S , \mathcal { T } )$ is a matroid where ${ \mathcal { T } } \doteq \{ X \subset S : | X \cap S _ { i } ^ { \prime } | \leq l _ { i } f o r i \in [ { \dot { k } } ] \}$

Generally, the obtained strategy set G by Algorithm 2 applying to multi-feasible geometric areas can be defined as the disjoint union of the Q strategy sets of different types of chargers, i.e., $\Gamma = \textstyle \bigcup { { \mathcal { Q } } _ { q = 1 } \Gamma _ { q } , }$ , and thereby, define the partition matroid $\mathcal { M } = ( \Gamma , \mathcal { T } )$ with ${ \mathcal { T } } = \{ X \subset { \mathbf { \bar {Gamma } } } \} : | X \cap \Gamma _ { q } | \leq N _ { s } ^ { q }$ for $i \in [ \mathcal { Q } ] \}$ g. Based on these definitions, problem $P 2$ in Equation (9) can be rewritten as

$$
\begin{array}{l l} \text {(P3)} & \max _ {s _ {i}, \phi_ {s} ^ {i}} f (X) = \frac {1}{N _ {o}} \sum_ {j = 1} ^ {N _ {o}} \mathcal {U} _ {j} \Bigg (\sum_ {\langle s _ {i}, \phi_ {s} ^ {i} \rangle \in X} \widetilde {P _ {w}} (s _ {i}, \phi_ {s} ^ {i}, o _ {j}, \phi_ {o} ^ {j}) \Bigg), \\ & \text {s.t.} \quad s _ {i} \in \gamma , \quad \phi_ {s} ^ {i} \in [ 0, 2 \pi), \\ & \qquad X \in L, \\ & \qquad L = \{X \subseteq \Gamma : | X \cap \Gamma_ {q} | \leq N _ {s} ^ {q} \}. \end{array}\tag{10}
$$

For the problem P3 shown in Equation (10), we have the following critical lemma.

Lemma 4.6. The objective function $f ( X )$ in Equation (10) is a monotone submodular function, and the constraint is a partition matroid constraint.

Proof. To check whether $f ( X )$ in Equation (10) is a monotone submodular function, we can check whether it satisfies the three requirements in Definition 4.5. First, it is obvious that $f ( \boldsymbol { X } ) = 0$ when $X = \emptyset$ since there is no charger to provide charging utility. Second, it is also clear that when a new strategy is selected, the charging utility increases since the charging utility function $\bar { \mathcal { U } } _ { j } ( \cdot )$ defined in Equation (3) is non-decreasing. Third, we define $\begin{array} { r } { g ( X , \hat { j } ) = \sum _ { \langle s _ { i } , \phi _ { \mathrm { s } } ^ { i } \rangle \in X } \widetilde { P _ { w } } ( s _ { i } , \phi _ { s } ^ { i } , o _ { j } , \phi _ { o } ^ { j } ) } \end{array}$ and obviously $g ( \cdot , j )$ is <sup>f f f</sup>non-decreasing since the charging power is non-decreasing with more chargers. Thus, for strategy sets $A \subseteq B \subseteq \Gamma$ and a strategy $e \in \Gamma \backslash B ,$ , it can be seen that $g ( A , j ) \leq$ $\{ g ( A \cup \{ e \} , j ) , g ( B , j ) \} \leq g ( B \cup \{ e \} , j )$ As a result, we have

$$
\begin{array}{l} \left[ \mathcal {U} _ {j} (g (A \cup \{e \}, j)) - \mathcal {U} _ {j} (g (A, j)) \right] \\ - \left[ \mathcal {U} _ {j} (g (B \cup \{e \}, j)) - \mathcal {U} _ {j} (g (B, j)) \right] \geq 0, \end{array}\tag{11}
$$

since for any $0 \leq x _ { 1 } \leq x _ { 2 }$ and $\Delta x \ge 0$

$$
\left[ \mathcal {U} _ {j} (x _ {1} + \Delta x) - \mathcal {U} _ {j} (x _ {1}) \right] - \left[ \mathcal {U} _ {j} (x _ {2} + \Delta x) - \mathcal {U} _ {j} (x _ {2}) \right] \geq 0,\tag{12}
$$

and

$$
g (A \cup \{e \}, j) - g (A, j) = g (B \cup \{e \}, j) - g (B, j) = g (\{e \}, j).\tag{13}
$$

![](images/b03f4e252699a67415e5eb501e2d61053adaa89366cfba01edad98a04fbbd313.jpg)  
Fig. 9. Distributed PDCS extraction.

Therefore, we have

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
$[f(A \cup \{e\}) - f(A)] - [f(B \cup \{e\}) - f(B)] = \frac{1}{N_o} \sum_{j=1}^{N_o} \left\{ \left[ \mathcal{U}_j(g(A \cup \{e\}, j)) - \mathcal{U}_j(g(A, j)) \right] - \left[ \mathcal{U}_j(g(B \cup \{e\}, j)) - \mathcal{U}_j(g(B, j)) \right] \right\}$ $\geq 0$.
</div>

(14)

Moreover, it is obvious that the constraint in Equation (10) is a partition matroid constraint. tu

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 3. Strategy Selection for Heterogeneous Chargers
Input: Number of chargers  $N_{s}^{q}$  for the qth type, candidate strategy set  $\Gamma_{q}$  for the qth type, objective function  $f(X)$ 
Output: Selected strategy set  $X_{q}$  ( $1 \leq q \leq Q$ )
1:  $X_{q} = \emptyset$  ( $1 \leq q \leq Q$ ).
2: for all  $q \in [Q]$  do
3: while  $|X_{q}| \leq N_{s}^{q}$  do
4:  $X = \bigcup_{q=1}^{Q} X_{q}$ .
5:  $e^{*} = \arg\max_{e \in \Gamma_{q} \setminus X_{q}} f(X \cup \{e\}) - f(X)$ .
6:  $X_{q} = X_{q} \cup \{e^{*}\}$ .
</div>

Thus, we use the algorithm described in Algorithm 3 to select strategies for heterogeneous chargers. Algorithm 3 is essentially a greedy algorithm that goes through all types of chargers, and greedily selects the strategy that leads to maximum charging utility increment on global.

Theorem 4.2. Setting $\begin{array} { r } { \epsilon _ { 1 } = \frac { 2 \epsilon } { 1 - 2 \epsilon } , } \end{array}$ our algorithm to HIPO achieves an approximation ratio of $\frac { 1 } { 2 } - \epsilon$ and its time complexity is $O ( N _ { s } \dot { N } _ { o } { } ^ { 4 } \epsilon ^ { - 2 } N _ { h } { } ^ { 2 } c ^ { 2 } )$

Proof. First, it is proved that the greedy algorithm solves the monotone submodular function with partition matroid constraint with <sup>1</sup>-approximation ratio in [38]. Taking the utility approximation in Lemma 4.3 into consideration, the total approximation ratio is $\begin{array} { r } { \frac { 1 } { 2 ( 1 + \epsilon _ { 1 } ) } = \frac { 1 } { 2 } - \epsilon } \end{array}$ by setting $\begin{array} { r } { \epsilon _ { 1 } = \frac { 2 \epsilon } { 1 - 2 \epsilon } . } \end{array}$

Then, according to Algorithm 2, we should enumerate every pair of devices in each feasible geometric area, of which the number is $O ( N _ { o } ^ { 2 } )$ . Thus, there are $O ( N _ { o } ^ { 4 } \epsilon _ { 1 } ^ { - 2 } N _ { h } ^ { 2 } c ^ { 2 } )$ strategies in the candidate solution set, according to the number of feasible geometric areas in Lemma 4.4. Moreover, Algorithm 3 requires $O ( N _ { s } )$ iterations for all the elements in the candidate solution set, so the time complexity is $O ( N _ { s } N _ { o } { } ^ { 4 } \epsilon _ { 1 } ^ { - 2 } N _ { h } { } ^ { 2 } c ^ { 2 } )$ . Since $\epsilon _ { 1 }$ and  are equivalent infinitesimals, the final result of time complexity is $O ( N _ { s } N _ { o } { } ^ { 4 } \epsilon ^ { - 2 } N _ { h } { } ^ { 2 } c ^ { 2 } )$ tu

Note that we can improve the approximation ratio from $\frac { 1 } { 2 } - \epsilon$ to $1 - 1 / e - \epsilon$ by adopting the algorithm in [39], which is, however, too computationally demanding to use in practice.

## 5 DISTRIBUTED ALGORITHM FOR PDCSS EXTRACTION

In this section, we consider the distributed algorithm for PDCS extraction. The detailed algorithm for a single task is described in Algorithm 4. The basic idea is to divide extracting PDCSs corresponding to each set of neighboring devices into several independent tasks. We first calculate the set of devices for each device within distance $2 d _ { m a x } ^ { k }$ as the neighboring device set in terms of the charger type of $s _ { k } .$ . Then, for each set, Algorithm 2 can be conducted independently for different tasks. As shown in Fig. 9, for device $o _ { 1 }$ , we calculate its neighboring device set, say $\{ o _ { 2 } , o _ { 3 } , o _ { 4 } , o _ { 5 } , o _ { 6 } \}$ , and draw the line and arcs through each pair of devices including $o _ { 1 }$ . For example, we draw a line and two arcs with circumferential angle $\alpha _ { s } ^ { k }$ through $o _ { 1 }$ and $O _ { 2 } ,$ <sup>a</sup>and intersect with the feasible geometric area boundaries generated by the common neighbors and related obstacles. To avoid repeated calculating, these tasks only conduct Algorithm 2 on the devices with larger indices $j > i$ in the neighboring device set $\hat { O } ^ { k i }$ <sup>i</sup> as Algorithm 4 shows.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 4. PDCS Extraction for Single Neighboring Device Set

Input: The index of device to be computed i, parameters of devices, and the charger type of  $s_{k}$ 

Output: PDCSs and corresponding strategies

1: Compute the neighboring device set  $\hat{O}^{ki}$  of device  $o_{i}$  in terms of the charger type of  $s_{k}$ .

2: for each device in  $\hat{O}^{ki}$  with index j &gt; i, say  $o_{j}$  do

3: Draw a straight line crossing  $o_{i}$  and  $o_{j}$ , and intersect the boundaries of feasible geometric area generated by neighboring devices and related obstacles.

4: Put the charger at the intersection point, and let the clockwise boundary cross  $o_{i}$  and  $o_{j}$ .

5: Add the PDCS and the corresponding strategy under this setting into the candidate solution set.

6: Draw arcs crossing  $o_{i}$  and  $o_{j}$  with circumferential angle  $\alpha_{s}^{k}$ , and intersect the boundaries of the feasible geometric area generated by neighboring devices and related obstacles.

7: Put the charger at the intersection point, and let the two line segment boundaries cross  $o_{i}$  and  $o_{j}$ .

8: Add the PDCS and the corresponding strategy under this setting into the candidate solution set.

9: Perform PDCS extraction algorithm for point case in Algorithm 1 at the intersection points of the approximated power receiving area of  $o_{i}$  and  $o_{j}$ . If the point is not in the feasible geometric area, ignore it.

10: Perform PDCS extraction algorithm for point case in Algorithm 1 at the intersection points of the approximated power receiving area of  $o_{i}$  and the related obstacles and holes.

11: Filter the PDCSs and remove the subsets and their corresponding strategies.
</div>

![](images/2c476ca20f480ddabd6d4151ae11d15ddde3ea6cf57d767a84a944a2baaf6ff0.jpg)  
(a) Simulation scenaric

![](images/7700c5b81b379da159a6b01ea03e544158b8d408c320f51a5db28f3779102c22.jpg)  
(b) Sensors instance

![](images/7cecbea57f34a870e3127a4c39d6b44b6daaea818fbee9e804301cdd3e375636.jpg)  
(c) HIPO instance

![](images/c9f2b8372a45ab6b3e9757dcf7ef37f4bb471cce2a11366572e3d76f7cc76cb9.jpg)  
(d) RPAR instance

![](images/cdd12747033679f39e960a978f34a9883cf8b399947e001de6c0475d7658abef.jpg)  
(e) RPAD instance

![](images/db084ddcdc4200213bbc2517b74bc07b5384517416eed5f404768c3a41f2ce68.jpg)  
(f) GPAR Square instance

![](images/494b6243cb30ba00e83c7a87c603226268eb870d3b9801c44a723c422d6ec1ba.jpg)  
(g) GPAR Triangle instance

![](images/5677d50190255eb86e368d3f0ca77d958dc36e9f2a5fa177dbce317aff918262.jpg)  
(h) GPAD Square instance

![](images/5fcb62adfda03a7972e95d9a21c18c7ee32a7c258768bf0807f07b88d725af31.jpg)  
(i) GPAD Triangle instance

![](images/7ba91288ae2dcb538f7e53bfe7fcc705ae399830aafd48828c28ee979a43e1c8.jpg)  
(j) GPPDCS Square instance

![](images/5cfd23ce808a07da384a9b33be5afe618911db585a36117bb769f13e318fd1fe.jpg)  
(k) GPPDCS Triangle instance  
Fig. 10. Instances.

Moreover, we use Longest Processing Time (LPT) algorithm [40] to assign the tasks to different machines since the time span of tasks varies, which achieves 4=3-approximation ratio to minimize the longest time span of machines. The final distributed algorithm is described in Algorithm 5.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 5. Distributed HIPO
Input: The number of parallel machines n, all the parameters of chargers and devices, objective function  $f(X)$ 
Output: PDCSs and corresponding strategies
1: if  $n \geq N_{o}$  then
2: Assign the task (Algorithm 4) with device index i and all the charger types to parallel machine i.
3: else
4: Apply LPT algorithm [40] to assign tasks to different parallel machines.
5: Execute Algorithm 3 when all the parallel machines have done their tasks.
</div>

One may argue that why not establish the distributed algorithm on the original feasible geometric areas. This is because for programming, it is hard to obtain the feasible geometric areas and it consumes much more time complexity. Moreover, there might be a large number of feasible geometric areas, which makes it challenging and almost impossible to process all the information about the feasible geometric areas.

## 6 SIMULATION RESULTS

In this section, we conduct simulation experiments to evaluate our algorithm.

In the simulations, we randomly distributed some hetero geneous devices of four types on a 40 m  40 m square area where two obstacles exist as shown in Fig. 10a. Moreover, there are three types of chargers for placement whose detailed hardware parameters are shown in Tables 2, 3, and 4. The initial number of chargers are one, two, and three for charger type 1, 2, and 3, respectively, while that of devices are four, three, two, and one for device type 1, 2, 3, and 4. The default setting for charger number is three times of initial setting while that for device number is four times of initial setting. The default value of $P _ { t h } ^ { j }$ for all devices and  are set to 0.05 and September 11.2024 at 08:07:32 UTC from IEEE Xplore. Restrictions apply

TABLE 2  
Default Charger Parameters

<table><tr><td></td><td>Charger type 1</td><td>Charger type 2</td><td>Charger type 3</td></tr><tr><td> $\alpha_{s}^{i}$ </td><td> $\pi/6$ </td><td> $\pi/3$ </td><td> $\pi/2$ </td></tr><tr><td> $d_{min}^{i}$ </td><td>5</td><td>3</td><td>2</td></tr><tr><td> $d_{max}^{i}$ </td><td>10</td><td>8</td><td>6</td></tr></table>

TABLE 3  
Default Device Parameters

<table><tr><td></td><td>Device type 1</td><td>Device type 2</td></tr><tr><td rowspan="2"> $\alpha_{o}^{i}$ </td><td> $\pi/2$ </td><td> $2\pi/3$ </td></tr><tr><td>Device type 3</td><td>Device type 4</td></tr><tr><td> $\alpha_{o}^{i}$ </td><td> $3\pi/4$ </td><td> $\pi$ </td></tr></table>

0.15, respectively. Note that if the randomly generated position happens to be inside an obstacle and is thus infeasible, we repeat the process until a feasible position is obtained. Besides, each point in the evaluation figures indicates the average value of 100 experiments of random device topologies.

As there is no existing algorithm for our considered problem, we propose eight algorithms for comparison as follows. Randomized Position with Angular Randomization (RPAR) randomly generates charger positions and orientations. Randomized Position with Angular Discretization (RPAD) improves RPAR by enumerating the orientation of a charger on each position with an angle of value in $0 , \alpha _ { s } ^ { i } , \ldots ,$ , and $( \lceil 2 \pi / \alpha _ { s } ^ { i } \rceil ^ { - } 1 ) \alpha _ { s } ^ { i }$ <sup>a</sup>. Grid Point with Angular Randomization <sup>p a a</sup>(GPAR) and Grid Point with Angular Discretization (GPAD) improves RPAR and RPAD, respectively, by placing chargers on grid points, and Grid Point with Practical Dominating Coverage Set Extraction for point case (GPPDCS) further improves GPAD by replacing the above orientation selection method by our practical dominating coverage set extraction algorithm for point case. Further, each of the above three algorithms has two versions: triangle grid points (GPAR Triangle, GPAD Triangle, GPPDCS Triangle) and square grid points (GPAR Square, GPAD Square, GPPDCS Square), both with grid length ${ \sqrt { 2 } } / 2 \cdot d _ { m a x } ^ { i }$ for each charger with specified charging radius $d _ { m a x } ^ { i } .$

## 6.1 Performance Comparison

## 6.1.1 Instance Illustration

We show the solutions for all algorithms for an instance with sensors shown in Fig. 10b. We set the number of chargers four times of the initial setting, that is, there are 12, 8, and 4 charger for type 1, 2, and 3, respectively. The charging utility for our algorithm is 0.8495, while the others are 0.1000, 0.4046, 0.4605, 0.4867, 0.6006, 0.6191, 0.6348, and 0.6932 for RPAR, RPAD, GPAR Square, GPAR Triangle, GPAD Square, GPAD

Triangle, GPPDCS Square, and GPPDCS Triangle, respectively. We can see that the placement of our algorithm can charge all the devices while others cannot. In particular, for the RPAR solution as shown in Fig. 10d, only few devices are charged due to the randomness of positions and orientations, and many strategies share the same positions and orientations to charge the limited covered devices. In the RPAD solution shown in Fig. 10e, the result improves due to the enumeration and selection of orientations, but still almost half of the devices are not charged. The algorithms based on grid points perform better, as shown in Figs. 10f, 10g, 10h, 10i, 10j, and 10k, but most of the devices in the right down corner are not charged.

## 6.1.2 Impact of Number of Chargers $N _ { s }$

Our simulation results show that on average, HIPO outperforms GPPDCS Triangle, GPPDCS Square, GPAD Triangle, GPAD Square, GPAR Triangle, GPAR Square, RPAD, and RPAR by 33.49, 38.32, 43.43, 47.65, 116.60, 144.15, 166.85, and 970.37 percent, respectively, in terms of $N _ { s } .$ . Fig. 11a shows that the charging utility increases monotonically with $N _ { s } .$ . The charging utility of our algorithm first increases at a high rate and becomes almost 1 when $N _ { s }$ is five times of initial setting; then, the increasing rate tends to be gentle, while that of compari son algorithms still remain low. In contrast, the charging utilities of the comparison algorithms are limited because the positions or the orientations of chargers are predetermined or randomly generated for these algorithms. Note that RPAD increases at a relatively higher rate and even performs better than the two GPAR algorithms when the number of chargers is larger than five times of the default setting. It is because there are more better choices of charger orientations in RPAD when number of position choices increases, while GPAR may generate trivial orientations with low charging utility.

## 6.1.3 Impact of Number of Devices $N _ { o }$

Our simulation results show that on average, HIPO outperforms GPPDCS Triangle, GPPDCS Square, GPAD Triangle, $\dot { G } P A D$ Square, GPAR Triangle, GPAR Square, RPAD, and RPAR by 37.13, 42.84, 49.87, 55.50, 124.50, 141.66, 197.85, and 1106.68 percent, respectively, in terms of $N _ { o } .$ . It can be seen from Fig. 11b that the charging utility monotonically decreases with the number of devices. Our algorithm performs well when the numbers of devices are one and two times of the initial setting, but degrades relatively fast when the number of devices becomes larger. The charging utilities of the four grid points based algorithms with orientations selected decrease at nearly the same rate with the GPPDCS algorithm gaining a bit higher utility, while the two GPAR algorithms and RPAD algorithm gain low charging utility with relatively slower decreasing rate. As a charger is expected to cover more devices when devices become dense, the charging utility decreases more slowly when the number of devices becomes larger.

TABLE 4  
Correlated Parameters

<table><tr><td></td><td>Device type 1</td><td>Device type 2</td><td>Device type 3</td><td>Device type 4</td></tr><tr><td>Charger type 1</td><td>a = 100, b = 40</td><td>a = 130, b = 52</td><td>a = 160, b = 64</td><td>a = 190, b = 76</td></tr><tr><td>Charger type 2</td><td>a = 110, b = 44</td><td>a = 140, b = 56</td><td>a = 170, b = 68</td><td>a = 200, b = 80</td></tr><tr><td>Charger type 3</td><td>a = 120, b = 48</td><td>a = 150, b = 60</td><td>a = 180, b = 72</td><td>a = 210, b = 84</td></tr></table>

Authorized licensed use limited to: SHANGHAI UNIVERSITY. Downloaded on September 11,2024 at 08:07:32 UTC from IEEE Xplore. Restrictions apply.

GPPDCS Triangle GPAR Triangle

![](images/a05a8134a1c2faa25148c02717af004b712113ebae516951423edaa109001f1e.jpg)  
(a) $N _ { s }$ vs. charging utility

![](images/8c74ec61705d2e8c595acb212cd0805328404e86e86e864114281f111aff7d12.jpg)  
(b) $N _ { o }$ vs. charging utility

![](images/5dda41b5512f886b2f917ce058ab8aaf9cbc388380daedf4593512db2a6eda27.jpg)  
(c) $\alpha _ { s } ^ { i }$ vs. charging utility

![](images/136dbf0524794bb52ce7efdb1016f2ad0c0ecc0fa1ea855396794cd3726a2e38.jpg)  
(d) $\alpha _ { o } ^ { i }$ vs. charging utility

![](images/5e96e1d56d5301f5b9947e141fa9474a22134f12601ee24be173a20bef51e3c4.jpg)  
Fig. 11. Simulations.  
(e) $P _ { t h } ^ { j }$ vs. charging utility

![](images/0474dfb7087e14b21b6b67c6deea41a92ff3bf634f1a3f2a2dea2d91238146ce.jpg)  
(f) $d _ { m i n } ^ { i }$ vs. charging utility

## 6.1.4 Impact of Charging Angle $\alpha _ { s } ^ { i }$

Our simulation results show that on average HIPO outperforms GPPDCS Triangle, GPPDCS Square, GPAD Triangle, GPAD Square, GPAR Triangle, GPAR Square, RPAD, and RPAR by 38.54, 42.64, 51.86, 55.94, 109.53, 124.91, 198.37, and 997.82 percent, respectively, in terms of $\alpha _ { s } ^ { i }$ . Fig. 11c shows that <sup>a</sup>the charging utility increases slowly with charging angle, while RPAD remains relatively stable. For our algorithm and grid points based algorithms, chargers with larger charging angles generally cover more devices, while RPAD may select relatively bad positions around which there are only a few devices. Moreover, the influence of charger orientations decreases with larger charging angle, so the performance of algorithms with angular randomization is approaching that of according algorithms with angular discretization. Our algorithm always gains much higher charging utility than the other algorithms.

## 6.1.5 Impact of Receiving Angle $\alpha _ { o } ^ { i }$

Our simulation results show that on average, HIPO outperforms GPPDCS Triangle, GPPDCS Square, GPAD Triangle, GPAD Square, GPAR Triangle, GPAR Square, RPAD, and RPAR by 33.03, 36.59, 45.72, 49.85, 110.05, 25.88, 189.07, and 1016.13 percent, respectively, in terms of $\alpha _ { o } ^ { i } .$ . Fig. 11d shows the <sup>a</sup>trend of charging utility with receiving angles of devices. The charging utilities of all the algorithms increase when receiving angles of devices becomes larger.

## 6.1.6 Impact of Power Threshold $P _ { t h } ^ { j }$

Our simulation results show that on average, HIPO outperforms GPPDCS Triangle, GPPDCS Square, GPAD Triangle, GPAD Square, GPAR Triangle, GPAR Square, RPAD, and RPAR by 36.21, 39.73, 50.24, 55.33, 111.64, 131.15, 192.40, and 1089.49, respectively, in terms of $P _ { t h } ^ { j }$ . Fig. 11e demonstrates that the charging utility first remains stable, then gradually decreases when $P _ { t h } ^ { j }$ becomes larger. The reason is that with a higher value of $P _ { t h } ^ { j } ,$ all the algorithms need to select more chargers to charge a device. Our algorithm performs much better than the other algorithms.

## 6.1.7 Impact of Nearest Distance $d _ { m i n } ^ { i }$

Our simulation results show that on average, HIPO outperforms GPPDCS Triangle, GPPDCS Square, GPAD Triangle, GPAD Square, GPAR Triangle, GPAR Square, RPAD, and RPAR by 40.38, 43.93, 53.65, 58.12, 117.69, 136.21, 188.26, and 1024.88 percent, respectively, in terms of $d _ { m i n } ^ { i }$ . The values of x-axis in Fig. 11f means the times of the original setting of $d _ { m i n } ^ { i }$ . It shows that the charging utility gradually decreases when $d _ { m i n } ^ { i }$ becomes larger, since the charging area becomes smaller. Moreover, the charging utility decreases faster when $d _ { m i n } ^ { i }$ is larger, since the charging area decreases more when $d _ { m i n } ^ { i }$ becomes larger. Still, our HIPO algorithm outperforms the other algorithms.

## 6.1.8 Impact of Number of Devices $N _ { o }$ and Number of Parallel Machines on Time Consumption

Our simulation results show that on average, 5-distributed, 10- distributed, 15-distributed, 20-distributed, and 25-distributed reduce the time consumption by 80.10, 88.79, 91.05, 92.32, and 92.39 percent, respectively, in terms of $N _ { o }$ . We conduct the simulation on time consumption comparison between non-distributed and distributed HIPO algorithm. Fig. 12 shows the results, in which we plot data with logarithmic scale for the y-axis to show the differences between these settings more clearly. All the values of time consumption are divided by the value of non-distributed at one time of number of devices to eliminate the influence of different platforms. We just show the time consumption of the parallel-processing part. We can see that the distributed algorithm consumes much less time than the non-distributed algorithm, and the larger the number of devices, the more reduced time consumption. Note that September 11,2024 at 08:07:32 UTC from IEEE Xplore. Restrictions apply.

![](images/b90bc7fb24055430c9ae5c751f8ef3c0de2350ff51db09423edfd54425f83eaa.jpg)  
Fig. 12. Comparison of time complexity.

![](images/b72b1ecd262a00fd1f3b6b52b0759bf43c7fed8c3b0f28735859b0eadde40845.jpg)  
Fig. 13. Comparison of different $P _ { t h } ^ { j } \mathfrak { s }$ .

according to Algorithm 5, the time consumption will not continue to reduce when the number of machines becomes no smaller than that of devices. We can see that in Fig. 12, when there are more machines, the time consumption reduces more slowly or even does not reduce since the number of machines is approaching the number of devices, thus, the time consumption is approaching the time span of the longest task.

## 6.1.9 Impact of Different Power Thresholds $P _ { \mathit { t h } } ^ { j } s$

Our simulation results show that in HIPO, the changing trends of charging utility are almost the same with different power thresholds for different types of devices, and the difference is 3.20 percent on average, in terms of $N _ { o } .$ . In Fig. 13, the legend means the power threshold difference of each two adjacent device types, and we always keep the power threshold of device type 2 as 0.05. For example, the legend 0:01 means that the power thresholds for device types 1-4 are 0.06, 0.05, 0.04, and 0.03, respectively. We also change the default number of devices as the same number 2 for all four types of devices to better show the impact of different power thresholds. The x-axis in Fig. 13 shows the multiple of the default number of devices. As shown in Fig. 13, the charging utility of all different settings of power thresholds go with the same pattern as $N _ { o }$ increases, just like that in Fig. 11b. Moreover, according to the parameters shown in Tables 2 and $^ { 4 , }$ the received charging power of devices of type 1-4 monotonically decreases with the same charger at the same distance. Thus, if the power threshold for a larger device type number is larger, more chargers are needed to get the power threshold for these types of devices, so the charging utility decreases, which explains the difference between the different settings in Fig. 13.

## 6.2 Insights

In this subsection, we study the impact of $d _ { m a x } ^ { i }$ and $d _ { m i n } ^ { i }$ and the charging utility distribution of all the devices to reveal the advantages of our algorithm. First, we study the impact of $d _ { m a x } ^ { i }$ and $d _ { m i n } ^ { i } .$ . We set the number of chargers to be two times of the initial setting, and the multiple of $d _ { m a x } ^ { i }$ varies from 0.6 to 2 while $d _ { m i n } ^ { i } / { \bar { d } } _ { m a x } ^ { i }$ varies from 0 to 0.9. Note that each obtained date point denotes the average value of 100 experimental results. Fig. 14 shows that if $\bar { d } _ { m i n } ^ { i }$ tends to be zero, the charging utility increases much faster, while $d _ { m i n } ^ { i } / d _ { m a x } ^ { i }$ remains high. That is, if the charging area is relatively small, the charging utility increases very slow with $d _ { m a x } ^ { i } .$ The other comparison algorithms suffer from $d _ { m i n } ^ { i }$ because the predetermined positions may cause some devices within the distance of $\bar { d _ { m i n } ^ { i } }$ not charged.

![](images/00530a5126d6d81ffd727dafec4cf3d3abb881c93ea26e9c13b9d22ef64a8e46.jpg)  
Fig. 14. Impact of $d _ { m i n } ^ { i }$ and $d _ { m a x } ^ { i } .$

![](images/f182a10585f879645371b988dd900845a575b7984ea602be9e49104f17489b21.jpg)  
Fig. 15. Charging utility CDF of different devices.

Fig. 15 shows the cumulative distribution function (CDF) of charging utilities of all the 40 devices in one topology. We can see that no device obtains charging utility under 0.5 in our algorithm while a large amount of devices in other comparison algorithms do not harvest any charging utility. Therefore, the charging utility gained by devices in our algorithm is relatively balanced at a high rate, which contributes to the good performance of our algorithm.

## 7 FIELD EXPERIMENTS

In this section, we conduct field experiments to evaluate our proposed algorithm.

Our testbed consists of six chargers with three TB-Powersource power adjustable wireless power transmitters [41] as shown in Fig. 16 with one tuned to 1W working power and two of 2W, and three TX91501 wireless power transmitters [4] with working power of 3W as shown in Fig. 17. Thus, there are three types of chargers. Moreover, there are two types of rechargeable sensor nodes equipped with P2110 power receivers both produced by Powercast [42] as shown in Figs. 18 and 19, respectively. Each type has five nodes. An AP is connected to the laptop to report the collected data from sensor nodes as shown in Fig. 20. The rechargeable sensor nodes are placed with strategies ð20; 15Þ; 200<sup></sup> h i, $\langle ( 4 7 , 2 0 ) , 3 5 0 ^ { \circ } \rangle , \stackrel { \cdot } { \langle } ( 1 1 3 , 6 5 ) , 2 0 ^ { \circ } \rangle , \ : \langle ( 2 0 , 8 5 \bar { ) } , 1 4 0 ^ { \circ } \rangle , \ : \langle ( 1 3 , 9 5 ) , 4 0 ^ { \circ } \rangle .$ ð7; 115Þ; 190<sup></sup> h i, ð27; 110Þ; 310<sup></sup> h i, ð47; 100Þ; 150<sup></sup> h i, hð50; 118Þ; 160<sup></sup>i, and ð60; 93Þ; 270<sup></sup> h i in a square area of 120 cm  120 cm. September 11,2024 at 08:07:32 UTC from IEEE Xplore. Restrictions apply.

![](images/7b28c164f8f242764682a0ae16f4884e35bc8046fc90a1c2f73eb1ddf72b5876.jpg)  
Fig. 16. Charger type 1.

![](images/8ffdc4300b3187e29d0952385a30eda159b8d22a6d2087aec55f7fe9aee94855.jpg)  
Fig. 17. Charger type 2.

![](images/9d87157a2076f489aaaef6a14c2bf4faab27eb1b9ecb347de2bbeb4bb299188c.jpg)  
Fig. 18. Sensor type 1.

![](images/b224a0a886edbe181aa32789858d938f72034c156a9ff29d7e8b275973b1c2b6.jpg)  
Fig. 19. Sensor type 2.

This square area is bounded by the dotted square including three obstacles as shown in Fig. 24. The layout scenes of HIPO, GPPDCS triangle, and GPAD triangle are shown in Figs. 21, 22, and 23, respectively.

The placement of all the three algorithms can be seen directly from Fig. 24. We can see that in our algorithm, the chargers are deployed around the sensors closely and uniformly, while the chargers are placed a little far from the placing field of sensors in other two algorithms. This leads to the results in Fig. 25 that all the devices can receive charging utility from the chargers in our algorithm, while that of the other algorithms cannot. Although comparison algorithms have more charging utility for device #2, #3, and #4, but this is not the case for other devices. The CDF of charging power is depicted in Fig. 26, and it shows that the line of HIPO approaches 1 at the lowest speed, which indicates that HIPO generally leads to more charging power for the devices.

## 8 DISCUSSION

## 8.1 Charger Redeployment

In this subsection, we discuss the charger redeployment problem when the topology of devices dynamically change, that is, how to schedule the chargers so that the incurred overhead of switching from their previous deployment scheme to a new redeployment scheme for the chargers, such as moving and rotating cost, is minimized.

![](images/8f9bfea18ee0b28f81acc0ba657a3a827720f565dec1e3fb3d24a38acf0e1401.jpg)

Fig. 20. AP.  
![](images/e7a4acd170036a444ec0cef807bad0e8afb70ad5b7d75f743d4df447fcc68c79.jpg)  
Fig. 21. HIPO testbed.

![](images/62f930b5677a8ea03b69bd57be76fbb718a003ef258a6fc1f7d816ac7a7cebc7.jpg)  
Fig. 22. GPPDCS Triangle testbed.

![](images/70126d4060436b1eff61af20dbf31c64e02ae82c74877178d1db55eb3cf4af89.jpg)  
Fig. 23. GPAD Triangle testbed.

Suppose that we know the original and new device topologies. Naturally, we can perform our HIPO algorithm two times for the two different topologies and obtain their corresponding solutions. In the following, we consider two different optimization problems, i.e., minimizing the overall switching overhead and minimizing the maximum switch ing overhead for all chargers.

## 8.1.1 Minimizing Overall Switching Overhead

Generally, the problem can be formulated into multiple weighted bipartite graph perfect matching subproblems, that is, given complete bipartite graphs $G _ { q } ( U _ { q } , \bar { V _ { q } } , E _ { q } ) , q =$ $1 , \ldots , \mathcal { Q }$ and a weight $w _ { i j } ^ { q }$ for the edge connecting the ith vertex in $U _ { q }$ and the jth vertex in $V _ { q } ,$ where Q is the number of charger types, $U _ { q }$ and $V _ { q }$ are the original and the new strategies sets, respectively, we need to find matchings for all the graphs which match all the vertices such that the total value of weights of the selected edges is minimized. These September 11,2024 at 08:07:32 UTC from IEEE Xplore. Restrictions apply.

![](images/4c85a6d1e763d19c487dadcb60d56cf37905981777c23aa970df860ddb86ab57.jpg)

![](images/679fde2d96c47199e10e77c5369eb24121942915ae65ff446c5b7836ff6a137f.jpg)  
Fig. 24. Positions & orientations of chargers & sensors.

![](images/1a1da266e09e9158316cd6074bb14c6f5c4b44ea284133f40d031fe599945507.jpg)  
Fig. 25. Charging power of each device.

subproblems can be then easily addressed by using the well-known Hungarian algorithm [43], [44].

We take a toy example shown in Fig. 27 for illustration. Suppose there are two types of chargers to be deployed: $s _ { 1 } ,$ $s _ { 2 } ,$ and $s _ { 3 }$ are of type 1 while $s _ { 4 }$ and $s _ { 5 }$ are of type 2. Suppose the strategies obtained by HIPO are as shown in Fig. 27a, and the new obtained strategies for a new device topology are shown in Fig. 27b. And we need to determine how to transfer the original strategies to the new ones with the minimum overhead. For example, $s _ { 1 }$ has three choices of transferring to $s _ { 1 } ^ { \prime } , s _ { 2 } ^ { \prime } , \mathrm { o r } s _ { 3 } ^ { \prime } ,$ while $s _ { 4 }$ has two choices of transferring to $s _ { 4 } ^ { \prime } \mathrm { o r } s _ { 5 } ^ { \prime }$ Each transformation leads to different switching overhead. The set of the original strategies and that of the new strategies of each charger type constitute a weighted bipartite graph, in which the weight of an edge denotes the switching overhead of the associated transformation, as shown in Fig. 28. Finally, we can apply the Hungarian algorithm to optimally solve the obtained two weighted bipartite matching subproblems.

## 8.1.2 Minimizing Maximum Switching Overhead

Clearly, for this problem, we also need to find perfect matchings in multiple weighted bipartite graphs, though the final objective function is changed. Besides, we stress that we take our study one step further by continuing minimizing the overall switching overhead after the maximum switching overhead is minimized.

In particular, our proposed algorithm consists of two steps: the minimum maximum weight searching step and the perfect matching generating step. In the first step, we sort all the weights and apply binary search to determine the minimum maximum weight. In each iteration of the searching process, we first remove all the edges with weights larger than the current selected weight whose initial value is set to the maximum value of weights in the graph. Then, we use the Hall’s Theorem [45] to check whether the remained bipartite graph can induce a perfect matching, and accordingly adjust the searching range. We record the final selected weight as the minimum maximum weight when the searching process terminates. In the second step, we remove all the edges with weights larger than the minimum maximum weight, and apply the Hungarian algorithm [43], [44] to further optimize the overall switching overhead. Obviously, the obtained solution has the minimum overall switching overhead given that the maximum switching overhead of chargers is minimized.

![](images/a48d292455a6864b325ff591b55561d9cac8c1af6b473856140497251d2ad98b.jpg)  
Fig. 26. Charging power CDF of different devices.

![](images/36f023125217d1d622902f0c14b325f80c25496f206d736a43dc8aaed553cead.jpg)

![](images/41588123e1798df72dea58f87fd6ff9d819c9e74717144549af8ae3683b5e57a.jpg)  
(b) Redeployment  
Fig. 27. A toy example of redeployment.

## 8.2 Deployment Costs

In this subsection, we discuss the charger deployment costs. We first introduce means to deploy chargers, then measure these costs, and finally jointly consider charging utility and deployment costs by formulating the whole problem.

In general, there are two methods to deploy chargers: by manual work or by machines. In the case of small range walking available area, the chargers can be deployed by manual work easily. In more complicated cases, the chargers can be transported by machines such as mobile cars, aeroplanes, and robots. The transportation cost can be formulated as the sum of functions of travel distance and rotating angles of all the chargers, since traveling and rotating are the only two ways for energy consumption no matter how to deploy these charg ers. The other part of deployment cost is how the chargers are charged. For the traditional chargers such as TX91501 wireless power transmitters [4] and TB-Powersource power adjustable wireless power transmitters [41], they are charged by cables, while for future chargers, they may be charged by solar or wind energy. Moreover, as the working charging power of the chargers is fixed, this part of deployment cost can be described as a function of the working charging power. Thus, the overall deployment costs can be formulated as

$$
c (S) = \sum_ {s _ {i} \in S} f _ {d} (d _ {i}) + f _ {\theta} (\theta_ {i}) + f _ {P} (P _ {i}),
$$

![](images/61ec02ac631c9543d95e2515bf7c603b99e495cfc788f6cd73fb6f5ed1d523d2.jpg)  
Fig. 28. Bipartite graphs of Fig. 27.

where $d _ { i } , \theta _ { i } ,$ , and $P _ { i }$ denote the traveling distance (from <sup>u</sup>the former position), the rotating angle, and the working charging power of charger $s _ { i }$ when deploying it, respectively; $f _ { d } ( \cdot ) , f _ { \theta } ( \cdot )$ and $f _ { P } ( \cdot )$ are three monotone increasing functions <sup>u</sup>of the traveling distance, the rotating angle, and the working charging power, respectively; and $c ( \cdot )$ is the cost function.

To combine the original HIPO problem in Section 3.3, one way is to limit the deployment costs not to exceed a certain level B. Note that it is more beneficial to obtain more charging utility in the long term since the charging scenario is static. Thus, the whole optimization problem can be formalized as follows:

$$
\begin{array}{l} \max _ {s _ {i}, \phi_ {s} ^ {i}} \frac {1}{N _ {o}} \sum_ {j = 1} ^ {N _ {o}} \mathcal {U} _ {j} \left(\sum_ {i = 1} ^ {N _ {s}} P _ {w} (s _ {i}, \phi_ {s} ^ {i}, o _ {j}, \phi_ {o} ^ {j})\right), \\ \text {s.t.} c (S) \leq B, s _ {i} \in S, s _ {i} \in \gamma , \text {and} \phi_ {s} ^ {i} \in [ 0, 2 \pi). \end{array}
$$

Note that the sum of the functions of traveling distance and rotating angle can be formalized as a TSP problem (chargers in one base station initially) or an m-TSP problem (chargers in m base stations initially), while the sum of the function of consuming power can be seen as the nodes placing cost. After performing our PDCS extraction algorithm (Algorithm 2), we obtain the whole candidate strategy set. Then, we can solve this problem by referring to the algorithm in [46] to get the $\ : \ : ( 1 - e ^ { - 1 } ) \ :$ approximation ratio.

## 8.3 Charging Utility Balancing

Rather than maximizing the overall charging utility, it is also important to consider the problem of charging utility balancing of all the devices, that is, to guarantee the fairness of charging utility. The traditional and the most commonly discussed fairness criterion is max-min fairness [47]. The maxmin fairness in our problem, i.e., maximizing the minimum charging utility of the devices, is formulated as follows:

$$
\begin{array}{l} \max _ {s _ {i}, \phi_ {s} ^ {i}} \min _ {j} \mathcal {U} _ {j} \Bigg (\sum_ {i = 1} ^ {N _ {s}} P _ {w} (s _ {i}, \phi_ {s} ^ {i}, o _ {j}, \phi_ {o} ^ {j}) \Bigg), \\ \text {s.t.} s _ {i} \in \gamma \text {and} \phi_ {s} ^ {i} \in [ 0, 2 \pi). \end{array}\tag{15}
$$

Unfortunately, to the best of our knowledge, there is no efficient approximation algorithm for the max-min fairness problem of the original submodular optimization as problem P3 , but it can be solved by heuristic algorithms such as Particle Swarm Optimization [48], Ant Colony Optimization [49], and Simulated Annealing Algorithm [50].

Moreover, proportional fairness [47] is another fairness criterion. It optimizes the sum of individual utility which is an increasing, strictly concave, and continuously differentiable function. In fact, the charging utility model in Equation (3) has made HIPO formalized in Equation (4) become an approximated proportional fairness problem of charging power since it is a concave function. To further achieve proportional fairness of charging utility, we can maximize the sum of logarithmic of the individual charging utility [47]. We formulate the proportional fairness HIPO problem as follows:

$$
\begin{array}{l} \max _ {s _ {i}, \phi_ {s} ^ {i}} \sum_ {j = 1} ^ {N _ {o}} \log \left(\mathcal {U} _ {j} (\sum_ {i = 1} ^ {N _ {s}} P _ {w} (s _ {i}, \phi_ {s} ^ {i}, o _ {j}, \phi_ {o} ^ {j})) + 1\right), \\ \text {s.t.} s _ {i} \in \gamma \text {and} \phi_ {s} ^ {i} \in [ 0, 2 \pi). \end{array}\tag{16}
$$

After PDCS extration, the objective function can still be reformulated as a monotone submodular function, so we can obtain the final strategies by Algorithm 3 with $\frac { 1 } { 2 } - \epsilon$ approximation ratio.

## 9 CONCLUSION

In this paper, we deal with the problem of practical heterogeneous wireless charger placement with obstacles. Our key contributions are building the practical charging model, proposing an approximation algorithm, and conducting both simulation and field experiments. The key technical depth of this paper is to reduce the infinite solution space to a limited one by using multi-feasible geometric area discretization and PDCS extraction algorithm, plus proving the problem as maximizing a submodular function subject to a partition matroid constraint. The experimental results show that our algorithm outperforms comparison algorithms by at least 33.49 percent.

## ACKNOWLEDGMENTS

This work was supported in part by the National Key R&D Program of China under Grant No. 2018YFB1004704, in part by the National Natural Science Foundation of China under Grant No. 61872178, 61502229, 61832005, 61672276, 61872173, 61802 $^ { 7 2 , }$ and 61321491, in part by the Natural Science Foundation of Jiangsu Province under Grant No. BK20181251, in part by the Fundamental Research Funds for the Central Universities under Grant 021014380079, in part by the Key Research and Development Project of Jiangsu Province under Grant No. BE2015154 and BE2016120, and the Collaborative Innovation Center of Novel Software Technology and Industrialization, Nanjing University, in part by the Jiangsu Highlevel Innovation and Entrepreneurship (Shuangchuang) Program, and in part by the Postgraduate Research & Practice Innovation Program of Jiangsu Province No. KYCX18\_0044.

## REFERENCES

[1] 2017. [Online]. Available: https://www.wirelesspowerconso rtium.com.

[2] H. Dai, et al., “Omnidirectional chargability with directional antennas,” in Proc. IEEE 24th Int. Conf. Netw. Protocols, Nov. 2016, pp. 1–10.

[3] H. Dai, et al., “Optimizing wireless charger placement for directional charging,” in Proc. IEEE Conf. Comput. Commun., May 2017, pp. 1–9.

[4] 2018. [Online]. Available: http://www.powercastco.com

[5] Y. T. Hou, et al., “Prolonging sensor network lifetime with energy provisioning and relay node placement,” in Proc. 2nd Annu. IEEE Commun. Society Conf. Sensor Ad Hoc Commun. Netw., Sep. 2005, pp. 295–304.

[6] I. Katsidimas, et al., “Efficient algorithms for power maximization in the vector model for wireless energy transfer,” in Proc. ACM Int. Conf. Distrib. Comput. Netw., 2017, pp. 30:1–30:10.

[7] Y. Shi, et al., “On renewable sensor networks with wireless energy transfer,” in Proc. IEEE INFOCOM, Apr. 2011, pp. 1350–1358.

[8] S. Zhang, et al., “P<sup>3</sup>: Joint optimization of charger placement and power allocation for wireless power transfer,” in Proc. IEEE INFO COM, Apr. 2015, pp. 2344–2352.

[9] L. Xie, et al., “Making sensor networks immortal: An energyrenewal approach with wireless power transfer,” IEEE/ACM Trans. Netw., vol. 20, no. 6, pp. 1748–1761, Dec. 2012.

[10] S. He, et al., “Energy provisioning in wireless rechargeable sensor networks,” IEEE Trans. Mobile Comput., vol. 12, no. 10, pp. 1931–1942, Oct. 2013.

[11] L. Xie, et al., “Wireless power transfer and applications to sensor networks,” IEEE Wireless Commun., vol. 20, no. 4, pp. 140–145, Aug. 2013.

[12] C. Wang, et al., “NETWRAP: An NDN based real-timewireless recharging framework for wireless sensor networks,” IEEE Trans. Mobile Comput., vol. 13, no. 6, pp. 1283–1297, Jun. 2014.

[13] X. Ye and W. Liang, “Charging utility maximization in wireless rechargeable sensor networks,” Wireless Netw., vol. 23, no. 7, pp. 2069–2081, Oct. 2017.

[14] P. Zhou, et al., “Leveraging target k-coverage in wireless rechargeable sensor networks,” in Proc. IEEE 37th Int. Conf. Distrib. Comput. Syst., Jun. 2017, pp. 1291–1300.

[15] W. Xu, et al., “Maximizing sensor lifetime with the minimal service cost of a mobile charger in wireless sensor networks,” IEEE Trans. Mobile Comput., vol. 17, no. 11, pp. 1–1, Nov. 2018.

[16] S. Nikoletseas, et al., “Low radiation efficient wireless energy Proc. IEEE 35th Int. Conf. Distrib. Comput. Syst., Jun. 2015, pp. 196–204.

[17] H. Dai, et al., “Radiation constrained wireless charger placement,” in Proc. 35th Annu. IEEE Int. Conf. Comput. Commun., Apr. 2016, pp. 1–9.

[18] H. Dai, et al., “SCAPE: Safe charging with adjustable power,” in Proc. IEEE Conf. Comput. Commun. Workshops, Jun. 2014, pp. 439–448.

[19] H. Dai, et al., “SCAPE: Safe charging with adjustable power,” IEEE/ACM Trans. Netw., vol. 26, no. 1, pp. 520–533, Feb. 2018.

[20] L. Li, et al., “Radiation constrained fair wireless charging,” in Proc. 14th Annu. IEEE Int. Conf. Sensing, Commun. Netw., Jun. 2017, pp. 1–9.

[21] L. Li, et al., “Radiation constrained fair charging for wireless power transfer,” ACM Trans. Sen. Netw., vol. 15, no. 2, pp. 15:1–15:33, 2019.

[22] H. Dai, et al., “Safe charging for wireless power transfer,” IEEE ACM Trans. Netw., vol. 25, no. 6, pp. 3531–3544, Dec. 2017.

[23] H. Dai, et al., “Radiation constrained scheduling of wireless charging tasks,” in Proc. 18th ACM Int. Symp. Mobile Ad Hoc Netw. Comput., 2017, pp. 17:1–17:10.

[24] H. Dai, et al., “Wireless charger placement for directional charging,” IEEE/ACM Trans. Netw., vol. 26, no. 4, pp. 1865–1878, Aug. 2018.

[25] H. Dai, et al., “Charging task scheduling for directional wireless charger networks,” in Proc. 47th Int. Conf. Parallel Process., 2018, pp. 10:1–10:10.

[26] X. Wang, et al., “Heterogeneous wireless charger placement with obstacles,” in Proc. 47th Int. Conf. Parallel Process., 2018, pp. 16:1–16:10.

[27] P. K. Agarwal, et al., Efficient Sensor Placement for Surveillance Problems. Berlin, Germany: Springer, 2009, pp. 301–314.

[28] C.-H. Wu, et al., “A delaunay triangulation based method for wireless sensor network deployment,” Comput. Commun., vol. 30, no. 14, pp. 2744–2752, 2007.

[29] H. Tan, et al., Arbitrary Obstacles Constrained Full Coverage in Wireless Sensor Networks. Berlin, Germany: Springer, 2010, pp. 1–10.

[30] C. Y. Chang, et al., “Obstacle-resistant deployment algorithms for wireless sensor networks,” IEEE Trans. Veh. Technol., vol. 58, no. 6, pp. 2925–2941, Jul. 2009.

[31] A. Saeed, et al., “Argus: Realistic target coverage by drones,” in Proc. ACM/IEEE Int. Conf. Inf. Process. Sensor Netw., Apr. 2017, pp. 155–166.

[32] Z. Zhang, et al., “Energy-efficient multihop polling in clusters of two-layered heterogeneous sensor networks,” IEEE Trans. Comput., vol. 57, no. 2, pp. 231–245, Feb. 2008.

[33] W. Liang, et al., “Approximation algorithms for capacitated minimum forest problems in wireless sensor networks with a mobile sink,” IEEE Trans. Comput., vol. 62, no. 10, pp. 1932–1944, Oct. 2013.

[34] X. Wang, et al., “Coverage and energy consumption control in mobile heterogeneous wireless sensor networks," IEEE Trans Autom. Control, vol. 58, no. 4, pp. 975–988, Apr. 2013.

[35] H. P. Gupta, et al., “Analysis of stochastic coverage and connectiv ity in three-dimensional heterogeneous directional wireless sensor networks,” Pervasive Mobile Comput., vol. 29, pp. 38–56, 2016.

[36] J. Guo, et al., “Sensor deployment with limited communication range in homogeneous and heterogeneous wireless sensor networks,” IEEE Trans. Wireless Commun., vol. 15, no. 10, pp. 6771– 6784, Oct. 2016.

[37] B. Xiao, et al., “Approximation algorithms design for disk partia covering problem,” in Proc. 7th Int. Symp. Parallel Architectures Algorithms Netw., May 2004, pp. 104–109.

[38] S. Fujishige, Submodular Functions and Optimization. vol. 58, Amsterdam, The Netherlands: Elsevier, 2005.

[39] G. Calinescu, et al., Maximizing a Submodular Set Function Subject to a Matroid Constraint (Extended Abstract). Berlin, Germany: Springer, 2007, pp. 182–196.

[40] R. Graham, “Bounds on multiprocessing timing anomalies,” SIAM J. Appl. Math., vol. 17, no. 2, pp. 416–429, 1969.

[41] 2018. [Online]. Available: http://www.terabits.cn/product

[42] [Online]. Available: https://www.powercastco.com/products/ development-kits/, access date as 2019.

[43] H. W. Kuhn, “The hungarian method for the assignment prob lem,” Naval Res. Logistics Quarterly, vol. 2, no. 1–2, pp. 83–97, 1955.

[44] J. Munkres, “Algorithms for the assignment and transportation problems,” J. Soc. Ind. Appl. Math., vol. 5, no. 1, pp. 32–38, 1957.

[45] P. Hall, “On representatives of subsets,” J. London Math. Soc., vol. s1–10, no. 1, pp. 58–62, 2009.

[46] H. Zhang and Y. Vorobeychik, “Submodular optimization with routing constraints,” in Proc. 30th AAAI Conf. Artif. Intell., 2016, pp. 819–825.

[47] F. P. Kelly, et al., “Rate control for communication networks: shadow prices, proportional fairness and stability,” J. Operationa Res. Society, vol. 49, no. 3, pp. 237–252, Mar. 1998.

[48] J. Kennedy, “Particle Swarm Optimization,” Encyclopedia Mach. Learn., C. Sammut and G. I. Webb, eds. Boston, MA: Springer US. 2010, pp. 760–766. https://doi.org/10.1007/978-0-387-30164-8\_630.

[49] M. Dorigo and M. Birattari, Ant Colony Optimization. Berlin, Germany: Springer, 2010

[50] S. Kirkpatrick, et al., “Optimization by simulated annealing,” Sci., vol. 220, no. 4598, pp. 671–680, 1983.

![](images/1f64bd9419730fe24916ab7022b60184b8aba4c49af0e4120cb7f17cae8509fc.jpg)  
Xiaoyu Wang received the BS degree from the Department of Computer Science and Technology, Soochow University, Suzhou, Jiangsu, China, in 2016. He is working towards the PhD degree in the Department of Computer Science and Technology, Nanjing University. His research interests include the wireless charging and data mining. He is a student member of the IEEE.

![](images/dba67028bae9026123cb412d1dd578d16de3cd789c52f6f0ba0f7ebe8516023c.jpg)

Haipeng Dai received the BS degree from the Department of Electronic Engineering, Shanghai Jiao Tong University, Shanghai, China, in 2010, and the PhD degree from the Department of Computer Science and Technology, Nanjing University, Nanjing, China, in 2014. His research interests include the areas of wireless charging, mobile computing, and data mining. He is a research assistant professor with the Departmen of Computer Science and Technology, Nanjing University. His research papers have been pub-

lished in many prestigious conferences and journals such as ACM MobiSys, ACM MobiHoc, ACM VLDB, ACM SIGMETRICS, ACM Ubi-Comp, IEEE INFOCOM, IEEE ICDCS, IEEE ICNP, IEEE SECON, IEEE IPSN, IEEE JSAC, the IEEE/ACM Transactions on Networking, the IEEE Transactions on Mobile Computing, the IEEE Transactions on Par allel and Distributed Systems, and the IEEE Transactions on Sensor Networks. He is an IEEE and ACM member. He serves/ed as poster chair of the IEEE ICNP'14. track chair of the ICCCN'19. TPC member of the IEEE INFOCOM’20, IEEE IWQoS’19, IEEE ICNP’14, IEEE ICC’14- 18, IEEE ICCCN’15-18 and the IEEE Globecom’14-18. He received the Best Paper Award from IEEE ICNP’15, Best Paper Award Runner-up from IEEE SECON’18, and Best Paper Award Candidate from IEEE INFOCOM’17.

![](images/123b33bdc8d0e5671d72a4c78b4310502f2b87e777213de96420611d8266eeb6.jpg)

Weijun Wang received the BS degree from the Department of Computer and Software, Nanjing University of Post and Telecommunication, Nanjing, China, in 2014, and the ME degree in computer technology from the PLA University of Science and Technology, Nanjing, China, in 2017. He is working towards the PhD degree in the Department of Computer Science and Technology, Nanjing University. His research interests include the UAV monitoring, MAC protocols in UAV networks and ad hoc networks

![](images/d9e7c3e72ac23ba2f73ea56a35658fd2aac1f1e6639e38b0d8bc691131363f12.jpg)

Jiaqi Zheng received the PhD degree from Nanjing University, in 2017. He is currently an assistant researcher in the Department of Computer Science and Technology, Nanjing University, China. His research area is computer networking, particu larly data center networks, SDN, and NFV. He was an assistant researcher with the City University of Hong Kong in 2015, and a visiting scholar at Temple University in 2016. He received the best paper award from IEEE ICNP 2015 and Doctorial Dissertation Award from ACM SIGCOMM China 2018. He is a member of the ACM and IEEE.

![](images/69662c5668fc4eebf47bef851ec4c9bdc8edbd94955ec55d712cebc5c7c6c205.jpg)

Nan Yu received the BS degree from the Depart ment of Computer Science and Technology, Jilin University, Changchun, Jilin, China, in 2015. She is working towards the PhD degree in the Department of Computer Science and Technology, Nanjing University, Nanjing, Jiangsu, China. Her research interests include the areas of wireless charging and device-free sensing.

![](images/2e9c0ea0aeac19ed625461a45f0632e497589f80e5c64b1d31ba5c97ede03351.jpg)

Guihai Chen received BS degree in computer software from Nanjing University, in 1984, the ME degree in computer applications from Southeast University, in 1987, and the PhD degree in computer science from the University of Hong Kong, in 1997. He is a professor and deputy chair of the Department of Computer Science, Nanjing University, China. He has been invited as a visiting professor by many foreign universities including Kyushu Institute of Technology, Japan, in 1998, University of Queensland, Australia, in

2000, and Wayne State University, USA, during Sep. 2001 to Aug. 2003. He has a wide range of research interests with focus on sensor networks, peer-to-peer computing, high-performance computer architec ture, and combinatorics. He is a member of the IEEE.

![](images/14131a2dc439d5a4cb9676faa7a99ab9c76ad3283e443d2010f6a2ff9b4b1438.jpg)

Wanchun Dou received the PhD degree in mechanical and electronic engineering from the Nanjing University of Science and Technology, China, in 2001. He is currently a full professor of the State Key Laboratory for Novel Software Tech nology, Nanjing University. From April 2005 to June 2005 and from November 2008 to February 2009, he respectively visited the Department of Computer Science and Engineering, Hong Kong University of Science and Technology, Hong Kong, as a visiting scholar. Up to now. he has chaired three National Natural Science Foundation of China projects and published more than 100 research papers in international journals and internationa conferences. His research interests include workflow, cloud computing, and service computing. He is a member of the IEEE.

![](images/cd89f4c166f3657b45f812e37d95c749fec4c814b6c94c15f05bb1f1f98bfef1.jpg)

Xiaobing Wu received the BS and ME degrees in computer science from Wuhan University, in 2000 and 2003, and the PhD degree in computer science from Nanjing University, in 2009. He is with the Wireless Research Centre, University of Canterbury, New Zealand. His research interests include the fields of wireless networking and communications, Internet of Things, and cyber physi cal systems. His publications appeared in the IEEE Transactions on Parallel and Distributed Systems, the ACM Transactions on Sensor Networks, IEEE INFOCOM, IEEE ICDCS, etc. He won the Honoured Mention Award in ACM MobiCom 2009 Demos and Exhibitions. He is a member of the IEEE.

" For more information on this or any other computing topic please visit our Digital Library at www.computer.org/csdl.
