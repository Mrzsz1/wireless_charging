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

# Chapter 10 (MinerU semantic layer)

<!-- source-pages: 264-287; printed-pages: 264-287; mineru-part: algorithmic-game-theory-ch-10-part-011 -->

# Mechanism Design without Money

James Schummer and Rakesh V. Vohra

## Abstract

Despite impossibility results on general domains, there are some classes of situations in which there exist interesting dominant-strategy mechanisms. While some of these situations (and the resulting mechanisms) involve the transfer of money, we examine some that do not. Specifically, we analyze problems where agents have single-peaked preferences over a one-dimensional “public” policy space; and problems where agents must match with each other.

## 10.1 Introduction

The Gibbard–Satterthwaite Theorem (Theorem 9.8) is a Procrustean bed<sup>1</sup> that is escaped only by relaxing its assumptions. In conjunction with the Revelation Principle (Proposition 9.25), it states that on the general domain of preferences, only dictatorial rules can be implemented in dominant strategies (if the range contains at least three alternatives). In this chapter we escape Procrustes by examining dominant strategy

In most applications it is clearly unreasonable to assume that agents’ preferences are completely unrestricted, as was assumed in the voting context of Section 9.2.4. For instance, in situations involving the allocation of goods, including money, one can safely assume that each agent prefers to receive more money (or other goods). As can be seen in the following chapters, the ability for agents to make monetary transfers allows for a rich class of strategy-proof rules.

Nevertheless there are many important environments where money cannot be used as a medium of compensation. This constraint can arise from ethical and/or institutiona considerations: many political decisions must be made without monetary transfers; organ donations can be arranged by “trade” involving multiple needy patients and their relatives, yet monetary compensation is illegal. In this chapter we focus on a few examples of just this kind.

Before proceeding with the examples, we formalize the idea that dominantstrategy implementation is a weaker concept on restricted domains of preferences. In general, a decision problem can be described by these parameters: a set of agents $N = \{ 1 , 2 , \dots , n \}$ , a set of alternatives A, and for each agent $i \in N$ a set of potential preference relations $\mathcal { R } _ { i }$ over the alternatives in $A . ^ { 3 }$ The Gibbard–Satterthwaite Theorem (Theorem 9.8) applies, for example, when each $\mathcal { R } _ { i }$ is the entire set of linear orders on A.

An allocation rule is a function $f \colon \times { \mathcal { R } } _ { i } \to A$ , mapping preferences of the agents into alternatives. It is strategy-proof if its use makes it a weakly dominant strategy for agents to truthfully report their preferences. (See Section 9.4). We observe the following principle.

Consider two decision problems $( N , A , \mathcal { R } _ { 1 } , \ldots , \mathcal { R } _ { n } )$ and $( N , A , \mathcal { R } _ { 1 } ^ { \prime } , \ldots , \mathcal { R } _ { n } ^ { \prime } )$ where $\mathcal { R } _ { i } ^ { \prime } \subseteq \mathcal { R } _ { i }$ for each $i \in N$ . Suppose $f \colon \times { \mathcal { R } } _ { i } \to A$ is a strategy-proof rule for the former problem. Then the restriction of the function $f$ to $( \times \mathcal { R } _ { i } ^ { \prime } )$ ), namely $f | _ { \times \mathcal { R } _ { i } ^ { \prime } }$ defines a strategy-proof rule for the latter problem.

The proof of this is straightforward: on a smaller domain of preferences, strategyproofness is easier to satisfy because it imposes strictly fewer constraints. This simple observation justifies the search for reasonable (or at least nondictatorial) rules for decision problems involving “smaller” domains of preferences than those that yield the Gibbard–Satterthwaite Theorem.

In Section 10.2 we analyze a problem involving a natural domain restriction when agents vote over one-dimensional policies. It is one of the canonical “public $\mathrm { { g o o d } ^ { \prime } }$ settings $( \mathcal { R } _ { i } = \mathcal { R } _ { j }$ for all $i , j \in N )$ in which interesting, strategy-proof rules can be obtained. The analysis here is illustrative of the approach used to characterize such rules in other environments. In Sections 10.3 and 10.4 we analyze matching problems. As opposed to the previous setting, these problems have the feature that each agent cares only about his own private consumption; that is, each $\mathcal { R } _ { i }$ contains only preference relations that are sensitive only to certain dimensions of the alternative space A; hence $\mathcal { R } _ { i } \neq \mathcal { R } _ { j }$ whenever $i \neq j$ . These are examples of what are called “private good” problems. Two kinds of matching problems are analyzed, demonstrating the limits of what can be implemented in dominant strategies in such environments.

## 10.2 Single-Peaked Preferences over Policies

A simple but elegant class of domains involves single-peaked preferences over one dimensional policy spaces. This domain can be used to model political policies, eco nomic decisions, location problems, or any allocation problem where a single poin must be chosen in an interval. The key assumption we make is that agents’ preferences are assumed to have a single most-preferred point in the interval, and that preferences are “decreasing” as one moves away from that peak.

Formally, the allocation space (or policy space) is the unit interval $A = [ 0 , 1 ]$ . An outcome in this model is a single point $x \in A$ . Each agent $i \in N$ has a preference ordering $\succeq _ { i } ( \mathrm { i . e . , a }$ weak order) over the outcomes in [0, 1]. The preference relation $\succeq _ { i }$ is single-peaked if there exists a point $p _ { i } \in A$ (the peak of $\succeq _ { i } )$ ) such that for all $x \in A \setminus \{ p _ { i } \}$ and all $\lambda \in [ 0 , 1 )$ , $( \lambda x + ( 1 - \lambda ) p _ { i } ) \succ _ { i } x . ^ { 4 }$ Let R denote the class of single-peaked preferences.

We denote the peaks of preference relations $\succeq _ { i } , \succeq _ { i } ^ { \prime } , \succeq _ { j }$ , etc., respectively by $p _ { i } , p _ { i } ^ { \prime }$ $p _ { j }$ , etc. Denote a profile (n-tuple) of preferences as $\succeq \in \mathcal { R } ^ { n }$

One can imagine this model as representing a political decision such as an income tax rate, another political issue with conservative/liberal extremes, the location of a public facility on a road, or even something as simple as a group of people deciding on the temperature setting for a shared office. In these and many other examples, the agents have an ideal preferred policy in mind, and would prefer that a decision be made as close as possible to this “peak.”

A rule f: ${ \mathcal { R } } ^ { n }  A$ assigns an outcome $f ( \succeq )$ to any preference profile $\succeq$ . As before, a rule is strategy-proof ifit is a dominant strategy for each agent to report his preferences truthfully when the rule is being used to choose a point.

In contrast to the impossibility result of Gibbard (1973) and Satterthwaite (1975), that obtain on the universal domain of preferences, we shall see that this class of problems admits a rich family of strategy-proof rules whose ranges include more than two alternatives. In fact, the family of such rules remains rich even when one restricts attention (as we do in this chapter) to rules that satisfy the following condition.

We say that a rule f is onto if for all $x \in A$ there exists $\succeq \in \mathcal { R } ^ { n }$ such that $f ( \succeq ) = x$ An onto rule cannot preclude an outcome from being chosen ex ante. It is not without loss of generality to impose this condition. For instance, fix two points x, $y \in [ 0 , 1 ]$ and consider a rule that chooses whichever of the two points is preferred to the other by a majority of agents (and where x is chosen in case of a tie). Such a rule is strategy-proof, but not onto. Similar strategy-proof rules can even break ties between x and y by using preference information about other points $x ^ { \prime } , y ^ { \prime } , \ldots$ , in [0, 1], even though $x ^ { \prime }$ , etc., are not in the range of the rule.

The onto condition is even weaker than what is called unanimity, which requires that whenever all agents’ preferences have the same peak $( p _ { i } = p _ { j }$ for all $i , j )$ , the rule must choose that location as the outcome. In turn, unanimity is weaker than Pareto optimality: for all $\succeq \in \mathcal { R } ^ { n }$ , there exists no point $x \in [ 0 , 1 ]$ such that $x \succeq _ { i } f ( \succeq )$ for all $i \in N$

As it turns out, these three requirements are all equivalent among strategy-proof rules.

Lemma 10.1 Suppose f is strategy-proof. Then f is onto if and only if it is unanimous ifand only ifit is Pareto-optimal.

proof It is clear that Pareto-optimality implies the other two conditions. Sup pose f is strategy-proof and onto. Fix $x \in [ 0 , 1 ]$ and let $\succeq \in \mathcal { R } ^ { n }$ be such that $f ( \succeq ) = x$ . Consider any “unanimous” profile $\succeq ^ { \prime } \in \mathcal { R } ^ { n }$ such that $p _ { i } ^ { \prime } = x$ for each $i \in N$ . By strategy-proofness, $f ( \Sigma _ { 1 } ^ { \prime } , \Sigma 2 , \dots , \Sigma _ { n } ) = x$ , otherwise agent 1 could manipulate f. Repeating this argument, $f ( \succeq _ { 1 } ^ { \prime } , \succeq _ { 2 } ^ { \prime } , \succeq _ { 3 } , \dots , \succeq _ { n } ) = x , \dots ,$ $f ( \succeq ^ { \prime } ) = x$ . That is, f is unanimous.

Finally, to derive a contradiction, suppose that f is not Pareto-optimal at some profile $\succeq \in \mathcal { R } ^ { n }$ . This implies that either (i) $f ( \succeq ) < p _ { i }$ for all $i \in N \mathrm { o r } ( \mathrm { i i } ) f ( \succeq ) >$ $p _ { i }$ for all $i \in N$ . Without loss of generality, assume (i) holds. Furthermore, assume that the agents are labeled so that $p _ { 1 } \leq p _ { 2 } \leq \cdots \leq p _ { n }$

