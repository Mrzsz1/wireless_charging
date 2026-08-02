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

# Feedback Vertex Set

6 Feedback Vertex Set




In this chapter we will use the technique of layering, introduced in Chapter
2, to obtain a factor 2 approximation algorithm for:
Problem 6.1 (Feedback vertex set) Given an undirected graph G =
(V, E) and a function w assigning nonnegative weights to its vertices, ﬁnd a
minimum weight subset of V whose removal leaves an acyclic graph.


6.1 Cyclomatic weighted graphs
Order the edges of G in an arbitrary order. The characteristic vector of a sim-
                                     m
ple cycle C in G is a vector in GF[2] , m = |E|, which has 1’s in components
corresponding to edges of C and 0’s in the remaining components. The cycle
                                     m
space of G is the subspace of GF[2] that is spanned by the characteristic
vectors of all simple cycles of G, and the cyclomatic number of G, denoted
cyc(G), is the dimension of this space.

Theorem 6.2 cyc(G) = |E| − |V | + κ(G), where κ(G) denotes the number
of connected components of G.

Proof: The cycle space of a graph is the direct sum of the cycle spaces of
its connected components, and so its cyclomatic number is the sum of the
cyclomatic numbers of its connected components. Therefore, it is suﬃcient
to prove the theorem for a connected graph G.
    Let T be a spanning tree in G. For each nontree edge e, deﬁne its funda-
mental cycle to be the unique cycle formed in T ∪{e}. The set of characteristic
vectors of all such cycles is linearly independent (each cycle includes an edge
that is in no other fundamental cycle). Thus, cyc(G) ≥ |E| − |V | + 1.
    Each edge e of T deﬁnes a fundamental cut (S, S) in G, S ⊂ V (S and S
are the vertex sets of two connected components formed by removing e from
                                                                         m
T ). Deﬁne the characteristic vector of a cut to be a vector in GF[2] that
has 1’s in components corresponding to the edges of G in the cut and 0’s
in the remaining components. Consider the |V | − 1 vectors deﬁned by edges
of T . Since each cycle must cross each cut an even number of times, these
vectors are orthogonal to the cycle space of G. Furthermore, these |V | − 1
                                         6.1   Cyclomatic weighted graphs        55

vectors are linearly independent, since each cut has an edge (the tree edge
deﬁning this cut) that is not in any of the other |V | − 2 cuts. Therefore the
dimension of the orthogonal complement to the cycle space is at least |V | − 1.
Hence, cyc(G) ≤ |E| − |V | + 1. Combining with the previous inequality we
get cyc(G) = |E| − |V | + 1.                                                 ✷
   Denote by δG (v) the decrease in the cyclomatic number of the graph on
removing vertex v. Since the removal of a feedback vertex set F = {v1 , . . . , vf }
decreases the cyclomatic number of G down to 0,

                 f
                 
      cyc(G) =         δGi−1 (vi ),
                 i=1


where G0 = G and, for i > 0, Gi = G − {v1 , . . . , vi }. By Lemma 6.4 below,
we get:
                 
      cyc(G) ≤         δG (v).                                                (6.1)
                 v∈F


    Let us say that a function assigning vertex weights is cyclomatic if there
is a constant c > 0 such that the weight of each vertex v is c · δG (v). By
inequality (6.1), for such a weight function, c · cyc(G) is a lower bound on
OPT. The importance of cyclomatic weight functions is established in Lemma
6.5 below, which shows that for such a weight function, any minimal feedback
vertex set has a weight within twice the optimal.
    Let degG (v) denote the degree of v in G, and comps(G − v) denote the
number of connected components formed by removing v from G. The claim
below follows in a straightforward way by applying Theorem 6.2 to G and
G − v.

Claim 6.3 For a connected graph G, δG (v) = degG (v) − comps(G − v).

Lemma 6.4 Let H be a subgraph of G (not necessarily vertex induced).
Then, δH (v) ≤ δG (v).

