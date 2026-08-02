---
title: "algorithmic-game-theory-ch-29-part-030"
year: null
source_type: paper
why_relevant: ""
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: ""
canonicalized_at: 2026-08-01
ingest_status: pending_ingest
pdf_path: "work/core-books/algorithmic-game-theory/parts/algorithmic-game-theory-ch-29-part-030.pdf"
raw_md: "raw/canonical/algorithmic-game-theory-ch-29-part-030/full.md"
---
# Computational Evolutionary Game Theory

Siddharth Suri

## Abstract

This chapter examines the intersection of evolutionary game theory and theoretical computer science. We will show how techniques from each field can be used to answer fundamental questions in the other. In addition, we will analyze a model that arises by combining ideas from both fields. First, we describe the classical model of evolutionary game theory and analyze the computational complexity of its central equilibrium concept. Doing so involves applying techniques from complexity theory to the problem of finding a game-theoretic equilibrium. Second, we show how agents using imitative dynamics, often considered in evolutionary game-theory, converge to an equilibrium in a routing game. This is an instance of an evolutionary game-theoretic concept providing an algorithm for finding an equilibrium. Third, we generalize the classical model of evolutionary game theory to a graph-theoretic setting. Finally, this chapter concludes with directions for future research. Taken as a whole, this chapter describes how the fields of theoretical computer science and evolutionary game theory can inform each other.

## 29.1 Evolutionary Game Theory

Classical evolutionary game theory models organisms in a population interacting and competing for resources. The classical model assumes that the population is infinite. It models interaction by choosing two organisms uniformly at random, who then play a 2-player, symmetric game. The payoffs that these organisms earn represent an increase or a loss in fitness, which either helps or hinders the organisms ability to reproduce. In this model, when an organism reproduces, it does so by making an exact replica of itself, thus a child will adopt the same strategy as its parent.

One of the fundamental goals of evolutionary game theory is to characterize which strategies are resilient to small mutant invasions. In the classical model of evolutionary game theory, a large fraction of the population, called the incumbents, all adopt the same strategy. The rest of the population, called the mutants, all adopt some other strategy. The incumbent strategy is considered to be stable if the incumbents retain a higher fitness than the mutants. Since the incumbents are more fit, they reproduce more frequently and the fraction of mutants in the population will eventually go to 0. Put another way, an evolutionarily stable strategy (ESS) is a strategy such that if all the members of a population adopt it, then no mutant strategy could overrun the population. We shall see in Section 29.1.1 that ESS are a refinement of Nash equilibria.

Replication is not the only type of dynamic studied in evolutionary game theory. Imitation is another widely studied dynamic. In imitative dynamics, each agent initially plays some pure strategy. As time goes on, agents interact pairwise. After this pairwise interaction, if one agents sees the other agent earned a higher payoff, the agent with the lower payoff may adopt, or imitate, the strategy of the agent who earned the higher payoff. Imitative dynamics model, for example, a new idea, innovation, or fad spreading through a population of individuals or firms.

In general, there are two main characteristics common to most evolutionary game theoretic models. The first is that the population is infinite. The second is that players adopt a very simple, local dynamic, such as replication or imitation, for choosing and updating their strategies. These dynamics result in the agents learning from the other agents in their environment; they provide a method for an equilibrium strategy to emerge from the population. These types of dynamics explain how a population can converge to an equilibrium. For example, Section 18.3.1 shows that equilibria for the nonatomic selfish routing game exists, whereas Section 29.3 will show how agents obeying imitative dynamics can converge to it.

Next we will formally describe the basic model of evolutionary game theory. Then, in Section 29.2, we will analyze the computational complexity of finding and recog nizing stable strategies. After that, in Section 29.3, we will see an example of imitative dynamics. We will apply imitative dynamics to the problem of selfish routing and show how agents converge to an equilibrium. Finally, in Section 29.4, we will examine the no tion of stable strategies in a context where agents play against their local neighborhood in a graph, as opposed to playing against another agent chosen uniformly at random.

## 29.1.1 The Classical Model of Evolutionary Game Theory

The classical model of evolutionary game theory considers an infinite population of organisms, where each organism is assumed to be equally likely to interact with each other organism. Interaction is modeled as playing a fixed, 2-player, symmetric game defined by a fitness function F (we emphasize that the same game F is played in all interactions). Let A denote the set of actions available to both players, and let $\Delta ( A )$ denote the set of probability distributions or mixed strategies over A, then $F \colon \Delta ( A ) \times \Delta ( A )  \mathfrak { R }$ . If two organisms interact, one playing a mixed strategy s and the other playing a mixed strategy t, the s-player earns a fitness of $F ( s | t )$ while the t-player earns a fitness of $F ( t | s )$

In this infinite population of organisms, suppose that there is a $1 - \epsilon$ fraction who play strategy s, and call these organisms incumbents, and suppose that there is $\mathrm { a n ~ } \epsilon$ fraction who play t, and call these organisms mutants. Assume that two organisms are chosen uniformly at random to play each other. The strategy s is an ESS if the expected fitness of an organism playing s is higher than that of an organism playing t, for al $t \neq s$ and all sufficiently small -. Since an incumbent will meet another incumbent with probability $1 - \epsilon$ and it will meet a mutant with probability -, we can calculate the expected fitness of an incumbent, which is simply $( 1 - \epsilon ) F ( s | s ) + \epsilon F ( s | t )$ . Similarly, the expected fitness of a mutant is $( 1 - \epsilon ) F ( t | s ) + \epsilon F ( t | t )$ . Thus we come to the formal definition of an ESS.

Definition 29.1 A strategy s is an evolutionarily stable strategy (ESS) for the 2-player, symmetric game given by fitness function F, if for every strategy $t \neq s$ , there exists an - such that for all $0 < \epsilon < \epsilon _ { t } , ( 1 - \epsilon ) F ( s | s ) + \epsilon F ( s | t ) >$ $( 1 - \epsilon ) F ( t | s ) + \epsilon F ( t | t )$

If one assumes that each organism reproduces asexually, and spawns a number of offspring proportional to its fitness, then stable strategies will be those where the incumbent population will reproduce more than any small mutant invasion. Thus the mutant invasion will have fewer offspring and, in the long run, the fraction of mutants in the population will tend to 0. In fact, a continuous time analysis of the replicator dynamics shows that every ESS is asymptotically stable.

Definition 29.1 holds if and only if either of two conditions on s is satisfied $\forall t \neq s \colon$ (1) $F ( s | s ) > F ( t | s )$ , or (2) $F ( s | s ) = F ( t | s )$ and $F ( s | t ) > F ( t | t )$ . A consequence of this alternate formulation of an ESS is that for s to be an ESS, it must be the case that $F ( s | s ) \geq F ( t | s )$ , for all strategies t. This inequality means that s must be a best response to itself, and thus for any ESS s, the strategy profile $( s , s )$ must also be a Nash equilibrium. This results in another, equivalent way to define an ESS.

Theorem 29.2 A strategy s is an ESSfor a 2-player, symmetric game given by fitnessfunction F, if and only $i f ( s , s )$ ) is a Nash equilibrium of F, andfor every best response t to s, where $t \neq s , F ( s | t ) > F ( t | t )$

In general the notion of ESS is more restrictive than Nash equilibrium, and not al 2-player, symmetric games have an ESS.

Next, we give an example of a 2-player, symmetric game called Hawks and Doves, and then solve for its ESS. The game of Hawks and Doves models two organisms fighting over a resource. Obtaining the resource results in a gain of fitness of V, while fighting for the resource and losing results in a fitness decrease of C. If a Hawk plays a Dove, the Hawk will fight for the resource and the Dove will give up. This results in a Hawk earning in increase of fitness of V, and the Dove’s fitness staying the same. If two Doves play each other, they split the resource earning them both a fitness increase of $V / 2$ . If two Hawks play, eventually one will win and one will lose, and it assumed that each organism has a 1/2 chance of being the winner. Figure 29.1 shows the payoff matrix for this game.

The strategy profile $( D , D )$ is not a Nash Equilibrium because one player could unilaterally deviate and play H and increase its payoff from $V / 2$ to V. Since $( D , D )$ is

$$
\begin{array}{c c c} & H & D \\ H & (V - C) / 2 & V \\ D & 0 & V / 2 \end{array}
$$

Figure 29.1. The game of Hawks and Doves.

not a Nash Equilibrium, D cannot be an ESS. Now, if $V > C$ then H is an ESS. To see this observe that $F ( H | H ) = ( V - C ) / 2$ . Let t be any mixed strategy with probability $p < 1$ of playing H and $1 - p$ of playing D, then $\begin{array} { r } { F ( t | H ) = p \frac { V - C } { \gamma } + ( 1 - p ) 0 < } \end{array}$ $( V - C ) / 2$ . Since $F ( H | H ) > F ( t | H )$ for all $t \neq H$ , H is an ESS. We leave it as an exercise for the reader (see Section $2 9 . 6 )$ to show that if $V \leq C$ , the mixed strategy o playing H with probability $V / C$ and D with probability $1 - V / C$ is an ESS. Observe that as $V  C$ , the probability of playing H approaches 1. This coincides with the pure strategy ESS of playing H when $V > C$

## 29.2 The Computational Complexity of Evolutionarily Stable Strategies

Next we show the computational complexity of finding an ESS given a 2-player symmetric game is both NP-hard and coNP-hard. To prove this, we will make a reduction from the problem of checking if a graph has a maximum clique of size exactly k. Prior work has shown that this problem is both NP-hard and coNP-hard. Along the way to proving the hardness of finding an ESS, we will see that the problem of recognizing whether a given strategy is an ESS is also coNP-hard.

Next we will give the intuition behind the reduction. The reduction will transform a graph G into a payoff matrix F which will have an ESS if and only if the size of the largest clique in G is not equal to k. The reduction transforms the adjacency matrix of G into the payoff matrix F by replacing all the diagonal entries with the value $1 / 2$ inserting a 0th row with each entry having a constant value, and inserting a 0th column with each entry having the same constant value.

Informally speaking, for a mixed strategy s to be an ESS, incumbents should receive a relatively high payoff when playing other incumbents. In order for a strategy s to have this property for the game F, when s plays itself it must guarantee that the pure strategies chosen will correspond to two adjacent vertices. One can see that having a mixed strategy with support over a clique will achieve this. We will show in Lemma 29.3 that having support over a clique will result in a higher payoff than having support over a dense subgraph that is not a clique. Having the diagonal entries consist of the constant $1 / 2$ will help us prove this. This lemma will allow us to prove that when the size of the maximum clique is greater than k, the uniform mixed strategy corresponding to vertices of the clique will be an ESS. In addition, setting the 0th row and column of F to a carefully chosen constant will give us a pure strategy ESS in the case where the size of the maximum clique is less than k. This constant will also allow us to show that there is no ESS in the case where the size of the maximum clique in G is exactly k.

In describing this reduction, and for the rest of this chapter, we use the notation $F ( s | t )$ to denote the payoff of the player playing strategy s when confronted with a player playing strategy t. When we are referring to a specific entry in the payoff matrix of $F _ { ; }$ , we will use the notation $F ( i , j )$ to denote the entry in the ith row and jth column. Also, if s is a mixed strategy, we let $s _ { i }$ denote the probability that the pure strategy i is played. (Thus we will use s and t to denote mixed strategies, and i and j to denote indices into these mixed strategies, as well as indices into the payoff matrix F.)

The reduction from a graph $G = ( V , E )$ to a payoff matrix F that we consider works as follows.

 for $1 \leq i \neq j \leq n \colon F ( i , j ) = 1 { \mathrm { ~ i f ~ } } ( i , j ) \in E { \mathrm { ~ a n d ~ } } F ( i , j ) = 0 { \mathrm { ~ i f ~ } } ( i , j ) \not \in E$

 for $1 \leq i \leq n \colon F ( i , i ) = 1 / 2$

 fo $\cdot 0 \leq i \leq n \colon F ( 0 , i ) = F ( i , 0 ) = 1 - 1 / ( 2 k )$ )

To show that F has an ESS if and only if the size of the largest clique in G its not equal to k, we will need the following technical lemma.

Lemma 29.3 If s is a strategy with $s _ { 0 } = 0$ , then $F ( s | s ) \leq 1 - 1 / ( 2 k ^ { \prime } )$ , where $k ^ { \prime }$ is the size ofthe maximum clique in G. This holds with equality ifand only $i f s$ is the uniform distribution over a k<sup></sup>-clique.

proof The proof is by induction on the number of nonedges between the vertices in $G = ( V , E )$ corresponding to elements of the support set of s. The base case is when there are 0 such non-edges, which means the vertices corresponding to the support set of s form a $k ^ { \prime \prime } { \mathrm { - c l i q u e } }$ , where $k ^ { \prime \prime } \leq k$ . We assume, without loss of generality, that the vertices in the $k ^ { \prime \prime } { \mathrm { - c l i q u e } }$ are numbered $1 , 2 , \ldots , k ^ { \prime \prime }$

$$
\begin{array}{l} F (s | s) = \sum_ {i \in [ k ^ {\prime \prime} ]} \sum_ {j \in [ k ^ {\prime \prime} ]} s _ {i} s _ {j} F (i, j) \\ \qquad = \sum_ {i \in [ k ^ {\prime \prime} ]} \sum_ {j \in [ k ^ {\prime \prime} ]} s _ {i} s _ {j} - \sum_ {i \in [ k ^ {\prime \prime} ]} s _ {i} ^ {2} / 2 \\ \qquad = \sum_ {i \in [ k ^ {\prime \prime} ]} s _ {i} \sum_ {j \in [ k ^ {\prime \prime} ]} s _ {j} - 1 / 2 \sum_ {i \in [ k ^ {\prime \prime} ]} s _ {i} ^ {2} \\ \qquad \leq 1 - 1 / (2 k ^ {\prime \prime}) \end{array}
$$

The last inequality comes from the fact that when $| | s | | _ { 1 } = 1 , | | s | | _ { 2 }$ is minimized, and the inequality is tight, only when all of the components of s are equal. Conversely, if s is the uniform distribution over a k<sup></sup>-clique then, the inequality is tight, which is shown as follows,

$$
\begin{array}{c} \sum_ {i \in [ k ^ {\prime} ]} \sum_ {j \in [ k ^ {\prime} ]} s _ {i} s _ {j} F (i, j) = 1 / k ^ {\prime 2} \sum_ {i \in [ k ^ {\prime} ]} \sum_ {j \in [ k ^ {\prime} ]} F (i, j) \\ = 1 / k ^ {\prime 2} [ k ^ {\prime 2} - k ^ {\prime} / 2 ] \\ = 1 - 1 / (2 k ^ {\prime}). \end{array}
$$

