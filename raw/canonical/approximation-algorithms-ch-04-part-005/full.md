---
title: "approximation-algorithms-ch-04-part-005"
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
pdf_path: "work/core-books/approximation-algorithms/parts/approximation-algorithms-ch-04-part-005.pdf"
raw_md: "raw/canonical/approximation-algorithms-ch-04-part-005/full.md"
---
## 4 Multiway Cut and k-Cut

The theory of cuts occupies a central place in the study of exact algorithms. In this chapter, we will present approximation algorithms for natural generalizations of the minimum cut problem. These generalizations are NP-hard.

Given a connected, undirected graph $G = ( V , E )$ with an assignment of weights to edges, $w : E \to \mathbf { R } ^ { + }$ , a cut is defined by a partition of $V$ into two sets, say $V ^ { \prime }$ and $V - V ^ { \prime } .$ , and consists of all edges that have one endpoint in each partition. Clearly, the removal of the cut from $G$ disconnects $G .$ . Given terminals $s , t \in V$ , consider a partition of V that separates s and t. The cut defined by such a partition will be called an s–t cut. The problems of finding a minimum weight cut and a minimum weight $^ { s - t }$ cut can be eficiently solved using a maximum flow algorithm. Let us generalize these two notions:

Problem 4.1 (Multiway cut) Given a set of terminals $S = \{ s _ { 1 } , s _ { 2 } , . . . , s _ { k } \}$ $\subseteq V$ , a multiway cut is a set of edges whose removal disconnects the terminals from each other. The multiway cut problem asks for the minimum weight such set.

Problem 4.2 (Minimum k-cut) A set of edges whose removal leaves k connected components is called a k-cut. The k-cut problem asks for a minimum weight k-cut.

The problem of finding a minimum weight multiway cut is NP-hard for any fixed $k \geq 3$ . Observe that the case $k = 2$ is precisely the minimum $^ { s - t }$ cut problem. The minimum k-cut problem is polynomial time solvable for fixed $k ;$ however, it is NP-hard if k is specified as part of the input. In this chapter, we will obtain factor $2 - 2 / k$ approximation algorithms for both problems. In Chapter 19 we will improve the guarantee for the multiway cut problem to $3 / 2$

## 4.1 The multiway cut problem

Define an isolating cut for $s _ { i }$ to be a set of edges whose removal disconnects $s _ { i }$ from the rest of the terminals.

## Algorithm 4.3 (Multiway cut)

1. For each $i = 1 , \ldots , k$ , compute a minimum weight isolating cut for $s _ { i }$ , say $C _ { i }$

2. Discard the heaviest of these cuts, and output the union of the rest, say C.

Each computation in Step 1 can be accomplished by identifying the terminals in $S - \{ s _ { i } \}$ into a single node, and finding a minimum cut separating this node from $s _ { i } ;$ this takes one max-flow computation. Clearly, removing $C$ from the graph disconnects every pair of terminals, and so is a multiway cut.

Theorem 4.4 Algorithm $4 . 3$ achieves an approximation guarantee $o f 2 \mathrm { - } 2 / k$

Proof: Let A be an optimal multiway cut in $G .$ We can view $A$ as the union of k cuts as follows: The removal of A from $G$ will create k connected components, each having one terminal (since A is a minimum weight multiway cut, no more than k components will be created). Let $A _ { i }$ be the cut separating the component containing $s _ { i }$ from the rest of the graph. Then $\textstyle A = \bigcup _ { i = 1 } ^ { k } A _ { i }$

Since each edge of $A$ is incident at two of these components, each edge will be in two of the cuts $A _ { i }$ . Hence,

$$
\sum_ {i = 1} ^ {k} w (A _ {i}) = 2 w (A).
$$

Clearly, $A _ { i }$ is an isolating cut for $s _ { i } .$ . Since $C _ { i }$ is a minimum weight isolating cut for $s _ { i } , w ( C _ { i } ) \leq w ( A _ { i } )$ . Notice that this already gives a factor 2 algorithm, by taking the union of all k cuts $C _ { i }$ . Finally, since C is obtained by discarding the heaviest of the cuts $C _ { i }$ •

