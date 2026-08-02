---
title: "approximation-algorithms-ch-09-part-010"
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
pdf_path: "work/core-books/approximation-algorithms/parts/approximation-algorithms-ch-09-part-010.pdf"
raw_md: "raw/canonical/approximation-algorithms-ch-09-part-010/full.md"
---
Consider the following problem.

Problem 9.1 (Bin packing) Given n items with sizes $a _ { 1 } , \ldots , a _ { n } \in ( 0 , 1 ]$ find a packing in unit-sized bins that minimizes the number of bins used.

This problem finds many industrial applications. For instance, in the stock-cutting problem, bins correspond to a standard length of paper and items correspond to specified lengths that need to be cut.

It is easy to obtain a factor 2 approximation algorithm for this problem. For instance, let us consider the algorithm called First-Fit. This algorithm considers items in an arbitrary order. In the ith step, it has a list of partially packed bins, say $B _ { 1 } , \ldots , B _ { k }$ . It attempts to put the next item, ${ { a } _ { i } } ,$ in one of these bins, in this order. If $a _ { i }$ does not fit into any of these bins, it opens a new bin $B _ { k + 1 }$ , and puts $a _ { i }$ in it. If the algorithm uses m bins, then at least $m - 1$ bins are more than half full. Therefore,

$$
\sum_ {i = 1} ^ {n} a _ {i} > \frac {m - 1}{2}.
$$

Since the sum of the item sizes is a lower bound on OPT, $m - 1 < 2 \mathrm { O P T }$ • $\mathrm { i . e . , } m \leq 2 \mathrm { O P T }$ (see Notes for a better analysis). On the negative side:

Theorem 9.2 For any $\varepsilon > 0$ , there is no approximation algorithm having a guarantee of $3 / 2 - \varepsilon$ for the bin packing problem, assuming $\mathbf { P } \neq \mathbf { N P }$

Proof: If there were such an algorithm, then we show how to solve the NPhard problem of deciding if there is a way to partition n nonnegative numbers $a _ { 1 } , \ldots , a _ { n }$ into two sets, each adding up to ${ \frac { 1 } { 2 } } \sum _ { i } a _ { i }$ . Clearly, the answer to this question is ‘yes’ if the n items can be packed in 2 bins of size ${ \frac { 1 } { 2 } } \sum _ { i } a _ { i }$ . If the answer is ‘yes’ the $3 / 2 - \varepsilon$ factor algorithm will have to give an optimal packing, and thereby solve the partitioning problem. ✷

## 9.1 An asymptotic PTAS

Notice that the argument in Theorem 9.2 uses very special instances: those for which $\mathrm { O P T }$ is a small number, such as 2 or 3, even though the number of items is unbounded. What can we say about “typical” instances, those for which $\mathrm { O P T }$ increases with n?

Theorem 9.3 For any ε, $0 < \varepsilon \le 1 / 2$ , there is an algorithm A that runs in time polynomial in n and finds a packing using at most $( 1 + 2 \varepsilon ) \mathrm { O P T } + 1$ bins.

The sequence of algorithms, $A _ { \varepsilon } ,$ , form an asymptotic polynomial time approximation scheme for bin packing, since for each $\varepsilon > 0 \exists N > 0$ , and a polynomial time algorithm in this sequence, say $B ,$ such that B has an approximation guarantee of $1 + \varepsilon$ for all instances having $\mathrm { O P T } \geq N$ . However, Theorem 9.3 should not be considered a practical solution to the bin packing problem, since the running times of the algorithms $\mathcal { A } _ { \varepsilon }$ are very high.

We will prove Theorem 9.3 in three steps.

Lemma 9.4 Let $\varepsilon > 0$ be fixed, and let K be a fixed nonnegative integer. Consider the restriction of the bin packing problem to instances in which each item is of size at least ε and the number of distinct item sizes is K. There is a polynomial time algorithm that optimally solves this restricted problem.

