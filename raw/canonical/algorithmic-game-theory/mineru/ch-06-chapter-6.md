---
type: "book-chapter"
book_id: "algorithmic-game-theory"
chapter_id: "ch-06"
chapter_number: 6
chapter_title: "Chapter 6"
source_pdf: "raw/inbox/manual-drop/PDF_B.pdf"
source_page_start: 156
source_page_end: 179
printed_page_start: 156
printed_page_end: 179
part_ids: ["algorithmic-game-theory-ch-06-part-007"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Chapter 6 (MinerU semantic layer)

<!-- source-pages: 156-179; printed-pages: 156-179; mineru-part: algorithmic-game-theory-ch-06-part-007 -->

# Computation of Market Equilibria by Convex Programming

# Bruno Codenotti and Kasturi Varadarajan

## Abstract

We introduce convex programming techniques to compute market equilibria in general equilibrium models. We show that this approach provides an effective arsenal of tools for several restricted, yet important, classes of markets. We also point out its intrinsic limitations.

## 6.1 Introduction

The market equilibrium problem consists of finding a set of prices and allocations of goods to economic agents such that each agent maximizes her utility, subject to her budget constraints, and the market clears. Since the nineteenth century, economists have introduced models that capture the notion of market equilibrium. In 1874, Walras published the “Elements ofPure Economics,” in which he describes a model for the state of an economic system in terms of demand and supply, and expresses the supply equal demand equilibrium conditions (Walras, 1954). In 1936, Wald gave the first proof of the existence of an equilibrium for the Walrasian system, albeit under severe restrictions (Wald, 1951). In 1954, Nobel laureates Arrow and Debreu proved the existence of an equilibrium under much milder assumptions (Arrow and Debreu, 1954).

The market equilibrium problem can be stated as a fixed point problem, and indeed the proofs of existence of a market equilibrium are based on either Brouwer’s or Kakutani’s fixed point theorem, depending on the setting (see, e.g., the beautiful monograph (Border, 1985) for a friendly exposition of the main results in this vein).

Under a capitalistic economic system, the prices and production of all goods are interrelated, so that the equilibrium price of one good may depend on all the different markets of goods that are available. Equilibrium models must therefore take into account a multitude ofdifferent markets ofgoods. This intrinsic large-scale nature ofthe problem calls for algorithmic investigations and shows the central role of computation.

Starting from the 60’s, the intimate connection between the notions offixed-point and market equilibrium was exploited for computational goals by Scarf and some coauthors, who employed path-following techniques to compute approximate equilibrium prices (Eaves and Scarf, 1976; Hansen and Scarf, 1973; Scarf, 1967, 1982). In their simplest form these methods are based upon a decomposition of the price simplex into a large number of small regions and on the use of information about the problem instance to construct a path that can be shown to terminate close to a fixed point. While the appropriate termination is guaranteed by the fixpoint theorems, the worst case running time of these algorithms turns out to be exponential.

Over the last few years, the problem of computing market equilibria has received significant attention within the theoretical computer science community. Inspired by Papadimitriou (2001), and starting with the work of Deng, Papadim itriou, and Safra (2003), theoretical computer scientists have developed polynomial time algorithms for several restricted versions of the market equilibrium problem.

In this chapter we focus on algorithms based on convex programming techniques. Elsewhere in this book (Vazirani, 2007), algorithms of a combinatorial nature are presented.

## 6.1.1 Definitions: Models and Equilibrium

We start by describing a model of the so-called exchange economy, an important special case of the model considered by Arrow and Debreu (1954). The more general one, which we will call the Arrow-Debreu model, includes the production of goods. We will deal with models with production in Section 6.6.

Let us consider m economic agents that represent traders of n goods. Let $\mathbf { R } _ { + } ^ { n }$ denote the subset of $\mathbf { R } ^ { n }$ with all nonnegative coordinates. The -th coordinate in $\mathbf { R } ^ { n }$ will stand for good j. Each trader i has a concave utility function $u _ { i } : \mathbf { R } _ { + } ^ { n }  \mathbf { R } _ { + }$ , which represents her preferences for the different bundles of goods, and an initial endowment of goods $w _ { i } = ( w _ { i 1 } , \ldots , w _ { i n } ) \in \mathbf { R } _ { + } ^ { n }$ . We make the standard assumption that $u _ { i }$ is nonsatiable, that is, for any $x \in \mathbf { R } _ { + } ^ { n }$ , there is a $y \in \mathbf { R } _ { + } ^ { n }$ such that $u _ { i } ( y ) > u _ { i } ( x )$ ). We also assume that $u _ { i }$ is monotone, that is, $u _ { i } ( y ) \geq u _ { i } ( x ) { \mathrm { ~ i f ~ } } y \geq x$ . For the initial endowment of trader i, we assume that $w _ { i j } > 0$ for at least one j. At given prices $\pi \in \mathbf { R } _ { + } ^ { n }$ , trader i will sell her endowment, and ask for the bundle of goods $x _ { i } = ( x _ { i 1 } , \ldots , x _ { i n } ) \in \mathbf { R } _ { + } ^ { n }$ which maximizes $u _ { i } ( x )$ subject to the budget constraint<sup>1</sup> $\pi \cdot x \leq \pi \cdot w _ { i }$ . The budget constraint simply says that the bundles of goods that are available to trader i are the ones that cost no more than her income $\pi \cdot w _ { i }$

An equilibrium is a vector of prices $\pi = ( \pi _ { 1 } , \ldots , \pi _ { n } ) \in \mathbf { R } _ { + } ^ { n }$ at which, for each trader i, there is a bundle $\bar { x } _ { i } = ( \bar { x } _ { i 1 } , \ldots , \bar { x } _ { i n } ) \in \mathbf { R } _ { + } ^ { n }$ of goods such that the following two conditions hold:

(i) For each trader i, the vector ${ \bar { x } } _ { i }$ maximizes $u _ { i } ( x )$ subject to the constraints π $\cdot x \leq \pi \cdot w _ { i }$ and $\boldsymbol { x } \in \mathbf { R } _ { + } ^ { n }$

(ii) For each good $j , \sum _ { i } \bar { x } _ { i j } \le \sum _ { i } w _ { i j }$

Let $\mathbf { R } _ { + + } ^ { n }$ be the set of vectors in $R ^ { n }$ , whose components are strictly positive. For purposes of exposition, we will generally restrict our attention to price vectors in $\mathbf { R } _ { + + } ^ { n }$ When we violate this convention, we will be explicit about it.

For any price vector π, a vector $x _ { i } ( \pi )$ , which maximizes $u _ { i } ( x )$ subject to the budge constraint $\pi \cdot x \leq \pi \cdot w _ { i }$ and $x \in \mathbf { R } _ { + } ^ { n }$ , is called a demand of trader i at prices π. Observe that there is at least one demand vector, and that there can be multiple demand vectors. We will usually assume that there is exactly one demand vector at price π; that is, we have a demand function. This assumption holds if the utility function satisfies a condition known as strict quasi-concavity. Once again, we will be explicit when we will deal with exceptions, since for some common utility functions such as the linear ones, the demand is not a function but a correspondence or a set valued function.

The vector $z _ { i } ( \pi ) = x _ { i } ( \pi ) - w _ { i }$ is called the individual excess demand of trader i. Then $\begin{array} { r } { X ^ { k } ( \pi ) = \sum _ { i } x _ { i k } ( \pi ) } \end{array}$ denotes the market demand of good k at prices $\pi$ , and $\begin{array} { r } { Z ^ { k } ( \pi ) = X ^ { k } ( \pi ) - \sum _ { i } w _ { i k } } \end{array}$ the market excess demand of good k at prices $\pi$ . The vectors $X ( \pi ) = ( X ^ { 1 } ( \pi ) , \ldots , X ^ { n } ( \pi ) )$ ) and $Z ( \pi ) = ( Z ^ { 1 } ( \pi ) , \ldots , Z ^ { n } ( \pi ) )$ are called market demand (or aggregate demand) and market excess demand, respectively. Observe that the economy satisfies positive homogeneity, i.e., for any price vector π and any $\lambda > 0$ we have $Z ( \pi ) = Z ( \lambda \pi )$ . The assumptions on the utility functions imply that for any price π, we have $\pi \cdot x _ { i } ( \pi ) = \pi \cdot w _ { i }$ . Thus the economy satisfies Walras’ Law: for any price π, we have $\pi \cdot Z ( \pi ) = 0 $

In terms of the aggregate excess demand function, the equilibrium can be equivalently defined as a vector of prices $\pi = ( \pi _ { 1 } , \ldots , \pi _ { n } ) \in \mathbf { R } _ { + } ^ { n }$ such that $Z ^ { j } ( \pi ) \leq 0$ for each $j$ .

## 6.1.2 The Tatonnement Processˆ

The model of an economy and the definition of the market equilibrium fail to predict any kind of dynamics leading to an equilibrium, although they convey the intuition that, in any process leading to a stable state where demand equals supply, a disequilibrium price of a good will have to increase if the demand for such a good exceeds its supply, and vice versa.

Walras (1954) introduced a price-adjustment mechanism, which he called taton-ˆ nement. He took inspiration from the workings of the stock-exchange in Paris, and suggested a trial-and-error process run by a fictitious auctioneer. The economic agents receive a price signal, and report their demands at these prices to the auctioneer. The auctioneer then adjusts the prices in proportion to the magnitude of the aggregate de mands, and announces the new prices. In each round, agents recalculate their demands upon receiving the newly adjusted price signal and report these new demands to the auctioneer. The process continues until prices converge to an equilibrium. In its contin uous version, as formalized by Samuelson (1947), the tatonnement process is governedˆ by the differential equation system:

$$
\frac {d \pi_ {k}}{d t} = G _ {k} (Z _ {k} (\pi)), k = 1, 2, \ldots , n,\tag{6.1}
$$

where $G _ { k } ( )$ denotes some continuous and differentiable, sign-preserving function, and $Z _ { k } ( )$ is the market excess demand function for good k.

## 6.1.3 Approximate Equilibria

Since a price equilibrium vector that is rational exists only in very special cases, most algorithms actually compute an approximate equilibrium.

Definition 6.1 A bundle $x _ { i } \in \mathbf { R } _ { + } ^ { n }$ is a µ-approximate demand, for $\mu \geq 1$ of trader i at prices π if $\begin{array} { r } { u _ { i } ( x _ { i } ) \geq \frac { 1 } { \mu } u ^ { * } } \end{array}$ and $\pi \cdot x _ { i } \leq \mu \pi \cdot w _ { i }$ , where $u ^ { * } =$ max $\{ u _ { i } ( x ) | x \in \mathbf { R } _ { + } ^ { n } , \pi \cdot x \leq \pi \cdot w _ { i } \}$

A price vector π is a strong µ-approximate equilibrium $( \mu \geq 1 )$ if there are bundles $x _ { i }$ such that (1) for each trader $i , x _ { i }$ is the demand of trader i at prices π, and $( 2 ) \textstyle \sum _ { i } x _ { i j } \leq$ $\mu \sum _ { i } w _ { i j }$ for each good j. A price vector π is a weak µ-approximate equilibrium $( \mu \geq 1 )$ if there are bundles $x _ { i }$ such that (1) for each trader $i , x _ { i }$ is a µ-approximate demand of trader i at prices π, and $\begin{array} { r } { ( 2 ) \sum _ { i } x _ { i j } \le \mu \sum _ { i } w _ { i j } } \end{array}$ for each good $j$ .

