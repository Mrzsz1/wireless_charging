---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-24"
chapter_number: 24
chapter_title: "Facility Location"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 250
source_page_end: 260
printed_page_start: 232
printed_page_end: 242
part_ids: ["approximation-algorithms-ch-24-part-025"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Facility Location (MinerU semantic layer)

<!-- source-pages: 250-260; printed-pages: 232-242; mineru-part: approximation-algorithms-ch-24-part-025 -->

## 24 Facility Location

The facility location problem has occupied a central place in operations research since the early 1960’s. It models design situations such as deciding placements of factories, warehouses, schools, and hospitals. Modern day applications include placement of proxy servers on the web.

In this chapter, we will present a primal–dual schema based factor 3 approximation algorithm for the special case when connection costs satisfy the triangle inequality. The algorithm difers in two respects from previous primal–dual algorithms. First, the primal and dual pair of LPs have negative coeficients and do not form a covering-packing pair. Second, we will relax primal complementary slackness conditions rather than the dual ones. Also, the idea of synchronization, introduced in the primal–dual schema in Chapter 22, is developed further, with an explicit timing of events playing a role.

Problem 24.1 (Metric uncapacitated facility location) Let G be a bipartite graph with bipartition $( F , C )$ , where $F$ is the set of facilities and $C$ is the set of cities. Let $f _ { i }$ be the cost of opening facility $i ,$ , and $c _ { i j }$ be the cost of connecting city j to (opened) facility i. The connection costs satisfy the triangle inequality. The problem is to find a subset $I \subseteq F$ of facilities that should be opened, and a function $\phi : { \cal C }  I$ assigning cities to open facilities in such a way that the total cost of opening facilities and connecting cities to open facilities is minimized.

Consider the following integer program for this problem. In this program, $y _ { i }$ is an indicator variable denoting whether facility i is open, and $x _ { i j }$ is an indicator variable denoting whether city $j$ is connected to the facility i. The first set of constraints ensures that each city is connected to at least one facility, and the second ensures that this facility must be open.

$$
\begin{array}{l l l} \text {minimize} & \sum_ {i \in F, j \in C} c _ {i j} x _ {i j} + \sum_ {i \in F} f _ {i} y _ {i} \\ \text {subject to} & \sum_ {i \in F} x _ {i j} \geq 1, & j \in C \\ & y _ {i} - x _ {i j} \geq 0, & i \in F, j \in C \\ & x _ {i j} \in \{0, 1 \}, & i \in F, j \in C \\ & y _ {i} \in \{0, 1 \}, & i \in F \end{array}\tag{24.1}
$$

The LP-relaxation of this program is:

$$
\begin{array}{l l l} \text {minimize} & \sum_ {i \in F, j \in C} c _ {i j} x _ {i j} + \sum_ {i \in F} f _ {i} y _ {i} \\ \text {subject to} & \sum_ {i \in F} x _ {i j} \geq 1, & j \in C \\ & y _ {i} - x _ {i j} \geq 0, & i \in F, j \in C \\ & x _ {i j} \geq 0, & i \in F, j \in C \\ & y _ {i} \geq 0, & i \in F \end{array}\tag{24.2}
$$

The dual program is:

maximize

$$
\sum_ {j \in C} \alpha_ {j}
$$

subject to

$$
\begin{array}{l l} \alpha_ {j} - \beta_ {i j} \leq c _ {i j}, & i \in F, j \in C \\ \sum_ {j \in C} \beta_ {i j} \leq f _ {i}, & i \in F \\ \alpha_ {j} \geq 0, & j \in C \\ \beta_ {i j} \geq 0, & i \in F, j \in C \end{array}\tag{24.3}
$$

## 24.1 An intuitive understanding of the dual

Let us first give the reader some feel for how the dual variables $\mathrm { \Delta ^ { 6 6 } p a y ^ { 5 } }$ for a primal solution by considering the following simple setting. Suppose LP (24.2) has an optimal solution that is integral, say $I \subseteq F$ and $\phi : C \to I .$ Thus, under this solution, $y _ { i } = 1$ if $i \in I ,$ , and $x _ { i j } = 1$ if $i = \phi ( j )$ . Let $( \alpha , \beta )$ denote an optimal dual solution.

The primal and dual complementary slackness conditions are:

(i)

$$
\forall i \in F, j \in C: x _ {i j} > 0 \Rightarrow \alpha_ {j} - \beta_ {i j} = c _ {i j}\tag{ii}
$$

$$
\forall i \in F: y _ {i} > 0 \Rightarrow \sum_ {j \in C} \beta_ {i j} = f _ {i}\tag{iii}
$$

$$
\forall j \in C: \alpha_ {j} > 0 \Rightarrow \sum_ {i \in F} x _ {i j} = 1\tag{iv}
$$

$$
\forall i \in F, j \in C: \beta_ {i j} > 0 \Rightarrow y _ {i} = x _ {i j}
$$

By condition (ii), each open facility must be fully paid for, i.e., ${ \mathrm { i f ~ } } i \in I .$ then

$$
\sum_ {j \colon \phi (j) = i} \beta_ {i j} = f _ {i}.
$$

Consider condition (iv). Now, if facility i is open, but $\phi ( j ) \neq i ,$ then $y _ { i } \neq x _ { i j }$ , and so $\beta _ { i j } = 0$ , i.e., city $j$ does not contribute to opening any facility besides the one it is connected to.

By condition (i), if $\phi ( j ) = i$ , then $\alpha _ { j } - \beta _ { i j } = c _ { i j }$ . Thus, we can think of $\alpha _ { j }$ as the total price paid by city $j ;$ of this, $c _ { i j }$ goes towards the use of edge $( i , j )$ , and $\beta _ { i j }$ is the contribution of $j$ towards opening facility i.

## 24.2 Relaxing primal complementary slackness conditions

Suppose the primal complementary slackness conditions were relaxed as follows, while maintaining the dual conditions:

$$
\forall j \in C: (1 / 3) c _ {\phi (j) j} \leq \alpha_ {j} - \beta_ {\phi (j) j} \leq c _ {\phi (j) j},
$$

and

$$
\forall i \in I: (1 / 3) f _ {i} \leq \sum_ {j: \phi (j) = i} \beta_ {i j} \leq f _ {i}.
$$

Then, the cost of the (integral) solution found would be within thrice the dual found, thus leading to a factor 3 approximation algorithm. However, we would like to obtain the stronger inequality stated in Theorem 24.7. Now, the dual pays at least one-third the connection cost, but must pay completely for opening facilities. This stronger inequality will be needed in order to use this algorithm to solve the k-median problem in Chapter 25.

For this reason, we will relax the primal conditions as follows. The cities are partitioned into two sets, directly connected and indirectly connected. Only directly connected cities will pay for opening facilities, i.e., $\beta _ { i j }$ can be nonzero only $\mathrm { i f } \ j$ is a directly connected city and $i = \phi ( j )$ . For an indirectly connected city j, the primal condition is relaxed as follows:

$$
(1 / 3) c _ {\phi (j) j} \leq \alpha_ {j} \leq c _ {\phi (j) j}.
$$

All other primal conditions are maintained, i.e., for a directly connected city $j ,$ 9

$$
\alpha_ {j} - \beta_ {\phi (j) j} = c _ {\phi (j) j},
$$

and for each open facility $i ,$

$$
\sum_ {j: \phi (j) = i} \beta_ {i j} = f _ {i}.
$$

## 24.3 Primal–dual schema based algorithm

The algorithm consists of two phases. In Phase 1, the algorithm operates in a primal–dual fashion. It finds a dual feasible solution and also determines a set of tight edges and temporarily open facilities, $F _ { t }$ . Phase 2 consists of choosing a subset I of $F _ { t }$ to open, and finding a mapping, $\phi ,$ from cities to $I .$

## Algorithm 24.2

## Phase 1

We would like to find as large a dual solution as possible. This motivates the following underlying process for dealing with the non-covering-packing pair of LPs. Each city j raises its dual variable, $\alpha _ { j }$ , until it gets connected to an open facility. All other primal and dual variables simply respond to this change, trying to maintain feasibility or satisfying complementary slackness conditions.

A notion of time is defined in this phase, so that each event can be associated with the time at which it happened; the phase starts at time 0. Initially, each city is defined to be unconnected. Throughout this phase, the algorithm raises the dual variable $\alpha _ { j }$ for each unconnected city $j$ uniformly at unit rate, $\mathrm { i . e . , } \alpha _ { j }$ will grow by 1 in unit time. When $\alpha _ { j } = c _ { i j }$ for some edge $( i , j )$ , the algorithm will declare this edge to be tight. Henceforth, dual variable $\beta _ { i j }$ will be raised uniformly, thus ensuring that the first constraint in LP (24.3) is not violated. $\beta _ { i j }$ goes towards paying for facility i. Each edge (i, j) such that $\beta _ { i j } > 0$ is declared special.

Facility i is said to be paid for if $\textstyle \sum _ { i } \beta _ { i j } = f _ { i }$ . If so, the algorithm de-clares this facility temporarily open. Furthermore, all unconnected cities having tight edges to this facility are declared connected and facility i is declared the connecting witness for each of these cities. (Notice that the dual variables $\alpha _ { j }$ of these cities are not raised anymore.) In the future, as soon as an unconnected city $j$ gets a tight edge to i, j will also be declared connected and i will be declared the connecting witness for $j$ (notice that $\beta _ { i j } = 0$ and thus edge $( i , j )$ is not special). When all cities are connected, the first phase terminates. If several events happen simultaneously, the algorithm executes them in arbitrary order.

Remark 24.3 At the end of Phase 1, a city may have paid towards temporarily opening several facilities. However, we want to ensure that a city pays only for the facility that it is eventually connected to. This is ensured in Phase 2, which chooses a subset of temporarily open facilities for opening permanently.

## Phase 2

Let $F _ { t }$ denote the set of temporarily open facilities and $T$ denote the subgraph of $G$ consisting of all special edges. Let $T ^ { 2 }$ denote the graph that has edge $( u , v )$ if there is a path of length at most 2 between u and v in $T ,$ and let H be the subgraph of $T ^ { 2 }$ induced on $F _ { t }$ . Find any maximal independent set in $H ,$ , say I. All facilities in the set I are declared open.

For city $j ,$ , define $\mathcal { F } _ { j } = \{ i \in F _ { t } ~ | ~ ( i , j )$ is special}. Since I is an independent set, at most one of the facilities in ${ \mathcal { F } } _ { j }$ is opened. If there is a facility $i \in$ ${ \mathcal { F } } _ { j }$ that is opened, then set $\phi ( j ) = i$ and declare city $j$ directly connected. Otherwise, consider tight edge $( i ^ { \prime } , j )$ such that $i ^ { \prime }$ was the connecting witness for $j , \mathrm { H } i ^ { \prime } \in I ,$ again set $\phi ( j ) = i \prime$ and declare city j directly connected (notice that in this case $\beta _ { i ^ { \prime } j } = 0 )$ . In the remaining case that $i ^ { \prime } \notin I ,$ let i be any neighbor of $i ^ { \prime }$ in graph H such that $i \in I .$ Set $\phi ( j ) = i$ and declare city j indirectly connected.

I and φ define a primal integral solution: $x _ { i j } = 1$ if $\phi ( j ) = i$ and $y _ { i } = 1$ $\mathrm { i f } \ i \in I$ . The values of $\alpha _ { j }$ and $\beta _ { i j }$ obtained at the end of Phase 1 form a dua feasible solution.

## 24.4 Analysis

We will show how the dual variables $\alpha _ { j } \mathrm { ^ { * } s }$ pay for the primal costs of opening facilities and connecting cities to facilities. Denote by $\alpha _ { j } ^ { f }$ and $\alpha _ { j } ^ { e }$ the contributions of city $j$ to these two costs respectively; $\alpha _ { j } = \alpha _ { j } ^ { f } + \alpha _ { j } ^ { e } . \mathrm { ~ I f ~ } j$ is indirectly connected, then $\alpha _ { j } ^ { f } = 0$ and $\alpha _ { j } ^ { e } = \alpha _ { j }$ . If j is directly connected, then the following must hold:

$$
\alpha_ {j} = c _ {i j} + \beta_ {i j},
$$

where $i = \phi ( j )$ . Now, let $\alpha _ { j } ^ { f } = \beta _ { i j }$ and $\alpha _ { j } ^ { e } = c _ { i j }$

Lemma 24.4 Let $i \in I$ . Then,

$$
\sum_ {j: \phi (j) = i} \alpha_ {j} ^ {f} = f _ {i}.
$$

Proof: Since i is temporarily open at the end of Phase 1, it is completely paid for, i.e.,

$$
\sum_ {j: (i, j) \text { is   special }} \beta_ {i j} = f _ {i}.
$$

The critical observation is that each city j that has contributed to $f _ { i }$ must be directly connected to i. For each such city, $\alpha _ { j } ^ { f } = \beta _ { i j }$ . Any other city $j ^ { \prime }$ that is connected to facility i must satisfy $\alpha _ { j ^ { \prime } } ^ { f } = \dot { 0 }$ . The lemma follows. ✷

Corollary 24.5 $\begin{array} { r } { \sum _ { i \in I } f _ { i } = \sum _ { j \in C } \alpha _ { j } ^ { f } } \end{array}$

Recall that $\alpha _ { j } ^ { f }$ was defined to be 0 for indirectly connected cities. Thus, only the directly connected cities pay for the cost of opening facilities.

Lemma 24.6 For an indirectly connected city j, $c _ { i j } \leq 3 \alpha _ { j } ^ { e }$ , where $i = \phi ( j )$

Proof: Let $i ^ { \prime }$ be the connecting witness for city $j .$ . Since $j$ is indirectly connected to $i , \ ( i , i ^ { \prime } )$ must be an edge in H. In turn, there must be a city, say $j ^ { \prime }$ , such that $( i , j ^ { \prime } )$ and $( i ^ { \prime } , j ^ { \prime } )$ are both special edges. Let $t _ { 1 }$ and $t _ { 2 }$ be the times at which i and $i ^ { \prime }$ were declared temporarily open during Phase 1.

![](images/67ed21fbc6255c89201925b740ac1c98eb0d00140edd14d1f806ebcd441ce113.jpg)

Since edge $( i ^ { \prime } , j )$ is tight, $\alpha _ { j } \ \geq \ c _ { i ^ { \prime } j }$ . We will show that $\alpha _ { j } \ \geq \ c _ { i j ^ { \prime } }$ and $\alpha _ { j } \geq c _ { i ^ { \prime } j ^ { \prime } }$ . Then, the lemma will follow by using the triangle inequality.

Since edges $( i ^ { \prime } , j ^ { \prime } )$ and $( i , j ^ { \prime } )$ are tight, $\alpha _ { j ^ { \prime } } \geq c _ { i j ^ { \prime } }$ and $\alpha _ { j ^ { \prime } } \geq c _ { i ^ { \prime } j ^ { \prime } }$ . Since both these edges are special, they must both have gone tight before either i or $i ^ { \prime }$ is declared temporarily open. Consider the time min $( t _ { 1 } , t _ { 2 } )$ . Clearly, $\alpha _ { j ^ { \prime } }$ cannot be growing beyond this time. Therefore, $\alpha _ { j ^ { \prime } } \leq$ min $( t _ { 1 } , t _ { 2 } )$ . Fina $\mathrm { l y , }$ since $i ^ { \prime }$ is the connecting witness for $j , \alpha _ { j } \geq t _ { 2 }$ . Therefore, $\alpha _ { j } \geq \alpha _ { j ^ { \prime } }$ , and the required inequalities follow. ✷

Theorem 24.7 The primal and dual solutions constructed by the algorithm satisfy:

$$
\sum_ {i \in F, j \in C} c _ {i j} x _ {i j} + 3 \sum_ {i \in F} f _ {i} y _ {i} \leq 3 \sum_ {j \in C} \alpha_ {j}.
$$

Proof: For a directly connected city $j , c _ { i j } = \alpha _ { j } ^ { e } \leq 3 \alpha _ { j } ^ { e }$ , where $\phi ( j ) = i$ Combining with Lemma 24.6 we get

$$
\sum_ {i \in F, j \in C} c _ {i j} x _ {i j} \leq 3 \sum_ {j \in C} \alpha_ {j} ^ {e}.
$$

Adding to this the equality stated in Corollary 24.5 multiplied by 3 gives the theorem. ✷

## 24.4.1 Running time

A special feature of the primal–dual schema is that it yields algorithms with good running times. Since this is especially so for the current algorithm, we will provide some implementation details. We will adopt the following notation: $n _ { c } = | C |$ and $n _ { f } = | \boldsymbol { F } |$ . The total number of vertices $n _ { c } + n _ { f } = n$ and the total number of edges $n _ { c } \times n _ { f } = m$

Sort all the edges by increasing cost – this gives the order and the times at which edges go tight. For each facility, i, we maintain the number of cities that are currently contributing towards it, and the anticipated time, $t _ { i } .$ at which it would be completely paid for if no other event happens on the way. Initially all $t _ { i } \mathrm { \ ' } \mathrm { s }$ are infinite, and each facility has 0 cities contributing to it. The $t _ { i } \mathrm { \ ' } \mathrm { s }$ are maintained in a binary heap so we can update each one and find the current minimum in $O ( \log n _ { f } )$ time. Two types of events happen, and they lead to the following updates.

• An edge (i, j) goes tight.

– If facility i is not temporarily open, then it gets one more city contributing towards its cost. The amount contributed towards its cost at the current time can be easily computed. Therefore, the anticipated time for facility i to be paid for can be recomputed in constant time. The heap can be updated in $O ( \log n _ { f } )$ time.

– If facility i is already temporarily open, city $j$ is declared connected, and $\alpha _ { j }$ is not raised anymore. For each facility $i ^ { \prime }$ that was counting j as a contributor, we need to decrease the number of contributors by 1 and recompute the anticipated time at which it gets paid for.

• Facility i is completely paid for. In this event, i will be declared temporarily open, and all cities contributing to i will be declared connected. For each of these cities, we will execute the second case of the previous event, i.e., update facilities that they were contributing towards.

The next theorem follows by observing that each edge $( i , j )$ will be considered at most twice. First, when it goes tight. Second, when city j is declared connected. For each consideration of this edge, we will do $O ( \log n _ { f } )$ work.

Theorem 24.8 Algorithm 24.2 achieves an approximation factor of 3 for the facility location problem and has a running time of O(m log m).

## 24.4.2 Tight example

The following infinite family of examples shows that the analysis of our algorithm is tight: The graph has n cities, $c _ { 1 } , c _ { 2 } , \ldots , c _ { n }$ and two facilities $f _ { 1 }$ and $f _ { 2 }$ . Each city is at a distance of 1 from $f _ { 2 }$ . City c<sub>1</sub> is at a distance of 1 from $f _ { 1 }$ , and $c _ { 2 } , \ldots , c _ { n }$ are at a distance of 3 from $f _ { 1 }$ . The opening cost of $f _ { 1 }$ and $f _ { 2 }$ are ε and $( n + 1 ) \varepsilon$ , respectively, for a small number ε.

![](images/ec172788b1a2adae834444062317ee16f44ab7f1f93bf42a12c37d609f9e79d5.jpg)  
The optimal solution is to open $f _ { 2 }$ and connect all cities to it, at a total cost of $( n + 1 ) \varepsilon + n$ . Algorithm 24.2 will however open facility $f _ { 1 }$ and connect all cities to it, at a total cost of $\varepsilon + 1 + 3 ( n - 1 )$ ).