For the inductive step, let u and v be two vertices such that $( u , v ) \notin E$ . We construct a new strategy $s ^ { \prime }$ by moving the probability from v to u. So let $s _ { u } ^ { \prime } = s _ { u } +$ $s _ { v }$ and $s _ { v } ^ { \prime } = 0$ , and let the rest of the values of $s ^ { \prime }$ be identical to those of s. Since v is no longer in the support set of s, we can use the induction hypothesis to conclude that $F ( s ^ { \prime } | s ^ { \prime } ) \leq 1 - 1 / ( 2 k ^ { \prime } )$ . Let $\begin{array} { r } { p = \sum _ { ( u , w ) \in E } s _ { w } } \end{array}$ and let $\begin{array} { r } { \boldsymbol { q } = \sum _ { \boldsymbol { v } , \boldsymbol { w } ) \in E } \boldsymbol { s } _ { w } } \end{array}$ , and without loss of generality assume that $p \geq q$ . By writing out the expressions for $F ( s ^ { \prime } | s ^ { \prime } )$ and $F ( s | s )$ one can show $F ( s ^ { \prime } | s ^ { \prime } ) = F ( s | s ) + 2 s _ { v } ( p - q ) + s _ { u } s _ { v } >$ $F ( s | s )$ . Thus, $F ( s | s ) \leq 1 - 1 / ( 2 k ^ { \prime } )$ , which proves the inductive step.

Now we will use this lemma to prove the necessary properties of the reduction. The next two lemmas, when taken together, show that if the maximum size clique in G has size not equal to k, then F has an ESS.

Lemma 29.4 IfC is a maximal clique in G ofsize $k ^ { \prime } > k$ , and s is the uniform distribution on C, then s is an ESS.

proof By Lemma 29.3, $F ( s | s ) = 1 - 1 / ( 2 k ^ { \prime } )$ . By the construction of the payoff matrix $F , F ( 0 | s ) = 1 - 1 / ( 2 k ) < F ( s | s )$ . Also, for any $u \notin C , u$ is connected to at most $k ^ { \prime } - 1$ vertices in C, thus $F ( u | s ) \leq 1 - 1 / k ^ { \prime } < F ( s | s )$ . Thus any best response to s must have support only over C. Furthermore, by Lemma 29.3 the payoff of s against s is maximized when s is the uniform distribution over C. Thus, s is a best response to itself. To prove that s is an ESS, it remains to show that for all $t \neq s$ , that are best responses, to s, $F ( s | t ) > F ( t | t )$ . Again by Lemma 29.3, $F ( t | t ) < 1 - 1 / ( 2 k ^ { \prime } )$ . Since C is a clique and s and t are distributions with support over C, using the structure of F one can compute that $F ( s | t ) = 1 - 1 / ( 2 k ^ { \prime } )$ Thus, $F ( s | t ) > F ( t | t )$ and s is an ESS.

Lemma 29.5 If the maximum size clique in G is of size $k ^ { \prime } < k$ then the pure strategy 0 is an ESS.

proof For any mutant strategy t, $F ( t | 0 ) = 1 - 1 / ( 2 k ) = F ( 0 | 0 )$ , thus 0 is a best response to itself. Next, we show that for any t not equal to the pure strategy 0, $F ( 0 | t ) > F ( t | t )$ . To do so, we first show that we can assume that t places no weight on the pure strategy 0. Let $t ^ { * }$ be the strategy t with the probability of playing the pure strategy 0 set to the value 0 and then renormalized. So, $t _ { 0 } ^ { * } = 0$ and for $i \neq 0 , t ^ { * } = t _ { i } / ( 1 - t _ { 0 } )$ . By writing out the expressions for $F ( t | t )$ and $F ( t ^ { * } | t ^ { * } )$ , one can show $F ( t | t ) = ( 2 t _ { 0 } - t _ { 0 } ^ { 2 } ) ( 1 - 1 / ( 2 k ) ) + ( 1 - 2 t _ { 0 } + t _ { 0 } ^ { 2 } ) F ( t ^ { * } | t ^ { * } )$ Since $F ( 0 | t ) = 1 - 1 / ( 2 k ) , \ F ( 0 | t ) > F ( t | t )$ if and only if $F ( 0 | t ) > F ( t ^ { * } | t ^ { * } )$ Next, since the maximum size clique in G has size $k ^ { \prime } < k$ , applying Lemma 29.3 gives $F ( t ^ { * } | t ^ { * } ) \leq 1 - 1 / ( 2 k ^ { \prime } ) < 1 - 1 / ( 2 k ) = F ( 0 | t )$ □

The next two lemmas, when combined, show that if the maximum size clique in $G$ has size exactly k, then F has no ESS.

Lemma 29.6 Ifthe maximum size clique ofG is at least k, then thepure strategy 0 is not an ESS.

proof Since $F ( 0 | 0 ) = F ( t | 0 ) = 1 - 1 / ( 2 k )$ for any strategy t, the pure strategy 0 is a best response to itself. But, if t is the uniform distribution on the maximum clique of G, which has size $k ^ { \prime } \geq k$ , then by Lemma 29.3 $F ( t | t ) = 1 - 1 / ( 2 k ^ { \prime } ) \geq$ $F ( 0 | t )$ . By Theorem 29.2, this means the pure strategy 0 cannot be an ESS.

Lemma 29.7 If the maximum size clique of G is at most k, then any strategy for F that is not equal to the pure strategy 0, is not an ESSfor F.

The proof of this lemma uses techniques similar to those used in Lemmas 29.5 and 29.6, so we leave it as an exercise for the reader (see Section 29.6).

Taking Lemmas 29.4, 29.5, 29.6, and 29.7 together, we get the following theorem.

Theorem 29.8 Given a 2-player, symmetric game F computing whether or not F has an ESS is both NP-hard and coNP-hard.

Combining Lemmas 29.5 and 29.6 shows that it is coNP-hard to check whether a given strategy is an ESS or not.

Theorem 29.9 Given a 2-player, symmetric game F and a strategy s, it is coNP-hard to compute whether or not s in an ESS ofF.

proof Lemmas 29.5 and 29.6 imply that G has maximum clique of size less than k if and only if the pure strategy 0 is an ESS of F. Since the problem of determining whether a graph has a maximum clique of size less than k is coNP-hard, the problem of recognizing an ESS is also coNP-hard.

Theorems 29.8 and 29.9 imply that there exist games for which, in all likelihood, efficient algorithms for finding and recognizing ESS do not exist. These results are important because if finding an ESS for a given class of games is NP-hard, it is unlikely that a finite population obeying some simple dynamic will quickly converge to it. But, this observation does not mean that one should avoid using models based on ESS. It simply means that to ensure the plausibility of a finite population model, one should check whether it is computationally tractable to find the ESS of the games the model considers. Moreover, this result does not directly imply that an infinite population, however, cannot quickly converge to an equilibrium. In fact, the next section explores the convergence time of an infinite population to an equilibrium.

## 29.3 Evolutionary Dynamics Applied to Selfish Routing

In this section we describe a method for applying evolutionary dynamics to the problem of selfish routing. The model will consider an infinite population of agents, each of which carries an infinitesimally small amount of flow in a network. The agents actions allow them to change the path that they traverse; however, agents will not be allowed to change their paths arbitrarily. The space of actions available to these agents will be governed by simple, imitative dynamics. We show how agents selfishly seeking out low latency paths, while obeying these imitative dynamics, converge to an approximate equilibrium. First, we will formally describe the model which is similar to the nonatomic selfish routing model shown in Section 18.2.1. Then, we will briefly outline a technique that shows, in the limit, these dynamics converge to an equilibrium. Finally, we wil analyze the time of convergence to an approximate equilibrium.

## 29.3.1 The Selfish Routing Model with Imitative Dynamics

Let $G = ( V , E )$ be a network with latency functions $l _ { e } \colon [ 0 , 1 ] \to \Re$ defined over each edge. We assume the latency functions are nonnegative, nondecreasing, and Lipschitz continuous. We also assume that there is one unit of flow that is to be routed from a source s to a sink t, and we let P denote the set of s-t paths in G. We also assume that there are infinitely many agents, each of which carries an infinitesimally small amount of flow. Let $x _ { p }$ denote the fraction of flow that is being routed over path p. Thus the vector ${ \vec { x } } ,$ , which is indexed by the paths in P, will describe the flow over $G$ at a given point in time. A flow $\vec { x }$ is feasible if it routes 1 unit of flow from s to t. Let $\begin{array} { r } { x _ { e } = \sum _ { p \ni e } x _ { p } } \end{array}$ be the total load of an edge. The total latency of an edge is denoted $l _ { e } ( x _ { e } )$ and the total latency of a path is the sum of the latencies of the edges in the path, $\begin{array} { r } { l _ { p } ( \vec { x } ) = \sum _ { e \in p } l _ { e } ( x _ { e } ) } \end{array}$ . Finally, the average latency of the entire network is $\begin{array} { r } { \bar { l } = \sum _ { p \in P } x _ { p } l _ { p } ( \vec { x } ) } \end{array}$

Initially each agent is assumed to play an arbitrary pure strategy. Then at each point in time, each agent is randomly paired with another agent and they compare the latencies of their paths. If the latency of one agent’s path is less than the latency of the other agent’s path, the agent experiencing higher latency switches to the lower latency path with probability proportional to the difference in latencies. These imitative dynamics model a source node gathering statistics on how long it takes for its packets to reach the destination and changing the route accordingly. In Section 29.3.2 we will describe why these dynamics will continue until the agents reach a Nash flow (also called Wardrop equilibrium), which is a pure strategy Nash equilibrium for this routing game, that we define next.

Definition 29.10 A feasible flow $\vec { x }$ is a Nash flow if and only if for all $p , p ^ { \prime } \in P$ with $x _ { p } > 0 , l _ { p } ( \vec { x } ) \leq l _ { p ^ { \prime } } ( \vec { x } )$

This definition ensures that, at a Nash flow, all s–t paths have the same latency (this is precisely Definition 18.1 when restricted to the single commodity case). Ifwe further restrict the latency functions to be strictly increasing, then Nash flows are essentially ESS. We omit the proof of this since this section focuses on the convergence of the imitative dynamics (we refer the interested reader to Section 29.6 for the appropriate references).

To analyze the convergence of these dynamics to either a Nash flow or an approximate equilibrium, it is necessary to compute the rate of change of the amount of flow over each path. Throughout this section we will use the notation $x ^ { \prime }$ to denote the derivative with respect to time of the variable x, that is, $x ^ { \prime } = d x / d t$ . The following set of differential equations describe the rate of change of the flow over each path.

$$
\begin{array}{l} x _ {p} ^ {\prime} = - x _ {p} \sum_ {q \in P: l _ {q} (\vec {x}) <   l _ {p} (\vec {x})} x _ {q} \lambda (\vec {x}) [ l _ {p} (\vec {x}) - l _ {q} (\vec {x}) ] \\ \quad + \sum_ {q \in P: l _ {q} (\vec {x}) > l _ {p} (\vec {x})} x _ {p} x _ {q} \lambda (\vec {x}) [ l _ {q} (\vec {x}) - l _ {p} (\vec {x}) ] \\ = \sum_ {q \in P} x _ {p} x _ {q} \lambda (\vec {x}) [ l _ {q} (\vec {x}) - l _ {p} (\vec {x}) ] \\ = \lambda (\vec {x}) x _ {p} \left[ \sum_ {q \in P} x _ {q} l _ {q} (\vec {x}) - l _ {p} (\vec {x}) \sum_ {q \in P} x _ {q} \right] \\ = \lambda (\vec {x}) x _ {p} [ \bar {l} (\vec {x}) - l _ {p} (\vec {x}) ] \end{array}\tag{29.1}
$$

(29.2)

In this derivation, the function λ accounts for normalizing factors so that the probabili ties are bounded above by 1, and it accounts for the rate at which organisms are paired. The first summation in Equation 29.1 represents the expected number of agents that switch from path $p$ to lower latency paths. The probability than an agent on path $p$ is paired with an agent of path $q$ is equal to the fraction of agents using $q$ , which is $x _ { q }$ Then the agent using $p$ would switch to $q$ with probability $l _ { p } ( \vec { x } ) - l _ { q } ( \vec { x } )$ . Multiplying this product by $x _ { p }$ gives the expected number of agents moving from $p$ to a lower latency path $q$ . Similarly, the second summation of Equation 29.1 represents the number of agents that switch to path $p$ from a higher latency path. The rest of the derivation results from straightforward algebraic manipulations.

Intuitively, Equation 29.2 says that paths with below average latency will have more agents switching to them than from them; paths with above average latency will have more agents switching from them than to them. In Section 29.3.3, where we bound the time it takes for the system to converge to an approximate equilibrium, we would like the rate of change of the population to be independent of the scale of the latency functions. Thus we will replace $\lambda ( \vec { x } )$ by $\bar { l } ( \vec { x } ) ^ { - 1 }$ to give a relative rate of change.

While these equations resulted from imitative dynamics, the same equations can be derived from a type of replication dynamic. In the literature, these equations are often called the replicator dynamics. Now that we have defined the model and the dynamics, we will show that the population of agents using imitative dynamics will converge to an approximate equilibrium.

## 29.3.2 Convergence to Nash Flow

It has been shown that as time goes to infinity, any initial flow that has support over all paths in P will eventually converge to a Nash flow. In this section we give an overview of the technique used to prove this. It is not clear how these techniques could yield a bound on the time to convergence, so we do not go into specific details of the proof. Since this text is focused on algorithmic game theory, we shall instead give more attention to another result, shown in Section 29.3.3, that bounds the time of convergence to an approximate equilibrium.

The main vehicle for proving that imitative dynamics converge to a Nash flow is Lyapunov’s direct method. This is a general framework for proving that a system of differential equations converges to a stable point, without necessarily knowing how to solve the system of differential equations. Intuitively, this method works by first defining a real valued potential function  that measures the potential energy of the system of differential equations. The direct method requires that  be defined around a neighborhood of a stable point and vanish at the stable point itself. Then, if one can show that the dynamics of the system cause the potential function to decrease with respect to time (along with a few other technical properties of the potential function), Lyapunov’s theorems will imply that if the system reaches the neighborhood of the stable point, the system will converge to the stable point. One drawback to this method is that it provides no guidance for choosing such a potential function.

