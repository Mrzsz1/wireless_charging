---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-21"
chapter_number: 21
chapter_title: "Sparsest Cut"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 198
source_page_end: 215
printed_page_start: 180
printed_page_end: 197
part_ids: ["approximation-algorithms-ch-21-part-022"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Sparsest Cut

21 Sparsest Cut




In this chapter we will obtain an approximation algorithm for the sparsest cut
problem using an interesting LP-rounding procedure that employs results on
low distortion embeddings of metrics in 1 spaces. As mentioned in Chapter
20, we will get as a corollary an approximate max-ﬂow min-cut theorem for
the demands version of multicommodity ﬂow. Approximation algorithms for
several other important problems will also follow.


21.1 Demands multicommodity ﬂow

Problem 21.1 (Demands multicommodity ﬂow) Let G = (V, E) be
an undirected graph with a nonnegative capacity ce for each edge e ∈ E.
Let {(s1 , t1 ), . . . , (sk , tk )} be a speciﬁed set of pairs of vertices, where each
pair is distinct, but vertices in diﬀerent pairs are not required to be distinct.
A separate commodity is deﬁned for each (si , ti ) pair; for convenience, we
will think of si as the source and ti as the sink of this commodity. For each
commodity i, a nonnegative demand, dem(i), is also speciﬁed. The objective
is to maximize f , called throughput, such that for each commodity i, f ·
dem(i) units of this commodity can be routed simultaneously, subject to ﬂow
conservation and capacity constraints, i.e., each commodity must satisfy ﬂow
conservation at each vertex other than its own source and sink, and the sum
of ﬂows routed through an edge, in both directions combined, should not
exceed the capacity of this edge. We will denote the optimal throughput by
f ∗.
     Consider a cut (S, S) in G. Let c(S) denote the capacity of edges in this
cut and dem(S) denote the total demand separated by this cut, i.e.,
                        
      dem(S) =                         dem(i).
                  i: |{si ,ti }∩S|=1


Clearly, the ratio of these quantities places an upper bound on the through-
put, i.e., f ∗ ≤ c(S) . This motivates:
                dem(S)
Problem 21.2 (Sparsest cut) Let G = (V, E) be an undirected graph
with capacities, source–sink pairs, and demands deﬁned as in Problem 21.1.
                                          21.2   Linear programming formulation           181

The sparsity of cut (S, S) is given by c(S) . The problem is to ﬁnd a cut
                                         dem(S)
of minimum sparsity. We will denote the sparsity of this cut by α∗ .
   Among all cuts, α∗ puts the most stringent upper bound on f ∗ . Is this
upper bound tight? Example 21.3 shows that it is not. However, minimum
sparsity cannot be arbitrarily larger than maximum throughput; we will show
that their ratio is bounded by O(log k).
Example 21.3 Consider the bipartite graph K3,2 with all edges of unit
capacity and a unit demand between each pair of nonadjacent vertices – a
total of four commodities.
                                                              1/2


                                                              1/2
    1


                                                                                  1/2
1                              1

                                                                                        1/2
    1
                                                        1/2
                                                                1/2




   It is easy to check that a sparsest cut of K3,2 has sparsity 1. This graph
can be viewed as the union of two stars K3,1 (the centers of the stars are
the vertices on the right side of the bipartition), and, as in Example 18.2,
we get the unique way of routing one unit of each of the three commodities
having source and sink on the left side of the bipartition. However, this sat-
urates all edges, making it impossible to route the fourth commodity. Hence,
throughput is strictly smaller than 1.                                      ✷


21.2 Linear programming formulation
We start by giving a linear programming formulation of the problem of max-
imizing throughput, f . Let Pi = {qji } denote the set of all paths between si
and ti . Introduce variable fji to denote the ﬂow of commodity i sent along
path qji . The ﬁrst set of constraints ensures that the demand of each commod-
ity is met (with factor f ), and the second set are edge capacity constraints.

        maximize     f                                                              (21.1)
                     
        subject to           fji ≥ f · dem(i),      i = 1, . . . , k
                      j
                     
                             fji ≤ ce ,             e∈E
                     e∈qji

                     f ≥0
                     fji ≥ 0
182    21   Sparsest Cut

   Deﬁne the graph H with vertex set VH = {si , ti |1 ≤ i ≤ k} and edge set
EH = {(si , ti )|1 ≤ i ≤ k} to be the demand graph. For each edge e = (si , ti )
of H, let dem(e) = dem(i). We will show that the dual to LP (21.1) yields a
metric (V, d) satisfying:
Theorem 21.4 Let f ∗ denote the optimal throughput. Then,
                     
                           ce de
    f = min  e∈G
     ∗
                                  .
         metric d    e∈H dem(e)de

     Let li and de be dual variables associated with the ﬁrst and second set of
inequalities of LP (21.1). We will interpret de ’s as distance label assignments
to the edges of G. The ﬁrst set of inequalities ensures that for each commodity
i, li is upper bounded by the length of any path from si to ti w.r.t. the distance
label assignment.
                    
      minimize              ce de                                                 (21.2)
                    e∈E
                    
      subject to            d e ≥ li ,             qji ∈ Pi , i = 1, . . . , k
                    e∈qji

                    k
                    
                            li dem(i) ≥ 1
                    i=1
                    de ≥ 0,                        e∈E
                    li ≥ 0,                        i = 1, . . . , k

Example 21.5 For the instance given in Example 21.3, the optimal through-
put is f ∗ = 3/4; this corresponds to routing the four commodities as follows:
                             3/8


                                         1/4
                             3/8
                                               1/4 1/4


                                                                      3/8


                                                                            3/8
                     3/8
                               3/8




   The optimal dual solution is: de = 1/8 for each edge e and li = 1/4 for
each commodity i. It would be instructive for the reader to verify feasibility
and optimality of these solutions.                                          ✷
                         21.3   Metrics, cut packings, and 1 -embeddability         183

Claim 21.6 There is an optimal distance label assignment d for the dual
program (21.2) that is a metric on V . Furthermore, for each     commodity i, li =
d(si ,ti ) , and the second inequality holds with equality, i.e., i d(si ,ti ) dem(i) =
1.

Proof: If for some three points u, v, and w, duw > duv + dvw , then decrease
duw to duv + dvw . Since this does not decrease the shortest path between
any si –ti pair, the solution still remains feasible. Moreover, the objective
function value cannot increase by this process. Continuing in this manner,
we will obtain a metric on V .
    Now, the length of a shortest path from si to ti is given by the distance la-
bel d(si ,ti ) . Setting li = d(si ,ti ) does not change the feasibility or the objective
function value of the solution. Finally, if the second inequality holds strictly,
then we can scale down all distance labels without violating feasibility, thus
contradicting the optimality of d.                                                     ✷
    By Claim 21.6, the dual program yields a metric (V, d) that minimizes
           
            e∈G ce de
                        .
         e∈H dem(e)de

By the LP-duality theorem, this equals the optimal throughput. This proves
Theorem 21.4.


21.3 Metrics, cut packings, and 1 -embeddability
In Section 21.3.1, we will deﬁne the notion of a cut packing for a metric and
will show that the question of ﬁnding a good approximation to the sparsest
cut for graph G reduces to that of ﬁnding a “good” cut packing for the metric
obtained in Theorem 21.4. The latter question is reduced, in Section 21.3.2,
to the question of ﬁnding a “good” 1 -embedding for the metric. Eventually,
Section 21.4 deals with ﬁnding the embedding itself.

21.3.1     Cut packings for metrics

Let us think of a metric (V, d) as deﬁning the lengths of edges of the complete
graph on V . Let En denote the set of all edges in the complete graph on n
vertices. Let y be a function assigning nonnegative values to subsets of V ,
i.e., y : 2V → R+ . We will denote the value of y on set S by yS . As before,
let us say that edge  e feels yS if e is in the cut (S, S). The amount of cut
that edge e feels is     S:e∈δ(S) y(S). Function y is called a cut packing for
metric (V,  d) if no edge feels more cut than its length, i.e., for each edge
e ∈ En , S:e∈δ(S) y(S) ≤ de . If this inequality holds with equality for each
edge e ∈ En , then y is said to be an exact cut packing. The reason for the
184     21   Sparsest Cut

name “cut packing” is that equivalently, we can think of y as assigning value
y(S) + y(S) to each cut (S, S).
    As shown below, in general, there may not be an exact cut packing for
metric (V, d). Let us relax this notion by allowing edges to be underpacked up
to a speciﬁed extent. For β ≥ 1, y is said to be a β-approximate cut packing
if the amount of cut felt by anyedge is at least 1/β fraction of its length, i.e.,
for each edge e ∈ En , de /β ≤ S:e∈δ(S) y(S) ≤ de . Clearly, the smaller β is,
the better the cut packing. The following theorem shows the importance of
ﬁnding a good cut packing for (V, d).

Theorem 21.7 Let (V, d) be the metric obtained in Theorem 21.4, and let
y be a β-approximate cut packing for (V, d). Among cuts with y(S) = 0, let
(S  , S  ) be the sparsest. Then, the sparsity of this cut is at most β · f ∗ .

Proof: Let y be a β-approximate cut packing for metric (V, d). Then,
                                              
       ∗          e∈G ce de            e∈G ce    S:e∈δ(S) y(S)
      f =                    ≥                
               e∈H dem(e)de        e∈H dem(e)       S:e∈δ(S) βy(S)
              
                  y(S)c(S)
           = S
            β S y(S)dem(S)
                          
            1     c(S  )
           ≥ ·               .
            β    dem(S  )

    The ﬁrst inequality follows using both the upper bound and the lower
bound on the amount of cut felt by an edge; the former in the numerator and
the latter in the denominator. The equality after that follows by changing the
order of summation. The last inequality follows from the well known result
stated below.                                                               ✷

Proposition 21.8 For any nonnegative reals a1 , . . . , an and positive reals
b1 , . . . , bn and α1 , . . . , αn ,
           
                αa              a
           i i i ≥ min i .
              i αi bi
                          i bi


Moreover, this inequality holds with equality iﬀ the n values ai /bi are all
equal.

Corollary 21.9 If there is an exact cut packing for metric (V, d), then every
cut (S, S) with yS = 0 has sparsity f ∗ and thus is a sparsest cut in G.

Proof: By Theorem 21.7, the minimum sparsity cut with yS = 0 has sparsity
at most f ∗ (since β = 1). Since the sparsity of any cut upper bounds f ∗ , the
sparsity of this cut equals f ∗ , and this is a sparsest cut in G. But then all
                        21.3    Metrics, cut packings, and 1 -embeddability   185

inequalities in the proof of Theorem 21.7 must hold with equality. Now, by
the second statement in Proposition 21.8, we get that every cut (S, S) with
yS = 0 has sparsity f ∗ .                                                ✷
    The sparsest cut in the instance speciﬁed in Example 21.3 has sparsity
strictly larger than f ∗ . By Corollary 21.9, the optimal metric for this instance
does not have an exact cut packing. However, it turns out that every metric
has an O(log n)-approximate cut packing – we will show this using the notion
of 1 -embeddability of metrics.

21.3.2   1 -embeddability of metrics

A norm on the vector space Rm is a function * · * : Rm → R+ , such that for
any x, y ∈ Rm , and λ ∈ R:
• *x* = 0 iﬀ x = 0,
• *λx* = |λ| · *x*,
• *x + y* ≤ *x* + *y*.
For p ≥ 1, the p -norm is deﬁned by
                               p1
                    
      *x*p =           |xk |p  .
                1≤k≤m


The associated p -metric, denoted by d2p , is deﬁned by

      d2p (x, y) = *x − y*p

for all x, y ∈ Rm . In this section, we will only consider the 1 -norm.
     Let σ be a mapping, σ : V → Rm for some m. Let us say that *σ(u) −
σ(v)*1 is the 1 length of edge (u, v) under σ. We will say that σ is an isometric
1 -embedding for metric (V, d) if it preserves the 1 lengths of all edges, i.e.,

      ∀u, v ∈ V, d(u, v) = *σ(u) − σ(v)*1 .

    As shown below, in general, the metric computed by solving the dual
program may not be isometrically 1 -embeddable. Thus, we will relax this
notion – we will ensure that the mapping does not stretch any edge, but we
will allow it to shrink edges up to a speciﬁed factor. For β ≥ 1, we will say
that σ is a β-distortion 1 -embedding for metric (V, d) if

                    1
      ∀u, v ∈ V :     d(u, v) ≤ *σ(u) − σ(v)*1 ≤ d(u, v).
                    β
186      21   Sparsest Cut

    Next, we show that the question of ﬁnding an approximate cut packing for
a metric is intimately related to that of ﬁnding a low distortion 1 embedding
for it.

Lemma 21.10 Let σ : V → Rm be a mapping. There is a cut packing
y : 2V → R+ such that each edge feels as much cut under y as its 1 length
under σ. Moreover, the number of nonzero yS ’s is at most m(n − 1).

Proof: First consider the case when m = 1. Let the n vertices of V be
mapped to u1 ≤ u2 ≤ · · · ≤ un . Assume w.l.o.g. that the vertices are also
numbered in this order. For each i, 1 ≤ i ≤ n − 1, let y{v1 ,...,vi } = ui+1 − ui .
Clearly, this cut packing satisﬁes the required condition.
   For arbitrary m, we observe that since the 1 -norm is additive, we can
deﬁne a cut packing for each dimension independently, and the sum of these
packings satisﬁes the required condition.                                       ✷

Lemma 21.11 Let y : 2V → R+ be a cut packing with m nonzero yS ’s.
There is a mapping σ : V → Rm such that for each edge, its 1 length under
σ is the same as the amount of cut it feels under y.

Proof: We will have a dimension corresponding to each set S ⊆ V such that
yS = 0. For vertices in S, this coordinate will be 0, and for vertices in S, this
coordinate will be yS . Thus, this dimension contributes exactly as much to
the 1 length of an edge as the amount of cut felt by this edge due to yS .
Hence this mapping satisﬁes the required condition.                             ✷
      Lemmas 21.10 and 21.11 give:
Theorem 21.12 There exists a β-distortion 1 -embedding for metric (V, d)
iﬀ there exists a β-approximate cut packing for it. Moreover, the number of
nonzero cuts and the dimension of the 1 -embedding are polynomially related.

Corollary 21.13 Metric (V, d) is isometrically 1 -embeddable iﬀ there exists
an exact cut packing for it.
   We have already shown that the metric obtained for the instance in Ex-
ample 21.3 does not have an exact cut packing. Therefore, it is not iso-
metrically 1 -embeddable. However, we will show that any metric has an
O(log n)-distortion 1 -embedding; this fact lies at the heart of the approxi-
mation algorithm for the sparsest cut problem.


21.4 Low distortion 1 -embeddings for metrics
First consider the following one-dimensional embedding for metric (V, d): pick
a set S ⊆ V , and deﬁne the coordinate of vertex v to be σ(v) = mins∈S d(s, v),
                           21.4   Low distortion 1 -embeddings for metrics   187

i.e., the length of the shortest edge from v to S. This mapping does not stretch
any edge:

Lemma 21.14 For the one-dimensional embedding given above,

      ∀u, v ∈ V, |σ(u) − σ(v)| ≤ d(u, v).

Proof: Let s1 and s2 be the closest vertices of S to u and v, respectively.
Assume w.l.o.g. that d(s1 , u) ≤ d(s2 , v). Then, |σ(u) − σ(v)| = d(s2 , v) −
d(s1 , u) ≤ d(s1 , v) − d(s1 , u) ≤ d(u, v). The last inequality follows by the
triangle inequality.                                                         ✷
    More generally, consider the following m-dimensional embedding: Pick
m subsets of V , S1 , . . . , Sm , and deﬁne the ith coordinate of vertex v to be
σi (v) = mins∈Si d(s, v)/m; notice the scaling factor of m used. The additivity
of 1 metric, together with Lemma 21.14, imply that this mapping also does
not stretch any edge.

21.4.1   Ensuring that a single edge is not overshrunk

The remaining task is to choose the sets in such a way that no edge shrinks by
a factor of more than O(log n). It is natural to use randomization for picking
the sets. Let us ﬁrst ensure that a single edge (u, v) is not overshrunk. For
this purpose, deﬁne the expected contribution of set Si to the 1 length of
edge (u, v) to be E[|σi (u) − σi (v)|].
    For simplicity, assume that n is a power of 2; let n = 2l . For 2 ≤ i ≤
l + 1, set Si is formed by picking each vertex of V with probability 1/2i .
The embedding w.r.t. these sets works for the single edge (u, v) with high
probability. The proof of this fact involves cleverly taking into consideration
the expected contribution of each set. For diﬀerent metrics, diﬀerent sets
have a large contribution. In order to develop intuition for the proof, we ﬁrst
illustrate this through a series of examples.
Example 21.15 In the following three metrics, d(u, v) = 1, and the n
vertices are placed as shown in the ﬁgure below.

                    u s                                    sv
                     n/2                                  n/2


                    u s                 s                   sv
                      1                n−2                 1


                    u√s                s√                 √s v
                      n              n−2 n                 n
188      21   Sparsest Cut

    For each metric, the expected contribution of one of the sets is Ω(d(u, v)/l).
For the ﬁrst metric, this set is Sl , since it will be a singleton with constant
probability. For the second metric, this set is S2 , since it will contain exactly
one of u and v with constant probability. For the third metric, this set is
Sl/2√
      , since with constant probability, it will contain exactly one vertex of
the 2 n vertices bunched up with u and v.                                       ✷
    In the next lemma, we encapsulate the basic mechanism for establishing
a lower bound on the expected contribution of a set Si . For any vertex x
and nonnegative real r, let B(x, r) denote the ball of radius r around x, i.e.,
B(x, r) = {s ∈ V |d(x, s) ≤ r}.

Lemma 21.16 If for some choice of r1 ≥ r2 ≥ 0, and constant c,

      Pr[(Si ∩ B(u, r1 ) = ∅) and (Si ∩ B(v, r2 ) = ∅)] ≥ c,

then the expected contribution of Si is ≥ c(r1 − r2 )/l.

Proof: Under the event described, d(u, Si ) ≥ r1 and d(v, Si ) ≤ r2 . If so,
σi (u) ≥ r1 /l and σi (v) ≤ r2 /l. Therefore, |σi (u) − σi (v)| ≥ (r1 − r2 )/l, and
the lemma follows.                                                                ✷
   The remaining task is to deﬁne suitable radii r1 and r2 for each set Si
such that the probabilistic statement of Lemma 21.16 holds. We will need
the following simple probabilistic fact:
Lemma 21.17 For 1 ≤ t ≤ l − 1, let A and B be disjoint subsets of V ,
such that |A| < 2t and |B| ≥ 2t−1 . Form set S by picking each vertex of V
independently with probability p = 1/(2t+1 ). Then,

      Pr[(S ∩ A = ∅) and (S ∩ B = ∅)] ≥ (1/2)(1 − e−1/4 ).


Proof:
                                                    1
      Pr[S ∩ A = ∅] = (1 − p)|A| ≥ (1 − p|A|) ≥       ,
                                                    2
where the ﬁrst inequality follows by taking the ﬁrst two terms of the binomial
expansion.

      Pr[S ∩ B = ∅] = (1 − p)|B| ≤ e−p|B| ≤ e−1/4 ,

where we have used the inequality 1 − x ≤ e−x . Therefore,

      Pr[S ∩ B = ∅] = 1 − (1 − p)|B| ≥ 1 − e−1/4 .
                            21.4   Low distortion 1 -embeddings for metrics         189

Finally, observe that since A and B are disjoint, the two events [S ∩ A = ∅]
and [S ∩ B = ∅] are independent. The lemma follows.                      ✷
     For convenience, let c = (1/2)(1 − e−1/4 ).
     For 0 ≤ t ≤ l, deﬁne ρt = min{ρ ≥ 0 : |B(u, ρ)| ≥ 2t and |B(v, ρ)| ≥
  t
2 }, i.e., ρt is the smallest radius such that the ball around u and the ball
around v each has at least 2t vertices. Clearly, ρ0 = 0 and ρl ≥ d(u, v). Let
t̂ = max{t : ρt < d(u, v)/2}; clearly, t̂ ≤ l − 1. Finally, for any vertex x and
nonnegative real r, let B ◦ (x, r) denote the open ball of radius r around x,
i.e., B ◦ (x, r) = {s ∈ V |d(x, s) < r}.
Lemma 21.18 For 1 ≤ t ≤ t̂, the expected contribution of St+1 is at most
  ρt −ρt−1
c·   l            t = t̂ + 1, the expected contribution of St+1 is at most
           , and for
c     d(u,v)
l ·     2    − ρt−1   .

Proof: First consider t such that 1 ≤ t ≤ t̂. By the deﬁnition of ρt , for
at least one of the two vertices u and v, the open ball of radius ρt contains
fewer than 2t vertices. Assume w.l.o.g. that this happens for vertex u, i.e.,
|B ◦ (u, ρt )| < 2t . Again, by deﬁnition, |B(v, ρt−1 )| ≥ 2t−1 . Since ρt−1 < ρt <
d(u, v)/2, the two sets B ◦ (u, ρt ) and B(v, ρt−1 ) are disjoint. Thus, by Lemma
21.17, the probability that St+1 is disjoint from the ﬁrst set and intersects
the second is least c. Now, the ﬁrst claim follows from Lemma 21.16.
    Next, let t = t̂ + 1. By the deﬁnition of t̂, for at least one of the
two vertices u and v, the open ball of radius d(u, v)/2 contains fewer
than 2t vertices. As before, w.l.o.g. assume this happens for vertex u, i.e.,
|B ◦ (u, d(u, v)/2)| < 2t . Clearly, |B(v, ρt−1 )| ≥ 2t−1 . Since ρt−1 < d(u, v)/2,
the two sets B ◦ (u, d(u, v)/2) and B(v, ρt−1 ) are disjoint. The rest of the rea-
soning is the same as before.                                                     ✷

Lemma 21.19 The expected contribution of all sets S2 , . . . , Sl+1 is at most
c d(u,v)
2 · l    .

Proof: By Lemma 21.18, the expected contribution of all sets S2 , . . . , Sl+1
is at least the following telescoping sum:
                                                           
       c                                       d(u, v)           c d(u, v)
         · (ρ1 − ρ0 ) + (ρ2 − ρ1 ) + . . . +           − ρt̂    = ·        .
       l                                         2               2    l
                                                                                        ✷
                            ,                                       -         c/2
Lemma 21.20               Pr contribution of all sets is ≥ c d(u,v)
                                                              4l      ≥             .
                                                                            2 − c/2

Proof: Denote the probability in question by p. Clearly, the total contribu-
tion of all sets S2 , . . . , Sl+1 to the 1 length of edge (u, v) is at most d(u, v)/2l.
This fact and Lemma 21.19 give:
190      21     Sparsest Cut

             d(u, v)             c d(u, v)   d(u, v)
        p·           + (1 − p) ·           ≥         .
                l                   4l          l
                c/2
Therefore, p ≥ 2−c/2 .                                                          ✷


21.4.2       Ensuring that no edge is overshrunk

The above embedding does not overshrink edge (u, v) with constant proba-
bility. In order to ensure that no edge is overshrunk, we will ﬁrst enhance this
probability. The key idea is to repeat the entire process several times indepen-
dently and use Chernoﬀ bounds to bound the error probability. We will use
the following statement of the Chernoﬀ bound: Let X1 , . . . , Xn be   indepen-
                                                                          n
dent Bernoulli trials with Pr[Xi = 1] = p, 0 < p < 1, and let X = i=1 Xi ;
clearly, E[X] = n p. Then, for 0 < δ ≤ 1,

        Pr[X < (1 − δ) n p] < exp(−δ 2 n p/2).

     Pick sets S2 , . . . , Sl+1 using probabilities speciﬁed above, independently
N = O(log n) times each. Call the sets so obtained Sij , 2 ≤ i ≤ l + 1, 1 ≤
j ≤ N . Consider the N · l = O(log2 n) dimensional embedding of metric
(V, d) w.r.t. these N · l sets. We will prove that this is an O(log n)-distortion
1 -embedding for metric (V, d).

Lemma 21.21 For N = O(log n), this embedding satisﬁes:

                                  p c d(u, v)         1
        Pr[*σ(u) − σ(v)*1 ] ≥                 ] ≥ 1 − 2,
                                       4l            2n

