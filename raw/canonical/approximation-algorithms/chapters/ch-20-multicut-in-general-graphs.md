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

# Multicut in General Graphs

20 Multicut in General Graphs




The importance of min–max relations to combinatorial optimization was
mentioned in Chapter 1. Perhaps the most useful of these is the celebrated
max-ﬂow min-cut theorem. Indeed, much of ﬂow theory, and the theory of
cuts in graphs, has been built around this theorem. It is not surprising, there-
fore, that a concerted eﬀort was made to obtain generalizations of this theo-
rem to the case of multiple commodities.
    There are two such generalizations. In the ﬁrst one, the objective is to
maximize the sum of the commodities routed, subject to ﬂow conservation
and capacity constraints. In the second generalization, a demand dem(i) is
speciﬁed for each commodity i, and the objective is to maximize f , called
throughput, such that for each i, f · dem(i) amount of commodity i can
be routed simultaneously. We will call these sum multicommodity ﬂow and
demands multicommodity ﬂow problems, respectively. Clearly, for the case
of a single commodity, both problems are the same as the maximum ﬂow
problem.
    Each of these generalizations is associated with a fundamental NP-hard
cut problem, the ﬁrst with the minimum multicut problem, Problem 18.1,
and the second with the sparsest cut problem, Problem 21.2. In each case
an approximation algorithm for the cut problem gives, as a corollary, an
approximate max-ﬂow min-cut theorem. In this chapter we will study the
ﬁrst generalization; the second is presented in Chapter 21. We will obtain an
O(log k) factor approximation algorithm for the minimum multicut problem,
where k is the number of commodities. A factor 2 algorithm for the special
case of trees was presented in Chapter 18.


20.1 Sum multicommodity ﬂow

Problem 20.1 (Sum multicommodity ﬂow) Let G = (V, E) be an
undirected graph with nonnegative capacity ce for each edge e ∈ E. Let
{(s1 , t1 ), . . . , (sk , tk )} be a speciﬁed set of pairs of vertices where each pair
is distinct, but vertices in diﬀerent pairs are not required to be distinct. A
separate commodity is deﬁned for each (si , ti ) pair. For convenience, we will
think of si as the source and ti as the sink of this commodity. The objective
                                             20.1   Sum multicommodity ﬂow     169

is to maximize the sum of the commodities routed. Each commodity must
satisfy ﬂow conservation at each vertex other than its own source and sink.
Also, the sum of ﬂows routed through an edge, in both directions combined,
should not exceed the capacity of this edge.
    Let us ﬁrst give a linear programming formulation for this problem. For
each commodity i, let Pi denote the set of all paths from si to ti in G, and
           k
let P = i=1 Pi . The LP will have a variable fp for each p ∈ P , which will
denote the ﬂow along path p. The endpoints of this path uniquely specify
the commodity that ﬂows on this path. The objective is to maximize the
sum of ﬂows routed on these paths, subject to edge capacity constraints.
Notice that ﬂow conservation constraints are automatically satisﬁed in this
formulation. The program has exponentially many variables; however, that
is not a concern since we will use it primarily to obtain a clean formulation
of the dual program.
                    
      maximize            fp                                                 (20.1)
                   p∈P
                    
      subject to           fp ≤ ce ,    e∈E
                   p:e∈p
                   fp ≥ 0,              p∈P

   Let us obtain the dual of this program. For this, let de be the dual variable
associated with edge e. We will interpret these variables as distance labels of
edges.
                   
      minimize           ce de                                               (20.2)
                   e∈E
                   
                         de ≥ 1,       p∈P
                   e∈p
                   de ≥ 0,             e∈E

    The dual program tries to ﬁnd a distance label assignment to edges so
that on each path p ∈ P , the distance labels of edges add up to at least 1.
Equivalently, a distance label assignment is feasible iﬀ for each commodity i,
the shortest path from si to ti has length at least 1.
    Notice that the programs (18.2) and (18.1) are special cases of the two
programs presented above for the restriction that G is a tree.
    The following remarks made in Chapter 18 hold for the two programs pre-
sented above as well: an optimal integral solution to LP (20.2) is a minimum
multicut, and an optimal fractional solution can be viewed as a minimum
fractional multicut. By the LP-duality theorem, minimum fractional multi-
cut equals maximum multicommodity ﬂow and, as shown in Example 18.2,
it may be strictly smaller than minimum integral multicut.
170     20   Multicut in General Graphs

    This naturally raises the question whether the ratio of minimum multicut
