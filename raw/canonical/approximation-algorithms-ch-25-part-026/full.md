---
title: "approximation-algorithms-ch-25-part-026"
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
pdf_path: "work/core-books/approximation-algorithms/parts/approximation-algorithms-ch-25-part-026.pdf"
raw_md: "raw/canonical/approximation-algorithms-ch-25-part-026/full.md"
---
The k-median problem difers from the facility location problem in two respects – there is no cost for opening facilities and there is an upper bound, $k ,$ on the number of facilities that can be opened. It models the problem of finding a minimum cost clustering, and therefore has numerous applications.

The primal–dual schema works by making judicious local improvements and is not suitable for ensuring a global constraint, such as the constraint in the k-median problem that at most k facilities be opened. We will get around this dificulty by borrowing the powerful technique of Lagrangian relaxation from combinatorial optimization.

Problem 25.1 (Metric k-median) Let G be a bipartite graph with bipartition $( F , C )$ , where F is the set of facilities and C is the set of $_ { c i t i e s , }$ , and let k be a positive integer specifying the number of facilities that are allowed to be opened. Let $c _ { i j }$ be the cost of connecting city j to (opened) facility i. The connection costs satisfy the triangle inequality. The problem is to find a subset $I \subseteq F , | I | \leq k ,$ of facilities that should be opened and a function $\phi : \ C \to I$ assigning cities to open facilities in such a way that the total connecting cost is minimized.

## 25.1 LP-relaxation and dual

The following is an integer program for the k-median problem. The indicator variables $y _ { i }$ and $x _ { i j }$ play the same role as in (24.1).

$$
\begin{array}{l l l} \text {minimize} & \sum_ {i \in F, j \in C} c _ {i j} x _ {i j} \\ \text {subject to} & \sum_ {i \in F} x _ {i j} \geq 1, & j \in C \\ & y _ {i} - x _ {i j} \geq 0, & i \in F, j \in C \\ & \sum_ {i \in F} - y _ {i} \geq - k \\ & x _ {i j} \in \{0, 1 \}, & i \in F, j \in C \\ & y _ {i} \in \{0, 1 \}, & i \in F \end{array}\tag{25.1}
$$

The LP-relaxation of this program is:

$$
\begin{array}{l l l} \text {minimize} & \sum_ {i \in F, j \in C} c _ {i j} x _ {i j} \\ \text {subject to} & \sum_ {i \in F} x _ {i j} \geq 1, & j \in C \\ & y _ {i} - x _ {i j} \geq 0, & i \in F, j \in C \\ & \sum_ {i \in F} - y _ {i} \geq - k \\ & x _ {i j} \geq 0, & i \in F, j \in C \\ & y _ {i} \geq 0, & i \in F \end{array}\tag{25.2}
$$

The dual program is:

$$
\begin{array}{l l l} \text {maximize} & \sum_ {j \in C} \alpha_ {j} - z k \\ \text {subject to} & \alpha_ {j} - \beta_ {i j} \leq c _ {i j}, & i \in F, j \in C \\ & \sum_ {j \in C} \beta_ {i j} \leq z, & i \in F \\ & \alpha_ {j} \geq 0, & j \in C \\ & \beta_ {i j} \geq 0, & i \in F, j \in C \\ & z \geq 0 \end{array}\tag{25.3}
$$

## 25.2 The high-level idea

The similarity between the two problems, facility location and k-median, leads to a similarity in their linear programs, which will be exploited as follows. Take an instance of the k-median problem, assign a cost of z for opening each facility, and find optimal solutions to LP (24.2) and LP (24.3), say $( { \pmb x } , { \pmb y } )$ and $( \alpha , \beta )$ , respectively. By the strong duality theorem,

$$
\sum_ {i \in F, j \in C} c _ {i j} x _ {i j} + \sum_ {i \in F} z y _ {i} = \sum_ {j \in C} \alpha_ {j}.
$$

Now, suppose that the primal solution $( { \pmb x } , { \pmb y } )$ happens to open exactly k facilities (fractionally), i.e., $\begin{array} { r } { \sum _ { i } y _ { i } = k . } \end{array}$ Then, we claim that $( { \pmb x } , { \pmb y } )$ and $( \alpha , \beta , z )$ are optimal solutions to LP (25.2) and LP (25.3), respectively. Feasibility is easy to check. Optimality follows by substituting $\textstyle \sum _ { i } y _ { i } \ = \ k$ in the above equality and rearranging terms to show that the primal and dual solutions achieve the same objective function value:

