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

# Minimum Makespan Scheduling (MinerU semantic layer)

<!-- source-pages: 97-101; printed-pages: 79-83; mineru-part: approximation-algorithms-ch-10-part-011 -->

# 10 Minimum Makespan Scheduling

A central problem in scheduling theory is the following.

Problem 10.1 (Minimum makespan scheduling) Given processing times for n jobs, $p _ { 1 } , p _ { 2 } , \ldots , p _ { n }$ , and an integer m, find an assignment of the jobs to m identical machines so that the completion time, also called the makespan, is minimized.

We will give a simple factor 2 algorithm for this problem before presenting a PTAS for it.

## 10.1 Factor 2 algorithm

The algorithm is very simple: schedule the jobs one by one, in an arbitrary order, each job being assigned to a machine with least amount of work so far. This algorithm is based on the following two lower bounds on the optimal makespan, OPT:

1. The average time for which a machine has to run, $\left( \sum _ { i } p _ { i } \right) / m ;$ and

2. The largest processing time.

Let LB denote the combined lower bound, i.e.,

$$
\mathrm{LB} = \max \left\{\frac {1}{m} \sum_ {i} p _ {i}, \max _ {i} \{p _ {i} \} \right\}.
$$

## Algorithm 10.2 (Minimum makespan scheduling)

1. Order the jobs arbitrarily.

2. Schedule jobs on machines in this order, scheduling the next job on the machine that has been assigned the least amount of work so far.

Theorem 10.3 Algorithm 10.2 achieves an approximation guarantee of 2 for the minimum makespan problem.

Proof: Let $M _ { i }$ be the machine that completes its jobs last in the schedule produced by the algorithm, and let $j$ be the index of the last job scheduled on this machine.

![](images/0e061d1d933a37e473fd3e988b5b7c9f60afab7b78f753cefc782465ee70a3db.jpg)

Let $s t a r t _ { j }$ be the time at which job $j$ starts execution on $M _ { i } .$ Since the algorithm assigns a job to the least loaded machine, it follows that all machines are busy until start<sub>j</sub>. This implies that

$$
s t a r t _ {j} \leq \frac {1}{m} \sum_ {i} p _ {i} \leq \mathrm{OPT}.
$$

Further, $p _ { j } \ \leq \ \mathrm { O P T }$ . Thus, the makespan of the schedule is $s t a r t _ { j } + p _ { j } \ \leq$ 2 · OPT. ✷

Example 10.4 A tight example for this algorithm is provided by a sequence of $m ^ { 2 }$ jobs with unit processing time, followed by a single job of length m. The schedule obtained by the algorithm has a makespan of $2 m ,$ while $\mathrm { O P T = }$ $m + 1$ ✷

## 10.2 A PTAS for minimum makespan

The minimum makespan problem is strongly NP-hard; thus, by Corollary 8.6, it does not admit an FPTAS, assuming $\mathbf { P } \neq \mathbf { N P }$ . We will obtain a PTAS for it. The minimum makespan problem is closely related to the bin packing problem by the following observation. There exists a schedule with makespan t if n objects of sizes $p _ { 1 } , p _ { 2 } , \ldots , p _ { n }$ can be packed into m bins of capacity t each. This suggests a reduction from minimum makespan to bin packing as follows. Denoting the sizes of the n objects, $p _ { 1 } , \ldots , p _ { n }$ , by $I ,$ let bins(I, t) represent the minimum number of bins of size t required to pack these n objects. Then, the minimum makespan is given by

$$
\min \{t: \text {   bins } (I, t) \leq m \}.
$$

As shown above, LB and 2 · LB are lower and upper bounds on the minimum makespan. Thus, we can determine the minimum makespan by a binary search in this interval. At first sight, this reduction may not seem very useful since the bin packing problem is also NP-hard. However, it turns out that this problem is polynomial time solvable if the object sizes are drawn from a set of fixed cardinality. We will use this fact critically for solving the minimum makespan problem.

## 10.2.1 Bin packing with fixed number of object sizes

