---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-28"
chapter_number: 28
chapter_title: "Counting Problems"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 312
source_page_end: 323
printed_page_start: 294
printed_page_end: 305
part_ids: ["approximation-algorithms-ch-28-part-029"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Counting Problems

28 Counting Problems




The techniques for approximately counting the number of solutions to #P-
complete problems are quite diﬀerent from those for obtaining approximation
algorithms for NP-hard optimization problems. Much of the former theory
is built around the Markov chain Monte Carlo method, see Section 28.4 for
references. In this chapter, we will present combinatorial algorithms (not
using Markov chains) for two fundamental problems, counting the number of
satisfying truth assignments for a DNF formula, and estimating the failure
probability of an undirected network.
    Intuitively, the class #P captures the problems of counting the number of
solutions to NP problems. Let us formalize this notion. Let L be a language
in NP, M be its associated veriﬁer, and polynomial p be the bound on the
length of its Yes certiﬁcates (see Section A.1). For string x ∈ Σ ∗ , deﬁne f (x)
to be the number of strings y such that |y| ≤ p(|x|) and M (x, y) accepts.
Functions f : Σ ∗ → Z+ that arise in this manner constitute the class #P.
    Function f ∈ #P is said to be #P-complete if every function g ∈ #P can
be reduced to f in the following sense. There is a polynomial time transducer
R : Σ ∗ → Σ ∗ , that, given an instance, x, of g, produces an instance, R(x),
of f . Furthermore, there is a polynomial time computable function S : Σ ∗ ×
Z+ → Z+ that given x and f (R(x)) computes g(x), i.e.,

       ∀x ∈ Σ ∗ , g(x) = S(x, f (R(x))).

In other words, an oracle for f can be used to compute g in polynomial time.
    The solution counting versions of all known NP-complete problems are
#P-complete.1 Interestingly enough, other than a handful of exceptions, this
is true of problems in P as well. This raises the question of designing poly-
nomial time algorithms for approximately counting the number of solutions
to these latter problems (see Exercise 28.3 regarding the question of approx-
imately counting the number of solutions to NP-complete problems). These
problems admit only two interesting possibilities: they either allow approx-
imability to any required degree, or essentially not at all (see Section 28.4).
1
    In fact, typically a polynomial time reduction from one NP-complete problem
    to another maps solutions of the given instance to solutions of the transformed
    instance, and so preserves the number of solutions; hence, the proof of #P-
    completeness follows directly from the proof of NP-completeness.
                                               28.1   Counting DNF solutions         295

The former possibility is captured in the deﬁnition of a fully polynomial ran-
domized approximation scheme, abbreviated FPRAS.
    Consider a problem in P whose counting version, f , is #P-complete. An
algorithm A is an FPRAS for this problem if for each instance x ∈ Σ ∗ , and
error parameter ε > 0,

                                         3
      Pr[|A(x) − f (x)| ≤ εf (x)] ≥        ,
                                         4
and the running time of A is polynomial in |x| and 1/ε. (See Exercise 28.1
for a method for reducing the error probability of an FPRAS.)


28.1 Counting DNF solutions

Problem 28.1 (Counting DNF solutions) Let f = C1 ∨ C2 ∨ . . . ∨ Cm be
a formula in disjunctive normal form on n Boolean variables x1 , . . . , xn . Each
clause Ci is of the form Ci = l1 ∧ l2 ∧ . . . ∧ lri , where each lj is a literal, i.e.,
it is either a Boolean variable or its negation. We may assume w.l.o.g. that
each clause is satisﬁable, i.e., does not contain a variable and its negation.
The problem is to compute #f, the number of satisfying truth assignments
of f .
    The main idea is to deﬁne an eﬃciently samplable random variable X
which is an unbiased estimator for #f, i.e., E[X] = #f. If in addition, the stan-
dard deviation of X is within a polynomial factor of E[X], then an FPRAS for
#f can be obtained in a straightforward manner by sampling X a polynomial
number of times (in n and 1/ε) and outputting the mean.
    Constructing an unbiased estimator for #f is easy. Let random variable Y
have uniform distribution on all 2n truth assignments, and let Y (τ ) be 2n if τ
satisﬁes f , and 0 otherwise (see Exercise 28.4). However, this random variable
can have a very large standard deviation, and does not yield an FPRAS. For
instance, suppose f has only polynomially many satisfying truth assignments.
Then, with high probability, a polynomial number of randomly picked truth
assignments will all have Y = 0, giving a poor estimate for #f.
    We will rectify this by deﬁning a random variable that assigns nonzero
probability to only the satisfying truth assignments of f . Let Si denote the set
of truth assignments to x1 , . . . , xn that satisfy clause Ci . Clearly, |Si | = 2n−ri ,
where ri is the number of literals in clause Ci . Also, #f = | ∪m          i=1 Si |. Let
c(τ ) denote the number of clauses that truth assignment τ satisﬁes. Let M
denote the multiset union of the sets Si , i.e., it contains each   satisfying truth
assignment, τ , c(τ ) number of times. Notice that |M | = i |Si | is easy to
compute.
    Pick a satisfying truth assignment, τ , for f with probability c(τ )/|M |,
and deﬁne X(τ ) = |M |/c(τ ). We will ﬁrst show that X can be eﬃciently
sampled, i.e., using a randomized polynomial time algorithm.
296      28   Counting Problems

Lemma 28.2 Random variable X can be eﬃciently sampled.

Proof: Picking a random element from the multiset M ensures that each
truth assignment is picked with the desired probability. The following two-
step process will accomplish this. First pick a clause so that the probability of
picking clause Ci is |Si |/|M |. Next, among the truth assignments satisfying
the picked clause, pick one at random.
   Now, the probability with which truth assignment τ is picked is
                       |Si |   1    c(τ )
                              ×    =       .
                        |M | |Si |   |M |
      i:τ satisﬁes Ci

                                                                                       ✷

Lemma 28.3 X is an unbiased estimator for #f.

Proof:
                                                               c(τ )   |M |
      E[X] =          Pr[τ is picked] · X(τ ) =                        ×       = #f.
                                                                 |M |    c(τ )
                  τ                               τ satisﬁes f

                                                                                       ✷
   X takes values only in a “polynomial range”, thereby ensuring that its
standard deviation is not large compared to its expectation. This fact is
proved in the next lemma, and leads to the FPRAS construction.
Lemma 28.4 If m denotes the number of clauses in f , then

      σ(X)
           ≤ m − 1.
      E[X]

Proof: Denote |M |/m by α. Clearly, E[X] ≥ α. For each satisfying truth
assignment τ of f , 1 ≤ c(τ ) ≤ m. Therefore, X(τ ) lies in the range [α, mα],
and so the random variable deviates from its mean by at most (m − 1)α,
i.e., |X(τ ) − E[X]| ≤ (m − 1)α. Therefore, the standard deviation of X is
bounded by (m − 1)α. Using the lower bound on E[X] stated above, we get
the lemma.                                                                  ✷
   Finally, we will show that sampling X polynomially many times (in n
and 1/ε) and simply outputting the mean leads to an FPRAS for #f. Let Xk
denote the mean of k samples of X.
Lemma 28.5 For any ε > 0,

      Pr[|Xk − #f | ≤ ε#f ] ≥ 3/4,
                                                    28.2     Network reliability          297

where k = 4(m − 1)2 /ε2 .

Proof: We will use Chebyshev’s inequality (see Section B.2), with a =
εE[Xk ]. Using the value of k stated above we get
                                                   2                    2
                                          σ(Xk )                   σ(X)             1
      Pr[|Xk − E[Xk ]| ≥ εE[Xk ]] ≤                      =        √             ≤     ,
                                          εE[Xk ]                ε kE[X]            4

      the equality follows by noting that E[Xk ] = E[X] and σ(Xk ) =
where √
σ(X)/ k, and the last inequality follows by applying Lemma 28.4. The
lemma follows.                                                     ✷

Theorem 28.6 There is an FPRAS for the problem of counting DNF solu-
tions.


28.2 Network reliability

Problem 28.7 (Network reliability) Given a connected, undirected graph
G = (V, E), with failure probability pe speciﬁed for each edge e, compute the
probability that the graph becomes disconnected.
    Graph G will become disconnected if all edges in some cut (C, C), C ⊂ V
fail. We will present an FPRAS for this problem.
    Let us ﬁrst handle the case that each edge has the same failure probability,
denoted by p. However, we will allow G to have parallel edges between any
two vertices. Denote by FAIL(p) the probability that G gets disconnected. If
FAIL(p) is at least inverse polynomial, then it can be eﬃciently estimated by
Monte Carlo sampling (see proof of Theorem 28.11 for details). Let us handle
the diﬃcult case that FAIL(p) is small. Assume that FAIL(p) ≤ n−4 . The
reason for this choice will become clear below.
    The probability that cut (C, C) gets disconnected is simply pc where c
is the number of edges crossing this cut. Since the failure probability of a
cut decreases exponentially with capacity, the most important cuts for the
purpose of estimating FAIL(p) are cuts with “small” capacity. The algorithm
is built around two ideas:
 1. For any ε > 0, we will show that only polynomially many “small” cuts (in
    n and 1/ε) are responsible for 1−ε fraction of the total failure probability
    FAIL(p). Moreover, these cuts, say E1 , . . . Ek , Ei ⊆ E, can be enumerated
    in polynomial time.
 2. We will construct a polynomial sized DNF formula f whose probability
    of being satisﬁed is precisely the probability that at least one of these
    cuts fails.
298      28   Counting Problems

    As a result of the ﬁrst idea, it is suﬃcient to estimate the probability
that one of the cuts E1 , . . . Ek fails. However, because of correlations, this is
nontrivial. The second idea reduces this problem to counting DNF solutions,
for which we have an FPRAS.
    Formula f has a Boolean variable xe for each edge e. xe is set to true with
probability p, the failure probability of edge e. Suppose cut Ei = {e1 , . . . , ej }.
Construct the clause Di = xe1 ∧ · · · ∧ xej , i.e., the conjunct of all variables
corresponding to edges in this cut. The probability that this clause is satisﬁed
is precisely the failure probability of cut Ei . Finally, f = D1 ∨ · · · ∨ Dk , i.e.,
the disjunct of clauses corresponding to cuts.

28.2.1    Upperbounding the number of near-minimum cuts

The ﬁrst idea has its roots in the fact that one can place upper bounds on
the number of minimum and near-minimum capacity cuts in an undirected
graph. Let c be the capacity of a minimum cut in G. Recall that all edges in
G are assumed to be of unit capacity, and that G is allowed to have parallel
edges between any two vertices.
Lemma 28.8 The number of minimum cuts in G = (V, E) is bounded by
n(n − 1)/2.

Proof: By contracting an edge (u, v) in a graph we mean merging the vertices
u and v into a single vertex. All edges running between u and v are discarded.
Those running between u or v and some other vertex w will now run between
the merged vertex and w, their number being conserved.
   Now consider the following random contraction process. Iteratively, pick
a random edge (u, v) in the current graph and contract it. Terminate when
exactly two vertices are left. Suppose these two vertices correspond to sets S
and V − S, S ⊂ V , of vertices of the starting graph G. Then, the algorithm
outputs the cut (S, S). We will say that this cut survives. Clearly, a cut
survives iﬀ none of its edges is contracted during the algorithm.
   Let (C, C) be any minimum cut in G. We will show

                             1
      Pr[(C, C) survives] ≥ n .
                                 2

This statement yields the lemma via an interesting argument. Let M be the
number of minimum cuts in G. The survival of each of these cuts is a mutually
exclusive event, and the total probability of these events adds up to at most
1. Hence M/(n(n − 1)/2) ≤ 1, thereby giving the desired bound.
    Consider an arbitrary iteration in the random contraction process, and
let H be a graph at the beginning of this iteration. Since the process of
contraction cannot decrease the capacity of the minimum cut, the capacity
of each cut in H is at least c. This holds for cuts separating one vertex of H
                                                 28.2   Network reliability   299

from the rest. Therefore, the degree of each vertex in H must be at least c.
Hence, H must have at least cm/2 edges, where m is the number of vertices
in H.
    Now, the conditional probability that cut (C, C) survives the current it-
                                                               c
eration, given that it has survived so far, is at least (1 − cm/2 ) = (1 − 2/m)
(this is simply the probability that the randomly chosen edge in this iteration
is not picked from the cut (C, C)). The probability that (C, C) survives the
whole algorithm is simply the product of these conditional probabilities. This
gives
                                                             
                                    2           2               2      1
       Pr[(C, C) survives] ≥ 1 −          1−          ... 1 −       =  n .
                                    n         n−1               3       2

                                                                                ✷
      For α ≥ 1, we will say that a cut is an α-min cut if its capacity is at most
αc.
Lemma 28.9 For any α ≥ 1, the number of α-min cuts in G is at most n2α .

Proof: We will prove the lemma for the case that α is a half-integer. The
proof for arbitrary α follows by applying the same ideas to generalized bino-
mial coeﬃcients. Let 2α = k.
    Consider the following two-phase process: First, run the random contrac-
tion algorithm until there are k vertices remaining in the graph. Next, pick
a random cut among all 2k−1 cuts in this graph. This will deﬁne a cut in the
original graph.
    Let (C, C) be any α-min cut in G. We will show that the probability
that it survives the two phase process is at least 1/n2α , thereby proving the
desired bound.
    Let H be the graph at the beginning of an arbitrary iteration in the ﬁrst
phase. As argued in Lemma 28.8, if H has m vertices, it has at least mc/2
edges. Therefore, the conditional probability that (C, C) survives the current
                                                               αc
iteration, given that it has survived so far, is at least 1 − mc/2 = 1 − 2α/m.
The probability that (C, C) survives the ﬁrst phase is at least
                                     
          2α           2α              2α       1
       1−         1−          ... 1 −       =  n .
           n          n−1               3       k

   The conditional probability that (C, C) survives the second phase, given
that it has survived the ﬁrst, is 1/2k−1 . Therefore,

                                             1      1   1
        Pr[(C, C) survives both phases] ≥ n k−1 ≥ k = 2α .
                                           k 2     n   n

                                                                                ✷
300      28   Counting Problems

28.2.2    Analysis

Recall that we are considering the case that FAIL(p) ≤ n−4 . We can now jus-
tify this choice. The failure probability of a minimum cut is pc ≤ FAIL(p) ≤
n−4 . Let pc = n−(2+δ) , where δ ≥ 2. Now, by Lemma 28.9, for any α ≥ 1, the
total failure probability of all cuts of capacity αc is at most pαc n2α = n−αδ .
This rapid decrease in the total failure probability of all cuts of capacity αc
will enable us to bound the total failure probability of “large” capacity cuts.

Lemma 28.10 For any α ≥ 1,
                                                                         
                                                         −αδ          2
      Pr[some cut of capacity > αc fails] ≤ n                      1+         .
                                                                      δ

Proof: Number all cuts in G by increasing capacity. Let ck and pk denote
the capacity and failure probability of the kth cut in this numbering. Let a
be the number of the ﬁrst cut of capacity greater than αc. It suﬃces to show
that
                          
                          2
         pk ≤ n−αδ 1 +        .
                          δ
      k≥a


We will evaluate this sum in two steps. First, we will consider the ﬁrst n2α
terms. Each of these terms is at most pαc = n−α(2+δ) . Therefore, their sum
is at most n−αδ .
    Next, let us bound the sum of the remaining terms. Clearly, this sum
is bounded by k>n2α pk . By Lemma 28.9, there are at most n2α cuts of
capacity bounded by αc. Therefore, cn2β ≥ βc. Writing k = n2β we get
ck ≥ c ln k/2 ln n, and
                  ln k
      pk ≤ (pc ) 2 ln n = k −(1+δ/2) .

Therefore,
                        ∞
                                               2 −αδ
              pk ≤           k −(1+δ/2) dk ≤     n   .
                     n2α                       δ
      k>n2α


This proves the lemma.                                                            ✷

Theorem 28.11 There is an FPRAS for estimating network reliability.

Proof: We will ﬁrst consider the case that each edge in graph G has the
same failure probability, p.
                                               28.2   Network reliability   301

   If FAIL(p) > n−4 , then we will resort to Monte Carlo sampling. Flip
a coin with bias p for failure of each edge, and check if G is disconnected.
Repeat this experiment O(log n/(ε2 FAIL(p))) times, and output the mean
number of times G is disconnected. A straightforward application of Chernoﬀ
bounds shows that the mean lies in [(1−ε)FAIL(p), (1+ε)FAIL(p)] with high
probability.
   Next, assume that FAIL(p) ≤ n−4 . Now, for any ε > 0, we want to
determine α such that the total failure probability of all cuts of capacity
> αc is bounded by εFAIL(p). By Lemma 28.10, it suﬃces to ﬁnd α such
that
                  
        −αδ      2
      n       1+     ≤ εFAIL(p) ≤ εn−(2+δ) .
                 δ

Solving, we get

                2 ln(ε/2)      ln(ε/2)
      α=1+        −        ≤2−         .
                δ   δ ln n      2 ln n
By Lemma 28.9, cn2α > αc. For the value of α computed,

      Pr[one of the ﬁrst n2α fails] ≥ (1 − ε)FAIL(p).

The ﬁrst n2α = O(n4 /ε) cuts can be enumerated in polynomial time (see
Exercises). We will use these to construct the corresponding DNF formula,
and estimate the probability that it is satisﬁable, as described above.
    Finally, we show how to “reduce” the case of arbitrary edge failure prob-
abilities to the simpler case analyzed above. Suppose edge e has failure prob-
ability pe . Choose a small parameter θ. Replace edge e with ke = −(ln pe )/θ
parallel edges each with failure probability 1 − θ. Then, the probability that
all ke edges fail is

      (1 − θ)−(ln pe )/θ .

As θ → 0, this failure probability converges to pe . Let H be the graph ob-
tained by doing this transformation on each edge of G. In the limit as θ → 0,
each cut in H has the same failure probability as that in G.
    Let us give an eﬃcient implementation of this idea. All we really want is
a listing of the “small” capacity cuts in G. Once this is done, we can apply
the more general DNF counting algorithm developed in Exercise 28.5, where
each variable is set to true with its own probability pe . Observe that changing
θ scales the capacities of cuts in H without changing their relative values.
Thus, it suﬃces to assign a weight of − ln pe to each edge e of G, and ﬁnd
“small” capacity cuts in this graph. This completes the proof.                 ✷
302     28   Counting Problems

28.3 Exercises

28.1 Given an FPRAS for a problem, show that its success probability can
be improved to 1−δ, for any δ > 0, by a multiplicative increase in the running
time of only O(log(1/δ)).
Hint: Run the FPRAS O(log(1/δ)) times and output the median value.

28.2 Suppose we make the deﬁnition of an FPRAS more stringent by re-
quiring it to have a ﬁxed additive error α with high probability, i.e.,

                                                3
      Pr[f (x) − α ≤ A(x) ≤ f (x) + α] ≥          .
                                                4
Show that if there were such an algorithm for a #P-complete problem, then
P= NP.

28.3 Show that if there were an FPRAS for counting the number of satisfying
truth assignments to SAT then every problem in NP could be solved in
random polynomial time. How weak an approximate counting algorithm for
SAT suﬃces to give this consequence? What does this say for the question
of approximately counting the number of solutions to other NP-complete
problems?
Hint: Use solution ampliﬁcation. Given SAT formula f , deﬁne formula
f  over k new Boolean variables which is a tautology. Then the number of
solutions of φ = f ∧ f  is #f · 2k .

28.4 Given a DNF formula f , let Y be a random variable that on a random
truth assignment τ is 2n if τ satisﬁes f and 0 otherwise. Show that Y is an
unbiased estimator for #f. How large can the ratio σ(Y )/E[Y ] be?

28.5 (Karp and Luby [165]) You are given a DNF formula f on n Boolean
variables, x1 , . . . , xn , and probabilities p1 , . . . , pn with which these variables
are (independently) set to true. Let D denote the resulting probability distri-
bution over the 2n truth assignments to the Boolean variables, and p denote
the probability that f is satisﬁed by a truth assignment picked from D. Con-
struct an FPRAS for estimating p.
Hint: Let qi denote the probability that         clause Ci is satisﬁed by a truth
assignment picked from D, and Q =                   i qi . Now, consider random vari-
able X that assigns to each satisfying truth assignment τ a probability of
PrD [τ ]c(τ )/Q, and deﬁne X(τ ) = Q/c(τ ).

28.6 A uniform generator for an NP problem Π is a randomized polynomial
time algorithm A that given an instance I of a problem, outputs either a
solution to I, or else the special symbol “⊥”, such that
                                                         28.3   Exercises    303

• each solution to I is output with the same probability, i.e., there is a number
  α ∈ (0, 1] such that

        Pr[A outputs s] = α, for each solution s of I, and

• the probability of outputting ⊥, i.e., failing to output a solution, is < 1/2.
Give a uniform generator for picking a random satisfying truth assignment
to a given DNF formula.
Hint: The essential idea behind the construction of random variable X
works.

28.7 (Jerrum, Valiant, and Vazirani [147]) Let Π be an NP problem that
is self-reducible (see Section A.5 and Exercise 1.15). Show that there is an
FPRAS for Π iﬀ there is an almost uniform generator for it. An almost
uniform generator for Π is a randomized polynomial time algorithm A such
that for any µ > 0 and instance I of Π, there is a number α ∈ (0, 1] such
that
• for each solution s of I, Pr[A outputs s] ∈ [(1 − µ)α, (1 + µ)α],
• Pr[A fails to output a solution] < 1/2, and
• the running time of A is polynomial in |I| and log(1/µ).
Observe that unlike an FPRAS, which can only achieve inverse polynomial
error, a uniform generator can achieve inverse exponential error (µ), in poly-
nomial time.
Hint: For the forward direction, ﬁrst construct a uniform generator, as-
suming that the FPRAS makes no error. (Traverse down the self-reducibility
tree for I, with biases determined by estimates on the number of solutions.
Accept leaf with appropriate probability, to achieve uniform generation.) Use
the fact that the error probability of the FPRAS can be made exponentially
small to obtain an almost uniform generator. For the reverse direction, obtain
instance Iα , with |Iα | < |I|, and a good estimate of the ratio of the number
of solutions to I and Iα .

28.8 (Jerrum, Valiant, and Vazirani [147]) This exercise leads to strong
evidence that the problem of estimating the number of simple cycles in a
directed graph is essentially not approximable. Show that if there is an almost
uniform generator for this problem, then there is a randomized polynomial
time algorithm for deciding if a given directed graph has a Hamiltonian cycle.

Hint: Obtain a graph G from G that ampliﬁes the number of cycles of each
length. However, it ampliﬁes bigger cycles more than it ampliﬁes smaller
cycles, so that most cycles in G are of maximum length and correspond to
the largest cycles in G.
304     28   Counting Problems

28.9 Show that the random contraction algorithm of Lemma 28.8 can be
used to obtain a randomized algorithm for ﬁnding a minimum cut in an
undirected graph.

28.10 (Karger and Stein [159]) Obtain a randomized algorithm for enu-
merating all α-min cuts in G using the random contraction algorithm and
Lemma 28.9.

    In the next three exercises (from Vazirani and Yannakakis [252]), we will
develop a deterministic algorithm for enumerating in an undirected graph by
increasing weight, with polynomial delay, i.e., the algorithm spends polyno-
mial time between successive outputs.
    Assume that graph G = (V, E) has n vertices besides s and t, numbered 1
to n. Every s–t cut in G can be represented as an n bit 0/1 vector. A partially
speciﬁed cut, in which the sides of only vertices numbered 1 to k are decided,
is represented as a k bit 0/1 vector. Consider a binary tree T of height n. Its
leaves represent s–t cuts and internal nodes represent partially speciﬁed cuts.
All cuts consistent with a partially speciﬁed cut lie in the subtree rooted at
it. Clearly, a minimum weight cut in this subtree can be computed with one
max-ﬂow computation.

28.11 Let a be an n − k bit 0/1 vector representing a partially speciﬁed
cut, as well as an internal node in T . The subtree, T  , rooted at this node
is of height k and contains 2k leaves (s–t cuts). Among these 2k cuts, let a
be a minimum weight cut. Show how the remaining 2k − 1 cuts of T  can be
partitioned into k subtrees which are of height 0, 1, . . . , k − 1.

28.12 Using a heap, give an algorithm for enumerating s–t cuts in G by
increasing weight.
Hint: The heap is initialized with a minimum cut in G. At an arbitrary
point, the cuts not enumerated so far can be partitioned into subtrees (see
Exercise 28.11). The heap contains a minimum cut from each subtree.

28.13 Give an algorithm for enumerating all cuts in an undirected graph by
increasing weight with polynomial delay.
Hint: Assume that the graph has a special vertex s, which always goes on
side 0 of the cut, and n other vertices, numbered 1 to n. A cut is speciﬁed
by an n bit vector specifying the sides of vertices numbered 1 to n. The
main diﬀerence arises in ﬁnding a minimum cut in the subtree rooted at the
internal node 0k , k < n. This is done by ﬁnding a minimum cut separating
the vertices s, 1, . . . , i from vertex i + 1 for k ≤ i < n, and picking the lightest
of these cuts.

28.14 (Karger [156]) Consider the generalization of network reliability to
estimating the probability that G disconnects into r or more components,
where r is a ﬁxed constant. Obtain an FPRAS for this problem.
                                                          28.4   Notes    305

28.4 Notes
The counting class #P was deﬁned by Valiant [249]. The FPRAS for counting
DNF solutions is due to Karp and Luby [165], who also gave the deﬁnition of
FPRAS (see also Karp, Luby, and Madras [166]). The FPRAS for estimating
network reliability is due to Karger [156].
    Most algorithms for approximate counting work by constructing an al-
most uniform generator for the problem and appealing to the equivalence
established in Exercise 28.7. Broder [33] introduced the use of rapidly mixing
Markov chains for almost uniform generation (see also Mihail [205]).
    Jerrum and Sinclair [145] gave the ﬁrst FPRAS using this approach, for
counting the number of perfect matchings in dense bipartite graphs (each
vertex should have a degree ≥ n/2; see also Section 30.3). They also showed
that a crude approximate counter, with polynomial error, can be transformed
into an FPRAS (with inverse polynomial error), by deﬁning an appropri-
ate Markov chain on the self-reducibility tree of an instance. As a result,
#P-complete problems either admit an FPRAS or are essentially not ap-
proximable at all (see Exercise 28.8) For Markov–chain based approximate
counting algorithms, see Jerrum and Sinclair [142], Sinclair [242], and the
references in Section 30.3.