$$
w (C) \leq \left(1 - \frac {1}{k}\right) \sum_ {i = 1} ^ {k} w \left(C _ {i}\right) \leq \left(1 - \frac {1}{k}\right) \sum_ {i = 1} ^ {k} w \left(A _ {i}\right) = 2 \left(1 - \frac {1}{k}\right) w (A).\tag{□}
$$

Once again, Algorithm 4.3 is not based on a lower bounding scheme. Exercise 19.2 gives an algorithm with the same guarantee using an LP-relaxation as the lower bound. The use of LP-relaxations is fruitful for this problem as well. Section 19.1 gives an algorithm with an improved guarantee, using another LP-relaxation.

Example 4.5 A tight example for this algorithm is given by a graph on 2k vertices consisting of a k-cycle and a distinct terminal attached to each vertex of the cycle. The edges of the cycle have weight 1 and edges attaching terminals to the cycle have weight $2 - \varepsilon$ for a small fraction $\varepsilon > 0$

For example, the graph corresponding to $k = 4$ is:

![](images/ae24d953de40d2d132eae085c2ae94f80d551155e1eb7eadd55fcd212c533199.jpg)  
For each terminal $s _ { i }$ , the minimum weight isolating cuts for $s _ { i }$ is given by the edge incident to $s _ { i } . \mathrm { ~ S o } .$ , the cut $C$ returned by the algorithm has weight $( k - 1 ) ( 2 - \varepsilon )$ . On the other hand, the optimal multiway cut is given by the cycle edges, and has weight $k .$ ✷

## 4.2 The minimum k-cut problem

A natural algorithm for finding a k-cut is as follows. Starting with $G ,$ compute a minimum cut in each connected component and remove the lightest one; repeat until there are $k$ connected components. This algorithm does achieve a guarantee of $2 - 2 / k$ , however, the proof is quite involved. Instead we will use the Gomory–Hu tree representation $o f$ minimum cuts to give a simpler algorithm achieving the same guarantee.

Minimum cuts, as well as sub-optimal cuts, in undirected graphs have several interesting structural properties, as opposed to cuts in directed graphs (the algorithm of Section 28.2 is based on exploiting some of these properties). The existence of Gomory–Hu trees is one of the remarkable consequences of these properties.

Let $T$ be a tree on vertex set $V ;$ the edges of $T$ need not be in $E .$ Let $e$ be an edge in $T .$ . Its removal from $T$ creates two connected components. Let $S$ and $\overline { S }$ be the vertex sets of these components. The cut defined in graph $G$ by the partition $( S , { \overline { { S } } } )$ is the cut associated with $e$ in $G$ . Define a weight function $w ^ { \prime }$ on the edges of $T$ . Tree $T$ will be said to be a Gomory–Hu tree for G if

1. for each pair of vertices $u , v \in V$ , the weight of a minimum $u - v$ cut in $G$ is the same as that in $T$

2. for each edge $e \in T , w ^ { \prime } ( e )$ is the weight of the cut associated with e in $G ,$ and

We will need the following lemma.

A Gomory–Hu tree encodes, in a succinct manner, a minimum u–v cut in $G ,$ for each pair of vertices $u , v \in V$ as follows. A minimum u–v cut in $T$ is given by a minimum weight edge on the unique path from u to v in $T$ , say e. $\mathrm { B y }$ the properties stated above, the cut associated with e in $G$ is a minimum u–v cut, and has weight $w ^ { \prime } ( e )$ . So, for the $\binom { n } { 2 }$ pairs of vertices $u , v \in V$ , we need only $n - 1$  cuts, those encoded by the edges of a Gomory–Hu tree, to give minimum u–v cuts in $G .$

The following figure shows a weighted graph and its associated Gomory– Hu tree. Exercise 4.6 shows how to construct a Gomory–Hu tree for an undirected graph, using only n − 1 max-flow computations.

![](images/2c77969beeafbfb3eb663de1e3006edd7d80a5d5f34737c6af90585ba48025d6.jpg)