## 24.5 Exercises

24.1 Consider the general uncapacitated facility location problem in which the connection costs are not required to satisfy the triangle inequality. Give a reduction from the set cover problem to show that approximating this problem is as hard as approximating set cover and therefore cannot be done better than O(log n) factor unless $\mathbf { \tilde { N P } } \subseteq \tilde { \mathbf { P } }$ . Also, give an O(log n) factor algorithm for this problem.

24.2 In Phase 2, instead of picking all special edges in $T _ { : }$ , pick all tight edges. Show that now Lemma 24.6 does not hold. Give a suitable modification to the algorithm that restores Lemma 24.6.

Hint: Order facilities in H in the order in which they were temporarily opened, and pick I to be the lexicographically first maximal independent set.

24.3 Give a factor 3 tight example for Algorithm 24.2 in which the set of cities and facilities is the same, i.e., $C = F$

24.4 Consider the proof of Lemma 24.6. Give an example in which $\alpha _ { j } > t _ { 2 }$

24.5 The vector α found by Algorithm 24.2 is maximal in the sense that if we increase any $\alpha _ { j }$ in this vector, then there is no way of setting the $\beta _ { i j } \mathrm { ^ { * } s }$ to get a feasible dual solution. Is every maximal solution α within 3 times the optimal solution to dual program for facility location?