$$
\sum_ {i \in F, j \in C} c _ {i j} x _ {i j} = \sum_ {j \in C} \alpha_ {j} - z k.
$$

Let’s use this idea, together with Algorithm 24.2 and Theorem $2 4 . 7 ,$ to obtain a “good” integral solution to LP (25.2). Suppose with a cost of z for opening each facility, Algorithm 24.2, happens to find solutions $( { \pmb x } , { \pmb y } )$ and $( \alpha , \beta )$ , where the primal solution opens exactly k facilities. By Theorem 24.7,

$$
\sum_ {i \in F, j \in C} c _ {i j} x _ {i j} + 3 z k \leq 3 \sum_ {j \in C} \alpha_ {j}.
$$

Now, observe that $( { \pmb x } , { \pmb y } )$ and $( \alpha , \beta , z )$ are primal (integral) and dual feasible solutions to the k-median problem satisfying

$$
\sum_ {i \in F, j \in C} c _ {i j} x _ {i j} \leq 3 (\sum_ {j \in C} \alpha_ {j} - z k).
$$

Therefore, $( { \pmb x } , { \pmb y } )$ is a solution to the k-median problem within thrice the optimal.

Notice that the factor 3 proof given above would not work if less than k facilities were opened; if more than k facilities are opened, the solution is infeasible for the k-median problem. The remaining problem is to find a value of z so that exactly k facilities are opened. Several ideas are required for this. The first is the following principle from economics. Taxation is an efective way of controlling the amount of goods coming across a border – raising tarifs will reduce inflow and vice versa. In a similar manner, raising z should reduce the number of facilities opened and vice versa.

It is natural now to seek a modification to Algorithm 24.2 that can find a value of z so that exactly k facilities are opened. This would lead to a factor 3 approximation algorithm. Such a modification is not known. Instead, we present the following strategy which leads to a factor 6 algorithm. For the rest of the discussion, assume that we never encountered a run of the algorithm which resulted in exactly k facilities being opened.

Clearly, when $z = 0$ the algorithm will open all facilities, and when z is very large it will open only one facility. The latter value of z can be picked to be $n c _ { \mathrm { m a x } }$ , where $c _ { \mathrm { m a x } }$ is the length of the longest edge. We will conduct a binary search on the interval $[ 0 , n c _ { \mathrm { m a x } } ]$ to find $z _ { 2 }$ and z<sub>1</sub> for which the algorithm opens $k _ { 2 } >$ k and $k _ { 1 } < k$ facilities, respectively, and, furthermore, $z _ { 1 } - z _ { 2 } \leq ( c _ { \operatorname* { m i n } } / 1 2 n _ { c } ^ { 2 } )$ , where $c _ { \mathrm { m i n } }$ is the length of the shortest nonzero edge. As before, we will adopt the following notation: $n _ { c } = | C |$ and $n _ { f } ~ = ~ | F |$ The total number of vertices $n _ { c } + n _ { f } = n$ and the total number of edges $n _ { c } \times n _ { f } = m$ . Let $( \boldsymbol x ^ { s } , \boldsymbol y ^ { s } )$ and $( \pmb { x } ^ { l } , \pmb { y } ^ { l } )$ be the two primal solutions found, with $\textstyle \sum _ { i \in F } y _ { i } ^ { s } = k _ { 1 }$ and $\textstyle \sum _ { i \in F } y _ { i } ^ { l } = k _ { 2 }$ (the superscripts s and l denote “small” and $\mathrm { ^ { * } l a r g e , } ^ { \mathrm { * } }$ respectively). Further, let $( \alpha ^ { s } , \beta ^ { s } )$ and $( \alpha ^ { l } , \beta ^ { l } )$ be the corresponding dual solutions found.

Let $( { \pmb x } , { \pmb y } ) = a ( { \pmb x } ^ { s } , { \pmb y } ^ { s } ) + b ( { \pmb x } ^ { l } , { \pmb y } ^ { l } )$ be a convex combination of these two solutions, with $a k _ { 1 } + b k _ { 2 } = k$ . Under these conditions, $a = ( k _ { 2 } - k ) / ( k _ { 2 } - k _ { 1 } )$ and $b = ( k - k _ { 1 } ) / ( k _ { 2 } - k _ { 1 } )$ . Since $( { \pmb x } , { \pmb y } )$ is a feasible (fractional) solution to the facility location problem that opens exactly k facilities, it is also a feasible (fractional) solution to the k-median problem. In this solution each city is connected to at most two facilities.

