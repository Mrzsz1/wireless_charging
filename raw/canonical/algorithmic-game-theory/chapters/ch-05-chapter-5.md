---
type: "book-chapter"
book_id: "algorithmic-game-theory"
chapter_id: "ch-05"
chapter_number: 5
chapter_title: "Chapter 5"
source_pdf: "raw/inbox/manual-drop/PDF_B.pdf"
source_page_start: 124
source_page_end: 155
printed_page_start: 124
printed_page_end: 155
part_ids: ["algorithmic-game-theory-ch-05-part-006"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Chapter 5

P1: SBT
9780521872829main       CUNY1061-Nisan        0 521 87282 0     July 15, 2007    11:45




                                                                CHAPTER 5


                                    Combinatorial Algorithms
                                      for Market Equilibria

                                                          Vijay V. Vazirani




                                                                 Abstract

                    Combinatorial polynomial time algorithms are presented for finding equilibrium prices and allocations
                    for the linear utilities case of the Fisher and Arrow–Debreu models using the primal-dual schema and
                    an auction-based approach, respectively. An intersting feature of the first algorithm is that it finds an
                    optimal solution to a nonlinear convex program, the Eisenberg-Gale program.
                        Resource allocation markets in Kelly’s model are also discussed and a strongly polynomial
                    combinatorial algorithm is presented for one of them.


                                                              5.1 Introduction

                    Thinkers and philosophers have pondered over the notions of markets and money
                    through the ages. The credit for initiating formal mathematical modeling and study
                    of these notions is generally attributed to nineteenth-century economist Leon Walras
                    (1874). The fact that Western economies are capitalistic had a lot to do with the over-
                    whelming importance given to this study within mathematical economics – essentially,
                    our most critical decision-making is relegated to pricing mechanisms. They largely de-
                    termine the relative prices of goods and services, ensure that the economy is efficient,
                    in that goods and services are made available to entities that produce items that are
                    most in demand, and ensure a stable operation of the economy.
                       A central tenet in pricing mechanisms is that prices be such that demand equals
                    supply; that is, the economy should operate at equilibrium. It is not surprising therefore
                    that perhaps the most celebrated theorem within general equilibrium theory, the Arrow–
                    Debreu Theorem, establishes precisely the existence of such prices under a very general
                    model of the economy. The First Welfare Theorem, which shows Pareto optimality of
                    allocations obtained at equilibrium prices, provides important social justification for
                    this theory.
                       Although general equilibrium theory enjoyed the status of crown jewel within math-
                    ematical economics, it suffers from a serious shortcoming – other than a few isolated
                    results, some of which were real gems, e.g., Eisenberg and Gale (1959) and Scarf
                                                                      103
P1: SBT
9780521872829main      CUNY1061-Nisan           0 521 87282 0       July 15, 2007      11:45




                    104        combinatorial algorithms for market equilibria

                    (1973), it was essentially a nonalgorithmic theory. With the emergence of new markets
                    on the Internet, which already form an important part of today’s economy and are pro-
                    jected to grow considerably in the future, and the availability of massive computational
                    power for running these markets in a distributed or centralized manner, the need for
                    developing an algorithmic theory of markets and market equilibria is apparent. Such
                    algorithms can also provide a valuable tool for understanding the repercussions of
                    technological advances, new goods or changes to the tax structure on existing prices,
                    production, and consumption.
                        A good beginning has been made over the last 5 years within algorithmic game
                    theory, starting with the work of Deng et al. (2002). However, considering the fact that
                    markets were an active area of study for over a century within mathematical economics,
                    it is safe to say that we have only scratched the surface of what should be a rich theory.
                        Irving Fisher (see Brainard and Scarf, 2000) and Walras (1874) gave two fundamen-
                    tal market models that were studied extensively within mathematical economics. The
                    latter model is also called the exchange model or the Arrow–Debreu model (Arrow and
                    Debreu, 1954). In this chapter we will present combinatorial algorithms for both these
                    models for the case of linear utility functions. A second approach that has emerged for
                    computing equilibria for these models is the efficient solution of convex programs, since
                    equilibrium alloctions for both these models can be captured via convex programs; see
                    Chapter 6 for this approach.
                        Two techniques have been primarily used for obtaining combinatorial algorithms
                    for these models – the primal-dual schema (Devanur et al. 2002) and an auction-based
                    approach (Garg and Kapoor, 2004). We will present algorithms for the Fisher and
                    Arrow–Debreu models, using the first and second techniques, respectively.
                        An interesting aspect of the first algorithm was the extension of the primal-dual
                    schema from its usual setting of combinatorially solving, either exactly or ap-
                    proximately, linear programs, to exactly solving a nonlinear convex program (see
                    Section 5.5). The latter program, due to Eisenberg and Gale (1959), captures
                    equilibrium allocations for the linear case of Fisher’s model. Unlike complementary
                    slackness conditions for linear programs, which involve either primal or dual variables,
                    but not both, KKT conditions for a nonlinear convex program simultaneously involve
                    both types of variables. The repercussions of this are apparent in the way the algorithm
                    is structured.
                        In a different context, that of modeling and understanding TCP congestion control,1
                    Kelly (1997) defined a class of resource allocation markets and gave a convex pro-
                    gram that captures equilibrium allocations for his model. Interestingly enough, Kelly’s
                    program has the same structure as the Eisenberg–Gale program (see also Chapter 22).

                    1 In particular, Kelly’s object was to explain the unprecedented success of TCP, and its congestion avoidance

                      protocol due to Jacobson (1988), which played a crucial role in the phenomenal growth of the Internet and the
                      deployment of a myriad of diverse applications on it. Fairness is a key property desired of a congestion avoidance
                      protocol and Jacobson’s protocol does seem to ensure fairness. Recent results show that if Jacobson’s protocol
                      is run on the end-nodes and the Floyd–Jacobson protocol (Floyd and Jacobson, 1993) is run at buffer queues,
                      in the limit, traffic flows converge to an optimal solution of Kelly’s convex program, i.e., they are equilibrium
                      allocations, see Low and Lapsley (1999). Furthermore, Kelly used his convex programming formulation to
                      prove that equilibrium allocations in his model satisfy proportional fairness (see Section 5.13), thereby giving
                      a formal ratification of Jacobson’s protocol.
P1: SBT
9780521872829main       CUNY1061-Nisan          0 521 87282 0       July 15, 2007        11:45




                           fisher’s linear case and the eisenberg–gale convex program 105

                       The flow market is of special significance within this framework. It consists of a
                    network, with link capacities specified, and source – sink pairs of nodes, each with an
                    initial endowment of money; allocations in this market are flows from each source to
                    the corresponding sink. The problem is to find equilibrium flows and prices of edges
                    (in the context of TCP, the latter can be viewed as drop rates at links).
                       Kelly’s model attracted much theoretical study, partly with a view to designing
                    next-generation protocols. Continuous time algorithms (though not having polynomial
                    running time), for finding equilibrium flows in the flow market, were given by Kelly
                    et al. (1998) (see also Wang et al., 2005, for more recent work along these lines). Soon
                    after the appearance of Devanur et al. (2002), Kelly and Vazirani (2002) observed that
                    Kelly’s model esentially generalizes Fisher’s linear case and stated, “Continuous time
                    algorithms similar to TCP are known, but insights from discrete algorithms may be
                    provocative.”
                       With a view to answering this question, a systematic study of markets whose equilib-
                    ria are captured by Eisenberg-Gale-type programs was undertaken by Jain and Vazirani
                    (2006). In Section 5.14 we present, from this paper, a strongly polynomial algorithm
                    for the special case of the flow market when there is one source and multiple sinks.


                                     5.2 Fisher’s Linear Case and the Eisenberg–Gale
                                                     Convex Program

                    Fisher’s linear case2 is the following. Consider a market consisting of a set B of buyers
                    and a set A of divisible goods. Assume |A| = n and |B| = n . We are given for each
                    buyer i the amount ei of money she possesses and for each good j the amount bj of
                    this good. In addition, we are given the utility functions of the buyers. Our critical
                    assumption is that these functions are linear. Let uij denote the utility derived by i on
                    obtaining a unit amount of good j . Thus if the buyer i is given xij units of good j , for
                    1 ≤ j ≤ n, then the happiness she derives is
                                                                     
                                                                     n
                                                                             uij xij .
                                                                      j =1

                    Prices p1 , . . . , pn of the goods are said to be market clearing prices if, after each buyer
                    is assigned an optimal basket of goods relative to these prices, there is no surplus or
                    deficiency of any of the goods. Our problem is to compute such prices in polynomial
                    time.
                       First observe that w.l.o.g. we may assume that each bj is unit – by scaling the uij ’s
                    appropriately. The uij ’s and ei ’s are in general rational; by scaling appropriately, they
                    may be assumed to be integral. We will make the mild assumption that each good has
                    a potential buyer; i.e., a buyer who derives nonzero utility from this good. Under this
                    assumption, market clearing prices do exist.
                       It turns out that equilibrium allocations for Fisher’s linear case are captured as op-
                    timal solutions to a remarkable convex program, the Eisenberg–Gale convex program.

                    2 See Section 5.13 for a special case of this market and a simple polynomial time algorithm for it.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0              July 15, 2007                  11:45




                    106      combinatorial algorithms for market equilibria

                    Before stating the program, it will be instructive to list considerations that would be
                    useful in deriving such a program.
                       Clearly, a convex program whose optimal solution is an equilibrium allocation must
                    have as constraints the packing constraints on the xij ’s. Furthermore, its objective
                    function, which attempts to maximize utilities derived, should satisfy the following:
                    r If the utilities of any buyer are scaled by a constant, the optimal allocation remains
                      unchanged.
                    r If the money of a buyer b is split among two new buyers whose utility functions are the
                      same as that of b then sum of the optimal allocations of the new buyers should be an
                      optimal allocation for b.

                      The money weighted geometric mean of buyers’ utilities satisfies both these
                    conditions:
                                                          1/i ei
                                                     e
                                             max      ui i          .
                                                                          i∈A

                    Clearly, the following objective function is equivalent:
                                                                e
                                                          max      ui i .
                                                                                 i∈A

                    Its log is used in the Eisenberg–Gale convex program:
                                                             
                                                          
                                                          n
                                        maximize                      ei log ui
                                                          i=1
                                                                          
                                                                          n
                                        subject to        ui =                   uij xij            ∀i ∈ B
                                                                          j =1
                                                                                                                     (5.1)
                                                                 
                                                          
                                                          n
                                                                     xij ≤ 1                        ∀j ∈ A
                                                          i=1
                                                          xij ≥ 0                                   ∀i ∈ B, ∀j ∈ A
                    where xij is the amount of good j allocated to buyer i. Interpret Lagrangian variables,
                    say pj ’s, corresponding to the second set of conditions as prices of goods. By the
                    Karush, Kuhn, Tucker (KKT) conditions, optimal solutions to xij ’s and pj ’s must
                    satisfy the following:

                     (i) ∀j ∈ A : pj ≥ 0.
                                          
                    (ii) ∀j ∈ A : pj > 0 ⇒ i∈A xij = 1.
                                           u         j ∈A uij xij
                    (iii) ∀i ∈ B, ∀j ∈ A : pijj ≤       ei
                                                                      .
                                                                            
                                                        u                        j ∈A uij xij
                    (iv) ∀i ∈ B, ∀j ∈ A : xij > 0     ⇒ pijj          =             ei
                                                                                                .

                    From these conditions, one can derive that an optimal solution to convex program (5.1)
                    must satisfy the market clearing conditions.
                       The Eisenberg and Gale program also helps prove, in a very simple manner, the
                    following basic properties of equilibria for the linear case of Fisher’s model.
P1: SBT
9780521872829main   CUNY1061-Nisan      0 521 87282 0   July 15, 2007        11:45




                       fisher’s linear case and the eisenberg–gale convex program 107

                    Theorem 5.1 For the linear case of Fisher’s model:
                    r If each good has a potential buyer, equilibrium exists.
                    r The set of equilibrium allocations is convex.
                    r Equilibrium utilities and prices are unique.
                    r If all u ’s and e ’s are rational, then equilibrium allocations and prices are also
                              ij       i
                      rational. Moreover, they can be written using polynomially many bits in the length
                      of the instance.

                    proof Corresponding to good j there is a buyer i such that uij > 0. By the
                    third KKT condition,
                                                            ei uij
                                                   pj ≥              > 0.
                                                            j uij xij
                                                           
                    Now, by the second KKT condition, i∈A xij = 1. Hence, prices of all goods are
                    positive and all goods are fully sold.
                       The third and fourth conditions imply that if buyer i gets good j then j must
                    be among the goods that give buyer i maximum utility per unit money spent at
                    current prices. Hence each buyer gets only a bundle consisting of her most desired
                    goods, i.e., an optimal bundle.
                       The fourth condition is equivalent to
                                                          ei uij xij
                                        ∀i ∈ B, ∀j ∈ A :               = pj xij .
                                                           j ∈A uij xij

                    Summing over all j gives
                                                      
                                                    ei j uij xij     
                                           ∀i ∈ B :               =   pj xij .
                                                      j ∈A uij xij   j

                    This implies
                                                                   
                                                 ∀i ∈ B : ei =              pj xij .
                                                                        j


                       Hence the money of each buyer is fully spent. This completes the proof that
                    market equilibrium exists.
                       Since each equilibrium allocation is an optimal solution to the Eisenberg-Gale
                    convex program, the set of equilibrium allocations must form a convex set.
                       Since log is a strictly concave function, if there is more than one equilibrium,
                    the utility derived by each buyer must be the same in all equilibria. This fact,
                    together with the fourth condition, gives that the equilibrium prices are unique.
                       Finally, we prove the fourth claim by showing that equilibrium allocations
                    and prices are solutions to a system of linear equations. Let qj = 1/pj be a new
                    variable corresponding to each good j and let k be the number of nonzero xij ’s in
                    an equilibrium allocation. The system will consist of k + l equations over k + l
                    unknowns, the latter being the n qj ’s and the k the nonzero xij ’s. The equations are
                    corresponding to each good j , the equality given by the second KKT condition,
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0     July 15, 2007    11:45




                    108      combinatorial algorithms for market equilibria

                      and corresponding to each nonzero xij , the equality given by the fourth KKT
                      condition.



                                  5.3 Checking If Given Prices Are Equilibrium Prices

                    Let p = (p1 , . . . , pn ) denote a vector of prices. Let us first devise an algorithm for
                    answering the following question: Is p the equilibrium price vector, and if so, find
                    equilibrium allocations for the buyers.
                        At prices p, buyer i derives uij /pj amount of utility per unit money spent on good j .
                    Clearly, she will be happiest with goods that maximize this ratio. Define her bang per
                    buck to be α i = maxj {uij /pj }. For each i ∈ B, j ∈ A, α i ≥ uij /pj , with equality
                    holding only if j is i’s bang per buck good. If there are several goods maximizing
                    this ratio, she is equally happy with any combination of these goods. This motivates
                    defining the following bipartite graph, G. Its bipartition is (A, B) and for i ∈ B, j ∈ A,
                    (i, j ) is an edge in G iff α i = uij /pj . We will call this graph the equality subgraph and
                    its edges the equality edges.


                                                   5.3.1 The Network N( p)
                    Any goods sold along the edges of the equality subgraph will make buyers happiest,
                    relative to prices p. Computing the largest amount of goods that can be sold in this
                    manner, without exceeding the budgets of buyers or the amount of goods available
                    (assumed unit for each good), can be accomplished by computing max-flow in the
                    following network (see Figure 5.1). Direct edges of G from A to B and assign a
                    capacity of infinity to all these edges. Introduce source vertex s and a directed edge
                    from s to each vertex j ∈ A with a capacity of pj . Introduce sink vertex t and a directed
                    edge from each vertex i ∈ B to t with a capacity of ei . The network is clearly a function
                    of the prices p and will be denoted by N( p).

                                                 A: goods                     B: buyers
                                                     1                              1
                                            p1
                                                                                          m1
                                           p2
                                                     2
                                                                                          m2
                              s                                                                       t
                                           p3                                       2
                                                                                          m3
                                           p4
                                                     3


                                                     4                              3

                                                          infinite capacity edges

                                                   Figure 5.1. The network N( p).
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 15, 2007   11:45




                                  the primal-dual schema in the enhanced setting                           109

                       Corresponding to a feasible flow f in network N( p), let us define the allocation of
                    goods to the buyers to be the following. If edge (j, i) from good j to buyer i carries
                    flow f (j, i), then buyer i receives f (j, i)/pj units of good j .
                       The question posed above can be answered via one max-flow computation, as
                    asserted in the following lemma. Its proof is straightforward and is omitted.

                      Lemma 5.2 Prices p are equilibrium prices iff in the network N( p) the two cuts
                      (s, A ∪ B ∪ t) and (s ∪ A ∪ B, t) are min-cuts. If so, allocations corresponding
                      to any max-flow in N are equilibrium allocations.


                                    5.4 Two Crucial Ingredients of the Algorithm

                    The algorithm starts with very low prices that are guaranteed to be below the equilibrium
                    prices for each good. The algorithm always works on the network N(p) w.r.t. the current
                    prices p. W.r.t. the starting prices, buyers have surplus money left. The algorithm raises
                    prices iteratively and reduces the surplus. When the surplus vanishes, it terminates;
                    these prices are equilibrium prices.
                       This algorithmic outline immediately raises two questions:
                    r How do we ensure that the equilibrium price of no good is exceeded?
                    r How do we ensure that the surplus money of buyers reduces fast enough that the
                      algorithm terminates in polynomial time?
                    The answers to these two questions lead to two crucial ingredients of the algorithm:
                    tight sets and balanced flows.


                              5.5 The Primal-Dual Schema in the Enhanced Setting

                    We will use the notation setup in the previous sections to describe at a high level the
                    new difficulties presented by the enhanced setting of convex programs and the manner
                    in which the primal-dual schema is modified to obtain a combinatorial algorithm for
                    solving the Eisenberg–Gale convex program.
                        The fundamental difference between complementary slackness conditions for linear
                    programs and KKT conditions for nonlinear convex programs is that whereas the
                    former do not involve both primal and dual variables simultaneously in an equality
                    constraint (obtained by assuming that one of the variables takes a nonzero value), the
                    latter do.
                        As described in the previous section, the algorithm will start with very low prices and
                    keep increasing them greedily, i.e., the dual growth process is greedy. Indeed, all known
                    primal-dual algorithms use a greedy dual growth process – with one exception, namely
                    Edmonds’ algorithm for maximum weight matching in general graphs (Edmonds,
                    1965).
                        Now, the disadvantage of a greedy dual growth process is obvious – the fact that a
                    raised dual is “bad,” in the sense that it “obstructs” other duals that could have led to a
                    larger overall dual solution, may become clear only later in the run of the algorithm. In
P1: SBT
9780521872829main      CUNY1061-Nisan    0 521 87282 0      July 15, 2007   11:45




                    110     combinatorial algorithms for market equilibria

                    view of this, the issue of using more sophisticated dual growth processes has received
                    a lot of attention, especially in the context of approximation algorithms. The problem
                    with such a process is that it will make primal objects go tight and loose and the
                    number of such reversals will have to be upper bounded in the running time analysis.
                    The impeccable combinatorial structure of matching supports such an accounting and
                    in fact this leads to a strongly polynomial algorithm. However, thus far, all attempts at
                    making such a scheme work out for other problems have failed.
                        In our case, even though the dual growth process is greedy, because of the more
                    complex nature of KKT conditions, edges in the equality subgraph appear and disappear
                    as the algorithm proceeds. Hence, we are forced to carry out the difficult accounting
                    process alluded to above for bounding the running time.
                        We next point out which KKT conditions the algorithm enforces and which ones
                    it relaxes, as well as the exact mechanism by which it satisfies the latter. Throughout
                    the algorithm, we enforce the first two conditions listed in Section 5.2. As mentioned
                    in Section 5.4, at any point in the algorithm, via a max-flow in the network N( p), all
                    goods can be sold; however, buyers may have surplus money left over. W.r.t. a balanced
                    flow in network N( p) (see Section 5.7 for a definition of such a flow), let mi be the
                    money spent by buyer i. Thus, buyer i’s surplus money is γ i = ei − mi . We will relax
                    the third and fourth KKT conditions to the following:
                                               
                    r ∀i ∈ B, ∀j ∈ A : uij ≤       j ∈A uij xij
                                                    .
                                       pj      mi   
                    r ∀i ∈ B, ∀j ∈ A : x > 0 ⇒ uij = j ∈A uij xij .
                                        ij
                                               pj       mi

                      Consider the following potential function:

                                                    = γ 21 + γ 22 + · · · + γ 2n .

                    We will give a process by which this potential function decreases by an inverse poly-
                    nomial fraction in polynomial time (in each phase, as detailed in Lemma 5.21). When
                     drops all the way to zero, all KKT conditions are exactly satisfied.
                        Finally, there is a marked difference between the way this algorithm will satisfy
                    KKT conditions and the way primal-dual algorithms for LP’s do. The latter satisfy
                    complementary conditions in discrete steps, i.e., in each iteration, the algorithm sat-
                    isfies at least one new condition. So, if each iteration can be implemented in strongly
                    polynomial time, the entire algorithm has a similar running time. On the other hand,
                    the algorithm for Fisher’s linear case satisfies KKT conditions continuously – as the
                    algorithm proceeds, the KKT conditions corresponding to each buyer get satisfied to a
                    greater extent.
                        Observe that at the start of the algorithm, the value of φ is a function not just of
                    the number of buyers and goods but of the length of the input (since it depends on
                    the money possessed by buyers). Therefore, even though a phase of the algorithm can
                    be implemented in strongly polynomial time, the running time of the entire algorithm
                    is polynomial and not strongly polynomial. Indeed, obtaining a strongly polynomial
                    algorithm for this problem remains a tantalizing open problem (see Section 5.15).
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0     July 15, 2007   11:45




                                                           balanced flows                                   111

                                             5.6 Tight Sets and the Invariant

                    Let p denote the current prices within the run of the algorithm. For a set S ⊆ A of goods,
                    let p(S) denote the total value of goods in S; this is simply the sum of current prices of
                    goods in S. For a set T ⊆ B of buyers, let m(T ) denote the total money possessed by
                    the buyers in T ; i.e., m(T ) = i∈T ei . For S ⊆ A, define its neighborhood in N( p),
                                          (S) = {j ∈ B | ∃i ∈ S with (i, j ) ∈ N( p)}.
                    Clearly, (S) is the set of buyers who are interested in goods in S at current prices.
                       We will say that S is a tight set if the current value of S exactly equals the money
                    possessed by buyers who are interested in goods in S; i.e., p(S) = m((S)). Under this
                    circumstance, increasing prices of goods in S may lead to exceeding the equilibrium
                    price of some good. Therefore, when a set of goods goes tight, the algorithm freezes
                    the prices of all goods in S. As described in Section 5.7, when new edges enter the
                    equality subgraph, the algorithm may unfreeze certain frozen goods and again start
                    increasing their prices.
                       A systematic way of ensuring that the equilibrium price of no good is exceeded is
                    to ensure the following Invariant.
                       Invariant: The prices p are such that the cut (s, A ∪ B ∪ t) is a min-cut in N( p).

                      Lemma 5.3        For given prices p, network N( p) satisfies the Invariant iff
                                                    ∀S ⊆ A : p(S) ≤ m((S)).

                      proof The forward direction is trivial, since under max-flow (of value p(A))
                      every set S ⊆ A must be sending p(S) amount of flow to its neighborhood.
                         Let us prove the reverse direction. Assume that (s ∪ A1 ∪ B1 , A2 ∪ B2 ∪ t) is a
                      min-cut in N( p), with A1 , A2 ⊆ A and B1 , B2 ⊆ B (see Figure 5.2). The capacity
                      of this cut is p(A2 ) + m(B1 ). Now, (A1 ) ⊆ B1 , since otherwise the cut will have
                      infinite capacity. Moving A1 and (A1 ) to the t side also results in a cut. By
                      the condition stated in the Lemma, p(A1 ) ≤ m((A1 )). Therefore, the capacity
                      of this cut is no larger than the previous one and this is also a min-cut in N( p).
                      Hence the Invariant holds.

                       The Invariant ensures that, at current prices, all goods can be sold. The only even-
                    tuality is that buyers may be left with surplus money. The algorithm raises prices
                    systematically, thereby decreasing buyers’ surplus money. When (s ∪ A ∪ B, t) is also
                    a min-cut in N( p), by Lemma 5.2, equilibrium has been attained.


                                                      5.7 Balanced Flows

                    Denote the current network, N( p), by simply N. We will assume that network N
                    satisfies the Invariant; i.e., (s, A ∪ B ∪ t) is a min-cut in N. Given a feasible flow f in
                    N, let R(f ) denote the residual graph w.r.t. f . Define the surplus of buyer i, γi (N, f ),
                    to be the residual capacity of the edge (i, t) with respect to flow f in network N,
P1: SBT
9780521872829main       CUNY1061-Nisan          0 521 87282 0        July 15, 2007     11:45




                    112        combinatorial algorithms for market equilibria


                                                                A1                      B1




                                               s                                               t




                                                                A2                      B2


                                      Figure 5.2. Min-cut in N( p). There are no edges from A1 to B2 .


                    i.e., ei minus the flow sent through the edge (i, t). The surplus vector is defined to be
                    γ (N, f ) := (γ1 (N, f ), γ2 (N, f ), . . . , γn (N, f )). Let v denote the l2 norm of vector
                    v. A balanced flow in network N is a flow that minimizes γ (N, f ) . A balanced flow
                    must be a max-flow in N because augmenting a given flow can only lead to a decrease
                    in the l2 norm of the surplus vector.

                       Lemma 5.4          All balanced flows in N have the same surplus vector.

                       proof It is easy to see that if γ1 and γ2 are the surplus vectors w.r.t flows f1
                       and f2 , then (γ1 + γ2 )/2 is the surplus vector w.r.t the flow (f1 + f2 )/2. Since the
                       set of feasible flows in N is a convex region, so is the set of all feasible surplus
                       vectors. Since a balanced flow minimizes a strictly concave function of the surplus
                       vector, the optimal surplus vector must be unique.

                       The following property of balanced flows will be used critically in the algorithm. 3
                       Property 1: If γ j (N, f ) < γ i (N, f ) then there is no path from node j to node i
                       in R(f ) − {s, t}.

                       Theorem 5.5           A maximum-flow in N is balanced iff it satisfies Property 1.

                       proof Let f be a balanced flow and let γi (N, f ) > γj (N, f ) for some i, j ∈ B.
                       Suppose, for the sake of contradiction, there is a path from j to i in R(f ) − {s, t}.
                          In N, the only edge out of j is the edge (j, t). Since the path in R(f ) from j to i
                       must start with a positive capacity edge which is different from edge (j, t), by flow
                       conservation, the capacity of (t, j ) must be positive in R(f ). Since γi (N, f ) > 0,
                       the edge (i, t) has a positive capacity in R(f ). Now, the edges (t, j ) and (i, t)

                    3 Unlike the previous sections, in Section 5.7, j will denote a buyer.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0     July 15, 2007   11:45




                                                           balanced flows                                   113




                                  Figure 5.3. The circulation in R( f ) if Property 1 does not hold.


                      concatenated with the path from j to i gives us a cycle with positive residual
                      capacity in R(f ) (see Figure 5.3). Sending a circulation of positive value along
                      this cycle will result in another max-flow in which the residual capacity of j is
                      slightly larger and that of i is slightly smaller; i.e., the flow is more balanced. This
                      contradicts the fact that f is a balanced flow.
                         To prove the other direction, first observe that the l2 norm of the surplus vector
                      of a max-flow f satisfying Property 1 is locally optimum w.r.t. changes in pairs
                      of components of the surplus vector. This is so because any circulation in R(f )
                      can only send flow from a high surplus buyer to a low surplus buyer resulting
                      in a less balanced flow. Now, since l2 norm is a strictly concave function, any
                      locally optimal solution is also globally optimal. Hence, a max-flow f satisfying
                      Property 1 must be a balanced flow.



                                               5.7.1 Finding a Balanced Flow
                    We will show that the following algorithm, which uses a divide and conquer strategy,
                    finds a balanced flow in the given network N in polynomial time. As stated above, we
                    will assume that this network satisfies the Invariant, i.e., (s, A ∪ B ∪ t) is a min-cut
                    in N.
                       Continuously reduce the capacities of all edges that go from B to t, other than those
                    edges whose capacity becomes zero, until the capacity of the cut ({s} ∪ A ∪ B, {t})
                    becomes the same as the capacity of the cut ({s}, A ∪ B ∪ {t}). Let the resulting network
                    be N  and let f  be a max-flow in N  . Find a maximal s − t min-cut in N  , say (S, T ),
                    with s ∈ S and t ∈ T .
                       Case 1: If T = {t} then find a max-flow in N  and output it – this will be a balanced
                    flow in N.
                       Case 2: Otherwise, let N1 and N2 be the subnetworks of N induced by S ∪ {t}
                    and T ∪ {s}, respectively. (Observe that N1 and N2 inherit original capacities from
                    N and not the reduced capacities from N  .) Let A1 and B1 be the subsets of A and
                    B, respectively, induced by N1 . Similarly, let A2 and B2 be the subsets of A and B,
                    respectively, induced by N2 . Recursively find balanced flows, f1 and f2 , in N1 and N2 ,
                    respectively. Output the flow f = f1 ∪ f2 – this will be a balanced flow in N.
P1: SBT
9780521872829main     CUNY1061-Nisan      0 521 87282 0   July 15, 2007   11:45




                    114     combinatorial algorithms for market equilibria

                      Lemma 5.6      f is a max-flow in N.

                      proof In the first case, i.e., T = {t}, the algorithm outputs a max-flow in N  .
                      This flow must saturate the cut ({s} ∪ A ∪ B, {t}). However, since the capacity
                      of this cut in N  is the same as the capacity of the cut ({s}, A ∪ B ∪ {t}), by the
                      Invariant, this is also a max-flow in N.
                          Next let us consider the second case. Since N1 and N2 are edge-disjoint net-
                      works, f = f1 ∪ f2 will be a feasible flow in N. We will show that f must saturate
                      all edges from s to A and therefore by the Invariant, it is a max-flow.
                          Let g be a max-flow in N. Observe that N  , and hence N, cannot have any edges
                      from A1 to B2 . Therefore, all flow of g going to A1 must flow via B1 . Therefore,
                      the restriction of g to N1 saturates all edges from s to A1 in N1 . Therefore, so
                      must f1 since it is a max-flow in N1 .
                          Let f  be a max-flow in N  . Since (S, T ) is a min-cut in N  , f  must saturate
                      all edges from s to A2 . Furthermore, all flow of f  going to A2 must flow via B2 ,
                      i.e., the restriction of f  to flow going through A2 is a feasible flow in N2 . Since
                      f2 is a max-flow in N2 , it must also saturate all edges from s to A2 . Hence f
                      saturates all edges from s to A in N, and is therefore a max-flow.

                      Lemma 5.7      f is a balanced flow in network N.

                      proof We will show, by induction on the depth of recursion, that the max-flow
                      output by the algorithm is a balanced flow in N. In the base case, the algorithm
                      terminates in the first case; i.e., T = {t}, the surplus vector is precisely the amounts
                      subtracted from capacities of edges from B to t in going from N to N  . Clearly,
                      this surplus vector makes components as equal as possible, thus minimizing its l2
                      norm.
                          Next assume that the algorithm terminates in the second case. By Lemma 5.6, f
                      is a max-flow; we will show that it satisfies Property 1 and is therefore a balanced
                      flow. By the induction hypothesis, f1 and f2 are balanced flows in N1 and N2 ,
                      respectively, and therefore Property 1 cannot be violated in these two networks.
                          Let R be the residual graph of N w.r.t. flow f ; we only need to show that
                      paths in R that go from one part to the other do not violate Property 1. As already
                      observed in the proof of Lemma 5.6, there are no edges from A1 to B2 in N, and
                      therefore there are no residual paths from j ∈ B1 to i ∈ B2 . There may however
                      be paths going from j ∈ B2 to i ∈ B1 in R. We will show that for any two nodes
                      i ∈ B1 and j ∈ B2 , γi (N, f ) < γj (N, f ), thereby establishing Property 1.
                          First observe that by the maximality of the min-cut found in N  , all nodes in B2
                      have surplus capacity > 0 w.r.t. flow f  in N  (all nodes having surplus zero must
                      be in B1 ). Therefore, the same amount, say X, was subtracted from the capac ity
                      of each edge (i, t), i ∈ B2 , in going from network N to N  . We will show that
                      γi (N, f ) > X for each i ∈ B2 . A similar proof shows that γi (N, f ) < X for each
                      i ∈ B1 , thereby establishing Property 1.
                          Let L be the set of vertices in B2 having minimum surplus w.r.t. f . Let K be
                      the set of vertices in A2 that are reachable via an edge from L in R. We claim
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 15, 2007   11:45




                                                      the main algorithm                                    115

                      that (K) = L, because otherwise, there will be a residual path from i ∈ L to
                      j ∈ B2 − L, thereby violating Property 1.
                          Let c(K) denote the sum of capacities of all edges from s to vertices of K.
                      Observe that all these edges are saturated in f  and this flow must leave via
                      vertices of L. Let EL denote the set of edges going from L to t. Let c(L) and
                      c (L) denote the sum of capacities of all edges in EL in networks N and N  ,
                      respectively. By the argument given above, c (L) > c(K).
                          Since X is subtracted from all edges in EL in going from network N to N  ,
                      c(L) = c (L) + |L|X. The total surplus of the edges in EL w.r.t. flow f is
                                         c(L) − c(K) = c (L) + |L|X − c(K) > |L|X.
                      Finally, since all edges in EL have the same surplus, each has surplus > X. The
                      lemma follows.

                      Theorem 5.8 The above-stated algorithm computes a balanced flow in network
                      N using at most n max-flow computations.

                      proof Clearly, the number of goods in the biggest piece drops by at least 1 in
                      each iteration. Therefore, the depth of recursion is at most n. Next, observe that
                      N1 and N2 are vertex disjoint, other than s and t, and therefore, the time needed
                      to compute max-flows in them is bounded by the time needed to compute a max-
                      flow in N. Hence, the total computational overhead is n max-flow computations.
                      Finally, as shown in Lemma 5.7, the flow output by the algorithm is a balanced
                      flow in N.


                                                  5.8 The Main Algorithm

                    First we show how to initialize prices so the Invariant holds. The following two
                    conditions guarantee this.
                    r The initial prices are low enough prices that each buyer can afford all the goods. Fixing
                      prices at 1/n suffices, since the goods together cost one unit and all ei ’s are integral.
                    r Each good j has an interested buyer, i.e., has an edge incident at it in the equality
                      subgraph. Compute αi for each buyer i at the prices fixed in the previous step and
                      compute the equality subgraph. If good j has no edge incident, reduce its price to
                                                                      
                                                                       uij
                                                          pj = max          .
                                                                  i    αi
                       If the Invariant holds, it is easy to see that there is a unique maximal tight set S ⊆ A.
                    Clearly, the prices of goods in the tight set cannot be increased without violating the
                    Invariant. On the other hand, the algorithm can raise prices of all goods in A − S.
                    However, we do not know any way of bounding the running time of any algorithm
                    based on such an approach. In fact, it seems that any such algorithm can be forced
                    to take a large number of steps in which it makes only very small progress toward
                    decreasing the surplus of the buyers, thereby taking super polynomial time.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 15, 2007   11:45




                    116      combinatorial algorithms for market equilibria

                       Instead, we will show how to use the notion of balanced flow to give a polynomial
                    time algorithm. The idea is to always raise prices of those goods which are desired by
                    buyers having a lot of surplus money. Eventually, when a subset of these goods goes
                    tight, the surplus of some of these buyers vanishes, thus leading to substantial progress.
                    Property 1 of balanced flows provides us with a powerful condition to ensure that even
                    as the network N( p) changes because of changes in p, the algorithm can still keep
                    working with a set of buyers having a large surplus.
                       The iterative improvement steps follow the spirit of the primal-dual schema: The
                    “primal” variables are the flows in the edges of N( p) and the “dual” variables are
                    the current prices. The current flow suggests how to improve the prices and vice
                    versa.
                       A run of the algorithm is partitioned into phases, each phase ends with a new set
                    going tight. Each phase is partitioned into iterations that are defined below.
                       A phase starts with computation of a balanced flow, say f , in the current network,
                    N( p). If the algorithm of Section 5.7 for finding a balanced flow terminates in
                    Case 1, then by Lemma 5.2 the current prices and allocations are equilibrium prices
                    and allocations and the algorithm halts. Otherwise, let δ be the maximum surplus of
                    buyers w.r.t. f . Initialize I to be the set of buyers having surplus δ. Let J be the set of
                    goods that have edges to I in N( p). The network induced by I ∪ J is called the active
                    subgraph.
                       At this point, we are ready to raise prices of goods in J . However, we would like to
                    do this in such a way that for each buyer i ∈ I , the set of goods she likes best, which
                    are all in J , remains unchanged as prices increase. This can be accomplished by raising
                    prices of goods in J in such a way that the ratio of any two prices remains unchanged.
                    The rest of the algorithm for a phase is as follows.
                       Step : Multiply the current prices of all goods in J by variable x, initialize x to 1
                    and raise x continuously until one of the following two events happens. Observe that
                    as soon as x > 1, buyers in B − I are no longer interested in goods in J and all such
                    edges can be dropped from the equality subgraph and N.

                    r Event 1: If a subset S ⊆ J goes tight, the current phase terminates and the algorithm
                      starts with the next phase.
                    r Event 2: As prices of goods in J keep increasing, goods in A − J become more and
                      more desirable for buyers in I . If as a result an edge (i, j ), with i ∈ I and j ∈ A − J ,
                      enters the equality subgraph (see Figure 5.4). add directed edge (j, i) to network N ( p)
                      and compute a balanced flow, say f , in the current network, N ( p). If the balanced
                      flow algorithm terminates in Case 1, halt and output the current prices and allocations.
                      Otherwise, let R be the residual graph corresponding to f . Determine the set of all
                      buyers that have residual paths to buyers in the current set I (clearly, this set will contain
                      all buyers in I ). Update the new set I to be this set. Update J to be the set of goods that
                      have edges to I in N (p). Go to Step .


                    To complete the algorithm, we simply need to compute the smallest values of x at
                    which Event 1 and Event 2 happen, and consider only the smaller of these. For Event
                    2, this is straightforward. We give an algorithm for Event 1 in the next section.
P1: SBT
9780521872829main      CUNY1061-Nisan        0 521 87282 0           July 15, 2007   11:45




                                                         finding tight sets                                117

                                                             A−J                     B−I


                                                                                           i




                                         s                                                     t



                                                             j             active
                                                                         subgraph

                                                                 J                     I

                                    Figure 5.4. If Event 2 happens, edge ( j , i ) is added to N( p).


                                                      5.9 Finding Tight Sets

                    Let p denote the current price vector (i.e., at x = 1). We first present a lemma that
                    describes how the min-cut changes in N(x · p) as x increases. Throughout this section,
                    we will use the function m to denote money w.r.t. prices p. W.l.o.g. assume that w.r.t.
                    prices p the tight set in G is empty (since we can always restrict attention to the active
                    subgraph, for the purposes of finding the next tight set). Define
                                                                               m((S))
                                                         x ∗ = min                     ,
                                                                      ∅=S⊆A    m(S)
                    the value of x at which a nonempty set goes tight. Let S ∗ denote the tight set at
                    prices x ∗ · p. If (s ∪ A1 ∪ B1 , A2 ∪ B2 ∪ t) is a cut in the network, we will assume that
                    A1 , A2 ⊆ A and B1 , B2 ⊆ B.

                      Lemma 5.9 W.r.t. prices x · p:
                      r if x ≤ x ∗ then (s, A ∪ B ∪ t) is a min-cut.
                       r if x > x ∗ then (s, A ∪ B ∪ t) is not a min-cut. Moreover, if (s ∪ A ∪ B , A ∪
                                                                                             1   1   2
                         B2 ∪ t) is a min-cut in N (x · p) then S ∗ ⊆ A1 .

                      proof Suppose x ≤ x ∗ . By definition of x ∗ ,
                                                   ∀S ⊆ A : x · m(S) ≤ m((S)).
                      Therefore by Lemma 5.3, w.r.t. prices x · p, the Invariant holds. Hence (s, A ∪
                      B ∪ t) is a min-cut.
                          Next suppose that x > x ∗ . Since x · m(S ∗ ) > x ∗ · m(S ∗ ) = m((S ∗ )), w.r.t.
                      prices x · p, the cut (s ∪ S ∗ ∪ (S ∗ ), t) has strictly smaller capacity than the cut
                      (s ∪ A ∪ B, t). Therefore the latter cannot be a min-cut.
                          Now consider the min-cut (s ∪ A1 ∪ B1 , A2 ∪ B2 ∪ t). Let S ∗ ∩ A2 = S2 and
                      S ∗ − S2 = S1 . Suppose S2 = ∅. Clearly (S1 ) ⊆ B1 (otherwise the cut will have
                      infinite capacity). If m((S2 ) ∩ B2 ) < x · m(S2 ), then by moving S2 and (S2 ) to
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 15, 2007   11:45




                    118     combinatorial algorithms for market equilibria

                      the s side of this cut, we can get a smaller cut, contradicting the minimality of the
                      cut picked. In particular, m((S ∗ ) ∩ B2 ) ≤ m((S ∗ )) = x ∗ · m(S ∗ ) < x · m(S ∗ ).
                      Therefore S2 = S ∗ , and hence, S1 = ∅. Furthermore,
                                             m((S2 ) ∩ B2 ) ≥ x · m(S2 ) > x ∗ m(S2 ).
                      On the other hand,
                                        m((S2 ) ∩ B2 ) + m((S1 )) ≤ x ∗ (m(S2 ) + m(S1 )).
                      The two imply that
                                                           m((S1 ))
                                                                     < x∗,
                                                            m(S1 )
                      contradicting the definition of x ∗ . Hence S2 = ∅ and S ∗ ⊆ A1 .

                      Lemma 5.10 Let x = m(B)/m(A) and suppose that x > x ∗ . If (s ∪ A1 ∪
                      B1 , A2 ∪ B2 ∪ t) be a min-cut in N(x · p) then A1 must be a proper subset of
                      A.

                      proof If A1 = A, then B1 = B (otherwise this cut has ∞ capacity), and (s ∪
                      A ∪ B, t) is a min-cut. But for the chosen value of x, this cut has the same capacity
                      as (s, A ∪ B ∪ t). Since x > x ∗ , the latter is not a min-cut by Lemma 5.9. Hence,
                      A1 is a proper subset of A.

                      Lemma 5.11        x ∗ and S ∗ can be found using n max-flow computations.

                      proof Let x = m(B)/m(A). Clearly, x ≥ x ∗ . If (s, A ∪ B ∪ t) is a min-cut in
                      N(x · p), then by Lemma 5.9, x ∗ = x. If so, S ∗ = A.
                         Otherwise, let (s ∪ A1 ∪ B1 , A2 ∪ B2 ∪ t) be a min-cut in N(x · p). By Lem-
                      mas 5.9 and 5.10, S ∗ ⊆ A1 ⊂ A. Therefore, it is sufficient to recurse on the smaller
                      graph (A1 , (A1 )).


                                          5.10 Running Time of the Algorithm

                    Let U = maxi∈B,j ∈A {uij } and let  = nU n .

                      Lemma 5.12 At the termination of a phase, the prices of goods in the newly
                      tight set must be rational numbers with denominator ≤ .

                      proof Let S be the newly tight set and consider the equality subgraph induced
                      on the bipartition (S, (S)). Assume w.l.o.g. that this graph is connected (other-
                      wise we prove the lemma for each connected component of this graph). Let j ∈ S.
                      Pick a subgraph in which j can reach all other vertices j  ∈ S. Clearly, at most
                      2|S| ≤ 2n edges suffice. If j reaches j  with a path of length 2l, then pj  = apj /b
                      where a and b are products of l utility parameters (uik ’s) each. Since alternate
                      edges of this path contribute to a and b, we can partition the uik ’s in this subgraph
P1: SBT
9780521872829main    CUNY1061-Nisan         0 521 87282 0      July 15, 2007      11:45




                                                running time of the algorithm                                              119

                    into two sets such that a and b use uik ’s from distinct sets. These considerations
                    lead easily to showing that m(S) = pj c/d where c ≤ . Now,
                                                            pj = m((S))d/c,
                    hence proving the lemma.

                    Lemma 5.13 Consider two phases P and P  , not necessarily consecutive, such
                    that good j lies in the newly tight sets at the end of P as well as P  . Then the
                    increase in the price of j , going from P to P  , is ≥ 1/2 .

                    proof Let the prices of j at the end of P and P  be p/q and r/s, respectively.
                    Clearly, r/s > p/q. By Lemma 5.12, q ≤  and r ≤ . Therefore the increase
                    in price of j ,
                                                               r  p  1
                                                                 − ≥ 2.
                                                               s  q 



                    Within a phase, we will call each occurrence of Events 1 and 2 an iteration.

                    Lemma 5.14          The total number of iterations in a phase is bounded by n.

                    proof After an iteration due to Event 2, at least one new good must move into
                    the active subgraph. Since there is at least one good in the active subgraph at the
                    start of a phase, the total number of iterations in a phase due to Event 2 is at
                    most n − 1. Finally, the last iteration in each phase is due to Event 1. The lemma
                    follows.

                    Lemma 5.15 If f and f ∗ are respectively a feasible and a balanced flow
                    in N( p) such that γi ( p, f ∗ ) = γi ( p, f ) − δ, for some i ∈ B and δ > 0, then
                     γ ( p, f )∗ 2 ≤ γ ( p, f ) 2 − δ 2 .

                    proof Suppose we start with f and get a new flow f  by decreasing the surplus
                    of i by δ, and increasing the surpluses of some other buyers in the process. We
                    show that this already decreases the l2 norm of the surplus vector by δ 2 and so the
                    lemma follows.
                       Consider the flow f ∗ − f . Decompose this flow into flow paths and circula-
                    tions. Among these, augment f with only those that go through the edge (i, t), to
                    get f  . These are either paths that go from s to i to t, or circulations that go from
                    i to t to some il and back to i. Then γi (f  ) = γi (f ∗ ) = γi (f ) − δ and for a set
                    of vertices i1 , i2 , . . . , ik , we have γil (f  ) = γil (f ) + δl , s.t. δ1 , δ2 , . . . , δk > 0 and
                      k                                                                                         ∗          ∗
                       l=1 δl ≤ δ. Moreover, for all l, there is a path from i to il in R( p, f ). Since f
                    is balanced, and satisfies Property 1, γi (f  ) = γi (f ∗ ) ≥ γil (f ∗ ) ≥ γil (f  ).
                       By Lemma 5.16, γ ( p, f  ) 2 ≤ γ ( p, f ) 2 − δ 2 and since f ∗ is a balanced
                    flow in N( p), γ ( p, f ∗ ) 2 ≤ γ ( p, f  ) 2 .
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0     July 15, 2007     11:45




                    120      combinatorial algorithms for market equilibria
                                                                                           n
                      Lemma 5.16 If a ≥ bi ≥ 0, i = 1, 2, . . . , n and δ ≥                    j =1 δj    where δ, δj ≥
                      0, j = 1, 2, . . . , n, then

                            (a, b1 , b2 , . . . , bn ) 2 ≤ (a + δ, b1 − δ1 , b2 − δ2 , . . . , bn − δn ) 2 − δ 2 .

                      proof
                                                                                                              
                                         
                                         n                         
                                                                   n                                
                                                                                                    n
                           (a + δ) +2
                                               (bi − δi ) − a −
                                                        2     2
                                                                          bi2 ≥ δ 2 + 2a       δ−         δi       ≥ δ2.
                                         i=1                        i=1                             i=1




                       Let N0 denote the network at the beginning of a phase. Assume that the phase
                    consists of k iterations, and that Nt denotes the network at the end of iteration t. Let ft
                    be a balanced flow in Nt , 0 ≤ t ≤ k.

                      Lemma 5.17        ft is a feasible flow in Nt+1 , for 0 ≤ t < k.

                      proof The lemma follows from the fact that each of the two actions, raising
                      the prices of goods in J or adding an edge as required in Event 2, can only lead
                      to a network that supports an augmented max-flow.

                      Corollary 5.18           γ (Nt ) is monotonically decreasing with t.

                       Let δt denote the minimum surplus of a buyer in the active subgraph in network Nt ,
                    for 0 ≤ t < k; clearly, δ0 = δ.

                      Lemma 5.19 If δt−1 − δt > 0 then there exists an i ∈ H such that γi ( pt−1 ) −
                      γi ( pt ) ≥ δt−1 − δt .

                      proof Consider the residual network R( pt , f ) corresponding to the balanced
                      flow computed at the end of iteration t. By definition of Ht , every vertex v ∈
                      Ht \ Ht−1 can reach a vertex i ∈ Ht−1 in R( pt , f ) and therefore, by Theorem 5.5,
                      γv ( pt ) ≥ γi ( pt ). This means that minimum surplus δt is achieved by a vertex i
                      in Ht−1 . Hence, the surplus of vertex i is decreased by at least δt−1 − δt during
                      iteration t.

                      Lemma 5.20        If δt+1 < δt then γ (Nt ) 2 − γ (Nt+1 ) 2 ≥ (δt − δt+1 )2 , for 0 ≤
                      t < k.

                      proof By Lemma 5.19, if δt+1 < δt then there is a buyer i whose surplus drops
                      by δt − δt+1 in going from ft to ft+1 . By Lemmas 5.15 and 5.17, we get the
                      desired conclusion.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 15, 2007       11:45




                                    the linear case of the arrow–debreu model                              121

                      Lemma 5.21        In a phase, the square of the l2 norm of the surplus vector drops
                      by a factor of
                                                           
                                                                    1
                                                               1−         .
                                                                    n2

                      proof We will first prove that
                                                                           δ2
                                                   γ (N0 ) 2 − γ (Nk ) 2 ≥    .
                                                                            n
                         Observe that the left-hand side can be written as a telescoping sum in which
                      each term is of the form γ (Nt ) 2 − γ (Nt+1 ) 2 . By Corollary 5.18, each of these
                      terms is positive. Consider only those terms in which the difference δt − δt+1 >
                      0. Their sum is minimized when all these differences are equal. Now using
                      Lemma 5.20 and the fact that δ0 = δ and δk = 0, we get that
                                                                         δ2
                                                   γ (N0 ) 2 − γ (Nk ) 2 ≥  .
                                                                         k
                      By Lemma 5.14, k ≤ n, giving the desired inequality.
                        The above-stated inequality and the fact that γ (N0 ) 2 ≤ nδ 2 gives us
                                                                   
                                                                          1
                                            γ (Nk ) 2 ≤ γ (N0 ) 2 1 − 2 .
                                                                         n
                      The lemma follows.

                      Theorem 5.22 The algorithm finds equilibrium prices and allocations for linear
                      utility functions in Fisher’s model using
                                               O(n4 (log n + n log U + log M))
                      max-flow computations.

                      proof By Lemma 5.21, the square of the surplus vector drops by a factor of half
                      after O(n2 ) phases. At the start of the algorithm, the square of the surplus vector is
                      at most M 2 . Once its value drops below 1/4 , the algorithm achieves equilibrium
                      prices. This follows from Lemmas 5.12 and 5.13 Therefore the number of phases
                      is
                                   O(n2 log(4 M 2 ) = O(n2 (log n + n log U + log M)).
                      By Lemma 5.14 each phase consists of n iterations and by Lemma 5.11 each
                      iteration requires n max-flow computations. The theorem follows.


                                5.11 The Linear Case of the Arrow–Debreu Model

                    The Arrow–Debreu model is also known as the Walrasian model or the exchange
                    model, and it generalizes Fisher’s model. Consider a market consisting of a set A of
                    agents and a set G of goods; assume |G| = n and |A| = m. Each agent i comes to the
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    July 15, 2007   11:45




                    122      combinatorial algorithms for market equilibria

                    market with an initial endowment of goods, ei = (ei1 , ei2 , . . . , ein ). Wemay assume
                    w.l.o.g. that the total amount of each good is unit, i.e., for 1 ≤ j ≤ n, m      i=1 eij = 1.
                    Each agent has linear utilities for thesen  goods.  The utility of agent   i on deriving  xij
                    amount of good j , for 1 ≤ j ≤ n, is j =1 uij xij .
                       The problem is to find prices p = (p1 , . . . , pm ) for the goods so that if each agent
                    sells her initial endowment at these prices and buys her optimal bundle, the market
                    clears; i.e., there is no deficiency or surplus of any good. An agent may have more than
                    one optimal bundle; we will assume that we are free to give each agent any optiaml
                    bundle to meet the market clearing condition.
                       Observe that a Fisher market with linear utilities, n goods, and m buyers reduces
                    to an Arrow–Debreu market with linear utilities, n + 1 goods and m + 1 agents as
                    follows. In the Arrow–Debreu market, we will assume that money is the n + 1’st good,
                    the first m agents correspond to the m buyers whose initial endowment is the money
                    they come to the market with and the m + 1’st agent’s initial endowment is all n goods.
                    The first m agents have utilities for goods, as given by the Fisher market and no utility
                    for money, whereas the m + 1’st agent has utility for money only.                  
                       We define the following terms for the algorithm below. For agent i, let ai = m     j =1 eij .
                    Let amin be the minimum among ai , 1 ≤ i ≤ m. Denote by pmax the maximum price
                    assigned to a good by the algorithm. Denote by umin and umax the minimum and
                    maximum values of uij over all agents i and goods j .


                                            5.12 An Auction-Based Algorithm

                    We will present an auction-based algorithm for the linear case of the Arrow–Debreu
                    model. It will find an approximate equilibrium in the following sense. For any fixed
                     > 0, it will find prices p for the goods such that the market clears and each agent
                    gets a bundle of goods that provides her utility at least (1 − )2 times the utility of her
                    optimal bundle.
                        The algorithm initializes the price of each good to be unit, computes the worth of
                    the initial endowment of each agent, and gives this money to each agent. All goods are
                    initially fully unsold.
                        We will denote by p = (p1 , p2 , . . . , pn ) the vector of prices of goods at any point in
                    the algorithm. As p changes, the algorithm recomputes the value of each agent’s initial
                    endowment and updates her money accordingly. Clearly, at the start of the algorithm,
                    the total surplus (unspent) money of all agents is n.
                        At any point in the algorithm, a part of good j is sold at price pj and part of it is
                    sold at (1 + )pj . The run of the algorithm is partitioned into iterations. Each iteration
                    terminates when the price of some good is raised by a factor of (1 + ). Each iteration
                    is further partitioned into rounds. In a round, the algorithm considers agents one by one
                    in some arbitrary but fixed order, say 1, 2, . . . , m. If the agent being considered, i, has
                    no surplus money, the algorithm moves to the next agent. Otherwise, it finds i’s optimal
                    good, in terms of bang per buck, at current prices; say, it is good j . It then proceeds
                    to execute the operation of outbid. This entails buying back good j from agents who
                    have it at price pj and selling it to i at price pj (1 + ). This process can end in one of
                    two ways:
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0       July 15, 2007      11:45




                                                an auction-based algorithm                                   123

                    r Agent i’s surplus money is exhausted. If so, the algorithm moves on to the next agent.
                    r No agent has good j at price pj anymore. If so, it raises the price of good j to pj (1 + )
                      by setting pj to pj (1 + ). The current iteration terminates and agents’ moneys are
                      updated because of this price rise.

                       When the current round comes to an end, the algorithm checks if the total surplus
                    money with the buyers is at most amin . If so, the algorithm terminates. Otherwise, it
                    goes to the next round.
                       At termination, the algorithm gives the unsold goods to an arbitrary agent to en-
                    sure that the market clears. It outputs the allocations received by all agents and the
                    terminating prices p. Observe, however, that some of good j may have been sold at
                    price (1 + )pj even though the equilibrium price of good j is pj . Because of this
                    descrepancy, agents will only get approximately optimal bundles. Lemma 5.25 will
                    establish a bound on the approximation factor.


                      Lemma 5.23        The number of rounds executed in an iteration is bounded by
                                                               
                                                                   1     npmax
                                                           O         log           .
                                                                        amin

                      proof Observe that if outbid buys a good at price pj , it sells it at price (1 +
                      )pj , thereby decreasing the overall surplus. Therefore, in each round that is fully
                      completed (i.e., does not terminate mid-way because of a price increase), the
                      total surplus of agents is reduced by a factor of (1 + ). The total surplus at the
                      beginning of the iteration is at most the total money possessed by all agents, i.e.,
                      npmax . The iteration terminates (and in fact the algorithm terminates) as soon as
                      the total surplus is at most amin . Therefore, a bound on the number of rounds in
                      an iteration is
                                                                        npmax
                                                               log1+         .
                                                                        amin



                      Lemma 5.24        The total number of iterations is bounded by
                                                               
                                                                   n
                                                           O         log pmax .
                                                                   

                      proof Each iteration raises the price of a good by a factor of (1 + ). Therefore
                      the number of iterations is bounded by

                                                               n log1+ pmax .
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    July 15, 2007   11:45




                    124      combinatorial algorithms for market equilibria

                      Lemma 5.25 Relative to terminating prices, each agent gets a bundle of goods
                      that provides her utility at least (1 − )2 times the utility of her optimal bundle.

                      proof The algorithm always sells an agent her optimal goods relative to current
                      prices p (recall, however, that at the time of the sale, an agent is charged a price
                      of (1 + )pj for good j ). There are two reasons why an agent i may end up with a
                      suboptimal bundle in the end. First, at termination, part of her money may remain
                      unspent. Let M denote the total worth of i’s initial endowment at terminating
                      prices. Assume that she spent M1 of this. Since the total surplus money left at
                      termination is at most amin , M1 ≥ (1 − )M.
                         The second reason is that some part of good j may have been sold at price (1 +
                      )pj to agent i, even though the equilibrium price announced is pj . Equivalently,
                      we may assume that i gets her optimal goods at prices p for a fraction of her
                      money. The latter is at least
                                            M1    (1 − )M
                                                ≥          ≥ (1 − )2 M
                                           1+      1+
                      money. The lemma follows.

                      Theorem 5.26 The algorithm given above finds an approximate equilibrium for
                      the linear case of the Arrow–Debreu model in time
                                                 
                                                   mn      nvmax         vmax
                                              O       log            log      .
                                                   2     amin vmin     vmin

                      proof Observe that each good whose price is raised beyond 1 is fully sold.
                      Since the total money of agents is the total worth of all goods at prices p, the
                      condition that the total surplus money of agents is at most amin must be reached
                      before the price of all goods increases beyond 1. Hence at termination, the price
                      of at least one good is 1.
                         Clearly, at termination, the ratio of maximum to minimum price of a good is
                      bounded by vmax /vmin . Therefore, pmax is bounded by vmax /vmin . Each round is
                      executed in O(m) time. Now the bound on the total running time follows from
                      Lemmas 5.23 and 5.24.


                                            5.13 Resource Allocation Markets

                    Kelly considered the following general setup for modeling resource allocation. Let R
                    be a set of resources and c: R → Z+ be the function specifying the available capacity
                    of each resource r ∈ R. Let A = {a1 , . . . , an } be a set of agents and mi ∈ Z+ be the
                    money available with agent ai .
                       Each agent wants to build as many objects as possible using resources in R. An
                    agent may be able to use several different subsets of R to make one object. Let
                    Si1 , Si2 , . . . , Siki be subsets of R usable by agent ai , ki ∈ Z+ . Denote by xij the number
                    of objects ai makes using the subset Sij , 1 ≤ j ≤ ki ; xij is not rquired to be integral.
                                  
                    Let fi = kji=1 xij be the total number of objects made by agent ai . We will say that
P1: SBT
9780521872829main       CUNY1061-Nisan        0 521 87282 0          July 15, 2007     11:45




                                                 resource allocation markets                                           125

                    fi , 1 ≤ i ≤ n is feasible if simultaneously each agent ai can make fi objects without
                    violating capacity constraints on R.
                        Kelly gave the following convex program and showed that an optimal solution to it
                    satisfies proportional fairness; i.e., if fi∗ is an optimal solution and fi is any feasible
                    solution, then

                                                              
                                                              n
                                                                fi − f ∗        i
                                                                                     ≤ 0.
                                                               i=1
                                                                          fi∗

                    Intuitively, the only way of making an agent happier by 5% is to make other agents
                    unhappy by at least a total of 5%.
                                                       
                                      Maximize                mi log fi
                                                      ai ∈A
                                                               
                                                               ki
                                      Subject to       fi =            xij            ∀ai ∈ A
                                                                j =1
                                                                                                                      (5.2)
                                                         
                                                                     xij ≤ c(r)       ∀r ∈ R
                                                       (ij ):r∈Sij
                                                       xij ≥ 0                        ∀ai ∈ A, 1 ≤ j ≤ ki

                       This general setup can be used to model many situations. The following are examples
                    of situations of a combinatorial nature.

                      (i) Market 1 (flow market): Given a directed or undirected graph G = (V , E), E is
                          the set of resources, with capacities specified. Agents are source-sink pairs of nodes,
                          (s1 , t1 ), . . . , (sk , tk ), with money m1 , . . . , mk , respectively. Each si − ti path is an
                          object for agent (si , ti ).
                     (ii) Market 2: Given a directed graph G = (V , E), E is the set of resources, with
                          capacities specified. Agents are A ⊂ V , each with specified money. For s ∈ A objects
                          are branchings rooted at s and spanning all V .
                    (iii) Market 3: Same as above, except the graph is undirected and the objects are spanning
                          trees.

                        Using KKT conditions, one can show that an optimal solution to this convex program
                    is an equilibrium solution. Let pr , r ∈ R be Lagrangian variables corresponding to the
                    second set of conditions; we will interpret these as prices of resources. By the KKT
                    conditions optimal solutions to xij ’s and pr ’s must satisfy the following equilibrium
                    conditions:

                      (i) Resource r ∈ R has positive price only if it is used to capacity.
                     (ii) Each agent uses only the cheapest sets to make objects.
                    (iii) The money of each agent is fully used up.

                       Since the objective function of convex program (5.2) is strictly concave, one can
                    see that at optimality, the vector f1 , . . . , fn is unique. Clearly, this also holds for every
                    equilibrium allocation.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0   July 15, 2007   11:45




                    126      combinatorial algorithms for market equilibria

                            5.14 Algorithm for Single-Source Multiple-Sink Markets

                    In this section, we consider the special case of a flow market, Market 1, with a single
                    source and multiple sinks. We will assume that the underlying graph is directed. In case
                    it is undirected, one can use the standard reduction from undirected graphs to directed
                    graphs – replace each undirected edge (u, v) with the two edges (u, v) and (v, u) of the
                    same capacity.
                        Formally, let G = (V , E) be a directed graph with capacities on edges. Let s ∈ V
                    be the source node and T = {t1 , . . . , tr } be the set of sink nodes, also called terminals.
                    Let mi be the money possessed by sink ti . The problem is to determine equilibrium
                    flow and edge prices. The following example may help appreciate better some of the
                    intricacies of this problem.

                      Example 5.27 Consider graph G = (V , E) with V = {s, a, b, c, d} and sinks
                      b and d with $120 and $10, respectively. The edges are (s, a), (s, c) having
                      capacity 2, (a, b) having capacity 1, and (a, d), (c, d), (c, b) having capacity
                      10 (see Figure 5.5). The unique equilibrium prices are p(s,a) = $10, p(a,b) =
                      $30, p(s,c) = $40, and the rest of the edges have zero price. At equilibrium, flow
                      on path s, a, d is 1, on s, a, b is 1, and on s, c, b is 2. Simulating the algorithm
                      below on this example will reveal the complex sequence of cuts it needs to find
                      in order to compute the equilibrium. Computing equilibrium for other values of
                      money is left as an intersting exercise.

                       We will present a strongly polynomial algorithm for this problem which is based
                    on the primal-dual schema; i.e., it alternately adjusts flows and prices, attempting to
                    satisfy all KKT conditions. Often, primal-dual algorithms can naturally be viewed as
                    executing an auction. This viewpoint is leads to a particularly simple way of presenting
                    the current algorithm. We will describe it as an ascending price auction in which the
                    buyers are sinks and sellers are edges. The buyers have fixed budgets and are trying to
                    maximize the flow they receive and the sellers are trying to extract as high a price as
                    possible from the buyers. One important deviation from the usual auction situation is

                                                               a            1            b
                                                                                             $120

                                                    2
                                                                                    10

                                        s


                                                                                    10
                                                    2

                                                                                             $10
                                                               c            10           d

                                              Figure 5.5. The network for Example 5.27.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    July 15, 2007       11:45




                               algorithm for single-source multiple-sink markets                            127

                    that the sellers act in a highly coordinated manner – at any point in the algorithm, all
                    edges in a particular cut, say (S, S), raise their prices simultaneously while prices of
                    the remaining edges remain unchanged. The prices of all edges are initialized to zero.
                    The first cut considered by the algorithm is the (unique) maximal min-cut separating
                    all sinks from s, say (S0 , S0 ).
                        Denote by rate(ti ) the cost of the cheapest s − ti path w.r.t. current prices. The flow
                    demanded by sink ti at this point is mi /rate(ti ). At the start of the algorithm, when all
                    edge prices are zero, each sink is demanding infinite flow. Therefore, the algorithm
                    will not be able to find a feasible flow that satisfies all demands. Indeed, this will be
                    the case all the way until termination; at any intermediate point, some cuts will need
                    to be oversaturated in order to meet all the demand.
                        The price of edges in cut (S, S) is raised as long as the demand across it exceeds
                    supply; i.e., the cut is oversaturated because of flow demanded by sinks in S. At the
                    moment that demand exactly equals supply, the edges in this cut stop raising prices and
                    declare themselves sold at current prices. This makes sense from the viewpoint of the
                    edges in the cut – if they raise prices any more, demand will be less than supply; i.e.,
                    the cut will be under-saturated, and then these edges will have to be priced at zero!
                        The crucial question is: when does the cut (S, S) realize that it needs to sell itself?
                    This point is reached as soon as there is a cut, say (U, U ), with S ⊂ U , such that the
                    difference in the capacities of the two cuts is precisely equal to the flow demanded by
                    sinks in S − U (see Figure 5.6). Let (U, U ) be the maximal such cut (it is easy to see
                    that it will be unique). If U = V , the algorithm halts. Otherwise, cut (U, U ) must be
                    oversaturated – it assumes the role of (S, S) and the algorithm goes to the next iteration.
                        Note that an edge may be present in more than one cut whose price is raised by the
                    algorithm. If so, its price will be simply the sum of the prices assigned to these cuts.
                        Suppose that the algorithm executes k iterations. Let (Si , Si ) be the cut it finds in
                    iteration i, 1 ≤ i ≤ k, with Sk = V . Clearly, we have S0 ⊂ S1 ⊂ · · · ⊂ Sk = V . Let Ti
                    be the set of terminals in Si − Si−1 , for 1 ≤ i ≤ k. Let ci be the set of edges of G in
                    the cut (Si , Si ), for 0 ≤ i < k and pi be the price assigned to edges in ci . Clearly, for
                    each terminal t ∈ Ti , rate (t) = p0 + · · · + pi−1 , for 1 ≤ i ≤ k.




                                            s                     t2

                                                                       t3




                                                     Cut(S, S )             Cut(U, U )

                    Figure 5.6. The total flow demanded by t2 and t3 equals the difference in capacities of cut
                    (S, S) and cut (U, U).
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 15, 2007    11:45




                    128      combinatorial algorithms for market equilibria

                       Let G denote the graph obtained by adding a new sink node t to G and edges (ti , t)
                    from each of the original sinks to t. Let the capacity of edge (ti , t) be mi /rate(ti ). For
                    convenience, even in G , we will denote V − S by S. It is easy to see that each of the
                    cuts (Si , Si ∪ {t}) in G has the same capacity, for 0 ≤ i ≤ k, and each of these k + 1
                    cuts is a mininimum s − t cut in G .
                       Let f  denote a maximum s − t flow in G . Obtain flow f from f  by ignoring flow
                    on the edges into t. Then f is a feasible flow in G that sends mi /rate(ti ) flow to each
                    sink ti .

                      Lemma 5.28 Flow f and the prices found by the algorithm constitute an
                      equilibrium flow and prices.

                      proof We will show that flow f and the prices found satisfy all KKT condi-
                      tions.
                       r Since each of the cuts (S , S ∪ {t}), for 0 ≤ i < k is saturated in G by flow f  ,
                                                  i   i
                          each of the cuts c0 , c1 , . . . , ck−1 is saturated by f . Hence, all edges having nonzero
                          prices must be saturated.
                       r The cost of the cheapest path to terminal t  ∈ T is rate(t  ). Clearly, every flow to t 
                         uses a path of this cost.
                       r Since the flow sent to t  ∈ T is mi /rate(t  ), the money of each terminal is fully
                          spent.


                       Below we give a strongly polynomial time subroutine for computing the next cut in
                    each iteration.


                                                   5.14.1 Finding the Next Cut
                    Let (S, S) be the cut in G, whose price is being raised in the current iteration and let c
                    be the set of edges in this cut and f its capacity. Let T  denote the set of sinks in S. Let
                    p  denote the sum of the prices assigned to all cuts found so far in the algorithm (this
                    is a constant for the purposes of this subroutine) and let p denote the price assigned to
                    edges in c. The cut (S, S) satisfies the following conditions:
                    r It is a maximal minimum cut separating T  from s.
                    r At p = 0, every cut (U, U ), with S ⊆ U , is oversaturated.

                       Let p ∗ be the smallest value of p at which there is a cut (U, U ), with S ⊂ U , in G
                    such that the difference in the capacities of (S, S) and (U, U ) is precisely equal to the
                    flow demanded by sinks in U − S at prices p ∗ ; moreover, (U, U ) is the maximal such
                    cut. Below we give a strongly polynomial algorithm for finding p ∗ and (U, U ).
                       Define graph G by adding a new sink node t to G and edges (ti , t) for each sink
                    ti ∈ S. Define the capacity of edge (ti , t) to be mi /(p  + p) where mi is the money of
                    sink ti (see Figure 5.7). As in Section 5.14 we will denote V − S by S even in G . The
                    proof of the following lemma is obvious.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0          July 15, 2007    11:45




                                 algorithm for single-source multiple-sink markets                                129

                                                                                                          t
                                                                                                         mi
                                                                                                     p        p




                             s
                                                             t5       t2    t3             t7   ti




                                             Cut(S, S )                           Cut(U, U )
                                             priced at p

                                                           Figure 5.7. Graph G .



                      Lemma 5.29 At the start of the current iteration, (S, S ∪ {t}) is a maximal
                      minimum s − t cut in G . p ∗ is the smallest value of p at which a new minimum
                      s − t cut appears in G . (U, U ∪ {t}) is the maximal minimum s − t cut in G at
                      price p ∗ .

                       For any cut C in G , let capp (C) denote its capacity, assuming that the prices of edges
                    in c is p. For p ≥ 0, define cut(p) to be the maximal s − t min-cut in G assuming
                    that the price assigned to edges in c is p. For cut (A, A ∪ {t}), A ⊆ V , let price(A, A ∪
                    {t}) denote the smallest price that needs to be assigned to edges in c to ensure that
                    capp (A, A ∪ {t}) = f ; i.e., (A, A ∪ {t}) is also a min s − t cut in G ; if (A, A ∪ {t})
                    cannot be made a minimum s − t cut for any price p then price(A, A ∪ {t}) = ∞.
                    Clearly, price(A, A ∪ {t}) ≥ p∗ . Observe that determining price(A, A ∪ {t}) involves
                    simply solving an equation in which p is unknown.

                      Lemma 5.30 Suppose p > p ∗ . Let cut(p) = (A, A ∪ {t}), where A = U . Let
                      price(A, A ∪ {t}) = q and cut(q) = (B, B ∪ {t}). Then B ⊂ A.

                      proof Since we have assumed that A = U , it must be the case that
                      capp (A, A ∪ {t}) > f . Therefore, q = price(A, A ∪ {t}) < p. Let cA and cB de-
                      note the capacities of (A, A ∪ {t}) and (B, B ∪ {t}) at price p = 0. Let mA and
                      mB denote the money possessed by sinks in (A − S) and (B − S), respectively.
                         Since (A, A ∪ {t}) is a maximal s − t mincut at price p,
                                                                   mA        mB
                                                      cA +            < cB +    .
                                                                   p         p
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0    July 15, 2007   11:45




                    130      combinatorial algorithms for market equilibria


                     Subroutine
                     Inputs: Cut (S, S) in G whose price is being raised in the current iteration.
                     Output: Price p ∗ and next cut (U, U ).

                      (i) C ← (V , t)
                     (ii) p ← price(C)
                    (iii) While cut(p) = C do:
                          (a) C ← cut(p)
                          (b) p ← price(C)
                    (iv) Output (C, p)

                                             Figure 5.8. Subroutine for finding next cut.


                      Since (B, B ∪ {t}) is a maximal s − t mincut at price q,
                                                             mB        mA
                                                      cB +      < cA +    .
                                                              q        q

                      The two together imply

                                               mB − mA             mB − mA
                                                       < cA − cB <         .
                                                  q                   p

                         First suppose that A ⊂ B. Clearly mA ≤ mB . But this contradicts the last
                      inequality since q < p.
                         Next, suppose that A and B cross. By the last inequality above, there must be a
                      price, r, such that q < r < p at which capr (A, A ∪ {t}) = capr (B, B ∪ {t}) = g,
                      say. By the submodularity of cuts, one of the following must hold:
                       (i) capr ((A ∩ B), (A ∩ B) ∪ {t}) ≤ g. Since the money possessed by sinks in (A ∩
                           B) − S is at most mB , at price q, capq ((A ∩ B), (A ∩ B){t}) < capq (B, B ∪ {t}).
                           This contradicts the fact that (B, B ∪ {t}) is a min-cut at price q.
                      (ii) capr ((A ∪ B), (A ∪ B) ∪ {t}) ≤ g. Since the money possessed by sinks in (A ∪
                           B) − S is at least mA , at price p, capp ((A ∪ B), (A ∪ B) ∪ {t}) < capp (A, A ∪
                           {t}). This contradicts the fact that (A, A ∪ {t}) is a min-cut at price p.
                          Hence we get that B ⊂ A.

                      Lemma 5.31 Subroutine 5.8 terminates with the cut (U, U ∪ {t}) and price p ∗
                      in at most r max-flow computations, where r is the number of sinks.

                      proof As long as p > p ∗ , by Lemma 5.30, the algorithm keeps finding smaller
                      and smaller cuts, containing fewer sinks on the s side. Therefore, in at most r
                      iterations, it must arrive at a cut such that p = p∗ . Since cut(p ∗ ) = (U, U ∪ {t}),
                      the next cut it considers is (U, U ∪ {t}). Since price(U, U ∪ {t}) = p ∗ , at this
                      point the algorithm terminates.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 15, 2007   11:45




                                               discussion and open problems                                  131

                      Theorem 5.32 The algorithm given in Section 5.14 finds equilibrium edge
                      prices and flows using O(r 2 ) max-flow computations, where r is the number of
                      sinks.

                      proof Clearly, the number of sinks trapped in the sets S0 ⊂ S1 ⊂ · · · ⊂ Sk
                      keeps increasing and therefore, the number of iterations k ≤ r. The running time
                      for each iteration is dominated by the time taken by subroutine (5.8), which
                      by Lemma 5.31 is r max-flow computations. Hence the total time taken by the
                      algorithm is O(r 2 ) max-flow computations. By Lemma 5.28 the flow and prices
                      found by the algorithm are equilibrium flow and prices.


                                          5.15 Discussion and Open Problems

                    Linear utility functions provided us with perhaps the easiest algorithmic questions that
                    helped us commence our algorithmic study of market equilibria. However, such func-
                    tions are much too restrictive to be useful. Concave utility functions are considered
                    especially useful in economics because they model the important condition of decreas-
                    ing marginal utilities as a function of the amount of good obtained. Furthermore, if
                    the utility functions are strictly concave, at any given prices, there is a unique optimal
                    bundle of goods for each agent. This leads to the following remarkable communication
                    complexity fact: In such a market, it suffices to simply announce equilibrium prices –
                    then, all agents can individually compute and buy their optimal bundles and the market
                    clears!
                        On the other hand, concave utility functions, even if they are additively separable
                    over the goods, are not easy to deal with algorithmically. In fact, obtaining a polynomial
                    time algorithm for such functions is a premier open problem today. For the case of
                    linear functions, the approach used in Section 5.8 – of starting with very low prices and
                    gradually raising them until the equilibrium is reached – is made possible by the prop-
                    erty of weak gross substitutability. This property holds for a utility function if on raising
                    the price of one good, the demand of another good cannot go down. As a consequence
                    of this property, the need to decrease the price of the second good does not arise.
                        Concave utility functions do not satisfy weak gross substitutability. Exercises 5.5
                    and 5.6 outline an approach that attempts to finesse this difficulty for the case of
                    piecewise-linear, concave functions. Does this approach lead to an efficient algorithm
                    for computing, either exactly or approximately, equilibrium prices for such functions?
                    If so, one can handle a concave function by approximating it with a piecewise-linear,
                    concave function. Alternatively, can one show that finding an equilibrium for such
                    utility functions is PPAD-hard?
                        Considering the properties of the linear case of Fisher’s model established in
                    Theorem 5.1, one wonders whether its equilibrium allocations can be captured via
                    a linear program. Resolving this, positively or negatively, seems an exciting problem.
                    Another question remaining open is whether there is a strongly polynomial algorithm
                    for computing equilibrium prices for this case. Finally, we would like to point to the
                    numerous questions remaining open for gaining a deeper algorithmic understanding of
                    Eisenberg–Gale markets (Jain and Vazirani, 2006).
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0     July 15, 2007   11:45




                    132       combinatorial algorithms for market equilibria

                                                         Acknowledgments

                    I wish to thank Deeparnab Chakrabarty, Nikhil Devanur, Sergei Izmalkov, Kamal Jain
                    and Kasturi Vardarajan for valuable discussions and comments on the writeup.



                                                             Bibliography
                    K. Arrow and G. Debreu. Existence of an equilibrium for a competitive economy. Econometrica,
                       22:265–290, 1954.
                    W.C. Brainard and H.E. Scarf. How to compute equilibrium prices in 1891. Cowles Foundation
                       Discussion Paper, (1270) 2000.
                    X. Deng, C. Papadimitriou, and S. Safra. On the complexity of equilibria. In Proc. ACM Symp. on
                       Theor. Comp., 2002.
                    N. Devanur, C.H. Papadimitriou, A. Saberi, and V.V. Vazirani. Market equilibrium via a primal-dual-
                       type algorithm. In Proc. IEEE Annual Symp. Fdns. of Comp. Sci., 2002. To appear in J. ACM.
                       Journal version available at: http://www-static.cc.gatech.edu/vazirani/market.ps.
                    N. Devanur and V.V. Vazirani. The spending constraint model for market equilibrium: Algorithmic,
                       existence and uniqueness results. In Proc. 36th Symp. on Theory of Computing, 2004.
                    J. Edmonds. Maximum matching and a polyhedron with 0,1-vertices. J. Res. Natl. Bur. Standards,
                       69:125–130, 1965.
                    J. Edmonds. Optimum branchings. J. Res. Natl. Bur. Standards, Section B, 71:233–240, 1967.
                    E. Eisenberg and D. Gale. Consensus of subjective probabilities: The Pari-Mutuel method. Annals
                       Math. Stat., 30:165–168, 1959.
                    S. Floyd and V. Jacobson. Random early detection gateways for congestion avoidance. IEEE/ACM
                       Trans. Networking, 1(1):397–413, 1993.
                    R. Garg and S. Kapoor. Auction algorithms for market equilibrium. In Proc. 36th Symp. on Theory
                       of Computing, 2004.
                    V. Jacobson. Congestion avoidance and control. In ACM SIGCOMM, pp. 314–329, 1988.
                    K. Jain and V.V. Vazirani. Eisenberg-gale markets: Algorithms and structural properties. In Proc.
                       39th Symp. on Theory of Computing, 2007.
                    F.P. Kelly. Charging and rate control for elastic traffic. Euro. Trans. on Telecomm., 8:33–37, 1997.
                    F.P. Kelly, A.K. Maulloo, and D.K.H. Tan. Rate control in communication networks. J. Oper. Res.
                       Soc., 49:237–252, 1998.
                    F.P. Kelly and V.V. Vazirani. Rate control as a market equilibrium. Unpublished manuscript 2002.
                       Available at: http://www-static.cc.gatech.edu/vazirani/KV.pdf.
                    S. Low and D. Lapsley. Optimization flow control, 1: basic algorithm and convergence. IEEE/ACM
                       Trans. Networking, 7(6):861–874, 1999.
                    C.S.J.A. Nash-Williams. Edge-disjoint spanning trees of finite graphs. J. London Math. Soc., 36:445–
                       450, 1961.
                    H. Scarf. The Computation of Economic Equilibria (with collaboration of T. Hansen). Cowles Foun-
                       dation Monograph No. 24., New Haven: Yale University Press, 1973.
                    W.T. Tutte. On the problem of decomposing a graph into n connected factors. J. London Math. Soc.,
                       36:221–230, 1961.
                    V.V. Vazirani. Spending constraint utilities, with applications to the Adwords market. Submitted to
                       Math. of Operations Research, 2006.
                    L. Walras. Éléments d’économie politique pure ou théorie de la richesse sociale (Elements of Pure
                       Economics, or the theory of social wealth). Lausanne, Paris, 1874. (1899, 4th ed.; 1926, rev ed.,
                       1954, Engl. transl.).
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 15, 2007   11:45




                                                              exercises                                          133

                    J. Wang, L. Li, S.H. Low, and J.C. Doyle. Cross-layer optimization in TCP/IP networks. IEEE/ACM
                       Trans. Networking, 13:582–268, 2005.



                                                              Exercises
                    5.1 Give a strongly polynomial algorithm for Fisher’s linear case under the assumption
                        that all ui j ’s are 0/1 (the algorithm given in Section 5.8 is not strongly polynomial).
                    5.2 Let us extend Fisher’s linear model to assume that buyers have utility for money
                        (Vazirani, 2006). Let ui 0 denote the utility accrued by buyer i for one unit of money.
                        Now, each buyer’s optimal bundle can also include money—effectively this is part
                        of their own money which they prefer not to spend at current prices. The notion of
                        equilibrium also generalizes—all goods need to be sold and all money needs to be
                        either spent or returned as part of optimal bundles. Extend the algorithm given in
                        Section 5.8 to this situation, still maintaining its polynomial running time.
                    5.3 Let us define a new class of utility functions, spending constraint utility functions
                        for Fisher’s model (Vazirani, 2006). As before, let A and B be the set of goods and
                        buyers, respectively. For i ∈ B and j ∈ A, let r ij : [0, e(i )] → R+ be the rate function
                        of buyer i for good j ; it specifies the rate at which i derives utility per unit of j
                        received, as a function of the amount of her budget spent on j . If the price of j is
                        fixed at p j per unit amount of j , then the function r ij / p j gives the rate at which i
                        derives utility per dollar spent, as a function of the amount of her budget spent on
                         j.
                            Relative to prices p for the goods, give efficient algorithms for

                         (a) computing buyer i ’s optimal bundle,
                         (b) determining if p are equilibrium prices, and
                         (c) computing equilibrium allocations if p are equilibrium prices.

                    5.4 Prove that equilibrium prices are unique for the model of Exercise 5.3.
                    5.5 It turns out that there is a polynomial time algorithm for computing equilibrium
                        prices and allocations for the utility functions defined in Exercise 5.3 (Devanur and
                        Vazirani, 2004; Vazirani, 2006). The following is an attempt to use this algorithm
                        to derive an algorithm for computing equilibrium prices for the case of piecewise-
                        linear, concave utility functions for Fisher’s model.
                             Let f i j be the piecewise-linear, concave utility function of buyer i for good j ; f i j
                        is a function of xi j , the allocation of good j to buyer i . Let p be any prices of goods
                        that sum up to the total money possessed by buyers (as before, we will assume that
                        there is a unit amount of each good in the market).
                             Let us obtain spending constraint utility functions from the f i j ’s as follows. Let
                        gi j be the derivative of f i j ; clearly, gi j is a decreasing step function. Define
                                                                                
                                                                                  yi j
                                                               hi j (yi j ) = g        ,
                                                                                  pi j
                          where yi j denotes the amount of money spent by i on good j . Observe that function
                          hi j gives the rate at which i derives utility per unit of j received as a function of the
                          amount of money spent on j . Hence hi j is precisely a spending constraint utility
                          function. Let us run the algorithm mentioned above on these functions hi j ’s to obtain
                          equilibrium prices, say p .
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 15, 2007   11:45




                    134      combinatorial algorithms for market equilibria

                             Show that p = p iff prices p are equilibrium prices for the piecewise-linear, con-
                          cave utility functions f i j ’s (equilibrium prices for piecewise-linear, concave utility
                          functions need not be unique).
                    5.6 Open problem (Devanur and Vazirani, 2004): Consider the process given in Exercise
                        5.3, which, given starting prices p, finds new prices p . By the assertion made in
                        Exercise 5.3, the fixed points of this process are precisely equilibrium prices for the
                        piecewise-linear, concave utility functions f i j ’s.
                           Does this procedure converge to a fixed point, and if so, how fast? If it does
                        not converge fast enough, does it converge quickly to an approximate fixed point,
                        which may be used to obtain approximate equilibrium prices?
                    5.7 Consider the single-source multiple-sink market for which a strongly polynomial
                        algorithm is given in Section 5.14. Obtain simpler algorithms for the case that the
                        underlying graph is a path or a tree.
                    5.8 Observe that the algorithm given in Section 5.14 for Market 1 defined in Section
                        5.13 uses the max-flow min-cut theorem critically (Jain and Vazirani, 2006). Obtain
                        a strongly polynomial algorithm for Market 3 using the following max–min theorem.
                            For a partition V1 , . . . , Vk , k ≥ 2 of the vertices of an undirected graph G, let C
                        be the capacity of edges whose end points are in different parts. Let us define the
                        edge-tenacity of this partition to be C/(k − 1), and let us define the edge-tenacity
                        of G to be the minimum edge-tenacity over all partitions. Nash-William (1961) and
                        Tutte (1961) proved that the maximum fractional packing of spanning trees in G is
                        exactly equal to its edge-tenacity.
                    5.9 Next consider Market 2 defined in Section 5.13. For the case |A| = 1, a polynomial
                        time algorithm follows from the following max–min theorem due to Edmonds (1967).
                            Let G = (V , E ) be a directed graph with edge capacities specified and source
                        s ∈ V . The maximum number of branchings rooted out of s that can be packed in
                        G equals minv∈V c(v), where c(v) is the capacity of a minimum s − v cut.
                            Next assume that there are two agents, s1 , s2 ∈ V . Derive a strongly polynomial
                        algorithm for this market using the following fact from Jain and Vazirani (2006). Let
                        F 1 and F 2 be capacities of a minimum s1 − s2 and s2 − s1 cut, respectively. Let F be
                        minv∈V −{s1 ,s2 } f  (v), where f  (v) is the capacity of a minimum cut separating v from
                        s1 and s2 . Then:
                          (a) The maximum number of branchings, rooted at s1 and s2 , that can be packed in
                              G is exactly min{F 1 + F 2 , F }.
                          (b) Let f 1 and f 2 be two nonnegative real numbers such that f 1 ≤ F 1 , f 2 ≤ F 2 , and
                               f 1 + f 2 ≤ F . Then there exists a packing of branchings in G with f 1 of them
                              rooted at s1 and f 2 of them rooted at s2 .
