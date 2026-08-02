---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-20"
chapter_number: 20
chapter_title: "Multicut in General Graphs"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 186
source_page_end: 197
printed_page_start: 168
printed_page_end: 179
part_ids: ["approximation-algorithms-ch-20-part-021"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Multicut in General Graphs (MinerU semantic layer)

<!-- source-pages: 186-197; printed-pages: 168-179; mineru-part: approximation-algorithms-ch-20-part-021 -->

## 20 Multicut in General Graphs

The importance of min–max relations to combinatorial optimization was mentioned in Chapter 1. Perhaps the most useful of these is the celebrated max-flow min-cut theorem. Indeed, much of flow theory, and the theory of cuts in graphs, has been built around this theorem. It is not surprising, therefore, that a concerted efort was made to obtain generalizations of this theorem to the case of multiple commodities.

There are two such generalizations. In the first one, the objective is to maximize the sum of the commodities routed, subject to flow conservation and capacity constraints. In the second generalization, a demand dem(i) is specified for each commodity $i ,$ and the objective is to maximize $f ,$ called throughput, such that for each i, $, \ f \cdot \mathrm { d e m } ( i )$ amount of commodity i can be routed simultaneously. We will call these sum multicommodity flow and demands multicommodity flow problems, respectively. Clearly, for the case of a single commodity, both problems are the same as the maximum flow problem.

Each of these generalizations is associated with a fundamental NP-hard cut problem, the first with the minimum multicut problem, Problem 18.1, and the second with the sparsest cut problem, Problem 21.2. In each case an approximation algorithm for the cut problem gives, as a corollary, an approximate max-flow min-cut theorem. In this chapter we will study the first generalization; the second is presented in Chapter 21. We will obtain an O(log k) factor approximation algorithm for the minimum multicut problem, where k is the number of commodities. A factor 2 algorithm for the special case of trees was presented in Chapter 18.

## 20.1 Sum multicommodity flow

Problem 20.1 (Sum multicommodity flow) Let $G = ( V , E )$ be an undirected graph with nonnegative capacity $c _ { e }$ for each edge $e \in E .$ Let $\{ ( s _ { 1 } , t _ { 1 } ) , \ldots , ( s _ { k } , t _ { k } ) \}$ be a specified set of pairs of vertices where each pair is distinct, but vertices in diferent pairs are not required to be distinct. A separate commodity is defined for each $( s _ { i } , t _ { i } )$ pair. For convenience, we will think of $s _ { i }$ as the source and $t _ { i }$ as the sink of this commodity. The objective is to maximize the sum of the commodities routed. Each commodity must satisfy flow conservation at each vertex other than its own source and sink. Also, the sum of flows routed through an edge, in both directions combined, should not exceed the capacity of this edge.

Let us first give a linear programming formulation for this problem. For each commodity $i ,$ let $P _ { i }$ denote the set of all paths from $s _ { i }$ to $t _ { i }$ in $G ,$ and let $\textstyle P = \bigcup _ { i = 1 } ^ { k } { \tilde { P _ { i } } }$ . The LP will have a variable $f _ { p }$ for each $p \in P ,$ which will denote the flow along path $p .$ The endpoints of this path uniquely specify the commodity that flows on this path. The objective is to maximize the sum of flows routed on these paths, subject to edge capacity constraints. Notice that flow conservation constraints are automatically satisfied in this formulation. The program has exponentially many variables; however, that is not a concern since we will use it primarily to obtain a clean formulation of the dual program.

$$
\begin{array}{l l} \text {maximize} & \sum_ {p \in P} f _ {p} \\ \text {subject to} & \sum_ {p: e \in p} f _ {p} \leq c _ {e}, \quad e \in E \\ & f _ {p} \geq 0, \quad \quad \quad \quad p \in P \end{array}\tag{20.1}
$$

Let us obtain the dual of this program. For this, let $d _ { e }$ be the dual variable associated with edge $e .$ We will interpret these variables as distance labels of edges.

$$
\begin{array}{l l} \text {minimize} & \sum_ {e \in E} c _ {e} d _ {e} \\ & \sum_ {e \in p} d _ {e} \geq 1, \quad p \in P \\ & d _ {e} \geq 0, \quad e \in E \end{array}\tag{20.2}
$$

The dual program tries to find a distance label assignment to edges so that on each path $p \in P$ , the distance labels of edges add up to at least 1. Equivalently, a distance label assignment is feasible if for each commodity $i ,$ the shortest path from $s _ { i }$ to $t _ { i }$ has length at least 1.

Notice that the programs (18.2) and (18.1) are special cases of the two programs presented above for the restriction that $G$ is a tree.

The following remarks made in Chapter 18 hold for the two programs presented above as well: an optimal integral solution to LP (20.2) is a minimum multicut, and an optimal fractional solution can be viewed as a minimum fractional multicut. $\mathrm { B y }$ the LP-duality theorem, minimum fractional multicut equals maximum multicommodity flow and, as shown in Example 18.2, it may be strictly smaller than minimum integral multicut.

This naturally raises the question whether the ratio of minimum multicut and maximum multicommodity flow is bounded. Equivalently, is the integrality gap of LP (20.2) bounded? In the next section we present an algorithm for finding a multicut within an O(log k) factor of the maximum flow, thereby showing that the gap is bounded by O(log k).

## 20.2 LP-rounding-based algorithm

First notice that the dual program (20.2) can be solved in polynomial time using the ellipsoid algorithm, since there is a simple way of obtaining a separation oracle for it: simply compute the length of a minimum $s _ { i } - t _ { i }$ path, for each commodity i, w.r.t. the current distance labels. If all these lengths are $\geq 1$ , we have a feasible solution. Otherwise, the shortest such path provides a violated inequality. Alternatively, the LP obtained in Exercise 20.1 can be solved in polynomial time. Let $d _ { e }$ be the distance label computed for each edge e, and let $\begin{array} { r } { F = \sum _ { e \in E } c _ { e } d _ { e } } \end{array}$

Our goal is to pick a set of edges of small capacity, compared to $F ,$ that is a multicut. Let D be the set of edges with positive distance labels, $\mathrm { i . e . , }$ $D = \{ e \mid d _ { e } > 0 \}$ . Clearly, D is a multicut; however, its capacity may be very large compared to $F$ (Exercises 20.3 and 20.4). How do we pick a small capacity subset of D that is still a multicut? Since the optimal fractional multicut is the most cost-efective way of disconnecting all source–sink pairs, edges with large distance labels are more important than those with small distance labels for this purpose. The algorithm described below indirectly gives preference to edges with large distance labels.

The algorithm will work on graph $G = ( V , E )$ with edge lengths given by $d _ { e }$ . The weight of edge e is defined to be $c _ { e } d _ { e }$ . Let dist ${ \mathrm { : } } ( u , v )$ denote the length of the shortest path from u to v in this graph. For a set of vertices $S \subset V$ $\delta ( S )$ denotes the set of edges in the cut (S, S), c(S) denotes the capacity of this cut, i.e., the total capacity of edges in $\delta ( S )$ , and $\mathrm { w t } ( S )$ denotes the weight of set $S ,$ which is roughly the sum of weights of all edges having both endpoints in S (a more precise definition is given below).

The algorithm will find disjoint sets of vertices, $S _ { 1 } , \ldots , S _ { l } , l \le k$ , in $G ,$ called regions, such that:

• No region contains any source–sink pair, and for each $i ,$ either $s _ { i }$ or $t _ { i }$ is in one of the regions.

• For each region $S _ { i } , c ( S _ { i } ) \leq \varepsilon \mathrm { w t } ( S _ { i } )$ , where ε is a parameter that will be defined below.

By the first condition, the union of the cuts of these regions, i.e., $M \ =$ $\delta ( S _ { 1 } ) \cup \delta ( S _ { 2 } ) \cup \ldots \cup \delta ( S _ { l } )$ , is a multicut, and by the second condition, its capacity $c ( M ) \leq \varepsilon F$ . (When we give the precise definition of $\mathrm { w t } ( S )$ , this inequality will need to be modified slightly.)

## 20.2.1 Growing a region: the continuous process

The sets $S _ { 1 } , \ldots , S _ { l }$ are found through a region growing process. Let us first present a continuous process to clarify the issues. For the sake of time eficiency, the algorithm itself will use a discrete process (see Section 20.2.2).

Each region is found by growing a set starting from one vertex, which is the source or sink of a pair. This will be called the root of the region. Suppose the root is $s _ { 1 }$ . The process consists of growing a ball around the root. For each radius $r ,$ , define $S ( r )$ to be the set of vertices at a distance $\leq r$ from $s _ { 1 }$ ${ \mathrm { i . e . , ~ } } S ( r ) = \{ v \mid { \mathrm { d i s t } } ( s _ { 1 } , v ) \leq r \} . \ S ( 0 ) = \{ s _ { 1 } \}$ , and as $r$ increases continuously from 0, at discrete points, $S ( r )$ grows by adding vertices in increasing order of their distance from $s _ { 1 }$

Lemma 20.2 If the region growing process is terminated before the radius becomes 1/2, then the set S that is found contains no source–sink pair.

Proof: The distance between any pair of vertices in $S ( r )$ is $\leq 2 r$ . Since for each commodity $i ,$ dist $\mathbf { \Phi } _ { : } ( s _ { i } , t _ { i } ) \geq 1$ , the lemma follows. ✷

For technical reasons that will become clear in Lemma 20.3 (see also Exercises 20.5 and $2 0 . 6 )$ , we will assign a weight to the root, $\mathrm { w t } ( s _ { 1 } ) = F / k$ The weight of $S ( r )$ is the sum of $\mathrm { w t } ( s _ { 1 } )$ and the sum of the weights of edges, or parts of edges, in the ball of radius r around $s _ { 1 }$ . Let us state this formally. For edges $e$ having at least one endpoint in $S ( r )$ , let $q _ { e }$ denote the fraction of edge e that is in $S ( r )$ . If both endpoints of $e$ are in $S ( r )$ , then $q _ { e } = 1$ Otherwise, suppose $\boldsymbol { e } = \left( u , v \right)$ ) with $u \in S ( r )$ and v $\notin S ( r )$ . For such edges,

