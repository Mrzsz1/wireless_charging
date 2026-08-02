---
title: "approximation-algorithms-ch-29-part-030"
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
pdf_path: "work/core-books/approximation-algorithms/parts/approximation-algorithms-ch-29-part-030.pdf"
raw_md: "raw/canonical/approximation-algorithms-ch-29-part-030/full.md"
---
## 29 Hardness of Approximation

A remarkable achievement of the theory of exact algorithms is that it has provided a fairly complete characterization<sup>1</sup> of the intrinsic complexity of natural computational problems, modulo some strongly believed conjectures. Recent impressive developments raise hopes that we will some day have a comprehensive understanding of the approximability of $\mathbf { N P } .$ -hard optimization problems as well. In this chapter we will give a brief overview of these developments.

Current hardness results fall into three important classes. For minimization problems, the hardness factors for these classes are constant $( > 1 )$ 2 $\Omega ( \log n )$ , and $n ^ { \varepsilon }$ for a fixed constant $\varepsilon > 0$ , where n is the size of the instance. For maximization problems, the factors are constant $( < 1 ) , O ( 1 / \log n )$ , and $1 / n ^ { \varepsilon }$ for a fixed $\varepsilon > 0$ . In this chapter we will present hardness results for $\mathrm { M A X - 3 S A T } .$ vertex cover, and Steiner tree in the first class, set cover in the second class, and clique in the third class. For all these problems, we will establish hardness for their cardinality versions, i.e., the unit cost case.

## 29.1 Reductions, gaps, and hardness factors

Let us start by recalling the methodology for establishing hardness results for exact optimization problems. The main technical core is the Cook–Levin theorem which establishes the hardness, assuming $\mathbf { P } \neq \mathbf { N P }$ , of distinguishing between instances of SAT that are satisfiable and those that are not. To show hardness of computing an optimal solution to, say the cardinality vertex cover problem, one shows, via a polynomial time reduction from SAT, that it is hard to distinguish between graphs that have covers of size at most k from graphs that don’t, where k is provided as part of the input. Since an exact algorithm can make this distinction, this reduction establishes the non-existence of an eficient exact algorithm.

The main technical core of hardness of approximation results is the PCP theorem, which is stated in Section 29.2. For establishing a hardness of approximation result for, say, the vertex cover problem, this theorem is used to show the following polynomial time reduction. It maps an instance $\phi$ of SAT to a graph $G = ( V , E )$ such that

• if $\phi$ is satisfiable, G has a vertex cover of size $\leq { \frac { 2 } { 3 } } | V |$ , and

• if $\phi$ is not satisfiable, the smallest vertex cover in $G$ is of size $> \alpha \cdot { \frac { 2 } { 3 } } | V |$ 2

where $\alpha > 1$ is a fixed constant.

Claim 29.1 As a consequence of the reduction stated above, there is no polynomial time algorithm for vertex cover that achieves an approximation guarantee of $\alpha ,$ , assuming $\mathbf { P } \neq \mathbf { N P }$

Proof: Essentially, this reduction establishes the hardness, assuming $\mathbf { P } \neq$ $\mathbf { N P } ,$ of distinguishing graphs having a cover of size $\leq { \frac { 2 } { 3 } } | V |$ from those having a cover of size $> \alpha \cdot { \frac { 2 } { 3 } } | V |$ . An approximation algorithm for vertex cover, having a guarantee of α or better, will find a cover of size $\leq \alpha \cdot { \frac { 2 } { 3 } } | V |$ when given a graph $G$ from the first class. Thus, it will be able to distinguish the two classes of graphs, leading to a contradiction. ✷

The reduction stated above introduces a gap, of factor $\alpha ,$ , in the optimal objective function value achieved by the two classes of graphs $( \mathrm { i f } \alpha = 1$ then this is an ordinary polynomial time reduction from $\mathrm { S A T }$ to vertex cover). Let us formally state the central notion of a gap-introducing reduction. The definition is slightly diferent for minimization and maximization problems. For simplicity, let us assume that we are always reducing from $\mathrm { S A T }$

Let Π be a minimization problem. A gap-introducing reduction from SAT to Π comes with two parameters, functions $f$ and $\alpha .$ . Given an instance $\phi$ of SAT, it outputs, in polynomial time, an instance x of $\pi$ , such that

• if φ is satisfiable, $\mathrm { O P T } ( x ) \leq f ( x )$ , and

• if $\phi$ is not satisfiable, $\mathrm { O P T } ( x ) > \alpha ( | x | ) \cdot f ( x )$

Notice that $f$ is a function of the instance (such as ${ \frac { 2 } { 3 } } \vert V \vert$ in the example given above), and $\alpha$ is a function of the size of the instance. Since Π is a minimization problem, the function α satisfies $\alpha ( | x | ) \geq 1$

If Π is a maximization problem, we want the reduction to satisfy

• if $\phi$ is satisfiable, $\mathrm { O P T } ( x ) \geq f ( x )$ , and

• if $\phi$ is not satisfiable, $\mathrm { O P T } ( x ) < \alpha ( | x | ) \cdot f ( x )$

In this case, $\alpha ( | x | ) \leq 1$ . The $\mathrm { g a p } , \alpha ( | x | )$ , is precisely the hardness factor established by the gap-introducing reduction for the NP-hard optimization problem.

Once we have obtained a gap-introducing reduction from $\mathrm { S A T }$ (or any other NP-hard problem) to an optimization problem, say $\varPi _ { 1 }$ , we can prove a hardness result for another optimization problem, say $\varPi _ { 2 }$ , by giving a special reduction, called a gap-preserving reduction, from $\varPi _ { 1 }$ to $\varPi _ { 2 }$ . Now there are four possibilities, depending on whether $\varPi _ { 1 }$ and $\varPi _ { 2 }$ are minimization or maximization problems. We give the definition below assuming that $\varPi _ { 1 }$ is a minimization problem and $\varPi _ { 2 }$ is a maximization problem. The remaining cases are similar.

A gap-preserving reduction, Γ, from $\varPi _ { 1 }$ to $\varPi _ { 2 }$ comes with four parameters (functions), $f _ { 1 } , \alpha , f _ { 2 }$ , and $\beta .$ . Given an instance x of $\varPi _ { 1 }$ , it computes, in polynomial time, an instance y of $\varPi _ { 2 }$ such that

$$
\bullet \quad \mathrm{OPT} (x) \leq f _ {1} (x) \Rightarrow \mathrm{OPT} (y) \geq f _ {2} (y),
$$

$$
\bullet \quad \mathrm{OPT} (x) > \alpha (| x |) f _ {1} (x) \Rightarrow \mathrm{OPT} (y) <   \beta (| y |) f _ {2} (y).
$$

Observe that x and y are instances of two diferent problems, and so it would be more appropriate to write $\mathrm { O P T } _ { \varPi _ { 1 } } ( x )$ and $\mathrm { O P T } _ { \varPi _ { 2 } } ( y )$ instead of $\mathrm { O P T } ( x )$ and $\mathrm { O P T } ( y )$ , respectively. However, we will avoid this extra notation, since the context clarifies the problems being talked about. In keeping with the fact that $\varPi _ { 1 }$ is a minimization problem and $\varPi _ { 2 }$ is a maximization problem, $\alpha ( | x | ) \geq 1$ and $\beta ( | y | ) \leq 1$

Composing a gap-introducing reduction with a gap-preserving reduction gives a gap-introducing reduction, provided all the parameters match up. For example, suppose that in addition to the reduction Γ defined above, we have obtained a gap-introducing reduction, $\varGamma ^ { \prime }$ , from SAT to $\varPi _ { 1 }$ , with parameters $f _ { 1 }$ and α. Then, composing $\varGamma ^ { \prime }$ with $T ,$ we get a gap-introducing reduction from SAT to $\varPi _ { 2 }$ , with parameters $f _ { 2 }$ and $\beta .$ . This composed reduction shows that there is no $\beta ( | y | )$ factor approximation algorithm for $\varPi _ { 2 }$ , assuming $\mathbf { P } \neq$ NP. In each gap-preserving reduction stated below, we will take special care to ensure that the parameters match up.

## Remark 29.2

• The “gap” $\beta$ can, in general, be bigger or smaller than α. In this sense, “gap-preserving” is a slight misnomer.

• We do not require any guarantee from reduction Γ if instance x of $\varPi _ { 1 }$ falls in the first gap, i.e., satisfies $f _ { 1 } ( x ) < \mathrm { O P T } ( x ) \leq \alpha ( | x | ) f _ { 1 } ( x )$

• An approximation algorithm for $\varPi _ { 2 }$ together with a gap-preserving reduction Γ from $\varPi _ { 1 }$ to $\varPi _ { 2 }$ does not necessarily yield an approximation algorithm for $\varPi _ { 1 }$ . Observe the contrast with an approximation factor preserving reduction (see Section A.3.1 for definition). The latter reduction additionally requires a means of transforming a near-optimal solution to the transformed instance y of $\varPi _ { 2 }$ into a near-optimal solution to the given instance x of $\varPi _ { 1 }$

On the other hand, Γ together with an appropriate gap-introducing reduction from SAT to $\varPi _ { 1 }$ does sufice for proving a hardness of approximation result for $\varPi _ { 2 }$ . Obviously the less stringent requirement on gap-preserving reductions makes them easier to design.

• We have already presented some gap-introducing reductions, e.g., Theorems 3.6 and 5.7. The reader may wonder why these do not sufice as the starting point for further hardness results and why the PCP theorem was needed. The reason is that these reductions simply exploit the freedom to choose edge costs and not the deep combinatorial structure of the problem.

The following figure shows the gap-preserving reductions presented in this chapter:

![](images/f8a81c3ca5178495bee7715cfa22817b0577d160ffbe3611ca26ef3c122e81a1.jpg)

## 29.2 The PCP theorem

Probabilistic characterizations of the class NP yield a general technique for obtaining gap-introducing reductions. The most useful of these characterizations is captured in the PCP theorem. PCP stands for probabilistically checkable proof systems.

Recall the usual definition of NP (see Appendix A) as the class of languages whose yes instances support short (polynomial in the length of the input) witnesses that can be verified quickly (in polynomial time). Informally, a probabilistically checkable proof for an NP language encodes the witness in a special way so that it can be verified probabilistically by examining very few of its bits.

A probabilistically checkable proof system comes with two parameters, the number of random bits required by the verifier, and the number of bits of the witness that the verifier is allowed to examine. In keeping with established terminology, let us call a witness string the proof. The most useful setting for these parameters is O(log n) and O(1), respectively. This defines the class PCP(log n, 1).

