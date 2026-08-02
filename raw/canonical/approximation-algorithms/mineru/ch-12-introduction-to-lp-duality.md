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

# Introduction to LP-Duality (MinerU semantic layer)

<!-- source-pages: 111-125; printed-pages: 93-107; mineru-part: approximation-algorithms-ch-12-part-013 -->

## 12 Introduction to LP-Duality

A large fraction of the theory of approximation algorithms, as we know it today, is built around linear programming (LP). In Section 12.1 we will review some key concepts from this theory. In Section 12.2 we will show how the LP-duality theorem gives rise to min-max relations which have far-reaching algorithmic significance. Finally, in Section 12.3 we introduce the two fundamental algorithm design techniques of rounding and the primal–dual schema, as well as the method of dual fitting, which yield all the algorithms of Part II of this book.

## 12.1 The LP-duality theorem

Linear programming is the problem of optimizing (i.e., minimizing or maximizing) a linear function subject to linear inequality constraints. The function being optimized is called the objective function. Perhaps the most interesting fact about this problem from our perspective is that it is well-characterized (see definition in Section 1.2). Let us illustrate this through a simple example.

$$
\begin{array}{l l} \text {minimize} & 7 x _ {1} + x _ {2} + 5 x _ {3} \\ \text {subject to} & x _ {1} - x _ {2} + 3 x _ {3} \geq 1 0 \\ & 5 x _ {1} + 2 x _ {2} - x _ {3} \geq 6 \\ & x _ {1}, x _ {2}, x _ {3} \geq 0 \end{array}
$$

Notice that in this example all constraints are of the kind $\because$ and all variables are constrained to be nonnegative. This is the standard form of a minimization linear program; a simple transformation enables one to write any minimization linear program in this manner. The reason for choosing this form will become clear shortly.

Any solution, i.e., a setting for the variables in this linear program, that satisfies all the constraints is said to be a feasible solution. Let $z ^ { * }$ denote the optimum value of this linear program. Let us consider the question, ${ ^ { 6 } \mathrm { T s } } ~ z ^ { \ast }$ at most $\alpha ? ^ { \mathfrak { n } }$ where α is a given rational number. For instance, let us ask whether $z ^ { * } \leq 3 0$ . A Yes certificate for this question is simply a feasible solution whose objective function value is at most 30. For example, $\pmb { x } = ( 2 , 1 , 3 )$ constitutes such a certificate since it satisfies the two constraints of the problem, and the objective function value for this solution is $7 \cdot 2 + 1 + 5 \cdot 3 = 3 0$ . Thus, any Yes certificate to this question provides an upper bound on $z ^ { * }$

How do we provide a No certificate for such a question? In other words, how do we place a good lower bound on $z ^ { \ast \smash { \gamma } }$ In our example, one such bound is given by the first constraint: since the $x _ { i }$ ’s are restricted to be nonnegative, term-by-term comparison of coeficients shows that $7 x _ { 1 } + x _ { 2 } + 5 x _ { 3 } \geq x _ { 1 } -$ $x _ { 2 } + 3 x _ { 3 }$ . Since the right-hand side of the first constraint is 10, the objective function is at least 10 for any feasible solution. A better lower bound can be obtained by taking the sum of the two constraints: for any feasible solution ${ \mathbf { } } ^ { \mathbf { } } \mathbf { { \mathbf { x } } } ,$

$$
7 x _ {1} + x _ {2} + 5 x _ {3} \geq (x _ {1} - x _ {2} + 3 x _ {3}) + (5 x _ {1} + 2 x _ {2} - x _ {3}) \geq 1 6.
$$

The idea behind this process of placing a lower bound is that we are finding suitable nonnegative multipliers for the constraints so that when we take their sum, the coeficient of each $x _ { i }$ in the sum is dominated by the coeficient in the objective function. Now, the right-hand side of this sum is a lower bound on $z ^ { * }$ since any feasible solution has a nonnegative setting for each $x _ { i }$ . Notice the importance of ensuring that the multipliers are nonnegative: they do not reverse the direction of the constraint inequality.

Clearly, the rest of the game lies in choosing the multipliers in such a way that the right-hand side of the sum is as large as possible. Interestingly enough, the problem of finding the best such lower bound can be formulated as a linear program:

$$
\begin{array}{l l} \text {maximize} & 1 0 y _ {1} + 6 y _ {2} \\ \text {subject to} & y _ {1} + 5 y _ {2} \leq 7 \\ & - y _ {1} + 2 y _ {2} \leq 1 \\ & 3 y _ {1} - y _ {2} \leq 5 \\ & y _ {1}, y _ {2} \geq 0 \end{array}
$$

