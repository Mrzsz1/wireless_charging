---
type: "book-chapter"
book_id: "algorithmic-game-theory"
chapter_id: "ch-10"
chapter_number: 10
chapter_title: "Chapter 10"
source_pdf: "raw/inbox/manual-drop/PDF_B.pdf"
source_page_start: 264
source_page_end: 287
printed_page_start: 264
printed_page_end: 287
part_ids: ["algorithmic-game-theory-ch-10-part-011"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Chapter 10

P1: SBT
9780521872829main       CUNY1061-Nisan           0 521 87282 0       July 5, 2007      14:18




                                                                  CHAPTER 10


                                    Mechanism Design without
                                            Money

                                          James Schummer and Rakesh V. Vohra




                                                                      Abstract

                    Despite impossibility results on general domains, there are some classes of situations in which there
                    exist interesting dominant-strategy mechanisms. While some of these situations (and the resulting
                    mechanisms) involve the transfer of money, we examine some that do not. Specifically, we analyze
                    problems where agents have single-peaked preferences over a one-dimensional “public” policy space;
                    and problems where agents must match with each other.



                                                               10.1 Introduction

                    The Gibbard–Satterthwaite Theorem (Theorem 9.8) is a Procrustean bed1 that is es-
                    caped only by relaxing its assumptions. In conjunction with the Revelation Principle
                    (Proposition 9.25), it states that on the general domain of preferences, only dictatorial
                    rules can be implemented in dominant strategies (if the range contains at least three
                    alternatives). In this chapter we escape Procrustes by examining dominant strategy
                    implementation on restricted domains of preferences.2
                       In most applications it is clearly unreasonable to assume that agents’ preferences
                    are completely unrestricted, as was assumed in the voting context of Section 9.2.4.
                    For instance, in situations involving the allocation of goods, including money, one can
                    safely assume that each agent prefers to receive more money (or other goods). As can
                    be seen in the following chapters, the ability for agents to make monetary transfers
                    allows for a rich class of strategy-proof rules.
                       Nevertheless there are many important environments where money cannot be used as
                    a medium of compensation. This constraint can arise from ethical and/or institutional

                    1 Procrustes was a giant that lived by one of the roads that led to Attica. He boasted of a bed whose length exactly

                      matched the size of its occupant. What he neglected to mention was that this remarkable feature was obtained
                      by either stretching or butchering his guest to fit the bed.
                    2 Other avenues of escape not discussed here include randomization, making preferences common knowledge,

                      and using weaker notions of implementation.

                                                                           243
P1: SBT
9780521872829main      CUNY1061-Nisan          0 521 87282 0        July 5, 2007   14:18




                    244                          mechanism design without money

                    considerations: many political decisions must be made without monetary transfers;
                    organ donations can be arranged by “trade” involving multiple needy patients and
                    their relatives, yet monetary compensation is illegal. In this chapter we focus on a few
                    examples of just this kind.
                       Before proceeding with the examples, we formalize the idea that dominant-
                    strategy implementation is a weaker concept on restricted domains of preferences.
                    In general, a decision problem can be described by these parameters: a set of
                    agents N = {1, 2, . . . , n}, a set of alternatives A, and for each agent i ∈ N a set of po-
                    tential preference relations Ri over the alternatives in A.3 The Gibbard–Satterthwaite
                    Theorem (Theorem 9.8) applies, for example, when each Ri is the entire set of linear
                    orders on A.
                       An allocation rule is a function f : × Ri → A, mapping preferences of the agents
                    into alternatives. It is strategy-proof if its use makes it a weakly dominant strategy
                    for agents to truthfully report their preferences. (See Section 9.4). We observe the
                    following principle.
                       Consider two decision problems (N, A, R1 , . . . , Rn ) and (N, A, R1 , . . . , Rn ),
                    where Ri ⊆ Ri for each i ∈ N. Suppose f : × Ri → A is a strategy-proof rule for
                    the former problem. Then the restriction of the function f to (×Ri ), namely f |×Ri ,
                    defines a strategy-proof rule for the latter problem.
                       The proof of this is straightforward: on a smaller domain of preferences, strategy-
                    proofness is easier to satisfy because it imposes strictly fewer constraints. This simple
                    observation justifies the search for reasonable (or at least nondictatorial) rules for
                    decision problems involving “smaller” domains of preferences than those that yield the
                    Gibbard–Satterthwaite Theorem.
                       In Section 10.2 we analyze a problem involving a natural domain restriction when
                    agents vote over one-dimensional policies. It is one of the canonical “public good”
                    settings (Ri = Rj for all i, j ∈ N) in which interesting, strategy-proof rules can
                    be obtained. The analysis here is illustrative of the approach used to characterize
                    such rules in other environments. In Sections 10.3 and 10.4 we analyze matching
                    problems. As opposed to the previous setting, these problems have the feature that
                    each agent cares only about his own private consumption; that is, each Ri con-
                    tains only preference relations that are sensitive only to certain dimensions of the
                    alternative space A; hence Ri = Rj whenever i = j . These are examples of what
                    are called “private good” problems. Two kinds of matching problems are analyzed,
                    demonstrating the limits of what can be implemented in dominant strategies in such
                    environments.



                                        10.2 Single-Peaked Preferences over Policies

                    A simple but elegant class of domains involves single-peaked preferences over one-
                    dimensional policy spaces. This domain can be used to model political policies, eco-
                    nomic decisions, location problems, or any allocation problem where a single point

                    3 A preference relation is a weak order on A.
P1: SBT
9780521872829main       CUNY1061-Nisan           0 521 87282 0       July 5, 2007      14:18




                                            single-peaked preferences over policies                                                245

                    must be chosen in an interval. The key assumption we make is that agents’ preferences
                    are assumed to have a single most-preferred point in the interval, and that preferences
                    are “decreasing” as one moves away from that peak.
                       Formally, the allocation space (or policy space) is the unit interval A = [0, 1]. An
                    outcome in this model is a single point x ∈ A. Each agent i ∈ N has a preference
                    ordering i (i.e., a weak order) over the outcomes in [0, 1]. The preference relation
                    i is single-peaked if there exists a point pi ∈ A (the peak of i ) such that for all
                    x ∈ A \ {pi } and all λ ∈ [0, 1), (λx + (1 − λ)pi ) i x.4 Let R denote the class of
                    single-peaked preferences.
                       We denote the peaks of preference relations i , i , j , etc., respectively by pi , pi ,
                    pj , etc. Denote a profile (n-tuple) of preferences as  ∈ Rn .
                       One can imagine this model as representing a political decision such as an income
                    tax rate, another political issue with conservative/liberal extremes, the location of a
                    public facility on a road, or even something as simple as a group of people deciding
                    on the temperature setting for a shared office. In these and many other examples, the
                    agents have an ideal preferred policy in mind, and would prefer that a decision be made
                    as close as possible to this “peak.”
                       A rule f : Rn → A assigns an outcome f () to any preference profile . As before,
                    a rule is strategy-proof if it is a dominant strategy for each agent to report his preferences
                    truthfully when the rule is being used to choose a point.
                       In contrast to the impossibility result of Gibbard (1973) and Satterthwaite (1975),
                    that obtain on the universal domain of preferences, we shall see that this class of
                    problems admits a rich family of strategy-proof rules whose ranges include more than
                    two alternatives. In fact, the family of such rules remains rich even when one restricts
                    attention (as we do in this chapter) to rules that satisfy the following condition.
                       We say that a rule f is onto if for all x ∈ A there exists  ∈ Rn such that f () = x.
                    An onto rule cannot preclude an outcome from being chosen ex ante. It is not without
                    loss of generality to impose this condition. For instance, fix two points x, y ∈ [0, 1] and
                    consider a rule that chooses whichever of the two points is preferred to the other by a
                    majority of agents (and where x is chosen in case of a tie). Such a rule is strategy-proof,
                    but not onto. Similar strategy-proof rules can even break ties between x and y by using
                    preference information about other points x  , y  , . . ., in [0, 1], even though x  , etc., are
                    not in the range of the rule.
                       The onto condition is even weaker than what is called unanimity, which requires
                    that whenever all agents’ preferences have the same peak (pi = pj for all i, j ), the rule
                    must choose that location as the outcome. In turn, unanimity is weaker than Pareto-
                    optimality: for all  ∈ Rn , there exists no point x ∈ [0, 1] such that x i f () for all
                    i ∈ N.
                       As it turns out, these three requirements are all equivalent among strategy-proof
                    rules.

                       Lemma 10.1 Suppose f is strategy-proof. Then f is onto if and only if it is
                       unanimous if and only if it is Pareto-optimal.

                    4 The binary relation  is the strict (asymmetric) part of  . Under a single-peaked preference relation, preference
                                           i                                    i
                      is strictly decreasing as one moves away from pi .
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0      July 5, 2007     14:18




                    246                     mechanism design without money

                      proof It is clear that Pareto-optimality implies the other two conditions. Sup-
                      pose f is strategy-proof and onto. Fix x ∈ [0, 1] and let  ∈ Rn be such that
                      f () = x. Consider any “unanimous” profile  ∈ Rn such that pi = x for
                      each i ∈ N. By strategy-proofness, f (1 , 2 , . . . , n ) = x, otherwise agent 1
                      could manipulate f. Repeating this argument, f (1 , 2 , 3 , . . . , n ) = x, . . . ,
                      f ( ) = x. That is, f is unanimous.
                         Finally, to derive a contradiction, suppose that f is not Pareto-optimal at some
                      profile  ∈ Rn . This implies that either (i) f () < pi for all i ∈ N or (ii) f () >
                      pi for all i ∈ N. Without loss of generality, assume (i) holds. Furthermore, assume
                      that the agents are labeled so that p1 ≤ p2 ≤ · · · ≤ pn .
                         If p1 = pn then unanimity is violated, completing the proof. Otherwise, let
                      j ∈ N be such that p1 = pj < pj +1 ; that is, j < n agents have the minimum
                      peak. For all i > j , let i be a preference relation such that both pi = p1 and
                      f () i pi .
                         Let xn = f (1 , . . . , n−1 , n ). By strategy-proofness, xn ∈ [f (), pn ], other-
                      wise agent n (with preference n ) could manipulate f by reporting preference n .
                      Similarly, xn ∈ (f (), pn ], otherwise agent n (with preference n ) could manip-
                      ulate f by reporting preference n . Therefore xn = f ().
                         Repeating this argument as each i > j replaces i with i , we have

                                             f (1 , . . . , j , j +1 , . . . , n ) = f ()

                      which contradicts unanimity. Since a strategy-proof, onto rule must be unanimous,
                      this is a contradiction.


                                                             10.2.1 Rules
                    The central strategy-proof rule on this domain is the simple median-voter rule. Suppose
                    that the number of agents n is odd. Then the rule that picks the median of the agents’
                    peaks (pi ’s) is a strategy-proof rule.
                       It is straightforward to see why this rule is strategy-proof : If an agent’s peak pi lies
                    below the median peak, then he can change the median only by reporting a preference
                    relation whose peak lies above the true median. The effect of this misreport is for
                    the rule to choose a point even further away from pi , making the agent worse off. A
                    symmetric argument handles the case in which the peak is above the median. Finally,
                    an agent cannot profitably misreport his preferences if his peak is the median one to
                    begin with.
                       More generally, for any number of agents n and any positive integer k ≤ n, the
                    rule that picks the kth highest peak is strategy-proof for precisely the same reasons as
                    above. An agent can only move the kth peak further from his own. The median happens
                    to be the case where k = (n + 1)/2.
                       The strategy-proofness of such rules stands in contrast to the incentives properties
                    of rules that choose average-type statistics. Consider the rule that chooses the average
                    of the n agents’ peaks. Any agent with peak pi ∈ (0, 1) that is not equal to the average
                    can manipulate the rule by reporting preferences with a more extreme peak (closer to
                    0 or 1) than his true peak.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0     July 5, 2007     14:18




                                        single-peaked preferences over policies                              247

                       This would also hold for any weighted average of the agents’ peaks, with one
                    exception. If a rule allocated all of the weight to one agent, then the resulting rule
                    simply picks that agent’s peak always. Such a dictatorial rule is strategy-proof and
                    onto.
                       In addition to favorable incentives properties, rules based on order statistics have
                    the property that they require little information to be computed. Technically a rule
                    requires agents to report an entire preference ordering over [0, 1]. The rules we have
                    discussed so far, however, only require agents to report their most preferred point, i.e.,
                    a single number. In fact, under the onto assumption, this informational property is a
                    consequence of the strategy-proofness requirement; that is, all strategy-proof and onto
                    rules have the property that they can be computed solely from information about the
                    agents’ peaks.
                       To begin showing this, we first observe that the class of “kth-statistic rules” can
                    be further generalized as follows. Consider a fixed set of points y1 , y2 , . . . , yn−1 ∈ A.
                    Consider the rule that, for any profile of preferences , chooses the median of the
                    2n − 1 points consisting of the n agents’ peaks and the n − 1 points of y. This kind of
                    rule differs from the previous ones in that, for some choices of y and some profiles of
                    preferences, the rule may choose a point that is not the peak of any agent’s preferences.
                    Yet, for the same reasons as above, such a rule is strategy-proof.
                       It turns out that such rules compose the entire class of strategy-proof and onto
                    rules that treat agents symmetrically. To formalize this latter requirement, we call a
                    rule anonymous if for any  ∈ Rn and any permutation  of , f ( ) = f (). This
                    requirement captures the idea that the agents’ names play no role in the behavior of
                    a rule. Dictatorial rules mentioned above are examples of rules that are strategy-proof
                    and onto, but not anonymous.

                      Theorem 10.2 A rule f is strategy-proof, onto, and anonymous if and only if
                      there exist y1 , y2 , . . . , yn−1 ∈ [0, 1] such that for all  ∈ Rn ,

                                        f () = med{p1 , p2 , . . . , pn , y1 , y2 , . . . , yn−1 }.      (10.1)

                      proof We leave it as an exercise to verify that such a rule satisfies the three
                      axioms in the Theorem. To prove the converse, suppose f is strategy-proof, onto,
                      and anonymous.
                         We make extensive use of the two (extreme) preference relations that have
                      peaks at 0 and 1 respectively. Since preferences relations are ordinal, there is only
                      one preference relation with a peak at 0 and only one with a peak at 1. Denote
                      these two preference relations by 0i and 1i respectively.
                         (Construct the ym ’s.) For any 1 ≤ m ≤ n − 1, let ym denote the outcome of f
                      when m agents have preference relation 1i and the remainder have 0i :
                                                                                          
                                         ym = f 01 , . . . , 0n−m , 1n−m+1 , . . . , 1n .

                      Recall that by anonymity the order of the arguments of f is irrelevant; if pre-
                      cisely m agents have preference relation 1i and the rest have 0i then the out-
                      come is ym . In addition, we leave it to the reader to verify that stragegy proofness
P1: SBT
9780521872829main       CUNY1061-Nisan           0 521 87282 0       July 5, 2007      14:18




                    248                            mechanism design without money

                       implies monotonicity of the ym ’s: ym ≤ ym+1 for each 1 ≤ m ≤ n − 2. We prove
                       the theorem by showing that f satisfies Eq. (10.1) with respect to this list of ym ’s.
                          Consider a profile of preferences  ∈ Rn with peaks p1 , . . . , pn . Without loss
                       of generality (by anonymity) assume that pi ≤ pi+1 for each i ≤ n − 1. We wish
                       to show f () = x ∗ ≡ med{p1 , . . . , pn , y1 , . . . , yn −1 }.
                          (Case 1: the median is some ym .) Suppose x ∗ = ym for some m. By mono-
                       tonicity of the peaks and ym ’s, since x ∗ is the median of 2n − 1 points this implies
                       pn−m ≤ x ∗ = ym ≤ pn−m+1 . By assumption,
                                                                                               
                                        x ∗ = ym = f 01 , . . . , 0n−m , 1n−m+1 , . . . , 1n .     (10.2)

                       Let x1 = f (1 , 02 , . . . , 0n−m , 1n−m+1 , . . . , 1n ). Strategy-proofness implies x1
                       ≥ x ∗ , otherwise agent 1 with preference 01 could manipulate f . Similarly, since
                       p1 ≤ ym , we cannot have x1 > x ∗ , otherwise agent 1 with preference 1 could
                       manipulate f . Hence x1 = x ∗ . Repeating this argument for all i ≤ n − m, x ∗ =
                       f (1 , . . . , n−m , 1n−m+1 , . . . , 1n ). The symmetric argument for all i > n − m
                       implies

                                                                 f (1 , . . . , n ) = x ∗ .                (10.3)

                          (Case 2: the median is an agent’s peak.) The remaining case is that ym < x ∗ <
                       ym+1 for some m. (The cases where x ∗ < y1 and x ∗ > yn−1 are similar, denoting
                       y0 = 0 and yn = 1.) In this case, since the agents’ peaks are in increasing order,
                       we have x ∗ = pn−m .
                          If
                                                                                      
                                 f 01 , . . . , 0n−m−1 , n−m , 1n−m+1 , . . . , 1n = x ∗ = pn−m (10.4)

                       then, analogous to the way Eq. (10.2) implied Eq. (10.3), repeated applications
                       of strategy-proofness (to the n − 1 agents other than i = n − m) would imply
                       f (1 , . . . , n ) = x ∗ , and the proof would be finished. The remainder of the
                       proof is devoted to showing that indeed Eq. (10.4) must hold.
                          Suppose to the contrary that
                                                                                           
                                      f 01 , . . . , 0n−m−1 , n−m , 1n−m+1 , . . . , 1n = x  < x ∗ . (10.5)

                       (The case x  > x ∗ can be proven symmetrically.) If agent (n − m) were to report
                       preference 0n−m instead, f would choose outcome ym ; hence strategy-proofness
                       implies ym ≤ x  < x ∗ . See Figure 10.1.
                          Denote the outcomes that agent (n − m) can obtain by varying his preferences,
                       fixing the others, as5
                                                                                                           
                          O = x : ∃                                            ˜ n−m , 1n−m+1 , . . . , 1n .
                                       ˜ n−m s.t. x = f 01 , . . . , 0n−m−1 , 

                       By definition, x  ∈ O; Case 1 implies ym , ym+1 ∈ O. Strategy proofness implies
                       that x  = max{x ∈ O : x ≤ x ∗ }, otherwise by reporting some other preference,
                       agent (n − m) could obtain some x ∈ (x  , x ∗ ), violating strategy proofness.

                    5 The literature on strategy proofness refers to this as an option set.
P1: SBT
9780521872829main       CUNY1061-Nisan          0 521 87282 0         July 5, 2007    14:18




                                           single-peaked preferences over policies                                             249

                                                                pn − m    pL
                                                                           i     pH
                                                                                  i




                                             ym            x    x∗                           x     ym+1

                    Figure 10.1. Proof of Theorem 10.2. If a strategy-proof, onto rule does not pick x ∗ when it is
                    the median of peaks and ym’s, then a contradiction is reached using preferences with peaks at
                    piL and piH .


                             Letting x  ≡ inf{x ∈ O : x ≥ x ∗ }, strategy proofness implies x  ∈ O.6 To
                       see this, let n−m be a preference relation with peak pn−m                    
                                                                                                            = x  and such
                                               
                       that (x + ) n−m x for some small  > 0. Then strategy proofness implies
                       f (01 , . . . , 0n−m−1 , n−m , 1n−m+1 , . . . , 1n )} = x̂ ∈ [x  , x  + ]. But if x̂ = x  ,
                       then there would exist a misreport resulting in an outcome arbitrarily closer to
                       x  , making agent (n − m) (with preference n−m ) better off. Hence x̂ = x  =
                       min{x ∈ O : x ≥ x ∗ }. With Eq. (10.5), we have x  > x ∗ .
                             We have shown that O ∩ (x  , x  ) = ∅. Let piL be a symmetric preference
                       relation with peak at p L = (x  + x  )/2 − ε, where ε > 0 is sufficiently small;
                       see Figure 10.1. Similarly let piH be a symmetric preference relation with peak at
                       (x  + x  )/2 + ε. Then strategy-proofness implies
                                                                                                   
                                         f 01 , . . . , 0n−m−1 , H
                                                                    n−m , n−m+1 , . . . , n
                                                                               1                 1
                                                                                                          = x  .
                       By repeated application of strategy-proofness (along the lines used in proving
                       Eq. (10.3)), this implies
                                                                                         
                                     f L1 , . . . , Ln−m−1 , H
                                                                n−m , n−m+1 , . . . , n
                                                                       1                1
                                                                                             = x  .
                       Lemma 10.1 (Pareto-optimality) implies
                                                                                       
                                 f L1 , . . . , Ln−m−1 , Ln−m , 1n−m+1 , . . . , 1n ≥ piL .
                       Therefore, strategy-proofness implies
                                                                                           
                                     f L1 , . . . , Ln−m−1 , Ln−m , 1n−m+1 , . . . , 1n = x                        (10.6)
                       otherwise agent n − m could manipulate at one of the two profiles (since ε is
                       small).
                          On the other hand, strategy-proofness implies
                                                                                           
                                     f 01 , . . . , 0n−m−1 , Ln−m , 1n−m+1 , . . . , 1n = x 
                       by the definition of Li . Strategy-proofness implies that if agent (n − m − 1)
                       instead reports preference L , a point must be chosen that is in the interval
                       [x  , x  − 2ε], otherwise, he could report 0 to gain. By repeated application of
                       this argument, this continues to hold as each agent 1 ≤ i ≤ n − m − 1 changes
                       his report from 0i to Li , so
                                                                                          
                                    f L1 , . . . , Ln−m−1 , Ln−m , 1n−m+1 , . . . , 1n ∈ [x  , x  − 2ε].

                    6 More generally, strategy-proofness alone implies O is closed. For brevity we prove only x  ∈ O.