The verifier is a polynomial time Turing machine which, besides its input tape and work tape, has a special tape that provides it with a string of random bits and another special tape on which it is provided with the proof. The machine can read any bit of the proof by simply specifying its location. Of course, the particular locations it examines are a function of the input string and the random string. At the end of its computation, the machine goes into either an accept state or a reject state.

![](images/df6490c75b5542a7f3a4dc0608d7c4958435298b4ca70ce33f63d1495b54beca.jpg)

A language ${ \cal L } \in { \bf P C P } ( \log n , 1 )$ if there is a verifier $V ,$ and constants c and q, such that on input $x , V$ obtains a random string, $r ,$ of length c log |x| and queries q bits of the proof. Furthermore,

• if $x \in L$ , then there is a proof y that makes $V$ accept with probability 1, • if $x \notin L .$ , then for every proof y, V accepts with probability $< 1 / 2$ ,

where the probability is over the random string r. The probability of accepting in case $x \notin L$ is called the error probability.

In general, for two functions $r ( n )$ and $q ( n )$ , we can define the class $\mathbf { P C P } ( r ( n ) , q ( n ) )$ , under which the verifier obtains $O ( r ( n ) )$ random bits and queries $O ( q ( n ) )$ bits of the proof. The acceptance criteria for input strings are the same as above. In this terminology, $\mathbf { N P } = \mathbf { P C P } ( 0 , p o l y ( n ) )$ , where $\textstyle p o l y ( n ) = \bigcup _ { k > 0 } \{ n ^ { k } \}$ . In this case, the verifier is not allowed any random bits. It must deterministically accept strings in the language and reject strings not in the language, as in the definition of NP. The PCP theorem gives another characterization of NP.

## Theorem 29.3 $\mathbf { N P } = \mathbf { P C P } ( \log n , 1 )$

One half of this theorem, that $\mathbf { P C P } ( \log n , 1 ) \subseteq \mathbf { N P }$ , is easy to prove (see Exercise 29.1). The other half, that $\mathbf { N P } \subseteq \mathbf { P C P } ( \log n , 1 )$ , is a dificult result, and gives a useful tool for establishing hardness of approximation results. The currently known proof of this half is too complicated for exposition in this book. Fortunately, the statement of the theorem is suficient to derive the hardness results.

In order to provide the reader with some feel for the PCP theorem, let us make an observation. It is easy to construct a verifier for 3SAT whose error probability (i.e., probability of accepting unsatisfiable formulae) is $\leq 1 - 1 / m .$ where m is the number of clauses in the input 3SAT formula, say φ. The verifier expects a satisfying truth assignment to φ as the proof. It uses the $O ( \log n )$ random bits to pick a random clause of φ. It then reads the truth assignments for the three variables occurring in this clause. Notice that this is only a constant number of bits. It accepts if the truth setting for these three variables satisfies the clause. Clearly, if φ is satisfiable, there is a proof that makes the verifier accept with probability 1, and if φ is not satisfiable, on every proof, the verifier accepts with probability $\leq 1 - 1 / m$ . The interesting and dificult part of the $\mathrm { P C P }$ theorem is decreasing the error probability to $< 1 / 2$ , even though the verifier is allowed to read only a constant number of bits of the proof. It involves a complex algebraic construction that ensures that small parts of the proof depend on every bit of the input.

The PCP theorem directly gives an optimization problem – in particular, a maximization problem – for which there is no factor $1 / 2$ approximation algorithm, assuming $\mathbf { P } \neq \mathbf { N P }$

Problem 29.4 (Maximize accept probability) Let V be a $\mathbf { P C P } ( \log n , 1 )$ verifier for SAT. On input $\phi , \mathrm { a }$ SAT formula, find a proof that maximizes the probability of acceptance of V.

Claim 29.5 Assuming $\mathbf { P } \neq \mathbf { N P }$ , there is no factor $1 / 2$ approximation algorithm for Problem 29.4.

Proof: If φ is satisfiable, then there is a proof that makes V accept with probability 1, and if $\phi$ is not satisfiable, then on every proof, V accepts with probability $< 1 / 2 .$ . Suppose there is a factor $1 / 2$ approximation algorithm for Problem 29.4. If φ is satisfiable, then this algorithm must provide a proof on which $V \mathrm { { s } }$ acceptance probability is $\geq 1 / 2$ . The acceptance probability can be computed in polynomial time, by simply simulating V for all random strings of length $O ( \log n )$ . Thus, this approximation algorithm can be used for deciding SAT in polynomial time, contradicting the assumption $\mathbf { P } \neq \mathbf { N P }$ ✷

Claim 29.5 directly gives the following corollary. In subsequent sections, we will use the PCP theorem to obtain hardness results for natural computational problems. A similar corollary follows in each case.

Corollary 29.6 Assuming $\mathbf { P } \neq \mathbf { N P }$ , there is no PTAS for Problem ${ \it 2 9 . 4 } .$

## 29.3 Hardness of MAX-3SAT

MAX-3SAT is the restriction of MAX-SAT (see Problem 16.1) to instances in which each clause has at most three literals. This problem plays a similar role in hardness of approximation as 3SAT plays in the theory of NP-hardness, as a “seed” problem from which reductions to numerous other problems have been found. The main result of this section is:

Theorem 29.7 There is a constant $\varepsilon _ { M } ~ > ~ 0$ for which there is a gapintroducing reduction from SAT to MAX-3SAT that transforms a Boolean formula φ to ψ such that

$i f \phi$ is satisfiable, $\mathrm { O P T } ( \psi ) = m$ , and

$i f \phi$ is not satisfiable, $\mathrm { O P T } ( \psi ) < ( 1 - \varepsilon _ { M } ) m$ 9

where m is the number of clauses in $\psi .$ .

Corollary 29.8 There is no approximation algorithm for MAX-3SAT with an approximation guarantee of $1 - \varepsilon _ { M }$ , assuming $\mathbf { P } \neq \mathbf { N P }$ , where $\varepsilon _ { M } > 0$ is the constant defined in Theorem 29.7.

The exact solution of MAX-3SAT is shown hard under the assumption $\mathbf { P } \neq \mathbf { N P }$ . It is interesting to note that hardness of approximate solution of MAX-3SAT is also being established under the same assumption.

For clarity, let us break the proof into two parts. We will first prove hardness for the following problem.

Problem 29.9 (MAX k-FUNCTION SAT) Given n Boolean variables $x _ { 1 } , \ldots , x _ { n }$ and m functions $f _ { 1 } , \ldots , f _ { m }$ , each of which is a function of k of the Boolean variables, find a truth assignment to $x _ { 1 } , \ldots , x _ { n }$ that maximizes the number of functions satisfied. Here k is assumed to be a fixed constant. Thus, we have a class of problems, one for each value of k.

Lemma 29.10 There is a constant k for which there is a gap-introducing reduction from SAT to MAX k-FUNCTION SAT that transforms a Boolean formula φ to an instance I of MAX k-FUNCTION SAT such that

$i f \phi$ is satisfiable, $\mathrm { O P T } ( I ) = m _ { \mathrm { 3 } }$ , and

$i f \phi$ is not satisfiable, $\mathrm { O P T } ( I ) < \frac { 1 } { 2 } m$

where m is the number of formulae in $I .$

Proof: Let V be a $\mathbf { P C P } ( \log n , 1 )$ verifier for SAT, with associated parameters c and q. Let $\phi$ be an instance of SAT of length n. Corresponding to each string, r, of length c log n (the “random” string), V reads q bits of the proof. Thus, V reads a total of at most $q n ^ { c }$ bits of the proof. We will have a Boolean variable corresponding to each of these bits. Let B be the set of Boolean variables. Thus, the relevant part of each proof corresponds to a truth assignment to the variables in $B .$

We will establish the lemma for $k = q$ . Corresponding to each string $r ,$ we will define a Boolean function, $f _ { r }$ . This will be a function of $q$ variables from B. The acceptance or rejection of V is of course a function of $\phi , r ,$ and the $q$ bits of the proof read by $V .$ For fixed $\phi$ and $r ,$ consider the restriction of this function to the $q$ bits of the proof. This is the function $f _ { r }$

Clearly, there is a polynomial time algorithm which, given input $\phi ,$ outputs the $m = n ^ { c }$ functions $f _ { r }$ . If $\phi$ is satisfiable, there is a proof that makes $V$ accept with probability 1. The corresponding truth assignment to $B$ satisfies all $n ^ { c }$ functions $f _ { r }$ . On the other hand, if $\phi$ is not satisfiable, then on every proof, $V$ accepts with probability $< 1 / 2$ . Thus, in this case every truth assignment satisfies $< { \frac { 1 } { 2 } } n ^ { c }$ of these functions. The lemma follows. ✷

Proof of Theorem 29.7: Using Lemma 29.10 we transform a SAT formula $\phi$ to an instance of MAX k-FUNCTION SAT. We now show how to obtain a 3SAT formula from the $n ^ { c }$ functions.

Each Boolean function $f _ { r }$ constructed in Lemma 29.10 can be written as a SAT formula, say $\psi _ { r } ,$ , containing at most $2 ^ { q }$ clauses. Each clause of $\psi _ { r }$ contains at most $q$ literals. Let $\psi$ be the SAT formula obtained by taking the conjunct of all these formulae, $\mathrm { i . e . , } \psi = \Lambda _ { r }$ ψ<sub>r</sub>.

\*If a truth assignment satisfies formula $f _ { r } ,$ , then it satisfies all clauses of $\psi _ { r }$ . On the other hand, if it does not satisfy $f _ { r } ,$ then it must leave at least one clause of $\psi _ { r }$ unsatisfied. Therefore, if $\phi$ is not satisfiable, any truth assignment must leave $> { \frac { 1 } { 2 } } n ^ { c }$ clauses of $\psi$ unsatisfied.

Finally, let us transform $\psi$ into a 3SAT formula. This is done using the standard trick of introducing new variables to obtain small clauses from a big clause. Consider clause $C = ( x _ { 1 } \vee x _ { 2 } \vee . . . \vee x _ { k } )$ , with $k > 3$ . Introduce $k - 2$ new Boolean variables, $y _ { 1 } , \ldots , y _ { k - 2 }$ , and consider the formula

$$
f = (x _ {1} \vee x _ {2} \vee y _ {1}) \wedge (\overline {{y}} _ {1} \vee x _ {3} \vee y _ {2}) \wedge \ldots \wedge (\overline {{y}} _ {k - 2} \vee x _ {k - 1} \vee x _ {k}).
$$

