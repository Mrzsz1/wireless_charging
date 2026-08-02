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

# Multicut and Integer Multicommodity Flow in Trees

18 Multicut and
Integer Multicommodity Flow in Trees




The theory of cuts in graphs occupies a central place not only in the study of
exact algorithms, but also approximation algorithms. We will present some
key results in the next four chapters. This will also give us the opportunity to
develop further the two fundamental algorithm design techniques introduced
in Chapters 14 and 15.
    In Chapter 15 we used the primal–dual schema to derive a factor 2 algo-
rithm for the weighted vertex cover problem. This algorithm was particularly
easy to obtain because the relaxed dual complementary slackness conditions
were automatically satisﬁed in any integral solution. In this chapter, we will
use the primal–dual schema to obtain an algorithm for a generalization of this
problem (see Exercise 18.1). This time, enforcing relaxed dual complementary
slackness conditions will be a nontrivial part of the algorithm. Furthermore,
we will introduce the procedure of reverse delete, which will be used in several
other primal–dual algorithms.


18.1 The problems and their LP-relaxations
The following is an important generalization of the minimum s–t cut problem.
In fact, it also generalizes the multiway cut problem (Problem 4.1).
Problem 18.1 (Minimum multicut) Let G=(V, E) be an undirected graph
with nonnegative capacity ce for each edge e ∈ E. Let {(s1 , t1 ), . . . , (sk , tk )}
be a speciﬁed set of pairs of vertices, where each pair is distinct, but vertices
in diﬀerent pairs are not required to be distinct. A multicut is a set of edges
whose removal separates each of the pairs. The problem is to ﬁnd a minimum
capacity multicut in G.
    The minimum s–t cut problem is the special case of multicut for k = 1.
Problem 18.1 generalizes multiway cut because separating terminals s1 , . . . , sl
is equivalent to separating all pairs (si , sj ), for 1 ≤ i < j ≤ l. This observation
implies that the minimum multicut problem is NP-hard even for k = 3, since
the multiway cut problem is NP-hard for the case of 3 terminals.
    In Chapter 20 we will obtain an O(log k) factor approximation algorithm
for the minimum multicut problem. In this chapter, we will obtain a factor
2 algorithm for the special case when G is restricted to be a tree. Since G is
                                 18.1      The problems and their LP-relaxations     147

a tree, there is a unique path between si and ti , and the multicut must pick
an edge on this path to disconnect si from ti . Although the problem looks
deceptively simple, Exercise 18.1 should convince the reader that this is not
so. The minimum multicut problem is NP-hard even if restricted to trees of
height 1 and unit capacity edges.
    Since we want to apply LP-duality theory to design the algorithm, let us
ﬁrst give an integer programming formulation for the problem and obtain its
LP-relaxation. Introduce a 0/1 variable de for each edge e ∈ E, which will be
set to 1 iﬀ e is picked in the multicut. Let pi denote the unique path between
si and ti in the tree.
                    
      minimize            ce de
                   e∈E
                    
      subject to          de ≥ 1,          i ∈ {1, . . . , k}
                   e∈pi
                   de ∈ {0, 1},            e∈E

The LP-relaxation is obtained by replacing the constraint de ∈ {0, 1} by
de ≥ 0. As in the derivation of LP (13.2), there is no need to add the constraint
de ≤ 1 explicitly.
                    
      minimize            ce de                                                    (18.1)
                   e∈E
                    
      subject to          de ≥ 1,          i ∈ {1, . . . , k}
                   e∈pi
                   de ≥ 0,                 e∈E

We can now think of de as specifying the fractional extent to which edge e is
picked. A solution to this linear program is a fractional multicut: on each path
pi , the sum of fractions of edges picked is at least 1. In general, minimum
fractional multicut may be strictly cheaper than minimum integral multicut.
This is illustrated in Example 18.2.
     We will interpret the dual program as specifying a multicommodity ﬂow
in G, with a separate commodity corresponding to each vertex pair (si , ti ).
Dual variable fi will denote the amount of this commodity routed along the
unique path from si to ti .

                   k
                   
      maximize            fi                                                       (18.2)
                    i=1
                     
      subject to               fi ≤ ce ,      e∈E
                   i: e∈pi
                   fi ≥ 0,                    i ∈ {1, . . . , k}