$$
q _ {e} = \frac {r - \mathrm{dist} (s _ {1} , u)}{\mathrm{dist} (s _ {1} , v) - \mathrm{dist} (s _ {1} , u)}.
$$

Define the weight of region S(r), $S ( r )$

$$
\mathrm{wt} (S (r)) = \mathrm{wt} (s _ {1}) + \sum c _ {e} d _ {e} q _ {e},
$$

where the sum is over all edges having at least one endpoint in $S ( r )$

We want to fix ε so that we can guarantee that we will encounter the condition $c ( S ( r ) ) \leq \varepsilon \mathrm { w t } ( S ( r ) )$ for $r < 1 / 2$ . The important observation is that at each point the rate at which the weight of the region is growing is at least $c ( S ( r ) )$ . Until this condition is encountered,

$$
\mathrm{d} \operatorname{wt} (S (r)) \geq c (S (r)) \mathrm{d} r > \varepsilon \operatorname{wt} (S (r)) \mathrm{d} r.
$$

Exercise 20.5 will help the reader gain some understanding of such a process.

Lemma 20.3 Picking $\varepsilon = 2 \ln ( k + 1 )$ sufices to ensure that the condition $c ( S ( r ) ) \leq \varepsilon \mathrm { w t } ( S ( r ) )$ will be encountered before the radius becomes $1 / 2$