where p = c/(2 − c).

Proof: We will think of the process of picking sets S2 , . . . , Sl+1 once as
a single Bernoulli trial; thus, we have N such trials. A trial succeeds if the
contribution of all its sets is ≥ (c d(u, v))/2l. By Lemma 21.20, the probability
of success is at least p. Using the Chernoﬀ bound with δ = 1/2, the probability
that at most N p/2 of these trials succeed is at most exp(N p/8). Clearly, this
is bounded by 1/2n2 for N = O(log n). If at least N p/2 trials succeed, the
1 length of edge (u, v) will be at least p c d(u, v)/4l = d(u, v)/O(log n). The
lemma follows.                                                                 ✷
      Adding the error probabilities for all n(n − 1)/2 edges, we get:

Theorem 21.22 The N l = O(log2 n) dimensional embedding given above
is an O(log n)-distortion 1 -embedding for metric (V, d), with probability at
least 1/2.
                                        21.5   LP-rounding-based algorithm         191

21.5 LP-rounding-based algorithm
The reader can verify that Claim 21.6 and Theorems 21.7, 21.12, and 21.22
lead to an O(log n) factor approximation algorithm for the sparsest cut prob-
lem. In this section, we will improve the approximation guarantee to O(log k)
where k is the number of source–sink pairs speciﬁed.
    For this purpose, notice that Theorem 21.7 holds even for the following
