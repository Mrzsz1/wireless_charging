---
title: "algorithmic-game-theory-ch-28-part-029"
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
pdf_path: "work/core-books/algorithmic-game-theory/parts/algorithmic-game-theory-ch-28-part-029.pdf"
raw_md: "raw/canonical/algorithmic-game-theory-ch-28-part-029/full.md"
---
# Sponsored Search Auctions

Sebastien Lahaie, David M. Pennock, Amin Saberi,´ and Rakesh V. Vohra

## Abstract

One of the more visible means by which the Internet has disrupted traditional activity is the manner in which advertising is sold. Offline, the price for advertising is typically set by negotiation or posted price. Online, much advertising is sold via auction. Most prominently, Web search engines like Google and Yahoo! auction space next to search results, a practice known as sponsored search. This chapter describes the auctions used and how the theory developed in earlier chapters of this book can shed light on their properties. We close with a brief discussion of unresolved issues associated with the sale of advertising on the Internet.

## 28.1 Introduction

Web search engines like Google and Yahoo! monetize their service by auctioning off advertising space next to their standard algorithmic search results. For example, Apple or Best Buy may bid to appear among the advertisements – usually located above or to the right of the algorithmic results – whenever users search for “ipod.” These sponsored results are displayed in a format similar to algorithmic results: as a list of items each containing a title, a text description, and a hyperlink to the advertiser’s Web page. We call each position in the list a slot. Generally, advertisements that appear in a higher ranked slot (higher on the page) garner more attention and more clicks from users. Thus, all else being equal, merchants generally prefer higher ranked slots to lower ranked slots. Figure 28.1(a) shows an example layout of sponsored search results for the query “las vegas travel.” Figure 28.1(b) shows the advertisers’ bids in the corresponding auction.

Advertisers bid for placement on the page in an auction-style format where the larger their bid the more likely their listing will appear above other advertisements on the page. By convention, sponsored search advertisers generally pay per click, meaning that they pay only when a user clicks on their advertisement, and do not pay if their advertisement is displayed but not clicked. Overture Services, formerly GoTo.com and now owned by Yahoo! Inc., is credited with pioneering sponsored search advertising. Overture’s success prompted a number of companies to adopt similar business models, most prominently Google, the leading Web search engine today. Sponsored search is one of the fastest growing, most effective, and most profitable forms of advertising, generating roughly \$7 billion in revenue in 2005 after nearly doubling every year for the previous 5 years.

![](images/69b777927a6e347d97e3380cab0b564d00679f13b8f9b3631bc6584fda2136f2.jpg)  
Figure 28.1. (a) An example display of sponsored search listings above the regular algorithmic listings for the query “las vegas travel.” The ordering of sponsored listings is determined via a continuous auction mechanism. (b) The top advertisers’ bids (maximum willingness to pay per click) in the auction.

The sponsored search industry typically runs separate auctions for each search query: for example, the queries “plasma television” and “investment advice” are associated with two distinct auctions. The entity being sold in each auction is the right to appear alongside the results of that search query. As mentioned, bids are expressed as a maximum willingness to pay per click. For example, a 40-cent bid by HostRocket for “Web hosting” means HostRocket is willing to pay up to 40 cents every time a user clicks on their advertisement. Advertisers may also set daily or monthly budget caps. In practice, hundreds of thousands of advertisers compete for positions alongside several millions of search queries every day. Generally the auctions are continuous and dynamic, meaning that advertisers can change their bids at any time, and a new auction clears every time a user enters a search query. In this way advertisers can adapt to changing environments, for instance by boosting their bids for the query “buy flowers” during the week before Valentine’s Day. The search engine evaluates the bids and allocates slots to advertisers. Notice that, although bids are expressed as payments per click, the search engine cannot directly allocate clicks, but rather allocates impressions, or placements on the screen. Clicks relate only stochastically to impressions.

Advertising in traditional media is typically sold on a per-impression basis, or according to the (estimated) number of people exposed to the advertisement, in part because of the difficulty of measuring and charging based on the actual effectiveness of the advertisement. Traditional (offline) advertising, and to a large extent banner advertising on the Web, is usually priced via an informal process of estimation and negotiation. The Web’s capability for two-way communication makes it easy to track some measures of effectiveness, in particular user clicks. Many advertisers, especially direct marketers looking to close a sale as opposed to brand advertisers, prefer to pay per click rather than per impression, alleviating some of the uncertainty inherent in an impression. More direct performance-based pricing is possible by charging per “action” or per conversion (sale) on the merchant’s site.

Search engines are an information gateway to many search and decision-making tasks. Industry surveys report that more than 50% of Web users visit a search engine every day, Americans conduct roughly 6 billion Web searches per month, over 13% of traffic to commercial sites is generated by search engines, and over 40% of product searches on the Web are initiated via search engines. As a result, entire niche industries exist touting services to boost a Web page’s ranking on the popular search engines, in part by reverse engineering the search engines’ information retrieval algorithms. Research has shown that good placement on a search page leads to high traffic, and eventually an increased financial payoff. Paying for sponsored slots is an alternative means of obtaining prominent positioning. Sponsored search works because users often tolerate or even welcome targeted advertisements directly related to what they are actively searching for. For example, Majestic Research reports that as many as 17% of Google searches result in a paid click, and that Google earns roughly nine cents on average for every search query they process. Today, Internet giants Google and Yahoo! boast a combined market capitalization of over \$150 billion, largely on the strength of sponsored search. PricewaterhouseCoopers and the Interactive Advertising Bureau estimate that in 2005, industry-wide sponsored search revenue in the United States reached \$5.1 billion, or 41% of total U.S. Internet advertising revenues and 2% of all U.S. advertising revenues. Roughly 85% of Google’s \$4.1 billion in 2005 revenue and roughly 45% of Yahoo!’s \$3.7 billion in 2005 revenue is likely attributable to sponsored search. A number of other companies – including eBay (Shopping.com), FindWhat, InterActiveCorp (Ask.com), LookSmart, and Microsoft (MSN.com) – earn hundreds of millions of dollars in sponsored search revenue annually.

The goal of this chapter is to formally model and analyze various mechanisms used in this domain and to study potential improvements. In Section 28.2, we briefly describe existing mechanisms used to allocate and price sponsored search advertisements. Subsequently in Sections 28.3 and 28.4 we discuss formal models used to analyze the properties of these auctions. Section 28.5 discusses further extensions and open problems.

## 28.2 Existing Models and Mechanisms