The argument that applies this method to the system of differential equations de scribed in Equation 29.2 works as follows. First, define  over the current flow such that it will measure the total amount of latency the agents are experiencing. We wil define just such a function in the next section. Then, show that the imitative dynam ics cause $\Phi$ to decrease over time, and that  will achieve its minimum value at a Nash flow. Applying one of the theorems in the Lyapunov’s framework allows one to conclude that if the dynamics ever reach a neighborhood of an equilibrium, they will converge to it. Finally, one has to show this neighborhood of convergence contains any initial, feasible flow with support over all paths in P. This comes from the fact that the dynamics cause the potential of any nonequilibrium flow to decrease and thus move toward an equilibrium. Thus, in this model of selfish routing with imitative dynamics, the Lyapunov framework allows one to show that the system will not get stuck in any local minima and will converge to global minimum from any initial state with support over all paths in P.

## 29.3.3 Convergence to Approximate Equilibrium

In this section we will give a bound on how long it takes for the population of agents using imitative dynamics to come to an approximate equilibrium.

One might consider using Euclidean distance between the current flow and an equilibrium flow as a measure of approximation. To see intuitively why this is not a suitable metric, consider a network and a flow where an - fraction of the agents uses a path $p _ { : }$ , which has a latency that is slightly less than the current average latency. If it were essential for an equilibrium to have a large fraction of the population using p, we could take $\epsilon$ to be arbitrarily small, which, by Equation 29.2, means we could make $x _ { p } ^ { \prime }$ arbitrarily small. Thus the imitative dynamics would cause the population to move arbitrarily slowly to $p ,$ and therefore it would take arbitrarily long for the population to approach, in Euclidean distance, a Nash flow. Thus, we define an --approximate equilibrium next.

Definition 29.11 Let $P _ { \epsilon }$ be the paths that have latency at least $( 1 + \epsilon ) \bar { l }$ , that is $P _ { \epsilon } = \{ p \in P \ | \ l _ { p } ( \vec { x } ) \geq ( 1 + \epsilon ) \bar { l } \}$ , and let $\begin{array} { r } { x _ { \epsilon } = \sum _ { p \in P _ { \epsilon } } x _ { p } } \end{array}$ be the fraction of agents using these paths. A population $\vec { x }$ is said to be at an --approximate equilibrium if and only if $x _ { \epsilon } \leq \epsilon$

This definition ensures at such an equilibrium that only a small fraction of agents experience latency significantly worse than the average latency. In contrast, the definition of a Nash flow requires that all agents experience the same latency (see Definition 29.10).

To prove the convergence of these imitative dynamics to an approximate equilibrium, we will make use of the following potential function. This function is one way to measure the total amount of latency the agents experience.

$$
\Phi (\vec {x}) = l ^ {*} + \sum_ {e \in E} \int_ {0} ^ {x _ {e}} l _ {e} (u) d u\tag{29.3}
$$

The integral sums the latency each agent that traverses edge e would experience if the agents were inserted one at a time. Summing this over each edge gives the total latency that each agent would experience if they were entered into the network one at a time. The term $l ^ { * }$ denotes the minimum average latency of a feasible flow, $l ^ { * } = \operatorname* { m i n } _ { \vec { x } } \bar { l } .$ . We add this term as a technicality that will help prove our bounds on the time convergence to approximate equilibrium. With the exception of the $l ^ { * }$ term, this is the same potential function described in Equation 18.3.

Theorem 29.12 The imitative dynamics converge to an --approximate equilibrium within time $O ( \epsilon ^ { - 3 } \ln ( l _ { \mathrm { m a x } } / l ^ { \ast } ) )$ ).

This proof works by analyzing the rate of change of $\Phi$ under the imitative dynamics. If the current flow is not at an --approximate equilibrium, we can lower bound the absolute rate of change of $\Phi$ in terms of ${ \bar { l } } .$ We then lower bound $\bar { l }$ in terms of , resulting in a differential inequality. Solving it leads to an upper bound on the time it takes for  reach an approximate equilibrium.

proof We start by computing the derivative with respect to time of the potential function .

$$
\Phi^ {\prime} = \sum_ {e \in E} x _ {e} ^ {\prime} l _ {e} (x _ {e}) = \sum_ {e \in E} \sum_ {p \ni e} x _ {p} ^ {\prime} l _ {e} (x _ {e})
$$

Next we substitute in the imitative dynamics, given by Equation 29.2. After that we simplify the expression with the aim of using Jensen’s inequality.

$$
\begin{array}{l} \Phi^ {\prime} = \sum_ {e \in E} \sum_ {p \ni e} \lambda (\vec {x}) x _ {p} [ \bar {l} (\vec {x}) - l _ {p} (\vec {x}) ] l _ {e} (x _ {e}) \\ \qquad = \lambda (\vec {x}) \sum_ {p \in P} \sum_ {e \in p} x _ {p} [ \bar {l} (\vec {x}) - l _ {p} (\vec {x}) ] l _ {e} (x _ {e}) \\ \qquad = \lambda (\vec {x}) \sum_ {p \in P} x _ {p} [ \bar {l} (\vec {x}) - l _ {p} (\vec {x}) ] l _ {p} (x _ {p}) \\ \qquad = \lambda (\vec {x}) \bigg (\bar {l} (\vec {x}) \sum_ {p \in P} x _ {p} l _ {p} (x _ {p}) - \sum_ {p \in P} x _ {p} l _ {p} (\vec {x}) ^ {2} \bigg) \\ \qquad = \lambda (\vec {x}) \bigg (\bar {l} (\vec {x}) ^ {2} - \sum_ {p \in P} x _ {p} l _ {p} (\vec {x}) ^ {2} \bigg) \end{array}\tag{29.4}
$$

Jensen’s inequality shows that this equation is bounded above by $0 .$

We would like to upper bound $\Phi ^ { \prime }$ . To do so, first observe as long as $\vec { x }$ is not at an --approximate equilibrium, by definition at least an $\epsilon$ fraction of the population experiences latency at least $( 1 + \epsilon ) \bar { l } ( \vec { x } )$ . Jensen’s inequality also shows that for a fixed value of $\bar { l } ( \vec { x } )$ , the $\sum _ { p \in P } x _ { p } l _ { p } ( \vec { x } ) ^ { 2 }$ term is minimized when the less expensive paths all have equal latency which we denote $l ^ { \prime } .$ . Thus, for the purposes of upper bounding $\Phi ^ { \prime }$ , we assume $\bar { l } = \epsilon ( 1 + \epsilon ) \bar { l } + ( 1 - \epsilon ) l ^ { \prime }$ . Plugging this into Equation 29.4 gives

$$
\Phi^ {\prime} \leq \lambda (\vec {x}) [ \bar {l} (\vec {x}) ^ {2} - (\epsilon ((1 + \epsilon) \bar {l} (\vec {x})) ^ {2} + (1 - \epsilon) l ^ {\prime 2}) ].
$$

Now we substitute in $\begin{array} { r } { l ^ { \prime } = \bar { l } \frac { 1 - \epsilon - \epsilon ^ { 2 } } { 1 - \epsilon } } \end{array}$ and perform some arithmetic giving,

$$
\begin{array}{r l} & {\Phi^ {\prime} \leq - \lambda (\vec {x}) \frac {\epsilon^ {3}}{1 - \epsilon} \bar {l} (\vec {x}) ^ {2}} \\ & {\qquad \leq - \lambda (\vec {x}) \frac {\epsilon^ {3}}{2} \bar {l} (\vec {x}) ^ {2}.} \end{array}
$$

We also replace $\lambda ( \vec { x } )$ with $\bar { l } ( \vec { x } ) ^ { - 1 }$ to measure the relative rate of change of  under the imitative dynamics,

$$
\Phi^ {\prime} \leq - \frac {\epsilon^ {3}}{2} \bar {l} (\vec {x}).\tag{29.5}
$$

We can bound <sup>¯</sup>l from below by $\Phi / 2$ in the following way,

$$
\begin{array}{l} \bar {l} (\vec {x}) = \sum_ {p \in P} x _ {p} l _ {p} (\vec {x}) = \sum_ {p \in P} x _ {p} \sum_ {e \in p} l _ {e} (x _ {e}) \\ \qquad = \sum_ {e \in E} \sum_ {p \ni e} x _ {p} l _ {e} (x _ {e}) = \sum_ {e \in E} x _ {e} l _ {e} (\vec {x}) \\ \qquad \geq \sum_ {e \in E} \int_ {0} ^ {x _ {e}} l _ {e} (u) d u. \end{array}\tag{29.6}
$$

The inequality holds because of the assumed monotonicity of the latency functions. Now by the definition of $l ^ { * }$ , it is easy to see that ${ \bar { l } } \geq l ^ { * }$ . Combining this fact with Equation 29.6, we get that $\begin{array} { r } { \bar { l } + \bar { l } \ge l ^ { * } + \sum _ { e \in E } \int _ { 0 } ^ { x _ { e } } l _ { e } ( u ) d u = \Phi } \end{array}$ . Thus $\bar { l } \ge \Phi / 2$ . Substituting this into Inequality 29.5, we get the following differentia inequality,

$$
\Phi^ {\prime} \leq - \epsilon^ {3} \Phi / 4.
$$

It can be shown via standard methods that any function of the following form is a solution to the above inequality,

$$
\Phi (t) \leq \Phi (0) e ^ {- \epsilon^ {3} t / 4}.
$$

Here $\Phi ( 0 )$ is given by the initial boundary conditions. Recall that this inequality only holds as long as x is not an --approximate equilibrium. Thus, x must reach an --approximate equilibrium when  reaches its minimum, $\Phi ^ { * }$ , at the latest. So we find the smallest t such that $\Phi ( t ) \leq \Phi ^ { * }$ ,

$$
t = 4 \epsilon^ {- 3} \ln \frac {\Phi (0)}{\Phi^ {*}}.
$$

It is easy to see that $\Phi ^ { * } \geq l ^ { * }$ and $\Phi ( 0 ) \leq 2 l _ { \mathrm { m a x } }$ , which proves the theorem.

## 29.4 Evolutionary Game Theory over Graphs

Next, we will consider a model similar to the classical model of evolutionary game theory described in Section 29.1, but we will no longer assume that two organisms are chosen uniformly at random to interact. Instead, we assume that organisms interact only with those in their local neighborhood, as defined by an undirected graph or network

As in the classical setting (which can be viewed as the special case of the complete network or clique), we shall assume an infinite population, by which we mean we examine limiting behavior in a family of graphs of increasing size.

Before giving formal definitions, some comments are in order on what to expect in moving from the classical to the graph-theoretic setting. In the classical (complete graph) setting, there exist many symmetries that may be broken in moving to the network setting, at both the group and individual level. Indeed, such asymmetries are the primary interest in examining a graph-theoretic generalization.

For example, at the group level, in the standard ESS definition, one need not discuss any particular set of mutants of population fraction -. Since all organisms are equally likely to interact, the survival or fate of any specific mutant set is identical to that of any other. In the network setting, this may not be true: some mutant sets may be better able to survive than others due to the specific topologies of their interactions in the network. For instance, foreshadowing some of our analysis, if s is an ESS but $F ( t | t )$ is much larger than $F ( s | s )$ and $F ( s | t )$ , a mutant set with a great deal of “internal” interaction (i.e., edges between mutants) may be able to survive, whereas one without this may suffer. At the level of individuals, in the classical setting, the assertion that one mutant dies implies that all mutants die, again by symmetry. In the network setting, individual fates may differ within a group all playing a common strategy. These observations imply that in examining ESS on networks we face definitional choices that were obscured in the classical model.

If G is a graph representing the allowed pairwise interactions between organisms (vertices), and u is a vertex of G playing strategy $s _ { u }$ , then the fitness of u is given by

$$
F (u) = \frac {\sum_ {v \in \Gamma (u)} F (s _ {u} | s _ {v})}{| \Gamma (u) |}.
$$

Here $s _ { v }$ is the strategy being played by the neighbor v, and $\Gamma ( u ) = \{ v \in V : ( u , v ) \in E \}$ One can view the fitness of u as the average fitness u would obtain if it played each of its neighbors, or the expected fitness u would obtain if it were assigned to play one of its neighbors chosen uniformly at random.

Classical evolutionary game theory examines an infinite, symmetric population. Graphs or networks are inherently finite objects, and we are specifically interested in their asymmetries, as discussed above. Thus all of our definitions shall revolve around an infinite family $G = \{ G _ { n } \} _ { n = 0 } ^ { \infty }$ of finite graphs $G _ { n }$ over n vertices, but we shall examine asymptotic (large n) properties of such families.

We first give a definition for a family of mutant vertex sets in such an infinite graph family to contract.

Definition 29.13 Let $G = \{ G _ { n } \} _ { n = 0 } ^ { \infty }$ be an infinite family of graphs, where $G _ { n }$ has n vertices. Let $M = \{ M _ { n } \} _ { n = 0 } ^ { \infty }$ be any family of subsets of vertices of the $G _ { n }$ such that $| M _ { n } | \geq$ -n for some constant $\epsilon > 0$ . Suppose all the vertices of $M _ { n }$ play a common (mutant) strategy t, and suppose the remaining vertices in $G _ { n }$ play a common (incumbent) strategy s. We say that $M _ { n }$ contracts if for sufficientl large n, for all but $o ( n )$ of the $j \in M _ { n } , j$ has an incumbent neighbor i such that $F ( j ) < F ( i )$

A reasonable alternative would be to ask that the condition above holds for all mutants rather than all but $o ( n )$ . Note also that we only require that a mutant have one incumbent neighbor of higher fitness in order to die; one might consider requiring more. In Section 29.6 we ask the reader to consider one of these stronger conditions and demonstrate that our results can no longer hold.

To properly define an ESS for an infinite family of finite graphs in a way that recovers the classical definition asymptotically in the case of the family of complete graphs, we first must give a definition that restricts attention to families of mutant vertices that are smaller than some invasion threshold $\epsilon ^ { \prime } n$ , yet remain some constant fraction of the population. This prevents “invasions” that survive merely by constituting a vanishing fraction of the population.

Definition 29.14 Let $\epsilon ^ { \prime } > 0 $ , and let $G = \{ G _ { n } \} _ { n = 0 } ^ { \infty }$ be an infinite family of graphs, where $G _ { n }$ has n vertices. Let $M = \{ M _ { n } \} _ { n = 0 } ^ { \infty }$ be any family of (mutant) vertices in $G _ { n }$ . We say that M is -<sup></sup>-linear if there exists an $\epsilon , \epsilon ^ { \prime } > \epsilon > 0$ , such that for all sufficiently large $n , \epsilon ^ { \prime } n > | M _ { n } | > \epsilon n$

We can now give our definition for a strategy to be evolutionarily stable when employed by organisms interacting with their neighborhood in a graph.

