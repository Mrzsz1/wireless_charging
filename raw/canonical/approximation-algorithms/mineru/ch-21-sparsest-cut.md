---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-21"
chapter_number: 21
chapter_title: "Sparsest Cut"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 198
source_page_end: 215
printed_page_start: 180
printed_page_end: 197
part_ids: ["approximation-algorithms-ch-21-part-022"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Sparsest Cut (MinerU semantic layer)

<!-- source-pages: 198-215; printed-pages: 180-197; mineru-part: approximation-algorithms-ch-21-part-022 -->

## 21 Sparsest Cut

In this chapter we will obtain an approximation algorithm for the sparsest cut problem using an interesting LP-rounding procedure that employs results on low distortion embeddings of metrics in $\ell _ { 1 }$ spaces. As mentioned in Chapter 20, we will get as a corollary an approximate max-flow min-cut theorem for the demands version of multicommodity flow. Approximation algorithms for several other important problems will also follow.

## 21.1 Demands multicommodity flow

Problem 21.1 (Demands multicommodity flow) Let $G = ( V , E )$ be an undirected graph with a nonnegative capacity $c _ { e }$ for each edge $e \in E$ Let $\{ ( s _ { 1 } , t _ { 1 } ) , \ldots , ( s _ { k } , t _ { k } ) \}$ be a specified set of pairs of vertices, where each pair is distinct, but vertices in diferent pairs are not required to be distinct. $\mathrm { A }$ separate commodity is defined for each $( s _ { i } , t _ { i } )$ pair; for convenience, we will think of $s _ { i }$ as the source and $t _ { i }$ as the sink of this commodity. For each commodity $i ,$ a nonnegative demand, dem(i), is also specified. The objective is to maximize $f ,$ called throughput, such that for each commodity $i , \ f$ dem(i) units of this commodity can be routed simultaneously, subject to flow conservation and capacity constraints, i.e., each commodity must satisfy flow conservation at each vertex other than its own source and sink, and the sum of flows routed through an edge, in both directions combined, should not exceed the capacity of this edge. We will denote the optimal throughput by $f ^ { * }$

Consider a cut $( S , { \overline { { S } } } )$ in G. Let $c ( S )$ denote the capacity of edges in this cut and dem(S) denote the total demand separated by this cut, i.e.,

$$
\operatorname{dem} (S) = \sum_ {i: | \{s _ {i}, t _ {i} \} \cap S | = 1} \operatorname{dem} (i).
$$

Clearly, the ratio of these quantities places an upper bound on the throughput, i.e., $\begin{array} { r } { f ^ { * } \leq \frac { c ( S ) } { \mathrm { d e m } ( S ) } } \end{array}$ . This motivates:

Problem 21.2 (Sparsest cut) Let $G = ( V , E )$ be an undirected graph with capacities, source–sink pairs, and demands defined as in Problem 21.1.

The sparsity of cut (S, S) is given by $\frac { c ( S ) } { \mathrm { d e m } ( S ) }$ . The problem is to find a cut of minimum sparsity. We will denote the sparsity of this cut by $\alpha ^ { * }$

Among all cuts, $\alpha ^ { * }$ puts the most stringent upper bound on $f ^ { * }$ . Is this upper bound tight? Example 21.3 shows that it is not. However, minimum sparsity cannot be arbitrarily larger than maximum throughput; we will show that their ratio is bounded by $O ( \log k )$ ).

Example 21.3 Consider the bipartite graph $K _ { 3 , 2 }$ with all edges of unit capacity and a unit demand between each pair of nonadjacent vertices – a total of four commodities.

![](images/124a405b28a40e770b8fc15a78d9d698efdbe7ce302da4e29f27547c7baeba51.jpg)  
It is easy to check that a sparsest cut of $K _ { 3 , 2 }$ has sparsity 1. This graph can be viewed as the union of two stars $K _ { 3 , 1 }$ (the centers of the stars are the vertices on the right side of the bipartition), and, as in Example 18.2, we get the unique way of routing one unit of each of the three commodities having source and sink on the left side of the bipartition. However, this saturates all edges, making it impossible to route the fourth commodity. Hence, throughput is strictly smaller than 1. ✷

## 21.2 Linear programming formulation

We start by giving a linear programming formulation of the problem of maximizing throughput, $f .$ Let $\mathcal { P } _ { i } = \{ q _ { j } ^ { i } \}$ denote the set of all paths between $s _ { i }$ and $t _ { i }$ . Introduce variable $f _ { j } ^ { i }$ to denote the flow of commodity i sent along path $q _ { j } ^ { \ i }$ . The first set of constraints ensures that the demand of each commodity is met (with factor $f )$ , and the second set are edge capacity constraints.

$$
\begin{array}{l l} \text {maximize} & f \\ \text {subject to} & \sum_ {j} f _ {j} ^ {i} \geq f \cdot \operatorname{dem} (i), \quad i = 1, \ldots , k \\ & \sum_ {e \in q _ {j} ^ {i}} f _ {j} ^ {i} \leq c _ {e}, \quad e \in E \\ & f \geq 0 \\ & f _ {j} ^ {i} \geq 0 \end{array}\tag{21.1}
$$

Define the graph H with vertex set $V _ { H } = \{ s _ { i } , t _ { i } | 1 \le i \le k \}$ and edge set $E _ { H } = \{ ( s _ { i } , t _ { i } ) | 1 \leq i \leq k \}$ to be the demand graph. For each edge $e = ( s _ { i } , t _ { i } )$ of $H .$ , let dem $\mathfrak { r } ( e ) = d e m ( i )$ . We will show that the dual to LP (21.1) yields a metric $( V , d )$ satisfying:

Theorem 21.4 Let $f ^ { * }$ denote the optimal throughput. Then,

$$
f ^ {*} = \min _ {m e t r i c \textbf {d}} \frac {\sum_ {e \in G} c _ {e} d _ {e}}{\sum_ {e \in H} \mathrm{dem} (e) d _ {e}}.
$$

