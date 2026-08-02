# 3-D Placement of an Unmanned Aerial Vehicle Base Station for Maximum Coverage of Users With Different QoS Requirements

Mohamed Alzenad, Amr El-Keyi, and Halim Yanikomeroglu 

Abstract—The need for a rapid-to-deploy solution for providing wireless cellular services can be realized by unmanned aerial vehicle base stations (UAV-BSs). To the best of our knowledge, this letter is the first in literature that studies a novel 3-D UAV-BS placement that maximizes the number of covered users with different quality-of-service requirements. We model the placement problem as a multiple circles placement problem and propose an optimal placement algorithm that utilizes an exhaustive search (ES) over a 1-D parameter in a closed region. We also propose a low-complexity algorithm, namely, maximal weighted area (MWA) algorithm to tackle the placement problem. Numerical simulations are presented showing that the MWA algorithm performs very close to the ES algorithm with a significant complexity reduction. 

Index Terms—Unmanned aerial vehicles, drone, coverage, optimization. 

## I. INTRODUCTION

wireless services [1], [2]. The need for UAV-BSs could arise in various scenarios, for instance, during a malfunction of the terrestrial infrastructure or for the purpose of offloading traffic from a congested macro BS [1]. UAV-BSs can also play a key role for providing an energy efficient Internet of Things (IoT) communications where UAV-BSs can collect data from the IoT devices and forward it to other devices [3]. 

Despite its promising benefits, UAV-aided communication is facing many challenges. Unlike terrestrial channels, where the location of the BS is fixed, and hence the path loss depends on the location of the user, the air-to-ground (A2G) channel model is a function of the location of the user as well as the UAV-BS. A key challenge in UAV-aided communications is where to deploy the UAV-BS. Furthermore, the UAV-BS placement is no longer a 2D placement problem as for terrestrial BSs. It is indeed a 3D placement problem. Furthermore, the energy available for powering the onboard electronics is limited because of using batteries as a source of power [4]. Therefore, the UAV-BS may not be capable of providing a full coverage for the serving area, and only partial coverage is possible. A key challenge that is addressed in this letter is that given a limited UAV-BS transmit power and users with different quality of service (QoS) requirements, defined in terms of the received signal to noise ratio (SNR), where to deploy the UAV-BS such that the number of covered users is maximized. 

The work in [5] proposed a polynomial-time spiral algorithm for multiple UAVs placement. Alzenad et al. [6] proposed a framework for evaluating the 3D location of the UAV-BS that maximizes the number of covered users using minimum transmit power. The work in [7] evaluated the optimal UAV-BS altitude that maximizes the coverage region. The work in [8] made a further step and deployed the UAV-BS based on the locations of the users and formulated the UAV-BS placement problem as a quadratically-constraint mixed integer non-linear problem. A grid search algorithm was proposed in [9] to tackle a backhaul-aware 3D UAV-BS placement problem. Kalantari et al. [10] developed a particle swarm optimization framework to find the minimum number of UAV-BSs and their locations to serve a particular region. A 3D UAV-BS placement for two cases, one UAV-BS and two UAV-BSs was examined in [11]. Furthermore, Mozaffari et al. [11] optimized the 3D UAV-BS deployment with the aim of maximizing the coverage region with the minimum transmit power. However, the work in [5]–[11] assumes that all the users have the same QoS requirement. 

In this letter, we study a novel 3D UAV-BS placement that has not been previously addressed. Our work aims to maximize the number of covered users demanding different QoS requirements. We model the UAV-BS placement as a multiple circles placement problem. We propose an algorithm that utilizes an exhaustive search (ES) over a one-dimensional parameter in a closed region to determine the optimal height and 2D location of the UAV-BS. In addition, we propose a low-complexity algorithm, namely maximal weighted area (MWA) algorithm to solve the placement problem. We also show by simulations that the proposed MWA algorithm performs very close to the ES algorithm with a significant complexity reduction. 