Definition 29.15 Let $G = \{ G _ { n } \} _ { n = 0 } ^ { \infty }$ be an infinite family of graphs, where $G _ { n }$ has n vertices. Let F be any 2-player, symmetric game for which s is a strategy. We say that s is an ESS with respect to F and G if for all mutant strategies $t \neq s$ , there exists an $\epsilon _ { t } > 0$ such that for any $\epsilon _ { t }$ -linear family of mutant vertices $M = \{ M _ { n } \} _ { n = 0 } ^ { \infty }$ all playing t, for n sufficiently large, $M _ { n }$ contracts.

Thus, to violate the ESS property for G, one must witness a family of mutations M in which each $M _ { n }$ is an arbitrarily small but nonzero constant fraction of the population of $G _ { n }$ , but does not contract (i.e., every mutant set has a subset of linear size that survives all of its incumbent interactions). One can show that the definition given coincides with the classical one in the case where G is the family of complete graphs, in the limit of large n. We note that even in the classical model, small sets of mutants were allowed to have greater fitness than the incumbents, as long as the size of the set was $o ( n )$

In the definition above there are three parameters: the game F, the graph family G, and the mutation family M. Our main results will hold for any 2-player, symmetric game F. We will study a rather general setting for G and M: that in which G is a family of random graphs and M is arbitrary. We will see that, subject to conditions on degree or edge density (essentially forcing connectivity of G but not much more), for any 2- player, symmetric game, the ESS of the classical settings, and only those strategies, are always preserved. Thus, for the purposes ofcharacterizing stable strategies, the classica method of pairing organisms at random, is equivalent to randomizing the graph.

## 29.4.1 Random Graphs, Adversarial Mutations

We now proceed to state and prove the random graph result in the network ESS model. We consider a setting in which the graphs are generated via the $G _ { n , p }$ model ofErdos and¨

Renyi. In this model, every pair of vertices is joined by an edge independently and with´ probability $p$ (where p may depend on n). The mutant set, however, will be constructed adversarially (subject to the linear size constraint given by Definition 29.15). For this setting, we show that for any 2-player, symmetric game, s is a classical ESS of that game, if and only if s is an ESS for $\{ G _ { n , p } \} _ { n = 0 } ^ { \infty } ,$ , where $p = \Omega ( 1 / n ^ { c } )$ and $0 \leq c < 1$ , and any mutant family $\{ M _ { n } \} _ { n = 0 } ^ { \infty }$ , where each $M _ { n }$ has linear size. We note that under these settings, if we let $c = 1 - \gamma$ for small $\gamma > 0$ , the expected number of edges in $G _ { n }$ is $n ^ { 1 + \gamma }$ or larger – that is, just superlinear in the number of vertices and potentially far smaller than $O ( n ^ { 2 } )$ . It is easy to convince oneself that once the graphs have only a linear number of edges, we are flirting with disconnectedness, and there may simply be large mutant sets that can survive in isolation due to the lack of any incumbent interactions in certain games. Thus in some sense we examine the minimum plausible edge density.

Theorem 29.16 Let F be any 2-player, symmetric game, and suppose s is a classical ESS ofF. Let the infinite graphfamily $G = \{ G _ { n } \} _ { n = 0 } ^ { \infty }$ be drawn according $G _ { n , p } ,$ where $p = \Omega ( 1 / n ^ { c } )$ and $0 \leq c < 1$ . Then with probability 1, s is an ESS with respect to F and G.

A central idea in the proof is to divide mutants into two categories, those with “normal” fitness and those with “abnormal” fitness. Normal fitness means within a $( 1 \pm \tau )$ factor of the fitness given by the classical model, where τ is a small constant greater than 0, and abnormal fitness means outside of that range. We will use the lemma below (provided without proof) to bound the number of incumbents and mutants of abnormal fitness.

Lemma 29.17 For almost every graph $G _ { n , p }$ with $( 1 - \epsilon ) n$ incumbents, all but $\frac { 2 4 \log n } { \tau ^ { 2 } p }$ incumbents have fitness in the range $( 1 \pm \tau ) [ ( 1 - \epsilon ) F ( s | s ) + \epsilon F ( s | t ) ]$ where $p = \Omega ( 1 / n ^ { c } )$ and -, τ and c are constants satisfying $0 < \epsilon < 1 , 0 < \tau <$ $1 / 6 , 0 \leq c < 1$ . Similarly, under the same assumptions, all but $\frac { 2 4 \log n } { \tau ^ { 2 } p }$ mutants havefitness in the range $( 1 \pm \tau ) [ ( 1 - \epsilon ) F ( t | s ) + \epsilon F ( t | t ) ]$

With this lemma we first show that all but $o ( n )$ of the population (incumbent or mutant) have an incumbent neighbor of normal fitness. This will imply that all but $o ( n )$ of the mutants of normal fitness have an incumbent neighbor of higher fitness. The vehicle for proving this is the following result from random graph theory, which gives an upper bound on the number of vertices not connected to a sufficiently large set, U.

Theorem 29.18 Suppose $\delta = \delta ( n )$ and $C = C ( n )$ satisfy $\delta p n \ge 3$ log n, $C \geq$ $3 \log ( e / \delta )$ , and $C \delta n  \infty$ . Then almost every $G _ { n , p }$ is such that for every $U \subset$ $V , | U | = u = \lceil C / p \rceil$ the set $T _ { u } = \{ x \in V \setminus U \mid \Gamma ( x ) \cap U = \emptyset \}$ has at most δn elements.

This theorem assumes that the size of this large set U is known with equality, which necessitates the union bound argument below. The second main step of the proof uses Lemma 29.17 again, to show that there can be at most $o ( n )$ mutants with abnorma fitness. Since there are so few of them, even if none of them have an incumbent neighbor of higher fitness, s will still be an ESS with respect to $F$ and G.

proof (Sketch) Let $t \neq s$ be the mutant strategy. Since s is a classical ESS, there exists an $\epsilon _ { t }$ such that $( 1 - \epsilon ) F ( s | s ) + \epsilon F ( s | t ) > ( 1 - \epsilon ) F ( t | s ) + \epsilon F ( t | t )$ for all $0 < \epsilon < \epsilon _ { t }$ . Let M be any mutant family that is $\epsilon _ { t } { \mathrm { - l i n e a r } }$ . Thus for any fixed value of n that is sufficiently large, there exists an - such that $\lvert M _ { n } \rvert = \epsilon n$ and $\epsilon _ { t } >$ $\epsilon > 0$ . Also, let $I _ { n } = V _ { n } \backslash M _ { n }$ and let $I ^ { \prime } \subseteq I _ { n }$ be the set of incumbents that have fitness in the range $( 1 \pm \tau ) [ ( 1 - \epsilon ) F ( s | s ) + \epsilon F ( s | t ) ]$ for some constant $\tau , 0 <$ $\tau < 1 / 6 .$ . Lemma 29.17 shows $\begin{array} { r } { ( 1 - \epsilon ) n \geq | I ^ { \prime } | \geq ( 1 - \epsilon ) n - \frac { 2 4 \log n } { \tau ^ { 2 } p } } \end{array}$ . Finally, let

$$
T _ {I ^ {\prime}} = \{x \in V \setminus I ^ {\prime} \mid \Gamma (x) \cap I ^ {\prime} \neq \emptyset \}.
$$

(For the sake of clarity we suppress the subscript n on the sets $I ^ { \prime }$ and $T . )$ The union bound gives us

$$
\operatorname * {P r} \left(| T _ {I ^ {\prime}} | \geq \delta n\right) \leq \sum_ {i = (1 - \epsilon) n - \frac {2 4 \log n}{\tau^ {2} p}} ^ {(1 - \epsilon) n} \operatorname * {P r} \left(| T _ {I ^ {\prime}} | \geq \delta n \text {   and   } | I ^ {\prime} | = i\right).\tag{29.7}
$$

Letting $\delta = n ^ { - \gamma }$ for some $\gamma > 0$ gives $\delta n = o ( n )$ . We will apply Theorem 29.18 to the summand on the-right hand side of Equation 29.7. If we let $\gamma = ( 1 - c ) / 2$ , and combine this with the fact that $0 \leq c < 1$ , all of the require ments of this theorem will be satisfied (details omitted). Now when we apply this theorem to Equation 29.7, we get

$$
\begin{array}{l} \operatorname * {P r} (| T _ {I ^ {\prime}} | \geq \delta n) \leq \sum_ {i = (1 - \epsilon) n - \frac {2 4 \log n}{\tau^ {2} p}} ^ {(1 - \epsilon) n} \exp \left(- \frac {1}{6} C \delta n\right) \\ = o (1). \end{array}\tag{29.8}
$$

This is because Equation 29.8 has only $\frac { 2 4 \log n } { \tau ^ { 2 } p }$ terms, and Theorem 29.18 gives us that $\begin{array} { r } { C \ge ( 1 - \epsilon ) n ^ { 1 - c } - \frac { 2 4 \log n } { \tau ^ { 2 } } } \end{array}$ . Thus we have shown, with probability tending to 1 as $n \to \infty$ , at most $o ( n )$ individuals are not attached to an incumbent which has fitness in the range $( 1 \pm \tau ) [ ( 1 - \epsilon ) F ( s | s ) + \epsilon F ( s | t ) ]$ . This implies that the number of mutants of approximately normal fitness, not attached to an incumbent of approximately normal fitness, is also $o ( n )$ .

Now those mutants of approximately normal fitness that $a r e$ attached to an incumbent of approximately normal fitness have fitness in the range $( 1 \pm \tau ) [ ( 1 -$ $\epsilon ) F ( t | s ) + \epsilon F ( t | t ) ]$ . The incumbents that they are attached to have fitness in the range $( 1 \pm \tau ) [ ( 1 - \epsilon ) F ( s | s ) + \epsilon F ( s | t ) ]$ . Since s is an ESS of $F _ { ; }$ , we know (1 − $\epsilon ) F ( s | s ) + \epsilon F ( s | t ) > ( 1 - \epsilon ) F ( t | s ) + \epsilon F ( t | t )$ , thus if we choose τ small enough, we can ensure that all but $o ( n )$ mutants of normal fitness have a neighboring incumbent of higher fitness.

Finally by Lemma 29.17, we know that there are at most $o ( n )$ mutants of abnormal fitness. So even if all of them are more fit than their respective incumbent neighbors, we have shown all but $o ( n )$ of the mutants have an incumbent neighbor of higher fitness.

Next we briefly outline how to prove a converse to Theorem 29.16. Observe that if in the statement of Theorem 29.16 we let $c = 0$ , then $p = 1$ , which in turn, make $G =$ $\{ K _ { n } \} _ { n = 0 } ^ { \infty }$ , where $K _ { n }$ is a clique of n vertices. Then for any $K _ { n }$ all of the incumbents will have identical fitness and all of the mutants will have identical fitness. Furthermore, if s is an ESS for G, the incumbent fitness will be higher than the mutant fitness. Finally, one can show that as $n \to \infty$ , the incumbent fitness converges to $( 1 - \epsilon ) F ( s | s ) + \epsilon F ( s | t )$ and the mutant fitness converges to $( 1 - \epsilon ) F ( t | s ) + \epsilon F ( t | t )$ . In other words, s must be a classical ESS, providing a converse to Theorem 29.16.

## 29.5 Future Work

Most evolutionary game-theoretic models consider an infinite population of agents. These agents usually obey some simple dynamic such as imitation or replication. Typical results in these models show that in the limit (as time goes to infinity) the population converges to an equilibrium. A major open problem in the intersection of evolutionary game theory and theoretical computer science is to analyze a population of n agents, who obey one of these dynamics, and bound the time of convergence to an equilibrium. The notions of equilibrium and stability might have to be adapted to this new finite setting. Results along these lines would yield simple, distributed algorithms that agents could implement and converge to an equilibrium in a bounded (and hopefully short) amount of time. This would provide contribution beyond proving the existence of equilibria, and beyond showing that an infinite population will eventually converge to it. It will show that a population of a given size will converge to a stable equilibrium within a certain amount of time.

To start on this endeavor, the simplest models could consider n agents, where each agent could interact with each other agent. One example of such a problem would be to analyze a selfish routing model, such as the one described in Section 29.3, except with n agents, as opposed to infinitely many, and show a strongly polynomial time bound for their convergence. After baseline models such as this have been developed and studied, one might then try to find dynamics that result in these agents converging to an equilibrium that maximizes an appropriate notion of social welfare. Another extension would be to consider models where agents are arranged in a graph and can only interact with agents in their local neighborhood. One could then analyze not only the effect of the graph topology on equilibrium, as was done in Section 29.4, but also how it affects the convergence time.

It may turn out that hardness results stand in the way of such progress. Then one could try to bound the time of convergence to an approximate equilibrium, or simply bound the amount of time the population spends far away from an equilibrium. Also results such as the one given in Section 29.2 imply that there exist games for which it is hard to compute equilibria. There still could be many well-motivated classes of games for which arriving at an equilibrium is computationally tractable.

## 29.6 Notes

The motivation for evolutionary game theory and the description of the model, definitions, and dynamics were inspired by Smith (1982), Osborne and Rubinstein (1994), Weibull (1995), Hofbauer and Sigmund (1998), Kontogiannis and Spirakis (2005), and Kearns and Suri (2006). The Hawks and Doves game and its motivation come from Smith (1982), Osborne and Rubinstein (1994), Weibull (1995), and Alexander (2003).

The section on the computational complexity of ESS comes from Nisan (2006), which extended work by Etessami and Lochbihler (2004). Lemma 29.3 is a slight modification of a lemma in Motzkin and Straus (1965). Papadimitriou and Yannakakis (1982) show the problem of determining whether or not a graph has a maximum clique of size k is coD<sup>p</sup>-hard. We will not define the complexity class $\mathsf { c o D } ^ { p }$ here, but simply state that it contains both NP and coNP. Etessami and Lochbihler (2004) show that finding a strategy that is close in $\ell _ { p }$ norm to and ESS takes super-polynomial time unless P=NP. They also show that finding an ESS is in $\Sigma _ { 2 } ^ { P }$ , and that finding a regular ESS is NP-complete. In addition, they prove that counting the number of ESS and counting the number of regular ESS are both #P-hard.

Most of Section 29.3 comes from Fischer and Vocking (2004) and Fischer (2005).¨ For more details regarding the convergence of the imitative dynamics to a Nash flow, see those two references. We refer the reader to Brauer and Nohel (1969) for an excellent introduction into the Lyapunov framework. For a more extensive and technical treatment see Bhatia and Szego (1970). For applications of the Lyapunov framework¨ to other evolutionary game theoretic models and dynamics, see Weibull (1995) and Hofbauer and Sigmund (1998). There are many other places where evolutionary game theory is studied in conjunction with imitative dynamics, for example see Bjornerstedt¨ and Schlag (1996) and Schlag (1998) and chapter 4 of Weibull (1995).