Lemma 25.2 The cost of (x, y) is within a factor of $\left( 3 + 1 / n _ { c } \right)$ of the cost of an optimal fractional solution to the k-median problem.

Proof: By Theorem 24.7 we have

$$
\sum_ {i \in F, j \in C} c _ {i j} x _ {i j} ^ {s} \leq 3 (\sum_ {j \in C} \alpha_ {j} ^ {s} - z _ {1} k _ {1}),
$$

and

$$
\sum_ {i \in F, j \in C} c _ {i j} x _ {i j} ^ {l} \leq 3 (\sum_ {j \in C} \alpha_ {j} ^ {l} - z _ {2} k _ {2}).
$$

Since $z _ { 1 } > z _ { 2 } , ( \alpha ^ { l } , \beta ^ { l } )$ is a feasible dual solution to the facility location problem even if the cost of facilities is $z _ { 1 }$ . We would like to replace $z _ { 2 }$ with $z _ { 1 }$ in the second inequality, at the expense of the increased factor. This is achieved using the upper bound on $z _ { 1 } - z _ { 2 }$ and the fact that $\begin{array} { r } { \sum _ { i \in F , \ j \in C } c _ { i j } x _ { i j } ^ { l } \ \ge \ c _ { \operatorname* { m i n } } , } \end{array}$ We get

$$
\sum_ {i \in F, j \in C} c _ {i j} x _ {i j} ^ {l} \leq \left(3 + \frac {1}{n _ {c}}\right) \left(\sum_ {j \in C} \alpha_ {j} ^ {l} - z _ {1} k _ {2}\right).
$$

Adding this inequality multiplied by b with the first inequality multiplied by a gives

$$
\sum_ {i \in F, j \in C} c _ {i j} x _ {i j} \leq \left(3 + \frac {1}{n _ {c}}\right) \left(\sum_ {j \in C} \alpha_ {j} - z _ {1} k\right),
$$

where $\pmb { \alpha } = a \pmb { \alpha } ^ { s } + b \pmb { \alpha } ^ { l }$ . Let $\beta = a \beta ^ { s } + b \beta ^ { l }$ . Observe that $( \alpha , \beta , z _ { 1 } )$ is a feasible solution to the dual of the k-median problem. The lemma follows. ✷

In Section 25.3 we give a randomized rounding procedure that obtains an integral solution to the k-median problem from $( { \pmb x } , { \pmb y } )$ , with a small increase in cost. In Section 25.3.1 we derandomize this procedure.

## 25.3 Randomized rounding

We give a randomized rounding procedure that produces an integral solution to the k-median problem from $( { \pmb x } , { \pmb y } )$ . In the process, it increases the cost by a multiplicative factor of $1 + \operatorname* { m a x } ( a , b )$

Let A and B be the sets of facilities opened in the two solutions, $| A | = k _ { 1 }$ and $| B | = k _ { 2 }$ . For each facility in A, find the closest facility in $B \mathrm { ~ - ~ }$ these facilities are not required to be distinct. Let $B ^ { \prime } \subset B$ be these facilities. If $| B ^ { \prime } | < k _ { 1 }$ , arbitrarily include additional facilities from $B - B ^ { \prime }$ into $B ^ { \prime }$ until $| B ^ { \prime } | = k _ { 1 }$

With probability $^ { a , }$ open all facilities in A, and with probability $b = 1 - a$ open all facilities in $B ^ { \prime }$ . In addition, a set of cardinality $k - k _ { 1 }$ is picked randomly from $B - B ^ { \prime }$ and facilities in this set are opened. Notice that each facility in $B - B ^ { \prime }$ has a probability of b of being opened. Let I be the set of facilities opened, $| I | = k .$

The function $\phi : C \to I$ is defined as follows. Consider city $j$ and suppose that it is connected to $i _ { 1 } \in A$ and $i _ { 2 } \in B$ in the two solutions. If $i _ { 2 } \in B ^ { \prime }$ , then one of $i _ { 1 }$ and $i _ { 2 }$ is opened by the procedure given above, $i _ { 1 }$ with probability a and $i _ { 2 }$ with probability b. City j is connected to the open facility.

![](images/2c4638eed4b19e96e43a8256bccbbc092968341f7cf122a9aecf5fbaee048f70.jpg)

