# Concurrent Charging with Wave Interference

Yuzhuo Ma<sup>∗†</sup>, Dié Wu<sup>∗†</sup>, Meixuan Ren<sup>∗†</sup>, Jian Peng<sup>‡</sup>, Jilin Yang<sup>∗†</sup>, Tang Liu<sup>∗†</sup> 

<sup>∗</sup>College of Computer Science, Sichuan Normal University, Chengdu, Sichuan 610101, China 

<sup>†</sup>Visual computing and virtual reality Key Lab, Sichuan Normal University, Chengdu, Sichuan 610068, China 

<sup>‡</sup>College of Computer Science, Sichuan University, Chengdu, Sichuan 610065, China 

Email: {yuzhuoma, meixuanren}@stu.sicnu.edu.cn, {wd, jilinyang, liutang}@sicnu.edu.cn, jianpeng@scu.edu.cn 

Abstract—To improve the charging performance, employing multiple wireless chargers to charge sensors concurrently is an effective way. In such charging scenarios, the radio waves radiated from multiple chargers will interfere with each other. Though a few work have realized the wave interference, they do not fully utilize the high power caused by constructive interference while avoiding the negative impacts brought by the destructive interference. In this paper, we aim to investigate the power distribution regularity of concurrent charging and take full advantage of the high power to enhance the charging efficiency. Specifically, we formulate a concurrent charGing utility mAxImizatioN (GAIN) problem and build a practical charging model with wave interference. Further, we propose a concurrent charging scheme, which not only can improve the power of interference enhanced regions by deploying chargers, but also find a set of points with the highest power to locate sensors. Finally, we conduct both simulations and field experiments to evaluate the proposed scheme. The results demonstrate that our scheme outperforms the comparison algorithms by 40.48% on average. 

Index Terms—wave interference, concurrent charging, charger placement, sensor deployment, wireless power transfer 

## I. INTRODUCTION

Benefiting from the breakthrough of Wireless Power Transfer (WPT) technology [1], the Wireless Rechargeable Sensor Network (WRSN) has become a promising platform for wide applications, including precision agriculture, ecological environment monitoring, military fields, etc [2]–[6]. In such largescale scenarios, deploying a large number of chargers to enable sensors to harvest energy from multiple sources is an effective way to improve charging efficiency [7]–[17]. Apparently, this charging paradigm means a significant increase in charger density, introducing numerous overlaps of charger coverage. 

Sensors located within such overlaps will be charged by multiple chargers concurrently (called as concurrent charging in this paper). According to the wave interference and poweramplitude relationship [18], the combined power at any point in overlaps depends on the amplitude and phase of the arriving waves. The phase difference among the waves is determined by the distance difference, and the combined amplitude of multiple waves equals the vector sum of the amplitudes of individual waves. Specifically, when the waves are in phase (i.e., the crests of waves encounter), constructive interference occurs. At this time, the combined power is more significant than the sum of all waves’ power. On the contrary, when the waves are in anti-phase (i.e., a crest meets a trough), destructive interference occurs. Then they cancel and the combined power may be less than any of the waves’ power or even close to zero. 

![](images/473466ceb0ab0d2747538112ab5dc9235bf5bd33e2fac7ac1b87187353427c19.jpg)



Fig. 1. A simulation of how power is distributed between two chargers with frequency of 915MHz


Fig. 1 shows the power distribution between two chargers at a distance of 100cm. We can see the distribution of combined power (i.e., the orange line) shows fluctuation, meanwhile the adjacent crest and trough present a significant difference in power. This indicates that the wave interference has a notable impact on the energy eventually received by sensors. 

Although much effort has been devoted to constructing WRSN with multiple chargers, most of them ignore the wave interference and instead assume the charging power is additive from different chargers [7]–[14]. By plotting the additive power of the two waves (i.e., the blue line) in Fig. 1, we can see that there is a great difference between additive power and combined power, especially at the crests and troughs. On the other hand, a few researchers have realized the interference effect, but they just achieved to avoid the negative influences of destructive interference by scheduling (i.e., turn on/off) the chargers [16], [17]. Thus we can conclude that if we simply combine existing arts, it is impossible to significantly improve the charging performance by using constructive interference and avoiding destructive interference simultaneously. 

In this paper, we consider a practical concurrent charging scenario, in which each sensor has a specific deployable range around each Point of Interest (PoI). We aim to take full advantage of the nonlinear effect of the wave interference, to enable sensors to harvest considerable combined power from multiple chargers. Thus, we state our concurrent charGing utility mAxImizatioN (GAIN) problem as follows. Given a fixed number of chargers, a set of PoIs and sensors, how to design a concurrent charging scheme to maximize the overall charging utility for all sensors. In particular, our objective can be divided into two folds: (i) how to deploy the chargers so that within the deployable ranges of the sensors, the constructive interference can provide as high as possible power for sensors. (ii) how to find the highest-power point for each sensor in limited deployable ranges. 

![](images/05c34fbe37dc63b8164a0c87a9e104c1830a227b9f40d16aa61b45e5cac4d116.jpg)



Fig. 2. A simulation of how power is distributed when 5 chargers with frequency of 915MHz are placed on a 10m × 10m 2D plane.


Generally, there are two main challenges in our problem. 

The first challenge is selecting placement positions for a limited number of chargers, which is exactly the traditional NP-hard partial disk coverage problem [19]. Besides, we also need to promote the combined power within the sensor deployable ranges, which further increases the difficulty of charger placement. 

The second challenge is that finding the highest-power point in each sensor deployable range is difficult under complicated energy distribution. The complexity results from (i) the charging power is nonlinear with distance; (ii) the interference effect from multiple waves is nonlinear too. Moreover, though the sensor deployable range is limited, the available locations are still continuous values leading to infinite candidate options, further raising computational complexity. 

Fig. 2 shows the complicated energy distribution in real charging scenarios. We can see that the whole network appears to be alternating bright (i.e., interference enhanced) and dark (i.e., interference weakened) regions with different shapes and sizes, even around chargers. Moreover, there is a great power difference between different positions in the network. For example, for two adjacent points $a$ and $b ,$ their power is 0.56mW and 19.73mW, respectively, the difference between them reaches 35.23 times. And for the points c and $d ,$ their energy difference is also 1.92 times, even though they are both located in the enhanced regions. 

To address the GAIN problem, for the first challenge, we develop a charger placement algorithm to maximize the overall additive power of the waves arriving at the centers of all sensor deployable ranges. The rationale behind is that only when the power of the waves involved in interference is high, the power of the interference enhanced regions can be high enough. For the second one, we investigate this complicated power distribution by proposing a practical charging model with wave interference. Through this model, we explore the power distribution regularity caused by the nonlinear interference of multiple chargers. Then, to tackle the problem of continuous search space, we divide each sensor deployable range into several subareas by the number of interference enhanced regions, reducing the number of candidate sensor locations from infinite to finite. Based on this, we develop a sensor deployment algorithm to find the optimal deployment location of each sensor in this limited number of options. 

