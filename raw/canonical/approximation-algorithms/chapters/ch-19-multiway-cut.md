---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-19"
chapter_number: 19
chapter_title: "Multiway Cut"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 173
source_page_end: 185
printed_page_start: 155
printed_page_end: 167
part_ids: ["approximation-algorithms-ch-19-part-020"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Multiway Cut

19 Multiway Cut




A simple combinatorial algorithm achieving an approximation factor of 2 −
2/k for the multiway cut problem, Problem 4.1, was presented in Chapter 4.
In this chapter we will use LP-rounding to improve the factor to 3/2.
    In Chapter 14 we mentioned the remarkable property of half-integrality,
possessed by LP-relaxations of certain NP-hard problems. The multiway cut
problem and its generalization, the node multiway cut problem, possess this
property. We will present a proof of this fact in Section 19.3. This is the only
avenue known for obtaining a constant factor approximation algorithm for
the latter problem.


19.1 An interesting LP-relaxation
The usual LP-relaxation for multiway cut has an integrality gap of 2 − 2/k
(see Exercise 19.2). The key to an improved approximation guarantee is a
clever LP-relaxation.
    Let ∆k denote the k −1 dimensional simplex. This is thek −1 dimensional
convex polytope in Rk deﬁned by {x ∈ Rk | x ≥ 0 and          i xi = 1}, where
xi is the ith coordinate of point x. The simplex ∆3 is shown below.

                           1111111111111
                   (0,0,1) 0000000000000
                          1111111111111
                          0000000000000
                          1111111111111
                          0000000000000
                          0000000000000
                          1111111111111
                          1111111111111
                          0000000000000
                          1111111111111
                          0000000000000
                          1111111111111
                          0000000000000
                                      (0,1,0)
                          1111111111111
                          0000000000000
                          1111111111111
                          0000000000000
                          0000000000000
                          1111111111111
                          1111111111111
                          0000000000000
                          1111111111111
                          0000000000000
                          1111111111111
                          0000000000000
                                          (1,0,0)
   The relaxation will map each vertex of G to a point in ∆k . Each of the k
terminals will be mapped to a distinct vertex of this simplex, i.e., to a unit
vector ei ∈ Rk . Let xv ∈ ∆k denote the point to which vertex v is mapped.
The length of an edge (u, v) ∈ E will be deﬁned to be half the 1 distance
between xu and xv . The entire relaxation is:
 156      19   Multiway Cut
                           
        minimize                  c(u, v)d(u, v)                                     (19.1)
                        (u,v)∈E

                                        k
                                     1 i
        subject to      d(u, v) =         |x − xiv |,     (u, v) ∈ E
                                     2 i=1 u
                        xv ∈ ∆k ,                         v∈V
                        xsi = ei ,                        si ∈ S

     In Lemma 19.1 we show that this relaxation is really a linear program. An
 integral solution to this relaxation maps each vertex of G to a vertex of the
 simplex, respectively. Each edge (u, v) has length either 0 or 1, depending on
 whether u and v are mapped to the same or diﬀerent vertices of the simplex.
 Edges of length 1 form a multiway cut. The cost of this cut is the objective
 function value of this integral solution. Thus, an optimal integral solution
 corresponds to an optimal multiway cut.
 Lemma 19.1 Relaxation (19.1) can be expressed as a linear program.

 Proof: For each edge (u, v), replace the ﬁrst constraint with:

        xiuv ≥ xiu − xiv , 1 ≤ i ≤ k

        xiuv ≥ xiv − xiu , 1 ≤ i ≤ k

                       k
                    1 i
        d(u, v) =        x
                    2 i=1 uv

 Since the objective function is being minimized, an optimal solution must
 satisfy xiuv = |xiu − xiv |. The rest of the constraints are clearly linear. ✷

  Example 19.2 In the example given below, the optimal fractional multiway
  cut is cheaper than than the optimal integral cut. The mapping of vertices
  to ∆3 in the optimal fractional solution is shown below; it achieves a cost of
  7.5. On the other hand, the optimal integral solution costs 8.
                 s1                                             s1
                  t                                              t (1, 0, 0)
                 ✂❇                                            ❙
             2
                ✂ ❇2                                                ❙
               ✂ ❇                        (.5, .5, 0)                 ❙      (.5, 0, .5)
             ✂      ❇                                 u t               ❙ tv
          u ✂t 1 ❇tv                                   ❡                ✪❙
          ✑❆         ✁◗◗ 2                           ❡                      ❙
      2 ✑                ◗                                             ✪
     ✑        1❆ ✁1        ◗                               ❡       ✪          ❙
 t✑✑ 2          ❆t✁     2    ◗t    (0, 1, 0) t               ❡t✪                 ❙t (0, 0, 1)
s2               w            s3             s2                 w                  s3
                                                            (0, .5, .5)                     ✷
                                        19.2    Randomized rounding algorithm              157

    The following property will greatly simplify matters:

Lemma 19.3 Let x be a feasible solution to relaxation (19.1). We may as-
sume w.l.o.g. that for each edge (u, v) ∈ E, xu and xv diﬀer in at most two
coordinates.

Proof: We will divide edges by adding new vertices in such a way that this
property holds and the cost of the solution remains unchanged.
     Suppose that (u, v) ∈ E and that xu and xv diﬀer in more than two
coordinates. Replace this edge by two new edges (u, w) and (w, v), where w
is a new vertex. Each of the new edges is of the same cost as c(u, v), thereby
ensuring that the cost of the integral optimal solution is unchanged. We show
below how to enforce d(u, v) = d(u, w) + d(w, v), thereby ensuring that the
cost of the fractional solution remains unchanged.
     Consider the coordinates in which xu and xv diﬀer. Let i be the coordinate
in which the diﬀerence is minimum. Without loss of generality, assume xiu <
xiv . Let α = xiv − xiu . There must be a coordinate j such that xju ≥ xjv + α.
We will deﬁne point xw as follows. The ith and jth coordinates of xw are
xiw = xiu and xjw = xjv + α. The remaining coordinates of xw are the same as
those of xv . Clearly, xw ∈ ∆k and d(u, v) = d(u, w) + d(w, v).
     Notice that u and w diﬀer in two coordinates and w and v diﬀer in fewer
coordinates than u and v. Therefore, each edge of E requires at most k − 2
such subdivisions to enforce the required property.                          ✷



19.2 Randomized rounding algorithm

Let x be an optimal solution to relaxation (19.1) satisfying the property
stated in Lemma 19.3, and let OPTf denote its cost. Let Ei denote the
subset of edges whose endpoints diﬀer in coordinate i, i.e., Ei = {(u, v) ∈
E | xiu = x   i
                v }. Clearly, each edge e with d(e) > 0 will lie in two of these sets.
Let Wi = e∈Ei c(e)d(e). Renumber the terminals so that Wk is the largest
of W1 , . . . , Wk . For ρ ∈ (0, 1), deﬁne

       B(si , ρ) = {v ∈ V | xiv ≥ ρ}.

     Algorithm 19.4 operates as follows. It picks ρ at random in (0, 1) and
σ at random from the two permutations (1, 2, . . . , k − 1, k) and (k − 1, k −
2, . . . , 1, k). It uses ρ and σ to construct a partition of V into k sets, V1 , . . . , Vk ,
ensuring that si ∈ Vi . Edges running between these sets will form the multi-
way cut.
     If σ is the ﬁrst (second) permutation, then these sets are constructed in the
order V1 , V2 , . . . , Vk (Vk−1 , Vk−2 , . . . , V1 , Vk ). If ρ > 1/2, the sets B(si , ρ) are
pairwise disjoint. Observe that in this case the partition is not aﬀected by σ,
158     19    Multiway Cut

because Vi is simply B(si , ρ) for 1 ≤ i ≤ k − 1, and Vk = V − (V1 ∪ · · · ∪ Vk−1 ).
If ρ ≤ 1/2, the sets B(si , ρ) overlap and σ plays a role, as illustrated in the
ﬁgure below for k = 3.


             sr1                            sr1                        sr1
             ✔❚                             ✔❚                         ✔❚
          ✔ V1 ❚                            ✔ ❚                       ✔ ❚
         ✔      ❚                          ✔ V ❚                     ✔❚ V ❚
                                             1
       ✔          ❚                    ✔          ❚                 ✔ ❚ 1 ❚
      ✔❚            ❚               ✔              ❚               ✔ V ❚      ❚
     ✔ ❚      V3     ❚             ✔                             ✔    2
                                                                          ❚ V❚
      V2                               V2    ❚ V3❚
s2 ✔
   r      ❚            ❚ rs3   s2 ✔
                                  r           ❚      ❚rs3   s2 ✔
                                                               r           ❚ 3 ❚ rs3
         ρ ≥ 1/2                 ρ < 1/2, σ = (1, 2, 3)       ρ < 1/2, σ = (2, 1, 3)


  Algorithm 19.4 (Multiway cut)
   1. Compute an optimal solution, x, to relaxation (19.1).
   2. Renumber the terminals so that Wk is largest among W1 , . . . , Wk .
   3. Pick uniformly at random ρ ∈ (0, 1) and
      σ ∈ {(1, 2, . . . , k − 1, k), (k − 1, k − 2, . . . , 1, k)}.
   4. For i = 1 to k − 1: Vσ(i) ← B(si , ρ) − j<i Vσ(j) .
   5. Vk ← V − i<k Vi .
   6. Let C be the set of edges that run between sets in the partition
      V1 , . . . , Vk . Output C.



   We will show that the expected cost of the multiway cut produced by the
algorithm, E[c(C)], is at most (1.5 − 1/k) · OPTf . The following lemma will
be critical.
Lemma 19.5 If e ∈ E − Ek , Pr[e ∈ C] ≤ 1.5 d(e),
and if e ∈ Ek , Pr[e ∈ C] ≤ d(e).

Proof: Suppose e ∈ E −Ek . Let e = (u, v), and let i and j be the coordinates
in which xu and xv diﬀer. There are two cases: the intervals [xiu , xiv ] and
[xjv , xju ] either overlap or they are disjoint. These two cases are shown below.
Note that in either case the two intervals have the same length since xiv −xiu =
xju − xjv = d(e). Intervals α and β are deﬁned in the ﬁgure below for the two
cases.
                                        19.2    Randomized rounding algorithm         159

                    ✛            α       ✲       ✛          β       ✲

    0            xiu                      xiv xjv                    xju          1




                                 α                          β
                    ✛                    ✲✛                         ✲


    0            xiu                      xjv xiv                    xju          1


    Observe that the vertices u and v can end up in one of three sets, Vi , Vj ,
or Vk . Furthermore, if ρ ∈ [0, 1] − (α ∪ β), then both vertices will end up in
the same set, and edge e will not be in the cut. Clearly, Pr[ρ ∈ (α ∪ β)] =
|α| + |β| ≤ 2d(e).
    The critical observation that leads to the desired bound is that in the
event ρ ∈ α and σ(j) < σ(i), u and v will both be put in the set Vj , and
thus e will not be in the cut. Clearly, the probability of this event is |α|/2.
Therefore

        Pr[e ∈ C] = |β| + |α|/2 ≤ 1.5 d(e).

    Next, suppose that e ∈ Ek , and that its endpoints diﬀer in coordinates i
and k. In this case σ(i) < σ(k), and u and v will end up in diﬀerent sets only
if ρ falls between xiu and xiv . The probability of this is d(e).           ✷

Lemma 19.6 The multiway cut, C, output by Algorithm 19.4 satisﬁes

        E[c(C)] ≤ (1.5 − 1/k)OPTf .

                                                             
Proof: Clearly, C forms a multiway cut. Now, OPTf = e c(e)d(e). Since
                                                         k
each edge with nonzero length is in two of the sets Ei , i=1 Wi = 2 · OPTf .
Since k was chosen so that Wk is the largest of these sets, Wk ≥ (2/k)·OPTf .
Therefore
                                                                 
        E[c(C)] =         c(e)Pr[e ∈ C] =        c(e)Pr[e ∈ C] +        c(e)Pr[e ∈ C]
                    e∈E                  e∈E−Ek                    e∈Ek
                        k−1
                                               k
                                                
                ≤ 1.5           Wi + Wk = 1.5         Wi − 0.5Wk
                          i=1                   i=1
                ≤ (1.5 − 1/k) · OPTf

where the ﬁrst inequality follows from Lemma 19.5.                                      ✷
160     19   Multiway Cut

    Lemma 19.6 places an upper bound of 1.5 − 1/k on the integrality gap of
relaxation 19.1 (see the notes in Section 19.5 for references to a slightly better
                                                                              1
result). The worst lower bound know on the integrality gap is 8/(7 + k−1        );
Example 19.2 places a lower bound of 16/15.
    The bound on the expected weight of the multiway cut established in
Lemma 19.6 can be converted into a high probability statement using stan-
dard techniques (see Exercises 1.10 and 19.4). Hence we get
Theorem 19.7 There is a 3/2 factor randomized approximation algorithm
for the multiway cut problem.



19.3 Half-integrality of node multiway cut
The following is a generalization of the multiway cut problem, in the sense
that there is an approximation factor preserving reduction from the multiway
cut problem to it (see Exercise 19.13).
Problem 19.8 (Node multiway cut) Given a connected, undirected graph
G = (V, E) with an assignment of costs to vertices, c : V → R+ , and a set of
terminals S = {s1 , s2 , . . . , sk } ⊆ V that form an independent set in G, a node
multiway cut is a subset of V − S whose removal disconnects the terminals
from each other. The node multiway cut problem asks for the minimum cost
such subset.
    We will show that the relaxation to the following integer program always
has a half-integral optimal solution. A factor 2−2/k approximation algorithm
will follow from this fact (see Exercise 19.11). In this program we have intro-
duced a 0/1 variable dv for each vertex v ∈ V − S, which indicates whether
vertex v has been picked. Let P denote the set of all paths running between
distinct terminals. There is a constraint for each path p in P – it ensures that
at least one vertex is picked from each path.
                      
      minimize               cv dv
                    v∈V −S
                    
      subject to          dv ≥ 1,    p∈P
                    v∈p
                    dv ∈ {0, 1},     v ∈V −S

    The LP-relaxation is given below. As before, we will interpret dv ’s as
distance labels. With respect to an assignment to these distance labels, let us
deﬁne the length of a path to be the sum of distance labels of nonterminals
on this path. The distance between a pair of vertices will be the length of
the shortest path between them. A solution, d, is feasible only if the distance
between every pair of terminals is at least 1.
                                   19.3   Half-integrality of node multiway cut     161
                      
      minimize                  cv dv                                             (19.2)
                     v∈V −S
                     
      subject to           dv ≥ 1,        p∈P
                     v∈p
                     dv ≥ 0,              v ∈V −S

   As in Chapter 18, the dual will be interpreted as seeking a maximum
multicommodity ﬂow. The commodities ﬂow between distinct terminals, and
the constraint is that the total amount of ﬂow through a vertex be bounded
by its cost.
                     
      maximize             fp                                                     (19.3)
                     p∈P
                     
      subject to             fp ≤ ce ,     v ∈V −S
                     p:v∈p
                     fp ≥ 0,               p∈P

    Let d be an optimal solution to LP (19.2). We will show how to obtain,
eﬃciently, a half-integral optimal solution from d . For the purposes of proof,
let f be an optimal solution to the dual LP. Complementary slackness con-
ditions give:
Primal conditions: For each v ∈ V − S, if dv > 0 then v must be saturated.
Dual conditions: For each path p, if fp > 0 then the length of p is exactly
1.
   Consider graph G with distance labels on vertices v ∈ V − S speciﬁed
by d. For each terminal si , deﬁne its region Si to be the set of vertices
reachable from si by paths of length zero (we will assume that si ∈ Si ).
Deﬁne the boundary, Bi , of Si to be all vertices that are adjacent to Si , i.e.,
Bi = {v ∈ Si | for some u ∈ Si , (u, v) ∈ E}. The feasibility of d ensures that
the k regions are disjoint and the boundaries do not contain any terminals.
Claim 19.9 Suppose v ∈ Bi ∩ Bj for i = j. Then dv = 1.

Proof: Clearly there is a path from si to sj on which v is the only vertex
having a positive distance label. The claim follows from the feasibility of d.
✷
                 k
    Let M = i=1 Bi be the set of boundary vertices. Partition this into two
sets: M int being boundary vertices that occur in two or more boundary sets,
and M disj being the rest; each vertex in M disj is in a unique boundary set.
By Claim 19.9, each vertex in M int has distance label of 1.
162    19   Multiway Cut

Lemma 19.10 Let p be a path between two distinct terminals such that fp >
0. Then, from the vertices in M , p uses either exactly one vertex of M int or
exactly two vertices of M disj .

Proof: By the dual complementary slackness condition, the length of p must
be exactly 1. Thus, if p uses a vertex of M int , then it cannot have any other
vertices of M on it.
    Suppose p uses three or more vertices of M disj . Assume that p runs from
si to sj and that u and w are the ﬁrst and last vertices of M disj on p,
respectively. Let v be any intermediate vertex of M disj on p. Since v ∈ M disj ,
v must be in a unique boundary, say Bk ; k = i or k = j are possible.


                                           sk




                                       v
                            u
              si                                                   sj

                                                    w

    Let q be a path connecting v to sk via vertices in Sk ; such a path must
exist since v ∈ Bk . Now consider the following two paths: the ﬁrst consists of
the part of the path p from si to v followed by q, and the second consists of
the reverse of q followed by the part of p from v to sj . At least one of these
is a valid path running between distinct terminals (even if k = i or k = j).
Moreover, since it is missing at least one of the positive distance label vertices
of p, it must have length strictly less than 1. This contradicts the feasibility
of d. The lemma follows.                                                        ✷
   Let h be a solution to LP (19.2) that assigns distance labels of 1 to each
vertex in M int , 1/2 to each vertex in M disj , and 0 to all remaining vertices.
Lemma 19.11 h is an optimal solution to LP (19.2).

Proof: Any valid path, p, from terminal si to sj must use vertices of both
boundary sets Bi and Bj . Suppose it uses v ∈ Bi ∩Bj . By deﬁnition v ∈ M int ,
and so hv = 1. Otherwise, it uses two vertices of M disj . In either case the
length of p is at least 1, thus showing that h is a feasible solution.
   Next we will show that the objective function value of h is the same as
that of ﬂow f , thereby showing that h is optimal. Partition paths carrying
nonzero ﬂow in f into two sets: P1 consists of paths that use one vertex of
M int and P2 consists of paths that use two vertices of M disj . By Lemma 19.10
these are the only two possibilities. By the primal complementary slackness
conditions and the optimality of d, each vertex in M is saturated by f .
                                                                   19.4   Exercises   163
                                                               
Therefore,the total ﬂow carried by paths in P1 is             v∈M int cv and by paths
in P2 is 12 v∈M disj cv . Hence the total ﬂow is

                     1                     
               cv +                  cv =            hv cv .
                      2
     v∈M int              v∈M disj          v∈V −S


This proves the lemma.                                                                 ✷
   Clearly h can be obtained from an optimal solution, d, to LP (19.2) in
polynomial time. This gives:

Theorem 19.12 LP (19.2) always has a half-integral solution. Moreover,
any optimal solution can be converted into such a solution in polynomial
time.



19.4 Exercises
In Chapter 4 we presented a 2 − 2/k factor algorithm for the minimum mul-
tiway cut problem by comparing the solution found to the integral optimal
solution. In the next two exercises we develop an algorithm with the same
guarantee using LP-duality.

19.1 Given terminals s1 , . . . , sk , consider the multicommodity ﬂow problem
kwhich each pair of terminals can form a source–sink pair. Thus there are
in
 2 commodities. Give an LP for maximizing this multicommodity ﬂow and
obtain the dual LP. The dual seeks a distance label assignment for edges
satisfying the triangle inequality and ensures that the distance between any
two terminals is at least 1. An optimal solution to the dual can be viewed as
a fractional multiway cut.

19.2 Consider the following algorithm for ﬁnding a multiway cut. Solve the
dual LP to obtain an optimal fractional multiway cut. This gives a distance
label assignment, say d. Pick ρ at random in the interval [0, 12 ]. An edge
(u, v) is picked iﬀ for some terminal s, d(u, s) ≤ ρ ≤ d(v, s). Prove that
the expected cost of the cut picked is at most twice the optimal fractional
multiway cut. Derandomize this algorithm, and give a modiﬁcation to make
it a factor 2 − 2/k algorithm.
Hint: Show that for each edge (u, v), the probability that it is picked is
bounded by 2 · d(u, v).

19.3 In an attempt to improve the factor of the previous algorithm, suppose
we choose ρ at random in the interval [0, 1]. What goes wrong? How is this
rectiﬁed in Algorithm 19.4?
164     19   Multiway Cut

19.4 Derive Theorem 19.7 from Lemma 19.6.
Hint: Lemma 19.6 implies that Pr[c(C) ≤ 1.5 · OPTf ] ≥ 2/k ≥ 2/n. Run
Algorithm 19.4 polynomially many times and output the best cut.

19.5 How does the approximation guarantee of the algorithm change if σ is
picked to be a random permutation from Sk ?

19.6 (Y. Rabani) For the case k = 3, replace the randomized rounding
procedure of Algorithm 19.4 with the following. Pick ρ1 and ρ2 independently
and uniformly from (0, 1). Pick one of the three dimensions at random, say i.
Merge with si all nonterminals v satisfying xiv ≥ ρ1 . Arbitrarily pick one of
the remaining two dimensions, say j, and denote the third dimension by k.
Merge with sj all remaining nonterminals v satisfying xjv +xiv /2 ≥ ρ2 . Finally,
merge with sk all remaining nonterminals. Show that this modiﬁed algorithm
achieves an approximation guarantee of 7/6 for the 3-way cut problem.

19.7 We present another relaxation for the multiway cut problem for which
the worst integrality gap known is no worse than that for LP (19.1) (see
also Chapter 30). Given an undirected graph G = (V, E) with costs on edges,
obtain the directed graph H by replacing each edge (u, v) of G by two directed
edges (u → v) and (v → u), each having the same cost as (u, v). Assign
a 0/1 indicator variable de to each edge e in H. Suppose the terminals are
numbered s1 , . . . , sk in some order. Let P be the collection of all simple paths
from a lower-numbered terminal to a higher-numbered terminal. Consider
the following bidirected integer programming formulation for the multiway
cut problem.
                    
      minimize            c(e)de                                            (19.4)
                    e∈H
                    
      subject to          de ≥ 1,   p∈P
                    e∈p
                    de ∈ {0, 1},    e∈H

 1. Show that an optimal solution to IP (19.4) yields an optimal solution to
    the multiway cut problem.
 2. Obtain the LP-relaxation and dual program. Give a good physical inter-
    pretation of the dual.
 3. Show that the graph given in Example 19.2 has an integrality gap of
    16/15 for this relaxation as well (by showing a primal and dual solution
    of cost 7.5).
 4. Show that the cost of the optimal solution to the integer program and
    the relaxation is independent of the ordering imposed on the terminals.

19.8 Consider Algorithm 4.3 for the multiway cut problem. Show that the
analogous algorithm for the node multiway cut problem, based on isolating
                                                                  19.4    Exercises   165

cuts, does not achieve a constant factor. What is the best factor you can
prove for this algorithm?

19.9 The multiway cut problem also possesses the half-integrality property.
Give an LP for the multiway cut problem similar to LP (19.2), and prove
this fact.

19.10 Show that the lower bound on OPT given by LP (19.2) can be smaller
by a factor of 2 − 2/k by giving a graph in which the optimal node multiway
cut is 2 − 2/k times bigger than the maximum ﬂow.

19.11 Theorem 19.12 leads directly to a factor 2 approximation algorithm
for the node multiway cut problem, by rounding up the halves in a half-
integral solution. Obtain a factor 2 − 2/k algorithm, and give a family of
tight examples for this algorithm.
Hint: Not all vertices of M disj are required for obtaining a multiway cut.
For the tight example, consider the following graph.
                                                   s3
                         s2

                                               2            ..
                                  2                           .


                 s1           2                         2                sk
                                        k+ ε


19.12 Consider the following problem.
Problem 19.13 (Directed multiway cut) Given a directed graph G =
(V, E) with an assignment of capacities to edges, c : E → R+ , and a set of
terminals S = {s1 , s2 , . . . , sk } ⊆ V , a directed multiway cut is a set of edges
whose removal ensures that the remaining graph has no path from si to sj for
each pair of distinct terminals si and sj . The directed multiway cut problem
asks for the minimum cost such set.
    Obtain an LP-relaxation for this problem similar to LP (19.2). The dual
can be interpreted as a directed multicommodity ﬂow LP. Find the optimal
fractional directed multiway cut and ﬂow in the following example:
166     19    Multiway Cut




             s1                                                         s2




   Notice that unlike LP (19.2), this relaxation does not always have an
optimal half-integral solution.

19.13 Let us deﬁne the following two problems:

Problem 19.14 (Subset feedback edge set) Given a connected, undi-
rected graph G = (V, E) with an assignment of weights to edges, w : E → R+ ,
and a set of special vertices S = {s1 , s2 , . . . , sk } ⊆ V , a subset feedback edge
set is a set of edges whose removal ensures that the remaining graph has no
cycle containing a special vertex. The subset feedback edge set problem asks
for the minimum weight such set.

Problem 19.15 (Subset feedback vertex set) Given a connected,
undirected graph G = (V, E) with an assignment of weights to vertices,
w : V → R+ , and a set of special vertices S = {s1 , s2 , . . . , sk } ⊆ V , a
subset feedback vertex set is a subset of V − S whose removal ensures that
the remaining graph has no cycle containing a special vertex. The subset
feedback vertex set problem asks for the minimum weight such set.

    These and previously introduced problems are related by approximation
factor preserving reductions given in the following ﬁgure (each arrow rep-
resents such a reduction). Give these reductions. For a deﬁnition of such
reductions, see Section A.3.1.

                     Vertex Cover                    Multiway Cut
                    ✟✟       ❍❍
                                                   ✟
                                                        ✟ ❍❍
                  ✙✟
                  ✟               ❍❍
                                   ❥            ✙✟
                                                ✟           ❍❍
                                                             ❥
 Multicut in Trees               Node Multiway Cut         Subset Feedback Edge Set

                                           ❄
                               Directed Multiway Cut

                           ❄
                  Feedback Vertex Set
                             ◗
                              ◗                   ✠
                               ◗
                               s
                             Subset Feedback Vertex Set
                                                        19.5   Notes   167

   The current best factors known for multiway cut and subset feedback
vertex set are 1.34 and 8, respectively. For the rest of the problems, the
current best factor is 2.


19.5 Notes
Algorithm 19.4 is due to Calinescu, Karloﬀ, and Rabani [36]. The current
best guarantee known for the multiway cut problem is 1.3438, due to Karger,
Klein, Stein, Thorup, and Young [157]. This is also the best upper bound
known on the integrality gap of the relaxation used. Freund and Karloﬀ
                                                                    1
[87] give a family of instances achieving a lower bound of 8/(7 + k−1  ) on
the integrality gap; Example 19.2 is from their paper. Theorem 19.12 is due
to Garg, Vazirani, and Yannakakis [96]. For currently best approximation
algorithms known for directed multiway cut, subset feedback edge set, and
subset feedback vertex set, see Naor and Zosin [210], Even, Naor, Schieber,
and Zosin [76], and Even, Naor, and Zosin [77], respectively.