Let $l _ { i }$ and $d _ { e }$ be dual variables associated with the first and second set of inequalities of LP (21.1). We will interpret $d _ { e } \mathrm { ' s }$ as distance label assignments to the edges of G. The first set of inequalities ensures that for each commodity $i , l _ { i }$ is upper bounded by the length of any path from $s _ { i }$ to $t _ { i }$ w.r.t. the distance label assignment.

$$
\begin{array}{l l} \text {minimize} & \sum_ {e \in E} c _ {e} d _ {e} \\ \text {subject to} & \sum_ {e \in q _ {j} ^ {i}} d _ {e} \geq l _ {i}, \qquad q _ {j} ^ {i} \in \mathcal {P} _ {i}, i = 1, \ldots , k \\ & \sum_ {i = 1} ^ {k} l _ {i} \operatorname * {d e m} (i) \geq 1 \\ & d _ {e} \geq 0, \qquad \qquad e \in E \\ & l _ {i} \geq 0, \qquad \qquad i = 1, \ldots , k \end{array}\tag{21.2}
$$

Example 21.5 For the instance given in Example 21.3, the optimal throughput is $f ^ { * } = 3 / 4$ ; this corresponds to routing the four commodities as follows:

![](images/6ee326741080e412079bcbb9e8b2e1c5fb071beb3bc8d847337d338ac74499ec.jpg)

The optimal dual solution is: $d _ { e } = 1 / 8$ for each edge e and $l _ { i } = 1 / 4$ for each commodity i. It would be instructive for the reader to verify feasibility and optimality of these solutions. ✷

Claim 21.6 There is an optimal distance label assignment d for the dual program (21.2) that is a metric on V . Furthermore, for each commodity i, $l _ { i } =$ $d _ { ( s _ { i } , t _ { i } ) }$ , and the second inequality holds with equality, $\begin{array} { r l } { i . e . , \sum _ { i } d _ { ( s _ { i } , t _ { i } ) } \mathrm { d e m } ( i ) = } \end{array}$ 1.

Proof: If for some three points $u , v ,$ and w, $d _ { u w } > d _ { u v } + d _ { v w }$ , then decrease $d _ { u w }$ to $d _ { u v } + d _ { v w }$ . Since this does not decrease the shortest path between any $s _ { i } - t _ { i }$ pair, the solution still remains feasible. Moreover, the objective function value cannot increase by this process. Continuing in this manner, we will obtain a metric on $V .$

Now, the length of a shortest path from $s _ { i }$ to $t _ { i }$ is given by the distance label $d _ { ( s _ { i } , t _ { i } ) }$ . Setting $l _ { i } = d _ { ( s _ { i } , t _ { i } ) }$ does not change the feasibility or the objective function value of the solution. Finally, if the second inequality holds strictly, then we can scale down all distance labels without violating feasibility, thus contradicting the optimality of $\mathbf { \delta } d .$ ✷

By Claim 21.6, the dual program yields a metric $( V , d )$ that minimizes

$$
\frac {\sum_ {e \in G} c _ {e} d _ {e}}{\sum_ {e \in H} \mathrm{dem} (e) d _ {e}}.
$$

By the LP-duality theorem, this equals the optimal throughput. This proves Theorem 21.4.

## 21.3 Metrics, cut packings, and $\ell _ { 1 }$ -embeddability

In Section 21.3.1, we will define the notion of a cut packing for a metric and will show that the question of finding a good approximation to the sparsest cut for graph G reduces to that of finding a “good” cut packing for the metric obtained in Theorem 21.4. The latter question is reduced, in Section 21.3.2, to the question of finding a “good” $\ell _ { 1 } \cdot$ -embedding for the metric. Eventually, Section 21.4 deals with finding the embedding itself.

## 21.3.1 Cut packings for metrics

Let us think of a metric $( V , d )$ as defining the lengths of edges of the complete graph on V . Let $E _ { n }$ denote the set of all edges in the complete graph on n vertices. Let y be a function assigning nonnegative values to subsets of $V _ { : }$ $\mathrm { i . e . , ~ } y : 2 ^ { V } \to \mathbf { R } ^ { + }$ . We will denote the value of $y$ on set S by $y _ { S } .$ . As before, let us say that edge e feels $y _ { S }$ if e is in the cut (S, S). The amount of cut that edge e feels is $\sum _ { \boldsymbol { S } : e \in \delta ( \boldsymbol { S } ) } \boldsymbol { y } ( \boldsymbol { S } )$ . Function $y$ is called a cut packing for metric $( V , d )$ if no edge feels more cut than its length, i.e., for each edge $e \in E _ { n } , \sum _ { S : e \in \delta ( S ) } y ( S ) \leq d _ { e }$ . If this inequality holds with equality for each edge $e \in E _ { n }$ , then $y$ is said to be an exact cut packing. The reason for the name “cut packing” is that equivalently, we can think of y as assigning value $y ( S ) + y ( { \overline { { S } } } )$ to each cut (S, S).

As shown below, in general, there may not be an exact cut packing for metric $( V , d )$ . Let us relax this notion by allowing edges to be underpacked up to a specified extent. For $\beta \geq 1 .$ , y is said to be a β-approximate cut packing if the amount of cut felt by any edge is at least $1 / \beta$ fraction of its length, i.e., for each edge $\begin{array} { r } { e \in E _ { n } , d _ { e } / \beta \le \sum _ { S : e \in \delta ( S ) } y ( S ) \le d _ { e } } \end{array}$ . Clearly, the smaller $\beta$ is, the better the cut packing. The following theorem shows the importance of finding a good cut packing for $( V , d )$

Theorem 21.7 Let $( V , d )$ be the metric obtained in Theorem $\it { 2 1 . 4 } ,$ and let y be a β-approximate cut packing for $( V , d )$ . Among cuts with $y ( S ) \neq 0 ,$ , let $( S ^ { \prime } , { \overline { { S ^ { \prime } } } } )$ be the sparsest. Then, the sparsity of this cut is at most $\beta \cdot f ^ { * }$

Proof: Let y be a $\beta \mathrm { . }$ -approximate cut packing for metric $( V , d )$ . Then,

$$
\begin{array}{l} f ^ {*} = \frac {\sum_ {e \in G} c _ {e} d _ {e}}{\sum_ {e \in H} \mathrm{dem} (e) d _ {e}} \geq \frac {\sum_ {e \in G} c _ {e} \sum_ {S : e \in \delta (S)} y (S)}{\sum_ {e \in H} \mathrm{dem} (e) \sum_ {S : e \in \delta (S)} \beta y (S)} \\ = \frac {\sum_ {S} y (S) c (S)}{\beta \sum_ {S} y (S) \mathrm{dem} (S)} \\ \geq \frac {1}{\beta} \cdot \left(\frac {c (S ^ {\prime})}{\mathrm{dem} (S ^ {\prime})}\right). \end{array}
$$

The first inequality follows using both the upper bound and the lower bound on the amount of cut felt by an edge; the former in the numerator and the latter in the denominator. The equality after that follows by changing the order of summation. The last inequality follows from the well known result stated below. ✷

Proposition 21.8 For any nonnegative reals $a _ { 1 } , \ldots , a _ { n }$ and positive reals $b _ { 1 } , \ldots , b _ { n }$ and $\alpha _ { 1 } , \ldots , \alpha _ { n }$

$$
\frac {\sum_ {i} \alpha_ {i} a _ {i}}{\sum_ {i} \alpha_ {i} b _ {i}} \geq \min _ {i} \frac {a _ {i}}{b _ {i}}.
$$

Moreover, this inequality holds with equality $i f f$ the n values $a _ { i } / b _ { i }$ are all equal.

Corollary 21.9 If there is an exact cut packing for metric $( V , d )$ , then every cut (S, S) with $y _ { S } \neq 0$ has sparsity $f ^ { * }$ and thus is a sparsest cut in $G$

Proof: $\mathrm { B y }$ Theorem 21.7, the minimum sparsity cut with $y _ { S } \neq 0$ has sparsity at most $f ^ { * } \ ( { \mathrm { s i n c e } } \ \beta = 1 )$ . Since the sparsity of any cut upper bounds $f ^ { * }$ , the sparsity of this cut equals $f ^ { * }$ , and this is a sparsest cut in $G .$ . But then al inequalities in the proof of Theorem 21.7 must hold with equality. Now, by the second statement in Proposition 21.8, we get that every cut $( S , { \overline { { S } } } )$ with $y _ { S } \neq 0$ has sparsity $f ^ { * }$ ✷

The sparsest cut in the instance specified in Example 21.3 has sparsity strictly larger than $f ^ { * } . \mathrm { { B y } }$ Corollary 21.9, the optimal metric for this instance does not have an exact cut packing. However, it turns out that every metric has an O(log n)-approximate cut packing – we will show this using the notion of $\ell _ { 1 }$ -embeddability of metrics.

## 21.3.2 <sub>1</sub>-embeddability of metrics

A norm on the vector space $\mathbf { R } ^ { m }$ is a function $\| \cdot \| : \mathbf { R } ^ { m }  \mathbf { R } ^ { + }$ , such that for any $\pmb { x } , \pmb { y } \in \mathbf { R } ^ { m }$ , and $\lambda \in \mathbf { R }$ :

$\| { \pmb x } \| = 0 \operatorname { i f f } { \pmb x } = 0 .$

$\| \lambda \pmb { x } \| = | \lambda | \cdot \| \pmb { x } \| ,$

$\| { \pmb x } + { \pmb y } \| \leq \| { \pmb x } \| + \| { \pmb y } \| .$

For $p \geq 1$ , the $\ell _ { p } { - } n o r m$ is defined by

$$
\| \boldsymbol {x} \| _ {p} = \left(\sum_ {1 \leq k \leq m} | x _ {k} | ^ {p}\right) ^ {\frac {1}{p}}.
$$

The associated $\ell _ { p } .$ -metric, denoted by $d _ { \ell _ { p } }$ , is defined by

$$
d _ {\ell_ {p}} (\boldsymbol {x}, \boldsymbol {y}) = \| \boldsymbol {x} - \boldsymbol {y} \| _ {p}
$$

for all $\pmb { x } , \pmb { y } \in \mathbf { R } ^ { m }$ . In this section, we will only consider the $\ell _ { 1 } { \mathrm { - n o r m } }$

Let σ be a mapping, $\sigma : V  \mathbf { R } ^ { m }$ for some $m$ . Let us say that $\| \sigma ( u ) -$ $\sigma ( v ) \| _ { 1 }$ is the $\ell _ { 1 }$ length ofedge $( u , v )$ under σ. We will say that σ is an isometric  -embedding for metric $( V , d )$ if it preserves the $\ell _ { 1 }$ lengths of all edges, i.e.,

$$
\forall u, v \in V, d (u, v) = \| \sigma (u) - \sigma (v) \| _ {1}.
$$

As shown below, in general, the metric computed by solving the dual program may not be isometrically $\ell _ { 1 } { \mathrm { - e m b e d d a b l e } } .$ . Thus, we will relax this notion – we will ensure that the mapping does not stretch any edge, but we will allow it to shrink edges up to a specified factor. For $\beta \geq 1$ , we will say that $\sigma$ is a β-distortion $\ell _ { 1 } { - } e m b e d d i n g$ for metric $( V , d )$ if

$$
\forall u, v \in V: \frac {1}{\beta} d (u, v) \leq \| \sigma (u) - \sigma (v) \| _ {1} \leq d (u, v).
$$

Next, we show that the question of finding an approximate cut packing for a metric is intimately related to that of finding a low distortion $\ell _ { 1 }$ embedding for it.

Lemma 21.10 Let $\sigma : V  \mathbf { R } ^ { m }$ be a mapping. There is a cut packing $y : 2 ^ { V } \to \mathbf { R } ^ { + }$ such that each edge feels as much cut under y as its $\ell _ { 1 }$ length under σ. Moreover, the number of nonzero $y _ { S } \ ' _ { s }$ is at most m $( n - 1 )$ .

Proof: First consider the case when $m = 1$ . Let the n vertices of $V$ be mapped to $u _ { 1 } \leq u _ { 2 } \leq \cdot \cdot \cdot \leq u _ { n }$ . Assume w.l.o.g. that the vertices are also numbered in this order. For each $i , 1 \leq i \leq n - 1$ , let $y _ { \{ v _ { 1 } , \ldots , v _ { i } \} } = u _ { i + 1 } - u _ { i }$ Clearly, this cut packing satisfies the required condition.

For arbitrary m, we observe that since the <sub>1</sub>-norm is additive, we can define a cut packing for each dimension independently, and the sum of these packings satisfies the required condition. ✷

Lemma 21.11 Let $y : 2 ^ { V } \to \mathbf { R } ^ { + }$ be a cut packing with m nonzero $y _ { S } \ ' _ { s }$ There is a mapping $\sigma : V  \mathbf { R } ^ { m }$ such that for each edge, its $\ell _ { 1 }$ length under σ is the same as the amount of cut it feels under $y .$ .

Proof: We will have a dimension corresponding to each set $S \subseteq V$ such that $y _ { S } \neq 0$ . For vertices in S, this coordinate will be 0, and for vertices in ${ \overline { { S } } } _ { ; }$ this coordinate will be $y _ { S }$ . Thus, this dimension contributes exactly as much to the $\ell _ { 1 }$ length of an edge as the amount of cut felt by this edge due to $y _ { S } .$ Hence this mapping satisfies the required condition. ✷

Lemmas 21.10 and 21.11 give:

Theorem 21.12 There exists a β-distortion  -embedding for metric $( V , d )$ $i f f$ there exists a $\beta .$ -approximate cut packing for it. Moreover, the number of nonzero cuts and the dimension of the $\ell _ { 1 }$ -embedding are polynomially related.

Corollary 21.13 Metric $( V , d )$ is isometrically $\ell _ { 1 }$ -embeddable if there exists an exact cut packing for it.

We have already shown that the metric obtained for the instance in Example 21.3 does not have an exact cut packing. Therefore, it is not isometrically <sub>1</sub>-embeddable. However, we will show that any metric has an $O ( \log n )$ -distortion $\ell _ { 1 }$ -embedding; this fact lies at the heart of the approximation algorithm for the sparsest cut problem.

## 21.4 Low distortion $\ell _ { 1 }$ -embeddings for metrics

First consider the following one-dimensional embedding for metric $( V , d ) ;$ pick a set $S \subseteq V$ , and define the coordinate of vertex v to be $\begin{array} { r } { \sigma ( v ) = \operatorname* { m i n } _ { s \in S } d ( s , v ) } \end{array}$ ， i.e., the length of the shortest edge from v to S. This mapping does not stretch any edge:

Lemma 21.14 For the one-dimensional embedding given above,

$$
\forall u, v \in V, | \sigma (u) - \sigma (v) | \leq d (u, v).
$$

Proof: Let $s _ { 1 }$ and $s _ { 2 }$ be the closest vertices of $S$ to u and v, respectively. Assume w.l.o.g. that $d ( s _ { 1 } , u ) \leq d ( s _ { 2 } , v )$ . Then, $| \sigma ( u ) - \sigma ( v ) | = d ( s _ { 2 } , v ) -$ $d ( s _ { 1 } , u ) \leq d ( s _ { 1 } , v ) - d ( s _ { 1 } , u ) \leq d ( u , v )$ . The last inequality follows by the triangle inequality. ✷

More generally, consider the following m-dimensional embedding: Pick m subsets of $V , S _ { 1 } , \ldots , S _ { m } .$ , and define the ith coordinate of vertex v to be $\begin{array} { r } { \sigma _ { i } ( v ) = \operatorname* { m i n } _ { s \in S _ { i } } d ( s , v ) / m ; } \end{array}$ notice the scaling factor of m used. The additivity of $\ell _ { 1 }$ metric, together with Lemma 21.14, imply that this mapping also does not stretch any edge.

## 21.4.1 Ensuring that a single edge is not overshrunk

The remaining task is to choose the sets in such a way that no edge shrinks by a factor of more than $O ( \log n )$ . It is natural to use randomization for picking the sets. Let us first ensure that a single edge $( u , v )$ is not overshrunk. For this purpose, define the expected contribution of set $S _ { i }$ to the $\ell _ { 1 }$ length of edge (u, v) to be ${ \bf E } [ | \sigma _ { i } ( u ) - \sigma _ { i } ( v ) | ]$

For simplicity, assume that n is a power of 2; let $n = 2 ^ { l }$ . For $2 \leq i \leq$ $l + 1$ , set $S _ { i }$ is formed by picking each vertex of V with probability $1 / 2 ^ { i }$ The embedding w.r.t. these sets works for the single edge $( u , v )$ with high probability. The proof of this fact involves cleverly taking into consideration the expected contribution of each set. For diferent metrics, diferent sets have a large contribution. In order to develop intuition for the proof, we first illustrate this through a series of examples.

Example 21.15 In the following three metrics, $d ( u , v ) = 1$ , and the n vertices are placed as shown in the figure below.

![](images/49bff575ef423ac4a0da0995eb0fca341716a3318574aacb80569a33d0ea15b1.jpg)

For each metric, the expected contribution of one of the sets is $\mathcal { Q } ( d ( u , v ) / l )$ For the first metric, this set is $S _ { l } ,$ , since it will be a singleton with constant probability. For the second metric, this set is $S _ { 2 }$ , since it will contain exactly one of u and $v$ with constant probability. For the third metric, this set is $S _ { \lceil l / 2 \rceil }$ , since with constant probability, it will contain exactly one vertex of the $2 { \sqrt { n } }$ vertices bunched up with u and $v .$ ✷

In the next lemma, we encapsulate the basic mechanism for establishing a lower bound on the expected contribution of a set $S _ { i } .$ . For any vertex x and nonnegative real $r ,$ let $B ( x , r )$ denote the ball of radius r around $x ,$ , i.e., $B ( x , r ) = \{ s \in V | d ( x , s ) \leq r \}$

Lemma 21.16 If for some choice of $r _ { 1 } \ge r _ { 2 } \ge 0$ , and constant $^ { c , }$

$$
\operatorname * {P r} \left[ \left(S _ {i} \cap B (u, r _ {1}) = \emptyset\right) a n d \left(S _ {i} \cap B (v, r _ {2}) \neq \emptyset\right) \right] \geq c,
$$

then the expected contribution of $S _ { i } { \mathrm { ~ } i s } \ge c ( r _ { 1 } - r _ { 2 } ) / l$

Proof: Under the event described, $d ( u , S _ { i } ) \geq r _ { 1 }$ and $d ( v , S _ { i } ) \leq r _ { 2 }$ . If so, $\sigma _ { i } ( u ) \geq r _ { 1 } / l$ and $\sigma _ { i } ( v ) \leq r _ { 2 } / l$ . Therefore, $| \sigma _ { i } ( u ) - \sigma _ { i } ( v ) | \geq ( r _ { 1 } - r _ { 2 } ) / l .$ , and the lemma follows. ✷

The remaining task is to define suitable radii $r _ { 1 }$ and $r _ { 2 }$ for each set $S _ { i }$ such that the probabilistic statement of Lemma 21.16 holds. We will need the following simple probabilistic fact:

Lemma 21.17 For $1 \leq t \leq l - 1$ , let A and B be disjoint subsets of $V$ such that $| A | < 2 ^ { t }$ and $| B | \geq 2 ^ { t - 1 }$ . Form set S by picking each vertex of V independently with probability $p = 1 / ( 2 ^ { t + 1 } )$ . Then,

$$
\mathbf {P r} [ (S \cap A = \emptyset) a n d (S \cap B \neq \emptyset) ] \geq (1 / 2) (1 - e ^ {- 1 / 4}).
$$

Proof:

$$
\mathbf {P r} [ S \cap A = \emptyset ] = (1 - p) ^ {| A |} \geq (1 - p | A |) \geq \frac {1}{2},
$$

where the first inequality follows by taking the first two terms of the binomia expansion.

$$
\mathbf {P r} [ S \cap B = \emptyset ] = (1 - p) ^ {| B |} \leq e ^ {- p | B |} \leq e ^ {- 1 / 4},
$$

where we have used the inequality $1 - x \leq e ^ { - x }$ . Therefore,

$$
\mathbf {P r} [ S \cap B \neq \emptyset ] = 1 - (1 - p) ^ {| B |} \geq 1 - e ^ {- 1 / 4}.
$$

Finally, observe that since A and $B$ are disjoint, the two events $[ S \cap A = \emptyset ]$ and $[ S \cap B \neq \emptyset ]$ are independent. The lemma follows. ✷

For convenience, let $c = ( 1 / 2 ) ( 1 - e ^ { - 1 / 4 } )$

For $0 \leq t \leq l ,$ define $\rho _ { t } = \operatorname* { m i n } \{ \rho \geq 0 : | B ( u , \rho ) | \geq 2 ^ { t }$ and $| B ( v , \rho ) | \geq$ $2 ^ { t } \} , { \mathrm { i . e . , ~ } } \rho _ { t }$ is the smallest radius such that the ball around u and the ball around v each has at least $2 ^ { t }$ vertices. Clearly, $\rho _ { 0 } = 0$ and $\rho _ { l } \geq d ( u , v )$ . Let $\hat { t } = \operatorname* { m a x } \{ t : \ \rho _ { t } < d ( u , v ) / 2 \}$ ; clearly, $\hat { t } \leq l - 1$ . Finally, for any vertex x and nonnegative real $r ,$ let $B ^ { \circ } ( x , r )$ denote the open ball of radius r around $x ,$ $\mathrm { i . e . , } B ^ { \circ } ( x , r ) = \{ s \in V | d ( x , s ) < r \}$

Lemma 21.18 For $1 \leq t \leq { \hat { t } }$ , the expected contribution of $S _ { t + 1 }$ is at most $c \cdot { \frac { \rho _ { t } - \rho _ { t - 1 } } { I } }$ , and for $t = \hat { t } + 1$ , the expected contribution of $S _ { t + 1 }$ is at most $\begin{array} { r } { \frac { c } { l } \cdot \left( \frac { d \left( u , v \right) } { 2 } - \rho _ { t - 1 } \right) } \end{array}$

Proof: First consider t such that $1 \leq t \leq { \hat { t } } .$ By the definition of $\rho _ { t }$ , for at least one of the two vertices u and $v ,$ the open ball of radius $\rho _ { t }$ contains fewer than $2 ^ { t }$ vertices. Assume w.l.o.g. that this happens for vertex $u ,$ i. $. \mathrm { e . , }$ $| B ^ { \circ } ( u , \rho _ { t } ) | < 2 ^ { t }$ . Again, by definition, $| B ( v , \rho _ { t - 1 } ) | \geq 2 ^ { { \bar { t } } - 1 }$ . Since $\rho _ { t - 1 } < \rho _ { t } <$ $d ( u , v ) / 2$ , the two sets $B ^ { \circ } ( u , \rho _ { t } )$ and $B ( v , \rho _ { t - 1 } )$ are disjoint. Thus, by Lemma $2 1 . 1 7 ,$ the probability that $S _ { t + 1 }$ is disjoint from the first set and intersects the second is least $c .$ Now, the first claim follows from Lemma 21.16.

Next, let $t ~ = ~ { \hat { t } } + 1$ . By the definition of $\hat { t } ,$ for at least one of the two vertices u and $v ,$ the open ball of radius $d ( u , v ) / 2$ contains fewer than $2 ^ { t }$ vertices. As before, w.l.o.g. assume this happens for vertex $u ,$ i.e., $| B ^ { \circ } ( u , d ( u , v ) / 2 ) | < 2 ^ { t }$ . Clearly, $| B ( v , \rho _ { t - 1 } ) | \geq 2 ^ { t - 1 }$ . Since $\rho _ { t - 1 } < d ( u , v ) / 2 .$ the two sets $B ^ { \circ } ( u , d ( u , v ) / 2 )$ and $B ( v , \rho _ { t - 1 } )$ are disjoint. The rest of the reasoning is the same as before. ✷

Lemma 21.19 The expected contribution of all sets $S _ { 2 } , \ldots , S _ { l + 1 }$ is at most ${ \frac { c } { 2 } } \cdot { \frac { d ( u , v ) } { l } }$

Proof: By Lemma 21.18, the expected contribution of all sets $S _ { 2 } , \ldots , S _ { l + 1 }$ is at least the following telescoping sum:

$$
\frac {c}{l} \cdot \left(\left(\rho_ {1} - \rho_ {0}\right) + \left(\rho_ {2} - \rho_ {1}\right) + \dots + \left(\frac {d (u , v)}{2} - \rho_ {\hat {t}}\right)\right) = \frac {c}{2} \cdot \frac {d (u , v)}{l}.
$$

Lemma 21.20 Pr contribution of all sets is $\geq \frac { c d ( u , v ) } { 4 l } \bigg ] \geq \frac { c / 2 } { 2 - c / 2 } .$