P1: SBT
9780521872829main      CUNY1061-Nisan           0 521 87282 0     July 5, 2007     14:18




                    250                           mechanism design without money

                         This contradicts Eq. (10.6). Hence Eq. (10.5) cannot hold, so x  ≥ x ∗ ; the
                       symmetric argument implies x  = x ∗ , resulting in Eq. (10.4). Thus f chooses the
                       median of these 2n − 1 points for profile .

                       The parameters (ym ’s) in Theorem 10.2 can be thought of as the rule’s degree of
                    compromise when agents have extremist preferences. If m agents prefer the highest
                    possible outcome (1), while n − m prefer the lowest (0), then which point should
                    be chosen? A true median rule would pick whichever extreme (0 or 1) contains
                    the most peaks. On the other hand, the other rules described in the Theorem may
                    choose intermediate points (ym ) as a compromise. The degree of compromise (which
                    ym ) can depend on the degree to which the agents’ opinions are divided (the size
                    of m).
                       The anonymity requirement is a natural one in situations where agents are to be
                    treated as equals. If one does not require this, however, the class of strategy-proof
                    rules becomes even larger. We have already mentioned dictatorial rules, which always
                    chooses a predetermined agent’s peak. There are less extreme violations of anonymity:
                    The full class of strategy-proof, onto rules, which we now define, allows agents to be
                    treated with varying degrees of asymmetry.

                       Definition 10.3 A rule f is a generalized median voter scheme (g.m.v.s.) if
                       there exist 2n points in [0, 1], {αS }S⊆N , such that
                          (i) S ⊆ T ⊆ N implies αS ≤ αT ,
                        (ii) α∅ = 0, αN = 1, and
                       (iii) for all  ∈ Rn , f () = maxS⊂N min{αS , pi : i ∈ S}.

                        An example is given below. It is worth making two observations regarding Defi-
                    nition 10.3. First, the monotonicity condition (i) is actually redundant. If parameters
                    {αS }S⊆N fail this condition, they still define some strategy-proof rule via condition (iii).
                    However, the resulting rule could also be defined by an alternate set of parameters
                    {αS }S⊆N that do satisfy condition (i). Second, condition (ii) is present merely to guar-
                    antee the rule to be onto. Parameters that fail this condition still define a strategy-proof
                    rule whose range is [α∅ , αN ].7
                        Consider the rule described by the parameters (αS ’s) in Figure 10.2, for the 3-agent
                    case. The reader should first verify the following. If each agent in some set S ⊆ N
                    were to have a preference peak at 1, while each remaining agent (in N \S) were to have
                    a preference peak at 0, then the rule would choose αS as the outcome. In this sense, the
                    αS parameters reflect a (nonanonymous) degree of compromise at extreme preference
                    profiles, analogous to the ym parameters of Theorem 10.2.
                        Without the anonymity condition, some agents – more generally some coalitions of
                    agents – are more powerful than others. To see this, consider the profile of preferences
                    represented in Figure 10.2 with peaks p1 , p2 , p3 . Following condition (iii) of Defi-
                    nition 10.3, calculate min{αS , pi : i ∈ S} for each S ⊆ N. Beginning with the three

                    7 To avoid potential confusion, we point out that, in some of the literature, the term generalized median voter

                      scheme also refers to such rules.