## II. SYSTEM MODEL

We consider a congested area containing a set of stationary or low-mobility users. The congestion at the terrestrial BS might have occurred due to a number of reasons including a malfunction at the BS or a temporary event such as a festival or a sports event. Therefore, in order to relieve the stress at the terrestrial BS, a UAV-BS is deployed for serving as many users as possible. We assume that each user has one of K different QoS requirements defined in terms of the SNR. Let U denote the set of the users and $\mathcal { U } _ { k } \subseteq \mathcal { U }$ is the set of the users corresponding to QoS k such that $\cup _ { k = 1 } ^ { K } \mathcal { U } _ { k } = \mathcal { U } .$ 

We also denote by $( x _ { i k } , y _ { i k } ) , i = 1 , 2 , \ldots | \mathcal { U } _ { k } | , k = 1 , 2 , \ldots K .$ the 2D location of the user i of the set ${ { \mathcal U } _ { k } }$ 

As discussed in [7], the A2G links are either line-of-sight (LoS) or non line-of-sight (NLoS) with some probability. Assuming a UAV-BS located at $( x _ { D } , y _ { D } , h )$ , the path loss for the LoS and NLoS links in dB is given respectively by 

$$
\begin{array}{r} L _ {\mathrm{LoS}} = 2 0 \log \left(\frac {4 \pi f _ {c} d _ {i k}}{c}\right) + \eta_ {\mathrm{LoS}} \\ L _ {\mathrm{NLoS}} = 2 0 \log \left(\frac {4 \pi f _ {c} d _ {i k}}{c}\right) + \eta_ {\mathrm{NLoS}}, \end{array}\tag{1}
$$

where $f _ { c }$ is the carrier frequency, $d _ { i k }$ is the distance between the UAV-BS and user i of Uk, given by $d _ { i k } = \sqrt { h ^ { 2 } + r _ { i k } ^ { 2 } } .$ where $r _ { i k } = \sqrt { ( x _ { i k } - x _ { D } ) ^ { 2 } + ( y _ { i k } - y _ { D } ) ^ { 2 } }$ . Furthermore, η<sub>LoS</sub> and $\eta _ { \mathrm { N L o S } }$ are the average additional losses for LoS and NLoS, respectively, and are given in [7]. The probability of occurrence of a LoS connection between the UAV-BS and user i of set $\mathcal { U } _ { k }$ located at an elevation angle $\begin{array} { r } { \theta _ { i k } = \tan ^ { - 1 } ( \frac { h } { r _ { i k } } ) } \end{array}$ is given by 

$$
\mathrm {P_ {LoS}} = \frac {1}{1 + a \exp (- b (\frac {1 8 0}{\pi} \theta_ {i k} - a))},\tag{2}
$$

where a and b are constants that depend on the environment. Also, the probability of NLoS is $\mathrm { P _ { N L o S } = 1 - P _ { L o S } }$ In this letter, we only deal with the mean path loss rather than its random behavior because BS deployment often deals with long term variations of the channel rather than small scale variations [6]. Finally, the probabilistic mean path loss is given by 

$$
L (h, r _ {i k}) = L _ {\mathrm{LoS}} P _ {\mathrm{LoS}} + L _ {\mathrm{NLoS}} P _ {\mathrm{NLoS}},\tag{3}
$$

which yields 

$$
L (h, r _ {i k}) = \frac {A}{1 + a \exp (- b (\frac {1 8 0}{\pi} \tan^ {- 1} (\frac {h}{r _ {i k}}) - a))} + 1 0 \log (h ^ {2} + r _ {i k} ^ {2}) + B,\tag{4}
$$

where $A = \eta _ { \mathrm { L o S } } - \eta _ { \mathrm { N L o S } }$ and $\begin{array} { r } { B = 2 0 \log ( \frac { 4 \pi f _ { c } } { c } ) + \eta _ { \mathrm { N L o S } } } \end{array}$ Equation (4) can be further rewritten as 

