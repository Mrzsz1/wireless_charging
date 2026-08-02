---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-24"
chapter_number: 24
chapter_title: "Facility Location"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 250
source_page_end: 260
printed_page_start: 232
printed_page_end: 242
part_ids: ["approximation-algorithms-ch-24-part-025"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Facility Location

24 Facility Location




The facility location problem has occupied a central place in operations re-
search since the early 1960’s. It models design situations such as deciding
placements of factories, warehouses, schools, and hospitals. Modern day ap-
plications include placement of proxy servers on the web.
    In this chapter, we will present a primal–dual schema based factor 3
approximation algorithm for the special case when connection costs satisfy
the triangle inequality. The algorithm diﬀers in two respects from previous
primal–dual algorithms. First, the primal and dual pair of LPs have negative
coeﬃcients and do not form a covering-packing pair. Second, we will relax
primal complementary slackness conditions rather than the dual ones. Also,
the idea of synchronization, introduced in the primal–dual schema in Chapter
22, is developed further, with an explicit timing of events playing a role.
Problem 24.1 (Metric uncapacitated facility location) Let G be a
bipartite graph with bipartition (F, C), where F is the set of facilities and C
is the set of cities. Let fi be the cost of opening facility i, and cij be the cost
of connecting city j to (opened) facility i. The connection costs satisfy the
triangle inequality. The problem is to ﬁnd a subset I ⊆ F of facilities that
should be opened, and a function φ : C → I assigning cities to open facilities
in such a way that the total cost of opening facilities and connecting cities
to open facilities is minimized.
    Consider the following integer program for this problem. In this program,
yi is an indicator variable denoting whether facility i is open, and xij is an
indicator variable denoting whether city j is connected to the facility i. The
ﬁrst set of constraints ensures that each city is connected to at least one
facility, and the second ensures that this facility must be open.
                                          
      minimize                 cij xij +         fi yi                      (24.1)
                    i∈F, j∈C               i∈F
                    
      subject to          xij ≥ 1,                       j∈C
                    i∈F
                    yi − xij ≥ 0,                        i ∈ F, j ∈ C
                    xij ∈ {0, 1},                        i ∈ F, j ∈ C
                    yi ∈ {0, 1},                         i∈F
                                  24.1    An intuitive understanding of the dual     233

        The LP-relaxation of this program is:
                                             
          minimize                cij xij +         fi yi                          (24.2)
                       i∈F, j∈C               i∈F
                       
          subject to         xij ≥ 1,                       j∈C
                       i∈F
                       yi − xij ≥ 0,                        i ∈ F, j ∈ C
                       xij ≥ 0,                             i ∈ F, j ∈ C
                       yi ≥ 0,                              i∈F

        The dual program is:
                       
          maximize           αj                                                    (24.3)
                       j∈C

          subject to   αj − βij ≤ cij ,        i ∈ F, j ∈ C
                       
                           βij ≤ fi ,          i∈F
                       j∈C
                       αj ≥ 0,                 j∈C
                       βij ≥ 0,                i ∈ F, j ∈ C


 24.1 An intuitive understanding of the dual
 Let us ﬁrst give the reader some feel for how the dual variables “pay” for
 a primal solution by considering the following simple setting. Suppose LP
 (24.2) has an optimal solution that is integral, say I ⊆ F and φ : C → I.
 Thus, under this solution, yi = 1 iﬀ i ∈ I, and xij = 1 iﬀ i = φ(j). Let (α, β)
 denote an optimal dual solution.
    The primal and dual complementary slackness conditions are:

 (i)          ∀i ∈ F, j ∈ C : xij 
                                  > 0 ⇒ αj − βij = cij
(ii)          ∀i ∈ F : yi > 0 ⇒      βij = fi
                                   j∈C
                                    
(iii)         ∀j ∈ C : αj > 0 ⇒           xij = 1
                                    i∈F
(iv)          ∀i ∈ F, j ∈ C : βij > 0 ⇒ yi = xij
    By condition (ii), each open facility must be fully paid for, i.e., if i ∈ I,
 then
        
              βij = fi .
          j: φ(j)=i
234     24   Facility Location

     Consider condition (iv). Now, if facility i is open, but φ(j) = i, then
yi = xij , and so βij = 0, i.e., city j does not contribute to opening any
facility besides the one it is connected to.
     By condition (i), if φ(j) = i, then αj − βij = cij . Thus, we can think of
αj as the total price paid by city j; of this, cij goes towards the use of edge
(i, j), and βij is the contribution of j towards opening facility i.


24.2 Relaxing primal complementary slackness
conditions
Suppose the primal complementary slackness conditions were relaxed as fol-
lows, while maintaining the dual conditions:

      ∀j ∈ C : (1/3)cφ(j)j ≤ αj − βφ(j)j ≤ cφ(j)j ,

and
                                 
      ∀i ∈ I : (1/3)fi ≤                   βij ≤ fi .
                               j: φ(j)=i


    Then, the cost of the (integral) solution found would be within thrice the
dual found, thus leading to a factor 3 approximation algorithm. However, we
would like to obtain the stronger inequality stated in Theorem 24.7. Now, the
dual pays at least one-third the connection cost, but must pay completely for
opening facilities. This stronger inequality will be needed in order to use this
algorithm to solve the k-median problem in Chapter 25.
    For this reason, we will relax the primal conditions as follows. The cities
are partitioned into two sets, directly connected and indirectly connected. Only
directly connected cities will pay for opening facilities, i.e., βij can be nonzero
only if j is a directly connected city and i = φ(j). For an indirectly connected
city j, the primal condition is relaxed as follows:

      (1/3)cφ(j)j ≤ αj ≤ cφ(j)j .

All other primal conditions are maintained, i.e., for a directly connected city
j,

      αj − βφ(j)j = cφ(j)j ,

and for each open facility i,
        
                  βij = fi .
      j: φ(j)=i
                              24.3   Primal–dual schema based algorithm       235

24.3 Primal–dual schema based algorithm
The algorithm consists of two phases. In Phase 1, the algorithm operates in
a primal–dual fashion. It ﬁnds a dual feasible solution and also determines
a set of tight edges and temporarily open facilities, Ft . Phase 2 consists of
choosing a subset I of Ft to open, and ﬁnding a mapping, φ, from cities to I.
Algorithm 24.2
Phase 1
We would like to ﬁnd as large a dual solution as possible. This motivates
the following underlying process for dealing with the non-covering-packing
pair of LPs. Each city j raises its dual variable, αj , until it gets connected to
an open facility. All other primal and dual variables simply respond to this
change, trying to maintain feasibility or satisfying complementary slackness
conditions.
     A notion of time is deﬁned in this phase, so that each event can be associ-
ated with the time at which it happened; the phase starts at time 0. Initially,
each city is deﬁned to be unconnected. Throughout this phase, the algorithm
raises the dual variable αj for each unconnected city j uniformly at unit rate,
i.e., αj will grow by 1 in unit time. When αj = cij for some edge (i, j), the
algorithm will declare this edge to be tight. Henceforth, dual variable βij will
be raised uniformly, thus ensuring that the ﬁrst constraint in LP (24.3) is
not violated. βij goes towards paying for facility i. Each edge (i, j) such that
βij > 0 is declared special.              
     Facility i is said to be paid for if j βij = fi . If so, the algorithm de-
clares this facility temporarily open. Furthermore, all unconnected cities hav-
ing tight edges to this facility are declared connected and facility i is declared
the connecting witness for each of these cities. (Notice that the dual vari-
ables αj of these cities are not raised anymore.) In the future, as soon as an
unconnected city j gets a tight edge to i, j will also be declared connected
and i will be declared the connecting witness for j (notice that βij = 0 and
thus edge (i, j) is not special). When all cities are connected, the ﬁrst phase
terminates. If several events happen simultaneously, the algorithm executes
them in arbitrary order.
Remark 24.3 At the end of Phase 1, a city may have paid towards tem-
porarily opening several facilities. However, we want to ensure that a city
pays only for the facility that it is eventually connected to. This is ensured
in Phase 2, which chooses a subset of temporarily open facilities for opening
permanently.
Phase 2
Let Ft denote the set of temporarily open facilities and T denote the subgraph
of G consisting of all special edges. Let T 2 denote the graph that has edge
(u, v) iﬀ there is a path of length at most 2 between u and v in T , and let H
236     24   Facility Location

be the subgraph of T 2 induced on Ft . Find any maximal independent set in
H, say I. All facilities in the set I are declared open.
    For city j, deﬁne F j = {i ∈ Ft | (i, j) is special}. Since I is an independent
set, at most one of the facilities in F j is opened. If there is a facility i ∈
F j that is opened, then set φ(j) = i and declare city j directly connected.
Otherwise, consider tight edge (i , j) such that i was the connecting witness
for j. If i ∈ I, again set φ(j) = i and declare city j directly connected (notice
that in this case βi j = 0). In the remaining case that i ∈     / I, let i be any
neighbor of i in graph H such that i ∈ I. Set φ(j) = i and declare city j
indirectly connected.
    I and φ deﬁne a primal integral solution: xij = 1 iﬀ φ(j) = i and yi = 1
iﬀ i ∈ I. The values of αj and βij obtained at the end of Phase 1 form a dual
feasible solution.


24.4 Analysis
We will show how the dual variables αj ’s pay for the primal costs of opening
facilities and connecting cities to facilities. Denote by αjf and αje the contribu-
tions of city j to these two costs respectively; αj = αjf + αje . If j is indirectly
connected, then αjf = 0 and αje = αj . If j is directly connected, then the
following must hold:

      αj = cij + βij ,

where i = φ(j). Now, let αjf = βij and αje = cij .
Lemma 24.4 Let i ∈ I. Then,
        
                  αjf = fi .
      j: φ(j)=i



Proof: Since i is temporarily open at the end of Phase 1, it is completely
paid for, i.e.,
             
                            βij = fi .
      j: (i,j) is special


The critical observation is that each city j that has contributed to fi must
be directly connected to i. For each such city, αjf = βij . Any other city j 
that is connected to facility i must satisfy αjf = 0. The lemma follows.  ✷

                                            f
Corollary 24.5           i∈I fi =        j∈C αj .
                                                                  24.4   Analysis       237

   Recall that αjf was deﬁned to be 0 for indirectly connected cities. Thus,
only the directly connected cities pay for the cost of opening facilities.

Lemma 24.6 For an indirectly connected city j, cij ≤ 3αje , where i = φ(j).

Proof: Let i be the connecting witness for city j. Since j is indirectly
connected to i, (i, i ) must be an edge in H. In turn, there must be a city,
say j  , such that (i, j  ) and (i , j  ) are both special edges. Let t1 and t2 be
the times at which i and i were declared temporarily open during Phase 1.
                                i                   i
                                s                    s
                                ❏                  ✡❏
                                                   ✡
                                  ❏              ✡ ❏
                                   ❏            ✡      ❏
                                      ❏ ✡                ❏
                                        ❏❏✡  s            ❏❏s
                                           j               j

    Since edge (i , j) is tight, αj ≥ ci j . We will show that αj ≥ cij  and
αj ≥ ci j  . Then, the lemma will follow by using the triangle inequality.
    Since edges (i , j  ) and (i, j  ) are tight, αj  ≥ cij  and αj  ≥ ci j  . Since
both these edges are special, they must both have gone tight before either i
or i is declared temporarily open. Consider the time min(t1 , t2 ). Clearly, αj 
cannot be growing beyond this time. Therefore, αj  ≤ min(t1 , t2 ). Finally,
since i is the connecting witness for j, αj ≥ t2 . Therefore, αj ≥ αj  , and the
required inequalities follow.                                                             ✷

Theorem 24.7 The primal and dual solutions constructed by the algorithm
satisfy:
                                               
                 cij xij + 3         fi yi ≤ 3         αj .
      i∈F, j∈C                 i∈F               j∈C



Proof: For a directly connected city j, cij = αje ≤ 3αje , where φ(j) = i.
Combining with Lemma 24.6 we get
                              
                cij xij ≤ 3          αje .
      i∈F,j∈C                  j∈C


Adding to this the equality stated in Corollary 24.5 multiplied by 3 gives the
theorem.                                                                    ✷
238      24   Facility Location

24.4.1    Running time

A special feature of the primal–dual schema is that it yields algorithms with
good running times. Since this is especially so for the current algorithm,
we will provide some implementation details. We will adopt the following
notation: nc = |C| and nf = |F |. The total number of vertices nc + nf = n,
and the total number of edges nc × nf = m.
    Sort all the edges by increasing cost – this gives the order and the times
at which edges go tight. For each facility, i, we maintain the number of cities
that are currently contributing towards it, and the anticipated time, ti , at
which it would be completely paid for if no other event happens on the way.
Initially all ti ’s are inﬁnite, and each facility has 0 cities contributing to it.
The ti ’s are maintained in a binary heap so we can update each one and ﬁnd
the current minimum in O(log nf ) time. Two types of events happen, and
they lead to the following updates.

• An edge (i, j) goes tight.
  – If facility i is not temporarily open, then it gets one more city contribut-
     ing towards its cost. The amount contributed towards its cost at the
     current time can be easily computed. Therefore, the anticipated time for
     facility i to be paid for can be recomputed in constant time. The heap
     can be updated in O(log nf ) time.
  – If facility i is already temporarily open, city j is declared connected, and
     αj is not raised anymore. For each facility i that was counting j as a
     contributor, we need to decrease the number of contributors by 1 and
     recompute the anticipated time at which it gets paid for.
• Facility i is completely paid for. In this event, i will be declared temporarily
  open, and all cities contributing to i will be declared connected. For each
  of these cities, we will execute the second case of the previous event, i.e.,
  update facilities that they were contributing towards.
   The next theorem follows by observing that each edge (i, j) will be consid-
ered at most twice. First, when it goes tight. Second, when city j is declared
connected. For each consideration of this edge, we will do O(log nf ) work.

Theorem 24.8 Algorithm 24.2 achieves an approximation factor of 3 for
the facility location problem and has a running time of O(m log m).

24.4.2    Tight example

The following inﬁnite family of examples shows that the analysis of our algo-
rithm is tight: The graph has n cities, c1 , c2 , . . . , cn and two facilities f1 and
f2 . Each city is at a distance of 1 from f2 . City c1 is at a distance of 1 from
f1 , and c2 , . . . , cn are at a distance of 3 from f1 . The opening cost of f1 and
f2 are ε and (n + 1)ε, respectively, for a small number ε.
                                                          24.5   Exercises    239
                                    c1
                                      s
                                 ✑ ✑◗
                              1 ✑ c2 ◗◗ 1
                              ✑       s
                            ✑3 ✏✏✏PPP◗  1◗
                           ✑✏✏          P P◗
                         ✑
                         ✏ ✏ 3      c3  1 P◗
                 f1   s✑
                       ✏              s      P◗
                                              Ps f2
                       ❅
                         ❅
                           ❅         ..
                             ❅3       . 1
                              ❅
                                ❅
                                  ❅ cn
                                   ❅s

    The optimal solution is to open f2 and connect all cities to it, at a total
cost of (n + 1)ε + n. Algorithm 24.2 will however open facility f1 and connect
all cities to it, at a total cost of ε + 1 + 3(n − 1).


24.5 Exercises

24.1 Consider the general uncapacitated facility location problem in which
the connection costs are not required to satisfy the triangle inequality. Give
a reduction from the set cover problem to show that approximating this
problem is as hard as approximating set cover and therefore cannot be done
better than O(log n) factor unless NP ⊆ P̃. Also, give an O(log n) factor
algorithm for this problem.

24.2 In Phase 2, instead of picking all special edges in T , pick all tight edges.
Show that now Lemma 24.6 does not hold. Give a suitable modiﬁcation to
the algorithm that restores Lemma 24.6.
Hint: Order facilities in H in the order in which they were temporarily
opened, and pick I to be the lexicographically ﬁrst maximal independent set.


24.3 Give a factor 3 tight example for Algorithm 24.2 in which the set of
cities and facilities is the same, i.e., C = F .

24.4 Consider the proof of Lemma 24.6. Give an example in which αj > t2 .

24.5 The vector α found by Algorithm 24.2 is maximal in the sense that if
we increase any αj in this vector, then there is no way of setting the βij ’s to
get a feasible dual solution. Is every maximal solution α within 3 times the
optimal solution to dual program for facility location?
240    24   Facility Location

Hint: It is easy to construct a maximal solution that is 2/n times the
optimal. Consider n facilities with an opening cost of 1 each and n cities
connected to distinct facilities by edges of cost ε each. In addition, there is
another city that is connected to each facility with an edge of cost 1.

24.6 Consider the following modiﬁcation to the metric uncapacitated facility
location problem. Deﬁne the cost of connecting city j to facility i to be c2ij .
The cij ’s still satisfy the triangle inequality (but the new connection costs, of
c2ij , do not). Show that Algorithm 24.2 achieves an approximation guarantee
of factor 9 for this case.

24.7 Consider the following generalization to arbitrary demands. For each
city j, a nonnegative demand dj is speciﬁed, and any open facility can serve
this demand. The cost of serving this demand via facility i is cij dj . Give an
IP and LP-relaxation for this problem, and extend Algorithm 24.2 to get a
factor 3 algorithm.
Hint: Raise αj at rate dj .

24.8 In the capacitated facility location problem, we are given a number ui
for each facility i, and facility i can serve at most ui cities. Show that the
modiﬁcation of LP (24.2) to this problem has an unbounded integrality gap.

24.9 Consider the variant of the capacitated metric facility location problem
in which each facility can be opened an unbounded number of times. If facility
i is opened yi times, it can serve at most ui yi cities. Give an IP and LP-
relaxation for this problem, and extend Algorithm 24.2 to obtain a constant
factor algorithm.

24.10 (Charikar, Khuller, Mount, and Narshimhan [40]) Consider the prize-
collecting variant of the facility location problem, in which there is a speciﬁed
penalty for not connecting a city to an open facility. The objective is to min-
imize the sum of the connection costs, facility opening costs, and penalties.
Give a factor 3 approximation algorithm for this problem.

24.11 (Jain and Vazirani [140]) Consider the fault tolerant variant of the
facility location problem, in which the additional input is a connection re-
quirement rj for each city j. In the solution, city j needs to be connected to
rj distinct open facilities. The objective, as before, is to minimize the sum of
the connection costs and the facility opening costs.
    Decompose the problem into k phases, numbered k down to 1, as in
Exercise 23.7. In phase p, all cities having a residual requirement of p are
provided one more connection to an open facility. In phase p, the facility
location algorithm of this chapter is run on the following modiﬁed graph, Gp .
The cost of each facility that is opened in an earlier phase is set to 0. If city
j is connected to facility i in an earlier phase, then cij is set to ∞.
                                                           24.5   Exercises    241

 1. Show that even though Gp violates the triangle inequality at some places,
    the algorithm gives a solution within factor 3 of the optimal solution for
    this graph.
    Hint: Every time short-cutting is needed; the triangle inequality holds.
 2. Show that the solution found in phase p is of cost at most 3 · OPT/p,
    where OPT is the cost of the solution to the entire problem.
    Hint: Remove ∞ cost edges of Gp from the optimal solution and divide
    the rest by p. Show that this is a feasible fractional solution for phase p.
 3. Show that this algorithm achieves an approximation factor of 3 · Hk for
    the fault tolerant facility location problem.


24.12 (Mahdian, Markakis, Saberi, and Vazirani [201]) This exercise devel-
ops a factor 3 greedy algorithm for the metric uncapacitated facility location
problem, together with an analysis using the method of dual ﬁtting.
    Consider the following modiﬁcation to Algorithm 24.2. As before, dual
variables, αj , of all unconnected cities, j, are raised uniformly. If edge (i, j)
is tight, βij is raised. As soon as a facility, say i, is paid for, it is declared
open. Let S be the set of unconnected cities having tight edges to i. Each city
j ∈ S is declared connected and stops raising its αj . So far, the new algorithm
is the same as Algorithm 24.2. The main diﬀerence appears at this stage:
Each city j ∈ S withdraws its contribution from other facilities, i.e., for each
facility i = i, set βi j = 0. When all cities have been declared connected, the
algorithm terminates. Observe that each city contributes towards the opening
cost of at most one facility – the facility it gets connected to.

 1. This algorithm actually has a simpler description as a greedy algorithm.
    Provide this description.
    Hint: Use the notion of cost–eﬀectiveness deﬁned for the greedy set
    cover algorithm.
 2. The next 3 parts use the method of dual ﬁtting to analyze this algorithm.
    First observe that the primal solution found is fully paid for by the dual
    computed.
 3. Let i be an open facility and let {1, . . . , k} be the set of cities that con-
    tributed to opening i at some point in the algorithm. Assume w.l.o.g.
    that α1 ≤ αj for j ≤ k. Show that for j ≤ k, αj − cij ≤ 2α1 . Also, show
    that
          k
                        k
                         
                αj ≤ 3         cij + fi .
          j=1            j=1


    Hint: Use the triangle inequality and the following inequality which is a
    consequence of the fact that at any point, the total amount contributed
    for opening facility i is at most fi :
242    24     Facility Location
              
                         α1 − cij ≤ fi .
            j: cij ≤α1


 4. Hence show that α/3 is a dual feasible solution.
 5. How can the analysis be improved – a factor 1.86 analysis is known for
    this algorithm.
 6. Give a time eﬃcient implementation of this algorithm, matching the run-
    ning time of Algorithm 24.2
 7. Do you see room for improving the algorithm?
    Hint: Suppose city j is connected to open facility i at some point in the
    algorithm. Later, facility i is opened, and suppose that cij > ci j . Then,
    connecting j to i will reduce the cost of the solution.


24.13 (Mahdian, Markakis, Saberi, and Vazirani [201]) Consider the follow-
ing variant of the metric uncapacitated facility location problem. Instead of
fi , the opening cost for each facility i ∈ F , we are provided a startup cost
si and an incremental cost ti . Deﬁne the new opening cost for connecting
k > 0 cities to facility i to be si + kti . Connection costs are speciﬁed by a
metric, as before. The object again is to connect each city to an open facility
so as to minimize the sum of connection costs and opening costs. Give an
approximation factor preserving reduction from this problem to the metric
uncapacitated facility location problem.
Hint: Modify the metric appropriately.


24.6 Notes
The ﬁrst approximation algorithm for the metric uncapacitated facility loca-
tion problem, due to Hochbaum [124], achieved an approximation guarantee
of O(log n). The ﬁrst constant factor approximation algorithm, achieving a
guarantee of 3.16, was due to Shmoys, Tardos, and Aardal [239]. It was based
on LP-rounding. The current best algorithm, achieving an approximation
guarantee of 1.61, is due to Jain, Mahdian, and Saberi [138]. This algorithm,
a small modiﬁcation of the greedy algorithm presented in Exercise 24.12, is
analyzed using the method of dual ﬁtting. The primal–dual schema based
Algorithm 24.2 is due to Jain and Vazirani [141].
