---
title: "approximation-algorithms-ch-13-part-014"
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
pdf_path: "work/core-books/approximation-algorithms/parts/approximation-algorithms-ch-13-part-014.pdf"
raw_md: "raw/canonical/approximation-algorithms-ch-13-part-014/full.md"
---
## 13 Set Cover via Dual Fitting

In this chapter we will introduce the method of dual fitting, which helps analyze combinatorial algorithms using LP-duality theory. Using this method, we will present an alternative analysis of the natural greedy algorithm (Algorithm 2.2) for the set cover problem (Problem 2.1). Recall that in Section 2.1 we deferred giving the lower bounding method on which this algorithm was based. We will provide the answer below. The power of this approach will become apparent when we show the ease with which it extends to solving several generalizations of the set cover problem (see Section 13.2).

The method of dual fitting can be described as follows, assuming a minimization problem: The basic algorithm is combinatorial – in the case of set cover it is in fact the simple greedy algorithm. Using the linear programming relaxation of the problem and its dual, one shows that the primal integral solution found by the algorithm is fully paid for by the dual computed; however, the dual is infeasible. By $f u l l y$ paid $f o r$ we mean that the objective function value of the primal solution found is at most the objective function value of the dual computed. The main step in the analysis consists of dividing the dual by a suitable factor and showing that the shrunk dual is feasible, $\mathrm { i . e . }$ , it fits into the given instance. The shrunk dual is then a lower bound on OPT, and the factor is the approximation guarantee of the algorithm.

## 13.1 Dual-fitting-based analysis for the greedy set cover algorithm

To formulate the set cover problem as an integer program, let us assign a variable $x _ { S }$ for each set $S \in S$ , which is allowed $0 / 1$ values. This variable will be set to 1 if set $S$ is picked in the set cover. Clearly, the constraint is that for each element $e \in U$ we want that at least one of the sets containing it be picked.

$$
\text { minimize } \quad \sum_ {S \in \mathcal {S}} c (S) x _ {S}
$$

$$
\text { subject   to } \quad \sum_ {S: e \in S} x _ {S} \geq 1, \quad e \in U\tag{13.1}
$$

$$
x _ {S} \in \{0, 1 \}, \qquad S \in \mathcal {S}
$$

The LP-relaxation of this integer program is obtained by letting the domain of variables $x _ { S }$ be $1 \geq x _ { S } \geq 0$ . Since the upper bound on $x _ { S }$ is redundant, we get the following LP. A solution to this LP can be viewed as a fractional set cover.

$$
\mathrm{minimize} \sum_ {S \in \mathcal {S}} c (S) x _ {S}\tag{13.2}
$$

$$
\begin{array}{l l} \text {subject to} & \sum_ {S: e \in S} x _ {S} \geq 1, \quad e \in U \\ & x _ {S} \geq 0, \quad S \in \mathcal {S} \end{array}
$$

Example 13.1 Let us give a simple example to show that a fractional set cover may be cheaper than the optimal integral set cover. Let $U = \{ e , f , g \}$ and the specified sets be $S _ { 1 } = \{ e , f \} , S _ { 2 } = \{ f , g \} , S _ { 3 } = \{ e , g \}$ , each of unit cost. An integral cover must pick two of the sets for a cost of 2. On the other hand, picking each set to the extent of $1 / 2$ gives a fractional cover of cost $3 / 2$ ✷

Introducing a variable $y _ { e }$ corresponding to each element $e \in U$ , we obtain the dual program.

$$
\begin{array}{l l} \text { maximize } & \sum_ {e \in U} y _ {e} \\ \text { subject   to } & \sum_ {e: e \in S} y _ {e} \leq c (S), \quad S \in \mathcal {S} \\ & y _ {e} \geq 0, \quad e \in U \end{array}\tag{13.3}
$$

Intuitively, why is LP (13.3) the dual of LP (13.2)? In our experience, this is not the right question to be asked. As stated in Section 12.1, there is a purely mechanical procedure for obtaining the dual of a linear program. Once the dual is obtained, one can devise intuitive, and possibly physically meaningful, ways of thinking about it. Using this mechanical procedure, one can obtain the dual of a complex linear program in a fairly straightforward manner. Indeed, the LP-duality-based approach derives its wide applicability from this fact.

