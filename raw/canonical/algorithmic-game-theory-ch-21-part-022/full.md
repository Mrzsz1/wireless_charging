---
title: "algorithmic-game-theory-ch-21-part-022"
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
pdf_path: "work/core-books/algorithmic-game-theory/parts/algorithmic-game-theory-ch-21-part-022.pdf"
raw_md: "raw/canonical/algorithmic-game-theory-ch-21-part-022/full.md"
---
# The Price of Anarchy and the Design of Scalable Resource Allocation Mechanisms

Ramesh Johari

## Abstract

In this chapter, we study the allocation of a single infinitely divisible resource among multiple competing users. While we aim for efficient allocation of the resource, the task is complicated by the fact that users’ utility functions are typically unknown to the resource manager. We study the design of resource allocation mechanisms that are approximately efficient (i.e., have a low price of anarchy), with low communication requirements (i.e., the strategy spaces of users are low dimensional).

Our main results concern the proportional allocation mechanism, for which a tight bound on the price of anarchy can be provided. We also show that in a wide range of market mechanisms that use a single market-clearing price, the proportional allocation mechanism minimizes the price of anarchy. Finally, we relax the assumption of a single market-clearing price, and show that by extending the class of Vickrey–Clarke–Groves mechanisms all Nash equilibria can be guaranteed to be fully efficient.

## 21.1 Introduction

This chapter deals with a canonical resource allocation problem. Suppose that a finite number of users compete to acquire a share of an infinitely divisible resource of fixed capacity. How should the resource be shared among the users? We will frame this problem as an economic problem: we assume that each user has a utility function that is increasing in the amount of the resource received, and then design a mechanism to maximize aggregate utility. In the absence of any strategic considerations, this is a simple optimization problem; however, if we assume that the agents are strategic, we need to design the resource allocation mechanisms to be robust to gaming behavior.

A central theme of this chapter is that the price of anarchy can be used as a design metric; i.e., “robust” allocation mechanisms are those that have a low price of anarchy. The present chapter is thus a bridge between two different themes of the book. The first theme is that of optimal mechanism design (Part II): given selfish agents, how do we successfully design mechanisms that nevertheless yield efficient outcomes? The second theme is that of quantifying inefficiency (Part III): given a prediction of game theoretic behavior, how well does it perform relative to some efficient benchmark? In this chapter, we use the quantification of inefficiency as the “objective function” with which we will design optimal mechanisms. As we will see, for the resource allocation problems we consider, this approach yields surprising insights into the structure of optimal mechanisms.

The mechanisms we consider for resource allocation are motivated by constraints present in modern communication networks, and similar systems where communication is limited; this precludes use of the traditional Vickrey–Clarke–Groves mechanisms (Chapter 9), which require declaration of the entire utility function. If we interpret the single resource above as a communication link, then we view the mechanism as an allocation policy operating on that link. We wish to design mechanisms that, intuitively, impose low communication overhead on the overall system; throughout this chapter, that scalability constraint translates into the assumption that the players can use only low-dimensional (in fact, one-dimensional) strategy spaces.

The remainder of the chapter is organized as follows. In Section 21.2, we introduce the basic resource allocation model we will consider in this chapter, and then introduce a simple approach to allocating the fixed resource: the proportional allocation mecha nism. In this mechanism, each user submits a bid, and receives a share of the resource in proportion to their bid. We analyze this model under both the assumption that users are price takers (i.e., that they do not anticipate the effect of their strategic decision on the price of the resource); and the assumption that users are price anticipators. The former case yields full efficiency, while in the latter we characterize the price of anarchy. In Section 21.3, we state and prove a theorem showing that in a nontrivia class of “scalable” market mechanisms (in the sense informally discussed above), the proportional allocation mechanism has the lowest price of anarchy (i.e., minimizes the efficiency loss) when users are price anticipating.

In all the mechanisms considered in the first two sections, players have onedimensional strategy spaces, and the mechanism also only chooses a single price. Because of these constraints, even the highest performance mechanisms suffer a posi tive efficiency loss, as demonstrated in Section 21.3. In the final section of the chapter, we consider the implications of removing the “single price” constraint. We show in Section 21.4 that if we consider mechanisms with scalar strategy spaces, and allow the mechanism to choose one price per user of the resource, then in fact full efficiency is achievable at Nash equilibrium. The result involves extending the well-known class of Vickrey–Clarke–Groves (VCG) mechanisms to use only a scalar strategy space; for more on VCG mechanisms, see Chapter 9.

## 21.2 The Proportional Allocation Mechanism

Suppose that R users share a resource of capacity $C > 0$ . Let $d _ { r }$ denote the amount allocated to user r. We assume that user r receives a utility equal to $U _ { r } ( d _ { r } )$ if the allocated amount is $d _ { r }$ ; we assume that utility is measured in monetary units. We make the following assumptions on the utility function; we emphasize that this assumption will be inforcefor the duration ofthe chapter, unless otherwise mentioned.

Assumption 1 For each r, over the domain $d _ { r } \geq 0$ the utility function $U _ { r } ( d _ { r } )$ is concave, strictly increasing, and continuous; and over the domain $d _ { r } > 0 , U _ { r } ( d _ { r } )$ is continuously differentiable. Furthermore, the right directional derivative at 0, denoted $U _ { r } ^ { \prime } ( 0 )$ , is finite. We let U denote the set of all utility functions satisfying these conditions.

We note that we make rather strong differentiability assumptions here on the utility functions; these assumptions are primarily made to ease the presentation. It is possible to relax the differentiability assumptions (see Notes for details).

Given complete knowledge and centralized control of the system, a natural problem for the network manager to try to solve is the following optimization problem:

SYSTEM:

$$
\text { maximize } \sum_ {r} U _ {r} (d _ {r})\tag{21.1}
$$

$$
\text { subject   to } \sum_ {r} d _ {r} \leq C;\tag{21.2}
$$

$$
d _ {r} \geq 0, r = 1, \ldots , R.\tag{21.3}
$$

Note that the objective function of this problem is the utilitarian social welfare function (cf. Chapter 17); it becomes a reasonable objective if we assume that all utilities are measured in the same (monetary) units. Since the objective function is continuous and the feasible region is compact, an optimal solution $\mathbf { d } = ( d _ { 1 } , \ldots , d _ { R } )$ exists. If the functions $U _ { r }$ are strictly concave, then the optimal solution is unique, since the feasible region is convex.

In general, the utility functions are not available to the resource manager. As a result, we consider the following pricing scheme for resource allocation, which we refer to as the proportional allocation mechanism. Each user r gives a payment (also called a bid) of $w _ { r }$ to the resource manager; we assume $w _ { r } \geq 0 .$ . Given the vector $\mathbf { w } = ( w _ { 1 } , \ldots , w _ { r } )$ , the resource manager chooses an allocation d $\mathbf { \Omega } = ( d _ { 1 } , \ldots , d _ { r } )$ . We assume the manager treats all users alike—in other words, the network manager does not price discriminate. Each user is charged the same price $\mu > 0$ , leading to $d _ { r } = w _ { r } / \mu$ . We further assume that the manager always seeks to allocate the entire resource capacity $C ;$ in this case, we expect the price $\mu$ to satisfy

$$
\sum_ {r} \frac {w _ {r}}{\mu} = C.
$$

The preceding equality can only be satisfied if $\textstyle \sum _ { r } w _ { r } > 0$ , in which case we have

$$
\mu = \frac {\sum_ {r} w _ {r}}{C}.\tag{21.4}
$$

In other words, if the manager chooses to allocate the entire resource, and does not price discriminate between users, then for every nonzero w there is a unique price $\mu > 0$ , which must be chosen by the network, given by the previous equation.

We can interpret this mechanism as a market-clearing process by which a price is set so that demand equals supply. To see this interpretation, note that when a user chooses a total payment $w _ { r }$ , it is as if the user has chosen a demandfunction $D ( p , w _ { r } ) = w _ { r } / p$ for $p > 0$ . The demand function describes the quantity the user demands at any given price $p > 0$ . The resource manager then chooses a price $\mu$ so that $\begin{array} { r } { \sum _ { r } D ( \mu , w _ { r } ) = C } \end{array}$ i.e., so that the aggregate demand equals the supply $C$ . For the specific form of demand functions we consider here, this leads to the expression for $\mu$ given in (21.4). User r then receives an allocation given by $D ( \mu , w _ { r } )$ , and makes a payment $\mu D ( \mu , w _ { r } ) = w _ { r }$ This interpretation will be further explored in Section 21.3, where we consider other market-clearing mechanisms for allocating a single resource in inelastic supply, with the users choosing demand functions from a family parameterized by a single scalar.

## 21.2.1 Price Taking Users and Competitive Equilibrium

In this section, we consider a competitive equilibrium between the users and the resource manager. A central assumption in the definition of competitive equilibrium is that each user does not anticipate the effect of their payment $w _ { r }$ on the price $\mu ;$ i.e., each user acts as a price taker. In this case, given a price $\mu > 0$ , user $r$ acts to maximize the following payoff function over $w _ { r } \geq 0 \mathrm { . }$

$$
P _ {r} (w _ {r}; \mu) = U _ {r} \left(\frac {w _ {r}}{\mu}\right) - w _ {r}.\tag{21.5}
$$

The first term represents the utility to user r of receiving a resource allocation equal to $w _ { r } / \mu ;$ the second term is the payment $w _ { r }$ made to the manager. Observe that this definition is consistent with the notion that all utilities are measured in monetary units.

We now say a pair $( \mathbf { w } , \mu )$ with $\mathbf { w } \geq 0$ and $\mu > 0$ is a competitive equilibrium if users maximize their payoff as defined in (21.5), and the network “clears the market” by setting the price $\mu$ according to (21.4):

$$
P _ {r} (w _ {r}; \mu) \geq P _ {r} (\bar {w} _ {r}; \mu) \quad \text { for } \bar {w} _ {r} \geq 0, r = 1, \dots , R;\tag{21.6}
$$

$$
\mu = \frac {\sum_ {r} w _ {r}}{C}.\tag{21.7}
$$

The following theorem shows that under our assumptions, a competitive equilibrium always exists, and any competitive equilibrium maximizes aggregate utility.

Theorem 21.1 There exists a competitive equilibrium $( \mathbf { w } , \mu )$ . In this case, the vector $\mathbf { d } = \mathbf { w } / \mu$ is an optimal solution to SYSTEM.