There is a nice sequence of papers that continues the work of Fischer and Vocking¨ (2004) shown in Section 29.3. Fischer and Vocking (2005) consider a similar model¨ where agents may have stale information regarding the latencies of other paths. Fischer et al. (2006) consider a model where agents switch paths in a round based fashion.

Section 29.4 comes from Kearns and Suri (2006) . Vickery (1987) first noticed that a constant number of mutants may have higher fitness than the incumbents who are playing an ESS. Theorem 29.18 is Theorem 2.15 from Bollobas (2001) . In Kearns and´ Suri (2006), the authors give a pair of results dual to Theorem 29.16 and its converse. They show that if the graph is chosen adversarially, subject to some density restrictions, and the mutants are chosen randomly then ESS are preserved.

## Acknowledgments

The author gives many thanks to Michael Kearns, Simon Fischer, Berthold Vocking,¨ Larry Samuelson, Huanlei Ni, and Eyal Even-Dar for very helpful comments on an earlier draft of this chapter.

## Bibliography

J. M. Alexander. Probability and evolutionary game theory. London School ofEconomics and Politica Science, July 2003.

N.P. Bhatia and G.P. Szego.¨ Stability Theory ofDynamical Systems. Springer-Verlag, 1970.

J. Bjornerstedt and K.H. Schlag. On the evolution of imitative behavior. Discussion Paper B-378,¨ University of Bonn, 1996.

B. Bollobas.´ Random Graphs. Cambridge University Press, 2001.

F. Brauer and J.A. Nohel. The Qualitative Theory ofOrdinary Differential Equations. W.A. Benjamin, Inc., 1969.

K. Etessami and A. Lochbihler. The computational complexity of evolutionarily stable strategies. Technical Report TR04-055, Electronic Colloquium on Computational Complexity, 2004.

S. Fischer. Evolutionary game theory. Informatik 1, RWTH Aachen University, July 2005.

S. Fischer, H. Racke, and B. V¨ ocking. Fast convergence to Wardrop equilibria by adaptive sampling¨ methods. In Proc. 38th Symp. Theory ofComputing, pp. 653–662, 2006.

S. Fischer and B. Vocking. On the evolution of selfish routing. In¨ Proc. 12th Annual Euro. Symp. on Algorithms, pp. 323–334, 2004.

S. Fischer and B. Vocking. Adaptive routing with stale information. In¨ Proc. 24th Annual ACM SIGACT-SIGOPS Symp. Princ. ofDistributed Comput., pp. 276–283, 2005.

J. Hofbauer and K. Sigmund. Evolutionary Games and Population Dynamics. Cambridge University Press, 1998.

M. Kearns and S. Suri. Networks preserving evolutionary equilibria and the power of randomization. In Proc. 7th ACM Conf. on Electronic Commerce, 2006.

S. Kontogiannis and P. Spirakis. Evolutionary games: An algorithmic view. In O. Babaoglu, M. Jelasity, A. Montresor, C. Fetzer, S. Leonardi, A. van Moorsel, and M. van Steen, eds., Self-star Properties in Complex Information Systems: Conceptual and Practical Foundations, pp. 97–111. Springer-Verlag, 2005.

T.S. Motzkin and E.G. Straus. Maxima for graphs and a new proof of a theorem of Turan. Can. J. Math., 17:533–540, 1965.

N. Nisan. A note on the computational hardness of evolutionary stable strategies. Technical Report TR06-076, Electronic Colloquium on Computational Complexity, 2006.

M.J. Osborne and A. Rubinstein. A Course in Game Theory. The MIT Press, 1994.

C.H. Papadimitriou and M. Yannakakis. The complexity of facets (and some facets of complexity). In Proc. 14th Symp. Theory ofComputing, pp. 255–260, 1982.

K.H. Schlag. Why imitate and if so, how? J. Econ. Theory, 78:130–156, 1998.

J.M. Smith. Evolution and the Theory ofGames. Cambridge University Press, 1982.

W.L. Vickery. How to cheat against a simple mixed strategy ESS. J. Theor. Biol., 127:133–139, 1987.

J.W. Weibull. Evolutionary Game Theory. The MIT Press, 1995.

## Exercises

29.1 Find the ESS of Prisoners Dilemma.

29.2 In the game of Hawks and Doves, given by Figure 29.1, if $V \leq C ,$ , show that $V / C$ is a mixed strategy ESS. (Hint: Use the fact that for any mixed Nash equilibrium, s<sup>∗</sup> with support $s _ { 1 } , s _ { 2 } , . . . , s _ { k } , F ( s _ { 1 } | s ^ { * } ) = F ( s _ { 2 } | s ^ { * } ) = \cdot \cdot \cdot = F ( s _ { k } | s ^ { * } ) = F ( s ^ { * } | s ^ { * } ) )$

29.3 Consider a 2 × 2-symmetric game with four arbitrary constants for payoffs. Characterize the ESS for such a game in terms of the payoffs. Use this to conclude that any $2 \times 2 \cdot$ -symmetric game has an ESS.

29.4 Give an example of a game that has a Nash Equilibrium but no ESS.

29.5 Prove Lemma 29.7.

29.6 Show that $\textstyle \sum _ { p \in P } x _ { p } ^ { \prime } = 0 $ , where $ { \boldsymbol { X } } _ { p } ^ { \prime }$ is defined by Equation 29.2. Using this, conclude that ${ \mathrm { i f } } ,$ in the selfish routing model of Section 29.3, the imitative dynamics initially start with a feasible flow, then for all time the flow remains feasible.

29.7 Show that there exists a game such that with high probability for a family of random graphs with $p = \Omega ( 1 / n ^ { c } )$ and $0 \leq c < 1$ , an adversary can construct a mutant set such that there will exist at least one mutant with higher fitness than all of its incumbent neighbors.

AAE example, 466–467, 476 aborting games, 188, 190 adaptive behavior, 81 adaptive limited-supply auction, 424–427 adoption as coordination problem, 636 adverse selection, 677 advertisements. See sponsored search auctions affiliate search engines, 712 affine maximizer, 228, 317, 320 affinely independent, 57 agents. See players aggregation of preferences. See mechanism design aggregation problem, 651–655 algorithmic mechanism design. See also mechanism design; distributed algorithmic mechanism design allocation in combinatorial auction, 268, 270–272 AMD. See algorithmic mechanism design “AND” technology, 603–606 announcement strategies, 685–686 anonymous games, 40 anonymous rules, 247, 250 approximate core, 389–391 approximate equilibria, 45, 138, 143, 167 ApproximateTreeNash, 166–168, 176 approximation mechanisms, computationally efficient alternative solution concepts, 321–327 dominant strategy, impossibilities of, 317–320 history, 327 multidimensional domains, 310–317 overview, 301–303

single-dimensional domains, 303–310 submodularity, 623–624 theorems, 305, 307, 309, 315, 318, 324 Arrow–Debreu market model, 103, 104, 121–122, 136 Arrow’s theorem, 212–213, 239 artificial equilibrium, 61 ascending auctions, 289–294 ascending price auction, 126 assortative assignment, 704 asymmetries in information security, 636–639 atomic bids, 280, 282 atomic selfish routing, 461, 465–468, 470–472, 475–477, 482–483 atomic splittable model, 483 attribute auction, 344 auctions adaptive, limited-supply, 424–427 ascending, 289–294 bidding languages, 279–283 call market, 654–655 combinatorial. See combinatorial auctions competitive framework, 344–345 convergence rates, 342–344 deterministic optimal price, 340 digital goods, 332, 338, 340, 345–346 dynamic, with expiring items, 412, 420–424 examples in mechanism design, 209–210, 220–221 first price (Bayesian analysis), 20, 234–236 frugality, 350–354 iterative, 283–287 known single-minded combinatorial, 418 lower bounds, 346–347 profit maximization, 331–332, 336

auctions (cont.) random sampling optimal price (RSOP) auction, 341–342 random sampling profit extraction, 348–349 single-item, 332, 337 sponsored search auctions. See sponsored search auctions symmetric, 340 truthful combinatorial, 316–317 Vickrey auction. See Vickrey auction automated market makers, 662–665, 670 autonomous systems (ASes), 364–365, 370–371, 373–379, 487, 507 axiomatic method, 404 backward induction, 69 balanced flow, 111–116, 119 balls into bins problem, 451–452, 530 bandwidth-sharing game, 6–7, 452–455, 587, 588 banking and security, 634, 647 barter-based system, 600–601 basis matrix, 65 battle of the sexes game, 7, 12 Bayes’ rule, 667 Bayesian first price auction, 20 Bayesian-Nash implementation, 233–237, 416, 431–436 Bayesian network structured market, 662 Bayesian optimal mechanism design, 333, 335–338, 357 behavior strategy, 67 sequence form, 71 best response in graphical games, 162 and learning in games, 18 max-weight best response policy, 524 and Nash equilibrium, 30–31, 54, 497 in peer-to-peer networks, 605 polyhedron, 57–59 for identical machines. 522–524 in reputation systems, 686 in strict incomplete information games, 223 best response polyhedron, 57 BGP. See Border Gateway Protocol (BGP) bid format and price formation, 666–667 bid vector, 453–454 bidders bidding languages, 279–283 in combinatorial auctions, 267–268 exposure problem, 292 iterative auctions (query model), 283–287 single-minded, 270–275, 295, 323–324, 332

single-value, 322 sponsored search auctions. See sponsored search auctions bidding languages, 279–283, 295, 310 bilateral network formation game, 50 bilateral trade, 220–221 bimatrix game, 30, 54–57, 62, 152 binding inequality, 57–59 BitTorrent, 570, 589, 596, 600–601 blocking coalition, 253–255 blocking pair, 255, 256, 507 blogs, 622, 627, 630 BNIC. See Bayes-Nash incentive-compatible Bondareva–Shapley theorem, 388, 389, 391, 407 Boolean circuit, 41, 43 Boolean events, 658, 661 Boolean market model, 666, 668 bootstrapping problems, 636, 647, 689 Borda count, 211 Border Gateway Protocol (BGP), 372, 374, 376 378–379,381 bounded communication, 356 Braess’s Paradox, 464–465, 475, 481 Brandes’ algorithm, 645 brittle and nonbrittle comparators, 43 broadcast and secure channels, 185, 201 Brouwer’s fixpoint theorem, 32, 41–43 budget balanced, 22, 392, 393, 501 budget constraints. See sponsored search auctions bundle-price ascending auctions, 292–295 bundles of items. See combinatorial auctions bundling, 356 call market auction, 654–655 capacity augmentation, 479–480 capacity investments, 590 Cascade Model, 620–621, 624–625 cascading behavior in networks contagion threshold, 615–616 finding influential sets of nodes, 622–627 general social contagion, 618–622 history, 630–631 networked coordination games, 614–618 online data empirical studies, 627–630 overview, 613–614 theorems, 617, 618, 624–626 CE. See correlated equilibrium cell structure, 644–645 censorship resistance, 640–643 centrality attacks, 645 CEPE auction. See consensus estimate profi extraction (CEPE) auction

CES. See constant elasticity of substitution (CES)

cheap pseudonyms, 597, 679, 683

“cheap talk” preamble phase, 188

Chernoff bound, 532, 533–535

chicken game, 45–46

churn, 594

Clarke pivot rule, 219–221, 561

clearing prices. See market clearing prices

click through rate (CTR), 701–704, 707, 712

clique strategy, 644–646, 721–722

coalition game. See cooperative game theory coalition-proof equilibrium, 192

coalitions of agents, 250, See also collusions coarsest common refinement, 653

Cobb-Douglas functions, 139, 143, 146, 155 collective utility function, 405

collusion-proof ex-post Nash equilibria, 376 collusions, 189, 191, 199, 356, 597

combinatorial auctions

alternative solution concepts, 321–327

applications of, 269–270

ascending auctions, 289–294

bidding languages, 279–283

communication complexity, 287–289

computationally efficient mechanisms. See approximation mechanisms

history, 295–296

iterative auctions (query model), 283–287

definitions and problem, 267–269

linear programming relaxation, 275–277

multidimensional domains, 310–317

single-minded case, 270–275, 332, 418

theorems, 273, 277, 278, 282, 285, 288, 289, 291, 294

truthful, 316–317

Walrasian equilibrium, 277–279

combinatorial prediction markets, 657–662, 670

combined value trading, 658, 672

combining expert advice. See external regret Internet routing, 376–379

commitment types, 682

common value model, 238

communication complexity in combinatorial auctions, 287–289, 295

communications networks alternative pricing and incentive approaches, 587–590

efficiency analysis, 583–584

future research, 589–590

large networks (competitive models), 572–578

monopoly pricing and equilibrium, 582

oligopoly pricing and equilibrium, 582–583

overview, 571–572

pricing and efficiency with congestion externalities, 579–582

pricing and resource allocation theoretic models, 578–579, 584–587

theorems, 584, 585

compact prediction markets, 661–662

competitive analysis, 344–345, 351, 352–354, 413, 417, 421

competitive auctions, 345–349, 355

competitive digital goods auction, 345–346

competitive equilibrium definition, 292 large communications networks, 572–578 price takers, 546–547 smooth market-clearing mechanism, 552 social welfare, 293

competitive ratio, 345–348, 354, 357, 358, 422, 425

complementary slackness, 74

complementary slackness conditions, 104, 109

complements vs. substitutes, 268, 290, 292

complete information models, 239

completely labeled, 58, 59, 61–63, 66

complex networks and topology, 643–646

compound prediction markets, 659–661

computational aspects of prediction markets. See prediction markets

computational evolutionary game theory

classical evolutionary model, 718–720

computational complexity of evolutionarily stable strategies, 720–723

evolutionary dynamics applied to selfish routing, 723–728

future research, 733

graphs, 728–733

history, 733–734

overview, 717–718

theorems, 719, 723, 727, 731

computational indistinguishability, 185

computational manipulation example, 366–367

computationally efficient mechanisms. See approximation mechanisms

computer science and game theory, 363–364

computer science vs. economics, 301–303

concave games. See submodular games

conditional equilibrium, 164, 176

conditional securities, 659

Condorcet’s paradox, 211

congestion control algorithm, 576–577

congestion games, 41, 463, 482, 497–498, 579–582

consensus, 349–350

consensus estimates, 356

consensus estimate profit extraction (CEPE) auction, 350

constant elasticity of substitution (CES), 139, 149–151, 155

constant sum games, 89–90

constraint satisfaction programming (CSP), 169

consumer demand and constant elasticity of substitution, 149–150

consumer sovereignty (CS), 392

consumer surplus, 580

contact process, 630

contagion threshold, 615–618, 620

contingent claims/contracts. See prediction markets

continuous double market, 654, 662, 666–667