An intuitive way of thinking about LP (13.3) is that it is packing “stuf” into elements, trying to maximize the total amount packed, subject to the constraint that no set is overpacked. A set is said to be overpacked if the total amount packed into its elements exceeds the cost of the set. Whenever the coeficients in the constraint matrix, objective function, and right-hand side are all nonnegative, the minimization LP is called a covering $L P$ and the maximization LP is called a packing LP. Thus, (13.2) and (13.3) are a covering-packing pair of linear programs. Such pairs of programs will arise frequently in subsequent chapters.

![](images/93fb37a34ebc7466dbf9528bcabf6fbde039dff2352d3970506b9adafc75ab2f.jpg)

At this point, we can state the lower bounding scheme being used by Algorithm 2.2. Denote by $\mathrm { O P T } _ { f }$ the cost of an optimal fractional set cover, i.e., an optimal solution to LP (13.2). Clearly $\mathrm { O P T } _ { f } \ \leq \ \mathrm { O P T }$ , the cost of an optimal (integral) set cover. The cost of any feasible solution to the dua program, LP (13.3), is a lower bound on $\mathrm { O P T } _ { f } ,$ , and hence also on $\mathrm { O P T }$ Algorithm 2.2 uses this as the lower bound.

Algorithm 2.2 defines dual variables price(e), for each element, e. Observe that the cover picked by the algorithm is fully payed for by this dual solution. However, in general, this dual solution is not feasible (see Exercise 13.2). We will show below that if this dual is shrunk by a factor of $H _ { n } ,$ it fits into the given set cover instance, i.e., no set is overpacked. For each element e define,

$$
y _ {e} = \frac {\mathrm{price} (e)}{H _ {n}}.
$$

Algorithm 2.2 uses the dual feasible solution, $^ { \mathbf { \psi } } { \mathbf { \psi } } _ { \mathbf { { y } } } ,$ as the lower bound on $\mathrm { O P T } .$

Lemma 13.2 The vector y defined above is a feasible solution for the dual program (13.3).

Proof: We need to show that no set is overpacked by the solution y. Consider a set $S \in S$ consisting of k elements. Number the elements in the order in which they are covered by the algorithm, breaking ties arbitrarily, say $e _ { 1 } , \ldots , e _ { k }$

Consider the iteration in which the algorithm covers element $e _ { i } .$ . At this point, S contains at least $k - i + 1$ uncovered elements. Thus, in this iteration, $S$ itself can cover $e _ { i }$ at an average cost of at most $c ( S ) / ( k - i + 1 )$ . Since the algorithm chose the most cost-efective set in this iteration, $\mathrm { p r i c e } ( e _ { i } ) \leq$ $c ( S ) / ( k - i + 1 )$ . Thus,

$$
y _ {e _ {i}} \leq \frac {1}{H _ {n}} \cdot \frac {c (S)}{k - i + 1}.
$$

Summing over all elements in $S ,$

$$
\sum_ {i = 1} ^ {k} y _ {e _ {i}} \leq \frac {c (S)}{H _ {n}} \cdot \left(\frac {1}{k} + \frac {1}{k - 1} + \dots + \frac {1}{1}\right) = \frac {H _ {k}}{H _ {n}} \cdot c (S) \leq c (S).
$$

Therefore, S is not overpacked.

Theorem 13.3 The approximation guarantee of the greedy set cover algorithm is $H _ { n }$

Proof: The cost of the set cover picked is

$$
\sum_ {e \in U} \mathrm{price} (e) = H _ {n} \left(\sum_ {e \in U} y _ {e}\right) \leq H _ {n} \cdot \mathrm{OPT},
$$

where OPT denotes the cost of the optimal fractional set cover. The last inequality follows from the fact that y is dual feasible. ✷

## 13.1.1 Can the approximation guarantee be improved?

Consider the three questions raised in Section 1.1.2 regarding improving the approximation guarantee for vertex cover. Let us ask analogous questions for set cover. The first and third questions are already answered in Section 2.1.

As a corollary of Theorem 13.3 we get an upper bound of $H _ { n }$ on the integrality gap of relaxation (13.2). Example 13.4 shows that this bound is essentially tight. Since the integrality gap of the LP-relaxation used bounds the best approximation factor one can hope to achieve using this relaxation, the answer to the second question is also essentially $^ { 6 } \mathrm { n o } ^ { 9 }$