Definition 6.2 An algorithm that computes an approximate equilibrium, for any $\varepsilon > 0$ , in time that is polynomial in the input size and $1 / \varepsilon$ (resp., log $1 / \varepsilon )$ is called polynomial time approximation scheme (resp., polynomial time algorithm).

## 6.1.4 Gross Substitutability

In general, not only equilibria are not unique, but the set of equilibrium points may be disconnected. Yet many real markets do work, and economists have struggled to capture realistic restrictions on markets, where the equilibrium problem exhibits some structure, like uniqueness or convexity. The general approach has been to impose restrictions either at the level of individuals (by restricting the utility functions considered and/or by making assumptions on the initial endowments) or at the level of the aggregate market (by assuming that the composition of the individual actions is particularly well behaved).

The property of gross substitutability (GS) plays a significant role in the theory of equilibrium and in related computational results based on convex programming.

The market excess demand is said to satisfy gross substitutability (resp., weak gross substitutability [WGS]) if for any two sets of prices $\pi$ and $\pi ^ { \prime }$ such that $0 < \pi _ { j } \leq \pi _ { j } ^ { \prime }$ , for each $j ,$ , and $\pi _ { j } < \pi _ { j } ^ { \prime }$ for some $j .$ , we have that $\pi _ { k } = \pi _ { k } ^ { \prime }$ for any good k implies $Z ^ { k } ( \pi ) < Z ^ { k } ( \pi ^ { \prime } )$ (resp., $Z ^ { k } ( \pi ) \leq Z ^ { k } ( \pi ^ { \prime } ) )$ . In words, GS means that increasing the price of some of the goods while keeping some others fixed can only cause an increase in the demand for the goods whose price is fixed.

It is easy to see that WGS implies that the equilibrium prices are unique up to scaling (Varian, 1992, p. 395) and that the market excess demand satisfies WGS when each individual excess demand does.

## 6.1.5 Special Forms of the Utility Functions

A utility function $u ( \cdot )$ is homogeneous (of degree 1) if it satisfies $u ( \alpha x ) = \alpha u ( x )$ , for all $\alpha > 0$

A utility function $u ( \cdot )$ is log-homogeneous if it satisfies $u ( \alpha x ) = \log \alpha + u ( x )$ , for all $\alpha > 0$

Three popular examples of homogeneous utility functions are as follows.

 The linear utility function, which has the form $\begin{array} { r } { u _ { i } ( x ) = \sum _ { j } a _ { i j } x _ { i j } } \end{array}$

 The Cobb–Douglas function, which has the form $\begin{array} { r } { u _ { i } ( x ) = \prod _ { j } ( x _ { i j } ) ^ { a _ { i j } } } \end{array}$ , where $\textstyle \sum _ { j } a _ { i j } = 1$

 The Leontief (or fixed-proportions) utility function, which has the form $u _ { i } ( x ) =$ min<sub>j</sub> $a _ { i j } x _ { i j }$

We now define the constant elasticity of substitution functional form (CES, for short), which is a family of homogeneous utility functions of particular importance in applications. A CES function is a concave function defined as

$$
u (x _ {1}, \ldots , x _ {n}) = \left(\sum_ {i = 1} ^ {n} \alpha_ {i} x _ {i} ^ {\rho}\right) ^ {\frac {1}{\rho}},
$$

where the $\alpha _ { i } \mathrm { ^ { * } s }$ are the utility parameters, and $- \infty < \rho < 1 , \rho \neq 0$ , is a parameter representing the elasticity ofsubstitution $1 / 1 - \rho$ (see Varian, 1992, p. 13).

CES functions have been thoroughly analyzed in Arrow et al. (1961), where it has also been shown how to derive, in the limit, their special cases, i.e., linear, Cobb– Douglas, and Leontief functions (see Arrow et al., 1961, p. 231). For $\rho \to 1$ , CES take the linear form, and the goods are perfect substitutes, so that there is no preference for variety. For $\rho > 0$ , the goods are partial substitutes, and different values of σ in this range allow us to express different levels of preference for variety. For $\rho \to 0$ , CES become Cobb–Douglas functions, and express a perfect balance between substitution and complementarity effects. Indeed it is not difficult to show that a trader with a Cobb–Douglas utility spends a fixed fraction of her income on each good.

For $\rho < 0$ , CES functions model markets with significant complementarity effects between goods. This feature reaches its extreme (perfect complementarity) as $\rho $ $- \infty , \mathrm { i . e . }$ , when CES take the form of Leontief functions.

## 6.1.6 Equilibrium vs Optimization

In 1960, Negishi showed that equilibrium allocations ofgoods for an exchange economy can be determined by solving a convex program where the weights of the function to be maximized are unknown (Negishi, 1960).

Negishi proved the following theorem.

Theorem 6.3 Suppose that the initial endowment of each trader includes a positive amount ofeach good.

Given positive welfare weights $\alpha _ { i } , i = 1 , \ldots , m$ , consider the convex program

$$
\begin{array}{l l} \text { Maximize } & \sum_ {i} \alpha_ {i} u _ {i} (x _ {i}) \\ \text { Subject   to } & \sum_ {i} x _ {i j} \leq \sum_ {i} w _ {i j}, \text { for } 1 \leq j \leq n. \end{array}
$$

There exist $\alpha _ { i } > 0 , i = 1 , . . . , m$ , such that the optimal solutions ${ \bar { x } } _ { i }$ to the program above with these $\alpha _ { i }$ are equilibrium allocations. That is,for some price vector $\pi , \bar { x } _ { i } = x _ { i } ( \pi ) f o r$ each i.

In the proofofNegishi’s theorem, the price vector π for a given set ofwelfare weights $\alpha _ { i }$ is obtained from the dual variables in the Karush–Kuhn–Tucker characterization of the optimal solution to the convex program. Whenever the utility functions are loghomogeneous, the Karush–Kuhn–Tucker characterization implies that $\alpha _ { i }$ is always equal to $\pi \cdot { \bar { x } } _ { i }$ . For the welfare weights that correspond to equilibrium, we must then have $\alpha _ { i } = \pi \cdot w _ { i }$

Negishi’s characterization of the equilibrium has inspired certain algorithmic approaches to compute it (Rutherford, 1999). It is also connected to some recent theoretical computer science work (Jain et al., 2003; Ye, in press).

## 6.1.7 The Fisher Model

A special case of the exchange model occurs when the initial endowments are $p r o \AA { - }$ portional; i.e., when $w _ { i } = \delta _ { i } w , \delta _ { i } > 0$ , so that the relative incomes of the traders are independent of the prices. This special case is equivalent to Fisher model, which is a market of n goods desired by m utility maximizing buyers with fixed incomes. In the standard account of Fisher model, each buyer has a concave utility function $u _ { i } : \mathbf { R } _ { + } ^ { n }  \mathbf { R } _ { + }$ and an endowment $e _ { i } > 0$ of money. There is a seller with an amount $q _ { j } > 0$ of good j. An equilibrium in this setting is a nonnegative vector of prices $\pi = ( \pi _ { 1 } , \ldots , \pi _ { n } ) \in \mathbf { R } _ { + } ^ { G }$ at which there is a bundle $\bar { x } _ { i } = ( x _ { i 1 } , \ldots , x _ { i n } ) \in \mathbf { R } _ { + } ^ { G }$ of goods for each trader i such that the following two conditions hold:

(i) The vector x¯ maximizes $u _ { i } ( x )$ subject to the constraints $\pi \cdot x \leq e _ { i }$ and $x \in \mathbf { R } _ { + } ^ { n }$

(ii) For each good $j , \sum _ { i } \bar { x } _ { i j } = q _ { j }$

## 6.1.8 Overview

The rest of this chapter is organized as follows.

In Section 6.2, we analyze the Fisher model under the assumption that the traders are endowed with homogeneous utility functions, and present Eisenberg’s convex program for computing an equilibrium in such models.

In Section 6.3, we consider exchange economies that satisfy weak gross substi tutability, and show that, under such conditions, an important inequality holds, which implicitly gives a convex feasibility formulation for the equilibrium. We discuss algo rithmic work that exploits this formulation.

In Section 6.4, we discuss convex feasibility formulations for exchange economies with some special and widely used utility functions, more precisely, linear and CES functions.

In Section 6.5, we expose the limitations of convex programming techniques, by presenting examples where convexity is violated (the equilibria are multiple and disconnected), and relating some of these examples to other equilibrium problems and to recently proven hardness results.

In Section 6.6, we discuss convex feasibility formulations for economies that gen eralize the exchange model by including production technologies.

Finally, in Section 6.7, we guide the reader through the bibliography.

## 6.2 Fisher Model with Homogeneous Consumers

Whenever the traders have homogeneous utility functions, the equilibrium conditions for Fisher model can be rewritten as the solution to the following convex program (Eisenberg’s program), on nonnegative variables $x _ { i j }$ :

$$
\text { Maximize } \sum_ {i} e _ {i} \log u _ {i} (x _ {i})
$$

$$
\text { Subject   to } \quad \sum_ {i} x _ {i j} \leq q _ {j} \quad \text { for   each } j.
$$

Recall that $u _ { i }$ is the i-th trader’s utility function, $e _ { i }$ is the i-th trader’s endowment of money, and $q _ { j }$ is the amount of the j-th good.

Notice that the program does not have variables corresponding to prices. The optimal solution to this program yields allocations for each trader that, at prices given by the Lagrangian dual variables corresponding to the optimal solution, are exactly the individual demands of the traders. We present a proof of this result for the case where the utility functions are differentiable.

Let x¯ be an optimal solution to Eisenberg’s program. Observe that $u _ { i } ( \bar { x } _ { i } ) > 0$ for each i. The Karush–Kuhn–Tucker necessary optimality theorem (Mangasarian, 1969, Chapter 7.7) says that there exist $\pi _ { j } \geq 0$ , for each good $j .$ , and $\lambda _ { i j } \geq 0$ , for each trader i and good j, such that

$$
\pi_ {j} \left(\left(\sum_ {i} x _ {i j}\right) - q _ {j}\right) = 0 \quad \text { for   each   good } j,\tag{6.2}
$$

$$
\lambda_ {i j} x _ {i j} = 0 \quad \text { for   each } i, j,\tag{6.3}
$$

and

$$
\frac {e _ {i}}{u _ {i} (\bar {x} _ {i})} \times \frac {\partial u _ {i} (\bar {x} _ {i})}{\partial x _ {i j}} = \pi_ {j} - \lambda_ {i j} \quad \text { for   each } i, j.\tag{6.4}
$$

For trader i, let us multiply the j-th equality in (6.4) by $\bar { x } _ { i j }$ , and add the resulting equalities. We obtain

$$
\frac {e _ {i}}{u _ {i} (\bar {x} _ {i})} \sum_ {j} \bar {x} _ {i j} \frac {\partial u _ {i} (\bar {x} _ {i})}{\partial x _ {i j}} = \sum_ {j} (\pi_ {j} - \lambda_ {i j}) \bar {x} _ {i j}.
$$

Using 6.3 and Euler’s identity $\begin{array} { r } { u _ { i } ( x _ { i } ) = \sum _ { j } x _ { i j } \frac { \partial u _ { i } } { \partial x _ { i j } } } \end{array}$ for the homogeneous $u _ { i }$ , this equality becomes

