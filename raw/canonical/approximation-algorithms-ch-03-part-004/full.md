---
title: "approximation-algorithms-ch-03-part-004"
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
pdf_path: "work/core-books/approximation-algorithms/parts/approximation-algorithms-ch-03-part-004.pdf"
raw_md: "raw/canonical/approximation-algorithms-ch-03-part-004/full.md"
---
## 3 Steiner Tree and TSP

In this chapter, we will present constant factor algorithms for two fundamental problems, metric Steiner tree and metric TSP. The reasons for considering the metric case of these problems are quite diferent. For Steiner tree, this is the core of the problem – the rest of the problem reduces to this case. For TSP, without this restriction, the problem admits no approximation factor, assuming $\mathbf { P } \neq \mathbf { N P }$ . The algorithms, and their analyses, are similar in spirit, which is the reason for presenting these problems together.

## 3.1 Metric Steiner tree

The Steiner tree problem was defined by Gauss in a letter he wrote to Schumacher (reproduced on the cover of this book). Today, this problem occupies a central place in the field of approximation algorithms. The problem has a wide range of applications, all the way from finding minimum length interconnection of terminals in VLSI design to constructing phylogeny trees in computational biology. This problem and its generalizations will be studied extensively in this book, see Chapters 22 and 23.

Problem 3.1 (Steiner tree) Given an undirected graph $G = ( V , E )$ with nonnegative edge costs and whose vertices are partitioned into two sets, required and Steiner, find a minimum cost tree in G that contains all the required vertices and any subset of the Steiner vertices.

We will first show that the core of this problem lies in its restriction to instances in which the edge costs satisfy the triangle inequality, i.e., G is a complete undirected graph, and for any three vertices u, v, and w, cost $( u , v ) \leq$ $\mathrm { c o s t } ( u , w ) + \mathrm { c o s t } ( v , w )$ . Let us call this restriction the metric Steiner tree problem.

Theorem 3.2 There is an approximation factor preserving reduction from the Steiner tree problem to the metric Steiner tree problem.

Proof: We will transform, in polynomial time, an instance I of the Steiner tree problem, consisting of graph $G = ( V , E )$ , to an instance I<sup></sup> of the metric Steiner tree problem as follows. Let $G ^ { \prime }$ be the complete undirected graph on vertex set V. Define the cost of edge $( u , v )$ in $G ^ { \prime }$ to be the cost of a shortest u–v path in G. $G ^ { \prime }$ is called the metric closure of $G _ { \ l }$ . The partition of $V$ into required and Steiner vertices in $I ^ { \prime }$ is the same as in $I .$

For any edge $( u , v ) \in E$ , its cost in $G ^ { \prime }$ is no more than its cost in $G .$ Therefore, the cost of an optimal solution in $I ^ { \prime }$ does not exceed the cost of an optimal solution in I.

Next, given a Steiner tree $T ^ { \prime }$ in $I ^ { \prime } ,$ we will show how to obtain, in polynomial time, a Steiner tree $T$ in I of at most the same cost. The cost of an edge $( u , v )$ in $G ^ { \prime }$ corresponds to the cost of a path in $G .$ . Replace each edge of $T ^ { \prime }$ by the corresponding path to obtain a subgraph of G. Clearly, in this subgraph, all the required vertices are connected. However, this subgraph may, in general, contain cycles. If so, remove edges to obtain tree T. This completes the approximation factor preserving reduction. ✷

As a consequence of Theorem 3.2, any approximation factor established for the metric Steiner tree problem carries over to the entire Steiner tree problem.

## 3.1.1 MST-based algorithm

Let R denote the set of required vertices. Clearly, a minimum spanning tree (MST) on R is a feasible solution for this problem. Since the problem of finding an MST is in P and the metric Steiner tree problem is NP-hard, we cannot expect the MST on R to always give an optimal Steiner tree; below is an example in which the MST is strictly costlier.

![](images/1725c1d998f9bba137c768678a6d571d1d1edf03ecf8f4c3e035367ee305c10c.jpg)  
Even ${ \mathrm { s o } } ,$ an MST on R is not much more costly than an optimal Steiner tree:

Theorem 3.3 The cost of an $M S T$ on R is within $2 \cdot \mathrm { O P T }$