We first present a dynamic programming algorithm for the restricted bin packing problem, thereby improving on the result of Lemma 9.4 in two ways. We will not require a lower bound on item sizes and will improve on the running time. Let k be the fixed number of object sizes, and assume that bins have capacity 1. Fix an ordering on the object sizes. Now, an instance of the bin packing problem can be described by a k-tuple, $( i _ { 1 } , i _ { 2 } , \ldots , i _ { k } )$ 2 specifying the number of objects of each size. Let $\mathrm { B I N S } ( i _ { 1 } , i _ { 2 } , \dots , i _ { k } )$ denote the minimum number of bins needed to pack these objects.

For a given instance, $( n _ { 1 } , n _ { 2 } , . . . , n _ { k } ) , \sum _ { i = 1 } ^ { k } n _ { i } = n$ , we first compute $\mathcal { Q } ,$ the set of all k-tuples $( q _ { 1 } , q _ { 2 } , \ldots , q _ { k } )$ such that BINS $( q _ { 1 } , q _ { 2 } , \ldots , q _ { k } ) = 1$ and $0 \leq q _ { i } \leq n _ { i } , 1 \leq i \leq k$ . Clearly, $\mathcal { Q }$ contains at most $O ( n ^ { k } )$ elements. Next, we compute all entries of the k-dimensional table BINS $( i _ { 1 } , i _ { 2 } , \ldots , i _ { k } )$ for every $( i _ { 1 } , i _ { 2 } , \ldots , i _ { k } ) \in \{ 0 , \ldots , n _ { 1 } \} \times \{ 0 , \ldots , n _ { 2 } \} \times \ldots \times \{ 0 , \ldots , n _ { k } \}$ . The table is initialized by setting $\mathrm { B I N S } ( q ) = 1$ for every $q \in \mathcal { Q }$ . Then, we use the following recurrence to compute the remaining entries:

$$
\operatorname{BINS} (i _ {1}, i _ {2}, \dots , i _ {k}) = 1 + \min _ {q \in \mathcal {Q}} \operatorname{BINS} (i _ {1} - q _ {1}, \dots , i _ {k} - q _ {k}).\tag{10.1}
$$

Computing each entry takes $O ( n ^ { k } )$ time. Thus, the entire table can be computed in ${ \bar { O } } ( n ^ { 2 k } )$ time, thereby determining $\mathrm { B I N S } ( n _ { 1 } , n _ { 2 } , \dots , n _ { k } )$

## 10.2.2 Reducing makespan to restricted bin packing

The basic idea is that if we can tolerate some error in computing the minimum makespan, then we can reduce this problem to the restricted version of bin packing in polynomial time. There will be two sources of error:

• rounding object sizes so that there are a bounded number of diferent sizes, and

• terminating the binary search to ensure polynomial running time.

Each error can be made as small as needed, at the expense of running time. Moreover, for any fixed error bound, the running time is polynomial in $n _ { \mathrm { : } }$ and thus we obtain a polynomial approximation scheme.