convergence, 342–344, 373, 523–524, 669; see also learning rates, 342–344, 523–524 times, 669

convex program, 104, 105–109, 112

convex programming for market equilibria

definitions, 136–137

equilibrium vs. optimization, 139–140

exchange economies and weak gross sustainability, 142–148

Fisher model with homogeneous consumers, 141–142

gross substitutability, 138

limitations, 150–152

models with production, 152–155

overview, 135–136, 155–156

tatonnement process, 137–138, 144, 147ˆ

utility function special forms, 139, 148–150

cooperative game theory barter-based system, 600–601 and cost sharing, 21–22, 385–387 graphical games, 177 overview, 20–21 in peer-to-peer networks, 588–589, 593, 596 reputation as incentive, 596–600 strong Nash equilibrium, 21 in wireless networks, 589

coordination game, 7–8, 614–618

coordination ratio. See price of anarchy

core, 22, 387–391, 402

correctness and privacy properties, 184, 194–195, 197

correlated equilibrium approximating, 48 definition, 46, 47, 90 ex ante, 196 in graphical games, 161–163, 169–175

Markov networks, 170–174 mediators, removing, 192–195 vs. Nash equilibria, 47–48 overview, 14–16, 45–47 regret minimization, 88–92 in succinct games, 48–49 and swap regret minimization, 90–91

cost benchmark, 352

cost function, 462, 663–665

cost matrix, 4, 5, 8

facility location game, 397–402

and fair division, 21–22, 347

games, 501

group-strategyproof mechanisms and cross-monotonic schemes, 391–394

history, 406–408

limitations of cross-monotonic schemes, 400–402

mechanism, 392

multicast transmission mechanism, 367–370

overview, 405–406

primal-dual schema, 394–400

Shapley value and Nash bargaining solution, 402–405

submodular game, 395–397

theorems, 388, 389, 391, 394, 396, 398, 401, 404, 405

costs. See also prices censorship, 642–643 defense vs. attack, 644 defining, 9 function, 9–10

Credence system, 597

critical payment, 274, 419, 430–431

critical values, 229

cross-monotonic cost-sharing schemes, 391–394, 396–397, 400–402

cryptography game theory influences on, 197–202 game theory notions and settings, 187–189 history, 203–204 influence on game theory, 191–197 multiparty computation, 181–182, 185–187 multiparty computation vs. games, 189–191 overview, 202 security of multiparty computation, 182–185

CS. See consumer sovereignty

currency-based p2p systems, 594, 601–602

DAMD. See distributed algorithmic mechanism design

decision making in uncertain environment, 79–81. See also regret analysis decision policy, 414 decomposition-based mechanism, 312–314 deferred acceptance algorithm, 256–258 degenerate games, 56, 65–66 delegation defense. 646 demand bundle, 284, 292–294 denial of service attacks, 634 derandomization, 355 design metric and inefficiency of equilibria, 454–456 design of scalable resource allocation mechanisms. See scalable resource allocation mechanisms deterministic algorithm, 308–309 deterministic optimal price auction, 340 dictatorship. 214. 247 diffusion of innovations, 613–614, 622, 627–630 digital goods auctions competitive model, 345–346 consensus estimation and truthfulness with high probability, 349–350 convergence rates, 342–344 decision problem, 347 definition, 332 theorems, 340 and virtual surplus, 338 diminishing returns, 621, 624–626, 628 direct reciprocity, 594 direct-revelation online mechanisms, 414–416 disagreement outcome, 404–405 discrete tatonnement process, 144, 147ˆ dispute wheel, 373–374, 378–380 distance-vector, 371 distributed algorithmic mechanism design (DAMD) vs. algorithmic mechanism design, 365, 380 combining networking and mechanism design perspectives, 376–379 history, 380–381 interdomain routing, 374–376 multicast transmission cost-sharing, 367–370 networking perspective, interdomain routing, 371–374 open problems, 380 overview, 363–365, 379–380 theorems, 369, 370, 378 of Vickrey–Clarke–Groves mechanisms, 366–367 distributed computation through markets, 665–669, 670–671 distributed mechanism, 375

distributed reputation systems, 693 distributed shortest-path routing, 481 divisible matching problem, 660–661 divisible vs. indivisible orders, 659 dominant strategies, 10–12, 91–92, 222–225, 317–320 dominant strategy incentive-compatible (DSIC), 415, 428, 430, 436 dominated strategy, 60 DOP auction. See deterministic optimal price auction double marginalization problem, 586 DSIC. See dominant strategy incentive-compatible dual growth process, 109–110 duopoly pricing, 580 dynamic aspects of sponsored search auctions, 707–711 dynamic attacks in reputation systems, 694 dynamic environments and online mechanism design, 413–417 dynamic parimutuel markets, 664–665 dynamic VCG mechanism, 433–434 dynamics of regret minimization, 99 early-arrival misreports, 415, 430 early stopping, 190 economics vs. computer science, 301–303 effective bandwidth pricing, 587 efficiency in sponsored search auctions, 703–705 efficient market hypothesis, 657, 672 egalitarian function, 443 Eigentrust algorithm, 597 Eisenberg–Gale program combinatorial algorithms, 104 convex, 105–108, 155 Nash bargaining solution, 402 primal-dual schema, 109 elastic traffic, 584–585 elasticity of substitution, 139 elections and mechanism design, 209. 211-212 electronic market design, 210 Ellipsoid method, 156 empirical distribution, 339–341 empirical Myerson mechanism, 339–341 empty threats, 195–196, 201 envy-freedom, 355, 712 epidemic. See cascading behavior in networks equilibria approximate, 45 artificial, 61 atomic flow, 466

equilibria (cont.) Bavesian-Nash, 235 complexity of finding, 16, 29–31 computational, 191 correlated, 14–16, 45–49 equilibrium price, 23, 25, 108–109, 135 finding. See finding equilibria graphical games. See graphical games for identical machines, 522–524, 529–533 inefficiency. See inefficiency of equilibria via labeled polytopes, 57–60 of markets. See market equilibria Nash. See Nash equilibrium nonatomic flow, 463 noncooperatively computable, 198 vs. optimization, 139–140 price characterization, 667–669 reduced strategic form, 69–70 regret analysis. See regret analysis sequence form, 73–74 in sponsored search auctions, 705–707 subgame perfect, 19–20, 68–69 for uniformly related machines, 524–529, 533–537 Wardrop. See Wardrop equilibria Euler’s identity, 142 evolutionarily stable strategy (ESS), 718–723, 729–731, 734 evolutionary game theory. See computational evolutionary game theory ex ante correlated equilibrium, 196 ex-post incentive compatible. See incentive compatible mechanisms ex-post Nash equilibrium, 375–376, 377–379 ExactTreeNash, 168, 177 exchange economy, 136, 142–148, 566 exchange model. See Arrow–Debreu model exclusivity, 197 existence and uniqueness in atomic selfish routing, 470–472 existence and uniqueness in nonatomic selfish routing, 468–470 expected capacity pricing, 587 exporting routes in BGP, 372–373 exposure problem, 292 expressiveness vs. simplicity in language, 279 extensive game, 40, 54, 66–68, 188–189, 195–197 external regret in constant sum games, 89–90 generic reduction to swap regret, 92–94 minimization of, 82–88 model, 81–82 overview, 80–81

and partial information model, 94–96 externality, 273, 579 facet, 57 facility location game and cost sharing, 386–387, 389–390 and network formation games, 502–506 open problems, 510–511 primal-dual scheme and cross-monotonicity, 397-402 Shapley values and, 40 fair division, 21–22 Fair, Optimal eXchange (FOX) protocol, 60 fair sharing rule, 489 fairness, 184, 194, 355, 501, 572, 581, 584, 639 faulty parties, 182–184, 186 FCC auctions, 269 feedback in reputation systems, 683–689 file-sharing game, 594–596, 640 finding equilibria PPAD, 36–39 complexity, 16 correlated equilibrium, 45–49 Lemke–Howson algorithm, 33–36 NP-completeness and Nash equilibrium, 31–33 overview, 29–31, 49–50 reduction to Brouwer, 41–45 succinct game representations, 39–41 first price auction (Bayesian analysis), 234–236, 335 first welfare theorem, 103, 277 Fisher’s model Arrow–Debreu model and, 121–122 concave utilities, 131 exchange model with proportional endowments, 140 with homogeneous consumers, 141–142 linear case, 104, 105–108, 121, 131 linear utilities, 121–122, 131 fitness function, 718–719, 729–732 fixed pricing, 588 fixpoint. See Brouwer's fixpoint theorem flat fees, 588 flow, 462, 463, 465, 468–470, 723 forecast, 653–654. See also prediction markets formation games and network design. See network formation games FPTAS. See fully polynomial time approximation schemes fractional allocations algorithm, 306–307 domain, 311 load function, 307

optimum, 314–315 free-market environment, 597–598 free-riding, 595, 597, 599, 601, 608, 637, 647 frugality, 350–354 full information model, 81 fully mixed equilibria, 529–533 fully mixed Nash equilibrium conjecture, 531 fully polynomial time approximation schemes (FPTAS), 607

gadgets, 42–43

game theory computational evolutionary. See computational evolutionary game theory and computer science, 363–364 cryptography, influences on, 197–202 efficiency, 191 and information security, 635–636 vs. issues in cryptography, 189 and multiparty computation. See multiparty computation (MPC) notions and settings, 187–189 and regret minimization, 88–92

game tree, 54, 68, 70, 72–74

games. See also specific game names and types aborting, 188, 190 battle of the sexes, 7–12 Bayesian, 20 best response and learning in, 18, 30–31 compactly represented, 9–10 cooperative, 20–22 cooperative and cost sharing, 385–387 coordination, 7–8, 614–620 cost sharing, 501 definition, 3, 88 graphical. See graphical games ISP routing, 4–5 matching pennies, 8–9 pricing, 14, 502 prisoners’ dilemma, 3–6, 443–444, 446–447, 595, 680, 681 repeated and online, 356 routing. See routing games routing congestion, 7–8, 96–99 simultaneous move, 9 standard form, 9–10 succinct representations of, 39–41 tragedy of the commons, 6–7, 595 transferable utility, 21, 385–386, 387–391 two-person zero-sum, 16–18, 73 ultimatum, 19–20 with turns, 18–20 Gao–Rexford conditions, 376–380

general equilibrium theory, 22–23, 103

General Threshold Model, 619–620, 626

generalized first price (GFP) auctions, 702 704–705

generalized median voter scheme (g.m.v.s.), 250, 251

generalized second price (GSP) auctions, 702, 704–706

generalized-WMON, 318–319

Gibbard–Satterthwaite theorem, 213–215, 243, 244

Gittins’ index policy, 435

global connection game, 488–489, 494–498, 500–502, 509–510

global trust values, 597

goods. See market equilibria

government policy and mechanism design, 210, 221

future research and open problems, 177

interdisciplinary benefits, 160

graphical exchange economies, 176–177, 178

Markov networks, 170–174

correlated equilibrium, 161–163, 169–175

Nash equilibrium in, 160–161

Nash equilibrium in tree graphical games, 164–169

overview, 159–161, 177–178

structural benefits, 160

greedy algorithms, 83–84, 315, 522

greedy auctions, 273–274, 422, 709

Green-Laffont, 368

grim-trigger strategy, 601, 681, 683

gross substitutability, 138, 145

group-strategyproof mechanisms, 391–394

GS. See gross substitutability

GSP auctions. See generalized second price (GSP) auctions

ham sandwich problem, 38

Hawks and Doves game, 719–720, 734

hill-climbing, 623–624, 630 hidden actions, 239, 594, 602–609, 636–638, 648

hiring-a-team auctions, 351

hiring, secretary problem, 424–425, 427

honest-but-curious parties, 182, 186, 197

honest parties, 182, 183

hot potato routing, 602

house allocation problem, 253–255, 262, 263

IC. See incentive compatible mechanisms idea futures. See prediction markets ideal model, 183 identity,682. See also reputation systems IDoWDS, 200–202 imitative dynamics of selfish routing model, 723–726, 734 importing routes in BGP, 372 improvement step, 519–520, 522–524, 528 incentive compatible differentiated pricing, 589–590 incentive compatible mechanisms approximation in ascending auctions, 286 characterizations of, 225–226 direct characterization, 226 interdomain routing, 375 mechanisms with money, 217–218 price uniqueness, 230–231 randomized mechanisms, 231–233 scalable resource allocation mechanisms, 560 single-minded bidders, approximation, 272–275 single-parameter domains, 228–230 social choice, 214, 215 weak monotonicity, 226–227 weighted Vickrey–Clarke–Grove mechanisms, 227–228 incentives and information security. See information security incentives for honest reporting, 690 incentives in communication networks. See communications networks incentives in peer-to-peer networks. See peer-to-peer networks (P2P) incomplete information games, 187–188, 222–223, 647 incremental cost-sharing, 403 incremental function, 620, 621, 624–626 incumbents, 717, 718, 720, 729–732 Independent Cascade Model, 621, 625 independent private values, 222–223 indirect reciprocity, 594, 596 individual rationality (IR), 219, 252, 333,419 see also voluntary participation indivisible matching problem, 659–660 indivisible order matching, 660, 661 inefficiency of equilibria communications networks. See communications networks as a design metric, 454–456 examples, 446–452 history, 456–457 measures of, 444–445

in network formation games. See network formation games overview, 443–444 price of anarchy, 445 price of stability, 446 in resource allocation. See scalable resource allocation mechanisms in routing games. See routing games in selfish load balancing. See selfish load balancing inequalities binding, 57–59 characterizing equilibrium, 154 correlated equilibrium, 46 irredundant, 57 Jensen’s, 727 infinite time horizon and discounting, 434 influential sets of nodes, 622–627, 630 information aggregation problem, 651–655 information cascades, 684 information markets. See prediction markets information-measuring software security, 638 information security censorship resistance economics, 640–643 complex networks and topology, 643–646 informational asymmetries, 636–639 insurance-based approaches to information security, 639 misaligned incentives, 634–636 overview, 633–634, 646–647 in reputation systems, 678 information set, 54, 67 initiation fee, 682 integer pivoting, 63–65 integrality gap, 314–316 interdependent values, 238–239 interdomain routing combining networking and mechanism design perspectives, 376–379 introduction, 370–371 mechanism design perspective, 374–376 networking perspective, 371–374 internal regret. See swap regret Internet Service Providers (ISPs), 4–5, 587 602 invisible hand, 217 Iowa Electronic Market (IEM), 655, 671 irrelevant information sets, 70–72 IR. See individual rationality item-price ascending auctions, 290–292, 295 iterated deletion of weakly dominated strategies (IDoWDS), 200–202 iterative auctions (query model), 283–287 iterative wrapper, 322

