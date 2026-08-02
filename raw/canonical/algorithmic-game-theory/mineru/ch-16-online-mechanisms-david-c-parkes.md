---
type: "book-chapter"
book_id: "algorithmic-game-theory"
chapter_id: "ch-16"
chapter_number: 16
chapter_title: "Online Mechanisms David C. Parkes"
source_pdf: "raw/inbox/manual-drop/PDF_B.pdf"
source_page_start: 432
source_page_end: 463
printed_page_start: 432
printed_page_end: 461
part_ids: ["algorithmic-game-theory-ch-16-part-017"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Online Mechanisms David C. Parkes (MinerU semantic layer)

<!-- source-pages: 432-463; printed-pages: 432-461; mineru-part: algorithmic-game-theory-ch-16-part-017 -->

David C. Parkes

## Abstract

Online mechanisms extend the methods of mechanism design to dynamic environments with multiple agents and private information. Decisions must be made as information about types is revealed online and without knowledge of the future, in the sense of online algorithms. We first consider single valued preference domains and characterize the space of decision policies that can be truthfully implemented in a dominant strategy equilibrium. Working in a model-free environment, we present truthful auctions for domains with expiring items and limited-supply items. Turning to a more general preference domain, and assuming the existence of a probabilistic model for agent types, we define a dynamic Vickrey–Clarke–Groves mechanism that is efficient and Bayes–Nash incentive compatible. We close with some thoughts about future research directions in this area.

## 16.1 Introduction

The decision problem in many multiagent problem domains is inherently dynamic rather than static. Consider, for instance, the following environments:

 Selling seats on an airplane to buyers arriving over time.

 Allocating computational resources (bandwidth, CPU, etc.) to jobs arriving over time.

 Selling adverts on a search engine to a possibly changing group of buyers and with uncertainty about the future supply of search terms.

 Allocating tasks to a dynamically changing team of agents.

In each of these settings at least one of the following is true: either agents are dynamically arriving or departing, or there is uncertainty about the set of feasible decisions in the future. These dynamics present a new challenge when seeking to sustain good systemwide decisions in multiagent systems with self-interested agents.

This chapter introduces the problem of online mechanism design (online MD), which generalizes the theory of computational mechanism design to apply to dynamic problems. Decisions must be made dynamically and without knowledge of future agent types or future decision possibilities, in the sense of online algorithms.

## 16.1.1 Example: Dynamic Auction with Expiring Items

Consider a dynamic auction model with discrete time periods $T = \{ 1 , 2 , \dots , \}$ and a single indivisible item to allocate in each time period. The type of an agent $i \in$ $\{ 1 , \ldots , N \}$ is denoted $\theta _ { i } = ( a _ { i } , d _ { i } , w _ { i } ) \in T \times T \times \mathbb { R } _ { > 0 }$ . Agent i has arrival time $a _ { i }$ departure time $d _ { i }$ , value $w _ { i }$ for an allocation of a single unit of the item in some period $t \in [ a _ { i } , d _ { i } ]$ , and wants at most one unit. This type information is all private to an agent. We refer to this as the canonical expiring items environment.

The arrival time has a special meaning: it is the first period in which information about the type of this agent can be made available to the auction. (We say “can be made available” because a self-interested agent may choose to delay its report.) Assume quasi-linear utility, with utility $w _ { i } - p$ when the item is allocated in some $t \in [ a _ { i } , d _ { i } ]$ and payment $p$ is collected from the agent. Consider the following naive generalization of the Vickrey auction to this dynamic environment.

Auction 1. A bid from an agent is a claim about its type, $\hat { \theta } _ { i } = ( \hat { a } _ { i } , \hat { d } _ { i } , \hat { w } _ { i } )$ , necessarily made in period $t = \hat { a } _ { i }$ . Then: in each period t, allocate the item to the highest unassigned bid, breaking ties at random. Collect payment equal to the second-highes unallocated bid in this round.

Example 16.1 Jane sells ice cream and can make one cone each hour. The ice cream melts if it is not sold. There are three buyers, with types (1, 2, 100), (1, 2, 80), and (2, 2, 60), indicating (arrival, departure, value). Buyers 1 and 2 are willing to buy an ice cream in either period 1 or 2 while buyer 3 will only buy an ice cream in period 2. In this example, if every buyer is truthful then buyer 1 wins in period 1 for 80, stops bidding, and buyer 2 wins in period 2 for 60. But buyer 1 can do better. For example, buyer 1 can report type (1, 2, 61), so that buyer 2 wins in period 1 for 61, stops bidding, and then buyer 1 wins for 60 in period 2. Buyer 1 can also report type (2, 2, 80) and delay its bid until period 2, so that buyer 2 wins for 0 in period 1, stops bidding, and then buyer 1 wins fo 60 in period 2.

In a static situation the Vickrey auction is (dominant-strategy) truthful because an agent does not affect the price it faces. But, in a sequential setting an agent can choose the auction in which it participates and thus choose the other agents against which it competes and, in turn, the price faced. In fact, if every agent was impatient (with $d _ { i } = a _ { i } )$ , then, prices in future periods are irrelevant and the dominant strategy is to bid truthfully immediately upon arrival. Note also that buyer 1’s manipulation relied on a suitable bid from buyer 3 in period 2 and will not always be useful. Nevertheless, this serves to demonstrate the failure of dominant strategy truthfulness.

## 16.1.2 The Challenge of Online MD

The dynamics of agent arrivals and departures, coupled perhaps with uncertainty about the set of feasible decisions in the future and in general about the state of the environment, makes the problem of online MD fundamentally different from that of standard (offline) MD. Important new considerations in online MD are as follows.

(i) Decisions must be made without information about agent types not yet arrived, coupled perhaps with uncertainty about which decisions will be feasible in future periods.

(ii) Agents can misrepresent their arrival and departure time in addition to their valuation for sequences of decisions. Because of this agent strategies also have a temporal aspect.

(iii) Only limited misreports of type may be available, for instance it may be impossible for an agent to report an earlier arrival than its true arrival.

More generally, online MD can also model settings in which an agent’s type is revealed to itself over time and with its ability to learn dependent on decisions made by the online mechanism; e.g., a bidder needs to receive a resource to understand its value for the resource.

There are two main frameworks in which to study the performance of online mechanisms. The first is model-free and adopts a worst-case analysis and is useful when a designer does not have good probabilistic information about future agent types or about feasible decisions in future periods. The second is model-based and adopts an average-case analysis. As a motivating example, consider a search engine selling search terms to advertisers. This is a data-rich environment and it is reasonable to believe that the seller can build an accurate model to predict the distribution on types of buyers, including the process governing arrival and departures.

## 16.1.3 Outline

In Section 16.2 we present a general model for online MD and introduce the concept of limited misreports. Given this, we define direct-revelation, online mechanisms together with appropriate notions of incentive compatibility. Section 16.3 provides a characterization of truthful online mechanisms in the restricted domain of single-valued preferences and gives detailed examples of truthful, dynamic auctions. These auctions are analyzed within the framework of worst-case, competitive analysis. Section 16.4 considers general preference domains, and defines a dynamic Vickrey– Clarke–Groves mechanism, that is efficient and applicable when a model is available and common knowledge to agents. Section 16.5 closes with open problems and future directions.

## 16.2 Dynamic Environments and Online MD

The basic setting assumes risk neutral agents with quasi-linear utility functions, such that an agent acts to maximize the expected difference between its value from a sequence of decisions and its total payment. Consider discrete time periods $T = \{ 1 , 2 , \ldots \}$ , indexed by t and possibly infinite. A mechanism makes (and enforces) a sequence of decisions $k = ( k ^ { 1 } , k ^ { 2 } , \ldots ) \in \mathcal { O }$ , with decision $k ^ { t }$ made in period t. Let $k ^ { [ t _ { 1 } , t _ { 2 } ] } =$ $( k ^ { t _ { 1 } } , \ldots , k ^ { t _ { 2 } } )$ . The decisions made by a mechanism can depend on messages, such as bids, received from agents as well as uncertain events that occur in the environment.

For example, in sponsored search the realized supply of search terms determines the feasible allocation of user attention to advertisers.

An agent’s type, $\theta _ { i } = ( a _ { i } , d _ { i } , w _ { i } ) \in \Theta _ { i }$ , where $\Theta _ { i }$ is the set of possible types for agent i, defines a valuation function $v _ { i } ( \theta _ { i } , k ) \in \mathbb { R }$ on a sequence of decisions k and is private to an agent. Time periods $a _ { i } , d _ { i } \in T$ denote an agent’s arrival and departure period and $v _ { i } ( \theta _ { i } , k ) = v _ { i } ( \theta _ { i } , k ^ { [ a _ { i } , d _ { i } ] } ) ;$ ; i.e., an agent’s value is invariant to decisions outside of its arrival–departure window. In addition to restricting the scope of decisions that influence an agent’s value, the arrival period models the first period at which the agent is able to report its type to the mechanism.

The valuation component $w _ { i } \in \mathbb { W } _ { i }$ of an agent’s type, where $\mathbb { W } _ { i }$ denotes the set of possible valuations, parameterizes the agent’s valuation function and can be more expressive than a single real number. For example, in an online combinatorial auction this needs to convey enough information to define substitutes $\binom { 6 6 } { \cdot }$ want item A or item B but not both”) or complements (“I only want item A if I also get item $B ^ { \prime \prime } )$ preferences. Nor does the valuation need to be constant across all periods, for instance an agent could discount its future value in future periods $t > a _ { i }$ by discount factor $\gamma ^ { t - a _ { i } }$ for $\gamma \in ( 0 , 1 )$

## 16.2.1 Direct-Revelation Mechanisms

The family of direct-revelation, online mechanisms restricts the message that an agent can send to the mechanism to a single, direct claim about its type. For the most part we consider “closed” mechanisms so that an agent receives no feedback before reporting its type, and cannot condition its strategy on the report of another agent.

The mechanism state, $h ^ { t } \in H ^ { t }$ , where $H ^ { t }$ is the set of possible states in period $t ,$ captures all information relevant to the decision by the mechanism in that period. Let $\omega \in \Omega$ define the set of possible stochastic events that can occur in the environment, such as the realization of uncertain supply. This does not include the types of agents or any randomization within the mechanism itself. Write $\Omega = \Pi _ { t \in T } \Omega ^ { t }$ and let $\boldsymbol { \omega } ^ { t } \in \Omega ^ { t }$ denote the information about $\omega$ that is revealed in period t. Similarly, let $\theta ^ { t }$ denote the set of agent types reported in period t. Given this, it is convenient to define $h ^ { t } = ( \theta ^ { 1 } , \dots , \theta ^ { t } ; \omega ^ { 1 } , \dots , \omega ^ { t } ; k ^ { 1 } , \dots , k ^ { t - 1 } )$ . In practice, the state will be represented by a small, sufficient statistic of this information. The state space $\textstyle H = \bigcup _ { t } H ^ { t }$ may be finite, countably infinite, or continuous. This depends, in part, on whether agent types are discrete or continuous. Let $K ( h ^ { t } )$ denote the set of all feasible decisions in the current time period, assumed finite for all $h ^ { t }$ . Let $I ( h ^ { t } )$ denote the set of active agents in state $h ^ { t }$ , i.e. those agents for which $t \in [ a _ { i } , d _ { i } ]$

Definition 16.2 (direct-revelation online mechanism) A direct-revelation online mechanism, $M = ( \pi , x )$ , restricts each agent to making a single claim about its type, and defines decisionpolicy $\pi = \{ \pi ^ { t } \} ^ { t \in T }$ andpaymentpolicy, $x = \{ x ^ { t } \} ^ { t \in T }$ where decision $\pi ^ { t } ( h ^ { t } ) \in K ( h ^ { t } )$ is made in state $h ^ { t }$ and payment $x _ { i } ^ { t } ( h ^ { t } ) \in \mathbb { R }$ is collected from each agent $i \in I ( h ^ { t } )$

Decision policy π may be stochastic. The payment policy may collect payments from an agent across multiple periods. For notational convenience, we let $\pi ( \theta , \omega ) = ( k ^ { 1 } , k ^ { 2 } , \dots )$ denote the sequence of decisions, and $p _ { i } ( \theta , \omega ) \in \mathbb { R }$ denote the total payment collected from agent i, given type profile θ and a realization of uncertain events $\omega \in \Omega$

Example 16.3 Consider the canonical expiring items environment. The state $h ^ { t }$ can be defined as a list of reported agent types that are present in period t, indicating whether each agent is already allocated or not. Decision $k \in K ( h ^ { t } )$ decides whether to allocate the item in the current period to some agent that is present and unallocated.

Limited misreports constrain the strategy space available to agents in directrevelation, online mechanisms:

Definition 16.4 (limited misreports) Let $C ( \theta _ { i } ) \subseteq \Theta _ { i }$ for $\theta _ { i } \in \Theta _ { i }$ denote the set of available misreports to an agent with true type $\theta _ { i }$ .