If $i _ { 2 } \in B - B ^ { \prime }$ , let $i _ { 3 } \in B ^ { \prime }$ be the facility in B that is closest to $i _ { 1 }$ . City $j$ is connected to $i _ { 2 }$ if it is open. Otherwise, it is connected to $i _ { 1 }$ if it is open. If neither $i _ { 2 }$ or $i _ { 1 }$ is open, then $j$ is connected to $i _ { 3 }$

Denote by cost(j) the connection cost for city j in the fractional solution $( { \pmb x } , { \pmb y } ) ;$ cos $\mathbf { \chi } ( j ) = a c _ { i _ { 1 } j } + b c _ { i _ { 2 } j }$

Lemma 25.3 The expected connection cost for city j in the integral solution, $\mathbf { E } [ c _ { \phi ( j ) j } ]$ , is ≤ (1 + max(a, b))cost(j). Moreover, $\mathbf { E } [ c _ { \phi ( j ) j } ]$ can be eficiently computed.

Proof: If $i _ { 2 } \in B ^ { \prime } , \mathbf { E } [ c _ { \phi ( j ) j } ] = a c _ { i _ { 1 } j } + b c _ { i _ { 2 } j } = \mathrm { c o s t } ( j )$ . Consider the second case, that $i _ { 2 } \notin B ^ { \prime }$ . Now, i<sub>2</sub> is open with probability b. The probability that $i _ { 2 }$ is not open and $i _ { 1 }$ is open is $( 1 - b ) a = a ^ { 2 }$ , and the probability that both $i _ { 2 }$ and $i _ { 1 }$ are not open is $( 1 - b ) ( 1 - a ) = a b$ . This gives

$$
\mathbf {E} [ c _ {\phi (j) j} ] \leq b c _ {i _ {2} j} + a ^ {2} c _ {i _ {1} j} + a b c _ {i _ {3} j}.
$$

Since $i _ { 3 }$ is the facility in B that is closest to ${ i _ { 1 } , c _ { i _ { 1 } i _ { 3 } } \leq c _ { i _ { 1 } i _ { 2 } } \leq c _ { i _ { 1 } j } + c _ { i _ { 2 } j } }$ where the second inequality follows from the triangle inequality. Again, by the triangle inequality, $c _ { i _ { 3 } j } \leq c _ { i _ { 1 } j } + c _ { i _ { 1 } i _ { 3 } } \leq 2 c _ { i _ { 1 } j } + c _ { i _ { 2 } j }$ . Therefore,

$$
\mathbf {E} [ c _ {\phi (j) j} ] \leq b c _ {i _ {2} j} + a ^ {2} c _ {i _ {1} j} + a b (2 c _ {i _ {1} j} + c _ {i _ {2} j}).
$$

Now, $a ^ { 2 } c _ { i _ { 1 } j } + a b c _ { i _ { 1 } j } = a c _ { i _ { 1 } j }$ . Therefore,

$$
\begin{array}{r l} & {\mathbf {E} [ c _ {\phi (j) j} ] \leq (a c _ {i _ {1} j} + b c _ {i _ {2} j}) + a b (c _ {i _ {1} j} + c _ {i _ {2} j})} \\ & {\qquad \leq (a c _ {i _ {1} j} + b c _ {i _ {2} j}) (1 + \max (a, b)).} \end{array}
$$

Clearly, in both cases, $\mathbf { E } [ c _ { \phi ( j ) j } ]$ is easy to compute.

Let $( \boldsymbol { x } ^ { k } , \boldsymbol { y } ^ { k } )$ denote the integral solution obtained to the k-median problem by this randomized rounding procedure. Then,

Lemma 25.4

$$
\mathbf {E} \left[ \sum_ {i \in F, j \in C} c _ {i j} x _ {i j} ^ {k} \right] \leq (1 + \max (a, b)) \left(\sum_ {i \in F, j \in C} c _ {i j} x _ {i j}\right)
$$

and, moreover, the expected cost of the solution found can be computed $e f f -$ ciently.

## 25.3.1 Derandomization

Derandomization follows in a straightforward manner using the method of conditional expectation. First, the algorithm opens the set A with probability a and the set $B ^ { \prime }$ with probability $b = 1 { - } a . \operatorname { P i c k } A$ , and compute the expected value if $k - k _ { 1 }$ facilities are randomly chosen from $B - B ^ { \prime }$ . Next, do the same by picking $B ^ { \prime }$ instead of A. Choose to open the set that gives the smaller expectation.

