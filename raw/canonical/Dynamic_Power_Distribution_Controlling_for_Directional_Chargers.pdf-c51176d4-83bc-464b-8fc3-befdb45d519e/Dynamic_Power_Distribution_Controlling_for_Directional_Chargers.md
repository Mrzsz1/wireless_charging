# Dynamic Power Distribution Controlling for Directional Chargers

Yuzhuo Ma<sup>∗†</sup>, Die Wu ´ <sup>∗†‡</sup>, Jing Gao<sup>∗†</sup>, Wen Sun<sup>§</sup>, Jilin Yang<sup>∗†</sup>, Tang Liu<sup>∗†</sup> 

<sup>∗</sup>College of Computer Science, Sichuan Normal University, Chengdu, Sichuan 610101, China <sup>†</sup>Visual Computing and Virtual Reality Key Lab, Sichuan Normal University, Chengdu, Sichuan 610068, China <sup>‡</sup>School of Information and Software Engineering, 

University of Electronic Science and Technology of China, Chengdu, Sichuan 610054, China <sup>§</sup>School of Cyberspace Security, Northwestern Polytechnical University, Xi’an, Shanxi 710071, China Email: {yuzhuoma, jinggao}@stu.sicnu.edu.cn, {wd, jilinyang, liutang}@sicnu.edu.cn, sunwen@nwpu.edu.cn 

Abstract—Recently, deploying static chargers to construct timely and robust Wireless Rechargeable Sensor Networks (WRSNs) has become an important research issue for solving the limited energy problem of wireless sensor networks. However, the established fixed power distribution lacks flexibility in response to dynamic charging requests from sensors and may render some sensors to be continuously impacted by destructive wave interference. This results in a gap between energy supply and practical demand, making the charging process less efficient. In this paper, we focus on the real-time sensor charging requests and formulate a dynamic power disTributIon controlling for Directional chargErs (TIDE) problem to maximize the overall charging utility. To solve the problem, we first build a charging model for directional chargers while considering wave interference and extract the candidate charging orientations from the continuous search space. Then we propose the neighbor set division method to narrow the scope of calculation. Finally, we design a dynamic power distribution controlling algorithm to update the neighbor sets timely and select optimal orientations for chargers. Our experimental results demonstrate the effectiveness and efficiency of the proposed scheme, it outperforms the comparison algorithms by 142.62% on average. 

Index Terms—directional charging, power distribution controlling, wave interference, wireless power transfer 

## I. INTRODUCTION

With the advance of Wireless Power Transfer (WPT) technology [1], Wireless Rechargeable Sensor Networks (WRSNs) [2]–[11] have witnessed the bloom in recent years. This progress has alleviated the long-standing challenge of limited energy capacity that used to hinder the development of wireless sensing. Moreover, the employment of directional chargers further promotes charging efficiency since their highgain and directional antennas enable the concentration of radiated energy in a narrow beam, thereby enhancing power intensity at some intended angle. In light of this, much effort has been devoted to exploring network performance improvements for directional charging [12]–[24]. 

In the directional charging network, static chargers, which are deployed with fixed locations and orientations to emit energy continuously, are preferable to mobile chargers in many scenarios. Their merits of timeliness and robustness empower them to better cope with the changes in the network without concerning their own energy supply [14]–[17]. It is noted that, in order to prevent coverage holes and simultaneously improve charging efficiency, it is necessary and unavoidable for the charging ranges to overlap when deploying static chargers. In that case, sensors located within these overlaps will receive multiple electromagnetic waves from chargers concurrently, and subsequent wave interference [25] will exert a significant impact on the final power received by sensors. For example, destructive interference may result in sensors covered by more chargers receiving less power instead, which neither fully utilizes the charger resources nor achieves the desired results. 

![](images/c462970803c8ce7ed100df6e7d0dc5e45e30f96d088eebe397025c1a6caa0753.jpg)



Fig. 1. Power distribution under different chargers’ orientations.


The state-of-art literature [26] has devoted attention to wave interference, which refers to the power distribution obtained from an elaborated charger placement algorithm. And they carefully select sensor deployment locations in the high-power interference enhanced regions to avoid negative effects. This scheme indeed promotes charging efficiency, but there are two negligible shortages: (i) achieving sensor deployment accuracy at the millimeter level presents a huge challenge in terms of network construction costs, it is also not a general solution in scenarios where a large number of sensors need to be randomly scattered [27]; (ii) once chargers are deployed, the power distribution of the whole network becomes fixed, lacking flexibility in response to dynamic charging requests from sensors and little improvement in the utilization of charger resources. 

A more ideal charging method would be to always locate sensors with charging requests in high-power interference enhanced regions. And once a sensor is fully charged, there is no longer preferential treatment for it. This entails accommodating the diverse requests of different sensors and tailoring the power distribution accordingly. Take Fig. 1 as an example, which shows the power distribution of three directional chargers. The colored sector presents the charging range of each charger with brighter/darker indicating interference enhanced/weakened regions. Let’s consider the case where sensor $s _ { 1 }$ with charging request can first receive considerable power. Once $s _ { 1 }$ is fulfilled, sensor $s _ { 2 } ,$ , which was originally located within the interference weakened region, sends a charging request. To satisfy its demand, two chargers change their orientations, introducing a new power distribution. With this adjustment, $s _ { 2 }$ can now receive a significantly higher power than before without any movement. The rationale leading to these differences is the nonlinearity of the interference effect resulting from multiple waves and the change in the number of chargers involved. 

This example inspires us to understand the importance of power distribution control in addressing the dynamic demands of the network. In this paper, we focus on a more practical scenario where sensor locations can not be further adjusted and they will launch real-time charging requests according to their dynamic residual lifetimes. Our goal is to respond to sensors’ charging requests in an online manner by dynamically controlling the power distribution and finally maximizing the overall charging utility. Thus, we state our dynamic power disTributIon controlling for Directional chargErs (TIDE) problem as follows. Given a number of rotatable directional chargers with fixed locations and a set of sensors, how to design an optimal dynamic power distribution controlling scheme to maximize the overall charging utility for all sensors while taking the wave interference into consideration. 

Generally, there are three main challenges in our problem. 

The first challenge arises from the nonlinearity when building the charging model for directional chargers while considering wave interference. It encompasses multiple factors such as charging distances, orientations, the number of chargers involved in wave interference, and so on. 

The second challenge lies in the difficulty of ensuring the desired power distribution always appear around sensors with charging requests by adjusting the orientations of the chargers. This is because the orientation decisions of chargers will interact with each other due to wave interference, and this interaction does not only occur between directly adjacent chargers, it is more transitive and may affect chargers that are far away without any overlap. 

The third challenge is that the real-time charging requests from sensors ask us to make decisions in an online manner, while the optional orientations at different times are dynamically changing and the charging demand of each sensor is different. Besides, there are infinite orientations available for a charger and picking up an orientation from the candidate set is similar to solving a multiple-choice knapsack problem [28], which is NP-hard. 

To tackle these challenges, we first incorporate the power amplitude relationship into the directional charging model and figure out the influence of various factors on the power received by sensors. Then, we extract the dominant sensor set to filter candidate orientations without causing performance loss for the problem of continuous search space and a further division is carried out considering the negative effects of destructive interference. We also propose the concept of neighbor set to organize the chargers that will interact with each other in every time slot. Afterwards, we accordingly design a dynamic power distribution controlling algorithm to update the neighbor sets timely and select optimal orientations for chargers that maximize the charging utility of the whole network. 

