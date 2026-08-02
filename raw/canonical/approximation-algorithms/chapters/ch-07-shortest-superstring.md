---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-07"
chapter_number: 7
chapter_title: "Shortest Superstring"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 79
source_page_end: 85
printed_page_start: 61
printed_page_end: 67
part_ids: ["approximation-algorithms-ch-07-part-008"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Shortest Superstring

7 Shortest Superstring




In Chapter 2 we deﬁned the shortest superstring problem (Problem 2.9) and
gave a preliminary approximation algorithm using set cover. In this chapter,
we will ﬁrst give a factor 4 algorithm, and then we will improve this to factor
3.


7.1 A factor 4 algorithm
We begin by developing a good lower bound on OPT. Let us assume that
s1 , s2 , . . . , sn are numbered in order of leftmost occurrence in the shortest
superstring, s.


                 )
     pref(s1 , s2✲
     ✛                                           ✛ n−1 , s✲✛
                                                 pref(s   n )pref(sn , s1 ) over(sn , s1 )
                                                                        ✲✛             ✲
s
s1
              s2
                                     ..
                                          .
                                              sn−1
                                                           sn
                                                                        s1

    Let overlap(si , sj ) denote the maximum overlap between si and sj , i.e.,
the longest suﬃx of si that is a preﬁx of sj . Also, let preﬁx(si , sj ) be the preﬁx
of si obtained by removing its overlap with sj . The overlap in s between two
consecutive si ’s is maximum possible, because otherwise a shorter superstring
can be obtained. Hence, assuming that no si is a substring of another, we get

        OPT = |preﬁx(s1 , s2 )| + |preﬁx(s2 , s3 )| + . . . + |preﬁx(sn , s1 )|
                   + |overlap(sn , s1 )|.                                              (7.1)

   Notice that we have repeated s1 at the end in order to obtain the last two
terms of (7.1). This equality shows the close relation between the shortest
62        7   Shortest Superstring

superstring of S and the minimum traveling salesman tour on the preﬁx graph
of S, deﬁned as the directed graph on vertex set {1, . . . , n} that contains an
edge i → j of weight |preﬁx(si , sj )| for each i, j (self loops included). Clearly,
|preﬁx(s1 , s2 )| + |preﬁx(s2 , s3 )| + . . . + |preﬁx(sn , s1 )| represents the weight of
the tour 1 → 2 → . . . → n → 1. Hence, by (7.1), the minimum weight of a
traveling salesman tour of the preﬁx graph gives a lower bound on OPT. As
such, this lower bound is not very useful, since we cannot eﬃciently compute
a minimum traveling salesman tour.
     The key idea is to lower-bound OPT using the minimum weight of a
cycle cover of the preﬁx graph (a cycle cover is a collection of disjoint cycles
covering all vertices). Since the tour 1 → 2 → . . . → n → 1 is a cycle cover,
from (7.1) we get that the minimum weight of a cycle cover lower-bounds
OPT.
     Unlike minimum TSP, a minimum weight cycle cover can be computed in
polynomial time. Corresponding to the preﬁx graph, construct the following
bipartite graph, H. U = {u1 , . . . , un } and V = {v1 , . . . , vn } are the vertex
sets of the two sides of the bipartition. For each i, j ∈ {1, . . . , n} add edge
(ui , vj ) of weight |preﬁx(si , sj )|. It is easy to see that each cycle cover of the
preﬁx graph corresponds to a perfect matching of the same weight in H and
vice versa. Hence, ﬁnding a minimum weight cycle cover reduces to ﬁnding a
minimum weight perfect matching in H.
     If c = (i1 → i2 → . . . il → i1 ) is a cycle in the preﬁx graph, let

        α(c) = preﬁx(si1 , si2 ) ◦ . . . ◦ preﬁx(sil−1 , sil ) ◦ preﬁx(sil , si1 ).

Notice that each string si1 , si2 , . . . , sil is a substring of (α(c))∞ . Next, let

        σ(c) = α(c) ◦ si1 .

Then σ(c) is a superstring of si1 , . . . , sil .1 In the above construction, we
“opened” cycle c at an arbitrary string si1 . For the rest of the algorithm, we
will call si1 the representative string for c. We can now state the complete
algorithm:


 Algorithm 7.1 (Shortest superstring – factor 4)
     1. Construct the preﬁx graph corresponding to strings in S.
     2. Find a minimum weight cycle cover of the preﬁx graph, C = {c1 , . . . , ck }.
     3. Output σ(c1 ) ◦ . . . ◦ σ(ck ).



1
     This remains true even for the shorter string α(c) ◦ overlap(sl , s1 ). We will work
     with σ(c), since it will be needed for the factor 3 algorithm presented in the next
     section, where we use the property that σ(c) begins and ends with a copy of si1 .
                                                      7.1   A factor 4 algorithm   63

   Clearly, the output is a superstring of the strings in S. Notice that if in
each of the cycles we can ﬁnd a representative string of length at most the
weight of the cycle, then the string output is within 2 · OPT. Thus, the hard
case is when all strings of some cycle c are long. But since they must all
be substrings of (α(c))∞ , they must be periodic. This will be used to prove
Lemma 7.3, which establishes another lower bound on OPT.
Lemma 7.2 If each string in S  ⊆ S is a substring of t∞ for a string t,
then there is a cycle of weight at most |t| in the preﬁx graph covering all the
vertices corresponding to strings in S  .

Proof: For each string in S  , locate the starting point of its ﬁrst occurrence
in t∞ . Clearly, all these starting points will be distinct (since no string in S
is a substring of another) and will lie in the ﬁrst copy of t. Consider the cycle
in the preﬁx graph visiting the corresponding vertices in this order. Clearly,
the weight of this cycle is at most |t|.                                        ✷

Lemma 7.3 Let c and c be two cycles in C, and let r, r be representative
strings from these cycles. Then

      |overlap(r, r )| < wt(c) + wt(c ).

Proof: Suppose, for contradiction, that |overlap(r, r )| ≥ wt(c) + wt(c ). De-
note by α (α ) the preﬁx of length wt(c) (wt(c ), respectively) of overlap(r, r ).

                                    overlap(r, r )
                 ✛                                                    ✲
r                        α                        α

                 ✛          α ◦ α = α ◦ α           ✲
             
         r
                       α                α                   α

    Clearly, overlap(r, r ) is a preﬁx of both α∞ and (α )∞ . In addition, α is
a preﬁx of (α )∞ and α is a preﬁx of α∞ . Since overlap(r, r ) ≥ |α| + |α |, it
follows that α and α commute, i.e., α ◦ α = α ◦ α. But then, α∞ = (α )∞ .
This is so because for any k > 0,

      αk ◦ (α )k = (α )k ◦ αk .

Hence, for any N > 0, the preﬁx of length N of α∞ is the same as that of
(α )∞ .
    Now, by Lemma 7.2, there is a cycle of weight at most wt(c) in the
preﬁx graph covering all strings in c and c , contradicting the fact that C is
a minimum weight cycle cover.                                                ✷
64       7    Shortest Superstring

Theorem 7.4 Algorithm 7.1 achieves an approximation factor of 4 for the
shortest superstring problem.
                            k
Proof: Let wt(C) =               i=1 wt(ci ). The output of the algorithm has length

      k
                                      k
                                       
             |σ(ci )| = wt(C) +              |ri |,
      i=1                              i=1

where ri denotes the representative string from cycle ci . We have shown that
wt(C) ≤ OPT. Next, we show that the sum of the lengths of representative
strings is at most 3 · OPT.
    Assume that r1 , . . . , rk are numbered in order of their leftmost occurrence
in the shortest superstring of S. Using Lemma 7.3, we get the following lower
bound on OPT:
                 k
                                k−1
                                                                k
                                                                                  k
                                                                                   
      OPT ≥            |ri | −         |overlap(ri , ri+1 )| ≥         |ri | − 2         wt(ci ).
                 i=1             i=1                             i=1               i=1

Hence,

      k
                                 k
                                  
             |ri | ≤ OPT + 2            wt(ci ) ≤ 3 · OPT.
      i=1                         i=1

                                                                                                    ✷



7.2 Improving to factor 3
Notice that any superstring of the strings σ(ci ), i = 1, . . . , k, is also a super-
string of all strings in S. Instead of simply concatenating these strings, let
us make them overlap as much as possible (this may sound circular, but it is
not!).
     Let X be a set of strings. We will denote by ||X|| the sum of the lengths
of the strings in X. Let us deﬁne the compression achieved by a superstring
s as the diﬀerence between the sum of the lengths of the input strings and
|s|, i.e., ||S|| − |s|. Clearly, maximum compression is achieved by the shortest
superstring. Several algorithms are known to achieve at least half the optimal
compression. For instance, the greedy superstring algorithm, described in
Section 2.3, does so; however, its proof is based on a complicated case analysis.
For a less eﬃcient algorithm, see Section 7.2.1. Either of these algorithms can
be used in Step 3 of Algorithm 7.5.
                                                 7.2   Improving to factor 3    65


 Algorithm 7.5 (Shortest superstring – factor 3)
  1. Construct the preﬁx graph corresponding to strings in S.
  2. Find a minimum cycle cover of the preﬁx graph, C = {c1 , . . . , ck }.
  3. Run the greedy superstring algorithm on {σ(c1 ), . . . , σ(ck )} and output
     the resulting string, say τ .


   Let OPTσ denote the length of the shortest superstring of the strings in
Sσ = {σ(c1 ) . . . σ(ck )}, and let ri be the representative string of ci .
Lemma 7.6                |τ | ≤ OPTσ + wt(C).

Proof: Assume w.l.o.g. that σ(c1 ), . . . , σ(ck ) appear in this order in a short-
est superstring of Sσ . The maximum compression that can be achieved on
Sσ is given by

      k−1
      
            |overlap(σ(ci ), σ(ci+1 ))|.
      i=1


Since each string σ(ci ) has ri as a preﬁx as well as suﬃx, by Lemma 7.3,

      |overlap(σ(ci ), σ(ci+1 ))| ≤ wt(ci ) + wt(ci+1 ).

Hence, the maximum compression achievable on Sσ is at most 2 · wt(C), i.e.,
||Sσ || − OPTσ ≤ 2 · wt(C).
    The compression achieved by the greedy superstring algorithm on Sσ is
at least half the maximum compression. Therefore,

                         1
      ||Sσ || − |τ | ≥     (||Sσ || − OPTσ ).
                         2
Therefore,

      2(|τ | − OPTσ ) ≤ ||Sσ || − OPTσ ≤ 2 · wt(C).

The lemma follows.                                                                 ✷
   Finally, we relate OPTσ to OPT.
Lemma 7.7                OPTσ ≤ OPT + wt(C).

Proof: Let OPTr denote the length of the shortest superstring of the strings
in Sr = {r1 , . . . , rk }. The key observation is that each σ(ci ) begins and ends
with ri . Therefore, the maximum compression achievable on Sσ is at least as
large as that achievable on Sr , i.e.,
66       7     Shortest Superstring

        ||Sσ || − OPTσ ≥ ||Sr || − OPTr .

     Clearly, ||Sσ || = ||Sr || + wt(C). This gives

        OPTσ ≤ OPTr + wt(C).

The lemma follows by noticing that OPTr ≤ OPT.                                   ✷
     Combining the previous two lemmas we get:
Theorem 7.8 Algorithm 7.5 achieves an approximation factor of 3 for the
shortest superstring problem.

7.2.1     Achieving half the optimal compression

We give a superstring algorithm that achieves at least half the optimal com-
pression. Suppose that the strings to be compressed, s1 , · · · , sn , are numbered
in the order in which they appear in a shortest superstring. Then, the optimal
compression is given by

        n−1
        
              |overlap(σi , σi+1 )|.
        i=1

This is the weight of the traveling salesman path 1 → 2 → . . . → n in the
overlap graph, H, of the strings s1 , · · · , sn . H is a directed graph that has a
vertex vi corresponding to each string si , and contains an edge (vi → vj ) of
weight |overlap(si , sj )| for each i = j, 1 ≤ i, j ≤ n (H has no self loops).
    The optimal compression is upper bounded by the cost of a maximum
traveling salesman tour in H, which in turn is upper bounded by the cost
of a maximum cycle cover. The latter can be computed in polynomial time
using matching, similar to the way we computed a minimum weight cycle
cover. Since H has no self loops, each cycle has length at least 2. Remove
the lightest edge from each cycle of the maximum cycle cover to obtain a set
of disjoint paths. The sum of weights of edges on these paths is at least half
the optimal compression. Overlap strings s1 , · · · , sn according to the edges
of these paths and concatenate the resulting strings. This gives a superstring
achieving at least half the optimal compression.


7.3 Exercises

7.1 Show that Lemma 7.3 cannot be strengthened to

        |overlap(r, r )| < max {wt(c), wt(c )}.
                                                           7.4   Notes    67

7.2 (Jiang, Li, and Du [148]) Obtain constant factor approximation algo-
rithms for the variants of the shortest superstring problem given in Exercise
2.16.


7.4 Notes
The algorithms given in this chapter are due to Blum, Jiang, Li, Tromp, and
Yannakakis [27].
