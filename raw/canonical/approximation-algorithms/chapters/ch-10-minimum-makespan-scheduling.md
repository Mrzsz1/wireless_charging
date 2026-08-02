---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-10"
chapter_number: 10
chapter_title: "Minimum Makespan Scheduling"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 97
source_page_end: 101
printed_page_start: 79
printed_page_end: 83
part_ids: ["approximation-algorithms-ch-10-part-011"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Minimum Makespan Scheduling

10 Minimum Makespan Scheduling




A central problem in scheduling theory is the following.
Problem 10.1 (Minimum makespan scheduling) Given processing
times for n jobs, p1 , p2 , . . . , pn , and an integer m, ﬁnd an assignment of the
jobs to m identical machines so that the completion time, also called the
makespan, is minimized.
   We will give a simple factor 2 algorithm for this problem before presenting
a PTAS for it.


10.1 Factor 2 algorithm
The algorithm is very simple: schedule the jobs one by one, in an arbitrary
order, each job being assigned to a machine with least amount of work so far.
This algorithm is based on the following two lower bounds on the optimal
makespan, OPT:
                                                        
 1. The average time for which a machine has to run, ( i pi ) /m; and
 2. The largest processing time.
Let LB denote the combined lower bound, i.e.,
                                     
                     1 
      LB = max           pi , max{pi } .
                     m i       i




 Algorithm 10.2 (Minimum makespan scheduling)
  1. Order the jobs arbitrarily.
  2. Schedule jobs on machines in this order, scheduling the next job on the
     machine that has been assigned the least amount of work so far.



Theorem 10.3 Algorithm 10.2 achieves an approximation guarantee of 2
for the minimum makespan problem.
80      10     Minimum Makespan Scheduling

Proof: Let Mi be the machine that completes its jobs last in the schedule
produced by the algorithm, and let j be the index of the last job scheduled
on this machine.

        M1
          ..
           .
        Mi                  1
                                
               ✛           ≤m      i pi           ✲✛         pj      ✲
          ..
           .
        Mm


Let startj be the time at which job j starts execution on Mi . Since the algo-
rithm assigns a job to the least loaded machine, it follows that all machines
are busy until startj . This implies that

                   1 
      startj ≤         pi ≤ OPT.
                   m i

Further, pj ≤ OPT. Thus, the makespan of the schedule is startj + pj ≤
2 · OPT.                                                             ✷

Example 10.4 A tight example for this algorithm is provided by a sequence
of m2 jobs with unit processing time, followed by a single job of length m.
The schedule obtained by the algorithm has a makespan of 2m, while OPT =
m + 1.                                                                   ✷



10.2 A PTAS for minimum makespan
The minimum makespan problem is strongly NP-hard; thus, by Corollary
8.6, it does not admit an FPTAS, assuming P = NP. We will obtain a
PTAS for it. The minimum makespan problem is closely related to the bin
packing problem by the following observation. There exists a schedule with
makespan t iﬀ n objects of sizes p1 , p2 , . . . , pn can be packed into m bins of
capacity t each. This suggests a reduction from minimum makespan to bin
packing as follows. Denoting the sizes of the n objects, p1 , . . . , pn , by I, let
bins(I, t) represent the minimum number of bins of size t required to pack
these n objects. Then, the minimum makespan is given by

      min{t : bins(I, t) ≤ m}.
                                           10.2    A PTAS for minimum makespan                     81

As shown above, LB and 2 · LB are lower and upper bounds on the mini-
mum makespan. Thus, we can determine the minimum makespan by a bi-
nary search in this interval. At ﬁrst sight, this reduction may not seem very
useful since the bin packing problem is also NP-hard. However, it turns out
that this problem is polynomial time solvable if the object sizes are drawn
from a set of ﬁxed cardinality. We will use this fact critically for solving the
minimum makespan problem.

10.2.1      Bin packing with ﬁxed number of object sizes

We ﬁrst present a dynamic programming algorithm for the restricted bin
packing problem, thereby improving on the result of Lemma 9.4 in two ways.
We will not require a lower bound on item sizes and will improve on the
running time. Let k be the ﬁxed number of object sizes, and assume that
bins have capacity 1. Fix an ordering on the object sizes. Now, an instance
of the bin packing problem can be described by a k-tuple, (i1 , i2 , . . . , ik ),
specifying the number of objects of each size. Let BINS(i1 , i2 , . . . , ik ) denote
the minimum number of bins needed to pack                    k these objects.
     For a given instance, (n1 , n2 , . . . , nk ), i=1 ni = n, we ﬁrst compute Q,
the set of all k-tuples (q1 , q2 , . . . , qk ) such that BINS(q1 , q2 , . . . , qk ) = 1 and
0 ≤ qi ≤ ni , 1 ≤ i ≤ k. Clearly, Q contains at most O(nk ) elements. Next, we
compute all entries of the k-dimensional table BINS(i1 , i2 , . . . , ik ) for every
(i1 , i2 , . . . , ik ) ∈ {0, . . . , n1 } × {0, . . . , n2 } × . . . × {0, . . . , nk }. The table is
initialized by setting BINS(q) = 1 for every q ∈ Q. Then, we use the following
recurrence to compute the remaining entries:

       BINS(i1 , i2 , . . . , ik ) = 1 + min BINS(i1 − q1 , . . . , ik − qk ).                (10.1)
                                          q∈Q


Computing each entry takes O(nk ) time. Thus, the entire table can be com-
puted in O(n2k ) time, thereby determining BINS(n1 , n2 , . . . , nk ).

10.2.2      Reducing makespan to restricted bin packing

The basic idea is that if we can tolerate some error in computing the minimum
makespan, then we can reduce this problem to the restricted version of bin
packing in polynomial time. There will be two sources of error:
• rounding object sizes so that there are a bounded number of diﬀerent sizes,
  and
• terminating the binary search to ensure polynomial running time.
Each error can be made as small as needed, at the expense of running time.
Moreover, for any ﬁxed error bound, the running time is polynomial in n,
and thus we obtain a polynomial approximation scheme.
82      10   Minimum Makespan Scheduling

    Let ε be an error parameter and t be in the interval [LB, 2 · LB]. We say
that an object is small if its size is less than tε; small objects are discarded
for now.The rest of the objects are rounded down as follows: each pj in the
interval tε(1 + ε)i , tε(1 + ε)i+1 is replaced by pj = tε(1+ε)i , for i ≥ 0. The
resulting pj ’s can assume at most k = log1+ε 1ε  distinct values. Determine
an optimal packing for the rounded objects in bins of size t using the dynamic
programming algorithm. Since rounding reduces the size of each object by a
factor of at most 1 + ε, if we consider the original sizes of the objects, then
the packing determined is valid for a bin size of t(1 + ε). Keeping this as the
bin size, pack the small objects greedily in leftover spaces in the bins; open
new bins only if needed. Clearly, any time a new bin is opened, all previous
bins must be full to the extent of at least t. Denote with α(I, t, ε) the number
of bins used by this algorithm; recall that these bins are of size t(1 + ε).
    Let us call the algorithm presented above the core algorithm since it will
form the core of the PTAS for computing makespan. As shown in Lemma
10.5 and its corollary, the core algorithm also helps establish a lower bound
on the optimal makespan.

Lemma 10.5            α(I, t, ε) ≤ bins(I, t).

Proof: If the algorithm does not open any new bins for the small objects,
then the assertion clearly holds since the rounded down pieces have been
packed optimally in bins of size t. In the other case, all but the last bin are
packed at least to the extent of t. Hence, the optimal packing of I in bins of
size t must also use at least α(I, t, ε) bins.                               ✷
     Since OPT = min{t : bins(I, t) ≤ m}, Lemma 10.5 gives:
Corollary 10.6           min{t : α(I, t, ε) ≤ m} ≤ OPT.
    If min{t : α(I, t, ε) ≤ m} could be determined with no additional error
during the binary search, then clearly we could use the core algorithm to
obtain a schedule with a makespan of (1 + ε)OPT. Next, we will specify the
details of the binary search and show how to control the error it introduces.
The binary search is performed on the interval [LB, 2 · LB]. Thus, the length
of the available interval is LB at the start of the search, and it reduces by
a factor of 2 in each iteration. We continue the search until the available
interval drops to a length of ε · LB. This will require log2 1ε  iterations. Let
T be the right endpoint of the interval we terminate with.

Lemma 10.7            T ≤ (1 + ε) · OPT.

Proof: Clearly, min{t : α(I, t, ε) ≤ m} must be in the interval [T − ε · LB, T ].
Hence,

       T ≤ min{t : α(I, t, ε) ≤ m} + ε · LB.
                                                          10.4   Notes     83

Now, using Corollary 10.6 and the fact that LB ≤ OPT, the lemma follows.
✷
  Finally, the output of the core algorithm for t = T gives a schedule whose
makespan is at most T · (1 + ε). We get:

Theorem 10.8 The algorithm produces a valid schedule having makespan at
most

     (1 + ε)2 · OPT ≤ (1 + 3ε) · OPT.
                                                               
    The running time of the entire algorithm is O n2k log2 1ε  , where k =
log1+ε 1ε .


10.3 Exercises

10.1 (Graham [114]) The tight example for the factor 2 algorithm, Example
10.4, involves scheduling a very long job last. This suggests sorting the jobs
by decreasing processing times before scheduling them. Show that this leads
to a 4/3 factor algorithm. Provide a tight example for this algorithm.

10.2 (Horowitz and Sahni [131]) Give an FPTAS for the variant of the
minimum makespan scheduling problem in which the number of machines,
m, is a ﬁxed constant.


10.4 Notes
Algorithm 10.2 is due to Graham [113]. The PTAS is due to Hochbaum and
Shmoys [128].
