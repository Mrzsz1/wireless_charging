# Cooperative Charging as Service: Scheduling for Mobile Wireless Rechargeable Sensor Networks

Jia Xu<sup>∗</sup>, Suyi Hu<sup>∗</sup>, Sixu Wu<sup>∗</sup>, Kaijun Zhou<sup>∗</sup>, Haipeng Dai<sup>†</sup>, Lijie Xu<sup>∗</sup> 

<sup>∗</sup>Jiangsu Key Laboratory of Big Data Security and Intelligent Processing, Nanjing University of Posts and Telecommunications, Nanjing, Jiangsu 210023 China 

<sup>†</sup>State Key Laboratory for Novel Software Technology, Nanjing University, Nanjing, Jiangsu 210023 China Email: xujia@njupt.edu.cn, 1218043215@njupt.edu.cn, 1019041115@njupt.edu.cn, 1019041113@njupt.edu.cn, haipengdai@nju.edu.cn, ljxu@njupt.edu.cn Corresponding author: Lijie Xu 

Abstract—Wireless Power Transmission (WPT) has been widely used to replenish energy for Wireless Rechargeable Sensor Networks. However, the charging service model, which is of the essence to commercial WPT, has not emerged so far. In this paper, we present a wireless charging service model from the perspective of cooperative charging economics, and formulate the Cooperative Charging Scheduling (CCS) problem for joint optimization of rechargeable devices’ charging cost and moving cost. We first propose two intragroup cost sharing schemes to sustain the cooperation among devices. Then, the approximation algorithm CCSA of the CCS problem is proposed based on greedy approach and submodular function minimization. Furthermore, we model the large-scale CCS problem as a coalition formation game and present a game theoretic algorithm CCSGA. We show that CCSGA finally converges to a pure Nash Equilibrium. We conduct simulations, and field experiments on a testbed consisting of 5 chargers and 8 rechargeable sensor nodes. The results show that the average comprehensive cost of CCSA is 27.3% lower than the noncooperation algorithm and is only 7.3% higher than the optimal solution on average. In field experiments, CCSA outperforms the noncooperation algorithm by 42.9% in terms of comprehensive cost on average. Moreover, CCSGA is much faster than the approximation algorithm and is more suitable for large-scale cooperative charging scheduling. 

Index Terms—WRSN, cooperative charging service, submodular function, coalition formation game, Nash Equilibrium 

## I. INTRODUCTION

Since most existing Wireless Sensor Networks (WSN) are battery-powered, the lifetime of sensor nodes is limited [1]. In addition, it will be very costly to replace batteries for the sensor nodes in harsh environments [2]. At present, the rechargeable devices can absorb various forms of energy, such as solar energy and wind energy, from the surrounding environment [3]. However, the energy extraction efficiency is largely influenced by the deployment environment and weather, which is highly unpredictable and unstable. 

Wireless Power Transmission (WPT) provides continuous and reliable power supply for the rechargeable devices without battery replacement [4] [5]. With the advance of WPT technology, Wireless Rechargeable Sensor Network (WRSN) has been largely developed in real life, such as unmanned aerial vehicles [6], driverless electric vehicles [7], industrial robots [8], automated underwater vehicles [9] and RFID systems [10]. to prolong the lifetime of traditional WSN. 

In most situations, the number of chargers is limited, and it is infeasible to make the chargers cover all sensor nodes. For example, the WRSN can help exhibition guiding in a largescale area, e.g., world exhibition. According to the historical statistics [11], the average scale of world exhibition is about 100 hectares. Since the average coverage range of a charger is only a few meters, we need to deploy about one million chargers to fully cover the sensor nodes in exhibition area. 

Many existing studies focused on the problem of charger deployment for both static sensor nodes [12] [13] and mobile sensor nodes [14] [15]. There are also studies related to cooperative charging. [16] proposed a reliable cooperative charging protocol with a data screening mechanism to guarantee the charging system from fault data. [17] extended the constant-current constant-voltage charging protocol to multicharger systems using a cooperative control method, which can alleviate the current imbalance among chargers effectively. However, there is no off-the-shelf rechargeable device scheduling for the cooperation among rechargeable devices to save the cost of charging service. 

Different from the existing works, we focus on exploiting the wireless charging service model for omnidirectional chargers from the new perspective of cooperative charging economics. The business model of wireless charging services is of the essence to popularize the WPT technology further. With the development of WPT technology, the wireless charger is going to become the infrastructure, which provides paid energy supply service for rechargeable devices, just like the 5G base station for providing high speed communication service and the electric vehicle charging station for providing fast energy replenishment service. Different from the directional charging technology, such as laser charging technology [18], the omnidirectional chargers can provide the energy supply for multiple rechargeable devices in the near open field simultaneously without additional discharging cost [4]. Therefore, multiple rechargeable devices in the common charging hours can share the charging cost, reducing the individual cost. Thus, the cooperative charging is a natural and economical service model for the omnidirectional charging technology. Note that the cooperative charging service is a common service model and can be applied to many existing wireless charging systems to reduce the actual charging expenditure. 

![](images/b148f8542f04d5f5fd08c0d2f8ce2ac517507b71b0d6c3382413fb04205a2374.jpg)



Fig. 1. Illustration of cooperative charging system


The key problem of cooperative charging service is how to assign the mobile devices to the appropriate chargers to reduce the cost of whole system. In other words, the economic surplus of the cooperative charging service largely depends on the scheduling of mobile devices. From the view of wireless charging market, such strategic charging scheduling can bring the competition among the Charging Service Providers (CSP), and help to promote the marketized price of charging service. 

In this paper, we present a cooperative charging system for the environmental monitoring shown in Fig. 1. We consider a set of omnidirectional wireless chargers located at fixed positions in a 2D plane. These wireless chargers are operated by different CSPs and may have different charging prices. The charging service is provided with fixed charging cycle, which can be adjusted on demand. These rechargeable devices can move from the initial locations to the corresponding chargers to obtain the charging service. The comprehensive cost of any device is the sum of charging cost (payment to the charger) and the round-trip moving cost between the initial location and the charging position. The devices assigned to the same charger form a charging group, in which the devices can obtain the surplus by sharing the charging cost in the common charging hours. The objective is minimizing the total comprehensive cost of all charging groups such that each device can obtain the required charging service. 

The problem of scheduling mobile rechargeable devices for cooperative charging service is very challenging. First, we need to design an intragroup cost sharing scheme to sustain not only the economic surplus of each device but also the cooperation among all users in the charging group in an economically stable manner. Second, in order to optimize the comprehensive cost, we need to partition the devices into multiple charging groups. However, finding the optimal partition needs exponential time. We will show that this problem is harder than standard Facility Location Problem (FLP) [19]. Moreover, the optimal algorithm or approximation algorithm (if exists) may have high time complexity. Thus, lightweight algorithm is needed to deal with the large-scale cooperative charging scheduling problem. 

The main contributions of this paper are as follows: 

• To the best of our knowledge, this is the first work to exploit the wireless charging service model from the perspective of cooperative charging economics. 

• We present the cooperative charging model, and formulate the Cooperative Charging Scheduling (CCS) problem to minimize the comprehensive cost. 

• We present the Cooperative Charging Scheduling Algorithm (CCSA) based on greedy approach and submodular function minimization [20] to solve the CCS problem with $( { \frac { \ln n + 1 } { 1 - \varepsilon } } ) ^ { }$ )-approximation, where n is the number of rechargeable devices, and ε is the search precision. 

• To avoid the high complexity of approximation algorithm, we model the large-scale CCS problem as a coalition formation game, called CCS Game. We show that the proposed CCS Game Algorithm (CCSGA) will finally converges to a Nash-stable Coalition Structure. 

• We conduct extensive simulations and field experiments. The results show that the average comprehensive cost of CCSA is 27.3% lower than the noncooperation algorithm and is only 7.3% higher than the optimal solution on average. Furthermore, CCSA outperforms the noncooperation algorithm by 42.9% in terms of comprehensive cost on average in field experiments. 

The rest of this paper is organized as follows. We review the state-of-art research in Section II. We present the cooperative charging model and formulate the CCS problem in Section III. We present two intragroup cost sharing schemes in Section IV. The approximation algorithm and the theoretical analysis are presented in Section V. In Section VI, we present the CCSGA for large-scale CCS problem and analyze the game theoretical properties of the proposed algorithm. The simulation results are presented in Section VII. Field experiments are shown in Section VIII. We conclude this paper in Section IX. 

## II. RELATED WORK

## A. Charger Deployment

The current researches of charger deployment mainly consider the following three scenarios: 

• Deploying static chargers for static rechargeable devices Dai et al. [21] considered the ElectroMagnetic Radiation (EMR) induced by chargers and proposed the approximation algorithms to find the charger placement that maximizes the overall charging utility subject to an EMR safety threshold. They further studied the charging task scheduling and power adjustment issues in [22] and [23], respectively. 

Li et al. [14] studied wireless charging service provision for wearable devices worn by users in a 2-D area, where the users have a specific stay-move behavior pattern characterized by the trajectories, stay points and stay time distribution. Zhang et al. [15] considered the static devices and mobile devices to optimize the charging quality where the power of each charger is adjustable. He et al. [24] also studied the problem of path provisioning that exploits the potential mobility of devices to further reduce the number of required chargers. 