If $p _ { 1 } = p _ { n }$ then unanimity is violated, completing the proof. Otherwise, let $j \in N$ be such that $p _ { 1 } = p _ { j } < p _ { j + 1 } ;$ ; that is, $j < n$ agents have the minimum peak. For all $i > j$ , let $\succeq _ { i } ^ { \prime }$ be a preference relation such that both $p _ { i } ^ { \prime } = p _ { 1 }$ and $f ( \succeq ) \succeq _ { i } ^ { \prime } p _ { i }$

Let $x _ { n } = f ( \succeq _ { 1 } , \dots , \succeq _ { n - 1 } , \succeq _ { n } ^ { \prime } )$ . By strategy-proofness, $x _ { n } \in [ f ( \succeq ) , p _ { n } ]$ , otherwise agent n (with preference $\succeq _ { n } ^ { \prime } )$ could manipulate f by reporting preference $\succeq _ { n } .$ Similarly, $x _ { n } \notin ( f ( \succeq ) , p _ { n } ]$ , otherwise agent n (with preference $\succeq _ { n } )$ could manip ulate $f$ by reporting preference $\succeq _ { n } ^ { \prime }$ . Therefore $x _ { n } = f ( \succeq )$

Repeating this argument as each $i > j$ replaces $\succeq _ { i }$ with $\succeq _ { i } ^ { \prime }$ , we have

$$
f (\succeq_ {1}, \dots , \succeq_ {j}, \succeq_ {j + 1} ^ {\prime}, \dots , \succeq_ {n} ^ {\prime}) = f (\succeq)
$$

which contradicts unanimity. Since a strategy-proof, onto rule must be unanimous, this is a contradiction.

## 10.2.1 Rules

The central strategy-proof rule on this domain is the simple median-voter rule. Suppose that the number of agents n is odd. Then the rule that picks the median of the agents peaks $( p _ { i } \mathrm { { \ ' s ) } }$ is a strategy-proof rule.

It is straightforward to see why this rule is strategy-proof: If an agent’s peak $p _ { i }$ lies below the median peak, then he can change the median only by reporting a preference relation whose peak lies above the true median. The effect of this misreport is for the rule to choose a point even further away from $p _ { i }$ , making the agent worse off. A symmetric argument handles the case in which the peak is above the median. Finally, an agent cannot profitably misreport his preferences if his peak is the median one to begin with.

More generally, for any number of agents n and any positive integer $k \leq n$ , the rule that picks the kth highest peak is strategy-proof for precisely the same reasons as above. An agent can only move the kth peak further from his own. The median happens to be the case where $k = ( n + 1 ) / 2$

The strategy-proofness of such rules stands in contrast to the incentives properties of rules that choose average-type statistics. Consider the rule that chooses the average of the n agents’ peaks. Any agent with peak $p _ { i } \in ( 0 , 1 )$ ) that is not equal to the average can manipulate the rule by reporting preferences with a more extreme peak (closer to 0 or 1) than his true peak.

This would also hold for any weighted average of the agents’ peaks, with one exception. If a rule allocated all of the weight to one agent, then the resulting rule simply picks that agent’s peak always. Such a dictatorial rule is strategy-proof and onto.

In addition to favorable incentives properties, rules based on order statistics have the property that they require little information to be computed. Technically a rule requires agents to report an entire preference ordering over [0, 1]. The rules we have discussed so far, however, only require agents to report their most preferred point, i.e., a single number. In fact, under the onto assumption, this informational property is a consequence of the strategy-proofness requirement; that is, all strategy-proof and onto rules have the property that they can be computed solely from information about the agents’ peaks.

To begin showing this, we first observe that the class of “kth-statistic rules” can be further generalized as follows. Consider a fixed set of points $y _ { 1 } , y _ { 2 } , \dotsc , y _ { n - 1 } \in A$ Consider the rule that, for any profile of preferences $\succeq$ , chooses the median of the 2n − 1 points consisting of the n agents’ peaks and the $n - 1$ points of $y$ . This kind of rule differs from the previous ones in that, for some choices of $y$ and some profiles of preferences, the rule may choose a point that is not the peak of any agent’s preferences. Yet, for the same reasons as above, such a rule is strategy-proof.

It turns out that such rules compose the entire class of strategy-proof and onto rules that treat agents symmetrically. To formalize this latter requirement, we call a rule anonymous if for any $\succeq \in \mathcal { R } ^ { n }$ and any permutation $\succeq ^ { \prime } \mathrm { o f } \succeq , f ( \succeq ^ { \prime } ) = f ( \succeq )$ ). This requirement captures the idea that the agents’ names play no role in the behavior of a rule. Dictatorial rules mentioned above are examples of rules that are strategy-proof and onto, but not anonymous.

Theorem 10.2 A rule f is strategy-proof, onto, and anonymous if and only if there exist $y _ { 1 } , y _ { 2 } , \dotsc , y _ { n - 1 } \in [ 0 , 1 ]$ such thatfor all $\succeq \in \mathcal { R } ^ { n }$

$$
f (\succeq) = \operatorname{med} \left\{p _ {1}, p _ {2}, \dots , p _ {n}, y _ {1}, y _ {2}, \dots , y _ {n - 1} \right\}.\tag{10.1}
$$

proof We leave it as an exercise to verify that such a rule satisfies the three axioms in the Theorem. To prove the converse, suppose f is strategy-proof, onto, and anonymous.

We make extensive use of the two (extreme) preference relations that have peaks at 0 and 1 respectively. Since preferences relations are ordinal, there is only one preference relation with a peak at 0 and only one with a peak at 1. Denote these two preference relations by $\succeq _ { i } ^ { 0 }$ and $\succeq _ { i } ^ { 1 }$ respectively.

(Construct the $y _ { m } ^ { \phantom { \dagger } } ( s . )$ For any $1 \leq m \leq n - 1$ , let $y _ { m }$ denote the outcome of f when m agents have preference relation $\succeq _ { i } ^ { 1 }$ and the remainder have $\succeq _ { i } ^ { 0 }$ :

$$
y _ {m} = f \big (\succeq_ {1} ^ {0}, \dots , \succeq_ {n - m} ^ {0}, \succeq_ {n - m + 1} ^ {1}, \dots , \succeq_ {n} ^ {1} \big).
$$

Recall that by anonymity the order of the arguments of $f$ is irrelevant; if precisely m agents have preference relation $\succeq _ { i } ^ { 1 }$ and the rest have $\succeq _ { i } ^ { 0 }$ then the outcome is $y _ { m }$ . In addition, we leave it to the reader to verify that stragegy proofness implies monotonicity of the $y _ { m } \mathrm { ' s } \colon y _ { m } \leq y _ { m + 1 }$ for each $1 \leq m \leq n - 2$ . We prove the theorem by showing that $f$ satisfies Eq. (10.1) with respect to this list of $y _ { m } \mathbf { \bar { s } } .$

Consider a profile of preference $\succeq \in \mathcal { R } ^ { n }$ with peaks $p _ { 1 } , . . . , p _ { n }$ . Without loss of generality (by anonymity) assume that $p _ { i } \leq p _ { i + 1 }$ for each $i \le n - 1$ . We wish to show $f ( \succeq ) = x ^ { * } \equiv \mathrm { m e d } \{ p _ { 1 } , \dots , p _ { n } , y _ { 1 } , \dots , y _ { n - 1 } \}$

(Case 1: the median is some $y _ { m } . )$ Suppose $x ^ { * } = y _ { m }$ for some m. By monotonicity of the peaks and $y _ { m } \mathbf { \bar { s } } .$ , since $x ^ { * }$ is the median of $2 n - 1$ points this implies $p _ { n - m } \leq x ^ { * } = y _ { m } \leq p _ { n - m + 1 }$ . By assumption,

$$
x ^ {*} = y _ {m} = f \big (\succeq_ {1} ^ {0}, \dots , \succeq_ {n - m} ^ {0}, \succeq_ {n - m + 1} ^ {1}, \dots , \succeq_ {n} ^ {1} \big).\tag{10.2}
$$

Let $x _ { 1 } = f ( \succeq _ { 1 } , \succeq _ { 2 } ^ { 0 } , \ldots , \succeq _ { n - m } ^ { 0 } , \succeq _ { n - m + 1 } ^ { 1 } , \ldots , \succeq _ { n } ^ { 1 } )$ . Strategy-proofness implies $x _ { 1 }$ $\geq x ^ { * }$ , otherwise agent 1 with preference ${ \succeq } _ { 1 } ^ { 0 }$ could manipulate $f .$ . Similarly, since $p _ { 1 } \leq y _ { m }$ , we cannot have $x _ { 1 } > x ^ { * }$ , otherwise agent 1 with preference $\succeq 1$ could manipulate $f$ . Hence $x _ { 1 } = x ^ { * }$ . Repeating this argument for all $i \leq n - m , x ^ { * } =$ $f ( \succeq _ { 1 } , \dots , \succeq _ { n - m } , \succeq _ { n - m + 1 } ^ { 1 } , \dots , \succeq _ { n } ^ { 1 } )$ . The symmetric argument for all $i > n - m$ implies

$$
f (\succeq_ {1}, \dots , \succeq_ {n}) = x ^ {*}.\tag{10.3}
$$

(Case 2: the median is an agent’s peak.) The remaining case is that $y _ { m } < x ^ { * } <$ $y _ { m + 1 }$ for some m. (The cases where $x ^ { * } < y _ { 1 }$ and $x ^ { * } > y _ { n - 1 }$ are similar, denoting $y _ { 0 } = 0$ and $y _ { n } = 1 . )$ In this case, since the agents’ peaks are in increasing order, we have $x ^ { * } = p _ { n - m }$

If

$$
f \big (\succeq_ {1} ^ {0}, \ldots , \succeq_ {n - m - 1} ^ {0}, \succeq_ {n - m}, \succeq_ {n - m + 1} ^ {1}, \ldots , \succeq_ {n} ^ {1} \big) = x ^ {*} = p _ {n - m}\tag{10.4}
$$

then, analogous to the way Eq. (10.2) implied Eq. (10.3), repeated applications of strategy-proofness (to the $n - 1$ agents other than $i = n - m )$ ) would imply $f ( \succeq _ { 1 } , \dots , \succeq _ { n } ) = x ^ { * }$ , and the proof would be finished. The remainder of the proof is devoted to showing that indeed Eq. (10.4) must hold.

Suppose to the contrary that

$$
f \big (\succeq_ {1} ^ {0}, \ldots , \succeq_ {n - m - 1} ^ {0}, \succeq_ {n - m}, \succeq_ {n - m + 1} ^ {1}, \ldots , \succeq_ {n} ^ {1} \big) = x ^ {\prime} <   x ^ {*}.\tag{10.5}
$$

(The case $x ^ { \prime } > x ^ { * }$ can be proven symmetrically.) If agent $( n - m )$ were to report preference $\succeq _ { n - m } ^ { 0 }$ instead, f would choose outcome $y _ { m } ;$ ; hence strategy-proofness implies $y _ { m } \leq x ^ { \prime } < x ^ { * }$ . See Figure 10.1.

Denote the outcomes that agent $( n - m )$ can obtain by varying his preferences, fixing the others, $\mathrm { { a s } } ^ { 5 }$

$$
O = \left\{x: \exists \tilde {\succeq} _ {n - m} \text {   s.t.   } x = f \big (\succeq_ {1} ^ {0}, \dots , \succeq_ {n - m - 1} ^ {0}, \tilde {\succeq} _ {n - m}, \succeq_ {n - m + 1} ^ {1}, \dots , \succeq_ {n} ^ {1} \big) \right\}.
$$

By definition, $x ^ { \prime } \in O$ ; Case 1 implies $y _ { m } , y _ { m + 1 } \in O$ . Strategy proofness implies that $x ^ { \prime } = \operatorname* { m a x } \{ x \in O : x \leq x ^ { * } \}$ , otherwise by reporting some other preference, agent $( n - m )$ could obtain some $x \in ( x ^ { \prime } , x ^ { * } )$ , violating strategy proofness.

![](images/8cf81d2ae71040ce4cb656b4f9c6db3545315b2c70b2a6cba6224651d92b07e6.jpg)  
Figure 10.1. Proof of Theorem 10.2. If a strategy-proof, onto rule does not pick $x ^ { * }$ when it is the median of peaks and $\gamma _ { m } \mathrm { ' s }$ , then a contradiction is reached using preferences with peaks at $p _ { i } ^ { L }$ and $p _ { i } ^ { H }$

Letting $x ^ { \prime \prime } \equiv \operatorname* { i n f } \{ x \in O : x \geq x ^ { * } \}$ , strategy proofness implies $x ^ { \prime \prime } \in O . ^ { 6 }$ To see this, let $\succeq _ { n - m } ^ { \prime \prime }$ be a preference relation with peak $p _ { n - m } ^ { \prime \prime } = x ^ { \prime \prime }$ and such that $( x ^ { \prime \prime } + \epsilon ) \succ _ { n - m } ^ { \prime \prime } x ^ { \prime }$ for some small $\epsilon > 0$ . Then strategy proofness implies $\begin{array} { r } { f ( \succeq _ { 1 } ^ { 0 } , \dots , \succeq _ { n - m - 1 } ^ { 0 } , \succeq _ { n - m } ^ { \prime \prime } , \succeq _ { n - m + 1 } ^ { 1 } , \dots , \succeq _ { n } ^ { 1 } ) \} = \hat { x } \in [ x ^ { \prime \prime } , x ^ { \prime \prime } + \epsilon ] } \end{array}$ . But $\mathrm { i f } \hat { x } \neq x ^ { \prime \prime }$ then there would exist a misreport resulting in an outcome arbitrarily closer to $x ^ { \prime \prime }$ , making agent $( n - m )$ (with preference $\succeq _ { n - m } ^ { \prime \prime } )$ better off. Hence $\hat { x } = x ^ { \prime \prime } =$ min $\{ x \in O : x \geq x ^ { * } \}$ }. With Eq. (10.5), we have $x ^ { \prime \prime } > x ^ { * }$

We have shown that $O \cap ( x ^ { \prime } , x ^ { \prime \prime } ) = \emptyset$ . Let $p _ { i } ^ { L }$ be a symmetric preference relation with peak at $p ^ { L } = ( x ^ { \prime } + x ^ { \prime \prime } ) / 2 - \varepsilon$ , where $\varepsilon > 0$ is sufficiently small; see Figure 10.1. Similarly let $p _ { i } ^ { H }$ be a symmetric preference relation with peak at $( x ^ { \prime } + x ^ { \prime \prime } ) / 2 + \varepsilon$ . Then strategy-proofness implies

$$
f \big (\succeq_ {1} ^ {0}, \dots , \succeq_ {n - m - 1} ^ {0}, \succeq_ {n - m} ^ {H}, \succeq_ {n - m + 1} ^ {1}, \dots , \succeq_ {n} ^ {1} \big) \big \} = x ^ {\prime \prime}.
$$