Proof: The number of items in a bin is bounded by $\lfloor 1 / \varepsilon \rfloor$ . Denote this by M. Therefore, the number of diferent bin types is bounded by $R = \bigl ( \begin{array} { c } { { M + K } } \\ { { M } } \end{array} \bigr )$  (see Exercise 9.4), which is a (large!) constant. Clearly, the total number of bins used is at most n. Therefore, the number of possible feasible packings is bounded by $P = \left( { n + R } \atop { R } \right)$ , which is polynomial in n (see Exercise 9.4).  Enumerating them and picking the best packing gives the optimal answer. ✷

Lemma 9.5 Let $\varepsilon > 0$ be fixed. Consider the restriction of the bin packing problem to instances in which each item is of size at least ε. There is a polynomial time approximation algorithm that solves this restricted problem within a factor of $( 1 + \varepsilon )$

Proof: Let I denote the given instance. Sort the n items by increasing size, and partition them into $\bar { K } = \lceil 1 / \varepsilon ^ { 2 } \rceil$ groups each having at most $Q = \lfloor n \varepsilon ^ { 2 } \rfloor$ items. Notice that two groups may contain items of the same size.

![](images/8323af89e00c8be3aca25bd0e1bc3ff819caf92fc84b00be2ab0ee974bd91d51.jpg)

Construct instance J by rounding up the size of each item to the size of the largest item in its group. Instance J has at most K diferent item sizes.

Therefore, by Lemma $9 . 4$ , we can find an optimal packing for J. Clearly, this will also be a valid packing for the original item sizes. We show below that $\mathrm { O P T } ( J ) \leq ( 1 + \varepsilon ) \mathrm { O P T } ( I )$ , thereby proving the lemma.

The following clever argument accomplishes this. Let us construct another instance, say $J ^ { \prime } { } _ { ; }$ , by rounding down the size of each item to that of the smallest item in its group. Clearly $\mathrm { O P T } ( J ^ { \prime } ) \leq \mathrm { O P T } ( I )$ . The crucial observation is that a packing for instance $J ^ { \prime }$ yields a packing for all but the largest $Q$ items of instance $J$ (Exercise 9.6 asks for a formal proof). Therefore,

$$
\operatorname{OPT} (J) \leq \operatorname{OPT} (J ^ {\prime}) + Q \leq \operatorname{OPT} (I) + Q.
$$

Since each item in I has size at least $\varepsilon , \operatorname { O P T } ( I ) \geq n \varepsilon$ . Therefore, $Q = \lfloor n \varepsilon ^ { 2 } \rfloor \leq$ $\varepsilon \mathrm { O P T }$ . Hence, $\mathrm { O P T } ( J ) \leq ( 1 + \varepsilon ) \mathrm { O P T } ( I )$ ✷

Proof of Theorem 9.3: Let I denote the given instance, and $I ^ { \prime }$ denote the instance obtained by discarding items of size $< \varepsilon$ from $I .$ By Lemma $9 . 5 ^ { }$ , we can find a packing for $I ^ { \prime }$ using at most $( 1 + \varepsilon ) \mathrm { O P T } ( I ^ { \prime } )$ bins. Next, we start packing the small items (of size $< \varepsilon )$ in a First-Fit manner in the bins opened for packing $I ^ { \prime }$ . Additional bins are opened if an item does not fit into any of the already open bins.

If no additional bins are needed, then we have a packing in $( 1 + \varepsilon ) \mathrm { O P T } ( I ^ { \prime } ) \leq$ $( 1 + \varepsilon ) \mathrm { O P T } ( I )$ bins. In the second case, let M be the total number of bins used. Clearly, all but the last bin must be full to the extent of at least $1 - \varepsilon$ Therefore, the sum of the item sizes in I is at least $( M - 1 ) ( 1 - \varepsilon )$ . Since this is a lower bound on OPT, we get

$$
M \leq \frac {\mathrm{OPT}}{(1 - \varepsilon)} + 1 \leq (1 + 2 \varepsilon) \mathrm{OPT} + 1,
$$

where we have used the assumption that $\varepsilon \le 1 / 2$ . Hence, for each value of $\varepsilon ,$ $0 < \varepsilon \le 1 / 2$ , we have a polynomial time algorithm achieving a guarantee of $( 1 + 2 \varepsilon ) \mathrm { O P T } + 1$ ✷