Proof: Denote the probability in question by $p .$ . Clearly, the total contribution of all sets $S _ { 2 } , \ldots , S _ { l + 1 }$ to the $\ell _ { 1 }$ length of edge $( u , v )$ is at most $d ( u , v ) / 2 l$ This fact and Lemma 21.19 give:

$$
p \cdot \frac {d (u , v)}{l} + (1 - p) \cdot \frac {c d (u , v)}{4 l} \geq \frac {d (u , v)}{l}.
$$

Therefore, $\begin{array} { r } { p \ge \frac { c / 2 } { 2 - c / 2 } } \end{array}$

## 21.4.2 Ensuring that no edge is overshrunk

The above embedding does not overshrink edge $( u , v )$ with constant probability. In order to ensure that no edge is overshrunk, we will first enhance this probability. The key idea is to repeat the entire process several times independently and use Chernof bounds to bound the error probability. We will use the following statement of the Chernof bound: Let ${ \bar { X } } _ { 1 } , \ldots , { \bar { X _ { n } } }$ be independent Bernoulli trials with $\mathbf { P r } [ X _ { i } = 1 ] = p , \ 0 < p < 1$ , and let $\begin{array} { r } { X = \sum _ { i = 1 } ^ { n } X _ { i } ; } \end{array}$ clearly, $\mathbf { E } [ X ] = n p$ . Then, for $0 < \delta \leq 1$ ，