less stringent approximate cut packing: no edge is allowed to be overpacked,
and the edges of the demand graph are not under-packed by more than a β
factor (the rest of the edges are allowed to be under-packed to any extent).
In turn, such a cut packing can be obtained from an 1 -embedding that does
not overshrink edges of the demand graph only. Since these are only O(k 2 )
in number, where k is the number of source–sink pairs, we can ensure that
these edges are not shrunk by a factor of more than O(log k), thus enabling
an improvement in the approximation guarantee.
    Let V  ⊆ V be the set of vertices that are sources or sinks, |V  | ≤ 2k. For
simplicity, assume |V  | is a power of 2; let |V  | = 2l . The sets S2 , . . . , Sl+1
will be picked from V  , and it is easy to verify from the proof of Lemma 21.21
that N = O(log k) will suﬃce to ensure that none of the O(k 2 ) edges of the
demand graph is shrunk by more than a factor of O(log k). The complete
algorithm is:


 Algorithm 21.23 (Sparsest cut)
  1. Solve the dual LP (21.2) to obtain metric (V, d).
  2. Pick sets Sij , 2 ≤ i ≤ l + 1, 1 ≤ j ≤ N , where set Sij is formed by
     picking each vertex of V  independently with probability 1/2i .
  3. Obtain an 1 -embedding of (V, d) in O(log2 k)-dimensional space w.r.t.
     these sets.
  4. Obtain an approximate cut packing for (V, d) from the 1 -embedding.
  5. Output the sparsest cut used by the cut packing.