• Deploying mobile chargers for static rechargeable devices Shu et al. [25] first studied traveling velocity control of the mobile charger for the time-bounded charging scenario. Fusco et al. [26] addressed the problem of selecting positions and adjusting their orientations for directional sensors with the objective of maximizing their joint coverage area. Liu et al. [27] proposed the grid-based algorithm, dominatingset-based algorithm, and circle-intersection-based algorithm to find a set of anchor points. Then, the mobile device scheduling algorithm is proposed to schedule minimum mobile devices to visit the generated anchor points. Tomar et al. [28] proposed a fuzzy logic-based scheduling scheme for ondemand charging of the sensor nodes. The proposed scheme blends different network parameters, such as residual energy, distance to mobile chargers and critical node density to make decisions while scheduling the sensor nodes. 

Different from the works mentioned above, this paper aims to schedule the rechargeable devices for the deployed chargers from the perspective of economic cost. 

## B. Cooperative Charging

In recent years, much attention has been paid to cooperative charging methods. Zhang et al. [29] studied how to schedule multiple Wireless Charging Vehicles (WCVs) to maximize energy usage effectiveness. In addition, they proposed a scheduling algorithm, which is proved to be optimal for onedimensional WSN. Wu et $a l .$ [30] gave a short survey of research in the area of collaborative mobile charging. In [31], the authors further formed a hierarchical charging architecture to enhance the collaborative feature. Lin et al. [32] proposed a game theoretical collaborative charging scheme, in which each WCV seeks for the maximum profit when fulfilling charging tasks. Different from the existing study of cooperative charging, our work studies a novel scheduling problem for the cooperation among rechargeable devices to save the cost of charging service. 

Overall, the economic model of charging service has not been studied so far. There is no off-the-shelf rechargeable device scheduling proposed in the literature for the cooperative charging service. 

## III. SYSTEM MODEL AND PROBLEM FORMULATION

## A. Cooperative Charging Model

We consider a set of m omnidirectional wireless chargers $M = \{ s _ { 1 } , s _ { 2 } , . . . , s _ { m } \}$ located at fixed positions in a 2D plane Ω. These wireless chargers are operated by different $C S P s ,$ , and therefore, they may have different charging prices. Without loss of generality, we consider that each charger $s _ { j } ~ \in ~ M$ provides paid energy supply service with unit charging price $a _ { j }$ [33]. Suppose that there are a set of n mobile rechargeable devices $N = \{ o _ { 1 } , o _ { 2 } , . . . , o _ { n } \}$ located in the same 2D plane. Each mobile rechargeable device $o _ { i } \in N$ requires energy $E _ { i } .$ 

In order to guarantee the quality and efficiency of charging, each charger $s _ { j }$ has a charging equipment to fix the rechargeable devices with charging distance $d _ { j }$ The mobile rechargeable devices need to move to the charging equipment for charging. We assume that the remaining energy of the rechargeable device is sufficient to move to the corresponding charging equipment when it submits the charging request. 

We employ Friis’s free space equation as the charging model [28]. The charging power from any charger $s _ { j }$ to any rechargeable device $o _ { i }$ is given by 

$$
P r (s _ {j}, o _ {i}) = \left\{ \begin{array}{c c} \frac {\alpha}{(\beta + d _ {j}) ^ {2}}, & d _ {j} \leq D _ {j} \\ 0, & o t h e r w i s e \end{array} \right.\tag{1}
$$

where α and $\beta$ are two parameters determined by the magnetic environment and hardware [34]. $D _ { j }$ is the maximum charging distance to obtain the positive power from the charger $s _ { j } .$ . Note that $d _ { j }$ is a known constant and is less than $D _ { j }$ definitely, thus the rechargeable devices can always obtain positive power. 

Let $b _ { i }$ be the unit moving cost of device $o _ { i }$ . Without loss of generality, we consider that the distance between the rechargeable device $o _ { i }$ and charger $s _ { j }$ is $| | s _ { j } o _ { i } | |$ when $o _ { i }$ submits the charging request. Then the moving distance from $o _ { i }$ to $s _ { j }$ can be calculated by 

$$
r (s _ {j}, o _ {i}) = | | | s _ {j} o _ {i} | | - d _ {j} |\tag{2}
$$

Let $c o _ { j }$ be the charging group of $s _ { j }$ . Then the charging cost of group $c o _ { j }$ can be calculated by 

$$
c c (c o _ {j}) = a _ {j} \max _ {o _ {i} \in c o _ {j}} \frac {E _ {i}}{P r (s _ {j} , o _ {i})}\tag{3}
$$

where $\operatorname* { m a x } _ { o _ { i } \in c o _ { j } } \frac { E _ { i } } { P r ( s _ { j } , o _ { i } ) }$ represents the maximum charging time of devices in group $c o _ { j }$ . Here, we ignore the moving time of rechargeable devices since it is small compared to the charging time. This assumption is reasonable for mobile wireless charging as made in [35]. 

To keep working (e.g., sensing tasks), the devices must return to the initial locations after their energy are replenished. The moving cost of group $c o _ { j }$ can be calculated by 

$$
m c (c o _ {j}) = 2 \sum_ {o _ {i} \in c o _ {j}} b _ {i} r (s _ {j}, o _ {i})\tag{4}
$$

We define the comprehensive cost of group co<sub>j</sub> as the sum of charging cost and moving cost 

$$
\begin{array}{l} c (c o _ {j}) = c c (c o _ {j}) + m c (c o _ {j}) \\ = a _ {j} \max _ {o _ {i} \in c o _ {j}} \frac {E _ {i}}{P r (s _ {j} , o _ {i})} + 2 \sum_ {o _ {i} \in c o _ {j}} b _ {i} r (s _ {j}, o _ {i}) \end{array}\tag{5}
$$

## B. Problem Formulation

The problem is to schedule the mobile rechargeable devices to the chargers with objective of minimizing the total comprehensive cost of all charging groups such that each device is assigned to exactly one charger. We refer to this problem as Cooperative Charging Scheduling (CCS) problem, which can be formulated as follows: 

$$
(C C S): \quad \min \sum_ {s _ {j} \in M} c (c o _ {j})\tag{6}
$$


TABLE I FREQUENTLY USED NOTATIONS


<table><tr><td>Symbol</td><td>Description</td></tr><tr><td><eq>M, m</eq></td><td>Set of chargers, Number of chargers</td></tr><tr><td><eq>N, n</eq></td><td>Set of mobile rechargeable devices, Number of mobile rechargeable devices</td></tr><tr><td><eq>E_i</eq></td><td>Required energy of rechargeable device <eq>o_i</eq></td></tr><tr><td><eq>||s_j o_i||</eq></td><td>Initial distance between charger <eq>s_j</eq> and mobile rechargeable device <eq>o_i</eq></td></tr><tr><td><eq>a_j</eq></td><td>Unit charging price of charger <eq>s_j</eq></td></tr><tr><td><eq>d_j</eq></td><td>Charging distance of charger <eq>s_j</eq></td></tr><tr><td><eq>D_j</eq></td><td>Maximum charging distance of charger <eq>s_j</eq></td></tr><tr><td><eq>\alpha, \beta</eq></td><td>Charging parameters</td></tr><tr><td><eq>b_i</eq></td><td>Unit moving cost of device <eq>o_i</eq></td></tr><tr><td><eq>r(s_j, o_i)</eq></td><td>Moving distance of <eq>o_i</eq> for charging by <eq>s_j</eq></td></tr><tr><td><eq>co_j</eq></td><td>Charging group of devices that charged by <eq>s_j</eq></td></tr><tr><td><eq>cc(co_j)</eq></td><td>Charging cost of group <eq>co_j</eq></td></tr><tr><td><eq>cc_i (co_j)</eq></td><td>Charging cost of rechargeable device <eq>o_i</eq> in charging group <eq>co_j</eq></td></tr><tr><td><eq>mc(co_j)</eq></td><td>Moving cost of group <eq>co_j</eq></td></tr><tr><td><eq>c (co_j)</eq></td><td>Comprehensive cost of group <eq>co_j</eq></td></tr><tr><td><eq>\varepsilon</eq></td><td>Search precision</td></tr><tr><td><eq>z_i</eq></td><td>Strategy of rechargeable device <eq>o_i</eq></td></tr><tr><td><eq>z_{-i}</eq></td><td>Strategies of other rechargeable devices except <eq>o_i</eq></td></tr><tr><td><eq>\mathbf{Z}</eq></td><td>Strategy profile of all rechargeable devices</td></tr><tr><td><eq>\mathbf{CO}</eq></td><td>Coalition structure</td></tr><tr><td><eq>u_i</eq></td><td>Utility of rechargeable device <eq>o_i</eq></td></tr><tr><td><eq>\phi</eq></td><td>Potential function</td></tr></table>

$$
s. t. \quad \bigcup_ {s _ {j} \in M} c o _ {j} = N\tag{6-1}
$$

$$
c o _ {j} \cap c o _ {j ^ {\prime}} = \emptyset , \forall s _ {j} \neq s _ {j ^ {\prime}}, s _ {j} \in M, s _ {j ^ {\prime}} \in M\tag{6-2}
$$

The constraint (6-1) ensures that all rechargeable devices should be charged. The constraint (6-2) ensures that each rechargeable device can be scheduled to exact one charger. In view of the commercial feasibility of charging economy, we consider that each mobile rechargeable device only can obtain the charging service from one charger every time. 

We list the frequently used notations in Table I. 

## IV. INTRAGROUP COST SHARING SCHEME

As a paid charging service, we should determine the payment of each rechargeable device to the corresponding CSP for each charging group. On the other words, we need a cost sharing scheme to share the charging cost of each group. In the section, we introduce two cost sharing schemes. 

## A. Proportional Cost Sharing Scheme

We consider that the charging cost $c c _ { i } ( c o _ { j } )$ of any device $o _ { i } \in c o _ { j }$ in charging group $c o _ { j }$ is proportional to its required energy: 

$$
c c _ {i} (c o _ {j}) = c c (c o _ {j}) \frac {E _ {i}}{\sum_ {o _ {i ^ {\prime}} \in c o _ {j}} E _ {i ^ {\prime}}}\tag{7}
$$

Despite its simplicity, the proportional cost sharing scheme can achieve some desirable properties. 

Theorem 1. The proportional cost sharing scheme satisfies the following desirable properties: 

• For any $c o _ { j } , \sum _ { \mathrm { ~ \tiny ~ . ~ . ~ . ~ } } c c _ { i } ( c o _ { j } ) = c c ( c o _ { j } ) ;$ o ∈co 

• For any ${ c o _ { j } } ^ { \prime } \stackrel { \cdot } { \subseteq } { c o _ { j } } , \sum _ { o _ { i } \in { c o _ { j } } ^ { \prime } } { c c _ { i } } ( { c o _ { j } } ) \leq { c c } ( { c o _ { j } } ^ { \prime } ) ;$ 

• For any $c o _ { j } { } ^ { \prime } , \ c o _ { j } { } ^ { \prime \prime } \ \subseteq \ c o _ { j }$ and $o _ { i } \in c o _ { j } { ' } , c c _ { i } ( c o _ { j } { ' } ) \ge$ $c c _ { i } ( c o _ { j } ^ { \prime } \cup c o _ { j } ^ { \prime \prime } ) .$ 