Here $y _ { 1 }$ and $y _ { 2 }$ were chosen to be the nonnegative multipliers for the first and the second constraint, respectively. Let us call the first linear program the primal program and the second the dual program. There is a systematic way of obtaining the dual of any linear program; one is a minimization problem and the other is a maximization problem. Further, the dual of the dual is the primal program itself (Exercise 12.1). By construction, every feasible solution to the dual program gives a lower bound on the optimum value of the primal. Observe that the reverse also holds. Every feasible solution to the primal program gives an upper bound on the optimal value of the dual. Therefore, if we can find feasible solutions for the dual and the primal with matching objective function values, then both solutions must be optimal. In our example, ${ \pmb x } = ( 7 / 4 , 0 , 1 1 / 4 )$ and $\begin{array} { r } { \pmb { y } = ( 2 , 1 ) } \end{array}$ ) both achieve objective function values of 26, and thus both are optimal solutions (see figure below). The reader may wonder whether our example was ingeniously constructed to make this happen. Surprisingly enough, this is not an exception, but the rule! This is the central theorem of linear programming: the LP-duality theorem.

dual opt = primal opt  
![](images/e9d5bc4ee64216a9dba5a61007d82388804716255059cea3f832420b91399752.jpg)

In order to state this theorem formally, let us consider the following minimization problem, written in standard form, as the primal program; equivalently, we could have started with a maximization problem as the primal program.

$$
\begin{array}{l l} \text {minimize} & \sum_ {j = 1} ^ {n} c _ {j} x _ {j} \\ \text {subject to} & \sum_ {j = 1} ^ {n} a _ {i j} x _ {j} \geq b _ {i}, \quad i = 1, \ldots , m \\ & x _ {j} \geq 0, \quad j = 1, \ldots , n \end{array}\tag{12.1}
$$

where $a _ { i j } , b _ { i }$ , and $c _ { j }$ are given rational numbers.

Introducing variables $y _ { i }$ for the ith inequality, we get the dual program:

$$
\begin{array}{l l} \text {maximize} & \sum_ {i = 1} ^ {m} b _ {i} y _ {i} \\ \text {subject to} & \sum_ {i = 1} ^ {m} a _ {i j} y _ {i} \leq c _ {j}, \quad j = 1, \ldots , n \\ & y _ {i} \geq 0, \quad i = 1, \ldots , m \end{array}\tag{12.2}
$$

Theorem 12.1 (LP-duality theorem) The primal program has finite optimum $i f f$ its dual has finite optimum. Moreover, $i f x ^ { * } = ( x _ { 1 } ^ { * } , \ldots , x _ { n } ^ { * } )$ and $\pmb { y } ^ { * } = ( y _ { 1 } ^ { * } , \ldots , y _ { m } ^ { * } )$ are optimal solutions for the primal and dual programs, respectively, then

$$
\sum_ {j = 1} ^ {n} c _ {j} x _ {j} ^ {*} = \sum_ {i = 1} ^ {m} b _ {i} y _ {i} ^ {*}.
$$

Notice that the LP-duality theorem is really a min–max relation, since one program is a minimization problem and the other is a maximization problem. A corollary of this theorem is that the linear programming problem is well-characterized. Feasible solutions to the primal (dual) provide Yes $( \mathrm { N o ) }$ certificates to the question, “Is the optimum value less than or equal to $\alpha ? ^ { \mathfrak { n } }$ Thus, as a corollary of this theorem we get that linear programming is in NP ∩ co-NP.

Going back to our example, by construction, any feasible solution to the dual program gives a lower bound on the optimal value of the primal. In fact, it also gives a lower bound on the objective function value achieved by any feasible solution to the primal. This is the easy half of the LP-duality theorem, sometimes called the weak duality theorem. We give a formal proof of this theorem, since some steps in the proof will lead to the next important fact. The design of several exact algorithms have their basis in the LP-duality theorem. In contrast, in approximation algorithms, typically the weak duality theorem sufices.

Theorem 12.2 (Weak duality theorem) $I f { \textbf { \em x } } = ( x _ { 1 } , \ldots , x _ { n } )$ and ${ \textbf { \em y } } =$ $\left( y _ { 1 } , \ldots , y _ { m } \right)$ are feasible solutions for the primal and dual program, respectively, then

$$
\sum_ {j = 1} ^ {n} c _ {j} x _ {j} \geq \sum_ {i = 1} ^ {m} b _ {i} y _ {i}.\tag{12.3}
$$

Proof: Since y is dual feasible and $x _ { j } { } ^ { \dag } \mathrm { s }$ are nonnegative,

$$
\sum_ {j = 1} ^ {n} c _ {j} x _ {j} \geq \sum_ {j = 1} ^ {n} \left(\sum_ {i = 1} ^ {m} a _ {i j} y _ {i}\right) x _ {j}.\tag{12.4}
$$