By repeated application of strategy-proofness (along the lines used in proving Eq. (10.3)), this implies

$$
\left. f \big (\succeq_ {1} ^ {L}, \dots , \succeq_ {n - m - 1} ^ {L}, \succeq_ {n - m} ^ {H}, \succeq_ {n - m + 1} ^ {1}, \dots , \succeq_ {n} ^ {1} \big) \right\} = x ^ {\prime \prime}.
$$

Lemma 10.1 (Pareto-optimality) implies

$$
\left. f \left(\succeq_ {1} ^ {L}, \dots , \succeq_ {n - m - 1} ^ {L}, \succeq_ {n - m} ^ {L}, \succeq_ {n - m + 1} ^ {1}, \dots , \succeq_ {n} ^ {1}\right) \right\} \geq p _ {i} ^ {L}.
$$

Therefore, strategy-proofness implies

$$
f \big (\succeq_ {1} ^ {L}, \dots , \succeq_ {n - m - 1} ^ {L}, \succeq_ {n - m} ^ {L}, \succeq_ {n - m + 1} ^ {1}, \dots , \succeq_ {n} ^ {1} \big) \big \} = x ^ {\prime \prime}\tag{10.6}
$$

otherwise agent $n - m$ could manipulate at one of the two profiles (since $\varepsilon$ is small).

On the other hand, strategy-proofness implies

$$
f \big (\succeq_ {1} ^ {0}, \ldots , \succeq_ {n - m - 1} ^ {0}, \succeq_ {n - m} ^ {L}, \succeq_ {n - m + 1} ^ {1}, \ldots , \succeq_ {n} ^ {1} \big) = x ^ {\prime}
$$

by the definition of $\succeq _ { i } ^ { L }$ . Strategy-proofness implies that if agent $( n - m - 1 )$ instead reports preference $\succeq ^ { L }$ , a point must be chosen that is in the interval $[ x ^ { \prime } , x ^ { \prime \prime } - 2 \varepsilon ]$ , otherwise, he could report $\succeq ^ { 0 }$ to gain. By repeated application of this argument, this continues to hold as each agent $1 \leq i \leq n - m - 1$ changes his report from $\succeq _ { i } ^ { 0 }$ to $\succeq _ { i } ^ { L }$ , so

$$
f \big (\succeq_ {1} ^ {L}, \dots , \succeq_ {n - m - 1} ^ {L}, \succeq_ {n - m} ^ {L}, \succeq_ {n - m + 1} ^ {1}, \dots , \succeq_ {n} ^ {1} \big) \in [ x ^ {\prime}, x ^ {\prime \prime} - 2 \varepsilon ].
$$

This contradicts Eq. (10.6). Hence Eq. (10.5) cannot hold, so $x ^ { \prime } \geq x ^ { * }$ ; the symmetric argument implies $x ^ { \prime } = x ^ { * }$ , resulting in Eq. (10.4). Thus f chooses the median of these $2 n - 1$ points for profile .

The parameters $( y _ { m } \mathbf { \hat { s } } )$ in Theorem 10.2 can be thought of as the rule’s degree of compromise when agents have extremist preferences. If m agents prefer the highest possible outcome (1), while $n - m$ prefer the lowest (0), then which point should be chosen? A true median rule would pick whichever extreme (0 or 1) contains the most peaks. On the other hand, the other rules described in the Theorem may choose intermediate points $( y _ { m } )$ as a compromise. The degree of compromise (which $y _ { m } )$ can depend on the degree to which the agents’ opinions are divided (the size of m).

The anonymity requirement is a natural one in situations where agents are to be treated as equals. If one does not require this, however, the class of strategy-proof rules becomes even larger. We have already mentioned dictatorial rules, which always chooses a predetermined agent’s peak. There are less extreme violations of anonymity: The full class of strategy-proof, onto rules, which we now define, allows agents to be treated with varying degrees of asymmetry.

Definition 10.3 A rule f is a generalized median voter scheme (g.m.v.s.) if there exist 2<sup>n</sup> points in $[ 0 , 1 ] , \{ \alpha _ { S } \} _ { S \subseteq N }$ , such that

(i) $S \subseteq T \subseteq N$ implies $\alpha _ { S } \leq \alpha _ { T }$

(ii) $\alpha _ { \emptyset } = 0 , \alpha _ { N } = 1$ , and

(iii) for $\mathrm { a l l } \succeq \in { \mathcal { R } } ^ { n } , f ( \succeq ) = \operatorname* { m a x } _ { S \subset N } \operatorname* { m i n } \{ \alpha _ { S } , p _ { i } : i \in S \} .$

An example is given below. It is worth making two observations regarding Defi nition 10.3. First, the monotonicity condition (i) is actually redundant. If parameters $\{ \alpha _ { S } \} _ { S \subseteq N }$ fail this condition, they still define some strategy-proof rule via condition (iii). However, the resulting rule could also be defined by an alternate set of parameters $\{ \alpha _ { S } ^ { \prime } \} _ { S \subseteq N }$ that do satisfy condition (i). Second, condition (ii) is present merely to guarantee the rule to be onto. Parameters that fail this condition still define a strategy-proof rule whose range is $[ \alpha _ { \emptyset } , \alpha _ { N } ] . ^ { 7 }$