Proof: It is suﬃcient to prove the lemma for the connected components
of G and H containing v. We may thus assume w.l.o.g. that G and H are
connected (H may be on a smaller set of vertices). By Claim 6.3, proving the
following inequality is suﬃcient:

      degH (v) − comps(H − v) ≤ degG (v) − comps(G − v).

We will show that edges in G − H can only help this inequality. Let
c1 , c2 , . . . , ck be components formed by removing v from H. Edges of G − H
not incident at v can only help merge some of these components (and of
56     6     Feedback Vertex Set

course, they don’t change the degree of v). An edge of G − H that is incident
at v can lead to an additional component, but this is compensated by the
contribution the edge has to the degree of v.                              ✷

Lemma 6.5 If F is a minimal feedback vertex set of G, then
      
            δG (v) ≤ 2 · cyc(G).
     v∈F


Proof: Since the cycle space of G is the direct sum of the cycle spaces of its
connected components, it suﬃces to prove the lemma for a connected graph
G.
   Let F = {v1 , . . . , vf }, and let k be the number of connected components
obtained by deleting F from G. Partition these components into two types:
those that have edges incident to only one of the vertices of F , and those
that have edges incident to two or more vertices of F . Let t and k − t be
the number of components of the ﬁrst and second type, respectively. We will
prove that

     f
                        f
                         
            δG (vi ) =         (degG (vi ) − comps(G − vi )) ≤ 2(|E| − |V |),
      i=1                i=1

                                              f
thereby proving the lemma. Clearly,               i=1 comps(G − vi ) = f + t. Therefore,
we are left to prove

     f
     
            degG (vi ) ≤ 2(|E| − |V |) + f + t.
      i=1

                                   v1        v2               vk
                                                     ...




                                                        ...
                 comp. 1                comp. 2                    comp. k


   Since F is a feedback vertex set, each of the k components is acyclic and is
therefore a tree. Thus, the number of edges in these components is |V |−f −k.
Next, we put a lower bound on the number of edges in the cut (F, V − F ).
                                     6.2   Layering applied to feedback vertex set   57

Since F is minimal, each vi ∈ F must be in a cycle that contains no other
vertices of F . Therefore, each vi must have at least two edges incident at one
of the components. For each vi , arbitrarily remove one of these edges from
G, thus removing a total of f edges. Now, each of the t components must
still have at least one edge and each of the k − t components must still have
at least two edges incident at F . Therefore, the number of edges in the cut
(F, V − F ) is at least f + t + 2(k − t) = f + 2k − t.
     These two facts imply that

      f
      
            degG (vi ) ≤ 2|E| − 2(|V | − f − k) − (f + 2k − t).
      i=1

The lemma follows.                                                                   ✷

Corollary 6.6 Let w be a cyclomatic weight function on the vertices of G,
and let F be a minimal feedback vertex set in it. Then w(F ) ≤ 2 · OPT.



6.2 Layering applied to feedback vertex set
Let us now deal with arbitrary weighted graphs. Consider the following basic
operation: Given graph G = (V, E) and a weight function w, let
                             
                    w(v)
      c = min                    .
            v∈V     δG (v)

The weight function t(v) = cδG (v) is the largest cyclomatic weight function
in w. Deﬁne w (v) = w(v) − t(v) to be the residual weight function. Finally,
let V  be the set of vertices having positive residual weight (clearly, V  ⊂ V ),
and let G be the subgraph of G induced on V  .
     Using this basic operation, decompose G into a nested sequence of induced
subgraphs, until an acyclic graph is obtained, each time ﬁnding the largest
cyclomatic weight function in the current residual weight function. Let these
graphs be G = G0 ⊃ G1 ⊃ · · · ⊃ Gk , where Gk is acyclic; Gi is the induced
subgraph of G on vertex set Vi , where V = V0 ⊃ V1 ⊃ · · · ⊃ Vk . Let ti , i =
0, . . . , k − 1 be the cyclomatic weight function deﬁned on graph Gi . Thus,
w0 = w is the residual weight function for G0 , t0 is the largest cyclomatic
weight function in w0 , w1 = w0 − t0 is the residual weight function for G1 ,
and so on. Finally, wk is the residual weight function for Gk . For convenience,
deﬁne tk = wk . Since the weight of a vertex v has been decomposed into the
weights t0 , t1 , . . . , tk , we have
       
                ti (v) = w(v).
      i: v∈Vi