and maximum multicommodity ﬂow is bounded. Equivalently, is the integral-
ity gap of LP (20.2) bounded? In the next section we present an algorithm for
ﬁnding a multicut within an O(log k) factor of the maximum ﬂow, thereby
showing that the gap is bounded by O(log k).


20.2 LP-rounding-based algorithm
First notice that the dual program (20.2) can be solved in polynomial time
using the ellipsoid algorithm, since there is a simple way of obtaining a sepa-
ration oracle for it: simply compute the length of a minimum si –ti path, for
each commodity i, w.r.t. the current distance labels. If all these lengths are
≥ 1, we have a feasible solution. Otherwise, the shortest such path provides
a violated inequality. Alternatively, the LP obtained in Exercise 20.1 can be
solved in polynomialtime. Let de be the distance label computed for each
edge e, and let F = e∈E ce de .
     Our goal is to pick a set of edges of small capacity, compared to F , that
is a multicut. Let D be the set of edges with positive distance labels, i.e.,
D = {e | de > 0}. Clearly, D is a multicut; however, its capacity may be
very large compared to F (Exercises 20.3 and 20.4). How do we pick a small
capacity subset of D that is still a multicut? Since the optimal fractional
multicut is the most cost-eﬀective way of disconnecting all source–sink pairs,
edges with large distance labels are more important than those with small
distance labels for this purpose. The algorithm described below indirectly
gives preference to edges with large distance labels.
     The algorithm will work on graph G = (V, E) with edge lengths given by
de . The weight of edge e is deﬁned to be ce de . Let dist(u, v) denote the length
of the shortest path from u to v in this graph. For a set of vertices S ⊂ V ,
δ(S) denotes the set of edges in the cut (S, S), c(S) denotes the capacity
of this cut, i.e., the total capacity of edges in δ(S), and wt(S) denotes the
weight of set S, which is roughly the sum of weights of all edges having both
endpoints in S (a more precise deﬁnition is given below).
     The algorithm will ﬁnd disjoint sets of vertices, S1 , . . . , Sl , l ≤ k, in G,
called regions, such that:
• No region contains any source–sink pair, and for each i, either si or ti is
  in one of the regions.
• For each region Si , c(Si ) ≤ ε wt(Si ), where ε is a parameter that will be
  deﬁned below.
By the ﬁrst condition, the union of the cuts of these regions, i.e., M =
δ(S1 ) ∪ δ(S2 ) ∪ . . . ∪ δ(Sl ), is a multicut, and by the second condition, its
capacity c(M ) ≤ εF . (When we give the precise deﬁnition of wt(S), this
inequality will need to be modiﬁed slightly.)
                                             20.2   LP-rounding-based algorithm   171

20.2.1   Growing a region: the continuous process

The sets S1 , . . . , Sl are found through a region growing process. Let us ﬁrst
present a continuous process to clarify the issues. For the sake of time eﬃ-
ciency, the algorithm itself will use a discrete process (see Section 20.2.2).
     Each region is found by growing a set starting from one vertex, which is
the source or sink of a pair. This will be called the root of the region. Suppose
the root is s1 . The process consists of growing a ball around the root. For
each radius r, deﬁne S(r) to be the set of vertices at a distance ≤ r from s1 ,
i.e., S(r) = {v | dist(s1 , v) ≤ r}. S(0) = {s1 }, and as r increases continuously
from 0, at discrete points, S(r) grows by adding vertices in increasing order
of their distance from s1 .
Lemma 20.2 If the region growing process is terminated before the radius
becomes 1/2, then the set S that is found contains no source–sink pair.

Proof: The distance between any pair of vertices in S(r) is ≤ 2r. Since for
each commodity i, dist(si , ti ) ≥ 1, the lemma follows.                 ✷
    For technical reasons that will become clear in Lemma 20.3 (see also
Exercises 20.5 and 20.6), we will assign a weight to the root, wt(s1 ) = F/k.
The weight of S(r) is the sum of wt(s1 ) and the sum of the weights of edges,
or parts of edges, in the ball of radius r around s1 . Let us state this formally.
For edges e having at least one endpoint in S(r), let qe denote the fraction
of edge e that is in S(r). If both endpoints of e are in S(r), then qe = 1.
Otherwise, suppose e = (u, v) with u ∈ S(r) and v ∈ S(r). For such edges,

                  r − dist(s1 , u)
      qe =                               .
             dist(s1 , v) − dist(s1 , u)

Deﬁne the weight of region S(r),
                                
      wt(S(r)) = wt(s1 ) +          ce de qe ,