Theorem 21.24 Algorithm 21.23 achieves an approximation guarantee of
O(log k) for the sparsest cut problem.
Corollary 21.25 For a demands multicommodity ﬂow instance with k source–
sink pairs,
                                  
         1                 c(S)                                      c(S)
                      min              ≤       max        f   ≤ min        .
      O(log k)       S⊂V  dem(S)           throughput f         S⊂V dem(S)
192      21    Sparsest Cut

21.6 Applications
We present below a number of applications of the sparsest cut problem.

21.6.1    Edge expansion

Expander graphs have numerous applications; for instance, see Example 20.9.
We will obtain an O(log n) factor algorithm for the problem of determining
the edge expansion of a graph:
Problem 21.26 (Edge expansion) Given an undirected graph G = (V, E),
the edge expansion of a set S ⊂ V with |S| ≤ n/2, is deﬁned to be |δ(S)|,
i.e., the number of edges in the cut (S, S). The problem is to ﬁnd a minimum
expansion set.
     Consider the special case of demands multicommodity ﬂow in which we
have n(n − 1)/2 distinct commodities, one for each pair of vertices. This
is called the uniform multicommodity ﬂow problem. For this problem, the
sparsity of any cut (S, S) is given by

        c(S)
                .
      |S| · |S|

    Let (S, S), with |S| ≤ |S|, be the cut found by Algorithm 21.23 when run