Let $\tau$ be any truth assignment to $x _ { 1 } , \ldots , x _ { k }$ . If τ satisfies $C .$ , then it can be extended to a truth assignment satisfying all clauses of $f .$ On the other hand, if $\tau$ does not satisfy $C .$ , then for every way of setting $y _ { 1 } , \ldots , y _ { k - 2 }$ , at least one of the clauses of $f$ remains unsatisfied.

We apply this construction to every clause of $\psi$ containing more than 3 literals. Let $\psi ^ { \prime }$ be the resulting 3SAT formula. It contains at most $n ^ { c } 2 ^ { q } ( q - 2 )$ clauses. If $\phi$ is satisfiable, then there is a truth assignment satisfying all clauses of $\psi ^ { \prime }$ . If $\phi$ is not satisfiable, $> ~ { \frac { 1 } { 2 } } n ^ { c }$ of the clauses remain unsatisfied, under every truth assignment. Setting $\varepsilon _ { M } = 1 / ( 2 ^ { q + 1 } ( q - 2 ) )$ gives the theorem. ✷

## 29.4 Hardness of MAX-3SAT with bounded occurrence of variables

For each fixed $k ,$ define MAX-3SAT(k) to be the restriction of MAX-3SAT to Boolean formulae in which each variable occurs at most $k$ times. This problem leads to reductions to some key optimization problems.

## 314 29 Hardness of Approximation

Theorem 29.11 There is a gap preserving reduction from MAX-3SAT to MAX-3SAT(29) that transforms a Boolean formula φ to ψ such that

$i f \operatorname { O P T } ( \phi ) = m$ , then $\mathrm { O P T } ( \psi ) = m ^ { \prime } .$ , and

$i f \mathrm { O P T } ( \phi ) < ( 1 - \varepsilon _ { M } ) m _ $ , then $\mathrm { O P T } ( \psi ) < ( 1 - \varepsilon _ { b } ) m ^ { \prime } ;$

where m and $m ^ { \prime }$ are the number of clauses in $\phi$ and $\psi , \varepsilon _ { M }$ is the constant determined in Theorem 29.7, and $\varepsilon _ { b } = \varepsilon _ { M } / 4 3$

Proof: The proof critically uses expander graphs. Recall, from Section 20.3, that graph $G = ( V , E )$ is an expander if every vertex has the same degree, and for any nonempty subset $S \subset V$ 2

$$
| E (S, \overline {{S}}) | > \min (| S |, | \overline {{S}} |),
$$

where $E ( S , { \overline { { S } } } )$ denotes the set of edges in the cut (S, S), i.e., edges that have one endpoint in $S$ and the other in S. Let us assume that such graphs are eficiently constructible in the following sense. There is an algorithm $\mathcal { A }$ and a constant $N _ { 0 }$ such that for each $N \geq N _ { 0 } .$ , A constructs a degree 14 expander graph on $N$ vertices in time polynomial in $N$ (Remark 29.12 clarifies this point).

Expanders enable us to construct the following device whose purpose is to ensure that in any optimal truth assignment, a given set of Boolean variables must have consistent assignment, i.e., all true or all false. Let $k \geq N _ { 0 } ,$ , and let $G _ { x }$ be a degree 14 expander graph on $k$ vertices. Label the vertices with distinct Boolean variables $x _ { 1 } , \ldots , x _ { k }$ . We will construct a CNF formula $\psi _ { x }$ on these Boolean variables. Corresponding to each edge $( x _ { i } , x _ { j } )$ of $G _ { x }$ , we will include the clauses $( { \overline { { x } } } _ { i } \vee x _ { j } )$ and $( { \overline { { x } } } _ { j } \vee x _ { i } )$ in $\psi _ { x } . \mathrm { ~ A ~ }$ truth assignment to $x _ { 1 } , \ldots , x _ { k }$ is said to be consistent if either all the variables are set to true or all are set to false. An inconsistent truth assignment partitions the vertices of $G _ { x }$ into two sets, say $S$ and S. Assume w.l.o.g. that $S$ is the smaller set. Now, corresponding to each edge in the cut (S, S), $\psi _ { x }$ will have an unsatisfied clause. Therefore, the number of unsatisfied clauses, $| E ( S , { \overline { { S } } } ) |$ |, is at least $\vert S \vert + 1$ . We will use this fact critically.

Next, we describe the reduction. We may assume w.l.o.g. that every variable occurs in $\phi$ at least $N _ { 0 }$ times. If not, we can replicate each clause $N _ { 0 }$ times without changing the approximability properties of the formula in any essential way.

Let B denote the set of Boolean variables occurring in φ. For each variable $x \in B$ , we will do the following. Suppose x occurs $k \geq N _ { 0 }$ times in $\phi .$ . Let $V _ { x } = \{ x _ { 1 } , \ldots , x _ { k } \}$ be a set of completely new Boolean variables. Let $G _ { x }$ be a degree 14 expander graph on k vertices. Label its vertices with variables from $V _ { x }$ and obtain formula $\psi _ { x }$ as described above. Finally, replace each occurrence of x in $\phi$ by a distinct variable from $V _ { x } .$ . After this process is carried out for each variable $x \in B$ , every occurrence of a variable in $\phi$ is replaced by a distinct variable from the set of new variables

$$
V = \bigcup_ {x \in B} V _ {x}.
$$

Let $\phi ^ { \prime }$ be the resulting formula. In addition, corresponding to each variable $x \in B$ , a formula $\psi _ { x }$ has been constructed.

Finally, let

$$
\psi = \phi^ {\prime} \wedge (\bigwedge_ {x \in B} \psi_ {x}).
$$

Observe that for each $x \in B .$ each variable of $V _ { x }$ occurs exactly 29 times in $\psi$ – once in $\phi ^ { \prime } .$ , and 28 times in $\psi _ { x }$ . Therefore, ψ is an instance of MAX-3SAT(29). We will say that the clauses of $\phi ^ { \prime }$ are Type I clauses, and the remaining clauses of ψ are Type II clauses.

Now, the important claim is that an optimal truth assignment for $\psi$ must satisfy all Type II clauses, and therefore must be consistent for each set $V _ { x } , x \ \in \ B$ Suppose that this is not the case. Let τ be an optimal truth assignment that is not consistent for $V _ { x } ,$ for some $x \in B , \tau$ partitions the vertices of $G _ { x }$ into two sets, say S and ${ \overline { { S } } } ,$ with S being the smaller set. Now, flip the truth assignment to variables in $S ,$ keeping the rest of the assignment the same as τ. As a result, some Type I clauses that were satisfied under τ may now be unsatisfied. Each of these must contain a variable of $S ,$ and so their number is at most |S|. On the other hand we get at least $\vert S \vert + 1$ new satisfied clauses corresponding to the edges in the cut (S, S). Thus, the flipped assignment satisfies more clauses than $\tau ,$ contradicting the optimality of $\tau$ .

Let m and $m ^ { \prime }$ be the number of clauses in $\phi$ and $\psi .$ The total number of occurrences of all variables in $\phi$ is at most 3m. Each occurrence participates in 28 Type II two-literal clauses, giving a total of at most 42m Type II clauses. In addition, ψ has m Type I clauses. Therefore, $m ^ { \prime } \leq 4 3 m$

If φ is satisfiable, then so is $\psi .$ . Next, consider the case that $\mathrm { O P T } ( \phi ) <$ $( 1 - \varepsilon _ { M } ) m , { \mathrm { i . e . , } } > \varepsilon _ { M } m$ clauses of $\phi$ remain unsatisfied under any truth assignment. If so, by the above claim, $> \varepsilon _ { M } m \ge \varepsilon _ { M } m ^ { \prime } / 4 3$ of the clauses of $\psi$ must remain unsatisfied. The theorem follows. ✷

Remark 29.12 The assumption about the eficient construction of expander graphs is slightly untrue. It is known that for each $N \geq N _ { 0 }$ , an expander of size $\le N ( 1 { + } o ( 1 ) )$ can be constructed eficiently (see Section 29.9). The reader can verify that this does not change the status of Theorem 29.11.

Exercise 29.4 extends Theorem 29.11 to establishing hardness for MAX-3SAT(5).

## 29.5 Hardness of vertex cover and Steiner tree

In this section, we will apply the machinery developed above to some graph theoretic problems. For integer $d \geq 1$ , let $\mathrm { V C } ( d )$ denote the restriction of the cardinality vertex cover problem to instances in which each vertex has degree at most $d .$

Theorem 29.13 There is a gap preserving reduction from $M A X  – 3 S A T ( 2 9 )$ $t o \ V C ( { \mathcal { 3 0 } } )$ that transforms a Boolean formula $\phi$ to a graph $G = ( V , E )$ such that

$i f \operatorname { O P T } ( \phi ) = m$ , then $\begin{array} { r } { \mathrm { O P T } ( G ) \le \frac { 2 } { 3 } | V | _ { ; } } \end{array}$ , and

$\ i f \mathrm { O P T } ( \phi ) < ( 1 - \varepsilon _ { b } ) m _ { \cdot }$ , then $\mathrm { O P T } ( G ) > ( 1 + \varepsilon _ { v } ) \frac { 2 } { 3 } | V |$

where m is the number of clauses in $\phi , \ \varepsilon _ { b }$ is the constant determined in Theorem ${ \it 2 9 . 1 1 , }$ and $\varepsilon _ { v } = \varepsilon _ { b } / 2$

Proof: Assume w.l.o.g. that each clause of $\phi$ has exactly 3 literals (this can be easily accomplished by repeating the literals within a clause, if necessary). We will use the standard transformation. Corresponding to each clause of φ, G has $3$ vertices. Each of these vertices is labeled with one literal of the clause. Thus, $| V | = 3 m$ . G has two types of edges (see the illustration below):

• for each clause, G has 3 edges connecting its 3 vertices, and

• for each $u , v \in V .$ , if the literals labeling u and v are negations of each other, then $( u , v )$ is an edge in G.

Each vertex of G has two edges of the first type and at most 28 edges of the second type. Hence, G has degree at most 30.

We claim that the size of a maximum independent set in $G$ is precisely $\mathrm { O P T } ( \phi )$ ). Consider an optimal truth assignment and pick one vertex, corresponding to a satisfied literal, from each satisfied clause. Clearly, the picked vertices form an independent set. Conversely, consider an independent set I in $G ,$ and set the literals corresponding to its vertices to be true. Any extension of this truth setting to all variables must satisfy at least |I| clauses.

