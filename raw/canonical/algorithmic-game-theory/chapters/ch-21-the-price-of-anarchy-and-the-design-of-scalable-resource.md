---
type: "book-chapter"
book_id: "algorithmic-game-theory"
chapter_id: "ch-21"
chapter_number: 21
chapter_title: "The Price of Anarchy and the Design of Scalable Resource"
source_pdf: "raw/inbox/manual-drop/PDF_B.pdf"
source_page_start: 564
source_page_end: 591
printed_page_start: 564
printed_page_end: 589
part_ids: ["algorithmic-game-theory-ch-21-part-022"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# The Price of Anarchy and the Design of Scalable Resource

P1: SBT
9780521872829main       CUNY1061-Nisan       0 521 87282 0     July 5, 2007    14:35




                                                              CHAPTER 21


                               The Price of Anarchy and the
                               Design of Scalable Resource
                                 Allocation Mechanisms

                                                             Ramesh Johari




                                                                 Abstract

                    In this chapter, we study the allocation of a single infinitely divisible resource among multiple
                    competing users. While we aim for efficient allocation of the resource, the task is complicated by the
                    fact that users’ utility functions are typically unknown to the resource manager. We study the design
                    of resource allocation mechanisms that are approximately efficient (i.e., have a low price of anarchy),
                    with low communication requirements (i.e., the strategy spaces of users are low dimensional).
                        Our main results concern the proportional allocation mechanism, for which a tight bound on
                    the price of anarchy can be provided. We also show that in a wide range of market mechanisms
                    that use a single market-clearing price, the proportional allocation mechanism minimizes the price
                    of anarchy. Finally, we relax the assumption of a single market-clearing price, and show that by
                    extending the class of Vickrey–Clarke–Groves mechanisms all Nash equilibria can be guaranteed to
                    be fully efficient.




                                                          21.1 Introduction

                    This chapter deals with a canonical resource allocation problem. Suppose that a finite
                    number of users compete to acquire a share of an infinitely divisible resource of fixed
                    capacity. How should the resource be shared among the users? We will frame this
                    problem as an economic problem: we assume that each user has a utility function that
                    is increasing in the amount of the resource received, and then design a mechanism
                    to maximize aggregate utility. In the absence of any strategic considerations, this is a
                    simple optimization problem; however, if we assume that the agents are strategic, we
                    need to design the resource allocation mechanisms to be robust to gaming behavior.
                        A central theme of this chapter is that the price of anarchy can be used as a design
                    metric; i.e., “robust” allocation mechanisms are those that have a low price of anarchy.
                    The present chapter is thus a bridge between two different themes of the book. The
                    first theme is that of optimal mechanism design (Part II): given selfish agents, how do
                    we successfully design mechanisms that nevertheless yield efficient outcomes? The
                                                                     543
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:35




                    544      the design of scalable resource allocation mechanisms

                    second theme is that of quantifying inefficiency (Part III): given a prediction of game
                    theoretic behavior, how well does it perform relative to some efficient benchmark? In
                    this chapter, we use the quantification of inefficiency as the “objective function” with
                    which we will design optimal mechanisms. As we will see, for the resource allocation
                    problems we consider, this approach yields surprising insights into the structure of
                    optimal mechanisms.
                       The mechanisms we consider for resource allocation are motivated by constraints
                    present in modern communication networks, and similar systems where communication
                    is limited; this precludes use of the traditional Vickrey–Clarke–Groves mechanisms
                    (Chapter 9), which require declaration of the entire utility function. If we interpret the
                    single resource above as a communication link, then we view the mechanism as an
                    allocation policy operating on that link. We wish to design mechanisms that, intuitively,
                    impose low communication overhead on the overall system; throughout this chapter,
                    that scalability constraint translates into the assumption that the players can use only
                    low-dimensional (in fact, one-dimensional) strategy spaces.
                       The remainder of the chapter is organized as follows. In Section 21.2, we introduce
                    the basic resource allocation model we will consider in this chapter, and then introduce
                    a simple approach to allocating the fixed resource: the proportional allocation mecha-
                    nism. In this mechanism, each user submits a bid, and receives a share of the resource
                    in proportion to their bid. We analyze this model under both the assumption that users
                    are price takers (i.e., that they do not anticipate the effect of their strategic decision
                    on the price of the resource); and the assumption that users are price anticipators.
                    The former case yields full efficiency, while in the latter we characterize the price of
                    anarchy. In Section 21.3, we state and prove a theorem showing that in a nontrivial
                    class of “scalable” market mechanisms (in the sense informally discussed above), the
                    proportional allocation mechanism has the lowest price of anarchy (i.e., minimizes the
                    efficiency loss) when users are price anticipating.
                       In all the mechanisms considered in the first two sections, players have one-
                    dimensional strategy spaces, and the mechanism also only chooses a single price.
                    Because of these constraints, even the highest performance mechanisms suffer a posi-
                    tive efficiency loss, as demonstrated in Section 21.3. In the final section of the chapter,
                    we consider the implications of removing the “single price” constraint. We show in
                    Section 21.4 that if we consider mechanisms with scalar strategy spaces, and allow the
                    mechanism to choose one price per user of the resource, then in fact full efficiency is
                    achievable at Nash equilibrium. The result involves extending the well-known class of
                    Vickrey–Clarke–Groves (VCG) mechanisms to use only a scalar strategy space; for
                    more on VCG mechanisms, see Chapter 9.


                                    21.2 The Proportional Allocation Mechanism

                    Suppose that R users share a resource of capacity C > 0. Let dr denote the amount
                    allocated to user r. We assume that user r receives a utility equal to Ur (dr ) if the
                    allocated amount is dr ; we assume that utility is measured in monetary units. We make
                    the following assumptions on the utility function; we emphasize that this assumption
                    will be in force for the duration of the chapter, unless otherwise mentioned.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 5, 2007      14:35




                                        the proportional allocation mechanism                                   545

                    Assumption 1 For each r, over the domain dr ≥ 0 the utility function Ur (dr ) is
                    concave, strictly increasing, and continuous; and over the domain dr > 0, Ur (dr ) is
                    continuously differentiable. Furthermore, the right directional derivative at 0, denoted
                    Ur (0), is finite. We let U denote the set of all utility functions satisfying these conditions.

                       We note that we make rather strong differentiability assumptions here on the utility
                    functions; these assumptions are primarily made to ease the presentation. It is possible
                    to relax the differentiability assumptions (see Notes for details).
                       Given complete knowledge and centralized control of the system, a natural problem
                    for the network manager to try to solve is the following optimization problem:

                    SYSTEM:
                                                             
                                                maximize          Ur (dr )                                    (21.1)
                                                              r
                                                             
                                                subject to        dr ≤ C;                                     (21.2)
                                                              r
                                                             dr ≥ 0,        r = 1, . . . , R.                 (21.3)
                    Note that the objective function of this problem is the utilitarian social welfare function
                    (cf. Chapter 17); it becomes a reasonable objective if we assume that all utilities are
                    measured in the same (monetary) units. Since the objective function is continuous
                    and the feasible region is compact, an optimal solution d = (d1 , . . . , dR ) exists. If the
                    functions Ur are strictly concave, then the optimal solution is unique, since the feasible
                    region is convex.
                       In general, the utility functions are not available to the resource manager. As a result,
                    we consider the following pricing scheme for resource allocation, which we refer to as
                    the proportional allocation mechanism. Each user r gives a payment (also called a bid)
                    of wr to the resource manager; we assume wr ≥ 0. Given the vector w = (w1 , . . . , wr ),
                    the resource manager chooses an allocation d = (d1 , . . . , dr ). We assume the manager
                    treats all users alike—in other words, the network manager does not price discriminate.
                    Each user is charged the same price µ > 0, leading to dr = wr /µ. We further assume
                    that the manager always seeks to allocate the entire resource capacity C; in this case,
                    we expect the price µ to satisfy
                                                              wr
                                                                     = C.
                                                              r
                                                                  µ
                                                                       
                    The preceding equality can only be satisfied if r wr > 0, in which case we have
                                                                   
                                                                      wr
                                                             µ= r .                                       (21.4)
                                                                     C
                    In other words, if the manager chooses to allocate the entire resource, and does not
                    price discriminate between users, then for every nonzero w there is a unique price
                    µ > 0, which must be chosen by the network, given by the previous equation.
                       We can interpret this mechanism as a market-clearing process by which a price is set
                    so that demand equals supply. To see this interpretation, note that when a user chooses
                    a total payment wr , it is as if the user has chosen a demand function D(p, wr ) = wr /p
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0    July 5, 2007      14:35




                    546      the design of scalable resource allocation mechanisms

                    for p > 0. The demand function describes the quantity the user demands
                                                                                               at any given
                    price p > 0. The resource manager then chooses a price µ so that r D(µ, wr ) = C,
                    i.e., so that the aggregate demand equals the supply C. For the specific form of demand
                    functions we consider here, this leads to the expression for µ given in (21.4). User r
                    then receives an allocation given by D(µ, wr ), and makes a payment µD(µ, wr ) = wr .
                    This interpretation will be further explored in Section 21.3, where we consider other
                    market-clearing mechanisms for allocating a single resource in inelastic supply, with
                    the users choosing demand functions from a family parameterized by a single scalar.


                                21.2.1 Price Taking Users and Competitive Equilibrium
                    In this section, we consider a competitive equilibrium between the users and the resource
                    manager. A central assumption in the definition of competitive equilibrium is that each
                    user does not anticipate the effect of their payment wr on the price µ; i.e., each user
                    acts as a price taker. In this case, given a price µ > 0, user r acts to maximize the
                    following payoff function over wr ≥ 0:
                                                                     
                                                                     wr
                                                   Pr (wr ; µ) = Ur       − wr .                        (21.5)
                                                                      µ
                    The first term represents the utility to user r of receiving a resource allocation equal
                    to wr /µ; the second term is the payment wr made to the manager. Observe that this
                    definition is consistent with the notion that all utilities are measured in monetary units.
                       We now say a pair (w, µ) with w ≥ 0 and µ > 0 is a competitive equilibrium if
                    users maximize their payoff as defined in (21.5), and the network “clears the market”
                    by setting the price µ according to (21.4):
                                     Pr (wr ; µ) ≥ Pr (w̄r ; µ) for w̄r ≥ 0, r = 1, . . . , R;     (21.6)
                                                   
                                                        w  r
                                              µ= r .                                               (21.7)
                                                      C
                       The following theorem shows that under our assumptions, a competitive equilibrium
                    always exists, and any competitive equilibrium maximizes aggregate utility.

                      Theorem 21.1 There exists a competitive equilibrium (w, µ). In this case, the
                      vector d = w/µ is an optimal solution to SYSTEM.

                      proof The key idea in the proof is to use Lagrangian techniques to establish that
                      optimality conditions for (21.6)–(21.7) are identical to the optimality conditions
                      for the problem SYSTEM, under the identification d = w/µ.
                         Observe that under Assumption 1, the payoff (21.5) is concave in wr for any
                      µ > 0. Thus considering the first-order condition for maximization of Pr (wr ; µ)
                      over wr ≥ 0, we conclude w and µ are a competitive equilibrium if and only if
                                                    Ur (dr ) = µ,        if dr > 0;                    (21.8)
                                                     Ur (0) ≤ µ,         if dr = 0;                    (21.9)
                                                     
                                                          dr = C,                                      (21.10)
                                                      r
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0   July 5, 2007     14:35




                                        the proportional allocation mechanism                                   547

                      where dr = wr /µ. A straightforward Lagrangian optimization shows that the pre-
                      ceding conditions are exactly the optimality conditions for the problem SYSTEM,
                      so we conclude w and µ are a competitive equilibrium if and only if d = w/µ is
                      a solution to SYSTEM with Lagrange multiplier µ. Since at least one solution to
                      SYSTEM must exist, the proof is complete.

                       Theorem 21.1 shows that under the assumption that the users of the resource behave
                    as price takers, there exists a bid vector w whereall users have optimally chosen their
                    bids wr , with respect to the given price µ = r wr /C; and at this “equilibrium,”
                    aggregate utility is maximized. However, when the price taking assumption is violated,
                    the model changes into a game and the guarantee of Theorem 21.1 is no longer valid.
                    We investigate this game in the following section.


                                 21.2.2 Price Anticipating Users and Nash Equilibrium
                    We now consider an alternative model where the users of a single resource are price
                    anticipating, rather than price takers. The key difference is that while the payoff function
                    Pr takes the price µ as a fixed parameter in (21.5), price anticipating users will realize
                    that µ is set according to (21.4), and adjust their payoff accordingly; this makes the
                    model a game between the R players.
                        We use the notation w−r to denote the vector of all bids by users other than r;
                    i.e., w−r = (w1 , w2 , . . . , wr−1 , wr+1 , . . . , wR ). Given w−r , each user r chooses wr to
                    maximize:
                                                          ⎧                   
                                                          ⎪
                                                          ⎪
                                                                       wr
                                                          ⎨Ur               C − wr , if wr > 0;
                                     Qr (wr ; w−r ) =                   s ws                                 (21.11)
                                                          ⎪
                                                          ⎪
                                                          ⎩
                                                            Ur (0),                       if wr = 0.
                    over nonnegative wr . The second condition is required so that the resource
                                                                                             allocation to
                    user r is zero when wr = 0, even if all other users choose w−r so that s=r ws = 0. The
                    payoff function Qr is similar to the payoff function Pr , except that the user anticipates
                    that the network will set the price µ according to (21.4). A Nash equilibrium of the
                    game defined by (Q1 , . . . , QR ) is a vector w ≥ 0 such that for all r:

                                         Qr (wr ; w−r ) ≥ Qr (w̄r ; w−r ),     for all w̄r ≥ 0.            (21.12)

                     Note that the payoff function in (21.11) may be discontinuous at wr = 0, if
                       s=r ws = 0. This discontinuity may preclude existence of a Nash equilibrium; it
                    is easy to see this in the case where the system consists of only a single user with a
                    strictly increasing utility function. Nevertheless, as long as at least two users are com-
                    peting, it is possible to show that a unique Nash equilibrium exists, by noting that such
                    an equilibrium solves a version of the SYSTEM problem but with “modified” utility
                    functions.

                      Theorem 21.2 Suppose that R > 1. Then there exists a unique           Nash equilib-
                      rium w ≥ 0 of the game defined by (Q1 , . . . , QR ), and it satisfies r wr > 0. In
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0    July 5, 2007         14:35




                    548       the design of scalable resource allocation mechanisms

                      this case, the vector d defined by
                                                       wr
                                               dr =        C,         r = 1, . . . , R,                         (21.13)
                                                       s ws

                      is the unique optimal solution to the following optimization problem:

                      GAME:
                                                            
                                                maximize         Ûr (dr )                                       (21.14)
                                                             r
                                                            
                                              subject to         dr ≤ C;                                         (21.15)
                                                             r
                                                            dr ≥ 0,         r = 1, . . . , R,                    (21.16)
                      where
                                                                                         dr           
                                                 dr                        dr        1
                                 Ûr (dr ) = 1 −          Ur (dr ) +                               Ur (z) dz .   (21.17)
                                                 C                         C         dr   0

                      proof The proof is similar to the proof of Theorem 21.1. The first key step
                      is to note that at any Nash equilibrium, at least two components of w must be
                      positive; this follows from the payoff (21.11) (see Exercise 17.5). Given this fact,
                      the payoff of each user wr is strictly concave and continuous in wr so that w is a
                      Nash equilibrium if and only if the following first-order conditions hold:
                                                                
                                          wr                wr            ws
                                    Ur         C    1−             = s , if wr > 0;               (21.18)
                                           s ws              s ws        C
                                                                       
                                                                          ws
                                                              Ur (0) ≤ s , if wr = 0.               (21.19)
                                                                         C
                                                   
                      Note that if we define ρ = s ws /C and dr = wr /ρ, then the preceding condi-
                      tions can be rewritten as
                                                    Ûr (dr ) = ρ,        if dr > 0;                            (21.20)
                                                     Ûr (0) ≤ ρ,         if dr = 0;                            (21.21)
                                                     
                                                          dr = C.                                                (21.22)
                                                      r

                      Note that these are identical to (21.8)–(21.10), but for the modified objective func-
                      tion (21.14). Since the utility functions Ûr (dr ) are strictly concave and continuous
                      over 0 ≤ dr ≤ C, the preceding first-order conditions are sufficient optimality
                      conditions for GAME. We conclude that w is a Nash equilibrium if and only if
                      
                         s ws > 0, and the resulting
                                                       allocation d solves the problem GAME with La-
                      grange multiplier ρ = s ws /C. To conclude the proof, observe that GAME has
                      a strictly concave and continuous objective function over a compact feasible re-
                      gion, and thus has a unique optimal solution. It is straightforward to verify that
                      this implies uniqueness of the Nash equilibrium as well.

                      Note that the preceding theorem gives a form of “potential” for the game under
                    consideration: the Nash equilibrium is characterized as the unique solution to a natural
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0         July 5, 2007    14:35




                                        the proportional allocation mechanism                             549

                    optimization problem. However, the objective function for this optimization problem
                    is not a true (exact or ordinal) potential for the game under consideration; this is
                    because while the objective function (21.14) depends on allocations, the users’ strategic
                    decisions are bids. Notably, this observation is in sharp contrast to the potentials found
                    for routing games in Chapter 18, or for network formation in Chapter 19. For example,
                    we cannot use the objective function (21.14) to conclude that best response dynamics
                    will converge for our game. Nevertheless, the optimization formulation will help us
                    study the price of anarchy of the game in the following section. For later reference,
                    we note the following corollary, which uses a variational inequality formulation of the
                    preceding theorem.


                      Corollary 21.3 Suppose that R > 1. Let w be the unique Nash equilibrium of
                      the game defined by (Q1 , . . . , QR   ), and define d according to (21.13). Then for
                      any other vector d̄ ≥ 0 such that r d̄r ≤ C, there holds:
                                                  
                                                        Ûr (dr )(d̄r − dr ) ≤ 0.                     (21.23)
                                                        r


                      proof The stated condition follows easily from (21.20)–(21.22), the optimality
                      conditions for the problem GAME.


                                                   21.2.3 Price of Anarchy
                    We let dS denote an optimal solution to SYSTEM, and let dG denote the unique optimal
                    solution to GAME. We now investigate the price of anarchy of this system; i.e., how
                                               the users are price anticipating? To answer this question, we
                    much utility is lost because
                    must compare the utility r Ur (drG ) obtained  when Sthe users fully evaluate the effect
                                                                                    by choosing
                    of their actions on the price, and the utility r Ur (dr ) obtained             the point
                    that maximizes aggregate utility. (We know, of course, that r Ur (drG ) ≤ r Ur (drS ),
                    by definition of dS .) As we show in the following theorem, the efficiency loss is exactly
                    25% in the worst case.


                      Theorem 21.4 Suppose that R > 1. Suppose also that Ur (0) ≥ 0 for all r. If
                      dS is any optimal solution to SYSTEM, and dG is the unique optimal solution to
                      GAME, then:
                                                                         3
                                                            Ur (drG ) ≥       Ur (drS ).
                                                    r
                                                                          4 r

                      Furthermore, this bound is tight: for every  > 0, there exists a choice of R, and
                      a choice of (linear) utility functions Ur , r = 1, . . . , R, such that

                                                                           
                                                                     3
                                                 Ur (drG ) ≤           +            Ur (drS ) .
                                             r
                                                                     4           r
P1: SBT
9780521872829main       CUNY1061-Nisan           0 521 87282 0        July 5, 2007      14:35




                    550         the design of scalable resource allocation mechanisms

                       proof Our proof will rely on the following constant β:1
                                                                          U (d) + Û  (d)(d̄ − d)
                                                 β = inf inf          inf                          .                          (21.24)
                                                        U ∈U C>0 0≤d,d̄≤C         U (d̄)
                       Recall the definition of U in Assumption 1, and of Û in (21.17).
                          Our proof involves using Corollary 21.3 to prove that β is a tight bound on the
                       efficiency of Nash equilibria. We first establish that β ≥ 3/4. Note that in (21.24),
                       the quotient is strictly larger than 1 if d > d̄, and equal to 1 if d = d̄. Thus in
                       computing β we can assume that d < d̄ in (21.24). We then have:
                                                                                       
                                                                                  d
                                   U (d) + Û (d)(d̄ − d) = U (d) + U (d) 1 −             (d̄ − d)
                                                                                    C
                                                                               
                                                                             d
                                                          ≥ U (d) + 1 −           (U (d̄) − U (d))
                                                                             d̄
                                                              2                      
                                                                d                    d
                                                          ≥         U (d̄) + 1 −          U (d̄)
                                                                d̄                   d̄
                                                             3
                                                          ≥ U (d̄).
                                                             4
                       The first inequality follows since d̄ ≤ C and U is concave. The second inequality
                       follows since U is concave and nonnegative and d ≤ d̄, so U (d) ≥ (d/d̄)U (d̄).
                       Finally, the third inequality follows since x 2 − x + 1 is minimized at x = 1/2. It
                       follows from (21.24) that β ≥ 3/4.
                           Next, we show that for any δ > 0, there exists an example where the ratio
                       of Nash aggregate utility to maximum aggregate utility is at least β + δ. Our
                       approach is essentially the same as that in Example 17.6. Fix U , d < d̄, and let
                       C = d̄. Consider the following example. Suppose that R > 1 users compete for the
                       resource. Let user 1 have utility function U1 = U , and suppose users 2, . . . , R have
                       linear utility functions with slope Û  (d); i.e., Ur (dr ) = Û  (d)dr = (U  (d)(1 −
                       d/C))dr . Let dS denote an optimal solution to SYSTEM for this model; since
                       one feasible
                                    solution involves allocating the entire resource d̄ to user 1, we must
                       have s Us (dsS ) ≥ U (d̄). On the other hand, recall that at any Nash equilibrium
                       at least two users have positive quantities; and since the Nash equilibrium is
                       unique, we conclude that all users 2, . . . , R receive the same positive quantity.
                       Thus as R → ∞, we must have     dr ↓ 0 for r = 2, . . . , R. From (21.20)–(21.21),
                       it follows that the Nash price s ws /C must converge to Û  (d) as R → ∞. Thus,
                       at the Nash equilibrium, user 1 receives an allocation d + , and all other users
                       receive an allocation (1 − d − )/(R − 1), where  → 0 as R → ∞. The total
                       Nash utility thus converges to U (d) + Û  (d)(d̄ − d). The limiting ratio of Nash
                       aggregate utility to maximum aggregate utility is thus less than or equal to
                                                                U (d) + Û  (d)(d̄ − d)
                                                                                         .
                                                                        U (d̄)

                    1 A slight subtlety arises in this definition if U (x̄) = 0; however, in this latter case we can define β by only taking

                      the infimum over x̄ > 0. This does not change any of the subsequent arguments.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0       July 5, 2007   14:35




                                                    a characterization theorem                                    551

                      We conclude that for any δ > 0, there exists a game (Q1 , . . . , QR ) in which the
                      ratio of Nash aggregate utility to maximum aggregate utility is at most β + δ.
                      By considering the special case in which U (d̂) = d̂, d = 1/2, and d̄ = 1, the
                      preceding construction yields a limiting efficiency ratio of exactly 3/4. Combined
                      with the previous argument that β ≥ 3/4, it follows that in fact β = 3/4.
                         It remains to show that the bound holds for every resource allocation game.
                      Here we simply apply the result of Corollary 21.3. Let (Q1 , . . . , QR ) be a resource
                      allocation game where users have utility functions (U1 , . . . , UR ). Let dS be a
                      solution to SYSTEM, and let dG be a solution to GAME. We have
                                               1                                               1
                                  Us (dsS ) ≤            Us (dsG ) + Ûs (dsG )(dsS − dsG ) ≤       Us (dsG ).
                              s                 s
                                                     β                                           β s

                      The first inequality follows by the definition of β, and the second follows from
                      Corollary 21.3. Since β = 3/4, this concludes the proof.

                       The preceding theorem shows that in the worst case, aggregate utility falls by no
                    more than 25% when users are able to anticipate the effects of their actions on the
                    price of the resource. Furthermore, this bound is essentially tight. In fact, it follows
                    from the proof that the worst case consists of a resource of capacity 1, where user
                    1 has utility U1 (d1 ) = d1 , and all other users have utility Ur (dr ) ≈ dr /2 (when R is
                    large). As R → ∞, at the Nash equilibrium of this game user 1 receives a quantity
                    d1G = 1/2, while the remaining users uniformly split the quantity 1 − d1G = 1/2 among
                    themselves, yielding an aggregate utility of 3/4. On the other hand, the maximum
                    aggregate utility possible is clearly 1, achieved by allocating the entire resource to
                    user 1.



                                            21.3 A Characterization Theorem

                    In this chapter we ask an axiomatic question: Is the mechanism we have chosen
                    “desirable” among a class of mechanisms satisfying certain “reasonable” properties?
                    Defining desirability is the simpler of the two tasks: we consider a mechanism to be
                    desirable if it minimizes efficiency loss when users are price anticipating. Importantly,
                    we ask for this efficiency property independent of the characteristics of the market
                    participants (i.e., their cost functions or utility functions). That is, the mechanisms
                    we seek are those that perform well under broad assumptions on the nature of the
                    preferences of market participants.
                       How do we define “reasonable” mechanisms? The most important condition we
                    impose is that the strategy space of each market participant should be “simple,” which
                    we interpret as low dimensional. Formally, we will focus on mechanisms for which the
                    strategy space of each market participant is R+ ; i.e., each market participant chooses a
                    scalar, which is a parameter that determines his demand function as input to the market-
                    clearing mechanism. The primary motivation is that if we view such a mechanism to
                    be useful for a communication network setting, information flow is limited; and in
                    particular, we would like to implement a market with as little overhead as possible.
P1: SBT
9780521872829main       CUNY1061-Nisan          0 521 87282 0         July 5, 2007    14:35




                    552        the design of scalable resource allocation mechanisms

                    Thus keeping the strategy spaces of the users low dimensional is a reasonable goal.2
                    We will show that under a specific set of mathematical assumptions, the proportional
                    allocation mechanism in fact minimizes the worst-case efficiency loss when users are
                    price anticipating.
                       The class of market mechanisms we will consider is defined as follows. A market
                    mechanism must operate on a particular environment, defined by a triple (C, R, U):
                    C > 0 denotes the capacity of the resource; R > 1 denotes the number of users sharing
                    the resource; and U = (U1 , . . . , UR ) denotes the utility functions of the users, with
                    Ur ∈ U (cf. Assumption 1). The following definition captures our notion of a market
                    mechanism.

                       Definition 21.5 A smooth market-clearing mechanism is a differentiable func-
                       tion D : (0, ∞) × [0, ∞) → R+ such that for all C > 0, for all R > 1, and for
                       all nonzero θ ∈ (R+ )R , there exists a unique solution p > 0 to the following
                       equation:

                                                                 
                                                                 R
                                                                        D(p, θr ) = C.
                                                                  r=1

                       We let pD (θ ) denote this solution.3

                    Note that the market-clearing price is undefined if θ = 0. As we will see below, when
                    we formulate a game between users for a given mechanism D, we will assume that
                    the payoff to all players is −∞ if the composite strategy vector is θ = 0. Note that
                    this is slightly different from the definition in Section 21.1, where the payoff is U (0)
                    to a player with utility function U who submits a strategy θ = 0. We will discuss this
                    distinction further later; we simply note for the moment that it does not affect the results
                    of this section.
                       Our definition of a smooth market-clearing mechanism generalizes the demand
                    function interpretation of the proportional allocation mechanism. Recall that for that
                    mechanism, each user submits a demand function of      the form D(p, θ) = θ/p, and the
                                                                             R
                    link manager chooses a price p   D (θ) to ensure that   r=1 D(p, θr ) = C. Thus, for this
                    mechanism, we have pD (θ ) = Rr=1 θr /C if θ = 0.
                       We now generalize competitive equilibria and Nash equilibria to this setting.

                       Definition 21.6 Given a utility system (C, R, U) and a smooth market-clearing
                       mechanism D, we say that a nonzero vector θ ∈ (R+ )R is a competitive equilib-
                       rium if, for µ = pD (θ ), there holds for all r:

                                                 θr ∈ arg max[Ur (D(µ, θ̄r )) − µD(µ, θ̄r )].                             (21.25)
                                                             θ̄r ≥0


                    2 Note that this notion is distinct from “single-parameter domains” as studied in Chapter 9; there it is the true

                      valuations of the agents that are one-dimensional, whereas here the true valuations of the agents may be arbitrary
                      functions. With one-dimensional strategy spaces, we restrict the ability of users to communicate information
                      about their valuations to the mechanism.
                    3 Note that we suppress the dependence of this solution on C; where necessary, we will emphasize this dependence.
P1: SBT
9780521872829main      CUNY1061-Nisan           0 521 87282 0    July 5, 2007   14:35




                                                     a characterization theorem                                     553

                      Definition 21.7 Given a utility system (C, R, U) and a smooth market-clearing
                      mechanism D, we say that a nonzero vector θ ∈ (R+ )R is a Nash equilibrium if
                      there holds for all r:
                                                          θr ∈ arg max Qr (θ̄r ; θ −r ).                     (21.26)
                                                                    θ̄r ≥0

                      where
                                            
                                                Ur (D(pD (θ), θr )) − pD (θ )D(pD (θ ), θr ),      if θ = 0;
                        Qr (θr ; θ −r ) =                                                                     (21.27)
                                                −∞,                                                if θ = 0.

                    Notice that the payoff Qr is −∞ if the composite strategy vector is θ = 0, since in this
                    case no market-clearing price exists.
                       We are now ready to frame the specific class D of market mechanisms we will
                    consider in this section, defined as follows.

                      Definition 21.8 The class D consists of all functions D(p, θ) such that the
                      following conditions are satisfied:
                        (i) D is a smooth market-clearing mechanism (cf. Definition 21.5).
                       (ii) For all C > 0, and for all Ur ∈ U, a user’s payoff is concave if he is price
                            anticipating; i.e., for all R, and for all θ −r ∈ (R+ )R , the function:
                                                      Ur (D(pD (θ), θr ) − pD (θ)D(pD (θ ), θr )
                           is concave in θr > 0 if θ −r = 0, and concave in θr ≥ 0 if θ −r = 0.
                      (iii) For all p > 0, and for all d ≥ 0, there exists a θ > 0 such that D(p, θ ) = d.
                      (iv) The demand functions are nonnegative; i.e., for all p > 0 and θ ≥ 0, D(p, θ ) ≥ 0.

                       We pause here to briefly discuss the conditions in the previous definition. The
                    second allows us to characterize Nash equilibria in terms of only first-order conditions.
                    To justify this condition, we note that some assumption of quasiconcavity is generally
                    used to guarantee existence of pure strategy Nash equilibria. The third condition ensures
                    that given a price p and desired allocation d ∈ [0, C], each player can make a choice of
                    θ to guarantee precisely the allocation d. This is an “expressiveness” condition on the
                    mechanism that ensures that all possible demands can be chosen at any market-clearing
                    price. The last condition is a normalization condition, which ensures that regardless of
                    the bid of a user, he is never required to supply some quantity of the resource (which
                    would be the case if we allowed D(p, θ) < 0). The following example gives a family
                    of mechanisms that lie in D.

                      Example 21.9 Suppose that D(p, θ) = θp −1/c , where c ≥ 1. It is easy to check
                      that this class of mechanisms satisfies D ∈ D for all choices of c; when c = 1,
                      we recover the proportional allocation mechanism
                                                                          of Section 21.2. The market-
                      clearing condition yields that pD (θ) = ( r θr /C)1/c . Note that as a result, the
                      allocation to user r at a nonzero vector θ is
                                                                     θr
                                                   D(pD (θ), θr ) =  C.
                                                                      s θs
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0   July 5, 2007    14:35




                    554        the design of scalable resource allocation mechanisms

                      In other words, regardless of the value of c, the market clearing allocations are
                      chosen proportional to the bids. This remarkable fact is a special case of a more
                      general result we establish below: all mechanisms in D yield market-clearing
                      allocations that are proportional to the bids; they differ only in the market-clearing
                      price that is chosen. The exercises study the price of anarchy of the mechanisms
                      defined in this example using an approach analogous to the proof of Theorem 21.4.

                       Our interest is in the worst-case ratio of aggregate utility at any Nash equilibrium
                    to the optimal value of SYSTEM. Formally, for D ∈ D we define a constant ρ(D) as
                    follows:
                                             R                          
                                                                          
                                                 r=1 Ur (D(pD (θ ), θr )) 
                               ρ(D) = inf           R                     C > 0, R > 1, U ∈ U ,
                                                                                               R

                                                      r=1 Ur (dr )
                                                                                               
                                              d solves SYSTEM, and θ is a Nash equilibrium .

                    Note that since all U ∈ U are strictly increasing and nonnegative, the aggregate utility
                      R         S
                       r=1 Ur (dr ) is positive for any utility system (C, R, U) with C > 0, and any optimal
                               S
                    solution d to SYSTEM. Note also that we are considering the ratio over all possible
                    Nash equilibria, not just the best one for a given instance; thus, we are studying the
                    price of anarchy, not the price of stability (cf. Chapter 17). However, Nash equilibria
                    may not exist for some utility systems (C, R, U); in this case we set ρ(D) = −∞.
                       Our main result in this section is the following theorem.

                      Theorem 21.10         Let D ∈ D be a smooth market-clearing mechanism. Then:
                          (i) There exists a competitive equilibrium θ . Furthermore, for any such θ , the re-
                              sulting allocation d given by dr = D(pD (θ ), θr ) solves SYSTEM.
                       (ii) There exists a concave, strictly increasing, differentiable, and invertible function
                            B : (0, ∞) → (0, ∞) such that for all p > 0 and θ ≥ 0:
                                                                            θ
                                                             D(p, θ ) =         .
                                                                           B(p)
                      (iii) ρ(D) ≤ 3/4, and this bound is met with equality if and only if D(p, θ ) = θ/p
                            for some  > 0.

                       Before continuing to the proof of the theorem, we pause to make several critical
                    comments about the result. Results (i) and (ii) of the theorem are a characterization of
                    the types of mechanisms allowed by the constraints that define D. In particular, notice
                    that from (ii), for nonzero θ we have
                                                                  R
                                                                         θr
                                                     B(pD (θ )) = r=1 .                             (21.28)
                                                                      C
                    Thus we must have
                                                                        θr
                                                      D(pD (θ), θr ) =  C;                               (21.29)
                                                                         s θs
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:35




                                              a characterization theorem                                 555

                    in other words, every mechanism in D chooses allocations in proportion to the bids.
                    As a result, we conclude that for a given vector θ , when the market clears, mechanisms
                    in D differ from the proportional allocation mechanism only in the market-clearing
                    price—the allocation is the same. Result (iii) of the theorem is then a price of anarchy
                    result that concerns mechanisms of this form.
                       We emphasize that the theorem here is distinguished from related work because
                    the allocation rule (21.29) was not assumed in advance. Rather, the result here starts
                    from a set of simple assumptions on the structure of mechanisms to be considered (the
                    definition of the class D), and uses them to prove that any mechanism in the class must
                    lead to the allocation in (21.29). (See Notes for details.)

                      proof Throughout the proof we fix a particular mechanism D ∈ D. Some
                      computational details are left to the reader.

                          Step 1: A user’s payoff is concave if he is price taking. In other words, we will
                      show that for all U ∈ U and for all p > 0, U (D(p, θ)) − pD(p, θ) is concave in
                      θ. The key idea is to use a limiting regime where capacity grows large, so that
                      users that are price anticipating effectively become price taking.
                          Formally, we first observe that since D must possess a unique market-clearing
                      price regardless of the value of C, D(p, θ) must be strictly monotonic in p (for
                      fixed θ > 0) where it is nonzero, and either (1) D(p, θ) is nondecreasing in p for
                      all θ > 0, or (2) D(p, θ) is nonincreasing in p for all θ > 0.
                          To complete the proof of this step, fix µ > 0, and fix θ > 0. Now consider a
                      limit where R → ∞, and C R = RD(µ, θ) is the capacity in the R’th system.
                      It is straightforward to check that if the R − 1 users 2, . . . , R submit strategy
                      θ, and the first user submits strategy θ  , then the resulting market-clearing price
                      pD converges to µ as R → ∞, regardless of the value of θ  . This step uses the
                      fact that either (1) or (2) above holds. Applying the fact that player 1’s payoff
                      must be concave when he is price anticipating and taking limits as R → ∞, it
                      follows that player 1’s payoff is concave when he is price taking for any fixed
                      price µ > 0.

                         Step 2: There exists a positive function B such that D(p, θ) = θ/B(p) for
                      p > 0 and θ ≥ 0. By Step 1, a player’s payoff is concave when he is price taking.
                      By appropriately choosing a linear utility function with very large slope and very
                      small slope, it follows that D(p, θ) must be concave and convex, respectively, in
                      θ for a given p > 0. Thus for fixed p > 0, D(p, θ) is an affine function of θ.
                      Conditions 3 and 4 in Definition 21.8 then imply that the constant term must be
                      zero, while the coefficient of the linear term is positive; thus, D(p, θ) = θ/B(p)
                      for some positive function B(p).

                         Before continuing, we note that the previous step already implies the remark-
                      able fact that for any mechanism D ∈ D, the allocation at the market-clearing
                      price is made in proportion to the bids θ. This follows from the discussion
                      following (21.28) above.
P1: SBT
9780521872829main       CUNY1061-Nisan          0 521 87282 0      July 5, 2007      14:35




                    556        the design of scalable resource allocation mechanisms

                          Step 3: For all utility systems (C, R, U), there exists a competitive equilibrium,
                       and it is fully efficient. This step follows primarily because of Condition 3 in
                       Definition 21.8: given a price µ, a user can first determine his optimal choice
                       of quantity, and then choose a parameter θ to express this choice. Formally,
                       suppose that µ = pD (θ ), and (21.25) holds. Let dr = D(µ, θr ); then (21.25)
                       implies that the necessary conditions (21.8)–(21.9) hold; these are also sufficient
                       because of Step 1. Furthermore, market clearing implies (21.10) holds. Thus
                       any competitive equilibrium is fully efficient. Existence follows by letting dS
                       be a solution to SYSTEM with Lagrange multiplier µ, and choosing θr = dr /B(µ).

                          Step 4: For all R > 1 and θ −r ∈ (R+ )R−1 , the functions D(pD (θ ), θr ) and
                       −pD (θ )D(pD (θ ), θr ) are concave in θr > 0 if θ −r = 0, and concave in θr ≥ 0
                       if θ −r = 0. As in Step 2, this conclusion follows by considering linear utility
                       functions with very large and very small slope, respectively.

                          Step 5: B is an invertible, differentiable, strictly increasing, and concave
                       function on (0, ∞). We immediately see that B must be invertible on (0, ∞); it
                       is clearly onto, as the right-hand side of (21.28) can take any value in (0, ∞).
                       Furthermore, uniqueness of the market-clearing price in (21.28) requires that B
                       is one-to-one as well, and hence invertible. Since D is differentiable, B must be
                       differentiable as well. Let  denote the differentiable inverse of B on (0, ∞); we
                       will show  is strictly increasing and convex.
                          Let
                                                                                  R
                                                                                      s=1 θs        θr
                                  wr (θ) = pD (θ )D(pD (θ ), θr ) =                               R           C .         (21.30)
                                                                                       C               s=1 θs

                       By Step 4, wr (θ) is convex in θr > 0. By considering strategy vectors θ for which
                       θ −r = 0, it follows that  is convex. Finally, the fact that  is strictly increasing
                       follows by differentiating twice and considering the limit where θr → 0, while
                       keeping θ −r constant and nonzero.4 This establishes the desired facts regarding B.

                           Step 6: Let (C, R, U) be a utility system. A vector θ ≥ 0 is a Nash equilibrium
                       if and only if at least two components of θ are nonzero, and there exists a nonzero
                       vector d ≥ 0 and a scalar µ > 0 such that θr = µdr for all r, Rr=1 dr = C, and
                       the following conditions hold:
                                                                           
                                           dr                  dr               dr
                            Ur (dr ) 1 −        = (µ) 1 −         + µ (µ)       , if dr > 0;       (21.31)
                                           C                   C                C
                                         Ur (0) ≤ (µ), if dr = 0.                                    (21.32)
                                                             
                       In this case dr = D(pD (θ ), θr ), µ = Rr=1 θr /C, and (µ) = pD (θ ). Further,
                       there exists a unique Nash equilibrium. The proof of this step is similar to the

                    4 While the most direct argument uses twice differentiability of , it is possible to make a similar argument even

                      if  is only once differentiable, by arguing only in terms of increments of .
P1: SBT
9780521872829main   CUNY1061-Nisan       0 521 87282 0     July 5, 2007   14:35




                                             a characterization theorem                                   557

                    proof of Nash equilibrium characterization in Theorem 21.2; we omit the details,
                    and refer the reader to the Notes section.

                       Step 7: For any  > 0, there exists a utility systems (C, R, U) such that at any
                    Nash equilibrium θ, the aggregate utility is no more than 3/4 +  of the maximal
                    aggregate utility. Consider a utility system with the following properties. Let
                    C = 1. Fix µ > 0, and let U1 (d1 ) = Ad1 , where A > (µ). We will search for
                    a solution to the Nash conditions (21.31) to (21.32) with market-clearing price
                    (µ).
                       We start by calculating d1 by assuming it is nonzero, and applying (21.31):
                                                            (A − (µ))C
                                                 d1 =                       .                       (21.33)
                                                         A − (µ) + µ (µ)
                    In the spirit of the proof of Theorem 21.4, we will now choose users 2, . . . , R to
                    have identical linear utility functions, with slopes less than A. As we will see, this
                    will be possible if R is large enough.
                       Formally, let d = (C − d1 )/(R − 1), and (cf. (21.31)) define
                                                   (µ)C + (µ (µ) − (µ))d
                                             α=                              .                      (21.34)
                                                            C−d
                    Let Ur (dr ) = αdr for r = 2, . . . , R. Note that if
                                                 C      (A − (µ))C
                                                   ≤                    ,                           (21.35)
                                                 R   A − (µ) + µ (µ)
                    then α ≤ A. This guarantees d1 must be nonzero at any Nash equilibrium, so
                    that the computation in (21.33) is valid. In turn, letting dr = d for r = 2, . . . , R,
                    this implies that (d1 , . . . , dR ) and µ are a valid solution to (21.31)–(21.32), when
                    users have utility functions U1 , . . . , UR .
                       Now consider the limiting ratio of Nash aggregate utility to maximal aggregate
                    utility, as R → ∞. We have d → 0, so α → (µ). Furthermore, regardless of
                    R a solution to SYSTEM is to allocate the entire resource to user 1, so the
                    maximal aggregate utility is AC. Thus the limiting ratio of Nash aggregate utility
                    to maximal aggregate utility becomes
                                                                                            
                               (A − (µ))                          (A − (µ))            (µ)
                                                     + 1−                                        .   (21.36)
                          A − (µ) + µ (µ)                  A − (µ) + µ (µ)           A
                    We now want to find the choices of A and µ which minimize this value.
                       For notational simplicity, we define x = (µ)/A, and (µ) = µ (µ)/(µ).
                    Note that given the convexity and invertibility of , we have (µ) ≥ 1. Then
                    (21.36) is equivalent to

                                                                (1 − x)2
                                              F (x; µ) =                    + x.                    (21.37)
                                                            1 + ( (µ) − 1)x
                    It is straightforward to establish that the preceding expression is strictly convex
                    in x for fixed µ. Let G( (µ)) denote the minimal value of F (x; µ) for x ∈ (0, 1);
P1: SBT
9780521872829main      CUNY1061-Nisan        0 521 87282 0        July 5, 2007       14:35




                    558      the design of scalable resource allocation mechanisms

                                                1




                                             0.75




                                     G (Ψ)    0.5




                                             0.25




                                                    0        10          20           30     40   50
                                                                                 Ψ

                    Figure 21.1. The function G() defined in (21.38). Note that G() is strictly decreasing, with
                    G(1) = 3/4.


                      by differentiating, it follows that G( ) is defined for ≥ 1 according to
                                                  ⎧
                                                  ⎪
                                                  ⎪
                                                    3
                                                                             if = 1;
                                                  ⎪
                                                  ⎪   ,
                                                  ⎨4
                                       G( ) =                 √       √                        (21.38)
                                                  ⎪
                                                  ⎪ 2 2−3           +
                                                  ⎪
                                                  ⎪               √
                                                  ⎩                        , if > 1.
                                                         ( − 1)2
                          The function G is plotted in Figure 21.1. It is straightforward to verify that
                      G( ) is continuous and strictly decreasing for        ≥ 1 so that the worst-case
                      example is given by finding µ > 0 such that (µ) is maximized. Furthermore, it
                      is straightforward to check that G( ) ≤ 3/4, establishing the required claim.

                         Step 8: For any mechanism other than the proportional allocation mechanism,
                      the worst-case efficiency is strictly lower than 3/4. For the proportional alloca-
                      tion mechanism, we have (µ) = 1, and we have already established that the
                      efficiency ρ is exactly 3/4. On the other hand, it is straightforward to check that if
                      B(p) is nonlinear, then the maximal value of (µ) in the preceding step is strictly
                      greater than 1; and in this case G( (µ)) is strictly less than 3/4. Thus there exists
                      a game with efficiency ratio strictly lower than 3/4 for such a mechanism. This
                      completes the proof.

                       We make several comments regarding the proof. First, notice that every mechanism
                    in the described class allocates in proportion to the bids of the players; in this sense all
                    mechanisms in D are “proportional allocation mechanisms.” However, the efficiency
                    loss is minimized exactly when this mechanism charges each user exactly their bid.
                    Second, it is possible to show that the bound constructed in Steps 7–8 of the proof is
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:35




                                         the vickrey–clarke–groves approach                                559

                    in fact a tight bound on the price of anarchy of the mechanisms under consideration;
                    it is possible to reformulate this bound so that it depends only on the elasticity of
                    the function B(p), i.e., the quantity inf p>0 pB  (p)/B(p). (This is not surprising, since
                       (µ) is the elasticity of the function , which is the inverse of B.) It is surprising
                    that the price of anarchy of a general class of such mechanisms can be reduced to this
                    parsimonious calculation.
                        Finally, we note one potentially undesirable feature of the family of market-clearing
                    mechanisms considered: the payoff to user r is defined as −∞ when the composite
                    strategy vector is θ = 0 (cf. (21.27)). This definition is required because when the
                    composite strategy vector is θ = 0, a market-clearing price may not exist. One possible
                    remedy is to restrict attention instead to mechanisms where D(p, θ) = 0 if θ = 0, for
                    all p ≥ 0; in this case we can define pD (θ ) = 0 if θ = 0, and let the payoff to user r be
                    Ur (0) if θr = 0. This condition amounts to a “normalization” on the market-clearing
                    mechanism. It is possible to show that this modification does not alter the conclusion
                    of Theorem 21.10.


                                    21.4 The Vickrey–Clarke–Groves Approach

                    The mechanisms we considered in the last section had several restrictions placed on
                    them; chief among these are that (1) users are restricted to using “simple” strategy
                    spaces and (2) the mechanism uses only a single price to clear the market. On the other
                    hand, one could consider both generalizations where users are allowed to use more
                    complex strategies, perhaps declaring their entire utility function to the market; and
                    also, where price discrimination is allowed so that each user is charged a personalized
                    per-unit price for the resource.
                       The best known solution employing both these generalizations is the VCG approach
                    to eliciting utility information (see Notes, and Chapter 9). Such mechanisms allow
                    users to declare their entire utility functions, and then charge users individualized
                    prices so that they have the incentive to truthfully declare their utilities. We review
                    VCG mechanisms in Section 21.4.1.
                       In this section we are interested in deciding whether the same outcome can be
                    realized preserving restriction (1) above, but removing restriction (2): that is, can
                    mechanisms with “simple” strategy spaces that employ price discrimination achieve
                    full efficiency? In Section 21.4.2 we present an alternate class of mechanisms, inspired
                    by the VCG class, in which users only submit scalar strategies to the mechanism; we
                    call such mechanisms scalar strategy VCG (SSVCG) mechanisms. We show that these
                    mechanisms have desirable efficiency properties. In particular, we establish existence
                    of an efficient Nash equilibrium, and under an additional condition, we also establish
                    that all Nash equilibria are efficient.


                                                  21.4.1 VCG Mechanisms
                    In the VCG class of mechanisms, the basic approach is to let the strategy space of
                    each user r be the set U of possible utility functions, as defined in Assumption 1, and
                    structure the payments made by each user so that the payoff of each user r has the same
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0      July 5, 2007         14:35




                    560      the design of scalable resource allocation mechanisms

                    form as the objective function in SYSTEM, (21.1). As VCG mechanisms have been
                    introduced in Chapter 9, we only use this section to fix notation for our subsequent
                    discussion. For each r, we use Ũr to denote the declared utility function of user r, and
                    use Ũ = (Ũ1 , . . . , ŨR ) to denote the vector of declared utilities.
                       Suppose that user r receives an allocation dr , but has to make a payment tr ; we use
                    the notation tr to distinguish from the bid wr of Section 21.2. Then the payoff to user
                    r is

                                                              Ur (dr ) − tr .

                    On the other hand, the social objective (21.1) can be written as
                                                                       
                                                          Ur (dr ) +          Us (ds ).
                                                                       s=r


                    Given a vector of declared utility functions Ũ, a VCG mechanism chooses the allocation
                    d(Ũ) as an optimal solutionto SYSTEM for the declared utility functions Ũ. For
                    simplicity, let X = {d ≥ 0 : r dr ≤ C}; this is the feasible region for SYSTEM. Then
                    for a VCG mechanism, we have
                                                                              
                                                   d(Ũ) ∈ arg max                Ũr (dr ).             (21.39)
                                                                      d∈X
                                                                              r


                    The payments are structured so that
                                                             
                                              tr (Ũ) = −           Ũs (ds (Ũ)) + hr (Ũ−r ).          (21.40)
                                                             s=r


                    Here hr is an arbitrary function of the declared utilities of users other than r. In general,
                    we note that mechanisms of this form do not use a single price to clear the market; i.e.,
                    the per-unit price paid by user r, tr (Ũ)/dr (Ũ), will not be the same for all users. (See
                    also Exercise 21.3.)
                        For our purposes, the interesting feature of the VCG mechanism is that there exists a
                    dominant strategy equilibrium that elicits the true utility functions from the users, and
                    in turn (because of the definition of d(Ũ)) chooses an efficient allocation. (See Chapter
                    9 for a formal statement of these results, where it is shown that the VCG mechanism is
                    incentive compatible.) In the next section, we explore a class of mechanisms inspired
                    by the VCG mechanisms, but with limited communication requirements.


                                         21.4.2 Scalar Strategy VCG Mechanisms
                    We now consider a class of mechanisms where each user’s strategy is a submitted
                    utility function (as in the VCG mechanisms) except that users are allowed only to
                    choose from a given single parameter family of utility functions. One cannot expect
                    such mechanisms to have efficient dominant strategy equilibria, and we will focus
                    instead on the efficiency properties of the resulting Nash equilibria.
P1: SBT
9780521872829main       CUNY1061-Nisan          0 521 87282 0       July 5, 2007       14:35




                                              the vickrey–clarke–groves approach                                                  561

                       Formally, scalar strategy VCG (SSVCG) mechanisms allow users to choose from a
                    given family of utility functions U (·; θ), parameterized by θ ∈ (0, ∞).5 We make the
                    following assumptions about this family.

                    Assumption 2:

                       (i) For every θ > 0, the function U (·; θ ) : d → U (d; θ ) belongs to U (i.e., it is concave,
                           strictly increasing, continuous, and differentiable), and is also strictly concave.
                                                                                                 
                      (ii) For every γ ∈ (0, ∞) and d ≥ 0, there exists a θ > 0 such that U (d; θ ) = γ .6

                       Given θ , the mechanism chooses d(θ ) such that
                                                               
                                               d(θ ) = arg max     U (dr ; θr ).                                             (21.41)
                                                                         d∈X
                                                                                r

                    Since U (·; θr ) is strictly concave for each r, the solution d(θ) is uniquely defined. (Note
                    the similarity between (21.39) and (21.41).)
                       By analogy with the expression (21.40), the monetary payment by user r is
                                                            
                                                 tr (θ) = −    U (ds (θ ); θs ) + hr (θ −r ).             (21.42)
                                                                  s=r

                    Here hr is a function that depends only on the strategies θ −r = (θs , s = r) submitted by
                    the users other than r. While we     do not advocate any particular choice of hr , a natural
                    candidate is to define hr (θ −r ) = s=r U (ds (θ −r ); θs ), where vd(θ −r ) is the aggregate
                    utility maximizing allocation excluding user r. This leads to a natural scalar strategy
                    analogue of the Clarke pivot mechanism (cf. Chapter 9).
                       Given hr , the payoff to user r is
                                                                         
                                   Pr (dr (θ ), tr (θ)) = Ur (dr (θ )) +   U (ds (θ ); θs ) − hr (θ −r ).
                                                                                s=r

                       A strategy vector θ is a Nash equilibrium if no user can profitably deviate through
                    a unilateral deviation, i.e., if for all users r there holds:

                                    Pr (dr (θ), tr (θ )) ≥ Pr (dr (θr , θ −r ), tr (θr , θ −r )), for all θr > 0.         (21.43)

                       We start with the following key lemma, proven using an argument analogous to the
                    proof that truthtelling is a dominant strategy equilibrium of the VCG mechanism (see
                    Chapter 9).

                    5 Note that, by contrast with Section 21.3, the choice of bid θ by a user indexes a utility function, rather than

                      a demand function. However, this is not particularly crucial: if a user with utility function U maximizes
                      U (d) − pd (i.e., the user acts as a price taker), the solution yields the demand function D(p) = (U  )−1 (p).
                      Up to additive constant, the utility function and demand function can be recovered from each other. Thus,
                      equivalently, we could define SSVCG mechanisms where users submit demand functions from a parameterized
                      class. We define our SSVCG mechanisms according to Assumption 2 to maintain consistency with the definition
                      of VCG mechanisms in Section 21.4.1, as well as in Chapter 9.
                    6 Since we do not assume differentiability with respect to θ , the only differentiation of U is with respect to the
                                                
                      first coordinate d, and U (d; θ ) will always stand for the derivative with respect to d.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0      July 5, 2007           14:35




                    562      the design of scalable resource allocation mechanisms

                      Lemma 21.11 Then the vector θ is a Nash equilibrium of the SSVCG mechanism
                      if and only if for all r:
                                                          ⎡                         ⎤
                                                                      
                                          d(θ ) ∈ arg max ⎣Ur (dr ) +   U (ds ; θs )⎦ .     (21.44)
                                                        d∈X
                                                                                 s=r


                      proof Fix a user r. Since θr does not affect hr , from (21.43) user r will choose
                      θr to maximize the following effective payoff:
                                                             
                                              Ur (dr (θ )) +   U (ds (θ ); θs ).                   (21.45)
                                                                       s=r

                      The optimal value of the objective function in (21.44) is certainly an upper bound
                      to user r’s effective payoff (21.45). Thus, given a vector θ, if (21.44) is satisfied
                      for all users r, then (21.43) holds for all users r, and we conclude θ is a Nash
                      equilibrium.
                         Conversely, given a vector θ , suppose that (21.44) is not satisfied for some user
                      r. We will show θ cannot be a Nash equilibrium. Since X is compact, an optimal
                      solution exists to the problem in (21.44) for user r; call this optimal solution d∗ .
                      The vector d∗ must satisfy the first-order optimality conditions (21.8)–(21.10),
                                                                                       
                      which only involve the first derivatives Ur (dr∗ ) and (U (ds∗ ; θs ), s = r). Suppose
                                                                             
                      now that user r chooses θr > 0 such that U (dr∗ ; θr ) = Ur (dr∗ ). Then, d∗ also
                      satisfies the optimality conditions for the problem (21.41). Since d(θr , θ −r ) is the
                      unique optimal solution to (21.41) when the strategy vector is (θr , θ −r ), we must
                      have d(θr , θ −r ) = d∗ . Thus we have
                                                                
                            Pr (dr (θ ), tr (θ )) < Ur (dr∗ ) +   U (ds∗ ; θs ) + hr (θ −r )
                                                                s=r
                                                                              
                                              = Ur (dr (θr , θ −r )) +              U (ds (θr , θ −r ); θs ) + hr (θ −r )
                                                                              s=r

                                              = Pr (dr (θr , θ −r ), tr (θr , θ −r )).

                      (The first inequality follows by the assumption that (21.44) is not satisfied for
                      user r.) We conclude that (21.43) is violated for user r, so θ is not a Nash
                      equilibrium.

                        The following corollary states that there exists a Nash equilibrium which is efficient.
                    Furthermore, at this efficient Nash equilibrium, all users truthfully reveal their utilities
                                                                                                     
                    in a local sense: each user r chooses θr so that the declared marginal utility U (dr (θ); θr )
                    is equal to the true marginal utility Ur (dr (θ )).

                      Corollary 21.12 For any SSVCG mechanism, there exists an efficient Nash
                      equilibrium θ defined as follows: Let dS be an optimal solution to SYSTEM. Each
                                                   
                      user r chooses θr so that U (drS ; θr ) = Ur (drS ). The resulting allocation satisfies
                      d(θ ) = dS .
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 5, 2007   14:35




                                         the vickrey–clarke–groves approach                                   563

                                                                                                    
                      proof By Assumption 2, each user r can choose θr so that U (drS ; θr ) =
                      Ur (drS ). For this vector θ , it is clear that d(θ) = dS , since the optimal solution to
                      (21.41) is uniquely determined, and the optimality conditions for (21.41) involve
                                                       
                      only the first derivatives U (dr (θ ); θr ). By the same argument it also follows that
                      dS is an optimal solution in (21.44). Since d(θ ) = dS , we conclude that (21.44)
                      is satisfied for all r, and thus θ is a Nash equilibrium.

                       We note that, as in classical VCG mechanisms, there can be additional, possibly
                    inefficient, Nash equilibria, as the following example shows.

                      Example 21.13 Consider a system with R identical users with strictly concave
                                                                                       
                      utility function U . Suppose that user 1 chooses θ1 so that U (C; θ1 ) > U  (0), and
                                                                              
                      every other user r chooses θr so that U (0; θr ) < U (C). Since U  (C) ≤ U  (0),
                      it follows that (21.44) is satisfied for all users r. Thus this is a Nash equilibrium
                      where the entire resource is allocated to user 1; however, the unique optimal
                      solution to SYSTEM is symmetric, and allocates C/R units of the resource to each
                      of the R users.

                       The equilibrium in the preceding example involves a “bluff”: user 1 declares such a
                    high marginal utility at C that all other users concede. One way to preclude such equi-
                    libria is to enforce an assumption that guarantees participation. The next proposition
                    assumes that all users have infinite marginal utility at zero allocation; this guarantees
                    that all Nash equilibria are efficient.

                      Proposition 21.14 Suppose that Ur (0) = ∞ for all r. Suppose that θ is a Nash
                      equilibrium. Then d(θ ) is an optimal solution to SYSTEM.

                      proof Let d = d(θ ). The proof follows by noting that all users must have
                      positive allocations at equilibrium if Ur (0) = ∞, from (21.44). Thus at equilib-
                                                                          
                      rium, for all users r, s we have Ur (dr ) = U (ds ; θs ). But this in turn implies that
                      Ur (dr ) = Us (ds ) for all r, s, a sufficient condition for optimality for the problem
                      SYSTEM.

                       Intuitively, for efficiency to hold, we need to have a number of actively “competing”
                    users. In the previous result, this is guaranteed because every user will want strictly
                    positive rate at any equilibrium.
                       The results of this section demonstrate that by relaxing the assumption that the
                    resource allocation mechanism must set a single price, we can in fact significantly
                    improve upon the efficiency guarantee of Theorem 21.10. It is critical to note that this
                    gain in efficiency occurs only at Nash equilibria. The classical VCG mechanisms are
                    unique in that they guarantee efficient outcomes as dominant strategy equilibria; it is
                    straightforward to check that the SSVCG mechanisms described in this section will
                    not have dominant strategy equilibria in general—e.g., the “bluff” example above is
                    one such case.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:35




                    564      the design of scalable resource allocation mechanisms

                                  21.5 Chapter Summary and Further Directions

                    This chapter considered the allocation of a single resource of fixed supply among
                    multiple strategic users. We evaluated a variety of market mechanisms through
                    Nash equilibria of the resulting resource allocation game. Our key insights are the
                    following:


                      (i) A simple proportional allocation mechanism, where each user receives a share of
                          the resource in proportion to their bid, ensures full efficiency when users are price
                          takers, and exhibits no worse than a 25% efficiency loss when users are price
                          anticipators.
                     (ii) In a natural class of mechanisms where users choose one-dimensional strategies, and
                          the market sets a single price, the proportional allocation mechanism minimizes the
                          worst-case efficiency loss when users are price anticipating; i.e., the best possible
                          guarantee here is 75% of maximal aggregate utility.
                    (iii) This guarantee can be improved if the mechanism is allowed to set one price per
                          user. Using an adapted version of the VCG class of mechanisms, we can construct
                          mechanisms that ensure fully efficient Nash equilibria.


                       Our investigation also reveals several further directions open for future research,
                    including the following:


                      (i) For the proportional allocation mechanism, we have proven a bound on the price of
                          anarchy that shows that the ratio of the Nash equilibrium aggregate utility is no worse
                          than 3/4 the maximum possible aggregate utility. For nonatomic selfish routing (cf.
                          Chapter 18), a similar price of anarchy result holds: the ratio of Nash cost to the
                          optimal cost is no worse than 4/3; furthermore, both proofs use the characterization of
                          Nash equilibria as solutions to an optimization problem, with structure similar to the
                          respective efficient optimization problems. These results are suggestive of perhaps a
                          deeper generalization of price of anarchy for games with equilibria characterized as
                          the solution to optimization problems.
                     (ii) While Theorem 21.10 proves optimality of the proportional allocation mechanism in
                          a reasonable class of mechanisms, the result depends critically on the assumption that
                          all mechanisms in D yield concave payoffs when agents are price anticipating. Given
                          that some type of quasiconcavity assumption is typically necessary on payoffs to
                          even guarantee existence of Nash equilibria, one might informally expect the result of
                          Theorem 21.10 to hold even if Condition 2 is removed in the definition of D. Whether
                          this is in fact possible remains an open question.
                    (iii) Our investigation shows, under reasonable assumptions, that with a single market-
                          clearing price a 75% efficiency guarantee is possible, while with one price per user
                          (the scalar strategy VCG approach), full efficiency is possible. This warrants further
                          investigation: what is the exact trade-off between the number of prices and the effi-
                          ciency guarantee possible? Furthermore, how does increasing the dimensionality of
                          users’ strategy affect this efficiency guarantee?
P1: SBT
9780521872829main       CUNY1061-Nisan          0 521 87282 0       July 5, 2007    14:35




                                                                        notes                                                 565

                                                                    21.6 Notes

                                                               21.6.1 Section 20.2
                    Much of the material in this section is based on Chapter 2 of Johari (2004) and the
                    corresponding paper (Johari and Tsitsiklis, 2004).
                       The mechanism discussed here was first studied in the context of communication
                    networks by Kelly (1997). (See Chapter 22 for a discussion of the proportional al-
                    location mechanism in congestion control algorithms for communication networks.)
                    Theorem 21.1 is adapted from Kelly (1997), where it is proven in greater generality
                    for an extension of the proportional allocation mechanism to a network context. This
                    theorem is an extension of the classical first fundamental theorem of welfare economics;
                    see Mas-Colell et al. (1995, Chapter 16), for details.
                       The first proof of uniqueness of Nash equilibrium for the proportional allocation
                    mechanism was provided by La and Anantharam (2000). The most general result of
                    existence and uniqueness, and the basis for the result in Theorem 21.2, is due to Hajek
                    and Gopalakrishnan (2002); a less general result was proven by Maheswaran and Basar
                    (2003). The explicit formulation of the problem GAME is given by Johari and Tsitsiklis
                    (2004).
                       The price of anarchy result of Theorem 21.4 is due to Johari and Tsitsiklis (2004).
                    The original proof of this result uses a two-step approach: it is first shown that the worst
                    case is achieved using linear utility functions, and then the efficiency loss calculation
                    is solved directly as a mathematical programming problem. The proof based on the
                    problem GAME presented here is due to Roughgarden (2006), who also successfully
                    applies the same method to efficiency loss calculations in several other games.

                                                               21.6.2 Section 20.3
                    Much of the material in this section is based on Chapter 5 of Johari (2004) and Section
                    4 of Johari and Tsitsiklis (2007).
                       The most closely related result to this section is presented by Maheswaran and Basar
                    (2004). In their result, they consider mechanisms where each user r chooses a bid wr ,
                    and the allocation is still made proportional to each player’s bid. However, rather
                    than assuming that every player pays wr as in the standard proportional allocation
                    mechanism, Maheswaran and Basar consider a class of mechanisms where the user
                    pays c(wr ), where c is a convex function. They show that in this class of mechanisms,
                    the proportional allocation mechanism (i.e., a linear c) achieves the minimal worst-case
                    efficiency loss when users are price anticipating.
                       Our work is substantially different, because we do not postulate that the mechanism
                    must use the proportional rule (21.29) in allocating the resource; rather, this emerges
                    as a consequence of rather simple assumptions on our mechanisms. We note that other
                    works on inefficiency of resource allocation mechanisms, including Maheswaran and
                    Basar (2004) and Yang and Hajek (2004), also assume a priori that allocations are made
                    in proportion to users’ bids.7 In this sense, our result lends a rigorous foundation to the

                    7 A notable exception is Sanghavi and Hajek (2004), which assumes that users pay their bid, and then designs an

                      allocation rule to minimize worst case efficiency loss.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0      July 5, 2007   14:35




                    566       the design of scalable resource allocation mechanisms

                    intuition that the proportional allocation rule (21.29) is a natural choice to determine
                    the allocation among users.


                                                            21.6.3 Section 20.4
                    This section is based on Section 5.2 of the paper by Johari and Tsitsiklis (2007).
                    Simultaneously and independently, a nearly identical formulation was developed by
                    Yang and Hajek (2007). It is worth noting that Yang and Hajek and Maheswaran and
                    Basar had earlier presented a resource allocation mechanism where users receive an
                    allocation in proportion to their bids, but prices are chosen on an individualized basis
                    (Maheswaran and Basar, 2004; Yang and Hajek, 2004); this mechanism can be seen to
                    be a special case of the SSVCG mechanisms (Johari and Tsitsiklis, 2007).
                       Subsequent to the above work, several papers have presented related constructions of
                    mechanisms that use limited communication yet achieve fully efficient Nash equilibria.
                    Building on earlier work by Semret (1999), Dimakis et al. establish that a VCG-like
                    mechanism where agents submit a pair (price and quantity requested) can achieve fully
                    efficient equilibrium for a related resource allocation game (Dimakis et al., 2006).
                    Stoenescu and Ledyard consider the problem of resource allocation by building on the
                    notion of minimal message spaces addressed in earlier literature on mechanism design,
                    and build a class of efficient mechanisms with scalar strategy spaces (Stoenescu and
                    Ledyard, 2006).
                       The latter work of Stoenescu and Ledyard recalls perhaps the most related reference
                    (and most seminal) in this area by Reiter and Reichelstein (1988). Their paper calcu-
                    lates the minimal dimension of strategy space that would be necessary to achieve fully
                    efficient Nash equilibria for a general class of economic models known as exchange
                    economies. For our model, their bound evaluates to a strategy space per user of dimen-
                    sion 1 + 2/(R(R − 1)), where R denotes the number of users. This is slightly higher
                    than our result because Reiter and Reichelstein consider a much more general resource
                    allocation problem.


                                                              Bibliography
                    A. Dimakis, R. Jain, and J. Walrand. Mechanisms for efficient allocation in divisible capacity net-
                       works. Proceedings of IEEE CDC, pp. 1264–1269, 2006.
                    B. Hajek and G. Gopalakrishnan. Do greedy autonomous systems make for a sensible Internet?
                       Presented at the Conf. Stochastic Networks, Stanford University, 2002.
                    R. Johari. Efficiency Loss in Market Mechanisms for Resource Allocation. PhD thesis, Massachusetts
                       Institute of Technology, 2004.
                    R. Johari and J.N. Tsitsiklis. Efficiency loss in a network resource allocation game. Math. Operat.
                       Res., 29(3):407–435, 2004.
                    R. Johari and J.N. Tsitsiklis. Efficiency of scalar-parameterized mechanisms. Management Science
                       and Engineering Working Paper 07-04-6126-34, Stanford University, 2007.
                    F.P. Kelly. Charging and rate control for elastic traffic. Euro. Trans. Telecommun., 8:33–37,
                       1997.
                    R.J. La and V. Anantharam. Charge-sensitive TCP and rate control in the Internet. In Proc. IEEE
                       INFOCOM, pp. 1166–1175, 2000.
P1: SBT
9780521872829main       CUNY1061-Nisan       0 521 87282 0    July 5, 2007     14:35




                                                               exercises                                           567

                    R.T. Maheswaran and T. Basar. Nash equilibrium and decentralized negotiation in auctioning divisible
                       resources. Group Decis. Negotiation, 12(5):361–395, 2003.
                    R.T. Maheswaran and T. Basar. Social welfare of selfish agents: motivating efficiency for divisible
                       resources. In Proc. IEEE CDC, pp. 1550–1555, 2004.
                    A. Mas-Colell, M.D. Whinston, and J.R. Green. Microeconomic Theory. Oxford University Press,
                       Oxford, UK, 1995.
                    S. Reichelstein and S. Reiter. Game forms with minimal message spaces. Econometrica, 56(3):661–
                       692, 1988.
                    T. Roughgarden. Potential functions and the inefficiency of equilibria. In Proc. Intl. Congress of
                       Mathematicians, Vol. III, pp. 1071–1094, 2006.
                    S. Sanghavi and B. Hajek. Optimal allocation of a divisible good to strategic buyers. In Proc. IEEE
                       CDC, pp. 2748–2753, 2004.
                    N. Semret. Market Mechanisms for Network Resource Sharing. PhD thesis, Columbia University,
                       1999.
                    T.M. Stoenescu and J. Ledyard. A pricing mechanism which implements a network rate allocation
                       problem in Nash equilibria. 2006. Submitted.
                    S. Yang and B. Hajek. An efficient mechanism for allocation of a divisible good and its application
                       to network resource allocation. 2004. Preprint.
                    S. Yang and B. Hajek. VCG-Kelly mechanisms for divisible goods: adapting VCG mechanisms to
                       one-dimensional signals. To appear in IEEE Journal on Selected Areas in Communications, 2007.



                                                               Exercises
                    21.1 This exercise, together with the next one, studies the efficiency loss properties of
                         the mechanisms defined in Example 21.9, by following the proof of Theorem 21.4.
                         Suppose that D( p, θ ) = θ p−1/c, where c ≥ 1. Suppose that given a utility system
                         (C, R, U), a bid vector θ is a Nash equilibrium, and let the resulting allocation
                         vector be d; i.e., dr = D( pD (θ ), θr ).

                           (a) Verify the Nash equilibrium conditions (21.31)–(21.32).
                           (b) Show that d is the unique solution to GAME, but where Ûr is defined as follows
                               for each r :
                                                                dr                   
                                                                         1 − z/C
                                                  Ûr (dr ) =                           Ur (z) dz.      (E1.1)
                                                              0      1 + (c − 1)(z/C)
                               (Hint: rearrange the Nash equilibrium conditions (21.31)—(21.32).)
                           (c) Show that Ûr satisfies Assumption 1.

                    21.2 Fix D( p, θ ) = θ p−1/c and define Û as in the previous exercise. Define β(D) accord-
                         ing to (21.24), i.e.,

                                                                             U(d) + Û  (d)(d − d)
                                                β(D) = inf inf       inf                              .
                                                         U∈U C>0 0≤d,d ≤C              U(d )


                           (a) Show that ρ(D) ≥ β(D). (Hint: first construct the variational inequality that
                               identifies the optimality conditions for GAME, then argue as in the proof of
                               Theorem 21.4.)
                           (b) Show that β(D) ≥ G(c).
P1: SBT
9780521872829main     CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:35




                    568     the design of scalable resource allocation mechanisms

                          (c) Using a construction analogous to the proof of Theorem 21.4, show that for
                              any δ there exists a utility system for which the ratio of Nash aggregate utility
                              to the maximum aggregate utility is no more than G(c) + δ. Conclude that
                              ρ(D) = G(c).
                    21.3 Show by example that a VCG mechanism does not necessarily charge each user
                         the same per-unit price for the resource.
P1: SBT
9780521872829main   CUNY1061-Nisan   0 521 87282 0   July 5, 2007   14:36




                                                     PART FOUR

                                 Additional Topics
P1: SBT
9780521872829main   CUNY1061-Nisan   0 521 87282 0   July 5, 2007   14:36