Similarly, since x is primal feasible and $y _ { i } \mathrm { \dot { s } }$ are nonnegative,

$$
\sum_ {i = 1} ^ {m} \left(\sum_ {j = 1} ^ {n} a _ {i j} x _ {j}\right) y _ {i} \geq \sum_ {i = 1} ^ {m} b _ {i} y _ {i}.\tag{12.5}
$$

The theorem follows by observing that

$$
\sum_ {j = 1} ^ {n} \left(\sum_ {i = 1} ^ {m} a _ {i j} y _ {i}\right) x _ {j} = \sum_ {i = 1} ^ {m} \left(\sum_ {j = 1} ^ {n} a _ {i j} x _ {j}\right) y _ {i}.
$$

By the LP-duality theorem, x and y are both optimal solutions if (12.3) holds with equality. Clearly, this happens if both (12.4) and (12.5) hold with equality. Hence, we get the following result about the structure of optimal solutions:

Theorem 12.3 (Complementary slackness conditions) Let x and y be primal and dual feasible solutions, respectively. Then, x and y are both optimal if all of the following conditions are satisfied:

Primal complementary slackness conditions

For each $1 \leq j \leq n \colon$ either $x _ { j } = 0$ or $\begin{array} { r } { \sum _ { i = 1 } ^ { m } a _ { i j } y _ { i } = c _ { j } } \end{array}$ ; and

Dual complementary slackness conditions

For each $1 \leq i \leq m \colon$ either $\begin{array} { r } { y _ { i } = 0 \ o r \sum _ { j = 1 } ^ { n } a _ { i j } x _ { j } = b _ { i } } \end{array}$

The complementary slackness conditions play a vital role in the design of eficient algorithms, both exact and approximation; see Chapter 15 for details. (For a better appreciation of their importance, we recommend that the reader study algorithms for the weighted matching problem, see Section 12.5.)

## 12.2 Min–max relations and LP-duality

In order to appreciate the role of LP-duality theory in approximation algorithms, it is important to first understand its role in exact algorithms. To do so, we will review some of these ideas in the context of the max-flow min-cut theorem. In particular, we will show how this and other min–max relations follow from the LP-duality theorem. Some of the ideas on cuts and flows developed here will also be used in the study of multicommodity flow in Chapters 18, 20, and 21.

The problem of computing a maximum flow in a network is: given a directed<sup>1</sup> graph, $G = ( V , E )$ with two distinguished nodes, source s and sink t, and positive arc capacities, $c : E \to \mathbf { R } ^ { + }$ , find the maximum amount of flow that can be sent from s to t subject to

1. capacity constraint: for each arc $e ,$ the flow sent through e is bounded by its capacity, and

2. flow conservation: at each node $v ,$ other than s and $t ,$ the total flow into v should equal the total flow out of v.

An s–t cut is defined by a partition of the nodes into two sets X and $\overline { { X } }$ so that $s \in X$ and $t \in { \overline { { X } } }$ , and consists of the set of arcs going from X to X. The capacity of this cut, $c ( X , { \overline { { X } } } )$ , is defined to be the sum of capacities of these arcs. Because of the capacity constraints on flow, the capacity of any s–t cut is an upper bound on any feasible flow. Thus, if the capacity of an s–t cut, say $( X , { \bar { X } } )$ , equals the value of a feasible flow, then $( X , { \bar { X } } )$ must be a minimum s–t cut and the flow must be a maximum flow in G. The max-flow min-cut theorem proves that it is always possible to find a flow and an $^ { s - t }$ cut so that equality holds.

Let us formulate the maximum flow problem as a linear program. First, introduce a fictitious arc of infinite capacity from t to $s ,$ thus converting the flow to a circulation; the objective now is to maximize the flow on this arc, denoted by $f _ { t s }$ . The advantage of making this modification is that we can now require flow conservation at s and t as well. If $f _ { i j }$ denotes the amount of flow sent through arc $( i , j ) \in E$ , we can formulate the maximum flow problem as follows:

$$
\begin{array}{l l} \text {maximize} & f _ {t s} \\ \text {subject to} & f _ {i j} \leq c _ {i j}, \\ & \sum_ {j: (j, i) \in E} f _ {j i} - \sum_ {j: (i, j) \in E} f _ {i j} \leq 0, \quad i \in V \\ & f _ {i j} \geq 0, \end{array}
$$

The second set of inequalities say that for each node i, the total flow into i is at most the total flow out of i. Notice that if this inequality holds at each node, then in fact it must be satisfied with equality at each node, thereby implying flow conservation at each node (this is so because a deficit in flow balance at one node implies a surplus at some other node). With this trick, we get a linear program in standard form.