P1: SBT
9780521872829main        CUNY1061-Nisan            0 521 87282 0         July 5, 2007         14:18




                                                 single-peaked preferences over policies                                                     251

                                                                        p2              p1                 p3
                                             0                                                                          1
                                            α∅     α1      α2    α1,2         α3             α1,3      α2,3            αN

                                Figure 10.2. An example of a generalized median voter scheme for n = 3.



                    singleton coalitions of the form S = {i}, these values are α1 , α2 , and α3 , because each
                    pi is above that agent’s corresponding α{i} . (For peak p3 , the third value would have
                    been p3 instead.) Since the g.m.v.s. eventually chooses the maximum of these kinds of
                    values (after we also check larger coalitions), agent 3 can be said to have more power
                    than the other two agents, as a singleton. A large α3 corresponds to more instances
                    in which agent 3’s peak is a candidate outcome for this rule. A small α1 corresponds
                    to more instances in which agent 1 has no impact on the outcome (i.e., whenever
                    p1 > α{1} ).
                       On the other hand, we also need to calculate these minimum-values for larger
                    coalitions. For the pairs of agents {1, 2}, {1, 3}, and {2, 3}, these values are α{1,2} , p1 ,
                    and p2 respectively. Coalition {1, 2} is the weakest two-agent coalition in the sense that
                    they have the lowest αS . After checking S = ∅ (which yields 0) and S = N (yielding a
                    repetition of the value p2 ), we calculate the rule’s outcome to be the maximum of the
                    2n values {0, α1 , α2 , α3 , α{1,2} , p1 , p2 , p2 } we have obtained, which is α{3} .
                       We close by stating the main result of this section. We omit its proof, which has
                    much in common with the proof of Theorem 10.2.

                       Theorem 10.4 A rule f is strategy-proof and onto if and only if it is a general-
                       ized median voter scheme.


                                           10.2.2 Application to Public Good Cost Sharing
                    Consider a group of n agents who have access to a machine that can convert their labor
                    into some public good. Specifically, suppose that the machine requires the simultaneous
                    labor of all n agents in order to work. The agents are free to jointly decide how many
                    hours of labor, , to work. Implicit is the requirement that each agent work for  hours,
                    however, since the machine requires all n agents’ labor simultaneously. After  hours of
                    labor, the machine outputs y = Y () units of some public good, where the production
                    function Y is assumed to be an increasing and strictly concave function, with Y (0) = 0.
                        Different agents may have different preferences over how much labor they should
                    provide, in exchange for the public good. Let us suppose that we know nothing about
                    their preferences, other than the fact that they are represented by some utility function
                    ui (, y) which is strictly increasing in y, strictly decreasing in , and is quasi-concave.8
                    See Figure 10.3.
                        In this environment, a rule takes as input the reported utility functions of the agents,
                    subject only to the assumptions we have made. It then gives as output a single labor
                    requirement  = f (u1 , . . . , un ). Each agent is then required to provide  units of labor,

                    8 The function u() is quasi-concave if, at each (, y), the upper contour set {( , y  ) : u( , y  ) ≥ u(, y)} is convex.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0      July 5, 2007         14:18




                    252                       mechanism design without money

                                        y

                                                                          u
                                                                                                      f
                                                                       ( , y )
                                                                                            (ˆ, y)
                                                                                                 ˆ



                                                      (, y)
                                                                                                u


                                         O                                                            

                    Figure 10.3. An agent with utility function u most prefers the outcome (y, ); one with u
                    prefers (y ,  ).



                    and they enjoy Y () units of output as a reward. What rules are strategy-proof and
                    onto?
                        By assumption, outcomes may only be attained along the graph of Y. Because of
                    the assumptions on Y and on preferences, it is clear that agents have single-peaked
                    preferences over this consumption space. It follows that any strategy-proof, onto rule
                    for this environment is a generalized median voter schemes operating along the graph
                    of Y .
                        Proving this is not difficult, but involves some technical details that we omit. First
                    the outcome space is not bounded as we assumed before, although it would certainly be
                    reasonable to bound it by assumption. Second, the preference domain here should be
                    verified to yield all the single-peaked preferences necessary to characterize generalized
                    median voter schemes; e.g., we used symmetric single-peaked preferences to construct
                    the proof of Theorem 10.2. Third, one should demonstrate that a strategy-proof rule in
                    this environment is invariant to utility information away from the graph of Y . We leave
                    it to the interested reader to verify our claim despite these technicalities.
                        In this kind of problem, it may be reasonable to add additional requirements to
                    a rule. One that we address is the requirement that an agent should be better off as
                    part of this decision-making group than if he were simply to walk away. Formally, if
                    this public good technology did not exist, each agent would provide no labor ( = 0),
                    and would enjoy none of the public good (y = 0). We say a rule is individually
                    rational if for all U = (u1 , . . . , un ) and 1 ≥ i ≥ n, we have ui (f (U ), Y (f (U ))) ≥
                    ui (0, 0).
                        What strategy-proof and onto rules satisfy individual rationality? In terms of our
                    earlier model, where agents have single-peaked preferences on [0, 1], that question
                    translates as follows: What g.m.v.s. has the property that, for any preference profile,
                    each agent (weakly) prefers the chosen outcome to the outcome x = 0?
                        The answer is that there is a unique such rule. As an exercise, we leave it to the
                    reader to show that the rule that chooses the minimum peak is the unique strategy-proof,
                    onto rule that satisfies this individual rationality condition. In terms of this public good
                    model, this corresponds to asking each agent their most preferred labor level , and
                    choosing the minimum.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:18




                                                house allocation problem                                   253

                                             10.3 House Allocation Problem

                    The House allocation problem is a model for understanding the allocation of indivisible
                    goods. It involves a set N of n agents, each owning a unique house and a strict preference
                    ordering over all n houses. The objective is to reallocate the houses among the agents
                    in an appropriate way. A modern version of the same would replace houses by kidneys.
                        While any possible (strict) preference ordering over the homes is permitted, the set of
                    preferences over allocations is restricted. In particular, an agent is indifferent between
                    all allocations that give her the same house. Therefore the Gibbard–Satterthwaite
                    Theorem does not apply in this setting.
                        One could select an allocation of homes in a variety of ways, perhaps so as to optimize
                    some function of the preferences and then investigate if the resulting allocation rule
                    is strategy-proof. However, this ignores an important feature not present in earlier
                    examples. In this environment, agents control the resources to be allocated. Therefore
                    an allocation can be subverted by a subset of agents who might choose to break away
                    and trade among themselves. For this reason it is natural to focus on allocations that
                    are invulnerable to agents opting out.
                        Number each house by the number of the agent who owns that house. An allocation
                    is an n vector a whose ith component, ai , is the number of the house assigned to agent
                    i. If a is the initial allocation then ai = i. For an allocation to be feasible, we require
                    that ai = aj for all i = j . The preference ordering of an agent i will be denoted i
                    and x i y will mean that agent i ranks house x above house y. Denote by A the set
                    of all feasible allocations. For every S ⊆ N let A(S) = {z ∈ A : zi ∈ S ∀i ∈ S} denote
                    the set of allocations that can be achieved by the agents in S trading among themselves
                    alone. Given an allocation a ∈ A, a set S of agents is called a blocking coalition (for
                    a) if there exists a z ∈ A(S) such that for all i ∈ S either zi i ai or zi = ai and for
                    at least one j ∈ S we have that zj j aj . A blocking coalition can, by trading among
                    themselves, receive homes that each strictly prefers (or is equivalent) to the home she
                    receives under a, with at least one agent being strictly better off. The set of allocations
                    that is not blocked by any subset of agents is called the core.
                        The reader will be introduced to the notion of the core in Chapter 15 (Section 15.2)
                    where it will be defined for a cooperative game in which utility is transferable via
                    money (a TU game). The house allocation problem we consider is an example of a
                    cooperative game with nontransferable utility (an NTU game). The definition of the
                    core offered here is the natural modification of the notion of TU core to the present
                    setting.
                        The theorem below shows the core to be nonempty. The proof is by construction
                    using the top trading cycle algorithm (TTCA).

                      Definition 10.5 (Top Trading Cycle Algorithm) Construct a directed graph
                      using one vertex for each agent. If house j is agent i’s kth ranked choice, in-
                      sert a directed edge from i to j and color the edge with color k. An edge of
                      the form (i, i) will be called a loop. First, identify all directed cycles and loops
                      consisting only of edges colored 1. The strict preference ordering implies that the
                      set of such cycles and loops is node disjoint. Let N1 be the set of vertices (agents)
                      incident to these cycles. Each cycle implies a sequence of swaps. For example,