where the sum is over all edges having at least one endpoint in S(r).
    We want to ﬁx ε so that we can guarantee that we will encounter the
condition c(S(r)) ≤ ε wt(S(r)) for r < 1/2. The important observation is
that at each point the rate at which the weight of the region is growing is at
least c(S(r)). Until this condition is encountered,

      d wt(S(r)) ≥ c(S(r)) dr > ε wt(S(r)) dr.

Exercise 20.5 will help the reader gain some understanding of such a process.
Lemma 20.3 Picking ε = 2 ln(k + 1) suﬃces to ensure that the condition
c(S(r)) ≤ ε wt(S(r)) will be encountered before the radius becomes 1/2.
172      20    Multicut in General Graphs

Proof: The proof is by contradiction. Suppose that throughout the region
growing process, starting with r = 0 and ending at r = 1/2, c(S(r)) >
ε wt(S(r)). At any point the incremental change in the weight of the region
is
                   
      d wt(S(r)) =     ce de dqe .
                       e


Clearly, only edges having one endpoint in S(r) will contribute to the sum.
Consider such an edge e = (u, v) such that u ∈ S(r) and v ∈ S(r). Then,

                                     de
      ce de dqe = ce                                dr.
                        dist(s1 , v) − dist(s1 , u)

Since dist(s1 , v) ≤ dist(s1 , u) + de , we get de ≥ dist(s1 , v) − dist(s1 , u), and
hence ce de dqe ≥ ce dr. This gives

      d wt(S(r)) ≥ c(S(r)) dr > ε wt(S(r)) dr.

As long as the terminating condition is not encountered, the weight of the
region increases exponentially with the radius. The initial weight of the region
is F/k and the ﬁnal weight is at most F + F/k. Integrating we get

         F+F                                    1
           k       1                            2
                         d wt(S(r)) >               ε dr.
         F
         k
                wt(S(r))                    0


Therefore, ln(k + 1) > 12 ε. However, this contradicts the assumption that
ε = 2 ln(k + 1), thus proving the lemma.                                ✷


20.2.2       The discrete process

The discrete process starts with S = {s1 } and adds vertices to S in increasing
order of their distance from s1 . Essentially, it involves executing a shortest
path computation from the root. Clearly, the sets of vertices found by both
processes are the same.
   The weight of region S is redeﬁned for the discrete process as follows:
                           
      wt(S) = wt(s1 ) +          ce de ,
                             e

where the sum is over all edges that have at least one endpoint in S, and
wt(s1 ) = F/k. The discrete process stops at the ﬁrst point when c(S) ≤
ε wt(S), where ε is again 2 ln(k + 1). Notice that for the same set S, wt(S)
in the discrete process is at least as large as that in the continuous process.
                                               20.2   LP-rounding-based algorithm   173

Therefore, the discrete process cannot terminate with a larger set than that
found by the continuous process. Hence, the set S found contains no source–
sink pair.

20.2.3      Finding successive regions

The ﬁrst region is found in graph G, starting with any one of the sources
as the root. Successive regions are found iteratively. Let G1 = G and S1
be the region found in G1 . Consider a general point in the algorithm when
regions S1 , . . . , Si−1 have already been found. Now, Gi is deﬁned to be the
graph obtained by removing vertices S1 ∪ . . . ∪ Si−1 , together with all edges
incident at them from G.
    If Gi does not contain a source–sink pair, we are done. Otherwise, we pick
the source of such a pair, say sj , as the root, deﬁne its weight to be F/k,
and grow a region in Gi . All deﬁnitions, such as distance and weight, are
w.r.t. graph Gi . We will denote these with a subscript of Gi . Also, for a set
of vertices S in Gi , cGi (S) will denote the total capacity of edges incident at
S in Gi , i.e., the total capacity of edges in δGi (S). As before, the value of ε is
2 ln(k +1), and the terminating condition is cGi (Si ) ≤ ε wtGi (Si ). Notice that
in each iteration the root is the only vertex that is deﬁned to have nonzero
weight.
            S1                       t4                  S2



                      s1
                           t2
                                                              s3




                                S3


       s2
                                          s4    t1                    δG (S1 )
                                                                         1



                                                                       δG (S2 )
                 t3                                                      2



                                                                      δG (S3 )
                                                                         2

    In this manner, we will ﬁnd regions S1 , . . . , Sl , l ≤ k, and will output the