To obtain the dual program we introduce variables $d _ { i j }$ and $p _ { i }$ corresponding to the two types of inequalities in the primal. We will view these variables as distance labels on arcs and potentials on nodes, respectively. The dual program is:

$$
\begin{array}{l l} \text { minimize } & \sum_ {(i, j) \in E} c _ {i j} d _ {i j} \\ \text { subject   to } & d _ {i j} - p _ {i} + p _ {j} \geq 0, \quad (i, j) \in E \\ & p _ {s} - p _ {t} \geq 1 \\ & d _ {i j} \geq 0, \quad (i, j) \in E \end{array}\tag{12.6}
$$

$$
p _ {i} \geq 0,
$$

$$
i \in V\tag{12.7}
$$

For developing an intuitive understanding of the dual program, it will be best to first transform it into an integer program that seeks $0 / 1$ solutions to the variables:

$$
\begin{array}{l l l} \text {minimize} & \sum_ {(i, j) \in E} c _ {i j} d _ {i j} \\ \text {subject to} & d _ {i j} - p _ {i} + p _ {j} \geq 0, & (i, j) \in E \\ & p _ {s} - p _ {t} \geq 1 \\ & d _ {i j} \in \{0, 1 \}, & (i, j) \in E \\ & p _ {i} \in \{0, 1 \}, & i \in V \end{array}
$$

Let $( d ^ { * } , p ^ { * } )$ be an optimal solution to this integer program. The only way to satisfy the inequality $p _ { s } ^ { * } - p _ { t } ^ { * } \ge 1$ with a $0 / 1$ substitution is to set $p _ { s } ^ { * } = 1$ and $p _ { t } ^ { * } = 0$ . This solution naturally defines an $^ { s - t }$ cut $( X , { \overline { { X } } } )$ , where $X$ is the set of potential 1 nodes, and $\overline { { X } }$ the set of potential 0 nodes. Consider an arc $( i , j )$ with $i \in X$ and $j \in { \overline { { X } } }$ . Since $p _ { i } ^ { * } = 1$ and $p _ { j } ^ { * } = 0$ , by the first constraint, $d _ { i j } ^ { * } \geq 1$ . But since we have a $0 / 1$ solution, $d _ { i j } ^ { * } = 1$ . The distance label for each of the remaining arcs can be set to either 0 or 1 without violating the first constraint; however, in order to minimize the objective function value, it must be set to 0. The objective function value must thus be equal to the capacity of the cut (X, X), and (X, X) must be a minimum s–t cut.

Thus, the previous integer program is a formulation of the minimum $^ { s - t }$ cut problem! What about the dual program? The dual program can be viewed as a relaxation of the integer program where the integrality constraint on the variables is dropped. This leads to the constraints $1 \geq d _ { i j } \geq 0$ for $( i , j ) \in E$ and $1 \geq p _ { i } \geq 0$ for $i \in V$ . Next, we notice that the upper bound constraints on the variables are redundant; their omission cannot give a better solution. Dropping these constraints gives the dual program in the form given above. We will say that this program is the LP-relaxation of the integer program.

Consider an $^ { s - t }$ cut C. Set C has the property that any path from s to t in G contains at least one edge of C. Using this observation, we can interpret any feasible solution to the dual program as a fractional s–t cut: the distance labels it assigns to arcs satisfy the property that on any path from s to t the distance labels add up to at least 1. To see this, consider an $^ { s - t }$ path $( s = v _ { 0 } , v _ { 1 } , \ldots , v _ { k } = t )$ . Now, the sum of the potential diferences on the endpoints of arcs on this path is

$$
\sum_ {i = 0} ^ {k - 1} (p _ {i} - p _ {i + 1}) = p _ {s} - p _ {t}.
$$

By the first constraint, the sum of the distance labels on the arcs must add up to at least $p _ { s } - p _ { t }$ , which is $\geq 1$ . Let us define the capacity of this fractional $^ { s - t }$ cut to be the dual objective function value achieved by it.

In principle, the best fractional s–t cut could have lower capacity than the best integral cut. Surprisingly enough, this does not happen. Consider the polyhedron defining the set of feasible solutions to the dual program. Let us call a feasible solution an extreme point solution if it is a vertex of this polyhedron, $\mathrm { i . e . , }$ it cannot be expressed as a convex combination of two feasible solutions. From linear programming theory we know that for any objective function, $\mathrm { i . e . , }$ assignment of capacities to the arcs of $G ,$ there is an extreme point solution that is optimal (for this discussion let us assume that for the given objective function, an optimal solution exists). Now, it can be proven that each extreme point solution of the polyhedron is integral, with each coordinate being 0 or 1 (see Exercise 12.6). Thus, the dual program always has an integral optimal solution.

