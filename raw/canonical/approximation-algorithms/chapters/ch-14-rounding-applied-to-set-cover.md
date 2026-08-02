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

# Rounding Applied to Set Cover

14 Rounding Applied to Set Cover




We will introduce the technique of LP-rounding by using it to design two
approximation algorithms for the set cover problem, Problem 2.1. The ﬁrst
is a simple rounding algorithm achieving a guarantee of f , where f is the
frequency of the most frequent element. The second algorithm, achieving an
approximation guarantee of O(log n), illustrates the use of randomization in
rounding.
    Consider the polyhedron deﬁned by feasible solutions to an LP-relaxation.
For some problems, one can ﬁnd special properties of extreme point solutions
of this polyhedron, which can yield rounding-based algorithms. One such
property is half-integrality, i.e., in each extreme point solution, every coordi-
nate is 0, 1, or 1/2. In Section 14.3 we will show that the vertex cover problem
possesses this remarkable property. This directly gives a factor 2 algorithm
for weighted vertex cover; namely, ﬁnd an optimal extreme point solution and
round all the halves to 1. A more general property, together with an enhanced
rounding algorithm, called iterated rounding, is introduced in Chapter 23.


14.1 A simple rounding algorithm
A linear programming relaxation for the set cover problem is given in
LP(13.2). One way of converting a solution to this linear program into an
integral solution is to round up all nonzero variables to 1. It is easy to con-
struct examples showing that this could increase the cost by a factor of Ω(n)
(see Example 14.3). However, this simple algorithm does achieve the desired
approximation guarantee of f (see Exercise 14.1). Let us consider a slight
modiﬁcation of this algorithm that is easier to prove and picks fewer sets in
general:


 Algorithm 14.1 (Set cover via LP-rounding)
  1. Find an optimal solution to the LP-relaxation.
  2. Pick all sets S for which xS ≥ 1/f in this solution.
120     14   Rounding Applied to Set Cover

Theorem 14.2 Algorithm 14.1 achieves an approximation factor of f for
the set cover problem.

Proof: Let C be the collection of picked sets. Consider an arbitrary element
e. Since e is in at most f sets, one of these sets must be picked to the extent
of at least 1/f in the fractional cover. Thus, e is covered by C, and hence C is
a valid set cover. The rounding process increases xS , for each set S ∈ C, by
a factor of at most f . Therefore, the cost of C is at most f times the cost of
the fractional cover, thereby proving the desired approximation guarantee. ✷
    The set cover instance arising from a vertex cover problem has f = 2.
Therefore, Algorithm 14.1 gives a factor 2 approximation algorithm for the
weighted vertex cover problem, thus matching the approximation guarantee
established in Theorem 2.7.
Example 14.3 Let us give a tight example for Algorithm 14.1. For sim-
plicity, we will view a set cover instance as a hypergraph: sets correspond to
vertices and elements correspond to hyperedges (this is a generalization of
the transformation that helped us view a set cover instance with each element
having frequency 2 as a vertex cover instance).
    Let V1 , . . . , Vk be k disjoint sets of cardinality n each. The hypergraph has
vertex set V = V1 ∪ . . . ∪ Vk , and nk hyperedges; each hyperedge picks one
vertex from each Vi . In the set cover instance, elements correspond to hyper-
edges and sets correspond to vertices. Once again, inclusion corresponds to
incidence. Each set has cost 1. Picking each set to the extent of 1/k gives an
optimal fractional cover of cost n. Given this fractional solution, the rounding
algorithm will pick all nk sets. On the other hand, picking all sets correspond-
ing to vertices in V1 gives a set cover of cost n.                                ✷



14.2 Randomized rounding
A natural idea for rounding an optimal fractional solution is to view the frac-
tions as probabilities, ﬂip coins with these biases and round accordingly. Let
us show how this idea leads to an O(log n) factor randomized approximation
algorithm for the set cover problem.
    First, we will show that each element is covered with constant probability