148      18   Multicut and Integer Multicommodity Flow in Trees

The commodities are routed concurrently. The object is to maximize the sum
of the commodities routed, subject to the constraint that the sum of ﬂows
routed through an edge is bounded by the capacity of the edge. Notice that
the sum of ﬂows through an edge (u, v) includes ﬂow going in either direction,
u to v and v to u.
    By the weak duality theorem, a feasible multicommodity ﬂow gives a lower
bound on the minimum fractional multicut and hence also on the minimum
integral multicut. By the LP-duality theorem, minimum fractional multicut
equals maximum multicommodity ﬂow.
Example 18.2 Consider the following graph with unit capacity edges and
3 vertex pairs:

                                       t1 ,s2




                                1/2             1/2




                                       1/2
                    s1 ,t3                                t2 ,s3


    The arrows show how to send 3/2 units of ﬂow by sending 1/2 unit of
each commodity. Picking each edge to the extent of 1/2 gives a multicut of
capacity 3/2 as well. These must be optimal solutions to the primal and dual
programs. On the other hand, any integral multicut must pick at least two
of the three edges in order to disconnect all three pairs. Hence, minimum
integral multicut has capacity 2.                                         ✷
      Finally, let us state one more problem.
Problem 18.3 (Integer multicommodity ﬂow) Graph G and the source–
sink pairs are speciﬁed as in the minimum multicut problem; however, the
edge capacities are all integral. A separate commodity is deﬁned for each
(si , ti ) pair. The object is to maximize the sum of the commodities routed,
subject to edge capacity constraints and subject to routing each commodity
integrally.
     Let us consider this problem when G is restricted to be a tree. If in (18.2),
the variables are constrained to be nonnegative integers, we would get an inte-
ger programming formulation for this problem. Clearly, the objective function
value of this integer program is bounded by that of the linear program (18.2).
                             18.2   Primal–dual schema based algorithm    149

Furthermore, the best fractional ﬂow may be strictly larger. For instance, in
Example 18.2, maximum integral multicommodity ﬂow is 1, since sending
1 unit of any of the three commodities will saturate two of the edges. This
problem is NP-hard, even for trees of height 3 (though the capacity has to
be arbitrary).


18.2 Primal–dual schema based algorithm
We will use the primal–dual schema to obtain an algorithm that simultane-
ously ﬁnds a multicut and an integer multicommodity ﬂow that are within a
factor of 2 of each other, provided the given graph is a tree. Hence, we get
approximation algorithms for both problems, of factor 2 and 1/2, respectively.
    Let us deﬁne the multicut LP to be the primal program. An edge e is
saturated if the total ﬂow through it equals its capacity. We will ensure pri-
mal complementary slackness conditions, i.e., α = 1, and relax the dual
conditions with β = 2, where α and β are the parameters used in the general
description of the primal–dual schema given in Chapter 15.
                                                   
Primal conditions: For each e ∈ E, de = 0 ⇒ i: e∈pi fi = ce .
Equivalently, any edge picked in the multicut must be saturated.
                                                                 
Relaxed dual conditions: For each i ∈ {1, . . . , k}, fi = 0 ⇒ e∈pi de ≤ 2.
Equivalently, at most two edges can be picked from a path carrying nonzero
ﬂow. (Clearly, we must pick at least one edge from each (si , ti ) path simply
to ensure the feasibility of the multicut.)
    Let us root the tree G at an arbitrary vertex. Deﬁne the depth of vertex
v to be the length of the path from v to the root; the depth of the root is 0.
For two vertices u, v ∈ V , let lca(u, v) denote the lowest common ancestor of
u and v, i.e., the minimum depth vertex on the path from u to v. Let e1 and
e2 be two edges on a path from a vertex to the root. If e1 occurs before e2
on this path, then e1 is said to be deeper than e2 .
    The algorithm starts with an empty multicut and ﬂow, and iteratively
