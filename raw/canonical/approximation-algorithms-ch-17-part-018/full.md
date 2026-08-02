---
title: "approximation-algorithms-ch-17-part-018"
year: null
source_type: paper
why_relevant: ""
acquisition_method: manual_upload
discovered_via: []
discovery_run: ""
triage_status: promoted
selected_by_user: true
acquired_at: ""
canonicalized_at: 2026-08-01
ingest_status: pending_ingest
pdf_path: "work/core-books/approximation-algorithms/parts/approximation-algorithms-ch-17-part-018.pdf"
raw_md: "raw/canonical/approximation-algorithms-ch-17-part-018/full.md"
---
## 17 Scheduling on Unrelated Parallel Machines

LP-rounding has yielded approximation algorithms for a large number of NP-hard problems in scheduling theory (see Section 17.6). As a illustrative example, we present a factor 2 algorithm for the problem of scheduling on unrelated parallel machines. We will apply the technique of parametric pruning, introduced in Chapter 5, together with LP-rounding, to obtain the algorithm.

Problem 17.1 (Scheduling on unrelated parallel machines) Given a set J of jobs, a set M of machines, and for each $j \in J$ and $i \in M , p _ { i j } \in { \bf Z } ^ { + }$ the time taken to process job j on machine i, the problem is to schedule the jobs on the machines so as to minimize the makespan, i.e., the maximum processing time of any machine. We will denote the number of jobs by n and the number of machines by m.

The reason for the name “unrelated” is that we have not assumed any relation between the processing times of a job on the diferent machines. If each job j has the same running time, say $p _ { j }$ , on each of the machines, then the machines are said to be identical. This problem was studied in Chapter 10 under the name minimum makespan scheduling, and we had derived a PTAS for it. A generalization of minimum makespan, that also admits a PTAS, is that of uniform parallel machines (see Exercise 17.5). In this case there is a speed $s _ { i }$ associated with each machine $i ,$ and the processing time for job j on machine i is $p _ { j } / s _ { i }$

## 17.1 Parametric pruning in an LP setting

An obvious integer program for this problem is the following. In this program $\boldsymbol { x } _ { i j }$ is an indicator variable denoting whether job $j$ is scheduled on machine i. The objective is to minimize t, the makespan. The first set of constraints ensures that each job is scheduled on one of the machines, and the second set ensures that each machine has a processing time of at most t.

$$
\begin{array}{l l} \text { minimize } & t \\ \text { subject   to } & \sum_ {i \in M} x _ {i j} = 1, \qquad j \in J \end{array}\tag{17.1}
$$

$$
\begin{array}{l l} \sum_ {j \in J} x _ {i j} p _ {i j} \leq t, & i \in M \\ x _ {i j} \in \{0, 1 \}, & i \in M, j \in J \end{array}
$$

We show below that this integer program has unbounded integrality gap.

Example 17.2 Suppose we have only one job, which has a processing time of m on each of the m machines. Clearly, the minimum makespan is m. However, the optimal solution to the linear relaxation is to schedule the job to the extent of 1/m on each machine, thereby leading to an objective function value of 1, and giving an integrality gap of m. ✷

This example is exploiting an “unfair” advantage that we have given to the linear relaxation. The integer program automatically sets $x _ { i j }$ to 0 if $p _ { i j } > t .$ On the other hand, the linear relaxation is allowed to set these variables to nonzero values, and thereby obtain a cheaper solution. The situation could be rectified if we could add the following constraint to the linear relaxation:

$$
\forall i \in M j \in J: \text {   if   } p _ {i j} > t \text {   then   } x _ {i j} = 0.
$$

However, this is not a linear constraint.

We will use the technique of parametric pruning to get around this dificulty. The parameter will be $T \in \mathbf { Z } ^ { + }$ , which is our guess for a lower bound on the optimal makespan. The parameter will enable us to prune away all job–machine pairs such that $p _ { i j } > T$ . Define $S _ { T } = \{ ( i , j ) \mid p _ { i j } \leq T \}$ . We will define a family of linear programs, LP(T), one for each value of parameter $T \in { \bf Z } ^ { + } . \ \mathrm { L P } ( T )$ uses the variables $x _ { i j }$ for $( i , j ) \in S _ { T }$ only, and asks if there is a feasible, fractional schedule of makespan ≤ T using the restricted possibilities.

$$
\begin{array}{l l} \sum_ {i: (i, j) \in S _ {T}} x _ {i j} = 1, & j \in J \\ \sum_ {j: (i, j) \in S _ {T}} x _ {i j} p _ {i j} \leq T, & i \in M \\ x _ {i j} \geq 0, & (i, j) \in S _ {T} \end{array}
$$

## 17.2 Properties of extreme point solutions

Via an appropriate binary search, we will find the smallest value of $T$ such that $\mathrm { L P } ( { \cal T } )$ has a feasible solution. Let $T ^ { * }$ be this value. Clearly, $T ^ { * }$ is a lower bound on OPT. The algorithm will round an extreme point solution to $\mathrm { L P } ( T ^ { * } )$ to find a schedule having makespan $\leq 2 T ^ { * }$ . Extreme point solutions to $\mathrm { L P } ( { \cal T } )$ have several useful properties.

