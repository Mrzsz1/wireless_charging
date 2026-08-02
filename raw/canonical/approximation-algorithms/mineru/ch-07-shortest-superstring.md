---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-07"
chapter_number: 7
chapter_title: "Shortest Superstring"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 79
source_page_end: 85
printed_page_start: 61
printed_page_end: 67
part_ids: ["approximation-algorithms-ch-07-part-008"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Shortest Superstring (MinerU semantic layer)

<!-- source-pages: 79-85; printed-pages: 61-67; mineru-part: approximation-algorithms-ch-07-part-008 -->

## 7 Shortest Superstring

In Chapter $2$ we defined the shortest superstring problem (Problem 2.9) and gave a preliminary approximation algorithm using set cover. In this chapter, we will first give a factor 4 algorithm, and then we will improve this to factor 3.

## 7.1 A factor 4 algorithm

We begin by developing a good lower bound on OPT. Let us assume that $s _ { 1 } , s _ { 2 } , \ldots , s _ { n }$ are numbered in order of leftmost occurrence in the shortest superstring, s.

![](images/d81a81d4732c0d77be53cf585cc24b3274b44a8bca0da56c47e6a2b4ecbfd1c6.jpg)  
Let overlap $\textstyle ( s _ { i } , s _ { j } )$ denote the maximum overlap between $s _ { i }$ and $s _ { j } .$ , i.e., the longest sufix of $s _ { i }$ that is a prefix of $s _ { j }$ . Also, let prefix $( s _ { i } , s _ { j } )$ be the prefix of $s _ { i }$ obtained by removing its overlap with $s _ { j }$ . The overlap in s between two consecutive $s _ { i } \mathrm { : }$ s is maximum possible, because otherwise a shorter superstring can be obtained. Hence, assuming that no $s _ { i }$ is a substring of another, we get

$$
\begin{array}{l} \text {OPT} = | \text {prefix} (s _ {1}, s _ {2}) | + | \text {prefix} (s _ {2}, s _ {3}) | + \ldots + | \text {prefix} (s _ {n}, s _ {1}) | \\ \qquad + | \text {overlap} (s _ {n}, s _ {1}) |. \end{array}\tag{7.1}
$$

Notice that we have repeated $s _ { 1 }$ at the end in order to obtain the last two terms of (7.1). This equality shows the close relation between the shortest superstring of S and the minimum traveling salesman tour on the prefix graph $o f S$ , defined as the directed graph on vertex set $\{ 1 , \ldots , n \}$ that contains an edge $i  j$ of weight $| \mathrm { p r e f i x } ( s _ { i } , s _ { j } ) |$ for each $i , j$ (self loops included). Clearly, |prefix( $^ { \mathfrak { j } _ { 1 } , s _ { 2 } ) | + | \mathrm { p r e f i x } ( s _ { 2 } , s _ { 3 } ) | + . . . + | \mathrm { p r e f i x } ( s _ { n } , s _ { 1 } ) | }$ represents the weight of the tour $1 \to 2 \to \dots \to n \to 1$ 1. Hence, by (7.1), the minimum weight of a traveling salesman tour of the prefix graph gives a lower bound on OPT. As such, this lower bound is not very useful, since we cannot eficiently compute a minimum traveling salesman tour.

The key idea is to lower-bound OPT using the minimum weight of a cycle cover of the prefix graph (a cycle cover is a collection of disjoint cycles covering all vertices). Since the tour $1 \to 2 \to \ldots \to n \to 1$ is a cycle cover, from (7.1) we get that the minimum weight of a cycle cover lower-bounds OPT.

Unlike minimum TSP, a minimum weight cycle cover can be computed in polynomial time. Corresponding to the prefix graph, construct the following bipartite graph, H. $U = \{ u _ { 1 } , \ldots , u _ { n } \}$ and $V = \{ v _ { 1 } , \ldots , v _ { n } \}$ are the vertex sets of the two sides of the bipartition. For each $i , j \in \{ 1 , \ldots , n \}$ add edge $( u _ { i } , v _ { j } )$ of weight $| \mathrm { p r e f i x } ( s _ { i } , s _ { j } ) |$ . It is easy to see that each cycle cover of the prefix graph corresponds to a perfect matching of the same weight in H and vice versa. Hence, finding a minimum weight cycle cover reduces to finding a minimum weight perfect matching in H.

If $c = ( i _ { 1 } \to i _ { 2 } \to \dots i _ { l } \to i _ { 1 } )$ is a cycle in the prefix graph, let

$$
\alpha (c) = \operatorname{prefix} \left(s _ {i _ {1}}, s _ {i _ {2}}\right) \circ \dots \circ \operatorname{prefix} \left(s _ {i _ {l - 1}}, s _ {i _ {l}}\right) \circ \operatorname{prefix} \left(s _ {i _ {l}}, s _ {i _ {1}}\right).
$$

Notice that each string $s _ { i _ { 1 } } , s _ { i _ { 2 } } , \ldots , s _ { i _ { l } }$ is a substring of $( \alpha ( c ) ) ^ { \infty }$ . Next, let

$$
\sigma (c) = \alpha (c) \circ s _ {i _ {1}}.
$$

Then $\sigma ( c )$ is a superstring of $s _ { i _ { 1 } } , \ldots , s _ { i _ { l } } . ^ { 1 }$ In the above construction, we “opened” cycle c at an arbitrary string $s _ { i _ { 1 } }$ . For the rest of the algorithm, we will call $s _ { i _ { 1 } }$ the representative string for c. We can now state the complete algorithm:

## Algorithm 7.1 (Shortest superstring – factor 4)

1. Construct the prefix graph corresponding to strings in S.

2. Find a minimum weight cycle cover of the prefix graph, ${ \mathcal { C } } = \{ c _ { 1 } , \ldots , c _ { k } \}$

3. Output $\sigma ( c _ { 1 } ) \circ \dots \circ \sigma ( c _ { k } )$

overlap(r, r<sup></sup>)

Clearly, the output is a superstring of the strings in S. Notice that if in each of the cycles we can find a representative string of length at most the weight of the cycle, then the string output is within $2 \cdot \mathrm { O P T }$ . Thus, the hard case is when all strings of some cycle c are long. But since they must all be substrings of $( \alpha ( c ) ) ^ { \infty }$ , they must be periodic. This will be used to prove Lemma 7.3, which establishes another lower bound on OPT.

Lemma 7.2 If each string in $S ^ { \prime } \subseteq S$ is a substring of $t ^ { \infty }$ for a string $t ,$ then there is a cycle of weight at most |t| in the prefix graph covering all the vertices corresponding to strings in $S ^ { \prime }$

Proof: For each string in $S ^ { \prime }$ , locate the starting point of its first occurrence in $t ^ { \infty }$ . Clearly, all these starting points will be distinct (since no string in $S$ is a substring of another) and will lie in the first copy of t. Consider the cycle in the prefix graph visiting the corresponding vertices in this order. Clearly, the weight of this cycle is at most |t|. ✷

Lemma 7.3 Let c and $c ^ { \prime }$ be two cycles in ${ \mathcal { C } } ,$ and let $r , r ^ { \prime }$ be representative strings from these cycles. Then

$$
| \mathrm{overlap} (r, r ^ {\prime}) | <   \mathrm{wt} (c) + \mathrm{wt} (c ^ {\prime}).
$$

Proof: Suppose, for contradiction, that |overlap $( r , r ^ { \prime } ) | \geq \mathrm { w t } ( c ) + \mathrm { w t } ( c ^ { \prime } )$ . Denote by $\alpha \left( \alpha ^ { \prime } \right)$ the prefix of length wt $( c ) \ ( \mathrm { w t } ( c ^ { \prime } )$ , respectively) of $\mathrm { o v e r l a p } ( r , r ^ { \prime } )$

![](images/70f38e8777abf9bc99e156b08b4bb06f0ef4586f04d1482c5616edbd03dd5252.jpg)

Clearly, overlap $( r , r ^ { \prime } )$ is a prefix of both $\alpha ^ { \infty }$ and $( \alpha ^ { \prime } ) ^ { \infty }$ . In addition, α is a prefix of $( \alpha ^ { \prime } ) ^ { \infty }$ and $\alpha ^ { \prime }$ is a prefix of $\alpha ^ { \infty }$ . Since overla $\mathfrak { z } ( r , r ^ { \prime } ) \geq | \alpha | + | \alpha ^ { \prime } |$ , it follows that α and $\alpha ^ { \prime }$ commute, i.e., α $\circ \alpha ^ { \prime } = \alpha ^ { \prime } \circ \alpha$ . But then, $\alpha ^ { \infty } = ( \alpha ^ { \prime } ) ^ { \infty }$ This is so because for any $k > 0$ ，

$$
\alpha^ {k} \circ (\alpha^ {\prime}) ^ {k} = (\alpha^ {\prime}) ^ {k} \circ \alpha^ {k}.
$$

Hence, for any $N > 0$ , the prefix of length N of $\alpha ^ { \infty }$ is the same as that of $( \alpha ^ { \prime } ) ^ { \infty }$

Now, by Lemma 7.2, there is a cycle of weight at most $\mathrm { w t } ( c )$ in the prefix graph covering all strings in c and $c ^ { \prime } ,$ contradicting the fact that C is a minimum weight cycle cover. ✷

Theorem 7.4 Algorithm 7.1 achieves an approximation factor of 4 for the shortest superstring problem.

Proof: Let $\begin{array} { r } { \mathrm { w t } ( \mathcal { C } ) = \sum _ { i = 1 } ^ { k } \mathrm { w t } ( c _ { i } ) } \end{array}$ . The output of the algorithm has length

$$
\sum_ {i = 1} ^ {k} | \sigma (c _ {i}) | = \mathrm{wt} (\mathcal {C}) + \sum_ {i = 1} ^ {k} | r _ {i} |,
$$

where $r _ { i }$ denotes the representative string from cycle $c _ { i }$ . We have shown that $\mathrm { w t } ( \mathcal { C } ) \leq \mathrm { O P T }$ . Next, we show that the sum of the lengths of representative strings is at most 3 · OPT.

Assume that $r _ { 1 } , \ldots , r _ { k }$ are numbered in order of their leftmost occurrence in the shortest superstring of S. Using Lemma 7.3, we get the following lower bound on OPT:

$$
\mathrm{OPT} \geq \sum_ {i = 1} ^ {k} | r _ {i} | - \sum_ {i = 1} ^ {k - 1} | \text { overlap } (r _ {i}, r _ {i + 1}) | \geq \sum_ {i = 1} ^ {k} | r _ {i} | - 2 \sum_ {i = 1} ^ {k} \operatorname{wt} (c _ {i}).
$$

Hence,

$$
\sum_ {i = 1} ^ {k} \left| r _ {i} \right| \leq \mathrm{OPT} + 2 \sum_ {i = 1} ^ {k} \mathrm{wt} (c _ {i}) \leq 3 \cdot \mathrm{OPT}.
$$

## 7.2 Improving to factor 3

Notice that any superstring of the strings $\sigma ( c _ { i } ) , i = 1 , \ldots , k$ , is also a superstring of all strings in S. Instead of simply concatenating these strings, let us make them overlap as much as possible (this may sound circular, but it is not!).

Let X be a set of strings. We will denote by ||X|| the sum of the lengths of the strings in X. Let us define the compression achieved by a superstring s as the diference between the sum of the lengths of the input strings and $| s | , \mathrm { i . e . , } | | S | | - | s |$ . Clearly, maximum compression is achieved by the shortest superstring. Several algorithms are known to achieve at least half the optimal compression. For instance, the greedy superstring algorithm, described in Section 2.3, does so; however, its proof is based on a complicated case analysis. For a less eficient algorithm, see Section 7.2.1. Either of these algorithms can be used in Step 3 of Algorithm 7.5.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 7.5 (Shortest superstring - factor 3)  
1. Construct the prefix graph corresponding to strings in $S$.  
2. Find a minimum cycle cover of the prefix graph, $\mathcal{C} = \{c_1, \ldots, c_k\}$.  
3. Run the greedy superstring algorithm on $\{\sigma(c_1), \ldots, \sigma(c_k)\}$ and output the resulting string, say $\tau$.
</div>

Let $\mathrm { O P T } _ { \sigma }$ denote the length of the shortest superstring of the strings in ${ \cal { S } } _ { \sigma } = \{ \sigma ( c _ { 1 } ) \ldots \sigma ( c _ { k } ) \}$ , and let $r _ { i }$ be the representative string of $c _ { i }$ .

Lemma 7.6 $| \tau | \leq \mathrm { O P T } _ { \sigma } + \mathrm { w t } ( \mathcal { C } ) .$

Proof: Assume w.l.o.g. that $\sigma ( c _ { 1 } ) , \ldots , \sigma ( c _ { k } )$ appear in this order in a shortest superstring of $S _ { \sigma }$ . The maximum compression that can be achieved on $S _ { \sigma }$ is given by

$$
\sum_ {i = 1} ^ {k - 1} | \mathrm{overlap} (\sigma (c _ {i}), \sigma (c _ {i + 1})) |.
$$

Since each string $\sigma ( c _ { i } )$ has $r _ { i }$ as a prefix as well as sufix, by Lemma 7.3,

$$
\left| \operatorname{overlap} \left(\sigma \left(c _ {i}\right), \sigma \left(c _ {i + 1}\right)\right) \right| \leq \operatorname{wt} \left(c _ {i}\right) + \operatorname{wt} \left(c _ {i + 1}\right).
$$

Hence, the maximum compression achievable on $S _ { \sigma }$ is at most $2 \cdot \mathrm { w t } ( { \mathcal { C } } )$ , i.e., $| | S _ { \sigma } | | - \mathrm { O P T } _ { \sigma } \leq 2 \cdot \mathrm { w t } ( \mathcal { C } )$

The compression achieved by the greedy superstring algorithm on $S _ { \sigma }$ is at least half the maximum compression. Therefore,

$$
| | S _ {\sigma} | | - | \tau | \geq \frac {1}{2} (| | S _ {\sigma} | | - \mathrm{OPT} _ {\sigma}).
$$

Therefore,

$$
2 (| \tau | - \mathrm{OPT} _ {\sigma}) \leq | | S _ {\sigma} | | - \mathrm{OPT} _ {\sigma} \leq 2 \cdot \operatorname{wt} (\mathcal {C}).
$$

The lemma follows.

Finally, we relate $\mathrm { O P T } _ { \sigma }$ to OPT.

Lemma 7.7

$$
\mathrm{OPT} _ {\sigma} \leq \mathrm{OPT} + \operatorname{wt} (\mathcal {C}).
$$

Proof: Let $\mathrm { O P T } _ { \ l }$ denote the length of the shortest superstring of the strings in $S _ { r } = \{ r _ { 1 } , \ldots , r _ { k } \}$ . The key observation is that each $\sigma ( c _ { i } )$ begins and ends with $r _ { i }$ . Therefore, the maximum compression achievable on $S _ { \sigma }$ is at least as large as that achievable on $S _ { r } , \mathrm { i . e . }$ ，

$$
\left| \left| S _ {\sigma} \right| \right| - \mathrm{OPT} _ {\sigma} \geq \left| \left| S _ {r} \right| \right| - \mathrm{OPT} _ {r}.
$$

Clearly, $\lvert \lvert S _ { \sigma } \rvert \rvert = \lvert \lvert S _ { r } \rvert \rvert + \mathrm { w t } ( \mathcal { C } )$ . This gives

$$
\mathrm{OPT} _ {\sigma} \leq \mathrm{OPT} _ {r} + \mathrm{wt} (\mathcal {C}).
$$

The lemma follows by noticing that $\mathrm { O P T } _ { r } \leq \mathrm { O P T }$

Combining the previous two lemmas we get:

Theorem 7.8 Algorithm 7.5 achieves an approximation factor of 3 for the shortest superstring problem.

## 7.2.1 Achieving half the optimal compression

We give a superstring algorithm that achieves at least half the optimal compression. Suppose that the strings to be compressed, $s _ { 1 } , \cdots , s _ { n }$ , are numbered in the order in which they appear in a shortest superstring. Then, the optimal compression is given by

$$
\sum_ {i = 1} ^ {n - 1} | \mathrm{overlap} (\sigma_ {i}, \sigma_ {i + 1}) |.
$$

This is the weight of the traveling salesman path $1 \to 2 \to \ldots \to n$ in the overlap $g r a p h , H$ , of the strings $s _ { 1 } , \cdots , s _ { n } . \ H$ is a directed graph that has a vertex $v _ { i }$ corresponding to each string $s _ { i } .$ and contains an edge $( v _ { i }  v _ { j } )$ of weight $| \mathrm { o v e r l a p } ( s _ { i } , s _ { j } ) |$ | for each $i \neq j , 1 \leq i , j \leq n$ (H has no self loops).

The optimal compression is upper bounded by the cost of a maximum traveling salesman tour in H, which in turn is upper bounded by the cost of a maximum cycle cover. The latter can be computed in polynomial time using matching, similar to the way we computed a minimum weight cycle cover. Since H has no self loops, each cycle has length at least 2. Remove the lightest edge from each cycle of the maximum cycle cover to obtain a set of disjoint paths. The sum of weights of edges on these paths is at least half the optimal compression. Overlap strings $s _ { 1 } , \cdots , s _ { n }$ according to the edges of these paths and concatenate the resulting strings. This gives a superstring achieving at least half the optimal compression.

## 7.3 Exercises

7.1 Show that Lemma 7.3 cannot be strengthened to

$$
\left| \operatorname{overlap} \left(r, r ^ {\prime}\right) \right| <   \max \left\{\operatorname{wt} (c), \operatorname{wt} \left(c ^ {\prime}\right) \right\}.
$$

7.2 (Jiang, Li, and Du [148]) Obtain constant factor approximation algorithms for the variants of the shortest superstring problem given in Exercise 2.16.

## 7.4 Notes

The algorithms given in this chapter are due to Blum, Jiang, Li, Tromp, and Yannakakis [27].