set M = δG1 (S1 ) ∪ . . . ∪ δGl (Sl ). Since edges of each cut are removed from
the graph for successive iterations, the sets in this union are disjoint, and
c(M ) = i cGi (Si ).
    The algorithm is summarized below. Notice that while a region is growing,
edges with large distance labels will remain in its cut for a longer time, and
174    20   Multicut in General Graphs

thus are more likely to be included in the multicut found. (Of course, the
precise time that an edge remains in the cut is given by the diﬀerence between
the distances from the root to the two endpoints of the edge.) As promised,
the algorithm indirectly gives preference to edges with large distance labels.


 Algorithm 20.4 (Minimum multicut)
  1. Find an optimal solution to the LP (20.2), thus obtaining distance labels
     for edges of G.
  2. ε ← 2 ln (k + 1) , H ← G, M ← ∅;
  3. While ∃ a source–sink pair in H do:
      Pick such a source, say sj ;
      Grow a region S with root sj until cH (S) ≤ ε wtH (S);
      M ← M ∪ δH (S);
      H ← H with vertices of S removed;
  4. Output M .


Lemma 20.5 The set M found is a multicut.

Proof: We need to prove that no region contains a source–sink pair. In each
iteration i, the sum of weights of edges of the graph and the weight deﬁned
on the current root is bounded by F + F/k. By the proof of Lemma 20.3, the
continuous region growing process is guaranteed to encounter the terminating
condition before the radius of the region becomes 1/2. Therefore, the distance
between a pair of vertices in the region, Si , found by the discrete process is
also bounded by 1. Notice that we had deﬁned these distances w.r.t. graph
Gi . Since Gi is a subgraph of G, the distance between a pair of vertices in G
cannot be larger than that in Gi . Hence, Si contains no source–sink pair. ✷

Lemma 20.6 c(M ) ≤ 2εF = 4 ln(k + 1)F .

Proof: In each iteration i, by the terminating condition we have cGi (Si ) ≤
ε wtGi (Si ). Since all edges contributing to wtGi (Si ) will be removed from the
graph after this iteration, each edge of G contributes to the weight of at most
one region. The total weight of all edges of G is F . Since each iteration helps
disconnect at least one source–sink pair, the number of iterations is bounded
by k. Therefore, the total weight attributed to source vertices is at most F .
Summing gives:
                                                                      
                                                          F 
      c(M ) =       cGi (Si ) ≤ ε        wtGi (Si )   ≤ε   k +   ce de       = 2εF.
                i                    i
                                                            k  e

                                                                                 ✷
                                                 20.3    A tight example      175

Theorem 20.7 Algorithm 20.4 achieves an approximation guarantee of
O(log k) for the minimum multicut problem.

Proof: The proof follows from Lemmas 20.5 and 20.6, and from the fact
that the value of the fractional multicut, F , is a lower bound on the minimum
multicut.                                                                    ✷
   Exercise 20.6 justiﬁes the choice of wt(s1 ) = F/k.
Corollary 20.8 In an undirected graph with k source–sink pairs,
                                                                       
         max     |F | ≤      min       |C| ≤ O(log k)       max      |F | ,
     m/c ﬂow F            multicut C                     m/c ﬂow F

where |F | represents the value of multicommodity ﬂow F , and |C| represents
the capacity of multicut C.



20.3 A tight example

Example 20.9 We will construct an inﬁnite family of graphs for which the
integrality gap for LP (20.2) is Ω(log k), thereby showing that our analy-
sis of Algorithm 20.4 and the approximate max-ﬂow min-multicut theorem
presented in Corollary 20.8 are tight within constant factors.
    The construction uses expander graphs. An expander is a graph G =
(V, E) in which every vertex has the same degree, say d, and for any nonempty
subset S ⊂ V ,

     |E(S, S)| > min(|S|, |S|),

where E(S, S) denotes the set of edges in the cut (S, S), i.e., edges that
have one endpoint in S and the other in S. Standard probabilistic arguments
show that almost every constant degree graph, with d ≥ 3, is an expander
(see Section 20.6). Let H be such a graph containing k vertices.
    Source–sink pairs are designated in H as follows. Consider a breadth ﬁrst
search tree rooted at some vertex v. The number of vertices within distance
α−1 of vertex v is at most 1+d+d2 +. . .+dα−1 < dα . Picking α = logd k/2
ensures that at least k/2 vertices are at a distance ≥ α from v. Let us say
that a pair of vertices are a source–sink pair if the distance between them is
at least α. Therefore, we have chosen Θ(k 2 ) pairs of vertices as source–sink
pairs.
    Each edge in H is of unit capacity. Thus, the total capacity of edges of