$$
\mathbf {P r} [ X <   (1 - \delta) n p ] <   \exp (- \delta^ {2} n p / 2).
$$

Pick sets $S _ { 2 } , \ldots , S _ { l + 1 }$ using probabilities specified above, independently $N = O ( \log n )$ times each. Call the sets so obtained $S _ { i } ^ { j } , 2 \leq i \leq l + 1 , 1 \leq$ $j \le N$ . Consider the $N \cdot l = O ( \log ^ { 2 } n )$ dimensional embedding of metric $( V , d )$ w.r.t. these $N \cdot l$ sets. We will prove that this is an $O ( \log n )$ -distortion $\ell _ { 1 }$ -embedding for metric $( V , d )$

Lemma 21.21 For $N = O ( \log n )$ , this embedding satisfies:

$$
\mathbf {P r} [ \| \sigma (u) - \sigma (v) \| _ {1} ] \geq \frac {p c d (u , v)}{4 l} ] \geq 1 - \frac {1}{2 n ^ {2}},
$$

where $\boldsymbol { p } = \boldsymbol { c } / ( 2 - { c } )$

Proof: We will think of the process of picking sets $S _ { 2 } , \ldots , S _ { l + 1 }$ once as a single Bernoulli trial; thus, we have N such trials. A trial succeeds if the contribution of all its sets is $\geq ( c d ( u , v ) ) / 2 l$ . By Lemma 21.20, the probability of success is at least $p .$ Using the Chernof bound with $\delta = 1 / 2$ , the probability that at most $N p / 2$ of these trials succeed is at most exp $( N p / 8 )$ . Clearly, this is bounded by $1 / 2 n ^ { 2 }$ for $N = O ( \log n )$ . If at least $N p / 2$ trials succeed, the $\ell _ { 1 }$ length of edge $( u , v )$ will be at least $p c d ( u , v ) / 4 l = d ( u , v ) / O ( \log n )$ . The lemma follows. ✷