$$
e _ {i} = \sum_ {j} \pi_ {j} \bar {x} _ {i j}.
$$

At the price vector $\pi$ , the bundle ${ \bar { x } } _ { i }$ thus exhausts the budget of trader i. Let $y _ { i } \in \mathbf { R } _ { + } ^ { n }$ be any bundle such that $\pi \cdot y _ { i } \leq e _ { i }$ . We proceed along the lines of the Karush–Kuhn– Tucker sufficient optimality theorem (Mangasarian, 1969, Chapter 7.2) to show that $u _ { i } ( { \bar { x } } _ { i } ) \geq u _ { i } ( y _ { i } )$ . Using the concavity of $u _ { i }$ ,

$$
\begin{array}{l} u _ {i} (y _ {i}) - u _ {i} (\bar {x} _ {i}) \leq \nabla u (\bar {x} _ {i}) \cdot (y _ {i} - \bar {x} _ {i}) \\ = \frac {u _ {i} (\bar {x} _ {i})}{e _ {i}} \sum_ {j} (\pi_ {j} - \lambda_ {i j}) (y _ {i j} - \bar {x} _ {i j}) \\ = \frac {u _ {i} (\bar {x} _ {i})}{e _ {i}} \left(\sum_ {j} (\pi_ {j} y _ {i j} - \lambda_ {i j} y _ {i j}) - e _ {i}\right) \\ \leq \frac {u _ {i} (\bar {x} _ {i})}{e _ {i}} \left(\sum_ {j} \pi_ {j} y _ {i j} - e _ {i}\right) \\ \leq 0. \end{array}
$$

We have shown that that ${ \bar { x } } _ { i }$ is a demand of trader i at price π. Turning now to market clearance, observe that (6.2) implies that $\begin{array} { r } { \sum _ { i } \bar { x } _ { i j } = q _ { j } } \end{array}$ for any good $j$ such that $\pi _ { j } > 0$ For each good $j$ such that $\pi _ { j } = 0$ , feasibility tells us that $\textstyle \sum _ { i } { \bar { x } } _ { i j } \leq q _ { j } $ ; let us allocate the excess of any such good to trader 1. Slightly abusing notation, let $\bar { x } _ { 1 }$ still denote the first trader’s allocation. The bundle $\bar { x } _ { 1 }$ continues to be a demand of trader 1 at price $\pi$ , since the newly allocated goods have price zero and adding positive quantities of a certain good cannot decrease $u _ { 1 }$ . We have now satisfied all the requirements of an equilibrium.

## 6.3 Exchange Economies Satisfying WGS

We now consider exchange economies that satisfy WGS. In this scenario the following important Lemma holds.

Lemma 6.4 Let πˆ be an equilibrium price vector for an exchange economy that satisfies gross substitutability, and π be any nonequilibrium price vector. We then have πˆ $Z ( \pi ) > 0$

This lemma implies that the set of equilibrium prices forms a convex set by providing for any positive price vector π that is not an equilibrium price vector, a separating hyperplane, i.e., a hyperplane that separates π from the set of equilibrium prices. This is the hyperplane $\{ x \in \Re ^ { n } \mid x \cdot Z ( \pi ) = 0 \}$ : indeed we have $\hat { \pi } \cdot Z ( \pi ) > 0$ , whereas $\pi \cdot Z ( \pi ) = 0 $ , by Walras’ law. To compute this separating hyperplane, we need to compute the demands $Z _ { j } ( \pi )$ at the prices π.

## 6.3.1 Computational Results

Lemma 6.4 tells us that if we start at price π, and move in the direction $Z ( \pi )$ , the Euclidean distance to the equilibrium ˆπ decreases. This observation is in fact the crux of the proof that a certain tatonnement process converges to the equilibrium.ˆ

We now present a simple algorithm, which is a discrete version of the tatonnementˆ process, and prove that it converges to an approximate equilibrium in polynomial time for exchange markets satisfying WGS. For this, however, we will need to work with a transformed market.

## Two Useful Transformations

We now describe a transformation that, given the exchange market M, produces a new market M<sup></sup> in which the total amount of each good is 1. The new utility function of the i-th trader is given by $u _ { i } ^ { \prime } ( x _ { 1 } , \ldots , x _ { n } ) = u _ { i } ( W _ { 1 } x _ { 1 } , \ldots , W _ { n } x _ { n } )$ , where $W _ { j }$ denotes $\sum _ { i } w _ { i j }$ . It can be verified that, if $u _ { i } ( \boldsymbol { \mathbf { \rho } } )$ is concave, then $u _ { i } ^ { \prime } ( \boldsymbol { \mathbf { \rho } } )$ is concave. The new initial endowment of the j-th good held by the i-th trader is $w _ { i j } ^ { \prime } = w _ { i j } / W _ { j }$ . Let $w _ { i } ^ { \prime }$ denote $( w _ { i 1 } ^ { \prime } , \ldots , w _ { i n } ^ { \prime } ) \in { \bf R } _ { + } ^ { n }$ . Clearly, $\begin{array} { r } { W _ { j } ^ { \prime } = \sum _ { i } w _ { i j } ^ { \prime } = 1 } \end{array}$

The following lemma summarizes some key properties of the transformation.

Lemma 6.5

(i) For any $\mu \geq 1 , ( x _ { i 1 } , \ldots , x _ { i n } )$ is a µ-approximate demand at prices $( \pi _ { 1 } , \ldots , \pi _ { n } )$ for trader i in M<sup></sup> ifand only ifthe vector $( W _ { 1 } x _ { i 1 } , \dots , W _ { n } x _ { i n } )$ is a µ-approximate demand at prices $\big ( \frac { \pi _ { 1 } } { W _ { 1 } } , \ldots , \frac { \pi _ { n } } { W _ { n } } \big )$ for trader i in M.

(ii) For any $\mu \geq 1 , ( \pi _ { 1 } , \ldots , \pi _ { n } )$ is a weak µ-approximate equilibrium for M<sup></sup> if and only $\begin{array} { r } { i f ( \frac { \pi _ { 1 } } { W _ { 1 } } , \ldots , \frac { \pi _ { n } } { W _ { n } } ) } \end{array}$ is a weak µ-approximate equilibriumfor M.

(iii) The excess demand ofM<sup></sup> satisfies WGS ifthe excess demand ofM does.

We transform $M ^ { \prime }$ into another market $\hat { M }$ as follows. Let $0 < \eta \leq 1$ be a parameter. For each trader i, the new utility function and initial endowments are the same, i.e., $\hat { u } _ { i } ( ) = u _ { i } ^ { \prime } ( )$ , and $\hat { w } _ { i } = w _ { i } ^ { \prime }$ . The new market $\hat { M }$ has one extra trader, whose initial endowment is given by $\hat { w } _ { m + 1 } = ( \eta , \dots , \eta )$ , and whose utility function is the Cobb– Douglas function $\begin{array} { r } { u _ { m + 1 } ( x _ { m + 1 } ) = \prod _ { j } x _ { m + 1 , j } ^ { 1 / n } } \end{array}$ . A trader with this Cobb–Douglas utility function spends $1 / n \mathrm { - t h }$ of her budget on each good. Stated precisely, $\pi _ { j } x _ { m + 1 , j } ( \pi ) =$ $\pi \cdot \hat { w } _ { m + 1 } / n$

Note that the total amount of good j in the market M<sup>ˆ</sup> is $\begin{array} { r } { \hat { W } _ { j } = \sum _ { i = 1 } ^ { m + 1 } \hat { w } _ { i j } = 1 + \eta } \end{array}$

Lemma 6.6 (1) The market $\hat { M }$ has an equilibrium. (2) Every equilibrium π of $\hat { M }$ satisfies the condition $\begin{array} { r } { \frac { \operatorname* { m a x } _ { j } \pi _ { j } } { \operatorname* { m i n } _ { j } \pi _ { j } } \leq 2 n / \eta } \end{array}$ . (3) For any $\mu \geq 1$ , a weak µ-approx equilibriumfor $\hat { M }$ is a weak $\mu ( 1 + \eta )$ -approx equilibriumfor M<sup></sup>. (4) M<sup>ˆ</sup> satisfies WGS ifM<sup></sup> does.

proof Statement (1) follows from arguments that are standard in microeconomic theory. Briefly, a quasi-equilibrium $\pi \in \mathbf { R } _ { + } ^ { n }$ with $\textstyle \sum _ { j } \pi _ { j } = 1$ always exists (Mas-Colell et al., 1995, Chapter 17, Proposition 17.BB.2). At price π the income $\boldsymbol { \pi } \cdot \hat { \boldsymbol { w } } _ { m + 1 }$ of the $( m + 1 )$ -th trader is strictly positive. This ensures that that $\pi _ { j } > 0$ for each good $j .$ . But this implies (Mas-Colell et al., 1995, Chapter 17, Proposition 17.BB.1) that π is an equilibrium.

The proofs of the remaining statements are left as Exercise 6.4. The proof of (2) illustrates one crucial role that the extra trader plays.

We define $\Delta = \{ \pi \in \mathbf { R } _ { + } ^ { n } | \eta / 2 n \leq \pi _ { j } \leq 1$ for each $j \}$ . Note that Lemma 6.6 implies that $\hat { M }$ has an equilibrium price in $\Delta$ . We define $\Delta ^ { + } = \{ \pi \in \mathbf { R } _ { + } ^ { n } | \eta / 4 n \leq \pi _ { j } \leq 1 +$ $\eta / 4 n$ for each $j \}$ . For any $\pi \in \Delta ^ { + }$ , we have $\begin{array} { r } { \frac { \operatorname* { m a x } _ { j } \pi _ { j } } { \operatorname* { m i n } _ { j } \pi _ { j } } \leq \frac { 1 + \eta / 4 n } { \eta / 4 n } \leq \frac { 5 n } { \eta } } \end{array}$

Abusing notation slightly, we henceforth let $\scriptstyle { \vec { Z } } ( \pi )$ and $X ( \pi )$ denote, respectively, the excess demand vector and the aggregate demand vector in the market $\hat { M }$

## The Discrete Tatonnement Processˆ

We now state an algorithm for computing a weak $( 1 + \varepsilon )$ -approximate equilibrium for $\hat { M }$ . From Lemma 6.5 and Lemma 6.6 (applied with $\eta = \varepsilon )$ , this $( 1 + \varepsilon )$ )-approximate equilibrium for $\hat { M }$ will then be a $( 1 + O ( \varepsilon ) )$ )-approximate equilibrium for M. The algorithm assumes access to an oracle that can compute the excess demand vector of $\hat { M }$ at any given price vector in $\Delta ^ { + }$ . Such an oracle is readily constructed from an oracle for computing the excess demand for $M$

Let $\pi ^ { 0 }$ , the initial price, be any point in $\Delta$ . Suppose that we have computed a sequence of prices $\pi ^ { 0 } , \ldots , \pi ^ { i - 1 }$ . We compute $\pi ^ { i }$ as follows. If $\pi ^ { i - 1 } \notin \Delta ^ { + }$ , we let $\pi ^ { i }$ be the point in $\Delta$ closest to $\pi ^ { i - 1 }$ . In other words, $\pi _ { j } ^ { i } = \pi _ { j } ^ { i - 1 }$ if $\eta / 2 n \leq \pi _ { j } ^ { i - 1 } \leq 1 ;$ $\pi _ { j } ^ { i } = 1 \mathrm { { i f } } \pi _ { j } ^ { i - 1 } > 1 ; \pi _ { j } ^ { i } = \eta / 2 n \mathrm { { i f } } \pi _ { j } ^ { i - 1 } < \eta / 2 n$