P1: SBT
9780521872829main      CUNY1061-Nisan    0 521 87282 0   July 5, 2007   14:18




                    254                    mechanism design without money

                      suppose i1 → i2 → i3 → · · · → ir is one such cycle. Give house i1 to agent ir ,
                      house ir to agent ir−1 , and so on. After all such swaps are performed, delete all
                      edges colored 1. Repeat with the edges colored 2 and call the corresponding set
                      of vertices incident to these edges N2 , and so on. The TTCA yields the resulting
                      matching.

                      This algorithm is used to prove the following result.

                      Theorem 10.6      The core of the house allocation problem consists of exactly one
                      matching.

                      proof We prove that if a matching is in the core, it must be the one returned
                      by the TTCA.
                         Under the TTCA, each agent in N1 receives his favorite house, i.e., the house
                      ranked first in his preference ordering. Therefore, N1 would form a blocking
                      coalition to any allocation that does not assign to all of those agents the houses
                      they would receive under the TTCA. That is, any core allocation must assign N1
                      to houses just as the TTCA assigns them.
                         Given this fact, the same argument applies to N2 : Under the TTCA, each agent
                      in N2 receives his favorite house not including those houses originally endowed
                      by agents in N1 . Therefore, if an allocation is in the core and the agents in N1
                      are assigned each other’s houses, then agents in N2 must receive the same houses
                      they receive under the TTCA.
                         Continuing the argument for each Nk proves that if an allocation is in the core,
                      then it is the one determined by the TTCA. This proves that there is at most one
                      core allocation.
                         To prove that the TTCA allocation is in the core, it remains to be shown that
                      there is no other blocking coalition S ⊆ N. This is left to the reader.

                       To apply the TTCA, one must know the preferences of agents over homes. Do
                    they have an incentive to truthfully report these? To give a strongly positive answer
                    to this question, we first associate the TTCA with its corresponding direct revelation
                    mechanism. Define the Top Trading Cycle (TTC) Mechanism to be the function
                    (mechanism) that, for each profile of preferences, returns the allocation computed by
                    the TTCA.

                      Theorem 10.7      The TTC mechanism is strategy-proof.

                      proof Let π be a profile of preference orderings and a the allocation returned
                      by TTCA when applied to π. Suppose that agent j ∈ Nk for some k misreports
                      her preference ordering. Denote by π  the new profile of preference orderings.
                      Let a  the allocation returned by TTCA when applied to        π  . If the TTCA is
                                                                       
                      not strategy-proof ai > ai . Observe that ai = ai for all i ∈ k−1
                                              i
                                                                                        r=1 Nr . Therefore,
                       
                                 k−1
                      ai ∈ N \ { r=1 Nr }. However, the TTCA chooses ai to be agent i’s top ranked
                                                                                
                      choice from N \ { k−1r=1 Nr } contradicting the fact that ai > ai .
                                                                                    i


                       If we relax the requirement that preferences be strict, what we had previously called
                    a blocking set is now called a weakly blocking set. What we had previously called the
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0   July 5, 2007   14:18




                                                         stable matchings                                  255

                    core is now called the strict core. With indifference, a blocking set S is one where all
                    agents in S are strictly better off by trading among themselves. Note the requirement
                    that all agents be strictly better off. The core is the set of allocations not blocked by
                    any set S.
                        When preferences are strict, every minimal weakly blocking set is a blocking set. To
                    see this, fix a weakly blocking set S. An agent in S who is not made strictly better off
                    by trade among agents in S must have been assigned their own home. Remove them
                    from S. Repeat. The remaining agents must all be allocated houses that make them
                    strictly better off. Hence, when preferences are strict the core and strict core coincide.
                    With indifference permitted, the strict core can be different from the core. In fact, there
                    are examples where the strict core is empty and others where it is not unique. Deciding
                    emptiness of the strict core is polynomial in |N|.
                        Another possible extension of the model is to endow the agents with more than
                    one good. For example, a home and a car. Clearly, if preferences over pairs of goods
                    are sufficiently rich, the core can be empty. It turns out that even under very severe
                    restrictions the core can still be empty. For example, when preferences are separable,
                    i.e., one’s ranking over homes does not depend on which car one has.


                                                       10.4 Stable Matchings

                    The stable matching problem was introduced as a model of how to assign students to
                    colleges. Since its introduction, it has been the object of intensive study by both com-
                    puter scientists and economists. In computer science it used as vehicle for illustrating
                    basic ideas in the analysis of algorithms. In economics it is used as a stylized model
                    of labor markets. It has a direct real-world counterpart in the procedure for matching
                    medical students to residencies in the United States.
                       The simplest version of the problem involves a set M of men and a set W of women.
                    Each m ∈ M has a strict preference ordering over the elements of W and each w ∈ W
                    has a strict preference ordering over the men. As before the preference ordering of
                    agent i will be denoted i and x i y will mean that agent i ranks x above y. A
                    matching is an assignment of men to women such that each man is assigned to at most
                    one woman and vice versa. We can accommodate the possibility of an agent choosing
                    to remain single as well. This is done by including for each man (woman) a dummy
                    woman (man) in the set W (M) that corresponds to being single (or matched with
                    oneself). With this construction we can always assume that |M| = |W |.
                       As in the house allocation problem a group of agents can subvert a prescribed
                    matching by opting out. In a manner analogous to the house allocation problem, we
                    can define a blocking set. A matching is called unstable if there are two men m, m
                    and two women w, w such that
                      (i) m is matched to w,
                     (ii) m is matched to w  , and
                    (iii) w  m w and m w m
                    The pair (m, w ) is called a blocking pair. A matching that has no blocking pairs is
                    called stable.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0    July 5, 2007    14:18




                    256                     mechanism design without money

                      Example 10.8 The preference orderings for the men and women are shown in
                      the table below
                                             m1     m2     m3     w1     w2    w3
                                             w2      w1      w1      m1      m3     m1
                                             w1      w3      w2      m3      m1     m3
                                             w3      w2      w3      m2      m2     m2
                      Consider the matching {(m1 , w1 ), (m2 , w2 ), (m3 , w3 )}. This is an unstable match-
                      ing since (m1 , w2 ) is a blocking pair. The matching {(m1 , w1 ), (m3 , w2 ), (m2 , w3 )},
                      however, is stable.

                       Given the preferences of the men and women, is it always possible to find a sta-
                    ble matching? Remarkably, yes, using what is now called the deferred acceptance
                    algorithm. We describe the male-proposal version of the algorithm.

                      Definition 10.9 (Deferred Acceptance Algorithm, male-proposals) First, each
                      man proposes to his top-ranked choice. Next, each woman who has received at
                      least two proposals keeps (tentatively) her top-ranked proposal and rejects the rest.
                      Then, each man who has been rejected proposes to his top-ranked choice among
                      the women who have not rejected him. Again each woman who has at least two
                      proposals (including ones from previous rounds) keeps her top-ranked proposal
                      and rejects the rest. The process repeats until no man has a woman to propose to
                      or each woman has at most one proposal. At this point the algorithm terminates
                      and each man is assigned to a woman who has not rejected his proposal. Notice
                      that no man is assigned to more than one woman. Since each woman is allowed
                      to keep only one proposal at any stage, no woman is assigned to more than one
                      man. Therefore the algorithm terminates in a matching.

                       We illustrate how the (male-proposal) algorithm operates using Example 10.8 above.
                    In the first round, m1 proposes to w2 , m2 to w1 , and m3 to w1 . At the end of this round
                    w1 is the only woman to have received two proposals. One from m3 and the other from
                    m2 . Since she ranks m3 above m2 , she keeps m3 and rejects m2 . Since m3 is the only
                    man to have been rejected, he is the only one to propose again in the second round. This
                    time he proposes to w3 . Now each woman has only one proposal and the algorithm
                    terminates with the matching {(m1 , w2 ), (m2 , w3 ), (m3 , w2 )}. It is easy to verify that
                    the matching is stable and that it is different from the one presented earlier.

                      Theorem 10.10       The male propose algorithm terminates in a stable matching.

                      proof Suppose not. Then there exists a blocking pair (m1 , w1 ) with m1 matched
                      to w2 , say, and w1 matched to m2 . Since (m1 , w1 ) is blocking and w1 m1 w2 , in
                      the proposal algorithm, m1 would have proposed to w1 before w2 . Since m1 was
                      not matched with w1 by the algorithm, it must be because w1 received a proposal
                      from a man that she ranked higher than m1 . Since the algorithm matches her to
                      m2 it follows that m2 w1 m1 . This contradicts the fact that (m1 , w1 ) is a blocking
                      pair.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:18




                                                      stable matchings                                   257

                        One could just as well have described an algorithm where the women propose and
                    the outcome would also be a stable matching. Applied to the example above, this would
                    produce a stable matching different from the one generated when the men propose.
                    Thus, not only is a stable matching guaranteed to exist but there can be more than 1. If
                    there can be more than one stable matching, is there a reason to prefer one to another?
                    Yes. To explain why, some notation.
                        Denote a matching by µ. the woman assigned to man m in the matching µ is denoted
                    µ(m). Similarly, µ(w) is the man assigned to woman w. A matching µ is male-optimal
                    if there is no stable matching ν such that ν(m) m µ(m) or ν(m) = µ(m) for all m with
                    ν(j ) j µ(j ) for at least one j ∈ M. Similarly define female-optimal.

                      Theorem 10.11 The stable matching produced by the (male-proposal) Deferred
                      Acceptance Algorithm is male-optimal.

                      proof Let µ be the matching returned by the male-propose algorithm. Suppose
                      µ is not male optimal. Then, there is a stable matching ν such that ν(m) m µ(m)
                      or ν(m) = µ(m) for all m with ν(j ) j µ(j ) for at least one j ∈ M. Therefore, in
                      the application of the proposal algorithm, there must be an iteration where some
                      man j proposes to ν(j ) before µ(j ) since ν(j ) j µ(j ) and is rejected by woman
                      ν(j ). Consider the first such iteration. Since woman ν(j ) rejects j she must have
                      received a proposal from a man i she prefers to man j . Since this is the first
                      iteration at which a male is rejected by his partner under ν it follows that man
                      i ranks woman ν(j ) higher than ν(i). Summarizing, i ν(j ) j and ν(j ) i ν(i)
                      implying that ν is not stable, a contradiction.

                        Clearly one can replace the word “male” by the word “female” in the statement
                    of the theorem above. It is natural to ask if there is a stable matching that would be
                    optimal with respect to both men and women. Alas, no. The example above has two
                    stable matchings: one male optimal and the other female optimal. At least one female
                    is strictly better off under the female optimal matching than the male optimal one and
                    no female is worse off. A similar relationship holds when comparing the two stable
                    matchings from the point of view of the men.
                        A stable matching is immune to a pair of agents opting out of the matching. We
                    could be more demanding and ask that no subset of agents should have an incentive
                    to opt out of the matching. Formally, a matching µ dominates a matching µ if there
                    is a set S ⊂ M ∪ W such that for all m, w ∈ S, both (i) µ (m), µ (w) ∈ S and (ii)
                    µ (m) m µ(m) and µ (w) w µ(w). Stability is a special case of this dominance
                    condition when we restrict attention to sets S consisting of a single couple. The set
                    of undominated matchings is called the core of the matching game. The next result is
                    straightforward.

                      Theorem 10.12      The core of the matching game is the set of all stable matchings.

                       Thus far we have assumed that the preference orderings of the agents is known to
                    the planner. Now suppose that they are private information to the agent. As before
                    we can associate a direct revelation mechanism with an algorithm for finding a stable
                    matching.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:18




                    258                     mechanism design without money

                      Theorem 10.13 The direct mechanism associated with the male propose algo-
                      rithm is strategy-proof for the males.

                      proof Suppose not. Then there is a profile of preferences π = (m1 , m2 ,
                      . . . , mn ) for the men, such that man m1 , say, can misreport his preferences and
                      obtain a better match. To express this formally, let µ be the stable matching
                      obtained by applying the male proposal algorithm to the profile π. Suppose that
                      m1 reports the preference ordering ∗ instead. Let ν be the stable matching that
                      results when the male-proposal algorithm is applied to the profile π 1 = (∗ ,
                      m2 , . . . , mn ). For a contradiction, suppose ν(m1 ) m1 µ(m1 ). For notational
                      convenience we will write a m b to mean that a m b or a = b.
                           First we show that m1 can achieve the same effect by choosing an ordering        ¯
                      where woman ν(m1 ) is ranked first. Let π 2 = (,   ¯ m2 , . . . , mn ). Knowing that
                      ν is stable with respect to the profile π 1 we show that it is stable with respect to
                      the profile π 2 . Suppose not. Then under the profile π 2 there must be a pair (m, w)
                      that blocks ν. Since ν assigns to m1 its top choice with respect to π 2 , m1 cannot
                      be part of this blocking pair. Now the preferences of all agents other than m1 are
                      the same in π 1 and π 2 . Therefore, if (m, w) blocks ν with respect to the profile
                      π 2 , it must block ν with respect to the profile π 1 , contradicting the fact that ν is
                      a stable matching under π 1 .
                           Let λ be the male propose stable matching for the profile π 2 . Since ν is a stable
                      matching with respect to the profile π 2 . As λ is male optimal with respect to the
                      profile π 2 , it follows that λ(m1 ) = ν(m1 ).
                           Thus we can assume that ν(m1 ) is the top-ranked woman in the ordering ∗ .
                      Next we show that the set B = {mj : µ(mj ) mj ν(mj )} is empty. This means that
                      all men, not just m1 , are no worse off under ν compared to µ. Since ν is stable
                      with respect to the original profile, π this contradicts the male optimality of µ
                      and completes the proof.
                           Suppose B = ∅. Therefore, when the male proposal algorithm is applied to the
                      profile π 1 , each mj ∈ B is rejected by their match under µ, i.e., µ(mj ). Consider
                      the first iteration of the proposal algorithm where some mj is rejected by µ(mj ).
                      This means that woman µ(mj ) has a proposal from man mk that she ranks higher,
                      i.e., mk µ(mj ) mj . Since mk was not matched to µ(mj ) under µ it must be that
                      µ(mk ) mk µ(mj ). Hence mk ∈ B, otherwise
                                           µ(mj )  mk ν(mk ) mk µ(mk ) mk µ(mj ),
                      which is a contradiction.
                         Since mk ∈ B and mk has proposed to µ(mj ) at the time man mj proposes,
                      it means that mk must have been rejected by µ(mk ) prior to mj being rejected,
                      contradicting our choice of mj .

                       The mechanism associated with the male propose algorithm is not strategy-proof for
                    the females. To see why, it is enough to consider example. The male propose algorithm
                    returns the matching {(m1 , w2 ), (m2 , w3 ), (m3 , w1 )}. In the course of the algorithm the
                    only woman who receives at least two proposals is w1 . She received proposals from
                    m2 and m3 . She rejects m2 who goes on to propose to w3 and the algorithm terminates.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:18




                                                      stable matchings                                    259

                    Notice that w1 is matched with her second choice. Suppose now that she had rejected
                    m3 instead. Then m3 would have gone on to proposes to w2 . Woman w2 now has a
                    choice between m1 and m3 . She would keep m3 and reject m1 , who would go on to
                    propose to w1 . Woman w1 would keep m1 over m2 and in the final matching be paired
                    with a her first-rank choice.
                       It is interesting to draw an analogy between the existence of stable matchings and
                    that of Walrasian equilibrium. We know (Chapter 6) that Walrasian equilibria exist.
                    Furthermore, they are the solutions of a fixed point problem. In the cases when they can
                    be computed efficiently it is because the set of Walrasian equilibria can be described
                    by a set of convex inequalities. The same can be said of stable matchings. The set of
                    stable matchings is fixed points of a nondecreasing function defined on a lattice. In
                    addition, one can describe the set of stable matchings as the solutions to a set of linear
                    inequalities.


                                               10.4.1 A Lattice Formulation
                    We describe a proof of the existence of stable matchings using Tarski’s fixed point
                    theorem. It will be useful to relax the notion of a matching. Call an assignment of
                    women to men such that each man is assigned to at most one woman (but a woman
                    may be assigned to more than one man) a male semimatching. The analogous object
                    for women will be called a female semimatching. For example, assigning each man
                    his first choice would be a male semimatching. Assigning each woman her third choice
                    would be an example of a female semimatching.
                       A pair of male and female semimatchings will be called a semimatching which we
                    will denote by µ, ν, etc. An example of a semi-matching would consist of each man
                    being assigned his first choice and each woman being assigned her last choice.
                       The woman assigned to the man m under the semi-matching µ will be denoted
                    µ(m). If man m is assigned to no woman under µ, then µ(m) = m. Similarly for µ(w).
                    Next we define a partial order over the set of semimatchings. Write µ  ν if

                     (i) µ(m) m ν(m) or µ(m) = µ(m) for all m ∈ M and
                    (ii) µ(w) ≺w ν(w) or µ(w) = ν(w) for all w ∈ W .

                    Therefore µ  ν if all the men are better off under µ than in ν and all the women are
                    worse off under µ than in ν.
                       Next we define the meet and join operations. Given two semimatchings µ and ν
                    define λ = µ ∨ ν as follows:

                     (i) λ(m) = µ(m) if µ(m) m ν(m) otherwise λ(m) = ν(m),
                    (ii) λ(w) = µ(w) if µ(w) ≺w ν(w) otherwise λ(w) = ν(w).

                    Define λ = µ ∧ ν as follows:

                     (i) λ (m) = µ(m) if µ(m) ≺m ν(m) otherwise λ(m) = ν(m),
                    (ii) λ(w) = µ(w) if µ(w) w ν(w) otherwise λ(w) = ν(w).

                    With these definitions it is easy to check that the set of semimatchings forms a compact
                    lattice.
