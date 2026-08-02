---
type: "book-chapter"
book_id: "algorithmic-game-theory"
chapter_id: "ch-12"
chapter_number: 12
chapter_title: "Chapter 12"
source_pdf: "raw/inbox/manual-drop/PDF_B.pdf"
source_page_start: 322
source_page_end: 351
printed_page_start: 322
printed_page_end: 351
part_ids: ["algorithmic-game-theory-ch-12-part-013"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Chapter 12

P1: SBT
9780521872829main       CUNY1061-Nisan       0 521 87282 0     August 3, 2007     17:17




                                                             CHAPTER 12


                                  Computationally Efficient
                                 Approximation Mechanisms

                                                                Ron Lavi




                                                                 Abstract

                    We study the integration of game theoretic and computational considerations. In particular, we study
                    the design of computationally efficient and incentive compatible mechanisms, for several different
                    problem domains. Issues like the dimensionality of the domain, and the goal of the algorithm designer,
                    are examined by providing a technical discussion on four results: (i) approximation mechanisms
                    for single-dimensional scheduling, where truthfulness reduces to a simple monotonicity condition;
                    (ii) randomness as a tool to resolve the computational vs. incentives clash for Combinatorial Auctions,
                    a central multidimensional domain where this clash is notable; (iii) the impossibilities of determin-
                    istic dominant-strategy implementability in multidimensional domains; and (iv) alternative solution
                    concepts that fit worst-case analysis, and aim to resolve the above impossibilities.




                                                          12.1 Introduction

                    Algorithms in computer science, and Mechanisms in game theory, are very close in
                    nature. Both disciplines aim to implement desirable properties, drawn from “real-life”
                    needs and limitations, but the resulting two sets of properties are completely different.
                    A natural need is then to merge them – to simultaneously exhibit “good” game theoretic
                    properties as well as “good” computational properties. The growing importance of the
                    Internet as a platform for computational interactions only strengthens the motivation
                    for this.
                       However, this integration task poses many difficult challenges. The two disciplines
                    clash and contradict in several different ways, and new understandings must be ob-
                    tained to achieve this hybridization. The classic Mechanism Design literature is rich
                    and contains many technical solutions when incentive issues are the key goal. Quite
                    interestingly, most of these are not computationally efficient. In parallel, most existing
                    algorithmic techniques, answering the computational questions at hand, do not yield
                    the game theoretic needs. There seems to be a certain clash between classic algorith-
                    mic techniques and classic mechanism design techniques. This raises many intriguing
                                                                     301
P1: SBT
9780521872829main      CUNY1061-Nisan    0 521 87282 0   August 3, 2007   17:17




                    302       computationally efficient approximation mechanisms

                    questions: In what cases this clash is fundamental – a mathematical impossibility?
                    Alternatively, can we “fix” this clash by applying new techniques? We will try to give
                    a feel for these issues.
                       The possibility of constructing mechanisms with desirable computational proper-
                    ties turns out to be strongly related to the dimensionality of the problem domain.
                    In single-dimensional domains, the requirement for game-theoretic truthfulness re-
                    duces to a convenient algorithmic monotonicity condition that leaves ample flexibility
                    for the algorithm designer. We demonstrate this in Section 12.2, were we study the
                    construction of computationally efficient approximation mechanisms for the classic
                    machine scheduling problem. Although there exists a rich literature on approximation
                    algorithms for this problem domain, quite remarkably none of these classic results
                    satisfy the desired game-theoretic properties. We show that when the scheduling prob-
                    lem is single-dimensional, then this clash is not fundamental, and can be successfully
                    resolved.
                       The problem domain of job scheduling has one additional interesting aspect that
                    makes it worth studying: it demonstrates a key difference between economics and
                    computer science, namely the goals of algorithms vs. the goals of classic mechanisms.
                    While the economics literature mainly studies welfare and/or revenue maximization,
                    computational models raise the need for completely different objectives. In scheduling
                    problems, a common objective is to minimize the load on the most loaded machine. As
                    is usually the case, existing techniques for incentive-compatible mechanism design do
                    not fit such an objective (and, on the other hand, most existing algorithmic solutions do
                    not yield the desired incentives). The resolution of these clashes has led to insightful
                    techniques, and the technical exploration of Section 12.2 serves as an example.
                       As opposed to single-dimensional domains, multi-dimensionality seems to pose
                    much harder obstacles. In Chapter 9, the monotonicity conditions that characterize
                    truthfulness for multidimensional domains were discussed, but it seems that these
                    conditions do not translate well to algorithmic constructions. This issue will be handled
                    in the rest of the chapter, and will be approached in three different ways: we will
                    explore the inherent impossibilities that the required monotonicity conditions cast
                    on deterministic algorithmic constructions, we will introduce randomness to solve
                    these difficulties, and we will consider alternative notions to the solution concept of
                    truthfulness.
                       Our main example for a multidimensional domain will be the domain of combina-
                    torial auctions (CAs). Chapter 11 studies CAs mostly from a computational point of
                    view, and in contrast our focus is on designing computationally efficient and incentive
                    compatible CAs. This demonstrates a second key difference between economics and
                    computer science, namely the requirement for computational efficiency. Even if our
                    goal is the classic economic goal of welfare maximization, we cannot use Vickrey–
                    Clarke–Groves mechanisms (which classically implement this goal) since in many
                    cases they are computationally inefficient. The domain of CAs captures exactly this
                    point, and the need for computationally efficient techniques that translate algorithms to
                    mechanisms is central. In Section 12.3 we will see how randomness can help. We de-
                    scribe a rather general technique that uses randomness and linear programming in order
                    to convert algorithms to truthful-in-expectation mechanisms. Thus we get a positive
                    answer to the computational clash, by introducing randomness.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   August 3, 2007   17:17




                                     single-dimensional domains: job scheduling                              303

                        In Section 12.4 we return to deterministic settings and to the classic definition
                    of deterministic truthfulness, and study the impossibilities associated with it. Our
                    motivating question is whether the three requirements (i) deterministic truthfulness,
                    (ii) computational efficiency, and (iii) nontrivial approximation guarantees, clash in a
                    fundamental and well-defined way. We already know that single dimensionality does
                    not exhibit such a clash, and in this section we describe the other extreme. If a domain
                    has full dimensionality (in a certain formal sense, to be discussed in the section body),
                    then any truthful mechanism must be VCG. It is important to remark that this result fur-
                    ther emphasizes our lack of knowledge about the state of affairs for all the intermediate
                    range of multidimensional domains, to which CAs and its different variants belong.
                        As was motivated in previous chapters, the game-theoretic quest should start with the
                    solution concept of “implementation in dominant strategies,” and indeed most of this
                    chapter follows this line of thought. However, to avoid the impossibilities mentioned
                    earlier, we have to deepen our understandings about the alternatives at hand. Studies
                    in economics usually turn to the solution concept of Bayesian–Nash that requires
                    strong distributional assumptions, namely that the input distributions are known, and,
                    furthermore, that they are commonly known, and agreed upon. Such assumptions seem
                    too strong for CS settings, and criticism about these assumptions have been also raised
                    by economists (e.g., “Wilson’s doctrine”). We have already seen that randomization,
                    and truthful-in-expectation in particular, can provide a good alternative. We conclude
                    the chapter by providing an additional example, of a deterministic alternative solution
                    concept, and describe a deterministic CA that uses this notion to provide nontrivial
                    approximation guarantees.
                        Let us mention two other types of GT-versus-CS clashes, not studied in this chap-
                    ter, to complete the picture. Different models: Some CS models have a significantly
                    different structure, which causes the above-mentioned clash even when traditional ob-
                    jectives are considered. In online computation, for example, players arrive over time,
                    a fundamentally different assumption than classic mechanism design. The difficulties
                    that emerge, and the novel solutions proposed, are discussed in Chapter 16. Differ-
                    ent analysis conventions: CS usually employs worst-case analysis, avoiding strong
                    distributional assumptions, while in economics, the underlying distribution is usually
                    assumed. This greatly affects the character of results, and the reader is referred to, e.g.,
                    Chapter 13 for a broader discussion.

                                 12.2 Single-Dimensional Domains: Job Scheduling

                    As a first example for the interaction between game theory and algorithmic theory, we
                    consider single-dimensional domains. Simple single-dimensional domains were intro-
                    duced in Chapter 9, where every alternative is either a winning or a losing alternative
                    for each player. Here we discuss a more general case. Intuitively, single dimensionality
                    implies that a single parameter determines the player’s valuation vector. In Chapter 9,
                    this was simply the value for winning, but less straight-forward cases also make sense:
                    Scheduling related machines. In this domain, n jobs are to be assigned to m machines,
                    where job j consumes pj time-units, and machine i     has speed si . Thus machine i
                    requires pj /si time-units to complete job j . Let li = j | j is assigned to i pj be the load
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   August 3, 2007   17:17




                    304        computationally efficient approximation mechanisms

                    on machine i. Our schedule aims to minimizes the term maxi li /si , (the makespan).
                    Each machine is a selfish entity, incurring a constant cost for every consumed time unit
                    (and w.l.o.g. assume this cost is 1). Thus the utility of a machine from a load li and
                    a payment Pi is −li /si − Pi . The mechanism designer knows the processing times of
                    the jobs and constructs a scheduling mechanism.
                       Although here the set of alternatives cannot be partitioned to “wins” and “loses,”
                    this is clearly a single-dimensional domain.

                      Definition 12.1 (single-dimensional linear domains) A domain Vi of player
                      i is single-dimensional and linear if there exist nonnegative real constants (the
                      “loads”) {qi,a }a∈A such that, for any vi ∈ Vi , there exists c ∈ − (the “cost”) such
                      that vi (a) = qi,a · c.

                       In other words, the type of a player is simply her cost c, as disclosing it gives us the
                    entire valuation vector. Note that the scheduling domain is indeed single-dimensional
                    and linear: the parameter c is equal to 1/si , and the constant qi,a for alternative a is the
                    load assigned to i according to a.
                       A natural symmetric definition exists for value-maximization (as opposed to cost-
                    minimization) problems, where the types are nonnegative.
                       We aim to design a computationally efficient approximation algorithm, that is also
                    implementable. As the social goal is a certain min–max criterion, and not to minimize
                    the sum of costs, we cannot use the general VCG technique. Since we have a convex
                    domain, Chapter 9 tells us that we need a “weakly monotone” algorithm. But what
                    exactly does this mean? Luckily, the formulation of weak monotonicity can be much
                    simplified for single-dimensional domains.
                       If we fix the costs c−i declared by the other players, an algorithm for a single-
                    dimensional linear domain determines the load qi (c) of player i as a function of her
                    reported cost c. Take two possible types c and c , and suppose c > c. Then the weak
                    monotonicity condition from Chapter 9 reduces to −qi (c )(c − c) ≥ −qi (c)(c − c),
                    which holds iff qi (c ) ≤ qi (c). Hence from Chapter 9 we know that such an algorithm is
                    implementable if and only if its load functions are monotone nonincreasing. Figure 12.1
                    describes this, and will help us figure out the required prices for implementability.




                                               Figure 12.1. A monotone load curve.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   August 3, 2007   17:17




                                     single-dimensional domains: job scheduling                              305
                                                                            c
                        Suppose that we charge a payment of Pi (c) = 0 [qi (x) − qi (c)] dx from player i
                    if he declares a cost of c. Using Figure 12.1, we can easily verify that these prices
                    lead to incentive compatibility: Suppose that player i’s true cost is c. If he reports the
                    truth, his utility is the entire area below the load curve up to c. Now if he declares
                    some c > c, his utility will decrease by exactly the area marked by A: his cost from
                    the resulting load will indeed decrease to c · qi (c ), but his payment will increase to be
                    the area between the line qi (c ) and the load curve. On the other hand, if the player
                    will report c < c, his utility will decrease by exactly the area marked by B, since his
                    cost from the resulting load will increase to c · qi (c ). Thus these prices satisfy the
                    incentive-compatibility inequalities, and in fact this is a simple direct proof for the
                    sufficiency of load monotonicity for this case.
                        The above prices do not satisfy individual rationality, since a player always incurs
                                                                                          ∞ exercise is to add
                    a negative utility if we use these prices. To overcome this, the usual
                    a large enough constant to the prices, which in our case can be 0 qi (x) dx. Note that
                    if we add this to the above prices we get that a player that does not receive any load
                    (i.e., declares a cost of infinity) will have azero utility, and in general the utility of a
                                                                     ∞
                    truthful player will be nonnegative, exactly c qi (x) dx. From all the above we get the
                    following theorem.
                      Theorem 12.2 An algorithm for a single-dimensional linear domain is imple-
                      mentable if and only if its load functions are nonincreasing. Furthermore, if this
                      is the case then charging from every player i a price
                                                  c                         ∞
                                        Pi (c) =     [qi (x) − qi (c)] dx −     qi (x) dx
                                                    0                         c
                      will result in an individually rational dominant strategy implementation.
                        In the application to scheduling, we will construct a randomized mechanism, as well
                    as a deterministic one. In the randomized case, we will employ truthfulness in expec-
                    tation (see Chapter 9, Definition 9.27). One should observe that, from the discussion
                    above, it follows that truthfulness in expectation is equivalent to the monotonicity of
                    the expected load.

                            12.2.1 A Monotone Algorithm for the Job Scheduling Problem
                    Now that we understand the exact form of an implementable algorithm, we can con-
                    struct one that approximates the optimal outcome. In fact, the optimum itself is imple-
                    mentable, since it can satisfy weak monotonicity (see the exercises for more details),
                    but the computation of the optimal outcome is NP-hard. We wish to construct effi-
                    ciently computable mechanisms, and hence design a monotone and polynomial-time
                    approximation algorithm. Note that we face a “classic” algorithmic problem – no
                    game-theoretic issues are left for us to handle.
                       Before we start, let us assume that jobs and machines are reordered so that s1 ≥
                    s2 ≥ · · · ≥ sm and p1 ≥ p2 ≥ · · · ≥ pn . For the algorithmic construction, we first need
                    to estimate the optimal makespan of a given instance.
                    Estimating the optimal makespan. Fix a job-index j , and some target makespan T .
                    If a schedule has makespan at most T , then it must assign any job out of 1, . . . , j to a
P1: SBT
9780521872829main      CUNY1061-Nisan           0 521 87282 0          August 3, 2007   17:17




                    306        computationally efficient approximation mechanisms

                    machine i such that T ≥ pj /si . Let i(j, T ) = max{i | T ≥ pj /si }. Thus any schedule
                    with makespan at most T assigns jobs 1, . . . , j to machines 1, . . . , i(j, T ). From space
                    considerations, it immediately follows that
                                                               j
                                                                  k=1 pk
                                                        T ≥ i(j,T    )
                                                                         .                                 (12.1)
                                                                  l=1 sl

                       Now define
                                                                              j      
                                                                         pj    k=1 pk
                                                            Tj = min max    , i                            (12.2)
                                                                  i      si     l=1 sl


                      Lemma 12.3              For any job-index j , the optimal makespan is at least Tj .

                      proof Fix any T < Tj . We prove that T violates 12.1, hence cannot be any
                      feasible makespan, and the claim follows. Let ij be the index that determines Tj .
                      The left expression in the max term is increasing with i, while the right term is
                      decreasing. Thus ij is either the last i where the right term is larger than the left
                      one, or the first i for which the left term is larger than the right one. We prove that
                      T violates 12.1 for each case separately.
                                 j
                                        p           p                                                 p
                      Case 1 ( k=1
                                 ij
                                    k
                                      ≥ sij ): For ij + 1 the max term is received by si +1
                                                                                         j
                                                                                            , Since Tj
                                     l=1 sl
                                                        j                                             j
                                                                       p
                      is the min-max, we get Tj ≤ si +1
                                                     j
                                                        . Since T < Tj , we have i(j, T ) ≤ ij , and
                                                                       j
                                  j              j
                                      k=1 pk       k=1 pk
                      T < Tj =       ij        ≤ i(j,T ) . Hence T violates 12.1, as claimed.
                                       l=1 sl      l=1 sl


                                j                                j
                                        p           p                      p
                      Case 2 ( k=1
                                 ij
                                    k
                                      < sij ): Tj ≤ k=1
                                                     ij −1
                                                           k
                                                             since Tj is the min–max, and the max for
                                   l=1 sl                                  sl
                                                    j
                                                                    l=1
                                                                                                  p
                      ij − 1 is received at the right. In addition, i(j, T ) < ij since Tj = sij and T < Tj .
                                                                                                  j
                                               j             j
                                                k=1 pk         k=1 pk
                      Thus T < Tj ≤            ij −1       ≤ i(j,T ) , as we need.
                                                l=1 sl         l=1 sl



                    With this, we get a good lower bound estimate of the optimal makespan:
                                                                       TLB = maxj Tj                        (12.3)
                    The optimal makespan is at least Tj for any j , hence it is at least TLB .

                    A fractional algorithm. We start with a fractional schedule. If machine i gets an α
                    fraction of job j then the resulting load is assumed to be (α · pj )/si . This is of course
                    not a valid schedule, and we later round it to an integral one.

                      Definition
                      j          12.4 (The fractional allocation) Let j be the first job such that
                         k=1 pk > TLB · s1 . Assign to machine 1 jobs 1, . . . , j − 1, plus a fraction of
                      j in order to equate l1 = TLB · s1 . Continue recursively with the unassigned frac-
                      tions of jobs and with machines 2, . . . , m.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0     August 3, 2007    17:17




                                      single-dimensional domains: job scheduling                                    307

                      Lemma 12.5 There is enough space to fractionally assign all jobs, and if job
                      j is fractionally assigned to machine i then pj /si ≤ TLB .

                                                                                                        j
                                                                                                              p
                      proof Let ij be the index that determines Tj . Since TLB ≥ Tj ≥ k=1
                                                                                        ij
                                                                                           k
                                                                                             , we
                                                                                                           l=1 sl
                      can fractionally assign jobs 1, .., j up to machine ij . Since Tj ≥ pj /sij we get
                      the second part of the claim, and setting j = n gives the first part.

                      Lemma 12.6         The fractional load function is monotone.

                      proof We show that if si increases to si = α · si (for α > 1) then li ≤ li . Let
                                                                                                               
                      TLB  denote the new estimate of the optimal makespan. We first claim that TLB                 ≤
                                                                         
                      α · TLB . For an instance s1 , . . . , sm such that sl = α · sl for all machines l we have
                             
                      that TLB  = α · TLB since both terms in the max expression of Tj were multiplied
                      by α. Since sl ≤ sl for all l we have that TLB             
                                                                                ≤ TLB . Now, if li = TLB · si , i.e. i
                                                 
                      was full, then li ≤ TLB · si ≤ TLB · si = li . Otherwise li < TLB · si , hence i is the
                                                              
                      last nonempty machine. Since TLB            ≥ TLB , all previous machines now get at least
                      the same load as before, hence machine i cannot get more load.

                       We now round to an integral schedule. The natural rounding, of integrally placing
                    each job on one of the machines that got some fraction of it, provides a 2-approximation,
                    but violates the required monotonicity (see the exercises). We offer two types of
                    rounding, a randomized rounding and a deterministic one. The former is simpler,
                    and results in a better approximation ratio, but uses the weaker solution concept of
                    truthfulness in expectation. The latter is slightly more involved, and uses deterministic
                    truthfulness, but results in an inferior approximation ratio.

                      Definition 12.7 (A randomized rounding) Choose α ∈ [0, 1] uniformly at
                      random. For every job j that was fractionally assigned to i and i + 1, if j ’s
                      fraction on i is at least α, assign j to i in full, otherwise assign j to i + 1.

                      Theorem 12.8 The randomized scheduling algorithm is truthful in expectation,
                      and obtains a 2-approx. to the optimal makespan in polynomial-time.

                      proof Let us check the approximation first. A machine i may get, in addition
                      to its full jobs, two more jobs. One, j , is shared with machine i − 1, and the
                      other, k, is shared with machine i + 1. If j was rounded to i then i initially has
                      at least 1 − α fraction of j , hence the additional load caused by j is at most
                      α · pj . Similarly, If k was rounded to i then i initially has at least α fraction of k,
                      hence the additional load caused by k is at most (1 − α) · pk . Thus the maximal
                      total additional load that i gets is α · pj + (1 − α) · pk . By Lemma 12.5 we have
                      that max{pj , pk } ≤ TLB and since TLB is not larger than the optimal maximal
                      makespan, the approximation claim follows.
                         For truthfulness, we only need that the expected load is monotone. Note that
                      machine i − 1 gets job j with probability α, so i gets it with probability 1 − α,
P1: SBT
9780521872829main      CUNY1061-Nisan         0 521 87282 0     August 3, 2007     17:17




                    308          computationally efficient approximation mechanisms

                      and i gets k with probability α. So the expected load of machine i is exactly its
                      fractional load. The claim now follows from Lemma 12.6.


                    An integral deterministic algorithm. To be accurate, what follows is not exactly
                    a rounding of the fractional assignment we obtained above, but a similar-in-spirit
                    deterministic assignment. We set virtual speeds, where the fastest machine is set to
                    be slightly faster, and the others are set to be slightly slower, we find a fractional
                    assignment according to these virtual speeds, and then use the “natural” rounding of
                    placing each job fully on the first machine it is fractionally assigned to. With these
                    virtual speeds, the rounding that previously failed to be monotone, now succeeds:


                      Definition 12.9 (A deterministic algorithm)                 Given the bids s1 , . . . , sm , per-
                      form:
                          (i) Set new (virtual) speeds d1 , . . . , dm , as follows. Let d1 = 85 s1 , and for i ≥ 2, let
                                                                                         i (for i = 1, 2, . . .) such that
                                                                                      s1
                              di be the the closest value of the “breakpoints” 2.5
                              di ≤ si .
                       (ii) Compute TLB according to the virtual speeds, i.e. TLB = TLB (di , d−i ).
                      (iii) Assign jobs to machines, starting from the largest job and the fastest machine.
                            Move to the next machine when the current machine, i, holds jobs with total
                            processing time larger or equal to TLB · di .


                    Note that if the fastest machine changes its speed, then all the di ’s may change. Also
                    note that step 3 manages to assign all jobs, since what we are doing is exactly the
                    deterministic natural rounding described above for the fractional assignment, using the
                    di ’s instead of the si ’s. As we shall see, this crucial difference enables monotonicity,
                    in the cost of a certain loss in the approximation.
                        To exactly see the approximation loss, first note that TLB (d) ≤ 2.5TLB (s), since
                    speeds are made slower by at most this factor. For the fastest machine, since s1 is
                    lower than d1 , the actual load up to TLB (d) may be 1.6TLB (d) ≤ 4TLB (s). As we may
                    integrally place on machine 1 one job that is partially assigned also to machine 2,
                    observe (i) that d1 ≥ 4d2 , and (ii) by the fractional rules the added job has load at most
                    TLB (d)d2 . Thus get that the load on machine 1 is at most 54 1.6TLB (d) ≤ 5TLB (s). For
                    any other machine, di ≤ si , and so after we integrally place the one extra partial job
                    the load can be at most 2TLB (d)di ≤ 2 · 2.5TLB (s)si = 5TLB (s)si . Since TLB (s) lower
                    bounds the optimal makespan for s the approximation follows.
                        To understand why monotonicity holds, we first need few observations that easily
                    follow from our knowledge on the fractional assignment.

                       For any i > 1 and β < di , TLB (β, d−i ) ≤ 54 TLB (di , d−i ). Consider the following mod-
                       ification to the fractional assignment for (di , d−i ): machine i does not get any job, and
                       each machine 1 ≤ i  < i gets the jobs that were previously assigned to machine i  + 1.
                       Since i  is faster than i  + 1, any machine 2 ≤ i  < i does not cross the TLB (di , d−i )
                       limit. As for machine 1, note that it is always the case that d1 ≥ 4d2 , hence the new load
                       on machine 1 is at most 54 TLB (di , d−i ).
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0     August 3, 2007    17:17




                                      single-dimensional domains: job scheduling                                    309

                       If a machine i > 1 slows down then the total work assigned to the faster machines does
                       not decrease, which follows immediately from the fact that TLB (di , d−i ) ≥ TLB (di , d−i ),
                       for di ≥ di .

                       If the fastest machine slows down, yet remains the fastest, then its assigned work does
                       not increase. Let s1 = c · s1 for some c < 1. Therefore all breakpoints shift by a factor
                       of c. If no speed si moves to a new breakpoint then all d’s move by a factor of c, the
                       resulting TLB will therefore also move by a factor of c, meaning that machine 1 will
                       get the same set of jobs as before. If additionally some si ’s move to a new breakpoint
                       this implies that the respective di ’s decrease, and by the monotonicity of TLB it also
                       decreases, which means that machine 1 will not get more work.

                      Lemma 12.10         The deterministic algorithm is monotone.

                      proof Suppose that machine i slows down from si to si < si . We need to show
                      that it does not get more work. Assume that the vector d has indeed changed
                      because of i’s change.
                          If i is the fastest machine and it remains the fastest then the above observation
                      is what we need. If the fastest machine changes to i  , then we add an artificial
                      breakpoint to the slowdown decrease, where i and i  ’s speeds are identical, and the
                      title of the “fastest machine” moves from i to i  . Note that the same threshold, T , is
                      computed when the title goes from i to i  . i’s work when it is the “fastest machine”
                      is at least 85 si · T , while i’s work when i  is the fastest is at most 2 2.5
                                                                                                    s1
                                                                                                       T < 85 si · T ,
                      hence decreases.
                          If i is not the fastest, but still full, then di < di (since the breakpoints remain
                      fixed), and therefore TLB (di , d−i ) ≤ 54 TLB (di , d−i ). With si , i  s work is at least
                      T · di (where T = TLB (di , d−i )), and with si its work is at most 2 · 54 T 2.5 di
                                                                                                           = T · di ,
                      hence i’s load does not increase.
                          Finally, note that if i’s is not full then by the third observation, since the work
                      of the previous machines does not decrease, then i’s work does not increase.

                      By the above arguments we immediately get the following theorem.

                      Theorem 12.11 There exists a truthful deterministic mechanism for scheduling
                      related machines, that approximates the makespan by a factor of 5.

                       A note about price computation is in place. A polynomial-time mechanism must
                    compute the prices in polynomial time. To compute the prices for both the randomized
                    and the deterministic mechanisms, we need to integrate over the load function of a
                    player, fixing the others’ speeds. In both cases this is a step function, with polynomial
                    number of steps (when a player declares a large enough speed she will get all jobs, and
                    as she decreases her speed more and more jobs will be assigned elsewhere, where the set
                    of assigned jobs will decrease monotonically). Thus we can see that price computation
                    is polynomial-time.
                       Without the monotonicity requirement, a PTAS for related machines exists. The
                    question whether one can incorporate truthfulness is still open.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   August 3, 2007   17:17




                    310        computationally efficient approximation mechanisms

                    Open Question       Does there exist a truthful PTAS for related machines?
                       The technical discussion of this section aims to demonstrate that, for single-
                    dimensional domains, the algorithmic implications of the game-theoretic requirement
                    are “manageable,” and leave ample flexibility for the algorithmic designer. Multi-
                    dimensionality, on the other hand, does not exhibit this easy structure, and the rest of
                    this chapter is concerned with exactly this issue.


                            12.3 Multidimensional Domains: Combinatorial Auctions

                    As opposed to single-dimensional domains, the monotonicity conditions that charac-
                    terize implementability in multidimensional domains are far more complex (see the
                    discussion in Chapter 9), hence designing implementable approximation algorithms is
                    harder. As discussed in the Introduction, this chapter examines three aspects of this
                    issue, and in this section we will utilize randomness to overcome the difficulties of
                    implementability in multidimensional domains. We study this for the representative
                    and central problem domain of Combinatorial Auctions.
                        Combinatorial Auctions (CAs) are a central model with theoretical importance
                    and practical relevance. It generalizes many theoretical algorithmic settings, like job
                    scheduling and network routing, and is evident in many real-life situations. Chapter 11
                    is exclusively devoted to CAs, providing a comprehensive discussion on the model and
                    its various computational aspects. Our focus here is different: how to design CAs that
                    are, simultaneously, computationally efficient and incentive-compatible. While each
                    aspect is important on its own, obviously only the integration of the two provides an
                    acceptable solution.
                        Let us shortly restate the essentials. In a CA, we allocate m items () to n play-
                    ers. Players value subsets of items, and vi (S) denotes i’s value of a bundle S ⊆ .
                    Valuations additionally satisfy (i) monotonicity, i.e., vi (S) ≤ vi (T ) for S ⊆ T , and (ii)
                    normalization, i.e., vi (∅) = 0. In this section we consider the   goal of maximizing the
                    social welfare: find an allocation (S1 , . . . , Sn ) that maximizes i vi (Si ).
                        Since a general valuation has size exponential in n and m, the representation issue
                    must be taken into account. Chapter 11 examines two models. In the bidding languages
                    model, the bid of a player represents his valuation in a concise way. For this model it is
                    NP-hard to approximate the social welfare within a ratio of (m1/2− ), for any  > 0 (if
                    single-minded bids are allowed). In the query access model, the mechanism iteratively
                    queries the players in the course of computation. For this model, any algorithm with
                    polynomial communication cannot obtain an approximation ratio of (m1/2− ) for
                                                                                            √
                    any  > 0. These bounds are tight, as there exists a deterministic m-approximation
                    with polynomial computation and communication. Thus, for the general case, the
                    computational status by itself is well-understood.
                        The basic incentives issue is again well-understood: with VCG (which requires the
                    exact optimum) we can obtain truthfulness. The two considerations therefore clash if
                    we attempt to use classic techniques, and our aim is to develop a new technique that will
                    combine the two desirable aspects of efficient computation and incentive compatibility.
                        We describe a rather general LP-based technique to convert approximation algo-
                    rithms to truthful mechanisms, by using randomization: given any algorithm to the
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0       August 3, 2007            17:17




                                multidimensional domains: combinatorial auctions                           311

                    general CA problem that outputs a c-approximation to the optimal fractional social
                    welfare, one can construct a randomized c-approximation mechanism that is truthful in
                    expectation. Thus, the same approximation guarantee is maintained. The construction
                    and proof are described in three steps. We first discuss the fractional domain, where
                    we allocate fractions of items. We then show how to move back to the original do-
                    main while maintaining truthfulness, by using randomization. This uses an interesting
                    decomposition technique, which we then describe.

                    The fractional domain. Let xi,S denote the fraction of subset S that player i receives
                    in allocation x. Assume that her value for that fraction is xi,S · vi (S). The welfare
                    maximization becomes an LP:

                                                          
                                              max                  xi,S ·vi (S)                        (CA-P)
                                                          i,S =∅
                                                            
                                        subject to                 xi,S ≤ 1        for each player i    (12.4)
                                                            S =∅
                                                     
                                                                   xi,S ≤ 1        for each item j      (12.5)
                                                     i    S:j ∈S

                                                                   xi,S ≥ 0 ∀i, S = ∅.

                    By constraint 12.4, a player receives at most one integral subset, and constraint 12.5
                    ensures that each item is not overallocated. The empty set is excluded for technical
                    reasons that will become clear below. This LP is solvable in time polynomial in its size
                    by using, e.g., the ellipsoid method. Its size is related to our representation assumption.
                    If we assume the bidding languages model, where the LP has size polynomial in the
                    size of the bid (e.g., k-minded players), then we have a polynomial-time algorithm. If
                    we assume general valuations and a query-access, this LP is solvable with a polynomial
                    number of demand queries (see Chapter 11). Note that, in either case, the number of
                    nonzero xi,S coordinates is polynomial, since we obtain x in polynomial-time (this will
                    become important below). In addition, since we obtain the optimal allocation, we can
                    use VCG (see Chapter 9) to get:

                      Proposition 12.12 In the fractional case, there exists a truthful optimal mech-
                      anism with efficient computation and communication, for both the bidding lan-
                      guages model and the query-access model.

                    The transition to the integral case. The following technical lemma allows for an
                    elegant transition, by using randomization.

                      Definition 12.13 Algorithm A “verifies a c-integrality-gap” (for the linear pro-
                      gram CA-P) if it receives as input real numbers wi,S , and outputs an integral point
                      x̃ which is feasible for CA-P, and
                                                                     
                                           c·    wi,S · x̃i,S ≥ max        wi,S · xi,S
                                                                        feasible x s
                                               i,S                                     i,S
P1: SBT
9780521872829main      CUNY1061-Nisan         0 521 87282 0      August 3, 2007      17:17




                    312          computationally efficient approximation mechanisms

                       Lemma 12.14 (The decomposition lemma) Suppose that A verifies a c-
                       integrality-gap for CA-P (in polynomial time), and x is any feasible point of
                       CA-P. Then one can decompose x/c to a convex combination of integral feasible
                       points. Furthermore, this can be done in polynomial-time.
                    Let {x l }l∈I be                       The proof will find {λl }l∈I such that (i) ∀l ∈
                                     all integral allocations.
                    I, λl ≥ 0, (ii) l∈I λl = 1, and (iii) l∈I λl · x l = x/c. We will also need to provide
                    the integrality gap verifier. But first we show how to use all this to move back to the
                    integral case, while maintaining truthfulness.
                       Definition 12.15 (The decomposition-based mechanism)
                         (i) Compute an optimal fractional solution, x ∗ , and VCG prices piF (v).
                                                            
                        (ii) Obtain a decomposition x ∗ /c = l∈I λl · x l .
                       (iii) With probability λl : (i) choose allocation x l , (ii) set prices piR (v) =
                             [vi (x l )/vi (x ∗ )]piF (v).

                    The strategic properties of this mechanism hold whenever the expected price equals
                    the fractional price over c. The specific prices chosen satisfy, in addition to that, strong
                    individual rationality (i.e., truth-telling ensures a nonnegative utility, regardless of
                    the randomized choice)1 : VCG is individually rational, hence piF (v) ≤ vi (x ∗ ). Thus
                    piR (v) ≤ vi (x l ) for any l ∈ I.

                       Lemma 12.16 The decomposition-based mechanism is truthful in expectation,
                       and obtains a c-approximation to the social welfare.
                                                                                               
                       proof The expected social welfare of the mechanism is (1/c) i vi (x ∗ ), and
                       since x ∗ is the optimal fractional allocation, the approximation guarantee follows.
                       For truthfulness, we first need that the expected price of a player equals her
                       fractional price over c, i.e., Eλl [piR (v)] = piF (v)/c:
                                                  
                                 E{λl }l∈I piR (v) =      λl · [vi (x l )/vi (x ∗ )] · piF (v)
                                                         l∈I
                                                                         
                                                      = piF (v)/vi (x ∗ ) ·  λl · vi (x l )
                                                                               l∈I
                                                                        
                                                      = piF (v)/vi (x ∗ ) · vi (x ∗ /c) = piF (v)/c          (12.6)
                       Fix any v−i ∈ V−i . Suppose that when i declares vi , the fractional optimum is
                       x ∗ , and when she declares vi , the fractional optimum is z∗ . The VCG fractional
                       prices are truthful, hence
                                              vi (x ∗ ) − piF (vi , v−i ) ≥ vi (z∗ ) − piF (vi , v−i )    (12.7)
                       By 12.6 and by the decomposition, dividing 12.7 by c yields
                                                                                                          
                                 λl · vi (x ∗ ) − Eλl piR (vi , v−i ) ≥   λl · vi (z∗ ) − Eλl piR (vi , v−i )
                                             l                                       l


                           l∈I                                                 l∈I


                    1 See Chapter 9 for definitions and a discussion on randomized mechanisms.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    August 3, 2007   17:17




                                 multidimensional domains: combinatorial auctions                              313

                      The left-hand side is the expected utility for declaring vi and the right-hand side
                      is the expected utility for declaring vi , and the lemma follows.

                        The above analysis is for one-shot mechanisms, where a player declares his valuation
                    up-front (the bidding languages model). For the query-access model, where players
                    are being queried iteratively, the above analysis leads to the weaker solution concept
                    of ex-post Nash: if all other players are truthful, player i will maximize his expected
                    utility by being truthful.
                        For example, consider the following single item auction for two players: player I
                    bids first, player II observes I ’s bid and then bids. The highest bidder wins and pays
                    the second highest value. Here, truthfulness fails to be a dominant strategy. Suppose II
                    chooses the strategy “if I bids above 5, I bid 20, otherwise I bid 2.” If I ’s true value is 6,
                    his best response is to declare 5. However, truthfulness is an ex-post Nash equilibrium:
                    if II fixes any value and bids that, then, regardless of II’s bid, I ’s best response is the
                    truth.
                        In our case, if all others answer queries truthfully, the analysis carry through as
                    is, and so truth-telling maximizes i’s the expected utility. The decomposition-based
                    mechanism thus has truthfulness-in-expectation as an ex-post Nash equilibrium for the
                    query-access model. Putting it differently, even if a player was told beforehand the
                    types of the other players, he would have no incentive to deviate from truth-telling.
                                                                                          
                    The decomposition technique. We now decompose x/c = l∈I λl · x l , for any x
                    feasible to CA-P. We first write the LP P and its dual D. Let E = {(i, S)|xi,S > 0}.
                    Recall that E is of polynomial size.
                                                                               1 
                             min         λl                 (P)          max              xi,S wi,S + z        (D)
                                                                                c (i,S)∈E
                       s.t.         l∈I                                s.t.
                                                                         
                            λl xi,S =
                                l       xi,S
                                             ∀(i, S) ∈ E (12.8)
                                                                                l
                                                                              xi,S wi,S + z ≤ 1 ∀l ∈ I (12.9)
                         l
                                         c                              (i,S)∈E
                       
                            λl ≥ 1                                     z≥0
                       l    λl ≥ 0       ∀l ∈ I                          wi,S unconstrained     ∀(i, S) ∈ E.
                    Constraints
                                12.8 of P describe the decomposition; hence, if the optimum satisfies
                       l∈I λl = 1, we are almost done. P has exponentially many variables, so we need to
                    show how to solve it in polynomial time. The dual D will help. It has variables wi,S
                    for each constraint 12.8 of P, so it has polynomially many variables but exponentially
                    many constraints. We use the ellipsoid method to solve it, and construct a separation
                    oracle using our verifier A.
                                                                     
                       Claim 12.17 If w, z is feasible for D then 1c (i,S)∈E xi,S wi,S + z ≤ 1. Further-
                       more, if this inequality is reversed, one can use A to find a violated constraint
                       of D in polynomial-time.
                                              
                      proof Suppose 1c · (i,S)∈E xi,S wi,S + z > 1. Let A receive w as input and sup-
                                                                                                l
                      pose that the integral allocation that A outputs is x l . We have (i,S)∈E xi,S wi,S ≥
                        
                          (i,S)∈E xi,S wi,S > 1 − z, where the first inequality follows since A is a
                      1
                      c
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    August 3, 2007   17:17




                    314        computationally efficient approximation mechanisms

                      c-approximation to the fractional optimum, and the second inequality is the vio-
                      lated inequality of the claim. Thus constraint 12.9 is violated (for x l ).
                                                                                                       
                      Corollary 12.18 The optimum of D is 1, and the decomposition x/c =                   l∈I λl ·
                      x l is polynomial-time computable.

                      proof z = 1, wi,S = 0 ∀(i, S) ∈ E is feasible; hence, the optimum is at least
                      1. By claim 12.17 it is at most 1. To solve P, we first solve D with the following
                      separation oracle: given w, z, if c (i,S)∈E xi,S wi,S + z ≤ 1, return the separating
                                                        1
                                      
                      hyperplane 1c (i,S)∈E xi,S wi,S + z = 1. Otherwise, find the violated constraint,
                      which implies the separating hyperplane. The ellipsoid method uses polynomial
                      number of constraints; thus, there is an equivalent program with only those con-
                      straints. Its dual is a program that is equivalent to P but with polynomial number
                      of variables. We solve that to get the decomposition.

                    Verifying the integrality gap. We now construct the integrality gap verifier for CA-P.
                    Recall that it receives as input weights wi,S , and outputs an integral allocation x l which
                    is a c-approximation to the social welfare w.r.t. wi,S . Two requirements differentiate
                    it from a “regular” c-approximation for CAs: (i) it cannot assume any structure on
                    the weights wi,S (unlike CA, where we have non-negativity and monotonicity), and
                    (ii) the obtained welfare must be compared to the fractional optimum (usually we care
                    for the integral optimum). The first property is not a problem.

                      Claim 12.19 Given a c-approximation for general CAs, A , where the approx-
                      imation is with respect to the fractional optimum, one can obtain an algorithm A
                      that verifies a c-integrality-gap for the linear program CA-P, with a polynomial
                      time overhead on top of A.

                                                                                    +
                      proof Given w = {wi,S }(i,S)∈E , define w+ by wi,S                = max(wi,S , 0), and w̃
                                                         +
                      by w̃i,S = maxT ⊆S , (i,T )∈E wi,T (where the maximum is 0 if no T ⊆ S has
                      (i, T ) ∈ E. w̃ is a valid valuation, and    can be succinctly represented        with size
                                    ∗                                                              
                      |E|. Let O = maxx is feasible for CA-P (i,S)∈E xi,S wi,S . Feed w̃ to A to get x̃ such
                                               ∗
                      that i,S x̃i,S w̃i,S ≥ Oc (since    w̃i,S ≥ wi,S for every
                                                                                (i, S)).
                           Note that it is possible that (i,S)∈E x̃i,S wi,S < i,S x̃i,S w̃i,S , since (i) the left
                      hand sum only considers coordinates in E and (ii) some wi,S coordinates might
                      be negative. To fix the first problem define x + as follows: for any (i, S) such that
                                       +                                          +
                      x̃i,S = 1, set xi,T  = 1 for T = arg maxT ⊆S:(i,T )∈E wi,T (set all other coordinates
                                                                           
                             +                                                         +   +
                      of x to 0). By construction, i,S x̃i,S w̃i,S = (i,S)∈E xi,S        wi,S . To fix the second
                                                                        +
                      problem,
                                 define x asfollows: set xi,S = xi,S if wi,S ≥ 0 and 0 otherwise. Clearly,
                                            l                   l
                                                          +     +
                          (i,S)∈E i,S wi,S =
                                   l                                      l
                                 x              (i,S)∈E xi,S wi,S , and x is feasible for CA-P.


                       The requirement to approximate the fractional optimum does affect generality.
                    However, one can use the many algorithms that use the primal-dual method, or a
                    derandomization of an LP randomized rounding. Simple combinatorial algorithms
                    may also satisfy this property. In fact, the greedy algorithm from Chapter 11 for
P1: SBT
9780521872829main      CUNY1061-Nisan          0 521 87282 0         August 3, 2007        17:17




                                    multidimensional domains: combinatorial auctions                                  315

                    single-minded
                    √ √           players satisfies the requirement, and a natural variant verifies a
                      2 · m integrality-gap for CA-P.

                      Definition 12.20 (Greedy (revisited)) Fix {wi,S√            }(i,S)∈E as the input. Construct
                      x as follows. Let (i, S) = arg max(i  ,S  )∈E (wi  ,S  / |S  |). Set xi,S = 1. Remove
                      from E all (i  , S  ) with i  = i or S  ∩ S = ∅. If E = ∅, reiterate.
                                                         √
                      Lemma 12.21           Greedy is a ( 2m)-approximation to the fractional optimum.

                      proof Let y = {yi,S }(i,S)∈E be the optimal fractional allocation. For every
                      player i with xi,Si = 1 (for some Si ), let Yi = { (i  , S) ∈ E | yi  ,S >0 and (i  , S)
                       was removed from E when (i, Si ) was added }. We show that                   (i  ,S)∈Yi yi  ,S
                                 √ √
                      wi  ,S ≤ ( 2 m)wi,Si , which proves the claim. We first have
                                                                                  wi  ,S
                                             yi  ,S wi  ,S =              yi  ,S √          |S|
                                 (i  ,S)∈Yi                   (i  ,S)∈Yi
                                                                                      |S|
                                                                wi,S 
                                                             ≤ √ i                    yi  ,S · |S|
                                                                     |Si | (i  ,S)∈Y
                                                                              i

                                                                        ⎛                   ⎞⎛               ⎞
                                                              wi,S                                
                                                            ≤ √ i ⎝          yi  ,S ⎠ ⎝         yi  ,S · |S|⎠   (12.10)
                                                               |Si |   
                                                                     (i ,S)∈Y              
                                                                                         (i ,S)∈Y
                                                                                    i                  i


                      The first inequality follows since (i, Si ) was chosen by greedy when (i  , S) was
                      in E, and the second inequality is a simple algebraic fact. We also have:
                                                                     
                            yi  ,S ≤              yi  ,S +      yi,S ≤    1 + 1 ≤ |Si | + 1     (12.11)
                      (i  ,S)∈Yi      j ∈Si (i  ,S)∈Yi ,j ∈S         (i,S)∈Yi          j ∈Si

                      where the first inequality holds since every (i  , S) ∈ Yi has either S ∩ Si = ∅ or
                      i  = i, and the second inequality follows from the feasibility constraints of CA-P,
                      and,
                                                                   
                                                 yi  ,S · |S| ≤              yi  ,S ≤ m           (12.12)
                                              (i  ,S)∈Yi                 j ∈ (i  ,S)∈Yi ,j ∈S

                      Combining 12.10, 12.11, and 12.12, we get what we need:
                                                        wi,S             √   √ √
                                       yi  ,S wi  ,S ≤ √ i · |Si | + 1 · m ≤ 2 · m · wi,Si
                            (i  ,S)∈Y
                                                          |Si |
                                       i


                       Greedy is not truthful, but with the decomposition-based mechanism, we use
                    randomness in order to “plug-in” truthfulness. We get the following theorem.

                      Theorem 12.22 The decomposition-based mechanism with Greedy as the
                                                     √ √ rational and truthful-in-expectation, and
                      integrality-gap verifier is individually
                      obtains an approximation of 2 · m to the social welfare.

                    Remarks. The decomposition-based technique is quite general, and can be used in
                    other cases, if an integrality-gap verifier exists for the LP formulation of the problem.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   August 3, 2007   17:17




                    316        computationally efficient approximation mechanisms

                    Perhaps the most notable case is multiunit CAs, where there exist B copies of each
                    item, and any player desires at most one copy from each item. In this case, one can
                                    1
                    verify a O(m B+1 ) integrality gap, and this is the best possible in polynomial time. To
                    date, the decomposition-based mechanism is the only truthful mechanism with this
                    tight guarantee.
                        Nevertheless, this method is not completely general, as VCG is. One drawback is for
                    special cases of CAs, where low approximation ratios exist, but the integrality gap of
                    the LP remains the same. For example, with sub-modular valuations, the integrality gap
                    of CA-P is the same (the constraints do not change), but lower-than-2 approximations
                    exist. To date, no truthful mechanism with constant approximation guarantees is
                    known for this case. One could, in principle, construct a different LP formulation for
                    this case, with a smaller integrality gap, but these attempts were unsuccessful so far.
                        While truthfulness-in-expectation is a natural modification of (deterministic)
                    truthfulness, and although this notion indeed continues to be a worst-case notion, still
                    it is inferior to truthfulness. Players are assumed to only care about their expected
                    utility, and not about the variance, for example. A stronger notion is that of “universal
                    truthfulness,” were players maximize their utility for every coin toss. But even this is
                    still weaker. While in classic algorithmic settings one can use the law of large numbers
                    to approach the expected performance, in mechanism design one cannot repeat
                    the execution and choose the best outcome as this affects the strategic properties.
                    Deterministic mechanisms are still a better choice.


                           12.3.1 A General Overview of Truthful Combinatorial Auctions
                    The search for truthful CAs is an active field of research. Roughly speaking, two
                    techniques have proved useful for constructing truthful CAs. In “Maximal-in-Range”
                    mechanisms, the range of possible allocations is restricted, and the optimal-in-this-
                                                                                                          √
                    range allocation is chosen. This achieves deterministic truthfulness with an O( m)-
                                                                                                           m
                    approximation for subadditive valuations (Dobzinski et al., 2005), an O( √log            m
                                                                                                               )-
                    approximation for general valuations (Holzman et al., 2004), and a 2-approximation.
                    when all items are identical (“multi-unit auctions”) (Dobzinski and Nisan, 2006). A
                    second technique is to partition the set of players, sample statistics from one set, and use
                    it to obtain a good approximation for the other. See Chapter 13 for details. This tech-
                                          √
                    nique obtains an O( m)-approximation. for general valuations, and an O(log2 m) for
                    XOS valuations (Dobzinski et al., 2006). The truthfulness here is “universal,” i.e., for
                    any coin toss – a stronger notion than truthfulness in expectation. Bartal et al. (2003)
                                                                                         1
                    use a similar idea to obtain a truthful and deterministic O(B · m B−2 )-approximation for
                    multiunit CAs with B ≥ 3 copies of each item. For special cases of CAs, these tech-
                    niques do not yet manage to obtain constant-factor truthful approximations (Dobzinski
                    and Nisan, 2006 prove this impossibility for Maximal-In-Range mechanisms). Due to
                    the importance of constant-factor approximations, explaining this gap is challenging:

                    Open Question Does there exist truthful constant-factor approximations for special
                    cases of CAs that are NP-hard and yet constant algorithmic approximations are known?
                    For example, does there exist a truthful constant-factor approximation for CAs with
                    submodular valuations?
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   August 3, 2007   17:17




                              impossibilities of dominant strategy implementability                         317

                    For general valuations, the above shows a significant gap in the power of randomized vs.
                    deterministic techniques. It is not known if this gap is essential. A possible argument for
                    this gap is that, for general valuations, every deterministic mechanism is VCG-based,
                    and these have no power. Lavi et al. (2003) have initiated an investigation for the first
                    part of the argument, obtaining only partial results. Dobzinski and Nisan (2006) have
                    studied the other part of the argument, again with only partial results.

                    Open Question What are the limitations of deterministic truthful CAs? Does ap-
                    proximation and dominant-strategies clash in some fundamental and well-defined way
                    for CAs?

                       This section was devoted to welfare maximization. Revenue maximization is another
                    important goal for CA design. The mechanism of Bartal et al. (2003) obtains the same
                    guarantees with respect to the optimal revenue. More tight results for multi-unit auctions
                    with budget constrained players are given by Borgs et al. (2005), and for unlimited-
                    supply CAs by Balcan et al. (2005). It should be noted that these are preliminary
                    results for special cases; this issue is still quite unexplored.


                          12.4 Impossibilities of Dominant Strategy Implementability

                    In the previous sections we saw an interesting contrast between deterministic and
                    randomized truthfulness, where the key difference seems to be the dimensionality of
                    the domain. We now ask whether the source of this difficulty can be rigorously identified
                    and characterized. What exactly do we mean by an “impossibility,” especially since we
                    know that VCG mechanisms are possible, in every domain? Well, we mean that nothing
                    besides VCG is possible. Such a situation should be viewed as an impossibility, since
                    (i) many times VCG is computationally intractable (as we saw for CAs), and (ii) many
                    times we seek goals different from welfare maximization (as we saw for scheduling
                    domains). The monotonicity characterizations of Chapter 9 almost readily provide few
                    easy impossibilities for some special domains (see the exercises at the end of this
                    chapter), and in this section we will study a more fundamental case.
                        To formalize our exact question, it will be convenient to use the abstract social choice
                    setting introduced in Chapter 9: there is a finite set A of alternatives, and each player
                    has a type (valuation function) v : A →  that assigns a real number to every possible
                    alternative. vi (a) should be interpreted as i’s value for alternative a. The valuation
                    function vi (·) belongs to the domain Vi of all possible valuation functions. Our goal is
                    to implement in dominant strategies the social choice function f : V1 × · · · × Vn → A
                    (where w.l.o.g. assume that f : V → A is onto A). From chapter 9 we know that VCG
                    implements welfare maximization, for any domain, and that affine maximizers are also
                    always implementable.

                      Definition 12.23 (Affine maximizer) f is an “affine maximizer” if there exist
                      weights k1 , . . . , kn and {Cx }x∈A such that, for all v ∈ V ,

                                            f (v) ∈ argmaxx∈A {i=1
                                                                n
                                                                    ki vi (x) + Cx }.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0     August 3, 2007    17:17




                    318        computationally efficient approximation mechanisms

                      The fundamental question is what other function forms are implementable. This
                    question has remained mostly unexplored, with few exceptions. In particular, if the
                    domain is unrestricted, the answer is sharp.

                      Theorem 12.24 Suppose |A| ≥ 3 and Vi = A for all i. Then f is dominant-
                      strategy implementable iff it is an affine maximizer.

                       We will prove here a slightly easier version of the sufficiency direction. The proof
                    is simplified by adding an extra requirement, but the essential structure is kept. The
                    exercises give guidelines to complete the full proof.

                      Definition 12.25 (Neutrality) f is neutral if for all v ∈ V , if there exists an
                      alternative x such that vi (x) > vi (y), for all i and y = x, then f (v) = x.

                    Neutrality essentially implies that if a function is indeed an affine maximizer then the
                    additive constants Cx are all zero.

                      Theorem 12.26 Suppose |A| ≥ 3 and for every i, Vi = A . If f is dominant-
                      strategy implementable and neutral then it must be an affine maximizer.

                       For the proof, we start with two monotonicity conditions. Recall that Chapter 9
                    portrayed the strong connection between implementability and certain monotonicity
                    properties. The monotonicity conditions that we consider here are stronger, and are not
                    necessary for all domains. However, for an unrestricted domain, their importance will
                    soon become clear.

                      Definition 12.27 (Positive association of differences (PAD)) f satisfies PAD
                      if the following holds for any v, v  ∈ V . Suppose f (v) = x, and for any y = x,
                      and any i, vi (x) − vi (x) > vi (y) − vi (y). Then f (v  ) = x.

                      Claim 12.28       Any implementable function f , on any domain, satisfies PAD.

                      proof Let v i = (v1 , . . . , vi , vi+1 , . . . , vn ), i.e., players up to i declare accord-
                      ing to v  ; the rest declare according to v. Thus v 0 = v, v n = v  , and f (v 0 ) = x.
                      Suppose f (v i−1 ) = x for some 1 ≤ i ≤ n. For every alternative y = x we have
                      vii (y) − vii−1 (y) < vii (x) − vii−1 (x), and in addition v−i      i−1
                                                                                              = v−i
                                                                                                  i
                                                                                                    . Thus, W-MON
                      implies that f (v ) = x. By induction, f (v ) = x.
                                          i                                n



                    In an unrestricted domain, weak monotonicity can be generalized as follows.

                      Definition 12.29 (Generalized-WMON) For every v, v  ∈ V with f (v) = x
                      and f (v  ) = y there exists a player i such that vi (y) − vi (y) ≥ vi (x) − vi (x).

                    With weak monotonicity, we fix a player and fix the declarations of the others. Here,
                    this qualifier is dropped. Another way of looking at this property is the following: If
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   August 3, 2007   17:17




                              impossibilities of dominant strategy implementability                           319

                    f (v) = x and v  (x) − v(x) > v  (y) − v(y) then f (v  ) = y (a word about notation: for
                    α, β ∈ n , we use α > β to denote that ∀i, αi > βi ).

                      Claim 12.30 If the domain is unrestricted and f is implementable then f
                      satisfies Generalized-WMON.

                      proof Fix any v, v  . We show that if f (v  ) = x and v  (y) − v(y) > v  (x) −
                      v(x) for some y ∈ A then f (v) = y. By contradiction, suppose that f (v) = y.
                      Fix  ∈ n such that v  (x) − v  (y) = v(x) − v(y) − , and define v  :
                                               ⎧
                                               ⎪
                                               ⎪ min{vi (z) , vi (z) + vi (x) − vi (x)} − i z = x, y
                                               ⎨
                        ∀i, z ∈ A : vi (z) = vi (x) − i                                     z=x
                                               ⎪
                                               ⎪             2
                                               ⎩
                                                 vi (y)                                        z = y.
                      By PAD, the transition v → v  implies f (v  ) = y, and the transition v  → v 
                      implies f (v  ) = x, a contradiction.

                    We now get to the main construction. For any x, y ∈ A, define:
                                 P (x, y) = {α ∈ n | ∃v ∈ V : v(x) − v(y) = α, f (v) = x }.               (12.13)
                                                                                 
                    Looking at differences helps since we need to show that i ki [vi (x) − vi (y)] ≥ Cy −
                    Cx if f (v) = x. Note that P (x, y) is not empty (by assumption there exists v ∈ V with
                    f (v) = x), and that if α ∈ P (x, y) then for any δ ∈ n++ (i.e., δ > 0), α + δ ∈ P (x, y):
                                                                                     
                    take v with f (v) = x and v(x) − v(y) = α, and construct v by increasing v(x) by δ,
                    and setting the other coordinates as in v. By PAD f (v  ) = x, and v  (x) − v  (y) = α + δ.

                      Claim 12.31 For any α,  ∈ n ,  > 0:  (i) α −  ∈ P (x, y) ⇒ −α ∈
                                                                                         / P (y, x),
                                 / P (x, y) ⇒ −α ∈ P (y, x).
                      and (ii) α ∈

                      proof (i) Suppose by contradiction that −α ∈ P (y, x). Therefore there exists
                      v ∈ V with v(y) − v(x) = −α and f (v) = y. As α −  ∈ P (x, y), there also
                      exists v  ∈ V with v  (x) − v  (y) = α −  and f (v  ) = x. But since v(x) − v(y) =
                      α > v  (x) − v  (y), this contradicts Generalized-WMON. (ii) For any z = x, y
                      take some βz ∈ P (x, z) and fix some  > 0.    Fix some v such that v(x) − v(y) = α
                      and v(x) − v(z) = βz +  for all z = x, y. By the above argument, f (v) ∈ {x, y}.
                      Since v(x) − v(y) = α ∈     / P (x, y) it follows that f (v) = y. Thus −α = v(y) −
                      v(x) ∈ P (y, x), as needed.

                                                                    such that α − 1 ∈ P (x, y) and
                      Claim 12.32 Fix α, β, 1 , 2 , ∈ n , i > 0,
                      β − 2 ∈ P (y, z). Then α + β − (1 + 2 )/2 ∈ P (x, z).

                      proof For any w = x, y, z fix some δw ∈ P (x, w). Choose any v such that
                      v(x) − v(y) = α − 1 /2, v(y) − v(z) = β − 2 /2, and v(x) − v(w) = δw +  for
                                                     By Generalized-WMON, f (v) = x. Thus α +
                      all w = x, y, z (for some  > 0).
                      β − (1 + 2 )/2 = v(x) − v(z) ∈ P (x, z).
P1: SBT
9780521872829main       CUNY1061-Nisan            0 521 87282 0      August 3, 2007       17:17




                    320           computationally efficient approximation mechanisms

                       Claim 12.33 If α is in the interior of P (x, y) then α is in the interior of P (x, z),
                       for any z = x, y.

                       proof Suppose α −  ∈ P (x, y) for some  > 0.        By neutrality we have that
                       /4 − /8 = /8 ∈ P (y, z). By Claim 12.32 we now get that α − /4 ∈ P (x, z),
                       which implies that α is in the interior of P (x, z).

                        By similar arguments, we also have that if α is in the interior of P (x, z) then α
                    is in the interior of P (w, z). Thus we get that for any x, y, w, z ∈ A, not necessarily
                    distinct, the interior of P (x, y) is equal to the interior of P (w, z). Denote the interior
                    of P (x, y) as P .

                       Claim 12.34              P is convex.

                       proof We show that α, β ∈ P implies (α + β)/2 ∈ P . A known fact from
                       convexity theory then implies that P is convex.2 By Claim 12.32, α + β ∈ P . We
                       show that for any α ∈ P we have α/2 ∈ P as well, which then implies the Claim.
                       Suppose by contradiction that α/2 ∈ / P . Thus by Claim 12.31, −α/2 ∈ P . Then
                       α/2 = α + (−α/2) ∈ P , a contradiction.

                        We now conclude the proof of Theorem 12.26. Neutrality implies that 0 is on the
                    boundary of any P (x, y); hence, it is not in P . Let P̄ denote the closure of P . By the
                    separation lemma, there exists a k ∈ n such that for any α ∈ P̄ , k · α ≥ 0. Suppose
                    that f (v) = x for some v ∈ V , and fix any y = x. Thus v(x) − v(y) ∈ P (x, y), and
                    k · v(x) − v(y) ≥ 0. Hence k · v(x) ≥ k · v(y), and the theorem follows.
                        We have just seen a unique example, demonstrating that there exists a domain
                    for which affine maximizers are the only possibility. However, our natural focus is on
                    restricted domains, as most of the computational models that we consider do have some
                    structure (e.g., the two domains we have considered in this chapter). Unfortunately,
                    clear-cut impossibilities for such domains are not known.

                    Open Question Characterize the class of domains for which affine maximizers are
                    the only implementable functions.

                       Even this question does not capture the entire picture, as, for example, it is known that
                    there exists an implementable but not an affine-maximizer CA.3 Nevertheless, there
                    do seem to be some inherent difficulties in designing truthful and computationally-
                    efficient CAs.4 The less formal open question therefore searches for the fundamental
                    issues that cause the clash. Obviously, these are related to the monotonicity conditions,
                    but an exact quantification of this is still unknown.

                    2 For α, β ∈ P and 0 ≤ λ ≤ 1, build a series of points that approach λα + (1 − λ)β, such that any point in the

                      series has a ball of some fixed radius around it that fully belongs to P .
                    3 See Lavi et al. (2003).
                    4 Note that we have in mind deterministic CAs.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   August 3, 2007   17:17




                                            alternative solution concepts                                321

                                          12.5 Alternative Solution Concepts

                    In light of the conclusions of the previous section, a natural way to advance would
                    be to reexamine the solution concept that we are using. In Section 12.3 we saw that
                    randomization certainly helps, but also carries with it some disadvantages. However, in
                    some cases randomization is not known to help, and additionally sometimes we want to
                    stick to deterministic mechanisms. What other solution concepts that fit the worst-case
                    way of thinking in CS can we use?
                       One simple thought is that algorithm designers do not care so much about actually
                    reaching an equilibrium point – our major concern is to guarantee the optimality of the
                    solution, taking into account the strategic behavior of the players. One way of doing
                    this is to reach a good equilibrium point. But there is no reason why we should not
                    allow the mechanism designer to “leave in” several acceptable strategic choices for the
                    players, and to require the approximation to be achieved in each of these choices.
                       As a first attempt, one is tempted to simply let the players try and improve the
                    basic result by allowing them to lie. However, this can cause unexpected dynamics, as
                    each player chooses her lies under some assumptions about the lies of the others, etc.
                    etc. We wish to avoid such an unpredictable situation, and we insist on using rigorous
                    game theoretic reasoning to explain exactly why the outcome will be satisfactory. The
                    following definition captures the initial intuition, without falling to such pitfalls:

                      Definition 12.35 (Algorithmic implementation) A mechanism M is an algo-
                      rithmic implementation of a c-approximation (in undominated strategies) if there
                      exists a set of strategies, D, such that (i) M obtains a c-approximation for any
                      combination of strategies from D, in polynomial time, and (ii) for any strategy
                      not in D, there exists a strategy in D that weakly dominates it, and this transition
                      is polynomial-time computable.

                       The important ingredients of a dominant-strategies implementation are here: the
                    only assumption is that a player is willing to replace any chosen strategy with a
                    strategy that dominates it. Indeed, this guarantees at least the same utility, even in
                    the worst case, and by definition can be done in polynomial time. In addition, again
                    as in dominant-strategy implementability, this notion does not require any form of
                    coordination among the players (unlike Nash equilibrium), or that players have any
                    assumptions on the rationality of the others (as in “iterative deletion of dominated
                    strategies”).
                       However, two differences from dominant-strategies implementation are worth men-
                    tioning: (I) A player might regret his chosen strategy, realizing in retrospect that
                    another strategy from D would have performed better, and (II) deciding how to play
                    is not straight-forward. While a player will not end up playing a strategy that does not
                    belong to D, it is not clear how he will choose one of the strategies of D. This may
                    depend, for example, on the player’s own beliefs about the other players, or on the
                    computational power of the player.
                       Another remark, about the connection to the notion of implementation in undomi-
                    nated strategies, is in place. The definition of D does not imply that all undominated
                    strategies belong to D, but rather that for every undominated strategy, there is an
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   August 3, 2007   17:17




                    322        computationally efficient approximation mechanisms

                    equivalent strategy inside D (i.e., a strategy that yields the same utility, no matter
                    what the others play). The same problem occurs with dominant-strategy implementa-
                    tions, e.g., VCG, where it is not required that truthfulness should be the only dominant
                    strategy, just a dominant strategy.
                       In this section we illustrate how to use such a solution concept to design CAs for
                    a special class of “single-value” players. The resulting auction has another interesting
                    feature: while most mechanisms we have seen so far are direct revelation, in practice
                    indirect mechanisms, and especially ascending auctions (players compete by raising
                    prices and winners pay their last bid) are much preferred. The following result is an
                    attempt to handle this issue as well.
                    Single-value players. The mechanisms of this section fit the special case of players
                    that desire several different bundles, all for the same value: Player i is single-valued
                    if there exists v̄i ≥ 1 such that for any bundle s, vi (s) ∈ {0, v̄i }. That is, i desires any
                    one bundle out of a collection S̄i of bundles, for a value v̄i . We denote such a player
                    by (v̄i , S̄i ). v̄i and S̄i are private information of the player. Since S̄i may be of size
                    exponential in m, we assume the query access model, as detailed below.

                    An iterative wrapper. We start with a wrapper to a given algorithmic subprocedure,
                    which will eventually convert algorithms to a mechanism, with a small approximation
                    loss. It operates in iterations, with iteration index j , and maintains the tentative winners
                                                                                   j
                    Wj , the sure-losers Lj , and a “tentative winning bundle” si for every i. In each iteration,
                    the subprocedure is invoked to update the set of winners to Wj +1 and the winning
                    bundles to s j +1 . Every active nonwinner then chooses to double his bid (vi ) or to
                                                                                                          j

                    permanently retire. This is iterated until all nonwinners retire.

                      Definition 12.36 (The wrapper) Initialize j = 0, Wj = Lj = ∅, and for every
                      player i, vi0 = 1 and si0 = . While Wj ∪ Lj = “all players” perform:
                      1. (Wj +1 , s j +1 ) ← PROC(v j , s j , Wj ).
                                                                                    j +1    j
                      2. ∀i ∈
                            / Wj +1 ∪ Lj , i chooses whether to double his value (vi ← 2 · vi ) or to
                                              j +1                       j +1     j
                       permanently retire (vi ← 0). For all others set vi ← vi .
                                                      j +1
                      3. Update Lj +1 = {i ∈ N | vi = 0} and j → j + 1, and reiterate.
                      Outcome: Let J = j (total number of iterations). Every i ∈ WJ gets siJ and pays
                      viJ . All others lose (get nothing, pay 0).

                                                                              j +1    j +1
                    For feasibility, PROC must maintain: ∀i, i  ∈ Wj +1 , si ∩ si  = ∅.
                       We need to analyze the strategic choices of the players, and the approximation loss
                    (relative to PROC). This will be done gradually. We first worry about minimizing the
                    number of iterations.

                      Definition 12.37 (Proper procedure) PROC is proper if (1) Pareto: ∀i ∈      /
                                    j +1           j +1                                  j +1 j
                      Wj +1 ∪ Lj , si ∩ (∪l∈Wj +1 sl ) = ∅, and (2) Shrinking-sets: ∀i, si ⊆ si .

                    In words, the pareto property implies that the set of winners that PROC outputs is
                    maximal, i.e., that any loser that has not retired desires a bundle that intersects some
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    August 3, 2007     17:17




                                              alternative solution concepts                                  323

                    winner’s bundle. The shrinking-sets property says that a player’s new tentative bundle
                    must be a subset of the old tentative bundle.
                                                                    j
                       A “reasonable” player will not increase vi above v̄i ; otherwise, his utility will be
                    nonpositive (this strategic issue is formally discussed below). Assuming this, there
                    will clearly be at most n · log(vmax ) iterations, where vmax = maxi v̄i . With a proper
                    procedure this bound becomes independent of n.

                                                                                   j
                      Lemma 12.38 If every player i never increases vi above v̄i , then any proper
                      procedure performs at most 2 · log(vmax ) + 1 iterations.

                      proof Consider iteration j = 2 · log(vmax ) + 1, and some i1 ∈  / Wj +1 ∪ Lj that
                      (by contradiction) doubles his value. By Pareto, there exists i2 ∈ Wj +1 such
                            j +1    j +1
                      that si1 ∩ si2 = ∅. By “shrinking-sets,” in every j  < j their winning bundles
                      intersect, hence at least one of them was not a winner, and doubled his value. But
                              j
                      then vi1 ≥ vmax , a contradiction.

                    This affects the approximation guarantee, as shown below, and also implies that the
                    Wrapper adds only a polynomial-time overhead to PROC.

                    A warm-up analysis. To warm up and to collect basic insights, we first consider
                    the case of known single-minded players (KSM), where a player desires one specific
                    bundle, S̄i , which is public information (she can lie only about her value). This allows
                    for a simple analysis: the wrapper converts any given c-approximation. to a dominant-
                    strategy mechanism with O(log(vmax ) · c) approximation. Thus, we get a deterministic
                    technique to convert algorithms to mechanisms, with a small approximation loss.
                                                                j +1   j
                       Here, we initialize si0 = S̄i , and set si = si , which trivially satisfies the shrinking-
                    sets property. In addition, pareto is satisfied w.l.o.g. since if not, add winning players in
                    an arbitrary order until pareto holds. For KSM players, this takes O(n · m) time. Third,
                    we need one more property:
                                                                          j          j
                      Definition 12.39     (Improvement)          i∈Wj +1 vi ≥   i∈Wj vi .


                    This is again without loss of generality: if the winners outputted by PROC violate this,
                    simply output Wj as the new winners. To summarize, we use:

                      Definition 12.40 (The KSM-PROC) Given a c-approximation. A for KSM
                      players, KSM-PROC invokes A with s j (the desired bundles) and v j (player
                      values). Then, it postprocesses the output to verify pareto and improvement.

                                                                                                    j
                      Proposition 12.41       Under dominant strategies, i retires iff v̄i /2 ≤ vi ≤ v̄i .

                    (The simple proof is omitted.) For the approximation, the following analysis carries
                                                                                   j
                    through to the single-value case. Let Si |s j = {s ∈ Si | s ⊆ si }, and
                                                                  i


                                                  = { (vi , Si | j )|i retired at iteration j },
                                        Rj (v , S)                                                      (12.14)
                                                                 s
                                                                 i
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    August 3, 2007   17:17




                    324        computationally efficient approximation mechanisms

                                                                                            contains a single-value
                    i.e., for every player i that retired at iteration j the set Rj (v , S)
                    player, with value vi (given as a parameter), and desired bundles Si |s j (where Si is given
                                                                                                   i
                    as a parameter). For the KSM case, Rj (v̄, S̄) is exactly all retired players in iteration j , as
                    the operator “|s j ” has no effect. Hence, to prove the approximation, we need to bound the
                                     i
                    value of the optimal allocation to the players in R̄ = ∪Jj=1 Rj (v̄, S̄). For an instance X of
                    single-value players, let OPT (X) be the value of the optimal allocation           to the players
                                                          = maxall allocations(s ,...,s ) s.t.s ∈S | { i: s =∅ vi }.
                    in X. In particular: OPT (Rj (v , S))                       1      n       i  i j      i
                                                                                                  i  s




                      Definition 12.42 (Local approximation) A proper procedure is a c-local-
                      approximation w.r.t a strategy set D if it satisfies improvement, and, for any
                      combination of strategies in D and any iteration j ,
                                                                                    j
                        Algorithmic approximation OPT (Rj (v j , S̄)) ≤ c · i∈Wj vi
                                             j         j                                j
                          Value bounds vi ≤ vi (si ), and, if i retires at j then vi ≥ v̄i /2.

                      Claim 12.43 Given a c-approximation A for single minded players, KSM-PROC
                      is a c-local-approximation for the set D of dominant strategies.

                      proof The algorithmic approximation property follows since A out-
                      puts a c-approximation outcome. The value bounds property is exactly
                      Proposition 12.41.

                       We next translate local approximation to global approximation (this is valid also for
                    the single-value case).

                      Claim
                            12.44 A c-local-approximation satisfies OPT (R̄) ≤ 5 · log(vmax ) · c ·
                        i∈WJ v̄i whenever players play strategies in D.


                      proof By the value bounds, OPT (Rj (v̄, S̄)) ≤ 2 · OPT (Rj (v j , S̄)). We have
                                                          j                                      
                      (i) OPT (Rj (v j , S̄)) ≤ c · i∈Wj vi by algorithmic approximation, (ii) i∈Wj
                        j             j +1
                      vi ≤ i∈Wj +1 vi by improvement, and (iii) viJ ≤ v̄i (by the value bounds), and
                                                                                                  J
                      therefore we get OPT (Rj (v̄,  S̄)) ≤ 2 · c · i∈WJ v̄i . Hence OPT (R̄) ≤ j =1
                      OPT (Rj (v̄, S̄)) ≤ J · 2 · c · i∈WJ v̄i . Since J ≤ 2 · log(vmax ) + 1, the claim
                      follows.

                    For single-minded players, R̄ is the set of losing players, hence we conclude:

                      Theorem 12.45 Given any c-approximation. for KSM players, the Wrapper
                      with KSM-PROC implements an O(log(vmax ) · c) approximation. in dominant
                      strategies.

                    A subprocedure for single-value players. Two assumptions are relaxed: players
                    are now multiminded, and their desired bundles are unknown. Here, we define the
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   August 3, 2007   17:17




                                              alternative solution concepts                                   325

                    following specific subprocedure. For a set of players X, let Free(X, s j +1 ) denote the
                                       j
                    items not in ∪i∈X si .

                                                                                      j
                      Definition 12.46 (1-CA-PROC) Let Mj = argmaxi∈N {vi }, GREEDY j = ∅.
                                               j
                      For every player i with vi > 0, in descending order of values, perform:
                                                                                                       j +1
                        Shrinking the winning set: If i ∈ / Wj allow him to pick a bundle si ⊆
                                             j +1     j         j +1    √
                        Free(GREEDY j , s ) ∩ si such that |si | ≤ m. In any other case (i ∈ Wj
                                                 j +1   j
                        or i does not pick) set si = si .
                                                             j +1    √
                        Updating the current winners: If |si | ≤ m, add i to any of the alloca-
                                                                      j +1
                        tions W ∈ {Wj , Mj , GREEDY j } for which si ⊆ Free(W, s j +1 ).
                                                                                
                      Output s j +1 and W ∈ {Wj , Mj , GREEDY j } that maximizes i∈W vi .
                                                                                        j



                       Recall that the nonwinners then either double their value or retire, and we reiterate.
                    This is the main conceptual difference from “regular” direct revelation mechanisms:
                    here, the players themselves gradually determine their winning set (focusing on one
                    of their desired bundles), and their price. Intuitively, it is not clear how a “reasonable”
                    player should shrink his winning set, when approached. Ideally, a player should focus
                    on a desired bundle that intersects few, low-value competitors. But in early iterations
                    this information is not available. Thus there is no clear-cut on how to shrink the winning
                    set, and the resulting mechanism does not contain a dominant strategy. This is exactly
                    the point where we use the new notion of algorithmic implementation.

                    Analysis. We proceed by characterizing the required set D of strategies. We say
                    that player i is “loser-if-silent” at iteration j if, when asked to shrink her bundle by
                                        j
                    1-CA-PROC, vi ≥ v̄i /2 (retires if losing), i ∈     / Wj and i ∈/ Mj (not a winner), and
                     j              j +1           j              j +1
                    si ∩ (∪i  ∈Wj si  ) = ∅ and si ∩ (∪i  ∈Mj si  ) = ∅ (remains a loser after pareto). In
                    other words, a loser-if-silent loses (regardless of the others’ actions) unless she shrinks
                    her winning set. Let D be all strategies that satisfy, in every iteration j :
                         j        j                             j
                     (i) vi ≤ vi (si ), and, if i retires at j then vi ≥ v̄i /2.
                                                                                             j +1
                    (ii) If i is “loser-if-silent” then she declares a valid desired bundle si , if such a bundle
                         exists.
                    There clearly exists a (poly-time) algorithm to find a strategy st  ∈ D that dominates a
                    given strategy st. Hence, D satisfies the second requirement of algorithmic implemen-
                    tation. It remains to show that the approximation is achieved for every combination of
                    strategies from D.
                                                           √
                      Lemma 12.47        1-CA-PROC is an O( m)-local-approximation w.r.t. D.

                      proof (sketch). The pareto, improvement, and value-bounds properties are
                                                                                                       √
                      immediate from the definition of the procedure and the set D. The O( m)-
                      algorithmic-approximation property follows from the following argument. We
                                                         j
                      need to bound OPT = OPT ({(vi , S̄i |s j ) | i retired at iteration j }) by the sum of
                                                               i
                      values of the players in Wj +1 . We divide the winners in OPT to four sets. Those
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   August 3, 2007   17:17




                    326       computationally efficient approximation mechanisms

                      that are in Mj , GREEDY j , Wj , or in none of the above. For the first three sets
                      the 1-CA-PROC explicitly verifies our need. It remains to handle players in the
                      forth set. First notice that such a player is loser-if-silent. If such a player receives
                                                            √
                      in OPT a bundle with size at least m we match him to the player with the highest
                                                              √
                      value in Mj . There can be at most m players in OPT with bundles of size at
                            √                   √
                      least m, so we lose a m factor for these players. If a player, i, in the forth set,
                                                                      √
                      receives in OPT a bundle with size at most m, let si∗ be that bundle. Since he is
                      a loser-if-silent, there exists i  ∈ GREEDY j such that si  ∩ si∗ = ∅ and vi ≤ vi  .
                                                                                     j                 j    j
                                                                                                 ∗     ∗
                      We map i to i . For any i1 , i2 that were mapped to i we have that si1 ∩ si2 = ∅
                                                                          j              √
                      since both belong to OPT . Since the size of si  is at most m it follows that at
                            √                                                   √
                      most m players can be mapped to i  , so we lose a m factor for these players
                      as well. This completes the argument.
                       In the single-value case, R̄ does not contain all players, so we cannot repeat the
                    argument from the KSM case that immediately linked local approximation and global
                    approximation. However, Claim 12.44 still holds, and we use R̄ as an intermediate set
                    of “virtual” players. The link to the true players is as follows (recall that m denotes the
                    number of items).
                      Definition 12.48 (First-time shrink) PROC satisfies “first time shrink” if for
                                           j           j +1       j +1  j +1
                      any i1 , i2 ∈ {i : |si | = m & |si | < m}, si1 ∩ si2 = ∅.

                    1-CA-PROC satisfies this since any player that shrinks his winning bundle is added to
                    GREEDY j .
                      Lemma 12.49 Given a c-local-approximation (w.r.t. D) that satisfies first-time
                      shrink, the Wrapper obtains an O(log2 (vmax ) · c) approximation for any profile of
                      strategies in D.

                      proof We continue to use the notation of Claim 12.44. Let P = {(v̄i , S̄i ) :
                      i lost, and |siJ | < m}. Players in P appear with all their desired bundles, while
                      players in R̄ appear with only part of their desired bundles. However, ignoring
                      the extra bundles in P incurs only a bounded loss:
                      Claim 12.50       OPT (P ) ≤ J · OPT (R̄).

                      proof Define Pj to be all players in P that first shrank their bundle at iteration
                                                                                              j     j
                      j . By “first-time shrink,” and since winning    bundles only shrink, si1 ∩ si2 = ∅
                      for every i1 , i2 ∈ Pj . Therefore OPT (R̄) ≥ i∈Pj v̄i : every player i in Pj cor-
                      responds to a player in R̄, and all these players have disjoint bundles inR̄ since
                                                          j
                      the bundles of i are contained in si . We also trivially have OPT (Pj ) ≤ i∈Pj v̄i .
                                                                                     
                      Thus, for any j , OPT (Pj ) ≤ OPT (R̄), and OPT (P ) ≤ j OPT (Pj ) ≤ J ·
                      OPT (R̄).
                         To prove the lemma, first notice that all true players are contained in P ∪
                      R̄ ∪ WJ : all retiring players belong to R̄ ∪ P (if a player shrank his bundle then
                      he belongs to P with all his true bundles, and if a player did not shrink his
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    August 3, 2007   17:17




                                                      bibliographic notes                                      327

                      bundle at all then he belongs to R̄ with all his true bundles) and all nonretiring
                      players belong to WJ . From the above we have OPT       (P ∪ R̄) ≤ OPT       (P ) +
                      OPT (R̄) ≤ J · OPT (R̄) + OPT (R̄) ≤ 4 · J 2 · c · i∈WJ v̄iJ    . Since siJ contain
                      some desired bundle of player i, we have that OPT (WJ ) = i∈WJ v̄i . Thus we
                                                                  
                      get that OPT (P ∪ R̄ ∪ WJ ) ≤ 5 · J 2 · c̃ · i∈WJ v̄iJ . Since J ≤ 2 · log(vmax ) + 1
                      by Lemma 12.38, the lemma follows.

                    By all the above, we conclude the following.

                      Theorem 12.51 The Wrapper with 1-CA-PROC is an algorithmic implementa-
                      tion of an O(log2 (vmax ) · c)-approximation for single-value players.

                    This result has demonstrated that if we are less interested in reaching an equilibrium
                    point, but rather in guaranteeing a good-enough outcome, then alternative solution
                    concepts, that are no worse than classic dominant strategies, can be of much help.
                    However, the true power of relaxing dominant strategies to undominated strategies was
                    not formally settled.
                    Open Question Does there exist a domain in which a computationally efficient
                    algorithmic implementation achieves a better approximation than any computationally
                    efficient dominant-strategy implementation?

                                                  12.6 Bibliographic Notes

                    The connection between classic scheduling and mechanism design was suggested by
                    Nisan and Ronen (2001), that studied unrelated machines and reached mainly im-
                    possibilities. Archer and Tardos (2001) studied the case of related machines, and the
                    monotonicity characterization of Section 12.2 is based on their work. Deterministic
                    mechanisms for the problem have been suggested by several works, and the algorithm
                    presented here is by Andelman, Azar, and Sorani (2005). The current best approxi-
                    mation ratio, 3, is given by Kovacs (2005). Section 12.3 is based on the work of Lavi
                    and Swamy (2005). Roberts (1979) characterized dominant strategy implementability
                    for unrestricted domains. The proof given here is based on Lavi, Mu’alem, and Nisan
                    (2004). Generalized-WMON was suggested by Lavi, Mu’alem, and Nisan (2003),
                    which explored the same characterization question for restricted domains in general,
                    and for CAs in particular. Section 12.5 is based on the work of Babaioff, Lavi, and
                    Pavlov (2006). There have been several other suggestions for alternative solution con-
                    cepts. For example, Kothari et al. (2005) describe an “almost truthful” deterministic
                    FPAS for multiunit auctions, and Lavi and Nisan (2005) define a notion of “Set-Nash”
                    for multi-unit auctions in an online setting, for which they show that deterministic truth-
                    fulness obtains significantly lower approximations than Set-Nash implementations.

                                                           Bibliography
                    N. Andelman, Y. Azar, and M. Sorani. Truthful approximation mechanisms for scheduling selfish
                      related machines. In Proc. of the 22nd Intl. Symp. Theor. Asp. Comp. Sci. (STACS), pp. 69–82,
                      2005.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    August 3, 2007    17:17




                    328         computationally efficient approximation mechanisms

                    A. Archer and E. Tardos. Truthful mechanisms for one-parameter agents. In Proc. of the 42nd Annual
                       Symp. Fdns. of Computer Science, 2001.
                    M. Babaioff, R. Lavi, and E. Pavlov. Single-value combinatorial auctions and implementation in
                       undominated strategies. In Proc. of the 17th Symp. Discrete Algorithms, 2006.
                    M. Balcan, A. Blum, J. Hartline, and Y. Mansour. Mechanism design via machine learning. In Proc.
                       of the 46th Annual Symp. Fdns. of Computer Science, 2005.
                    Y. Bartal, R. Gonen, and N. Nisan. Incentive compatible multi-unit combinatorial auctions. In Proc.
                       of the 9th Conf. Theoretical Aspects of Rationality and Knowledge (TARK), 2003.
                    C. Borgs, J. Chayes, N. Immorlica, M. Mahdian, and A. Saberi. Multi-unit auctions with budget-
                       constrained bidders. In Proc. of the 6th ACM Conf. Electronic Commerce (ACM-EC), 2005.
                    S. Dobzinski and N. Nisan. Approximations by computationally-efficient vcg-based mechanisms,
                       2006. Working paper.
                    S. Dobzinski, N. Nisan, and M. Schapira. Approximation algorithms for combinatorial auctions with
                       complement-free bidders. In Proc. of the 37th ACM Symp. Theory of Computing, 2005.
                    S. Dobzinski, N. Nisan, and M. Schapira. Truthful randomized mechanisms for combinatorial auc-
                       tions. In Proc. of the 38th ACM Symp. Theory of Computing, 2006.
                    R. Holzman, N. Kfir-Dahav, D. Monderer, and M. Tennenholtz. Bundling equilibrium in combinatorial
                       auctions. Games Econ. Behav., 47:104–123, 2004.
                    A. Kothari, D. Parkes, and S. Suri. Approximately-strategy proof and tractable multi-unit auctions.
                       Decis. Support Systems, 39:105–121, 2005.
                    A. Kovacs. Fast monotone 3-approximation algorithm for scheduling related machines. In Proc. of
                       the 13th Annual Eur. Symp. Algo. (ESA), 2005.
                    R. Lavi, A. Mu’alem, and N. Nisan. Towards a characterization of truthful combinatorial auctions. In
                       Proc. of the 44th Annual Symp. Fdns. of Computer Science, 2003.
                    R. Lavi, A. Mu’alem, and N. Nisan. Two simplified proofs for Roberts’ theorem, 2004. Working
                       paper.
                    R. Lavi and N. Nisan. Online ascending auctions for gradually expiring items. In Proc. of the 16th
                       Symp. on Discrete Algorithms, 2005.
                    R. Lavi and C. Swamy. Truthful and near-optimal mechanism design via linear programming. In
                       Proc. of the 46th Annual Symp. Fdns. of Computer Science, 2005.
                    N. Nisan and A. Ronen. Algorithmic mechanism design. Games and Economic Behavior, 35:166–
                       196, 2001.
                    K. Roberts. The characterization of implementable choice rules. In Jean-Jacques Laffont, editor,
                       Aggregation and Revelation of Preferences, pp. 321–349, North-Holland, 1979.



                                                               Exercises
                    12.1 (Scheduling related machines) Find an implementable algorithm that exactly ob-
                         tains the optimal makespan, for scheduling on related machines (since this is an
                         NP-hard problem, obviously you may ignore the computational complexity of your
                         algorithm).
                    12.2 (Scheduling unrelated machines) In the model of unrelated machines, each job j
                         creates a load pi j on each machine i , where the loads are completely unrelated.
                         Prove, using W-MON, that no truthful mechanism can approximate the makespan
                         with a factor better than 2. Hint: Start with four jobs that have pi j = 1 for all i, j .
                    12.3 A deterministic greedy rounding of the fractional scheduling 12.4 assigns each
                         job in full to the first machine that got a fraction of it. Explain why this is a 2-
                         approximation, and show by an example that this violates monotonicity.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    August 3, 2007   17:17




                                                             exercises                                          329

                    12.4 Prove that 1-CA-PROC of Definition 12.46, and Greedy for multiminded players
                         of Definition 12.20 are not dominant-strategy implementable.
                    12.5 (Converting algorithms to mechanisms) Fix an alternative set A, and suppose that
                         for any player i , there is a fixed, known subset Ai ⊂ A, such that a valid valua-
                         tion assigns some positive real number in [vmin , vmax ] to every alternative in Ai ,
                         and zero to the other alternatives. Suppose vmin and vmax are known. Given a
                         c-approximation algorithm to the social welfare for this domain, construct a ran-
                         domized truthful mechanism that obtains a O(log(vmax /vmin ) · c) approximation to
                         the social welfare. (Hint: choose a threshold price, uniformly at random). Is this
                         construction still valid when the sets Ai are unknown? (If not, show a counter
                         example).
                    12.6 Describe a domain for which there exists an implementable social choice function
                         that does not satisfy Generalized-WMON.
                    12.7 Describe a deterministic CA for general valuations that is not an affine maximizer.
                    12.8 This exercise aims to complete the characterization of Section 12.4:
                                                            ∈ P (x, y) }. Show that γ (x, y) is well-defined, that
                         Let γ (x, y) = i nf { p ∈  | p · 1
                         γ (x, y) = −γ (y, x), and that γ (x, z) = γ (x, y) + γ (y, z). Let C(x, y) = {α − γ (x, y) ·
                          | α ∈ P (x, y) }. Show that for any x, y, w, z ∈ A, the interior of C(x, y) is equal to
                         1
                         the interior of C(w, z). Use this to show that C(x, y) is convex.
                         Conclude, by the separation lemma, that f is an affine maximizer (give an explicit
                         formula for the additive terms C x ).
P1: SBT
9780521872829main   CUNY1061-Nisan   0 521 87282 0   August 3, 2007   17:17