I $\bar { \cdot } \pi ^ { i - 1 } \in \Delta ^ { + }$ , we let

$$
\pi^ {i} = \pi^ {i - 1} + \frac {\delta}{(1 2 n ^ {2} / \eta) ^ {2}} Z (\pi^ {i - 1}).
$$

## Analysis ofConvergence

Lemma 6.4 is the building block upon which the proof of convergence of the (continuous) tatonnement process is based. To prove the (fast) convergence of the discreteˆ process just described, we need a more general result (Lemma 6.7 below). Together with Lemma 6.8, it says that if a vector $\pi \in \Delta ^ { + }$ is not a weak (1 + ε)-approx equilib rium for $\hat { M }$ , then the hyperplane normal to $Z ( \pi )$ and passing through π separates π from all points within a certain distance of any equilibrium of $\hat { M }$ in $\Delta$

Lemma 6.7 Let $\pi \in \Delta ^ { + }$ be a price vector that is not a weak $( 1 + \varepsilon ) \cdot$ approximate equilibriumfor $\hat { M }$ ,for some $\varepsilon > 0$ . Thenfor any equilibrium πˆ $\in \Delta$ we have ${ \hat { \pi } } \cdot Z ( \pi ) \geq \delta > 0$ , where $1 / \delta$ is bounded by a polynomial in $n , \textstyle { \frac { 1 } { \varepsilon } }$ , and $\frac { 1 } { \eta }$ .

proof We can assume that the goods are ordered so that $\begin{array} { r } { \frac { \pi _ { 1 } } { \hat { \pi } _ { 1 } } \leq \frac { \pi _ { 2 } } { \hat { \pi } _ { 2 } } \leq \cdots \leq \frac { \pi _ { n } } { \hat { \pi } _ { n } } } \end{array}$ Let $\alpha _ { s }$ denote the quantity $\frac { \pi _ { s } } { \hat { \pi } _ { s } }$ . For $1 \leq s \leq n$ , let $q ^ { s }$ denote the price vector min $\{ \alpha _ { s } \hat { \pi } , \pi \}$ , i.e., the componentwise minimum of $\alpha _ { s } \hat { \pi }$ and $\pi$ . Note that

$$
q ^ {s} = (\pi_ {1}, \dots , \pi_ {s - 1}, \pi_ {s} = \alpha_ {s} \hat {\pi} _ {s}, \alpha_ {s} \hat {\pi} _ {s + 1}, \dots , \alpha_ {s} \hat {\pi} _ {n}).
$$

The first price $q _ { 1 }$ in the sequence is an equilibrium price vector, being a scaling of ˆπ by $\alpha _ { 1 }$ , and the last price vector $q _ { n }$ is $\pi$ . For $1 \leq s \leq n - 1$ , let $G _ { s } ^ { h }$ denote the set of goods $\{ 1 , \ldots , s \}$ and $G _ { s } ^ { t }$ denote the set of goods $\{ s + 1 , \ldots , n \}$ }. If $\alpha _ { s } < \alpha _ { s + 1 } , G _ { s } ^ { h }$ is the subset of goods whose prices remain fixed during the s-th step, where we move from $q ^ { s }$ to $q ^ { s + 1 }$ , and $G _ { s } ^ { t }$ is the complement set.

Focusing on the s-th step, we have

$$
\begin{array}{l} 0 = q ^ {s + 1} \cdot Z (q ^ {s + 1}) - q ^ {s} \cdot Z (q ^ {s}) \\ \quad = \sum_ {j \in G _ {s} ^ {h}} \pi_ {j} \left(Z _ {j} (q ^ {s + 1}) - Z _ {j} (q ^ {s})\right) + \sum_ {j \in G _ {s} ^ {t}} \left(\alpha_ {s + 1} \hat {\pi} _ {j} Z _ {j} (q ^ {s + 1}) - \alpha_ {s} \hat {\pi} _ {j} Z _ {j} (q ^ {s})\right) \\ \quad = \alpha_ {s + 1} \sum_ {j} \hat {\pi} _ {j} \left(Z _ {j} (q ^ {s + 1}) - Z _ {j} (q ^ {s})\right) + \sum_ {j \in G _ {s} ^ {t}} (\alpha_ {s + 1} - \alpha_ {s}) \hat {\pi} _ {j} Z _ {j} (q ^ {s}) \\ \quad - \sum_ {j \in G _ {s} ^ {h}} (\alpha_ {s + 1} \hat {\pi} _ {j} - \pi_ {j}) \left(Z _ {j} (q ^ {s + 1}) - Z _ {j} (q ^ {s})\right). \end{array}
$$

Applying weak GS to the price vectors $q ^ { s }$ and $\alpha _ { s } \hat { \pi }$ , we see that $Z _ { j } ( q ^ { s } ) \leq 0$ for $j \in G _ { s } ^ { t }$ . Applying weak GS to the price vectors $q ^ { s }$ and $q ^ { s + 1 }$ , we see that $Z _ { j } ( q ^ { s + 1 } ) \geq Z _ { j } ( q ^ { s } )$ for $j \in G _ { s } ^ { h }$ . Noting that $\pi _ { j } \leq \alpha _ { s } \hat { \pi } _ { j } \leq \alpha _ { s + 1 } \hat { \pi } _ { j }$ for $j \in G _ { s } ^ { h }$ , we have

$$
\begin{array}{l} \alpha_ {s + 1} \sum_ {j} \hat {\pi} _ {j} \left(Z _ {j} (q ^ {s + 1}) - Z _ {j} (q ^ {s})\right) \\ = \sum_ {j \in G _ {s} ^ {h}} (\alpha_ {s + 1} \hat {\pi} _ {j} - \pi_ {j}) \left(Z _ {j} (q ^ {s + 1}) - Z _ {j} (q ^ {s})\right) \\ - \sum_ {j \in G _ {s} ^ {t}} (\alpha_ {s + 1} - \alpha_ {s}) \hat {\pi} _ {j} Z _ {j} (q ^ {s}) \\ \geq \sum_ {j \in G _ {s} ^ {h}} (\alpha_ {s + 1} \hat {\pi} _ {j} - \pi_ {j}) \left(Z _ {j} (q ^ {s + 1}) - Z _ {j} (q ^ {s})\right) \\ \geq (\alpha_ {s + 1} - \alpha_ {s}) \sum_ {j \in G _ {s} ^ {h}} \hat {\pi} _ {j} \left(Z _ {j} (q ^ {s + 1}) - Z _ {j} (q ^ {s})\right). \end{array}
$$

That is,

$$
\hat {\pi} \cdot (Z _ {j} (q ^ {s + 1}) - Z _ {j} (q ^ {s})) \geq \left(1 - \frac {\alpha_ {s}}{\alpha_ {s + 1}}\right) \sum_ {j \in G _ {s} ^ {h}} \hat {\pi} _ {j} \left(Z _ {j} (q ^ {s + 1}) - Z _ {j} (q ^ {s})\right)\tag{6.5}
$$

Since the right-hand side is nonnegative, we have, for each $1 \leq s \leq n - 1$

$$
\hat {\pi} \cdot (Z _ {j} (q ^ {s + 1}) - Z _ {j} (q ^ {s})) \geq 0.\tag{6.6}
$$

Because $\pi = q ^ { n }$ is not a weak ε-approximate equilibrium for $\hat { M }$ , we must have $\begin{array} { r } { \frac { \alpha _ { n } } { \alpha _ { 1 } } \ge 1 + \varepsilon / 3 } \end{array}$ . (See Exercise 6.5.) So there is some value $1 \leq k \leq n - 1$ so that $\textstyle \frac { \widehat { \alpha } _ { k + 1 } ^ { \mathtt { i } } } { \alpha _ { k } } \geq 1 + \varepsilon / 6 n$ . We will show that the right-hand side of equation (6.5) is large for k.

We have $\textstyle 1 - { \frac { \alpha _ { k } } { \alpha _ { k + 1 } } } \geq { \frac { \varepsilon / 6 n } { 1 + \varepsilon / 6 n } } \geq { \frac { \varepsilon } { 1 2 n } }$

We can lower bound that the increase in income of the $( m + 1 )$ -th trader when we move from $q ^ { k }$ to $q ^ { k + 1 }$ :

$$
\begin{array}{c} q ^ {k + 1} \cdot \hat {w} _ {m + 1} - q ^ {k} \cdot \hat {w} _ {m + 1} \geq (q _ {n} ^ {k + 1} - q _ {n} ^ {k}) \hat {w} _ {m + 1, n} = (\alpha_ {k + 1} - \alpha_ {k}) \hat {\pi} _ {n} \hat {w} _ {m + 1, n} \\ \geq \frac {\varepsilon \alpha_ {k}}{6 n} \hat {\pi} _ {n} \hat {w} _ {m + 1, n}. \end{array}
$$

Recall that the $( m + 1 )$ -th trader is a Cobb–Douglas trader with a utility function that ensures that she spends $\frac { 1 } { n }$ th of her income on each good. As a result, we have

$$
\begin{array}{l} x _ {m + 1, 1} (q ^ {k + 1}) - x _ {m + 1, 1} (q ^ {k}) = \frac {q ^ {k + 1} \cdot \hat {w} _ {m + 1}}{n q _ {1} ^ {k + 1}} - \frac {q ^ {k} \cdot \hat {w} _ {m + 1}}{n q _ {1} ^ {k}} \\ \qquad = \frac {1}{n \pi_ {1}} (q ^ {k + 1} \cdot \hat {w} _ {m + 1} - q ^ {k} \cdot \hat {w} _ {m + 1}) \\ \qquad \geq \frac {\varepsilon \alpha_ {k} \hat {\pi} _ {n} \hat {w} _ {m + 1 , n}}{6 n ^ {2} \pi_ {1}}. \end{array}
$$

Since the market $M ^ { \prime }$ (the one without the $( m + 1 )$ -th trader) satisfies weak GS and $1 \in G _ { s } ^ { h }$ , we have

$$
\sum_ {i = 1} ^ {m} x _ {i, 1} (q ^ {k + 1}) - \sum_ {i = 1} ^ {m} x _ {i, 1} (q ^ {k}) \geq 0.
$$

Adding the two inequalities, we get $\begin{array} { r } { Z _ { 1 } ( q ^ { k + 1 } ) - Z _ { 1 } ( q ^ { k } ) \ge \frac { \varepsilon \alpha _ { k } \hat { \pi } _ { n } \hat { w } _ { m + 1 , n } } { 6 n ^ { 2 } \pi _ { 1 } } } \end{array}$ . Plugging this into equation (6.5), and recalling that $Z _ { j } ( q ^ { k + 1 } ) - Z _ { j } ( q ^ { k } ) \geq 0$ <sup>1</sup> for $j \in G _ { k } ^ { h }$ , we have

$$
\begin{array}{l} \hat {\pi} \cdot (Z _ {j} (q ^ {k + 1}) - Z _ {j} (q ^ {k})) \geq \left(1 - \frac {\alpha_ {k}}{\alpha_ {k + 1}}\right) \sum_ {j \in G _ {k} ^ {h}} \hat {\pi} _ {j} \left(Z _ {j} (q ^ {k + 1}) - Z _ {j} (q ^ {k})\right) \\ \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \\ \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qquad \qend{array}
$$