Hint: It is easy to construct a maximal solution that is $2 / n$ times the optimal. Consider n facilities with an opening cost of 1 each and n cities connected to distinct facilities by edges of cost ε each. In addition, there is another city that is connected to each facility with an edge of cost 1.

24.6 Consider the following modification to the metric uncapacitated facility location problem. Define the cost of connecting city $j$ to facility i to be $c _ { i j } ^ { 2 } .$ The $c _ { i j } \mathrm { ^ { \circ } s }$ still satisfy the triangle inequality (but the new connection costs, of $c _ { i j } ^ { 2 }$ , do not). Show that Algorithm 24.2 achieves an approximation guarantee of factor 9 for this case.

24.7 Consider the following generalization to arbitrary demands. For each city $j ,$ a nonnegative demand $d _ { j }$ is specified, and any open facility can serve this demand. The cost of serving this demand via facility i is $c _ { i j } d _ { j }$ . Give an IP and LP-relaxation for this problem, and extend Algorithm 24.2 to get a factor 3 algorithm.

Hint: Raise $\alpha _ { j }$ at rate $d _ { j }$

24.8 In the capacitated facility location problem, we are given a number $u _ { i }$ for each facility $i ,$ and facility i can serve at most $u _ { i }$ cities. Show that the modification of LP (24.2) to this problem has an unbounded integrality gap.

