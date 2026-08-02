---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-08"
chapter_number: 8
chapter_title: "Knapsack"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 86
source_page_end: 91
printed_page_start: 68
printed_page_end: 73
part_ids: ["approximation-algorithms-ch-08-part-009"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Knapsack

8 Knapsack




In Chapter 1 we mentioned that some NP-hard optimization problems allow
approximability to any required degree. In this chapter, we will formalize this
notion and will show that the knapsack problem admits such an approxima-
bility.
    Let Π be an NP-hard optimization problem with objective function fΠ .
We will say that algorithm A is an approximation scheme for Π if on input
(I, ε), where I is an instance of Π and ε > 0 is an error parameter, it outputs
a solution s such that:
• fΠ (I, s) ≤ (1 + ε) · OPT if Π is a minimization problem.
• fΠ (I, s) ≥ (1 − ε) · OPT if Π is a maximization problem.
A will be said to be a polynomial time approximation scheme, abbreviated
PTAS, if for each ﬁxed ε > 0, its running time is bounded by a polynomial
in the size of instance I.
    The deﬁnition given above allows the running time of A to depend arbi-
trarily on ε. This is rectiﬁed in the following more stringent notion of approx-
imability. If the previous deﬁnition is modiﬁed to require that the running
time of A be bounded by a polynomial in the size of instance I and 1/ε, then
A will be said to be a fully polynomial approximation scheme, abbreviated
FPTAS.
    In a very technical sense, an FPTAS is the best one can hope for an NP-
hard optimization problem, assuming P = NP; see Section 8.3.1 for a short
discussion on this issue. The knapsack problem admits an FPTAS.
Problem 8.1 (Knapsack) Given a set S = {a1 , . . . , an } of objects, with
speciﬁed sizes and proﬁts, size(ai ) ∈ Z+ and proﬁt(ai ) ∈ Z+ , and a “knapsack
capacity” B ∈ Z+ , ﬁnd a subset of objects whose total size is bounded by B
and total proﬁt is maximized.
    An obvious algorithm for this problem is to sort the objects by decreasing
ratio of proﬁt to size, and then greedily pick objects in this order. It is easy
to see that as such this algorithm can be made to perform arbitrarily badly
(Exercise 8.1).
                                              8.2   An FPTAS for knapsack           69

8.1 A pseudo-polynomial time algorithm for knapsack
Before presenting an FPTAS for knapsack, we need one more concept. For
any optimization problem Π, an instance consists of objects, such as sets or
graphs, and numbers, such as cost, proﬁt, size, etc. So far, we have assumed
that all numbers occurring in a problem instance I are written in binary. The
size of the instance, denoted |I|, was deﬁned as the number of bits needed
to write I under this assumption. Let us say that Iu will denote instance I
with all numbers occurring in it written in unary. The unary size of instance
I, denoted |Iu |, is deﬁned as the number of bits needed to write Iu .
    An algorithm for problem Π is said to be eﬃcient if its running time on
instance I is bounded by a polynomial in |I|. Let us consider the following
weaker deﬁnition. An algorithm for problem Π whose running time on in-
stance I is bounded by a polynomial in |Iu | will be called a pseudo-polynomial
time algorithm.
    The knapsack problem, being NP-hard, does not admit a polynomial
time algorithm; however, it does admit a pseudo-polynomial time algorithm.
This fact is used critically in obtaining an FPTAS for it. All known pseudo-
polynomial time algorithms for NP-hard problems are based on dynamic
programming.
    Let P be the proﬁt of the most proﬁtable object, i.e., P = maxa∈S proﬁt(a).
Then nP is a trivial upperbound on the proﬁt that can be achieved by any
solution. For each i ∈ {1, . . . , n} and p ∈ {1, . . . , nP }, let Si,p denote a subset
of {a1 , . . . , ai } whose total proﬁt is exactly p and whose total size is mini-
mized. Let A(i, p) denote the size of the set Si,p (A(i, p) = ∞ if no such set
exists). Clearly A(1, p) is known for every p ∈ {1, . . . , nP }. The following
recurrence helps compute all values A(i, p) in O(n2 P ) time:

       A(i + 1, p) =
          min {A(i, p), size(ai+1 ) + A(i, p − proﬁt(ai+1 ))} if proﬁt(ai+1 ) < p
          A(i + 1, p) = A(i, p)                               otherwise

   The maximum proﬁt achievable by objects of total size bounded by B is
max {p| A(n, p) ≤ B}. We thus get a pseudo-polynomial algorithm for knap-
sack.


8.2 An FPTAS for knapsack
Notice that if the proﬁts of objects were small numbers, i.e., they were
bounded by a polynomial in n, then this would be a regular polynomial
time algorithm, since its running time would be bounded by a polynomial in
|I|. The key idea behind obtaining an FPTAS is to exploit precisely this fact:
we will ignore a certain number of least signiﬁcant bits of proﬁts of objects
70       8   Knapsack

(depending on the error parameter ε), so that the modiﬁed proﬁts can be
viewed as numbers bounded by a polynomial in n and 1/ε. This will enable
us to ﬁnd a solution whose proﬁt is at least (1 − ε) · OPT in time bounded
by a polynomial in n and 1/ε.


 Algorithm 8.2 (FPTAS for knapsack)
     1. Given ε > 0, let K = εP
                              n .                             
                                                    proﬁt(ai )
     2. For each object ai , deﬁne proﬁt (ai ) =      K        .
     3. With these as proﬁts of objects, using the dynamic programming
        algorithm, ﬁnd the most proﬁtable set, say S  .
     4. Output S  .



Lemma 8.3 Let A denote the set output by the algorithm. Then,

        proﬁt(A) ≥ (1 − ε) · OPT.

Proof: Let O denote the optimal set. For any object a, because of rounding
down, K · proﬁt (a) can be smaller than proﬁt(a), but by not more than K.
Therefore,

        proﬁt(O) − K · proﬁt (O) ≤ nK.

The dynamic programming step must return a set at least as good as O under
the new proﬁts. Therefore,

        proﬁt(S  ) ≥ K · proﬁt (O) ≥ proﬁt(O) − nK = OPT − εP
                  ≥ (1 − ε) · OPT ,

where the last inequality follows from the observation that OPT ≥ P .      ✷

Theorem 8.4 Algorithm 8.2 is a fully polynomial approximation scheme for
knapsack.

Proof: By Lemma 8.3, the solution found iswithin                  of OPT.
                                                  P (1 − ε)factor
Since the running time of the algorithm is O n2 K        = O n2 nε , which
is polynomial in n and 1/ε, the theorem follows.                           ✷
                 8.3   Strong NP-hardness and the existence of FPTAS’s       71

8.3 Strong NP-hardness and the existence of FPTAS’s
In this section, we will prove in a formal sense that very few of the known NP-
hard problems admit an FPTAS. First, here is a strengthening of the notion
of NP-hardness in a similar sense in which a pseudo-polynomial algorithm is
a weakening of the notion of an eﬃcient algorithm. A problem Π is strongly
NP-hard if every problem in NP can be polynomially reduced to Π in such
a way that numbers in the reduced instance are always written in unary.
    The restriction automatically forces the transducer to use polynomially
bounded numbers only. Most known NP-hard problems are in fact strongly
NP-hard; this includes all the problems in previous chapters for which ap-
proximation algorithms were obtained. A strongly NP-hard problem cannot
have a pseudo-polynomial time algorithm, assuming P = NP (Exercise 8.4).
Thus, knapsack is not strongly NP-hard, assuming P = NP.
    We will show below that under some very weak restrictions, any NP-
hard problem admitting an FPTAS must admit a pseudo-polynomial time
algorithm. Theorem 8.5 is proven for a minimization problem; a similar proof
holds for a maximization problem.
Theorem 8.5 Let p be a polynomial and Π be an NP-hard minimization
problem such that the objective function fΠ is integer valued and on any
instance I, OPT(I) < p(|Iu |). If Π admits an FPTAS, then it also admits a
pseudo-polynomial time algorithm.

Proof: Suppose there is an FPTAS for Π whose running time on instance
I and error parameter ε is q(|I|, 1/ε), where q is a polynomial.
   On instance I, set the error parameter to ε = 1/p(|Iu |), and run the
FPTAS. Now, the solution produced will have objective function value less
than or equal to:

      (1 + ε)OPT(I) < OPT(I) + εp(|Iu |) = OPT(I) + 1.

In fact, with this error parameter, the FPTAS will be forced to produce an
optimal solution. The running time will be q(|I|, p(|Iu |)), i.e., polynomial in
|Iu |. Therefore, we have obtained a pseudo-polynomial time algorithm for Π.
✷
   The following corollary applies to most known NP-hard problems.
Corollary 8.6 Let Π be an NP-hard optimization problem satisfying the
restrictions of Theorem 8.5. If Π is strongly NP-hard, then Π does not admit
an FPTAS, assuming P = NP.

Proof: If Π admits an FPTAS, then it admits a pseudo-polynomial time
algorithm by Theorem 8.5. But then it is not strongly NP-hard, assuming
P = NP, leading to a contradiction.                                  ✷
72       8   Knapsack

    The stronger assumption that OPT < p(|I|) in Theorem 8.5 would have
enabled us to prove that there is a polynomial time algorithm for Π. However,
this stronger assumption is less widely applicable. For instance, it is not
satisﬁed by the minimum makespan problem, which we will study in Chapter
10.

8.3.1    Is an FPTAS the most desirable approximation algorithm?

The design of almost all known FPTAS’s and PTAS’s is based on the idea
of trading accuracy for running time – the given problem instance is mapped
to a coarser instance, depending on the error parameter ε, which is solved
optimally by a dynamic programming approach. The latter ends up being an
exhaustive search of polynomially many diﬀerent possibilities (for instance,
for knapsack, this involves computing A(i, p) for all i and p). In most such
algorithms, the running time is prohibitive even for reasonable n and ε. Fur-
ther, if the algorithm had to resort to exhaustive search, does the problem
really oﬀer “footholds” to home in on a solution eﬃciently? Is an FPTAS or
PTAS the best one can hope for for an NP-hard problem? Clearly, the issue
is complex and there is no straightforward answer.


8.4 Exercises

8.1 Consider the greedy algorithm for the knapsack problem. Sort the objects
by decreasing ratio of proﬁt to size, and then greedily pick objects in this
order. Show that this algorithm can be made to perform arbitrarily badly.

8.2 Consider the following modiﬁcation to the algorithm given in Exercise
8.1. Let the sorted order of objects be a1 , . . . , an . Find the lowest k such that
the size of the ﬁrst k objects exceeds B. Now, pick the more proﬁtable of
{a1 , . . . , ak−1 } and {ak } (we have assumed that the size of each object is at
most B). Show that this algorithm achieves an approximation factor of 2.

8.3 (Bazgan, Santha, and Tuza [22]) Obtain an FPTAS for the following
problem.
Problem 8.7 (Subset-sum ratio problem) Given n positive integers,
a
1 < ... < an , ﬁnd two disjoint nonempty subsets S1 , S2 ⊆ {1, . . . , n} with
            
  i∈S1 ai ≥   i∈S2 ai , such that the ratio
        
              ai
        i∈S1
         i∈S2 ai

is minimized.
                                                        8.5   Notes   73

Hint: First, obtain a pseudo-polynomial time algorithm for this problem.
Then, scale and round appropriately.

8.4 Show that a strongly NP-hard problem cannot have a pseudo-polynomial
time algorithm, assuming P = NP.


8.5 Notes
Algorithm 8.2 is due to Ibarra and Kim [134]. Theorem 8.5 is due to Garey
and Johnson [92].