H is O(k). Since the distance between each source–sink pair is Ω(log k), any
ﬂow path carrying a unit of ﬂow uses up Ω(log k) units of capacity. Therefore,
176    20   Multicut in General Graphs

the value of maximum multicommodity ﬂow in H is bounded by O(k/log k).
Next we will prove that a minimum multicut in H, say M , has capacity
Ω(k), thereby proving the claimed integrality gap. Consider the connected
components obtained by removing M from H.
Claim 20.10 Each connected component has at most k/2 vertices.

Proof: Suppose a connected component has strictly more than k/2 vertices.
Pick an arbitrary vertex v in this component. By the argument given above,
the number of vertices that are within distance α − 1 of v in the entire graph
H is < dα ≤ k/2. Thus, there is a vertex u in the component such that the
distance between u and v is at least α, i.e., u and v form a source–sink pair.
Thus removal of M has failed to disconnect a source–sink pair, leading to a
contradiction.                                                               ✷
    By Claim 20.10, and the fact that H is an expander, each component
S
  has  |δ(S)| ≥ |S|. Since each vertex of H is in one of the components,
   S |δ(S)| ≥ k, where the sum is over all connected components. Since an
edge contributes to the cuts of at most two components, the number of edges
crossing components is Ω(k). This gives the desired lower bound on the min-
imum multicut.
    Next, let us ensure that the number of source–sink pairs deﬁned in the
graph is not related to the number of vertices in it. Notice that replacing
an edge of H by a path of unit capacity edges does not change the value of
maximum ﬂow or minimum multicut. Using this operation we can construct
from H a graph G having n vertices, for arbitrary n ≥ k. The integrality gap
of LP (20.2) for G is Ω(log k).
                                                                          ✷



20.4 Some applications of multicut
We will obtain an O(log n) factor approximation algorithm for the following
problem by reducing to the minimum multicut problem. See Exercise 20.7
for further applications.
Problem 20.11 (2CNF≡ clause deletion) A 2CNF≡ formula consists
of a set of clauses of the form (u ≡ v), where u and v are literals. Let F be
such a formula, and wt be a function assigning nonnegative rational weights
to its clauses. The problem is to delete a minimum weight set of clauses of F
so that the remaining formula is satisﬁable.
    Given a 2CNF≡ formula F on n Boolean variables, let us deﬁne graph
G(F ) with edge capacities as follows: The graph has 2n vertices, one corre-
sponding to each literal. Corresponding to each clause (p ≡ q) we include the
two edges (p, q) and (p, q), each having capacity equal to the weight of the
clause (p ≡ q).
                                                        20.5   Exercises   177

   Notice that the two clauses (p ≡ q) and (p ≡ q) are equivalent. We may
assume w.l.o.g. that F does not contain two such equivalent clauses, since we
can merge their weights and drop one of these clauses. With this assumption
each clause corresponds to two distinct edges in G(F ).
Lemma 20.12 Formula F is satisﬁable iﬀ no connected component of G(F )
contains a variable and its negation.

Proof: If (p, q) is an edge in G(F ) then the literals p and q must take the
same truth value in every satisfying truth assignment. Thus, all literals of
a connected component of G(F ) are forced to take the same truth value.
Therefore, if F is satisﬁable, no connected component in G(F ) contains a
variable and its negation.
   Conversely, notice that if literals p and q occur in the same connected
component, then so do their negations. If no connected component contains
a variable and its negation, the components can be paired so that in each
pair, one component contains a set of literals and the other contains the
complementary literals. For each pair, set the literals of one component to
true and the other to false to obtain a satisfying truth assignment.      ✷
   For each variable and its negation, designate the corresponding vertices
in G(F ) to be a source–sink pair, thus deﬁning n source–sink pairs. Let M
be a minimum multicut in G(F ) and C be a minimum weight set of clauses
whose deletion makes F satisﬁable. In general, M may have only one of the
two edges corresponding to a clause.
Lemma 20.13 wt(C) ≤ c(M ) ≤ 2 · wt(C).

Proof: Delete clauses corresponding to edges of M from F to get formula F  .
The weight of clauses deleted is at most c(M ). Since G(F  ) does not contain
any edges of M , it does not have any component containing a variable and its
negation. By Lemma 20.12, F  is satisﬁable, thus proving the ﬁrst inequality.
    Next, delete from G(F ) the two edges corresponding to each clause in C.