The first property ensures that the summation of the individual charging cost is equal to the group charging cost for any group, i.e., the proportional cost sharing scheme satisfies budget balance. The second property ensures that no subset of devices can benefit by breaking away from the current charging group for any fixed charger. Thus, the cost sharing scheme based on energy proportion sustains cooperation among all devices at local charger in an economically stable manner. The last property captures the notion that the devices should not be penalized as the group grows, i.e., no device can benefit by breaking away from the charging group for any fixed charger. 

Obviously, the proportional cost sharing scheme satisfies all three properties since the devices in the same charging group can share the charging cost during the common charging time. 

## B. Shapley Value based Cost Sharing Scheme

Without loss of generality, the cost sharing for a cooperative game $( G , h )$ can be defined as: There is a set G of n players and a characteristic cost function that maps subsets of users to the real number: $h : 2 ^ { G } $ satisfying $h ( \emptyset ) = 0$ . The Shapley value, which is a widely used cost sharing scheme to share the total cost to the players fairly [36], takes a random ordering of the players picked uniformly from the set G of all n! possible orderings, and charging each player its expected marginal cost in this ordering. 

In the setting of cooperative charging, the characteristic function $h ( \cdot )$ represents the charging cost of any group $c o _ { j } \in$ CO in which the devices accept the cooperative charging service, $\mathbf { i . e . , } h ( \cdot ) = c c ( \cdot )$ . Therefore, we can caculate the cost share of device $o _ { i } \in c o _ { j }$ in given cooperative game $( c o _ { j } , c c )$ for each charging group $c o _ { j }$ 

$$
c c _ {i} (c o _ {j}) = \sum_ {Q \subseteq c o _ {j} \setminus \{o _ {i} \}} \frac {| Q | ! (| c o _ {j} | - 1 - | Q |) !}{| c o _ {j} | !} (c c (Q \cup \{o _ {i} \}) - c c (Q))\tag{8}
$$

Note that the Shapley value satisfies many desirable properties, such as efficiency, symmetry, linearity, anonymity and dummy [36] [37]. 

## V. COOPERATIVE CHARGING SCHEDULING AS AN OPTIMIZATION PROBLEM

In this section, we present Cooperative Charging Scheduling Algorithm (CCSA) based on greedy approach and submodular function minimization [20] to solve the CCS problem. 

## A. Hardness

First, we attempt to find an optimal algorithm for the CCS problem. Unfortunately, as the following theorem shows, the CCS problem is NP-hard. 

Theorem 2. The CCS problem is NP-hard. 

Proof: Our CCS problem defined in (6) is equivalent to the Generalized Facility Location Problem $( G F L P )$ : There are a set M of facilities and a set N of clients. The connection cost of any $o _ { i } \in N$ to any facility $s _ { j } ~ \in ~ M$ is $2 b _ { i } r ( s _ { j } , o _ { i } )$ . The facility cost of any facility $s _ { j } \in M$ is $c c ( c o _ { j } )$ . The objective is to find an assignment of each client to an open facility to minimize the total cost incurred. The GFLP can be formulated as follows: 

$$
(G F L P): \quad \min \sum_ {s _ {j} \in M} c c (c o _ {j}) y _ {j} + \sum_ {s _ {j} \in M} \sum_ {o _ {i} \in N} 2 b _ {i} r (s _ {j}, o _ {i}) x _ {i j}\tag{9}
$$

$$
s. t. \quad \sum_ {s _ {j} \in M} x _ {i j} = 1, \forall o _ {i} \in N\tag{9-1}
$$

$$
x _ {i j} \leq y _ {j}, \forall s _ {j} \in M, \forall o _ {i} \in N\tag{9-2}
$$

$$
x _ {i j} \in \{0, 1 \}, \forall s _ {j} \in M, \forall o _ {i} \in N\tag{9-3}
$$

$$
y _ {j} \in \{0, 1 \}, \forall s _ {j} \in M\tag{9-4}
$$

where $y _ { j }$ is a binary variate indicating whether facility $s _ { j }$ is open. $\boldsymbol { x } _ { i j }$ is a binary variate indicating whether client $o _ { i }$ is assigned to facility $s _ { j }$ 

If $c c ( c o _ { j } )$ is a constant for each $s _ { j } ~ \in ~ M$ , the problem defined in (9) is simplified to the standard Facility Location Problem (FLP) [19]. In the scenario of cooperative charging, $c c ( c o _ { j } )$ is related to the devices assigned to the facility $s _ { j }$ and cannot be known in advance. Since the FLP is NP-hard, the CCS problem is NP-hard.  

## B. Design Rationale

Since the CCS problem is NP-hard, it is impossible to compute the optimal solution in polynomial time. We turn our attention to the approximation algorithm design. 

We give the following definition. 

Definition 1. (Nonnegativity, Monotonicity, and Submodularity) Given a finite ground set N, a real-valued set function defined asc $: 2 ^ { N }  \mathbb { R } ,$ , c is called nonnegative, monotone (nondecreasing), and submodular if and only if it satisfies following conditions: 

$c ( \emptyset ) = 0$ and $c ( A ) \geq 0$ for all $A \subseteq N$ (nonnegative); 

$c ( A ) \leq c ( B )$ for all $A \subseteq B \subseteq N$ (monotone); 

$c ( A \cup \{ e \} ) - c ( A ) \geq c ( B \cup \{ e \} ) - c ( B )$ , for all $A \subseteq$ $B \subseteq N , e \in N \backslash B$ (submodular). 

We have the following theorem. 

Theorem 3. The objective function of CCS problem is nonnegative, monotone and submodular. 

Proof: Obviously, the objective function of CCS problem is nonnegative and monotone. Since the summation of multiple submodular functions is also a submodular function. We next prove that $c ( \cdot )$ is a submodular function. 

Given any two charging groups $c o _ { j } \subseteq \ c o _ { j } ^ { \prime }$ with same charger $s _ { j } ,$ , there must be 

$$
\max _ {o _ {i} \in c o _ {j}} \frac {E _ {i}}{P r (s _ {j} , o _ {i})} \leq \max _ {o _ {i} \in c o _ {j ^ {\prime}}} \frac {E _ {i}}{P r (s _ {j} , o _ {i})}\tag{10}
$$

Given any mobile rechargeable device $o _ { e } ,$ 