By the LP-duality theorem maximum flow in $G$ must equal capacity of a minimum fractional s–t cut. But since the latter equals the capacity of a minimum s–t cut, we get the max-flow min-cut theorem.

The max-flow min-cut theorem is therefore a special case of the LP-duality theorem; it holds because the dual polyhedron has integral vertices. In fact, most min–max relations in combinatorial optimization hold for a similar reason.

Finally, let us illustrate the usefulness of complementary slackness conditions by utilizing them to derive additional properties of optimal solutions to the flow and cut programs. Let $f ^ { * }$ be an optimum solution to the primal LP (i.e., a maximum s–t flow). Also, let $( d ^ { * } , p ^ { * } )$ be an integral optimum solution to the dual LP, and let $( X , { \overline { { X } } } )$ be the cut defined by $( d ^ { * } , p ^ { * } )$ . Consider an arc $( i , j )$ such that $i \in X$ and $j \in { \overline { { X } } }$ . We have proven above that $d _ { i j } ^ { * } = 1$ Since $d _ { i j } ^ { * } \neq 0 .$ , by the dual complementary slackness condition, $f _ { i j } ^ { * } = c _ { i j }$ Next, consider an arc $( k , l )$ such that $k \in { \overline { { X } } }$ and $l \in X$ . Since $p _ { k } ^ { * } - p _ { l } ^ { * } = - 1$ and $d _ { k l } ^ { * } \in \{ 0 , 1 \}$ , the constraint $d _ { k l } ^ { * } - p _ { k } ^ { * } + p _ { l } ^ { * } \ge 0$ must be satisfied as a strict inequality. By the primal complementary slackness condition, $f _ { k l } ^ { * } = 0$ . Thus, we have proven that arcs going from $X$ to $\overline { { X } }$ are saturated by $f ^ { * }$ and the reverse arcs carry no flow. (Observe that it was not essential to invoke complementary slackness conditions to prove these facts; they also follow from the fact that flow across cut (X, X) equals its capacity.)

## 12.3 Two fundamental algorithm design techniques

We can now explain why linear programming is so useful in approximation algorithms. Many combinatorial optimization problems can be stated as integer programs. Once this is done, the linear relaxation of this program provides a natural way of lower bounding the cost of the optimal solution. As stated in Chapter 1, this is typically a key step in the design of an approximation algorithm. As in the case of the minimum s–t cut problem, a feasible solution to the LP-relaxation can be thought of as a fractional solution to the original problem. However, in the case of an NP-hard problem, we cannot expect the polyhedron defining the set of feasible solutions to have integer vertices. Thus, our task is not to look for an optimal solution to the LP-relaxation, but rather a near-optimal integral solution.

There are two basic techniques for obtaining approximation algorithms using linear programming. The first, and more obvious, method is to solve the linear program and then convert the fractional solution obtained into an integral solution, trying to ensure that in the process the cost does not increase much. The approximation guarantee is established by comparing the cost of the integral and fractional solutions. This technique is called LProunding or simply rounding.

The second, less obvious and perhaps more sophisticated, method is to use the dual of the LP-relaxation in the design of the algorithm. This technique is called the primal–dual schema. Let us call the LP-relaxation the primal program. Under this schema, an integral solution to the primal program and a feasible solution to the dual program are constructed iteratively. Notice that any feasible solution to the dual also provides a lower bound on OPT. The approximation guarantee is established by comparing the two solutions.

Both these techniques have been used extensively to obtain algorithms for many fundamental problems. Fortunately, once again, these techniques can be illustrated in the simple setting of the set cover problem. This is done in Chapters 14 and 15. Later chapters will present ever more sophisticated use of these techniques for solving a variety of problems.

LP-duality theory has also been useful in analyzing combinatorially obtained approximation algorithms, using the method of dual fitting. In Chapter 13 we will give an alternative analysis of the greedy set cover algorithm, Algorithm 2.2, using this method. This method has also been used to analyze greedy algorithms for the metric uncapacitated facility location problem (see Exercise 24.12). The method seems quite basic and should find other applications as well.

## 12.3.1 A comparison of the techniques and the notion of integrality gap

The reader may suspect that from the viewpoint of approximation guarantee, the primal–dual schema is inferior to rounding, since an optimal solution to the primal gives a tighter lower bound than a feasible solution to the dual. It turns out that this is not so. In order to give a formal explanation, we need to introduce the crucial notion of integrality gap of an LP-relaxation.

Given an LP-relaxation for a minimization problem Π, let $\mathrm { O P T } _ { f } ( I )$ denote the cost of an optimal fractional solution to instance I, i.e., the objective function value of an optimal solution to the LP-relaxation. Define the integrality gap, sometimes also called the integrality ratio, of the relaxation to be