Jensen’s inequality, 727 job scheduling problem, 302–310 joint deviation. See coalitions of agents joint forecast, 653

K-rank-sybilproof, 691–692

k-resiliency, 191–194, 200

Karush-Kuhn-Tucker (KKT) conditions, 104, 106, 107, 109–110, 125, 128, 140, 141, 573, 575

Kelly’s model, 104–105, 124–125, 402

keyword auctions. See sponsored search auctions

kidney matching model, 262, 263

KKT conditions. See Karush-Kuhn-Tucker (KKT) conditions

known interesting-set assumption, 429–430

known single-minded combinatorial auction, 332

known single-minded (KSM) players, 323–324, 418

KP model. See load balancing games

Kuhn’s theorem, 71

labeled polytopes and equilibria, 57–60

labels, 57–60

Lagrangian function and multipliers, 173, 547, 556, 573–575, 578

liability, in information security, 634–636

limited misreports, 415, 419, 420, 423, 428–430

large actions spaces and regret minimization, 98

LH algorithm. See Lemke–Howson algorithm largest processing time (LPT) algorithm, 528–529

linear complementarity problem, 74

late-departure misreports, 415, 423, 430

linear exchange economies, 149

Leontief functions, 139, 152

linear programming relaxation, 260–261, 275–278, 284–285, 388, 395, 406

latency function, 96, 97, 584, 724,726; see also cost function

Linear Threshold Model, 619, 626

LCP. See linear complementarity problem leaders, 43

lattice formulation, 259–260, 263

local game matrices, 162

link-state, 371, 373

locally envy-free, 705–707

local-to-global link, 624, 626

Lipschitz continuous, 723–725

locally optimal solutions, 378

Lemke’s algorithm, 74

local neighborhood equivalence, 170–17

learning. See also regret analysis coordinated learning, 435 response and learning, 18, 30–31, 54

LiveJournal, 627–630

local effect games, 41

local connection game, 489–494, 506–509

logarithmic scoring rule, 686, 687

Lemke–Howson algorithm, 33–36, 59, 61–63, 391

load balancing games defining price of anarchy, 521–522 example, 520–521 history, 538–540 introduction to, 518–520 mixed equilibria on identical machines, 529–533 mixed equilibria on uniformly related machines, 533–537 overview, 517–518, 537–538 price of anarchy, 521–522 pure equilibria for identical machines, 522–524 pure equilibria for uniformly related machines. 524–529

loser-if-silent, 325

low communication, 544, 551–552

low-dimensional strategies, 544, 551–552, 564

lower bounds, 287–289, 346–347, 421

LP formulation. See linear programming relaxation

Lyapunov function, 575–576, 725–726, 734

MAB. See partial information mode

makespan minimization, 305–310, 450, 452, 517, 518, 525–530

malicious parties, 182

manipulation-resistant reputation systems. See reputation systems (manipulation-resistant)

marginal cost (MC), 368–370, 468

marginal cost pricing, 478–480, 588

marginal traders, 655

marginal utility, 562

market-based approaches to information security, 638–639

market clearing prices bid format and price formation, 666 definition, 23–24, 105 equilibrium price characterization, 668–669 proportional allocation mechanism, 545–546 rational expectations equilibrium, 656 in resource allocation, 555–557 smooth market-clearing mechanism, 552–553 and Walrasian equilibrium, 277

market equilibria

Arrow–Debreu model, 121–122

auction-based algorithm, 122–124

balanced flows, 111–115

combinatorial algorithms for, 103–105

convex programming limitations, 150–152

convex programming models with production, 152–155

convex programming techniques for, 135–141, 155–156

exchange economies and weak gross sustainability, 142–14

finding tight sets, 117–118

Fisher model with homogeneous consumers, 141–142

Fisher’s linear case and Eisenberg–Gale convex program, 105–108

graphical exchange economies, 176–177

and mechanism design, 209

open problems, 109

overview, 22–23, 131

prices as equilibrium prices, 108–109

in resource allocation markets, 124–125

simple algorithm, 23–26

single-source multiple-sink markets algorithm, 126–131

utility functions for, 148–150

market maker, 652, 654–655, 662–665, 670

market power, 454

market predictions. See prediction markets

market scoring rules, 663–664

marketing. See cascading behavior in networks; sponsored search auctions

Markov decision process, 432, 435

Markov networks, 170–174

Markov process, 93

matching. See stable matching problem

matching pennies game, 8–9

matching problem, 659–661

matrix form, 9–10

matroid, 353

maximal Nash subset, 66

maximum aggregate utility, 550–551

maximum flow, 112–114, 690, 692

MC. See marginal cost (MC)

McDiarmid’s inequality, 343

MDP. See Markov decision process model measures of inefficiency, 444–445

mechanism design Bayesian-Nash implementation, 233–237 Clarke pivot rule, 219–220 combinatorial auctions. See combinatorial auctions complete information models, 239

computationally efficient mechanisms. See approximation mechanisms

definition, 209

direct characterization of incentive compatible mechanisms, 226

distributed algorithmic. See distributed algorithmic mechanism design

examples and applications, 209–211

hidden actions, 239

history, 239–240

implementation in dominant strategies, 222–225

incentive compatible, 217–218, 225–226

interdependent values, 238–239

online. See online mechanism design

price uniqueness, 230–23

randomized mechanisms, 231–233

risk aversion model, 238

single-parameter domains, 228–230

social choice, 211–215

theorems, 213, 214, 219, 227–230, 232, 236

Vickrey auction, 216–217

Vickrey–Clarke–Groves mechanisms, 218–219

weak monotonicity, 226–227

weighted Vickrey–Clarke–Groves mechanisms, 227–228

mechanism design and profit maximization Bayesian optimal mechanism design, 335–338

examples and applications, 331–332

frugality, 350–354

history, 357–358

overview, 331–334

prior-free approximations to the optimal mechanism, 339–344

prior-free optimal mechanism design, 344–350

open problems, 354–357

theorems, 334, 336, 338, 340, 341, 343, 345, 346, 348, 353

truthful mechanisms, 333–334

mechanism design without money future research and open problems, 262 history, 263

house allocation problem, 253–255

lattice formulation, 259–260

overview, 243–244

single-peaked preferences over policies, 244–252

stable matchings, 255–262

theorems, 247, 251, 254, 256–258, 260, 261

median voter rule, 246

mediated games, 188

mediators, removing in correlated equilibrium 192-195 minimax theorem, 89–90 misreports, 415, 419, 420, 423, 428–430 mixed strategy bimatrix games and best response, 54 graphical games, 162, 167 introduction to, 8–9 in load balancing games, 518, 529–537 vs. pure strategies, 520–522 mixed strategy Nash equilibria, 13, 450–452 mobile ad hoc networks (MANETs), 602 model-free vs. model-based frameworks, 413 monopoly pricing and equilibrium, 580, 582 monotone algorithm for job scheduling, 305–310 monotone hazard rate, 337 monotonicity cross-, 392–393 deterministic policy, 418 in facility location problems, 505 in peer-to-peer networks, 606, 619, 623–624 progressive cascading behavior, 616–617 single-minded bidders, 274 weak, 226–227, 304–305, 307–309, 318–319, 428 Moulin’s theorem, 392–394, 402, 403, 407, 408 MPC. See multiparty computation multi-armed bandits problem (MAB). See partial information model multicast cost-sharing, 332, 367–370 multicommodity flow network, 462 multidimensional domains, 302, 310–317 multiparty computation (MPC) cryptographic influences on game theory, 191–197 existing results, 185–187 game theory influences on cryptography, 197–202 game theory notions and settings, 187–189 vs. games, 189–191 generalizations, 182 history, 203–204 overview, 181–182, 202 rational, 199–202 security of, 182–185 theorems, 185, 193, 199 multipath routing, 603 multiplayer games. See also graphical games; specific multiplayer games definitions, 161–163 graphical, 159–161 multiplication game, 42 mutants, 717, 718, 722, 729–732

Myerson’s mechanism, 337–339, 341–342, 357, 435, 703 myopic behavior, 667

Nash bargaining solution, 404–40 Nash equilibrium aggregate utility, 550–551 Bayesian-Nash implementation, 233–23 and bimatrix games, 54–57, 152 is a combinatorial problem, 31 computational, 191 and correlated equilibrium, 14–15, 163 in degenerate games, 66 and evolutionarily stable strategy, 719–720 finding. See finding equilibria and frugality, 352 in games with turns, 18–20 games without, 13–14 in graphical games, 160–162 inefficiency of equilibria, 446 k-resiliency, 194 and Lemke–Howson algorithm, 33–36, 61–63 mixed strategy, 13, 529–533 in network formation games, 488 and NP-completeness, 31–33 in potential games, 497, 499–500 in resource allocation games, 547–549 pure strategy, 12–13, 55, 519, 520, 528–529, 724 and regret minimization, 96–99 selfish routing, evolutionary dynamics of, 725–726 in Shapley network design games, 449–450 smooth market-clearing mechanism. 552–553 strong, 21 subgame perfect, 19–20, 68–69, 681–683 with succinct game representations, 39–41 symmetric, 30–31, 34 theorems, 13, 17, 34, 47 in tree graphical games, 164–169 in two-person zero-sum games, 16–18 without full information (Bayesian games), 20 Nashification, 529 NashProp, 161, 164, 168–169, 177–178 NCC. See noncooperatively computable (NCC) NE. See Nash equilibrium network complexity, 365, 367–370, 380, 381 network congestion games, 41 network formation games and facility location, 502–506 global connection games, 500–50 local connection games, 489–494, 506–509

network formation games (cont.) Nash equilibrium in potential games, 499-500 open problems, 508–511 overview, 448–450, 487–489 potential function method and price of stability, 498–499 potential games and congestion games, 497–498 potential games and global connection games, 494–497, 509–510 theorems, 491–493, 497, 498, 500, 501, 503, 505, 506 neutrality, 318, 320 no dispute wheel, 373–374, 378–380 no positive transfer (NPT), 392 no-trade theorems, 657, 663, 672 nonatomic selfish routing, 461–465, 468–470, 472–475, 478, 480–482, 499 noncooperatively computable (NCC), 197–199 nondegenerate, 56, 60 nondirect revelation, 223–224 nonlinear Pigou’s example, 464, 479 nonoblivious cost-sharing scheme, 501 nonprogressive vs. progressive processes, 616–617, 621–622 nontransferable utilities (NTU) in cooperative games, 385–386, 391,405. See also house allocation problem nonutilitarian, 518 normal form games, 161; see also standard form Northwest corner rule, 704, 712 NP-completeness and Nash equilibrium, 31–33, 271, 623, 661, 720, 723 NTU. See nontransferable utilities oblivious cost-sharing schemes, 501 oligopoly pricing and equilibrium, 582–583, 586 one-dimensional strategies, 564 one-shot simultaneous move games, 9 online allocation problem, 707–711 online mechanism adaptive, limited supply auction, 424–427 challenge of, 412–413 dynamic auction with expiring items, 420–424 dynamic environments, 413–417 dynamic Vickrey–Clarke–Groves mechanism, 433–434 ex-post incentive compatible, 428 future research, 435–436 history, 436–437 known interesting-set assumption, 429–430

Markov decision process model, 432

overview, 411–413

planning in model-based environments, 434–435

simple-price-based online auctions, 428

stochastic policies, 430–431

theorems, 419, 420, 422, 423, 426, 427, 430, 433

truthfulness for single-value preference domains, 417–420

onto condition, 245, 247, 249–252, 263

operationally complete market, 662

opportunistic unchoking mechanism, 600

opportunity cost, 708–709

optimal contract, 605–607

optimal sale price, 338, 341, 342

optimal single price profit, 345, 348

optimal stopping theory, 424–425

optimization program in sponsored search engines, 710

optimization vs. equilibrium, 139–140

option set for strategy-proofness, 248

OR bids, 280–283

“OR” technology, 603–604, 606, 607, 669

organisms, in evolutionary game theory, 717–718

P2P. See peer-to-peer networks (P2P) PageRank, 404, 406, 408, 597, 689–690, 692 pairwise stable equilibrium, 507, 615, 729 parallel information sets, 70 parallel-serial topologies, 585–586 Pareto-optimality, 103, 245, 249, 662 parimutuel games, 664–665 partial information model, 81, 94–96 parties in multiparty computation, 182–184, 193–194 partition model of knowledge, 653 path auctions, 351, 353, 354 path-vector, 371–373 Pathrank algorithm, 690 pay per click, 699, 701, 703, 707, 711 pay-your-dues (PYD) strategy, 682–683, 695 payment policy, 414–415, 422 payoffs in bimatrix games, 54, 55 defining, 9 evolutionarily stable strategy, 720–721 and inefficiency, 444, 453 in parimutuel games, 665 with risk-neutral players, 13 in scalable resource allocation mechanisms, 555 sequence form, 72–73

payoff matrix, 8, 12, 15

peer-prediction scoring, 686–689

peer-to-peer networks (P2P) barter-based system, 600–601 and censorship resistance, 640 currency as incentive, 601–602 pricing and incentive models, 588–589 file-sharing game, 594–596 hidden actions, 602–608, 637 history, 608–609 open problems, 608 overview, 593–594, 608 reputation as incentive, 596–600, 678 theorems, 607

peering, 377

perfect information, 67

perfect recall, 54, 71

perfect security, 184

phantom feedback, 679

Pigou’s example, 447–448, 456, 462–464, 469, 472–474, 479

Pigouvian taxes, 480, 580; see also margina cost pricing

pivoting, 63–65

players. See also bidders; specific games leaders, 43 limited information, 20 loser-if-silent, 325 in multiplayer games. See graphical games payoffs. See payoffs in peer-to-peer networks, 596 price anticipating, 547–549 price takers, 546–547, 573, 574 risk-neutral, 13 and transferable utility, 21–22 in two-person zero-sum games, 16–18

policy-consistency, 377–379

pollution game, 5–6

polyhedra, 53, 57

polynomial local search (PLS) problems, 499–500

polynomial parity argument (directed case). See PPAD

polynomial weights (PW) algorithm, 86–88

polytopes, 57–60, 65

population and strategy, 595–596, 613–614, 618–622. See also computational evolutionary game theory

positive association of differences (PAD), 318, 319

potential function method, 448, 468, 469, 471, 472, 482, 489, 494, 496

potential games congestion games, 497–498