Adding this inequality and the inequalities (6.6) for each $s \neq k$ , we get

$$
\hat {\pi} \cdot Z (\pi) = \hat {\pi} \cdot (Z (q ^ {n}) - Z (q ^ {1})) \geq \frac {\varepsilon^ {2} \alpha_ {k} \hat {\pi} _ {n} \hat {w} _ {m + 1 , n}}{7 2 n ^ {3} \pi_ {1}} = \delta .
$$

It is easily verified that $1 / \delta$ is bounded by a polynomial in $n , 1 / \varepsilon$ , and $1 / \eta$ .

Lemma 6.8 For any $\pi \in \Delta ^ { + } , | | Z ( \pi ) | | _ { 2 } \leq 1 2 n ^ { 2 } / \eta$

proof

$$
\begin{array}{l} | | Z (\pi) | | _ {2} \leq \sum_ {j} | Z _ {j} (\pi) | \\ \quad \leq \sum_ {j} X _ {j} (\pi) + \sum_ {j} \hat {W} _ {j} \\ \quad \leq \frac {\max _ {k} \pi_ {k}}{\min _ {k} \pi_ {k}} \sum_ {j} \hat {W} _ {j} + \sum_ {j} \hat {W} _ {j} \\ \quad \leq \frac {5 n}{\eta} \sum_ {j} \hat {W} _ {j} + \sum_ {j} \hat {W} _ {j} \\ \quad \leq \frac {1 0 n ^ {2}}{\eta} + 2 n \\ \quad \leq \frac {1 2 n ^ {2}}{\eta}, \end{array}
$$

where the third inequality follows from a simple calculation, the fourth inequal ity holds because $\pi \in \Delta ^ { + }$ , and the fifth inequality holds because $\hat { W } _ { j } \le 2$ for each $\begin{array} { r l } { j . } & { { } \square } \end{array}$

We are now ready for the proof of correctness of the discrete tatonnement process.ˆ

Theorem 6.9 Let µ denote min $\{ \frac { \delta ^ { 2 } } { ( 1 2 n ^ { 2 } / \eta ) ^ { 2 } } , ( \eta / 4 n ) ^ { 2 } \}$ . Within $n / \mu$ iterations, the algorithm computes a price in $\Delta ^ { + }$ which is a weak $( 1 + \varepsilon )$ -approximate equi librium for $\hat { M }$ . (Note that the bound on $\mu$ is polynomial in the input size of the original market M, $1 / \varepsilon _ { \ast }$ , and $1 / \eta . \jmath$ 1

proof Let us fix an equilibrium $\pi ^ { * }$ of $\hat { M }$ in $\Delta$ . We argue that in each iteration, the distance to $\pi ^ { * }$ falls significantly so long as we do not encounter an approximate equilibrium in $\Delta ^ { + } . \operatorname { I f } \pi ^ { i - 1 } \notin \Delta ^ { + }$ , we have $| \pi _ { j } ^ { i - 1 } - \pi _ { j } ^ { * } | - | \pi _ { j } ^ { i } - \pi _ { j } ^ { * } | \geq 0$ for each $j ,$ , while $| \pi _ { j } ^ { i - 1 } - \pi _ { j } ^ { * } | - | \pi _ { j } ^ { i } - \pi _ { j } ^ { * } | \geq \eta / 4 n$ for some $j .$ . From this it follows that

$$
| | \pi^ {*} - \pi^ {i - 1} | | ^ {2} - | | \pi^ {*} - \pi^ {i} | | ^ {2} \geq (\eta / 4 n) ^ {2}.
$$

Now suppose that $\pi ^ { i - 1 } \in \Delta ^ { + }$ and that $\pi ^ { i - 1 }$ is not a weak $( 1 + \varepsilon )$ )-approx equilibrium for $\hat { M }$ . By Lemma $6 . 7 , \pi ^ { * } \cdot Z ( \pi ^ { i - 1 } ) \geq \delta$ . Since $\pi ^ { i - 1 } \cdot Z ( \pi ^ { i - 1 } ) = 0$ by Walras’ Law, we have $( \pi ^ { * } - \pi ^ { i - 1 } ) \cdot Z ( \pi ^ { i - 1 } ) \geq \delta$

Let $q$ denote the vector $\begin{array} { r } { \pi ^ { i } - \pi ^ { i - 1 } = \frac { \delta } { ( 1 2 n ^ { 2 } / \eta ) ^ { 2 } } Z ( \pi ^ { i - 1 } ) } \end{array}$ . We have

$$
\begin{array}{l} (\pi^ {*} - \pi^ {i - 1} - q) \cdot q \\ = (\pi^ {*} - \pi^ {i - 1}) \cdot q - q \cdot q \\ = \frac {\delta}{(1 2 n ^ {2} / \eta) ^ {2}} \left((\pi^ {*} - \pi^ {i - 1}) \cdot Z (\pi^ {i - 1}) - \frac {\delta}{(1 2 n ^ {2} / \eta) ^ {2}} | | Z (\pi^ {i - 1}) | | _ {2} ^ {2}\right) \\ \geq \frac {\delta}{(1 2 n ^ {2} / \eta) ^ {2}} \left(\delta - \frac {\delta}{(1 2 n ^ {2} / \eta) ^ {2}} 1 2 n ^ {2} / \eta\right) \geq 0. \end{array}
$$

Thus,

$$
\begin{array}{l} | | \pi^ {*} - \pi^ {i - 1} | | ^ {2} - | | \pi^ {*} - \pi^ {i} | | ^ {2} \\ = | | \pi^ {*} - \pi^ {i - 1} | | ^ {2} - | | \pi^ {*} - \pi^ {i - 1} - q | | ^ {2} \\ = (\pi^ {*} - \pi^ {i - 1}) \cdot q + (\pi^ {*} - \pi^ {i - 1} - q) \cdot q \\ \geq (\pi^ {*} - \pi^ {i - 1}) \cdot q \\ = \frac {\delta}{(1 2 n ^ {2} / \eta) ^ {2}} (\pi^ {*} - \pi^ {i - 1}) \cdot Z (\pi^ {i - 1}) \\ \geq \frac {\delta^ {2}}{(1 2 n ^ {2} / \eta) ^ {2}}, \end{array}
$$

Suppose that every vector in the sequence $\pi ^ { 0 } , \ldots , \pi ^ { k }$ is either not in $\Delta ^ { + }$ or not a weak (1 + ε)-approx equilibrium. We then have

$$
\left. \right.\left|\left| \pi^ {*} - \pi^ {i - 1} \right|\right| ^ {2} - \left|\left| \pi^ {*} - \pi^ {i} \right|\right| ^ {2} \geq \min \left\{\frac {\delta^ {2}}{(1 2 n ^ {2} / \eta) ^ {2}}, (\eta / 4 n) ^ {2} \right\} = \mu ,
$$

for $1 \leq i \leq k$ . Adding these inequalities, we get

$$
k \mu \leq | | \pi^ {*} - \pi^ {0} | | ^ {2} - | | \pi^ {*} - \pi^ {k} | | ^ {2} \leq n.
$$

Putting everything together, we can state the main result of this section.

Theorem 6.10 Let M be an exchange market whose excess demand function satisfies WGS, and suppose that M is equipped with an oraclefor computing the excess demand at any given price vector. For any $\varepsilon > 0$ , the tatonnement-basedˆ algorithm computes, in timepolynomial in the input size ofM and $1 / \varepsilon ,$ , a sequence ofprices one ofwhich is a weak (1 + ε)-approx equilibriumfor M.

In order to actually pick the approximate equilibrium price from the sequence of prices, we need an efficient algorithm that recognizes an approximate equilibrium of M. In fact, it is sufficient for this algorithm to assert that a given price π is a weak $( 1 + 2 \varepsilon )$ approximate equilibrium provided π is a weak (1 + ε)-approximate equilibrium. Since the problem of recognizing an approximate equilibrium is an explicitly presented convex programming problem, such an algorithm is generally quite easy to construct.

## 6.4 Specific Utility Functions

In many economic scenarios, the market is modeled by consumers having some specific utility functions. While in some cases this does not lead to a simplified computationa problem, in other instances, the specific utility functions might expose a computation ally useful structure. This turns out to be the case for linear utility functions, as well as for certain CES utility functions.

## 6.4.1 Convex Programs for Linear Exchange Economies

The equilibrium conditions for an exchange economy with linear utilities can be written as a finite convex feasibility problem. Suppose that the linear utility function of the i-th trader is $\sum _ { j } a _ { i j } x _ { i j }$ , and suppose that $w _ { i j } > 0$ for each $i , j$

Consider now the problem of finding $\psi _ { j }$ and nonnegative $x _ { i j }$ such that

$$
\begin{array}{l} \sum_ {k} a _ {i k} x _ {i k} \geq a _ {i j} \sum_ {k} w _ {i k} e ^ {\psi_ {k} - \psi_ {j}}, \text {   for   each   } 1 \leq i \leq m, 1 \leq j \leq n. \\ \sum_ {i} x _ {i} = \sum_ {i} w _ {i}. \end{array}
$$

Any solution to this program corresponds to an equilibrium obtained by setting $\pi _ { j } = e ^ { \psi _ { j } }$ . The converse also holds, i.e., any equilibrium corresponds to a solution to this program.

We will discuss the ideas behind the derivation of the convex program above in the context of economies with production (Section 6.6).

## 6.4.2 Convex Programs for CES Exchange Economies

Demand of CES Consumers. We start by characterizing the demand function of traders with CES utility functions. Consider a setting where trader i has an initial endowment $w _ { i } = ( w _ { i 1 } , \ldots , w _ { i n } ) \in \mathbf { R } _ { + } ^ { n }$ of goods, and the CES utility function $\begin{array} { r } { u _ { i } ( x _ { i 1 } , \dots , x _ { i n } ) = ( \sum _ { j = 1 } ^ { n } \alpha _ { i j } x _ { i j } ^ { \rho _ { i } } ) ^ { \frac { 1 } { \rho _ { i } } } } \end{array}$ , where $\alpha _ { i j } > 0 , w _ { i j } > 0$ , and $- \infty < \rho _ { i } < 1$ , but $\rho _ { i } \neq 0 . \mathrm { I f } \rho _ { i } < 0$ , we define $u _ { i } ( x _ { i 1 } , \dots , x _ { i n } ) = 0$ if there is $\textbf { a } j$ such that $x _ { i j } = 0$ . Note that this ensures that $u _ { i }$ is continuous over $\mathbf { R } _ { + } ^ { n }$

The demand vector for the i-th consumer is unique and is given by the expression

$$
x _ {i j} (\pi) = \frac {\alpha_ {i j} ^ {1 / 1 - \rho_ {i}}}{\pi_ {j} ^ {1 / 1 - \rho_ {i}}} \times \frac {\sum_ {k} \pi_ {k} w _ {i k}}{\sum_ {k} \alpha_ {k} ^ {1 / 1 - \rho_ {i}} \pi_ {k} ^ {- \rho_ {i} / 1 - \rho_ {i}}}.\tag{6.7}
$$

The formula above can be derived using the Karush–Kuhn–Tucker conditions.