P1: SBT
9780521872829main      CUNY1061-Nisan    0 521 87282 0   July 5, 2007   14:18




                    260                    mechanism design without money

                       Now define a function f on the set of semi-matchings that is nondecreasing. Given
                    a semi-matching µ define f (µ) to be the following semi-matching:

                     (i) f (µ)(m) is man m’s most preferred woman from the set {w : m w µ(w), m = µ(w)}.
                         If this set is empty set f (µ)(m) = m.
                    (ii) f (µ)(w) is woman w’s most preferred man from the set {m : w m µ(m), w = µ(m)}.
                         If this set is empty set f (µ)(w) = w.

                      It is clear that f maps semi-matchings into semi-matchings.


                      Theorem 10.14 There is a semi-matching µ such that f (µ) = µ and that µ is
                      a stable matching.


                      proof We use Tarski’s theorem. It suffices to check that f is nondecreasing.
                      Suppose µ  ν. Pick any m ∈ M. From the definition of , the women are worse
                      off under µ than in ν. Thus

                                            {w : m w ν(w)} ⊆ {w : m w µ(w)}

                      and so f (µ)(m) m f (ν)(m) or f (µ)(m) = f (ν)(m). A similar argument applies
                      for each w ∈ W . Thus f is nondecreasing.
                          Since the conditions of Tarski’s theorem hold, it follows that there is a semi-
                      matching µ such that f (µ) = µ. We show that the semi-matching is a stable
                      matching.
                          By the definition of a semi-matching we have for every m ∈ M, µ(m) single
                      valued as is µ(w) for all w ∈ W . To show that µ is a matching, suppose not. Then
                      there is a pair m1 , m2 ∈ M, say, such that µ(m1 ) = µ(m2 ) = w∗ . Since f (µ) = µ
                      it follows that w∗ is m1 ’s top-ranked choice in {w : m1 w µ(w), m1 = µ(w)} and
                      m2 ’s top ranked choice in {w : m2 w µ(w), m2 = µ(w)}. From this we deduce
                                                              ∗
                      that µ(w∗ ) = m3 where m1 , m2 >w m3 . However, m3 = µ(w∗ ) = f (µ∗ )(w∗ ),
                      which is woman w∗ ’s top-ranked choice in {m : w∗ m µ(m), µ(m) = w∗ }. Since
                      m1 , m2 are members of this set, we get a contradiction.
                          To show that the matching µ is stable suppose not. Then there must be a
                      blocking pair (m∗ , w∗ ). Let w = µ(m∗ ) and m = µ(w∗ ), m = m∗ and w∗ =
                      w . Since (m∗ , w∗ ) is blocking, m∗ w∗ m and w∗ m∗ w . Now w = µ(m∗ ) =
                      f (µ)(m∗ ), which is man m∗ ’s top-ranked choice from {w : m∗ w µ(w), m∗ =
                      µ(w)}. But this set contains w∗ , which is ranked higher by man m∗ than w , a
                      contradiction.


                                                10.4.2 The LP Formulation
                    One can formulate the problem of finding a stable matching as the solution to a set of
                    linear inequalities. For each man m and woman w let xmw = 1 if man m is matched
                    with woman w and zero otherwise. Then, every stable matching must satisfy the