Proof: The proof is by contradiction. Suppose that throughout the region growing process, starting with $r ~ = ~ 0$ and ending at $r \ = \ 1 / 2 , \ c ( S ( r ) ) \ >$ $\varepsilon \operatorname { w t } ( S ( r ) )$ ). At any point the incremental change in the weight of the region is

$$
\mathrm{d} \operatorname{wt} (S (r)) = \sum_ {e} c _ {e} d _ {e} \mathrm{d} q _ {e}.
$$

Clearly, only edges having one endpoint in $S ( r )$ will contribute to the sum. Consider such an edge $\boldsymbol { e } = ( u , v )$ such that $u \in S ( r )$ and $v \not \in S ( r )$ . Then,

$$
c _ {e} d _ {e} \mathrm{d} q _ {e} = c _ {e} \frac {d _ {e}}{\mathrm{dist} (s _ {1} , v) - \mathrm{dist} (s _ {1} , u)} \mathrm{d} r.
$$

Since dist $( s _ { 1 } , v ) \leq \mathrm { d i s t } ( s _ { 1 } , u ) + d _ { e } . $ we get $d _ { e } \geq \mathrm { d i s t } ( s _ { 1 } , v ) - \mathrm { d i s t } ( s _ { 1 } , u )$ , and hence $c _ { e } d _ { e } \mathrm { d } q _ { e } \geq c _ { e } \mathrm { d } r$ . This gives

