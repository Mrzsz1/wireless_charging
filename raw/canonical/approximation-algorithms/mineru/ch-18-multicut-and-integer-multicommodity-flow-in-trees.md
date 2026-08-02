---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-18"
chapter_number: 18
chapter_title: "Multicut and Integer Multicommodity Flow in Trees"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 164
source_page_end: 172
printed_page_start: 146
printed_page_end: 154
part_ids: ["approximation-algorithms-ch-18-part-019"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Multicut and Integer Multicommodity Flow in Trees (MinerU semantic layer)

<!-- source-pages: 164-172; printed-pages: 146-154; mineru-part: approximation-algorithms-ch-18-part-019 -->

## 18 Multicut and Integer Multicommodity Flow in Trees

The theory of cuts in graphs occupies a central place not only in the study of exact algorithms, but also approximation algorithms. We will present some key results in the next four chapters. This will also give us the opportunity to develop further the two fundamental algorithm design techniques introduced in Chapters 14 and 15.

In Chapter 15 we used the primal–dual schema to derive a factor 2 algorithm for the weighted vertex cover problem. This algorithm was particularly easy to obtain because the relaxed dual complementary slackness conditions were automatically satisfied in any integral solution. In this chapter, we will use the primal–dual schema to obtain an algorithm for a generalization of this problem (see Exercise 18.1). This time, enforcing relaxed dual complementary slackness conditions will be a nontrivial part of the algorithm. Furthermore, we will introduce the procedure of reverse delete, which will be used in several other primal–dual algorithms.

## 18.1 The problems and their LP-relaxations

The following is an important generalization of the minimum s–t cut problem. In fact, it also generalizes the multiway cut problem (Problem 4.1).

Problem 18.1 (Minimum multicut) Let $G { = } ( V , E )$ be an undirected graph with nonnegative capacity $c _ { e }$ for each edge $e \in E$ . Let $\{ ( s _ { 1 } , t _ { 1 } ) , \ldots , ( s _ { k } , t _ { k } ) \}$ be a specified set of pairs of vertices, where each pair is distinct, but vertices in diferent pairs are not required to be distinct. A multicut is a set of edges whose removal separates each of the pairs. The problem is to find a minimum capacity multicut in $G _ { \ l }$

The minimum $^ { s - t }$ cut problem is the special case of multicut for $k = 1$ Problem 18.1 generalizes multiway cut because separating terminals $s _ { 1 } , \ldots , s _ { l }$ is equivalent to separating all pairs $( s _ { i } , s _ { j } )$ ), for $1 \leq i < j \leq l .$ . This observation implies that the minimum multicut problem is NP-hard even for $k = 3$ , since the multiway cut problem is NP-hard for the case of 3 terminals.

In Chapter 20 we will obtain an $O ( \log k )$ factor approximation algorithm for the minimum multicut problem. In this chapter, we will obtain a factor 2 algorithm for the special case when $G$ is restricted to be a tree. Since $G$ is a tree, there is a unique path between $s _ { i }$ and $t _ { i } .$ and the multicut must pick an edge on this path to disconnect $s _ { i }$ from $t _ { i } .$ . Although the problem looks deceptively simple, Exercise 18.1 should convince the reader that this is not so. The minimum multicut problem is NP-hard even if restricted to trees of height 1 and unit capacity edges.

Since we want to apply LP-duality theory to design the algorithm, let us first give an integer programming formulation for the problem and obtain its LP-relaxation. Introduce a $0 / 1$ variable $d _ { e }$ for each edge $e \in E .$ , which will be set to 1 if e is picked in the multicut. Let $p _ { i }$ denote the unique path between $s _ { i }$ and $t _ { i }$ in the tree.

$$
\begin{array}{l l} \text {minimize} & \sum_ {e \in E} c _ {e} d _ {e} \\ \text {subject to} & \sum_ {e \in p _ {i}} d _ {e} \geq 1, \quad i \in \{1, \ldots , k \} \\ & d _ {e} \in \{0, 1 \}, \quad e \in E \end{array}
$$

The LP-relaxation is obtained by replacing the constraint $d _ { e } ~ \in ~ \{ 0 , 1 \}$ by $d _ { e } \geq 0$ . As in the derivation of LP (13.2), there is no need to add the constraint $d _ { e } \leq 1$ explicitly.

$$
\begin{array}{l l} \text { minimize } & \sum_ {e \in E} c _ {e} d _ {e} \\ \text { subject   to } & \sum_ {e \in p _ {i}} d _ {e} \geq 1, \quad i \in \{1, \ldots , k \} \\ & d _ {e} \geq 0, \quad e \in E \end{array}\tag{18.1}
$$