P1: SBT
9780521872829main      CUNY1061-Nisan        0 521 87282 0      July 5, 2007   14:18




                                                             stable matchings                             261

                    following.
                                                              
                                                                    xmw = 1       ∀m ∈ M
                                                              w∈W
                                                              
                                                                    xmw = 1       ∀w ∈ W
                                                              m∈M
                                                    
                                             xmj +           xiw + xmw ≤ 1        ∀m ∈ M, w ∈ W
                                    j ≺m w           i≺w m

                                                                    xmw ≥ 0       ∀m ∈ M, w ∈ W

                    Let P be the polyhedron defined by these inequalities.
                       The first two constraints of P ensure that each agent is matched with exactly one
                              of the opposite sex.
                    other agent                    The third constraint ensures stability. To see why,
                    suppose j ≺m w xmj = 1 and i≺w m xiw = 1. Then man m is matched to a woman, j
                    that he ranks below w. Similarly, woman w is matched to a man she ranks below m.
                    This would make the pair (m, w) a blocking pair.

                      Theorem 10.15          P is the convex hull of all stable matchings.



                                                             10.4.3 Extensions
                    We have been careful to specify that preferences are strict. If we allow for indifference,
                    Theorem 10.7 becomes false. This is because there are instances of the stable matching
                    problem in which no male or female optimal stable matching exists. The other theorems
                    stated above continue to hold in the presence of indifferences.
                       We also limited ourselves to one-to-one matchings. There are situations where one
                    side of the market wishes to match with more than one agent. The college admissions
                    market is the classic example. Each student can be assigned to at most one college
                    but each college can be assigned to many students. In this more general setup colleges
                    will have preferences over subsets of students. In the absence of any restrictions on
                    these preferences a stable matching need not exist. One restriction on preferences for
                    which the results above carry over with no change in statement or proof is the quota
                    model. Each college has a strict preference ordering over the students and a quota r
                    of students it wishes to admit. Consider two subsets, S and T , of students of size r
                    that differ in exactly one student. The college prefers the subset containing the more
                    preferred student.
                       A third extension is to relax the bipartite nature of the stable matching problem.
                    The nonbipartite version is called the stable roommates problem. Suppose that a set
                    of N individuals such that |N| is even. A matching in this setting is a partition of N
                    into disjoint pairs of individuals (roommates). Each individual has a strict preference
                    ordering over the other individuals that they would like to be paired with. As before,
                    a matching is unstable if there exists a pair who prefer each other to the person they
                    are matched with. Such a pair is called blocking. Unlike the stable matching problem,
                    stable roommates need not exist as the following four person example illustrates.
