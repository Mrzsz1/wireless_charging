---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-08"
chapter_number: 8
chapter_title: "Knapsack"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 86
source_page_end: 91
printed_page_start: 68
printed_page_end: 73
part_ids: ["approximation-algorithms-ch-08-part-009"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Knapsack (MinerU semantic layer)

<!-- source-pages: 86-91; printed-pages: 68-73; mineru-part: approximation-algorithms-ch-08-part-009 -->

## 8 Knapsack

In Chapter 1 we mentioned that some NP-hard optimization problems allow approximability to any required degree. In this chapter, we will formalize this notion and will show that the knapsack problem admits such an approximability.

Let Π be an NP-hard optimization problem with objective function $f _ { \pi }$ We will say that algorithm A is an approximation scheme for Π if on input $( I , \varepsilon )$ , where I is an instance of Π and $\varepsilon > 0$ is an error parameter, it outputs a solution s such that:

$f _ { \pi } ( I , s ) \leq ( 1 + \varepsilon ) \cdot \mathrm { O P T }$ if Π is a minimization problem.

$f _ { \pi } ( I , s ) \geq ( 1 - \varepsilon )$ · OPT if Π is a maximization problem.

A will be said to be a polynomial time approximation scheme, abbreviated PTAS, if for each fixed $\varepsilon > 0$ , its running time is bounded by a polynomial in the size of instance I.

The definition given above allows the running time of A to depend arbitrarily on ε. This is rectified in the following more stringent notion of approximability. If the previous definition is modified to require that the running time of A be bounded by a polynomial in the size of instance I and $1 / \varepsilon$ , then A will be said to be a fully polynomial approximation scheme, abbreviated FPTAS.

In a very technical sense, an FPTAS is the best one can hope for an NPhard optimization problem, assuming $\mathbf { P } \neq \mathbf { N P } ;$ see Section 8.3.1 for a short discussion on this issue. The knapsack problem admits an FPTAS.

Problem 8.1 (Knapsack) Given a set $S = \{ a _ { 1 } , \ldots , a _ { n } \}$ of objects, with specified sizes and profits, size $( a _ { i } ) \in \mathbf { Z } ^ { + }$ and profi ${ \bf \Xi } ( a _ { i } ) \in { \bf Z } ^ { + }$ , and a “knapsack capacity” $B \in \mathbf { Z } ^ { + }$ , find a subset of objects whose total size is bounded by B and total profit is maximized.

An obvious algorithm for this problem is to sort the objects by decreasing ratio of profit to size, and then greedily pick objects in this order. It is easy to see that as such this algorithm can be made to perform arbitrarily badly (Exercise 8.1).

## 8.1 A pseudo-polynomial time algorithm for knapsack

Before presenting an FPTAS for knapsack, we need one more concept. For any optimization problem Π, an instance consists of objects, such as sets or graphs, and numbers, such as cost, profit, size, etc. So far, we have assumed that all numbers occurring in a problem instance I are written in binary. The size of the instance, denoted $| \bar { I | } .$ was defined as the number of bits needed to write I under this assumption. Let us say that $I _ { u }$ will denote instance I with all numbers occurring in it written in unary. The unary size of instance $I ,$ denoted $\left| I _ { u } \right|$ , is defined as the number of bits needed to write $I _ { u }$

An algorithm for problem Π is said to be eficient if its running time on instance I is bounded by a polynomial in |I|. Let us consider the following weaker definition. An algorithm for problem Π whose running time on instance I is bounded by a polynomial in $| I _ { u } |$ will be called a pseudo-polynomial time algorithm.

The knapsack problem, being NP-hard, does not admit a polynomial time algorithm; however, it does admit a pseudo-polynomial time algorithm. This fact is used critically in obtaining an FPTAS for it. All known pseudopolynomial time algorithms for NP-hard problems are based on dynamic programming.

Let P be the profit of the most profitable object, i.e., $P = \operatorname* { m a x } _ { a \in S }$ profit $( a )$ Then $n P$ is a trivial upperbound on the profit that can be achieved by any solution. For each $i \in \{ 1 , \ldots , n \}$ and $p \in \{ 1 , \ldots , n P \}$ , let $S _ { i , p }$ denote a subset of $\{ a _ { 1 } , \ldots , a _ { i } \}$ whose total profit is exactly p and whose total size is minimized. Let $A ( i , p )$ denote the size of the set $S _ { i , p } \ ( A ( i , p ) = \infty$ if no such set exists). Clearly $A ( 1 , p )$ is known for every $p \in \{ 1 , \ldots , n P \}$ . The following recurrence helps compute all values $A ( i , p )$ in $O ( n ^ { 2 } P )$ time:

$$
\begin{array}{l} A (i + 1, p) = \\ \left\{ \begin{array}{l l} \min \left\{A (i, p), \text {size} (a _ {i + 1}) + A (i, p - \text {profit} (a _ {i + 1})) \right\} & \text {if profit} (a _ {i + 1}) <   p \\ A (i + 1, p) = A (i, p) & \text {otherwise} \end{array} \right. \end{array}
$$

The maximum profit achievable by objects of total size bounded by B is max {p| $A ( n , p ) \leq B \}$ . We thus get a pseudo-polynomial algorithm for knapsack.

## 8.2 An FPTAS for knapsack

Notice that if the profits of objects were small numbers, i.e., they were bounded by a polynomial in n, then this would be a regular polynomial time algorithm, since its running time would be bounded by a polynomial in $| I |$ . The key idea behind obtaining an FPTAS is to exploit precisely this fact: we will ignore a certain number of least significant bits of profits of objects (depending on the error parameter $\varepsilon )$ , so that the modified profits can be viewed as numbers bounded by a polynomial in n and $1 / \varepsilon .$ . This will enable us to find a solution whose profit is at least (1 − ε) · OPT in time bounded by a polynomial in n and $1 / \varepsilon$

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 8.2 (FPTAS for knapsack)
1. Given $\varepsilon &gt; 0$, let $K = \frac{\varepsilon P}{n}$.
2. For each object $a_i$, define $\text{profit}'(a_i) = \left\lfloor \frac{\text{profit}(a_i)}{K} \right\rfloor$.
3. With these as profits of objects, using the dynamic programming algorithm, find the most profitable set, say $S'$.
4. Output $S'$.
</div>

## Lemma 8.3 Let A denote the set output by the algorithm. Then,

$$
\operatorname{profit} (A) \geq (1 - \varepsilon) \cdot \mathrm{OPT}.
$$

Proof: Let O denote the optimal set. For any object $^ { a , }$ because of rounding down, K · profit<sup></sup>(a) can be smaller than profit(a), but by not more than $K$ Therefore,

$$
\operatorname{profit} (O) - K \cdot \operatorname{profit} ^ {\prime} (O) \leq n K.
$$

The dynamic programming step must return a set at least as good as O under the new profits. Therefore,

$$
\begin{array}{c} \operatorname{profit} (S ^ {\prime}) \geq K \cdot \operatorname{profit} ^ {\prime} (O) \geq \operatorname{profit} (O) - n K = \operatorname{OPT} - \varepsilon P \\ \geq (1 - \varepsilon) \cdot \operatorname{OPT}, \end{array}
$$

where the last inequality follows from the observation that ${ \mathrm { O P T } } \geq P .$ ✷

Theorem 8.4 Algorithm 8.2 is a fully polynomial approximation scheme $f o r$ knapsack.

Proof: By Lemma 8.3, the solution found is within $( 1 - \varepsilon )$ factor of $\mathrm { O P T }$ Since the running time of the algorithm is $O \left( n ^ { 2 } \left\lfloor { \frac { P } { K } } \right\rfloor \right) = O \left( n ^ { 2 } \left\lfloor { \frac { n } { \varepsilon } } \right\rfloor \right)$ , which is polynomial in n and $1 / \varepsilon$ , the theorem follows. ✷

## 8.3 Strong NP-hardness and the existence of FPTAS’s

In this section, we will prove in a formal sense that very few of the known NPhard problems admit an FPTAS. First, here is a strengthening of the notion of NP-hardness in a similar sense in which a pseudo-polynomial algorithm is a weakening of the notion of an eficient algorithm. A problem Π is strongly NP-hard if every problem in NP can be polynomially reduced to Π in such a way that numbers in the reduced instance are always written in unary.

The restriction automatically forces the transducer to use polynomially bounded numbers only. Most known NP-hard problems are in fact strongly NP-hard; this includes all the problems in previous chapters for which approximation algorithms were obtained. A strongly NP-hard problem cannot have a pseudo-polynomial time algorithm, assuming $\mathbf { P } \neq \mathbf { N P }$ (Exercise 8.4). Thus, knapsack is not strongly NP-hard, assuming $\mathbf { P } \neq \mathbf { N P }$

We will show below that under some very weak restrictions, any NPhard problem admitting an FPTAS must admit a pseudo-polynomial time algorithm. Theorem 8.5 is proven for a minimization problem; a similar proof holds for a maximization problem.

Theorem 8.5 Let p be a polynomial and Π be an NP-hard minimization problem such that the objective function $f _ { \pi }$ is integer valued and on any instance $I , \mathrm { O P T } ( I ) < p ( | I _ { u } | )$ . If Π admits an $F P T A S$ , then it also admits a pseudo-polynomial time algorithm.

Proof: Suppose there is an FPTAS for Π whose running time on instance I and error parameter $\varepsilon { \mathrm { ~ i s ~ } } q ( | I | , 1 / \varepsilon )$ , where q is a polynomial.

On instance $I ,$ set the error parameter to $\varepsilon = 1 / p ( | I _ { u } | )$ , and run the FPTAS. Now, the solution produced will have objective function value less than or equal to:

$$
(1 + \varepsilon) \mathrm{OPT} (I) <   \mathrm{OPT} (I) + \varepsilon p (| I _ {u} |) = \mathrm{OPT} (I) + 1.
$$

In fact, with this error parameter, the FPTAS will be forced to produce an optimal solution. The running time will be $q ( | I | , p ( | I _ { u } | ) )$ , i.e., polynomial in $\left| I _ { u } \right|$ . Therefore, we have obtained a pseudo-polynomial time algorithm for Π. ✷

The following corollary applies to most known NP-hard problems.

Corollary 8.6 Let Π be an NP-hard optimization problem satisfying the restrictions of Theorem 8.5. If Π is strongly NP-hard, then Π does not admit an FPTAS, assuming $\mathbf { P } \neq \mathbf { N P }$

Proof: If Π admits an FPTAS, then it admits a pseudo-polynomial time algorithm by Theorem 8.5. But then it is not strongly NP-hard, assuming $\mathbf { P } \neq \mathbf { N P }$ , leading to a contradiction. ✷

The stronger assumption that $\mathrm { O P T } < p ( | I | )$ in Theorem 8.5 would have enabled us to prove that there is a polynomial time algorithm for Π. However, this stronger assumption is less widely applicable. For instance, it is not satisfied by the minimum makespan problem, which we will study in Chapter 10.

## 8.3.1 Is an FPTAS the most desirable approximation algorithm?

The design of almost all known FPTAS’s and PTAS’s is based on the idea of trading accuracy for running time – the given problem instance is mapped to a coarser instance, depending on the error parameter $\varepsilon ,$ which is solved optimally by a dynamic programming approach. The latter ends up being an exhaustive search of polynomially many diferent possibilities (for instance, for knapsack, this involves computing $A ( i , p )$ for all i and $p )$ . In most such algorithms, the running time is prohibitive even for reasonable n and ε. Further, if the algorithm had to resort to exhaustive search, does the problem really ofer “footholds” to home in on a solution eficiently? Is an FPTAS or PTAS the best one can hope for for an NP-hard problem? Clearly, the issue is complex and there is no straightforward answer.

## 8.4 Exercises

8.1 Consider the greedy algorithm for the knapsack problem. Sort the objects by decreasing ratio of profit to size, and then greedily pick objects in this order. Show that this algorithm can be made to perform arbitrarily badly.

8.2 Consider the following modification to the algorithm given in Exercise 8.1. Let the sorted order of objects be $a _ { 1 } , \ldots , a _ { n }$ . Find the lowest k such that the size of the first k objects exceeds B. Now, pick the more profitable of $\{ a _ { 1 } , \dotsc , a _ { k - 1 } \}$ and $\{ a _ { k } \}$ (we have assumed that the size of each object is at most $B )$ . Show that this algorithm achieves an approximation factor of 2.

8.3 (Bazgan, Santha, and Tuza [22]) Obtain an FPTAS for the following problem.

Problem 8.7 (Subset-sum ratio problem) Given n positive integers, $a _ { 1 } < \ldots < a _ { n }$ , find two disjoint nonempty subsets $S _ { 1 } , S _ { 2 } \subseteq \{ 1 , \dots , n \}$ with $\textstyle \sum _ { i \in S _ { 1 } } a _ { i } \geq \sum _ { i \in S _ { 2 } } a _ { i }$ , such that the ratio

$$
\frac {\sum_ {i \in S _ {1}} a _ {i}}{\sum_ {i \in S _ {2}} a _ {i}}
$$

is minimized.

Hint: First, obtain a pseudo-polynomial time algorithm for this problem. Then, scale and round appropriately.

8.4 Show that a strongly NP-hard problem cannot have a pseudo-polynomia time algorithm, assuming $\mathbf { P } \neq \mathbf { N P }$

## 8.5 Notes

Algorithm 8.2 is due to Ibarra and Kim [134]. Theorem 8.5 is due to Garey and Johnson [92].