We can now think of $d _ { e }$ as specifying the fractional extent to which edge e is picked. A solution to this linear program is a fractional multicut: on each path $p _ { i } ,$ the sum of fractions of edges picked is at least 1. In general, minimum fractional multicut may be strictly cheaper than minimum integral multicut. This is illustrated in Example 18.2.

We will interpret the dual program as specifying a multicommodity flow in $G ,$ , with a separate commodity corresponding to each vertex pair $( s _ { i } , t _ { i } )$ Dual variable $f _ { i }$ will denote the amount of this commodity routed along the unique path from $s _ { i }$ to $t _ { i }$ .

$$
\begin{array}{l l} \text {maximize} & \sum_ {i = 1} ^ {k} f _ {i} \\ \text {subject to} & \sum_ {i: e \in p _ {i}} f _ {i} \leq c _ {e}, \quad e \in E \\ & f _ {i} \geq 0, \quad i \in \{1, \ldots , k \} \end{array}\tag{18.2}
$$

The commodities are routed concurrently. The object is to maximize the sum of the commodities routed, subject to the constraint that the sum of flows routed through an edge is bounded by the capacity of the edge. Notice that the sum of flows through an edge $( u , v )$ includes flow going in either direction, u to v and v to u.

$\mathrm { B y }$ the weak duality theorem, a feasible multicommodity flow gives a lower bound on the minimum fractional multicut and hence also on the minimum integral multicut. By the LP-duality theorem, minimum fractional multicut equals maximum multicommodity flow.

Example 18.2 Consider the following graph with unit capacity edges and 3 vertex pairs:

![](images/bad683bdac0a0b46112d7fcaa79f088be20b4fdf0db89f28e81dea9e66fe1fd5.jpg)

The arrows show how to send $3 / 2$ units of flow by sending $1 / 2$ unit of each commodity. Picking each edge to the extent of $1 / 2$ gives a multicut of capacity $3 / 2$ as well. These must be optimal solutions to the primal and dual programs. On the other hand, any integral multicut must pick at least two of the three edges in order to disconnect all three pairs. Hence, minimum integral multicut has capacity 2. ✷

Finally, let us state one more problem.

Problem 18.3 (Integer multicommodity flow) Graph G and the source– sink pairs are specified as in the minimum multicut problem; however, the edge capacities are all integral. A separate commodity is defined for each $( s _ { i } , t _ { i } )$ pair. The object is to maximize the sum of the commodities routed, subject to edge capacity constraints and subject to routing each commodity integrally.

Let us consider this problem when G is restricted to be a tree. If in (18.2), the variables are constrained to be nonnegative integers, we would get an integer programming formulation for this problem. Clearly, the objective function value of this integer program is bounded by that of the linear program (18.2).

Furthermore, the best fractional flow may be strictly larger. For instance, in Example 18.2, maximum integral multicommodity flow is 1, since sending 1 unit of any of the three commodities will saturate two of the edges. This problem is NP-hard, even for trees of height 3 (though the capacity has to be arbitrary).

## 18.2 Primal–dual schema based algorithm

We will use the primal–dual schema to obtain an algorithm that simultaneously finds a multicut and an integer multicommodity flow that are within a factor of 2 of each other, provided the given graph is a tree. Hence, we get approximation algorithms for both problems, of factor 2 and $1 / 2$ , respectively.

Let us define the multicut LP to be the primal program. An edge e is saturated if the total flow through it equals its capacity. We will ensure primal complementary slackness conditions, $\operatorname { i . e . , } \alpha = 1$ , and relax the dual conditions with $\beta = 2$ , where α and $\beta$ are the parameters used in the general description of the primal–dual schema given in Chapter 15.

Primal conditions: For each $\textstyle e \in E , d _ { e } \neq 0 \Rightarrow \sum _ { i : \ e \in p _ { i } } f _ { i } = c _ { e }$

Equivalently, any edge picked in the multicut must be saturated.

Relaxed dual conditions: For each $\begin{array} { r } { i \in \{ 1 , . . . , k \} , f _ { i } \neq 0 \Rightarrow \sum _ { e \in p _ { i } } d _ { e } \leq 2 } \end{array}$ Equivalently, at most two edges can be picked from a path carrying nonzero flow. (Clearly, we must pick at least one edge from each $( s _ { i } , t _ { i } )$ ) path simply to ensure the feasibility of the multicut.)