The main contributions of this work are summarized below. 

• To the best of our knowledge, we are the first to fully utilize the high power caused by the wave interference to promote charging efficiency. We build a practical charging model with the wave interference to investigate how nonlinear interference impacts the energy distribution. Further, we explore the distribution regularity of interference enhanced regions. The evaluation results show that our charging model is accurate, andhence it can be applied into other charging scenarios. 

• To maximize the overall charging utility of all sensors, we develop a concurrent charging scheme. Specifically, we propose a charger placement algorithm to enable the high-power interference enhanced regions to appear close to PoIs. We then design a sensor deployment algorithm to locate each sensor to the highest-power point within the deployable range. 

• We conduct extensive simulations and field experiments to verify the proposed scheme. Results show that our scheme outperforms other comparison algorithms by 40.48% on average in charging utility. 

## II. PRELIMINARIES

## A. Network Model

Consider there are N PoIs denoted as $O = \{ o _ { 1 } , o _ { 2 } , . . . , o _ { N } \}$ on a 2D plane Ω. Each PoI has a sensor deployable disk (SDD) centered at itself with radius of r to deploy a sensor. All omnidirectional sensors are denoted by ${ \cal S } = \{ s _ { 1 } , s _ { 2 } , . . . , s _ { N } \}$ If no confusion arises, we still use $s _ { i }$ to denote the location of sensor $s _ { i }$ 

A given number of omnidirectional wireless chargers $C =$ $\left\{ c _ { 1 } , c _ { 2 } , . . . , c _ { M } \right\}$ are employed to provide charging service for sensors, we still use $c _ { j }$ to represent the placement position of $c _ { j }$ . When both chargers and sensors are deployed, each sensor $s _ { i }$ will be concurrently charged by a subset of the chargers, denoted by $C _ { i } ( C _ { i } \subseteq C )$ . The number of the chargers in $C _ { i }$ is denoted by $m _ { i } ( m _ { i } \leq M )$ . Moreover, we use $d _ { i j }$ and $P _ { s _ { i } | C _ { i } }$ to represent the Euclidean distance between sensor $s _ { i }$ and charger $c _ { j }$ and the combined power arrived at $s _ { i }$ from $C _ { i }$ , respectively. 

## B. Charging Model

To mathematically explain the complicated power distribution, a practical charging model with the wave interference needs to be established. First, we present the radio wave radiated by the charger $c _ { j }$ as: 

$$
A (t) = A _ {0} \cos 2 \pi f t,\tag{1}
$$

where $A _ { 0 } , f$ are amplitude and frequency of this wave, respectively. Since the amplitude of the radio emitted by $c _ { j }$ decreases with the distance, the wave arrived at $s _ { i }$ can be written as: 

$$
A (t) = \frac {A _ {0}}{\hat {d _ {i j}}} \cos (2 \pi f t - \frac {2 \pi}{\lambda} d _ {i j}).\tag{2}
$$

In this equation, $\begin{array} { r } { \hat { d _ { i j } } \ = \ \frac { d _ { i j } + \beta } { \sqrt { \alpha } } } \end{array}$ is the attenuation factor for wave propagation due to the empirical model in [20], the $\begin{array} { r } { \alpha = \frac { \hat { G } _ { s } \hat { G _ { r } } } { L _ { p } } ( \frac { \Breve { \lambda } } { 4 \pi } ) ^ { 2 } } \end{array}$ , where $G _ { s } , \ G _ { r }$ are charger and sensor antenna gain, respectively, and λ is the wavelength. $\beta$ is a parameter to adjust the Friis’ free space equation for short distance transmission. 

Thus, when $s _ { i }$ is covered by a single charger $c _ { j } ,$ the wave received by $s _ { i }$ satisfies the Eq. (2). Moreover, its average power can be expressed as: 

$$
\begin{array}{r} p (s _ {i}, c _ {j}) = \frac {1}{T} \int_ {- \frac {T}{2}} ^ {\frac {T}{2}} [ A (t) ] ^ {2} d t \\ = \frac {1}{T} \int_ {- \frac {T}{2}} ^ {\frac {T}{2}} \left[ \frac {A _ {0}}{\hat {d _ {i j}}} \cos (2 \pi f t - \frac {2 \pi}{\lambda} d _ {i j}) \right] ^ {2} d t = \frac {A _ {0} ^ {2}}{2 \hat {d _ {i j}} ^ {2}}, \end{array}\tag{3}
$$

where $T$ is the period of the radio wave. Since the power quadratically decreases with the charging distance, we denote by D the farthest charging distance, i.e., if the distance between charger and sensor is greater than $D ,$ the arriving power can neither enable the sensor to receive non-negligible energy, nor make an obvious effect on interference. 

When $m _ { i }$ chargers concurrently charge $s _ { i }$ , the combined wave arrived at $s _ { i }$ can be written as: 

$$
A (t) = \sum_ {j = 1} ^ {m _ {i}} \frac {A _ {0}}{\hat {d _ {i j}}} \cos (2 \pi f t - \frac {2 \pi}{\lambda} d _ {i j}),\tag{4}
$$

where $\begin{array} { r } { A _ { 0 } = \left\lceil m _ { i } A _ { 0 } ^ { 2 } + 2 A _ { 0 } ^ { 2 } \sum _ { j > k } ^ { m _ { i } } \sum _ { k = 1 } ^ { m _ { i } } \cos ( 2 \pi \frac { d _ { i j } - d _ { i k } } { \lambda } ) \right\rceil ^ { \frac { 1 } { 2 } } } \end{array}$ 

Thus, the average power of the combined wave at $s _ { i }$ is: 

$$
\begin{array}{l} P _ {s _ {i} | C _ {i}} = \sum_ {c _ {j} \in C _ {i}} p (s _ {i}, c _ {j}) = \frac {1}{T} \int_ {- \frac {T}{2}} ^ {\frac {T}{2}} [ A (t) ] ^ {2} d t \\ \qquad = \frac {1}{T} \int_ {- \frac {T}{2}} ^ {\frac {T}{2}} \left[ \sum_ {j = 1} ^ {m _ {i}} \frac {A _ {0}}{\hat {d _ {i j}}} \cos (2 \pi f t - \frac {2 \pi}{\lambda} d _ {i j}) \right] ^ {2} d t \\ \qquad = \frac {A _ {0} ^ {2}}{2} \left(\sum_ {j = 1} ^ {m _ {i}} \frac {1}{\hat {d _ {i j}} ^ {2}} + \sum_ {j > k} ^ {m _ {i}} \sum_ {k = 1} ^ {m _ {i}} \frac {2 \cos (2 \pi \frac {d _ {i j} - d _ {i k}}{\lambda})}{\hat {d _ {i j}} \hat {d _ {i k}}}\right). \end{array}\tag{5}
$$