on G with uniform demands. Notice that |S| is known within a factor of 2,
since n/2 ≤ |S| ≤ n. Thus, S has expansion within an O(log n) factor of the
minimum expansion set in G. Clearly, the generalization of this problem to
arbitrary edge costs also has an O(log n) factor approximation algorithm.

21.6.2    Conductance

The conductance of a Markov chain characterizes its mixing rate, i.e., the
number of steps needed to ensure that the probability distribution over states
is suﬃciently close to its stationary distribution. Let P be the transition ma-
trix of a discrete-time Markov chain on a ﬁnite state space X, and let π
denote the stationary probability distribution of this chain. We will assume
that the chain is aperiodic, connected, and that it satisﬁes the detailed bal-
ance condition, i.e.,

      π(x)P (x, y) = π(y)P (y, x) ∀x, y ∈ X.

    Deﬁne undirected graph G = (X, E) on vertex set X such that (x, y) ∈ E
iﬀ π(x)P (x, y) = 0. The edge weights are deﬁned to be w(x, y) = π(x)P (x, y).
The conductance of this chain is given by

                               w(S, S)
      Φ=            min                ,
              S⊂X,0<π(S)≤1/2    π(S)
                                                       21.6   Applications     193

where w(S, S) is the sum of weights of all edges in the cut (S, S). For any set
S, the numerator of the quotient deﬁned above is the probability that the
chain in equilibrium escapes from set S to S in one step. Thus the quotient
gives the conditional probability of escape, given that the chain is initially
in S and Φ measures the ability of the chain to not get trapped in any small
region of the state space.
    Theorem 21.24 leads to an O(log n) factor approximation algorithm for