Algorithm $\mathcal { A } _ { \varepsilon }$ is summarized below.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 9.6 (Algorithm $\mathcal{A}_{\varepsilon}$ for bin packing)  
1. Remove items of size $&lt; \varepsilon$.  
2. Round to obtain constant number of item sizes (Lemma 9.5).  
3. Find optimal packing (Lemma 9.4).  
4. Use this packing for original item sizes.  
5. Pack items of size $&lt; \varepsilon$ using First-Fit.
</div>

## 9.2 Exercises

9.1 Give an example on which First-Fit does at least as bad as $5 / 3 \cdot \mathrm { O P T }$

9.2 (Johnson [149]) Consider a more restricted algorithm than First-Fit, called Next-Fit, which tries to pack the next item only in the most recently started bin. If it does not fit, it is packed in a new bin. Show that this algorithm also achieves factor 2. Give a factor 2 tight example.

9.3 (C. Kenyon) Say that a bin packing algorithm is monotonic if the number of bins it uses for packing a subset of the items is at most the number of bins it uses for packing all n items. Show that whereas Next-Fit is monotonic, First-Fit is not.

9.4 Prove the bounds on R and P stated in Lemma 9.4.

Hint: Use the fact that the number of ways of throwing n identical balls into k distinct bins is $\binom { n + k - 1 } { n }$ .

9.5 Consider an alternative way of establishing Lemma 9.5. All items having sizes in the interval $( \varepsilon ( 1 + \varepsilon ) ^ { r } , \varepsilon ( 1 + \varepsilon ) ^ { r + 1 } ]$ are rounded up to $\operatorname* { m i n } ( \varepsilon ( 1 +$ $\varepsilon ) ^ { r + 1 } , 1 )$ , for $r \geq 0$ . Clearly, this yields a constant number of item sizes. Does the rest of the proof $_ \mathrm { g o }$ through?

Hint: Consider the situation that there are lots of items of size $1 / 2$ and $1 / 2 \neq \varepsilon ( 1 + \varepsilon ) ^ { r }$ for any $r \geq 0$

9.6 Prove the following statement made in Lemma 9.5, “A packing for instance $J ^ { \prime }$ yields a packing for all but the largest $Q$ items of instance $J . ^ { \ : \mathfrak { n } }$ Hint: Throw away the $Q$ largest items of J and the $Q$ smallest items of $J ^ { \prime } { } _ { ; }$ and establish a domination.

9.7 Use the fact that integer programming with a fixed number of variables is in P to give an alternative proof of Lemma 9.4. (Because of the exorbitant running time of the integer programming algorithm, this variant is also impractical.)

9.8 Show that if there is an algorithm for bin packing having a guarantee of $\mathrm { O P T } ( I ) + \log ^ { 2 } ( \mathrm { O P T } ( I ) )$ , then there is a fully polynomial approximation scheme for this problem.

## 9.9 (C. Kenyon) Consider the following problem.

Problem 9.7 (Bin covering) Given n items with sizes $a _ { 1 } , \ldots , a _ { n } \in ( 0 , 1 ]$ • maximize the number of bins opened so that each bin has items summing to at least 1.

Give an asymptotic PTAS for this problem when restricted to instances in which item sizes are bounded below by c, for a fixed constant $c > 0$ Hint: The main idea of Algorithm 9.6 applies to this problem as well.

## 9 Bin Packing

## 9.3 Notes

The first nontrivial bin packing result, showing that First-Fit requires at most $( 1 7 / 1 0 ) \mathrm { O P T + 3 }$ bins, was due to Ullman [248]. The asymptotic PTAS is due to Fernandez de la Vega and Lueker [86]. An improved algorithm, having a guarantee of $\operatorname { O P T } ( I ) + \log ^ { 2 } ( \operatorname { O P T } ( I ) )$ was given by Karmarkar and Karp [163]. For further results, see the survey of Cofman, Garey, and Johnson [50]. The result cited in Exercise 9.7, showing that integer programming with a fixed number of variables is in P, is due to Lenstra [185]. Bin packing has also been extensively studied in the on-line model. For these and other on-line algorithms see Borodin and El-Yaniv [31].