The complement of a maximum independent set in G is a minimum vertex cover. Therefore, if $\mathrm { O P T } ( \phi ) = m$ then $\mathrm { O P T } ( G ) = 2 m$ . If $\mathrm { O P T } ( \phi ) < ( 1 - \varepsilon _ { b } ) m .$ then $\mathrm { O P T } ( G ) > ( 2 + \varepsilon _ { b } ) m$ . The theorem follows. ✷

As an illustration, consider the formula $( x _ { 1 } \lor { \overline { { x } } } _ { 2 } \lor x _ { 3 } ) \land ( { \overline { { x } } } _ { 1 } \lor x _ { 2 } \lor x _ { 3 } )$ The graph produced by the reduction given in Theorem 29.13 is given below:

![](images/c156c92be620cda6011e91a9139ef0bebcf04adf6538695504dec416c5908a14.jpg)  
Theorem 29.14 There is a gap preserving reduction from $V C ( 3 0 )$ to the Steiner tree problem. It transforms an instance $G = ( V , E )$ of $V C ( 3 0 )$ to an instance $H = ( R , S ;$ , cost) of Steiner tree, where R and S are the required and Steiner vertices of H, and cost is a metric on $R \cup S$ . It satisfies:

$i f \operatorname { O P T } ( G ) \leq { \frac { 2 } { 3 } } | V |$ , then $\begin{array} { r } { \mathrm { O P T } ( H ) \le | R | + \frac { 2 } { 3 } | S | - 1 } \end{array}$ , and

$\begin{array} { r } { i f \mathrm { O P T } ( G ) > ( 1 + \varepsilon _ { v } ) \frac { 2 } { 3 } | V | } \end{array}$ , then $\mathrm { O P T } ( H ) \stackrel { \smile } { > } ( 1 + \varepsilon _ { s } ) ( | R | + { \textstyle { \frac { 2 } { 3 } } } | S | - 1 )$

where $\varepsilon _ { s } = 4 \varepsilon _ { v } / 9 7$ , and $\varepsilon _ { v }$ is the constant determined in Theorem 29.13.

Proof: Graph $H = ( R , S , \mathrm { c o s t } )$ will be such that G has a vertex cover of size c if H has a Steiner tree of cost $| R | + c - 1$ . H will have a required vertex $r _ { e }$ corresponding to each edge $e \in E$ and a Steiner vertex $s _ { v }$ corresponding to each vertex $v \in V$ . The edge costs are as follows. An edge between a pair of Steiner vertices is of cost 1, and an edge between a pair of required vertices is of cost 2. An edge $( r _ { e } , s _ { v } )$ is of cost 1 if edge e is incident at vertex v in $G _ { \ l }$ and it is of cost 2 otherwise.

Let us show that G has a vertex cover of size c if H has a Steiner tree of cost $| R | + c - 1$ . For the forward direction, let $S _ { c }$ be the set of Steiner vertices in H corresponding to the c vertices in the cover. Observe that there is a tree in H covering $R \cup S _ { c }$ using cost 1 edges only (since every edge $e \in E$ must be incident at a vertex in the cover). This Steiner tree has cost $| R | + c - 1$

For the reverse direction, let T be a Steiner tree in H of cost $| R | + c - 1$ We will show below that $T$ can be transformed into a Steiner tree of the same cost that uses edges of cost 1 only. If so, the latter tree must contain exactly c Steiner vertices. Moreover, every required vertex of H must have a unit cost edge to one of these Steiner vertices. Therefore, the corresponding c vertices of G form a cover.

Let $( u , v )$ be an edge of cost 2 in T. We may assume w.l.o.g. that u and v are both required. (If u is Steiner, remove $( u , v )$ from $T$ , getting two components. Throw in an edge from v to a required vertex to connect the two sides, and get a Steiner tree of the same cost as $T . )$ Let $e _ { u }$ and $e _ { v }$ be the edges, in $G ,$ , corresponding to these vertices. Since G is connected, there is a path, $p ,$ from one of the endpoints of $e _ { u }$ to one of the endpoints of $e _ { v }$ in G. Now, removing $( u , v )$ from $T$ gives two connected components. Let the set of required vertices in these two sets be $R _ { 1 }$ and $R _ { 2 }$ . Clearly, u and v lie in diferent sets, so path p must have two adjacent edges, say $( a , b )$ and $( b , c )$ such that their corresponding vertices, say w and $w ^ { \prime } .$ , lie in $R _ { 1 }$ and $R _ { 2 }$ respectively. Let the Steiner vertex, in $H$ , corresponding to b be $s _ { b }$ . Now, throwing in the edges $( s _ { b } , w )$ and $( s _ { b } , w ^ { \prime } )$ must connect the two components. Observe that these two edges are of unit cost.

Now, if $\begin{array} { r } { \mathrm { O P T } ( G ) \le \frac { 2 } { 3 } | V | } \end{array}$ , then $\mathrm { O P T } ( H ) > | R | + \frac { 2 } { 3 } | S | - 1$ , and if $\operatorname { O P T } ( G ) >$ $( 1 + \varepsilon _ { v } ) { \frac { 2 } { 3 } } | V |$ , then $\mathrm { O P T } ( H ) > | R | + ( 1 + \varepsilon _ { v } ) { \frac { 2 } { 3 } } | S | - \overset { \circ } { 1 }$ . The theorem follows. ✷

The reduction is illustrated below. Graph G is an instance of the vertex cover problem. The highlighted vertices form a cover. Graph H shows the Steiner tree corresponding to this cover in the reduced graph. Required vertices have been marked with squares, and the three Steiner vertices corresponding to the cover have been marked with circles (the remaining Steiner vertices have been omitted for clarity). The edge between two Steiner vertices in the tree is dotted to distinguish it from the remaining edges, which connect required and Steiner vertices.

![](images/9d7d8955d596487b8708ab704507b29eb11cc7e99b85c79966884ecde69be8cf.jpg)  
G

![](images/8dfe615b5fffcfc855f7757488a6e413e941dcb676ea4df3fee220a9f1307317.jpg)  
H

## 29.6 Hardness of clique

The best approximation algorithms known for some problems, including clique, are extremely weak – to the extent that the solution produced by the best known algorithm is only very slightly better than picking a trivial feasible solution. Recent hardness results have been invaluable in explaining why this is so: these problems are inherently inapproximable (essentially). In this section, we will establish this for clique:

Problem 29.15 (Clique) Given an undirected graph $G = ( V , E )$ with nonnegative weights on vertices, find a clique of maximum weight. A clique in G is a subset of vertices, $S \subseteq V$ , such that for each pair $u , v \in S , ( u , v ) \in E$ Its weight is the sum of weights of its vertices.

Consider the cardinality version of this problem, i.e., when all vertex weights are unit. In this section we will show that there is a constant ${ \varepsilon } _ { q } > 0$ such that there is no $1 / ( n ^ { \varepsilon _ { q } } )$ factor approximation algorithm for this problem, assuming $\mathbf { P } \neq \mathbf { N P }$ . Let us first prove the following weaker result.

Lemma 29.16 For fixed constants b and $q _ { \mathrm { { ; } } }$ there is a gap-introducing reduction from SAT to clique that transforms a Boolean formula φ of size n to a graph $G = ( V , E )$ , where $| V | = 2 ^ { q } n ^ { b }$ , such that

$i f \phi$ is satisfiable, $\mathrm { O P T } ( G ) \geq n ^ { b }$ , and

$i f \phi$ is not satisfiable, $\mathrm { O P T } ( G ) < { \textstyle { \frac { 1 } { 2 } } } n ^ { b }$

Proof: Let $F$ be a $\mathbf { P C P } ( \log n , 1 )$ verifier for SAT that requires b log n random bits and queries $q$ bits of the proof. We will transform a SAT instance, $\phi ,$ of size n to a graph $G = ( V , E )$ as follows. For each choice of a binary string, $r ,$ of b log n bits, and each truth assignment, $\tau _ { : }$ , to $q$ Boolean variables, there is a vertex $v _ { r , \tau }$ in G. Thus, $| V | = 2 ^ { q } \bar { n ^ { b } }$

Let $Q ( r )$ represent the $q$ positions in the proof that $F$ queries when it is given string $r$ as the “random” string. We will say that vertex $v _ { r , \tau }$ is accepting if $F$ accepts when it is given random string $r$ and when it reads $\tau$ in the $Q ( r )$ positions of the proof; it is rejecting otherwise. Vertices $v _ { r _ { 1 } , \tau _ { 1 } }$ and $v _ { r _ { 2 } , \tau _ { 2 } }$ are consistent if $\tau _ { 1 }$ and $\tau _ { 2 }$ agree at each position at which $Q ( r _ { 1 } )$ and $Q ( r _ { 2 } )$ overlap. Clearly, a necessary condition for consistency is that $r _ { 1 } \neq r _ { 2 }$ Two distinct vertices $v _ { r _ { 1 } , \tau _ { 1 } }$ and $v _ { r _ { 2 } , \tau _ { 2 } }$ are connected by an edge in $G$ if they are consistent and they are both accepting. Vertex $v _ { r , \tau }$ is consistent with proof $p$ if positions $Q ( r )$ of $p$ contain $\tau .$

If $\phi$ is satisfiable, there is a proof, $p ,$ on which $F$ accepts for each choice, $r ,$ of the random string. For each $r ,$ let $p ( r )$ be the truth setting assigned by proof $p$ to positions $Q ( r )$ . Now, the vertices $\{ v _ { r , p ( r ) } \mid | r | = b \log n \}$ form a clique in $G$ of size $n ^ { b }$

Next, suppose that $\phi$ is not satisfiable, and let $C$ be a clique in $G .$ Since the vertices of $C$ are pairwise consistent, there is a proof, $p _ { : }$ , that is consistent with all vertices of $C$ . Therefore, the probability of acceptance of $F$ on proof $p$ is at least $| C | / n ^ { b }$ (notice that the vertices of $C$ must correspond to distinct random strings). Since the probability of acceptance of any proof is $< 1 / 2$ the largest clique in $G$ must be of size $< { \textstyle { \frac { 1 } { 2 } } } n ^ { b }$ ✷

As a consequence of Lemma 29.16, there is no factor $1 / 2$ approximation algorithm for clique assuming $\mathbf { P } \neq \mathbf { N P }$ . Observe that the hardness factor established is precisely the bound on the error probability of the probabilistically checkable proof for SAT. By the usual method of simulating the verifier a constant number of times, this can be made $1 / k$ for any constant $k ,$ , leading to a similar hardness result for clique. In order to achieve the claimed hardness, the error probability needs to be made inverse polynomial. This motivates generalizing the definition of PCP as follows. Let us define two additional parameters, c and $s ,$ called completeness and soundness, respectively. A language ${ \cal L } \in { \bf P C P } _ { c , s } [ r ( n ) , q ( n ) ]$ if there is a verifier $V _ { ; }$ , which on input x of length $n ,$ obtains a random string of length $O ( r ( n ) )$ , queries $O ( q ( n ) )$ bits of the proof, and satisfies:

• if $x \in L .$ , there is a proof y that makes V accept with probability $\geq c .$

• if $x \notin L .$ , then for every proof $y , V$ accepts with probabilit $\mathrm {  ~ y ~ } < s .$

Thus, the previously defined class $\mathbf { P C P } [ r ( n ) , q ( n ) ] = \mathbf { P C P } _ { 1 , \frac { 1 } { 2 } } [ r ( n ) , q ( n ) ]$ . In general, c and s may be functions of n.

We would like to obtain a PCP characterization of NP which has inverse polynomial soundness. An obvious way of reducing soundness is to simulate a $\mathbf { P C P } [ \log n , 1 ]$ verifier multiple number of times and accept if the verifier accepts each time. Simulating k times will reduce soundness to $1 / 2 ^ { k }$ ; however, this will increase the number of random bits needed to $O ( k \log n )$ and the number of query bits to $O ( k )$ . Observe that the number of vertices in the graph constructed in Lemma 29.16 is $2 ^ { O ( r ( n ) + q ( n ) ) }$ <sup>)</sup>. To achieve inverse polynomial soundness, k needs to be $\Omega ( \log n )$ . For this value of $k ,$ the number of bits queried is $O ( \log n )$ , which is not a problem. However, the number of random bits needed is ${ \bar { O } } ( \log ^ { 2 } n )$ , which leads to a superpolynomial sized graph.

The following clever idea overcomes this dificulty. We will use a constant degree expander graph to generate $O ( \log n )$ strings of b log n bits each, using only $O ( \log n )$ truly random bits. The verifier will be simulated using these $O ( \log n )$ strings as the “random” strings. Clearly, these are not truly random strings. Properties of expanders help show that they are “almost random” – the probability of error still drops exponentially in the number of times the verifier is simulated.

Let H be a constant degree expander on $n ^ { b }$ vertices, each vertex having a unique b log n bit label. A random walk on H of length $O ( \log n )$ can be constructed using only $O ( \log n )$ bits, b log n bits to pick the starting vertex at random and a constant number of bits to pick each successive vertex. (Observe that the random walk is started in the stationary distribution, which is uniform since the graph is regular.) The precise property of expanders we will need is the following.

Theorem 29.17 Let S be any set of vertices of H of size $< ( n ^ { b } ) / 2$ . There is a constant k such that

Pr[ all vertices of a k log n length random walk lie in $S \mid < { \frac { 1 } { n } }$

For intuitive justification for Theorem 29.17, observe that a constant fraction of the edges incident at vertices of S have their other end points in $\overline { S }$ these help the walk escape from S. The following figure shows a walk on H that does not lie in S:

![](images/6d813f7c267ad3a60006ba11ece404fec429acc4bc69d9576d40fa2012fdf099.jpg)  
NP = PCP<sub>1,</sub> 1 [log n, log n]

Theorem 29.18

Proof: We will prove the dificult half,

$$
\mathbf {P C P} _ {1, \frac {1}{2}} [ \log n, 1 ] \subseteq \mathbf {P C P} _ {1, \frac {1}{n}} [ \log n, \log n ],
$$

and leave the rest as Exercise 29.5. Let $L \in \mathbf { P C P } _ { 1 , \frac { 1 } { 2 } } [ \log n , 1 ]$ . Let $F$ be a verifier for L which requires b log n random bits and queries q bits of the proof, where b and q are constants.

Next, we give a $\mathbf { P C P } _ { 1 , \frac { 1 } { n } }$ [log n, log n] verifier for $L , F ^ { \prime }$ , which constructs the expander graph H defined above. It then constructs a random walk of length k log n on H, using $O ( \log n )$ random bits. Both constructions can be accomplished in polynomial time. The label of each vertex on this path specifies a b log n bit string. It uses these k log $n + 1$ strings as the “random” strings on which it simulates verifier $F , F ^ { \prime }$ accepts if $F$ accepts on all k log n+ 1 runs.

Consider string $x \in L$ , and let p be a proof that makes verifier $F$ accept x with probability 1. Clearly, $F ^ { \prime }$ , given proof p, also accepts x with probability 1. Hence the completeness of the new proof system is 1.

Next, consider string $x \notin L .$ , and let $p$ be an arbitrary proof supplied to $F ^ { \prime }$ When given proof $p ,$ verifier $F$ accepts on $< ( n ^ { b } ) / 2$ random strings of length b log n. Let $S$ denote the corresponding set of vertices of $H , | S | < ( n ^ { b } ) / 2$ . Now, $F ^ { \prime }$ accepts x if the random walk remains entirely in $S .$ . Since the probability of this event $\mathrm { i s } < 1 / n$ , the soundness of $F ^ { \prime }$ is $1 / n$ . Finally observe that $F ^ { \prime }$ requires only $O ( \log n )$ random bits and queries $O ( \log n )$ bits of the proof. ✷

Theorem 29.19 For fixed constants b and q, there is a gap-introducing reduction from SAT to clique that transforms a Boolean formula φ of size n to a graph $G = ( V , E )$ , where $| V | = n ^ { b + q }$ , such that

$i f \phi$ is satisfiable, $\mathrm { O P T } ( G ) \geq n ^ { b }$ , and

$i f \phi$ is not satisfiable, $\mathrm { O P T } ( G ) < n ^ { b - 1 }$

Proof: Let $F$ be a $\mathbf { P C P } _ { 1 , \frac { 1 } { n } }$ [log n, log n] verifier for SAT that requires b log n random bits and queries q log n bits of the proof. The transformation of SAT instance $\phi$ to graph G is exactly as in Lemma 29.16. The only diference is that the increased number of bits queried results in a larger number of vertices.

The correctness of the construction also along the lines of Lemma 29.16. If $\phi$ is satisfiable, let $p$ be a good proof, and pick the $n ^ { b }$ vertices of $G$ that are consistent with $p ,$ one for each choice of the random string. These vertices will form a clique in $G .$ Furthermore, any clique $C$ in G gives rise to a proof that is accepted by F with probabilit $\mathrm { y } \geq | C | / n ^ { b }$ . Since the soundness of $F$ is $1 / n$ , if $\phi$ is not satisfiable, the largest clique in G is of size $< n ^ { b - 1 }$ ✷

Corollary 29.20 There is no $1 / ( n ^ { \varepsilon _ { q } } )$ factor approximation algorithm for the cardinality clique problem, assuming $\mathbf { P } \neq \mathbf { N P }$ , where $\varepsilon _ { q } = 1 / ( b + q )$ , for constants b and $q$ defined in Theorem 29.19.

## 29.7 Hardness of set cover

As stated in Chapter 2, the simple greedy algorithm for the set cover problem, which is perhaps the first algorithmic idea one would attempt, has remained essentially the best algorithm. Since set cover is perhaps the single most important problem in the theory of approximation algorithms, a lot of efort was expended on obtaining an improved algorithm.

In this section, we will present the remarkable result that the approximation factor of this algorithm is tight up to a constant multiplicative factor. Improved hardness results show that it is tight up to lower order terms as well (see Section 29.9). This should put to rest nagging doubts about the true approximability of this central problem.

## 29.7.1 The two-prover one-round characterization of NP

Observe that for the purpose of showing hardness of MAX-3SAT and clique (Theorems 29.7 and 29.19), we did not require a detailed description of the kinds of queries made by the verifier – we only required a bound on the number of queries made. In contrast, this time we do need a description, and moreover, we want to first establish that a particularly simple verifier sufices. For this purpose, we will introduce a new model for probabilistically checkable proofs, the two-prover one-round proof system. This model is best understood by thinking of the proof system as a game between the prover and the verifier. The prover is trying to cheat – it is trying to convince the verifier that $\mathrm { ~ a ~ } ^ { 6 6 } \mathrm { n o } ^ { 5 5 }$ instance for language L is actually in L. Is there a verifier that can ensure that the probability of getting cheated $\mathrm { i s } < 1 / 2$ for every $" \mathrm { n o } ^ { \mathrm { 3 } \mathrm { 5 } }$ instance?

In the two-prover model, the verifier is allowed to query two noncommunicating provers, denoted $P _ { 1 }$ and $P _ { 2 }$ . Since the verifier can cross-check the provers’ answers, the provers’ ability to cheat gets restricted in this model. In turn, we will impose restrictions on the verifier as well, and thereby obtain a new characterization of NP. Under a one-round proof system, the verifier is allowed only one round of communication with each prover. The simplest way of formalizing this is as follows. We will assume that the two proofs are written in two alphabets, say $\Sigma _ { 1 }$ and $\Sigma _ { 2 }$ . In general, the sizes of these alphabets may be unbounded and may depend on the size of the input. The verifier is allowed to query one position in each of the two proofs.

The two-prover one-round model comes with three parameters: completeness, soundness and the number of random bits provided to the verifier, denoted by $^ { c , }$ s and $r ( n )$ , respectively. This defines the class $\mathbf { 2 P 1 R } _ { c , s } ( r ( n ) )$ ). A language L is in $\mathbf { 2 P 1 R } _ { c , s } ( r ( n ) )$ if there is a polynomial time bounded verifier V that receives $O ( r ( n ) )$ truly random bits and satisfies:

• for every input $x \in L$ , there is a pair of proofs $y _ { 1 } \in \Sigma _ { 1 } ^ { * }$ and $y _ { 2 } \in \Sigma _ { 2 } ^ { * }$ that makes V accept with probability $\geq c ,$

• for every input $x \notin L$ and every pair of proofs $y _ { 1 } \in \Sigma _ { 1 } ^ { * }$ and $y _ { 2 } \in \Sigma _ { 2 } ^ { * }$ , V accepts with probability $< s .$

The PCP theorem implies, and in fact is equivalent to, the fact that there is a gap-introducing reduction from SAT to MAX-3SAT(5) (see Theorem 29.7 and Exercises 29.3 and 29.4). We will use this to show:

Theorem 29.21 There is a constant $\varepsilon _ { P } > 0$ such that

$$
\mathbf {N P} = \mathbf {2 P 1 R} _ {1, 1 - \varepsilon_ {P}} (\log (n))
$$