Proof: Consider a Steiner tree of cost OPT. $\mathrm { B y }$ doubling its edges we obtain an Eulerian graph connecting all vertices of R and, possibly, some Steiner vertices. Find an Euler tour of this graph, for example by traversing the edges in DFS (depth first search) order:

![](images/d9bbcc5c0467b87eddc5ae74f7e26a99a6466778a744eac4a44101405555650f.jpg)

The cost of this Euler tour is 2 · OPT. Next obtain a Hamiltonian cycle on the vertices of R by traversing the Euler tour and “short-cutting” Steiner vertices and previously visited vertices of R:

![](images/7421bdb1b76158f21bc60ba98e8a19d0282cc6e3f218fb99b2d5959bfd754b52.jpg)

Because of triangle inequality, the shortcuts do not increase the cost of the tour. If we delete one edge of this Hamiltonian cycle, we obtain a path that spans R and has cost at most 2 · OPT. This path is also a spanning tree on R. Hence, the MST on R has cost at most 2 · OPT. ✷

Theorem 3.3 gives a straightforward factor 2 algorithm for the metric Steiner tree problem: simply find an MST on the set of required vertices. As in the case of set cover, the “correct” way of viewing this algorithm is in the setting of LP-duality theory. In Chapters 22 and 23 we will see that LP-duality provides the lower bound on which this algorithm is based and also helps solve generalizations of this problem.

Example 3.4 For a tight example, consider a graph with n required vertices and one Steiner vertex. An edge between the Steiner vertex and a required vertex has cost 1, and an edge between two required vertices has cost 2 (not all edges of cost 2 are shown below). In this graph, any MST on R has cost $2 ( n - 1 )$ , while $\mathrm { O P T } = n$

![](images/5e6ba2cd0cf222e9116483ce9f5c01e6930735711dcaf0e0d34253df050f53ca.jpg)

## 3.2 Metric TSP

The following is a well-studied problem in combinatorial optimization.

Problem 3.5 (Traveling salesman problem (TSP)) Given a complete graph with nonnegative edge costs, find a minimum cost cycle visiting every vertex exactly once.

In its full generality, TSP cannot be approximated, assuming $\mathbf { P } \neq \mathbf { N P }$

Theorem 3.6 For any polynomial time computable function $\alpha ( n )$ , TSP cannot be approximated within a factor of $\alpha ( n )$ , unless $\mathbf { P } = \mathbf { N P }$

Proof: Assume, for a contradiction, that there is a factor $\alpha ( n )$ polynomial time approximation algorithm, ${ \mathcal { A } } ,$ for the general TSP problem. We will show that A can be used for deciding the Hamiltonian cycle problem (which is NPhard) in polynomial time, thus implying $\mathbf { P } = \mathbf { N P }$

The central idea is a reduction from the Hamiltonian cycle problem to TSP, that transforms a graph G on n vertices to an edge-weighted complete graph $G ^ { \prime }$ on n vertices such that

• if G has a Hamiltonian cycle, then the cost of an optimal TSP tour in $G ^ { \prime }$ is $n ,$ and

• if $G$ does not have a Hamiltonian cycle, then an optimal TSP tour in $G ^ { \prime }$ is of cost $> \alpha ( n ) \cdot n$

Observe that when run on graph $G ^ { \prime }$ , algorithm $\mathcal { A }$ must return a solution of cost $\leq \alpha ( n ) \cdot n$ in the first case, and a solution of cost $> \alpha ( n )$ · n in the second case. Thus, it can be used for deciding whether $G$ contains a Hamiltonian cycle.

The reduction is simple. Assign a weight of 1 to edges of $G ,$ , and a weight of $\alpha ( n ) \cdot n$ to nonedges, to obtain $G ^ { \prime }$ . Now, if $G$ has a Hamiltonian cycle, then the corresponding tour in $G ^ { \prime }$ has cost n. On the other hand, if G has no Hamiltonian cycle, any tour in $G ^ { \prime }$ must use an edge of cost $\alpha ( n ) \cdot n ,$ and therefore has cost $> \alpha ( n ) \cdot n$ ✷

Notice that in order to obtain such a strong nonapproximability result, we had to assign edge costs that violate triangle inequality. If we restrict ourselves to graphs in which edge costs satisfy triangle inequality, i.e., consider metric TSP, the problem remains NP-complete, but it is no longer hard to approximate.

