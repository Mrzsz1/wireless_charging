---
title: "approximation-algorithms-ch-23-part-024"
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
pdf_path: "work/core-books/approximation-algorithms/parts/approximation-algorithms-ch-23-part-024.pdf"
raw_md: "raw/canonical/approximation-algorithms-ch-23-part-024/full.md"
---
## 23 Steiner Network

The following generalization of the Steiner forest problem to higher connectivity requirements has applications in network design and is also known as the survivable network design problem. In this chapter, we will give a factor 2 approximation algorithm for this problem by enhancing the LP-rounding technique to iterated rounding. A special case of this problem was considered in Exercise 22.10.

Problem 23.1 (Steiner network) We are given an undirected graph $G = ( V , E )$ , a cost function on edges $c : E \to \mathbf { Q } ^ { + }$ (not necessarily satisfying the triangle inequality), a connectivity requirement function r mapping unordered pairs of vertices to $\mathbf { Z } ^ { + }$ , and a function $u : E \to \mathbf { Z } ^ { + } \cup \{ \infty \}$ stating an upper bound on the number of copies of edge e we are allowed to use; if $u _ { e } = \infty$ , there is no upper bound for edge $e .$ The problem is to find a minimum cost multigraph on vertex set V that has $r ( u , v )$ edge disjoint paths for each pair of vertices $u , v \in V$ . Each copy of edge e used for constructing this graph will cost $c ( e )$

## 23.1 The LP-relaxation and half-integrality

In order to give an integer programming formulation for this problem, we will first define a cut requirement function, $f : 2 ^ { V } \to \mathbf { Z } ^ { + }$ , as we did for the metric Steiner forest problem. For every $S \subseteq V , f ( S )$ is defined to be the largest connectivity requirement separated by the cut (S, S), i.e., $f ( S ) =$ max $\{ r ( u , v ) | u \in S$ and $v \in { \overline { { S } } } \}$

$$
\begin{array}{l l} \text {minimize} & \sum_ {e \in E} c _ {e} x _ {e} \\ \text {subject to} & \sum_ {e: e \in \delta (S)} x _ {e} \geq f (S), \quad S \subseteq V \\ & x _ {e} \in \mathbf {Z} ^ {+}, \qquad \qquad e \in E \text {and} u _ {e} = \infty \\ & x _ {e} \in \{0, 1, \ldots , u _ {e} \}, \qquad e \in E \text {and} u _ {e} \neq \infty \end{array}\tag{23.1}
$$

The LP-relaxation is:

$$
\begin{array}{l l} \text {minimize} & \sum_ {e \in E} c _ {e} x _ {e} \\ \text {subject to} & \sum_ {e: e \in \delta (S)} x _ {e} \geq f (S), \quad S \subseteq V \\ & x _ {e} \geq 0, \quad e \in E \text {and} u _ {e} = \infty \\ & u _ {e} \geq x _ {e} \geq 0, \quad e \in E \text {and} u _ {e} \neq \infty \end{array}\tag{23.2}
$$

Since LP (23.2) has exponentially many constraints, we will need the ellipsoid algorithm for finding an optimal solution. Exercise 23.1 develops a polynomial-sized LP.

As shown in Chapters 14 and 19, certain NP-hard problems, such as vertex cover and node multiway cut, admit LP-relaxations having the remarkable property that they always have a half-integral optimal solution. Rounding up all halves to 1 in such a solution leads to a factor 2 approximation algorithm. Does relaxation (23.2) have this property? The following lemma shows that the answer is $^ { 6 6 } \mathrm { n o } ^ { 9 9 }$

Lemma 23.2 Consider the Petersen graph (see Section 1.2) with a connectivity requirement of 1 between each pair of vertices and with each edge of unit cost. Relaxation (23.2) does not have a half-integral optimal solution for this instance.

Proof: Consider the fractional solution $x _ { e } = 1 / 3$ for each edge e. Since the Petersen graph is 3-edge connected (in fact, it is 3-vertex connected as well), this is a feasible solution. The cost of this solution is 5. In any feasible solution, the sum of edge variables incident at any vertex must be at least 1, to allow connectivity to other vertices. Therefore, any feasible solution must have cost at least 5 (since the Petersen graph has 10 vertices). Hence, the solution given above is in fact optimal.

Any solution with $x _ { e } = 1$ for some edge e must have cost exceeding 5, since additional edges are required to connect the endpoints of e to the rest of the graph. Therefore, any half-integral solution of cost 5 would have to pick, to the extent of one half each, the edges of a Hamiltonian cycle. Since the Petersen graph has no Hamiltonian cycles, there is no half-integral optimal solution. ✷

Let us say that an extreme point solution, also called a vertex solution or a basic feasible solution, for an LP is a feasible solution that cannot be written as the convex combination of two feasible solutions. The solution $x _ { e } = 1 / 3$ • for each edge e, is not an extreme point solution. An extreme optimal solution is shown in the figure below; thick edges are picked to the extent of $1 / 2$ , thin edges to the extent of $1 / 4 .$ and the missing edge is not picked.

![](images/53ea80bc52e264d94dd11510781f28fe8434ccda85dcc1ae2d29759191391777.jpg)

The isomorphism group of the Petersen graph is edge-transitive, and there are 15 related extreme point solutions; the solution $x _ { e } = 1 / 3$ for each edge e is the average of these.

Notice that although the extreme point solution is not half-integral, it picks some edges to the extent of half. We will show below that in fact this is a property of any extreme point solution to LP (23.2). We will obtain a factor 2 algorithm by rounding up these edges and iterating. Let H be the set of edges picked by the algorithm at some point. Then, the residual requirement of cut $( S , { \overline { { S } } } )$ is $f ^ { \prime } ( S ) = f ( S ) - | \delta _ { H } ( S ) |$ |, where $\delta _ { H } ( S )$ represents the set of edges of H crossing the cut $( S , { \overline { { S } } } )$ . In general, the residual cut requirement function, $f ^ { \prime }$ , may not correspond to the cut requirement function for any set of connectivity requirements. We will need the following definitions to characterize it:

Function $f : 2 ^ { V } \to \mathbf { Z } ^ { + }$ is said to be submodular if $f ( V ) = 0$ , and for every two sets $A , B \subseteq V .$ , the following two conditions hold:

$$
\bullet f (A) + f (B) \geq f (A \cap B) + f (A \cup B)
$$

$$
\bullet f (A) + f (B) \geq f (A - B) + f (B - A).
$$