Consider the rule described by the parameters $( \alpha _ { S } { ' } \mathrm { s } )$ in Figure 10.2, for the 3-agent case. The reader should first verify the following. If each agent in some set $S \subseteq N$ were to have a preference peak at 1, while each remaining agent (in $N \backslash S )$ were to have a preference peak at 0, then the rule would choose $\alpha _ { S }$ as the outcome. In this sense, the $\alpha _ { S }$ parameters reflect a (nonanonymous) degree of compromise at extreme preference profiles, analogous to the $y _ { m }$ parameters of Theorem 10.2.

Without the anonymity condition, some agents – more generally some coalitions of agents – are more powerful than others. To see this, consider the profile of preferences represented in Figure 10.2 with peaks $p _ { 1 } , p _ { 2 } , p _ { 3 }$ . Following condition (iii) of Defi nition 10.3, calculate min $\{ \alpha _ { S } , p _ { i } : i \in S \}$ for each $S \subseteq N$ . Beginning with the three singleton coalitions of the form $S = \{ i \}$ , these values are $\alpha _ { 1 } , \alpha _ { 2 }$ , and $\alpha _ { 3 }$ , because each $p _ { i }$ is above that agent’s corresponding $\alpha _ { \{ i \} }$ . (For peak $p _ { 3 } ^ { \prime }$ , the third value would have been $p _ { 3 } ^ { \prime }$ instead.) Since the g.m.v.s. eventually chooses the maximum of these kinds of values (after we also check larger coalitions), agent 3 can be said to have more power than the other two agents, as a singleton. A large $\alpha _ { 3 }$ corresponds to more instances in which agent $_ { 3 \mathrm { { : } s } }$ peak is a candidate outcome for this rule. A small $\alpha _ { 1 }$ corresponds to more instances in which agent 1 has no impact on the outcome (i.e., whenever $p _ { 1 } > \alpha _ { \{ 1 \} } )$

![](images/c29e905e1825200498e3470ce461e856634afe3ac6fc42d2a84ea41f9997c22c.jpg)  
Figure 10.2. An example of a generalized median voter scheme for $n = 3$

On the other hand, we also need to calculate these minimum-values for larger coalitions. For the pairs of agents $\{ 1 , 2 \} , \{ 1 , 3 \}$ , and {2, 3}, these values are $\alpha _ { \{ 1 , 2 \} } , p _ { 1 }$ and $p _ { 2 }$ respectively. Coalition {1, 2} is the weakest two-agent coalition in the sense that they have the lowest $\alpha _ { S }$ . After checking $S = \emptyset$ (which yields 0) and $S = N$ (yielding a repetition of the value $p _ { 2 } )$ , we calculate the rule’s outcome to be the maximum of the $2 ^ { n }$ values $\{ 0 , \alpha _ { 1 } , \alpha _ { 2 } , \alpha _ { 3 } , \alpha _ { \{ 1 , 2 \} } , p _ { 1 } , p _ { 2 } , p _ { 2 } \}$ we have obtained, which is $\alpha _ { \{ 3 \} }$

We close by stating the main result of this section. We omit its proof, which has much in common with the proof of Theorem 10.2.

Theorem 10.4 A rule f is strategy-proof and onto ifand only ifit is a general ized median voter scheme.

## 10.2.2 Application to Public Good Cost Sharing

Consider a group of n agents who have access to a machine that can convert their labor into some public good. Specifically, suppose that the machine requires the simultaneous labor of all n agents in order to work. The agents are free to jointly decide how many hours of labor, , to work. Implicit is the requirement that each agent work for  hours, however, since the machine requires all n agents’ labor simultaneously. After  hours of labor, the machine outputs $y = Y ( \ell )$ ) units of some public good, where the production function Y is assumed to be an increasing and strictly concave function, with $Y ( 0 ) = 0$

Different agents may have different preferences over how much labor they should provide, in exchange for the public good. Let us suppose that we know nothing about their preferences, other than the fact that they are represented by some utility function $u _ { i } ( { \boldsymbol { \ell } } , { \boldsymbol { y } } )$ which is strictly increasing in y, strictly decreasing in , and is quasi-concave.<sup>8</sup> See Figure 10.3.

In this environment, a rule takes as input the reported utility functions of the agents, subject only to the assumptions we have made. It then gives as output a single labor requirement $\ell = f ( u _ { 1 } , \ldots , u _ { n } )$ . Each agent is then required to provide  units of labor, and they enjoy $Y ( \ell )$ units of output as a reward. What rules are strategy-proof and onto?

![](images/ede84ea6bce5365171efcb7304701539ef9704ce935b3d2679247ac28deedfb3.jpg)  
Figure 10.3. An agent with utility function u most prefers the outcome $( \gamma , \ell ) ;$ one with $u ^ { \prime }$ prefers $( \boldsymbol { y } ^ { \prime } , \boldsymbol { \ell } ^ { \prime } )$

By assumption, outcomes may only be attained along the graph of Y. Because of the assumptions on Y and on preferences, it is clear that agents have single-peaked preferences over this consumption space. It follows that any strategy-proof, onto rule for this environment is a generalized median voter schemes operating along the graph of Y.

Proving this is not difficult, but involves some technical details that we omit. First the outcome space is not bounded as we assumed before, although it would certainly be reasonable to bound it by assumption. Second, the preference domain here should be verified to yield all the single-peaked preferences necessary to characterize generalized median voter schemes; e.g., we used symmetric single-peaked preferences to construct the proof of Theorem 10.2. Third, one should demonstrate that a strategy-proof rule in this environment is invariant to utility information away from the graph of Y. We leave it to the interested reader to verify our claim despite these technicalities.

In this kind of problem, it may be reasonable to add additional requirements to a rule. One that we address is the requirement that an agent should be better off as part of this decision-making group than if he were simply to walk away. Formally, if this public good technology did not exist, each agent would provide no labor $( \ell = 0 )$ and would enjoy none of the public good $( y = 0 )$ . We say a rule is individually rational if for all $U = ( u _ { 1 } , \ldots , u _ { n } )$ and $1 \geq i \geq n$ , we have $u _ { i } ( f ( U ) , Y ( f ( U ) ) ) \geq$ $u _ { i } ( 0 , 0 )$

What strategy-proof and onto rules satisfy individual rationality? In terms of our earlier model, where agents have single-peaked preferences on $[ 0 , 1 ]$ , that question translates as follows: What g.m.v.s. has the property that, for any preference profile, each agent (weakly) prefers the chosen outcome to the outcome $x = 0 ?$

The answer is that there is a unique such rule. As an exercise, we leave it to the reader to show that the rule that chooses the minimum peak is the unique strategy-proof, onto rule that satisfies this individual rationality condition. In terms of this public good model, this corresponds to asking each agent their most preferred labor level $\ell ,$ and choosing the minimum.

## 10.3 House Allocation Problem

The House allocation problem is a model for understanding the allocation of indivisible goods. It involves a set N ofn agents, each owning a unique house and a strict preference ordering over all n houses. The objective is to reallocate the houses among the agents in an appropriate way. A modern version of the same would replace houses by kidneys.

While any possible (strict) preference ordering over the homes is permitted, the set of preferences over allocations is restricted. In particular, an agent is indifferent between all allocations that give her the same house. Therefore the Gibbard–Satterthwaite Theorem does not apply in this setting.

One could select an allocation ofhomes in a variety ofways, perhaps so as to optimize some function of the preferences and then investigate if the resulting allocation rule is strategy-proof. However, this ignores an important feature not present in earlier examples. In this environment, agents control the resources to be allocated. Therefore an allocation can be subverted by a subset of agents who might choose to break away and trade among themselves. For this reason it is natural to focus on allocations that are invulnerable to agents opting out.

Number each house by the number of the agent who owns that house. An allocation is an n vector a whose ith component, $a _ { i }$ , is the number of the house assigned to agent i. If a is the initial allocation then $a _ { i } = i$ . For an allocation to be feasible, we require that $a _ { i } \neq a _ { j }$ for all $i \neq j$ . The preference ordering of an agent i will be denoted $\succ _ { i }$ and $x \succ i \textit { y }$ will mean that agent i ranks house x above house y. Denote by A the set of all feasible allocations. For every $S \subseteq N$ let $A ( S ) = \{ z \in A : z _ { i } \in S \forall i \in S \}$ denote the set of allocations that can be achieved by the agents in S trading among themselves alone. Given an allocation $a \in A$ , a set S of agents is called a blocking coalition (for a) if there exists ${ \textbf { a } } z \in A ( S )$ such that for all $i \in S$ either $z _ { i } \succ _ { i } a _ { i }$ or $z _ { i } = a _ { i }$ and for at least one $j \in S$ we have that $z _ { j } \succ _ { j } a _ { j }$ . A blocking coalition can, by trading among themselves, receive homes that each strictly prefers (or is equivalent) to the home she receives under a, with at least one agent being strictly better off. The set of allocations that is not blocked by any subset of agents is called the core.

The reader will be introduced to the notion of the core in Chapter 15 (Section 15.2) where it will be defined for a cooperative game in which utility is transferable via money (a TU game). The house allocation problem we consider is an example of a cooperative game with nontransferable utility (an NTU game). The definition of the core offered here is the natural modification of the notion of TU core to the present setting.

The theorem below shows the core to be nonempty. The proof is by construction using the top trading cycle algorithm (TTCA).

Definition 10.5 (Top Trading Cycle Algorithm) Construct a directed graph using one vertex for each agent. If house j is agent i’s kth ranked choice, in sert a directed edge from i to j and color the edge with color k. An edge of the form $( i , i )$ will be called a loop. First, identify all directed cycles and loops consisting only of edges colored 1. The strict preference ordering implies that the set of such cycles and loops is node disjoint. Let $N _ { 1 }$ be the set of vertices (agents) incident to these cycles. Each cycle implies a sequence of swaps. For example, suppose $i _ { 1 }  i _ { 2 }  i _ { 3 }  \cdot \cdot \cdot  i _ { r }$ is one such cycle. Give house $i _ { 1 }$ to agent $i _ { r } .$ house $i _ { r }$ to agent $i _ { r - 1 }$ , and so on. After all such swaps are performed, delete all edges colored 1. Repeat with the edges colored 2 and call the corresponding set of vertices incident to these edges $N _ { 2 }$ , and so on. The TTCA yields the resulting matching.

This algorithm is used to prove the following result.

Theorem 10.6 The core ofthe house allocation problem consists ofexactly one matching.

proof We prove that if a matching is in the core, it must be the one returned by the TTCA.

Under the TTCA, each agent in $N _ { 1 }$ receives his favorite house, i.e., the house ranked first in his preference ordering. Therefore, $N _ { 1 }$ would form a blocking coalition to any allocation that does not assign to all of those agents the houses they would receive under the TTCA. That is, any core allocation must assign $N _ { 1 }$ to houses just as the TTCA assigns them.

Given this fact, the same argument applies to $N _ { 2 }$ : Under the TTCA, each agent in $N _ { 2 }$ receives his favorite house not including those houses originally endowed by agents in $N _ { 1 }$ . Therefore, if an allocation is in the core and the agents in $N _ { 1 }$ are assigned each other’s houses, then agents in $N _ { 2 }$ must receive the same houses they receive under the TTCA.

Continuing the argument for each $N _ { k }$ proves that if an allocation is in the core, then it is the one determined by the TTCA. This proves that there is at most one core allocation.

To prove that the TTCA allocation is in the core, it remains to be shown that there is no other blocking coalition $S \subseteq N$ . This is left to the reader.

To apply the TTCA, one must know the preferences of agents over homes. Do they have an incentive to truthfully report these? To give a strongly positive answer to this question, we first associate the TTCA with its corresponding direct revelation mechanism. Define the Top Trading Cycle (TTC) Mechanism to be the function (mechanism) that, for each profile of preferences, returns the allocation computed by the TTCA.

Theorem 10.7 The TTC mechanism is strategy-proof.

proof Let π be a profile of preference orderings and a the allocation returned by TTCA when applied to π. Suppose that agent $j \in N _ { k }$ for some k misreports her preference ordering. Denote by $\pi ^ { \prime }$ the new profile of preference orderings. Let $a ^ { \prime }$ the allocation returned by TTCA when applied to $\pi ^ { \prime }$ . If the TTCA is not strategy-proof $a _ { i } ^ { \prime } > ^ { i } a _ { i }$ . Observe that $a _ { i } = a _ { i } ^ { \prime }$ for all $i \in \bigcup _ { r = 1 } ^ { k - 1 } N _ { r }$ . Therefore, $\begin{array} { r } { a _ { i } ^ { \prime } \in N \setminus \{ \bigcup _ { r = 1 } ^ { k - 1 } N _ { r } \} } \end{array}$ . However, the TTCA chooses $a _ { i }$ to be agent i’s top ranked choice from $N \setminus \{ \bigcup _ { r = 1 } ^ { k - 1 } N _ { r } \}$ contradicting the fact that $a _ { i } ^ { \prime } > ^ { i } a _ { i }$ □

If we relax the requirement that preferences be strict, what we had previously called a blocking set is now called a weakly blocking set. What we had previously called the core is now called the strict core. With indifference, a blocking set S is one where all agents in S are strictly better off by trading among themselves. Note the requirement that all agents be strictly better off. The core is the set of allocations not blocked by any set S.

When preferences are strict, every minimal weakly blocking set is a blocking set. To see this, fix a weakly blocking set S. An agent in S who is not made strictly better off by trade among agents in S must have been assigned their own home. Remove them from S. Repeat. The remaining agents must all be allocated houses that make them strictly better off. Hence, when preferences are strict the core and strict core coincide. With indifference permitted, the strict core can be different from the core. In fact, there are examples where the strict core is empty and others where it is not unique. Deciding emptiness of the strict core is polynomial in $| N |$

Another possible extension of the model is to endow the agents with more than one good. For example, a home and a car. Clearly, if preferences over pairs of goods are sufficiently rich, the core can be empty. It turns out that even under very severe restrictions the core can still be empty. For example, when preferences are separable, i.e., one’s ranking over homes does not depend on which car one has.

## 10.4 Stable Matchings

The stable matching problem was introduced as a model of how to assign students to colleges. Since its introduction, it has been the object of intensive study by both computer scientists and economists. In computer science it used as vehicle for illustrating basic ideas in the analysis of algorithms. In economics it is used as a stylized model of labor markets. It has a direct real-world counterpart in the procedure for matching medical students to residencies in the United States.

The simplest version of the problem involves a set M of men and a set W of women. Each $m \in M$ has a strict preference ordering over the elements of W and each $w \in W$ has a strict preference ordering over the men. As before the preference ordering of agent i will be denoted $\succ _ { i }$ and $x \succ i \ y$ will mean that agent i ranks x above y. A matching is an assignment of men to women such that each man is assigned to at most one woman and vice versa. We can accommodate the possibility of an agent choosing to remain single as well. This is done by including for each man (woman) a dummy woman (man) in the set W (M) that corresponds to being single (or matched with oneself). With this construction we can always assume that $| M | = | W |$

As in the house allocation problem a group of agents can subvert a prescribed matching by opting out. In a manner analogous to the house allocation problem, we can define a blocking set. A matching is called unstable if there are two men $m , m ^ { \prime }$ and two women $w , w ^ { \prime }$ such that

(i) m is matched to w,

(ii) $m ^ { \prime }$ is matched to $w ^ { \prime }$ , and

(iii) $w ^ { \prime } \succ _ { m } w$ and $m \succ _ { w ^ { \prime } } m ^ { \prime }$

The pair $( m , w ^ { \prime } )$ is called a blocking pair. A matching that has no blocking pairs is called stable.

Example 10.8 The preference orderings for the men and women are shown in the table below

<table><tr><td> $\succ_{m_1}$ </td><td> $\succ_{m_2}$ </td><td> $\succ_{m_3}$ </td><td> $\succ_{w_1}$ </td><td> $\succ_{w_2}$ </td><td> $\succ_{w_3}$ </td></tr><tr><td> $w_2$ </td><td> $w_1$ </td><td> $w_1$ </td><td> $m_1$ </td><td> $m_3$ </td><td> $m_1$ </td></tr><tr><td> $w_1$ </td><td> $w_3$ </td><td> $w_2$ </td><td> $m_3$ </td><td> $m_1$ </td><td> $m_3$ </td></tr><tr><td> $w_3$ </td><td> $w_2$ </td><td> $w_3$ </td><td> $m_2$ </td><td> $m_2$ </td><td> $m_2$ </td></tr></table>

Consider the matching $\{ ( m _ { 1 } , w _ { 1 } ) , ( m _ { 2 } , w _ { 2 } ) , ( m _ { 3 } , w _ { 3 } ) \}$ . This is an unstable match ing since $( m _ { 1 } , w _ { 2 } )$ is a blocking pair. The matching $\{ ( m _ { 1 } , w _ { 1 } ) , ( m _ { 3 } , w _ { 2 } ) , ( m _ { 2 } , w _ { 3 } ) \}$ }, however, is stable.

