---
title: "approximation-algorithms-ch-19-part-020"
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
pdf_path: "work/core-books/approximation-algorithms/parts/approximation-algorithms-ch-19-part-020.pdf"
raw_md: "raw/canonical/approximation-algorithms-ch-19-part-020/full.md"
---
A simple combinatorial algorithm achieving an approximation factor of $2 -$ 2/k for the multiway cut problem, Problem 4.1, was presented in Chapter 4. In this chapter we will use LP-rounding to improve the factor to $3 / 2$

In Chapter 14 we mentioned the remarkable property of half-integrality, possessed by LP-relaxations of certain NP-hard problems. The multiway cut problem and its generalization, the node multiway cut problem, possess this property. We will present a proof of this fact in Section 19.3. This is the only avenue known for obtaining a constant factor approximation algorithm for the latter problem.

## 19.1 An interesting LP-relaxation

The usual LP-relaxation for multiway cut has an integrality gap of $2 - 2 / k$ (see Exercise 19.2). The key to an improved approximation guarantee is a clever LP-relaxation.

Let $\varDelta _ { k }$ denote the $k - 1$ dimensional simplex. This is the $k - 1$ dimensional convex polytope in $\mathbf { R } ^ { k }$ defined by $\{ { \pmb x } \in { \bf R } ^ { k } \ | \ { \pmb x } \geq 0$ and $\textstyle \sum _ { i } x _ { i } = 1 \}$ , where $x _ { i }$ is the ith coordinate of point x. The simplex $\varDelta _ { 3 }$ is shown below.

![](images/b5ec0af26c3360e05b2e9a11a8ff40c0daf50f701ddc8f3e334f7c165705c1e8.jpg)

The relaxation will map each vertex of $G$ to a point in $\varDelta _ { k }$ . Each of the k terminals will be mapped to a distinct vertex of this simplex, i.e., to a unit vector $e _ { i } \in \mathbf { R } ^ { k }$ . Let $x _ { v } \in \varDelta _ { k }$ denote the point to which vertex v is mapped. The length of an edge $( u , v ) \in E$ will be defined to be half the $\ell _ { 1 }$ distance between $x _ { u }$ and $x _ { v }$ . The entire relaxation is:

(0, 1, 0)

$$
\begin{array}{l l} \text {minimize} & \sum_ {(u, v) \in E} c (u, v) d (u, v) \\ \text {subject to} & d (u, v) = \frac {1}{2} \sum_ {i = 1} ^ {k} | x _ {u} ^ {i} - x _ {v} ^ {i} |, \quad (u, v) \in E \\ & x _ {v} \in \varDelta_ {k}, \qquad \qquad \qquad v \in V \\ & x _ {s _ {i}} = e _ {i}, \qquad \qquad s _ {i} \in S \end{array}\tag{19.1}
$$

In Lemma 19.1 we show that this relaxation is really a linear program. An integral solution to this relaxation maps each vertex of G to a vertex of the simplex, respectively. Each edge $( u , v )$ has length either 0 or 1, depending on whether u and v are mapped to the same or diferent vertices of the simplex. Edges of length 1 form a multiway cut. The cost of this cut is the objective function value of this integral solution. Thus, an optimal integral solution corresponds to an optimal multiway cut.

Lemma 19.1 Relaxation (19.1) can be expressed as a linear program.

Proof: For each edge $( u , v )$ , replace the first constraint with:

$$
x _ {u v} ^ {i} \geq x _ {u} ^ {i} - x _ {v} ^ {i}, 1 \leq i \leq k
$$

$$
x _ {u v} ^ {i} \geq x _ {v} ^ {i} - x _ {u} ^ {i}, 1 \leq i \leq k
$$

$$
d (u, v) = \frac {1}{2} \sum_ {i = 1} ^ {k} x _ {u v} ^ {i}
$$

Since the objective function is being minimized, an optimal solution must satisfy $x _ { u v } ^ { i } = | x _ { u } ^ { i } - x _ { v } ^ { i } |$ . The rest of the constraints are clearly linear. ✷

