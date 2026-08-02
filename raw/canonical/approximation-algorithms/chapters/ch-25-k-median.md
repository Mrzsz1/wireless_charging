---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-25"
chapter_number: 25
chapter_title: "k-Median"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 261
source_page_end: 272
printed_page_start: 243
printed_page_end: 254
part_ids: ["approximation-algorithms-ch-25-part-026"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# k-Median

25 k-Median




The k-median problem diﬀers from the facility location problem in two re-
spects – there is no cost for opening facilities and there is an upper bound,
k, on the number of facilities that can be opened. It models the problem of
ﬁnding a minimum cost clustering, and therefore has numerous applications.
    The primal–dual schema works by making judicious local improvements
and is not suitable for ensuring a global constraint, such as the constraint in
the k-median problem that at most k facilities be opened. We will get around
this diﬃculty by borrowing the powerful technique of Lagrangian relaxation
from combinatorial optimization.
Problem 25.1 (Metric k-median) Let G be a bipartite graph with bi-
partition (F, C), where F is the set of facilities and C is the set of cities, and
let k be a positive integer specifying the number of facilities that are allowed
to be opened. Let cij be the cost of connecting city j to (opened) facility i.
The connection costs satisfy the triangle inequality. The problem is to ﬁnd
a subset I ⊆ F, |I| ≤ k, of facilities that should be opened and a function
φ : C → I assigning cities to open facilities in such a way that the total
connecting cost is minimized.


25.1 LP-relaxation and dual
The following is an integer program for the k-median problem. The indicator
variables yi and xij play the same role as in (24.1).
                      
      minimize                 cij xij                                     (25.1)
                    i∈F, j∈C
                    
      subject to          xij ≥ 1,       j∈C
                    i∈F
                    yi − xij ≥ 0,        i ∈ F, j ∈ C
                    
                        −yi ≥ −k
                    i∈F
                    xij ∈ {0, 1},        i ∈ F, j ∈ C
                    yi ∈ {0, 1},         i∈F
244      25   k-Median

      The LP-relaxation of this program is:
                           
        minimize                     cij xij                           (25.2)
                        i∈F, j∈C
                        
        subject to             xij ≥ 1,          j∈C
                        i∈F
                        yi − xij ≥ 0,            i ∈ F, j ∈ C
                        
                            −yi ≥ −k
                        i∈F
                        xij ≥ 0,                 i ∈ F, j ∈ C
                        yi ≥ 0,                  i∈F

      The dual program is:
                        
        maximize               αj − zk                                 (25.3)
                        j∈C

        subject to      αj − βij ≤ cij ,             i ∈ F, j ∈ C
                        
                            βij ≤ z,                 i∈F
                        j∈C
                        αj ≥ 0,                      j∈C
                        βij ≥ 0,                     i ∈ F, j ∈ C
                        z≥0


25.2 The high-level idea
The similarity between the two problems, facility location and k-median,
leads to a similarity in their linear programs, which will be exploited as
follows. Take an instance of the k-median problem, assign a cost of z for
opening each facility, and ﬁnd optimal solutions to LP (24.2) and LP (24.3),
say (x, y) and (α, β), respectively. By the strong duality theorem,
                                             
                   cij xij +         zyi =           αj .
        i∈F, j∈C               i∈F             j∈C


    Now, suppose that the primal    solution (x, y) happens to open exactly
k facilities (fractionally), i.e.,  i yi = k. Then, we claim that (x, y) and
(α, β, z) are optimal solutions to LP (25.2) and LP (25.3), respectively.
                                                                         Fea-
sibility is easy to check. Optimality follows by substituting     i y i = k in
the above equality and rearranging terms to show that the primal and dual
solutions achieve the same objective function value:
                                                   25.2   The high-level idea    245
                            
                 cij xij =         αj − zk.
      i∈F, j∈C               j∈C


   Let’s use this idea, together with Algorithm 24.2 and Theorem 24.7, to
obtain a “good” integral solution to LP (25.2). Suppose with a cost of z for
opening each facility, Algorithm 24.2, happens to ﬁnd solutions (x, y) and
(α, β), where the primal solution opens exactly k facilities. By Theorem 24.7,
                                     
                 cij xij + 3zk ≤ 3          αj .
      i∈F, j∈C                        j∈C


Now, observe that (x, y) and (α, β, z) are primal (integral) and dual feasible
solutions to the k-median problem satisfying
                               
                 cij xij ≤ 3(       αj − zk).
      i∈F, j∈C                j∈C


Therefore, (x, y) is a solution to the k-median problem within thrice the
optimal.
    Notice that the factor 3 proof given above would not work if less than
k facilities were opened; if more than k facilities are opened, the solution
is infeasible for the k-median problem. The remaining problem is to ﬁnd a
value of z so that exactly k facilities are opened. Several ideas are required
for this. The ﬁrst is the following principle from economics. Taxation is an
eﬀective way of controlling the amount of goods coming across a border –
raising tariﬀs will reduce inﬂow and vice versa. In a similar manner, raising
z should reduce the number of facilities opened and vice versa.
    It is natural now to seek a modiﬁcation to Algorithm 24.2 that can ﬁnd a
value of z so that exactly k facilities are opened. This would lead to a factor
3 approximation algorithm. Such a modiﬁcation is not known. Instead, we
present the following strategy which leads to a factor 6 algorithm. For the rest
of the discussion, assume that we never encountered a run of the algorithm
which resulted in exactly k facilities being opened.
    Clearly, when z = 0 the algorithm will open all facilities, and when z is
very large it will open only one facility. The latter value of z can be picked
to be ncmax , where cmax is the length of the longest edge. We will conduct
a binary search on the interval [0, ncmax ] to ﬁnd z2 and z1 for which the
algorithm opens k2 > k and k1 < k facilities, respectively, and, furthermore,
z1 − z2 ≤ (cmin /12n2c ), where cmin is the length of the shortest nonzero edge.
As before, we will adopt the following notation: nc = |C| and nf = |F |.
The total number of vertices nc + nf = n and the total number of edges
                     s    s
n
c ×nf s= m. Let (x, y ) and   (xl , y l ) be the two primal solutions found, with
                            l
   i∈F yi = k1 and     i∈F yi = k2 (the superscripts s and l denote “small” and
“large,” respectively). Further, let (αs , β s ) and (αl , β l ) be the corresponding
dual solutions found.
246     25   k-Median

    Let (x, y) = a(xs , y s ) + b(xl , y l ) be a convex combination of these two
solutions, with ak1 + bk2 = k. Under these conditions, a = (k2 − k)/(k2 − k1 )
and b = (k − k1 )/(k2 − k1 ). Since (x, y) is a feasible (fractional) solution to
the facility location problem that opens exactly k facilities, it is also a feasible
(fractional) solution to the k-median problem. In this solution each city is
connected to at most two facilities.

Lemma 25.2 The cost of (x, y) is within a factor of (3 + 1/nc ) of the cost
of an optimal fractional solution to the k-median problem.

Proof: By Theorem 24.7 we have
                                 
                 cij xsij ≤ 3(          αjs − z1 k1 ),
      i∈F, j∈C                    j∈C


and
                                 
                 cij xlij ≤ 3(          αjl − z2 k2 ).
      i∈F, j∈C                    j∈C


   Since z1 > z2 , (αl , β l ) is a feasible dual solution to the facility location
problem even if the cost of facilities is z1 . We would like to replace z2 with z1 in
the second inequality, at the expense of the increased   factor. This is achieved
using the upper bound on z1 − z2 and the fact that i∈F, j∈C cij xlij ≥ cmin .
We get
                                                                 
                                                
                                     1         
                 cij xlij ≤       3+                   αjl − z1 k2  .
                                     nc
      i∈F, j∈C                                  j∈C


   Adding this inequality multiplied by b with the ﬁrst inequality multiplied
by a gives
                                                                
                                                
                                     1         
                 cij xij ≤        3+                   αj − z1 k  ,
                                     nc
      i∈F, j∈C                                  j∈C


where α = aαs + bαl . Let β = aβ s + bβ l . Observe that (α, β, z1 ) is a feasible
solution to the dual of the k-median problem. The lemma follows.                ✷
    In Section 25.3 we give a randomized rounding procedure that obtains an
integral solution to the k-median problem from (x, y), with a small increase
in cost. In Section 25.3.1 we derandomize this procedure.
                                                       25.3      Randomized rounding   247

25.3 Randomized rounding
We give a randomized rounding procedure that produces an integral solution
to the k-median problem from (x, y). In the process, it increases the cost by
a multiplicative factor of 1 + max(a, b).
     Let A and B be the sets of facilities opened in the two solutions, |A| = k1
and |B| = k2 . For each facility in A, ﬁnd the closest facility in B – these
facilities are not required to be distinct. Let B  ⊂ B be these facilities. If
|B  | < k1 , arbitrarily include additional facilities from B − B  into B  until
|B  | = k1 .
     With probability a, open all facilities in A, and with probability b = 1−a,
open all facilities in B  . In addition, a set of cardinality k − k1 is picked
randomly from B − B  and facilities in this set are opened. Notice that each
facility in B − B  has a probability of b of being opened. Let I be the set of
facilities opened, |I| = k.
     The function φ : C → I is deﬁned as follows. Consider city j and suppose
that it is connected to i1 ∈ A and i2 ∈ B in the two solutions. If i2 ∈ B  , then
one of i1 and i2 is opened by the procedure given above, i1 with probability
a and i2 with probability b. City j is connected to the open facility.
                               A                    B
                         k1                  k2

                           i1 .................................
                               s                                si3   B
                               ❏
                                 ❏
                                    ❏
                                      ❏
                                         ❏
                                         ❏s                     si2
                                        j                             B − B



If i2 ∈ B − B  , let i3 ∈ B  be the facility in B that is closest to i1 . City j is
connected to i2 if it is open. Otherwise, it is connected to i1 if it is open. If
neither i2 or i1 is open, then j is connected to i3 .
    Denote by cost(j) the connection cost for city j in the fractional solution
(x, y); cost(j) = aci1 j + bci2 j .

Lemma 25.3 The expected connection cost for city j in the integral solution,
E[cφ(j)j ], is ≤ (1 + max(a, b))cost(j). Moreover, E[cφ(j)j ] can be eﬃciently
computed.

Proof: If i2 ∈ B  , E[cφ(j)j ] = aci1 j + bci2 j = cost(j). Consider the second
              / B  . Now, i2 is open with probability b. The probability that
case, that i2 ∈
i2 is not open and i1 is open is (1 − b)a = a2 , and the probability that both
i2 and i1 are not open is (1 − b)(1 − a) = ab. This gives
248       25   k-Median

        E[cφ(j)j ] ≤ bci2 j + a2 ci1 j + abci3 j .

   Since i3 is the facility in B that is closest to i1 , ci1 i3 ≤ ci1 i2 ≤ ci1 j + ci2 j ,
where the second inequality follows from the triangle inequality. Again, by
the triangle inequality, ci3 j ≤ ci1 j + ci1 i3 ≤ 2ci1 j + ci2 j . Therefore,

        E[cφ(j)j ] ≤ bci2 j + a2 ci1 j + ab(2ci1 j + ci2 j ).

Now, a2 ci1 j + abci1 j = aci1 j . Therefore,

        E[cφ(j)j ] ≤ (aci1 j + bci2 j ) + ab(ci1 j + ci2 j )
                   ≤ (aci1 j + bci2 j )(1 + max(a, b)).

      Clearly, in both cases, E[cφ(j)j ] is easy to compute.                           ✷
   Let (xk , y k ) denote the integral solution obtained to the k-median prob-
lem by this randomized rounding procedure. Then,
                                                                           
                                                                 
Lemma 25.4             E          cij xkij  ≤ (1 + max(a, b))      cij xij 
                               i∈F, j∈C                               i∈F, j∈C

and, moreover, the expected cost of the solution found can be computed eﬃ-
ciently.


25.3.1      Derandomization

Derandomization follows in a straightforward manner using the method of
conditional expectation. First, the algorithm opens the set A with probability
a and the set B  with probability b = 1−a. Pick A, and compute the expected
value if k − k1 facilities are randomly chosen from B − B  . Next, do the same
by picking B  instead of A. Choose to open the set that gives the smaller
expectation.
   Second, the algorithm opens a random subset of k − k1 facilities from
B − B  . For a choice D ⊂ B − B  , |D| ≤ k − k1 , denote by E[D, B − (B  ∪ D)]
the expected cost of the solution if all facilities in D and additionally k −
k1 − |D| facilities are randomly opened from B − (B  ∪ D). Since each facility
of B − (B  ∪ D) is equally likely to be opened, we get

        E[D, B − (B  ∪ D)] =
              1            
                
                                          E[D ∪ {i}, B − (B  ∪ D ∪ {i})].
        |B − (B ∪ D)|         
                            i∈B−(B ∪D)


This implies that there is an i such that
                                                                       25.3        Randomized rounding        249

      E[D ∪ {i}, B − (B  ∪ D ∪ {i})] ≤ E[B  , B − (B  ∪ D)].

Choose such an i and replace D with D ∪ {i}. Notice that the computation
of E[D ∪ {i}, B − (B  ∪ D ∪ {i})] can be done as in Lemma 25.4.

25.3.2   Running time

It is easy to see that a ≤ 1 − 1/nc (this happens for k1 = k − 1 and k2 = nc )
and b ≤ 1 − 1/k (this happens for k1 = 1 and k2 = k + 1). Therefore,
1 + max(a, b) ≤ 2 − 1/nc . Altogether, the approximation guarantee is (2 −
1/nc )(3 + 1/nc ) < 6. This procedure can be derandomized using the method
of conditional probabilities, as in Section 25.3.1. The binary search will make
O(log2 (n3 cmax /cmin )) = O(L+log n) probes. The running time for each probe
is dominated by the time taken to run Algorithm 24.2; randomized rounding
takes O(n) time and derandomization takes O(m) time. Hence we get

Theorem 25.5 The algorithm given above achieves an approximation factor
of 6 for the k-median problem, and has a running time of O(m log m(L +
log(n))).

25.3.3   Tight example

A tight example for the factor 6 k-median algorithm is not known. However,
below we give an inﬁnite family of instances which show that the analysis of
the randomized rounding procedure cannot be improved.
    The two solutions (xs , y s ) and (xl , y l ) open one facility, f0 , and k + 1
facilities, f1 , . . . , fk+1 , respectively. The distance between f0 and any other
fi is 1, and that between two facilities in the second set is 2. All n cities
are at a distance of 1 from f0 , and at a distance of ε from fk+1 . The rest of
the distances are given by the triangle inequality. The convex combination is
constructed with a = 1/k and b = 1 − 1/k.
                                                                                                       c1
                                                                                                     ✏s
                                                                                             ✏  ✏✏ ✁
                                                                               f0         1✏
                                                                                  s✏ ✏✏   1
                                                                                                     ✁ c2
                                                                                                          s
                                                                                   P               ✁
                                                                              ✚ ✡❇❙ PP
                                                                                ✚         1           ✡
                                                                         ✚ ✡ ❇ ❙ 1 PPP✁ ✡ c3
                                                                      1✚ ✡                      ✁ PPs
                                                                    ✚ ✡  1 1        ❇1 ❙      ε✁ε ✡
                                                                  ✚                  ❇ ❙ ✁ ✡ε
                                                       ✚               ✡              ❇ ❙✁ ✡
                                               ✚                     ✡                                  ..
                                        ✚                                              ❇ ✁✡  ❙           .
                                 ✚                                  ✡                   ❇ ✁✡ ❙
                    ✚
                    s                       s
                f1 ...... f2 ........... f3 ......                ✡
                                                                  s       . . .    fk+1 ❍❇ ✡
                                                                                           ✁
                                                                                           s    ❙
                        ......... .......... ......... ..........
                                .                    .                                       ❍❍ ❙
                                2                     2                                       ε ❍ cn
                                                                                                    ❍❙
                                                                                                     ❍❙s
250      25   k-Median

    Now, the cost of the convex combination is an + bεn. Suppose the algo-
rithm picks f1 as the closest neighbor of f0 . The expected cost of the solutions
produced by the randomized rounding procedure is then n(bε+a2 +ab(2+ε)).
Letting ε tend to 0, the cost of the convex combination is essentially na and
that of the rounded solution is na(1 + b).

25.3.4    Integrality gap

The algorithm given above places an upper bound of 6 on the integrality gap
of relaxation (25.2). The following example places a lower bound of essentially
2. The graph is a star with n + 1 vertices and unit cost edges. F consists of
all n + 1 vertices, C consists of all but the center vertex and k = n − 2. An
optimal integral solution is to open facilities at n − 2 vertices of C and has
a cost of 2. Consider the following fractional solution. Open a facility to the
extent of 1/(n − 1) on the center vertex and (n − 2)/(n − 1) on each vertex
of C. This has a cost of n/(n − 1), giving a ratio of 2(n − 1)/n.



                                     t           ...            t
                                     ❅
                                       ❅
                                         ❅
                         n−2
                                             ❅
                         n−1   t              ❅t                    t
                                                  1
                                                 n−1




25.4 A Lagrangian relaxation technique
for approximation algorithms
In this section we will abstract away the ideas developed above so they may
be more widely applicable. First, let us recall the fundamental technique
of Lagrangian relaxation from combinatorial optimization. This technique
consists of relaxing a constraint by moving it into the objective function,
together with an associated Lagrange multiplier.
    Let us apply this relaxation to the constraint, in the k-median IP (25.1),
that at most k facilities be opened. Let λ be the Lagrangian multiplier.
                                                           
                                             
      minimize                 cij xij + λ         yi − k                 (25.4)
                    i∈F, j∈C                 i∈F
                    
      subject to          xij ≥ 1,                              j∈C
                    i∈F
                                                           25.5   Exercises     251

                    yi − xij ≥ 0,                          i ∈ F, j ∈ C
                    xij ∈ {0, 1},                          i ∈ F, j ∈ C
                    yi ∈ {0, 1},                           i∈F

    This is precisely the facility location IP, with the restriction that the cost
of each facility is the same, i.e., λ. It contains an additional constant term
of −λk in the objective function. We may assume w.l.o.g. that an optimal
solution, (x, y), to IP (25.1) opens exactly k facilities. Now, (x, y) is a feasible
solution to IP (25.4) as well, with the same objective function value. Hence,
for each value of λ, IP (25.4) is a lower bound on IP (25.1).
    We have shown that a Lagrangian relaxation of the k-median problem is
the facility location problem. In doing so, the global constraint that at most
k facilities be opened has been replaced with a penalty for opening facilities,
the penalty being the Lagrangian multiplier. (See Exercise 25.4 for another
application of this idea.)
    The next important observation was to notice that in the facility location
approximation algorithm, Theorem 24.7, the duals pay one-for-one for the
cost of opening facilities, i.e., with approximation factor 1. (See Exercise 22.9
for another such algorithm.)
    The remaining diﬃculty was ﬁnding a value of λ so that the facility loca-
tion algorithm opened exactly k facilities. The fact that the facility location
algorithm works with the linear relaxation of the problem helped. The con-
vex combination of two (integer) solutions was a feasible (fractional) solution.
The last step was rounding this (special) fractional solution into an integral
one. For the k-median problem we used randomized rounding (see Exercise
25.4 for a diﬀerent rounding procedure).


25.5 Exercises

25.1 (Lin and Vitter [188]) Consider the general k-median problem in which
the connection costs are not required to satisfy the triangle inequality. Give
a reduction from the set cover problem to show that approximating this
problem is as hard as approximating set cover, and therefore cannot be done
with a factor better than O(log n) unless NP ⊆ P̃.

25.2 Obtain the dual of LP-relaxation to (25.4). (The constant term in the
objective function will simply carry over.) How does it relate with the dual
of the k-median LP?

25.3 Use the Lagrangian relaxation technique to give a constant factor ap-
proximation algorithm for the following common generalization of the facility
location and k-median problems. Consider the uncapacitated facility location
252    25   k-Median

problem with the additional constraint that at most k facilities can be opened.
This is a common generalization of the two problems solved in this paper:
if k is made nf , we get the ﬁrst problem, and if the facility costs are set to
zero, we get the second problem.

25.4 (Garg [94] and Chudak, Roughgarden, and Williamson [47]) Consider
the following variant of the metric Steiner tree problem.
Problem 25.6 (Metric k-MST) We are given a complete undirected
graph G = (V, E), a special vertex r ∈ V , a positive integer k, and a function
cost : E → Q+ satisfying the triangle inequality. The problem is to ﬁnd a
minimum cost tree containing exactly k vertices, including r.
   We will develop a factor 5 algorithm for this problem.

 1. Observe that a Lagrangian relaxation of this problem is the prize-
    collecting Steiner tree problem, Problem 22.12, stated in Exercise 22.9.
 2. Observe that the approximation algorithm for the latter problem, given
    in Exercise 22.9, pays for the penalties one-for-one with the dual, i.e.,
    with an approximation factor of 1.
 3. Use the prize-collecting algorithm as a subroutine to obtain two trees, T1
    and T2 , for very close values of the penalty, containing k1 and k2 vertices,
    with k1 < k < k2 . Obtain a convex combination of these solutions, with
    multipliers α1 and α2 .
 4. We may assume that every vertex in G is at a distance of ≤ OPT from
    r. (Use the idea behind parametric pruning, introduced in Chapter 5.
    The parameter t is the length of the longest edge used by the optimal
    solution, which is clearly a lower bound on OPT. For each value of t,
    instance G(t) is obtained by restricting G to vertices that are within a
    distance of t of r. The algorithm is run on each graph of this family, and
    the best tree is output.) Consider the following procedure for rounding
    the convex combination. If α2 ≥ 1/2, then cost(T2 ) ≤ 4 · OPT; remove
    k2 − k vertices from T2 . Otherwise, double every edge of T2 , ﬁnd an Euler
    tour, and shortcut the tour to a cycle containing only those vertices that
    are in T2 and not in T1 (i.e., at most k2 − k1 vertices). Pick the cheapest
    path of length k − k1 − 1 from this cycle, and connect it by means of an
    edge to vertex r in T1 . The resulting tree has exactly k vertices. Show
    that the cost of this tree is ≤ 5 · OPT.
    Hint: Use the fact that α2 = (k − k1 )/(k2 − k1 ).


25.5 Let us apply the Lagrangian relaxation technique to the following linear
program.

      minimize     cT x                                                   (25.5)
      subject to   Ax = b
                                                             25.5   Exercises     253

Then the lower bound is given by
                                                            
      max min cT x − y T (Ax − b) = max min (cT − y T A)x + y T b
       y x                           y    x

If y does not satisfy AT y = c, then by a suitable choice of x, the lower
bound given by this expression can be made as small as desired and therefore
meaningless. Meaningful lower bounds arise only if we insist that AT y = c.
But then we get the following LP:

      maximize       yT b                                                       (25.6)

      subject to     AT y = c

Notice that this is the dual of LP (25.5)! Hence, the Lagrangian relaxation
of a linear program is simply its dual and is therefore tight.
    Obtain the Lagrangian relaxation of the following LP:

      minimize       cT x                                                       (25.7)

      subject to     Ax ≥ b
                     x≥0


25.6 (Jain and Vazirani [141]) Consider the l22 clustering problem. Given a
set of n points S = {v1 , . . . , vn } in Rd and a positive integer k, the problem
is to ﬁnd a minimum cost k-clustering, i.e., to ﬁnd k points, called centers,
f1 , . . . , fk ∈ Rd , so as to minimize the sum of squares of distances from each
point vi to its closest center. This naturally deﬁnes a partitioning of the n
points into k clusters. Give a constant factor approximation algorithm for
this problem.
Hint: First show that restricting the centers to be a subset S increases the
cost of the optimal solution by a factor of at most 2. Apply the solution of
Exercise 24.6 to this modiﬁed problem.

25.7 (Korupolu, Plaxton, and Rajaraman [176] and Arya et al. [15]) For a
set S of k facilities, deﬁne cost(S) to be the total cost of connecting each city
to its closest facility in S. Deﬁne a swap to be the process of replacing one
facility in S by a facility from S. A natural algorithm for metric k-median,
based on local search, is: Start with an arbitrary set S of k facilities. In
each iteration, check if there is a swap that leads to a lower cost solution.
If so, execute any such swap and go to the next iteration. If not, halt. The
terminating solution is said to be locally optimal.
     Let G = {o1 , . . . , ok } be an optimal solution and L = {s1 , . . . , sk } be a
locally optimal solution. This exercise develops a proof showing cost(L) ≤
5 · cost(G), as well as a constant factor approximation algorithm.
254    25   k-Median

 1. For o ∈ G, let NG (o) denote the set of cities connected to facility o in the
    optimal solution. Similarly, for s ∈ L, let NL (s) denote the set of cities
    connected to facility s in the locally optimal solution. Say that s ∈ L
    captures o ∈ G if |NG (o) ∩ NL (s)| > |NG (o)|/2. Clearly, each o ∈ G
    is captured by at most one facility in L. In this part let us make the
    simplifying assumption that each facility s ∈ L captures a unique facility
    in G. Assume that the facilities are numbered so that si captures oi , for
    1 ≤ i ≤ k. Use the fact that for 1 ≤ i ≤ k, cost(L + oi − si ) ≥ cost(L) to
    show that cost(L) ≤ 3 · cost(G).
    Hint: cost(L + oi − si ) is bounded by the cost of the following solution:
    The cities in NL (si ) ∪ NG (oi ) are connected as in the locally optimal
    solution. Those in NG (oi ) are connected to facility oi . Cities in NL (si ) −
    NG (oi ) are connected to facilities in L − si using “3 hops” in such a way
    that each connecting edge of G and each connecting edge of L is used at
    most once in the union of all these hops.
 2. Show that without the simplifying assumption of the previous part,
    cost(L) ≤ 5 · cost(G).
    Hint: Consider k appropriately chosen swaps so that each facility o ∈ G
    is swapped in exactly once and each facility s ∈ L is swapped out at most
    twice.
 3. Strengthen the condition for swapping so as to obtain, for any ε > 0 a
    factor 5 + ε algorithm running in time polynomial in 1/ε and the size of
    the instance.


25.6 Notes
The ﬁrst approximation algorithm, achieving a factor of O(log n log log n),
was given by Bartal [21]. The ﬁrst constant factor approximation algorithm
for the k-median problem, achieving a guarantee of 6 23 , was given by Charikar,
Guha, Tardos, and Shmoys [39], using ideas from Lin and Vitter [189]. This
algorithm used LP-rounding. The results of this chapter are due to Jain and
Vazirani [141]. The current best factor is 3 + 2/p, with a running time of
O(np ), due to Arya et al. [15]. This is a local search algorithm that swaps p
facilities at a time (see Exercise 25.7 for the algorithm for p = 1).
     The example of Section 25.3.4 is due to Jain, Mahdian, and Saberi [138].
The best upper bound on the integrality gap of relaxation (25.2) is 4, due
to Charikar and Guha [38]. For a factor 2 approximation algorithm for the
l22 clustering problem (Exercise 25.6), see Drineas, Kannan, Frieze, Vempala,
and Vinay [62].