Remark 23.3 Sometimes submodularity is defined only with the first condition. We will need to work with the stronger definition given above.

Two subsets of $V , A$ and B, are said to cross if each of the sets, $A - B ,$ $B - A$ , and $A \cap B$ , is nonempty. If A and B don’t cross then either they are disjoint or one of these sets is contained in the other.

Lemma 23.4 For any graph $G$ on vertex set $V _ { ☉ }$ , the function $| \delta _ { G } ( . ) |$ is submodular.

Proof: If sets A and B do not cross, then the two conditions given in the definition of submodular functions hold trivially. Otherwise, edges having one endpoint in $A \cap B$ and the other in A ∪ B (edge e in the figure below) contribute to $\delta ( A )$ and $\delta ( B )$ but not to $\delta ( A - B )$ or $\delta ( B - A )$ . Similarly, edge $e _ { 2 }$ below does not contribute to $\delta ( A \cap B )$ or to $\delta ( A \cup B )$ . The remaining edges contribute equally to both sides of both conditions. ✷

![](images/af9a24e0372ebbbea8cac1c2d7fba958cc68bebb353fd162b1fbe5b313bd4240.jpg)

Function $f : 2 ^ { V } \to \mathbf { Z }$ is said to be weakly supermodular if $f ( V ) = 0 ;$ , and for every two sets $A , B \subseteq V$ , at least one of the following conditions holds:

$$
\begin{array}{l} \bullet f (A) + f (B) \leq f (A - B) + f (B - A) \\ \bullet f (A) + f (B) \leq f (A \cap B) + f (A \cup B). \end{array}
$$

It is easy to check that the original cut requirement function is weakly supermodular; by Lemma 23.5, so is the residual cut requirement function.

Lemma 23.5 Let H be a subgraph of G. $I f ~ f : 2 ^ { V ( G ) } \to \mathbf { Z } ^ { + }$ is a weakly supermodular function, then so is the residual cut requirement function $f ^ { \prime }$

Proof: Suppose $f ( A ) + f ( B ) \leq f ( A - B ) + f ( B - A )$ ; the proof of the other case is similar. By Lemma $2 3 . 4 , | \delta _ { H } ( A ) | + | \delta _ { H } ( B ) | \geq | \delta _ { H } ( A - B ) | + | \delta _ { H } ( B - A ) |$ Subtracting, we get $f ^ { \prime } ( A ) + f ^ { \prime } ( B ) \leq f ^ { \prime } ( A - B ) + f ^ { \prime } ( B - A )$ ✷

We can now state the central polyhedral fact needed for the factor 2 algorithm in its full generality.

Theorem 23.6 For any weakly supermodular function f, any extreme point solution, x, to $L P \ ( { \mathcal { Q } } { \mathcal { 3 } } . { \mathcal { Q } } ) $ must pick some edge to the extent of at least a half, $i . e . , x _ { e } \ge 1 / 2$ for at least one edge e.

## 23.2 The technique of iterated rounding

In this section, we will give an iterated rounding algorithm for the Steiner network problem, using Theorem 23.6.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 23.7 (Steiner network)
1. Initialization:  $H \leftarrow \emptyset$ :  $f' \leftarrow f$ .
2. While  $f' \not\equiv 0$ , do:
    Find an extreme optimal solution,
    x, to LP (23.2) with cut requirements given by  $f'$ .
    For each edge e such that  $x_{e} \geq 1/2$ , include  $\lceil x_{e} \rceil$  copies of e in H, and decrement  $u_{e}$  by this amount.
    Update  $f'$ : for  $S \subseteq V$ ,  $f'(S) \leftarrow f(S) - |\delta_{H}(S)|$ .
3. Output H.
</div>

The algorithm presented above achieves an approximation guarantee of factor 2 for an arbitrary weakly supermodular function $f .$ Establishing a polynomial running time involves showing that an extreme optimal solution to LP (23.2) can be found eficiently. We do not know how to do this for an arbitrary weakly supermodular function $f .$ However, if $f$ is the original cut requirement function for some connectivity requirements, then a polynomial time implementation follows from the existence of a polynomial time separation oracle for each iteration.

For the first iteration, a separation oracle follows from a max-flow subroutine. Given a solution ${ \mathbf { } } ^ { \mathbf { } } \mathbf { { \mathbf { x } } } ,$ construct a graph on vertex set V with capacity $x _ { e }$ for each edge e. Then, for each pair of vertices $u , v \in V$ , check if this graph admits a flow of at least $r ( u , v )$ from u to v. If not, we will get a violated cut, i.e., a cut (S, S) such that $\delta \mathbf { { z } } ( S ) < f ( S )$ ), where

$$
\delta_ {\boldsymbol {x}} (S) = \sum_ {e: e \in \delta (S)} x _ {e}.
$$

Let $f ^ { \prime }$ be the cut requirement function of a subsequent iteration. Given a solution to LP (23.2) for this function, say $\mathbf { x } ^ { \prime }$ , define x as follows: for each edge $e , x _ { e } = x _ { e } ^ { \prime } + e _ { H }$ , where $e _ { H }$ is the number of copies of edge e in $H$ . The following lemma shows that a separation oracle for the original function $f$ leads to a separation oracle for $f ^ { \prime }$ . Furthermore, this lemma also shows that there is no need to update $f ^ { \prime }$ explicitly after each iteration.

Lemma 23.8 A cut $( S , { \overline { { S } } } )$ is violated by solution $\mathbf { x } ^ { \prime }$ under cut requirement function $f ^ { \prime }$ if it is violated by solution x under cut requirement function $f .$

Proof: Notice that $\delta { \pmb x } ( S ) = \delta { \pmb x } ^ { \prime } ( S ) + | \delta _ { H } ( S ) |$ . Since $f ( S ) = f ^ { \prime } ( S ) + | \delta _ { H } ( S ) | .$ $\delta \mathbf { x } ( S ) \geq f ( S ) { \mathrm { ~ i f ~ } } \delta \mathbf { x } ^ { \prime } ( S ) \geq f ^ { \prime } ( S )$ ✷

Lemma 23.8 implies that solution $\mathbf { x } ^ { \prime }$ is feasible for the cut requirement function $f ^ { \prime }$ if solution x is feasible for $f .$ . Assuming Theorem 23.6, whose proof we will provide below, let us show that Algorithm 23.7 achieves an approximation guarantee of 2.

Theorem 23.9 Algorithm 23.7 achieves an approximation guarantee of $\mathcal { Q }$ for the Steiner network problem.