## 3.2.1 A simple factor 2 algorithm

We will first present a simple factor 2 algorithm. The lower bound we will use for obtaining this factor is the cost of an MST in G. This is a lower bound because deleting any edge from an optimal solution to TSP gives us a spanning tree of G.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 3.7 (Metric TSP - factor 2)  
1. Find an MST, $T$, of $G$.  
2. Double every edge of the MST to obtain an Eulerian graph.  
3. Find an Eulerian tour, $\mathcal{T}$, on this graph.  
4. Output the tour that visits vertices of $G$ in the order of their first appearance in $\mathcal{T}$. Let $\mathcal{C}$ be this tour.
</div>

Notice that Step 4 is similar to the “short-cutting” step in Theorem 3.3.

Theorem 3.8 Algorithm 3.7 is a factor 2 approximation algorithm for metric TSP.

Proof: As noted above, cost $( T ) \leq \mathrm { O P T }$ . Since T contains each edge of T twice, cost $( \tau ) = 2$ · cost(T). Because of triangle inequality, after the “shortcutting” step, cost $( { \mathcal { C } } ) \leq \mathrm { c o s t } ( { \mathcal { T } } )$ . Combining these inequalities we get that cost $( { \mathcal { C } } ) \leq 2 \cdot \mathrm { O P T }$ ✷

Example 3.9 A tight example for this algorithm is given by a complete graph on n vertices with edges of cost 1 and 2. We present the graph for $n = 6$ below, where thick edges have cost 1 and remaining edges have cost 2. For arbitrary n the graph has 2n−2 edges of cost 1, with these edges forming the union of a star and an $n - 1$ cycle; all remaining edges have cost 2. The optimal TSP tour has cost n, as shown below for n = 6:

![](images/f56c1ef9b1426ffdbb388db958b7343aee49e790b24b136bddb99614beef0ba0.jpg)

Suppose that the MST found by the algorithm is the spanning star created by edges of cost 1. Moreover, suppose that the Euler tour constructed in Step 3 visits vertices in order shown below for $n = 6 \mathrm { : }$

![](images/b41c355bc3c8fe6efdd6c9663bcfe87608726bf4ded6354803150e8c35eb8d20.jpg)  
Then the tour obtained after short-cutting contains $n - 2$ edges of cost 2 and has a total cost of 2n − 2. Asymptotically, this is twice the cost of the optimal TSP tour. ✷

## 3.2.2 Improving the factor to 3/2

Algorithm 3.7 first finds a low cost Euler tour spanning the vertices of $G ,$ and then short-cuts this tour to find a traveling salesman tour. Is there a cheaper Euler tour than that found by doubling an MST? Recall that a graph has an Euler tour if all its vertices have even degrees. Thus, we only need to be concerned about the vertices of odd degree in the MST. Let $V ^ { \prime }$ denote this set of vertices. $\vert V ^ { \prime } \vert$ must be even since the sum of degrees of all vertices in the MST is even. Now, if we add to the MST a minimum cost perfect matching on $V ^ { \prime }$ , every vertex will have an even degree, and we get an Eulerian graph. With this modification, the algorithm achieves an approximation guarantee of $3 / 2$

Algorithm 3.10 (Metric TSP – factor $3 / 2 )$

1. Find an MST of G, say T.

2. Compute a minimum cost perfect matching, M, on the set of odd-degree vertices of T. Add M to $T$ and obtain an Eulerian graph.

3. Find an Euler tour, $\tau ,$ of this graph.

4. Output the tour that visits vertices of $G$ in order of their first appearance in $\tau$ . Let C be this tour.

Interestingly, the proof of this algorithm is based on a second lower bound on OPT.

Lemma 3.11 Let $V ^ { \prime } \subseteq V$ , such that $| V ^ { \prime } |$ is even, and let M be a minimum cost perfect matching on $V ^ { \prime }$ . Then, cost $( M ) \leq \mathrm { O P T / 2 }$