Proof: We will establish the dificult half, i.e., $\mathbf { N P } \subseteq 2 \mathbf { P } \mathbf { 1 } \mathbf { R } _ { 1 , 1 - \varepsilon _ { P } } \left( \log ( n ) \right)$ • and leave the rest as Exercise 29.7. Clearly, it is suficient to show that SAT $\in \mathbf { 2 P 1 R } _ { 1 , 1 - \varepsilon _ { P } } ( \log ( n ) )$ .

As a result of Theorem 29.7 and Exercise 29.4, there is gap-introducing reduction from SAT to $\mathrm { M A X - 3 S A T ( 5 ) ^ { 2 } }$ . More precisely, there is a constant $\varepsilon _ { 5 } > 0$ for which there is a reduction $\varGamma$ from SAT to MAX-3SAT(5) that transforms a Boolean formula φ to ψ such that

• if $\phi$ is satisfiable, $\mathrm { O P T } ( \psi ) = m$ , and

• if $\phi$ is not satisfiable, $\mathrm { O P T } ( \psi ) < ( 1 - \varepsilon _ { 5 } ) m$ 2

where m is the number of clauses in $\psi .$

The two-prover one-round verifier, $V ,$ for SAT works as follows. Given a SAT formula $\phi ,$ it uses the above stated reduction to obtain a MAX-3SAT(5) instance $\psi .$ . It assumes that $P _ { 1 }$ contains an optimal truth assignment, $\tau ,$ for $\psi$ and $P _ { 2 }$ contains, for each clause, the assignment to its three Boolean variables under τ (hence, $| \Sigma _ { 1 } | = 2$ and $| \Sigma _ { 2 } | = 2 ^ { 3 } )$ . It uses the $O ( \log n )$ random bits to pick a random clause, $C$ , from ψ, and further, a random Boolean variable, $x ,$ occurring in $C .$ . V obtains the truth assignments to x and the three variables in $C$ by querying $P _ { 1 }$ and $P _ { 2 }$ , respectively. It accepts if $C$ is satisfied and the two proofs agree on their assignment for x.

If $\phi$ is satisfiable, then so is ψ. Clearly, there are proofs $y _ { 1 }$ and $y _ { 2 }$ such that $V$ accepts with probability 1.

Next assume that $\phi$ is not satisfiable. Any truth assignment to $\psi$ must leave strictly more than $\varepsilon _ { 5 }$ fraction of the clauses unsatisfied. Consider any pair of proofs $( y _ { 1 } , y _ { 2 } )$ . Interpret $y _ { 1 }$ as a truth assignment, say τ. The random clause, $C ,$ picked by V is not satisfied by τ with probability $> \varepsilon _ { 5 }$ . If $\mathrm { s o } ,$ and if the assignment for C contained in $y _ { 2 }$ is satisfying, then $y _ { 1 }$ and $y _ { 2 }$ must be inconsistent. In the latter case, the verifier catches this with probability $\geq 1 / 3$ . Hence overall, V must reject with probability $> \varepsilon _ { 5 } / 3$ ✷

Remark 29.22 Using standard techniques (see Exercise 29.8), Γ can be modified to ensure that the instance of MAX-3SAT(5) produced satisfies the following uniformity conditions: each Boolean variable occurs in exactly 5 clauses and each clause contains 3 distinct variables (negated or unnegated). This modification changes the constant $\varepsilon _ { 5 }$ to some other constant, say $\varepsilon _ { 5 } ^ { \prime } > 0$ These uniformity conditions will be needed in the main reduction.

Remark 29.23 As a result of the uniformity conditions, if ψ has n variables, then it has $5 n / 3$ clauses. Therefore, the two proofs are of length n and $5 n / 3$ respectively. For carrying out the main reduction, it will be important to ensure that the two proofs are of equal length. This can be easily achieved by repeating the first proof 5 times and the second proof 3 times. The verifier will query a random copy of each proof. It is easy to verify that Theorem 29.21 still holds (even though the “copies” may be diferent).

## 29.7.2 The gadget

The following set system will be a basic gadget in the main reduction: $( U , C _ { 1 } , \ldots , C _ { m } , \overline { { { C } } } _ { 1 } , \ldots , \overline { { { C } } } _ { m } )$ , where $U$ is the universal set and $C _ { 1 } , \ldots , C _ { m }$ are subsets of $U .$ . Clearly, $U$ can be covered by picking a set $C _ { i }$ and its complement $\overline { { C } } _ { i }$ . Such a cover will be called a good cover. A cover that does not include a set and its complement will be called a bad cover. The following theorem, which can be proven using the probabilistic method (see Exercise 29.9), shows the existence of such set systems for which the sizes of good and bad covers are widely diferent. Moreover, they can be constructed eficiently, with high probability.

Theorem 29.24 There exists a polynomial $p ( . , . )$ such that there is a randomized algorithm which generates, for each m and $l ,$ a set system

$$
(U, C _ {1}, \dots , C _ {m}, \overline {{C}} _ {1}, \dots , \overline {{C}} _ {m}),
$$

with $| U | = p ( m , 2 ^ { l } )$ . With probability $> 1 / 2$ the gadget produced satisfies that every bad cover is $o f$ size $> l .$ . Moreover, the running time of the algorithm is polynomial in $| U |$

A good cover is well coordinated – it involves picking a set $C _ { i }$ and its complement. Acceptance in the two-prover one-round proof system also involves coordination – on random string $r ,$ the verifier queries the two proofs and accepts if the answers are coordinated. The choice of this proof system, for establishing hardness of set cover, should be more convincing in light of this observation.

## 29.7.3 Reducing error probability by parallel repetition

Before presenting the reduction, we would like improve the soundness of the two-prover one-round proof system for SAT. The usual way of accomplishing this is parallel repetition: The verifier picks k clauses randomly and independently, and a random Boolean variable from each of the clauses. It queries $P _ { 1 }$ on the $k$ variables and $P _ { 2 }$ on the k clauses, and accepts if all answers are accepting. One would expect that probability that the provers manage to cheat drops to $< ( 1 - \varepsilon _ { P } ) ^ { k }$

Surprisingly enough, this is not true. Since each prover is allowed to look at all k questions before providing its k answers, it may be able to coordinate its answers and thereby cheat with a higher probability. Example 29.25 illustrates this in a simple setting. If the provers are required to answer each question before being given the next question, the probability of error drops in the usual fashion; however, this requires $k$ rounds of communication and falls outside the two-prover one-round model.

Example 29.25 Consider the following setting in which the two noncommunicating provers are attempting to agree on a random bit. The verifier gives random, independent bits $r _ { 1 }$ and $r _ { 2 }$ to $P _ { 1 }$ and $P _ { 2 } .$ , respectively. The protocol succeeds if the two provers manage to commit to one of the two bits, $\mathrm { i . e . }$ , either both provers output $( 1 , r _ { 1 } )$ or both provers output $( 2 , r _ { 2 } )$ ; the first element of a pair says whose bit the provers are outputting and the second element is the bit itself. Since $P _ { 1 }$ does not know $r _ { 2 }$ and $P _ { 2 }$ does not know $r _ { 1 }$ the probability of their succeeding is $1 / 2$

Now consider parallel repetitions of this protocol. The verifier gives two bits, $r _ { 1 }$ and $s _ { 1 } .$ , to $P _ { 1 }$ and two bits, $r _ { 2 }$ and $s _ { 2 } .$ , to $P _ { 2 }$ . The four bits are random and independent. The provers succeed if they can commit to one of the $r _ { \mathrm { } } \mathrm { { s } }$ and one of the $s \mathrm { { ^ { s } s } }$

One would expect the probability of success to be $1 / 4$ . However, by cleverly coordinating answers, the provers can make it $1 / 2$ as follows. The answers of $P _ { 1 }$ are $( 1 , r _ { 1 } )$ and $( 2 , r _ { 1 } )$ , and those of $P _ { 2 }$ are $( 1 , s _ { 2 } )$ and $( 2 , s _ { 2 } )$ . The provers succeed if $r _ { 1 } = s _ { 2 }$ , which happens with probability $1 / 2$ ✷

Despite this dificulty, one can still prove that the probability of error does drop exponentially with k. However, the proof of this useful fact is not easy.

Theorem 29.26 Let the error probability of a two-prover one-round proof system be $\delta < 1$ . Then the error probability on k parallel repetitions is at most $\delta ^ { d k }$ , where d is a constant that depends only on the length of the answers of the original proof system.

## 29.7.4 The reduction

We will prove the following.

Theorem 29.27 There is a constant $c > 0$ for which there is a randomized gap-introducing reduction $T ,$ requiring time $n ^ { O ( \log \log n ) }$ , from $S A T$ to the cardinality set cover problem that transforms a Boolean formula $\phi$ to a set system S over a universal set of size $n ^ { O ( \log \log n ) }$ such that

$i f \phi$ is satisfiable, $\mathrm { O P T } ( S ) = 2 n ^ { k }$ , and

$i f \phi$ is not satisfiable, $\mathbf { P r } [ \mathrm { O P T } ( G ) > c n ^ { k } k \log n ] > 1 / 2 ,$

where n is the length of each of the two proofs for SAT under the two-prover one-round model (see Remark 29.23); n is polynomial in the size of $\phi .$ . The parameter k is $O ( \log \log n )$

Remark 29.28 This is slight abuse of notation, since gap-introducing reductions were defined to run in polynomial time.

Proof: Let V be the two-prover one-round verifier for SAT, described in Theorem 29.21. Assume further that the MAX-3SAT(5) formula produced by V satisfies the uniformity conditions stated in Remark 29.22 and that the two proofs queried by $V$ are of equal length, say $n ,$ as stated in Remark 29.23. Denote by $\psi$ the MAX- $3 \mathrm { S A T ( 5 ) }$ formula produced by V when given SAT formula $\phi .$

Let $V ^ { \prime }$ be a two-prover one-round verifier that executes $k$ parallel repetitions of $V ,$ as described in Section 29.7.3. Now, each of the proofs is of length $n ^ { k }$ . Each position of $P _ { 1 }$ contains a truth assignment to $k$ Boolean variables (not necessarily distinct) and each position of $P _ { 2 }$ contains a truth assignment to the 3k Boolean variables occurring in $k$ clauses. Thus, proofs $P _ { 1 }$ and $P _ { 2 }$ are written in alphabets $\Sigma _ { 1 }$ and $\Sigma _ { 2 }$ whose sizes are $2 ^ { k }$ and $2 ^ { 3 k }$ , respectively. k will be fixed to be $O ( \log \log n )$ for reasons clarified below.