In the standard model adopted in offline MD, it is typical to assume $C ( \theta _ { i } ) = \Theta _ { i }$ . We shall assume no early-arrival misreports, with $C ( \theta _ { i } ) = \{ \hat { \theta } _ { i } = ( \hat { a } _ { i } , \hat { d } _ { i } , \hat { w } _ { i } ) : a _ { i } \leq \hat { a } _ { i } \leq$ $\hat { d } _ { i } , \hat { w } _ { i } \in \mathbb { W } _ { i } \}$ ; i.e., agent i cannot report an earlier arrival because it does not know its type (or know about the mechanism) until $a _ { i }$ . Sometimes, we shall also assume no late-departure misreports, which together with no early arrivals provides $C ( \theta _ { i } ) =$ $\{ \hat { \theta } _ { i } = ( \hat { a } _ { i } , \hat { d } _ { i } , \hat { w } _ { i } ) : a _ { i } \le \hat { a } _ { i } \le \hat { d } _ { i } \le d _ { i } , \hat { w } _ { i } \in \mathbb { W } _ { i } \}$ . For example, we could argue that it is not credible to claim to have value for a ticket for a last minute Broadway show after 5 p.m. because the auctioneer knows that it takes at least 2 hours to get to the theater and the show starts at 7 p.m.

We restrict attention to mechanisms that are either dominant-strategy or Bayes– Nash incentive compatible. Let $\theta _ { - i } = ( \theta _ { 1 } , \ldots , \theta _ { i - 1 } , \theta _ { i + 1 } , \ldots ) , \Theta _ { - i } = \Pi _ { j \neq i } \Theta _ { j }$ , and $C ( \boldsymbol { \theta } _ { - i } ) = \Pi _ { j \neq i } C ( \boldsymbol { \theta } _ { j } )$ , and consider misreports $\theta _ { i } \in C ( \theta _ { i } )$

Definition 16.5 (DSIC) Online mechanism $M = ( \pi , x )$ is dominant-strategy incentive-compatible (DSIC) given limited misreports C if

$$
v _ {i} (\theta_ {i}, \pi (\theta_ {i}, \theta_ {- i} ^ {\prime}, \omega)) - p _ {i} (\theta_ {i}, \theta_ {- i} ^ {\prime}, \omega) \geq v _ {i} (\theta_ {i}, \pi (\hat {\theta} _ {i}, \theta_ {- i} ^ {\prime}, \omega)) - p _ {i} (\hat {\theta} _ {i}, \theta_ {- i} ^ {\prime}, \omega),
$$

for all $\hat { \theta } _ { i } \in C ( \theta _ { i } )$ , all $\theta _ { i }$ , all $\theta _ { - i } ^ { \prime } \in C ( \theta _ { - i } )$ , all $\theta _ { - i } \in \Theta _ { - i }$ , all $\omega \in \Omega$

It will be convenient to also adopt the terminology truthful in place of DSIC. The concept of DSIC requires that an agent maximizes its utility by reporting its true type whatever the reports of other agents and for all stochastic events $\omega .$ . When the decision policy itself is stochastic then DSIC requires that the expected utility is maximized from a truthful report, whatever the reports of other agents and (again) for all stochastic events ω. A randomized mechanism (i.e., one with a stochastic policy) is said to satisfy strong-truthfulness when truthful reporting is a dominant strategy for all random coin flips by the mechanism, and for all external stochastic events $\omega .$

For Bayes–Nash incentive compatibility (BNIC), we assume in addition that all agents know the correct probabilistic model of the distribution on types and uncertain events, and that this is common knowledge.

Definition 16.6 (BNIC) Online mechanism $M = ( \pi , x )$ is Bayes–Nash incentive-compatible (BNIC) given limited misreports C if

$$
\mathbb {E} \left\{v _ {i} \left(\theta_ {i}, \pi \left(\theta_ {i}, \theta_ {- i}, \omega\right)\right) - p _ {i} \left(\theta_ {i}, \theta_ {- i}, \omega\right) \right\} \geq \mathbb {E} \left\{v _ {i} \left(\theta_ {i}, \pi \left(\hat {\theta} _ {i}, \theta_ {- i}, \omega\right)\right) - p _ {i} \left(\hat {\theta} _ {i}, \theta_ {- i}, \omega\right) \right\},
$$

for all $\hat { \theta } _ { i } \in C ( \theta _ { i } )$ , all $\theta _ { i }$ , where the expectation is taken with respect to the distri bution on types $\theta _ { - i }$ , and stochastic events $\omega ,$ and any randomization within the policy.

BNIC is a weaker solution concept than DSIC because it requires only that truth revelation is a best response when other agents are also truthful, and in expectation given the distribution on agent types and on stochastic events in the environment.

## 16.2.2 Remark: The Revelation Principle

Commonly held intuition from offline MD suggests that focusing on the class of incentive compatible, direct-revelation online mechanisms is without loss of generality. However, if agents are unable to send messages to a mechanism in periods $t \notin [ a _ { i } , d _ { i } ]$ then this is not true.

Example 16.7 (failure of the revelation principle) Consider the model with no early-arrival misreports but allow for late-departure misreports. Consider two time periods $T = \{ 1 , 2 \}$ , a single unit of an indivisible item to allocate in either period and an environment with a single agent. Denote the type of the agent $( a _ { i } , d _ { i } , w _ { i } )$ with $w _ { i } > 0$ to denote its value for the item if allocated in period $t \in [ a _ { i } , d _ { i } ]$ . Suppose that possible types are (1, 1, 1) or (1, 2, 1). Consider an indirect mechanism that allows an agent to send one of messages {1, 2} in period 1 and {1} in period 2. Let $\phi$ denote a null message. Consider decision policy: $\pi ^ { 1 } ( 1 ) = 0 , \pi ^ { 1 } ( 2 ) = 1 , \pi ^ { 2 } ( 1 , z ) = \pi ^ { 2 } ( 2 , z ) = 0 .$ , for $z \in \{ 1 , \phi \}$ writing the state as the sequence of messages received and decision $k ^ { t } \in \{ 0 , 1 \}$ to indicate whether or not the agent is allocated in period $t \in \{ 1 , 2 \}$ . Consider payment policy: $x ^ { 1 } ( 1 ) = x ^ { 2 } ( 1 , \phi ) = x ^ { 2 } ( 1 , 1 ) = 0 , \ x ^ { 1 } ( 2 ) = 3 , x ^ { 2 } ( 2 , 1 ) =$ $- 2 . 0 1 , x ^ { 2 } ( 2 , \phi ) = 0$ . Type (1, 1, 1) will report message 1 in period 1 because reporting message 2 is not useful and it cannot report messages (2,1). Type (1, 2, 1) will report messages (2,1) and has no useful deviation. This policy cannot be implemented as a DSIC direct-revelation mechanism because type (1, 2, 1) is allocated in period 1 for payment 0.99, and so type (1, 1, 1) (which is unallocated if truthful) will want to report type (1, 2, 1).

The revelation principle fails in this example because the indirect mechanism prevents the agent from claiming a later departure than its true departure. In fact, the revelation principle continues to hold when misreports are limited to no-late departures in addition to no-early arrivals. A form of the revelation principle can also be recovered by introducing simple “heartbeat” messages into a direct-revelation mechanism, whereby an agent still makes a single report about its type but must also send a noninformative heartbeat message in every period $t \in [ \hat { a } _ { i } , \hat { d } _ { i } ] . ^ { 1 }$ <sup>1</sup> We leave the derivation of this “revelation principle plus heartbeat” result as an exercise.

With this in hand, and in keeping with the current literature on online mechanisms, we will focus on incentive-compatible, direct revelation online mechanisms in this chapter.

## 16.3 Single-Valued Online Domains

In this section we develop a methodology for the design of DSIC online mechanisms in the restricted domain ofsingle-valued preferences. We identify the central role ofmono tonic decision policies in the design of truthful online mechanisms. The methodology is illustrated in the design of a dynamic auction for two environments: (a) allocating a sequence of expiring items and (b) allocating a single, indivisible item in some period while adapting to information about agent types. Both auctions are model-free and we use competitive analysis to study their efficiency and revenue properties. We close the section with remarks that situate the study of truthful online mechanisms in the context of the wider mechanism design literature.

## 16.3.1 Truthfulness for Single-Valued Preference Domains

An agent with single-valued preferences has the same value, $r _ { i }$ , whenever any of a set of interesting decisions is made in some period $t \in [ a _ { i } , d _ { i } ]$ , and has value for at most one such decision. For example, in the single-item allocation problems considered earlier an agent’s interesting set was all decisions that allocate an item to the agent.

Let $\mathcal { L } _ { i } = \{ L _ { 1 } , \ldots , L _ { m } \}$ describe a language for defining interesting sets for agent $i ,$ , where $\textstyle L \subseteq K = \bigcup _ { h } K ( h )$ , for any $L \in { \mathcal { L } } _ { i }$ , defines a subset of single-period decisions. Let $\succeq _ { L }$ be a partial order defined on $\mathcal { L } _ { i }$ . The valuation component $w _ { i } \in \mathbb { W } _ { i }$ of an agent’s type, $\theta _ { i } = ( a _ { i } , d _ { i } , w _ { i } )$ , defines $w _ { i } = ( r _ { i } , L _ { i } )$ with $\mathbb { W } _ { i } = \mathbb { R } \times \mathcal { L } _ { i }$ . This picks out the interesting set and defines the value on decisions in that set.

Definition 16.8 (single-valued) A single-valued online domain is one where each agent i has a type $\theta _ { i } = ( a _ { i } , d _ { i } , ( r _ { i } , L _ { i } ) )$ ), with reward $r _ { i } \in \mathbb { R }$ and interesting set $L _ { i } \in { \mathcal { L } } _ { i }$ , where type $\theta _ { i }$ defines valuation:

$$
v _ {i} (\theta_ {i}, k) = \left\{ \begin{array}{l l} r _ {i}, & \text { if } k ^ {t} \in \bigcup_ {L: L \succeq_ {L} L _ {i}, L \in \mathcal {L} _ {i}} L \text { for some } t \in [ a _ {i}, d _ {i} ] \\ 0, & \text { otherwise }, \end{array} \right.\tag{16.1}
$$

To keep things simple, we assume that the set of interesting decisions is known by the mechanism and thus the private information is restricted to arrival, departure, and its value for a decision. We comment on how to relax this assumption at the end of the section. Given the known interesting-set assumption, define a partial-order $\preceq \theta$ on

types:

$$
\theta_ {1} \preceq_ {\theta} \theta_ {2} \equiv (a _ {1} \geq a _ {2}) \wedge (d _ {1} \leq d _ {2}) \wedge (r _ {1} \leq r _ {2}) \wedge (L _ {1} = L _ {2}).\tag{16.2}
$$

This will be sufficient because we will not need to reason about misreports of interesting set $L _ { i }$ . Consider the following example.

Example 16.9 (known single-minded combinatorial auction) Multiple units of indivisible, heterogeneous items G, are in uncertain supply and cannot be stored from one period to the next. Consider single-valued preferences, where interesting set $L _ { i } \in { \mathcal { L } } _ { i }$ has an associated bundle $S ( L _ { i } ) \subseteq G$ , and characterizes all single-period decisions that allocate agent i bundle $S ( L _ { i } )$ , irrespective of the allocation to other agents. Define partial order $L _ { 1 } \succeq _ { L } L _ { 2 } \equiv S ( L _ { 1 } ) \supseteq S ( L _ { 2 } )$ for all $L _ { 1 } , L _ { 2 } \in \mathcal { L } _ { i }$ . Agent i with type $\theta _ { i } = ( a _ { i } , d _ { i } , ( r _ { i } , L _ { i } ) )$ has value $r _ { i }$ when decision $k ^ { t }$ allocates a bundle containing at least $S ( L _ { i } )$ items to the agent in some period $t \in [ a _ { i } , d _ { i } ]$

The subsequent analysis is developed for deterministic policies. We adopt shorthand $\pi _ { i } ( \theta _ { i } , \theta _ { - i } , \omega ) \in \{ 0 , 1 \}$ to indicate whether policy π makes an interesting decision for agent i with type $\theta _ { i }$ in some period $t \in [ a _ { i } , d _ { i } ]$ , fixing type profile $\theta _ { - i }$ and stochastic (external) events $\omega \in \Omega$ . Since we are often considering auction domains, we may also refer to an interesting decision for an agent as an allocation to the agent. The analysis immediately applies to the case of stochastic policies when coupled with strong-truthfulness.<sup>2</sup> We elaborate more on stochastic policies at the end of the section.

Definition 16.10 (critical value) The critical-value for agent i given type $\theta _ { i } =$ $( a _ { i } , d _ { i } , ( r _ { i } , L _ { i } ) )$ and deterministic policy π in a single-valued domain, is defined as

$$
v _ {(a _ {i}, d _ {i}, L _ {i})} ^ {c} (\theta_ {- i}, \omega) = \left\{ \begin{array}{l l} \min r _ {i} ^ {\prime} \text {s.t.} \pi_ {i} (\theta_ {i} ^ {\prime}, \theta_ {- i}, \omega) = 1 & \text {for} \theta_ {i} ^ {\prime} = (a _ {i}, d _ {i}, (r _ {i} ^ {\prime}, L _ {i})) \\ \infty , \quad \text {if no such} r _ {i} ^ {\prime} \text {exists,} \end{array} \right.\tag{16.3}
$$

where types $\theta _ { - i }$ and stochastic events $\omega \in \Omega$ are fixed.

Definition 16.11 (monotonic) Deterministic policy π is monotonic if $( \pi _ { i } ( \theta _ { i }$ $\theta _ { - i } , \omega ) = 1 ) \wedge ( ( r _ { i } > v _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega ) ) \Rightarrow \pi _ { i } ( \theta _ { i } ^ { \prime } , \theta _ { - i } , \omega ) = 1 )$ for all $\theta _ { i } ^ { \prime } \succ _ { \theta } \theta _ { i }$ , for all $\theta _ { - i } .$ , all $\omega \in \Omega$

The “strict profit” condition, $r _ { i } > v _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega )$ , is added to prevent weak in difference when $\theta _ { i } ^ { \prime } \succ _ { \theta } \theta _ { i }$ and $r _ { i } ^ { \prime } = r _ { i }$ , and is redundant when $r _ { i } ^ { \prime } > r _ { i }$ . Say that an arrival-departure interval $[ a _ { i } ^ { \prime } , d _ { i } ^ { \prime } ]$ is tighter than $[ a _ { i } , d _ { i } ]$ if $a _ { i } ^ { \prime } \geq a _ { i }$ and $d _ { i } ^ { \prime } \leq d _ { i }$ , and weaker otherwise.