Adding the error probabilities for all $n ( n - 1 ) / 2$ edges, we $\mathrm { g e t } { \cdot }$ :

Theorem 21.22 The $N l = O ( \log ^ { 2 } n )$ dimensional embedding given above is an O(log n)-distortion $\ell _ { 1 } { - } e m b e d d i n g$ for metric $( V , d )$ , with probability at least $1 / 2$

## 21.5 LP-rounding-based algorithm

The reader can verify that Claim 21.6 and Theorems 21.7, 21.12, and 21.22 lead to an $O ( \log n )$ factor approximation algorithm for the sparsest cut problem. In this section, we will improve the approximation guarantee to $O ( \log k )$ where $k$ is the number of source–sink pairs specified.

For this purpose, notice that Theorem 21.7 holds even for the following less stringent approximate cut packing: no edge is allowed to be overpacked, and the edges of the demand graph are not under-packed by more than a $\beta$ factor (the rest of the edges are allowed to be under-packed to any extent). In turn, such a cut packing can be obtained from an $\ell _ { 1 } { \mathrm { - e m b e d d i n g } }$ that does not overshrink edges of the demand graph only. Since these are only $O ( k ^ { 2 } )$ in number, where $k$ is the number of source–sink pairs, we can ensure that these edges are not shrunk by a factor of more than $O ( \log k )$ , thus enabling an improvement in the approximation guarantee.