improves the feasibility of the primal solution and the optimality of the dual
solution. In an iteration, it picks the deepest unprocessed vertex, say v, and
greedily routes integral ﬂow between pairs that have v as their lowest com-
mon ancestor. When no more ﬂow can be routed between these pairs, all
edges that were saturated in this iteration are added to the list D in arbi-
trary order. When all the vertices have been processed, D will be a multicut;
however, it may have redundant edges. To remove them, a reverse delete step
is performed: edges are considered in the reverse of the order in which they
were added to D, and if the deletion of edge e from D still gives a valid
multicut, e is discarded from D.
150    18   Multicut and Integer Multicommodity Flow in Trees


 Algorithm 18.4 (Multicut and integer multicommodity ﬂow in
 trees)
  1. Initialization: f ← 0; D ← ∅.
  2. Flow routing: For each vertex v, in nonincreasing order of depth, do:
      For each pair (si , ti ) such that lca(si , ti ) = v, greedily route integral
         ﬂow from si to ti .
      Add to D all edges that were saturated in the current iteration in
         arbitrary order.
  3. Let e1 , e2 , . . . , el be the ordered list of edges in D.
  4. Reverse delete: For j = l downto 1 do:
         If D − {ej } is a multicut in G, then D ← D − {ej }.
  5. Output the ﬂow and multicut D.


Lemma 18.5 Let (si , ti ) be a pair with nonzero ﬂow, and let lca(si , ti ) = v.
At most one edge is picked in the multicut from each of the two paths, si to
v and ti to v.

Proof: The argument is the same for each path. Suppose two edges e and
e are picked from the si –v path, with e being the deeper edge. Clearly, e
must be in D all through reverse delete. Consider the moment during reverse
delete when edge e is being tested. Since e is not discarded, there must be
a pair, say (sj , tj ), such that e is the only edge of D on the sj –tj path. Let
u be the lowest common ancestor of sj and tj . Since e does not lie on the
sj –tj path, u must be deeper than e , and hence deeper than v. After u has
been processed, D must contain an edge from the sj –tj path, say e .
                                        vs
                                         ❧
                                           ❧
                                             ❧
                                              ❧
                               e
                                                 ❧
                                                   ❧
                         us                           ❧
                          ❅                             ❧
                  e          ❅                            ❧
                                                            ❧
                                ❅                             ❧
                                  ❅                             ❧
             ❅                                                     ❧
         s    ❅s             s                                        ❧❧s
        si      sj          tj                                          ti
    Since nonzero ﬂow has been routed from si to ti , e must be added during
or after the iteration in which v is processed. Since v is an ancestor of u, e is
added after e . So e must be in D when e is being tested. This contradicts
the fact that at this moment e is the only edge of D on the sj –tj path. ✷
                               18.2     Primal–dual schema based algorithm   151

Theorem 18.6 Algorithm 18.4 achieves approximation guarantees of factor
2 for the minimum multicut problem and factor 1/2 for the maximum integer
multicommodity ﬂow problem on trees.

Proof: The ﬂow found at the end of Step 2 is maximal, and since at this
point D contains all the saturated edges, D is a multicut. Since the reverse
delete step only discards redundant edges, D is a multicut after this step
as well. Thus, feasible solutions have been found for both the ﬂow and the
multicut.
    Since each edge in the multicut is saturated, the primal conditions are
satisﬁed. By Lemma 18.5, at most two edges have been picked in the multicut
from each path carrying nonzero ﬂow. Therefore, the relaxed dual conditions
are also satisﬁed. Hence, by Proposition 15.1, the capacity of the multicut
found is within twice the ﬂow. Since a feasible ﬂow is a lower bound on the
optimal multicut, and a feasible multicut is an upper bound on the optimal
integer multicommodity ﬂow, the claim follows.                            ✷
   Finally, we obtain the following approximate min–max relation from The-
orem 18.6:
Corollary 18.7 On trees with integer edge capacities,

        max       |F | ≤      min       c(C) ≤ 2 ·     max        |F |,
     int. ﬂow F            multicut C                int. ﬂow F

where |F | represents the value of ﬂow function F and c(C) represents the
capacity of multicut C.

    In Chapter 20 we will present an O(log k) factor algorithm for the mini-
