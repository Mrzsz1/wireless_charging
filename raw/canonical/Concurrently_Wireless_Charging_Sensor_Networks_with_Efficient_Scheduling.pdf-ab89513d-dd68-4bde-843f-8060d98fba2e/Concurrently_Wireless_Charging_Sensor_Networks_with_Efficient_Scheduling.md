# Concurrently Wireless Charging Sensor Networks with Efficient Scheduling

Peng Guo, Xuefeng Liu, Shaojie Tang, and Jiannong Cao, Fellow, IEEE 

Abstract—Wireless charging technology is considered as a promising solution to address the energy limitation problem for wireless sensor networks (WSNs). In scenarios where the deployed chargers are static, we generally require a number of chargers to work simultaneously. However, due to the radio interference among different wireless chargers, scheduling these chargers is generally necessary. This scheduling problem is challenging since each charger’s charging utility cannot be calculated independently due to the nonlinear superposition charging effect caused by radio interference. In this paper, based on the concurrent charging model, we formulate the concurrent charging scheduling problem (CCSP) with the objective of quickly fully charging all the sensor nodes. After proving the NP-hardness of CCSP, we propose two efficient greedy algorithms, and give the approximation ratio of one of them. Both the two greedy algorithms’ performances are very close to that of a well-designed genetic algorithm (GA) which performs almost as wel as a brute force algorithm at small network and charger scale. However, the running time of the two greedy algorithms is far lower than that of the GA. We conduct extensive simulations and specially implemented a testbed for wireless chargers. The results verified the good performance of the proposed algorithms. 

Index Terms—Wireless charging, wireless sensor networks (WSNs), scheduling, radio interferenc 

Ç 

## 1 INTRODUCTION

IMITED energy is always a bottleneck of most applicathe lifetime of WSNs, a variety of wireless charging techniques have been proposed to provide WSNs additional energy supply [1], [2], [3], [4], [5]. To achieve high charging efficiency, usually a mobile charger moves around in WSNs with a carefully designed route, and charges the sensor node nearby [6], [7]. However, in many practical applications of WSNs, the mobile charger may not be able to move freely in WSNs, or even could hardly move around (e.g., scenario of structural health monitoring with WSNs). 

With recent advances in radio energy harvesting techniques, it is possible to charge sensor nodes in a relative long distance ( > <sup>10 m</sup> away) with a fixed charger connecting to the power line. It has been validated that sensor node could harvest radio energy with <sup>6</sup> m<sup>W</sup> power when putting a charger with transmission power <sup>4 W</sup> about 12 meters away (the received radio power is <sup>20</sup> m <sup>W</sup> and the transition efficiency is 30 percent) [9]. The long-distance charging can be free from the practical terrains. However, compared to the mobile charging, the long-distance charging provides much weaker harvesting power (usually mW-level) at sensor nodes. As a result, to support <sup>60 m</sup>W-level packet communication (e.g., typical value in Micaz nodes), it may take thousands of packet transmission durations for a charger at long distance to charge the sensor node. 

To accelerate the long-distance charging, a straightforward way is to increase the charger’s power, which however may lead to serious electromagnetic radiation pollution and hence is strictly restricted by Federal Communications Commission (FCC). Another way is to employ multiple chargers located at different positions in WSNs to charge the sensor nodes concurrently (called as concurrent charging in this paper). With the cooperation of the multiple chargers, the harvesting power at the sensor nodes can be increased, and the charging range can also be extended. 

To facilitate sensor nodes to harvest the energy from multiple chargers’ radio signals, the chargers should constrain their radio signals’ power spectral density (PSD) within a narrow spectrum band which is determined by the radio inducing circuit of the sensor nodes. As a result, the chargers’ PSD curves are largely overlapped with each other. We call these chargers are in-band. When multiple inband chargers transmit radio, there will be radio interference among the concurrent emitted radio waves. In particular, if the phase difference of two encountering waves is less than p=<sup>2</sup>, the waves will combine constructively, and the combined radio’s power can be even larger than the sum of each wave’s power. However, if the phase difference of the encountering waves is over p=<sup>2</sup>, the waves will combine destructively, and the combined radio’s power can be less than any one of the waves’ power or even be close to zero. 

To show this typical phenomenon, Fig. 1 gives a simple experiment. When putting a <sup>40</sup> <sup>mW</sup>-level charger (along with a small reference radio source introduced later) near to an energy harvesting mote, the mote gets about <sup>4</sup>:<sup>1</sup> <sup>V</sup> voltage. However, when additionally putting an in-band charger at some certain position near to the mote, the voltage at the mote decreases to be about <sup>1</sup>:<sup>2 V</sup>. This radio interference phenomenon is also shown in [11], [12]. It can be seen, concurrent radio charging induces a typical nonlinear superposition charging effect. Adding chargers may not increase the combined radio power at a sensor node, and turning off some chargers may not decrease the power either. 

![](images/45325068b4b3986e0003a897ba4118d4b5218afe02ffed8dd1ea7a32c5118100.jpg)



Fig. 1. A simple experiment on radio interference.


To show the reason behind the nonlinear superposition charging effect, Fig. 2 illustrates the typical instance on concurrent charging with two radio chargers. Suppose the radio waves emitted by the two chargers $c _ { 1 }$ and $c _ { 2 }$ have the same phase. With the waves propagating in the air, the phase of the wave arriving at some position is different from the current phase of the source wave, and the difference is determined by $d | _ { m o d \lambda } * 2 \pi ,$ , where $d$ is the distance between the position and the source. It can be seen from Fig. 2 that the phase of wave arriving at $s _ { 1 }$ from $c _ { 1 }$ is always the same as that at $c _ { 1 . }$ , due to the distance of integer number of -. Similarly, the phase of wave arriving at $s _ { 1 }$ from $c _ { 2 }$ is also the same as that at $c _ { 2 } .$ . Consequently, two waves from $c _ { 1 }$ and $c _ { 2 }$ will combine constructively at $s _ { 1 } .$ . As for $s _ { 2 } ,$ , the phase of wave arriving at $s _ { 2 }$ from $c _ { 1 }$ is the same as that at $c _ { 1 , }$ , while the phase of wave arriving at $s _ { 2 }$ from $c _ { 2 }$ lags that at $c _ { 2 }$ for $( k \lambda + \lambda / 2 ) | _ { m o d \lambda } * 2 \pi = \pi$ . Therefore, the two waves respectively from $c _ { 1 }$ and $c _ { 2 }$ will combine destructively at $s _ { 2 }$ . 

Due to the special charging effect of concurrent charging, turning on all chargers does not always lead to higher charging efficiency, as analyzed in Section 3. Hence, scheduling the concurrent charging is quite necessary. In this paper, we are concerned with such a question: given a group of chargers and sensor nodes, how to optimally schedule the chargers so as to use the minimum time to charge each sensor node with at least energy $E ?$ To answer this question, based on the concurrent charging model, we formulate the concurrent charging scheduling problem (CCSP). A specialty of CCSP is that the charging utility of each charger cannot be defined or calculated separately due to the nonlinear superposition charging effect in the concurrent charging, which brings some new challenges to the scheduling algorithm design. 

Contributions of the paper are summarized as follows: 

Based on the concurrent charging model, we formulate the concurrent charging scheduling problem, and prove the NP-hardness of the problem by reducing set cover problem to it. 

![](images/e48c6f2d198bdac4acb445fb89dbc26f17779c692cabb1a357c7d749342745ff.jpg)



Fig. 2. Concurrent charging with two radio chargers. Concurrent charging with $c _ { 1 }$ and $c _ { 2 }$ significantly enhances the radio power at $s _ { 1 }$ while leads to almost zero radio at $s _ { 2 }$


We propose two efficient greedy algorithms to solve CCSP. One is based on submodular set cover problem, and we give the proof of approximation ratio. Another is based on the idea of balanced charging, which usually outperforms the former. In addition, we design a genetic algorithm (GA) whose performance can at least be much close to that of a brute force algorithm at a small scale. 

We conduct both simulations and real experiments. The results show that the two greedy algorithms perform almost as good as GA, while taking much less running time than GA. 

The remainder of the paper is organized as follows. Section 2 reviews related works. Section 3 introduces the concurrent charging model. In Section $^ { 4 , }$ we formulate the CCSP problem and propose three algorithms in Section 5. Section 6 gives the simulation results and some experimental results, followed by conclusions in Section 7. 

## 2 RELATED WORK

Wireless charging for WSNs has been widely studied in recent years. Electromagnetic radiation is a cost-efficient way for sensor nodes to harvest the energy, and hence is often explored for charging WSNs [1], [13], [14]. Electromagnetic radiation usually transfers energy in a narrow ISM band (typical center frequency <sup>915</sup> <sup>MHz</sup>). In [8], a receiver node obtains <sup>1</sup>:<sup>5 mW</sup> power when putting a radio transmitter (called as charger in this paper) with power <sup>100 mW 30 cm</sup> away, and similar experiments were also reported in [13] and [14]. Due to the low charging efficiency of radio, most people study using a mobile charger to move around in WSNs and charge sensor node when the charger is close to the node [15], [16], [17], [18], [19], [20]. However, in many scenarios the mobile chargers may not move freely. In addition, the energy supply of the mobile chargers themselves is also a bottleneck. 

With the improvement of radio energy harvesting technique, people find it possible to charge sensor nodes in relatively long distance. By setting a small antenna array, some commercial wireless charging products [21] can harvest sufficient power, which can be used to charge smartphone, from a <sup>4 W</sup> charger 15 feet (about <sup>4</sup>:<sup>6 m</sup>) away. As for the low-power terminals like sensor nodes, the charging range can be further extended. In [9], the authors design an efficient energy harvesting circuit which can achieve 35 percent efficiency at <sup>20</sup> <sup>dBm</sup> input radio. According to this parameter, the circuit can obtain about <sup>6</sup> $\mu \mathrm { W }$ power from a <sup>4</sup> <sup>W</sup> charger <sup>12</sup> <sup>m</sup> away. Therefore, it is feasible to charge sensor nodes in a long distance, thus making the charging free from the practical terrain. 