58        6   Feedback Vertex Set

    The next fact suggests an algorithm for constructing a feedback vertex
set on which Lemma 6.5 can be applied.
Lemma 6.7 Let H be a subgraph of G = (V, E), induced on vertex set V  ⊂
V . Let F be a minimal feedback vertex set in H, and let F  ⊆ V − V  be a
minimal set such that F ∪ F  is a feedback vertex set for G. Then F ∪ F  is
a minimal feedback vertex set for G.

Proof: Since F is minimal for H, for each v ∈ F , there is a cycle, say C, in
H that does not use any other vertex of F . Since F  ∩ V  = ∅, C uses only
one vertex, v, from F ∪ F  as well, and so v is not redundant.            ✷
    After the entire decomposition, Fk = ∅ is a minimal feedback vertex set
of Gk . For i = k, k − 1, . . . , 1, the minimal feedback vertex set Fi found in Gi
is extended in a minimal way using vertices of Vi−1 − Vi to yield a minimal
feedback vertex set, say Fi−1 , for Gi−1 . The last set, F0 , is a feedback vertex
set for G.

                                       Gk


                               Gk-1                  Fk-1

                                              ...


                     G1                                     F1 - F2


              G0                                                      F0 - F1




 Algorithm 6.8 (Feedback vertex set)
     1. Decomposition phase
         H ← G, w ← w, i ← 0
         While H is not acyclic,
                           
            c ← minu∈H δwH(u) (u)
             Gi ← H, ti ← c · δGi , w ← w − ti
             H ← the subgraph of Gi induced by vertices u with w (u) > 0
             i ← i + 1,
         k ← i, Gk ← H
     2. Extension phase
         Fk ← ∅
         For i = k, . . . , 1, extend Fi to a feedback vertex set Fi−1 of Gi−1 by
            adding a minimal set of vertices from Vi−1 − Vi .
         Output F0 .
                                                       6.2                                          Layering applied to feedback vertex set                                                                                                                                       59

Theorem 6.9 Algorithm 6.8 achieves an approximation guarantee of factor
2 for the feedback vertex set problem.

Proof: Let F ∗ be an optimal feedback vertex set for G. Since Gi is an
induced subgraph of G, F ∗ ∪ Vi must be a feedback vertex set for Gi (not
necessarily optimal). Since the weights of vertices have been decomposed into
the functions ti , we have
                           k
                                                                                                                                                                            k
                                                                                                                                                                             
     OPT = w(F ∗ ) =                             ti (F ∗ ∩ Vi ) ≥                                                                                                                                             OPTi ,
                           i=0                                                                                                                                                 i=0

where OPTi is the weight of an optimal feedback vertex set of Gi with weight
function ti .
   By decomposing the weight of F0 , we get
                k
                                                                                              k
                                                                                               
     w(F0 ) =         ti (F0 ∩ Vi ) =                                                                                           ti (Fi ).
                i=0                                                                              i=0

By Lemma 6.7, Fi is a minimal feedback vertex set in Gi . Since for 0 ≤ i ≤
k − 1, ti is a cyclomatic weight function, by Lemma 6.5, ti (Fi ) ≤ 2OPTi ;
recall that Fk = ∅. Therefore,
                  k
                  
     w(F0 ) ≤ 2         OPTi ≤ 2 · OPT.
                  i=0

                                                                                                                                                                                                                                                                                  ✷