computing conductance. First, observe that it suﬃces to approximate the
following symmetrized variant of Φ:

                              w(S, S)
      Φ =       min                  ,                                      (21.3)
             S⊂X,0<π(S)≤1    π(S)π(S)

since Φ and Φ are within a factor of 2 of each other (notice that if 0 < π(S) ≤
1/2, then 1/2 ≤ π(S) < 1).
    Next, let us show that computing Φ is really a special case of the sparsest
cut problem. Consider graph G = (X, E) with edge weights as deﬁned above.
For each pair of vertices x, y ∈ X, deﬁne a distinct commodity with a demand
of π(x)π(y). It is easy to see that the sparsity of a cut (S, S) for this instance
is simply the quotient deﬁned in (21.3). Hence, the sparsity of the sparsest
cut is Φ .

21.6.3   Balanced cut

The following problem ﬁnds applications in partitioning problems, such as
circuit partitioning in VLSI design. Furthermore, it can be used to perform
the “divide” step of the divide-and-conquer algorithms for certain problems;
for instance, see the algorithm for Problem 21.29 below.
Problem 21.27 (Minimum b-balanced cut) Given an undirected graph
G = (V, E) with nonnegative edge costs and a rational b, 0 < b ≤ 1/2, ﬁnd a
minimum capacity cut (S, S) such that b · n ≤ |S| < (1 − b) · n.
    A b-balanced cut for b = 1/2 is called a bisection cut, and the problem of