Typically, in sponsored search mechanisms, the advertisers specify a list of pairs of keywords and bids as well as a total maximum daily or weekly budget. Then, every time a user searches for a keyword, an auction takes place among the set of interested advertisers who have not exhausted their budgets.

Focusing on a single auction, let n be the number of bidders and m < n the number of slots. The search engine estimates $\alpha _ { i j }$ , the probability that a user will click on the ith slot when it is occupied by bidder $j$ . The quantity $\alpha _ { i j }$ is called a click through rate (CTR). It is usually presumed for all $j$ that $\alpha _ { i j } \geq \alpha _ { i + 1 , j }$ for $i = 1 , \ldots , m - 1 .$ 1

The search engine also assigns a weight $w _ { j }$ to each advertiser j. The weight can be thought of as a relevance or quality metric. If agent j bids $b _ { j }$ , his corresponding score is $s _ { j } = w _ { j } b _ { j }$ . The search engine allocates slots in decreasing order of scores, so that the agent with highest score is ranked first, and so on. We assume throughout that agents are numbered so that agent j obtains slot j. An agent pays per click the lowest bid necessary to retain his position, so that the agent in slot $j \mathrm { \ p a y s \ } s _ { j + 1 } / w _ { j }$ This weighted bid ranking mechanism includes the two most prominent keyword auction designs that have been used in practice: Overture introduced a “rank by bid” mechanism $( w _ { j } = 1 )$ whereas Google uses a “rank by revenue” mechanism $( w _ { j } =$ $\alpha _ { 1 j } )$ . Both variants are sometimes called generalized second price (GSP) auctions. Prior to 2004, Yahoo! used what is now known as a generalized first price (GFP) auction. Agents are ranked by bid but each bidder who secures a slot pays their bid per click.

## 28.3 A Static Model

The most popular model used to analyze keyword auctions is a static one where the private information of bidder $j ,$ , the expected payoff from a click, $v _ { j }$ , is one dimensional. The expected payoff to a bidder from not obtaining a slot is assumed to be 0.

Four features of the model deserve comment. The first is its static nature: a substantial departure from reality. Since the study of recurrent auctions is rather daunting, one may be disposed to accept this simplification. Second, the expected payoff per click to a bidder is slot independent. This is tied to the assumption that all bidders prefer the top slot to the second slot to the third slot and so on. Some advertisers believe that the probability of a click being converted into a purchase is lower in the top slot than in the second slot because many clicks on the top slot are made in error, or because a searcher who clicks on a lower-ranked slot is more serious in their intent to purchase. Although the story sounds plausible, conversion-tracking data from Isobar Communications and other sources does not substantiate the hypothesis: in reality the top slot appears to convert about as well as other slots. Third, a bidder’s value and CTR for a slot does not depend on the identity of other bidders. It seems plausible that Avis might value the fact that Hertz is not present in any slot when Avis is present. Fourth, CTRs are assumed to be common knowledge. In practice CTRs are estimated by the search engine and can be conditioned on many factors, including user characteristics and page context. Estimating CTRs is a significantly complex machine learning problem for the search engine, including a built-in explore/exploit trade-off. Moreover, bidders’ estimates of CTRs may be less accurate since bidders do not have access to the same contextual information available to the search engine. The dynamic nature of the environment means that CTRs can fluctuate dramatically over small periods.

As usual we assume that bidders are risk neutral and that their utility for a slot can be denominated on a common monetary scale. Supplied with copious amounts of salt, let us see where this model takes us.

## 28.3.1 Revenue Maximization and Efficiency

An auctioneer usually has one of two objectives: revenue maximization or allocative efficiency. In the static model one knows exactly what auction design will achieve either objective.

If the goal is revenue maximization, the classic result of Myerson (described in Chapter 13) applies directly. One simply relabels the allocation variables. In Chapter 13 Section 13.1.12, the allocation variable, $x _ { j } ( b )$ , is defined to be the expected quantity received by bidder i who bids b. For our setting, $x _ { j } ( b )$ becomes the expected click through rate for a bidder who bids b. Basically the generalized Vickrey auction is applied not to the actual values, $v _ { j }$ , but to the corresponding virtual values. The upshot is that the revenue maximizing auction is a generalized Vickrey auction with reserve prices.

If the goal is allocative efficiency, the generalized Vickrey auction will do the trick. The auction is described in Chapters 9 and 11 of this book. The underlying problem of finding the efficient allocation in this case is an instance of the maximum weight assignment problem. For each slot i and bidder $j$ let $x _ { i j } = 1$ if bidder $j$ is assigned to slot i and zero otherwise. The object is to choose $\boldsymbol { x } _ { i j } \mathrm { \widetilde { s } }$ to solve the following:

$$
\max \sum_ {i = 1} ^ {k} \sum_ {j = 1} ^ {n} \alpha_ {i j} v _ {j} x _ {i j}\tag{28.1}
$$

$$
\text { s.t. } \quad \sum_ {j = 1} ^ {n} x _ {i j} \leq 1 \quad \forall i = 1, \dots , k\tag{28.2}
$$

$$
\sum_ {i = 1} ^ {k} x _ {i j} \leq 1 \quad \forall j = 1, \dots , n\tag{28.3}
$$

$$
x _ {i j} \geq 0 \quad \forall i = 1, \ldots , k, \forall j = 1, \ldots , n\tag{28.4}
$$

This is equivalent to finding a maximum-weight perfect matching in a bipartite graph and hence can be solved in polynomial time. In fact, because the constraint matrix of this linear program is totally unimodular, it will have an optimal solution that is integral. Any feasible integer solution is called an assignment.

A single computation of the maximum weight assignment is sufficient to determine both the allocation and the generalized Vickrey payments. This is because the Vickrey payments lie in the dual to the above linear program. To write down the dual, let $p _ { i }$ be the dual variable associated with (28.2) and $q _ { j }$ the dual associated with (28.3).

$$
\min \quad \sum_ {i = 1} ^ {k} p _ {i} + \sum_ {j = 1} ^ {n} q _ {j}\tag{28.5}
$$

$$
\mathrm{s.t.} \quad p _ {i} + q _ {j} \geq \alpha_ {i j} v _ {j} \quad \forall i = 1, \dots , k, \quad \forall j = 1, \dots , n\tag{28.6}
$$

$$
p _ {i}, q _ {j} \geq 0 \quad \forall i = 1, \dots , k, \quad \forall j = 1, \dots , n\tag{28.7}
$$