$$
\mathrm{d} \operatorname{wt} (S (r)) \geq c (S (r)) \mathrm{d} r > \varepsilon \operatorname{wt} (S (r)) \mathrm{d} r.
$$

As long as the terminating condition is not encountered, the weight of the region increases exponentially with the radius. The initial weight of the region is $F / k$ and the final weight is at most $F + F / k$ . Integrating we get

$$
\int_ {\frac {F}{k}} ^ {F + \frac {F}{k}} \frac {1}{\operatorname{wt} (S (r))} \mathrm{d} \operatorname{wt} (S (r)) > \int_ {0} ^ {\frac {1}{2}} \varepsilon \mathrm{d} r.
$$

Therefore, l $\mathrm { n } ( k + 1 ) > \frac { 1 } { 2 } \varepsilon$ . However, this contradicts the assumption that $\varepsilon = 2 \ln ( k + 1 )$ ), thus proving the lemma. ✷

## 20.2.2 The discrete process

The discrete process starts with $S = \{ s _ { 1 } \}$ and adds vertices to $S$ in increasing order of their distance from $s _ { 1 }$ . Essentially, it involves executing a shortest path computation from the root. Clearly, the sets of vertices found by both processes are the same.

The weight of region S is redefined for the discrete process as follows:

$$
\operatorname{wt} (S) = \operatorname{wt} (s _ {1}) + \sum_ {e} c _ {e} d _ {e},
$$

where the sum is over all edges that have at least one endpoint in $S ,$ and $\mathrm { w t } ( s _ { 1 } ) = F / k$ . The discrete process stops at the first point when $c ( S ) \ \leq$ $\varepsilon \mathrm { w t } ( S )$ , where $\varepsilon$ is again $2 \ln ( k + 1 )$ . Notice that for the same set $S ,$ wt(S) in the discrete process is at least as large as that in the continuous process.

Therefore, the discrete process cannot terminate with a larger set than that found by the continuous process. Hence, the set $S$ found contains no source– sink pair.

## 20.2.3 Finding successive regions

The first region is found in graph G, starting with any one of the sources as the root. Successive regions are found iteratively. Let $G _ { 1 } = G$ and $S _ { 1 }$ be the region found in $G _ { 1 }$ . Consider a general point in the algorithm when regions $S _ { 1 } , \ldots , S _ { i - 1 }$ have already been found. Now, $G _ { i }$ is defined to be the graph obtained by removing vertices $S _ { 1 } \cup \dots \cup S _ { i - 1 }$ , together with all edges incident at them from $G .$

If $G _ { i }$ does not contain a source–sink pair, we are done. Otherwise, we pick the source of such a pair, say $s _ { j } .$ , as the root, define its weight to be $F / k ,$ and grow a region in $G _ { i }$ . All definitions, such as distance and weight, are w.r.t. graph $G _ { i }$ . We will denote these with a subscript of $G _ { i }$ . Also, for a set of vertices S in $G _ { i } , c _ { G _ { i } } ( S )$ will denote the total capacity of edges incident at S in $G _ { i }$ , i.e., the total capacity of edges in $\delta _ { \boldsymbol { G } _ { i } } ( \boldsymbol { S } )$ . As before, the value of ε is $2 \ln ( k + 1 )$ , and the terminating condition is $c _ { G _ { i } } ( S _ { i } ) \leq \varepsilon \mathrm { w t } _ { G _ { i } } ( S _ { i } )$ . Notice that in each iteration the root is the only vertex that is defined to have nonzero weight.