$$
\sup _ {I} \frac {\mathrm{OPT} (I)}{\mathrm{OPT} _ {f} (I)},
$$

$\mathrm { i . e . , }$ the supremum of the ratio of the optimal integral and fractional solutions. In the case of a maximization problem, the integrality gap will be defined to be the infimum of this ratio. As stated in Section 12.2, most min–max relations arise from LP-relaxations that always have integral optimal solutions. Clearly, the integrality gap of such an LP is 1. We will call such an LP-relaxation an exact relaxation.

If the cost of the solution found by the algorithm is compared directly with the cost of an optimal fractional solution (or a feasible dual solution), as is done in most algorithms, the best approximation factor we can hope to prove is the integrality gap of the relaxation (see Exercise 12.4). Interestingly enough, for many problems, both techniques have been successful in yielding algorithms having guarantees essentially equal to the integrality gap of the relaxation.

The main diference in performance between the two techniques lies in the running times of the algorithms produced. An LP-rounding algorithm needs to find an optimal solution to the linear programming relaxation. Since linear programming is in P, this can be done in polynomial time if the relaxation has polynomially many constraints. Even if the relaxation has exponentially many constraints, this may still be achievable, if a polynomial time separation oracle can be constructed, i.e., a polynomial time algorithm that given a point in $\mathbf { R } ^ { n }$ , where n is the number of variables in the relaxation, either confirms that this point is a feasible solution (i.e., satisfies all constraints), or produces a violated constraint (see the notes in Section 12.5 for references). The running time for both possibilities is high; for the second it may be exorbitant. Let us remark that for certain problems, extreme point solutions have additional structural properties and some LP-rounding algorithms require such a solution to the linear programming relaxation. Such solutions can also be found in polynomial time.

On the other hand, the primal–dual schema leaves enough room to exploit the special combinatorial structure of individual problems and is thereby able to yield algorithms having good running times. It provides only a broad outline of the algorithm – the details have to be designed individually for specific problems. In fact, for many problems, once the algorithm has been designed using the primal–dual schema, the scafolding of linear programming can be completely dispensed with to get a purely combinatorial algorithm.

This brings us to another advantage of the primal–dual schema – this time not objectively quantifiable. A combinatorial algorithm is more malleable than an algorithm that requires an LP-solver. Once a basic problem is solved using the primal–dual schema, one can also solve variants and generalizations of the basic problem. Exercises in Chapters 22 and 24 illustrate this point. From a practical standpoint, a combinatorial algorithm is more useful, since it is easier to adapt it to specific applications and fine tune its performance for specific types of inputs.

## 12.4 Exercises

12.1 Show that the dual of the dual of a linear program is the original program itself.

12.2 Show that any minimization program can be transformed into an equivalent program in standard form, i.e., the form of LP (12.1).

12.3 Change some of the constraints of the primal program (12.1) into equalities, i.e., so they are of the form

$$
\sum_ {j = 1} ^ {n} a _ {i j} x _ {j} = b _ {i}, i \in I.
$$

Show that the dual of this program involves modifying program (12.2) so that the corresponding dual variables $y _ { i } , i \in I$ are unconstrained, i.e., they are not constrained to be nonnegative. Additionally, if some of the variables $x _ { j } , j \in J$ in program (12.1) are unconstrained, then the corresponding constraints in the dual become equalities.

12.4 Is the following a theorem: An approximation algorithm designed using an LP-relaxation cannot achieve a better approximation guarantee than the integrality gap of the relaxation.

Hint: In principle it may be possible to show, using additional structural properties, that whenever an instance has a bad gap, the cost of the solution found by the algorithm is much less that $\alpha \mathrm { O P T }$ , where α is the integrality gap of the relaxation. (Observe that if the instance has a bad gap, the cost of the solution found cannot be much less than $\alpha \mathrm { O P T } _ { f } . )$

## 12.5 Use the max-flow min-cut theorem to derive Menger’s Theorem:

Theorem 12.4 Let $G = ( V , E )$ be a directed graph with $s , t \in V$ . Then, the maximum number of edge-disjoint (vertex-disjoint) s–t paths is equal to the minimum number of edges (vertices) whose removal disconnects s from t.

12.6 Show that each extreme point solution for LP (12.6) is $0 / 1$ , and hence represents a valid cut.

Hint: An $n \times m$ matrix A is said to be totally unimodular if the determinant of every square submatrix of A is $1 , - 1$ , or 0. Show, by induction, that the constraint matrix of this LP is totally unimodular. $\mathrm { A l s o } ,$ use the fact that a feasible solution for a set of linear inequalities in $\mathbf { R } ^ { n }$ is an extreme point solution if it satisfies n linearly independent inequalities with equality.

