---
type: "book-chapter"
book_id: "algorithmic-game-theory"
chapter_id: "ch-14"
chapter_number: 14
chapter_title: "Chapter 14"
source_pdf: "raw/inbox/manual-drop/PDF_B.pdf"
source_page_start: 384
source_page_end: 405
printed_page_start: 384
printed_page_end: 405
part_ids: ["algorithmic-game-theory-ch-14-part-015"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Chapter 14

P1: SBT
9780521872829main       CUNY1061-Nisan       0 521 87282 0    July 5, 2007   14:26




                                                             CHAPTER 14


                                      Distributed Algorithmic
                                        Mechanism Design

                          Joan Feigenbaum, Michael Schapira, and Scott Shenker




                                                               Abstract

                    Most discussions of algorithmic mechanism design (AMD) presume the existence of a trusted center
                    that implements the required economic mechanisms. This chapter focuses on mechanism-design
                    problems that are inherently distributed, i.e., those in which such a trusted center cannot be used.
                    Such problems require that the AMD paradigm be generalized to distributed algorithmic mechanism
                    design (DAMD).
                        We begin this chapter by exploring the reasons that DAMD is needed and why it requires different
                    notions of economic equilibrium and computational complexity than centralized AMD. We then
                    consider two DAMD problems, namely distributed VCG computation and multicast cost sharing, that
                    illustrate the concepts of ex-post Nash equilibrium and network complexity, respectively.
                        The archetypal example of a DAMD challenge is interdomain routing, which we treat in detail. We
                    show that, under certain realistic and general assumptions, one can achieve incentive compatibility
                    in a collusion-proof ex-post Nash equilibrium without payments, simply by executing the Border
                    Gateway Protocol (BGP), which is the standard for interdomain routing in today’s Internet.



                                                         14.1 Introduction

                    To motivate the material in this chapter, we begin with a review of why game theory is
                    relevant to computer science. As noted in the Preface to this book, computer science
                    has traditionally assumed the existence of a central planner who dictates the algorithms
                    used by computational nodes. While most nodes are assumed to be obedient, some
                    nodes may malfunction or be subverted by attackers; such byzantine nodes may act
                    arbitrarily.
                       This book’s founding premise, in fact its raison d’être, is that there are many
                    computational contexts in which there is no central (or cooperative) authority that
                    controls the computational nodes. In particular, the Internet has changed computation
                    from a largely local endeavor to one that frequently involves diverse collections of
                    individuals (or machines acting on their behalf). For example, Web services, peer-to-
                    peer systems, and even the interaction among packets on a wire are all cases in which
                                                                    363
P1: SBT
9780521872829main      CUNY1061-Nisan          0 521 87282 0      July 5, 2007     14:26




                    364                   distributed algorithmic mechanism design

                    individuals with no ties to each other, except perhaps a common interest in a document
                    or simultaneous use of a link, find themselves interacting over the Internet.
                       In such cases, it is often best to treat the computational entities as independent and
                    selfish agents, interested only in optimizing their own outcome. As a category of be-
                    havior, selfishness lies between the extremes of automatic obedience and byzantine dis-
                    ruption; selfish agents are unwilling to follow a central planner’s instructions, but they
                    do not act arbitrarily. Instead, their actions are driven by incentives, i.e., the prospect
                    of good or bad outcomes. The field of mechanism design, described in Chapter 9,
                    has shown how, by carefully constructing economic mechanisms to provide the proper
                    incentives, one can use selfish behavior to guide the system toward a socially desir-
                    able outcome.1 This book is devoted to exploring the interaction of incentives and
                    computing, a topic that has come to be known as Algorithmic Mechanism Design
                    (AMD).
                       Substituting a decentralized set of incentives for a central planner is a radical de-
                    parture from traditional algorithm design. However, most work in this new field of
                    AMD assumes the presence of a central computational facility that performs the cal-
                    culations required by the economic mechanism. In auctions, for example, the agents
                    each have independent goals and desires, but the computation to determine winners
                    and payments is done by the auctioneer, and the hardness of the computation is eval-
                    uated using traditional notions of complexity (see, e.g., Chapters 1, 9, 11, and 12).
                    As such, AMD considers novel incentive-related algorithm design but uses a standard
                    centralized model of algorithm execution.
                       This combination of decentralized incentives but centralized computation applies in
                    a wide variety of settings, many of which have been described elsewhere in this book.
                    This approach requires transmitting all the relevant information to a single, trusted
                    entity (hereafter called the trusted center), which is feasible if (i) such a trusted center
                    exists, and (ii) the communication required to transmit the information and the resulting
                    computational burden on the trusted center are both manageable. However, if either of
                    these two assumptions fails, then a more decentralized approach must be considered.
                       As we discuss in more detail in Section 14.3 of this chapter, the problem of inter-
                    domain routing is one in which a decentralized approach is valuable. The Internet is a
                    collection of smaller networks, called Autonomous Systems (ASes), that are stitched
                    together by the interdomain-routing system to form the fully connected Internet. The
                    interdomain-routing system therefore plays a crucial role in the functioning, even the
                    existence, of the Internet. However, any approach to interdomain routing must address
                    the challenges of trust, scalability, and reliability. The ASes are competing economic
                    entities who want to optimize the routing outcome achieved and minimize the private
                    information revealed; accordingly, they not only act selfishly but are also unwilling to
                    share private information with, or cede control to, any trusted center. Thus, the ASes
                    must distribute the route computation among themselves.
                       Even if trust were not an issue, scalability would drive the system toward distributed
                    route computation. Centralizing the route computation would involve transmitting the
                    entire AS graph to a central location and updating it whenever the graph changed.

                    1 This desired outcome is often defined as the optimum of some global objective function, but a wide variety of

                      social standards can also be used.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    July 5, 2007   14:26




                                                           introduction                                     365

                    Given the considerable size and volatility of the AS graph, such a centralized route
                    computation would be infeasible.
                       Similarly, the need for reliability, so crucial in the Internet, tends to favor decen-
                    tralized designs. In a centralized design, the trusted center becomes a single point of
                    failure; the fate of the entire network rests on this single system that could fail or
                    be subverted. As an example of how scalability and reliability can drive the need for
                    decentralization, we note that current intradomain-routing algorithms, which do not
                    span more than one AS and so are designed with the assumption of mutual trust among
                    routers, are almost all distributed.
                       Thus, there is a need to decentralize not only incentives but also computation; this
                    leads to Distributed Algorithmic Mechanism Design (DAMD), which is the central
                    focus of this chapter. DAMD has the same dual concerns, incentive compatibility, and
                    computational complexity, as AMD, but it differs in two important respects.
                       The first difference involves the nature of complexity. DAMD’s measure of com-
                    putational complexity is quite different from AMD’s, because the computation is
                    distributed. Any measure of the complexity of a distributed algorithm executed over
                    an interconnection network T must consider at least five quantities: the total number
                    of messages sent over T , the maximum number of messages sent over any one link
                    in T , the maximum size of a message, the local computational burden at each node,
                    and the storage required at each node. If a distributed algorithm requires an excessive
                    expenditure of any one of these resources, then its complexity is unacceptable. We will
                    use the term network complexity to refer to these, and other, metrics of the difficulty of
                    distributed implementation.
                       If the interconnection network T is trusted by all the agents and can feasibly serve
                    as the trusted center, then the measure of complexity is the main difference between
                    AMD and DAMD. However, if the distributed computation is done by the agents, then
                    a second difference arises: the strategic nature of the computation itself. In AMD,
                    agents can manipulate a game only by their selection of actions among those described
                    in the definition of the economic mechanism; they cannot affect the computation
                    of the mechanism, because all outcomes are computed (by the trusted center) from
                    the vector of strategies, according to the definition of the mechanism. If the agents
                    themselves perform the computation using some distributed algorithm, then they have
                    more opportunities to manipulate the outcome, e.g., by misrepresenting the results
                    of a local computation to a neighboring agent or, more drastically, by simply not
                    communicating with that neighboring agent at all, in an attempt to exclude him from the
                    game. Our assumption of selfishness requires that we consider all forms of manipulative
                    behavior when designing the economic mechanism; in particular, this means that we
                    must provide incentives that ensure selfish agents find it in their best interest to perform
                    the distributed computation correctly.
                       While this chapter discusses the use of incentives to prevent these other forms of
                    manipulation, one can also use cryptographic protocols to replace trusted parties in
                    mechanism computation. This active area of study is covered in Chapter 8 of this
                    volume.
                       In the next section of this chapter, we briefly discuss two examples of DAMD. Our
                    third section is devoted to an in-depth exploration of the incentive issues in interdomain
                    routing. We conclude with open questions and exercises.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:26




                    366              distributed algorithmic mechanism design

                                              14.2 Two Examples of DAMD

                    As noted above, DAMD differs from AMD in two respects: the additional ways in
                    which the agents can influence the outcome (referred to hereafter as “computational
                    manipulation”) and the measure of computational complexity (the aforementioned
                    “network complexity”).
                        Here, we briefly discuss two examples of DAMD that illustrate these issues. The
                    first is a distributed implementation of a VCG mechanism (see Chapter 9); we will
                    ignore network complexity in this example and focus on how to prevent manipulation
                    of the computation. The second example is sharing the cost of a multicast transmission;
                    it illustrates the notion of network complexity but, because we assume the presence of
                    a trusted computational infrastructure, does not involve computational manipulation.



                                        14.2.1 Distributed Implementation of VCG
                    We now discuss one way a set of agents can jointly implement a VCG mechanism
                    without fear of manipulation. We start with a set of outcomes O and a collection
                    of agents N, each with his own valuation vi over those outcomes. In our notation,
                                   that maximizes the total social welfare of the agents. That is, õ =
                    õ is an outcome
                    argmaxo∈O i∈N vi (o), W is the maximum total social welfare value, and W−i denotes
                    the maximum total social welfare of all agents except the i’th. For convenience, we
                    focus on the particular mechanism in which pi = W−i − W + vi (õ), where pi is the
                    payment by agent i.
                        We assume that there is no trusted center; i.e., that the computation of the VCG
                    mechanism must be done by the agents themselves. However, we do presume the
                    existence of some central enforcer whose responsibility it is to implement the outcome
                    õ decided upon by the agents and collect the payments; the enforcer can impose severe
                    penalties if the agents do not agree on an outcome.
                        To see how a distributed computation can be manipulated, consider a network in
                    which the nodes are connected in a ring, and there is exactly one agent at each node.
                    Assume that the agents are computing a second-price auction of a single good by
                    passing around a message containing the top two bids for that good. If an agent puts
                    his bid on top and puts in a very low bid for the second bid, then he can get the good
                    more cheaply (as long as these fields are not overwritten by some later agents that have
                    higher bids).
                        More generally, consider any distributed algorithm A, capable of running over an
                    arbitrary number of computational nodes, that takes as input a set of agent valuations
                    and produces the maximizing outcome and the payments. As the preceding example
                    suggests, if we run A over any subset of N to compute õ, W , and each W−i , then there
                    is the possibility that an agent can manipulate the computation.
                        One way to avoid this is replication: Break the agents into two groups, have them
                    exchange all their valuations, and then have each group compute its own version of õ
                    and the pi . If the two groups agree on the outcomes and payments, then those outcomes
                    and payments are adopted; if not, all agents suffer a severe penalty. Here, an agent plays
                    different roles in the two versions of the computation: In the first, his role is to help
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:26




                                                   two examples of damd                                   367

                    compute the outcome and payments; in the other, his role is to provide his valuation so
                    that others may perform this computation. For the first version, an agent i could engage
                    in arbitrary computational manipulation in an attempt to obtain a more favorable pi or
                    choose an outcome he prefers to the socially optimal one; in the second version, all
                    he could do is lie about vi . Because the VCG mechanism is strategyproof, the agent
                    will reveal truthfully to the other computational group and therefore, to avoid a severe
                    penalty for inconsistency, will carry out the computation faithfully.
                       Notice that faithful computation is not a dominant strategy. If, for instance, all the
                    other agents decide to choose a suboptimal outcome, then agent i is better off going
                    along with that choice rather than causing a disagreement (and triggering the severe
                    penalty). However, if all the other agents faithfully execute the prescribed algorithm
                    A, then agent i is best off doing so as well. Thus, the most natural solution concept
                    when considering computational manipulation is not dominant strategies but instead
                    ex-post Nash equilibrium, which was defined in Chapter 9. We will expand on this
                    point further when we discuss interdomain routing in Section 14.3 below.
                       In this example, we have focused on computational manipulation and ignored net-
                    work complexity. In our next example, we do the opposite.


                                  14.2.2 Sharing the Cost of a Multicast Transmission
                    Multicast is an Internet packet-transmission mode that delivers a single packet to
                    multiple receivers. It is accomplished by setting up a shared delivery tree that spans all
                    the receivers; packets sent down this tree are replicated at branch points so that no more
                    than one copy of each packet traverses each link. Because it is far more efficient than
                    traditional unicast transmission (in which packets are sent only to a single destination),
                    multicast is particularly appropriate for distributing popular real-time content, such as
                    movies, to a large number of receivers.
                       Internet content distribution both provides benefits and incurs cost, which we can
                    model as follows. We assume that there are agents, located at various places in the
                    network, who would derive some utility from receiving the content and that a cost is
                    incurred each time the content is transmitted over a network link. The policy question
                    is how these costs and benefits should be distributed; more specifically, which agents
                    should receive the content, and how much should each agent pay?
                       To define the problem more precisely, we consider a user population P residing at
                    a set of network nodes N that are connected by bidirectional network links L. The
                    multicast flow emanates from a source node αo ∈ N; given any set of receivers S ⊆ P ,
                    the transmission flows through a multicast tree T (S) ⊆ L rooted at αo that spans the
                    nodes at which users in S reside. We make the natural assumption that routing is
                    monotonic, i.e., that S1 ⊆ S2 ⇒ T (S1 ) ⊆ T (S2 ).
                       Each link l ∈ L has an associated cost c(l) ≥ 0 that is known by the nodes on
                    each end, and each user i assigns a utility value ui to receiving the transmission.
                                                                                                  
                    The total cost C(S) of reaching a set S of receivers is given by C(S) = l∈T (S) c(l),
                    and the net welfare NW (S) of delivering content to this set of receivers is given by
                    NW (S) = i∈S ui − C(S).
                       A cost-sharing mechanism determines which users receive the multicast transmis-
                    sion and how much each receiver is charged. We let pi ≥ 0 denote how much user i
P1: SBT
9780521872829main      CUNY1061-Nisan         0 521 87282 0      July 5, 2007    14:26




                    368                  distributed algorithmic mechanism design

                    is charged and σi denote whether user i receives the transmission; σi = 1 if the user
                    receives the multicast transmission, and σi = 0 otherwise.
                        The mechanism M is then a pair of functions M(u) = (σ (u), p(u)). It is important
                    to note that both the inputs and the outputs of these functions are distributed throughout
                    the network; that is, each user inputs his ui from his network location, and the outputs
                    σi (u) and pi (u) must be delivered to him at that location. The practicality of deploying
                    the mechanism on the Internet depends on the feasibility of computing the functions
                    σ (u) and p(u) and distributing the results.
                        In our model, it is the agents who are selfish. The routers (represented by tree
                    nodes), links, and other network-infrastructural components are obedient. The cost-
                    sharing algorithm does not know the individual utilities, and so users could lie about
                    them, but once they are reported to the network infrastructure (e.g., by sending them to
                    the nearest router), the algorithms for computing σ (u) and p(u) can be reliably executed
                    by the network. Thus, our interest here is in network complexity, not computational
                    manipulation.
                        Given the selfish nature of agents, the mechanism should be strategyproof, i.e.,
                    revealing ui truthfully should be a dominant strategy. There are two other desirable
                    features one would want in a cost-sharing mechanism: budget balance (the sum of
                    the charges pi covers the total cost of transmitting the content) and efficiency (the
                    total welfare is maximized). The classic result of Laffont and Green, as reviewed in
                    Chapter 9, implies that no strategyproof mechanism with quasilinear utilities can
                    achieve both budget balance and efficiency2 ; we therefore consider two separate mech-
                    anisms, one that achieves budget balance and one that achieves efficiency.
                        To achieve efficiency, we consider a VCG mechanism called marginal cost (MC). Let
                                                                                                         =
                    S̃ denote the largest set that maximizes NW (S) (this is uniquely defined), and let NW
                                          
                    NW (S̃); similarly, NW −i is the maximum value over all S of NW (S − i). Then the MC
                    mechanism chooses the receiver set S̃ and sets payments pi = σi ui − NW      + NW   −i .
                        For budget balance, we choose the Shapley Value (SH) mechanism. The mechanism
                    shares the cost of each link equally among all the agents downstream of that link; an
                    agent i is downstream of a link l if l ∈ T ({i}). To determine which agents receive the
                    transmission, we first start with S = P and compute the charges. We then eliminate any
                    agent for which the charge exceeds the agent’s utility (i.e., pi > ui ) and recursively
                    prune the receiver set until all agents within the set have utilities greater than or
                    equal to their charge. The cross-monotonic nature of these charges (an agent is never
                    charged less after another agent leaves the receiver set) guarantees that the resulting
                    set is well defined, independent of the order in which agents are eliminated. To see
                    why the ordering does not matter, consider the following. We say that an elimination
                    (or pruning) is “legal” if the node to be removed is charged more than its utility; an
                    elimination ordering is “legal” if each individual pruning is legal. We note that, if an
                    agent i is charged more than his utility when the set S of agents remains, then this
                    continues to hold when any subset of S remains (because cross-monotonicity requires

                    2 More precisely, the Laffont–Green result reviewed in Chapter 9 shows that the only strategyproof, welfare-

                      maximizing mechanisms with quasi-linear utilities are the VCG mechanisms, which are known not to be
                      budget-balanced. Myerson and Satterthwaite have shown a more general result about the impossibility of
                      achieving efficient and budget-balanced allocations with rational agents; see Chapter 9 for details.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 5, 2007   14:26




                                                    two examples of damd                                      369

                    that i’s charges are at least as great). This means that the concatenation of any two legal
                    elimination orderings is also a legal elimination ordering (where we ignore duplicate
                    prunings). For example, if (1, 5, 7, 3) and (7, 2, 5, 8) are two legal orderings, then
                    (7, 2, 5, 8, 1, 3) is also legal, as is (1, 5, 7, 3, 2, 8). Thus, if any two subsets S and S 
                    can be arrived at by sequences of legal eliminations, then S ∩ S  can also be arrived at
                    by a sequence of legal eliminations.
                       It is easy to see that both MC and SH are polynomial-time computable by centralized
                    algorithms; so the issue is whether it is hard to implement them in a distributed fashion.
                    Certainly any mechanism can be computed by sending all the valuations to a single
                    node, doing the computation, and then returning the results to each agent. In the worst
                    case, this would require sending (|P |) bits over some number of links, which is
                    clearly not desirable. It turns out that we cannot do substantially better than this for the
                    SH mechanism.

                      Theorem 14.1 Any distributed algorithm, deterministic or randomized, that
                      computes the SH multicast cost-sharing mechanism must send (|P |) bits over
                      linearly many links in the worst case.

                       By contrast, it is possible to compute MC using only two short messages per link
                    and two simple calculations per node. This is done in two phases, the first a bottom-up
                    traversal in which welfare values are computed for each subtree of T (P ) and the second
                    a top-down traversal in which membership bits σi and cost shares pi are computed for
                    each i ∈ P . The algorithms are given in Figures 14.1 and 14.2. In these figures, V (P )
                    denotes the node set of tree T (P ), Ch(α) the set of children of node α, res(α) the set
                    of users resident at node α, uα the sum of utilities of users in res(α), cα the cost of the
                    link connecting α to its parent in T (P ), and T α (P ) the union of the subtree rooted at
                    α and the link connecting α to its parent.
                       The reason that this simple two-phase algorithm suffices is that computing the MC
                    cost share pi does not require a from-scratch computation of NW−i . Rather, it is enough
                    to compute W α for every node α in V (P ) during the computation of NW . Suppose that

                    At node α ∈ V (P )
                        After receiving a message
                                                  Aβ from each child β ∈ Ch(α)
                            W ← u + ( β∈Ch(α) Aβ ) − cα
                               α      α

                            If W α ≥ 0 then
                            {
                                 σi ← 1 for all i ∈ res(α)
                                 Send W α to parent(α)
                            }
                            Else
                            {
                                 σi ← 0 for all i ∈ res(α)
                                 Send 0 to parent(α)
                            }
                                    Figure 14.1. Bottom-up traversal: Computing welfare values.
P1: SBT
9780521872829main      CUNY1061-Nisan          0 521 87282 0       July 5, 2007      14:26




                    370                   distributed algorithmic mechanism design

                    Initialize: Root αo sends W αo to each of its children.
                    For each α ∈ V (P ) − {αo }
                         After receiving message A from parent(α)
                              //Case 1: T α (P ) ∩ T (S̃) = ∅.
                              //Set σi ’s properly at α and propagate non-membership downward.
                              If σi = 0, for all i ∈ res(α), or A < 0, then
                              {
                                   pi ← 0 and σi ← 0 for all i ∈ res(α)
                                   send −1 to β for all β ∈ Ch(α)
                              }
                              //Case 2: T α (P ) ∩ T (S̃) = ∅.
                              //Compute cost shares and propagate minimum welfare value downward.
                              Else
                              {
                                   A ← min(A, W α )
                                   For each i ∈ res(α)
                                        If ui ≤ A, then pi ← 0, else pi ← ui − A
                                   For each β ∈ Ch(α)
                                        Send A to β
                              }

                            Figure 14.2. Top-down traversal: Computing membership bits and cost shares.


                    i ∈ res(β) and that yi (u) is the smallest W α of any node α on the path from β to the
                    root of T (P ). If ui ≤ yi (u), then removing i from the set of potential receivers does not
                    change the set of nodes to which the content is delivered. If ui > yi (u), then removing
                    i from the set of potential receivers does change the set of nodes, and the resulting
                                 − NW
                    difference NW          −i is yi (u). The proofs of these facts are left as an exercise for
                    the reader.

                       Theorem 14.2 MC cost sharing requires exactly two messages per link. There is
                       an algorithm that computes the cost shares by performing one bottom-up traversal
                       of T(P), followed by one top-down traversal.3

                       More information about AMD for cost sharing can be found in Chapter 15.


                                                       14.3 Interdomain Routing

                    We now turn to the problem of interdomain routing. To provide reachability between
                    hosts, the various ASes that make up the Internet must be interconnected. However, as

                    3 The algorithm is provably optimal with respect to the number of messages sent but is not known to be optimal

                      with respect to the maximum size of a message. However, the maximum size of a message is polynomial in
                      maxl size(c(l)) and maxi size(ui ) and polylogarithmic in |P | and |N |, and the two local computations required
                      at each node are fast and space-efficient.
P1: SBT
9780521872829main       CUNY1061-Nisan          0 521 87282 0      July 5, 2007      14:26




                                                           interdomain routing                                                  371

                    we noted earlier, the ASes are economically independent entities (indeed, frequently
                    competitors), and there is no trusted center to which they are all accountable that could
                    assign interdomain routes. Thus, the ASes themselves must compute the routes in
                    a distributed fashion. The route computation scheme must handle three problematic
                    aspects of interdomain routing: (i) there is a large number of ASes; (ii) different ASes
                    have different criteria for choosing one route over another, and these criteria may
                    conflict; and (iii) the collection of ASes and the links between them change frequently.
                    All of these factors make DAMD a highly suitable approach to interdomain routing.
                       We can formally define the interdomain-routing problem as follows. The network
                    topology is defined in terms of the AS graph G = (N, L), where each node in N =
                    {1, . . . , n} corresponds to an AS in the Internet, and each link in L corresponds to
                    a direct connection between a pair of neighboring ASes. Because routing protocols
                    typically compute routes for each destination independently, we can choose a particular
                    destination AS d and let P i be the set of all loop-free paths from i to d in G that are
                    not removed from consideration.4 An interdomain-routing protocol allocates to each
                    source node i ∈ N a route Ri ∈ P i .
                       We now describe this problem in greater detail, first from the networking perspective
                    and then from the mechanism-design perspective.


                                                      14.3.1 Networking Perspective
                    From a networking or protocol-design point of view, any wide-area routing protocol
                    must fulfill, to some extent, the following requirements:
                    r For reasons of trust, scale, and robustness, the routing protocol must be distributed,
                      carried out by the ASes themselves.
                    r In order to reduce routing state, the routing protocol must use destination-based for-
                      warding; i.e., all routing decisions must be based solely on a packet’s destination.
                      Each AS has a single next hop for the destination d, and the resulting route allocation
                      Td = {R1 , . . . , Rn } forms a confluent tree to the destination d.
                    r The routing protocol should be adaptive, adjusting to the current network topology
                      without relying on any a priori topology information.
                    r The routing protocol should be time-efficient, communication-efficient (in its use of
                      communication between the ASes), and space-efficient (in its use of the storage space
                      that each individual AS needs in order to participate in the protocol).

                    These requirements are satisfied by each of the common routing-protocol designs –
                    namely distance-vector, link-state, and path-vector – although these designs differ in
                    their space requirements. However, interdomain routing has one additional require-
                    ment:
                    r The routing protocol must produce loop-free routes even while individual ASes make
                      autonomous decisions about which routes are preferable.

                    4 A path from i to d could be “removed from consideration” because it is filtered by i or one of i’s neighbors or

                      because of link or node failures.
P1: SBT
9780521872829main      CUNY1061-Nisan         0 521 87282 0       July 5, 2007    14:26




                    372                 distributed algorithmic mechanism design

                                                                  Update messages between neighboring ASes




                                                         BGP router               BGP router             BGP router
                                                                                                          Via update
                                                                                                          messages
                                    Via update
                                    messages                  Import routes      Wait for updates      Export route
                                                              from neighbors                           to neighbors
                                Announce                                         If best route
                                destination                                      is unchanged
                                                         Store routes
                               Initialize at             in routing table         Choose best
                               destination AS                                     route based          If best route
                                                                                  on policy            changes


                                                                                                 BGP Router at one AS

                                    Figure 14.3. Route computation using a path-vector protocol.



                       Of the common routing-protocol designs, only path-vector satisfies this requirement.
                    As a result, the current standard protocol for Internet interdomain routing, the Border
                    Gateway Protocol (BGP), is a path-vector protocol. To see why path-vector is a suitable
                    design choice, we describe BGP in more detail.
                       BGP allows adjacent nodes to exchange information through update messages that
                    announce newly chosen routes (see illustration in Figure 14.3); a route announcement
                    contains the entire path to the destination (the list of ASes in the path). A path-
                    vector protocol (like most other routing protocols) computes routes to every destination
                    AS independently; so we can focus on routes to a single destination d. The route-
                    computation process is initialized when d announces itself to its neighbors by sending
                    update messages. The rest of the routing tree to d is built recursively, as knowledge of
                    how to reach d propagates through the network via subsequent update messages. We
                    assume that the network is asynchronous, meaning that the arrival of update messages
                    along selective links can be delayed.
                       The routing process at a particular node i has three stages that are iteratively applied:

                      (i) Importing routes: Routes to d are received via update messages from its neighbors.
                          Node i has an import policy that specifies which of the routes it is willing to consider.
                          All such importable routes are stored in an internal routing table. At any given time,
                          i’s internal routing table contains the latest importable routes.
                     (ii) Route selection: If there is more than one route to d in the routing table (i.e., more than
                          one of i’s neighbors has announced an importable route to d), node i must choose one
                          (expressing a local preference over routes).
                    (iii) Exporting routes: Whenever there is a change to i’s best route, it announces the newly
                          selected route to some or all of its neighbors using update messages. Node i has
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007       14:26




                                                    interdomain routing                                    373


                                                            1            2



                                                                 d

                    Figure 14.4. When AS 1 prefers route 12d to 1d, and AS 2 prefers route 21d to 2d, BGP (or
                    any other path-vector protocol) can oscillate indefinitely.


                        an export policy that determines, for each neighbor j , which routes it is willing to
                        announce to j at any given time.

                        AS autonomy is expressed through the freedom each AS has in choosing its routes,
                    its import policy, and its export policy. These choices are based on local policy con-
                    siderations and need not be coordinated with any other AS. The inclusion of the entire
                    path in route announcements allows ASes to avoid routes with loops even while making
                    otherwise arbitrary policy choices. Link-state or distance-vector routing protocols can
                    avoid loops only if all ASes use the same criterion to choose routes and thus do not
                    support autonomy.
                        One design requirement not explicitly listed here is convergence. Clearly the routing
                    protocol should eventually enter a stable state in which every node prefers its currently
                    chosen route to all others in its routing table, and all routing tables reflect the current
                    route choices of its neighbors. Moreover, we would like the protocol to be robust,
                    converging for every AS graph obtained by removing any set of nodes and links from
                    the original instance.
                        Unfortunately, while the path-vector form of routing prevents loops, it does not
                    ensure convergence; the routing announcements can enter a persistent oscillatory state.
                    Consider the simple example depicted in Figure 14.4. Both nodes 1 and 2 would rather
                    send traffic through the other source node than send traffic directly to the destination.
                    Let us now simulate the execution of a path-vector protocol in the worst-case scenario:
                    The computation is initialized when d announces itself to its two neighbors, nodes
                    1 and 2. At this point in time, these direct paths are the only routes available to d.
                    Hence, 1 and 2 will choose the routes 1d and 2d, respectively, and inform each other,
                    via update messages, of their selected routes. Upon receipt of these update messages,
                    nodes 1 and 2 will change their selected routes to, respectively, 12d and 21d. However,
                    now that none of the direct routes is being used, the indirect routes are no longer viable;
                    so 1 and 2 are forced to return to their former routes 1d and 2d, and the oscillation
                    continues indefinitely. Note that, if the network had started with node 1’s choosing and
                    announcing 1d (having not yet seen an announcement of route 2d), and then node 2
                    had chosen 21d (having seen route 1d announced before it chose and announced its
                    own direct route 2d), then no further changes would occur, and the network would be
                    in a stable configuration; thus, convergence and oscillations can depend on timing.
                        A large body of networking research has addressed the problem of providing suffi-
                    cient conditions on routing policies for the convergence of path-vector protocols. There
                    is an inherent trade-off between the desired autonomy at the local level and robustness
                    (in the sense defined above) at the global level. However, there is a known sufficient
                    condition on policies, called no dispute wheel, that guarantees robust convergence
P1: SBT
9780521872829main       CUNY1061-Nisan          0 521 87282 0       July 5, 2007     14:26




                    374                   distributed algorithmic mechanism design

                    while allowing fairly expressive local routing policies. Any network instance on which
                    a path-vector protocol might oscillate contains a dispute wheel and, more importantly,
                    the absence of a dispute wheel means that the instance and every subinstance of it have
                    unique stable route allocations to which the routing protocol converges, i.e., no dispute
                    wheel implies robustness. The following definition provides an equivalent sufficient
                    condition:

                       Definition 14.3         Define two relations on permitted routes:
                        (i) Let R1      1 R2 iff R1 is a subpath of R2 that ends at d.

                                        2 R2 iff ∃ i ∈ N : R1 , R2 ∈ P , and i prefers R1 over R2 .
                                                                      i
                       (ii) Let R1
                       Let = ( 1 ∪ 2 )∗ be the transitive closure of                       1,     2 . Note that   is inherently
                       reflexive and transitive.

                       An interdomain-routing instance has no dispute wheel iff R1 R2 and R2 R1
                    together imply that R1 , R2 start at the same node. (Informally, this is antisymmetry of
                       except that ties are allowed in valuations.)
                       Let us revisit the example in Figure 14.4. Recall that, on this instance, path-vector
                    protocols may oscillate forever. This anomaly is manifested by the following dispute
                    wheel:

                                                       1d     1 21d      2 2d      1 12d        2 1d.

                      So far, our discussion of interdomain routing has focused on traditional networking
                    concerns. We now consider the problem from a mechanism-design perspective.


                                                14.3.2 Mechanism-Design Perspective
                    The policy autonomy in BGP, which was previously allowed to be an arbitrary choice,
                    can be seen as expressing a preference that an AS is selfishly trying to satisfy. To do so,
                    we let each source node i have a private valuation function vi : S i → R≥0 , where S i
                    is the set of all simple (noncyclic) routes from i to d in the complete graph we get by
                    adding links to G.5 The valuation function vi specifies the “monetary value” of each
                    route to source node i. We assume that vi (∅) = 0 and that, for all pairs of routes R1
                    and R2 through different neighboring nodes, vi (R1 ) = vi (R2 ).6 The routing policy of
                    each node i is thus captured by vi .
                        While each individual AS is trying to optimize its individual welfare, society as a
                    whole has an interest in reaching a globally desirable outcome. While there are many
                    goals one could choose, we shall focus here on social-welfare maximization. A route

                    5 Because we do not assume that nodes know the network topology, we cannot assume that they can distinguish

                      valid routes from invalid ones. Thus, the valuation functions are defined over the complete graph to model the
                      possibility of nodes’ announcing nonexistent routes.
                    6 This assumption is consistent with current interdomain routing: Because at most one route to each destination

                      can be installed in a router’s forwarding table, nodes have some way to break ties, e.g., based on the next hop’s
                      IP address; so, valuations can be adjusted accordingly to match this. However, because only one route per
                      neighbor is considered at a time, ties in valuation are permitted for routes through the same neighboring node.
P1: SBT
9780521872829main      CUNY1061-Nisan           0 521 87282 0         July 5, 2007       14:26




                                                             interdomain routing                                                       375

                    allocation Td maximizes the social welfare if
                                                                                          
                                                                                          n
                                                     Td = argmaxT ={R1 ,...,Rn }                vi (Ri ).
                                                                                          i=1

                       If we view a routing protocol from a mechanism-design perspective, it should satisfy
                    the following two requirements:
                    r If implemented honestly, the protocol should maximize the social welfare.
                    r The protocol should be incentive-compatible, in that no AS is motivated to deviate from
                      the actions it is asked to perform.

                        The precise definition of incentive compatibility needed in this setting depends on
                    the nature of the solution concept (or economic equilibrium). We shall now discuss in
                    detail the solution concept that we adopt for interdomain-routing mechanisms. Recall
                    from Section 14.1 that DAMD poses inherently different strategic challenges from
                    AMD, because, in the absence of a trusted center, the computation is performed by
                    the strategic agents themselves. This allows the computational nodes to manipulate the
                    mechanism strategically in ways other than “lying” about their private types. They can,
                    for instance, alter the computation to their own benefit or refuse to pass messages if
                    it suits their needs. In such a scenario, aiming for strategyproofness might be futile,
                    because it is unlikely that there is a single computational behavior that is optimal no
                    matter what the other agents do.
                        A more suitable solution concept is ex-post Nash equilibrium. The need to settle for
                    ex-post Nash, rather than strategyproofness, can be viewed as the cost of distributing
                    mechanism computation among the agents. We shall now formally define ex-post Nash
                    in a distributed setting: Consider a computational network with n nodes and a set of
                    possible outcomes O. Each node i has a private type θi ∈ i and a utility function
                    ui : O × i → R.

                      Definition 14.4 A distributed mechanism d M is a 3-tuple d M = ( , g , s M ),
                      where  = (1 , . . . , n ) is the feasible strategy space of the nodes, g :  → O is
                      the outcome function computed by the mechanism, and s M = (s1M , . . . , snM ) ∈ 
                      is the prescribed strategy.

                       For every node i, siM ∈ i can be thought of as the algorithm that the mechanism
                    designer intends i to execute. siM is parameterized by the private type θi of the node
                    i, with siM (θi ) specifying which actions node i should perform in every state of the
                    mechanism and network, given that its type is θi .

                      Definition 14.5 A strategy profile s ∗ ∈  is an ex-post Nash equilibrium of a
                      distributed mechanism d M = ( , g , s M ), if

                            ui (g(s1∗ (θ1 ), . . . , sn∗ (θn )), θi ) ≥ ui (g(s1∗ (θ1 ), . . . , si (θi ), . . . , sn∗ (θn )), θi )

                      for every node i, for every possible strategy si ∈ i , for every possible θi , and for
                      all possible private types θ1 , . . . , θi−1 , θi+1 , . . . , θn of the other nodes.
P1: SBT
9780521872829main      CUNY1061-Nisan          0 521 87282 0      July 5, 2007     14:26




                    376                  distributed algorithmic mechanism design

                        Although weaker than a dominant-strategy equilibrium, ex-post Nash equilibrium
                    is a fairly strong solution concept; it does not require strategic agents to have any
                    knowledge of or to make any assumptions about the private types of other agents.
                    Contrast this with the standard Nash-equilibrium concept, in which agents are assumed
                    to know the private types of other agents; in the interdomain-routing context, this would
                    mean that ASes are assumed to know the local routing policies of other ASes, which
                    is certainly unrealistic.
                        The ex-post Nash equilibrium solution concept is susceptible to collusion.7 That is,
                    while it is true that unilateral deviation by an AS from the prescribed strategy profile
                    cannot benefit it, coordinated deviation by several ASes might prove to be beneficial
                    to some. Therefore, if at all possible, we would like our mechanisms to ensure that no
                    deviation by a group of ASes from the prescribed strategy profile is worthwhile. To
                    achieve this, we introduce collusion-proof ex-post Nash equilibria. In a collusion-proof
                    ex-post Nash equilibrium, no deviation by a group of agents can strictly improve the
                    outcome of even a single agent in that group without strictly harming another.

                               14.3.3 A DAMD Approach: Combining the Two Perspectives
                    To achieve incentive-compatible interdomain routing, we must design a protocol that
                    makes sense from both the networking and the mechanism-design perspectives. The
                    networking requirements point to a path-vector framework combined with a class of
                    routing preferences that guarantees convergence. Mechanism design requires that we
                    incent agents to implement this routing protocol faithfully. Incentive compatibility is
                    often achieved through payments; however, below we show that, under a reasonable set
                    of assumptions about routing policies, one can achieve collusion-proof ex-post Nash
                    equilibrium without payments simply by executing BGP.

                              14.3.3.1 Commercial Internet Routing and the Gao–Rexford Model
                    There are two types of business relationships that characterize most AS intercon-
                    nections: customer-provider and peering. Customer ASes pay their provider ASes for
                    connectivity, and peers are AS pairs that find it mutually advantageous to exchange traf-
                    fic for free. One advantage of peering is that the two peers need not pay their respective
                    providers to exchange traffic directly. An AS can be in many different relationships si-
                    multaneously: It can be a customer of one or more ASes, a provider to others, and a peer
                    to yet others. These agreements are assumed to be relatively long-term contracts that
                    are formed because of various external factors, e.g., traffic patterns and network sizes.
                        These business relationships naturally induce the following constraints on routing
                    policies, known as the Gao–Rexford constraints:
                    No customer-provider cycles: Let GCP be the digraph with the same set of nodes as
                      G and with a directed edge from every customer to its provider. The Gao–Rexford
                      constraints require that there be no directed cycles in this graph. This requirement is
                      a natural economic assumption, because a cycle in GCP implies that at least one AS
                      is (indirectly) its own provider.

                    7 The Nash equilibrium and dominant-strategy equilibrium concepts are also susceptible to collusion.
P1: SBT
9780521872829main       CUNY1061-Nisan            0 521 87282 0         July 5, 2007        14:26




                                                               interdomain routing                                                       377


                                                                                 4     v4 (432d) = 1 + α
                                                                                       v4 (431d) = 0


                                                                                 3     v3 (31d) = 1
                                                                                       v3 (32d) = 0

                                                     v1 (1d) = 1          1             2    v2 (2d) = 1
                                                     v1 (132d) = 0                           v2 (231d) = 0

                                                                                 d

                    Figure 14.5. A routing instance that satisfies the Gao–Rexford constraints on which every
                    path-vector protocol converges to a route allocation that is arbitrarily far from optimal.


                    Prefer customers over peers and peers over providers: A customer route is a route
                      in which the next-hop AS is a customer. Provider and peer routes are defined sim-
                      ilarly. Because, typically, customers pay providers for service, and peers exchange
                      service for free, the Gao–Rexford constraints require that nodes always prefer (i.e.,
                      assign a higher value to) customer routes over peer routes, which are in turn preferred
                      over provider routes.
                    Provide transit services only to customers: Transit service is carrying packets that
                      originate and terminate at hosts outside the node. ASes are paid to carry customer
                      packets but are not paid to carry peer or provider traffic. The Gao–Rexford con-
                      straints require that ASes not carry transit traffic between their providers and peers.
                      Therefore, ASes should announce only customer routes to their providers and peers
                      but should announce all of their routes to their customers.
                        These constraints ensure robustness without requiring coordination between ASes.
                    In fact, if all ASes obey the Gao–Rexford constraints, then their valuations cannot
                    induce a dispute wheel.
                        The Gao–Rexford constraints ensure robust convergence, but in general they do not
                    guarantee that BGP converges to the social-welfare-maximizing route allocation. To see
                    this, consider the example in Figure 14.5. Assume that d is a customer of 1 and 2, that 1
                    and 2 are customers of 3, that 3 is a customer of 4, and that α > 0. Observe that this AS
                    graph satisfies all the Gao–Rexford constraints. The unique stable route allocation (to
                    1, . . . , 4, respectively) is {1d, 2d, 31d, 431d}. However, the optimal route allocation is
                    {1d, 2d, 32d, 432d}. This allocation will never be chosen by local decisions, because
                    node 3 would much prefer routing through node 1, a route that is always available for
                    it to choose. Therefore, because the value of α can be arbitrarily high, this implies that
                    the route allocation computed by a path-vector protocol could be arbitrarily far from
                    the welfare-maximizing route allocation.
                        This problem can be overcome by imposing the policy-consistency property.

                       Definition 14.6 Policy consistency holds iff, for every two adjacent nodes i, j ∈
                       N, and every two routes {Q, R} ⊆ P j such that {(i, j )Q, (i, j )R} ⊆ P i 8 (in

                    8 (i, j )Q and (i, j )R are the routes from i to d that have (i, j ) as a first link and then follow Q and R, respectively.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    July 5, 2007   14:26




                    378               distributed algorithmic mechanism design

                      particular, node i is not on Q or R),

                                        if vj (Q) ≥ vj (R), then vi ((i, j )Q) ≥ vi ((i, j )R).

                        Informally, policy consistency holds if, for every two neighboring nodes i, j , such
                    that j is i’s next-hop node on two routes, we have that, if j weakly prefers one route
                    over another, then so must i. The policy-consistency property holds in the two most
                    well studied special cases of interdomain routing. The first is the case in which the
                    valuation of a route is solely a function of the route’s next hop. (These are called
                    “next-hop policies.”) The second is the case in which there is some metric function that
                    assigns a “length” to every link, and every valuation function prefers “shorter” routes
                    (i.e., those with smaller total lengths in this metric). (These are called “metric-based
                    policies.”)
                        We are now ready to state, and prove, the following theorem.

                      Theorem 14.7 If the Gao–Rexford constraints and policy consistency hold, then
                      BGP converges to the social-welfare-maximizing route allocation and is incentive-
                      compatible in collusion-proof ex-post Nash equilibrium (without any monetary
                      transfer).


                    PROOF We will actually prove a result that is stronger in two senses: First, we shall
                    prove our result in the more general setting in which the valuation functions do not
                    induce a dispute wheel, and policy consistency holds. Second, we shall prove that BGP
                    actually converges to a solution (an allocation of routes) in which every AS gets its
                    most desired route to the destination. That is, every AS will be assigned a route that
                    maximizes its valuation function. We call this kind of route allocation a locally optimal
                    solution. Observe that any locally optimal solution is also globally optimal in that it
                    maximizes the total social welfare. Moreover, locally optimal solutions are deviation-
                    proof in that there is no deviation by a group of agents that can strictly improve the
                    outcome of even a single agent. This is far stronger than collusion-proof ex-post Nash
                    equilibrium, which only requires that no deviation by a group of agents can strictly
                    improve the outcome of a single agent in the group without strictly harming another
                    agent in the group.
                       Because the Gao–Rexford constraints imply that there is no dispute wheel, we are
                    assured (by the result mentioned in Section 14.3.1) that BGP will converge to a unique
                    stable solution. We denote this solution by Td = {S1 , . . . , Sn }, where Si is the route
                    allocated to node i.

                      Lemma 14.8 If the valuation functions do not induce a dispute wheel, and
                      policy consistency holds, then BGP converges to a unique stable, locally optimal
                      route allocation Td .

                      proof Consider a node m ∈ N. Let R = uk uk−1 . . . ui . . . u0 be some loop-free
                      route in P uk , such that uk = m and u0 = d. By induction, we show for each ui ∈ R
                      that Si , the solution’s route for node ui in Td , is at least as good as Ri = ui . . . u0 .
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0     July 5, 2007     14:26




                                               conclusion and open problems                                     379

                      If i = m, then Sm is at least as good as R; because R and m were chosen arbitrarily,
                      this establishes the local optimality of Td .
                      Base case. i = 0. The induction hypothesis is trivially true, because the only route
                      is the empty one.
                      Induction step. Assume that the induction hypothesis is true for ui−1 , i.e.,
                                                     vui−1 (Si−1 ) ≥ vui−1 (Ri−1 ).                        (14.1)
                      Note that ui does not lie on Ri−1 , because R is loop-free.
                      Case I. Assume that ui ∈       / Si−1 . Then extend Si−1 and Ri−1 along the edge
                      (ui , ui−1 ). (ui , ui−1 )Si−1 ∈ P ui ; thus, from (14.1) and policy consistency, we
                      have
                                                   vui ((ui , ui−1 )Si−1 ) ≥ vui (Ri ).                    (14.2)
                      Td is stable; so, Si is at least as good as any other route at ui ; in particular,
                                                   vui (Si ) ≥ vui ((ui , ui−1 )Si−1 ).                    (14.3)
                      Combining (14.2) and (14.3) gives
                                                           vui (Si ) ≥ vui (Ri ),
                      which is the induction statement for ui .
                      Case II. Assume that ui ∈ Si−1 . We cannot use the policy-consistency argument
                      as in Case I, because extending Si−1 to ui creates a loop. This implies that
                      ui−1 ∈ / Si . Suppose that the induction statement is not true for i, i.e., that vui (Ri ) >
                      vui (Si ). Then Ri 2 Si . Because ui−1 ∈  / Si but ui ∈ Si−1 , it must be that Si 1 Si−1 .
                      From the induction hypothesis, Si−1 2 Ri−1 , and, because Ri = (ui , ui−1 )Ri−1 ,
                      Ri−1 1 Ri . Therefore, we have a cycle in the relation ; in particular, we can
                      say that Ri Ri−1 and Ri−1 Ri , but these routes do not start at the same node.
                      This violates the no-dispute-wheel property and shows that the assumption that
                      vui (Ri ) > vui (Si ) leads to a contradiction. Therefore, vui (Ri ) ≤ vui (Si ), which is
                      the induction statement for ui . (Recall that there are no ties in valuations.)

                      Remark 14.9 Lemma 14.8 holds for every subinstance of the AS graph, be-
                      cause both the Gao–Rexford constraints and policy consistency hold for every
                      subinstance.

                      Remark 14.10 No dispute wheel implies a unique collusion-proof ex-post Nash
                      solution to which BGP converges. Hence, we are not concerned with the standard
                      problem that arises when multiple equilibria exist, namely whether nodes select
                      the same equilibrium.


                                          14.4 Conclusion and Open Problems

                    In this chapter, we have reviewed the work that has been done on distributed algorithmic
                    mechanism design, in which the presence of strategic computational agents introduces
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:26




                    380               distributed algorithmic mechanism design

                    new incentive and computational challenges for distributed computing. In particular,
                    we have presented in detail some of the known results about DAMD for interdomain
                    routing, which is the best motivated and most extensively studied problem in the area.
                    There are at least two interesting directions for further research.
                       First, there is the general question of which other problems in networked computa-
                    tion are amenable to the approaches explored in this chapter. Several good candidates
                    have been proposed, i.e., web caching, peer-to-peer file sharing, overlay-network con-
                    struction, and distributed task allocation. Although both distributed algorithms and
                    incentive compatibility have been considered in the literature about these problems,
                    the results have not been pulled together into a coherent DAMD theory. The construc-
                    tion of such a theory remains a worthy goal.
                       Second, there are many questions about interdomain routing that have not been fully
                    answered. There is still no complete characterization of the conditions under which BGP
                    converges robustly. (“No dispute wheel” is sufficient but not known to be necessary.)
                    Similarly, the conditions under which collusion-proof ex-post Nash equilibrium is
                    reached simply by executing BGP have not been characterized completely. (Again, the
                    Gao–Rexford and policy-consistency conditions presented in this chapter are sufficient
                    but not known to be necessary.) In fact, necessary and sufficient conditions on AS
                    graphs and routing policies have not yet been obtained for ex-post Nash equilibrium,
                    even if we ignore collusion and allow payments. Both policy consistency and local
                    optimality play an essential role in the main result presented in this chapter, and little is
                    known about what can be obtained without them. In general, the network complexity
                    of BGP is open, even in cases when convergence is assured.


                                                           14.5 Notes

                    Given the distributed and autonomous nature of Internet users, it is no surprise that the
                    networking and distributed-systems literature provides some of the earliest applications
                    of game theory and mechanism design to computer-science problems. These themes
                    were first explored in an early series of papers from Columbia University, e.g., Ferguson
                    (1989), Hsiao and Lazar (1988), Kurose et al. (1985), Kurose and Simha (1989),
                    Mazumdar and Douligeris (1992), and Yemini (1981), which were followed by contri-
                    butions from Miller and Drexler (1988a, 1988b), Sanders (1986, 1988a, 1988b), and
                    others (Kelly, 1997; Kelly et al., 1998; La and Anantharam 1997; Murphy and Murphy,
                    1994; Mackie-Mason and Varian, 1995; Shenker, 1990, 1995). Because networking
                    problems are inherently distributed, and network protocols must have reasonable net-
                    work complexity, these papers were actually early forerunners of DAMD.
                       Nisan and Ronen were the first to combine algorithmic and economic concerns in
                    a new area of study for which they coined the term “algorithmic mechanism design,”
                    and this book is largely an outgrowth of their seminal paper Nisan and Ronen (2001).
                    The extension of AMD to DAMD was first explored in Feigenbaum et al. (2001),
                    which considered the multicast cost-sharing problem described in Section 14.2 and
                    articulated the notion of network complexity; the DAMD agenda was more broadly
                    described soon thereafter in Feigenbaum and Shenker (2002). Subsequent work on
                    DAMD for multicast cost sharing can be found in, e.g., Archer et al. (2004), Adler and
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 5, 2007   14:26




                                                            bibliography                                         381

                    Rubenstein (2002), Fiat et al. (2002), and Feigenbaum et al. (2003). In particular, a
                    generalization of Theorem 14.1 is proven in Feigenbaum et al. (2003).
                       Distributed VCG computation and the importance of ex-post Nash equilibria in
                    DAMD were first presented by Parkes and Shneidman (2004) and developed further
                    by Petcu et al. (2006).
                       The BGP specification can be found in Rekhter et al. (2006). The fact that BGP
                    may not converge if there are no constraints on the AS graph or the domains’ routing
                    policies was first observed by Varadhan et al. (2000). The example of BGP divergence
                    in Figure 14.4 and the proof that “no dispute wheel” guarantees robust convergence
                    are presented in Griffin et al. (2002). Abstract properties of path-vector protocols are
                    developed in, e.g., Griffin et al. (1999, 2003), Sobrinho (2005). The Gao–Rexford
                    conditions and their implications were first studied in Gao and Rexford (2001) and
                    further developed in, e.g., Gao et al. (2001). Partial results on the network complexity
                    of BGP can be found in, e.g., Karloff (2004).
                       DAMD was first applied to interdomain routing by Feigenbaum et al. (2005b), who
                    devised a BGP-based algorithm for lowest-cost routing. Computational manipulation
                    by ASes and ex-post Nash equilibrium in BGP-based, lowest-cost routing was first
                    studied by Shneidman and Parkes (2004). Hardness results for more general classes of
                    routing policies can be found in Feigenbaum et al. (2005a, 2006b). A positive result
                    about BGP-based, incentive-compatible routing under the Gao–Rexford and policy-
                    consistency conditions is given in Feigenbaum et al. (2006a) and is the direct precursor
                    of the result presented in Section 14.3. Sobrinho was the first to study policy constraints
                    that guarantee optimality, both global and local; a result that is similar to (but weaker
                    than) Lemma 14.8 is presented in Sobrinho (2005).
                       For basic background on Internet routing, see Kurose and Ross (2005), Peterson and
                    Davie (2003), or other networking textbooks.


                                                        Acknowledgments

                    We thank Vijay Ramachandran and Rahul Sami for many helpful discussions of
                    interdomain routing. The work of the first author was supported in part by ONR
                    grants N00014-01-1-0795 and N00014-04-1-0725, NSF grant 0428422, HSARPA
                    grant ARO-1756303, and US–Israeli BSF grant 2002065. The work of the second
                    author was supported in part by US-Israeli BSF grant 2002065 and by Israeli Science
                    Foundation grant 169/03. The work of the third author was supported in part by NSF
                    grant 0428422.

                                                            Bibliography
                    A. Archer, J. Feigenbaum, A. Krishnamurthy, R. Sami, and S. Shenker. Approximation and collusion
                       in multicast cost sharing. Games Econ. Behav., 47(1):36–71, 2004.
                    M. Adler and D. Rubenstein. Pricing multicasting in more practical network models. In 13th Symp.
                       on Discrete Algorithms, pp. 981–990, ACM/SIAM, New York/Philadelphia, 2002.
                    J. Feigenbaum, D.R. Karger, V.S. Mirrokni, and R. Sami. Subjective-cost policy routing. In Xiaotie
                       Deng and Yinyu Ye, editors, First Workshop on Internet and Network Economics, LNCS 3828:174–
                       183, Springer, Berlin, 2005a.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 5, 2007   14:26




                    382                distributed algorithmic mechanism design

                    J. Feigenbaum, A. Krishnamurthy, R. Sami, and S. Shenker. Hardness results for multicast cost
                       sharing. Theor. Comput. Sci., 304(1–3):215–236, 2003.
                    J. Feigenbaum, C.H. Papadimitriou, and S. Shenker. Sharing the cost of multicast transmissions. J.
                       Comput. Syst. Sci., 63(1):21–41, 2001.
                    J. Feigenbaum, C.H. Papadimitriou, R. Sami, and S. Shenker. A BGP-based mechanism for lowest-
                       cost routing. Distr. Comput., 18(1):61–72, 2005b.
                    J. Feigenbaum, V. Ramachandran, and M. Schapira. Incentive-compatible interdomain routing. In 7th
                       Conference on Electronic Commerce, pp. 130–139, ACM, New York, 2006a.
                    J. Feigenbaum, R. Sami, and S. Shenker. Mechanism design for policy routing. Distr. Comp.,
                       18(4):293–305, 2006b.
                    J. Feigenbaum and S. Shenker. Distributed algorithmic mechanism design: Recent results and future
                       directions. In 6th Intl. Workshop on Discrete Algorithms and Methods for Mobile Computing and
                       Communications, pp. 1–13, ACM, New York, 2002.
                    A. Fiat, A.V. Goldberg, J.D. Hartline, and A.R. Karlin. Competitive generalized auctions. In 34th
                       Symposium on Theory of Computing, pp. 72–81, ACM, New York, 2002.
                    D.F. Ferguson. The Application of Microeconomics to the Design of Resource Allocation and Control
                       Algorithms. Ph.D. Thesis, Columbia University, 1989.
                    L. Gao, T.G. Griffin, and J. Rexford. Inherently safe backup routing with BGP. In 20th INFOCOM,
                       pp. 547–556, IEEE, Pistacaway, 2001.
                    L. Gao and J. Rexford. Stable Internet routing without global coordination. IEEE/ACM Trans. Net-
                       working, 9(6):681–692, 2001.
                    T.G. Griffin, A.D. Jaggard, and V. Ramachandran. Design principles of policy languages for path
                       vector protocols. In SIGCOMM ’03: Proc. 2003 Conf. Applications, Technologies, Architectures,
                       and Protocols for Computer Communications, pp. 61–72, ACM, New York, 2003.
                    T.G. Griffin, F.B. Shepherd, and G. Wilfong. Policy disputes in path-vector protocols. In 7th Intl.
                       Conf. on Network Protocols, pp. 21–30, IEEE Computer Society, Los Alamitos, 1999.
                    T.G. Griffin, F.B. Shepherd, and G. Wilfong. The stable paths problem and interdomain routing.
                       IEEE/ACM Trans. Networking, 10(2):232–243, April 2002.
                    M.-T. Hsiao and A.A. Lazar. A game theoretic approach to decentralized flow control of markovian
                       queueing networks. In Pierre-Jacques Courtois and Guy Latouche, editors, Performance 87’, Proc.
                       12th IFIP WG 7.3 Intl. Symp. Comp. Performance Modelling, Measurement, and Evaluation, pp.
                       55–73, North-Holland, Amsterdam, 1988.
                    H. Karloff. On the convergence time of a path-vector protocol. In Proc. 15th Symp. on Discrete
                       Algorithms, pp. 605–614, ACM/SIAM, New York/Philadelphia, 2004.
                    F.P. Kelly. Charging and rate control for elastic traffic. Euro. Trans. Telecommuncations, 8:33–37,
                       1997.
                    F.P. Kelly, A. Maulloo, and D. Tan. Rate control in communication networks: Shadow prices, pro-
                       portional fairness, and stability. J. Oper. Res. Soc., 49(3):237–252, 1998.
                    J.F. Kurose and K.W. Ross. Computer Networking: A Top Down Approach Featuring the Internet.
                       Addison-Wesley, 2005.
                    J.F. Kurose and R. Simha. A microeconomic approach to optimal resource allocation in distributed
                       computer systems. IEEE Trans. Comp., 38(5):705–717, 1989.
                    J.F. Kurose, M. Schwartz, and Y. Yemini. A microeconomic approach to decentralized optimization
                       of channel access policies in multiaccess networks. In 5th Intl. Conf. on Distr. Comp. Sys., pp.
                       70–77, IEEE Computer Society, Los Alamitos, 1985.
                    R.J. La and V. Anantharam. Optimal routing control: Game theoretic approach. In 36th Conf. on
                       Decision and Control, pp. 2910–2915, IEEE, Piscataway, 1997.
                    J.K. Mackie-Mason and H. Varian. Pricing the Internet. In Brian Kahin and James Keller, editors,
                       Public Access to the Internet, pp. 269–314, MIT Press, Cambridge, 1995.
P1: SBT
9780521872829main       CUNY1061-Nisan      0 521 87282 0    July 5, 2007    14:26




                                                            bibliography                                          383

                    R.R. Mazumdar and C. Douligeris. A game theoretic approach to flow control in an integrated
                       environment. J. Franklin Inst., 329(2):383–402, 1992.
                    M.S. Miller and K.E. Drexler. Incentive engineering: For computational resource management.
                       In Bernardo A. Huberman, editor, The Ecology of Computation, pp. 231–266. North-Holland,
                       Amsterdam, 1988a.
                    M.S. Miller and K.E. Drexler. Markets and computation: Agoric open systems. In Bernardo A.
                       Huberman, editor, The Ecology of Computation, pp. 133–176. North-Holland, Amsterdam,
                       1988b.
                    J. Murphy and L. Murphy. Bandwidth allocation by pricing in ATM networks. In Broadband Com-
                       munications II: Proc. 2nd Intl. Conf., pp. 333–351, Elsevier, Amsterdam, 1994.
                    N. Nisan and A. Ronen. Algorithmic mechanism design. Games Econ. Behav., 35(1):166–196, 2001.
                    D.C. Parkes and J. Shneidman. Distributed implementations of Vickrey-Clarke-Groves mechanism.
                       In 3rd Intl. Joint Conf. on Autonomous Systems and Multiagent Systems, pp. 261–268, IEEE
                       Computer Society, Los Alamitos, 2004.
                    A. Petcu, B. Faltings, and D.C. Parkes. MDPOP: Faithful distributed implementation of efficient
                       social choice problems. In Proc. 5th Intl. Joint Conf. Autonomous Agents and Multiagent Systems,
                       ACM Press, New York, NY, 2006.
                    L.L. Peterson and B.S. Davie. Computer Networks: A Systems Approach. Morgan Kaufmann, 2003.
                    Y. Rekhter, T. Li, and S. Hares. A Border Gateway Protocol 4 (BGP-4). RFC 4271, January 2006.
                    B.A. Sanders. An incentive compatible flow control algorithm for fair rate allocation in com-
                       puter/communication networks. In 6th Intl. Conf. on Distr. Comp. Syst., pp. 314–320, IEEE
                       Computer Society, Los Alamitos, 1986.
                    B.A. Sanders. An asynchronous, distributed flow control algorithm for rate allocation in computer
                       networks. IEEE Trans. Comp., 37(7):779–787, 1988.
                    B.A. Sanders. An incentive compatible flow control algorithm for rate allocation in computer net-
                       works. IEEE Trans. Comp., 37(9):1067–1072, 1988.
                    S. Shenker. Efficient network allocations with selfish users. In Peter J. B. King, Isi Mitrani, and
                       Rob Pooley, editors, Performance ’90, Proc. of the 14th IFIP WG 7.3 Intl. Symp. on Computer
                       Performance Modelling, Measurement and Evaluation, pp. 279–285, North-Holland, Amsterdam,
                       1990.
                    S. Shenker. Making greed work in networks: A game-theoretic analysis of switch service disciplines.
                       IEEE/ACM Trans. Networking, 3(6):819–831, 1995.
                    J. Shneidman and D.C. Parkes. Specification faithfulness in networks with rational nodes. In 23rd
                       Symp. on Princ. Distributed Computing, pp. 88–97, ACM, New York, 2004.
                    J.L. Sobrinho. An algebraic theory of dynamic network routing. IEEE/ACM Trans. Networking,
                       13(5):1160–1173, 2005.
                    K. Varadhan, R. Govindan, and D. Estrin. Persistent route oscillations in inter-domain routing.
                       Comput. Networks, 32(1):1–16, March 2000.
                    Y. Yemini. Selfish optimization in computer networks. In 20th Conf. Decision and Control, pp. 281–
                       285, IEEE, Pistacaway, 1981.




                                                               Exercises
                    14.1 Recall from Chapter 9 that, in a second-price Vickrey auction of a single item,
                         the item is sold to the highest bidder, and the price that the winner pays is the
                         second-highest bid. Consider a network in which there is one bidder at each node,
                         and the nodes lie on a cycle. As in Section 14.2, we assume that there is no
                         trusted center to implement an algorithm but that there is a central enforcer that
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0     July 5, 2007   14:26




                    384                distributed algorithmic mechanism design

                           can implement the outcome decided upon by the agents and can impose severe
                           penalties if the agents do not agree on an outcome. Give a distributed algorithm
                           for computing the winner and the price in a second-price Vickrey auction on such
                           a network that has the following properties: (i) it is incentive-compatible in ex-post
                           Nash equilibrium; (ii) it requires no more than two messages to cross each link;
                           and (iii) each message is at most O(log m + log n) bits long, where m is the highest
                           bid, and n is the number of bidders. Prove that your algorithm satisfies these three
                           properties
                    14.2 Prove that, in the MC multicast cost-sharing mechanism, there is a single “largest”
                         receiver set that maximizes NW.
                    14.3 Prove the correctness of the algorithm given in Section 14.2.2 for computation of
                         MC cost shares.
                    14.4 A strategyproof mechanism is group strategyproof (GSP) if no coalition of deviating
                         agents can achieve an outcome that is at least as good for all deviating agents and
                         strictly better for at least one. For each of the MC and SH multicast cost-sharing
                         mechanisms, either prove that it is GSP or provide a counterexample.
                    14.5 Consider a single-item, ascending-price auction with “jump bids.” Type θi denotes
                         agent i ’s value for the item. Bids are associated with a “bid price.” In round t, the
                         auctioneer announces an “ask price” pt that is  > 0 above the highest bid received
                         so far. Any agent can bid in round t, as long as the bid is at some price at or above
                         pt . The provisional winner is the agent with the current highest bid (breaking ties at
                         random). The auction terminates when no agent bids at the current ask price, and
                         the item is then sold to the provisional winner at its final bid price. The information
                         state ( pt , x t ) defines the current ask price pt and provisional winner x t ∈ {1, . . . , n}.
                         The following is a straightforward bidding strategy that determines what agent i
                         will do in state ( p, x): If p ≤ θi and x = i , then bid p; otherwise, do not bid. Prove
                         that this strategy profile is an ex-post Nash equilibrium but not a dominant-strategy
                         equilibrium.
                    14.6 Prove that policy consistency is satisfied if all ASes use next-hop policies, or if all
                         use metric-based policies.
                    14.7 Give an interdomain-routing instance (i.e., an AS graph in which one AS is identi-
                         fied as the destination, each edge is identified as a peer edge or a customer-provider
                         edge, and a valuation function is given for each source AS) that does not contain a
                         dispute wheel but also does not satisfy the Gao–Rexford constraints. Explain why
                         the Gao–Rexford constraints are not satisfied by this instance.
                    14.8 Prove that, in the interdomain-routing problem, it is NP-hard to find a route allo-
                         cation that comes within a constant factor of the maximum social welfare if no
                         restrictions are made on the valuation functions.