by the sets picked by this process. Repeating this process O(log n) times, and
picking a set if it is chosen in any of the iterations, we get a set cover with high
probability, by a standard coupon collector argument. The expected cost of
cover picked in this manner is O(log n)·OPTf ≤ O(log n)·OPT, where OPTf
is the cost of an optimal solution to the LP-relaxation. Applying Markov’s
Inequality, we convert this into a high probability statement. We provide
details below.
                                                  14.2   Randomized rounding          121

   Let x = p be an optimal solution to the linear program. For each set
S ∈ S, pick S with probability pS , the entry corresponding to S in p. Let C
be the collection of sets picked. The expected cost of C,
                                                      
       E[cost(C)] =         Pr[S is picked] · cS =           pS · cS = OPTf .
                      S∈S                              S∈S


    Next, let us compute the probability that an element a ∈ U is covered
by C. Suppose that a occurs in k sets of S. Let the probabilities associated
with these sets be p1 , . . . , pk . Since a is fractionally covered in the optimal
solution, p1 + p2 + · · · + pk ≥ 1. Using elementary calculus, it is easy to show
that under this condition, the probability that a is covered by C is minimized
when each of the pi ’s is 1/k. Thus,
                                                 k
                                       1                  1
       Pr[a is covered by C] ≥ 1 − 1 −                 ≥1− ,
                                       k                  e

where e is the base of natural logarithms. Hence each element is covered with
constant probability by C.
   To get a complete set cover, independently pick c log n such subcollections,
and compute their union, say C  , where c is a constant such that
        c log n
        1            1
                  ≤    .
        e           4n

Now,
                                         c log n
                                         1            1
       Pr[a is not covered by C  ] ≤              ≤    .
                                         e           4n

Summing over all elements a ∈ U , we get

                                                   1  1
       Pr[C  is not a valid set cover] ≤ n ·        ≤ .
                                                  4n  4

   Clearly, E[C  ] ≤ OPTf ·c log n. Applying Markov’s Inequality (see Section
B.2) with t = OPTf · 4c log n, we get

                                            1
       Pr[cost(C  ) ≥ OPTf · 4c log n] ≤     .
                                            4

The probability of the union of the two undesirable events is ≤ 1/2. Hence,

                                                                                1
       Pr[C  is a valid set cover and has cost ≤ OPTf · 4c log n] ≥              .
                                                                                2
122    14   Rounding Applied to Set Cover

Observe that we can verify in polynomial time whether C  satisﬁes both these
conditions. If not, we repeat the entire algorithm. The expected number of
repetitions needed at most 2.


14.3 Half-integrality of vertex cover
Consider the vertex cover problem with arbitrary weights. Let c : V → Q+
be the function assigning nonnegative weights to the vertices. The integer
program for this problem is:
                    
      minimize           c(v)xv                                           (14.1)
                   v∈V

      subject to   xu + xv ≥ 1,      (u, v) ∈ E
                   xv ∈ {0, 1},      v∈V

The LP-relaxation of this integer program is:
                    
      minimize           c(v)xv                                           (14.2)
                   v∈V

      subject to   xu + xv ≥ 1,      (u, v) ∈ E
                   xv ≥ 0,           v∈V

    Recall that an extreme point solution of a set of linear inequalities is a
feasible solution that cannot be expressed as convex combination of two other
feasible solutions. A half-integral solution to LP (14.2) is a feasible solution
in which each variable is 0, 1, or 1/2.

Lemma 14.4 Let x be a feasible solution to LP (14.2) that is not half-
integral. Then, x is the convex combination of two feasible solutions and is
therefore not an extreme point solution for the set of inequalities in LP (14.2).

Proof: Consider the set of vertices for which solution x does not assign
half-integral values. Partition this set as follows.
                &                              &        
                &1                              &      1
                &
      V+ = v & < xv < 1 ,                       &
                                     V− = v & 0 < xv <     .
                  2                                    2