From Eq. (5), we can see the complexity behind the power distribution: for the charger set $C _ { i }$ which concurrently charges $s _ { i } ,$ each wave radiated from each charger interferes with the other $m _ { i } \textrm { -- } 1$ waves. Specifically, if these $m _ { i }$ waves constructively interfere at $s _ { i } ,$ , the distance difference between any two chargers and $s _ { i }$ is kλ $( k \in N )$ and the combined power $P _ { s _ { i } | C _ { i } }$ at $s _ { i }$ will be significantly greater than the additive power of waves; on the contrary, if these $m _ { i }$ waves destructively interfere at $s _ { i } ,$ the distance difference between any two chargers and $s _ { i }$ is $k + { \textstyle { \frac { 1 } { 2 } } } \lambda ( k \in { \cal N } )$ and $P _ { s _ { i } | C _ { i } }$ will be weakened, even as low as 0. Generally, the model in Eq. (5) is totally consistent with our observations from Fig. 2. 

## C. Charging Utility Model

In practice, rechargeable sensors typically have a rated power Pth $P _ { t h }$ constrained by electric circuits. Accordingly, we present the charging utility for a single sensor $s _ { i }$ with the received power $P _ { s _ { i } | C _ { i } }$ is given by 

$$
u (P _ {s _ {i} | C _ {i}}) = \left\{ \begin{array}{l l} \frac {1}{P _ {t h}} \cdot P _ {s _ {i} | C _ {i}}, & P _ {s _ {i} | C _ {i}} \leq P _ {t h}, \\ 1, & P _ {s _ {i} | C _ {i}} > P _ {t h}. \end{array} \right.\tag{6}
$$

In this model, the normalized charging utility is first proportional to the received power, and then becomes constant when the received power is larger than the threshold $P _ { t h }$ 

## D. Problem Formulation

In this work, our goal is to design a concurrent charging scheme to maximize the overall charging utility for all sensors by utilizing the high power caused by the wave interference. Formally, we define the concurrent charGing utility mAxImizatioN (GAIN) problem as follows: 

$$
\begin{array}{l l} \text {(P1)} & \max \quad U (t o t a l) = \sum_ {i = 1} ^ {N} u (P _ {s _ {i} | C _ {i}}), \\ & s. t. \quad c _ {j}, s _ {i} \in \Omega , | | o _ {i} s _ {i} | | \leq r. \end{array}\tag{7}
$$

## Theorem 1. The GAIN problem P1 is NP-hard.

Proof: we omit the proof due to space limitations. 

## III. SOLUTION FOR THE CHARGER PLACEMENT

In this section, we aim to propose a charger placement algorithm to maximize the overall additive power arrived at all PoIs, whose resulting power distribution can be used as a reference for deploying sensors. The rationale behind is that only when the power of waves involved in interference is high, can the power of combined waves be high enough. 

## A. Extract Maximal Covering Sets and Corresponding Candidate Charger Placement Areas

In order to cover all PoIs by using a limited number of chargers, in this subsection, we first define the Maximal Covering Set (MCS) to indicate the representative sets of PoIs, and their corresponding candidate charger placement areas. Instead of enumerating all positions on the plane, our objective here is to obtain finite candidate charger placement areas from the plane by extracting MCSs. 

Generally, due to geometric symmetry, if a charger $c _ { j }$ is located within a circle centered at $o _ { i }$ with radius D, which we call the charger placeable circle of $o _ { i }$ , the PoI $o _ { i }$ is also located within the circle centered at $c _ { j }$ with radius D. Thus, when $c _ { j }$ is located within the overlap of multiple charger placeable circles, corresponding PoIs can be concurrently covered. 

Based on the relationship between position of each charger and the PoIs it covers, we have the following definitions: 

![](images/fe9dbbdfbbbc19e293d4a7b24c6137bad1240f4ef8947cdbb39a07229c3def5b.jpg)


![](images/7febfa247e1dd2c807c089e73d339d25eb939198b7834193ccfaf1adec524c23.jpg)



Fig. 3. The construction of Maximal Covering Sets (MCSs).



Fig. 4. Candidate areas discretization.


Algorithm 1 Extraction of MCSs and candidate subareas

Input: The set of PoIs O, the farthest charging distance D, the error threshold $\epsilon$ , and the constant $\beta$ Output: All MCSs and the candidate subarea set $\Gamma$ 1: for each PoI $o_{i} \in O$ do

2: Draw a circle centered at $o_{i}$ with radius D;

3: end for

4: for each area divided by the circles do

5: Calculate the corresponding covered PoI set;

6: Add the covered PoI set into the set of candidate MCS;

7: end for

8: Identify all MCSs and the corresponding candidate charger placement areas from the set of candidate MCS;

9: Calculate the number of segments Q and draw Q concentric circles centered at each PoI;

10: for each candidate charger placement area do

11: Obtain all candidate subareas and add them into the candidate subareas set $\Gamma$ ;

12: end for

13: Return MCSs and the candidate subareas set $\Gamma$ ; 

Definition 1. Maximal Covering Set: given a set of PoIs $O _ { i }$ that is covered by a charger located at $c _ { i } ,$ , if there does not exist a $c _ { j }$ when charger locates at such that $O _ { j } \supset O _ { i }$ , then $O _ { i }$ is called Maximal Covering Set (MCS). 

Definition 2. Candidate Charger Placement Area: given an MCS, if there is an area, no matter where a charger is placed in it, all PoIs in the MCS can be covered by this charger, then this area is called the corresponding candidate charger placement area of the MCS. 

As placing chargers at candidate charger placement areas of MCSs is always better than placing them at the corresponding areas of its subsets, we focus on how to extract all MCSs as well as their corresponding candidate charger placement areas. The extracting process is detailed in lines 1-8 in Algorithm 1. 

Fig. 3 depicts an example for three PoIs $o _ { 1 } , o _ { 2 } ,$ and $o _ { 3 } ,$ , and the overlap area is divided into 4 subareas, $A , B , C ,$ and $\gamma .$ Obviously, when a charger locates anywhere in γ can it covers $\left\{ o _ { 1 } , o _ { 2 } , o _ { 3 } \right\}$ . Thus, the MCS in Fig. 3 is $\left\{ o _ { 1 } , o _ { 2 } , o _ { 3 } \right\}$ , and its corresponding candidate charger placement area is $\gamma .$ 

## B. Discretizing for the Candidate Charger Placement Areas

Note that candidate charger placement areas are continuous, there are infinite available locations for placing each charger. To reduce the infinite solution space to a limited one without performance loss, in this subsection, we discrete each candidate charger placement area to a limited number of candidate subareas, by using a piecewise constant function $\widetilde { p } ( d )$ to approximate the nonlinear charging power. 

Algorithm 2 Charger placement algorithm

Input: The number of chargers M, all candidate MCSs and their corresponding candidate positions set $\Gamma$ , the set of PoIs O, and the objective function $P_{addit}(C)$ Output: Charger placement set C

1: $C = \emptyset$ 2: while $|C| \leq M$ do

3: $c^{*} \leftarrow \arg\max_{c \in \Gamma \setminus C}(P_{addit}(C \cup \{c\}) - P_{addit}(C))$ ;

4: $C = C \cup \{c^{*}\}$ ;

5: end while 

Theorem 2. Define the piecewise constant function $\widetilde { p } ( d )$ as 

$$
\widetilde {p} (d) = \left\{ \begin{array}{l l} p (l (1)), & d = l (0), \\ p (l (q)), & l (q - 1) <   d \leq l (q) (q = 1, 2,..., Q - 1), \\ 0, & d > l (Q), \end{array} \right.
$$

where $l ( 0 ) = 0 , \ l ( Q ) = D ,$ and $l ( q ) ~ = ~ \beta ( ( 1 + \epsilon ) ^ { q / 2 } ~ -$ $1 ) , ( q = 1 , 2 , . . . , Q - 1 )$ (therefore $\begin{array} { r } { Q = \frac { l n [ ( D \dot { + } \beta ) ^ { 2 } / \beta ^ { 2 } ] } { l n ( 1 + \epsilon ) } ) , } \end{array}$ the approximation error is subject to 

$$
1 \leq \frac {p (d)}{\widetilde {p} (d)} \leq 1 + \epsilon , d \leq D,
$$

where  is a predetermined error threshold. 

According to the predetermined approximation error threshold $\epsilon , Q$ concentric circles with increasing radius l(1), l(2), $l ( Q )$ centered at each PoI can be drawn. Apparently, a charger placed at any point between two adjacent circles with radius $l ( q )$ and $l ( q - 1 )$ provides the same power $p ( l ( q ) )$ with a uniform approximation ratio. 

Fig. 4 shows an instance of a candidate charger placement area divided into 4 candidate subareas, γ<sub>1</sub>, γ<sub>2</sub>, γ<sub>3</sub>, and $\gamma _ { 4 } ,$ by drawing concentric circles centered at each PoI $o _ { 1 }$ , o<sub>2</sub>, and $o _ { 3 }$ with radius l(1) and $l ( 2 )$ , respectively. If a charger is placed at any point in the same subarea, the power it provides is approximately the same. Therefore, by selecting a point randomly in each subarea, we can obtain a set of candidate placement positions also be denoted as Γ by abuse of notation. The details of the discretizing for candidate charger placement areas is described in lines 9-13 in Algorithm 1. 

Theoretically, we have the following theorem for area discretization: 

Theorem 3. Let $\widetilde { p } ( o _ { i } , c _ { j } )$ be the approximated charging power arriving PoI $o _ { i } ,$ we have the approximation error as: 

$$
1 \leq \frac {p (o _ {i} , c _ {j})}{\widetilde {p} (o _ {i} , c _ {j})} \leq 1 + \epsilon .
$$

Proof: we omit the proof due to space limitations. 

## C. Selection for Charger Placement Positions

In this subsection, we elaborate on how to select the charger placement positions from the obtained candidate set Γ such that the overall additive power arrived at all PoIs can be maximized, where $x _ { j }$ is a boolean value that determines whether to select this candidate position $c _ { j }$ to place charger or not. The overall additive power $P _ { a d d i t } ( C )$ arrived at all PoIs can be calculated as: 

$$
P _ {a d d i t} (C) = \sum_ {i = 1} ^ {N} \sum_ {c _ {j} \in \Gamma} x _ {j} \widetilde {p} \left(o _ {i}, c _ {j}\right).\tag{8}
$$

Note that, our ultimate goal is to deploy the sensor to constructive locations for obtaining high power after placing the chargers. Thus, the constraints on maximizing $P _ { a d d i t } ( C )$ is: the power of the combined wave at each PoI is not larger than the sensor’s power threshold $P _ { t h }$ when all individual arriving waves are constructive interference. 

Generally, we have the following lemma and theorem. 

Lemma 1. The function $P _ { a d d i t } ( C )$ is nonnegative, monotone, and submodular. 

Proof: we omit the proof due to space limitations. 

Thus, we use a greedy-based algorithm to greedily select the position that maximizes the marginal gain of the function $P _ { a d d i t } ( C )$ in each iteration. This process will stop if all chargers are deployed. The detailed charger placement algorithm is given in Algorithm 2. 

Theorem 4. The charger placement algorithm achieves an approximation ratio of $1 - 1 / e - \epsilon \epsilon$ 

Proof: we omit the proof due to space limitations. 

## IV. SOLUTION FOR THE SENSOR DEPLOYMENT

In this section, we present our sensor deployment algorithm to maximize the overall charging utility. Note that though the range of SDDs is limited, the solution space is unlimited due to the continuous values of available locations for sensor deployment. To tackle this problem, our basic idea is to find a limited number of interference enhanced regions in SDDs, and select the point with the highest power as the sensor deployment location from these limited regions, so as to reduce the solution space from infinite to finite. Thus, it is essential to clarify the power distribution regularity caused by the nonlinear interference of multiple chargers. 

We start by considering a basic situation of the PoI covered by only two chargers, and explore the method to find the optimal sensor locations. After that, we will further develop a solution for a complex situation, that is the PoI concurrently covered by multiple chargers (more than 2 chargers). 

## A. PoI Covered Concurrently by Two Chargers

First, we give the following theorem to explain the power distribution regularity of the area covered by two chargers. 

![](images/9f7e695b720b0f902226486555c2ced6aa9f3935bb73336ccfec14a824e00e9d.jpg)



Fig. 5. The power distribution in the overlap covered by two chargers.


Theorem 5. The interference enhanced and weakened regions alternate in fringes in the overlap concurrently covered by two chargers. 

Proof: Assume there are two chargers randomly placed on a 2D plane. Note that constructive interference occurs when the path difference of the two waves radiated by chargers is $\begin{array} { r } { k \lambda ( k \in N , 0 \leq k \leq \lfloor \frac { d _ { c _ { 1 } c _ { 2 } } } { \lambda } \rfloor ) } \end{array}$ . Thus, given a k, we can find a hyperbola (i.e., constructive curve) on the plane that satisfies the distance difference between any point on the hyperbola and the two chargers equals kλ. Correspondingly, there is also a hyperbola (i.e., destructive curve) beside. The distance difference between the point on it and the two chargers is $( k + 1 / 2 ) \lambda$ . In the middle of these two adjacent curves, a hyperbola at which the power of combined waves arrival will neither increase nor decrease due to the interference bisects them and the distance difference equals $( k + 1 / 4 ) \lambda$ . Thus, this hyperbola divides the region between adjacent constructive and destructive curves into interference enhanced and weakened regions, respectively. With various k, the overlap is partitioned into multiple interference enhanced and weakened regions alternating in fringes. 

Fig. 5 depicts the power distribution in the overlap covered by two chargers. It can be seen that there are total 9 fringeshaped interference enhanced regions in the overlap, and each region has a constructive curve in middle (solid yellow line). Since the combined power will be significantly increased when two waves arrive at any point on the constructive curve, we pay close attention to the position relationships between constructive curves and the SDD of each PoI. Based on this, we try to find the optimal sensor deployment location with the highest power from a limited number of interference enhanced regions. According to the number of constructive curves goes through the SDD, there are three different cases for selecting the optimal sensor locations. 

Case 1: The number of constructive curves passing through the SDD is zero (see Fig. 6(a)). In this case, the highestpower location in the SDD is the point on the circumference closest to the constructive curve. Then, we can obtain the optimal deployment location with the highest power by solving the minimum distance problem for separated hyperbolas and circle (red dot in Fig. 6(a)). 

![](images/33a65ec5843c6ead10845b6088c8d2af10143db4465a3a10ddb91b4d50a66925.jpg)



(a) Case 1: The number of constructive curves passing through the SDD is zero.


![](images/c3607070c5ddf35631e339437416781ec44a3b8a0d50fc359cc7d2535cd1d68e.jpg)



(b) Case 2: The number of constructive curves passing through the SDD is only one


![](images/b0a3e75693f3310156fa8ae2d6d89b151768840c57973b278c13c5517b545365.jpg)



(c) Case 3: The number of constructive curves passing through the SDD is more than one.



Fig. 6. An example for finding sensor deployment location in the overlap covered by two chargers.


Case 2: The number of constructive curves passing through the SDD is only one (see Fig. 6(b)). In this case, our basic idea here is to find the corresponding highest-power point from each interference enhanced region in the SDD. Then, we choose an optimal point from these limited points as the sensor deployment location. Specifically, for the interference enhanced region with constructive curve passing through, our solution is to find the point closest to the two chargers on the constructive curve as the highest-power point. Thus, we draw a straight line by connecting $c _ { 1 } , \ : c _ { 2 }$ . If the constructive curve intersects with $\overline { { c _ { 1 } c _ { 2 } } }$ in the SDD, the highest-power point is the intersection of them. Otherwise, the point must be on the circumference of the SDD. For the interference enhanced region without constructive curve passing through, we can use the method in Case 1 to find the highest-power point on the circumference closest to the corresponding constructive curve. Then, we compare the power of these limited points and select the highest one as the sensor deployment location (red dot in Fig. 6(b)). 

Case 3: The number of constructive curves passing through the SDD is more than one (see Fig. 6(c)). In this case, we aim to find the corresponding highest-power point on each constructive curve and in each interference enhanced region without constructive curve passing through, respectively. Then, we compare the power of these limited points and select an optimal one to deploy sensor (red dot in Fig. 6(c)). 

## B. PoI Covered Concurrently by Multiple Chargers

Next, we consider the complex situation that PoI is covered concurrently by multiple chargers (more than 2 chargers). Generally, when $m ( m > 2 )$ chargers concurrently cover a PoI, m waves will interfere with each other, which obviously makes the wave interference complicated. In order to reveal the power distribution regularity caused by the interference of multiple waves and help us design a feasible and effective sensor deployment method, we first consider the case where a PoI is covered by three chargers, and then we extend the solution to the scenario where the PoI is covered by more than 3 chargers. 

(1) Solution for PoI covered concurrently by 3 chargers. 

![](images/3f35407f4947abbc219fb7b00043970646caea3fbbc97df0ed251c62299a38a4.jpg)



Fig. 7. The power distribution in the overlap covered by three chargers.


To develop a sensor deployment method, we first give the following theorem to explore the regularity of the power distribution in the area covered by three chargers. 

## Theorem 6. The interference enhanced regions are distributed in spots in the overlap concurrently covered by three chargers.

Proof: Assume there are three chargers $c _ { 1 } , \ c _ { 2 } ,$ , and c<sub>3</sub> randomly placed on a 2D plane. Let any constructive curve of $c _ { 1 }$ and $c _ { 2 }$ denoted as $f ( c _ { 1 } c _ { 2 } )$ , any constructive curve of $c _ { 1 }$ and $c _ { 3 }$ denoted as $f ( c _ { 1 } c _ { 3 } )$ intersect at a point a. If we denote the distance between a and $c _ { 1 }$ as $d _ { a c _ { 1 } } = d ,$ then the distance between a and $c _ { 2 } , \ a$ and $c _ { 3 }$ is $d _ { a c _ { 2 } } \ = \ d + k _ { 1 } \lambda$ $d _ { a c _ { 3 } } = d + k _ { 2 } \lambda , k _ { 1 } , k _ { 2 } \in \mathcal { N }$ , respectively. What we found is that the distance difference between a and $^ { c _ { 2 } , }$ a and $c _ { 3 }$ denoted as $\Delta d \ = \ d _ { a c _ { 2 } } \ - \ d _ { a c _ { 3 } } \ = \ ( k _ { 1 } \ - \ k _ { 2 } ) \lambda ,$ which is exactly the integer multiples of wavelength. This suggests the point a is also located on the constructive curve of $c _ { 2 }$ and $c _ { 3 }$ denoted as $f ( c _ { 2 } c _ { 3 } )$ , that is, $f ( c _ { 2 } c _ { 3 } )$ passes through the intersection of $f ( c _ { 1 } c _ { 2 } )$ and $f ( c _ { 1 } c _ { 3 } )$ . Therefore, we can see that the intersection formed by any two constructive curves must be passed by the third constructive curve, i.e., the radio waves radiated by the three chargers will inevitably interfere constructively at a point. Furthermore, in the region around this constructive point, the combined power will also be enhanced to some degree. As a result, in the overlap concurrently covered by three chargers, the interference enhanced regions are spot-shaped. 

Fig. 7 shows the power distribution in the overlap covered by three chargers. We can see the interference enhanced regions distributed in spots. In order to take full advantage of the high power of interference enhanced regions, similar to the case covered by two chargers, we still focus on the position relationships between each SDD and the spots, and try to find the sensor deployment location with the highest power from a limited number of spot-shaped interference enhanced regions. According to the number of constructive spots located in the SDD, there are three different cases for selecting the optimal sensor locations. 

Case 1: The number of constructive spots located in the SDD is zero (see Fig. 8(a)). In this case, we find the point on the 

![](images/1a0d05330e0437762e9b29358f4aebbb8a56772b7a397d2d79224bbc72d6e5a7.jpg)



(a) Case 1: The number (b) Case 2: The number (c) Case 3: The number of constructive spots lo- of constructive spots lo- of constructive spots located in the SDD is zero. cated in the SDD is only cated in the SDD is more one. than one.


Fig. 8. An example for finding sensor deployment location in the overlap covered by three chargers. 

circumstance closest to the corresponding constructive points outside the SDD as the optimal sensor deployment location (red dot in Fig. 8(a)). 

Case 2: The number of constructive spots located in the SDD is only one (see Fig. 8(b)). Similar to Case 1, We first find a point on the circumference of the SDD that is closest to the constructive spots outside the SDD. Then, we calculate the power of this point and the only constructive spot in the SDD respectively, and select the point with the higher power as the optimal sensor location (red dot in Fig. 8(b)). 

Case 3: The number of constructive spots located in the SDD is more than one (see Fig. 8(c)). In this case, the points we need to compare include all constructive spots in the SDD, and the point on the circumference of the SDD closest to the constructive spots outside the SDD. The point which has the highest power is the optimal sensor deployment location (red dot in Fig. 8(c)). 

(2) Solution for PoI covered concurrently by more than 3 chargers. 

When PoI is covered by $m ( m > 3 )$ chargers concurrently, how these constructive curves intersect is a complicated problem, since the number of curves that can intersect at a certain point varies from 2 to $C ( m , 2 )$ . Obviously this makes it extremely difficult to find the highest-power point in the SDD. Fortunately, the regularity of the spotted power distribution covered by 3 chargers provides a way for us to design a feasible and effective sensor deployment method. 

Note that, when a PoI is concurrently covered by 3 chargers, the constructive curves formed by each pair of chargers will always intersect at a point according to Theorem 6. Though when more chargers interfere with each other introducing more curves accordingly, there will inevitably be points interacted by more than 3 curves, we actually can regard this point as a 3-curve intersection that happens to be passed by other constructive curves. Thus, we take the scenario covered by three chargers as the basis of the more complex situation. 

Based on the above analysis, for the problem of deploying sensors in the SDD concurrently covered by more than 3 chargers, our basic idea is to find all intersections of any three curves in the SDD and the point on the circumference of the SDD closest to the intersections of any three curves outside the SDD. Then we compare the power of them and select the point with the highest power as the sensor deployment location. Considering the SDD is very limited as well the minimum distance between adjacent constructive curves is also $\lambda / 2$ , then the number of intersections formed by any three curves appearing in the SDD is also limited, which guarantees the efficiency of our sensor deployment algorithm. 

Algorithm 3 Sensor deployment algorithm
Input: The set of chargers C, the set of PoIs O, the number of sensors N, and the radius r of SDDs
Output: N sensor deployment locations
1: $S = \emptyset$ 2: for each PoI $o_{i} \in O$ do
3: if $o_{i}$ is covered by only one charger then
4: select $o_{i}$ as the sensor deployment location $s_{i}$ ;
5: else if the number of chargers concurrently cover $o_{i}$ is two then
6: Find the corresponding highest-power point on each constructive curve passing through the SDD and the point on the SDD's circumference closest to the constructive curve outside the SDD;
7: Compare the power of these points and select the highest one as the sensor deployment location $s_{i}$ ;
8: else
9: Find all intersections of any three curves in the SDD and the point on the circumference of the SDD closest to the intersections of any three curves outside the SDD;
10: Compare the power of these points and select the highest one as the sensor deployment location $s_{i}$ ;
11: end if
12: $S = S \cup \{s_{i}\}$ ;
13: end for 

The detailed process of sensor deployment algorithm is given in Algorithm 3. 

## V. SIMULATIONS

## A. Simulation Setup

We consider a WRSN consisting of 15 PoIs, which are randomly distributed on a 2D plane of 20m × 20m. Each PoI has a sensor deployable disk (SDD) centered at itself with a radius of 10cm to deploy a sensor equipped with an omnidirectional antenna. We also have 10 omnidirectional chargers, the farthest charging distance $D = 4 m$ . The wavelength is set as $\lambda = 0 . 3 3 m$ according to the commercial off-the-shelf TX91501 wireless charger produced by Powercast [21] and the energy transmission power of the charger is 3W. Other relative parameters are set as: $\alpha = 1 0 0 , \beta = 4 0 , \epsilon = 0 . 2 $ and $P _ { t h } = 1 0 m W$ 

## B. Baseline Setup

To evaluate the performance of our GAIN scheme, we compare it with the following three charging algorithms. 

Balanced Concurrent Charging Scheduling Problem (BCCSP) [16] is a charging scheduling algorithm aiming at accelerating the concurrent charging. In one charging cycle, all chargers are turned on in some order until all sensors are fully charged. The overall charging utility BCCSP obtained can be calculated as the energy all sensors received divided by the cycle duration. Note that the amount of energy each sensor receives in one charging cycle is the battery capacity, thus the charging utility yielded by each sensor is equal. Specially, following the settings in [16], the battery capacity of each sensor is set to 4mJ. 

![](images/e06f2d75c06b75a4fbd0bba8539eafc7fc70a5b295b92bad32eeda5c6efae10d.jpg)



(a) M vs. charging utility


![](images/132da41cc8beef7645858e12263924b68bf4393db7ba8a0a08ae93f9ff59e010.jpg)



(b) N vs. charging utility


![](images/a6ae16924eb3cb9c0c5139af754fe7c9e14252507a1ef7ae4d43bc270ca908fa.jpg)



(c) D vs. charging utility


![](images/2e121c053813e36b5f440ba5a87ef5d6c3f738de3c77354bf896e1366d393b2e.jpg)



(d) r vs. charging utility



Fig. 9. Performance comparisons.


<sup>12</sup> <sup>16</sup> <sup>20</sup>Randomized Charger Position (RCP) is a concurrent charging algorithm developed by us, which follows the same sensor deployment strategy as GAIN but places chargers randomly. 

Deployed Sensor on PoI (DSP) [7] is an omnidirectional charger placement algorithm, which assumes the charging power is additive from different chargers. To maximize the overall additive power arrived at all PoIs, a greedy method is employed to select charger placement locations. Since DSP does not consider the effects of wave interference, all sensors are deployed on corresponding PoIs. 

## C. Performance Comparisons

Impact of number of chargers M. Our simulation results show that on average, GAIN outperforms BCCSP, RCP, and DSP by 84.51%, 112.74%, and 25.72%, respectively, in terms of M . Fig. 9(a) shows that the charging utility of all algorithms increases as M grows. When fewer chargers are placed, the GAIN and DSP have an obvious advantage since the charger placement strategy of these two algorithms enables as many sensors as possible to be covered. When M is larger, locating sensors to the highest-power point guarantees the best performance of our proposed scheme. 

Impact of number of PoIs N. Our simulation results show that on average, GAIN outperforms BCCSP, RCP, and DSP by 42.50%, 34.48%, and 24.27%, respectively, in terms of N. From Fig. 9(b), we can see that the overall charging utility achieved by all algorithms increases with the number of PoIs. Our scheme always maintains the best performance, which suggests that our GAIN is robust to different PoI density. 

Impact of farthest charging distance D. Our simulation results show that on average, GAIN outperforms BCCSP, RCP, and DSP by 79.38%, 102.02%, and 24.55%, respectively, in terms of D. To guarantee the sensors covered by chargers can receive non-negligible power, we set the farthest charging distance D from 2m to 4m. Fig. 9(c) demonstrates that the overall charging utility yielded by four algorithms shows an increasing trend with D. The reason is that a larger D means more sensors can be covered by chargers. It also can be seen that the GAIN scheme achieves the best performance under different D. 

Impact of Radius of SDDs r. Our simulation results show <sup>15</sup> ti<sup>l</sup>ti<sup>l15</sup> ti<sup>l</sup>ti<sup>l</sup>that on average, GAIN outperforms BCCSP, RCP, and DSP <sub>i</sub>n<sup>g</sup><sub>i</sub>n<sup>g</sup> <sub>i</sub>n<sup>g</sup><sub>i</sub>n<sup>g</sup>by 25.57%, 37.97%, and 20.95%, respectively, in terms of r. h<sup>a</sup>h<sup>a</sup> h<sup>a</sup>h<sup>a</sup>Fig. 9(d) shows that the overall charging utility achieved by 55GAIN first increases rapidly with $r ,$ <sup>55</sup> but grows slowly when 010 12 14 16 18 20 0 410 12 14 16 18 20r approaches 10. This suggests that even sensors can only be deployed very close to the PoIs, the GAIN scheme is able to find the deployment locations with the highest power. Besides, without the elaborate charger placement strategy, the charging utility of RCP grows slowly after $r = 7 . 5 c m$ , since the highpower interference enhanced regions rarely appear around the PoIs. While the BCCSP and DSP remain constant with r because their sensor deployment locations are fixed at PoIs. 

## VI. FIELD EXPERIMENTS

To better verify the performance of our proposed GAIN scheme, we conduct field experiments in this section. 

## A. Testbed

As Fig. 10 shows, our test-bed consists of three wireless chargers (TX91501 power transmitters produced by Powercast [21]) with $\lambda \ : = \ : 3 3 c m$ , eight rechargeable sensors and an AP connecting to a laptop to record the collected data from sensors. All sensors are deployed within the SDDs with radius r = 10cm centered at corresponding PoIs in a 3m × 3m square areas, and their coordinates are (48, 152), (125, 226), (120, 77), (196, 106), (205, 210), (248, 118), (260, 108), (262, 225) as shown in Fig. 11. We set D = 1.5m, $P _ { t h } = 1 0 m W .$ Moreover, we eliminate the impact of randomness by taking the same charger placement strategy for BCCSP with GAIN and DSP for a better comparison. Since the TX91501 is a directional charger, we always rotate it to face the corresponding sensor when necessary to record the experimental data. 

## B. Experimental Results

Table I shows the overall charging utility for all algorithms, and GAIN outperforms BCCSP, DSP, and RCP by 39.8%, 49.9%, and 87.2%, respectively. This verifies that GAIN achieves the excellent performance by carefully selecting the charger positions and locating each sensor to the highest-power point in each SDD. It also can be seen that BCCSP collects more charging utility than RCP and DSP do. The reason is that BCCSP avoids the sensors located within the interference weakened regions by turning off some chargers. Since DSP assumes that the charging power is additive from different chargers, and accordingly deploys all sensors on PoIs, it gets the worst performance. 

![](images/e53ad72e17648b712a728b2794042fd3e41f959b386e87222ef2ec15c8e20848.jpg)



Fig. 10. Testbed.<sup>SDD</sup>


![](images/9581b424923f840de6147cdbf5b448cade8864853906d6612b1d32a246cac5e7.jpg)



Fig. 11. Illustration of field experiment.r


Fig. 12 shows the charging utility obtained by each sensor,<sub>sor</sub> <sub>ID Sensor</sub> <sub>ID</sub> and we can see GAIN enables five sensors to obtain the highest charging utility, and that obtained by the other three sensors is also more than 0.5. This verifies the effectiveness and fairness of our GAIN scheme. 

## VII. RELATED WORK<sup>0.6</sup> <sup>U</sup>

Wireless charger placement: In recent years, many wireless<sup>0.4</sup>r<sup>g</sup> charger placement schemes have been proposed. For example,C Zhang et al. [7] employed omnidirectional chargers with adjustable power, jointly determining the charger placement<sup>0.0</sup> and corresponding power allocation to maximize the charging utility. Wang et al. [8] considered the problem of practical heterogeneous wireless charger placement with obstacles, and proposed a charger placement algorithm to maximize the overall charging utility. Dai et al. [9] studied how to improve the charging efficiency by placing wireless chargers with limited mobility. However, all these work ignore the wave interference and instead assume that the charging power is additive from different chargers. 

There are also a few researchers having realized the interference effect. Guo et al. [16] tried to enhance the charging efficiency by scheduling (i.e., turn on/off) the chargers in concurrent charging scenarios. Similar work has been reported in [17], which also designed a charger scheduling algorithm. However, turning off the chargers may not only reduce the output on the energy supply side but also not make full use of the high power due to constructive interference. 


TABLE I



CHARGING UTILITY FOR FOUR ALGORITHMS.


<table><tr><td></td><td>GAIN</td><td>BCCSP</td><td>RCP</td><td>DSP</td></tr><tr><td>charging utility</td><td>7.159</td><td>5.120</td><td>4.776</td><td>3.824</td></tr></table>

![](images/46f9fcfee2e40fc2301d8a6fa3a75fab014e1764f86ad43cb9bcaf58dc54199b.jpg)



Fig. 12. Charging utility of 8 sensors.


Sensor deployment: Deploying sensors is a traditional issue in WSNs, and much effort has been devoted to improving the sensing, connectivity, and coverage quality. For instance, Guo et al. [22] focused on optimizing the sensing quality with a constraint of communication range by deploying sensors. Boubrima et al. [23] studied how to use WSN for air pollution mapping, and then they proposed a sensor deployment algorithm. Saad et al. [24] noticed a more practical scenario and studied the 3D WSNs deployment problem. Fu et al. [25] studied the sensor calibration problem. They achieved the khop calibration of all sensors in the network by deploying high-precision reference sensors. 

## VIII. CONCLUSION

Wave interference is a typical physical phenomenon when multiple chargers concurrently transmit power. In this work, we explore the power distribution regularity of concurrent charging and take full advantage of the high power caused by wave interference accordingly to promote network performance. To this end, we formulate the concurrent charging utility maximization problem and propose a scheme consisting of a charger placement algorithm and a sensor deployment algorithm to solve the problem. Extensive simulations are conducted and the results show GAIN outperforms the comparison algorithms by 40.48% on average. Field experiments also demonstrate the feasibility of GAIN in practical scenarios. 

## ACKNOWLEDGMENT

The authors would like to thank Dr. Baijun Wu for providing us useful feed-back and insightful suggestions, which helped us improve the quality and presentation of the paper greatly. This work is partially supported by the National Natural Science Foundation of China (62072320, 62002250), the Natural Science Foundation of Sichuan Province (2022NS-FSC0569, 2022NSFSC0929), the Key R&D Program of Sichuan Province (22ZDZX0021). 



[1] A. Kurs, A. Karalis, R. Moffatt, J. D. Joannopoulos, P. Fisher, and M. Soljaciˇ c, “Wireless power transfer via strongly coupled magnetic´ resonances,” Science, vol. 317, no. 5834, pp. 83–86, 2007. 





[2] Y. Yang and C. Wang, Wireless rechargeable sensor networks. Springer, 2015. 





[3] P. Abouzar, D. G. Michelson, and M. Hamdi, “Rssi-based distributed self-localization for wireless sensor networks used in precision agriculture,” IEEE Transactions on Wireless Communications, vol. 15, no. 10, pp. 6638–6650, 2016. 





[4] Y. Sun, C. Lin, H. Dai, P. Wang, L. Wang, G. Wu, and Q. Zhang, “Trading off charging and sensing for stochastic events monitoring in wrsns,” IEEE/ACM Transactions on Networking, vol. 30, no. 2, pp. 557– 571, 2022. 





[5] X. Fan, L. Shangguan, R. Howard, Y. Zhang, Y. Peng, J. Xiong, Y. Ma, and X.-Y. Li, “Towards flexible wireless charging for medical implants using distributed antenna system,” in ACM MobiCom, 2020, pp. 1–15. 





[6] T. Liu, B. Wu, W. Xu, X. Cao, J. Peng, and H. Wu, “Rlc: a reinforcement learning-based charging algorithm for mobile devices,” ACM Transactions on Sensor Networks, vol. 17, no. 4, pp. 1–23, 2021. 





[7] S. Zhang, Z. Qian, F. Kong, J. Wu, and S. Lu, “P3: Joint optimization of charger placement and power allocation for wireless power transfer,” in IEEE INFOCOM, 2015, pp. 2344–2352. 





[8] X. Wang, H. Dai, W. Wang, J. Zheng, N. Yu, G. Chen, W. Dou, and X. Wu, “Practical heterogeneous wireless charger placement with obstacles,” IEEE Transactions on Mobile Computing, vol. 19, no. 8, pp. 1910–1927, 2020. 





[9] H. Dai, X. Wang, X. Lin, R. Gu, S. Shi, Y. Liu, W. Dou, and G. Chen, “Placing wireless chargers with limited mobility,” IEEE Transactions on Mobile Computing, 2021. 





[10] N. Yu, H. Dai, G. Chen, A. X. Liu, B. Tian, and T. He, “Connectivityconstrained placement of wireless chargers,” IEEE Transactions on Mobile Computing, vol. 20, no. 3, pp. 909–927, 2021. 





[11] H. Dai, X. Wang, A. X. Liu, H. Ma, and G. Chen, “Optimizing wireless charger placement for directional charging,” in IEEE INFOCOM 2017- IEEE Conference on Computer Communications. IEEE, 2017, pp. 1–9. 





[12] C. Lin, W. Yang, H. Dai, T. Li, Y. Wang, L. Wang, G. Wu, and Q. Zhang, “Near optimal charging schedule for 3-d wireless rechargeable sensor networks,” IEEE Transactions on Mobile Computing, 2022. 





[13] H. Dai, Y. Liu, G. Chen, X. Wu, T. He, A. X. Liu, and H. Ma, “Safe charging for wireless power transfer,” IEEE/ACM Transactions on Networking, vol. 25, no. 6, pp. 3531–3544, 2017. 





[14] T. Wu, P. Yang, and H. Dai, “Charging on the move: Scheduling static chargers with tunable power for mobile devices,” in IEEE/ACM IWQOS, 2021, pp. 1–10. 





[15] T. Liu, B. Wu, H. Wu, and J. Peng, “Low-cost collaborative mobile charging for large-scale wireless sensor networks,” IEEE Transactions on Mobile Computing, vol. 16, no. 8, pp. 2213–2227, 2017. 





[16] P. Guo, X. Liu, S. Tang, and J. Cao, “Concurrently wireless charging sensor networks with efficient scheduling,” IEEE Transactions on Mobile Computing, vol. 16, no. 9, pp. 2450–2463, 2017. 





[17] Z. Ma, S. Zhang, J. Wu, Z. Qian, Y. Zhao, and S. Lu, “Fast charging scheduling under the nonlinear superposition model with adjustable phases,” ACM Transactions on Sensor Networks, vol. 15, no. 4, pp. 1–23, 2019. 





[18] D. K. Cheng et al., Field and wave electromagnetics. Pearson Education India, 1989. 





[19] B. Xiao, J. Cao, Q. Zhuge, Y. He, and E.-M. Sha, “Approximation algorithms design for disk partial covering problem,” in IEEE ISPAN, 2004, pp. 104–109. 





[20] S. He, J. Chen, F. Jiang, D. K. Yau, G. Xing, and Y. Sun, “Energy provisioning in wireless rechargeable sensor networks,” IEEE transactions on mobile computing, vol. 12, no. 10, pp. 1931–1942, 2012. 





[21] [Online]. Available: http://www.powercastco.com. 





[22] J. Guo and H. Jafarkhani, “Movement-efficient sensor deployment in wireless sensor networks with limited communication range,” IEEE Transactions on Wireless Communications, vol. 18, no. 7, pp. 3469– 3484, 2019. 





[23] A. Boubrima, W. Bechkit, and H. Rivano, “On the deployment of wireless sensor networks for air quality mapping: optimization models and algorithms,” IEEE/ACM Transactions on Networking, vol. 27, no. 4, pp. 1629–1642, 2019. 





[24] A. Saad, M. R. Senouci, and O. Benyattou, “Toward a realistic approach for the deployment of 3d wireless sensor networks,” IEEE Transactions on Mobile Computing, vol. 21, no. 4, pp. 1508–1519, 2022. 





[25] K. Fu, W. Ren, and W. Dong, “Multihop calibration for mobile sensing: K-hop calibratability and reference sensor deployment,” in IEEE INFOCOM, 2017, pp. 1–9. 