Lemma 16.12 The critical value to agent i is independent of reward $r _ { i }$ and (weakly) monotonically increasing in tighter arrival–departure intervals, given a deterministic, monotonic policy.

proof Fix some $\theta _ { - i } , \omega \in \Omega$ . Assume for contradiction that $\theta _ { i } ^ { \prime } \preceq _ { \theta } \theta _ { i }$ so that $a _ { i } ^ { \prime } \geq$ $a _ { i }$ and $d _ { i } ^ { \prime } \leq d _ { i }$ , but $v _ { ( a _ { i } ^ { \prime } , d _ { i } ^ { \prime } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega ) < v _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega )$ . Modify the reward of type $\theta _ { i } ^ { \prime } = ( a _ { i } ^ { \prime } , d _ { i } ^ { \prime } , ( r _ { i } ^ { \prime } , L _ { i } ) )$ such that $r _ { i } ^ { \prime } : = v _ { ( a _ { i } ^ { \prime } , d _ { i } ^ { \prime } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega )$ and modify the reward of type $\theta _ { i } = ( a _ { i } , d _ { i } , ( r _ { i } , L _ { i } ) )$ such that $r _ { i } : = v _ { ( a _ { i } ^ { \prime } , d _ { i } ^ { \prime } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega )$ . Now, we still have $\theta _ { i } ^ { \prime } \preceq _ { \theta } \theta _ { i }$ , but $\pi _ { i } ( \theta _ { i } ^ { \prime } , \theta _ { - i } , \omega ) = 1$ while $\pi _ { i } ( \theta _ { i } , \theta _ { - i } , \omega ) = 0$ and a contradiction with monotonicity.

Theorem 16.13 A monotonic, deterministic decision policy π can be truthfully implemented in a domain with known interesting set single-valued preferences, and no early-arrival and no late-departure misreports.

proof Define payment policy $x _ { i } ^ { t } ( h ^ { t } ) = 0$ for all $t \neq \hat { d } _ { i }$ , and with

$$
x _ {i} ^ {t} (h ^ {t}) = \left\{ \begin{array}{l l} v _ {(\hat {a} _ {i}, \hat {d} _ {i}, L _ {i})} ^ {c} (\hat {\theta} _ {- i}, \omega), & \text { if } \pi_ {i} (\hat {\theta} _ {i}, \hat {\theta} _ {- i}, \omega) = 1 \\ 0, & \text { otherwise } \end{array} \right.\tag{16.4}
$$

when $t = \hat { d } _ { i }$ . This critical-value payment is collected upon departure. Fix $\theta _ { - i } .$ $\theta _ { i } = ( a _ { i } , d _ { i } , ( r _ { i } , L _ { i } ) )$ , and $\omega \in \Omega$ , assume that agent i is truthful, and proceed by case analysis. (a) If agent i is not allocated, $v _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega ) > r _ { i }$ and to be allocated, the agent must report some $\theta _ { i } ^ { \prime } \succ _ { \theta } \theta _ { i }$ , which it can only do with a report $\theta _ { i } ^ { \prime } = ( a _ { i } , d _ { i } , ( r _ { i } ^ { \prime } , L _ { i } ) )$ , and $r _ { i } ^ { \prime } > r _ { i }$ , by limited misreports. But since the critical value is greater than its true value $r _ { i }$ , it will have negative utility if it wins for $r _ { i } ^ { \prime }$ (b) If agent i is allocated, its utility is nonnegative since $v _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega ) \leq r _ { i }$ and it does not want to report a type for which it would not be allocated. Consider any report $\theta _ { i } ^ { \prime } \in C ( \theta _ { i } )$ for which the agent continues to be allocated. But, the critical value for $\theta _ { i } ^ { \prime }$ is (weakly) greater than for $\theta _ { i }$ since it is independent of the reported reward $r _ { i } ^ { \prime }$ and weakly increasing for an alternate arrival–departure interval since it must be tighter by limited misreports, and then by appeal to Lemma 16.12.

We turn now to identifying necessary conditions for truthfulness. An online mechanism satisfies individual rationality (IR) when every agent has nonnegative utility in equilibrium. This is required when agents cannot be forced to participate in the mechanism.

Lemma 16.14 (critical payment) In a (known interesting set) single-valued preference domain, any truthful online mechanism that is definedfor a deterministic decision policy and satisfies IR must collect a payment equal to the critical valuefrom each allocated agent.

proof Fix $\theta _ { - i }$ and $\omega \in \Omega$ . Payment $p _ { i } ( \theta _ { i } , \theta _ { - i } , \omega )$ , made by agent i contingent on successful allocation, cannot depend on reward $r _ { i }$ because if $p _ { i } ( \theta _ { i } , \theta _ { - i } , \omega ) < p _ { i } ( \theta _ { i } ^ { \prime } , \theta _ { - i } , \omega )$ for $\theta _ { i } = ( a _ { i } , d _ { i } , ( r _ { i } , L _ { i } ) )$ and $\theta _ { i } ^ { \prime } = ( a _ { i } , d _ { i } , ( r _ { i } ^ { \prime } , L _ { i } ) )$

and $r _ { i } ^ { \prime } \neq r _ { i }$ and min $( r _ { i } ^ { \prime } , r _ { i } ) \ge v _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega )$ then an agent with type $\theta _ { i } ^ { \prime }$ should report type $\theta _ { i }$ . Fix type $\theta _ { i }$ such that $\pi _ { i } ( \theta _ { i } , \theta _ { - i } , \omega ) = 1$ . Now, if $p _ { i } ( \theta _ { i } , \theta _ { - i } , \omega ) <$ $v _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega )$ then an agent with type $\theta _ { i } ^ { \prime } = ( a _ { i } , d _ { i } , ( r _ { i } ^ { \prime } , L _ { i } ) )$ ) and $p _ { i } ( \theta _ { i } , \theta _ { - i } , \omega ) <$ $r _ { i } ^ { \prime } < v _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega )$ should report $\theta _ { i }$ . This is possible even with negative payment $p _ { i } ( \theta _ { i } , \theta _ { - i } , \omega )$ as long as rewards can also be negative. On the other hand, if $v _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega ) < p _ { i } ( \theta _ { i } , \theta _ { - i } , \omega )$ then the mechanism fails IR for an agent with type $\begin{array} { r } { \theta _ { i } ^ { \prime } = ( a _ { i } , d _ { i } , ( r _ { i } ^ { \prime } , L _ { i } ) ) } \end{array}$ ) and $v _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega ) < r _ { i } ^ { \prime } < p _ { i } ( \theta _ { i } , \theta _ { - i } , \omega )$ .□

Say that a domain satisfies reasonable misreporting when an agent with type $\theta _ { i }$ has available at least misreports $\theta _ { i } ^ { \prime } \in C ( \theta _ { i } )$ ) with $a _ { i } ^ { \prime } \geq a _ { i } , d _ { i } ^ { \prime } \leq d _ { i }$ and any reward $r _ { i } ^ { \prime } .$ .

Theorem 16.15 In a known interesting set single-valued preference domain with reasonable misreporting, any deterministic policy π that can be truthfully implemented in an IR mechanism that does not pay unallocated agents must be monotonic.

proof Fix $\theta _ { - i } , ~ \omega \in \Omega$ . Assume, for contradiction, that $\theta _ { i } \prec _ { \theta } \theta _ { i } ^ { \prime }$ with $\theta _ { i } = ( a _ { i } , d _ { i } , ( r _ { i } , L _ { i } ) )$ and $\theta _ { i } ^ { \prime } = ( a _ { i } ^ { \prime } , d _ { i } ^ { \prime } , ( r _ { i } ^ { \prime } , L _ { i } ) )$ , but $\pi _ { i } ( \theta _ { i } , \theta _ { - i } , \omega ) = 1 _ { : }$ , value $r _ { i } > v _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega )$ and $\pi _ { i } ( \theta _ { i } ^ { \prime } , \theta _ { - i } , \omega ) = 0$ . We must have $p _ { i } ( \theta _ { i } , \theta _ { - i } , \omega ) =$ $v _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega )$ by Lemma 16.14. Thus, agent i with type $\theta _ { i }$ must have strictly positive utility in the mechanism. On the other hand, the agent with type $\theta _ { i } ^ { \prime } \succ _ { \theta } \theta _ { i }$ is not allocated, makes nonnegative payment, and has (weakly) negative utility. But, an agent with type $\theta _ { i } ^ { \prime }$ can report $\theta _ { i }$ , which presents a contradiction with truthfulness.

The restriction that losing agents do not receive a payment plays an important role. To see this, consider a domain with no late-departure misreports, fix $\theta _ { - i }$ , and consider a single-item valuation with possible types $\Theta _ { i } = \{ ( 1 , 1 , \ S 1 0 ) , ( 1 , 2 , \ S 1 0 ) \}$ }. Policy $\pi _ { i } ( ( 1 , 1 , \mathbb { S } 1 0 ) , \theta _ { - i } ) = 1$ and $\pi _ { i } ( ( 1 , 2 , \mathbb { S } 1 0 ) , \theta _ { - i } ) = 0$ is nonmonotonic, but can be truth fully implemented with payments $p _ { i } ( ( 1 , 1 , \ S 1 0 ) , \theta _ { - i } ) = 8$ and ${ p } _ { i } ( ( 1 , 2 , \$ 1 0 ) , \theta _ { - i } ) =$ $- 1 0 0 .$