Here $p _ { i }$ can be interpreted as the expected payment (CTR times price per click) of the bidder obtaining slot i, and $q _ { j }$ as the profit of bidder $j$ . The objective in this program is to minimize the bidders’ and auctionee $\mathrm { \nabla ^ { \cdot } s }$ profits combined. Among all optimal dual solutions, pick the one that minimizes $\textstyle \sum _ { i = 1 } ^ { k } p _ { i }$ . The corresponding $p _ { i }$ is the price that the generalized Vickrey auction would set for slot $i .$

In the special case when the CTRs are bidder independent $( \mathrm { i . e . , } \alpha _ { i j } = \mu _ { i } )$ there is a particularly simple algorithm, called the Northwest corner rule, to find the maximum weight assignment. Assign the bidder with the highest value per click to the top slot, the bidder with the second highest value per click to the second slot, and so on. In the Economics literature this is called an assortative assignment.

If one objects to the sealed bid nature of the generalized Vickrey auction there are ascending implementations available.

Interestingly, neither of these auctions corresponds to the GFP or GSP auctions. In particular, bidding truthfully is not an equilibrium of either the GFP or GSP auctions. It is interesting to observe that Google’s promotional material touts their auction as a modification of Vickrey’s sealed bid auction for a single item (which it is) and concluding, therefore, that bidding sincerely is the correct thing to do (which it is not). A similar claim was made with respect to their auction used to sell shares of their IPO. They are not the first and quite possibly not the last to make such claims. For example, the financial services firm Hambrecht, which pioneered the use of auctions to sell IPO’s in 1998, says that their auction design is based on the Vickrey auction fo a single good. While the Hambrecht auction does specialize to the Vickrey auction for a single good, it does not inherit the attractive properties of the Vickrey auction when applied to multiple units.<sup>2</sup>

To see why one must be careful when generalizing the Vickrey auction to the sale of more than one unit, suppose that there are three bidders with $v _ { 1 } > v _ { 2 } > v _ { 3 }$ and two slots. Also, suppose that $\alpha _ { i j } = \mu _ { i }$ with $\mu _ { 1 } > \mu _ { 2 }$ . If one were to auction off the top slot only, by an English ascending auction, each bidder would remain in as long as at the current price their surplus is nonnegative. So, if the current price on the top slot is $p _ { 1 }$ bidder j remains active if $\mu _ { 1 } ( v _ { j } - p _ { 1 } ) \geq 0$ . Hence the auction ends at a price $p _ { 1 }$ where $\mu _ { 1 } ( v _ { 2 } - p _ { 1 } ) = 0$ , i.e., $p _ { 1 } = v _ { 2 }$ . Now suppose that both slots are available but we will auction off the top slot first followed by the second slot. Let $p _ { 1 }$ be the current price of slot 1, $p _ { 2 } = 0$ the current price of slot 2. Now bidder $j$ will remain active in the auction for the top slot provided their surplus from the top slot is at least as large the surplus they could get from the second slot (which is currently priced at zero). That is,

$$
\mu_ {1} (v _ {j} - p _ {1}) \geq \mu_ {2} (v _ {j} - 0) \Rightarrow p _ {1} \leq \left(1 - \frac {\mu_ {2}}{\mu_ {1}}\right) v _ {j}.
$$

Therefore the auction on the top slot terminates at a price of $\begin{array} { r } { ( 1 - \frac { \mu _ { 2 } } { \mu _ { 1 } } ) v _ { 2 } < v _ { 2 } } \end{array}$ . The point is that the presence of a second slot lowers the price at which a bidder on the top slot will drop out of the auction on the top slot. The generalized Vickrey auction incorporates this change in the outside option of a bidder to ensure truthful bidding. The GSP auction does not. The generalized Vickrey auction, however, would allocate the top slot to bidder 1 and charge her $\begin{array} { r } { ( 1 - \frac { \mu _ { 2 } } { \mu _ { 1 } } ) v _ { 2 } } \end{array}$ and the second slot to bidder 2 and charge her $v _ { 3 }$

As noted above, the GFP and GSP are special cases of what have been called ranking auctions. Bids (the reported $\boldsymbol { v } _ { j } \mathbf { \prime } { \mathbf { s } } )$ are weighted (weights are independent of the bids) and then ranked in the descending order. The highest ranked bidder gets the top slot, the second highest ranked bidder gets the second slot, and so on. The higher the bid the higher the slot one obtains (other bids held fixed). Since the assignment of bidders to slots is monotonic in the bid (other bids held fixed) it follows from standard results (see Section 9.36 of Chapter 9 for example) that there exists a payment rule that will make truthful bidding an equilibrium of the resulting auction. That payment rule is described, for example, in Section 13.1.2 of Chapter 13. Let $x _ { j } ( b | b _ { - j } )$ denote the expected click through rate for agent j when she bids b, given the profile of other bids is $b _ { - j }$ . Then the payment $P _ { j } ( b | b _ { - j } )$ she must make to ensure incentive compatibility is given by

$$
P _ {j} (b \mid b _ {- j}) = b x (b \mid b _ {- j}) - \int_ {0} ^ {b} x (t \mid b _ {- j}) d t.\tag{28.8}
$$

These ranking auctions are, in general, neither efficient nor revenue maximizing. (Though in the exercises, we explore a special case ranking that is efficient.) The payment rules associated with the GFP and GSP are not such as to induce truthful bidding as an equilibrium.

## 28.3.2 Equilibrium Properties

The fact that neither the GFP nor GSP is incentive compatible does not imply that they are inefficient or suboptimal in terms of revenue. It is possible that the equilibrium outcomes of both these auctions may be efficient or revenue maximizing. To identify the revenue and efficiency properties of these auctions, it is necessary to determine their equilibria.

The GFP auction does not admit a pure strategy full-information equilibrium but does admit a pure strategy Bayes-Nash symmetric equilibrium. The argument is identical to that of the sealed bid first price auction for a single good. The equilibrium bid functions are monotonic in the value. Therefore the equilibrium allocation of bidders to slots is the same as in the efficient allocation. Hence, by the revenue equivalence theorem, the symmetric equilibrium is efficient.

The efficiency of the GFP (in a Bayesian setting) lends it some appeal but this is where the “static” assumption has bite. In a dynamic setting, the absence of a pure strategy full-information equilibrium encourages bidders to constantly adjust their bids from one period to the next. This produces fluctuations in the bids over time and it has been argued that these fluctuations resulted in significant inefficiencies.

To date nothing is known about the Bayesian equilibrium of the GSP auction. Assume for simplicity that CTRs are bidder-independent, so $\alpha _ { i j } = \mu _ { i }$ , and that all weights are set to 1. The analysis in this section generalizes straightforwardly to the case where CTRs are separable (i.e., $\alpha _ { i j } = \mu _ { i } \beta _ { j } )$ and agents are assigned arbitrary weights $w _ { j }$ . These extensions are developed in the exercises.

