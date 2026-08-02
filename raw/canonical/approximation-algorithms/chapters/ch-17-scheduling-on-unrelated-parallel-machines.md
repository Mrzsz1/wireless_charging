---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-17"
chapter_number: 17
chapter_title: "Scheduling on Unrelated Parallel Machines"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 158
source_page_end: 163
printed_page_start: 140
printed_page_end: 145
part_ids: ["approximation-algorithms-ch-17-part-018"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Scheduling on Unrelated Parallel Machines

17 Scheduling on Unrelated Parallel Machines




LP-rounding has yielded approximation algorithms for a large number of
NP-hard problems in scheduling theory (see Section 17.6). As a illustrative
example, we present a factor 2 algorithm for the problem of scheduling on un-
related parallel machines. We will apply the technique of parametric pruning,
introduced in Chapter 5, together with LP-rounding, to obtain the algorithm.
Problem 17.1 (Scheduling on unrelated parallel machines) Given a
set J of jobs, a set M of machines, and for each j ∈ J and i ∈ M , pij ∈ Z+ ,
the time taken to process job j on machine i, the problem is to schedule the
jobs on the machines so as to minimize the makespan, i.e., the maximum
processing time of any machine. We will denote the number of jobs by n and
the number of machines by m.
    The reason for the name “unrelated” is that we have not assumed any
relation between the processing times of a job on the diﬀerent machines. If
each job j has the same running time, say pj , on each of the machines, then
the machines are said to be identical. This problem was studied in Chapter 10
under the name minimum makespan scheduling, and we had derived a PTAS
for it. A generalization of minimum makespan, that also admits a PTAS, is
that of uniform parallel machines (see Exercise 17.5). In this case there is a
speed si associated with each machine i, and the processing time for job j
on machine i is pj /si .


17.1 Parametric pruning in an LP setting
An obvious integer program for this problem is the following. In this program
xij is an indicator variable denoting whether job j is scheduled on machine
i. The objective is to minimize t, the makespan. The ﬁrst set of constraints
ensures that each job is scheduled on one of the machines, and the second
set ensures that each machine has a processing time of at most t.

     minimize      t                                                    (17.1)
                   
     subject to          xij = 1,    j∈J
                   i∈M
                                    17.2    Properties of extreme point solutions   141
                       
                             xij pij ≤ t,    i∈M
                       j∈J
                       xij ∈ {0, 1},         i ∈ M, j ∈ J

   We show below that this integer program has unbounded integrality gap.
Example 17.2 Suppose we have only one job, which has a processing time
of m on each of the m machines. Clearly, the minimum makespan is m.
However, the optimal solution to the linear relaxation is to schedule the job
to the extent of 1/m on each machine, thereby leading to an objective function
value of 1, and giving an integrality gap of m.                              ✷
    This example is exploiting an “unfair” advantage that we have given to the
linear relaxation. The integer program automatically sets xij to 0 if pij > t.
On the other hand, the linear relaxation is allowed to set these variables to
nonzero values, and thereby obtain a cheaper solution. The situation could
be rectiﬁed if we could add the following constraint to the linear relaxation:

     ∀i ∈ M j ∈ J : if pij > t then xij = 0.

However, this is not a linear constraint.
    We will use the technique of parametric pruning to get around this diﬃ-
culty. The parameter will be T ∈ Z+ , which is our guess for a lower bound
on the optimal makespan. The parameter will enable us to prune away all
job–machine pairs such that pij > T . Deﬁne ST = {(i, j) | pij ≤ T }. We
will deﬁne a family of linear programs, LP(T ), one for each value of param-
eter T ∈ Z+ . LP(T ) uses the variables xij for (i, j) ∈ ST only, and asks if
there is a feasible, fractional schedule of makespan ≤ T using the restricted
possibilities.
           
                     xij = 1,          j∈J
        i:(i,j)∈ST
           
                     xij pij ≤ T,      i∈M
        j:(i,j)∈ST

        xij ≥ 0,                       (i, j) ∈ ST




17.2 Properties of extreme point solutions
Via an appropriate binary search, we will ﬁnd the smallest value of T such
that LP(T ) has a feasible solution. Let T ∗ be this value. Clearly, T ∗ is a
lower bound on OPT. The algorithm will round an extreme point solution to
142    17   Scheduling on Unrelated Parallel Machines

LP(T ∗ ) to ﬁnd a schedule having makespan ≤ 2T ∗ . Extreme point solutions
to LP(T ) have several useful properties.
Lemma 17.3 Any extreme point solution to LP(T ) has at most n + m
nonzero variables.

Proof: Let r = |ST | represent the number of variables on which LP(T ) is
deﬁned. Recall that a feasible solution to LP(T ) is an extreme point solution
iﬀ it corresponds to setting r linearly independent constraints of LP(T ) to
equality. Of these r linearly independent constraints, at least r − (n + m)
must be chosen from the third set of constraints (of the form xij ≥ 0). The
corresponding variables are set to 0. So, any extreme point solution has at
most n + m nonzero variables.                                                ✷
    Let x be an extreme point solution to LP(T ). We will say that job j is
integrally set in x if it is entirely assigned to one machine. Otherwise, we will
say that job j is fractionally set.
Corollary 17.4 Any extreme point solution to LP(T ) must set at least n−m
jobs integrally.

Proof: Let x be an extreme point solution to LP(T ), and let α and β be
the number of jobs that are integrally and fractionally set by x, respectively.
Each job of the latter kind is assigned to at least 2 machines and therefore
results in at least 2 nonzero entries in x. Hence we get

      α + β = n and α + 2β ≤ n + m.

Therefore, β ≤ m and α ≥ n − m.                                                ✷
     The LP-rounding algorithm is based on several interesting combinatorial
properties of extreme point solutions to LP(T ). Some of these are established
in Section 17.4. Corresponding to an extreme point solution x to LP(T ),
deﬁne G = (J, M, E) to be the bipartite graph on vertex set J ∪ M such that
(j, i) ∈ E iﬀ xij = 0. Let F ⊂ J be the set of jobs that are fractionally set
in x, and let H be the subgraph of G induced on vertex set F ∪ M . Clearly,
(i, j) is an edge in H iﬀ 0 < xij < 1. A matching in H will be called a perfect
matching if it matches every job j ∈ F . The rounding procedure uses the fact
that graph H has a perfect matching (see Lemma 17.7).


17.3 The algorithm
The algorithm starts by computing the range in which it ﬁnds the right value
of T . For this, it constructs the greedy schedule, in which each job is assigned
to the machine on which it has the smallest processing time. Let α be the
makespan of this schedule. Then the range is [α/m, α].
                   17.4   Additional properties of extreme point solutions    143


 Algorithm 17.5 (Scheduling on unrelated parallel machines)
  1. By a binary search in the interval [α/m, α], ﬁnd the smallest value of
     T ∈ Z+ for which LP(T ) has a feasible solution. Let this value be T ∗ .
  2. Find an extreme point solution, say x, to LP(T ∗ ).
  3. Assign all integrally set jobs to machines as in x.
  4. Construct graph H and ﬁnd a perfect matching M in it (e.g., using the
     procedure of Lemma 17.7).
  5. Assign fractionally set jobs to machines according to matching M.




17.4 Additional properties of extreme point solutions
We will say that a connected graph on vertex set V is a pseudo-tree if it
contains at most |V | edges. Since the graph is connected, it must have at
least |V | − 1 edges. So, it is either a tree or a tree plus a single edge. In the
latter case it has a unique cycle. Let us say that a graph is a pseudo-forest if
each of its connected components is a pseudo-tree. Recall that in Section 17.2
we deﬁned two graphs, G and H, corresponding to an extreme point solution
x to LP(T ).

Lemma 17.6 Graph G is a pseudo-forest.

Proof: We will show that the number of edges in each connected component
of G is bounded by the number of vertices in it. Hence, each connected
component is a pseudo-tree.
    Consider a connected component Gc . Restrict LP(T ) and x to the jobs
and machines of Gc only, to obtain LPc (T ) and xc . Let xc represent the
rest of x. The important observation is that xc must be an extreme point
solution to LPc (T ). Suppose that this is not the case. Then, xc is a convex
combination of two feasible solutions to LPc (T ). Each of these, together with
xc , form a feasible solution to LP(T ). Therefore, x is a convex combination
of two feasible solutions to LP(T ), leading to a contradiction.
    Now, applying Lemma 17.3, we get that Gc is a pseudo-tree.                ✷

Lemma 17.7 Graph H has a perfect matching.

Proof: Each job that is integrally set in x has exactly one edge incident at it
in G. Remove these jobs, together with their incident edges, from G. Clearly,
the remaining graph is H. Since an equal number of edges and vertices were
removed, H is also a pseudo-forest.
144    17       Scheduling on Unrelated Parallel Machines

                                   J                   M
                                                                    M
 M          J              M


                                           M                    J           J


      M                J       J                   J


                                                            M           M       M


                  M        M                       M




   In H, each job has a degree of at least 2. So, all leaves in H must be
machines. Keep matching a leaf with the job it is incident to, and remove
them both from the graph. (At each stage all leaves must be machines.) In the
end we will be left with even cycles (since we started with a bipartite graph).
Match oﬀ alternate edges of each cycle. This gives a perfect matching in H.
✷

Theorem 17.8 Algorithm 17.5 achieves an approximation guarantee of fac-
tor 2 for the problem of scheduling on unrelated parallel machines.

Proof: Clearly, T ∗ ≤ OPT, since LP(OPT) has a feasible solution. The
extreme point solution, x, to LP(T ∗ ) has a fractional makespan of ≤ T ∗ .
Therefore, the restriction of x to integrally set jobs has a (integral) makespan
of ≤ T ∗ . Each edge (i, j) of H satisﬁes pij ≤ T ∗ . The perfect matching found
in H schedules at most one extra job on each machine. Hence, the total
makespan is ≤ 2T ∗ ≤ 2 · OPT. The algorithm clearly runs in polynomial
time.                                                                          ✷

Example 17.9 Let us provide a family of tight examples. The mth instance
consists of m2 −m+1 jobs that need to be scheduled on m machines. The ﬁrst
job has a processing time of m on all machines, and all the remaining jobs
have unit processing time on each machine. The optimal schedule assigns
the ﬁrst job to one machine, and m of the remaining jobs to each of the
remaining m − 1 machines. Its makespan is m. It is easy to see that LP(T )
has no feasible solutions for T < m.
    Now suppose the following extreme point solution to LP(m) is picked. It
assigns 1/m of the ﬁrst job and m − 1 other jobs to each of the m machines.
Rounding will produce a schedule having a makespan of 2m − 1.           ✷



17.5 Exercises

17.1 Give an alternative proof of Lemma 17.7 by using Hall’s Theorem.
(This theorem states that a bipartite graph G = (U, V, E) has a matching
                                                            17.6   Notes    145

that matches all vertices of U iﬀ for every set U  ⊆ U , the neighborhood
of U  is at least as large as U  . The neighborhood of U  is {v ∈ V | ∃u ∈
U  with (u, v) ∈ E}.)
Hint: For any set F  ⊂ F , let M  be its neighborhood. Show that the graph
induced on F  ∪ M  must have at most |F  | + |M  | edges. On the other hand,
since each vertex in F has a degree of at least 2, this graph must have at
least 2|F  | edges.

17.2 Prove that the solution given to LP(m) in Example 17.9 is an extreme
point solution.

17.3 Does Algorithm 17.5 achieve a better factor than 2 for the special case
that the machines are identical?

17.4 Prove the following strengthening of Lemma 17.6. There is an extreme
point solution to LP(T ) such that its corresponding bipartite graph, G, is a
forest.

17.5 (Hochbaum and Shmoys [129]) Give a PTAS for the problem of min-
imizing makespan on uniform parallel machines. In this problem there is a
speed si associated with each machine i, and the processing time for job j
on machine i is pj /si .


17.6 Notes

The result of this chapter is due to Lenstra, Shmoys, and Tardos [184]. For
other LP-rounding based scheduling algorithms, see the survey by Hall [120].