ﬁnding a minimum capacity such cut is called the minimum bisection problem.
We will use Theorem 21.24 to obtain a pseudo-approximation algorithm for
Problem 21.27 – we will ﬁnd a (1/3)-balanced cut whose capacity is within
an O(log n) factor of the capacity of a minimum bisection cut (see the notes
in Section 21.8 for a true approximation algorithm).
    For V  ⊂ V , let GV  denote the subgraph of G induced by V  . The
algorithm is: Initialize U ← ∅ and V  ← V . Until |U | ≥ n/3, ﬁnd a minimum
expansion set in GV  , say W , then set U ← U ∪W and V  ← V  −W . Finally,
let S ← U , and output the cut (S, V − S).
Claim 21.28 The cut output by the algorithm is a (1/3)-balanced cut whose
capacity is within an O(log n) factor of the capacity of a minimum bisection
cut in G.
194      21   Sparsest Cut

Proof: At the end of the penultimate iteration, |U | < n/3. Thus, at the
beginning of the last iteration, |V  | ≥ 2n/3. At most half of these vertices are
added to U in the last iteration. Therefore, |V − S| ≥ n/3 and n/3 ≤ |S| <
n/3. Hence, (S, V − S) is a (1/3)-balanced cut.
    Let (T, T ) be a minimum bisection cut in G. Since at the beginning of
each iteration, |V  | ≥ 2n/3, each of the sets T ∩ V  and T ∩ V  has at
least n/6 vertices. Thus, the expansion of a minimum expansion set in GV 
                                c(T )
in each iteration is at most (n/6)     . Since the algorithm ﬁnds a set having
expansion within a factor of O(log n) of optimal in any iteration, the set U
found satisﬁes:

      c(U )              c(T )
            ≤ O(log n) ·       .
       |U |              n/6

Since the ﬁnal set S has at most 2n/3 vertices, summing up we get

                          c(T )(2n/3)
      c(S) ≤ O(log n) ·               ,
                              n/6

thereby giving c(S) ≤ O(log n) · c(T ).                                         ✷


21.6.4    Minimum cut linear arrangement

Problem 21.29 (Minimum cut linear arrangement) Given an undi-
rected graph G = (V, E) with nonnegative edge costs, for a numbering of its
vertices from 1 to n, deﬁne Si to be the set of vertices numbered at most i,
for 1 ≤ i ≤ n − 1; this deﬁnes n − 1 cuts. The problem is to ﬁnd a numbering
that minimizes the capacity of the largest of these n−1 cuts, i.e., it minimizes
max{c(Si )| 1 ≤ i ≤ (n − 1)}.
    Using the pseudo-approximation algorithm obtained above for the (1/3)-
balanced cut problem, we will obtain a true O(log2 n) factor approximation
algorithm for this problem. A key observation is that in any arrangement,
Sn/2 is a bisection cut, and thus the capacity of a minimum bisection cut
in G, say β, is a lower bound on the optimal arrangement. The reason we
get a true approximation algorithm is that the (1/3)-balanced cut algorithm
compares the cut found to β.
    The algorithm is recursive: ﬁnd a (1/3)-balanced cut in GV , say (S, S),
and recursively ﬁnd a numbering of S in GS using numbers from 1 to |S|
and a numbering of S in GS using numbers from |S| + 1 to n. Of course,
the recursion ends when the set is a singleton, in which case the prescribed
number is assigned to this vertex.

Claim 21.30 The algorithm given above achieves an O(log2 n) factor for the
minimum cut linear arrangement problem.
                                                          21.7   Exercises    195

Proof: The following binary tree T (not necessarily complete) encodes the
outcomes of the recursive calls made by the algorithm: Each recursive call
corresponds to a node of the tree. Suppose recursive call α ends with two
further calls, α1 and α2 , where the ﬁrst call assigns smaller numbers and the
second assigns larger numbers. Then, α1 will be made the left child of α in
T and α2 will be made the right child of α. If recursive call α was made with
a singleton, then α will be a leaf of the tree.
     To each nonleaf, we will assign the set of edges in the cut found during
this call, and to each leaf we will assign its singleton vertex. Thus, the left to
right ordering of leaves gives the numbering assigned by the algorithm to the
vertices. Furthermore, the edge sets associated with nonleaf nodes deﬁne a
partitioning of all edges of G. The cost of edges associated with any nonleaf
is O(log n)β by Claim 21.28. Since each recursive call ﬁnds a (1/3)-balanced
cut, the depth of recursion, and hence the depth of T , is O(log n).
     Following is a crucial observation: Consider any edge (u, v) in G. Let α be
the lowest common ancestor of leaves corresponding to u and v in T . Then,
(u, v) belongs to the set of edges associated with node α.
     With respect to the numbering found by the algorithm, consider a cut