Proof: Consider an optimal TSP tour of $G ,$ , say τ . Let $\tau ^ { \prime }$ be the tour on $V ^ { \prime }$ obtained by short-cutting τ . By the triangle inequality, cost $\left( \tau ^ { \prime } \right) \ \leq$ cos ${ \mathrm { ; } } ( \tau )$ . Now, $\tau ^ { \prime }$ is the union of two perfect matchings on $V ^ { \prime }$ , each consisting of alternate edges of $\tau .$ . Thus, the cheaper of these matchings has cost $\leq$ cost $( \tau ^ { \prime } ) / 2 \le \mathrm { O P T / 2 }$ . Hence the optimal matching also has cost at most OPT/2. ✷

Theorem 3.12 Algorithm 3.10 achieves an approximation guarantee $o f 3 / 2$ for metric TSP.

Proof: The cost of the Euler tour,

$$
\operatorname{cost} (\mathcal {T}) \leq \operatorname{cost} (T) + \operatorname{cost} (M) \leq \mathrm{OPT} + \frac {1}{2} \mathrm{OPT} = \frac {3}{2} \mathrm{OPT},
$$

where the first inequality follows by using the two lower bounds on OPT. Using the triangle inequality, cost $( { \mathcal { C } } ) \leq \mathrm { c o s t } ( { \mathcal { T } } )$ , and the theorem follows. ✷

Example 3.13 A tight example for this algorithm is given by the following graph on n vertices, with n odd:

![](images/ac07c3a115b814b4f129dfde4daae88d1633ac86f9cfe2d6cf6b6c979a445597.jpg)  
Thick edges represent the MST found in step 1. This MST has only two odd vertices, and by adding the edge joining them we obtain a traveling salesman tour of cost $\left( n - 1 \right) + \left\lceil n / 2 \right\rceil$ . In contrast, the optimal tour has cost n. ✷

Finding a better approximation algorithm for metric TSP is currently one of the outstanding open problems in this area. Many researchers have conjectured that an approximation factor of $4 / 3$ may be achievable.

## 3.3 Exercises

3.1 The hardness of the Steiner tree problem lies in determining the optimal subset of Steiner vertices that need to be included in the tree. Show this by proving that if this set is provided, then the optimal Steiner tree can be computed in polynomial time.

Hint: Find an MST on the union of this set and the set of required vertices.

3.2 Let $G = ( V , E )$ be a graph with nonnegative edge costs. S, the senders and $R _ { : }$ , the receivers, are disjoint subsets of V. The problem is to find a minimum cost subgraph of $G$ that has a path connecting each receiver to a sender (any sender sufices). Partition the instances into two cases: $S \cup R = V$ and $S \cup R \neq V$ . Show that these two cases are in P and NP-hard, respectively. For the second case, give a factor 2 approximation algorithm.

Hint: Add a new vertex which is connected to each sender by a zero cost edge. Consider the new vertex and all receivers as required and the remaining vertices as Steiner, and find a minimum cost Steiner tree.

3.3 Give an approximation factor preserving reduction from the set cover problem to the following problem, thereby showing that it is unlikely to have a better approximation guarantee than $O ( \log n )$

Problem 3.14 (Directed Steiner tree) $G = ( V , E )$ is a directed graph with nonnegative edge costs. The vertex set V is partitioned into two sets, required and Steiner. One of the required vertices, r, is special. The problem is to find a minimum cost tree in G rooted into r that contains all the required vertices and any subset of the Steiner vertices.

Hint: Construct a three layer graph: layer 1 contains a required vertex corresponding to each element, layer 2 contains a Steiner vertex corresponding to each set, and layer 3 contains r.

3.4 (Hoogeveen [130]) Consider variants on the metric TSP problem in which the object is to find a simple path containing all the vertices of the graph. Three diferent problems arise, depending on the number $( 0 , 1 , \mathrm { o r } 2 )$ of endpoints of the path that are specified. Obtain the following approximation algorithms.

• If zero or one endpoints are specified, obtain a $3 / 2$ factor algorithm.

• If both endpoints are specified, obtain a $5 / 3$ factor algorithm.

Hint: Use the idea behind Algorithm 3.10.

3.5 (Papadimitriou and Yannakakis [219]) Let G be a complete undirected graph in which all edge lengths are either 1 or 2 (clearly, G satisfies the triangle inequality). Give a $4 / 3$ factor algorithm for TSP in this special class of graphs.

Hint: Start by finding a minimum 2-matching in G. A 2-matching is a subset S of edges so that every vertex has exactly 2 edges of S incident at it.