Second, the algorithm opens a random subset of $k - k _ { 1 }$ facilities from $B - B ^ { \prime }$ . For a choice $D \subset B - B ^ { \prime } , | D | \leq k - k _ { 1 }$ , denote by $\mathbf { E } [ D , B - ( B ^ { \prime } \cup D ) ]$ the expected cost of the solution if all facilities in D and additionally k − $k _ { 1 } - | D |$ facilities are randomly opened from $B - ( B ^ { \prime } \cup D )$ ). Since each facility of $B - ( B ^ { \prime } \cup D )$ is equally likely to be opened, we get

$$
\frac {\mathbf {E} [ D , B - (B ^ {\prime} \cup D) ] =}{\frac {1}{| B - (B ^ {\prime} \cup D) |} \sum_ {i \in B - (B ^ {\prime} \cup D)} \mathbf {E} [ D \cup \{i \}, B - (B ^ {\prime} \cup D \cup \{i \}) ].}
$$

This implies that there is an i such that

$$
\mathbf {E} [ D \cup \{i \}, B - (B ^ {\prime} \cup D \cup \{i \}) ] \leq \mathbf {E} [ B ^ {\prime}, B - (B ^ {\prime} \cup D) ].
$$

Choose such an i and replace D with $D \cup \{ i \}$ . Notice that the computation of $\mathbf { E } [ D \cup \{ i \} , B - ( B ^ { \prime } \cup D \cup \{ i \} ) ]$ ] can be done as in Lemma 25.4.

## 25.3.2 Running time

It is easy to see that $a \leq 1 - 1 / n _ { c }$ (this happens for $k _ { 1 } = k - 1$ and $k _ { 2 } = n _ { c } )$ and $b \ \leq \ 1 - 1 / k$ (this happens for $k _ { 1 } ~ = ~ 1$ and $k _ { 2 } ~ = ~ k + 1 )$ . Therefore, 1 + max $\left( a , b \right) \leq 2 - 1 / n _ { c }$ . Altogether, the approximation guarantee is $( 2 \textrm { -- }$ $1 / n _ { c } ) ( 3 + 1 / n _ { c } ) < 6$ . This procedure can be derandomized using the method of conditional probabilities, as in Section 25.3.1. The binary search will make $O ( \log _ { 2 } ( n ^ { 3 } c _ { \mathrm { m a x } } / c _ { \mathrm { m i n } } ) ) = O ( L +$ log n) probes. The running time for each probe is dominated by the time taken to run Algorithm 24.2; randomized rounding takes $O ( n )$ time and derandomization takes $O ( m )$ time. Hence we get

Theorem 25.5 The algorithm given above achieves an approximation factor of 6 for the k-median problem, and has a running time of O(m log m(L + log(n))).

## 25.3.3 Tight example

A tight example for the factor 6 k-median algorithm is not known. However, below we give an infinite family of instances which show that the analysis of the randomized rounding procedure cannot be improved.

The two solutions $( \boldsymbol x ^ { s } , \boldsymbol y ^ { s } )$ and $( \pmb { x } ^ { l } , \pmb { y } ^ { l } )$ open one facility, $f _ { 0 } ,$ and $k + 1$ facilities, $f _ { 1 } , \ldots , f _ { k + 1 }$ respectively. The distance between $f _ { 0 }$ and any other $f _ { i }$ is 1, and that between two facilities in the second set is 2. All n cities are at a distance of 1 from $f _ { 0 } ,$ , and at a distance of ε from $f _ { k + 1 }$ . The rest of the distances are given by the triangle inequality. The convex combination is constructed with $a = 1 / k$ and $b = 1 - 1 / k$

![](images/03eb34b467507e0cb0a1eb1341564bfefef2697052a61ccaaeb765f1f70d44d6.jpg)

Now, the cost of the convex combination is $a n + b \varepsilon n$ . Suppose the algorithm picks $f _ { 1 }$ as the closest neighbor of $f _ { 0 }$ . The expected cost of the solutions produced by the randomized rounding procedure is then $n ( b \varepsilon + a ^ { 2 } + a b ( 2 + \varepsilon ) )$ Letting ε tend to $0 ,$ the cost of the convex combination is essentially na and that of the rounded solution is $n a ( 1 + b )$

## 25.3.4 Integrality gap

The algorithm given above places an upper bound of 6 on the integrality gap of relaxation (25.2). The following example places a lower bound of essentially 2. The graph is a star with $n + 1$ vertices and unit cost edges. F consists of all $n + 1$ vertices, $C$ consists of all but the center vertex and $k = n - 2$ . An optimal integral solution is to open facilities at $n - 2$ vertices of C and has a cost of 2. Consider the following fractional solution. Open a facility to the extent of $1 / ( n - 1 )$ on the center vertex and $( n - 2 ) / ( n - 1 )$ on each vertex of $C .$ This has a cost of $n / ( n - 1 )$ , giving a ratio of $2 ( n - 1 ) / n$