facility location games, 503–504 global connection games, 494–497, 509–510 Nash equilibrium, 499–500 price of stability, 498–499 PPAD, 36–39, 151–152, 156 PPAD-complete, 16, 41–42, 44, 45 prediction markets automated market makers, 662–665 combinatorial, 657–662 definition. 651–652 distributed computation, 665–669 history, 671–672 open problems, 670–67 setup and notation, 652–654 survey of field, 654–657 theorems, 660, 661, 668, 669 preference ordering, 9 prices equilibrium, 123 price anticipating users (in resource allocation games), 547–549 price characterization, 667–669 price competition game, 583 price correspondences, 657 price discriminate, 545 price formation, 666–667 price of anarchy of atomic selfish routing, 459, 463–466, 468–470, 473–479, 480–481 coordination ratio, 456 definition, 445, 517, 520–522 facility location games, 504–505, 511 fully mixed Nash equilibrium, 531–533 in global connection games, 495 in local connection games, 491–494 mixed equilibria on uniformly related machines, 533 of nonatomic selfish routing, 463–464, 472–477, 481, 447–448 of the proportional sharing mechanism, 455–456 pure equilibria for identical machines, 522–523 pure equilibria for uniformly related machines, 524–528 pure vs. mixed equilibria, 537–538 reducing in routing games, 478–480 of scalable resource allocation mechanisms, 549–551, 558–559 in scheduling games, 451 utility games, 505, 507 price of stability, 446–449, 490–491, 495, 498–499, 520 price of unaccountability (POU), 605–607

price takers (in resource allocation games), 546–547, 573, 574, 576

prices automated market makers, 662–665 in communications networks. See communications networks for differentiated services, 587–588 equilibrium, 23, 25, 108–109, 135; see also market equilibria and information security, 638 market clearing, 23, 24, 105, 106, 122; see also market clearing prices in sponsored search auctions, 699–701 uniqueness of, 230–231

pricing game, 14, 502

primal-dual schema, 104, 109–110, 126, 291, 394–400, 407

Prim’s algorithm, 501

principal-agent model in peer-to-peer networks, 602–606

prior distribution, 333, 337, 339

prior-free mechanism design, 344–350 convergence rates, 342–344 empirical distributions, 339–341 random sampling, 341–342

prior probability distribution,653; see also Bayesian-Nash implementation

Prisoners’ dilemma, 3–6, 443–444, 446–447, 595, 680, 681

privacy and correctness properties, 184, 194–195, 197

probabilistic functions, 182, 186, 201, 620, 679

procurement auction, 220, 269

profit benchmark, 333, 344–345, 349, 350, 354

profit extraction problem, 347

profit extractor, 347–350, 358

profit maximization and mechanism design Bayesian optimal mechanism design, 335–338 in communications networks, 579–582 examples and applications, 331–332 frugality, 350–354 history, 357–358 overview, 331–334 prior-free approximations to the optimal mechanism, 339–344 prior-free optimal mechanism design, 344–350 future research, 354–357 theorems, 334, 336, 338, 340, 341, 343, 345, 346, 348, 350, 353

progressive vs. nonprogressive processes, 616–617, 621–622

proportional allocation mechanism, 544–551, 558, 564

proportional fairness, 125

proportional sharing, 452, 455–456

pseudonyms, 597, 679, 683

public good cost sharing, 251–252

pure strategy Nash equilibrium, 12–13, 55, 466, 519. 520. 528–529, 724

PW algorithm. See polynomial weights (PW) algorithm

quadratic scoring rule market maker, 664

quality of service (QoS), 587

query model (iterative auctions), 283–287, 310

random ordering, 403, 424, 427

random replenishment, 644

random sampling empirical Myerson, 341–342

random sampling optimal price (RSOP) auction, 341–346, 355, 357

random sampling profit extraction auction, 348–349

randomized-greedy (RG) algorithm, 83, 84

randomized incentive compatible mechanisms, 231–233

randomized rounding, 307–308

randomized scheduling algorithm, 307–308

randomized strategies, 8–9; see also mixed strategies

randomized weighted majority (RWM) algorithm, 85–86

rank-strategyproof, 690

rater reputations, 679–680, 684–688, 695

rational expectations equilibrium, 652, 656–657, 672

rational multiparty computation, 199–202

realization plan, 71–74

reciprocity, 594, 600

recommendation incentive programs, 626–627, 630

Red-Blue utility model, 640–641

reduced strategy, 69–70

reductions, 41–45

regret analysis external regret minimization, 82–88 generic reduction from external to swap regret, 92–94 lower bounds, 87–88 model, 81–82 overview, 80–81, 99 partial information model, 94–96 regret minimization and game theory, 88–92 regret minimization strategies in routing games, 96–99

theorems, 82–85, 87, 88 relative optimality, 333; see also competitive analysis replicator dynamics, 727 reputation as incentive, 594, 596–600, 678 reputation systems (manipulation-resistant) dynamics, 678 effect of, 680–683 eliciting effort and honest feedback, 683–689 history, 694–695 importance of, 677–680 meta-evaluation, 684 metrics and benchmarks in reputation systems, 694 open problems and extensions, 693–694 sybilproofness, 690–693 theorems, 691, 692 and transitive trust, 689–693 whitewashing, 682–683 residency matching, 255 resilient equilibrium, 191–192 resource allocation markets, 124–125, 452–454,573. See also communications networks; scalable resource allocation mechanisms revelation principle, 12, 224–225, 231, 234, 356, 416–417, 589 revenue equivalence, 236–237, 356, 705 revenue maximization. See profit maximization and mechanism design reverse auction, 220 ring structure, 644–645, 647 risk aversion model, 238 risk-neutral, 13 Roberts theorem, 228 rock-paper-scissors game, 44, 45 routing congestion game, 7–8, 96–99; see also routing games routing games atomic selfish routing, 465–468, 482–483 Braess’s Paradox, 464–465, 475, 481 existence and uniqueness, 468–470 vs. global connection games, 495 history, 480–483 network formation games. See network formation games nonatomic selfish routing, 462–465, 480–482 nonexistence in weighted atomic instances, 467 overview, 461–462 Pigou’s example, 447–448, 456, 462–464, 469, 472–474, 479 potential function, 470–472

price of anarchy in atomic selfish routing, 475–477 price of anarchy in nonatomic selfish routing, 472–475 reducing the price of anarchy, 478–480 theorems, 468, 471, 472, 476, 478, 479 routing matrix, 572, 575 routing protocol, 371–379 routing security, 636 RSEM. See random sampling empirical Myerson RSOP auction. See random sampling optimal price (RSOP) auction RSPE auction. See random sampling profit extraction auction RWM algorithm. See randomized weighte majority (RWM) algorithm satisfiability, 31–33, 500, 524, 529 scalable resource allocation mechanisms characterization theorem, 551–559 history, 565–566 overview, 543–544, 564 proportional allocation, 544–551 theorems, 546, 547, 549, 554 Vickrey–Clarke–Groves approach to, 559–563 scalar strategy VCG mechanisms, 559–563 scale-free networks, 643, 648 scheduling games. See load balancing games scheduling related machines, 303–304, 450–452, 577 scoring peer-prediction, 686–689 second-price auction. See Vickrey auction second welfare theorem, 278 secret-sharing, 186–187, 200, 201 secretary problem, 424–425, 427 secure and broadcast channels, 185 securities markets. See prediction markets security. See information security security of multiparty computation, 182–185, 190 security parameters, 185 seeder, 600 selfish load balancing. See load balancing game selfish routing, 447–448, 723–728; see also routing games semihonest parties, 182 sequence form, 70–74 sequential decision problem, 431, 437 serial connection, 585–586 service differentiation, 598–600 Shamir secret-sharing scheme, 186, 187, 20 Shapley cost-sharing mechanism, 495

Shapley network design game, 448–450; see also network formation games Shapley value, 22, 368–369, 402–405, 407–408, 489, 501 signal, 685, 687, 688 simple polytope, 60 simple pricing rules, 590 simultaneous move game, 9 simultaneous reporting game, 685 single-dimensional domains, 303–310; see single-parameter domains single-item auction, 332, 335, 337, 338, 351 single-minded bidders, 270–275, 295, 323–324, 332 greedy mechanism for single-minded bidders, 273–274 single-parameter domains, 228–230, 303–310, 350, 354, 356, 417–420 single-peaked preferences, 244–252 single-source multiple-sink markets algorithm, 126–131 single-value players, 322, 324–325 single-valued preference domains. See Single parameter domains slots, 699 smart market, 587 Smith, Adam, 217 smooth market-clearing mechanism, 552–554 social choice Arrow’s theorem, 212–213 Condorcet’s paradox, 211 Gibbard–Satterthwaite theorem, 213–215 and mechanism design, 209, 210 and mechanisms with money, 216–222 voting methods, 211–212 social choice function, 212–215, 225–226, 237, 405 social cost, 488, 490–491, 518, 520–522, 528 social network, 614–618, 622–625, 627, 637, 643.630 social welfare function, 212–213, 215, 218 socially efficient networks, 488, 490, 682–683 sock puppet identities. See phantom feedback; Sybil attacks software security, 638 solution concepts correlated equilibrium, 14–15 dominant strategy, 10–12 mixed strategy Nash equilibrium, 13 pure strategy Nash equilibrium, 12–13 source routing, 481, 603 spanning tree auctions, 351 sparse games, 40

specification faithfulness, 601 spectrum auctions, 269, 295 SPNE. See subgame perfect equilibrium sponsored search auctions discussion of practice, 712 dynamic aspects of, 707–711 equilibrium properties, 705–707 history, 712–713 models and mechanisms, 701–702 open problems, 711–712 overview, 699–701 static model, 702–707 theorems, 706, 709 stable matching problem college student matching, 255, 261 deferred acceptance algorithm, 256–258 extensions, 261–262 lattice formulation, 259–260 LP formulation, 260–261 overview, 255 stalling, 433 standard form, 9–10 statistical security, 184 Steiner forest problem, 406, 407, 495 Stirling’s formula, 288 stochastic policies, 430–431 strategic and privacy equivalence, 196 strategic form. See standard form strategic network formation, 594, 609; see also network formation games strategic voting, 211–212 strategy proof mechanism. See truthfulness strategy-proof rules, 243–251, 258, 262, 263, 690 strategy, 9, 10, 12, 18, 556, 561 strict equilibrium, 586 strict incomplete information, 222–223 strict quasi-concavity, 137 strong Nash equilibrium, 21 strong truthfulness, 415, 430 subgame perfect equilibrium, 19–20, 68–69, 681–683 subgames, 54 submodular function, 624–626, 630 submodular games, 395–397, 403, 504 submodularity, 623–626 substitutes vs. complements, 139, 268, 290 292 succinct game representations, 39–41, 48–49 supply and demand, 135; see also market equilibria support, 31, 34–36, 54 surplus sharing problem,386; see also cost sharing

surplus vector, 112, 121, 659–660 surplus, 119–121, 335–337,583. See also market equilibria swap regret definition, 80–82 and dominated strategies, 91–92 generic reduction from external to, 92–94 minimization and correlated equilibrium, 90–91 swarming download, 600, 601 Sybil attacks, 597, 601, 602, 608, 679, 680, 690–693 symmetric game, 30, 40, 45–46, 340 Tarski’s fixed point theorem, 259–260 tatonnement process, 137–138, 144, 147ˆ TCP congestion control, 104–105 thin market problem, 662 tit-for-tat strategy, 595, 596 top trading cycle mechanism, 254 traffic light example, 14–15 tragedy of the commons, 6–7, 595 transferable utilities (TU) in cooperative games, 21–22, 385–391 transitivity of trust, 679, 680, 689–693 tree graphical games, 164–169 TreeNash, 164–167, 176 trembling hand perfect equilibrium, 503 trusted parties, 182, 190 truthful with high probability truthfulness adaptive limited-supply auction, 425–426 automated market makers, 662–665 in canonical expiring items environment, 412 combinatorial auctions, 312–314 and dominant strategy incentive-compatible, 415 with high probability, 349–350 and profit maximization, 356–357 single-valued preference domains, 417–420 TU. See transferable utilities two-person zero-sum games, 16–18, 73 two-plaver game equilibrium computation bimatrix games and best response, 54–57 degenerate games, 65–66 extensive games, 66–68 further reading for, 75 integer pivoting, 63–65 via labeled polytopes, 57–60 Lemke-Howson algorithm, 61–63 overview, 53–54, 75–76 reduced strategic form, 69–70 sequence form, 70–73

ultimatum game, 19 uniqueness of prices, 230–231 unit demand, 280 upper envelope, 57, 59 users. See players utilitarian function, 443 utility, 331, 334, 357 utility function Cobb-Douglas, 139, 143, 146, 155 definition, 9–10 gross substitutability, 138, 145 in information security, 640–641 Leontief, 139, 152 market equilibria, 131, 148–150 maximizing with convex programs, 106 for scalable resource allocation mechanisms, 544–545, 556 special forms of, 139 valuation, 12, 20, 216–222, 238–240, 268, 331–334, 335–339, 355, 356, 374 value queries, 284 variational inequalities, 473–474 VCG mechanism. See Vickrey–Clarke–Groves mechanisms vertex-order attacks, 644–646 Vickrey auction, 11–12, 216–217, 220, 335, 422, 703–704 reserve price, 338 Vickrey–Clarke–Groves mechanisms and Clarke pivot rule, 219, 221 competitive communications network problems, 573 definition, 218–219 distributed implementation of, 366–367 in dynamic environments, 434–435 and frugality, 352–353 incentive compatible approximation, 273 marginal cost, 368–370 multidimensional domains and combinatorial auctions, 311 scalable resource allocation mechanisms, 559–564 and Walrasian equilibrium, 292 with scaler strategies, 559–563 weighted, 227–228 viral marketing, 622–623, 626–627, 630 virtual surplus, 336, 337, 338 virtual valuation, 335–336, 338 voluntary participation (VT), 392, 608; see also individual rationality voting and mechanism design, 209, 211–215, 246

voyeurism, 197

Walras’ Law, 137, 147

Walrasian equilibrium, 277–279, 290–292, 121–122

Walrasian model. See Arrow–Debreu mode

Wardrop equilibria, 480, 579–581, 724; see also equilibria nonatomic flow

Wardrop model of traffic flow, 96–98,585; see also selfish routing

weak gross substitutability, 131

weak gross sustainability (WGS), 138, 142–148

weak monotonicity, 226–227, 304–305, 307–309, 318–319, 428

weighted-packing problem, 271

threshold function, 669

weighted Vickrey–Clarke–Groves mechanisms, 227–228

WGS. See weak gross sustainability (WGS)

whitewashing attacks, 597, 601, 602, 608, 679, 682–683, 695

winner’s curse, 238

wireless networks, 577, 588, 589

“The Wisdom of Crowds”, 652

WMON. See weak monotonicity

worst-case analysis, 333, 357, 558; see also competitive analysis

XOR bids, 280–283, 668

Zermelo’s algorithm, 69

zero-sum games, 16–18, 73, 662