Example 19.2 In the example given below, the optimal fractional multiway cut is cheaper than than the optimal integral cut. The mapping of vertices to $\varDelta _ { 3 }$ in the optimal fractional solution is shown below; it achieves a cost of 7.5. On the other hand, the optimal integral solution costs 8.

![](images/c80dd2b0d93d3418dc061665744e08723e8a967f354aaab44f55481d703161b8.jpg)

![](images/69a85bb8be1d886fb4bd352ecfc4f6e25cd52470b7c9ff6b0150c43293c85fd1.jpg)

The following property will greatly simplify matters:

Lemma 19.3 Let x be a feasible solution to relaxation $( 1 9 . 1 )$ . We may assume w $) . l . o . g .$ . that for each edge $( u , v ) \in E , x _ { u }$ and $x _ { v } \ d i f f e r$ in at most two coordinates.

Proof: We will divide edges by adding new vertices in such a way that this property holds and the cost of the solution remains unchanged.

Suppose that $( u , v ) \in E$ and that $x _ { u }$ and $x _ { v }$ difer in more than two coordinates. Replace this edge by two new edges $( u , w )$ and $( w , v )$ , where w is a new vertex. Each of the new edges is of the same cost as $c ( u , v )$ , thereby ensuring that the cost of the integral optimal solution is unchanged. We show below how to enforce $d ( u , v ) = d ( u , w ) + d ( w , v )$ , thereby ensuring that the cost of the fractional solution remains unchanged.

Consider the coordinates in which $x _ { u }$ and $x _ { v }$ difer. Let i be the coordinate in which the diference is minimum. Without loss of generality, assume $x _ { u } ^ { i } <$ $x _ { v } ^ { i }$ . Let $\alpha = x _ { v } ^ { i } - x _ { u } ^ { i }$ . There must be a coordinate $j$ such that $x _ { u } ^ { j } \ge x _ { v } ^ { j } + \alpha .$ We will define point $x _ { w }$ as follows. The ith and jth coordinates of $x _ { w }$ are $x _ { w } ^ { i } = x _ { u } ^ { i }$ and $x _ { w } ^ { j } = x _ { v } ^ { j } + \alpha$ . The remaining coordinates of $x _ { w }$ are the same as those of $x _ { v }$ . Clearly, $x _ { w } \in \varDelta _ { k }$ and $d ( u , v ) = d ( u , w ) + d ( w , v )$

Notice that u and w difer in two coordinates and w and v difer in fewer coordinates than u and v. Therefore, each edge of $E$ requires at most $k - 2$ such subdivisions to enforce the required property. ✷

## 19.2 Randomized rounding algorithm

Let x be an optimal solution to relaxation (19.1) satisfying the property stated in Lemma 19.3, and let $\mathrm { O P T } _ { f }$ denote its cost. Let $E _ { i }$ denote the subset of edges whose endpoints difer in coordinate $i ,$ i.e., $E _ { i } = \{ ( u , v ) \in$ $E \mid x _ { u } ^ { i } \neq x _ { v } ^ { i } \}$ }. Clearly, each edge e with $d ( e ) > 0$ will lie in two of these sets. Let $\begin{array} { r } { \bar { W _ { i } } = \bar { \sum _ { e \in E _ { i } } } c ( e ) d ( e ) } \end{array}$ . Renumber the terminals so that $W _ { k }$ is the largest of $W _ { 1 } , \ldots , W _ { k }$ . For $\rho \in ( 0 , 1 )$ , define

$$
B (s _ {i}, \rho) = \{v \in V | x _ {v} ^ {i} \geq \rho \}.
$$

Algorithm 19.4 operates as follows. It picks $\rho$ at random in $( 0 , 1 )$ and σ at random from the two permutations $( 1 , 2 , \ldots , k - 1 , k )$ and $( k - 1 , k -$ $2 , \ldots , 1 , k )$ . It uses $\rho$ and σ to construct a partition of V into k sets, $V _ { 1 } , \dots , V _ { k }$ • ensuring that $s _ { i } \in V _ { i }$ . Edges running between these sets will form the multiway cut.