P1: SBT
9780521872829main      CUNY1061-Nisan    0 521 87282 0        July 5, 2007   14:18




                    262                    mechanism design without money

                                                         1      2     3    4
                                                         3       1      2     2
                                                         2       3      1     1
                                                         4       4      4     4


                       Each column lists the preference ordering that one agent has over the others. A
                    matching that pairs agent 1 with agent 4 will always be blocked by the pair (1, 2). A
                    matching that pairs 2 with 4 will be blocked by (2, 3). A matching that pairs 3 and 4
                    will be blocked by (3, 1).
                       An O(|N|2 ) algorithm to determine if a stable matching exists is known. One
                    can also associate a collection of linear inequalities with the stable roommates prob-
                    lem such that the system is feasible if and only if a stable roommates solution
                    exists.


                                                  10.5 Future Directions

                    While the models in this chapter have been studied and extended in a variety of ways,
                    there are plenty of open questions for the creative researcher.
                       One direction of future research on the single-peaked preference model of
                    Section 10.2 would be to consider choosing multiple alternatives (locations) on an
                    interval (or more general graph) when agents’ preferences are single-peaked with
                    respect to the one location that is closest to his peak. As an idealized example,
                    when downloading files on the Internet one cares only about the location (dis-
                    tance) of the closest “mirror” site. If a planner can elicit preferences to choose
                    the location of k mirrors on a network, how can this be done in a strategy-proof
                    way?
                       As for the house allocation model of Section 10.3 and the stable matching model of
                    Section 10.4, observe that both models are static in nature. Yet, there are a variety of
                    dynamic environments that resemble these models in important ways. As an example,
                    take the problem of allocating kidneys. Until quite recently those needing a kidney
                    transplant would have to wait in a queue (the wait list) for an available kidney that
                    would be an appropriate “fit” or else find a donor fulfilling the appropriate medical
                    conditions.
                       More recently, however, exchange systems have been implemented which al-
                    low kidney patients to “swap” their incompatible (but willing) friends and rela-
                    tives who are willing to donate a kidney. (Suppose that Alice needs a kidney,
                    and her incompatible friend Bob is willing to donate; also suppose that Carmina
                    and Dijen are in a similar situation. If Alice and Dijen are compatible, and if
                    Carmina and Bob are compatible, then a compatible “swap” can be arranged.)
                    Static versions of such a model have been analyzed by Roth, Sönmez, and Ünver
                    (2004).
                       Those authors and others have developed a substantial literature around this impor-
                    tant problem. If donors and recipients arrive dynamically to such a setting, how should
                    swaps be arranged?
