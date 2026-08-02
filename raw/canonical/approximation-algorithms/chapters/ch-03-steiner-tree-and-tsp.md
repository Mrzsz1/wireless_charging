---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-03"
chapter_number: 3
chapter_title: "Steiner Tree and TSP"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 45
source_page_end: 55
printed_page_start: 27
printed_page_end: 37
part_ids: ["approximation-algorithms-ch-03-part-004"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Steiner Tree and TSP

3 Steiner Tree and TSP




In this chapter, we will present constant factor algorithms for two fundamen-
tal problems, metric Steiner tree and metric TSP. The reasons for considering
the metric case of these problems are quite diﬀerent. For Steiner tree, this is
the core of the problem – the rest of the problem reduces to this case. For
TSP, without this restriction, the problem admits no approximation factor,
assuming P = NP. The algorithms, and their analyses, are similar in spirit,
which is the reason for presenting these problems together.


3.1 Metric Steiner tree

The Steiner tree problem was deﬁned by Gauss in a letter he wrote to Schu-
macher (reproduced on the cover of this book). Today, this problem occupies
a central place in the ﬁeld of approximation algorithms. The problem has a
wide range of applications, all the way from ﬁnding minimum length inter-
connection of terminals in VLSI design to constructing phylogeny trees in
computational biology. This problem and its generalizations will be studied
extensively in this book, see Chapters 22 and 23.
Problem 3.1 (Steiner tree) Given an undirected graph G = (V, E) with
nonnegative edge costs and whose vertices are partitioned into two sets, re-
quired and Steiner, ﬁnd a minimum cost tree in G that contains all the re-
quired vertices and any subset of the Steiner vertices.
    We will ﬁrst show that the core of this problem lies in its restriction to
instances in which the edge costs satisfy the triangle inequality, i.e., G is a
complete undirected graph, and for any three vertices u, v, and w, cost(u, v) ≤
cost(u, w) + cost(v, w). Let us call this restriction the metric Steiner tree
problem.

Theorem 3.2 There is an approximation factor preserving reduction from
the Steiner tree problem to the metric Steiner tree problem.

Proof: We will transform, in polynomial time, an instance I of the Steiner
tree problem, consisting of graph G = (V, E), to an instance I  of the metric
Steiner tree problem as follows. Let G be the complete undirected graph on
28      3   Steiner Tree and TSP

vertex set V . Deﬁne the cost of edge (u, v) in G to be the cost of a shortest
u–v path in G. G is called the metric closure of G. The partition of V into
required and Steiner vertices in I  is the same as in I.
    For any edge (u, v) ∈ E, its cost in G is no more than its cost in G.
Therefore, the cost of an optimal solution in I  does not exceed the cost of
an optimal solution in I.
    Next, given a Steiner tree T  in I  , we will show how to obtain, in poly-
nomial time, a Steiner tree T in I of at most the same cost. The cost of an
edge (u, v) in G corresponds to the cost of a path in G. Replace each edge
of T  by the corresponding path to obtain a subgraph of G. Clearly, in this
subgraph, all the required vertices are connected. However, this subgraph
may, in general, contain cycles. If so, remove edges to obtain tree T . This
completes the approximation factor preserving reduction.                      ✷
    As a consequence of Theorem 3.2, any approximation factor established
for the metric Steiner tree problem carries over to the entire Steiner tree
problem.

3.1.1   MST-based algorithm

Let R denote the set of required vertices. Clearly, a minimum spanning tree
(MST) on R is a feasible solution for this problem. Since the problem of
ﬁnding an MST is in P and the metric Steiner tree problem is NP-hard, we
cannot expect the MST on R to always give an optimal Steiner tree; below
is an example in which the MST is strictly costlier.
                                       t
                                     ✡❏
                                        ❏
                                 5 ✡ 3 ❏5
                                 ✡
                                ✡ ✟ ❜❍ ❏
                                 3✟         ❏
                                          ❍3❍
                               ✡
                              t ✟
                              ✡
                              ✟              ❍❏t
                                       5
Even so, an MST on R is not much more costly than an optimal Steiner tree:

Theorem 3.3 The cost of an MST on R is within 2 · OPT.

Proof: Consider a Steiner tree of cost OPT. By doubling its edges we
obtain an Eulerian graph connecting all vertices of R and, possibly, some
Steiner vertices. Find an Euler tour of this graph, for example by traversing
the edges in DFS (depth ﬁrst search) order:
                                              3.1   Metric Steiner tree   29




   The cost of this Euler tour is 2 · OPT. Next obtain a Hamiltonian cycle
on the vertices of R by traversing the Euler tour and “short-cutting” Steiner
vertices and previously visited vertices of R:




   Because of triangle inequality, the shortcuts do not increase the cost of
the tour. If we delete one edge of this Hamiltonian cycle, we obtain a path
that spans R and has cost at most 2 · OPT. This path is also a spanning tree
on R. Hence, the MST on R has cost at most 2 · OPT.                       ✷
    Theorem 3.3 gives a straightforward factor 2 algorithm for the metric
Steiner tree problem: simply ﬁnd an MST on the set of required vertices.
As in the case of set cover, the “correct” way of viewing this algorithm is
in the setting of LP-duality theory. In Chapters 22 and 23 we will see that
LP-duality provides the lower bound on which this algorithm is based and
also helps solve generalizations of this problem.
Example 3.4 For a tight example, consider a graph with n required vertices
and one Steiner vertex. An edge between the Steiner vertex and a required
vertex has cost 1, and an edge between two required vertices has cost 2 (not
all edges of cost 2 are shown below). In this graph, any MST on R has cost
2(n − 1), while OPT = n.
30     3   Steiner Tree and TSP


                                                ✭✭    s
                                          s✭
                                          ✭   ✟ ✟❜
                                                       ❜
                                          ❆❆
                                         ✁✟ ✟      
                                     ✟                ❜s
                                                          
                                       ✁
                                 ✟
                                 s❍          ❆          ✱❇
                                ✆❉ ❍✁
                                       ❍       ❆  ✱✱  ❇
                               ✆ ❉✁        ❍ ❆ ✱
                                              ❍             ❇❇s
                                ✁      ✥   ✥✥ ❝
                           ✆s✁✆✥❉✥             ✪❡
                               ❧❉
                               ❚            ✪         ❡
                                  ❚❧❉ ✪
                                       ❧                ❡ ...
                                   ❚❉❉❛
                                      s✪❛❧              ✦❡s
                                                         ✦
                                           ❛❧  ❛s✦✦

                                                                              ✷



3.2 Metric TSP
The following is a well-studied problem in combinatorial optimization.
Problem 3.5 (Traveling salesman problem (TSP)) Given a complete
graph with nonnegative edge costs, ﬁnd a minimum cost cycle visiting every
vertex exactly once.
   In its full generality, TSP cannot be approximated, assuming P = NP.
Theorem 3.6 For any polynomial time computable function α(n), TSP can-
not be approximated within a factor of α(n), unless P = NP.

Proof: Assume, for a contradiction, that there is a factor α(n) polynomial
time approximation algorithm, A, for the general TSP problem. We will show
that A can be used for deciding the Hamiltonian cycle problem (which is NP-
hard) in polynomial time, thus implying P = NP.
   The central idea is a reduction from the Hamiltonian cycle problem to
TSP, that transforms a graph G on n vertices to an edge-weighted complete
graph G on n vertices such that
• if G has a Hamiltonian cycle, then the cost of an optimal TSP tour in G
  is n, and
• if G does not have a Hamiltonian cycle, then an optimal TSP tour in G
  is of cost > α(n) · n.
   Observe that when run on graph G , algorithm A must return a solution of
cost ≤ α(n) · n in the ﬁrst case, and a solution of cost > α(n) · n in the second
case. Thus, it can be used for deciding whether G contains a Hamiltonian
cycle.
   The reduction is simple. Assign a weight of 1 to edges of G, and a weight
of α(n) · n to nonedges, to obtain G . Now, if G has a Hamiltonian cycle,
then the corresponding tour in G has cost n. On the other hand, if G has
                                                        3.2   Metric TSP      31

no Hamiltonian cycle, any tour in G must use an edge of cost α(n) · n, and
therefore has cost > α(n) · n.                                            ✷
    Notice that in order to obtain such a strong nonapproximability result,
we had to assign edge costs that violate triangle inequality. If we restrict our-
selves to graphs in which edge costs satisfy triangle inequality, i.e., consider
metric TSP, the problem remains NP-complete, but it is no longer hard to
approximate.

3.2.1   A simple factor 2 algorithm
We will ﬁrst present a simple factor 2 algorithm. The lower bound we will
use for obtaining this factor is the cost of an MST in G. This is a lower
bound because deleting any edge from an optimal solution to TSP gives us
a spanning tree of G.

 Algorithm 3.7 (Metric TSP – factor 2)
  1. Find an MST, T , of G.
  2. Double every edge of the MST to obtain an Eulerian graph.
  3. Find an Eulerian tour, T , on this graph.
  4. Output the tour that visits vertices of G in the order of their ﬁrst
     appearance in T . Let C be this tour.


Notice that Step 4 is similar to the “short-cutting” step in Theorem 3.3.
Theorem 3.8 Algorithm 3.7 is a factor 2 approximation algorithm for met-
ric TSP.

Proof: As noted above, cost(T ) ≤ OPT. Since T contains each edge of T
twice, cost(T ) = 2 · cost(T ). Because of triangle inequality, after the “short-
cutting” step, cost(C) ≤ cost(T ). Combining these inequalities we get that
cost(C) ≤ 2 · OPT.                                                             ✷

Example 3.9 A tight example for this algorithm is given by a complete
graph on n vertices with edges of cost 1 and 2. We present the graph for
n = 6 below, where thick edges have cost 1 and remaining edges have cost 2.
For arbitrary n the graph has 2n − 2 edges of cost 1, with these edges forming
the union of a star and an n − 1 cycle; all remaining edges have cost 2. The
optimal TSP tour has cost n, as shown below for n = 6:
                                 r                    r
                               ✑✁◗
                                 ❆◗                  ✑◗◗
                          ✑          ◗             ✑     ◗
                     ✑✑
                     rP        ✁   ❆    ◗  r r
                                             ✑✑             ◗
                     P
                     ❧    P   ✁     ❆
                                    ✏   ✏
                                        ✏                ✏ ✏✏r
                     ❇         P✏r       ✂   ❇        r
                                                      ✏
                       ❇❧   ❧ ❙ ❉ ✂          ❇      ❙
                        ❇  ✟  ❜✟❙ ❉ ✂
                                  ❜              ❇      ❙
                           r
                         ❇✟
                                   ❜ ❉
                                     ❙✂r          ❇r      ❙r
32       3   Steiner Tree and TSP

Suppose that the MST found by the algorithm is the spanning star created
by edges of cost 1. Moreover, suppose that the Euler tour constructed in Step
3 visits vertices in order shown below for n = 6:
                               r               3r
                                                ✂❇       6
                       rP
                       P           ✏✏r 5r      ✂ ❇
                                                     ✦✦r
                          PPr✏✏           ✂ r✦❇  ✦
                              ❙            ✂ 1 ❇
                                               
                                               
                                ❙           ✂  ❇
                          r      ❙r        r
                                            ✂2     ❇r 4

Then the tour obtained after short-cutting contains n − 2 edges of cost 2 and
has a total cost of 2n − 2. Asymptotically, this is twice the cost of the optimal
TSP tour.                                                                      ✷


3.2.2     Improving the factor to 3/2

Algorithm 3.7 ﬁrst ﬁnds a low cost Euler tour spanning the vertices of G, and
then short-cuts this tour to ﬁnd a traveling salesman tour. Is there a cheaper
Euler tour than that found by doubling an MST? Recall that a graph has
an Euler tour iﬀ all its vertices have even degrees. Thus, we only need to be
concerned about the vertices of odd degree in the MST. Let V  denote this
set of vertices. |V  | must be even since the sum of degrees of all vertices in the
MST is even. Now, if we add to the MST a minimum cost perfect matching
on V  , every vertex will have an even degree, and we get an Eulerian graph.
With this modiﬁcation, the algorithm achieves an approximation guarantee
of 3/2.


 Algorithm 3.10 (Metric TSP – factor 3/2)
     1. Find an MST of G, say T .
     2. Compute a minimum cost perfect matching, M , on the set of
        odd-degree vertices of T . Add M to T and obtain an Eulerian graph.
     3. Find an Euler tour, T , of this graph.
     4. Output the tour that visits vertices of G in order of their ﬁrst
        appearance in T . Let C be this tour.


   Interestingly, the proof of this algorithm is based on a second lower bound
on OPT.
Lemma 3.11 Let V  ⊆ V , such that |V  | is even, and let M be a minimum
cost perfect matching on V  . Then, cost(M ) ≤ OPT/2.

Proof: Consider an optimal TSP tour of G, say τ . Let τ  be the tour
on V  obtained by short-cutting τ . By the triangle inequality, cost(τ  ) ≤
                                                           3.3    Exercises   33

cost(τ ). Now, τ  is the union of two perfect matchings on V  , each consisting
of alternate edges of τ . Thus, the cheaper of these matchings has cost ≤
cost(τ  )/2 ≤ OPT/2. Hence the optimal matching also has cost at most
OPT/2.                                                                          ✷

Theorem 3.12 Algorithm 3.10 achieves an approximation guarantee of 3/2
for metric TSP.

Proof: The cost of the Euler tour,

                                            1     3
      cost(T ) ≤ cost(T ) + cost(M ) ≤ OPT + OPT = OPT,
                                            2     2
where the ﬁrst inequality follows by using the two lower bounds on OPT.
Using the triangle inequality, cost(C) ≤ cost(T ), and the theorem follows. ✷

Example 3.13 A tight example for this algorithm is given by the following
graph on n vertices, with n odd:
                        1             ...         1
               ❙
                             ❙❙
                                             ❙
                                               ❙                 ❙
                                                                  ❙
             1     ❙1    1                      ❙1    1           ❙1
                    ❙                            ❙                 ❙
                 1    ❙
                       ❙    1        ...              
                                                      ❙         1      ❙
                                     n/2
         ✫                                                                 ✪
Thick edges represent the MST found in step 1. This MST has only two odd
vertices, and by adding the edge joining them we obtain a traveling salesman
tour of cost (n − 1) + n/2. In contrast, the optimal tour has cost n.    ✷
   Finding a better approximation algorithm for metric TSP is currently
one of the outstanding open problems in this area. Many researchers have
conjectured that an approximation factor of 4/3 may be achievable.


3.3 Exercises

3.1 The hardness of the Steiner tree problem lies in determining the optimal
subset of Steiner vertices that need to be included in the tree. Show this
by proving that if this set is provided, then the optimal Steiner tree can be
computed in polynomial time.
Hint: Find an MST on the union of this set and the set of required vertices.

3.2 Let G = (V, E) be a graph with nonnegative edge costs. S, the senders
and R, the receivers, are disjoint subsets of V . The problem is to ﬁnd a
minimum cost subgraph of G that has a path connecting each receiver to a
34     3   Steiner Tree and TSP

sender (any sender suﬃces). Partition the instances into two cases: S ∪R = V
and S∪R = V . Show that these two cases are in P and NP-hard, respectively.
For the second case, give a factor 2 approximation algorithm.
Hint: Add a new vertex which is connected to each sender by a zero cost
edge. Consider the new vertex and all receivers as required and the remaining
vertices as Steiner, and ﬁnd a minimum cost Steiner tree.

3.3 Give an approximation factor preserving reduction from the set cover
problem to the following problem, thereby showing that it is unlikely to have
a better approximation guarantee than O(log n).
Problem 3.14 (Directed Steiner tree) G = (V, E) is a directed graph
with nonnegative edge costs. The vertex set V is partitioned into two sets,
required and Steiner. One of the required vertices, r, is special. The problem
is to ﬁnd a minimum cost tree in G rooted into r that contains all the required
vertices and any subset of the Steiner vertices.
Hint: Construct a three layer graph: layer 1 contains a required vertex
corresponding to each element, layer 2 contains a Steiner vertex corresponding
to each set, and layer 3 contains r.

3.4 (Hoogeveen [130]) Consider variants on the metric TSP problem in
which the object is to ﬁnd a simple path containing all the vertices of the
graph. Three diﬀerent problems arise, depending on the number (0, 1, or 2) of
endpoints of the path that are speciﬁed. Obtain the following approximation
algorithms.
• If zero or one endpoints are speciﬁed, obtain a 3/2 factor algorithm.
• If both endpoints are speciﬁed, obtain a 5/3 factor algorithm.
Hint: Use the idea behind Algorithm 3.10.

3.5 (Papadimitriou and Yannakakis [219]) Let G be a complete undirected
graph in which all edge lengths are either 1 or 2 (clearly, G satisﬁes the
triangle inequality). Give a 4/3 factor algorithm for TSP in this special class
of graphs.
Hint: Start by ﬁnding a minimum 2-matching in G. A 2-matching is a subset
S of edges so that every vertex has exactly 2 edges of S incident at it.

3.6 (Frieze, Galbiati, and Maﬃoli [89]) Give an O(log n) factor approxima-
tion algorithm for the following problem.
Problem 3.15 (Asymmetric TSP) We are given a directed graph G on
vertex set V , with a nonnegative cost speciﬁed for edge (u → v), for each pair
u, v ∈ V . The edge costs satisfy the directed triangle inequality, i.e., for any
three vertices u, v, and w, cost(u → v) ≤ cost(u → w) + cost(w → v). The
problem is to ﬁnd a minimum cost cycle visiting every vertex exactly once.
                                                         3.3   Exercises    35

Hint: Use the fact that a minimum cost cycle cover (i.e., disjoint cycles
covering all the vertices) can be found in polynomial time. Shrink the cycles
and recurse.

3.7 Let G = (V, E) be a graph with edge costs satisfying the triangle in-
equality, and V  ⊆ V be a set of even cardinality. Prove or disprove: The cost
of a minimum cost perfect matching on V  is bounded above by the cost of
a minimum cost perfect matching on V .

3.8 Given n points in R2 , deﬁne the optimal Euclidean Steiner tree to be a
minimum length tree containing all n points and any other subset of points
from R2 . Prove that each of the additional points must have degree three,
with all three angles being 120◦ .

3.9 (Rao, Sadayappan, Hwang, and Shor [230]) This exercise develops a
factor 2 approximation algorithm for the following problem.
Problem 3.16 (Rectilinear Steiner arborescence) Let p1 , . . . , pn be
points given in R2 in the positive quadrant. A path from the origin to point
pi is said to be monotone if it consists of segments traversing in the positive
x direction or the positive y direction (informally, going right or up). The
problem is to ﬁnd a minimum length tree containing monotone paths from
the origin to each of the n points; such a tree is called rectilinear Steiner
arborescence.
     For point p, deﬁne xp and yp to be its x and y coordinates, and |p|1 =
|xp | + |yp |. Say that point p dominates point q if xp ≤ xq and yp ≤ yq . For
sets of points A and B, we will say that A dominates B if for each point
b ∈ B, there is a point a ∈ A such that a dominates b. For points p and
q, deﬁne dom(p, q) = (x, y), where x = min(xp , xq ) and y = min(yp , yq ).
If p dominates q, deﬁne segments(p, q) to be a monotone path from p to q.
Consider the following algorithm.


 Algorithm 3.17 (Rectilinear Steiner arborescence)
  1. T ← ∅.
  2. P ← {p1 , . . . , pn } ∪ {(0, 0)}.
  3. while |P | > 1 do:
      Pick p, q = arg maxp,q∈P (|dom(p, q)|1 ).
      P ← (P − {p, q}) ∪ {dom(p, q)}.
      T ← T ∪ segments(dom(p, q), p) ∪ segments(dom(p, q), q).
  4. Output T .
36      3    Steiner Tree and TSP

   For z ≥ 0, deﬁne z to be the line x + y = z. For a rectilinear Steiner
arborescence T , let T (z) = |T ∩ z |. Prove that the length of T is
        ∞
             T (z) dz.
       z=0

Also, for every x ≥ 0 deﬁne Pz = {p ∈ P s.t. |p|1 > z}, and

      N (z) = min{|C| : C ⊂ z and C dominates Pz }.

Prove that
        ∞
             N (z) dz
       z=0

is a lower bound on OPT.
    Use these facts to show that Algorithm 3.17 achieves an approximation
guarantee of 2.

3.10 (I. Măndoiu) This exercise develops a factor 9 approximation algorithm
for the following problem, which ﬁnds applications in VLSI clock routing.
Problem 3.18 (Rectilinear zero-skew tree) Given a set S of points in
the rectilinear plane, ﬁnd a minimum length zero-skew tree (ZST) for S, i.e.,
a rooted tree T embedded in the rectilinear plane such that points in S are
leaves of T and all root-to-leaf paths in T have equal length. By length of a
path we mean the sum of the lengths of edges on it.

1. Let T be an arbitrary zero-skew tree, and let R denote the common
   length of all root-to-leaf paths. For r ≥ 0, let T (r) denote the number of
   points of T that are at a length of R − r from the root. Prove that the
   length of T is

                 R
                      T (r)dr
             0

2. A closed 1 ball of radius r centered at point p is the set of all points
   whose 1 -distance from p is ≤ r. Let R denote the radius of the smallest
   1 -ball that contains all points of S. For r ≥ 0, let N (r) denote the
   minimum number of closed 1 -balls of radius r needed to cover all points
   of S. Prove that
                 R
                     N (r)dr
             0

     is a lower bound on the length of the optimum ZST.
                                                             3.4   Notes     37

 3. Consider the following algorithm. First, compute R and ﬁnd a radius R
    1 -ball enclosing all points of S. The center of this ball is chosen as the
    root of the resulting ZST. This ball can be partitioned into 4 balls, called
    its quadrants, of radius R/2 each. The root can be connected to the center
    of any of these balls by an edge of length R/2. These balls can be further
    partitioned into 4 balls each of radius R/4, and so on.
    The ZST is constructed recursively, starting with the ball of radius R.
    The center of the current ball is connected to the centers of each of its
    quadrants that has a point of S. The algorithm then recurses on each
    of these quadrants. If the current ball contains exactly one point of S,
    then this ball is not partitioned into quadrants. Let r be the radius of
    this ball, c its center, and p ∈ S the point in it. Clearly, the 1 distance
    between c and p is ≤ r . Connect c to p by a rectilinear path of length
    exactly r .
    Show that for 0 ≤ r ≤ R, T (r) ≤ 9N (r). Hence, show that this is a factor
    9 approximation algorithm.


3.4 Notes
The Steiner tree problem has its origins in a problem posed by Fermat, and
was deﬁned by Gauss in a letter he wrote to his student Schumacher on March
21, 1836. Parts of the letter are reproduced on the cover of this book. Courant
and Robbins [55] popularized this problem under the name of Steiner, a well
known 19th century geometer. See Hwang, Richards, and Winter [133] and
Schreiber [236] for the fascinating history of this problem.
    The factor 2 Steiner tree algorithm was discovered independently by
Choukhmane [44], Iwainsky, Canuto, Taraszow, and Villa [136], Kou, Mar-
kowsky, and Berman [177], and Plesnı́k [221]. The factor 3/2 metric TSP
algorithm is due to Christoﬁdes [45], and Theorem 3.6 is due to Sahni and
Gonzalez [232]. The lower bound in Exercise 3.10 is from Charikar, Klein-
berg, Kumar, Rajagopalan, Sahai, and Tomkins [41]. The best factor known
for the rectilinear zero-skew tree problem, due to Zelikovsky and Măndoiu
[263], is 3.
    Given n points on the Euclidean√plane, the minimum spanning tree on
these points is within a factor of 2/ 3 of the minimum Steiner tree (which
is allowed to use any set of points on the plane as Steiner points). This was
shown by Du and Hwang [63], thereby settling the conjecture of Gilbert and
Pollak [100].
