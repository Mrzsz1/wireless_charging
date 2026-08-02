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

# Maximum Satisﬁability

16 Maximum Satisﬁability




The maximum satisﬁability problem has been a classical problem in approx-
imation algorithms. More recently, its study has led to crucial insights in
the area of hardness of approximation (see Chapter 29). In this chapter, we
will use LP-rounding, with randomization, to obtain a 3/4 factor approxi-
mation algorithm. We will derandomize this algorithm using the method of
conditional expectation.
Problem 16.1 (Maximum satisﬁability (MAX-SAT)) Given a con-
junctive normal form formula f on Boolean variables x1 , . . . , xn , and non-
negative weights, wc , for each clause c of f , ﬁnd a truth assignment to the
Boolean variables that maximizes the total   * weight of satisﬁed clauses. Let C
represent the set of clauses of f , i.e., f = c∈C c. Each clause is a disjunction
of literals; each literal being either a Boolean variable or its negation. Let
size(c) denote the size of clause c, i.e., the number of literals in it. We will
assume that the sizes of clauses in f are arbitrary.
    For any positive integer k, we will denote by MAX-kSAT the restriction
of MAX-SAT to instances in which each clause is of size at most k. MAX-
SAT is NP-hard; in fact, even MAX-2SAT is NP-hard (in contrast, 2SAT
is in P). We will ﬁrst present two approximation algorithms for MAX-SAT,
having guarantees of 1/2 and 1−1/e, respectively. The ﬁrst performs better if
the clause sizes are large, and the seconds performs better if they are small.
We will then show how an appropriate combination of the two algorithms
achieves the promised approximation guarantee.
    In the interest of minimizing notation, let us introduce common terminol-
ogy for all three algorithms. Random variable W will denote the total weight
of satisﬁed clauses. For each clause c, random variable Wc denotes the weight
contributed by clause c to W . Thus, W = c∈C Wc and

      E[Wc ] = wc · Pr[c is satisﬁed].

(Strictly speaking, this is abuse of notation, since the randomization used by
the three algorithms is diﬀerent.)
132      16   Maximum Satisﬁability

16.1 Dealing with large clauses
The ﬁrst algorithm is straightforward. Set each Boolean variable to be True
independently with probability 1/2 and output the resulting truth assign-
ment, say τ . For k ≥ 1, deﬁne αk = 1 − 2−k .

Lemma 16.2 If size(c) = k, then E[Wc ] = αk wc .

Proof: Clause c is not satisﬁed by τ iﬀ all its literals are set to False. The
probability of this event is 2−k .                                          ✷
      For k ≥ 1, αk ≥ 1/2. By linearity of expectation,
                                  1        1
        E[W ] =         E[Wc ] ≥         wc ≥ OPT,
                                   2         2
                  c∈C                c∈C


where we have used a trivial upper bound on OPT – the total weight of
clauses in C.
    Instead of converting this into a high probability statement, with a cor-
responding loss in guarantee, we show how to derandomize this procedure.
The resulting algorithm deterministically computes a truth assignment such
that the weight of satisﬁed clauses is ≥ E[W ] ≥ OPT/2.
    Observe that αk increases with k and the guarantee of this algorithm is
3/4 if each clause has two or more literals. (The next algorithm is designed
to deal with unit clauses more eﬀectively.)


16.2 Derandomizing via the method of conditional
expectation
We will critically use the self-reducibility of SAT (see Section A.5). Consider
the self-reducibility tree T for formula f . Each internal node at level i corre-
sponds to a setting for Boolean variables x1 , . . . , xi , and each leaf represents
a complete truth assignment to the n variables. Let us label each node of
T with its conditional expectation as follows. Let a1 , . . . , ai be a truth as-
signment to x1 , . . . , xi . The node corresponding to this assignment will be
labeled with E[W |x1 = a1 , . . . , xi = ai ]. If i = n, this is a leaf node and its
conditional expectation is simply the total weight of clauses satisﬁed by its
truth assignment.

Lemma 16.3 The conditional expectation of any node in T can be computed
in polynomial time.

Proof: Consider a node x1 = a1 , . . . , xi = ai . Let φ be the Boolean formula,
on variables xi+1 , . . . , xn , obtained for this node via self-reducibility. Clearly,
          16.2   Derandomizing via the method of conditional expectation          133

the expected weight of satisﬁed clauses of φ under a random truth assignment
to the variables xi+1 , . . . , xn can be computed in polynomial time. Adding to
this the total weight of clauses of f already satisﬁed by the partial assignment
x1 = a1 , . . . , xi = ai gives the answer.                                   ✷

