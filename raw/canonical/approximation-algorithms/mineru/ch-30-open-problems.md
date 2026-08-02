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

# Open Problems (MinerU semantic layer)

<!-- source-pages: 352-396; printed-pages: 334-378; mineru-part: approximation-algorithms-ch-30-part-031 -->

## 30 Open Problems

This chapter is centered around problems and issues currently in vogue in the field of approximation algorithms. Important new issues are bound to arise in the future. With each of these problems two questions arise – that of obtaining the best approximation guarantee and a matching hardness result<sup>1</sup>

## 30.1 Problems having constant factor algorithms

Since a large number of important open problems in the field today involve improving the guarantee for problems for which we already know constant factor algorithms, we found it convenient to present them in a separate section. Of course, we are not looking for small improvements using incremental means. A good model is Goemans and Williamson’s improvement to the MAX-CUT problem, from factor $1 / 2$ to 0.878, which introduced semidefinite programming into the repertoire of techniques in this field. Most of the problems listed below have the potential of extending known methods in significant ways and introducing important new ideas.

Vertex cover, Problem 1.1: Improve on factor 2 (see algorithms in Chapters 1, 2, 14, and 15). Semidefinite programming may be a possible avenue, see, e.g., the attempt by Goemans and Kleinberg [103].

Set cover, Problem 2.1: This question generalizes the previous one. Consider the restriction of the set cover problem to instances in which the frequency of each element is bounded by a fixed constant f. Improve on factor $f$ (see algorithms in Chapters 2, 14, and 15). The best hardness result known is $\bar { f } ^ { 1 / 1 9 }$ , assuming $\mathbf { P } \neq \mathbf { N P }$ , due to Trevisan [247].

Acyclic subgraph, Problem 1.9: Improve on factor $1 / 2$ (see Exercise 1.1). Semidefinite programming may be applicable.

Metric TSP, Problem 3.5: As stated in Exercise 23.13, the solution produced by Christofides’ algorithm (Algorithm 3.10) is within a factor of

3/2 of the subtour elimination LP-relaxation for this problem. However, the worst integrality gap example known is (essentially) 4/3. Can a 4/3 factor algorithm be obtained using this relaxation?

Christofides’ algorithm consists of two steps: obtaining an MST and patching up its odd degree vertices. The above stated result follows by bounding the cost of each of these steps individually. It might be a good idea to first look for a “one–shot” factor 3/2 algorithm which compares the entire solution to the LP-relaxation. The primal–dual schema may hold the key.

Steiner tree, Problem 3.1: The best approximation guarantee known is essentially 5/3 (see Exercise 22.12). A promising avenue for obtaining an improved guarantee is to use the bidirected cut relaxation (22.7). This relaxation is exact for the minimum spanning tree problem. For the Steiner tree problem, the worst integrality gap known is (essentially) 8/7, due to Goemans (see Exercise 22.11). The best upper bound known on the integrality gap is 3/2 for quasi-bipartite graphs (graphs that do not contain edges connecting pairs of Steiner vertices), due to Rajagopalan and Vazirani [226]. Determine the integrality gap of this relaxation and obtain an algorithm achieving this guarantee<sup>2</sup>.

Recall that in contrast, LP-relaxation (22.2) has an integrality gap of (essentially) 2, not only for this problem, but also for its special case, the minimum spanning tree problem, and its generalization, the Steiner network problem.

Steiner network, Problem 23.1: Chapter 23 gives a factor 2 algorithm. However, it uses LP-rounding and has a prohibitive running time. Obtain a factor 2 combinatorial algorithm for this problem. A corollary of Algorithm 23.7 is that the integrality gap of LP-relaxation (23.2) is bounded by 2. Therefore, this relaxation can be used as a lower bound for obtaining a factor 2 combinatorial algorithm. The primal–dual schema appears to be the most promising avenue. A starting point may be determining if the following is true:

For each instance of the Steiner forest problem (and more generally, the Steiner network problem) there is an integral primal solution x and dual feasible solution y such that each edge picked by x is tight w.r.t. the dual y and each raised dual S has degree ≤ 2 (≤ 2f(S)). Observe that the dual found by Algorithm 22.3 can have arbitrarily high degree.

Multiway cut, Problem 4.1: A 1.5 factor is presented in Chapter 19. As stated, this can be improved to 1.3438. However, the worst integrality gap example known for LP-relaxation (19.1) is (essentially) 8/7. Determine the integrality gap of this relaxation, and obtain an algorithm achieving this guarantee. A diferent relaxation is presented in Exercise 19.7. How are the two relaxations related? Are they equivalent in that any feasible solution to one be converted to a solution of the other of the same cost?

Subset feedback vertex set, Problem 19.15: The best factor known is 8, via a fairly complicated algorithm (see Exercise 19.13). Is a factor 2 algorithm possible, matching several of the other related problems stated in Exercise 19.13?

## 30.2 Other optimization problems

Shortest vector, Problem 27.1: Obtain a polynomial factor algorithm for this problem. As shown in Chapter 27, the dual lattice helps give a factor n co-NP certificate for this problem. Is the dual lattice of further algorithmic use? The best hardness result known for this problem, of factor ${ \sqrt { 2 } } - \varepsilon$ , for any $\varepsilon > 0$ , assuming $\mathbf { R P } \neq \mathbf { N P }$ , is due to Micciancio [204].

Sparsest cut, Problem 21.2: The best approximation factor known is O(log n) (see Chapter 21). However, no hardness of approximation results have been established for this problem – as far as we know a PTAS not yet ruled out. Is there a constant factor algorithm or a PTAS for this problem?

Minimum b-balanced cut and minimum bisection cut, Problem 21.27: An $O ( \log ^ { 2 } n )$ factor algorithm for both these problems was given by Feige and Krauthgamer [83]. As in the case of sparsest cut, a PTAS is not yet ruled out for these problems. Is there a constant factor algorithm or a PTAS for these problems? When restricted to planar graphs, the minimum b-balanced cut problem, for $b \leq 1 / 3$ , can be approximated within a factor of 2, see Garg, Saran, and Vazirani [95].

Minimum multicut, 18.1: An O(log n) factor algorithm is given in Chapter 20. A long standing open problem is whether there is a constant factor deterministic algorithm for this problem.

Asymmetric TSP, Problem 3.15: The best factor known is $O ( \log n )$ (see Exercise 3.6). Is there a constant factor algorithm for this problem?

Vertex-connectivity network design: This variant of the Steiner network problem (Problem 23.1) asks for a minimum cost subgraph containing $r _ { u , v }$ vertex-disjoint paths, instead of edge-disjoint paths, for each pair of vertices $u , v \in V$ . No nontrivial approximation algorithms are known for this variant. For the special case when $r _ { u , v } = k$ for each pair of vertices $u , v \in V$ and the edge costs obey the triangle inequality, a $\textstyle { \left( { 2 + { \frac { 2 ( k - 1 ) } { n } } } \right) }$ factor algorithm is given by Khuller and Raghavachari [170]. A problem of intermediate dificulty is the element-connectivity network design problem, in which vertices are partitioned into two sets: terminals and non-terminals. Only edges and non-terminals, referred to as elements, can fail. Only pairs of terminals have connectivity requirements, specifying the number of element-disjoint paths required. An algorithm with an approximation guarantee of factor $2 H _ { k }$ , where k is the largest requirement, is given by Jain, M˘andoiu, Vazirani, and Williamson [139].

Maximum integer multicommodity flow, Problem 18.3: Example 18.8 shows that the natural LP-relaxation has an integrality gap of $\varOmega ( n )$ It is easy to get around this dificulty while still retaining the essence of the original problem by asking for a maximum half-integral flow. Is there an $O ( \log n )$ factor algorithm for this latter problem?

Metric uncapacitated facility location and k-median, Problems 24.1 and 25.1: Determine the integrality gaps of the LP-relaxations (24.2) and (25.2).

Capacitated facility location problem, Exercise 24.8: As stated in Exercise 24.8 the modification of LP (24.2) to this problem has unbounded integrality gap. Is there some other lower bounding method that leads to a good approximation algorithm?

Directed multicut and sparsest cut: In Chapters 20 and 21 we considered two generalizations of the undirected maximum flow problem and derived approximation algorithms for the corresponding cut problems, multicut and sparsest cut. Not much is known at present about analogous problems in directed graphs.

Directed Steiner tree, Problem 3.14: As shown in Exercise 3.3 this problem is unlikely to have a better approximation guarantee than O(log n). Is a guarantee of O(log n) possible? The best guarantee known is $n ^ { \varepsilon }$ for any fixed $\varepsilon > 0 .$ due to Charikar et. al. [37]. Generalizations of this problem to higher connectivity requirements, analogous to the Steiner network problem, also need to be studied.

Directed feedback edge (vertex) set: Given a directed graph $G =$ $( V , E )$ , a feedback edge (vertex) set is a set of edges (vertices) whose removal leaves an acyclic graph. The problem is to find the minimum cardinality such set. More generally, consider the weighted version in which the edges (vertices) have assigned weights, and we want to find the minimum weight such set. It is easy to see that the edge and vertex versions are inter-reducible via approximation factor preserving reductions. An O(log n log log n) factor approximation algorithm is known for the weighted version, due to Seymour [238]. Can this be improved to O(log n) or even a constant factor?

Cover time: Given an undirected graph $G = ( V , E )$ , the cover time starting at vertex $v \in V , C ( v )$ is the expected number of steps taken by a random walk on $G ,$ which starts at v and visits all vertices. The cover time of $G$ is ma $\varsigma _ { v \in V } C ( v )$ . Clearly, a randomized algorithm can estimate the cover time to any desired accuracy by empirically simulating the random walk many times and taking the average. Kahn, Kim, Lov´asz, and Vu [151] have given an $O ( ( \log \log n ) ^ { 2 } )$ factor deterministic algorithm for this problem. Is a constant factor deterministic algorithm possible?

## 30.3 Counting problems

For the problems presented below (other than graphs with given degree sequence and triangulations), the decision version is in P, the counting version is #P-complete, and the complexity of approximately counting the number of solutions is unresolved. The complexity of counting the number of graphs with given degree sequence and triangulations is open, though conjectured to be #P-complete.

Perfect matchings in general graphs: When restricted to planar graphs, this problem is polynomial time solvable using the classic algorithm of Kastelyn [168]. This result extends to $K _ { 3 , 3 } \cdot$ -free graphs (graphs that do not contain a subgraph homeomorphic to $K _ { 3 , 3 } )$ as well, see Little [191] and Vazirani [251]. A FPRAS is known for the restriction of this problem to bipartite graphs, which is the same as the problem of evaluating a $0 / 1$ permanent, due to Jerrum, Sinclair, and Vigoda [143] (more generally, this work gives a FPRAS for evaluating the permanent of a square matrix with nonnegative integer entries).

Volume of a convex body: Given a convex body in $\mathbf { R } ^ { n }$ via an oracle, the problem is to estimate its volume. A number of other counting problems can be reduced to this fundamental problem. The first FPRAS for this problem was given by Dyer, Frieze, and Kannan [67]. Although polynomial, the running time of this algorithm was exorbitant. It required $O ^ { * } ( n ^ { 2 3 } )$ oracle calls – the “soft-O” notation of $O ^ { * }$ suppresses factors of log n as well as ε, the error bound. The current best algorithm, due to Kannan, Lov´asz, and Simonovits [155] requires $O ^ { * } ( n ^ { 5 } )$ oracle calls and $O ^ { * } ( n ^ { 7 } )$ arithmetic operations. Can the running time be further improved?

Acyclic orientations: Count the number of acyclic orientations of a given undirected graph G. An orientation of the edges of G is acyclic if the resulting directed graph is acyclic. Several Markov chains on the set of acyclic orientations are known that asymptotically converge to the uniform distribution; however, none of them is known to be rapidly mixing. For instance, say that two orientations are adjacent if one can be obtained from the other by flipping directions of the edges incident at a source or a sink, where a source has all outgoing edges and a sink has all incoming edges. Do a random walk on this graph.

Forests: A forest in an undirected graph is a set of edges that contain no cycles. A maximal forest is a spanning tree (assume the graph is connected). Interestingly enough, the problem of counting the number of spanning trees in a graph is in P – this being one of the very few counting problems known to be polynomial time solvable. This follows as a consequence of the classic matrix tree theorem of Kirchhof, see [194]. It is worth remarking that elegant polynomial time algorithms are known for generating a random spanning tree in an undirected graph using rapidly mixing Markov chains, due to Aldous [3], Broder [34], and Wilson [259]. On the other hand, the complexity of approximately counting forests in arbitrary graphs is open. The case of dense graphs (each vertex having degree at least αn, for 0 < α < 1) is handled by Annan [8]. Forests and spanning trees are the independent sets and bases, respectively, of the graphic matroid of the given graph.

Bases of a matroid: Given an arbitrary matroid via an independence oracle, count the number of bases. Define the basis exchange graph of a matroid as follows. Its vertices are all bases. Two bases are adjacent if their symmetric diference is two elements. The Markov chain defined by a random walk on the basis exchange graph is conjectured to be rapidly mixing by Dagum, Luby, Mihail, and Vazirani [56]. If so, a FPRAS for approximately counting the number of bases will follow. Examples of matroids for which this conjecture has been positively resolved are graphic matroids (see previous problem) and their generalization, balanced matroids. For the latter result, see Feder and Mihail [78]. A positive resolution of this question will also resolve the question of approximately counting forests (since forests of any particular size are bases of a truncation of the graphic matroid).

Network reliability: Many versions of the network reliability problem have found practical applications and have been studied in the past. Two basic versions for undirected graphs with edge failure probabilities are s–t reliability, which asks for the probability that special vertices s and t get disconnected, and global reliability, which asks for the probability that any part of the graph gets disconnected. One can define two analogous problems in directed graphs as well. Of these four problems, only undirected global reliability is settled – a FPRAS for this version is presented in Chapter 28. In addition, for each of these four cases one can also ask for the probability that s–t or the entire graph remain connected. This version is open even for the undirected global case.

Euler tours: Count the number of Euler tours of a given undirected graph (a connected graph is Eulerian if all vertices have even degrees). Interestingly enough, there is a polynomial time algorithm for the analogous problem for directed graphs – again following from Kirchhof’s Theorem.

Trees: Given an undirected graph G, count the number of subgraphs of G that are trees.

Antichains in a partial order: See Exercise 1.7 for the definition. For the related problem of counting the number of total orders consistent with a partial order, a FPRAS is known, due to Matthews [202], Karzanov and Khachian [167], and Bubley and Dyer [35].

Graphs with given degree sequence: Given n nonnegative integers $d _ { 1 } ,$ $\cdots , d _ { n } ,$ which represent the degrees of the n vertices, $v _ { 1 } , \ldots , v _ { n } $ , of a simple graph, count the number of such graphs. A related problem is to count the number of connected graphs having this degree sequence. In both cases, the question of existence of one such graph can be solved in polynomial time using a matching algorithm. If G is restricted to be a bipartite graph, with the bipartition specified, then a FPRAS follows from that for 0/1 permanents [143].

Contingency tables: Given the row sums and column sums of an m × n matrix with nonnegative integer entries, count the number of such matrices. A FPRAS is known if the row sums and column sums are all suficiently large, being at least $( m + n ) m n$ , due to Dyer, Kannan, and Mount [64]. Morris [208] improves this to the case where each row sum is $\varOmega ( n ^ { 3 / 2 }$ m log m) and each column sum is $\varOmega ( m ^ { 3 / 2 } n \log n )$ . If the matrices are constrained to be $0 / 1$ , this is same as the degree sequence problem restricted to bipartite graphs, for which a FPRAS follows from that for 0/1 permanents [143].

Triangulations: Compute the number of triangulations of n points on the plane, i.e., the number of ways of putting down non–intersecting line segments connecting pairs of points so that all internal faces are triangles. Consider the graph G on all possible triangulations whose edges are defined as follows: Remove an edge in a triangulation t that is not on the infinite face. If the resulting quadrilateral is convex, let t<sup></sup> be the triangulation obtained by adding an edge connecting the other two points of this quadrilateral. Then, G has an edge connecting t and t<sup></sup>. A random walk on this graph is conjectured to be rapidly mixing. If the n points form the vertices of a convex n-gon, then the number of triangulations is known to be the Catalan number $C _ { n - 2 }$ , and hence polynomial time computable. For this special case, the Markov chain defined above is known to be rapidly mixing, see McShine and Tetali [203].

Stable marriages: An instance of the stable marriage problem consists of n boys and n girls, together with an ordered list of the preferences of each boy and each girl (each boy orders all n girls and each girl orders all n boys). A marriage is a perfect matching of the boys and girls. Boy b and girl g who are not married to each other are said to form a rogue couple if b prefers g to the girl he is married to and g prefers b to the boy she is married to. The marriage is stable if there are no rogue couples. The complexity of approximately counting the number of stable marriages is unresolved. For numerous structural properties of the set of stable marriages, see Gusfield and Irving [119].

Colorings of a graph: Consider an undirected graph $G ~ = ~ ( V , E )$ with maximum degree ∆. Jerrum [144] gave a FPRAS for counting the number of valid k-colorings of G for any $k > 2 \varDelta$ , and Vigoda [254] extended this to any $k > 1 1 \varDelta / 6$ . Can this be improved to counting the number of valid k-colorings of $G$ for any $k \geq \varDelta + 2 ?$ (If the number of colors is $\leq \varDelta + 1$ then the natural Markov chain, that at each step picks a random vertex and recolors it with a random consistent color, may not be connected.) This quantity finds applications in statistical physics.

Hamiltonian cycles: If each vertex of an undirected graph G has degree at least $n / 2$ then $G$ must have a Hamiltonian cycles (see Dirac’s condition in $\left[ 1 9 4 \right] )$ . If the minimum degree is $( 1 / 2 + \varepsilon ) n$ , for $\varepsilon > 0$ , Dyer, Frieze, and Jerrum [65] have given a FPRAS for this problem. Can this be extended to $\varepsilon = 0$ , i.e., graphs having minimum degree $n / 2 ?$

Independent sets: For graphs having $\varDelta = 4 ,$ , a FPRAS was given by Luby and Vigoda [198], where ∆ denotes the maximum degree of the graph. Dyer, Frieze, and Jerrum [66] show that the problem is not approximable for $\varDelta \geq 2 5$ , assuming $\mathbf { R P } \neq \mathbf { N P }$ . They also give an argument to show that the Markov chain Monte Carlo is unlikely to succeed for $\varDelta \geq 6$ Besides the question of $\varDelta = 5$ , this leaves the question of determining whether other methods will work for $6 \leq \varDelta \leq 2 4$ or whether these cases are also inapproximable.

