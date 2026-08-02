---
title: "approximation-algorithms-ch-15-part-016"
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
pdf_path: "work/core-books/approximation-algorithms/parts/approximation-algorithms-ch-15-part-016.pdf"
raw_md: "raw/canonical/approximation-algorithms-ch-15-part-016/full.md"
---
# 15 Set Cover via the Primal–Dual Schema

As noted in Section 12.3, the primal–dual schema is the method of choice for designing approximation algorithms since it yields combinatorial algorithms with good approximation factors and good running times. We will first present the central ideas behind this schema and then use it to design a simple f factor algorithm for set cover, where f is the frequency of the most frequent element.

The primal–dual schema has its origins in the design of exact algorithms. In that setting, this schema yielded the most eficient algorithms for some of the cornerstone problems in P, including matching, network flow, and shortest paths. These problems have the property that their LP-relaxations have integral optimal solutions. By Theorem 12.3 we know that optimal solutions to linear programs are characterized by fact that they satisfy all the complementary slackness conditions. In fact, the primal–dual schema for exact algorithms is driven by these conditions. Starting with initial feasible solutions to the primal and dual programs, it iteratively starts satisfying complementary slackness conditions. When they are all satisfied, both solutions must be optimal. During the iterations, the primal is always modified integrally, so that eventually we get an integral optimal solution.

Consider an LP-relaxation for an NP-hard problem. In general, the relaxation will not have an optimal solution that is integral. Does this rule out a complementary slackness condition driven approach? Interestingly enough, the answer is ‘no’. It turns out that the algorithm can be driven by a suitable relaxation of these conditions! This is the most commonly used way of designing primal–dual based approximation algorithms – but not the only way.

## 15.1 Overview of the schema

Let us consider the following primal program, written in standard form.

$$
\text { minimize } \quad \sum_ {j = 1} ^ {n} c _ {j} x _ {j}
$$

$$
\begin{array}{l l} \text { subject   to } & \sum_ {j = 1} ^ {n} a _ {i j} x _ {j} \geq b _ {i}, \quad i = 1, \ldots , m \\ & x _ {j} \geq 0, \quad j = 1, \ldots , n \end{array}
$$

where $a _ { i j } , b _ { i } ,$ , and $c _ { j }$ are specified in the input. The dual program is:

$$
\begin{array}{l l} \text {maximize} & \sum_ {i = 1} ^ {m} b _ {i} y _ {i} \\ \text {subject to} & \sum_ {i = 1} ^ {m} a _ {i j} y _ {i} \leq c _ {j}, \quad j = 1, \ldots , n \\ & y _ {i} \geq 0, \quad i = 1, \ldots , m \end{array}
$$

Most known approximation algorithms using the primal–dual schema run by ensuring one set of conditions and suitably relaxing the other. In the following description we capture both situations by relaxing both conditions. Eventually, if primal conditions are ensured, we set $\alpha = 1$ , and if dual conditions are ensured, we set $\beta = 1$

Primal complementary slackness conditions

Let $\alpha \geq 1 .$

For each $\begin{array} { r } { 1 \leq j \leq n \colon \mathrm { e i t h e r } \ x _ { j } = 0 \ \mathrm { o r } \ c _ { j } / \alpha \leq \sum _ { i = 1 } ^ { m } a _ { i j } y _ { i } \leq c _ { j } . } \end{array}$

Dual complementary slackness conditions

Let $\beta \geq 1$

For each $1 \leq i \leq m \colon$ either $\begin{array} { r } { y _ { i } = 0 \mathrm { ~ o r ~ } b _ { i } \le \sum _ { j = 1 } ^ { n } a _ { i j } x _ { j } \le \beta \cdot b _ { i } , } \end{array}$

Proposition 15.1 If x and y are primal and dual feasible solutions satisfying the conditions stated above then

$$
\sum_ {j = 1} ^ {n} c _ {j} x _ {j} \leq \alpha \cdot \beta \cdot \sum_ {i = 1} ^ {m} b _ {i} y _ {i}.
$$

