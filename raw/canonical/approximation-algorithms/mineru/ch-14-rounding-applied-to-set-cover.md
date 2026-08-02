---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-14"
chapter_number: 14
chapter_title: "Rounding Applied to Set Cover"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 137
source_page_end: 142
printed_page_start: 119
printed_page_end: 124
part_ids: ["approximation-algorithms-ch-14-part-015"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Rounding Applied to Set Cover (MinerU semantic layer)

<!-- source-pages: 137-142; printed-pages: 119-124; mineru-part: approximation-algorithms-ch-14-part-015 -->

## 14 Rounding Applied to Set Cover

We will introduce the technique of LP-rounding by using it to design two approximation algorithms for the set cover problem, Problem 2.1. The first is a simple rounding algorithm achieving a guarantee of $f ,$ where $f$ is the frequency of the most frequent element. The second algorithm, achieving an approximation guarantee of $O ( \log n )$ , illustrates the use of randomization in rounding.

Consider the polyhedron defined by feasible solutions to an LP-relaxation. For some problems, one can find special properties of extreme point solutions of this polyhedron, which can yield rounding-based algorithms. One such property is half-integrality, i.e., in each extreme point solution, every coordinate is 0, 1, or 1/2. In Section 14.3 we will show that the vertex cover problem possesses this remarkable property. This directly gives a factor 2 algorithm for weighted vertex cover; namely, find an optimal extreme point solution and round all the halves to 1. A more general property, together with an enhanced rounding algorithm, called iterated rounding, is introduced in Chapter 23.

## 14.1 A simple rounding algorithm

A linear programming relaxation for the set cover problem is given in LP(13.2). One way of converting a solution to this linear program into an integral solution is to round up all nonzero variables to 1. It is easy to construct examples showing that this could increase the cost by a factor of $\varOmega ( n )$ (see Example 14.3). However, this simple algorithm does achieve the desired approximation guarantee of f (see Exercise 14.1). Let us consider a slight modification of this algorithm that is easier to prove and picks fewer sets in general:

Algorithm 14.1 (Set cover via LP-rounding)

1. Find an optimal solution to the LP-relaxation.

2. Pick all sets S for which $x _ { S } \geq 1 / f$ in this solution.

Theorem 14.2 Algorithm $1 / . 1$ achieves an approximation factor of f for the set cover problem.

Proof: Let C be the collection of picked sets. Consider an arbitrary element $e .$ Since e is in at most $f$ sets, one of these sets must be picked to the extent of at least $1 / f$ in the fractional cover. Thus, e is covered by $\mathcal { C } _ { : }$ , and hence $\mathcal { C }$ is a valid set cover. The rounding process increases $x _ { S }$ , for each set $S \in { \mathcal { C } } .$ by a factor of at most $f .$ . Therefore, the cost of $\mathcal { C }$ is at most $f$ times the cost of the fractional cover, thereby proving the desired approximation guarantee. ✷

The set cover instance arising from a vertex cover problem has $f = 2$ Therefore, Algorithm 14.1 gives a factor 2 approximation algorithm for the weighted vertex cover problem, thus matching the approximation guarantee established in Theorem 2.7.

Example 14.3 Let us give a tight example for Algorithm 14.1. For simplicity, we will view a set cover instance as a hypergraph: sets correspond to vertices and elements correspond to hyperedges (this is a generalization of the transformation that helped us view a set cover instance with each element having frequency 2 as a vertex cover instance).

Let $V _ { 1 } , \dots , V _ { k }$ be k disjoint sets of cardinality n each. The hypergraph has vertex set $V = V _ { 1 } \cup \ldots \cup V _ { k }$ , and $n ^ { k }$ hyperedges; each hyperedge picks one vertex from each $V _ { i }$ . In the set cover instance, elements correspond to hyperedges and sets correspond to vertices. Once again, inclusion corresponds to incidence. Each set has cost 1. Picking each set to the extent of $1 / k$ gives an optimal fractional cover of cost n. Given this fractional solution, the rounding algorithm will pick all nk sets. On the other hand, picking all sets corresponding to vertices in $V _ { 1 }$ gives a set cover of cost n. ✷

## 14.2 Randomized rounding

A natural idea for rounding an optimal fractional solution is to view the fractions as probabilities, flip coins with these biases and round accordingly. Let us show how this idea leads to an $O ( \log n )$ factor randomized approximation algorithm for the set cover problem.

First, we will show that each element is covered with constant probability by the sets picked by this process. Repeating this process $O ( \log n )$ times, and picking a set if it is chosen in any of the iterations, we get a set cover with high probability, by a standard coupon collector argument. The expected cost of cover picked in this manner is $O ( \log n ) { \cdot } \mathrm { O P T } _ { f } \leq O ( \log n ) { \cdot } \mathrm { O P T }$ , where $\mathrm { O P T } _ { f }$ is the cost of an optimal solution to the $\mathrm { L P } .$ -relaxation. Applying Markov’s Inequality, we convert this into a high probability statement. We provide details below.

Let $\mathbf { \boldsymbol { x } } = \mathbf { \boldsymbol { p } }$ be an optimal solution to the linear program. For each set $S \in S$ , pick S with probability $p _ { S }$ , the entry corresponding to $S$ in $\mathbf { \delta } _ { p . }$ Let $\mathcal { C }$ be the collection of sets picked. The expected cost of $\mathcal { C } _ { : }$

$$
\mathbf {E} [ \mathrm{cost} (\mathcal {C}) ] = \sum_ {S \in \mathcal {S}} \mathbf {P r} [ S \text {   is   picked } ] \cdot c _ {S} = \sum_ {S \in \mathcal {S}} p _ {S} \cdot c _ {S} = \mathrm{OPT} _ {f}.
$$

Next, let us compute the probability that an element $a \in U$ is covered by C. Suppose that a occurs in $k$ sets of S. Let the probabilities associated with these sets be $p _ { 1 } , \ldots , p _ { k }$ . Since a is fractionally covered in the optimal solution, $p _ { 1 } + p _ { 2 } + \cdot \cdot \cdot + p _ { k } \ge 1$ . Using elementary calculus, it is easy to show that under this condition, the probability that a is covered by $\mathcal { C }$ is minimized when each of the $p _ { i } \mathrm { ^ { * } s }$ is $1 / k$ . Thus,

$$
\mathbf {P r} [ a \text {   is   covered   by   } \mathcal {C} ] \geq 1 - \left(1 - \frac {1}{k}\right) ^ {k} \geq 1 - \frac {1}{e},
$$

where e is the base of natural logarithms. Hence each element is covered with constant probability by $\mathcal { C } .$

To get a complete set cover, independently pick c log n such subcollections, and compute their union, say $\scriptstyle { \mathcal { C } } ^ { \prime }$ , where c is a constant such that

$$
\left(\frac {1}{e}\right) ^ {c \log n} \leq \frac {1}{4 n}.
$$

Now,

Pr[a is not covered by $\mathcal { C } ^ { \prime } ] \leq \left( \frac { 1 } { e } \right) ^ { c \log n } \leq \frac { 1 } { 4 n } .$

Summing over all elements $a \in U$ , we get

$$
\mathbf {P r} [ \mathcal {C} ^ {\prime} \text {   is   not   a   valid   set   cover } ] \leq n \cdot \frac {1}{4 n} \leq \frac {1}{4}.
$$

Clearly, $\mathbf { E } [ \mathcal { C } ^ { \prime } ] \leq \mathrm { O P T } _ { f } \cdot c \log n$ . Applying Markov’s Inequality (see Section B.2) with $t = \mathrm { O P T } _ { f } \cdot 4 c \log n .$ , we get

$$
\mathbf {P r} [ \mathrm{cost} (\mathcal {C} ^ {\prime}) \geq \mathrm{OPT} _ {f} \cdot 4 c \log n ] \leq \frac {1}{4}.
$$

The probability of the union of the two undesirable events is $\leq 1 / 2$ . Hence,

$$
\mathbf {P r} [ \mathcal {C} ^ {\prime} \text {   is   a   valid   set   cover   and   has   cost   } \leq \mathrm{OPT} _ {f} \cdot 4 c \log n ] \geq \frac {1}{2}.
$$

## 14 Rounding Applied to Set Cover

Observe that we can verify in polynomial time whether $\mathcal { C } ^ { \prime }$ satisfies both these conditions. If not, we repeat the entire algorithm. The expected number of repetitions needed at most 2.

## 14.3 Half-integrality of vertex cover

Consider the vertex cover problem with arbitrary weights. Let $c : V \to \mathbf { Q } ^ { + }$ be the function assigning nonnegative weights to the vertices. The integer program for this problem is:

$$
\begin{array}{l l} \text { minimize } & \sum_ {v \in V} c (v) x _ {v} \\ \text { subject   to } & x _ {u} + x _ {v} \geq 1, \quad (u, v) \in E \\ & x _ {v} \in \{0, 1 \}, \quad v \in V \end{array}\tag{14.1}
$$

The LP-relaxation of this integer program is:

$$
\begin{array}{l l} \text { minimize } & \sum_ {v \in V} c (v) x _ {v} \\ \text { subject   to } & x _ {u} + x _ {v} \geq 1, \quad (u, v) \in E \\ & x _ {v} \geq 0, \quad v \in V \end{array}\tag{14.2}
$$

Recall that an extreme point solution of a set of linear inequalities is a feasible solution that cannot be expressed as convex combination of two other feasible solutions. A half-integral solution to LP (14.2) is a feasible solution in which each variable is $0 , 1 , \mathrm { o r ~ } 1 / 2$

Lemma 14.4 Let x be a feasible solution to $L P \ ( 1 \not { q } . \not { Q } )$ that is not halfintegral. Then, x is the convex combination of two feasible solutions and is therefore not an extreme point solution for the set of inequalities in $L P \left( { 1 4 . 2 } \right)$

Proof: Consider the set of vertices for which solution x does not assign half-integral values. Partition this set as follows.

$$
V _ {+} = \left\{v \left| \frac {1}{2} <   x _ {v} <   1 \right. \right\}, \quad V _ {-} = \left\{v \left| 0 <   x _ {v} <   \frac {1}{2} \right. \right\}.
$$

For $\varepsilon > 0$ , define the following two solutions.

$$
y _ {v} = \left\{ \begin{array}{c c} x _ {v} + \varepsilon , & x _ {v} \in V _ {+} \\ x _ {v} - \varepsilon , & x _ {v} \in V _ {-} \\ x _ {v}, o t h e r w i s e \end{array} \right., \quad z _ {v} = \left\{ \begin{array}{c c} x _ {v} - \varepsilon , & x _ {v} \in V _ {+} \\ x _ {v} + \varepsilon , & x _ {v} \in V _ {-} \\ x _ {v}, o t h e r w i s e. \end{array} \right.
$$

By assumption, $V _ { + } \cup V _ { - } \ne \emptyset$ , and so x is distinct from y and z. Furthermore, x is a convex combination of y and z, since $\pmb { x } = \frac { 1 } { 2 } ( \pmb { y } + \pmb { z } )$ . We will show, by choosing $\varepsilon > 0$ small enough, that y and z are both feasible solutions for LP (14.2), thereby establishing the lemma.

Ensuring that all coordinates of y and z are nonnegative is easy. Next, consider the edge constraints. Suppose $x _ { u } + x _ { v } > 1$ . Clearly, by choosing ε small enough, we can ensure that y and z do not violate the constraint for such an edge. Finally, consider an edge such that $x _ { u } + x _ { v } = 1$ . There are essentially three possibilities for $x _ { u }$ and $x _ { v } . \ x _ { u } = x _ { v } = { \textstyle { \frac { 1 } { 2 } } } ; \ x _ { u } = 0 , x _ { v } = 1$ ; and u ${ \mathrm { : } } \in V _ { + } , v \in V _ { - }$ . In all three cases, for any choice of $\varepsilon ,$

$$
x _ {u} + x _ {v} = y _ {u} + y _ {v} = z _ {u} + z _ {v} = 1.
$$

The lemma follows.

✷

This leads to:

Theorem 14.5 Any extreme point solution for the set of inequalities in $L P \ ( { \it 1 4 . 2 } )$ is half-integral.

Theorem 14.5 directly leads to a factor 2 approximation algorithm for weighted vertex cover: find an extreme point solution, and pick all vertices that are set to half or one in this solution.

## 14.4 Exercises

14.1 Modify Algorithm 14.1 so that it picks all sets that are nonzero in the fractional solution. Show that the algorithm also achieves a factor of $f .$ Hint: Use the primal complementary slackness conditions to prove this.

14.2 Consider the collection of sets, ${ \mathcal { C } } ,$ picked by the randomized rounding algorithm. Show that with some constant probability, C covers at least half the elements at a cost of at most O(OPT).

14.3 Give O(log n) factor randomized rounding algorithms for the set multicover and multiset multicover problems (see Section 13.2).

14.4 Give a (non-bipartite) tight example for the half-integrality-based algorithm for weighted vertex cover.

14.5 (J. Cheriyan) Give a polynomial time algorithm for the following problem. Given a graph G with nonnegative vertex weights and a valid, though not necessarily optimal, coloring of G, find a vertex cover of weight $\leq ( 2 - \frac { 2 } { k } ) \mathrm { O P T }$ where k is the number of colors used.

## 14 Rounding Applied to Set Cover

14.6 Give a counterexample to the following claim. A set cover instance in which each element is in exactly f sets has a $( 1 / f )$ -integral optimal fractional solution (i.e., in which each set is picked an integral multiple of $1 / f )$ .

14.7 This exercise develops a combinatorial algorithm for finding an optimal half integral vertex cover. Given undirected graph $G = ( V , E )$ and a nonnegative cost function c on vertices, obtain bipartite graph $H ( V ^ { \prime } , V ^ { \prime \prime } , E ^ { \prime } )$ as follows. Corresponding to each vertex $v \in V$ , there is a vertex $\boldsymbol { v } ^ { \prime } \in V ^ { \prime }$ and $v ^ { \prime \prime } \in V ^ { \prime \prime }$ each of cost $c ( v ) / 2$ . Corresponding to each edge $( u , v ) \in E$ , there are two edges $( u ^ { \prime } , v ^ { \prime \prime } ) , ( \dot { u } ^ { \prime \prime } , v ^ { \prime } ) \in E ^ { \prime }$ . Show that a vertex cover in H can be mapped to a half-integral vertex cover in G preserving total cost and vice versa. Use the fact that an optimal vertex cover in a bipartite graph can be found in polynomial time to obtain an optimal half-integral vertex cover in G.

14.8 Consider LP (12.8), introduced in Exercise 12.7, for a non-bipartite graph $G = ( V , E )$

1. Show that it is not an exact relaxation for the maximum matching problem in G.

2. Show that this LP always has a half-integral optimal solution.

14.9 In an attempt to improve the running time of the algorithm obtained in Exercise 9.7 for bin packing, consider going to the LP-relaxation of the integer programming and using LP-rounding. What guarantee can you establish for bin packing through this method?

## 14.5 Notes

Algorithm 14.1 is due to Hochbaum [125]. For a more sophisticated randomized rounding algorithm for set cover, see Srinivasan [244]. Theorem 14.5 is due to Nemhauser and Trotter [213].