Lemma 17.3 Any extreme point solution to $\mathrm { L P } ( { \cal T } )$ has at most $n + m$ nonzero variables.

Proof: Let $r = | S _ { T } |$ represent the number of variables on which $\mathrm { L P } ( { \cal T } )$ is defined. Recall that a feasible solution to $\mathrm { L P } ( { \cal T } )$ is an extreme point solution if it corresponds to setting r linearly independent constraints of $\mathrm { L P } ( { \cal T } )$ to equality. Of these r linearly independent constraints, at least $r - ( n + m )$ must be chosen from the third set of constraints (of the form $x _ { i j } \geq 0 )$ . The corresponding variables are set to 0. So, any extreme point solution has at most $n + m$ nonzero variables. ✷

Let x be an extreme point solution to $\mathrm { L P } ( { \cal T } )$ . We will say that job $j$ is integrally set in x if it is entirely assigned to one machine. Otherwise, we will say that job j is fractionally set.

Corollary 17.4 Any extreme point solution to $\mathrm { L P } ( { \cal T } )$ must set at least $n { - } m$ jobs integrally.

Proof: Let x be an extreme point solution to $\mathrm { L P } ( { \cal T } )$ , and let α and $\beta$ be the number of jobs that are integrally and fractionally set by ${ \mathbf { } } ^ { \mathbf { } } \mathbf { { \mathbf { x } } } ,$ respectively. Each job of the latter kind is assigned to at least 2 machines and therefore results in at least 2 nonzero entries in x. Hence we get

$$
\alpha + \beta = n \text { and } \alpha + 2 \beta \leq n + m.
$$

Therefore, $\beta \leq m$ and $\alpha \ge n - m$

The LP-rounding algorithm is based on several interesting combinatorial properties of extreme point solutions to $\mathrm { L P } ( { \cal T } )$ . Some of these are established in Section 17.4. Corresponding to an extreme point solution x to $\mathrm { L P } ( { \cal T } )$ define $G = ( J , M , E )$ to be the bipartite graph on vertex set $J \cup M$ such that $( j , i ) \in E$ if $x _ { i j } \neq 0$ . Let $F \subset J$ be the set of jobs that are fractionally set in x, and let H be the subgraph of G induced on vertex set $F \cup M$ . Clearly, $( i , j )$ is an edge in H if $0 < x _ { i j } < 1 .$ . A matching in H will be called a perfect matching if it matches every job $j \in F$ . The rounding procedure uses the fact that graph H has a perfect matching (see Lemma 17.7).

## 17.3 The algorithm

The algorithm starts by computing the range in which it finds the right value of T. For this, it constructs the greedy schedule, in which each job is assigned to the machine on which it has the smallest processing time. Let α be the makespan of this schedule. Then the range is $[ \alpha / m , \alpha ]$

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 17.5 (Scheduling on unrelated parallel machines)
1. By a binary search in the interval  $[\alpha/m, \alpha]$ , find the smallest value of  $T \in Z^{+}$  for which  $\mathrm{LP}(T)$  has a feasible solution. Let this value be  $T^{*}$ .
2. Find an extreme point solution, say x, to  $\mathrm{LP}(T^{*})$ .
3. Assign all integrally set jobs to machines as in x.
4. Construct graph H and find a perfect matching M in it (e.g., using the procedure of Lemma 17.7).
5. Assign fractionally set jobs to machines according to matching M.
</div>

## 17.4 Additional properties of extreme point solutions

We will say that a connected graph on vertex set V is a pseudo-tree if it contains at most |V| edges. Since the graph is connected, it must have at least $| V | - 1$ edges. So, it is either a tree or a tree plus a single edge. In the latter case it has a unique cycle. Let us say that a graph is a pseudo-forest if each of its connected components is a pseudo-tree. Recall that in Section 17.2 we defined two graphs, G and H, corresponding to an extreme point solution x to $\mathrm { L P } ( T )$

## Lemma 17.6 Graph G is a pseudo-forest.

Proof: We will show that the number of edges in each connected component of G is bounded by the number of vertices in it. Hence, each connected component is a pseudo-tree.

Consider a connected component $G _ { c } .$ . Restrict LP(T) and x to the jobs and machines of $G _ { c }$ only, to obtain $\mathrm { L P } _ { c } ( T )$ and $\mathbf { \delta _ { x } } _ { c }$ . Let x represent the rest of x. The important observation is that $\mathbf { \delta _ { x } } _ { c }$ must be an extreme point solution to $\mathrm { L P } _ { c } ( T )$ . Suppose that this is not the case. Then, $\mathbf { \delta _ { x } } _ { c }$ is a convex combination of two feasible solutions to $\mathrm { L P } _ { c } ( T )$ . Each of these, together with ${ \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } } { \mathbf { } \mathbf { } } { \mathbf { } } { \mathbf } { \mathbf { } } { \mathbf } { \mathbf { } } \mathbf { } \mathbf { } { \Sigma } \mathbf { } \mathbf { } \mathbf { } \mathbf { } \Sigma  { \mathbf } { \mathbf } { \mathbf } { \Sigma } \mathbf { } \mathbf { } \mathbf { } \mathbf \Sigma \Sigma  { \Sigma \Sigma } \mathbf { \Sigma } \Sigma \Sigma \Sigma \Sigma \Sigma \Sigma \Sigma \Sigma \Sigma \Sigma \Sigma \Sigma \Sigma \Sigma \Sigma \Sigma \Sigma \Sigma $ form a feasible solution to $\mathrm { L P } ( T )$ . Therefore, x is a convex combination of two feasible solutions to $\mathrm { L P } ( { \cal T } )$ , leading to a contradiction.