In this case one can show that the GSP is efficient under full information and a restricted notion ofequilibrium called locally envy-free. An assignment x is called locally envy-free if there exist prices, $\{ p _ { i } \}$ }, one for each slot, such that for all $i , j$ with $x _ { i j } = 1$

$$
\mu_ {i} v _ {j} - p _ {i} \geq \mu_ {i - 1} v _ {j} - p _ {i - 1}\tag{28.9}
$$

and

$$
\mu_ {i} v _ {j} - p _ {i} \geq \mu_ {i + 1} v _ {j} - p _ {i + 1}\tag{28.10}
$$

In words, if bidder j is assigned to slot i, then she prefers slot i to the slot just above her and the slot just below her.

Theorem 28.1 An assignment $x ^ { * }$ is optimal ifand only ifit is locally envy-free.

proof Suppose first that $x ^ { * }$ is locally envy-free and let $p ^ { * }$ be the corresponding price vector. It suffices to prove that the assignment $x ^ { * }$ is assortative. Let $j$ be such that $x _ { i j } ^ { * } = 1$ and $j ^ { \prime }$ such that $x _ { i + 1 , j ^ { \prime } } = 1$ . To show that the assignment is assortative, we must show that $v _ { j } \geq v _ { j { ' } }$ . From the property of being locally envy-free, we have

$$
\mu_ {i} v _ {j} - p _ {i} ^ {*} \geq \mu_ {i + 1} v _ {j} - p _ {i + 1} ^ {*}
$$

and

$$
\mu_ {i + 1} v _ {j ^ {\prime}} - p _ {i + 1} ^ {*} \geq \mu_ {i} v _ {j ^ {\prime}} - p _ {i} ^ {*}.
$$

Adding them together yields

$$
(\mu_ {i} - \mu_ {i + 1}) (v _ {j} - v _ {j ^ {\prime}}) \geq 0.
$$

Since $\mu _ { i } > \mu _ { i + 1 }$ it follows from this inequality that $v _ { j } \geq v _ { j { ' } }$

Now let $x ^ { * }$ be an optimal assignment. Let $( p ^ { * } , q ^ { * } )$ denote an optimal dual solution. It suffices to show that $( \boldsymbol { x } ^ { * } , \boldsymbol { p } ^ { * } )$ is locally envy-free. Consider a pair $( r , j )$ such that $x _ { r j } ^ { * } = 1$ . Complementary slackness and dual feasibility implies that $\mu _ { r } v _ { j } - p _ { r } ^ { * } = \stackrel { } { q } _ { j } ^ { * } = \operatorname* { m a x } _ { i } \{ \mu _ { i } v _ { j } - p _ { i } ^ { * } \}$ . Therefore

$$
\mu_ {r} v _ {j} - p _ {r} ^ {*} \geq \max \{\mu_ {r - 1} v _ {j} - p _ {r - 1} ^ {*}, \mu_ {r + 1} v _ {j} - p _ {r + 1} ^ {*} \}.
$$

Theorem 28.2 The GSP has afull information equilibrium that yields an allocation that is locally envy-free.

proof Order the bidders so that $v _ { 1 } \geq v _ { 2 } \geq \cdot \cdot \cdot \geq v _ { n }$ . Let $p _ { i } ^ { * }$ be the Vickrey price of slot i. Let bidder 1 bid $b _ { 1 } = v _ { 1 }$ and each bidder $j \geq 2$ bids $\begin{array} { r } { b _ { j } = \frac { p _ { j - 1 } ^ { * } } { \mu _ { j - 1 } } } \end{array}$ First we show that under the rules of the GSP, bidder 1 is assigned to slot 1, bidder 2 to slot 2, and so on. To do this, it suffices to show that $b _ { j - 1 } \geq b _ { j }$ . Since the optimal assignment is locally envy-free, we have

$$
\mu_ {j} v _ {j} - p _ {j} ^ {*} \geq \mu_ {j - 1} v _ {j} - p _ {j - 1} ^ {*}.
$$

Therefore

$$
v _ {j} - \frac {p _ {j} ^ {*}}{\mu_ {j}} \geq \frac {\mu_ {j - 1}}{\mu_ {j}} v _ {j} - \frac {p _ {j - 1} ^ {*}}{\mu_ {j}},
$$

which implies

$$
b _ {j - 1} = \frac {p _ {j - 1} ^ {*}}{\mu_ {j - 1}} \geq \frac {p _ {j - 1} ^ {*}}{\mu_ {j}} \geq \frac {p _ {j} ^ {*}}{\mu_ {j}} + \left(\frac {\mu_ {j - 1}}{\mu_ {j}} - 1\right) v _ {j} \geq \frac {p _ {j} ^ {*}}{\mu_ {j}} = b _ {j}.
$$

Hence if each bidder j bids $b _ { j }$ the GSP returns the optimal assignment. It is also easy to see that bidder $j \leq m$ pays $p _ { j } ^ { * }$ for their slot. Bidder $j > m$ pays zero.

Since each bidder pays their Vickrey price and receives the slot they would have under the efficient allocation, no bidder has a unilateral incentive to change thei bid. Therefore we have an equilibrium that, from Theorem 1, is envy-free.

Absent the recurrent nature of keyword auctions, they are similar to what are known as condominium auctions. In a condominium auction, bidders are interested in purchasing a condominium in a building. The condominiums are identical except for their height above the ground, the side of the building they are located on, etc. If all bidders have identical preferences over the condominiums; i.e., everyone prefers to be on a higher floor, they coincide with keyword auctions.

## 28.4 Dynamic Aspects

Since these auctions are repeated with great frequency, one should properly model them as repeated games of incomplete information. The set of equilibria of such games is quite rich and complicated, even when restricted to the setting considered here. A full treatment of this case will not be given here. Rather we mention two phenomena that arise in this setting.

One is known as bid rotation. This occurs when competing bidders take turns at winning the auction. In our context this might mean bidders take turns at occupying the top slot. If bidders are short lived, this is unlikely to be a problem, if not, this will lower the auctioneers revenue.

Another possibility that repetition makes possible is vindictive bidding. In the GSP auction one’s bid determines the payment of the bidder in the slot above and not one’s own. Therefore one can increase the payment of the bidder in the slot above by raising one’s bid without affecting one’s own payment. This may be beneficial if the bidder in the slot above is a competitor with a limited budget for advertising. In a dynamic environment this encourages a bidder to constantly adjust their bids so as to inflict or avoid damage upon or from their competitor.

Even if one could ignore strategic considerations, a problem remains. The online nature of the auctions in sponsored search complicates the computation of an efficient allocation. Below we describe one model that addresses this difficulty.

## 28.4.1 The Online Allocation Problem

In this model, the search engine receives the bids of advertisers and their maximum budget for a certain period (e.g., a day). As users search for these keywords during the day, the search engine assigns their advertisement space to advertisers and charges them the value of their bid for the impression of the advertisement.<sup>3</sup> For simplicity of notation we assume that each page has only one slot for advertisements. The objective is to maximize total revenue while respecting the budget constraint of the bidders. Note that in this model bidders pay their bid which is counter to practice. On the other hand, budget constraints that apply across a set of keywords, a real-world feature, are part of the model.

Let n be the number of advertisers and m the number of keywords. Suppose that advertiser $j$ has a bid of $b _ { i j }$ for keyword i and a total budget of $B _ { j }$ . In this context, it is reasonable to assume that bids are small compared to budgets, i.e., $b _ { i j } \ll B _ { j }$

If the search engine has an accurate estimate of $r _ { i }$ , the number of people searching for keyword i for all $1 \leq i \leq m$ , then it is easy to approximate the optimal allocation using a simple linear program. Let $x _ { i j }$ be the total number of queries on keyword i allocated to bidder $j$ . The linear program is

$$
\begin{array}{l l} \max & \sum_ {i = 1} ^ {m} \sum_ {j = 1} ^ {n} b _ {i j} x _ {i j} \\ \text { s.t. } & \sum_ {j = 1} ^ {n} x _ {i j} \leq r _ {i} \quad \forall 1 \leq i \leq m \\ & \sum_ {i = 1} ^ {m} b _ {i j} x _ {i j} \leq B _ {j} \quad \forall 1 \leq j \leq n \\ & x _ {i j} \geq 0 \quad \forall 1 \leq i \leq m, \quad \forall 1 \leq j \leq n \\ \min & \sum_ {j = 1} ^ {n} B _ {j} \beta_ {j} + \sum_ {i = 1} ^ {m} r _ {i} \alpha_ {i} \\ \text { s.t. } & \alpha_ {i} + b _ {i j} \beta_ {j} \geq b _ {i j} \quad \forall 1 \leq i \leq m, \forall 1 \leq j \leq n \\ & \beta_ {j} \geq 0 \quad \forall 1 \leq j \leq n \\ & \alpha_ {i} \geq 0 \quad \forall 1 \leq i \leq m \end{array}\tag{28.11}
$$

By complementary slackness, in an optimal solution, advertiser $j$ is assigned to keyword i if $( 1 - \beta _ { j } ) b _ { i j } = \mathrm { { m a x } } _ { 1 \leq k \leq n } ( 1 - \beta _ { k } ) b _ { i k }$ . Using this property, the search engine can use the solution of the dual linear program to find the optimum allocation: every time a user searches for keyword i, the search engine allocates its corresponding ad vertisement space to the bidder $j$ with the highest $b _ { i j } ( 1 - \beta _ { j } )$ . In other words, the bid of advertiser j will be scaled down by $1 - \beta _ { j }$

Now $\beta _ { j }$ represents rate of change of the optimal objective function value of (28.11) for a sufficiently small change in the right-hand side of the corresponding constraint. In other words, if advertiser $j ^ { \circ } \mathbf { s }$ budget were to increase by $\Delta .$ , the optimal objective function value would increase by $\beta _ { j } \Delta$ . Equivalently, it is the opportunity cost of consuming agent $j ^ { \circ } \mathrm { s }$ budget. Hence, if we allocate keyword $i$ to agent now we obtain an immediate ‘payoff’ of $b _ { i j }$ . However, this consumes $b _ { i j }$ of the budget, which imposes an opportunity cost of $\beta _ { j } b _ { i j }$ . Therefore, it makes sense in the optimal solution to (28.11) to assign keyword i to j provided $b _ { i j } - \beta _ { j } b _ { i j } > 0$

In practice, a good estimate of the frequencies of all search queries is unavailable. Queries arrive sequentially and the search engine must instantly decide to allocate thei advertisement space to bidders without knowledge of the future queries. Therefore, what is needed is a dynamic procedure for allocating bidders to keywords that are queried. We describe one such procedure and analyze its performance within the usual competitive ratio framework. Specifically, we compare the revenue achieved by a dynamic procedure that does not know the $r _ { i } \mathrm { { ' s } }$ in advance, with the revenue that could be achieved knowing the $r _ { i }$ ’s advance. The revenue in this second case is given by the optimal objective function value of the program (28.11).

The obvious dynamic procedure to consider is a greedy one: among the bidders whose budgets are not exhausted, allocate the query to the one with the highest bid. It is easy to see that this approach is equivalent to setting all $\beta _ { j } \mathrm { ^ { \circ } s }$ to 0.

The greedy procedure is not guaranteed to find the optimum solution. It is easy to construct a simple example with two bidders and two keywords in which the revenue of the greedy algorithm is as small as half of the optimum revenue. For example, suppose two bidders each with a budget of \$2. Assume that $b _ { 1 1 } = 2 , b _ { 1 2 } = 2 - \epsilon , b _ { 2 1 } = 2$ and $b _ { 2 2 } = \epsilon$ . If query 1 arrives before query 2, it will be assigned to bidder 1. Then bidder $1 { \mathrm { : } } { \mathrm { } } $ budget is exhausted. When query 2 arrives, it is assigned to bidder 2. This produces an objective function value of $2 + \epsilon$ . The optimal solution would assign query 2 to bidder 1 and query 1 to bidder 2, yielding an objective function value of 4. The problem with the greedy algorithm is that, unlike the solution to (28.11), it ignores the opportunity cost of assigning a query to a bidder.

One can prove that the revenue of greedy algorithm is at least half of the optimum revenue for any instance. In the standard terminology of online algorithms, the competitive ratio of greedy algorithm is $1 / 2$ . Can one do better in terms of competitive ratio? Yes. One does so by trying to dynamically estimate the opportunity cost , i.e., the $\beta _ { j } \mathrm { ^ { \prime } s } ,$ of assigning a query to a bidder. This has the effect of spreading the bidders expenditures over time. The effect is called “budget smoothing,” and is a feature that some search engines offer their advertisers.

The following modification of the greedy algorithm adaptively updates the $\beta _ { j } \mathrm { ^ { \circ } s }$ as a function of the bidders spent budget. Let

$$
\phi (x) = 1 - e ^ {x - 1}.
$$

The algorithm sets $\beta _ { j } = 1 - \phi ( f _ { j } )$ , where $f _ { j }$ is the fraction of the budget of bidder $j .$ which has been spent.

Algorithm 1. Every time a query i arrives, allocate its advertisement space to the bidder $j .$ , who maximizes $b _ { i j } \phi ( f _ { j } )$ , where $f _ { j }$ is the fraction of the bidder $j ^ { \circ } \mathbf { s }$ budget which has been spent so far.

The revenue of this algorithm is at least $1 - 1 / e$ of the optimum revenue. It is also possible to prove that no deterministic or randomized algorithm can achieve a better competitive ratio.

## Theorem 28.3 The competitive ratio ofAlgorithm 1 is $1 - 1 / e$

We outline the main ideas in the proof of the theorem. Let k be a sufficiently large number used for discretizing the budgets of the bidders. We say that an advertiser is of type $j$ if she has spent within $( \frac { \breve { j } - 1 } { k } , ~ \frac { j } { k } ]$ fraction of her budget so far. Let $s _ { j }$ be the total budget of type j bidders. For $i = 0 , 1 , \ldots , k$ , define $w _ { i }$ to be the amount of money spent by all the bidders from the interval $( \frac { i - 1 } { k } , \ : \frac { i } { k } ]$ of their budgets.Also define

the discrete version of function $\phi .$ ,

$$
\Phi (s) = 1 - \left(1 - \frac {1}{k}\right) ^ {k - s}.\tag{28.12}
$$

It is easy to see that when k tends to infinity $\Phi ( s ) \to \phi ( { \frac { s } { k } } )$ . Let $O P T$ be the solution of the optimal off-line algorithm (i.e., the solution of the optimization program (28.11)). For simplicity, assume that the optimal algorithm spends all of the budget of the bidders. We have the following lemma.

Lemma 28.4 At the end ofthe algorithm, this inequality holds:

$$
\sum_ {i = 0} ^ {k} \Phi (i) s _ {i} \leq \sum_ {i = 0} ^ {k}. \Phi (i) w _ {i}\tag{28.13}
$$

proof Consider the time that query $q$ arrives. Suppose that $O P T$ allocates q to a bidder of current type t, whose type at the end of the algorithm will be $t ^ { \prime } .$ Let $b _ { \mathrm { o p t } }$ and $b _ { \mathrm { a l g } }$ be the amount of money that $O P T$ and the algorithm get from bidders for $q$ . Let i be the type of the bidder that the algorithm allocates the query. We have

$$
\Phi (t ^ {\prime}) b _ {\text { opt }} \leq \Phi (t) b _ {\text { opt }} \leq \Phi (i) b _ {\text { alg }}.\tag{28.14}
$$

Now summing the inequality above over all the queries, the left-hand side of (28.14) contributes to the sum $\sum _ { i } \Phi ( i ) s _ { i }$ , and the right-hand side contributes to $\sum \Phi ( i ) w _ { i }$ . So the lemma follows.

Now, we are ready to prove the Theorem 28.3.

proof By definition $\begin{array} { r } { w _ { i } \leq \frac { 1 } { k } \sum _ { j = i } ^ { k } s _ { j } } \end{array}$ . Using Lemma 28.4,

$$
\sum_ {i = 0} ^ {k} \Phi (i) s _ {i} \leq \frac {1}{k} \sum_ {i = 0} ^ {k} \Phi (i) \sum_ {j = i} ^ {k} s _ {j}.
$$

Changing the order of the sums and computing the sum of the geometric series, we have

$$
\begin{array}{l} \sum_ {i = 0} ^ {k} \Phi (i) s _ {i} \leq \frac {1}{k} \sum_ {i = 0} ^ {k} \Phi (i) \sum_ {j = i} ^ {k} s _ {j} \\ \quad \leq \frac {1}{k} \sum_ {i = 0} ^ {k} \bigg (\sum_ {j = 0} ^ {i} \Phi (i) \bigg) s _ {i} \\ \quad \leq \sum_ {i = 0} ^ {k} \bigg (\frac {i}{k} + \Phi (i) - \Phi (0) + O \bigg (\frac {1}{k} \bigg) \bigg) s _ {i} \\ \quad \leq \sum_ {i = 0} ^ {k} \frac {i}{k} s _ {i} - \bigg (\Phi (0) - O \bigg (\frac {1}{k} \bigg) \bigg) \sum_ {i = 0} ^ {k} s _ {i} + \sum_ {i = 0} ^ {k} \Phi (i) s _ {i}, \end{array}
$$

which yields

$$
\left(\Phi (0) - O \left(\frac {1}{k}\right)\right) \sum_ {i = 0} ^ {k} s _ {i} \leq \sum_ {i = 0} ^ {k} \frac {i}{k} s _ {i}.
$$

Note that as k goes to infinity the left-hand side tends to $( 1 - \textstyle { \frac { 1 } { e } } ) O P T$ The right-hand side is equal to the revenue of the algorithm. So the theorem follows.

The same algorithm can be applied when multiple advertisement can appear with the result of a query or when advertisers enter at different times. At present, the equilibrium properties of this allocation rule are unknown.

## 28.5 Open Questions

We close this chapter with a brief review of important issues not directly addressed in this chapter.

While our discussion has focused on existing mechanisms, one should not conclude that there is no room for improvement in their design. For example, there is debate over the role of the budget constraints in these auction. In many cases they do not appear to be hard constraints as bidders frequently adjust them. A bidder can also “expand” their budget simply by lowering their bid and paying less per click. Some argue that the budget constraint is merely a convenient way to express other desires. For example, limiting one’s exposure or spreading one’s advertising over a longer period. All of this suggests the need for richer bidding models. Ones that might allow bidders to express decreasing marginal value for clicks, or distinct values for traffic from certain geographic regions, demographic profiles, etc., support greater allocative efficiency, though pose a significant burden in terms of computational and elicitation costs.

When advertiser payments are based on user clicks, search engines must invest in the task of detecting and ignoring robot clicks, spam clicks as well as clicks from an advertiser trying to impose costs on their competitor or from an affiliate who actually benefits monetarily from additional clicks. For this reason there is interest in exploring alternate pricing conventions. The most compelling is pay per action or conversion. The advertiser pays only if a click results in a sale, for example. This raises new incentive issues associated with tracking sales.

The models in this chapter, as do most analyses in the literature, assume a monopoly search engine with a static user base. This would be an appropriate model if switching costs for advertisers and users were high. In fact, switching costs for many advertisers are low; many advertisers work with both Google and Yahoo! simultaneously, or work with third-party search engine marketers to manage their account across multiple search engines. Switching costs for users are essentially zero: to patronize a different search engine, users need merely type a new address into their web browser.<sup>4</sup> The competitive pressures to retain advertisers able to switch advertisement networks or use multiple networks may cause firms to focus less on extracting the maximum revenue from advertisers possible and more on attracting and retaining advertisers. Similarly, search engines must make trade-off decisions between maximizing current period revenue and attracting and retaining users in the long term. For this reason it would be very instructive to understand the properties of keyword auctions in competition with each other.

The major search engines syndicate their advertisements to affiliate search engines and content providers. For example, Google, through its AdSense program, syndicates advertisements to AOL, MySpace, and thousands of other Web sites. The introduction of affiliates greatly complicates the semantics of bidding and allocation.

We have assumed that CTRs are given. In practice, CTRs are learned over time and can depend on a variety of factors such as bidder identity; advertisement identity and content; user characteristics, including demographics, location, and history; and/or page context including other advertisements and algorithmic results. Learning CTRs poses an explore/exploit trade-off: the auctioneer can exploit known high-CTR ad vertisements, or explore new advertisements or infrequently shown advertisements to uncover even higher-CTR advertisements. The auctioneer’s CTR estimate may differ from the bidder’s estimate; in particular, the auctioneer usually has more contextual information to learn from.

In this chapter, we have focused on the auctioneer’s mechanism design problem. The advertiser’s bidding optimization problem is also challenging and the focus of a great deal of commercial and research activity.

## 28.6 Bibliographic Notes

The growth of paid placement has attracted recent research on this topic. Hoffman and Novak (2000) discuss the trend in Internet advertising toward per-click pricing rather than the traditional per-impression model. A good discussion of the practice of sponsored search is available on the Web at http://searchenginewatch.com/ webmasters/paid.html.

Computing the explicit form of incentive compatible payments for ranking auctions is carried out in Aggarwal et al. (2006) and Iyengar and Kumar (2006). The Bayesian equilibrium of the GFP is derived in Lahaie (2006). The details of the revenue maximizing auction for (static) slot auctions is derived in Feng (2005) and Iyengar and Kumar (2006). The envy-free analysis of the static model is due to Edelman et al. (in press). A similar analysis can be found in Varian (in press). The latter paper shows how upper and lower bounds on bidders’ actual values can be derived given their bids. Feng et al. (2006) explore four ranking algorithms via simulation. All of these results would apply to condominium auctions as well; see Burguet (2005) for a discussion of condominium auctions.

The Northwest corner rule for the assignment problem dates back to Monge (1981). Ascending implementations of the Vickrey auction for the static model can be found in Crawford and Knoer (1981) and Demange, Gale, and Sotomayor (1986) (which is a variant of the Hungarian algorithm for solving the assignment problem). The auction of Demange, Gale, and Sotomayor was dubbed, in Edelman et al. (in press), the generalized English auction.

The online allocation problem studied in Section 28.4.1 is proposed and analyzed by Mehta et al. (2005). This problem is a generalization of the online bipartite matching problem studied by Karp et al. (1990) and Kalyanasundaram and Pruhs (2000). More recently Buchbinder et al. (2006) gave a primal-dual algorithm and analysis for the problem given in Mehta et al. They also extended that framework to scenarios in which additional information is available, yielding improved worst-case competitive factors.

Mahdian et al. (2006) study the online allocation problem when the search engine has a somewhat reliable estimate of the number of users searching for a keyword everyday. Mahdian and Saberi (2006) study multiunit auctions for perishable goods, in a setting where the supply arrives online. They motivate their model by its application to sponsored search. Abrams (2006) and Borgs et al. (2005) design multiunit auctions for budget-constrained bidders, which can be interpreted as slot auctions, with a focus on revenue optimization and truthfulness. For a discussion of vindictive bidding and some of the dynamic aspects of slot auctions see Asdemir (2006) and Zhou and Lukose (2006).

Weber and Zheng (2006) study the implementation of paid placement strategies, and find that the revenue-maximizing search engine design bases rankings on a weighted average of relative quality performance and bid amount. Hu (2003) uses contract theory to show that performance-based pricing models can give the publisher proper incentives to improve the effectiveness of advertising campaigns. Rolland and Patterson (2003) propose a methodology, using expert systems to improve the matching between advertisers and Web users.

Besides the optimal ranking mechanism, the search engine must also choose the number of paid slots by finding the optimal trade-off between sponsorship and user retention. Bhargava and Feng (2002) provide a theoretical model to explain and analyze this trade-off.

The problem of learning CTRs is nontrivial and presents an explore/exploit tradeoff. Pandey and Olston (2006) formulate the problem as an appropriate multiarmed bandit optimization; Gonen and Pavlov (2007) derive a bandit optimization algorithm that retains incentive compatibility for bidders.

Several authors explore the advertiser’s bidding optimization problem (Borgs et al., 2005; Cary et al., 2007; Kitts et al., 2005; Kitts and LeBlanc, 2004; Rusmevichientong and Williamson, 2006). Kitts et al. (2005) provide evidence that the first slot does not have an appreciably lower conversion rate than the second slot as some advertisers believe.

## Bibliography

Z. Abrams. Revenue maximization when bidders have budgets. In Proc. Symp. on DiscreteAlgorithms, Miami, FL, 2006.

G. Aggarwal, A. Goel, and R. Motwani. Truthful auctions for pricing search keywords. In Proc. 7th ACM Conf. on Electronic Commerce, Ann Arbor, MI, 2006.

K. Asdemir. Bidding patterns in search engine auctions. In Proc. 2nd Workshop on Sponsored Search Auctions, Ann Arbor, MI, 2006.

H.K. Bhargava and J. Feng. Preferential placement in internet search engines. In Proc. 11th World Wide Web Conf., Honolulu, HI, 2002.

C. Borgs, J. Chayes, O. Etesami, N. Immorlica, K. Jain, and M. Mahdian. Bid optimization in online advertisement auctions. Preprint, 2005.

C. Borgs, J. Chayes, N. Immorlica, M. Mahdian, and A. Saberi. Multi-unit auctions with budget constrained bidders. In Proc. 6th Conf. Electronic Commerce, Vancouver, British Columbia, Canada, 2005.

N. Buchbinder, K. Jain, and J. Naor. Online primal-dual algorithms for maximizing ad-auction revenue. Preprint, 2006.

R. Burguet. The condominium problem; auctions for substitutes. Rev. Econ. Design, 9, 2005.

M. Cary, A. Das, B. Edelman, I. Giotis, K. Heimerl, A. Karlin, C. Mathieu, and M. Schwarz. Greedy bidding strategies for keyword auctions. Preprint, 2007.

V.P. Crawford and E.M. Knoer. Job matching with heterogeneous firms and workers. Econometrica, 49(2):437–450, 1981.

G. Demange, D. Gale, and M. Sotomayor. Multi-item auctions. J. Political Econ., 94(4):863–872, 1986.

B. Edelman, M. Ostrovsky, and M. Schwarz. Internet advertising and the Generalized Second Price auction: Selling billions of dollars worth of keywords. Amer. Econ. Review, In press.

J. Feng. Optimal mechanism for selling a set of commonly ranked objects. Working paper, University of Florida, February 2005.

J. Feng, H.K. Bhargava, and D.M. Pennock. Implementing sponsored search in Web search engines: Computational evaluation of alternative mechanisms. INFORMS J. Computing, 2006, In press.

R. Gonen and E. Pavlov. An incentive compatible multi armed bandit mechanism. Preprint, 2007.

D.L. Hoffman and T.P. Novak. How to acquire customers on the Web. Harv. Busin. Rev., 78(3), May–June 2000.

Y.J. Hu. Performance-based pricing models in online advertising. Technical report, Sloan School of Management, MIT, 2003.

G. Iyengar and A. Kumar. Characterizing optimal keyword auctions. In Proc. 2nd Workshop on Sponsored Search Auctions, Ann Arbor, MI, 2006.

B. Kalyanasundaram and K.R. Pruhs. An optimal deterministic algorithm for online b-matching. Theor. Comp. Sci., 233(1–2):319–325, 2000.

R. Karp, U. Vazirani, and V. Vazirani. An optimal algorithm for online bipartite matching. In Proc. 22nd Symp. Theory ofComputing, Baltimore, MD, 1990.

B. Kitts, P. Laxminarayan, B. LeBlanc, and R. Meech. A formal analysis of search auctions including predictions on click fraud and bidding tactics. In Proc. 1st Workshop on Sponsored Search Auctions at the ACM Conf. on Electronic Commerce, Vancouver, British Columbia, Canada, 2005.

B. Kitts and B. LeBlanc. Optimal bidding on keyword auctions. Electronic Markets, 14(3):186–201, 2004.

S. Lahaie. An analysis of alternative slot auction designs for sponsored search. In Proc. 7th Conf. on Electronic Commerce, Ann Arbor, MI, 2006.

M. Mahdian, H. Nazerzadeh, and A. Saberi. Allocating online advertisement space with unreliable estimates. In Proc. 8th ACM Conf. on Electronic Commerce, San Diego, CA, 2007.

M. Mahdian and A. Saberi. Multiunit auctions with unknown supply. In Proc. 7th ACM Conf. on Electronic Commerce, Ann Arbor, MI, 2006.

A. Mehta, A. Saberi, U. Vazirani, and V. Vazirani. AdWords and generalized on-line matching. In Proc. 46th Annual Symp. on Fdns. ofComp. Sci., 2005.

G. Monge. Sur la theorie des d´ eblais et des remblais´ . Memoires de l’acad´ emie de Paris, 1781.´

S. Pandey and C. Olston. Handling advertisements of unknown quality in search advertising. In Neural Information Processing Systems, 2006.

E. Rolland and R.A. Patterson. Classification in online pay-for-performance advertising. In Proc. 13th Annual Workshop On Information Technologies and Systems, Seattle, WA, 2003.

P. Rusmevichientong and D.P. Williamson. An adaptive algorithm for selecting profitable keywords for search-based advertising services. In Proc. 7th ACM Conf. on Electronic Commerce, pp. 260– 269, Ann Arbor, MI, 2006.

H.R. Varian. Position auctions. Intl. J. Industrial Organization, in press.

T.A. Weber and Z. Zheng. A model of search intermediaries and paid referrals. OPIM Working Paper 02-12-01, Wharton School, April 2002.

Y. Zhou and R. Lukose. Vindictive bidding in keyword auctions. In Proc. 2nd Workshop on Sponsored Search Auctions

## Exercises

28.1 Consider the model of keyword auctions where the CTR of agent j in slot i is $\mu _ { j }$ . Is every full-information equilibrium of the GSP locally envy-free?

28.2 Consider the model of keyword auctions where the CTR of agent j in slot i is $\mu _ { i } \beta _ { j } ;$ i.e.; the CTR is separable into a bidder effect $\beta _ { j }$ and a position effect $\mu _ { j }$ Suppose also that $\mu _ { 1 } > \mu _ { 2 } > \dots > \mu _ { m }$ . Give a simple algorithm for determining the efficient allocation of bidders to slots. Derive the payment rule implied by the VCG mechanism for this environment.

28.3 In the model of the previous exercise, suppose also that the auctioneer assigns a weight $w _ { j } \equiv w _ { j } ( \beta _ { j } )$ to each bidder; weights may depend on the bidder effects, but not on their bids. Suppose bidders are assigned to slots by decreasing order of their scores $w _ { j } b _ { j }$ . Use formula (28.8) to derive the payment rule that combined with the allocation rule just described would yield an incentive compatible mechanism.

28.4 Consider the model of keyword auctions where the CTR of agent j in slot i is $\mu _ { i } \beta _ { j } ;$ i.e., the CTR is separable into a bidder effect $\beta _ { j }$ and a position effect $\mu _ { j }$ . The auctioneer sets weights $w _ { j } = \beta _ { j }$ , and a bidder pays the lowest amount necessary to retain his position.

(a) Give the inequalities that characterize a full-information (Nash) equilibrium in this model. Strenghten them to give the inequalities for a locally envy-free equilibrium.

(b) Show that in a locally envy-free equilibrium, bidders are ranked in order of decreasing $\beta _ { j } v _ { j }$

(c) From among the set of locally envy-free equilibria, exhibit the one that yield the smallest possible revenue to the auctioneer.

28.5 Consider the model of keyword auctions where the CTR of agent j in slot i is $\mu _ { j }$ . Give an example of where the GFP auction does not admit a pure strategy full-information equilibrium. For simplicity, you may assume a discretized set of allowable bids.

28.6 Consider the online allocation problem discussed in Section 28.4. Show that the competitive ratio of the algorithm remains the same even if the optimum solution does not exhaust all the budgets.