For ε > 0, deﬁne the following two solutions.
                                             
               xv + ε, xv ∈ V+                xv − ε, xv ∈ V+
          yv = xv − ε, xv ∈ V− ,        zv = xv + ε, xv ∈ V−
                                             
                     xv , otherwise                 xv , otherwise.
                                                        14.4   Exercises   123

    By assumption, V+ ∪ V− = ∅, and so x is distinct from y and z. Fur-
thermore, x is a convex combination of y and z, since x = 12 (y + z). We
will show, by choosing ε > 0 small enough, that y and z are both feasible
solutions for LP (14.2), thereby establishing the lemma.
    Ensuring that all coordinates of y and z are nonnegative is easy. Next,
consider the edge constraints. Suppose xu + xv > 1. Clearly, by choosing ε
small enough, we can ensure that y and z do not violate the constraint for
such an edge. Finally, consider an edge such that xu + xv = 1. There are
essentially three possibilities for xu and xv . xu = xv = 12 ; xu = 0, xv = 1;
and u ∈ V+ , v ∈ V− . In all three cases, for any choice of ε,

     xu + xv = yu + yv = zu + zv = 1.

The lemma follows.                                                           ✷
   This leads to:
Theorem 14.5 Any extreme point solution for the set of inequalities in
LP (14.2) is half-integral.
   Theorem 14.5 directly leads to a factor 2 approximation algorithm for
weighted vertex cover: ﬁnd an extreme point solution, and pick all vertices
that are set to half or one in this solution.


14.4 Exercises

14.1 Modify Algorithm 14.1 so that it picks all sets that are nonzero in the
fractional solution. Show that the algorithm also achieves a factor of f .
Hint: Use the primal complementary slackness conditions to prove this.

14.2 Consider the collection of sets, C, picked by the randomized rounding
algorithm. Show that with some constant probability, C covers at least half
the elements at a cost of at most O(OPT).

14.3 Give O(log n) factor randomized rounding algorithms for the set mul-
ticover and multiset multicover problems (see Section 13.2).

14.4 Give a (non-bipartite) tight example for the half-integrality-based al-
gorithm for weighted vertex cover.

14.5 (J. Cheriyan) Give a polynomial time algorithm for the following prob-
lem. Given a graph G with nonnegative vertex weights and a valid, though not
necessarily optimal, coloring of G, ﬁnd a vertex cover of weight ≤ (2− k2 )OPT,
where k is the number of colors used.
124     14   Rounding Applied to Set Cover

14.6 Give a counterexample to the following claim. A set cover instance in
which each element is in exactly f sets has a (1/f )-integral optimal fractional
solution (i.e., in which each set is picked an integral multiple of 1/f ).

14.7 This exercise develops a combinatorial algorithm for ﬁnding an optimal
half integral vertex cover. Given undirected graph G = (V, E) and a non-
negative cost function c on vertices, obtain bipartite graph H(V  , V  , E  ) as
follows. Corresponding to each vertex v ∈ V , there is a vertex v  ∈ V  and
v  ∈ V  each of cost c(v)/2. Corresponding to each edge (u, v) ∈ E, there
are two edges (u , v  ), (u , v  ) ∈ E  . Show that a vertex cover in H can be
mapped to a half-integral vertex cover in G preserving total cost and vice
versa. Use the fact that an optimal vertex cover in a bipartite graph can be
found in polynomial time to obtain an optimal half-integral vertex cover in
G.

14.8 Consider LP (12.8), introduced in Exercise 12.7, for a non-bipartite
graph G = (V, E).
 1. Show that it is not an exact relaxation for the maximum matching prob-
    lem in G.
 2. Show that this LP always has a half-integral optimal solution.

14.9 In an attempt to improve the running time of the algorithm obtained in
Exercise 9.7 for bin packing, consider going to the LP-relaxation of the integer
programming and using LP-rounding. What guarantee can you establish for
bin packing through this method?


14.5 Notes
Algorithm 14.1 is due to Hochbaum [125]. For a more sophisticated random-
ized rounding algorithm for set cover, see Srinivasan [244]. Theorem 14.5 is
due to Nemhauser and Trotter [213].
