---
type: "book-chapter"
book_id: "algorithmic-game-theory"
chapter_id: "ch-28"
chapter_number: 28
chapter_title: "Sponsored Search Auctions S´ebastien Lahaie, David M. Pennock, Amin Saberi,"
source_pdf: "raw/inbox/manual-drop/PDF_B.pdf"
source_page_start: 720
source_page_end: 737
printed_page_start: 720
printed_page_end: 737
part_ids: ["algorithmic-game-theory-ch-28-part-029"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Sponsored Search Auctions S´ebastien Lahaie, David M. Pennock, Amin Saberi,

P1: SBT
9780521872829main       CUNY1061-Nisan        0 521 87282 0     July 28, 2007     19:4




                                                              CHAPTER 28


                                   Sponsored Search Auctions

                              Sébastien Lahaie, David M. Pennock, Amin Saberi,
                                             and Rakesh V. Vohra




                                                                  Abstract

                    One of the more visible means by which the Internet has disrupted traditional activity is the manner
                    in which advertising is sold. Offline, the price for advertising is typically set by negotiation or posted
                    price. Online, much advertising is sold via auction. Most prominently, Web search engines like Google
                    and Yahoo! auction space next to search results, a practice known as sponsored search. This chapter
                    describes the auctions used and how the theory developed in earlier chapters of this book can shed
                    light on their properties. We close with a brief discussion of unresolved issues associated with the
                    sale of advertising on the Internet.



                                                           28.1 Introduction

                    Web search engines like Google and Yahoo! monetize their service by auctioning off
                    advertising space next to their standard algorithmic search results. For example, Apple
                    or Best Buy may bid to appear among the advertisements – usually located above
                    or to the right of the algorithmic results – whenever users search for “ipod.” These
                    sponsored results are displayed in a format similar to algorithmic results: as a list of
                    items each containing a title, a text description, and a hyperlink to the advertiser’s Web
                    page. We call each position in the list a slot. Generally, advertisements that appear
                    in a higher ranked slot (higher on the page) garner more attention and more clicks
                    from users. Thus, all else being equal, merchants generally prefer higher ranked slots
                    to lower ranked slots. Figure 28.1(a) shows an example layout of sponsored search
                    results for the query “las vegas travel.” Figure 28.1(b) shows the advertisers’ bids in
                    the corresponding auction.
                       Advertisers bid for placement on the page in an auction-style format where the
                    larger their bid the more likely their listing will appear above other advertisements on
                    the page. By convention, sponsored search advertisers generally pay per click, meaning
                    that they pay only when a user clicks on their advertisement, and do not pay if their
                    advertisement is displayed but not clicked. Overture Services, formerly GoTo.com and
                                                                      699
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    July 28, 2007   19:4




                    700                           sponsored search auctions




                                 (a) Search results                                (b) Advertisers’ bids

                    Figure 28.1. (a) An example display of sponsored search listings above the regular algorithmic
                    listings for the query “las vegas travel.” The ordering of sponsored listings is determined via a
                    continuous auction mechanism. (b) The top advertisers’ bids (maximum willingness to pay per
                    click) in the auction.


                    now owned by Yahoo! Inc., is credited with pioneering sponsored search advertising.
                    Overture’s success prompted a number of companies to adopt similar business models,
                    most prominently Google, the leading Web search engine today. Sponsored search is
                    one of the fastest growing, most effective, and most profitable forms of advertising,
                    generating roughly $7 billion in revenue in 2005 after nearly doubling every year for
                    the previous 5 years.
                       The sponsored search industry typically runs separate auctions for each search query:
                    for example, the queries “plasma television” and “investment advice” are associated
                    with two distinct auctions. The entity being sold in each auction is the right to appear
                    alongside the results of that search query. As mentioned, bids are expressed as a
                    maximum willingness to pay per click. For example, a 40-cent bid by HostRocket
                    for “Web hosting” means HostRocket is willing to pay up to 40 cents every time a
                    user clicks on their advertisement. Advertisers may also set daily or monthly budget
                    caps. In practice, hundreds of thousands of advertisers compete for positions alongside
                    several millions of search queries every day. Generally the auctions are continuous and
                    dynamic, meaning that advertisers can change their bids at any time, and a new auction
                    clears every time a user enters a search query. In this way advertisers can adapt to
                    changing environments, for instance by boosting their bids for the query “buy flowers”
                    during the week before Valentine’s Day. The search engine evaluates the bids and
                    allocates slots to advertisers. Notice that, although bids are expressed as payments per
                    click, the search engine cannot directly allocate clicks, but rather allocates impressions,
                    or placements on the screen. Clicks relate only stochastically to impressions.
                       Advertising in traditional media is typically sold on a per-impression basis, or
                    according to the (estimated) number of people exposed to the advertisement, in part
                    because of the difficulty of measuring and charging based on the actual effectiveness
                    of the advertisement. Traditional (offline) advertising, and to a large extent banner
                    advertising on the Web, is usually priced via an informal process of estimation and
                    negotiation. The Web’s capability for two-way communication makes it easy to track
                    some measures of effectiveness, in particular user clicks. Many advertisers, especially
                    direct marketers looking to close a sale as opposed to brand advertisers, prefer to
P1: SBT
9780521872829main       CUNY1061-Nisan           0 521 87282 0       July 28, 2007       19:4




                                                    existing models and mechanisms                                                   701

                    pay per click rather than per impression, alleviating some of the uncertainty inherent
                    in an impression. More direct performance-based pricing is possible by charging per
                    “action” or per conversion (sale) on the merchant’s site.
                        Search engines are an information gateway to many search and decision-making
                    tasks. Industry surveys report that more than 50% of Web users visit a search engine
                    every day, Americans conduct roughly 6 billion Web searches per month, over 13%
                    of traffic to commercial sites is generated by search engines, and over 40% of product
                    searches on the Web are initiated via search engines. As a result, entire niche industries
                    exist touting services to boost a Web page’s ranking on the popular search engines,
                    in part by reverse engineering the search engines’ information retrieval algorithms.
                    Research has shown that good placement on a search page leads to high traffic, and
                    eventually an increased financial payoff. Paying for sponsored slots is an alternative
                    means of obtaining prominent positioning. Sponsored search works because users
                    often tolerate or even welcome targeted advertisements directly related to what they
                    are actively searching for. For example, Majestic Research reports that as many as 17%
                    of Google searches result in a paid click, and that Google earns roughly nine cents on
                    average for every search query they process. Today, Internet giants Google and Yahoo!
                    boast a combined market capitalization of over $150 billion, largely on the strength
                    of sponsored search. PricewaterhouseCoopers and the Interactive Advertising Bureau
                    estimate that in 2005, industry-wide sponsored search revenue in the United States
                    reached $5.1 billion, or 41% of total U.S. Internet advertising revenues and 2% of
                    all U.S. advertising revenues. Roughly 85% of Google’s $4.1 billion in 2005 revenue
                    and roughly 45% of Yahoo!’s $3.7 billion in 2005 revenue is likely attributable to
                    sponsored search. A number of other companies – including eBay (Shopping.com),
                    FindWhat, InterActiveCorp (Ask.com), LookSmart, and Microsoft (MSN.com) – earn
                    hundreds of millions of dollars in sponsored search revenue annually.
                        The goal of this chapter is to formally model and analyze various mechanisms used
                    in this domain and to study potential improvements. In Section 28.2, we briefly describe
                    existing mechanisms used to allocate and price sponsored search advertisements. Sub-
                    sequently in Sections 28.3 and 28.4 we discuss formal models used to analyze the prop-
                    erties of these auctions. Section 28.5 discusses further extensions and open problems.

                                              28.2 Existing Models and Mechanisms

                    Typically, in sponsored search mechanisms, the advertisers specify a list of pairs of
                    keywords and bids as well as a total maximum daily or weekly budget. Then, every
                    time a user searches for a keyword, an auction takes place among the set of interested
                    advertisers who have not exhausted their budgets.
                       Focusing on a single auction, let n be the number of bidders and m < n the number
                    of slots. The search engine estimates αij , the probability that a user will click on the
                    ith slot when it is occupied by bidder j . The quantity αij is called a click through rate
                    (CTR). It is usually presumed for all j that αij ≥ αi+1,j for i = 1, . . . , m − 1.1

                    1 The assumption that clickthrough rate decays monotonically with lower slots is a distinguishing feature of

                      keyword auctions; in particular, it implies that all bidders prefer the first slot to the second, the second slot to
                      the third, etc. This allows for more refined equilibrium analyses than in the more general multi-item case.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 28, 2007   19:4




                    702                         sponsored search auctions

                       The search engine also assigns a weight wj to each advertiser j . The weight can
                    be thought of as a relevance or quality metric. If agent j bids bj , his corresponding
                    score is sj = wj bj . The search engine allocates slots in decreasing order of scores,
                    so that the agent with highest score is ranked first, and so on. We assume throughout
                    that agents are numbered so that agent j obtains slot j . An agent pays per click the
                    lowest bid necessary to retain his position, so that the agent in slot j pays sj +1 /wj .
                    This weighted bid ranking mechanism includes the two most prominent keyword
                    auction designs that have been used in practice: Overture introduced a “rank by bid”
                    mechanism (wj = 1) whereas Google uses a “rank by revenue” mechanism (wj =
                    α1j ). Both variants are sometimes called generalized second price (GSP) auctions.
                    Prior to 2004, Yahoo! used what is now known as a generalized first price (GFP)
                    auction. Agents are ranked by bid but each bidder who secures a slot pays their bid per
                    click.



                                                     28.3 A Static Model

                    The most popular model used to analyze keyword auctions is a static one where
                    the private information of bidder j , the expected payoff from a click, vj , is one
                    dimensional. The expected payoff to a bidder from not obtaining a slot is assumed to
                    be 0.
                        Four features of the model deserve comment. The first is its static nature: a
                    substantial departure from reality. Since the study of recurrent auctions is rather
                    daunting, one may be disposed to accept this simplification. Second, the expected
                    payoff per click to a bidder is slot independent. This is tied to the assumption that all
                    bidders prefer the top slot to the second slot to the third slot and so on. Some advertisers
                    believe that the probability of a click being converted into a purchase is lower in
                    the top slot than in the second slot because many clicks on the top slot are made
                    in error, or because a searcher who clicks on a lower-ranked slot is more serious in
                    their intent to purchase. Although the story sounds plausible, conversion-tracking data
                    from Isobar Communications and other sources does not substantiate the hypothesis:
                    in reality the top slot appears to convert about as well as other slots. Third, a bidder’s
                    value and CTR for a slot does not depend on the identity of other bidders. It seems
                    plausible that Avis might value the fact that Hertz is not present in any slot when Avis
                    is present. Fourth, CTRs are assumed to be common knowledge. In practice CTRs are
                    estimated by the search engine and can be conditioned on many factors, including user
                    characteristics and page context. Estimating CTRs is a significantly complex machine
                    learning problem for the search engine, including a built-in explore/exploit trade-off.
                    Moreover, bidders’ estimates of CTRs may be less accurate since bidders do not have
                    access to the same contextual information available to the search engine. The dynamic
                    nature of the environment means that CTRs can fluctuate dramatically over small
                    periods.
                        As usual we assume that bidders are risk neutral and that their utility for a slot can
                    be denominated on a common monetary scale. Supplied with copious amounts of salt,
                    let us see where this model takes us.
P1: SBT
9780521872829main      CUNY1061-Nisan        0 521 87282 0              July 28, 2007    19:4




                                                                   a static model                                      703

                                          28.3.1 Revenue Maximization and Efficiency
                    An auctioneer usually has one of two objectives: revenue maximization or allocative
                    efficiency. In the static model one knows exactly what auction design will achieve
                    either objective.
                       If the goal is revenue maximization, the classic result of Myerson (described in
                    Chapter 13) applies directly. One simply relabels the allocation variables. In Chapter 13
                    Section 13.1.12, the allocation variable, xj (b), is defined to be the expected quantity re-
                    ceived by bidder i who bids b. For our setting, xj (b) becomes the expected click through
                    rate for a bidder who bids b. Basically the generalized Vickrey auction is applied not
                    to the actual values, vj , but to the corresponding virtual values. The upshot is that the
                    revenue maximizing auction is a generalized Vickrey auction with reserve prices.
                       If the goal is allocative efficiency, the generalized Vickrey auction will do the trick.
                    The auction is described in Chapters 9 and 11 of this book. The underlying problem
                    of finding the efficient allocation in this case is an instance of the maximum weight
                    assignment problem. For each slot i and bidder j let xij = 1 if bidder j is assigned to
                    slot i and zero otherwise. The object is to choose xij ’s to solve the following:
                                               k 
                                                n
                                     max                    αij vj xij                                               (28.1)
                                               i=1 j =1
                                                   
                                                   n
                                      s.t.                 xij ≤ 1          ∀i = 1, . . . , k                        (28.2)
                                                    j =1

                                                
                                                k
                                                         xij ≤ 1            ∀j = 1, . . . , n                        (28.3)
                                                   i=1
                                                    xij ≥ 0                 ∀i = 1, . . . , k, ∀j = 1, . . . , n     (28.4)
                        This is equivalent to finding a maximum-weight perfect matching in a bipartite
                    graph and hence can be solved in polynomial time. In fact, because the constraint
                    matrix of this linear program is totally unimodular, it will have an optimal solution that
                    is integral. Any feasible integer solution is called an assignment.
                        A single computation of the maximum weight assignment is sufficient to determine
                    both the allocation and the generalized Vickrey payments. This is because the Vickrey
                    payments lie in the dual to the above linear program. To write down the dual, let pi be
                    the dual variable associated with (28.2) and qj the dual associated with (28.3).
                                             
                                             k             
                                                           n
                                  min              pi +            qj                                                (28.5)
                                             i=1            j =1
                                   s.t.      pi + qj ≥ αij vj               ∀i = 1, . . . , k,   ∀j = 1, . . . , n   (28.6)
                                              pi , qj ≥ 0                   ∀i = 1, . . . , k,   ∀j = 1, . . . , n   (28.7)
                    Here pi can be interpreted as the expected payment (CTR times price per click) of the
                    bidder obtaining slot i, and qj as the profit of bidder j . The objective in this program
                                                             profits combined. Among all optimal dual
                    is to minimize the bidders’ and auctioneer’s
                    solutions, pick the one that minimizes ki=1 pi . The corresponding pi is the price that
                    the generalized Vickrey auction would set for slot i.
P1: SBT
9780521872829main      CUNY1061-Nisan            0 521 87282 0   July 28, 2007    19:4




                    704                                sponsored search auctions

                        In the special case when the CTRs are bidder independent (i.e., αij = µi ) there is a
                    particularly simple algorithm, called the Northwest corner rule, to find the maximum
                    weight assignment. Assign the bidder with the highest value per click to the top slot,
                    the bidder with the second highest value per click to the second slot, and so on. In the
                    Economics literature this is called an assortative assignment.
                        If one objects to the sealed bid nature of the generalized Vickrey auction there are
                    ascending implementations available.
                        Interestingly, neither of these auctions corresponds to the GFP or GSP auctions. In
                    particular, bidding truthfully is not an equilibrium of either the GFP or GSP auctions.
                    It is interesting to observe that Google’s promotional material touts their auction as
                    a modification of Vickrey’s sealed bid auction for a single item (which it is) and
                    concluding, therefore, that bidding sincerely is the correct thing to do (which it is not).
                    A similar claim was made with respect to their auction used to sell shares of their
                    IPO. They are not the first and quite possibly not the last to make such claims. For
                    example, the financial services firm Hambrecht, which pioneered the use of auctions
                    to sell IPO’s in 1998, says that their auction design is based on the Vickrey auction for
                    a single good. While the Hambrecht auction does specialize to the Vickrey auction for
                    a single good, it does not inherit the attractive properties of the Vickrey auction when
                    applied to multiple units.2
                        To see why one must be careful when generalizing the Vickrey auction to the sale
                    of more than one unit, suppose that there are three bidders with v1 > v2 > v3 and two
                    slots. Also, suppose that αij = µi with µ1 > µ2 . If one were to auction off the top slot
                    only, by an English ascending auction, each bidder would remain in as long as at the
                    current price their surplus is nonnegative. So, if the current price on the top slot is p1 ,
                    bidder j remains active if µ1 (vj − p1 ) ≥ 0. Hence the auction ends at a price p1 where
                    µ1 (v2 − p1 ) = 0, i.e., p1 = v2 . Now suppose that both slots are available but we will
                    auction off the top slot first followed by the second slot. Let p1 be the current price of
                    slot 1, p2 = 0 the current price of slot 2. Now bidder j will remain active in the auction
                    for the top slot provided their surplus from the top slot is at least as large the surplus
                    they could get from the second slot (which is currently priced at zero). That is,
                                                                                      
                                                                                    µ2
                                        µ1 (vj − p1 ) ≥ µ2 (vj − 0) ⇒ p1 ≤ 1 −           vj .
                                                                                    µ1
                       Therefore the auction on the top slot terminates at a price of (1 − µµ21 )v2 < v2 . The
                    point is that the presence of a second slot lowers the price at which a bidder on the
                    top slot will drop out of the auction on the top slot. The generalized Vickrey auction
                    incorporates this change in the outside option of a bidder to ensure truthful bidding.
                    The GSP auction does not. The generalized Vickrey auction, however, would allocate
                    the top slot to bidder 1 and charge her (1 − µµ21 )v2 and the second slot to bidder 2 and
                    charge her v3 .
                       As noted above, the GFP and GSP are special cases of what have been called ranking
                    auctions. Bids (the reported vj ’s) are weighted (weights are independent of the bids)
                    and then ranked in the descending order. The highest ranked bidder gets the top slot,

                    2 All of this reminds one of what is known as the freshman binomial theorem: (a + b)n = a n + bn . True for

                      n = 1 but not for n > 2.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0    July 28, 2007   19:4




                                                           a static model                                     705

                    the second highest ranked bidder gets the second slot, and so on. The higher the bid the
                    higher the slot one obtains (other bids held fixed). Since the assignment of bidders to
                    slots is monotonic in the bid (other bids held fixed) it follows from standard results (see
                    Section 9.36 of Chapter 9 for example) that there exists a payment rule that will make
                    truthful bidding an equilibrium of the resulting auction. That payment rule is described,
                    for example, in Section 13.1.2 of Chapter 13. Let xj (b|b−j ) denote the expected click
                    through rate for agent j when she bids b, given the profile of other bids is b−j . Then
                    the payment Pj (b|b−j ) she must make to ensure incentive compatibility is given by
                                                                         b
                                         Pj (b | b−j ) = bx(b | b−j ) −      x(t | b−j ) dt.            (28.8)
                                                                            0
                    These ranking auctions are, in general, neither efficient nor revenue maximizing.
                    (Though in the exercises, we explore a special case ranking that is efficient.) The
                    payment rules associated with the GFP and GSP are not such as to induce truthful
                    bidding as an equilibrium.

                                                28.3.2 Equilibrium Properties
                    The fact that neither the GFP nor GSP is incentive compatible does not imply that they
                    are inefficient or suboptimal in terms of revenue. It is possible that the equilibrium
                    outcomes of both these auctions may be efficient or revenue maximizing. To identify
                    the revenue and efficiency properties of these auctions, it is necessary to determine
                    their equilibria.
                        The GFP auction does not admit a pure strategy full-information equilibrium but does
                    admit a pure strategy Bayes-Nash symmetric equilibrium. The argument is identical to
                    that of the sealed bid first price auction for a single good. The equilibrium bid functions
                    are monotonic in the value. Therefore the equilibrium allocation of bidders to slots is
                    the same as in the efficient allocation. Hence, by the revenue equivalence theorem, the
                    symmetric equilibrium is efficient.
                        The efficiency of the GFP (in a Bayesian setting) lends it some appeal but this is
                    where the “static” assumption has bite. In a dynamic setting, the absence of a pure
                    strategy full-information equilibrium encourages bidders to constantly adjust their bids
                    from one period to the next. This produces fluctuations in the bids over time and it has
                    been argued that these fluctuations resulted in significant inefficiencies.
                        To date nothing is known about the Bayesian equilibrium of the GSP auction.
                    Assume for simplicity that CTRs are bidder-independent, so αij = µi , and that all
                    weights are set to 1. The analysis in this section generalizes straightforwardly to the
                    case where CTRs are separable (i.e., αij = µi βj ) and agents are assigned arbitrary
                    weights wj . These extensions are developed in the exercises.
                        In this case one can show that the GSP is efficient under full information and a re-
                    stricted notion of equilibrium called locally envy-free. An assignment x is called locally
                    envy-free if there exist prices, {pi }, one for each slot, such that for all i, j with xij = 1
                                                   µi vj − pi ≥ µi−1 vj − pi−1                             (28.9)
                    and
                                                   µi vj − pi ≥ µi+1 vj − pi+1                            (28.10)
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0       July 28, 2007        19:4




                    706                             sponsored search auctions

                    In words, if bidder j is assigned to slot i, then she prefers slot i to the slot just above
                    her and the slot just below her.

                      Theorem 28.1       An assignment x ∗ is optimal if and only if it is locally envy-free.

                      proof Suppose first that x ∗ is locally envy-free and let p ∗ be the corresponding
                      price vector. It suffices to prove that the assignment x ∗ is assortative. Let j be such
                      that xij∗ = 1 and j  such that xi+1,j  = 1. To show that the assignment is assortative,
                      we must show that vj ≥ vj  . From the property of being locally envy-free, we
                      have
                                                                              ∗
                                                     µi vj − pi∗ ≥ µi+1 vj − pi+1
                      and
                                                                 ∗
                                                    µi+1 vj  − pi+1 ≥ µi vj  − pi∗ .
                      Adding them together yields
                                                        (µi − µi+1 )(vj − vj  ) ≥ 0.
                      Since µi > µi+1 it follows from this inequality that vj ≥ vj  .
                          Now let x ∗ be an optimal assignment. Let (p ∗ , q ∗ ) denote an optimal dual
                      solution. It suffices to show that (x ∗ , p ∗ ) is locally envy-free. Consider a pair
                      (r, j ) such that xrj∗ = 1. Complementary slackness and dual feasibility implies
                      that µr vj − pr∗ = qj∗ = maxi {µi vj − pi∗ }. Therefore
                                                                   ∗
                                      µr vj − pr∗ ≥ max{µr−1 vj − pr−1              ∗
                                                                       , µr+1 vj − pr+1 }.

                      Theorem 28.2 The GSP has a full information equilibrium that yields an allo-
                      cation that is locally envy-free.

                      proof Order the bidders so that v1 ≥ v2 ≥ · · · ≥ vn . Let pi∗ be the Vickrey
                                                                                                              p∗
                      price of slot i. Let bidder 1 bid b1 = v1 and each bidder j ≥ 2 bids bj = µjj −1  −1
                                                                                                           .
                      First we show that under the rules of the GSP, bidder 1 is assigned to slot 1, bidder
                      2 to slot 2, and so on. To do this, it suffices to show that bj −1 ≥ bj . Since the
                      optimal assignment is locally envy-free, we have
                                                    µj vj − pj∗ ≥ µj −1 vj − pj∗−1 .
                      Therefore
                                                                                        ∗
                                                              pj∗       µj −1      pj −1
                                                     vj −           ≥         vj −       ,
                                                              µj         µj         µj
                      which implies
                                                                                          
                                            pj∗−1       pj∗−1       pj∗           µj −1          pj∗
                                  bj −1 =           ≥           ≥         +             − 1 vj ≥     = bj .
                                            µj −1        µj         µj             µj            µj
                      Hence if each bidder j bids bj the GSP returns the optimal assignment. It is also
                      easy to see that bidder j ≤ m pays pj∗ for their slot. Bidder j > m pays zero.
P1: SBT
9780521872829main       CUNY1061-Nisan         0 521 87282 0     July 28, 2007     19:4




                                                               dynamic aspects                               707

                       Since each bidder pays their Vickrey price and receives the slot they would have
                       under the efficient allocation, no bidder has a unilateral incentive to change their
                       bid. Therefore we have an equilibrium that, from Theorem 1, is envy-free.

                       Absent the recurrent nature of keyword auctions, they are similar to what are known
                    as condominium auctions. In a condominium auction, bidders are interested in pur-
                    chasing a condominium in a building. The condominiums are identical except for their
                    height above the ground, the side of the building they are located on, etc. If all bidders
                    have identical preferences over the condominiums; i.e., everyone prefers to be on a
                    higher floor, they coincide with keyword auctions.


                                                        28.4 Dynamic Aspects

                    Since these auctions are repeated with great frequency, one should properly model
                    them as repeated games of incomplete information. The set of equilibria of such games
                    is quite rich and complicated, even when restricted to the setting considered here. A
                    full treatment of this case will not be given here. Rather we mention two phenomena
                    that arise in this setting.
                       One is known as bid rotation. This occurs when competing bidders take turns at
                    winning the auction. In our context this might mean bidders take turns at occupying
                    the top slot. If bidders are short lived, this is unlikely to be a problem, if not, this will
                    lower the auctioneers revenue.
                       Another possibility that repetition makes possible is vindictive bidding. In the GSP
                    auction one’s bid determines the payment of the bidder in the slot above and not one’s
                    own. Therefore one can increase the payment of the bidder in the slot above by raising
                    one’s bid without affecting one’s own payment. This may be beneficial if the bidder
                    in the slot above is a competitor with a limited budget for advertising. In a dynamic
                    environment this encourages a bidder to constantly adjust their bids so as to inflict or
                    avoid damage upon or from their competitor.
                       Even if one could ignore strategic considerations, a problem remains. The online
                    nature of the auctions in sponsored search complicates the computation of an efficient
                    allocation. Below we describe one model that addresses this difficulty.

                                               28.4.1 The Online Allocation Problem
                    In this model, the search engine receives the bids of advertisers and their maximum
                    budget for a certain period (e.g., a day). As users search for these keywords during
                    the day, the search engine assigns their advertisement space to advertisers and charges
                    them the value of their bid for the impression of the advertisement.3 For simplicity of
                    notation we assume that each page has only one slot for advertisements. The objective
                    is to maximize total revenue while respecting the budget constraint of the bidders. Note
                    that in this model bidders pay their bid which is counter to practice. On the other hand,
                    budget constraints that apply across a set of keywords, a real-world feature, are part of
                    the model.

                    3 If one scales the bids by the CTR, the model would accommodate pay per click.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0        July 28, 2007   19:4




                    708                            sponsored search auctions

                        Let n be the number of advertisers and m the number of keywords. Suppose that
                    advertiser j has a bid of bij for keyword i and a total budget of Bj . In this context, it
                    is reasonable to assume that bids are small compared to budgets, i.e., bij  Bj .
                        If the search engine has an accurate estimate of ri , the number of people searching
                    for keyword i for all 1 ≤ i ≤ m, then it is easy to approximate the optimal allocation
                    using a simple linear program. Let xij be the total number of queries on keyword i
                    allocated to bidder j . The linear program is
                                              m 
                                               n
                                     max                  bij xij
                                              i=1 j =1
                                              n
                                     s.t.             xij ≤ ri         ∀1 ≤ i ≤ m
                                               j =1
                                                                                                        (28.11)
                                            
                                            m
                                                  bij xij ≤ Bj         ∀1 ≤ j ≤ n
                                            i=1
                                         xij ≥ 0         ∀1 ≤ i ≤ m,                    ∀1 ≤ j ≤ n
                                            n         
                                                       m
                                     min       Bj βj +   ri αi
                                               j =1              i=1
                                     s.t.         αi + bij βj ≥ bij        ∀1 ≤ i ≤ m, ∀1 ≤ j ≤ n
                                                       βj ≥ 0              ∀1 ≤ j ≤ n
                                                       αi ≥ 0              ∀1 ≤ i ≤ m
                    By complementary slackness, in an optimal solution, advertiser j is assigned to key-
                    word i if (1 − βj )bij = max1≤k≤n (1 − βk )bik . Using this property, the search engine
                    can use the solution of the dual linear program to find the optimum allocation: every
                    time a user searches for keyword i, the search engine allocates its corresponding ad-
                    vertisement space to the bidder j with the highest bij (1 − βj ). In other words, the bid
                    of advertiser j will be scaled down by 1 − βj .
                       Now βj represents rate of change of the optimal objective function value of (28.11)
                    for a sufficiently small change in the right-hand side of the corresponding constraint.
                    In other words, if advertiser j ’s budget were to increase by , the optimal objective
                    function value would increase by βj . Equivalently, it is the opportunity cost of
                    consuming agent j ’s budget. Hence, if we allocate keyword i to agent now we obtain
                    an immediate ‘payoff’ of bij . However, this consumes bij of the budget, which imposes
                    an opportunity cost of βj bij . Therefore, it makes sense in the optimal solution to (28.11)
                    to assign keyword i to j provided bij − βj bij > 0.
                       In practice, a good estimate of the frequencies of all search queries is unavailable.
                    Queries arrive sequentially and the search engine must instantly decide to allocate their
                    advertisement space to bidders without knowledge of the future queries. Therefore,
                    what is needed is a dynamic procedure for allocating bidders to keywords that are
                    queried. We describe one such procedure and analyze its performance within the usual
                    competitive ratio framework. Specifically, we compare the revenue achieved by a
                    dynamic procedure that does not know the ri ’s in advance, with the revenue that could
                    be achieved knowing the ri ’s advance. The revenue in this second case is given by the
                    optimal objective function value of the program (28.11).
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0     July 28, 2007   19:4




                                                           dynamic aspects                                    709

                        The obvious dynamic procedure to consider is a greedy one: among the bidders
                    whose budgets are not exhausted, allocate the query to the one with the highest bid. It
                    is easy to see that this approach is equivalent to setting all βj ’s to 0.
                        The greedy procedure is not guaranteed to find the optimum solution. It is easy to
                    construct a simple example with two bidders and two keywords in which the revenue of
                    the greedy algorithm is as small as half of the optimum revenue. For example, suppose
                    two bidders each with a budget of $2. Assume that b11 = 2, b12 = 2 − , b21 = 2,
                    and b22 = . If query 1 arrives before query 2, it will be assigned to bidder 1. Then
                    bidder 1’s budget is exhausted. When query 2 arrives, it is assigned to bidder 2. This
                    produces an objective function value of 2 + . The optimal solution would assign query
                    2 to bidder 1 and query 1 to bidder 2, yielding an objective function value of 4. The
                    problem with the greedy algorithm is that, unlike the solution to (28.11), it ignores the
                    opportunity cost of assigning a query to a bidder.
                        One can prove that the revenue of greedy algorithm is at least half of the optimum
                    revenue for any instance. In the standard terminology of online algorithms, the com-
                    petitive ratio of greedy algorithm is 1/2. Can one do better in terms of competitive
                    ratio? Yes. One does so by trying to dynamically estimate the opportunity cost , i.e.,
                    the βj ’s, of assigning a query to a bidder. This has the effect of spreading the bidders
                    expenditures over time. The effect is called “budget smoothing,” and is a feature that
                    some search engines offer their advertisers.
                        The following modification of the greedy algorithm adaptively updates the βj ’s as
                    a function of the bidders spent budget. Let

                                                           φ(x) = 1 − ex−1 .

                    The algorithm sets βj = 1 − φ(fj ), where fj is the fraction of the budget of bidder j ,
                    which has been spent.


                      Algorithm 1. Every time a query i arrives, allocate its advertisement space to
                      the bidder j , who maximizes bij φ(fj ), where fj is the fraction of the bidder j ’s
                      budget which has been spent so far.


                      The revenue of this algorithm is at least 1 − 1/e of the optimum revenue. It is also
                    possible to prove that no deterministic or randomized algorithm can achieve a better
                    competitive ratio.


                      Theorem 28.3        The competitive ratio of Algorithm 1 is 1 − 1/e.


                    We outline the main ideas in the proof of the theorem. Let k be a sufficiently large
                    number used for discretizing the budgets of the bidders. We say that an advertiser
                    is of type j if she has spent within ( j −1
                                                             k
                                                                , jk ] fraction of her budget so far. Let sj be
                    the total budget of type j bidders. For i = 0, 1, . . . , k, define wi to be the amount of
                    money spent by all the bidders from the interval ( i−1   k
                                                                               , ki ] of their budgets.Also define
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0        July 28, 2007     19:4




                    710                         sponsored search auctions

                    the discrete version of function φ,
                                                                                k−s
                                                                     1
                                                      (s) = 1 − 1 −                     .             (28.12)
                                                                     k
                    It is easy to see that when k tends to infinity (s) → φ( ks ). Let OPT be the solution of
                    the optimal off-line algorithm (i.e., the solution of the optimization program (28.11)).
                    For simplicity, assume that the optimal algorithm spends all of the budget of the bidders.
                    We have the following lemma.

                      Lemma 28.4        At the end of the algorithm, this inequality holds:
                                                      
                                                      k                 
                                                                        k
                                                            (i)si ≤           .(i)wi                 (28.13)
                                                      i=0               i=0


                      proof Consider the time that query q arrives. Suppose that OPT allocates q
                      to a bidder of current type t, whose type at the end of the algorithm will be t  .
                      Let bopt and balg be the amount of money that OPT and the algorithm get from
                      bidders for q. Let i be the type of the bidder that the algorithm allocates the query.
                      We have
                                                (t  )bopt ≤ (t)bopt ≤ (i)balg .                 (28.14)
                         Now summing the inequalityabove over all the queries, the left-hand side of
                      (28.14)
                             contributes to the sum i (i)si , and the right-hand side contributes to
                         (i)wi . So the lemma follows.

                      Now, we are ready to prove the Theorem 28.3.
                                                            k
                      proof By definition wi ≤ k1              j =i sj . Using Lemma 28.4,

                                                 
                                                 k
                                                              1
                                                                 k        k
                                                     (i)si ≤       (i)      sj .
                                                 i=0
                                                              k i=0      j =i

                        Changing the order of the sums and computing the sum of the geometric series,
                      we have
                               
                               k
                                             1
                                                k        k
                                    (i)si ≤       (i)      sj
                                i=0
                                             k i=0      j =i
                                                                     
                                               1
                                                  k     i
                                           ≤                       (i) si
                                               k i=0        j =0
                                               k 
                                                                        
                                                   i                     1
                                           ≤          + (i) − (0) + O        si
                                             i=0
                                                   k                     k
                                             k                      k       k
                                                 i                  1
                                           ≤       si − (0) − O             si +     (i)si ,
                                             i=0
                                                 k                  k    i=0      i=0
P1: SBT
9780521872829main       CUNY1061-Nisan           0 521 87282 0       July 28, 2007      19:4




                                                                 open questions                                                    711

                       which yields
                                                                k       k
                                                                1                i
                                                      (0) − O          si ≤       si .
                                                                k   i=0      i=0
                                                                                 k

                          Note that as k goes to infinity the left-hand side tends to (1 − 1e )OPT .
                       The right-hand side is equal to the revenue of the algorithm. So the theorem
                       follows.

                    The same algorithm can be applied when multiple advertisement can appear with the
                    result of a query or when advertisers enter at different times. At present, the equilibrium
                    properties of this allocation rule are unknown.


                                                            28.5 Open Questions

                    We close this chapter with a brief review of important issues not directly addressed in
                    this chapter.
                       While our discussion has focused on existing mechanisms, one should not conclude
                    that there is no room for improvement in their design. For example, there is debate over
                    the role of the budget constraints in these auction. In many cases they do not appear
                    to be hard constraints as bidders frequently adjust them. A bidder can also “expand”
                    their budget simply by lowering their bid and paying less per click. Some argue that
                    the budget constraint is merely a convenient way to express other desires. For example,
                    limiting one’s exposure or spreading one’s advertising over a longer period. All of
                    this suggests the need for richer bidding models. Ones that might allow bidders to
                    express decreasing marginal value for clicks, or distinct values for traffic from certain
                    geographic regions, demographic profiles, etc., support greater allocative efficiency,
                    though pose a significant burden in terms of computational and elicitation costs.
                       When advertiser payments are based on user clicks, search engines must invest in
                    the task of detecting and ignoring robot clicks, spam clicks as well as clicks from an
                    advertiser trying to impose costs on their competitor or from an affiliate who actually
                    benefits monetarily from additional clicks. For this reason there is interest in exploring
                    alternate pricing conventions. The most compelling is pay per action or conversion. The
                    advertiser pays only if a click results in a sale, for example. This raises new incentive
                    issues associated with tracking sales.
                       The models in this chapter, as do most analyses in the literature, assume a monopoly
                    search engine with a static user base. This would be an appropriate model if switching
                    costs for advertisers and users were high. In fact, switching costs for many advertisers
                    are low; many advertisers work with both Google and Yahoo! simultaneously, or work
                    with third-party search engine marketers to manage their account across multiple
                    search engines. Switching costs for users are essentially zero: to patronize a different
                    search engine, users need merely type a new address into their web browser.4 The

                    4 Personalization features may begin to introduce moderate switching costs for users. For now, reputation and

                      branding seem to play a major role in search engine loyalty: blind relevance tests show little or no difference in
                      quality among major search engines.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 28, 2007   19:4




                    712                         sponsored search auctions

                    competitive pressures to retain advertisers able to switch advertisement networks or use
                    multiple networks may cause firms to focus less on extracting the maximum revenue
                    from advertisers possible and more on attracting and retaining advertisers. Similarly,
                    search engines must make trade-off decisions between maximizing current period rev-
                    enue and attracting and retaining users in the long term. For this reason it would be very
                    instructive to understand the properties of keyword auctions in competition with each
                    other.
                       The major search engines syndicate their advertisements to affiliate search engines
                    and content providers. For example, Google, through its AdSense program, syndicates
                    advertisements to AOL, MySpace, and thousands of other Web sites. The introduction
                    of affiliates greatly complicates the semantics of bidding and allocation.
                       We have assumed that CTRs are given. In practice, CTRs are learned over time
                    and can depend on a variety of factors such as bidder identity; advertisement identity
                    and content; user characteristics, including demographics, location, and history; and/or
                    page context including other advertisements and algorithmic results. Learning CTRs
                    poses an explore/exploit trade-off: the auctioneer can exploit known high-CTR ad-
                    vertisements, or explore new advertisements or infrequently shown advertisements to
                    uncover even higher-CTR advertisements. The auctioneer’s CTR estimate may differ
                    from the bidder’s estimate; in particular, the auctioneer usually has more contextual
                    information to learn from.
                       In this chapter, we have focused on the auctioneer’s mechanism design problem.
                    The advertiser’s bidding optimization problem is also challenging and the focus of a
                    great deal of commercial and research activity.


                                                 28.6 Bibliographic Notes

                    The growth of paid placement has attracted recent research on this topic. Hoffman
                    and Novak (2000) discuss the trend in Internet advertising toward per-click pricing
                    rather than the traditional per-impression model. A good discussion of the practice of
                    sponsored search is available on the Web at http://searchenginewatch.com/
                    webmasters/paid.html.
                       Computing the explicit form of incentive compatible payments for ranking auctions
                    is carried out in Aggarwal et al. (2006) and Iyengar and Kumar (2006). The Bayesian
                    equilibrium of the GFP is derived in Lahaie (2006). The details of the revenue max-
                    imizing auction for (static) slot auctions is derived in Feng (2005) and Iyengar and
                    Kumar (2006). The envy-free analysis of the static model is due to Edelman et al. (in
                    press). A similar analysis can be found in Varian (in press). The latter paper shows
                    how upper and lower bounds on bidders’ actual values can be derived given their bids.
                    Feng et al. (2006) explore four ranking algorithms via simulation. All of these results
                    would apply to condominium auctions as well; see Burguet (2005) for a discussion of
                    condominium auctions.
                       The Northwest corner rule for the assignment problem dates back to Monge (1981).
                    Ascending implementations of the Vickrey auction for the static model can be found
                    in Crawford and Knoer (1981) and Demange, Gale, and Sotomayor (1986) (which
                    is a variant of the Hungarian algorithm for solving the assignment problem). The
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 28, 2007   19:4




                                                            bibliography                                        713

                    auction of Demange, Gale, and Sotomayor was dubbed, in Edelman et al. (in press),
                    the generalized English auction.
                       The online allocation problem studied in Section 28.4.1 is proposed and analyzed by
                    Mehta et al. (2005). This problem is a generalization of the online bipartite matching
                    problem studied by Karp et al. (1990) and Kalyanasundaram and Pruhs (2000). More
                    recently Buchbinder et al. (2006) gave a primal-dual algorithm and analysis for the
                    problem given in Mehta et al. They also extended that framework to scenarios in which
                    additional information is available, yielding improved worst-case competitive factors.
                       Mahdian et al. (2006) study the online allocation problem when the search engine
                    has a somewhat reliable estimate of the number of users searching for a keyword
                    everyday. Mahdian and Saberi (2006) study multiunit auctions for perishable goods, in
                    a setting where the supply arrives online. They motivate their model by its application
                    to sponsored search. Abrams (2006) and Borgs et al. (2005) design multiunit auctions
                    for budget-constrained bidders, which can be interpreted as slot auctions, with a
                    focus on revenue optimization and truthfulness. For a discussion of vindictive bidding
                    and some of the dynamic aspects of slot auctions see Asdemir (2006) and Zhou and
                    Lukose (2006).
                       Weber and Zheng (2006) study the implementation of paid placement strategies, and
                    find that the revenue-maximizing search engine design bases rankings on a weighted
                    average of relative quality performance and bid amount. Hu (2003) uses contract
                    theory to show that performance-based pricing models can give the publisher proper
                    incentives to improve the effectiveness of advertising campaigns. Rolland and Patterson
                    (2003) propose a methodology, using expert systems to improve the matching between
                    advertisers and Web users.
                       Besides the optimal ranking mechanism, the search engine must also choose the
                    number of paid slots by finding the optimal trade-off between sponsorship and user
                    retention. Bhargava and Feng (2002) provide a theoretical model to explain and analyze
                    this trade-off.
                       The problem of learning CTRs is nontrivial and presents an explore/exploit trade-
                    off. Pandey and Olston (2006) formulate the problem as an appropriate multiarmed
                    bandit optimization; Gonen and Pavlov (2007) derive a bandit optimization algorithm
                    that retains incentive compatibility for bidders.
                       Several authors explore the advertiser’s bidding optimization problem (Borgs et al.,
                    2005; Cary et al., 2007; Kitts et al., 2005; Kitts and LeBlanc, 2004; Rusmevichientong
                    and Williamson, 2006). Kitts et al. (2005) provide evidence that the first slot does not
                    have an appreciably lower conversion rate than the second slot as some advertisers
                    believe.

                                                            Bibliography
                    Z. Abrams. Revenue maximization when bidders have budgets. In Proc. Symp. on Discrete Algorithms,
                       Miami, FL, 2006.
                    G. Aggarwal, A. Goel, and R. Motwani. Truthful auctions for pricing search keywords. In Proc. 7th
                       ACM Conf. on Electronic Commerce, Ann Arbor, MI, 2006.
                    K. Asdemir. Bidding patterns in search engine auctions. In Proc. 2nd Workshop on Sponsored Search
                       Auctions, Ann Arbor, MI, 2006.
P1: SBT
9780521872829main      CUNY1061-Nisan        0 521 87282 0    July 28, 2007    19:4




                    714                            sponsored search auctions

                    H.K. Bhargava and J. Feng. Preferential placement in internet search engines. In Proc. 11th World
                       Wide Web Conf., Honolulu, HI, 2002.
                    C. Borgs, J. Chayes, O. Etesami, N. Immorlica, K. Jain, and M. Mahdian. Bid optimization in online
                       advertisement auctions. Preprint, 2005.
                    C. Borgs, J. Chayes, N. Immorlica, M. Mahdian, and A. Saberi. Multi-unit auctions with budget-
                       constrained bidders. In Proc. 6th Conf. Electronic Commerce, Vancouver, British Columbia,
                       Canada, 2005.
                    N. Buchbinder, K. Jain, and J. Naor. Online primal-dual algorithms for maximizing ad-auctions
                       revenue. Preprint, 2006.
                    R. Burguet. The condominium problem; auctions for substitutes. Rev. Econ. Design, 9, 2005.
                    M. Cary, A. Das, B. Edelman, I. Giotis, K. Heimerl, A. Karlin, C. Mathieu, and M. Schwarz. Greedy
                       bidding strategies for keyword auctions. Preprint, 2007.
                    V.P. Crawford and E.M. Knoer. Job matching with heterogeneous firms and workers. Econometrica,
                       49(2):437–450, 1981.
                    G. Demange, D. Gale, and M. Sotomayor. Multi-item auctions. J. Political Econ., 94(4):863–872,
                       1986.
                    B. Edelman, M. Ostrovsky, and M. Schwarz. Internet advertising and the Generalized Second Price
                       auction: Selling billions of dollars worth of keywords. Amer. Econ. Review, In press.
                    J. Feng. Optimal mechanism for selling a set of commonly ranked objects. Working paper, University
                       of Florida, February 2005.
                    J. Feng, H.K. Bhargava, and D.M. Pennock. Implementing sponsored search in Web search engines:
                       Computational evaluation of alternative mechanisms. INFORMS J. Computing, 2006, In press.
                    R. Gonen and E. Pavlov. An incentive compatible multi armed bandit mechanism. Preprint, 2007.
                    D.L. Hoffman and T.P. Novak. How to acquire customers on the Web. Harv. Busin. Rev., 78(3),
                       May–June 2000.
                    Y.J. Hu. Performance-based pricing models in online advertising. Technical report, Sloan School of
                       Management, MIT, 2003.
                    G. Iyengar and A. Kumar. Characterizing optimal keyword auctions. In Proc. 2nd Workshop on
                       Sponsored Search Auctions, Ann Arbor, MI, 2006.
                    B. Kalyanasundaram and K.R. Pruhs. An optimal deterministic algorithm for online b-matching.
                       Theor. Comp. Sci., 233(1–2):319–325, 2000.
                    R. Karp, U. Vazirani, and V. Vazirani. An optimal algorithm for online bipartite matching. In Proc.
                       22nd Symp. Theory of Computing, Baltimore, MD, 1990.
                    B. Kitts, P. Laxminarayan, B. LeBlanc, and R. Meech. A formal analysis of search auctions including
                       predictions on click fraud and bidding tactics. In Proc. 1st Workshop on Sponsored Search Auctions
                       at the ACM Conf. on Electronic Commerce, Vancouver, British Columbia, Canada, 2005.
                    B. Kitts and B. LeBlanc. Optimal bidding on keyword auctions. Electronic Markets, 14(3):186–201,
                       2004.
                    S. Lahaie. An analysis of alternative slot auction designs for sponsored search. In Proc. 7th Conf. on
                       Electronic Commerce, Ann Arbor, MI, 2006.
                    M. Mahdian, H. Nazerzadeh, and A. Saberi. Allocating online advertisement space with unreliable
                       estimates. In Proc. 8th ACM Conf. on Electronic Commerce, San Diego, CA, 2007.
                    M. Mahdian and A. Saberi. Multiunit auctions with unknown supply. In Proc. 7th ACM Conf. on
                       Electronic Commerce, Ann Arbor, MI, 2006.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 28, 2007   19:4




                                                             exercises                                          715

                    A. Mehta, A. Saberi, U. Vazirani, and V. Vazirani. AdWords and generalized on-line matching. In
                       Proc. 46th Annual Symp. on Fdns. of Comp. Sci., 2005.
                    G. Monge. Sur la théorie des déblais et des remblais. Mémoires de l’académie de Paris, 1781.
                    S. Pandey and C. Olston. Handling advertisements of unknown quality in search advertising. In
                       Neural Information Processing Systems, 2006.
                    E. Rolland and R.A. Patterson. Classification in online pay-for-performance advertising. In Proc.
                       13th Annual Workshop On Information Technologies and Systems, Seattle, WA, 2003.
                    P. Rusmevichientong and D.P. Williamson. An adaptive algorithm for selecting profitable keywords
                       for search-based advertising services. In Proc. 7th ACM Conf. on Electronic Commerce, pp. 260–
                       269, Ann Arbor, MI, 2006.
                    H.R. Varian. Position auctions. Intl. J. Industrial Organization, in press.
                    T.A. Weber and Z. Zheng. A model of search intermediaries and paid referrals. OPIM Working Paper
                       02-12-01, Wharton School, April 2002.
                    Y. Zhou and R. Lukose. Vindictive bidding in keyword auctions. In Proc. 2nd Workshop on Sponsored
                       Search Auctions, Ann Arbor, MI, 2006.




                                                              Exercises

                    28.1 Consider the model of keyword auctions where the CTR of agent j in slot i is µi .
                         Is every full-information equilibrium of the GSP locally envy-free?
                    28.2 Consider the model of keyword auctions where the CTR of agent j in slot i is
                         µi β j ; i.e.; the CTR is separable into a bidder effect β j and a position effect µi .
                         Suppose also that µ1 > µ2 > · · · > µm. Give a simple algorithm for determining
                         the efficient allocation of bidders to slots. Derive the payment rule implied by the
                         VCG mechanism for this environment.
                    28.3 In the model of the previous exercise, suppose also that the auctioneer assigns a
                         weight w j ≡ w j (β j ) to each bidder; weights may depend on the bidder effects, but
                         not on their bids. Suppose bidders are assigned to slots by decreasing order of their
                         scores w j b j . Use formula (28.8) to derive the payment rule that combined with the
                         allocation rule just described would yield an incentive compatible mechanism.
                    28.4 Consider the model of keyword auctions where the CTR of agent j in slot i is
                         µi β j ; i.e., the CTR is separable into a bidder effect β j and a position effect µi . The
                         auctioneer sets weights w j = β j , and a bidder pays the lowest amount necessary
                         to retain his position.

                           (a) Give the inequalities that characterize a full-information (Nash) equilibrium
                               in this model. Strenghten them to give the inequalities for a locally envy-free
                               equilibrium.
                           (b) Show that in a locally envy-free equilibrium, bidders are ranked in order of
                               decreasing β j v j .
                           (c) From among the set of locally envy-free equilibria, exhibit the one that yields
                               the smallest possible revenue to the auctioneer.
P1: SBT
9780521872829main      CUNY1061-Nisan    0 521 87282 0   July 28, 2007   19:4




                    716                        sponsored search auctions

                    28.5 Consider the model of keyword auctions where the CTR of agent j in slot i is
                         µi . Give an example of where the GFP auction does not admit a pure strategy
                         full-information equilibrium. For simplicity, you may assume a discretized set of
                         allowable bids.
                    28.6 Consider the online allocation problem discussed in Section 28.4. Show that the
                         competitive ratio of the algorithm remains the same even if the optimum solution
                         does not exhaust all the budgets.
