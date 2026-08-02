---
type: "book-chapter"
book_id: "algorithmic-game-theory"
chapter_id: "ch-20"
chapter_number: 20
chapter_title: "Selﬁsh Load Balancing Berthold V¨ocking"
source_pdf: "raw/inbox/manual-drop/PDF_B.pdf"
source_page_start: 538
source_page_end: 563
printed_page_start: 538
printed_page_end: 563
part_ids: ["algorithmic-game-theory-ch-20-part-021"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Selﬁsh Load Balancing Berthold V¨ocking

P1: SBT
9780521872829main       CUNY1061-Nisan        0 521 87282 0     July 5, 2007    14:34




                                                              CHAPTER 20


                                        Selfish Load Balancing

                                                         Berthold Vöcking




                                                                 Abstract

                    Suppose that a set of weighted tasks shall be assigned to a set of machines with possibly different
                    speeds such that the load is distributed evenly among the machines. In computer science, this problem
                    is traditionally treated as an optimization problem. One of the classical objectives is to minimize the
                    makespan, i.e., the maximum load over all machines. Here we study a natural game theoretic variant
                    of this problem: We assume that the tasks are managed by selfish agents, i.e., each task has an agent
                    that aims at placing the task on the machine with smallest load. We study the Nash equilibria of this
                    game and compare them with optimal solutions with respect to the makespan. The ratio between the
                    worst-case makespan in a Nash equilibrium and the optimal makespan is called the price of anarchy.
                    In this chapter, we study the price of anarchy for such load balancing games in four different variants,
                    and we investigate the complexity of computing equilibria.


                                                          20.1 Introduction

                    The problem of load balancing is fundamental to networks and distributed systems.
                    Whenever a set of tasks should be executed on a set of resources, one needs to balance
                    the load among the resources in order to exploit the available resources efficiently.
                    Often also fairness aspects have to be taken into account. Load balancing has been
                    studied extensively and in many variants. One of the most fundamental load balancing
                    problems is makespan scheduling on uniformly related machines. This problem is
                    defined by m machines with speeds s1 , . . . , sm and n tasks with weights w1 , . . . , wn .
                    Let [n] = {1, . . . , n} denote the set of tasks and [m] = {1, . . . , m} the set of machines.
                    One seeks for an assignment A : [n] → [m] of the tasks to the machines that is as
                    balanced as possible. The load of machine j ∈ [m] under assignment A is defined as
                                                                    wi
                                                            j =            .
                                                                   i∈[n]
                                                                         sj
                                                                      j =A(i)


                    The makespan is defined to be the maximum load over all machines. The objective is
                    to minimize the makespan. If all machines have the same speed, then the problem is
                                                                      517
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0        July 5, 2007      14:34




                    518                             selfish load balancing

                    known as makespan scheduling on identical machines, in which case we shall assume
                    s1 = s2 = · · · = sm = 1.
                       In computer science, load balancing is traditionally viewed as an algorithmic prob-
                    lem. We design and analyze algorithms, either centralized or distributed, that compute
                    the mapping A. Suppose, however, there is no global authority that can enforce an
                    efficient mapping of the tasks to the machines. For example, in a typical Internet appli-
                    cation, tasks might correspond to requests for downloading large files that users send
                    to servers. To maximize the quality of service, each of the users aims at contacting a
                    server with smallest load. This naturally leads to the following game theoretic setting
                    in which we will be able to analyze what happens to the makespan if there is no global
                    authority but selfish users aiming at maximizing their individual benefit decide about
                    the assignment of tasks to machines.
                       This chapter differs from the other chapters in Part III of this book in two important
                    aspects. At first, the considered objective function, the makespan, is nonutilitarian. At
                    second, our analysis does not only consider pure but also mixed equilibria. By using
                    the makespan as objective function, our analysis simultaneously captures the aspects of
                    efficiency and fairness. By considering mixed equilibria, our analysis explicitly takes
                    into account the effects of uncoordinated random behavior.


                                               20.1.1 Load Balancing Games
                    We identify agents and tasks, i.e., task i ∈ [n] is managed by agent i. Each agent can
                    place its task on one of the machines. In other words, the set of pure strategies for an
                    agent is [m]. A combination of pure strategies, one for each task, yields an assignment
                    A : [n] → [m]. We assume that the cost of agent i under the assignment A corresponds
                    to the load on machine A(i), i.e., its cost is A(i) . The social cost of an assignment
                                                                                                       is
                    denoted cost(A) and is defined to be the makespan, i.e., cost(A) = maxj ∈[m] j .
                        Agents may use mixed strategies, i. e., probability distributions on the set of pure
                                      j
                    strategies. Let pi denote the probability that agent i assigns its task to machine j , i.e.,
                      j                                             j
                    pi = P[A(i) = j ]. A strategy profile P = (pi )i∈[n],j ∈[m] specifies the probabilities for
                    all agents and all machines. Clearly, every strategy profile P induces a random mapping
                                                  j
                    A. For i ∈ [n], j ∈ [m], let xi be a random variable that takes the value 1 if A(i) = j
                    and 0, otherwise. The expected load of machine j under the strategy profile P is
                    thus
                                               ⎡                 ⎤
                                                    wi x j               wi E[x j ]             wi p j
                                   E[j ] = E ⎣                i ⎦
                                                                     =                   i
                                                                                             =                i
                                                                                                                  .
                                                 i∈[n]
                                                          sj             i∈[n]
                                                                                   sj            i∈[n]
                                                                                                         sj

                    The social cost of a strategy profile P is defined as the expected makespan, i.e.,
                                                                        
                                           cost(P ) = E[cost(A)] = E max j                         .
                                                                                  j ∈[m]


                    We assume that every agent aims at minimizing its expected cost. From point of view
                                                                             j       j
                    of agent i, the expected cost on machine j , denoted by ci , is ci = E[j | A(i) = j ].
P1: SBT
9780521872829main      CUNY1061-Nisan         0 521 87282 0    July 5, 2007   14:34




                                                              introduction                                519

                    For any profile P ,
                                                                  j
                                          j    wi +      k=i wk pk                     j      wi
                                      ci =                            = E[j ] + (1 − pi ) ·      .    (20.1)
                                                        sj                                     sj
                       In general, a strategy profile of a game is a Nash equilibrium if there is no incentive
                    for any agent to unilaterally change its strategy. For the load balancing game, such a
                    profile is characterized by the property that every agent assigns positive probabilities
                    only to those machines that minimize its expected cost. This is formalized as follows.

                      Proposition 20.1 A strategy profile P is a Nash equilibrium if and only if
                                             j                   j
                      ∀i ∈ [n] : ∀j ∈ [m] : pi > 0 ⇒ ∀k ∈ [m] : ci ≤ cik .

                       The existence of a Nash equilibrium in mixed strategies is guaranteed by the theorem
                    of Nash, see Chapters 1 and 2. A strategy profile P is called pure if, for each agent,
                    there exists only one machine with positive probability. A Nash equilibrium in pure
                    strategies is called a pure Nash equilibrium. Applying the proposition above to pure
                    profiles and the corresponding assignments yields the following characterization of a
                    pure Nash equilibrium.

                      Proposition 20.2 An assignment A is a pure Nash equilibrium if and only if
                      ∀i ∈ [n] : ∀k ∈ [m] : ciA(i) ≤ cik .

                       In words, an assignment is a pure Nash equilibrium if and only if no agent can
                    improve its cost by unilaterally moving its task to another machine. A special property
                    of load balancing games is that they always admit pure Nash equilibria.

                      Proposition 20.3 Every instance of the load balancing game admits at least
                      one pure Nash equilibrium.

                      proof An assignment A induces a sorted load vector (λ1 , . . . , λm ), where λj
                      denotes the load on the machine that has the j -th highest load. If an assignment is
                      not a Nash equilibrium, then there exists an agent i that can perform an improve-
                      ment step, i.e., it can decrease its cost by moving its task to another machine.
                      We show that the sorted load vector obtained after performing an improvement
                      step is lexicographically smaller than the one preceding it. Hence, a pure Nash
                      equilibrium is reached after a finite number of improvement steps.
                         Suppose, given any sorted load vector (λ1 , . . . , λm ), agent i performs an im-
                      provement step and moves its task from machine j to machine k where the indices
                      are with respect to the positions of the machines in the sorted load vector. Clearly,
                      k > j . The improvement step decreases the load on machine j and it increases the
                      load on machine k. However, the increased load on machine k is smaller than λj
                      as, otherwise, agent i would not decrease its cost. Hence, the number of machines
                      with load at least λj is decreasing. Furthermore, the loads on all other machines
                      with load at least λj are left unchanged. Consequently, the improvement step
                      yields a sorted load vector i.e. lexicographically smaller than (λ1 , . . . , λm ).
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0     July 5, 2007   14:34




                    520                             selfish load balancing

                       Thus improvement steps naturally lead to a pure Nash equilibrium. This issue is
                    also discussed for a broader class of games, so-called potential games, in Chapter 19.
                    Let us remark that this convergence result implies that there exists even a pure Nash
                    equilibrium that minimizes the makespan. Given any optimal assignment, such an
                    equilibrium can be found by performing improvement steps until a Nash equilibrium
                    is reached because improvement steps do not increase the makespan. Thus, for load
                    balancing games with social cost equal to the makespan, it does not make much sense
                    to study the ratio between the social cost in a best Nash equilibrium and the optimal
                    social cost. This ratio is called the “price of stability.” It is studied in Chapters 17–19
                    in the context of other games. In this chapter, we are mainly interested in the ratio
                    between the social cost of the worst Nash equilibrium and the optimal social cost, the
                    so-called the “price of anarchy.”


                                         20.1.2 Example of a Load Balancing Game
                    Suppose that there are two identical machines both of which have speed 1 and four tasks,
                    two small tasks of weight 1 and two large tasks of weight 2. An optimal assignment
                    would map a small and a large task to each of the machines so that the load on both
                    machines is 3. This assignment is illustrated in Figure 20.1(a).
                        Now consider an assignment A that maps the two large tasks to the first machine and
                    the two small tasks to the second machine as illustrated in Figure 20.1(b). This way, the
                    first machine has a load of 4 and the second machine has a load of 2. Obviously, a small
                    task cannot improve its cost by moving from the second to the first machine. A large
                    task cannot improve its cost by moving from the first to the second machine either as its
                    cost would remain 4 if it does. Thus assignment A constitutes a pure Nash equilibrium
                    with cost(A) = 4. Observe that all assignments that yield a larger makespan than 4
                    cannot be a Nash equilibrium as, in this case, one of the machines has a load of at least
                    5 and the other has a load of at most 1 so that moving any task from the former to the
                    latter would decrease the cost of this task. Thus, for this instance of the load balancing
                    game, the social cost of the worst pure Nash equilibrium is 4.
                        Clearly, the worst mixed equilibrium cannot be better than the worst pure equilibrium
                    as the set of mixed equilibria is a superset of the set of pure equilibria, but can it really
                    be worse? Suppose that each task is assigned to each of the machines with probability

                                                       (a)                     (b)




                    Figure 20.1. (a) Illustration of the optimal assignment of an instance of the load balancing
                    game with two large tasks of size 2 and two small tasks of size 1 as described in the example
                    given in Section 20.1.2. The social cost of this assignment is 3. (b) Illustration of a pure Nash
                    equilibrium for the same instance. The social cost of this assignment is 4, which is the maximum
                    among all pure Nash equilibria for this instance.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    July 5, 2007     14:34




                                                           introduction                                     521

                                                                       j
                    1
                    2
                      . This corresponds to a strategy profile P with pi = 12 for 1 ≤ i ≤ 4, 1 ≤ j ≤ 2. The
                    expected load on machine j is thus
                                                        j          1         1
                                      E[j ] =       wi pi = 2 · 2 · + 2 · 1 · = 3.
                                               1≤i≤4
                                                                    2         2

                    It is important to notice that the expected cost of a task on a machine is larger than
                    the expected load of the machine, unless the task is assigned with probability 1 to this
                    machine. For example, if we assume that task 1 is a large task then Equation 20.1 yields
                                                                                     1
                                          c11 = E[1 ] + (1 − p11 ) w1 = 3 +           · 2 = 4,
                                                                                     2
                    and, if task 3 is a small task, then
                                                                              1
                                         c31 = E[1 ] + (1 − p31 ) w3 = 3 +     · 1 = 3.5.
                                                                              2
                    For symmetry reasons, the expected cost of each task under the considered strategy
                    profile P is the same on both machines so that P is a Nash equilibrium. The social cost of
                    this Nash equilibrium, cost(P ), is defined to be the expected makespan, E[cost(A)], of
                    the random assignment A induced by P . The makespan, cost(A), is a random variable.
                    This variable can possibly take one of the four values 3, 4, 5, or 6. There are 24 = 16
                    different assignments of four tasks to two machines. The number of assignments that
                    yield a makespan of 3 is 4, 4 is 6, 5 is 4, and 6 is 2. Consequently, the social cost of the
                    mixed Nash equilibrium is
                                                         1
                              cost(P ) = E[cost(A)] =       (3 · 4 + 4 · 6 + 5 · 4 + 6 · 2) = 4.25.
                                                        16
                    Thus mixed equilibria can, in fact, be worse than the worst pure equilibrium.


                                         20.1.3 Definition of the Price of Anarchy
                    Not surprisingly, the example above shows that uncoordinated, selfish behavior can
                    lead to suboptimal assignments. We are interested in the ratio between the social cost
                    (makespan) of a worst-case Nash equilibrium, i.e., the Nash equilibrium with highest
                    social cost, and the social cost of an optimal assignment.

                      Definition 20.4 (Price of anarchy) For m ∈ N, let G(m) denote the set of all
                      instances of load balancing games with m machines. For G ∈ G(m), let Nash(G)
                      denote the set of all strategy profiles being a Nash equilibrium for G, and let
                      opt(G) denote the minimum social cost over all assignments. Then the price of
                      anarchy is defined by
                                                                                cost(P )
                                               PoA(m) = max            max                 .
                                                            G∈G(m) P ∈Nash(G) opt(G)


                       In the following, we study the price of anarchy in load balancing games in four
                    different variants in which we distinguish, as a first criterion, between games with
                    identical and uniformly related machines and, as a second criterion, between pure
P1: SBT
9780521872829main      CUNY1061-Nisan    0 521 87282 0   July 5, 2007   14:34




                    522                           selfish load balancing

                    Nash equilibria and mixed Nash equilibria. Technically, when considering the price of
                    anarchy for load balancing games with identical machines then we restrict the set G(m)
                    to instances in which the m machines have all the same speed. When considering the
                    price of anarchy with respect to pure equilibria then the set Nash(G) refers only to pure
                    Nash equilibria rather than mixed equilibria; i.e., we take the maximum only among
                    pure equilibrium assignments rather than among possibly mixed equilibrium strategy
                    profiles.
                       The motivation behind studying the price of anarchy is to quantify the increase of
                    the social cost due to selfish behavior. With this motivation in mind, does it make
                    more sense to consider pure or mixed equilibria? If one wants to study a distributed
                    system in which agents repeatedly perform improvement steps until they reach a Nash
                    equilibrium, then pure equilibria are the right solution concept. However, there might
                    be other means by which agents come to a Nash equilibrium. In particular, if one views
                    load balancing games as one shot games, then mixed equilibria are a very reasonable
                    solution concept. Moreover, upper bounds about the price of anarchy for mixed equi-
                    libria are more robust than upper bounds for pure equilibria as mixed equilibria are
                    more general than pure ones. In this chapter, we consider both of these equilibrium
                    concepts. Our analysis begins with the study of pure equilibria as they are usually
                    easier to handle than mixed equilibria whose analysis requires a bit of probability
                    theory.


                                    20.2 Pure Equilibria for Identical Machines

                    Our analysis of equilibria in load balancing games begins with the most basic case,
                    namely the study of pure equilibria on identical machines. Our first topic is the price
                    of anarchy. As a second topic, we investigate how long it takes until a pure Nash equi-
                    librium is reached when the agents repeatedly perform “best response” improvement
                    steps.


                                                20.2.1 The Price of Anarchy
                    In case of pure equilibria and identical machines, the analysis of the price of anarchy
                    is quite similar to the well-known analysis of the greedy load balancing algorithm that
                    assigns the tasks one after the other in arbitrary order giving each task to the least
                    loaded machine. Graham (1966) has shown that the approximation factor of the greedy
                    algorithm is 2 − m1 . We show that the price of anarchy for pure equilibria is, in fact,
                    slightly better than the approximation factor of the greedy algorithm.


                      Theorem 20.5 Consider an instance G of the load balancing game with n tasks
                      of weight w1 , . . . , wn and m identical machines. Let A : [n] → [m] denote any
                      Nash equilibrium assignment. Then, it holds that

                                                                  2
                                              cost(A) ≤ 2 −               · opt(G).
                                                                 m+1
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:34




                                        pure equilibria for identical machines                           523

                      proof Let j ∗ be a machine with the highest load under assignment A, and
                      let i ∗ be a task of smallest weight assigned to this machine. Without loss of
                      generality, there are at least two tasks assigned to machine j ∗ as, otherwise,
                      cost(A) = opt(G) so that the upper bound given in the theorem follows trivially.
                      Thus wi ∗ ≤ 12 cost(A).
                         Suppose there is a machine j ∈ [n] \ {j ∗ } with load less than j ∗ − wi ∗ . Then
                      moving the task i ∗ from j ∗ to j would decrease the cost for this task. Hence, as
                      A is a Nash equilibrium, it holds
                                                                  1          1
                                    j ≥ j ∗ − wi ∗ ≥ cost(A) −    cost(A) = cost(A).
                                                                  2          2
                      Now observe that the cost of an optimal assignment cannot be smaller than the
                      average load over all machines so that
                                                        i∈[n] wi
                                           opt(G) ≥
                                                          m
                                                        j ∈[m] j
                                                  =
                                                          m
                                                     cost(A) + 12 cost(A)(m − 1)
                                                  ≥
                                                                   m
                                                     (m + 1)cost(A)
                                                  =                   .
                                                             2m
                      As a consequence,
                                                 2m                 2
                                    cost(A) ≤       · opt(G) = 2 −                     · opt(G).
                                                m+1                m+1


                       Observe that the example of a game instance with two identical machines given
                    in Section 20.1.2 has a price of anarchy of 43 = 2 − m+1
                                                                           2
                                                                             , for m = 2. Exercise 20.2
                    generalizes this example. It shows that, for every m ∈ N, there exists an instance G
                    of the load balancing game with m identical machines and 2m tasks that has a Nash
                    equilibrium assignment A : [n] → [m] with
                                                                   2
                                              cost(A) = 2 −                · opt(G).
                                                                  m+1
                    Thus the upper bound on the price of anarchy given in Theorem 20.5 is tight.


                                        20.2.2 Convergence Time of Best Responses
                    Our analysis about the price of anarchy leaves open the question of how agents may find
                    or compute a Nash equilibrium efficiently. In the existence proof for pure equilibria
                    in Proposition 20.3, we have implicitly shown that every sequence of improvement
                    steps by the agents leads to a Nash equilibrium. However, if players do not converge to
                    an equilibrium in reasonable time, then it might also not matter if the finally reached
                    equilibrium is good. This naturally leads to the question of how many improvement
                    steps are needed to reach a Nash equilibrium. The following result shows that, in case
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:34




                    524                            selfish load balancing

                    of identical machines, there is a short sequence of improvement steps that leads from
                    any given initial assignment to a pure Nash equilibrium. An agent is said to be satisfied
                    if it cannot reduce its cost by unilaterally moving its task to another machine. The max-
                    weight best response policy activates the agents one after the other always activating
                    an agent with maximum weight among the unsatisfied agents. An activated agent plays
                    a best response; i.e., the agent moves its task to the machine with minimum load.

                      Theorem 20.6 Let A : [n] → [m] denote any assignment of n tasks to m iden-
                      tical machines. Starting from A, the max-weight best response policy reaches a
                      pure Nash equilibrium after each agent was activated at most once.

                      proof We claim, once an agent i ∈ [n] was activated and played its best re-
                      sponse, it never gets unsatisfied again. This claim immediately implies the the-
                      orem. Our analysis starts with two observations both of which holding only for
                      identical machines. At first, we observe that an agent is satisfied if and only if its
                      task is placed on a machine on which the load due to the other tasks is minimal.
                      At second, we observe that a best response never decreases the minimum load
                      among the machines. As a consequence, a satisfied agent can get unsatisfied only
                      for one reason: the load on the machine holding its task increases because another
                      agent moves its task to the same machine. Suppose that agent k is activated after
                      agent i, and it moves its task to the machine holding task i. Let j ∗ denote the
                      machine on which i is placed and to which k is moved. For j ∈ [m], let j denote
                      the load on machine j at the time immediately after the best response of agent k.
                      Since the assignment of k to j ∗ is a best response and as wk ≤ wi because of the
                      max-weight policy, it follows
                                                    j ∗ ≤ j + wk ≤ j + wi ,
                      for all j ∈ [m]. Hence, after the best response of k, agent i remains satisfied
                      on machine j ∗ as it cannot reduce its cost by moving from j ∗ to any other
                      machine.

                        Let us remark that the order in which the agents are activated is crucial. For example,
                    if one would always activate an agent of minimum weight among the unsatisfied agents,
                    then there are instances of load balancing games on identical machines where one needs
                    an exponential number of best response steps to reach a pure Nash equilibrium (Even-
                    Dar et al., 2003).


                              20.3 Pure Equilibria for Uniformly Related Machines

                    We now switch from identical to uniformly related machines. First, we study the price
                    of anarchy. Then we discuss the complexity of computing equilibria.


                                                20.3.1 The Price of Anarchy
                    The analysis in Section 20.2.1 shows that, in case of identical machines, the makespan
                    of a pure Nash equilibrium is less than twice the optimal makespan. In this section, we
P1: SBT
9780521872829main       CUNY1061-Nisan           0 521 87282 0   July 5, 2007       14:34




                                      pure equilibria for uniformly related machines                                     525

                                               (a)                                                 (b)
                                                                                                                 q
                       c
                                                                             k+1
                    c −1
                                                                                k
                    c −2
                                                                             k−1
                    c −3

                              Lc −1
                                                                                            Lk+1            Lk
                                       Lc −2
                                               Lc −3

                    Figure 20.2. (a) Illustration of the definition of the lists L c−1 , L c−2 , . . . , L 0 from the proof of
                    Theorem 20.7. (b) Illustration of the lists L k and L k+1 and the machine q used in the proof of
                    Lemma 20.8.



                    show that the makespan of pure equilibria on uniformly related machines can deviate
                    by more than a constant factor. The price of anarchy is bounded, however, by a slowly
                    growing function in the number of machines. Our analysis begins with an upper bound
                    on the price of anarchy followed by the presentation of a family of game instances that
                    match this upper bound up to a small constant factor.


                       Theorem 20.7 Consider an instance G of the load balancing game with n tasks
                       of weight w1 , . . . , wn and m machines of speed s1 , . . . , sm . Let A : [n] → [m]
                       denote any Nash equilibrium assignment. Then, it holds that

                                                                        log m
                                                       cost(A) = O                    · opt(G).
                                                                      log log m

                       proof Let c = cost(A)/opt(G) . We show c ≤  −1 (m), where  −1 denotes
                       the inverse of the gamma function, an extension of the factorial function with
                       the property that (k) = (k − 1)!, for every positive integer k. This yields the
                       theorem as

                                                                             log m
                                                           −1 (m) =                .
                                                                           log log m

                          Without loss of generality, let us assume s1 ≥ s2 ≥ · · · ≥ sm , and let L =
                       [1, 2, . . . , m] denote the list of machines in nonincreasing order of speed. For
                       k ∈ {0, . . . , c − 1}, let Lk denote the maximum length prefix of L such that the
                       load of each server in Lk is at least k · opt(G). Figure 20.2(a) illustrates this
                       definition. We will show the following recurrence on the lengths of these lists.

                                                 |Lk | ≥ (k + 1) · |Lk+1 |          (0 ≤ k ≤ c − 2)
                                               |Lc−1 | ≥ 1
P1: SBT
9780521872829main     CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:34




                    526                            selfish load balancing

                      Solving the recurrence yields |L0 | ≥ (c − 1)! = (c). Now observe that L0 = L
                      and, hence, |L0 | = m. Consequently, m ≥ (c) so that c ≤  −1 (m), which proves
                      the theorem.
                         It remains to prove the recurrence. We first prove |Lc−1 | ≥ 1. For the purpose
                      of a contradiction, assume that the list Lc−1 is empty. Then the load of machine
                      1 is less than (c − 1) · opt(G) in the equilibrium assignment A. Let i be a task
                      placed on a machine j with load at least c · opt(G). Moving i to machine 1 reduces
                      the cost of i to strictly less than
                                                   wi
                              (c − 1) · opt(G) +      ≤ (c − 1) · opt(G) + opt(G) ≤ c · opt(G),
                                                   s1
                      where the inequality ws1i ≤ opt(G) follows from the fact that s1 is the speed of the
                      fastest machine. Consequently, agent i is able to unilaterally decrease its cost by
                      moving its task from machine j to machine 1, which contradicts the assumption
                      that A is a Nash equilibrium. Thus, we have shown that |Lc−1 | ≥ 1.
                         Next, we show |Lk | ≥ (k + 1) · |Lk+1 |, for 0 ≤ k ≤ c − 2. Let A∗ be an optimal
                      assignment, i.e., an assignment whose makespan is equal to opt(G). The following
                      lemma relates the placement of tasks in the equilibrium assignment A to the
                      placement of tasks in the optimal assignment A∗ .

                      Lemma 20.8       Suppose i is a task with A(i) ∈ Lk+1 . Then A∗ (i) ∈ Lk .

                      proof If L \ Lk = ∅ then this claim follows trivially. Let q be the smallest
                      index in L \ Lk , i.e., machine q is one of the machines with maximum speed
                      among the machines L \ Lk . By the definition of the group Lk , the load of q is
                      less than k · opt(G), i.e., q < k · opt(G). Figure 20.2(b) illustrates the situation.
                         By the definition of the groups, A(i) ∈ Lk+1 implies A(i) ≥ (k + 1) · opt(G).
                      For the purpose of a contraction, assume wi ≤ sq · opt(G). Then moving task i to
                      machine q would reduce the cost of i to
                                                  wi
                                           q +      < k · opt(G) + opt(G) ≤ A(i) ,
                                                  sq
                      which contradicts the assumption that A is a Nash equilibrium. Hence, every
                      task i with A(i) ∈ Lk+1 satisfies wi > sq · opt(G). Now, for the purpose of a
                      contradiction, suppose A∗ (i) = j and j ∈ L \ Lk . Then the load on j under A∗
                      would be at least
                                                   wi   sq · opt(G)
                                                      >             ≥ opt(G)
                                                   sj        sj
                      because sj ≤ sq . However, this contradicts that A∗ is an optimal assignment.
                      Consequently, A∗ (i) ∈ Lk .

                         By the definition of Lk+1 , the sum of the weights that A assigns to a machine
                      j ∈ Lk+1 is at least (k + 1) · opt(G) · sj . Hence, the total weight assigned to the
                      machines in Lk+1 is at least j ∈Lk+1 (k + 1) · opt(G) · sj . By Lemma 20.8 an
                      optimal assignment has to assign all this weight to the machines in Lk such that
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0         July 5, 2007     14:34




                                 pure equilibria for uniformly related machines                             527

                      the load on each of these machines is at most opt(G). As a consequence,
                                                                     
                                              (k + 1) · opt(G) · sj ≤    opt(G) · sj .
                                         j ∈Lk+1                                j ∈Lk

                      Dividing by opt(G) and subtracting j ∈Lk+1 sj from both sides yields
                                                               
                                                      k · sj ≤         sj .
                                                      j ∈Lk+1             j ∈Lk \Lk+1

                      Now let s ∗ denote the speed of the slowest machine in Lk+1 , i.e., s ∗ = s|Lk+1 | . For
                      all j ∈ Lk+1 , sj ≥ s ∗ , and, for all j ∈ Lk \ Lk+1 , sj ≤ s ∗ . Hence, we obtain
                                                                     
                                                            k · s∗ ≤          s∗,
                                                      j ∈Lk+1             j ∈Lk \Lk+1

                      which implies |Lk+1 | · k ≤ |Lk \ Lk+1 | = |Lk | − |Lk+1 |. Thus, |Lk | ≥ (k + 1) ·
                      |Lk+1 |. This completes the proof of Theorem 20.7.

                       We now prove a lower bound showing that the upper bound on the price of anarchy
                    given in Theorem 20.7 is essentially tight.

                      Theorem 20.9 For every m ∈ N, there exists an instance G of the load bal-
                      ancing game with m machines and n ≤ m tasks that has a Nash equilibrium
                      assignment A : [n] → [m] with
                                                                      log m
                                              cost(A) =                            · opt(G).
                                                                    log log m

                      proof Recall the definition of the gamma function from the proof of Theo-
                      rem 20.7. We describe a game instance G together with an equilibrium assignment
                      A satisfying
                                                        1  −1              
                                         cost(A) ≥        ·  (m) − 2 − o(1) · opt(G),
                                                        2
                      which yields the theorem.
                         Our construction uses q + 1 disjoint groups of machines denoted G0 , . . . , Gq
                      with q ≈  −1 (m). More, precisely, we set
                                         q =  −1 (m/3) − 1 ≥  −1 (m) − 2 − o(1).
                      For 0 ≤ k ≤ q, group Gk consists of q!/k! machines of speed 2k each of which
                      is assigned k tasks of weight 2k . Let us remark that 0! = 1. The total number of
                      machines in these groups is thus
                                           
                                           q
                                                                 
                                                                 q
                                                                   1
                                                   |Gk | = q!             ≤ 3 (q + 1) ≤ m
                                           k=0                   k=0
                                                                     k!
                                  q
                                  k=0 k! ≤ 3 and 3(q + 1) ≤ m, which follows directly from the defi-
                                      1
                      because
                      nition of q. As m might be larger than the number of the machines in the groups,
                      there might be some machines that do not belong to any of the groups. We assume
P1: SBT
9780521872829main      CUNY1061-Nisan    0 521 87282 0     July 5, 2007   14:34




                    528                            selfish load balancing

                      that these machines have the same parameters as the machines in group G0 ; i.e.,
                      they have speed 20 = 1 and A does not assign a tasks to them.
                         We need to show that the described assignment is a Nash equilibrium. An agent
                      with a task on a machine from group Gk has cost k. It can neither reduce its cost
                      by moving its task to a machine in group Gj with j ≥ k as these machines have
                      at least a load of k, nor can it reduce its cost by moving its task to a machine in
                      group Gj with j < k as the load on such a machine, after the task moved to this
                      machine, would be
                                             2k
                                        j+      = j + 2k−j ≥ j + (k − j + 1) = k + 1
                                             2j
                      since 2t ≥ t + 1, for every t ≥ 1. Hence, none of the agents can unilaterally
                      decrease its cost. In other words, A is a Nash equilibrium.
                         The social cost of the equilibrium assignment A is q. Next we show that
                      opt(G) ≤ 2 so that the theorem follows. We construct an assignment with load at
                      most 2 on every machine. For each k ∈ {1, . . . , q}, the tasks mapped by A to the
                      machines in group Gk are now assigned to the machines in group Gk−1 . Observe
                      that the total number of tasks that A maps to the machines in Gk is
                                                               q!      q!
                                             k · |Gk | = k ·      =          = |Gk−1 |.
                                                               k!   (k − 1)!
                      Hence, we can assign the tasks in such a way that each machine in group Gk−1
                      receives exactly one of the tasks that A mapped to a machine in group Gk . This
                      task has a weight of 2k and the speed of the machine is 2k−1 . Hence, the load of
                      each machine in this assignment is at most 2, which completes the proof.


                                   20.3.2 Algorithms for Computing Pure Equilibria
                    The proof of Proposition 20.3 reveals that, starting from any initial assignment, a pure
                    Nash equilibrium is reached after a finite number of improvement steps. Theorem 20.6
                    shows that there exists a sequence of improvement steps of length O(n) in case of
                    identical machines and this sequence can be computed efficiently. However, in the case
                    of uniformly related machines, it is not known whether there always exists a short se-
                    quence of improvement steps and whether such a sequence can be efficiently computed
                    like in the case of identical machines. However, the well-known LPT (largest process-
                    ing time) scheduling algorithm allows us to efficiently compute a Nash equilibrium.
                    This algorithm inserts the tasks in a nonincreasing order of weights, assigning each
                    task to a machine that minimizes the cost of the task at its insertion time.

                      Theorem 20.10 The LPT algorithm computes a pure Nash equilibrium for load
                      balancing games on uniformly related machines.

                      proof Let the tasks be numbered from 1 to n in the order of their insertion.
                      Let time t ∈ {0, . . . , n} denote the point of time after the first t tasks have been
                      inserted. We show by an induction that the partial assignment A : [t] → [m]
                      computed by LPT at time t is a Nash equilibrium. By our induction assumption
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:34




                                        mixed equilibria on identical machines                             529

                      the tasks 1, . . . , t − 1 are satisfied at time t − 1; i.e., none of these tasks can
                      improve its cost by a unilateral deviation. When task t is inserted, it might be
                      mapped to a machine j ∗ ∈ [m] that holds already some other tasks. We only have
                      to show that these tasks do not get unsatisfied because of the increased load on
                      j ∗ because of the assignment of task t. Let i < t be one of the tasks mapped to
                      machine j ∗ . For j ∈ [m], let j denote the load on machine j at time t. Since the
                      assignment of task t to machine j ∗ minimizes the cost of agent t and as wt ≤ wi ,
                                                   j ∗   j + wt   j + wi
                                                        ≤         ≤         ,
                                                   sj ∗      sj        sj
                      for all j ∈ [m]. Hence, also at time t, agent i is satisfied on machine j ∗ as it
                      cannot reduce its cost by moving from j ∗ to another machine.

                        The assignment computed by the LPT algorithm is not only a Nash equilibrium but
                    it also approximates the optimal makespan within a ratio of at most 53 for uniformly
                    related machines and 43 − 3m  1
                                                    for identical machines, see Friesen (1987) and Graham
                    (1966), respectively. As makespan scheduling is NP-hard even on identical machines,
                    one cannot hope for an efficient algorithm that computes an assignment with optimal
                    makespan, unless P = NP. However, the polynomial time approximation scheme of
                    Hochbaum and Shmoys (1988) computes an assignment of tasks to uniformly related
                    machines minimizing the makespan within a ratio of (1 + ), for any given  > 0. This
                    assignment is not necessarily a Nash equilibrium. Feldmann et al. (2003a) present an
                    efficient algorithm that transforms any given assignment into an equilibrium assignment
                    without increasing the makespan. This approach is called Nashification. Combining
                    the polynomial time approximation scheme with the Nashification approach yields a
                    polynomial time algorithm that computes an equilibrium assignment for scheduling on
                    uniformly related machines minimizing the makespan within a factor of (1 + ), for
                    any given  > 0.


                                    20.4 Mixed Equilibria on Identical Machines

                    The example with two identical machines presented in Section 20.1.2 shows that the
                    social cost can increase if players make use of randomization. Let us now study this
                    effect systematically. We analyze by how much the price of anarchy is increased
                    when the set of strategies is extended from pure to mixed strategies. First, we con-
                    sider an extreme case of randomization in which every agent randomizes over all
                    strategies.

                                               20.4.1 Fully Mixed Equilibria
                    The support of an agent is the set of strategies to which the agent assigns positive
                    probability. In a fully mixed strategy profile all pure strategies are in the support of
                    every agent. There is exactly one fully mixed strategy profile for load balancing games
                    on identical machines i.e. a Nash equilibrium. In this fully mixed Nash equilibrium every
                                                                                                       j
                    player assigns every task with probability m1 to each of the machines, i.e., P = (pi ) with
                      j
                    pi = m1 , for every i ∈ [n] and j ∈ [m]. The fully mixed Nash equilibrium maximizes
P1: SBT
9780521872829main      CUNY1061-Nisan    0 521 87282 0   July 5, 2007       14:34




                    530                            selfish load balancing

                    the randomization and, hence, seems to be a good candidate to study the effects of
                    randomization.
                       Our analysis begins with a particularly simple class of load balancing games: Sup-
                    pose that we have not only identical machines but also identical tasks. That is, we
                    assume that there are m machines of speed 1 and n tasks of weight 1. In the unique
                    fully mixed Nash equilibrium for such a game, each task is assigned to each machine
                                                                                                            j
                    with probability m1 . This strategy profile is a Nash equilibrium as the expected cost ci
                    of any task i on any machine j is the same. In particular, Equation 20.1 yields

                                               j                        1           1
                                              ci = E[j ] + 1 −              =2−      .
                                                                        m           m
                    This setup corresponds to a well-studied balls-and-bins experiment from probability
                    theory in which n balls are assigned independently, uniformly at random to m bins,
                    which is also discussed in Chapter 17. How bad is such a fully mixed Nash equilibrium
                    in comparison to an optimal assignment that distributes the tasks evenly among the ma-
                    chines? An optimal assignment minimizes the makespan, and the optimal makespan is
                    obviously mn . The expected makespan of the fully mixed strategy profile corresponds
                    to the expected maximum occupancy of the corresponding balls-and-bins experiment,
                    i.e., the expected number of balls in the fullest bin. The following proposition yields a
                    simple formula for this quantity that is exact up to constant factors for any choice of m
                    and n.

                      Proposition 20.11 Suppose that n ≥ 1 balls are placed independently, uni-
                      formly at random into m ≥ 1 bins. Then the expected maximum occupancy is
                                                                     
                                                           ln m
                                                                   .
                                                     ln 1 + mn ln m

                       Let us illustrate the formula for the expected maximum occupancy given in the
                    proposition with a few examples. If n ≥ m log m, then the expected
                                                                                         maximum occu-
                    pancy is ( mn ) as, in this case, ln 1 + mn ln m =  mn ln m . If n ≤ m1− , for any
                    fixed  > 0, then the expected maximum occupancy is  (1). Observe, in both of
                    these cases, the ratio between the expected makespan for the fully mixed equilib-
                    rium and the makespan of an optimal assignment is O(1). It turns out that this ratio
                    is maximized when setting m = n. In this case, the expected maximum occupancy
                    is  (log m/ log log m) while the optimal makespan is 1. This yields the following
                    result.

                      Theorem 20.12 For every m ∈ N, there exists an instance G of a load balancing
                      game with m identical machines and n = m tasks that has a Nash equilibrium
                      strategy profile P with
                                                                log m
                                             cost(P ) =                       · opt(G).
                                                              log log m
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007     14:34




                                        mixed equilibria on identical machines                             531

                        As the fully mixed Nash equilibrium is the equilibrium that maximizes the ran-
                    domization, one could guess that this is also the equilibrium that maximizes the ratio
                    between the expected makespan and the optimal makespan for load balancing games.
                    This guess is known as the so-called fully mixed Nash equilibrium conjecture. This
                    conjecture is appealing as it would yield a simple characterization of the worst-case
                    Nash equilibrium for load balancing games. Unfortunately, however, the conjecture is
                    wrong. With the help of Proposition 20.11, we can easily construct a counterexample.
                                                               √
                    Let m = 22k , for some k ∈ N. This way, m as well as log m are integers. Now consider
                    the following instance of the load balancing game on m identical machines. Suppose
                                    √                                       √
                    that there are m large tasks of weight 1, and (m − m) · log m small tasks of weight
                      1
                    log m
                          . The balls-and-bins analysis above shows that the maximum number of large tasks
                    that are assigned to the same machine by a fully mixed Nash equilibrium is O(1), and
                    the maximum number of small tasks assigned to the same machine is O(log m). Hence,
                    the expected makespan of the fully mixed Nash equilibrium is O(1). Now consider
                    the following strategy profile: Assign the large tasks uniformly at random to the first
                    √
                       m machines (called group A) and the small tasks uniformly at random to the other
                    machines (called group B). This profile is a Nash equilibrium as Equation 20.1 yields
                    that, for a large task, the expected cost on a machine of group A is less than the expected
                    cost on a machine of group B and, for a small task, the expected cost on a machine of
                    group B is less than the expected cost on a machine of group A. In this equilibrium,
                    the expected maximum occupancy among the large tasks is ( logloglogmm ), which shows
                    that there is a mixed Nash equilibrium whose expected makespan is larger than the
                    expected makespan of the fully mixed Nash equilibrium by a factor of ( logloglogmm ).


                                                   20.4.2 Price of Anarchy
                    The fully mixed Nash equilibrium is not necessarily the worst-case Nash equilibrium
                    for every instance of the load balancing game on identical machines. Nevertheless, the
                    following analysis shows that the lower bound on the price of anarchy that we obtained
                    from studying this kind of equilibria is tight.

                      Theorem 20.13 Consider an instance G of the load balancing game with n
                                                                                          j
                      tasks of weight w1 , . . . , wn and m identical machines. Let P = (pi )i∈[n],j ∈[m]
                      denote any Nash equilibrium strategy profile. Then, it holds that
                                                                 log m
                                              cost(P ) = O                   · opt(G).
                                                               log log m

                      proof Without loss of generality, we assume that all machines have speed 1.
                      Recall that cost(P ) = E[maxj ∈[m] (j )], i.e., cost(P ) corresponds to the expected
                      maximum load over all machines or, in other words, the expected makespan.
                      Our analysis starts with proving an upper bound on the maximum expected load
                      instead of the expected maximum load.
                         We claim that, for every j ∈ [m], E[j ] ≤ (2 − m+1     2
                                                                                    ) opt(G). The proof for
                      this claim follows the course of the analysis for the upper bound on the price of
                      anarchy for pure equilibria. More specifically, the proof of Theorem 20.5 can be
P1: SBT
9780521872829main     CUNY1061-Nisan        0 521 87282 0        July 5, 2007         14:34




                    532                              selfish load balancing

                      adapted as follows to mixed equilibria: Instead of considering a smallest weight
                      task i ∗ placed on a maximum load machine j ∗ , one defines i ∗ to be the smallest
                      weight task with positive probability on a machine j ∗ maximizing the expected
                      load. Also in all other occurrences one considers the expected load instead of the
                      load.
                          We conclude that the maximum expected load is less than 2 opt(G). Next we
                      show that the expected maximum load deviates at most by a factor of O( logloglogmm )
                      from the maximum expected load. We use a weighted Chernoff bound in order to
                      show that it is unlikely that there is a machine that deviates by a large factor from
                      its expectation.


                      Lemma 20.14 (weighted Chernoff bound) Let X1 , . . . , XN be independent
                      random variables with values in the interval [0, z] for some z > 0, and let
                      X= N  i=1 Xi , then for any t it holds that P[ i=1 Xi ≥ t] ≤ (e · E[X] / t) .
                                                                     N                           t/z




                      A description how to derive this and other variants of the Chernoff bound can be
                      found, e.g., in Mitzenmacher and Upfal (2005).
                         Fix j ∈ [m]. Let w denote the largest weight of any task. Applying the weighted
                      Chernoff bound shows that, for every t,
                                                                                    
                                                 e · E[j ]                 t/w
                                                                                                2 e opt(G)      t/opt(G)
                              P[j ≥ t] ≤ min 1,                                         ≤                                 .
                                                      t                                              t

                      because E[j ] ≤ 2 opt(G) and w ≤ opt(G). Now let τ = 2 opt(G) lnlnlnmm . Then,
                      for any x ≥ 0,
                                                                                     2 ln m/ ln ln m+x/opt(G)
                                                                     e ln ln m
                                       P[j ≥ τ + x] ≤
                                                                        ln m
                                                                         2 ln m/ ln ln m
                                                                   1
                                                             ≤  √                        · e−x/opt(G)
                                                                  ln m
                                                             = m−1 · e−x/opt(G) ,

                      where the second inequality holds asymptotically as, for sufficiently large m,
                                 √
                         ln m
                      e ln ln m
                                ≥ log m and e lnlnlnmm ≥ e.
                           Now with the help of the tail bound we can upper-bound
                                                                        ∞        cost(P ) as follows. For
                      every nonnegative random variable X, E[X] = 0 P[X ≥ t]dt. Consequently,
                                                                           ∞ 
                                     cost(P ) = E max j                 =    P max j ≥ t dt .
                                                            j ∈[m]               0           j ∈[m]


                      Substituting t by τ + x and then applying the union bound yields
                                      ∞                          ∞
                      cost(P ) ≤ τ +    P max j ≥ τ + x dx ≤ τ +      P[j ≥ τ + x] dx .
                                        0        j ∈[m]                                          0    j ∈[m]
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0        July 5, 2007     14:34




                                  mixed equilibria on uniformly related machines                          533

                      Finally, we apply the tail bound derived above and obtain
                                                       ∞
                                     cost(P ) ≤ τ +        e−x/opt(G) dx = τ + opt(G) ,
                                                              0

                      which yields the theorem as τ = 2 opt(G) lnlnlnmm .


                             20.5 Mixed Equilibria on Uniformly Related Machines

                    Finally, we come to the most general case, namely mixed equilibria on uniformly related
                    machines. The following theorem shows that the price of anarchy for this case is only
                    slightly larger than the one for mixed equilibria on identical machines or pure equilibria
                    on uniformly related machines. The analysis combines the methods from both of these
                    more restricted cases: First, we show that the maximum expected makespan is bounded
                    by
                                                                log m
                                                    O                         · opt(G)
                                                              log log m
                    using the same kind of arguments as in the analysis of the price of anarchy for pure
                    equilibria on uniformly related machines. Then, as in the case of mixed equilibria on
                    identical machines, we use a Chernoff bound to show that the expected maximum load
                    is not much larger than the maximum expected load. In fact, this last step loses only a
                    factor of order log log m/ log log log m, which results in an upper bound on the price
                    of anarchy of
                                                                      log m
                                                          O                     .
                                                                  log log log m
                    After proving this upper bound, we present a corresponding lower bound by adding
                    some randomization to the lower bound construction for pure equilibria on uni-
                    formly related machines, which increases also the lower bound by a factor of order
                    log log m/ log log log m and, hence, yields a tight result about the price of anarchy.

                      Theorem 20.15 Consider an instance G of the load balancing game with n
                      tasks of weight w1 , . . . , wn and m machines of speed s1 , . . . , sm . Let P be any
                      Nash equilibrium strategy profile. Then, it holds that
                                                                       log m
                                            cost(P ) = O                            · opt(G).
                                                                   log log log m

                      proof As in the case of identical machines, our analysis starts with proving an
                      upper bound on the maximum expected load instead of the expected maximum
                      load. To simplify the notation, we assume opt(G)
                                                                       = 1, whichcan be achieved by
                      scaling the weights appropriately. Let c = maxj ∈[m] E[j ] . We first prove an
                      upper bound on c following the analysis for pure Nash equilibria in Theorem 20.7.
                      Without loss of generality, assume s1 ≥ s2 ≥ · · · ≥ sm . Let L = [1, 2, . . . , m]
                      denote the list of machines in non increasing order of speed. For k ∈ {0, . . . , c −
                      1}, let Lk denote the maximum length prefix of L such that the expected load
P1: SBT
9780521872829main     CUNY1061-Nisan      0 521 87282 0       July 5, 2007           14:34




                    534                                 selfish load balancing

                      of each server in Lk is at least k. Analogously to the analysis in the proof of
                      Theorem 20.7, one shows the recurrence |Lk | ≥ (k + 1) · |Lk+1 |, for 0 ≤ k ≤ c −
                      2, and |Lc−1 | ≥ 1. Solving the recurrence yields |L0 | ≥ (c − 1)! = (c). Thus,
                      |L0 | = m implies c ≤  −1 (m) =  (ln m/ ln ln m). Now let
                                                                 
                                                            ln m            ln m
                                        C = max c + 1,              =              .
                                                          ln ln m          ln ln m

                      In the rest of the proof, we show that the expected makespan of the equilibrium
                      assignment can exceed C at most by a factor of order ln ln m/ ln ln ln m so that
                      the expected makespan is O(ln m/ ln ln ln m), which proves the theorem as we
                      assume opt(G) = 1.
                         As the next step, we prove a tail bound on j , for any fixed j ∈ [m] and, after-
                      ward, we use this tail bound to derive an upper bound on the expected makespan.
                                                                                                j
                      For a machine j ∈ [m], let Tj(1) denote the set of tasks i with pi ≥ 14 and Tj(2)
                                               j
                      the set of tasks i with pi ∈ (0, 14 ). Let j(1) and j(2) denote random variables that
                      describe the load on link j only taking into account the tasks in Tj(1) and Tj(2) ,
                      respectively. Observe that j = j(1) + j(2) . For the tasks in Tj(1) , we immediately
                      obtain
                                                  wi              wi p j
                                       j(1) ≤               ≤4                  i
                                                                                     = 4 E[j(1) ] ≤ 4C.   (20.2)
                                                    (1)
                                                        sj                  sj
                                                 i∈Tj             i∈Tj(1)


                      To prove an upper bound on j(2) , we use the weighted Chernoff bound from
                      Lemma 20.14. This bound requires an upper bound on the maximum weight.
                      As a first step to bound the weights, we prove a result about the relationship
                      between the speeds of the machines in the different groups that are defined by
                      the prefixes. For 0 ≤ k ≤ c − 2, let Gk = Lk \ Lk+1 , and let Gc−1 = Lc−1 . For
                      0 ≤ k ≤ c − 1, let s(k) denote the speed of the fastest machine in Gk . Clearly,
                      s(c − 1) ≥ s(c − 2) ≥ · · · ≥ s(1) ≥ s(0). We claim that this sequence is, in fact,
                      geometrically decreasing.

                      Lemma 20.16        For 0 ≤ k ≤ c − 4, s(k + 2) ≥ 2 s(k).

                      proof To prove the claim, we first observe that there exists a task j ∗ with
                      wj ∗ ≤ s(k + 2) that has positive probability on a machine in Lk+3 . This is because
                      an optimal assignment strategy has to move some of the expected load from the
                      machines in Lk+3 to machines in L \ Lk+3 and it can only assign those tasks
                      to machines in L \ Lk+3 whose weights are not larger than the maximum speed
                      among this set of machines, which is s(k + 2). Now suppose s(k) > 12 s(k + 2).
                      The expected load of the fastest machine in Gk = Lk \ Lk+1 is at most k + 1.
                      Thus the expected cost of j ∗ on the fastest machine in Gk is at most

                                                        wj ∗         2wj ∗
                                          k+1+               <k+1+          ≤ k + 3.
                                                        s(k)       s(k + 2)
P1: SBT
9780521872829main   CUNY1061-Nisan      0 521 87282 0    July 5, 2007      14:34




                               mixed equilibria on uniformly related machines                                535

                    This contradicts that the expected cost of j ∗ in the considered Nash equilib-
                    rium is at least k + 3 as it has positive probability on a machine in Lk+3 . Thus,
                    Lemma 20.16 is shown.

                       Now we apply Lemma 20.16 to prove an upper bound on the weights of the
                    tasks in the set Tj(2) .


                    Lemma 20.17        For every j ∈ [m] and i ∈ Tj(2) , wi ≤ 12 sj .

                                                                j
                    proof Let i be a task from Tj(2) , i.e., pi ∈ (0, 14 ). Let j ∈ Gk , for 0 ≤ k ≤ c − 1.
                    The expected cost of i on j is
                                                              w       3wi
                                          j                 j     i
                                         ci = E[j ] + 1 − pi       ≥k+     .
                                                                sj      4sj
                                                                                         j
                    Suppose that k ≥ c − 3. In this case, wi > 12 sj implies ci > k + 34 · 12 ≥ c + 6,
                    which contradicts that, under the Nash equilibrium profile, the expected cost of
                    any task on the fastest machine is at most c + 1. Hence, the lemma is shown for
                    k ≥ c − 3. Now suppose k ≤ c − 4. Let q denote the fastest machine from Gk+2 .
                    Lemma 20.16 yields sq = s(k + 2) ≥ 2s(k) ≥ 2 sj . Hence, the expected cost of i
                    on q is
                                        q                q  wi       wi
                                       ci = E[q ] + 1 − pi      ≤k+3+     .
                                                              sq       2sj
                         j                                                         j     q
                    As pi > 0, the Nash equilibrium condition yields ci ≤ ci . Consequently,
                                                        3wi       wi
                                                  k+        ≤k+3+     ,
                                                        4sj       2sj
                    which implies wi ≤ 12 sj and, hence, completes the proof of Lemma 20.17.

                      Let z = maxi∈T (2) (wi /sj ). Lemma 20.17 implies z ≤ 12. Now applying the
                                     j
                    weighted Chernoff bound from Lemma 20.14 yields that, for every α > 0,
                                                                          αC/z
                                                          e · E[j(2) ]                 e αC/12
                                     P[j(2) ≥ αC] ≤                               ≤
                                                              αC                        α

                    since E[j(2) ] ≤ C. We define τ = 24 C ln ln m/ ln ln ln m. As C is of order
                    ln m/ ln ln m, it follows that τ is of order ln m/ ln ln ln m. Let x ≥ 0. We sub-
                    stitute τ + x for αC and obtain
                                                                    (τ +x)/12
                                                          eC
                                 P[j(2) ≥ τ + x] ≤
                                                         τ +x
                                                                           2C ln ln m/ ln ln ln m+x/12
                                                         e ln ln ln m
                                                    ≤                                                    .
                                                         24 ln ln m
P1: SBT
9780521872829main      CUNY1061-Nisan   0 521 87282 0     July 5, 2007          14:34




                    536                              selfish load balancing
                                                                                √
                      Observe that 24 ln ln m/(e ln ln ln m) is lower-bounded by ln ln m and also
                      lower-bounded by e2 . Furthermore, C ≥ ln m/ ln ln m. Applying these bounds
                      yields
                                                                     2 ln m/ ln ln ln m
                                                         1
                             P[j(2) ≥ τ + x] ≤       √                                   · e−x/6 = m−1 · e−x/6 .
                                                       ln ln m
                      As a consequence,
                                                   ∞ 
                                    E max j =(2)
                                                      P max j(2) ≥ t dt
                                       j ∈[m]       0    j ∈[m]
                                                       ∞ 
                                                  ≤τ+       P max j(2) ≥ τ + x dx
                                                       0       j ∈[m]
                                                       ∞ 
                                                  ≤τ+             P[j(2) ≥ τ + x]dx.
                                                                 0     j ∈[m]

                      Now applying our tail bound yields
                                                         ∞
                                       E max j ≤ τ +
                                                (2)
                                                             e−x/6 dx = τ + 6.                                      (20.3)
                                            j ∈[m]                     0

                      Finally, we combine Equations 20.2 and 20.3 and obtain
                                                                              log m
                                 cost(P ) = E max j ≤ 4 C + τ + 6 = O(                  ),
                                              j ∈[m]                       log log log m
                      which completes the proof of Theorem 20.15.

                       Next we show that the upper bound given in Theorem 20.15 is tight by showing
                    that for every number of machines there exists a game instance that matches the upper
                    bound up to a constant factor.

                      Theorem 20.18 For every m ∈ N, there exists an instance G of the load bal-
                      ancing game with m machines and n ≤ m tasks that has a Nash equilibrium
                      strategy profile P with
                                                                 log m
                                          cost(P ) =                                     · opt(G).
                                                             log log log m

                      proof The starting point for our construction is the game and the Nash as-
                      signment A from the proof of Theorem 20.9. We use mixed strategies in only
                      one of the groups, namely in the group Gk with k = q/2. Let M denote the
                      number of machines in this group, i.e., M = q!/k! ≥ (q/2) q/2 . Observe that
                      log M = (q log q) = (log m).
                         Let T denote the set of tasks mapped by A to one of the machines in Gk .
                      The tasks in T have weight 2k . Each of these tasks is now assigned uniformly at
                                                               j
                      random to a machine group Gk , i.e., pi = M1 , for each j ∈ Gk and each i ∈ T .
                      For all other tasks the strategy profile P corresponds without any change to the
                      pure strategy profile of assignment A. Observe that the randomization increases
                      the expected cost of the tasks. The expected cost of a task i ∈ T on a machine
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0       July 5, 2007   14:34




                                                  summary and discussion                                  537

                      j ∈ Gk is now
                                                        w          1
                                    j                 j     i
                                   ci = E[j ] + 1 − pi       =k+ 1−                        < k + 1.
                                                          sj         M
                      In the proof of Theorem 20.9, we have shown that the cost of a task i of weight 2k
                      on a machine of group Gj with j = k is at least k + 1. Thus, the strategy profile
                      P is a Nash equilibrium.
                         It remains to compare the social cost of the equilibrium profile P with the
                      optimal cost. The structure of the optimal assignment is not affected by the
                      modifications. It has social cost opt(G) = 2. Now we give a lower bound for
                      the social cost of P . This social cost is, obviously, bounded from below by the
                      maximum number of tasks that are mapped to the same machine in the group
                      Gk . Applying Proposition 20.11 with M bins and N = kM balls shows that the
                      expected makespan is
                                                             
                                                  ln M                     log m
                                                           =                      ,
                                             ln 1 + k ln M
                                                     1                 log log log m

                      where the last estimate holds as k =  (log m/ log log m) and log M = (log m).
                      This completes the proof of Theorem 20.18.


                                             20.6 Summary and Discussion

                    In this chapter, we studied the price of anarchy in load balancing games in four different
                    variants. Table 20.1 summarizes the results about the price of anarchy that we have
                    presented. In the case of pure equilibria on identical machines, the price of anarchy is
                    bounded from above by a small constant term. In all other cases, the price of anarchy
                    is bounded from above by a slowly growing, sublogarithmic function in the number of
                    machines. One might interpret these results as a first game theoretic explanation why
                    the resources in a large distributed system like the Internet that widely lacks global
                    control are shared in a more or less efficient and fair way among different users with
                    different interests, although the considered model is clearly oversimplifying in several
                    aspects.
                       It is an interesting coincidence that both the price of anarchy for pure equilibria
                    on uniformly related machines as well as the price of anarchy for mixed equilibria


                                         Table 20.1. The price of anarchy for pure and
                                         mixed equilibria in load balancing games on
                                         identical and uniformly related machines

                                                          Identical          Uniformly related
                                                                                          
                                          Pure            2 − m+1
                                                                2
                                                                              logloglogmm
                                                                                            
                                          Mixed            logloglogmm             log m
                                                                              log log   log m
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:34




                    538                            selfish load balancing

                    on identical machines are of order log m/ log log m. Although both of these models
                    result in essentially the same price of anarchy, the reasons for the increase in the social
                    cost are quite different: In the case of pure equilibria on uniformly related machines,
                    equilibrium assignments correspond to local optima with respect to moves of single
                    tasks. That is, tasks are placed in a suboptimal but nevertheless coordinated fashion.
                    On the contrary, in case of mixed equilibria, the increase in cost is due to collisions
                    between uncoordinated random decisions. If one combines these two effects, then one
                    loses only another very small factor of order log log m/ log log log m, which results
                    in a price of anarchy of order log m/ log log log m for mixed equilibria on uniformly
                    related machines.
                        Obviously, the price of anarchy for load balancing games as we have defined them
                    in the beginning of this chapter is well understood. As mentioned above, however, this
                    model is very simplistic. To make these results more realistic, one needs to incorporate
                    other aspects from practical application areas like, e.g., more realistic cost functions or
                    other ways to define the social cost. We give pointers to studies of quite a few variants
                    of load balancing games in the bibliographic notes. In Christodoulou et al. (2004), it
                    is made an interesting attempt that adds an algorithmic or constructive element to the
                    analysis of the price of anarchy. The idea behind so-called “coordination mechanisms”
                    is not to study the price of anarchy for a fixed system, but to design the system in such
                    a way that the increase in cost or the loss in performance due to selfish behavior is as
                    small as possible. Similar aspects are also discussed in Chapter 17. We believe that
                    this is a promising direction of research that might result in practical guidelines of how
                    to build a distributed system that does not suffer from selfish behavior but might even
                    exploit the selfishness of the agents.
                        Besides the price of anarchy, we have studied the question of how agents reach a
                    Nash equilibrium. We have observed that any sequence of improvement steps reaches
                    a pure Nash equilibrium after a finite number of steps. In case of identical machines
                    the max-weight best-response policy reaches an equilibrium in only O(n). In case of
                    uniformly related machines, it is open whether there exists a short sequence of im-
                    provement steps that lead from any given assignment to a pure Nash equilibrium. We
                    think that this question is of great importance as Nash equilibria are only of interest
                    if agents can reach them quickly. It is not clear that the only reasonable approach for
                    the agents to reach a Nash equilibrium in a distributed way is to use improvement
                    steps. There might also be other, possibly more strategic or more coordinated behav-
                    ioral rules that quickly converge to a Nash equilibrium or to an approximate Nash
                    equilibrium. For example, Chapter 29 considers some approaches from evolutionary
                    game theory in the context of routing in networks. It is an interesting research prob-
                    lem to design distributed protocols that ensure that agents reach a Nash equilibrium
                    quickly. Pointers to first results toward this direction can be found in the bibliographic
                    notes.


                                                 20.7 Bibliographic Notes

                    The concept of the price of anarchy was introduced by Koutsoupias and Papadimitriou
                    (1999). In their seminal work, they study load balancing in form of a routing game
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:34




                                                    bibliographic notes                                   539

                    consisting of two nodes connected by parallel edges with possibly different speeds.
                    Each agent has an amount of traffic that the agent seeks to map to one of the edges such
                    that the load on this edge is as small as possible. In our notation, the parallel edges
                    between the source and the sink correspond to the machines and the pieces of traffic
                    of the agents correspond to the tasks. Let us remark that originally the ratio between
                    the social cost in a worst-case Nash equilibrium and the optimal social cost was called
                    coordination ratio but in this chapter we switched to the now commonly used term
                    price of anarchy. The game theoretic model underlying the load balancing games is
                    also known as KP model.
                        The results presented in Table 20.1 have been obtained in the following studies.
                    The upper bound of 2 − m+1     2
                                                       on the price of anarchy for pure equilibria in load
                    balancing games with identical machines goes back to the scheduling literature (Finn
                    and Horowitz, 1979), where the same ratio occurs in form of an approximation factor
                    for a local search optimization heuristic. The lower bound on the price of anarchy for
                    mixed equilibria on identical machines is presented in Koutsoupias and Papadimitriou
                    (1999). The analysis for the corresponding upper bound is obtained in Czumaj and
                    Vöcking (2002) and Koutsoupias et al. (2003). Let us remark that the analysis in
                    Czumaj and Vöcking (2002) is tight up to a constant additive term. It shows that the
                    price of anarchy for mixed equilibria in load balancing games on identical machines
                    is  −1 (m) ± (1). The upper and lower bounds on the price of anarchy for pure and
                    mixed equilibria in load balancing games with uniformly related machines are from
                    Czumaj and Vöcking (2002) as well. This work also contains a tight characterization
                    of the price of anarchy as a function of the ratio between the speeds of the fastest and
                    the slowest machine.
                        The existence proof for pure equilibria presented in Section 20.1.1 can be found in
                    Fotakis et al. (2002) and Even-Dar et al. (2003). The result from Section 20.3.2 that
                    the LPT algorithm computes a pure Nash equilibrium is presented in Fotakis et al.
                    (2002) together with several further results about the complexity of computing pure
                    and mixed equilibria in load balancing games. The uniqueness of the fully mixed Nash
                    equilibrium is shown in Mavronicolas and Spirakis (2001). Exercise 20.5 reworks the
                    nice proof for this result. The counterexample to the fully mixed Nash equilibrium
                    conjecture presented in Section 20.4.1 is from Fischer and Vöcking (2005). Finally,
                    the results from Section 20.2.2 about the convergence of best response sequences are
                    from Even-Dar et al. (2003).
                        Let us remark that this chapter does by far not give a complete overview of the rich
                    literature about different variants of games for load balancing or routing on parallel
                    links. We conclude this chapter with a few pointers to further literature. Load balancing
                    games with more general cost functions are considered, e.g., in Caragiannis et al. (2006),
                    Czumaj et al. (2002), Libman and Orda (1999, 2001). Other definitions of the social
                    cost are considered, e.g., in Caragiannis et al. (2006), Gairing et al. (2004a, 2004b) and
                    Suri et al. (2004). Another interesting variant of load balancing games assumes that
                    agents come with subsets of the machines on which they have to place their tasks. The
                    price of anarchy in such a restricted assignment model is investigated in Awerbuch et al.
                    (2003), Gairing et al. (2006), and Suri et al. (2004). The price of anarchy with respect
                    to equilibria that are robust against coalitions is studied in Andelman et al. (2007).
                    An important aspect that we have only touched in this chapter is the complexity of
P1: SBT
9780521872829main      CUNY1061-Nisan        0 521 87282 0     July 5, 2007    14:34




                    540                                selfish load balancing

                    computing Nash equilibria for load balancing games. Further work dealing with the
                    computation of Nash equilibria can be found, e.g., in Even-Dar et al. (2003), Feldmann
                    et al. (2003a), Fotakis et al. (2002), Fischer and Vöcking (2005), and Gairing et al.
                    (2004a). Recent work deals also with the convergence time of distributed load balancing
                    processes in which agents make parallel attempts for improvement steps until they find
                    a Nash equilibrium (Berenbrink et al., 2006; Even-Dar and Mansour, 2005). Another
                    interesting topic is load balancing games with incomplete information that have been
                    considered, e.g., in Beier et al. (2004) and Gairing et al. (2005). Finally, let us remark
                    that the concept of coordination mechanisms has been suggested in Christodoulou
                    et al. (2004) and some further results on this topic can be found in Immorlica et al.
                    (2005).
                       Several other results for load balancing and routing on parallel links have been
                    collected in the surveys (Czumaj, 2004; Feldmann et al., 2003b; Koutsoupias, 2003).



                                                              Bibliography
                    N. Andelman, M. Feldman, and Y. Mansour. Strong price of anarchy. In Proc. 18th Annual ACM-SIAM
                       Symp. on Discrete Algorithms, 2007.
                    B. Awerbuch, Y. Azar, Y. Richter, and D. Tsur. Tradeoffs in worst-case equilibria. In Proc. 1st
                       International Workshop on Approximation and Online Algorithms (WAOA), pp. 41–52, 2003.
                    R. Beier, A. Czumaj, P. Krysta, and B.Vöcking. Computing equilibria for congestion games with
                       (im)perfect information. In Proc. 15th Annual ACM-SIAM Symp. Discrete Algorithms, pp. 746–
                       755, 2004.
                    P. Berenbrink, T. Friedetzky, L.A. Goldberg, P.W. Goldberg, Z. Hu, and R.A. Martin. Distributed
                       selfish load balancing. In Proc. 17th Annual ACM-SIAM Symp. Discrete Algorithms, pp. 354–363,
                       2006.
                    I. Caragiannis, M. Flammini, C. Kaklamanis, P. Kanellopoulos, and L. Moscardelli. Tight bounds
                       for selfish and greedy load balancing. In Proc. 33rd Intl. Colloq. on Automata, Languages, and
                       Programming, pp. 311–322, 2006.
                    G. Christodoulou, E. Koutsoupias, and A. Nanavati. Coordination Mechanisms. In Proc. 31st Intl.
                       Colloq. on Automata, Languages and Programming, pp. 345–357, 2004.
                    A. Czumaj. Selfish Routing on the Internet. Chapter 42 in Handbook of Scheduling: Algorithms,
                       Models, and Performance Analysis, edited by J. Leung, CRC Press, Boca Raton, FL, 2004.
                    A. Czumaj, P. Krysta, and B. Vöcking. Selfish traffic allocation for server farms. In Proc. 34th Annual
                       ACM Symp. Theory of Computing, pp. 287–296, 2002.
                    A. Czumaj and B. Vöcking. Tight bounds for worst-case equilibria. In Proc. 13th Annual ACM-SIAM
                       Symp. on Discrete Algorithms, pp. 413–420, 2002.
                    E. Even-Dar, A. Kesselman, and Y. Mansour. Convergence time to Nash equilibria. In Proc. 30th
                       International Colloq. on Automata, Languages and Programming, pp. 502–513, 2003.
                    E. Even-Dar and Y. Mansour. Fast convergence of selfish rerouting. In Proc. 16th Annual ACM-SIAM
                       Symp. on Discrete Algorithms, pp. 772–781, 2005.
                    R. Feldmann, M. Gairing, T. Lücking, B. Monien, and M. Rode. Nashification and the coordination
                       ratio for a selfish routing game. In Proc. 30th International Colloq. on Automata, Languages and
                       Programming, pp. 414–426, 2003a.
                    R. Feldmann, M. Gairing, T. Lücking, B. Monien, and M. Rode. Selfish routing in non-cooperative
                       networks: a survey. In Proc. 28th International Symp. on Mathematical Foundations of Computer
                       Science, pp. 21–45, 2003b.
P1: SBT
9780521872829main       CUNY1061-Nisan       0 521 87282 0    July 5, 2007   14:34




                                                             bibliography                                          541

                    G. Finn and E. Horowitz. A linear time approximation algorithm for multiprocessor scheduling. BIT,
                       19(3):312–320, 1979.
                    S. Fischer and B. Vöcking. On the structure and complexity of worst-case equilibria. In Proc. 1st
                       Workshop on Internet and Network Economics, pp. 151–160, 2005.
                    D. Fotakis, S. Kontogiannis, E. Koutsoupias, M. Mavronicolas, and P. Spirakis. The structure and
                       complexity of Nash equilibria for a selfish routing game. In Proc. 29th Intl. Colloquium on
                       Automata, Languages and Programming (ICALP), pp. 123–134, 2002.
                    D.K. Friesen. Tighter bounds for LPT scheduling on uniform processors. SIAM J. Computing,
                       16(3):554–560, 1987.
                    M. Gairing, T. Lücking, M. Mavronicolas, and B. Monien. Computing Nash equilibria for scheduling
                       on restricted parallel links. In Proc. 36th Annual ACM Symp. on Theory of Computing, pp. 613–622,
                       2004a.
                    M. Gairing, T. Lücking, M. Mavronicolas, and B. Monien. The price of anarchy for polynomial social
                       cost. In Proc. 29th Intl. Symp. on Mathematical Foundations of Computer Science, pp. 574–585,
                       2004b.
                    M. Gairing, T. Lücking, M. Mavronicolas, B. Monien, and M. Rode. Nash equilibria in discrete
                       routing games with convex latency functions. In Proc. 31st Intl. Colloq. on Automata, Languages
                       and Programming, pp. 645–657, 2004c.
                    M. Gairing, T. Lücking, M. Mavronicolas, and B. Monien. The Price of Anarchy for Restricted
                       Parallel Links. Parallel Process. Lett., 16(1):117–132, 2006.
                    M. Gairing, B. Monien, and K. Tiemann. Selfish routing with incomplete information. In Proc. 17th
                       Annual ACM Symp. on Parallel Algorithms, pp. 203–212, 2005.
                    R.L. Graham. Bounds for certain multiprocessing anomalies. Bell System Tech. J., 45: 1563–1581,
                       1966.
                    R.L. Graham. Bounds on multiprocessing timing anomalies. SIAM J. Appl. Math., 17: 263–269, 1969.
                    D.S. Hochbaum and D.B. Shmoys. A polynomial approximation scheme for scheduling on uniform
                       processors. SIAM J. Computing, 17(3):539–551, 1988.
                    N. Immorlica, L. Li, V.S. Mirrokni, and A. Schulz. Coordination mechanisms for selfish scheduling.
                       In Proc. 1st Workshop on Internet and Network Economics, pp. 55–69, 2005.
                    E. Koutsoupias. Selfish task allocation. Bulletin of the EATCS (81), pp. 79–88, 2003.
                    E. Koutsoupias, M. Mavronicolas, and P. Spirakis. Approximate equilibria and ball fusion. Theory of
                       Computing Systems, 36(6):683–693, 2003.
                    E. Koutsoupias and C.H. Papadimitriou. Worst-case equilibria. In Proc. 16th Annual Symp. on
                       Theoretical Aspects of Computer Science, pp. 404–413, 1999.
                    L. Libman and A. Orda. The designer’s perspective to atomic noncooperative networks. IEEE/ACM
                       Trans. Networking, 7(6):875–884, 1999.
                    L. Libman and A. Orda. Atomic resource sharing in noncooperative networks. Telecommun. Systems,
                       17(4):385–409, 2001.
                    M. Mavronicolas and P. Spirakis. The price of selfish routing. In Proc. 33rd ACM Symp. on Theory
                       of Computing, pp. 510–519, 2001.
                    M. Mitzenmacher and E. Upfal. Probability and Computing: Randomized Algorithms and Proba-
                       bilistic Analysis. Cambridge University Press, 2005.
                    S. Suri, C. Toth, and Y. Zhou. Selfish load balancing and atomic congestion games. In Proc. 16th
                       Annual ACM Symp. on Parallel Algorithms and Architectures, pp. 188–195, 2005.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007     14:34




                    542                            selfish load balancing


                                                           Exercises
                    20.1 Let G be any instance of the load balancing game with three tasks that should be
                         placed on two identical machines. Show that any pure Nash equilibrium for G is
                         optimal, i.e., cost(A) = opt(G) for any equilibrium assignment A.
                         Remark: Interestingly, the example presented in Section 20.1.2 that yields the
                         worst-case price of anarchy for two identical machines uses only four tasks.
                    20.2 Show, for every m ∈ N, there exists an instance G of the load balancing game
                         with m identical machines and 2m tasks that has a Nash equilibrium assignment
                         A : [n] → [m] with
                                                                          2
                                                    cost(A) = 2 −                  · opt(G).
                                                                         m+ 1
                          Hint: Generalize the example with two machines given in Section 20.1.2.
                    20.3 Prove that the price of anarchy for pure equilibria on instances of the load balancing
                         game with two tasks and two machines
                                                         √         with possibly different speeds corresponds
                         to the golden ratio φ = 12 (1 + 5). That is, show that
                          a) there is a game instance G admitting an equilibrium assignment A with
                             cost(A) = φ · opt(G).
                          b) for every game instance G and every equilibrium assignment A for this instance,
                             it holds cost(A) ≤ φ · opt(G).
                    20.4 Consider an instance of the load balancing game with two tasks both of which
                         have weight 1 and two machines, one of speed 1 and the other of speed s > 0.
                          (a) Show that there does not exist a fully mixed Nash equilibrium if s ≤ 12 or
                              s ≥ 2.
                          (b) Show that there exists a unique fully mixed Nash equilibrium if 12 < s < 2.
                              Describe the strategy profile of this equilibrium as a function of s.
                    20.5 Show that there exists at most one fully mixed Nash equilibrium for every instance
                         of the load balancing game.
                                                                               j
                         Hint: Describe the conditions on the probabilities pi imposed by a fully mixed
                         Nash equilibrium in form of a system of linear equations and show that this system
                                                                                      j
                         has a unique solution. If all the values for the variables pi in this solution are
                         positive then the solution describes a fully mixed Nash equilibrium. Otherwise,
                         there does not exist a fully mixed equilibrium.
                    20.6 Suppose that we are given an instance G of the load balancing game with m
                         identical machines and n tasks whose weights are bounded from above by α ·
                         opt(G), for 0 < α < 1.
                          (a) Show that cost(A) < (1 + α) · opt(G), for every equilibrium assignment A.
                          (b) Let α = log1 m . Show that cost(A) = O(opt(G)), for every equilibrium strategy
                              profile P .