Given the preferences of the men and women, is it always possible to find a stable matching? Remarkably, yes, using what is now called the deferred acceptance algorithm. We describe the male-proposal version of the algorithm.

Definition 10.9 (Deferred Acceptance Algorithm, male-proposals) First, each man proposes to his top-ranked choice. Next, each woman who has received at least two proposals keeps (tentatively) her top-ranked proposal and rejects the rest. Then, each man who has been rejected proposes to his top-ranked choice among the women who have not rejected him. Again each woman who has at least two proposals (including ones from previous rounds) keeps her top-ranked proposal and rejects the rest. The process repeats until no man has a woman to propose to or each woman has at most one proposal. At this point the algorithm terminates and each man is assigned to a woman who has not rejected his proposal. Notice that no man is assigned to more than one woman. Since each woman is allowed to keep only one proposal at any stage, no woman is assigned to more than one man. Therefore the algorithm terminates in a matching.

We illustrate how the (male-proposal) algorithm operates using Example 10.8 above. In the first round, $m _ { 1 }$ proposes to $w _ { 2 } , m _ { 2 }$ to $w _ { 1 }$ , and $m _ { 3 }$ to $w _ { 1 }$ . At the end of this round $w _ { 1 }$ is the only woman to have received two proposals. One from $m _ { 3 }$ and the other from $m _ { 2 }$ . Since she ranks $m _ { 3 }$ above $m _ { 2 }$ , she keeps $m _ { 3 }$ and rejects $m _ { 2 }$ . Since $m _ { 3 }$ is the only man to have been rejected, he is the only one to propose again in the second round. This time he proposes to $w _ { 3 }$ . Now each woman has only one proposal and the algorithm terminates with the matching $\{ ( m _ { 1 } , w _ { 2 } ) , ( m _ { 2 } , w _ { 3 } ) , ( m _ { 3 } , w _ { 2 } ) \}$ . It is easy to verify that the matching is stable and that it is different from the one presented earlier.

## Theorem 10.10 The male propose algorithm terminates in a stable matching.

proof Suppose not. Then there exists a blocking pair $( m _ { 1 } , w _ { 1 } )$ with $m _ { 1 }$ matched to $w _ { 2 } , \mathrm { s a y } .$ , and $w _ { 1 }$ matched to $m _ { 2 }$ . Since $( m _ { 1 } , w _ { 1 } )$ is blocking and $w _ { 1 } \succ _ { m _ { 1 } } w _ { 2 }$ , in the proposal algorithm, $m _ { 1 }$ would have proposed to $w _ { 1 }$ before $w _ { 2 }$ . Since $m _ { 1 }$ was not matched with $w _ { 1 }$ by the algorithm, it must be because $w _ { 1 }$ received a proposal from a man that she ranked higher than $m _ { 1 }$ . Since the algorithm matches her to $m _ { 2 }$ it follows that $m _ { 2 } \sim _ { w _ { 1 } } m _ { 1 }$ . This contradicts the fact that $( m _ { 1 } , w _ { 1 } )$ is a blocking pair.