Example 13.4 Consider the following set cover instance. Let $n = 2 ^ { k } - 1$ where k is a positive integer, and let $U = \{ e _ { 1 } , e _ { 2 } , \ldots , e _ { n } \}$ . For $1 \leq i \leq n ,$ consider i written as a k-bit number. We can view this as a k-dimensional vector over $G F [ 2 ]$ . Let i denote this vector. For $1 \leq i \leq n$ define set $S _ { i } =$ $\{ e _ { j } | { \bf \delta i } \cdot { \bf j } = 1 \}$ , where $\mathbf { i } \cdot \mathbf { j }$ denotes the inner product of these two vectors. Finally, let $S = \{ S _ { 1 } , \ldots , S _ { n } \}$ , and define the cost of each set to be 1.

It is easy to check that each set contains $2 ^ { k - 1 } = ( n + 1 ) / 2$ elements, and each element is contained in $( n + 1 ) / 2$ sets. Thus, $x _ { i } = 2 / ( n + 1 )$ , $1 \leq i \leq n$ is a fractional set cover. Its cost is $2 n / ( n + 1 )$

Next, we will show that any integral set cover must pick at least k of the sets. Consider the union of any $p$ sets, where $p < k$ . Let $i _ { 1 } , \ldots , i _ { p }$ be the indices of these $p$ sets, and let A be a $p \times k$ matrix over $G F [ 2 ]$ whose rows consist of vectors $\mathbf { i } _ { 1 } , \ldots , \mathbf { i } _ { p } ,$ respectively. Since the rank of A is $< k ,$ , the dimension of its null space is $\geq 1$ , and so the null space contains a nonzero vector, say j. Since $A { \bf j } = { \bf 0 }$ , the element $e _ { j }$ is not in any of the $p$ sets. Hence the p sets do not form a cover.

Therefore, any integral set cover has cost at least $k = \log _ { 2 } { ( n + 1 ) }$ . Hence, the lower bound on the integrality gap established by this example is

$$
\left(\frac {n + 1}{2 n}\right) \cdot \log_ {2} (n + 1) > \frac {\log_ {2} n}{2}.
$$

✷

## 13.2 Generalizations of set cover

The greedy algorithm and its analysis using dual fitting extend naturally to several generalizations of the set cover problem (see Exercise 13.4).

• Set multicover: Each element, e, needs to be covered a specified integer number, $r _ { e } ,$ , of times. The objective again is to cover all elements up to their coverage requirements at minimum cost. We will assume that the cost of picking a set S k times is kcost(S).

• Multiset multicover: We are given a collection of multisets, rather than sets, of U. A multiset contains a specified number of copies of each element. Let $M ( S , e )$ denote the multiplicity of element e in set S. The instance satisfies the condition that the multiplicity of an element in a set is at most its coverage requirement, i.e., $\forall S , e \ M ( S , e ) \ \leq \ r _ { e }$ . The objective is the same as before.

• Covering integer programs: These are integer programs of the form

minimize c · x

subject to Ax ≥ b,

where all entries in $A , b , c$ are nonnegative and x is required to be nonnegative and integral.

## 13.2.1 Dual fitting applied to constrained set multicover

In this section, we will present an $H _ { n }$ factor approximation algorithm for set multicover with the additional constraint that each set can be picked at most once. Let us call this the constrained set multicover problem. One interesting feature of this problem is that its linear relaxation and dual contain negative coeficients and thus do not form a covering-packing pair of $\mathrm { L P s }$

Let $r _ { e } \in Z _ { + }$ be the coverage requirement for each element $e \in U$ . The integer programming formulation of constrained set multicover is not very diferent from that of set cover.

$$
\begin{array}{l l} \text {minimize} & \sum_ {S \in \mathcal {S}} c (S) x _ {S} \\ \text {subject to} & \sum_ {S: e \in S} x _ {S} \geq r _ {e}, \quad e \in U \\ & x _ {S} \in \{0, 1 \}, \quad S \in \mathcal {S} \end{array}\tag{13.4}
$$