$$
L (h, r _ {i k}) = \frac {A}{1 + a \exp (- b (\frac {1 8 0}{\pi} \theta_ {i k} - a))} + 2 0 \log (\frac {r _ {i k}}{\cos (\theta_ {i k})}) + B.\tag{5}
$$

Let $P _ { t }$ denote the transmit power of the UAV-BS in dB. The received power at user i of the set ${ { \mathcal U } _ { k } }$ in dB is given by 

$$
P _ {r} ^ {i k} = P _ {t} - L (h, r _ {i k}).\tag{6}
$$

In a noise limited scenario, the conventional approach to define coverage is through the SNR. The ith user of set ${ { \mathcal U } _ { k } }$ is covered if the probabilistic mean SNR exceeds a predefined threshold $\gamma _ { \mathrm { t h } } ^ { k } \ \mathrm { ( d \bar { B } ) }$ . That is if 

$$
\gamma (h, r _ {i k}) (\mathrm{dB}) = P _ {r} ^ {i k} - P _ {n} = P _ {t} - L (h, r _ {i k}) - P _ {n} \geq \gamma_ {\mathrm{th}} ^ {k}\tag{7}
$$

where $P _ { n }$ is the noise power in dB. Clearly, the coverage condition can be equivalently defined in terms of the probabilistic mean path loss. Hence, a user i of set Uk is covered if its link experiences a mean path loss less than or equal to some threshold $L _ { \mathrm { t h } } ^ { k }$ , where $L _ { \mathrm { t h } } ^ { k } \overset { \_ } { = } P _ { t } - P _ { n } - \gamma _ { \mathrm { t h } } ^ { k }$ 

It was shown in [6] and [7] and can also be seen from (4) that, for a given environment, a UAV-BS altitude and a QoS requirement $L _ { \mathrm { t h } } ^ { k }$ , the coverage region is a circular disc with radius $R _ { k } ( h ) = \ddot { r } \vert _ { L ( h , r _ { k } ) = L _ { \mathrm { t h } } ^ { k } }$ . However, for multiple QoS requirements, the coverage region is no longer a single circular disc. We can see from (4) that the region over which all the QoS requirements $\{ L ( h , r _ { k } ) \leq L _ { \mathrm { t h } } ^ { k } \} _ { k = 1 } ^ { K }$ are satisfied forms a set of circular discs with radii $\{ R _ { k } ( h ) \} _ { k = 1 } ^ { K }$ and center $( x _ { D } , y _ { D } )$ Obviously, the larger the required path loss threshold $L _ { \mathrm { t h } } ^ { k }$ , the larger the coverage radius $R _ { k } ( h )$ is. It was shown in [6] that for any QoS requirement $L _ { \mathrm { t h } } ^ { k }$ , the optimal elevation angle $\theta ^ { * }$ that maximizes the coverage radius, is constant and depends only on the environment. The optimal elevation angle is given by [6] 

$$
\theta^ {*} = \tan^ {- 1} (\frac {h _ {k} ^ {*}}{R _ {k} ^ {*}})\tag{8}
$$

where $h _ { k } ^ { * }$ and $R _ { k } ^ { * }$ are the optimal altitude that maximizes the coverage region and the associated maximum coverage radius, respectively, and optimal elevation angle $\theta ^ { * } \ = \ 2 0 . 3 4 ^ { \circ } , 4 2 . 4 4 ^ { \circ } , 5 4 . 6 2 ^ { \circ }$ and $7 5 . 5 2 ^ { \circ }$ for the suburban, urban, dense urban and high-rise urban environments, respectively [6]. For a given environment and a path loss threshold $L _ { \mathrm { t h } } ^ { k } .$ , the maximum coverage radius can be evaluated by solving (5). Finally, $h _ { k } ^ { * }$ can be evaluated by solving (8). 

## III. PROBLEM FORMULATION AND ALGORITHMS