One could just as well have described an algorithm where the women propose and the outcome would also be a stable matching. Applied to the example above, this would produce a stable matching different from the one generated when the men propose. Thus, not only is a stable matching guaranteed to exist but there can be more than 1. If there can be more than one stable matching, is there a reason to prefer one to another? Yes. To explain why, some notation.

Denote a matching by $\mu .$ . the woman assigned to man m in the matching $\mu$ is denoted $\mu ( m )$ . Similarly, $\mu ( w )$ is the man assigned to woman w. A matching $\mu$ is male-optimal if there is no stable matching ν such that $\nu ( m ) \succ _ { m } \mu ( m ) \mathrm { o r } \nu ( m ) = \mu ( m )$ for all m with $\nu ( j ) \succ _ { j } \mu ( j )$ for at least one $j \in M$ . Similarly define female-optimal.

Theorem 10.11 The stable matchingproduced by the (male-proposal) Deferred Acceptance Algorithm is male-optimal.

proof Let $\mu$ be the matching returned by the male-propose algorithm. Suppose $\mu$ is not male optimal. Then, there is a stable matching ν such that $\nu ( m ) \nu _ { m } \ \mu ( m )$ or $\nu ( m ) = \mu ( m )$ for all m with $\nu ( j ) \succ _ { j } \mu ( j )$ for at least one $j \in M$ . Therefore, in the application of the proposal algorithm, there must be an iteration where some man j proposes to $\nu ( j )$ before $\mu ( j )$ since $\nu ( j ) \succ _ { j } \mu ( j )$ and is rejected by woman $\nu ( j )$ . Consider the first such iteration. Since woman $\nu ( j )$ rejects $j$ she must have received a proposal from a man i she prefers to man $j$ . Since this is the first iteration at which a male is rejected by his partner under ν it follows that man i ranks woman $\nu ( j )$ higher than $\nu ( i )$ . Summarizing, $i \succ _ { \nu ( j ) } j$ and $\nu ( j ) \succ _ { i } \nu ( i )$ implying that ν is not stable, a contradiction.

Clearly one can replace the word “male” by the word “female” in the statement of the theorem above. It is natural to ask if there is a stable matching that would be optimal with respect to both men and women. Alas, no. The example above has two stable matchings: one male optimal and the other female optimal. At least one female is strictly better off under the female optimal matching than the male optimal one and no female is worse off. A similar relationship holds when comparing the two stable matchings from the point of view of the men.

A stable matching is immune to a pair of agents opting out of the matching. We could be more demanding and ask that no subset of agents should have an incentive to opt out of the matching. Formally, a matching $\mu ^ { \prime }$ dominates a matching $\mu$ if there is a set $S \subset M \cup W$ such that for all $m , w \in S$ , both (i) $\mu ^ { \prime } ( m ) , \mu ^ { \prime } ( w ) \in S$ and (ii) $\mu ^ { \prime } ( m ) \succ _ { m } \mu ( m )$ and $\mu ^ { \prime } ( w ) \sim _ { w } \mu ( w )$ . Stability is a special case of this dominance condition when we restrict attention to sets S consisting of a single couple. The set of undominated matchings is called the core of the matching game. The next result is straightforward.

Theorem 10.12 The core ofthe matching game is the set ofall stable matchings.

Thus far we have assumed that the preference orderings of the agents is known to the planner. Now suppose that they are private information to the agent. As before we can associate a direct revelation mechanism with an algorithm for finding a stable matching.

Theorem 10.13 The direct mechanism associated with the male propose algorithm is strategy-prooffor the males.

proof Suppose not. Then there is a profile of preferences $\pi = ( \succ _ { m _ { 1 } } , \succ _ { m _ { 2 } }$ $\ldots , \succ _ { m _ { n } } )$ for the men, such that man $m _ { 1 }$ , say, can misreport his preferences and obtain a better match. To express this formally, let $\mu$ be the stable matching obtained by applying the male proposal algorithm to the profile $\pi$ . Suppose that $m _ { 1 }$ reports the preference ordering $\succ _ { \ast }$ instead. Let ν be the stable matching that results when the male-proposal algorithm is applied to the profile $\pi ^ { 1 } = ( \succ$ $\succ _ { m _ { 2 } } , \ldots , \succ _ { m _ { n } } )$ . For a contradiction, suppose $\nu ( m _ { 1 } ) \sim _ { m _ { 1 } } \mu ( m _ { 1 } )$ . For notational convenience we will write $a \succeq _ { m } b$ to mean that $a \succ _ { m } b { \mathrm { ~ o r ~ } } a = b$

First we show that $m _ { 1 }$ can achieve the same effect by choosing an ordering $\bar { \succ }$ where woman $\nu ( m _ { 1 } )$ is ranked first. Let $\pi ^ { 2 } = ( \bar { > } , > _ { m _ { 2 } } , . . . , > _ { m _ { n } } )$ . Knowing that ν is stable with respect to the profile $\pi ^ { 1 }$ we show that it is stable with respect to the profile $\pi ^ { 2 }$ . Suppose not. Then under the profile $\pi ^ { 2 }$ there must be a pair $( m , w )$ that blocks ν. Since ν assigns to $m _ { 1 }$ its top choice with respect to $\pi ^ { 2 } , m _ { 1 }$ cannot be part of this blocking pair. Now the preferences of all agents other than $m _ { 1 }$ are the same in $\pi ^ { 1 }$ and $\pi ^ { 2 }$ . Therefore, if $( m , w )$ blocks ν with respect to the profile $\pi ^ { 2 }$ , it must block ν with respect to the profile $\pi ^ { 1 }$ , contradicting the fact that ν is a stable matching under $\pi ^ { 1 }$

Let λ be the male propose stable matching for the profile $\pi ^ { 2 }$ . Since ν is a stable matching with respect to the profile $\pi ^ { 2 }$ . As λ is male optimal with respect to the profile $\pi ^ { 2 }$ , it follows that $\lambda ( m _ { 1 } ) = \nu ( m _ { 1 } )$

Thus we can assume that $\nu ( m _ { 1 } )$ is the top-ranked woman in the ordering $\succ _ { \ast }$ Next we show that the set $B = \{ m _ { j } \colon \mu ( m _ { j } ) \succ _ { m _ { j } } \nu ( m _ { j } ) \}$ is empty. This means that all men, not just $m _ { 1 }$ , are no worse off under ν compared to $\mu$ . Since ν is stable with respect to the original profile, $\pi$ this contradicts the male optimality of $\mu$ and completes the proof.

Suppose $B \neq \emptyset$ . Therefore, when the male proposal algorithm is applied to the profile $\pi ^ { 1 }$ , each $m _ { j } \in B$ is rejected by their match under $\mu , \mathrm { i } . \mathrm { e } . , \mu ( m _ { j } )$ . Consider the first iteration of the proposal algorithm where some $m _ { j }$ is rejected by $\mu ( m _ { j } )$ ). This means that woman $\mu ( m _ { j } )$ has a proposal from man $m _ { k }$ that she ranks higher, i.e., $m _ { k } \succ _ { \mu ( m _ { j } ) } m _ { j }$ . Since $m _ { k }$ was not matched to $\mu ( m _ { j } )$ ) under $\mu$ it must be that $\mu ( m _ { k } ) \sim _ { m _ { k } } \mu ( m _ { j } )$ . Hence $m _ { k } \in B$ , otherwise

$$
\mu (m _ {j}) \succeq m _ {k} v (m _ {k}) \succeq_ {m _ {k}} \mu (m _ {k}) \succ_ {m _ {k}} \mu (m _ {j}),
$$

which is a contradiction.

Since $m _ { k } \in B$ and $m _ { k }$ has proposed to $\mu ( m _ { j } )$ at the time man $m _ { j }$ proposes, it means that $m _ { k }$ must have been rejected by $\mu ( m _ { k } )$ prior to $m _ { j }$ being rejected, contradicting our choice of $m _ { j }$ .

The mechanism associated with the male propose algorithm is not strategy-proof for the females. To see why, it is enough to consider example. The male propose algorithm returns the matching $\{ ( m _ { 1 } , w _ { 2 } ) , ( m _ { 2 } , w _ { 3 } ) , ( m _ { 3 } , w _ { 1 } ) \}$ . In the course of the algorithm the only woman who receives at least two proposals is $w _ { 1 }$ . She received proposals from $m _ { 2 }$ and $m _ { 3 }$ . She rejects $m _ { 2 }$ who goes on to propose to $w _ { 3 }$ and the algorithm terminates.

Notice that $w _ { 1 }$ is matched with her second choice. Suppose now that she had rejected $m _ { 3 }$ instead. Then $m _ { 3 }$ would have gone on to proposes to $w _ { 2 }$ . Woman $w _ { 2 }$ now has a choice between $m _ { 1 }$ and $m _ { 3 }$ . She would keep $m _ { 3 }$ and reject $m _ { 1 }$ , who would go on to propose to $w _ { 1 }$ . Woman $w _ { 1 }$ would keep $m _ { 1 }$ over $m _ { 2 }$ and in the final matching be paired with a her first-rank choice.

It is interesting to draw an analogy between the existence of stable matchings and that of Walrasian equilibrium. We know (Chapter 6) that Walrasian equilibria exist. Furthermore, they are the solutions of a fixed point problem. In the cases when they can be computed efficiently it is because the set of Walrasian equilibria can be described by a set of convex inequalities. The same can be said of stable matchings. The set of stable matchings is fixed points of a nondecreasing function defined on a lattice. In addition, one can describe the set of stable matchings as the solutions to a set of linear inequalities.

## 10.4.1 A Lattice Formulation

