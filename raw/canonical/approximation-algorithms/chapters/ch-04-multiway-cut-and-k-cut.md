---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-04"
chapter_number: 4
chapter_title: "Multiway Cut and k-Cut"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 56
source_page_end: 64
printed_page_start: 38
printed_page_end: 46
part_ids: ["approximation-algorithms-ch-04-part-005"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Multiway Cut and k-Cut

4 Multiway Cut and k-Cut




The theory of cuts occupies a central place in the study of exact algorithms.
In this chapter, we will present approximation algorithms for natural gener-
alizations of the minimum cut problem. These generalizations are NP-hard.
    Given a connected, undirected graph G = (V, E) with an assignment of
weights to edges, w : E → R+ , a cut is deﬁned by a partition of V into two
sets, say V  and V − V  , and consists of all edges that have one endpoint in
each partition. Clearly, the removal of the cut from G disconnects G. Given
terminals s, t ∈ V , consider a partition of V that separates s and t. The cut
deﬁned by such a partition will be called an s–t cut. The problems of ﬁnding
a minimum weight cut and a minimum weight s–t cut can be eﬃciently
solved using a maximum ﬂow algorithm. Let us generalize these two notions:
Problem 4.1 (Multiway cut) Given a set of terminals S = {s1 , s2 , . . . , sk }
⊆ V , a multiway cut is a set of edges whose removal disconnects the terminals
from each other. The multiway cut problem asks for the minimum weight such
set.
Problem 4.2 (Minimum k-cut) A set of edges whose removal leaves k
connected components is called a k-cut. The k-cut problem asks for a mini-
mum weight k-cut.
    The problem of ﬁnding a minimum weight multiway cut is NP-hard for
any ﬁxed k ≥ 3. Observe that the case k = 2 is precisely the minimum s–t cut
problem. The minimum k-cut problem is polynomial time solvable for ﬁxed
k; however, it is NP-hard if k is speciﬁed as part of the input. In this chapter,
we will obtain factor 2 − 2/k approximation algorithms for both problems.
In Chapter 19 we will improve the guarantee for the multiway cut problem
to 3/2.


4.1 The multiway cut problem
Deﬁne an isolating cut for si to be a set of edges whose removal disconnects
si from the rest of the terminals.
                                                4.1    The multiway cut problem       39


 Algorithm 4.3 (Multiway cut)
  1. For each i = 1, . . . , k, compute a minimum weight isolating cut for si ,
     say Ci .
  2. Discard the heaviest of these cuts, and output the union of the rest, say
     C.


    Each computation in Step 1 can be accomplished by identifying the ter-
minals in S − {si } into a single node, and ﬁnding a minimum cut separating
this node from si ; this takes one max-ﬂow computation. Clearly, removing C
from the graph disconnects every pair of terminals, and so is a multiway cut.
Theorem 4.4 Algorithm 4.3 achieves an approximation guarantee of 2−2/k.

Proof: Let A be an optimal multiway cut in G. We can view A as the
union of k cuts as follows: The removal of A from G will create k connected
components, each having one terminal (since A is a minimum weight multiway
cut, no more than k components will be created). Let Ai be the cut separating
                                                                      k
the component containing si from the rest of the graph. Then A = i=1 Ai .
    Since each edge of A is incident at two of these components, each edge
will be in two of the cuts Ai . Hence,

      k
      
            w(Ai ) = 2w(A).
      i=1

Clearly, Ai is an isolating cut for si . Since Ci is a minimum weight isolating
cut for si , w(Ci ) ≤ w(Ai ). Notice that this already gives a factor 2 algorithm,
by taking the union of all k cuts Ci . Finally, since C is obtained by discarding
the heaviest of the cuts Ci ,
                          
                            k                         
                                                        k                     
                       1                           1                       1
      w(C) ≤        1−           w(Ci ) ≤       1−          w(Ai ) = 2 1 −         w(A).
                       k   i=1
                                                   k    i=1
                                                                           k

                                                                                       ✷
    Once again, Algorithm 4.3 is not based on a lower bounding scheme. Exer-
cise 19.2 gives an algorithm with the same guarantee using an LP-relaxation
as the lower bound. The use of LP-relaxations is fruitful for this problem
as well. Section 19.1 gives an algorithm with an improved guarantee, using
another LP-relaxation.
Example 4.5 A tight example for this algorithm is given by a graph on
2k vertices consisting of a k-cycle and a distinct terminal attached to each
vertex of the cycle. The edges of the cycle have weight 1 and edges attach-
ing terminals to the cycle have weight 2 − ε for a small fraction ε > 0.
40     4   Multiway Cut and k-Cut

For example, the graph corresponding to k = 4 is:

                         s1                         s2
                          r                          r
                          ❅
                        2 − ε❅                    2−ε
                               ❅
                               ❅r        1   r

                                 1           1
                                     r       r
                                         1   ❅
                        2−ε                    ❅2−ε
                         r                      ❅
                                                ❅r
                        s4                       s3

For each terminal si , the minimum weight isolating cuts for si is given by
the edge incident to si . So, the cut C returned by the algorithm has weight
(k − 1)(2 − ε). On the other hand, the optimal multiway cut is given by the
cycle edges, and has weight k.                                            ✷



4.2 The minimum k-cut problem
A natural algorithm for ﬁnding a k-cut is as follows. Starting with G, compute
a minimum cut in each connected component and remove the lightest one;
repeat until there are k connected components. This algorithm does achieve
a guarantee of 2 − 2/k, however, the proof is quite involved. Instead we will
use the Gomory–Hu tree representation of minimum cuts to give a simpler
algorithm achieving the same guarantee.
    Minimum cuts, as well as sub-optimal cuts, in undirected graphs have
several interesting structural properties, as opposed to cuts in directed graphs
(the algorithm of Section 28.2 is based on exploiting some of these properties).
The existence of Gomory–Hu trees is one of the remarkable consequences of
these properties.
    Let T be a tree on vertex set V ; the edges of T need not be in E. Let e
be an edge in T . Its removal from T creates two connected components. Let
S and S be the vertex sets of these components. The cut deﬁned in graph
G by the partition (S, S) is the cut associated with e in G. Deﬁne a weight
function w on the edges of T . Tree T will be said to be a Gomory–Hu tree
for G if
 1. for each pair of vertices u, v ∈ V , the weight of a minimum u–v cut in G
    is the same as that in T .
 2. for each edge e ∈ T , w (e) is the weight of the cut associated with e in
    G, and
                                               4.2         The minimum k-cut problem   41

    A Gomory–Hu tree encodes, in a succinct manner, a minimum u–v cut in
G, for each pair of vertices u, v ∈ V as follows. A minimum u–v cut in T is
given by a minimum weight edge on the unique path from u to v in T , say e.
By the properties stated above, the cut associated
                                                  with e in G is a minimum
u–v cut, and has weight w (e). So, for the n2 pairs of vertices u, v ∈ V , we
need only n − 1 cuts, those encoded by the edges of a Gomory–Hu tree, to
give minimum u–v cuts in G.
    The following ﬁgure shows a weighted graph and its associated Gomory–
Hu tree. Exercise 4.6 shows how to construct a Gomory–Hu tree for an undi-
rected graph, using only n − 1 max-ﬂow computations.

                                      b                      c
                                               4
                                                                      5
                             10
                                           2           2
                   a                  3                      4        2       d
                         8
                                                                          7
                                                   3
                                      f                          e




                       a 18       b
                                          17 f 13 e              15       c

                                                       14
                                                             d
   We will need the following lemma.
Lemma 4.6 Let S be the union of cuts in G associated with l edges of T .
Then, the removal of S from G leaves a graph with at least l + 1 components.

Proof: Removing the corresponding l edges from T leaves exactly l + 1 con-
nected components, say with vertex sets V1 , V2 , . . . , Vl+1 . Clearly, removing
S from G will disconnect each pair Vi and Vj . Hence we must get at least
l + 1 connected components.                                                      ✷
    As a consequence of Lemma 4.6, the union of k − 1 cuts picked from T
will form a k-cut in G. The complete algorithm is given below.


 Algorithm 4.7 (Minimum k-cut)
  1. Compute a Gomory–Hu tree T for G.
  2. Output the union of the lightest k − 1 cuts of the n − 1 cuts associated
     with edges of T in G; let C be this union.
42      4    Multiway Cut and k-Cut

   By Lemma 4.6, the removal of C from G will leave at least k components.
If more than k components are created, throw back some of the removed
edges until there are exactly k components.
Theorem 4.8 Algorithm 4.7 achieves an approximation factor of 2 − 2/k.

Proof: Let A be an optimal k-cut in G. As in Theorem 4.4, we can view
A as the union of k cuts: Let V1 , V2 , . . . , Vk be the k components formed by
removing A from G, and let Ai denote the cut separating Vi from the rest of
the graph. Then A = A1 ∪ · · · ∪ Ak , and, since each edge of A lies in two of
these cuts,
      k
      
            w(Ai ) = 2w(A).
      i=1

Without loss of generality assume that Ak is the heaviest of these cuts. The
idea behind the rest of the proof is to show that there are k − 1 cuts deﬁned
by the edges of T whose weights are dominated by the weight of the cuts
A1 , A2 , . . . , Ak−1 . Since the algorithm picks the lightest k − 1 cuts deﬁned by
T , the theorem follows.
    The k −1 cuts are identiﬁed as follows. Let B be the set of edges of T that
connect across two of the sets V1 , V2 , . . . , Vk . Consider the graph on vertex set
V and edge set B, and shrink each of the sets V1 , V2 , . . . , Vk to a single vertex.
This shrunk graph must be connected (since T was connected). Throw edges
away until a tree remains. Let B  ⊆ B be the left over edges, |B  | = k − 1.
The edges of B  deﬁne the required k − 1 cuts.
    Next, root this tree at Vk (recall that Ak was assumed to be the heaviest
cut among the cuts Ai ). This helps in deﬁning a correspondence between the
edges in B  and the sets V1 , V2 , . . . , Vk−1 : each edge corresponds to the set it
comes out of in the rooted tree.


                                 Vk




        v
               c’(u,v)<c(A i )
                                                                         edge of B’

                                                                         edge of B-B’
                   u
                       Vi
                                               4.2   The minimum k-cut problem           43

Suppose edge (u, v) ∈ B  corresponds to set Vi in this manner. The weight of
a minimum u–v cut in G is w (u, v). Since Ai is a u–v cut in G,

      w(Ai ) ≥ w (u, v).

Thus each cut among A1 , A2 , . . . , Ak−1 is at least as heavy as the cut deﬁned
in G by the corresponding edge of B  . This, together with the fact that C is
the union of the lightest k − 1 cuts deﬁned by T , gives:

                             k−1
                                                        
                                                           k                     
                                                     1                       1
   w(C) ≤           w (e) ≤         w(Ai ) ≤       1−          w(Ai ) = 2 1 −         w(A).
                              i=1
                                                      k    i=1
                                                                              k
            e∈B 

                                                                                         ✷

Example 4.9 The tight example given above for multiway cuts on 2k ver-
tices also serves as a tight example for the k-cut algorithm (of course, there is
no need to mark vertices as terminals). Below we give the example for k = 4,
together with its Gomory–Hu tree.

                              r                                r
                              ❅
                            2 − ε❅                         2−ε
                                     ❅
                                     ❅r        1     r

                                       1              1
                                           r         r
                                               1     ❅
                            2−ε                           ❅2 − ε
                             r                             ❅❅r

                              r                                r
                              ❅
                            2 − ε❅                         2−ε
                                     ❅
                                     ❅r        2     r

                                       2              2
                                           r         r
                                                     ❅
                            2−ε                        ❅2 − ε
                             r                          ❅❅r

The lightest k − 1 cuts in the Gomory–Hu tree have weight 2 − ε each,
corresponding to picking edges of weight 2 − ε of G. So, the k-cut returned
44      4    Multiway Cut and k-Cut

by the algorithm has weight (k − 1)(2 − ε). On the other hand, the optimal
k-cut picks all edges of weight 1, and has weight k.                    ✷



4.3 Exercises

4.1 Show that Algorithm 4.3 can be used as a subroutine for ﬁnding a k-cut
within a factor of 2 − 2/k of the minimum k-cut. How many subroutine calls
are needed?

4.2 A natural greedy algorithm for computing a multiway cut is the follow-
ing. Starting with G, compute minimum si –sj cuts for all pairs si , sj that are
still connected and remove the lightest of these cuts; repeat this until all pairs
si , sj are disconnected. Prove that this algorithm also achieves a guarantee
of 2 − 2/k.

  The next 4 exercises provide background and an algorithm for ﬁnding
Gomory–Hu trees.

4.3 Let G = (V, E) be a graph and w : E → R+ be an assignment of
nonnegative weights to its edges. For u, v ∈ V let f (u, v) denote the weight
of a minimum u–v cut in G.
 1. Let u, v, w ∈ V , and suppose f (u, v) ≤ f (u, w) ≤ f (v, w). Show that
                                   two smaller numbers are equal.
    f (u, v) = f (u, w), i.e., the
 2. Show that among the n2 values f (u, v), for all pairs u, v ∈ V , there are
    at most n − 1 distinct values.
 3. Show that for u, v, w ∈ V ,

            f (u, v) ≥ min{f (u, w), f (w, v)}.

 4. Show that for u, v, w1 , . . . , wr ∈ V

            f (u, w) ≥ min{f (u, w1 ), f (w1 , w2 ), . . . , f (wr , v)}    (4.1)


4.4 Let T be a tree on vertex set V with weight function w on its edges.
We will say that T is a ﬂow equivalent tree if it satisﬁes the ﬁrst of the two
Gomory–Hu conditions. i.e., for each pair of vertices u, v ∈ V , the weight of
a minimum u–v cut in G is the same as that in T . Let K be the complete
graph on V . Deﬁne the weight of each edge (u, v) in K to be f (u, v). Show
that any maximum weight spanning tree in K is a ﬂow equivalent tree for G.
Hint: For u, v ∈ V , let u, w1 , . . . , wr , v be the unique path from u to v in
T . Use (4.1) and the fact that since T is a maximum weight spanning tree,
f (u, v) ≤ min{f (u, w1 ), . . . , f (wr , v)}.
                                                                       4.3   Exercises   45

4.5 Let (A, Ā) be a minimum s–t cut such that s ∈ A. Let x and y be any
two vertices in A. Consider the graph G obtained by collapsing all vertices
of Ā to a single vertex vĀ . The weight of any edge (a, vĀ ) in G is deﬁned to
be the sum of the weights of edges (a, b) where b ∈ Ā. Clearly, any cut in G
deﬁnes a cut in G. Show that a minimum x–y cut in G deﬁnes a minimum
x–y cut in G.

4.6 Now we are ready to state the Gomory–Hu algorithm. The algorithm
maintains a partition of V , (S1 , S2 , . . . St ), and a spanning tree T on the
vertex set {S1 , . . . , St }. Let w be the function assigning weights to the edges
of T . Tree T satisﬁes the following invariant.
Invariant: For any edge (Si , Sj ) in T there are vertices a and b in Si and
Sj respectively, such that w (Si , Sj ) = f (a, b), and the cut deﬁned by edge
(Si , Sj ) is a minimum a–b cut in G.
     The algorithm starts with the trivial partition V , and proceeds in n − 1
iterations. In each iteration, it selects a set Si in the partition such that
|Si | ≥ 2 and reﬁnes the partition by splitting Si , and ﬁnding a tree on the
reﬁned partition satisfying the invariant. This is accomplished as follows. Let
x and y be two distinct vertices in Si . Root the current tree T at Si , and
consider the subtrees rooted at the children of Si . Each of these subtrees is
collapsed into a single vertex, to obtain graph G (besides these collapsed
vertices, G contains all vertices of Si ). A minimum x–y cut is found in G .
Let (A, B) be the partition of the vertices of G deﬁning this cut, with x ∈ A
and y ∈ B, and let wxy be the weight of this cut. Compute Six = S ∩ A and
Siy = S ∩ B, the two sets into which Si splits.
     The algorithm updates the partition and the tree as follows. It reﬁnes the
partition by replacing Si with two sets Six and Siy . The new tree has the edge
(Six , Siy ), with weight wxy . Consider a subtree T  that was incident at Si in
T . Assume w.l.o.g. that the node corresponding to T  lies in A. Then, T  is
connected by an edge to Six . The weight of this connecting edge is the same
as the weight of the edge connecting T  to Si . All edges in T  retain their
weights.
     Show that the new tree satisﬁes the invariant. Hence show that the al-
gorithm terminates (when the partition consists of singleton vertices) with a
Gomory–Hu tree for G.
     Consider the graph:
                                   b                   c
                                           4
                                                               5
                              10
                                       2           2
                    a              3                   4       2        d
                          8
                                                                   7
                                               3
                                   f                       e
46        4       Multiway Cut and k-Cut

The execution of the Gomory–Hu algorithm is demonstrated below:

Initial partition:                                a, b, c, d, e, f




Select b and f:                        a, b             17               c, d, e, f




Select a and b:             a     18          b         17               c, d, e, f




Select c and f:             a     18          b         17           f            13       c, d, e




Select d and e:             a     18          b         17           f            13        c, e

                                                                                               14

                                                                                             d




Select c and e:             a     18          b         17           f            13   e             15   c

                                                                                       14

                                                                                       d



4.7 Prove that if the Gomory–Hu tree for an edge-weighted undirected graph
G contains all n − 1 distinct weights, then G can have only one minimum
weight cut.


4.4 Notes
Algorithm 4.3 is due to Dahlhaus, Johnson, Seymour, Papadimitriou and
Yannakakis [57]. Algorithm 4.7 is due to Saran and Vazirani [233]; the proof
given here is due to R. Ravi. For Gomory–Hu trees see Gomory and Hu [110].