As discussed previously, the coverage region for each set $\mathcal { U } _ { k }$ , denoted by $C _ { k } ,$ is a circular disk with center $( x _ { D } , y _ { D } )$ and radius $R _ { k } ( h )$ . Therefore, placing the coverage regions $\{ C _ { k } \} _ { k = 1 } ^ { K }$ horizontally corresponds to placing the UAV-BS in the horizontal dimension. It is worth mentioning that the coverage regions $\{ C _ { k } \} _ { k = 1 } ^ { K }$ have the same center which corresponds to the horizontal location of the UAV-BS, i.e., (x<sub>D</sub>, y<sub>D</sub>). The user i of set ${ { \mathcal U } _ { k } }$ is covered if it is located within a distance at most $R _ { k } ( h )$ from the center $( x _ { D } , y _ { D } )$ . Let $u _ { i k } \in \{ 0 , 1 \}$ be a binary variable such that $u _ { i k } = 1$ if the user i of set ${ { \mathcal U } _ { k } }$ is within the coverage region $C _ { k }$ and $u _ { i k } = 0$ otherwise. This condition can be written as 

$$
u _ {i k} ((x _ {i k} - x _ {D}) ^ {2} + (y _ {i k} - y _ {D}) ^ {2}) ^ {\frac {1}{2}} \leq R _ {k} (h)\tag{9}
$$

Clearly, when $u _ { i k } = 1 , ( ( x _ { i k } - x _ { D } ) ^ { 2 } + ( y _ { i k } - y _ { D } ) ^ { 2 } ) ^ { \frac { 1 } { 2 } } \le R _ { k } ( h )$ must be satisfied. On the other hand, when $u _ { i k } = 0 ,$ the constraint (9) is trivially satisfied. To avoid the multiplication of the variables $u _ { i k } , x _ { D }$ and $y _ { D } .$ , we use the big-M method. The constraint (9) can thus be further rewritten as 

$$
((x _ {i k} - x _ {D}) ^ {2} + (y _ {i k} - y _ {D}) ^ {2}) ^ {\frac {1}{2}} \leq R _ {k} (h) + M (1 - u _ {i k})\tag{10}
$$

where M is a constant chosen large enough such that the constraint (10) is trivially satisfied when $u _ { i k } = 0$ . The 3D placement problem can be formulated as 

maximize $\sum _ { k = 1 } ^ { K } \sum _ { i \in \mathcal { U } _ { k } } u _ { i k }$ x<sub>D</sub>,y<sub>D</sub>,h,u<sub>ik</sub> 

subject to 

$$
\begin{array}{r l} & \left((x _ {i k} - x _ {D}) ^ {2} + (y _ {i k} - y _ {D}) ^ {2}\right) ^ {\frac {1}{2}} \leq R _ {k} (h) + M (1 - u _ {i k}), \\ & \quad \forall i \in \mathcal {U} _ {k}, k = 1, 2 \dots K, \\ & u _ {i k} \in \{0, 1 \}, \quad \forall i \in \mathcal {U} _ {k}, k = 1, 2, \dots K. \end{array} \tag {11}
$$

The problem (11) is a mixed integer non-linear problem (MINLP) which is difficult to solve. The difficulty of the problem (11) arises due to the coupling between the vertical placement, i.e., h and the horizontal placement, i.e., $( x _ { D } , y _ { D } )$ through the parameters $\{ R _ { k } ( h ) \} _ { k = 1 } ^ { K }$ . In order to simplify problem (11), we decouple the vertical and the horizontal placements. Such decoupling can be performed by utilizing an exhaustive search for the optimal altitude that solves (11). In the following lemma, we show that there exists a closed region in which the optimal altitude is guaranteed to exist. 