24.9 Consider the variant of the capacitated metric facility location problem in which each facility can be opened an unbounded number of times. If facility i is opened $y _ { i }$ times, it can serve at most $u _ { i } y _ { i }$ cities. Give an IP and LPrelaxation for this problem, and extend Algorithm 24.2 to obtain a constant factor algorithm.

24.10 (Charikar, Khuller, Mount, and Narshimhan [40]) Consider the prizecollecting variant of the facility location problem, in which there is a specified penalty for not connecting a city to an open facility. The objective is to minimize the sum of the connection costs, facility opening costs, and penalties. Give a factor 3 approximation algorithm for this problem.

24.11 (Jain and Vazirani [140]) Consider the fault tolerant variant of the facility location problem, in which the additional input is a connection requirement $r _ { j }$ for each city $j .$ In the solution, city j needs to be connected to $r _ { j }$ distinct open facilities. The objective, as before, is to minimize the sum of the connection costs and the facility opening costs.

Decompose the problem into k phases, numbered k down to 1, as in Exercise 23.7. In phase $p ,$ , all cities having a residual requirement of $p$ are provided one more connection to an open facility. In phase $p ,$ the facility location algorithm of this chapter is run on the following modified graph, $G _ { p }$ The cost of each facility that is opened in an earlier phase is set to 0. If city $j$ is connected to facility i in an earlier phase, then $c _ { i j }$ is set to $\infty$

