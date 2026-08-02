---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-15"
chapter_number: 15
chapter_title: "Set Cover via the Primal–Dual Schema"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 143
source_page_end: 148
printed_page_start: 125
printed_page_end: 130
part_ids: ["approximation-algorithms-ch-15-part-016"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Set Cover via the Primal–Dual Schema

15 Set Cover via the Primal–Dual Schema




As noted in Section 12.3, the primal–dual schema is the method of choice
for designing approximation algorithms since it yields combinatorial algo-
rithms with good approximation factors and good running times. We will
ﬁrst present the central ideas behind this schema and then use it to design a
simple f factor algorithm for set cover, where f is the frequency of the most
frequent element.
    The primal–dual schema has its origins in the design of exact algorithms.
In that setting, this schema yielded the most eﬃcient algorithms for some of
the cornerstone problems in P, including matching, network ﬂow, and short-
est paths. These problems have the property that their LP-relaxations have
integral optimal solutions. By Theorem 12.3 we know that optimal solutions
to linear programs are characterized by fact that they satisfy all the comple-
mentary slackness conditions. In fact, the primal–dual schema for exact al-
gorithms is driven by these conditions. Starting with initial feasible solutions
to the primal and dual programs, it iteratively starts satisfying complemen-
tary slackness conditions. When they are all satisﬁed, both solutions must
be optimal. During the iterations, the primal is always modiﬁed integrally,
so that eventually we get an integral optimal solution.
    Consider an LP-relaxation for an NP-hard problem. In general, the re-
laxation will not have an optimal solution that is integral. Does this rule out
a complementary slackness condition driven approach? Interestingly enough,
the answer is ‘no’. It turns out that the algorithm can be driven by a suit-
able relaxation of these conditions! This is the most commonly used way of
designing primal–dual based approximation algorithms – but not the only
way.


15.1 Overview of the schema

Let us consider the following primal program, written in standard form.
                   n
                   
      minimize           cj xj
                   j=1
126      15   Set Cover via the Primal–Dual Schema
                        n
                        
      subject to              aij xj ≥ bi ,             i = 1, . . . , m
                        j=1
                        xj ≥ 0,                         j = 1, . . . , n

where aij , bi , and cj are speciﬁed in the input. The dual program is:
                        m
                        
      maximize                bi y i
                        i=1
                        m
                        
      subject to              aij yi ≤ cj ,            j = 1, . . . , n
                        i=1
                        yi ≥ 0,                        i = 1, . . . , m

    Most known approximation algorithms using the primal–dual schema run
by ensuring one set of conditions and suitably relaxing the other. In the fol-
lowing description we capture both situations by relaxing both conditions.
Eventually, if primal conditions are ensured, we set α = 1, and if dual condi-
tions are ensured, we set β = 1.
Primal complementary slackness conditions
   Let α ≥ 1.                                  m
   For each 1 ≤ j ≤ n: either xj = 0 or cj /α ≤ i=1 aij yi ≤ cj .
Dual complementary slackness conditions
  Let β ≥ 1.                               n
  For each 1 ≤ i ≤ m: either yi = 0 or bi ≤ j=1 aij xj ≤ β · bi ,
Proposition 15.1 If x and y are primal and dual feasible solutions satisfy-
ing the conditions stated above then
      n
                              m
                               
            cj xj ≤ α · β ·            bi y i .
      j=1                      i=1


Proof:
                          m                                                       
      n
                       n
                                                             m
                                                                        n
                                                                         
            cj xj ≤ α                   aij yi        xj = α                  aij xj  yi
      j=1               j=1     i=1                            i=1       j=1
                         m
                         
                 ≤ αβ          bi y i .                                                      (15.1)
                         i=1

The ﬁrst and second inequalities follow from the primal and dual conditions,
respectively. The equality follows by simply changing the order of summation.
✷
                            15.2     Primal–dual schema applied to set cover   127

    The algorithm starts with a primal infeasible solution and a dual feasible
solution; these are usually the trivial solutions x = 0 and y = 0. It iteratively
improves the feasibility of the primal solution, and the optimality of the dual
solution, ensuring that in the end a primal feasible solution is obtained and
all conditions stated above, with a suitable choice of α and β, are satisﬁed.
The primal solution is always extended integrally, thus ensuring that the ﬁnal
solution is integral. The improvements to the primal and the dual go hand-
in-hand: the current primal solution is used to determine the improvement
to the dual, and vice versa. Finally, the cost of the dual solution is used as a
lower bound on OPT, and by Proposition 15.1, the approximation guarantee
of the algorithm is αβ.


15.2 Primal–dual schema applied to set cover
Let us obtain a factor f algorithm for the set cover problem using the primal–
dual schema. For this algorithm, we will choose α = 1 and β = f . We will work
with the primal and dual pair of LP’s given in (13.2) and (13.3), respectively.
The complementary slackness conditions are:
Primal conditions
                                
      ∀S ∈ S : xS = 0 ⇒             ye = c(S).
                            e: e∈S

                                  
Set S will be said to be tight if e: e∈S ye = c(S). Since we will increment
the primal variables integrally, we can state the conditions as: Pick only tight
sets in the cover.
Clearly, in order to maintain dual feasibility, we are not allowed to overpack
any set.
Dual conditions
                        
      ∀e : ye = 0 ⇒            xS ≤ f
                       S: e∈S


Since we will ﬁnd a 0/1 solution for x, these conditions are equivalent to:
Each element having a nonzero dual value can be covered at most f times.
Since each element is in at most f sets, this condition is trivially satisﬁed for
all elements.
    The two sets of conditions naturally suggest the following algorithm:
128     15   Set Cover via the Primal–Dual Schema


 Algorithm 15.2 (Set cover – factor f )
  1. Initialization: x ← 0; y ← 0
  2. Until all elements are covered, do:
      Pick an uncovered element, say e, and raise ye until some set goes
         tight.
      Pick all tight sets in the cover and update x.
      Declare all the elements occurring in these sets as “covered”.
  3. Output the set cover x.


Theorem 15.3 Algorithm 15.2 achieves an approximation factor of f .

Proof: Clearly there will be no uncovered elements and no overpacked sets
at the end of the algorithm. Thus, the primal and dual solutions will both be
feasible. Since they satisfy the relaxed complementary slackness conditions
with α = f , by Proposition 15.1 the approximation factor is f .           ✷

Example 15.4 A tight example for this algorithm is provided by the fol-
lowing set system:




                      1           1                             1
                                                ...
                      e1           e2                         en-1



                                          en



                                               en+1                    1+ε


Here, S consists of n − 1 sets of cost 1, {e1 , en }, . . . , {en−1 , en }, and one set
of cost 1 + ε, {e1 , . . . , en+1 }, for a small ε > 0. Since en appears in all n sets,
this set system has f = n.
    Suppose the algorithm raises yen in the ﬁrst iteration. When yen is raised
to 1, all sets {ei , en }, i = 1, . . . , n − 1 go tight. They are all picked in the
cover, thus covering the elements e1 , . . . , en . In the second iteration, yen+1 is
raised to ε and the set {e1 , . . . , en+1 } goes tight. The resulting set cover has
a cost of n + ε, whereas the optimum cover has cost 1 + ε.                           ✷
                                                           15.4   Notes     129

15.3 Exercises

15.1 How is the algorithm given in Exercise 2.11 for the weighted vertex
cover problem related to Algorithm 15.2 for the case f = 2?

15.2 Remove the scaﬀolding of linear programming from Algorithm 15.2 to
obtain a purely combinatorial factor f algorithm for set cover.
Hint: See the algorithm in Exercise 2.11.

15.3 Let k be a ﬁxed constant, and consider instances of set cover whose
maximum frequency, f , is bounded by k. Algorithm 15.2 shows that the
integrality gap of LP (13.2) is upper bounded by k for these instances. Provide
examples to show that this bound is essentially tight.
Hint: Consider a regular hypergraph, G, on n vertices which has a hyperedge
corresponding to each choice of k of the n vertices. Construct the set system
as follows. It has an element corresponding to each hyperedge and a set
corresponding to each vertex, with incidence deﬁning inclusion.

15.4 The following LP-relaxation is exact for the maximum weight matching
problem (see deﬁnition in Exercise 12.8) in bipartite graphs but not in gen-
eral graphs. Give a primal–dual algorithm, relaxing complementary slackness
conditions appropriately, to show that the integrality gap of this LP is ≥ 1/2.
What is the best upper bound you can place on the integrality gap?
                   
     maximize           we xe                                             (15.2)
                    e
                          
     subject to                         xe ≤ 1,   v∈V
                   e: e incident at v
                   xe ≥ 0,                        e∈E


15.5 (Chudak, Goemans, Hochbaum, and Williamson [46]) Interpret the
layering-based algorithms obtained for set cover and feedback vertex set prob-
lems in Chapters 2 and 6 as primal–dual schema based algorithms. How are
the complementary slackness conditions being relaxed?


15.4 Notes

Kuhn [179] gave the ﬁrst primal–dual algorithm – for the weighted bipartite
matching problem – however, he used the name “Hungarian Method” to
describe his algorithm. Dantzig, Ford, and Fulkerson [60] used this method
130    15   Set Cover via the Primal–Dual Schema

for giving another means of solving linear programs and called it the primal–
dual method. Although the schema was not very successful for solving linear
programs, it soon found widespread use in combinatorial optimization.
    Algorithm 15.2 is due to Bar-Yehuda and Even [20]. Although it was
not originally stated as a primal–dual algorithm, in retrospect, this was the
ﬁrst use of the schema in approximation algorithms. The works of Agrawal,
Klein, and Ravi [1] and Goemans and Williamson [105] revived the use of
this schema in the latter setting, and introduced the powerful idea of growing
duals in a synchronized manner (see Chapter 22). The mechanism of relax-
ing complementary slackness conditions was ﬁrst formalized in Williamson,
Goemans, Mihail, and Vazirani [258]. For further historical information, see
Goemans and Williamson [107].