12.7 This exercise develops a proof of the K¨onig-Egerv´ary Theorem (Theorem 1.6). Let $G = ( V , E )$ be a bipartite graph.

1. Show that the following is an exact LP-relaxation $( { \mathrm { i . e . } }$ , always has an integral optimal solution) for the maximum matching problem in G.

$$
\begin{array}{l l} \text {maximize} & \sum_ {e} x _ {e} \\ \text {subject to} & \sum_ {\substack {e: e \text {incident at} v \\ x _ {e} \geq 0,}} x _ {e} \leq 1, \quad v \in V \\ \end{array}\tag{12.8}
$$

Hint: Using the technique of Exercise 12.6 show that each extreme point solution for LP (12.8) is $0 / 1$ , and hence represents a valid matching.

2. Obtain the dual of this $\mathrm { L P }$ and show that it is an exact LP-relaxation for the problem of finding a minimum vertex cover in bipartite graph G.

3. Use the previous result to derive the K¨onig-Egerv´ary Theorem.

## 12.8 (Edmonds [68])

1. Let $G = ( V , E )$ be an undirected graph, with weights $w _ { e }$ on edges. The following is an exact LP-relaxation for the problem of finding a maximum weight matching in $G . \mathrm { \ : } ( \mathrm { B y } \textit { e } \colon \textit { e } \in S$ we mean edges e that have both endpoints in S.)

$$
\begin{array}{l l} \text {maximize} & \sum_ {e} w _ {e} x _ {e} \\ \text {subject to} & \sum_ {e: e \text {incident at} v} x _ {e} \leq 1, \quad v \in V \\ & \sum_ {e: e \in S} x _ {e} \leq \frac {| S | - 1}{2}, \quad S \subset V, | S | \text {odd} \\ & x _ {e} \geq 0, \quad e \in E \end{array}\tag{12.9}
$$

Obtain the dual of this LP. If the weight function is integral, the dual is also exact. Observe that Theorem 1.7 follows from these facts.

2. Assume that $| V |$ is even. The following is an exact LP-relaxation for the minimum weight perfect matching problem in G (a matching is perfect if it matches all vertices). Obtain the dual of this LP. Use complementary slackness conditions to give conditions satisfied by a pair of optimal primal (integral) and dual solutions for both formulations.

$$
\begin{array}{l l} \text {minimize} & \sum_ {e} w _ {e} x _ {e} \\ \text {subject to} & \sum_ {e: e \text {incident at} v} x _ {e} = 1, \quad v \in V \\ & \sum_ {e: e \in S} x _ {e} \leq \frac {| S | - 1}{2}, \qquad \qquad S \subset V, | S | \text {odd} \\ & x _ {e} \geq 0, \qquad \qquad e \in E \end{array}\tag{12.10}
$$

12.9 (Edmonds [71]) Show that the following is an integer programming formulation for the minimum spanning tree (MST) problem. Assume we are given graph $G = ( V , E ) , | V | = n .$ , with cost function $c : E \to \mathbf { Q } ^ { + }$ . For $A \subseteq E .$ , we will denote by $\kappa ( A )$ the number of connected components in graph $G _ { A } = ( V , A )$

$$
\begin{array}{l l} \text {minimize} & \sum_ {e} c _ {e} x _ {e} \\ \text {subject to} & \sum_ {e \in A} x _ {e} = n - \kappa (A), \quad A \subset E \\ & \sum_ {e \in E} x _ {e} = n - 1 \\ & x _ {e} \in \{0, 1 \}, \qquad e \in E \end{array}\tag{12.11}
$$

The rest of this exercise develops a proof that the LP-relaxation of this integer program is exact for the MST problem.

1. First, it will be convenient to change the objective function of IP (12.11) to max $\sum _ { e } - c _ { e } x _ { e }$ . Obtain the LP-relaxation and dual of this modified formulation.

2. Consider the primal solution produced by Kruskal’s algorithm. Let $e _ { 1 } , \ldots , e _ { 3 } $ $e _ { m }$ be the edges sorted by increasing cost, $| E | = m$ . This algorithm greedily picks a maximal acyclic subgraph from this sorted list. Obtain a suitable dual feasible solution so that all complementary slackness conditions are satisfied.

Hint: Let $A _ { t } = \{ e _ { 1 } , \ldots , e _ { t } \}$ . Set $y _ { A _ { t } } = e _ { t + 1 } - e _ { t }$ , for $1 \leq t < m$ , and $y _ { E } = - c _ { m }$ , where y is the dual variable.

3. Show that x is a feasible solution to the above-stated primal program if it is a feasible solution to the following LP. That is, prove that this is also an exact relaxation for the MST problem.