Tutte polynomial: Several of the problems stated above are special cases of evaluating the Tutte polynomial of the given graph $G = ( V , E )$ at a particular point of the (x, y)-plane. For $A \subseteq E$ , define the rank of A, denoted $r ( A )$ , to be $| V | - k ( A )$ , where $k ( A )$ is the number of connected components in the graph having vertex set V and edge set A. The Tutte polynomial of G at point $( x , y )$ is

$$
T (G; x, y) = \sum_ {A \subseteq E} (x - 1) ^ {r (E) - r (A)} (y - 1) ^ {| A | - r (A)}.
$$

Some of the natural quantities captured by this polynomial are:

$\mathrm { A t } \ ( 1 , 1 ) , T$ counts the number of spanning trees in $G _ { \ l }$

• At (2, 1), T counts the number of forests in G.

$\mathrm { A t } \ ( 1 , 2 ) , T$ counts the number of connected subgraphs of $G _ { \ l }$

• At (2, 0), T counts the number of acyclic orientations of G.

• At (0, 2), T counts the number of orientations of G that form a strongly connected digraph.

• The chromatic polynomial of G is given by

$$
P (G, \lambda) = (- 1) ^ {r (E)} \lambda^ {k (E)} T (G; 1 - \lambda , 0),
$$

where $P ( G , \lambda )$ is the number of colorings of $G$ using λ colors.

• If the failure probability of each edge is $p ,$ then the probability that G remains connected is given by

$$
R (G; p) = q ^ {| E | - r (E)} p ^ {r (E)} T (G; 1, 1 / (1 - p)).
$$

Vertigan and Welsh [253] have shown that other than a few special points and two special hyperbolae (see next problem for definition), the exact evaluation of the Tutte polynomial is $\# \mathbf { P - h a r d }$ . The question of designing FPRAS’s is wide open. Say that a graph is α-dense if each vertex has degree $\geq \alpha n$ , where $0 < \alpha < 1$ . Annan [7] and Alon, Frieze, and Welsh [5] have given FPRAS’s for α-dense graphs for the cases $y = 1 , x \ge 1$ and $y > 1 , x \ge 1$ , respectively.

Partition functions of the Ising and Potts models: The hyperbolae $H _ { \alpha }$ defined by

$$
H _ {\alpha} = \{(x, y) \mid (x - 1) (y - 1) = \alpha \}
$$

play a special role in the context of the Tutte polynomial. In particular, along $H _ { 2 } , T$ gives the partition function of the Ising model for $G ,$ and along $H _ { Q }$ , for integer $Q \geq 2$ , T gives the partition function of the Potts model for $G .$ Both these quantities find use in statistical physics; see Welsh [256] for precise definitions and further details (the points on each hyperbola are parametrized by “temperature” and $Q$ represents the number of “color” classes). Jerrum and Sinclair [146] gave a FPRAS for estimating, at any temperature, the partition function of the Ising model of a graph, and Randall and Wilson [228] extended this to a polynomial time sampling procedure. However, because of large exponents in the running times, these algorithms are not practical. The Swendsen-Wang process [245] provides a natural and practically used Markov chain for estimating these quantities. This leads to the question of determining, formally, whether this chain is rapidly mixing. A negative result was provided by Gore and Jerrum [112] who show that this chain is not rapidly mixing for the complete graph, $K _ { n } ,$ for $Q \geq 3$ . Positive results for certain classes of graphs were provided by Cooper and Frieze [53]. Is this chain rapidly mixing for the partition function of the Ising model for an arbitrary graph? Is there some other way of estimating the partition function of the Potts model for an arbitrary graph?

# A An Overview of Complexity Theory for the Algorithm Designer

## A.1 Certificates and the class NP

A decision problem is one whose answer is either “yes” or $" \mathrm { n o } ^ { \mathrm { 7 } }$ . Two examples are:

SAT: Given a Boolean formula in conjunctive normal form, $f ,$ is there is a satisfying truth assignment for $f ?$

Cardinality vertex cover: Given an undirected graph G and integer $k ,$ does G have a vertex cover of size $\leq k ?$

For any positive integer k, we will denote by kSAT the restriction of SAT to instances in which each clause contains at most k literals.

It will be convenient to view a decision problem as a language, i.e., a subset of $\{ 0 , 1 \} ^ { * }$ . The language consists of all strings that encode $\mathrm { ^ { 6 6 } y e s ^ { 9 } }$ instances of the decision problem. A language $L \in \bf N P$ if there is a polynomial p and a polynomial time bounded Turing machine M, called the verifier, such that for each string $x \in \{ 0 , 1 \} ^ { * }$

• if $\mathbf { { \boldsymbol { x } } } \in L ,$ then there is a string y (the certificate) of polynomially bounded length, $\mathrm { i . e . , } | y | \leq p ( | x | )$ , such that $M ( x , y )$ accepts, and

• if x /∈ L, then for any string y, such that $| y | \leq p ( | x | ) , M ( x , y )$ rejects.

![](images/1eee8ffe787d58a66682317bdd84bdd42c4407fe8309815e1301f90fbfb4f79a.jpg)

String y that helps ascertain that x is a $\mathrm { ^ { 6 6 } y e s ^ { 5 9 } }$ instance will be called a Yes certificate. We will also refer to y as a proof or a solution; in the context of randomized computation, it is also referred to as a witness. Thus, NP is the class of languages that have “short, quickly verifiable” Yes certificates.

For example, the verifier for cardinality vertex cover assumes that y specifies a subset of the vertices. It checks whether this subset is indeed a vertex cover and is of the desired size bound. (Observe that no claim has been made about the time needed to actually find such a certificate.) It is also easy to see that the class NP defined above is precisely the class of languages that are decidable by nondeterministic polynomial time Turing machines (see Section A.6 for references), hence the name.

A language L belongs to the class co-NP if $\overline { { L } } \in \mathbf { N P }$ . Thus, co-NP is the class of languages that have “short, quickly verifiable” No certificates. For instance, let L be the language consisting of all prime numbers. This language allows No certificates: a factorization for number n is proof that n /∈ L. Hence $L \in \mathrm { c o - N P }$ . Interestingly enough, $\boldsymbol { L } \in \mathbf { N P }$ as well (see Exercise 1.13), though it is not known to belong to P.

## A.2 Reductions and NP-completeness

Next, let us introduce the crucial notion of a polynomial time reduction. Let $L _ { 1 }$ and $L _ { 2 }$ be two languages in NP. We will say that $L _ { 1 }$ reduces to $L _ { 2 } .$ , and write $L _ { 1 } \preceq L _ { 2 }$ , if there is a polynomial time Turing machine $T$ which given a string $x \in \{ 0 , 1 \} ^ { * }$ , outputs string y such that $x \in L _ { 1 }$ if $y \in L _ { 2 } .$ . In general, $T$ does not have to decide whether x is a “yes” or $\mathrm { ~ a ~ } ^ { 6 6 } \mathrm { n o } ^ { 5 5 }$ instance in order to output y. Clearly, if $L _ { 1 } \preceq L _ { 2 }$ and $L _ { 2 }$ is polynomial time decidable, then so is $L _ { 1 }$

A language L is NP-hard if for every language $L ^ { \prime } \in \mathbf { N P }$ $L ^ { \prime } \preceq L$ . A language L is NP-complete if $\boldsymbol { L } \in \mathbf { N P }$ , and L is NP-hard. An NP-complete language L is a hardest language in NP, in the sense that a polynomial time algorithm for L implies a polynomial time algorithm for every language in $\mathbf { N P } , \mathrm { i . e . }$ , it implies $\mathbf { P } = \mathbf { N P }$

The central theorem of complexity theory gives a proof of NP-hardness for a natural problem, namely SAT. The idea of the proof is as follows. Let L be an arbitrary language in NP. Let M be a nondeterministic polynomial time Turing machine that decides L, and let p be the polynomial bounding the running time of M. The proof involves showing that there is a deterministic polynomial time Turing machine T, that “knows” M and $p ,$ , and given a string $x \in \{ 0 , 1 \} ^ { * }$ , outputs a SAT formula f such that each satisfying truth assignment of $f$ encodes an accepting computation of M on input x. Thus, $f$ is satisfiable if there is an accepting computation of M on input $x ,$ i.e., if $x \in L$

Once one problem, namely SAT, has been shown to be NP-hard, the hardness of other natural problems can be established by simply giving polynomial time reductions from SAT to these problems (see Exercise 1.11). Perhaps the most impressive feature of the theory of NP-completeness is the ease with which the latter task can be accomplished in most cases, so that with relatively little work, a lot of crucial information is obtained. Other than a handful of (important) problems, most natural problems occurring in NP have been classified as being either in P or being NP-complete. Indeed, it is remarkable to note that other basic complexity classes, defined using notions of time, space and nondeterminism, also tend to have natural complete problems (under suitably defined reducibilities).

Establishing NP-hardness for vertex cover involves giving a polynomial time algorithm that, given a SAT formula $f ,$ outputs an instance $( G , k )$ such that G has a vertex cover of size ≤ k if f is satisfiable. As a corollary, we get that under the assumption $\mathbf { P } \neq \mathbf { N P } ,$ , there is no polynomial time algorithm that can distinguish $\mathrm { ^ { 6 6 } y e s ^ { 5 } }$ instances of vertex cover from $" \mathrm { n o } ^ { \mathrm { 3 } \mathrm { 5 } }$ instances. As stated above, this also shows that if $\mathbf { P } \neq \mathbf { N P }$ , there is no polynomial time algorithm for solving vertex cover exactly.

Considering the large and very diverse collection of NP-complete problems, none of which has yielded to a polynomial time algorithm for so many years, it is widely believed that $\mathbf { P } \neq \mathbf { N P } , \mathrm { i . e . }$ ., that there is no polynomial time algorithm for deciding an NP-complete language.

The $\mathbf { P } \neq \mathbf { N P }$ conjecture has a deep philosophical point to it. The conjecture asserts that the task of finding a proof for a mathematical statement is qualitatively harder than the task of simply verifying the correctness of a given proof for the statement. To see this, observe that the language

$L = \{ ( S , 1 ^ { n } )$ | statement S has a proof of length $\leq n \}$

is in NP, assuming any reasonable axiomatic system.

## A.3 NP-optimization problems and approximation algorithms

Combinatorial optimization problems are problems of picking the “best” solution from a finite set. An NP-optimization problem, $\pi _ { \ast }$ , consists of:

• A set of valid instances, $D _ { \pi }$ , recognizable in polynomial time. We will assume that all numbers specified in an input are rationals, since our model of computation cannot handle infinite precision arithmetic. The size of an instance $I \in D _ { \pi }$ , denoted by $| I | ,$ is defined as the number of bits needed to write I under the assumption that all numbers occurring in the instance are written in binary.

• Each instance $I \in D _ { \pi }$ has a set of feasible solutions, $S _ { \pi } ( I )$ . We require that $S _ { \pi } ( I ) \neq \emptyset$ , and that every solution $s \in S _ { \pi } ( I )$ is of length polynomially bounded in $| I |$ . Furthermore, there is polynomial time algorithm that, given a pair $( I , s )$ , decides whether $s \in S _ { \pi } ( I )$

• There is a polynomial time computable objective function, $\mathrm { \ o b j } _ { \varPi }$ , that assigns a nonnegative rational number to each pair $( I , s )$ , where I is an instance and s is a feasible solution for I. The objective function is frequently given a physical interpretation, such as cost, length, weight, etc.

• Finally, Π is specified to be either a minimization problem or a maximization problem.

The restriction of Π to unit cost instances will be called the cardinality version of Π.

An optimal solution for an instance of a minimization (maximization) problem is a feasible solution that achieves the smallest (largest) objective function value. $\mathrm { O P T } _ { \pi } ( I )$ will denote the objective function value of an optimal solution to instance I. We will shorten this to OPT when it is clear that we are referring to a generic instance of the particular problem being studied.

With every NP-optimization problem, one can naturally associate a decision problem by giving a bound on the optimal solution. Thus, the decision version of NP-optimization problem Π consist of pairs $( I , B )$ , where I is an instance of Π and B is a rational number. $\operatorname { I f } \ \pi$ is a minimization (maximization) problem, then the answer to the decision version is $\mathrm { ^ { 6 6 } y e s ^ { 9 } }$ if there is a feasible solution to I of cost $\leq B \ ( \geq B )$ ). If so, we will say that $( I , B )$ is a $\mathrm { ^ { 6 6 } y e s ^ { 9 } }$ instance; we will call it $\mathrm { ~ a ~ } ^ { 6 6 } \mathrm { n o } ^ { 5 7 }$ instance otherwise. For example, the decision version of cardinality vertex cover is stated in Section A.1.

Clearly, a polynomial time algorithm for Π can help solve the decision version – by computing the cost of an optimal solution and comparing it with B. Conversely, hardness established for the decision version carries over to Π. Indeed hardness for an NP-optimization problem is established by showing that its decision version is NP-hard. With a slight abuse of notation, we will also say that the optimization version is NP-hard.

An approximation algorithm produces a feasible solution that is “close” to the optimal one, and is time eficient. The formal definition difers for minimization and maximization problems. Let Π be a minimization (maximization) problem, and let δ be a function, $\delta : { \bf Z } ^ { + }  { \bf Q } ^ { + }$ , with $\delta \geq 1$ $( \delta \leq 1 )$ . An algorithm $\mathcal { A }$ is said to be a factor δ approximation algorithm for Π if, on each instance $I , A$ produces a feasible solution s for I such that $f _ { \mathit { T } } ( I , s ) \leq \delta ( | I | ) \cdot \mathrm { O P T } ( I ) \ ( f _ { \mathit { T } } ( I , s ) \geq \delta ( | I | ) \cdot \mathrm { O P T } ( I ) )$ , and the running time of A is bounded by a fixed polynomial in $| I |$ . Clearly, the closer $\delta$ is to 1, the better is the approximation algorithm.

On occasion we will relax this definition and will allow A to be randomized, $\mathrm { i . e . } .$ it will be allowed to use the flips of a fair coin. Assume we have a minimization problem. Then we will say that A is a factor $\delta$ randomized approximation algorithm for Π if, on each instance I, A produces a feasible solution s for I such that

$$
\mathbf {P r} [ f _ {\Pi} (I, s) \leq \delta (| I |) \cdot \mathrm{OPT} (I) ] \geq \frac {1}{2},
$$

where the probability is over the coin flips. The running time of A is still required to be polynomial in $| I |$ . The definition for a maximization problem is analogous.

Remark A.1 Even though δ has been defined to be a function of the size of the input, we will sometimes pick δ to be a function of a more convenient parameter. For instance, for the set cover problem (Chapter 2), we will pick this parameter to be the number of elements in the ground set.

## A.3.1 Approximation factor preserving reductions

Typically, polynomial time reductions map optimal solutions to optimal solutions; however, they do not preserve near-optimality of solutions. Indeed, all NP-complete problems are equally hard from the viewpoint of obtaining exact solutions. However, from the viewpoint of obtaining near-optimal solutions, they exhibit the rich set of possibilities alluded to earlier.

In this book we will encounter pairs of problems which may look quite diferent superficially, but whose approximability properties are closely linked (e.g., see Exercise 19.13). Let us define a suitable reducibility in order to formally establish such connections. Several reductions have been defined that preserve constant factor approximability. The reducibility stated below is a stringent version of these, and actually preserves the constant itself. Pair of problems that are linked in this manner are either both minimization problems or both maximization problems.

Let $\varPi _ { 1 }$ and $\varPi _ { 2 }$ be two minimization problems (the definition for two maximization problems is quite similar). An approximation factor preserving reduction from $\varPi _ { 1 }$ to $\varPi _ { 2 }$ consists of two polynomial time algorithms, $f$ and $^ { g , }$ such that

• for any instance $I _ { 1 }$ of $\varPi _ { 1 } , \ I _ { 2 } \ = \ f ( I _ { 1 } )$ is an instance of $\varPi _ { 2 }$ such that $\mathrm { O P T } _ { \pi _ { 2 } } ( I _ { 2 } ) \leq \mathrm { O P T } _ { \pi _ { 1 } } ( I _ { 1 } )$ , and

• for any solution t of $I _ { 2 } , s = g ( I _ { 1 } , t )$ is a solution of $I _ { 1 }$ such that

$$
\operatorname{obj} _ {\Pi_ {1}} (I _ {1}, s) \leq \operatorname{obj} _ {\Pi_ {2}} (I _ {2}, t).
$$

It is easy to see that this reduction, together with an $\alpha$ factor algorithm for $\varPi _ { 2 }$ , gives an α factor algorithm for $\varPi _ { 1 }$ (see Exercise 1.16).

## A.4 Randomized complexity classes

Certain NP languages<sup>1</sup> are characterized by the fact that they possess an abundance of Yes certificates, which renders them essentially tractable, assuming availability of a source of random bits. Such languages belong to the class RP, short for Randomized Polynomial Time. A language $L \in \mathbf { R P }$ if there is a polynomial p and a polynomial time bounded Turing machine M such that for each string $x \in \{ 0 , 1 \} ^ { * }$ :

• if $\mathbf { { \boldsymbol { x } } } \in L ,$ , then $M ( x , y )$ accepts for at least half the strings y of length $p ( | x | )$ and

• if $x \notin L ,$ then for any string y of length $p ( | x | ) , M ( x , y )$ rejects.

Clearly, $\mathbf { P } \subseteq \mathbf { R } \mathbf { P } \subseteq \mathbf { N P }$ . Suppose language $L \in \mathbf { R P }$ . On input $x ,$ we will pick a random string, y, of length $p ( | x | )$ and will run $M ( x , y )$ . Clearly, the entire computation takes polynomial time. We may erroneously reject x even though $x \in L$ . However, the probability of this is at most $1 / 2$ Let us call this the error probability. By the usual trick of making repeated independent runs, we can reduce the error probability to inverse exponential in the number of runs.

A language L belongs to the class co-RP if $\overline { { L } } \in \mathbf { R P }$ . Such a language has an abundance of No certificates. The corresponding machine may make an error on inputs $x \notin L$ . Finally, let us define ZPP, short for Zero-error Probabilistic Polynomial Time, to be the class of languages for which there is a randomized Turing machine $( \mathrm { i . e . , a }$ Turing machine equipped with a source of random bits) that always terminates with the correct answer and whose expected running time is polynomial. It is easy to see (Exercise 1.17) that

$$
L \in \mathbf {Z P P} \text {   iff   } L \in (\mathbf {R P} \cap \mathrm{co-RP}).
$$