Proof: By induction on the number of iterations executed by the algorithm when run with a weakly supermodular cut requirement function $f ,$ , we will prove that the cost of the integral solution obtained is within a factor of two of the cost of the optimal fractional solution. Since the latter is a lower bound on the cost of the optimal integral solution, the claim follows.

For the base case, if f requires one iteration, the claim follows, since the algorithm rounds up only edges e with $x _ { e } \geq 1 / 2$

For the induction step, assume that x is the extreme optimal solution obtained in the first iteration. Obtain xˆ from x by zeroing out components that are strictly smaller than $1 / 2$ . By Theorem $2 3 . 6 , \hat { \pmb x } \neq 0$ . Let $H$ be the set of edges picked in the first iteration. Since H is obtained by rounding up nonzero components of xˆ and each of these components is $\geq 1 / 2$ , cost $( H ) \leq 2 { \cdot } \mathrm { c o s t } ( \hat { x } )$

Let $f ^ { \prime }$ be the residual requirement function after the first iteration and $H ^ { \prime }$ be the set of edges picked in subsequent iterations for satisfying $f ^ { \prime }$ . The key observation is that ${ \pmb x } - \hat { { \pmb x } }$ is a feasible solution for $f ^ { \prime } ,$ , and thus by the induction hypothesis, cos $\begin{array} { r } { \dot { \mathbf { \eta } } ( H ^ { \prime } ) \leq 2 \cdot \mathrm { c o s t } ( { \pmb x } - \hat { { \pmb x } } ) } \end{array}$ . Let us denote by $H + H ^ { \prime }$ the edges of H together with those of $H ^ { \prime }$ . Clearly, $H + H ^ { \prime }$ satisfies $f .$ . Now,

$$
\begin{array}{c} \operatorname{cost} (H + H ^ {\prime}) \leq \operatorname{cost} (H) + \operatorname{cost} (H ^ {\prime}) \\ \leq 2 \cdot \operatorname{cost} (\hat {\boldsymbol {x}}) + 2 \cdot \operatorname{cost} (\boldsymbol {x} - \hat {\boldsymbol {x}}) \leq 2 \cdot \operatorname{cost} (\boldsymbol {x}). \end{array}\tag{□}
$$

Corollary 23.10 The integrality gap of LP (23.2) is bounded by 2.

Notice that previous algorithms obtained using LP-rounding solved the relaxation once and did the entire rounding based on this solution. These algorithms did not exploit the full power of rounding – after part of the solution is rounded, the remaining fractional solution may not be the best solution to continue the rounding process. It may be better to assume integral values for the rounded variables and recompute fractional values for the remaining variables, as is done above. We will call this technique iterated rounding.

Example 23.11 The tight example given for the metric Steiner tree problem, Example 3.4, is also a tight example for this algorithm. Observe that after including a subset of edges of the cycle, an extreme optimal solution to the resulting problem picks the remaining edges of the cycle to the extent of one half each. The algorithm finds a solution of cost $( 2 - \varepsilon ) ( n - 1 )$ , whereas the cost of the optimal solution is $n .$ ✷

## 23.3 Characterizing extreme point solutions

From polyhedral combinatorics we know that a feasible solution for a set of linear inequalities in $\mathbf { R } ^ { m }$ is an extreme point solution if it satisfies m linearly independent inequalities with equality. Extreme solutions of LP (23.2) satisfy an additional property which leads to a proof of Theorem 23.6.

We will assume that the cut requirement function f in LP (23.2) is an arbitrary weakly supermodular function. Given a solution x to this LP, we will say that an inequality is tight if it holds with equality. If this inequality corresponds to the cut requirement of a set $S ,$ then we will say that set $S$ is tight. Let us make some simplifying assumptions. If $x _ { e } = 0$ for some edge e, this edge can be removed from the graph, and if $x _ { e } \geq 1 , \lfloor x _ { e } \rfloor$ copies of edge e can be picked and the cut requirement function be updated accordingly. We may assume without loss of generality that an extreme point solution x satisfies $0 < x _ { e } < 1$ , for each edge e in graph G. Therefore, each tight inequality corresponds to a tight set. Let the number of edges in G be m.

We will say that a collection, L, of subsets of V forms a laminar family if no two sets in this collection cross. The inequality corresponding to a set $S$ defines a vector in $\mathbf { R } ^ { m }$ : the vector has a 1 corresponding to each edge $e \in \delta _ { G } ( S )$ , and 0 otherwise. We will call this the incidence vector of set $S _ { ☉ }$ and will denote it by $\mathbf { \mathcal { A } } _ { S }$

Theorem 23.12 Corresponding to any extreme point solution to LP (23.2) there is a collection of m tight sets such that

• their incidence vectors are linearly independent, and

• collection of sets forms a laminar family.

Example 23.13 The extreme point solution for the Peterson graph assigns nonzero values to 14 of the 15 edges. By Theorem 23.12, there should be 14 tight sets whose incidence vectors are linearly independent. These are marked in figure. ✷

Fix an extreme point solution, x, to LP (23.2). Let L be a laminar family of tight sets whose incidence vectors are linearly independent. Denote by span(L) the vector space generated by the set of vectors $\{ \mathcal { A } _ { S } | S \in \mathcal { L } \}$ . Since x is an extreme point solution, the span of the collection of all tight sets is m. We will show that if span $( \mathcal { L } ) < m$ , then there is a tight set $S$ whose addition to L does not violate laminarity and also increases the span. Continuing in this manner, we will obtain m tight sets as required in Theorem 23.12.

We begin by studying properties of crossing tight sets.

Lemma 23.14 Let A and B be two crossing tight sets. Then, one of the following must hold:

$A - B$ and $B - A$ are both tight and $\mathcal { A } _ { A } + \mathcal { A } _ { B } = \mathcal { A } _ { A - B } + \mathcal { A } _ { B - A }$

$A \cup B$ and A ∩ B are both tight and $\mathcal { A } _ { A } + \mathcal { A } _ { B } = \mathcal { A } _ { A \cup B } + \mathcal { A } _ { A \cap B }$

Proof: Since f is weakly supermodular, either $f ( A ) + f ( B ) \leq f ( A - B ) +$ $f ( B - A )$ or $f ( A ) + f ( B ) \leq f ( A \cup B ) + f ( A \cap B )$ . Let us assume the former holds; the proof for the latter is similar. Since A and $B$ are tight, we have

$$
\delta_ {\boldsymbol {x}} (A) + \delta_ {\boldsymbol {x}} (B) = f (A) + f (B).
$$

