---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-22"
chapter_number: 22
chapter_title: "Steiner Forest"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 216
source_page_end: 230
printed_page_start: 198
printed_page_end: 212
part_ids: ["approximation-algorithms-ch-22-part-023"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Steiner Forest (MinerU semantic layer)

<!-- source-pages: 216-230; printed-pages: 198-212; mineru-part: approximation-algorithms-ch-22-part-023 -->

## 22 Steiner Forest

We will obtain a factor 2 approximation algorithm for the Steiner forest problem by enhancing the primal–dual schema with the idea of growing duals in a synchronized manner. The Steiner forest problem generalizes the metric Steiner tree problem, for which a factor 2 algorithm was presented in Chapter 3. Recall, however, that we had postponed giving the lower bounding method behind that algorithm; we will clarify this as well.

As in the Steiner tree problem (Theorem 3.2), the main case of the Steiner forest problem is also the metric case (see Exercise 22.2). However, the primal–dual algorithm remains the same for both cases, so we don’t impose this restriction.

Problem 22.1 (Steiner forest) Given an undirected graph $G = ( V , E )$ • a cost function on edges $c : E \to \mathbf { Q } ^ { + }$ , and a collection of disjoint subsets of $V , S _ { 1 } , \ldots S _ { k }$ , find a minimum cost subgraph in which each pair of vertices belonging to the same set $S _ { i }$ is connected.

Let us restate the problem; this will also help generalize it later. Define a connectivity requirement function r that maps unordered pairs of vertices to {0, 1} as follows:

$$
r (u, v) = \left\{ \begin{array}{l} 1 \text {   if   } u \text {   and   } v \text {   belong   to   the   same   set   } S _ {i} \\ 0 \text {   otherwise } \end{array} \right.
$$

Now, the problem is to find a minimum cost subgraph F that contains a u–v path for each pair $( u , v )$ with $r ( u , v ) = 1$ . In general, the solution will be a forest.

## 22.1 LP-relaxation and dual

In order to give an integer programming formulation for this problem, let us define a function on all cuts in $G , \ f : 2 ^ { V } \to \{ 0 , 1 \}$ , which specifies the minimum number of edges that must cross each cut in any feasible solution.

$$
f (S) = \left\{ \begin{array}{l} 1 \text {   if   } \exists u \in S \text {   and   } v \in \overline {{S}} \text {   such   that   } r (u, v) = 1 \\ 0 \text {   otherwise } \end{array} \right.
$$

Let us also introduce a $0 / 1$ variable $x _ { e }$ for each edge $e \in E ; x _ { e }$ will be set to 1 if e is picked in the subgraph. The integer program is:

$$
\begin{array}{l l} \text {minimize} & \sum_ {e \in E} c _ {e} x _ {e} \\ \text {subject to} & \sum_ {e: e \in \delta (S)} x _ {e} \geq f (S), \quad S \subseteq V \\ & x _ {e} \in \{0, 1 \}, \qquad e \in E \end{array}\tag{22.1}
$$

where $\delta ( S )$ denotes the set of edges crossing the cut $( S , { \overline { { S } } } )$

Following is the LP-relaxation of (22.1); once again, we have dropped the redundant conditions $x _ { e } \leq 1$

$$
\begin{array}{l l} \text { minimize } & \sum_ {e \in E} c _ {e} x _ {e} \\ \text { subject   to } & \sum_ {e: e \in \delta (S)} x _ {e} \geq f (S), \quad S \subseteq V \\ & x _ {e} \geq 0, \quad e \in E \end{array}\tag{22.2}
$$

The dual program is:

$$
\begin{array}{l l} \text {maximize} & \sum_ {S \subseteq V} f (S) \cdot y _ {S} \\ \text {subject to} & \sum_ {S: e \in \delta (S)} y _ {S} \leq c _ {e}, \quad e \in E \\ & y _ {S} \geq 0, \quad S \subseteq V \end{array}\tag{22.3}
$$

Notice that the primal and dual programs form a covering and packing pair of LPs (see Section 13.1 for definitions).

## 22.2 Primal–dual schema with synchronization

We will introduce a new idea in the primal–dual schema for approximation algorithms, setting it apart from the way this schema is used for designing exact algorithms. The later algorithms work on demand – in each iteration, we pick one unsatisfied complementary slackness condition, and satisfy it by modifying the primal and dual solutions suitably. The new idea is that of raising duals in a synchronized manner. The algorithm is not trying to rectify a specific condition. Instead, it tries many possibilities simultaneously, one of which leads to primal improvement.

Some figurative terminology will help describe the algorithm more easily. Let us say that edge e feels dual $y _ { S }$ if $y _ { S } > 0$ and $e \in \delta ( S )$ . Say that set S has been raised in a dual solution if $y _ { S } > 0$ . Clearly, raising S or S has the same efect. Sometimes we will also say that we have raised the cut (S, S). Further, there is no advantage in raising set S with $f ( S ) = 0 ,$ , since this does not contribute to the dual objective function. Thus, we may assume that such cuts are never raised. Say that edge e is tight if the total amount of dual it feels equals its cost. The dual program is trying to maximize the sum of the dual variables $y _ { S }$ subject to the condition that no edge feels more dual than its cost, $\mathrm { i . e . , }$ no edge is overtight.

Next, let us state the primal and relaxed dual complementary slackness conditions. The algorithm will pick edges integrally only. Define the degree $o f$ set S to be the number of picked edges crossing the cut (S, S).

Primal conditions: For each $\begin{array} { r } { e \in E , x _ { e } \neq 0 \Rightarrow \sum _ { i : e \in \delta ( S ) } y _ { S } = c _ { e } } \end{array}$

Equivalently, every picked edge must be tight.

Relaxed dual conditions: The following relaxation of the dual conditions would have led to a factor 2 algorithm: for each $S \subseteq V , y _ { S } \neq 0 \Rightarrow$ $\begin{array} { r } { \sum _ { e \colon \ e \in \delta ( S ) } x _ { e } \leq 2 \cdot f ( S ) } \end{array}$ , i.e., every raised cut has degree at most 2. However, we do not know how to ensure this condition. Interestingly enough, we can still obtain a factor 2 algorithm – by relaxing this condition further! Raised sets will be allowed to have high degree; however, we will ensure that on average, raised duals have degree at most 2. The exact definition of “on average” will be given later.

The algorithm starts with null primal and dual solutions. In the spirit of the primal–dual schema, the current primal solution indicates which cuts need to be raised, and in turn, the current dual solution indicates which edge needs to be picked. Thus, the algorithm iteratively improves the feasibility of the primal, and the optimality of the dual, until a feasible primal is obtained.

Let us describe what happens in an iteration. At any point, the picked edges form a forest. Say that set S is unsatisfied if $f ( S ) = 1$ , but there is no picked edge crossing the cut $( S , { \overline { { S } } } )$ . Set $S$ is said to be active if it is a minimal (w.r.t. inclusion) unsatisfied set in the current iteration. Clearly, if the currently picked primal solution is infeasible, there must an unsatisfied set and therefore an active set w.r.t. it.

Lemma 22.2 Set S is active $i f f$ it is a connected component in the currently picked forest and $f ( S ) = 1$

Proof: Let $S$ be an active set. Now, S cannot contain part of a connected component because otherwise there will already be a picked edge in the cut $( S , { \bar { S } } )$ . Thus, $S$ is a union of connected components. Since $f ( S ) = 1$ , there is a vertex $u \in S$ and $v \in { \overline { { S } } }$ such that $r ( u , v ) = 1 . \operatorname { L e t } S ^ { \prime }$ be the connected component containing u. Clearly, $S ^ { \prime }$ is also unsatisfied, and by the minimality of $S , S = S ^ { \prime }$ ✷

By the characterization of active sets given in Lemma 22.2, it is easy to find all active sets in the current iteration. The dual variables of these sets are raised in a synchronized manner, until some edge goes tight. Any one of the newly tight edges is picked, and the current iteration terminates.

When a primal feasible solution is found, say $F ,$ the edge augmentation step terminates. However, $F$ may contain redundant edges, which need to be pruned for achieving the desired approximation factor; this is illustrated in Example 22.4. Formally, edge $e \in F$ is said to be redundant if $\boldsymbol { F } - \{ \boldsymbol { e } \}$ is also a feasible solution. All redundant edges can be dropped simultaneously from $F .$ Equivalently, only nonredundant edges are retained.

This algorithm is presented below. We leave its eficient implementation as an exercise.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 22.3 (Steiner forest)
1. (Initialization) $F \leftarrow \emptyset$; for each $S \subseteq V$, $y_S \leftarrow 0$.
2. (Edge augmentation) while there exists an unsatisfied set do: simultaneously raise $y_S$ for each active set $S$, until some edge $e$ goes tight;
$F \leftarrow F \cup \{e\}$.
3. (Pruning) return $F' = \{e \in F | F - \{e\} \text{ is primal infeasible}\}$
</div>

Example 22.4 Consider a star in which all edges have cost 1, except one edge whose cost is 3.

![](images/8c9b48e10894abc10222795b194b786d32640ca7aee2929e58fb0de3f65e4392.jpg)

The only requirement is to connect the end vertices of the edge of cost 3. The algorithm will add to F all edges of cost 1 before adding the edge of cost 3. Clearly, at this point, F is not within twice the optimal. However, this wil be corrected in the pruning step when all edges of cost 1 will be removed. ✷

Let us run the algorithm on a nontrivial example to illustrate its finer points.

Example 22.5 Consider the following graph. Costs of edges are marked, and the only nonzero connectivity requirements are $r ( u , v ) = 1$ and $r ( s , t ) = 1$ The thick edges indicate an optimal solution of cost 45.

![](images/1e4114c59b2c5bdd581b352b9748988850d1eb15d655ac2125dec28355726977.jpg)

In the first iteration, the following four singleton sets are active: $\{ s \} , \{ t \}$ • $\{ u \}$ , and $\{ v \}$ . When their dual variables are raised to $6$ each, edges $( u , a )$ and $( v , b )$ go tight. One of them, say $( u , a )$ is picked, and the iteration ends. In the second iteration, $\{ u , a \}$ replaces $\{ u \}$ as an active set. However, in this iteration there is no need to raise duals, since there is already a tight edge, $( v , b )$ . This edge is picked, and the iteration terminates. The primal and dual solutions at this point are shown below, with picked edges marked thick:

![](images/570af3f94a27c0d59774a35e07db3f8553eba3845a70397c0cc891f439e350e0.jpg)

In the third iteration, $\{ v , b \}$ replaces $\{ v \}$ as an active set. When the active sets are raised by 2 each, edge $( u , s )$ goes tight and is picked. In the fourth iteration, the active sets are $\{ u , s , a \} , \{ v \}$ and $\{ t \}$ . When they are raised by 1 each, edge $( b , t )$ goes tight and is picked. The situation now is:

![](images/9effafa690d9bfce4eeb002adca84c5072239aa0ac74d3b2e66865c37b245342.jpg)  
In the fifth iteration, the active sets are $\{ a , s , u \}$ and $\{ b , v , t \}$ . When they are raised by 1 each, $( u , v )$ goes tight, and we now have a primal feasible solution:

![](images/9f497c26089dc636d63b6668d9207f66ad1059bf1adce4fec1db64bae2caa324.jpg)  
In the pruning step, edge $( u , a )$ is deleted, and we obtain the following solution of cost 54:

![](images/3826e565d401f9f2bdcdfde9993aeaeab1ed09eec4d08e30c715059971aecff3.jpg)

## 22.3 Analysis

In Lemma 22.6 we will show that simultaneously deleting all redundant edges still leaves us with a primal feasible solution, $\mathrm { i . e . } .$ , it is never the case that two edges $e$ and $f$ are both redundant individually, but on deletion of $e , f$ becomes nonredundant.

Lemma 22.6 At the end of the algorithm, $F ^ { \prime }$ and y are primal and dual feasible solutions, respectively.

Proof: At the end of Step 2, F satisfies all connectivity requirements. In each iteration, dual variables of connected components only are raised. Therefore, no edge running within the same component can go tight, and so $F$ is acyclic, i.e., it is a forest. Therefore, if $r ( u , v ) = 1$ , there is a unique u–v path in $F .$ Thus, each edge on this path in nonredundant and is not deleted in Step 3. Hence, $F ^ { \prime }$ is primal feasible.

When an edge goes tight, the current iteration ends and active sets are redefined. Therefore, no edge is overtightened. Hence, y is dual feasible. ✷

Let $\deg _ { F ^ { \prime } } ( S )$ denote the number of edges of $F ^ { \prime }$ crossing the cut $( S , { \overline { { S } } } )$ The characterization of degrees of satisfied components established in the next lemma will be crucial in proving the approximation guarantee of the algorithm.

Lemma 22.7 Consider any iteration of the $a l g o r i t h m$ , and let C be a component w.r.t. the currently picked edges. If $f ( C ) = 0$ then $\deg _ { F ^ { \prime } } ( C ) \neq 1$

Proof: Suppose $\deg _ { F ^ { \prime } } ( C ) = 1$ , and let $e$ be the unique edge of $F ^ { \prime }$ crossing the cut $( C , { \overline { { C } } } )$ . Since e is nonredundant (every edge in $F ^ { \prime }$ is nonredundant), there is a pair of vertices, say $u , v ,$ such that $r ( u , v ) = 1$ and e lies on the unique $u - v$ path in $F ^ { \prime }$ . Since this path crosses the cut $( C , { \overline { { C } } } )$ exactly once, one of these vertices must lie in $C$ and the other in $\overline { { C } } .$ . Now, since $r ( u , v ) = 1$ 2 we get that $f ( C ) = 1$ , thus leading to a contradiction. ✷

Lemma 22.8

$$
\sum_ {e \in F ^ {\prime}} c _ {e} \leq 2 \sum_ {S \subseteq V} y _ {S}
$$

Proof: Since every picked edge is tight,

$$
\sum_ {e \in F ^ {\prime}} c _ {e} = \sum_ {e \in F ^ {\prime}} \left(\sum_ {S: e \in \delta (S)} y _ {S}\right).
$$

Changing the order of summation we get:

$$
\sum_ {e \in F ^ {\prime}} c _ {e} = \sum_ {S \subseteq V} \left(\sum_ {e \in \delta (S) \cap F ^ {\prime}} y _ {S}\right) = \sum_ {S \subseteq V} \deg_ {F ^ {\prime}} (S) \cdot y _ {S}.
$$

Thus, we need to show that

$$
\sum_ {S \subseteq V} \deg_ {F ^ {\prime}} (S) \cdot y _ {S} \leq 2 \sum_ {S \subseteq V} y _ {S}.\tag{22.4}
$$

We will prove the following stronger claim. In each iteration, the increase in the left-hand side of inequality (22.4) is bounded by the increase in the right-hand side. Consider an iteration, and let $\varDelta$ be the extent to which active sets were raised in this iteration. Then, we need to show:

$$
\Delta \times \left(\sum_ {S \text {active}} \deg_ {F ^ {\prime}} (S)\right) \leq 2 \Delta \times (\# \text {of active sets}).
$$

Notice that the degree w.r.t. $F ^ { \prime }$ of any active set $S$ is due to edges that will be picked during or after the current iteration. Let us rewrite this inequality as follows:

$$
\frac {\sum_ {S \text {active}} \deg_ {F ^ {\prime}} (S)}{\# \text {of active sets}} \leq 2.\tag{22.5}
$$

Thus, we need to show that in this iteration, the average degree of active sets w.r.t. $F ^ { \prime }$ is at most 2. The mechanics of the argument lies in the fact that in a tree, or in general in a forest, the average degree of vertices is at most 2.

Let H be a graph on vertex set $V$ and edge set $F ^ { \prime }$ . Consider the set of connected components w.r.t. $F$ at the beginning of the current iteration. In $H$ , shrink the set of vertices of each of these components to a single node to obtain graph $H ^ { \prime }$ (we will call the vertices of $H ^ { \prime }$ nodes for clarity). Notice that in going from $H$ to $H ^ { \prime } { \mathrm { . } }$ , all edges picked in $F$ before the current iteration have been shrunk. Clearly, the degree of a node in $H ^ { \prime }$ is equal to the degree of the corresponding set in H. Let us say that a node of $H ^ { \prime }$ corresponding to an active component is an active node; any other node will be called inactive. Each active node of $H ^ { \prime }$ has nonzero degree (since there must be an edge incident to it to satisfy its requirement), and $H ^ { \prime }$ is a forest. Now, remove all isolated nodes from $H ^ { \prime }$ . The remaining graph is a forest with average degree at most 2. By Lemma 22.7 the degree of each inactive node in this graph is at least 2, i.e., the forest has no inactive leaves. Hence, the average degree of active nodes is at most 2. ✷

Observe that the proof given above is essentially a charging argument: for each active node of degree greater than 2, there must be correspondingly many active nodes of degree 1, i.e., leaves, in the forest. The exact manner in which the dual conditions have been relaxed must also be clear now: in each iteration, the duals being raised have average degree at most 2. Lemmas 22.6 and 22.8 give:

Theorem 22.9 Algorithm 22.3 achieves an approximation guarantee of factor 2 for the Steiner forest problem.

The tight example given for the metric Steiner tree problem, Example 3.4, is also a tight example for this algorithm. Algorithm 22.3 places an upper bound of 2 on the integrality gap of LP-relaxation (22.2) for the Steiner forest problem. Example 22.10 places a lower bound of (essentially) 2 on this LP, even if restricted to the minimum spanning tree problem.

Let us run Algorithm 22.3 on an instance of the metric Steiner tree problem. If the edge costs satisfy the strict triangle inequality, i.e., for any three vertices $u , v , w , \ c ( u , v ) < c ( u , w ) + c ( v , w )$ , then it is easy to see that the algorithm will find a minimum spanning tree on the required vertices, i.e., it is essentially the algorithm for the metric Steiner tree problem presented in Chapter 3. Even if the triangle inequality is not strictly satisfied, the cost of the solution found is the same as the cost of an MST. Furthermore, if among multiple tight edges, the algorithm always prefers picking edges running between required vertices, it will find an MST. This clarifies the lower bound on which that algorithm was based.

The MST problem is a further special case: every pair of vertices need to be connected. Observe that when run on such an instance, Algorithm 22.3 essentially executes Kruskal’s algorithm, i.e., in each iteration, it picks the cheapest edge running between two connected components. Hence it finds an optimal MST. However, as shown in Example 22.10, the dual found may be as small as half the primal.

Example 22.10 Consider a cycle on n vertices, with all edges of cost 1. The cost of an optimal MST is $n - 1$ . the dual found is $n / 2$ . Algorithm 22.3 finds a dual of value $n / 2 \colon 1 / 2$ around each vertex. Indeed, this is an optimal dual solution, since there is a fractional primal solution of the same value: pick each edge to the extent of half. This places a lower bound of (essentially) 2 on the integrality gap of LP (22.2), even if restricted to the minimum spanning tree problem. ✷

## 22.4 Exercises

22.1 Show, using the max-flow min-cut theorem, that a subgraph of G has all the required paths if it does not violate any of the cut requirements in IP (22.1). Use this fact to show that IP (22.1) is an integer programming formulation for the Steiner forest problem.

22.2 Show that there is an approximation-factor-preserving reduction from the Steiner forest problem to the metric Steiner forest problem.

Show that there is no loss of generality in requiring that the edge costs satisfy the triangle inequality for the Steiner network problem.

Hint: The reasoning is the same as that for the Steiner tree problem.

22.3 How does the feasibility and approximation guarantee of the solution found change if

1. the pruning step of Algorithm 22.3 is replaced with the reverse delete step of Algorithm 18.4.

2. the reverse delete step of Algorithm 18.4 is replaced by the pruning step of Algorithm 22.3.

22.4 Give an example for which some cut raised by Algorithm 22.3 has degree at least 3 w.r.t. the primal solution found.

22.5 Run Algorithm 22.3 on an instance of the minimum spanning tree problem. Pick an arbitrary vertex as the root, and throw away all raised duals containing this vertex. Show that the cost of the tree found is twice the sum of the remaining duals.

Hint: Show that in an iteration which starts with k connected components, and lasts for time ∆, the total increase to the left-hand side of inequality (22.4) is precisely $2 ( k - 1 ) \varDelta$

22.6 Let us think of running Step 2 of Algorithm 22.3 continuously in time. Thus, in unit time, a dual grows a unit amount. Consider an instance of the

Steiner forest problem, $( G = ( V , E ) , c , S _ { 1 } , \ldots , S _ { k } )$ , and its modification in which one of the vertices from $V - ( S _ { 1 } \cup \ldots \cup S _ { k } )$ is added to one of the sets. Run Algorithm 22.3 on both these instances. Call these runs $R _ { 1 }$ and $R _ { 2 }$ , respectively.

1. Show that if $k = 1$ , i.e., the starting instance was a Steiner tree instance, then the following holds. If at time t two vertices $u , v \in S _ { 1 }$ are connected by a tight path in run $R _ { 1 }$ , then they are connected by a tight path at time t in run $R _ { 2 }$ as well.

2. Give a counterexample to the previous claim in case $k > 1$

22.7 (Goemans and Williamson [105]) Algorithm 22.3 actually works for a general class of problems that includes the Steiner forest problem as a special case. A function $f : 2 ^ { V } \to \{ 0 , 1 \}$ is said to be proper if it satisfies the following properties:

1. $f ( V ) = 0 ;$

2. $f ( S ) = f ( { \overline { { S } } } )$

3. If A and B are two disjoint subsets of V and $f ( A \cup B ) = 1$ , then $f ( A ) = 1$ or $f ( B ) = 1 \quad$

Notice that function f defined for the Steiner forest problem is a proper function. Consider the integer program (22.1) with $f$ restricted to be a proper function. Show that Algorithm 22.3 is in fact a factor 2 approximation algorithm for this class of integer programs.

22.8 (Goemans and Williamson [105]) Consider the following problem.

Problem 22.11 (Point-to-point connection) Given a graph $G = ( V , E )$ a cost function on edges c : $E \to \mathbf { Q } ^ { + }$ (not necessarily satisfying the triangle inequality) and two disjoint sets of vertices, S and $T ,$ , of equal cardinality, find a minimum cost subgraph that has a path connecting each vertex in $S$ to a unique vertex in T.

1. Give a factor 2 approximation algorithm for this problem.

Hint: Show that this can be formulated as an integer program using (22.1), with f being a proper function.

2. Relax the problem to requiring that each vertex in $S$ be connected to some vertex in $T$ (not necessarily unique). Give a factor 2 approximation algorithm for this problem as well.

Hint: Reduce to the Steiner tree problem.

22.9 (Goemans and Williamson [105]) Consider the following variant of the metric Steiner tree problem.

Problem 22.12 (Prize-collecting Steiner tree) We are given a complete undirected graph $G = ( V , E )$ and a special vertex $r \in V$ . Function cost : $E $ ${ \bf Q } ^ { + }$ satisfies the triangle inequality, and $\pi : V \to \mathbf { Q } ^ { + }$ is the penalty function for vertices. The problem is to find a tree containing r which minimizes the sum of the costs of the edges in the tree and the penalties of vertices not in the tree.

1. Consider the following integer program for this problem. It has a variable, $x _ { e } ,$ , for each edge e and a variable, $Z _ { T } .$ , for each set T of vertices not containing $r . \ Z _ { T }$ is set to 1 for the set $T$ of vertices that are not included in the optimal tree. Obtain the LP-relaxation and dual for this LP. The dual will have a variable for each set $S$ of vertices not containing r. Let’s call this variable $y _ { S }$

$$
\begin{array}{l l} \text { minimize } & \sum_ {e \in E} c _ {e} x _ {e} + \sum_ {T \subseteq V; r \not \in T} \left(Z _ {T} \sum_ {v \in T} \pi_ {v}\right) \\ \text { subject   to } & \sum_ {e \in \delta (S)} x _ {e} + \sum_ {T \supseteq S} Z _ {T} \geq 1, \\ & x _ {e} \in \{0, 1 \}, \\ & Z _ {T} \in \{0, 1 \}, \end{array} \quad \begin{array}{l l} & (2 2) \\ & S \subseteq V; r \not \in S \\ & e \in E \\ & T \subseteq V; r \not \in T. \end{array}\tag{22.6}
$$

2. The following primal–dual algorithm for this problem is along the lines of Algorithm 22.3. Initialize as follows. Each vertex $v \neq r$ is a singleton active set, with a charge of $\pi _ { v } .$ . Ordered list $F$ is set to $\varnothing .$ . The dual variables of all active sets are grown in a synchronized manner. As a dual grows, its charge decreases by the same amount. If set $S$ runs out of charge, it is declared dead and all of its unmarked vertices are marked with $^ { \mathfrak { a } } \bar { S } ^ { \mathfrak { n } }$ . When an edge e goes tight, it is added to $F .$ The rest of the action depends on the following cases.

• If e connects active set $S$ to $r :$ Set S is deactivated and is declared connected to r. All unmarked vertices of $S$ are marked $^ { 6 } r ^ { 5 }$

• If e connects active set $S$ to a set that is connected to r: Same action as in previous case.

• If e connects sets $S$ and $S ^ { \prime }$ which are either both active or one is active and one is dead: The active sets among $S$ and $S ^ { \prime }$ are deactivated. $S \cup S ^ { \prime }$ is declared active and is given the sum of the leftover charges of S and $S ^ { \prime } .$

When there are no more active sets, the algorithm performs a dynamic reverse delete on $F .$ This is an enhanced reverse delete procedure in which requirements change dynamically. All vertices marked $^ { 6 } r ^ { 5 }$ are labeled Required. We will say that $F$ is feasible if there is a path from each Required vertex to $r$ using edges of $F .$ . Let $e = ( u , v ) \in F .$ . Then the maximal dead set w.r.t. e, containing v is the maximal set $S$ such that $v \in S , u \notin S$ and $S$ was declared dead by the algorithm. If there is no set satisfying these conditions, then the maximal dead set w.r.t. e, containing v is defined to be ∅.

Edges $\textit { e } \in \textit { F }$ are considered in the reverse order in which they were inserted in $F$ . For each edge $e ,$ if $F - e$ is feasible, then e is removed from $F .$ Otherwise, suppose $\boldsymbol { e } = \left( u , v \right)$ , and let S be the maximal dead set w.r.t. e, containing v. If $S \neq \emptyset$ , then declare all vertices marked $^ { 6 6 } S ^ { 9 }$ Required. Repeat for the maximal dead set w.r.t. e containing $u .$

3. Show that at the beginning of the reverse delete step, $F$ is feasible. Also, show that F is never infeasible, even if the set of Required vertices grows.

4. Show that at the end, $F$ is a tree containing r and satisfying a) all vertices marked $^ { 6 } r ^ { 5 }$ are included in $F$ , and b) if a vertex marked $^ { 6 6 } S ^ { 9 }$ is included in $F$ , then all vertices marked $^ { 6 6 } T ^ { 5 }$ where $T \supseteq S .$ , are also included in $F .$

5. The primal solution is constructed as follows. For each edge e in $F _ { ; }$ , set $x _ { e }$ to 1, and set the remaining $x _ { e } \mathrm { { ^ { * } s } }$ to 0. For each maximal dead set $T$ , none of whose vertices are in $F ,$ set $Z _ { T }$ to 1. For all the remaining sets, set their $Z$ variable to 0.

Prove that this is a factor 2 approximation algorithm for the prizecollecting Steiner tree problem, by showing that the primal and dual solutions produced satisfy

$$
\sum_ {e \in E} c _ {e} x _ {e} + 2 \cdot \sum_ {T \subseteq V; r \not \in T} \left(Z _ {T} \sum_ {v \in T} \pi_ {v}\right) \leq 2 \cdot \sum_ {S \subseteq V; r \not \in T} y _ {S}.
$$

Hint: If $Z _ { T } = 1$ , the sum of penalties of vertices in $T$ equals the total dual contained in $T .$ . Twice the rest of the duals pays for the cost of the tree. Show the latter by proving, similar to Lemma $2 2 . 7$ , that at any point in the algorithm, there is at most one inactive set of degree 1 (the one containing r).

22.10 Consider the following generalization of the Steiner forest problem to higher connectivity requirements: the specified connectivity requirement function r maps pairs of vertices to $\{ 0 , \ldots , k \}$ , where $k$ is part $\mathrm { o f }$ the input. Assume that multiple copies of any edge can be used; each copy of edge e wil cost $c ( e )$ . Using Algorithm 22.3 as a subroutine, give a factor $2 \cdot ( \lfloor \log _ { 2 } k \rfloor + 1 )$ algorithm for the problem of finding a minimum cost graph satisfying all connectivity requirements.

22.11 We give below the bidirected cut relaxation for the Steiner tree problem. This is believed to have a smaller integrality gap than the undirected relaxation (22.2), though there is no proof of this fact yet. From graph $G ,$ obtain directed graph H by replacing each edge $( u , v )$ by the two edges $( u  v )$ and $( v  u )$ , each of the same cost as $( u , v )$ . Designate an arbitrary required vertex, say $r ,$ as the root. Say that $S \subset V$ is valid if it contains a required vertex and $r \in { \overline { { S } } }$ . Let $x _ { e }$ be an indicator variable for each edge $e \in H$ . The integer program is:

$$
\begin{array}{l l} \text {minimize} & \sum_ {e \in E} c _ {e} x _ {e} \\ \text {subject to} & \sum_ {e: e \in \delta (S)} x _ {e} \geq 1, \quad \text {valid set} S \\ & x _ {e} \in \{0, 1 \}, \qquad e \in H \end{array}\tag{22.7}
$$

1. Show that the optimal solution to this integer program is an optimal Steiner tree.

2. Obtain the LP-relaxation and dual for IP (22.7).

3. Show that the cost of the optimal solution to (22.7) and its relaxation is independent of the root chosen.

4. Show that the integrality gap of the relaxation is bounded by 2.

5. (Rajagopalan and Vazirani [226]) Show that the integrality gap of this relaxation for the following graph is $1 0 / 9$ . In this graph, the bold vertices are required and the remaining vertices are Steiner.

![](images/178e2925a822343eb3972f6add6c40fe7e0ab1c010b875ac423f3d953a3ffe8e.jpg)

6. (M. Goemans) The following family of graphs puts a lower bound of essentially $8 / 7$ on the integrality gap of relaxation (22.7). This is currently the worst example known.

![](images/d450123b63c76ff375b20229dcb4ad255e9dbd858ddff85f7886c5751e95aaf3.jpg)

Graph $G _ { n }$ has $n { + 1 }$ required vertices $a _ { 0 } , a _ { 1 } , \ldots , a _ { n }$ , and $n ^ { 2 }$ Steiner vertices $b _ { 1 } , \ldots , b _ { n }$ and $c _ { i j }$ and $d _ { i j }$ for $1 \leq i < j \leq n$ . The figure above gives edges and costs. Verify that the optimal Steiner tree has cost 4n and the optimal solution to relaxation (22.7) has cost $7 n + 1 / 2$

7. Construct other graphs for which this relaxation has a gap (it is not easy!).

8. (Edmonds [70]) Consider the special case that there are no Steiner vertices, i.e., we want to find a minimum spanning tree in G. Give a primal– dual algorithm that uses this relaxation to find a tree and a dual of the same cost, thereby showing that this relaxation is exact, $\mathrm { i . e . , }$ always has an integral optimal solution, for the minimum spanning tree problem. (In contrast, the undirected relaxation has an integrality gap of 2 even for the minimum spanning tree problem.)

22.12 (Pr¨omel and Steger [223]) This exercise develops an algorithm for the Steiner tree problem using the weighted matroid parity problem and the following structural fact. Let us say that a Steiner tree is 3-restricted if every Steiner vertex used in this tree has exactly three neighbors, all of which are required vertices. The cost of an optimal 3-restricted Steiner tree is within $5 / 3$ of the cost of an optimal Steiner tree (Zelikovsky [262]). Show that an optimal 3-restricted Steiner tree can be found in polynomial time, given an oracle for the weighted matroid parity problem. The latter problem is neither known to be in P nor is it known to be NP-hard. However, a randomized polynomial time algorithm is known for the case of unary weights. Use this fact, and scaling, to obtain a $5 / 3 + \varepsilon$ factor algorithm for the Steiner tree problem for any $\varepsilon > 0$

The weighted matroid parity problem is the following. Let (S, I) be a matroid, where S is the ground set and I is the collection of independent sets. Nonnegative weights are provided for elements of S. Further, a partition of S into pairs $( x _ { 1 } , x _ { 2 } ) , \dotsc , ( x _ { 2 n - 1 } , x _ { 2 n } )$ is also provided. The problem is to pick a maximum weight collection of pairs so that the picked elements form an independent set.

## 22.5 Notes

This chapter is based on the work of Goemans and Williamson [105]. The first factor 2 approximation algorithm for the Steiner forest problem was given by Agrawal, Klein, and Ravi [1]. See also the survey by Goemans and Williamson [107].