Let ε be an error parameter and t be in the interval [LB, 2 · LB]. We say that an object is small if its size is less than $t \varepsilon ;$ small objects are discarded for now. The rest of the objects are rounded down as follows: each $p _ { j }$ in the interval $\lceil t \varepsilon ( 1 + \varepsilon ) ^ { i } , t \varepsilon ( 1 + \stackrel { \sim } { \varepsilon } ) ^ { i + 1 } \rceil$ is replaced by $p _ { i } ^ { \prime } = t \varepsilon ( 1 + \varepsilon ) ^ { i }$ , for $i \geq 0$ . The <sub>resulting</sub> $p _ { j } ^ { \prime } \mathrm { { ' s } }$ can assume at most $k = \lceil \log _ { 1 + \varepsilon } \frac { 1 } { \varepsilon } \rceil$ distinct values. Determine an optimal packing for the rounded objects in bins of size t using the dynamic programming algorithm. Since rounding reduces the size of each object by a factor of at most $1 + \varepsilon$ , if we consider the original sizes of the objects, then the packing determined is valid for a bin size of $t ( 1 + \varepsilon )$ . Keeping this as the bin size, pack the small objects greedily in leftover spaces in the bins; open new bins only if needed. Clearly, any time a new bin is opened, all previous bins must be full to the extent of at least t. Denote with $\alpha ( I , t , \varepsilon )$ the number of bins used by this algorithm; recall that these bins are of size $t ( 1 + \varepsilon )$

Let us call the algorithm presented above the core algorithm since it will form the core of the PTAS for computing makespan. As shown in Lemma 10.5 and its corollary, the core algorithm also helps establish a lower bound on the optimal makespan.

Lemma 10.5

$$
\alpha (I, t, \varepsilon) \leq \operatorname{bins} (I, t).
$$

Proof: If the algorithm does not open any new bins for the small objects, then the assertion clearly holds since the rounded down pieces have been packed optimally in bins of size t. In the other case, all but the last bin are packed at least to the extent of t. Hence, the optimal packing of I in bins of size t must also use at least $\alpha ( I , t , \varepsilon )$ bins. ✷

Since OPT = min{t : bins $( I , t ) \leq m \}$ , Lemma 10.5 gives:

Corollary 10.6 min $\{ t : \ \alpha ( I , t , \varepsilon ) \leq m \} \leq \mathrm { O P T } .$

If min $\{ t : ~ \alpha ( I , t , \varepsilon ) \leq m \}$ could be determined with no additional error during the binary search, then clearly we could use the core algorithm to obtain a schedule with a makespan of $( 1 + \varepsilon ) \mathrm { O P T }$ . Next, we will specify the details of the binary search and show how to control the error it introduces. The binary search is performed on the interval [LB, 2 · LB]. Thus, the length of the available interval is LB at the start of the search, and it reduces by a factor of 2 in each iteration. We continue the search until the available interval drops to a length of $\varepsilon \cdot \mathrm { L B }$ . This will require $\left\lceil \log _ { 2 } \frac { 1 } { \varepsilon } \right\rceil$ iterations. Let T be the right endpoint of the interval we terminate with.

Lemma 10.7

$$
T \leq (1 + \varepsilon) \cdot \text { OPT }.
$$

Proof: Clearly, min $\{ t : \alpha ( I , t , \varepsilon ) \leq m \}$ must be in the interval $[ T - \varepsilon \cdot L B , T ]$ Hence,

$$
T \leq \min \{t: \alpha (I, t, \varepsilon) \leq m \} + \varepsilon \cdot \mathrm{LB}.
$$

Now, using Corollary 10.6 and the fact that $\mathrm { L B } \leq \mathrm { O P T }$ , the lemma follows. ✷

Finally, the output of the core algorithm for $t = T$ gives a schedule whose makespan is at most $T \cdot ( 1 + \varepsilon )$ . We get:

Theorem 10.8 The algorithm produces a valid schedule having makespan at most

$$
(1 + \varepsilon) ^ {2} \cdot \mathrm{OPT} \leq (1 + 3 \varepsilon) \cdot \mathrm{OPT}.
$$

The running time of the entire algorithm is $O \left( n ^ { 2 k } \lceil \log _ { 2 } \frac { 1 } { \varepsilon } \rceil \right)$ , where $k =$ $\lceil \log _ { 1 + \varepsilon } \frac { 1 } { \varepsilon } \rceil$

## 10.3 Exercises

10.1 (Graham [114]) The tight example for the factor 2 algorithm, Example 10.4, involves scheduling a very long job last. This suggests sorting the jobs by decreasing processing times before scheduling them. Show that this leads to a $4 / 3$ factor algorithm. Provide a tight example for this algorithm.

10.2 (Horowitz and Sahni [131]) Give an FPTAS for the variant of the minimum makespan scheduling problem in which the number of machines, m, is a fixed constant.

## 10.4 Notes

Algorithm 10.2 is due to Graham [113]. The PTAS is due to Hochbaum and Shmoys [128].