$$
\begin{array}{l} c \left(c o _ {j} \cup \left\{o _ {e} \right\}\right) - c \left(c o _ {j}\right) \\ = \left(a _ {j} \max _ {o _ {i} \in c o _ {j} \cup \left\{o _ {e} \right\}} \frac {E _ {i}}{P r \left(s _ {j} , o _ {i}\right)} + 2 \sum_ {o _ {i} \in c o _ {j} \cup \left\{o _ {e} \right\}} b _ {i} r \left(s _ {j}, o _ {i}\right)\right) \\ - \left(a _ {j} \max _ {o _ {i} \in c o _ {j}} \frac {E _ {i}}{P r \left(s _ {j} , o _ {i}\right)} + 2 \sum_ {o _ {i} \in c o _ {j}} b _ {i} r \left(s _ {j}, o _ {i}\right)\right) \\ = a _ {j} \left(\max _ {o _ {i} \in c o _ {j} \cup \left\{o _ {e} \right\}} \frac {E _ {i}}{P r \left(s _ {j} , o _ {i}\right)} - \max _ {o _ {i} \in c o _ {j}} \frac {E _ {i}}{P r \left(s _ {j} , o _ {i}\right)}\right) + 2 b _ {e} r \left(s _ {j}, o _ {e}\right) \end{array} \tag {11}\tag{11}
$$

Similarly, we have 

$$
\begin{array}{l} c \left(c o _ {j} ^ {\prime} \cup \left\{o _ {e} \right\}\right) - c \left(c o _ {j} ^ {\prime}\right) \\ = a _ {j} \left(\max _ {o _ {i} \in c o _ {j} ^ {\prime} \cup \left\{o _ {e} \right\}} \frac {E _ {i}}{P r \left(s _ {j} , o _ {i}\right)} - \max _ {o _ {i} \in c o _ {j} ^ {\prime}} \frac {E _ {i}}{P r \left(s _ {j} , o _ {i}\right)}\right) + 2 b _ {e} r \left(s _ {j}, o _ {e}\right) \end{array} \tag {13}\tag{12}
$$

Now, we consider the following two cases: 

Case 1: $\frac { E _ { e } } { P r ( s _ { j } , o _ { e } ) } \ \leq \ \operatorname* { m a x } _ { i \in c o _ { j } } \frac { E _ { i } } { P r ( s _ { j } , o _ { i } ) } ,$ i.e., the introduction of rechargeable device $o _ { e }$ does not change the maximum charging time of $c o _ { j } .$ In this case, we have $\operatorname* { m a x } _ { \substack { o _ { i } \in c o _ { j } \cup \{ o _ { e } \} } } \frac { E _ { i } } { P r ( s _ { j } , o _ { i } ) } { = \operatorname* { m a x } _ { o _ { i } \in c o _ { j } } \frac { E _ { i } } { P r ( s _ { j } , o _ { i } ) } }$ , therefore, $c \left( c o _ { j } \cup \{ o _ { e } \} \right) - c \left( c o _ { j } \right) ^ { \circ } = 2 b _ { e } r ( s _ { j } , o _ { e } )$ 

According to (10), we also have $\begin{array} { r l } { \frac { E _ { e } } { P r ( s _ { j } , o _ { e } ) } } & { { } \leq } \end{array}$ $\operatorname* { m a x } _ { i \in c o _ { j } ^ { \prime } } \frac { E _ { i } } { P r ( s _ { j } , o _ { i } ) } ,$ i.e., the introduction of rechargeable device $o _ { e }$ does not change the maximum charging time of $c o _ { j } ^ { \prime }$ too. In this case, we have $c \left( c o _ { j } ^ { \prime } \cup \{ o _ { e } \} \right) - c \left( c o _ { j } ^ { \prime } \right) = 2 b _ { e } r ( \check { s } _ { j } , o _ { e } )$ 

As a result, we have 

$$
c \left(c o _ {j} \cup \left\{o _ {e} \right\}\right) - c \left(c o _ {j}\right) = c \left(c o _ {j} ^ {\prime} \cup \left\{o _ {e} \right\}\right) - c \left(c o _ {j} ^ {\prime}\right).
$$

Case $2 \colon \frac { E _ { e } } { P r ( s _ { j } , o _ { e } ) } > \operatorname* { m a x } _ { i \in c o _ { j } } \frac { E _ { i } } { P r ( s _ { j } , o _ { i } ) } ,$ i.e., the introduce of rechargeable device $o _ { e }$ changes the maximum charging time of $c o _ { j }$ . In this case, we have 

$$
\max _ {o _ {i} \in c o _ {j} \cup \{o _ {e} \}} \frac {E _ {i}}{P r (s _ {j} , o _ {i})} - \max _ {o _ {i} \in c o _ {j}} \frac {E _ {i}}{P r (s _ {j} , o _ {i})} = \frac {E _ {e}}{P r (s _ {j} , o _ {e})}
$$

Based on (11), we have 

$$
c \left(c o _ {j} \cup \{o _ {e} \}\right) - c \left(c o _ {j}\right) = a _ {j} \frac {E _ {e}}{P r (s _ {j} , o _ {e})} + 2 b _ {e} r (s _ {j}, o _ {e}).
$$

We further discuss the following two cases: 

Case 2.1: $\frac { E _ { e } } { P r ( s _ { j } , o _ { e } ) } > \operatorname* { m a x } _ { i \in c o _ { j } \prime } \frac { E _ { i } } { P r ( s _ { j } , o _ { i } ) }$ , i.e., the introduction of rechargeable device $o _ { e }$ changes the maximum charging time of $c o _ { j } ^ { \prime }$ . Based on (12), we have 

$$
\begin{array}{l} c (c o _ {j} ^ {\prime} \cup \{o _ {e} \}) - c (c o _ {j} ^ {\prime}) = a _ {j} \frac {E _ {e}}{P r (s _ {j} , o _ {e})} + 2 b _ {e} r (s _ {j}, o _ {e}) \\ = c (c o _ {j} \cup \{o _ {e} \}) - c (c o _ {j}) \end{array}
$$

Case 2.2: $\frac { E _ { e } } { P r ( s _ { j } , o _ { e } ) } \leq \operatorname* { m a x } _ { i \in c o _ { j } \prime } \frac { E _ { i } } { P r ( s _ { j } , o _ { i } ) } ,$ , i.e., the introduction of rechargeable device $o _ { e }$ does not change the maximum charging time of $c o _ { j } ^ { \prime }$ . Based on (12), we have 

$$
\begin{array}{r l} & c (c o _ {j} ^ {\prime} \cup \{o _ {e} \}) - c (c o _ {j} ^ {\prime}) = 2 b _ {e} r (s _ {j}, o _ {e}) \\ & <   c (c o _ {j} \cup \{o _ {e} \}) - c (c o _ {j}) \end{array}
$$

As a result of Case 2, we have 

$$
c \left(c o _ {j} \cup \{o _ {e} \}\right) - c \left(c o _ {j}\right) \leq c \left(c o _ {j} ^ {\prime} \cup \{o _ {e} \}\right) - c \left(c o _ {j} ^ {\prime}\right)
$$

Therefore, the objective function is submodular. 

For our CCS problem, we repeatedly select an unassigned device set to a charger to minimize the ratio of the marginal comprehensive cost to the number of newly covered devices (termed comprehensive cost effectiveness). However, the unassigned device set can be any subset of all unassigned sensors, therefore, the number of possible unassigned device sets is exponential. To solve this problem, we use submodular function minimization [20] to find the unassigned device set with the best comprehensive cost effectiveness in polynomial time. 

We find the best set $F _ { j }$ for each charger $s _ { j } ,$ , and then select the one with the best comprehensive cost effectiveness. To minimize this ratio, we can do a binary search to find the minimum value of λ for which there exists a set $F$ such that $\underline { { c ( c o _ { j } \cup F ) - c ( c o _ { j } ) } } _ { < } ~ _ { \lambda }$ ≤ λ, i.e., 

$$
c (c o _ {j} \cup F) - c (c o _ {j}) - | F | \lambda \leq 0\tag{13}
$$

The left-hand side of (13) is a submodular function. This is because the function $c ( c o _ { j } \cup F )$ is submodular based on Theorem 3. The second term $c ( c o _ { j } )$ is a constant when finding set F . The last term is a modular function. Thus, it can be minimized in polynomial time. 

## C. Algorithm Design

Let $\mathbf { C O } = ( c o _ { 1 } , c o _ { 2 } , . . . , c o _ { m } )$ be the all charging groups. As illustrated in Algorithm 1, we call the function $\mathbf { B } \mathbf { S } ( \cdot )$ (line 7) to find the unassigned rechargeable device set $F _ { j }$ for each $s _ { j }$ from residual unassigned rechargeable device set $N ^ { \prime }$ that can minimize the comprehensive cost effectiveness over $\boldsymbol { s } _ { j } { } ^ { \ , } { } \boldsymbol { \mathbf { s } }$ current charging group co<sub>j</sub>. Then we find the rechargeable device set co with the smallest comprehensive cost effectiveness (line 9). Then the unassigned rechargeable device set $F _ { j }$ is merged into $c o _ { j }$ (line 10). The iteration terminates when all devices are assigned. 