Notice, however, that in the LP-relaxation, the constraints $x _ { S } \ \leq \ 1$ are no longer redundant. If we drop them, then a set may be picked multiple times to satisfy the coverage requirement of the elements. Thus, the LP-relaxation looks diferent from that for set cover. In particular, because of the negative numbers in the constraint matrix and the right-hand side, it is not even a covering linear program. The analysis given below deals with this added complexity.

$$
\begin{array}{l l} \text {minimize} & \sum_ {S \in \mathcal {S}} c (S) x _ {S} \\ \text {subject to} & \sum_ {S: e \in S} x _ {S} \geq r _ {e}, \quad e \in U \\ & - x _ {S} \geq - 1, \quad S \in \mathcal {S} \\ & x _ {S} \geq 0, \quad S \in \mathcal {S} \end{array}\tag{13.5}
$$

The additional constraints in the primal lead to new variables, $z _ { S } ,$ , in the dual. The dual also has negative numbers in the constraint matrix and is not a packing program. Now, a set S can be overpacked with the $y _ { e } \mathrm { ^ s } .$ However, this can be done only if we raise $z _ { S }$ to ensure feasibility, which in turn decreases the objective function value. Overall, overpacking may still be advantageous, since the $y _ { e } \mathrm { ^ s }$ appear with coeficients of $r _ { e }$ in the objective function.

$$
\begin{array}{l l} \text {maximize} & \sum_ {e \in U} r _ {e} y _ {e} - \sum_ {S \in \mathcal {S}} z _ {S} \\ \text {subject to} & \left(\sum_ {e: e \in S} y _ {e}\right) - z _ {S} \leq c (S), \quad S \in \mathcal {S} \\ & y _ {e} \geq 0, \quad e \in U \\ & z _ {S} \geq 0, \quad S \in \mathcal {S} \end{array}\tag{13.6}
$$

The algorithm is again greedy. Let us say that element e is alive if it occurs in fewer than $r _ { e }$ of the picked sets. In each iteration, the algorithm picks, from amongst the currently unpicked sets, the most cost-efective set, where the cost-efectiveness of a set is defined to be the average cost at which it covers alive elements. The algorithm halts when there are no more alive elements, i.e., each element has been covered to the extent of its requirement.

When a set $S$ is picked, its cost is distributed equally among the alive elements it covers as follows: if $S$ covers e for the jth time, we set $\mathrm { p r i c e } ( e , j )$ to the current cost-efectiveness of $S .$ . Clearly, the cost-efectiveness of sets picked is nondecreasing. Hence, for each element $e , \mathrm { p r i c e } ( e , 1 ) \leq \mathrm { p r i c e } ( e , 2 ) \leq$ $\ldots \leq \mathrm { p r i c e } ( e , r _ { e } )$

At the end of the algorithm, the dual variables are set as follows: For each $e \in U$ , let $\alpha _ { e } = ( 1 / H _ { n } )$ · price(e, r ). For each $S \in S$ that is picked by the algorithm, let

$$
\beta_ {S} = \frac {1}{H _ {n}} \cdot \left[ \sum_ {e \text {covered by} S} (\mathrm{price} (e, r _ {e}) - \mathrm{price} (e, j _ {e})) \right],
$$

where $j _ { e }$ is the copy of e that is covered by S. Notice that since price $( e , j _ { e } ) \le$ $\mathrm { p r i c e } ( e , r _ { e } ) , \ \beta _ { S }$ is nonnegative. If S is not picked by the algorithm, $\beta _ { S }$ is defined to be 0.

Lemma 13.5 The multicover picked by the algorithm is fully paid for by the dual solution $( \alpha , \beta )$

Proof: Since the cost of the sets picked by the algorithm is distributed among the covered elements, it follows that the total cost of the multicover produced by the algorithm is

$$
\sum_ {e \in U} \sum_ {j = 1} ^ {r _ {e}} \mathrm{price} (e, j).
$$

The objective function value of the dual solution $( \alpha , \beta )$ is

$$
\sum_ {e \in U} r _ {e} \alpha_ {e} - \sum_ {S \in \mathcal {S}} \beta_ {S} = \sum_ {e \in U} \sum_ {j = 1} ^ {r _ {e}} \operatorname{price} (e, j).
$$

The lemma follows.

The dual solution defined above is, in general, infeasible. We will show that when scaled by a factor of $H _ { n } , \mathrm { a }$ feasible solution results. Define for each element $e \in U$ and each set $S \in S$ ，