Monotonic-Late. Theorem 16.13 can be generalized to a domain with arbitrary misreports of departure. For a particular $\theta _ { - i } , \omega \in \Omega$ and type $\theta _ { i } = ( a _ { i } , d _ { i } , ( r _ { i } , L _ { i } ) )$ ), define the critical departure, $d _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega )$ , as the earliest departure $d _ { i } ^ { \prime } \leq d _ { i }$ for which $v _ { ( a _ { i } , d _ { i } ^ { \prime } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega ) = v _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { c } ( \theta _ { - i } , \omega )$ . This is the earliest departure time that agent i <sup>i</sup>could have reported without increasing the critical value. Given this, we say that policy π is monotonic-late if it is monotonic and if no interesting decision is made for agent i before its critical departure period. A monotonic-late, deterministic decision policy π can be truthfully implemented in a domain with no early-arrival misreports but arbitrary misreports of departure. Moreover, this requirement of monotonic-late is necessary for truthfulness in this environment.

## 16.3.2 Example: A Dynamic Auction with Expiring Items

For our first detailed example we revisit the problem of selling an expiring item, such as ice cream, time on a shared computer, or network resources, to dynamically arriving buyers. This is the canonical expiring items environment. Assume for notational convenience that the time horizon is finite. We design a strongly truthful online auction that includes random tie-breaking and satisfies monotonicity (however ties are broken).

We assume no early-arrival and no late-departure misreports. The no late-departure assumption can be readily motivated in physical environments. For ice cream, think about a tour group that will be leaving at a designated time so that it is not credible to claim a willingness to wait for an ice cream beyond that period. For network resources, such as an auction for access to WiFi bandwidth in a coffee house, think about requiring a user to be present for the entire period of time reported to the mechanism. A technical argument for why we need this assumption is also provided below.<sup>3</sup>

Competitive analysis. We perform a worst-case analysis and consider the performance of the mechanism, given a sequence of types that are generated by an “adversary” whose task it is to make the performance as bad as possible. Of particular relevance is the method of competitive analysis, typically adopted in the study of online algorithms. The following question is asked: how effectively does the performance of the online mechanism “compete” with that ofan offline mechanism that is given complete information about thefuture arrival ofagent types? This question is asked in the worst-case, for an adversarially defined input.

Competitive analysis is most easily justified when the designer does not have a good model of the environment. As a motivating example, consider selling a completely new product or service, for which it is not possible to conduct market research to get a good model of demand. Competitive analysis can also lead to mechanisms that enjoy good average-case performance in practice, provide insight into how to design robust mechanisms, and produce useful “lower-bounds.” A lower-bound for a problem makes a statement about the best possible performance that can be achieved by any mechanism. Online mechanisms are of special interest when their performance matches the lower bound.

In performing competitive analysis, one needs to define: an optimality criterion; a model of the power of the adversary is selecting worst-case inputs; and an offline benchmark, defined with perfect information about the future. We are interested in the efficiency of a dynamic auction for expiring items and adopt as our optimality criterion the value of the best possible offline allocation. This can be computed as follows:

$$
V ^ {*} (\theta) = \max _ {x, y} \sum_ {i = 1} ^ {N} y _ {i} w _ {i}\tag{16.5}
$$

$$
\text { s.t. } \quad \sum_ {t = a _ {i}} ^ {d _ {i}} x _ {i t} \geq y _ {i}, \quad \forall i \in \{1, \dots , N \}\tag{16.6}
$$

$$
\sum_ {i: t \in [ a _ {i}, d _ {i} ]} x _ {i t} \leq 1, \quad \forall t \in T,\tag{16.7}
$$

where $y _ { i } \in \{ 0 , 1 \}$ indicates whether bid i is allocated and $x _ { i t } \in \{ 0 , 1 \}$ indicates the period in which it is allocated.<sup>4</sup> For our adversarial model, we consider a powerful adversary that is able to pick arbitrary agent types, including the value, arrival, and departure of agents.

Let $z \in { \mathcal { Z } }$ denote the set of inputs available to the adversary and $\theta _ { z }$ the corresponding type profile. Let ${ \mathrm { V a l } } ( \pi ( \theta _ { z } ) )$ denote the total value of the decisions made by policy π given input $\theta _ { z }$ . An online mechanism is c-competitivefor efficiency if

$$
\min _ {z \in \mathcal {Z}} \mathbb {E} \left\{\frac {\operatorname{Val} \left(\pi \left(\theta_ {z}\right)\right)}{V ^ {*} \left(\theta_ {z}\right)} \right\} \geq \frac {1}{c},\tag{16.8}
$$

for some constant $c \geq 1$ . Such a mechanism is guaranteed to achieve within fraction $\frac { 1 } { c }$ of the value of the optimal offline algorithm, whatever the input sequence. The expectation allows for stochastic policies and can also allow for the use of randomization in defining the power of the adversary (we will see this in the next section). Competitive ratio c is referred to as an upper-bound on the online performance of the mechanism.

Now consider the following modification to Auction 1:

Auction 2. A bid from an agent is a claim about its type, $\hat { \theta } _ { i } = ( \hat { a } _ { i } , \hat { d } _ { i } , \hat { w } _ { i } )$ , necessarily made in period $t = \hat { a } _ { i }$

(i) In each period, t, allocate the item to the highest unassigned bid, breaking ties at random.

(ii) Every allocated agent pays its critical-value payment, collected upon its reported departure.

The auction is the same as Auction 1 except for the payment rule, which now charges the critical value rather than the second price in the period in which an agent wins. We refer to this as a “greedy auction” because the decision policy myopically maximizes value in each period. When every bidder is impatient, then the auction reduces to a sequence of Vickrey auctions (i.e., Auction 1.)

Example 16.16 Consider the earlier example, with three agents and types $\theta _ { 1 } =$ (1, 2, 100), $\theta _ { 2 } = ( 1 , 2 , 8 0 )$ , and $\theta _ { 3 } = ( 2 , 2 , 6 0 )$ , and one item to sell in each period. Suppose that all three agents bid truthfully. The greedy allocation rule sells to agent 1 in period 1 and then agent 2 in period 2. Agent 1’s payment is 60 because this is the critical value for arrival–departure (1, 2), given the bids of other agents. (A bid of just above 60 would allow the agent to win, albeit in period 2 instead of period 1.) Agent $2 \mathrm { { : } } \mathrm { { s } }$ payment is also 60.

Theorem 16.17 Auction 2 is strongly truthful and 2-competitive for efficiency in the expiring-items environment with no early-arrival and no late-departure misreports.

proof Suppose that random tie-breaking is invariant to reported arrival and departure. The auction is strongly truthful because the allocation function is monotone: if agent i wins in some period $t \in [ a _ { i } , d _ { i } ]$ then it continues to win either earlier or in the same period for $w _ { i } ^ { \prime } > w _ { i }$ , and for $a _ { i } ^ { \prime } < a _ { i }$ or $d _ { i } ^ { \prime } > d _ { i }$ . For competitiveness, consider a set of types $\theta$ and establish that the greedy online allocation rule is 2-competitive by a charging argument. For any agent i that is allocated offline but not online, charge its value to the online agent that was allocated in period t in which agent i is allocated offline. Since agent i is not allocated online, it is present in period t, and the greedy rule allocates to another agent in that period with at least as much value as agent i. For any agent i that is allocated offline and also online, charge its value to itself in the online solution. Each agent that is allocated in the online solution is charged at most twice, and in all cases for a value less than or equal to its own value. Therefore the optimal offline value $V ^ { * } ( \theta )$ is at most twice the value of the greedy solution.

We now understand that the decision policy in Auction 1 was monotonic but that Auction 1 was not truthful because the payments were not critical-value payments.

It is interesting to note that there is a 1.618-competitive online algorithm for this problem. However, this algorithm is not monotonic and cannot be implemented truthfully. In fact, we have a tight lower bound for the problem of achieving efficiency and truthfulness.

Theorem 16.18 No truthful, IR, and deterministic online auction can obtain a (2 − 	)-approximation for efficiency in the expiring items environment with no early-arrival and no late-departure misreports,for any constant $\epsilon > 0$

proof Fix $\epsilon > 0$ , consider $T = \{ 1 , 2 \}$ and construct the following three scenarios: (i) Consider agents $\theta _ { 1 } = ( 1 , 1 , q ( 1 + \delta ) ) , \theta _ { 2 } = ( 1 , 2 , q )$ , and choose $\textstyle 0 < \delta < { \frac { \epsilon } { 1 - \epsilon } }$ so that $\begin{array} { r } { \frac { q ( 1 + \delta ) } { q ( 2 + \delta ) } < \frac { 1 } { 2 - \epsilon } } \end{array}$ and the auction must allocate to both agents to be $( 2 - \epsilon ) { \mathrm { - c o m p e t i t i v e } }$ . Let $q \ge v _ { ( 1 , 1 ) } ^ { c } ( \theta _ { 2 } )$ (dropping dependence on $\omega$ because there are no stochastic events to consider), so that agent 1 must have strictly positive utility since the price is independent of reported value (for truth fulness) and less than or equal to $v _ { ( 1 , 1 ) } ^ { c } ( \theta _ { - 1 } )$ for IR. (ii) As in (i) except $\theta _ { 1 }  \theta _ { 1 } ^ { \prime } = ( 1 , 2 , q ( 1 + \delta ) )$ ) and a new type $\theta _ { 3 } = ( 2 , 2 , \infty )$ is introduced. Agent 1 must be allocated else it can report type $\theta _ { 1 }$ . Moreover, agent 1 must be allocated in period 1 because otherwise the mechanism cannot compete when $\theta _ { 3 }$ arrives. Agent 2 is not allocated. (iii) As in (i) except $\theta _ { 1 }  \theta _ { 1 } ^ { \prime } = ( 1 , 2 , q ( 1 + \delta ) )$ and $\theta _ { 2 }  \theta _ { 2 } ^ { \prime } = ( 1 , 1 , q )$ . The auction must allocate to both agents to be $( 2 - \epsilon ) \cdot$ competitive. Further assume that $q > v _ { ( 1 , 1 ) } ^ { c } ( \theta _ { 1 } ^ { \prime } )$ , which is without loss of generality because if $q = v _ { ( 1 , 1 ) } ^ { c } ( \theta _ { 1 } ^ { \prime } )$ then we can repeat the analysis with $q ^ { \prime } = \alpha q$ for $\alpha > 1$ replacing q throughout. But now agent 2 with type $\theta _ { 2 } ^ { \prime }$ has strictly positive utility since its payment is no greater than its critical value and the auction is not truthful in scenario (ii) because agent 2 can benefit by deviating and reporting $\theta _ { 2 } ^ { \prime } . \quad \sqcup$

The following provides a technical justification for why the no late-departure misreports assumption is required in this environment.

Theorem 16.19 No truthful, IR, and deterministic online auction can obtain a constant approximation ratiofor efficiency in the expiring items environment with no early-arrival misreports but arbitrary misreports ofdeparture.

proof Consider M periods. Fix $\theta _ { - i }$ . Fix $v _ { ( 1 , 1 ) } ^ { c } ( \theta _ { - i } ) < \infty$ (dropping dependence on ω because there are no stochastic events to consider). First show that any agent with type $\theta _ { i } = ( 1 , M , w _ { i } )$ for $w _ { i } > v _ { ( 1 , M ) } ^ { c } ( \theta _ { - i } )$ must be allocated in period 1. For this, first show that $v _ { ( 1 , M ) } ^ { c } ( \theta _ { - i } ) = v _ { ( 1 , 1 ) } ^ { c } ( \theta _ { - i } )$ . Construct $\theta _ { i } ^ { \prime } = ( 1 , M , w _ { i } ^ { \prime } )$ with $w _ { i } ^ { \prime } = v _ { ( 1 , 1 ) } ^ { c } + \epsilon$ , some $\epsilon > 0$ . By truthfulness and thus monotonicity we have $v _ { ( 1 , M ) } ^ { c } ( \theta _ { - i } ) \leq v _ { ( 1 , 1 ) } ^ { c } ( \theta _ { - i } )$ and agent i must be allocated. Moreover, it must be allocated in period 1 else an adversary can generate $M - 1$ bids $\{ ( t , t , \beta ^ { t - 1 } ) \}$ } for large $\beta > 0$ and $t \in \{ 2 , \ldots , M \}$ }, all of which must be accepted for the auction to be constant competitive. But in this case the agent should deviate and report $( 1 , 1 , w _ { i } ^ { \prime } )$ , and be allocated in period 1 with payment $v _ { ( 1 , 1 ) } ^ { c } ( \theta _ { - i } ) < w _ { i } ^ { \prime }$ and have positive utility. Since type $( 1 , M , w _ { i } ^ { \prime } )$ is allocated in period 1, we must have $v _ { ( 1 , M ) } ^ { c } ( \theta _ { - i } ) = v _ { ( 1 , 1 ) } ^ { c } ( \theta _ { - i } )$ by truthfulness and the critical-payment lemma else type $( 1 , 1 , w _ { i } ^ { \prime } )$ can deviate and report $( 1 , M , w _ { i } ^ { \prime } )$ and do better. Consider again type $( 1 , M , w _ { i } )$ , we now have $w _ { i } > v _ { ( 1 , M ) } ^ { c } ( \theta _ { - i } ) \Rightarrow w _ { i } > v _ { ( 1 , 1 ) } ^ { c } ( \theta _ { - i } )$ and the agent must be allocated in period 1. To finish the proof, now construct type profile $\theta = \{ ( 1 , M , q _ { 1 } ) , \dots , ( 1 , M , q _ { M } ) \}$ with $q _ { 1 } , \ldots , q _ { m }$ unique values drawn from $[ q , q + \delta ]$ for some $q > 0$ and $\delta > 0$ . For any i, we must have $v _ { ( 1 , 1 ) } ^ { c } ( \theta _ { - i } ) < \infty$ else the mechanism is not competitive because the adversary could replace type i with $\theta _ { i } ^ { \prime } = ( 1 , 1 , w _ { i } ^ { \prime \prime } )$ and some arbitrarily large $w _ { i } ^ { \prime \prime }$ . We can also assume $q _ { i } \geq v _ { ( 1 , M ) } ^ { c } ( \theta _ { - i } ) \Rightarrow q _ { i } > v _ { ( 1 , M ) } ^ { c } ( \theta _ { - i } )$ , which can be achieved by a slight upward perturbation of any value $q _ { i } = v _ { ( 1 , M ) } ^ { c } ( \theta _ { - i } )$ . Finally, the online mechanism can allocate at most one of these bids since any bid allocated must be allocated in period 1 and can achieve value at most $q + \delta$ while the efficient offline allocation has value $V ^ { * } ( \theta ) \geq M q$ . Thus, no constant approximation is possible because M can be selected to be arbitrarily large.

## 16.3.3 Example: An Adaptive, Limited-Supply Auction

For our second detailed example, we consider an environment with a single, indi visible item to be allocated to one of N agents. Each agent’s type is still denoted $\theta _ { i } = ( a _ { i } , d _ { i } , w _ { i } ) \in T \times T \times \mathbb { R } _ { > 0 }$ , with $w _ { i }$ denoting the agent’s value for the item. This fits into the known interesting-set model. We assume no early-arrival misreports but will allow arbitrary misreports of departure. Our goal is to define an auction with good revenue and efficiency properties. We will work with a weaker adversarial model than in the setting with expiring items.