We describe a proof of the existence of stable matchings using Tarski’s fixed point theorem. It will be useful to relax the notion of a matching. Call an assignment of women to men such that each man is assigned to at most one woman (but a woman may be assigned to more than one man) a male semimatching. The analogous object for women will be called a female semimatching. For example, assigning each man his first choice would be a male semimatching. Assigning each woman her third choice would be an example of a female semimatching.

A pair of male and female semimatchings will be called a semimatching which we will denote by µ, ν, etc. An example of a semi-matching would consist of each man being assigned his first choice and each woman being assigned her last choice.

The woman assigned to the man m under the semi-matching $\mu$ will be denoted $\mu ( m )$ . If man m is assigned to no woman under $\mu$ , then $\mu ( m ) = m$ . Similarly for $\mu ( w )$ ). Next we define a partial order over the set of semimatchings. Write $\mu \succeq \nu$ if

(i) µ(m) <sub>m</sub> ν(m) or $\mu ( m ) = \mu ( m )$ for all $m \in M$ and

(ii) $\mu ( w ) \prec _ { w } \nu ( w ) \mathrm { o r } \mu ( w ) = \nu ( w )$ for all $w \in W$

Therefore $\mu \succeq \nu$ if all the men are better off under $\mu$ than in ν and all the women are worse off under $\mu$ than in ν.

Next we define the meet and join operations. Given two semimatchings $\mu$ and ν define $\lambda = \mu \vee \nu$ as follows:

(i) $\lambda ( m ) = \mu ( m ) \operatorname { i f } \mu ( m ) \succ _ { m } \nu ( m )$ otherwise $\lambda ( m ) = \nu ( m )$

(ii) $\lambda ( w ) = \mu ( w ) { \mathrm { i f } } \mu ( w ) \prec _ { w } \nu ( w )$ otherwise λ(w) = ν(w).

Define $\lambda ^ { \prime } = \mu \wedge \nu$ as follows:

(i) λ<sup></sup>(m) = µ(m) if µ(m) ≺ ν(m) otherwise λ(m) = ν(m),

(ii) λ(w) = µ(w) if µ(w)  ν(w) otherwise λ(w) = ν(w).

With these definitions it is easy to check that the set of semimatchings forms a compact lattice.

Now define a function f on the set of semi-matchings that is nondecreasing. Given a semi-matching $\mu$ define $f ( \mu )$ to be the following semi-matching:

(i) $f ( \mu ) ( m )$ is man m’s most preferred woman from the set $\{ w : m \succ _ { w } \mu ( w ) , m = \mu ( w ) \}$ If this set is empty set $f ( \mu ) ( m ) = m .$

(ii) $f ( \mu ) ( w )$ is woman $w \mathbf { \bar { s } }$ most preferred man from the set $\{ m \colon w \succ _ { m } \mu ( m ) , w = \mu ( m ) \}$ If this set is empty set $f ( \mu ) ( w ) = w$

It is clear that f maps semi-matchings into semi-matchings.

Theorem 10.14 There is a semi-matching µ such that $f ( \mu ) = \mu$ and that $\mu$ is a stable matching.

proof We use Tarski’s theorem. It suffices to check that f is nondecreasing. Suppose $\mu \succeq \nu$ . Pick any $m \in M$ . From the definition of $\succeq$ , the women are worse off under $\mu$ than in ν. Thus

$$
\{w: m \succ_ {w} \nu (w) \} \subseteq \{w: m \succ_ {w} \mu (w) \}
$$

and so $f ( \mu ) ( m ) \succ _ { m } f ( \nu ) ( m ) { \mathrm { o r } } f ( \mu ) ( m ) = f ( \nu ) ( m )$ . A similar argument applies for each $w \in W$ . Thus $f$ is nondecreasing.

Since the conditions of Tarski’s theorem hold, it follows that there is a semimatching $\mu$ such that $f ( \mu ) = \mu$ . We show that the semi-matching is a stable matching.

By the definition of a semi-matching we have for every $m \in M , \mu ( m )$ single valued as is $\mu ( w )$ for all $w \in W$ . To show that $\mu$ is a matching, suppose not. Then there is a pair $m _ { 1 } , m _ { 2 } \in M$ , say, such that $\mu ( m _ { 1 } ) = \mu ( m _ { 2 } ) = w ^ { * }$ . Since $f ( \mu ) = \mu$ it follows that $w ^ { * }$ is $m _ { 1 } \mathrm { { ' s } }$ top-ranked choice in $\{ w : m _ { 1 } \succ _ { w } \mu ( w ) , m _ { 1 } = \mu ( w ) \}$ and $m _ { 2 } \mathrm { ^ { * } s }$ top ranked choice in $\{ w : m _ { 2 } \succ _ { w } \mu ( w ) , m _ { 2 } = \mu ( w ) \}$ . From this we deduce that $\mu ( w ^ { * } ) = m _ { 3 }$ where $m _ { 1 } , m _ { 2 } > ^ { w ^ { * } } m _ { 3 }$ . However, $m _ { 3 } = \mu ( w ^ { * } ) = f ( \mu ^ { * } ) ( w ^ { * } )$ 1 which is woman $w ^ { \ast } \boldsymbol { \mathfrak { s } }$ top-ranked choice in {m : $w ^ { * } \succ m \mu ( m ) , \mu ( m ) = w ^ { * } \}$ . Since $m _ { 1 } , m _ { 2 }$ are members of this set, we get a contradiction.

To show that the matching µ is stable suppose not. Then there must be a blocking pair $( m ^ { * } , w ^ { * } )$ ). Let $w ^ { \prime } = \mu ( m ^ { * } )$ and $m ^ { \prime } = \mu ( w ^ { * } ) , m ^ { \prime } \neq m ^ { * }$ and $w ^ { \ast } \neq$ $w ^ { \prime }$ . Since $( m ^ { * } , w ^ { * } )$ is blocking, $m ^ { * } \succ _ { w ^ { * } } m ^ { \prime }$ and $w ^ { * } \succ _ { m ^ { * } } w ^ { \prime }$ . Now $w ^ { \prime } = \mu ( m ^ { * } ) =$ $f ( \mu ) ( m ^ { * } )$ , which is man $m ^ { * } \mathrm { { } } ^ { , } \mathrm { { s } }$ top-ranked choice from $\{ w : m ^ { * } \succ w \mu ( w ) , m ^ { * } =$ $\mu ( w ) \}$ }. But this set contains $w ^ { * }$ , which is ranked higher by man $m ^ { * }$ than $w ^ { \prime }$ , a contradiction.

## 10.4.2 The LP Formulation

One can formulate the problem of finding a stable matching as the solution to a set of linear inequalities. For each man m and woman w let $x _ { m w } = 1$ if man m is matched with woman w and zero otherwise. Then, every stable matching must satisfy the

following.

$$
\begin{array}{r l} \sum_ {w \in W} x _ {m w} = 1 & \quad \forall m \in M \\ \sum_ {m \in M} x _ {m w} = 1 & \quad \forall w \in W \\ \sum_ {j <   _ {m} w} x _ {m j} + \sum_ {i <   _ {w} m} x _ {i w} + x _ {m w} \leq 1 & \quad \forall m \in M, w \in W \\ x _ {m w} \geq 0 & \quad \forall m \in M, w \in W \end{array}
$$

Let P be the polyhedron defined by these inequalities.

The first two constraints of P ensure that each agent is matched with exactly one other agent of the opposite sex. The third constraint ensures stability. To see why, suppose $\sum _ { j < _ { m } w } x _ { m j } = 1$ and $\begin{array} { r } { \sum _ { i < _ { w } m } x _ { i w } = 1 } \end{array}$ . Then man m is matched to a woman, j that he ranks below w. Similarly, woman w is matched to a man she ranks below m. This would make the pair (m, w) a blocking pair.

Theorem 10.15 P is the convex hull ofall stable matchings.

## 10.4.3 Extensions

We have been careful to specify that preferences are strict. If we allow for indifference, Theorem 10.7 becomes false. This is because there are instances of the stable matching problem in which no male or female optimal stable matching exists. The other theorems stated above continue to hold in the presence of indifferences.

We also limited ourselves to one-to-one matchings. There are situations where one side of the market wishes to match with more than one agent. The college admissions market is the classic example. Each student can be assigned to at most one college but each college can be assigned to many students. In this more general setup colleges will have preferences over subsets of students. In the absence of any restrictions on these preferences a stable matching need not exist. One restriction on preferences for which the results above carry over with no change in statement or proof is the quota model. Each college has a strict preference ordering over the students and a quota r of students it wishes to admit. Consider two subsets, S and T, of students of size r that differ in exactly one student. The college prefers the subset containing the more preferred student.

A third extension is to relax the bipartite nature of the stable matching problem. The nonbipartite version is called the stable roommates problem. Suppose that a set of N individuals such that |N| is even. A matching in this setting is a partition of N into disjoint pairs of individuals (roommates). Each individual has a strict preference ordering over the other individuals that they would like to be paired with. As before, a matching is unstable if there exists a pair who prefer each other to the person they are matched with. Such a pair is called blocking. Unlike the stable matching problem, stable roommates need not exist as the following four person example illustrates.

<table><tr><td> $\succ_1$ </td><td> $\succ_2$ </td><td> $\succ_3$ </td><td> $\succ_4$ </td></tr><tr><td>3</td><td>1</td><td>2</td><td>2</td></tr><tr><td>2</td><td>3</td><td>1</td><td>1</td></tr><tr><td>4</td><td>4</td><td>4</td><td>4</td></tr></table>

Each column lists the preference ordering that one agent has over the others. A matching that pairs agent 1 with agent 4 will always be blocked by the pair (1, 2). A matching that pairs 2 with 4 will be blocked by (2, 3). A matching that pairs 3 and 4 will be blocked by (3, 1).

An $O ( | N | ^ { 2 } )$ algorithm to determine if a stable matching exists is known. One can also associate a collection of linear inequalities with the stable roommates prob lem such that the system is feasible if and only if a stable roommates solution exists.

## 10.5 Future Directions

While the models in this chapter have been studied and extended in a variety of ways, there are plenty of open questions for the creative researcher.