3.6 (Frieze, Galbiati, and Mafioli [89]) Give an $O ( \log n )$ factor approximation algorithm for the following problem.

Problem 3.15 (Asymmetric TSP) We are given a directed graph G on vertex set $V _ { : }$ with a nonnegative cost specified for edge $( u  v )$ , for each pair $u , v \in V$ . The edge costs satisfy the directed triangle inequality, i.e., for any three vertices u, v, and w, cost $( u \to v ) \leq \mathrm { c o s t } ( u \to w ) + \mathrm { c o s t } ( w \to v )$ . The problem is to find a minimum cost cycle visiting every vertex exactly once.

Hint: Use the fact that a minimum cost cycle cover $( \mathrm { i . e . }$ , disjoint cycles covering all the vertices) can be found in polynomial time. Shrink the cycles and recurse.

3.7 Let $G = ( V , E )$ be a graph with edge costs satisfying the triangle inequality, and $V ^ { \prime } \subseteq V$ be a set of even cardinality. Prove or disprove: The cost of a minimum cost perfect matching on $V ^ { \prime }$ is bounded above by the cost of a minimum cost perfect matching on $V$ .

3.8 Given n points in $\mathbf { R } ^ { 2 }$ , define the optimal Euclidean Steiner tree to be a minimum length tree containing all n points and any other subset of points from $\mathbf { R } ^ { 2 }$ . Prove that each of the additional points must have degree three, with all three angles being 120<sup>◦</sup>.

3.9 (Rao, Sadayappan, Hwang, and Shor [230]) This exercise develops a factor 2 approximation algorithm for the following problem.

Problem 3.16 (Rectilinear Steiner arborescence) Let $p _ { 1 } , \ldots , p _ { n }$ be points given in $\mathbf { R } ^ { 2 }$ in the positive quadrant. A path from the origin to point $p _ { i }$ is said to be monotone if it consists of segments traversing in the positive x direction or the positive y direction (informally, going right or up). The problem is to find a minimum length tree containing monotone paths from the origin to each of the n points; such a tree is called rectilinear Steiner arborescence.

For point $p ,$ define $x _ { p }$ and $y _ { p }$ to be its x and y coordinates, and $| p | _ { 1 } =$ $| x _ { p } | + | y _ { p } |$ . Say that point $p$ dominates point q if $x _ { p } \leq x _ { q }$ and $y _ { p } \le y _ { q }$ . For sets of points $A$ and $B ,$ we will say that A dominates B if for each point $b \in B ,$ there is a point $a \in A$ such that a dominates b. For points $p$ and q, define dom $\displaystyle ( p , q ) = ( x , y )$ , where $x = \operatorname* { m i n } ( x _ { p } , x _ { q } )$ and $y = \operatorname* { m i n } ( y _ { p } , y _ { q } )$ If $p$ dominates $q ,$ define segments $( p , q )$ to be a monotone path from p to $q .$ Consider the following algorithm.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 3.17 (Rectilinear Steiner arborescence)
1.  $T \leftarrow \emptyset$ .
2.  $P \leftarrow \{p_{1}, \ldots, p_{n}\} \cup \{(0,0)\}$ .
3. while  $|P| &gt; 1$  do:
    Pick  $p, q = \arg\max_{p, q \in P} (|\text{dom}(p, q)|_{1})$ .
    $P \leftarrow (P - \{p, q\}) \cup \{\text{dom}(p, q)\}$ .
    $T \leftarrow T \cup \text{segments}(\text{dom}(p, q), p) \cup \text{segments}(\text{dom}(p, q), q)$ .
4. Output T.
</div>

For $z \geq 0$ , define $\ell _ { z }$ to be the line $x + y = z$ . For a rectilinear Steiner arborescence $T _ { \cdot }$ , let $T ( z ) = | T \cap \ell _ { z } |$ . Prove that the length of $T$ is

$$
\int_ {z = 0} ^ {\infty} T (z) \mathrm{d} z.
$$

Also, for every $x \geq 0$ define $P _ { z } = \{ p \in P \ s . t . \ | p | _ { 1 } > z \}$ , and

$N ( z ) = \operatorname* { m i n } \{ | C | : C \subset \ell _ { z }$ and C dominates $P _ { z } \}$

Prove that