1. Show that even though $G _ { p }$ violates the triangle inequality at some places, the algorithm gives a solution within factor 3 of the optimal solution for this graph.

Hint: Every time short-cutting is needed; the triangle inequality holds.

2. Show that the solution found in phase $p$ is of cost at most $3 \cdot \mathrm { O P T } / p .$ where OPT is the cost of the solution to the entire problem. Hint: Remove ∞ cost edges of $G _ { p }$ from the optimal solution and divide the rest by $p .$ Show that this is a feasible fractional solution for phase $p .$

3. Show that this algorithm achieves an approximation factor of $3 \cdot H _ { k }$ for the fault tolerant facility location problem.

24.12 (Mahdian, Markakis, Saberi, and Vazirani [201]) This exercise develops a factor 3 greedy algorithm for the metric uncapacitated facility location problem, together with an analysis using the method of dual fitting.

Consider the following modification to Algorithm 24.2. As before, dual variables, $\alpha _ { j } .$ , of all unconnected cities, $j ,$ , are raised uniformly. If edge $( i , j )$ is tight, $\beta _ { i j }$ is raised. As soon as a facility, say $i ,$ is paid for, it is declared open. Let S be the set of unconnected cities having tight edges to i. Each city $j \in S$ is declared connected and stops raising its $\alpha _ { j }$ . So far, the new algorithm is the same as Algorithm 24.2. The main diference appears at this stage: Each city $j \in S$ withdraws its contribution from other facilities, i.e., for each facility $i ^ { \prime } \neq i ,$ set $\beta _ { i ^ { \prime } j } = 0$ . When all cities have been declared connected, the algorithm terminates. Observe that each city contributes towards the opening cost of at most one facility – the facility it gets connected to.

