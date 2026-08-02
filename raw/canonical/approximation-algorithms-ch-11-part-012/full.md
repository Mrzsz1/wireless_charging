---
title: "approximation-algorithms-ch-11-part-012"
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
pdf_path: "work/core-books/approximation-algorithms/parts/approximation-algorithms-ch-11-part-012.pdf"
raw_md: "raw/canonical/approximation-algorithms-ch-11-part-012/full.md"
---
## 11 Euclidean TSP

In this chapter, we will give a PTAS for the special case of the traveling salesman problem in which the points are given in a d-dimensional Euclidean space. As before, the central idea of the PTAS is to define a “coarse solution”, depending on the error parameter $\varepsilon$ , and to find it using dynamic programming. A feature this time is that we do not know a deterministic way of specifying the coarse solution – it is specified probabilistically.

Problem 11.1 (Euclidean TSP) For fixed d, given n points in $R^{d}$ , the problem is to find the minimum length tour of the n points. The distance between any two points x and y is defined to be the Euclidean distance between them, i.e., $\left(\sum_{i=1}^{d}(x_{i}-y_{i})^{2}\right)^{1/2}$ .

## 11.1 The algorithm

We will give the algorithm for points on the plane, i.e., d = 2. The extension to arbitrary d is straightforward. The algorithm involves numerous details. In the interest of highlighting the main ideas, some of these details will be left as exercises.

Define the bounding box of the instance to be the smallest axis-parallel square that contains all n points. Via a simple perturbation of the instance, we may assume that the length of this square, L, is $4n^{2}$ and that there is a unit grid defined on the square such that each point lies on a gridpoint (see Exercise 11.1). Further, assume w.l.o.g. that n is a power of 2, and let $L = 2^{k}, k = 2 + \log_{2} n$ .

The basic dissection of the bounding box is a recursive partitioning into smaller squares. Thus, the $L \times L$ square is divided into four $L/2 \times L/2$ squares, and so on. It will be convenient to view this dissection as a 4-ary tree, T, whose root is the bounding box. The four children of the root are the four $L/2 \times L/2$ squares, and so on. The nodes of T are assigned levels. The root is at level 0, its children at level 1, and so on. The squares represented by nodes get levels accordingly. Thus, squares at level i have dimensions $L/2^{i} \times L/2^{i}$ . The dissection is continued until we obtain unit squares. Clearly, T has depth $k = O(\log n)$ . By a useful square we mean a square represented by a node in T.

Next, let us define levels for the horizontal and vertical lines that accomplish the basic dissection (these are all the lines of the grid defined on the bounding box). The two lines that divide the bounding box into four squares have level 1. In general, the $2^{i}$ lines that divide the level i - 1 squares into level i squares each have level i. Therefore, a line of level i forms the edge of useful squares at levels $i, i + 1, \ldots$ , i.e., the largest useful square on it has dimensions $L/2^{i} \times L/2^{i}$ :

![](images/032b788ff9b6558223b653902e16681132def4af39ac6088b7e6e3965b0e065e.jpg)

Each line will have a special set of points called portals. The coarse solution we will be seeking is allowed to cross a line only at a portal. The portals on each line are equidistant points. On a line of level i, these points are $L/(2^{i}m)$ apart, where the parameter m is fixed to be a power of 2 in the range $[k/\varepsilon, 2k/\varepsilon]$ . Clearly, $m = O(\log n/\varepsilon)$ . Since the largest useful square on a level i line has dimensions $L/2^{i} \times L/2^{i}$ , each useful square has a total of at most 4m portals on its four sides and corners. We have chosen m to be a power of 2 so that a portal in a lower level square is a portal for all higher level squares it lies in.

We will say that a tour $\tau$ is well behaved w.r.t. the basic dissection if it is a tour on the n points and any subset of the portals. In addition, this tour is allowed to visit portals multiple times, but other than that it must be non-self-intersecting. The key structural fact to be established is that there is such a tour of length at most $(1+\varepsilon)\cdot\text{OPT}$ . This requires a probabilistic argument, and we will return to it. First let us show why a PTAS follows from this fact.

We will say that tour $\tau$ is well behaved w.r.t. the basic dissection and has limited crossings if it is well behaved w.r.t. the basic dissection, and furthermore, it visits each portal at most twice.

Lemma 11.2 Let tour $\tau$ be well behaved w.r.t. the basic dissection. Then there must be a tour that is well behaved with limited crossings, whose length is at most that of $\tau$ .

Proof: The basic reason is that removing self-intersections by “short-cutting” can only result in a shorter tour, since Euclidean distance satisfies the triangle inequality. If $\tau$ uses a portal on line l more than twice, we can keep “short-cutting” on the two sides of l until the portal is used at most twice. If this introduces additional self-intersections, they can also be removed. ☐