$$
\int_ {z = 0} ^ {\infty} N (z) \mathrm{d} z
$$

is a lower bound on $\mathrm { O P T }$

Use these facts to show that Algorithm 3.17 achieves an approximation guarantee of 2.

3.10 (I. M˘andoiu) This exercise develops a factor 9 approximation algorithm for the following problem, which finds applications in VLSI clock routing.

Problem 3.18 (Rectilinear zero-skew tree) Given a set S of points in the rectilinear plane, find a minimum length zero-skew tree (ZST) for S, i.e., a rooted tree $T$ embedded in the rectilinear plane such that points in $S$ are leaves of $T$ and all root-to-leaf paths in T have equal length. By length of a path we mean the sum of the lengths of edges on it.

1. Let $T$ be an arbitrary zero-skew tree, and let $R ^ { \prime }$ denote the common length of all root-to-leaf paths. For $r \geq 0$ , let $T ( r )$ denote the number of points of $T$ that are at a length of $R ^ { \prime } - r$ from the root. Prove that the length of $T$ is

$$
\int_ {0} ^ {R ^ {\prime}} T (r) \mathrm{d} r
$$

2. A closed $\ell _ { 1 }$ ball of radius r centered at point $p$ is the set of all points whose $\ell _ { 1 } { \mathrm { - d i s t a n c e } }$ from $p { \mathrm { ~ i s ~ } } \leq r$ . Let R denote the radius of the smallest $\ell _ { 1 } \cdot$ -ball that contains all points of $S .$ For $r \geq 0$ , let $N ( r )$ denote the minimum number of closed <sub>1</sub>-balls of radius r needed to cover all points of S. Prove that

$$
\int_ {0} ^ {R} N (r) \mathrm{d} r
$$

is a lower bound on the length of the optimum ZST.

3. Consider the following algorithm. First, compute R and find a radius R $\ell _ { 1 }$ -ball enclosing all points of S. The center of this ball is chosen as the root of the resulting ZST. This ball can be partitioned into 4 balls, called its quadrants, of radius $R / 2$ each. The root can be connected to the center of any of these balls by an edge of length $R / 2$ . These balls can be further partitioned into 4 balls each of radius $R / 4 ,$ , and so on.

The ZST is constructed recursively, starting with the ball of radius R. The center of the current ball is connected to the centers of each of its quadrants that has a point of S. The algorithm then recurses on each of these quadrants. If the current ball contains exactly one point of $S ,$ then this ball is not partitioned into quadrants. Let $r ^ { \prime }$ be the radius of this ball, c its center, and $p \in S$ the point in it. Clearly, the $\ell _ { 1 }$ distance between c and $p$ is $\leq r ^ { \prime } .$ . Connect c to $p$ by a rectilinear path of length exactly $r ^ { \prime } .$

Show that for $0 \leq r \leq R , T ( r ) \leq 9 N ( r )$ . Hence, show that this is a factor 9 approximation algorithm.

## 3.4 Notes

The Steiner tree problem has its origins in a problem posed by Fermat, and was defined by Gauss in a letter he wrote to his student Schumacher on March 21, 1836. Parts of the letter are reproduced on the cover of this book. Courant and Robbins [55] popularized this problem under the name of Steiner, a well known 19th century geometer. See Hwang, Richards, and Winter [133] and Schreiber [236] for the fascinating history of this problem.

The factor 2 Steiner tree algorithm was discovered independently by Choukhmane [44], Iwainsky, Canuto, Taraszow, and Villa [136], Kou, Markowsky, and Berman [177], and Plesn´ık [221]. The factor $3 / 2$ metric TSP algorithm is due to Christofides [45], and Theorem 3.6 is due to Sahni and Gonzalez [232]. The lower bound in Exercise 3.10 is from Charikar, Kleinberg, Kumar, Rajagopalan, Sahai, and Tomkins [41]. The best factor known for the rectilinear zero-skew tree problem, due to Zelikovsky and M˘andoiu [263], is 3.

Given n points on the Euclidean plane, the minimum spanning tree on these points is within a factor of $2 / \sqrt { 3 }$ of the minimum Steiner tree (which is allowed to use any set of points on the plane as Steiner points). This was shown by Du and Hwang [63], thereby settling the conjecture of Gilbert and Pollak [100].