![](images/811b5833d4d050d51783d2cb89cd5ed7c212db565dd7a38f39feb07a242d82e3.jpg)

In this manner, we will find regions $S _ { 1 } , \ldots , S _ { l } , l \le k .$ , and will output the set $M = \delta _ { G _ { 1 } } ( S _ { 1 } ) \cup \ldots \cup \delta _ { G _ { l } } ( S _ { l } )$ . Since edges of each cut are removed from the graph for successive iterations, the sets in this union are disjoint, and $\begin{array} { r } { c ( M ) = \sum _ { i } c _ { G _ { i } } ( S _ { i } ) } \end{array}$

The algorithm is summarized below. Notice that while a region is growing, edges with large distance labels will remain in its cut for a longer time, and thus are more likely to be included in the multicut found. (Of course, the precise time that an edge remains in the cut is given by the diference between the distances from the root to the two endpoints of the edge.) As promised, the algorithm indirectly gives preference to edges with large distance labels.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 20.4 (Minimum multicut)
1. Find an optimal solution to the LP (20.2), thus obtaining distance labels for edges of G.
2.  $\varepsilon \leftarrow 2\ln(k+1)$ ,  $H \leftarrow G$ ,  $M \leftarrow \emptyset$ ;
3. While  $\exists$  a source–sink pair in H do:
Pick such a source, say  $s_{j}$ ;
Grow a region S with root  $s_{j}$  until  $c_{H}(S) \leq \varepsilon \operatorname{wt}_{H}(S)$ ;
 $M \leftarrow M \cup \delta_{H}(S)$ ;
 $H \leftarrow H$  with vertices of S removed;
4. Output M.
</div>

## Lemma 20.5 The set M found is a multicut.

Proof: We need to prove that no region contains a source–sink pair. In each iteration i, the sum of weights of edges of the graph and the weight defined on the current root is bounded by $F + F / k$ . By the proof of Lemma 20.3, the continuous region growing process is guaranteed to encounter the terminating condition before the radius of the region becomes $1 / 2$ . Therefore, the distance between a pair of vertices in the region, $S _ { i } ,$ found by the discrete process is also bounded by 1. Notice that we had defined these distances w.r.t. graph $G _ { i } .$ Since $G _ { i }$ is a subgraph of G, the distance between a pair of vertices in G cannot be larger than that in $G _ { i }$ . Hence, $S _ { i }$ contains no source–sink pair. ✷

## Lemma 20.6 $c ( M ) \leq 2 \varepsilon F = 4 \ln ( k + 1 ) F .$

Proof: In each iteration $i ,$ by the terminating condition we have $c _ { G _ { i } } ( S _ { i } ) \leq$ $\varepsilon \mathrm { w t } _ { G _ { i } } ( S _ { i } )$ . Since all edges contributing to $\mathrm { w t } _ { G _ { i } } ( S _ { i } )$ ) will be removed from the graph after this iteration, each edge of G contributes to the weight of at most one region. The total weight of all edges of G is $F .$ Since each iteration helps disconnect at least one source–sink pair, the number of iterations is bounded by $k .$ . Therefore, the total weight attributed to source vertices is at most $F .$ Summing gives:

$$
c (M) = \sum_ {i} c _ {G _ {i}} (S _ {i}) \leq \varepsilon \left(\sum_ {i} \operatorname{wt} _ {G _ {i}} (S _ {i})\right) \leq \varepsilon \left(k \frac {F}{k} + \sum_ {e} c _ {e} d _ {e}\right) = 2 \varepsilon F.
$$

✷

Theorem 20.7 Algorithm $\it 2 0 . 4$ achieves an approximation guarantee of O(log k) for the minimum multicut problem.

Proof: The proof follows from Lemmas 20.5 and 20.6, and from the fact that the value of the fractional multicut, $F$ , is a lower bound on the minimum multicut. ✷

Exercise 20.6 justifies the choice of $\mathrm { w t } ( s _ { 1 } ) = F / k$

Corollary 20.8 In an undirected graph with k source–sink pairs,