Theorem 16.4 We can compute, in polynomial time, a path from the root
to a leaf such that the conditional expectation of each node on this path is
≥ E[W ].

Proof: The conditional expectation of a node is the average of the condi-
tional expectations of its two children, i.e.,

      E[W |x1 = a1 , ..., xi = ai ] = E[W |x1 = a1 , ..., xi = ai , xi+1 = True]/2 +
                                      E[W |x1 = a1 , ..., xi = ai , xi+1 = False]/2.

The reason, of course, is that xi+1 is equally likely to be set to True or False.
As a result, the child with the larger value has a conditional expectation at
least as large as that of the parent. This establishes the existence of the desired
path. As a consequence of Lemma 16.3, it can be computed in polynomial
time.                                                                             ✷
    The deterministic algorithm follows as a corollary of Theorem 16.4. We
simply output the truth assignment on the leaf node of the path computed.
The total weight of clauses satisﬁed by it is ≥ E[W ].
    Let us show that the technique outlined above can, in principle, be used to
derandomize more complex randomized algorithms. Suppose the algorithm
does not set the Boolean variables independently of each other (for instance,
see Remark 16.6). Now,

    E[W |x1 = a1 , ..., xi = ai ] =
E[W |x1 = a1 , ..., xi = ai , xi+1 = True] · Pr[xi+1 = True|x1 = a1 , ..., xi = ai ]+
E[W |x1 = a1 , ..., xi = ai , xi+1 = False] · Pr[xi+1 = False|x1 = a1 , ..., xi = ai ].

    The sum of the two conditional probabilities is again 1, since the two
events are exhaustive. So, the conditional expectation of the parent is still
a convex combination of the conditional expectations of the two children. If
we can determine, in polynomial time, which of the two children has a larger
value, we can again derandomize the algorithm. However, computing the con-
ditional expectations may not be easy. Observe how critically independence
was used in the proof of Lemma 16.3. It was because of independence that we
could assume a random truth assignment on Boolean variables xi+1 , . . . , xn
and thereby compute the expected weight of satisﬁed clauses of φ.
    In general, a randomized algorithm may pick from a larger set of choices
and not necessarily with equal probability. But once again a convex combina-
tion of the conditional expectations of these choices, given by the probabilities
134      16    Maximum Satisﬁability

of picking them, equals the conditional expectation of the parent. Hence there
must be a choice that has at least as large a conditional expectation as the
parent.


16.3 Dealing with small clauses via LP-rounding
Following is an integer program for MAX-SAT. For each clause c ∈ C, let Sc+
(Sc− ) denote the set of Boolean variables occurring nonnegated (negated) in
c. The truth assignment is encoded by y. Picking yi = 1 (yi = 0) denotes
setting xi to True (False). The constraint for clause c ensures that zc can be
set to 1 only if at least one of the literals occurring in c is set to True, i.e., if
clause c is satisﬁed by the picked truth assignment.
                       
        maximize             wc zc                                            (16.1)
                       c∈C
                                                   
        subject to     ∀c ∈ C :              yi +           (1 − yi ) ≥ zc
                                     i∈Sc+          i∈Sc−
                       ∀c ∈ C : zc ∈ {0, 1}
                       ∀i : yi ∈ {0, 1}

      The LP-relaxation is:
                       
        maximize             wc zc                                            (16.2)
                       c∈C
                                                   
        subject to     ∀c ∈ C :              yi +           (1 − yi ) ≥ zc
                                     i∈Sc+          i∈Sc−
                       ∀c ∈ C : 1 ≥ zc ≥ 0
                       ∀i : 1 ≥ yi ≥ 0

    The algorithm is again straightforward. Solve LP (16.2). Let (y ∗ , z ∗ ) de-
note the optimal solution. Independently set xi to True with probability yi∗ ,
for 1 ≤ i ≤ n. Output the resulting truth assignment, say τ .
    We will use the random variables W and Wc deﬁned in Section 16.1. For
k ≥ 1, deﬁne
                      k
                     1
        βk = 1 − 1 −      .
                     k

Lemma 16.5 If size(c) = k, then

        E[Wc ] ≥ βk wc zc∗ .
                            16.3   Dealing with small clauses via LP-rounding          135

Proof: We may assume w.l.o.g. that all literals in c appear nonnegated
(if xi appears negated, we can replace xi with xi throughout f and modify
LP (16.2) accordingly without aﬀecting zc∗ or Wc ). Further, by renaming
variables, we may assume c = (x1 ∨ . . . ∨ xk ).
    Clause c is satisﬁed if x1 , . . . , xk are not all set to False. The probability