This will disconnect all source–sink pairs. Since the capacity of edges deleted
is 2wt(C), this proves the second inequality.                                 ✷
   Since we can approximate minimum multicut to within an O(log n) factor,
we get:
Theorem 20.14 There is an O(log n) factor approximation algorithm for
Problem 20.11.


20.5 Exercises

20.1 By deﬁning for each edge e and commodity i a ﬂow variable fe,i , give
an LP that is equivalent to LP (20.1) and has polynomially many variables.
178    20   Multicut in General Graphs

Obtain the dual of this program and show that it is equivalent to LP (20.2);
however, unlike LP (20.2), it has only polynomially many constraints.

20.2 Let d be an optimal solution to LP (20.2). Show that d must satisfy
the triangle inequality.

20.3 Intuitively, our goal in picking a multicut is picking edges that are
bottlenecks for multicommodity ﬂow. In this sense, D is a very good starting
point: prove that D is precisely the set of edges that are saturated in every
maximum multicommodity ﬂow.
Hint: Use complementary slackness conditions.

20.4 Give an example to show that picking all of D gives an Ω(n) factor for
multicut.

20.5 Consider the following growth process. W (t) denotes the weight at time
t. Assume that the initial weight is W (0) = W0 , and that at each point the
rate of growth is proportional to the current weight, i.e.,

      dW (t) = εW (t)dt.

Give the function W (t). Next, assume that W0 = F/k and that W (1/2) =
F + F/k. What is ε?
Hint: W (t) = W0 eεt and ε = 2 ln(k + 1).

20.6 This exercise justiﬁes the choice of wt(s1 ), which was ﬁxed to be
F/k. Suppose we ﬁx it at W0 . Clearly, ε is inversely related to W0 (see
Lemma 20.3). However, the approximation factor of the algorithm is given
by ε(F + kW0 ) (see Lemma 20.6). For what value of W0 is the approximation
factor minimized?

20.7 Consider the following problem, which has applications in VLSI design.

Problem 20.15 (Graph bipartization by edge deletion) Given an
edge weighted undirected graph G = (V, E), remove a minimum weight set
of edges to leave a bipartite graph.
    Obtain an O(log n) factor approximation algorithm for this problem by
reducing it to Problem 20.11.

20.8 (Even, Naor, Schieber, and Rao [75]) This exercise develops an
O(log2 n) factor algorithm for the following problem.
Problem 20.16 (Minimum length linear arrangement) Given an
undirected graph G = (V, E), ﬁnd a numbering of its vertices from 1 to
n, h : V → {1, . . . , n}, so as to minimize
                                                                  20.6   Notes     179
       
               |h(u) − h(v)|.
     (u,v)∈E


1. Show that the following is an LP-relaxation of this problem. This LP
   has a variable de for each edge e ∈ E, which we will interpret as a
   distance label. For any distance label assignment d to the edges of G,
   deﬁne distd (u, v) to be the length of the shortest path from u to v in G.
   Give a polynomial time separation oracle for this LP, thereby showing
   that it can be solved in polynomial time.
                        
         minimize             de                                                 (20.3)
                        e∈E
                                              1
                              distd (u, v) ≥     (|S|2 − 1),   S ⊆ V, v ∈ S
                                               4
                        u∈S
                        de ≥ 0,                                e∈E

2. Let d be an optimal solution to LP (20.3). Show that for any S ⊆ V, v ∈ S
   there is a vertex u ∈ S such that distd (u, v) ≥ (|S| + 1)/4.
3. For S ⊆ V , deﬁne wt(S) to be the sum of distance labels of all edges
   having both endpoints in S. Also, deﬁne c(S, S) to be the number of edges
   in the cut (S, S). Give a region growing process similar to that described
   in Section 20.2.1 that ﬁnds a cut (S, S) in G with wt(S) ≤ wt(S) such
   that c(S, S) is O(wt(S)(log n)/n).
4. Show that a divide-and-conquer algorithm that recursively ﬁnds a num-
   bering for vertices in S from 1 to |S|, and a numbering for vertices in S
   from |S| + 1 to n achieves an approximation guarantee of O(log2 n).
   Hint: Assuming each edge in the cut (S, S) is of length n − 1, write a
   suitable recurrence for the cost incurred by the algorithm.


20.6 Notes

Theorem 20.7 and its corollary are due to Garg, Vazirani, and Yannakakis
[97]. Problem 20.11 was introduced in Klein, Rao, Agrawal, and Ravi [173].
For showing existence of expanders via a probabilistic argument, see Pinsker
[220].
