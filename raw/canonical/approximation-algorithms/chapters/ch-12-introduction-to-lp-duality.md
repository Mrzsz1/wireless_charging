---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-12"
chapter_number: 12
chapter_title: "Introduction to LP-Duality"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 111
source_page_end: 125
printed_page_start: 93
printed_page_end: 107
part_ids: ["approximation-algorithms-ch-12-part-013"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Introduction to LP-Duality

12 Introduction to LP-Duality




A large fraction of the theory of approximation algorithms, as we know it to-
day, is built around linear programming (LP). In Section 12.1 we will review
some key concepts from this theory. In Section 12.2 we will show how the
LP-duality theorem gives rise to min-max relations which have far-reaching
algorithmic signiﬁcance. Finally, in Section 12.3 we introduce the two funda-
mental algorithm design techniques of rounding and the primal–dual schema,
as well as the method of dual ﬁtting, which yield all the algorithms of Part
II of this book.


12.1 The LP-duality theorem
Linear programming is the problem of optimizing (i.e., minimizing or maxi-
mizing) a linear function subject to linear inequality constraints. The function
being optimized is called the objective function. Perhaps the most interesting
fact about this problem from our perspective is that it is well-characterized
(see deﬁnition in Section 1.2). Let us illustrate this through a simple example.

      minimize      7x1 + x2 + 5x3

      subject to     x1 −     x2 + 3x3 ≥ 10
                    5x1 + 2x2 −        x3 ≥ 6
                    x1 , x2 , x3 ≥ 0

    Notice that in this example all constraints are of the kind “≥” and all
variables are constrained to be nonnegative. This is the standard form of a
minimization linear program; a simple transformation enables one to write
any minimization linear program in this manner. The reason for choosing
this form will become clear shortly.
    Any solution, i.e., a setting for the variables in this linear program, that
satisﬁes all the constraints is said to be a feasible solution. Let z ∗ denote the
optimum value of this linear program. Let us consider the question, “Is z ∗ at
most α?” where α is a given rational number. For instance, let us ask whether
z ∗ ≤ 30. A Yes certiﬁcate for this question is simply a feasible solution whose
94     12   Introduction to LP-Duality

objective function value is at most 30. For example, x = (2, 1, 3) constitutes
such a certiﬁcate since it satisﬁes the two constraints of the problem, and the
objective function value for this solution is 7 · 2 + 1 + 5 · 3 = 30. Thus, any
Yes certiﬁcate to this question provides an upper bound on z ∗ .
    How do we provide a No certiﬁcate for such a question? In other words,
how do we place a good lower bound on z ∗ ? In our example, one such bound
is given by the ﬁrst constraint: since the xi ’s are restricted to be nonnegative,
term-by-term comparison of coeﬃcients shows that 7x1 + x2 + 5x3 ≥ x1 −
x2 + 3x3 . Since the right-hand side of the ﬁrst constraint is 10, the objective
function is at least 10 for any feasible solution. A better lower bound can be
obtained by taking the sum of the two constraints: for any feasible solution
x,

      7x1 + x2 + 5x3 ≥ (x1 − x2 + 3x3 ) + (5x1 + 2x2 − x3 ) ≥ 16.

    The idea behind this process of placing a lower bound is that we are ﬁnding
suitable nonnegative multipliers for the constraints so that when we take their
sum, the coeﬃcient of each xi in the sum is dominated by the coeﬃcient in
the objective function. Now, the right-hand side of this sum is a lower bound
on z ∗ since any feasible solution has a nonnegative setting for each xi . Notice
the importance of ensuring that the multipliers are nonnegative: they do not
reverse the direction of the constraint inequality.
    Clearly, the rest of the game lies in choosing the multipliers in such a
way that the right-hand side of the sum is as large as possible. Interestingly
enough, the problem of ﬁnding the best such lower bound can be formulated
as a linear program:

      maximize      10y1 + 6y2

      subject to     y1 + 5y2        ≤ 7
                    −y1 + 2y2 ≤ 1
                     3y1 −         y2 ≤ 5
                    y 1 , y2 ≥ 0

    Here y1 and y2 were chosen to be the nonnegative multipliers for the ﬁrst
and the second constraint, respectively. Let us call the ﬁrst linear program
the primal program and the second the dual program. There is a systematic
way of obtaining the dual of any linear program; one is a minimization prob-
lem and the other is a maximization problem. Further, the dual of the dual
is the primal program itself (Exercise 12.1). By construction, every feasible
solution to the dual program gives a lower bound on the optimum value of
the primal. Observe that the reverse also holds. Every feasible solution to
the primal program gives an upper bound on the optimal value of the dual.
Therefore, if we can ﬁnd feasible solutions for the dual and the primal with
                                               12.1    The LP-duality theorem      95

matching objective function values, then both solutions must be optimal. In
our example, x = (7/4, 0, 11/4) and y = (2, 1) both achieve objective func-
tion values of 26, and thus both are optimal solutions (see ﬁgure below).
The reader may wonder whether our example was ingeniously constructed to
make this happen. Surprisingly enough, this is not an exception, but the rule!
This is the central theorem of linear programming: the LP-duality theorem.



                          dual opt = primal opt

    0                              26                                            ∞✲



     ✛                             ✲✛                                             ✲
            dual solutions                             primal solutions




    In order to state this theorem formally, let us consider the following min-
imization problem, written in standard form, as the primal program; equiv-
alently, we could have started with a maximization problem as the primal
program.
                   n
                   
      minimize            cj xj                                                 (12.1)
                    j=1

                   n
                   
      subject to          aij xj ≥ bi ,   i = 1, . . . , m
                    j=1
                   xj ≥ 0,                j = 1, . . . , n

where aij , bi , and cj are given rational numbers.
Introducing variables yi for the ith inequality, we get the dual program:
                   m
                   
      maximize            bi y i                                                (12.2)
                    i=1
                   m
                   
      subject to          aij yi ≤ cj ,   j = 1, . . . , n
                    i=1
                   yi ≥ 0,                i = 1, . . . , m

Theorem 12.1 (LP-duality theorem) The primal program has ﬁnite
optimum iﬀ its dual has ﬁnite optimum. Moreover, if x∗ = (x∗1 , . . . , x∗n ) and
96     12       Introduction to LP-Duality

                      ∗
y ∗ = (y1∗ , . . . , ym ) are optimal solutions for the primal and dual programs,
respectively, then
      n
                      m
                       
            cj x∗j =          bi yi∗ .
      j=1               i=1


    Notice that the LP-duality theorem is really a min–max relation, since
one program is a minimization problem and the other is a maximization
problem. A corollary of this theorem is that the linear programming problem
is well-characterized. Feasible solutions to the primal (dual) provide Yes (No)
certiﬁcates to the question, “Is the optimum value less than or equal to α?”
Thus, as a corollary of this theorem we get that linear programming is in
NP ∩ co-NP.
    Going back to our example, by construction, any feasible solution to the
dual program gives a lower bound on the optimal value of the primal. In
fact, it also gives a lower bound on the objective function value achieved by
any feasible solution to the primal. This is the easy half of the LP-duality
theorem, sometimes called the weak duality theorem. We give a formal proof
of this theorem, since some steps in the proof will lead to the next important
fact. The design of several exact algorithms have their basis in the LP-duality
theorem. In contrast, in approximation algorithms, typically the weak duality
theorem suﬃces.

Theorem 12.2 (Weak duality theorem) If x = (x1 , . . . , xn ) and y =
(y1 , . . . , ym ) are feasible solutions for the primal and dual program, respec-
tively, then
      n
                      m
                       
            cj xj ≥           bi y i .                                     (12.3)
      j=1              i=1



Proof: Since y is dual feasible and xj ’s are nonnegative,

      n                n
                         m                       
                        
            cj xj ≥                      aij yi       xj .                 (12.4)
      j=1              j=1        i=1


Similarly, since x is primal feasible and yi ’s are nonnegative,
                             
      m
               n
                                          m
                                           
                     aij xj  yi ≥               bi y i .                 (12.5)
      i=1       j=1                        i=1


The theorem follows by observing that
                                             12.2      Min–max relations and LP-duality   97
         m                                                
       n
                                     m
                                                n
                                                 
                   aij yi       xj =                  aij xj  yi .
       j=1   i=1                       i=1       j=1


                                                                                          ✷
    By the LP-duality theorem, x and y are both optimal solutions iﬀ (12.3)
holds with equality. Clearly, this happens iﬀ both (12.4) and (12.5) hold with
equality. Hence, we get the following result about the structure of optimal
solutions:

Theorem 12.3 (Complementary slackness conditions) Let x and y
be primal and dual feasible solutions, respectively. Then, x and y are both
optimal iﬀ all of the following conditions are satisﬁed:
 Primal complementary slackness conditions
                                       m
   For each 1 ≤ j ≤ n: either xj = 0 or i=1 aij yi = cj ; and
 Dual complementary slackness conditions
                                       n
   For each 1 ≤ i ≤ m: either yi = 0 or j=1 aij xj = bi .

    The complementary slackness conditions play a vital role in the design
of eﬃcient algorithms, both exact and approximation; see Chapter 15 for
details. (For a better appreciation of their importance, we recommend that
the reader study algorithms for the weighted matching problem, see Section
12.5.)


12.2 Min–max relations and LP-duality
In order to appreciate the role of LP-duality theory in approximation algo-
rithms, it is important to ﬁrst understand its role in exact algorithms. To
do so, we will review some of these ideas in the context of the max-ﬂow
min-cut theorem. In particular, we will show how this and other min–max
relations follow from the LP-duality theorem. Some of the ideas on cuts and
ﬂows developed here will also be used in the study of multicommodity ﬂow
in Chapters 18, 20, and 21.
    The problem of computing a maximum ﬂow in a network is: given a
directed1 graph, G = (V, E) with two distinguished nodes, source s and sink
t, and positive arc capacities, c : E → R+ , ﬁnd the maximum amount of ﬂow
that can be sent from s to t subject to
1. capacity constraint: for each arc e, the ﬂow sent through e is bounded by
   its capacity, and
1
    The maximum ﬂow problem in undirected graphs reduces to that in directed
    graphs: replace each edge (u, v) by two directed edges, (u → v) and (v → u),
    each of the same capacity as (u, v).
98     12   Introduction to LP-Duality

 2. ﬂow conservation: at each node v, other than s and t, the total ﬂow into
    v should equal the total ﬂow out of v.
    An s–t cut is deﬁned by a partition of the nodes into two sets X and X
so that s ∈ X and t ∈ X, and consists of the set of arcs going from X to X.
The capacity of this cut, c(X, X), is deﬁned to be the sum of capacities of
these arcs. Because of the capacity constraints on ﬂow, the capacity of any
s–t cut is an upper bound on any feasible ﬂow. Thus, if the capacity of an
s–t cut, say (X, X), equals the value of a feasible ﬂow, then (X, X) must be a
minimum s–t cut and the ﬂow must be a maximum ﬂow in G. The max-ﬂow
min-cut theorem proves that it is always possible to ﬁnd a ﬂow and an s–t
cut so that equality holds.
    Let us formulate the maximum ﬂow problem as a linear program. First,
introduce a ﬁctitious arc of inﬁnite capacity from t to s, thus converting the
ﬂow to a circulation; the objective now is to maximize the ﬂow on this arc,
denoted by fts . The advantage of making this modiﬁcation is that we can
now require ﬂow conservation at s and t as well. If fij denotes the amount of
ﬂow sent through arc (i, j) ∈ E, we can formulate the maximum ﬂow problem
as follows:

      maximize       fts

      subject to     fij ≤ cij ,                                    (i, j) ∈ E
                                            
                           fji −                       fij ≤ 0,     i∈V
                   j: (j,i)∈E             j: (i,j)∈E

                     fij ≥ 0,                                       (i, j) ∈ E

The second set of inequalities say that for each node i, the total ﬂow into i
is at most the total ﬂow out of i. Notice that if this inequality holds at each
node, then in fact it must be satisﬁed with equality at each node, thereby
implying ﬂow conservation at each node (this is so because a deﬁcit in ﬂow
balance at one node implies a surplus at some other node). With this trick,
we get a linear program in standard form.
    To obtain the dual program we introduce variables dij and pi correspond-
ing to the two types of inequalities in the primal. We will view these variables
as distance labels on arcs and potentials on nodes, respectively. The dual pro-
gram is:
                       
      minimize                  cij dij                                          (12.6)
                     (i,j)∈E

      subject to     dij − pi + pj ≥ 0,                (i, j) ∈ E
                     ps − pt ≥ 1
                     dij ≥ 0,                          (i, j) ∈ E
                                          12.2   Min–max relations and LP-duality      99

                      pi ≥ 0,                      i∈V                              (12.7)

   For developing an intuitive understanding of the dual program, it will be
best to ﬁrst transform it into an integer program that seeks 0/1 solutions to
the variables:
                        
      minimize                  cij dij
                      (i,j)∈E

      subject to      dij − pi + pj ≥ 0,           (i, j) ∈ E
                      ps − pt ≥ 1
                      dij ∈ {0, 1},                (i, j) ∈ E
                      pi ∈ {0, 1},                 i∈V

     Let (d∗ , p∗ ) be an optimal solution to this integer program. The only way
to satisfy the inequality p∗s − p∗t ≥ 1 with a 0/1 substitution is to set p∗s = 1
and p∗t = 0. This solution naturally deﬁnes an s–t cut (X, X), where X is the
set of potential 1 nodes, and X the set of potential 0 nodes. Consider an arc
(i, j) with i ∈ X and j ∈ X. Since p∗i = 1 and p∗j = 0, by the ﬁrst constraint,
d∗ij ≥ 1. But since we have a 0/1 solution, d∗ij = 1. The distance label for
each of the remaining arcs can be set to either 0 or 1 without violating the
ﬁrst constraint; however, in order to minimize the objective function value,
it must be set to 0. The objective function value must thus be equal to the
capacity of the cut (X, X), and (X, X) must be a minimum s–t cut.
     Thus, the previous integer program is a formulation of the minimum s–t
cut problem! What about the dual program? The dual program can be viewed
as a relaxation of the integer program where the integrality constraint on the
variables is dropped. This leads to the constraints 1 ≥ dij ≥ 0 for (i, j) ∈ E
and 1 ≥ pi ≥ 0 for i ∈ V . Next, we notice that the upper bound constraints
on the variables are redundant; their omission cannot give a better solution.
Dropping these constraints gives the dual program in the form given above.
We will say that this program is the LP-relaxation of the integer program.
     Consider an s–t cut C. Set C has the property that any path from s to t
in G contains at least one edge of C. Using this observation, we can interpret
any feasible solution to the dual program as a fractional s–t cut: the distance
labels it assigns to arcs satisfy the property that on any path from s to t
the distance labels add up to at least 1. To see this, consider an s–t path
(s = v0 , v1 , . . . , vk = t). Now, the sum of the potential diﬀerences on the
endpoints of arcs on this path is

      k−1
      
            (pi − pi+1 ) = ps − pt .
      i=0
100     12   Introduction to LP-Duality

By the ﬁrst constraint, the sum of the distance labels on the arcs must add
up to at least ps −pt , which is ≥ 1. Let us deﬁne the capacity of this fractional
s–t cut to be the dual objective function value achieved by it.
    In principle, the best fractional s–t cut could have lower capacity than
the best integral cut. Surprisingly enough, this does not happen. Consider
the polyhedron deﬁning the set of feasible solutions to the dual program.
Let us call a feasible solution an extreme point solution if it is a vertex of
this polyhedron, i.e., it cannot be expressed as a convex combination of two
feasible solutions. From linear programming theory we know that for any
objective function, i.e., assignment of capacities to the arcs of G, there is an
extreme point solution that is optimal (for this discussion let us assume that
for the given objective function, an optimal solution exists). Now, it can be
proven that each extreme point solution of the polyhedron is integral, with
each coordinate being 0 or 1 (see Exercise 12.6). Thus, the dual program
always has an integral optimal solution.
    By the LP-duality theorem maximum ﬂow in G must equal capacity of
a minimum fractional s–t cut. But since the latter equals the capacity of a
minimum s–t cut, we get the max-ﬂow min-cut theorem.
    The max-ﬂow min-cut theorem is therefore a special case of the LP-duality
theorem; it holds because the dual polyhedron has integral vertices. In fact,
most min–max relations in combinatorial optimization hold for a similar rea-
son.
    Finally, let us illustrate the usefulness of complementary slackness condi-
tions by utilizing them to derive additional properties of optimal solutions to
the ﬂow and cut programs. Let f ∗ be an optimum solution to the primal LP
(i.e., a maximum s–t ﬂow). Also, let (d∗ , p∗ ) be an integral optimum solution
to the dual LP, and let (X, X) be the cut deﬁned by (d∗ , p∗ ). Consider an
arc (i, j) such that i ∈ X and j ∈ X. We have proven above that d∗ij = 1.
Since d∗ij = 0, by the dual complementary slackness condition, fij      ∗
                                                                            = cij .
                                                                     ∗
Next, consider an arc (k, l) such that k ∈ X and l ∈ X. Since pk − p∗l = −1,
and d∗kl ∈ {0, 1}, the constraint d∗kl − p∗k + p∗l ≥ 0 must be satisﬁed as a strict
                                                                     ∗
inequality. By the primal complementary slackness condition, fkl       = 0. Thus,
we have proven that arcs going from X to X are saturated by f ∗ and the
reverse arcs carry no ﬂow. (Observe that it was not essential to invoke com-
plementary slackness conditions to prove these facts; they also follow from
the fact that ﬂow across cut (X, X) equals its capacity.)


12.3 Two fundamental algorithm design techniques
We can now explain why linear programming is so useful in approximation al-
gorithms. Many combinatorial optimization problems can be stated as integer
programs. Once this is done, the linear relaxation of this program provides
a natural way of lower bounding the cost of the optimal solution. As stated
in Chapter 1, this is typically a key step in the design of an approximation
                     12.3   Two fundamental algorithm design techniques       101

algorithm. As in the case of the minimum s–t cut problem, a feasible solution
to the LP-relaxation can be thought of as a fractional solution to the original
problem. However, in the case of an NP-hard problem, we cannot expect
the polyhedron deﬁning the set of feasible solutions to have integer vertices.
Thus, our task is not to look for an optimal solution to the LP-relaxation,
but rather a near-optimal integral solution.
    There are two basic techniques for obtaining approximation algorithms
using linear programming. The ﬁrst, and more obvious, method is to solve
the linear program and then convert the fractional solution obtained into
an integral solution, trying to ensure that in the process the cost does not
increase much. The approximation guarantee is established by comparing
the cost of the integral and fractional solutions. This technique is called LP-
rounding or simply rounding.
    The second, less obvious and perhaps more sophisticated, method is to use
the dual of the LP-relaxation in the design of the algorithm. This technique
is called the primal–dual schema. Let us call the LP-relaxation the primal
program. Under this schema, an integral solution to the primal program and
a feasible solution to the dual program are constructed iteratively. Notice
that any feasible solution to the dual also provides a lower bound on OPT.
The approximation guarantee is established by comparing the two solutions.
    Both these techniques have been used extensively to obtain algorithms for
many fundamental problems. Fortunately, once again, these techniques can
be illustrated in the simple setting of the set cover problem. This is done in
Chapters 14 and 15. Later chapters will present ever more sophisticated use
of these techniques for solving a variety of problems.
    LP-duality theory has also been useful in analyzing combinatorially ob-
tained approximation algorithms, using the method of dual ﬁtting. In Chapter
13 we will give an alternative analysis of the greedy set cover algorithm, Al-
gorithm 2.2, using this method. This method has also been used to analyze
greedy algorithms for the metric uncapacitated facility location problem (see
Exercise 24.12). The method seems quite basic and should ﬁnd other appli-
cations as well.

12.3.1 A comparison of the techniques and the notion of
integrality gap

The reader may suspect that from the viewpoint of approximation guarantee,
the primal–dual schema is inferior to rounding, since an optimal solution to
the primal gives a tighter lower bound than a feasible solution to the dual. It
turns out that this is not so. In order to give a formal explanation, we need
to introduce the crucial notion of integrality gap of an LP-relaxation.
    Given an LP-relaxation for a minimization problem Π, let OPTf (I) de-
note the cost of an optimal fractional solution to instance I, i.e., the objective
function value of an optimal solution to the LP-relaxation. Deﬁne the inte-
102     12    Introduction to LP-Duality

grality gap, sometimes also called the integrality ratio, of the relaxation to
be

             OPT(I)
      sup             ,
       I     OPTf (I)

i.e., the supremum of the ratio of the optimal integral and fractional solutions.
In the case of a maximization problem, the integrality gap will be deﬁned to be
the inﬁmum of this ratio. As stated in Section 12.2, most min–max relations
arise from LP-relaxations that always have integral optimal solutions. Clearly,
the integrality gap of such an LP is 1. We will call such an LP-relaxation an
exact relaxation.
     If the cost of the solution found by the algorithm is compared directly
with the cost of an optimal fractional solution (or a feasible dual solution),
as is done in most algorithms, the best approximation factor we can hope to
prove is the integrality gap of the relaxation (see Exercise 12.4). Interestingly
enough, for many problems, both techniques have been successful in yielding
algorithms having guarantees essentially equal to the integrality gap of the
relaxation.
     The main diﬀerence in performance between the two techniques lies in the
running times of the algorithms produced. An LP-rounding algorithm needs
to ﬁnd an optimal solution to the linear programming relaxation. Since linear
programming is in P, this can be done in polynomial time if the relaxation
has polynomially many constraints. Even if the relaxation has exponentially
many constraints, this may still be achievable, if a polynomial time separa-
tion oracle can be constructed, i.e., a polynomial time algorithm that given
a point in Rn , where n is the number of variables in the relaxation, either
conﬁrms that this point is a feasible solution (i.e., satisﬁes all constraints), or
produces a violated constraint (see the notes in Section 12.5 for references).
The running time for both possibilities is high; for the second it may be ex-
orbitant. Let us remark that for certain problems, extreme point solutions
have additional structural properties and some LP-rounding algorithms re-
quire such a solution to the linear programming relaxation. Such solutions
can also be found in polynomial time.
     On the other hand, the primal–dual schema leaves enough room to exploit
the special combinatorial structure of individual problems and is thereby
able to yield algorithms having good running times. It provides only a broad
outline of the algorithm – the details have to be designed individually for
speciﬁc problems. In fact, for many problems, once the algorithm has been
designed using the primal–dual schema, the scaﬀolding of linear programming
can be completely dispensed with to get a purely combinatorial algorithm.
     This brings us to another advantage of the primal–dual schema – this time
not objectively quantiﬁable. A combinatorial algorithm is more malleable
than an algorithm that requires an LP-solver. Once a basic problem is solved
using the primal–dual schema, one can also solve variants and generalizations
                                                         12.4   Exercises    103

of the basic problem. Exercises in Chapters 22 and 24 illustrate this point.
From a practical standpoint, a combinatorial algorithm is more useful, since
it is easier to adapt it to speciﬁc applications and ﬁne tune its performance
for speciﬁc types of inputs.


12.4 Exercises

12.1 Show that the dual of the dual of a linear program is the original
program itself.

12.2 Show that any minimization program can be transformed into an equiv-
alent program in standard form, i.e., the form of LP (12.1).

12.3 Change some of the constraints of the primal program (12.1) into
equalities, i.e., so they are of the form
       n
       
             aij xj = bi , i ∈ I.
       j=1


Show that the dual of this program involves modifying program (12.2) so that
the corresponding dual variables yi , i ∈ I are unconstrained, i.e., they are not
constrained to be nonnegative. Additionally, if some of the variables xj , j ∈ J
in program (12.1) are unconstrained, then the corresponding constraints in
the dual become equalities.

12.4 Is the following a theorem: An approximation algorithm designed using
an LP-relaxation cannot achieve a better approximation guarantee than the
integrality gap of the relaxation.
Hint: In principle it may be possible to show, using additional structural
properties, that whenever an instance has a bad gap, the cost of the solution
found by the algorithm is much less that αOPT, where α is the integrality
gap of the relaxation. (Observe that if the instance has a bad gap, the cost
of the solution found cannot be much less than αOPTf .)

12.5    Use the max-ﬂow min-cut theorem to derive Menger’s Theorem:
Theorem 12.4 Let G = (V, E) be a directed graph with s, t ∈ V . Then, the
maximum number of edge-disjoint (vertex-disjoint) s–t paths is equal to the
minimum number of edges (vertices) whose removal disconnects s from t.


12.6 Show that each extreme point solution for LP (12.6) is 0/1, and hence
represents a valid cut.
104      12     Introduction to LP-Duality

Hint: An n × m matrix A is said to be totally unimodular if the determinant
of every square submatrix of A is 1, −1, or 0. Show, by induction, that the
constraint matrix of this LP is totally unimodular. Also, use the fact that
a feasible solution for a set of linear inequalities in Rn is an extreme point
solution iﬀ it satisﬁes n linearly independent inequalities with equality.

12.7 This exercise develops a proof of the König-Egerváry Theorem (Theo-
rem 1.6). Let G = (V, E) be a bipartite graph.
1. Show that the following is an exact LP-relaxation (i.e., always has an
   integral optimal solution) for the maximum matching problem in G.
                           
              maximize          xe                                         (12.8)
                            e
                                     
              subject to                        xe ≤ 1,   v∈V
                           e: e incident at v
                           xe ≥ 0,                        e∈E

   Hint: Using the technique of Exercise 12.6 show that each extreme point
   solution for LP (12.8) is 0/1, and hence represents a valid matching.
2. Obtain the dual of this LP and show that it is an exact LP-relaxation for
   the problem of ﬁnding a minimum vertex cover in bipartite graph G.
3. Use the previous result to derive the König-Egerváry Theorem.

12.8 (Edmonds [68])
1. Let G = (V, E) be an undirected graph, with weights we on edges. The
   following is an exact LP-relaxation for the problem of ﬁnding a maximum
   weight matching in G. (By e : e ∈ S we mean edges e that have both
   endpoints in S.)
                           
              maximize          we xe                                      (12.9)
                            e
                                     
              subject to                        xe ≤ 1,   v∈V
                           e: e incident at v
                                          |S| − 1
                                    xe ≤           ,      S ⊂ V, |S| odd
                                              2
                           e: e∈S
                           xe ≥ 0,                        e∈E

      Obtain the dual of this LP. If the weight function is integral, the dual is
      also exact. Observe that Theorem 1.7 follows from these facts.
                                                               12.4   Exercises      105

 2. Assume that |V | is even. The following is an exact LP-relaxation for the
    minimum weight perfect matching problem in G (a matching is perfect
    if it matches all vertices). Obtain the dual of this LP. Use complemen-
    tary slackness conditions to give conditions satisﬁed by a pair of optimal
    primal (integral) and dual solutions for both formulations.
                           
          minimize              we xe                                             (12.10)
                            e
                                    
          subject to                             xe = 1,   v∈V
                           e: e incident at v
                                          |S| − 1
                                    xe ≤           ,       S ⊂ V, |S| odd
                                              2
                           e: e∈S
                           xe ≥ 0,                         e∈E


12.9 (Edmonds [71]) Show that the following is an integer programming
formulation for the minimum spanning tree (MST) problem. Assume we are
given graph G = (V, E), |V | = n, with cost function c : E → Q+ . For
A ⊆ E, we will denote by κ(A) the number of connected components in
graph GA = (V, A).
                     
      minimize             ce xe                                                  (12.11)
                       e
                     
      subject to           xe = n − κ(A),          A⊂E
                     e∈A
                     
                           xe = n − 1
                     e∈E
                     xe ∈ {0, 1},                  e∈E

The rest of this exercise develops a proof that the LP-relaxation of this integer
program is exact for the MST problem.

             will be convenient to change the objective function of IP (12.11)
 1. First, it
    to max e −ce xe . Obtain the LP-relaxation and dual of this modiﬁed
    formulation.
 2. Consider the primal solution produced by Kruskal’s algorithm. Let e1 , . . . ,
    em be the edges sorted by increasing cost, |E| = m. This algorithm greed-
    ily picks a maximal acyclic subgraph from this sorted list. Obtain a suit-
    able dual feasible solution so that all complementary slackness conditions
    are satisﬁed.
    Hint: Let At = {e1 , . . . , et }. Set yAt = et+1 − et , for 1 ≤ t < m, and
    yE = −cm , where y is the dual variable.
106    12     Introduction to LP-Duality

 3. Show that x is a feasible solution to the above-stated primal program iﬀ
    it is a feasible solution to the following LP. That is, prove that this is
    also an exact relaxation for the MST problem.
                         
            minimize           ce xe                                   (12.12)
                          e
                         
            subject to         xe = |S| − 1,   S⊂V
                         e∈S
                         
                               xe = n − 1
                         e∈E
                         xe ≥ 0,               e∈E


12.10 In this exercise, you will derive von Neumann’s minimax theorem
in game theory from the LP-duality theorem. A ﬁnite two-person zero-sum
game is speciﬁed by an m × n matrix A with real entries. In each round,
the row player, R, selects a row, say i; simultaneously, the column player,
C, selects a column, say j. The payoﬀ to R at the end of this round is aij .
Thus, |aij | is the amount that C pays R (R pays C) if aij is positive (aij
is negative); no money is exchanged if aij is zero. Zero-sum game refers to
the fact that the total amount of money possessed by R and C together is
conserved.
     The strategy of each player is speciﬁed by a vector whose entries are non-
negative and add up to one, giving the probabilities with which the player
picks each row or column. Let R’s strategy be given by m-dimensional vec-
tor x, and C’s strategy be given by n-dimensional vector y. Then, the ex-
pected payoﬀ to R in a round is xT Ay. The job of each player is to pick
a strategy that guarantees maximum possible expected winnings (equiva-
lently, minimum possible expected losses), regardless of the strategy cho-
sen by the other player. If R chooses strategy x, he can be sure of win-
ning only miny xT Ay, where the minimum is taken over all possible strate-
gies of C. Thus, the optimal choice for R is given by maxx miny xT Ay.
Similarly, C will minimize her losses by choosing the strategy given by
miny maxx xT Ay. The minimax theorem states that for every matrix A,
maxx miny xT Ay = miny maxx xT Ay.
     Let us say that a strategy is pure if it picks a single row or column,
i.e., the vector corresponding to it consists of one 1 and the rest 0’s. A key
observation is that for any strategy x of R, miny xT Ay is attained for a pure
strategy of C: Suppose the minimum is attained for strategy y. Consider the
pure strategy corresponding to any nonzero component of y. The fact that
the components of y are nonnegative and add up to one leads to an easy
                                   m the same minimum. Thus, R’s optimum
proof that this pure strategy attains
strategy is given by maxx minj i=1 aij xi . The second critical observation
                                                                 12.5   Notes   107

is that the problem of computing R’s optimal strategy can be expressed as a
linear program:

      maximize      z
                          m
                          
      subject to    z−          aij xi ≤ 0,   j = 1, . . . , n
                          i=1
                    m
                    
                          xi = 1
                    i=1
                    xi ≥ 0,                   i = 1, . . . , m

    Find the dual of this LP and show that it computes the optimal strategy
for C. (Use the fact that for any strategy y of C, maxx xT Ay is attained for a
pure strategy of R.) Hence, prove the minimax theorem using the LP-duality
theorem.


12.5 Notes
For a good introduction to theory of linear programming, see Chvátal [49].
There are numerous other books on the topic, e.g., Dantzig [58], Karloﬀ [160],
Nemhauser and Wolsey [212], and Schrijver [237]. Linear programming has
been extensively used in combinatorial optimization, see Cook, Cunningham,
Pulleyblank, and Schrijver [52], Grötschel, Lovász, and Schrijver [117], Lovász
[194], Lovász and Plummer [195], and Papadimitriou and Steiglitz [217]. For
a good explanation of Edmonds’ weighted matching algorithm, see Lovász
and Plummer [195]. For algorithms for ﬁnding a solution to an LP, given a
separation oracle, see Grötschel, Lovász, and Schrijver [116, 117] and Schrijver
[237].