Taking an example of MicaZ sensor node, the wireless communication power is about <sup>60 mW</sup>. If keeping charging communication power is about 60 mW. If keeping charging the node for one minute with harvesting power $6 \mu \mathrm { W } .$ , the node can supply one 125 bytes packet transmission (about <sup>6ms</sup> active time). Such charging efficiency could hardly meet the energy demand of sensor nodes or satisfy their communication delay requirement. To this end, with the constraint of the charger’s power level, it is needed to employ multiple chargers to charge the sensor nodes concurrently, so as to enhance the charging efficiency. However, concurrent charging will cause radio interference, as a result leading to the nonlinear superposition charging effect. As analyzed in Section 3, the nonlinear superposition charging effect may make some sensor nodes hardly harvest any energy, although several chargers are turned on. Hence, it is needed to schedule the chargers so as to efficiently charge each node in the scenario. 

![](images/916b3f45426f2bb4ab988bfb5ace23efd195ec892cb6646ee54b624db7ef3117.jpg)


![](images/0d24593a444a3796c2376397392d49a7e6953e2b9be5c8df157f7e626d8944e6.jpg)



(a)



Fig. 3. (a) Scenario of concurrent charging with multiple chargers and sensor nodes. (b) Relationship between chargers’ PSD and the bandwidth of sensor node’s antenna.


In [8], the authors studied efficient deployment of multiple fixed readers to concurrently charge the possible tags spread around (or say, just to cover the target area). However, the radio interference effect among the readers is ignored in their simulations. To our best knowledge, there is little work studying concurrent charging of WSNs with considering the radio interference, except for [10], [11], [12]. The authors in [11] noticed the radio interference effect among the in-band chargers, and propose RF-MAC protocol which studies how to access sensor nodes’ charging requests and cooperatively charge one accessed node at a time, by partitioning the chargers into two groups. As in RF-MAC sensor nodes are charged on-demand, the chargers’ energy is efficiently utilized. However, in RF-MAC, a sensor node is charged only when it has communication request, which always incurs certain delay to execute the communication. 

In this paper, we study the simple TDMA-based MAC which fully charges all sensor nodes in advance so as to real-time supply the following possible communications of the nodes. Hence, different from roughly partitioning the chargers into two groups in RF-MAC, in this paper we study how to efficiently schedule the chargers so as to fully charge all the sensor nodes as soon as possible. In Section $^ { 3 , }$ we give more detailed analysis on RF-MAC and the TDMA-based MAC. 

## 3 PRELIMINARY

## 3.1 Background and Radio Power Model

Fig. 3a illustrates the scenario of CCSP. Multiple chargers and a group of sensor nodes are deployed in a common area. The chargers transmit radio waves with certain power level. The spectrum of each charger’s radio signal usually can be illustrated with a PSD curve, as shown in Fig. 3b. The sensor where illustrated with a PSD curve, as shown in Fig. 3b. The sensor nodes harvest the energy of the concurrent radio waves around, through an antenna working within a narrow bandwidth. To facilitate sensor nodes to harvest the energy of the chargers’ radio, the chargers should set their PSD curves within the bandwidth of the antenna, as illustrated in Fig. 3b. 

Although there is usually slight difference in chargers center frequencies and PSD curves in practice, we assume theoretically that the chargers have the same center frequency and PSD curve. This assumption leads to just a little magnification of the radio interference effect but does not change the nonlinear superposition attribute. 

To mathematically describe the nonlinear superposition attribute, a model on the concurrent charging needs to be established. In [10], the authors model the concurrent charging in the ideal condition of single-frequency radio wave. This model can be easily extended for the case of narrowband radio wave. We take example of an arbitrary frequency point $\omega _ { 0 }$ on the PSD curve of the narrow-band radio wave to extend the model as follows. 

Suppose the amplitude of frequency component $\omega _ { \mathrm { 0 } }$ in the chargers’ PSD curve is $A _ { 0 } ,$ , and the corresponding initial phase is $\varphi _ { 0 }$ . Hence, the power density of each charger at v0 is $p _ { 0 } = A _ { 0 } ^ { 2 } / 2$ . Due to the radio attenuation in the space, the amplitude of the radio emitted by charger $c _ { i }$ decreases with the distance. For simpleness, we assume the power attenuation factor is 2. Thus, the radio signal of frequency component v0 arriving at sensor node $s _ { j }$ from $c _ { i }$ can be expressed as 

$$
a _ {i 0} (t) = \frac {A _ {0}}{4 \pi d _ {i j} / \lambda_ {\omega_ {0}}} \cos \left(\omega_ {0} t + \varphi_ {0} - 2 \pi \frac {d _ {i j}}{\lambda_ {\omega_ {0}}}\right),\tag{1}
$$

where $d _ { i j }$ is the distance between $c _ { i }$ and sensor node $s _ { j } ,$ and $\lambda _ { \omega _ { 0 } }$ is the wave length of the radio at frequency v0. Considering that the chargers’ PSD is much narrow (e.g., centered at <sup>915 MHz</sup> with range about tens of <sup>kHz</sup>), the variation of wave length at different frequencies of PSD is ultra small $( < \pm 0 . 0 1$ percent). Thus, we have $\lambda _ { \omega _ { 0 } } \approx \lambda .$ 

The compound radio signal of frequency component v0 at $s _ { j }$ from a group of chargers C is: 

$$
A _ {0} ^ {j} (t) = \sum_ {c _ {i} \in C} a _ {i 0} (t) = \sum_ {c _ {i} \in C} \frac {A _ {0}}{\hat {d} _ {i j}} \cos \left(\omega_ {0} t + \varphi_ {0} - 2 \pi \frac {d _ {i j}}{\lambda}\right),\tag{2}
$$

where $\begin{array} { r } { \hat { d _ { i j } } = \frac { 4 \pi d _ { i j } } { \lambda } } \end{array}$ 

The power density of the compound radio signal $A _ { 0 } ^ { j } ( t )$ is $\overline { { { [ A _ { 0 } ^ { j } ( t ) ] } ^ { 2 } } }$ , where  denotes the average of . Therefore, we have: 

$$
\begin{array}{c}\overline{[A_{0}^{j}(t)]^{2}} = \frac{A_{0}^{2}}{2}\sum_{c_{i}\in C}\frac{1}{\hat{d}_{ij}{}^{2}} +\frac{A_{0}^{2}}{2}\sum_{c_{i}\in C}\sum_{\substack{c_{m}\in C\\ c_{m}\neq c_{i}}}\frac{\cos\left(2\pi\frac{d_{ij} - d_{mj}}{\lambda}\right)}{\hat{d}_{ij}\hat{d}_{mj}}\\ \\ = p_{0}\sum_{c_{i}\in C}\frac{1}{\hat{d}_{ij}{}^{2}} +p_{0}\sum_{c_{i}\in C}\sum_{\substack{c_{m}\in C\\ c_{m}\neq c_{i}}}\frac{\cos\left(2\pi\frac{d_{ij} - d_{mj}}{\lambda}\right)}{\hat{d}_{ij}\hat{d}_{mj}}. \end{array}\tag{3}
$$

Hence, the power of compound radio signal at sensor $s _ { j }$ is: 

$$
\begin{array}{l}P_{j}|_{C} = \int \overline{[A_{0}^{j}(t)]^{2}} d\omega \\ = P\sum_{c_{i}\in C}\frac{1}{\hat{d}_{ij}^{2}} +P\sum_{c_{i}\in C}\sum_{\substack{c_{m}\in C\\ c_{m}\neq c_{i}}}\frac{1}{\hat{d}_{ij}\hat{d}_{mj}}\cos \left(2\pi \frac{d_{ij} - d_{mj}}{\lambda}\right), \end{array}\tag{4}
$$

$\begin{array} { r } { P = \int p _ { i } d \omega } \end{array}$ is the radio power of the chargers. <sup>i</sup>       Authorized licensed use limited to: Nanjing Univ of Post & Telecommunications. Downloaded on October 20,2024 at 09:30:54 UTC from IEEE Xplore. Restrictions apply. 

(m) 

![](images/f23424fb90953bb512127315df8784b99d8cd4e93f12c733743d44e537422db0.jpg)



Fig. 4. Distribution of compound radio power in an area with multiple chargers.


From Equation (4), we can see the nonlinear superposition charging effect in the concurrent charging. In particular, if the distance between each charger $c _ { i }$ and sensor $s _ { j }$ is the same, the radio waves from the chargers will combined constructively at $s _ { j } ,$ and the power of the compound radio signal is $N ^ { 2 } P / \hat { d _ { i j } } ^ { 2 }$ , where $N$ is number of chargers in $C .$ However, if the chargers work individually, the sum of the power for each charger is $N P / { \hat { d _ { i j } } } ^ { 2 }$ . It can be seen, constructive interference significantly increases the power of the compound signal at the sensor node. However, if the chargers’ signal arriving at sensor $s _ { j }$ combine destructively with phase difference over $\pi / 2 ,$ , the second part in the right side of Equation (4) can be negative, thus making the power of the compound radio signal even smaller than the sum of the power for each charger. Furthermore, in some special cases, the compound radio power $P _ { j | _ { C } }$ can even be close to zero although there are multiple chargers working around. 

It is notable that, the model in Equation (4) is much consistent with the experimental results in [11]. In their experiment, a charger and a harvester are fixed, while another charger gradually increases its distance to the harvester, thus resulting in a decreasing cosine-like curve for the charging efficiency, just as the calculation in Equation (4). The experiment provides a good verification of the model. 

An additional issue in the above model is the initial radio phase in practice. When the chargers generate radio independently, their radios may have different initial phase. To get to know their initial phase, we place a special radio source as reference for all the chargers, which helps to ascertain the relative radio phases of the chargers. The details are introduced in the experiments of Section 6. Note that, we do not strictly require the chargers to have the same initial phase but requires to have steady relative phase. The difference of initial phase $\Delta \varphi$ can be regarded as a virtual deviation of distance $\Delta \varphi * \lambda$ 

To clearly illustrate the nonlinear superposition charging effect in the concurrent charging, Fig. 4 shows the distribution of compound radio power in an area with four chargers (located at the four corners in the square area). It can be seen, the concurrent wireless charging results in much uneven distribution of compound radio power, and at many positions the compound radio power is even too low to be harvested. However, we expect that each sensor node spread in the area can obtain sufficient radio power. Therefore, appropriately scheduling the chargers is needed. 

## 3.2 Radio Harvesting Model