Lemma 11.3 The optimal well behaved tour w.r.t. the basic dissection, having limited crossings, can be computed in time $2^{O(m)} = n^{O(1/\varepsilon)}$ .

Proof: We will build a table, using dynamic programming, that contains, for each useful square, the cost of each valid visit. We will sketch the main ideas, leaving details as Exercise 11.2.

Let $\tau$ be the optimal tour we wish to find. Clearly, the total number of times $\tau$ can enter and exit a useful square, S, is at most 8m. The part of $\tau$ inside S is simply a set of at most 4m paths, each entering and exiting S at portals, and together covering all the points inside the square. Furthermore, the paths must be internally non-self-intersecting, i.e., two paths can intersect only at their entrance or exit points. This means that the pairing of entrance and exit points of the paths must form a balanced arrangement of parentheses.

![](images/9b8d2a478850a4e316f764434e6067e269abe3e22aa8da30250707c30c1cdba1.jpg)

![](images/00f06b10539fe5835f8fc1439b7fd116b61e1f6911459b7b94740821f78dd2fe.jpg)

Let us call such a listing of portals, together with their pairing as entrance and exit points, a valid visit.

The number of useful squares is clearly $\text{poly}(n)$ . Let us first show that the number of valid visits in a useful square is at most $n^{O(1/\varepsilon)}$ , thereby showing that the number of entries in the table is bounded by $n^{O(1/\varepsilon)}$ .

Consider a useful square S. Each of its portals is used 0, 1, or 2 times, a total of $3^{4m} = n^{O(1/\varepsilon)}$ possibilities. Of these, retain only those possibilities that involve an even number of portal usages. Consider one such possibility, and suppose that it uses 2r portals. Next, we need to consider all possible pairings of these portals that form a balanced arrangement of parentheses. The number of such arrangements is the rth Catalan number, and is bounded by $2^{2r} = n^{O(1/\varepsilon)}$ . Hence, the total number of valid visits in S is bounded by $n^{O(1/\varepsilon)}$ .

For each entry in the table, we need to compute the optimal length of this valid visit. The table is built up the decomposition tree, starting at its leaves. Consider a valid visit V in a square S. Let S be a level i square. We have already fixed the entrances and exits on the boundary of S. Square S has four children at level $i + 1$ , which have four sides internal to S, with a total of at most 4m more portals. Each of these portals is used 0, 1, or 2 times, giving rise again to $n^{O(1/\varepsilon)}$ possibilities. Consider one such possibility, and consider all its portal usages together with portal usages of a valid visit V. Obtain all possible valid pairings of these portals that are consistent with those of visit V. Again, using Catalan numbers, their number is bounded by $n^{O(1/\varepsilon)}$ . Each such pairing will give rise to valid visits in the four squares.

The cost of the optimal way of executing these valid visits in the four squares has already been computed. Compute their sum. The smallest of these sums is the optimal way of executing visit V in square S. □

## 11.2 Proof of correctness

For the proof of correctness, it suffices to show that there is a well behaved tour w.r.t. the basic dissection whose length is bounded by $(1 + \varepsilon)$ OPT. It turns out that this is not always the case (see Exercise 11.3). Instead, we will construct a larger family of dissections and will show that, for any placement of the n points, at least half these dissections have short well behaved tours with limited crossings. So, picking a random dissection from this set suffices.

Let us define $L^{2}$ different dissections of the bounding box, which are shifts of the basic dissection. Given integers a, b with $0 \leq a$ , b < L, the $(a, b)$ -shifted dissection is obtained by moving each vertical line from its original location x to $(a + x) \mod L$ , and moving each horizontal line from its original location y to $(b + y) \mod L$ . Thus, the middle lines of the shifted dissection are located at $(a + L/2) \mod L$ and $(b + L/2) \mod L$ , respectively.

![](images/c2c74964c44e38d36d20f24776686945e6c2705eb37794a39787bed86b4932f5.jpg)  
The entire bounding box is thought of as being “wrapped around”. Useful squares that extend beyond L in their x or y coordinates will thus be thought of as “wrapped around”, and will still be thought of as a single square. Of course, the positions of the given n points remains unchanged; only the dissection is shifted.

Let $\pi$ be the optimal tour, and $N(\pi)$ be the total number of times $\pi$ crosses horizontal and vertical grid lines. If $\pi$ uses a point at the intersection of two grid lines, then we will count it as two crossings. The following fact is left as Exercise 11.4.

Lemma 11.4 $N(\pi)\leq 2\cdot \mathrm{OPT}.$

Following is the central fact leading to the PTAS.

