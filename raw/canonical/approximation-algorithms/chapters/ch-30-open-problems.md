---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-30"
chapter_number: 30
chapter_title: "Open Problems"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 352
source_page_end: 396
printed_page_start: 334
printed_page_end: 378
part_ids: ["approximation-algorithms-ch-30-part-031"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Open Problems

30 Open Problems




This chapter is centered around problems and issues currently in vogue in
the ﬁeld of approximation algorithms. Important new issues are bound to
arise in the future. With each of these problems two questions arise – that of
obtaining the best approximation guarantee and a matching hardness result1


30.1 Problems having constant factor algorithms
Since a large number of important open problems in the ﬁeld today involve
improving the guarantee for problems for which we already know constant
factor algorithms, we found it convenient to present them in a separate sec-
tion. Of course, we are not looking for small improvements using incremental
means. A good model is Goemans and Williamson’s improvement to the
MAX-CUT problem, from factor 1/2 to 0.878, which introduced semideﬁ-
nite programming into the repertoire of techniques in this ﬁeld. Most of the
problems listed below have the potential of extending known methods in
signiﬁcant ways and introducing important new ideas.

Vertex cover, Problem 1.1: Improve on factor 2 (see algorithms in Chap-
   ters 1, 2, 14, and 15). Semideﬁnite programming may be a possible avenue,
   see, e.g., the attempt by Goemans and Kleinberg [103].
Set cover, Problem 2.1: This question generalizes the previous one. Con-
   sider the restriction of the set cover problem to instances in which the
   frequency of each element is bounded by a ﬁxed constant f . Improve on
   factor f (see algorithms in Chapters 2, 14, and 15). The best hardness
   result known is f 1/19 , assuming P = NP, due to Trevisan [247].
Acyclic subgraph, Problem 1.9: Improve on factor 1/2 (see Exercise
   1.1). Semideﬁnite programming may be applicable.
Metric TSP, Problem 3.5: As stated in Exercise 23.13, the solution pro-
  duced by Christoﬁdes’ algorithm (Algorithm 3.10) is within a factor of
1
    For an up-to-date status of the best positive and negative results known for
    numerous NP-hard optimization problems, see the excellent compendium at
    http://www.nada.kth.se/˜viggo/problemlist/compendium.html
                        30.1   Problems having constant factor algorithms     335

     3/2 of the subtour elimination LP-relaxation for this problem. However,
     the worst integrality gap example known is (essentially) 4/3. Can a 4/3
     factor algorithm be obtained using this relaxation?
     Christoﬁdes’ algorithm consists of two steps: obtaining an MST and
     patching up its odd degree vertices. The above stated result follows by
     bounding the cost of each of these steps individually. It might be a good
     idea to ﬁrst look for a “one–shot” factor 3/2 algorithm which compares
     the entire solution to the LP-relaxation. The primal–dual schema may
     hold the key.
Steiner tree, Problem 3.1: The best approximation guarantee known is
   essentially 5/3 (see Exercise 22.12). A promising avenue for obtaining
   an improved guarantee is to use the bidirected cut relaxation (22.7).
   This relaxation is exact for the minimum spanning tree problem. For the
   Steiner tree problem, the worst integrality gap known is (essentially) 8/7,
   due to Goemans (see Exercise 22.11). The best upper bound known on
   the integrality gap is 3/2 for quasi-bipartite graphs (graphs that do not
   contain edges connecting pairs of Steiner vertices), due to Rajagopalan
   and Vazirani [226]. Determine the integrality gap of this relaxation and
   obtain an algorithm achieving this guarantee2 .
   Recall that in contrast, LP-relaxation (22.2) has an integrality gap of
   (essentially) 2, not only for this problem, but also for its special case,
   the minimum spanning tree problem, and its generalization, the Steiner
   network problem.
Steiner network, Problem 23.1: Chapter 23 gives a factor 2 algorithm.
   However, it uses LP-rounding and has a prohibitive running time. Obtain
   a factor 2 combinatorial algorithm for this problem. A corollary of Algo-
   rithm 23.7 is that the integrality gap of LP-relaxation (23.2) is bounded
   by 2. Therefore, this relaxation can be used as a lower bound for obtain-
   ing a factor 2 combinatorial algorithm. The primal–dual schema appears
   to be the most promising avenue. A starting point may be determining
   if the following is true:
   For each instance of the Steiner forest problem (and more generally, the
   Steiner network problem) there is an integral primal solution x and dual
   feasible solution y such that each edge picked by x is tight w.r.t. the dual
   y and each raised dual S has degree ≤ 2 (≤ 2f (S)). Observe that the
   dual found by Algorithm 22.3 can have arbitrarily high degree.
Multiway cut, Problem 4.1: A 1.5 factor is presented in Chapter 19. As
  stated, this can be improved to 1.3438. However, the worst integrality gap
  example known for LP-relaxation (19.1) is (essentially) 8/7. Determine
  the integrality gap of this relaxation, and obtain an algorithm achieving
2
    A more general issue along these lines is to clarify the mysterious connection
    between the integrality gap of an LP-relaxation and the approximation factor
    achievable using it.
336      30   Open Problems

      this guarantee. A diﬀerent relaxation is presented in Exercise 19.7. How
      are the two relaxations related? Are they equivalent in that any feasible
      solution to one be converted to a solution of the other of the same cost?
Subset feedback vertex set, Problem 19.15: The best factor known is
   8, via a fairly complicated algorithm (see Exercise 19.13). Is a factor 2
   algorithm possible, matching several of the other related problems stated
   in Exercise 19.13?


30.2 Other optimization problems
Shortest vector, Problem 27.1: Obtain a polynomial factor algorithm
   for this problem. As shown in Chapter 27, the dual lattice helps give
   a factor n co-NP certiﬁcate for this problem. Is the dual lattice of fur-
   ther algorithmic
            √        use? The best hardness result known for this problem,
   of factor 2 − ε, for any ε > 0, assuming RP = NP, is due to Micciancio
   [204].
Sparsest cut, Problem 21.2: The best approximation factor known is
   O(log n) (see Chapter 21). However, no hardness of approximation re-
   sults have been established for this problem – as far as we know a PTAS
   not yet ruled out. Is there a constant factor algorithm or a PTAS for this
   problem?
Minimum b-balanced cut and minimum bisection cut, Problem
  21.27: An O(log2 n) factor algorithm for both these problems was given
  by Feige and Krauthgamer [83]. As in the case of sparsest cut, a PTAS
  is not yet ruled out for these problems. Is there a constant factor algo-
  rithm or a PTAS for these problems? When restricted to planar graphs,
  the minimum b-balanced cut problem, for b ≤ 1/3, can be approximated
  within a factor of 2, see Garg, Saran, and Vazirani [95].
Minimum multicut, 18.1: An O(log n) factor algorithm is given in Chap-
  ter 20. A long standing open problem is whether there is a constant factor
  deterministic algorithm for this problem.
Asymmetric TSP, Problem 3.15: The best factor known is O(log n) (see
   Exercise 3.6). Is there a constant factor algorithm for this problem?
Vertex-connectivity network design: This variant of the Steiner net-
   work problem (Problem 23.1) asks for a minimum cost subgraph con-
   taining ru,v vertex-disjoint paths, instead of edge-disjoint paths, for each
   pair of vertices u, v ∈ V . No nontrivial approximation algorithms are
   known for this variant. For the special case when ru,v = k for each pair
   of vertices u, v ∈ V and the edge costs obey the triangle inequality, a
   (2 + 2(k−1)
           n   ) factor algorithm is given by Khuller and Raghavachari [170].
   A problem of intermediate diﬃculty is the element-connectivity network
                                    30.2   Other optimization problems   337

   design problem, in which vertices are partitioned into two sets: terminals
   and non-terminals. Only edges and non-terminals, referred to as elements,
   can fail. Only pairs of terminals have connectivity requirements, specify-
   ing the number of element-disjoint paths required. An algorithm with an
   approximation guarantee of factor 2Hk , where k is the largest require-
   ment, is given by Jain, Măndoiu, Vazirani, and Williamson [139].
Maximum integer multicommodity ﬂow, Problem 18.3: Example
  18.8 shows that the natural LP-relaxation has an integrality gap of Ω(n).
  It is easy to get around this diﬃculty while still retaining the essence of
  the original problem by asking for a maximum half-integral ﬂow. Is there
  an O(log n) factor algorithm for this latter problem?
Metric uncapacitated facility location and k-median, Problems 24.1
  and 25.1: Determine the integrality gaps of the LP-relaxations (24.2)
  and (25.2).
Capacitated facility location problem, Exercise 24.8: As stated in Ex-
  ercise 24.8 the modiﬁcation of LP (24.2) to this problem has unbounded
  integrality gap. Is there some other lower bounding method that leads to
  a good approximation algorithm?
Directed multicut and sparsest cut: In Chapters 20 and 21 we consid-
   ered two generalizations of the undirected maximum ﬂow problem and
   derived approximation algorithms for the corresponding cut problems,
   multicut and sparsest cut. Not much is known at present about analo-
   gous problems in directed graphs.
Directed Steiner tree, Problem 3.14: As shown in Exercise 3.3 this prob-
   lem is unlikely to have a better approximation guarantee than O(log n). Is
   a guarantee of O(log n) possible? The best guarantee known is nε for any
   ﬁxed ε > 0, due to Charikar et. al. [37]. Generalizations of this problem
   to higher connectivity requirements, analogous to the Steiner network
   problem, also need to be studied.
Directed feedback edge (vertex) set: Given a directed graph G =
   (V, E), a feedback edge (vertex) set is a set of edges (vertices) whose
   removal leaves an acyclic graph. The problem is to ﬁnd the minimum
   cardinality such set. More generally, consider the weighted version in
   which the edges (vertices) have assigned weights, and we want to ﬁnd
   the minimum weight such set. It is easy to see that the edge and vertex
   versions are inter-reducible via approximation factor preserving reduc-
   tions. An O(log n log log n) factor approximation algorithm is known for
   the weighted version, due to Seymour [238]. Can this be improved to
   O(log n) or even a constant factor?
Cover time: Given an undirected graph G = (V, E), the cover time starting
   at vertex v ∈ V , C(v) is the expected number of steps taken by a random
338      30   Open Problems

      walk on G, which starts at v and visits all vertices. The cover time of
      G is maxv∈V C(v). Clearly, a randomized algorithm can estimate the
      cover time to any desired accuracy by empirically simulating the random
      walk many times and taking the average. Kahn, Kim, Lovász, and Vu
      [151] have given an O((log log n)2 ) factor deterministic algorithm for this
      problem. Is a constant factor deterministic algorithm possible?


30.3 Counting problems
For the problems presented below (other than graphs with given degree se-
quence and triangulations), the decision version is in P, the counting version
is #P-complete, and the complexity of approximately counting the number
of solutions is unresolved. The complexity of counting the number of graphs
with given degree sequence and triangulations is open, though conjectured
to be #P-complete.

Perfect matchings in general graphs: When restricted to planar graphs,
   this problem is polynomial time solvable using the classic algorithm of
   Kastelyn [168]. This result extends to K3,3 -free graphs (graphs that do
   not contain a subgraph homeomorphic to K3,3 ) as well, see Little [191]
   and Vazirani [251]. A FPRAS is known for the restriction of this problem
   to bipartite graphs, which is the same as the problem of evaluating a
   0/1 permanent, due to Jerrum, Sinclair, and Vigoda [143] (more gener-
   ally, this work gives a FPRAS for evaluating the permanent of a square
   matrix with nonnegative integer entries).
Volume of a convex body: Given a convex body in Rn via an oracle, the
   problem is to estimate its volume. A number of other counting problems
   can be reduced to this fundamental problem. The ﬁrst FPRAS for this
   problem was given by Dyer, Frieze, and Kannan [67]. Although poly-
   nomial, the running time of this algorithm was exorbitant. It required
   O∗ (n23 ) oracle calls – the “soft-O” notation of O∗ suppresses factors of
   log n as well as ε, the error bound. The current best algorithm, due to
   Kannan, Lovász, and Simonovits [155] requires O∗ (n5 ) oracle calls and
   O∗ (n7 ) arithmetic operations. Can the running time be further improved?
Acyclic orientations: Count the number of acyclic orientations of a given
   undirected graph G. An orientation of the edges of G is acyclic if the
   resulting directed graph is acyclic. Several Markov chains on the set of
   acyclic orientations are known that asymptotically converge to the uni-
   form distribution; however, none of them is known to be rapidly mixing.
   For instance, say that two orientations are adjacent if one can be obtained
   from the other by ﬂipping directions of the edges incident at a source or
   a sink, where a source has all outgoing edges and a sink has all incoming
   edges. Do a random walk on this graph.
                                               30.3   Counting problems     339

Forests: A forest in an undirected graph is a set of edges that contain no
   cycles. A maximal forest is a spanning tree (assume the graph is con-
   nected). Interestingly enough, the problem of counting the number of
   spanning trees in a graph is in P – this being one of the very few count-
   ing problems known to be polynomial time solvable. This follows as a
   consequence of the classic matrix tree theorem of Kirchhoﬀ, see [194]. It
   is worth remarking that elegant polynomial time algorithms are known for
   generating a random spanning tree in an undirected graph using rapidly
   mixing Markov chains, due to Aldous [3], Broder [34], and Wilson [259].
   On the other hand, the complexity of approximately counting forests in
   arbitrary graphs is open. The case of dense graphs (each vertex having
   degree at least αn, for 0 < α < 1) is handled by Annan [8]. Forests and
   spanning trees are the independent sets and bases, respectively, of the
   graphic matroid of the given graph.
Bases of a matroid: Given an arbitrary matroid via an independence or-
   acle, count the number of bases. Deﬁne the basis exchange graph of a
   matroid as follows. Its vertices are all bases. Two bases are adjacent iﬀ
   their symmetric diﬀerence is two elements. The Markov chain deﬁned by
   a random walk on the basis exchange graph is conjectured to be rapidly
   mixing by Dagum, Luby, Mihail, and Vazirani [56]. If so, a FPRAS for
   approximately counting the number of bases will follow. Examples of ma-
   troids for which this conjecture has been positively resolved are graphic
   matroids (see previous problem) and their generalization, balanced ma-
   troids. For the latter result, see Feder and Mihail [78]. A positive res-
   olution of this question will also resolve the question of approximately
   counting forests (since forests of any particular size are bases of a trun-
   cation of the graphic matroid).
Network reliability: Many versions of the network reliability problem have
   found practical applications and have been studied in the past. Two ba-
   sic versions for undirected graphs with edge failure probabilities are s–t
   reliability, which asks for the probability that special vertices s and t get
   disconnected, and global reliability, which asks for the probability that
   any part of the graph gets disconnected. One can deﬁne two analogous
   problems in directed graphs as well. Of these four problems, only undi-
   rected global reliability is settled – a FPRAS for this version is presented
   in Chapter 28. In addition, for each of these four cases one can also ask
   for the probability that s–t or the entire graph remain connected. This
   version is open even for the undirected global case.
Euler tours: Count the number of Euler tours of a given undirected graph
   (a connected graph is Eulerian iﬀ all vertices have even degrees). Inter-
   estingly enough, there is a polynomial time algorithm for the analogous
   problem for directed graphs – again following from Kirchhoﬀ’s Theorem.
340     30   Open Problems

Trees: Given an undirected graph G, count the number of subgraphs of G
   that are trees.
Antichains in a partial order: See Exercise 1.7 for the deﬁnition. For the
   related problem of counting the number of total orders consistent with a
   partial order, a FPRAS is known, due to Matthews [202], Karzanov and
   Khachian [167], and Bubley and Dyer [35].
Graphs with given degree sequence: Given n nonnegative integers d1 ,
   . . . , dn , which represent the degrees of the n vertices, v1 , . . . , vn , of a
   simple graph, count the number of such graphs. A related problem is to
   count the number of connected graphs having this degree sequence. In
   both cases, the question of existence of one such graph can be solved
   in polynomial time using a matching algorithm. If G is restricted to be
   a bipartite graph, with the bipartition speciﬁed, then a FPRAS follows
   from that for 0/1 permanents [143].
Contingency tables: Given the row sums and column sums of an m ×
  n matrix with nonnegative integer entries, count the number of such
  matrices. A FPRAS is known if the row sums and column sums are all
  suﬃciently large, being at least (m + n)mn, due to Dyer, Kannan, and
  Mount [64]. Morris [208] improves this to the case where each row sum is
  Ω(n3/2 m log m) and each column sum is Ω(m3/2 n log n). If the matrices
  are constrained to be 0/1, this is same as the degree sequence problem
  restricted to bipartite graphs, for which a FPRAS follows from that for
  0/1 permanents [143].
Triangulations: Compute the number of triangulations of n points on the
   plane, i.e., the number of ways of putting down non–intersecting line seg-
   ments connecting pairs of points so that all internal faces are triangles.
   Consider the graph G on all possible triangulations whose edges are de-
   ﬁned as follows: Remove an edge in a triangulation t that is not on the
   inﬁnite face. If the resulting quadrilateral is convex, let t be the trian-
   gulation obtained by adding an edge connecting the other two points of
   this quadrilateral. Then, G has an edge connecting t and t . A random
   walk on this graph is conjectured to be rapidly mixing. If the n points
   form the vertices of a convex n-gon, then the number of triangulations is
   known to be the Catalan number Cn−2 , and hence polynomial time com-
   putable. For this special case, the Markov chain deﬁned above is known
   to be rapidly mixing, see McShine and Tetali [203].
Stable marriages: An instance of the stable marriage problem consists of
   n boys and n girls, together with an ordered list of the preferences of each
   boy and each girl (each boy orders all n girls and each girl orders all n
   boys). A marriage is a perfect matching of the boys and girls. Boy b and
   girl g who are not married to each other are said to form a rogue couple
   if b prefers g to the girl he is married to and g prefers b to the boy she
                                                  30.3   Counting problems   341

   is married to. The marriage is stable if there are no rogue couples. The
   complexity of approximately counting the number of stable marriages
   is unresolved. For numerous structural properties of the set of stable
   marriages, see Gusﬁeld and Irving [119].
Colorings of a graph: Consider an undirected graph G = (V, E) with
   maximum degree ∆. Jerrum [144] gave a FPRAS for counting the number
   of valid k-colorings of G for any k > 2∆, and Vigoda [254] extended this
   to any k > 11∆/6. Can this be improved to counting the number of valid
   k-colorings of G for any k ≥ ∆ + 2? (If the number of colors is ≤ ∆ + 1
   then the natural Markov chain, that at each step picks a random vertex
   and recolors it with a random consistent color, may not be connected.)
   This quantity ﬁnds applications in statistical physics.
Hamiltonian cycles: If each vertex of an undirected graph G has degree at
  least n/2 then G must have a Hamiltonian cycles (see Dirac’s condition
  in [194]). If the minimum degree is (1/2+ε)n, for ε > 0, Dyer, Frieze, and
  Jerrum [65] have given a FPRAS for this problem. Can this be extended
  to ε = 0, i.e., graphs having minimum degree n/2?
Independent sets: For graphs having ∆ = 4, a FPRAS was given by Luby
   and Vigoda [198], where ∆ denotes the maximum degree of the graph.
   Dyer, Frieze, and Jerrum [66] show that the problem is not approximable
   for ∆ ≥ 25, assuming RP = NP. They also give an argument to show
   that the Markov chain Monte Carlo is unlikely to succeed for ∆ ≥ 6.
   Besides the question of ∆ = 5, this leaves the question of determining
   whether other methods will work for 6 ≤ ∆ ≤ 24 or whether these cases
   are also inapproximable.
Tutte polynomial: Several of the problems stated above are special cases
   of evaluating the Tutte polynomial of the given graph G = (V, E) at a
   particular point of the (x, y)-plane. For A ⊆ E, deﬁne the rank of A,
   denoted r(A), to be |V | − k(A), where k(A) is the number of connected
   components in the graph having vertex set V and edge set A. The Tutte
   polynomial of G at point (x, y) is
                         
         T (G; x, y) =         (x − 1)r(E)−r(A) (y − 1)|A|−r(A) .
                         A⊆E


   Some of the natural quantities captured by this polynomial are:
   • At (1, 1), T counts the number of spanning trees in G.
   • At (2, 1), T counts the number of forests in G.
   • At (1, 2), T counts the number of connected subgraphs of G.
   • At (2, 0), T counts the number of acyclic orientations of G.
   • At (0, 2), T counts the number of orientations of G that form a strongly
     connected digraph.
342      30     Open Problems

      • The chromatic polynomial of G is given by

                P (G, λ) = (−1)r(E) λk(E) T (G; 1 − λ, 0),

        where P (G, λ) is the number of colorings of G using λ colors.
      • If the failure probability of each edge is p, then the probability that G
        remains connected is given by

                R(G; p) = q |E|−r(E) pr(E) T (G; 1, 1/(1 − p)).

      Vertigan and Welsh [253] have shown that other than a few special points
      and two special hyperbolae (see next problem for deﬁnition), the exact
      evaluation of the Tutte polynomial is #P-hard. The question of designing
      FPRAS’s is wide open. Say that a graph is α-dense if each vertex has
      degree ≥ αn, where 0 < α < 1. Annan [7] and Alon, Frieze, and Welsh
      [5] have given FPRAS’s for α-dense graphs for the cases y = 1, x ≥ 1 and
      y > 1, x ≥ 1, respectively.
Partition functions of the Ising and Potts models: The hyperbolae
   Hα deﬁned by

              Hα = {(x, y) | (x − 1)(y − 1) = α}

      play a special role in the context of the Tutte polynomial. In particu-
      lar, along H2 , T gives the partition function of the Ising model for G,
      and along HQ , for integer Q ≥ 2, T gives the partition function of the
      Potts model for G. Both these quantities ﬁnd use in statistical physics;
      see Welsh [256] for precise deﬁnitions and further details (the points on
      each hyperbola are parametrized by “temperature” and Q represents the
      number of “color” classes). Jerrum and Sinclair [146] gave a FPRAS for
      estimating, at any temperature, the partition function of the Ising model
      of a graph, and Randall and Wilson [228] extended this to a polynomial
      time sampling procedure. However, because of large exponents in the
      running times, these algorithms are not practical. The Swendsen-Wang
      process [245] provides a natural and practically used Markov chain for
      estimating these quantities. This leads to the question of determining,
      formally, whether this chain is rapidly mixing. A negative result was pro-
      vided by Gore and Jerrum [112] who show that this chain is not rapidly
      mixing for the complete graph, Kn , for Q ≥ 3. Positive results for certain
      classes of graphs were provided by Cooper and Frieze [53]. Is this chain
      rapidly mixing for the partition function of the Ising model for an arbi-
      trary graph? Is there some other way of estimating the partition function
      of the Potts model for an arbitrary graph?
A An Overview of Complexity Theory
for the Algorithm Designer




A.1 Certiﬁcates and the class NP
A decision problem is one whose answer is either “yes” or “no”. Two examples
are:
SAT: Given a Boolean formula in conjunctive normal form, f , is there is a
   satisfying truth assignment for f ?
Cardinality vertex cover: Given an undirected graph G and integer k, does
   G have a vertex cover of size ≤ k?
    For any positive integer k, we will denote by kSAT the restriction of SAT
to instances in which each clause contains at most k literals.
    It will be convenient to view a decision problem as a language, i.e., a subset
of {0, 1}∗ . The language consists of all strings that encode “yes” instances of
the decision problem. A language L ∈ NP if there is a polynomial p and a
polynomial time bounded Turing machine M , called the veriﬁer, such that
for each string x ∈ {0, 1}∗ :
• if x ∈ L, then there is a string y (the certiﬁcate) of polynomially bounded
  length, i.e., |y| ≤ p(|x|), such that M (x, y) accepts, and
       / L, then for any string y, such that |y| ≤ p(|x|), M (x, y) rejects.
• if x ∈




                        M
                                                       Certiﬁcate
                                                      ❄
                                                     y




                          Input and work tape
                         ❄
                           x
344    An overview of complexity theory

     String y that helps ascertain that x is a “yes” instance will be called a
Yes certiﬁcate. We will also refer to y as a proof or a solution; in the context
of randomized computation, it is also referred to as a witness. Thus, NP is
the class of languages that have “short, quickly veriﬁable” Yes certiﬁcates.
     For example, the veriﬁer for cardinality vertex cover assumes that y spec-
iﬁes a subset of the vertices. It checks whether this subset is indeed a vertex
cover and is of the desired size bound. (Observe that no claim has been made
about the time needed to actually ﬁnd such a certiﬁcate.) It is also easy to see
that the class NP deﬁned above is precisely the class of languages that are
decidable by nondeterministic polynomial time Turing machines (see Section
A.6 for references), hence the name.
     A language L belongs to the class co-NP iﬀ L ∈ NP. Thus, co-NP is
the class of languages that have “short, quickly veriﬁable” No certiﬁcates. For
instance, let L be the language consisting of all prime numbers. This language
allows No certiﬁcates: a factorization for number n is proof that n ∈ / L. Hence
L ∈ co-NP. Interestingly enough, L ∈ NP as well (see Exercise 1.13), though
it is not known to belong to P.


A.2 Reductions and NP-completeness
Next, let us introduce the crucial notion of a polynomial time reduction. Let
L1 and L2 be two languages in NP. We will say that L1 reduces to L2 , and
write L1 4 L2 , if there is a polynomial time Turing machine T which given
a string x ∈ {0, 1}∗ , outputs string y such that x ∈ L1 iﬀ y ∈ L2 . In general,
T does not have to decide whether x is a “yes” or a “no” instance in order
to output y. Clearly, if L1 4 L2 and L2 is polynomial time decidable, then
so is L1 .
    A language L is NP-hard if for every language L ∈ NP, L 4 L. A
language L is NP-complete if L ∈ NP, and L is NP-hard. An NP-complete
language L is a hardest language in NP, in the sense that a polynomial time
algorithm for L implies a polynomial time algorithm for every language in
NP, i.e., it implies P = NP.
    The central theorem of complexity theory gives a proof of NP-hardness
for a natural problem, namely SAT. The idea of the proof is as follows. Let
L be an arbitrary language in NP. Let M be a nondeterministic polynomial
time Turing machine that decides L, and let p be the polynomial bounding the
running time of M . The proof involves showing that there is a deterministic
polynomial time Turing machine T , that “knows” M and p, and given a
string x ∈ {0, 1}∗ , outputs a SAT formula f such that each satisfying truth
assignment of f encodes an accepting computation of M on input x. Thus,
f is satisﬁable iﬀ there is an accepting computation of M on input x, i.e., iﬀ
x ∈ L.
    Once one problem, namely SAT, has been shown to be NP-hard, the
hardness of other natural problems can be established by simply giving poly-
          A.3   NP-optimization problems and approximation algorithms        345

nomial time reductions from SAT to these problems (see Exercise 1.11). Per-
haps the most impressive feature of the theory of NP-completeness is the
ease with which the latter task can be accomplished in most cases, so that
with relatively little work, a lot of crucial information is obtained. Other than
a handful of (important) problems, most natural problems occurring in NP
have been classiﬁed as being either in P or being NP-complete. Indeed, it
is remarkable to note that other basic complexity classes, deﬁned using no-
tions of time, space and nondeterminism, also tend to have natural complete
problems (under suitably deﬁned reducibilities).
    Establishing NP-hardness for vertex cover involves giving a polynomial
time algorithm that, given a SAT formula f , outputs an instance (G, k) such
that G has a vertex cover of size ≤ k iﬀ f is satisﬁable. As a corollary, we get
that under the assumption P = NP, there is no polynomial time algorithm
that can distinguish “yes” instances of vertex cover from “no” instances. As
stated above, this also shows that if P = NP, there is no polynomial time
algorithm for solving vertex cover exactly.
    Considering the large and very diverse collection of NP-complete prob-
lems, none of which has yielded to a polynomial time algorithm for so many
years, it is widely believed that P = NP, i.e., that there is no polynomial
time algorithm for deciding an NP-complete language.
    The P = NP conjecture has a deep philosophical point to it. The con-
jecture asserts that the task of ﬁnding a proof for a mathematical statement
is qualitatively harder than the task of simply verifying the correctness of a
given proof for the statement. To see this, observe that the language

      L = {(S, 1n ) | statement S has a proof of length ≤ n}

is in NP, assuming any reasonable axiomatic system.


A.3 NP-optimization problems and approximation
algorithms
Combinatorial optimization problems are problems of picking the “best” so-
lution from a ﬁnite set. An NP-optimization problem, Π, consists of:
• A set of valid instances, DΠ , recognizable in polynomial time. We will
  assume that all numbers speciﬁed in an input are rationals, since our model
  of computation cannot handle inﬁnite precision arithmetic. The size of an
  instance I ∈ DΠ , denoted by |I|, is deﬁned as the number of bits needed
  to write I under the assumption that all numbers occurring in the instance
  are written in binary.
• Each instance I ∈ DΠ has a set of feasible solutions, SΠ (I). We require that
  SΠ (I) = ∅, and that every solution s ∈ SΠ (I) is of length polynomially
  bounded in |I|. Furthermore, there is polynomial time algorithm that, given
  a pair (I, s), decides whether s ∈ SΠ (I).
346    An overview of complexity theory

• There is a polynomial time computable objective function, objΠ , that as-
  signs a nonnegative rational number to each pair (I, s), where I is an in-
  stance and s is a feasible solution for I. The objective function is frequently
  given a physical interpretation, such as cost, length, weight, etc.
• Finally, Π is speciﬁed to be either a minimization problem or a maximiza-
  tion problem.
    The restriction of Π to unit cost instances will be called the cardinality
version of Π.
    An optimal solution for an instance of a minimization (maximization)
problem is a feasible solution that achieves the smallest (largest) objective
function value. OPTΠ (I) will denote the objective function value of an opti-
mal solution to instance I. We will shorten this to OPT when it is clear that
we are referring to a generic instance of the particular problem being studied.
    With every NP-optimization problem, one can naturally associate a de-
cision problem by giving a bound on the optimal solution. Thus, the decision
version of NP-optimization problem Π consist of pairs (I, B), where I is an
instance of Π and B is a rational number. If π is a minimization (maximiza-
tion) problem, then the answer to the decision version is “yes” iﬀ there is a
feasible solution to I of cost ≤ B (≥ B). If so, we will say that (I, B) is a
“yes” instance; we will call it a “no” instance otherwise. For example, the
decision version of cardinality vertex cover is stated in Section A.1.
    Clearly, a polynomial time algorithm for Π can help solve the decision
version – by computing the cost of an optimal solution and comparing it with
B. Conversely, hardness established for the decision version carries over to Π.
Indeed hardness for an NP-optimization problem is established by showing
that its decision version is NP-hard. With a slight abuse of notation, we will
also say that the optimization version is NP-hard.
    An approximation algorithm produces a feasible solution that is “close”
to the optimal one, and is time eﬃcient. The formal deﬁnition diﬀers for
minimization and maximization problems. Let Π be a minimization (max-
imization) problem, and let δ be a function, δ : Z+ → Q+ , with δ ≥ 1
(δ ≤ 1). An algorithm A is said to be a factor δ approximation algorithm
for Π if, on each instance I, A produces a feasible solution s for I such that
fΠ (I, s) ≤ δ(|I|) · OPT(I) (fΠ (I, s) ≥ δ(|I|) · OPT(I)), and the running time
of A is bounded by a ﬁxed polynomial in |I|. Clearly, the closer δ is to 1, the
better is the approximation algorithm.
    On occasion we will relax this deﬁnition and will allow A to be random-
ized, i.e., it will be allowed to use the ﬂips of a fair coin. Assume we have
a minimization problem. Then we will say that A is a factor δ randomized
approximation algorithm for Π if, on each instance I, A produces a feasible
solution s for I such that
                                           1
      Pr[fΠ (I, s) ≤ δ(|I|) · OPT(I)] ≥      ,
                                           2
                                         A.4   Randomized complexity classes   347

where the probability is over the coin ﬂips. The running time of A is still
required to be polynomial in |I|. The deﬁnition for a maximization problem
is analogous.
Remark A.1 Even though δ has been deﬁned to be a function of the size
of the input, we will sometimes pick δ to be a function of a more convenient
parameter. For instance, for the set cover problem (Chapter 2), we will pick
this parameter to be the number of elements in the ground set.

A.3.1     Approximation factor preserving reductions

Typically, polynomial time reductions map optimal solutions to optimal so-
lutions; however, they do not preserve near-optimality of solutions. Indeed,
all NP-complete problems are equally hard from the viewpoint of obtain-
ing exact solutions. However, from the viewpoint of obtaining near-optimal
solutions, they exhibit the rich set of possibilities alluded to earlier.
    In this book we will encounter pairs of problems which may look quite
diﬀerent superﬁcially, but whose approximability properties are closely linked
(e.g., see Exercise 19.13). Let us deﬁne a suitable reducibility in order to
formally establish such connections. Several reductions have been deﬁned
that preserve constant factor approximability. The reducibility stated below
is a stringent version of these, and actually preserves the constant itself.
Pair of problems that are linked in this manner are either both minimization
problems or both maximization problems.
    Let Π1 and Π2 be two minimization problems (the deﬁnition for two
maximization problems is quite similar). An approximation factor preserving
reduction from Π1 to Π2 consists of two polynomial time algorithms, f and
g, such that
• for any instance I1 of Π1 , I2 = f (I1 ) is an instance of Π2 such that
  OPTΠ2 (I2 ) ≤ OPTΠ1 (I1 ), and
• for any solution t of I2 , s = g(I1 , t) is a solution of I1 such that

          objΠ1 (I1 , s) ≤ objΠ2 (I2 , t).

    It is easy to see that this reduction, together with an α factor algorithm
for Π2 , gives an α factor algorithm for Π1 (see Exercise 1.16).


A.4 Randomized complexity classes
Certain NP languages1 are characterized by the fact that they possess an
abundance of Yes certiﬁcates, which renders them essentially tractable, as-
suming availability of a source of random bits. Such languages belong to the
1
    The deﬁnitions of this section will be useful in Chapter 29.
348     An overview of complexity theory

class RP, short for Randomized Polynomial Time. A language L ∈ RP if
there is a polynomial p and a polynomial time bounded Turing machine M
such that for each string x ∈ {0, 1}∗ :
• if x ∈ L, then M (x, y) accepts for at least half the strings y of length p(|x|),
  and
       / L, then for any string y of length p(|x|), M (x, y) rejects.
• if x ∈
    Clearly, P ⊆ RP ⊆ NP. Suppose language L ∈ RP. On input x, we will
pick a random string, y, of length p(|x|) and will run M (x, y). Clearly, the
entire computation takes polynomial time. We may erroneously reject x even
though x ∈ L. However, the probability of this is at most 1/2. Let us call
this the error probability. By the usual trick of making repeated independent
runs, we can reduce the error probability to inverse exponential in the number
of runs.
    A language L belongs to the class co-RP iﬀ L ∈ RP. Such a language
has an abundance of No certiﬁcates. The corresponding machine may make
an error on inputs x ∈ / L. Finally, let us deﬁne ZPP, short for Zero-error
Probabilistic Polynomial Time, to be the class of languages for which there is
a randomized Turing machine (i.e., a Turing machine equipped with a source
of random bits) that always terminates with the correct answer and whose
expected running time is polynomial. It is easy to see (Exercise 1.17) that

      L ∈ ZPP iﬀ L ∈ (RP ∩ co-RP).

   DTIME(t) denotes the class of problems for which there is a deter-
ministic algorithm running in time O(t). Thus, P = DTIME(poly(n)),
where poly(n) = k≥0 nk . ZTIME(t) denotes the class of problems for
which there is a randomized algorithm running in expected time O(t). Thus,
ZPP = ZTIME(poly(n)).


A.5 Self-reducibility
Most known problems in NP exhibit an interesting property, called self-
reducibility, which yields a polynomial time algorithm for ﬁnding a solution
(a Yes certiﬁcate), given an oracle for the decision version. A slightly more
elaborate version of this property yields an exact polynomial time algorithm
for an NP-optimization problem, again given an oracle for the decision ver-
sion. In a sense this shows that the diﬃcult core of NP and NP-optimization
problems is their decision versions (see Section 16.2 and Exercise 28.7 for
other fundamental uses of self-reducibility).
    Perhaps the simplest setting to describe self-reducibility is SAT. Let φ be
a SAT formula on n Boolean variables x1 , . . . , xn . We will represent a truth
assignment to these n variables as n-bit 0/1 vectors (True = 1 and False
= 0). Let S be the set of satisfying truth assignments, i.e., solutions, to φ.
                                                          A.5     Self-reducibility     349

The important point is that for the setting of x1 to 0 (1), we can ﬁnd, in
polynomial time, a formula φ0 (φ1 ) on the remaining n − 1 variables whose
solutions, S0 (S1 ), are precisely solutions of φ having x1 = 0 (x1 = 1).
Example A.2 Suppose φ = (x1 ∨x2 ∨x3 )∧(x1 ∨x2 ∨x4 ). Then φ0 = (x2 ∨x3 )
and φ1 = (x2 ∨ x4 )                                                  ✷
    Using this property, an oracle for the decision version of SAT can be
used to ﬁnd a solution to φ, assuming it is satisﬁable, as follows. First check
whether φ0 is satisﬁable. If so, set x0 = 0, and ﬁnd any solution to φ0 .
Otherwise, set x1 = 1 (in this case φ1 must be satisﬁable), and ﬁnd a solution
to φ1 . In each case the problem has been reduced to a smaller one, and we
will be done in n iterations.
    The following representation will be particularly useful. Let T be a binary
tree of depth n whose leaves are all n-bit 0/1 strings, representing truth
assignments to the n variables. Leaves that are solutions to φ are marked
special. The root of T is labeled with φ and its internal nodes are labeled
with formulae whose solutions are in one-to-one correspondence with the
marked leaves in the subtree rooted at this node. Thus, the 0th child of the
root is labeled with φ0 and the 1st child is labeled with φ1 . Tree T is called
the self-reducibility tree for instance φ.
                                                   φ
                                                    s
                                                 ✑ ◗
                                              ✑       ◗
                                          ✑               ◗
                                      ✑                       ◗ φ
                              φ0   ✑                              ◗ 1
                                s
                                ✑                                     ◗s
                              ✁✁❆❆                                     ✁✁❆❆
                             ✁ ❆                                      ✁ ❆
                           ✁       ❆                                ✁       ❆
                         ✁           ❆                            ✁           ❆
                       ✁               ❆                        ✁               ❆
                      ✁                  ❆                    ✁                   ❆
                    ✁                      ❆                ✁                       ❆
                  ✁                          ❆            ✁                           ❆
                ✁                              ❆        ✁                               ❆
              ✁n                                 ❆
                                                n−1
                                                      ✁  n−1
                                                                                          ❆n
               0          ...           01      10              ...         1

    We will formalize the notion of self-reducibility for NP-optimization prob-
lems. Formalizing this notion for NP problems is an easier task and is left
as Exercise 1.15.
    First, let us illustrate self-reducibility for cardinality vertex cover. Observe
that an oracle for the decision version enables us to compute the size of the
optimal cover, OPT(G), by binary search on k. To actually ﬁnd an optimal
cover, remove a vertex v together with its incident edges to obtain graph
G , and compute OPT(G ). Clearly, v is in an optimal cover iﬀ OPT(G ) =
OPT(G) − 1. Furthermore, if v is in an optimal cover, then any optimal cover
in G , together with v, is an optimal cover in G. Otherwise, any optimal cover
350     An overview of complexity theory

for G must contain all neighbors, say N (v), of v (in order to cover all edges
incident at v). Let G be the graph obtained by removing v and N (v) from
G. Any optimal cover in G , together with N (v), is an optimal cover in G.
Thus, in both cases, we are left with the problem of ﬁnding an optimal cover
in a smaller graph, G or G . Continuing this way, an optimal cover in G can
be found in polynomial time.
    The above-stated reduction from the cardinality vertex cover problem to
its decision version works because we could demonstrate polynomial time
algorithms for
• obtaining the smaller graphs, G and G ,
• computing the size of the best cover in G, consistent with the atomic
  decision, and
• constructing an optimal cover in G, given an optimal cover in the smaller
  instance.
    The exact manner in which self-reducibility manifests itself is quite dif-
ferent for diﬀerent problems. Below we state a fairly general deﬁnition that
covers a large number of problems. In the interest of conveying the main idea
behind this important concept, we will provide an intuitive, though easily
formalizable, deﬁnition.
    We will assume that solutions to an instance I of NP-optimization prob-
lem Π have granularity, i.e., consist of smaller pieces called atoms that are
meaningful in the context of the problem. For instance, for cardinality vertex
cover, the atoms consist of specifying whether or not a certain vertex is in the
cover. Clearly, for vertex cover this can be done using O(log n) bits. Indeed,
all problems considered in this book have granularity O(log n). Let us assume
this for problem Π.

                                                I✲
            I✲
                                   
                      A           I✲            α✲        f           s✲
            α✲                                   
                                                s✲


    We will say that problem Π is self-reducible if there is a polynomial time
algorithm, A, and polynomial time computable functions, f (·, ·, ·) and g(·, ·, ·),
satisfying the following conditions.
• Given instance I and an atom α of a solution to I, A outputs an instance
  Iα . We require that |Iα | < |I|. Let S(I | α) represent the set of feasible
  solutions to I that are consistent with atom α. We require that the feasible
  solutions of Iα , S(Iα ), are in one-to-one correspondence with S(I | α).
  This correspondence is given by the polynomial time computable function
  f (·, ·, ·) as follows.
                                                                      A.6    Notes       351

         f (I, α, ·) : S(Iα ) → S(I | α).

• The correspondence f (I, α, ·) preserves order in the objective function val-
  ues of solutions. Thus, if s1 and s2 are two feasible solutions of Iα with
  objΠ (Iα , s1 ) ≤ objΠ (Iα , s2 ), and f (I, α, s1 ) = s1 and f (I, α, s2 ) = s2 , then
  objΠ (I, s1 ) ≤ objΠ (I, s2 ).
• Given the cost of an optimal solution to Iα , the cost of the best solution
  in S(I | α) can be computed eﬃciently, and is given by g(I, α, OPT(Iα )).

Theorem A.3 Let Π be an NP-optimization problem that is self-reducible.
There is a polynomial time (exact) algorithm for Π, given an oracle, O, for
the decision version of Π.

Proof: As remarked earlier, via a suitable binary search we can use O to
compute the cost of the optimal solution to an instance in polynomial time.
    We will derive polynomial time algorithm R for solving Π exactly. Assume
that A, f , and g are deﬁned as above for the self-reducibility of Π. Let I be
an instance of Π. R ﬁrst ﬁnds one atom of an optimal solution to I. An
atom, say β, satisﬁes this condition iﬀ g(I, β, OPT(Iβ )) = OPT(I), where
Iβ = A(I, β). Since atoms are only O(log n) bits long, ﬁnding such an atom
involves simply searching the polynomially many possibilities. Let α be the
atom found, and let Iα = A(I, α). R then recursively computes an optimal
solution, say s , to Iα . Finally, it outputs f (I, α, s ), which is guaranteed to
be an optimal solution to I. Since |Iα | < |I|, the recursion also takes only
polynomial time.                                                                  ✷

Remark A.4 The number of strings of length O(log n) that algorithm R
needs to examine for ﬁnding a good atom depends on the speciﬁc problem.
For instance, in the case of cardinality vertex cover we picked an arbitrary
vertex, say v, and considered only two atoms, that v is or isn’t in the cover.


A.6 Notes
The deﬁnition of an NP-optimization problem is due to Krentel [178].
Approximation factor preserving reductions are a stringent version of L-
reducibility from Papadimitriou and Yannakakis [218]. Self-reducibility was
ﬁrst deﬁned by Schnorr [234]. See Khuller and Vazirani [171] for a problem
that is not self-reducible, assuming P = NP. For further information on
NP-completeness and complexity theory see Garey and Johnson [93] and
Papadimitriou [216].
B Basic Facts from Probability Theory




Let us recall some useful facts from probability theory. We assume that the
reader has already had a detailed exposure to this material (see Section B.4
for references).


B.1 Expectation and moments
Two quantities provide much information about a random variable: the mean,
also called expectation, and variance. A key property of the expectation,
which often simpliﬁes its evaluation, is called linearity of expectation. It states
that if X, X1 , . . . , Xn are random variables such that X = c1 X1 + . . . + cn Xn ,
where c1 , . . . , cn are constants, then

      E[X] = c1 E[X1 ] + . . . + cn E[Xn ].

(In particular, the expectation of a sum of random variables is the sum of
their expectations.) The usefulness of this property arises from the fact that
no assumption is made about independence between the random variables
X1 , . . . , Xn . Often a complex random variable can be written as the sum of
indicator random variables (i.e., random variables taking on 0/1 values only),
thereby simplifying the evaluation of its expectation.
    The variance of random variable X measures the spread of X from its
mean, and is deﬁned as

      V [X] = E[(X − E[X])2 ] = E[X 2 ] − E[X]2 .

Its positive square root is called the standard deviation. The mean and stan-
dard deviation of X are denoted by µ(X) and σ(X), respectively.
    For k ∈ N, the kth moment and kth central moment of X are deﬁned to
be E[X k ] and E[(X − E[X])k ], respectively. Thus the variance is the second
central moment.
    In general, the expectation of the product of random variables is not the
product of expectations. An important exception is when the random vari-
ables are independent. Thus, if X and Y are independent random variables,
then E[XY ] = E[X]E[Y ]. This immediately implies that the variance of the
sum of independent random variables is the sum of their variances, i.e., for
independent random variables X and Y , V [X + Y ] = V [X] + V [Y ].
                                                   B.2          Deviations from the mean   353

B.2 Deviations from the mean
If X is a nonnegative random variable with a known expectation, then
Markov’s Inequality helps bound the probability of deviations from the mean
as follows. For t ∈ R+ ,

                      E[X]
      Pr[X ≥ t] ≤          .
                        t
This obvious inequality ﬁnds surprisingly many uses. For instance, it helps
in obtaining a high probability statement from a bound on the expectation
(e.g., see Section 14.2).
    If the variance of a random variable is small, then large deviations from the
mean are improbable. This intuitive statement is formalized by Chebyshev’s
inequality which states that for any random variable X and a ∈ R+,
                                             2
                                       σ(X)
      Pr[|X − E[X]| ≥ a] ≤                         .
                                         a

See Lemma 28.5 for an application.
    Poisson trials are repeated independent trials, each of which has two pos-
sible outcomes, called success and failure. In general, the success probability
is allowed to change with the trials. They are called Bernoulli trials if the
success probability is the same for each trial.
    The Chernoﬀ bounds, which provide bounds on the tail probabilities of
Poisson trials, are very useful in analyzing algorithms. Let us represent n Pois-
son trials by indicator random variables X1 , . . . , Xn , with 1 and 0 representing
success and failure, respectively. Let Pr[Xi = 1] = pi , where 0 < pi       < 1 for
                                                                              n
1 ≤ i ≤ n. Let random variable X = X1 + . . . + Xn and µ = E[X] = i=1 pi .
For the bound on the lower tail assume 0 < δ ≤ 1. Then,
                                       2
      Pr[X < (1 − δ)µ] < e(−µδ /2) .

The expression for the upper tail is more involved: for any δ > 0,
                                                      µ
                                        eδ
      Pr[X > (1 + δ)µ] <                                    .
                                   (1 + δ)(1+δ)

It can be simpliﬁed by considering two ranges for δ. For δ > 2e − 1,

      Pr[X > (1 + δ)µ] < 2−(1+δ)µ ,

and for δ ≤ 2e − 1,
                                       2
      Pr[X > (1 + δ)µ] < e−µδ /4 .
354    B   Basic Facts from Probability Theory

B.3 Basic distributions
Three distributions, of great universality, are deﬁned below. The probability
distribution of the number of successes in Bernoulli trials is called the bino-
mial distribution. Consider n Bernoulli trials with probability of success p.
The probability of k successes, for 0 ≤ k ≤ n, is given by
                      
                      n k
      B(k; n, p) =       p (1 − p)n−k .
                      k

   The Poisson distribution with parameter λ > 0 is as follows. For each
nonnegative integer k, the probability of exactly k successes is deﬁned to be

                      λk
      p(k; λ) = e−λ      .
                      k!

The limit of the binomial distribution B(k; n, p) as n → ∞ and np → λ, a
constant, is the Poisson distribution p(k; λ). Indeed, in many applications one
comes across Bernoulli trials in which n is large, p is small, and the product
λ = np is moderate. In these situations, p(k; np) is a good approximation to
B(k; n, p).
   The normal density function with mean µ and standard deviation σ is

                  1  (x−µ)2
      n(x) =     √ e− 2σ2 ,
                σ 2π

and the normal distribution function is its integral,
                             x
                  1                (y−µ)2
      N (x) =    √               e− 2σ2 dy.
                σ 2π     −∞

The normal distribution also approximates the binomial distribution. Let us
state this for the case p = 1/2. Let n be even, n = 2ν, say. For −ν ≤ k ≤ ν,
deﬁne

      ak = a−k = B(ν + k; 2ν, 1/2).
                                                               √
                                  in the range 0 < k <
In the limit as ν → ∞ and k varies.                                ν, ak can be
                                              2   √2 .
approximated by hn(kh), where h =             ν =   n




B.4 Notes
For further information see the books by Feller [85], Motwani and Raghavan
[209], Spencer [243], and Alon and Spencer [6].
References




 1. A. Agrawal, P. Klein, and R. Ravi. When trees collide: an approximation
    algorithm for the generalized Steiner network problem on networks. SIAM
    Journal on Computing, 24:440–456, 1995. (Cited on pp. 130, 212)
 2. M. Ajtai. The shortest vector problem in L2 is NP-hard for randomized
    reductions. In Proc. 30th ACM Symposium on the Theory of Computing,
    pages 10–19, 1998. (Cited on p. 293)
 3. D. Aldous. The random walk construction for spanning trees and uniform la-
    beled trees. SIAM Journal on Discrete Mathematics, 3:450–465, 1990. (Cited
    on p. 339)
 4. F. Alizadeh. Interior point methods in semideﬁnite programming with applica-
    tions to combinatorial optimization. SIAM Journal on Optimization, 5:13–51,
    1995. (Cited on p. 268)
 5. N. Alon, A. Frieze, and D. Welsh. Polynomial time randomised approximation
    schemes for Tutte-Grothendieck invariants: the dense case. Random Structures
    and Algorithms, 6:459–478, 1995. (Cited on p. 342)
 6. N. Alon and J. Spencer. The Probabilistic Method. John Wiley & Sons, New
    York, NY, 2000. (Cited on pp. 139, 354)
 7. J. D. Annan. The complexities of the coeﬃcients of the Tutte polynomial.
    Discrete Applied Mathematics, 57:93–103, 1995. (Cited on p. 342)
 8. J.D. Annan. A randomized approximation algorithm for counting the number
    of forests in dense graphs. Combinatorics, Probability and Computing, 3:273–
    283, 1994. (Cited on p. 339)
 9. S. Arora. Polynomial time approximation scheme for Euclidean TSP and other
    geometric problems. In Proc. 37th IEEE Annual Symposium on Foundations
    of Computer Science, pages 2–11, 1996. (Cited on p. 89)
10. S. Arora. Nearly linear time approximation scheme for Euclidean TSP and
    other geometric problems. In Proc. 38th IEEE Annual Symposium on Foun-
    dations of Computer Science, pages 554–563, 1997. (Cited on p. 89)
11. S. Arora and C. Lund. Hardness of approximations. In D.S. Hochbaum,
    editor, Approximation Algorithms for NP-Hard Problems, pages 46–93. PWS
    Publishing, Boston, MA, 1997. (Cited on p. 332)
12. S. Arora, C. Lund, R. Motwani, M. Sudan, and M. Szegedy. Proof veriﬁcation
    and intractability of approximation problems. In Proc. 33rd IEEE Annual
    Symposium on Foundations of Computer Science, pages 13–22, 1992. (Cited
    on p. 332)
13. S. Arora, P. Raghavan, and S. Rao. Approximation schemes for Euclidean k-
    medians and related problems. In Proc. 30th ACM Symposium on the Theory
    of Computing, pages 106–113, 1998. (Cited on p. 89)
356   References

14. S. Arora and S. Safra. Probabilistic checking of proofs: a new characterization
    of NP. In Proc. 33rd IEEE Annual Symposium on Foundations of Computer
    Science, pages 2–13, 1992. (Cited on p. 332)
15. V. Arya, N. Garg, R. Khandekar, A. Meyerson, K. Munagala, and V. Pan-
    dit. Local search heuristics for k-median and facility location problems. In
    Proc. 33rd ACM Symposium on the Theory of Computing, 2001. (Cited on
    pp. 253, 254)
16. Y. Aumann and Y. Rabani. An O(log k) approximate min-cut max-ﬂow theo-
    rem and approximation algorithms. SIAM Journal on Computing, 27:291–301,
    1998. (Cited on p. 197)
17. G. Ausiello, P. Crescenzi, G. Gambosi, V. Kann, A. Marchetti-Spaccamela,
    and M. Protasi. Complexity and Approximation. Combinatorial Optimization
    Problems and their Approximability Properties. Springer-Verlag, Berlin, 1999.
    (Cited on pp. 11, 333)
18. L. Babai. Trading group theory for randomness. In Proc. 17th ACM Sympo-
    sium on the Theory of Computing, pages 421–429, 1985. (Cited on p. 332)
19. V. Bafna, P. Berman, and T. Fujito. Constant ratio approximations of the
    weighted feedback vertex set problem for undirected graphs. In Algorithms
    and Computation, 6th International Symposium, ISAAC, volume 1004 of Lec-
    ture Notes in Computer Science, pages 142–151. Springer-Verlag, Berlin, 1995.
    (Cited on p. 60)
20. R. Bar-Yehuda and S. Even. A linear-time approximation algorithm for
    the weighted vertex cover problem. Journal of Algorithms, 2:198–203, 1981.
    (Cited on p. 130)
21. Y. Bartal. Probabilistic approximation of metric spaces and its algorithmic
    applications. In Proc. 37th IEEE Annual Symposium on Foundations of Com-
    puter Science, pages 184–193, 1996. (Cited on p. 254)
22. C. Bazgan, M. Santha, and Z. Tuza. Eﬃcient approximation algorithms for the
    subset-sum problem. In Proc. 25th International Colloquium on Automata,
    Languages, and Programming, volume 1443 of Lecture Notes in Computer
    Science, pages 387–396. Springer-Verlag, Berlin, 1998. (Cited on p. 72)
23. A. Becker and D. Geiger. Approximation algorithms for the loop cutset prob-
    lem. In Proc. 10th Conference on Uncertainty in Artiﬁcial Intelligence, pages
    60–68, 1994. (Cited on p. 60)
24. M. Ben-or, S. Goldwasser, J. Kilian, and A. Wigderson. Multi-prover interac-
    tive proofs: How to remove intractability. In Proc. 20th ACM Symposium on
    the Theory of Computing, pages 113–131, 1988. (Cited on p. 332)
25. M. Bern and P. Plassmann. The Steiner problem with edge lengths 1 and 2.
    Information Processing Letters, 32:171–176, 1989. (Cited on p. 332)
26. S.N. Bhatt and F.T. Leighton. A framework for solving VLSI graph lay-
    out problems. Journal of Computer and System Sciences, 28:300–343, 1984.
    (Cited on p. 197)
27. A. Blum, T. Jiang, M. Li, J. Tromp, and M. Yannakakis. Linear approximation
    of shortest superstring. Journal of the ACM, 41:630–647, 1994. (Cited on
    p. 67)
28. M. Blum and S. Kannan. Designing programs that check their work. In Proc.
    21st ACM Symposium on the Theory of Computing, pages 86–97, 1989. (Cited
    on p. 332)
                                                                References     357

29. M. Blum, M. Luby, and R. Rubinfeld. Testing/correcting with applications to
    numerical problems. Journal of Computer and System Sciences, 47:549–595,
    1993. (Cited on p. 332)
30. R. Boppana and M.M. Halldórsson. Approximating maximum independent
    sets by excluding subgraphs. BIT, 32:180–196, 1992. (Cited on p. 332)
31. A. Borodin and R. El-Yaniv. Online Computation and Competitive Analysis.
    Cambridge University Press, Cambridge, UK, 1998. (Cited on p. 78)
32. J. Bourgain. On Lipschitz embedding of ﬁnite metric spaces in Hilbert spaces.
    Israeli J. Math., 52:46–52, 1985. (Cited on p. 197)
33. A.Z. Broder. How hard is it to marry at random? In Proc. 18th ACM Sym-
    posium on the Theory of Computing, pages 50–58, 1986. (Cited on p. 305)
34. A.Z. Broder. Generating random spanning trees. In Proc. 30th IEEE Annual
    Symposium on Foundations of Computer Science, pages 442–447, 1989. (Cited
    on p. 339)
35. R. Bubley and M. Dyer. Faster random generation of linear extensions. Dis-
    crete Mathematics, 201:81–88, 1999. (Cited on p. 340)
36. G. Calinescu, H. Karloﬀ, and Y. Rabani. An improved approximation algo-
    rithm for multiway cut. In Proc. 30th ACM Symposium on the Theory of
    Computing, pages 48–52, 1998. (Cited on p. 167)
37. M. Charikar, C. Chekuri, T. Cheung, Z. Dai, A. Goel, S. Guha, and M. Li.
    Approximation algorithms for directed Steiner tree problems. In Proc. 9th
    ACM-SIAM Annual Symposium on Discrete Algorithms, pages 192–200, 1998.
    (Cited on p. 337)
38. M. Charikar and S. Guha. Improved combinatorial algorithms for the facility
    location and k-median problems. In Proc. 40th IEEE Annual Symposium on
    Foundations of Computer Science, pages 378–388, 1999. (Cited on p. 254)
39. M. Charikar, S. Guha, É. Tardos, and D.B. Shmoys. A constant-factor approx-
    imation algorithm for the k-median problem. In Proc. 31st ACM Symposium
    on the Theory of Computing, pages 1–10, 1999. (Cited on p. 254)
40. M. Charikar, S. Khuller, D.M. Mount, and G. Narshimhan. Algorithms for
    facility location problems with outliers. In Proc. 12th ACM-SIAM Annual
    Symposium on Discrete Algorithms, pages 642–651, 2001. (Cited on p. 240)
41. M. Charikar, J. Kleinberg, R. Kumar, S. Rajagopalan, A. Sahai, and
    A. Tomkins. Minimizing wirelength in zero and bounded skew clock trees.
    In Proc. 10th ACM-SIAM Annual Symposium on Discrete Algorithms, pages
    177–184, 1999. (Cited on p. 37)
42. J. Cheriyan and R. Thurimella. Approximating minimum-size k-connected
    spanning subgraphs via matching. In Proc. 37th IEEE Annual Symposium
    on Foundations of Computer Science, pages 292–301, 1996.            (Cited on
    pp. 226, 227, 231)
43. B. Chor and M. Sudan. A geometric approach to betweenness. SIAM Journal
    on Discrete Mathematics, 11:511–523, 1998. (Cited on p. 267)
44. E.-A. Choukhmane. Une heuristique pour le problème de l’arbre de Steiner.
    RAIRO Rech. Opér., 12:207–212, 1978. (Cited on p. 37)
45. N. Christoﬁdes. Worst-case analysis of a new heuristic for the traveling sales-
    man problem. Technical report, Graduate School of Industrial Administration,
    Carnegie-Mellon University, Pittsburgh, PA, 1976. (Cited on p. 37)
46. F. Chudak, M.X. Goemans, D. Hochbaum, and D.P. Williamson. A primal–
    dual interpretation of two 2-approximation algorithms for the feedback vertex
358   References

    set problem in undirected graphs. Operations Research Letters, 22:111–118,
    1998. (Cited on pp. 60, 129)
47. F. Chudak, T. Roughgarden, and D.P. Williamson. Approximate k-MSTs
    and k-Steiner trees via the primal–dual method and Lagrangian relaxation.
    Manuscript, 2000. (Cited on p. 252)
48. V. Chvátal. A greedy heuristic for the set covering problem. Mathematics of
    Operations Research, 4:233–235, 1979. (Cited on pp. 26, 118)
49. V. Chvátal. Linear Programming. W.H. Freeman and Co., New York, NY,
    1983. (Cited on p. 107)
50. E.G. Coﬀman Jr., M.R. Garey, and D.S. Johnson. Approximation algorithms
    for bin backing: a survey. In D.S. Hochbaum, editor, Approximation Algo-
    rithms for NP-Hard Problems, pages 46–93. PWS Publishing, Boston, MA,
    1997. (Cited on p. 78)
51. S.A. Cook. The complexity of theorem-proving procedures. In Proc. 3rd
    ACM Symposium on the Theory of Computing, pages 151–158, 1971. (Cited
    on p. 10)
52. W.J. Cook, W.H. Cunningham, W.R. Pulleyblank, and A. Schrijver. Combi-
    natorial Optimization. John Wiley & Sons, New York, NY, 1998. (Cited on
    p. 107)
53. C. Cooper and A. Frieze. Mixing properties of the Swendsen-Wang process on
    classes of graphs. Random Structures Algorithms, 15:242–261, 1999. (Cited
    on p. 342)
54. T. H. Cormen, C. E. Leiserson, R. L. Rivest, and C. Stein. Introduction to
    Algorithms. Second edition. MIT Press and McGraw-Hill, 2001. (Cited on
    p. 11)
55. R. Courant and H. Robbins. What Is Mathematics? Oxford University Press,
    New York, NY, 1941. (Cited on p. 37)
56. P. Dagum, M. Luby, M. Mihail, and U.V. Vazirani. Polytopes, permanents
    and graphs with large factors. In Proc. 29th IEEE Annual Symposium on
    Foundations of Computer Science, pages 412–421, 1988. (Cited on p. 339)
57. E. Dahlhaus, D.S. Johnson, C.H. Papadimitriou, P.D. Seymour, and M. Yan-
    nakakis. The complexity of multiterminal cuts. SIAM Journal on Computing,
    23:864–894, 1994. (Cited on p. 46)
58. G.B. Dantzig. Linear Programming and Extensions. Reprint of the 1968
    corrected edition. Princeton University Press, Princeton, NJ, 1998. (Cited on
    p. 107)
59. G.B. Dantzig, L.R. Ford, and D.R. Fulkerson. Solution of a large-scale
    traveling-salesman problem. Operations Research, 2:393–410, 1954. (Cited
    on p. 231)
60. G.B. Dantzig, L.R. Ford, and D.R. Fulkerson. A primal–dual algorithm for
    linear programs. In H.W. Kuhn and A.W. Tucker, editors, Linear Inequalities
    and Related Systems, pages 171–181. Princeton University Press, Princeton,
    NJ, 1956. (Cited on p. 129)
61. G. Dobson. Worst-case analysis of greedy heuristics for integer programming
    with non-negative data. Mathematics of Operations Research, 7:515–531, 1982.
    (Cited on p. 118)
62. P. Drineas, R. Kannan, A. Frieze, S. Vempala, and V. Vinay. Clustering in
    large graphs and matrices. In Proc. 10th ACM-SIAM Annual Symposium on
    Discrete Algorithms, pages 291–299, 1999. (Cited on p. 254)
                                                                      References       359

63. D.Z. Du and F.K. Hwang. Gilbert-Pollack conjecture on Steiner ratio is true.
    Algorithmica, 7:121–135, 1992. (Cited on p. 37)
64. M. Dyer, R. Kannan, and J. Mount. Sampling contingency tables. Random
    Structures and Algorithms, 10:487–506, 1997. (Cited on p. 340)
65. M.E. Dyer, A. Frieze, and M.R. Jerrum. Approximately counting hamilton
    cycles in dense graphs. SIAM Journal on Computing, 27:1262–1272, 1998.
    (Cited on p. 341)
66. M.E. Dyer, A. Frieze, and M.R. Jerrum. On counting independent sets in
    sparse graphs. In Proc. 40th IEEE Annual Symposium on Foundations of
    Computer Science, pages 210–217, 1999. (Cited on p. 341)
67. M.E. Dyer, A. Frieze, and R. Kannan. A random polynomial time algorithm
    for approximating the volume of convex bodies. Journal of the ACM, 38:1–17,
    1991. (Cited on p. 338)
68. J. Edmonds. Maximum matching and a polyhedron with 0,1-vertices. Journal
    of Research of the National Bureau of Standards. Section B, 69:125–130, 1965.
    (Cited on p. 104)
69. J. Edmonds. Paths, trees, and ﬂowers. Canadian Journal of Mathematics,
    17:449–467, 1965. (Cited on pp. 10, 11)
70. J. Edmonds. Optimum branchings. Journal of Research of the National Bu-
    reau of Standards. Section B, 71:233–240, 1967. (Cited on p. 212)
71. J. Edmonds. Matroids and the greedy algorithm. Mathematical Programming,
    1:127–136, 1971. (Cited on p. 105)
72. J. Edmonds. Matroid intersection. Annals of Discrete Mathematics, 4:185–
    204, 1979. (Cited on p. 228)
73. P. Erdős. Gráfok páros körüljárású részgráfjairól (On bipartite subgraphs of
    graphs, in Hungarian). Mat. Lapok, 18:283–288, 1967. (Cited on p. 10)
74. P. Erdős and J.L. Selfridge. On a combinatorial game. Journal of Combina-
    torial Theory, Series A, 14:298–301, 1973. (Cited on p. 139)
75. G. Even, J. Naor, B. Schieber, and S. Rao. Divide-and-conquer approximation
    algorithms via spreading metrics. Journal of the ACM, 47:585–616, 2000.
    (Cited on p. 178)
76. G. Even, J. Naor, B. Schieber, and L. Zosin. Approximating minimum sub-
    set feedback sets in undirected graphs with applications. In Proc. 4th Israel
    Symposium on Theory of Computing and Systems, pages 78–88, 1996. (Cited
    on p. 167)
77. G. Even, J. Naor, and L. Zosin. An 8-approximation algorithm for the sub-
    set feedback vertex set problem. In Proc. 37th IEEE Annual Symposium on
    Foundations of Computer Science, pages 310–319, 1996. (Cited on p. 167)
78. T. Feder and M. Mihail. Balanced matroids. In Proc. 24th ACM Symposium
    on the Theory of Computing, pages 26–38, 1992. (Cited on p. 339)
79. U. Feige. Approximating the bandwidth via volume respecting embeddings.
    In Proc. 30th ACM Symposium on the Theory of Computing, pages 90–99,
    1998. (Cited on p. 196)
80. U. Feige. A treshold of ln n for approximating set cover. Journal of the ACM,
    45:634–652, 1998. (Cited on pp. 26, 331, 332)
81. U. Feige and M.X. Goemans. Approximating the value of two prover proof
    systems, with applications to MAX-CUT and MAX DICUT. In Proc. 3rd
    Israel Symposium on Theory of Computing and Systems, pages 182–189, 1995.
    (Cited on p. 269)
360    References

82. U. Feige, S. Goldwasser, L. Lovász, S. Safra, and M. Szegedy. Approximating
    clique is almost NP-complete. In Proc. 32nd IEEE Annual Symposium on
    Foundations of Computer Science, pages 2–12, 1991. (Cited on p. 332)
83. U. Feige and R. Krauthgamer. A polylogarithmic approximation of the min-
    imum bisection. In Proc. 41st IEEE Annual Symposium on Foundations of
    Computer Science, pages 105–115, 2000. (Cited on pp. 197, 336)
84. U. Feige and G. Schechtman. On the optimality of the random hyperplane
    rounding technique for MAX-CUT. In Proc. 33rd ACM Symposium on the
    Theory of Computing, 2001. (Cited on p. 268)
85. W. Feller. An Introduction to Probability Theory and its Applications. John
    Wiley & Sons, New York, NY, 1950. (Cited on p. 354)
86. W. Fernandez de la Vega and G.S. Lueker. Bin packing can be solved within
    1 + ε in linear time. Combinatorica, 1:349–355, 1981. (Cited on p. 78)
                                                          1
87. A. Freund and H. Karloﬀ. A lower bound of 8/(7 + k−1    ) on the integrality ra-
    tio of the Calinescu–Karloﬀ–Rabani relaxation for multiway cut. Information
    Processing Letters, 75:43–50, 2000. (Cited on p. 167)
88. A. Frieze. On the Lagarias–Odlyzko algorithm for the subset sum problem.
    SIAM Journal on Computing, 15:536–539, 1986. (Cited on p. 291)
89. A. Frieze, G. Galbiati, and F. Maﬃoli. On the worst-case performance of
    some algorithms for the asymmetric traveling salesman problem. Networks,
    12:23–39, 1982. (Cited on p. 34)
90. A. Frieze and M. Jerrum. Improved approximation algorithms for MAX k-
    CUT and MAX BISECTION. Algorithmica, 18:67–81, 1997. (Cited on p. 269)
91. M.R. Garey, R.L. Graham, and J.D. Ullman. An analysis of some packing
    algorithms. In Combinatorial Algorithms (Courant Computer Science Sympo-
    sium, No. 9), pages 39–47, 1972. (Cited on p. 10)
92. M.R. Garey and D.S. Johnson. Strong NP-completeness results: motivation,
    examples, and implications. Journal of the ACM, 25:499–508, 1978. (Cited
    on p. 73)
93. M.R. Garey and D.S. Johnson. Computers and Intractability: A Guide to the
    Theory of NP-Completeness. W.H. Freeman and Co., New York, NY, 1979.
    (Cited on pp. 11, 351)
94. N. Garg. A 3-approximation for the minimum tree spanning k vertices. In
    Proc. 37th IEEE Annual Symposium on Foundations of Computer Science,
    pages 302–309, 1996. (Cited on p. 252)
95. N. Garg, H. Saran, and V.V. Vazirani. Finding separator cuts in planar graphs
    within twice the optimal. SIAM Journal on Computing, 29:159–179, 1999.
    (Cited on p. 336)
96. N. Garg, V.V. Vazirani, and M. Yannakakis. Multiway cuts in directed and
    node weighted graphs. In Proc. 21st International Colloquium on Automata,
    Languages, and Programming, volume 820 of Lecture Notes in Computer Sci-
    ence, pages 487–498. Springer-Verlag, Berlin, 1994. (Cited on p. 167)
97. N. Garg, V.V. Vazirani, and M. Yannakakis. Approximate max-ﬂow min-
    (multi)cut theorems and their applications. SIAM Journal on Computing,
    25:235–251, 1996. (Cited on p. 179)
98. N. Garg, V.V. Vazirani, and M. Yannakakis. Primal–dual approximation al-
    gorithms for integral ﬂow and multicut in trees. Algorithmica, 18:3–20, 1997.
    (Cited on pp. 152, 153, 154)
99. C.F. Gauss. Disquisitiones Arithmeticae. English edition translated by A.A.
    Clarke. Springer-Verlag, New York, NY, 1986. (Cited on p. 292)
                                                                References     361

100. E.N. Gilbert and H.O. Pollak. Steiner minimal trees. SIAM Journal on Applied
     Mathematics, 16:1–29, 1968. (Cited on p. 37)
101. M.X. Goemans and D.J. Bertsimas. Survivable networks, linear program-
     ming relaxations and the parsimonious property. Mathematical Programming,
     60:145–166, 1993. (Cited on p. 228)
102. M.X. Goemans, A.V. Goldberg, S. Plotkin, D.B. Shmoys, É. Tardos, and D.P.
     Williamson. Improved approximation algorithms for network design problems.
     In Proc. 5th ACM-SIAM Annual Symposium on Discrete Algorithms, pages
     223–232, 1994. (Cited on p. 225)
103. M.X. Goemans and J. Kleinberg. The Lovász theta function and a semideﬁnite
     programming relaxation of vertex cover. SIAM Journal on Discrete Mathe-
     matics, 11:196–204, 1998. (Cited on p. 334)
104. M.X. Goemans and D.P. Williamson. New 34 -approximation algorithms for
     the maximum satisﬁability problem. SIAM Journal on Discrete Mathematics,
     7:656–666, 1994. (Cited on pp. 138, 139)
105. M.X. Goemans and D.P. Williamson. A general approximation technique for
     constrained forest problems. SIAM Journal on Computing, 24:296–317, 1995.
     (Cited on pp. 130, 208, 212)
106. M.X. Goemans and D.P. Williamson. Improved approximation algorithms
     for maximum cut and satisﬁability problems using semideﬁnite programming.
     Journal of the ACM, 42:1115–1145, 1995. (Cited on pp. 267, 268)
107. M.X. Goemans and D.P. Williamson. The primal–dual method for approxi-
     mation algorithms and its applications to network design problems. In D.S.
     Hochbaum, editor, Approximation Algorithms for NP-Hard Problems, pages
     144–191. PWS Publishing, Boston, MA, 1997. (Cited on pp. 130, 212)
108. O. Goldreich, D. Micciancio, S. Safra, and J.-P. Seifert. Approximating short-
     est lattice vectors is not harder than approximating closest lattice vectors.
     Information Processing Letters, 71, 1999. (Cited on p. 292)
109. S. Goldwasser, S. Micali, and C. Rackoﬀ. The knowledge complexity of in-
     teractive proofs. SIAM Journal on Computing, 18:186–208, 1989. (Cited on
     p. 332)
110. R.E. Gomory and T.C. Hu. Multi-terminal network ﬂows. Journal of the
     SIAM, 9:551–570, 1961. (Cited on p. 46)
111. T.F. Gonzalez. Clustering to minimize the maximum inter-cluster distance.
     Theoretical Computer Science, 38:293–306, 1985. (Cited on p. 52)
112. V. Gore and M. Jerrum. The Swendsen-Wang process does not always mix
     rapidly. In Proc. 29th ACM Symposium on the Theory of Computing, pages
     674–681, 1997. (Cited on p. 342)
113. R.L. Graham. Bounds for certain multiprocessing anomalies. Bell System
     Technical Journal, 45:1563–1581, 1966. (Cited on pp. 10, 83)
114. R.L. Graham. Bounds on multiprocessing timing anomalies. SIAM Journal
     on Applied Mathematics, 17:416–429, 1969. (Cited on p. 83)
115. M. Grigni, E. Koutsoupias, and C. Papadimitriou. An approximation scheme
     for planar graph TSP. In Proc. 36th IEEE Annual Symposium on Foundations
     of Computer Science, pages 640–646, 1995. (Cited on p. 89)
116. M. Grötschel, L. Lovász, and A. Schrijver. The ellipsoid method and its
     consequences in combinatorial optimization. Combinatorica, 1:169–197, 1981.
     (Cited on p. 107)
362    References

117. M. Grötschel, L. Lovász, and A. Schrijver. Geometric Algorithms and Combi-
     natorial Optimization. Second edition. Springer-Verlag, Berlin, 1993. (Cited
     on p. 107)
118. V. Guruswami, S. Khanna, R. Rajaraman, B. Sheperd, and M. Yannakakis.
     Near-optimal hardness results and approximation algorithms for edge-disjoint
     and related problems. In Proc. 31st ACM Symposium on the Theory of Com-
     puting, pages 19–28, 1999. (Cited on p. 154)
119. D. Gusﬁeld and R. W. Irving. The Stable Marriage Problem: Structure and
     Algorithms. MIT Press, Cambridge, MA, 1989. (Cited on p. 341)
120. L.A. Hall. Approximation algorithms for scheduling. In D.S. Hochbaum,
     editor, Approximation Algorithms for NP-Hard Problems, pages 1–45. PWS
     Publishing, Boston, MA, 1997. (Cited on p. 145)
121. J. Hastad. Clique is hard to approximate within n1−ε . In Proc. 37th IEEE An-
     nual Symposium on Foundations of Computer Science, pages 627–636, 1996.
     (Cited on p. 332)
122. J. Hastad. Some optimal inapproximability results. In Proc. 29th ACM Sym-
     posium on the Theory of Computing, pages 1–10, 1997. (Cited on pp. 330, 332)
123. M. Held and R.M. Karp. The traveling-salesman and minimum cost spanning
     trees. Operations Research, 18:1138–1162, 1970. (Cited on p. 230)
124. D. S. Hochbaum. Heuristics for the ﬁxed cost median problem. Mathematical
     Programming, 22:148–162, 1982. (Cited on p. 242)
125. D.S. Hochbaum. Approximation algorithms for the set covering and vertex
     cover problems. SIAM Journal on Computing, 11:555–556, 1982. (Cited on
     pp. 25, 124)
126. D.S. Hochbaum, editor. Approximation Algorithms for NP-Hard Problems.
     PWS Publishing, Boston, MA, 1997. (Cited on p. 11)
127. D.S. Hochbaum and D.B. Shmoys. A uniﬁed approach to approximation al-
     gorithms for bottleneck problems. Journal of the ACM, 33:533–550, 1986.
     (Cited on p. 53)
128. D.S. Hochbaum and D.B. Shmoys. Using dual approximation algorithms for
     scheduling problems: theoretical and practical results. Journal of the ACM,
     34:144–162, 1987. (Cited on p. 83)
129. D.S. Hochbaum and D.B. Shmoys. A polynomial approximation scheme for
     machine scheduling on uniform processors: using the dual approximation ap-
     proach. SIAM Journal on Computing, 17:539–551, 1988. (Cited on p. 145)
130. J.A. Hoogeveen. Analysis of Christoﬁdes’ heuristic: some paths are more
     diﬃcult than cycles. Operations Research Letters, 10:291–295, 1991. (Cited
     on p. 34)
131. E. Horowitz and S.K. Sahni. Exact and approximate algorithms for scheduling
     nonidentical processors. Journal of the ACM, 23:317–327, 1976. (Cited on
     p. 83)
132. W.L. Hsu and G.L. Nemhauser. Easy and hard bottleneck location problems.
     Discrete Applied Mathematics, 1:209–216, 1979. (Cited on p. 53)
133. F. K. Hwang, D. S. Richards, and P. Winter. The Steiner Tree Problem,
     volume 53 of Annals of Discrete Mathematics. North-Holland, Amsterdam,
     Netherlands, 1992. (Cited on p. 37)
134. O.H. Ibarra and C.E. Kim. Fast approximation algorithms for the knapsack
     and sum of subset problems. Journal of the ACM, 22:463–468, 1975. (Cited
     on p. 73)
                                                                References     363

135. R. Impagliazzo and D. Zuckerman. How to recycle random bits. In Proc.
     30st IEEE Annual Symposium on Foundations of Computer Science, pages
     248–253, 1989. (Cited on p. 332)
136. A. Iwainsky, E. Canuto, O. Taraszow, and A. Villa. Network decomposition
     for the optimization of connection structures. Networks, 16:205–235, 1986.
     (Cited on p. 37)
137. K. Jain. A factor 2 approximation algorithm for the generalized Steiner net-
     work problem. Combinatorica, 1:39–60, 2001. (Cited on p. 231)
138. K. Jain, M. Mahdian, and A. Saberi. A new greedy approach for facility
     location problems. Manuscript, 2001. (Cited on pp. 242, 254, 331)
139. K. Jain, I. I. Măndoiu, V.V. Vazirani, and D. P. Williamson. Primal–dual
     schema based approximation algorithms for the element connectivity problem.
     In Proc. 10th ACM-SIAM Annual Symposium on Discrete Algorithms, pages
     484–489, 1999. (Cited on p. 337)
140. K. Jain and V.V. Vazirani. An approximation algorithm for the fault tolerant
     metric facility location problem. In Proc. 3rd International Workshop on
     Approximation Algorithms for Combinatorial Optimization Problems, volume
     1913 of Lecture Notes in Computer Science. Springer-Verlag, Berlin, 2000.
     (Cited on p. 240)
141. K. Jain and V.V. Vazirani. Approximation algorithms for the metric facil-
     ity location and k-median problems using the primal–dual schema and La-
     grangian relaxation. Journal of the ACM, 48:274–296, 2001.          (Cited on
     pp. 242, 253, 254)
142. M. Jerrum and A. Sinclair. The Markov chain Monte Carlo method: an ap-
     proach to approximate counting. In D.S. Hochbaum, editor, Approximation
     Algorithms for NP-Hard Problems, pages 482–520. PWS Publishing, Boston,
     MA, 1997. (Cited on p. 305)
143. M. Jerrum, A. Sinclair, and E. Vigoda. A polynomial-time approximation
     algorithm for the permanent of a matrix with non-negative entries. Electronic
     Colloquium on Computational Complexity, pages TR00–079, 2000. (Cited on
     pp. 338, 340)
144. M.R. Jerrum. A very simple algorithm for estimating the number of k-
     colorings of a low-degree graph. Random Structures and Algorithms, 7, 1995.
     (Cited on p. 341)
145. M.R. Jerrum and A. Sinclair. Approximating the permanent. SIAM Journal
     on Computing, 18:1149–1178, 1989. (Cited on p. 305)
146. M.R. Jerrum and A. Sinclair. Polynomial time approximation algorithms for
     the Ising model. SIAM Journal on Computing, 22:1087–1116, 1993. (Cited
     on p. 342)
147. M.R. Jerrum, L.G. Valiant, and V.V. Vazirani. Random generation of combi-
     natorial structures from a uniform distribution. Theoretical Computer Science,
     43:169–188, 1986. (Cited on p. 303)
148. T. Jiang, M. Li, and D. Du. A note on shortest common superstrings with
     ﬂipping. Information Processing Letters, 44:195–199, 1992. (Cited on p. 67)
149. D.S. Johnson. Near-optimal bin packing algorithms. PhD thesis, Massachusetts
     Institute of Technology, Department of Mathematics, Cambridge, MA, 1973.
     (Cited on p. 77)
150. D.S. Johnson. Approximation algorithms for combinatorial problems. Journal
     of Computer and System Sciences, 9:256–278, 1974. (Cited on pp. 10, 26, 139)
364    References

151. J. Kahn, J.H. Kim, L. Lovász, and V.H. Vu. The cover time, the blanket
     time, and the Matthews bound. In Proc. 41st IEEE Annual Symposium on
     Foundations of Computer Science, pages 467–475, 2000. (Cited on p. 338)
152. M. Kaib and C.-P. Schnorr. The generalized Gauss reduction algorithm. Jour-
     nal of Algorithms, 21(3):565–578, 1996. (Cited on p. 288)
153. R. Kannan. Algorithmic geometry of numbers. In Annual Review of Computer
     Science, Vol. 2, pages 231–267. Annual Reviews, Palo Alto, CA, 1987. (Cited
     on p. 293)
154. R. Kannan. Minkowski’s convex body theorem and integer programming.
     Mathematics of Operations Research, 12(3):415–440, 1987. (Cited on p. 293)
155. R. Kannan, L. Lovász, and M. Simonovits. Random walks and an o∗ (n5 )
     volume algorithm for convex bodies. Random Structures and Algorithms, 11:1–
     50, 1997. (Cited on p. 338)
156. D. Karger. A randomized fully polynomial time approximation scheme for
     the all-terminal network reliability problem. SIAM Journal on Computing,
     29:492–514, 1999. (Cited on pp. 304, 305)
157. D. Karger, P. Klein, C. Stein, M. Thorup, and N. Young. Rounding algorithms
     for a geometric embedding of minimum multiway cut. In Proc. 29th ACM
     Symposium on the Theory of Computing, pages 668–678, 1999. (Cited on
     p. 167)
158. D. Karger, R. Motwani, and M. Sudan. Approximate graph coloring by
     semideﬁnite programming. Journal of the ACM, 45:246–265, 1998. (Cited on
     pp. 267, 269)
159. D. Karger and C. Stein. A new approach to the minimum cut problem. Journal
     of the ACM, 43(4):601–640, 1996. (Cited on p. 304)
160. H. Karloﬀ. Linear Programming. Birkhäuser, Boston, MA, 1991. (Cited on
     p. 107)
161. H. Karloﬀ. How good is the Goemans-Williamson MAX CUT algorithm.
     SIAM Journal on Computing, 29:336–350, 1999. (Cited on p. 268)
162. H. Karloﬀ and U. Zwick. A 7/8-approximation algorithm for MAX-3SAT?
     In Proc. 38th IEEE Annual Symposium on Foundations of Computer Science,
     pages 406–415, 1997. (Cited on p. 332)
163. N. Karmakar and R.M. Karp. An eﬃcient approximation scheme for the one-
     dimensional bin packing problem. In Proc. 23rd IEEE Annual Symposium on
     Foundations of Computer Science, pages 312–320, 1982. (Cited on p. 78)
164. R.M. Karp. Reducibility among combinatorial problems. In R.E. Miller and
     J.W. Thatcher, editors, Complexity of Computer Computations, pages 85–103.
     Plenum Press, New York, NY, 1972. (Cited on p. 10)
165. R.M. Karp and M. Luby. Monte Carlo algorithms for enumeration and reli-
     ability problems. In Proc. 24th IEEE Annual Symposium on Foundations of
     Computer Science, pages 56–64, 1983. (Cited on pp. 302, 305)
166. R.M. Karp, M. Luby, and N. Madras. Monte Carlo approximation algorithms
     for enumeration problems. Journal of Algorithms, 10:429–448, 1989. (Cited
     on p. 305)
167. A. Karzanov and L. Khachiyan. On the conductance of order Markov chains.
     Technical Report DCS 268, Rutgers University, 1990. (Cited on p. 340)
168. P.W. Kasteleyn. Graph theory and crystal physics. In F. Harary, editor,
     Graph Theory and Theoretical Physics, pages 43–110. Academic Press, New
     York, NY, 1967. (Cited on p. 338)
                                                                 References     365

169. S. Khuller, R. Pless, and Y.J. Sussmann. Fault tolerant k-center problems.
     Theoretical Computer Science, 242:237–245, 2000. (Cited on pp. 52, 53)
170. S. Khuller and B. Raghavachari. Improved approximation algorithms for uni-
     form connectivity problems. Journal of Algorithms, 21:434–450, 1996. (Cited
     on p. 336)
171. S. Khuller and V.V. Vazirani. Planar graph colourability is not self-reducible,
     assuming P = N P . Theoretical Computer Science, 88(1):183–190, 1991.
     (Cited on p. 351)
172. S. Khuller and U. Vishkin. Biconnectivity approximations and graph carvings.
     Journal of the ACM, 42, 2:214–235, 1994. (Cited on p. 228)
173. P. Klein, S. Rao, A. Agrawal, and R. Ravi. An approximate max-ﬂow min-cut
     relation for undirected multicommodity ﬂow, with applications. Combinator-
     ica, 15:187–202, 1995. (Cited on pp. 179, 197)
174. D.E. Knuth. The Art of Computer Programming. Vol. 2. Seminumerical Al-
     gorithms. Second edition. Addison-Wesley, Reading, MA, 1981. (Cited on
     p. 266)
175. A. Korkine and G. Zolotareﬀ. Sur les formes quadratiques. Math. Annalen,
     6:366–389, 1873. (Cited on p. 290)
176. M. Korupolu, C. Plaxton, and R. Rajaraman. Analysis of a local search heuris-
     tic for facility location problems. In Proc. 9th ACM-SIAM Annual Symposium
     on Discrete Algorithms, pages 1–10, 1998. (Cited on p. 253)
177. L. Kou, G. Markowsky, and L. Berman. A fast algorithm for Steiner trees.
     Acta Informatica, 15:141–145, 1981. (Cited on p. 37)
178. M.W. Krentel. The complexity of optimization problems. Journal of Computer
     and System Sciences, 36:490–509, 1988. (Cited on p. 351)
179. H.W. Kuhn. The Hungarian method for the assignment problem. Naval
     Research Logistics Quarterly, 2:83–97, 1955. (Cited on p. 129)
180. J. Lagarias. Worst case complexity bounds for algorithms in the the theory
     of integral quadratic forms. Journal of Algorithms, 1:142–186, 1980. (Cited
     on p. 292)
181. J. Lagarias, H.W. Lenstra, Jr., and C.-P. Schnorr. Korkin–Zolotarev bases
     and successive minima of a lattice and its reciprocal lattice. Combinatorica,
     10:333–348, 1990. (Cited on p. 293)
182. T. Leighton and S. Rao. Multicommodity max-ﬂow min-cut theorems and
     their use in designing approximation algorithms. Journal of the ACM, 46:787–
     832, 1999. (Cited on p. 197)
183. A.K. Lenstra, H.W. Lenstra, Jr., and L. Lovász. Factoring polynomials with
     rational coeﬃcients. Math. Ann., 261:513–534, 1982. (Cited on p. 292)
184. J.K. Lenstra, D.B. Shmoys, and É. Tardos. Approximation algorithms for
     scheduling unrelated parallel machines. Mathematical Programming, 46:259–
     271, 1990. (Cited on p. 145)
185. H.W. Lenstra, Jr. Integer programming with a ﬁxed number of variables.
     Mathematics of Operations Research, 8:538–548, 1983. (Cited on p. 78)
186. L.A. Levin. Universal sorting problems. Problemy Peredaci Informacii, 9:115–
     116, 1973. English translation in Problems of Information Transmission 9:265–
     266. (Cited on p. 10)
187. M. Li. Towards a DNA sequencing theory. In Proc. 31st IEEE Annual Sym-
     posium on Foundations of Computer Science, pages 125–134, 1990. (Cited
     on p. 26)
366    References

188. J. H. Lin and J. S. Vitter. Approximation algorithms for geometric median
     problems. Information Processing Letters, 44:245–249, 1992. (Cited on p. 251)
189. J. H. Lin and J. S. Vitter. ε-approximation with minimum packing constraint
     violation. In Proc. 24th ACM Symposium on the Theory of Computing, pages
     771–782, 1992. (Cited on p. 254)
190. N. Linial, E. London, and Y. Rabinovich. The geometry of graphs and some
     of its algorithmic applications. Combinatorica, 15:215–245, 1995. (Cited on
     pp. 196, 197, 266)
191. C.H.C. Little. An extension of Kasteleyn’s method of enumerating 1-factors
     of planar graphs. In D. Holton, editor, Proc. 2nd Australian Conference on
     Combinatorial Mathematics, volume 403 of Lecture Notes in Computer Sci-
     ence, pages 63–72. Springer-Verlag, Berlin, 1974. (Cited on p. 338)
192. L. Lovász. On the ratio of optimal integral and fractional covers. Discrete
     Mathematics, 13:383–390, 1975. (Cited on pp. 11, 26, 118)
193. L. Lovász. An Algorithmic Theory of Numbers, Graphs and Convexity. CBMS-
     NSF Regional Conference Series in Applied Mathematics, 50. SIAM, Philadel-
     phia, PA, 1986. (Cited on p. 291)
194. L. Lovász. Combinatorial Problems and Exercises. Second edition. North-
     Holland, Amsterdam–New York, 1993. (Cited on pp. 107, 339, 341)
195. L. Lovász and M.D. Plummer. Matching Theory. North-Holland, Amsterdam–
     New York, 1986. (Cited on pp. 8, 11, 107)
196. L. Lovász and A. Schrijver. Cones of matrices and set functions, and 0-1
     optimization. SIAM Journal on Optimization, 1:166–190, 1990. (Cited on
     p. 269)
197. A. Lubotzky, R. Phillips, and P. Sarnak. Ramanujan graphs. Combinatorica,
     8:261–277, 1988. (Cited on p. 332)
198. M. Luby and E. Vigoda. Approximately counting up to four. In Proc. 29th
     ACM Symposium on the Theory of Computing, pages 682–687, 1997. (Cited
     on p. 341)
199. C. Lund and M. Yannakakis. On the hardness of approximating minimization
     problems. Journal of the ACM, 41:960–981, 1994. (Cited on pp. 26, 332)
200. S. Mahajan and H. Ramesh. Derandomizing semideﬁnite programming based
     approximation algoirthms. In Proc. 36th IEEE Annual Symposium on Foun-
     dations of Computer Science, pages 162–169, 1995. (Cited on p. 268)
201. M. Mahdian, E. Markakis, A. Saberi, and V. V. Vazirani. A greedy facility lo-
     cation algorithm analyzed using dual ﬁtting. In Proc. 4th International Work-
     shop on Approximation Algorithms for Combinatorial Optimization Problems,
     volume 2129 of Lecture Notes in Computer Science. Springer-Verlag, Berlin,
     2001. (Cited on pp. 241, 242)
202. P. Matthews. Generating random linear extensions of a partial order. The
     Annals of Probability, 19:1367–1392, 1991. (Cited on p. 340)
203. L. McShine and P. Tetali. On the mixing time of the triangulation walk
     and other Catalan structures. Randomization methods in Algorithm Design,
     DIMACS-AMS, 43:147–160, 1998. (Cited on p. 340)
204. D. Micciancio. The shortest vector in a lattice is hard to approximate to
     within some constant. In Proc. 39th IEEE Annual Symposium on Foundations
     of Computer Science, pages 92–98, 1998. (Cited on p. 336)
205. M. Mihail. On coupling and the approximation of the permanent. Information
     Processing Letters, 30:91–95, 1989. (Cited on p. 305)
                                                              References     367

206. M. Mihail. Set cover with requirements and costs evolving over time. In
     International Workshop on Randomization, Approximation and Combinato-
     rial Optimization, volume 1671 of Lecture Notes in Computer Science, pages
     63–72. Springer-Verlag, Berlin, 1999. (Cited on p. 117)
207. J.S.B. Mitchell. Guillotine subdivisions approximate polygonal subdivisions:
     a simple polynomial-time approximation scheme for geometric TSP, k-MST,
     and related problems. SIAM Journal on Computing, 28:1298–1309, 1999.
     (Cited on p. 89)
208. B. Morris. Improved bounds for sampling contingency tables. In Interna-
     tional Workshop on Randomization, Approximation and Combinatorial Opti-
     mization, volume 1671 of Lecture Notes in Computer Science, pages 121–129.
     Springer-Verlag, Berlin, 1999. (Cited on p. 340)
209. R. Motwani and P. Raghavan. Randomized Algorithms. Cambridge University
     Press, Cambridge, UK, 1995. (Cited on p. 354)
210. J. Naor and L. Zosin. A 2-approximation algorithm for the directed multi-
     way cut problem. In Proc. 38th IEEE Annual Symposium on Foundations of
     Computer Science, pages 548–553, 1997. (Cited on p. 167)
211. M. Naor, L. Schulman, and A. Srinivasan. Splitters and near-optimal de-
     randomization. In Proc. 36th IEEE Annual Symposium on Foundations of
     Computer Science, pages 182–191, 1995. (Cited on p. 332)
212. G. Nemhauser and L. Wolsey. Integer and Combinatorial Optimization. John
     Wiley & Sons, New York, NY, 1988. (Cited on p. 107)
213. G.L. Nemhauser and L.E. Trotter. Vertex packings: structural properties and
     algorithms. Mathematical Programming, 8:232–248, 1975. (Cited on p. 124)
214. Y. Nesterov and A. Nemirovskii. Interior Point Polynomial Methods in Convex
     Programming. SIAM, Philadelphia, PA, 1994. (Cited on p. 268)
215. M.L. Overton. On minimizing the maximum eigenvalue of a symmetric matrix.
     SIAM J. on Matrix Analysis and Appl., 13:256–268, 1992. (Cited on p. 268)
216. C.H. Papadimitriou. Computational Complexity. Addison-Wesley, Reading,
     MA, 1994. (Cited on p. 351)
217. C.H. Papadimitriou and K. Steiglitz. Combinatorial Optimization: Algorithms
     and Complexity. Prentice-Hall, Englewood Cliﬀs, NJ, 1982.         (Cited on
     pp. 11, 107)
218. C.H. Papadimitriou and M. Yannakakis. Optimization, approximation, and
     complexity classes. Journal of Computer and System Sciences, 43:425–440,
     1991. (Cited on pp. 332, 351)
219. C.H. Papadimitriou and M. Yannakakis. The traveling salesman problem with
     distances one and two. Mathematics of Operations Research, 18:1–11, 1993.
     (Cited on p. 34)
220. M. Pinsker. On the complexity of a concentrator. In Proc. 7th Annual Tele-
     traﬃc Conference, pages 318/1–318/4, 1973. (Cited on p. 179)
221. J. Plesnı́k. A bound for the Steiner tree problem in graphs. Math. Slovaca,
     31:155–163, 1981. (Cited on p. 37)
222. V.R. Pratt. Every prime has a succinct certiﬁcate. SIAM Journal on Com-
     puting, 4:214–220, 1975. (Cited on p. 9)
223. H. J. Prömel and A. Steger. RNC-approximation algorithms for the Steiner
     problem. In Proc. Symposium on Theoretical Aspects of Computer Science,
     volume 1200 of Lecture Notes in Computer Science, pages 559–570. Springer-
     Verlag, Berlin, 1997. (Cited on p. 212)
368    References

224. M.O. Rabin. Probabilistic algorithms. In J.F. Traub, editor, Algorithms and
     Complexity, Recent Results and New Directions, pages 21–39. Academic Press,
     New York, NY, 1976. (Cited on p. 11)
225. P. Raghavan. Probabilistic construction of deterministic algorithms: approxi-
     mating packing integer programs. Journal of Computer and System Sciences,
     37:130–143, 1988. (Cited on p. 139)
226. S. Rajagopalan and V.V. Vazirani. On the bidirected cut relaxation for the
     metric Steiner tree problem. In Proc. 10th ACM-SIAM Annual Symposium
     on Discrete Algorithms, pages 742–751, 1999. (Cited on pp. 211, 335)
227. S. Rajagopalan and V.V. Vazirani. Primal–dual RNC approximation algo-
     rithms for set cover and covering integer programs. SIAM Journal on Com-
     puting, 28:526–541, 1999. (Cited on p. 118)
228. D. Randall and D.B. Wilson. Sampling spin conﬁgurations of an Ising system.
     In Proc. 10th ACM-SIAM Annual Symposium on Discrete Algorithms, pages
     S959–960, 1999. (Cited on p. 342)
229. S. Rao and W.D. Smith. Approximating geometrical graphs via “spanners”
     and “banyans”. In Proc. 30th ACM Symposium on the Theory of Computing,
     pages 540–550, 1998. (Cited on p. 89)
230. S.K. Rao, P. Sadayappan, F.K. Hwang, and P.W. Shor. The rectilinear Steiner
     arborescence problem. Algorithmica, 7:277–288, 1992. (Cited on p. 35)
231. R. Raz. A parallel repetition theorem. SIAM Journal on Computing, 27:763–
     803, 1998. (Cited on p. 332)
232. S.K. Sahni and T.F. Gonzalez. P-complete approximation problems. Journal
     of the ACM, 23:555–565, 1976. (Cited on p. 37)
233. H. Saran and V.V. Vazirani. Finding k-cuts within twice the optimal. SIAM
     Journal on Computing, 24:101–108, 1995. (Cited on p. 46)
234. C.P. Schnorr. Optimal algorithms for self-reducible problems. In Proc. 3rd
     International Colloquium on Automata, Languages, and Programming, pages
     322–337, 1976. (Cited on p. 351)
235. C.P. Schnorr. A hierarchy of polynomial time lattice basis reduction algo-
     rithms. Theoretical Computer Science, 53:201–224, 1987. (Cited on p. 292)
236. P. Schreiber. On the history of the so-called Steiner Weber problem. Wiss.
     Z. Ernst-Moritz-Arndt-Univ. Greifswald, Math.-nat.wiss. Reihe, 35, 3, 1986.
     (Cited on p. 37)
237. A. Schrijver. Theory of Linear and Integer Programming. John Wiley & Sons,
     New York, NY, 1986. (Cited on p. 107)
238. P.D. Seymour. Packing directed circuits fractionally. Combinatorica, 15:281–
     288, 1995. (Cited on p. 337)
239. D.B. Shmoys, É. Tardos, and K.I. Aardal. Approximation algorithms for
     facility location problems. In Proc. 29th ACM Symposium on the Theory of
     Computing, pages 265–274, 1997. (Cited on p. 242)
240. D.B. Shmoys and D.P. Williamson. Analyzing the Held-Karp TSP bound:
     a monotonicity property with applications. Information Processing Letters,
     35:281–285, 1990. (Cited on p. 231)
241. A. Sinclair. Improved bounds for mixing rates of Markov chains and multi-
     commodity ﬂow. Combinatorics, Probability and Computing, 1:351–370, 1992.
     (Cited on p. 197)
242. A. Sinclair. Algorithms for Random Generation and Counting: a Markov
     Chain Approach. Birkhäuser, Boston, MA, 1993. (Cited on p. 305)
                                                              References     369

243. J. Spencer. Ten Lectures on the Probabilistic Method. SIAM, Philadelphia,
     PA, 1987. (Cited on pp. 139, 354)
244. A. Srinivasan. Improved approximations of packing and covering problems.
     In Proc. 27th ACM Symposium on the Theory of Computing, pages 268–276,
     1995. (Cited on p. 124)
245. R.H. Swendsen and J.S. Wang. Non-universal critical dynamics in Monte
     Carlo simulations. Physics Review Letters, 58:86–90, 1987. (Cited on p. 342)
246. R.E. Tarjan. Data Structures and Network Algorithms. SIAM, Philadelphia,
     PA, 1983. (Cited on p. 11)
247. L. Trevisan. Non-approximability results for optimization problems on
     bounded degree instance. In Proc. 33rd ACM Symposium on the Theory of
     Computing, 2001. (Cited on p. 334)
248. J.D. Ullman. The performance of a memory allocation algorithm. Technical
     Report 100, Princeton University, Princeton, NJ, 1971. (Cited on p. 78)
249. L.G. Valiant. The complexity of computing the permanent. Theoretical Com-
     puter Science, 8:189–201, 1979. (Cited on p. 305)
250. L. Vandeberghe and S. Boyd. Semideﬁnite programming. SIAM Review,
     38:49–95, 1996. (Cited on p. 268)
251. V.V. Vazirani. NC algorithms for computing the number of perfect match-
     ings in K3,3 -free graphs and related problems. Information and Computation,
     80:152–164, 1989. (Cited on p. 338)
252. V.V. Vazirani and M. Yannakakis. Suboptimal cuts: their enumeration, weight
     and number. In Proc. 19th International Colloquium on Automata, Languages,
     and Programming, volume 623 of Lecture Notes in Computer Science, pages
     366–377. Springer-Verlag, Berlin, 1992. (Cited on p. 304)
253. D.L. Vertigan and D.J.A. Welsh. The computational complexity of the Tutte
     plane. Combinatorics, Probability and Computing, 1:181–187, 1992. (Cited
     on p. 342)
254. E. Vigoda. Improved bounds for sampling colorings. In Proc. 40th IEEE
     Annual Symposium on Foundations of Computer Science, pages 51–59, 1999.
     (Cited on p. 341)
255. V.G. Vizing. On an estimate of the chromatic class of a p-graph. Diskret.
     Analiz., 3:25–30, 1964 (in Russian). (Cited on p. 10)
256. D.J.A. Welsh. Knots, Colourings and Counting. Cambridge University Press,
     Cambridge, UK, 1993. (Cited on p. 342)
257. A. Wigderson. Improving the performance guarantee for approximate graph
     coloring. Journal of the ACM, 30:729–735, 1983. (Cited on p. 23)
258. D.P. Williamson, M.X. Goemans, M. Mihail, and V.V. Vazirani. A primal–
     dual approximation algorithm for generalized Steiner network problems. Com-
     binatorica, 15:435–454, 1995. (Cited on pp. 130, 224)
259. D. B. Wilson. Generating random spanning trees more quickly than the cover
     time. In Proc. 30th ACM Symposium on the Theory of Computing, pages
     296–303, 1996. (Cited on p. 339)
260. L.A. Wolsey. Heuristic analysis, linear programming and branch and bound.
     Mathematical Programming Study, 13:121–134, 1980. (Cited on pp. 231, 268)
261. M. Yannakakis. On the approximation of maximum satisﬁability. Journal of
     Algorithms, 3:475–502, 1994. (Cited on p. 139)
262. A.Z. Zelikovsky. An 11/6-approximation algorithm for the network Steiner
     problem. Algorithmica, 9:463–470, 1993. (Cited on p. 212)
370    References

263. A.Z. Zelikovsky and I. I. Măndoiu. Practical approximation algorithms for
     zero- and bounded-skew trees. In Proc. 12th ACM-SIAM Annual Symposium
     on Discrete Algorithms, pages 407–416, 2001. (Cited on p. 37)
Problem Index




2CNF≡ clause deletion     176, 179         Cover time 337
                                           Covering integer programs     112, 116,
Acyclic subgraph 7, 334                      118
Antichain cover 8                          Cycle cover 35, 62

Bandwidth minimization 196                 Dominating set   48, 50, 52
Betweenness 267
                                           Edge coloring 10
Bin covering 77
                                           Edge expansion 192
Bin packing 74, 74–78, 80, 124
                                           Enumerating cuts 304
– with ﬁxed number of object sizes    81
                                           Feedback edge set
Chain cover 8                              – directed 337
Clique 9, 306, 309, 318–322                – subset 166, 166, 167
Closest vector 292                         Feedback vertex set 25, 54, 54–60,
Clustering 243                               129, 166
– 22 253, 254                             – directed 337
– metric k-cluster 52                      – subset 166, 166, 167, 336
Counting problems 294–305
– acyclic orientations 338                 Graph bipartization by edge deletion
– antichains 340                             178
– bases of a matroid 339
– colorings of a graph 341                 Hamiltonian cycle   30, 303
– contingency tables 340
– DNF solutions 295, 305                   Independent set 48, 51–53
– – weighted version 302                   – maximal 239
– Euler tours 339
                                           Knapsack   68, 68–73
– forests 339
– graphs with given degree sequence        Linear equations over GF[2]    138
   340
– Hamiltonian cycles 341                   Matching 3, 104
– independent sets 341                     – b-matching 152, 227
– perfect matchings 305, 338               – bipartite 129
– simple cycles in a directed graph        – – maximum weight 129
   303                                     – maximal 3, 8
– stable marriages 340                     – – minimum cardinality 8
– trees 340                                – maximum 3, 5, 9, 124, 152, 153
– triangulations 340                       – minimum weight 107
– volume of a convex body 338              – perfect 105, 142, 143
372     Problem Index

– – minimum weight 32, 35, 62, 105,         – uniform parallel machines 140, 145
    230                                     Minimum spanning tree (MST)
Matroid intersection 228                       28–31, 105, 206, 207, 212
Matroid parity 212, 212                     Multicommodity ﬂow 97, 147, 163
MAX k-CUT 23, 138, 267, 269                 – demands 168, 180, 180–197
Maximum antichain 8                         – directed 165
Maximum coverage 25                         – integer 148, 153, 154, 337
Maximum cut (MAX-CUT) 10, 22,               – – in trees 146–154
   138, 255, 255, 256, 260–263, 267, 268,   – – in trees of height one 152
   334                                      – – in unit capacity trees 153
– directed 23, 138, 267, 269                – sum 168, 168–176, 179
Maximum ﬂow 38, 97, 97–100, 168             – uniform 192, 197
Maximum satisﬁability (MAX-SAT)             Multicut 146, 153, 168–179, 336
   9, 131, 131–139, 263, 306                – directed 337
– MAX k-FUNCTION SAT 312                    – in trees 146–154, 166
– MAX-2SAT 131, 263, 268                    – in trees of height one 152
– MAX-3SAT 131, 309, 311–315, 322,          Multiway cut 38, 38–40, 155–167, 335
   323, 326, 330, 331                       – bidirected integer program formula-
– – with bounded occurrence of                 tion 164
    variables 313–316, 330                  – directed 165, 166, 167
Metric k-center 47, 47–50, 53               – fractional 156
– fault-tolerant 52                         – node 160, 160–163, 166
– weighted 50, 50–52                        Network design
Metric k-median 243, 243–254, 337           – element connectivity 337
Metric k-MST 252                            – vertex connectivity 336
Metric facility location                    Network reliability 297, 304, 305, 339
– capacitated 240, 337                      – s–t reliability 339
– fault tolerant 240                        – global 339
– metric uncapacitated 242
– prize-collecting 240                      Point-to-point connection   208
– uncapacitated 232, 232–239, 242,          Satisﬁability (SAT) 9, 330, 343, 344
   337                                      – 3SAT 310, 343
Minimum k-connected subgraph                Scheduling on unrelated parallel
– edge 228                                    machines 140, 140–145
– vertex 226                                Semideﬁnite programming 258,
Minimum k-cut 38, 40–44                       255–269
Minimum bisection 193, 196, 197, 336        Set cover VIII, 11, 15, 15–26, 34,
Minimum chain cover 8                         108–122, 124–130, 239, 251, 306, 309,
Minimum cut 38, 298                           322–329, 334
– b-balanced 193, 193–194, 196, 197,        – constrained set multicover 112, 116,
   336                                        118
– s–t 38, 98, 97–100, 146                   – multiset multicover 112, 116, 117,
Minimum cut linear arrangement                123
   194, 194–195, 197                        – set multicover 24, 112, 116, 123
Minimum length linear arrangement           – with concave costs 117
   178                                      Shortest superstring 9, 20, 19–22, 26,
Minimum makespan scheduling 9, 10,            61–67
   79, 79–83, 140                           – variants 25, 67
                                                         Problem Index       373

Shortest vector 273, 273–293, 336       – asymmetric 34, 336
Sparsest cut 180, 180–197, 336, 337     – Euclidean 84, 84–89
Steiner arborescence                    – metric 30–33, 37, 229, 231, 334
– rectilinear 35                        – – lengths one and two 34
Steiner forest 198, 198–213             – – variants 34
Steiner network 213, 213–231, 335       Tutte polynomial 341
Steiner tree 27, 27–30, 33, 37, 198,
  213, 306, 309, 335
– directed 34, 337                      Vertex coloring 23
– Euclidean 89                          – k-coloring 267, 269
– prize-collecting 208, 252             Vertex cover 1, 15, 17–19, 23, 24, 104,
Subset sum 291                            122–124, 129, 146, 152, 166, 306, 307,
Subset-sum ratio problem 72               309, 334
Survivable network design see Steiner   – cardinality 1, 2–5, 8, 152
  network and network design
Traveling salesman problem (TSP)        Zero-skew tree
  30, 229, 231                          – rectilinear 36, 37
Subject Index




α-min cut 304                          co-NP 344
#P 294, 305                            co-RP 10, 330, 348
1-tree 230                             Complementary slackness conditions
                                         97, 100, 105, 125, 149, 161, 178, 199,
Active set 200, 209                      233
Approximation algorithm 2, 345–347     – relaxed 126, 129, 130, 146, 149, 199,
– approximation factor 346               234
– randomized 346                       Compression 64
Approximation scheme 68                Concave function 135
– fully polynomial randomized          Convex combination 258, 259
   (FPRAS) 295, 295, 297, 300, 302,    Convex set 259
   303, 305, 338–340                   Cost-eﬀectiveness of a set 16, 113
– fully polynomial time (FPTAS) 68,    Counting problems VII, 294–305,
   69–70, 72, 77, 83                     338–342
– polynomial time (PTAS) 68, 80–89,    – #P-complete VII, 294, 294, 305,
   140, 145, 311, 336                    338
– – asymptotic 75, 74–78               Covering LP 109
Arborescence 228                       Crossing sets 215, 219
Arithmetic-geometric mean inequality   Cut packing 183–191
   135                                 – approximate 184
                                       Cut requirement function 213
Basis of a lattice 274                 Cycle space 54
– Gauss reduced 281, 290               – cyclomatic number 54
– KZ reduced 290                       Cyclomatic weighted graphs 54–57
– Lovasz reduced 283
– weakly reduced 283, 290              Decision problem 343
Bernoulli trials 190, 353              – NP-complete 344
                                       – well-characterized 6, 5–7, 10, 93
Catalan numbers 86, 340                – Yes/No certiﬁcate
Certiﬁcate                             – – approximate 7
– co-NP 336                            Deﬁciency of a set 226
– Yes/No 5–7, 93, 96, 294, 343–344,    Demand graph 182
   348                                 Derandomization 132–134, 138,
– – approximate 274, 288                  248–250, 268
Chebyshev’s inequality 297, 353        Determinant of a lattice 274
Chernoﬀ bounds 9, 190, 353             Dilworth’s theorem 8
Christoﬁdes’ algorithm 37, 229, 334    Divide-and-conquer algorithm 179,
Chromatic polynomial 342                  193
376    Subject Index

DTIME 331, 332, 348                      Integrality gap 102, 101–103, 111,
Dual ﬁtting 101, 108–118, 241               129, 137, 151, 164, 167, 207, 210, 211,
Dual growing                                218, 229, 254, 262, 335, 337
– synchronized 198                       Integrality ratio see Integrality gap
Dual lattice 284, 284–288                Interactive proof systems 332
Dynamic programming 69, 81, 153          Ising model 342
                                         Isolating cut 38
Edge expansion 192
                                         Kirchhoﬀ’s theorem 339
Edge-disjoint s–t paths 103, 336
                                         Konig-Egervary theorem 5, 104
Eigenvalue 257
                                         Kruskal’s algorithm 105, 206
Eigenvector 257
Ellipsoid algorithm 170, 214, 255, 259
                                         Lagrangian relaxation 250–252
Euclid’s algorithm 273, 276–278          Laminar family of sets 219
Euler tour 28, 32                        Layering 17–19, 25, 57, 60, 129
Eulerian graph 28, 31                    Linearity of expectation 136, 352
Expander graph 175, 179, 192, 320,       Local search 23, 253
  332                                    Lower bounding OPT 2, 17, 31, 32, 39,
Expander graphs 314                         47, 62, 79, 89, 108, 206, 278–280
Extreme point solution 100, 102–104,     Lowest common ancestor 149
  119, 122, 141–145, 214, 219–221        LP-duality
                                         – theorem 6, 95, 93–97, 100, 106, 107,
First-ﬁt algorithm 74, 77                   148, 183
Flow-equivalent tree 44                  – – weak 96, 148, 169
Forward delete 153                       – theory 6, 17, 29, 97, 101, 108, 147
Frequency of an element 15, 119
Function                                 Mader’s theorem 227, 231
– degree-weighted 17                     Markov chain 192, 338, 339
– proper 208                             – conductance 192–193, 197
Fundamental cycle 54                     – Markov chain Monte Carlo method
                                           VIII, 294
                                         – rapidly mixing 305, 339, 340
Game
                                         – stationary probability distribution
– two-person zero-sum 106
                                           192
Gauss’ algorithm 273, 276–278, 288
                                         – Swendsen-Wang process 342
Gomory–Hu tree 40, 44, 46
                                         – transition matrix 192
Gram–Schmidt lower bound 287, 288
                                         Markov’s inequality 88, 353
Gram–Schmidt orthogonalization           Matroid 339
  278, 278–280, 282, 285                 – balanced 339
Greedy algorithm 8, 16–17, 24, 44, 60,   – basis exchange graph 339
  64, 72, 108, 138, 241                  – graphic 339
                                         – independent sets 212
Half-integrality 119, 122–124, 153,      Max-ﬂow min-cut theorem 97, 103,
  160–163, 165, 213–221                    168, 207
Hall’s theorem 144                       – approximate version for demands
Hamiltonian cycle 29, 214                  multicommodity ﬂow 191
Hardness of approximation VIII,          – approximate version for uniform
  306–333                                  multicommodity ﬂow 197
Hungarian method 129                     MAX-SNP-completeness 332
                                                            Subject Index       377

Maximum weight spanning tree 44         Primal–dual schema VII, 101,
Menger’s theorem 103                      125–130, 149–152, 235–236, 335
Method of conditional expectation       – with synchronization 199–204
   131–134, 138, 139, 248               Primitive root 9
Metric 183–191                          Primitive vector 275, 285, 286, 290
– 1 -embedding 183–191                 Principal submatrix 265
– – β-distortion 185                    Probabilistic argument 179
– – isometric 185, 186                  Probabilistic method 324
– 2 -embedding 196                     Probabilistically checkable proof system
– 22 -embedding                          (PCP) 309, 332
– – isometric 195                       – completeness 319
– – optimal distortion 197, 266         – parallel repetition 325–326
– p 185                                – soundness 319
Min–max relation 5–7, 11, 97–100, 168   – two-prover one round 322–324, 332
– approximate 7, 151                    Probability distribution
Minkovski’s theorem 287                 – binomial 354
Moments of a random variable 352        – normal 261, 266, 354
– central 352                           – Poisson 354
Monte Carlo sampling 297, 301           – spherically symmetric 261
                                        Probability theory 352–354
Near-minimum cuts 298–299               Pseudo-approximation algorithm
Next-ﬁt algorithm 77                      193–195, 197
Norm 185                                Pseudo-forest 143
– p 185                                Pseudo-polynomial time algorithm
NP 343                                    69, 69, 71–73
                                        Pseudo-tree 143
Odd set cover 6                         Quadratic forms 292
Optimization problem 2, 345, 351        Quadratic program 255
– NP-complete 10                        – strict 255, 255–257, 267, 268
– NP-hard 68, 344
– strongly NP-hard 71                   Random contraction algorithm 298,
Orthogonality defect 275, 279              304
Overlap graph 66                        Random walk 320, 338–340
                                        Reduction
P=NP conjecture VII, 10, 68, 71, 345   – L- 332, 351
Packing LP 110                          – approximation factor preserving
Parametric pruning 47–52, 140–141,         24, 27, 34, 60, 152, 160, 166, 196, 242,
   252                                     347, 351
Parsimonious property 229, 230          – gap-introducing 307
Partial ordering 8                      – gap-preserving 307
PCP theorem VIII, 306, 308–311, 323,    – polynomial time 344
   332                                  – randomized 293
Petersen graph 6, 214                   Region growing 171–175
Poisson trials 353                      Relaxation
Positive semideﬁnite matrix 257,        – convex 269
   257–258                              – exact 102
Potts model 342                         – – for maximum weight bipartite
Preﬁx graph 62                              matching 129
378     Subject Index

– – for MST 212, 230                         Supermodular function
– LP- VII, 39, 99, 100–106, 109, 111,        – weakly supermodular      216
   113, 119, 120, 122, 124, 125, 134, 147,
   153, 155–157, 160, 164, 165, 179, 199,    Throughput 180, 182
   206, 209, 211, 213–221, 224, 229–231,     Tight example IX, 4, 8, 17, 19, 23–25,
   233, 240, 244, 251, 335, 337                29, 31, 33, 39, 43, 49, 51, 59, 80, 83,
– – bidirected cut relaxation for Steiner      120, 123, 128, 137, 144, 153, 165, 175,
    tree 210, 335                              206, 218, 238, 239, 249, 268
– – subtour elimination relaxation for       Totally unimodular matrix 104
    TSP 229, 229–231                         Tournament 25
Reverse delete 149, 210                      Traveling salesman tour
– dynamic 209                                – maximum weight 66
Rounding VII, 101, 119–124, 134–136,         – minimum weight 62
   170–175, 191                              Triangle inequality 27, 51, 52, 178
– iterated 213, 217–218                      – directed 34
– randomized 120–122, 124, 157–160,
   164, 247–248, 260–263                     Unbiased estimator 295
RP 348                                       Uncrossable function 224
                                             Uniform generator 302, 303
Scaling and rounding 73, 117                 – almost uniform 303
Self-reducibility IX, 9, 10, 303,            Unimodular matrix 274, 274–276, 288
  348–351                                    Unit sphere 260
– tree 10, 303, 349                          Upper bounding OPT 256
Semideﬁnite program 197, 266, 267
– duality theory 268                         Vector program 256, 255–257, 266,
Separating hyperplane 259                      267
Separation oracle 102, 107, 170, 179,        Veriﬁer 309
  217                                        Vertex cover 316–318
Short-cutting 29, 31, 32, 85, 241            Vertex-disjoint s–t paths 103, 336
Simplex 155                                  VLSI design 178
Sparsity of a cut 181                        – clock routing 36
Spread of an edge 196                        von Neumann’s minimax theorem 106
Square of a graph 48
Standard deviation 352                       Witness family    225
Steiner tree 316–318
Sublattice 285, 290                          ZPP 10, 348
Submodular function 215, 224                 ZTIME 329, 332, 348