Let $V ^ { \prime } \subseteq V$ be the set of vertices that are sources or sinks, $| V ^ { \prime } | \leq 2 k$ . For simplicity, assume $\vert V ^ { \prime } \vert$ is a power of 2; let $| V ^ { \prime } | = 2 ^ { l }$ . The sets $S _ { 2 } , \ldots , S _ { l + 1 }$ will be picked from $V ^ { \prime }$ , and it is easy to verify from the proof of Lemma 21.21 that $N = O ( \log k )$ will sufice to ensure that none of the $O ( k ^ { 2 } )$ edges of the demand graph is shrunk by more than a factor of $O ( \log k )$ . The complete algorithm is:

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 21.23 (Sparsest cut)
1. Solve the dual LP (21.2) to obtain metric  $(V, \mathbf{d})$ .
2. Pick sets  $S_{i}^{j}$ ,  $2 \leq i \leq l + 1$ ,  $1 \leq j \leq N$ , where set  $S_{i}^{j}$  is formed by picking each vertex of  $V'$  independently with probability  $1/2^{i}$ .
3. Obtain an  $\ell_{1}$ -embedding of  $(V, \mathbf{d})$  in  $O(\log^{2} k)$ -dimensional space w.r.t. these sets.
4. Obtain an approximate cut packing for  $(V, \mathbf{d})$  from the  $\ell_{1}$ -embedding.
5. Output the sparsest cut used by the cut packing.
</div>

Theorem 21.24 Algorithm 21.23 achieves an approximation guarantee of $O ( \log k )$ for the sparsest cut problem.

Corollary 21.25 For a demands multicommodity flow instance with $k$ source– sink pairs,

$$
\frac {1}{O (\log k)} \left(\min _ {S \subset V} \frac {c (S)}{\operatorname * {d e m} (S)}\right) \leq \max _ {\textit {t h r o u g h p u t f}} f \leq \min _ {S \subset V} \frac {c (S)}{\operatorname * {d e m} (S)}.
$$

## 21.6 Applications

We present below a number of applications of the sparsest cut problem.

## 21.6.1 Edge expansion

Expander graphs have numerous applications; for instance, see Example 20.9. We will obtain an O(log n) factor algorithm for the problem of determining the edge expansion of a graph:

Problem 21.26 (Edge expansion) Given an undirected graph $G = ( V , E )$ the edge expansion of a set $S \subset V$ with $| S | \le n / 2$ , is defined to be $| \delta ( S ) |$ i.e., the number of edges in the cut (S, S). The problem is to find a minimum expansion set.

Consider the special case of demands multicommodity flow in which we have $n ( n - 1 ) / 2$ distinct commodities, one for each pair of vertices. This is called the uniform multicommodity flow problem. For this problem, the sparsity of any cut (S, S) is given by

$$
\frac {c (S)}{| S | \cdot | \overline {{S}} |}.
$$

Let $( S , { \overline { { S } } } )$ , with $| S | \le | \overline { { S } } |$ , be the cut found by Algorithm 21.23 when run on G with uniform demands. Notice that |S| is known within a factor of $^ { 2 , }$ since $n / 2 \le | \overline { { S } } | \le n$ . Thus, S has expansion within an $O ( \log n )$ factor of the minimum expansion set in G. Clearly, the generalization of this problem to arbitrary edge costs also has an $O ( \log n )$ factor approximation algorithm.

## 21.6.2 Conductance

The conductance of a Markov chain characterizes its mixing rate, i.e., the number of steps needed to ensure that the probability distribution over states is suficiently close to its stationary distribution. Let P be the transition matrix of a discrete-time Markov chain on a finite state space X, and let π denote the stationary probability distribution of this chain. We will assume that the chain is aperiodic, connected, and that it satisfies the detailed balance condition, i.e.,

$$
\pi (x) P (x, y) = \pi (y) P (y, x) \forall x, y \in X.
$$

Define undirected graph $G = ( X , E )$ on vertex set X such that $( x , y ) \in E$ if $\pi ( x ) P ( x , y ) \neq 0$ . The edge weights are defined to be $w ( x , y ) = \pi ( x ) P ( x , y )$ The conductance of this chain is given by

$$
\varPhi = \min _ {S \subset X, 0 <   \pi (S) \leq 1 / 2} \frac {w (S , \overline {{S}})}{\pi (S)},
$$

where $w ( S , { \overline { { S } } } )$ is the sum of weights of all edges in the cut $( S , { \overline { { S } } } )$ . For any set $S _ { \mathrm { { ; } } }$ the numerator of the quotient defined above is the probability that the chain in equilibrium escapes from set $S$ to S in one step. Thus the quotient gives the conditional probability of escape, given that the chain is initially in $S$ and Φ measures the ability of the chain to not get trapped in any small region of the state space.

Theorem 21.24 leads to an $O ( \log n )$ factor approximation algorithm for computing conductance. First, observe that it sufices to approximate the following symmetrized variant of Φ:

$$
\varPhi^ {\prime} = \min _ {S \subset X, 0 <   \pi (S) \leq 1} \frac {w (S , \overline {{S}})}{\pi (S) \pi (\overline {{S}})},\tag{21.3}
$$