Efficient Computation by Convex Programming. Consider an economy in which each trader i has a CES utility function with $- 1 \le \rho _ { i } < 0$ . We show that the equilibria of such an economy can be characterized as the solutions of a convex feasibility problem.

Since the demand of every trader is well-defined and unique at any price, we may write the equilibria as the set $\pi \in \mathbf { R } _ { + + }$ such that for each good $j$ , we have $\begin{array} { r } { \sum _ { i } x _ { i j } ( \pi ) \le } \end{array}$ $\sum _ { i } w _ { i j }$ . Let $\rho = - 1$ , and note that $\rho \leq \rho _ { i }$ , for each i. Let $f _ { i j } ( \pi ) = \pi _ { j } ^ { 1 / ( 1 - \rho ) } x _ { i j } ( \pi )$ , and $\sigma _ { j } = \pi _ { i } ^ { 1 / ( 1 - \rho ) }$ . In terms of the $\sigma _ { j } { \mathrm {  ~ s } } _ { : }$ , we obtain the set of $\sigma = ( \sigma _ { 1 } , \ldots , \sigma _ { n } ) \in \mathbf { R } _ { + + }$ such that for each good $j$ ,

$$
\sum_ {i} f _ {i j} (\sigma) \leq \sigma_ {j} \left(\sum_ {i} w _ {i j}\right).
$$

We now show that these inequalities give rise to a convex feasibility program. Since the right-hand side of each inequality is a linear function, it suffices to argue that the left-hand side is a convex function. The latter claim is established by the following proposition.

Proposition 6.11 Thefunction $f _ { i j } ( \sigma )$ is a convexfunction over $\mathbf { R } _ { + + }$

proof Clearly, it suffices to show that the constraint $f _ { i j } \leq t$ defines a convex set for positive t. Using formula (6.7) for the demand, this constraint can be written as

$$
\frac {\alpha_ {i j} ^ {\frac {1}{1 - \rho_ {i}}}}{\sigma_ {j} ^ {\frac {\rho_ {i} - \rho}{1 - \rho_ {i}}}} \times \frac {\sum_ {k} \sigma_ {k} ^ {1 - \rho} w _ {i k}}{\sum_ {k} \alpha_ {i k} ^ {\frac {1}{1 - \rho_ {i}}} \sigma_ {k} ^ {\frac {- \rho_ {i} (1 - \rho)}{1 - \rho_ {i}}}} \leq t.
$$

Rewriting, and raising both sides to the power $1 / ( 1 - \rho )$ , we obtain

$$
\alpha_ {i j} ^ {\frac {1}{(1 - \rho) (1 - \rho_ {i})}} \times \left(\sum_ {k} \sigma_ {k} ^ {1 - \rho} w _ {i k}\right) ^ {\frac {1}{1 - \rho}} \leq t ^ {\frac {1}{1 - \rho}} \sigma_ {j} ^ {\frac {\rho_ {i} - \rho}{(1 - \rho_ {i}) (1 - \rho)}} v _ {i} ^ {\frac {- \rho_ {i}}{1 - \rho_ {i}}},\tag{6.8}
$$

where

$$
v _ {i} = \left(\sum_ {k} \alpha_ {i k} ^ {\frac {1}{1 - \rho_ {i}}} \sigma_ {k} ^ {\frac {- \rho_ {i} (1 - \rho)}{1 - \rho_ {i}}}\right) ^ {\frac {1 - \rho_ {i}}{- \rho_ {i} (1 - \rho)}}.\tag{6.9}
$$

The left-hand side of inequality 6.8 is a convex function, and the right-hand side is a concave function that is nondecreasing in each argument when viewed as a function of $t , \sigma _ { j }$ , and $v _ { i }$ , since the exponents are nonnegative and add up to one. Since $\begin{array} { r } { 0 < \frac { - \rho _ { i } ( 1 - \dot { \rho } ) } { 1 - \rho _ { i } } \le 1 } \end{array}$ , the right-hand side of equality $6 . 9$ is a concave function, in fact a CES function. It follows that the right-hand side of inequality 6.8 remains a concave function when $v _ { i }$ is replaced by the right-hand side of equality 6.9. This completes the proof.

It is not hard to verify that the demand generated by an economy with CES util ities as above need not satisfy WGS. Indeed, the connectedness of the equilibria that is a corollary of the above convex feasibility formulation is an interesting new consequence.

## 6.5 Limitations

So far, we have presented efficient algorithms for restricted versions of the market equilibrium problem, which take advantage of the convexity of the set of equilibria. However, the set of equilibria in a general exchange economy does not even need to be connected. This implies that it is not possible to characterize the set of equilibria by a convex formulation.

In Section 6.5.1 we report an example that shows that CES exchange economies may present multiple disconnected equilibria, whenever $\rho < - 1$ . This suggests that it is unlikely that the results shown in Section 6.4.2 can be extended to encompass markets where some traders have CES utility functions with $\rho < - 1$

In Section 6.5.2 we outline some more general obstacles to the efficient solvabil ity of the market equilibrium problem. More precisely, we give a tour of a number of recent computational complexity results which imply that Leontief exchange economies are hard for PPAD, a complexity class that contains a wealth of equilibrium problems. This shows that it is unlikely that the market equilibrium problem, even when restricted to exchange economies with Leontief consumers, can be solved in polynomial time.

## 6.5.1 Multiple Disconnected Equilibria

We describe a simple market with two traders and two goods that has multiple disconnected equilibria. The first trader has an initial bundle $w _ { 1 } = ( 1 , 0 )$ and the CES utility function $u _ { 1 } ( x , y ) = ( ( a x ) ^ { \rho } + y ^ { \rho } ) ^ { 1 / \rho }$ , where $a > 0$ . The second trader has an initial bundle $w _ { 2 } = ( 0 , 1 )$ and the CES utility function $u _ { 2 } ( x , y ) = ( ( x / a ) ^ { \rho } + y ^ { \rho } ) ^ { 1 / \rho }$ . It is possible to show that for each $\rho < - 1$ there is a sufficiently small value of a for which

(i) the vector $( 1 / 2 , 1 / 2 )$ is an equilibrium price and

(ii) the vector $( p , 1 - p )$ is an equilibrium price for some $p < 1 / 2$ , and the vector $( q , 1 -$ $q )$ is not an equilibrium price for any $p < q < 1 / 2$

This economy therefore does not admit a convex programming formulation in terms of some “relative” of the prices (such as the one given in Section 6.4.2 in terms of the $\sigma _ { k } )$ that captures all the price equilibria. Such a formulation implies that if $( p _ { 1 } , 1 - p _ { 1 } )$ is a price equilibrium and $( p _ { 2 } , 1 - p _ { 2 } )$ is a price equilibrium for some $p _ { 1 } < p _ { 2 }$ , then $( p _ { 3 } , 1 - p _ { 3 } )$ is also a price equilibrium for every $p _ { 1 } < p _ { 3 } < p _ { 2 }$

This example suggests that it may not be possible to extend convex programming techniques to encompass markets where some traders have a CES utility function with $\rho < - 1$

## 6.5.2 Hardness for the Class PPAD

The context of computation of equilibria calls for a complexity analysis conducted within the class TFNP of total search problems, i.e., problems whose set of solutions is guaranteed to be non empty. Nash Theorem guarantees that the problem of finding a Nash equilibrium in a noncooperative game in normal form is a total search problem. Arrow and Debreu Theorem gives sufficient conditions under which an exchange economy has an equilibrium. Therefore, under suitable sufficient conditions, the problem of finding a market equilibrium is a total search problem.

An important subclass of $T F N P$ is the class $P P A D$ , which is the class of total functions whose totality is proven by the following simple combinatorial argument: if a directed graph whose nodes have in-degree and out-degree at most one has a source, it must have a sink (see Chapter 2 ofthis book for more background, Papadimitriou, 2007).

This class captures a wealth of equilibrium problems, e.g., the market equilibrium problem as well as Nash equilibria for n-player games. Problems complete for this class include a (suitably defined) computational version of the Brouwer Fixed Point Theorem.

Consider exchange economies where m, the number of traders, is equal to the number of goods, and the i-th trader has an initial endowment given by one unit of the i-th good. The traders have a Leontief (or fixed-proportion) utility function, which describes their goal of getting a bundle of goods in proportions determined by m given parameters.

Given an arbitrary bimatrix game, specified by a pair of $n \times m$ matrices A and B, with positive entries, one can construct a Leontief exchange economy with $n + m$ traders and $n + m$ goods as follows.

Trader i has an initial endowment consisting of one unit of good i, for $i = 1 , \ldots , n +$ m. Traders indexed by any $j \in \{ 1 , \ldots , n \}$ receive some utility only from goods $j \in$ $\{ n + 1 , \ldots , n + m \}$ , and this utility is specified by parameters corresponding to the entries of the matrix B. More precisely the proportions in which the j-th trader wants the goods are specified by the entries on the jth row of B. Vice versa, traders indexed by any $j \in \{ n + 1 , \ldots , n + m \}$ receive some utility only from goods $j \in \{ 1 , \ldots , n \}$ In this case, the proportions in which the j-th trader wants the goods are specified by the entries on the jth column of A.

In the economy above, one can partition the traders in two groups, which bring to the market disjoint sets of goods, and are interested only in the goods brought by the group they do not belong to.

It is possible to show that the Nash equilibria of any bimatrix game $( A , B )$ are in one-to-one correspondence with the market equilibria of such an economy, and that the correspondence can be computed in polynomial time. (For the Leontief economies under consideration, we need to get rid of the assumption – see the Introduction – that we will be concerned only with positive price equilibria. It is only then that they capture the complexity of bimatrix games.)

The problem of computing a Nash equilibrium for two-player nonzero sum games have been proven PPAD-complete. Combined with the game-market correspondence mentioned above, these hardness results imply that the problem of computing a market equilibrium, even when confined to the restrictive scenario of a special family of Leontief economies, is PPAD-complete.

## 6.6 Models with Production

In this section, we derive convex programs for certain economies that generalize the exchange model by including constant returns to scale technologies. The ideas for deriving these convex programs build on the ones developed for exchange economies with special utility functions. In a constant returns economy M, there are  producers, as well as the m consumers and n goods of the exchange model. The k-th producer is equipped with a technology that is capable of producing some good, say $o _ { k }$ , using the n goods as input. The technology is specified by a concave function $f _ { k } : \mathbf { R } _ { + } ^ { n } \to \mathbf { R } _ { + }$ that is assumed to be homogeneous of degree 1. The interpretation is that given quantity $z _ { j } \geq 0$ of good $j$ , for $1 \leq j \leq n$ , the technology can produce up to $f _ { k } ( z _ { 1 } , \ldots , z _ { n } )$ units of good $o _ { k }$ .

At a given price vector $\pi = ( \pi _ { 1 } , \ldots , \pi _ { n } ) \in \mathbf { R } _ { + } ^ { n }$ , the producer will choose a technologically feasible production plan that maximizes her profit. That is, she will choose $z _ { 1 } , \ldots , z _ { n } \geq 0$ that maximizes the profit $\begin{array} { r } { \pi _ { o _ { k } } f _ { k } ( z _ { 1 } , . . . , z _ { n } ) - \sum _ { j = 1 } ^ { n } \pi _ { j } z _ { j } } \end{array}$ . Now if there is a choice of nonnegative $z _ { 1 } , \ldots , z _ { n }$ such that $\begin{array} { r } { \pi _ { o _ { k } } f _ { k } ( z _ { 1 } , . . . , \bar { z } _ { n } ) - \sum _ { j = 1 } ^ { n } \pi _ { j } z _ { j } > 0 } \end{array}$ then using inputs $\alpha z _ { 1 } , \ldots , \alpha z _ { n }$ , for $\alpha > 1$ , she can obtain a profit of