of this event is

           k
                                                      k             k        k
           +                            k
                                        i=1 (1 − yi )                    i=1 yi
      1−         (1 − yi ) ≥ 1 −                             =1−   1−
           i=1
                                              k                           k
                                             
                                             ∗ k
                                         zc
                          ≥1− 1−                   ,
                                         k

where the ﬁrst inequality follows from the arithmetic-geometric mean in-
equality which states that for nonnegative numbers a1 , . . . , ak ,
      a1 + . . . + ak   √
                      ≥ k a1 × . . . × ak .
            k
The second inequality uses the constraint in LP (16.2) that y1 + . . . + yk ≥ zc .

      g(z)




                  0                                          1                    z

   Deﬁne function g by:
                    z k
      g(z) = 1 − 1 −      .
                     k
This is a concave function with g(0) = 0 and g(1) = βk . Therefore, for
z ∈ [0, 1], g(z) ≥ βk z. Hence, Pr[c is satisﬁed] ≥ βk zc∗ . The lemma follows. ✷
   Notice that βk is a decreasing function of k. Thus, if all clauses are of size
at most k,
                                      
      E[W ] =           E[Wc ] ≥ βk          wc zc∗ = βk OPTf ≥ βk OPT,
                  c∈C                  c∈C
136      16   Maximum Satisﬁability

where OPTf is the optimal solution to LP (16.2). Clearly, OPTf ≥ OPT.
This algorithm can also be derandomized using the method of conditional
expectation (Exercise 16.3). Hence, for MAX-SAT instances with clause sizes
at most k, it is a βk factor approximation algorithm. Since
                               k
               +            1            1
        ∀k ∈ Z :         1−          >     ,
                            k            e

this is a 1 − 1/e factor algorithm for MAX-SAT.


16.4 A 3/4 factor algorithm
We will combine the two algorithms as follows. Let b be the ﬂip of a fair coin.
If b = 0, run the ﬁrst randomized algorithm, and if b = 1, run the second
randomized algorithm.
Remark 16.6 Notice that we are eﬀectively setting xi to True with proba-
bility 14 + 12 yi∗ ; however, the xi ’s are not set independently!
    Let z ∗ be the optimal solution of LP (16.2) on the given instance.
                                         3
Lemma 16.7                 E[Wc ] ≥        wc zc∗ .
                                         4

Proof: Let size(c) = k. By Lemma 16.2,

        E[Wc | b = 0] = αk wc ≥ αk wc zc∗ ,

where we have used the fact that zc∗ ≤ 1. By Lemma 16.5,

        E[Wc | b = 1] ≥ βk wc zc∗ .

Combining we get

                   1                                          (αk + βk )
        E[Wc ] =     (E[Wc | b = 0] + E[Wc | b = 1]) ≥ wc zc∗            .
                   2                                              2
Now, α1 +β1 = α2 +β2 = 3/2, and for k ≥ 3, αk +βk ≥ 7/8+(1−1/e) ≥ 3/2.
The lemma follows.                                                  ✷
      By linearity of expectation,
                                    3            3      3
        E[W ] =          E[Wc ] ≥          wc zc∗ = OPTf ≥ OPT,              (16.3)
                                     4             4      4
                   c∈C                 c∈C


where OPTf is the optimal solution to LP (16.2). Finally, consider the fol-
lowing deterministic algorithm.
                                                         16.5   Exercises    137


 Algorithm 16.8 (MAX-SAT – factor 3/4)
  1. Use the derandomized factor 1/2 algorithm to get a truth assignment,
     τ1 .
  2. Use the derandomized factor 1 − 1/e algorithm to get a truth
     assignment, τ2 .
  3. Output the better of the two assignments.



Theorem 16.9 Algorithm 16.8 is a deterministic factor 3/4 approximation
algorithm for MAX-SAT.

Proof: One of the two conditional expectations, E[W | b = 0] and E[W | b =
1], is at least as large as E[W ]. Hence, the total weight of clauses satisﬁed by
the better of τ1 and τ2 is at least as large as E[W ].                          ✷
    By (16.3), E[W ] ≥ 34 OPTf . The weight of the integral solution produced
