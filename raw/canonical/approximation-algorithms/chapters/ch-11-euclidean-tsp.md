---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-11"
chapter_number: 11
chapter_title: "Euclidean TSP"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 102
source_page_end: 110
printed_page_start: 84
printed_page_end: 92
part_ids: ["approximation-algorithms-ch-11-part-012"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Euclidean TSP

11 Euclidean TSP




In this chapter, we will give a PTAS for the special case of the traveling
salesman problem in which the points are given in a d-dimensional Euclidean
space. As before, the central idea of the PTAS is to deﬁne a “coarse solution”,
depending on the error parameter ε, and to ﬁnd it using dynamic program-
ming. A feature this time is that we do not know a deterministic way of
specifying the coarse solution – it is speciﬁed probabilistically.
Problem 11.1 (Euclidean TSP) For ﬁxed d, given n points in Rd , the
problem is to ﬁnd the minimum length tour of the n points. The distance
between any two points x and y is deﬁned to be the Euclidean distance
                                    1/2
                      d             2
between them, i.e.,   i=1 (xi − yi )       .


11.1 The algorithm
We will give the algorithm for points on the plane, i.e., d = 2. The extension
to arbitrary d is straightforward. The algorithm involves numerous details.
In the interest of highlighting the main ideas, some of these details will be
left as exercises.
    Deﬁne the bounding box of the instance to be the smallest axis-parallel
square that contains all n points. Via a simple perturbation of the instance,
we may assume that the length of this square, L, is 4n2 and that there is
a unit grid deﬁned on the square such that each point lies on a gridpoint
(see Exercise 11.1). Further, assume w.l.o.g. that n is a power of 2, and let
L = 2k , k = 2 + log2 n.
    The basic dissection of the bounding box is a recursive partitioning into
smaller squares. Thus, the L×L square is divided into four L/2×L/2 squares,
and so on. It will be convenient to view this dissection as a 4-ary tree, T ,
whose root is the bounding box. The four children of the root are the four
L/2 × L/2 squares, and so on. The nodes of T are assigned levels. The root is
at level 0, its children at level 1, and so on. The squares represented by nodes
get levels accordingly. Thus, squares at level i have dimensions L/2i × L/2i .
The dissection is continued until we obtain unit squares. Clearly, T has depth
k = O(log n). By a useful square we mean a square represented by a node in
T.
                                                      11.1   The algorithm       85

    Next, let us deﬁne levels for the horizontal and vertical lines that accom-
plish the basic dissection (these are all the lines of the grid deﬁned on the
bounding box). The two lines that divide the bounding box into four squares
have level 1. In general, the 2i lines that divide the level i − 1 squares into
level i squares each have level i. Therefore, a line of level i forms the edge of
useful squares at levels i, i + 1, . . . , i.e., the largest useful square on it has
dimensions L/2i × L/2i :




                                                        Level 1 line


                                                        Level 2 line

                                                        Level 3 line



    Each line will have a special set of points called portals. The coarse solution
we will be seeking is allowed to cross a line only at a portal. The portals
on each line are equidistant points. On a line of level i, these points are
L/(2i m) apart, where the parameter m is ﬁxed to be a power of 2 in the
range [k/ε, 2k/ε]. Clearly, m = O(log n/ε). Since the largest useful square on
a level i line has dimensions L/2i × L/2i , each useful square has a total of
at most 4m portals on its four sides and corners. We have chosen m to be a
power of 2 so that a portal in a lower level square is a portal for all higher
level squares it lies in.
    We will say that a tour τ is well behaved w.r.t. the basic dissection if it
is a tour on the n points and any subset of the portals. In addition, this
tour is allowed to visit portals multiple times, but other than that it must be
non-self-intersecting. The key structural fact to be established is that there
is such a tour of length at most (1 + ε) · OPT. This requires a probabilistic
argument, and we will return to it. First let us show why a PTAS follows
from this fact.
    We will say that tour τ is well behaved w.r.t. the basic dissection and
has limited crossings if it is well behaved w.r.t. the basic dissection, and
furthermore, it visits each portal at most twice.
Lemma 11.2 Let tour τ be well behaved w.r.t. the basic dissection. Then
there must be a tour that is well behaved with limited crossings, whose length
is at most that of τ .

Proof: The basic reason is that removing self-intersections by “short-
cutting” can only result in a shorter tour, since Euclidean distance satisﬁes
86     11   Euclidean TSP

the triangle inequality. If τ uses a portal on line l more than twice, we can keep
“short-cutting” on the two sides of l until the portal is used at most twice.
If this introduces additional self-intersections, they can also be removed. ✷

Lemma 11.3 The optimal well behaved tour w.r.t. the basic dissection, hav-
ing limited crossings, can be computed in time 2O(m) = nO(1/ε) .

Proof: We will build a table, using dynamic programming, that contains,
for each useful square, the cost of each valid visit. We will sketch the main
ideas, leaving details as Exercise 11.2.
    Let τ be the optimal tour we wish to ﬁnd. Clearly, the total number of
times τ can enter and exit a useful square, S, is at most 8m. The part of τ
inside S is simply a set of at most 4m paths, each entering and exiting S at
portals, and together covering all the points inside the square. Furthermore,
the paths must be internally non-self-intersecting, i.e., two paths can intersect
only at their entrance or exit points. This means that the pairing of entrance
and exit points of the paths must form a balanced arrangement of parentheses.



        Invalid pairing                              Valid pairing
Let us call such a listing of portals, together with their pairing as entrance
and exit points, a valid visit.
    The number of useful squares is clearly poly(n). Let us ﬁrst show that the
number of valid visits in a useful square is at most nO(1/ε) , thereby showing
that the number of entries in the table is bounded by nO(1/ε) .
    Consider a useful square S. Each of its portals is used 0, 1, or 2 times, a
total of 34m = nO(1/ε) possibilities. Of these, retain only those possibilities
that involve an even number of portal usages. Consider one such possibility,
and suppose that it uses 2r portals. Next, we need to consider all possible
pairings of these portals that form a balanced arrangement of parentheses.
The number of such arrangements is the rth Catalan number, and is bounded
by 22r = nO(1/ε) . Hence, the total number of valid visits in S is bounded by
nO(1/ε) .
    For each entry in the table, we need to compute the optimal length of
this valid visit. The table is built up the decomposition tree, starting at its
leaves. Consider a valid visit V in a square S. Let S be a level i square. We
have already ﬁxed the entrances and exits on the boundary of S. Square S
has four children at level i + 1, which have four sides internal to S, with a
total of at most 4m more portals. Each of these portals is used 0, 1, or 2
times, giving rise again to nO(1/ε) possibilities. Consider one such possibility,
and consider all its portal usages together with portal usages of a valid visit
V . Obtain all possible valid pairings of these portals that are consistent with
those of visit V . Again, using Catalan numbers, their number is bounded by
nO(1/ε) . Each such pairing will give rise to valid visits in the four squares.
                                               11.2   Proof of correctness     87

The cost of the optimal way of executing these valid visits in the four squares
has already been computed. Compute their sum. The smallest of these sums
is the optimal way of executing visit V in square S.                         ✷



11.2 Proof of correctness
For the proof of correctness, it suﬃces to show that there is a well behaved
tour w.r.t. the basic dissection whose length is bounded by (1 + ε)OPT. It
turns out that this is not always the case (see Exercise 11.3). Instead, we will
construct a larger family of dissections and will show that, for any placement
of the n points, at least half these dissections have short well behaved tours
with limited crossings. So, picking a random dissection from this set suﬃces.
    Let us deﬁne L2 diﬀerent dissections of the bounding box, which are shifts
of the basic dissection. Given integers a, b with 0 ≤ a, b < L, the (a, b)-shifted
dissection is obtained by moving each vertical line from its original location x
to (a + x) mod L, and moving each horizontal line from its original location y
to (b + y) mod L. Thus, the middle lines of the shifted dissection are located

                           11111111111111
                           00000000000000
at (a + L/2) mod L and (b + L/2) mod L, respectively.

                           11111111111111
                           00000000000000
                           11111111111111
                           00000000000000
                           11111111111111
                           00000000000000
                           11111111111111
                           00000000000000
                          11111111
                          00000000
                           11111111111111
                           00000000000000
                          11111111
                          00000000
                       00000
                       11111
                                       0000
                                       1111
                                       1111
                                       0000
                       00000
                       11111           0000
                                       1111
                          11111111
                          00000000
                       11111
                       00000
                       11111
                       00000
                                       0000
                                       1111
                                       1111
                                       0000
                          11111111
                          00000000
                       11111
                       00000
                       00000
                       11111
                                       0000
                                       1111
                                       1111
                                       0000
                          11111111
                          00000000
                       11111
                       00000
                       11111
                       00000
                                       0000
                                       1111
                                       0000
                                       1111
                          11111111
                          00000000
                       11111
                       00000
                       11111
                       00000
                                       0000
                                       1111
                                       1111
                                       0000
                          00000000
                          11111111
                           00000000000000
                           11111111111111
                       11111
                       00000
                       11111
                       00000
                                       1111
                                       0000
                                       1111
                                       0000
                          00000000
                          11111111
                           00000000000000
                           11111111111111
                       00000
                       11111           1111
                                       0000
                     b
                           00000000000000
                           11111111111111
                           00000000000000
                           11111111111111
                               a
The entire bounding box is thought of as being “wrapped around”. Useful
squares that extend beyond L in their x or y coordinates will thus be thought
of as “wrapped around”, and will still be thought of as a single square. Of
course, the positions of the given n points remains unchanged; only the dis-
section is shifted.
    Let π be the optimal tour, and N (π) be the total number of times π
crosses horizontal and vertical grid lines. If π uses a point at the intersection
of two grid lines, then we will count it as two crossings. The following fact is
left as Exercise 11.4.
Lemma 11.4            N (π) ≤ 2 · OPT.
88      11   Euclidean TSP

     Following is the central fact leading to the PTAS.
Theorem 11.5 Pick a and b uniformly at random from [0, L). Then, the
expected increase in cost in making π well behaved w.r.t. the (a, b)-shifted
dissection is bounded by 2ε · OPT.

Proof: Given any dissection, consider the process of making π well behaved
w.r.t. it. This involves replacing a segment of π that does not cross a line l
at a portal by two segments so that the crossing is at the closest portal on
l. The corresponding increase in the length of the tour is bounded by the
interportal distance on line l.
    Consider the expected increase in length due to one of the crossings of tour
π with a line. Let l be this line. l will be a level i line in the randomly picked
dissection with probability 2i /L. If l is a level i line, then the interportal
distance on it is L/(2i m). Thus, the expected increase in the length of the
tour due to this crossing is at most
        L 2i   k
          i
              =   ≤ ε,
       i
         2mL    m

where we have used the fact that m lies in [k/ε, 2k/ε]. The theorem follows
by summing over all N (π) crossings and using Lemma 11.4.                 ✷

Remark 11.6 The ideas leading up to Theorem 11.5 can be summarized as
follows. Since lower level lines have bigger useful squares incident at them, we
had to place portals on them further apart to ensure that any useful square
had at most 4m portals on it (thereby ensuring that dynamic programming
could be carried out in polynomial time). But this enabled us to construct
instances for which there was no short well behaved tour w.r.t. the basic
dissection (Exercise 11.3). On the other hand, there are fewer lines having
lower levels – Theorem 11.5 exploits this fact.
    Now, using Markov’s inequality we get:
Corollary 11.7 Pick a and b uniformly at random from [0, L). Then, the
probability that there is a well behaved tour of length at most 4ε · OPT w.r.t.
the (a, b)-shifted dissection is greater or equal to 1/2.
    Notice that Lemma 11.2 holds in the setting of an (a, b)-shifted dissection
as well. The PTAS is now straightforward. Simply pick a random dissection,
and ﬁnd an optimal well behaved tour with limited crossings w.r.t. this dis-
section using the dynamic programming procedure of Lemma 11.3. Notice
that the same procedure holds even for a shifted dissection. The algorithm
can be derandomized by trying all possible shifts and outputting the shortest
tour obtained. Thus, we get:
Theorem 11.8 There is a PTAS for the Euclidean TSP problem in R2 .
                                                          11.4   Notes    89

11.3 Exercises

11.1 Show that we may assume that the length of the bounding square can
be taken to be L = 4n2 and that there is a unit grid deﬁned on the square
such that each point lies on a gridpoint.
Hint: Since we started with the smallest axis-parallel bounding square, its
length is a lower bound on OPT. Therefore, moving each point to a grid point
can increase the length of the tour by at most OPT/n2 .

11.2 Provide the missing details in the proof of Lemma 11.3.

11.3 Give an instance of the Euclidean TSP problem for which, w.r.t. the ba-
sic dissection, the process of making the optimal tour well behaved increases
its length by a ﬁxed constant factor.
Hint: Make the optimal tour cross the middle line of the dissection that has
the largest interportal distance numerous times.

11.4 Prove Lemma 11.4.
Hint: Notice that√the left-hand side simply measures the 1 length of tour
π. The bound of 2 2 · OPT is easier to prove, since this applies to single
edges as well. This bound suﬃces for the PTAS.

11.5 Extend the arguments given to obtain a PTAS for the Euclidean TSP
problem in Rd .

11.6 Generalize the algorithm to norms other than the Euclidean norm.

11.7 (Arora [10]) Obtain a PTAS for the Euclidean Steiner tree problem.
Given n points in Rd , ﬁnd the minimum length tree containing all n points
and any other subset of points. The latter points are called Steiner. The
distance between two points is assumed to be their Euclidean distance.

11.8 Consider the Euclidean Steiner tree problem in R2 . Show that in any
optimal Steiner tree each Steiner point has degree 3 and the three angles so
formed are of 120◦ each. (See Gauss’ ﬁgures on cover for an illustration of
this fact.)


11.4 Notes
The ﬁrst PTAS for Euclidean TSP was given by Arora [9], following a PTAS
for the planar graph TSP problem due to Grigni, Koutsoupias, and Papadim-
itriou [115]. Subsequently, Mitchell [207] independently obtained the same
result. Later, Arora [10] went on to give an n(log n)O(1/ε) algorithm for the
problem for any ﬁxed d. For a PTAS with an improved running time see Rao
and Smith [229]. This chapter is based on Arora [10] and Arora, Raghavan,
and Rao [13].
             Part II

LP-Based Algorithms
