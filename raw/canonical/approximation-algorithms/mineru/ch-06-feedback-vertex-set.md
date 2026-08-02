---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-06"
chapter_number: 6
chapter_title: "Feedback Vertex Set"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 72
source_page_end: 78
printed_page_start: 54
printed_page_end: 60
part_ids: ["approximation-algorithms-ch-06-part-007"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Feedback Vertex Set (MinerU semantic layer)

<!-- source-pages: 72-78; printed-pages: 54-60; mineru-part: approximation-algorithms-ch-06-part-007 -->

## 6 Feedback Vertex Set

In this chapter we will use the technique of layering, introduced in Chapter 2, to obtain a factor $2$ approximation algorithm for:

Problem 6.1 (Feedback vertex set) Given an undirected graph $G =$ $( V , E )$ and a function w assigning nonnegative weights to its vertices, find a minimum weight subset of $\check { V }$ whose removal leaves an acyclic graph.

## 6.1 Cyclomatic weighted graphs

Order the edges of G in an arbitrary order. The characteristic vector of a simple cycle C in $G$ is a vector in $\mathrm { G F } [ 2 ] ^ { m } , m = | E |$ , which has 1’s in components corresponding to edges of $C$ and $0 \mathrm { { s } }$ in the remaining components. The cycle space of $G$ is the subspace of $\mathrm { G F } [ 2 ] ^ { m }$ that is spanned by the characteristic vectors of all simple cycles of $G ,$ and the cyclomatic number of $G ,$ denoted $\operatorname { c y c } ( G )$ , is the dimension of this space.

Theorem 6.2 $\operatorname { c y c } ( G ) = | E | - | V | + \kappa ( G )$ , where $\kappa ( G )$ denotes the number $o f$ connected components $o f G$

Proof: The cycle space of a graph is the direct sum of the cycle spaces of its connected components, and so its cyclomatic number is the sum of the cyclomatic numbers of its connected components. Therefore, it is suficient to prove the theorem for a connected graph $G$

Let $T$ be a spanning tree in $G .$ . For each nontree edge $e ,$ define its fundamental cycle to be the unique cycle formed in $T \cup \{ e \}$ . The set of characteristic vectors of all such cycles is linearly independent (each cycle includes an edge that is in no other fundamental cycle). Thus, $\operatorname { c y c } ( G ) \geq | E | - | V | + 1$

Each edge e of $T$ defines a fundamental cut $( S , { \overline { { S } } } )$ in $G , S \subset V ( S$ and $\overline { S }$ are the vertex sets of two connected components formed by removing e from $T )$ . Define the characteristic vector of a cut to be a vector in $\mathrm { G F } [ 2 ] ^ { m }$ that has 1’s in components corresponding to the edges of $G$ in the cut and $0 \mathrm { { s } }$ in the remaining components. Consider the $| V | - 1$ vectors defined by edges of $T .$ . Since each cycle must cross each cut an even number of times, these vectors are orthogonal to the cycle space of $G .$ Furthermore, these $| V | - 1$ vectors are linearly independent, since each cut has an edge (the tree edge defining this cut) that is not in any of the other $| V | - 2$ cuts. Therefore the dimension of the orthogonal complement to the cycle space is at least $| V | - 1$ Hence, $\operatorname { c y c } ( G ) \leq | E | - | V | + 1$ . Combining with the previous inequality we get cyc $( G ) = | E | - | V | + 1$ ✷

Denote by $\delta _ { G } ( v )$ the decrease in the cyclomatic number of the graph on removing vertex v. Since the removal of a feedback vertex set $F = \{ v _ { 1 } , \ldots , v _ { f } \}$ decreases the cyclomatic number of G down to 0,

$$
\operatorname{cyc} (G) = \sum_ {i = 1} ^ {f} \delta_ {G _ {i - 1}} (v _ {i}),
$$

where $G _ { 0 } = G$ and, for $i > 0 , G _ { i } = G - \left\{ v _ { 1 } , \ldots , v _ { i } \right\}$ . By Lemma 6.4 below, we get:

$$
\operatorname{cyc} (G) \leq \sum_ {v \in F} \delta_ {G} (v).\tag{6.1}
$$

Let us say that a function assigning vertex weights is cyclomatic if there is a constant $c > 0$ such that the weight of each vertex v is $c \cdot \delta _ { G } ( v )$ . By inequality (6.1), for such a weight function, $c \cdot \operatorname { c y c } ( G )$ is a lower bound on OPT. The importance of cyclomatic weight functions is established in Lemma 6.5 below, which shows that for such a weight function, any minimal feedback vertex set has a weight within twice the optimal.

Let $\deg _ { G } ( v )$ denote the degree of v in $G ,$ and comps $( G - v )$ denote the number of connected components formed by removing v from $G _ { \ l }$ . The claim below follows in a straightforward way by applying Theorem 6.2 to G and $G - v$

Claim 6.3 For a connected graph $G , \delta _ { G } ( v ) = \deg _ { G } ( v ) - \mathrm { c o m p s } ( G - v )$

Lemma 6.4 Let H be a subgraph of G (not necessarily vertex induced). Then, $\delta _ { H } ( v ) \leq \delta _ { G } ( v )$

Proof: It is suficient to prove the lemma for the connected components of G and H containing v. We may thus assume w.l.o.g. that G and H are connected (H may be on a smaller set of vertices). By Claim 6.3, proving the following inequality is suficient:

$$
\deg_ {H} (v) - \operatorname{comps} (H - v) \leq \deg_ {G} (v) - \operatorname{comps} (G - v).
$$

We will show that edges in $G \mathrm { ~ - ~ } H$ can only help this inequality. Let $c _ { 1 } , c _ { 2 } , \ldots , c _ { k }$ be components formed by removing v from H. Edges of $G - H$ not incident at v can only help merge some of these components (and of course, they don’t change the degree of v). An edge of $G - H$ that is incident at v can lead to an additional component, but this is compensated by the contribution the edge has to the degree of v. ✷

Lemma 6.5 If F is a minimal feedback vertex set of G, then

$$
\sum_ {v \in F} \delta_ {G} (v) \leq 2 \cdot \operatorname{cyc} (G).
$$

Proof: Since the cycle space of G is the direct sum of the cycle spaces of its connected components, it sufices to prove the lemma for a connected graph G.

Let $F = \{ v _ { 1 } , \ldots , v _ { f } \}$ , and let k be the number of connected components obtained by deleting F from G. Partition these components into two types: those that have edges incident to only one of the vertices of $F ,$ and those that have edges incident to two or more vertices of $F .$ . Let t and $k - t$ be the number of components of the first and second type, respectively. We will prove that

$$
\sum_ {i = 1} ^ {f} \delta_ {G} (v _ {i}) = \sum_ {i = 1} ^ {f} (\deg_ {G} (v _ {i}) - \operatorname{comps} (G - v _ {i})) \leq 2 (| E | - | V |),
$$

thereby proving the lemma. Clearly, $\textstyle \sum _ { i = 1 } ^ { f }$ com $\mathrm { p s } ( G - v _ { i } ) = f + t$ . Therefore, we are left to prove

$$
\sum_ {i = 1} ^ {f} \deg_ {G} (v _ {i}) \leq 2 (| E | - | V |) + f + t.
$$

![](images/1150e941c089e76e7dc56b58abc5fc44f929893bf6b760f061b81b15e7ed5b1c.jpg)

Since $F$ is a feedback vertex set, each of the k components is acyclic and is therefore a tree. Thus, the number of edges in these components is $| V | - f - k$ Next, we put a lower bound on the number of edges in the cut $( F , V - F )$

Since $F$ is minimal, each $v _ { i } \in F$ must be in a cycle that contains no other vertices of $F .$ . Therefore, each $v _ { i }$ must have at least two edges incident at one of the components. For each $v _ { i }$ , arbitrarily remove one of these edges from $G$ , thus removing a total of $f$ edges. Now, each of the t components must still have at least one edge and each of the $k - t$ components must still have at least two edges incident at $F$ . Therefore, the number of edges in the cut $( F , V - F )$ is at least $f + t + 2 ( k - t ) = f + 2 k - t$

These two facts imply that

$$
\sum_ {i = 1} ^ {f} \deg_ {G} (v _ {i}) \leq 2 | E | - 2 (| V | - f - k) - (f + 2 k - t).
$$

The lemma follows.

✷

Corollary 6.6 Let w be a cyclomatic weight function on the vertices of $G _ { i }$ and let F be a minimal feedback vertex set in it. Then $w ( F ) \leq 2 \cdot \mathrm { O P T }$

## 6.2 Layering applied to feedback vertex set

Let us now deal with arbitrary weighted graphs. Consider the following basic operation: Given graph $G = ( V , E )$ and a weight function w, let

$$
c = \min _ {v \in V} \left\{\frac {w (v)}{\delta_ {G} (v)} \right\}.
$$

The weight function $t ( v ) = c \delta _ { G } ( v )$ is the largest cyclomatic weight function in w. Define $w ^ { \prime } ( v ) = w ( v ) - t ( v )$ to be the residual weight function. Finally, let $V ^ { \prime }$ be the set of vertices having positive residual weight (clearly, $V ^ { \prime } \subset V )$ and let $G ^ { \prime }$ be the subgraph of $G$ induced on $V ^ { \prime }$

Using this basic operation, decompose G into a nested sequence of induced subgraphs, until an acyclic graph is obtained, each time finding the largest cyclomatic weight function in the current residual weight function. Let these graphs be $G = G _ { 0 } \supset G _ { 1 } \supset \cdot \cdot \cdot \supset G _ { k } .$ , where $G _ { k }$ is acyclic; $G _ { i }$ is the induced subgraph of $G$ on vertex set $V _ { i } ,$ where $V = V _ { 0 } \supset V _ { 1 } \supset \cdots \supset V _ { k }$ . Let $t _ { i } , i =$ $0 , \ldots , k - 1$ be the cyclomatic weight function defined on graph $G _ { i }$ . Thus, $w _ { 0 } = w$ is the residual weight function for $G _ { 0 } , \ t _ { 0 }$ is the largest cyclomatic weight function in $w _ { 0 } , w _ { 1 } = w _ { 0 } - t _ { 0 }$ is the residual weight function for $G _ { 1 }$ and so on. Finally, $w _ { k }$ is the residual weight function for $G _ { k }$ . For convenience, define $t _ { k } = w _ { k }$ . Since the weight of a vertex v has been decomposed into the weights $t _ { 0 } , t _ { 1 } , \ldots , t _ { k }$ , we have

$$
\sum_ {i: v \in V _ {i}} t _ {i} (v) = w (v).
$$

The next fact suggests an algorithm for constructing a feedback vertex set on which Lemma 6.5 can be applied.

Lemma 6.7 Let H be a subgraph of $G = ( V , E )$ , induced on vertex set $V ^ { \prime } \subset$ V. Let F be a minimal feedback vertex set in H, and let $F ^ { \prime } \subseteq V - V ^ { \prime }$ be a minimal set such that $F \cup F ^ { \prime }$ is a feedback vertex set for G. Then $F \cup F ^ { \prime }$ is a minimal feedback vertex set $f o r G$

Proof: Since F is minimal for H, for each $v \in F$ , there is a cycle, say C, in H that does not use any other vertex of $F$ . Since $F ^ { \prime } \cap V ^ { \prime } = \bar { \varnothing } , C$ uses only one vertex, v, from $F \cup F ^ { \prime }$ as well, and so v is not redundant. ✷

After the entire decomposition, $F _ { k } = \emptyset$ is a minimal feedback vertex set of $G _ { k } . { \mathrm { F o r ~ } } i = k , k - 1 , \ldots , 1$ , the minimal feedback vertex set $F _ { i }$ found in $G _ { i }$ is extended in a minimal way using vertices of $V _ { i - 1 } - V _ { i }$ to yield a minimal feedback vertex set, say $F _ { i - 1 }$ , for $G _ { i - 1 }$ . The last set, $F _ { 0 }$ , is a feedback vertex set for G.

![](images/b605fb2ecec25f10e4168cd3da1a453414ac4714a5572fa8e0ed155094052ee5.jpg)

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 6.8 (Feedback vertex set)
1. Decomposition phase
    $H \leftarrow G, w' \leftarrow w, i \leftarrow 0$
    While H is not acyclic,
    $c \leftarrow \min_{u \in H} \left\{ \frac{w'(u)}{\delta_H(u)} \right\}$ $G_i \leftarrow H, t_i \leftarrow c \cdot \delta_{G_i}, w' \leftarrow w' - t_i$ $H \leftarrow$ the subgraph of $G_i$ induced by vertices u with $w'(u) &gt; 0$ $i \leftarrow i + 1,$ $k \leftarrow i, G_k \leftarrow H$
2. Extension phase
    $F_k \leftarrow \emptyset$
    For $i = k, \ldots, 1$, extend $F_i$ to a feedback vertex set $F_{i-1}$ of $G_{i-1}$ by adding a minimal set of vertices from $V_{i-1} - V_i$.
    Output $F_0$.
</div>

Theorem 6.9 Algorithm 6.8 achieves an approximation guarantee of factor 2 for the feedback vertex set problem.

Proof: Let $F ^ { * }$ be an optimal feedback vertex set for G. Since $G _ { i }$ is an induced subgraph of $G , F ^ { * } \cup V _ { i }$ must be a feedback vertex set for $G _ { i }$ (not necessarily optimal). Since the weights of vertices have been decomposed into the functions $t _ { i } ,$ , we have

$$
\mathrm{OPT} = w (F ^ {*}) = \sum_ {i = 0} ^ {k} t _ {i} (F ^ {*} \cap V _ {i}) \geq \sum_ {i = 0} ^ {k} \mathrm{OPT} _ {i},
$$

where $\mathrm { O P T } _ { i }$ is the weight of an optimal feedback vertex set of $G _ { i }$ with weight function $t _ { i }$ .

By decomposing the weight of $F _ { 0 }$ , we get

$$
w (F _ {0}) = \sum_ {i = 0} ^ {k} t _ {i} (F _ {0} \cap V _ {i}) = \sum_ {i = 0} ^ {k} t _ {i} (F _ {i}).
$$

By Lemma 6.7, $F _ { i }$ is a minimal feedback vertex set in $G _ { i }$ . Since for $0 \leq i \leq$ $k - 1 , t _ { i }$ is a cyclomatic weight function, by Lemma 6.5, $t _ { i } ( F _ { i } ) \le 2 \mathrm { O P T } _ { i }$ ; recall that $F _ { k } = \emptyset$ . Therefore,

$$
w (F _ {0}) \leq 2 \sum_ {i = 0} ^ {k} \mathrm{OPT} _ {i} \leq 2 \cdot \mathrm{OPT}.
$$

Example 6.10 A tight example for the algorithm is given by the graph obtained by removing a perfect matching from a complete bipartite graph and duplicating every edge. (Note that the algorithm works for parallel edges as well. If a tight example without parallel edges is desired, then a vertex with very high weight can be placed on every edge.)

![](images/24096af7f19c474a385079b577b3ad014e2e2e45cf8ea5927052cb4eee8012ae.jpg)

Assuming that the graph is cyclomatic weighted, each vertex receives the same weight. The decomposition obtained by the algorithm consists of only one nontrivial graph, G itself, on which the algorithm computes a minima feedback vertex set. A possible output of the algorithm is the set shown above; this set contains $2 n - 2$ vertices as compared with the optimum of n given by one side of the bipartition. ✷

## 6.3 Exercises

6.1 A natural greedy algorithm for finding a minimum feedback vertex set is to repeatedly pick and remove the most cost-efective vertex, i.e., a vertex minimizing $w ( v ) / \delta _ { H } ( v )$ , where H is the current graph, until there are no more cycles left. Give examples to show that this is not a constant factor algorithm. What is the approximation guarantee of this algorithm?

6.2 Give an approximation factor preserving reduction from the vertex cover problem to the feedback vertex set problem (thereby showing that improving the factor for the latter problem will also improve it for the former; also see Section 30.1).

## 6.4 Notes

Algorithm 6.8 is due to Bafna, Berman, and Fujito [19] (see also Becker and Geiger [23] and Chudak, Goemans, Hochbaum, and Williamson [46] for other factor 2 algorithms for the feedback vertex set problem).