Now, applying Lemma 17.3, we get that $G _ { c }$ is a pseudo-tree.

## Lemma 17.7 Graph H has a perfect matching.

Proof: Each job that is integrally set in x has exactly one edge incident at it in G. Remove these jobs, together with their incident edges, from G. Clearly, the remaining graph is H. Since an equal number of edges and vertices were removed, H is also a pseudo-forest.

![](images/c23bede5279317035ca1d501fff7c580fc94af8e83cfc00042318fc22e5885d2.jpg)

In H, each $\mathrm { j o b }$ has a degree of at least 2. ${ \mathrm { S o } } ,$ all leaves in H must be machines. Keep matching a leaf with the job it is incident to, and remove them both from the graph. (At each stage all leaves must be machines.) In the end we will be left with even cycles (since we started with a bipartite graph). Match of alternate edges of each cycle. This gives a perfect matching in H. ✷

Theorem 17.8 Algorithm 17.5 achieves an approximation guarantee of factor 2 for the problem of scheduling on unrelated parallel machines.

Proof: Clearly, $T ^ { \ast } \leq \mathrm { O P T }$ , since $\mathrm { L P ( O P T ) }$ has a feasible solution. The extreme point solution, x, to $\mathrm { L P } ( T ^ { * } )$ has a fractional makespan of $\leq T ^ { * }$ Therefore, the restriction of x to integrally set jobs has a (integral) makespan of $\leq T ^ { * }$ . Each edge $( i , j )$ of H satisfies $p _ { i j } \leq T ^ { * }$ . The perfect matching found in H schedules at most one extra job on each machine. Hence, the total makespan is $\leq 2 T ^ { * } \leq 2 \cdot \mathrm { O P T }$ . The algorithm clearly runs in polynomial time. ✷

Example 17.9 Let us provide a family of tight examples. The mth instance consists of $m ^ { 2 } - m + 1$ jobs that need to be scheduled on m machines. The first job has a processing time of m on all machines, and all the remaining jobs have unit processing time on each machine. The optimal schedule assigns the first job to one machine, and m of the remaining jobs to each of the remaining $m - 1$ machines. Its makespan is m. It is easy to see that $\mathrm { L P } ( T )$ has no feasible solutions for $T < m$

Now suppose the following extreme point solution to $\mathrm { L P } ( m )$ is picked. It assigns $1 / m$ of the first job and $m - 1$ other jobs to each of the m machines. Rounding will produce a schedule having a makespan of $2 m - 1$ ✷

## 17.5 Exercises

17.1 Give an alternative proof of Lemma 17.7 by using Hall’s Theorem. (This theorem states that a bipartite graph $G = ( U , V , E )$ has a matching that matches all vertices of U if for every set $U ^ { \prime } \subseteq U$ , the neighborhood of $U ^ { \prime }$ is at least as large as $U ^ { \prime } .$ The neighborhood of $U ^ { \prime }$ is $\{ v \in V \mid$ ∃u $\in$ $U ^ { \prime }$ with $( u , v ) \in E \} . \quad$ )

Hint: For any set $F ^ { \prime } \subset F$ , let $M ^ { \prime }$ be its neighborhood. Show that the graph induced on $F ^ { \prime } \cup M ^ { \prime }$ must have at most $\vert F ^ { \prime } \vert + \vert M ^ { \prime } \vert$ edges. On the other hand, since each vertex in $F$ has a degree of at least 2, this graph must have at least $2 | F ^ { \prime } |$ edges.

17.2 Prove that the solution given to $\mathrm { L P } ( m )$ in Example 17.9 is an extreme point solution.

17.3 Does Algorithm 17.5 achieve a better factor than 2 for the special case that the machines are identical?

17.4 Prove the following strengthening of Lemma 17.6. There is an extreme point solution to $\mathrm { L P } ( { \cal T } )$ such that its corresponding bipartite graph, $G ,$ is a forest.

17.5 (Hochbaum and Shmoys [129]) Give a PTAS for the problem of minimizing makespan on uniform parallel machines. In this problem there is a speed $s _ { i }$ associated with each machine $i ,$ and the processing time for job j on machine i is $p _ { j } / s _ { i }$

## 17.6 Notes

The result of this chapter is due to Lenstra, Shmoys, and Tardos [184]. For other LP-rounding based scheduling algorithms, see the survey by Hall [120].