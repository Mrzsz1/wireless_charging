---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-05"
chapter_number: 5
chapter_title: "k-Center"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 65
source_page_end: 71
printed_page_start: 47
printed_page_end: 53
part_ids: ["approximation-algorithms-ch-05-part-006"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# k-Center

5 k-Center




Consider the following application. Given a set of cities, with intercity dis-
tances speciﬁed, pick k cities for locating warehouses in so as to minimize the
maximum distance of a city from its closest warehouse. We will study this
problem, called the k-center problem, and its weighted version, under the
restriction that the edge costs satisfy the triangle inequality. Without this
restriction, the k-center problem cannot be approximated within factor α(n),
for any computable function α(n), assuming P = NP (see Exercise 5.1).
    We will introduce the algorithmic technique of parametric pruning for
solving this problem. In Chapter 17 we will use this technique in a linear
programming setting.
Problem 5.1 (Metric k-center) Let G = (V, E) be a complete undirected
graph with edge costs satisfying the triangle inequality, and k be a positive
integer. For any set S ⊆ V and vertex v ∈ V , deﬁne connect(v, S) to be the
cost of the cheapest edge from v to a vertex in S. The problem is to ﬁnd a
set S ⊆ V , with |S| = k, so as to minimize maxv {connect(v, S)}.


5.1 Parametric pruning applied to metric k-center
If we know the cost of an optimal solution, we may be able to prune away
irrelevant parts of the input and thereby simplify the search for a good so-
lution. However, as stated in Chapter 1, computing the cost of an optimal
solution is precisely the diﬃcult core of NP-hard NP-optimization problems.
The technique of parametric pruning gets around this diﬃculty as follows.
A parameter t is chosen, which can be viewed as a “guess” on the cost of
an optimal solution. For each value of t, the given instance I is pruned by
removing parts that will not be used in any solution of cost > t. Denote the
pruned instance by I(t). The algorithm consists of two steps. In the ﬁrst step,
the family of instances I(t) is used for computing a lower bound on OPT, say
t∗ . In the second step, a solution is found in instance I(α · t∗ ), for a suitable
choice of α.
     A restatement of the k-center problem shows how parametric pruning
applies naturally to it. Sort the edges of G in nondecreasing order of cost,
i.e., cost(e1 ) ≤ cost(e2 ) ≤ . . . ≤ cost(em ), and let Gi = (V, Ei ), where Ei =
48       5   k-Center

{e1 , e2 , . . . , ei }. A dominating set in an undirected graph H = (U, F ) is a
subset S ⊆ U such that every vertex in U − S is adjacent to a vertex in
S. Let dom(H) denote the size of a minimum cardinality dominating set in
H. Computing dom(H) is NP-hard. The k-center problem is equivalent to
ﬁnding the smallest index i such that Gi has a dominating set of size at most
k, i.e., Gi contains k stars spanning all vertices, where a star is the graph
K1,p , with p ≥ 1. If i∗ is the smallest such index, then cost(ei∗ ) is the cost
of an optimal k-center. We will denoted this by OPT. We will work with the
family of graphs G1 , . . . , Gm .
    Deﬁne the square of graph H to be the graph containing an edge (u, v)
whenever H has a path of length at most two between u and v, u = v. We
will denote it by H 2 . The following structural result gives a method for lower
bounding OPT.

Lemma 5.2 Given a graph H, let I be an independent set in H 2 . Then,
|I| ≤ dom(H).

Proof: Let D be a minimum dominating set in H. Then, H contains |D|
stars spanning all vertices. Since each of these stars will be a clique in H 2 ,
H 2 contains |D| cliques spanning all vertices. Clearly, I can pick at most one
vertex from each clique, and the lemma follows.                              ✷
     The k-center algorithm is:


 Algorithm 5.3 (Metric k-center)
     1. Construct G21 , G22 , . . . , G2m .
     2. Compute a maximal independent set, Mi , in each graph G2i .
     3. Find the smallest index i such that |Mi | ≤ k, say j.
     4. Return Mj .


     The lower bound on which this algorithm is based is:

Lemma 5.4 For j as deﬁned in the algorithm, cost(ej ) ≤ OPT.

Proof: For every i < j we have that |Mi | > k. Now, by Lemma 5.2,
dom(Gi ) > k, and so i∗ > i. Hence, j ≤ i∗ .                   ✷

Theorem 5.5 Algorithm 5.3 achieves an approximation factor of 2 for the
metric k-center problem.

Proof: The key observation is that a maximal independent set, I, in a graph
is also a dominating set (for, if some vertex v is not dominated by I, then
I ∪{v} must also be an independent set, contradicting I’s maximality). Thus,
                      5.1   Parametric pruning applied to metric k-center      49

there exist stars in G2j , centered on the vertices of Mj , covering all vertices.
By the triangle inequality, each edge used in constructing these stars has cost
at most 2 · cost(ej ). The theorem follows from Lemma 5.4.                      ✷

Example 5.6 A tight example for the previous algorithm is given by a wheel
graph on n + 1 vertices, where all edges incident to the center vertex have
cost 1, and the rest of the edges have cost 2:

                                       r✭
                                       ✭     ✭✭
                                             ✟  ✟r❜
                                                       ❜
                                                       
                                      ✁❆✟✟                 ❜
                                                           
                                     ✟  ❆                    ❜r
                                  ✟✁ ❆                      ✱❇
                               ✟
                               r❍ ✁                         ✱ ❇
                              ✆❉ ❍✁❍       ❆             ✱
                             ✆ ❉✁      ❍❍❆  ✱                 ❇
                            ✆ ✁❉            ❍❆
                                             ✥ ✱
                                               
                                               r                 ❇ r
                                 ✥   ✥ ✥✥ ✪❡
                         ✆r✁✥✥❉
                            ❧ ❉
                            ❚             ✪          ❡
                               ❚❧❉ ❧ ✪                   ❡       .
                                 ❚❉ ✪ ❧                    ❡ ..
                                    r✪
                                  ❚❉❛❛❛❧                    ❡r
                                          ❛❧          ✦   ✦✦
                                            ❧❛✦r

(Here, thin edges have cost 1 and thick edges have cost 2; not all edges of
cost 2 are shown.)
   For k = 1, the optimal solution is the center of the wheel, and OPT = 1.
The algorithm will compute index j = n. Now, G2n is a clique and, if a
peripheral vertex is chosen as the maximal independent set, then the cost of
the solution found is 2.                                                  ✷
   Next, we will show that 2 is essentially the best approximation factor
achievable for the metric k-center problem.

Theorem 5.7 Assuming P = NP, there is no polynomial time algorithm
achieving a factor of 2 − ε, ε > 0, for the metric k-center problem.

Proof: We will show that such an algorithm can solve the dominating set
problem in polynomial time. The idea is similar to that of Theorem 3.6 and
involves giving a reduction from the dominating set problem to metric k-
center. Let G = (V, E), k be an instance of the dominating set problem.
Construct a complete graph G = (V, E  ) with edge costs given by

                     1, if (u, v) ∈ E,
      cost(u, v) =
                     2, if (u, v) ∈ E.

Clearly, G satisﬁes the triangle inequality. This reduction satisﬁes the con-
ditions:
• if dom(G) ≤ k, then G has a k-center of cost 1, and
50     5    k-Center

• if dom(G) > k, then the optimum cost of a k-center in G is 2.
In the ﬁrst case, when run on G , the (2 − ε)-approximation algorithm must
give a solution of cost 1, since it cannot use an edge of cost 2. Hence, using
this algorithm, we can distinguish between the two possibilities, thus solving
the dominating set problem.                                                 ✷



5.2 The weighted version
We will use the technique of parametric pruning to obtain a factor 3 ap-
proximation algorithm for the following generalization of the metric k-center
problem.
Problem 5.8 (Metric weighted k-center) In addition to a cost function
on edges, we are given a weight function on vertices, w : V → R+ , and a
bound W ∈ R+ . The problem is to pick S ⊆ V of total weight at most W ,
minimizing the same objective function as before, i.e.,

      max{min{cost(u, v)}}.
      v∈V   u∈S


    Let wdom(G) denote the weight of a minimum weight dominating set in
G. Then, with respect to the graphs Gi deﬁned above, we need to ﬁnd the
smallest index i such that wdom(Gi ) ≤ W . If i∗ is this index, then the cost
of the optimal solution is OPT = cost(ei∗ ).
    Given a vertex weighted graph H, let I be an independent set in H 2 . For
each u ∈ I, let s(u) denote a lightest neighbor of u in H, where u is also
considered a neighbor of itself. (Notice that the neighbor is picked in H and
not in H 2 .) Let S = {s(u)| u ∈ I}. The following fact, analogous to Lemma
5.2, will be used to derive a lower bound on OPT:
Lemma 5.9              w(S) ≤ wdom(H).

Proof: Let D be a minimum weight dominating set of H. Then there exists
a set of disjoint stars in H, centered on the vertices of D and covering all the
vertices. Since each of these stars becomes a clique in H 2 , I can pick at most
one vertex from each of them. Thus, each vertex in I has the center of the
corresponding star available as a neighbor in H. Hence, w(S) ≤ w(D).          ✷
    The algorithm is given below. In it, si (u) will denote a lightest neighbor
of u in Gi ; for this deﬁnition, u will also be considered a neighbor of itself.
                                                                                         5.2          The weighted version                 51


 Algorithm 5.10 (Metric weighted k-center)
  1. Construct G21 , G22 , . . . , G2m .
  2. Compute a maximal independent set, Mi , in each graph G2i .
  3. Compute Si = {si (u)| u ∈ Mi }.
  4. Find the minimum index i such that w(Si ) ≤ W , say j.
  5. Return Sj .


Theorem 5.11 Algorithm 5.10 achieves an approximation factor of 3 for
the weighted k-center problem.

Proof: By Lemma 5.9, cost(ej ) is a lower bound on OPT; the argument is
identical to that in Lemma 5.4 and is omitted here. Since Mj is a dominating
set in G2j , we can cover V with stars of G2j centered in vertices of Mj . By the
triangle inequality these stars use edges of cost at most 2 · cost(ej ).

                                                         ..r.
                                                            .❏
                                                            ❏...
                 rP                                            ...
                  ..P
                    .... P                                       ...❏                                                        ✦  ...r
                        .... PP                                     ... ❏                        ≤ 2c(ei ) ✦✦
                                                                                                               ✦         .......
                            ....                                                                                        .
                                .... PPP ..... ❏                                                    ✦✦ .......
                                    ....                        P..P u ✦✦                                         ...
                                        ....                            .. ✭  P
                                                                              ✭❏r✦                            ....
                                         ✭
                                            ...✭
                                               ...   ✭✭✭.....                                          .......
                             ✭     ✭              .....                    .
                                                                           ... ≤ c(ei ) ....≤ 3c(e )  .
                  ......✭
                 r✭                                    ....
                        .........                             .... .....                          ...
                                                                                              ....
                                                                                                                      i
                                 .........                                                   .
                                          .........                  .... ...              ..
                                                   ......... .... .. ......
                                                               ......... ... .. ...
                                                                          ............r...
                                                                                      si (u)
Each star center is adjacent to a vertex in Sj , using an edge of cost at most
cost(ej ). Move each of the centers to the adjacent vertex in Sj and redeﬁne
the stars. Again, by the triangle inequality, the largest edge cost used in
constructing the ﬁnal stars is at most 3 · cost(ej ).                       ✷

Example 5.12 A tight example is provided by the following graph on n + 4
vertices. Vertex weights and edge costs are as marked; all missing edges have
a cost given by the shortest path.
              ∞ r
                 ❅
                   ❅ 1+ε
                 r   ❅
              ∞ PP ❅
                     PP a               b          c            d
                          ❅
                          P
                          ❅
                          Pr
                        1+ε
                                  1     r     1     r    1       r
                ..                           1                              2                              2                           2
                 .
                              1+ε

             ∞ r
52     5   k-Center

    It is not diﬃcult to see that for W = 3 the optimum cost of a k-center
is 1 + ε: a k-center achieving this cost is {a, c}. For any i < n + 3, the
set Si computed by the algorithm will contain a vertex of inﬁnite weight.
Suppose that, for i = n + 3, the algorithm chooses Mn+3 = {b} as a maximal
independent set. Then Sn+3 = {a}, and this is the output of the algorithm.
The cost of this solution is 3.                                         ✷



5.3 Exercises

5.1 Show that if the edge costs do not satisfy the triangle inequality, then
the k-center problem cannot be approximated within factor α(n) for any
computable function α(n).
Hint: Put together ideas from Theorems 3.6 and 5.7.

5.2 Consider Step 2 of Algorithm 5.3, in which a maximal independent set
is found in G2i . Perhaps a more natural choice would have been to ﬁnd a
minimal dominating set. Modify Algorithm 5.3 so that Mi is picked to be
a minimal dominating set in G2i . Show that this modiﬁed algorithm does
not achieve an approximation guarantee of 2 for the k-center problem. What
approximation factor can you establish for this algorithm?
Hint: With this modiﬁcation, the lower bounding method does not work,
since Lemma 5.2 does not hold if I is picked to be a minimal dominating set
in H 2 .

5.3 (Gonzalez [111]) Consider the following problem.
Problem 5.13 (Metric k-cluster) Let G = (V, E) be a complete undi-
rected graph with edge costs satisfying the triangle inequality, and let k be
a positive integer. The problem is to partition V into sets V1 , . . . , Vk so as
to minimize the costliest edge between two vertices in the same set, i.e.,
minimize

           max        cost(u, v).
      1≤i≤k, u,v∈Vi


 1. Give a factor 2 approximation algorithm for this problem, together with
    a tight example.
 2. Show that this problem cannot be approximated within a factor of 2 − ε,
    for any ε > 0, unless P = NP.


5.4 (Khuller, Pless, and Sussmann [169]) The fault-tolerant version of the
metric k-center problem has an additional input, α ≤ k, which speciﬁes the
                                                             5.4   Notes     53

number of centers that each city should be connected to. The problem again
is to pick k centers so that the length of the longest edge used is minimized.
    A set S ⊆ V in an undirected graph H = (V, E) is an α-dominating set
if each vertex v ∈ V is adjacent to at least α vertices in S (assuming that
a vertex is adjacent to itself). Let domα (H) denote the size of a minimum
cardinality α-dominating set in H.

 1. Let I be an independent set in H 2 . Show that α|I| ≤ domα (H).
 2. Give a factor 3 approximation algorithm for the fault-tolerant k-center
    problem.
    Hint: Compute a maximal independent set Mi in G2i , for 1 ≤ i ≤ m.
    Find the smallest index i such that |Mi | ≤  αk , and moreover, the degree
    of each vertex of Mi in Gi is ≥ α − 1.


5.5 (Khuller, Pless, and Sussmann [169]) Consider a modiﬁcation of the
problem of Exercise 5.4 in which vertices of S have no connectivity require-
ments and only vertices of V −S have connectivity requirements. Each vertex
of V − S needs to be connected to α vertices in S. The object again is to pick
S, |S| = k, so that the length of the longest edge used is minimized.
    The algorithm for this problem works on each graph Gi . It starts with
Si = ∅. Vertex v ∈ V − Si is said to be j-connected if it is adjacent to j
vertices in Si , using edges of G2i . While there is a vertex v ∈ V − Si that is
not k-connected, pick the vertex with minimum connectivity, and include it
in Si . Finally, ﬁnd the minimum index i such that |Si | ≤ k, say l. Output Sl .
Prove that this is a factor 2 approximation algorithm.


5.4 Notes
Both k-center algorithms presented in this chapter are due to Hochbaum and
Shmoys [127], and Theorem 5.7 is due to Hsu and Nemhauser [132].