Let us root the tree $G$ at an arbitrary vertex. Define the depth of vertex v to be the length of the path from v to the root; the depth of the root is $0 .$ For two vertices $u , v \in V$ , let lca(u, v) denote the lowest common ancestor of u and $v ,$ i.e., the minimum depth vertex on the path from u to $v .$ Let $e _ { 1 }$ and $e _ { 2 }$ be two edges on a path from a vertex to the root. $\operatorname { I f } \ e _ { 1 }$ occurs before $e _ { 2 }$ on this path, then $e _ { 1 }$ is said to be deeper than $e _ { 2 }$

The algorithm starts with an empty multicut and flow, and iteratively improves the feasibility of the primal solution and the optimality of the dua solution. In an iteration, it picks the deepest unprocessed vertex, say $v ,$ and greedily routes integral flow between pairs that have v as their lowest common ancestor. When no more flow can be routed between these pairs, all edges that were saturated in this iteration are added to the list $D$ in arbitrary order. When all the vertices have been processed, D will be a multicut; however, it may have redundant edges. To remove them, a reverse delete step is performed: edges are considered in the reverse of the order in which they were added to $D _ { \ast }$ and if the deletion of edge e from D still gives a valid multicut, e is discarded from D.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 18.4 (Multicut and integer multicommodity flow in trees)

1. Initialization:  $f \leftarrow 0$ ;  $D \leftarrow \emptyset$ .

2. Flow routing: For each vertex v, in nonincreasing order of depth, do:
    For each pair  $(s_i, t_i)$  such that  $\text{lca}(s_i, t_i) = v$ , greedily route integral flow from  $s_i$  to  $t_i$ .

    Add to D all edges that were saturated in the current iteration in arbitrary order.

3. Let  $e_1, e_2, \ldots, e_l$  be the ordered list of edges in D.

4. Reverse delete: For j = l downto 1 do:
    If  $D - \{e_j\}$  is a multicut in G, then  $D \leftarrow D - \{e_j\}$ .

5. Output the flow and multicut D.
</div>

Lemma 18.5 Let $( s _ { i } , t _ { i } )$ be a pair with nonzero $f l o w ,$ and let lca $( s _ { i } , t _ { i } ) = v$ At most one edge is picked in the multicut from each of the two paths, $s _ { i }$ to v and $t _ { i }$ to v.

Proof: The argument is the same for each path. Suppose two edges e and $e ^ { \prime }$ are picked from the $s _ { i } - v$ path, with e being the deeper edge. Clearly, $e ^ { \prime }$ must be in D all through reverse delete. Consider the moment during reverse delete when edge e is being tested. Since e is not discarded, there must be a pair, say $( s _ { j } , t _ { j } )$ , such that e is the only edge of D on the $s _ { j } - t _ { j }$ path. Let u be the lowest common ancestor of $s _ { j }$ and $t _ { j }$ . Since $e ^ { \prime }$ does not lie on the $s _ { j } - t _ { j }$ path, u must be deeper than $e ^ { \prime } { \mathrm { . } }$ and hence deeper than v. After u has been processed, D must contain an edge from the $s _ { j } - t _ { j }$ path, say $e ^ { \prime \prime }$

![](images/daa29a6d76f6daf2f9dc2bcbd973201362b0a4aaed0bf161303984805a2285b9.jpg)  
Since nonzero flow has been routed from $s _ { i }$ to $t _ { i } ,$ , e must be added during or after the iteration in which v is processed. Since v is an ancestor of $u , e$ is added after $e ^ { \prime \prime }$ . So $e ^ { \prime \prime }$ must be in $D$ when e is being tested. This contradicts the fact that at this moment e is the only edge of $D$ on the $s _ { j } - t _ { j }$ path. ✷

Theorem 18.6 Algorithm 18.4 achieves approximation guarantees of factor 2 for the minimum multicut problem and factor 1/2 for the maximum integer multicommodity flow problem on trees.

Proof: The flow found at the end of Step 2 is maximal, and since at this point D contains all the saturated edges, D is a multicut. Since the reverse delete step only discards redundant edges, D is a multicut after this step as well. Thus, feasible solutions have been found for both the flow and the multicut.

Since each edge in the multicut is saturated, the primal conditions are satisfied. By Lemma 18.5, at most two edges have been picked in the multicut from each path carrying nonzero flow. Therefore, the relaxed dual conditions are also satisfied. Hence, by Proposition 15.1, the capacity of the multicut found is within twice the flow. Since a feasible flow is a lower bound on the optimal multicut, and a feasible multicut is an upper bound on the optimal integer multicommodity flow, the claim follows. ✷

Finally, we obtain the following approximate min–max relation from Theorem 18.6:

Corollary 18.7 On trees with integer edge capacities,

$$
\max _ {i n t. f l o w F} | F | \leq \min _ {m u l t i c u t C} c (C) \leq 2 \cdot \max _ {i n t. f l o w F} | F |,
$$

where $| F |$ represents the value of flow function F and $c ( C )$ represents the capacity of multicut C.

In Chapter 20 we will present an $O ( \log k )$ factor algorithm for the minimum multicut problem in general graphs; once again, the lower bound used is an optimal fractional multicut. On the other hand, no nontrivial approximation algorithms are known for the integer multicommodity flow problem in graphs more general than trees. As shown in Example 18.8, even for planar graphs, the integrality gap of an LP analogous to (18.2) is lower bounded by $n / 2$ , where n is the number of source–sink pairs specified.

Example 18.8 Consider the following planar graph with n source–sink pairs. Every edge is of unit capacity. Any pair of paths between the ith and jth source–sink pairs intersect in at least one unit capacity edge. The magnified part shows how this is arranged at each intersection. Thus, sending one unit of any commodity blocks all other commodities. On the other hand, half a unit of each commodity can be routed simultaneously.

![](images/0276e5ca0913c2d66b4d476a305c2d5edcd2b9fea018f602fb5ba76f1ca86610.jpg)

## 18.3 Exercises

18.1 (Garg, Vazirani, and Yannakakis [98]) Give approximation factor preserving reductions between the following pairs of problems:

(a) cardinality vertex cover and minimum multicut in trees of height 1 and unit capacity edges,

(b) vertex cover with arbitrary weights and minimum multicut in trees of height 1 and arbitrary edge capacities.

Hint: Given a vertex cover instance $G ,$ construct a height 1 tree that has a leaf corresponding to each vertex of G and a source–sink pair corresponding to each edge of G.

18.2 The following is a well-studied polynomial time solvable generalization of the maximum matching problem. Given an undirected graph $G = ( V , E )$ and a function $b : V \to \mathbf { Z } ^ { + }$ , a b-matching is a set of edges, $E ^ { \prime } \subseteq E ,$ with associated multiplicities, m : $E ^ { \prime } \to \mathbf { Z } ^ { + }$ , such that each vertex $v \in V$ has at most $b ( v )$ edges incident at it, counting multiplicities. The size of this bmatching is the sum of multiplicities of edges in $E ^ { \prime }$ . The maximum b-matching problem is that of finding a b-matching of maximum size. Show that the following pairs of problems are polynomial time equivalent:

(a) maximum integer multicommodity flow problem on trees of height 1 and unit capacity edges, and the maximum matching problem,

(b) maximum integer multicommodity flow problem on trees of height 1 and arbitrary capacity edges, and the maximum b-matching problem.

18.3 (Garg, Vazirani, and Yannakakis [98]) Give a polynomial time algorithm for computing a maximum integer multicommodity flow on unit capacity trees of arbitrary height.

Hint: Apply dynamic programming, and use a subroutine for the maximum matching problem.

18.4 If Step 2 of Algorithm 18.4 is modified to include only one saturated edge after each iteration, show that the resulting set D may not even be a multicut.

18.5 If Step 4 in Algorithm 18.4 is removed, or is changed to a forward delete, show that its approximation factor is unbounded.

18.6 Modify step 4 in Algorithm 18.4 to: sort edges in D by decreasing capacity and remove redundant edges in this order. What factor can you prove for the modified algorithm?

18.7 Give tight examples for Algorithm 18.4 for both multicut and integer multicommodity flow.

18.8 Prove that if e and $e ^ { \prime }$ are both in D in Step 3 of Algorithm 18.4, and e is deeper than $e ^ { \prime } .$ , then e is added before or in the same iteration as $e ^ { \prime }$

18.9 Find the best integral and fractional multicut and the best multicommodity flow in the following graph. All capacities are 1, and the specified pairs are $( s _ { 1 } , t _ { 1 } ) , \dotsc , ( s _ { 5 } , t _ { 5 } )$ . Notice that the optimal fractional multicut is not half integral. In contrast, the LP-relaxation of the multiway cut problem always has a half-integral optimal solution (see Chapter 19).

![](images/7cdd68207fa1f5b519d661350d472de7c6a5e43911080fc732d6b25448547bc7.jpg)

## 18.4 Notes

Algorithm 18.4 is due to Garg, Vazirani, and Yannakakis [98]. For recent results on the integer multicommodity flow problem, see Guruswami, Khanna, Rajaraman, Sheperd, and Yannakakis [118].