Verifier $V ^ { \prime }$ uses random bits provided to it to pick k random clauses of $\psi ,$ and a random Boolean variable from each of these k clauses, thereby specifying a position in $P _ { 1 }$ and a position in $P _ { 2 }$ . These involve picking from one of $n ^ { k }$ and $3 ^ { k }$ choices, respectively. Therefore, the total number of random strings is $( 3 n ) ^ { k }$ . Denote by $Q _ { 1 } ( r )$ and $Q _ { 2 } ( r )$ the positions in $P _ { 1 }$ and $P _ { 2 }$ respectively, specified by random string r.

Suppose the answers in positions $Q _ { 1 } ( r )$ and $Q _ { 2 } ( r )$ are a and $b ,$ respectively. Recall that $V ^ { \prime }$ accepts on random string r if b satisfies all k clauses picked, and a and b assign the same truth values to the k chosen variables. Given r and the answer in $Q _ { 2 } ( r )$ , say $b ,$ the “acceptable” answer in $Q _ { 1 } ( r )$ is uniquely specified. Let projection function $\pi ( r , b )$ denote this answer.

![](images/68cce1ea5e7070e45c598820b2a027d201d6a31be56b0d29c6d31262c211d4f0.jpg)

The parameters m and l for the gadget are fixed as follows. We will set $m = | \Sigma _ { 1 } | = 2 ^ { k }$ , and $l = O ( k \log n ) = O ( \log n$ log log n). Let $( U , C _ { 1 } , \dots , C _ { 2 ^ { k } } $ $\overline { { C } } _ { 1 } , \ldots , \overline { { C } } _ { 2 ^ { k } } )$ be the gadget with these parameters. Thus, corresponding to each answer $a \in \Sigma _ { 1 }$ , we have a unique set $C _ { a }$ . As stated in Theorem 29.24, $| U | = p ( 2 ^ { k } , 2 ^ { l } ) = n ^ { O ( \log \log n ) }$ , and the gadget can be constructed probabilistically in time polynomial in |U|.

The gadget will be constructed once, and as stated in Theorem 29.24, will satisfy the chosen parameters with probability $> 1 / 2$ . For the rest of the proof, assume that it does. We will make $( 3 n ) ^ { k }$ copies of the gadget over disjoint universal sets. Each copy corresponds to a random string. Denote the copy corresponding to random string r to be $( U ^ { r } , C _ { 1 } ^ { r } , \ldots , C _ { 2 ^ { k } } ^ { r } , \overleftarrow { C } _ { 1 } ^ { r } , \ldots , \overline { { C } } _ { 2 ^ { k } } ^ { r } )$

The reduction Γ transforms φ to a set cover instance $s$ as follows. The universal set

$$
\mathcal {U} = \bigcup_ {r} U ^ {r},
$$

$$
S _ {i, a} = \bigcup_ {r: Q _ {1} (r) = i} C _ {a} ^ {r},
$$

where the union is over all $( 3 n ) ^ { k }$ random strings. Clearly, $| \mathcal { U } | = | U | ( 3 n ) ^ { k }$ = $n ^ { O ( \log \log n ) }$ . The subsets of $\mathcal { U }$ specified by $s$ are of two kinds. First, corresponding to each position i in $P _ { 1 }$ and answer $a \in \Sigma _ { 1 }$ , there is a set

where the union is over all random strings r such that $Q _ { 1 } ( r ) = i$ . Second, corresponding to each position $j$ in $P _ { 2 }$ and answer $b \in \Sigma _ { 2 }$ , there is a set $S _ { j , b }$ . If b does not satisfy all $k$ clauses of $\psi _ { ; }$ specified by position $Q _ { 2 } ( r )$ , then $S _ { j , b } = \emptyset$ . Otherwise,

$$
S _ {j, b} = \bigcup_ {r: Q _ {2} (r) = j} \overline {{C}} _ {\pi (r, b)} ^ {r},
$$

where the union is over all random strings r such that $Q _ { 2 } ( r ) = j$

Let $r$ be a random string, and let $Q _ { 1 } ( r ) = i$ and $Q _ { 2 } ( r ) = j$ . Then, the only sets in $s$ that contain elements of $U ^ { r }$ are:

$S _ { i , a } ,$ for $a \in \Sigma _ { 1 }$ , and

$S _ { j , b }$ , for $b \in \Sigma _ { 2 }$ such that b satisfies the k clauses specified by position $j$ in $P _ { 2 }$

Moreover, each set of the first type contains exactly one set from $C _ { 1 } ^ { r } , \ldots , C _ { 2 ^ { k } } ^ { r }$ and each set of the second type contains exactly one set from $\overline { { C } } _ { 1 } ^ { r } , \ldots , \overline { { C } } _ { 2 ^ { k } } ^ { r }$

Let $r$ be a random string, and let $Q _ { 1 } ( r ) = i$ and $Q _ { 2 } ( r ) = j$ . Observe that $S _ { i , a } \cup S _ { j , b }$ covers $U ^ { r }$ if $\pi ( r , b ) = a$ and b satisfies the k clauses specified by position $j$ in $P _ { 2 }$ . Let $\mathcal { C }$ be a cover for U. If C contains such a pair of sets then we will say that $\mathcal { C }$ contains a good cover for $U ^ { r }$ . If C does not contain a good cover for $U ^ { r }$ , then it must contain $> l$ sets of the form $S _ { i , a } , S _ { j , b } , a \in \Sigma _ { 1 } , b \in \Sigma _ { 2 }$ in order to cover $U ^ { r }$ . In this case, we will say that $\mathcal { C }$ contains a bad cover for $U ^ { r }$

Suppose $\phi$ is satisfiable. Then there is a pair of proofs $( y _ { 1 } , y _ { 2 } )$ on which the verifier accepts with probability 1. Let us pick a cover C as follows. Corresponding to each position i in $P _ { 1 }$ and $j$ in $P _ { 2 }$ pick sets $S _ { i , a }$ and $S _ { j , b }$ , where a and b are the answers for these queries in $y _ { 1 }$ and $y _ { 2 } .$ , respectively. Hence, $| { \mathcal { C } } | = 2 n ^ { k }$ . It is easy to see that $\mathcal { C }$ contains a good cover for each set $U ^ { r }$

Next suppose that $\phi$ is not satisfiable. Now, $V ^ { \prime }$ will reject any pair of proofs with high probability. We have assumed that the gadget found satisfies the chosen parameters; this happens with probabili $\mathrm { { y } > 1 / 2 }$ . Let $\mathcal { C }$ denote an optimal cover for $\mathcal { U } .$ Is $\mathcal { C }$ forced to contain a bad cover for $U ^ { r }$ , for most random strings $r ?$ Clearly, C is allowed to pick sets corresponding to portions of many diferent proofs. Using this added capability, can we not construct a cover that is only slightly larger than $2 n ^ { k } ? \mathrm { ~ A ~ }$ set from $s$ helps cover elements from several diferent universes $U ^ { r }$ , making the rest of the argument more involved.

Below we will give a procedure for constructing, from ${ \mathcal { C } } ,$ a pair of proofs, $( y _ { 1 } , y _ { 2 } )$ , in such a way that if |C| is small, then $V ^ { \prime }$ must accept this pair with high probability. Hence, we will derive the desired lower bound on |C|.

Consider the set $o f$ answers picked by C for each position of the two proofs. For each position i in $P _ { 1 }$ , define $A ( i ) ~ = ~ \{ a ~ \mid ~ S _ { i , a } ~ \in ~ \mathcal { C } \}$ , and for each position $j$ in $P _ { 2 }$ , define $A ( j ) = \{ b \ \mid S _ { j , b } \in \mathcal { C } \}$ . Construct proofs $y _ { 1 }$ and $y _ { 2 }$ by picking for each position i in $P _ { 1 }$ and j in $P _ { 2 }$ a random element of $A ( i )$ and $A ( j )$ , respectively. If any of the answer sets is empty, pick an arbitrary answer for that position. Define $B _ { 1 } \ = \ \{ r \ | \ | A ( Q _ { 1 } ( r ) ) | \ > \ l / 2 \}$ • $B _ { 2 } = \{ r \mid | A ( Q _ { 2 } ( r ) ) | > l / 2 \}$ and $G = \overline { { B _ { 1 } \cup B _ { 2 } } }$

Thus, G is the set of random strings r for which C picks at most $l / 2$ answers each for $Q _ { 1 } ( r )$ and $Q _ { 2 } ( r )$ . Hence, C contains a good cover for $U ^ { r }$ say $S _ { i , a } \cup S _ { j , b }$ , where $a \in A ( Q _ { 1 } ( r ) )$ and $b \in A ( Q _ { 2 } ( r ) )$ . The pair of proofs, $( y _ { 1 } , y _ { 2 } )$ , contain a and b in positions $Q _ { 1 } ( r )$ and $Q _ { 2 } ( r )$ , respectively, with probability $\geq ( \frac { 2 } { l } ) ^ { 2 }$ . Hence $V ^ { \prime }$ , when given proofs $( y _ { 1 } , y _ { 2 } )$ , accepts on random string r with at least this probability.

Let $f _ { G }$ denote the fraction of random strings contained in $G .$ Then, using Theorem 29.26,

$$
f _ {G} \left(\frac {2}{l}\right) ^ {2} \leq \operatorname * {P r} \left[ V ^ {\prime} \text {   accepts   } \phi \text {   when   given   proofs   } (y _ {1}, y _ {2}) \right] \leq \delta^ {d k}.
$$

Hence, $f _ { G } \leq \delta ^ { d k } l ^ { 2 } / 4$ . Since $l ^ { 2 }$ is $O ( \log ^ { 4 } n )$ , by picking $k = O ( \log \log n )$ we can ensure that $f _ { G } < 1 / 2$ . As a result, $B _ { 1 } \cup B _ { 2 }$ contains at least half the random strings, and therefore one of these sets contains at least a quarter. Denote this set by $B _ { i }$

Because of the uniformity property (Remark 29.22), if r is chosen at random, then $Q _ { 1 } ( r )$ is a random position in $P _ { 1 }$ and $Q _ { 2 } ( r )$ is a random position in $P _ { 2 }$ (although they will be correlated). Furthermore, r has probability $> 1 / 4$ of being in $B _ { i }$ . Therefore, the answer sets of $> 1 / 4$ of the positions of $B _ { i }$ are of cardinality $> l / 2$ . Hence the size of the cover $> l n ^ { k } / 8 = \varOmega ( n ^ { k } k \log n )$ . ✷