Lemma 1: Let $h _ { 1 } ^ { * }$ and $R _ { 1 } ^ { * }$ be the optimal altitude and the associated maximum coverage radius corresponding to the smallest path loss threshold $L _ { t h } ^ { 1 }$ , respectively, and let $h _ { K } ^ { * }$ and $R _ { K } ^ { * }$ be the optimal altitude and the associated maximum coverage radius corresponding to the largest path loss threshold $L _ { t h } ^ { \check { K } } ,$ respectively, then $\exists \check { h ^ { * } } \in [ h _ { 1 } ^ { * } , h _ { K } ^ { * } ]$ that yields $N ( h ^ { * } ) ~ \geq ~ \widetilde { N ( h ) } ~ \forall ~ \bar { h } ~ \notin ~ [ h _ { 1 } ^ { * } , h _ { K } ^ { * } ]$ , where $N ( h )$ is the number of covered users obtained by solving (11) for a given h. 

Proof: Note that $R _ { k } ( h )$ is a concave function in h and has one maxima at $h _ { k } ^ { * } \left[ 1 1 \right]$ . Also, we note that $h _ { m } ^ { * } < h _ { l } ^ { * } \ \mathrm { i f } \ L _ { t h } ^ { m } < L _ { t h } ^ { l }$ since from (4) we have $R _ { m } ( h ) < R _ { l } ( h )$ if $L _ { t h } ^ { m } < L _ { t h } ^ { l }$ and also $\begin{array} { r } { \theta ^ { * } = \tan ^ { - 1 } ( \frac { h _ { k } ^ { * } } { R _ { k } ^ { * } } ) } \end{array}$ is constant. Therefore, $\forall \bar { h } \ < \ h _ { 1 } ^ { * }$ we have $R _ { k } ( \bar { h } ) < R _ { k } ( h _ { 1 } ^ { * } ) \forall k \ { \stackrel { ( a ) } { \Longrightarrow } } \ N ( \bar { h } ) \leq N ( h _ { 1 } ^ { * } )$ . Similarly, $\forall \bar { h } > h _ { K } ^ { * }$ we have $R _ { k } ( h _ { K } ^ { * } ) ~ > ~ R _ { k } ( \bar { h } ) ~ \forall k ~ \stackrel { ( b ) } { \Longrightarrow } ~ N ( h _ { K } ^ { * } ) ~ \geq ~ N ( \bar { h } )$ , which completes the proof. (a) and (b) result from the fact that increasing $R _ { k } ( h )$ enlarges the feasible region of (11) which does not decrease the optimal value of the objective function of (11). 

## A. Exhaustive Search (ES)

The ES algorithm performs an exhaustive search for the optimal altitude $h _ { \mathrm { E } } ^ { \ast }$ over the closed region $[ h _ { 1 } ^ { * } , h _ { K } ^ { * } ]$ . For a given altitude $\begin{array} { r c l } { \breve { h _ { E } } } & { \in } & { [ h _ { 1 } ^ { * } , h _ { K } ^ { * } ] . } \end{array}$ , the associated coverage radii $\{ R _ { k } ( h _ { \mathrm { E } } ) \} _ { k = 1 } ^ { K }$ are computed by solving (4) numerically. Next, (12) is solved to find the optimal horizontal UAV-BS location 

maximize $\sum _ { k = 1 } ^ { K } \sum _ { i \in \mathcal { U } _ { k } } u _ { i k }$ x<sub>D</sub>,y<sub>D</sub>,u<sub>ik</sub> 

subject to 

$$
\begin{array}{c} \left(\left(x _ {i k} - x _ {D}\right) ^ {2} + \left(y _ {i k} - y _ {D}\right) ^ {2}\right) ^ {\frac {1}{2}} \leq R _ {k} \left(h _ {E}\right) + M \left(1 - u _ {i k}\right), \\ \forall i \in \mathcal {U} _ {k}, k = 1, 2 \dots K, \\ u _ {i k} \in \{0, 1 \}, \quad \forall i \in \mathcal {U} _ {k}, k = 1, 2, \dots K. \end{array} \tag {12}
$$