Example 6.10 A tight example for the algorithm is given by the graph
obtained by removing a perfect matching from a complete bipartite graph
and duplicating every edge. (Note that the algorithm works for parallel edges
as well. If a tight example without parallel edges is desired, then a vertex
with very high weight can be placed on every edge.)
                                                          sP                  ✏  s
                                                           ◗
                                                           ❙P   ◗PPP    ✏  ✏✏✑✑
                                                                    ✏✏P✑ 
                             ....................................................................................................
                             .....                             ❙ ◗                                                                                                                                     .....
                                 ...
                                                                ✏❙✏◗ ✑P                                                                                                                                   ...
                                                           s✏         ✑ P    P
                                                                              ✏s
                                   ....                                                                                                                                                                      ....
                                      ...                   PP        ◗                                                                                                                                         ...
                                        ...
                                          ....              ❅ P   ❙ ✑
                                                                    P   ◗
                                                                        ✏  ✏✏                                                                                                                                     ...
                                                                                                                                                                                                                    ....
                                             ....
                                                                ❅ ✑✏P ✏P ◗                                                                                                                                            ....
                                                                ✑✏❙
                                                ...                                                                                                                                                                       ...
                                                  ....                     P◗                                                                                                                                               ....
                                                     ...
                                                       .... ✏✏❅ ❙
                                                            s✑               P◗
                                                                              Ps                                                                                                                                               ...
                                                                                                                                                                                                                                 ....
                                                          ...◗      ❅       ✑✑                                                                                                                                                     ...
                                                            ...
                                                                ◗ 
                                                              ...      ❙                                                                                                                                                              ....
                                                                                                                                                                                                                                         ...
                                                                ....
                                                                  ◗ ❅✑
                                                                   ....  ❙ ✑                                                                                                                                                               ...
                                                                                                                                                                                                                                             ....
                                                         ..       ◗ ❅❙
                                                                      ...
                                                                                ..                                                                                                                                                              ...

                                                          .  ✑✑
                                                                        ....                                                                                                                                                                      ....
                                                                      ◗    ...
                                                                                 .                                                                                                                                                                   ...
                                                                        ◗  ❅ ....                                                                                                                                                                      ....

                                                               ✑✑         ◗❙
                                                                                ...                                                                                                                                                                       ...
                                                                                  ....                                                                                                                                                                      ....
                                                                             ❅
                                                                             ◗❙      ...
                                                                                       ...
                                                                                                                                                                                                                                                               ...
                                                                                                                                                                                                                                                                 ...
                                                             
                                                             s✑               ❅
                                                                              ◗❙s        ...
                                                                                           ....
                                                                                                                                                                                                                                                                   ...
                                                                                                                                                                                                                                                                     ....
                                                                                              ....                                                                                                                                                                      ....
                                                                                                 ...                                                                                                                                                                       ...
                                                                                                   .
                                                                                                   ....................................................................................................                                                                      ..
60     6   Feedback Vertex Set

Assuming that the graph is cyclomatic weighted, each vertex receives the
same weight. The decomposition obtained by the algorithm consists of only
one nontrivial graph, G itself, on which the algorithm computes a minimal
feedback vertex set. A possible output of the algorithm is the set shown
above; this set contains 2n − 2 vertices as compared with the optimum of n
given by one side of the bipartition.                                   ✷



6.3 Exercises

6.1 A natural greedy algorithm for ﬁnding a minimum feedback vertex set
is to repeatedly pick and remove the most cost-eﬀective vertex, i.e., a vertex
minimizing w(v)/δH (v), where H is the current graph, until there are no
more cycles left. Give examples to show that this is not a constant factor
algorithm. What is the approximation guarantee of this algorithm?

6.2 Give an approximation factor preserving reduction from the vertex cover
problem to the feedback vertex set problem (thereby showing that improving
the factor for the latter problem will also improve it for the former; also see
Section 30.1).


6.4 Notes
Algorithm 6.8 is due to Bafna, Berman, and Fujito [19] (see also Becker and
Geiger [23] and Chudak, Goemans, Hochbaum, and Williamson [46] for other
factor 2 algorithms for the feedback vertex set problem).