proof The key idea in the proof is to use Lagrangian techniques to establish that optimality conditions for (21.6)–(21.7) are identical to the optimality conditions for the problem SYSTEM, under the identification d $1 = \mathbf { w } / \mu$

Observe that under Assumption 1, the payoff (21.5) is concave in $w _ { r }$ for any $\mu > 0$ . Thus considering the first-order condition for maximization of $P _ { r } ( w _ { r } ; \mu )$ over $w _ { r } \ge 0$ , we conclude w and $\mu$ are a competitive equilibrium if and only if

$$
U _ {r} ^ {\prime} (d _ {r}) = \mu , \quad \mathrm{if} d _ {r} > 0;\tag{21.8}
$$

$$
U _ {r} ^ {\prime} (0) \leq \mu , \quad \text { if } d _ {r} = 0;
$$

$$
\sum_ {r} d _ {r} = C,\tag{21.9}
$$

(21.10)

where $d _ { r } = w _ { r } / \mu$ . A straightforward Lagrangian optimization shows that the pre ceding conditions are exactly the optimality conditions for the problem SYSTEM, so we conclude w and $\mu$ are a competitive equilibrium if and only if $\mathbf { d } = \mathbf { w } / \mu$ is a solution to SYSTEM with Lagrange multiplier $\mu$ . Since at least one solution to SYSTEM must exist, the proof is complete.

Theorem 21.1 shows that under the assumption that the users of the resource behave as price takers, there exists a bid vector w where all users have optimally chosen their bids $w _ { r }$ , with respect to the given price $\mu = \textstyle \sum _ { r } w _ { r } / C$ ; and at this “equilibrium,” aggregate utility is maximized. However, when the price taking assumption is violated, the model changes into a game and the guarantee of Theorem 21.1 is no longer valid. We investigate this game in the following section.

## 21.2.2 Price Anticipating Users and Nash Equilibrium

We now consider an alternative model where the users of a single resource are price anticipating, rather than price takers. The key difference is that while the payofffunction $P _ { r }$ takes the price $\mu$ as a fixed parameter in (21.5), price anticipating users will realize that $\mu$ is set according to (21.4), and adjust their payoff accordingly; this makes the model a game between the R players.

We use the notation $\mathbf { w } _ { - r }$ to denote the vector of all bids by users other than $r ;$ i.e., $\mathbf { w } _ { - r } = ( w _ { 1 } , w _ { 2 } , \ldots , w _ { r - 1 } , w _ { r + 1 } , \ldots , w _ { R } )$ . Given $\mathbf { w } _ { - r } ,$ each user r chooses $w _ { r }$ to maximize:

$$
Q _ {r} (w _ {r}; \mathbf {w} _ {- r}) = \left\{ \begin{array}{l l} U _ {r} \left(\frac {w _ {r}}{\sum_ {s} w _ {s}} C\right) - w _ {r}, & \text { if } w _ {r} > 0; \\ U _ {r} (0), & \text { if } w _ {r} = 0. \end{array} \right.\tag{21.11}
$$

over nonnegative $w _ { r }$ . The second condition is required so that the resource allocation to user r is zero when $w _ { r } = 0$ , even if all other users choose $\mathbf { w } _ { - r }$ so that $\textstyle \sum _ { s \neq r } w _ { s } = 0$ . The payoff function $Q _ { r }$ <sub>r</sub> is similar to the payoff function $P _ { r }$ , except that the user anticipates that the network will set the price $\mu$ according to (21.4). A Nash equilibrium of the game defined by $( Q _ { 1 } , \ldots , Q _ { R } )$ is a vector $\mathbf { w } \geq 0$ such that for all $r :$

$$
Q _ {r} (w _ {r}; \mathbf {w} _ {- r}) \geq Q _ {r} (\bar {w} _ {r}; \mathbf {w} _ {- r}), \quad \text { for   all } \bar {w} _ {r} \geq 0.\tag{21.12}
$$

Note that the payoff function in (21.11) may be discontinuous at $w _ { r } = 0$ , if $\textstyle \sum _ { s \neq r } w _ { s } = 0$ . This discontinuity may preclude existence of a Nash equilibrium; it is easy to see this in the case where the system consists of only a single user with a strictly increasing utility function. Nevertheless, as long as at least two users are competing, it is possible to show that a unique Nash equilibrium exists, by noting that such an equilibrium solves a version of the SYSTEM problem but with “modified” utility functions.

Theorem 21.2 Suppose that $R > 1$ . Then there exists a unique Nash equilib rium $\mathbf { w } \geq 0$ ofthe game defined by $( Q _ { 1 } , \ldots , Q _ { R } )$ , and it satisfies $\textstyle \sum _ { r } w _ { r } > 0$ . In

this case, the vector d defined by

$$
d _ {r} = \frac {w _ {r}}{\sum_ {s} w _ {s}} C, \quad r = 1, \dots , R,\tag{21.13}
$$

is the unique optimal solution to thefollowing optimization problem:

GAME:

$$
\text { maximize } \sum_ {r} \hat {U} _ {r} (d _ {r})\tag{21.14}
$$

$$
\text { subject   to } \sum_ {r} d _ {r} \leq C;\tag{21.15}
$$

$$
d _ {r} \geq 0, \quad r = 1, \ldots , R,\tag{21.16}
$$

where

$$
\hat {U} _ {r} (d _ {r}) = \left(1 - \frac {d _ {r}}{C}\right) U _ {r} (d _ {r}) + \left(\frac {d _ {r}}{C}\right) \left(\frac {1}{d _ {r}} \int_ {0} ^ {d _ {r}} U _ {r} (z) d z\right).\tag{21.17}
$$

proof The proof is similar to the proof of Theorem 21.1. The first key step is to note that at any Nash equilibrium, at least two components of w must be positive; this follows from the payoff (21.11) (see Exercise 17.5). Given this fact, the payoff of each user $w _ { r }$ is strictly concave and continuous in $w _ { r }$ so that w is a Nash equilibrium if and only if the following first-order conditions hold:

$$
U _ {r} ^ {\prime} \left(\frac {w _ {r}}{\sum_ {s} w _ {s}} C\right) \left(1 - \frac {w _ {r}}{\sum_ {s} w _ {s}}\right) = \frac {\sum_ {s} w _ {s}}{C}, \quad \text { if } w _ {r} > 0;\tag{21.18}
$$

$$
U _ {r} ^ {\prime} (0) \leq \frac {\sum_ {s} w _ {s}}{C}, \quad \text { if } w _ {r} = 0.\tag{21.19}
$$

Note that if we define $\rho = \sum _ { s } w _ { s } / C$ and $d _ { r } = w _ { r } / \rho$ , then the preceding condi tions can be rewritten as

$$
\hat {U} _ {r} ^ {\prime} (d _ {r}) = \rho , \quad \mathrm{if} d _ {r} > 0;\tag{21.20}
$$

$$
\hat {U} _ {r} ^ {\prime} (0) \leq \rho , \quad \mathrm{if} d _ {r} = 0;\tag{21.21}
$$

$$
\sum_ {r} d _ {r} = C.\tag{21.22}
$$

Note that these are identical to (21.8)–(21.10), but for the modified objective function (21.14). Since the utility functions $\hat { U } _ { r } ( d _ { r } )$ are strictly concave and continuous over $0 \leq d _ { r } \leq C$ , the preceding first-order conditions are sufficient optimality conditions for GAME. We conclude that w is a Nash equilibrium if and only if $\textstyle \sum _ { s } w _ { s } > 0$ , and the resulting allocation d solves the problem GAME with Lagrange multiplier $\rho = \sum _ { s } w _ { s } / C$ . To conclude the proof, observe that GAME has a strictly concave and continuous objective function over a compact feasible region, and thus has a unique optimal solution. It is straightforward to verify that this implies uniqueness of the Nash equilibrium as well.

Note that the preceding theorem gives a form of “potential” for the game under consideration: the Nash equilibrium is characterized as the unique solution to a natura optimization problem. However, the objective function for this optimization problem is not a true (exact or ordinal) potential for the game under consideration; this is because while the objective function (21.14) depends on allocations, the users’ strategic decisions are bids. Notably, this observation is in sharp contrast to the potentials found for routing games in Chapter 18, or for network formation in Chapter 19. For example, we cannot use the objective function (21.14) to conclude that best response dynamics will converge for our game. Nevertheless, the optimization formulation will help us study the price of anarchy of the game in the following section. For later reference, we note the following corollary, which uses a variational inequality formulation of the preceding theorem.

Corollary 21.3 Suppose that $R > 1$ . Let w be the unique Nash equilibrium of the game defined by $( Q _ { 1 } , \ldots , Q _ { R } )$ , and define d according to (21.13). Then fo any other vector $\bar { \mathbf { d } } \geq 0$ such that $\begin{array} { r } { \sum _ { r } \bar { d } _ { r } \leq C } \end{array}$ , there holds:

$$
\sum_ {r} \hat {U} _ {r} ^ {\prime} (d _ {r}) (\bar {d} _ {r} - d _ {r}) \leq 0.\tag{21.23}
$$

proof The stated condition follows easily from (21.20)–(21.22), the optimality conditions for the problem GAME.

## 21.2.3 Price of Anarchy

We let ${ \bf d } ^ { S }$ denote an optimal solution to SYSTEM, and let ${ \bf d } ^ { G }$ denote the unique optimal solution to $G A M E$ . We now investigate the price of anarchy of this system; i.e., how much utility is lost because the users are price anticipating? To answer this question, we must compare the utility $\textstyle \sum _ { r } U _ { r } ( d _ { r } ^ { G } )$ ) obtained when the users fully evaluate the effect of their actions on the price, and the utility $\textstyle \sum _ { r } U _ { r } ( d _ { r } ^ { S } )$ obtained by choosing the point that maximizes aggregate utility. (We know, of course, that $\begin{array} { r } { \sum _ { r } U _ { r } ( d _ { r } ^ { G } ) \leq \sum _ { r } U _ { r } ( d _ { r } ^ { S } ) } \end{array}$ , by definition of $\mathbf { d } ^ { \hat { S } } . )$ ) As we show in the following theorem, the efficiency loss is exactly 25% in the worst case.

Theorem 21.4 Suppose that $R > 1$ . Suppose also that $U _ { r } ( 0 ) \geq 0$ for all r. If ${ \bf d } ^ { S }$ is any optimal solution to SYSTEM, and ${ \bf d } ^ { G }$ is the unique optimal solution to GAME, then:

$$
\sum_ {r} U _ {r} (d _ {r} ^ {G}) \geq \frac {3}{4} \sum_ {r} U _ {r} (d _ {r} ^ {S}).
$$

Furthermore, this bound is tight:for every $\epsilon > 0$ , there exists a choice of R, and a choice of(linear) utilityfunctions $U _ { r } , r = 1 , \ldots , R$ , such that

$$
\sum_ {r} U _ {r} (d _ {r} ^ {G}) \leq \left(\frac {3}{4} + \epsilon\right) \left(\sum_ {r} U _ {r} (d _ {r} ^ {S})\right).
$$

proof Our proof will rely on the following constant $\beta : ^ { 1 }$

$$
\beta = \inf _ {U \in \mathcal {U}} \inf _ {C > 0} \inf _ {0 \leq d, \bar {d} \leq C} \frac {U (d) + \hat {U} ^ {\prime} (d) (\bar {d} - d)}{U (\bar {d})}.\tag{21.24}
$$

Recall the definition of $\mathcal { U }$ in Assumption 1, and of $\hat { U }$ in (21.17).

Our proof involves using Corollary 21.3 to prove that $\beta$ is a tight bound on the efficiency of Nash equilibria. We first establish that $\beta \ge 3 / 4$ . Note that in (21.24), the quotient is strictly larger than 1 if $d > \bar { d }$ , and equal to 1 if $d = \bar { d }$ . Thus in computing $\beta$ we can assume that $d < \bar { d }$ in (21.24). We then have:

$$
\begin{array}{l} U (d) + \hat {U} ^ {\prime} (d) (\bar {d} - d) = U (d) + U ^ {\prime} (d) \left(1 - \frac {d}{C}\right) (\bar {d} - d) \\ \qquad \qquad \qquad \geq U (d) + \left(1 - \frac {d}{\bar {d}}\right) (U (\bar {d}) - U (d)) \\ \qquad \qquad \qquad \geq \left(\frac {d}{\bar {d}}\right) ^ {2} U (\bar {d}) + \left(1 - \frac {d}{\bar {d}}\right) U (\bar {d}) \\ \qquad \qquad \qquad \geq \frac {3}{4} U (\bar {d}). \end{array}
$$

The first inequality follows since $\bar { d } \le C$ and $U$ is concave. The second inequality follows since $U$ is concave and nonnegative and $d \leq \bar { d }$ , so $U ( d ) \geq ( d / \bar { d } ) U ( \bar { d } )$ Finally, the third inequality follows since $x ^ { 2 } - x + 1$ is minimized at $x = 1 / 2$ . It follows from (21.24) that $\beta \ge 3 / 4$

Next, we show that for any $\delta > 0$ , there exists an example where the ratio of Nash aggregate utility to maximum aggregate utility is at least $\beta + \delta$ . Our approach is essentially the same as that in Example 17.6. Fix $U , d < \bar { d }$ , and let $C = { \bar { d } }$ . Consider the following example. Suppose that $R > 1$ 1 users compete for the resource. Let user 1 have utility function $U _ { 1 } = U$ , and suppose users $2 , \ldots$ , R have linear utility functions with slope $\hat { U } ^ { \prime } ( d )$ ; i.e., $U _ { r } ( d _ { r } ) = \hat { U } ^ { \prime } ( d ) d _ { r } = ( U ^ { \prime } ( d ) ( 1 -$ $d / C ) ) d _ { r }$ . Let ${ \bf d } ^ { S }$ denote an optimal solution to SYSTEM for this model; since one feasible solution involves allocating the entire resource $\bar { d }$ to user 1, we must have $\begin{array} { r } { \sum _ { s } U _ { s } ( d _ { s } ^ { S } ) \geq U ( \bar { d } ) } \end{array}$ . On the other hand, recall that at any Nash equilibrium at least two users have positive quantities; and since the Nash equilibrium is unique, we conclude that all users $2 , \ldots , R$ receive the same positive quantity. Thus as $R \to \infty$ , we must have $d _ { r } \downarrow 0$ for $r = 2 , \ldots , R$ . From (21.20)–(21.21), it follows that the Nash price $\sum _ { s } w _ { s } / C$ must converge to $\hat { U } ^ { \prime } ( d )$ as $R \to \infty$ . Thus, at the Nash equilibrium, user 1 receives an allocation $d + \epsilon$ , and all other users receive an allocation $( 1 - d - \epsilon ) / ( R - 1 )$ , where $\epsilon  0$ as $R \to \infty$ . The total Nash utility thus converges to $U ( d ) + \hat { U } ^ { \prime } ( d ) ( \bar { d } - d )$ . The limiting ratio of Nash aggregate utility to maximum aggregate utility is thus less than or equal to

$$
\frac {U (d) + \hat {U} ^ {\prime} (d) (\bar {d} - d)}{U (\bar {d})}.
$$

We conclude that for any $\delta > 0$ , there exists a game $( Q _ { 1 } , \ldots , Q _ { R } )$ in which the ratio of Nash aggregate utility to maximum aggregate utility is at most $\beta + \delta$ By considering the special case in which $U ( \hat { d } ) = \hat { d } , d = 1 / 2$ , and $\bar { d } = 1$ , the preceding construction yields a limiting efficiency ratio of exactly $3 / 4$ . Combined with the previous argument that $\beta \ge 3 / 4$ , it follows that in fact $\beta = 3 / 4$

It remains to show that the bound holds for every resource allocation game. Here we simply apply the result of Corollary 21.3. Let $( Q _ { 1 } , \ldots , Q _ { R } )$ be a resource allocation game where users have utility functions $( U _ { 1 } , \dots , U _ { R } )$ . Let ${ \bf d } ^ { S }$ be a solution to SYSTEM, and let ${ \bf d } ^ { G }$ be a solution to GAME. We have

$$
\sum_ {s} U _ {s} \left(d _ {s} ^ {S}\right) \leq \sum_ {s} \frac {1}{\beta} \left(U _ {s} \left(d _ {s} ^ {G}\right) + \hat {U} _ {s} ^ {\prime} \left(d _ {s} ^ {G}\right) \left(d _ {s} ^ {S} - d _ {s} ^ {G}\right)\right) \leq \frac {1}{\beta} \sum_ {s} U _ {s} \left(d _ {s} ^ {G}\right).
$$

The first inequality follows by the definition of $\beta _ { ; }$ , and the second follows from Corollary 21.3. Since $\beta = 3 / 4$ , this concludes the proof.

The preceding theorem shows that in the worst case, aggregate utility falls by no more than 25% when users are able to anticipate the effects of their actions on the price of the resource. Furthermore, this bound is essentially tight. In fact, it follows from the proof that the worst case consists of a resource of capacity 1, where user 1 has utility $U _ { 1 } ( d _ { 1 } ) = d _ { 1 }$ , and all other users have utility $U _ { r } ( d _ { r } ) \approx d _ { r } / 2$ (when R is large). As $R \to \infty$ , at the Nash equilibrium of this game user 1 receives a quantity $d _ { 1 } ^ { G } = 1 / 2$ , while the remaining users uniformly split the quantity $1 - d _ { 1 } ^ { G } = 1 / 2$ among themselves, yielding an aggregate utility of $3 / 4 .$ . On the other hand, the maximum aggregate utility possible is clearly 1, achieved by allocating the entire resource to user 1.

## 21.3 A Characterization Theorem

In this chapter we ask an axiomatic question: Is the mechanism we have chosen “desirable” among a class of mechanisms satisfying certain “reasonable” properties? Defining desirability is the simpler of the two tasks: we consider a mechanism to be desirable if it minimizes efficiency loss when users are price anticipating. Importantly, we ask for this efficiency property independent of the characteristics of the market participants (i.e., their cost functions or utility functions). That is, the mechanisms we seek are those that perform well under broad assumptions on the nature of the preferences of market participants.

How do we define “reasonable” mechanisms? The most important condition we impose is that the strategy space of each market participant should be “simple,” which we interpret as low dimensional. Formally, we will focus on mechanisms for which the strategy space of each market participant is $\mathbb { R } ^ { + }$ ; i.e., each market participant chooses a scalar, which is a parameter that determines his demand function as input to the marketclearing mechanism. The primary motivation is that if we view such a mechanism to be useful for a communication network setting, information flow is limited; and in particular, we would like to implement a market with as little overhead as possible.

Thus keeping the strategy spaces of the users low dimensional is a reasonable goal.<sup>2</sup> We will show that under a specific set of mathematical assumptions, the proportional allocation mechanism in fact minimizes the worst-case efficiency loss when users are price anticipating.

The class of market mechanisms we will consider is defined as follows. A market mechanism must operate on a particular environment, defined by a triple $( C , R , { \bf U } )$ $C > 0$ denotes the capacity of the resource; $R > 1$ denotes the number of users sharing the resource; and $\mathbf { U } = ( U _ { 1 } , \dots , U _ { R } )$ denotes the utility functions of the users, with $U _ { r } \in \mathcal { U }$ (cf. Assumption 1). The following definition captures our notion of a market mechanism.

Definition 21.5 A smooth market-clearing mechanism is a differentiable function $D \colon ( 0 , \infty ) \times [ 0 , \infty ) \to \mathbb { R } ^ { + }$ such that for all $C > 0 ,$ , for all $R > 1$ , and for all nonzero $\pmb { \theta } \in ( \mathbb { R } ^ { + } ) ^ { R }$ , there exists a unique solution $p > 0$ to the following equation:

$$
\sum_ {r = 1} ^ {R} D (p, \theta_ {r}) = C.
$$

We let $p _ { D } ( \pmb \theta )$ denote this solution.<sup>3</sup>

Note that the market-clearing price is undefined if ${ \pmb \theta } = { \bf 0 }$ . As we will see below, when we formulate a game between users for a given mechanism D, we will assume that the payoff to all players is $- \infty$ if the composite strategy vector is ${ \pmb \theta } = { \bf 0 }$ . Note that this is slightly different from the definition in Section 21.1, where the payoff is $U ( 0 )$ to a player with utility function U who submits a strategy $\theta = 0$ . We will discuss this distinction further later; we simply note for the moment that it does not affect the results of this section.

Our definition of a smooth market-clearing mechanism generalizes the demand function interpretation of the proportional allocation mechanism. Recall that for that mechanism, each user submits a demand function of the form $D ( p , \theta ) = \theta / p$ , and the link manager chooses a price $p _ { D } ( \pmb \theta )$ to ensure that $\begin{array} { r } { \sum _ { r = 1 } ^ { R } D ( p , \theta _ { r } ) = C } \end{array}$ . Thus, for this mechanism, we have $\textstyle p _ { D } ( \pmb { \theta } ) = \sum _ { r = 1 } ^ { R } \theta _ { r } / C$ if $\pm 0$