1. This algorithm actually has a simpler description as a greedy algorithm. Provide this description.

Hint: Use the notion of cost–efectiveness defined for the greedy set cover algorithm.

2. The next 3 parts use the method of dual fitting to analyze this algorithm. First observe that the primal solution found is fully paid for by the dual computed.

3. Let i be an open facility and let $\{ 1 , \ldots , k \}$ be the set of cities that contributed to opening i at some point in the algorithm. Assume w.l.o.g. that $\alpha _ { 1 } \leq \alpha _ { j }$ for $j \le k$ . Show that for $j \le k , \alpha _ { j } - c _ { i j } \le 2 \alpha _ { 1 }$ . Also, show that

$$
\sum_ {j = 1} ^ {k} \alpha_ {j} \leq 3 \sum_ {j = 1} ^ {k} c _ {i j} + f _ {i}.
$$

Hint: Use the triangle inequality and the following inequality which is a consequence of the fact that at any point, the total amount contributed for opening facility i is at most $f _ { i }$ :

$$
\sum_ {j: c _ {i j} \leq \alpha_ {1}} \alpha_ {1} - c _ {i j} \leq f _ {i}.
$$

4. Hence show that $\alpha / 3$ is a dual feasible solution.

5. How can the analysis be improved – a factor 1.86 analysis is known for this algorithm.