DTIME(t) denotes the class of problems for which there is a deterministic algorithm running in time $O ( t )$ . Thus, $\mathbf { P } \ = \ \mathbf { D T I M E } ( p o l y ( n ) )$ • where $p o l y ( n ) \ = \ \bigcup _ { k > 0 } n ^ { k }$ $\mathbf { Z T I M E } ( t )$ denotes the class of problems for which there is a randomized algorithm running in expected time $O ( t )$ . Thus, ${ \bf Z P P } = { \bf Z T I M E } ( p o l y ( n ) )$ .

## A.5 Self-reducibility

Most known problems in NP exhibit an interesting property, called selfreducibility, which yields a polynomial time algorithm for finding a solution (a Yes certificate), given an oracle for the decision version. A slightly more elaborate version of this property yields an exact polynomial time algorithm for an NP-optimization problem, again given an oracle for the decision version. In a sense this shows that the dificult core of NP and NP-optimization problems is their decision versions (see Section 16.2 and Exercise 28.7 for other fundamental uses of self-reducibility).

Perhaps the simplest setting to describe self-reducibility is SAT. Let φ be a SAT formula on n Boolean variables $x _ { 1 } , \ldots , x _ { n }$ . We will represent a truth assignment to these n variables as n-bit $0 / 1$ vectors (True = 1 and False $= 0 )$ . Let S be the set of satisfying truth assignments, i.e., solutions, to $\phi .$

The important point is that for the setting of $x _ { 1 }$ to $0 ~ ( 1 )$ , we can find, in polynomial time, a formula φ<sub>0</sub> $\left( \phi _ { 1 } \right)$ on the remaining $n - 1$ variables whose solutions, $S _ { 0 } \ ( S _ { 1 } )$ , are precisely solutions of $\phi$ having $x _ { 1 } = 0 \ ( x _ { 1 } = 1 )$ .

Example A.2 Suppose $\phi = \left( x _ { 1 } \vee x _ { 2 } \vee x _ { 3 } \right) \wedge \left( \overline { { x } } _ { 1 } \vee x _ { 2 } \vee x _ { 4 } \right)$ . Then $\phi _ { 0 } = ( x _ { 2 } \lor x _ { 3 } )$ and $\phi _ { 1 } = \left( x _ { 2 } \lor x _ { 4 } \right)$ ✷

Using this property, an oracle for the decision version of SAT can be used to find a solution to $\phi ,$ assuming it is satisfiable, as follows. First check whether $\phi _ { 0 }$ is satisfiable. If ${ \mathrm { s o } } ,$ set $x _ { 0 } ~ = ~ 0$ , and find any solution to $\phi _ { 0 }$ Otherwise, set $x _ { 1 } = 1$ (in this case $\phi _ { 1 }$ must be satisfiable), and find a solution to $\phi _ { 1 }$ . In each case the problem has been reduced to a smaller one, and we will be done in n iterations.

The following representation will be particularly useful. Let $T$ be a binary tree of depth n whose leaves are all n-bit $0 / 1$ strings, representing truth assignments to the $n$ variables. Leaves that are solutions to $\phi$ are marked special. The root of $T$ is labeled with $\phi$ and its internal nodes are labeled with formulae whose solutions are in one-to-one correspondence with the marked leaves in the subtree rooted at this node. Thus, the 0th child of the root is labeled with $\phi _ { 0 }$ and the 1st child is labeled with $\phi _ { 1 }$ . Tree $T$ is called the self-reducibility tree for instance $\phi .$

![](images/b6f38de1bb0f276e549ae56a8a838328906a5b12ebc79e96d0714bf6918a9bf1.jpg)

We will formalize the notion of self-reducibility for NP-optimization problems. Formalizing this notion for NP problems is an easier task and is left as Exercise 1.15.

First, let us illustrate self-reducibility for cardinality vertex cover. Observe that an oracle for the decision version enables us to compute the size of the optimal cover, $\mathrm { O P T } ( G )$ , by binary search on $k .$ To actually find an optimal cover, remove a vertex v together with its incident edges to obtain graph $G ^ { \prime } { } .$ , and compute $\mathrm { O P T } ( G ^ { \prime } )$ . Clearly, v is in an optimal cover if $\mathrm { O P T } ( G ^ { \prime } ) =$ OPT(G) − 1. Furthermore, if v is in an optimal cover, then any optimal cover in $G ^ { \prime }$ , together with $v ,$ is an optimal cover in G. Otherwise, any optimal cover for G must contain all neighbors, say $N ( v )$ , of v (in order to cover all edges incident at v). Let $G ^ { \prime \prime }$ be the graph obtained by removing v and $N ( v )$ from G. Any optimal cover in $G ^ { \prime \prime }$ , together with $N ( v )$ , is an optimal cover in G. Thus, in both cases, we are left with the problem of finding an optimal cover in a smaller graph, $G ^ { \prime }$ or $G ^ { \prime \prime }$ . Continuing this way, an optimal cover in G can be found in polynomial time.

The above-stated reduction from the cardinality vertex cover problem to its decision version works because we could demonstrate polynomial time algorithms for

• obtaining the smaller graphs, $G ^ { \prime }$ and $G ^ { \prime \prime }$ ,

• computing the size of the best cover in G, consistent with the atomic decision, and

• constructing an optimal cover in $G ,$ , given an optimal cover in the smaller instance.

The exact manner in which self-reducibility manifests itself is quite different for diferent problems. Below we state a fairly general definition that covers a large number of problems. In the interest of conveying the main idea behind this important concept, we will provide an intuitive, though easily formalizable, definition.

We will assume that solutions to an instance I of NP-optimization problem Π have granularity, i.e., consist of smaller pieces called atoms that are meaningful in the context of the problem. For instance, for cardinality vertex cover, the atoms consist of specifying whether or not a certain vertex is in the cover. Clearly, for vertex cover this can be done using O(log n) bits. Indeed, all problems considered in this book have granularity $O ( \log n )$ . Let us assume this for problem Π.

![](images/143cb89fbe9ca89b2da2d59129099785faa36947b4d5667d7b305cc8c1f48849.jpg)

We will say that problem Π is self-reducible if there is a polynomial time algorithm, A, and polynomial time computable functions, $f ( \cdot , \cdot , \cdot )$ and $g ( \cdot , \cdot , \cdot )$ , satisfying the following conditions.

• Given instance I and an atom α of a solution to $I , A$ outputs an instance $I _ { \alpha }$ . We require that $| I _ { \alpha } | < | I |$ . Let $S ( I \mid \alpha )$ represent the set of feasible solutions to I that are consistent with atom α. We require that the feasible solutions of $I _ { \alpha } , \ S ( I _ { \alpha } )$ , are in one-to-one correspondence with $S ( I \mid \alpha )$ This correspondence is given by the polynomial time computable function $f ( \cdot , \cdot , \cdot )$ as follows.

$$
f (I, \alpha , \cdot): S (I _ {\alpha}) \to S (I \mid \alpha).
$$

• The correspondence $f ( I , \alpha , \cdot )$ preserves order in the objective function values of solutions. Thus, if $s _ { 1 } ^ { \prime }$ and $s _ { 2 } ^ { \prime }$ are two feasible solutions of $I _ { \alpha }$ with $\mathrm { o b j } _ { \pi } ( I _ { \alpha } , s _ { 1 } ^ { \prime } ) \leq \mathrm { o b j } _ { \pi } ( I _ { \alpha } , s _ { 2 } ^ { \prime } )$ , and $f ( I , \alpha , s _ { 1 } ^ { \prime } ) = s _ { 1 }$ and $f ( I , \alpha , s _ { 2 } ^ { \prime } ) = s _ { 2 }$ , then ob $\mathfrak { j } _ { \pi } ( I , s _ { 1 } ) \leq \mathrm { o b j } _ { \pi } ( I , s _ { 2 } )$

• Given the cost of an optimal solution to $I _ { \alpha } .$ , the cost of the best solution in $S ( I \mid \alpha )$ can be computed eficiently, and is given by $g ( I , \alpha , \mathrm { O P T } ( I _ { \alpha } ) )$ .

Theorem A.3 Let Π be an NP-optimization problem that is self-reducible. There is a polynomial time (exact) algorithm for Π, given an oracle, O, for the decision version of Π.

Proof: As remarked earlier, via a suitable binary search we can use $\mathcal { O }$ to compute the cost of the optimal solution to an instance in polynomial time.

We will derive polynomial time algorithm R for solving Π exactly. Assume that ${ \mathcal { A } } , f ,$ and g are defined as above for the self-reducibility of Π. Let I be an instance of Π. R first finds one atom of an optimal solution to I. An atom, say $\beta ,$ satisfies this condition if $g ( I , \beta , \mathrm { O P T } ( I _ { \beta } ) ) = \mathrm { O P T } ( I )$ , where $I _ { \beta } = \mathcal { A } ( I , \beta )$ . Since atoms are only $O ( \log n )$ bits long, finding such an atom involves simply searching the polynomially many possibilities. Let α be the atom found, and let $I _ { \alpha } = \mathcal { A } ( I , \alpha )$ . R then recursively computes an optimal solution, say $s ^ { \prime } ,$ to $I _ { \alpha } .$ . Finally, it outputs $f ( I , \alpha , s ^ { \prime } )$ , which is guaranteed to be an optimal solution to I. Since $| I _ { \alpha } | < | I |$ , the recursion also takes only polynomial time. ✷

Remark A.4 The number of strings of length O(log n) that algorithm R needs to examine for finding a good atom depends on the specific problem. For instance, in the case of cardinality vertex cover we picked an arbitrary vertex, say $v ,$ and considered only two atoms, that v is or isn’t in the cover.

## A.6 Notes

The definition of an NP-optimization problem is due to Krentel [178]. Approximation factor preserving reductions are a stringent version of $L _ { - }$ reducibility from Papadimitriou and Yannakakis [218]. Self-reducibility was first defined by Schnorr [234]. See Khuller and Vazirani [171] for a problem that is not self-reducible, assuming $\mathbf { P } \neq \mathbf { N P }$ . For further information on NP-completeness and complexity theory see Garey and Johnson [93] and Papadimitriou [216].

## B Basic Facts from Probability Theory

Let us recall some useful facts from probability theory. We assume that the reader has already had a detailed exposure to this material (see Section B.4 for references).

## B.1 Expectation and moments

Two quantities provide much information about a random variable: the mean, also called expectation, and variance. A key property of the expectation, which often simplifies its evaluation, is called linearity of expectation. It states that if $X , X _ { 1 } , \ldots , X _ { n }$ are random variables such that $X = c _ { 1 } X _ { 1 } + . . . + c _ { n } X _ { n }$ 2 where $c _ { 1 } , \ldots , c _ { n }$ are constants, then

$$
\mathbf {E} [ X ] = c _ {1} \mathbf {E} [ X _ {1} ] + \ldots + c _ {n} \mathbf {E} [ X _ {n} ].
$$

(In particular, the expectation of a sum of random variables is the sum of their expectations.) The usefulness of this property arises from the fact that no assumption is made about independence between the random variables $X _ { 1 } , \ldots , X _ { n }$ . Often a complex random variable can be written as the sum of indicator random variables $( \mathrm { i . e . } ,$ random variables taking on $0 / 1$ values only), thereby simplifying the evaluation of its expectation.

The variance of random variable X measures the spread of X from its mean, and is defined as

$$
\pmb {V} [ X ] = \mathbf {E} [ (X - \mathbf {E} [ X ]) ^ {2} ] = \mathbf {E} [ X ^ {2} ] - \mathbf {E} [ X ] ^ {2}.
$$

Its positive square root is called the standard deviation. The mean and standard deviation of X are denoted by $\mu ( X )$ and $\sigma ( X )$ , respectively.

For $k \in \mathbf N$ , the kth moment and kth central moment of X are defined to be $\mathbf { E } [ X ^ { k } ]$ and $\mathbf { E } [ ( X - \mathbf { E } [ X ] ) ^ { k } ]$ , respectively. Thus the variance is the second central moment.

In general, the expectation of the product of random variables is not the product of expectations. An important exception is when the random variables are independent. Thus, if X and Y are independent random variables, then $\mathbf { E } [ X Y ] = \mathbf { E } [ X ] \mathbf { E } [ Y ]$ . This immediately implies that the variance of the sum of independent random variables is the sum of their variances, $\mathrm { i . e . }$ , for independent random variables X and Y, $\pmb { V } [ X + Y ] = \pmb { V } [ X ] + \pmb { V } [ Y ]$

## B.2 Deviations from the mean

If X is a nonnegative random variable with a known expectation, then Markov’s Inequality helps bound the probability of deviations from the mean as follows. For $t \in \mathbf { R } ^ { + }$

$$
\mathbf {P r} [ X \geq t ] \leq \frac {\mathbf {E} [ X ]}{t}.
$$

This obvious inequality finds surprisingly many uses. For instance, it helps in obtaining a high probability statement from a bound on the expectation $( \mathrm { e . g . } ,$ see Section 14.2).

If the variance of a random variable is small, then large deviations from the mean are improbable. This intuitive statement is formalized by Chebyshev’s inequality which states that for any random variable X and $a \in \mathbf { R } +$ ,

$$
\mathbf {P r} [ | X - \mathbf {E} [ X ] | \geq a ] \leq \left(\frac {\sigma (X)}{a}\right) ^ {2}.
$$

See Lemma 28.5 for an application.

Poisson trials are repeated independent trials, each of which has two possible outcomes, called success and failure. In general, the success probability is allowed to change with the trials. They are called Bernoulli trials if the success probability is the same for each trial.

The Chernof bounds, which provide bounds on the tail probabilities of Poisson trials, are very useful in analyzing algorithms. Let us represent n Poisson trials by indicator random variables $X _ { 1 } , \ldots , X _ { n }$ , with 1 and 0 representing success and failure, respectively. Let $\mathbf { P r } [ X _ { i } = 1 ] = p _ { i }$ , where $0 < p _ { i } < 1$ for $1 \leq i \leq n$ . Let random variable $X = X _ { 1 } + \ldots + X _ { n } $ and $\textstyle \mu = \mathbf { E } [ X ] = \sum _ { i = 1 } ^ { n } p _ { i }$ For the bound on the lower tail assume $0 < \delta \leq 1$ . Then,

$$
\mathbf {P r} [ X <   (1 - \delta) \mu ] <   e ^ {(- \mu \delta^ {2} / 2)}.
$$

The expression for the upper tail is more involved: for any $\delta > 0$ 2

$$
\operatorname * {P r} [ X > (1 + \delta) \mu ] <   \left(\frac {e ^ {\delta}}{(1 + \delta) ^ {(1 + \delta)}}\right) ^ {\mu}.
$$

It can be simplified by considering two ranges for δ. For $\delta > 2 e - 1$ 2

$$
\operatorname * {P r} [ X > (1 + \delta) \mu ] <   2 ^ {- (1 + \delta) \mu},
$$

and for $\delta \leq 2 e - 1$

$$
\mathbf {P r} [ X > (1 + \delta) \mu ] <   e ^ {- \mu \delta^ {2} / 4}.
$$

## B.3 Basic distributions

Three distributions, of great universality, are defined below. The probability distribution of the number of successes in Bernoulli trials is called the binomial distribution. Consider n Bernoulli trials with probability of success $p .$ The probability of k successes, for $0 \leq k \leq n$ , is given by

$$
B (k; n, p) = \binom {n} {k} p ^ {k} (1 - p) ^ {n - k}.
$$

The Poisson distribution with parameter $\lambda > 0$ is as follows. For each nonnegative integer $k ,$ the probability of exactly k successes is defined to be

$$
p (k; \lambda) = e ^ {- \lambda} \frac {\lambda^ {k}}{k !}.
$$

The limit of the binomial distribution $B ( k ; n , p )$ as $n  \infty$ and $n p  \lambda ,$ a constant, is the Poisson distribution $p ( k ; \lambda )$ . Indeed, in many applications one comes across Bernoulli trials in which n is large, $p$ is small, and the product $\lambda = n p$ is moderate. In these situations, $p ( k ; n p )$ is a good approximation to $B ( k ; n , p )$

The normal density function with mean $\mu$ and standard deviation $\sigma$ is

$$
n (x) = \frac {1}{\sigma \sqrt {2 \pi}} e ^ {- \frac {(x - \mu) ^ {2}}{2 \sigma^ {2}}},
$$

and the normal distribution function is its integral,

$$
N (x) = \frac {1}{\sigma \sqrt {2 \pi}} \int_ {- \infty} ^ {x} e ^ {- \frac {(y - \mu) ^ {2}}{2 \sigma^ {2}}} \mathrm{d} y.
$$

The normal distribution also approximates the binomial distribution. Let us state this for the case $p = 1 / 2$ . Let n be even, $n = 2 \nu$ , say. For $- \nu \leq k \leq \nu .$ define

$$
a _ {k} = a _ {- k} = B (\nu + k; 2 \nu , 1 / 2).
$$

In the limit as $\nu \to \infty$ and $k$ varies in the range $0 < k < \sqrt { \nu } , a _ { k }$ can be approximated by $h n ( k h )$ , where $\begin{array} { r } { h = \sqrt { \frac { 2 } { \nu } } = \frac { 2 } { \sqrt { n } } } \end{array}$

## B.4 Notes

For further information see the books by Feller [85], Motwani and Raghavan [209], Spencer [243], and Alon and Spencer [6].

1. A. Agrawal, P. Klein, and R. Ravi. When trees collide: an approximation algorithm for the generalized Steiner network problem on networks. SIAM Journal on Computing<sub>,</sub> 24<sub>:</sub>440<sub>–</sub>456<sub>,</sub> 1995<sub>.</sub> (Cited on pp<sub>.</sub> 130<sub>,</sub> 212)

2. M. Ajtai. The shortest vector problem in L2 is NP-hard for randomized reductions. In Proc. 30th ACM Symposium on the Theory of Computing, pages 10–19, 1998. (Cited on p. 293)

3. D. Aldous. The random walk construction for spanning trees and uniform labeled trees. SIAM Journal on Discrete Mathematics, 3:450–465, 1990. (Cited on p. 339)

4. F. Alizadeh. Interior point methods in semidefinite programming with applications to combinatorial optimization. SIAM Journal on Optimization, 5:13–51, 1995. (Cited on p. 268)

5. N. Alon, A. Frieze, and D. Welsh. Polynomial time randomised approximation schemes for Tutte-Grothendieck invariants: the dense case. Random Structures and Algorithms, 6:459–478, 1995. (Cited on p. 342)

6. N. Alon and J. Spencer. The Probabilistic Method. John Wiley & Sons, New York<sub>,</sub> NY<sub>,</sub> 2000<sub>.</sub> (Cited on pp<sub>.</sub> 139<sub>,</sub> 354)