Proof:

$$
\begin{array}{r l} & {\sum_ {j = 1} ^ {n} c _ {j} x _ {j} \leq \alpha \sum_ {j = 1} ^ {n} \left(\sum_ {i = 1} ^ {m} a _ {i j} y _ {i}\right) x _ {j} = \alpha \sum_ {i = 1} ^ {m} \left(\sum_ {j = 1} ^ {n} a _ {i j} x _ {j}\right) y _ {i}} \\ & {\qquad \leq \alpha \beta \sum_ {i = 1} ^ {m} b _ {i} y _ {i}.} \end{array}\tag{15.1}
$$

The first and second inequalities follow from the primal and dual conditions, respectively. The equality follows by simply changing the order of summation. ✷

The algorithm starts with a primal infeasible solution and a dual feasible solution; these are usually the trivial solutions ${ \pmb x } = { \bf 0 }$ and $\mathbf { \nabla } _ { \mathbf { y } } = \mathbf { 0 }$ . It iteratively improves the feasibility of the primal solution, and the optimality of the dual solution, ensuring that in the end a primal feasible solution is obtained and all conditions stated above, with a suitable choice of $\alpha$ and $\beta ,$ are satisfied. The primal solution is always extended integrally, thus ensuring that the final solution is integral. The improvements to the primal and the dual $_ \mathrm { g o }$ handin-hand: the current primal solution is used to determine the improvement to the dual, and vice versa. Finally, the cost of the dual solution is used as a lower bound on $\mathrm { O P T }$ , and by Proposition 15.1, the approximation guarantee of the algorithm is $\alpha \beta$

## 15.2 Primal–dual schema applied to set cover

Let us obtain a factor $f$ algorithm for the set cover problem using the primal– dual schema. For this algorithm, we will choose $\alpha = 1$ and $\beta = f$ . We will work with the primal and dual pair of $\mathrm { L P } { \mathrm { { s } } }$ given in (13.2) and (13.3), respectively. The complementary slackness conditions are:

Primal conditions

$$
\forall S \in \mathcal {S}: x _ {S} \neq 0 \Rightarrow \sum_ {e: e \in S} y _ {e} = c (S).
$$

Set $S$ will be said to be tight if $\begin{array} { r } { \sum _ { e \colon { e \in S } } y _ { e } = c ( S ) } \end{array}$ . Since we will increment the primal variables integrally, we can state the conditions as: Pick only tight sets in the cover.

Clearly, in order to maintain dual feasibility, we are not allowed to overpack any set.

Dual conditions

$$
\forall e: y _ {e} \neq 0 \Rightarrow \sum_ {S: e \in S} x _ {S} \leq f
$$

Since we will find a $0 / 1$ solution for ${ \mathbf { } } ^ { \mathbf { } } \mathbf { { \mathbf { x } } } ,$ these conditions are equivalent to: Each element having a nonzero dual value can be covered at most $f$ times. Since each element is in at most $f$ sets, this condition is trivially satisfied for all elements.

The two sets of conditions naturally suggest the following algorithm:

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 15.2 (Set cover - factor $f$)
1. Initialization: $x \leftarrow 0$; $y \leftarrow 0$
2. Until all elements are covered, do:
    Pick an uncovered element, say $e$, and raise $y_e$ until some set goes tight.
    Pick all tight sets in the cover and update $x$.
    Declare all the elements occurring in these sets as “covered”.
3. Output the set cover $x$.
</div>

Theorem 15.3 Algorithm 15.2 achieves an approximation factor of $f .$

Proof: Clearly there will be no uncovered elements and no overpacked sets at the end of the algorithm. Thus, the primal and dual solutions will both be feasible. Since they satisfy the relaxed complementary slackness conditions with $\alpha = f$ , by Proposition 15.1 the approximation factor is $f .$ ✷

Example 15.4 A tight example for this algorithm is provided by the following set system:

![](images/6ff5135f4f8ff85aad6fd5205ea8818df323723ad127844a0e3a2ced73513ea8.jpg)

Here, $s$ consists of $n - 1$ sets of cost 1, $\{ e _ { 1 } , e _ { n } \} , \ldots , \{ e _ { n - 1 } , e _ { n } \}$ , and one set of cost $1 + \varepsilon , \{ e _ { 1 } , \ldots , e _ { n + 1 } \}$ , for a small $\varepsilon > 0$ . Since $e _ { n }$ appears in all n sets, this set system has $f = n .$

Suppose the algorithm raises $y _ { e _ { n } }$ in the first iteration. When $y _ { e _ { n } }$ is raised to 1, all sets $\{ e _ { i } , e _ { n } \} , i = 1 , \dots , n - 1$ go tight. They are all picked in the cover, thus covering the elements $e _ { 1 } , \ldots , e _ { n }$ . In the second iteration, $y _ { e _ { n + 1 } }$ is raised to ε and the set $\{ e _ { 1 } , \ldots , e _ { n + 1 } \}$ goes tight. The resulting set cover has a cost of $n + \varepsilon ,$ , whereas the optimum cover has cost $1 + \varepsilon$ ✷

## 15.3 Exercises

15.1 How is the algorithm given in Exercise 2.11 for the weighted vertex cover problem related to Algorithm 15.2 for the case $f = 2 ?$

15.2 Remove the scafolding of linear programming from Algorithm 15.2 to obtain a purely combinatorial factor f algorithm for set cover.

Hint: See the algorithm in Exercise 2.11.

15.3 Let k be a fixed constant, and consider instances of set cover whose maximum frequency, f, is bounded by k. Algorithm 15.2 shows that the integrality gap of LP (13.2) is upper bounded by k for these instances. Provide examples to show that this bound is essentially tight.

Hint: Consider a regular hypergraph, G, on n vertices which has a hyperedge corresponding to each choice of k of the n vertices. Construct the set system as follows. It has an element corresponding to each hyperedge and a set corresponding to each vertex, with incidence defining inclusion.

15.4 The following LP-relaxation is exact for the maximum weight matching problem (see definition in Exercise 12.8) in bipartite graphs but not in general graphs. Give a primal–dual algorithm, relaxing complementary slackness conditions appropriately, to show that the integrality gap of this $\mathrm { L P i s } \geq 1 / 2$ What is the best upper bound you can place on the integrality gap?

$$
\mathrm{maximize} \sum_ {e} w _ {e} x _ {e}\tag{15.2}
$$

$$
\text{subject to} \sum_{\substack{e: e\text{incident at} v\\ x_{e}\geq 0,}}x_{e}\leq 1,\quad v\in V\\ e\in E
$$

15.5 (Chudak, Goemans, Hochbaum, and Williamson [46]) Interpret the layering-based algorithms obtained for set cover and feedback vertex set problems in Chapters 2 and 6 as primal–dual schema based algorithms. How are the complementary slackness conditions being relaxed?

## 15.4 Notes

Kuhn [179] gave the first primal–dual algorithm – for the weighted bipartite matching problem – however, he used the name “Hungarian Method” to describe his algorithm. Dantzig, Ford, and Fulkerson [60] used this method for giving another means of solving linear programs and called it the primal– dual method. Although the schema was not very successful for solving linear programs, it soon found widespread use in combinatorial optimization.

Algorithm 15.2 is due to Bar-Yehuda and Even [20]. Although it was not originally stated as a primal–dual algorithm, in retrospect, this was the first use of the schema in approximation algorithms. The works of Agrawal, Klein, and Ravi [1] and Goemans and Williamson [105] revived the use of this schema in the latter setting, and introduced the powerful idea of growing duals in a synchronized manner (see Chapter 22). The mechanism of relaxing complementary slackness conditions was first formalized in Williamson, Goemans, Mihail, and Vazirani [258]. For further historical information, see Goemans and Williamson [107].