Since $A - B$ and $B - A$ are not violated,

$$
\delta \pmb {x} (A - B) + \delta \pmb {x} (B - A) \geq f (A - B) + f (B - A).
$$

Therefore,

$$
\delta_ {\boldsymbol {x}} (A) + \delta_ {\boldsymbol {x}} (B) \leq \delta_ {\boldsymbol {x}} (A - B) + \delta_ {\boldsymbol {x}} (B - A).
$$

As argued in Lemma 23.4 (which established the submodularity of function $| \delta _ { G } ( . ) | )$ , edges having one endpoint in A ∪ B and the other in $A \cap B$ can contribute only to the left-hand side of this inequality. The rest of the edges must contribute equally to both sides. So, this inequality must be satisfied with equality. Furthermore, since $x _ { e } ~ > ~ 0$ for each edge $e , G$ cannot have any edge having one endpoint in $A \cup B$ and the other in $A \cap B .$ Therefore, $\mathcal { A } _ { A } + \mathcal { A } _ { B } = \mathcal { A } _ { A - B } + \mathcal { A } _ { B - A }$ ✷

For any set $S \subseteq V$ , define its crossing number to be the number of sets of $\mathcal { L }$ that S crosses.

Lemma 23.15 Let S be a set that crosses set $T \in { \mathcal { L } }$ . Then, each of the sets $S - T , T - S , S \cup T$ and $S \cap T$ has a smaller crossing number than $S$

Proof: The figure below illustrates the three ways in which a set $T ^ { \prime } \in { \mathcal { L } }$ can cross one of these four sets without crossing $T$ itself $( T ^ { \prime }$ is shown dotted). In all cases, $T ^ { \prime }$ crosses $S$ as well. In addition, $T$ crosses $S$ but not any of the four sets. ✷

![](images/b2a3e0ff2d3debcf8c1e60932768b324c70a4766a9c6d2a04021733434161b0b.jpg)

Lemma 23.16 $L e t \ S$ be a tight set such that $\mathscr { A } _ { S } \notin \mathrm { s p a n } ( \mathcal { L } )$ and $S$ crosses some set in L. Then, there is a tight set $S ^ { \prime }$ having a smaller crossing number than $S$ and such that $\mathcal { A } _ { S ^ { \prime } } \notin$ span(L).

Proof: Let $S$ cross $T \in { \mathcal { L } }$ . Suppose the first possibility established in Lemma 23.14 holds; the proof of the second possibility is similar. Then, $S - T$ and $T -$ $S$ are both tight sets and $\mathcal { A } _ { S } + \mathcal { A } _ { T } = \mathcal { A } _ { S - T } + \mathcal { A } _ { T - S }$ . This linear dependence implies that $A _ { S - T }$ and $A _ { T - S }$ cannot both be in span $( \mathcal { L } )$ , since otherwise $\mathbf { \mathcal { A } } _ { S } ~ \in$ span $( \mathcal { L } )$ . By Lemma $2 3 . 1 5 , \ S \mathrm { ~ - ~ } T$ and $T - S$ both have a smaller crossing number than S. The lemma follows. ✷

Corollary 23.17 If span $( \mathcal { L } ) \neq \mathbf { R } ^ { m }$ , then there is a tight set $S$ such that A<sub>S</sub> -∈ span(L) and ${ \mathcal { L } } \cup \{ S \}$ is a laminar family.

$\mathrm { B y }$ Corollary 23.17, if L is a maximal laminar family of tight sets with linearly independent incidence vectors, then $| { \mathcal { L } } | = m$ . This establishes Theorem 23.12.

## 23.4 A counting argument

The characterization of extreme point solutions given in Theorem 23.12 will yield Theorem 23.6 via a counting argument. Let x be an extreme point solution and $\mathcal { L }$ be the collection of tight sets established in Theorem 23.12. The number of sets in $\mathcal { L }$ equals the number of edges in $G , { \mathrm { i . e . , } } m$ . The proof is by contradiction. Suppose that for each edge $e , x _ { e } < 1 / 2$ . Then, we will show that G has more than m edges.

Since $\mathcal { L }$ is a laminar family, it can be viewed as a forest of trees if its elements are ordered by inclusion. Let us make this precise. For $S \in { \mathcal { L } } .$ , if S is not contained in any other set of $\mathcal { L } .$ then we will say that $S$ is a root set. If $S$ is not a root set, we will say that T is the parent of S if T is a minimal set in $\mathcal { L }$ containing S; by laminarity of ${ \mathcal { L } } , T$ is unique. Further, $S$ will be called a child of $T .$ . Let the relation descendent be the reflexive transitive closure of the relation $\mathrm { ^ { 6 } c h i l d ^ { 3 } }$ . Sets that have no children will be called leaves. In this manner, $\mathcal { L }$ can be partitioned into a forest of trees, each rooted at a root set. For any set $S _ { ☉ }$ , by the subtree rooted at $S$ we mean the set of all descendents of S.

Edge e is incident at set $S$ if $e \in \delta _ { G } ( S )$ . The degree of $S$ is defined to be $| \delta _ { G } ( S ) |$ |. Set S owns endpoint v of edge $\boldsymbol { e } = \left( u , v \right)$ if S is the smallest set of L containing v. The subtree rooted at set $S$ owns endpoint v of edge $\boldsymbol { e } = ( u , v )$ if some descendent of S owns v.

Since G has m edges, it has 2m endpoints. Under the assumption that $\forall e , x _ { e } < 1 / 2 ,$ , we will prove that for any set $S ,$ the endpoints owned by the subtree rooted at $S$ can be redistributed in such a way that $S$ gets at least 3 endpoints, and each of its proper descendents gets 2 endpoints. Carrying out this procedure for each of the root sets of the forest, the total number of endpoints in the graph must exceed 2m, leading to a contradiction.

We have assumed that ∀e $: 0 < x _ { e } < 1 / 2$ . For edge $e ,$ define $y _ { e } = 1 / 2 - x _ { e }$ the halves complement of e. Clearly, $0 < y _ { e } < 1 / 2$ . For $S \in { \mathcal { L } }$ define its corequirement to be

$$
\operatorname{coreq} (S) = \sum_ {e \in \delta (S)} y _ {e} = \frac {1}{2} | \delta_ {G} (S) | - f (S).
$$