7. J. D. Annan. The complexities of the coeficients of the Tutte polynomial. Discrete Applied Mathematics, 57:93–103, 1995. (Cited on p. 342)

8. J.D. Annan. A randomized approximation algorithm for counting the number of forests in dense graphs. Combinatorics, Probability and Computing, 3:273– 283, 1994. (Cited on p. 339)

9. S. Arora. Polynomial time approximation scheme for Euclidean TSP and other geometric problems. In Proc. 37th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 2<sub>–</sub>11<sub>,</sub> 1996<sub>.</sub> (Cited on p<sub>.</sub> 89)

10. S. Arora. Nearly linear time approximation scheme for Euclidean TSP and other geometric problems. In Proc. 38th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 554<sub>–</sub>563<sub>,</sub> 1997<sub>.</sub> (Cited on p<sub>.</sub> 89)

11. S. Arora and C. Lund. Hardness of approximations. In D.S. Hochbaum, editor, Approximation Algorithms for NP-Hard Problems, pages 46–93. PWS Publishing, Boston, MA, 1997. (Cited on p. 332)

12. S. Arora, C. Lund, R. Motwani, M. Sudan, and M. Szegedy. Proof verification and intractability of approximation problems. In Proc. 33rd IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 13<sub>–</sub>22<sub>,</sub> 1992<sub>.</sub> (Cited on p. 332)

13. S. Arora, P. Raghavan, and S. Rao. Approximation schemes for Euclidean kmedians and related problems. In Proc. 30th ACM Symposium on the Theory of Computing<sub>,</sub> pages 106<sub>–</sub>113<sub>,</sub> 1998<sub>.</sub> (Cited on p<sub>.</sub> 89)

14. S. Arora and S. Safra. Probabilistic checking of proofs: a new characterization of NP. In Proc. 33rd IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 2<sub>–</sub>13<sub>,</sub> 1992<sub>.</sub> (Cited on p<sub>.</sub> 332)

15. V. Arya, N. Garg, R. Khandekar, A. Meyerson, K. Munagala, and V. Pandit<sub>.</sub> Local search heuristics for k<sub>-</sub>median and facilit<sub>y</sub> location problems<sub>.</sub> In Proc<sub>.</sub> 33rd ACM Symposium on the Theory of Computing<sub>,</sub> 2001<sub>.</sub> (Cited on pp. 253, 254)

16. Y. Aumann and Y. Rabani. An O(log k) approximate min-cut max-flow theorem and approximation algorithms. SIAM Journal on Computing, 27:291–301, 1998. (Cited on p. 197)

17. G. Ausiello, P. Crescenzi, G. Gambosi, V. Kann, A. Marchetti-Spaccamela, and M. Protasi. Complexity and Approximation. Combinatorial Optimization Problems and their Approximability Properties. Springer-Verlag, Berlin, 1999. (Cited on pp. 11, 333)

18. L. Babai. Trading group theory for randomness. In Proc. 17th ACM Symposium on the Theory of Computing<sub>,</sub> pages 421<sub>–</sub>429<sub>,</sub> 1985<sub>.</sub> (Cited on p<sub>.</sub> 332)

19. V. Bafna, P. Berman, and T. Fujito. Constant ratio approximations of the weighted feedback vertex set problem for undirected graphs. In Algorithms and Computation, 6th International Symposium, ISAAC, volume 1004 of Lecture Notes in Computer Science, pages 142–151. Springer-Verlag, Berlin, 1995. (Cited on p. 60)

20. R. Bar-Yehuda and S. Even. A linear-time approximation algorithm for the weighted vertex cover problem. Journal of Algorithms, 2:198–203, 1981. (Cited on p. 130)

21. Y. Bartal. Probabilistic approximation of metric spaces and its algorithmic applications. In Proc. 37th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 184<sub>–</sub>193<sub>,</sub> 1996<sub>.</sub> (Cited on p<sub>.</sub> 254)

22. C. Bazgan, M. Santha, and Z. Tuza. Eficient approximation algorithms for the subset-sum problem. In Proc. 25th International Colloquium on Automata, Languages, and Programming, volume 1443 of Lecture Notes in Computer Science<sub>,</sub> pages 387<sub>–</sub>396<sub>.</sub> Springer<sub>-</sub>Verlag<sub>,</sub> Berlin<sub>,</sub> 1998<sub>.</sub> (Cited on p<sub>.</sub> 72)

23. A. Becker and D. Geiger. Approximation algorithms for the loop cutset problem. In Proc. 10th Conference on Uncertainty in Artificial Intelligence, pages 60–68, 1994. (Cited on p. 60)

24. M. Ben-or, S. Goldwasser, J. Kilian, and A. Wigderson. Multi-prover interactive proofs: How to remove intractability. In Proc. 20th ACM Symposium on the Theory of Computing<sub>,</sub> pages 113<sub>–</sub>131<sub>,</sub> 1988<sub>.</sub> (Cited on p<sub>.</sub> 332)

25. M. Bern and P. Plassmann. The Steiner problem with edge lengths 1 and 2. Information Processing Letters<sub>,</sub> 32<sub>:</sub>171<sub>–</sub>176<sub>,</sub> 1989<sub>.</sub> (Cited on p<sub>.</sub> 332)

26. S.N. Bhatt and F.T. Leighton. A framework for solving VLSI graph layout problems. Journal of Computer and System Sciences, 28:300–343, 1984. (Cited on p. 197)

27. A. Blum, T. Jiang, M. Li, J. Tromp, and M. Yannakakis. Linear approximation of shortest superstring<sub>.</sub> Journal of the ACM<sub>,</sub> 41<sub>:</sub>630<sub>–</sub>647<sub>,</sub> 1994<sub>.</sub> (Cited on p. 67)

28. M. Blum and S. Kannan. Designing programs that check their work. In Proc. 21st ACM Symposium on the Theory of Computing<sub>,</sub> pages 86<sub>–</sub>97<sub>,</sub> 1989<sub>.</sub> (Cited on p. 332)

29. M. Blum, M. Luby, and R. Rubinfeld. Testing/correcting with applications to numerical problems. Journal of Computer and System Sciences, 47:549–595, 1993. (Cited on p. 332)

30. R. Boppana and M.M. Halld´orsson. Approximating maximum independent sets by excluding subgraphs. BIT, 32:180–196, 1992. (Cited on p. 332)

31. A. Borodin and R. El-Yaniv. Online Computation and Competitive Analysis. Cambridge University Press, Cambridge, UK, 1998. (Cited on p. 78)

32. J. Bourgain. On Lipschitz embedding of finite metric spaces in Hilbert spaces. Israeli J. Math., 52:46–52, 1985. (Cited on p. 197)

33. A.Z. Broder. How hard is it to marry at random? In Proc. 18th ACM Symposium on the Theory of Computing<sub>,</sub> pages 50<sub>–</sub>58<sub>,</sub> 1986<sub>.</sub> (Cited on p<sub>.</sub> 305)

34. A.Z. Broder. Generating random spanning trees. In Proc. 30th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 442<sub>–</sub>447<sub>,</sub> 1989<sub>.</sub> (Cited on p. 339)

35. R. Bubley and M. Dyer. Faster random generation of linear extensions. Discrete Mathematics<sub>,</sub> 201<sub>:</sub>81<sub>–</sub>88<sub>,</sub> 1999<sub>.</sub> (Cited on p<sub>.</sub> 340)

36. G. Calinescu, H. Karlof, and Y. Rabani. An improved approximation algorithm for multiway cut. In Proc. 30th ACM Symposium on the Theory of Computing<sub>,</sub> pages 48<sub>–</sub>52<sub>,</sub> 1998<sub>.</sub> (Cited on p<sub>.</sub> 167)

37. M. Charikar, C. Chekuri, T. Cheung, Z. Dai, A. Goel, S. Guha, and M. Li. Approximation algorithms for directed Steiner tree problems. In Proc. 9th ACM-SIAM Annual Symposium on Discrete Algorithms, pages 192–200, 1998. (Cited on p. 337)

38. M. Charikar and S. Guha. Improved combinatorial algorithms for the facility location and k-median problems. In Proc. 40th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 378<sub>–</sub>388<sub>,</sub> 1999<sub>.</sub> (Cited on p<sub>.</sub> 254)

39. M. Charikar, S. Guha, E. Tardos, and D.B. Shmoys. A constant-factor approx- <sup>´</sup> imation algorithm for the k<sub>-</sub>median problem<sub>.</sub> In Proc<sub>.</sub> 31st ACM Symposium on the Theory of Computing<sub>,</sub> pages 1<sub>–</sub>10<sub>,</sub> 1999<sub>.</sub> (Cited on p<sub>.</sub> 254)

40. M. Charikar, S. Khuller, D.M. Mount, and G. Narshimhan. Algorithms for facility location problems with outliers. In Proc. 12th ACM-SIAM Annual Symposium on Discrete Algorithms<sub>,</sub> pages 642<sub>–</sub>651<sub>,</sub> 2001<sub>.</sub> (Cited on p<sub>.</sub> 240)

41. M. Charikar, J. Kleinberg, R. Kumar, S. Rajagopalan, A. Sahai, and A. Tomkins. Minimizing wirelength in zero and bounded skew clock trees. In Proc. 10th ACM-SIAM Annual Symposium on Discrete Algorithms, pages 177–184, 1999. (Cited on p. 37)

42. J. Cheriyan and R. Thurimella. Approximating minimum-size k-connected spanning subgraphs via matching. In Proc. 37th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 292<sub>–</sub>301<sub>,</sub> 1996<sub>.</sub> (Cited on pp. 226, 227, 231)

43. B. Chor and M. Sudan. A geometric approach to betweenness. SIAM Journal on Discrete Mathematics<sub>,</sub> 11<sub>:</sub>511<sub>–</sub>523<sub>,</sub> 1998<sub>.</sub> (Cited on p<sub>.</sub> 267)