The main contributions of this work are summarized below. 

• To the best of our knowledge, we are the first to study the dynamic power distribution controlling of directional chargers that considers both sensors’ online requests and wave interference. We build a practical charging directional model incorporating wave interference for this. 

• We develop a candidate sensor sets extraction algorithm to reduce the computation complexity and the negative impact of destructive interference. We prove the NPhardness of the charger orientation selection process and further design a dynamic power distribution controlling algorithm to determine the charger orientations according to the sensor requests. 

• Extensive simulations and field experiments are conducted to verify our scheme, the results show that our scheme outperforms other comparison algorithms by 142.62% on average in charging utility. 

## II. PRELIMINARIES

## A. Network Model

Suppose there are N omnidirectional sensors denoted as ${ \cal S } = \{ s _ { 1 } , s _ { 2 } , . . . , s _ { N } \}$ located on a 2D plane Ω with battery energy capacity b. The energy consumption rate and residual energy of each sensor are denoted as $e c _ { i }$ and $r e _ { i } .$ , respectively. Accordingly, the residual lifetime $r l _ { i }$ of $s _ { i }$ is $\textstyle { \frac { r e _ { i } } { e c _ { i } } }$ . There are also M static directional chargers $C = \{ c _ { 1 } , c _ { 2 } , \ldots , c _ { M } \}$ deployed in advance to provide charging service for sensors, which can continuously rotate within [0, 2π). 

Basically, once the $r l _ { i }$ is less than a lifetime threshold $\Upsilon _ { l } .$ , sensor $s _ { i }$ will launch a charging request $R E Q _ { i } =$ $\left( t , s _ { i } , r e _ { i } , e c _ { i } , b - r e _ { i } \right)$ to the chargers that possibly can emit power to it, where t is time point and $b \mathrm { ~ - ~ } r e _ { i }$ represents the amount of energy required to be replenished. A request queue $S _ { j } ^ { r e q }$ will be maintained in each charger to store the received charging requirements and the chargers will schedule their orientations thus controlling the power distribution in an online manner to serve the requests when the queue updates, $i . e .$ , a sensor sends a new charging request or a request is fulfilled. A power distribution will last until a new batch of charging orientations are calculated with the arrival of the next update of the queue. 

Considering the continuity of time, we apply a time discretization mechanism to partition the time into multiple time slots with uniform duration $\Delta t .$ For simplicity, we assume the request queue always updates at the beginning of a time slot. Since the orientation transform process for rotatable cradles where directional chargers are mounted only lasts a few seconds or even shorter [29], we can set the duration of a time slot as dozens of seconds, so that the calculation for charging orientations and the rotation of corresponding chargers can be finished within a time slot. Besides, it is worth mentioning that the charging duration to fully charge a sensor always costs dozens of minutes to hours [11], [12], [30] and a sensor with full battery capacity can work for days to weeks [31]. Therefore, slots of tens of seconds long will also prevent a new request from being unresponsive for a long time. 

![](images/adfebe459843c17b5984ba021620470e280a6ac54fa64bfdcc2f0fff57e42cc9.jpg)



Fig. 2. Charging model.


## B. Charging Model

We demonstrate our charging model with reference to the general model proposed in [15]. Typically, the charging area of a directional charger is modeled as a sector with charging angle $\varPhi$ and radius $D ,$ only the omnidirectional sensors located within the scope can receive non-negligible power. As shown in Fig. 2, there are two chargers $c _ { 1 }$ and $c _ { 2 }$ with working orientations denoted by vectors $\overrightarrow { r _ { \theta _ { 1 } } }$ and $\begin{array} { r } { \overrightarrow { r _ { \theta _ { 2 } } } . } \end{array}$ , respectively. Obviously, $s _ { 1 }$ can be charged by them concurrently while $s _ { 2 }$ only receives power from $c _ { 1 } .$ , and $s _ { 3 }$ can not be replenished because it has out of the scope of the sector of any chargers. 

To mathematically describe the impact of the wave interference in the directional concurrent charging scenario, we formulate a charging model incorporated with it. First, the radiated wave arriving at sensor $s _ { i }$ from a single directional charger $c _ { j }$ can be presented as: 

$$
a (t) = \frac {A _ {0}}{| | \hat {s _ {i} c _ {j}} | |} \cos (2 \pi f t - \frac {2 \pi}{\lambda} | | s _ {i} c _ {j} | |).\tag{1}
$$

In Eq. (1), $A _ { 0 }$ is the amplitude of the wave, $\left| \left| s _ { i } c _ { j } \right| \right|$ represents the distance between sensor $s _ { i }$ and charger $c _ { j }$ $\begin{array} { r } { | | \dot { s _ { i } c _ { j } } | | = \frac { | | s _ { i } c _ { j } | | + \beta } { \sqrt { \alpha } } } \end{array}$ refers to the attenuation factor for wave propagation due to the empirical model proposed in [32], the $\begin{array} { r } { \alpha = \frac { G _ { s } G _ { r } } { L _ { p } } ( \frac { \lambda } { 4 \pi } ) ^ { 2 } } \end{array}$ , where $G _ { s } , \ G _ { r }$ are charger and sensor antenna gain, respectively, and λ is the wavelength, $L _ { p }$ is the polarization loss. $\beta$ is a parameter to adjust the Friis’ free space equation for short distance transmission. 

Given a period of time of duration T , thereby, the average power arrived at $s _ { i }$ over that period can be calculated by $\begin{array} { r } { { \frac { 1 } { T } } \int _ { - \frac { T } { 2 } } ^ { \frac { T } { 2 } } [ a ( t ) ] ^ { 2 } \mathrm { d } t } \end{array}$ . Accordingly, we have the power arrived at $s _ { i }$ from a single charger as: 