Clearly, $0 < \mathrm { c o r e q } ( S ) < | \delta _ { G } ( S ) | / 2$ . Furthermore, since $| \delta _ { G } ( S ) |$ and $f ( S )$ are both integral, core $\mathfrak { j } ( S )$ is half-integral. Let us say that $c o r e q ( S )$ is semiintegral if it is not integral, i.e., if core $\mathfrak { l } ( S ) \in \{ 1 / 2 , 3 / 2 , 5 / 2 , . . . \}$ . Since $f ( S )$ is integral, $c o r e q ( S )$ is semi-integral if $| \delta _ { G } ( S ) |$ is odd.

Sets having a corequirement of $1 / 2$ play a special role in this argument. The following lemma will be useful in establishing that certain sets have this corequirement.

Lemma 23.18 Suppose S has α children and owns $\beta$ endpoints, where $\alpha +$ $\beta = 3$ . Furthermore, each child of S, if any, has a corequirement of $1 / 2$ $T h e n , { \mathrm { c o r e q } } ( S ) = 1 / 2$

Proof: Since each child of $S$ has corequirement of $1 / 2$ , it has odd degree. Using this and the fact that $\alpha + \beta = 3 ,$ one can show that S must have odd degree (see Exercise 23.3). Therefore the corequirement of $S$ is semi-integral. Next, we show that coreq(S) is strictly smaller than $3 / 2$ , thereby proving the lemma. Clearly,

$$
\operatorname{coreq} (S) = \sum_ {e \in \delta (S)} y _ {e} \leq \sum_ {S ^ {\prime}} \operatorname{coreq} \left(S ^ {\prime}\right) + \sum_ {e} y _ {e},
$$

where the first sum is over all children $S ^ { \prime }$ of $S ,$ and the second sum is over all edges e having an endpoint in S. Since $y _ { e }$ is strictly smaller than $1 / 2$ , if $\beta > 0$ , then core ${ \mathfrak { g } } ) < 3 / 2 . { \mathrm { ~ I f ~ } } \beta = 0$ , all edges incident at the children of $S$ cannot also be incident at $S ,$ since otherwise the incidence vectors of these four sets will be linearly dependent. Therefore,

$$
\operatorname{coreq} (S) <   \sum_ {S ^ {\prime}} \operatorname{coreq} (S ^ {\prime}) = 3 / 2.
$$

The next two lemmas place lower bounds on the number of endpoints owned by certain sets.

Lemma 23.19 If set S has only one child, then it must own at least two endpoints.

Proof: Let $S ^ { \prime }$ be the child of S. If S has no endpoint incident at it, the set of edges incident at S and $S ^ { \prime }$ must be the same. But then $\mathbf { \mathcal { A } } _ { S } = \mathbf { \mathcal { A } } _ { S ^ { \prime } }$ , leading to a contradiction. $S$ cannot own exactly one endpoint, because then $\delta \mathbf { \boldsymbol { x } } ( S )$ and $\delta _ { \pmb { x } } ( S ^ { \prime } )$ will difer by a fraction, contradicting the fact that both these sets are tight and have integral requirements. The lemma follows. ✷

Lemma 23.20 If set S has two children, one of which has a corequirement of $1 / 2$ , then it must own at least one endpoint.