If σ is the first (second) permutation, then these sets are constructed in the order $V _ { 1 } , V _ { 2 } , \ldots , V _ { k } \ ( V _ { k - 1 } , V _ { k - 2 } , \ldots , V _ { 1 } , V _ { k } ) . \ \mathrm { I f } \ \rho > 1 / 2$ , the sets $B ( s _ { i } , \rho )$ are pairwise disjoint. Observe that in this case the partition is not afected by $\sigma .$ , because V<sub>i</sub> is simply $B ( s _ { i } , \rho )$ for $1 \leq i \leq k - 1$ , and $V _ { k } = V - ( V _ { 1 } \cup \cdot \cdot \cdot \cup V _ { k - 1 } )$ If $\rho \le 1 / 2$ , the sets $B ( s _ { i } , \rho )$ overlap and σ plays a role, as illustrated in the figure below for $k = 3$

![](images/7c7d9bda7029f93475924743001860e4a052126a7a8d95b35304e6a08493794c.jpg)

![](images/16560fa9d502bd2eb5bfbae0c01ed83f7db63385e2667576567b9295c783e481.jpg)

![](images/9a6cfa90acf5417df96cb1dda9af75723cf79e80aed0e5f3b7bf2e6d6022b4cb.jpg)

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 19.4 (Multiway cut)
1. Compute an optimal solution, x, to relaxation (19.1).
2. Renumber the terminals so that  $W_{k}$  is largest among  $W_{1},\ldots,W_{k}$ .
3. Pick uniformly at random  $\rho\in(0,1)$  and
 $\sigma\in\{(1,2,\ldots,k-1,k),(k-1,k-2,\ldots,1,k)\}$ .
4. For i=1 to k-1:  $V_{\sigma(i)}\leftarrow B(s_{i},\rho)-\bigcup_{j&lt;i}V_{\sigma(j)}$ .
5.  $V_{k}\leftarrow V-\bigcup_{i&lt;k}V_{i}$ .
6. Let C be the set of edges that run between sets in the partition
 $V_{1},\ldots,V_{k}$ . Output C.
</div>

We will show that the expected cost of the multiway cut produced by the algorithm, $\mathbf { E } [ c ( C ) ]$ , is at most $( 1 . 5 - 1 / k ) \cdot \mathrm { O P T } _ { f }$ . The following lemma will be critical.

Lemma 19.5 $I f e \in E - E _ { k } , \mathbf { P r } [ e \in C ] \leq 1 . 5 d ( e )$

and $i f e \in E _ { k } , \mathbf { P r } [ e \in C ] \le d ( e )$

Proof: Suppose e $\in E - E _ { k }$ . Let $\boldsymbol { e } = \left( u , v \right)$ , and let i and j be the coordinates in which $x _ { u }$ and $x _ { v }$ difer. There are two cases: the intervals $[ x _ { u } ^ { i } , x _ { v } ^ { i } ]$ and $[ x _ { v } ^ { j } , x _ { u } ^ { j } ]$ either overlap or they are disjoint. These two cases are shown below. Note that in either case the two intervals have the same length since $x _ { v } ^ { i } - x _ { u } ^ { i } =$ $x _ { u } ^ { j } - x _ { v } ^ { j } = d ( e )$ . Intervals α and $\beta$ are defined in the figure below for the two cases.

![](images/44440221eddfcc0ca06edfe2900299f041a73af61cafd45338eac072446782e8.jpg)  
19.2 Randomized rounding algorithm

Observe that the vertices u and v can end up in one of three sets, $V _ { i } , V _ { j }$ or $V _ { k }$ . Furthermore, if $\rho \in [ 0 , 1 ] - ( \alpha \cup \beta )$ , then both vertices will end up in the same set, and edge e will not be in the cut. Clearly, $\mathbf { P r } [ \rho \in ( \alpha \cup \beta ) ] =$ $| \alpha | + | \beta | \leq 2 d ( e )$ .

The critical observation that leads to the desired bound is that in the event $\rho \in \alpha$ and $\sigma ( j ) < \sigma ( i )$ , u and v will both be put in the set $V _ { j } ,$ and thus e will not be in the cut. Clearly, the probability of this event is $| \alpha | / 2$ Therefore

$$
\mathbf {P r} [ e \in C ] = | \beta | + | \alpha | / 2 \leq 1. 5 d (e).
$$