As a consequence of Theorem 29.27, inapproximability of set cover modulo $\mathbf { N P }$ not being in a one-sided-error complexity class with running time $n ^ { O ( \log \log n ) }$ follows directly. Standard techniques from complexity theory (see Exercise 1.18) lead to the following slightly stronger result.

Corollary 29.29 There is a constant b such that if there is a b log n factor approximation algorithm for the cardinality set cover problem, where n is the size of the universal set of the set cover instance, then $\mathbf { N P } \subseteq$ $\mathbf { Z T I M E } ( n ^ { O ( \log \log n ) } )$ (see Section A.4 for definition).

## 29.8 Exercises

## 29.1 Show that $\mathbf { P C P } ( \log n , 1 ) \subseteq \mathbf { N P }$

Hint: Let ${ \cal L } \in { \bf P C P } ( \log n , 1 )$ . The NP machine for accepting L guesses the proof, simulates the verifier for L on all $O ( \log n )$ length random strings, and accepts if the verifier accepts on all the random strings.

29.2 Show (see Appendix A for definitions):

1. $\mathbf { P C P } ( 0 , 0 ) = \mathbf { P C P } ( 0 , \log n ) = \mathbf { P } .$

2. $\begin{array} { r } { { \bf P } { \bf C } { \bf P } ( p o l y ( n ) , 0 ) = \mathrm { c o } { \bf R } { \bf P } , } \end{array}$ where poly(n) = n<sup>k</sup>.

3. PCP $\begin{array} { r } { \mathbf { \Theta } ^ { * } ( \log n , 1 ) = \mathbf { P C P } ( \log n , \operatorname { p o l y } ) . } \end{array}$

Hint: $\mathbf { N P } \subseteq \mathbf { P C P } ( \log n , 1 ) \subseteq \mathbf { P C P } ( \log n , \operatorname { p o l y } ) \subseteq \mathbf { N P } .$

29.3 Show the converse of Theorem 29.7, i.e., if there is a gap-introducing reduction from SAT to MAX-3SAT, then $\mathbf { N P } \subseteq \mathbf { P C P } ( \log n , 1 )$

Hint: Reduce the given SAT formula $\phi$ to an instance $\psi$ of MAX-3SAT. The verifier expects, as proof, an optimal truth assignment to $\psi .$ . This gives an error probability of $1 - \varepsilon _ { M }$ . Repeat to decrease the error probability to $< 1 / 2$

29.4 Give a gap-preserving reduction from MAX-3SAT(29) to MAX-3SAT(5), with appropriate parameters, to show hardness for the latter problem.

Hint: The reduction is similar, though easier, than that in Theorem 29.11. Instead of using an expander graph, use a cycle. Now, an inconsistent assignment can gain as many as 14 clauses corresponding to each old variable x. However, it must leave at least two clauses, corresponding to edges of the cycle of $x ,$ unsatisfied.

## 29.5 Complete the proof of Theorem 29.18.

29.6 (Hastad [122]) An important consideration, while obtaining a PCP characterization of NP, is reducing the number of bits of the proof that the verifier needs to query. The following remarkable result reduces it to just 3.

Theorem 29.30 For every $\varepsilon > 0$

$$
\mathbf {N P} = \mathbf {P C P} _ {1 - \varepsilon , \frac {1}{2} + \varepsilon} [ \log n, 1 ].
$$

Moreover, there is a particularly simple PCP verifier for SAT. It uses the $O ( \log n )$ random bits to compute three positions in the proof, say $i , j$ and $k ,$ 4 and a bit $b ,$ and accepts $i f f$

$$
y (i) + y (j) + y (k) \equiv b \pmod {2}.
$$

Here $y ( i )$ is the $i ^ { t h }$ bit in the proof $y$ .

1. Consider the restriction of Problem 16.12 (Exercise 16.7), linear equations over GF[2], in which each equation has exactly 3 variables. Use the characterization stated in Theorem 29.30 to give an appropriate gapintroducing reduction from SAT to this problem which shows that if, for any $\varepsilon > 0$ , there is a $2 - \varepsilon$ factor approximation algorithm for the latter problem then $\mathbf { P } = \mathbf { N P }$

2. Give an appropriate gap-preserving reduction from linear equations over $\mathrm { G F } [ 2 ]$ to MAX-3SAT which shows that if, for any $\varepsilon > 0$ , there is a $8 / 7 - \varepsilon$ factor approximation algorithm for $\mathrm { M A X - 3 S A T }$ then $\mathbf { P } = \mathbf { N P }$ Hint: The equation $x _ { i } + x _ { j } + x _ { k } \equiv 0$ (mod 2) is transformed into the clauses

$$
\left(\overline {{x}} _ {i} \vee x _ {j} \vee x _ {k}\right) \wedge \left(x _ {i} \vee \overline {{x}} _ {j} \vee x _ {k}\right) \wedge \left(x _ {i} \vee x _ {j} \vee \overline {{x}} _ {k}\right) \wedge \left(\overline {{x}} _ {i} \vee \overline {{x}} _ {j} \vee \overline {{x}} _ {k}\right).
$$

29.7 Complete the proof of Theorem 29.21, i.e., show that $\mathbf { 2 P 1 R } _ { 1 , 1 - \varepsilon _ { P } } \left( \log ( n ) \right) \subseteq \mathbf { N P }$

29.8 Prove the uniformity conditions stated in Remark 29.22.

Hint: Use the standard technique of introducing new Boolean variables.

29.9 Prove Theorem 29.24 using the probabilistic method.

Hint: $p ( m , 2 ^ { l } ) = O ( m 2 ^ { 2 l } )$ sufices. Pick each set $C _ { i }$ by including each element of U in it randomly and independently with probability $1 / 2$

29.10 (Feige [80]) The following stronger hardness result can be established for set cover:

Theorem 29.31 For any constant $\delta > 0$ , if there is $a \ ( 1 - \delta )$ ln n factor approximation algorithm for the cardinality set cover problem, where n is the size of the universal set of the set cover instance, then $\mathbf { N P } \subseteq$ DTIM ${ \bf E } ( n ^ { O ( \log \log n ) } )$ , where DTIME(t) is the class of problems for which there is a deterministic algorithm running in time $O ( t )$

Consider the maximum coverage problem, Problem 2.18 in Exercise 2.15. Using Theorem 29.31 show that if there is an $\varepsilon > 0$ for which there is a $( 1 - 1 / e + \varepsilon )$ factor approximation algorithm for the maximum coverage problem, then $\mathbf { N P } \subseteq \tilde { \mathbf { D T I M E } } ( n ^ { O ( \log \log n ) } )$

Hint: Use the maximum coverage algorithm to obtain a $( 1 - \delta )$ ln n factor algorithm for set cover, for some $\delta > 0$ , as follows: Guess k, the optimal number of sets needed for the given instance. Run the maximum coverage algorithm, with parameter $k ,$ , iteratively, until a cover is found. In each iteration, $\mathrm { ~ a ~ } ( 1 - 1 / e + \varepsilon )$ fraction of the uncovered elements is covered. Therefore, the number of iterations, $l ,$ satisfies, $( 1 / e - \varepsilon ) ^ { l } = 1 / n .$

29.11 (Jain, Mahdian, and Saberi [138]) Using Theorem 29.31 show that if there is an $\varepsilon \ > \ 0$ for which there is a $( 1 + 2 / e - \varepsilon )$ factor approximation algorithm for the metric k-median problem, Problem 25.1, then $\mathbf { N P } \subseteq \mathbf { D T I M E } ( n ^ { O ( \log \log n ) } )$ .

## 29.9 Notes

The first hardness of approximation result based on probabilistically checkable proofs was due to Feige, Goldwasser, Lov´asz, Safra, and Szegedy [82]. This work motivated the discovery of the PCP theorem, which additionally builds on work on interactive proof systems (Babai [18] and Goldwasser, Micali, and Rackof [109]) and program checking (Blum and Kannan [28] and Blum, Luby, and Rubinfeld [29]), and is due to Arora and Safra [14], and Arora, Lund, Motwani, Sudan, and Szegedy [14, 12]. Theorem 29.30, which yields optimal inapproximability results for several problems, is due to Hastad [122].

Before this development, the pioneering work of Papadimitriou and Yannakakis [218] had established evidence of inapproximability of several natural problems using their notion of Max-SNP-completeness. Gap preserving reductions are weaker than their L-reductions. Consequently, the ideas behind their reductions carry over directly to the new development, as in the reductions given in Theorems 29.11 and 29.13. Indeed, one of the motivations for the PCP theorem was that establishing an inapproximability result for MAX SAT would directly yield inapproximability results for all Max-SNP-hard problems. Theorem 29.14 is from Bern and Plassmann [25].

The construction of expander graphs is due to Lubotzky, Phillips, and Sarnak [197]. Theorem 29.17 is due to Impagliazzo and Zuckerman [135]. Theorem 29.19 on hardness of clique follows from [82] and [14, 12]. The current best hardness result for clique, due to Hastad [121], states that it cannot be approximated within a factor of $n ^ { 1 - \varepsilon }$ for $\mathrm { a n y } \ \varepsilon > 0$ , unless $\mathbf { N P } = \mathbf { Z P P }$ This is quite close to the best approximation algorithm, due to Boppana and Holld´orsson [30], achieving a guarantee of $O ( n / ( \log ^ { 2 } n ) )$ .

Lund and Yannakakis [199] gave the first hardness result for set cover, showing that it cannot be approximated within a factor of log $n / 2$ unless $\mathbf { N P } \subseteq \mathbf { Z T I M E } \big ( n ^ { O ( \mathrm { p o l y l o g \ } n ) } \big )$ . The improved result, presented in Theorem 29.31, is due to Feige [80]. This enhancement comes about by using a k prover proof system. A deterministic construction of the set system gadget of Theorem 29.24, due to Naor, Schulman, and Srinivasan [211], allows replacing ZTIME by DTIME in the complexity assumption. The two-prover oneround proof system was defined by Ben-or, Goldwasser, Kilian, and Wigderson [24]. Theorem 29.26 is due to Raz [231].

Karlof and Zwick [162] give an algorithm for MAX-3SAT that achieves an approximation guarantee of $8 / 7$ when restricted to satisfiable formulae. This complements the hardness result stated in Exercise 29.6.

For further information on this topic, see the survey by Arora and Lund [11]. For an up-to-date status of the best positive and negative results known for numerous NP-hard optimization problems, see the excellent compendium maintained online at

http://www.nada.kth.se/˜viggo/problemlist/compendium.html

The compendium also appears in Ausiello, Crescenzi, Gambosi, Kann, Marchetti-Spaccamela, and Protasi [17].