![](images/55e677dbd5d42d8d54239771ca4b253b8ae9cafbc9e727ff64c9d07c78061168.jpg)

Lemma 4.6 Let $S$ be the union of cuts in G associated with l edges of T. Then, the removal of S from G leaves a graph with at least $l + 1$ components.

Proof: Removing the corresponding l edges from T leaves exactly l + 1 connected components, say with vertex sets $V _ { 1 } , V _ { 2 } , \dots , V _ { l + 1 }$ . Clearly, removing S from G will disconnect each pair $V _ { i }$ and $V _ { j }$ . Hence we must get at least l + 1 connected components. ✷

As a consequence of Lemma 4.6, the union of $k - 1$ cuts picked from $T$ will form a k-cut in G. The complete algorithm is given below.

## Algorithm 4.7 (Minimum k-cut)

1. Compute a Gomory–Hu tree T for G.

2. Output the union of the lightest $k - 1$ cuts of the $n - 1$ cuts associated with edges of $T$ in $G ;$ let $C$ be this union.

By Lemma 4.6, the removal of C from G will leave at least k components. If more than $k$ components are created, throw back some of the removed edges until there are exactly k components.

Theorem 4.8 Algorithm 4.7 achieves an approximation factor of $2 - 2 / k$

Proof: Let A be an optimal k-cut in G. As in Theorem 4.4, we can view A as the union of k cuts: Let $V _ { 1 } , V _ { 2 } , \ldots , V _ { k }$ be the k components formed by removing A from $G ,$ and let $A _ { i }$ denote the cut separating $V _ { i }$ from the rest of the graph. Then $A = A _ { 1 } \cup \ldots \cup A _ { k }$ , and, since each edge of A lies in two of these cuts,

$$
\sum_ {i = 1} ^ {k} w (A _ {i}) = 2 w (A).
$$

Without loss of generality assume that $A _ { k }$ is the heaviest of these cuts. The idea behind the rest of the proof is to show that there are $k - 1$ cuts defined by the edges of $T$ whose weights are dominated by the weight of the cuts $A _ { 1 } , A _ { 2 } , \ldots , A _ { k - 1 }$ . Since the algorithm picks the lightest $k - 1$ cuts defined by $T ,$ the theorem follows.

The k −1 cuts are identified as follows. Let B be the set of edges of $T$ that connect across two of the sets $V _ { 1 } , V _ { 2 } , \dots , V _ { k }$ . Consider the graph on vertex set V and edge set $B .$ , and shrink each of the sets $V _ { 1 } , V _ { 2 } , \ldots , V _ { k }$ to a single vertex. This shrunk graph must be connected (since $T$ was connected). Throw edges away until a tree remains. Let $B ^ { \prime } \subseteq B$ be the left over edges, $| B ^ { \prime } | = k - 1$ The edges of $B ^ { \prime }$ define the required $k - 1$ cuts.

Next, root this tree at $V _ { k }$ (recall that $A _ { k }$ was assumed to be the heaviest cut among the cuts $A _ { i } )$ . This helps in defining a correspondence between the edges in $B ^ { \prime }$ and the sets $V _ { 1 } , V _ { 2 } , \dots , V _ { k - 1 }$ : each edge corresponds to the set it comes out of in the rooted tree.

![](images/989229c0e4eac19c9448d968c6e4ec6a9a328bda95c6f7597f8bdc6fd865aca9.jpg)

Suppose edge $( u , v ) \in B ^ { \prime }$ corresponds to set $V _ { i }$ in this manner. The weight of a minimum u–v cut in G is $w ^ { \prime } ( u , v )$ . Since $A _ { i }$ is a u–v cut in $G .$ ,

$$
w (A _ {i}) \geq w ^ {\prime} (u, v).
$$

Thus each cut among $A _ { 1 } , A _ { 2 } , \ldots , A _ { k - 1 }$ is at least as heavy as the cut defined in G by the corresponding edge of $B ^ { \prime } .$ . This, together with the fact that $C$ is the union of the lightest $k - 1$ cuts defined by T, gives:

$$
w (C) \leq \sum_ {e \in B ^ {\prime}} w ^ {\prime} (e) \leq \sum_ {i = 1} ^ {k - 1} w (A _ {i}) \leq \left(1 - \frac {1}{k}\right) \sum_ {i = 1} ^ {k} w (A _ {i}) = 2 \left(1 - \frac {1}{k}\right) w (A).
$$

Example 4.9 The tight example given above for multiway cuts on $2 k$ vertices also serves as a tight example for the k-cut algorithm (of course, there is no need to mark vertices as terminals). Below we give the example for $k = 4$ together with its Gomory–Hu tree.

![](images/92c65b84594683ad7bcc8e8383c7670101a065ac36a93c57fdf3a32d9b143774.jpg)

The lightest $k - 1$ cuts in the Gomory–Hu tree have weight $2 - \varepsilon$ each, corresponding to picking edges of weight $2 - \varepsilon$ of $G . \ \mathrm { S o }$ , the k-cut returned by the algorithm has weight $( k - 1 ) ( 2 - \varepsilon )$ . On the other hand, the optimal k-cut picks all edges of weight 1, and has weight k. ✷

## 4.3 Exercises

4.1 Show that Algorithm 4.3 can be used as a subroutine for finding a k-cut within a factor of $2 - 2 / k$ of the minimum k-cut. How many subroutine calls are needed?

4.2 A natural greedy algorithm for computing a multiway cut is the following. Starting with $G ,$ compute minimum $s _ { i } - s _ { j }$ cuts for all pairs $s _ { i } , s _ { j }$ that are still connected and remove the lightest of these cuts; repeat this until all pairs $s _ { i } , s _ { j }$ are disconnected. Prove that this algorithm also achieves a guarantee of $2 - 2 / k$

The next 4 exercises provide background and an algorithm for finding Gomory–Hu trees.

4.3 Let $G = ( V , E )$ be a graph and $w : E \to \mathbf { R } ^ { + }$ be an assignment of nonnegative weights to its edges. For $u , v \in V$ let $f ( u , v )$ denote the weight of a minimum u–v cut in $G .$

1. Let $u , v , w \in V .$ , and suppose $f ( u , v ) \leq f ( u , w ) \leq f ( v , w )$ . Show that $f ( u , v ) = f ( u , w )$ , i.e., the two smaller numbers are equal.

2. Show that among the <sup>n</sup> values $f ( u , v )$ , for all pairs $u , v \in V$ , there are at most $n - 1$  distinct values.

3. Show that for $u , v , w \in V$

$$
f (u, v) \geq \min \{f (u, w), f (w, v) \}.
$$

4. Show that for $u , v , w _ { 1 } , \ldots , w _ { r } \in V$

$$
f (u, w) \geq \min \{f (u, w _ {1}), f (w _ {1}, w _ {2}), \dots , f (w _ {r}, v) \}\tag{4.1}
$$

4.4 Let $T$ be a tree on vertex set V with weight function $w ^ { \prime }$ on its edges. We will say that $T$ is a flow equivalent tree if it satisfies the first of the two Gomory–Hu conditions. i.e., for each pair of vertices $u , v \in V$ , the weight of a minimum u–v cut in $G$ is the same as that in T. Let K be the complete graph on V . Define the weight of each edge $( u , v )$ in K to be $f ( u , v )$ . Show that any maximum weight spanning tree in K is a flow equivalent tree for $G _ { \ l }$ Hint: For $u , v \in V$ , let $u , w _ { 1 } , \ldots , w _ { r } , $ v be the unique path from u to v in $T .$ . Use (4.1) and the fact that since $T$ is a maximum weight spanning tree, $f ( u , v ) \leq \operatorname* { m i n } \{ f ( u , w _ { 1 } ) , \ldots , f ( w _ { r } , v ) \}$

4.5 Let $( A , { \bar { A } } )$ be a minimum $^ { s - t }$ cut such that $s \in A$ . Let x and $y$ be any two vertices in A. Consider the graph $G ^ { \prime }$ obtained by collapsing all vertices of $\bar { A }$ to a single vertex $v _ { \bar { A } }$ . The weight of any edge $( a , v _ { \bar { A } } )$ in $G ^ { \prime }$ is defined to be the sum of the weights of edges $( a , b )$ where $b \in { \bar { A } }$ . Clearly, any cut in $G ^ { \prime }$ defines a cut in $G _ { \ l }$ . Show that a minimum x–y cut in $G ^ { \prime }$ defines a minimum $x - y$ cut in $G .$