Next, suppose that $e \in E _ { k }$ , and that its endpoints difer in coordinates i and k. In this case $\sigma ( i ) < \sigma ( k )$ , and u and v will end up in diferent sets only if $\rho$ falls between $x _ { u } ^ { i }$ and $x _ { v } ^ { i }$ . The probability of this is $d ( e )$ ✷

Lemma 19.6 The multiway cut, C, output by Algorithm 19.4 satisfies

$$
\mathbf {E} [ c (C) ] \leq (1. 5 - 1 / k) \mathrm{OPT} _ {f}.
$$

Proof: Clearly, C forms a multiway cut. Now, $\begin{array} { r } { \mathrm { O P T } _ { f } = \sum _ { e } c ( e ) d ( e ) } \end{array}$ . Since each edge with nonzero length is in two of the sets $E _ { i } , \sum _ { i = 1 } ^ { k } W _ { i } = 2 \cdot \mathrm { O P T } _ { f } .$ Since k was chosen so that $W _ { k }$ is the largest of these sets, $W _ { k } \ge ( 2 / k ) { \cdot } \mathrm { O P T } _ { f }$ Therefore

$$
\begin{array}{l} \mathbf {E} [ c (C) ] = \sum_ {e \in E} c (e) \mathbf {P r} [ e \in C ] = \sum_ {e \in E - E _ {k}} c (e) \mathbf {P r} [ e \in C ] + \sum_ {e \in E _ {k}} c (e) \mathbf {P r} [ e \in C ] \\ \leq 1. 5 \sum_ {i = 1} ^ {k - 1} W _ {i} + W _ {k} = 1. 5 \sum_ {i = 1} ^ {k} W _ {i} - 0. 5 W _ {k} \\ \leq (1. 5 - 1 / k) \cdot \mathrm{OPT} _ {f} \end{array}
$$

where the first inequality follows from Lemma 19.5.

Lemma 19.6 places an upper bound of $1 . 5 - 1 / k$ on the integrality gap of relaxation 19.1 (see the notes in Section 19.5 for references to a slightly better result). The worst lower bound know on the integrality gap is $8 / ( 7 + \frac { 1 } { k - 1 } )$ ; Example 19.2 places a lower bound of $1 6 / 1 5$

The bound on the expected weight of the multiway cut established in Lemma 19.6 can be converted into a high probability statement using standard techniques (see Exercises 1.10 and 19.4). Hence we get

Theorem 19.7 There is a $3 / 2$ factor randomized approximation algorithm for the multiway cut problem.

## 19.3 Half-integrality of node multiway cut

The following is a generalization of the multiway cut problem, in the sense that there is an approximation factor preserving reduction from the multiway cut problem to it (see Exercise 19.13).

Problem 19.8 (Node multiway cut) Given a connected, undirected graph $G = ( V , E )$ with an assignment of costs to vertices, $c : V \to \mathbf { R } ^ { + }$ , and a set of terminals $S = \{ s _ { 1 } , s _ { 2 } , \ldots , s _ { k } \} \subseteq V$ that form an independent set in $G ,$ a node multiway cut is a subset of $V - S$ whose removal disconnects the terminals from each other. The node multiway cut problem asks for the minimum cost such subset.

We will show that the relaxation to the following integer program always has a half-integral optimal solution. A factor $2 - 2 / k$ approximation algorithm will follow from this fact (see Exercise 19.11). In this program we have introduced a $0 / 1$ variable $d _ { v }$ for each vertex $v \in V - S$ , which indicates whether vertex v has been picked. Let $\mathcal { P }$ denote the set of all paths running between distinct terminals. There is a constraint for each path $p$ in $\mathcal { P } - \mathrm { i t }$ ensures that at least one vertex is picked from each path.

$$
\begin{array}{l l} \text {minimize} & \sum_ {v \in V - S} c _ {v} d _ {v} \\ \text {subject to} & \sum_ {v \in p} d _ {v} \geq 1, \quad p \in \mathcal {P} \\ & d _ {v} \in \{0, 1 \}, \quad v \in V - S \end{array}
$$