The problem (12) is a mixed integer second order cone problem (MISOCP). Such problems can be solved by branch and cut method whose worst-case complexity is ${ \dot { O ( 2 ^ { n } ) } }$ where n is the number of users. At each branching node, the underlying relaxed subproblem is a SOCP which can be solved in a polynomial time using primal-dual interior point method with complexity $O ( n ^ { 3 . 5 } \log ( \varepsilon ^ { - 1 } ) )$ where ε is the accepted duality gap [12]. Thus, the complexity of solving problem (12) is ${ \bar { O } } ( 2 ^ { n } n ^ { 3 . 5 } \log ( \varepsilon ^ { - 1 } ) )$ 

## B. Maximal Weighted Area (MWA)

Let us consider the case in which the users in the set ${ { \mathcal U } _ { k } }$ are uniformly distributed over the serving region with density $\lambda _ { k } .$ Given that the UAV-BS is at altitude $h ,$ the average number of covered users, denoted by $N _ { \mathrm { a v g } } ( h )$ , is then 

$$
N _ {\mathrm{avg}} (h) = \pi \sum_ {k = 1} ^ {K} \lambda_ {k} R _ {k} ^ {2} (h).\tag{13}
$$

Obviously, maximizing the average number of covered users $N _ { \mathrm { a v g } } ( h )$ for a uniformly distributed users depends only on the UAV-BS’s altitude. In order to obtain the optimal altitude $h _ { M } ^ { * }$ that maximizes (13), we need to search for h that satisfies 

$$
\frac {\partial}{\partial h} \sum_ {k = 1} ^ {K} \lambda_ {k} R _ {k} ^ {2} (h) = 0,\tag{14}
$$

which yields the following 

$$
\sum_ {k = 1} ^ {K} \frac {2 \lambda_ {k} X _ {k} (h) R _ {k} ^ {2} (h)}{R _ {k} ^ {2} (h) + h ^ {2} + h X _ {k} (h)} = 0,\tag{15}
$$

where 