Proof: Let $S ^ { \prime }$ and $S ^ { \prime \prime }$ be the two children of $S ,$ with $\mathrm { c o r e q } ( S ^ { \prime } ) = 1 / 2$ Suppose S does not own any endpoints. Since the three vectors $A _ { S } , A _ { S ^ { \prime } }$ , and $A _ { S ^ { \prime \prime } }$ are linearly independent, the set of edges incident at $S ^ { \prime }$ cannot all be incident at S or all be incident at $S ^ { \prime \prime }$ . Let a denote the sum of $y _ { e }$ ’s of all edges incident at $S ^ { \prime }$ and $S _ { ; }$ , and let b denote the sum of $y _ { e } \mathrm { ' s }$ of all edges incident at $S ^ { \prime }$ and $S ^ { \prime \prime }$ . Thus, $a > 0 , b > 0$ , and $a + b = \operatorname { c o r e q } ( S ) = 1 / 2$

Since $S ^ { \prime }$ has a semi-integral corequirement, it must have odd degree. Therefore, the degrees of $S$ and $S ^ { \prime \prime }$ have diferent parities, and these two sets have diferent corequirements. Furthermore, core $\mathfrak { q } ( S ) = \mathrm { c o r e q } ( S ^ { \prime \prime } ) + a - b$ Therefore, coreq(S) − coreq $( S ^ { \prime \prime } ) = a - b$ . But $- 1 / 2 < a - b < 1 / 2$ . Therefore, $S$ and $S ^ { \prime \prime }$ must have the same corequirement, leading to a contradiction. ✷

Lemma 23.21 Consider a tree T rooted at set S. Under the assumption that $\forall e , x _ { e } < 1 / 2$ , the endpoints owned by $T$ can be redistributed in such a way that S gets at least 3 endpoints, and each of its proper descendents gets 2 endpoints. Furthermore, if $c o r e q ( S ) \neq 1 / 2$ , then S must get at least $\it 4$ endpoints.

Proof: The proof is by induction on the height of tree T. For the base case, consider a leaf set S. S must have degree at least 3, because otherwise an edge e incident at it will have $x _ { e } \geq 1 / 2$ . If it has degree exactly 3, coreq(S) is semi-integral. Further, since core $\Im ( S ) < | \delta _ { G } ( S ) | / 2 = 3 / 2$ , the corequirement of S is $1 / 2$ . Since S is a leaf, it owns an endpoint of each edge incident at it. Therefore, S has the required number of endpoints.

Let us say that a set has a surplus of 1 if 3 endpoints have been assigned to it and a surplus of 2 if 4 endpoints have been assigned to it. For the induction step, consider a nonleaf set S. We will prove that by moving the surplus of the children of S and considering the endpoints owned by S itself, we can assign the required number of endpoints to S. There are four cases:

1. If S has 4 or more children, we can assign the surplus of each child to $S _ { ; }$ thus assigning at least 4 endpoints to $S .$

2. Suppose S has 3 children. If at least one of them has a surplus of 2, or if S owns an endpoint, we can assign 4 endpoints to S. Otherwise, each child must have a corequirement of half, and by Lemma 23.18, $\mathrm { c o r e q } ( S ) = 1 / 2$ as well. Thus, assigning $S$ the surplus of its children sufices.

✷

3. Suppose $S$ has two children. If each has a surplus of $2 ,$ we can assign 4 endpoints to S. If one of them has surplus 1, then by Lemma 23.20, S must own at least one endpoint. If each child has a surplus of 1 and S owns exactly one endpoint, then we can assign 3 endpoints to S, and this sufices by Lemma 23.18. Otherwise, we can assign 4 endpoints to S.

4. If S has one child, say $S ^ { \prime }$ , then by Lemma 23.19, S owns at least 2 endpoints. If S owns exactly 2 endpoints and $S ^ { \prime }$ has surplus of exactly 1, then we can assign 3 endpoints to $S ;$ by Lemma 23.18, coreq $( S ) = 1 / 2 ,$ so this sufices. In all other cases, we can assign 4 endpoints to S.

## 23.5 Exercises

23.1 Give an LP-relaxation for the Steiner network problem, having polynomially many constraints over polynomially many variables.

Hint: Pick a minimum cost set of edges so as to route $\binom { n } { 2 }$ independent  commodities, one for each pair of vertices. Each flow should be at least as large as the connectivity requirement of this pair. The extent to which an edge is picked bounds the amount of each commodity that can flow through this edge.

23.2 Show that a function $f : 2 ^ { V } \to \mathbf { Z } ^ { + }$ satisfying the following conditions is submodular: $f ( V ) = 0 , f$ is symmetric, i.e., for any set $A \subseteq V f ( A ) =$ $f ( V { - } A )$ , and for every two sets $A , B \subseteq V f ( A ) + f ( B ) \geq f ( A \cap B ) + f ( A \cup B )$

23.3 Prove that set S in Lemma 23.18 must have odd degree. (Consider the following possibilities: S owns endpoint v of edge $( u , v )$ that is incident at S, S owns endpoint v of edge $( u , v )$ that is incident at a child of S, and an edge is incident at two children of S.)

23.4 Prove that there must be a set in L that has degree at most 3, and thus some edge must have $x _ { e } \geq 1 / 3$ . The counting argument required for this is much simpler. Notice that this fact leads to a factor 3 algorithm. (The counting argument requires the use of Lemma 23.19.)

The next two exercises develop a factor $2 H _ { k }$ algorithm for the Steiner network problem using the primal–dual schema, where k is the largest connectivity requirement specified in the instance. For simplicity, assume that the upper bounds, $u _ { e } ,$ are 1 for each edge e.

23.5 (Williamson, Goemans, Mihail, and Vazirani [258]) Say that a function $h : 2 ^ { V } \to \{ 0 , 1 \}$ is uncrossable if $h ( V ) = 0$ , and for any two sets $A , B \subset V$ , if $h ( A ) = h ( B ) = 1$ then h(A−B) = h(B −A) = 1 or $h ( A \cap B ) = h ( A \cup B ) = 1$ Exercise 22.7 asked for a factor 2 approximation algorithm for IP (22.1) for the case that f was a proper function. In this exercise, we will extend this further to the case that f is an uncrossable function. Now, we need to enhance the last step of Algorithm 22.3; the pruning step needs to be done using reverse delete. Again, F denotes the forest of edges picked by the algorithm. Let us say that a set $A \subset V$ is unsatisfied w.r.t. the picked edges F if h(A) = 1 and $\delta _ { F } ( A ) = \varnothing . \mathrm { ~ A ~ }$ minimal unsatisfied set will be said to be active. The algorithm is as follows.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 23.22 (Uncrossable function)
1. (Initialization) $F \leftarrow \emptyset$; for each $S \subseteq V$, $y_S \leftarrow 0$.
2. (Edge augmentation) while there exists an unsatisfied set do:
    simultaneously raise $y_S$ for each active set $S$, until some edge $e$ goes tight;
    $F \leftarrow F \cup \{e\}$.
3. Let $e_1, e_2, \ldots, e_l$ be the ordered list of edges in $F$.
4. (Reverse delete) For $j = l$ downto 1 do:
    If $F - \{e_j\}$ satisfies $h$, then $F \leftarrow F - \{e_j\}$.
5. Return $F$.
</div>

Show that in each iteration, active sets must be disjoint. Assuming that active sets can be eficiently found, show that Algorithm 23.22 finds a primal solution of cost at most twice the dual, i.e.,

$$
\sum_ {e \in F} c _ {e} \leq 2 \sum_ {S} y _ {S}.
$$

Hint: Corresponding to each edge $e \in F$ , there must be a set $A \subset V$ such that $h ( A ) = 1$ and $\delta _ { F } ( A ) = \{ e \}$ . Call such a set a witness for e. A family C consisting of a witness for each $e \in F$ is called a witness family. Include V in this family. Show, by uncrossing, that C can be assumed to be laminar and therefore can be viewed as a tree. Use this to prove that in each iteration, the average degree of active sets is at most two, as in Lemma 22.8.

23.6 Give an example to show that if reverse delete is replaced by a forward delete, then the approximation factor for Algorithm 23.22 can be unbounded for some uncrossable function.

23.7 (Goemans, Goldberg, Plotkin, Shmoys, Tardos, and Williamson [102]) We will solve the Steiner network problem in k phases, numbered $0 , 1 , \ldots , k -$ 1. In each phase, we will pick a forest from the remaining graph. The solution will be the union of the k forests. Let $F _ { p - 1 }$ denote the set of edges picked in phases numbered $0 , 1 , \ldots , p - 1$ . At the beginning of the pth phase, define the deficiency of set $S \subset V$ to be max $\{ f ( S ) - | \delta _ { F _ { p - 1 } } ( A ) | , 0 \}$ . The first $p - 1$ phases ensure that every set has deficiency at most $k - p .$ . In the pth phase, define function h as

$$
h (S) = \left\{ \begin{array}{l} 1 \text {   if   } \operatorname{deficiency} (S) = k - p \\ 0 \text {   otherwise } \end{array} \right.
$$

Show that h is an uncrossable function. Show that Algorithm 23.22 can be implemented in polynomial time for this uncrossable function, i.e., active sets can be found in polynomial time. Let F be the set of edges picked by Algorithm 23.22 from $E - F _ { p - 1 }$ , and y be the dual solution constructed when run with function h. Construct the dual program to LP (23.2), and show that there is a feasible solution, say d, to this program such that

$$
\sum_ {e \in F} c _ {e} \leq 2 \sum_ {S} y _ {S} \leq \frac {2}{k - p} g (\pmb {d}),
$$

where $g ( d )$ is the objective function value of dual solution d. Adding over al k phases leads to the required factor.

Hint: Use a max-flow algorithm for finding active sets. The dual program will have a variable $z _ { e }$ for each edge e. For edges $e \in F _ { p - 1 }$ , set $\begin{array} { r } { z _ { e } = \sum _ { S : ~ e \in \delta ( S ) } y _ { S } } \end{array}$ , for constructing a dual feasible solution.

23.8 Give an infinite family of graphs to show that the performance guarantee of the algorithm in Exercise 23.7 is tight within constant factors.

The following definitions will be useful for the next three exercises. These notions are connected to the theme of this chapter, i.e., small subgraphs with specified numbers of disjoint paths, via Menger’s theorem (see Exercise 12.5. An undirected graph is said to be k-vertex $( k { - } e d g e )$ connected if it has at least $k { \pm } 1$ vertices, and the removal of any set of at most $k - 1$ vertices (edges) from it leaves a connected graph. A directed graph is said to be k-vertex $( k { - } e d g e )$ connected if it has at least $k + 1$ vertices, and the deletion of any set of at most $k - 1$ vertices (edges) leaves a strongly connected graph.

23.9 (Cheriyan and Thurimella [42]) This exercise develops a $1 + 2 / k$ factor algorithm for the following problem.

Problem 23.23 (Minimum k-vertex connected subgraph) Given a nonnegative integer k and an undirected graph $G = ( V , E )$ that is k-vertex connected, find a minimum cardinality set $E ^ { \prime } \subset E$ such that the subgraph $G ^ { \prime } = ( V , E ^ { \prime } )$ is k-vertex connected.

Let $G = ( V , E )$ be k-vertex connected. We will say that edge $e \in E$ is critical if its removal leaves a graph that is not k-vertex connected. A simple cycle C in G is critical if every edge on C is critical. A theorem of Mader, which states that a critical cycle in $G$ must have a vertex of degree exactly $k ,$ is central to the algorithm.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 23.24 (k-vertex connected subgraph)
1. Find a minimum cardinality set  $M \subset E$  such that
 $\forall v \in V : \deg_{M}(v) \geq k - 1.$ 
2. Find a minimal set F such that  $M \cup F$  is k-vertex connected.
3. Output  $G' = (V, M \cup F)$ .
</div>

1. Give a polynomial time algorithm for Step 1 of Algorithm 23.24. Observe that $| M | \leq \mathrm { O P T }$

Hint: Use a b-matching algorithm on the complement of G. Given an undirected graph $G = ( V , E )$ and a function $b : V \to \mathbf { Z } ^ { + }$ specifying an upper bound for each vertex, the b-matching problem asks for a maximum cardinality set $M \subseteq E$ such that $\forall v \in V , \deg _ { M } ( v ) \leq b ( v )$ . This problem is in P.

2. Give an eficient implementation for Step 2 of Algorithm 23.24.

3. Use Mader’s theorem to show that F must be acyclic, and hence $| F | \le$ $| V | - 1$ . Use this to show that Algorithm 23.24 achieves an approximation factor of $1 + 2 / k$

Hint: Use the fact that $k | V | / 2$ is a lower bound on OPT.

23.10 (Cheriyan and Thurimella [42]) Consider the problem of finding a minimum k-vertex connected subgraph of a directed graph. Give an algorithm similar to that in Exercise 23.9 for achieving factor $1 + 2 / k$ for this problem. Use the following two facts.

1. In a directed graph, an alternating cycle, C, is an even length sequence of distinct edges $( v _ { 0 } , v _ { 1 } ) ( v _ { 2 } , v _ { 1 } ) ( v _ { 2 } , v _ { 3 } ) ( v _ { 4 } , v _ { 3 } ) \ldots ( v _ { m - 1 } , v _ { m } ) ( v _ { 0 } , v _ { m } )$ , where vertices are allowed to repeat. Notice that alternate vertices on C have two out-edges (two in-edges). Vertices having two out-edges (two inedges) will be called C-out $( C \ – i n )$ vertices. Mader showed that if G is a k-vertex connected directed graph containing an alternating cycle $C ,$ each of whose edges is critical, then C contains either a C-out vertex having out-degree exactly k or a C-in vertex having in-degree exactly k.

2. Given a directed graph $G = ( V , E )$ , define its associated bipartite graph H to be the following. Corresponding to each vertex $v \in V$ , H has two vertices, v<sub>−</sub> and $v _ { + }$ , and corresponding to each edge $( u , v ) \in E$ , H has the edge $( u _ { + } , v _ { - } )$ . There is an alternating cycle in G if its associated bipartite graph contains a cycle.

23.11 (Khuller and Vishkin [172], using Edmonds [72]) This exercise develops a factor 2 algorithm for the following problem.

Problem 23.25 (Minimum k-edge connected subgraph) Given an undirected graph $G = ( V , E )$ , a function w : $E \to \mathbf { Q } ^ { + }$ , and an integer k, find a minimum weight subgraph of G that is k-edge connected.

1. Let $r \in V$ be any vertex of G. Consider the problem of finding a minimum weight subgraph $G ^ { \prime }$ of $G$ such that for each vertex $v \in V ,$ there are k edge-disjoint paths from r to v in $G ^ { \prime }$ . Show that this problem is the same as Problem 23.25, i.e., any solution to one is also a solution to the other.

2. Let $G = ( V , E )$ be an edge-weighted directed graph and $r \in V$ be one of its vertices. A set $E ^ { \prime } \subseteq E$ is said to be an r-arborescence if every vertex, other than $r ,$ has in-degree 1. In efect, an r-arborescence is a spanning tree directed out of r. Define the r-connectivity of $G$ to be

max $\{ k \mid \forall v \in V$ ∃k edge-disjoint paths from r to v in $G \}$

Edmonds showed that the maximum number of edge-disjoint r-arborescences in $G$ is equal to the r-connectivity of $G .$ Use this to show that the problem of finding a minimum weight subgraph of G that has an $r -$ connectivity of k is the same as the problem of finding a minimum weight subgraph of G that has k edge-disjoint r-arborescences.

3. Edmonds showed that the edges of a directed graph $G = ( V , E )$ can be partitioned into k edge-disjoint r-arborescences if, on ignoring directions, $E$ can be partitioned into k spanning trees, and the in-degree of every vertex, other than r, is exactly k. Use this characterization to show that the problem of finding a minimum weight subgraph of G that has k edgedisjoint arborescences can be solved in polynomial time.

Hint: This problem can be expressed as a matroid intersection problem, the two matroids being a partition matroid and the k-fold union of a graphic matroid (which is also a matroid).

4. Let $G = ( V , E )$ be an edge-weighted undirected graph and $r \in V$ be one of its vertices. Let $\mathrm { O P T } ( G )$ denote the weight of an optimal solution to Problem 23.25 on instance $G .$ Obtain graph H by bidirecting G, i.e., by replacing each edge $( u , v ) \in E$ with the two edges $( u \to v )$ and $( v  u )$ each having the same weight as $( u , v )$ . Let $\mathrm { O P T } ( H )$ denote the weight of a minimum weight subgraph of H that can be partitioned into $k \ r -$ arborescences. Show that

$$
\operatorname{OPT} (G) \leq \operatorname{OPT} (H) \leq 2 \cdot \operatorname{OPT} (G).
$$

Use this to obtain a factor 2 approximation algorithm for Problem 23.25.

23.12 (Goemans and Bertsimas [101]) The metric Steiner network problem is the Steiner network problem with the restrictions that $G$ is a complete graph, the cost function on edges satisfies the triangle inequality, and $u _ { e } = \infty$ for each edge. It generalizes the metric Steiner tree problem to arbitrary connectivity requirements. For $D \subseteq V .$ , define $L P _ { S } ( D )$ to be LP-relaxation (23.2), together with a set of equality constraints for vertices in $D ,$ as follows.

$$
\begin{array}{l l} \text {minimize} & \sum_ {e \in E} c _ {e} x _ {e} \\ \text {subject to} & \sum_ {e: e \in \delta (S)} x _ {e} \geq f (S), \qquad S \subseteq V \\ & \sum_ {e: e \in \delta (\{v \})} x _ {e} = f (\{v \}), \qquad v \in D \\ & x _ {e} \geq 0, \qquad \qquad \qquad \qquad e \in E \end{array}\tag{23.4}
$$

It turns out that the equality constraints are redundant for the metric Steiner network problem. For any choice of $D \subseteq V .$ , an optimal solution to $L P _ { S } ( D )$ is also an optimal solution to $L P _ { S } ( \emptyset )$ . This is called the parsimonious property. Let us say that a vertex v is Steiner if it has no connectivity requirements, i.e., if $\forall v \in V , r ( u , v ) = 0$ . Use the parsimonious property to prove that there is a fractional optimal solution to the metric Steiner network problem which has no edges incident at Steiner vertices.

23.13 Consider the following integer program for the traveling salesman problem (Problem 3.5).

$$
\begin{array}{l l} \text { minimize } & \sum_ {e \in E} c _ {e} x _ {e} \\ \text { subject   to } & \sum_ {e: e \text { incident   at } v} x _ {e} = 2, \quad v \in V \\ & \sum_ {e: e \in \delta (S)} x _ {e} \geq 2, \quad S \subset V \\ & x _ {e} \in \{0, 1 \}, \quad e \in E \end{array}\tag{23.5}
$$

Show that optimal solutions to this integer program are optimal TSP tours. The linear relaxation of this program is called the subtour elimination $\mathrm { L P } .$ relaxation for TSP.

The rest of this exercise deals with the special case of metric TSP and develops a proof that the solution found by Christofides’ algorithm, Algorithm 3.10, is within a factor of $3 / 2$ of the optimal solution to this LP-relaxation.

1. Give an example that puts a lower bound of (essentially) $4 / 3$ on the integrality gap of this relaxation.

Hint: Use the following graph.

![](images/46e2244b679ebcd241270b0a88d691b451dbaa215ddaede3d26d15178c2b20bf.jpg)

2. Let $v _ { 1 }$ be an arbitrary vertex in the given graph $G = ( V , E )$ . Define a 1-tree in $G$ to be a spanning tree on the vertices $V - \{ v _ { 1 } \}$ , together with two distinct edges incident at vertex $v _ { 1 }$ . Clearly, the cost of an optimal 1-tree is a lower bound on the cost of an optimal TSP tour. LP (12.12) stated in Exercise 12.9 was an exact relaxation for the MST problem. Use it to obtain an exact LP-relaxation for the minimum 1-tree problem.

3. (Held and Karp [123]) Show that the cost of a minimum 1-tree is a lower bound on the cost of an optimal solution to the subtour elimination LP. Hint: Compare the LP obtained above for minimum 1-tree with the following equivalent formulation of the subtour elimination LP. (By e : $e \in S$ we mean edges e that have both endpoints in S.)

$$
\begin{array}{l l} \text {minimize} & \sum_ {e \in E} c _ {e} x _ {e} \\ \text {subject to} & \sum_ {e: e \text {incident at} v} x _ {e} = 2, \quad v \in V \\ & \sum_ {e: e \in S} x _ {e} \leq | S | - 1, \quad S \subseteq V \\ & x _ {e} \geq 0, \quad e \in E \end{array}\tag{23.6}
$$

4. Use the parsimonious property, introduced in Exercise 23.12, to show that the equality constraints, on vertices, in the subtour elimination LP are redundant. (Observe that the LP obtained on removing these constraints is also an LP-relaxation for the problem of finding a minimum cost spanning two-edge connected subgraph of G.)

5. For $D \subseteq V$ , let $L P _ { T } ( D )$ denote the subtour elimination LP for $G _ { D }$ , the subgraph of G induced on D. Let $\mathrm { O P T } _ { f } ( L P _ { T } ( D )$ denote the cost of an optimal solution to $L P _ { T } ( D )$ . Show the following monotonicity property

$$
\mathrm{OPT} _ {f} (L P _ {T} (D)) \leq \mathrm{OPT} _ {f} (L P _ {T} (V)).
$$

Hint: Use the relaxation without equality constraints.

6. Let $D \subseteq V$ be of even cardinality. Show that the cost of a minimum cost perfect matching in the subgraph of G induced on D is $\begin{array} { r } { \leq \frac { 1 } { 2 } \mathrm { O P T } _ { f } ( \bar { L } P _ { T } ( D ) ) } \end{array}$ .

Hint: Use LP (12.9), introduced in Exercise 12.8, for matching, and LP (23.6) for TSP.

7. Show that the metric TSP solution found using Algorithm 3.10, is within a factor of 3/2 of the optimal solution to the subtour elimination LP.

## 23.6 Notes

The result of this chapter is due to Jain [137]. Cheriyan and Thurimella [42] contains further results on finding small subgraphs of a given graph with a specified connectivity, as well as references to Mader’s theorems. The subtour elimination LP-relaxation for TSP was given in Dantzig, Ford, and Fulkerson [59]. The result of Exercise 23.13 was first established by Wolsey [260]. The proof developed here is from Shmoys and Williamson [240].