44. E.-A. Choukhmane. Une heuristique pour le probl\`eme de l’arbre de Steiner. RAIRO Rech<sub>.</sub> Op´er<sub>.,</sub> 12<sub>:</sub>207<sub>–</sub>212<sub>,</sub> 1978<sub>.</sub> (Cited on p<sub>.</sub> 37)

45. N. Christofides. Worst-case analysis of a new heuristic for the traveling salesman problem. Technical report, Graduate School of Industrial Administration, Carnegie-Mellon University, Pittsburgh, PA, 1976. (Cited on p. 37)

46. F. Chudak, M.X. Goemans, D. Hochbaum, and D.P. Williamson. A primal– dual interpretation of two 2-approximation algorithms for the feedback vertex

set problem in undirected graphs. Operations Research Letters, 22:111–118, 1998. (Cited on pp. 60, 129)

47. F. Chudak, T. Roughgarden, and D.P. Williamson. Approximate k-MSTs and k-Steiner trees via the primal–dual method and Lagrangian relaxation. Manuscript, 2000. (Cited on p. 252)

48. V. Chv´atal. A greedy heuristic for the set covering problem. Mathematics of Operations Research<sub>,</sub> 4<sub>:</sub>233<sub>–</sub>235<sub>,</sub> 1979<sub>.</sub> (Cited on pp<sub>.</sub> 26<sub>,</sub> 118)

49. V. Chv´atal. Linear Programming. W.H. Freeman and Co., New York, NY, 1983. (Cited on p. 107)

50. E.G. Cofman Jr., M.R. Garey, and D.S. Johnson. Approximation algorithms for bin backing: a survey. In D.S. Hochbaum, editor, Approximation Algorithms for NP-Hard Problems, pages 46–93. PWS Publishing, Boston, MA, 1997. (Cited on p. 78)

51. S.A. Cook. The complexity of theorem-proving procedures. In Proc. 3rd ACM Symposium on the Theory of Computing<sub>,</sub> pages 151<sub>–</sub>158<sub>,</sub> 1971<sub>.</sub> (Cited on p. 10)

52. W.J. Cook, W.H. Cunningham, W.R. Pulleyblank, and A. Schrijver. Combinatorial Optimization. John Wiley & Sons, New York, NY, 1998. (Cited on p. 107)

53. C. Cooper and A. Frieze. Mixing properties of the Swendsen-Wang process on classes of graphs. Random Structures Algorithms, 15:242–261, 1999. (Cited on p. 342)

54. T. H. Cormen, C. E. Leiserson, R. L. Rivest, and C. Stein. Introduction to Algorithms. Second edition. MIT Press and McGraw-Hill, 2001. (Cited on p. 11)

55. R. Courant and H. Robbins. What Is Mathematics? Oxford University Press, Ne<sub>w</sub> York<sub>,</sub> NY<sub>,</sub> 1941<sub>.</sub> (Cited on p<sub>.</sub> 37)

56. P. Dagum, M. Luby, M. Mihail, and U.V. Vazirani. Polytopes, permanents and graphs with large factors. In Proc. 29th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 412<sub>–</sub>421<sub>,</sub> 1988<sub>.</sub> (Cited on p<sub>.</sub> 339)

57. E. Dahlhaus, D.S. Johnson, C.H. Papadimitriou, P.D. Seymour, and M. Yannakakis. The complexity of multiterminal cuts. SIAM Journal on Computing, 23:864–894, 1994. (Cited on p. 46)

58. G.B. Dantzig. Linear Programming and Extensions. Reprint of the 1968 corrected edition. Princeton University Press, Princeton, NJ, 1998. (Cited on p. 107)

59. G.B. Dantzig, L.R. Ford, and D.R. Fulkerson. Solution of a large-scale traveling-salesman problem. Operations Research, 2:393–410, 1954. (Cited on p. 231)

60. G.B. Dantzig, L.R. Ford, and D.R. Fulkerson. A primal–dual algorithm for linear programs. In H.W. Kuhn and A.W. Tucker, editors, Linear Inequalities and Related Systems, pages 171–181. Princeton University Press, Princeton, NJ, 1956. (Cited on p. 129)

61. G. Dobson. Worst-case analysis of greedy heuristics for integer programming with non-negative data. Mathematics ofOperations Research, 7:515–531, 1982. (Cited on p. 118)

62. P. Drineas, R. Kannan, A. Frieze, S. Vempala, and V. Vinay. Clustering in large graphs and matrices. In Proc. 10th ACM-SIAM Annual Symposium on Discrete Algorithms<sub>,</sub> pages 291<sub>–</sub>299<sub>,</sub> 1999<sub>.</sub> (Cited on p<sub>.</sub> 254)

63. D.Z. Du and F.K. Hwang. Gilbert-Pollack conjecture on Steiner ratio is true. Algorithmica<sub>,</sub> 7<sub>:</sub>121<sub>–</sub>135<sub>,</sub> 1992<sub>.</sub> (Cited on p<sub>.</sub> 37)

64. M. Dyer, R. Kannan, and J. Mount. Sampling contingency tables. Random Structures and Algorithms<sub>,</sub> 10<sub>:</sub>487<sub>–</sub>506<sub>,</sub> 1997<sub>.</sub> (Cited on p<sub>.</sub> 340)

65. M.E. Dyer, A. Frieze, and M.R. Jerrum. Approximately counting hamilton cycles in dense graphs. SIAM Journal on Computing, 27:1262–1272, 1998. (Cited on p. 341)

66. M.E. Dyer, A. Frieze, and M.R. Jerrum. On counting independent sets in sparse graphs. In Proc. 40th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 210<sub>–</sub>217<sub>,</sub> 1999<sub>.</sub> (Cited on p<sub>.</sub> 341)

67. M.E. Dyer, A. Frieze, and R. Kannan. A random polynomial time algorithm for approximating the volume of convex bodies. Journal of the ACM, 38:1–17, 1991. (Cited on p. 338)

68. J. Edmonds. Maximum matching and a polyhedron with 0,1-vertices. Journal of Research of the National Bureau of Standards. Section B, 69:125–130, 1965. (Cited on p. 104)

69. J. Edmonds. Paths, trees, and flowers. Canadian Journal of Mathematics, 17:449–467, 1965. (Cited on pp. 10, 11)

70. J. Edmonds. Optimum branchings. Journal of Research of the National Bureau of Standards<sub>.</sub> Section B<sub>,</sub> 71<sub>:</sub>233<sub>–</sub>240<sub>,</sub> 1967<sub>.</sub> (Cited on p<sub>.</sub> 212)

71. J. Edmonds. Matroids and the greedy algorithm. Mathematical Programming, 1:127–136, 1971. (Cited on p. 105)

72. J. Edmonds. Matroid intersection. Annals of Discrete Mathematics, 4:185– 204, 1979. (Cited on p. 228)

73. P. Erd˝os. Gr´afok p´aros k¨or¨ulj´ar´as´u r´eszgr´afjair´ol (On bipartite subgraphs of graphs<sub>,</sub> in Hungarian)<sub>.</sub> Mat. Lapok<sub>,</sub> 18<sub>:</sub>283<sub>–</sub>288<sub>,</sub> 1967<sub>.</sub> (Cited on p<sub>.</sub> 10)

74. P. Erd˝os and J.L. Selfridge. On a combinatorial game. Journal of Combinatorial Theory<sub>,</sub> Series A<sub>,</sub> 14<sub>:</sub>298<sub>–</sub>301<sub>,</sub> 1973<sub>.</sub> (Cited on p<sub>.</sub> 139)

75. G. Even, J. Naor, B. Schieber, and S. Rao. Divide-and-conquer approximation algorithms via spreading metrics. Journal of the ACM, 47:585–616, 2000. (Cited on p. 178)

76. G. Even, J. Naor, B. Schieber, and L. Zosin. Approximating minimum subset feedback sets in undirected graphs with applications. In Proc. 4th Israel Symposium on Theory of Computing and Systems<sub>,</sub> pages 78<sub>–</sub>88<sub>,</sub> 1996<sub>.</sub> (Cited on p. 167)

77. G. Even, J. Naor, and L. Zosin. An 8-approximation algorithm for the subset feedback vertex set problem. In Proc. 37th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 310<sub>–</sub>319<sub>,</sub> 1996<sub>.</sub> (Cited on p<sub>.</sub> 167)

78. T. Feder and M. Mihail. Balanced matroids. In Proc. 24th ACM Symposium on the Theory of Computing<sub>,</sub> pages 26<sub>–</sub>38<sub>,</sub> 1992<sub>.</sub> (Cited on p<sub>.</sub> 339)

79. U. Feige. Approximating the bandwidth via volume respecting embeddings. In Proc. 30th ACM Symposium on the Theory of Computing, pages 90–99, 1998. (Cited on p. 196)

80<sub>.</sub> U<sub>.</sub> Feige<sub>.</sub> A treshold of ln n for appro<sub>x</sub>imating set co<sub>v</sub>er<sub>.</sub> Journal of the ACM<sub>,</sub> 45:634–652, 1998. (Cited on pp. 26, 331, 332)

81. U. Feige and M.X. Goemans. Approximating the value of two prover proof systems, with applications to MAX-CUT and MAX DICUT. In Proc. 3rd Israel Symposium on Theory of Computing and Systems, pages 182–189, 1995. (Cited on p. 269)

82. U. Feige, S. Goldwasser, L. Lov´asz, S. Safra, and M. Szegedy. Approximating clique is almost NP-complete. In Proc. 32nd IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 2<sub>–</sub>12<sub>,</sub> 1991<sub>.</sub> (Cited on p<sub>.</sub> 332)

83. U. Feige and R. Krauthgamer. A polylogarithmic approximation of the minimum bisection. In Proc. 41st IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 105<sub>–</sub>115<sub>,</sub> 2000<sub>.</sub> (Cited on pp<sub>.</sub> 197<sub>,</sub> 336)

84. U. Feige and G. Schechtman. On the optimality of the random hyperplane rounding technique for MAX-CUT. In Proc. 33rd ACM Symposium on the Theory of Computing<sub>,</sub> 2001<sub>.</sub> (Cited on p<sub>.</sub> 268)

85. W. Feller. An Introduction to Probability Theory and its Applications. John Wile<sub>y</sub> & Sons<sub>,</sub> Ne<sub>w</sub> York<sub>,</sub> NY<sub>,</sub> 1950<sub>.</sub> (Cited on p<sub>.</sub> 354)

86. W. Fernandez de la Vega and G.S. Lueker. Bin packing can be solved within 1 + ε in linear time. Combinatorica, 1:349–355, 1981. (Cited on p. 78)

87. A. Freund and H. Karlof. A lower bound of $8 / ( 7 + { \frac { 1 } { k - 1 } } )$ on the integrality ratio of the Calinescu–Karlof–Rabani relaxation for multiway cut. Information Processing Letters, 75:43–50, 2000. (Cited on p. 167)

88. A. Frieze. On the Lagarias–Odlyzko algorithm for the subset sum problem. SIAM Journal on Computing<sub>,</sub> 15<sub>:</sub>536<sub>–</sub>539<sub>,</sub> 1986<sub>.</sub> (Cited on p<sub>.</sub> 291)

89. A. Frieze, G. Galbiati, and F. Mafioli. On the worst-case performance of some algorithms for the asymmetric traveling salesman problem. Networks, 12:23–39, 1982. (Cited on p. 34)

90<sub>.</sub> A<sub>.</sub> Frie<sub>z</sub>e and M<sub>.</sub> Jerrum<sub>.</sub> Impro<sub>v</sub>ed appro<sub>x</sub>imation algorithms for MAX k<sub>-</sub> CUT and MAX BISECTION<sub>.</sub> Algorithmica<sub>,</sub> 18<sub>:</sub>67<sub>–</sub>81<sub>,</sub> 1997<sub>.</sub> (Cited on p<sub>.</sub> 269)

91. M.R. Garey, R.L. Graham, and J.D. Ullman. An analysis of some packing algorithms<sub>.</sub> In Combinatorial Algorithms (Courant Computer Science Sympo<sub>-</sub> sium, No. 9), pages 39–47, 1972. (Cited on p. 10)

92. M.R. Garey and D.S. Johnson. Strong NP-completeness results: motivation, examples, and implications. Journal of the ACM, 25:499–508, 1978. (Cited on p. 73)

93. M.R. Garey and D.S. Johnson. Computers and Intractability: A Guide to the Theory of NP-Completeness. W.H. Freeman and Co., New York, NY, 1979. (Cited on pp. 11, 351)

94. N. Garg. A 3-approximation for the minimum tree spanning k vertices. In Proc. 37th IEEE Annual Symposium on Foundations of Computer Science, pages 302–309, 1996. (Cited on p. 252)

95. N. Garg, H. Saran, and V.V. Vazirani. Finding separator cuts in planar graphs ithi t i th ti l SIAM J l C ti 29 159 179 1999 (Cited on p. 336)

96. N. Garg, V.V. Vazirani, and M. Yannakakis. Multiway cuts in directed and node weighted graphs. In Proc. 21st International Colloquium on Automata, Languages, and Programming, volume 820 of Lecture Notes in Computer Science<sub>,</sub> pages 487<sub>–</sub>498<sub>.</sub> Springer<sub>-</sub>Verlag<sub>,</sub> Berlin<sub>,</sub> 1994<sub>.</sub> (Cited on p<sub>.</sub> 167)

97. N. Garg, V.V. Vazirani, and M. Yannakakis. Approximate max-flow min-(multi)cut theorems and their applications. SIAM Journal on Computing, 25:235–251, 1996. (Cited on p. 179)

98. N. Garg, V.V. Vazirani, and M. Yannakakis. Primal–dual approximation algorithms for integral flow and multicut in trees. Algorithmica, 18:3–20, 1997. (Cited on pp. 152, 153, 154)

99. C.F. Gauss. Disquisitiones Arithmeticae. English edition translated by A.A. Clarke. Springer-Verlag, New York, NY, 1986. (Cited on p. 292)

100. E.N. Gilbert and H.O. Pollak. Steiner minimal trees. SIAM Journal on Applied Mathematics<sub>,</sub> 16<sub>:</sub>1<sub>–</sub>29<sub>,</sub> 1968<sub>.</sub> (Cited on p<sub>.</sub> 37)

101. M.X. Goemans and D.J. Bertsimas. Survivable networks, linear programming relaxations and the parsimonious property. Mathematical Programming, 60:145–166, 1993. (Cited on p. 228)

102. M.X. Goemans, A.V. Goldberg, S. Plotkin, D.B. Shmoys, E. Tardos, and D.P. <sup>´</sup> Williamson. Improved approximation algorithms for network design problems. In Proc. 5th ACM-SIAM Annual Symposium on Discrete Algorithms, pages 223–232, 1994. (Cited on p. 225)

103. M.X. Goemans and J. Kleinberg. The Lov´asz theta function and a semidefinite programming relaxation of vertex cover. SIAM Journal on Discrete Mathematics, 11:196–204, 1998. (Cited on p. 334)

104. M.X. Goemans and D.P. Williamson. New 3 -approximation algorithms for the maximum satisfiability problem. SIAM Journal on Discrete Mathematics, 7:656–666, 1994. (Cited on pp. 138, 139)

105. M.X. Goemans and D.P. Williamson. A general approximation technique for constrained forest problems. SIAM Journal on Computing, 24:296–317, 1995. (Cited on pp. 130, 208, 212)

106. M.X. Goemans and D.P. Williamson. Improved approximation algorithms for maximum cut and satisfiability problems using semidefinite programming. Journal of the ACM<sub>,</sub> 42<sub>:</sub>1115<sub>–</sub>1145<sub>,</sub> 1995<sub>.</sub> (Cited on pp<sub>.</sub> 267<sub>,</sub> 268)

107. M.X. Goemans and D.P. Williamson. The primal–dual method for approximation algorithms and its applications to network design problems. In D.S. Hochbaum, editor, Approximation Algorithms for NP-Hard Problems, pages 144<sub>–</sub>191<sub>.</sub> PWS Publishing<sub>,</sub> Boston<sub>,</sub> MA<sub>,</sub> 1997<sub>.</sub> (Cited on pp<sub>.</sub> 130<sub>,</sub> 212)

108. O. Goldreich, D. Micciancio, S. Safra, and J.-P. Seifert. Approximating shortest lattice vectors is not harder than approximating closest lattice vectors. Information Processing Letters<sub>,</sub> 71<sub>,</sub> 1999<sub>.</sub> (Cited on p<sub>.</sub> 292)

109. S. Goldwasser, S. Micali, and C. Rackof. The knowledge complexity of interacti<sub>v</sub>e proofs<sub>.</sub> SIAM Journal on Computing<sub>,</sub> 18<sub>:</sub>186<sub>–</sub>208<sub>,</sub> 1989<sub>.</sub> (Cited on p. 332)

110. R.E. Gomory and T.C. Hu. Multi-terminal network flows. Journal of the SIAM<sub>,</sub> 9<sub>:</sub>551<sub>–</sub>570<sub>,</sub> 1961<sub>.</sub> (Cited on p<sub>.</sub> 46)

111. T.F. Gonzalez. Clustering to minimize the maximum inter-cluster distance. Theoretical Computer Science<sub>,</sub> 38<sub>:</sub>293<sub>–</sub>306<sub>,</sub> 1985<sub>.</sub> (Cited on p<sub>.</sub> 52)

112. V. Gore and M. Jerrum. The Swendsen-Wang process does not always mix rapidly. In Proc. 29th ACM Symposium on the Theory of Computing, pages 674–681, 1997. (Cited on p. 342)

113. R.L. Graham. Bounds for certain multiprocessing anomalies. Bell System Technical Journal, 45:1563–1581, 1966. (Cited on pp. 10, 83)

114. R.L. Graham. Bounds on multiprocessing timing anomalies. SIAM Journal on Applied Mathematics, 17:416–429, 1969. (Cited on p. 83)

115. M. Grigni, E. Koutsoupias, and C. Papadimitriou. An approximation scheme for planar graph TSP. In Proc. 36th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 640<sub>–</sub>646<sub>,</sub> 1995<sub>.</sub> (Cited on p<sub>.</sub> 89)

116. M. Gr¨otschel, L. Lov´asz, and A. Schrijver. The ellipsoid method and its consequences in combinatorial optimization. Combinatorica, 1:169–197, 1981. (Cited on p. 107)

117. M. Gr¨otschel, L. Lov´asz, and A. Schrijver. Geometric Algorithms and Combinatorial Optimization. Second edition. Springer-Verlag, Berlin, 1993. (Cited on p. 107)

118. V. Guruswami, S. Khanna, R. Rajaraman, B. Sheperd, and M. Yannakakis. Near-optimal hardness results and approximation algorithms for edge-disjoint and related problems. In Proc. 31st ACM Symposium on the Theory of Computing, pages 19–28, 1999. (Cited on p. 154)

119. D. Gusfield and R. W. Irving. The Stable Marriage Problem: Structure and Algorithms<sub>.</sub> MIT Press<sub>,</sub> Cambridge<sub>,</sub> MA<sub>,</sub> 1989<sub>.</sub> (Cited on p<sub>.</sub> 341)

120. L.A. Hall. Approximation algorithms for scheduling. In D.S. Hochbaum, editor, Approximation Algorithms for NP-Hard Problems, pages 1–45. PWS Publishing, Boston, MA, 1997. (Cited on p. 145)

121<sub>.</sub> J<sub>.</sub> H<sub>as</sub>t<sub>a</sub>d<sub>.</sub> Cli<sub>que</sub> i<sub>s</sub> h<sub>ar</sub>d t<sub>o approx</sub>i<sub>ma</sub>t<sub>e w</sub>ithi<sub>n</sub> n<sup>1</sup>−<sup>ε</sup><sub>.</sub> I<sub>n</sub> P<sub>roc.</sub> 37th IEEE A<sub>n-</sub> nual Symposium on Foundations of Computer Science, pages 627–636, 1996. (Cited on p. 332)

122. J. Hastad. Some optimal inapproximability results. In Proc. 29th ACM Symposium on the Theory of Computing<sub>,</sub> pages 1<sub>–</sub>10<sub>,</sub> 1997<sub>.</sub> (Cited on pp<sub>.</sub> 330<sub>,</sub> 332)

123. M. Held and R.M. Karp. The traveling-salesman and minimum cost spanning trees<sub>.</sub> Operations Research<sub>,</sub> 18<sub>:</sub>1138<sub>–</sub>1162<sub>,</sub> 1970<sub>.</sub> (Cited on p<sub>.</sub> 230)

124. D. S. Hochbaum. Heuristics for the fixed cost median problem. Mathematical Programming, 22:148–162, 1982. (Cited on p. 242)

125. D.S. Hochbaum. Approximation algorithms for the set covering and vertex co<sub>v</sub>er problems<sub>.</sub> SIAM Journal on Computing<sub>,</sub> 11<sub>:</sub>555<sub>–</sub>556<sub>,</sub> 1982<sub>.</sub> (Cited on pp. 25, 124)

126. D.S. Hochbaum, editor. Approximation Algorithms for NP-Hard Problems. PWS Publishing<sub>,</sub> Boston<sub>,</sub> MA<sub>,</sub> 1997<sub>.</sub> (Cited on p<sub>.</sub> 11)

127. D.S. Hochbaum and D.B. Shmoys. A unified approach to approximation algorithms for bottleneck problems. Journal of the ACM, 33:533–550, 1986. (Cited on p. 53)

128. D.S. Hochbaum and D.B. Shmoys. Using dual approximation algorithms for scheduling problems: theoretical and practical results. Journal of the ACM, 34:144–162, 1987. (Cited on p. 83)

129. D.S. Hochbaum and D.B. Shmoys. A polynomial approximation scheme for machine scheduling on uniform processors: using the dual approximation approach<sub>.</sub> SIAM Journal on Computing<sub>,</sub> 17<sub>:</sub>539<sub>–</sub>551<sub>,</sub> 1988<sub>.</sub> (Cited on p<sub>.</sub> 145)

130. J.A. Hoogeveen. Analysis of Christofides’ heuristic: some paths are more dificult than cycles. Operations Research Letters, 10:291–295, 1991. (Cited on p. 34)

131. E. Horowitz and S.K. Sahni. Exact and approximate algorithms for scheduling nonidentical processors. Journal of the ACM, 23:317–327, 1976. (Cited on p. 83)

132. W.L. Hsu and G.L. Nemhauser. Easy and hard bottleneck location problems. Discrete Applied Mathematics, 1:209–216, 1979. (Cited on p. 53)

133. F. K. Hwang, D. S. Richards, and P. Winter. The Steiner Tree Problem, volume 53 of Annals of Discrete Mathematics. North-Holland, Amsterdam, Netherlands, 1992. (Cited on p. 37)

134. O.H. Ibarra and C.E. Kim. Fast approximation algorithms for the knapsack and sum of subset problems. Journal of the ACM, 22:463–468, 1975. (Cited on p. 73)

135. R. Impagliazzo and D. Zuckerman. How to recycle random bits. In Proc. 30st IEEE Annual Symposium on Foundations of Computer Science, pages 248–253, 1989. (Cited on p. 332)

136. A. Iwainsky, E. Canuto, O. Taraszow, and A. Villa. Network decomposition for the optimization of connection structures. Networks, 16:205–235, 1986. (Cited on p. 37)

137. K. Jain. A factor 2 approximation algorithm for the generalized Steiner network problem. Combinatorica, 1:39–60, 2001. (Cited on p. 231)

138. K. Jain, M. Mahdian, and A. Saberi. A new greedy approach for facility location problems. Manuscript, 2001. (Cited on pp. 242, 254, 331)

139. K. Jain, I. I. M˘andoiu, V.V. Vazirani, and D. P. Williamson. Primal–dual schema based approximation algorithms for the element connectivity problem. In Proc. 10th ACM-SIAM Annual Symposium on Discrete Algorithms, pages 484–489, 1999. (Cited on p. 337)

140. K. Jain and V.V. Vazirani. An approximation algorithm for the fault tolerant metric facility location problem. In Proc. 3rd International Workshop on Approximation Algorithms for Combinatorial Optimization Problems, volume 1913 of Lecture Notes in Computer Science. Springer-Verlag, Berlin, 2000. (Cited on p. 240)

141. K. Jain and V.V. Vazirani. Approximation algorithms for the metric facility location and k-median problems using the primal–dual schema and Lagrangian rela<sub>x</sub>ation<sub>.</sub> Journal of the ACM<sub>,</sub> 48<sub>:</sub>274<sub>–</sub>296<sub>,</sub> 2001<sub>.</sub> (Cited on pp. 242, 253, 254)

142. M. Jerrum and A. Sinclair. The Markov chain Monte Carlo method: an approach to approximate counting. In D.S. Hochbaum, editor, Approximation Algorithms for NP-Hard Problems, pages 482–520. PWS Publishing, Boston, MA, 1997. (Cited on p. 305)

143. M. Jerrum, A. Sinclair, and E. Vigoda. A polynomial-time approximation algorithm for the permanent of a matri ith non negati e entries Electronic Colloquium on Computational Complexity, pages TR00–079, 2000. (Cited on pp. 338, 340)

144. M.R. Jerrum. A very simple algorithm for estimating the number of kcolorings of a low-degree graph. Random Structures and Algorithms, 7, 1995. (Cited on p. 341)

145. M.R. Jerrum and A. Sinclair. Approximating the permanent. SIAM Journal on Computing<sub>,</sub> 18<sub>:</sub>1149<sub>–</sub>1178<sub>,</sub> 1989<sub>.</sub> (Cited on p<sub>.</sub> 305)

146. M.R. Jerrum and A. Sinclair. Polynomial time approximation algorithms for the Ising model. SIAM Journal on Computing, 22:1087–1116, 1993. (Cited on p. 342)

147. M.R. Jerrum, L.G. Valiant, and V.V. Vazirani. Random generation of combinatorial structures from a uniform distribution. Theoretical Computer Science, 43:169–188, 1986. (Cited on p. 303)

148. T. Jiang, M. Li, and D. Du. A note on shortest common superstrings with flipping<sub>.</sub> Information Processing Letters<sub>,</sub> 44<sub>:</sub>195<sub>–</sub>199<sub>,</sub> 1992<sub>.</sub> (Cited on p<sub>.</sub> 67)

149. D.S. Johnson. Near-optimal bin packing algorithms. PhD thesis, Massachusetts Institute of Technology, Department of Mathematics, Cambridge, MA, 1973. (Cited on p. 77)

150. D.S. Johnson. Approximation algorithms for combinatorial problems. Journal of Computer and System Sciences<sub>,</sub> 9<sub>:</sub>256<sub>–</sub>278<sub>,</sub> 1974<sub>.</sub> (Cited on pp<sub>.</sub> 10<sub>,</sub> 26<sub>,</sub> 139)

151. J. Kahn, J.H. Kim, L. Lov´asz, and V.H. Vu. The cover time, the blanket time, and the Matthews bound. In Proc. 41st IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 467<sub>–</sub>475<sub>,</sub> 2000<sub>.</sub> (Cited on p<sub>.</sub> 338)

152. M. Kaib and C.-P. Schnorr. The generalized Gauss reduction algorithm. Journal of Algorithms<sub>,</sub> 21(3)<sub>:</sub>565<sub>–</sub>578<sub>,</sub> 1996<sub>.</sub> (Cited on p<sub>.</sub> 288)

153. R. Kannan. Algorithmic geometry of numbers. In Annual Review of Computer Science<sub>,</sub> Vol<sub>.</sub> 2<sub>,</sub> pages 231<sub>–</sub>267<sub>.</sub> Annual Re<sub>v</sub>ie<sub>w</sub>s<sub>,</sub> Palo Alto<sub>,</sub> CA<sub>,</sub> 1987<sub>.</sub> (Cited on p. 293)

154. R. Kannan. Minkowski’s convex body theorem and integer programming. Mathematics of Operations Research<sub>,</sub> 12(3)<sub>:</sub>415<sub>–</sub>440<sub>,</sub> 1987<sub>.</sub> (Cited on p<sub>.</sub> 293)

155<sub>.</sub> R<sub>.</sub> K<sub>annan,</sub> L<sub>.</sub> L<sub>ov</sub>´<sub>asz, an</sub>d M<sub>.</sub> Si<sub>monov</sub>it<sub>s.</sub> R<sub>an</sub>d<sub>om wa</sub>lk<sub>s an</sub>d <sub>an</sub> o∗(n<sup>5</sup>) volume algorithm for convex bodies. Random Structures and Algorithms, 11:1– 50, 1997. (Cited on p. 338)

156. D. Karger. A randomized fully polynomial time approximation scheme for the all-terminal network reliability problem. SIAM Journal on Computing, 29:492–514, 1999. (Cited on pp. 304, 305)

157. D. Karger, P. Klein, C. Stein, M. Thorup, and N. Young. Rounding algorithms for a geometric embedding of minimum multiway cut. In Proc. 29th ACM Symposium on the Theory of Computing<sub>,</sub> pages 668<sub>–</sub>678<sub>,</sub> 1999<sub>.</sub> (Cited on p. 167)

158. D. Karger, R. Motwani, and M. Sudan. Approximate graph coloring by semidefinite programming<sub>.</sub> Journal of the ACM<sub>,</sub> 45<sub>:</sub>246<sub>–</sub>265<sub>,</sub> 1998<sub>.</sub> (Cited on pp. 267, 269)

159. D. Karger and C. Stein. A new approach to the minimum cut problem. Journal of the ACM<sub>,</sub> 43(4)<sub>:</sub>601<sub>–</sub>640<sub>,</sub> 1996<sub>.</sub> (Cited on p<sub>.</sub> 304)

160. H. Karlof. Linear Programming. Birkh¨auser, Boston, MA, 1991. (Cited on p. 107)

161. H. Karlof. How good is the Goemans-Williamson MAX CUT algorithm. SIAM Journal on Computing<sub>,</sub> 29<sub>:</sub>336<sub>–</sub>350<sub>,</sub> 1999<sub>.</sub> (Cited on p<sub>.</sub> 268)

162. H. Karlof and U. Zwick. A 7/8-approximation algorithm for MAX-3SAT? In Proc. 38th IEEE Annual Symposium on Foundations of Computer Science, pages 406–415, 1997. (Cited on p. 332)

163. N. Karmakar and R.M. Karp. An eficient approximation scheme for the onedimensional bin packing problem. In Proc. 23rd IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 312<sub>–</sub>320<sub>,</sub> 1982<sub>.</sub> (Cited on p<sub>.</sub> 78)

164. R.M. Karp. Reducibility among combinatorial problems. In R.E. Miller and J.W. Thatcher, editors, Complexity of Computer Computations, pages 85–103. Plenum Press, New York, NY, 1972. (Cited on p. 10)

165. R.M. Karp and M. Luby. Monte Carlo algorithms for enumeration and reliability problems. In Proc. 24th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 56<sub>–</sub>64<sub>,</sub> 1983<sub>.</sub> (Cited on pp<sub>.</sub> 302<sub>,</sub> 305)

166. R.M. Karp, M. Luby, and N. Madras. Monte Carlo approximation algorithms for enumeration problems. Journal of Algorithms, 10:429–448, 1989. (Cited on p. 305)

167. A. Karzanov and L. Khachiyan. On the conductance of order Markov chains. Technical Report DCS 268<sub>,</sub> Rutgers Uni<sub>v</sub>ersit<sub>y,</sub> 1990<sub>.</sub> (Cited on p<sub>.</sub> 340)

168. P.W. Kasteleyn. Graph theory and crystal physics. In F. Harary, editor, Graph Theory and Theoretical Physics, pages 43–110. Academic Press, New York<sub>,</sub> NY<sub>,</sub> 1967<sub>.</sub> (Cited on p<sub>.</sub> 338)

169<sub>.</sub> S<sub>.</sub> Khuller<sub>,</sub> R<sub>.</sub> Pless<sub>,</sub> and Y<sub>.</sub>J<sub>.</sub> Sussmann<sub>.</sub> Fault tolerant k<sub>-</sub>center problems<sub>.</sub> Theoretical Computer Science<sub>,</sub> 242<sub>:</sub>237<sub>–</sub>245<sub>,</sub> 2000<sub>.</sub> (Cited on pp<sub>.</sub> 52<sub>,</sub> 53)

170. S. Khuller and B. Raghavachari. Improved approximation algorithms for uniform connectivity problems. Journal of Algorithms, 21:434–450, 1996. (Cited on p. 336)

171. S. Khuller and V.V. Vazirani. Planar graph colourability is not self-reducible, assuming P = NP. Theoretical Computer Science, 88(1):183–190, 1991. (Cited on p. 351)

172. S. Khuller and U. Vishkin. Biconnectivity approximations and graph carvings. Journal of the ACM<sub>,</sub> 42<sub>,</sub> 2<sub>:</sub>214<sub>–</sub>235<sub>,</sub> 1994<sub>.</sub> (Cited on p<sub>.</sub> 228)

173. P. Klein, S. Rao, A. Agrawal, and R. Ravi. An approximate max-flow min-cut relation for undirected multicommodity flow, with applications. Combinatorica, 15:187–202, 1995. (Cited on pp. 179, 197)

174. D.E. Knuth. The Art of Computer Programming. Vol. 2. Seminumerical Algorithms. Second edition. Addison-Wesley, Reading, MA, 1981. (Cited on p. 266)

175. A. Korkine and G. Zolotaref. Sur les formes quadratiques. Math. Annalen, 6:366–389, 1873. (Cited on p. 290)

176. M. Korupolu, C. Plaxton, and R. Rajaraman. Analysis of a local search heuristic for facility location problems. In Proc. 9th ACM-SIAM Annual Symposium on Discrete Algorithms<sub>,</sub> pages 1<sub>–</sub>10<sub>,</sub> 1998<sub>.</sub> (Cited on p<sub>.</sub> 253)

177. L. Kou, G. Markowsky, and L. Berman. A fast algorithm for Steiner trees. Acta Informatica<sub>,</sub> 15<sub>:</sub>141<sub>–</sub>145<sub>,</sub> 1981<sub>.</sub> (Cited on p<sub>.</sub> 37)

178. M.W. Krentel. The complexity of optimization problems. Journal of Computer and System Sciences<sub>,</sub> 36<sub>:</sub>490<sub>–</sub>509<sub>,</sub> 1988<sub>.</sub> (Cited on p<sub>.</sub> 351)

179. H.W. Kuhn. The Hungarian method for the assignment problem. Naval Research Logistics Quarterly<sub>,</sub> 2<sub>:</sub>83<sub>–</sub>97<sub>,</sub> 1955<sub>.</sub> (Cited on p<sub>.</sub> 129)

180. J. Lagarias. Worst case complexity bounds for algorithms in the the theory of integral quadratic forms. Journal of Algorithms, 1:142–186, 1980. (Cited on p. 292)

181. J. Lagarias, H.W. Lenstra, Jr., and C.-P. Schnorr. Korkin–Zolotarev bases and successive minima of a lattice and its reciprocal lattice. Combinatorica, 10:333–348, 1990. (Cited on p. 293)

182. T. Leighton and S. Rao. Multicommodity max-flow min-cut theorems and their use in designing approximation algorithms. Journal of the ACM, 46:787– 832, 1999. (Cited on p. 197)

183. A.K. Lenstra, H.W. Lenstra, Jr., and L. Lov´asz. Factoring polynomials with rational coeficients<sub>.</sub> Math. Ann.<sub>,</sub> 261<sub>:</sub>513<sub>–</sub>534<sub>,</sub> 1982<sub>.</sub> (Cited on p<sub>.</sub> 292)

184. J.K. Lenstra, D.B. Shmoys, and E. Tardos. Approximation algorithms for <sup>´</sup> scheduling unrelated parallel machines. Mathematical Programming, 46:259– 271, 1990. (Cited on p. 145)

185. H.W. Lenstra, Jr. Integer programming with a fixed number of variables. Mathematics of Operations Research<sub>,</sub> 8<sub>:</sub>538<sub>–</sub>548<sub>,</sub> 1983<sub>.</sub> (Cited on p<sub>.</sub> 78)

186. L.A. Levin. Universal sorting problems. Problemy Peredaci Informacii, 9:115– 116, 1973. English translation in Problems of Information Transmission 9:265– 266. (Cited on p. 10)

187. M. Li. Towards a DNA sequencing theory. In Proc. 31st IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 125<sub>–</sub>134<sub>,</sub> 1990<sub>.</sub> (Cited on p. 26)

188. J. H. Lin and J. S. Vitter. Approximation algorithms for geometric median problems<sub>.</sub> Information Processing Letters<sub>,</sub> 44<sub>:</sub>245<sub>–</sub>249<sub>,</sub> 1992<sub>.</sub> (Cited on p<sub>.</sub> 251)

189. J. H. Lin and J. S. Vitter. ε-approximation with minimum packing constraint violation. In Proc. 24th ACM Symposium on the Theory of Computing, pages 771–782, 1992. (Cited on p. 254)

190. N. Linial, E. London, and Y. Rabinovich. The geometry of graphs and some of its algorithmic applications. Combinatorica, 15:215–245, 1995. (Cited on pp. 196, 197, 266)

191. C.H.C. Little. An extension of Kasteleyn’s method of enumerating 1-factors of planar graphs. In D. Holton, editor, Proc. 2nd Australian Conference on Combinatorial Mathematics, volume 403 of Lecture Notes in Computer Science<sub>,</sub> pages 63<sub>–</sub>72<sub>.</sub> Springer<sub>-</sub>Verlag<sub>,</sub> Berlin<sub>,</sub> 1974<sub>.</sub> (Cited on p<sub>.</sub> 338)

192. L. Lov´asz. On the ratio of optimal integral and fractional covers. Discrete Mathematics<sub>,</sub> 13<sub>:</sub>383<sub>–</sub>390<sub>,</sub> 1975<sub>.</sub> (Cited on pp<sub>.</sub> 11<sub>,</sub> 26<sub>,</sub> 118)

193. L. Lov´asz. An Algorithmic Theory of Numbers, Graphs and Convexity. CBMS-NSF R i l C f S i i A li d M th ti 50 SIAM Phil d l phia<sub>,</sub> PA<sub>,</sub> 1986<sub>.</sub> (Cited on p<sub>.</sub> 291)

194. L. Lov´asz. Combinatorial Problems and Exercises. Second edition. North-Holland, Amsterdam–New York, 1993. (Cited on pp. 107, 339, 341)

195. L. Lov´asz and M.D. Plummer. Matching Theory. North-Holland, Amsterdam– Ne<sub>w</sub> York<sub>,</sub> 1986<sub>.</sub> (Cited on pp<sub>.</sub> 8<sub>,</sub> 11<sub>,</sub> 107)

196. L. Lov´asz and A. Schrijver. Cones of matrices and set functions, and 0-1 optimi<sub>z</sub>ation<sub>.</sub> SIAM Journal on Optimization<sub>,</sub> 1<sub>:</sub>166<sub>–</sub>190<sub>,</sub> 1990<sub>.</sub> (Cited on p. 269)

197. A. Lubotzky, R. Phillips, and P. Sarnak. Ramanujan graphs. Combinatorica, 8:261–277, 1988. (Cited on p. 332)

198. M. Luby and E. Vigoda. Approximately counting up to four. In Proc. 29th ACM Symposium on the Theory of Computing<sub>,</sub> pages 682<sub>–</sub>687<sub>,</sub> 1997<sub>.</sub> (Cited on p. 341)

199. C. Lund and M. Yannakakis. On the hardness of approximating minimization problems<sub>.</sub> Journal of the ACM<sub>,</sub> 41<sub>:</sub>960<sub>–</sub>981<sub>,</sub> 1994<sub>.</sub> (Cited on pp<sub>.</sub> 26<sub>,</sub> 332)

200. S. Mahajan and H. Ramesh. Derandomizing semidefinite programming based approximation algoirthms. In Proc. 36th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 162<sub>–</sub>169<sub>,</sub> 1995<sub>.</sub> (Cited on p<sub>.</sub> 268)

201. M. Mahdian, E. Markakis, A. Saberi, and V. V. Vazirani. A greedy facility location algorithm analyzed using dual fitting. In Proc. 4th International Workshop on Approximation Algorithms for Combinatorial Optimization Problems, volume 2129 of Lecture Notes in Computer Science. Springer-Verlag, Berlin, 2001. (Cited on pp. 241, 242)

202. P. Matthews. Generating random linear extensions of a partial order. The Annals of Probability, 19:1367–1392, 1991. (Cited on p. 340)

203. L. McShine and P. Tetali. On the mixing time of the triangulation walk and other Catalan structures. Randomization methods in Algorithm Design, DIMACS<sub>-</sub>AMS<sub>,</sub> 43<sub>:</sub>147<sub>–</sub>160<sub>,</sub> 1998<sub>.</sub> (Cited on p<sub>.</sub> 340)

204. D. Micciancio. The shortest vector in a lattice is hard to approximate to within some constant. In Proc. 39th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 92<sub>–</sub>98<sub>,</sub> 1998<sub>.</sub> (Cited on p<sub>.</sub> 336)

205. M. Mihail. On coupling and the approximation of the permanent. Information Processing Letters, 30:91–95, 1989. (Cited on p. 305)

206. M. Mihail. Set cover with requirements and costs evolving over time. In International Workshop on Randomization, Approximation and Combinatorial Optimization, volume 1671 of Lecture Notes in Computer Science, pages 63<sub>–</sub>72<sub>.</sub> Springer<sub>-</sub>Verlag<sub>,</sub> Berlin<sub>,</sub> 1999<sub>.</sub> (Cited on p<sub>.</sub> 117)

207. J.S.B. Mitchell. Guillotine subdivisions approximate polygonal subdivisions: a simple pol<sub>y</sub>nomial<sub>-</sub>time appro<sub>x</sub>imation scheme for geometric TSP<sub>,</sub> k<sub>-</sub>MST<sub>,</sub> and related problems. SIAM Journal on Computing, 28:1298–1309, 1999. (Cited on p. 89)

208. B. Morris. Improved bounds for sampling contingency tables. In International Workshop on Randomization, Approximation and Combinatorial Optimization, volume 1671 of Lecture Notes in Computer Science, pages 121–129. Springer<sub>-</sub>Verlag<sub>,</sub> Berlin<sub>,</sub> 1999<sub>.</sub> (Cited on p<sub>.</sub> 340)

209. R. Motwani and P. Raghavan. Randomized Algorithms. Cambridge University Press<sub>,</sub> Cambridge<sub>,</sub> UK<sub>,</sub> 1995<sub>.</sub> (Cited on p<sub>.</sub> 354)

210. J. Naor and L. Zosin. A 2-approximation algorithm for the directed multiway cut problem. In Proc. 38th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 548<sub>–</sub>553<sub>,</sub> 1997<sub>.</sub> (Cited on p<sub>.</sub> 167)

211. M. Naor, L. Schulman, and A. Srinivasan. Splitters and near-optimal derandomization. In Proc. 36th IEEE Annual Symposium on Foundations of Computer Science<sub>,</sub> pages 182<sub>–</sub>191<sub>,</sub> 1995<sub>.</sub> (Cited on p<sub>.</sub> 332)

212. G. Nemhauser and L. Wolsey. Integer and Combinatorial Optimization. John Wile<sub>y</sub> & Sons<sub>,</sub> Ne<sub>w</sub> York<sub>,</sub> NY<sub>,</sub> 1988<sub>.</sub> (Cited on p<sub>.</sub> 107)

213. G.L. Nemhauser and L.E. Trotter. Vertex packings: structural properties and algorithms. Mathematical Programming, 8:232–248, 1975. (Cited on p. 124)

214. Y. Nesterov and A. Nemirovskii. Interior Point Polynomial Methods in Convex Programming. SIAM, Philadelphia, PA, 1994. (Cited on p. 268)

215. M.L. Overton. On minimizing the maximum eigenvalue of a symmetric matrix. SIAM J<sub>.</sub> on Matrix Analysis and Appl<sub>.,</sub> 13<sub>:</sub>256<sub>–</sub>268<sub>,</sub> 1992<sub>.</sub> (Cited on p<sub>.</sub> 268)

216. C.H. Papadimitriou. Computational Complexity. Addison-Wesley, Reading, MA, 1994. (Cited on p. 351)

217. C.H. Papadimitriou and K. Steiglitz. Combinatorial Optimization: Algorithms and Complexity. Prentice-Hall, Englewood Clifs, NJ, 1982. (Cited on pp. 11, 107)

218. C.H. Papadimitriou and M. Yannakakis. Optimization, approximation, and complexity classes. Journal of Computer and System Sciences, 43:425–440, 1991. (Cited on pp. 332, 351)

219. C.H. Papadimitriou and M. Yannakakis. The traveling salesman problem with distances one and two. Mathematics of Operations Research, 18:1–11, 1993. (Cited on p. 34)

220. M. Pinsker. On the complexity of a concentrator. In Proc. 7th Annual Teletrafic Conference<sub>,</sub> pages 318/1<sub>–</sub>318/4<sub>,</sub> 1973<sub>.</sub> (Cited on p<sub>.</sub> 179)

221. J. Plesn´ık. A bound for the Steiner tree problem in graphs. Math. Slovaca, 31:155–163, 1981. (Cited on p. 37)

222. V.R. Pratt. Every prime has a succinct certificate. SIAM Journal on Computing, 4:214–220, 1975. (Cited on p. 9)

223. H. J. Pr¨omel and A. Steger. RNC-approximation algorithms for the Steiner problem. In Proc. Symposium on Theoretical Aspects of Computer Science, volume 1200 of Lecture Notes in Computer Science, pages 559–570. Springer-Verlag, Berlin, 1997. (Cited on p. 212)

224. M.O. Rabin. Probabilistic algorithms. In J.F. Traub, editor, Algorithms and Complexity, Recent Results and New Directions, pages 21–39. Academic Press, Ne<sub>w</sub> York<sub>,</sub> NY<sub>,</sub> 1976<sub>.</sub> (Cited on p<sub>.</sub> 11)

225. P. Raghavan. Probabilistic construction of deterministic algorithms: approximating packing integer programs. Journal of Computer and System Sciences, 37:130–143, 1988. (Cited on p. 139)

226. S. Rajagopalan and V.V. Vazirani. On the bidirected cut relaxation for the metric Steiner tree problem. In Proc. 10th ACM-SIAM Annual Symposium on Discrete Algorithms<sub>,</sub> pages 742<sub>–</sub>751<sub>,</sub> 1999<sub>.</sub> (Cited on pp<sub>.</sub> 211<sub>,</sub> 335)

227. S. Rajagopalan and V.V. Vazirani. Primal–dual RNC approximation algorithms for set cover and covering integer programs. SIAM Journal on Computing, 28:526–541, 1999. (Cited on p. 118)

228. D. Randall and D.B. Wilson. Sampling spin configurations of an Ising system. In Proc. 10th ACM-SIAM Annual Symposium on Discrete Algorithms, pages S959<sub>–</sub>960<sub>,</sub> 1999<sub>.</sub> (Cited on p<sub>.</sub> 342)

229. S. Rao and W.D. Smith. Approximating geometrical graphs via “spanners” and “banyans”. In Proc. 30th ACM Symposium on the Theory of Computing, pages 540–550, 1998. (Cited on p. 89)

230. S.K. Rao, P. Sadayappan, F.K. Hwang, and P.W. Shor. The rectilinear Steiner arborescence problem. Algorithmica, 7:277–288, 1992. (Cited on p. 35)

231. R. Raz. A parallel repetition theorem. SIAM Journal on Computing, 27:763– 803, 1998. (Cited on p. 332)

232. S.K. Sahni and T.F. Gonzalez. P-complete approximation problems. Journal of the ACM<sub>,</sub> 23<sub>:</sub>555<sub>–</sub>565<sub>,</sub> 1976<sub>.</sub> (Cited on p<sub>.</sub> 37)

233. H. Saran and V.V. Vazirani. Finding k-cuts within twice the optimal. SIAM Journal on Computing<sub>,</sub> 24<sub>:</sub>101<sub>–</sub>108<sub>,</sub> 1995<sub>.</sub> (Cited on p<sub>.</sub> 46)

234. C.P. Schnorr. Optimal algorithms for self-reducible problems. In Proc. 3rd International Colloquium on Automata, Languages, and Programming, pages 322–337, 1976. (Cited on p. 351)

235. C.P. Schnorr. A hierarchy of polynomial time lattice basis reduction algorithms<sub>.</sub> Theoretical Computer Science<sub>,</sub> 53<sub>:</sub>201<sub>–</sub>224<sub>,</sub> 1987<sub>.</sub> (Cited on p<sub>.</sub> 292)

236. P. Schreiber. On the history of the so-called Steiner Weber problem. Wiss. Z. Ernst-Moritz-Arndt-Univ. Greifswald, Math.-nat.wiss. Reihe, 35, 3, 1986. (Cited on p. 37)

237. A. Schrijver. Theory of Linear and Integer Programming. John Wiley & Sons, Ne<sub>w</sub> York<sub>,</sub> NY<sub>,</sub> 1986<sub>.</sub> (Cited on p<sub>.</sub> 107)

238. P.D. Seymour. Packing directed circuits fractionally. Combinatorica, 15:281– 288, 1995. (Cited on p. 337)

239. D.B. Shmoys, E. Tardos, and K.I. Aardal. Approximation algorithms for<sup>´</sup> facility location problems. In Proc. 29th ACM Symposium on the Theory of Computing<sub>,</sub> pages 265<sub>–</sub>274<sub>,</sub> 1997<sub>.</sub> (Cited on p<sub>.</sub> 242)

240. D.B. Shmoys and D.P. Williamson. Analyzing the Held-Karp TSP bound: a monotonicity property with applications. Information Processing Letters, 35:281–285, 1990. (Cited on p. 231)

241. A. Sinclair. Improved bounds for mixing rates of Markov chains and multicommodity flow. Combinatorics, Probability and Computing, 1:351–370, 1992. (Cited on p. 197)

242. A. Sinclair. Algorithms for Random Generation and Counting: a Markov Chain Approach<sub>.</sub> Birkh¨auser<sub>,</sub> Boston<sub>,</sub> MA<sub>,</sub> 1993<sub>.</sub> (Cited on p<sub>.</sub> 305)

243. J. Spencer. Ten Lectures on the Probabilistic Method. SIAM, Philadelphia, PA, 1987. (Cited on pp. 139, 354)

244. A. Srinivasan. Improved approximations of packing and covering problems. In Proc. 27th ACM Symposium on the Theory of Computing, pages 268–276, 1995. (Cited on p. 124)

245. R.H. Swendsen and J.S. Wang. Non-universal critical dynamics in Monte Carlo simulations. Physics Review Letters, 58:86–90, 1987. (Cited on p. 342)

246. R.E. Tarjan. Data Structures and Network Algorithms. SIAM, Philadelphia, PA, 1983. (Cited on p. 11)

247. L. Trevisan. Non-approximability results for optimization problems on bounded degree instance. In Proc. 33rd ACM Symposium on the Theory of Computing<sub>,</sub> 2001<sub>.</sub> (Cited on p<sub>.</sub> 334)

248. J.D. Ullman. The performance of a memory allocation algorithm. Technica Report 100, Princeton University, Princeton, NJ, 1971. (Cited on p. 78)

249. L.G. Valiant. The complexity of computing the permanent. Theoretical Computer Science<sub>,</sub> 8<sub>:</sub>189<sub>–</sub>201<sub>,</sub> 1979<sub>.</sub> (Cited on p<sub>.</sub> 305)

250. L. Vandeberghe and S. Boyd. Semidefinite programming. SIAM Review, 38:49–95, 1996. (Cited on p. 268)

251. V.V. Vazirani. NC algorithms for computing the number of perfect matchings in $K _ { 3 , 3 }$ -free graphs and related problems. Information and Computation, 80:152–164, 1989. (Cited on p. 338)

252. V.V. Vazirani and M. Yannakakis. Suboptimal cuts: their enumeration, weight and number. In Proc. 19th International Colloquium on Automata, Languages, and Programming, volume 623 of Lecture Notes in Computer Science, pages 366<sub>–</sub>377<sub>.</sub> Springer<sub>-</sub>Verlag<sub>,</sub> Berlin<sub>,</sub> 1992<sub>.</sub> (Cited on p<sub>.</sub> 304)

253. D.L. Vertigan and D.J.A. Welsh. The computational complexity of the Tutte plane. Combinatorics, Probability and Computing, 1:181–187, 1992. (Cited on p. 342)

254. E. Vigoda. Improved bounds for sampling colorings. In Proc. 40th IEEE Annual Symposium on Foundations of Computer Science, pages 51–59, 1999. (Cited on p. 341)

255<sub>.</sub> V<sub>.</sub>G<sub>.</sub> Vi<sub>z</sub>ing<sub>.</sub> On an estimate of the chromatic class of a p<sub>-</sub>graph<sub>.</sub> Diskret. Analiz., 3:25–30, 1964 (in Russian). (Cited on p. 10)

256. D.J.A. Welsh. Knots, Colourings and Counting. Cambridge University Press, Cambridge<sub>,</sub> UK<sub>,</sub> 1993<sub>.</sub> (Cited on p<sub>.</sub> 342)

257. A. Wigderson. Improving the performance guarantee for approximate graph coloring<sub>.</sub> Journal of the ACM<sub>,</sub> 30<sub>:</sub>729<sub>–</sub>735<sub>,</sub> 1983<sub>.</sub> (Cited on p<sub>.</sub> 23)

258. D.P. Williamson, M.X. Goemans, M. Mihail, and V.V. Vazirani. A primal– dual approximation algorithm for generalized Steiner network problems. Combinatorica<sub>,</sub> 15<sub>:</sub>435<sub>–</sub>454<sub>,</sub> 1995<sub>.</sub> (Cited on pp<sub>.</sub> 130<sub>,</sub> 224)

259. D. B. Wilson. Generating random spanning trees more quickly than the cover time. In Proc. 30th ACM Symposium on the Theory of Computing, pages 296–303, 1996. (Cited on p. 339)

260. L.A. Wolsey. Heuristic analysis, linear programming and branch and bound. Mathematical Programming Study, 13:121–134, 1980. (Cited on pp. 231, 268)

261. M. Yannakakis. On the approximation of maximum satisfiability. Journal of Algorithms<sub>,</sub> 3<sub>:</sub>475<sub>–</sub>502<sub>,</sub> 1994<sub>.</sub> (Cited on p<sub>.</sub> 139)

262. A.Z. Zelikovsky. An 11/6-approximation algorithm for the network Steiner problem. Algorithmica, 9:463–470, 1993. (Cited on p. 212)

263. A.Z. Zelikovsky and I. I. M˘andoiu. Practical approximation algorithms for d b d d k t I P 12th ACM SIAM A l S i on Discrete Algorithms<sub>,</sub> pages 407<sub>–</sub>416<sub>,</sub> 2001<sub>.</sub> (Cited on p<sub>.</sub> 37)

## 2CNF <sub>c</sub>l<sub>ause</sub> d<sub>e</sub>l<sub>e</sub>ti<sub>on</sub> 176<sub>,</sub> 179

A<sub>cyc</sub>li<sub>c</sub> <sub>su</sub>b<sub>grap</sub>h 7<sub>,</sub> 334 Antichain cover 8

B<sub>an</sub>d<sub>w</sub>idth <sub>m</sub>i<sub>n</sub>i<sub>m</sub>i<sub>za</sub>ti<sub>on</sub> 196 B<sub>e</sub>t<sub>weenness</sub> 267 Bi<sub>n cover</sub>i<sub>ng</sub> 77 Bi<sub>n pac</sub>ki<sub>ng</sub> 74<sub>,</sub> 74<sub>–</sub>78<sub>,</sub> 80<sub>,</sub> 124 – with fixed number of object sizes

## Chain cover 8

Clique 9, 306, 309, 318–322 Closest vector 292 Clustering 243 – <sup>2</sup><sub>2</sub> 253, 254 <sub>– me</sub>t<sub>r</sub>i<sub>c</sub> k<sub>-c</sub>l<sub>us</sub>t<sub>er</sub> 52 Counting problems 294–305 – acyclic orientations 338 antichains 340 bases of a matroid 339 colorings of a graph 341 contingency tables 340 DNF solutions 295, 305 – weighted version 302 – Euler tours 339 – forests 339 graphs with given degree sequence 340 Hamiltonian cycles 341 – independent sets 341 perfect matchings 305, 338 simple c cles in a directed graph 303 – stable marriages 340 t 340 – triangulations 340 – volume of a convex body 338

Cover time 337 Covering integer programs 112, 116, 118

## Cycle cover 35, 62

Dominating set 48, 50, 52

Edge coloring 10 Ed<sub>ge expans</sub>i<sub>on</sub> 192 Enumerating cuts 304

Feedback edge set – directed 337 <sub>– su</sub>b<sub>se</sub>t 166<sub>,</sub> 166<sub>,</sub> 167 F db k t t 25 54 54 60 129, 166 – directed 337 <sub>–</sub> <sub>su</sub>b<sub>se</sub>t 166<sub>,</sub> 166<sub>,</sub> 167<sub>,</sub> 336

Graph bipartization by edge deletion 178

Hamiltonian cycle 30, 303

Independent set 48, 51–53 – maximal 239

K<sub>napsac</sub>k 68<sub>,</sub> 68<sub>–</sub>73

Linear equations over GF[2] 138

M<sub>a</sub>t<sub>c</sub>hi<sub>ng</sub> 3<sub>,</sub> 104 – b-matching 152, 227 – bipartite 129 – – maximum weight 129 <sub>–</sub> <sub>max</sub>i<sub>ma</sub>l 3<sub>,</sub> 8 i i di lit 8 <sub>– max</sub>i<sub>mum</sub> 3<sub>,</sub> 5<sub>,</sub> 9<sub>,</sub> 124<sub>,</sub> 152<sub>,</sub> 153 – minimum weight 107 <sub>–</sub> <sub>per</sub>f<sub>ec</sub>t 105<sub>,</sub> 142<sub>,</sub> 143

– – minimum weight 32, 35, 62, 105, 230

Matroid intersection 228

M<sub>a</sub>t<sub>ro</sub>id <sub>par</sub>it<sub>y</sub> 212<sub>,</sub> 212

MAX k<sub>-</sub>CUT 23<sub>,</sub> 138<sub>,</sub> 267<sub>,</sub> 269

Maximum antichain 8

M<sub>ax</sub>i<sub>mum</sub> <sub>coverage</sub> 25

Maximum cut (MAX-CUT) 10, 22, 138<sub>,</sub> 255<sub>,</sub> 255<sub>,</sub> 256<sub>,</sub> 260<sub>–</sub>263<sub>,</sub> 267<sub>,</sub> 268<sub>,</sub> 334

M<sub>e</sub>t<sub>r</sub>i<sub>c</sub> k<sub>-cen</sub>t<sub>er</sub> 47<sub>,</sub> 47<sub>–</sub>50<sub>,</sub> 53 – fault-tolerant 52

directed 23, 138, 267, 269

<sub>–</sub> <sub>we</sub>i<sub>g</sub>ht<sub>e</sub>d 50<sub>,</sub> 50<sub>–</sub>52

– MAX k-FUNCTION SAT 312

Ma<sub>x</sub>imum satisfiabilit<sub>y</sub> (MAX<sub>-</sub>SAT) 9<sub>,</sub> 131<sub>,</sub> 131<sub>–</sub>139<sub>,</sub> 263<sub>,</sub> 306

M<sub>e</sub>t<sub>r</sub>i<sub>c</sub> k<sub>-me</sub>di<sub>an</sub> 243<sub>,</sub> 243<sub>–</sub>254<sub>,</sub> 337

M<sub>ax</sub>i<sub>mum</sub> fl<sub>ow</sub> 38<sub>,</sub> 97<sub>,</sub> 97<sub>–</sub>100<sub>,</sub> 168

M<sub>e</sub>t<sub>r</sub>i<sub>c</sub> k<sub>-</sub>MST 252 <sub>–</sub> MAX<sub>-</sub>2SAT 131<sub>,</sub> 263<sub>,</sub> 268 <sub>–</sub> MAX<sub>-</sub>3SAT 131<sub>,</sub> 309<sub>,</sub> 311<sub>–</sub>315<sub>,</sub> 322<sub>,</sub> 323, 326, 330, 331 ith b d d f variables 313–316, 330

Metric facility location

– capacitated 240, 337 f lt t l t 240 – metric uncapacitated 242

– prize-collecting 240 <sub>– uncapac</sub>it<sub>a</sub>t<sub>e</sub>d 232<sub>,</sub> 232<sub>–</sub>239<sub>,</sub> 242<sub>,</sub> 337

Minimum k-connected subgraph <sub>– e</sub>d<sub>ge</sub> 228

<sub>– ver</sub>t<sub>ex</sub> 226

– uniform parallel machines 140, 145

Mi<sub>n</sub>i<sub>mum</sub> k<sub>-cu</sub>t 38<sub>,</sub> 40<sub>–</sub>44

Mi<sub>n</sub>i<sub>mum</sub> bi<sub>sec</sub>ti<sub>on</sub> 193<sub>,</sub> 196<sub>,</sub> 197<sub>,</sub> 336

Minimum spanning tree (MST) 28–31, 105, 206, 207, 212

Minimum chain cover 8

Minimum cut 38, 298

Multicommodity flow 97, 147, 163

<sub>–</sub> b<sub>-</sub>b<sub>a</sub>l<sub>ance</sub>d 193<sub>,</sub> 193<sub>–</sub>194<sub>,</sub> 196<sub>,</sub> 197<sub>,</sub> 336

<sub>–</sub> s<sub>–</sub>t 38<sub>,</sub> 98<sub>,</sub> 97<sub>–</sub>100<sub>,</sub> 146

Minimum cut linear arrangement 194<sub>,</sub> 194<sub>–</sub>195<sub>,</sub> 197

<sub>–</sub> d<sub>eman</sub>d<sub>s</sub> 168<sub>,</sub> 180<sub>,</sub> 180<sub>–</sub>197

Minimum length linear arrangement 178

Minimum makespan scheduling 9, 10, 79<sub>,</sub> 79<sub>–</sub>83<sub>,</sub> 140

– directed 165

<sub>–</sub> i<sub>n</sub>t<sub>eger</sub> 148<sub>,</sub> 153<sub>,</sub> 154<sub>,</sub> 337

– – in trees 146–154

– – in trees of height one 152

– – in unit capacity trees 153

<sub>– sum</sub> 168<sub>,</sub> 168<sub>–</sub>176<sub>,</sub> 179

– uniform 192, 197

M<sub>u</sub>lti<sub>cu</sub>t 146<sub>,</sub> 153<sub>,</sub> 168<sub>–</sub>179<sub>,</sub> 336

– directed 337

– in trees 146–154, 166

– in trees of height one 152

M<sub>u</sub>lti<sub>way cu</sub>t 38<sub>,</sub> 38<sub>–</sub>40<sub>,</sub> 155<sub>–</sub>167<sub>,</sub> 335

– bidirected integer program formula-

<sub>–</sub> di<sub>rec</sub>t<sub>e</sub>d 165<sub>,</sub> 166<sub>,</sub> 167

– fractional 156

<sub>–</sub> <sub>no</sub>d<sub>e</sub> 160<sub>,</sub> 160<sub>–</sub>163<sub>,</sub> 166

## Network design

– element connectivity 337

– vertex connectivity 336

N<sub>e</sub>t<sub>wor</sub>k <sub>re</sub>li<sub>a</sub>bilit<sub>y</sub> 297<sub>,</sub> 304<sub>,</sub> 305<sub>,</sub> 339

– s–t reliability 339 – global 339

## P<sub>o</sub>i<sub>n</sub>t<sub>-</sub>t<sub>o-po</sub>i<sub>n</sub>t <sub>connec</sub>ti<sub>on</sub> 208

S<sub>a</sub>ti<sub>s</sub>fi<sub>a</sub>bilit<sub>y</sub> (SAT) 9<sub>,</sub> 330<sub>,</sub> 343<sub>,</sub> 344<sub>–</sub> 3SAT 310<sub>,</sub> 343

Scheduling on unrelated parallel <sub>mac</sub>hi<sub>nes</sub> 140<sub>,</sub> 140<sub>–</sub>145

S<sub>em</sub>id<sub>e</sub>fi<sub>n</sub>it<sub>e</sub> <sub>programm</sub>i<sub>ng</sub> 258<sub>,</sub> 255–269

S<sub>e</sub>t <sub>cover</sub> VIII<sub>,</sub> 11<sub>,</sub> 15<sub>,</sub> 15<sub>–</sub>26<sub>,</sub> 34<sub>,</sub> 108–122, 124–130, 239, 251, 306, 309, 322–329, 334

– constrained set multicover 112, 116, 118

– multiset multicover 112, 116, 117, 123

set multicover 24, 112, 116, 123

ith co ca e costs 117

Sh<sub>or</sub>t<sub>es</sub>t <sub>supers</sub>t<sub>r</sub>i<sub>ng</sub> 9<sub>,</sub> 20<sub>,</sub> 19<sub>–</sub>22<sub>,</sub> 26<sub>,</sub> 61–67 – variants 25, 67

Sh<sub>or</sub>t<sub>es</sub>t <sub>vec</sub>t<sub>or</sub> 273<sub>,</sub> 273<sub>–</sub>293<sub>,</sub> 336

– asymmetric 34, 336

S<sub>parses</sub>t <sub>cu</sub>t 180<sub>,</sub> 180<sub>–</sub>197<sub>,</sub> 336<sub>,</sub> 337

Steiner arborescence tili 35

<sub>–</sub> E<sub>uc</sub>lid<sub>ean</sub> 84<sub>,</sub> 84<sub>–</sub>89

St<sub>e</sub>i<sub>ner</sub> f<sub>ores</sub>t 198<sub>,</sub> 198<sub>–</sub>213 – metric 30–33, 37, 229, 231, 334 – – lengths one and two 34 – – variants 34

St<sub>e</sub>i<sub>ner ne</sub>t<sub>wor</sub>k 213<sub>,</sub> 213<sub>–</sub>231<sub>,</sub> 335

Tutte polynomial 341

V<sub>er</sub>t<sub>ex co</sub>l<sub>or</sub>i<sub>ng</sub> 23

– Euclidean 89

– prize-collecting 208, 252

Subset sum 291

S<sub>u</sub>b<sub>se</sub>t<sub>-sum ra</sub>ti<sub>o pro</sub>bl<sub>em</sub> 72

V<sub>er</sub>t<sub>ex</sub> <sub>cover</sub> 1<sub>,</sub> 15<sub>,</sub> 17<sub>–</sub>19<sub>,</sub> 23<sub>,</sub> 24<sub>,</sub> 104<sub>,</sub> 122–124, 129, 146, 152, 166, 306, 307, 309, 334

Survivable network design see Steiner network and network design

<sub>–</sub> <sub>car</sub>di<sub>na</sub>lit<sub>y</sub> 1<sub>,</sub> 2<sub>–</sub>5<sub>,</sub> 8<sub>,</sub> 152

Traveling salesman problem (TSP) 30<sub>,</sub> 229<sub>,</sub> 231

Zero-skew tree

<sub>– rec</sub>tili<sub>near</sub> 36<sub>,</sub> 37

## Subject Index

α-min cut 304 #P 294<sub>,</sub> 305 1-tree 230 A<sub>c</sub>ti<sub>ve se</sub>t 200<sub>,</sub> 209 A<sub>pprox</sub>i<sub>ma</sub>ti<sub>on a</sub>l<sub>gor</sub>ith<sub>m</sub> 2<sub>,</sub> 345<sub>–</sub>347 <sub>– approx</sub>i<sub>ma</sub>ti<sub>on</sub> f<sub>ac</sub>t<sub>or</sub> 346 <sub>– ran</sub>d<sub>om</sub>i<sub>ze</sub>d 346 A<sub>pprox</sub>i<sub>ma</sub>ti<sub>on sc</sub>h<sub>eme</sub> 68 – fully polynomial randomized (FPRAS) 295<sub>,</sub> 295<sub>,</sub> 297<sub>,</sub> 300<sub>,</sub> 302<sub>,</sub> 303, 305, 338–340 <sub>–</sub> f<sub>u</sub>ll<sub>y</sub> <sub>po</sub>l<sub>ynom</sub>i<sub>a</sub>l ti<sub>me</sub> (FPTAS) 68<sub>,</sub> 69–70, 72, 77, 83 <sub>– po</sub>l<sub>ynom</sub>i<sub>a</sub>l ti<sub>me</sub> (PTAS) 68<sub>,</sub> 80<sub>–</sub>89<sub>,</sub> 140, 145, 311, 336 <sub>– – asymp</sub>t<sub>o</sub>ti<sub>c</sub> 75<sub>,</sub> 74<sub>–</sub>78 Arborescence 228 Arithmetic-geometric mean inequality 135

<sub>co-</sub>NP 344 <sub>co-</sub>RP 10<sub>,</sub> 330<sub>,</sub> 348 Complementary slackness conditions 97<sub>,</sub> 100<sub>,</sub> 105<sub>,</sub> 125<sub>,</sub> 149<sub>,</sub> 161<sub>,</sub> 178<sub>,</sub> 199<sub>,</sub> 233 <sub>–</sub> <sub>re</sub>l<sub>axe</sub>d 126<sub>,</sub> 129<sub>,</sub> 130<sub>,</sub> 146<sub>,</sub> 149<sub>,</sub> 199<sub>,</sub> 234 Compression 64 Concave function 135 Convex combination 258, 259 Convex set 259 Cost-efectiveness of a set 16, 113 Counting problems VII, 294–305, 338 342 <sub>–</sub> #P<sub>-comp</sub>l<sub>e</sub>t<sub>e</sub> VII<sub>,</sub> 294<sub>,</sub> 294<sub>,</sub> 305<sub>,</sub> 338 Covering LP 109 C<sub>ross</sub>i<sub>ng se</sub>t<sub>s</sub> 215<sub>,</sub> 219 Cut packing 183–191 <sub>–</sub> <sub>approx</sub>i<sub>ma</sub>t<sub>e</sub> 184 Cut requirement function 213 Cycle space 54 – cyclomatic number 54 Cyclomatic weighted graphs 54–57 D<sub>ec</sub>i<sub>s</sub>i<sub>on</sub> <sub>pro</sub>bl<sub>em</sub> 343 <sub>–</sub> NP<sub>-comp</sub>l<sub>e</sub>t<sub>e</sub> 344 <sub>–</sub> <sub>we</sub>ll<sub>-c</sub>h<sub>arac</sub>t<sub>er</sub>i<sub>ze</sub>d 6<sub>,</sub> 5<sub>–</sub>7<sub>,</sub> 10<sub>,</sub> 93 <sub>–</sub> Yes/No certificate – – approximate 7 D<sub>e</sub>fi<sub>c</sub>i<sub>ency o</sub>f <sub>a se</sub>t 226 Demand graph 182 Derandomization 132–134, 138, 248–250, 268 D<sub>e</sub>t<sub>erm</sub>i<sub>nan</sub>t <sub>o</sub>f <sub>a</sub> l<sub>a</sub>tti<sub>ce</sub> 274 Dilworth’s theorem 8 Divide-and-conquer algorithm 179, 193

DTIME 331<sub>,</sub> 332<sub>,</sub> 348

Dual fitting 101, 108–118, 241

Dual growing

– synchronized 198

D<sub>ua</sub>l l<sub>a</sub>tti<sub>ce</sub> 284<sub>,</sub> 284<sub>–</sub>288

Dynamic programming 69, 81, 153

Edge expansion 192

Edge-disjoint s–t paths 103, 336

Eigenvalue 257

Eigenvector 257

Ellipsoid algorithm 170, 214, 255, 259

Euclid’s algorithm 273, 276–278

Euler tour 28, 32

E<sub>xpan</sub>d<sub>er</sub> <sub>grap</sub>h 175<sub>,</sub> 179<sub>,</sub> 192<sub>,</sub> 320<sub>,</sub> 332

Expander graphs 314

E<sub>x</sub>t<sub>reme</sub> <sub>po</sub>i<sub>n</sub>t <sub>so</sub>l<sub>u</sub>ti<sub>on</sub> 100<sub>,</sub> 102<sub>–</sub>104<sub>,</sub> 119<sub>,</sub> 122<sub>,</sub> 141<sub>–</sub>145<sub>,</sub> 214<sub>,</sub> 219<sub>–</sub>221

Eulerian graph 28, 31

First-fit algorithm 74, 77

Flow-equivalent tree 44

Forward delete 153

Frequency of an element 15, 119

– degree-weighted 17

– proper 208

Fundamental cycle 54

## Game

two-person zero-sum 106

Gauss’ algorithm 273, 276–278, 288

Gomory–Hu tree 40, 44, 46

Gram–Schmidt lower bound 287, 288

Gram–Schmidt orthogonalization 278<sub>,</sub> 278<sub>–</sub>280<sub>,</sub> 282<sub>,</sub> 285

Greedy algorithm 8, 16–17, 24, 44, 60, 64, 72, 108, 138, 241

Half-integrality 119, 122–124, 153, 160–163, 165, 213–221

Hall’s theorem 144

Hamiltonian cycle 29, 214

Hardness of approximation VIII, 306–333

Hungarian method 129

Kirchhof’s theorem 339

I<sub>n</sub>t<sub>egra</sub>lit<sub>y</sub> <sub>gap</sub> 102<sub>,</sub> 101<sub>–</sub>103<sub>,</sub> 111<sub>,</sub>129, 137, 151, 164, 167, 207, 210, 211,218, 229, 254, 262, 335, 337

Integrality ratio see Integrality gap

Isolating cut 38

Konig-Egervary theorem 5, 104

Interactive proof systems 332

Ising model 342

Kruskal’s algorithm 105, 206

Lagrangian relaxation 250–252

L<sub>am</sub>i<sub>nar</sub> f<sub>am</sub>il<sub>y o</sub>f <sub>se</sub>t<sub>s</sub> 219

Layering 17–19, 25, 57, 60, 129

Li<sub>near</sub>it<sub>y o</sub>f <sub>expec</sub>t<sub>a</sub>ti<sub>on</sub> 136<sub>,</sub> 352

Local search 23, 253

Lower bounding OPT 2, 17, 31, 32, 39, 47, 62, 79, 89, 108, 206, 278–280

Lowest common ancestor 149

## LP-duality

th<sub>eorem</sub> 6<sub>,</sub> 95<sub>,</sub> 93<sub>–</sub>97<sub>,</sub> 100<sub>,</sub> 106<sub>,</sub> 107<sub>,</sub> 148, 183

<sub>wea</sub>k 96<sub>,</sub> 148<sub>,</sub> 169

– theory 6, 17, 29, 97, 101, 108, 147

Mader’s theorem 227, 231

Markov chain 192, 338, 339

– conductance 192–193, 197

– Markov chain Monte Carlo method VIII, 294

– rapidly mixing 305, 339, 340

– stationary probability distribution 192

– Swendsen-Wang process 342 – transition matrix 192

M<sub>ar</sub>k<sub>ov</sub>’<sub>s</sub> i<sub>nequa</sub>lit<sub>y</sub> 88<sub>,</sub> 353

Matroid 339

– balanced 339

– basis exchange graph 339

– graphic 339

– independent sets 212

Max-flow min-cut theorem 97, 103, 168 207 – approximate version for demands multicommodity flow 191 – approximate version for uniform multicommodity flow 197

MAX-SNP-completeness 332

Maximum weight spanning tree 44

Primal–dual schema VII, 101,

Menger’s theorem 103

125–130, 149–152, 235–236, 335

Method of conditional expectation 131–134, 138, 139, 248

– with synchronization 199–204

Primitive root 9

Metric 183–191

P<sub>r</sub>i<sub>m</sub>iti<sub>ve vec</sub>t<sub>or</sub> 275<sub>,</sub> 285<sub>,</sub> 286<sub>,</sub> 290

– 1-embedding 183–191

Principal submatrix 265

<sub>– –</sub> β<sub>-</sub>di<sub>s</sub>t<sub>or</sub>ti<sub>on</sub> 185

Probabilistic argument 179

<sub>–</sub> <sub>–</sub> i<sub>some</sub>t<sub>r</sub>i<sub>c</sub> 185<sub>,</sub> 186

Probabilistic method 324

2-embedding 196

Probabilistically checkable proof system

– <sup>2</sup><sub>2</sub>-embedding

– – isometric 195

completeness 319

– – optimal distortion 197, 266

– parallel repetition 325–326

<sub>–</sub> <sub>p</sub> 185

Min–max relation 5–7, 11, 97–100, 168

– two-prover one round 322–324, 332

– approximate 7, 151

Minkovski’s theorem 287

M<sub>omen</sub>t<sub>s o</sub>f <sub>a ran</sub>d<sub>om var</sub>i<sub>a</sub>bl<sub>e</sub> 352 <sub>– cen</sub>t<sub>ra</sub>l 352

<sub>– norma</sub>l 261<sub>,</sub> 266<sub>,</sub> 354

Monte Carlo sampling 297, 301

– spherically symmetric 261

Probability theory 352–354

Near-minimum cuts 298–299

Pseudo-approximation algorithm 193–195, 197

Next-fit algorithm 77

N<sub>orm</sub> 185

P<sub>seu</sub>d<sub>o-</sub>f<sub>ores</sub>t 143

Pseudo-polynomial time algorithm 69<sub>,</sub> 69<sub>,</sub> 71<sub>–</sub>73

NP 343

P<sub>seu</sub>d<sub>o-</sub>t<sub>ree</sub> 143

Odd <sub>se</sub>t <sub>cover</sub> 6

O<sub>p</sub>ti<sub>m</sub>i<sub>za</sub>ti<sub>on</sub> <sub>pro</sub>bl<sub>em</sub> 2<sub>,</sub> 345<sub>,</sub> 351

Quadratic forms 292

Random contraction algorithm 298, 304

Random walk 320, 338–340

P=NP conjecture VII, 10, 68, 71, 345

Parametric pruning 47–52, 140–141, 252

Parsimonious property 229, 230

<sub>–</sub> <sub>gap-</sub>i<sub>n</sub>t<sub>ro</sub>d<sub>uc</sub>i<sub>ng</sub> 307

Partial ordering 8

<sub>– gap-preserv</sub>i<sub>ng</sub> 307

PCP theorem VIII, 306, 308–311, 323, 332

P<sub>e</sub>t<sub>ersen</sub> <sub>grap</sub>h 6<sub>,</sub> 214

Region growing 171–175

Poisson trials 353

P<sub>os</sub>iti<sub>ve sem</sub>id<sub>e</sub>fi<sub>n</sub>it<sub>e ma</sub>t<sub>r</sub>i<sub>x</sub> 257<sub>,</sub> 257–258

Potts model 342

Prefix graph 62

– – for maximum weight bipartite matching 129

– – for MST 212, 230 <sub>–</sub> LP<sub>-</sub> VII<sub>,</sub> 39<sub>,</sub> 99<sub>,</sub> 100<sub>–</sub>106<sub>,</sub> 109<sub>,</sub> 111<sub>,</sub> 113, 119, 120, 122, 124, 125, 134, 147, 153, 155–157, 160, 164, 165, 179, 199, 206, 209, 211, 213–221, 224, 229–231, 233, 240, 244, 251, 335, 337 – – bidirected cut relaxation for Steiner tree 210, 335 – subtour elimination relaxation for TSP 229<sub>,</sub> 229<sub>–</sub>231

Reverse delete 149, 210

– dynamic 209

Rounding VII, 101, 119–124, 134–136, 170–175, 191

– iterated 213, 217–218 – randomized 120–122, 124, 157–160, 164, 247–248, 260–263

RP 348

Scaling and rounding 73, 117

Self-reducibility IX, 9, 10, 303, 348–351 – tree 10, 303, 349

Semidefinite program 197, 266, 267 – duality theory 268

S<sub>epara</sub>ti<sub>ng</sub> h<sub>yperp</sub>l<sub>ane</sub> 259

S<sub>epara</sub>ti<sub>on</sub> <sub>orac</sub>l<sub>e</sub> 102<sub>,</sub> 107<sub>,</sub> 170<sub>,</sub> 179<sub>,</sub> 217

Short-cutting 29, 31, 32, 85, 241

Simplex 155

S<sub>pars</sub>it<sub>y o</sub>f <sub>a cu</sub>t 181

Spread of an edge 196

S<sub>quare o</sub>f <sub>a grap</sub>h 48

St<sub>an</sub>d<sub>ar</sub>d d<sub>ev</sub>i<sub>a</sub>ti<sub>on</sub> 352

Steiner tree 316–318

S bl tti 285 290

S<sub>u</sub>b<sub>mo</sub>d<sub>u</sub>l<sub>ar</sub> f<sub>unc</sub>ti<sub>on</sub> 215<sub>,</sub> 224

Supermodular function kl d l 216

Th<sub>roug</sub>h<sub>pu</sub>t 180<sub>,</sub> 182 Ti<sub>g</sub>ht <sub>examp</sub>l<sub>e</sub> IX<sub>,</sub> 4<sub>,</sub> 8<sub>,</sub> 17<sub>,</sub> 19<sub>,</sub> 23<sub>–</sub>25<sub>,</sub> 29, 31, 33, 39, 43, 49, 51, 59, 80, 83, 120, 123, 128, 137, 144, 153, 165, 175, 206, 218, 238, 239, 249, 268 Totally unimodular matrix 104 T<sub>ournamen</sub>t 25 Traveling salesman tour – maximum weight 66 – minimum weight 62 T<sub>r</sub>i<sub>ang</sub>l<sub>e</sub> i<sub>nequa</sub>lit<sub>y</sub> 27<sub>,</sub> 51<sub>,</sub> 52<sub>,</sub> 178 <sub>–</sub> di<sub>rec</sub>t<sub>e</sub>d 34

Unbiased estimator 295 Uncrossable function 224 Uniform generator 302, 303 – almost uniform 303 U<sub>n</sub>i<sub>mo</sub>d<sub>u</sub>l<sub>ar ma</sub>t<sub>r</sub>i<sub>x</sub> 274<sub>,</sub> 274<sub>–</sub>276<sub>,</sub> 288 Unit sphere 260 Upper bounding OPT 256

V<sub>ec</sub>t<sub>or</sub> <sub>program</sub> 256<sub>,</sub> 255<sub>–</sub>257<sub>,</sub> 266<sub>,</sub> 267 Verifier 309 Vertex cover 316–318 Vertex-disjoint s–t paths 103, 336 VLSI design 178 – clock routing 36 von Neumann’s minimax theorem 106

Witness family 225

ZPP 10<sub>,</sub> 348

ZTIME 329<sub>,</sub> 332<sub>,</sub> 348