We now generalize competitive equilibria and Nash equilibria to this setting.

Definition 21.6 Given a utility system (C, R, U) and a smooth market-clearing mechanism D, we say that a nonzero vector $\pmb { \theta } \in ( \mathbb { R } ^ { + } ) ^ { R }$ is a competitive equilib rium if, for $\mu = p _ { D } ( \pmb \theta )$ ), there holds for all r:

$$
\theta_ {r} \in \arg \max _ {\bar {\theta} _ {r} \geq 0} [ U _ {r} (D (\mu , \bar {\theta} _ {r})) - \mu D (\mu , \bar {\theta} _ {r}) ].\tag{21.25}
$$

Definition 21.7 Given a utility system $( C , R , \mathbf { U } )$ and a smooth market-clearing mechanism D, we say that a nonzero vector $\pmb { \theta } \in ( \mathbb { R } ^ { + } ) ^ { R }$ is a Nash equilibrium if there holds for all $r )$ :

$$
\theta_ {r} \in \arg \max _ {\bar {\theta} _ {r} \geq 0} Q _ {r} (\bar {\theta} _ {r}; \boldsymbol {\theta} _ {- r}).\tag{21.26}
$$

where

$$
Q _ {r} (\theta_ {r}; \boldsymbol {\theta} _ {- r}) = \left\{ \begin{array}{l l} U _ {r} (D (p _ {D} (\boldsymbol {\theta}), \theta_ {r})) - p _ {D} (\boldsymbol {\theta}) D (p _ {D} (\boldsymbol {\theta}), \theta_ {r}), & \text { if } \boldsymbol {\theta} \neq \mathbf {0}; \\ - \infty , & \text { if } \boldsymbol {\theta} = \mathbf {0}. \end{array} \right.\tag{21.27}
$$

Notice that the payoff $Q _ { r } \mathrm { i s } - \infty$ if the composite strategy vector is ${ \pmb \theta } = { \bf 0 }$ , since in this case no market-clearing price exists.

We are now ready to frame the specific class $\mathcal { D }$ of market mechanisms we will consider in this section, defined as follows.

Definition 21.8 The class D consists of all functions $D ( p , \theta )$ such that the following conditions are satisfied:

(i) $D$ is a smooth market-clearing mechanism (cf. Definition 21.5).

(ii) For all $C > 0$ , and for all $U _ { r } \in \mathcal { U } ,$ , a user’s payoff is concave if he is price anticipating; i.e., for all $R ,$ , and for all $\pmb { \theta } _ { - r } \in ( \mathbb { R } ^ { + } ) ^ { R }$ , the function:

$$
U _ {r} (D (p _ {D} (\boldsymbol {\theta}), \theta_ {r}) - p _ {D} (\boldsymbol {\theta}) D (p _ {D} (\boldsymbol {\theta}), \theta_ {r})
$$

is concave in $\theta _ { r } > 0 \mathrm { i f } \pmb { \theta } _ { - r } = \mathbf { 0 }$ , and concave in $\theta _ { r } \geq 0 \mathrm { i f } \pmb \theta _ { - r } \neq \mathbf 0$

(iii) For all $p > 0$ , and for all $d \geq 0$ , there exists a $\theta > 0$ such that $D ( p , \theta ) = d .$

(iv) The demand functions are nonnegative; i.e., for all $p > 0$ and $\theta \ge 0 , D ( p , \theta ) \ge 0$

We pause here to briefly discuss the conditions in the previous definition. The second allows us to characterize Nash equilibria in terms of only first-order conditions. To justify this condition, we note that some assumption of quasiconcavity is generall used to guarantee existence of pure strategy Nash equilibria. The third condition ensures that given a price $p$ and desired allocation $d \in [ 0 , C ]$ , each player can make a choice of $\theta$ to guarantee precisely the allocation $d .$ . This is an “expressiveness” condition on the mechanism that ensures that all possible demands can be chosen at any market-clearing price. The last condition is a normalization condition, which ensures that regardless of the bid of a user, he is never required to supply some quantity of the resource (which would be the case if we allowed $D ( p , \theta ) < 0 )$ . The following example gives a family of mechanisms that lie in D.

Example 21.9 Suppose that $D ( p , \theta ) = \theta p ^ { - 1 / c }$ , where $c \geq 1$ . It is easy to check that this class of mechanisms satisfies $D \in \mathcal { D }$ for all choices of $c ;$ when $c = 1$ we recover the proportional allocation mechanism of Section 21.2. The marketclearing condition yields that $\begin{array} { r } { p _ { D } ( \pmb { \theta } ) = ( \sum _ { r } \theta _ { r } / C ) ^ { 1 / c } } \end{array}$ . Note that as a result, the allocation to user at a nonzero vector $\pmb \theta$ is

$$
D (p _ {D} (\boldsymbol {\theta}), \theta_ {r}) = \frac {\theta_ {r}}{\sum_ {s} \theta_ {s}} C.
$$

In other words, regardless of the value of c, the market clearing allocations are chosen proportional to the bids. This remarkable fact is a special case of a more general result we establish below: all mechanisms in $\mathcal { D }$ yield market-clearing allocations that are proportional to the bids; they differ only in the market-clearing price that is chosen. The exercises study the price of anarchy of the mechanisms defined in this example using an approach analogous to the proofofTheorem 21.4.

Our interest is in the worst-case ratio of aggregate utility at any Nash equilibrium to the optimal value of SYSTEM. Formally, for $D \in \mathcal { D }$ we define a constant $\rho ( D )$ as follows:

$$
\begin{array}{l} \rho (D) = \inf \left\{\frac {\sum_ {r = 1} ^ {R} U _ {r} (D (p _ {D} (\boldsymbol {\theta}) , \theta_ {r}))}{\sum_ {r = 1} ^ {R} U _ {r} (d _ {r})} \Bigg | C > 0, R > 1, \mathbf {U} \in \mathcal {U} ^ {R}, \right. \\ \left. \mathbf {d} \text {   solves   SYSTEM,   and   } \boldsymbol {\theta} \text {   is   a   Nash   equilibrium } \right\}. \end{array}
$$

Note that since all $U \in \mathcal { U }$ are strictly increasing and nonnegative, the aggregate utility $\textstyle \sum _ { r = 1 } ^ { R } U _ { r } ( d _ { r } ^ { S } )$ is positive for any utility system $( C , R , { \bf U } )$ with $C > 0$ , and any optimal solution ${ \bf d } ^ { \bf S }$ to SYSTEM. Note also that we are considering the ratio over all possible Nash equilibria, not just the best one for a given instance; thus, we are studying the price of anarchy, not the price of stability (cf. Chapter 17). However, Nash equilibria may not exist for some utility systems $( C , R , { \bf U } ) ;$ ; in this case we set $\rho ( D ) = - \infty$

Our main result in this section is the following theorem.

Theorem 21.10 Let $D \in \mathcal { D }$ be a smooth market-clearing mechanism. Then:

(i) There exists a competitive equilibrium θ. Furthermore, for any such $\pmb \theta _ { i }$ , the $r e \mathrm { - }$ sulting allocation d given by $d _ { r } = D ( p _ { D } ( \pmb { \theta } ) , \theta _ { r } )$ solves SYSTEM.

(ii) There exists a concave, strictly increasing, differentiable, and invertiblefunction $B \colon ( 0 , \infty ) \to ( 0 , \infty )$ such thatfor all $p > 0$ and $\theta \ge 0$

$$
D (p, \theta) = \frac {\theta}{B (p)}.
$$

(iii) $\rho ( D ) \leq 3 / 4$ , and this bound is met with equality ifand only if $D ( p , \theta ) = \Delta \theta / p$ for some $\Delta > 0$

Before continuing to the proof of the theorem, we pause to make several critical comments about the result. Results (i) and (ii) of the theorem are a characterization of the types of mechanisms allowed by the constraints that define $\mathcal { D } .$ . In particular, notice that from (ii), for nonzero $\pmb \theta$ we have

$$
B (p _ {D} (\pmb {\theta})) = \frac {\sum_ {r = 1} ^ {R} \theta_ {r}}{C}.\tag{21.28}
$$

Thus we must have

$$
D (p _ {D} (\boldsymbol {\theta}), \theta_ {r}) = \frac {\theta_ {r}}{\sum_ {s} \theta_ {s}} C;\tag{21.29}
$$

in other words, every mechanism in D chooses allocations in proportion to the bids. As a result, we conclude that for a given vector $\pmb \theta$ , when the market clears, mechanisms in D differ from the proportional allocation mechanism only in the market-clearing price—the allocation is the same. Result (iii) of the theorem is then a price of anarchy result that concerns mechanisms of this form.

We emphasize that the theorem here is distinguished from related work because the allocation rule (21.29) was not assumed in advance. Rather, the result here starts from a set of simple assumptions on the structure of mechanisms to be considered (the definition of the class D), and uses them to prove that any mechanism in the class must lead to the allocation in (21.29). (See Notes for details.)

proof Throughout the proof we fix a particular mechanism $D \in \mathcal { D }$ . Some computational details are left to the reader.

Step 1: A user’s payoffis concave ifhe is price taking. In other words, we wil show that for all $U \in \mathcal { U }$ and for all $p > 0 , U ( D ( p , \theta ) ) - p D ( p , \theta )$ is concave in $\theta .$ . The key idea is to use a limiting regime where capacity grows large, so that users that are price anticipating effectively become price taking.

Formally, we first observe that since D must possess a unique market-clearing price regardless of the value of $C , D ( p , \theta )$ must be strictly monotonic in $p$ (for fixed $\theta > 0 )$ ) where it is nonzero, and either (1) $D ( p , \theta )$ is nondecreasing in $p$ for all $\theta > 0 , \mathrm { o r } ( 2 ) D ( p , \theta )$ is nonincreasing in $p$ for all $\theta > 0$

To complete the proof of this step, fix $\mu > 0$ , and fix $\theta > 0$ . Now consider a limit where $R \to \infty$ , and $C ^ { R } = R D ( \mu , \theta )$ is the capacity in the $R '$ th system. It is straightforward to check that if the $R - 1$ users $2 , \ldots , R$ submit strategy $\theta _ { ; }$ , and the first user submits strategy $\theta ^ { \prime }$ , then the resulting market-clearing price $p _ { D }$ converges to µ as $R \to \infty$ , regardless of the value of $\theta ^ { \prime }$ . This step uses the fact that either (1) or (2) above holds. Applying the fact that player 1’s payoff must be concave when he is price anticipating and taking limits as $R \to \infty ,$ , it follows that player 1’s payoff is concave when he is price taking for any fixed price $\mu > 0$

Step 2: There exists a positive function B such that $D ( p , \theta ) = \theta / B ( p )$ for $p > 0$ and $\theta \ge 0$ . By Step 1, a player’s payoff is concave when he is price taking. By appropriately choosing a linear utility function with very large slope and very small slope, it follows that $D ( p , \theta )$ must be concave and convex, respectively, in $\theta$ for a given $p > 0$ . Thus for fixed $p > 0 , D ( p , \theta )$ is an affine function of $\theta$ . Conditions 3 and 4 in Definition 21.8 then imply that the constant term must be zero, while the coefficient of the linear term is positive; thus, $D ( p , \theta ) = \theta / B ( p )$ for some positive function $B ( p )$

Before continuing, we note that the previous step already implies the remark able fact that for any mechanism $D \in \mathcal { D }$ , the allocation at the market-clearing price is made in proportion to the bids $\theta$ . This follows from the discussion following (21.28) above.

Step 3: For all utility systems $( C , R , { \bf U } )$ , there exists a competitive equilibrium, and it is fully efficient. This step follows primarily because of Condition 3 in Definition 21.8: given a price $\mu ,$ , a user can first determine his optimal choice of quantity, and then choose a parameter $\theta$ to express this choice. Formally, suppose that $\mu = p _ { D } ( \pmb \theta )$ , and (21.25) holds. Let $d _ { r } = D ( \mu , \theta _ { r } )$ ; then (21.25) implies that the necessary conditions (21.8)–(21.9) hold; these are also sufficient because of Step 1. Furthermore, market clearing implies (21.10) holds. Thus any competitive equilibrium is fully efficient. Existence follows by letting ${ \bf d } ^ { S }$ be a solution to SYSTEM with Lagrange multiplier $\mu ,$ , and choosing $\theta _ { r } = d _ { r } / B ( \mu )$

Step 4: For all $R > 1$ and $\pmb { \theta } _ { - r } \in ( \mathbb { R } ^ { + } ) ^ { R - 1 }$ , the functions $D ( p _ { D } ( \pmb { \theta } ) , \theta _ { r } )$ and $- p _ { D } ( \pmb { \theta } ) D ( p _ { D } ( \pmb { \theta } ) , \theta _ { r } )$ are concave in $\theta _ { r } > 0 \mathrm { \ } i f \pmb { \theta } _ { - r } = \mathbf { 0 }$ , and concave in $\theta _ { r } \geq 0$ $i f \pmb \theta _ { - r } \neq \pmb 0$ . As in Step 2, this conclusion follows by considering linear utilit functions with very large and very small slope, respectively.

Step 5: B is an invertible, differentiable, strictly increasing, and concave function on $( 0 , \infty )$ . We immediately see that B must be invertible on $( 0 , \infty ) ;$ ; it is clearly onto, as the right-hand side of (21.28) can take any value in $( 0 , \infty )$ Furthermore, uniqueness of the market-clearing price in (21.28) requires that B is one-to-one as well, and hence invertible. Since $D$ is differentiable, B must be differentiable as well. Let $\Phi$ denote the differentiable inverse of B on $( 0 , \infty )$ ; we will show $\Phi$ is strictly increasing and convex.

Let

$$
w _ {r} (\boldsymbol {\theta}) = p _ {D} (\boldsymbol {\theta}) D (p _ {D} (\boldsymbol {\theta}), \theta_ {r}) = \Phi \left(\frac {\sum_ {s = 1} ^ {R} \theta_ {s}}{C}\right) \left(\frac {\theta_ {r}}{\sum_ {s = 1} ^ {R} \theta_ {s}} C\right).\tag{21.30}
$$

By Step 4, $w _ { r } ( \pmb \theta )$ is convex in $\theta _ { r } > 0$ . By considering strategy vectors $\pmb \theta$ for which $\pmb \theta _ { - r } = \mathbf 0$ , it follows that $\Phi$ is convex. Finally, the fact that  is strictly increasing follows by differentiating twice and considering the limit where $\theta _ { r } \to 0 .$ , while keeping $\pmb { \theta } _ { - r }$ constant and nonzero.<sup>4</sup> This establishes the desired facts regarding B.

Step 6: Let (C, R, U) be a utility system. A vector $\pmb \theta \ge 0$ is a Nash equilibrium ifand only ifat least two components ofθ are nonzero, and there exists a nonzero vector ${ \bf d } \ge 0$ and a scalar $\mu > 0$ such that $\theta _ { r } = \mu d _ { r }$ for all $r , \textstyle \sum _ { r = 1 } ^ { R } d _ { r } = C$ , and thefollowing conditions hold:

$$
U _ {r} ^ {\prime} \left(d _ {r}\right) \left(1 - \frac {d _ {r}}{C}\right) = \Phi (\mu) \left(1 - \frac {d _ {r}}{C}\right) + \mu \Phi^ {\prime} (\mu) \left(\frac {d _ {r}}{C}\right), \quad i f d _ {r} > 0;\tag{21.31}
$$

$$
U _ {r} ^ {\prime} (0) \leq \Phi (\mu), \quad i f d _ {r} = 0.\tag{21.32}
$$

In this case $\begin{array} { r } { d _ { r } = D ( p _ { D } ( \pmb { \theta } ) , \theta _ { r } ) , \mu = \sum _ { r = 1 } ^ { R } \theta _ { r } / C } \end{array}$ , and $\Phi ( \mu ) = p _ { D } ( \pmb { \theta } )$ . Further, there exists a unique Nash equilibrium. The proof of this step is similar to the proof of Nash equilibrium characterization in Theorem 21.2; we omit the details, and refer the reader to the Notes section.

Step 7: For any $\epsilon > 0$ , there exists a utility systems $( C , R , { \bf U } )$ such that at any Nash equilibrium θ, the aggregate utility is no more than $3 / 4 + \epsilon$ ofthe maximal aggregate utility. Consider a utility system with the following properties. Let $C = 1$ . Fix $\mu > 0$ , and let $U _ { 1 } ( d _ { 1 } ) = A d _ { 1 }$ , where $A > \Phi ( \mu )$ . We will search for a solution to the Nash conditions (21.31) to (21.32) with market-clearing price $\Phi ( \mu )$

We start by calculating $d _ { 1 }$ by assuming it is nonzero, and applying (21.31):

$$
d _ {1} = \frac {(A - \Phi (\mu)) C}{A - \Phi (\mu) + \mu \Phi^ {\prime} (\mu)}.\tag{21.33}
$$

In the spirit of the proof of Theorem 21.4, we will now choose users $2 , \ldots , R$ to have identical linear utility functions, with slopes less than A. As we will see, this will be possible if R is large enough.

Formally, let $d = ( C - d _ { 1 } ) / ( R - 1 )$ , and (cf. (21.31)) define

$$
\alpha = \frac {\Phi (\mu) C + (\mu \Phi^ {\prime} (\mu) - \Phi (\mu)) d}{C - d}.\tag{21.34}
$$

Let $U _ { r } ( d _ { r } ) = \alpha d _ { r }$ for $r = 2 , \ldots , R$ . Note that if

$$
\frac {C}{R} \leq \frac {(A - \Phi (\mu)) C}{A - \Phi (\mu) + \mu \Phi^ {\prime} (\mu)},\tag{21.35}
$$

then $\alpha \leq A$ . This guarantees $d _ { 1 }$ must be nonzero at any Nash equilibrium, so that the computation in (21.33) is valid. In turn, letting $d _ { r } = d$ for $r = 2 , \ldots , R$ , this implies that $( d _ { 1 } , \ldots , d _ { R } )$ and $\mu$ are a valid solution to (21.31)–(21.32), when users have utility functions $U _ { 1 } , \dots , U _ { R }$

Now consider the limiting ratio of Nash aggregate utility to maximal aggregate utility, as $R \to \infty$ . We have $d \to 0 .$ , so $\alpha  \Phi ( \mu )$ . Furthermore, regardless of R a solution to SYSTEM is to allocate the entire resource to user 1, so the maximal aggregate utility is $A C$ . Thus the limiting ratio of Nash aggregate utility to maximal aggregate utility becomes

$$
\frac {(A - \Phi (\mu))}{A - \Phi (\mu) + \mu \Phi^ {\prime} (\mu)} + \left(1 - \frac {(A - \Phi (\mu))}{A - \Phi (\mu) + \mu \Phi^ {\prime} (\mu)}\right) \left(\frac {\Phi (\mu)}{A}\right).\tag{21.36}
$$

We now want to find the choices of A and $\mu$ which minimize this value.

For notational simplicity, we define $x = \Phi ( \mu ) / A$ , and $\Psi ( \mu ) = \mu \Phi ^ { \prime } ( \mu ) / \Phi ( \mu )$ Note that given the convexity and invertibility of $\Phi .$ , we have $\Psi ( \mu ) \geq 1$ . Then (21.36) is equivalent to

$$
F (x; \mu) = \frac {(1 - x) ^ {2}}{1 + (\Psi (\mu) - 1) x} + x.\tag{21.37}
$$

It is straightforward to establish that the preceding expression is strictly convex in x for fixed $\mu$ . Let $G ( \Psi ( \mu ) )$ ) denote the minimal value of $F ( x ; \mu )$ for $x \in ( 0 , 1 )$ ;