$$
\begin{array}{l l} \text {minimize} & \sum_ {e} c _ {e} x _ {e} \\ \text {subject to} & \sum_ {e \in S} x _ {e} = | S | - 1, \quad S \subset V \\ & \sum_ {e \in E} x _ {e} = n - 1 \\ & x _ {e} \geq 0, \qquad e \in E \end{array}\tag{12.12}
$$

12.10 In this exercise, you will derive von Neumann’s minimax theorem in game theory from the LP-duality theorem. A finite two-person zero-sum game is specified by an m × n matrix A with real entries. In each round, the row player, $\mathrm { R } .$ , selects a row, say $i ;$ simultaneously, the column player, C, selects a column, say $j .$ The payof to R at the end of this round is $a _ { i j }$ Thus, $| a _ { i j } |$ is the amount that C pays R (R pays C) if $a _ { i j }$ is positive $( a _ { i j }$ is negative); no money is exchanged if $a _ { i j }$ is zero. Zero-sum game refers to the fact that the total amount of money possessed by R and C together is conserved.

The strategy of each player is specified by a vector whose entries are nonnegative and add up to one, giving the probabilities with which the player picks each row or column. Let R’s strategy be given by m-dimensional vector x, and C’s strategy be given by n-dimensional vector y. Then, the expected payof to R in a round is ${ \pmb x } ^ { T } { \pmb A } { \pmb y }$ . The job of each player is to pick a strategy that guarantees maximum possible expected winnings (equivalently, minimum possible expected losses), regardless of the strategy chosen by the other player. If R chooses strategy x, he can be sure of winning only min<sub>y</sub> $\mathbf { x } ^ { T } A \mathbf { y }$ , where the minimum is taken over all possible strategies of C. Thus, the optimal choice for R is given by max<sub>x</sub> min<sub>y</sub> $\mathbf { \boldsymbol { x } } ^ { T } \mathbf { \boldsymbol { A } } \mathbf { \boldsymbol { y } } .$ Similarly, C will minimize her losses by choosing the strategy given by min<sub>y</sub> max<sub>x</sub> $\mathbf { x } ^ { T } A \mathbf { y }$ . The minimax theorem states that for every matrix A, max<sub>x</sub> min<sub>y</sub> $\ x ^ { T } A y \ =$ min<sub>y</sub> max<sub>x</sub> $\mathbf { x } ^ { T } A y$

Let us say that a strategy is pure if it picks a single row or column, $\mathrm { i . e . }$ , the vector corresponding to it consists of one 1 and the rest 0’s. A key observation is that for any strategy x of R, min<sub>y</sub> $\mathbf { x } ^ { T } A y$ is attained for a pure strategy of C: Suppose the minimum is attained for strategy $\mathbf { \pmb { y } } .$ . Consider the pure strategy corresponding to any nonzero component of $\mathbf { \pmb { y } } .$ The fact that the components of y are nonnegative and add up to one leads to an easy proof that this pure strategy attains the same minimum. Thus, R’s optimum strategy is given by max<sub>x</sub> min<sub>j</sub> $\textstyle \sum _ { i = 1 } ^ { m } a _ { i j } x _ { i }$ . The second critical observation is that the problem of computing R’s optimal strategy can be expressed as a linear program:

$$
\begin{array}{l l} \text {maximize} & z \\ \text {subject to} & z - \sum_ {i = 1} ^ {m} a _ {i j} x _ {i} \leq 0, \quad j = 1, \ldots , n \\ & \sum_ {i = 1} ^ {m} x _ {i} = 1 \\ & x _ {i} \geq 0, \quad i = 1, \ldots , m \end{array}
$$

Find the dual of this LP and show that it computes the optimal strategy for C. (Use the fact that for any strategy y of C, max<sub>x</sub> x<sup>T</sup> Ay is attained for a pure strategy of R.) Hence, prove the minimax theorem using the LP-duality theorem.

## 12.5 Notes

For a good introduction to theory of linear programming, see Chv´atal [49]. There are numerous other books on the topic, e.g., Dantzig [58], Karlof [160], Nemhauser and Wolsey [212], and Schrijver [237]. Linear programming has been extensively used in combinatorial optimization, see Cook, Cunningham, Pulleyblank, and Schrijver [52], Gr¨otschel, Lov´asz, and Schrijver [117], Lov´asz [194], Lov´asz and Plummer [195], and Papadimitriou and Steiglitz [217]. For a good explanation of Edmonds’ weighted matching algorithm, see Lov´asz and Plummer [195]. For algorithms for finding a solution to an LP, given a separation oracle, see Gr¨otschel, Lov´asz, and Schrijver [116, 117] and Schrijver [237].