(Si , Si ), 1 ≤ i ≤ n − 1. Any edge in this cut connects vertices numbered j
and k with j ≤ i and k ≥ i + 1. Thus, such an edge must be associated with
a node that is a common ancestor of the leaves numbered i and i + 1. Since
the depth of T is O(log n), there are O(log n) such common ancestors. Since
the cost of edges associated with any node in T is O(log n)β, the cost of cut
(Si , Si ) is bounded by O(log2 n)β. The claim follows since we have already
argued that β is a lower bound on the optimal arrangement.                     ✷



21.7 Exercises

21.1 For each of the three metrics given in Example 21.15, one of the sets
S2 , . . . , Sl+1 has an expected contribution of Ω(d(u, v)/l). Give a metric for
which each set has an expected contribution of Θ(d(u, v)/l2 ).

21.2 Show that n points embedded in 1 space can be an isometric embedding
in (a higher dimensional) 22 space.
Hint: Since 1 and 22 are both additive across dimensions, ﬁrst show that
it is suﬃcient to consider n points in one dimension. Sort these points, and
renumber, say x1 , . . . , xn . Now embed these in (Rn−1 , 22 ) as follows. Let
                                     √         √
αi = xi+1 − xi . Map point xi to ( α1 , . . . , αi−1 , 0, . . . , 0).

21.3 Why can’t the pseudo-approximation algorithm given at the beginning
of Section 21.6.3 be converted to a true approximation algorithm, i.e., so
that in the end, we compare the (1/3)-balanced cut found to the optimal
(1/3)-balanced cut?
196    21     Sparsest Cut

Hint: Construct graphs for which the capacity of a minimum bisection cut
is arbitrarily higher than that of a (1/3)-balanced cut.

21.4 Show that the above algorithm extends to ﬁnding a b-balanced cut
that is within an O(log n) factor of the best b -balanced cut for b ≤ 1/3 and
b < b . Where in the argument is the restriction b ≤ 1/3 used?

21.5 Give an approximation factor preserving reduction from the problem
of ﬁnding a minimum b-balanced cut, for b < 1/2, to the minimum bisection
problem.

21.6 (Linial, London and Rabinovich [190]) Extend Theorem 21.22 to show
that for any p ≥ 1, there is an O(log n) distortion p -embedding for metric
(V, d) in O(log2 n)-dimensional space.
Hint: Map point v to d(v,S   i)
                          Q1/p
                                , for i = 1, . . . , Q, where Q is the dimension
of the embedding. Use the fact that |d(u, Si ) − d(v, Si )| ≤ d(u, v) and the
monotonicity of p -norm.

21.7 (Feige [79]) Consider the following algorithm for:
Problem 21.31 (Bandwidth minimization) Given an undirected graph
G = (V, E), number the vertices with distinct integers from 1 to n so that
the spread of the longest edge is minimized, where the spread of edge (u, v)
is the absolute value of the diﬀerence of the numbers assigned to u and v.


 Algorithm 21.32 (Bandwidth minimization)
  1. Deﬁne metric (V, d), where duv is the length of the shortest path from u
     to v in G.
  2. Obtain an O(log n)-distortion 2 -embedding of (V, d).
  3. Pick a line  from a spherically symmetric distribution, and project the n
     points onto .
  4. Number the vertices from 1 to n according to their ordering on .
  5. Output the numbering.


Remark 21.33 Lemma 26.7 gives an algorithm for picking .

 1. Show that the expected number of pairs of vertices that are within a
    distance of 1 of each other on  is bounded by
                       1
            O(log n          ).
                         d
                      u,v uv
                                                          21.8   Notes    197

2. Show that
          1
                = O(n log n · OPT).
            d
         u,v uv


   Hint: Use the fact that in G, the number of vertices within a distance
   of k of a vertex v is bounded by 2k · OPT.
3. Show that √ with high probability, the spread√of the numbering output is
   at most O( nOPT log n), i.e., this is an O( n log n) factor algorithm.
   Hint: If the spread of the output numbering is s, then the number of
   pairs of vertices that are within a distance of 1 of each other on  is at
   least s2 .


21.8 Notes
The seminal work of Leighton and Rao [182] gave the ﬁrst approximate max-
ﬂow min-cut theorem, for the case of uniform multicommodity ﬂow. They
also gave a factor O(log n) approximation algorithm for the associated special
case of sparsest cut and a pseudo-approximation algorithm for the b-balanced
cut problem. The general version of demands multicommodity ﬂow was ﬁrst
considered by Klein, Agarwal, Ravi, and Rao [173]. Theorem 21.22 is due to
Linial, London, and Rabinovich [190], based on a result of Bourgain [32] who
showed the existence of such an embedding and gave an exponential time
algorithm for ﬁnding it. The application of this theorem to the sparsest cut
problem, Theorem 21.24, was independently given by Aumann and Rabani
[16], and Linial, London, and Rabinovich [190].
    An O(log2 n) factor algorithm for the minimum bisection problem, and
hence for the minimum b-balanced cut problem (see Exercise 21.5), was given
by Feige and Krauthgamer [83]. The application of sparsest cut to computing
conductance is due to Sinclair [241], and the application of balanced cuts to
the minimum cut linear arrangement problem is due to Bhatt and Leighton
[26]. See Exercise 26.9 for a semideﬁnite program for ﬁnding an optimal
distortion 22 -embedding of n points.