since Φ and $\varPhi ^ { \prime }$ are within a factor of 2 of each other (notice that if $0 < \pi ( S ) \leq$ $1 / 2 .$ , then $1 / 2 \le \pi ( \overline { { S } } ) < 1 )$

Next, let us show that computing $\varPhi ^ { \prime }$ is really a special case of the sparsest cut problem. Consider graph $G = ( X , E )$ ) with edge weights as defined above. For each pair of vertices $x , y \in X$ , define a distinct commodity with a demand of $\pi ( x ) \pi ( y )$ . It is easy to see that the sparsity of a cut (S, S) for this instance is simply the quotient defined in (21.3). Hence, the sparsity of the sparsest cut is $\varPhi ^ { \prime }$

## 21.6.3 Balanced cut

The following problem finds applications in partitioning problems, such as circuit partitioning in VLSI design. Furthermore, it can be used to perform the “divide” step of the divide-and-conquer algorithms for certain problems; for instance, see the algorithm for Problem 21.29 below.

Problem 21.27 (Minimum b-balanced cut) Given an undirected graph $G = ( V , E )$ with nonnegative edge costs and a rational $b , 0 < b \leq 1 / 2$ , find a minimum capacity cut (S, S) such that $b \cdot n \leq | S | < ( 1 - b ) \cdot n$

A b-balanced cut for $b = 1 / 2$ is called a bisection $c u t ,$ and the problem of finding a minimum capacity such cut is called the minimum bisection problem. We will use Theorem 21.24 to obtain a pseudo-approximation algorithm for Problem 21.27 – we will find a $( 1 / 3 )$ -balanced cut whose capacity is within an $O ( \log n )$ factor of the capacity of a minimum bisection cut (see the notes in Section 21.8 for a true approximation algorithm).

For $V ^ { \prime } ~ \subset ~ V$ , let $G _ { V ^ { \prime } }$ denote the subgraph of G induced by $V ^ { \prime }$ . The algorithm is: Initialize $U  \emptyset$ and $V ^ { \prime }  V$ . Until $| U | \ge n / 3$ , find a minimum expansion set in $G _ { V ^ { \prime } } ,$ say W, then set $U  U \cup W$ and $V ^ { \prime }  V ^ { \prime } - W$ . Finally, let $S  U$ , and output the cut $( S , V - S )$

Claim 21.28 The cut output by the algorithm is a $( 1 / 3 )$ -balanced cut whose capacity is within an O(log n) factor of the capacity of a minimum bisection cut in G.

Proof: At the end of the penultimate iteration, $| U | < n / 3$ . Thus, at the beginning of the last iteration, $| V ^ { \prime } | \ge 2 n / 3$ . At most half of these vertices are added to U in the last iteration. Therefore, $| V - S | \ge n / 3$ and $n / 3 \le | S | <$ $n / 3$ . Hence, $( S , V - S )$ is a $( 1 / 3 )$ -balanced cut.

Let $( T , { \overline { { T } } } )$ be a minimum bisection cut in $G .$ Since at the beginning of each iteration, $| V ^ { \prime } | ~ \geq ~ 2 n / 3$ , each of the sets $T \cap V ^ { \prime }$ and $\overline { { T } } \cap \bar { V } ^ { \prime }$ has at least $n / 6$ vertices. Thus, the expansion of a minimum expansion set in $G _ { V ^ { \prime } }$ in each iteration is at most $\frac { c ( \bar { T } ) } { ( n / 6 ) }$ . Since the algorithm finds a set having expansion within a factor of $O ( \log n )$ of optimal in any iteration, the set $U$ found satisfies:

$$
\frac {c (U)}{| U |} \leq O (\log n) \cdot \frac {c (T)}{n / 6}.
$$

Since the final set $S$ has at most $2 n / 3$ vertices, summing up we get

$$
c (S) \leq O (\log n) \cdot \frac {c (T) (2 n / 3)}{n / 6},
$$

thereby giving $c ( S ) \leq O ( \log n ) \cdot c ( T )$ ).

## 21.6.4 Minimum cut linear arrangement

Problem 21.29 (Minimum cut linear arrangement) Given an undirected graph $G = ( V , E )$ with nonnegative edge costs, for a numbering of its vertices from 1 to $n ,$ define $S _ { i }$ to be the set of vertices numbered at most $i ,$ for $1 \leq i \leq n - 1 ;$ this defines $n - 1$ cuts. The problem is to find a numbering that minimizes the capacity of the largest of these $n - 1$ cuts, i.e., it minimizes max $\{ c ( S _ { i } ) | 1 \le i \le ( n - 1 ) \}$ }.

Using the pseudo-approximation algorithm obtained above for the $( 1 / 3 ) .$ balanced cut problem, we will obtain a true $O ( \log ^ { 2 } n )$ factor approximation algorithm for this problem. A key observation is that in any arrangement, $S _ { n / 2 }$ is a bisection cut, and thus the capacity of a minimum bisection cut in $G ,$ say $\beta ,$ is a lower bound on the optimal arrangement. The reason we get a true approximation algorithm is that the $( 1 / 3 )$ -balanced cut algorithm compares the cut found to $\beta$

The algorithm is recursive: find $\mathrm { ~ a ~ } ( 1 / 3 )$ -balanced cut in $G _ { V }$ , say $( S , { \overline { { S } } } )$ and recursively find a numbering of $S$ in $G _ { S }$ using numbers from 1 to $| S |$ and a numbering of $\overline { S }$ in $G _ { \overline { { S } } }$ using numbers from $| S | + 1$ to n. Of course, the recursion ends when the set is a singleton, in which case the prescribed number is assigned to this vertex.

Claim 21.30 The algorithm given above achieves an $O ( \log ^ { 2 } n )$ factor for the minimum cut linear arrangement problem.

Proof: The following binary tree $T$ (not necessarily complete) encodes the outcomes of the recursive calls made by the algorithm: Each recursive call corresponds to a node of the tree. Suppose recursive call α ends with two further calls, $\alpha _ { 1 }$ and $\alpha _ { 2 }$ , where the first call assigns smaller numbers and the second assigns larger numbers. Then, $\alpha _ { 1 }$ will be made the left child of $\alpha$ in $T$ and $\alpha _ { 2 }$ will be made the right child of α. If recursive call α was made with a singleton, then α will be a leaf of the tree.

To each nonleaf, we will assign the set of edges in the cut found during this call, and to each leaf we will assign its singleton vertex. Thus, the left to right ordering of leaves gives the numbering assigned by the algorithm to the vertices. Furthermore, the edge sets associated with nonleaf nodes define a partitioning of all edges of G. The cost of edges associated with any nonleaf is $O ( \log n ) \beta$ by Claim 21.28. Since each recursive call finds a $( 1 / 3 )$ -balanced cut, the depth of recursion, and hence the depth of $T _ { \mathrm { : } }$ , is $O ( \log n )$

