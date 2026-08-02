---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-23"
chapter_number: 23
chapter_title: "Steiner Network"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 231
source_page_end: 249
printed_page_start: 213
printed_page_end: 231
part_ids: ["approximation-algorithms-ch-23-part-024"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Steiner Network

23 Steiner Network




The following generalization of the Steiner forest problem to higher connec-
tivity requirements has applications in network design and is also known as
the survivable network design problem. In this chapter, we will give a factor
2 approximation algorithm for this problem by enhancing the LP-rounding
technique to iterated rounding. A special case of this problem was considered
in Exercise 22.10.
Problem 23.1 (Steiner network) We are given an undirected graph
G = (V, E), a cost function on edges c : E → Q+ (not necessarily satisfying
the triangle inequality), a connectivity requirement function r mapping un-
ordered pairs of vertices to Z+ , and a function u : E → Z+ ∪ {∞} stating
an upper bound on the number of copies of edge e we are allowed to use; if
ue = ∞, there is no upper bound for edge e. The problem is to ﬁnd a mini-
mum cost multigraph on vertex set V that has r(u, v) edge disjoint paths for
each pair of vertices u, v ∈ V . Each copy of edge e used for constructing this
graph will cost c(e).


23.1 The LP-relaxation and half-integrality
In order to give an integer programming formulation for this problem, we
will ﬁrst deﬁne a cut requirement function, f : 2V → Z+ , as we did for the
metric Steiner forest problem. For every S ⊆ V , f (S) is deﬁned to be the
largest connectivity requirement separated by the cut (S, S), i.e., f (S) =
max{r(u, v)|u ∈ S and v ∈ S}.
                   
     minimize            ce xe                                          (23.1)
                   e∈E
                     
     subject to                  xe ≥ f (S),   S⊆V
                   e: e∈δ(S)

                   xe ∈ Z+ ,                   e ∈ E and ue = ∞
                   xe ∈ {0, 1, . . . , ue },   e ∈ E and ue = ∞

   The LP-relaxation is:
214     23   Steiner Network
                    
      minimize            ce xe                                             (23.2)
                    e∈E
                      
      subject to                  xe ≥ f (S),   S⊆V
                    e: e∈δ(S)
                    xe ≥ 0,                     e ∈ E and ue = ∞
                    ue ≥ xe ≥ 0,                e ∈ E and ue = ∞

    Since LP (23.2) has exponentially many constraints, we will need the
ellipsoid algorithm for ﬁnding an optimal solution. Exercise 23.1 develops a
polynomial-sized LP.
    As shown in Chapters 14 and 19, certain NP-hard problems, such as ver-
tex cover and node multiway cut, admit LP-relaxations having the remarkable
property that they always have a half-integral optimal solution. Rounding up
all halves to 1 in such a solution leads to a factor 2 approximation algorithm.
Does relaxation (23.2) have this property? The following lemma shows that
the answer is “no”.

Lemma 23.2 Consider the Petersen graph (see Section 1.2) with a connec-
tivity requirement of 1 between each pair of vertices and with each edge of
unit cost. Relaxation (23.2) does not have a half-integral optimal solution for
this instance.

Proof: Consider the fractional solution xe = 1/3 for each edge e. Since
the Petersen graph is 3-edge connected (in fact, it is 3-vertex connected as
well), this is a feasible solution. The cost of this solution is 5. In any feasible
solution, the sum of edge variables incident at any vertex must be at least 1,
to allow connectivity to other vertices. Therefore, any feasible solution must
have cost at least 5 (since the Petersen graph has 10 vertices). Hence, the
solution given above is in fact optimal.
    Any solution with xe = 1 for some edge e must have cost exceeding 5,
since additional edges are required to connect the endpoints of e to the rest of
the graph. Therefore, any half-integral solution of cost 5 would have to pick,
to the extent of one half each, the edges of a Hamiltonian cycle. Since the
Petersen graph has no Hamiltonian cycles, there is no half-integral optimal
solution.                                                                        ✷
    Let us say that an extreme point solution, also called a vertex solution or a
basic feasible solution, for an LP is a feasible solution that cannot be written
as the convex combination of two feasible solutions. The solution xe = 1/3,
for each edge e, is not an extreme point solution. An extreme optimal solution
is shown in the ﬁgure below; thick edges are picked to the extent of 1/2, thin
edges to the extent of 1/4, and the missing edge is not picked.
                            23.1   The LP-relaxation and half-integrality   215




The isomorphism group of the Petersen graph is edge-transitive, and there
are 15 related extreme point solutions; the solution xe = 1/3 for each edge e
is the average of these.
    Notice that although the extreme point solution is not half-integral, it
picks some edges to the extent of half. We will show below that in fact this is
a property of any extreme point solution to LP (23.2). We will obtain a factor
2 algorithm by rounding up these edges and iterating. Let H be the set of
edges picked by the algorithm at some point. Then, the residual requirement
of cut (S, S) is f  (S) = f (S) − |δH (S)|, where δH (S) represents the set of
edges of H crossing the cut (S, S). In general, the residual cut requirement
function, f  , may not correspond to the cut requirement function for any
set of connectivity requirements. We will need the following deﬁnitions to
characterize it:
    Function f : 2V → Z+ is said to be submodular if f (V ) = 0, and for every
two sets A, B ⊆ V , the following two conditions hold:
• f (A) + f (B) ≥ f (A ∩ B) + f (A ∪ B)
• f (A) + f (B) ≥ f (A − B) + f (B − A).

Remark 23.3 Sometimes submodularity is deﬁned only with the ﬁrst con-
dition. We will need to work with the stronger deﬁnition given above.
    Two subsets of V , A and B, are said to cross if each of the sets, A − B,
B − A, and A ∩ B, is nonempty. If A and B don’t cross then either they are
disjoint or one of these sets is contained in the other.

Lemma 23.4 For any graph G on vertex set V , the function |δG (.)| is sub-
modular.
216    23   Steiner Network

Proof: If sets A and B do not cross, then the two conditions given in the
deﬁnition of submodular functions hold trivially. Otherwise, edges having
one endpoint in A ∩ B and the other in A ∪ B (edge e1 in the ﬁgure below)
contribute to δ(A) and δ(B) but not to δ(A − B) or δ(B − A). Similarly, edge
e2 below does not contribute to δ(A ∩ B) or to δ(A ∪ B). The remaining edges
contribute equally to both sides of both conditions.                      ✷


                                       e1




                       A             e2            B
    Function f : 2V → Z is said to be weakly supermodular if f (V ) = 0, and
for every two sets A, B ⊆ V , at least one of the following conditions holds:
• f (A) + f (B) ≤ f (A − B) + f (B − A)
• f (A) + f (B) ≤ f (A ∩ B) + f (A ∪ B).
   It is easy to check that the original cut requirement function is weakly
supermodular; by Lemma 23.5, so is the residual cut requirement function.

Lemma 23.5 Let H be a subgraph of G. If f : 2V (G) → Z+ is a weakly
supermodular function, then so is the residual cut requirement function f  .

Proof: Suppose f (A) + f (B) ≤ f (A − B) + f (B − A); the proof of the other
case is similar. By Lemma 23.4, |δH (A)|+|δH (B)| ≥ |δH (A−B)|+|δH (B−A)|.
Subtracting, we get f  (A) + f  (B) ≤ f  (A − B) + f  (B − A).        ✷
   We can now state the central polyhedral fact needed for the factor 2
algorithm in its full generality.
Theorem 23.6 For any weakly supermodular function f , any extreme point
solution, x, to LP (23.2) must pick some edge to the extent of at least a half,
i.e., xe ≥ 1/2 for at least one edge e.
                                    23.2   The technique of iterated rounding   217

23.2 The technique of iterated rounding
In this section, we will give an iterated rounding algorithm for the Steiner
network problem, using Theorem 23.6.

 Algorithm 23.7 (Steiner network)
  1. Initialization: H ← ∅: f  ← f .
  2. While f  ≡ 0, do:
      Find an extreme optimal solution,
         x, to LP (23.2) with cut requirements given by f  .
      For each edge e such that xe ≥ 1/2, include xe  copies of e in H,
         and decrement ue by this amount.
      Update f  : for S ⊆ V , f  (S) ← f (S) − |δH (S)|.
  3. Output H.


     The algorithm presented above achieves an approximation guarantee of
factor 2 for an arbitrary weakly supermodular function f . Establishing a
polynomial running time involves showing that an extreme optimal solution
to LP (23.2) can be found eﬃciently. We do not know how to do this for
an arbitrary weakly supermodular function f . However, if f is the original
cut requirement function for some connectivity requirements, then a poly-
nomial time implementation follows from the existence of a polynomial time
separation oracle for each iteration.
     For the ﬁrst iteration, a separation oracle follows from a max-ﬂow sub-
routine. Given a solution x, construct a graph on vertex set V with capacity
xe for each edge e. Then, for each pair of vertices u, v ∈ V , check if this graph
admits a ﬂow of at least r(u, v) from u to v. If not, we will get a violated cut,
i.e., a cut (S, S) such that δx (S) < f (S), where
                   
      δx (S) =               xe .
                 e: e∈δ(S)


    Let f  be the cut requirement function of a subsequent iteration. Given
a solution to LP (23.2) for this function, say x , deﬁne x as follows: for each
edge e, xe = xe + eH , where eH is the number of copies of edge e in H. The
following lemma shows that a separation oracle for the original function f
leads to a separation oracle for f  . Furthermore, this lemma also shows that
there is no need to update f  explicitly after each iteration.
Lemma 23.8 A cut (S, S) is violated by solution x under cut requirement
function f  iﬀ it is violated by solution x under cut requirement function f .

Proof: Notice that δx (S) = δx (S) + |δH (S)|. Since f (S) = f  (S) + |δH (S)|,
δx (S) ≥ f (S) iﬀ δx (S) ≥ f  (S).                                           ✷
218    23   Steiner Network

   Lemma 23.8 implies that solution x is feasible for the cut requirement
function f  iﬀ solution x is feasible for f . Assuming Theorem 23.6, whose
proof we will provide below, let us show that Algorithm 23.7 achieves an
approximation guarantee of 2.
Theorem 23.9 Algorithm 23.7 achieves an approximation guarantee of 2
for the Steiner network problem.

Proof: By induction on the number of iterations executed by the algorithm
when run with a weakly supermodular cut requirement function f , we will
prove that the cost of the integral solution obtained is within a factor of two
of the cost of the optimal fractional solution. Since the latter is a lower bound
on the cost of the optimal integral solution, the claim follows.
    For the base case, if f requires one iteration, the claim follows, since the
algorithm rounds up only edges e with xe ≥ 1/2.
    For the induction step, assume that x is the extreme optimal solution
obtained in the ﬁrst iteration. Obtain x̂ from x by zeroing out components
that are strictly smaller than 1/2. By Theorem 23.6, x̂ = 0. Let H be the set of
edges picked in the ﬁrst iteration. Since H is obtained by rounding up nonzero
components of x̂ and each of these components is ≥ 1/2, cost(H) ≤ 2·cost(x̂).
    Let f  be the residual requirement function after the ﬁrst iteration and
  
H be the set of edges picked in subsequent iterations for satisfying f  . The
key observation is that x − x̂ is a feasible solution for f  , and thus by the
induction hypothesis, cost(H  ) ≤ 2 · cost(x − x̂). Let us denote by H + H 
the edges of H together with those of H  . Clearly, H + H  satisﬁes f . Now,

      cost(H + H  ) ≤ cost(H) + cost(H  )
                    ≤ 2 · cost(x̂) + 2 · cost(x − x̂) ≤ 2 · cost(x).           ✷

Corollary 23.10 The integrality gap of LP (23.2) is bounded by 2.
    Notice that previous algorithms obtained using LP-rounding solved the
relaxation once and did the entire rounding based on this solution. These al-
gorithms did not exploit the full power of rounding – after part of the solution
is rounded, the remaining fractional solution may not be the best solution
to continue the rounding process. It may be better to assume integral values
for the rounded variables and recompute fractional values for the remaining
variables, as is done above. We will call this technique iterated rounding.
Example 23.11 The tight example given for the metric Steiner tree prob-
lem, Example 3.4, is also a tight example for this algorithm. Observe that
after including a subset of edges of the cycle, an extreme optimal solution to
the resulting problem picks the remaining edges of the cycle to the extent of
one half each. The algorithm ﬁnds a solution of cost (2 − ε)(n − 1), whereas
the cost of the optimal solution is n.                                      ✷
                            23.3   Characterizing extreme point solutions     219

23.3 Characterizing extreme point solutions
From polyhedral combinatorics we know that a feasible solution for a set of
linear inequalities in Rm is an extreme point solution iﬀ it satisﬁes m linearly
independent inequalities with equality. Extreme solutions of LP (23.2) satisfy
an additional property which leads to a proof of Theorem 23.6.
    We will assume that the cut requirement function f in LP (23.2) is an
arbitrary weakly supermodular function. Given a solution x to this LP, we
will say that an inequality is tight if it holds with equality. If this inequality
corresponds to the cut requirement of a set S, then we will say that set S is
tight. Let us make some simplifying assumptions. If xe = 0 for some edge e,
this edge can be removed from the graph, and if xe ≥ 1, xe  copies of edge
e can be picked and the cut requirement function be updated accordingly.
We may assume without loss of generality that an extreme point solution
x satisﬁes 0 < xe < 1, for each edge e in graph G. Therefore, each tight
inequality corresponds to a tight set. Let the number of edges in G be m.
    We will say that a collection, L, of subsets of V forms a laminar family
if no two sets in this collection cross. The inequality corresponding to a set
S deﬁnes a vector in Rm : the vector has a 1 corresponding to each edge
e ∈ δG (S), and 0 otherwise. We will call this the incidence vector of set S,
and will denote it by AS .
Theorem 23.12 Corresponding to any extreme point solution to LP (23.2)
there is a collection of m tight sets such that
• their incidence vectors are linearly independent, and
• collection of sets forms a laminar family.

Example 23.13 The extreme point solution for the Peterson graph assigns
nonzero values to 14 of the 15 edges. By Theorem 23.12, there should be 14
tight sets whose incidence vectors are linearly independent. These are marked
in ﬁgure.                                                                   ✷
    Fix an extreme point solution, x, to LP (23.2). Let L be a laminar family
of tight sets whose incidence vectors are linearly independent. Denote by
span(L) the vector space generated by the set of vectors {AS |S ∈ L}. Since
x is an extreme point solution, the span of the collection of all tight sets is m.
We will show that if span(L) < m, then there is a tight set S whose addition
to L does not violate laminarity and also increases the span. Continuing in
this manner, we will obtain m tight sets as required in Theorem 23.12.
    We begin by studying properties of crossing tight sets.
Lemma 23.14 Let A and B be two crossing tight sets. Then, one of the
following must hold:
• A − B and B − A are both tight and AA + AB = AA−B + AB−A
• A ∪ B and A ∩ B are both tight and AA + AB = AA∪B + AA∩B .
220    23    Steiner Network

Proof: Since f is weakly supermodular, either f (A) + f (B) ≤ f (A − B) +
f (B − A) or f (A) + f (B) ≤ f (A ∪ B) + f (A ∩ B). Let us assume the former
holds; the proof for the latter is similar. Since A and B are tight, we have

      δx (A) + δx (B) = f (A) + f (B).

Since A − B and B − A are not violated,

      δx (A − B) + δx (B − A) ≥ f (A − B) + f (B − A).

Therefore,

      δx (A) + δx (B) ≤ δx (A − B) + δx (B − A).

    As argued in Lemma 23.4 (which established the submodularity of func-
tion |δG (.)|), edges having one endpoint in A ∪ B and the other in A ∩ B can
contribute only to the left-hand side of this inequality. The rest of the edges
must contribute equally to both sides. So, this inequality must be satisﬁed
with equality. Furthermore, since xe > 0 for each edge e, G cannot have
any edge having one endpoint in A ∪ B and the other in A ∩ B. Therefore,
AA + AB = AA−B + AB−A .                                                      ✷
   For any set S ⊆ V , deﬁne its crossing number to be the number of sets of
L that S crosses.
Lemma 23.15 Let S be a set that crosses set T ∈ L. Then, each of the sets
S − T, T − S, S ∪ T and S ∩ T has a smaller crossing number than S.

Proof: The ﬁgure below illustrates the three ways in which a set T  ∈ L can
cross one of these four sets without crossing T itself (T  is shown dotted). In
all cases, T  crosses S as well. In addition, T crosses S but not any of the
four sets.                                                                    ✷




                   S



                                                       T
                                             23.4   A counting argument      221

Lemma 23.16 Let S be a tight set such that AS ∈ span(L) and S crosses
some set in L. Then, there is a tight set S  having a smaller crossing number
than S and such that AS  ∈ span(L).

Proof: Let S cross T ∈ L. Suppose the ﬁrst possibility established in Lemma
23.14 holds; the proof of the second possibility is similar. Then, S −T and T −
S are both tight sets and AS + AT = AS−T + AT −S . This linear dependence
implies that AS−T and AT −S cannot both be in span(L), since otherwise
AS ∈ span(L). By Lemma 23.15, S − T and T − S both have a smaller
crossing number than S. The lemma follows.                                    ✷

Corollary 23.17 If span(L) = Rm , then there is a tight set S such that
AS ∈ span(L) and L ∪ {S} is a laminar family.
    By Corollary 23.17, if L is a maximal laminar family of tight sets with lin-
early independent incidence vectors, then |L| = m. This establishes Theorem
23.12.


23.4 A counting argument
The characterization of extreme point solutions given in Theorem 23.12 will
yield Theorem 23.6 via a counting argument. Let x be an extreme point
solution and L be the collection of tight sets established in Theorem 23.12.
The number of sets in L equals the number of edges in G, i.e., m. The proof
is by contradiction. Suppose that for each edge e, xe < 1/2. Then, we will
show that G has more than m edges.
    Since L is a laminar family, it can be viewed as a forest of trees if its
elements are ordered by inclusion. Let us make this precise. For S ∈ L, if S
is not contained in any other set of L, then we will say that S is a root set. If
S is not a root set, we will say that T is the parent of S if T is a minimal set
in L containing S; by laminarity of L, T is unique. Further, S will be called
a child of T . Let the relation descendent be the reﬂexive transitive closure of
the relation “child”. Sets that have no children will be called leaves. In this
manner, L can be partitioned into a forest of trees, each rooted at a root set.
For any set S, by the subtree rooted at S we mean the set of all descendents
of S.
    Edge e is incident at set S if e ∈ δG (S). The degree of S is deﬁned to be
|δG (S)|. Set S owns endpoint v of edge e = (u, v) if S is the smallest set of L
containing v. The subtree rooted at set S owns endpoint v of edge e = (u, v)
if some descendent of S owns v.
    Since G has m edges, it has 2m endpoints. Under the assumption that
∀e, xe < 1/2, we will prove that for any set S, the endpoints owned by the
subtree rooted at S can be redistributed in such a way that S gets at least
3 endpoints, and each of its proper descendents gets 2 endpoints. Carrying
222     23   Steiner Network

out this procedure for each of the root sets of the forest, the total number of
endpoints in the graph must exceed 2m, leading to a contradiction.
    We have assumed that ∀e : 0 < xe < 1/2. For edge e, deﬁne ye = 1/2−xe ,
the halves complement of e. Clearly, 0 < ye < 1/2. For S ∈ L deﬁne its
corequirement to be
                                  1
      coreq(S) =            ye =     |δG (S)| − f (S).
                                   2
                   e∈δ(S)


Clearly, 0 < coreq(S) < |δG (S)|/2. Furthermore, since |δG (S)| and f (S) are
both integral, coreq(S) is half-integral. Let us say that coreq(S) is semi-
integral if it is not integral, i.e., if coreq(S) ∈ {1/2, 3/2, 5/2, . . .}. Since f (S)
is integral, coreq(S) is semi-integral iﬀ |δG (S)| is odd.
    Sets having a corequirement of 1/2 play a special role in this argument.
The following lemma will be useful in establishing that certain sets have this
corequirement.
Lemma 23.18 Suppose S has α children and owns β endpoints, where α +
β = 3. Furthermore, each child of S, if any, has a corequirement of 1/2.
Then, coreq(S) = 1/2.

Proof: Since each child of S has corequirement of 1/2, it has odd degree.
Using this and the fact that α + β = 3, one can show that S must have odd
degree (see Exercise 23.3). Therefore the corequirement of S is semi-integral.
Next, we show that coreq(S) is strictly smaller than 3/2, thereby proving the
lemma. Clearly,
                                                      
      coreq(S) =            ye ≤        coreq(S  ) +       ye ,
                   e∈δ(S)          S                   e


where the ﬁrst sum is over all children S  of S, and the second sum is over
all edges e having an endpoint in S. Since ye is strictly smaller than 1/2, if
β > 0, then coreq(S) < 3/2. If β = 0, all edges incident at the children of S
cannot also be incident at S, since otherwise the incidence vectors of these
four sets will be linearly dependent. Therefore,
                   
      coreq(S) <         coreq(S  ) = 3/2.
                    S

                                                                                     ✷
   The next two lemmas place lower bounds on the number of endpoints
owned by certain sets.
Lemma 23.19 If set S has only one child, then it must own at least two
endpoints.
                                                23.4   A counting argument       223

Proof: Let S  be the child of S. If S has no endpoint incident at it, the set
of edges incident at S and S  must be the same. But then AS = AS  , leading
to a contradiction. S cannot own exactly one endpoint, because then δx (S)
and δx (S  ) will diﬀer by a fraction, contradicting the fact that both these
sets are tight and have integral requirements. The lemma follows.           ✷

Lemma 23.20 If set S has two children, one of which has a corequirement
of 1/2, then it must own at least one endpoint.

Proof: Let S  and S  be the two children of S, with coreq(S  ) = 1/2.
Suppose S does not own any endpoints. Since the three vectors AS , AS  , and
AS  are linearly independent, the set of edges incident at S  cannot all be
incident at S or all be incident at S  . Let a denote the sum of ye ’s of all edges
incident at S  and S, and let b denote the sum of ye ’s of all edges incident at
S  and S  . Thus, a > 0, b > 0, and a + b = coreq(S) = 1/2.
    Since S  has a semi-integral corequirement, it must have odd degree.
Therefore, the degrees of S and S  have diﬀerent parities, and these two sets
have diﬀerent corequirements. Furthermore, coreq(S) = coreq(S  ) + a − b.
Therefore, coreq(S) − coreq(S  ) = a − b. But −1/2 < a − b < 1/2. Therefore,
S and S  must have the same corequirement, leading to a contradiction. ✷

Lemma 23.21 Consider a tree T rooted at set S. Under the assumption
that ∀e, xe < 1/2, the endpoints owned by T can be redistributed in such
a way that S gets at least 3 endpoints, and each of its proper descendents
gets 2 endpoints. Furthermore, if coreq(S) = 1/2, then S must get at least 4
endpoints.

Proof: The proof is by induction on the height of tree T . For the base case,
consider a leaf set S. S must have degree at least 3, because otherwise an
edge e incident at it will have xe ≥ 1/2. If it has degree exactly 3, coreq(S) is
semi-integral. Further, since coreq(S) < |δG (S)|/2 = 3/2, the corequirement
of S is 1/2. Since S is a leaf, it owns an endpoint of each edge incident at it.
Therefore, S has the required number of endpoints.
    Let us say that a set has a surplus of 1 if 3 endpoints have been assigned to
it and a surplus of 2 if 4 endpoints have been assigned to it. For the induction
step, consider a nonleaf set S. We will prove that by moving the surplus of
the children of S and considering the endpoints owned by S itself, we can
assign the required number of endpoints to S. There are four cases:
 1. If S has 4 or more children, we can assign the surplus of each child to S,
    thus assigning at least 4 endpoints to S.
 2. Suppose S has 3 children. If at least one of them has a surplus of 2, or if S
    owns an endpoint, we can assign 4 endpoints to S. Otherwise, each child
    must have a corequirement of half, and by Lemma 23.18, coreq(S) = 1/2
    as well. Thus, assigning S the surplus of its children suﬃces.
224    23   Steiner Network

 3. Suppose S has two children. If each has a surplus of 2, we can assign 4
    endpoints to S. If one of them has surplus 1, then by Lemma 23.20, S
    must own at least one endpoint. If each child has a surplus of 1 and S
    owns exactly one endpoint, then we can assign 3 endpoints to S, and this
    suﬃces by Lemma 23.18. Otherwise, we can assign 4 endpoints to S.
 4. If S has one child, say S  , then by Lemma 23.19, S owns at least 2
    endpoints. If S owns exactly 2 endpoints and S  has surplus of exactly 1,
    then we can assign 3 endpoints to S; by Lemma 23.18, coreq(S) = 1/2,
    so this suﬃces. In all other cases, we can assign 4 endpoints to S.
                                                                              ✷



23.5 Exercises

23.1 Give an LP-relaxation for the Steiner network problem, having poly-
nomially many constraints over polynomially many variables. 
Hint: Pick a minimum cost set of edges so as to route n2 independent
commodities, one for each pair of vertices. Each ﬂow should be at least as
large as the connectivity requirement of this pair. The extent to which an
edge is picked bounds the amount of each commodity that can ﬂow through
this edge.

23.2 Show that a function f : 2V → Z+ satisfying the following conditions
is submodular: f (V ) = 0, f is symmetric, i.e., for any set A ⊆ V f (A) =
f (V −A), and for every two sets A, B ⊆ V f (A)+f (B) ≥ f (A∩B)+f (A∪B).

23.3 Prove that set S in Lemma 23.18 must have odd degree. (Consider the
following possibilities: S owns endpoint v of edge (u, v) that is incident at S,
S owns endpoint v of edge (u, v) that is incident at a child of S, and an edge
is incident at two children of S.)

23.4 Prove that there must be a set in L that has degree at most 3, and
thus some edge must have xe ≥ 1/3. The counting argument required for
this is much simpler. Notice that this fact leads to a factor 3 algorithm. (The
counting argument requires the use of Lemma 23.19.)

   The next two exercises develop a factor 2Hk algorithm for the Steiner
network problem using the primal–dual schema, where k is the largest con-
nectivity requirement speciﬁed in the instance. For simplicity, assume that
the upper bounds, ue , are 1 for each edge e.

23.5 (Williamson, Goemans, Mihail, and Vazirani [258]) Say that a function
h : 2V → {0, 1} is uncrossable if h(V ) = 0, and for any two sets A, B ⊂ V , if
                                                         23.5   Exercises      225

h(A) = h(B) = 1 then h(A−B) = h(B −A) = 1 or h(A∩B) = h(A∪B) = 1.
Exercise 22.7 asked for a factor 2 approximation algorithm for IP (22.1)
for the case that f was a proper function. In this exercise, we will extend
this further to the case that f is an uncrossable function. Now, we need
to enhance the last step of Algorithm 22.3; the pruning step needs to be
done using reverse delete. Again, F denotes the forest of edges picked by the
algorithm. Let us say that a set A ⊂ V is unsatisﬁed w.r.t. the picked edges
F if h(A) = 1 and δF (A) = ∅. A minimal unsatisﬁed set will be said to be
active. The algorithm is as follows.


 Algorithm 23.22 (Uncrossable function)
  1. (Initialization) F ← ∅; for each S ⊆ V , yS ← 0.
  2. (Edge augmentation) while there exists an unsatisﬁed set do:
       simultaneously raise yS for each active set S, until some edge e goes
          tight;
       F ← F ∪ {e}.
  3. Let e1 , e2 , . . . , el be the ordered list of edges in F .
  4. (Reverse delete) For j = l downto 1 do:
     If F − {ej } satisﬁes h, then F ← F − {ej }.
  5. Return F .


    Show that in each iteration, active sets must be disjoint. Assuming that
active sets can be eﬃciently found, show that Algorithm 23.22 ﬁnds a primal
solution of cost at most twice the dual, i.e.,
                    
            ce ≤ 2       yS .
      e∈F            S

Hint: Corresponding to each edge e ∈ F , there must be a set A ⊂ V such
that h(A) = 1 and δF (A) = {e}. Call such a set a witness for e. A family C
consisting of a witness for each e ∈ F is called a witness family. Include V in
this family. Show, by uncrossing, that C can be assumed to be laminar and
therefore can be viewed as a tree. Use this to prove that in each iteration,
the average degree of active sets is at most two, as in Lemma 22.8.

23.6 Give an example to show that if reverse delete is replaced by a forward
delete, then the approximation factor for Algorithm 23.22 can be unbounded
for some uncrossable function.

23.7 (Goemans, Goldberg, Plotkin, Shmoys, Tardos, and Williamson [102])
We will solve the Steiner network problem in k phases, numbered 0, 1, . . . , k −
1. In each phase, we will pick a forest from the remaining graph. The solution
will be the union of the k forests. Let Fp−1 denote the set of edges picked
226    23     Steiner Network

in phases numbered 0, 1, . . . , p − 1. At the beginning of the pth phase, deﬁne
the deﬁciency of set S ⊂ V to be max{f (S) − |δFp−1 (A)|, 0}. The ﬁrst p − 1
phases ensure that every set has deﬁciency at most k − p. In the pth phase,
deﬁne function h as

                 1 if deﬁciency(S) = k − p
      h(S) =
                 0 otherwise

Show that h is an uncrossable function. Show that Algorithm 23.22 can be
implemented in polynomial time for this uncrossable function, i.e., active
sets can be found in polynomial time. Let F be the set of edges picked by
Algorithm 23.22 from E −Fp−1 , and y be the dual solution constructed when
run with function h. Construct the dual program to LP (23.2), and show that
there is a feasible solution, say d, to this program such that
                               2
            ce ≤ 2       yS ≤       g(d),
                                k−p
      e∈F            S


where g(d) is the objective function value of dual solution d. Adding over all
k phases leads to the required factor.
Hint: Use a max-ﬂow algorithm for ﬁnding active sets. The dual pro-
gram will have a variable ze for each edge e. For edges e ∈ Fp−1 , set
ze = S: e∈δ(S) yS , for constructing a dual feasible solution.

23.8 Give an inﬁnite family of graphs to show that the performance guar-
antee of the algorithm in Exercise 23.7 is tight within constant factors.

     The following deﬁnitions will be useful for the next three exercises. These
notions are connected to the theme of this chapter, i.e., small subgraphs with
speciﬁed numbers of disjoint paths, via Menger’s theorem (see Exercise 12.5.
An undirected graph is said to be k-vertex (k-edge) connected if it has at least
k+1 vertices, and the removal of any set of at most k−1 vertices (edges) from
it leaves a connected graph. A directed graph is said to be k-vertex (k-edge)
connected if it has at least k + 1 vertices, and the deletion of any set of at
most k − 1 vertices (edges) leaves a strongly connected graph.

23.9 (Cheriyan and Thurimella [42]) This exercise develops a 1 + 2/k factor
algorithm for the following problem.
Problem 23.23 (Minimum k-vertex connected subgraph) Given a
nonnegative integer k and an undirected graph G = (V, E) that is k-vertex
connected, ﬁnd a minimum cardinality set E  ⊂ E such that the subgraph
G = (V, E  ) is k-vertex connected.
    Let G = (V, E) be k-vertex connected. We will say that edge e ∈ E is
critical if its removal leaves a graph that is not k-vertex connected. A simple
cycle C in G is critical if every edge on C is critical. A theorem of Mader,
                                                                   23.5    Exercises       227

which states that a critical cycle in G must have a vertex of degree exactly
k, is central to the algorithm.


 Algorithm 23.24 (k-vertex connected subgraph)
  1. Find a minimum cardinality set M ⊂ E such that
     ∀v ∈ V : degM (v) ≥ k − 1.
  2. Find a minimal set F such that M ∪ F is k-vertex connected.
  3. Output G = (V, M ∪ F ).




 1. Give a polynomial time algorithm for Step 1 of Algorithm 23.24. Observe
    that |M | ≤ OPT.
    Hint: Use a b-matching algorithm on the complement of G. Given an
    undirected graph G = (V, E) and a function b : V → Z+ specifying an
    upper bound for each vertex, the b-matching problem asks for a maximum
    cardinality set M ⊆ E such that ∀v ∈ V, degM (v) ≤ b(v). This problem
    is in P.
 2. Give an eﬃcient implementation for Step 2 of Algorithm 23.24.
 3. Use Mader’s theorem to show that F must be acyclic, and hence |F | ≤
    |V | −1. Use this to show that Algorithm 23.24 achieves an approximation
    factor of 1 + 2/k.
    Hint: Use the fact that k|V |/2 is a lower bound on OPT.


23.10 (Cheriyan and Thurimella [42]) Consider the problem of ﬁnding a
minimum k-vertex connected subgraph of a directed graph. Give an algorithm
similar to that in Exercise 23.9 for achieving factor 1 + 2/k for this problem.
Use the following two facts.
 1. In a directed graph, an alternating cycle, C, is an even length sequence of
    distinct edges (v0 , v1 )(v2 , v1 )(v2 , v3 )(v4 , v3 ) . . . (vm−1 , vm )(v0 , vm ), where
    vertices are allowed to repeat. Notice that alternate vertices on C have
    two out-edges (two in-edges). Vertices having two out-edges (two in-
    edges) will be called C-out (C-in) vertices. Mader showed that if G is
    a k-vertex connected directed graph containing an alternating cycle C,
    each of whose edges is critical, then C contains either a C-out vertex
    having out-degree exactly k or a C-in vertex having in-degree exactly k.
 2. Given a directed graph G = (V, E), deﬁne its associated bipartite graph
    H to be the following. Corresponding to each vertex v ∈ V , H has two
    vertices, v− and v+ , and corresponding to each edge (u, v) ∈ E, H has
    the edge (u+ , v− ). There is an alternating cycle in G iﬀ its associated
    bipartite graph contains a cycle.
228      23     Steiner Network

23.11 (Khuller and Vishkin [172], using Edmonds [72]) This exercise devel-
ops a factor 2 algorithm for the following problem.
Problem 23.25 (Minimum k-edge connected subgraph) Given an
undirected graph G = (V, E), a function w : E → Q+ , and an integer k, ﬁnd
a minimum weight subgraph of G that is k-edge connected.
1. Let r ∈ V be any vertex of G. Consider the problem of ﬁnding a minimum
   weight subgraph G of G such that for each vertex v ∈ V , there are k
   edge-disjoint paths from r to v in G . Show that this problem is the same
   as Problem 23.25, i.e., any solution to one is also a solution to the other.
2. Let G = (V, E) be an edge-weighted directed graph and r ∈ V be one of
   its vertices. A set E  ⊆ E is said to be an r-arborescence if every vertex,
   other than r, has in-degree 1. In eﬀect, an r-arborescence is a spanning
   tree directed out of r. Deﬁne the r-connectivity of G to be

              max{k | ∀v ∈ V ∃k edge-disjoint paths from r to v in G}.

   Edmonds showed that the maximum number of edge-disjoint r-arbores-
   cences in G is equal to the r-connectivity of G. Use this to show that
   the problem of ﬁnding a minimum weight subgraph of G that has an r-
   connectivity of k is the same as the problem of ﬁnding a minimum weight
   subgraph of G that has k edge-disjoint r-arborescences.
3. Edmonds showed that the edges of a directed graph G = (V, E) can be
   partitioned into k edge-disjoint r-arborescences iﬀ, on ignoring directions,
   E can be partitioned into k spanning trees, and the in-degree of every
   vertex, other than r, is exactly k. Use this characterization to show that
   the problem of ﬁnding a minimum weight subgraph of G that has k edge-
   disjoint arborescences can be solved in polynomial time.
   Hint: This problem can be expressed as a matroid intersection problem,
   the two matroids being a partition matroid and the k-fold union of a
   graphic matroid (which is also a matroid).
4. Let G = (V, E) be an edge-weighted undirected graph and r ∈ V be one
   of its vertices. Let OPT(G) denote the weight of an optimal solution to
   Problem 23.25 on instance G. Obtain graph H by bidirecting G, i.e., by
   replacing each edge (u, v) ∈ E with the two edges (u → v) and (v → u),
   each having the same weight as (u, v). Let OPT(H) denote the weight
   of a minimum weight subgraph of H that can be partitioned into k r-
   arborescences. Show that

              OPT(G) ≤ OPT(H) ≤ 2 · OPT(G).

      Use this to obtain a factor 2 approximation algorithm for Problem 23.25.

23.12 (Goemans and Bertsimas [101]) The metric Steiner network problem
is the Steiner network problem with the restrictions that G is a complete
                                                            23.5   Exercises     229

graph, the cost function on edges satisﬁes the triangle inequality, and ue = ∞
for each edge. It generalizes the metric Steiner tree problem to arbitrary
connectivity requirements. For D ⊆ V , deﬁne LPS (D) to be LP-relaxation
(23.2), together with a set of equality constraints for vertices in D, as follows.
                    
      minimize            ce xe                                                (23.4)
                    e∈E
                      
      subject to                  xe ≥ f (S),         S⊆V
                    e: e∈δ(S)
                       
                                   xe = f ({v}),      v∈D
                    e: e∈δ({v})
                    xe ≥ 0,                           e∈E

   It turns out that the equality constraints are redundant for the metric
Steiner network problem. For any choice of D ⊆ V , an optimal solution to
LPS (D) is also an optimal solution to LPS (∅). This is called the parsimo-
nious property. Let us say that a vertex v is Steiner if it has no connectivity
requirements, i.e., if ∀v ∈ V, r(u, v) = 0. Use the parsimonious property to
prove that there is a fractional optimal solution to the metric Steiner network
problem which has no edges incident at Steiner vertices.

23.13 Consider the following integer program for the traveling salesman
problem (Problem 3.5).
                    
      minimize            ce xe                                                (23.5)
                    e∈E
                             
      subject to                            xe = 2,   v∈V
                    e: e incident at v
                      
                                  xe ≥ 2,             S⊂V
                    e: e∈δ(S)

                    xe ∈ {0, 1},                      e∈E

Show that optimal solutions to this integer program are optimal TSP tours.
The linear relaxation of this program is called the subtour elimination LP-
relaxation for TSP.
    The rest of this exercise deals with the special case of metric TSP and de-
velops a proof that the solution found by Christoﬁdes’ algorithm, Algorithm
3.10, is within a factor of 3/2 of the optimal solution to this LP-relaxation.
 1. Give an example that puts a lower bound of (essentially) 4/3 on the
    integrality gap of this relaxation.
    Hint: Use the following graph.
230    23     Steiner Network
      t           t         t            t ...     t      t     t        t
                                                                       ✚
                                                                    ✚
                                                                  ✚
            t             t            t   ...     t     t      t✚
            ✚                                                     
          ✚                                                         
        ✚                                                             
      t✚      t             t            t ...     t      t     t      t

2. Let v1 be an arbitrary vertex in the given graph G = (V, E). Deﬁne a
   1-tree in G to be a spanning tree on the vertices V − {v1 }, together with
   two distinct edges incident at vertex v1 . Clearly, the cost of an optimal
   1-tree is a lower bound on the cost of an optimal TSP tour. LP (12.12)
   stated in Exercise 12.9 was an exact relaxation for the MST problem. Use
   it to obtain an exact LP-relaxation for the minimum 1-tree problem.
3. (Held and Karp [123]) Show that the cost of a minimum 1-tree is a lower
   bound on the cost of an optimal solution to the subtour elimination LP.
   Hint: Compare the LP obtained above for minimum 1-tree with the
   following equivalent formulation of the subtour elimination LP. (By e :
   e ∈ S we mean edges e that have both endpoints in S.)
                         
            minimize           ce xe                                   (23.6)
                         e∈E
                                  
            subject to                        xe = 2,   v∈V
                         e: e incident at v
                          
                                  xe ≤ |S| − 1,         S⊆V
                         e: e∈S
                         xe ≥ 0,                        e∈E

4. Use the parsimonious property, introduced in Exercise 23.12, to show
   that the equality constraints, on vertices, in the subtour elimination LP
   are redundant. (Observe that the LP obtained on removing these con-
   straints is also an LP-relaxation for the problem of ﬁnding a minimum
   cost spanning two-edge connected subgraph of G.)
5. For D ⊆ V , let LPT (D) denote the subtour elimination LP for GD , the
   subgraph of G induced on D. Let OPTf (LPT (D) denote the cost of an
   optimal solution to LPT (D). Show the following monotonicity property

            OPTf (LPT (D)) ≤ OPTf (LPT (V )).

   Hint: Use the relaxation without equality constraints.
6. Let D ⊆ V be of even cardinality. Show that the cost of a mini-
   mum cost perfect matching in the subgraph of G induced on D is
   ≤ 12 OPTf (LPT (D)).
   Hint: Use LP (12.9), introduced in Exercise 12.8, for matching, and LP
   (23.6) for TSP.
                                                          23.6   Notes   231

7. Show that the metric TSP solution found using Algorithm 3.10, is within
   a factor of 3/2 of the optimal solution to the subtour elimination LP.



23.6 Notes
The result of this chapter is due to Jain [137]. Cheriyan and Thurimella [42]
contains further results on ﬁnding small subgraphs of a given graph with a
speciﬁed connectivity, as well as references to Mader’s theorems. The subtour
elimination LP-relaxation for TSP was given in Dantzig, Ford, and Fulkerson
[59]. The result of Exercise 23.13 was ﬁrst established by Wolsey [260]. The
proof developed here is from Shmoys and Williamson [240].
