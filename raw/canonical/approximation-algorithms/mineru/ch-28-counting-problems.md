---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-28"
chapter_number: 28
chapter_title: "Counting Problems"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 312
source_page_end: 323
printed_page_start: 294
printed_page_end: 305
part_ids: ["approximation-algorithms-ch-28-part-029"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Counting Problems (MinerU semantic layer)

<!-- source-pages: 312-323; printed-pages: 294-305; mineru-part: approximation-algorithms-ch-28-part-029 -->

## 28 Counting Problems

The techniques for approximately counting the number of solutions to #Pcomplete problems are quite diferent from those for obtaining approximation algorithms for NP-hard optimization problems. Much of the former theory is built around the Markov chain Monte Carlo method, see Section 28.4 for references. In this chapter, we will present combinatorial algorithms (not using Markov chains) for two fundamental problems, counting the number of satisfying truth assignments for a DNF formula, and estimating the failure probability of an undirected network.

Intuitively, the class $\# \mathbf { P }$ captures the problems of counting the number of solutions to NP problems. Let us formalize this notion. Let $L$ be a language in NP, M be its associated verifier, and polynomial $p$ be the bound on the length of its Yes certificates (see Section $\mathrm { A . 1 } )$ . For string $x \in \Sigma ^ { * }$ , define $f ( x )$ to be the number of strings y such that $| y | \leq p ( | x | )$ and $M ( x , y )$ accepts. Functions $f : \Sigma ^ { * } \to \mathbf { Z } ^ { + }$ that arise in this manner constitute the class $\# \mathbf { P }$

Function $f \in \# \mathbf { P }$ is said to be #P-complete if every function $g \in \# \mathbf { P }$ can be reduced to $f$ in the following sense. There is a polynomial time transducer $R : \Sigma ^ { * } \to \Sigma ^ { * }$ , that, given an instance, $x ,$ of $^ { g , }$ produces an instance, $R ( x )$ of $f .$ Furthermore, there is a polynomial time computable function $S : \Sigma ^ { * } \times$ $\mathbf { Z } ^ { + } \to \mathbf { Z } ^ { + }$ that $\mathrm { g i }$ ven x and $f ( R ( x ) )$ computes $g ( x ) , \mathrm { i . e . }$ 2

$$
\forall x \in \Sigma^ {*}, g (x) = S (x, f (R (x))).
$$

In other words, an oracle for $f$ can be used to compute $g$ in polynomial time. The solution counting versions of all known NP-complete problems are #P-complete.<sup>1</sup> Interestingly enough, other than a handful of exceptions, this is true of problems in P as well. This raises the question of designing polynomial time algorithms for approximately counting the number of solutions to these latter problems (see Exercise 28.3 regarding the question of approximately counting the number of solutions to NP-complete problems). These problems admit only two interesting possibilities: they either allow approximability to any required degree, or essentially not at all (see Section 28.4).

The former possibility is captured in the definition of a fully polynomial randomized approximation scheme, abbreviated $\mathrm { F P R A S }$

Consider a problem in P whose counting version, $f ,$ is $\# \mathbf { P } \cdot$ -complete. An algorithm A is an FPRAS for this problem if for each instance $x \in \Sigma ^ { * }$ , and error parameter $\varepsilon > 0$

$$
\mathbf {P r} [ | \mathcal {A} (x) - f (x) | \leq \varepsilon f (x) ] \geq \frac {3}{4},
$$

and the running time of $\mathcal { A }$ is polynomial in $| x |$ and $1 / \varepsilon$ . (See Exercise 28.1 for a method for reducing the error probability of an FPRAS.)

## 28.1 Counting DNF solutions

Problem 28.1 (Counting DNF solutions) Let $f = C _ { 1 } \lor C _ { 2 } \lor \dots \lor C _ { m }$ be a formula in disjunctive normal form on n Boolean variables $x _ { 1 } , \ldots , x _ { n } .$ Each clause $C _ { i }$ is of the form $C _ { i } = l _ { 1 } \wedge l _ { 2 } \wedge . . . \wedge l _ { r _ { i } }$ , where each $l _ { j }$ is a literal, i.e., it is either a Boolean variable or its negation. We may assume w.l.o.g. that each clause is satisfiable, i.e., does not contain a variable and its negation. The problem is to compute $\# \mathrm { f } .$ , the number of satisfying truth assignments of $f .$

The main idea is to define an eficiently samplable random variable X which is an unbiased estimator for $\# \mathrm { f } .$ i.e., $\mathbf { E } [ X ] = \# \mathrm { f }$ . If in addition, the standard deviation of X is within a polynomial factor of $\mathbf { E } [ X ]$ , then an FPRAS for #f can be obtained in a straightforward manner by sampling $X$ a polynomial number of times (in n and $1 / \varepsilon )$ and outputting the mean.

Constructing an unbiased estimator for #f is easy. Let random variable $Y$ have uniform distribution on all $2 ^ { n }$ truth assignments, and let $Y ( \tau )$ be $2 ^ { n } { \mathrm { ~ i f ~ } } \tau$ satisfies $f ,$ and 0 otherwise (see Exercise 28.4). However, this random variable can have a very large standard deviation, and does not yield an FPRAS. For instance, suppose $f$ has only polynomially many satisfying truth assignments. Then, with high probability, a polynomial number of randomly picked truth assignments will all have $Y = 0$ , giving a poor estimate for $\# \mathrm { f . }$

We will rectify this by defining a random variable that assigns nonzero probability to only the satisfying truth assignments of f. Let $S _ { i }$ denote the set of truth assignments to $x _ { 1 } , \ldots , x _ { n }$ that satisfy clause $C _ { i }$ . Clearly, $| S _ { i } | = 2 ^ { n - r _ { i } }$ 2 where $r _ { i }$ is the number of literals in clause $C _ { i }$ . Also, $\# \mathrm { f } = | \cup _ { i = 1 } ^ { m } S _ { i } |$ . Let $c ( \tau )$ denote the number of clauses that truth assignment τ satisfies. Let M denote the multiset union of the sets $S _ { i } , \mathrm { i . e . }$ , it contains each satisfying truth assignment, $\tau , c ( \tau )$ number of times. Notice that $\begin{array} { r } { | M | = \sum _ { i } | S _ { i } | } \end{array}$ is easy to compute.

Pick a satisfying truth assignment, $\tau ,$ for $f$ with probability $c ( \tau ) / | M |$ • and define $X ( \tau ) = | M | / c ( \tau )$ . We will first show that $X$ can be eficiently sampled, i.e., using a randomized polynomial time algorithm.

Lemma 28.2 Random variable X can be eficiently sampled.

Proof: Picking a random element from the multiset M ensures that each truth assignment is picked with the desired probability. The following twostep process will accomplish this. First pick a clause so that the probability of picking clause $C _ { i }$ is $| S _ { i } | / | M |$ . Next, among the truth assignments satisfying the picked clause, pick one at random.

Now, the probability with which truth assignment $\tau$ is picked is

$$
\sum_ {i: \tau \text {satisfies} C _ {i}} \frac {| S _ {i} |}{| M |} \times \frac {1}{| S _ {i} |} = \frac {c (\tau)}{| M |}.
$$

Lemma 28.3 X is an unbiased estimator for $\# f .$

Proof:

$$
\mathbf {E} [ X ] = \sum_ {\tau} \mathbf {P r} [ \tau \text {is picked} ] \cdot X (\tau) = \sum_ {\tau \text {satisfies} f} \frac {c (\tau)}{| M |} \times \frac {| M |}{c (\tau)} = \# \mathrm{f}.
$$

X takes values only in a “polynomial range”, thereby ensuring that its standard deviation is not large compared to its expectation. This fact is proved in the next lemma, and leads to the FPRAS construction.

Lemma 28.4 $I f$ m denotes the number of clauses in $f ,$ then

$$
\frac {\sigma (X)}{\mathbf {E} [ X ]} \leq m - 1.
$$

Proof: Denote $| M | / m$ by α. Clearly, $\mathbf { E } [ X ] \geq \alpha$ . For each satisfying truth assignment τ of $f , 1 \leq c ( \tau ) \leq m$ . Therefore, $X ( \tau )$ lies in the range $[ \alpha , m \alpha ]$ and so the random variable deviates from its mean by at most $( m - 1 ) \alpha ,$ • i.e., $| X ( \tau ) - \mathbf { E } [ X ] | \leq ( m - 1 ) \alpha$ . Therefore, the standard deviation of $X$ is bounded by $( m - 1 ) \alpha$ . Using the lower bound on $\mathbf { E } [ X ]$ stated above, we get the lemma. ✷

Finally, we will show that sampling X polynomially many times (in n and $1 / \varepsilon )$ and simply outputting the mean leads to an FPRAS for #f. Let $X _ { k }$ denote the mean of k samples of $X$

Lemma 28.5 For any $\varepsilon > 0$

$$
\mathbf {P r} [ | X _ {k} - \# f | \leq \varepsilon \# f ] \geq 3 / 4,
$$

where $k = 4 ( m - 1 ) ^ { 2 } / \varepsilon ^ { 2 }$

Proof: We will use Chebyshev’s inequality (see Section B.2), with $a =$ $\varepsilon { \bf E } [ X _ { k } ]$ . Using the value of k stated above we get

$$
\mathbf {P r} [ | X _ {k} - \mathbf {E} [ X _ {k} ] | \geq \varepsilon \mathbf {E} [ X _ {k} ] ] \leq \left(\frac {\sigma (X _ {k})}{\varepsilon \mathbf {E} [ X _ {k} ]}\right) ^ {2} = \left(\frac {\sigma (X)}{\varepsilon \sqrt {k} \mathbf {E} [ X ]}\right) ^ {2} \leq \frac {1}{4},
$$

where the equality follows by noting that ${ \bf E } [ X _ { k } ] \ = \ { \bf E } [ X ]$ and $\sigma ( X _ { k } ) \ =$ $\sigma ( X ) / \sqrt { k }$ , and the last inequality follows by applying Lemma 28.4. The lemma follows. ✷

Theorem 28.6 There is an FPRAS for the problem of counting DNF solutions.

## 28.2 Network reliability

Problem 28.7 (Network reliability) Given a connected, undirected graph $G = ( V , E )$ , with failure probability $p _ { e }$ specified for each edge e, compute the probability that the graph becomes disconnected.

Graph G will become disconnected if all edges in some cut $( C , { \overline { { C } } } ) , C \subset V$ fail. We will present an FPRAS for this problem.

Let us first handle the case that each edge has the same failure probability, denoted by p. However, we will allow $G$ to have parallel edges between any two vertices. Denote by $\operatorname { F A I L } ( p )$ the probability that $G$ gets disconnected. If $\operatorname { F A I L } ( p )$ is at least inverse polynomial, then it can be eficiently estimated by Monte Carlo sampling (see proof of Theorem 28.11 for details). Let us handle the dificult case that $\operatorname { F A I L } ( p )$ is small. Assume that FAIL $\dot { ( p ) } \leq n ^ { - 4 }$ . The reason for this choice will become clear below.

The probability that cut $( C , { \overline { { C } } } )$ gets disconnected is simply $p ^ { c }$ where c is the number of edges crossing this cut. Since the failure probability of a cut decreases exponentially with capacity, the most important cuts for the purpose of estimating $\operatorname { F A I L } ( p )$ are cuts with “small” capacity. The algorithm is built around two ideas:

1. For any $\varepsilon > 0 .$ , we will show that only polynomially many “small” cuts (in n and $1 / \varepsilon )$ are responsible for $1 - \varepsilon$ fraction of the total failure probability $\operatorname { F A I L } ( p )$ . Moreover, these cuts, say $E _ { 1 } , \ldots E _ { k } , E _ { i } \subseteq E$ , can be enumerated in polynomial time.

2. We will construct a polynomial sized DNF formula $f$ whose probability of being satisfied is precisely the probability that at least one of these cuts fails.

As a result of the first idea, it is suficient to estimate the probability that one of the cuts $E _ { 1 } , \ldots E _ { k }$ fails. However, because of correlations, this is nontrivial. The second idea reduces this problem to counting DNF solutions, for which we have an FPRAS.

Formula $f$ has a Boolean variable $x _ { e }$ for each edge $e , x _ { e }$ is set to true with probability $p ,$ the failure probability of edge e. Suppose cut $E _ { i } = \{ e _ { 1 } , . . . , e _ { j } \}$ Construct the clause $D _ { i } = x _ { e _ { 1 } } \wedge \cdot \cdot \cdot \wedge x _ { e _ { i } } ,$ i.e., the conjunct of all variables corresponding to edges in this cut. The probability that this clause is satisfied is precisely the failure probability of cut $E _ { i }$ . Finally, $f = D _ { 1 } \lor \cdots \lor D _ { k }$ , i.e., the disjunct of clauses corresponding to cuts.

## 28.2.1 Upperbounding the number of near-minimum cuts

The first idea has its roots in the fact that one can place upper bounds on the number of minimum and near-minimum capacity cuts in an undirected graph. Let c be the capacity of a minimum cut in G. Recall that all edges in $G$ are assumed to be of unit capacity, and that G is allowed to have parallel edges between any two vertices.

Lemma 28.8 The number of minimum cuts in $G = ( V , E )$ is bounded $b y$ $n ( n - 1 ) / 2$

Proof: By contracting an edge $( u , v )$ in a graph we mean merging the vertices u and v into a single vertex. All edges running between u and v are discarded. Those running between u or v and some other vertex w will now run between the merged vertex and $w ,$ , their number being conserved.

Now consider the following random contraction process. Iteratively, pick a random edge $( u , v )$ in the current graph and contract it. Terminate when exactly two vertices are left. Suppose these two vertices correspond to sets S and $V - S , S \subset V$ , of vertices of the starting graph G. Then, the algorithm outputs the cut $( S , { \overline { { S } } } )$ . We will say that this cut survives. Clearly, a cut survives if none of its edges is contracted during the algorithm.

Let $( C , { \overline { { C } } } )$ be any minimum cut in G. We will show

$$
\mathbf {P r} [ (C, \overline {{C}}) \text { survives } ] \geq \frac {1}{\binom {n} {2}}.
$$

This statement yields the lemma via an interesting argument. Let M be the number of minimum cuts in $G$ . The survival of each of these cuts is a mutually exclusive event, and the total probability of these events adds up to at most 1. Hence $M / ( n ( n - 1 ) / 2 ) \leq 1$ , thereby giving the desired bound.

Consider an arbitrary iteration in the random contraction process, and let H be a graph at the beginning of this iteration. Since the process of contraction cannot decrease the capacity of the minimum cut, the capacity of each cut in H is at least $c .$ This holds for cuts separating one vertex of H from the rest. Therefore, the degree of each vertex in H must be at least c. Hence, H must have at least $c m / 2$ edges, where m is the number of vertices in $H .$

Now, the conditional probability that cut $( C , { \overline { { C } } } )$ survives the current iteration, given that it has survived so far, is at least $\begin{array} { r } { \left( 1 - \frac { c } { c m / 2 } \right) = \left( 1 - 2 / m \right) } \end{array}$ (this is simply the probability that the randomly chosen edge in this iteration is not picked from the cut $( C , { \overline { { C } } } ) )$ . The probability that $( C , { \overline { { C } } } )$ survives the whole algorithm is simply the product of these conditional probabilities. This gives

$$
\mathbf {P r} [ (C, \overline {{C}}) \text {   survives } ] \geq \left(1 - \frac {2}{n}\right) \left(1 - \frac {2}{n - 1}\right) \ldots \left(1 - \frac {2}{3}\right) = \frac {1}{\binom {n} {2}}.
$$

For $\alpha \geq 1$ , we will say that a cut is an α-min cut if its capacity is at most αc.

Lemma 28.9 For any $\alpha \geq 1$ , the number of α-min cuts in G is at most $n ^ { 2 \alpha }$

Proof: We will prove the lemma for the case that α is a half-integer. The proof for arbitrary α follows by applying the same ideas to generalized binomial coeficients. Let $2 \alpha = k$

Consider the following two-phase process: First, run the random contraction algorithm until there are k vertices remaining in the graph. Next, pick a random cut among all $2 ^ { k - 1 }$ cuts in this graph. This will define a cut in the original graph.

Let $( C , { \overline { { C } } } )$ be any α-min cut in G. We will show that the probability that it survives the two phase process is at least $1 / n ^ { 2 \alpha }$ , thereby proving the desired bound.

Let H be the graph at the beginning of an arbitrary iteration in the first phase. As argued in Lemma 28.8, if H has m vertices, it has at least $m c / 2$ edges. Therefore, the conditional probability that $( C , { \overline { { C } } } )$ survives the current iteration, given that it has survived so far, is at least $\begin{array} { r } { 1 - \frac { \alpha c } { m c / 2 } = 1 - 2 \alpha / m . } \end{array}$ The probability that $( C , { \overline { { C } } } )$ survives the first phase is at least

$$
\left(1 - \frac {2 \alpha}{n}\right) \left(1 - \frac {2 \alpha}{n - 1}\right) \ldots \left(1 - \frac {2 \alpha}{3}\right) = \frac {1}{\binom {n} {k}}.
$$

The conditional probability that $( C , { \overline { { C } } } )$ survives the second phase, given that it has survived the first, is $1 / 2 ^ { k - 1 }$ . Therefore,

$$
\mathbf {P r} [ (C, \overline {{C}}) \text { survives   both   phases } ] \geq \frac {1}{\binom {n} {k} 2 ^ {k - 1}} \geq \frac {1}{n ^ {k}} = \frac {1}{n ^ {2 \alpha}}.
$$

## 28.2.2 Analysis

Recall that we are considering the case that $\mathrm { F A I L } ( p ) \leq n ^ { - 4 }$ . We can now justify this choice. The failure probability of a minimum cut is $p ^ { c } \leq \mathrm { F A I L } ( p ) \leq$ $n ^ { - 4 }$ . Let $p ^ { c } = n ^ { - ( 2 + \delta ) }$ , where $\delta \geq 2$ . Now, by Lemma 28.9, for any $\alpha \geq 1$ , the total failure probability of all cuts of capacity αc is at most $p ^ { \alpha c } n ^ { 2 \alpha } = n ^ { - \alpha \delta }$ This rapid decrease in the total failure probability of all cuts of capacity αc will enable us to bound the total failure probability of “large” capacity cuts.

Lemma 28.10 For any $\alpha \geq 1$

$$
\mathbf {P r} [ \text { some   cut   of   capacity } > \alpha c \text { fails } ] \leq n ^ {- \alpha \delta} \left(1 + \frac {2}{\delta}\right).
$$

Proof: Number all cuts in G by increasing capacity. Let $c _ { k }$ and $p _ { k }$ denote the capacity and failure probability of the kth cut in this numbering. Let a be the number of the first cut of capacity greater than αc. It sufices to show that

$$
\sum_ {k \geq a} p _ {k} \leq n ^ {- \alpha \delta} \left(1 + \frac {2}{\delta}\right).
$$

We will evaluate this sum in two steps. First, we will consider the first $n ^ { 2 \alpha }$ terms. Each of these terms is at most $p ^ { \alpha c } = n ^ { - \alpha ( 2 + \delta ) }$ . Therefore, their sum is at most $n ^ { - \alpha \delta }$

Next, let us bound the sum of the remaining terms. Clearly, this sum is bounded by $\textstyle \sum _ { k > n ^ { 2 \alpha } } p _ { k }$ . By Lemma 28.9, there are at most $n ^ { 2 \alpha }$ cuts of capacity bounded by αc. Therefore, $c _ { n ^ { 2 \beta } } ~ \geq ~ \beta c$ . Writing $ { k } = n ^ { 2 \beta }$ we get $c _ { k } \geq$ c ln $k / 2$ ln $n ,$ and

$$
p ^ {k} \leq (p ^ {c}) ^ {\frac {\ln k}{2 \ln n}} = k ^ {- (1 + \delta / 2)}.
$$

Therefore,

$$
\sum_ {k > n ^ {2 \alpha}} p _ {k} \leq \int_ {n ^ {2 \alpha}} ^ {\infty} k ^ {- (1 + \delta / 2)} \mathrm{d} k \leq \frac {2}{\delta} n ^ {- \alpha \delta}.
$$

This proves the lemma.

Theorem 28.11 There is an FPRAS for estimating network reliability.

Proof: We will first consider the case that each edge in graph G has the same failure probability, $p .$

If $\mathrm { F A I L } ( p ) > n ^ { - 4 }$ , then we will resort to Monte Carlo sampling. Flip a coin with bias p for failure of each edge, and check if G is disconnected. Repeat this experiment $O ( \log n / ( \varepsilon ^ { 2 } \mathrm { F A I L } ( p ) ) )$ times, and output the mean number of times G is disconnected. A straightforward application of Chernof bounds shows that the mean lies in $[ ( 1 - \varepsilon ) \mathrm { F A I L } ( p ) , ( 1 + \varepsilon ) \mathrm { F A I L } ( p ) ]$ with high probability.

Next, assume that $\mathrm { F A I L } ( p ) ~ \leq ~ n ^ { - 4 }$ . Now, for any $\varepsilon \ > \ 0 .$ , we want to determine α such that the total failure probability of all cuts of capacity $>$ αc is bounded by $\varepsilon \mathrm { F A I L } ( p )$ . By Lemma 28.10, it sufices to find α such that

$$
n ^ {- \alpha \delta} \left(1 + \frac {2}{\delta}\right) \leq \varepsilon \text { FAIL } (p) \leq \varepsilon n ^ {- (2 + \delta)}.
$$

Solving, we get

$$
\alpha = 1 + \frac {2}{\delta} - \frac {\ln (\varepsilon / 2)}{\delta \ln n} \leq 2 - \frac {\ln (\varepsilon / 2)}{2 \ln n}.
$$

By Lemma 28.9, $c _ { n ^ { 2 \alpha } } > \alpha c .$ . For the value of α computed,

$$
\mathbf {P r} [ \text { one   of   the   first } n ^ {2 \alpha} \text { fails } ] \geq (1 - \varepsilon) \text { FAIL } (p).
$$

The first $n ^ { 2 \alpha } = O ( n ^ { 4 } / \varepsilon )$ cuts can be enumerated in polynomial time (see Exercises). We will use these to construct the corresponding DNF formula, and estimate the probability that it is satisfiable, as described above.

Finally, we show how to “reduce” the case of arbitrary edge failure probabilities to the simpler case analyzed above. Suppose edge e has failure probability $p _ { e }$ . Choose a small parameter θ. Replace edge e with $k _ { e } = - ( \ln p _ { e } ) / \theta$ parallel edges each with failure probability $1 - \theta$ . Then, the probability that all $k _ { e }$ edges fail is

$$
(1 - \theta) ^ {- (\ln p _ {e}) / \theta}.
$$

As $\theta  0 .$ this failure probability converges to $p _ { e }$ . Let H be the graph obtained by doing this transformation on each edge of $G .$ In the limit as $\theta  0$ , each cut in H has the same failure probability as that in $G$

Let us give an eficient implementation of this idea. All we really want is a listing of the “small” capacity cuts in G. Once this is done, we can apply the more general DNF counting algorithm developed in Exercise 28.5, where each variable is set to true with its own probability $p _ { e }$ . Observe that changing θ scales the capacities of cuts in H without changing their relative values. Thus, it sufices to assign a weight $\mathrm { o f } \mathrm { ~ - ~ } \ln p _ { e }$ to each edge e of $G ,$ and find “small” capacity cuts in this graph. This completes the proof. ✷

## 28.3 Exercises

28.1 Given an FPRAS for a problem, show that its success probability can be improved to $1 - \delta$ , for any $\delta > 0$ , by a multiplicative increase in the running time of only ${ \cal O } ( \log ( 1 / \delta ) )$

Hint: Run the FPRAS $O ( \log ( 1 / \delta ) )$ times and output the median value.

28.2 Suppose we make the definition of an FPRAS more stringent by requiring it to have a fixed additive error α with high probability, i.e.,

$$
\mathbf {P r} [ f (x) - \alpha \leq A (x) \leq f (x) + \alpha ] \geq \frac {3}{4}.
$$

Show that if there were such an algorithm for a $\# \mathbf { P } \cdot$ -complete problem, then $\mathbf { P } { = } \mathbf { N P }$

28.3 Show that if there were an FPRAS for counting the number of satisfying truth assignments to SAT then every problem in NP could be solved in random polynomial time. How weak an approximate counting algorithm for SAT sufices to give this consequence? What does this say for the question of approximately counting the number of solutions to other NP-complete problems?

Hint: Use solution amplification. Given SAT formula $f ,$ define formula $f ^ { \prime }$ over $k$ new Boolean variables which is a tautology. Then the number of solutions of $\phi = f \wedge f ^ { \prime }$ is $\# \mathrm { f } \cdot 2 ^ { k }$

28.4 Given a DNF formula $f ,$ let $Y$ be a random variable that on a random truth assignment $\tau$ is $2 ^ { n }$ if $\tau$ satisfies $f$ and 0 otherwise. Show that $Y$ is an unbiased estimator for #f. How large can the ratio σ $\mathbf { \partial } \cdot ( Y ) / \mathbf { E } [ Y ]$ be?

28.5 (Karp and Luby [165]) You are given a DNF formula $f$ on n Boolean variables, $x _ { 1 } , \ldots , x _ { n }$ , and probabilities $p _ { 1 } , \ldots , p _ { n }$ with which these variables are (independently) set to true. Let $D$ denote the resulting probability distribution over the $2 ^ { n }$ truth assignments to the Boolean variables, and $p$ denote the probability that $f$ is satisfied by a truth assignment picked from $D$ . Construct an FPRAS for estimating $p _ { \cdot }$

Hint: Let $q _ { i }$ denote the probability that clause $C _ { i }$ is satisfied by a truth assignment picked from $D _ { : }$ , and $Q \ = \ \textstyle \sum _ { i } q _ { i }$ . Now, consider random vari-able X that assigns to each satisfying truth assignment $\tau \ \mathrm { a }$ probability of ${ \bf P r } _ { D } [ \tau ] c ( \tau ) / Q$ , and define $X ( \tau ) = Q / c ( \tau )$

28.6 A uniform generator for an NP problem Π is a randomized polynomial time algorithm $\mathcal { A }$ that given an instance I of a problem, outputs either a solution to $I ,$ or else the special symbol $^ { 6 6 } \perp ^ { 5 }$ , such that

• each solution to I is output with the same probability, i.e., there is a number $\alpha \in ( 0 , 1 ]$ such that

Pr[A outputs $s ] = \alpha$ , for each solution s of I, and

• the probability of outputting ⊥, i.e., failing to output a solution, is $< 1 / 2$

Give a uniform generator for picking a random satisfying truth assignment to a given DNF formula.

Hint: The essential idea behind the construction of random variable X works.

28.7 (Jerrum, Valiant, and Vazirani [147]) Let Π be an NP problem that is self-reducible (see Section A.5 and Exercise 1.15). Show that there is an FPRAS for Π if there is an almost uniform generator for it. An almost uniform generator for Π is a randomized polynomial time algorithm A such that for any $\mu > 0$ and instance I of $I I ,$ there is a number $\alpha \in ( 0 , 1 ]$ such that

• for each solution s of I, $\mathbf { P r } | \mathcal { A }$ outputs $s ] \in [ ( 1 - \mu ) \alpha , ( 1 + \mu ) \alpha ]$

$\mathbf { P r } [ \mathcal { A }$ fails to output a solution] $< 1 / 2$ , and

• the running time of A is polynomial in |I| and $\log ( 1 / \mu )$

Observe that unlike an FPRAS, which can only achieve inverse polynomial error, a uniform generator can achieve inverse exponential error (µ), in polynomial time.

Hint: For the forward direction, first construct a uniform generator, assuming that the FPRAS makes no error. (Traverse down the self-reducibility tree for I, with biases determined by estimates on the number of solutions. Accept leaf with appropriate probability, to achieve uniform generation.) Use the fact that the error probability of the FPRAS can be made exponentially small to obtain an almost uniform generator. For the reverse direction, obtain instance $I _ { \alpha }$ , with $| I _ { \alpha } | < | I |$ , and a good estimate of the ratio of the number of solutions to I and $I _ { \alpha }$

28.8 (Jerrum, Valiant, and Vazirani [147]) This exercise leads to strong evidence that the problem of estimating the number of simple cycles in a directed graph is essentially not approximable. Show that if there is an almost uniform generator for this problem, then there is a randomized polynomia time algorithm for deciding if a given directed graph has a Hamiltonian cycle.

Hint: Obtain a graph $G ^ { \prime }$ from G that amplifies the number of cycles of each length. However, it amplifies bigger cycles more than it amplifies smaller cycles, so that most cycles in $G ^ { \prime }$ are of maximum length and correspond to the largest cycles in $G .$

28.9 Show that the random contraction algorithm of Lemma 28.8 can be used to obtain a randomized algorithm for finding a minimum cut in an undirected graph.

28.10 (Karger and Stein [159]) Obtain a randomized algorithm for enumerating all α-min cuts in G using the random contraction algorithm and Lemma 28.9.

In the next three exercises (from Vazirani and Yannakakis [252]), we will develop a deterministic algorithm for enumerating in an undirected graph by increasing weight, with polynomial delay, i.e., the algorithm spends polynomial time between successive outputs.

Assume that graph $G = ( V , E )$ has n vertices besides s and $t ,$ numbered 1 to n. Every s–t cut in $G$ can be represented as an n bit $0 / 1$ vector. A partially specified cut, in which the sides of only vertices numbered 1 to k are decided, is represented as a k bit $0 / 1$ vector. Consider a binary tree $T$ of height n. Its leaves represent $^ { s - t }$ cuts and internal nodes represent partially specified cuts. All cuts consistent with a partially specified cut lie in the subtree rooted at it. Clearly, a minimum weight cut in this subtree can be computed with one max-flow computation.

28.11 Let a be an $n - k$ bit $0 / 1$ vector representing a partially specified cut, as well as an internal node in $T .$ . The subtree, $T ^ { \prime } { \mathrm { . } }$ rooted at this node is of height k and contains $2 ^ { k }$ leaves (s–t cuts). Among these $2 ^ { k }$ cuts, let $\mathbf { { a } ^ { \prime } }$ be a minimum weight cut. Show how the remaining $2 ^ { k } - 1$ cuts of $T ^ { \prime }$ can be partitioned into k subtrees which are of height $0 , 1 , \ldots , k - 1$

28.12 Using a heap, give an algorithm for enumerating s–t cuts in $G$ by increasing weight.

Hint: The heap is initialized with a minimum cut in G. At an arbitrary point, the cuts not enumerated so far can be partitioned into subtrees (see Exercise 28.11). The heap contains a minimum cut from each subtree.

28.13 Give an algorithm for enumerating all cuts in an undirected graph by increasing weight with polynomial delay.

Hint: Assume that the graph has a special vertex $s ,$ which always goes on side 0 of the cut, and n other vertices, numbered 1 to n. A cut is specified by an n bit vector specifying the sides of vertices numbered 1 to $n .$ . The main diference arises in finding a minimum cut in the subtree rooted at the internal node $0 ^ { k } , k < n$ . This is done by finding a minimum cut separating the vertices $s , 1 , \ldots , i$ from vertex $i + 1$ for $k \leq i < n$ , and picking the lightest of these cuts.

28.14 (Karger [156]) Consider the generalization of network reliability to estimating the probability that G disconnects into r or more components, where r is a fixed constant. Obtain an FPRAS for this problem.

## 28.4 Notes

The counting class #P was defined by Valiant [249]. The FPRAS for counting DNF solutions is due to Karp and Luby [165], who also gave the definition of FPRAS (see also Karp, Luby, and Madras [166]). The FPRAS for estimating network reliability is due to Karger [156].

Most algorithms for approximate counting work by constructing an almost uniform generator for the problem and appealing to the equivalence established in Exercise 28.7. Broder [33] introduced the use of rapidly mixing Markov chains for almost uniform generation (see also Mihail [205]).

Jerrum and Sinclair [145] gave the first FPRAS using this approach, for counting the number of perfect matchings in dense bipartite graphs (each vertex should have a degree $\geq n / 2 ;$ see also Section 30.3). They also showed that a crude approximate counter, with polynomial error, can be transformed into an FPRAS (with inverse polynomial error), by defining an appropriate Markov chain on the self-reducibility tree of an instance. As a result, #P-complete problems either admit an FPRAS or are essentially not approximable at all (see Exercise 28.8) For Markov–chain based approximate counting algorithms, see Jerrum and Sinclair [142], Sinclair [242], and the references in Section 30.3.