We relate this dynamic auction problem to the classical secretary problem, a well studied problem in optimal stopping theory:

The Secretary Problem. An interviewer meets with each from a pool of N job appli cants in turn. The total number of applicants is known. Each applicant has a quality and the interviewer learns, upon meeting, the relative rank of each applicant among those already interviewed and must make an irrevocable decision about whether or not to hire the applicant. The goal is to hire the best applicant. By the “random-ordering hypothesis,” an adversary can choose an arbitrary set of N qualities but cannot control the assignment of quality to applicant, rather this is sampled uniformly at random and without replacement from the set. The problem is to design a stopping rule that maximizes the probability of hiring the highest rank applicant, in the worst case for all possible adversarially selected inputs. Say that a candidate is the most qualified of all applicants seen so far. The optimal policy (i.e., the policy that maximizes the probability of selecting the best applicant, in the worst case) is to interview the first $t - 1$ applicants and then hire the next candidate (if any), where t is defined by

$$
\sum_ {j = t + 1} ^ {N} \frac {1}{j - 1} \leq 1 <   \sum_ {j = t} ^ {N} \frac {1}{j - 1}.\tag{16.9}
$$

For instance, with $N = 1 0 { , } 0 0 0$ the optimal t is 3,680, i.e., sample 3,679 applicants and then accept the next candidate. As $N \to \infty$ , the probability of hiring the best applicant approaches $1 / e ,$ as does the ratio $t / N$ , and the optimal policy in this big N limit is to sample the first $\lfloor N / e \rfloor$ applicants and then immediately accept any subsequent candidate.

We can reinterpret the secretary problem in the auction context. Bidders, unlike the applicants in the classic model, are strategic and can misrepresent their value and time their entry into the market. Bidders also have both an entry and an exit time. We modify the adversarial model in the secretary problem while retaining the random-ordering hypothesis: an adversary picks a set of values and a set of arrival–departure intervals and agent types are then defined by sampling uniformly at random and without replacement from each set.<sup>5</sup>

In addition to efficiency, we will also consider revenue as an optimality criterion. The auction’s revenue for type profile θ is defined as $\begin{array} { r } { \mathrm { R e v } ( p ( \theta ) ) = \sum _ { i } p _ { i } ( \theta ) } \end{array}$ , where notation $p _ { i } ( \boldsymbol { \theta } )$ denotes the (expected) payment by agent i given type profile θ. Notation $\omega \in \Omega$ is suppressed because there are no external stochastic events in the problem. For an offline benchmark we consider the revenue from an offline Vickrey auction and define $R ^ { * } ( \theta )$ as the second-highest value in type profile θ. An online mechanism is c-competitivefor revenue if

$$
\min _ {z \in \mathcal {Z}} \mathbb {E} \left\{\frac {\operatorname{Rev} (p (\theta_ {z}))}{R ^ {*} (\theta_ {z})} \right\} \geq \frac {1}{c},\tag{16.10}
$$

where $z \in { \mathcal { Z } }$ is the set of inputs available to an adversary, in this case choosing the two sets described above, and the expectation here is taken with respect to the random choice of the sampling process that matches values with arrival–departure intervals.

As we have seen, the optimal policy for the secretary problem has a learning phase followed by an accepting phase. For a straw-man online auction interpreta tion, consider: observe the first $\lfloor N / e \rfloor$ reports and then price at the maximal value received so far, and sell to the first agent to subsequently report a value greater than this price. Break ties at random. The following example shows that this fails to be truthful.

Example 16.20 Consider six agents, with types $\theta _ { i } = ( a _ { i } , d _ { i } , w _ { i } )$ and $\theta _ { 1 } =$ $( 1 , 7 , 6 ) , \theta _ { 2 } = ( 3 , 7 , 2 ) , \theta _ { 3 } = ( 4 , 8 , 4 ) , \theta _ { 4 } = ( 6 , 7 , 8 )$ , and agents 5 and 6 arriving in later periods. The transition to the accepting phase occurs after $\lfloor 6 / e \rfloor = 2$ bids. Agent 4 wins in period 6 and makes payment 6. If agent 1 reports $\theta _ { 1 } ^ { \prime } = ( 5 , 7 , 6 )$ then it wins in period 5, for payment 4.

The auction is truthful when all agents are impatient $( a _ { i } = d _ { i } )$ but fails to be truthfu in the general setting with patient agents because the allocation policy is not monotonic with respect to arrival time. Consider instead the following simple variation.

Auction 3. A bid from an agent is a claim about its type, $\hat { \theta } _ { i } = ( \hat { a } _ { i } , \hat { d } _ { i } , \hat { w } _ { i } )$ ), necessarily made in period $t = \hat { a } _ { i }$

(i) (Learning): In period τ in which the $\lfloor N / e \rfloor t h$ bid is received let $p \geq q$ be the top two bid values received so far.

(ii) (Transition): If an agent bidding $p$ is still present in period $\tau$ then sell to that agent (breaking ties at random) at price $q$ .

(iii) (Accepting): Else, sell to the next agent to bid a price at least $p$ (breaking ties at random), collecting payment $p .$ .

Theorem 16.21 Auction 3 is strongly truthful in the single-unit, limited supply environment with no early-arrival misreports.

proof Assume that the method used to break ties is independent of the reported departure time of an agent. $\operatorname { F i x } \theta _ { - i }$ . Monotonicity is established by case analysis on type $\theta _ { i } \colon \left( \mathbf { a } \right)$ If $d _ { i }$ is to the left of the transition, the agent is not allocated and monotonicity trivially holds. (b) If $[ a _ { i } , d _ { i } ]$ spans the transition, agent i does not trigger the transition, and it wins with $w _ { i } > q$ then there is no tie-breaking and the agent continues to win for an earlier arrival or later departure (because this changes nothing about the price it faces when the transition occurs), and continues to win with a higher value. (c) If arrival, $a _ { i }$ , is after the transition and agent i wins with $w _ { i } > p$ (and perhaps winning a random selection over another agent $j$ arriving in the same period also with $w _ { j } > p )$ then it continues to win with an earlier arrival (even one that occurs before the transition because its value will define $p )$ , with a later departure (because tie-breaking is invariant to reported departure) and with a higher value. (d) If the agent triggers the transition and wins with $w _ { i } > q$ then its value $w _ { i } = p$ , there was no tie to break, and the agent continues to win for an earlier arrival (although at some point the transition will be triggered by the next earliest agent to arrive), for a higher value, and is unaffected by a later departure. The payment is the critical value, namely $q$ in case (b) and (d) and $p$ in case (c). Moreover, the policy is monotonic-late: in case (b) the critical value is infinite for all departures before the transition but constant with respect to departure otherwise and the critical departure period is that of the transition; in cases (c) and (d) the critical value payment is independent of departure time and the critical departure period is equal to the arrival period.

Example 16.22 Return to the earlier example with six agents and types $\theta _ { 1 } =$ $( 1 , 7 , 6 ) , \theta _ { 2 } = ( 3 , 7 , 2 ) , \theta _ { 3 } = ( 4 , 8 , 4 ) , \theta _ { 4 } = ( 6 , 7 , 8 )$ , with agents 5 and 6 arriving in later periods. The transition to the accepting phase occurs upon the arrival of agent 2. Then $p = 6 , q = 2$ , and agent 1 wins for 2. Consider instead that $\theta _ { 1 } ^ { \prime } = ( 1 , 2 , 6 )$ . The transition still occurs upon the arrival of agent 2 but now the item is sold in period 6 to agent 4 for a payment of6. An agent with true type $\theta _ { 1 } ^ { \prime }$ does not want to report $\theta _ { 1 }$ because of the monotonic-late property: although it would win, it would not be allocated until period 3, and this is after its true departure.

Theorem 16.23 Auction 3 is $e + o ( 1 )$ )-competitivefor efficiency and $e ^ { 2 } + o ( 1 ) \quad$ competitivefor revenue in the single-unit, limited supply environment in the limit as $N \to \infty$

proof Let $\tau = \lfloor N / e \rfloor$ . For efficiency, our competitive ratio is at least as great as the probability of selling to the highest value agent. Conditioned on selling at the transition, the probability that we sell to the highest value agent is at least $\begin{array} { r } { \frac { \lfloor N / e \rfloor } { N } = 1 / e - o ( 1 ) } \end{array}$ . Conditioned on selling after the transition, the probability of this event is $1 / e - o ( 1 )$ according to the analysis ofthe classical secretary problem. For revenue, our competitive ratio is at least as great as the probability of selling to the highest value agent at a price equal to the second-highest bid. Conditioned on selling at the transition, the probability of this event is $( 1 / e ) ^ { 2 } - o ( 1 )$ ) (i.e., the probability that both the highest and second-highest value agents arrive before period τ). Conditioned on selling after the transition, the probability of this event is $( 1 / e ) ( 1 - 1 / e ) - o ( 1 )$ , i.e., the probability that the second-highest value agent arrives before τ and the highest value agent arrives after τ. The unconditional probability of selling to the highest value agent at the second-highest price is a weighted average of the two conditional probabilities computed above, hence it is at least $( 1 / e ) ^ { 2 } - o ( 1 )$ □

The random-ordering hypothesis has a critical role in this analysis: there is no constant competitive mechanism in this environment for the adversarial model adopted in our analysis of the expiring items environment.

For the secretary problem it is well known that no stopping rule can achieve asymptotic success probability better than $1 / e$ . The same lower bound can be established in our setting, even though the mechanism has richer feedback (i.e., it sees numbers not ranks) and even though an allocation to some bidder other than the highest-rank bidder will contribute to expected efficiency. The proof of this result is beyond the scope of this chapter.<sup>6</sup>

## 16.3.4 Remarks

We end this section with some general remarks that mostly seek to place the study of online mechanisms in single-valued preference domains in the broader context of computational mechanism design.

Ex-post IC. A mechanism is ex-post IC if truth revelation is a best-response contingent on other agents being truthful, and whatever the types of other agents (and thus for all possible futures in the context of online MD). In offline mechanisms the solution concepts of ex-post incentive compatible (EPIC) and DSIC are equivalent with private value types. This equivalence continues to hold for closed online mecha nisms that provide no feedback to an agent before it submits a bid. However, an online mechanism that provides feedback, for instance prices, or in an extreme case reports of current bids, loses this property. The report of an agent can now be conditioned on the reports of earlier agents, and monotonicity provides EPIC but not necessarily DSIC. Consider again Auction 2 in the expiring items environment, with true types $\theta _ { 1 } = ( 1 , 2 , 1 0 0 ) , \theta _ { 2 } = ( 1 , 2 , 8 0 )$ , and $\theta _ { 3 } = ( 2 , 2 , 6 0 )$ . If the bids are public then a possible (crazy) strategy of agent 3 is to condition its bid as possible: “bid (2, 2, 1000) if a bid of (1, 2, 100) is received or bid (2, 2, 60) otherwise.” Agent 1 will now pay 60 if it bids truthfully, but would pay 60 with a bid of (1, 2, 90). Nevertheless, truthful bidding is a best response when other agents bid truthfully.

Simple price-based online auctions. One straightforward method to construct truthful online auctions for known-set, single-valued environments is to define an agent-independent price schedule $q _ { i } ^ { t } ( L , \theta _ { - i } , \omega ) \in \mathbb { R }$ to agent i for interesting decision set $L \in \mathcal { L } _ { i }$ , given stochastic events $\omega \in \Omega$ , where $q _ { i } ^ { t } ( L , \theta _ { - i } , \omega )$ defines the price for a decision in set L in period t. Given this, define payment $\begin{array} { r } { p _ { ( a _ { i } , d _ { i } , L _ { i } ) } ( \theta _ { - i } , \omega ) = \operatorname* { m i n } _ { t \in [ a _ { i } , d _ { i } ] } q _ { i } ^ { t } ( L _ { i } , \theta _ { - i } , \omega ) } \end{array}$ and let $t _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { * } ( \theta _ { - i } , \omega )$ denote the first period $t \in [ a _ { i } , d _ { i } ]$ in which $q _ { i } ^ { t } ( L _ { i } , \theta _ { - i } , \omega ) = p _ { ( a _ { i } , d _ { i } , L _ { i } ) } ( \theta _ { - i } , \omega )$ . Then, decision policy π that allocates to agent i with type $\theta _ { i } = ( a _ { i } , d _ { i } , ( r _ { i } , L _ { i } ) )$ ) if and only if $r _ { i } \ge q _ { i } ^ { t } ( L _ { i } , \theta _ { - i } , \omega )$ in some $t \in [ a _ { i } , d _ { i } ]$ , with the allocation period $t \geq t _ { ( a _ { i } , d _ { i } , L _ { i } ) } ^ { * } ( \theta _ { - i } , \omega )$ , is monotonic-late and the associated critical-value payment is just $p _ { ( a _ { i } , d _ { i } , L _ { i } ) } ( \theta _ { - i } , \omega )$ . Working with price schedules is quite natural in many domains, although not completely general, as shown in the following example:

Example 16.24 Consider the canonical expiring items environment. Fix $\theta _ { - i }$ , and consider a monotonic-late policy π with critical-value $v _ { ( 1 , 2 ) } ^ { c } ( \theta _ { - i } ) =$ $2 0 , v _ { ( 1 , 1 ) } ^ { c } ( \theta _ { - i } ) = v _ { ( 2 , 2 ) } ^ { c } ( \theta _ { - i } ) = 3 0$ (dropping dependence on $\omega$ because there are no stochastic events to consider). This policy allocates to type $\theta _ { i } = ( 1 , 2 , 2 5 )$ in period 2 but not type $\theta _ { i } ^ { \prime } = ( 1 , 1 , 2 8 )$ ) or $\theta _ { i } ^ { \prime } ( 2 , 2 , 2 8 )$ . No simple price schedule corresponds to this policy, because it would require $q _ { i } ^ { 1 } ( \theta _ { - i } ) > 2 8 , q _ { i } ^ { 2 } ( \theta _ { - i } ) > 2 8$ but min $( q _ { i } ^ { 1 } ( \theta _ { - i } ) , q _ { i } ^ { 2 } ( \theta _ { - i } ) ) \leq 2 5$

The role of limited misreports. Consider again the above example. The price on an allocation to agent i in period 2 depends on its report: if the agent’s type is $\theta _ { i } = ( 2 , 2 , w _ { i } )$ then the price is 30 but if the agent’s type is $\theta _ { i } = ( 1 , 2 , w _ { i } )$ then the price is 20. This is at odds with the principle of “agent-independent prices” that drives the standard analysis of truthful mechanisms. The example also fails weak-monotonicity, which is generally necessary for truthfulness.<sup>7</sup>

What is going on? In both cases, the reason for this departure from the standard theory for truthful mechanism design is the existence of limited misreports. The auction would not be truthful with early-arrival misreports because an agent with type (2, 2, 28) could usefully deviate and report (1, 2, 28). For limited misreports $C ( \theta _ { i } ) \subseteq \Theta _ { i }$ that satisfy transitivity (which holds for the no-early arrival and no-late departure assumptions that are motivated in online MD), so that $\theta _ { i } ^ { \prime } \in C ( \theta _ { i } )$ and $\theta _ { i } ^ { \prime \prime } \in C ( \theta _ { i } ^ { \prime } )$ ) implies $\theta _ { i } ^ { \prime \prime } \in C ( \theta _ { i } )$ , the payment $p _ { i } ( k , \theta _ { i } , \theta _ { - i } , \omega )$ ) collected from agent i conditioned on outcome $k \in \mathcal { O }$ , must satisfy $p _ { i } ( k , \theta _ { i } , \theta _ { - i } , \omega ) = \operatorname* { m i n } \{ p _ { i } ( k , \hat { \theta } _ { i } , \theta _ { - i } , \omega ) : \hat { \theta } _ { i } \in C ( \theta _ { i } ) , \pi ( \hat { \theta } _ { i } , \theta _ { - i } , \omega ) = k \}$ , or ∞ if no such $\widehat { \theta } _ { i }$ exists, for all i, all $k \in \mathcal { O }$ and all $\omega \in \Omega$ . Limited dependence on the reported type is possible as long as the price is independent across available misreports. For unlimited misreports we recover the standard requirement that prices are agentindependent.

So, the temporal aspect of online MD is both a blessing and a curse: on one hand we can justify limited misreports and gain more flexibility in pricing and in the timing of allocations, on the other hand decisions must be made in ignorance about future types.

Relaxing the known interesting-set assumption. We assumed that the interesting set $L _ { i } \in { \mathcal { L } } _ { i }$ was known by the mechanism. Domains in which the interesting set is private information to an agent can be handled by making the following modifications to the framework:

(i) Require that agent $i \ ' _ { \mathrm { { S } } }$ domain of interesting sets $\mathcal { L } _ { i } = \{ L _ { 1 } , \ldots , L _ { m } \}$ , defines disjoint sets so that $L _ { 1 } \cap L _ { 2 } = \emptyset$ for all $L _ { 1 } , L _ { 2 } \in \mathcal { L } _ { i }$

(ii) Require that a decision policy π is minimal so that it never makes decision $k ^ { t } \in L$ for some $L \succ _ { L } L _ { i }$ in some period $t \in [ a _ { i } , d _ { i } ]$ , given reported type $\theta _ { i } = ( a _ { i } , d _ { i } , ( r _ { i } , L _ { i } ) )$ (iii) Extend the partial-order so that

$$
\theta_ {1} \preceq_ {\theta} \theta_ {2} \equiv (a _ {1} \geq a _ {2}) \wedge (d _ {1} \leq d _ {2}) \wedge (r _ {1} \leq r _ {2}) \wedge (L _ {1} \succeq_ {L} L _ {2}),\tag{16.11}
$$

and adopt this partial order in defining monotonicity.

Given these modifications, the general methods developed above for the analysis of online mechanisms continue to hold. For instance, a monotonic, minimal, and deterministic policy continues to be truthful when combined with critical-value payments, and monotonicity remains necessary for truthfulness amongst minimal, deterministic policies. This is left as an exercise.

The requirement that interesting sets are disjoint can significantly curtail the generality of preference domains that can be modeled. It is especially hard to model substitutes preferences, for instance indifference across a set of items. Suppose that the items are fruit, with $G = \{ a p p l e$ , banana, pear, lime, lemon}. With known interesting sets, we can model an agent with a type that defines a value for receiving an item from any subset of the domain G. With unknown interesting sets, we must now assume that there is some partition, for instance into {{apple, pear}, {banana}, {lime, lemon}} so that the agent has either the same value for an apple or a pear and no value for anything else, or value for a banana and no value for anything else, or value for a lime and a lemon but no value for anything else.

Stochastic policies. Stochastic decision policies can be important, both algorithmically (many computational methods for online decision use a probabilistic model to sample possible state trajectories) and also to allow for tie breaking while retaining anonymity.

So far we have handled this by requiring strong-truthfulness. More generally, a stochastic mechanism is DSIC when truthful reporting maximizes expected utility fo an agent (with the expectation defined with respect to randomization in the policy), and for all reports of other agents, and all external stochastic events, $\omega \in \Omega$ . To handle this, we now $\pi _ { i } ( \theta _ { i } , \theta _ { - i } , \omega ) \in [ 0 ,$ 1] to denote the probability that agent i receives an interesting decision (“is allocated”), given type $\theta _ { i }$ , types $\theta _ { - i }$ and (external) stochastic events ω. The appropriate generalization ofmonotonicity to stochastic policies requires, for every $\theta _ { i } = ( a _ { i } , d _ { i } , ( r _ { i } , L _ { i } ) )$ , all $\theta _ { - i }$ , all $\omega \in \Omega$ , that

$$
\pi_ {i} ((a _ {i}, d _ {i}, (r _ {i}, L _ {i})), \theta_ {- i}, \omega) \geq \pi_ {i} ((a _ {i}, d _ {i}, (r _ {i} ^ {\prime}, L _ {i})), \theta_ {- i}, \omega), \quad \forall r _ {i} \geq r _ {i} ^ {\prime},\tag{16.12}
$$

and

$$
\int_ {x = 0} ^ {r _ {i}} \pi_ {i} ((a _ {i}, d _ {i}, (x, L _ {i})), \theta_ {- i}, \omega) \mathrm{d} x \geq \int_ {x = 0} ^ {r _ {i}} \pi_ {i} ((a _ {i} ^ {\prime}, d _ {i} ^ {\prime}, (x, L _ {i})), \theta_ {- i}, \omega) \mathrm{d} x,\tag{16.13}
$$

for all $a _ { i } ^ { \prime } \geq a _ { i } , d _ { i } ^ { \prime } \leq d _ { i }$ . The critical value payment becomes

$$
v _ {(a _ {i}, d _ {i}, (r _ {i}, L _ {i}))} ^ {c} (\theta_ {- i}, \omega) = \pi_ {i} (\theta , \omega) r _ {i} - \int_ {x = 0} ^ {r _ {i}} \pi_ {i} ((a _ {i}, d _ {i}, (x, L _ {i})), \theta_ {- i}, \omega) \mathrm{d} x\tag{16.14}
$$

These definitions of monotonicity and critical-value payment reduce to the earlier cases when the policy is deterministic.

Theorem 16.25 A stochastic decisionpolicy π can be implemented in a truthful, IR mechanism that does not pay unallocated agents in a domain with (known interesting set) single-valued preferences and no early-arrival or late-departure misreports ifand only ifthe policy is monotonic according to (16.12) and (16.13).

The payment collected from allocated agents is the critical-value payment. The following example illustrates a stochastic policy that satisfies this monotonicity requirement.

Example 16.26 Consider a domain with no early arrival and no late departure misreports, two time periods $T = \{ 1 , 2 \}$ , fix $\theta _ { - i }$ , and consider agent i with a single-item valuation and possible types $\Theta _ { i } = \{ ( 1 , 1 , w _ { i } ) , ( 1 , 2 , w _ { i } ) , ( 2 , 2 , w _ { i } ) \}$ For impatient type $( 1 , 1 , w _ { i } )$ , consider policy

