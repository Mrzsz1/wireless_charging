---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-05"
chapter_number: 5
chapter_title: "k-Center"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 65
source_page_end: 71
printed_page_start: 47
printed_page_end: 53
part_ids: ["approximation-algorithms-ch-05-part-006"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# k-Center (MinerU semantic layer)

<!-- source-pages: 65-71; printed-pages: 47-53; mineru-part: approximation-algorithms-ch-05-part-006 -->

Consider the following application. Given a set of cities, with intercity distances specified, pick k cities for locating warehouses in so as to minimize the maximum distance of a city from its closest warehouse. We will study this problem, called the k-center problem, and its weighted version, under the restriction that the edge costs satisfy the triangle inequality. Without this restriction, the k-center problem cannot be approximated within factor $\alpha ( n )$ for any computable function $\alpha ( n )$ , assuming $\mathbf { P } \neq \mathbf { N P }$ (see Exercise 5.1).

We will introduce the algorithmic technique of parametric pruning for solving this problem. In Chapter 17 we will use this technique in a linear programming setting.

Problem 5.1 (Metric k-center) Let $G = ( V , E )$ be a complete undirected graph with edge costs satisfying the triangle inequality, and k be a positive integer. For any set $S \subseteq V$ and vertex $v \in V$ , define connect $( v , S )$ to be the cost of the cheapest edge from v to a vertex in S. The problem is to find a set $S \subseteq V$ , with $| S | = k$ , so as to minimize max {connect(v, S)}.

## 5.1 Parametric pruning applied to metric k-center

If we know the cost of an optimal solution, we may be able to prune away irrelevant parts of the input and thereby simplify the search for a good solution. However, as stated in Chapter 1, computing the cost of an optimal solution is precisely the dificult core of NP-hard NP-optimization problems. The technique of parametric pruning gets around this dificulty as follows. $\mathrm { A }$ parameter t is chosen, which can be viewed as a $\mathrm { ^ { 6 } g u e s s ^ { 7 } }$ on the cost of an optimal solution. For each value of $t ,$ the given instance I is pruned by removing parts that will not be used in any solution of cost $> t .$ Denote the pruned instance by $I ( t )$ . The algorithm consists of two steps. In the first step, the family of instances $I ( t )$ is used for computing a lower bound on $\mathrm { O P T }$ , say $t ^ { * }$ . In the second step, a solution is found in instance $I ( \alpha \cdot t ^ { * } )$ , for a suitable choice of α.

A restatement of the k-center problem shows how parametric pruning applies naturally to it. Sort the edges of $G$ in nondecreasing order of cost, $\operatorname { i . e . , \ c o s t } ( e _ { 1 } ) \leq \operatorname { c o s t } ( e _ { 2 } ) \leq \ldots \leq \operatorname { c o s t } ( e _ { m } )$ , and let $G _ { i } = ( V , E _ { i } )$ , where $E _ { i } =$ $\{ e _ { 1 } , e _ { 2 } , \ldots , e _ { i } \}$ . A dominating set in an undirected graph $H = ( U , F )$ is a subset $S \subseteq U$ such that every vertex in $U - S$ is adjacent to a vertex in S. Let dom(H) denote the size of a minimum cardinality dominating set in H. Computing dom(H) is NP-hard. The k-center problem is equivalent to finding the smallest index i such that $G _ { i }$ has a dominating set of size at most k, i.e., $G _ { i }$ contains k stars spanning all vertices, where a star is the graph $K _ { 1 , p }$ , with $p \geq 1 . \mathrm { ~ H ~ } i ^ { * }$ is the smallest such index, then cost $( e _ { i ^ { * } } )$ is the cost of an optimal k-center. We will denoted this by OPT. We will work with the family of graphs $G _ { 1 } , \ldots , G _ { m }$

Define the square of graph H to be the graph containing an edge $( u , v )$ whenever H has a path of length at most two between u and $v , u \ne v$ . We will denote it by $H ^ { 2 }$ . The following structural result gives a method for lower bounding OPT.

Lemma 5.2 Given a graph H, let I be an independent set in $H ^ { 2 }$ . Then, $| I | \leq \mathrm { d o m } ( H )$

Proof: Let D be a minimum dominating set in H. Then, H contains $| D |$ stars spanning all vertices. Since each of these stars will be a clique in $H ^ { 2 }$ $H ^ { 2 }$ contains |D| cliques spanning all vertices. Clearly, I can pick at most one vertex from each clique, and the lemma follows. ✷

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 5.3 (Metric k-center)
1. Construct  $G_{1}^{2}, G_{2}^{2}, \ldots, G_{m}^{2}$ .
2. Compute a maximal independent set,  $M_{i}$ , in each graph  $G_{i}^{2}$ .
3. Find the smallest index i such that  $|M_{i}| \leq k$ , say j.
4. Return  $M_{j}$ .
</div>

The lower bound on which this algorithm is based is:

Lemma 5.4 For j as defined in the algorithm, cost $( e _ { j } ) \le \mathrm { O P T }$

Proof: For every $\textit { i } < \textit { j }$ we have that $| M _ { i } | > k$ . Now, by Lemma 5.2, dom $( G _ { i } ) > k$ , and so $i ^ { * } > i .$ . Hence, $j \leq i ^ { * }$ ✷

Theorem 5.5 Algorithm 5.3 achieves an approximation factor of 2 for the metric k-center problem.

Proof: The key observation is that a maximal independent set, I, in a graph is also a dominating set (for, if some vertex v is not dominated by I, then $I \cup \{ v \}$ must also be an independent set, contradicting I’s maximality). Thus, there exist stars in $G _ { j } ^ { 2 }$ , centered on the vertices of $M _ { j }$ , covering all vertices. By the triangle inequality, each edge used in constructing these stars has cost at most $2 \cdot \mathrm { c o s t } ( e _ { j } )$ . The theorem follows from Lemma 5.4. ✷

Example 5.6 A tight example for the previous algorithm is given by a whee graph on $n + 1$ vertices, where all edges incident to the center vertex have cost 1, and the rest of the edges have cost 2:

![](images/248530775ed254469363bdfa88bbed0fd99f5ce796598ddcb19ea71282b27ee6.jpg)

(Here, thin edges have cost 1 and thick edges have cost $2 ;$ not all edges of cost 2 are shown.)

For $k = 1$ , the optimal solution is the center of the wheel, and $\mathrm { { O P T } = 1 }$ The algorithm will compute index $j ~ = ~ n$ . Now, $G _ { n } ^ { 2 }$ is a clique and, if a peripheral vertex is chosen as the maximal independent set, then the cost of the solution found is 2. ✷

Next, we will show that 2 is essentially the best approximation factor achievable for the metric k-center problem.

Theorem 5.7 Assuming $\mathbf { P } \neq \mathbf { N P }$ , there is no polynomial time algorithm achieving a factor of $2 - \varepsilon , \varepsilon > 0$ , for the metric k-center problem.

Proof: We will show that such an algorithm can solve the dominating set problem in polynomial time. The idea is similar to that of Theorem 3.6 and involves giving a reduction from the dominating set problem to metric kcenter. Let $G = ( V , E )$ , k be an instance of the dominating set problem. Construct a complete graph $G ^ { \prime } = ( V , E ^ { \prime } )$ with edge costs given by

$$
\operatorname{cost} (u, v) = \left\{ \begin{array}{l} 1, \text {   if   } (u, v) \in E, \\ 2, \text {   if   } (u, v) \not \in E. \end{array} \right.
$$

Clearly, $G ^ { \prime }$ satisfies the triangle inequality. This reduction satisfies the conditions:

• if $\operatorname { d o m } ( G ) \leq k .$ , then $G ^ { \prime }$ has a k-center of cost 1, and

• if dom $( G ) > k ,$ , then the optimum cost of a k-center in $G ^ { \prime }$ is 2.

In the first case, when run on $G ^ { \prime }$ , the $( 2 - \varepsilon )$ -approximation algorithm must give a solution of cost 1, since it cannot use an edge of cost 2. Hence, using this algorithm, we can distinguish between the two possibilities, thus solving the dominating set problem. ✷

## 5.2 The weighted version

We will use the technique of parametric pruning to obtain a factor 3 approximation algorithm for the following generalization of the metric k-center problem.

Problem 5.8 (Metric weighted k-center) In addition to a cost function on edges, we are given a weight function on vertices, $w : V \to R ^ { + }$ , and a bound $W \in R ^ { + }$ . The problem is to pick $S \subseteq V$ of total weight at most $W$ minimizing the same objective function as before, i.e.,

$$
\max _ {v \in V} \{\min _ {u \in S} \{\text { cost } (u, v) \} \}.
$$

Let wdom $. ( G )$ denote the weight of a minimum weight dominating set in $G _ { \ l }$ . Then, with respect to the graphs $G _ { i }$ defined above, we need to find the smallest index i such that wdom $( G _ { i } ) \leq W$ . If $i ^ { * }$ is this index, then the cost of the optimal solution is $\mathrm { O P T } = \displaystyle \mathrm { c o s t } ( e _ { i ^ { * } } )$

Given a vertex weighted graph H, let I be an independent set in $H ^ { 2 }$ . For each $u \in I$ , let $s ( u )$ denote a lightest neighbor of u in $H _ { ; }$ , where u is also considered a neighbor of itself. (Notice that the neighbor is picked in H and not in $H ^ { 2 } . )$ Let $S = \{ s ( u ) | ~ u \in I \}$ . The following fact, analogous to Lemma 5.2, will be used to derive a lower bound on $\mathrm { O P T }$

Lemma 5.9 w(S) ≤ wdom(H).

Proof: Let D be a minimum weight dominating set of H. Then there exists a set of disjoint stars in H, centered on the vertices of $D$ and covering all the vertices. Since each of these stars becomes a clique in $H ^ { 2 } , I$ can pick at most one vertex from each of them. Thus, each vertex in I has the center of the corresponding star available as a neighbor in H. Hence, $w ( S ) \leq w ( D )$ ✷

The algorithm is given below. In it, $s _ { i } ( u )$ will denote a lightest neighbor of u in $G _ { i } ;$ ; for this definition, u will also be considered a neighbor of itself.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 5.10 (Metric weighted $k$-center)  
1. Construct $G_1^2, G_2^2, \ldots, G_m^2$.  
2. Compute a maximal independent set, $M_i$, in each graph $G_i^2$.  
3. Compute $S_i = \{s_i(u) | u \in M_i\}$.  
4. Find the minimum index $i$ such that $w(S_i) \leq W$, say $j$.  
5. Return $S_j$.
</div>

Theorem 5.11 Algorithm 5.10 achieves an approximation factor of 3 for the weighted k-center problem.

Proof: By Lemma 5.9, cost $( e _ { j } )$ is a lower bound on $\mathrm { O P T } ;$ ; the argument is identical to that in Lemma 5.4 and is omitted here. Since $M _ { j }$ is a dominating set in $G _ { j } ^ { 2 }$ , we can cover V with stars of $G _ { j } ^ { 2 }$ centered in vertices of $M _ { j }$ . By the triangle inequality these stars use edges of cost at most 2 · cost $( e _ { j } )$

![](images/97365cddda2e11fcaef5588c342a9bb8556373e3748609e029f9c8ea65901ab2.jpg)

Each star center is adjacent to a vertex in $S _ { j }$ , using an edge of cost at most cost $( e _ { j } )$ . Move each of the centers to the adjacent vertex in $S _ { j }$ and redefine the stars. Again, by the triangle inequality, the largest edge cost used in constructing the final stars is at most 3 · cost $( e _ { j } )$ ✷

Example 5.12 A tight example is provided by the following graph on $n + 4$ vertices. Vertex weights and edge costs are as marked; all missing edges have a cost given by the shortest path.

![](images/337487883295e1292f68e1be85ff275320a8ed977961dc75c5b72c7d3ec0e1f0.jpg)

It is not dificult to see that for $W = 3$ the optimum cost of a k-center is $1 + \varepsilon \colon \mathrm { a }$ k-center achieving this cost is $\{ a , c \}$ . For any $\textit { i } < n + 3$ , the set $S _ { i }$ computed by the algorithm will contain a vertex of infinite weight. Suppose that, for $i = n + 3 ,$ , the algorithm chooses $M _ { n + 3 } = \{ b \}$ as a maximal independent set. Then $S _ { n + 3 } = \{ a \}$ , and this is the output of the algorithm. The cost of this solution is 3. ✷

## 5.3 Exercises

5.1 Show that if the edge costs do not satisfy the triangle inequality, then the k-center problem cannot be approximated within factor $\alpha ( n )$ for any computable function $\alpha ( n )$

Hint: Put together ideas from Theorems 3.6 and 5.7.

5.2 Consider Step 2 of Algorithm 5.3, in which a maximal independent set is found in $G _ { i } ^ { 2 }$ . Perhaps a more natural choice would have been to find a minimal dominating set. Modify Algorithm 5.3 so that $M _ { i }$ is picked to be a minimal dominating set in $G _ { i } ^ { 2 }$ . Show that this modified algorithm does not achieve an approximation guarantee of 2 for the k-center problem. What approximation factor can you establish for this algorithm?

Hint: With this modification, the lower bounding method does not work, since Lemma 5.2 does not hold if I is picked to be a minimal dominating set in $H ^ { 2 }$

## 5.3 (Gonzalez [111]) Consider the following problem.

Problem 5.13 (Metric k-cluster) Let $G = ( V , E )$ be a complete undirected graph with edge costs satisfying the triangle inequality, and let k be a positive integer. The problem is to partition $V$ into sets $V _ { 1 } , \dots , V _ { k }$ so as to minimize the costliest edge between two vertices in the same set, i.e., minimize

$$
\max _ {1 \leq i \leq k, u, v \in V _ {i}} \operatorname{cost} (u, v).
$$

1. Give a factor 2 approximation algorithm for this problem, together with a tight example.

2. Show that this problem cannot be approximated within a factor of $2 - \varepsilon$ • for any $\varepsilon > 0$ , unless $\mathbf { P } = \mathbf { N P }$

5.4 (Khuller, Pless, and Sussmann [169]) The fault-tolerant version of the metric k-center problem has an additional input, $\alpha \leq k$ , which specifies the number of centers that each city should be connected to. The problem again is to pick k centers so that the length of the longest edge used is minimized.

A set $S \subseteq V$ in an undirected graph $H = ( V , E )$ is an α-dominating set if each vertex $v \in V$ is adjacent to at least α vertices in $S$ (assuming that a vertex is adjacent to itself). Let $\mathrm { d o m } _ { \alpha } ( H )$ denote the size of a minimum cardinality α-dominating set in $H$

1. Let I be an independent set in $H ^ { 2 }$ . Show that $\begin{array} { r } { \alpha | I | \leq \mathrm { d o m } _ { \alpha } ( H ) } \end{array}$

2. Give a factor 3 approximation algorithm for the fault-tolerant k-center problem.

Hint: Compute a maximal independent set $M _ { i }$ in $G _ { i } ^ { 2 }$ , for $1 \leq i \leq$ m. Find the smallest index i such that $\left| M _ { i } \right| \le \left\lfloor \frac { k } { \alpha } \right\rfloor$ , and moreover, the degree of each vertex of $M _ { i }$ in $G _ { i } \ { \mathrm { i s } } \geq \alpha - 1$

5.5 (Khuller, Pless, and Sussmann [169]) Consider a modification of the problem of Exercise 5.4 in which vertices of S have no connectivity requirements and only vertices of $V - S$ have connectivity requirements. Each vertex of $V - S$ needs to be connected to α vertices in S. The object again is to pick $S , | S | = k ,$ so that the length of the longest edge used is minimized.

The algorithm for this problem works on each graph $G _ { i }$ . It starts with $S _ { i } = \emptyset$ . Vertex $v \in V - S _ { i }$ is said to be j-connected if it is adjacent to $j$ vertices in $S _ { i }$ , using edges of $G _ { i } ^ { 2 }$ . While there is a vertex $v \in V - S _ { i }$ that is not k-connected, pick the vertex with minimum connectivity, and include it in $S _ { i }$ . Finally, find the minimum index i such that $| S _ { i } | \leq k ,$ , say l. Output $S _ { l }$ Prove that this is a factor 2 approximation algorithm.

## 5.4 Notes

Both k-center algorithms presented in this chapter are due to Hochbaum and Shmoys [127], and Theorem 5.7 is due to Hsu and Nemhauser [132].