We execute the binary search by calling function $\mathbf { B } \mathbf { S } ( \cdot )$ illustrated in Algorithm 2. Let low and high be the lower bound and upper bound of λ, respectively. We set $h i g h =$ $c ( c o _ { j } \cup N ^ { \prime } ) \stackrel { \sim } { - } c ( c o _ { j } )$ initially (Line 1). This is because $F =$ $\overline { { N ^ { \prime } \left. ( \mathrm { a s s i g n } \mathrm { a l } \right. } }$ l unassigned devices to $c o _ { j } )$ is a feasible solution of minimizing $\frac { c ( c o _ { j } \mathsf { \bar { U } } F ) - c ( c o _ { j } ) } { | F | } . \mathsf { S o } \frac { c ( c o _ { j } \cup N ^ { \prime } ) - c ( c o _ { j } ) } { | N ^ { \prime } | }$ is an upper bound of the value of $\frac { c ( c o _ { j } \cup S ^ { * } ) - c ( c o _ { j } ) } { | S ^ { * } | }$ indeed, where $S ^ { * }$ is the optimal solution of set F . We use the binary search (Lines 2-13) to find the set F for $c o _ { j }$ until the value of $( \frac { c ( c o _ { j } \cup F ) - c ( c o _ { j } ) } { | F | } - m i d )$ satisfies the search precision $\varepsilon \in ( 0 , 1 ) , \ \mathrm { i . e . , } \ | \frac { c ( c o _ { j } \cup F ) - c ( c o _ { j } ) } { | F | } - m i d | \leq \varepsilon$ (Line 4). In each iteration of binary search, we use submodular function minimization to compute the minimum of $( c ( c o _ { j } \cup F ) - c ( c o _ { j } )$ − mid $F | )$ (Line 3). 