One direction of future research on the single-peaked preference model of Section 10.2 would be to consider choosing multiple alternatives (locations) on an interval (or more general graph) when agents’ preferences are single-peaked with respect to the one location that is closest to his peak. As an idealized example, when downloading files on the Internet one cares only about the location (distance) of the closest “mirror” site. If a planner can elicit preferences to choose the location of k mirrors on a network, how can this be done in a strategy-proof way?

As for the house allocation model of Section 10.3 and the stable matching model of Section 10.4, observe that both models are static in nature. Yet, there are a variety of dynamic environments that resemble these models in important ways. As an example, take the problem of allocating kidneys. Until quite recently those needing a kidney transplant would have to wait in a queue (the wait list) for an available kidney that would be an appropriate “fit” or else find a donor fulfilling the appropriate medical conditions.

More recently, however, exchange systems have been implemented which al low kidney patients to “swap” their incompatible (but willing) friends and rela tives who are willing to donate a kidney. (Suppose that Alice needs a kidney, and her incompatible friend Bob is willing to donate; also suppose that Carmina and Dijen are in a similar situation. If Alice and Dijen are compatible, and if Carmina and Bob are compatible, then a compatible “swap” can be arranged.) Static versions of such a model have been analyzed by Roth, Sonmez, and¨ Unver<sup>¨</sup> (2004).

Those authors and others have developed a substantial literature around this important problem. If donors and recipients arrive dynamically to such a setting, how should swaps be arranged?

## 10.6 Notes and References

The canonical results for the single-peaked preference model are provided by Moulin (1980), who proved Theorems 10.2 and 10.4 with the additional requirement that rules take agents’ peaks as their only input. Ching (1997) subsequently showed that this requirement is redundant when a rule is strategy-proof and onto.

Border and Jordan (1983) generalize these conclusions to multidimensional models where the outcome space is $\mathbb { R } ^ { k }$ . They restrict attention to separable preferences, i.e., under the assumption that an agent’s (relative) preferences over any one dimension are fixed, as we vary any other dimensions of the altnerative. For example with $k = 3$ if $( x _ { 1 } , x _ { 2 } , x _ { 3 } ) \succeq _ { i } ( x _ { 1 } ^ { \prime } , x _ { 2 } , x _ { 3 } )$ then separability would imply $( x _ { 1 } , y _ { 2 } , y _ { 3 } ) \succeq _ { i } ( x _ { 1 } ^ { \prime } , y _ { 2 } , y _ { 3 } )$ Border and Jordan show that a strategy-proof, onto rule for separable preferences must be decomposable into k (possibly different) one-dimensional rules. Of course, these one-dimensional rules must be generalized median voter schemes. For further reference on such generalizations, one should consult the survey of Barbera\` (2001).

Another direction in which these results have been generalized pertains to situations in which agents have single-peaked preferences on graphs. Schummer and Vohra (2004) obtain two types of result, depending on whether the graph contains any cycle. Finally, the book of Austen-Smith and Banks (2005). contains more details on the key results of this literature, and a proof of Theorem 10.4.

The house allocation problem was introduced by Herbert Scarf and Lloyd Shapley (1974). The TTCA is attributed by these authors to David Gale. The idea that the house allocation problem can be used as a model for kidney exchanges is discussed in Roth et al. (2004).

The stable matching problem was introduced by David Gale and Lloyd Shapley (1962). The first algorithm for finding a stable matching was developed a decade earlier in 1951 to match interns to hospitals (Stalnaker, 1953). The intrinsic appeal of the model has inspired three books. The first, by Donald Knuth (1976) uses the stable matching problem as a vehicle to illustrate some of the basic ideas in the analysis of algorithms. The book by Gusfield and Irving (1989) is devoted to algorithmic aspects of the stable matching problem and some of its relatives. On the economics side, the book by Roth and Sotomayor (1991) gives a complete game theoretic treatment of the stable matching problem as well as some of its relatives.

The lattice theoretic treatment of the stable matching problem goes back to Knuth (1976). The proof of existence based on Tarski’s fixed point theorem is due to Adach (2000). In fact, the proposal algorithm is exactly one of the algorithms for finding a fixed point when specialized to the case of stable matchings.

The linear programming formulation of the stable matching problem is due to Vande Vate (1989). The extension of it to the stable room mates problem can be found in Teo and Sethuraman (1998). Gusfield and Irving (1989) give a full algorithmic account of the stable roommates problem.

In parallel, studies have been made of matching models where monetary transfers are allowed. This has inspired models that unify both the stable matching problem as well as matching problems where monetary transfers are allowed. Descriptions can be found in Fleiner (2003) and Hatfield and Milgrom (2005).

## Bibliography

H. Adachi. On a characterization of stable matchings. Economics Letters, 68:43–49, 2000.

D. Austen-Smith and J. Banks. Positive Political Theory II: Strategy and Structure. University of Michigan Press, 2005.

S. Barbera. An introduction of strategy-proof social choice functions.\` Soc. Choice Welfare, 18(4):619– 653, 2001.

K. Border and J. Jordan. Straightforward elections, unanimity and phantom voters. Rev. Econ. Stud., 50(1):153–170, 1983.

S. Ching. Strategy-proofness and Amedian voters.<sup>ˆ</sup> Intl. J. Game Theor., 26(4):473–490, 1997.

T. Fleiner. Some results on stable matchings and fixed points. Math. Oper. Res., 28(1):103–126, 2003.

D. Gale and L.S. Shapley. College admissions and the stability of marriage. Amer. Math. Monthly, 69(1):9–15, 1962.

A. Gibbard. Manipulation of voting schemes: A general result. Econometrica, 41(4):587–601, 1973.

D. Gusfield and R.W. Irving. The Stable Marriage Problem: Structure and Algorithms. MIT Press, 1989.

J.W. Hatfield and P.R. Milgrom. Matching with contracts. Amer. Econ. Rev., 95(4):913–935, 2005.

D. Knuth. Marriages Stables. Les Presses de l’Universite de Montreal, 1976.

H. Moulin. On strategy proofness and single peakedness. Public Choice, 35(4):437–455, 1980.

A. E. Roth and M. Sotomayor. Two-Sided Matching: A Study in Game-Theoretic Modelling and Analysis. Cambridge University Press, 1991.

A. E. Roth, T. Sonmez, and M. U.¨ Unver. Kidney exchange.<sup>¨</sup> Q. J. Econ., 119(2):457–488, 2004.

M. Satterthwaite. Strategy-proofness and arrow’s conditions. J. Econ. Theor., 10(2):187–217, 1975

J. Schummer and R.V. Vohra. Strategy-proof location on a network. J. Economic Theory, 104(2):405– 428, 2004.

L.S. Shapley and H. Scarf. On cores and indivisibility. J. Math. Econ., 1(1):23–28, 1974

J. M. Stalnaker. The matching program for intern placement: The second year of operation. J. Med. Educ., 28(1):13–19, 1953.

C. P. Teo and J. Sethuraman. Geometry of fractional stable matchings and its applications. Math. Oper. Res., 23(4):874–891, 1998.

J. H. VandeVate. Linear programming brings marital bliss. Oper. Res. Lett., 8(3):147–153, 1989.

## Exercises

10.1 To what extent is Lemma 10.1 sensitive to the richness of the preference domain? For example, does the result hold if the preference domain is even smaller, e.g., containing only symmetric single-peaked preferences?

10.2 Suppose that an anonymous rule described in Theorem 10.2 has parameters $( y _ { m } ) _ { m = 1 } ^ { n - 1 }$ . Express this rule as a generalized median voter scheme with parameters $( \alpha _ { S } ) _ { S \subseteq N }$

10.3 Suppose that a rule f is strategy-proof and onto, but not necessarily anonymous. Fix the preferences of agents 2 through $n , ( \succeq 2 , . . . . , \succeq _ { n } )$ , and denote the outcome obtainable by agent 1 as

$$
O = f (\cdot , \succeq_ {2}, \dots , \succeq_ {n}) = \{x \in [ 0, 1 ]: \exists \succeq_ {1} \in \mathcal {R} \text {s.t.} f (\succeq_ {1}, \succeq_ {2}, \dots , \succeq_ {n}) \}.
$$

Show that $O = [ a$ , b] for some a, $b \in [ 0 ,$ , 1] (without appealing directly to Theorem 10.4).

## 10.4 Prove Theorem 10.4.

10.5 For the case of three agents, generalize Theorem 10.2 to a 3-leaved tree. Specifically, consider a connected noncyclic graph (i.e., a tree) with exactly three leaves, $\ell _ { 1 } , \ell _ { 2 } , \ell _ { 3 }$ . Preferences over such a graph are single-peaked if there is a peak $p _ { i }$ such that for any x in the graph, and any $\gamma$ in the (unique shortest) path from x to $p _ { i }$ $\gamma \succeq _ { i } x$ . The concepts of strategy-proofness, onto, and anonymity generalize in the straightforward way to this setting. Describe all the rules that satisfy these conditions for the case $n = 3$ . (Hint: first show that when all agents’ peaks are restricted to the interval $[ \ell _ { 1 } , \ell _ { 2 } ]$ , the rule must behave like one described in Theorem 10.2.) For the nonanonymous case with $n \geq 3$ , see Schummer and Vohra (2004).

10.6 Prove that the TTCA returns an outcome in the core of the house allocation game.

10.7 The TTC mechanism is immune to agents misreporting their preferences. Is it immune to agents misreporting the identity of their houses? Specifically, suppose a subset of agents trade among themselves first before participating in the TTC mechanism. Can all of them be strictly better off by doing so?

10.8 Consider an instance of the stable matching problem. Let ν be a matching (not necessarily stable) and $\mu$ the male optimal stable matching. Let $B = \{ m : \nu ( m ) > ^ { m }$ $\mu ( m ) \}$ . Show that if $B \neq \varnothing$ then there is a m<sup></sup> $\notin B$ and woman w such that (m, w) is a blocking pair for ν.