![](images/ee779f8f85725e6d11d4f239998e9935580e614a87b2a7f2c082c26226b04e79.jpg)

## 25.4 A Lagrangian relaxation technique for approximation algorithms

In this section we will abstract away the ideas developed above so they may be more widely applicable. First, let us recall the fundamental technique of Lagrangian relaxation from combinatorial optimization. This technique consists of relaxing a constraint by moving it into the objective function, together with an associated Lagrange multiplier.

Let us apply this relaxation to the constraint, in the k-median IP (25.1), that at most k facilities be opened. Let λ be the Lagrangian multiplier.

$$
\begin{array}{l l} \text { minimize } & \sum_ {i \in F, j \in C} c _ {i j} x _ {i j} + \lambda \left(\sum_ {i \in F} y _ {i} - k\right) \\ \text { subject   to } & \sum_ {i \in F} x _ {i j} \geq 1, \quad j \in C \end{array}\tag{25.4}
$$

$$
y _ {i} - x _ {i j} \geq 0,
$$

$$
x _ {i j} \in \{0, 1 \},
$$

$$
i \in F, j \in C
$$

$$
i \in F, j \in C
$$

$$
y _ {i} \in \{0, 1 \},
$$

$$
i \in F
$$

This is precisely the facility location IP, with the restriction that the cost of each facility is the same, i.e., λ. It contains an additional constant term of −λk in the objective function. We may assume w.l.o.g. that an optimal solution, (x, y), to IP (25.1) opens exactly k facilities. Now, (x, y) is a feasible solution to IP (25.4) as well, with the same objective function value. Hence, for each value of λ, IP (25.4) is a lower bound on IP (25.1).

We have shown that a Lagrangian relaxation of the k-median problem is the facility location problem. In doing so, the global constraint that at most k facilities be opened has been replaced with a penalty for opening facilities, the penalty being the Lagrangian multiplier. (See Exercise 25.4 for another application of this idea.)

The next important observation was to notice that in the facility location approximation algorithm, Theorem 24.7, the duals pay one-for-one for the cost of opening facilities, i.e., with approximation factor 1. (See Exercise 22.9 for another such algorithm.)

The remaining dificulty was finding a value of λ so that the facility location algorithm opened exactly k facilities. The fact that the facility location algorithm works with the linear relaxation of the problem helped. The convex combination of two (integer) solutions was a feasible (fractional) solution. The last step was rounding this (special) fractional solution into an integral one. For the k-median problem we used randomized rounding (see Exercise 25.4 for a diferent rounding procedure).

## 25.5 Exercises

25.1 (Lin and Vitter [188]) Consider the general k-median problem in which the connection costs are not required to satisfy the triangle inequality. Give a reduction from the set cover problem to show that approximating this problem is as hard as approximating set cover, and therefore cannot be done with a factor better than O(log n) unless ${ \bf N P } \subseteq \tilde { \bf P }$

25.2 Obtain the dual of LP-relaxation to (25.4). (The constant term in the objective function will simply carry over.) How does it relate with the dual of the k-median LP?

25.3 Use the Lagrangian relaxation technique to give a constant factor approximation algorithm for the following common generalization of the facility location and k-median problems. Consider the uncapacitated facility location problem with the additional constraint that at most k facilities can be opened. This is a common generalization of the two problems solved in this paper: if k is made $n _ { f }$ , we get the first problem, and if the facility costs are set to zero, we get the second problem.

25.4 (Garg [94] and Chudak, Roughgarden, and Williamson [47]) Consider the following variant of the metric Steiner tree problem.

Problem 25.6 (Metric k-MST) We are given a complete undirected graph $G = ( V , E )$ , a special vertex $r \in V$ , a positive integer $k ,$ and a function cost : $E  \mathbf { Q } ^ { + }$ satisfying the triangle inequality. The problem is to find a minimum cost tree containing exactly k vertices, including $^ r .$

We will develop a factor 5 algorithm for this problem.

1. Observe that a Lagrangian relaxation of this problem is the prizecollecting Steiner tree problem, Problem 22.12, stated in Exercise 22.9.

2. Observe that the approximation algorithm for the latter problem, given in Exercise 22.9, pays for the penalties one-for-one with the dual, i.e., with an approximation factor of 1.