![](images/7d093b3b23d15d55ee8d1d8fdb62f066b936ba06a2cd790f0f35199046a7243d.jpg)  
Figure 21.1. The function $C ( \Psi )$ defined in (21.38). Note that G() is strictly decreasing, with $C ( 1 ) = 3 / 4$

by differentiating, it follows that $G ( \Psi )$ is defined for $\Psi \geq 1$ according to

$$
G (\Psi) = \left\{ \begin{array}{l l} \frac {3}{4}, & \text { if } \Psi = 1; \\ \frac {2 \Psi^ {2} - 3 \Psi \sqrt {\Psi} + \sqrt {\Psi}}{(\Psi - 1) ^ {2} \sqrt {\Psi}}, & \text { if } \Psi > 1. \end{array} \right.\tag{21.38}
$$

The function $G$ is plotted in Figure 21.1. It is straightforward to verify that $G ( \Psi )$ is continuous and strictly decreasing for $\Psi \geq 1$ so that the worst-case example is given by finding $\mu > 0$ such that $\Psi ( \mu )$ is maximized. Furthermore, it is straightforward to check that $G ( \Psi ) \leq 3 / 4$ , establishing the required claim.

Step 8: For any mechanism other than the proportional allocation mechanism, the worst-case efficiency is strictly lower than $3 / 4$ . For the proportional allocation mechanism, we have $\Psi ( \mu ) = 1$ , and we have already established that the efficiency $\rho$ is exactly $3 / 4$ . On the other hand, it is straightforward to check that if $B ( p )$ is nonlinear, then the maximal value of $\Psi ( \mu )$ in the preceding step is strictly greater than 1; and in this case $G ( \Psi ( \mu ) )$ is strictly less than $3 / 4$ . Thus there exists a game with efficiency ratio strictly lower than $3 / 4$ for such a mechanism. This completes the proof.

We make several comments regarding the proof. First, notice that every mechanism in the described class allocates in proportion to the bids of the players; in this sense all mechanisms in $\mathcal { D }$ are “proportional allocation mechanisms.” However, the efficiency loss is minimized exactly when this mechanism charges each user exactly their bid. Second, it is possible to show that the bound constructed in Steps $_ { 7 - 8 }$ of the proof is in fact a tight bound on the price of anarchy of the mechanisms under consideration; it is possible to reformulate this bound so that it depends only on the elasticity of the function $B ( p ) , \mathrm { i . e . }$ ., the quantity $\mathrm { i n f } _ { p > 0 } p B ^ { \prime } ( p ) / B ( p )$ . (This is not surprising, since $\Psi ( \mu )$ is the elasticity of the function , which is the inverse of B.) It is surprising that the price of anarchy of a general class of such mechanisms can be reduced to this parsimonious calculation.

Finally, we note one potentially undesirable feature of the family of market-clearing mechanisms considered: the payoff to user r is defined as $- \infty$ when the composite strategy vector is $\pmb \theta = 0$ (cf. (21.27)). This definition is required because when the composite strategy vector is $\pmb \theta = 0$ , a market-clearing price may not exist. One possible remedy is to restrict attention instead to mechanisms where $D ( p , \theta ) = 0 { \mathrm { i f } } \theta = 0$ , for all $p \geq 0 ;$ ; in this case we can define $p _ { D } ( \pmb \theta ) = 0 \mathrm { i f } \pmb \theta = 0$ , and let the payoff to user r be $U _ { r } ( 0 )$ if $\theta _ { r } = 0$ . This condition amounts to a “normalization” on the market-clearing mechanism. It is possible to show that this modification does not alter the conclusion of Theorem 21.10.

## 21.4 The Vickrey–Clarke–Groves Approach

The mechanisms we considered in the last section had several restrictions placed on them; chief among these are that (1) users are restricted to using “simple” strategy spaces and (2) the mechanism uses only a single price to clear the market. On the other hand, one could consider both generalizations where users are allowed to use more complex strategies, perhaps declaring their entire utility function to the market; and also, where price discrimination is allowed so that each user is charged a personalized per-unit price for the resource.

The best known solution employing both these generalizations is the VCG approach to eliciting utility information (see Notes, and Chapter 9). Such mechanisms allow users to declare their entire utility functions, and then charge users individualized prices so that they have the incentive to truthfully declare their utilities. We review VCG mechanisms in Section 21.4.1.

In this section we are interested in deciding whether the same outcome can be realized preserving restriction (1) above, but removing restriction (2): that is, can mechanisms with “simple” strategy spaces that employ price discrimination achieve full efficiency? In Section 21.4.2 we present an alternate class of mechanisms, inspired by the VCG class, in which users only submit scalar strategies to the mechanism; we call such mechanisms scalar strategy VCG (SSVCG) mechanisms. We show that these mechanisms have desirable efficiency properties. In particular, we establish existence of an efficient Nash equilibrium, and under an additional condition, we also establish that all Nash equilibria are efficient.

## 21.4.1 VCG Mechanisms

In the VCG class of mechanisms, the basic approach is to let the strategy space of each user r be the set U of possible utility functions, as defined in Assumption 1, and structure the payments made by each user so that the payoff of each user r has the same form as the objective function in SYSTEM, (21.1). As VCG mechanisms have been introduced in Chapter 9, we only use this section to fix notation for our subsequent discussion. For each $r _ { \ast }$ , we use $\tilde { U } _ { r }$ to denote the declared utility function of user $r ,$ and use $\tilde { \mathbf { U } } = ( \tilde { U } _ { 1 } , \dots , \tilde { U } _ { R } )$ to denote the vector of declared utilities.

Suppose that user r receives an allocation $d _ { r }$ , but has to make a payment $t _ { r } ;$ ; we use the notation $t _ { r }$ to distinguish from the bid $w _ { r }$ of Section 21.2. Then the payoff to user $r$ is

$$
U _ {r} (d _ {r}) - t _ {r}.
$$

On the other hand, the social objective (21.1) can be written as

$$
U _ {r} (d _ {r}) + \sum_ {s \neq r} U _ {s} (d _ {s}).
$$

Given a vector of declared utility functions $\tilde { \mathbf { U } }$ , a VCG mechanism chooses the allocation $\mathbf { d } ( \tilde { \mathbf { U } } )$ as an optimal solution to SYSTEM for the declared utility functions $\tilde { \mathbf { U } }$ . For simplicity, let $\begin{array} { r } { \mathcal { X } = \{ { \bf d } \ge 0 : \sum _ { r } d _ { r } \le C \} } \end{array}$ ; this is the feasible region for SYSTEM. Then for a VCG mechanism, we have

$$
\mathbf {d} (\tilde {\mathbf {U}}) \in \arg \max _ {\mathbf {d} \in \mathcal {X}} \sum_ {r} \tilde {U} _ {r} (d _ {r}).\tag{21.39}
$$

The payments are structured so that

$$
t _ {r} (\tilde {\mathbf {U}}) = - \sum_ {s \neq r} \tilde {U} _ {s} (d _ {s} (\tilde {\mathbf {U}})) + h _ {r} (\tilde {\mathbf {U}} _ {- r}).\tag{21.40}
$$

Here $h _ { r }$ is an arbitrary function of the declared utilities of users other than r. In general, we note that mechanisms of this form do not use a single price to clear the market; i.e., the per-unit price paid by user $r , t _ { r } ( \tilde { \mathbf { U } } ) / d _ { r } ( \tilde { \mathbf { U } } )$ , will not be the same for all users. (See also Exercise 21.3.)

For our purposes, the interesting feature of the VCG mechanism is that there exists a dominant strategy equilibrium that elicits the true utility functions from the users, and in turn (because of the definition of $\mathbf { d } ( \tilde { \mathbf { U } } ) )$ ) chooses an efficient allocation. (See Chapter 9 for a formal statement of these results, where it is shown that the VCG mechanism is incentive compatible.) In the next section, we explore a class of mechanisms inspired by the VCG mechanisms, but with limited communication requirements.