$$
p _ {s _ {i} | c _ {j}} ^ {\theta_ {j}} = \left\{ \begin{array}{l l} \frac {A _ {0} ^ {2}}{2 | | s _ {i} \hat {c} _ {j} | | ^ {2}} & 0 \leq | | s _ {i} c _ {j} | | \leq D, \\ & \overrightarrow {s _ {i} c _ {j}} \cdot \overrightarrow {r _ {\theta_ {j}}} - | | s _ {i} c _ {j} | | \cos (\varPhi / 2) \geq 0, \\ 0, & o t h e r w i s e. \end{array} \right.\tag{2}
$$

Actually, for sensor $s _ { i } ,$ all chargers within D on the plane are its potential providers, we can obtain a set $C _ { i }$ based on the known locations of chargers and sensors, where $| C _ { i } | = m _ { i }$ Thus, the combined wave arrived at $s _ { i }$ can be presented as: 

$$
\begin{array}{r l} & A _ {s _ {i} | C _ {i}} (t) = A _ {0} ^ {i} \cos (2 \pi f t - \phi) \\ & \qquad = \sum_ {j = 1} ^ {m _ {i}} \frac {A _ {0}}{| | s _ {i} c _ {j} | |} \cos (2 \pi f t - \frac {2 \pi}{\lambda} | | s _ {i} c _ {j} | |), \end{array}\tag{3}
$$

where $\begin{array} { r } { A _ { 0 } ^ { i } = \sqrt { m _ { i } A _ { 0 } ^ { 2 } + 2 A _ { 0 } ^ { 2 } \sum _ { j > l } ^ { m _ { i } } \sum _ { l = 1 } ^ { m _ { i } } \cos ( 2 \pi \frac { | | s _ { i } c _ { j } | | - | | s _ { i } c _ { l } | | } { \lambda } ) } . } \end{array}$ which presents the combined amplitude arrived at sensor $s _ { i } , \phi$ is the phase. Considering the limited space, we will use $\Delta \varphi _ { j l }$ instead of $2 \pi { \frac { | | s _ { i } c _ { j } | | - | | s _ { i } c _ { l } | | } { \lambda } }$ in the following text. Similarly, the corresponding combined power arrived at $s _ { i }$ can be written as follows: 

$$
\begin{array}{r l} & P _ {s _ {i} | C _ {i}} = \frac {1}{T} \int_ {- \frac {T}{2}} ^ {\frac {T}{2}} [ A _ {s _ {i} | C _ {i}} (t) ] ^ {2} \mathrm{d} t \\ & = \frac {1}{T} \int_ {- \frac {T}{2}} ^ {\frac {T}{2}} \left[ \sum_ {j = 1} ^ {m _ {i}} \frac {A _ {0}}{| | s _ {i} c _ {j} | |} \cos (2 \pi f t - \frac {2 \pi}{\lambda} | | s _ {i} c _ {j} | |) \right] ^ {2} \mathrm{d} t \\ & = \frac {A _ {0} ^ {2}}{2} \left(\sum_ {j = 1} ^ {m _ {i}} \frac {1}{| | s _ {i} c _ {j} | | ^ {2}} + \sum_ {j > l} ^ {m _ {i}} \sum_ {l = 1} ^ {m _ {i}} \frac {2 \cos (2 \pi \frac {| | s _ {i} c _ {j} | | - | | s _ {i} c _ {l} | |}{\lambda})}{| | s _ {i} c _ {j} | | \cdot | | s _ {i} c _ {l} | |}\right) \\ & = \sum_ {j = 1} ^ {m _ {i}} p _ {s _ {i} | c _ {j}} ^ {\theta_ {j}} + 2 \sum_ {j > l} ^ {m _ {i}} \sum_ {l = 1} ^ {m _ {i}} \sqrt {p _ {s _ {i} | c _ {j}} ^ {\theta_ {j}} p _ {s _ {i} | c _ {l}} ^ {\theta_ {l}}} \cos (\Delta \varphi_ {j l}). \end{array}\tag{4}
$$

Note that, if and only if the orientation of a charger is turned to enable the effective area to cover $s _ { i } ,$ , can $s _ { i }$ receive the nonnegligible power. Literally, $s _ { i }$ can be charged concurrently by all chargers in $C _ { i }$ only when the following condition is met: $\forall c _ { j } \in C _ { i } , \overrightarrow { r _ { \theta _ { j } } } - \vert \vert s _ { i } c _ { j } \vert \vert \cos ( \varPhi / 2 ) \geq 0 , 0 \leq \vert \vert s _ { i } c _ { j } \vert \vert \leq D$ With the Eq. (4), it is thus possible to visualize the power distribution in the network, and the effect of the influencing factors can also be clearly seen. 

## C. Utility Model

Committed to maintaining a high-efficiency network, the dynamic power distribution should always be targeted to serve the requested sensors. To avoid the sensor without charging requirement preempting the priority of the requested sensor, accordingly, we stipulate the charging utility generates only when the sensors in the request queue are served. Limited by the rated power $P _ { t h }$ of the practical electric circuits for each rechargeable sensor, the utility function of sensor $s _ { i }$ in $S _ { j } ^ { r e q }$ during $k _ { t h }$ time slot can be written as: 

$$
u (P _ {s _ {i}} ^ {k}) = \left\{ \begin{array}{l l} \frac {P _ {s _ {i}} ^ {k} \cdot \Delta t}{b - r e _ {i}}, & P _ {s _ {i}} ^ {k} <   P _ {t h}, \\ \frac {P _ {t h} \cdot \Delta t}{b - r e _ {i}}, & P _ {s _ {i}} ^ {k} \geq P _ {t h}, \end{array} \right.\tag{5}
$$

where $P _ { s _ { i } } ^ { k }$ is the power arrived at $s _ { i }$ in the k-th time slot under a certain power distribution. As we can see, the utility is broadly proportional to the arrived power, where a larger utility relies on higher arrived power at the requested sensor. With the shorter charging duration to fulfill the requests under considerable power, the chargers are able to serve more following requested sensors. We formulate this fraction to normalize the charging utility so that it will accumulate to 1 when the request of $s _ { i }$ is fulfilled after several time slots. 

## D. Problem Formulation

In the scenario where directional chargers can rotate with their orientations varying from 0 to $2 \pi$ to toggle power distribution, we define $\theta _ { j } ( k )$ as the orientation of charger $c _ { j }$ at k-th time slot. Thus, the power arrived at $s _ { i }$ $k \mathrm { - }$ th time slot can be written as $\begin{array} { r c l } { P _ { s _ { i } } ^ { k } } & { = } & { \sum _ { j = 1 } ^ { m _ { i } } p _ { s _ { i } | c _ { j } } ^ { \theta _ { j } ( k ) } + } \end{array}$ $\begin{array} { r } { 2 \sum _ { j > l } ^ { m _ { i } } \sum _ { l = 1 } ^ { m _ { i } } \sqrt { p _ { s _ { i } | c _ { j } } ^ { \theta _ { j } ( k ) } p _ { s _ { i } | c _ { l } } ^ { \theta _ { k } ( k ) } } \cos ( 2 \pi \frac { | | s _ { i } c _ { j } | | - | | s _ { i } c _ { l } | | } { \lambda } ) } \end{array}$ 

Our goal is to dynamically control the power distribution by scheduling the charging orientations of static directional chargers in each time slot to maximize the charging utility $( i . e .$ , fulfill the requests as much as possible) of the whole network lifetime (considered as K time slots). Formally, we define the problem of dynamic power disTributIon controlling with Directional chargErs (TIDE) as follows: 

$$
\begin{array}{l l} \text {(P1)} & \max \quad U (t o t a l) = \sum_ {k = 1} ^ {K} \sum_ {s _ {i} \in S ^ {r e q}} u (P _ {s _ {i}} ^ {k}), \\ & s. t. \quad K \in \mathbf {Z} _ {0} ^ {+}. \end{array}\tag{6}
$$

where the $S ^ { r e q }$ denotes the set of all requested sensors of the whole network. 

## III. SOLUTION

In this section, we aim to determine the specific power distribution at each queue update to address the TIDE problem during the whole lifetime of sensor networks. First, we identify all feasible power distributions by extracting a finite number of candidate sensor sets and obtaining the corresponding candidate charging orientations from the continuous solution space. Then, we accordingly reformulate the TIDE problem and narrow the scope of calculation at each decision point by proposing the neighbor set. Finally, we design a dynamic power distribution controlling algorithm to react to real-time charging requests of different sensors in the network. 

## A. Candidate Sensor Sets Extraction

Despite there being infinite potential power distribution resulting from the continuity of the orientations of each charger, the fixed location relationship between chargers and sensors indicates only a finite number of power distributions will impact the final power sensors received. Thus, in this subsection, we aim to identify this limited set of power distributions by extracting some of the representative orientations according to the following definition: 

Definition 1. Dominant Sensor Set: given a set of sensors $S _ { i }$ that covered by a charger with orientation $\theta _ { i } ,$ , if there does not exist a sensor set $S _ { j }$ be covered by the same charger with another orientation $\theta _ { j }$ such that $S _ { j } \supset S _ { i }$ , then $S _ { i }$ is called Dominant Sensor Set of this charger. 

We give an example of one charger and several sensors to briefly sketch the process of the dominant sensor set extracting in Fig. 3, which is also presented in lines 1-5 of Algorithm 1. Basically, the charger rotates continuously to cover the sensor one by one and records all the dominant sensor sets. The charger firstly covers the $s _ { 1 }$ and $s _ { 2 }$ , when trying to cover the next sensor $s _ { 3 }$ , the $s _ { 1 }$ will subsequently leave the current sensor set, thus $\{ s _ { 1 } , s _ { 2 } \}$ is a dominant sensor set. With the successive departure of $s _ { 1 }$ and $s _ { 2 } , s _ { 3 }$ is covered and $s _ { 4 }$ is then added. Keeping rotating, $s _ { 5 }$ will not be further added without missing $s _ { 3 }$ , therefore, $s _ { 3 }$ and $s _ { 4 }$ form a dominant sensor set. Similarly, the final obtained dominant sensor sets are $\{ s _ { 1 } , s _ { 2 } \}$ $\{ s _ { 3 } , s _ { 4 } \} , \{ s _ { 4 } , s _ { 5 } \}$ and $\left\{ s _ { 6 } , s _ { 7 } , s _ { 8 } , s _ { 9 } \right\}$ 

The extraction of dominant sensor sets enables the most representative power distribution to be reproduced with the least number of alternative orientations for chargers when serving the requested sensors, which sharply reduces the computational complexity. However, this behavior unintentionally binds the sensors, providing a charging service to a sensor will simultaneously charge other sensors in the same dominant sensor set. It turns out that when one or more chargers rotate to cover a set of sensors already covered by one charger, those sensors may have completely different effects with the new power distribution. Some sensors will gain significantly more power from being covered by multiple chargers concurrently, which is far greater than being covered by only one charger, while some sensors may be significantly weakened, far less than being served by fewer chargers. The rationale behind this diversity is the effect of wave interference, interrelating to the different locations of sensors and different distance relationships with chargers. Formally, we call such a sensor a black sheep (red dot in Fig. 3) when its energy obtained does not increase with the number of chargers that will charge it and others called premium sensor (black dot in Fig. 3). Considering the situation that enhancing a sensor may weaken the overall power of other sensors when selecting a power distribution, we should separate the binding of sensors by further dividing the dominant sensor set into several subsets. 

Here our goal is to find all possible orientations that can make the resulting power distributions sufficiently adaptive to the diverse requests from the sensors. Basically, we will discuss the process of subset partitioning for the following three dominant sensor sets to perfect our solutions. 

• For the dominant sensor sets that contain black sheep only, we should find all subsets of them. The rationale is that whichever black sheep is added to form a new subset may have a debilitating effect on the utility of the previous subset. It is caused by the difference when different sensors are covered by the same power distribution. Thus, the obtained subsets of the dominant sensor sets that contain black sheep only (blue sectors in Fig. 3), $e . g . \ \{ s _ { 1 } , s _ { 2 } \} , \ \mathrm { a r e } \ \{ s _ { 1 } \} , \ \{ s _ { 2 } \}$ 

![](images/91c4501f3dfd094624dc7a4c9b3abc646040e79a8ed8cf34f3279358898558ea.jpg)



Fig. 3. Extraction of candidate sensor sets.


Algorithm 1: Extraction of candidate sensor sets and candidate charging orientations

Input: The set of chargers C, the set of sensors S, the farthest charging distance D, and other necessary parameters
Output: All candidate sensor sets and candidate charging orientations $\Theta$ 1 Find the subset of sensors $S_{j}$ that could possibly cover by charger $c_{j}$ and the subset of chargers $C_{i}$ that could possibly cover sensor $s_{i}$ ;

2 Initialize the orientation of the charger to 0;

3 Rotate the charger $c_{j}$ anticlockwise to cover the sensors in $S_{j}$ one by one until the sensor currently being covered is about to leave. Terminate the rotating process once the rotation angle is larger than $2\pi$ ;

4 Add the current covered set of sensors to the collection of dominant sensor sets;

5 Rotate the charger anticlockwise until a new sensor in $S_{j}$ is added in the covered set. Terminate the rotating process once the rotation angle is larger than $2\pi$ . If not, goto line 4;

6 Calculate and find all black sheep sensors;

7 Divide the dominant sensor sets into 3 categories and obtain all feasible subsets;

8 Union all dominant sensor sets and subsets as the collection of candidate sensor sets;

9 Return the collection of all candidate sensor sets and corresponding candidate charging orientations $\Theta$ ; 

• For the dominant sensor sets that do not include any black sheep, we accordingly do not require a subset of them. Since these premium sensors show consistency when the power distribution alters, the dominant sensor sets are more conducive to maintaining the survival of sensors. Thus, in terms of the dominant sensor sets without black sheep (pink and green sectors in Fig. 3), $e . g . \ \{ s _ { 3 } , s _ { 4 } \}$ and $\{ s _ { 4 } , s _ { 5 } \}$ , the subsets are not necessary. 

• For other dominant sensor sets that contain both premium sensors and black sheep, we rotate the sector starting from the first sensor in the set to obtain the subsets. Specifically, we record all subsets, but overwrite the previous subset when a premium sensor is newly added, and keep the previous subset unchanged when a premium sensor leaves. Note that if sensors and the charger covering them are collinear such as the $s _ { 6 } , s _ { 7 }$ , they are spontaneously served by the same orientation, we deal with the special case as only black sheep exist. Thus, for the dominant sensor sets that contain both (yellow sectors in Fig. 3), ${ e . g . \ \left\{ { s } _ { 6 } , { s } _ { 7 } , { s } _ { 8 } , { s } _ { 9 } \right\} }$ , we get subsets $\{ s _ { 6 } , s _ { 7 } \} , \{ s _ { 8 } , s _ { 9 } \}$ and $\left\{ s _ { 9 } \right\}$ , respectively. 

Finally, the candidate sensor sets are the union of dominant sensor sets and subsets, the corresponding orientations enable us to provide effective power distributions for the requested sensors no matter how power received by sensor is affected by the interference of the wave. Take Fig. 3 as an example, the candidate sensor sets are $\{ s _ { 1 } , s _ { 2 } \} , \ \{ s _ { 1 } \} , \ \{ s _ { 2 } \} , \ \{ s _ { 3 } , s _ { 4 } \}$ $\left\{ s _ { 4 } , s _ { 5 } \right\} , \left\{ s _ { 6 } , s _ { 7 } , s _ { 8 } , s _ { 9 } \right\} , \left\{ s _ { 6 } , s _ { 7 } \right\} , \left\{ s _ { 8 } , s _ { 9 } \right\}$ and {s<sub>9</sub>}. The detailed process is shown in Algorithm 1. 

## B. Problem Reformulation

Suppose the corresponding candidate charging orientation sets of the obtained candidate sensor sets for charger $c _ { j }$ is $\Theta _ { j } ,$ the p-th candidate orientation in $\theta _ { j }$ is denoted as $\theta _ { j } ^ { p }$ . Let $\boldsymbol { x } _ { j , k } ^ { p }$ be a boolean variable indicating whether the p-th orientation is selected or not in the k-th time slot. Then the problem TIDE can be formulated as: 

$$
\begin{array}{l} \text {(RP1)} \quad m a x \quad U (t o t a l) = \sum_ {k = 1} ^ {K} \sum_ {s _ {i} \in S ^ {r e q}} u (P _ {s _ {i}} ^ {k}) \\ = \sum_ {k = 1} ^ {K} \sum_ {s _ {i} \in S ^ {r e q}} \left(\sum_ {j = 1} ^ {m _ {i}} x _ {j, k} ^ {p} p _ {s _ {i} | c _ {j}} ^ {\theta_ {j} (k)} \right. \\ \left. + 2 \sum_ {j > l} ^ {m _ {i}} \sum_ {l = 1} ^ {m _ {i}} x _ {j, k} ^ {p} x _ {l, k} ^ {q} \sqrt {p _ {s _ {i} | c _ {j}} ^ {\theta_ {j} (k)} p _ {s _ {i} | c _ {l}} ^ {\theta_ {l} (k)}} \cos (\Delta \varphi_ {j l})\right). \\ s. t. \sum_ {p = 1} ^ {| \Theta_ {j} |} x _ {j, k} ^ {p} = 1, (x _ {j, k} ^ {p} \in \{0, 1 \}, x _ {l, k} ^ {q} \in \{0, 1 \}). \end{array}\tag{7}
$$

![](images/859bbb11cbdbcd048e8888a8c40866a54d0d56aed935794b3d09970789646130.jpg)



Fig. 4. The structure changes of the neighbor sets.


## Theorem 1. The TIDE problem RP1 is NP-hard.

Proof: We omit the proof due to space limitations. 

## C. Neighbor Set Division

By serving a request, we mean the directional charger rotates to a corresponding orientation and charges the requested sensors until the power distribution changes due to the updated request queue. After extracting all feasible power distributions, we focus on which chargers have to rotate to alter the power distribution at each decision point. Compared with bothering all chargers to change their orientations, in this subsection, we propose a Neighbor Set Division method for the sake of a faster calculation process. Basically, we present a concept to assist analysis: 

Definition 2. Neighbor Set: refers to a group of chargers in a WRSN whose orientation selection affects each other due to the existence of requested sensors. Specially, a single charger can also serve as a neighbor set independently when it does not interact with any other chargers. 

For each charger to be a member of the neighbor set, the necessary condition is that there must be at least one requested sensor within its charging range. At this time, a charger’s orientation determination directly affects whether the nearby chargers can realize the concurrent charging for the commonly covered sensors. Specifically, chargers with requested sensors in their overlaps will invariably affect each other and belong to the same neighbor set, while the chargers that do not harbor any requested sensor in the overlap or do not have any overlaps in their possible charging range will also exhibit mutual influence. That is, if charger A influences charger B, and charger B influences charger C, charger A will accordingly influence charger C with the chain reaction of interconnected chargers. These correlations among the chargers underscore the influence scope of selecting a certain orientation and the partition of the neighbor sets can effectively help us define this scope. Consequently, when the charging demand of sensors varies, we can select the orientations at the level of the neighbor sets instead of the individual charger. 

Since charging requests may arrive or be fulfilled at any time, the structure of the neighbor sets will accordingly undergo continuous modification according to the changes in the intersection $( i . e .$ , commonly covered requested sensors) of request queues for each charger. It will not only affect the chargers with possible charging range overlapping but also may establish/cut off the correlation of the chargers that are farther away. We summarize how the structure of the neighbor sets change when a request arrives, or a request is fulfilled, respectively. Formally, we denote the collection of the current neighbor sets in the network as $\{ { \mathcal { N } } _ { 1 } , { \mathcal { N } } _ { 2 } , \cdots \}$ 

• When a sensor $s _ { i }$ launches a request, there are three possible cases of changes in the structure of the neighbor sets: (1) if there is a current neighbor set harbors all chargers that can cover $s _ { i } ( i . e . , \ C _ { i } .$ , which represents the charger set that can possibly cover $s _ { i } ,$ is a subset of some current neighbor set), the neighbor sets remain unchanged; (2) if there is no current neighbor set harbors chargers that can cover $s _ { i } , \ ( i . e .$ , the intersection of $C _ { i }$ and any current neighbor set is null), $C _ { i }$ independently becomes a new neighbor set; (3) if there is one or more neighbor sets harbor chargers that can cover $s _ { i } ~ ( i . e . , ~ C _ { i }$ has intersection with one or more neighbor sets), $C _ { i }$ merges with these neighbor sets into a bigger set. 

• When a requested sensor $s _ { i }$ is fulfilled, there are three possible cases of changes in the structure of the neighbor sets: (1) if $C _ { i }$ is used to constitute the exclusive neighbor set and there is no other requested sensor in the request queues of the chargers in $C _ { i }$ , the neighbor set used to serve $s _ { i }$ does not exist anymore; (2) if the queues of some chargers in $C _ { i }$ are empty while other chargers still have commonly requested sensors in their queues, the neighbor set used to serve $s _ { i }$ gets rid of a corresponding proper subset (contains no more than $| C _ { i } | - 1$ and no less than 0 elements) of $C _ { i } ; ( 3 )$ if $s _ { i }$ is the only common covered requested sensor amongst the chargers in $C _ { i }$ , the neighbor set used to serve $s _ { i }$ is split into several subsets. 

Fig. 4 demonstrates a neighbor set structure change process as sensor charging demands change. We consider Fig. $4 ( \mathrm { a } )$ as the neighbor set structure at a certain moment, where neighbor set ${ \mathcal { N } } _ { 1 } .$ consisting of chargers $c _ { 1 }$ and $c _ { 2 } ,$ , serves requested sensor $s _ { 2 } ,$ , and ${ \mathcal { N } } _ { 2 } ,$ comprising only $c _ { 4 } ,$ serves $s _ { 5 }$ . If $s _ { 1 }$ launches a charging request at this time, $\{ c _ { 5 } \}$ independently becomes a neighbor set ${ \mathcal { N } } _ { 3 }$ as shown in Fig. 4(b) since $\{ c _ { 5 } \}$ does not intersect with the existing neighbor sets $\mathcal { N } _ { 1 } , \mathcal { N } _ { 2 }$ . On the contrary, once $s _ { 1 }$ is fulfilled, ${ \mathcal { N } } _ { 3 }$ will not exist anymore since there is no element in the request queue of $c _ { 5 }$ . But if it is the $s _ { 4 }$ that launches a request as shown in Fig. 4(c), the neighbor sets $\mathcal { N } _ { 1 } , \mathcal { N } _ { 2 }$ and the $C _ { 4 }$ which contains $c _ { 1 } , c _ { 3 } , c _ { 4 }$ merge into one single set as $\{ c _ { 1 } , c _ { 2 } , c _ { 3 } , c _ { 4 } \}$ . Once $s _ { 4 }$ is fulfilled, it leaves from the request queues of the chargers $c _ { 1 } , \ : c _ { 3 }$ and $c _ { 4 } ,$ , the current neighbor set is then split into two neighbor sets $\mathcal { N } _ { 1 }$ and $\mathcal { N } _ { 2 }$ . The situation that $s _ { 3 }$ has a charging request and then $s _ { 2 }$ is fulfilled is not illustrated in detail in Fig. 4 since these two cases are very simple and do not cause changes in the number of neighbor sets. 

![](images/7b4c5c2af868d92ef2d013e00b56efc07be474f59bf7549cfb5d4df7f211ab55.jpg)



Fig. 5. The pruning process.


## D. Dynamic Power Distribution Controlling Algorithm

After discretizing the orientations and narrowing the calculation range, in this subsection, we propose a dynamic power distribution controlling algorithm to solve the TIDE problem. 

Note that even if we can control the power distribution only at the neighbor set level, there are still many options available due to the mutual effect of chargers, which increases exponentially with the number of chargers in the neighbor set. In order to reduce the impact of this mutual influence during dynamic control of power distribution, we also include the combinations of orientations that directly overlap into the candidate pool. Take Fig. 5 as an example, let’s consider a neighbor set consisting of four chargers: A, B, C, D. For each charger X, $X _ { i }$ is its candidate orientation. The numerical value corresponding to each orientation represents the power that can be provided by selecting this orientation. When determining the orientation for $A ,$ our options are no longer limited to $A _ { 1 }$ and $A _ { 2 }$ but $A _ { 2 } B _ { 1 }$ is included due to their mutual influence. $\boldsymbol { \mathrm { B y } }$ this means, once $A _ { 2 } B _ { 1 }$ is selected, it will not affect the subsequent outcome, thus reducing the difficulty of controlling. Besides, the addition of new alternatives remains manageable since the number of orientations that can directly overlap is limited. Even though all sensors have charging requests, we can employ the following pruning strategies to ensure optimal orientation selection while maintaining low computational complexity: 

• Charger number limit pruning: since the orientation collections are added as candidate options, the depth of each branch (formed by each charger’s orientation) is different, which may be 1 to $| \mathcal { N } |$ . Basically, when each charger has already been searched in the current branch, the branch can be terminated directly and the current optimal solution can be returned. For example, the $\{ A _ { 1 } , B _ { 2 } C _ { 1 } D _ { 1 } \}$ or $\{ A _ { 2 } , B _ { 2 } C _ { 1 } D _ { 1 } \}$ branch will stop at depth 2 since all chargers are already searched. 

Algorithm 2: Dynamic power distribution controlling
Input: Candidate charging orientations $\Theta$ , energy capacity of each sensor $b$ , lifetime threshold $\Upsilon_l$ and number of considered time slots $K$ for the whole network
Output: Power distributions $A$ for all chargers
1 Update the set $S_j^{req}$ of sensors with charging requirements that covered by charger $c_j$ ;
2 while $k \neq K$ do
3 if $s_i$ sends a charging request, i.e., $\frac{re_i}{ec_i} < \Upsilon_l$ then
4 Compare $C_i$ with current neighbor sets and update neighbor set structure according to their relationships;
5 Select the optimal power distribution for the newly generated neighbor set $\mathcal{N}(c_j)$ with the pruning strategies, and add it to $A$ ;
6 if $s_i$ is fulfilled then
7 Check the request queue for each charger in $C_i$ and split the neighbor sets according to the rest requested sensors;
8 Select the optimal power distribution for the newly generated neighbor set $\mathcal{N}(c_j)$ with the pruning strategies, and add it to $A$ ; 

• Optimal pruning: if the optimal branch under the current depth is found to be smaller than the previously searched one, the branch can be pruned and directly returned. For example, at depth 2, we record the current optimal branch as $\{ A _ { 1 } , B _ { 3 } \}$ , for the $\{ A _ { 1 } , B _ { 2 } C _ { 1 } \}$ branch, even the following chargers offers optimal orientations, it still will not be better than the current optimal, it is thus pruned. 

Greedy-based pruning: during the search processing, for the branches at the same depth and having the same chargers searched, the one that can not be the local optimal will definitely not be the candidate optimal orientation. By selecting the best branch under the same conditions, we can prune off other branches accordingly. For example, at depth 1, we can deduce $A _ { 2 }$ branch will not serve as the optimal branch. 

Therefore, we have obtained $\{ A _ { 1 } , B _ { 3 } , C _ { 1 } D _ { 1 } \}$ as the optimal solution for the example in Fig. 5. When a new request leaves or arrives, a new round of calculations will be conducted, and we can dynamically control the power distribution throughout the entire network cycle meanwhile maximizing the charging utility. The detailed process of the dynamic power distribution controlling algorithm is given in Algorithm 2. 

## IV. SIMULATION

## A. Simulation Setup

We consider a 20m × 20m WRSN, with 8 directional chargers and 12 omnidirectional sensors randomly distributed. We set the battery capacity of each sensor $b = 5 0 J$ , and the u<sup>ti</sup>initial residual energy $r e$ ut<sup>il</sup>is randomly generated in $[ 0 . 4 b , b ] .$ 6g<sup>in</sup><sub>r</sub>g<sup>in</sup> <sub>r</sub>g<sup>in</sup>The energy consumption rate ec of each sensor ranges between $[ 1 m J / s , 5 m J / s ]$ 20C. The lifetime threshold $\Upsilon _ { l }$ C<sup>h</sup>is set as 60min. TIDE DCS RO NFS  TIDE DCS RO Charging angle of each directional charger is set as $\varPhi = \pi / 3$ <sup>10</sup> <sup>11</sup>and we set $D \ = \ 4 m$ $\alpha ~ = ~ 4 0$ $\beta ~ = ~ 1 0 0$ <sup>0</sup> <sup>14</sup> <sup>15</sup>, respectively. The wavelength is set as $\lambda ~ = ~ 0 . 3 3 m$ according to the commercial off-the-shelf TX91501 wireless charger produced by Powercast [33] and the transmission power of the charger is 3W . The rated power $P _ { t h }$ is $2 0 m W$ . We set the length of each time slot as $\Delta t = 2 0 s$ . In every time slot, chargers can select whether to change their orientations. The operation period of the network is set as 24 hours. 

![](images/415c98fe8506a5df822c0a173bd3ba4c99dbb25d61389288555a8f9680676f94.jpg)



(a) $N$ vs. charging utility


![](images/c6a93f7dd7f88d4e580a865674ae782ec2a5d3571e37311a2d490e33ef20bb64.jpg)



(b) $P _ { 0 }$ vs. charging utility


![](images/f562ff6748253f9c9464c271786b8a040cb6eba2a1a7f09a2d900f6e1979081f.jpg)



(c) b vs. charging utility


![](images/360318c6db63b41a5b711c0ff5770e4e6ce5e39c9703938ebaeba514488e3776.jpg)



(d) $\Upsilon _ { l }$ vs. charging utility



Fig. 6. Performance comparisons.


To evaluate the performance of our TIDE scheme, we compare it with the following three charging algorithms. Directional Charger Scheduling (DCS) [19] selects the charging orientations according to dominant sensor sets. It ignores the wave interference and instead assumes the power is additive when multiple waves encounter. Randomized Orientations (RO) generates the orientations randomly according to the candidate sensor sets. Nearest Facing Sensor (NFS) always selects the nearest charger to face request sensor. Both the RO and NFS take wave interference into consideration. 

## B. Performance Comparisons

Impact of number of sensors N. As shown in Fig. 6(a), we vary N from 10 to 15, and the charging utilities of four algorithms show an increasing trend. When more sensors are placed, more charging requests will be launched. Our TIDE scheme can fulfill the most requests by providing the optimal power distribution for each request sensor. Overall, TIDE outperforms DCS, RO and NFS by 52.87%, 126.29%, and 236.70% in terms of N, respectively. 

Impact of charging power $P _ { 0 } .$ As shown in Fig. 6(b), when charging power increases, none of the other three algorithms can maintain a continuous upward trend. This is because these algorithms lack the capability to dynamically control power distribution, which results in sensors located where constructive interference occurs receiving power that may exceed $P _ { t h }$ and therefore unable to gain additional utility with higher charging power. Similarly, sensors located where destructive interference occurs may experience stronger interference effects and consequently generate less utility. Overall, TIDE outperforms DCS, RO and NFS by 94.81%, 182.63%, and 302.81% in terms of $P _ { 0 } .$ , respectively. 


TABLE I



<sub>i</sub>nCHARGING UTILITY FOR FOUR ALGORITHMS.


<table><tr><td></td><td>TIDE</td><td>DCS</td><td>RO</td><td>NFS</td></tr><tr><td>charging utility</td><td>13</td><td>9.470</td><td>5.017</td><td>4.958</td></tr></table>

Impact of battery capacity b. As shown in Fig. 6(c), all these four algorithms show a decreased trend with a larger $b ,$ while our TIDE always maintains the best performance. As the battery capacity increases, the lifetime of sensors will increase, making it more difficult to reach the lifetime threshold for sending charging requests, resulting in a decrease in the number of requests and a decrease in charging utility. Overall, TIDE outperforms DCS, RO and NFS by 38.88%, 105.61%, and 244.68% in terms of b, respectively. 

Impact of lifetime threshold $\Upsilon _ { l } .$ As shown in Fig. 6(d), the overall charging utility yielded by four algorithms shows an increasing trend with $\Upsilon _ { l }$ . This phenomenon suggests that the increment of the charging threshold will decrease the urgency of charging requests, thereby enabling our TIDE to meet more charging requests and produce more utility. Overall, TIDE outperforms DCS, RO and NFS by 40.54%, 97.58%, and 185.78% in terms of $\Upsilon _ { l } ,$ respectively. 

## V. FIELD EXPERIMENTS

In this section, we conduct field experiments to verify the performance of our TIDE scheme. 

## A. Testbed

As shown in Fig. 7, we use three TX91501 Powercast [33] transmitters and ten rechargeable sensors to conduct the testbed experiment. We also employ an AP connecting to a laptop to record the sensing data. The chargers and sensors are deployed in a 3m × 3m square area as shown in Fig. 8. We set $D = 4 m$ $P _ { t h } ~ = ~ 2 0 m W$ $\Delta t = 2 0 s$ , respectively. The operation period of the network is set as 12 hours. Each time charger needs to alter the charging orientation, we rotate the chargers to cover the selected candidate sensor sets. 

## B. Experimental Results

Table I shows the overall charging utility of the four algorithms, we can see our TIDE outperforms DCS, RO, NFS by 37.3%, 159.1% and 162.2%, respectively. This indicates our 

![](images/b8f863537376eadc399303c50845b81c229efbc79f06c117e2402b4a292ec25c.jpg)



Fig. 7. Testbed.


![](images/da7cfce6e4799e93cc13ae81e4a50eca8f8a66bb162927d848fc8f30039e672d.jpg)



Fig. 8. Field experiment.


TIDE better fulfills real-time charging requests from sensors by controlling power distribution dynamically. 

To verify the feasibility and robustness of TIDE, we conduct the experiments for different numbers of sensors as shown in Fig. 9(a). As the number of sensors increases, our algorithm provides considerable power to sensors due to its ability to dynamically control power distribution, thereby generating more charging utility. Then, we compare four algorithms as the lifetime threshold varies from 20min to 80min. As shown in Fig. 9(b), with the higher lifetime threshold, the utilities follow similar trends with those in the simulations. In conclusion, we claim that our TIDE is robust to different networking settings and it is feasible in real scenarios. 

## VI. RELATED WORK

In general, the existing work can be classified into two categories: stationary charging [17], [18], [26] and mobile charging [22]–[24]. 

In stationary charging, static chargers are deployed at fixed locations and assigned to emit energy continuously. Yu et al. [17] focused on the connectivity of chargers for communication needs. They designed effective algorithms with guaranteed approximation ratios to select the chargers’ positions and orientations. Dai et al. [18] studied the problem of wireless charger placement with multiple directional antennas. They aimed to maximize the overall charging utility by determining the chargers’ positions and the antennas’ orientations. Ma et al. [26] considered the wave interference in the concurrent charging scenario and they proposed a concurrent charging scheme to take full advantage of the high power caused by constructive interference to enhance the charging efficiency. 

In mobile charging, mobile chargers (MCs) can move and replenish energy for sensors in proximity due to their mobility. 

![](images/0d7f26303f9efcb249303378edd5b54c269a417162edcfab5578ae5c04b25576.jpg)



(a) N vs. charging utility


![](images/8461af8dcdd9407c27f6a5c1befda845d51045a834b61346e40b21d74830da40.jpg)



(b) $\Upsilon _ { l }$ vs. charging utility



<sup>15</sup>Fig. 9. Performance comparisons on test-bed experiments.


<sub>n</sub>g Yang et al. [22] concentrated on the defects of the inaccu-5ha<sup>r</sup>rate discretization methods and imprecise charging models. They addressed the precise charging issue in complicated 0environments by designing new discretization schemes and <sup>Number</sup> <sup>of</sup> <sup>Sensors</sup>building a reflection model. Ren et al. [23] exploited the neglected back lobe for mobile charging to simultaneously minimize the number of dead sensors and maximize energy usage efficiency. Sun et al. [24] focused on serving dynamic nodes whose locations vary randomly and pointed out longshort-term conflict of dynamic sensors. They developed an online learning algorithm to iteratively adjust the charging strategy to maximize charging utility. 

Nevertheless, little attention has been paid to the gap between energy supply and practical demand caused by fixed power distribution. It is an important issue to control the power distribution dynamically to serve sensors better. 

## VII. CONCLUSIONS

This paper focuses on the inflexibility of fixed power distribution in static charging networks. Our main contribution is to design a dynamic power distribution controlling scheme to respond to real-time charging requests from sensors to maximize the overall charging utility. Firstly, we incorporate wave interference into the directional charging model to depict the power distribution in a directional charging network. To overcome the challenge of selecting a specific orientation for each charger, we design a candidate sensor sets extraction algorithm to reduce the computation complexity and the negative impact of destructive interference. Furthermore, we propose a neighbor set division method to narrow the calculation scope and develop a dynamic power distribution controlling algorithm to update the neighbor sets timely and select optimal orientations for chargers. Finally, extensive simulations and test-bed experiments are conducted to prove our TIDE outper forms other comparison algorithms by 142.62% on average. 

## ACKNOWLEDGMENT

This work is partially supported by the National Natural Science Foundation of China (62072320, 62002250, 62272328), the Natural Science Foundation of Sichuan Province (2022NS-FSC0569, 2022NSFSC0929), the Key R&D Program of Sichuan Province (No. 22ZDZX0021), Humanities and Social Sciences Project of the Ministry of Education of China (No. 23YJA630114). 



[1] A. Kurs, A. Karalis, R. Moffatt, J. D. Joannopoulos, P. Fisher, and M. Soljacic, “Wireless power transfer via strongly coupled magnetic resonances,” Science, vol. 317, no. 5834, pp. 83–86, 2007. 





[2] Y. Yang and C. Wang, Wireless rechargeable sensor networks. Springer, 2015. 





[3] W. Zhou, H. Zhou, X. Cui, X. Wang, X. Wang, and Z. Liu, “Roland: Robust in-band parallel communication for magnetic MIMO wireless power transfer system,” in IEEE INFOCOM, 2023, pp. 1–10. 





[4] S. He, K. Hu, S. Li, L. Fu, C. Gu, and J. Chen, “A robust RF-based wireless charging system for dockless bike-sharing,” IEEE Transactions on Mobile Computing, 2023. 





[5] P. Zhou, C. Wang, and Y. Yang, “Design of self-sustainable wireless sensor networks with energy harvesting and wireless charging,” ACM Transactions on Sensor Networks (TOSN), vol. 17, no. 4, pp. 1–38, 2021. 





[6] C. Lin, S. Hao, H. Dai, W. Yang, L. Wang, G. Wu, and Q. Zhang, “Maximizing charging efficiency with fresnel zones,” IEEE Transactions on Mobile Computing, 2022. 





[7] T. Liu, B. Wu, S. Zhang, J. Peng, and W. Xu, “An effective multi-node charging scheme for wireless rechargeable sensor networks,” in IEEE INFOCOM, 2020, pp. 2026–2035. 





[8] T. Liu, B. Wu, W. Xu, X. Cao, J. Peng, and H. Wu, “RLC: a reinforcement learning-based charging algorithm for mobile devices,” ACM Transactions on Sensor Networks, vol. 17, no. 4, pp. 1–23, 2021. 





[9] H. Dai, X. Wang, X. Lin, R. Gu, S. Shi, Y. Liu, W. Dou, and G. Chen, “Placing wireless chargers with limited mobility,” IEEE Transactions on Mobile Computing, vol. 22, no. 6, pp. 3589–3603, 2023. 





[10] T. Wu, P. Yang, and H. Dai, “Charging on the move: Scheduling static chargers with tunable power for mobile devices,” in IEEE/ACM IWQOS, 2021, pp. 1–10. 





[11] Y. Sun, C. Lin, H. Dai, P. Wang, L. Wang, G. Wu, and Q. Zhang, “Trading off charging and sensing for stochastic events monitoring in WRSNs,” IEEE/ACM Transactions on Networking, vol. 30, no. 2, pp. 557–571, 2022. 





[12] W. Xu, W. Liang, H. Kan, Y. Xu, and X. Zhang, “Minimizing the longest charge delay of multiple mobile chargers for wireless rechargeable sensor networks by charging multiple sensors simultaneously,” in IEEE ICDCS, 2019, pp. 881–890. 





[13] Y. Liang, M. Yin, Y. Zhang, W. Wang, W. Jia, and T. Wang, “Grouping reduces energy cost in directionally rechargeable wireless vehicular and sensor networks,” IEEE Transactions on Vehicular Technology, vol. 72, no. 8, pp. 10 840–10 851, 2023. 





[14] H. Dai, Y. Zhang, X. Wang, A. X. Liu, and G. Chen, “Omnidirectional chargability with directional antennas,” IEEE Transactions on Mobile Computing, 2023. 





[15] H. Dai, X. Wang, A. X. Liu, H. Ma, G. Chen, and W. Dou, “Wireless charger placement for directional charging,” IEEE/ACM Transactions on Networking, vol. 26, no. 4, pp. 1865–1878, 2018. 





[16] X. Wang, H. Dai, W. Wang, J. Zheng, N. Yu, G. Chen, W. Dou, and X. Wu, “Practical heterogeneous wireless charger placement with obstacles,” IEEE Transactions on Mobile Computing, vol. 19, no. 8, pp. 1910–1927, 2020. 





[17] N. Yu, H. Dai, G. Chen, A. X. Liu, B. Tian, and T. He, “Connectivityconstrained placement of wireless chargers,” IEEE Transactions on Mobile Computing, vol. 20, no. 3, pp. 909–927, 2021. 





[18] H. Dai, Y. Zhao, W. Wang, R. Gu, Y. Qu, C. Lin, L. Xu, and W. Dou, “Placing wireless chargers with multiple antennas,” in IEEE SECON, 2022, pp. 479–487. 





[19] H. Dai, K. Sun, A. X. Liu, L. Zhang, J. Zheng, and G. Chen, “Charging task scheduling for directional wireless charger networks,” IEEE Transactions on Mobile Computing, vol. 20, no. 11, pp. 3163– 3180, 2021. 





[20] X. Wang, H. Dai, H. Huang, Y. Liu, G. Chen, and W. Dou, “Robust scheduling for wireless charger networks,” in IEEE INFOCOM, 2019, pp. 2323–2331. 





[21] J. Zhang, H. Gao, Q. Chen, and J. Li, “Task-oriented energy scheduling in wireless rechargeable sensor networks.” ACM Transactions on Sensor Networks, vol. 19, no. 4, pp. 1–32, 2023. 





[22] W. Yang, C. Lin, H. Dai, J. Ren, P. Wang, L. Wang, G. Wu, and Q. Zhang, “Precise wireless charging in complicated environments,” in IEEE ICDCS, 2022, pp. 765–775. 





[23] M. Ren, D. Wu, J. Xue, W. Xu, J. Peng, and T. Liu, “Utilizing the neglected back lobe for mobile charging,” in IEEE INFOCOM, 2023, pp. 1–10. 





[24] Y. Sun, C. Lin, W. Yang, J. Ren, L. Wang, G. Wu, and Q. Zhang, “Charging dynamic sensors through online learning,” in IEEE INFO-COM, 2023, pp. 1–10. 





[25] D. K. Cheng et al., Field and wave electromagnetics. Pearson Education India, 1989. 





[26] Y. Ma, D. Wu, M. Ren, J. Peng, J. Yang, and T. Liu, “Concurrent charging with wave interference,” in IEEE INFOCOM, 2023, pp. 1–10. 





[27] Y. Liu, Y. He, M. Li, J. Wang, K. Liu, and X. Li, “Does wireless sensor network scale? a measurement study on greenorbs,” IEEE Transactions on Parallel and Distributed Systems, vol. 24, no. 10, pp. 1983–1993, 2012. 





[28] H. Kellerer, U. Pferschy, and D. Pisinger, “Knapsack problems, 2004,” 2003. 





[29] [Online]. Available: https://www.shotoku.co.uk/products/smartped 





[30] L. Deng, W. Xu, W. Liang, J. Peng, Y. Zhou, L. Duan, and S. K. Das, “Approximation algorithms for the min-max cycle cover problem with neighborhoods,” IEEE/ACM Transactions on Networking, vol. 28, no. 4, pp. 1845–1858, 2020. 





[31] W. Xu, W. Liang, X. Lin, G. Mao, and X. Ren, “Towards perpetual sensor networks via deploying multiple mobile wireless chargers,” in IEEE ICPP, 2014, pp. 80–89. 





[32] S. He, J. Chen, F. Jiang, D. K. Yau, G. Xing, and Y. Sun, “Energy provisioning in wireless rechargeable sensor networks,” IEEE Transactions on Mobile Computing, vol. 12, no. 10, pp. 1931–1942, 2012. 





[33] [Online]. Available: http://www.powercastco.com. 

