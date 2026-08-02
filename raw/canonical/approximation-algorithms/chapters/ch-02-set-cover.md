---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-02"
chapter_number: 2
chapter_title: "Set Cover"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 33
source_page_end: 44
printed_page_start: 15
printed_page_end: 26
part_ids: ["approximation-algorithms-ch-02-part-003"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Set Cover

2 Set Cover




The set cover problem plays the same role in approximation algorithms that
the maximum matching problem played in exact algorithms – as a problem
whose study led to the development of fundamental techniques for the entire
ﬁeld. For our purpose this problem is particularly useful, since it oﬀers a very
simple setting in which many of the basic algorithm design techniques can be
explained with great ease. In this chapter, we will cover two combinatorial
techniques: the fundamental greedy technique and the technique of layering.
In Part II we will explain both the basic LP-based techniques of rounding
and the primal–dual schema using this problem.
    Among the ﬁrst strategies one tries when designing an algorithm for an
optimization problem is some form of the greedy strategy. Even if this strat-
egy does not work for a speciﬁc problem, proving this via a counterexample
can provide crucial insights into the structure of the problem.
    Perhaps the most natural use of this strategy in approximation algorithms
is to the set cover problem. Besides the greedy set cover algorithm, we will
also present the technique of layering in this chapter. Because of its generality,
the set cover problem has wide applicability, sometimes even in unexpected
ways. In this chapter we will illustrate such an application – to the shortest
superstring problem (see Chapter 7 for an improved algorithm for the latter
problem).
Problem 2.1 (Set cover) Given a universe U of n elements, a collection
of subsets of U , S = {S1 , . . . , Sk }, and a cost function c : S → Q+ , ﬁnd a
minimum cost subcollection of S that covers all elements of U .
    Deﬁne the frequency of an element to be the number of sets it is in.
A useful parameter is the frequency of the most frequent element. Let us
denote this by f . The various approximation algorithms for set cover achieve
one of two factors: O(log n) or f . Clearly, neither dominates the other in all
instances. The special case of set cover with f = 2 is essentially the vertex
cover problem (see Exercise 2.7), for which we gave a factor 2 approximation
algorithm in Chapter 1.
16        2   Set Cover

2.1 The greedy algorithm
The greedy strategy applies naturally to the set cover problem: iteratively
pick the most cost-eﬀective set and remove the covered elements, until all
elements are covered. Let C be the set of elements already covered at the be-
ginning of an iteration. During this iteration, deﬁne the cost-eﬀectiveness of a
set S to be the average cost at which it covers new elements, i.e., c(S)/|S − C|.
Deﬁne the price of an element to be the average cost at which it is covered.
Equivalently, when a set S is picked, we can think of its cost being distributed
equally among the new elements covered, to set their prices.

 Algorithm 2.2 (Greedy set cover algorithm)
     1. C ← ∅
     2. While C = U do
         Find the most cost-eﬀective set in the current iteration, say S.
         Let α = cost (S)
                  |S−C| , i.e., the cost-eﬀectiveness of S.
         Pick S, and for each e ∈ S − C, set price(e) = α.
         C ← C ∪ S.
     3. Output the picked sets.


   Number the elements of U in the order in which they were covered by the
algorithm, resolving ties arbitrarily. Let e1 , . . . , en be this numbering.
Lemma 2.3 For each k ∈ {1, . . . , n}, price(ek ) ≤ OPT/(n − k + 1).

Proof: In any iteration, the leftover sets of the optimal solution can cover
the remaining elements at a cost of at most OPT. Therefore, among these
sets, there must be one having cost-eﬀectiveness of at most OPT/|C|. In the
iteration in which element ek was covered, C contained at least n − k + 1
elements. Since ek was covered by the most cost-eﬀective set in this iteration,
it follows that
                       OPT     OPT
        price(ek ) ≤        ≤       .
                        |C|   n−k+1
                                                                                ✷
From Lemma 2.3 we immediately obtain:
Theorem 2.4 The greedy algorithm is an Hn factor approximation algo-
rithm for the minimum set cover problem, where Hn = 1 + 12 + · · · + n1 .

Proof: Since the cost of each set picked is distributed among     nthe new ele-
ments covered, the total cost of the
                                    set cover picked
                                                     is equal to  k=1 price(ek ).
By Lemma 2.3, this is at most 1 + 12 + · · · + n1 · OPT.                       ✷
                                                           2.2   Layering   17

Example 2.5 The following is a tight example for Algorithm 2.2:



                                         ...         1+ε


                       1/n    1/(n-1)           1
When run on this instance the greedy algorithm outputs the cover consisting
of the n singleton sets, since in each iteration some singleton is the most
cost-eﬀective set. Thus, the algorithm outputs a cover of cost

      1   1
        +   + · · · + 1 = Hn .
      n n−1

On the other hand, the optimal cover has a cost of 1 + ε.                    ✷
    Surprisingly enough, for the minimum set cover problem the obvious al-
gorithm given above is essentially the best one can hope for; see Sections 29.7
and 29.9.
    In Chapter 1 we pointed out that ﬁnding a good lower bound on OPT
is a basic starting point in the design of an approximation algorithm for a
minimization problem. At this point the reader may be wondering whether
there is any truth to this claim. We will show in Section 13.1 that the correct
way to view the greedy set cover algorithm is in the setting of the LP-duality
theory – this will not only provide the lower bound on which this algorithm
is based, but will also help obtain algorithms for several generalizations of
this problem.


2.2 Layering

The algorithm design technique of layering is also best introduced via set
cover. We note, however, that this is not a very widely applicable technique.
We will give a factor 2 approximation algorithm for vertex cover, assuming
arbitrary weights, and leave the problem of generalizing this to a factor f
approximation algorithm for set cover, where f is the frequency of the most
frequent element (see Exercise 2.13).
    The idea in layering is to decompose the given weight function on vertices
into convenient functions, called degree-weighted, on a nested sequence of
subgraphs of G. For degree-weighted functions, we will show that we will be
within twice the optimal even if we pick all vertices in the cover.
    Let us introduce some notation. Let w : V → Q+ be the function assigning
weights to the vertices of the given graph G = (V, E). We will say that a
function assigning vertex weights is degree-weighted if there is a constant
18      2   Set Cover

c > 0 such that the weight of each vertex v ∈ V is c · deg(v). The signiﬁcance
of such a weight function is captured in:
Lemma 2.6 Let w : V → Q+ be a degree-weighted function. Then w(V ) ≤
2 · OPT.

Proof: Let c be the constant such that w(v) = c · deg(v), and let U be an
optimal vertex cover in G. Since U covers all the edges,
      
            deg(v) ≥ |E|.
      v∈U

                                          
Therefore, w(U ) ≥ c|E|. Now, since            v∈V deg(v) = 2|E|, w(V ) = 2c|E|. The
lemma follows.                                                                    ✷
    Let us deﬁne the largest degree-weighted function in w as follows: remove
all degree zero vertices from the graph, and over the remaining vertices, com-
pute c = min{w(v)/deg(v)}. Then, t(v) = c · deg(v) is the desired function.
Deﬁne w (v) = w(v) − t(v) to be the residual weight function.
    The algorithm for decomposing w into degree-weighted functions is as
follows. Let G0 = G. Remove degree zero vertices from G0 , say this set is D0 ,
and compute the largest degree-weighted function in w. Let W0 be vertices of
zero residual weight; these vertices are included in the vertex cover. Let G1 be
the graph induced on V −(D0 ∪W0 ). Now, the entire process is repeated on G1
w.r.t. the residual weight function. The process terminates when all vertices
are of degree zero; let Gk denote this graph. The process is schematically
shown in the following ﬁgure.
                                        Gk       Dk


                                 Gk-1   Wk-1           Dk-1

                                                  ..
                                                   .

                   G1       W1                                D1


             G0    W0                                                D0


Let t0 , ..., tk−1 be the degree-weighted functions deﬁned on graphs G0 , ..., Gk−1 .
The vertex cover chosen is C = W0 ∪. . .∪Wk−1 . Clearly, V −C = D0 ∪. . .∪Dk .
Theorem 2.7 The layer algorithm achieves an approximation guarantee of
factor 2 for the vertex cover problem, assuming arbitrary vertex weights.

Proof: We need to show that set C is a vertex cover for G and w(C) ≤
2 · OPT. Assume, for contradiction, that C is not a vertex cover for G. Then
                                   2.3       Application to shortest superstring   19

there must be an edge (u, v) with u ∈ Di and v ∈ Dj , for some i, j. Assume
i ≤ j. Therefore, (u, v) is present in Gi , contradicting the fact that u is a
degree zero vertex.
   Let C ∗ be an optimal vertex cover. For proving the second part, consider
a vertex v ∈ C. If v ∈ Wj , its weight can be decomposed as
              
     w(v) =         ti (v).
              i≤j


Next, consider a vertex v ∈ V − C. If v ∈ Dj , a lower bound on its weight is
given by
              
     w(v) ≥         ti (v).
              i<j


     The important observation is that in each layer i, C ∗ ∩ Gi is a vertex
cover for Gi , since Gi is a vertex-induced graph. Therefore, by Lemma 2.6,
ti (C ∩ Gi ) ≤ 2 · ti (C ∗ ∩ Gi ). By the decomposition of weights given above,
we get

              k−1
                                      k−1
                                       
     w(C) =         ti (C ∩ Gi ) ≤ 2         ti (C ∗ ∩ Gi ) ≤ 2 · w(C ∗ ).
              i=0                      i=0

                                                                                   ✷

Example 2.8 A tight example is provided by the family of complete bi-
partite graphs, Kn,n , with all vertices of unit weight. The layering algorithm
will pick all 2n vertices of Kn,n in the cover, whereas the optimal cover picks
only one side of the bipartition.                                             ✷



2.3 Application to shortest superstring
The following algorithm is given primarily to demonstrate the wide applica-
bility of set cover. A constant factor approximation algorithm for shortest
superstring will be given in Chapter 7.
    Let us ﬁrst provide motivation for this problem. The human DNA can
be viewed as a very long string over a four-letter alphabet. Scientists are
attempting to decipher this string. Since it is very long, several overlapping
short segments of this string are ﬁrst deciphered. Of course, the locations of
these segments on the original DNA are not known. It is hypothesized that
the shortest string which contains these segments as substrings is a good
approximation to the original DNA string.
20      2   Set Cover

Problem 2.9 (Shortest superstring) Given a ﬁnite alphabet Σ, and a
set of n strings, S = {s1 , . . . , sn } ⊆ Σ + , ﬁnd a shortest string s that contains
each si as a substring. Without loss of generality, we may assume that no
string si is a substring of another string sj , j = i.
    This problem is NP-hard. Perhaps the ﬁrst algorithm that comes to mind
for ﬁnding a short superstring is the following greedy algorithm. Deﬁne the
overlap of two strings s, t ∈ Σ ∗ as the maximum length of a suﬃx of s that is
also a preﬁx of t. The algorithm maintains a set of strings T ; initially T = S.
At each step, the algorithm selects from T two strings that have maximum
overlap and replaces them with the string obtained by overlapping them as
much as possible. After n − 1 steps, T will contain a single string. Clearly,
this string contains each si as a substring. This algorithm is conjectured to
have an approximation factor of 2. To see that the approximation factor of
this algorithm is no better than 2, consider an input consisting of 3 strings:
abk , bk c, and bk+1 . If the ﬁrst two strings are selected in the ﬁrst iteration,
the greedy algorithm produces the string abk cbk+1 . This is almost twice as
long as the shortest superstring, abk+1 c.
    We will obtain a 2Hn factor approximation algorithm, using the greedy
set cover algorithm. The set cover instance, denoted by S, is constructed as
follows. For si , sj ∈ S and k > 0, if the last k symbols of si are the same as
the ﬁrst k symbols of sj , let σijk be the string obtained by overlapping these
k positions of si and sj :
                                  ✛ k ✲
                        si
                                                            sj
                                        
                                         σijk

Let M be the set that consists of the strings σijk , for all valid choices of
i, j, k. For a string π ∈ Σ + , deﬁne set(π) = {s ∈ S | s is a substring of π}.
The universal set of the set cover instance S is S, and the speciﬁed subsets
of S are set(π), for each string π ∈ S ∪ I. The cost of set(π) is |π|, i.e., the
length of string π.
     Let OPTS and OPT denote the cost of an optimal solution to S and the
length of the shortest superstring of S, respectively. As shown in Lemma 2.11,
OPTS and OPT are within a factor of 2 of each other, and so an approxima-
tion algorithm for set cover can be used to obtain an approximation algorithm
for shortest superstring. The complete algorithm is:
                                    2.3   Application to shortest superstring         21


 Algorithm 2.10 (Shortest superstring via set cover)
  1. Use the greedy set cover algorithm to ﬁnd a cover for the instance S.
     Let set(π1 ), . . . , set(πk ) be the sets picked by this cover.
  2. Concatenate the strings π1 , . . . , πk , in any order.
  3. Output the resulting string, say s.



Lemma 2.11              OPT ≤ OPTS ≤ 2 · OPT.

Proof: Consider an optimal set cover, say {set(πi )|1 ≤ i ≤ l}, and obtain a
string, say s, by concatenating the strings πi , 1 ≤ i ≤ l, in any order. Clearly,
|s| = OPTS . Since each string of S is a substring of some πi , 1 ≤ i ≤ l, it is
also a substring of s. Hence OPTS = |s| ≥ OPT.
    To prove the second inequality, let s be a shortest superstring of s1 , . . . , sn ,
|s| = OPT. It suﬃces to produce some set cover of cost at most 2 · OPT.
    Consider the leftmost occurrence of the strings s1 , . . . , sn in string s. Since
no string among s1 , . . . , sn is a substring of another, these n leftmost occur-
rences start at distinct places in s. For the same reason, they also end at
distinct places. Renumber the n strings in the order in which their leftmost
occurrences start. Again, since no string is a substring of another, this is also
the order in which they end.
  s
                                                                                           ...
       sb1


                      se1

                             sb2


                                             se2

                                                     sb3


                                                                       se3
                                                                             ..
                                                                                  .
      π1
                            π2
                                                    π3
                                                                             ..
                                                                                  .


    We will partition the ordered list of strings s1 , . . . , sn in groups as de-
scribed below. Each group will consist of a contiguous set of strings from this
22       2   Set Cover

list. Let bi and ei denote the index of the ﬁrst and last string in the ith group
(bi = ei is allowed). Thus, b1 = 1. Let e1 be the largest index of a string that
overlaps with s1 (there exists at least one such string, namely s1 itself). In
general, if ei < n we set bi+1 = ei + 1 and denote by ei+1 the largest index
of a string that overlaps with sbi+1 . Eventually, we will get et = n for some
t ≤ n.
     For each pair of strings (sbi , sei ), let ki > 0 be the length of the overlap
between their leftmost occurrences in s (this may be diﬀerent from their
maximum overlap).
                     Let πi = σbi ei ki . Clearly, {set(πi )|1 ≤ i ≤ t} is a solution
for S, of cost i |πi |.
     The critical observation is that πi does not overlap πi+2 . We will prove
this claim for i = 1; the same argument applies to an arbitrary i. Assume, for
contradiction, that π1 overlaps π3 . Then the occurrence of sb3 in s overlaps
the occurrence of se1 . However, sb3 does not overlap sb2 (otherwise, sb3 would
have been put in the second group). This implies that se1 ends later than
sb2 , contradicting the property of endings of strings established earlier.
                               each symbol of s is covered by at most two
     Because of this observation,
of the πi ’s. Hence OPTS ≤ i |πi | ≤ 2 · OPT.                                        ✷
    The size of the universal set in the set cover instance S is n, the number
of strings in the given shortest superstring instance. This fact, Lemma 2.11,
and Theorem 2.4 immediately give the following theorem.
Theorem 2.12 Algorithm 2.10 is a 2Hn factor algorithm for the shortest
superstring problem, where n is the number of strings in the given instance.


2.4 Exercises

2.1 Given an undirected graph G = (V, E), the cardinality maximum cut
problem asks for a partition of V into sets S and S so that the number of
edges running between these sets is maximized. Consider the following greedy
algorithm for this problem. Here v1 and v2 are arbitrary vertices in G, and
for A ⊂ V , d(v, A) denotes the number of edges running between vertex v
and set A.

 Algorithm 2.13
     1. Initialization:
        A ← {v1 }
        B ← {v2 }
     2. For v ∈ V − {v1 , v2 } do:
         if d(v, A) ≥ d(v, B) then B ← B ∪ {v},
         else A ← A ∪ {v}.
     3. Output A and B.
                                                           2.4   Exercises     23

Show that this is a factor 1/2 approximation algorithm and give a tight
example. What is the upper bound on OPT that you are using? Give examples
of graphs for which this upper bound is as bad as twice OPT. Generalize the
problem and the algorithm to weighted graphs.

2.2 Consider the following algorithm for the maximum cut problem, based
on the technique of local search. Given a partition of V into sets, the basic
step of the algorithm, called ﬂip, is that of moving a vertex from one side
of the partition to the other. The following algorithm ﬁnds a locally optimal
solution under the ﬂip operation, i.e., a solution which cannot be improved
by a single ﬂip.
    The algorithm starts with an arbitrary partition of V . While there is a
vertex such that ﬂipping it increases the size of the cut, the algorithm ﬂips
such a vertex. (Observe that a vertex qualiﬁes for a ﬂip if it has more neigh-
bors in its own partition than in the other side.) The algorithm terminates
when no vertex qualiﬁes for a ﬂip. Show that this algorithm terminates in
polynomial time, and achieves an approximation guarantee of 1/2.

2.3 Consider the following generalization of the maximum cut problem.
Problem 2.14 (MAX k-CUT) Given an undirected graph G = (V, E)
with nonnegative edge costs, and an integer k, ﬁnd a partition of V into
sets S1 , . . . , Sk so that the total cost of edges running between these sets is
maximized.
    Give a greedy algorithm for this problem that achieves a factor of (1 − k1 ).
Is the analysis of your algorithm tight?

2.4 Give a greedy algorithm for the following problem achieving an approx-
imation guarantee of factor 1/4.
Problem 2.15 (Maximum directed cut) Given a directed graph G =
(V, E) with nonnegative edge costs, ﬁnd a subset S ⊂ V so as to maximize
the total cost of edges out of S, i.e., cost({(u → v) | u ∈ S and v ∈ S}).

2.5 (N. Vishnoi) Use the algorithm in Exercise 2.2 and the fact that the
vertex cover problem is polynomial time solvable for bipartite graphs to give
a factor log2 ∆ algorithm for vertex cover, where ∆ is the degree of the
vertex having highest degree.
Hint: Let H denote the subgraph consisting of edges in the maximum
cut found by Algorithm 2.13. Clearly, H is bipartite, and for any vertex v,
degH (v) ≥ (1/2)degG (v).

2.6 (Wigderson [257]) Consider the following problem.
Problem 2.16 (Vertex coloring) Given an undirected graph G = (V, E),
color its vertices with the minimum number of colors so that the two end-
points of each edge receive distinct colors.
24       2   Set Cover

 1. Give a greedy algorithm for coloring G with ∆ + 1 colors, where ∆ is the
    maximum degree of a vertex in G.                            √
 2. Give an algorithm for coloring a 3-colorable graph with O( n) colors.
    Hint: For any vertex v, the induced subgraph on its neighbors,  √ N (v),
    is bipartite, and hence optimally colorable. If v has degree > n, color
    v ∪√N (v) using 3 distinct colors. Continue until every vertex has degree
    ≤ n. Then use the algorithm in the ﬁrst part.


2.7 Let 2SC denote the restriction of set cover to instances having f = 2.
Show that 2SC is equivalent to the vertex cover problem, with arbitrary costs,
under approximation factor preserving reductions.

2.8 Prove that Algorithm 2.2 achieves an approximation factor of Hk , where
k is the cardinality of the largest speciﬁed subset of U .

2.9 Give a greedy algorithm that achieves an approximation guarantee of Hn
for set multicover, which is a generalization of set cover in which an integral
coverage requirement is also speciﬁed for each element and sets can be picked
multiple numbers of times to satisfy all coverage requirements. Assume that
the cost of picking α copies of set Si is α · cost(Si ).

2.10 By giving an appropriate tight example, show that the analysis of
Algorithm 2.2 cannot be improved even for the cardinality set cover problem,
i.e., if all speciﬁed sets have unit cost.
Hint: Consider running the greedy algorithm on a vertex cover instance.

2.11 Consider the following algorithm for the weighted vertex cover problem.
For each vertex v, t(v) is initialized to its weight, and when t(v) drops to 0,
v is picked in the cover. c(e) is the amount charged to edge e.


 Algorithm 2.17
     1. Initialization:
        C←∅
        ∀v ∈ V , t(v) ← w(v)
        ∀e ∈ E, c(e) ← 0
     2. While C is not a vertex cover do:
         Pick an uncovered edge, say (u, v). Let m = min(t(u), t(v)).
         t(u) ← t(u) − m
         t(v) ← t(v) − m
         c(u, v) ← m
         Include in C all vertices having t(v) = 0.
     3. Output C.
                                                             2.4   Exercises     25

Show that this is a factor 2 approximation algorithm.
Hint: Show that the total amount charged to edges is a lower bound on OPT
and that the weight of cover C is at most twice the total amount charged to
edges.

2.12 Consider the layering algorithm for vertex cover. Another weight func-
tion for which we have a factor 2 approximation algorithm is the constant
function – by simply using the factor 2 algorithm for the cardinality vertex
cover problem. Can layering be made to work by using this function instead
of the degree-weighted function?

2.13 Use layering to get a factor f approximation algorithm for set cover,
where f is the frequency of the most frequent element. Provide a tight ex-
ample for this algorithm.

2.14 A tournament is a directed graph G = (V, E), such that for each pair of
vertices, u, v ∈ V , exactly one of (u, v) and (v, u) is in E. A feedback vertex set
for G is a subset of the vertices of G whose removal leaves an acyclic graph.
Give a factor 3 algorithm for the problem of ﬁnding a minimum feedback
vertex set in a directed graph.
Hint: Show that it is suﬃcient to “kill” all length 3 cycles. Use the factor
f set cover algorithm.

2.15 (Hochbaum [125]) Consider the following problem.
Problem 2.18 (Maximum coverage) Given a universal set U of n el-
ements, with nonnegative weights speciﬁed, a collection of subsets of U ,
S1 , . . . , Sl , and an integer k, pick k sets so as to maximize the weight of
elements covered.
     Show that the obvious algorithm, of greedily picking the best set in each
iteration until k sets are picked, achieves an approximation factor of
             k
            1       1
      1− 1−      >1− .
            k       e

2.16 Using set cover, obtain approximation algorithms for the following
variants of the shortest superstring problem (here sR is the reverse of string
s):
 1. Find the shortest string that contains, for each string si ∈ S, both si and
    sR
     i as substrings.
    Hint: The universal set for the set cover instance will contain 2n ele-
    ments, si and sR
                   i , for 1 ≤ i ≤ n.
 2. Find the shortest string that contains, for each string si ∈ S, either si or
    sR
     i as a substring.
    Hint: Deﬁne set(π) = {s ∈ S | s or sR is a substring of π}. Choose the
    strings π appropriately.
26     2   Set Cover

2.5 Notes
Algorithm 2.2 is due to Johnson [150], Lovász [192], and Chvátal [48]. The
hardness result for set cover, showing that this algorithm is essentially the
best possible, is due to Feige [80], improving on the result of Lund and Yan-
nakakis [199]. The application to shortest superstring is due to Li [187].