$$
y _ {e} = \frac {\alpha_ {e}}{H _ {n}} \mathrm{and} z _ {S} = \frac {\beta_ {S}}{H _ {n}}.
$$

Lemma 13.6 The pair $( y , z )$ is a feasible solution for the dual program $( 1 3 . 6 )$

Proof: Consider a set $S \in S$ consisting of k elements. Number its elements in the order in which their requirements are fulfilled, i.e., the order in which they stopped being alive. Let the ordered elements be $e _ { 1 } , \ldots , e _ { k }$

First, assume that S is not picked by the algorithm. When the algorithm is about to cover the last copy of $e _ { i } , S$ contains at least $k - i + 1$ alive elements, so

$$
\mathrm{price} (e _ {i}, r _ {e _ {i}}) \leq \frac {c (S)}{k - i + 1}.
$$

Since $z _ { S }$ is zero, we get

$$
\begin{array}{l} \left(\sum_ {i = 1} ^ {k} y _ {e _ {i}}\right) - z _ {S} = \frac {1}{H _ {n}} \sum_ {i = 1} ^ {k} \operatorname{price} (e _ {i}, r _ {e _ {i}}) \\ \qquad \leq \frac {c (S)}{H _ {n}} \cdot \left(\frac {1}{k} + \frac {1}{k - 1} + \dots + \frac {1}{1}\right) \leq c (S). \end{array}
$$

Next, assume that S is picked by the algorithm, and before this happens, $k ^ { \prime } \ge 0$ elements of $S$ are already completely covered. Then

$$
\begin{array}{l} \left(\sum_ {i = 1} ^ {k} y _ {e _ {i}}\right) - z _ {S} \\ \qquad = \frac {1}{H _ {n}} \cdot \left[ \sum_ {i = 1} ^ {k} \operatorname{price} (e _ {i}, r _ {e _ {i}}) - \sum_ {i = k ^ {\prime} + 1} ^ {k} (\operatorname{price} (e _ {i}, r _ {e _ {i}}) - \operatorname{price} (e _ {i}, j _ {i})) \right] \\ \qquad = \frac {1}{H _ {n}} \cdot \left[ \sum_ {i = 1} ^ {k ^ {\prime}} \operatorname{price} (e _ {i}, r _ {e _ {i}}) + \sum_ {i = k ^ {\prime} + 1} ^ {k} \operatorname{price} (e _ {i}, j _ {i}) \right], \end{array}
$$

where $S$ covers the $j _ { i }$ th copy of $e _ { i } ,$ for each $i \in \{ k ^ { \prime } + 1 , \ldots , k \}$

But $\begin{array} { r } { \sum _ { i = k ^ { \prime } + 1 } ^ { k } \operatorname { p r i c e } ( e _ { i } , j _ { i } ) = \cot ( S ) } \end{array}$ , since the cost of $S$ is equally distributed among the copies it covers. Finally consider elements $e _ { i } , i \in \{ 1 , \ldots , k ^ { \prime } \}$ . When the last copy of $e _ { i }$ is being covered, S is not yet picked and covers at least $k - i + 1$ alive elements. Thus, price $( e _ { i } , r _ { e _ { i } } ) \le c ( S ) / ( k - i + 1 )$ . Therefore,

$$
\left(\sum_ {i = 1} ^ {k} y _ {e _ {i}}\right) - z _ {S} \leq \frac {c (S)}{H _ {n}} \cdot \left(\frac {1}{k} + \dots + \frac {1}{k - k ^ {\prime} + 1} + 1\right) \leq c (S).
$$

Hence, $( y , z )$ is feasible for the dual program.

Theorem 13.7 The greedy algorithm achieves an approximation guarantee of $H _ { n }$ for the constrained set multicover problem.

Proof: By Lemmas 13.5 and 13.6, the total cost of the multicover produced by the algorithm is

$$
\sum_ {e \in U} r _ {e} \alpha_ {e} - \sum_ {S \in \mathcal {S}} \beta_ {S} = H _ {n} \cdot \left[ \sum_ {e \in U} r _ {e} y _ {e} - \sum_ {S \in \mathcal {S}} z _ {S} \right] \leq H _ {n} \cdot \mathrm{OPT}.
$$

