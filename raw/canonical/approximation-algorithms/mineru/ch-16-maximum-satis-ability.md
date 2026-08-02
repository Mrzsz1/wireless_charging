---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-16"
chapter_number: 16
chapter_title: "Maximum Satisﬁability"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 149
source_page_end: 157
printed_page_start: 131
printed_page_end: 139
part_ids: ["approximation-algorithms-ch-16-part-017"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Maximum Satisﬁability (MinerU semantic layer)

<!-- source-pages: 149-157; printed-pages: 131-139; mineru-part: approximation-algorithms-ch-16-part-017 -->

## 16 Maximum Satisfiability

The maximum satisfiability problem has been a classical problem in approximation algorithms. More recently, its study has led to crucial insights in the area of hardness of approximation (see Chapter 29). In this chapter, we will use LP-rounding, with randomization, to obtain a $3 / 4$ factor approximation algorithm. We will derandomize this algorithm using the method of conditional expectation.

Problem 16.1 (Maximum satisfiability (MAX-SAT)) Given a conjunctive normal form formula $f$ on Boolean variables $x _ { 1 } , \ldots , x _ { n }$ , and nonnegative weights, $w _ { c } .$ for each clause $c$ of $f ,$ , find a truth assignment to the Boolean variables that maximizes the total weight of satisfied clauses. Let C represent the set of clauses of $f , { \mathrm { i . e . , ~ } } f = \bigwedge _ { c \in { \mathcal { C } } } c .$ . Each clause is a disjunction \*of literals; each literal being either a Boolean variable or its negation. Let size(c) denote the size of clause $c , \ \mathrm { i . e . }$ , the number of literals in it. We will assume that the sizes of clauses in f are arbitrary.

For any positive integer $k ,$ we will denote by MAX-kSAT the restriction of MAX-SAT to instances in which each clause is of size at most k. MAX-SAT is NP-hard; in fact, even MAX-2SAT is NP-hard (in contrast, 2SAT is in P). We will first present two approximation algorithms for MAX-SAT, having guarantees of $1 / 2$ and $1 - 1 / e ,$ respectively. The first performs better if the clause sizes are large, and the seconds performs better if they are small. We will then show how an appropriate combination of the two algorithms achieves the promised approximation guarantee.

In the interest of minimizing notation, let us introduce common terminology for all three algorithms. Random variable W will denote the total weight of satisfied clauses. For each clause $^ { c , }$ random variable $W _ { c }$ denotes the weight contributed by clause c to W. Thus, $\begin{array} { r } { W = \sum _ { c \in \mathcal { C } } W _ { c } } \end{array}$ and

$$
\mathbf {E} [ W _ {c} ] = w _ {c} \cdot \mathbf {P r} [ c \text {   is   satisfied } ].
$$

(Strictly speaking, this is abuse of notation, since the randomization used by the three algorithms is diferent.)

## 16.1 Dealing with large clauses

The first algorithm is straightforward. Set each Boolean variable to be True independently with probability $1 / 2$ and output the resulting truth assignment, say τ. For $k \geq 1$ , define $\alpha _ { k } = 1 - 2 ^ { - k }$

Lemma 16.2 If size $\mathbf { \nabla } \cdot ( c ) = k _ { ; }$ , then ${ \bf E } [ W _ { c } ] = \alpha _ { k } w _ { c }$

Proof: Clause c is not satisfied by τ if all its literals are set to False. The probability of this event is $2 ^ { - k }$ ✷

For $k \geq 1 , \alpha _ { k } \geq 1 / 2$ . By linearity of expectation,

$$
\mathbf {E} [ W ] = \sum_ {c \in \mathcal {C}} \mathbf {E} [ W _ {c} ] \geq \frac {1}{2} \sum_ {c \in \mathcal {C}} w _ {c} \geq \frac {1}{2} \mathrm{OPT},
$$

where we have used a trivial upper bound on $\mathrm { O P T \mathrm { ~ - ~ } }$ the total weight of clauses in $\mathcal { C } .$

Instead of converting this into a high probability statement, with a corresponding loss in guarantee, we show how to derandomize this procedure. The resulting algorithm deterministically computes a truth assignment such that the weight of satisfied clauses is $\ge \mathbf { E } [ W ] \ge \mathrm { O P T / 2 }$

Observe that $\alpha _ { k }$ increases with k and the guarantee of this algorithm is $3 / 4$ if each clause has two or more literals. (The next algorithm is designed to deal with unit clauses more efectively.)

## 16.2 Derandomizing via the method of conditional expectation

We will critically use the self-reducibility of SAT (see Section A.5). Consider the self-reducibility tree $T$ for formula $f .$ Each internal node at level i corresponds to a setting for Boolean variables $x _ { 1 } , \ldots , x _ { i }$ , and each leaf represents a complete truth assignment to the n variables. Let us label each node of $T$ with its conditional expectation as follows. Let $a _ { 1 } , \ldots , a _ { i }$ be $\mathrm { a }$ truth assignment to $x _ { 1 } , \ldots , x _ { i }$ . The node corresponding to this assignment will be labeled with $\mathbf { E } [ W | x _ { 1 } = a _ { 1 } , \dots , x _ { i } = a _ { i } ]$ . If $i = n ,$ , this is a leaf node and its conditional expectation is simply the total weight of clauses satisfied by its truth assignment.

Lemma 16.3 The conditional expectation of any node in T can be computed in polynomial time.

Proof: Consider a node $x _ { 1 } = a _ { 1 } , . . . , x _ { i } = a _ { i }$ . Let $\phi$ be the Boolean formula, on variables $x _ { i + 1 } , \ldots , x _ { n } ,$ obtained for this node via self-reducibility. Clearly, the expected weight of satisfied clauses of $\phi$ under a random truth assignment to the variables $x _ { i + 1 } , \ldots , x _ { n }$ can be computed in polynomial time. Adding to this the total weight of clauses of $f$ already satisfied by the partial assignment $x _ { 1 } = a _ { 1 } , . . . , x _ { i } = a _ { i }$ gives the answer. ✷

Theorem 16.4 We can compute, in polynomial time, a path from the root to a leaf such that the conditional expectation of each node on this path is $\geq \mathbf { E } [ W ]$

Proof: The conditional expectation of a node is the average of the conditional expectations of its two children, i.e.,

$$
\begin{array}{c} \mathbf {E} [ W | x _ {1} = a _ {1},..., x _ {i} = a _ {i} ] = \mathbf {E} [ W | x _ {1} = a _ {1},..., x _ {i} = a _ {i}, x _ {i + 1} = \text {True} ] / 2 + \\ \mathbf {E} [ W | x _ {1} = a _ {1},..., x _ {i} = a _ {i}, x _ {i + 1} = \text {False} ] / 2. \end{array}
$$

The reason, of course, is that $x _ { i + 1 }$ is equally likely to be set to True or False. As a result, the child with the larger value has a conditional expectation at least as large as that of the parent. This establishes the existence of the desired path. As a consequence of Lemma 16.3, it can be computed in polynomial time. ✷

The deterministic algorithm follows as a corollary of Theorem 16.4. We simply output the truth assignment on the leaf node of the path computed. The total weight of clauses satisfied by it $\mathrm { i s } \geq \mathbf { E } [ W ]$

Let us show that the technique outlined above can, in principle, be used to derandomize more complex randomized algorithms. Suppose the algorithm does not set the Boolean variables independently of each other (for instance, see Remark 16.6). Now,

$$
\begin{array}{c} \mathbf {E} [ W | x _ {1} = a _ {1}, \dots , x _ {i} = a _ {i} ] = \\ \mathbf {E} [ W | x _ {1} = a _ {1}, \dots , x _ {i} = a _ {i}, x _ {i + 1} = \text {True} ] \cdot \mathbf {P r} [ x _ {i + 1} = \text {True} | x _ {1} = a _ {1}, \dots , x _ {i} = a _ {i} ] + \\ \mathbf {E} [ W | x _ {1} = a _ {1}, \dots , x _ {i} = a _ {i}, x _ {i + 1} = \text {False} ] \cdot \mathbf {P r} [ x _ {i + 1} = \text {False} | x _ {1} = a _ {1}, \dots , x _ {i} = a _ {i} ]. \end{array}
$$

The sum of the two conditional probabilities is again 1, since the two events are exhaustive. So, the conditional expectation of the parent is still a convex combination of the conditional expectations of the two children. If we can determine, in polynomial time, which of the two children has a larger value, we can again derandomize the algorithm. However, computing the conditional expectations may not be easy. Observe how critically independence was used in the proof of Lemma 16.3. It was because of independence that we could assume a random truth assignment on Boolean variables $x _ { i + 1 } , \ldots , x _ { n }$ and thereby compute the expected weight of satisfied clauses of $\phi .$

In general, a randomized algorithm may pick from a larger set of choices and not necessarily with equal probability. But once again a convex combination of the conditional expectations of these choices, given by the probabilities of picking them, equals the conditional expectation of the parent. Hence there must be a choice that has at least as large a conditional expectation as the parent.

## 16.3 Dealing with small clauses via LP-rounding

Following is an integer program for MAX-SAT. For each clause $c \in { \mathcal { C } } .$ , let $S _ { c } ^ { + }$ $( S _ { c } ^ { - } )$ denote the set of Boolean variables occurring nonnegated (negated) in $c .$ The truth assignment is encoded by $\mathbf { \nabla } _ { \mathbf { \mu } _ { y . } }$ Picking $y _ { i } = 1 \ ( y _ { i } = 0 )$ denotes setting $x _ { i }$ to True (False). The constraint for clause c ensures that $z _ { c }$ can be set to 1 only if at least one of the literals occurring in c is set to True, i.e., if clause c is satisfied by the picked truth assignment.

$$
\begin{array}{l l} \text {maximize} & \sum_ {c \in \mathcal {C}} w _ {c} z _ {c} \\ \text {subject to} & \forall c \in \mathcal {C}: \sum_ {i \in S _ {c} ^ {+}} y _ {i} + \sum_ {i \in S _ {c} ^ {-}} (1 - y _ {i}) \geq z _ {c} \\ & \forall c \in \mathcal {C}: z _ {c} \in \{0, 1 \} \\ & \forall i: y _ {i} \in \{0, 1 \} \end{array}\tag{16.1}
$$

The LP-relaxation is:

$$
\begin{array}{l l} \text {maximize} & \sum_ {c \in \mathcal {C}} w _ {c} z _ {c} \\ \text {subject to} & \forall c \in \mathcal {C}: \sum_ {i \in S _ {c} ^ {+}} y _ {i} + \sum_ {i \in S _ {c} ^ {-}} (1 - y _ {i}) \geq z _ {c} \\ & \forall c \in \mathcal {C}: 1 \geq z _ {c} \geq 0 \\ & \forall i: 1 \geq y _ {i} \geq 0 \end{array}\tag{16.2}
$$

The algorithm is again straightforward. Solve LP (16.2). Let $( y ^ { * } , z ^ { * } )$ denote the optimal solution. Independently set $x _ { i }$ to True with probability $y _ { i } ^ { * }$ for $1 \leq i \leq n$ . Output the resulting truth assignment, say $\tau .$

We will use the random variables $W$ and $W _ { c }$ defined in Section 16.1. For $k \geq 1$ , define

$$
\beta_ {k} = 1 - \left(1 - \frac {1}{k}\right) ^ {k}.
$$

$$
\mathbf {E} [ W _ {c} ] \geq \beta_ {k} w _ {c} z _ {c} ^ {*}.
$$

Lemma 16.5 If $\mathrm { s i z e } ( c ) = k$ , then

Proof: We may assume w.l.o.g. that all literals in c appear nonnegated $( { \mathrm { i f ~ } } x _ { i }$ appears negated, we can replace $x _ { i }$ with $\overline { { x } } _ { i }$ throughout $f$ and modify LP (16.2) accordingly without afecting $z _ { c } ^ { * }$ or $W _ { c } )$ . Further, by renaming variables, we may assume $c = ( x _ { 1 } \vee . . . \vee x _ { k } )$

Clause c is satisfied if $x _ { 1 } , \ldots , x _ { k }$ are not all set to False. The probability of this event is

$$
\begin{array}{c} 1 - \prod_ {i = 1} ^ {k} (1 - y _ {i}) \geq 1 - \left(\frac {\sum_ {i = 1} ^ {k} (1 - y _ {i})}{k}\right) ^ {k} = 1 - \left(1 - \frac {\sum_ {i = 1} ^ {k} y _ {i}}{k}\right) ^ {k} \\ \geq 1 - \left(1 - \frac {z _ {c} ^ {*}}{k}\right) ^ {k}, \end{array}
$$

where the first inequality follows from the arithmetic-geometric mean inequality which states that for nonnegative numbers $a _ { 1 } , \ldots , a _ { k }$ 2

$$
\frac {a _ {1} + \ldots + a _ {k}}{k} \geq \sqrt [ k ]{a _ {1} \times \ldots \times a _ {k}}.
$$

The second inequality uses the constraint in LP (16.2) that $y _ { 1 } + . . . + y _ { k } \ge z _ { c }$

![](images/dbd482918c1e9c9a33449faffe5a4cc9ac994aabff12ad3d0c42d130b936dff3.jpg)

Define function $g$ by:

$$
g (z) = 1 - \left(1 - \frac {z}{k}\right) ^ {k}.
$$

This is a concave function with $g ( 0 ) = 0$ and $g ( 1 ) \ = \ \beta _ { k }$ . Therefore, for $z \in [ 0 , 1 ] , g ( z ) \geq \beta _ { k } z$ . Hence, $\mathbf { P r } [ c$ is satisfied] $\ge \beta _ { k } z _ { c } ^ { * }$ . The lemma follows. ✷

Notice that $\beta _ { k }$ is a decreasing function of k. Thus, if all clauses are of size at most $k$ ,

$$
\mathbf {E} [ W ] = \sum_ {c \in \mathcal {C}} \mathbf {E} [ W _ {c} ] \geq \beta_ {k} \sum_ {c \in \mathcal {C}} w _ {c} z _ {c} ^ {*} = \beta_ {k} \mathrm{OPT} _ {f} \geq \beta_ {k} \mathrm{OPT},
$$

where $\mathrm { O P T } _ { f }$ is the optimal solution to LP (16.2). Clearly, $\mathrm { O P T } _ { f } \geq \mathrm { O P T }$ This algorithm can also be derandomized using the method of conditional expectation (Exercise 16.3). Hence, for MAX-SAT instances with clause sizes at most $k ,$ it is a $\beta _ { k }$ factor approximation algorithm. Since

$$
\forall k \in \mathbf {Z} ^ {+}: \left(1 - \frac {1}{k}\right) ^ {k} > \frac {1}{e},
$$

this is a $1 - 1 / e$ factor algorithm for MAX-SAT.

## 16.4 A 3/4 factor algorithm

We will combine the two algorithms as follows. Let b be the flip of a fair coin. If $b = 0$ , run the first randomized algorithm, and if $b = 1$ , run the second randomized algorithm.

Remark 16.6 Notice that we are efectively setting $x _ { i }$ to True with probability $\begin{array} { r } { \frac { 1 } { 4 } + \frac { 1 } { 2 } y _ { i } ^ { * } ; } \end{array}$ ; however, the $x _ { i }$ ’s are not set independently!

Let $z ^ { * }$ be the optimal solution of LP (16.2) on the given instance.

Lemma 16.7

$$
\mathbf {E} [ W _ {c} ] \geq \frac {3}{4} w _ {c} z _ {c} ^ {*}.
$$

Proof: Let s $\mathrm { z e } ( c ) = k$ . By Lemma 16.2,

$$
\mathbf {E} \left[ W _ {c} \mid b = 0 \right] = \alpha_ {k} w _ {c} \geq \alpha_ {k} w _ {c} z _ {c} ^ {*},
$$

where we have used the fact that $z _ { c } ^ { * } \leq 1$ . By Lemma 16.5,

$$
\mathbf {E} [ W _ {c} \mid b = 1 ] \geq \beta_ {k} w _ {c} z _ {c} ^ {*}.
$$

Combining we get

$$
\mathbf {E} [ W _ {c} ] = \frac {1}{2} (\mathbf {E} [ W _ {c} \mid b = 0 ] + \mathbf {E} [ W _ {c} \mid b = 1 ]) \geq w _ {c} z _ {c} ^ {*} \frac {(\alpha_ {k} + \beta_ {k})}{2}.
$$

Now, $\alpha _ { 1 } + \beta _ { 1 } = \alpha _ { 2 } + \beta _ { 2 } = 3 / 2$ , and for $k \ge 3 , \alpha _ { k } + \beta _ { k } \ge 7 / 8 + ( 1 - 1 / e ) \ge 3 / 2$ The lemma follows. ✷

By linearity of expectation,

$$
\mathbf {E} [ W ] = \sum_ {c \in \mathcal {C}} \mathbf {E} [ W _ {c} ] \geq \frac {3}{4} \sum_ {c \in \mathcal {C}} w _ {c} z _ {c} ^ {*} = \frac {3}{4} \mathrm{OPT} _ {f} \geq \frac {3}{4} \mathrm{OPT},\tag{16.3}
$$

where $\mathrm { O P T } _ { f }$ is the optimal solution to LP (16.2). Finally, consider the following deterministic algorithm.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 16.8 (MAX-SAT - factor $3/4$)
1. Use the derandomized factor $1/2$ algorithm to get a truth assignment, $\tau_1$.
2. Use the derandomized factor $1 - 1/e$ algorithm to get a truth assignment, $\tau_2$.
3. Output the better of the two assignments.
</div>

Theorem 16.9 Algorithm 16.8 is a deterministic factor $3 / 4$ approximation algorithm for MAX-SAT.

Proof: One of the two conditional expectations, $\mathbf { E } [ W \mid b = 0 ]$ and $\mathbf { E } [ W \mid b =$ 1], is at least as large as E[W]. Hence, the total weight of clauses satisfied by the better of $\tau _ { 1 }$ and $\tau _ { 2 }$ is at least as large as $\mathbf E [ W ]$ ✷

By (16.3), $\mathbf { E } [ W ] \geq { \frac { 3 } { 4 } } \mathrm { O P T } _ { f }$ . The weight of the integral solution produced by Algorithm 16.8 is at least $\mathbf E [ W ]$ . Therefore, the integrality gap of LP (16.2) is $\geq 3 / 4$ . Below we show that this is tight.

Example 16.10 Consider the SAT formula $f = ( x _ { 1 } \lor x _ { 2 } ) \land ( { \overline { { x } } } _ { 1 } \lor x _ { 2 } ) \land ( x _ { 1 } \lor$ $\overline { { x } } _ { 2 } ) \wedge ( \overline { { x } } _ { 1 } \vee \overline { { x } } _ { 2 } )$ , where each clause is of unit weight. It is easy to see that setting $y _ { i } = 1 / 2$ and $z _ { c } = 1$ for all i and c is an optimal solution to LP (16.2) for any instance having size 2 clauses. Therefore $\mathrm { O P T } _ { f } = 4$ . On the other hand $\mathrm { O P T } = 3$ , and thus for this instance LP (16.2) has a integrality gap of $4 / 3$ ✷

Example 16.11 Let us provide a tight example to Algorithm 16.8. Let $f = ( x \vee y ) \wedge ( x \vee \overline { { y } } ) \wedge ( \overline { { x } } \vee z )$ , and let the weights of these three clauses be 1, 1, and $2 + \varepsilon .$ , respectively. By the remark made in Example 16.10, on this instance the factor $1 - 1 / e$ algorithm will set each variable to True with probability $1 / 2$ and so will be the same as the factor $1 / 2$ algorithm. During derandomization, suppose variable x is set first. The conditional expectations are $\mathbf { E } [ W \mid x = \mathrm { T r u e } ] = 3 + \varepsilon / 2$ and $\mathbf { E } [ W \mid x = { \mathrm { F a l s e } } ] = 3 + \varepsilon$ . Thus, x will be set to False. But this leads to a total weight of $3 + \varepsilon ,$ whereas by setting x to True we can get a weight of $4 + \varepsilon$ . Clearly, we can get an infinite family of such examples by replicating these 3 clauses with new variables. ✷

## 16.5 Exercises

16.1 The algorithm of Section 16.1 achieves an approximation guarantee of $\alpha _ { k }$ if all clauses in the given instance have size at least k. Give a tight example of factor $\alpha _ { k }$ for this algorithm.

16.2 Show that the following is a factor $1 / 2$ algorithm for MAX-SAT. Let τ be an arbitrary truth assignment and $\tau ^ { \prime }$ be its complement, i.e., a variable is True in $\tau$ if it is False in $\tau ^ { \prime }$ . Compute the weight of clauses satisfied by $\tau$ and $\tau ^ { \prime }$ , then output the better assignment.

16.3 Use the method of conditional expectation to derandomize the $1 - 1 / e$ factor algorithm for MAX-SAT.

16.4 Observe that the randomization used in the $3 / 4$ factor algorithm does not set Boolean variables independently of each other. As remarked in Section 16.2, the algorithm can still, in principle, be derandomized using the method of conditional expectation. Devise a way of doing so. Observe that the algorithm obtained is diferent from Algorithm 16.8.

16.5 (Goemans and Williamson [104]) Instead of using the solution to LP (16.2), $y _ { i } ^ { * }$ , as probability of setting $x _ { i }$ to True, consider the more general scheme of using $g ( y _ { i } ^ { * } )$ , for a suitable function g. Can this lead to an improvement over the factor $1 - 1 / e$ algorithm?

16.6 Consider the following randomized algorithm for the maximum cut problem, defined in Exercise 2.1. After the initialization step of Algorithm 2.13, each of the remaining vertices is equally likely to $_ { \mathrm { g o } }$ in sets A or B. Show that the expected size of the cut found is at least $\mathrm { O P T } / 2$ . Show that the derandomization of this algorithm via the method of conditional expectation is precisely Algorithm 2.13.

16.7 Consider the following generalization of the maximum cut problem.

Problem 16.12 (Linear equations over $\mathbf { G F } [ \mathbf { 2 } ] )$ Given m equations over n GF[2] variables, find an assignment for the variables that maximizes the number of satisfied equations.

1. Show that if $m \leq n .$ , this problem is polynomial time solvable.

2. In general, the problem is NP-hard. Give a factor $1 / 2$ randomized algorithm for it, and derandomize using the method of conditional expectation.

16.8 Consider the obvious randomized algorithm for the MAX k-CUT problem, Problem 2.14 in Exercise 2.3, which assigns each vertex randomly to one of the sets $S _ { 1 } , \ldots , S _ { k }$ . Show that the expected number of edges running between these sets is at least $\mathrm { O P T / 2 }$ . Show that the derandomization of this algorithm, via the method of conditional expectation, gives the greedy algorithm sought in Exercise 2.3.

16.9 Repeat Exercise 16.8 for the maximum directed cut problem, Problem 2.15 in Exercise 2.4, i.e., give a factor $1 / 4$ randomized algorithm, and show that its derandomization gives a greedy algorithm.

## 16.6 Notes

The factor 1/2 algorithm, which was also the first approximation algorithm for MAX-SAT, is due to Johnson [150]. The first factor 3/4 algorithm was due to Yannakakis [261]. The (simpler) algorithm given here is due to Goemans and Williamson [104]. The method of conditional expectation is implicit in Erd¨os and Selfridge [74]. Its use for obtaining polynomial time algorithms was pointed out by Spencer [243] (see Raghavan [225] and Alon and Spencer [6] for enhancements to this technique).