$$
\pi_ {o _ {k}} f _ {k} (\alpha z _ {1}, \dots , \alpha z _ {n}) - \sum_ {j = 1} ^ {n} \pi_ {j} \alpha z _ {j} = \alpha \left(\pi_ {o _ {k}} f _ {k} (z _ {1}, \dots , z _ {n}) - \sum_ {j = 1} ^ {n} \pi_ {j} z _ {j}\right).
$$

Thus a profit-maximizing plan is not defined in this case. A profit-maximizing plan is defined if and only if no feasible plan can make a strictly positive profit. In such a case, a profit-maximizing plan is one that makes zero profit. In particular, the trivial choice $z _ { j } = 0$ , for $1 \leq j \leq n$ , for which $f _ { k } ( z _ { 1 } , \dots , z _ { n } ) = 0$ is always a profit-maximizing plan whenever profit maximization is well defined.

It is useful to restate the above in terms of the unit cost function $c _ { k } : \mathbf { R } _ { + } ^ { n } \to \mathbf { R } _ { + }$ This is defined, at any given price vector $( \pi _ { 1 } , \ldots , \pi _ { n } ) \in \mathbf { R } _ { + } ^ { n }$ , to be the minimum cost for producing one unit of good $o _ { k }$ . That is,

$$
c _ {k} (\pi) = \min \left\{\sum_ {j = 1} ^ {n} \pi_ {j} z _ {j} | z _ {j} \geq 0, f _ {k} (z _ {1}, \dots , z _ {n}) \geq 1 \right\}.
$$

If $\pi _ { o _ { k } } > c _ { k } ( \pi )$ , then profit maximization is undefined. If $\pi _ { o _ { k } } < c _ { k } ( \pi )$ , then the only profit-maximizing plan is the trivial plan. If $\pi _ { o _ { k } } = c _ { k } ( \pi )$ , the trivial plan, as well as any $( x _ { 1 } , \ldots , x _ { n } )$ such that $\begin{array} { r } { f _ { k } ( z _ { 1 } , . . . , z _ { n } ) c _ { k } ( \pi ) = \sum _ { i = 1 } ^ { n } \pi _ { j } z _ { j } } \end{array}$ , is a profit-maximizing plan. Each consumer is identical to the one in the exchange model: she has an initial endowment $w _ { i } \in \mathbf { R } _ { + } ^ { n }$ and a utility function $u _ { i }$ , which we now assume to be homogeneous. An equilibrium is a price vector $\pi = ( \pi _ { 1 } , \ldots , \pi _ { n } )$ at which there is a bundle $x _ { i } = ( x _ { i 1 } , \ldots , x _ { i n } ) \in \mathbf { R } _ { + } ^ { n }$ of goods for each trader i and a bundle $z _ { k } = ( z _ { k 1 } , \dots , z _ { k n } ) \in$ $\mathbf { R } _ { + } ^ { n }$ for each producer $k$ such that the following three conditions hold: (i) For each firm $k ,$ profit maximization is well-defined at $\pi$ and the inputs $z _ { k } = ( z _ { k 1 } , \dots , z _ { k n } )$ and output $q _ { k o _ { k } } = f _ { k } ( z _ { k 1 } , . . . , z _ { k n } )$ is a profit-maximizing plan; (ii) for each consumer $i ,$ , the vector $x _ { i }$ is her demand at price $\pi .$ ; and (iii) for each good $j ,$ , the total demand is no more than the total supply; i.e., the market clears:

$$
\sum_ {i} x _ {i j} + \sum_ {k} z _ {k j} \leq \sum_ {i} w _ {i j} + \sum_ {k: j = o _ {k}} q _ {k j}.
$$

Note that requirement (i) means that there is no feasible plan that makes positive profit. This rules out the trivial approach of ignoring the production units and computing an equilibrium for the resulting exchange model.

We now derive a convex program for certain kinds ofutility and production functions. We first transform the economy M into an economy $M ^ { \prime }$ with m consumers, $n + m$ goods, and $l + m$ producers. For each consumer i, an additional good, which will be the $( n + i )$ -th good, is added. The new utility function of the i-th consumer is $u _ { i } ^ { \prime } ( x _ { 1 } , \ldots , x _ { n + m } ) = x _ { n + i }$ ; that is, the i-th consumer wants only good $n + i$ . The new initial endowment $w _ { i } ^ { \prime }$ is the same as the old one; that is $w _ { i j } ^ { \prime } = w _ { i j } \mathrm { i f } j \le n$ , and $w _ { i j } ^ { \prime } = 0$ if $j > n$ . The first $l$ producers stay the same. That is, for $k \leq l .$ , the k-th producer outputs good $o _ { k }$ using the technology described by the function $f _ { k } ^ { \prime } ( z _ { 1 } , \ldots , z _ { n + m } ) =$ $f _ { k } ( z _ { 1 } , \ldots , z _ { n } )$ . For $1 \leq i \leq m$ , the $( l + i )$ )-th producer outputs good $n + i$ using the technology described by the function $f _ { l + i } ^ { \prime } ( z _ { 1 } , \ldots , z _ { n + m } ) = u _ { i } ( z _ { 1 } , \ldots , z _ { n } )$ . It can be shown that there is a one-to-one correspondence between the equilibria of M and $M ^ { \prime }$ We will therefore focus on characterizing the equilibria of $M ^ { \prime } -$ the simplicity of its consumption side will be of considerable help in this task.

## 6.6.1 Inequalities Characterizing Equilibrium

We begin by characterizing the equilibria for the market $M ^ { \prime }$ in terms of a system G of inequalities, in the following sets of nonnegative variables: (1) $\pi _ { 1 } , \ldots , \pi _ { n + m }$ for the prices; $( 2 ) \ x _ { i , n + i }$ , for the demand of consumer i for the $( n + i )$ )-th good; (3) $z _ { k } = ( z _ { k 1 } , \ldots , z _ { k n } ) \in \mathbf { R } _ { + } ^ { n }$ , standing for the inputs used by the k-th production sector; and $( 4 ) q _ { k o _ { k } }$ , for the output of the good $o _ { k }$ by the k-th producer.

$$
\pi_ {n + i} x _ {i, n + i} \geq \sum_ {j = 1} ^ {n} \pi_ {j} w _ {i j}, \text {   for   } 1 \leq i \leq m\tag{6.10}
$$

$$
q _ {k o _ {k}} \leq f _ {k} (z _ {k}), \text {   for   } 1 \leq k \leq l + m\tag{6.11}
$$

$$
\pi_ {o _ {k}} \leq c _ {k} (\pi_ {1}, \dots , \pi_ {n}), \text {   for   } 1 \leq k \leq l + m\tag{6.12}
$$

$$
\sum_ {k} z _ {k j} \leq \sum_ {i} w _ {i j} + \sum_ {k: o _ {k} = j} q _ {k j}, \text {   for   } 1 \leq j \leq n\tag{6.13}
$$

$$
x _ {i, n + i} \leq q _ {l + i, n + i} \mathrm{for} 1 \leq i \leq m\tag{6.14}
$$

Here, $c _ { k } ( )$ denotes the k-th producer’s unit cost function, which depends only on the prices of the first n goods. Evidently, any equilibrium is a feasible solution to the system of inequalities G. What is not so evident is that any feasible solution of G is an equilibrium. To see this, we first note that the sets of inequalities (6.12) and (6.13) imply that no producer can make positive profit: we have $\begin{array} { r } { \sum _ { j \leq n } \pi _ { j } z _ { k j } \geq \pi _ { o _ { k } } q _ { k o _ { k } } } \end{array}$ for each producer k. Adding up these inequalities, as well as the inequalities (6.10), we get a certain inequality that says that the cost of the consumer and producer demands is greater than or equal to the cost of the initial endowments and producer outputs. Whereas by multiplying each inequality in (6.13) and (6.14) by the corresponding price and adding up these inequalities, we get that the cost of the consumer and producer demands is less than or equal to the cost of the initial endowments and producer outputs.

This implies that the two costs must be equal. From this it follows that $\begin{array} { r } { \sum _ { j \leq n } \pi _ { j } z _ { k j } = } \end{array}$ $\pi _ { o _ { k } } q _ { k o _ { k } }$ for each producer k. Each production plan makes zero profit. Since (6.12) ensures that profit maximization is well defined, these are optimal production plans. Furthermore, we must have equality in (6.10): $x _ { i , n + i }$ is the demand of good $n + i$ at price π. Since conservation of goods is guaranteed by (6.13) and (6.14), we conclude that any solution of G is an equilibrium.

## 6.6.2 Convex Programs for Specific Functions

Let us make the substitution $\pi _ { j } = e ^ { \psi _ { j } }$ in the system of inequalities above. This makes all the constraints convex, except possibly for the ones in (6.12). Whenever each inequality in the set (6.13) also becomes a convex constraint, we get a convex feasibility characterization of the equilibrium prices.

Let us first consider what happens to the constraint in (6.12) corresponding to a CES production function $\begin{array} { r } { f _ { k } ( z _ { 1 } , . . . , z _ { n } ) = ( \sum _ { j } a _ { k j } x _ { j } ^ { \rho } ) ^ { 1 / \rho } } \end{array}$ , where $0 < \rho < 1$ . The corresponding constraint is $\begin{array} { r } { \pi _ { o _ { k } } \leq c _ { k } ( \pi ) = ( \sum _ { i } a _ { k i } ^ { \sigma } \pi _ { i } ^ { 1 - \sigma } ) ^ { 1 / 1 - \sigma } } \end{array}$ , where $\sigma = 1 / ( 1 - \rho )$ (we use a standard expression for the cost function corresponding to the CES production function $f _ { k } )$ . Raising both sides to the power $( 1 - \sigma )$ , and noting that $1 - \sigma < 0$ , this constraint becomes

$$
\pi_ {o _ {k}} ^ {1 - \sigma} \geq \left(\sum_ {j} a _ {k j} ^ {\sigma} \pi_ {j} ^ {1 - \sigma}\right).
$$

It is now easy to see that the substitution $\pi _ { j } = e ^ { \psi _ { j } }$ turns this inequality into a convex constraint.

It is also easy to verify, using standard formulas for the cost functions, that the constraint in (6.12) corresponding to a linear or a Cobb–Douglas production function also becomes convex after the substitution $\pi _ { j } = e ^ { \psi _ { j } }$

Thus, we obtain convex programs characterizing the equilibria in constant returns economies where the utility and production functions are linear, Cobb–Douglas, or CES with $\rho > 0$ . The approach also works for a certain family of nested CES functions. Interestingly, the use of production technologies to simplifying the consumption side plays a key role in obtaining convex programs for pure exchange economies with nested CES utility functions.

## 6.7 Bibliographic Notes

The convex program of Section 6.2 is due to Eisenberg (1961). Generalizing an approach due to Eisenberg and Gale (1959) and Gale (1960) for linear utilities, Eisenberg (1961) shows how to write the equilibrium conditions for the Fisher model as the solution to a convex program whenever the traders have homogeneous utility functions.