$$
\max _ {m / c \text {flow} F} | F | \leq \min _ {\text {multicut} C} | C | \leq O (\log k) \left(\max _ {m / c \text {flow} F} | F |\right),
$$

where $| F |$ represents the value of multicommodity flow $F _ { s }$ , and $| C |$ represents the capacity of multicut C.

## 20.3 A tight example

Example 20.9 We will construct an infinite family of graphs for which the integrality gap for $\mathrm { L P _ { \Delta } \left( 2 0 . 2 \right) }$ is $\itOmega \mathrm { ( l o g } k \mathrm { ) }$ , thereby showing that our analysis of Algorithm 20.4 and the approximate max-flow min-multicut theorem presented in Corollary 20.8 are tight within constant factors.

The construction uses expander graphs. An expander is a graph $G =$ $( V , E )$ in which every vertex has the same degree, say $d ,$ and for any nonempty subset $S \subset V$ ,

$$
| E (S, \overline {{S}}) | > \min (| S |, | \overline {{S}} |),
$$

where $E ( S , { \overline { { S } } } )$ denotes the set of edges in the cut $( S , { \overline { { S } } } ) , { \mathrm { ~ i . e . . } }$ edges that have one endpoint in S and the other in ${ \overline { { S } } } .$ Standard probabilistic arguments show that almost every constant degree graph, with $d \geq 3 .$ , is an expander (see Section 20.6). Let H be such a graph containing k vertices.

Source–sink pairs are designated in H as follows. Consider a breadth first search tree rooted at some vertex v. The number of vertices within distance $\alpha - 1$ of vertex v is at most $1 + d + d ^ { 2 } + \ldots + d ^ { \alpha - 1 } < d ^ { \alpha }$ . Picking $\alpha = \lfloor \log _ { d } k / 2 \rfloor$ ensures that at least $k / 2$ vertices are at a distance $\geq \alpha$ from v. Let us say that a pair of vertices are a source–sink pair if the distance between them is at least α. Therefore, we have chosen $\Theta ( k ^ { 2 } )$ pairs of vertices as source–sink pairs.

Each edge in H is of unit capacity. Thus, the total capacity of edges of $H$ is $O ( k )$ . Since the distance between each source–sink pair is $\varOmega ( \log k )$ , any flow path carrying a unit of flow uses up $\itOmega \mathrm { ( l o g } k \mathrm { ) }$ units of capacity. Therefore, the value of maximum multicommodity flow in H is bounded by $O ( k / \log k )$ Next we will prove that a minimum multicut in H, say M, has capacity $\varOmega ( k )$ , thereby proving the claimed integrality gap. Consider the connected components obtained by removing M from H.

Claim 20.10 Each connected component has at most $k / 2$ vertices.

Proof: Suppose a connected component has strictly more than $k / 2$ vertices. Pick an arbitrary vertex v in this component. By the argument given above, the number of vertices that are within distance $\alpha - 1$ of v in the entire graph H is $< d ^ { \alpha } \leq k / 2$ . Thus, there is a vertex u in the component such that the distance between u and v is at least α, i.e., u and v form a source–sink pair. Thus removal of M has failed to disconnect a source–sink pair, leading to a contradiction. ✷

By Claim 20.10, and the fact that H is an expander, each component S has $| \delta ( S ) | \ge | S |$ . Since each vertex of H is in one of the components, $\Sigma _ { S } \left| \delta ( S ) \right| \geq k .$ , where the sum is over all connected components. Since an edge contributes to the cuts of at most two components, the number of edges crossing components is $\varOmega ( k )$ . This gives the desired lower bound on the minimum multicut.

Next, let us ensure that the number of source–sink pairs defined in the graph is not related to the number of vertices in it. Notice that replacing an edge of H by a path of unit capacity edges does not change the value of maximum flow or minimum multicut. Using this operation we can construct from H a graph G having n vertices, for arbitrary $n \geq k$ . The integrality gap of LP (20.2) for G is $\varOmega ( \log k )$

## 20.4 Some applications of multicut

We will obtain an $O ( \log n )$ factor approximation algorithm for the following problem by reducing to the minimum multicut problem. See Exercise 20.7 for further applications.