P1: SBT
9780521872829main       CUNY1061-Nisan        0 521 87282 0     July 5, 2007    14:18




                                                        notes and references                                            263

                                                    10.6 Notes and References

                    The canonical results for the single-peaked preference model are provided by
                    Moulin (1980), who proved Theorems 10.2 and 10.4 with the additional requirement
                    that rules take agents’ peaks as their only input. Ching (1997) subsequently showed
                    that this requirement is redundant when a rule is strategy-proof and onto.
                        Border and Jordan (1983) generalize these conclusions to multidimensional models
                    where the outcome space is Rk . They restrict attention to separable preferences, i.e.,
                    under the assumption that an agent’s (relative) preferences over any one dimension
                    are fixed, as we vary any other dimensions of the altnerative. For example with k = 3,
                    if (x1 , x2 , x3 ) i (x1 , x2 , x3 ) then separability would imply (x1 , y2 , y3 ) i (x1 , y2 , y3 ).
                    Border and Jordan show that a strategy-proof, onto rule for separable preferences
                    must be decomposable into k (possibly different) one-dimensional rules. Of course,
                    these one-dimensional rules must be generalized median voter schemes. For fur-
                    ther reference on such generalizations, one should consult the survey of Barberà
                    (2001).
                        Another direction in which these results have been generalized pertains to situations
                    in which agents have single-peaked preferences on graphs. Schummer and Vohra (2004)
                    obtain two types of result, depending on whether the graph contains any cycle. Finally,
                    the book of Austen-Smith and Banks (2005). contains more details on the key results
                    of this literature, and a proof of Theorem 10.4.
                        The house allocation problem was introduced by Herbert Scarf and Lloyd Shapley
                    (1974). The TTCA is attributed by these authors to David Gale. The idea that the house
                    allocation problem can be used as a model for kidney exchanges is discussed in Roth
                    et al. (2004).
                        The stable matching problem was introduced by David Gale and Lloyd Shapley
                    (1962). The first algorithm for finding a stable matching was developed a decade
                    earlier in 1951 to match interns to hospitals (Stalnaker, 1953). The intrinsic appeal of
                    the model has inspired three books. The first, by Donald Knuth (1976) uses the stable
                    matching problem as a vehicle to illustrate some of the basic ideas in the analysis of
                    algorithms. The book by Gusfield and Irving (1989) is devoted to algorithmic aspects
                    of the stable matching problem and some of its relatives. On the economics side, the
                    book by Roth and Sotomayor (1991) gives a complete game theoretic treatment of the
                    stable matching problem as well as some of its relatives.
                        The lattice theoretic treatment of the stable matching problem goes back to Knuth
                    (1976). The proof of existence based on Tarski’s fixed point theorem is due to Adachi
                    (2000). In fact, the proposal algorithm is exactly one of the algorithms for finding a
                    fixed point when specialized to the case of stable matchings.
                        The linear programming formulation of the stable matching problem is due to Vande
                    Vate (1989). The extension of it to the stable room mates problem can be found in Teo
                    and Sethuraman (1998). Gusfield and Irving (1989) give a full algorithmic account of
                    the stable roommates problem.
                        In parallel, studies have been made of matching models where monetary transfers
                    are allowed. This has inspired models that unify both the stable matching problem as
                    well as matching problems where monetary transfers are allowed. Descriptions can be
                    found in Fleiner (2003) and Hatfield and Milgrom (2005).
P1: SBT
9780521872829main      CUNY1061-Nisan        0 521 87282 0     July 5, 2007   14:18




                    264                        mechanism design without money

                                                             Bibliography
                    H. Adachi. On a characterization of stable matchings. Economics Letters, 68:43–49, 2000.
                    D. Austen-Smith and J. Banks. Positive Political Theory II: Strategy and Structure. University of
                       Michigan Press, 2005.
                    S. Barberà. An introduction of strategy-proof social choice functions. Soc. Choice Welfare, 18(4):619–
                       653, 2001.
                    K. Border and J. Jordan. Straightforward elections, unanimity and phantom voters. Rev. Econ. Stud.,
                       50(1):153–170, 1983.
                    S. Ching. Strategy-proofness and Âmedian voters. Intl. J. Game Theor., 26(4):473–490, 1997.
                    T. Fleiner. Some results on stable matchings and fixed points. Math. Oper. Res., 28(1):103–126, 2003.
                    D. Gale and L.S. Shapley. College admissions and the stability of marriage. Amer. Math. Monthly,
                       69(1):9–15, 1962.
                    A. Gibbard. Manipulation of voting schemes: A general result. Econometrica, 41(4):587–601,
                       1973.
                    D. Gusfield and R.W. Irving. The Stable Marriage Problem: Structure and Algorithms. MIT Press,
                       1989.
                    J.W. Hatfield and P.R. Milgrom. Matching with contracts. Amer. Econ. Rev., 95(4):913–935, 2005.
                    D. Knuth. Marriages Stables. Les Presses de l’Universite de Montreal, 1976.
                    H. Moulin. On strategy proofness and single peakedness. Public Choice, 35(4):437–455, 1980.
                    A. E. Roth and M. Sotomayor. Two-Sided Matching: A Study in Game-Theoretic Modelling and
                       Analysis. Cambridge University Press, 1991.
                    A. E. Roth, T. Sönmez, and M. U. Ünver. Kidney exchange. Q. J. Econ., 119(2):457–488, 2004.
                    M. Satterthwaite. Strategy-proofness and arrow’s conditions. J. Econ. Theor., 10(2):187–217, 1975.
                    J. Schummer and R.V. Vohra. Strategy-proof location on a network. J. Economic Theory, 104(2):405–
                       428, 2004.
                    L.S. Shapley and H. Scarf. On cores and indivisibility. J. Math. Econ., 1(1):23–28, 1974.
                    J. M. Stalnaker. The matching program for intern placement: The second year of operation. J. Med.
                       Educ., 28(1):13–19, 1953.
                    C. P. Teo and J. Sethuraman. Geometry of fractional stable matchings and its applications. Math.
                       Oper. Res., 23(4):874–891, 1998.
                    J. H. VandeVate. Linear programming brings marital bliss. Oper. Res. Lett., 8(3):147–153, 1989.




                                                                Exercises
                    10.1 To what extent is Lemma 10.1 sensitive to the richness of the preference domain?
                         For example, does the result hold if the preference domain is even smaller, e.g.,
                         containing only symmetric single-peaked preferences?
                    10.2 Suppose that an anonymous rule described in Theorem 10.2 has parameters
                         (ym)n−1m=1 . Express this rule as a generalized median voter scheme with parameters
                         (α S ) S⊆N .
                    10.3 Suppose that a rule f is strategy-proof and onto, but not necessarily anonymous.
                         Fix the preferences of agents 2 through n, (2 ,. . .,n ), and denote the outcomes
                         obtainable by agent 1 as

                                 O = f ( · , 2 ,. . . , n ) = {x ∈ [0,1]:∃ 1 ∈ R s.t. f ( 1 , 2 ,. . . , n )}.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 5, 2007   14:18




                                                            exercises                                         265

                           Show that O = [a, b] for some a, b ∈ [0, 1] (without appealing directly to Theo-
                           rem 10.4).
                    10.4 Prove Theorem 10.4.
                    10.5 For the case of three agents, generalize Theorem 10.2 to a 3-leaved tree. Specifi-
                         cally, consider a connected noncyclic graph (i.e., a tree) with exactly three leaves,
                         1 , 2 , 3 . Preferences over such a graph are single-peaked if there is a peak pi such
                         that for any x in the graph, and any y in the (unique shortest) path from x to pi ,
                         y i x. The concepts of strategy-proofness, onto, and anonymity generalize in the
                         straightforward way to this setting. Describe all the rules that satisfy these condi-
                         tions for the case n = 3. (Hint: first show that when all agents’ peaks are restricted
                         to the interval [1 , 2 ], the rule must behave like one described in Theorem 10.2.)
                         For the nonanonymous case with n ≥ 3, see Schummer and Vohra (2004).
                    10.6 Prove that the TTCA returns an outcome in the core of the house allocation game.
                    10.7 The TTC mechanism is immune to agents misreporting their preferences. Is it
                         immune to agents misreporting the identity of their houses? Specifically, suppose
                         a subset of agents trade among themselves first before participating in the TTC
                         mechanism. Can all of them be strictly better off by doing so?
                    10.8 Consider an instance of the stable matching problem. Let ν be a matching (not
                         necessarily stable) and µ the male optimal stable matching. Let B = {m : ν(m) >m
                         µ(m)}. Show that if B = ∅ then there is a m ∈ B and woman w such that (m, w) is
                         a blocking pair for ν.
P1: SBT
9780521872829main   CUNY1061-Nisan   0 521 87282 0   July 5, 2007   14:18