Denote $P _ { j } ^ { H } | _ { C }$ as the harvesting power of sensor node $s _ { j }$ charged by a group of chargers $C .$ We assume $P _ { j } ^ { H } | _ { C } = \rho * P _ { j } | _ { C } ,$ where $\rho \left( 0 < \rho < 1 \right)$ is the transition coefficient. According to [9], when given the load impedance of a radio harvesting circuit, the harvesting efficiency of the circuit can be optimized, and $\rho$ can be approximately assumed to be constant for the optimized radio harvesting circuit, especially when $P _ { j } | _ { C } \in ($ <sup>20 dBm</sup>; <sup>0 dBm</sup>Þ (the most harvesting cases in practice). 

In addition, we notice that: if the radio power is too weak that the inducing voltage is lower than the forward threshold voltage of the diode in the harvesting circuit, the sensor node could not harvest any energy. Taking this consideration, we present the harvesting model as follows: 

$$
e _ {j} | _ {C, T} = \left\{ \begin{array}{l l} 0 & \text {if P_{j} |_{C} <  \delta ,} \\ \rho T (P _ {j} | _ {C} - \delta) & \text {otherwise.} \end{array} \right.\tag{5}
$$

where $e _ { j } | _ { C , T }$ denotes the energy of $s _ { j }$ obtained from a group of chargers $C$ during duration $T ,$ , and d is a threshold for the radio harvesting power at sensor node $s _ { j } .$ 

## 3.3 Coexistence of Charging and Communication

To facilitate sensor nodes to harvest at least $\mu \mathrm { W } .$ -level power at long distance ( > <sup>10 m</sup>), the chargers usually need to work at W-level power, which leads to quite large interference range for sensor nodes’ communication. For example, a <sup>1 W</sup> charger 100 meters away from a pair of ZigBee sensor nodes (only <sup>10 m</sup> apart), can lead to the quality of the nodes communication signal $( \mathrm { i . e . , }$ , SNIR) to be no more than <sup>6dB</sup>. In [12], the authors give an experimental study on the concurrent data and wireless charging for sensor networks, confirming the large interference range. Hence, to avoid the significant interference from the chargers, sensor nodes have to take communications only when no charger is active in the area. 

Furthermore, compared to the <sup>mW</sup>-level communication power of sensor nodes, the m<sup>W</sup>-level harvesting power at the sensor nodes is much small. Thus, to supply one packet communication lasting for just several milliseconds, a sensor node needs to be charged for over 1 minute during which all other nodes have to suspend their communications, resulting in high communication delay. Therefore, it is needed to design a mechanism to insulate the long-term charging and randomly arriving communication requests. 

Considering the uncertain time when sensor nodes’ communication requests arrive, a feasible solution to insulate the wireless charging and wireless communication is to employ time division multiple access (TDMA) mechanism. Fig. 5 shows the TDMA schedule for the charging and communication of sensor nodes. Generally, the sensor nodes work in turn for being charging and communications. During each turn (or round) of charging, it is expected to fully charge the sensor nodes as soon as possible. After being fully charged, the sensor nodes are ready for potential communications in the communication round, and no charger is allowed to be nloaded on October 20,2024 at 09:30:54 UTC from IEEE Xplore. Restrictions apply. 


Charging period  Slot for potential communications in WSN


![](images/68a604232f8cadc1c4fdd459719272a637cc9c0e4905c054022df5da7fa3c1f0.jpg)



Fig. 5. TDMA scheduling for charging and communication.


active at this time. When the sensor nodes tend to exhaust their energy, another round of charging starts. 

Due to the low efficiency of long-distance charging, the charging round still needs to take relatively long time. During each charging round, sensor nodes may have communication requests. To avoid long-time waiting for the communication round, it is needed to reserve some time slots, which are evenly spaced in the charging round, for sensor nodes’ potential communications, as shown in Fig. 5. Although a sensor node in low-duty WSNs may have few communication request in the charging round, the possibility that the whole network has communication request in the charging round is non-ignorable, making the reservation of the slots much necessary. 

Compared to RF-MAC in [11] where sensor nodes are charged on-demand, the above TDMA-based MAC has ease of implementation and flexibility, while RF-MAC requires complex interaction between sensor nodes and chargers. Moreover, due to the low efficiency of long-distance charging, it usually takes sensor nodes certain charging time to satisfy an energy demand. Hence, for each communication request, the sensor node with RF-MAC always has certain communication delay. And, the delay will be even higher when several nodes have requests simultaneously (which is very common in WSNs). Therefore, it is needed to charge sensor nodes in advance with TDMA instead of a hasty and crowded charging on-demand with RF-MAC, though the latter is more energy-efficient for the chargers. 

## 4 THE CCSP PROBLEM

## 4.1 Problem Formulation

In scenario where chargers cannot move around, we have to employ multiple chargers to cooperatively charge sensor nodes, which however inevitably leads to the nonlinear superposition effect. The nonlinear superposition charging effect may significantly increase the charging power at some sensor nodes while seriously decrease the charging power at other nodes. Hence, in order to efficiently charge each sensor node, it is needed to appropriately schedule the chargers. 

Since the charging round is partitioned into multiple charging periods by the time slots in the above TDMAbased MAC, to schedule the chargers, we need to ascertain each charger’s active time and inactive time in the charging periods. In consideration of the diversity of each charger’s clock, it is meaningless to set exact active time for each charger. Therefore, to simplify the scheduling instructions, we schedule the chargers’ active time with the unit of charging period $\Delta .$ That is to say, a charger keeps either active or inactive within one charging period, according to the scheduling instructions. Hence, the scheduling problem is actually that: how to appropriately set each charger active or not in each charging period so as to take the minimum number of charging periods to charge each sensor node with energy no less than E? 

To appropriately schedule each charger, it is usually needed to know the radio power that each charger brings to the sensor nodes, which can be called as the charger’s charging utility. However, as analyzed above, the compound radio power at a sensor node charged by a group of chargers, is usually not the sum of that when each charger works individually. Actually, in view of the compound radio power, the amount of power that an active charger brings to a sensor node, much depends on which other chargers are active. Therefore, it is meaningless to separately define or calculate the charging utility of each charger in advance. Thus, without knowing the charger’s charging utility at each node, it is challenging to decide the charger’s activity independently according to the current energy of sensor nodes. 

However, from the other point of view, if getting each charger’s state in each charging period, we can establish a series of charger sets, each of which consists of all active chargers in the charging period. With each set of active chargers, the compound radio power at each sensor node, i.e., the charging utility of the charger set, can be calculated with Equation (4). Thus, the objective of the scheduling can be regarded as finding a series of charger sets and determining when they are activated in turn, so that the total number of turns (i.e., charging periods) is minimized while guaranteeing the energy harvested by each node is no less than $E .$ 

Based on the analysis above, we formulate the concurrent charging scheduling problem as follows. 

Given: 

$\mathcal { C } = \{ c _ { i } | 1 \leq i \leq N \}$ , where $c _ { i }$ denotes the ith charger. 

$S = \{ s _ { j } | 1 \leq j \leq M \}$ g, where $s _ { j }$ denotes the jth sensor node. 

$\{ d _ { i j } | 1 \le i \le N , 1 \le j \le M \}$ , where $d _ { i j }$ is the distance between $c _ { i }$ and $s _ { j } .$ 

Assume: The energy capacity of each node is $E ,$ , and the size of each charging period is $\overset { \cdot } { \Delta }$ 

The problem is to find a family of active charger sets $S _ { 1 } , \dots , \hat { S } _ { p } , ( S _ { k } \subseteq \mathcal { C } , k = 1 , \dots , p )$ with the corresponding number of charging periods $\gamma _ { 1 } , \dotsc , \gamma _ { p }$ allocated for these sets such that: 

$\gamma _ { 1 } + \gamma _ { 2 } + \cdots + \gamma _ { p }$ is minimized, while subject to the following constraint: 

$\begin{array} { r } { \forall j = 1 , \dotsc , M , \sum _ { k = 1 } ^ { p } \gamma _ { k } u _ { j } ^ { k } \geq E , } \end{array}$ , where $u _ { i } ^ { k }$ is the charging utility of $S _ { k }$ at sensor node s<sub>j</sub>, i.e., $u _ { j } ^ { \check { k } } = e _ { j } | _ { S _ { k } , \Delta } .$ 

## 4.2 Complexity Analysis

We prove that the CCSP is NP-hard by proving that the decision version of the problem is NP complete which is defined as: given a threshold $k ,$ does there exist a collection of charger sets $\{ S _ { 1 } , S _ { 2 } , \dotsc , S _ { p } \} ( S _ { i } \subseteq { \mathcal { C } } , i = 1 , \dotsc , p )$ and the corresponding number of charging periods $\gamma _ { 1 } , \ldots , \gamma _ { p } ,$ , which satisfy the constraint above and $\gamma _ { 1 } + \gamma _ { 2 } + \cdots + \gamma _ { p }$ is equal or less than k? 

Proof. It is easy to prove this problem is NP since given a collection of charger sets and their corresponding numbers, the constraint and the sum of the numbers can be checked in a polynomial time. We show it is NP-complete by reducing the set cover problem [22] to it. The set cover problem is defined as: 

Given: A universe $U = \{ e _ { 1 } , e _ { 2 } , \dots , e _ { M } \}$ , a collection of subsets $\bar { C } = \{ \bar { S } _ { 1 } , \bar { S } _ { 2 } , \dots , \bar { S } _ { p } \}$ with where $\bar { S } _ { 1 } \cup \dots \cup$ $\bar { S } _ { p } = U ,$ , and a number k. 

Find: if there exist k subsets in $\bar { C }$ whose union covers U (in other words, each element in U is covered at least once). 

To reduce the set cover problem to the CCSP, we construct the sensor node set and a schedule of charger sets with corresponding charging utility at each sensor node from the inputs of set cover problem in the following way: 

1. According to $U ,$ construct a sensor node set $\left\{ s _ { 1 } , s _ { 2 } , \ldots , s _ { M } \right\}$ . Each node $s _ { j } , j = 1 , 2 , \ldots , M$ corresponds to the element $e _ { j } \in U$ 

2. For each $\bar { S } _ { i } \in \bar { C } , 1 \leq i \leq p ,$ construct a charger set $\hat { S } _ { i }$ with charging utility vector $\{ u _ { 1 } ^ { i } , u _ { 2 } ^ { i } , \ldots , u _ { M } ^ { i } \}$ where $u _ { j } ^ { i } \geq 0 \ ( 1 \leq j \leq M )$ is the charging utility at sensor node $s _ { j } .$ For each element in $U - { \bar { S } } _ { i } ,$ we set the corresponding node’s charging utility in $\hat { S } _ { i } ^ { \prime } { \bf s }$ utility vector to be zero, by assuming the compound radio power at the node with $\hat { S } _ { i }$ is less than the threshold δ. 

3. Adjusting the capacity of each node E ¼ <sup>min</sup> $\{ u _ { 1 } ^ { i } , \cdot \cdot \cdot , u _ { M } ^ { i } | 1 \le i \le p \}$ . This setting facilitates each charger set $\hat { S } _ { i }$ to take only one charging period to fully charge those sensor nodes with non-zero charging utility under $\hat { S } _ { i }$ 

With this transformation, it can be easily proved that: 

1. Assume, without loss of generality, $\{ \bar { S } _ { 1 } , \bar { S } _ { 2 } , \ldots ,$ ${ { \bar { S } } _ { k } } \rbrace$ is a solution to the set cover problem, then the collection of charger sets $\{ \hat { S } _ { 1 } , \hat { S } _ { 2 } , \ldots , \hat { S } _ { k } \}$ can satisfy the constraint of CCSP, and the number of charging periods assigned are $\gamma _ { 1 } = \gamma _ { 2 } = . ~ . . =$ $\gamma _ { k } = 1$ . Therefore, the total number of charging periods is $k .$ 

2. Assume a collection of charger sets $\hat { C } = \{ \hat { S } _ { 1 } , \hat { S } _ { 2 } , \cdot \cdot \cdot$ $\hat { S } _ { p } \}$ is constructed from the set cover problem, and we have a smaller collection of charger sets $\begin{array} { r } { \hat { C } ^ { \prime } = \{ \hat { S } _ { i } \} \subseteq \hat { C } \mathrm { w i t h } \sum _ { i } \gamma _ { i } = k } \end{array}$ as the solution to the CCSP problem. Then, the collection of subsets in $\bar { C } ,$ from which the elements in $\hat { C } ^ { \prime }$ are constructed, is a solution to the set cover problem. The detailed proof is omitted for brevity. tu 

## 5 PROPOSED METHODS

In this section, we propose two greedy algorithms to solve CCSP, one is based on submodular set cover problem (SSCP) [23] and another is based on the idea of balanced charging. In addition, we design the genetic algorithm for CCSP. 

## 5.1 A SSCP-Based Algorithm for the CCSP

In this section, we propose a method based on the SSCP. This method is divided into two stages. First, a set of candidate charger sets are enumerated. Then the problem is reduced to SSCP, based on which the classic greedy approximation algorithm for SSCP can be employed and we give the proof of the approximation ratio. 

with N. However, considering the fact that N is not too large in practice, we can enumerate all possible sets of chargers, and calculate the charging utility of each set at each sensor node in advance. We denote the collection of all possible charger sets and each set’s charging utility vector by: 

$$
\begin{array}{l} C _ {\mathcal {N}} = \{S _ {1}, S _ {2}, \dots , S _ {\mathcal {N}} \} \\ S _ {i}: \{u _ {1} ^ {i}, u _ {2} ^ {i}, \dots , u _ {M} ^ {i} \}, \end{array}\tag{6}
$$

where $S _ { i }$ is the ith set in $C _ { \mathcal { N } }$ and $\mathcal { N }$ is the total number of sets in $C _ { \mathcal { N } }$ . For convenience of computing, we suppose each candidate set in $C _ { \mathcal { N } }$ can charge sensor nodes with either zero energy or at least energy 1 during the charging period D. 

After all the possible charger sets have been enumerated, we transform CCSP to a SSCP problem by establishing a submodular set function $f ( * )$ as follows. 

In consideration of the fact that a fully-charged sensor node could not harvest energy any more, we first define harvesting utility of sensor node $s _ { j }$ with charger set $S _ { k }$ as 

$$
\sqcap_ {j} ^ {k} = \min \{u _ {j} ^ {k}, E - e r _ {j} \},\tag{7}
$$

where $e r _ { j }$ is the current energy stored in sensor node $s _ { j } .$ When the capacitor in $s _ { j }$ is full, i.e., $e r _ { j } = E ,$ , the harvesting utility of $s _ { j }$ with $S _ { k }$ is zero. 

Lemma 1. Given a collection of chargers sets $C _ { K } ,$ , define $\begin{array} { r } { f ( C _ { K } ) = \sum _ { k : S _ { k } \in C _ { K } } \sum _ { j = 1 } ^ { M } \prod _ { j } ^ { k } } \end{array}$ . Then, $f ( C _ { K } )$ is a submodular set function. 

Proof. According to the definition of harvesting utility $\Pi _ { j } ^ { k }$ in Equation $( 7 ) ,$ , as long as the capacitor of each sensor node is not full, we have $f ( C _ { K } ) + f ( \{ S _ { i } \} ) = f ( C _ { K } \cup \{ S _ { i } \} ) ,$ where $S _ { i }$ is any element in $C _ { \mathcal { N } } - C _ { K }$ . However, if $C _ { K }$ has resulted in the fullness of at least one sensor node’s capacitor, the harvesting utility of $S _ { i }$ at the sensor node is zero according to Equation $( 7 )$ . Thus, for this case, we have $f ( C _ { K } ) + { \overset { \smile } { f } } ( \{ S _ { i } \} { \overset {  } { ) } } > f ( C _ { K } \cup \{ S _ { i } \} ) , \quad { \mathrm { i . e . , } } \quad f ( C _ { K } \cup \{ S _ { i } \} ) -$ $f ( C _ { K } ) \ < \ f ( \{ S _ { i } \} )$ . Since $C _ { K }$ can potentially lead to the fullness of more sensor nodes’ capacity than $C _ { K ^ { \prime } } \subseteq C _ { K } ,$ we have $f ( C _ { K } \cup \{ S _ { i } \} ) - f ( C _ { K } ) \leq \bar { f } ( C _ { K ^ { ' } } \cup \{ S _ { i } \} ) - f ( C _ { K ^ { \prime } } ) ,$ $\mathrm { i . e . }$ , function $f ( * )$ has the property of decreasing marginal utility. Therefore, $f ( * )$ is a submodular set function. tu 

With establishing the submodular set function $f ( * ) .$ , we rewrite the CCSP in the form of SSCP as: Given a submodular function $f ( * )$ on $C _ { \mathcal { N } } ,$ find the smallest set $C _ { K } \subseteq C _ { \mathcal { N } }$ such that $f ( C _ { K } ) = f ( C _ { \mathcal { N } } )$ . To solve this SSCP problem, we employ the classic greedy approximation algorithm for SSCP [23] which can be described with Algorithm 1. 

Algorithm 1. Greedy CCSP Based on SSCP
1: Given: $C_{\mathcal{N}}$ and $\{u_1^i, \ldots, u_M^i | 1 \leq i \leq \mathcal{N}\}$ 2: $C_K \leftarrow \phi$ 3: while $f(C_K) \neq ME$ do
4: find $S_i \in C_N$ to maximize $f(C_K \cup \{S_i\}) - f(C_K)$ 5: $C_K \leftarrow C_K \cup \{S_i\}$ 6: end while
We give a simple example of four charger sets $S_1, S_2, S_3$ and $S_4$ and their charging utilities at four sensor nodes $s_1, s_2, s_3$ and $s_4$ shown in Table 1. Suppose the energy capacity of each node $E = 8$ . According to Algorithm 1, we first loaded on October 20, 2024 at 09:30:54 UTC from IEEE Xplore. Restrictions apply 

rging Given N chargers, the number of sets that the N chargers $s _ { 2 } , s _ { 3 }$ and $s _ { 4 }$ shown in Table 1. Suppose the energy capacity can be divided into is $\Sigma _ { i = 1 } ^ { N } \mathbf { C } _ { N } ^ { i }$ which grows exponentially of each node $E = 8 .$ . According to Algorithm 1, we first <sup>i¼</sup> <sup>N</sup>              Authorized licensed use limited to: Nanjing Univ of Post & Telecommunications. Downloaded on October 20,2024 at 09:30:54 UTC from IEEE Xplore. Restrictions apply. 


TABLE 1 Example of the Charging Utilities of Four Charger Sets at Four Sensor Nodes


<table><tr><td></td><td><eq>S_1</eq></td><td><eq>S_2</eq></td><td><eq>S_3</eq></td><td><eq>S_4</eq></td></tr><tr><td><eq>s_1</eq></td><td>4</td><td>0</td><td>0</td><td>1</td></tr><tr><td><eq>s_2</eq></td><td>0</td><td>4</td><td>0</td><td>1</td></tr><tr><td><eq>s_3</eq></td><td>0</td><td>0</td><td>4</td><td>1</td></tr><tr><td><eq>s_4</eq></td><td>0.5</td><td>0.3</td><td>0.2</td><td>1</td></tr></table>

select charger set $S _ { 1 } ,$ as it will provide the largest harvesting utility $\{ 4 , 0 , 0 , 0 . 5 \}$ (with submodular set function value 4.5) to the nodes. After the charging, the current energy stored in the nodes is $e r = \{ 4 , 0 , 0 , 0 . 5 \}$ To further charge the nodes, we again select $S _ { 1 }$ and the current energy of the nodes increases to be $e r = \{ 8 , 0 , 0 , 1 \}$ . Now, sensor node $s _ { 1 }$ becomes full. At this time, if still choosing $S _ { 1 }$ to charge the nodes, the harvesting utility that $S _ { 1 }$ can provide is only $\{ 0 , 0 , 0 , 0 . 5 \}$ . Hence, to further charge the nodes, according to Algorithm 1, we should select charger set $S _ { 2 }$ which provides the currently largest charging utility f<sup>0</sup>; <sup>4</sup>; <sup>0</sup>; <sup>0</sup>:<sup>3</sup>g. In this way, we get the sequence of the selected charger sets $\{ 2 * S _ { 1 } , \dot { 2 } * S _ { 2 } , \bar { 2 } * S _ { 3 } , 6 * \hat { S } _ { 4 } \}$ . The total number of charging periods is $2 + 2 + 2 + 6 = 1 2$ 

Theorem 1. Algorithm 1 is a ð<sup>ln</sup> ME þ <sup>1</sup>Þ approximation for CCSP. 

Proof. According to the definition of $f ( * ) , f ( C _ { \mathcal { N } } ) = M E$ . Let $x _ { i }$ denote the amount of energy harvested by sensor nodes with the ith set that Algorithm 1 picks (i.e., the sum of all harvesting utilities in the set). Let $z _ { i } = M E -$ $\textstyle \sum _ { j = 1 } ^ { i } x _ { i } ,$ , which means the amount of remaining energy required by the sensor nodes after i steps of Algorithm 1. According to the notations, $Z _ { 0 } = M E $ 

Suppose the optimal solution uses k sets to charge each node with $E , \mathrm { i . e . } ,$ the total energy is $M E ,$ we have: there exists at least one set in $C _ { \mathcal { N } }$ that must charge the nodes with at least $1 / k$ fraction of the total energy ME. Since Algorithm 1 always selects the set with the largest total harvesting utilities at each step, we have $\begin{array} { r } { x _ { 1 } \geq \frac { z _ { 0 } } { k } . } \end{array}$ Furthermore, since there exists a solution that uses only k sets to charge the nodes with total energy ME, for the remaining energy $z _ { i } \le z _ { 0 } = M E$ after the ith step of Algorithm 1, there must also exist a solution that uses only k sets to charge the nodes with total energy $z _ { i }$ (due to the monotone of the submodular set function). Thus, there exists at least one set that must charge the nodes with at least total energy $\begin{array} { r } { \frac { z _ { i } } { k } . } \end{array}$ . Hence, according to Algorithm 1, we have $\begin{array} { r } { x _ { i + 1 } \geq \frac { z _ { i } } { k } . } \end{array}$ 

Based on the result above, we have: 

$$
\begin{array}{l} z _ {i + 1} \leq z _ {i} - x _ {i + 1} \\ \quad \leq z _ {i} - \frac {z _ {i}}{k} = z _ {i} \bigg (1 - \frac {1}{k} \bigg) \\ \quad \leq z _ {i - 1} \bigg (1 - \frac {1}{k} \bigg) ^ {2} \leq \dots \\ \quad \leq z _ {0} \bigg (1 - \frac {1}{k} \bigg) ^ {i + 1} = \bigg (1 - \frac {1}{k} \bigg) ^ {i + 1} * M E. \end{array}\tag{8}
$$

Hence, after $\begin{array} { r } { i = k [ \mathrm { l n } \frac { M E } { k } ] } \end{array}$ steps of Algorithm 1, we have: 

$$
\begin{array}{l} z _ {i} \leq \left(1 - \frac {1}{k}\right) ^ {k \lceil \ln \frac {M E}{k} \rceil} * M E = \left[ \left(1 - \frac {1}{k}\right) ^ {k} \right] ^ {\lceil \ln \frac {M E}{k} \rceil} * M E \\ \leq \left(\frac {1}{e}\right) ^ {\ln \frac {M E}{k}} * M E = \frac {k}{M E} * M E = k. \end{array}\tag{9}
$$

Thus, after $\begin{array} { r } { i = k \lceil \ln \frac { M E } { k } \rceil } \end{array}$ steps, there are no more than k remaining energy that the sensor nodes need. Since each candidate set in $C _ { \mathcal { N } }$ can charge sensor nodes with at least total energy 1, at most k more steps, Algorithm 1 can finish. Therefore, the total number of steps of Algorithm 1 is less than $\begin{array} { r } { k \lceil \ln \frac { M E } { k } \rceil + k \leq k ( \ln M E + 1 ) } \end{array}$ , i.e., Algorithm 1 is a ð<sup>ln</sup> ME þ <sup>1</sup>Þ approximation for CCSP. tu 

## 5.2 Balanced Charging Algorithm for the CCSP

It can be seen, the proposed greedy CCSP algorithm always selects the charger set with the largest harvesting utility, hence the algorithm has the fastest charging speed at the beginning. However, the unilateral selection strategy in the greedy CCSP algorithm usually leads to continuous repetition of the same charger set, thus resulting in much unbalanced energy among the sensor nodes. Hence, some sensor nodes may get fully charged quickly while other nodes harvest very little energy. From this point of view, the greedy CCSP seems to exhibit a sequential-like characteristic in the charging of sensor nodes. 

In order to improve the performance of the greedy CCSP algorithm, we propose a balanced charging algorithm for CCSP (called as balanced CCSP). Algorithm 2 presents the balanced CCSP. At the beginning, the balanced CCSP chooses the charger set with the maximum harvesting utility. After that, the balanced CCSP takes a greedy selection strategy with a balanced metric between the harvesting utility on sensor nodes with the lowest harvested energy and the total harvesting utility on all sensor nodes (as shown in line 5 in Algorithm 2). 

Algorithm 2. Balanced CCSP Algorithm
1: Given: $C_{\mathcal{N}}$ and $\{u_1^i,\ldots ,u_M^i |1\leq i\leq \mathcal{N}\}$ 2: $C_K\gets \{\text{argmax}\sum_{j:s_j\in \mathcal{S}}\sqcap_j^k\}$ 3: while $f(C_K)\neq ME$ do
4: find a set containing $\lceil \alpha M\rceil$ sensor nodes that currently have the lowest energy, and denote the set as $\Gamma$ 5: find $S_{i}\in C_{\mathcal{N}}$ to maximize $\sum_{j:s_j\in \Gamma}\sqcap_j^k +\alpha \sum_{s_j\in \mathcal{S}}\sqcap_j^k$ 6: $C_K\gets C_K\cup \{S_i\}$ 7: end while 

We utilize the example in Table 1 to show the process of the balanced CCSP algorithm. The parameter a is set to be $1 / 4$ . According to the balanced CCSP algorithm, charger set $S _ { 1 }$ is chosen at first. Then, $S _ { 2 }$ and $S _ { 3 }$ are selected one after another. After that, the current energy stored in the nodes is $e r = \{ 4 , 4 , 4 , 1 \}$ . Hence, the balanced CCSP algorithm keeps selecting $S _ { 4 }$ seven times, and then all the sensor nodes become full. It can be seen, the total number of charging periods is 10, which is smaller than that $( \mathrm { i } . \mathrm { e } . , 1 2 )$ with the SSCP-based greedy CCSP algorithm. 

## 5.3 The GA Method for the CCSP

Besides the greedy algorithms present above, we additionally propose a Genetic Algorithm (GA) to solve the CCSP. Generally, GA method can achieve better results than the greedy methods, but taking much more computation time [24]. 

The first step in designing a GA for the CCSP is to devise a suitable representation scheme to represent solutions of CCSP. Since in CCSP each charger’s utility cannot be calculated independently, the representation scheme should take charger set as unit to represent the solution. A straightforward way for the representation scheme is to list all the possible charger sets and denote whether they are chosen or not. However, as analyzed above, the number of all possible charger sets grows exponentially with $N ,$ which may make the representation of the solution too long. Hence, considering the limited number of charging periods, we design the representation scheme as $\hat { C } _ { N } = \{ S _ { 1 } , S _ { 2 } , \ldots , S _ { K } \}$ , where $S _ { i }$ denotes the active charger set in the ith charging period, and K is an upper bound of the number of charging periods. We use a N-bits binary string to represent $S _ { i } .$ In this representation, a value of 1 or 0 at the jth bit of the string implies that whether the jth charger in $\mathcal { C }$ is selected or not. Each $S _ { i }$ is constructed by randomly generating a N-bits binary string for $S _ { i }$ . The case that all bits in the string are 0 is excluded. In this way, a solution of CCSP can be denoted by a N  K-bits binary string. 

For the upper bound $\kappa ,$ we need to appropriately selecting its value. Smaller $\kappa$ is always more favorable from computational point of view, but leading to high ratio of infeasible solutions. We compute $\kappa$ as: $\begin{array} { r } { \dot { K } = \sum _ { s _ { j } \in S } [ E / u _ { j } ^ { 0 } ] } \end{array}$ where $u _ { j } ^ { 0 }$ is the charging utility at $s _ { j }$ of the charger set that contains only the charger which is the closest to $s _ { j } .$ . It can be seen, K represents the number of charging periods required in a very bad case that each sensor node is charged only by the closest charger. Apparently, the number of charging periods in the optimal solution is much smaller than $\kappa .$ 

In addition, we notice that some charger sets should always be selected in a feasible solution (i.e., the sum of utilities of all sets in $\hat { C } _ { N }$ at each sensor node should be no less than E), while some other charger sets are definitely unworthy to be chosen. For example, if there exists a sensor node at which only one charger set’s charging utility is non-zero, the charger set must be selected (otherwise the node cannot be charged) and work for several charging periods till fully charge the node. In addition, if the charging utility of charger set S at each sensor node is always smaller than that of another charger set, S is obviously unworthy to be chosen. Based on these considerations, we refine the initial population as follows. 

First, we construct a big matrix A with M rows and $\textstyle \sum _ { i = 1 } ^ { N } \mathbf { C } _ { N } ^ { i }$ columns. Each column $j$ denotes the charging utility vector of charger set $S _ { j } ,$ and each row i corresponds to one sensor node $s _ { i } .$ . That is to say, $a _ { i j } = u _ { i } ^ { j }$ . We take a matrix reducing algorithm on A shown in Algorithm 3. Lines 9, 10 in the algorithm mean that the charging of sensor node $s _ { i }$ does not need to be concerned with, because the charging utility of any charger set at $s _ { i }$ is always larger than that at another node $s _ { i ^ { \prime } }$ (which means that any solution satisfying s must also satisfy $s _ { i } )$ . We put the charger sets selected by 

$$
\mathcal {S} _ {i ^ {\prime}}
$$

Algorithm 3 into the initial population $\hat { C } _ { N } .$ , and restrict $\hat { C } _ { N }$ to generating the charger sets deleted in Algorithm 3. 

Algorithm 3. Matrix Reducing Algorithm
1: Input: matrix A
2: Partial solution $J \leftarrow \phi$ 3: The current energy request $\{er_{i} = E\}$ 4: while $A \neq 0$ do
5: if row i has exactly one non-zero element, say, in column j then
6: $J = J \cup \{S_{j}\}$ , and eliminate column j
7: $er_{k} = er_{k} - \left\lceil \frac{er_{i}}{a_{ij}} \right\rceil * a_{kj}$ for each $1 \leq k \leq M$ .
8: end if
9: if row i “dominates” row i', i.e., for all columns, $a_{ij} \geq \frac{er_{i}}{er_{i'}} a_{i'j}$ then
10: eliminate row i.
11: end if
12: if column j is “dominated” by column j', i.e., for all rows, $a_{ij} \leq a_{ij'}$ then
13: eliminate column j, and delete $S_{j}$ from $C_{N}$ .
14: end if
15: Break, if no rows or columns are eliminated in the last iteration
16: end while 

Though Algorithm 3 is applied, $\hat { C } _ { N }$ is still possible to be an infeasible solution to CCSP. To this end, we further define the fitness of a solution as follows. For each feasible solution $\hat { C } _ { N } = \{ S _ { 1 } , S _ { 2 } , \ldots , S _ { K } \}$ , find the smallest value $m \leq \kappa$ satisfying that $\{ S _ { 1 } , S _ { 2 } , \ldots , S _ { m } \}$ is also a feasible solution, i.e., $\begin{array} { r } { \sum _ { i = 1 } ^ { m } u _ { j } ^ { \bar { i } } \ge E } \end{array}$ for each $1 \leq j \leq M$ . We define the fitness of $\hat { C } _ { N }$ to be $1 / m _ { ☉ }$ , and regard $\{ S _ { 1 } , S _ { 2 } , \ldots , S _ { m } \}$ as $\hat { C } _ { N } { ' } _ { \mathrm { { S } } }$ output which takes m charging periods. The smaller $m ,$ , the higher the fitness of the solution. 

Having determined the fitness function, now we can start the evolution process. We first generate, at random, a population of $N * { \hat { K } } { \cdot } \mathrm { b i t s }$ binary strings (i.e., the chromosomes). The chromosomes will be used for crossover and mating to create the next generation of chromosomes. We take the crossover between any two chromosomes in following way. For each valid chromosome (i.e., feasible solution), we partition the chromosome into two parts: the payload in the front part which is the output of the solution, and the redundant genes in the hind part (called as redundant part). To take the crossover between chromosome A and B, we exchange one randomly selected gene in the payload of chromosome $A$ and one randomly selected gene in the redundant part of chromosome $B ,$ as shown in Fig. 6a. Note that, the genes corresponding to the charger sets selected in Algorithm 3 should always be put in the payload and not be selected for the mutual crossover. After that, some chromosomes will further take crossover within their payload parts. And finally, a small portion of the chromosomes are selected for mating as well as mutation. 

To refine the selected chromosomes, we additionally take a modification on them as follows. For each selected chromosome, check each gene in the payload part sequentially from the front to the end whether deleting the gene still keeps the payload part valid or not. If it does, then move the gene to the end of the chromosome, as shown in Fig. 6b. 

![](images/c2e504d811717eb7e7ff7328174d482245e727a5a8daf2b8feb490ce1d5e093a.jpg)



Fig. 6. (a) Crossover between the two parts of chromosomes. (b) Refine the chromosome.


This modification makes the length of the payload part of the chromosome shorter, thus helping accelerate the convergency of the algorithm. 

With the crossover and mating, we create all the next generation of chromosomes. Then, the two generations of chromosomes are both evaluated by the fitness function. The fitter chromosomes will be selected for further crossover and mating. The iteration stops if there is no improvement in the maximum fitness value for ten consecutive generations. 

## 6 PERFORMANCE EVALUATION

## 6.1 Simulations

To evaluate the performance of the proposed methods, we conduct a series of simulations with Matlab tool, by generating a series of random deployments of chargers and sensor nodes within <sup>50 m</sup>  <sup>50 m</sup> area. In the simulations, we employed the energy harvesting model present in Section 3. With the deployments and the harvesting model, the charging utility of each possible charger set can be calculated, and procedures for the proposed three algorithms can be directly executed in Matlab. 

We assumed in the simulations that all the sensor nodes have the same capacity E and initially their capacitors are all empty. The transmission power P of each charger is also the same and set to be <sup>4</sup> <sup>W</sup>. The transition efficiency r is set to be 0.25. The threshold of harvesting power is $\delta = 1 5 ~ \mu \mathrm { W }$ We set the size of each charging period to be D ¼ <sup>20s</sup> and the wave length $\lambda = 3 * 1 0 ^ { 8 } ~ \mathrm { m } / 9 1 5 ~ \mathrm { M H z } \approx 0 . 3 3$ <sup>m</sup>. Based on these parameters, a sensor node ten meters away from a charger, can harvest $0 . 2 5 * 4 * 2 0 / { ( 4 \pi * 1 0 / \lambda ) } ^ { 2 }$ <sub></sub> 0<sub>:</sub>14 mj (joule) energy during one charging period. For reference, a 


TABLE 2



Parameters Set in the Simulations


<table><tr><td>P</td><td>ρ</td><td>Δ</td><td>Wave length</td><td>Area size</td><td>Threshold of harvesting power</td></tr><tr><td>4 W</td><td>0.25</td><td>20 s</td><td>0.33 m</td><td>50 m * 50 m</td><td>15 μW</td></tr></table>


TABLE 3



Comparison of the Energy Emitted by Chargers


<table><tr><td></td><td>Greedy CCSP</td><td>Balanced CCSP</td><td>GA</td></tr><tr><td>Total energy emitted</td><td>18.88 kj</td><td>20.24 kj</td><td>16.32 kj</td></tr></table>

ZigBee packet transmission generally consumes energy about <sup>20 mA</sup>  <sup>3</sup>:<sup>3 V</sup>  <sup>5 ms</sup> ¼ <sup>0</sup>:<sup>33 mj</sup>. It can be seen, to supply one packet transmission, three charging periods are needed for a sensor node charged by a charger ten meters away. Table 3 summarizes the parameters setting. 

Fig. 7a gives an example of placement with 12 chargers and 50 sensor nodes randomly deployed in <sup>50 m</sup>  <sup>50 m</sup> area. We show the radio power at each node respectively charged by two typical charger sets with Figs. 7b and 7c. One is the set consisting of all chargers, and another is the set with the largest charging utility. It can be seen, the distri bution of the radio power is much uneven in the area. The radio power at some nodes can be hundreds times of that at other nodes. For the set consisting of all chargers, the harvesting utility at several nodes is zero, which means that those nodes could hardly harvest any energy if keeping turning on all chargers. However, turning off some chargers can make things different. Therefore, scheduling the chargers is inevitable for the charging of all sensor nodes. 

We apply the greedy CCSP algorithm, the balanced CCSP and the proposed GA algorithm on the above placement and set the capacity of sensor node $E = 4 ~ \mathrm { m j }$ . The results are shown in Fig. 8a. The vertical axis represents the number of sensor nodes having been fully charged, and the horizontal axis represents the ID of charging period. From the results, we can see that chargers with the GA algorithm can charge the sensor nodes the most quickly with 30 charging periods, while the greedy CCSP algorithm and the balanced CCSP need 35 and 33 charging periods, respectively. In particular, during the first 15 charging periods, more nodes get fully charged with the greedy CCSP algorithm than that with the balanced CCSP or the GA algorithm. This is because the greedy CCSP always chooses the charger set with the maximum sum of harvesting utility (called as total harvesting utility), thus making it has quicker charging speed at the beginning. However, in the following charging periods, the balanced CCSP and the GA algorithm performs better than the greedy CCSP. 

To clearly show the difference on charger set selection of the three methods during the scheduling, Fig. 8b further gives the total harvesting utility of the charger sets in each charging period for the three methods. It can be seen, the greedy CCSP provides the higher total harvesting utility to the sensor nodes at the beginning. However, with some nodes get full, the total harvesting utility that greedy CCSP can provide becomes smaller and smaller, though it still keeps selecting the charger set with the highest total harvesting utility currently. For the GA algorithm, it does not provides much high total harvesting utility at the beginning, however it can provide relatively higher total harvesting utility in the following periods, thus getting earlier to fully charge the nodes. 

Fig. 8c additionally shows, for each charger, the number of charging periods during which the charger is active (called as charging periods during which the charger is active (called as loaded on October 20,2024 at 09:30:54 UTC from IEEE Xplore. Restrictions apply active periods). In can be seen, the chargers have different numbers of active periods. Some chargers need to be active for more than 30 periods, while some of them (e.g., charger 7 for the proposed GA) keeps active only for 6 periods. 

![](images/dace497f5be4cf418cc08676607c24a491d10b918b378318a796d1acc90f8c45.jpg)



(a) The placement


![](images/ebd63070526c1c79fea4fcf3a0c01f6d44502471f70ff3638a909f4524b3430e.jpg)



(b) Radio power at nodes concurrently charged by the set with all 12 chargers


![](images/21b3ef63cb92648b836d0e2836643e80b4cace4e8234dc416ad4032555c03f2d.jpg)



(c) Radio power at nodes concurrently charged by the set with the largest charging utility



Fig. 7. An example of concurrent charging with 12 chargers and 50 nodes.


![](images/6316e8e6bda4a72ddc62fc976e01512db8f87eae5ff4ee6610e02899b6ed0ce4.jpg)



(a) Charging speed


![](images/aacfdfdb8d54b6223e6dd8c4b0549f13cd3311316c475068bcb58e87171da526.jpg)



(b) Harvested energy


![](images/2ba636db635fe7277356a5d360de493800f2c56a0c5d6b7e468be18026d57a6a.jpg)



(c) Active periods



Fig. 8. Results of the three methods when N ¼ <sup>12</sup>, M ¼ <sup>50</sup>, and E ¼ <sup>4 mj</sup>.


In addition, Table 3 lists the total transmission energy emitted from all chargers respectively with the three algorithms. From the table, the chargers with the balanced CCSP consume a little more transmission energy than the greedy CCSP. However, as the chargers are fixed and connected to the power line in practice, the relatively higher energy consumption with the balanced CCSP can be acceptable. 

To evaluate the three proposed methods comprehensively, we take more simulations with different parameters on the scale of chargers and sensor nodes as well as the size of nodes’ energy capacity. Both the number of charging periods and the running time of the methods are evaluated. The simulations are executed on a computer with CPU type Intel i7-4790 <sup>3</sup>:<sup>6 GHz</sup> and <sup>8G</sup> RAM. For each parameter set, 10 simulations are performed and the average result of each method is calculated 

We first compare the proposed GA and a brute force algorithm at a small scale, as shown in Fig. 9 (where the number of chargers is only 7 and E ¼ <sup>3 mj</sup>). Although the charger scale is quite small, the running time of the brute force algorithm is generally no less than 10 hours. From Fig. 9, the proposed GA achieves almost the same performance as that of the brute force algorithm, except for the case when the number of nodes is 20. Hence, we take the GA as a reference to evaluate the performance of the greedy 

Fig. 10 shows the results at different scales of sensor nodes while the number of chargers and nodes’ capacity are fixed to be 10 and <sup>4 mj</sup>, respectively. It can be seen, with the increment of sensor nodes’ scale, generally more charging periods are required. However, when node scale is 50, there are sudden changes on the plots. This is because that the CCSP problem is a little sensitive to the deployment of sensor nodes and chargers. There are several typical deployments at M ¼ <sup>50</sup>, making the performance of the three scheduling methods change remarkably. However, comparing these three scheduling methods, the performances of greedy CCSP and balanced CCSP are always very close to that of GA. As for the time complexity, it can be seen, the running times of the greedy CCSP and the balanced CCSP are much lower than that of the proposed GA. 

![](images/5f47b1a6ca8fb83c8399aab22b067a9afb217a0e490aaf102e209889b7a6c0d2.jpg)



Fig. 9. Comparison between GA and brute force algorithm.


![](images/62505e427071a389d630ece04179105c86fe471751c8b14caf797163525040aa.jpg)



(a) Charging Periods vs. Node Scale


![](images/48a79a82ab5ee7c50d69d5bbf775c7c97bc76b264ef61ad8652bd36ce4047459.jpg)



(a) Charging Periods vs. Charger Scale


![](images/3b7b52b8a7bb1121bbea2e28cbf3354b014273f96957ab14a7c594485de50797.jpg)



(b) Running Time vs. Node Scale


![](images/789f3bc6bbcbd0e611716a1c7e8ae7a28ae1e9d992a6016398eab68acf3a5aed.jpg)



(b) Running Time vs. Charger Scale



Fig. 10. Node Scale.


Fig. 11 shows the results at different scales of chargers while the number of nodes and their capacity are fixed to be 50 and <sup>4 mj</sup>, respectively. It can be seen, with the increment of chargers’ scale, generally less charging periods are needed. This is as expected, as there are more chargers providing radio energy. The performances of greedy CCSP and balanced CCSP are still very close to that of GA. Moreover, with the increment of the chargers scale, the superiority of the proposed GA becomes more trivial. Especially, when the chargers scale is 14 and 15, the balanced CCSP even slightly outperforms the GA. This is because, larger charger scale leads to large population space in the GA algorithm, hence making the evolution more possible converge to a local optimal result. For the running times of the three algorithms, they all increase with the charger scale, which is as expected. 

Fig. 12 further shows the results for different nodes’ energy capacities when charger scale and node scale are 10 and 50, respectively. It can be seen, the increment of nodes capacity also leads to the increment of charging periods needed by the three methods. And, the greedy CCSP and balanced CCSP always achieve good performances close to that of GA, while still taking much lower running time than the latter 


Fig. 11. Charger Scale.


From Figs. 10, 11, and 12, we find that the balanced CCSP usually performs a little better than the greedy CCSP. We’d like to additionally give an insight on the differences between these two algorithms. According to the concept of harvesting utility, when the sensor nodes gradually get fully charged, the harvesting utilities of charger sets decrease correspondingly. Since the greedy CCSP keeps selecting the charger set with the highest harvesting utility, it leads to more distinct difference among the sensor nodes’ energy. As a result, some nodes are too early to be fully charged, making the harvesting utilities of the charger sets decrease more sharply. Due to this sharp decrement of harvesting utility, the greedy CCSP needs to take more time to fully charge the sensor nodes, i.e., it converges relatively slowly. From this point of view, to solve the CCSP problem, it is needed to find an appropriate tradeoff between the amount of harvesting utility and the decrement of harvesting utility. That’s why the balanced CCSP algorithm could have better performance than the greedy CCSP. 

![](images/0cba5db8812ef29e150365a677e5fcaf54a008e86195b28dccc116200297254a.jpg)



(a) Charging Periods vs. Energy Capacity


![](images/49d5cf4c29164ed0ab1089a1d374d8198be4ce95c3b52fd7434a935ff5591765.jpg)



(b) Running Time vs. Energy Capacity



Fig. 12. Energy Capacity.


## 6.2 Discussion of Some Practical Issues

We further discuss some non-ideal issues in practice. Although the transition coefficient r can be approximately assumed constant in optimized harvesting circuit, we’d like to discuss how the slight variation of r in imperfect harvesting circuit affect the performance of above algorithms. For the usual case that r slightly increases with the input RF power, high RF power at sensor nodes will lead to even larger harvesting power. Hence, the charging utility of a charger set will become more unbalanced at the sensor nodes. Since the greedy CCSP performs worse due to the more unbalanced charging strategy, the increasing r will enlarge the performance gap between the greedy CCSP and the balanced CCSP. For the same reason, the case of decreasing r will mitigate the performance gap between the greedy CCSP and the balanced CCSP. 

In low-duty WSNs, each sensor node may have little communication demand within the charging rounds. If occasionally a sensor node has a communication demand, the node will perform the communication at the earliest communication slot, thus consuming some energy. This energy reduction may affect the performance of the scheduling algorithms. However, compared to the balanced Authorized licensed use limited to: Nanjing Univ of Post & Telecommunications. Do 

![](images/bc086500ed5fa0ff016d9b6cb6381d23a76191c8a0b60d4c90b01cddeb968377.jpg)



Fig. 13. Scenario of the experiments.


CCSP, the greedy CCSP’s performance less likely suffers from the energy reduction, as more charging utilities of charger sets in greedy CCSP had not been made full use of (which can just right mend the energy consumption). In other word, the communication of sensor nodes within charging rounds can reduce the performance gap between the greedy CCSP and the balanced CCSP. 

In addition, we discuss the relationship between the chargers’ power and the performance of the algorithms. Larger chargers’ power obviously accelerates the charging round. However, larger chargers’ power provides more coarse-grained charging utility, thus aggravating the disadvantage of the greedy CCSP. Low chargers’ power apparently provides fine-grained charging utility, however it prolongs the charging round. We take example of the parameter used in Fig. 9. When doubling the chargers’ power to be <sup>8</sup>W, the greedy CCSP, the balanced CCSP and the GA need 17, 16 and 15 charging periods, respectively. When halving the chargers’ power to be <sup>2</sup>W, the three algorithms need 70, 65 and 59 charging periods, respectively. 

## 6.3 Experiments

To verify the proposed methods, we made some experiments with small scale of chargers and sensor nodes. The testbed consists of a RF signal generator (served as a reference radio source), three radio chargers and four radio harvesting motes, which are manufactured by us. The radio chargers do not generate radio themselves but amplify the input RF signal received from the unique reference radio source, then double the signal’s frequencies. After that, the chargers amplify the signal power to be about <sup>40mW</sup> and then transmit the signal. It can be seen, the chargers’ initial phases can be determined by the distances between the reference radio source and the chargers. In the experiments, we tune the center frequency of the generator’s signal to be <sup>457</sup>:<sup>5 MHz</sup>, thus to facilitate the chargers to transmit radio with center frequency <sup>915</sup> <sup>MHz</sup>. 

We placed the chargers and motes on a table with several kinds of topologies. The scenario is shown in Fig. 13, and the topologies are illustrated in Fig. 14. For each placement, we record the charging utility of each charger set at each mote by measuring the mote’s output voltage after one charging period. As observed in [11], [12], the charging utility of charger sets in our experiments also exhibits remarkable nonlinear superposition. 

In the experiments, the size of each charging period is set to be <sup>10s</sup>. The energy harvested by the motes is stored in the loaded on October 20,2024 at 09:30:54 UTC from IEEE Xplore. Restrictions apply capacitor with capacitance <sup>220</sup> mF and can be measured with the voltage of the capacitor. We measure the voltage with a digital multimeter. As long as the voltage increases to <sup>3</sup>v, we regard that the mote is fully charged. At this time, the harvested energy of the mote is about $\begin{array} { r } { \frac { 1 } { 2 } * 2 2 0 \mu F * ( 3 v ) ^ { 2 } \approx 1 } \end{array}$ mj<sub>.</sub> 

![](images/443b0277caec2b05ec745121995733fe4d884671e2aa7d914fd520f9ee753545.jpg)



Fig. 14. Six topologies of the placement.


We tested five methods: 1) non-scheduling method with which all the chargers keep active without scheduling; 2) single-charging method with which each mote is charged one by one with the closest charger; 3) the greedy CCSP; 4) the balanced CCSP; and 5) the proposed GA algorithm. Due to the small scale of chargers and motes, the result of the GA algorithm actually stands for the theoretical optimal solution. 

To implement the schedule results of the greedy CCSP, the balanced CCSP and the GA on the chargers, a coordinator is required, which helps to instruct each charger to be active or inactive in each charging period. Since the main purpose of the experiments is to validate whether the scheduling results can satisfy the charging requirement (i.e., fully charge the motes) or not, we just schedule (i.e., turn on/off) the chargers by artificially pressing the switch buttons on the chargers according to the schedule results of the scheduling methods in each charging period. In the future, the RF generator above can serve as the coordinator, with executing the scheduling methods and broadcasting the schedule results to the chargers at the beginning of the charging round. 

With the above implementation in the experiments, we find that all the proposed three scheduling methods can guarantee the motes’ voltage to exceed <sup>3</sup>v, with no more than 5 charging periods. As for the non-scheduling method and single-charging method, we just record the number of charging periods spent on fully charging the motes. Fig. 15 compares the numbers of charging periods required in the five methods. 

From Fig. 15, the performance of the scheduling methods are much sensitive to the placements. Though non-scheduling method keeps all chargers active, it usually does not perform well due to the destructive interference at some motes (e.g., the case of placement 1). For placement 2 and 5, as all the motes are in constructive interference, non-scheduling method performs as well as the proposed GA method. Single-charging method aims at avoiding the radio interference, making it perform not well for placement 2 and 5. For the greedy CCSP, it usually has the same performance as the balanced CCSP, except for placement 1 and 4. This is because that the greedy CCSP keeps selecting the charger set with the largest total harvesting utility but much small utility at some mote at the early periods. Hence, the charging rate at the mote with small harvesting utility is low. The balanced CCSP usually performs as well as the proposed GA, except for the case of placement 6. This is because that the balanced CCSP chooses the charger set with the largest total utility instead of balanced utility in its first step. 

![](images/0c7250d5316169be73de235fa8bab244382e2a3855a8fda589cefb55764f8037.jpg)



Fig. 15. Experimental results with the five scheduling methods.


Though the proposed scheduling methods implement “fully” charging the motes within expected charging periods, we notice that the energy harvested by the motes is a little smaller than that calculated theoretically. The reason is that the charging utility of the charger sets in practice is not a constant, and exhibits a slightly decreasing attribute during the charging of the motes. The longer the charging periods, the more remarkable decrement of charging utility will be. This encourages a future work on long-term charging the motes. 

## 7 CONCLUSION

Concurrent charging with fixed chargers is a promising way to charge sensor nodes in scenario where mobile charge cannot move freely. A typical characteristic of concurrent charging is the radio interference which has been verified by existing experimental works. In this paper, with addressing the nonlinear superposition charging effect caused by the radio interference, we study using efficient scheduling schemes to accelerate the concurrent charging for a group of sensor nodes. Based on an effective model of the nonlinear superposition charging effect, we formulate the concurrent charging scheduling problem and prove that it is NP-hard by reducing set cover problem to it. To solve the problem, we proposed a greedy CCSP algorithm based on the submodular set cover problem and give the proof of approximation ratio. We also design another heuristic algorithm, i.e., the balanced CCSP, as well as a genetic algorithm for CCSP which performs almost as well as the brute force algorithm at small network and charger scale. Both simulations and experiments are conducted. The results show that the greedy CCSP and the balanced CCSP can both achieve good performance close to that of GA, while taking much less running time than the latter. 

## ACKNOWLEDGMENTS

The work presented in this paper was supported in part by the NSF of China with Grant 61272053, 61572217, and 61572218. Xuefeng Liu is the corresponding author 

## REFERENCES



[1] S. Basagni, M. Y. Naderi, C. Petrioli, and D. Spenza, “Wireless sensor networks with energy harvesting,” in Mobile Ad Hoc Networking: Cutting Edge Directions, Hoboken, NJ, USA: Wiley, Mar. 5, 2013, ch. 20, pp. 703–736. 





[2] W. Ouyang, C. W. Yu, C. Huang, and T. H. Peng, “Optimum partition for distant charging in wireless sensor networks,” in Proc. 7th Int. Conf. Mobile Ad-Hoc Sensor Netw., 2011, pp. 413–417. 





[3] Y.-J. Hong, J. Kang, S. J. Kim, S. J. Kim, and U.-K. Kwon, “Ultralow power sensor platform with wireless charging system,” in Proc. IEEE Int. Symp. Circuits Syst., 2012, pp. 978–981. 





[4] Z. Li, Y. Peng, D. Qiao, and W. Zhang, “Joint charging and rate allocation for utility maximization in sustainable sensor networks,” in Proc. 11th Annu. IEEE Int. Conf. Sensing Commun. Netw., 2014, pp. 459–467. 





[5] C. Wang, J. Li, F. Ye, and Y. Yang, “Improve charging capability for wireless rechargeable sensor networks using resonant repeaters,” in Proc. 35th Int. Conf. Distrib. Comput. Syst., 2015, pp. 133–142. 





[6] K. Li, H. Luan, and C.-C. Shen, “Qi-ferry: Energy-constrained wireless charging in wireless sensor networks,” in Proc. IEEE Wireless Commun. Netw. Conf., 2012, pp. 2515–2520. 





[7] L. He, L. Kong, Y. Gu, J. Pan, and T. Zhu, “Evaluating the ondemand mobile charging in wireless sensor networks,” IEEE Trans. Mobile Comput., vol. 14, no. 9, pp. 1861–1875, Sep. 2015. 





[8] S. He, J. Chen, F. Jiang, D. K. Y. Yau, G. Xing, and Y. Sun, “Energy provisioning in wireless rechargeable sensor networks,” IEEE Trans. Mobile Comput., vol. 12, no. 10, pp. 1931–1942, Oct. 2013. 





[9] P. Nintanavongsa, U. Muncuk, D. R. Lewis, and K. R. Chowdhury, “Design optimization and implementation for RF energy harvesting circuits,” IEEE J. Emerging Select. Topics Circuits Syst., vol. 2, no. 1, pp. 24–33, 2012. 





[10] M. Y. Naderi, K. R. Chowdhury, and S. Basagni, “Wireless sensor networks with RF energy harvesting: Energy models and analysis,” in Proc. IEEE Wireless Commun. Netw. Conf., 2015, pp. 1494–1499. 





[11] M. Yousof Naderi, P. Nintanavongsa, and K. R. Chowdhury, “RF-MAC: A medium access control protocol for re-chargeable sensor networks powered by wireless energy harvesting,” IEEE Trans. Wireless Commun., vol. 13, no. 7, pp. 3926–3937, Jul. 2014. 





[12] M. Yousof Naderi, K. R. Chowdhury, and S. Basagni, “Experimental study of concurrent data and wireless energy transfer for sensor networks,” in Proc. IEEE Global Commun. Conf., Dec. 2014, pp. 2543–2549. 





[13] Y. Peng, Z. Li, G. Wang, W. Zhang, and D. Qiao, “Prolonging sensor network lifetime through wireless charging,” in Proc. IEEE RTSS, Nov. 30–Dec. 3 2010, pp. 129–139. 





[14] B. Tong, Z. Li, G. Wang, and W. Zhang, “How wireless power charging technology affects sensor network deployment and routing,” in Proc. IEEE 30th Int. Conf. Distrib. Comput. Syst., Jun. 2010, pp. 438–447. 





[15] X. Ren, W. Liang, and W. Xu, “Maximizing charging throughput in rechargeable sensor networks,” in Proc. 23rd Int. Conf. Comput. Commun. Netw., 2014, pp. 1–8. 





[16] L. Jiang, H. Dai, X. Wu, and G. Chen, “On-demand mobile charger scheduling for effective coverage in wireless rechargeable sensor networks,” Mobile Netw. Appl., vol. 19, no. 4, pp. 543–551, 2014. 





[17] C. Wang, J. Li, F. Ye, and Y. Yang, “Recharging schedules for wireless sensor networks with vehicle movement costs and capacity constraints,” in Proc. 11th Annu. IEEE Int. Conf. Sensing Commun. Netw., 2014, pp. 468–476. 





[18] H. Dai, G. Chen, C. Wang, S. Wang, X. Wu, and F. Wu, "Quality of energy provisioning for wireless power transfer,” IEEE Trans. Parallel Distrib. Syst., vol. 26, no. 2, pp. 527–537, Feb. 2015. 





[19] A. Madhja, S. Nikoletseas and T. P. Raptis, "Hierarchical, collaborative wireless charging in sensor networks,” in Proc. IEEE Wireless Commun. Netw. Conf., 2015, pp. 1285–1290. 





[20] L. Xie, Y. Shi, Y. T. Hou, W. Lou, H. D. Sherali and S. F. Midkiff, “Multi-node wireless energy charging in sensor networks,” IEEE ACM Trans. Netw., vol. 23, no. 2, 2015, pp. Apr. 437–450. 





[21] (2015). [Online]. Available: http://www.energous.com 





[22] M. Garey and D. Johnson, Computers and Intractability: A Guide to the Theory of NP-Completeness. San Francisco, CA, USA: Freeman, 1979. 





[23] V. V. Vazirani, Approximation Algorithms. Berlin, Germany: Springer, 2003. 





[24] D. E. Goldberg and J. H. Holland, “Genetic algorithms and machine learning,” Mach. Learn., vol. 3, no. 2, pp. 95–99, 1988. 



![](images/913a3d869212e34bbc9da5c88ba1a4b95d777d775782239f0eb08290135ba72f.jpg)


Peng Guo received the MS and PhD degrees from the Huazhong University of Science and Technology, Wuhan, China, in 2003 and 2008, respectively. He is currently an associate profes sor in the School of Electronic Information and Communications, Huazhong University of Science and Technology. His research interests include wireless sensor networks, distributed computing, and in-network processing. He has served as a reviewer for several international journals/conference proceedings. 

![](images/9984151aa05859356d4c6e8741249b9cdeb8fd2d37c3a905f898ca0205d3af8b.jpg)


Xuefeng Liu received the MS and PhD degrees from the Beijing Institute of Technology, China, and University of Bristol, United Kingdom, in 2003 and 2008, respectively. He is currently an associate professor in the School of Electronic Information and Communications, Huazhong University of Science and Technology. His research interests include wireless sensor networks and in-network processing. He has served as a reviewer for several international journals/conference proceedings. 

![](images/a40616f0d69b3708701966a3480f99f592babb7d8f0ebe6eab8f921151837a99.jpg)


Shaojie Tang received the PhD degree in computer science from the Illinois Institute of Technology, in 2012. He is currently an assistant professor of Naveen Jindal School of Manage ment, University of Texas at Dallas. His research interest includes social networks, e-business, and optimization. He served as a chair and TPC member of numerous conferences. 

![](images/dc7a1758d5d8f38524e56631d63a4b0c7a3c9dbb2314c44195bbee4fb972b93c.jpg)


Jiannong Cao received the MSc and PhD degrees in computer science from Washington State University, Pullman, Washington, in 1986 and 1990, respectively. He is currently the head and chair professor in the Department of Comput ing, Hong Kong Polytechnic University, Hong Kong. His research interests include parallel and distributed computing, mobile computing, and big data analytics. He has served as a member of the editorial boards of several international journals, a reviewer for international journals/conference proceedings, and also as an organizing/program committee member for many international conferences. He is a fellow of the IEEE and a senior member of the China Computer Federation. 

" For more information on this or any other computing topic, please visit our Digital Library at www.computer.org/publications/dlib. 