Problem 20.11 (2CNF≡ clause deletion) A 2CNF≡ formula consists of a set of clauses of the form $( u \equiv v )$ , where u and v are literals. Let F be such a formula, and wt be a function assigning nonnegative rational weights to its clauses. The problem is to delete a minimum weight set of clauses of $F$ so that the remaining formula is satisfiable.

Given a 2CNF≡ formula F on n Boolean variables, let us define graph $G ( F )$ with edge capacities as follows: The graph has 2n vertices, one corresponding to each literal. Corresponding to each clause $( p \equiv q )$ we include the two edges $( p , q )$ and $( { \overline { { p } } } , { \overline { { q } } } )$ , each having capacity equal to the weight of the clause $( p \equiv q )$ ).

Notice that the two clauses $( p \equiv q )$ and $( { \overline { { p } } } \equiv { \overline { { q } } } )$ are equivalent. We may assume w.l.o.g. that $F$ does not contain two such equivalent clauses, since we can merge their weights and drop one of these clauses. With this assumption each clause corresponds to two distinct edges in $G ( F )$

Lemma 20.12 Formula F is satisfiable $i f f$ no connected component of $G ( F )$ contains a variable and its negation.

Proof: If $( p , q )$ is an edge in $G ( F )$ then the literals $p$ and $q$ must take the same truth value in every satisfying truth assignment. Thus, all literals of a connected component of $G ( F )$ are forced to take the same truth value. Therefore, if $F$ is satisfiable, no connected component in $G ( F )$ contains a variable and its negation.

Conversely, notice that if literals p and q occur in the same connected component, then so do their negations. If no connected component contains a variable and its negation, the components can be paired so that in each pair, one component contains a set of literals and the other contains the complementary literals. For each pair, set the literals of one component to true and the other to false to obtain a satisfying truth assignment. ✷

For each variable and its negation, designate the corresponding vertices in $G ( F )$ to be a source–sink pair, thus defining n source–sink pairs. Let M be a minimum multicut in $G ( F )$ and C be a minimum weight set of clauses whose deletion makes $F$ satisfiable. In general, M may have only one of the two edges corresponding to a clause.

Lemma 20.13 w $\operatorname { t } ( C ) \leq c ( M ) \leq 2 \cdot \operatorname { w t } ( C )$

Proof: Delete clauses corresponding to edges of M from F to get formula $F ^ { \prime }$ The weight of clauses deleted is at most $c ( M )$ . Since $G ( F ^ { \prime } )$ does not contain any edges of M, it does not have any component containing a variable and its negation. By Lemma $2 0 . 1 2 , F ^ { \prime }$ is satisfiable, thus proving the first inequality.

Next, delete from $G ( F )$ the two edges corresponding to each clause in $C .$ This will disconnect all source–sink pairs. Since the capacity of edges deleted is $2 \mathrm { w t } ( C )$ , this proves the second inequality. ✷

Since we can approximate minimum multicut to within an $O ( \log n )$ factor, we get:

Theorem 20.14 There is an $O ( \log n )$ factor approximation algorithm for Problem 20.11.

## 20.5 Exercises

20.1 $\mathrm { B y }$ defining for each edge e and commodity i a flow variable $f _ { e , i }$ , give an LP that is equivalent to LP (20.1) and has polynomially many variables.

Obtain the dual of this program and show that it is equivalent to LP (20.2); however, unlike LP (20.2), it has only polynomially many constraints.

20.2 Let d be an optimal solution to LP (20.2). Show that d must satisfy the triangle inequality.

20.3 Intuitively, our goal in picking a multicut is picking edges that are bottlenecks for multicommodity flow. In this sense, D is a very good starting point: prove that D is precisely the set of edges that are saturated in every maximum multicommodity flow.

Hint: Use complementary slackness conditions.

20.4 Give an example to show that picking all of D gives an $\varOmega ( n )$ factor for multicut.

20.5 Consider the following growth process. $W ( t )$ denotes the weight at time t. Assume that the initial weight is $W ( 0 ) = W _ { 0 }$ , and that at each point the rate of growth is proportional to the current weight, i.e.,