by Algorithm 16.8 is at least E[W ]. Therefore, the integrality gap of LP (16.2)
is ≥ 3/4. Below we show that this is tight.
Example 16.10 Consider the SAT formula f = (x1 ∨ x2 ) ∧ (x1 ∨ x2 ) ∧ (x1 ∨
x2 ) ∧ (x1 ∨ x2 ), where each clause is of unit weight. It is easy to see that
setting yi = 1/2 and zc = 1 for all i and c is an optimal solution to LP (16.2)
for any instance having size 2 clauses. Therefore OPTf = 4. On the other
hand OPT = 3, and thus for this instance LP (16.2) has a integrality gap of
4/3.                                                                         ✷

Example 16.11 Let us provide a tight example to Algorithm 16.8. Let
f = (x ∨ y) ∧ (x ∨ y) ∧ (x ∨ z), and let the weights of these three clauses
be 1, 1, and 2 + ε, respectively. By the remark made in Example 16.10, on
this instance the factor 1 − 1/e algorithm will set each variable to True with
probability 1/2 and so will be the same as the factor 1/2 algorithm. During
derandomization, suppose variable x is set ﬁrst. The conditional expectations
are E[W | x = True] = 3 + ε/2 and E[W | x = False] = 3 + ε. Thus, x will
be set to False. But this leads to a total weight of 3 + ε, whereas by setting
x to True we can get a weight of 4 + ε. Clearly, we can get an inﬁnite family
of such examples by replicating these 3 clauses with new variables.          ✷



16.5 Exercises

16.1 The algorithm of Section 16.1 achieves an approximation guarantee
of αk if all clauses in the given instance have size at least k. Give a tight
example of factor αk for this algorithm.
138    16   Maximum Satisﬁability

16.2 Show that the following is a factor 1/2 algorithm for MAX-SAT. Let
τ be an arbitrary truth assignment and τ  be its complement, i.e., a variable
is True in τ iﬀ it is False in τ  . Compute the weight of clauses satisﬁed by τ
and τ  , then output the better assignment.

16.3 Use the method of conditional expectation to derandomize the 1 − 1/e
factor algorithm for MAX-SAT.

16.4 Observe that the randomization used in the 3/4 factor algorithm does
not set Boolean variables independently of each other. As remarked in Sec-
tion 16.2, the algorithm can still, in principle, be derandomized using the
method of conditional expectation. Devise a way of doing so. Observe that
the algorithm obtained is diﬀerent from Algorithm 16.8.

16.5 (Goemans and Williamson [104]) Instead of using the solution to LP
(16.2), yi∗ , as probability of setting xi to True, consider the more general
scheme of using g(yi∗ ), for a suitable function g. Can this lead to an improve-
ment over the factor 1 − 1/e algorithm?

16.6 Consider the following randomized algorithm for the maximum cut
problem, deﬁned in Exercise 2.1. After the initialization step of Algorithm
2.13, each of the remaining vertices is equally likely to go in sets A or B.
Show that the expected size of the cut found is at least OPT/2. Show that the
derandomization of this algorithm via the method of conditional expectation
is precisely Algorithm 2.13.

16.7 Consider the following generalization of the maximum cut problem.
Problem 16.12 (Linear equations over GF[2]) Given m equations over
n GF[2] variables, ﬁnd an assignment for the variables that maximizes the
number of satisﬁed equations.
 1. Show that if m ≤ n, this problem is polynomial time solvable.
 2. In general, the problem is NP-hard. Give a factor 1/2 randomized algo-
    rithm for it, and derandomize using the method of conditional expecta-
    tion.

16.8 Consider the obvious randomized algorithm for the MAX k-CUT prob-
lem, Problem 2.14 in Exercise 2.3, which assigns each vertex randomly to one
of the sets S1 , . . . , Sk . Show that the expected number of edges running be-
tween these sets is at least OPT/2. Show that the derandomization of this
algorithm, via the method of conditional expectation, gives the greedy algo-
rithm sought in Exercise 2.3.

16.9 Repeat Exercise 16.8 for the maximum directed cut problem, Problem
2.15 in Exercise 2.4, i.e., give a factor 1/4 randomized algorithm, and show
that its derandomization gives a greedy algorithm.
                                                        16.6   Notes   139

16.6 Notes
The factor 1/2 algorithm, which was also the ﬁrst approximation algorithm
for MAX-SAT, is due to Johnson [150]. The ﬁrst factor 3/4 algorithm was due
to Yannakakis [261]. The (simpler) algorithm given here is due to Goemans
and Williamson [104]. The method of conditional expectation is implicit in
Erdös and Selfridge [74]. Its use for obtaining polynomial time algorithms
was pointed out by Spencer [243] (see Raghavan [225] and Alon and Spencer
[6] for enhancements to this technique).