## 21.4.2 Scalar Strategy VCG Mechanisms

We now consider a class of mechanisms where each user’s strategy is a submitted utility function (as in the VCG mechanisms) except that users are allowed only to choose from a given single parameter family of utility functions. One cannot expect such mechanisms to have efficient dominant strategy equilibria, and we will focus instead on the efficiency properties of the resulting Nash equilibria.

Formally, scalar strategy VCG (SSVCG) mechanisms allow users to choose from a given family of utility functions $\overline { { U } } ( \cdot ; \theta )$ , parameterized by $\theta \in ( 0 , \infty ) . ^ { 5 }$ We make the following assumptions about this family.

## Assumption 2:

(i) For every $\theta > 0$ , the function $\overline { { U } } ( \cdot ; \theta ) : d \mapsto \overline { { U } } ( d ; \theta )$ belongs to $\mathcal { U } \left( \mathrm { i . e . } \right.$ ., it is concave, strictly increasing, continuous, and differentiable), and is also strictly concave.

(ii) For every $\gamma \in ( 0 , \infty )$ and $d \geq 0$ , there exists a $\theta > 0$ such that $\overline { { U } } ^ { \prime } ( d ; \theta ) = \gamma . ^ { 6 }$

Given $\pmb \theta _ { : }$ , the mechanism chooses $\mathbf { d } ( \pmb \theta )$ such that

$$
\mathbf {d} (\boldsymbol {\theta}) = \arg \max _ {\mathbf {d} \in \mathcal {X}} \sum_ {r} \overline {{U}} (d _ {r}; \theta_ {r}).\tag{21.41}
$$

Since $\overline { { U } } \left( \cdot ; \theta _ { r } \right)$ is strictly concave for each r, the solution $\mathbf { d } ( \pmb \theta )$ is uniquely defined. (Note the similarity between (21.39) and (21.41).)

By analogy with the expression (21.40), the monetary payment by user r is

$$
t _ {r} (\boldsymbol {\theta}) = - \sum_ {s \neq r} \overline {{{U}}} (d _ {s} (\boldsymbol {\theta}); \theta_ {s}) + h _ {r} (\boldsymbol {\theta} _ {- r}).\tag{21.42}
$$

Here $h _ { r }$ is a function that depends only on the strategies $\pmb { \theta } _ { - r } = ( \theta _ { s } , s \neq r )$ submitted by the users other than r. While we do not advocate any particular choice of $h _ { r }$ , a natural candidate is to define $\begin{array} { r } { h _ { r } ( \pmb { \theta } _ { - r } ) = \sum _ { s \neq r } \overline { { U } } ( d _ { s } ( \pmb { \theta } _ { - r } ) ; \theta _ { s } ) } \end{array}$ , where $v d ( \pmb \theta _ { - r } )$ is the aggregate utility maximizing allocation excluding user $r .$ . This leads to a natural scalar strategy analogue of the Clarke pivot mechanism (cf. Chapter 9).

Given $h _ { r }$ , the payoff to user r is

$$
P _ {r} (d _ {r} (\boldsymbol {\theta}), t _ {r} (\boldsymbol {\theta})) = U _ {r} (d _ {r} (\boldsymbol {\theta})) + \sum_ {s \neq r} \overline {{U}} (d _ {s} (\boldsymbol {\theta}); \theta_ {s}) - h _ {r} (\boldsymbol {\theta} _ {- r}).
$$

A strategy vector $\pmb \theta$ is a Nash equilibrium if no user can profitably deviate through a unilateral deviation, i.e., if for all users r there holds:

$$
P _ {r} (d _ {r} (\boldsymbol {\theta}), t _ {r} (\boldsymbol {\theta})) \geq P _ {r} (d _ {r} (\theta_ {r} ^ {\prime}, \boldsymbol {\theta} _ {- r}), t _ {r} (\theta_ {r} ^ {\prime}, \boldsymbol {\theta} _ {- r})), \text { for   all } \theta_ {r} ^ {\prime} > 0.\tag{21.43}
$$

We start with the following key lemma, proven using an argument analogous to the proof that truthtelling is a dominant strategy equilibrium of the VCG mechanism (see Chapter 9).

Lemma 21.11 Then the vector $\pmb \theta$ is aNash equilibrium ofthe SSVCG mechanism $i f$ and only iffor all $r$ :

$$
\mathbf {d} (\boldsymbol {\theta}) \in \arg \max _ {\mathbf {d} \in \mathcal {X}} \left[ U _ {r} (d _ {r}) + \sum_ {s \neq r} \overline {{U}} (d _ {s}; \theta_ {s}) \right].\tag{21.44}
$$

proof Fix a user $r .$ . Since $\theta _ { r }$ does not affect $h _ { r }$ , from (21.43) user r will choose $\theta _ { r }$ to maximize the following effective payoff:

$$
U _ {r} (d _ {r} (\boldsymbol {\theta})) + \sum_ {s \neq r} \overline {{{{U}}}} (d _ {s} (\boldsymbol {\theta}); \theta_ {s}).\tag{21.45}
$$

The optimal value of the objective function in (21.44) is certainly an upper bound to user $r { } _ { \mathrm { ~ s ~ } } ^ { \prime }$ effective payoff (21.45). Thus, given a vector $\pmb \theta ,$ , if (21.44) is satisfied for all users r, then (21.43) holds for all users $r ,$ , and we conclude $\pmb \theta$ is a Nash equilibrium.

Conversely, given a vector $\pmb \theta _ { : }$ , suppose that (21.44) is not satisfied for some user $r .$ . We will show $\pmb \theta$ cannot be a Nash equilibrium. Since $\mathcal { X }$ is compact, an optimal solution exists to the problem in (21.44) for user $r ;$ call this optimal solution $\mathbf { d } ^ { * }$ The vector $\mathbf { d } ^ { * }$ must satisfy the first-order optimality conditions (21.8)–(21.10), which only involve the first derivatives $U _ { r } ^ { \prime } ( d _ { r } ^ { * } )$ and $( \overline { { U } } ^ { \prime } ( d _ { s } ^ { \ast } ; \theta _ { s } ) , s \neq r )$ . Suppose now that user r chooses $\theta _ { r } ^ { \prime } > 0$ such that $\overline { { U } } ^ { \prime } ( d _ { r } ^ { * } ; \theta _ { r } ^ { \prime } ) = U _ { r } ^ { \prime } ( d _ { r } ^ { * } )$ . Then, $\mathbf { d } ^ { * }$ also satisfies the optimality conditions for the problem (21.41). Since $\mathbf { d } ( \theta _ { r } ^ { \prime } , \pmb { \theta } _ { - r } )$ is the unique optimal solution to (21.41) when the strategy vector is $( \theta _ { r } ^ { \prime } , \pmb \theta _ { - r } )$ , we must have $\mathbf { d } ( \theta _ { r } ^ { \prime } , \pmb { \theta } _ { - r } ) = \mathbf { d } ^ { * }$ . Thus we have

$$
\begin{array}{l} P _ {r} (d _ {r} (\boldsymbol {\theta}), t _ {r} (\boldsymbol {\theta})) <   U _ {r} (d _ {r} ^ {*}) + \sum_ {s \neq r} \overline {{U}} (d _ {s} ^ {*}; \theta_ {s}) + h _ {r} (\boldsymbol {\theta} _ {- r}) \\ \qquad = U _ {r} (d _ {r} (\theta_ {r} ^ {\prime}, \boldsymbol {\theta} _ {- r})) + \sum_ {s \neq r} \overline {{U}} (d _ {s} (\theta_ {r} ^ {\prime}, \boldsymbol {\theta} _ {- r}); \theta_ {s}) + h _ {r} (\boldsymbol {\theta} _ {- r}) \\ \qquad = P _ {r} (d _ {r} (\theta_ {r} ^ {\prime}, \boldsymbol {\theta} _ {- r}), t _ {r} (\theta_ {r} ^ {\prime}, \boldsymbol {\theta} _ {- r})). \end{array}
$$

(The first inequality follows by the assumption that (21.44) is not satisfied for user $r . )$ We conclude that (21.43) is violated for user r, so $\pmb \theta$ is not a Nash equilibrium.

The following corollary states that there exists a Nash equilibrium which is efficient Furthermore, at this efficient Nash equilibrium, all users truthfully reveal their utilities in a local sense: each user r chooses $\theta _ { r }$ so that the declared marginal utility $\overline { { U } } ^ { \prime } ( d _ { r } ( \pmb { \theta } ) ; \theta _ { r } )$ is equal to the true marginal utility $U _ { r } ^ { \prime } ( d _ { r } ( \pmb { \theta } ) )$ .

Corollary 21.12 For any SSVCG mechanism, there exists an efficient Nash equilibrium θ defined asfollows: $L e t { \bf d } ^ { S }$ be an optimal solution to SYSTEM. Each user r chooses $\theta _ { r }$ so that $\overline { { U } } ^ { \prime } ( d _ { r } ^ { S } ; \theta _ { r } ) = U _ { r } ^ { \prime } ( d _ { r } ^ { S } )$ . The resulting allocation satisfies $\mathbf { d } ( \pmb \theta ) = \mathbf { d } ^ { S }$

proof By Assumption 2, each user r can choose $\theta _ { r }$ so that $\overline { { U } } ^ { \prime } ( d _ { r } ^ { S } ; \theta _ { r } ) =$ $U _ { r } ^ { \prime } ( d _ { r } ^ { S } )$ . For this vector $\pmb \theta .$ , it is clear that $\mathbf { d } ( \pmb \theta ) = \mathbf { d } ^ { S }$ , since the optimal solution to (21.41) is uniquely determined, and the optimality conditions for (21.41) involve only the first derivatives $\overline { { U } } ^ { \prime } ( d _ { r } ( \pmb { \theta } ) ; \theta _ { r } )$ . By the same argument it also follows that ${ \bf d } ^ { S }$ is an optimal solution in (21.44). Since $\mathbf { d } ( \pmb \theta ) = \mathbf { d } ^ { S }$ , we conclude that (21.44) is satisfied for all $r ,$ and thus $\pmb \theta$ is a Nash equilibrium.

We note that, as in classical VCG mechanisms, there can be additional, possibly inefficient, Nash equilibria, as the following example shows.

Example 21.13 Consider a system with R identical users with strictly concave utility function $U .$ . Suppose that user 1 chooses $\theta _ { 1 }$ so that $\overline { { U } } ^ { \prime } ( C ; \theta _ { 1 } ) > U ^ { \prime } ( 0 )$ , and every other user r chooses $\theta _ { r }$ so that $\overline { { U } } ^ { \prime } ( 0 ; \theta _ { r } ) < U ^ { \prime } ( C )$ ). Since $U ^ { \prime } ( C ) \leq U ^ { \prime } ( 0 )$ , it follows that (21.44) is satisfied for all users r. Thus this is a Nash equilibrium where the entire resource is allocated to user 1; however, the unique optimal solution to SYSTEM is symmetric, and allocates $C / R$ units of the resource to each of the R users.

The equilibrium in the preceding example involves a “bluff”: user 1 declares such a high marginal utility at $C$ that all other users concede. One way to preclude such equi libria is to enforce an assumption that guarantees participation. The next proposition assumes that all users have infinite marginal utility at zero allocation; this guarantees that all Nash equilibria are efficient.

Proposition 21.14 Suppose that $U _ { r } ^ { \prime } ( 0 ) = \infty$ for all r. Suppose that θ is a Nash equilibrium. Then $\mathbf { d } ( \pmb \theta )$ is an optimal solution to SYSTEM.

proof Let $\mathbf { d } = \mathbf { d } ( \pmb \theta )$ . The proof follows by noting that all users must have positive allocations at equilibrium if $U _ { r } ^ { \prime } ( 0 ) = \infty$ , from (21.44). Thus at equilib rium, for all users $r , s$ we have $U _ { r } ^ { \prime } ( d _ { r } ) = \overline { { U } } ^ { \prime } ( d _ { s } ; \theta _ { s } )$ . But this in turn implies that $U _ { r } ^ { \prime } ( d _ { r } ) = U _ { s } ^ { \prime } ( d _ { s } )$ for all $r , s .$ , a sufficient condition for optimality for the problem SYSTEM.

Intuitively, for efficiency to hold, we need to have a number of actively “competing” users. In the previous result, this is guaranteed because every user will want strictly positive rate at any equilibrium.

The results of this section demonstrate that by relaxing the assumption that the resource allocation mechanism must set a single price, we can in fact significantly improve upon the efficiency guarantee of Theorem 21.10. It is critical to note that this gain in efficiency occurs only at Nash equilibria. The classical VCG mechanisms are unique in that they guarantee efficient outcomes as dominant strategy equilibria; it is straightforward to check that the SSVCG mechanisms described in this section will not have dominant strategy equilibria in general—e.g., the “bluff” example above is one such case.

## 21.5 Chapter Summary and Further Directions

This chapter considered the allocation of a single resource of fixed supply among multiple strategic users. We evaluated a variety of market mechanisms through Nash equilibria of the resulting resource allocation game. Our key insights are the following:

(i) A simple proportional allocation mechanism, where each user receives a share of the resource in proportion to their bid, ensures full efficiency when users are price takers, and exhibits no worse than a 25% efficiency loss when users are price anticipators.

(ii) In a natural class of mechanisms where users choose one-dimensional strategies, and the market sets a single price, the proportional allocation mechanism minimizes the worst-case efficiency loss when users are price anticipating; i.e., the best possible guarantee here is 75% of maximal aggregate utility.

(iii) This guarantee can be improved if the mechanism is allowed to set one price per user. Using an adapted version of the VCG class of mechanisms, we can construct mechanisms that ensure fully efficient Nash equilibria.

Our investigation also reveals several further directions open for future research, including the following:

(i) For the proportional allocation mechanism, we have proven a bound on the price of anarchy that shows that the ratio of the Nash equilibrium aggregate utility is no worse than 3/4 the maximum possible aggregate utility. For nonatomic selfish routing (cf. Chapter 18), a similar price of anarchy result holds: the ratio of Nash cost to the optimal cost is no worse than 4/3; furthermore, both proofs use the characterization of Nash equilibria as solutions to an optimization problem, with structure similar to the respective efficient optimization problems. These results are suggestive of perhaps a deeper generalization of price of anarchy for games with equilibria characterized as the solution to optimization problems.

(ii) While Theorem 21.10 proves optimality of the proportional allocation mechanism in a reasonable class of mechanisms, the result depends critically on the assumption that all mechanisms in D yield concave payoffs when agents are price anticipating. Given that some type of quasiconcavity assumption is typically necessary on payoffs to even guarantee existence of Nash equilibria, one might informally expect the result of Theorem 21.10 to hold even if Condition 2 is removed in the definition of D. Whether this is in fact possible remains an open question.

(iii) Our investigation shows, under reasonable assumptions, that with a single market clearing price a 75% efficiency guarantee is possible, while with one price per user (the scalar strategy VCG approach), full efficiency is possible. This warrants further investigation: what is the exact trade-off between the number of prices and the efficiency guarantee possible? Furthermore, how does increasing the dimensionality of users’ strategy affect this efficiency guarantee?

## 21.6 Notes

## 21.6.1 Section 20.2

Much of the material in this section is based on Chapter 2 of Johari (2004) and the corresponding paper (Johari and Tsitsiklis, 2004).

The mechanism discussed here was first studied in the context of communication networks by Kelly (1997). (See Chapter 22 for a discussion of the proportional allocation mechanism in congestion control algorithms for communication networks.) Theorem 21.1 is adapted from Kelly (1997), where it is proven in greater generality for an extension of the proportional allocation mechanism to a network context. This theorem is an extension of the classicalfirstfundamental theorem ofwelfare economics; see Mas-Colell et al. (1995, Chapter 16), for details.

The first proof of uniqueness of Nash equilibrium for the proportional allocation mechanism was provided by La and Anantharam (2000). The most general result of existence and uniqueness, and the basis for the result in Theorem 21.2, is due to Hajek and Gopalakrishnan (2002); a less general result was proven by Maheswaran and Basar (2003). The explicit formulation of the problem GAME is given by Johari and Tsitsiklis (2004).

The price of anarchy result of Theorem 21.4 is due to Johari and Tsitsiklis (2004). The original proof of this result uses a two-step approach: it is first shown that the worst case is achieved using linear utility functions, and then the efficiency loss calculation is solved directly as a mathematical programming problem. The proof based on the problem GAME presented here is due to Roughgarden (2006), who also successfully applies the same method to efficiency loss calculations in several other games.

## 21.6.2 Section 20.3

Much of the material in this section is based on Chapter 5 of Johari (2004) and Section 4 of Johari and Tsitsiklis (2007).

The most closely related result to this section is presented by Maheswaran and Basar (2004). In their result, they consider mechanisms where each user r chooses a bid $w _ { r }$ and the allocation is still made proportional to each player’s bid. However, rather than assuming that every player pays $w _ { r }$ as in the standard proportional allocation mechanism, Maheswaran and Basar consider a class of mechanisms where the user pays $c ( w _ { r } )$ , where c is a convex function. They show that in this class of mechanisms, the proportional allocation mechanism (i.e., a linear c) achieves the minimal worst-case efficiency loss when users are price anticipating.

Our work is substantially different, because we do not postulate that the mechanism must use the proportional rule (21.29) in allocating the resource; rather, this emerges as a consequence of rather simple assumptions on our mechanisms. We note that other works on inefficiency of resource allocation mechanisms, including Maheswaran and Basar (2004) and Yang and Hajek (2004), also assume a priori that allocations are made in proportion to users’ bids.<sup>7</sup> In this sense, our result lends a rigorous foundation to the intuition that the proportional allocation rule (21.29) is a natural choice to determine the allocation among users.

## 21.6.3 Section 20.4

This section is based on Section 5.2 of the paper by Johari and Tsitsiklis (2007). Simultaneously and independently, a nearly identical formulation was developed by Yang and Hajek (2007). It is worth noting that Yang and Hajek and Maheswaran and Basar had earlier presented a resource allocation mechanism where users receive an allocation in proportion to their bids, but prices are chosen on an individualized basis (Maheswaran and Basar, 2004; Yang and Hajek, 2004); this mechanism can be seen to be a special case of the SSVCG mechanisms (Johari and Tsitsiklis, 2007).

Subsequent to the above work, several papers have presented related constructions of mechanisms that use limited communication yet achieve fully efficient Nash equilibria. Building on earlier work by Semret (1999), Dimakis et al. establish that a VCG-like mechanism where agents submit a pair (price and quantity requested) can achieve fully efficient equilibrium for a related resource allocation game (Dimakis et al., 2006). Stoenescu and Ledyard consider the problem of resource allocation by building on the notion of minimal message spaces addressed in earlier literature on mechanism design, and build a class of efficient mechanisms with scalar strategy spaces (Stoenescu and Ledyard, 2006).

The latter work of Stoenescu and Ledyard recalls perhaps the most related reference (and most seminal) in this area by Reiter and Reichelstein (1988). Their paper calculates the minimal dimension of strategy space that would be necessary to achieve fully efficient Nash equilibria for a general class of economic models known as exchange economies. For our model, their bound evaluates to a strategy space per user of dimen sion $1 + 2 / ( R ( R - 1 ) )$ ), where R denotes the number of users. This is slightly higher than our result because Reiter and Reichelstein consider a much more general resource allocation problem.

## Bibliography

A. Dimakis, R. Jain, and J. Walrand. Mechanisms for efficient allocation in divisible capacity net works. Proceedings ofIEEE CDC, pp. 1264–1269, 2006.

B. Hajek and G. Gopalakrishnan. Do greedy autonomous systems make for a sensible Internet? Presented at the Conf. Stochastic Networks, Stanford University, 2002.

R. Johari. Efficiency Loss in Market Mechanismsfor Resource Allocation. PhD thesis, Massachusett Institute of Technology, 2004.

R. Johari and J.N. Tsitsiklis. Efficiency loss in a network resource allocation game. Math. Operat. Res., 29(3):407–435, 2004.

R. Johari and J.N. Tsitsiklis. Efficiency of scalar-parameterized mechanisms. Management Science and Engineering Working Paper 07-04-6126-34, Stanford University, 2007.

F.P. Kelly. Charging and rate control for elastic traffic. Euro. Trans. Telecommun., 8:33–37, 1997.

R.J. La and V. Anantharam. Charge-sensitive TCP and rate control in the Internet. In Proc. IEEE INFOCOM, pp. 1166–1175, 2000.

R.T. Maheswaran and T. Basar. Nash equilibrium and decentralized negotiation in auctioning divisible Group Decis. Ne otiation

R.T. Maheswaran and T. Basar. Social welfare of selfish agents: motivating efficiency for divisible resources. In Proc. IEEE CDC, pp. 1550–1555, 2004.

A. Mas-Colell, M.D. Whinston, and J.R. Green. Microeconomic Theory. Oxford University Press, Oxford, UK, 1995.

S. Reichelstein and S. Reiter. Game forms with minimal message spaces. Econometrica, 56(3):661– 692, 1988.

T. Roughgarden. Potential functions and the inefficiency of equilibria. In Proc. Intl. Congress of Mathematicians, Vol. III, pp. 1071–1094, 2006.

S. Sanghavi and B. Hajek. Optimal allocation of a divisible good to strategic buyers. In Proc. IEEE CDC, pp. 2748–2753, 2004.

N. Semret. Market Mechanisms for Network Resource Sharing. PhD thesis, Columbia University, 1999.

T.M. Stoenescu and J. Ledyard. A pricing mechanism which implements a network rate allocation problem in Nash equilibria. 2006. Submitted.

S. Yang and B. Hajek. An efficient mechanism for allocation of a divisible good and its application to network resource allocation. 2004. Preprint.

S. Yang and B. Hajek. VCG-Kelly mechanisms for divisible goods: adapting VCG mechanisms to one-dimensional signals. To appear in IEEE Journal on Selected Areas in Communications, 2007.

## Exercises

21.1 This exercise, together with the next one, studies the efficiency loss properties of the mechanisms defined in Example 21.9, by following the proof of Theorem 21.4. Suppose that $D ( p , \theta ) = \theta p ^ { - 1 / c }$ , where $c \geq 1$ . Suppose that given a utility system (C, R, U), a bid vector θ is a Nash equilibrium, and let the resulting allocation vector be d; i.e., $d _ { r } = D ( p _ { D } ( \pmb { \theta } ) , \theta _ { r } )$ ).

(a) Verify the Nash equilibrium conditions (21.31)–(21.32).

(b) Show that d is the unique solution to GAME, but where $\hat { U } _ { r }$ is defined as follows for each r:

$$
\hat {U} _ {r} (d _ {r}) = \int_ {0} ^ {d _ {r}} \left(\frac {1 - z / C}{1 + (c - 1) (z / C)}\right) U _ {r} ^ {\prime} (z) d z.\tag{E1.1}
$$

(Hint: rearrange the Nash equilibrium conditions (21.31)—(21.32).)

(c) Show that $\hat { U } _ { r }$ satisfies Assumption 1.

21.2 Fix $D ( p , \theta ) = \theta p ^ { - 1 / c }$ and define $\hat { U }$ as in the previous exercise. Define $\beta ( D )$ according to (21.24), i.e.,

$$
\beta (D) = \inf _ {U \in \mathcal {U}} \inf _ {C > 0} \inf _ {0 \leq d, \overline {{d}} \leq C} \frac {U (d) + \hat {U} ^ {\prime} (d) (\overline {{d}} - d)}{U (\overline {{d}})}.
$$

(a) Show that $\rho ( D ) \ge \beta ( D )$ ). (Hint: first construct the variational inequality that identifies the optimality conditions for GAME, then argue as in the proof of Theorem 21.4.)

(b) Show that $\beta ( D ) \ge C ( c )$

(c) Using a construction analogous to the proof of Theorem 21.4, show that for any δ there exists a utility system for which the ratio of Nash aggregate utility to the maximum aggregate utility is no more than $C ( c ) + \delta .$ . Conclude that $\rho ( D ) = G ( C )$

21.3 Show by example that a VCG mechanism does not necessarily charge each user the same per-unit price for the resource.