The LP-relaxation is given below. As before, we will interpret $d _ { v } \mathrm { { ' s } }$ as distance labels. With respect to an assignment to these distance labels, let us define the length of a path to be the sum of distance labels of nonterminals on this path. The distance between a pair of vertices will be the length of the shortest path between them. A solution, $^ { d , }$ is feasible only if the distance between every pair of terminals is at least 1.

$$
\text { minimize } \quad \sum_ {v \in V - S} c _ {v} d _ {v}\tag{19.2}
$$

$$
\begin{array}{l l} \text {subject to} & \sum_ {v \in p} d _ {v} \geq 1, \quad p \in \mathcal {P} \\ & d _ {v} \geq 0, \quad v \in V - S \end{array}
$$

As in Chapter 18, the dual will be interpreted as seeking a maximum multicommodity flow. The commodities flow between distinct terminals, and the constraint is that the total amount of flow through a vertex be bounded by its cost.

$$
\begin{array}{l l} \text {subject to} & \sum_ {p: v \in p} f _ {p} \leq c _ {e}, \quad v \in V - S \\ & f _ {p} \geq 0, \quad \quad \quad \quad p \in \mathcal {P} \end{array}\tag{19.3}
$$

Let d be an optimal solution to LP (19.2). We will show how to obtain, eficiently, a half-integral optimal solution from d . For the purposes of proof, let $f$ be an optimal solution to the dual LP. Complementary slackness conditions give:

Primal conditions: For each $v \in V - S , \operatorname { i f } d _ { v } > 0$ then v must be saturated.

Dual conditions: For each path $p ,$ if $f _ { p } > 0$ then the length of $p$ is exactly 1.

Consider graph G with distance labels on vertices $v \in V - S$ specified by d. For each terminal $s _ { i } .$ , define its region $S _ { i }$ to be the set of vertices reachable from $s _ { i }$ by paths of length zero (we will assume that $s _ { i } ~ \in ~ S _ { i } )$ Define the boundary, $B _ { i }$ , of $S _ { i }$ to be all vertices that are adjacent to $S _ { i } , \mathrm { i . e . }$ $B _ { i } = \{ v \in { \overline { { S _ { i } } } }$ | for some $u \in S _ { i } , ( u , v ) \in E \}$ . The feasibility of d ensures that the k regions are disjoint and the boundaries do not contain any terminals.

## Claim 19.9 Suppose $v \in B _ { i } \cap B _ { j } ~ f o r ~ i \neq j$ . Then $d _ { v } = 1$

Proof: Clearly there is a path from $s _ { i }$ to $s _ { j }$ on which v is the only vertex having a positive distance label. The claim follows from the feasibility of d. ✷

Let $\textstyle M = \bigcup _ { i = 1 } ^ { k } B _ { i }$ be the set of boundary vertices. Partition this into two sets: $M ^ { \mathrm { i n t } }$ being boundary vertices that occur in two or more boundary sets, and $M ^ { \mathrm { d i s j } }$ being the rest; each vertex in $M ^ { \mathrm { d i s j } }$ is in a unique boundary set. $\mathrm { B y }$ Claim 19.9, each vertex in $M ^ { \mathrm { i n t } }$ has distance label of 1.

Lemma 19.10 Let $p$ be a path between two distinct terminals such that $f _ { p } >$ 0. Then, from the vertices in M, $p$ uses either exactly one vertex of $M ^ { \mathrm { i n t } }$ or exactly two vertices of $M ^ { \mathrm { d i s j } }$ j

Proof: By the dual complementary slackness condition, the length of $p$ must be exactly 1. Thus, if $p$ uses a vertex of $M ^ { \mathrm { i n t } }$ , then it cannot have any other vertices of M on it.

Suppose $p$ uses three or more vertices of $M ^ { \mathrm { d i s j } }$ . Assume that $p$ runs from $s _ { i }$ to $s _ { j }$ and that u and w are the first and last vertices of $M ^ { \mathrm { d i s j } }$ on $p ,$ respectively. Let v be any intermediate vertex of $M ^ { \mathrm { d i s j } }$ on $p .$ Since $v \in M ^ { \mathrm { d i s j } }$ v must be in a unique boundary, say $B _ { k } ; k = i$ or $k = j$ are possible.

![](images/b10d1f52dd2d9a1b0034994c0bec2f86c1fe8f6894e08a69e21efd0632e459c7.jpg)

Let $q$ be a path connecting $v$ to $s _ { k }$ via vertices in $S _ { k } \mathbf { ; }$ ; such a path must exist since $v \in B _ { k }$ . Now consider the following two paths: the first consists of the part of the path $p$ from $s _ { i }$ to v followed by $q ,$ and the second consists of the reverse of $q$ followed by the part of $p$ from v to $s _ { j }$ . At least one of these is a valid path running between distinct terminals (even if $k = i$ or $k = j )$ Moreover, since it is missing at least one of the positive distance label vertices of $p ,$ it must have length strictly less than 1. This contradicts the feasibility of d. The lemma follows. ✷

Let h be a solution to LP (19.2) that assigns distance labels of 1 to each vertex in $M ^ { \mathrm { i n t } } , 1 / 2$ to each vertex in $M ^ { \mathrm { d i s j } }$ , and 0 to all remaining vertices.

Lemma 19.11 h is an optimal solution to LP (19.2).

Proof: Any valid path, $p ,$ from terminal $s _ { i }$ to $s _ { j }$ must use vertices of both boundary sets $B _ { i }$ and $B _ { j }$ . Suppose it uses $v \in B _ { i } \cap B _ { j }$ . By definition $v \in M ^ { \mathrm { i n t } }$ 9 and so $h _ { v } = 1$ . Otherwise, it uses two vertices of $\boldsymbol { \dot { M } } ^ { \mathrm { d i s j } }$ . In either case the length of $p$ is at least 1, thus showing that h is a feasible solution.

Next we will show that the objective function value of h is the same as that of flow $f _ { i }$ thereby showing that h is optimal. Partition paths carrying nonzero flow in $f$ into two sets: $\mathcal { P } _ { 1 }$ consists of paths that use one vertex of $M ^ { \mathrm { i n t } }$ and $\mathcal { P } _ { 2 }$ consists of paths that use two vertices of $M ^ { \mathrm { d i s j } }$ . By Lemma 19.10 these are the only two possibilities. $\mathrm { B y }$ the primal complementary slackness conditions and the optimality of $^ { d , }$ each vertex in $M$ is saturated by $f .$

Therefore, the total flow carried by paths in $\mathcal { P } _ { 1 }$ is $\textstyle \sum _ { v \in M ^ { \mathrm { i n t } } } c _ { v }$ and by paths in $\mathcal { P } _ { 2 }$ is ${ \begin{array} { r l } { { \frac { 1 } { 2 } } \sum _ { v \in M ^ { \mathrm { d i s j } } } c _ { v } } \end{array} }$ . Hence the total flow is

$$
\sum_ {v \in M ^ {\mathrm{int}}} c _ {v} + \frac {1}{2} \sum_ {v \in M ^ {\mathrm{disj}}} c _ {v} = \sum_ {v \in V - S} h _ {v} c _ {v}.
$$

This proves the lemma.

Clearly h can be obtained from an optimal solution, $^ { d , }$ to LP (19.2) in polynomial time. This gives:

Theorem 19.12 $L P \ ( { \cal { 1 9 . 2 } } )$ always has a half-integral solution. Moreover, any optimal solution can be converted into such a solution in polynomial time.

## 19.4 Exercises

In Chapter 4 we presented a $2 - 2 / k$ factor algorithm for the minimum multiway cut problem by comparing the solution found to the integral optima solution. In the next two exercises we develop an algorithm with the same guarantee using LP-duality.

19.1 Given terminals $s _ { 1 } , \ldots , s _ { k }$ , consider the multicommodity flow problem in which each pair of terminals can form a source–sink pair. Thus there are $\binom { k } { 2 }$ commodities. Give an LP for maximizing this multicommodity flow and  obtain the dual LP. The dual seeks a distance label assignment for edges satisfying the triangle inequality and ensures that the distance between any two terminals is at least 1. An optimal solution to the dual can be viewed as a fractional multiway cut.

19.2 Consider the following algorithm for finding a multiway cut. Solve the dual LP to obtain an optimal fractional multiway cut. This gives a distance label assignment, say d. Pick $\rho$ at random in the interval $[ 0 , \textstyle { \frac { 1 } { 2 } } ]$ . An edge $( u , v )$ is picked if for some terminal s, $d ( u , s ) \leq \rho \leq d ( v , s )$ . Prove that the expected cost of the cut picked is at most twice the optimal fractional multiway cut. Derandomize this algorithm, and give a modification to make it a factor $2 - 2 / k$ algorithm.

Hint: Show that for each edge $( u , v )$ , the probability that it is picked is bounded by $2 \cdot d ( u , v )$

19.3 In an attempt to improve the factor of the previous algorithm, suppose we choose $\rho$ at random in the interval [0, 1]. What goes wrong? How is this rectified in Algorithm 19.4?

## 19.4 Derive Theorem 19.7 from Lemma 19.6.

Hint: Lemma 19.6 implies that $\mathbf { P r } [ c ( C ) \leq 1 . 5 \cdot \mathrm { O P T } _ { f } ] \geq 2 / k \geq 2 / n$ . Run Algorithm 19.4 polynomially many times and output the best cut.

19.5 How does the approximation guarantee of the algorithm change if $\sigma$ is picked to be a random permutation from $S _ { k } ?$

19.6 (Y. Rabani) For the case $k = 3 .$ , replace the randomized rounding procedure of Algorithm 19.4 with the following. Pick $\rho _ { 1 }$ and $\rho _ { 2 }$ independently and uniformly from $( 0 , 1 )$ . Pick one of the three dimensions at random, say $i .$ Merge with $s _ { i }$ all nonterminals v satisfying $x _ { v } ^ { i } \geq \rho _ { 1 }$ . Arbitrarily pick one of the remaining two dimensions, say $j ,$ and denote the third dimension by k. Merge with $s _ { j }$ all remaining nonterminals v satisfying $x _ { v } ^ { j } + x _ { v } ^ { i } / 2 \ge \rho _ { 2 }$ . Finally, merge with $s _ { k }$ all remaining nonterminals. Show that this modified algorithm achieves an approximation guarantee of $7 / 6$ for the 3-way cut problem.

19.7 We present another relaxation for the multiway cut problem for which the worst integrality gap known is no worse than that for LP (19.1) (see also Chapter 30). Given an undirected graph $G = ( V , E )$ with costs on edges, obtain the directed graph H by replacing each edge $( u , v )$ of G by two directed edges $( u  v )$ and $( v  u )$ , each having the same cost as $( u , v )$ . Assign a $0 / 1$ indicator variable $d _ { e }$ to each edge e in H. Suppose the terminals are numbered $s _ { 1 } , \ldots , s _ { k }$ in some order. Let $\mathcal { P }$ be the collection of all simple paths from a lower-numbered terminal to a higher-numbered terminal. Consider the following bidirected integer programming formulation for the multiway cut problem.

$$
\begin{array}{l l} \text {minimize} & \sum_ {e \in H} c (e) d _ {e} \\ \text {subject to} & \sum_ {e \in p} d _ {e} \geq 1, \quad p \in \mathcal {P} \\ & d _ {e} \in \{0, 1 \}, \quad e \in H \end{array}\tag{19.4}
$$

1. Show that an optimal solution to IP (19.4) yields an optimal solution to the multiway cut problem.

2. Obtain the LP-relaxation and dual program. Give a good physical interpretation of the dual.

3. Show that the graph given in Example 19.2 has an integrality gap of 16/15 for this relaxation as well (by showing a primal and dual solution of cost 7.5).

4. Show that the cost of the optimal solution to the integer program and the relaxation is independent of the ordering imposed on the terminals.

19.8 Consider Algorithm 4.3 for the multiway cut problem. Show that the analogous algorithm for the node multiway cut problem, based on isolating cuts, does not achieve a constant factor. What is the best factor you can prove for this algorithm?

19.9 The multiway cut problem also possesses the half-integrality property. Give an LP for the multiway cut problem similar to LP (19.2), and prove this fact.

19.10 Show that the lower bound on $\mathrm { O P T }$ given by LP (19.2) can be smaller by a factor of $2 - 2 / k$ by giving a graph in which the optimal node multiway cut is $2 - 2 / \lambda$ k times bigger than the maximum flow.

19.11 Theorem 19.12 leads directly to a factor 2 approximation algorithm for the node multiway cut problem, by rounding up the halves in a halfintegral solution. Obtain a factor $2 - 2 / k$ algorithm, and give a family of tight examples for this algorithm.

Hint: Not all vertices of $M ^ { \mathrm { d i s j } }$ are required for obtaining a multiway cut. For the tight example, consider the following graph.

![](images/cc4278d074e98310aec902371275626b01f3d04b3022fa3b2e79950921402c7b.jpg)  
19.12 Consider the following problem.

Problem 19.13 (Directed multiway cut) Given a directed graph $G =$ $( V , E )$ with an assignment of capacities to edges, $c : E \to \mathbf { R } ^ { + }$ , and a set of terminals $S = \{ s _ { 1 } , s _ { 2 } , \dotsc , s _ { k } \} \subseteq V$ , a directed multiway cut is a set of edges whose removal ensures that the remaining graph has no path from $s _ { i }$ to $s _ { j }$ for each pair of distinct terminals $s _ { i }$ and $s _ { j }$ . The directed multiway cut problem asks for the minimum cost such set.

Obtain an LP-relaxation for this problem similar to LP (19.2). The dual can be interpreted as a directed multicommodity flow LP. Find the optimal fractional directed multiway cut and flow in the following example:

![](images/32f6480fbe44684fce44096dfb581ce1a7ffc68fa1fe646d9305647e603d18a7.jpg)

Notice that unlike LP (19.2), this relaxation does not always have an optimal half-integral solution.

19.13 Let us define the following two problems:

Problem 19.14 (Subset feedback edge set) Given a connected, undirected graph $G = ( V , E )$ with an assignment of weights to edges, w : $E  \mathbf { R } ^ { + }$ 2 and a set of special vertices $S = \{ s _ { 1 } , s _ { 2 } , \dotsc , s _ { k } \} \subseteq V$ , a subset feedback edge set is a set of edges whose removal ensures that the remaining graph has no cycle containing a special vertex. The subset feedback edge set problem asks for the minimum weight such set.

Problem 19.15 (Subset feedback vertex set) Given a connected, undirected graph $G \ : = \ : ( V , E )$ with an assignment of weights to vertices, $w : V \to \mathbf { R } ^ { + }$ , and a set of special vertices $S = \{ s _ { 1 } , s _ { 2 } , . . . , s _ { k } \} \subseteq V ,$ , a subset feedback vertex set is a subset of $V - S$ whose removal ensures that the remaining graph has no cycle containing a special vertex. The subset feedback vertex set problem asks for the minimum weight such set.

These and previously introduced problems are related by approximation factor preserving reductions given in the following figure (each arrow represents such a reduction). Give these reductions. For a definition of such reductions, see Section A.3.1.

![](images/b4631e636ae2530959f23fc6a4d8791e40728ecfb73ffe4ff181a0d3767954ee.jpg)

The current best factors known for multiway cut and subset feedback vertex set are 1.34 and 8, respectively. For the rest of the problems, the current best factor is 2.

## 19.5 Notes

Algorithm 19.4 is due to Calinescu, Karlof, and Rabani [36]. The current best guarantee known for the multiway cut problem is 1.3438, due to Karger, Klein, Stein, Thorup, and Young [157]. This is also the best upper bound known on the integrality gap of the relaxation used. Freund and Karlof [87] give a family of instances achieving a lower bound of $8 / ( 7 + \frac { 1 } { k - 1 } )$ on the integrality gap; Example 19.2 is from their paper. Theorem 19.12 is due to Garg, Vazirani, and Yannakakis [96]. For currently best approximation algorithms known for directed multiway cut, subset feedback edge set, and subset feedback vertex set, see Naor and Zosin [210], Even, Naor, Schieber, and Zosin [76], and Even, Naor, and Zosin [77], respectively.