4.6 Now we are ready to state the Gomory–Hu algorithm. The algorithm maintains a partition of $V , \ ( S _ { 1 } , S _ { 2 } , \ldots S _ { t } )$ , and a spanning tree $T$ on the vertex set $\{ S _ { 1 } , \ldots , S _ { t } \}$ . Let $w ^ { \prime }$ be the function assigning weights to the edges of $T .$ Tree $T$ satisfies the following invariant.

Invariant: For any edge $( S _ { i } , S _ { j } )$ in T there are vertices a and b in $S _ { i }$ and $S _ { j }$ respectively, such that $w ^ { \prime } ( S _ { i } , S _ { j } ) = f ( a , b )$ , and the cut defined by edge $( { \bar { S } } _ { i } , S _ { j } )$ is a minimum $a { - } b$ cut in $G$

The algorithm starts with the trivial partition $V ,$ and proceeds in $n - 1$ iterations. In each iteration, it selects a set $S _ { i }$ in the partition such that $| S _ { i } | \ge 2$ and refines the partition by splitting $S _ { i }$ , and finding a tree on the refined partition satisfying the invariant. This is accomplished as follows. Let x and y be two distinct vertices in $S _ { i }$ . Root the current tree $T$ at $S _ { i }$ , and consider the subtrees rooted at the children of $S _ { i }$ . Each of these subtrees is collapsed into a single vertex, to obtain graph $G ^ { \prime }$ (besides these collapsed vertices, $G ^ { \prime }$ contains all vertices of $S _ { i } )$ . A minimum x– $_ y$ cut is found in $G ^ { \prime }$ Let $( A , B )$ be the partition of the vertices of $G ^ { \prime }$ defining this cut, with $x \in A$ and $y \in B ,$ and let $w _ { x y }$ be the weight of this cut. Compute $S _ { i } ^ { x } = S \cap A$ and $S _ { i } ^ { y } = S \cap B$ , the two sets into which $S _ { i }$ splits.

The algorithm updates the partition and the tree as follows. It refines the partition by replacing $S _ { i }$ with two sets $S _ { i } ^ { x }$ and $S _ { i } ^ { y }$ . The new tree has the edge $( S _ { i } ^ { x } , S _ { i } ^ { y } )$ ), with weight $w _ { x y }$ . Consider a subtree $\scriptstyle { \dot { T } } ^ { \prime }$ that was incident at $S _ { i }$ in T. Assume w.l.o.g. that the node corresponding to $T ^ { \prime }$ lies in $A .$ . Then, $T ^ { \prime }$ is connected by an edge to $S _ { i } ^ { x }$ . The weight of this connecting edge is the same as the weight of the edge connecting $T ^ { \prime }$ to $S _ { i }$ . All edges in $T ^ { \prime }$ retain their weights.

Show that the new tree satisfies the invariant. Hence show that the algorithm terminates (when the partition consists of singleton vertices) with a Gomory–Hu tree for G.

Consider the graph:

![](images/8dfa86b10795218d8eeb4a63086c515fa8c1da4c7ce31a10b27281e11abec107.jpg)

The execution of the Gomory–Hu algorithm is demonstrated below:

![](images/61ab5f61b3241e1d286df449c2b1ab8b7fabc294ea425f5a08908e5ada265f5f.jpg)

4.7 Prove that if the Gomory–Hu tree for an edge-weighted undirected graph G contains all n − 1 distinct weights, then G can have only one minimum weight cut.

## 4.4 Notes

Algorithm 4.3 is due to Dahlhaus, Johnson, Seymour, Papadimitriou and Yannakakis [57]. Algorithm 4.7 is due to Saran and Vazirani [233]; the proof given here is due to R. Ravi. For Gomory–Hu trees see Gomory and Hu [110].