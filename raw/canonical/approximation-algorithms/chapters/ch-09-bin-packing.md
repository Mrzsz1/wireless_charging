---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-09"
chapter_number: 9
chapter_title: "Bin Packing"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 92
source_page_end: 96
printed_page_start: 74
printed_page_end: 78
part_ids: ["approximation-algorithms-ch-09-part-010"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Bin Packing

9 Bin Packing




Consider the following problem.
Problem 9.1 (Bin packing) Given n items with sizes a1 , . . . , an ∈ (0, 1],
ﬁnd a packing in unit-sized bins that minimizes the number of bins used.
   This problem ﬁnds many industrial applications. For instance, in the
stock-cutting problem, bins correspond to a standard length of paper and
items correspond to speciﬁed lengths that need to be cut.
   It is easy to obtain a factor 2 approximation algorithm for this problem.
For instance, let us consider the algorithm called First-Fit. This algorithm
considers items in an arbitrary order. In the ith step, it has a list of partially
packed bins, say B1 , . . . , Bk . It attempts to put the next item, ai , in one of
these bins, in this order. If ai does not ﬁt into any of these bins, it opens a
new bin Bk+1 , and puts ai in it. If the algorithm uses m bins, then at least
m − 1 bins are more than half full. Therefore,
      n
                  m−1
            ai >       .
      i=1
                    2

Since the sum of the item sizes is a lower bound on OPT, m − 1 < 2OPT,
i.e., m ≤ 2OPT (see Notes for a better analysis). On the negative side:
Theorem 9.2 For any ε > 0, there is no approximation algorithm having a
guarantee of 3/2 − ε for the bin packing problem, assuming P = NP.

Proof: If there were such an algorithm, then we show how to solve the NP-
hard problem of deciding if there is a way to partition
                                                         n nonnegative numbers
a1 , . . . , an into two sets, each adding up to 12 i ai . Clearly, the answer
                                                                              to
this question is ‘yes’ iﬀ the n items can be packed in 2 bins of size 12 i ai . If
the answer is ‘yes’ the 3/2 − ε factor algorithm will have to give an optimal
packing, and thereby solve the partitioning problem.                            ✷


9.1 An asymptotic PTAS
Notice that the argument in Theorem 9.2 uses very special instances: those
for which OPT is a small number, such as 2 or 3, even though the number
                                             9.1   An asymptotic PTAS       75

of items is unbounded. What can we say about “typical” instances, those for
which OPT increases with n?
Theorem 9.3 For any ε, 0 < ε ≤ 1/2, there is an algorithm Aε that runs
in time polynomial in n and ﬁnds a packing using at most (1 + 2ε)OPT + 1
bins.
   The sequence of algorithms, Aε , form an asymptotic polynomial time ap-
proximation scheme for bin packing, since for each ε > 0 ∃N > 0, and a
polynomial time algorithm in this sequence, say B, such that B has an ap-
proximation guarantee of 1 + ε for all instances having OPT ≥ N . However,
Theorem 9.3 should not be considered a practical solution to the bin packing
problem, since the running times of the algorithms Aε are very high.
   We will prove Theorem 9.3 in three steps.
Lemma 9.4 Let ε > 0 be ﬁxed, and let K be a ﬁxed nonnegative integer.
Consider the restriction of the bin packing problem to instances in which each
item is of size at least ε and the number of distinct item sizes is K. There is
a polynomial time algorithm that optimally solves this restricted problem.

Proof: The number of items in a bin is bounded by 1/ε. Denote this by
M . Therefore, the number of diﬀerent bin types is bounded by R = MM    +K

(see Exercise 9.4), which is a (large!) constant. Clearly, the total number
                         Therefore,
of bins used is at most n.            the number of possible feasible pack-
ings is bounded by P = n+R R   , which is polynomial in n (see Exercise 9.4).
Enumerating them and picking the best packing gives the optimal answer. ✷

Lemma 9.5 Let ε > 0 be ﬁxed. Consider the restriction of the bin packing
problem to instances in which each item is of size at least ε. There is a
polynomial time approximation algorithm that solves this restricted problem
within a factor of (1 + ε).

Proof: Let I denote the given instance. Sort the n items by increasing size,
and partition them into K = 1/ε2  groups each having at most Q = nε2 
items. Notice that two groups may contain items of the same size.


   J




  J’

   Construct instance J by rounding up the size of each item to the size of
the largest item in its group. Instance J has at most K diﬀerent item sizes.
76       9   Bin Packing

Therefore, by Lemma 9.4, we can ﬁnd an optimal packing for J. Clearly, this
will also be a valid packing for the original item sizes. We show below that
OPT(J) ≤ (1 + ε)OPT(I), thereby proving the lemma.
    The following clever argument accomplishes this. Let us construct another
instance, say J  , by rounding down the size of each item to that of the smallest
item in its group. Clearly OPT(J  ) ≤ OPT(I). The crucial observation is that
a packing for instance J  yields a packing for all but the largest Q items of
instance J (Exercise 9.6 asks for a formal proof). Therefore,

        OPT(J) ≤ OPT(J  ) + Q ≤ OPT(I) + Q.

Since each item in I has size at least ε, OPT(I) ≥ nε. Therefore, Q = nε2  ≤
εOPT. Hence, OPT(J) ≤ (1 + ε)OPT(I).                                         ✷

Proof of Theorem 9.3: Let I denote the given instance, and I  denote the
instance obtained by discarding items of size < ε from I. By Lemma 9.5, we
can ﬁnd a packing for I  using at most (1 + ε)OPT(I  ) bins. Next, we start
packing the small items (of size < ε) in a First-Fit manner in the bins opened
for packing I  . Additional bins are opened if an item does not ﬁt into any of
the already open bins.
    If no additional bins are needed, then we have a packing in (1+ε)OPT(I  ) ≤
(1 + ε)OPT(I) bins. In the second case, let M be the total number of bins
used. Clearly, all but the last bin must be full to the extent of at least 1 − ε.
Therefore, the sum of the item sizes in I is at least (M − 1)(1 − ε). Since this
is a lower bound on OPT, we get

              OPT
        M≤           + 1 ≤ (1 + 2ε)OPT + 1,
             (1 − ε)

where we have used the assumption that ε ≤ 1/2. Hence, for each value of ε,
0 < ε ≤ 1/2, we have a polynomial time algorithm achieving a guarantee of
(1 + 2ε)OPT + 1.                                                         ✷
     Algorithm Aε is summarized below.


 Algorithm 9.6 (Algorithm Aε for bin packing)
     1. Remove items of size < ε.
     2. Round to obtain constant number of item sizes (Lemma 9.5).
     3. Find optimal packing (Lemma 9.4).
     4. Use this packing for original item sizes.
     5. Pack items of size < ε using First-Fit.
                                                          9.2   Exercises     77

9.2 Exercises

9.1 Give an example on which First-Fit does at least as bad as 5/3 · OPT.

9.2 (Johnson [149]) Consider a more restricted algorithm than First-Fit,
called Next-Fit, which tries to pack the next item only in the most recently
started bin. If it does not ﬁt, it is packed in a new bin. Show that this
algorithm also achieves factor 2. Give a factor 2 tight example.

9.3 (C. Kenyon) Say that a bin packing algorithm is monotonic if the number
of bins it uses for packing a subset of the items is at most the number of bins
it uses for packing all n items. Show that whereas Next-Fit is monotonic,
First-Fit is not.

9.4 Prove the bounds on R and P stated in Lemma 9.4.
Hint: Use the fact that the number of ways of throwing n identical balls
into k distinct bins is n+k−1
                          n   .

9.5 Consider an alternative way of establishing Lemma 9.5. All items having
sizes in the interval (ε(1 + ε)r , ε(1 + ε)r+1 ] are rounded up to min(ε(1 +
ε)r+1 , 1), for r ≥ 0. Clearly, this yields a constant number of item sizes. Does
the rest of the proof go through?
Hint: Consider the situation that there are lots of items of size 1/2, and
1/2 = ε(1 + ε)r for any r ≥ 0.

9.6 Prove the following statement made in Lemma 9.5, “A packing for
instance J  yields a packing for all but the largest Q items of instance J.”
Hint: Throw away the Q largest items of J and the Q smallest items of J  ,
and establish a domination.

9.7 Use the fact that integer programming with a ﬁxed number of variables
is in P to give an alternative proof of Lemma 9.4. (Because of the exorbi-
tant running time of the integer programming algorithm, this variant is also
impractical.)

9.8 Show that if there is an algorithm for bin packing having a guarantee
of OPT(I) + log2 (OPT(I)), then there is a fully polynomial approximation
scheme for this problem.

9.9 (C. Kenyon) Consider the following problem.
Problem 9.7 (Bin covering) Given n items with sizes a1 , . . . , an ∈ (0, 1],
maximize the number of bins opened so that each bin has items summing to
at least 1.
    Give an asymptotic PTAS for this problem when restricted to instances
in which item sizes are bounded below by c, for a ﬁxed constant c > 0.
Hint: The main idea of Algorithm 9.6 applies to this problem as well.
78     9   Bin Packing

9.3 Notes
The ﬁrst nontrivial bin packing result, showing that First-Fit requires at most
(17/10)OPT + 3 bins, was due to Ullman [248]. The asymptotic PTAS is due
to Fernandez de la Vega and Lueker [86]. An improved algorithm, having
a guarantee of OPT(I) + log2 (OPT(I)) was given by Karmarkar and Karp
[163]. For further results, see the survey of Coﬀman, Garey, and Johnson [50].
The result cited in Exercise 9.7, showing that integer programming with a
ﬁxed number of variables is in P, is due to Lenstra [185]. Bin packing has
also been extensively studied in the on-line model. For these and other on-line
algorithms see Borodin and El-Yaniv [31].