$$
\pi_ {i} ((1, 1, w _ {i}), \theta_ {- i}) = \left\{ \begin{array}{l l} 0, & \text { if } w _ {i} \leq 8 \\ \frac {w _ {i} - 8}{2}, & \text { if } 8 <   w _ {i} \leq 1 0 \\ 1, & \text { otherwise. } \end{array} \right.\tag{16.15}
$$

Solving for the critical value payment (16.14), we find

$$
v _ {(1, 1, w _ {i})} ^ {c} (\theta_ {- i}) = \left\{ \begin{array}{l l} 0, & \text { if } w _ {i} \leq 8 \\ \frac {w _ {i} ^ {2}}{4} - 1 6, & \text { if } 8 <   w _ {i} \leq 1 0 \\ 9, & \text { otherwise. } \end{array} \right.\tag{16.16}
$$

The policy and critical value payment is defined identically for type $( 2 , 2 , w _ { i } )$ For patient type (1, 2, $w _ { i } )$ , consider policy

$$
\pi_ {i} ((1, 2, w _ {i}), \theta_ {- i}) = \left\{ \begin{array}{l l} \frac {w _ {i}}{2 0}, & \text { if } 0 \leq w _ {i} \leq 1 0 \\ \frac {w _ {i} - 5}{1 0}, & \text { if } 1 0 <   w _ {i} \leq 1 5 \\ 1, & \text { otherwise } \end{array} \right.\tag{16.17}
$$

and the critical value payment, from (16.14), is

$$
v _ {(2, 2, w _ {i})} ^ {c} (\theta_ {- i}) = \left\{ \begin{array}{l l} \frac {w _ {i} ^ {2}}{4 0}, & \text { if } 0 \leq w _ {i} \leq 1 0 \\ \frac {w _ {i} ^ {2}}{2 0} - \frac {5}{2}, & \text { if } 1 0 <   w _ {i} \leq 1 5 \\ 8. 7 5, & \text { otherwise }. \end{array} \right.\tag{16.18}
$$

Notice that $\pi _ { i } ( ( 1 , 1 , 1 0 ) , \theta _ { - i } ) = 1 \mathrm { a n d } \pi _ { i } ( ( 1 , 2 , 1 0 ) ) = 0 . 5 ,$ , contradicting more simplistic notions of monotonicity, but that truthfulness is retained because $v _ { ( 1 , 1 , 1 0 ) } ^ { c } ( \theta _ { - i } ) = 9$ while $v _ { ( 1 , 2 , 1 0 ) } ^ { c } ( \theta _ { - i } ) = 2 . 5$ . Although type (1, 2, 10) can misre port to (1, 1, 10) and be allocated with certainty, it prefers to report (1, 2, 10) because its expected utility is $( 0 . 5 ) ( 1 0 - 2 . 5 ) + ( 0 . 5 ) ( 0 ) > ( 1 . 0 ) ( 1 0 - 9 )$ ). We leave as an exercise to check that these policies satisfy monotonicity, with $\begin{array} { r } { \int _ { x = 0 } ^ { w _ { i } } \pi _ { i } ( ( 1 , 2 , x ) , \theta _ { - i } ) \mathrm { d } x \ge \int _ { x = 0 } ^ { w _ { i } } \pi _ { i } ( ( 1 , 1 , x ) , \theta _ { - i } ) } \end{array}$ for all $w _ { i }$

We make a final remark about stochastic policies. In an environment with a probabilistic model that is common knowledge, and that defines both a probability distribution for agent types and for stochastic events $\omega \in \Omega$ , we can settle for a weaker monotonicity requirement in which (16.12) and (16.13) are satisfied in expectation, given the model. However, this provides BNIC but not DSIC since monotonicity may not hold out of equilibrium when other agents are not truthful, since the probabilistic model of agent types upon which monotonicity is predicated would then be incorrect.

## 16.4 Bayesian Implementation in Online Domains

In this section we focus on Bayesian implementation of expected value-maximizing policies in environments in which the designer and every agent has a correct, probabilistic model for types and uncertain events, and this is common knowledge. We consider the goal of value maximization and present a dynamic variation of the offline Vickrey–Clarke–Groves (VCG) mechanism. This will involve computing expected value maximizing sequential decision policies and raise a number of computational challenges. We will see that the dynamic VCG mechanism is BNIC rather than DSIC, with incentive-compatibility contingent on future on-equilibrium play by all participants.

## 16.4.1 A General Model

A Markov decision process (MDP) provides a useful formalism for defining online mechanisms in model-based environments with general agent preferences. An MDP model $( H , K , \mathcal { P } , R )$ is defined for a set of states $H$ , feasible decisions $K ( h )$ in each state, a probabilistic transition function $\mathcal { P } ( h ^ { t + 1 } | h ^ { t } , k ^ { t } )$ on the next state given current state and decision (with $\begin{array} { r } { \sum _ { h ^ { \prime } \in H ^ { t + 1 } } \mathcal { P } ( h ^ { \prime } | h ^ { t } , k ^ { t } ) = 1 ) } \end{array}$ and a reward function $R ( h ^ { t } , k ^ { t } ) \in \mathbb { R }$ for decision $k ^ { t }$ in state $h ^ { t }$ . The Markov property requires that feasible decisions, transitions, and rewards depend on previous states and actions only through the current state. It is achieved here, for example, by defining $h ^ { t } \in H ^ { t } =$ $( \theta ^ { 1 } , \dots , \theta ^ { t } ; \omega ^ { 1 } , \dots , \omega ^ { t } ; k ^ { 1 } , \dots , k ^ { t - 1 } )$ so that the state captures the complete history of types, stochastic events, and decisions. In practice, a short summarization of state $h ^ { t }$ is often sufficient to retain the Markov property.

Given a social planner interested in maximizing total value, then define reward $\begin{array} { r } { R ( h ^ { t } , k ^ { t } ) = \sum _ { i \in I ( h ^ { t } ) } R _ { i } ( h ^ { t } , k ^ { t } ) } \end{array}$ , with $I ( h ^ { t } )$ used to denote the set of agents present in state $h ^ { t }$ and agent $i \ ' _ { \mathbf { S } }$ reward $R _ { i } ( h ^ { t } , k ^ { t } )$ ) is defined so that $\begin{array} { r } { v _ { i } ( \theta _ { i } , k ) = \sum _ { t = a _ { i } } ^ { d _ { i } } R _ { i } ( h ^ { t } , k ^ { t } ) } \end{array}$ for all sequences of decisions k. For finite time horizons, the expected value of policy $\pi$ in state $h ^ { t }$ is $\begin{array} { r } { V ^ { \pi } ( h ^ { t } ) = \mathbb { E } _ { \pi } \{ \sum _ { \tau = t } ^ { | T | } R ( h ^ { \tau } , \pi ^ { \tau } ( h ^ { \tau } ) ) \} } \end{array}$ , where the expectation is taken with respect to the transition model and given the state-dependent decisions implied by policy $\pi .$ For infinite time horizons, a standard approach is to define a discountfactor $\gamma \in ( 0 , 1 )$ so that the expected discounted value of policy $\pi$ in state $h ^ { t }$ is $V ^ { \pi } ( h ^ { t } ) =$ $\begin{array} { r } { \mathbb { E } _ { \pi } \{ \sum _ { \tau = t } ^ { \infty } \gamma ^ { \tau - t } R ( h ^ { \tau } , \pi ^ { \tau } ( h ^ { \tau } ) ) \} } \end{array}$ . This makes sense in a multiagent environment when every agent has the same discount factor $\gamma$ .

Given MDP value, $V ^ { \pi } ( h ^ { t } )$ , then the optimal policy $\pi ^ { * }$ maximizes this value, $V ^ { \pi } ( h ^ { t } )$ in every state $h ^ { t }$ . For instance, in the finite time-horizon (no discounting) setting, the optimal MDP-valuefunction, $V ^ { * }$ , is defined to satisfy recurrence:

$$
V ^ {*} (h) = \max _ {k \in K ^ {t} (h)} \left[ R (h, k) + \sum_ {h ^ {\prime} \in H ^ {t + 1}} \mathcal {P} \left(h ^ {\prime} \mid h, k\right) V ^ {*} \left(h ^ {\prime}\right) \right],\tag{16.19}
$$

for all time t and all $h \in H ^ { t }$ . Given this, the optimal decision policy solves:

$$
\pi^ {*} (h \in H ^ {t}) \in \arg \max _ {k \in K ^ {t} (h)} \left[ R (h, k) + \sum_ {h ^ {\prime} \in H ^ {t + 1}} \mathcal {P} (h ^ {\prime} | h, k) V ^ {*} (h ^ {\prime}) \right].\tag{16.20}
$$

Of course, the type information within the state is private to agents and we will need to provide incentive compatibility so that the policy has the correct view of the current state.

Example 16.27 The definition of state, feasible decision, and agent type is as in Example 16.3. The transition function $\mathcal { P } ( h ^ { t + 1 } | h ^ { t } , k ^ { t } )$ is constructed to reflect a probabilistic model of new agent arrivals, and also the allocation decision. The MDP reward function, $R ( h ^ { t } , k ^ { t } )$ , can be defined with $R ( h ^ { t } , k ^ { t } ) = w _ { i }$ if decision $k ^ { t }$ allocates the item to agent i, for some agent i present in the state, and zero otherwise.

## 16.4.2 A Dynamic Vickrey–Clarke–Groves Mechanism

For concreteness, consider an environment with a finite time horizon and no discounting, and with the optimal MDP value $V ^ { * } ( h )$ defined as the total expected reward from state h until the time horizon. We make some remarks about how to handle an infinite time horizon in Section 16.4.3. Consider the following dynamic VCG mechanism.<sup>8</sup> We assume that the decisions and reports in previous periods $t ^ { \prime } < t$ are all public in period t, although similar analysis holds without this.

Auction 4. The dynamic VCG mechanism for the finite time horizon and nodiscounting online MD environment works as follows:

(i) Each agent, i, reports a type $\widehat { \theta } _ { i }$ in some period $\hat { a } _ { i } \geq a _ { i }$

(ii) Decision policy: Implement optimal policy $\pi ^ { * }$ , which maximizes the total expected value, assuming the current state as defined by agent reports is the true state.

(iii) Payment policy: In an agent’s reported departure period, $t = \hat { d } _ { i }$ , collect payment

$$
x _ {i} ^ {t} (h ^ {t}) = v _ {i} (\hat {\theta} _ {i}, \pi^ {*} (\theta^ {\leq t}, \omega^ {\leq t})) - \left[ V ^ {*} (h ^ {\hat {a} _ {i}}) - V ^ {*} (h _ {- i} ^ {\hat {a} _ {i}}) \right],\tag{16.21}
$$

where $\pi ^ { * } ( \theta ^ { \leq t } , \omega ^ { \leq t } )$ denotes the sequence of decisions made up to and including period t based on types $\theta ^ { \leq t }$ and stochastic events $\omega ^ { \leq t } , V ^ { \ast } ( h ^ { t } )$ is the optimal MDP value in state $h ^ { t }$ , and $h _ { - i } ^ { t }$ defines the (counterfactual) MDP state constructed to be equal to $h ^ { t }$ but removing agent $i \ ' _ { \mathbf { S } }$ type from the state. The payment is zero otherwise.

Agent $i \ ' _ { \mathbf { S } }$ payment is its ex-post value discounted by term $( V ^ { * } ( h ^ { \hat { a } _ { i } } ) - V ^ { * } ( h _ { - i } ^ { \hat { a } _ { i } } ) )$ which is the expected marginal value it contributes to the system as estimated upon its arrival and based on its report. With this, the expected utility to agent i when reporting truthfully is equal to the expected marginal value that it contributes to the multiagent system through its presence.

For incentive-compatibility, we need the technical property of stalling, which requires that the expected value of policy $\pi ^ { * }$ cannot be improved (in expectation) by delaying the report of an agent.<sup>9</sup> In addition, we assume an independence property; namely, the probabilistic process defining the arrival of agents other than i is independent of whether or not agent i has arrived.

Theorem 16.28 The dynamic VCG mechanism, coupled with a policy that sat isfies stalling, is Bayes–Nash incentive compatible (BNIC) and implements the expected-value maximizing policy, in a domain with no early-arrival misreports but arbitrary misreports ofdeparture.

proof Consider the expected utility (defined with respect to its information in period $a _ { i } )$ to agent i for misreport $\hat { \theta } _ { i } \in C ( \theta _ { i } )$ . Let $c \geq 0$ denote the number of periods by which agent i misreports its arrival time. The agent’s expected utilit is

$$
\mathbb {E} _ {\pi^ {*}} \{v _ {i} (\theta_ {i}, \pi^ {*} (h ^ {a _ {i}})) | \hat {\theta} _ {i} \} + \mathbb {E} _ {\pi^ {*}} \left\{\sum_ {t = a _ {i} + c} ^ {| T |} R _ {- i} (h ^ {t}, \pi^ {*} (h ^ {t})) \right\} - \mathbb {E} _ {\pi^ {*}} \bigl \{V ^ {*} (h _ {- i} ^ {a _ {i} + c}) \bigr \}. \tag {A}
$$

Term (A) denotes the expected value to agent i given its misreport. Term (B), which denotes the total expected value to other agents forward from reported arrival, $a _ { i } + c .$ , given agent $i \ ' _ { \mathrm { { s } } }$ misreport, corresponds to the expected value of terms $\{ - v _ { i } ( \hat { \theta } _ { i } , \pi ^ { * } ( \theta ^ { \leq \hat { d } _ { i } } , \omega ^ { \leq \hat { d } _ { i } } ) ) + V ^ { * } ( { h } ^ { \hat { a } _ { i } } ) \}$ in the payment. Notation $R _ { - i }$ denotes the total reward that accrues due to all agents except agent i. Term (C), which denotes the total expected value to other agents forward from period $a _ { i } + c$ , but with agent i removed, corresponds to the final term in the payment. Now, add term $\begin{array} { r } { \mathbb { E } _ { \pi ^ { * } } \{ \sum _ { t = a _ { i } } ^ { a _ { i } + c - 1 } R _ { - i } ( h ^ { t } , \pi ^ { * } ( h ^ { t } ) ) \} } \end{array}$ to term (B) and subtract it again from term (C). The adjusted term $\mathrm { ( C ^ { \prime } ) }$ is now agent independent (by the independence property) and can be ignored for the purpose of establishing BNIC. Term (A) combined with adjusted term $( \mathbf { B ^ { \prime } } )$ is the expected value to all other agents forward from period $a _ { i }$ , plus the expected true value to agent i. Agent i’s best response is to report its true type (and immediately upon arrival) because the policy $\pi ^ { * }$ is defined to maximize $( \mathbf { A } ) { + } ( \mathbf { B } ^ { \prime } )$ when the other agents are truthful, i.e. in a Bayes–Nash equilibrium.

It bears repeating that truth telling is not a dominant strategy equilibrium. We have instead BNIC because the correctness of the policy depends on the center having the correct model for the distribution on agent types. Without the correct model, the policy is not optimal in expectation and an agent with beliefs different from that of the center may be able to improve (its belief about) the expected utility it will receive by misreporting its type and thus misrepresenting the state.<sup>10</sup>

## 16.4.3 Remarks

We end this section with some general remarks that touch on the computational aspects of planning in model-based environments, and also describe a couple of additional environments in which dynamic VCG mechanisms can be usefully applied.

Computational notes. Many algorithms exist to compute optimal decision policies in MDPs. These include dynamic programming, value iteration, policy iteration, and LP-based methods. However, the state space and action space for real-world online MD problems are large and approximations will typically be required. One appealing method is to couple the VCG mechanism with an online, sampling-based approximation algorithm. Rather than compute apriori an entire policy for every possible state one can determine the next decision to make in state $h ^ { t }$ by approximating the decision problem forward from that state. Given an 	-approximation, the dynamic VCG mechanism is -BNIC, in the sense that no agent can gain more than some amount $\epsilon > 0$ (that can be made arbitrarily small) by deviating from truthful reporting, as long as the other agents are truthful and an 	-accurate estimate of the optimal MDP value is also available. One class of online, sparse-sampling algorithms work by building out a sample tree of future states based on decisions that could be made by the policy forward to some look-ahead horizon. These algorithms have run time that is independent of the size of the state space but scales exponentially in the number of decisions and in the look-ahead horizon. More recently, a family of stochastic online combinatorial optimization algorithms has been proposed that seem especially applicable to online MD environments. The algorithms solve a subclass of MDPs in which the realization of uncertainty is independent of any decision. This is often a natural assumption for truthful dynamic auctions: the allocation decisions made by an IC auction will not affect the reports of agents, and thus the realization of new types is independent of decisions.

Infinite time horizon and discounting. The dynamic VCG mechanism can be extended to handle an infinite time horizon when every agent has a common discount factor. Rather than collect a payment once, upon departure, a payment can be collected from agent i in each period, so as to align its utility stream with the expected, marginal stream of value that it contributes through its presence in the multiagent system.

Coordinated learning. A variant on the dynamic VCG mechanism can be used to support optimal, coordinated learning among a fixed population of self-interested agents. Suppose that in addition to influencing the reward received by an agent in each time period, the decisions made by a mechanism also reveal information that an agent can use to update its belief about its type; i.e., types are revealed online. A simple model is provided by a multiagent variation on the classical multi-armed bandits problem. Each agent owns an “arm” and receives a reward when its arm is activated, sampled from a stationary distribution. The reward signals are privately observed and allow an agent to update its model for the reward on its arm. In a setting with an infinite time horizon and discounting, one can use Gittins’ celebrated index policy to characterize an efficient online policy that makes the optimal trade-off between exploitation and exploration. In the presence of self-interest, a variant on the dynamic VCG mechanism can provide incentives to support truthful reporting of reward signals by each agent, and thus implement the efficient learning policy.

## 16.5 Conclusions

We briefly consider some of the many possible future research directions in the area of online mechanism design:

 Revenue: Little work exists on the design of revenue-maximizing online mechanism in model-based environments. For example, the problem of designing an analog to Myerson’s optimal auction is only partially solved, even in the very simplest of online settings.

 Learning by the center: It is interesting to allow the mechanism to improve its probabilistic model of the distribution on agent types across time, while retaining incentive compatibility along the path of learning, and seek to converge to an efficient or revenueoptimal mechanism.

 Alternative solution concepts: Introduce weaker solution concepts than DSIC that avoid the strong common knowledge assumptions that are required to justify BNIC analysis. These could include, for instance, set Nash equilibria, implementation in undominated strategies, or implementation in minimax-regret equilibria and other robust solution concepts.

 Endogenous information: Extend online MD to domains in which decisions made by the mechanism affect the information available to agents about their types; i.e., cast online MD as a general problem of coordinated learning by self-interested agents in an uncertain environment.

 Richer domains: The current work on dominant-strategy implementation is limited to single-valued preference domains with quasi-linear utilities. Simple generalizations, such as to an environment in which some agents want an apple, some a banana, and some are indifferent across an apple and a banana do not satisfy the partition requirement on the structure of interesting sets and remain unsolved. Similar complications occur when one incorporates budget constraints, or generalizes to interdependent valuations. With time, perhaps progress can be made on the problem of online combinatorial auctions (and exchanges) in their full generality.

## 16.6 Notes

Lavi and Nisan (2000) coined the term online auction and initiated the study of truthfu mechanisms in dynamic environments within the computer science literature. Friedman and Parkes (2003) later coined the term online mechanism design. The characterization of monotonicity requirements for truthful online mechanisms in single-valued domains is based on Hajiaghayi et al. (2005), with extensions to single-valued preferences building on Babaioff et al. (2006), see also Chapter 12.<sup>11</sup> Weak-monotonicity and its role in truthful mechanism design are discussed in Bikhchandani et al. (2006).

The discussion of the secretary problem and adaptive truthful auctions in the singleitem setting is based on Hajiaghayi et al. (2004); see Babaioff et al. (2007) for a recent extension and (Gilbert and Mosteller, 1966; Dynkin, 1963) for classic references. The discussion of online mechanisms for expiring items is based on Hajiaghayi et al. (2005), and the negative result is due to Lavi and Nisan (2005), who also adopted an alternate solution concept in their analysis; see also (Ng et al., 2003; Porter, 2004; Juda and Parkes, 2006) and Awerbuch et al. (2003). Additional models of dynamic auctions in the computer science literature include unlimited supply, digital goods (Bar-Yossef et al., 2002; Blum et al., 2003; Blum and Hartline, 2005), two-sided auctions with both buyers and sellers (Bredin and Parkes, 2005; Blum et al., 2006), and interdependent value environments (Constantin et al., 2007). For an extended treatment of the single valued setting, see Parkes and Duong (2007).

Moving to the model-based framework, the discussion of the dynamic VCG mechanism is based on Parkes and Singh (Parkes and Singh, 2003; Parkes et al., 2004). A general presentation in given in Bergemann and Valim¨ aki (2006b), whose work along¨ with that of Cavallo et al. (2006) and Bapna and Weber (2006) pertains to a model of coordinated learning; see also (Bergemann and Valim¨ aki, 2003, 2006a; Athey and¨ Segal, 2007). Pai and Vohra (2006) advance the study of revenue-optimal online mechanisms in model-based environments, and together with Gallien (2006) work to extend Myerson’s (1981) optimal auction to dynamic environments; see also Cremer et al. (2007). The observation about the failure of the revelation principle, the example to illustrate the role of nonnegative payments, as well as inspiration for the example of a truthful, stochastic policy are due to Pai and Vohra (2006). For references on online algorithms and methods for solving sequential decision problems, see (Borodin and El-Yaniv, 1998; Van Hentenryck and Bent, 2006; Puterman, 1994; Kearns et al., 1999).

## Acknowledgments

Many thanks to Florin Constantin, Bobby Kleinberg, Mallesh Pai, and Rakesh Vohra for providing detailed and constructive comments on an earlier draft, and to my collaborators in this work, including Jonathan Bredin, Ruggiero Cavallo, Florin Constantin, Quang Duong, Eric Friedman, Mohammad Hajiaghayi, Adam Juda, Bobby Kleinberg, Mohammad Mahdian, Chaki Ng, and Satinder Singh. Parkes is supported in part by National Science Foundation grants IIS-0238147, IIS-0534620, and an Alfred P. Sloan Foundation award.

## Bibliography

S. Athey and I. Segal. An efficient dynamic mechanism. Technical report, Harvard University and Stanford University, 2007.

B. Awerbuch, Y. Azar, and A. Meyerson. Reducing truth-telling online mechanisms to online opti mization. In Proc. 35th Symp. on Theory ofComputing, 503–510, 2003.

M. Babaioff, N. Immorlica, and R. Kleinberg. Matroids, secretary problems, and online mechanisms. In Proc. 18th Symp. Discrete Algorithms, 434–443, 2007.

M. Babaioff, R. Lavi, and E. Pavlov. Mechanism design for single-value domains. In Proc. 20th Natl. Conf. on Artificial Intelligence, pp. 241–247, 2005.

A. Bapna and T.A. Weber. Efficient dynamic allocation with uncertain valuations. Technical report, Stanford University, 2006.

Z. Bar-Yossef, K. Hildrum, and F. Wu. Incentive-compatible online auctions for digital goods. In Proc. 13th ACM-SIAM Symp. Discrete Algorithms (SODA’02), 964–970, 2002.

D. Bergemann and J. Valim¨ aki. Dynamic common agency.¨ J. Econ. Theory, 11:23–48, 2003.

D. Bergemann and J. Valim¨ aki. Dynamic price competition.¨ J. Econ. Theory, 127:232–263, 2006a

D. Bergemann and J. Valim¨ aki. Efficient dynamic auctions. Cowles Foundation Discussion Paper¨ No. 1584, Yale University, 2006.

S. Bikhchandani, S. Chatterji, R. Lavi, A. Mu’alem, N. Nisan, and A. Sen. Weak monotonicity char acterizes deterministic dominant strategy implementation. Econometrica, 74:1109–1132, 2006.

A. Blum and J. Hartline. Near-optimal online auctions. In Proc. 16th Symp. on Discrete Algorithms, 1156–1163, 2005.

A. Blum, V. Kumar, A. Rudra, and F. Wu. Online learning in online auctions. In Proc. 14th Symp. Discrete Algorithms, 137–143, 2003.

A. Blum, T. Sandholm, and M. Zinkevich. Online algorithms for market clearing. J. ACM, 53:845– 875, 2006.

A. Borodin and R. El-Yaniv. Online Computation and Competitive Analysis. Cambridge University Press, 1998.

J. Bredin and D.C. Parkes. Models for truthful online double auctions. In Proc. 21st Conf. on Uncertainty in Artificial Intelligence, pp. 50–59, 2005.

R. Cavallo, D.C. Parkes, and S. Singh. Optimal coordinated learning among self-interested agents in the multi-armed bandit problem. In Proc. 22nd Conf. Uncertainty in Artificial Intelligence (UAI’2006), pp. 55–62, Cambridge, MA, 2006.

F. Constantin, T. Ito, and D.C. Parkes. Online auctions for bidders with interdependent values. In Proc. 6th Int. Conf. on Autonomus Agents and Multiagent Systems (AAMAS 07) poster paper, 2007.

E.B. Dynkin. The optimum choice of the instant for stopping a Markov process. Sov. Math. Dokl., 4:627–629, 1963.

E. Friedman and D.C. Parkes. Pricing WiFi at Starbucks – Issues in online mechanism design. In Proc. 4th ACM Conf. on Electronic Commerce (EC’03), pp. 240–241, 2003.

J. Gallien. Dynamic mechanism design for online commerce. Oper. Res., 54:291–310, 2006.

J. Gilbert and F. Mosteller. Recognizing the maximum of a sequence. J. Amer. Statist. Assoc., 61(313):35–73, 1966.

M.T. Hajiaghayi, R. Kleinberg, M. Mahdian, and D.C. Parkes. Online auctions with re-usable goods. In Proc. 6th ACM Conf. on Electronic Commerce (EC’05), pp. 165–174, 2005.

M.T. Hajiaghayi, R. Kleinberg, and D.C. Parkes. Adaptive limited-supply online auctions. In Proc. 5th ACM Conf. on Electronic Commerce (EC’04), pp. 71–80, 2004.

P. Van Hentenryck and R. Bent. Online Stochastic Combinatorial Optimization. MIT Press, 2006

A. Juda and D. Parkes. The sequential auction problem on eBay: An empirical analysis and a solution. In Proc. 7th ACM Conf. on Electronic Commerce (EC’06), pp. 180–189, 2006.

M. Kearns, Y. Mansour, and A.Y. Ng. A sparse sampling algorithm for near-optimal planning in large Markov Decision Processes. In Proc. 16th Int. Joint Conf. on Artificial Intelligence (IJCAI’99), pp. 1324–1331, 1999.

R. Lavi and N. Nisan. Competitive analysis of incentive compatible on-line auctions. In Proc. 2nd ACM Conf. on Electronic Commerce (EC-00), 233–241, 2000.

R. Lavi and N. Nisan. Online ascending auctions for gradually expiring goods. In Proc. 16th Annual ACM-SIAM Symp. on Discrete Algorithms (SODA’05), 2005.

R.B. Myerson. Optimal auction design. Math. ofOper. Res., 6:58–73, 1981.

C. Ng, D.C. Parkes, and M. Seltzer. Virtual Worlds: Fast and Strategyproof auctions for dynamic resource allocation. In Proc. 4th ACM Conf. on Electronic Commerce (EC’03) short paper, pp. 238–239, 2003.

M. Pai and R. Vohra. Notes on optimal dynamic auctions. Kellogg School of Management, North western University, 2006. Available from the authors.

D.C. Parkes and Q. Duong. An ironing-based approach to adaptive online mechanism design in single-valued domains. In Proc. 22nd Annual Conf. on Artificial Intelligence, 2007.

D.C. Parkes and S. Singh. An MDP-based approach to online mechanism design. In Proc. 17th Annual Conf. on Neural Information Processing Systems (NIPS’03), 2003.

D.C. Parkes, S. Singh, and D. Yanovsky. Approximately efficient online mechanism design. In Proc. 18th Annual Conf. on Neural Information Processing Systems (NIPS’04), 2004.

R. Porter. Mechanism design for online real-time scheduling. In Proc. 5th ACM Conf. on Electronic Commerce (EC’04), 61–70, 2004.

M.L. Puterman. Markov Decision Processes: Discrete Stochastic Dynamic Programming. John Wiley & Sons, New York, 1994

## Exercises

16.1 Prove that the revelation principle holds with no early-arrival and no late-departure misreports and prove the “revelation principle + heartbeats” result in combination with no early-arrival misreports.

16.2 Consider a (known interesting set) single-valued preference domain with no latedeparture misreports. Show that any decision policy π that can be truthfully implemented by an IR mechanism, and does not pay unallocated agents, must be monotonic-early (for a suitable definition of monotonic-early).

16.3 Prove that the approach outlined to constructing truthful online auctions in terms of an agent-independent price schedule q<sup>t</sup>(L , θ<sub>−</sub> , ω) induces a monotonic-late decision policy and critical-value payments. How would you modify the construction for an environment with both no early-arrival and no late-departure misreports?

16.4 Construct an example to show that the greedy auction in the expiring items setting has an arbitrarily bad competitive ratio with respect to offline VCG revenue.

16.5 Establish that the self-consistency property on prices in Section 16.3.4, coupled with the condition that a mechanism selects an outcome that maximizes utility for every agent at these prices is sufficient for truthfulness. Prove that the condition reduces to agent-independent prices for unrestrictedxs misreports.

16.6 Prove that modifications (i–iii) in Section 16.3.4 are sufficient to achieve truthful ness with agents with unknown interesting sets, together with no early-arrival and no late-departure misreports and a critical-value payment. What could break if the interesting sets are not disjoint, or if the policy is not minimal?

16.7 Show that the stochastic policy outlined in Example 16.26 satisfies monotonicity conditions (16.12) and (16.13).

16.8 Define a dynamic VCG mechanism that works for infinite time horizon and agents with a common, known discount factor $\gamma \in ( 0 , 1 )$ ).