3. Use the prize-collecting algorithm as a subroutine to obtain two trees, $T _ { 1 }$ and $T _ { 2 }$ , for very close values of the penalty, containing $k _ { 1 }$ and $k _ { 2 }$ vertices, with $k _ { 1 } < k < k _ { 2 }$ . Obtain a convex combination of these solutions, with multipliers $\alpha _ { 1 }$ and $\alpha _ { 2 }$

4. We may assume that every vertex in $G$ is at a distance of $\leq \mathrm { O P T }$ from r. (Use the idea behind parametric pruning, introduced in Chapter $5 .$ The parameter t is the length of the longest edge used by the optimal solution, which is clearly a lower bound on $\mathrm { O P T }$ . For each value of $t ,$ instance $G ( t )$ is obtained by restricting $G$ to vertices that are within a distance of t of $^ { r } \cdot$ The algorithm is run on each graph of this family, and the best tree is output.) Consider the following procedure for rounding the convex combination. If $\alpha _ { 2 } \geq 1 / 2$ , then cost $( T _ { 2 } ) \le 4 \cdot \mathrm { O P T }$ ; remove $k _ { 2 } - k$ vertices from $T _ { 2 }$ . Otherwise, double every edge of $T _ { 2 }$ , find an Euler tour, and shortcut the tour to a cycle containing only those vertices that are in $T _ { 2 }$ and not in $T _ { 1 } ~ ( \mathrm { i . e . }$ , at most $k _ { 2 } - k _ { 1 }$ vertices). Pick the cheapest path of length $k - k _ { 1 } - 1$ from this cycle, and connect it by means of an edge to vertex $r$ in $T _ { 1 }$ . The resulting tree has exactly k vertices. Show that the cost of this tree is $\leq 5 \cdot \mathrm { O P T }$

Hint: Use the fact that $\alpha _ { 2 } = ( k - k _ { 1 } ) / ( k _ { 2 } - k _ { 1 } )$

25.5 Let us apply the Lagrangian relaxation technique to the following linear program.

$$
\mathrm{minimize} \qquad \pmb {c} ^ {T} \pmb {x}
$$

$$
\text { subject   to } \quad A x = b\tag{25.5}
$$

Then the lower bound is given by

$$
\max _ {\boldsymbol {y}} \min _ {\boldsymbol {x}} \left(\boldsymbol {c} ^ {T} \boldsymbol {x} - \boldsymbol {y} ^ {T} (\boldsymbol {A} \boldsymbol {x} - \boldsymbol {b})\right) = \max _ {\boldsymbol {y}} \left(\left(\min _ {\boldsymbol {x}} (\boldsymbol {c} ^ {T} - \boldsymbol {y} ^ {T} \boldsymbol {A}) \boldsymbol {x}\right) + \boldsymbol {y} ^ {T} \boldsymbol {b}\right)
$$

If y does not satisfy $\mathbf { A } ^ { T } \pmb { y } = \pmb { c }$ , then by a suitable choice of ${ \mathbf { } } ^ { \mathbf { } } \mathbf { { \mathbf { x } } } ,$ the lower bound given by this expression can be made as small as desired and therefore meaningless. Meaningful lower bounds arise only if we insist that $\mathbf { A } ^ { T } \pmb { y } = \pmb { c }$ But then we get the following LP:

$$
\begin{array}{l l} \text {maximize} & \boldsymbol {y} ^ {T} \boldsymbol {b} \\ \text {subject to} & \boldsymbol {A} ^ {T} \boldsymbol {y} = \boldsymbol {c} \end{array}\tag{25.6}
$$

Notice that this is the dual of LP (25.5)! Hence, the Lagrangian relaxation of a linear program is simply its dual and is therefore tight.

Obtain the Lagrangian relaxation of the following LP:

$$
\begin{array}{l l} \text {minimize} & \boldsymbol {c} ^ {T} \boldsymbol {x} \\ \text {subject to} & \boldsymbol {A} \boldsymbol {x} \geq \boldsymbol {b} \\ & \boldsymbol {x} \geq 0 \end{array}\tag{25.7}
$$

25.6 (Jain and Vazirani [141]) Consider the $l _ { 2 } ^ { 2 }$ clustering problem. Given a set of n points $S = \{ v _ { 1 } , \ldots , v _ { n } \}$ in $\mathbf { R } ^ { d }$ and a positive integer k, the problem is to find a minimum cost k-clustering, i.e., to find k points, called centers, $f _ { 1 } , \ldots , f _ { k } \in \mathbf { R } ^ { d }$ , so as to minimize the sum of squares of distances from each point $v _ { i }$ to its closest center. This naturally defines a partitioning of the n points into k clusters. Give a constant factor approximation algorithm for this problem.