Following is a crucial observation: Consider any edge $( u , v )$ in G. Let α be the lowest common ancestor of leaves corresponding to u and v in $T$ . Then, $( u , v )$ belongs to the set of edges associated with node α.

With respect to the numbering found by the algorithm, consider a cut $( S _ { i } , \overline { { S _ { i } } } )$ , $1 \leq i \leq n - 1$ . Any edge in this cut connects vertices numbered $j$ and k with $j \le i$ and $k \geq i + 1$ . Thus, such an edge must be associated with a node that is a common ancestor of the leaves numbered i and $i + 1$ . Since the depth of $T$ is $O ( \log n )$ , there are $O ( \log n )$ such common ancestors. Since the cost of edges associated with any node in $T$ is $O ( \log n ) \beta$ , the cost of cut $( S _ { i } , \overline { { S _ { i } } } )$ is bounded by $O ( \log ^ { 2 } n ) \beta$ . The claim follows since we have already argued that $\beta$ is a lower bound on the optimal arrangement. ✷

## 21.7 Exercises

21.1 For each of the three metrics given in Example 21.15, one of the sets $S _ { 2 } , \ldots , S _ { l + 1 }$ has an expected contribution of $\mathcal { Q } ( d ( u , v ) / l )$ . Give a metric for which each set has an expected contribution of $\Theta ( d ( u , v ) / l ^ { 2 } )$ .

21.2 Show that n points embedded in $\ell _ { 1 }$ space can be an isometric embedding in (a higher dimensional) $\ell _ { 2 } ^ { 2 }$ space.

Hint: Since $\ell _ { 1 }$ and $\ell _ { 2 } ^ { 2 }$ are both additive across dimensions, first show that it is suficient to consider n points in one dimension. Sort these points, and renumber, say $x _ { 1 } , \ldots , x _ { n }$ . Now embed these in $( \mathbf { R } ^ { n - 1 } , \ell _ { 2 } ^ { 2 } )$ as follows. Let $\alpha _ { i } = x _ { i + 1 } - x _ { i }$ . Map point $x _ { i }$ to $( \sqrt { \alpha _ { 1 } } , \ldots , \sqrt { \alpha _ { i - 1 } } , 0 , \ldots , 0 )$

21.3 Why can’t the pseudo-approximation algorithm given at the beginning of Section 21.6.3 be converted to a true approximation algorithm, $\mathrm { i . e . }$ , so that in the end, we compare the $( 1 / 3 )$ -balanced cut found to the optimal $( 1 / 3 )$ -balanced cut?

Hint: Construct graphs for which the capacity of a minimum bisection cut is arbitrarily higher than that of a $( 1 / 3 )$ -balanced cut.

21.4 Show that the above algorithm extends to finding a b-balanced cut that is within an $O ( \log n )$ factor of the best b<sup></sup>-balanced cut for $b \leq 1 / 3$ and $b < b ^ { \prime }$ . Where in the argument is the restriction $b \leq 1 / 3$ used?

21.5 Give an approximation factor preserving reduction from the problem of finding a minimum b-balanced cut, for $b < 1 / 2$ , to the minimum bisection problem.

21.6 (Linial, London and Rabinovich [190]) Extend Theorem 21.22 to show that for any $p \geq 1$ , there is an $O ( \log n )$ distortion $\ell _ { p } .$ -embedding for metric $( V , d )$ in $O ( \log ^ { 2 } n )$ -dimensional space.

Hint: Map point v to $\frac { d ( v , S _ { i } ) } { Q ^ { 1 / p } }$ , for $i = 1 , \ldots , Q$ , where Q is the dimension of the embedding. Use the fact that $| d ( u , S _ { i } ) - d ( v , S _ { i } ) | \leq d ( u , v )$ and the monotonicity of $\ell _ { p } { \mathrm { - n o r m } }$

## 21.7 (Feige [79]) Consider the following algorithm for:

Problem 21.31 (Bandwidth minimization) Given an undirected graph $G = ( V , E )$ , number the vertices with distinct integers from 1 to n so that the spread of the longest edge is minimized, where the spread of edge $( u , v )$ is the absolute value of the diference of the numbers assigned to u and v.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 21.32 (Bandwidth minimization)
1. Define metric  $(V, \boldsymbol{d})$ , where  $d_{uv}$  is the length of the shortest path from u to v in G.
2. Obtain an  $O(\log n)$ -distortion  $\ell_{2}$ -embedding of  $(V, \boldsymbol{d})$ .
3. Pick a line  $\ell$  from a spherically symmetric distribution, and project the n points onto  $\ell$ .
4. Number the vertices from 1 to n according to their ordering on  $\ell$ .
5. Output the numbering.
</div>

## Remark 21.33 Lemma 26.7 gives an algorithm for picking .

1. Show that the expected number of pairs of vertices that are within a distance of 1 of each other on  is bounded by

$$
O (\log n \sum_ {u, v} \frac {1}{d _ {u v}}).
$$

2. Show that

$$
\sum_ {u, v} \frac {1}{d _ {u v}} = O (n \log n \cdot \mathrm{OPT}).
$$

Hint: Use the fact that in $G ,$ the number of vertices within a distance of k of a vertex v is bounded by $2 k \cdot \mathrm { O P T }$

3. Show that with high probability, the spread of the numbering output is at most $O ( { \sqrt { n \mathrm { O P T } } }$ log n), i.e., this is an $O ( { \sqrt { n } } \log n )$ factor algorithm. Hint: If the spread of the output numbering is $s ,$ then the number of pairs of vertices that are within a distance of 1 of each other on  is at least $s ^ { 2 }$

## 21.8 Notes

The seminal work of Leighton and Rao [182] gave the first approximate maxflow min-cut theorem, for the case of uniform multicommodity flow. They also gave a factor $O ( \log n )$ approximation algorithm for the associated special case of sparsest cut and a pseudo-approximation algorithm for the b-balanced cut problem. The general version of demands multicommodity flow was first considered by Klein, Agarwal, Ravi, and Rao [173]. Theorem 21.22 is due to Linial, London, and Rabinovich [190], based on a result of Bourgain [32] who showed the existence of such an embedding and gave an exponential time algorithm for finding it. The application of this theorem to the sparsest cut problem, Theorem 21.24, was independently given by Aumann and Rabani [16], and Linial, London, and Rabinovich [190].

An $O ( \log ^ { 2 } n )$ factor algorithm for the minimum bisection problem, and hence for the minimum b-balanced cut problem (see Exercise 21.5), was given by Feige and Krauthgamer [83]. The application of sparsest cut to computing conductance is due to Sinclair [241], and the application of balanced cuts to the minimum cut linear arrangement problem is due to Bhatt and Leighton [26]. See Exercise 26.9 for a semidefinite program for finding an optimal distortion $\ell _ { 2 } ^ { 2 } .$ -embedding of n points.