Eisenberg’s program can also be seen as following from Negishi’s theorem. However Eisenberg establishes an arguably stronger result. Without loss of generality, assume $\textstyle \sum _ { i } e _ { i } = 1$ . Consider the social utilityfunction u : ${ \bf R } _ { + } ^ { n }  R$ that assigns to each $s \in \mathbf { R } _ { + } ^ { n }$ the value

$$
\max \left\{\prod_ {i = 1} ^ {m} u _ {i} (x _ {i}) ^ {e _ {i}} \mid x _ {i} \in \mathbf {R} _ {+} ^ {n}, \sum_ {i} x _ {i} \leq s \right\}.
$$

Eisenberg shows that u is homogeneous and concave, and that at any price vector π the market demand generated by the Fisher economy with m traders is identical to the demand of a single trader with utility function u and income 1.

Polterovich (1973) extends Eisenberg’s program to a generalization of the Fisher model that includes production. Jain et al. (2005) generalize this result to quasi-concave, homothetic, utilities, and also consider economies of scale in production.

Lemma 6.4 of Section 6.3 has been proven by Arrow et al. (1959) under the stronger assumption of GS. It was later shown to generalize to markets which satisfy only WGS (Arrow and Hurwicz, 1960a, 1960b).

Polterovich and Spivak (1983) extended the characterization of Lemma 6.4 to scenarios where the demand is a set-valued function of the prices, which includes in particular the exchange model with linear utilities. This extension says that for any equilibrium price ˆπ, and nonequilibrium price π, and any vector $z \in \mathbf { R } ^ { n }$ that is chosen from the set of aggregate excess demands of the market at π, we have $\hat { \pi } \cdot z > 0$

The simple algorithm of Section 6.3.1, which is a discrete version of the tatonnementˆ process, is introduced and analyzed in Codenotti et al. (2005). Lemma 6.7 can also be used with the Ellipsoid method, as shown by Codenotti et al. (2005), to compute a weak (1 + ε)-approximate equilibrium in polynomial time. That is, the dependence of the running time on $\frac { 1 } { \varepsilon }$ can be made polynomial in log $\frac { 1 } { \varepsilon }$ .

The simple algorithm of Section 6.3.1, which is a discrete version of the tatonnementˆ process, is introduced and analyzed in Codenotti et al. (2005).

The convex feasibility program ofSection 6.4.1 is due to Nenakov and Primak (1983) and Jain (2004). For linear utilities, an equilibrium price vector whose components are small rational numbers exists. Jain (2004) proposes a variant of the Ellipsoid algorithm that, exploiting this, uses the separation hyperplane implied by the convex program to compute the equilibrium exactly in polynomial time. Ye (in press) presents an efficient interior-point algorithm that computes the exact equilibrium in polynomial time. The convex program of Section 6.4.2 has been introduced in Codenotti et al. (2005).

Section 6.5.1 describes a market with two traders and two goods that has multiple disconnected equilibria. Such example has been proposed by Gjerstad (1996).

The class PPAD introduced in Section 6.5.2 was defined by Papadimitriou (1994). The game-market correspondence was shown in Codenotti et al. (2006). The PPAD completeness of the computation of a Nash equilibrium for a bimatrix game is due to Chen and Deng (2005b). Chen and Deng’s result came after a sequence of works, where first the PPAD-completeness of 4-player games (Daskalakis et al., 2005), and then of 3-player games (Chen and Deng, 2005a; Daskalakis and Papadimitriou, 2005) were proven.

The convex program of Section 6.6 has been introduced in Jain and Varadarajan (2006). We have not mentioned several other results on convex programs for production models. We refer the interested reader to Jain and Varadarajan (2006) and the references therein.

## Bibliography

K.J. Arrow, H.D. Block, and L. Hurwicz. On the stability of the competitive equilibrium, ii. Econo metrica, 27(1):82–109, 1959.

K.J. Arrow, H.B. Chenery, B.S. Minhas, and R.M. Solow. Capital–labor substitution and economic efficiency. Rev. Econ. Stat., 43(3):225–250, 1961.

K.J. Arrow and G. Debreu. Existence of an equilibrium for a competitive economy. Econometrica, 22(3):265–290, 1954.

K.J. Arrow and L. Hurwicz. Competitive stability under weak gross substitutability: The euclidean distance approach. Intl. Econ. Rev., 1:38–49, 1960a.

K.J. Arrow and L. Hurwicz. Some remarks on the equilibria of economic systems. Econometrica, 28:640–646, 1960b.

K.C. Border. Fixed point Theorems with Applications to Economics and Game Theory. Cambridge University Press, 1985.

X. Chen and X. Deng. 3-NASH is PPAD-complete. Electronic Collog. Computational Complexity, 2005a.

X. Chen and X. Deng. Settling the complexity of 2-player Nash-Equilibrium. Electronic Collog. Computational Complexity, 2005b.

B. Codenotti, B. McCune, S. Penumatcha, and K. Varadarajan. Market equilibrium for CES exchange economies: Existence, multiplicity, and computation. In Proc. 25th Intl. Conf. Fdns. Software Tech. Theoretical Comp. Sci., pp. 505–516, 2005.

B. Codenotti, B. McCune, and K. Varadarajan. Market equilibrium via the excess demand function. In Proc. 37th Annual ACM Symp. Theo. Comp., pp. 74–83, 2005.

B. Codenotti, S. Pemmaraju, and K. Varadarajan. On the polynomial time computation of equilibria for certain exchange economies. In Proc. 16th Annual ACM-SIAM Symp. Disc. Algo., pp. 72–81, 2005.

B. Codenotti, A. Saberi, K. Varadarajan, and Y. Ye. Leontief economies encode nonzero sum two player games. In Proc. 17th Annual ACM-SIAM Symp. Disc. Algo., pp. 659–667, 2006.

C. Daskalakis, P. Goldberg, and C. Papadimitriou. The complexity of computing a Nash equilibrium. Electronic Collog. Computational Complexity, 2005.

C. Daskalakis and C. Papadimitriou. Three-player games are hard. Electronic Collog. Computational Complexity, 2005.

X. Deng, C. Papadimitriou, and S. Safra. On the complexity of price equilibrium. J. Comp. Syst. Sci., 67(2):311–324, 2003. (Special Issue on Symp. Theory of Computing, 2002).

B.C. Eaves and H. Scarf. The solution of systems of piecewise linear equations. Math. Oper. Res., 1(1):1–27, 1976.

E. Eisenberg. Aggregation of utility functions. Mgmt. Sci., 7(4):337–350, 1961.

E. Eisenberg and D. Gale. Consensus of subjective probabilities: The pari-mutuel method. Annals Math. Stat., 30:165–168, 1959.

D. Gale. The Theory ofLinear Economic Models. McGraw Hill, 1960.

S. Gjerstad. Multiple equilibria in exchange economies with homothetic, nearly identical preference. University of Minnesota, Center for Economic Research, Discussion Paper 288, 1996.

T. Hansen and H. Scarf. The Computation of Economic Equilibria. Cowles Foundation Monograph No. 24., New Haven: Yale University Press, 1973.

K. Jain. A polynomial time algorithm for computing the Arrow–Debreu market equilibrium for linear utilities. In Proc. 45th Annual Symp. Fdns. Comp. Sci., pp. 286–294, 2004.

K. Jain, M. Mahdian, and A. Saberi. Approximating market equilibria. In Proc. RANDOM-APPROX, pp. 98–108, 2003.

K. Jain and K. Varadarajan. Equilibria for economies with production: Constant-returns technologies and production planning constraints. In SODA 06: Proc. 17th Annual ACM-SIAM Symp. Disc. Algo., pp. 688–697, 2006.

K. Jain, V.V. Vazirani, and Y. Ye. Market equilibria for homothetic, quasi-concave utilities and economies of scale in production. In SODA 05: Proc. 16th Annual ACM-SIAM Symp. on Discrete Algorithms, pp. 63–71, 2005.

O.L. Mangasarian. Nonlinear Programming. McGraw-Hill, 1969.

A. Mas-Colell, M.D. Whinston, and J.R. Green. Microeconomic Theory. Oxford University Press, 1995.

T. Negishi. Welfare economics and existence of an equilibrium for a competitive economy. Metroe conomica, 12:92–97, 1960

E.I. Nenakov and M.E. Primak. One algorithm for finding solutions of the Arrow-Debreu model. Kibernetica, 3:127–128, 1983.

C.H. Papadimitriou. On the complexity of the parity argument and other inefficient proofs of existence. J. Comp. Syst. Sci., 48:498–532, 1994.

C.H. Papadimitriou. Algorithms, games, and the Internet. In Proc. 33rd Annual ACM Symp. Theo. Comp., pp. 749–753, 2001.

C.H. Papadimitriou. Algorithms for equilibria. In Algorithmic Game Theory, Chapter 2. Cambridge University Press, 2007.

V.M. Polterovich. Economic equilibrium and the optimum. Matekon, 5:3–20, 1973.

V.M. Polterovich and V.A. Spivak. Gross substitutability of point to set correspondences. J. Math. Econ., 11(2):117–140, 1983.

T. Rutherford. Sequential joint maximization. In J. Weyant Ed. Energy and Environmental Policy Modeling. Intl. Series Oper. Res. Mgmt. Sci., 18, 1999.

P.A. Samuelson. Foundations ofEconomic Analysis. Harvard University Press, 1947.

H. Scarf. The approximation of fixed points of a continuous mapping. SIAM J. Appl. Math., 15(1):1328–1343,1967

H. Scarf. The computation of equilibrium prices: An exposition. In Handbook of Mathematical Economics, Volume II, pp. 1008–1061, 1982.

H. Varian. Microeconomic Analysis. W.W. Norton, 1992.

V. Vazirani. Combinatorial algorithms for market equilibria. In Algorithmic Game Theory, Chapte 5. Cambridge University Press, 2007.

A. Wald. On some systems of equations of mathematical economics. Econometrica, 19(4):368–403, 1951. Original version: Zeitschrift fur National¨ okonomie, Vol. 7 (1936).¨

L. Walras. Elements ofPure Economics, or the Theory ofSocial Wealth. Richard Irwin, 1954. (Origina version published in French in 1874).

Y. Ye. A path to the Arrow–Debreu competitive market equilibrium. Math Progr.. In press.

## Exercises

6.1 Use the Karush–Kuhn–Tucker conditions to derive an explicit expression for the demand of a consumer with a Cobb–Douglas utility function. Also derive formula 6.7, the expression for the demand with a CES function.

6.2 Show that for an exchange economy with Cobb–Douglas utility functions, the positive equilirbium prices can be characterized as solutions to a linear feasibility program with variables for the prices. The number of constraints of the program must be polynomial in the number of traders and goods.

6.3 Prove that Lemma 6.4 implies that the set of equilibrium prices is convex.

6.4 Prove parts (2), (3), and (4) of Lemma 6.5.

6.5 Suppose that π and ˆπ are two price vectors such that max $\begin{array} { r } { \frac { \pi _ { j } } { \hat { \pi } _ { j } } \leq ( 1 + \varepsilon / 3 ) \operatorname* { m i n } _ { j } \frac { \pi _ { j } } { \hat { \pi } _ { j } } } \end{array}$ and ˆπ is an equilibrium. Show that π is a weak (1 + ε)-approximate equilibrium.