Theorem 11.5 Pick a and b uniformly at random from $[0,L)$ . Then, the expected increase in cost in making $\pi$ well behaved w.r.t. the $(a,b)$ -shifted dissection is bounded by $2\varepsilon\cdotOPT$ .

Proof: Given any dissection, consider the process of making $\pi$ well behaved w.r.t. it. This involves replacing a segment of $\pi$ that does not cross a line l at a portal by two segments so that the crossing is at the closest portal on l. The corresponding increase in the length of the tour is bounded by the interportal distance on line l.

Consider the expected increase in length due to one of the crossings of tour $\pi$ with a line. Let l be this line. l will be a level i line in the randomly picked dissection with probability $2^{i}/L$ . If l is a level i line, then the interportal distance on it is $L/(2^{i}m)$ . Thus, the expected increase in the length of the tour due to this crossing is at most

$$
\sum_ {i} \frac {L}{2 ^ {i} m} \frac {2 ^ {i}}{L} = \frac {k}{m} \leq \varepsilon ,
$$

where we have used the fact that m lies in $[k/\varepsilon, 2k/\varepsilon]$ . The theorem follows by summing over all $N(\pi)$ crossings and using Lemma 11.4. ☐

Remark 11.6 The ideas leading up to Theorem 11.5 can be summarized as follows. Since lower level lines have bigger useful squares incident at them, we had to place portals on them further apart to ensure that any useful square had at most $4m$ portals on it (thereby ensuring that dynamic programming could be carried out in polynomial time). But this enabled us to construct instances for which there was no short well behaved tour w.r.t. the basic dissection (Exercise 11.3). On the other hand, there are fewer lines having lower levels - Theorem 11.5 exploits this fact.

Now, using Markov's inequality we get:

Corollary 11.7 Pick a and b uniformly at random from $[0,L)$ . Then, the probability that there is a well behaved tour of length at most $4\varepsilon\cdot OPT$ w.r.t. the $(a,b)$ -shifted dissection is greater or equal to 1/2.

Notice that Lemma 11.2 holds in the setting of an $(a,b)$ -shifted dissection as well. The PTAS is now straightforward. Simply pick a random dissection, and find an optimal well behaved tour with limited crossings w.r.t. this dissection using the dynamic programming procedure of Lemma 11.3. Notice that the same procedure holds even for a shifted dissection. The algorithm can be derandomized by trying all possible shifts and outputting the shortest tour obtained. Thus, we get:

Theorem 11.8 There is a PTAS for the Euclidean TSP problem in $\mathbf{R}^2$ .

## 11.3 Exercises

11.1 Show that we may assume that the length of the bounding square can be taken to be $L = 4n^{2}$ and that there is a unit grid defined on the square such that each point lies on a gridpoint.

Hint: Since we started with the smallest axis-parallel bounding square, its length is a lower bound on OPT. Therefore, moving each point to a grid point can increase the length of the tour by at most $OPT/n^{2}$ .

11.2 Provide the missing details in the proof of Lemma 11.3.

11.3 Give an instance of the Euclidean TSP problem for which, w.r.t. the basic dissection, the process of making the optimal tour well behaved increases its length by a fixed constant factor.

Hint: Make the optimal tour cross the middle line of the dissection that has the largest interportal distance numerous times.

## 11.4 Prove Lemma 11.4.

Hint: Notice that the left-hand side simply measures the $\ell_{1}$ length of tour $\pi$ . The bound of $2\sqrt{2}\cdot OPT$ is easier to prove, since this applies to single edges as well. This bound suffices for the PTAS.

11.5 Extend the arguments given to obtain a PTAS for the Euclidean TSP problem in $\mathbf{R}^d$ .

11.6 Generalize the algorithm to norms other than the Euclidean norm.

11.7 (Arora [10]) Obtain a PTAS for the Euclidean Steiner tree problem. Given n points in $R^{d}$ , find the minimum length tree containing all n points and any other subset of points. The latter points are called Steiner. The distance between two points is assumed to be their Euclidean distance.

11.8 Consider the Euclidean Steiner tree problem in $R^{2}$ . Show that in any optimal Steiner tree each Steiner point has degree 3 and the three angles so formed are of $120^{\circ}$ each. (See Gauss' figures on cover for an illustration of this fact.)

## 11.4 Notes

The first PTAS for Euclidean TSP was given by Arora [9], following a PTAS for the planar graph TSP problem due to Grigni, Koutsoupias, and Papadimitriou [115]. Subsequently, Mitchell [207] independently obtained the same result. Later, Arora [10] went on to give an $n(\log n)^{O(1/\varepsilon)}$ algorithm for the problem for any fixed $d$ . For a PTAS with an improved running time see Rao and Smith [229]. This chapter is based on Arora [10] and Arora, Raghavan, and Rao [13].

Part II

LP-Based Algorithms