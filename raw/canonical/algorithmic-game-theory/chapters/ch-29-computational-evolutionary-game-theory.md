---
type: "book-chapter"
book_id: "algorithmic-game-theory"
chapter_id: "ch-29"
chapter_number: 29
chapter_title: "Computational Evolutionary Game Theory"
source_pdf: "raw/inbox/manual-drop/PDF_B.pdf"
source_page_start: 738
source_page_end: 775
printed_page_start: 738
printed_page_end: 775
part_ids: ["algorithmic-game-theory-ch-29-part-030"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Computational Evolutionary Game Theory

P1: SBT
9780521872829main       CUNY1061-Nisan       0 521 87282 0     July 5, 2007   16:22




                                                              CHAPTER 29


                                Computational Evolutionary
                                     Game Theory

                                                             Siddharth Suri




                                                                Abstract

                    This chapter examines the intersection of evolutionary game theory and theoretical computer science.
                    We will show how techniques from each field can be used to answer fundamental questions in the
                    other. In addition, we will analyze a model that arises by combining ideas from both fields. First, we
                    describe the classical model of evolutionary game theory and analyze the computational complexity
                    of its central equilibrium concept. Doing so involves applying techniques from complexity theory to
                    the problem of finding a game-theoretic equilibrium. Second, we show how agents using imitative
                    dynamics, often considered in evolutionary game-theory, converge to an equilibrium in a routing
                    game. This is an instance of an evolutionary game-theoretic concept providing an algorithm for
                    finding an equilibrium. Third, we generalize the classical model of evolutionary game theory to a
                    graph-theoretic setting. Finally, this chapter concludes with directions for future research. Taken as
                    a whole, this chapter describes how the fields of theoretical computer science and evolutionary game
                    theory can inform each other.



                                               29.1 Evolutionary Game Theory

                    Classical evolutionary game theory models organisms in a population interacting and
                    competing for resources. The classical model assumes that the population is infinite. It
                    models interaction by choosing two organisms uniformly at random, who then play a
                    2-player, symmetric game. The payoffs that these organisms earn represent an increase
                    or a loss in fitness, which either helps or hinders the organisms ability to reproduce.
                    In this model, when an organism reproduces, it does so by making an exact replica of
                    itself, thus a child will adopt the same strategy as its parent.
                       One of the fundamental goals of evolutionary game theory is to characterize which
                    strategies are resilient to small mutant invasions. In the classical model of evolutionary
                    game theory, a large fraction of the population, called the incumbents, all adopt the
                    same strategy. The rest of the population, called the mutants, all adopt some other
                    strategy. The incumbent strategy is considered to be stable if the incumbents retain
                    a higher fitness than the mutants. Since the incumbents are more fit, they reproduce
                                                                     717
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   16:22




                    718               computational evolutionary game theory

                    more frequently and the fraction of mutants in the population will eventually go to
                    0. Put another way, an evolutionarily stable strategy (ESS) is a strategy such that if
                    all the members of a population adopt it, then no mutant strategy could overrun the
                    population. We shall see in Section 29.1.1 that ESS are a refinement of Nash equilibria.
                       Replication is not the only type of dynamic studied in evolutionary game theory.
                    Imitation is another widely studied dynamic. In imitative dynamics, each agent initially
                    plays some pure strategy. As time goes on, agents interact pairwise. After this pairwise
                    interaction, if one agents sees the other agent earned a higher payoff, the agent with
                    the lower payoff may adopt, or imitate, the strategy of the agent who earned the higher
                    payoff. Imitative dynamics model, for example, a new idea, innovation, or fad spreading
                    through a population of individuals or firms.
                       In general, there are two main characteristics common to most evolutionary game
                    theoretic models. The first is that the population is infinite. The second is that players
                    adopt a very simple, local dynamic, such as replication or imitation, for choosing
                    and updating their strategies. These dynamics result in the agents learning from the
                    other agents in their environment; they provide a method for an equilibrium strategy
                    to emerge from the population. These types of dynamics explain how a population can
                    converge to an equilibrium. For example, Section 18.3.1 shows that equilibria for the
                    nonatomic selfish routing game exists, whereas Section 29.3 will show how agents
                    obeying imitative dynamics can converge to it.
                       Next we will formally describe the basic model of evolutionary game theory. Then,
                    in Section 29.2, we will analyze the computational complexity of finding and recog-
                    nizing stable strategies. After that, in Section 29.3, we will see an example of imitative
                    dynamics. We will apply imitative dynamics to the problem of selfish routing and show
                    how agents converge to an equilibrium. Finally, in Section 29.4, we will examine the no-
                    tion of stable strategies in a context where agents play against their local neighborhood
                    in a graph, as opposed to playing against another agent chosen uniformly at random.


                              29.1.1 The Classical Model of Evolutionary Game Theory
                    The classical model of evolutionary game theory considers an infinite population of
                    organisms, where each organism is assumed to be equally likely to interact with each
                    other organism. Interaction is modeled as playing a fixed, 2-player, symmetric game
                    defined by a fitness function F (we emphasize that the same game F is played in
                    all interactions). Let A denote the set of actions available to both players, and let
                    (A) denote the set of probability distributions or mixed strategies over A, then
                    F: (A) × (A) → . If two organisms interact, one playing a mixed strategy s and
                    the other playing a mixed strategy t, the s-player earns a fitness of F (s|t) while the
                    t-player earns a fitness of F (t|s).
                        In this infinite population of organisms, suppose that there is a 1 −  fraction who
                    play strategy s, and call these organisms incumbents, and suppose that there is an 
                    fraction who play t, and call these organisms mutants. Assume that two organisms are
                    chosen uniformly at random to play each other. The strategy s is an ESS if the expected
                    fitness of an organism playing s is higher than that of an organism playing t, for all
                    t = s and all sufficiently small . Since an incumbent will meet another incumbent
                    with probability 1 −  and it will meet a mutant with probability , we can calculate the
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   16:22




                                                evolutionary game theory                                   719

                    expected fitness of an incumbent, which is simply (1 − )F (s|s) + F (s|t). Similarly,
                    the expected fitness of a mutant is (1 − )F (t|s) + F (t|t). Thus we come to the formal
                    definition of an ESS.

                      Definition 29.1 A strategy s is an evolutionarily stable strategy (ESS) for
                      the 2-player, symmetric game given by fitness function F , if for every strategy
                      t = s, there exists an t such that for all 0 <  < t , (1 − )F (s|s) + F (s|t) >
                      (1 − )F (t|s) + F (t|t).

                       If one assumes that each organism reproduces asexually, and spawns a number
                    of offspring proportional to its fitness, then stable strategies will be those where the
                    incumbent population will reproduce more than any small mutant invasion. Thus the
                    mutant invasion will have fewer offspring and, in the long run, the fraction of mutants
                    in the population will tend to 0. In fact, a continuous time analysis of the replicator
                    dynamics shows that every ESS is asymptotically stable.
                       Definition 29.1 holds if and only if either of two conditions on s is satisfied ∀t = s:
                    (1) F (s|s) > F (t|s), or (2) F (s|s) = F (t|s) and F (s|t) > F (t|t). A consequence of
                    this alternate formulation of an ESS is that for s to be an ESS, it must be the case
                    that F (s|s) ≥ F (t|s), for all strategies t. This inequality means that s must be a best
                    response to itself, and thus for any ESS s, the strategy profile (s, s) must also be a Nash
                    equilibrium. This results in another, equivalent way to define an ESS.

                      Theorem 29.2 A strategy s is an ESS for a 2-player, symmetric game given by
                      fitness function F , if and only if (s, s) is a Nash equilibrium of F , and for every
                      best response t to s, where t = s, F (s|t) > F (t|t).

                    In general the notion of ESS is more restrictive than Nash equilibrium, and not all
                    2-player, symmetric games have an ESS.
                       Next, we give an example of a 2-player, symmetric game called Hawks and Doves,
                    and then solve for its ESS. The game of Hawks and Doves models two organisms
                    fighting over a resource. Obtaining the resource results in a gain of fitness of V , while
                    fighting for the resource and losing results in a fitness decrease of C. If a Hawk plays
                    a Dove, the Hawk will fight for the resource and the Dove will give up. This results in
                    a Hawk earning in increase of fitness of V , and the Dove’s fitness staying the same. If
                    two Doves play each other, they split the resource earning them both a fitness increase
                    of V /2. If two Hawks play, eventually one will win and one will lose, and it assumed
                    that each organism has a 1/2 chance of being the winner. Figure 29.1 shows the payoff
                    matrix for this game.
                       The strategy profile (D, D) is not a Nash Equilibrium because one player could
                    unilaterally deviate and play H and increase its payoff from V /2 to V . Since (D, D) is




                                           Figure 29.1. The game of Hawks and Doves.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   16:22




                    720               computational evolutionary game theory

                    not a Nash Equilibrium, D cannot be an ESS. Now, if V > C then H is an ESS. To see
                    this observe that F (H |H ) = (V − C)/2. Let t be any mixed strategy with probability
                    p < 1 of playing H and 1 − p of playing D, then F (t|H ) = p V −C     2
                                                                                             + (1 − p)0 <
                    (V − C)/2. Since F (H |H ) > F (t|H ) for all t = H , H is an ESS. We leave it as an
                    exercise for the reader (see Section 29.6) to show that if V ≤ C, the mixed strategy of
                    playing H with probability V /C and D with probability 1 − V /C is an ESS. Observe
                    that as V → C, the probability of playing H approaches 1. This coincides with the
                    pure strategy ESS of playing H when V > C.


                                          29.2 The Computational Complexity
                                           of Evolutionarily Stable Strategies

                    Next we show the computational complexity of finding an ESS given a 2-player
                    symmetric game is both NP-hard and coNP-hard. To prove this, we will make a
                    reduction from the problem of checking if a graph has a maximum clique of size
                    exactly k. Prior work has shown that this problem is both NP-hard and coNP-hard.
                    Along the way to proving the hardness of finding an ESS, we will see that the problem
                    of recognizing whether a given strategy is an ESS is also coNP-hard.
                       Next we will give the intuition behind the reduction. The reduction will transform
                    a graph G into a payoff matrix F which will have an ESS if and only if the size of the
                    largest clique in G is not equal to k. The reduction transforms the adjacency matrix
                    of G into the payoff matrix F by replacing all the diagonal entries with the value 1/2,
                    inserting a 0th row with each entry having a constant value, and inserting a 0th column
                    with each entry having the same constant value.
                       Informally speaking, for a mixed strategy s to be an ESS, incumbents should receive
                    a relatively high payoff when playing other incumbents. In order for a strategy s to
                    have this property for the game F , when s plays itself it must guarantee that the pure
                    strategies chosen will correspond to two adjacent vertices. One can see that having a
                    mixed strategy with support over a clique will achieve this. We will show in Lemma 29.3
                    that having support over a clique will result in a higher payoff than having support over
                    a dense subgraph that is not a clique. Having the diagonal entries consist of the constant
                    1/2 will help us prove this. This lemma will allow us to prove that when the size of
                    the maximum clique is greater than k, the uniform mixed strategy corresponding to
                    vertices of the clique will be an ESS. In addition, setting the 0th row and column of
                    F to a carefully chosen constant will give us a pure strategy ESS in the case where
                    the size of the maximum clique is less than k. This constant will also allow us to
                    show that there is no ESS in the case where the size of the maximum clique in G is
                    exactly k.
                       In describing this reduction, and for the rest of this chapter, we use the notation
                    F (s|t) to denote the payoff of the player playing strategy s when confronted with a
                    player playing strategy t. When we are referring to a specific entry in the payoff matrix
                    of F , we will use the notation F (i, j ) to denote the entry in the ith row and j th column.
                    Also, if s is a mixed strategy, we let si denote the probability that the pure strategy i
                    is played. (Thus we will use s and t to denote mixed strategies, and i and j to denote
                    indices into these mixed strategies, as well as indices into the payoff matrix F .)
P1: SBT
9780521872829main      CUNY1061-Nisan        0 521 87282 0              July 5, 2007                   16:22




                                           the computational complexity of ess                                                      721

                       The reduction from a graph G = (V , E) to a payoff matrix F that we consider works
                    as follows.
                    r for 1 ≤ i = j ≤ n: F (i, j ) = 1 if (i, j ) ∈ E and F (i, j ) = 0 if (i, j ) ∈
                                                                                                    /E
                    r for 1 ≤ i ≤ n: F (i, i) = 1/2
                    r for 0 ≤ i ≤ n: F (0, i) = F (i, 0) = 1 − 1/(2k)

                    To show that F has an ESS if and only if the size of the largest clique in G its not equal
                    to k, we will need the following technical lemma.

                      Lemma 29.3 If s is a strategy with s0 = 0, then F (s|s) ≤ 1 − 1/(2k  ), where
                      k  is the size of the maximum clique in G. This holds with equality if and only if s
                      is the uniform distribution over a k  -clique.

                      proof The proof is by induction on the number of nonedges between the
                      vertices in G = (V , E) corresponding to elements of the support set of s. The base
                      case is when there are 0 such non-edges, which means the vertices corresponding
                      to the support set of s form a k  -clique, where k  ≤ k. We assume, without loss
                      of generality, that the vertices in the k  -clique are numbered 1, 2, . . . , k  .
                                                         
                                             F (s|s) =                si sj F (i, j )
                                                                   i∈[k  ] j ∈[k  ]
                                                                                                        
                                                               =                          si sj −                    si2 /2
                                                                   i∈[k  ] j ∈[k  ]                  i∈[k  ]
                                                                                                                   
                                                               =               si                sj − 1/2                     si2
                                                                   i∈[k  ]        j ∈[k  ]                    i∈[k  ]
                                                                                        
                                                               ≤ 1 − 1/(2k )

                      The last inequality comes from the fact that when ||s||1 = 1, ||s||2 is minimized,
                      and the inequality is tight, only when all of the components of s are equal.
                      Conversely, if s is the uniform distribution over a k  -clique then, the inequality is
                      tight, which is shown as follows,
                                                                           
                                                   si sj F (i, j ) = 1/k 2        F (i, j )
                                          i∈[k  ] j ∈[k  ]                                           i∈[k  ] j ∈[k  ]
                                                                                                  2
                                                                                    = 1/k [k 2 − k  /2]
                                                                                    = 1 − 1/(2k  ).

                         For the inductive step, let u and v be two vertices such that (u, v) ∈          / E. We
                      construct a new strategy s  by moving the probability from v to u. So let su = su +
                      sv and sv = 0, and let the rest of the values of s  be identical to those of s. Since v is
                      no longer in the support set of s, we can  use the induction hypothesis  to conclude
                      that F (s  |s  ) ≤ 1 − 1/(2k  ). Let p = (u,w)∈E sw and let q = (v,w)∈E sw , and
                      without loss of generality assume that p ≥ q. By writing out the expressions
                      for F (s  |s  ) and F (s|s) one can show F (s  |s  ) = F (s|s) + 2sv (p − q) + su sv >
                      F (s|s). Thus, F (s|s) ≤ 1 − 1/(2k  ), which proves the inductive step.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0     July 5, 2007   16:22




                    722                computational evolutionary game theory

                       Now we will use this lemma to prove the necessary properties of the reduction. The
                    next two lemmas, when taken together, show that if the maximum size clique in G has
                    size not equal to k, then F has an ESS.

                      Lemma 29.4 If C is a maximal clique in G of size k  > k, and s is the uniform
                      distribution on C, then s is an ESS.

                      proof By Lemma 29.3, F (s|s) = 1 − 1/(2k  ). By the construction of the pay-
                      off matrix F , F (0|s) = 1 − 1/(2k) < F (s|s). Also, for any u ∈  / C, u is connected
                      to at most k  − 1 vertices in C, thus F (u|s) ≤ 1 − 1/k  < F (s|s). Thus any best
                      response to s must have support only over C. Furthermore, by Lemma 29.3 the
                      payoff of s against s is maximized when s is the uniform distribution over C. Thus,
                      s is a best response to itself. To prove that s is an ESS, it remains to show that for
                      all t = s, that are best responses, to s, F (s|t) > F (t|t). Again by Lemma 29.3,
                      F (t|t) < 1 − 1/(2k  ). Since C is a clique and s and t are distributions with sup-
                      port over C, using the structure of F one can compute that F (s|t) = 1 − 1/(2k  ).
                      Thus, F (s|t) > F (t|t) and s is an ESS.

                      Lemma 29.5 If the maximum size clique in G is of size k  < k then the pure
                      strategy 0 is an ESS.

                      proof For any mutant strategy t, F (t|0) = 1 − 1/(2k) = F (0|0), thus 0 is a
                      best response to itself. Next, we show that for any t not equal to the pure strategy
                      0, F (0|t) > F (t|t). To do so, we first show that we can assume that t places no
                      weight on the pure strategy 0. Let t ∗ be the strategy t with the probability of
                      playing the pure strategy 0 set to the value 0 and then renormalized. So, t0∗ = 0
                      and for i = 0, t ∗ = ti /(1 − t0 ). By writing out the expressions for F (t|t) and
                      F (t ∗ |t ∗ ), one can show F (t|t) = (2t0 − t02 )(1 − 1/(2k)) + (1 − 2t0 + t02 )F (t ∗ |t ∗ ).
                      Since F (0|t) = 1 − 1/(2k), F (0|t) > F (t|t) if and only if F (0|t) > F (t ∗ |t ∗ ).
                      Next, since the maximum size clique in G has size k  < k, applying Lemma 29.3
                      gives F (t ∗ |t ∗ ) ≤ 1 − 1/(2k  ) < 1 − 1/(2k) = F (0|t).

                       The next two lemmas, when combined, show that if the maximum size clique in G
                    has size exactly k, then F has no ESS.

                      Lemma 29.6 If the maximum size clique of G is at least k, then the pure strategy
                      0 is not an ESS.

                      proof Since F (0|0) = F (t|0) = 1 − 1/(2k) for any strategy t, the pure strategy
                      0 is a best response to itself. But, if t is the uniform distribution on the maximum
                      clique of G, which has size k  ≥ k, then by Lemma 29.3 F (t|t) = 1 − 1/(2k  ) ≥
                      F (0|t). By Theorem 29.2, this means the pure strategy 0 cannot be an ESS.

                      Lemma 29.7 If the maximum size clique of G is at most k, then any strategy
                      for F that is not equal to the pure strategy 0, is not an ESS for F .
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   16:22




                                evolutionary dynamics applied to selfish routing                         723

                      The proof of this lemma uses techniques similar to those used in Lemmas 29.5
                    and 29.6, so we leave it as an exercise for the reader (see Section 29.6).
                      Taking Lemmas 29.4, 29.5, 29.6, and 29.7 together, we get the following theorem.

                      Theorem 29.8 Given a 2-player, symmetric game F computing whether or not
                      F has an ESS is both NP-hard and coNP-hard.

                    Combining Lemmas 29.5 and 29.6 shows that it is coNP-hard to check whether a given
                    strategy is an ESS or not.

                      Theorem 29.9 Given a 2-player, symmetric game F and a strategy s, it is
                      coNP-hard to compute whether or not s in an ESS of F .

                      proof Lemmas 29.5 and 29.6 imply that G has maximum clique of size less
                      than k if and only if the pure strategy 0 is an ESS of F . Since the problem
                      of determining whether a graph has a maximum clique of size less than k is
                      coNP-hard, the problem of recognizing an ESS is also coNP-hard.

                       Theorems 29.8 and 29.9 imply that there exist games for which, in all likelihood,
                    efficient algorithms for finding and recognizing ESS do not exist. These results are
                    important because if finding an ESS for a given class of games is NP-hard, it is unlikely
                    that a finite population obeying some simple dynamic will quickly converge to it. But,
                    this observation does not mean that one should avoid using models based on ESS. It
                    simply means that to ensure the plausibility of a finite population model, one should
                    check whether it is computationally tractable to find the ESS of the games the model
                    considers. Moreover, this result does not directly imply that an infinite population,
                    however, cannot quickly converge to an equilibrium. In fact, the next section explores
                    the convergence time of an infinite population to an equilibrium.

                             29.3 Evolutionary Dynamics Applied to Selfish Routing

                    In this section we describe a method for applying evolutionary dynamics to the problem
                    of selfish routing. The model will consider an infinite population of agents, each of
                    which carries an infinitesimally small amount of flow in a network. The agents actions
                    allow them to change the path that they traverse; however, agents will not be allowed
                    to change their paths arbitrarily. The space of actions available to these agents will be
                    governed by simple, imitative dynamics. We show how agents selfishly seeking out
                    low latency paths, while obeying these imitative dynamics, converge to an approximate
                    equilibrium. First, we will formally describe the model which is similar to the nonatomic
                    selfish routing model shown in Section 18.2.1. Then, we will briefly outline a technique
                    that shows, in the limit, these dynamics converge to an equilibrium. Finally, we will
                    analyze the time of convergence to an approximate equilibrium.

                              29.3.1 The Selfish Routing Model with Imitative Dynamics
                    Let G = (V , E) be a network with latency functions le: [0, 1] →  defined over each
                    edge. We assume the latency functions are nonnegative, nondecreasing, and Lipschitz
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0        July 5, 2007       16:22




                    724               computational evolutionary game theory

                    continuous. We also assume that there is one unit of flow that is to be routed from a
                    source s to a sink t, and we let P denote the set of s-t paths in G. We also assume
                    that there are infinitely many agents, each of which carries an infinitesimally small
                    amount of flow. Let xp denote the fraction of flow that is being routed over path p.
                    Thus the vector x, which is indexed by the paths in P , will describe the flow over
                    G at a given point in time. A flow x is feasible if it routes 1 unit of flow from s
                    to t. Let xe = p e xp be the total load of an edge. The total latency of an edge is
                    denoted le (xe ) and thetotal latency of a path is the sum of the latencies of the edges
                    in the
                           path, lp ( x) =  e∈p le (xe ). Finally, the average latency of the entire network is
                    l̄ = p∈P xp lp (x).
                        Initially each agent is assumed to play an arbitrary pure strategy. Then at each
                    point in time, each agent is randomly paired with another agent and they compare
                    the latencies of their paths. If the latency of one agent’s path is less than the latency
                    of the other agent’s path, the agent experiencing higher latency switches to the lower
                    latency path with probability proportional to the difference in latencies. These imitative
                    dynamics model a source node gathering statistics on how long it takes for its packets
                    to reach the destination and changing the route accordingly. In Section 29.3.2 we will
                    describe why these dynamics will continue until the agents reach a Nash flow (also
                    called Wardrop equilibrium), which is a pure strategy Nash equilibrium for this routing
                    game, that we define next.

                      Definition 29.10 A feasible flow x is a Nash flow if and only if for all p, p  ∈ P
                      with xp > 0, lp (x) ≤ lp (x).

                    This definition ensures that, at a Nash flow, all s–t paths have the same latency (this is
                    precisely Definition 18.1 when restricted to the single commodity case). If we further re-
                    strict the latency functions to be strictly increasing, then Nash flows are essentially ESS.
                    We omit the proof of this since this section focuses on the convergence of the imitative
                    dynamics (we refer the interested reader to Section 29.6 for the appropriate references).
                        To analyze the convergence of these dynamics to either a Nash flow or an approx-
                    imate equilibrium, it is necessary to compute the rate of change of the amount of
                    flow over each path. Throughout this section we will use the notation x  to denote the
                    derivative with respect to time of the variable x, that is, x  = dx/dt. The following set
                    of differential equations describe the rate of change of the flow over each path.
                                                            
                                          xp = −xp                  xq λ(x)[lp (x) − lq (x)]
                                                        q∈P :lq (x)<lp (x)
                                                           
                                                 +                        xp xq λ(x)[lq (x) − lp (x)]    (29.1)
                                                     q∈P :lq (x)>lp (x)
                                                 
                                             =         xp xq λ(x)[lq (x) − lp (x)]
                                                 q∈P
                                                      ⎡                          ⎤
                                                                           
                                             = λ(x)xp ⎣  xq lq (x) − lp (x)   xq ⎦
                                                              q∈P                         q∈P

                                             = λ(x)xp [l̄(x) − lp (x)]                                   (29.2)
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   16:22




                                 evolutionary dynamics applied to selfish routing                         725

                    In this derivation, the function λ accounts for normalizing factors so that the probabili-
                    ties are bounded above by 1, and it accounts for the rate at which organisms are paired.
                    The first summation in Equation 29.1 represents the expected number of agents that
                    switch from path p to lower latency paths. The probability than an agent on path p is
                    paired with an agent of path q is equal to the fraction of agents using q, which is xq .
                    Then the agent using p would switch to q with probability lp (x) − lq (x). Multiplying
                    this product by xp gives the expected number of agents moving from p to a lower la-
                    tency path q. Similarly, the second summation of Equation 29.1 represents the number
                    of agents that switch to path p from a higher latency path. The rest of the derivation
                    results from straightforward algebraic manipulations.
                       Intuitively, Equation 29.2 says that paths with below average latency will have more
                    agents switching to them than from them; paths with above average latency will have
                    more agents switching from them than to them. In Section 29.3.3, where we bound
                    the time it takes for the system to converge to an approximate equilibrium, we would
                    like the rate of change of the population to be independent of the scale of the latency
                    functions. Thus we will replace λ(x) by l̄(x)−1 to give a relative rate of change.
                       While these equations resulted from imitative dynamics, the same equations can be
                    derived from a type of replication dynamic. In the literature, these equations are often
                    called the replicator dynamics. Now that we have defined the model and the dynamics,
                    we will show that the population of agents using imitative dynamics will converge to
                    an approximate equilibrium.


                                             29.3.2 Convergence to Nash Flow
                    It has been shown that as time goes to infinity, any initial flow that has support over
                    all paths in P will eventually converge to a Nash flow. In this section we give an
                    overview of the technique used to prove this. It is not clear how these techniques
                    could yield a bound on the time to convergence, so we do not go into specific details
                    of the proof. Since this text is focused on algorithmic game theory, we shall instead
                    give more attention to another result, shown in Section 29.3.3, that bounds the time of
                    convergence to an approximate equilibrium.
                        The main vehicle for proving that imitative dynamics converge to a Nash flow is
                    Lyapunov’s direct method. This is a general framework for proving that a system of
                    differential equations converges to a stable point, without necessarily knowing how
                    to solve the system of differential equations. Intuitively, this method works by first
                    defining a real valued potential function  that measures the potential energy of the
                    system of differential equations. The direct method requires that  be defined around
                    a neighborhood of a stable point and vanish at the stable point itself. Then, if one can
                    show that the dynamics of the system cause the potential function to decrease with
                    respect to time (along with a few other technical properties of the potential function),
                    Lyapunov’s theorems will imply that if the system reaches the neighborhood of the
                    stable point, the system will converge to the stable point. One drawback to this method
                    is that it provides no guidance for choosing such a potential function.
                        The argument that applies this method to the system of differential equations de-
                    scribed in Equation 29.2 works as follows. First, define  over the current flow such
                    that it will measure the total amount of latency the agents are experiencing. We will
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007       16:22




                    726               computational evolutionary game theory

                    define just such a function in the next section. Then, show that the imitative dynam-
                    ics cause  to decrease over time, and that  will achieve its minimum value at a
                    Nash flow. Applying one of the theorems in the Lyapunov’s framework allows one to
                    conclude that if the dynamics ever reach a neighborhood of an equilibrium, they will
                    converge to it. Finally, one has to show this neighborhood of convergence contains any
                    initial, feasible flow with support over all paths in P . This comes from the fact that the
                    dynamics cause the potential of any nonequilibrium flow to decrease and thus move
                    toward an equilibrium. Thus, in this model of selfish routing with imitative dynamics,
                    the Lyapunov framework allows one to show that the system will not get stuck in any
                    local minima and will converge to global minimum from any initial state with support
                    over all paths in P .


                                    29.3.3 Convergence to Approximate Equilibrium
                    In this section we will give a bound on how long it takes for the population of agents
                    using imitative dynamics to come to an approximate equilibrium.
                        One might consider using Euclidean distance between the current flow and an
                    equilibrium flow as a measure of approximation. To see intuitively why this is not a
                    suitable metric, consider a network and a flow where an  fraction of the agents uses a
                    path p, which has a latency that is slightly less than the current average latency. If it
                    were essential for an equilibrium to have a large fraction of the population using p, we
                    could take  to be arbitrarily small, which, by Equation 29.2, means we could make
                    xp arbitrarily small. Thus the imitative dynamics would cause the population to move
                    arbitrarily slowly to p, and therefore it would take arbitrarily long for the population
                    to approach, in Euclidean distance, a Nash flow. Thus, we define an -approximate
                    equilibrium next.

                      Definition 29.11 Let P be the paths that have   latency at least (1 + )l̄, that is
                      P = {p ∈ P | lp (x) ≥ (1 + )l̄}, and let x = p∈P xp be the fraction of agents
                      using these paths. A population x is said to be at an -approximate equilibrium if
                      and only if x ≤ .

                    This definition ensures at such an equilibrium that only a small fraction of agents expe-
                    rience latency significantly worse than the average latency. In contrast, the definition of
                    a Nash flow requires that all agents experience the same latency (see Definition 29.10).
                       To prove the convergence of these imitative dynamics to an approximate equilibrium,
                    we will make use of the following potential function. This function is one way to
                    measure the total amount of latency the agents experience.
                                                                 xe
                                                           ∗
                                                  (x) = l +            le (u)du                         (29.3)
                                                                 e∈E     0

                    The integral sums the latency each agent that traverses edge e would experience if the
                    agents were inserted one at a time. Summing this over each edge gives the total latency
                    that each agent would experience if they were entered into the network one at a time.
                    The term l ∗ denotes the minimum average latency of a feasible flow, l ∗ = minx l̄. We
                    add this term as a technicality that will help prove our bounds on the time convergence
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0      July 5, 2007     16:22




                                 evolutionary dynamics applied to selfish routing                         727

                    to approximate equilibrium. With the exception of the l ∗ term, this is the same potential
                    function described in Equation 18.3.

                      Theorem 29.12 The imitative dynamics converge to an -approximate equilib-
                      rium within time O( −3 ln(lmax / l ∗ )).

                       This proof works by analyzing the rate of change of  under the imitative dynamics.
                    If the current flow is not at an -approximate equilibrium, we can lower bound the
                    absolute rate of change of  in terms of l̄. We then lower bound l̄ in terms of ,
                    resulting in a differential inequality. Solving it leads to an upper bound on the time it
                    takes for  reach an approximate equilibrium.

                      proof We start by computing the derivative with respect to time of the potential
                      function .
                                                               
                                          =    xe le (xe ) =      xp le (xe )
                                                      e∈E                  e∈E p e

                         Next we substitute in the imitative dynamics, given by Equation 29.2. After
                      that we simplify the expression with the aim of using Jensen’s inequality.
                                             
                                        =          λ(x)xp [l̄(x) − lp (x)]le (xe )
                                                e∈E p e
                                                      
                                             = λ(x)             xp [l̄(x) − lp (x)]le (xe )
                                                      p∈P e∈p
                                                      
                                             = λ(x)         xp [l̄(x) − lp (x)]lp (xp )
                                                      p∈P
                                                                                 
                                             = λ(x) l̄(x)          xp lp (xp ) −         xp lp (x)2
                                                             p∈P                   p∈P
                                                                  
                                             = λ(x) l̄(x)2 −             xp lp (x)2                     (29.4)
                                                                   p∈P

                      Jensen’s inequality shows that this equation is bounded above by 0.
                         We would like to upper bound  . To do so, first observe as long as x is
                      not at an -approximate equilibrium, by definition at least an  fraction of the
                      population experiences latency at  least (1 + )l̄(x). Jensen’s inequality also shows
                      that for a fixed value of l̄(x), the p∈P xp lp (x)2 term is minimized when the less
                      expensive paths all have equal latency which we denote l  . Thus, for the purposes
                      of upper bounding  , we assume l̄ = (1 + )l̄ + (1 − )l  . Plugging this into
                      Equation 29.4 gives

                                         ≤ λ(x)[l̄(x)2 − (((1 + )l̄(x))2 + (1 − )l 2 )].
P1: SBT
9780521872829main      CUNY1061-Nisan    0 521 87282 0      July 5, 2007        16:22




                    728              computational evolutionary game theory

                          Now we substitute in l  = l̄ 1−−
                                                                2

                                                         1−
                                                              and perform some arithmetic giving,
                                                                   3
                                                      ≤ −λ(x)        l̄(x)2
                                                                  1−
                                                                  3
                                                           ≤ −λ(x) l̄(x)2 .
                                                                  2
                      We also replace λ(x) with l̄(x)−1 to measure the relative rate of change of  under
                      the imitative dynamics,
                                                                        3
                                                            ≤ −          l̄(x).                       (29.5)
                                                                        2
                          We can bound l̄ from below by /2 in the following way,
                                                                  
                                            l̄(x) =   xp lp (x) =   xp     le (xe )
                                                     p∈P                   p∈P        e∈p
                                                                                  
                                                 =              xp le (xe ) =             xe le (x)
                                                     e∈E p e                        e∈E
                                                       xe
                                                 ≥                  le (u)du.                           (29.6)
                                                     e∈E    0

                      The inequality holds because of the assumed monotonicity of the latency func-
                      tions. Now by the definition of l ∗ , it is easy to seethat l̄ ≥ l ∗ . Combining this
                      fact with Equation 29.6, we get that l̄ + l̄ ≥ l ∗ + e∈E 0 e le (u)du = . Thus
                                                                                      x

                      l̄ ≥ /2. Substituting this into Inequality 29.5, we get the following differential
                      inequality,
                                                            ≤ − 3 /4.
                          It can be shown via standard methods that any function of the following form
                      is a solution to the above inequality,
                                                         (t) ≤ (0)e− t/4 .
                                                                                3




                      Here (0) is given by the initial boundary conditions. Recall that this inequality
                      only holds as long as x is not an -approximate equilibrium. Thus, x must reach
                      an -approximate equilibrium when  reaches its minimum, ∗ , at the latest. So
                      we find the smallest t such that (t) ≤ ∗ ,
                                                                   (0)
                                                         t = 4 −3 ln    .
                                                                    ∗
                      It is easy to see that ∗ ≥ l ∗ and (0) ≤ 2lmax , which proves the theorem.


                                   29.4 Evolutionary Game Theory over Graphs

                    Next, we will consider a model similar to the classical model of evolutionary game
                    theory described in Section 29.1, but we will no longer assume that two organisms are
                    chosen uniformly at random to interact. Instead, we assume that organisms interact only
                    with those in their local neighborhood, as defined by an undirected graph or network.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    July 5, 2007   16:22




                                        evolutionary game theory over graphs                                   729

                    As in the classical setting (which can be viewed as the special case of the complete
                    network or clique), we shall assume an infinite population, by which we mean we
                    examine limiting behavior in a family of graphs of increasing size.
                        Before giving formal definitions, some comments are in order on what to expect
                    in moving from the classical to the graph-theoretic setting. In the classical (complete
                    graph) setting, there exist many symmetries that may be broken in moving to the
                    network setting, at both the group and individual level. Indeed, such asymmetries are
                    the primary interest in examining a graph-theoretic generalization.
                        For example, at the group level, in the standard ESS definition, one need not discuss
                    any particular set of mutants of population fraction . Since all organisms are equally
                    likely to interact, the survival or fate of any specific mutant set is identical to that of any
                    other. In the network setting, this may not be true: some mutant sets may be better able
                    to survive than others due to the specific topologies of their interactions in the network.
                    For instance, foreshadowing some of our analysis, if s is an ESS but F (t|t) is much
                    larger than F (s|s) and F (s|t), a mutant set with a great deal of “internal” interaction
                    (i.e., edges between mutants) may be able to survive, whereas one without this may
                    suffer. At the level of individuals, in the classical setting, the assertion that one mutant
                    dies implies that all mutants die, again by symmetry. In the network setting, individual
                    fates may differ within a group all playing a common strategy. These observations imply
                    that in examining ESS on networks we face definitional choices that were obscured in
                    the classical model.
                        If G is a graph representing the allowed pairwise interactions between organisms
                    (vertices), and u is a vertex of G playing strategy su , then the fitness of u is given by
                                                               
                                                                  v∈(u) F (su |sv )
                                                     F (u) =                         .
                                                                     |(u)|

                    Here sv is the strategy being played by the neighbor v, and (u) = {v ∈ V : (u, v) ∈ E}.
                    One can view the fitness of u as the average fitness u would obtain if it played each of
                    its neighbors, or the expected fitness u would obtain if it were assigned to play one of
                    its neighbors chosen uniformly at random.
                        Classical evolutionary game theory examines an infinite, symmetric population.
                    Graphs or networks are inherently finite objects, and we are specifically interested in
                    their asymmetries, as discussed above. Thus all of our definitions shall revolve around
                    an infinite family G = {Gn }∞ n=0 of finite graphs Gn over n vertices, but we shall examine
                    asymptotic (large n) properties of such families.
                        We first give a definition for a family of mutant vertex sets in such an infinite graph
                    family to contract.

                      Definition 29.13 Let G = {Gn }∞     n=0 be an infinite family of graphs, where Gn
                                                      ∞
                      has n vertices. Let M = {Mn }n=0 be any family of subsets of vertices of the Gn
                      such that |Mn | ≥ n for some constant  > 0. Suppose all the vertices of Mn play
                      a common (mutant) strategy t, and suppose the remaining vertices in Gn play
                      a common (incumbent) strategy s. We say that Mn contracts if for sufficiently
                      large n, for all but o(n) of the j ∈ Mn , j has an incumbent neighbor i such that
                      F (j ) < F (i).
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   16:22




                    730               computational evolutionary game theory

                        A reasonable alternative would be to ask that the condition above holds for all
                    mutants rather than all but o(n). Note also that we only require that a mutant have
                    one incumbent neighbor of higher fitness in order to die; one might consider requiring
                    more. In Section 29.6 we ask the reader to consider one of these stronger conditions
                    and demonstrate that our results can no longer hold.
                        To properly define an ESS for an infinite family of finite graphs in a way that recovers
                    the classical definition asymptotically in the case of the family of complete graphs, we
                    first must give a definition that restricts attention to families of mutant vertices that
                    are smaller than some invasion threshold   n, yet remain some constant fraction of the
                    population. This prevents “invasions” that survive merely by constituting a vanishing
                    fraction of the population.

                      Definition 29.14 Let   > 0, and let G = {Gn }∞         n=0 be an infinite family of
                      graphs, where Gn has n vertices. Let M = {Mn }∞        n=0 be any family of (mutant)
                                                               
                      vertices in Gn . We say that M is  -linear if there exists an ,   >  > 0, such
                      that for all sufficiently large n,   n > |Mn | > n.

                      We can now give our definition for a strategy to be evolutionarily stable when
                    employed by organisms interacting with their neighborhood in a graph.

                      Definition 29.15 Let G = {Gn }∞       n=0 be an infinite family of graphs, where Gn
                      has n vertices. Let F be any 2-player, symmetric game for which s is a strategy.
                      We say that s is an ESS with respect to F and G if for all mutant strategies
                      t = s, there exists an t > 0 such that for any t -linear family of mutant vertices
                      M = {Mn }∞  n=0 all playing t, for n sufficiently large, Mn contracts.


                        Thus, to violate the ESS property for G, one must witness a family of mutations M in
                    which each Mn is an arbitrarily small but nonzero constant fraction of the population of
                    Gn , but does not contract (i.e., every mutant set has a subset of linear size that survives
                    all of its incumbent interactions). One can show that the definition given coincides with
                    the classical one in the case where G is the family of complete graphs, in the limit of
                    large n. We note that even in the classical model, small sets of mutants were allowed
                    to have greater fitness than the incumbents, as long as the size of the set was o(n).
                        In the definition above there are three parameters: the game F , the graph family G,
                    and the mutation family M. Our main results will hold for any 2-player, symmetric
                    game F . We will study a rather general setting for G and M: that in which G is a family
                    of random graphs and M is arbitrary. We will see that, subject to conditions on degree
                    or edge density (essentially forcing connectivity of G but not much more), for any 2-
                    player, symmetric game, the ESS of the classical settings, and only those strategies, are
                    always preserved. Thus, for the purposes of characterizing stable strategies, the classical
                    method of pairing organisms at random, is equivalent to randomizing the graph.

                                     29.4.1 Random Graphs, Adversarial Mutations
                    We now proceed to state and prove the random graph result in the network ESS model.
                    We consider a setting in which the graphs are generated via the Gn,p model of Erdös and
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   16:22




                                        evolutionary game theory over graphs                               731

                    Rényi. In this model, every pair of vertices is joined by an edge independently and with
                    probability p (where p may depend on n). The mutant set, however, will be constructed
                    adversarially (subject to the linear size constraint given by Definition 29.15). For this
                    setting, we show that for any 2-player, symmetric game, s is a classical ESS of that
                    game, if and only if s is an ESS for {Gn,p }∞n=0 , where p = (1/n ) and 0 ≤ c < 1, and
                                                                                         c
                                               ∞
                    any mutant family {Mn }n=0 , where each Mn has linear size. We note that under these
                    settings, if we let c = 1 − γ for small γ > 0, the expected number of edges in Gn is
                    n1+γ or larger – that is, just superlinear in the number of vertices and potentially far
                    smaller than O(n2 ). It is easy to convince oneself that once the graphs have only a linear
                    number of edges, we are flirting with disconnectedness, and there may simply be large
                    mutant sets that can survive in isolation due to the lack of any incumbent interactions
                    in certain games. Thus in some sense we examine the minimum plausible edge density.

                      Theorem 29.16 Let F be any 2-player, symmetric game, and suppose s is a
                      classical ESS of F . Let the infinite graph family G = {Gn }∞
                                                                                  n=0 be drawn according
                      to Gn,p , where p = (1/n ) and 0 ≤ c < 1. Then with probability 1, s is an ESS
                                                    c

                      with respect to F and G.

                       A central idea in the proof is to divide mutants into two categories, those with
                    “normal” fitness and those with “abnormal” fitness. Normal fitness means within a
                    (1 ± τ ) factor of the fitness given by the classical model, where τ is a small constant
                    greater than 0, and abnormal fitness means outside of that range. We will use the lemma
                    below (provided without proof) to bound the number of incumbents and mutants of
                    abnormal fitness.

                      Lemma 29.17 For almost every graph Gn,p with (1 − )n incumbents, all but
                      24 log n
                        τ 2p
                               incumbents have fitness in the range (1 ± τ )[(1 − )F (s|s) + F (s|t)],
                      where p = (1/nc ) and , τ and c are constants satisfying 0 <  < 1, 0 < τ <
                      1/6, 0 ≤ c < 1. Similarly, under the same assumptions, all but 24τlog2p
                                                                                              n
                                                                                                mutants
                      have fitness in the range (1 ± τ )[(1 − )F (t|s) + F (t|t)].

                       With this lemma we first show that all but o(n) of the population (incumbent or
                    mutant) have an incumbent neighbor of normal fitness. This will imply that all but o(n)
                    of the mutants of normal fitness have an incumbent neighbor of higher fitness. The
                    vehicle for proving this is the following result from random graph theory, which gives
                    an upper bound on the number of vertices not connected to a sufficiently large set, U .

                      Theorem 29.18 Suppose δ = δ(n) and C = C(n) satisfy δpn ≥ 3 log n, C ≥
                      3 log(e/δ), and Cδn → ∞. Then almost every Gn,p is such that for every U ⊂
                      V , |U | = u = C/p the set Tu = {x ∈ V \ U | (x) ∩ U = ∅} has at most δn
                      elements.

                    This theorem assumes that the size of this large set U is known with equality, which
                    necessitates the union bound argument below. The second main step of the proof uses
                    Lemma 29.17 again, to show that there can be at most o(n) mutants with abnormal
                    fitness. Since there are so few of them, even if none of them have an incumbent neighbor
                    of higher fitness, s will still be an ESS with respect to F and G.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0      July 5, 2007           16:22




                    732              computational evolutionary game theory

                      proof (Sketch) Let t = s be the mutant strategy. Since s is a classical ESS,
                      there exists an t such that (1 − )F (s|s) + F (s|t) > (1 − )F (t|s) + F (t|t),
                      for all 0 <  < t . Let M be any mutant family that is t -linear. Thus for any fixed
                      value of n that is sufficiently large, there exists an  such that |Mn | = n and t >
                       > 0. Also, let In = Vn \Mn and let I  ⊆ In be the set of incumbents that have
                      fitness in the range (1 ± τ )[(1 − )F (s|s) + F (s|t)] for some constant τ , 0 <
                      τ < 1/6. Lemma 29.17 shows (1 − )n ≥ |I  | ≥ (1 − )n − 24τlog         n
                                                                                           2 p . Finally, let


                                              TI  = {x ∈ V \ I  | (x) ∩ I  = ∅}.
                      (For the sake of clarity we suppress the subscript n on the sets I  and T .) The
                      union bound gives us
                                                             
                                                            (1−)n
                                 Pr (|TI  | ≥ δn) ≤                        Pr (|TI  | ≥ δn and |I  | = i).   (29.7)
                                                       i=(1−)n− 24 log
                                                                    2
                                                                        n
                                                                     τ p


                         Letting δ = n−γ for some γ > 0 gives δn = o(n). We will apply Theo-
                      rem 29.18 to the summand on the-right hand side of Equation 29.7. If we let
                      γ = (1 − c)/2, and combine this with the fact that 0 ≤ c < 1, all of the require-
                      ments of this theorem will be satisfied (details omitted). Now when we apply this
                      theorem to Equation 29.7, we get
                                                               
                                                              (1−)n            
                                                                                  1
                                       Pr(|TI  | ≥ δn) ≤                    exp − Cδn             (29.8)
                                                                    24 log n
                                                                                  6
                                                              i=(1−)n−
                                                                              τ2p

                                                           = o(1).
                      This is because Equation 29.8 has only 24τlog
                                                                 2p
                                                                    n
                                                                      terms, and Theorem 29.18 gives
                      us that C ≥ (1 − )n1−c − 24 τlog2
                                                         n
                                                           . Thus we have shown, with probability tending
                      to 1 as n → ∞, at most o(n) individuals are not attached to an incumbent which
                      has fitness in the range (1 ± τ )[(1 − )F (s|s) + F (s|t)]. This implies that the
                      number of mutants of approximately normal fitness, not attached to an incumbent
                      of approximately normal fitness, is also o(n).
                         Now those mutants of approximately normal fitness that are attached to an
                      incumbent of approximately normal fitness have fitness in the range (1 ± τ )[(1 −
                      )F (t|s) + F (t|t)]. The incumbents that they are attached to have fitness in the
                      range (1 ± τ )[(1 − )F (s|s) + F (s|t)]. Since s is an ESS of F , we know (1 −
                      )F (s|s) + F (s|t) > (1 − )F (t|s) + F (t|t), thus if we choose τ small enough,
                      we can ensure that all but o(n) mutants of normal fitness have a neighboring
                      incumbent of higher fitness.
                         Finally by Lemma 29.17, we know that there are at most o(n) mutants of
                      abnormal fitness. So even if all of them are more fit than their respective incumbent
                      neighbors, we have shown all but o(n) of the mutants have an incumbent neighbor
                      of higher fitness.

                        Next we briefly outline how to prove a converse to Theorem 29.16. Observe that if
                    in the statement of Theorem 29.16 we let c = 0, then p = 1, which in turn, makes G =
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 5, 2007   16:22




                                                               notes                                        733

                    {Kn }∞n=0 , where Kn is a clique of n vertices. Then for any Kn all of the incumbents will
                    have identical fitness and all of the mutants will have identical fitness. Furthermore, if s
                    is an ESS for G, the incumbent fitness will be higher than the mutant fitness. Finally, one
                    can show that as n → ∞, the incumbent fitness converges to (1 − )F (s|s) + F (s|t),
                    and the mutant fitness converges to (1 − )F (t|s) + F (t|t). In other words, s must be
                    a classical ESS, providing a converse to Theorem 29.16.


                                                       29.5 Future Work

                    Most evolutionary game-theoretic models consider an infinite population of agents.
                    These agents usually obey some simple dynamic such as imitation or replication.
                    Typical results in these models show that in the limit (as time goes to infinity) the
                    population converges to an equilibrium. A major open problem in the intersection of
                    evolutionary game theory and theoretical computer science is to analyze a population
                    of n agents, who obey one of these dynamics, and bound the time of convergence to an
                    equilibrium. The notions of equilibrium and stability might have to be adapted to this
                    new finite setting. Results along these lines would yield simple, distributed algorithms
                    that agents could implement and converge to an equilibrium in a bounded (and hopefully
                    short) amount of time. This would provide contribution beyond proving the existence
                    of equilibria, and beyond showing that an infinite population will eventually converge
                    to it. It will show that a population of a given size will converge to a stable equilibrium
                    within a certain amount of time.
                       To start on this endeavor, the simplest models could consider n agents, where each
                    agent could interact with each other agent. One example of such a problem would be to
                    analyze a selfish routing model, such as the one described in Section 29.3, except with
                    n agents, as opposed to infinitely many, and show a strongly polynomial time bound
                    for their convergence. After baseline models such as this have been developed and
                    studied, one might then try to find dynamics that result in these agents converging to an
                    equilibrium that maximizes an appropriate notion of social welfare. Another extension
                    would be to consider models where agents are arranged in a graph and can only interact
                    with agents in their local neighborhood. One could then analyze not only the effect of
                    the graph topology on equilibrium, as was done in Section 29.4, but also how it affects
                    the convergence time.
                       It may turn out that hardness results stand in the way of such progress. Then one
                    could try to bound the time of convergence to an approximate equilibrium, or simply
                    bound the amount of time the population spends far away from an equilibrium. Also
                    results such as the one given in Section 29.2 imply that there exist games for which it is
                    hard to compute equilibria. There still could be many well-motivated classes of games
                    for which arriving at an equilibrium is computationally tractable.


                                                           29.6 Notes

                    The motivation for evolutionary game theory and the description of the model, defini-
                    tions, and dynamics were inspired by Smith (1982), Osborne and Rubinstein (1994),
                    Weibull (1995), Hofbauer and Sigmund (1998), Kontogiannis and Spirakis (2005),
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 5, 2007   16:22




                    734                computational evolutionary game theory

                    and Kearns and Suri (2006). The Hawks and Doves game and its motivation come
                    from Smith (1982), Osborne and Rubinstein (1994), Weibull (1995), and Alexander
                    (2003).
                       The section on the computational complexity of ESS comes from Nisan (2006),
                    which extended work by Etessami and Lochbihler (2004). Lemma 29.3 is a slight
                    modification of a lemma in Motzkin and Straus (1965). Papadimitriou and Yannakakis
                    (1982) show the problem of determining whether or not a graph has a maximum clique
                    of size k is coDp -hard. We will not define the complexity class coDp here, but simply
                    state that it contains both NP and coNP. Etessami and Lochbihler (2004) show that
                    finding a strategy that is close in p norm to and ESS takes super-polynomial time
                    unless P=NP. They also show that finding an ESS is in 2P , and that finding a regular
                    ESS is NP-complete. In addition, they prove that counting the number of ESS and
                    counting the number of regular ESS are both #P-hard.
                       Most of Section 29.3 comes from Fischer and Vöcking (2004) and Fischer (2005).
                    For more details regarding the convergence of the imitative dynamics to a Nash flow,
                    see those two references. We refer the reader to Brauer and Nohel (1969) for an
                    excellent introduction into the Lyapunov framework. For a more extensive and technical
                    treatment see Bhatia and Szegö (1970). For applications of the Lyapunov framework
                    to other evolutionary game theoretic models and dynamics, see Weibull (1995) and
                    Hofbauer and Sigmund (1998). There are many other places where evolutionary game
                    theory is studied in conjunction with imitative dynamics, for example see Björnerstedt
                    and Schlag (1996) and Schlag (1998) and chapter 4 of Weibull (1995).
                       There is a nice sequence of papers that continues the work of Fischer and Vöcking
                    (2004) shown in Section 29.3. Fischer and Vöcking (2005) consider a similar model
                    where agents may have stale information regarding the latencies of other paths.
                    Fischer et al. (2006) consider a model where agents switch paths in a round based
                    fashion.
                       Section 29.4 comes from Kearns and Suri (2006) . Vickery (1987) first noticed that
                    a constant number of mutants may have higher fitness than the incumbents who are
                    playing an ESS. Theorem 29.18 is Theorem 2.15 from Bollobás (2001) . In Kearns and
                    Suri (2006), the authors give a pair of results dual to Theorem 29.16 and its converse.
                    They show that if the graph is chosen adversarially, subject to some density restrictions,
                    and the mutants are chosen randomly then ESS are preserved.

                                                        Acknowledgments

                    The author gives many thanks to Michael Kearns, Simon Fischer, Berthold Vöcking,
                    Larry Samuelson, Huanlei Ni, and Eyal Even-Dar for very helpful comments on an
                    earlier draft of this chapter.



                                                            Bibliography
                    J. M. Alexander. Probability and evolutionary game theory. London School of Economics and Political
                       Science, July 2003.
                    N.P. Bhatia and G.P. Szegö. Stability Theory of Dynamical Systems. Springer-Verlag, 1970.
P1: SBT
9780521872829main       CUNY1061-Nisan         0 521 87282 0     July 5, 2007     16:22




                                                                  exercises                                                735

                    J. Björnerstedt and K.H. Schlag. On the evolution of imitative behavior. Discussion Paper B-378,
                       University of Bonn, 1996.
                    B. Bollobás. Random Graphs. Cambridge University Press, 2001.
                    F. Brauer and J.A. Nohel. The Qualitative Theory of Ordinary Differential Equations. W.A. Benjamin,
                       Inc., 1969.
                    K. Etessami and A. Lochbihler. The computational complexity of evolutionarily stable strategies.
                       Technical Report TR04-055, Electronic Colloquium on Computational Complexity, 2004.
                    S. Fischer. Evolutionary game theory. Informatik 1, RWTH Aachen University, July 2005.
                    S. Fischer, H. Räcke, and B. Vöcking. Fast convergence to Wardrop equilibria by adaptive sampling
                       methods. In Proc. 38th Symp. Theory of Computing, pp. 653–662, 2006.
                    S. Fischer and B. Vöcking. On the evolution of selfish routing. In Proc. 12th Annual Euro. Symp. on
                       Algorithms, pp. 323–334, 2004.
                    S. Fischer and B. Vöcking. Adaptive routing with stale information. In Proc. 24th Annual ACM
                       SIGACT-SIGOPS Symp. Princ. of Distributed Comput., pp. 276–283, 2005.
                    J. Hofbauer and K. Sigmund. Evolutionary Games and Population Dynamics. Cambridge University
                       Press, 1998.
                    M. Kearns and S. Suri. Networks preserving evolutionary equilibria and the power of randomization.
                       In Proc. 7th ACM Conf. on Electronic Commerce, 2006.
                    S. Kontogiannis and P. Spirakis. Evolutionary games: An algorithmic view. In O. Babaoglu, M.
                       Jelasity, A. Montresor, C. Fetzer, S. Leonardi, A. van Moorsel, and M. van Steen, eds., Self-star
                       Properties in Complex Information Systems: Conceptual and Practical Foundations, pp. 97–111.
                       Springer-Verlag, 2005.
                    T.S. Motzkin and E.G. Straus. Maxima for graphs and a new proof of a theorem of Turan. Can. J.
                       Math., 17:533–540, 1965.
                    N. Nisan. A note on the computational hardness of evolutionary stable strategies. Technical Report
                       TR06-076, Electronic Colloquium on Computational Complexity, 2006.
                    M.J. Osborne and A. Rubinstein. A Course in Game Theory. The MIT Press, 1994.
                    C.H. Papadimitriou and M. Yannakakis. The complexity of facets (and some facets of complexity).
                       In Proc. 14th Symp. Theory of Computing, pp. 255–260, 1982.
                    K.H. Schlag. Why imitate and if so, how? J. Econ. Theory, 78:130–156, 1998.
                    J.M. Smith. Evolution and the Theory of Games. Cambridge University Press, 1982.
                    W.L. Vickery. How to cheat against a simple mixed strategy ESS. J. Theor. Biol., 127:133–139,
                       1987.
                    J.W. Weibull. Evolutionary Game Theory. The MIT Press, 1995.




                                                                   Exercises
                    29.1 Find the ESS of Prisoners Dilemma.
                    29.2 In the game of Hawks and Doves, given by Figure 29.1, if V ≤ C, show that V /C
                         is a mixed strategy ESS. (Hint: Use the fact that for any mixed Nash equilibrium, s ∗
                         with support s1 , s2 , . . ., sk , F (s1 |s ∗ ) = F (s2 |s ∗ ) = · · · = F (sk |s ∗ ) = F (s ∗ |s ∗ )).
                    29.3 Consider a 2 × 2-symmetric game with four arbitrary constants for payoffs. Char-
                         acterize the ESS for such a game in terms of the payoffs. Use this to conclude that
                         any 2 × 2-symmetric game has an ESS.
                    29.4 Give an example of a game that has a Nash Equilibrium but no ESS.
                    29.5 Prove Lemma 29.7.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   16:22




                    736               computational evolutionary game theory
                                      
                    29.6 Show that p∈P x p = 0, where x p is defined by Equation 29.2. Using this, conclude
                         that if, in the selfish routing model of Section 29.3, the imitative dynamics initially
                         start with a feasible flow, then for all time the flow remains feasible.
                    29.7 Show that there exists a game such that with high probability for a family of random
                         graphs with p = (1/nc) and 0 ≤ c < 1, an adversary can construct a mutant set
                         such that there will exist at least one mutant with higher fitness than all of its
                         incumbent neighbors.
P1: SBT
9780521872829main       CUNY1061-Nisan      0 521 87282 0    July 17, 2007     18:55




                                                             Index




                    AAE example, 466–467, 476                              single-dimensional domains, 303–310
                    aborting games, 188, 190                               submodularity, 623–624
                    adaptive behavior, 81                                  theorems, 305, 307, 309, 315, 318, 324
                    adaptive limited-supply auction, 424–427            Arrow–Debreu market model, 103, 104,
                    adoption as coordination problem, 636                     121–122, 136
                    adverse selection, 677                              Arrow’s theorem, 212–213, 239
                    advertisements. See sponsored search auctions       artificial equilibrium, 61
                    affiliate search engines, 712                       ascending auctions, 289–294
                    affine maximizer, 228, 317, 320                     ascending price auction, 126
                    affinely independent, 57                            assortative assignment, 704
                    agents. See players                                 asymmetries in information security, 636–639
                    aggregation of preferences. See mechanism           atomic bids, 280, 282
                          design                                        atomic selfish routing, 461, 465–468, 470–472,
                    aggregation problem, 651–655                              475–477, 482–483
                    algorithmic mechanism design. See also              atomic splittable model, 483
                          mechanism design; distributed algorithmic     attribute auction, 344
                          mechanism design                              auctions
                    allocation in combinatorial auction, 268,              adaptive, limited-supply, 424–427
                          270–272                                          ascending, 289–294
                    AMD. See algorithmic mechanism design                  bidding languages, 279–283
                    “AND” technology, 603–606                              call market, 654–655
                    announcement strategies, 685–686                       combinatorial. See combinatorial auctions
                    anonymous games, 40                                    competitive framework, 344–345
                    anonymous rules, 247, 250                              convergence rates, 342–344
                    approximate core, 389–391                              deterministic optimal price, 340
                    approximate equilibria, 45, 138, 143, 167              digital goods, 332, 338, 340, 345–346
                    ApproximateTreeNash, 166–168, 176                      dynamic, with expiring items, 412, 420–424
                    approximation mechanisms, computationally              examples in mechanism design, 209–210,
                          efficient                                           220–221
                       alternative solution concepts, 321–327              first price (Bayesian analysis), 20, 234–236
                       dominant strategy, impossibilities of,              frugality, 350–354
                          317–320                                          iterative, 283–287
                       history, 327                                        known single-minded combinatorial, 418
                       multidimensional domains, 310–317                   lower bounds, 346–347
                       overview, 301–303                                   profit maximization, 331–332, 336

                                                                      737
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 17, 2007   18:55




                    738                                        index

                    auctions (cont.)                                    single-value, 322
                      random sampling optimal price (RSOP)              sponsored search auctions. See sponsored
                         auction, 341–342                                  search auctions
                      random sampling profit extraction, 348–349     bidding languages, 279–283, 295, 310
                      single-item, 332, 337                          bilateral network formation game, 507
                      sponsored search auctions. See sponsored       bilateral trade, 220–221
                         search auctions                             bimatrix game, 30, 54–57, 62, 152
                      symmetric, 340                                 binding inequality, 57–59
                      truthful combinatorial, 316–317                BitTorrent, 570, 589, 596, 600–601
                      Vickrey auction. See Vickrey auction           blocking coalition, 253–255
                    automated market makers, 662–665, 670            blocking pair, 255, 256, 507
                    autonomous systems (ASes), 364–365,              blogs, 622, 627, 630
                         370–371, 373–379, 487, 507                  BNIC. See Bayes-Nash incentive-compatible
                    axiomatic method, 404                            Bondareva–Shapley theorem, 388, 389, 391,
                                                                           407
                    backward induction, 69                           Boolean circuit, 41, 43
                    balanced flow, 111–116, 119                      Boolean events, 658, 661
                    balls into bins problem, 451–452, 530            Boolean market model, 666, 668
                    bandwidth-sharing game, 6–7, 452–455, 587,       bootstrapping problems, 636, 647, 689
                         588                                         Borda count, 211
                    banking and security, 634, 647                   Border Gateway Protocol (BGP), 372, 374, 376,
                    barter-based system, 600–601                           378–379, 381
                    basis matrix, 65                                 bounded communication, 356
                    battle of the sexes game, 7, 12                  Braess’s Paradox, 464–465, 475, 481
                    Bayes’ rule, 667                                 Brandes’ algorithm, 645
                    Bayesian first price auction, 20                 brittle and nonbrittle comparators, 43
                    Bayesian-Nash implementation, 233–237, 416,      broadcast and secure channels, 185, 201
                         431–436                                     Brouwer’s fixpoint theorem, 32, 41–43
                    Bayesian network structured market, 662          budget balanced, 22, 392, 393, 501
                    Bayesian optimal mechanism design, 333,          budget constraints. See sponsored search
                         335–338, 357                                      auctions
                    behavior strategy, 67                            bundle-price ascending auctions, 292–295
                      sequence form, 71                              bundles of items. See combinatorial auctions
                    best response                                    bundling, 356
                      in graphical games, 162
                      and learning in games, 18                      call market auction, 654–655
                      max-weight best response policy, 524           capacity augmentation, 479–480
                      and Nash equilibrium, 30–31, 54, 497           capacity investments, 590
                      in peer-to-peer networks, 605                  Cascade Model, 620–621, 624–625
                      polyhedron, 57–59                              cascading behavior in networks
                      for identical machines, 522–524                  contagion threshold, 615–616
                      in reputation systems, 686                       finding influential sets of nodes, 622–627
                      in strict incomplete information games,          general social contagion, 618–622
                         223                                           history, 630–631
                    best response polyhedron, 57                       networked coordination games, 614–618
                    BGP. See Border Gateway Protocol (BGP)             online data empirical studies, 627–630
                    bid format and price formation, 666–667            overview, 613–614
                    bid vector, 453–454                                theorems, 617, 618, 624–626
                    bidders                                          CE. See correlated equilibrium
                      bidding languages, 279–283                     cell structure, 644–645
                      in combinatorial auctions, 267–268             censorship resistance, 640–643
                      exposure problem, 292                          centrality attacks, 645
                      iterative auctions (query model), 283–287      CEPE auction. See consensus estimate profit
                      single-minded, 270–275, 295, 323–324, 332            extraction (CEPE) auction
P1: SBT
9780521872829main       CUNY1061-Nisan       0 521 87282 0     July 17, 2007    18:55




                                                                   index                                            739

                    CES. See constant elasticity of substitution           oligopoly pricing and equilibrium, 582–583
                          (CES)                                            overview, 571–572
                    cheap pseudonyms, 597, 679, 683                        pricing and efficiency with congestion
                    “cheap talk” preamble phase, 188                          externalities, 579–582
                    Chernoff bound, 532, 533–535                           pricing and resource allocation theoretic
                    chicken game, 45–46                                       models, 578–579, 584–587
                    churn, 594                                             theorems, 584, 585
                    Clarke pivot rule, 219–221, 561                      compact prediction markets, 661–662
                    clearing prices. See market clearing prices          competitive analysis, 344–345, 351, 352–354,
                    click through rate (CTR), 701–704, 707, 712               413, 417, 421
                    clique strategy, 644–646, 721–722                    competitive auctions, 345–349, 355
                    coalition game. See cooperative game theory          competitive digital goods auction, 345–346
                    coalition-proof equilibrium, 192                     competitive equilibrium
                    coalitions of agents, 250, See also collusions         definition, 292
                    coarsest common refinement, 653                        large communications networks, 572–578
                    Cobb-Douglas functions, 139, 143, 146, 155             price takers, 546–547
                    collective utility function, 405                       smooth market-clearing mechanism, 552
                    collusion-proof ex-post Nash equilibria, 376           social welfare, 293
                    collusions, 189, 191, 199, 356, 597                  competitive ratio, 345–348, 354, 357, 358, 422,
                    combinatorial auctions                                    425
                       alternative solution concepts, 321–327            complementary slackness, 74
                       applications of, 269–270                          complementary slackness conditions, 104, 109
                       ascending auctions, 289–294                       complements vs. substitutes, 268, 290, 292
                       bidding languages, 279–283                        complete information models, 239
                       communication complexity, 287–289                 completely labeled, 58, 59, 61–63, 66
                       computationally efficient mechanisms. See         complex networks and topology, 643–646
                          approximation mechanisms                       compound prediction markets, 659–661
                       definitions and problem, 267–269                  computational aspects of prediction markets.
                       history, 295–296                                       See prediction markets
                       iterative auctions (query model), 283–287         computational evolutionary game theory
                       linear programming relaxation, 275–277              classical evolutionary model, 718–720
                       multidimensional domains, 310–317                   computational complexity of evolutionarily
                       single-minded case, 270–275, 332, 418                  stable strategies, 720–723
                       theorems, 273, 277, 278, 282, 285, 288, 289,        evolutionary dynamics applied to selfish
                          291, 294                                            routing, 723–728
                       truthful, 316–317                                   future research, 733
                       Walrasian equilibrium, 277–279                      graphs, 728–733
                    combinatorial prediction markets, 657–662,             history, 733–734
                          670                                              overview, 717–718
                    combined value trading, 658, 672                       theorems, 719, 723, 727, 731
                    combining expert advice. See external regret         computational indistinguishability, 185
                       Internet routing, 376–379                         computational manipulation example, 366–367
                    commitment types, 682                                computationally efficient mechanisms. See
                    common value model, 238                                   approximation mechanisms
                    communication complexity in combinatorial            computer science and game theory, 363–364
                          auctions, 287–289, 295                         computer science vs. economics, 301–303
                    communications networks                              concave games. See submodular games
                       alternative pricing and incentive approaches,     conditional equilibrium, 164, 176
                          587–590                                        conditional securities, 659
                       efficiency analysis, 583–584                      Condorcet’s paradox, 211
                       future research, 589–590                          congestion control algorithm, 576–577
                       large networks (competitive models),              congestion games, 41, 463, 482, 497–498,
                          572–578                                             579–582
                       monopoly pricing and equilibrium, 582             consensus, 349–350
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 17, 2007   18:55




                    740                                          index

                    consensus estimates, 356                              Markov networks, 170–174
                    consensus estimate profit extraction (CEPE)           mediators, removing, 192–195
                          auction, 350                                    vs. Nash equilibria, 47–48
                    constant elasticity of substitution (CES), 139,       overview, 14–16, 45–47
                          149–151, 155                                    regret minimization, 88–92
                    constant sum games, 89–90                             in succinct games, 48–49
                    constraint satisfaction programming (CSP), 169        and swap regret minimization, 90–91
                    consumer demand and constant elasticity of         cost benchmark, 352
                          substitution, 149–150                        cost function, 462, 663–665
                    consumer sovereignty (CS), 392                     cost matrix, 4, 5, 8
                    consumer surplus, 580                              cost-sharing
                    contact process, 630                                  and cooperative games, 385–387
                    contagion threshold, 615–618, 620                     core, 387–391
                    contingent claims/contracts. See prediction           facility location game, 397–402
                          markets                                         and fair division, 21–22, 347
                    continuous double market, 654, 662, 666–667           games, 501
                    convergence, 342–344, 373, 523–524, 669; see          group-strategyproof mechanisms and
                          also learning                                      cross-monotonic schemes, 391–394
                      rates, 342–344, 523–524                             history, 406–408
                      times, 669                                          limitations of cross-monotonic schemes,
                    convex program, 104, 105–109, 112                        400–402
                    convex programming for market equilibria              mechanism, 392
                      approximate equilibrium, 138                        multicast transmission mechanism, 367–370
                      definitions, 136–137                                overview, 405–406
                      equilibrium vs. optimization, 139–140               primal-dual schema, 394–400
                      exchange economies and weak gross                   Shapley value and Nash bargaining solution,
                          sustainability, 142–148                            402–405
                      Fisher model with homogeneous consumers,            submodular game, 395–397
                          141–142                                         theorems, 388, 389, 391, 394, 396, 398, 401,
                      gross substitutability, 138                            404, 405
                      limitations, 150–152                             costs. See also prices
                      models with production, 152–155                     censorship, 642–643
                      overview, 135–136, 155–156                          defense vs. attack, 644
                      tâtonnement process, 137–138, 144, 147             defining, 9
                      utility function special forms, 139, 148–150        function, 9–10
                    cooperative game theory                            Credence system, 597
                      barter-based system, 600–601                     critical payment, 274, 419, 430–431
                      and cost sharing, 21–22, 385–387                 critical values, 229
                      graphical games, 177                             cross-monotonic cost-sharing schemes,
                      overview, 20–21                                        391–394, 396–397, 400–402
                      in peer-to-peer networks, 588–589, 593, 596      cryptography
                      reputation as incentive, 596–600                    game theory influences on, 197–202
                      strong Nash equilibrium, 21                         game theory notions and settings, 187–189
                      in wireless networks, 589                           history, 203–204
                    coordination game, 7–8, 614–618                       influence on game theory, 191–197
                    coordination ratio. See price of anarchy              multiparty computation, 181–182, 185–187
                    core, 22, 387–391, 402                                multiparty computation vs. games, 189–191
                    correctness and privacy properties, 184,              overview, 202
                          194–195, 197                                    security of multiparty computation, 182–185
                    correlated equilibrium                             CS. See consumer sovereignty
                      approximating, 48                                currency-based p2p systems, 594, 601–602
                      definition, 46, 47, 90
                      ex ante, 196                                     DAMD. See distributed algorithmic mechanism
                      in graphical games, 161–163, 169–175                design
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 17, 2007   18:55




                                                                 index                                           741

                    decision making in uncertain environment,          distributed reputation systems, 693
                          79–81. See also regret analysis              distributed shortest-path routing, 481
                    decision policy, 414                               divisible matching problem, 660–661
                    decomposition-based mechanism, 312–314             divisible vs. indivisible orders, 659
                    deferred acceptance algorithm, 256–258             dominant strategies, 10–12, 91–92, 222–225,
                    degenerate games, 56, 65–66                              317–320
                    delegation defense, 646                            dominant strategy incentive-compatible (DSIC),
                    demand bundle, 284, 292–294                              415, 428, 430, 436
                    denial of service attacks, 634                     dominated strategy, 60
                    derandomization, 355                               DOP auction. See deterministic optimal price
                    design metric and inefficiency of equilibria,            auction
                          454–456                                      double marginalization problem, 586
                    design of scalable resource allocation             DSIC. See dominant strategy
                          mechanisms. See scalable resource                  incentive-compatible
                          allocation mechanisms                        dual growth process, 109–110
                    deterministic algorithm, 308–309                   duopoly pricing, 580
                    deterministic optimal price auction, 340           dynamic aspects of sponsored search auctions,
                    dictatorship, 214, 247                                   707–711
                    diffusion of innovations, 613–614, 622,            dynamic attacks in reputation systems, 694
                          627–630                                      dynamic environments and online mechanism
                    digital goods auctions                                   design, 413–417
                       competitive model, 345–346                      dynamic parimutuel markets, 664–665
                       consensus estimation and truthfulness with      dynamic VCG mechanism, 433–434
                          high probability, 349–350                    dynamics of regret minimization, 99
                       convergence rates, 342–344
                       decision problem, 347                           early-arrival misreports, 415, 430
                       definition, 332                                 early stopping, 190
                       theorems, 340                                   economics vs. computer science, 301–303
                       and virtual surplus, 338                        effective bandwidth pricing, 587
                    diminishing returns, 621, 624–626, 628             efficiency in sponsored search auctions,
                    direct reciprocity, 594                                  703–705
                    direct-revelation online mechanisms, 414–416       efficient market hypothesis, 657, 672
                    disagreement outcome, 404–405                      egalitarian function, 443
                    discrete tâtonnement process, 144, 147            Eigentrust algorithm, 597
                    dispute wheel, 373–374, 378–380                    Eisenberg–Gale program
                    distance-vector, 371                                  combinatorial algorithms, 104
                    distributed algorithmic mechanism design              convex, 105–108, 155
                          (DAMD)                                          Nash bargaining solution, 402
                       vs. algorithmic mechanism design, 365, 380         primal-dual schema, 109
                       combining networking and mechanism              elastic traffic, 584–585
                          design perspectives, 376–379                 elasticity of substitution, 139
                       history, 380–381                                elections and mechanism design, 209,
                       interdomain routing, 374–376                          211–212
                       multicast transmission cost-sharing, 367–370    electronic market design, 210
                       networking perspective, interdomain routing,    Ellipsoid method, 156
                          371–374                                      empirical distribution, 339–341
                       open problems, 380                              empirical Myerson mechanism, 339–341
                       overview, 363–365, 379–380                      empty threats, 195–196, 201
                       theorems, 369, 370, 378                         envy-freedom, 355, 712
                       of Vickrey–Clarke–Groves mechanisms,            epidemic. See cascading behavior in networks
                          366–367                                      equilibria
                    distributed computation through markets,              approximate, 45
                          665–669, 670–671                                artificial, 61
                    distributed mechanism, 375                            atomic flow, 466
P1: SBT
9780521872829main      CUNY1061-Nisan        0 521 87282 0    July 17, 2007    18:55




                    742                                               index

                    equilibria (cont.)                                     and partial information model, 94–96
                       Bayesian-Nash, 235                                externality, 273, 579
                       complexity of finding, 16, 29–31
                       computational, 191                                facet, 57
                       correlated, 14–16, 45–49                          facility location game
                       equilibrium price, 23, 25, 108–109, 135              and cost sharing, 386–387, 389–390
                       finding. See finding equilibria                      and network formation games, 502–506
                       graphical games. See graphical games                 open problems, 510–511
                       for identical machines, 522–524, 529–533             primal-dual scheme and cross-monotonicity,
                       inefficiency. See inefficiency of equilibria            397–402
                       via labeled polytopes, 57–60                         Shapley values and, 403
                       of markets. See market equilibria                 fair division, 21–22
                       Nash. See Nash equilibrium                        Fair, Optimal eXchange (FOX) protocol, 601
                       nonatomic flow, 463                               fair sharing rule, 489
                       noncooperatively computable, 198                  fairness, 184, 194, 355, 501, 572, 581, 584, 639
                       vs. optimization, 139–140                         faulty parties, 182–184, 186
                       price characterization, 667–669                   FCC auctions, 269
                       reduced strategic form, 69–70                     feedback in reputation systems, 683–689
                       regret analysis. See regret analysis              file-sharing game, 594–596, 640
                       sequence form, 73–74                              finding equilibria
                       in sponsored search auctions, 705–707                PPAD, 36–39
                       subgame perfect, 19–20, 68–69                        complexity, 16
                       for uniformly related machines, 524–529,             correlated equilibrium, 45–49
                          533–537                                           Lemke–Howson algorithm, 33–36
                       Wardrop. See Wardrop equilibria                      NP-completeness and Nash equilibrium,
                    Euler’s identity, 142                                      31–33
                    evolutionarily stable strategy (ESS), 718–723,          overview, 29–31, 49–50
                          729–731, 734                                      reduction to Brouwer, 41–45
                    evolutionary game theory. See computational             succinct game representations, 39–41
                          evolutionary game theory                       first price auction (Bayesian analysis), 234–236,
                    ex ante correlated equilibrium, 196                        335
                    ex-post incentive compatible. See incentive          first welfare theorem, 103, 277
                          compatible mechanisms                          Fisher’s model
                    ex-post Nash equilibrium, 375–376, 377–379              Arrow–Debreu model and, 121–122
                    ExactTreeNash, 168, 177                                 concave utilities, 131
                    exchange economy, 136, 142–148, 566                     exchange model with proportional
                    exchange model. See Arrow–Debreu model                     endowments, 140
                    exclusivity, 197                                        with homogeneous consumers, 141–142
                    existence and uniqueness in atomic selfish              linear case, 104, 105–108, 121, 131
                          routing, 470–472                                  linear utilities, 121–122, 131
                    existence and uniqueness in nonatomic selfish        fitness function, 718–719, 729–732
                          routing, 468–470                               fixed pricing, 588
                    expected capacity pricing, 587                       fixpoint. See Brouwer’s fixpoint theorem
                    exporting routes in BGP, 372–373                     flat fees, 588
                    exposure problem, 292                                flow, 462, 463, 465, 468–470, 723
                    expressiveness vs. simplicity in language, 279       forecast, 653–654. See also prediction markets
                    extensive game, 40, 54, 66–68, 188–189,              formation games and network design. See
                          195–197                                              network formation games
                    external regret                                      FPTAS. See fully polynomial time
                       in constant sum games, 89–90                            approximation schemes
                       generic reduction to swap regret, 92–94           fractional allocations
                       minimization of, 82–88                               algorithm, 306–307
                       model, 81–82                                         domain, 311
                       overview, 80–81                                      load function, 307
P1: SBT
9780521872829main       CUNY1061-Nisan       0 521 87282 0    July 17, 2007   18:55




                                                                     index                                       743

                       optimum, 314–315                                 general equilibrium theory, 22–23, 103
                    free-market environment, 597–598                    General Threshold Model, 619–620, 626
                    free-riding, 595, 597, 599, 601, 608, 637, 647      generalized first price (GFP) auctions, 702,
                    frugality, 350–354                                        704–705
                    full information model, 81                          generalized median voter scheme (g.m.v.s.),
                    fully mixed equilibria, 529–533                           250, 251
                    fully mixed Nash equilibrium conjecture, 531        generalized second price (GSP) auctions, 702,
                    fully polynomial time approximation schemes               704–706
                          (FPTAS), 607                                  generalized-WMON, 318–319
                                                                        Gibbard–Satterthwaite theorem, 213–215, 243,
                    gadgets, 42–43                                            244
                    game theory                                         Gittins’ index policy, 435
                      computational evolutionary. See                   global connection game, 488–489, 494–498,
                         computational evolutionary game theory               500–502, 509–510
                      and computer science, 363–364                     global trust values, 597
                      cryptography, influences on, 197–202              goods. See market equilibria
                      efficiency, 191                                   government policy and mechanism design, 210,
                      and information security, 635–636                       221
                      vs. issues in cryptography, 189                   graphical exchange economies, 176–177, 178
                      and multiparty computation. See multiparty        graphical games
                         computation (MPC)                                 complexity of finding Nash equilibrium, 40
                      notions and settings, 187–189                        computational benefits, 160
                      and regret minimization, 88–92                       correlated equilibrium, 161–163, 169–175
                    game tree, 54, 68, 70, 72–74                           definitions, 161–163
                    games. See also specific game names and types          future research and open problems, 177
                      aborting, 188, 190                                   interdisciplinary benefits, 160
                      battle of the sexes, 7–12                            Markov networks, 170–174
                      Bayesian, 20                                         Nash equilibrium in, 160–161
                      best response and learning in, 18, 30–31             Nash equilibrium in tree graphical games,
                      compactly represented, 9–10                             164–169
                      cooperative, 20–22                                   overview, 159–161, 177–178
                      cooperative and cost sharing, 385–387                structural benefits, 160
                      coordination, 7–8, 614–620                        greedy algorithms, 83–84, 315, 522
                      cost sharing, 501                                 greedy auctions, 273–274, 422, 709
                      definition, 3, 88                                 Green-Laffont, 368
                      graphical. See graphical games                    grim-trigger strategy, 601, 681, 683
                      ISP routing, 4–5                                  gross substitutability, 138, 145
                      matching pennies, 8–9                             group-strategyproof mechanisms, 391–394
                      pricing, 14, 502                                  GS. See gross substitutability
                      prisoners’ dilemma, 3–6, 443–444, 446–447,        GSP auctions. See generalized second price
                         595, 680, 681                                        (GSP) auctions
                      repeated and online, 356
                      routing. See routing games                        ham sandwich problem, 38
                      routing congestion, 7–8, 96–99                    Hawks and Doves game, 719–720, 734
                      simultaneous move, 9                              hidden actions, 239, 594, 602–609, 636–638,
                      standard form, 9–10                                     648
                      succinct representations of, 39–41                hill-climbing, 623–624, 630
                      tragedy of the commons, 6–7, 595                  hiring-a-team auctions, 351
                      transferable utility, 21, 385–386, 387–391        hiring, secretary problem, 424–425, 427
                      two-person zero-sum, 16–18, 73                    honest-but-curious parties, 182, 186, 197
                      ultimatum, 19–20                                  honest parties, 182, 183
                      with turns, 18–20                                 hot potato routing, 602
                    Gao–Rexford conditions, 376–380                     house allocation problem, 253–255, 262, 263
P1: SBT
9780521872829main      CUNY1061-Nisan        0 521 87282 0    July 17, 2007    18:55




                    744                                               index

                    IC. See incentive compatible mechanisms                 in network formation games. See network
                    idea futures. See prediction markets                       formation games
                    ideal model, 183                                        overview, 443–444
                    identity,682. See also reputation systems               price of anarchy, 445
                    IDoWDS, 200–202                                         price of stability, 446
                    imitative dynamics of selfish routing model,            in resource allocation. See scalable resource
                          723–726, 734                                         allocation mechanisms
                    importing routes in BGP, 372                            in routing games. See routing games
                    improvement step, 519–520, 522–524, 528                 in selfish load balancing. See selfish load
                    incentive compatible differentiated pricing,               balancing
                          589–590                                        inequalities
                    incentive compatible mechanisms                         binding, 57–59
                       approximation in ascending auctions, 286             characterizing equilibrium, 154
                       characterizations of, 225–226                        correlated equilibrium, 46
                       direct characterization, 226                         irredundant, 57
                       interdomain routing, 375                             Jensen’s, 727
                       mechanisms with money, 217–218                    infinite time horizon and discounting, 434
                       price uniqueness, 230–231                         influential sets of nodes, 622–627, 630
                       randomized mechanisms, 231–233                    information aggregation problem, 651–655
                       scalable resource allocation mechanisms,          information cascades, 684
                          560                                            information markets. See prediction markets
                       single-minded bidders, approximation,             information-measuring software security, 638
                          272–275                                        information security
                       single-parameter domains, 228–230                    censorship resistance economics, 640–643
                       social choice, 214, 215                              complex networks and topology, 643–646
                       weak monotonicity, 226–227                           informational asymmetries, 636–639
                       weighted Vickrey–Clarke–Groves                       insurance-based approaches to information
                          mechanisms, 227–228                                  security, 639
                    incentives and information security. See                misaligned incentives, 634–636
                          information security                              overview, 633–634, 646–647
                    incentives for honest reporting, 690                    in reputation systems, 678
                    incentives in communication networks. See            information set, 54, 67
                          communications networks                        initiation fee, 682
                    incentives in peer-to-peer networks. See             integer pivoting, 63–65
                          peer-to-peer networks (P2P)                    integrality gap, 314–316
                    incomplete information games, 187–188,               interdependent values, 238–239
                          222–223, 647                                   interdomain routing
                    incremental cost-sharing, 403                           combining networking and mechanism
                    incremental function, 620, 621, 624–626                    design perspectives, 376–379
                    incumbents, 717, 718, 720, 729–732                      introduction, 370–371
                    Independent Cascade Model, 621, 625                     mechanism design perspective, 374–376
                    independent private values, 222–223                     networking perspective, 371–374
                    indirect reciprocity, 594, 596                       internal regret. See swap regret
                    individual rationality (IR), 219, 252, 333,419;      Internet Service Providers (ISPs), 4–5, 587,
                          see also voluntary participation                     602
                    indivisible matching problem, 659–660                invisible hand, 217
                    indivisible order matching, 660, 661                 Iowa Electronic Market (IEM), 655, 671
                    inefficiency of equilibria                           irrelevant information sets, 70–72
                      communications networks. See                       IR. See individual rationality
                          communications networks                        item-price ascending auctions, 290–292, 295
                       as a design metric, 454–456                       iterated deletion of weakly dominated strategies
                       examples, 446–452                                       (IDoWDS), 200–202
                       history, 456–457                                  iterative auctions (query model), 283–287
                       measures of, 444–445                              iterative wrapper, 322
P1: SBT
9780521872829main       CUNY1061-Nisan        0 521 87282 0     July 17, 2007    18:55




                                                                    index                                              745

                    Jensen’s inequality, 727                              LiveJournal, 627–630
                    job scheduling problem, 302–310                       load balancing games
                    joint deviation. See coalitions of agents                defining price of anarchy, 521–522
                    joint forecast, 653                                      example, 520–521
                                                                             history, 538–540
                    K-rank-sybilproof, 691–692                               introduction to, 518–520
                    k-resiliency, 191–194, 200                               mixed equilibria on identical machines,
                    Karush-Kuhn-Tucker (KKT) conditions, 104,                   529–533
                         106, 107, 109–110, 125, 128, 140, 141,              mixed equilibria on uniformly related
                         573, 575                                               machines, 533–537
                    Kelly’s model, 104–105, 124–125, 402                     overview, 517–518, 537–538
                    keyword auctions. See sponsored search                   price of anarchy, 521–522
                         auctions                                            pure equilibria for identical machines,
                    kidney matching model, 262, 263                             522–524
                    KKT conditions. See Karush-Kuhn-Tucker                   pure equilibria for uniformly related
                         (KKT) conditions                                       machines, 524–529
                    known interesting-set assumption, 429–430             local connection game, 489–494, 506–509
                    known single-minded combinatorial auction,            local effect games, 41
                         332                                              local game matrices, 162
                    known single-minded (KSM) players, 323–324,           local neighborhood equivalence, 170–171
                         418                                              local-to-global link, 624, 626
                    KP model. See load balancing games                    locally envy-free, 705–707
                    Kuhn’s theorem, 71                                    locally optimal solutions, 378
                                                                          logarithmic scoring rule, 686, 687
                    labels, 57–60                                         loser-if-silent, 325
                    labeled polytopes and equilibria, 57–60               low communication, 544, 551–552
                    Lagrangian function and multipliers, 173, 547,        low-dimensional strategies, 544, 551–552,
                          556, 573–575, 578                                     564
                    large actions spaces and regret minimization, 98      lower bounds, 287–289, 346–347, 421
                    largest processing time (LPT) algorithm,              LP formulation. See linear programming
                          528–529                                               relaxation
                    late-departure misreports, 415, 423, 430              Lyapunov function, 575–576, 725–726, 734
                    latency function, 96, 97, 584, 724,726; see also
                          cost function                                   MAB. See partial information model
                    lattice formulation, 259–260, 263                     makespan minimization, 305–310, 450, 452,
                    LCP. See linear complementarity problem                    517, 518, 525–530
                    leaders, 43                                           malicious parties, 182
                    learning. See also regret analysis                    manipulation-resistant reputation systems. See
                       coordinated learning, 435                               reputation systems (manipulation-resistant)
                       response and learning, 18, 30–31, 54               marginal cost (MC), 368–370, 468
                    Lemke–Howson algorithm, 33–36, 59, 61–63,             marginal cost pricing, 478–480, 588
                          391                                             marginal traders, 655
                    Lemke’s algorithm, 74                                 marginal utility, 562
                    Leontief functions, 139, 152                          market-based approaches to information
                    LH algorithm. See Lemke–Howson algorithm                   security, 638–639
                    liability, in information security, 634–636           market clearing prices
                    limited misreports, 415, 419, 420, 423, 428–430         bid format and price formation, 666
                    linear complementarity problem, 74                      definition, 23–24, 105
                    linear exchange economies, 149                          equilibrium price characterization, 668–669
                    linear programming relaxation, 260–261,                 proportional allocation mechanism, 545–546
                          275–278, 284–285, 388, 395, 406                   rational expectations equilibrium, 656
                    Linear Threshold Model, 619, 626                        in resource allocation, 555–557
                    link-state, 371, 373                                    smooth market-clearing mechanism, 552–553
                    Lipschitz continuous, 723–725                           and Walrasian equilibrium, 277
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    July 17, 2007   18:55




                    746                                         index

                    market equilibria                                   computationally efficient mechanisms. See
                     Arrow–Debreu model, 121–122                           approximation mechanisms
                     auction-based algorithm, 122–124                   definition, 209
                     balanced flows, 111–115                            direct characterization of incentive
                     combinatorial algorithms for, 103–105                 compatible mechanisms, 226
                     convex programming limitations, 150–152            distributed algorithmic. See distributed
                     convex programming models with                        algorithmic mechanism design
                         production, 152–155                            examples and applications, 209–211
                     convex programming techniques for,                 hidden actions, 239
                         135–141, 155–156                               history, 239–240
                     exchange economies and weak gross                  implementation in dominant strategies,
                         sustainability, 142–148                           222–225
                     finding tight sets, 117–118                        incentive compatible, 217–218, 225–226
                     Fisher model with homogeneous consumers,           interdependent values, 238–239
                         141–142                                        online. See online mechanism design
                     Fisher’s linear case and Eisenberg–Gale            price uniqueness, 230–231
                         convex program, 105–108                        randomized mechanisms, 231–233
                     graphical exchange economies, 176–177              risk aversion model, 238
                     and mechanism design, 209                          single-parameter domains, 228–230
                     open problems, 109                                 social choice, 211–215
                     overview, 22–23, 131                               theorems, 213, 214, 219, 227–230, 232, 236
                     prices as equilibrium prices, 108–109              Vickrey auction, 216–217
                     in resource allocation markets, 124–125            Vickrey–Clarke–Groves mechanisms,
                     simple algorithm, 23–26                               218–219
                     single-source multiple-sink markets                weak monotonicity, 226–227
                         algorithm, 126–131                             weighted Vickrey–Clarke–Groves
                     utility functions for, 148–150                        mechanisms, 227–228
                    market maker, 652, 654–655, 662–665, 670          mechanism design and profit maximization
                    market power, 454                                   Bayesian optimal mechanism design,
                    market predictions. See prediction markets             335–338
                    market scoring rules, 663–664                       examples and applications, 331–332
                    marketing. See cascading behavior in networks;      frugality, 350–354
                         sponsored search auctions                      history, 357–358
                    Markov decision process, 432, 435                   overview, 331–334
                    Markov networks, 170–174                            prior-free approximations to the optimal
                    Markov process, 93                                     mechanism, 339–344
                    matching. See stable matching problem               prior-free optimal mechanism design,
                    matching pennies game, 8–9                             344–350
                    matching problem, 659–661                           open problems, 354–357
                    matrix form, 9–10                                   theorems, 334, 336, 338, 340, 341, 343, 345,
                    matroid, 353                                           346, 348, 353
                    maximal Nash subset, 66                             truthful mechanisms, 333–334
                    maximum aggregate utility, 550–551                mechanism design without money
                    maximum flow, 112–114, 690, 692                     future research and open problems, 262
                    MC. See marginal cost (MC)                          history, 263
                    McDiarmid’s inequality, 343                         house allocation problem, 253–255
                    MDP. See Markov decision process model              lattice formulation, 259–260
                    measures of inefficiency, 444–445                   overview, 243–244
                    mechanism design                                    single-peaked preferences over policies,
                     Bayesian-Nash implementation, 233–237                 244–252
                     Clarke pivot rule, 219–220                         stable matchings, 255–262
                     combinatorial auctions. See combinatorial          theorems, 247, 251, 254, 256–258, 260, 261
                         auctions                                     median voter rule, 246
                     complete information models, 239                 mediated games, 188
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    July 17, 2007   18:55




                                                                index                                           747

                    mediators, removing in correlated equilibrium,    Myerson’s mechanism, 337–339, 341–342, 357,
                         192–195                                          435, 703
                    minimax theorem, 89–90                            myopic behavior, 667
                    misreports, 415, 419, 420, 423, 428–430
                    mixed strategy
                      bimatrix games and best response, 54            Nash bargaining solution, 404–405
                      graphical games, 162, 167                       Nash equilibrium
                      introduction to, 8–9                              aggregate utility, 550–551
                      in load balancing games, 518, 529–537             Bayesian-Nash implementation, 233–237
                      vs. pure strategies, 520–522                      and bimatrix games, 54–57, 152
                    mixed strategy Nash equilibria, 13, 450–452         is a combinatorial problem, 31
                    mobile ad hoc networks (MANETs), 602                computational, 191
                    model-free vs. model-based frameworks, 413          and correlated equilibrium, 14–15, 163
                    monopoly pricing and equilibrium, 580, 582          in degenerate games, 66
                    monotone algorithm for job scheduling,              and evolutionarily stable strategy, 719–720
                         305–310                                        finding. See finding equilibria
                    monotone hazard rate, 337                           and frugality, 352
                    monotonicity                                        in games with turns, 18–20
                      cross-, 392–393                                   games without, 13–14
                      deterministic policy, 418                         in graphical games, 160–162
                      in facility location problems, 505                inefficiency of equilibria, 446
                      in peer-to-peer networks, 606, 619, 623–624       k-resiliency, 194
                      progressive cascading behavior, 616–617           and Lemke–Howson algorithm, 33–36, 61–63
                      single-minded bidders, 274                        mixed strategy, 13, 529–533
                      weak, 226–227, 304–305, 307–309, 318–319,         in network formation games, 488
                         428                                            and NP-completeness, 31–33
                    Moulin’s theorem, 392–394, 402, 403, 407, 408       in potential games, 497, 499–500
                    MPC. See multiparty computation                     in resource allocation games, 547–549
                    multi-armed bandits problem (MAB). See              pure strategy, 12–13, 55, 519, 520, 528–529,
                         partial information model                         724
                    multicast cost-sharing, 332, 367–370                and regret minimization, 96–99
                    multicommodity flow network, 462                    selfish routing, evolutionary dynamics of,
                    multidimensional domains, 302, 310–317                 725–726
                    multiparty computation (MPC)                        in Shapley network design games, 449–450
                      cryptographic influences on game theory,          smooth market-clearing mechanism, 552–553
                         191–197                                        strong, 21
                      existing results, 185–187                         subgame perfect, 19–20, 68–69, 681–683
                      game theory influences on cryptography,           with succinct game representations, 39–41
                         197–202                                        symmetric, 30–31, 34
                      game theory notions and settings, 187–189         theorems, 13, 17, 34, 47
                      vs. games, 189–191                                in tree graphical games, 164–169
                      generalizations, 182                              in two-person zero-sum games, 16–18
                      history, 203–204                                  without full information (Bayesian games),
                      overview, 181–182, 202                               20
                      rational, 199–202                               Nashification, 529
                      security of, 182–185                            NashProp, 161, 164, 168–169, 177–178
                      theorems, 185, 193, 199                         NCC. See noncooperatively computable (NCC)
                    multipath routing, 603                            NE. See Nash equilibrium
                    multiplayer games. See also graphical games;      network complexity, 365, 367–370, 380, 381
                         specific multiplayer games                   network congestion games, 41
                      definitions, 161–163                            network formation games
                      graphical, 159–161                                and facility location, 502–506
                    multiplication game, 42                             global connection games, 500–501
                    mutants, 717, 718, 722, 729–732                     local connection games, 489–494, 506–509
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    July 17, 2007   18:55




                    748                                            index

                    network formation games (cont.)                     Markov decision process model, 432
                      Nash equilibrium in potential games,              overview, 411–413
                         499–500                                        planning in model-based environments,
                      open problems, 508–511                               434–435
                      overview, 448–450, 487–489                        simple-price-based online auctions, 428
                      potential function method and price of            stochastic policies, 430–431
                         stability, 498–499                             theorems, 419, 420, 422, 423, 426, 427, 430,
                      potential games and congestion games,                433
                         497–498                                        truthfulness for single-value preference
                      potential games and global connection                domains, 417–420
                         games, 494–497, 509–510                      onto condition, 245, 247, 249–252, 263
                      theorems, 491–493, 497, 498, 500, 501, 503,     operationally complete market, 662
                         505, 506                                     opportunistic unchoking mechanism, 600
                    neutrality, 318, 320                              opportunity cost, 708–709
                    no dispute wheel, 373–374, 378–380                optimal contract, 605–607
                    no positive transfer (NPT), 392                   optimal sale price, 338, 341, 342
                    no-trade theorems, 657, 663, 672                  optimal single price profit, 345, 348
                    nonatomic selfish routing, 461–465, 468–470,      optimal stopping theory, 424–425
                         472–475, 478, 480–482, 499                   optimization program in sponsored search
                    noncooperatively computable (NCC), 197–199             engines, 710
                    nondegenerate, 56, 60                             optimization vs. equilibrium, 139–140
                    nondirect revelation, 223–224                     option set for strategy-proofness, 248
                    nonlinear Pigou’s example, 464, 479               OR bids, 280–283
                    nonoblivious cost-sharing scheme, 501             “OR” technology, 603–604, 606, 607, 669
                    nonprogressive vs. progressive processes,         organisms, in evolutionary game theory,
                         616–617, 621–622                                  717–718
                    nontransferable utilities (NTU) in cooperative
                         games, 385–386, 391,405. See also house      P2P. See peer-to-peer networks (P2P)
                         allocation problem                           PageRank, 404, 406, 408, 597, 689–690, 692
                    nonutilitarian, 518                               pairwise stable equilibrium, 507, 615, 729
                    normal form games, 161; see also standard form    parallel information sets, 70
                    Northwest corner rule, 704, 712                   parallel-serial topologies, 585–586
                    NP-completeness and Nash equilibrium, 31–33,      Pareto-optimality, 103, 245, 249, 662
                         271, 623, 661, 720, 723                      parimutuel games, 664–665
                    NTU. See nontransferable utilities                partial information model, 81, 94–96
                                                                      parties in multiparty computation, 182–184,
                    oblivious cost-sharing schemes, 501                     193–194
                    oligopoly pricing and equilibrium, 582–583,       partition model of knowledge, 653
                          586                                         path auctions, 351, 353, 354
                    one-dimensional strategies, 564                   path-vector, 371–373
                    one-shot simultaneous move games, 9               Pathrank algorithm, 690
                    online allocation problem, 707–711                pay per click, 699, 701, 703, 707, 711
                    online mechanism                                  pay-your-dues (PYD) strategy, 682–683, 695
                       adaptive, limited supply auction, 424–427      payment policy, 414–415, 422
                       challenge of, 412–413                          payoffs
                       dynamic auction with expiring items,             in bimatrix games, 54, 55
                          420–424                                       defining, 9
                       dynamic environments, 413–417                    evolutionarily stable strategy, 720–721
                       dynamic Vickrey–Clarke–Groves                    and inefficiency, 444, 453
                          mechanism, 433–434                            in parimutuel games, 665
                       ex-post incentive compatible, 428                with risk-neutral players, 13
                       future research, 435–436                         in scalable resource allocation mechanisms,
                       history, 436–437                                     555
                       known interesting-set assumption, 429–430        sequence form, 72–73
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 17, 2007    18:55




                                                                 index                                             749

                    payoff matrix, 8, 12, 15                              facility location games, 503–504
                    peer-prediction scoring, 686–689                      global connection games, 494–497, 509–510
                    peer-to-peer networks (P2P)                           Nash equilibrium, 499–500
                       barter-based system, 600–601                       price of stability, 498–499
                       and censorship resistance, 640                  PPAD, 36–39, 151–152, 156
                       currency as incentive, 601–602                  PPAD-complete, 16, 41–42, 44, 45
                       pricing and incentive models, 588–589           prediction markets
                       file-sharing game, 594–596                         automated market makers, 662–665
                       hidden actions, 602–608, 637                       combinatorial, 657–662
                       history, 608–609                                   definition, 651–652
                       open problems, 608                                 distributed computation, 665–669
                       overview, 593–594, 608                             history, 671–672
                       reputation as incentive, 596–600, 678              open problems, 670–671
                       theorems, 607                                      setup and notation, 652–654
                    peering, 377                                          survey of field, 654–657
                    perfect information, 67                               theorems, 660, 661, 668, 669
                    perfect recall, 54, 71                             preference ordering, 9
                    perfect security, 184                              prices
                    phantom feedback, 679                                 equilibrium, 123
                    Pigou’s example, 447–448, 456, 462–464, 469,       price anticipating users (in resource allocation
                          472–474, 479                                       games), 547–549
                    Pigouvian taxes, 480, 580; see also marginal       price characterization, 667–669
                          cost pricing                                 price competition game, 583
                    pivoting, 63–65                                    price correspondences, 657
                    players. See also bidders; specific games          price discriminate, 545
                       leaders, 43                                     price formation, 666–667
                       limited information, 20                         price of anarchy
                       loser-if-silent, 325                               of atomic selfish routing, 459, 463–466,
                       in multiplayer games. See graphical games             468–470, 473–479, 480–481
                       payoffs. See payoffs                               coordination ratio, 456
                       in peer-to-peer networks, 596                      definition, 445, 517, 520–522
                       price anticipating, 547–549                        facility location games, 504–505, 511
                       price takers, 546–547, 573, 574                    fully mixed Nash equilibrium, 531–533
                       risk-neutral, 13                                   in global connection games, 495
                       and transferable utility, 21–22                    in local connection games, 491–494
                       in two-person zero-sum games, 16–18                mixed equilibria on uniformly related
                    policy-consistency, 377–379                              machines, 533
                    pollution game, 5–6                                   of nonatomic selfish routing, 463–464,
                    polyhedra, 53, 57                                        472–477, 481, 447–448
                    polynomial local search (PLS) problems,               of the proportional sharing mechanism,
                          499–500                                            455–456
                    polynomial parity argument (directed case). See       pure equilibria for identical machines,
                          PPAD                                               522–523
                    polynomial weights (PW) algorithm, 86–88              pure equilibria for uniformly related
                    polytopes, 57–60, 65                                     machines, 524–528
                    population and strategy, 595–596, 613–614,            pure vs. mixed equilibria, 537–538
                          618–622. See also computational                 reducing in routing games, 478–480
                          evolutionary game theory                        of scalable resource allocation mechanisms,
                    positive association of differences (PAD), 318,          549–551, 558–559
                          319                                             in scheduling games, 451
                    potential function method, 448, 468, 469, 471,        utility games, 505, 507
                          472, 482, 489, 494, 496                      price of stability, 446–449, 490–491, 495,
                    potential games                                          498–499, 520
                       congestion games, 497–498                       price of unaccountability (POU), 605–607
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 17, 2007   18:55




                    750                                          index

                    price takers (in resource allocation games),       proportional allocation mechanism, 544–551,
                          546–547, 573, 574, 576                            558, 564
                    prices                                             proportional fairness, 125
                       automated market makers, 662–665                proportional sharing, 452, 455–456
                       in communications networks. See                 pseudonyms, 597, 679, 683
                          communications networks                      public good cost sharing, 251–252
                       for differentiated services, 587–588            pure strategy Nash equilibrium, 12–13, 55, 466,
                       equilibrium, 23, 25, 108–109, 135; see also          519, 520, 528–529, 724
                          market equilibria                            PW algorithm. See polynomial weights (PW)
                       and information security, 638                        algorithm
                       market clearing, 23, 24, 105, 106, 122;
                          see also market clearing prices              quadratic scoring rule market maker, 664
                       in sponsored search auctions, 699–701           quality of service (QoS), 587
                       uniqueness of, 230–231                          query model (iterative auctions), 283–287, 310
                    pricing game, 14, 502
                    primal-dual schema, 104, 109–110, 126, 291,        random ordering, 403, 424, 427
                          394–400, 407                                 random replenishment, 644
                    Prim’s algorithm, 501                              random sampling empirical Myerson, 341–342
                    principal-agent model in peer-to-peer networks,    random sampling optimal price (RSOP) auction,
                          602–606                                            341–346, 355, 357
                    prior distribution, 333, 337, 339                  random sampling profit extraction auction,
                    prior-free mechanism design, 344–350                     348–349
                       convergence rates, 342–344                      randomized-greedy (RG) algorithm, 83, 84
                       empirical distributions, 339–341                randomized incentive compatible mechanisms,
                       random sampling, 341–342                              231–233
                    prior probability distribution,653; see also       randomized rounding, 307–308
                          Bayesian-Nash implementation                 randomized scheduling algorithm, 307–308
                    Prisoners’ dilemma, 3–6, 443–444, 446–447,         randomized strategies, 8–9; see also mixed
                          595, 680, 681                                      strategies
                    privacy and correctness properties, 184,           randomized weighted majority (RWM)
                          194–195, 197                                       algorithm, 85–86
                    probabilistic functions, 182, 186, 201, 620,       rank-strategyproof, 690
                          679                                          rater reputations, 679–680, 684–688, 695
                    procurement auction, 220, 269                      rational expectations equilibrium, 652,
                    profit benchmark, 333, 344–345, 349, 350, 354            656–657, 672
                    profit extraction problem, 347                     rational multiparty computation, 199–202
                    profit extractor, 347–350, 358                     realization plan, 71–74
                    profit maximization and mechanism design           reciprocity, 594, 600
                       Bayesian optimal mechanism design,              recommendation incentive programs, 626–627,
                          335–338                                            630
                       in communications networks, 579–582             Red-Blue utility model, 640–641
                       examples and applications, 331–332              reduced strategy, 69–70
                       frugality, 350–354                              reductions, 41–45
                       history, 357–358                                regret analysis
                       overview, 331–334                                  external regret minimization, 82–88
                       prior-free approximations to the optimal           generic reduction from external to swap
                          mechanism, 339–344                                 regret, 92–94
                       prior-free optimal mechanism design,               lower bounds, 87–88
                          344–350                                         model, 81–82
                       future research, 354–357                           overview, 80–81, 99
                       theorems, 334, 336, 338, 340, 341, 343, 345,       partial information model, 94–96
                          346, 348, 350, 353                              regret minimization and game theory, 88–92
                    progressive vs. nonprogressive processes,             regret minimization strategies in routing
                          616–617, 621–622                                   games, 96–99
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0   July 17, 2007    18:55




                                                                index                                             751

                       theorems, 82–85, 87, 88                          price of anarchy in atomic selfish routing,
                    relative optimality, 333; see also competitive         475–477
                          analysis                                      price of anarchy in nonatomic selfish routing,
                    replicator dynamics, 727                               472–475
                    reputation as incentive, 594, 596–600, 678          reducing the price of anarchy, 478–480
                    reputation systems (manipulation-resistant)         theorems, 468, 471, 472, 476, 478, 479
                       dynamics, 678                                  routing matrix, 572, 575
                       effect of, 680–683                             routing protocol, 371–379
                       eliciting effort and honest feedback,          routing security, 636
                          683–689                                     RSEM. See random sampling empirical
                       history, 694–695                                    Myerson
                       importance of, 677–680                         RSOP auction. See random sampling optimal
                       meta-evaluation, 684                                price (RSOP) auction
                       metrics and benchmarks in reputation           RSPE auction. See random sampling profit
                          systems, 694                                     extraction auction
                       open problems and extensions, 693–694          RWM algorithm. See randomized weighted
                       sybilproofness, 690–693                             majority (RWM) algorithm
                       theorems, 691, 692
                       and transitive trust, 689–693                  satisfiability, 31–33, 500, 524, 529
                       whitewashing, 682–683                          scalable resource allocation mechanisms
                    residency matching, 255                              characterization theorem, 551–559
                    resilient equilibrium, 191–192                       history, 565–566
                    resource allocation markets, 124–125,                overview, 543–544, 564
                          452–454,573. See also communications           proportional allocation, 544–551
                          networks; scalable resource allocation         theorems, 546, 547, 549, 554
                          mechanisms                                     Vickrey–Clarke–Groves approach to,
                    revelation principle, 12, 224–225, 231, 234,            559–563
                          356, 416–417, 589                           scalar strategy VCG mechanisms, 559–563
                    revenue equivalence, 236–237, 356, 705            scale-free networks, 643, 648
                    revenue maximization. See profit maximization     scheduling games. See load balancing games
                          and mechanism design                        scheduling related machines, 303–304,
                    reverse auction, 220                                    450–452, 577
                    ring structure, 644–645, 647                      scoring peer-prediction, 686–689
                    risk aversion model, 238                          second-price auction. See Vickrey auction
                    risk-neutral, 13                                  second welfare theorem, 278
                    Roberts theorem, 228                              secret-sharing, 186–187, 200, 201
                    rock-paper-scissors game, 44, 45                  secretary problem, 424–425, 427
                    routing congestion game, 7–8, 96–99; see also     secure and broadcast channels, 185
                          routing games                               securities markets. See prediction markets
                    routing games                                     security. See information security
                       atomic selfish routing, 465–468, 482–483       security of multiparty computation, 182–185,
                       Braess’s Paradox, 464–465, 475, 481                  190
                       existence and uniqueness, 468–470              security parameters, 185
                       vs. global connection games, 495               seeder, 600
                       history, 480–483                               selfish load balancing. See load balancing games
                       network formation games. See network           selfish routing, 447–448, 723–728; see also
                          formation games                                   routing games
                       nonatomic selfish routing, 462–465, 480–482    semihonest parties, 182
                       nonexistence in weighted atomic instances,     sequence form, 70–74
                          467                                         sequential decision problem, 431, 437
                       overview, 461–462                              serial connection, 585–586
                       Pigou’s example, 447–448, 456, 462–464,        service differentiation, 598–600
                          469, 472–474, 479                           Shamir secret-sharing scheme, 186, 187, 201
                       potential function, 470–472                    Shapley cost-sharing mechanism, 495
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 17, 2007   18:55




                    752                                        index

                    Shapley network design game, 448–450; see        specification faithfulness, 601
                          also network formation games               spectrum auctions, 269, 295
                    Shapley value, 22, 368–369, 402–405, 407–408,    SPNE. See subgame perfect equilibrium
                          489, 501                                   sponsored search auctions
                    signal, 685, 687, 688                               discussion of practice, 712
                    simple polytope, 60                                 dynamic aspects of, 707–711
                    simple pricing rules, 590                           equilibrium properties, 705–707
                    simultaneous move game, 9                           history, 712–713
                    simultaneous reporting game, 685                    models and mechanisms, 701–702
                    single-dimensional domains, 303–310; see            open problems, 711–712
                          single-parameter domains                      overview, 699–701
                    single-item auction, 332, 335, 337, 338, 351        static model, 702–707
                    single-minded bidders, 270–275, 295, 323–324,       theorems, 706, 709
                          332                                        stable matching problem
                       greedy mechanism for single-minded bidders,      college student matching, 255, 261
                          273–274                                       deferred acceptance algorithm, 256–258
                    single-parameter domains, 228–230, 303–310,         extensions, 261–262
                          350, 354, 356, 417–420                        lattice formulation, 259–260
                    single-peaked preferences, 244–252                  LP formulation, 260–261
                    single-source multiple-sink markets algorithm,      overview, 255
                          126–131                                    stalling, 433
                    single-value players, 322, 324–325               standard form, 9–10
                    single-valued preference domains. See Single     statistical security, 184
                          parameter domains                          Steiner forest problem, 406, 407, 495
                    slots, 699                                       Stirling’s formula, 288
                    smart market, 587                                stochastic policies, 430–431
                    Smith, Adam, 217                                 strategic and privacy equivalence, 196
                    smooth market-clearing mechanism, 552–554        strategic form. See standard form
                    social choice                                    strategic network formation, 594, 609; see also
                       Arrow’s theorem, 212–213                            network formation games
                       Condorcet’s paradox, 211                      strategic voting, 211–212
                       Gibbard–Satterthwaite theorem, 213–215        strategy proof mechanism. See truthfulness
                       and mechanism design, 209, 210                strategy-proof rules, 243–251, 258, 262, 263,
                       and mechanisms with money, 216–222                  690
                       voting methods, 211–212                       strategy, 9, 10, 12, 18, 556, 561
                    social choice function, 212–215, 225–226, 237,   strict equilibrium, 586
                          405                                        strict incomplete information, 222–223
                    social cost, 488, 490–491, 518, 520–522,         strict quasi-concavity, 137
                          528                                        strong Nash equilibrium, 21
                    social network, 614–618, 622–625, 627, 637,      strong truthfulness, 415, 430
                          643, 630                                   subgame perfect equilibrium, 19–20, 68–69,
                    social welfare function, 212–213, 215, 218             681–683
                    socially efficient networks, 488, 490, 682–683   subgames, 54
                    sock puppet identities. See phantom feedback;    submodular function, 624–626, 630
                          Sybil attacks                              submodular games, 395–397, 403, 504
                    software security, 638                           submodularity, 623–626
                    solution concepts                                substitutes vs. complements, 139, 268, 290,
                       correlated equilibrium, 14–15                       292
                       dominant strategy, 10–12                      succinct game representations, 39–41, 48–49
                       mixed strategy Nash equilibrium, 13           supply and demand, 135; see also market
                       pure strategy Nash equilibrium, 12–13               equilibria
                    source routing, 481, 603                         support, 31, 34–36, 54
                    spanning tree auctions, 351                      surplus sharing problem,386; see also cost
                    sparse games, 40                                       sharing
P1: SBT
9780521872829main       CUNY1061-Nisan       0 521 87282 0     July 17, 2007    18:55




                                                                   index                                             753

                    surplus vector, 112, 121, 659–660                    ultimatum game, 19
                    surplus, 119–121, 335–337,583. See also              uniqueness of prices, 230–231
                         market equilibria                               unit demand, 280
                    swap regret                                          upper envelope, 57, 59
                      definition, 80–82                                  users. See players
                      and dominated strategies, 91–92                    utilitarian function, 443
                      generic reduction from external to, 92–94          utility, 331, 334, 357
                      minimization and correlated equilibrium,           utility function
                         90–91                                              Cobb-Douglas, 139, 143, 146, 155
                    swarming download, 600, 601                             definition, 9–10
                    Sybil attacks, 597, 601, 602, 608, 679, 680,            gross substitutability, 138, 145
                         690–693                                            in information security, 640–641
                    symmetric game, 30, 40, 45–46, 340                      Leontief, 139, 152
                                                                            market equilibria, 131, 148–150
                    Tarski’s fixed point theorem, 259–260                   maximizing with convex programs, 106
                    tâtonnement process, 137–138, 144, 147                 for scalable resource allocation mechanisms,
                    TCP congestion control, 104–105                            544–545, 556
                    thin market problem, 662                                special forms of, 139
                    tit-for-tat strategy, 595, 596
                    top trading cycle mechanism, 254                     valuation, 12, 20, 216–222, 238–240, 268,
                    traffic light example, 14–15                               331–334, 335–339, 355, 356, 374
                    tragedy of the commons, 6–7, 595                     value queries, 284
                    transferable utilities (TU) in cooperative games,    variational inequalities, 473–474
                           21–22, 385–391                                VCG mechanism. See Vickrey–Clarke–Groves
                    transitivity of trust, 679, 680, 689–693                   mechanisms
                    tree graphical games, 164–169                        vertex-order attacks, 644–646
                    TreeNash, 164–167, 176                               Vickrey auction, 11–12, 216–217, 220, 335,
                    trembling hand perfect equilibrium, 503                    422, 703–704
                    trusted parties, 182, 190                               reserve price, 338
                    truthful with high probability                       Vickrey–Clarke–Groves mechanisms
                    truthfulness                                            and Clarke pivot rule, 219, 221
                        adaptive limited-supply auction, 425–426            competitive communications network
                        automated market makers, 662–665                       problems, 573
                        in canonical expiring items environment, 412        definition, 218–219
                        combinatorial auctions, 312–314                     distributed implementation of, 366–367
                        and dominant strategy incentive-compatible,         in dynamic environments, 434–435
                           415                                              and frugality, 352–353
                        with high probability, 349–350                      incentive compatible approximation, 273
                        and profit maximization, 356–357                    marginal cost, 368–370
                        single-valued preference domains, 417–420           multidimensional domains and combinatorial
                    TU. See transferable utilities                             auctions, 311
                    two-person zero-sum games, 16–18, 73                    scalable resource allocation mechanisms,
                    two-player game equilibrium computation                    559–564
                        bimatrix games and best response, 54–57             and Walrasian equilibrium, 292
                        degenerate games, 65–66                             with scaler strategies, 559–563
                        extensive games, 66–68                              weighted, 227–228
                        further reading for, 75                          viral marketing, 622–623, 626–627, 630
                        integer pivoting, 63–65                          virtual surplus, 336, 337, 338
                        via labeled polytopes, 57–60                     virtual valuation, 335–336, 338
                        Lemke–Howson algorithm, 61–63                    voluntary participation (VT), 392, 608; see also
                        overview, 53–54, 75–76                                 individual rationality
                        reduced strategic form, 69–70                    voting and mechanism design, 209, 211–215,
                        sequence form, 70–73                                   246
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0   July 17, 2007   18:55




                    754                                          index

                    voyeurism, 197                                    weighted Vickrey–Clarke–Groves mechanisms,
                                                                           227–228
                    Walras’ Law, 137, 147                             WGS. See weak gross sustainability (WGS)
                    Walrasian equilibrium, 277–279, 290–292,          whitewashing attacks, 597, 601, 602, 608, 679,
                         121–122                                           682–683, 695
                    Walrasian model. See Arrow–Debreu model           winner’s curse, 238
                    Wardrop equilibria, 480, 579–581, 724; see also   wireless networks, 577, 588, 589
                         equilibria nonatomic flow                    “The Wisdom of Crowds”, 652
                    Wardrop model of traffic flow, 96–98,585; see     WMON. See weak monotonicity
                         also selfish routing                         worst-case analysis, 333, 357, 558; see also
                    weak gross substitutability, 131                       competitive analysis
                    weak gross sustainability (WGS), 138, 142–148
                    weak monotonicity, 226–227, 304–305,              XOR bids, 280–283, 668
                         307–309, 318–319, 428
                    weighted-packing problem, 271                     Zermelo’s algorithm, 69
                    threshold function, 669                           zero-sum games, 16–18, 73, 662