mum multicut problem in general graphs; once again, the lower bound used
is an optimal fractional multicut. On the other hand, no nontrivial approx-
imation algorithms are known for the integer multicommodity ﬂow problem
in graphs more general than trees. As shown in Example 18.8, even for planar
graphs, the integrality gap of an LP analogous to (18.2) is lower bounded by
n/2, where n is the number of source–sink pairs speciﬁed.
Example 18.8 Consider the following planar graph with n source–sink pairs.
Every edge is of unit capacity. Any pair of paths between the ith and jth
source–sink pairs intersect in at least one unit capacity edge. The magniﬁed
part shows how this is arranged at each intersection. Thus, sending one unit
of any commodity blocks all other commodities. On the other hand, half a
unit of each commodity can be routed simultaneously.
152    18   Multicut and Integer Multicommodity Flow in Trees
                                               ✬✩
                     s1 r                               ... r
                                                   ..
                                                    .....
                                                  .               ❅❅r
                     s2 r                      ....
                                          ......
                                            .
                                         ..              ✫✪            .....
                                     ....
                                  ...✐                  ...............
                     s3 r                        ......
                                        ........



                  sn−1 r

                    sn r

                              r    r       r               r   r
                             t1   t2      t3             tn−1 tn

                                                                               ✷



18.3 Exercises

18.1 (Garg, Vazirani, and Yannakakis [98]) Give approximation factor pre-
serving reductions between the following pairs of problems:
(a) cardinality vertex cover and minimum multicut in trees of height 1 and
    unit capacity edges,
(b) vertex cover with arbitrary weights and minimum multicut in trees of
    height 1 and arbitrary edge capacities.
Hint: Given a vertex cover instance G, construct a height 1 tree that has a
leaf corresponding to each vertex of G and a source–sink pair corresponding
to each edge of G.

18.2 The following is a well-studied polynomial time solvable generalization
of the maximum matching problem. Given an undirected graph G = (V, E)
and a function b : V → Z+ , a b-matching is a set of edges, E  ⊆ E, with
associated multiplicities, m : E  → Z+ , such that each vertex v ∈ V has
at most b(v) edges incident at it, counting multiplicities. The size of this b-
matching is the sum of multiplicities of edges in E  . The maximum b-matching
problem is that of ﬁnding a b-matching of maximum size. Show that the
following pairs of problems are polynomial time equivalent:
(a) maximum integer multicommodity ﬂow problem on trees of height 1 and
    unit capacity edges, and the maximum matching problem,
(b) maximum integer multicommodity ﬂow problem on trees of height 1 and
    arbitrary capacity edges, and the maximum b-matching problem.
                                                              18.3   Exercises      153

18.3 (Garg, Vazirani, and Yannakakis [98]) Give a polynomial time algorithm
for computing a maximum integer multicommodity ﬂow on unit capacity trees
of arbitrary height.
Hint: Apply dynamic programming, and use a subroutine for the maximum
matching problem.

18.4 If Step 2 of Algorithm 18.4 is modiﬁed to include only one saturated
edge after each iteration, show that the resulting set D may not even be a
multicut.

18.5 If Step 4 in Algorithm 18.4 is removed, or is changed to a forward
delete, show that its approximation factor is unbounded.

18.6 Modify step 4 in Algorithm 18.4 to: sort edges in D by decreasing
capacity and remove redundant edges in this order. What factor can you
prove for the modiﬁed algorithm?

18.7 Give tight examples for Algorithm 18.4 for both multicut and integer
multicommodity ﬂow.

18.8 Prove that if e and e are both in D in Step 3 of Algorithm 18.4, and
e is deeper than e , then e is added before or in the same iteration as e .

18.9 Find the best integral and fractional multicut and the best multicom-
modity ﬂow in the following graph. All capacities are 1, and the speciﬁed
pairs are (s1 , t1 ), . . . , (s5 , t5 ). Notice that the optimal fractional multicut is
not half integral. In contrast, the LP-relaxation of the multiway cut problem
always has a half-integral optimal solution (see Chapter 19).
                s3                                                     s4
           t1                                                               t2




                                 t4                  t3




           s1                                                               s2
                s5                                                     t5
154    18   Multicut and Integer Multicommodity Flow in Trees

18.4 Notes
Algorithm 18.4 is due to Garg, Vazirani, and Yannakakis [98]. For recent re-
sults on the integer multicommodity ﬂow problem, see Guruswami, Khanna,
Rajaraman, Sheperd, and Yannakakis [118].