Algorithm 1 : CCSA
Input: N, M, $E_{i}$ , $b_{i}$ , $a_{j}$ , $d_{j}$ , $||s_{j}o_{i}||$ , $\alpha_{s_{j}o_{i}}$ , $\beta_{s_{j}o_{i}}$ , $\forall o_{i} \in N, \forall s_{j} \in M$ 1: foreach $s_{j} \in M$ do
2: $co_{j} \leftarrow \emptyset;$ 3: end
4: $N' \leftarrow N; CO \leftarrow (co_{1}, co_{2}, ..., co_{m});$ 5: while $N' \neq \emptyset$ do
6: foreach $s_{j} \in M$ do
7: $F_{j} \leftarrow BS(s_{j}, co_{j}, N')$ ;
8: end
9: $s_{j} \leftarrow \arg\min_{s_{j'} \in M} \frac{c(co_{j'} \cup F_{j'}) - c(co_{j'})}{|F_{j'}|};$ 10: $co_{j} \leftarrow co_{j} \cup F_{j}; N' \leftarrow N'\backslash F_{j};$ 11: end
12: return CO; 

Theorem 4. The time complexity of CCSA is $O ( m n ^ { 8 } \log n \log { \frac { n } { \varepsilon } } ) .$ 

Proof: We first analyze the time complexity of BS(·) (Algorithm 2). The binary search with search precision ε takes $O ( \log { \frac { n } { \varepsilon } } )$ time. If we use the strongly polynomial algorithm proposed in reference [20], the time complexity of minimizing submodular function (Line 3) is $O ( n ^ { 7 } \log n )$ Thus, the running time of Algorithm 2 is $O ( n ^ { 7 } \log n \log { \frac { n } { \varepsilon } } )$ CCSA (Algorithm 1) is dominated by finding the unassigned rechargeable device set $F _ { j }$ for all $s _ { j } \in M$ (Line 7), which takes $O ( m n ^ { 7 } \log n \log { \frac { n } { \varepsilon } } )$ . The while loop (Lines 5-11) is executed at most n times since there are n rechargeable devices and each iteration of the loop will cover at least one device. Thus, the running time of CCSA is $O ( m n ^ { 8 } \log n \log \frac { n } { \varepsilon } )$  

Theorem 5. CCSA is a $( { \frac { \ln n + 1 } { 1 - \varepsilon } } )$ -approximate algorithm of the CCS problem. 

Proof: We number the rechargeable devices of N in the order in which they were covered by CCSA resolving ties arbitrarily. Let $o _ { 1 } , o _ { 2 } , . . . , o _ { n }$ be this numbering. Assume $o _ { k } , k =$ $1 , 2 , . . . , n$ is covered by set $F _ { j }$ of charger $s _ { j }$ when the charging group is $c o _ { j }$ . Then the comprehensive cost effectiveness of $o _ { k }$ is 

$$
c o s t (o _ {k}) = \frac {c (c o _ {j} \cup F _ {j}) - c (c o _ {j})}{| F _ {j} |}\tag{14}
$$

Algorithm 2 : BS(·)
Input: charger $s_j$ , charging group $co_j$ , residual unassigned rechargeable device set $N'$ 1: low ← 0; high ← $\frac{c(co_j \cup N') - c(co_j)}{|N'|}$ ; mid ← $\frac{low + high}{2}$ ;
2: while (1) do
3:    F ← arg $\min_{F' \subseteq N', F' \neq \emptyset} (c(co_j \cup F') - c(co_j) - mid|F')$ ;
4:    if $|\frac{c(co_j \cup F) - c(co_j)}{|F|} - mid| \leq \varepsilon$ then
5:    return F;
6:    end
7:    if $c(co_j \cup F) - c(co_j) - mid|F| \leq 0$ then
8:    high ← mid;
9:    else
10:    low ← mid;
11:    end
12:    mid ← $\frac{low + high}{2}$ ;
13: end 

Let OP T be the optimal comprehensive cost of CCS problem. Consider the iteration in which $o _ { k }$ was covered, the charging groups of optimal solution can cover the remaining rechargeable devices in $N ^ { \prime }$ with comprehensive cost at most $O P T$ . Therefore, among all charging groups in the optimal solution, there must be one having comprehensive cost effectiveness at most $O P T / | N ^ { \prime } |$ , where $| N ^ { \prime } | \geq n - k + 1$ Since $o _ { k }$ was covered by set $F _ { j }$ of charger $s _ { j }$ with minimum comprehensive cost effectiveness in this iteration, it follows 

$$
c o s t (o _ {k}) \leq \frac {O P T}{| N ^ {\prime} |} \leq \frac {O P T}{n - k + 1}\tag{15}
$$

Since the comprehensive cost of each charging group is distributed among the new rechargeable devices covered, the total comprehensive cost of the charging groups obtained by CCSA is equal to $\sum _ { k = 1 } ^ { n } c o s t ( o _ { k } ) \leq \sum _ { k = 1 } ^ { n } \frac { \overbrace { O P T } } { n - k + 1 } = ( 1 + \frac { 1 } { 2 } +$ $\ldots + { \frac { 1 } { n } } ) O P T \leq ( \ln n + 1 ) O P T .$ 

Thus, CCSA is $( \ln n + 1 )$ -approximate if it can find the optimal solution to minimize the comprehensive cost effectiveness for any charging group. Considering the search precision $\varepsilon \in ( 0 , 1 )$ , the binary search approximates the optimal comprehensive cost effectiveness within a factor of $1 / ( 1 - \varepsilon )$ Thus, CCSA is $( { \frac { \ln n + 1 } { 1 - \varepsilon } } )$ -approximate.  

## VI. COOPERATIVE CHARGING SCHEDULING AS A COOPERATIVE GAME

Although CCSA is a polynomial algorithm, As shown in Theorem 4, CCSA still incurs high computing cost and is inefficient for the large-scale WRSNs. In this section, we formulate the CCS problem as a coalition formation game [38], termed CCS Game, which can improve the solution gradually. 

We propose the CCS Game Algorithm (CCSGA) to solve this problem. We will show that CCSGA is much faster than CCSA based on our simulation results. 

## A. Cooperative Charging Scheduling Game

We model the cooperative charging scheduling as a coalition formation game $\varphi ~ = ~ \{ N , u , { \bf Z } , { \bf C O } \}$ , where N is the rechargeable device set, and u is the utility function. For any rechargeable device $o _ { i } \in N$ , the strategy of $o _ { i }$ is denoted by $z _ { i } ,$ and the corresponding coalition is denoted by $c o _ { z _ { i } }$ . The other rechargeable devices’ strategies are denoted by $z _ { - i } .$ . Let $\mathbf { Z } = ( z _ { 1 } , z _ { 2 } , . . . , z _ { n } )$ be the strategy profile of all rechargeable devices. $\mathbf { C O } = ( c o _ { 1 } , c o _ { 2 } , . . . , c o _ { m } )$ is the coalition structure without overlap. Since all devices should be assigned and the comprehensive cost function is monotone, the coalition structure CO is a coalition partition of rechargeable device set $N .$ 

First, we define the comprehensive cost of any device $o _ { i } \in$ $N$ in coalition as $c _ { i } \big ( c o _ { j } \big )$ , which is the sum of charging cost and moving cost of device o<sub>i</sub>: 

$$
c _ {i} (c o _ {j}) = c c _ {i} (c o _ {j}) + 2 b _ {i} r (s _ {j}, o _ {i})\tag{16}
$$

Definition 2. (Preference Order): The preference order $\succ _ { i }$ for any rechargeable device $o _ { i } \in N$ is defined as a complete, reflexive, and transitive binary relation over the set of all feasible coalitions that rechargeable device $o _ { i }$ can possibly form. 

A rechargeable device decides to join or leave a coalition based on the preference order. For example, for two coalitions $c o _ { j } , c o _ { j ^ { \prime } } \in O ,$ the rechargeable device $o _ { i }$ will choose to join coalition $c o _ { j }$ rather than $c o _ { j ^ { \prime } }$ if $c o _ { j } \succ i \ c o _ { j ^ { \prime } }$ . The preference order will affect the final coalition structure and convergence. Since the objective is to minimize the comprehensive cost of whole coalition structure, we consider the coalition order defined in Definition 3. 

Definition 3. (Coalition Order): For each rechargeable deviceo<sub>i</sub> $\in \textit { N }$ and any two coalition co<sub>j</sub> and co<sub>j</sub>0 , $j \neq j ^ { \prime } ,$ we say that: 

$$
\begin{array}{l} c o _ {j} \succ_ {i} c o _ {j ^ {\prime}} \Leftrightarrow \sum_ {k \in c o _ {j}} c _ {k} (c o _ {j}) - \sum_ {k \in c o _ {j} \cup \{o _ {i} \}} c _ {k} (c o _ {j} \cup \{o _ {i} \}) \\ > \sum_ {k \in c o _ {j ^ {\prime}}} c _ {k} (c o _ {j ^ {\prime}}) - \sum_ {k \in c o _ {j ^ {\prime}} \cup \{o _ {i} \}} c _ {k} (c o _ {j ^ {\prime}} \cup \{o _ {i} \}) \end{array}\tag{17}
$$

This coalition order means that the device prefers the coalition with the minimum increase of comprehensive cost. This preference order cares about the comprehensive cost of whole coalition partition. 

The utility function of any rechargeable device $o _ { i } \in N$ is defined as: 

$$
u _ {i} (z _ {i}, z _ {- i}) = \sum_ {k \in c o _ {z _ {i}}} c _ {k} (c o _ {z _ {i}}) - \sum_ {k \in c o _ {z _ {i}} \cup \{o _ {i} \}} c _ {k} (c o _ {z _ {i}} \cup \{o _ {i} \})\tag{18}
$$

The utility represents the change of comprehensive cost of all rechargeable devices in the coalition $c o _ { z _ { i } }$ due to the join of rechargeable device $o _ { i }$ . Given the other rechargeable devices’ strategies $z _ { - i } ,$ the device $o _ { i }$ always tends to join the coalition with the minimal increase of comprehensive cost. 

Algorithm 3 : CCSGA
Input: N, M, $E_{i}$ , $b_{i}$ , $a_{j}$ , $d_{j}$ , $||s_{j}o_{i}||$ , $\alpha_{s_{j}o_{i}}$ , $\beta_{s_{j}o_{i}}, \forall o_{i} \in N, \forall s_{j} \in M$ 1: foreach $o_{i} \in N$ do
2: assign $o_{i}$ to the charging group with minimal comprehensive cost;
3: end
4: do
5: given other devices' strategies $z_{-i}$ , each device $o_{i} \in N$ chooses the charging group with maximum utility;
6: until (the strategy profile is convergent)
7: return coalition structure CO; 

## B. Algorithm Design and Analysis

We propose a CCS Game Algorithm (CCSGA), in which the rechargeable devices form the disjoint coalitions by strategical charging group selection. At the beginning, all rechargeable devices choose the charging groups with minimal comprehensive cost, and form the initial coalition structure. Then CCSGA follows the best-response dynamics, where the devices only choose the best response that would give them the highest utility [38]. At each round of iterations, each device chooses a charging group to maximize its utility, and the device leaves the current charging group and joins to the selected charging group. Repeat this process until no rechargeable device can improve the utility by changing the coalition selection unilat erally. We will show that CCSGA finally converges to a pure Nash Equilibrium (NE). 

We introduce some closely related definitions about coalition formation game. 

Definition 4. (Nash Equilibrium): A set of strategies $\mathbf { Z } ^ { \ast } =$ $\left( z _ { 1 } ^ { * } , z _ { 2 } ^ { * } , . . . , z _ { n } ^ { * } \right)$ is a Nash Equilibrium if for every rechargeable device $o _ { i } \in N$ and each its alternate strategy $z _ { i } ,$ 

$$
u _ {i} (z _ {i} ^ {*}, z _ {- i} ^ {*}) \geq u _ {i} (z _ {i}, z _ {- i} ^ {*})
$$

Definition 5. (Nash-stable Coalition Structure): The corresponding coalition structure CO<sup>∗</sup> of Nash Equilibrium $\mathbf { Z } ^ { * } = ( z _ { 1 } ^ { * } , z _ { 2 } ^ { * } , . . . , z _ { n } ^ { * } )$ is called Nash-stable Coalition Structure. 

Definition 6. (Exact Potential Game): The game is an exact potential game if and only if there exists a potential function $\phi ( a _ { i } , a _ { - i } ) , \forall i \in N$ such that: 

$$
\phi (z _ {i}, z _ {- i}) - \phi (z _ {i} ^ {\prime}, z _ {- i}) = u _ {i} (z _ {i}, z _ {- i}) - u _ {i} (z _ {i} ^ {\prime}, z _ {- i}), \forall z _ {i}, z _ {i} ^ {\prime} \in M
$$

Theorem 6. CCS Game has at least one Nash Equilibrium, and CCSGA finally converges to a Nash-stable Coalition Structure. 

Proof: The utility change of any device $o _ { i } \in N$ from $z _ { i }$ to $z ^ { \prime } { } _ { i }$ is: 

$$
\begin{array}{r l} & u _ {i} (z _ {i}, z _ {- i}) - u _ {i} (z _ {i} ^ {\prime}, z _ {- i}) \\ & = \sum_ {k \in c o _ {z _ {i}}} c _ {k} (c o _ {z _ {i}}) - \sum_ {k \in c o _ {z _ {i}} \cup \{o _ {i} \}} c _ {k} (c o _ {z _ {i}} \cup \{o _ {i} \}) \\ & - (\sum_ {k \in c o _ {z _ {i} ^ {\prime}}} c _ {k} (c o _ {z _ {i} ^ {\prime}}) - \sum_ {k \in c o _ {z _ {i} ^ {\prime}} \cup \{o _ {i} \}} c _ {k} (c o _ {z _ {i} ^ {\prime}} \cup \{o _ {i} \})) \end{array}\tag{19}
$$

We define the potential function $\phi$ as the opposite of sum of all devices’ comprehensive cost: 

$$
\phi (z _ {i}, z _ {- i}) = - \sum_ {z _ {k} \in M} \sum_ {k \in c o _ {z _ {k}}} c _ {k} (c o _ {z _ {k}})\tag{20}
$$

Since the strategy change of device $o _ { i }$ only affects the devices in $c o _ { z _ { i } }$ and $c o _ { z ^ { \prime } , i }$ , the change of potential function due to its unilateral change is given by: 

$$
\begin{array}{l} \phi (z _ {i}, z _ {- i}) - \phi (z _ {i} ^ {\prime}, z _ {- i}) \\ = - (\sum_ {k \in c o _ {z _ {i}} \cup \{o _ {i} \}} c _ {k} (c o _ {z _ {i}} \cup \{o _ {i} \}) - \sum_ {k \in c o _ {z _ {i}}} c _ {k} (c o _ {a _ {i}})) \\ - (\sum_ {k \in c o _ {z _ {i} ^ {\prime}}} c _ {k} (c o _ {z _ {i} ^ {\prime}}) - \sum_ {k \in c o _ {z _ {i} ^ {\prime}} \cup \{o _ {i} \}} c _ {k} (c o _ {z _ {i} ^ {\prime}} \cup \{o _ {i} \})) \\ = u _ {i} (z _ {i}, z _ {- i}) - u _ {i} (z _ {i} ^ {\prime}, z _ {- i}) \end{array}\tag{21}
$$

We can see from (19) and (21) that the change in total utility function caused by any device’s unilateral deviation is the same as the change in the potential function. Thus, according to the Definition $^ { 6 , }$ the CCS Game is an exact potential game, which has at least one pure NE. 

Based on the Lemma 2.3 of [39], every exact potential game with finite strategy sets has the Finite Improvement Property (FIP); that is, unilateral improvement dynamics is guaranteed to converge to a pure NE in a finite number of steps. Thus, CCSGA finally converges to a Nash-stable Coalition Structure. 

Note that since CCSGA aims to maximize the potential function, which is the opposite of sum of all devices’ comprehensive cost, CCSGA indeed minimizes the total comprehensive cost of all devices gradually. 

## VII. SIMULATION RESULTS

In this section, we perform simulations to evaluate the performance of CCSA and CCSGA. 

## A. Simulation Setup

Since there is no off-the-shelf rechargeable device scheduling for the cooperative charging service so far, we compare our solutions with following two naive scheduling algorithms: 


TABLE II



DEFAULT SETTINGS OF PARAMETERS


<table><tr><td>Parameter</td><td>Default value</td></tr><tr><td><eq>\Omega</eq></td><td>200m*200m</td></tr><tr><td><eq>m</eq></td><td>50</td></tr><tr><td><eq>n</eq></td><td>200</td></tr><tr><td><eq>E_{i}</eq></td><td>[10 J, 20 J]</td></tr><tr><td><eq>a_{j}</eq></td><td>[100, 150] per hour</td></tr><tr><td><eq>b_{i}</eq></td><td>[10, 12] per meter</td></tr><tr><td><eq>d_{j}</eq></td><td>0.9m</td></tr><tr><td><eq>\alpha, \beta</eq></td><td>10000, 40</td></tr><tr><td><eq>\varepsilon</eq></td><td>0.01</td></tr><tr><td>Iterations</td><td>1000</td></tr></table>

![](images/6423a2199dad03d2233a6a59699df6a9e95131faaa99a67ce356d6a9bb978227.jpg)



Fig. 2. Impact of number of chargers


![](images/debd7f809e86d271dcadedfcaac63a88f84055bca8650015bcf034b816a903e9.jpg)



Fig. 3. Impact of number of rechargeable devices


![](images/afef6579d8995a9ad9723e82ca836e8758b7324b1ad536a518ca5962fd576754.jpg)



Fig. 4. Impact of unit moving cost


![](images/dd66dc652c0cc2fc5fcaec8c0b65c43721a01b05cf8113ee2c4df5b34f9c9862.jpg)



Fig. 5. Impact of search precision on CCSA


![](images/6a2cdefb1a627aacdf20e77e60d18b7df291d6544d8051bd31174c6c3718bdc5.jpg)



Fig. 6. Impact of iterations on CCSGA


![](images/dbf9301a610c6471cd32fff4ce45455be88832757fee3e0b7bb45cf13a07c5e2.jpg)



Fig. 7. Comparison with optimal solution


• BC (Best Cooperation): Each device chooses the best charger according to the comprehensive cost independently and calculates the comprehensive cost in the same way as CCSA. 

• BN (Best Noncooperation): Each device chooses the best charger according to the comprehensive cost independently and pays the charging cost independently. 

Note that the scheduling strategy of BC and BN has been widely applied in the mobile charger scheduling algorithms by choosing the best position to the mobile charger as the next visiting point [25] [26]. 

For the simulations, we uniformly distribute chargers and devices in a 2D plane. The default values of parameters are given in Table II. The unit of power is watt. We will vary the value of the key parameters to explore the impacts on designed algorithms. All the simulations are run on a Windows machine with Intel(R) Xeon(R) CPU E5-2603 v2 and 16 GB memory. Each measurement is averaged over 100 instances. 

## B. Cost

We first increase the number of chargers from 50 to 110. As shown in Fig. 2, the average comprehensive cost of all algorithms decreases slightly with the increasing number of chargers. This is because with more chargers, the devices can move to closer chargers and the moving cost will decrease. We can see from Fig. 2 that the average comprehensive cost of cooperation algorithms is much lower than those of noncooperation algorithm. Averagely, CCSA can reduce the average comprehensive cost by 27.3% and 6.7% compared with BN and BC, respectively. Note that BC also calculates the comprehensive cost in the way of cooperation. The performance of CCSGA is very close to CCSA. The average comprehensive cost of CCSGA is 5.7% higher than that of CCSA on average. 

To test the scalability of proposed algorithms, we increase the number of devices from 200 to 500. As shown in Fig 3, the average comprehensive cost of cooperation algorithms decreases significantly with the increasing number of devices. This is because the number of cooperative devices assigned to each charger increases averagely, increasing the cooperative surplus in each charging group. However, BN pays the charging cost independently and the average comprehensive cost of BN does not change with the increasing number of devices. Furthermore, we can see that CCSA and CCSGA show more advantages in the large-scale wireless charging system. 


TABLE III



RUNNING TIME OF CCSA, CCSGA AND OPTIMAL SOLUTION


<table><tr><td>Number of rechargeable devices</td><td>CCSA (ms)</td><td>CCSGA (ms)</td><td>Optimal solution (ms)</td></tr><tr><td>5</td><td>4.12</td><td>1.21</td><td>1.78</td></tr><tr><td>6</td><td>6.29</td><td>1.43</td><td>11.42</td></tr><tr><td>7</td><td>10.07</td><td>1.51</td><td>234.12</td></tr><tr><td>8</td><td>15.77</td><td>1.83</td><td>2724.64</td></tr><tr><td>9</td><td>20.73</td><td>2.43</td><td>13980.35</td></tr><tr><td>10</td><td>41.23</td><td>3.35</td><td>168973.67</td></tr><tr><td>11</td><td>109.12</td><td>4.45</td><td>6907320.58</td></tr><tr><td>100</td><td>13534.65</td><td>10.98</td><td></td></tr><tr><td>150</td><td>26453.56</td><td>22.22</td><td></td></tr><tr><td>200</td><td>178212.90</td><td>92.15</td><td></td></tr></table>

Then we vary the unit moving cost from range [10, 12] to [22, 24]. As shown in Fig. 4, the average comprehensive cost of all algorithms increases accordingly. This is because the moving cost increases when the unit moving cost increases. Overall, CCSA always outputs the lowest comprehensive cost. 

![](images/62b86fb9d162d41503509dff7ecd394170797e510d2f8d48c8ec8b1d8ae9eaf7.jpg)



Fig. 8. Testbed


![](images/657894523f607f20daadebb895b97f4c436f02a951f93b265fbf91ab08bc2114.jpg)



Fig. 9. Scheduling results


![](images/4708694dab6c328590e9faf19f5bed21a7844cd04c000cff333f36e098ed32f3.jpg)



Fig. 10. Comprehensive cost


Fig. 5 shows the impact of search precision on the cost of CCSA. With of improvement of accuracy, the cost of CCSA decreases accordingly. The performance of CCSA becomes stable when the search precision is smaller than 0.01. 

To verify the convergence of CCSGA, we measure the average comprehensive cost of CCSGA with different number of iterations. We can see from Fig. 6 that the output becomes stable after 900 iterations when there are 200 devices and 50 chargers. 

We compare the performance of our algorithms with the optimal solution in small-scale setting. We also measure the performance gap between our algorithms and optimal solution. As shown in Fig. 7, the average comprehensive cost of CCSA is only 7.3% higher than that of optimal solution on average. CCSGA works well enough, and the average comprehensive cost is 16.8% higher than that of optimal solution on average. 

## C. Running Time

The running time of CCSA, CCSGA and optimal solution (only for small-scale setting) is shown in Table III. We can see that the running time grows rapidly with increasing number of devices. However, CCSGA shows great scalability and can output the solution within 0.1 second with 200 devices. 

## VIII. FIELD EXPERIMENTS

We have conducted the field experiments to evaluate all four algorithms. We implemented our proposed algorithms on a testbed which consists of eight rechargeable sensor nodes, five chargers (TX91501 power transmitters produced by Powercast [40]) and an AP that connects to a laptop for reporting energy data collected from the sensor nodes as shown in Fig. 8. We carried out the experiment in a 15 m × 15 m square area, where the eight sensor nodes are placed at the random positions in the area. The coordinates of the five chargers are (3,3), (12,3), (7.5,7.5), (3,12), and (12,12). According to our tests, we set α=7.32 and β=0.05. In our field experiments, the unit of power is milliwatt. The unit charging cost, unit moving cost and require energy of all sensor nodes are set to be in [100, 150], [3, 5] and [10 J, 20 J], respectively. Fig. 9 shows the scheduling results of four algorithms, where the triangles and squares represent the initial positions and destination positions of sensor nodes. The circles represent the positions of chargers. We can see that all sensors in CCSA move to the central charger. This is because the moving cost is this small experiment area is low, and the seneors tend to form a big charging group. In addition, BC and BN output the same charging group. This is because both of BC and BN only consider the individual comprehensive cost, so the sensors tend to choose the nearest charger while the unit charging prices are distributed uniformly. 

Fig. 10 shows the comprehensive cost of the eight sensor nodes. The CCSA has the best performance and outperforms CCSGA, BN and BC by 6.2%, 42.9% and 14.9% on average, respectively. Note that CCSA shows better performance in field experiments. This is because the actual charging power is less than the theoretical value, and therefore, more charging time is needed for given energy requirement. Thus, CCSA can reduce more charging cost. Moreover, although some sensor nodes of BC have less comprehensive cost than CCSGA, the average comprehensive cost of CCSGA is 9.3% lower than that of BC. 

## IX. CONCLUSION

In this paper, we have presented a wireless charging service model from the perspective of cooperative charging economics, and have formulated the CCS problem to minimize the comprehensive cost of whole cooperative charging system. We have proposed two intragroup cost sharing schemes: proportional cost sharing scheme and Shapley Value based cost sharing scheme. We have proposed a $( { \frac { \ln n + 1 } { 1 - \varepsilon } } )$ -approximate algorithm of the CCS problem based on greedy approach, using submodular function minimization. For the large-scale CCS problem, we have presented a game theoretic algorithm, which finally converges to a pure Nash Equilibrium. We demonstrate that our algorithm outperforms the noncooperation charging model by up to 27.3% and 42.9% in terms of comprehensive cost in simulations and field experiments, respectively. 

## ACKNOWLEDGMENT

This work has been supported in part by the NSFC (No. 61872193, 62072254). 

## REFERENCES



[1] G. Sun, Y. Liu, A. Wang, J. Zhang, X. Zhou, and Z. Liu, “Sidelobe Control by Node Selection Algorithm Based on Virtual Linear Array for Collaborative Beamforming in WSNs,” Wireless Personal Commu nications, vol. 90, no. 3, pp. 1443–1462, 2016. 





[2] H. Dai, G. Chen, C. Wang, S. Wang, X. Wu, and F. Wu, “Quality of energy provisioning for wireless power transfer,” IEEE Transactions on Parallel & Distributed Systems, vol. 26, no. 2, pp. 527–537, 2015. 





[3] G. Anastasi, M. Conti, M. Di Francesco, and A. Passarella, “Energy conservation in wireless sensor networks: A survey,” Ad Hoc Networks, vol. 7, no. 3, pp. 537–568, 2009. 





[4] X. Lu, P. Wang, D. Niyato, D. I. Kim, and Z. Han, “Wireless Charging Technologies: Fundamentals, Standards, and Network Appli cations,” IEEE Communications Surveys & Tutorials, vol. 18, no. 2, pp. 1413–1452, 2016. 





[5] T. W. Ching and Y. S. Wong, “Review of wireless charging technologies for electric vehicles,” in Proc. 2013 5th International Conference on Power Electronics Systems and Applications, 2013, pp. 1-4. 





[6] Y. Jin, J. Xu, S. Wu, L. Xu, D. Yang, K. Xia. “Bus network assisted drone scheduling for sustainable charging of wireless rechargeable sensor network,” Journal of Systems Architecture, vol. 116, doi:10.1016/j.sysarc.2021.102059, 2021. 





[7] Y. Jin, J. Xu, S. Wu, L. Xu, D. Yang, ”Enabling the Wireless Charging via Bus Network: Route Scheduling for Electric Vehicles,” IEEE Transactions on Intelligent Transportation Systems, vol. 22, no. 3, pp. 1827-1839, 2021. 





[8] H. Liu, X. Huang, L. Tan, J. Guo, W. Wang, C. Yan, and C. Xu, “Dy namic Wireless Charging for Inspection Robots Based on Decentralized Energy Pickup Structure,” IEEE Transactions on Industrial Informatics, vol. 14, no. 4, pp. 1786-1797, 2018. 





[9] F. Tang, K. Zhang, W. Yan, and B. Song, “Circuit Design of Compensation for Contactless Power System of AUV,” in Proc. of China International Conference on Electricity Distribution (CICED), Shanghai, China, Sept. 2012. 





[10] A. Sample, D. J. Yeager, P. S. Powledge, A. V. Mamishev, and J. R. Smith, “Design of an RFID-based Battery-Free Programmable Sensing Platform,” IEEE Transactions on Instrumentation and Measurement, vol. 57, no.11, pp. 2608-2615, 2008. 





[11] “Bureau International des Expositions.” http://www.expomuseum.com/. 





[12] H. Dai, X. Wu, L. Xu, G. Chen and S. Lin, ”Using Minimum Mobile Chargers to Keep Large-Scale Wireless Rechargeable Sensor Networks Running Forever,” in Proc. International Conference on Computer Communication and Networks (ICCCN), Nassau, Bahamas, 2013, pp. 1-7. 





[13] H. Dai, X. Wu, G. Chen, L. Xu, and S. Lin, “Minimizing the number of mobile chargers for large-scale wireless rechargeable sensor networks,” Computer Communications, vol. 46, pp. 54–65, 2014. 





[14] Y. Li, Y. Chen, C. S. Chen, Z. Wang, and Y. H. Zhu, “Charging while Moving: Deploying Wireless Chargers for Powering Wearable Devices,” IEEE Transactions on Vehicular Technology, vol. 67, no. 12, pp. 11575–11586, 2018. 





[15] S. Zhang, Z. Qian, J. Wu, F. Kong, and S. Lu, “Wireless Charger Placement and Power Allocation for Maximizing Charging Quality,” IEEE Transactions on Mobile Computing, vol. 17, no. 6, pp. 1483–1496, 2018. 





[16] Y. Miao, J. He, and S. Zhu, “Reliable cooperative charging protocol against fault data for supercapacitors charging systems,” in Proc. 2019 IEEE 15th International Conference on Automation Science and Engi neering, 2019, pp. 954–959. 





[17] H. Li, X. Zhang, J. Peng, J. He, Z. Huang, and J. Wang, “Cooperative CC-CV Charging of Supercapacitors Using Multicharger Systems,” IEEE Transactions on Industrial Electronics, vol. 67, no. 12, pp. 10497–10508, 2020. 





[18] L. Jun Wu, P. Xia, S. Zhao, Y. Yanping, W. Chen, and L. Hanzo, “Charging Unplugged,” IEEE Vehicular Technology Magazine, vol. 11, no. 4, pp. 36–45, 2016. 





[19] S. Guha and S. Khuller, “Greedy strikes back: Improved facility location algorithms,” Journal of Algorithms, vol. 31, no. 1, pp. 228–248, 1999. 





[20] S. Iwata, L. Fleischer, and S. Fujishige, “A combinatorial, strongly polynomial-time algorithm for minimizing submodular functions,” Journal of the Acm, vol. 48, no. 4, pp. 761-777, 2001. 





[21] H. Dai, Y. Liu, G. Chen, X. Wu, and T. He, “SCAPE: Safe charging with adjustable power,” IEEE/ACM Transactions on Networking, vol. 26, no. 1, pp. 520–533, 2018. 





[22] H. Dai, H. Ma, A. X. Liu and G. Chen, ”Radiation Constrained Scheduling of Wireless Charging Tasks,” IEEE/ACM Transactions on Networking, vol. 26, no. 1, pp. 314-327, 2018. 





[23] H. Dai et al., “Safe Charging for Wireless Power Transfer,” IEEE/ACM Transactions on Networking, vol. 25, no. 6, pp. 3531–3544, 2017. 





[24] M. Sheikhi, S. Sedighian Kashi, and Z. Samaee, “Energy provisioning in wireless rechargeable sensor networks with limited knowledge,” Wireless Networks, vol. 12, no. 10, pp. 1931–1942, 2019. 





[25] Y. Shu et al., “Near-Optimal Velocity Control for Mobile Charging in Wireless Rechargeable Sensor Networks,” IEEE Transactions on Mobile Computing, vol. 15, no. 7, pp. 1699–1713, 2016. 





[26] G. Fusco and H. Gupta, “Selection and orientation of directional sensors for coverage maximization,” in Proc. IEEE SECON, 2009, pp. 1–9. 





[27] O. Leon, J. Hern´ andez-Serrano, and M. Soriano, “Novel methods for´ energy charging and data collection in wireless rechargeable sensor networks,” International Journal of Communication Systems, vol. 23, no. 5, pp. 633–652, 2010. 





[28] A. Tomar, L. Muduli, and P. K. Jana, “An efficient scheduling scheme for on-demand mobile charging in wireless rechargeable sensor networks,” Pervasive and Mobile Computing, vol. 59, p. 101074, 2019. 





[29] S. Zhang and J. Wu, “Collaborative mobile charging,” Journal of Computer Science and Technology , vol. 64, no. 3, pp. 654–667, 2014. 





[30] J. Wu, “Collaborative mobile charging and coverage,” Journal of Computer Science and Technology , vol. 29, no. 4, pp. 550–561, 2014. 





[31] A. Madhja, S. Nikoletseas, and T. P. Raptis, “Hierarchical, collaborative wireless charging in sensor networks,” in Proc. IEEE WCNC, 2015, pp. 1285–1290. 





[32] C. Lin, Y. Wu, Z. Liu, M. S. Obaidat, C. W. Yu, and G. Wu, “GTCharge: A game theoretical collaborative charging scheme for wireless rechargeable sensor networks,” Journal of System Software, vol. 121, pp. 88–104, 2016. 





[33] Z. Li and M. Ouyang, “The pricing of charging for electric vehicles in China-Dilemma and solution,” Energy, vol. 36, no. 9, pp. 5765–5778, 2011. 





[34] H. Dai, X. Wang, A. X. Liu, H. Ma, G. Chen, and W. Dou, “Optimizing wireless charger placement for directional charging,” in Proc. IEEE INFOCOM, 2017, pp. 1–9. 





[35] P. Zhou, C. Wang and Y. Yang, ”Self-sustainable Sensor Networks with Multi-source Energy Harvesting and Wireless Charging,” in Proc. IEEE INFOCOM, 2019, pp. 1828-1836. 





[36] G. Chalkiadakis, E. Elkind, and M. Wooldridge, “Cooperative game theory: Basic concepts and computational challenges,” IEEE Intelligent Systems, vol. 27, no. 3, pp. 86–90, 2012. 





[37] K. Jain and M. Mahdian, “Cost Sharing,” in Algorithmic Game Theory, N. Nisan, T. Roughgarden, E. Tardos, and V. Vazirani, Eds. Cambridge, U.K.: Cambridge University Press, 2007. 





[38] A. Matsui, “Best response dynamics and socially stable strategies,” Journal of Economic Theory, vol. 57, no. 2, pp. 343–362, 1992. 





[39] D. Monderer, and L. Shapley, “Potential Games,” Games and Economic Behavior, vol. 14, no. 1, pp. 124-143, 1996. 





[40] 2018. www.powercastco.com. 