Hint: First show that restricting the centers to be a subset S increases the cost of the optimal solution by a factor of at most 2. Apply the solution of Exercise 24.6 to this modified problem.

25.7 (Korupolu, Plaxton, and Rajaraman [176] and Arya et al. [15]) For a set S of k facilities, define cost(S) to be the total cost of connecting each city to its closest facility in S. Define a swap to be the process of replacing one facility in S by a facility from S. A natural algorithm for metric k-median, based on local search, is: Start with an arbitrary set S of k facilities. In each iteration, check if there is a swap that leads to a lower cost solution. If so, execute any such swap and go to the next iteration. If not, halt. The terminating solution is said to be locally optimal.

Let $G = \{ o _ { 1 } , \ldots , o _ { k } \}$ be an optimal solution and $L = \{ s _ { 1 } , \ldots , s _ { k } \}$ be a locally optimal solution. This exercise develops a proof showing cost $( L ) \leq$ $5 \cdot \operatorname { c o s t } ( G )$ , as well as a constant factor approximation algorithm.

1. For $o \in G ,$ , let $N _ { G } ( o )$ denote the set of cities connected to facility o in the optimal solution. Similarly, for $s \in L ,$ , let $N _ { L } ( s )$ denote the set of cities connected to facility s in the locally optimal solution. Say that $s \in L$ captures $o \in G { \mathrm { ~ i f ~ } } | N _ { G } ( o ) \cap N _ { L } ( s ) | > | N _ { G } ( o ) | / 2$ . Clearly, each $o \in G$ is captured by at most one facility in L. In this part let us make the simplifying assumption that each facility $s \in L$ captures a unique facility in $G .$ . Assume that the facilities are numbered so that $s _ { i }$ captures $o _ { i } .$ , for $1 \leq i \leq k$ . Use the fact that for $1 \leq i \leq k ,$ cost $( L + o _ { i } - s _ { i } ) \ge \mathrm { c o s t } ( L )$ to show that cost $( L ) \leq 3 \cdot \cot ( G )$

Hint: cost $\left( L + o _ { i } - s _ { i } \right)$ is bounded by the cost of the following solution: The cities in $\overline { { N _ { L } ( s _ { i } ) \cup N _ { G } ( o _ { i } ) } }$ are connected as in the locally optimal solution. Those in $N _ { G } ( o _ { i } )$ are connected to facility $o _ { i }$ . Cities in ${ \cal N } _ { L } ( s _ { i } ) -$ $N _ { G } ( o _ { i } )$ are connected to facilities in $L - s _ { i }$ using “3 hops” in such a way that each connecting edge of G and each connecting edge of L is used at most once in the union of all these hops.

2. Show that without the simplifying assumption of the previous part, cos $; ( L ) \leq 5 \cdot \mathrm { c o s t } ( G )$

Hint: Consider k appropriately chosen swaps so that each facility $o \in G$ is swapped in exactly once and each facility $s \in L$ is swapped out at most twice.

3. Strengthen the condition for swapping so as to obtain, for any $\varepsilon > 0$ a factor $5 + \varepsilon$ algorithm running in time polynomial in $1 / \varepsilon$ and the size of the instance.

## 25.6 Notes

The first approximation algorithm, achieving a factor of O(log n log log n), was given by Bartal [21]. The first constant factor approximation algorithm for the k-median problem, achieving a guarantee of $6 { \frac { 2 } { 3 } }$ , was given by Charikar, Guha, Tardos, and Shmoys [39], using ideas from Lin and Vitter [189]. This algorithm used LP-rounding. The results of this chapter are due to Jain and Vazirani [141]. The current best factor is $3 + 2 / p$ , with a running time of $O ( n ^ { p } )$ , due to Arya et al. [15]. This is a local search algorithm that swaps p facilities at a time (see Exercise 25.7 for the algorithm for $p = 1 )$ ).

The example of Section 25.3.4 is due to Jain, Mahdian, and Saberi [138]. The best upper bound on the integrality gap of relaxation (25.2) is 4, due to Charikar and Guha [38]. For a factor 2 approximation algorithm for the $l _ { 2 } ^ { 2 }$ clustering problem (Exercise 25.6), see Drineas, Kannan, Frieze, Vempala, and Vinay [62].