6. Give a time eficient implementation of this algorithm, matching the running time of Algorithm 24.2

7. Do you see room for improving the algorithm?

Hint: Suppose city $j$ is connected to open facility i at some point in the algorithm. Later, facility $i ^ { \prime }$ is opened, and suppose that $c _ { i j } > c _ { i ^ { \prime } j }$ . Then, connecting j to $i ^ { \prime }$ will reduce the cost of the solution.

24.13 (Mahdian, Markakis, Saberi, and Vazirani [201]) Consider the following variant of the metric uncapacitated facility location problem. Instead of $f _ { i } ,$ the opening cost for each facility $i \in F$ , we are provided a startup cost $s _ { i }$ and an incremental cost $t _ { i } .$ Define the new opening cost for connecting $k > 0$ cities to facility i to be $s _ { i } + k t _ { i } .$ . Connection costs are specified by a metric, as before. The object again is to connect each city to an open facility so as to minimize the sum of connection costs and opening costs. Give an approximation factor preserving reduction from this problem to the metric uncapacitated facility location problem.

Hint: Modify the metric appropriately.

## 24.6 Notes

The first approximation algorithm for the metric uncapacitated facility location problem, due to Hochbaum [124], achieved an approximation guarantee of $O ( \log n )$ . The first constant factor approximation algorithm, achieving a guarantee of 3.16, was due to Shmoys, Tardos, and Aardal [239]. It was based on LP-rounding. The current best algorithm, achieving an approximation guarantee of 1.61, is due to Jain, Mahdian, and Saberi [138]. This algorithm, a small modification of the greedy algorithm presented in Exercise 24.12, is analyzed using the method of dual fitting. The primal–dual schema based Algorithm 24.2 is due to Jain and Vazirani [141].