Observe that as a corollary of Theorem 13.7 we get that the integrality gap of LP (13.5) is bounded by $H _ { n }$ . In contrast, the integrality gap of the corresponding LP for multiset multicover, with the restriction that each set be picked at most once, is not bounded by any function of n (see Exercise 13.5).

## 13.3 Exercises

13.1 Show that the dual-fitting-based analysis for the greedy set cover and constrained set multicover algorithms actually establishes an approximation guarantee of $H _ { k }$ , where k is size of the largest set in the given instance. (Notice the ease with which this can be established using the LP-duality approach; compare with Exercise 2.8.)

13.2 Give an example in which the dual solution, price(e), for each element $e ,$ computed by Algorithm 2.2 overpacks some sets, S, by a factor of essentially $H _ { | S | }$

13.3 Give examples to show that the lower bound used by Algorithm 2.2, $^ { \mathbf { \psi } } { \mathbf { \psi } } _ { \mathbf { { y } } } ,$ can be smaller than OPT by a factor of O(log n).

## 13.4 Give the following approximation algorithms.

1. $H _ { n }$ factor for set multicover.

2. $H _ { m }$ factor for multiset multicover, where m is the size of the largest multiset in the given instance (the size of a multiset counts elements with multiplicity).

3. O(log n) factor for covering integer programs.

Hint: For $H _ { m }$ factor algorithm for multiset multicover, set the dual variables according to the average price for covering elements, i.e.,

$$
y _ {e} = \frac {1}{H _ {m}} \sum_ {i = 1} ^ {r _ {e}} \mathrm{price} (e, i) / r _ {e}.
$$

Use scaling and rounding to reduce covering integer programs to multiset multicover, with m polynomially bounded in $n ,$ at the expense of a small error (which goes into the approximation factor).

13.5 Show that the integrality gap of the relaxation for the following two variants of multiset multicover, based on LP (13.2), is not bounded by any function of $n .$ .

1. Remove the restriction that $M ( S , e ) \subseteq r _ { e }$

2. Impose the constraint that each set can be picked at most once.

What is the best approximation guarantee you can establish for the greedy algorithm for the second variant. Why does the proof of factor $H _ { n }$ given in Section 13.2 not extend to this case?

13.6 (Mihail [206]) Consider the following variant on the set multicover problem. Let U be the universal set, $| U | = n ,$ and S a collection of subsets of U. For each $S \in S$ , its cost is given as a function of time, $t \in \{ 1 , \ldots , T \}$ Each of these cost functions is nonincreasing with time. In addition, for each element in $U ,$ a coverage requirement is specified, again as a function of time; these functions are nondecreasing with time. The problem is to pick sets at a minimum total cost so that the coverage requirements are satisfied for each element at each time. A set can be picked any number of times; the cost of picking a set depends on the time at which it is picked. Once picked, the set remains in the cover for all future times at no additional cost. Give an $H _ { n }$ factor algorithm for this problem. (An $H _ { ( n \cdot T ) }$ factor algorithm is straightforward.)

13.7 In many realistic situations, the cost of picking an item a multiple number of times does not grow linearly. Instead it is given by a concave function. The following variant of the set multicover problem models this situation. For each set $S _ { i }$ we are given a concave function $f _ { i }$ specifying the cost of picking this set multiple times. The problem again is to satisfy all coverage requirements of elements at minimum cost. Give a factor $H _ { n }$ algorithm for this problem.

Hint: Reduce the problem to a multiset multicover problem. For each set $S _ { i } .$ construct sets $S _ { i } ^ { j } , j \geq 1$ . Set $S _ { i } ^ { j }$ contains each element of $S _ { i }$ with multiplicity $j$ and has a cost of $f _ { i } ( j )$ . The greedy algorithm run on this instance achieves the required factor. Next show that there is no need to explicitly construct all the sets $S _ { i } ^ { j }$ . In each iteration of the greedy algorithm, the most cost-efective set can be computed directly in polynomial time, even if the requirements are exponentially large.

## 13.4 Notes

The dual-fitting-based analysis of set cover is due to Lov´asz [192] and Chv´atal [48]. The analysis of constrained set multicover is due to Rajagopalan and Vazirani [227]. For algorithms for covering integer programs, see Dobson [61] and Rajagopalan and Vazirani [227].