$$
\mathrm{d} W (t) = \varepsilon W (t) \mathrm{d} t.
$$

Give the function $W ( t )$ . Next, assume that $W _ { 0 } = F / k$ and that $W ( 1 / 2 ) =$ $F + F / k$ . What is $\varepsilon ?$

Hint: $W ( t ) = W _ { 0 } e ^ { \varepsilon t }$ and $\varepsilon = 2 \ln ( k + 1 )$

20.6 This exercise justifies the choice of $\mathrm { w t } ( s _ { 1 } )$ , which was fixed to be $F / k$ . Suppose we fix it at $W _ { 0 }$ ${ \mathrm { C l e a r l y } } , \ \varepsilon$ is inversely related to $W _ { 0 }$ (see Lemma 20.3). However, the approximation factor of the algorithm is given by $\varepsilon ( F + k W _ { 0 } )$ (see Lemma 20.6). For what value of $W _ { 0 }$ is the approximation factor minimized?

20.7 Consider the following problem, which has applications in VLSI design.

Problem 20.15 (Graph bipartization by edge deletion) Given an edge weighted undirected graph $G = ( V , E )$ , remove a minimum weight set of edges to leave a bipartite graph.

Obtain an $O ( \log n )$ factor approximation algorithm for this problem by reducing it to Problem 20.11.

20.8 (Even, Naor, Schieber, and Rao [75]) This exercise develops an $O ( \log ^ { 2 } n )$ factor algorithm for the following problem.

Problem 20.16 (Minimum length linear arrangement) Given an undirected graph $G = ( V , E )$ , find a numbering of its vertices from 1 to $n , h : V \to \{ 1 , . . . , n \}$ , so as to minimize

$$
\sum_ {(u, v) \in E} | h (u) - h (v) |.
$$

1. Show that the following is an LP-relaxation of this problem. This LP has a variable $d _ { e }$ for each edge $\textit { e } \in \textit { E }$ , which we will interpret as a distance label. For any distance label assignment d to the edges of $G ,$ define $\mathrm { d i s t } _ { d } ( u , v )$ to be the length of the shortest path from u to v in G. Give a polynomial time separation oracle for this LP, thereby showing that it can be solved in polynomial time.

$$
\begin{array}{l l} \text {minimize} & \sum_ {e \in E} d _ {e} \\ & \sum_ {u \in S} \mathrm{dist} _ {d} (u, v) \geq \frac {1}{4} (| S | ^ {2} - 1), \quad S \subseteq V, v \in S \\ & d _ {e} \geq 0, \qquad \qquad \qquad \qquad e \in E \end{array}\tag{20.3}
$$

2. Let d be an optimal solution to LP (20.3). Show that for any $S \subseteq V , v \in S$ there is a vertex $u \in S$ such that dis $_ d ( u , v ) \geq ( | S | + 1 ) / 4$

3. For $S \subseteq V$ , define $\mathrm { w t } ( S )$ to be the sum of distance labels of all edges having both endpoints in S. Also, define $c ( S , { \overline { { S } } } )$ to be the number of edges in the cut (S, S). Give a region growing process similar to that described in Section 20.2.1 that finds a cut $( S , { \breve { S } } )$ in G with w $( S ) \leq \mathrm { w t } ( \overline { { S } } )$ such that $c ( S , { \overline { { S } } } )$ is $O ( \operatorname { w t } ( S ) ( \log n ) / n )$

4. Show that a divide-and-conquer algorithm that recursively finds a numbering for vertices in S from 1 to |S|, and a numbering for vertices in $\overline { S }$ from $\vert S \vert + 1$ to n achieves an approximation guarantee of $O ( \log ^ { 2 } n )$ Hint: Assuming each edge in the cut (S, S) is of length $n - 1$ , write a suitable recurrence for the cost incurred by the algorithm.

## 20.6 Notes

Theorem 20.7 and its corollary are due to Garg, Vazirani, and Yannakakis [97]. Problem 20.11 was introduced in Klein, Rao, Agrawal, and Ravi [173]. For showing existence of expanders via a probabilistic argument, see Pinsker [220].
