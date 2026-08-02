---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-29"
chapter_number: 29
chapter_title: "Hardness of Approximation"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 324
source_page_end: 351
printed_page_start: 306
printed_page_end: 333
part_ids: ["approximation-algorithms-ch-29-part-030"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Hardness of Approximation

29 Hardness of Approximation




A remarkable achievement of the theory of exact algorithms is that it has
provided a fairly complete characterization1 of the intrinsic complexity of
natural computational problems, modulo some strongly believed conjectures.
Recent impressive developments raise hopes that we will some day have a
comprehensive understanding of the approximability of NP-hard optimiza-
tion problems as well. In this chapter we will give a brief overview of these
developments.
    Current hardness results fall into three important classes. For minimiza-
tion problems, the hardness factors for these classes are constant (> 1),
Ω(log n), and nε for a ﬁxed constant ε > 0, where n is the size of the instance.
For maximization problems, the factors are constant (< 1), O(1/ log n), and
1/nε for a ﬁxed ε > 0. In this chapter we will present hardness results for
MAX-3SAT, vertex cover, and Steiner tree in the ﬁrst class, set cover in the
second class, and clique in the third class. For all these problems, we will
establish hardness for their cardinality versions, i.e., the unit cost case.


29.1 Reductions, gaps, and hardness factors
Let us start by recalling the methodology for establishing hardness results
for exact optimization problems. The main technical core is the Cook–Levin
theorem which establishes the hardness, assuming P = NP, of distinguishing
between instances of SAT that are satisﬁable and those that are not. To show
hardness of computing an optimal solution to, say the cardinality vertex cover
problem, one shows, via a polynomial time reduction from SAT, that it is hard
to distinguish between graphs that have covers of size at most k from graphs
that don’t, where k is provided as part of the input. Since an exact algorithm
can make this distinction, this reduction establishes the non-existence of an
eﬃcient exact algorithm.
    The main technical core of hardness of approximation results is the PCP
theorem, which is stated in Section 29.2. For establishing a hardness of ap-
proximation result for, say, the vertex cover problem, this theorem is used to
1
    A few (important) exceptions, such as the graph isomorphism problem, remain
    uncharacterized.
                             29.1   Reductions, gaps, and hardness factors     307

show the following polynomial time reduction. It maps an instance φ of SAT
to a graph G = (V, E) such that
• if φ is satisﬁable, G has a vertex cover of size ≤ 23 |V |, and
• if φ is not satisﬁable, the smallest vertex cover in G is of size > α · 23 |V |,
where α > 1 is a ﬁxed constant.
Claim 29.1 As a consequence of the reduction stated above, there is no poly-
nomial time algorithm for vertex cover that achieves an approximation guar-
antee of α, assuming P = NP.

Proof: Essentially, this reduction establishes the hardness, assuming P =
NP, of distinguishing graphs having a cover of size ≤ 23 |V | from those having
a cover of size > α· 23 |V |. An approximation algorithm for vertex cover, having
a guarantee of α or better, will ﬁnd a cover of size ≤ α · 23 |V | when given
a graph G from the ﬁrst class. Thus, it will be able to distinguish the two
classes of graphs, leading to a contradiction.                                  ✷
    The reduction stated above introduces a gap, of factor α, in the optimal
objective function value achieved by the two classes of graphs (if α = 1 then
this is an ordinary polynomial time reduction from SAT to vertex cover).
Let us formally state the central notion of a gap-introducing reduction. The
deﬁnition is slightly diﬀerent for minimization and maximization problems.
For simplicity, let us assume that we are always reducing from SAT.
    Let Π be a minimization problem. A gap-introducing reduction from SAT
to Π comes with two parameters, functions f and α. Given an instance φ of
SAT, it outputs, in polynomial time, an instance x of Π, such that
• if φ is satisﬁable, OPT(x) ≤ f (x), and
• if φ is not satisﬁable, OPT(x) > α(|x|) · f (x).
Notice that f is a function of the instance (such as 23 |V | in the example
given above), and α is a function of the size of the instance. Since Π is a
minimization problem, the function α satisﬁes α(|x|) ≥ 1.
   If Π is a maximization problem, we want the reduction to satisfy
• if φ is satisﬁable, OPT(x) ≥ f (x), and
• if φ is not satisﬁable, OPT(x) < α(|x|) · f (x).
In this case, α(|x|) ≤ 1. The gap, α(|x|), is precisely the hardness factor
established by the gap-introducing reduction for the NP-hard optimization
problem.
    Once we have obtained a gap-introducing reduction from SAT (or any
other NP-hard problem) to an optimization problem, say Π1 , we can prove
a hardness result for another optimization problem, say Π2 , by giving a spe-
cial reduction, called a gap-preserving reduction, from Π1 to Π2 . Now there
are four possibilities, depending on whether Π1 and Π2 are minimization or
maximization problems. We give the deﬁnition below assuming that Π1 is
308    29   Hardness of Approximation

a minimization problem and Π2 is a maximization problem. The remaining
cases are similar.
    A gap-preserving reduction, Γ , from Π1 to Π2 comes with four parame-
ters (functions), f1 , α, f2 , and β. Given an instance x of Π1 , it computes, in
polynomial time, an instance y of Π2 such that
•       OPT(x) ≤ f1 (x) ⇒ OPT(y) ≥ f2 (y),
•       OPT(x) > α(|x|)f1 (x) ⇒ OPT(y) < β(|y|)f2 (y).
Observe that x and y are instances of two diﬀerent problems, and so it would
be more appropriate to write OPTΠ1 (x) and OPTΠ2 (y) instead of OPT(x)
and OPT(y), respectively. However, we will avoid this extra notation, since
the context clariﬁes the problems being talked about. In keeping with the
fact that Π1 is a minimization problem and Π2 is a maximization problem,
α(|x|) ≥ 1 and β(|y|) ≤ 1.
    Composing a gap-introducing reduction with a gap-preserving reduction
gives a gap-introducing reduction, provided all the parameters match up. For
example, suppose that in addition to the reduction Γ deﬁned above, we have
obtained a gap-introducing reduction, Γ  , from SAT to Π1 , with parameters
f1 and α. Then, composing Γ  with Γ , we get a gap-introducing reduction
from SAT to Π2 , with parameters f2 and β. This composed reduction shows
that there is no β(|y|) factor approximation algorithm for Π2 , assuming P =
NP. In each gap-preserving reduction stated below, we will take special care
to ensure that the parameters match up.
Remark 29.2
• The “gap” β can, in general, be bigger or smaller than α. In this sense,
  “gap-preserving” is a slight misnomer.
• We do not require any guarantee from reduction Γ if instance x of Π1 falls
  in the ﬁrst gap, i.e., satisﬁes f1 (x) < OPT(x) ≤ α(|x|)f1 (x).
• An approximation algorithm for Π2 together with a gap-preserving re-
  duction Γ from Π1 to Π2 does not necessarily yield an approximation
  algorithm for Π1 . Observe the contrast with an approximation factor pre-
  serving reduction (see Section A.3.1 for deﬁnition). The latter reduction
  additionally requires a means of transforming a near-optimal solution to
  the transformed instance y of Π2 into a near-optimal solution to the given
  instance x of Π1 .
  On the other hand, Γ together with an appropriate gap-introducing reduc-
  tion from SAT to Π1 does suﬃce for proving a hardness of approximation
  result for Π2 . Obviously the less stringent requirement on gap-preserving
  reductions makes them easier to design.
• We have already presented some gap-introducing reductions, e.g., Theo-
  rems 3.6 and 5.7. The reader may wonder why these do not suﬃce as the
  starting point for further hardness results and why the PCP theorem was
  needed. The reason is that these reductions simply exploit the freedom to
  choose edge costs and not the deep combinatorial structure of the problem.
                                                 29.2   The PCP theorem    309

   The following ﬁgure shows the gap-preserving reductions presented in this
chapter:
                                    PCP theorem
                                             ❅
                                                 ❅
                                                   ❅
                                                    ❅
                         ✠                ❄             ❅❅
                                                         ❘
                    MAX-3SAT         Set cover          Clique

                       ❄
                  MAX-3SAT(5)


                         ❄
                   Vertex cover


                          ❄
                    Steiner tree


29.2 The PCP theorem
Probabilistic characterizations of the class NP yield a general technique for
obtaining gap-introducing reductions. The most useful of these character-
izations is captured in the PCP theorem. PCP stands for probabilistically
checkable proof systems.
    Recall the usual deﬁnition of NP (see Appendix A) as the class of lan-
guages whose yes instances support short (polynomial in the length of the
input) witnesses that can be veriﬁed quickly (in polynomial time). Informally,
a probabilistically checkable proof for an NP language encodes the witness
in a special way so that it can be veriﬁed probabilistically by examining very
few of its bits.
    A probabilistically checkable proof system comes with two parameters,
the number of random bits required by the veriﬁer, and the number of bits of
the witness that the veriﬁer is allowed to examine. In keeping with established
terminology, let us call a witness string the proof. The most useful setting
for these parameters is O(log n) and O(1), respectively. This deﬁnes the class
PCP(log n, 1).
    The veriﬁer is a polynomial time Turing machine which, besides its input
tape and work tape, has a special tape that provides it with a string of
random bits and another special tape on which it is provided with the proof.
The machine can read any bit of the proof by simply specifying its location.
Of course, the particular locations it examines are a function of the input
310    29   Hardness of Approximation

string and the random string. At the end of its computation, the machine
goes into either an accept state or a reject state.

                                                 Input
                                             ❄
                                                   x
                   Veriﬁer
                                                            Proof
                      V                                 ❄
                                                       y


                                                  ❄ Random bits
                                                 r


  Work tape
                      ❄


    A language L ∈ PCP(log n, 1) if there is a veriﬁer V , and constants c and
q, such that on input x, V obtains a random string, r, of length c log |x| and
queries q bits of the proof. Furthermore,
• if x ∈ L, then there is a proof y that makes V accept with probability 1,
       / L, then for every proof y, V accepts with probability < 1/2,
• if x ∈
where the probability is over the random string r. The probability of accepting
          / L is called the error probability.
in case x ∈
    In general, for two functions r(n) and q(n), we can deﬁne the class
PCP(r(n), q(n)), under which the veriﬁer obtains O(r(n)) random bits and
queries O(q(n)) bits of the proof. The acceptance criteria for input strings
are the same as above. In this terminology, NP = PCP(0, poly(n)), where
poly(n) = k≥0 {nk }. In this case, the veriﬁer is not allowed any random bits.
It must deterministically accept strings in the language and reject strings not
in the language, as in the deﬁnition of NP. The PCP theorem gives another
characterization of NP.
Theorem 29.3 NP = PCP(log n, 1).
   One half of this theorem, that PCP(log n, 1) ⊆ NP, is easy to prove (see
Exercise 29.1). The other half, that NP ⊆ PCP(log n, 1), is a diﬃcult result,
and gives a useful tool for establishing hardness of approximation results. The
currently known proof of this half is too complicated for exposition in this
book. Fortunately, the statement of the theorem is suﬃcient to derive the
hardness results.
   In order to provide the reader with some feel for the PCP theorem, let us
make an observation. It is easy to construct a veriﬁer for 3SAT whose error
                                          29.3   Hardness of MAX-3SAT        311

probability (i.e., probability of accepting unsatisﬁable formulae) is ≤ 1−1/m,
where m is the number of clauses in the input 3SAT formula, say φ. The
veriﬁer expects a satisfying truth assignment to φ as the proof. It uses the
O(log n) random bits to pick a random clause of φ. It then reads the truth
assignments for the three variables occurring in this clause. Notice that this is
only a constant number of bits. It accepts iﬀ the truth setting for these three
variables satisﬁes the clause. Clearly, if φ is satisﬁable, there is a proof that
makes the veriﬁer accept with probability 1, and if φ is not satisﬁable, on
every proof, the veriﬁer accepts with probability ≤ 1 − 1/m. The interesting
and diﬃcult part of the PCP theorem is decreasing the error probability to
< 1/2, even though the veriﬁer is allowed to read only a constant number of
bits of the proof. It involves a complex algebraic construction that ensures
that small parts of the proof depend on every bit of the input.
    The PCP theorem directly gives an optimization problem – in particular,
a maximization problem – for which there is no factor 1/2 approximation
algorithm, assuming P = NP.
Problem 29.4 (Maximize accept probability) Let V be a PCP(log n, 1)
veriﬁer for SAT. On input φ, a SAT formula, ﬁnd a proof that maximizes the
probability of acceptance of V .
Claim 29.5 Assuming P = NP, there is no factor 1/2 approximation algo-
rithm for Problem 29.4.

Proof: If φ is satisﬁable, then there is a proof that makes V accept with
probability 1, and if φ is not satisﬁable, then on every proof, V accepts with
probability < 1/2. Suppose there is a factor 1/2 approximation algorithm for
Problem 29.4. If φ is satisﬁable, then this algorithm must provide a proof
on which V ’s acceptance probability is ≥ 1/2. The acceptance probability
can be computed in polynomial time, by simply simulating V for all random
strings of length O(log n). Thus, this approximation algorithm can be used
for deciding SAT in polynomial time, contradicting the assumption P = NP.
✷
    Claim 29.5 directly gives the following corollary. In subsequent sections,
we will use the PCP theorem to obtain hardness results for natural compu-
tational problems. A similar corollary follows in each case.
Corollary 29.6 Assuming P = NP, there is no PTAS for Problem 29.4.



29.3 Hardness of MAX-3SAT
MAX-3SAT is the restriction of MAX-SAT (see Problem 16.1) to instances in
which each clause has at most three literals. This problem plays a similar role
in hardness of approximation as 3SAT plays in the theory of NP-hardness,
312     29   Hardness of Approximation

as a “seed” problem from which reductions to numerous other problems have
been found. The main result of this section is:
Theorem 29.7 There is a constant εM > 0 for which there is a gap-
introducing reduction from SAT to MAX-3SAT that transforms a Boolean
formula φ to ψ such that
• if φ is satisﬁable, OPT(ψ) = m, and
• if φ is not satisﬁable, OPT(ψ) < (1 − εM )m,
where m is the number of clauses in ψ.
Corollary 29.8 There is no approximation algorithm for MAX-3SAT with
an approximation guarantee of 1 − εM , assuming P = NP, where εM > 0 is
the constant deﬁned in Theorem 29.7.
   The exact solution of MAX-3SAT is shown hard under the assumption
P = NP. It is interesting to note that hardness of approximate solution of
MAX-3SAT is also being established under the same assumption.
   For clarity, let us break the proof into two parts. We will ﬁrst prove
hardness for the following problem.
Problem 29.9 (MAX k-FUNCTION SAT) Given n Boolean variables
x1 , . . . , xn and m functions f1 , . . . , fm , each of which is a function of k of the
Boolean variables, ﬁnd a truth assignment to x1 , . . . , xn that maximizes the
number of functions satisﬁed. Here k is assumed to be a ﬁxed constant. Thus,
we have a class of problems, one for each value of k.
Lemma 29.10 There is a constant k for which there is a gap-introducing
reduction from SAT to MAX k-FUNCTION SAT that transforms a Boolean
formula φ to an instance I of MAX k-FUNCTION SAT such that
• if φ is satisﬁable, OPT(I) = m, and
• if φ is not satisﬁable, OPT(I) < 12 m,
where m is the number of formulae in I.

Proof: Let V be a PCP(log n, 1) veriﬁer for SAT, with associated param-
eters c and q. Let φ be an instance of SAT of length n. Corresponding to
each string, r, of length c log n (the “random” string), V reads q bits of the
proof. Thus, V reads a total of at most qnc bits of the proof. We will have
a Boolean variable corresponding to each of these bits. Let B be the set of
Boolean variables. Thus, the relevant part of each proof corresponds to a
truth assignment to the variables in B.
    We will establish the lemma for k = q. Corresponding to each string r,
we will deﬁne a Boolean function, fr . This will be a function of q variables
from B. The acceptance or rejection of V is of course a function of φ, r, and
the q bits of the proof read by V . For ﬁxed φ and r, consider the restriction
of this function to the q bits of the proof. This is the function fr .
     29.4   Hardness of MAX-3SAT with bounded occurrence of variables          313

    Clearly, there is a polynomial time algorithm which, given input φ, out-
puts the m = nc functions fr . If φ is satisﬁable, there is a proof that makes
V accept with probability 1. The corresponding truth assignment to B sat-
isﬁes all nc functions fr . On the other hand, if φ is not satisﬁable, then on
every proof, V accepts with probability < 1/2. Thus, in this case every truth
assignment satisﬁes < 12 nc of these functions. The lemma follows.           ✷

Proof of Theorem 29.7: Using Lemma 29.10 we transform a SAT formula
φ to an instance of MAX k-FUNCTION SAT. We now show how to obtain
a 3SAT formula from the nc functions.
    Each Boolean function fr constructed in Lemma 29.10 can be written
as a SAT formula, say ψr , containing at most 2q clauses. Each clause of ψr
contains at most q literals. Let ψ be the *  SAT formula obtained by taking the
conjunct of all these formulae, i.e., ψ = r ψr .
    If a truth assignment satisﬁes formula fr , then it satisﬁes all clauses of
ψr . On the other hand, if it does not satisfy fr , then it must leave at least one
clause of ψr unsatisﬁed. Therefore, if φ is not satisﬁable, any truth assignment
must leave > 12 nc clauses of ψ unsatisﬁed.
    Finally, let us transform ψ into a 3SAT formula. This is done using the
standard trick of introducing new variables to obtain small clauses from a
big clause. Consider clause C = (x1 ∨ x2 ∨ . . . ∨ xk ), with k > 3. Introduce
k − 2 new Boolean variables, y1 , . . . , yk−2 , and consider the formula

      f = (x1 ∨ x2 ∨ y1 ) ∧ (y 1 ∨ x3 ∨ y2 ) ∧ . . . ∧ (y k−2 ∨ xk−1 ∨ xk ).

Let τ be any truth assignment to x1 , . . . , xk . If τ satisﬁes C, then it can be
extended to a truth assignment satisfying all clauses of f . On the other hand,
if τ does not satisfy C, then for every way of setting y1 , . . . , yk−2 , at least
one of the clauses of f remains unsatisﬁed.
    We apply this construction to every clause of ψ containing more than 3 lit-
erals. Let ψ  be the resulting 3SAT formula. It contains at most nc 2q (q − 2)
clauses. If φ is satisﬁable, then there is a truth assignment satisfying all
clauses of ψ  . If φ is not satisﬁable, > 12 nc of the clauses remain unsatis-
ﬁed, under every truth assignment. Setting εM = 1/(2q+1 (q − 2)) gives the
theorem.                                                                         ✷



29.4 Hardness of MAX-3SAT with bounded occurrence
of variables
For each ﬁxed k, deﬁne MAX-3SAT(k) to be the restriction of MAX-3SAT
to Boolean formulae in which each variable occurs at most k times. This
problem leads to reductions to some key optimization problems.
314    29   Hardness of Approximation

Theorem 29.11 There is a gap preserving reduction from MAX-3SAT to
MAX-3SAT(29) that transforms a Boolean formula φ to ψ such that
• if OPT(φ) = m, then OPT(ψ) = m , and
• if OPT(φ) < (1 − εM )m, then OPT(ψ) < (1 − εb )m ,
where m and m are the number of clauses in φ and ψ, εM is the constant
determined in Theorem 29.7, and εb = εM /43.

Proof: The proof critically uses expander graphs. Recall, from Section 20.3,
that graph G = (V, E) is an expander if every vertex has the same degree,
and for any nonempty subset S ⊂ V ,

      |E(S, S)| > min(|S|, |S|),

where E(S, S) denotes the set of edges in the cut (S, S), i.e., edges that have
one endpoint in S and the other in S. Let us assume that such graphs are
eﬃciently constructible in the following sense. There is an algorithm A and a
constant N0 such that for each N ≥ N0 , A constructs a degree 14 expander
graph on N vertices in time polynomial in N (Remark 29.12 clariﬁes this
point).
    Expanders enable us to construct the following device whose purpose is to
ensure that in any optimal truth assignment, a given set of Boolean variables
must have consistent assignment, i.e., all true or all false. Let k ≥ N0 , and
let Gx be a degree 14 expander graph on k vertices. Label the vertices with
distinct Boolean variables x1 , . . . , xk . We will construct a CNF formula ψx
on these Boolean variables. Corresponding to each edge (xi , xj ) of Gx , we
will include the clauses (xi ∨ xj ) and (xj ∨ xi ) in ψx . A truth assignment
to x1 , . . . , xk is said to be consistent if either all the variables are set to
true or all are set to false. An inconsistent truth assignment partitions the
vertices of Gx into two sets, say S and S. Assume w.l.o.g. that S is the
smaller set. Now, corresponding to each edge in the cut (S, S), ψx will have
an unsatisﬁed clause. Therefore, the number of unsatisﬁed clauses, |E(S, S)|,
is at least |S| + 1. We will use this fact critically.
    Next, we describe the reduction. We may assume w.l.o.g. that every vari-
able occurs in φ at least N0 times. If not, we can replicate each clause N0
times without changing the approximability properties of the formula in any
essential way.
    Let B denote the set of Boolean variables occurring in φ. For each variable
x ∈ B, we will do the following. Suppose x occurs k ≥ N0 times in φ. Let
Vx = {x1 , . . . , xk } be a set of completely new Boolean variables. Let Gx be a
degree 14 expander graph on k vertices. Label its vertices with variables from
Vx and obtain formula ψx as described above. Finally, replace each occurrence
of x in φ by a distinct variable from Vx . After this process is carried out for
each variable x ∈ B, every occurrence of a variable in φ is replaced by a
distinct variable from the set of new variables
     29.4   Hardness of MAX-3SAT with bounded occurrence of variables        315
            2
      V =         Vx .
            x∈B


Let φ be the resulting formula. In addition, corresponding to each variable
x ∈ B, a formula ψx has been constructed.
   Finally, let
                    3
      ψ = φ ∧ (         ψx ).
                   x∈B


Observe that for each x ∈ B, each variable of Vx occurs exactly 29 times
in ψ – once in φ , and 28 times in ψx . Therefore, ψ is an instance of MAX-
3SAT(29). We will say that the clauses of φ are Type I clauses, and the
remaining clauses of ψ are Type II clauses.
     Now, the important claim is that an optimal truth assignment for ψ must
satisfy all Type II clauses, and therefore must be consistent for each set
Vx , x ∈ B. Suppose that this is not the case. Let τ be an optimal truth
assignment that is not consistent for Vx , for some x ∈ B. τ partitions the
vertices of Gx into two sets, say S and S, with S being the smaller set. Now,
ﬂip the truth assignment to variables in S, keeping the rest of the assignment
the same as τ . As a result, some Type I clauses that were satisﬁed under
τ may now be unsatisﬁed. Each of these must contain a variable of S, and
so their number is at most |S|. On the other hand we get at least |S| + 1
new satisﬁed clauses corresponding to the edges in the cut (S, S). Thus, the
ﬂipped assignment satisﬁes more clauses than τ , contradicting the optimality
of τ .
     Let m and m be the number of clauses in φ and ψ. The total number of
occurrences of all variables in φ is at most 3m. Each occurrence participates
in 28 Type II two-literal clauses, giving a total of at most 42m Type II clauses.
In addition, ψ has m Type I clauses. Therefore, m ≤ 43m.
     If φ is satisﬁable, then so is ψ. Next, consider the case that OPT(φ) <
(1 − εM )m, i.e., > εM m clauses of φ remain unsatisﬁed under any truth
assignment. If so, by the above claim, > εM m ≥ εM m /43 of the clauses of
ψ must remain unsatisﬁed. The theorem follows.                                 ✷

Remark 29.12 The assumption about the eﬃcient construction of expander
graphs is slightly untrue. It is known that for each N ≥ N0 , an expander of
size ≤ N (1+o(1)) can be constructed eﬃciently (see Section 29.9). The reader
can verify that this does not change the status of Theorem 29.11.
    Exercise 29.4 extends Theorem 29.11 to establishing hardness for MAX-
3SAT(5).
316    29   Hardness of Approximation

29.5 Hardness of vertex cover and Steiner tree
In this section, we will apply the machinery developed above to some graph
theoretic problems. For integer d ≥ 1, let VC(d) denote the restriction of the
cardinality vertex cover problem to instances in which each vertex has degree
at most d.

Theorem 29.13 There is a gap preserving reduction from MAX-3SAT(29)
to VC(30) that transforms a Boolean formula φ to a graph G = (V, E) such
that
• if OPT(φ) = m, then OPT(G) ≤ 23 |V |, and
• if OPT(φ) < (1 − εb )m, then OPT(G) > (1 + εv ) 23 |V |,
where m is the number of clauses in φ, εb is the constant determined in
Theorem 29.11, and εv = εb /2.

Proof: Assume w.l.o.g. that each clause of φ has exactly 3 literals (this can
be easily accomplished by repeating the literals within a clause, if necessary).
We will use the standard transformation. Corresponding to each clause of
φ, G has 3 vertices. Each of these vertices is labeled with one literal of the
clause. Thus, |V | = 3m. G has two types of edges (see the illustration below):
• for each clause, G has 3 edges connecting its 3 vertices, and
• for each u, v ∈ V , if the literals labeling u and v are negations of each
  other, then (u, v) is an edge in G.
Each vertex of G has two edges of the ﬁrst type and at most 28 edges of the
second type. Hence, G has degree at most 30.
    We claim that the size of a maximum independent set in G is precisely
OPT(φ). Consider an optimal truth assignment and pick one vertex, corre-
sponding to a satisﬁed literal, from each satisﬁed clause. Clearly, the picked
vertices form an independent set. Conversely, consider an independent set I in
G, and set the literals corresponding to its vertices to be true. Any extension
of this truth setting to all variables must satisfy at least |I| clauses.
    The complement of a maximum independent set in G is a minimum vertex
cover. Therefore, if OPT(φ) = m then OPT(G) = 2m. If OPT(φ) < (1−εb )m,
then OPT(G) > (2 + εb )m. The theorem follows.                                ✷
  As an illustration, consider the formula (x1 ∨ x2 ∨ x3 ) ∧ (x1 ∨ x2 ∨ x3 ).
The graph produced by the reduction given in Theorem 29.13 is given below:
                           29.5    Hardness of vertex cover and Steiner tree            317


                        ✎
                     x1 t                                    x1  t
                       ✔❚                                       ✔❚
                      ✔ ❚                                      ✔ ❚
                     ✔    ❚                                  ✔     ❚
                   ✔       ❚                               ✔           ❚
                 ✔             ❚                         ✔              ❚
                ✔                 ❚                    ✔                   ❚
              ✔                       ❚              ✔                         ❚
             ✔
          x2 t                         ❚t x3    x2 t✔                           ❚t x3
             ✍                                     ✌
Theorem 29.14 There is a gap preserving reduction from VC(30) to the
Steiner tree problem. It transforms an instance G = (V, E) of VC(30) to an
instance H = (R, S, cost) of Steiner tree, where R and S are the required and
Steiner vertices of H, and cost is a metric on R ∪ S. It satisﬁes:
• if OPT(G) ≤ 23 |V |, then OPT(H) ≤ |R| + 23 |S| − 1, and
• if OPT(G) > (1 + εv ) 23 |V |, then OPT(H) > (1 + εs )(|R| + 23 |S| − 1),
where εs = 4εv /97, and εv is the constant determined in Theorem 29.13.

Proof: Graph H = (R, S, cost) will be such that G has a vertex cover of size
c iﬀ H has a Steiner tree of cost |R| + c − 1. H will have a required vertex re
corresponding to each edge e ∈ E and a Steiner vertex sv corresponding to
each vertex v ∈ V . The edge costs are as follows. An edge between a pair of
Steiner vertices is of cost 1, and an edge between a pair of required vertices
is of cost 2. An edge (re , sv ) is of cost 1 if edge e is incident at vertex v in G,
and it is of cost 2 otherwise.
    Let us show that G has a vertex cover of size c iﬀ H has a Steiner tree of
cost |R| + c − 1. For the forward direction, let Sc be the set of Steiner vertices
in H corresponding to the c vertices in the cover. Observe that there is a tree
in H covering R ∪ Sc using cost 1 edges only (since every edge e ∈ E must
be incident at a vertex in the cover). This Steiner tree has cost |R| + c − 1.
    For the reverse direction, let T be a Steiner tree in H of cost |R| + c − 1.
We will show below that T can be transformed into a Steiner tree of the same
cost that uses edges of cost 1 only. If so, the latter tree must contain exactly c
Steiner vertices. Moreover, every required vertex of H must have a unit cost
edge to one of these Steiner vertices. Therefore, the corresponding c vertices
of G form a cover.
    Let (u, v) be an edge of cost 2 in T . We may assume w.l.o.g. that u
and v are both required. (If u is Steiner, remove (u, v) from T , getting two
components. Throw in an edge from v to a required vertex to connect the
two sides, and get a Steiner tree of the same cost as T .) Let eu and ev be
the edges, in G, corresponding to these vertices. Since G is connected, there
is a path, p, from one of the endpoints of eu to one of the endpoints of ev
in G. Now, removing (u, v) from T gives two connected components. Let the
318     29   Hardness of Approximation

set of required vertices in these two sets be R1 and R2 . Clearly, u and v
lie in diﬀerent sets, so path p must have two adjacent edges, say (a, b) and
(b, c) such that their corresponding vertices, say w and w , lie in R1 and R2 ,
respectively. Let the Steiner vertex, in H, corresponding to b be sb . Now,
throwing in the edges (sb , w) and (sb , w ) must connect the two components.
Observe that these two edges are of unit cost.
     Now, if OPT(G) ≤ 23 |V |, then OPT(H) > |R|+ 23 |S|−1, and if OPT(G) >
(1 + εv ) 23 |V |, then OPT(H) > |R| + (1 + εv ) 23 |S| − 1. The theorem follows. ✷
    The reduction is illustrated below. Graph G is an instance of the ver-
tex cover problem. The highlighted vertices form a cover. Graph H shows
the Steiner tree corresponding to this cover in the reduced graph. Required
vertices have been marked with squares, and the three Steiner vertices corre-
sponding to the cover have been marked with circles (the remaining Steiner
vertices have been omitted for clarity). The edge between two Steiner vertices
in the tree is dotted to distinguish it from the remaining edges, which connect
required and Steiner vertices.
   ✉                                           .✉.
                       
                                                 ..
                                                    ..
                                                     ..
                                                       ..                      
                                                         ..
                                                            ..                    ✉
                                 ✉                           ..
                                ✚✚                              ..                 ✚
                                                                                   ✚
                                                                  ..
                              ✚                                     ..           ✚
                            ✚                                         ..       ✚
                         ✚                                              ..
                                                                          .. ✚
                       ✉
                       ✚                                                    ✚
                                                                            ✉


               G                                             H


29.6 Hardness of clique
The best approximation algorithms known for some problems, including
clique, are extremely weak – to the extent that the solution produced by
the best known algorithm is only very slightly better than picking a trivial
feasible solution. Recent hardness results have been invaluable in explaining
why this is so: these problems are inherently inapproximable (essentially). In
this section, we will establish this for clique:
Problem 29.15 (Clique) Given an undirected graph G = (V, E) with
nonnegative weights on vertices, ﬁnd a clique of maximum weight. A clique
in G is a subset of vertices, S ⊆ V , such that for each pair u, v ∈ S, (u, v) ∈ E.
Its weight is the sum of weights of its vertices.
    Consider the cardinality version of this problem, i.e., when all vertex
weights are unit. In this section we will show that there is a constant εq > 0,
such that there is no 1/(nεq ) factor approximation algorithm for this problem,
assuming P = NP. Let us ﬁrst prove the following weaker result.
                                                 29.6   Hardness of clique     319

Lemma 29.16 For ﬁxed constants b and q, there is a gap-introducing reduc-
tion from SAT to clique that transforms a Boolean formula φ of size n to a
graph G = (V, E), where |V | = 2q nb , such that
• if φ is satisﬁable, OPT(G) ≥ nb , and
• if φ is not satisﬁable, OPT(G) < 12 nb .

Proof: Let F be a PCP(log n, 1) veriﬁer for SAT that requires b log n ran-
dom bits and queries q bits of the proof. We will transform a SAT instance,
φ, of size n to a graph G = (V, E) as follows. For each choice of a binary
string, r, of b log n bits, and each truth assignment, τ , to q Boolean variables,
there is a vertex vr,τ in G. Thus, |V | = 2q nb .
     Let Q(r) represent the q positions in the proof that F queries when it
is given string r as the “random” string. We will say that vertex vr,τ is
accepting if F accepts when it is given random string r and when it reads τ
in the Q(r) positions of the proof; it is rejecting otherwise. Vertices vr1 ,τ1 and
vr2 ,τ2 are consistent if τ1 and τ2 agree at each position at which Q(r1 ) and
Q(r2 ) overlap. Clearly, a necessary condition for consistency is that r1 = r2 .
Two distinct vertices vr1 ,τ1 and vr2 ,τ2 are connected by an edge in G iﬀ they
are consistent and they are both accepting. Vertex vr,τ is consistent with
proof p if positions Q(r) of p contain τ .
     If φ is satisﬁable, there is a proof, p, on which F accepts for each choice,
r, of the random string. For each r, let p(r) be the truth setting assigned by
proof p to positions Q(r). Now, the vertices {vr,p(r) | |r| = b log n} form a
clique in G of size nb .
     Next, suppose that φ is not satisﬁable, and let C be a clique in G. Since
the vertices of C are pairwise consistent, there is a proof, p, that is consistent
with all vertices of C. Therefore, the probability of acceptance of F on proof
p is at least |C|/nb (notice that the vertices of C must correspond to distinct
random strings). Since the probability of acceptance of any proof is < 1/2
the largest clique in G must be of size < 12 nb .                                 ✷
    As a consequence of Lemma 29.16, there is no factor 1/2 approximation
algorithm for clique assuming P = NP. Observe that the hardness factor
established is precisely the bound on the error probability of the probabilisti-
cally checkable proof for SAT. By the usual method of simulating the veriﬁer a
constant number of times, this can be made 1/k for any constant k, leading to
a similar hardness result for clique. In order to achieve the claimed hardness,
the error probability needs to be made inverse polynomial. This motivates
generalizing the deﬁnition of PCP as follows. Let us deﬁne two additional
parameters, c and s, called completeness and soundness, respectively. A lan-
guage L ∈ PCPc,s [r(n), q(n)] if there is a veriﬁer V , which on input x of
length n, obtains a random string of length O(r(n)), queries O(q(n)) bits of
the proof, and satisﬁes:
• if x ∈ L, there is a proof y that makes V accept with probability ≥ c,
320    29   Hardness of Approximation

       / L, then for every proof y, V accepts with probability < s.
• if x ∈
Thus, the previously deﬁned class PCP[r(n), q(n)] = PCP1, 12 [r(n), q(n)]. In
general, c and s may be functions of n.
    We would like to obtain a PCP characterization of NP which has inverse
polynomial soundness. An obvious way of reducing soundness is to simulate
a PCP[log n, 1] veriﬁer multiple number of times and accept iﬀ the veriﬁer
accepts each time. Simulating k times will reduce soundness to 1/2k ; how-
ever, this will increase the number of random bits needed to O(k log n) and
the number of query bits to O(k). Observe that the number of vertices in
the graph constructed in Lemma 29.16 is 2O(r(n)+q(n)) . To achieve inverse
polynomial soundness, k needs to be Ω(log n). For this value of k, the num-
ber of bits queried is O(log n), which is not a problem. However, the number
of random bits needed is O(log2 n), which leads to a superpolynomial sized
graph.
    The following clever idea overcomes this diﬃculty. We will use a constant
degree expander graph to generate O(log n) strings of b log n bits each, using
only O(log n) truly random bits. The veriﬁer will be simulated using these
O(log n) strings as the “random” strings. Clearly, these are not truly random
strings. Properties of expanders help show that they are “almost random” –
the probability of error still drops exponentially in the number of times the
veriﬁer is simulated.
    Let H be a constant degree expander on nb vertices, each vertex having
a unique b log n bit label. A random walk on H of length O(log n) can be
constructed using only O(log n) bits, b log n bits to pick the starting vertex
at random and a constant number of bits to pick each successive vertex.
(Observe that the random walk is started in the stationary distribution, which
is uniform since the graph is regular.) The precise property of expanders we
will need is the following.

Theorem 29.17 Let S be any set of vertices of H of size < (nb )/2. There
is a constant k such that
                                                                      1
      Pr[ all vertices of a k log n length random walk lie in S ] <     .
                                                                      n
    For intuitive justiﬁcation for Theorem 29.17, observe that a constant frac-
tion of the edges incident at vertices of S have their other end points in S –
these help the walk escape from S. The following ﬁgure shows a walk on H
that does not lie in S:
                                                   29.6   Hardness of clique   321




                                       S



Theorem 29.18             NP = PCP1, n1 [log n, log n]

Proof: We will prove the diﬃcult half,

      PCP1, 12 [log n, 1] ⊆ PCP1, n1 [log n, log n],

and leave the rest as Exercise 29.5. Let L ∈ PCP1, 12 [log n, 1]. Let F be a
veriﬁer for L which requires b log n random bits and queries q bits of the
proof, where b and q are constants.
    Next, we give a PCP1, n1 [log n, log n] veriﬁer for L, F  , which constructs
the expander graph H deﬁned above. It then constructs a random walk of
length k log n on H, using O(log n) random bits. Both constructions can
be accomplished in polynomial time. The label of each vertex on this path
speciﬁes a b log n bit string. It uses these k log n + 1 strings as the “random”
strings on which it simulates veriﬁer F . F  accepts iﬀ F accepts on all k log n+
1 runs.
    Consider string x ∈ L, and let p be a proof that makes veriﬁer F accept x
with probability 1. Clearly, F  , given proof p, also accepts x with probability
1. Hence the completeness of the new proof system is 1.
    Next, consider string x ∈/ L, and let p be an arbitrary proof supplied to F  .
When given proof p, veriﬁer F accepts on < (nb )/2 random strings of length
b log n. Let S denote the corresponding set of vertices of H, |S| < (nb )/2. Now,
F  accepts x iﬀ the random walk remains entirely in S. Since the probability
of this event is < 1/n, the soundness of F  is 1/n. Finally observe that F 
requires only O(log n) random bits and queries O(log n) bits of the proof. ✷
322      29   Hardness of Approximation

Theorem 29.19 For ﬁxed constants b and q, there is a gap-introducing re-
duction from SAT to clique that transforms a Boolean formula φ of size n to
a graph G = (V, E), where |V | = nb+q , such that
• if φ is satisﬁable, OPT(G) ≥ nb , and
• if φ is not satisﬁable, OPT(G) < nb−1 .

Proof: Let F be a PCP1, n1 [log n, log n] veriﬁer for SAT that requires b log n
random bits and queries q log n bits of the proof. The transformation of SAT
instance φ to graph G is exactly as in Lemma 29.16. The only diﬀerence
is that the increased number of bits queried results in a larger number of
vertices.
    The correctness of the construction also along the lines of Lemma 29.16.
If φ is satisﬁable, let p be a good proof, and pick the nb vertices of G that are
consistent with p, one for each choice of the random string. These vertices
will form a clique in G. Furthermore, any clique C in G gives rise to a proof
that is accepted by F with probability ≥ |C|/nb . Since the soundness of F is
1/n, if φ is not satisﬁable, the largest clique in G is of size < nb−1 .       ✷

Corollary 29.20 There is no 1/(nεq ) factor approximation algorithm for
the cardinality clique problem, assuming P = NP, where εq = 1/(b + q), for
constants b and q deﬁned in Theorem 29.19.



29.7 Hardness of set cover
As stated in Chapter 2, the simple greedy algorithm for the set cover problem,
which is perhaps the ﬁrst algorithmic idea one would attempt, has remained
essentially the best algorithm. Since set cover is perhaps the single most
important problem in the theory of approximation algorithms, a lot of eﬀort
was expended on obtaining an improved algorithm.
    In this section, we will present the remarkable result that the approxima-
tion factor of this algorithm is tight up to a constant multiplicative factor.
Improved hardness results show that it is tight up to lower order terms as
well (see Section 29.9). This should put to rest nagging doubts about the true
approximability of this central problem.

29.7.1    The two-prover one-round characterization of NP

Observe that for the purpose of showing hardness of MAX-3SAT and clique
(Theorems 29.7 and 29.19), we did not require a detailed description of the
kinds of queries made by the veriﬁer – we only required a bound on the
number of queries made. In contrast, this time we do need a description,
and moreover, we want to ﬁrst establish that a particularly simple veriﬁer
                                              29.7   Hardness of set cover    323

suﬃces. For this purpose, we will introduce a new model for probabilistically
checkable proofs, the two-prover one-round proof system. This model is best
understood by thinking of the proof system as a game between the prover
and the veriﬁer. The prover is trying to cheat – it is trying to convince the
veriﬁer that a “no” instance for language L is actually in L. Is there a veriﬁer
that can ensure that the probability of getting cheated is < 1/2 for every
“no” instance?
    In the two-prover model, the veriﬁer is allowed to query two non-
communicating provers, denoted P1 and P2 . Since the veriﬁer can cross-check
the provers’ answers, the provers’ ability to cheat gets restricted in this model.
In turn, we will impose restrictions on the veriﬁer as well, and thereby obtain
a new characterization of NP. Under a one-round proof system, the veriﬁer
is allowed only one round of communication with each prover. The simplest
way of formalizing this is as follows. We will assume that the two proofs are
written in two alphabets, say Σ1 and Σ2 . In general, the sizes of these al-
phabets may be unbounded and may depend on the size of the input. The
veriﬁer is allowed to query one position in each of the two proofs.
    The two-prover one-round model comes with three parameters: complete-
ness, soundness and the number of random bits provided to the veriﬁer, de-
noted by c, s and r(n), respectively. This deﬁnes the class 2P1Rc,s (r(n)). A
language L is in 2P1Rc,s (r(n)) if there is a polynomial time bounded veriﬁer
V that receives O(r(n)) truly random bits and satisﬁes:
• for every input x ∈ L, there is a pair of proofs y1 ∈ Σ1∗ and y2 ∈ Σ2∗ that
  makes V accept with probability ≥ c,
                    / L and every pair of proofs y1 ∈ Σ1∗ and y2 ∈ Σ2∗ , V
• for every input x ∈
  accepts with probability < s.
    The PCP theorem implies, and in fact is equivalent to, the fact that there
is a gap-introducing reduction from SAT to MAX-3SAT(5) (see Theorem
29.7 and Exercises 29.3 and 29.4). We will use this to show:

Theorem 29.21 There is a constant εP > 0 such that
NP = 2P1R1,1−εP (log(n)).

Proof: We will establish the diﬃcult half, i.e., NP ⊆ 2P1R1,1−εP (log(n)),
and leave the rest as Exercise 29.7. Clearly, it is suﬃcient to show that SAT
∈ 2P1R1,1−εP (log(n)).
   As a result of Theorem 29.7 and Exercise 29.4, there is gap-introducing
reduction from SAT to MAX-3SAT(5)2 . More precisely, there is a constant
ε5 > 0 for which there is a reduction Γ from SAT to MAX-3SAT(5) that
transforms a Boolean formula φ to ψ such that
• if φ is satisﬁable, OPT(ψ) = m, and
2
    The bounded occurrence version of MAX-3SAT is not essential for this theorem;
    however, we will require it in the main reduction.
324      29   Hardness of Approximation

• if φ is not satisﬁable, OPT(ψ) < (1 − ε5 )m,
where m is the number of clauses in ψ.
    The two-prover one-round veriﬁer, V , for SAT works as follows. Given a
SAT formula φ, it uses the above stated reduction to obtain a MAX-3SAT(5)
instance ψ. It assumes that P1 contains an optimal truth assignment, τ , for ψ
and P2 contains, for each clause, the assignment to its three Boolean variables
under τ (hence, |Σ1 | = 2 and |Σ2 | = 23 ). It uses the O(log n) random bits to
pick a random clause, C, from ψ, and further, a random Boolean variable, x,
occurring in C. V obtains the truth assignments to x and the three variables
in C by querying P1 and P2 , respectively. It accepts iﬀ C is satisﬁed and the
two proofs agree on their assignment for x.
    If φ is satisﬁable, then so is ψ. Clearly, there are proofs y1 and y2 such
that V accepts with probability 1.
    Next assume that φ is not satisﬁable. Any truth assignment to ψ must
leave strictly more than ε5 fraction of the clauses unsatisﬁed. Consider any
pair of proofs (y1 , y2 ). Interpret y1 as a truth assignment, say τ . The random
clause, C, picked by V is not satisﬁed by τ with probability > ε5 . If so, and
if the assignment for C contained in y2 is satisfying, then y1 and y2 must
be inconsistent. In the latter case, the veriﬁer catches this with probability
≥ 1/3. Hence overall, V must reject with probability > ε5 /3.                   ✷

Remark 29.22 Using standard techniques (see Exercise 29.8), Γ can be
modiﬁed to ensure that the instance of MAX-3SAT(5) produced satisﬁes the
following uniformity conditions: each Boolean variable occurs in exactly 5
clauses and each clause contains 3 distinct variables (negated or unnegated).
This modiﬁcation changes the constant ε5 to some other constant, say ε5 > 0.
These uniformity conditions will be needed in the main reduction.
Remark 29.23 As a result of the uniformity conditions, if ψ has n variables,
then it has 5n/3 clauses. Therefore, the two proofs are of length n and 5n/3,
respectively. For carrying out the main reduction, it will be important to
ensure that the two proofs are of equal length. This can be easily achieved by
repeating the ﬁrst proof 5 times and the second proof 3 times. The veriﬁer
will query a random copy of each proof. It is easy to verify that Theorem
29.21 still holds (even though the “copies” may be diﬀerent).

29.7.2    The gadget

The following set system will be a basic gadget in the main reduction:
(U, C1 , . . . , Cm , C 1 , . . . , C m ), where U is the universal set and C1 , . . . , Cm
are subsets of U . Clearly, U can be covered by picking a set Ci and its com-
plement C i . Such a cover will be called a good cover. A cover that does not
include a set and its complement will be called a bad cover. The following
theorem, which can be proven using the probabilistic method (see Exercise
29.9), shows the existence of such set systems for which the sizes of good and
                                                   29.7   Hardness of set cover   325

bad covers are widely diﬀerent. Moreover, they can be constructed eﬃciently,
with high probability.
Theorem 29.24 There exists a polynomial p(., .) such that there is a ran-
domized algorithm which generates, for each m and l, a set system

      (U, C1 , . . . , Cm , C 1 , . . . , C m ),

with |U | = p(m, 2l ). With probability > 1/2 the gadget produced satisﬁes that
every bad cover is of size > l. Moreover, the running time of the algorithm
is polynomial in |U |.
    A good cover is well coordinated – it involves picking a set Ci and its
complement. Acceptance in the two-prover one-round proof system also in-
volves coordination – on random string r, the veriﬁer queries the two proofs
and accepts iﬀ the answers are coordinated. The choice of this proof system,
for establishing hardness of set cover, should be more convincing in light of
this observation.

29.7.3    Reducing error probability by parallel repetition

Before presenting the reduction, we would like improve the soundness of the
two-prover one-round proof system for SAT. The usual way of accomplishing
this is parallel repetition: The veriﬁer picks k clauses randomly and indepen-
dently, and a random Boolean variable from each of the clauses. It queries
P1 on the k variables and P2 on the k clauses, and accepts iﬀ all answers
are accepting. One would expect that probability that the provers manage to
cheat drops to < (1 − εP )k .
    Surprisingly enough, this is not true. Since each prover is allowed to look
at all k questions before providing its k answers, it may be able to coordi-
nate its answers and thereby cheat with a higher probability. Example 29.25
illustrates this in a simple setting. If the provers are required to answer each
question before being given the next question, the probability of error drops
in the usual fashion; however, this requires k rounds of communication and
falls outside the two-prover one-round model.
Example 29.25 Consider the following setting in which the two non-
communicating provers are attempting to agree on a random bit. The veriﬁer
gives random, independent bits r1 and r2 to P1 and P2 , respectively. The pro-
tocol succeeds if the two provers manage to commit to one of the two bits,
i.e., either both provers output (1, r1 ) or both provers output (2, r2 ); the ﬁrst
element of a pair says whose bit the provers are outputting and the second
element is the bit itself. Since P1 does not know r2 and P2 does not know r1 ,
the probability of their succeeding is 1/2.
     Now consider parallel repetitions of this protocol. The veriﬁer gives two
bits, r1 and s1 , to P1 and two bits, r2 and s2 , to P2 . The four bits are random
326      29   Hardness of Approximation

and independent. The provers succeed iﬀ they can commit to one of the r’s
and one of the s’s.
    One would expect the probability of success to be 1/4. However, by clev-
erly coordinating answers, the provers can make it 1/2 as follows. The answers
of P1 are (1, r1 ) and (2, r1 ), and those of P2 are (1, s2 ) and (2, s2 ). The provers
succeed iﬀ r1 = s2 , which happens with probability 1/2.                             ✷
   Despite this diﬃculty, one can still prove that the probability of error does
drop exponentially with k. However, the proof of this useful fact is not easy.
Theorem 29.26 Let the error probability of a two-prover one-round proof
system be δ < 1. Then the error probability on k parallel repetitions is at most
δ dk , where d is a constant that depends only on the length of the answers of
the original proof system.


29.7.4    The reduction

We will prove the following.
Theorem 29.27 There is a constant c > 0 for which there is a random-
ized gap-introducing reduction Γ , requiring time nO(log log n) , from SAT to
the cardinality set cover problem that transforms a Boolean formula φ to a
set system S over a universal set of size nO(log log n) such that
• if φ is satisﬁable, OPT(S) = 2nk , and
• if φ is not satisﬁable, Pr[OPT(G) > cnk k log n] > 1/2,
where n is the length of each of the two proofs for SAT under the two-prover
one-round model (see Remark 29.23); n is polynomial in the size of φ. The
parameter k is O(log log n).

Remark 29.28 This is slight abuse of notation, since gap-introducing re-
ductions were deﬁned to run in polynomial time.
Proof: Let V be the two-prover one-round veriﬁer for SAT, described in
Theorem 29.21. Assume further that the MAX-3SAT(5) formula produced
by V satisﬁes the uniformity conditions stated in Remark 29.22 and that
the two proofs queried by V are of equal length, say n, as stated in Remark
29.23. Denote by ψ the MAX-3SAT(5) formula produced by V when given
SAT formula φ.
    Let V  be a two-prover one-round veriﬁer that executes k parallel repeti-
tions of V , as described in Section 29.7.3. Now, each of the proofs is of length
nk . Each position of P1 contains a truth assignment to k Boolean variables
(not necessarily distinct) and each position of P2 contains a truth assignment
to the 3k Boolean variables occurring in k clauses. Thus, proofs P1 and P2
are written in alphabets Σ1 and Σ2 whose sizes are 2k and 23k , respectively.
k will be ﬁxed to be O(log log n) for reasons clariﬁed below.
                                                    29.7    Hardness of set cover        327

    Veriﬁer V  uses random bits provided to it to pick k random clauses of
ψ, and a random Boolean variable from each of these k clauses, thereby
specifying a position in P1 and a position in P2 . These involve picking from
one of nk and 3k choices, respectively. Therefore, the total number of random
strings is (3n)k . Denote by Q1 (r) and Q2 (r) the positions in P1 and P2 ,
respectively, speciﬁed by random string r.
    Suppose the answers in positions Q1 (r) and Q2 (r) are a and b, respec-
tively. Recall that V  accepts on random string r iﬀ b satisﬁes all k clauses
picked, and a and b assign the same truth values to the k chosen variables.
Given r and the answer in Q2 (r), say b, the “acceptable” answer in Q1 (r) is
uniquely speciﬁed. Let projection function π(r, b) denote this answer.
                                                             nk positions
                                                      ✛                          ✲
                                                                                       ✻

                 nk positions
         ✛                           ✲
                                                                          b             3k bits
         ✻
k bits                  a

         ❄                                                                             ❄
                        i = Q1 (r)                                        j = Q2 (r)
                       P1                                            P2

     The parameters m and l for the gadget are ﬁxed as follows. We will set
m = |Σ1 | = 2k , and l = O(k log n) = O(log n log log n). Let (U, C1 , . . . , C2k ,
C 1 , . . . , C 2k ) be the gadget with these parameters. Thus, corresponding to
each answer a ∈ Σ1 , we have a unique set Ca . As stated in Theorem 29.24,
|U | = p(2k , 2l ) = nO(log log n) , and the gadget can be constructed probabilis-
tically in time polynomial in |U |.
     The gadget will be constructed once, and as stated in Theorem 29.24,
will satisfy the chosen parameters with probability > 1/2. For the rest of
the proof, assume that it does. We will make (3n)k copies of the gadget over
disjoint universal sets. Each copy corresponds to a random string. Denote the
                                                                          r             r
copy corresponding to random string r to be (U r , C1r , . . . , C2rk , C 1 , . . . , C 2k ).
     The reduction Γ transforms φ to a set cover instance S as follows. The
universal set
                 2
          U=         U r,
             r

where the union is over all (3n)k random strings. Clearly, |U| = |U |(3n)k =
nO(log log n) . The subsets of U speciﬁed by S are of two kinds. First, corre-
sponding to each position i in P1 and answer a ∈ Σ1 , there is a set
                  2
      Si,a =           Car ,
                 r:Q1 (r)=i
328     29   Hardness of Approximation

where the union is over all random strings r such that Q1 (r) = i. Second,
corresponding to each position j in P2 and answer b ∈ Σ2 , there is a set
Sj,b . If b does not satisfy all k clauses of ψ, speciﬁed by position Q2 (r), then
Sj,b = ∅. Otherwise,
                  2           r
      Sj,b =                C π(r,b) ,
               r:Q2 (r)=j


where the union is over all random strings r such that Q2 (r) = j.
   Let r be a random string, and let Q1 (r) = i and Q2 (r) = j. Then, the
only sets in S that contain elements of U r are:
• Si,a , for a ∈ Σ1 , and
• Sj,b , for b ∈ Σ2 such that b satisﬁes the k clauses speciﬁed by position j in
  P2 .
Moreover, each set of the ﬁrst type contains exactly one set from C1r , . . . , C2rk
                                                                          r         r
and each set of the second type contains exactly one set from C 1 , . . . , C 2k .
     Let r be a random string, and let Q1 (r) = i and Q2 (r) = j. Observe that
Si,a ∪ Sj,b covers U r iﬀ π(r, b) = a and b satisﬁes the k clauses speciﬁed by
position j in P2 . Let C be a cover for U. If C contains such a pair of sets then
we will say that C contains a good cover for U r . If C does not contain a good
cover for U r , then it must contain > l sets of the form Si,a , Sj,b , a ∈ Σ1 , b ∈ Σ2
in order to cover U r . In this case, we will say that C contains a bad cover for
U r.
     Suppose φ is satisﬁable. Then there is a pair of proofs (y1 , y2 ) on which
the veriﬁer accepts with probability 1. Let us pick a cover C as follows. Cor-
responding to each position i in P1 and j in P2 pick sets Si,a and Sj,b , where
a and b are the answers for these queries in y1 and y2 , respectively. Hence,
|C| = 2nk . It is easy to see that C contains a good cover for each set U r .
     Next suppose that φ is not satisﬁable. Now, V  will reject any pair of
proofs with high probability. We have assumed that the gadget found satisﬁes
the chosen parameters; this happens with probability > 1/2. Let C denote
an optimal cover for U. Is C forced to contain a bad cover for U r , for most
random strings r? Clearly, C is allowed to pick sets corresponding to portions
of many diﬀerent proofs. Using this added capability, can we not construct a
cover that is only slightly larger than 2nk ? A set from S helps cover elements
from several diﬀerent universes U r , making the rest of the argument more
involved.
     Below we will give a procedure for constructing, from C, a pair of proofs,
(y1 , y2 ), in such a way that if |C| is small, then V  must accept this pair with
high probability. Hence, we will derive the desired lower bound on |C|.
     Consider the set of answers picked by C for each position of the two
proofs. For each position i in P1 , deﬁne A(i) = {a | Si,a ∈ C}, and for
each position j in P2 , deﬁne A(j) = {b | Sj,b ∈ C}. Construct proofs y1
                                                           29.8   Exercises    329

and y2 by picking for each position i in P1 and j in P2 a random element
of A(i) and A(j), respectively. If any of the answer sets is empty, pick an
arbitrary answer for that position. Deﬁne B1 = {r | |A(Q1 (r))| > l/2},
B2 = {r | |A(Q2 (r))| > l/2} and G = B1 ∪ B2 .
     Thus, G is the set of random strings r for which C picks at most l/2
answers each for Q1 (r) and Q2 (r). Hence, C contains a good cover for U r ,
say Si,a ∪ Sj,b , where a ∈ A(Q1 (r)) and b ∈ A(Q2 (r)). The pair of proofs,
(y1 , y2 ), contain a and b in positions Q1 (r) and Q2 (r), respectively, with
probability ≥ ( 2l )2 . Hence V  , when given proofs (y1 , y2 ), accepts on random
string r with at least this probability.
     Let fG denote the fraction of random strings contained in G. Then, using
Theorem 29.26,
          2
          2
      fG      ≤ Pr[V  accepts φ when given proofs (y1 , y2 )] ≤ δ dk .
          l

Hence, fG ≤ δ dk l2 /4. Since l2 is O(log4 n), by picking k = O(log log n) we
can ensure that fG < 1/2. As a result, B1 ∪ B2 contains at least half the
random strings, and therefore one of these sets contains at least a quarter.
Denote this set by Bi .
    Because of the uniformity property (Remark 29.22), if r is chosen at
random, then Q1 (r) is a random position in P1 and Q2 (r) is a random position
in P2 (although they will be correlated). Furthermore, r has probability > 1/4
of being in Bi . Therefore, the answer sets of > 1/4 of the positions of Bi are
of cardinality > l/2. Hence the size of the cover > lnk /8 = Ω(nk k log n). ✷
   As a consequence of Theorem 29.27, inapproximability of set cover mod-
ulo NP not being in a one-sided-error complexity class with running time
nO(log log n) follows directly. Standard techniques from complexity theory (see
Exercise 1.18) lead to the following slightly stronger result.
Corollary 29.29 There is a constant b such that if there is a b log n fac-
tor approximation algorithm for the cardinality set cover problem, where
n is the size of the universal set of the set cover instance, then NP ⊆
ZTIME(nO(log log n) ) (see Section A.4 for deﬁnition).



29.8 Exercises

29.1 Show that PCP(log n, 1) ⊆ NP.
Hint: Let L ∈ PCP(log n, 1). The NP machine for accepting L guesses the
proof, simulates the veriﬁer for L on all O(log n) length random strings, and
accepts iﬀ the veriﬁer accepts on all the random strings.
330    29   Hardness of Approximation

29.2 Show (see Appendix A for deﬁnitions):
 1. PCP(0, 0) = PCP(0, log n) = P.
 2. PCP(poly(n), 0) = co-RP, where poly(n) = k≥0 nk .
 3. PCP(log n, 1) = PCP(log n, poly).
    Hint: NP ⊆ PCP(log n, 1) ⊆ PCP(log n, poly) ⊆ NP.

29.3 Show the converse of Theorem 29.7, i.e., if there is a gap-introducing
reduction from SAT to MAX-3SAT, then NP ⊆ PCP(log n, 1).
Hint: Reduce the given SAT formula φ to an instance ψ of MAX-3SAT.
The veriﬁer expects, as proof, an optimal truth assignment to ψ. This gives
an error probability of 1 − εM . Repeat to decrease the error probability to
< 1/2.

29.4 Give a gap-preserving reduction from MAX-3SAT(29) to MAX-
3SAT(5), with appropriate parameters, to show hardness for the latter prob-
lem.
Hint: The reduction is similar, though easier, than that in Theorem 29.11.
Instead of using an expander graph, use a cycle. Now, an inconsistent assign-
ment can gain as many as 14 clauses corresponding to each old variable x.
However, it must leave at least two clauses, corresponding to edges of the
cycle of x, unsatisﬁed.

29.5 Complete the proof of Theorem 29.18.

29.6 (Hastad [122]) An important consideration, while obtaining a PCP
characterization of NP, is reducing the number of bits of the proof that the
veriﬁer needs to query. The following remarkable result reduces it to just 3.
Theorem 29.30 For every ε > 0,

      NP = PCP1−ε, 12 +ε [log n, 1].

Moreover, there is a particularly simple PCP veriﬁer for SAT. It uses the
O(log n) random bits to compute three positions in the proof, say i, j and k,
and a bit b, and accepts iﬀ

      y(i) + y(j) + y(k) ≡ b   (mod 2).

Here y(i) is the ith bit in the proof y.
 1. Consider the restriction of Problem 16.12 (Exercise 16.7), linear equa-
    tions over GF[2], in which each equation has exactly 3 variables. Use
    the characterization stated in Theorem 29.30 to give an appropriate gap-
    introducing reduction from SAT to this problem which shows that if, for
    any ε > 0, there is a 2 − ε factor approximation algorithm for the latter
    problem then P = NP.
                                                           29.8   Exercises        331

 2. Give an appropriate gap-preserving reduction from linear equations over
    GF[2] to MAX-3SAT which shows that if, for any ε > 0, there is a 8/7 − ε
    factor approximation algorithm for MAX-3SAT then P = NP.
    Hint: The equation xi + xj + xk ≡ 0 (mod 2) is transformed into the
    clauses

          (xi ∨ xj ∨ xk ) ∧ (xi ∨ xj ∨ xk ) ∧ (xi ∨ xj ∨ xk ) ∧ (xi ∨ xj ∨ xk ).


29.7 Complete the proof of Theorem 29.21, i.e., show that
2P1R1,1−εP (log(n)) ⊆ NP.

29.8 Prove the uniformity conditions stated in Remark 29.22.
Hint: Use the standard technique of introducing new Boolean variables.

29.9 Prove Theorem 29.24 using the probabilistic method.
Hint: p(m, 2l ) = O(m22l ) suﬃces. Pick each set Ci by including each element
of U in it randomly and independently with probability 1/2.

29.10 (Feige [80]) The following stronger hardness result can be established
for set cover:
Theorem 29.31 For any constant δ > 0, if there is a (1 − δ) ln n fac-
tor approximation algorithm for the cardinality set cover problem, where
n is the size of the universal set of the set cover instance, then NP ⊆
DTIME(nO(log log n) ), where DTIME(t) is the class of problems for which
there is a deterministic algorithm running in time O(t).
    Consider the maximum coverage problem, Problem 2.18 in Exercise 2.15.
Using Theorem 29.31 show that if there is an ε > 0 for which there is a
(1 − 1/e + ε) factor approximation algorithm for the maximum coverage
problem, then NP ⊆ DTIME(nO(log log n) ).
Hint: Use the maximum coverage algorithm to obtain a (1 − δ) ln n factor
algorithm for set cover, for some δ > 0, as follows: Guess k, the optimal
number of sets needed for the given instance. Run the maximum coverage
algorithm, with parameter k, iteratively, until a cover is found. In each itera-
tion, a (1 − 1/e + ε) fraction of the uncovered elements is covered. Therefore,
the number of iterations, l, satisﬁes, (1/e − ε)l = 1/n.

29.11 (Jain, Mahdian, and Saberi [138]) Using Theorem 29.31 show that
if there is an ε > 0 for which there is a (1 + 2/e − ε) factor approx-
imation algorithm for the metric k-median problem, Problem 25.1, then
NP ⊆ DTIME(nO(log log n) ).
332    29   Hardness of Approximation

29.9 Notes
The ﬁrst hardness of approximation result based on probabilistically check-
able proofs was due to Feige, Goldwasser, Lovász, Safra, and Szegedy [82].
This work motivated the discovery of the PCP theorem, which additionally
builds on work on interactive proof systems (Babai [18] and Goldwasser, Mi-
cali, and Rackoﬀ [109]) and program checking (Blum and Kannan [28] and
Blum, Luby, and Rubinfeld [29]), and is due to Arora and Safra [14], and
Arora, Lund, Motwani, Sudan, and Szegedy [14, 12]. Theorem 29.30, which
yields optimal inapproximability results for several problems, is due to Hastad
[122].
    Before this development, the pioneering work of Papadimitriou and Yan-
nakakis [218] had established evidence of inapproximability of several natural
problems using their notion of Max-SNP-completeness. Gap preserving re-
ductions are weaker than their L-reductions. Consequently, the ideas behind
their reductions carry over directly to the new development, as in the reduc-
tions given in Theorems 29.11 and 29.13. Indeed, one of the motivations for
the PCP theorem was that establishing an inapproximability result for MAX
SAT would directly yield inapproximability results for all Max-SNP-hard
problems. Theorem 29.14 is from Bern and Plassmann [25].
    The construction of expander graphs is due to Lubotzky, Phillips, and
Sarnak [197]. Theorem 29.17 is due to Impagliazzo and Zuckerman [135].
Theorem 29.19 on hardness of clique follows from [82] and [14, 12]. The cur-
rent best hardness result for clique, due to Hastad [121], states that it cannot
be approximated within a factor of n1−ε for any ε > 0, unless NP = ZPP.
This is quite close to the best approximation algorithm, due to Boppana and
Holldórsson [30], achieving a guarantee of O(n/(log2 n)).
    Lund and Yannakakis [199] gave the ﬁrst hardness result for set cover,
showing that it cannot be approximated within a factor of log n/2 unless
NP ⊆ ZTIME(nO(polylog n) ). The improved result, presented in Theorem
29.31, is due to Feige [80]. This enhancement comes about by using a k prover
proof system. A deterministic construction of the set system gadget of The-
orem 29.24, due to Naor, Schulman, and Srinivasan [211], allows replacing
ZTIME by DTIME in the complexity assumption. The two-prover one-
round proof system was deﬁned by Ben-or, Goldwasser, Kilian, and Wigder-
son [24]. Theorem 29.26 is due to Raz [231].
    Karloﬀ and Zwick [162] give an algorithm for MAX-3SAT that achieves
an approximation guarantee of 8/7 when restricted to satisﬁable formulae.
This complements the hardness result stated in Exercise 29.6.
    For further information on this topic, see the survey by Arora and Lund
[11]. For an up-to-date status of the best positive and negative results known
for numerous NP-hard optimization problems, see the excellent compendium
maintained online at
http://www.nada.kth.se/˜viggo/problemlist/compendium.html
                                                     29.9   Notes   333

The compendium also appears in Ausiello, Crescenzi, Gambosi, Kann, Mar-
chetti-Spaccamela, and Protasi [17].