$$
X _ {k} (h) = \frac {- 9 \ln (1 0) A a b}{\pi} \frac {R _ {k} (h) \exp (- b [ \frac {1 8 0}{\pi} \tan^ {- 1} (\frac {h}{R _ {k} (h)}) - a ])}{(1 + a \exp (- b [ \frac {1 8 0}{\pi} \tan^ {- 1} (\frac {h}{R _ {k} (h)}) - a ]) ^ {2}} - h.\tag{16}
$$

It can be shown that (15) has a solution in the interval $[ h _ { 1 } ^ { * } , h _ { K } ^ { * } ]$ . However, this solution may not be unique. Clearly, (15) is an implicit function of h. Therefore, we need to search for $h _ { M } ^ { * }$ that satisfies (15) numerically. 

The maximal weighted area (MWA) algorithm deploys the UAV-BS at the altitude $h _ { M } ^ { * } . ^ { 1 }$ Let $\{ R _ { k } ( h _ { M } ^ { * } ) \} _ { k = 1 } ^ { K }$ be the coverage radii associated with the altitude $h _ { M } ^ { * }$ . The problem (11) then reduces to 

$$
\underset {x _ {D}, y _ {D}, u _ {i k}} {\text { maximize }} \sum_ {k = 1} ^ {N} \sum_ {i \in \mathcal {U} _ {k}} u _ {i k}
$$

subject to 

$$
\begin{array}{c} \left(\left(x _ {i k} - x _ {D}\right) ^ {2} + \left(y _ {i k} - y _ {D}\right) ^ {2}\right) ^ {\frac {1}{2}} \leq R _ {k} \left(h _ {M} ^ {*}\right) + M \left(1 - u _ {i k}\right), \\ \forall i \in \mathcal {U} _ {k}, k = 1, 2 \dots K, \\ u _ {i k} \in \{0, 1 \}, \quad \forall i \in \mathcal {U} _ {k}, k = 1, 2, \dots K, \end{array} \tag {17}
$$

which is a MISOCP. 

## IV. SIMULATION RESULTS

We consider a square 3 km × 3 km urban area with parameters $a = 9 . 6 1 , b = 0 . 1 6 , \eta _ { \mathrm { L o S } } = 1$ and $\eta _ { \mathrm { N L o S } } = 2 0$ . We also consider a UAV-BS that transmits its signal at $f _ { c } = 2$ GHz and $P _ { t } = 3 0$ dBm. We assume that there are two sets of users $\mathcal { U } _ { 1 }$ and $\mathcal { U } _ { 2 }$ uniformly distributed with densities $\lambda _ { 1 }$ and $\lambda _ { 2 }$ respectively. However, for a fair comparison, the total density of users is fixed at $\lambda = \lambda _ { 1 } + \lambda _ { 2 } = 1 1$ users/km<sup>2</sup>. Furthermore, we assume that the users demand QoS defined as $\gamma _ { \mathrm { t h } } ^ { 1 } = 5 0$ dBm and $\gamma _ { \mathrm { t h } } ^ { 2 } = 4 7$ dBm for $\mathcal { U } _ { 1 }$ and $\mathcal { U } _ { 2 }$ , respectively, with $P _ { n } ~ = ~ - 1 2 \ddot { 0 }$ dBm. For comparison, we assume a UAV-BS placement algorithm, namely largest QoS (LQ) algorithm. The LQ algorithm assumes that all the users have the same QoS requirement $\gamma _ { \mathrm { t h } } = 5 0 ~ \mathrm { d B }$ , i.e., $L _ { t \mathrm { h } } = 1 0 0$ dB and the UAV-BS is therefore deployed vertically at $h _ { L O } ^ { * } = 6 4 6 . 5$ m which results in maximal coverage radius $R ^ { * } \stackrel { \sim } { = } 7 0 7$ m. The LQ algorithm is based on the observation that any user i of set ${ { \mathcal U } _ { k } }$ , regardless of the required QoS, falling within the coverage region that corresponds to the largest SNR threshold will be covered. For the ES algorithm, we perform an exhaustive search for the optimal altitude $h _ { E } ^ { * }$ over the closed region [646.5, 913] m. Furthermore, we discretize the altitude range [646.5, 913] m into a uniform one-dimensional grid of 9 points where the discretization step is given by $\Delta h = 2 9 . 6$ m. In this letter, we use the CVX parser/solver with the MOSEK solver to solve problems (12) and (17). 

![](images/2fbbbd637395ad2537fa9e0fcb32acd517c2934de27566b773058420f64b9675.jpg)



Fig. 1. CDF of the number of covered users $( \rho = 1 )$


![](images/e0adbfcc2fd5c684d874f03a001cce966177e6fe7e99cbe996cae52605d0bc8c.jpg)



Fig. 2. CDF of execution time $( \rho = 1 )$


![](images/6358e73ebb7378221d229c943bb510bc0c295a9e4a1eaaa9ca5b395e7b86ec18.jpg)



Fig. 3. Average number of covered users versus density ratio.


The number of covered users and execution time are random quantities whose distributions can be measured by the cumulative distribution function (CDF). Fig. 1 and Fig. 2 show the CDF of the number of covered users and the CDF of the execution time for $\begin{array} { r } { \rho = \frac { \lambda _ { 2 } } { \lambda _ { 1 } } = 1 } \end{array}$ , respectively. As shown in Fig. 1, the ES and MWA algorithms have very close performance and both outperform the LQ algorithm. However, based on Fig. 2, the ES algorithm has the worst execution time with a significant gap to that of the MWA and the LQ algorithms. 

Fig. 3 shows the average number of covered users versus the density ratio $\rho .$ Clearly, the performance of the MWA algorithm is very close to that of the ES algorithm. It is also worth noting that as $\rho$ increases, the gap between the MWA and ES algorithms on one hand and the LQ algorithm on the other hand increases. This is because as $\rho$ increases, the number of elements in $\mathcal { U } _ { 2 }$ (U<sub>1</sub>) increases (decreases). However, the LQ algorithm does not consider the density of the users in the set $\mathcal { U } _ { 2 }$ which justifies the gap increase. 

## V. CONCLUSION

In this letter, we studied a novel 3D placement of a UAV-BS that maximizes the number of covered users with different QoS requirements. We modeled the placement problem as a multiple circles placement problem and proposed an optimal placement algorithm that utilizes an exhaustive search over a one-dimensional parameter in a closed region. We also proposed a low-complexity algorithm, referred to as the MWA algorithm, to solve the placement problem. Simulations have shown that the MWA algorithm performs very close to the ES algorithm with a significant reduction in complexity. 

## REFERENCES



[1] I. Bor-Yaliniz and H. Yanikomeroglu, “The new frontier in RAN heterogeneity: Multi-tier drone-cells,” IEEE Commun. Mag., vol. 54, no. 11, pp. 48–55, Nov. 2016. 





[2] Y. Zeng, R. Zhang, and T. J. Lim, “Wireless communications with unmanned aerial vehicles: Opportunities and challenges,” IEEE Commun. Mag., vol. 54, no. 5, pp. 36–42, May 2016. 





[3] M. Mozaffari, W. Saad, M. Bennis, and M. Debbah, “Mobile unmanned aerial vehicles (UAVs) for energy-efficient Internet of Things communications,” IEEE Trans. Wireless Commun., to be published, doi: 10.1109/TWC.2017.2751045. 





[4] Y. Zeng and R. Zhang, “Energy-efficient UAV communication with trajectory optimization,” IEEE Trans. Wireless Commun., vol. 16, no. 6, pp. 3747–3760, Jun. 2017. 





[5] J. Lyu, Y. Zeng, R. Zhang, and T. J. Lim, “Placement optimization of UAV-mounted mobile base stations,” IEEE Commun. Lett., vol. 21, no. 3, pp. 604–607, Mar. 2017. 





[6] M. Alzenad, A. El-Keyi, F. Lagum, and H. Yanikomeroglu, “3-D placement of an unmanned aerial vehicle base station (UAV-BS) for energy-efficient maximal coverage,” IEEE Wireless Commun. Lett., vol. 6, no. 4, pp. 434–437, Aug. 2017. 





[7] A. Al-Hourani, S. Kandeepan, and S. Lardner, “Optimal LAP altitude for maximum coverage,” IEEE Wireless Commun. Lett., vol. 3, no. 6, pp. 569–572, Dec. 2014. 





[8] R. I. Bor-Yaliniz, A. El-Keyi, and H. Yanikomeroglu, “Efficient 3-D placement of an aerial base station in next generation cellular networks,” in Proc. IEEE Int. Conf. Commun. (ICC), Kuala Lumpur, Malaysia, May 2016, pp. 1–5. 





[9] E. Kalantari, M. Z. Shakir, H. Yanikomeroglu, and A. Yongacoglu, “Backhaul-aware robust 3D drone placement in 5G+ wireless networks,” in Proc. IEEE Int. Conf. Commun. Workshop (ICCW), Paris, France, May 2017, pp. 109–114. 





[10] E. Kalantari, H. Yanikomeroglu, and A. Yongacoglu, “On the number and 3D placement of drone base stations in wireless cellular networks,” in Proc. IEEE 84th Veh. Technol. Conf. (VTC Fall), Montreal, QC, Canada, Sep. 2016, pp. 1–6. 





[11] M. Mozaffari, W. Saad, M. Bennis, and M. Debbah, “Drone small cells in the clouds: Design, deployment and performance analysis,” in Proc. IEEE Glob. Commun. Conf. (GLOBECOM), San Diego, CA, USA, Dec. 2015, pp. 1–6. 





[12] Y. E. Nesterov and M. J. Todd, “Self-scaled barriers and interior-point methods for convex programming,” Math. Oper. Res., vol. 22, no. 1, pp. 1–42, 1997. 

