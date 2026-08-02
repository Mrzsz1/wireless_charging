---
title: "approximation-algorithms-ch-26-part-027"
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
pdf_path: "work/core-books/approximation-algorithms/parts/approximation-algorithms-ch-26-part-027.pdf"
raw_md: "raw/canonical/approximation-algorithms-ch-26-part-027/full.md"
---
# 26 Semidefinite Programming

In the previous chapters of Part II of this book we have shown how linear programs provide a systematic way of placing a good upper bound on OPT (assuming a minimization problem), for numerous NP-hard problems. As stated earlier, this is a key step in the design of an approximation algorithm for an NP-hard problem. It is natural, then, to ask if there are other widely applicable ways of doing this.

In this chapter we provide another class of relaxations, called vector programs. These serve as relaxations for several NP-hard problems, in particular, for problems that can be expressed as strict quadratic programs (see Section 26.1 for a definition). Vector programs are equivalent to a powerful and well-studied generalization of linear programs, called semidefinite programs. Semidefinite programs, and consequently vector programs, can be solved within an additive error of $\varepsilon ,$ for any $\varepsilon > 0$ , in time polynomial in n and $\log ( 1 / \varepsilon )$ , using the ellipsoid algorithm (see Section 26.3).

We will illustrate the use of vector programs by deriving a 0.87856 factor algorithm for the following problem (see Exercises 2.1 and 16.6 for a factor $1 / 2$ algorithm).

Problem 26.1 (Maximum cut (MAX-CUT)) Given an undirected graph $G = ( V , E )$ , with edge weights $w : E \to \mathbf { Q } ^ { + }$ , find a partition (S, S) of $V$ so as to maximize the total weight of edges in this cut, i.e., edges that have one endpoint in $S$ and one endpoint in $\overline { S }$

## 26.1 Strict quadratic programs and vector programs

A quadratic program is the problem of optimizing (minimizing or maximizing) a quadratic function of integer valued variables, subject to quadratic constraints on these variables. If each monomial in the objective function, as well as in each of the constraints, is of degree 0 (i.e., is a constant) or $2 ,$ then we will say that this is a strict quadratic program.

Let us give a strict quadratic program for MAX-CUT. Let $y _ { i }$ be an indicator variable for vertex $v _ { i }$ which will be constrained to be either +1 or $- 1$ . The partition (S, S) will be defined as follows. $S = \{ v _ { i } \mid y _ { i } = 1 \}$ and $\overline { { S } } ~ = ~ \{ v _ { i } ~ \mid ~ y _ { i } ~ = ~ - 1 \}$ . If $v _ { i }$ and $v _ { j }$ are on opposite sides of this partition, then $y _ { i } y _ { j } = - 1$ , and edge $( v _ { i } , v _ { j } )$ contributes $w _ { i j }$ to the objective function. On the other hand, if they are on the same side, then $y _ { i } y _ { j } = 1$ , and edge $( v _ { i } , v _ { j } )$ makes no contribution. Hence, an optimal solution to this program is a maximum cut in $G .$

$$
\begin{array}{l l l} \text {maximize} & \frac {1}{2} \sum_ {1 \leq i <   j \leq n} w _ {i j} (1 - y _ {i} y _ {j}) \\ \text {subject to} & y _ {i} ^ {2} = 1, & v _ {i} \in V \\ & y _ {i} \in \mathbf {Z}, & v _ {i} \in V \end{array}\tag{26.1}
$$

We will relax this program to a vector program. A vector program is defined over n vector variables in $\mathbf { R } ^ { n }$ , say $\pmb { v } _ { 1 } , \ldots , \pmb { v } _ { n } .$ and is the problem of optimizing (minimizing or maximizing) a linear function of the inner products ${ \pmb v } _ { i } \cdot { \pmb v } _ { j } , 1 \le i \le j \le n$ , subject to linear constraints on these inner products. Thus, a vector program can be thought of as being obtained from a linear program by replacing each variable with an inner product of a pair of these vectors.

A strict quadratic program over n integer variables defines a vector program over n vector variables in $\mathbf { R } ^ { n }$ as follows. Establish a correspondence between the n integer variables and the n vector variables, and replace each degree 2 term with the corresponding inner product. For instance, the term y<sub>i</sub>y<sub>j</sub> in (26.1) is replaced with $v _ { i } \cdot v _ { j }$ . In this manner, we obtain the following vector program for MAX-CUT.

$$
\begin{array}{l l} \text { maximize } & \frac {1}{2} \sum_ {1 \leq i <   j \leq n} w _ {i j} (1 - \boldsymbol {v} _ {i} \cdot \boldsymbol {v} _ {j}) \\ \text { subject   to } & \boldsymbol {v} _ {i} \cdot \boldsymbol {v} _ {i} = 1, \quad v _ {i} \in V \\ & \boldsymbol {v} _ {i} \in \mathbf {R} ^ {n}, \quad v _ {i} \in V \end{array}\tag{26.2}
$$

Because of the constraint $\pmb { v } _ { i } \pmb { \cdot v } _ { i } = 1$ , the vectors $\pmb { v } _ { 1 } , \ldots , \pmb { v } _ { n }$ are constrained to lie on the n-dimensional sphere, $S _ { n - 1 }$ . Any feasible solution to (26.1) yields a solution to (26.2) having the same objective function value, by assigning the vector $( y _ { i } , 0 , \ldots , 0 )$ to ${ \mathbf { } } v _ { i }$ . (Notice that under this assignment, $\mathbf { } v _ { i } \cdot \mathbf { } v _ { j }$ is simply $y _ { i } y _ { j } . )$ Therefore, the vector program (26.2) is a relaxation of the strict quadratic program (26.1). Clearly, this holds in general as well; the vector program corresponding to a strict quadratic program is a relaxation of the quadratic program.

Interestingly enough, vector programs are approximable to any desired degree of accuracy in polynomial time, and thus relaxation (26.2) provides an upper bound on OPT for MAX-CUT. To show this, we need to recall some interesting and powerful properties of positive semidefinite matrices.

Remark 26.2 Vector programs do not always come about as relaxations of strict quadratic programs. Exercise 26.13 gives an NP-hard problem that has vector program relaxation; however, we do not know of a strict quadratic program for it.

## 26.2 Properties of positive semidefinite matrices

Let A be a real, symmetric n×n matrix. Then A has real eigenvalues and has n linearly independent eigenvectors (even if the eigenvalues are not distinct). We will say that A is positive semidefinite if

$$
\forall \boldsymbol {x} \in \mathbf {R} ^ {n}, \boldsymbol {x} ^ {T} \boldsymbol {A} \boldsymbol {x} \geq 0.
$$

We will use the following two equivalent conditions crucially. We provide a proof sketch for completeness.

Theorem 26.3 Let A be a real symmetric n×n matrix. Then, the following are equivalent:

1. $\forall x \in \mathbf { R } ^ { n } , \ x ^ { T } A x \geq 0 .$

2. All eigenvalues of A are nonnegative.

3. There is an $n \times n$ real matrix W, such that $\pmb { A } = \pmb { W } ^ { T } \pmb { W }$

Proof: $( 1 \Rightarrow 2 )$ : Let λ be an eigenvalue of A, and let v be a corresponding eigenvector. Therefore, $A v \ = \ \lambda v$ . Pre-multiplying by $v ^ { T }$ we get ${ \pmb v } ^ { T } { \pmb A } { \pmb v } =$ $\lambda \boldsymbol { v } ^ { T } \boldsymbol { v }$ . Now, by (1), ${ \pmb v } ^ { T } { \pmb A } { \pmb v } \geq 0$ . Therefore, $\lambda \bar { v } ^ { T } v \ge 0$ . Since $\pmb { v } ^ { T } \pmb { v } > 0 , \lambda \geq 0$ $( 2 \Rightarrow 3 ) \Rightarrow { }$ : Let $\lambda _ { 1 } , \ldots , \lambda _ { n }$ be the n eigenvalues of A, and $\pmb { v } _ { 1 } , \ldots , \pmb { v } _ { n }$ be the corresponding complete collection of orthonormal eigenvectors. Let Q be the matrix whose columns are $\pmb { v } _ { 1 } , \ldots , \pmb { v } _ { n }$ , and Λ be the diagonal matrix with entries $\lambda _ { 1 } , \ldots , \lambda _ { n }$ . Since for each $i , A { \pmb v } _ { i } = \lambda _ { i } { \pmb v } _ { i }$ , we have $A \mathbf { Q } = \mathbf { Q } A$ . Since Q is orthogonal, i.e., $\mathbf { Q Q } ^ { T } = I { \boldsymbol { \mathbf { \mathit { a } } } } $ , we get that $\mathbf { Q } ^ { T } = \mathbf { Q } ^ { - 1 }$ . Therefore,

$$
\boldsymbol {A} = \mathbf {Q} \boldsymbol {\Lambda} \mathbf {Q} ^ {T}.
$$

Let D be the diagonal matrix whose diagonal entries are the positive square roots of $\lambda _ { 1 } , \ldots , \lambda _ { n } { \mathrm { ~ ( b y ~ ( 2 ) , ~ } } \lambda _ { 1 } , \ldots , \lambda _ { n }$ are nonnegative, and thus their square roots are real). Then, $\pmb { \varLambda } = \pmb { D } \pmb { D } ^ { T }$ . Substituting, we get

$$
\boldsymbol {A} = \mathbf {Q} \boldsymbol {D} \boldsymbol {D} ^ {T} \mathbf {Q} ^ {T} = (\mathbf {Q} \boldsymbol {D}) (\mathbf {Q} \boldsymbol {D}) ^ {T}.
$$

Now, (3) follows by letting $W = ( \mathbf { Q } D ) ^ { T }$

$( 3 \Rightarrow 1 )$ : For any

$$
\boldsymbol {x} \in \mathbf {R} ^ {n}, \boldsymbol {x} ^ {T} \boldsymbol {A} \boldsymbol {x} = \boldsymbol {x} ^ {T} \boldsymbol {W} ^ {T} \boldsymbol {W} \boldsymbol {x} = (\boldsymbol {W} \boldsymbol {x}) ^ {T} (\boldsymbol {W} \boldsymbol {x}) \geq 0.
$$

Using Cholesky decomposition (see Section $2 6 . 7 )$ , a real symmetric matrix can be decomposed, in polynomial time, as $\mathbf { A } = \pmb { U } \mathbf { A } \pmb { U } ^ { T }$ , where Λ is a diagonal matrix whose diagonal entries are the eigenvalues of A. Now A is positive semidefinite if all the entries of Λ are nonnegative, thus giving a polynomial time test for positive semidefiniteness. The decomposition $W W ^ { T }$ is not polynomial time computable because in general it may contain irrational entries. However, it can be approximated to any desired degree by approximating the square roots of the entries of Λ. In the rest of this chapter we will assume that we have an exact decomposition, since the inaccuracy resulting from an approximate decomposition can be absorbed into the approximation factor (see Exercise 26.6).

It is easy to see that the sum of two $n \times n$ positive semidefinite matrices is also positive semidefinite (e.g., using characterization (1) of Theorem 26.3). This is also true of any convex combination of such matrices.

## 26.3 The semidefinite programming problem

Let Y be an $n \times n$ matrix of real valued variables whose $( i , j ) \mathrm { t h }$ entry is $y _ { i j }$ The problem of maximizing a linear function of the $y _ { i j } \mathrm { \ ' s }$ , subject to linear constraints on them, and the additional constraint that Y be symmetric and positive semidefinite, is called the semidefinite programming problem.

Let us introduce some notation to state this formally. Denote by R<sup>n×n</sup> the space of $n \times n$ real matrices. Recall that the trace of a matrix $\pmb { A } \in \mathbf { R } ^ { n \times n }$ is the sum of its diagonal entries and is denoted by $\operatorname { t r } ( A )$ . The Frobenius inner product of matrices $\pmb { A } , \pmb { B } \in \mathbf { R } ^ { n \times n }$ , denoted $\mathbfcal { A } \bullet \mathbfcal { B }$ , is defined to be

$$
\boldsymbol {A} \bullet \boldsymbol {B} = \operatorname{tr} (\boldsymbol {A} ^ {T} \boldsymbol {B}) = \sum_ {i = 1} ^ {n} \sum_ {j = 1} ^ {n} a _ {i j} b _ {i j},
$$

where $a _ { i j }$ and $b _ { i j }$ are the $( i , j )$ th entries of A and B, respectively. Let $M _ { n }$ denote the cone of symmetric n×n real matrices. For $A \in M _ { n } , A \succeq 0$ denotes the fact that matrix A is positive semidefinite.

Let $C , D _ { 1 } , \ldots , D _ { k } \in M _ { n }$ and $d _ { 1 } , \dotsc . . . d _ { k } \in \mathbf { R }$ . Following is a statement of the general semidefinite programming problem. Let us denote it by S.

maximize C • Y

$$
\begin{array}{l l} \text { subject   to } & D _ {i} \bullet Y = d _ {i}, \quad 1 \leq i \leq k \\ & Y \succeq 0, \\ & Y \in M _ {n}. \end{array}\tag{26.3}
$$

Observe that if $C , D _ { 1 } , \ldots , D _ { k }$ are all diagonal matrices, this is simply a linear programming problem. As in the case of linear programs, it is easy to see that allowing linear inequalities, in addition to equalities, does not make the problem more general.

Let us call a matrix in $\mathbf { R } ^ { n \times n }$ satisfying all the constraints of S a feasible solution. Since a convex combination of positive semidefinite matrices is positive semidefinite, it is easy to see that the set of feasible solutions is convex, i.e., if $\pmb { A } \in \mathbf { R } ^ { n \times n }$ and $\ b { B } \in \mathbf { R } ^ { n \times n }$ are feasible solutions then so is any convex combination of these solutions.

Let $\ b { A } \in \ b { \mathrm { \bf ~ R } } ^ { n \times n }$ be an infeasible point. Let $C \in \mathbf { R } ^ { n \times n }$ . A hyperplane $C \bullet Y \leq b$ is called a separating hyperplane for A if all feasible points satisfy it and point A does not satisfy it. In the next theorem we show how to find a separating hyperplane in polynomial time. As a consequence, for any $\varepsilon > 0$ semidefinite programs can be solved within an additive error of $\varepsilon _ { i }$ in time polynomial in n and $\log ( 1 / \varepsilon )$ , using the ellipsoid algorithm (see Section 26.7 for more eficient methods).

Theorem 26.4 Let S be a semidefinite programming problem, and A be a point in $\mathbf { R } ^ { n \times n }$ . We can determine, in polynomial time, whether A is feasible for S and, if it is not, find a separating hyperplane.

Proof: Testing for feasibility involves ensuring that A is symmetric and positive semidefinite and that it satisfies all the linear constraints. By remarks made in Section 26.2, this can be done in polynomial time. If A is infeasible, a separating hyperplane is obtained as follows.

• If A is not symmetric, $a _ { i j } > a _ { j i }$ for some $i , j$ . Then $y _ { i j } \le y _ { j i }$ is a separating hyperplane.

• If A is not positive semidefinite, then it has a negative eigenvalue, say λ. Let v be the corresponding eigenvector. Now $( { \pmb v } { \pmb v } ^ { \hat { T } } ) { \bullet \pmb Y } = \bar { { \pmb v } ^ { T } } \pmb Y { \pmb v } \geq 0$ is a separating hyperplane.

• If any of the linear constraints is violated, it directly yields a separating hyperplane.

Next, let us show that vector programs are equivalent to semidefinite programs, thereby showing that the former can be solved eficiently to any desired degree of accuracy. Let V be a vector program on n n-dimensional vector variables $\pmb { v } _ { 1 } , \ldots , \pmb { v } _ { n }$ . Define the corresponding semidefinite program, $s ,$ over $n ^ { 2 }$ variables $y _ { i j } , 1 \leq i , j \leq n .$ as follows. Replace each inner product ${ \mathbf { } } v _ { i } { \cdot } v _ { j }$ occurring in V by the variable $y _ { i j }$ . The objective function and constraints are now linear in the $y _ { i j } \mathrm { \ ' s }$ . Additionally, require that matrix $\mathbf { Y }$ , whose $( i , j )$ th entry is $y _ { i j }$ , be symmetric and positive semidefinite.

Lemma 26.5 Vector program V is equivalent to semidefinite program ${ \mathcal { S } } .$

Proof: We will show that corresponding to each feasible solution to V, there is a feasible solution to $s$ of the same objective function value, and vice versa. Let $\pmb { a } _ { 1 } , \ldots , \pmb { a } _ { n }$ be a feasible solution to V. Let W be the matrix whose columns are $\pmb { a } _ { 1 } , \ldots , \pmb { a } _ { n }$ . Then, it is easy to see that $\pmb { A } = \pmb { W } ^ { T } \pmb { W }$ is a feasible solution to $s$ having the same objective function value.

For the other direction, let A be a feasible solution to S. $\mathrm { B y }$ Theorem 26.3, there is an $n \times n$ matrix W such that $\pmb { A } = \pmb { W } ^ { T } \pmb { W }$ . Let $\pmb { a } _ { 1 } , \ldots , \pmb { a } _ { n }$ be the columns of W. Then, it is easy to see that $\pmb { a } _ { 1 } , \ldots , \pmb { a } _ { n }$ is a feasible solution to V having the same objective function value. ✷

Finally, we give the semidefinite programming relaxation to MAX-CUT that is equivalent to vector program 26.2.

$$
\begin{array}{l l} \text {maximize} & \frac {1}{2} \sum_ {1 \leq i <   j \leq n} w _ {i j} (1 - y _ {i} y _ {j}) \\ \text {subject to} & y _ {i} ^ {2} = 1, \qquad v _ {i} \in V \\ & \boldsymbol {Y} \succeq 0, \\ & \boldsymbol {Y} \in M _ {n}. \end{array}\tag{26.4}
$$

## 26.4 Randomized rounding algorithm

We now present the algorithm for MAX-CUT. For convenience, let us assume that we have an optimal solution to the vector program (26.2). The slight inaccuracy in solving it can be absorbed into the approximation factor (see Exercise 26.6). Let $\mathbf { } a _ { 1 } , \ldots , \mathbf { } a _ { n }$ be an optimal solution, and let $\mathrm { O P T } _ { v }$ denote its objective function value. These vectors lie on the n-dimensional unit sphere $S _ { n - 1 }$ . We need to obtain a cut $( S , { \overline { { S } } } )$ whose weight is a large fraction of $\mathrm { O P T } _ { v }$

Let $\theta _ { i j }$ denote the angle between vectors $\mathbf { a } _ { i }$ and ${ \pmb a } _ { j }$ . The contribution of this pair of vectors to $\mathrm { O P T } _ { v }$ is

$$
\frac {w _ {i j}}{2} (1 - \cos \theta_ {i j}).
$$

Clearly, the closer $\theta _ { i j }$ is to $\pi _ { \mathrm { : } }$ , the larger this contribution will be. In turn, we would like vertices $v _ { i }$ and $v _ { j }$ to be separated if $\theta _ { i j }$ is large. The following method accomplishes precisely this. Pick r to be a uniformly distributed vector on the unit sphere $S _ { n - 1 }$ , and let $S = \{ v _ { i } \mid { \pmb a } _ { i } \cdot { \pmb r } \geq 0 \}$

Lemma 26.6

$$
\mathbf {P r} [ v _ {i} a n d v _ {j} a r e s e p a r a t e d ] = \frac {\theta_ {i j}}{\pi}.
$$

Proof: Project r onto the plane containing ${ \mathbf { } } v _ { i }$ and ${ \pmb v } _ { j }$ . Now, vertices $v _ { i }$ and $v _ { j }$ will be separated if the projection lies in one of the two arcs of angle $\theta _ { i j }$ shown below.

![](images/72db202b59f128d43a32721ed053bdff6ebad14f07ec6c4423dd2cdb7a8c2281.jpg)

Since r has been picked from a spherically symmetric distribution, its projection will be a random direction on this plane. The lemma follows. ✷

The next lemma shows how to generate vectors that are uniformly distributed on the unit sphere $S _ { n - 1 }$

Lemma 26.7 Let $x _ { 1 } , \ldots , x _ { n }$ be picked independently from the normal distribution with mean 0 and unit standard deviation. Let $\dot { d } = ( x _ { 1 } ^ { 2 } + \ldots + x _ { n } ^ { 2 } ) ^ { 1 / 2 }$ Then, $( x _ { 1 } / d , \ldots , x _ { n } / d )$ is a random vector on the unit sphere $S _ { n - 1 }$

Proof: Consider the vector $\pmb { r } = ( x _ { 1 } , \dots , x _ { n } )$ . The distribution function for r has density

$$
f (y _ {1}, \ldots , y _ {n}) = \prod_ {i = 1} ^ {n} \frac {1}{\sqrt {2 \pi}} e ^ {- y _ {i} ^ {2} / 2} = \frac {1}{(2 \pi) ^ {n / 2}} e ^ {- \frac {1}{2} \sum_ {i} y _ {i} ^ {2}}.
$$

Notice that the density function depends only on the distance of the point from the origin. Therefore, the distribution of $\pmb { r }$ is spherically symmetric. Hence, dividing by the length of $r , \mathrm { i . e . , } d ,$ we get a random vector on $S _ { n - 1 }$

The algorithm is summarized below.

<div class="mineru-algorithm" style="white-space: pre-wrap; font-family:monospace;">
Algorithm 26.8 (MAX-CUT)  
1. Solve vector program (26.2). Let $\pmb{a}_1, \dots, \pmb{a}_n$ be an optimal solution.  
2. Pick $\pmb{r}$ to be a uniformly distributed vector on the unit sphere $S_{n-1}$.  
3. Let $S = \{v_i \mid \pmb{a}_i \cdot \pmb{r} \geq 0\}$.
</div>

Let W be the random variable denoting the weight of edges in the cut picked by Algorithm 26.8, and let

$$
\alpha = \frac {2}{\pi} \min _ {0 \leq \theta \leq \pi} \frac {\theta}{1 - \cos \theta}.
$$

One can show that $\alpha > 0 . 8 7 8 5 6$ (see Exercise 26.3).

Lemma 26.9 $\mathbf { E } [ W ] \geq \alpha \cdot \mathrm { O P T } _ { v }$

Proof: By the definition of α we have that for any θ, $0 \leq \theta \leq \pi$ 2

$$
\frac {\theta}{\pi} \geq \alpha \left(\frac {1 - \cos \theta}{2}\right).\tag{26.5}
$$

Using this and Lemma 26.6, we get

$$
\begin{array}{l} \mathbf {E} [ W ] = \sum_ {1 \leq i <   j \leq n} w _ {i j} \mathbf {P r} [ \boldsymbol {v} _ {i} \text {and} \boldsymbol {v} _ {j} \text {are separated} ] \\ = \sum_ {1 \leq i <   j \leq n} w _ {i j} \frac {\theta_ {i j}}{\pi} \geq \alpha \cdot \sum_ {1 \leq i <   j \leq n} \frac {1}{2} w _ {i j} (1 - \cos \theta_ {i j}) = \alpha \cdot \mathrm{OPT} _ {v}. \end{array}
$$

Let us define the integrality gap for relaxation (26.2) to be

$$
\inf _ {I} \frac {\operatorname{OPT} (I)}{\operatorname{OPT} _ {v} (I)},
$$

where the infimum is over all instances I of MAX-CUT.

Corollary 26.10 The integrality gap for relaxation $( 2 6 . 2 )$ is at least $\alpha \mathrm { ~ > ~ }$ 0.87856.

Theorem 26.11 There is a randomized approximation algorithm for MAX-CUT achieving an approximation factor of 0.87856.

Proof: Let us first obtain a “high probability” statement using the bound on expectation established in Lemma 26.9. Let T denote the sum of weights of all edges in $G$ , and define a so that $\mathbf { E } [ W ] = a T$ . Let

$$
p = \mathbf {P r} [ W <   (1 - \varepsilon) a T ],
$$

where $\varepsilon > 0$ is a constant. Since the random variable W is always bounded by $T _ { \cdot }$ , we get

$$
a T \leq p (1 - \varepsilon) a T + (1 - p) T.
$$

Therefore,

$$
p \leq \frac {1 - a}{1 - a + a \varepsilon}.
$$

Now,

$$
T \geq \mathbf {E} [ W ] = a T \geq \alpha \cdot \mathrm{OPT} _ {v} \geq \alpha \cdot \mathrm{OPT} \geq \frac {\alpha T}{2},
$$

where the last inequality follows from the fact that $\mathrm { O P T } \geq T / 2$ (see Exercise 2.1). Therefore, $1 \ge a \ge \alpha / 2$ . Using this upper and lower bound on a, we get

$$
p \leq 1 - \frac {\varepsilon \alpha / 2}{1 + \varepsilon - \alpha / 2} \leq 1 - c,
$$

where

$$
c = \frac {\varepsilon \alpha / 2}{1 + \varepsilon - \alpha / 2}.
$$

Run Algorithm 26.8 1/c times, and output the heaviest cut found in these runs. Let $W ^ { \prime }$ be the weight of this cut. Then,

$$
\mathbf {P r} [ W ^ {\prime} \geq (1 - \varepsilon) a T ] \geq 1 - (1 - c) ^ {1 / c} \geq 1 - \frac {1}{e}.
$$

Since $a T \ge \alpha \cdot \mathrm { O P T } > 0 . 8 7 8 5 6 ~ \mathrm { O P T }$ , we can pick a value of $\varepsilon > 0$ so that $( 1 - \varepsilon ) a T \ge 0 . 8 7 8 5 6 ~ \mathrm { O P T }$ ✷

Example 26.12 The following example shows that the bound on the integrality gap of relaxation (26.2) given in Corollary 26.10 is almost tight. Consider a graph which is a 5-cycle $v _ { 1 } , v _ { 2 } , v _ { 3 } , v _ { 4 } , v _ { 5 } , v _ { 1 }$ . Then, an optimal solution to relaxation (26.2) is to place the five vectors in a 2-dimensional subspace within which they are given by $\pmb { v } _ { i } = ( \cos ( \frac { 4 i \pi } { 5 } )$ , sin $\left( { \frac { 4 i \pi } { 5 } } \right) )$ ), for $1 \leq i \leq 5$ (see Exercise 26.5). The cost of this solution is $\begin{array} { r } { \mathrm { O P T } _ { v } = \frac { 5 } { 2 } ( 1 + \cos \frac { \pi } { 5 } ) = \frac { 2 5 + 5 \sqrt { 5 } } { 8 } } \end{array}$ Since $\mathrm { { O P T } = 4 }$ for this graph, the integrality gap for this example is $\textstyle { \frac { 3 2 } { 2 5 + 5 { \sqrt { 5 } } } } = 0 . 8 8 4 4 5 . . .$ ✷

## 26.5 Improving the guarantee for MAX-2SAT

MAX-2SAT is the restriction of MAX-SAT (Problem 16.1) to formulae in which each clause contains at most two literals. In Chapter 16 we obtained a factor $3 / 4$ algorithm for this problem using randomization, followed by the method of conditional expectation. We will give an improved algorithm using semidefinite programming.

The key new idea needed is a way of converting the obvious quadratic program (see Exercise 26.8) for this problem into a strict quadratic program. We will accomplish this as follows. Corresponding to each Boolean variable $x _ { i } ,$ , introduce variable $y _ { i }$ which is constrained to be either $+ 1 \mathrm { o r } \mathrm { - } 1$ , for $1 \leq i \leq n$ . In addition, introduce another variable, say $y _ { 0 } ,$ which is also constrained to be +1 or −1. Let us impose the convention that Boolean variable $x _ { i }$ is true if $y _ { i } = y _ { 0 }$ and false otherwise. Under this convention we can write the value of a clause in terms of the $y _ { i } \mathrm { \dot { s } }$ , where the value, $v ( C )$ , of clause C is defined to be 1 if $C$ is satisfied and 0 otherwise. Thus, for clauses containing only one literal,

$$
v (x _ {i}) = \frac {1 + y _ {0} y _ {i}}{2} \mathrm{and} v (\overline {{x _ {i}}}) = \frac {1 - y _ {0} y _ {i}}{2}.
$$

Consider a clause containing 2 literals, $\mathrm { e . g . } , ( x _ { i } \vee x _ { j } )$ . Its value is

$$
\begin{array}{c} v (x _ {i} \vee x _ {j}) = 1 - v (\overline {{x _ {i}}}) v (\overline {{x _ {j}}}) = 1 - \frac {1 - y _ {0} y _ {i}}{2} \frac {1 - y _ {0} y _ {j}}{2} \\ = \frac {1}{4} \left(3 + y _ {0} y _ {i} + y _ {0} y _ {j} - y _ {0} ^ {2} y _ {i} y _ {j}\right) \\ = \frac {1 + y _ {0} y _ {i}}{4} + \frac {1 + y _ {0} y _ {j}}{4} + \frac {1 - y _ {i} y _ {j}}{4}. \end{array}
$$

Observe that in this derivation we have used the fact that $y _ { 0 } ^ { 2 } = 1$ . In all the remaining cases as well, it is easy to check that the value of a 2 literal clause consists of a linear combination of terms of the form $( 1 + y _ { i } y _ { j } )$ or $( 1 - y _ { i } y _ { j } )$ . Therefore, a MAX-2SAT instance can be written as the following strict quadratic program, where the $a _ { i j }$ ’s and $b _ { i j } ` { : }$ s are computed by collecting terms appropriately.

$$
\begin{array}{l l} \text {maximize} & \sum_ {0 \leq i <   j \leq n} a _ {i j} (1 + y _ {i} y _ {j}) + b _ {i j} (1 - y _ {i} y _ {j}) \\ \text {subject to} & y _ {i} ^ {2} = 1, \quad 0 \leq i \leq n \\ & y _ {i} \in \mathbf {Z}, \quad 0 \leq i \leq n \end{array}\tag{26.6}
$$

Following is the vector program relaxation for (26.6), where vector variable ${ \mathbf { } } v _ { i }$ corresponds to $y _ { i }$

$$
\begin{array}{l l} \text { maximize } & \sum_ {0 \leq i <   j \leq n} a _ {i j} (1 + \boldsymbol {v} _ {i} \cdot \boldsymbol {v} _ {j}) + b _ {i j} (1 - \boldsymbol {v} _ {i} \cdot \boldsymbol {v} _ {j}) \\ \text { subject   to } & \boldsymbol {v} _ {i} \cdot \boldsymbol {v} _ {i} = 1, \\ & \boldsymbol {v} _ {i} \in \mathbf {R} ^ {n + 1}, \end{array} \quad \begin{array}{l l} (2) \\ 0 \leq i \leq n \\ 0 \leq i \leq n \end{array}\tag{26.7}
$$

The algorithm is similar to that for MAX-CUT. We solve vector program (26.7). Let $\pmb { a } _ { 0 } , \ldots , \pmb { a } _ { n }$ be an optimal solution. Pick a vector r uniformly distributed on the unit sphere in $( n + 1 )$ dimensions, $S _ { n } .$ , and let $y _ { i } = 1$ if $\pmb { r } \cdot { \pmb { a } } _ { i } \geq 0$ , for $0 \leq i \leq n$ . This gives a truth assignment for the Boolean variables. Let W be the random variable denoting the weight of this truth assignment.

Lemma 26.13 $\mathbf { E } [ W ] \geq \alpha \cdot \mathrm { O P T } _ { v }$

Proof:

$$
\mathbf {E} [ W ] = 2 \sum_ {0 \leq i <   j \leq n} a _ {i j} \mathbf {P r} [ y _ {i} = y _ {j} ] + b _ {i j} \mathbf {P r} [ y _ {i} \neq y _ {j} ].
$$

Let $\theta _ { i j }$ denote the angle between $\mathbf { a } _ { i }$ and $\mathbf { \delta } _ { \mathbf { \alpha } \mathbf { \beta } _ { \mathcal { I } } } ^ { \mathbf { \alpha } \mathbf { \alpha } \mathbf { \alpha } \mathbf { \alpha } \mathbf { \alpha } \mathbf { \alpha } \mathbf { \alpha } \mathbf { \alpha } }$ . By inequality (26.5),

$$
\mathbf {P r} [ y _ {i} \neq y _ {j} ] = \frac {\theta_ {i j}}{\pi} \geq \frac {\alpha}{2} (1 - \cos \theta_ {i j}).
$$

By Exercise 26.4,

$$
\mathbf {P r} [ y _ {i} = y _ {j} ] = 1 - \frac {\theta_ {i j}}{\pi} \geq \frac {\alpha}{2} (1 + \cos \theta_ {i j}).
$$

Therefore,

$$
\mathbf {E} [ W ] \geq \alpha \cdot \sum_ {0 \leq i <   j \leq n} a _ {i j} (1 + \cos \theta_ {i j}) + b _ {i j} (1 - \cos \theta_ {i j}) = \alpha \cdot \mathrm{OPT} _ {v}.
$$

## 26.6 Exercises

26.1 Is matrix W in Theorem 26.3 unique (up to multiplication $\mathrm { b y - 1 ) ? }$ Hint: Consider the matrix $\mathbf { Q } D \mathbf { Q } ^ { T }$

26.2 Let B be obtained from matrix A by throwing away a set of columns and the corresponding set of rows. We will say that B is a principal submatrix of A. Show that the following is another equivalent condition for a real symmetric matrix to be positive semidefinite: that all of its principal submatrices have nonnegative determinants. (See Theorem 26.3 for other conditions.)

26.3 Show, using elementary calculus, that $\alpha > 0 . 8 7 8 5 6$

26.4 Show that for any φ, $0 \le \phi \le \pi$

$$
1 - \frac {\phi}{\pi} \geq \frac {\alpha}{2} (1 + \cos \phi).
$$

## 26 Semidefinite Programming

Hint: Substitute $\theta = \pi - \phi$ in inequality (26.5).

26.5 Show that for a 5-cycle, the solution given in Example 26.12 is indeed an optimal solution to the vector program relaxation for MAX-CUT.

26.6 Show that the inaccuracies resulting from the fact we do not have an optimal solution to the vector program (26.2) and that matrix A is not exactly decomposed as $W W ^ { T }$ (see end of Section 26.2) can be absorbed into the approximation factor for MAX-CUT.

Hint: Use the idea behind the proof of Theorem 26.11 and the fact that the solution to program (26.2) lies in the range [T/2, T], where T is the sum of weights of all edges in G.

26.7 Theorem 26.11 shows how to obtain a “high probability” statement from Lemma 26.9. Obtain a similar statement for MAX-2SAT, using Lemma 26.13, thereby obtaining a 0.87856 factor algorithm for MAX-2SAT.

## 26.8 Give a quadratic program for MAX-2SAT.

26.9 (Linial, London, and Rabinovich [190]) Let G be the complete undirected graph on n vertices, V , and let w be a function assigning nonnegative weights to the edges of G. The object is to find an optimal distortion $\ell _ { 2 } ^ { 2 } -$ embedding of the vertices of G. Let vertex i be mapped to $\pmb { v } _ { i } \in \mathbf { R } ^ { n }$ by such an embedding. The embedding should satisfy:

1. no edge is overstretched, i.e., for $1 \leq i < j \leq n , | | \pmb { v } _ { i } - \pmb { v } _ { j } | | ^ { 2 } \leq w _ { ( } i j )$ , and 2. the maximum shrinkage is minimized, i.e.,

$$
\text { maximize } \min _ {(i, j): w _ {i j} \neq 0} (| | \boldsymbol {v} _ {i} - \boldsymbol {v} _ {j} | | ^ {2} / w _ {i j}).
$$

Give a vector program for finding such an optimal embedding and give the equivalent semidefinite program. Hint: The vector program is:

$$
\begin{array}{l l} \text { minimize } & c \\ \text { subject   to } & \boldsymbol {v} _ {i} \cdot \boldsymbol {v} _ {i} + \boldsymbol {v} _ {j} \cdot \boldsymbol {v} _ {j} - 2 \boldsymbol {v} _ {i} \cdot \boldsymbol {v} _ {j} \leq w _ {i j}, \quad 1 \leq i <   j \leq n \\ & \boldsymbol {v} _ {i} \cdot \boldsymbol {v} _ {i} + \boldsymbol {v} _ {j} \cdot \boldsymbol {v} _ {j} - 2 \boldsymbol {v} _ {i} \cdot \boldsymbol {v} _ {j} \geq c w _ {i j}, \quad 1 \leq i <   j \leq n \\ & \boldsymbol {v} _ {i} \in \mathbf {R} ^ {n}, \quad 1 \leq i \leq n \end{array}\tag{26.8}
$$

26.10 (Knuth [174]) Give an eficient algorithm for sampling from the normal distribution with mean 0 and unit standard deviation, given a source of unbiased random bits.

26.11 Give a strict quadratic program for the MAX k-CUT and maximum directed cut problems, Problems 2.14 and 2.15 stated in Exercises 2.3 and 2.4. Give a vector program relaxation and an equivalent semidefinite program as well.

26.12 (Goemans and Williamson [106]) Consider MAX-CUT with the additional constraint that specified pairs of vertices be on the same/opposite sides of the cut. Formally, we are specified two sets of pairs of vertices, $S _ { 1 }$ and $S _ { 2 }$ The pairs in $S _ { 1 }$ need to be separated, and those in $S _ { 2 }$ need to be on the same side of the cut sought. Under these constraints, the problem is to find a maximum weight cut. Assume that the constraints provided by $S _ { 1 }$ and $S _ { 2 }$ are not inconsistent. Give a strict quadratic program and vector program relaxation for this problem. Show how Algorithm 26.8 can be adapted to this problem so as to maintain the same approximation factor.

26.13 (Karger, Motwani, and Sudan [158]) Let $G = ( V , E )$ be an undirected graph. Consider a vector program with n n-dimensional vectors corresponding to the vertices of $G ,$ , and constraints that the vectors lie on the unit sphere, $S _ { n - 1 }$ , and that for each edge $( i , j ) \in G$

$$
\pmb {v} _ {i} \cdot \pmb {v} _ {j} \leq - \frac {1}{k - 1}.
$$

Show that this vector program is a relaxation of the k-coloring problem, i.e., if $G$ is k-colorable, then this vector program has a feasible solution. Hint: Consider the following k vectors in $\mathbf { R } ^ { n }$ . Each vector has 0 in the last $n - k$ positions. Vector i has $- \sqrt { \frac { k - 1 } { k } }$ in the ith position and $1 / \sqrt { k ( k - 1 ) }$ in the remaining positions.

## 26.14 (Chor and Sudan [43]) Consider the following problem:

Problem 26.14 (Betweenness) We are given a set $S = \{ x _ { 1 } , x _ { 2 } , \ldots , x _ { n } \}$ of n items and a set T of m triplets $T \subseteq S \times S \times S$ . Each triplet consists of three distinct items. A total ordering (permutation) of $S , x _ { \pi _ { 1 } } < x _ { \pi _ { 2 } } < . . . < x _ { \pi _ { r } }$ n satisfies a triplet $( x _ { i } , x _ { j } , x _ { k } ) \in { \textit { T } } \operatorname { i f } \ x _ { j }$ occurs between $x _ { i }$ and $x _ { k }$ in the ordering, i.e., if either $x _ { i } < x _ { j } < x _ { k }$ holds or $x _ { k } \ < \ x _ { j } \ < \ x _ { i }$ holds. The problem is to find a total ordering that maximizes the number of satisfied triplets.

1. Show that a random ordering (i.e., a permutation chosen uniformly at random among all possible permutations) will satisfy in expectation one third of all triplets in T.

2. Use the method of conditional expectation to derandomize the above algorithm, thereby obtaining a factor $1 / 3$ approximation algorithm. What upper bound on OPT is this algorithm using? Give an example showing that with this upper bound a better algorithm is not possible.

3. The rest of the exercise develops an algorithm based on semidefinite programming. The ideas can be illustrated more simply by assuming that the instance is satisfiable, i.e., that all m triplets can be satisfied simultaneously. Note that checking for this condition is NP-hard, so the restriction of the betweenness problem to such instances is not an NP-optimization problem (see Exercise 1.9). Show that an instance is satisfiable if the following strict quadratic program in variables $p _ { i } \in \mathbf { R } , i = 1 , \ldots , n$ , has a solution:

$$
\begin{array}{r l} (p _ {i} - p _ {j}) ^ {2} \geq 1 & \text {for all i,j}, \\ (p _ {i} - p _ {j}) (p _ {k} - p _ {j}) \leq 0 & \text {for all (x_{i} , x_{j} , x_{k})\in T}. \end{array}
$$

4. Obtain the vector programming relaxation of this strict quadratic program as well as the equivalent semidefinite program.

5. Give an instance where the above semidefinite program is satisfiable but the instance itself is not satisfiable.

6. Let us assume that $n \times n$ matrix Y is a feasible solution to the above semidefinite program, and let $v _ { i } \in \mathbf { R } ^ { n }$ for $i = 1 , \cdots , n$ be vectors such that $Y _ { i j } = v _ { i } ^ { T } v _ { j }$ . Now select r uniformly at random on the unit sphere $S _ { n - 1 }$ Consider the random ordering obtained by sorting $r ^ { T } v _ { i }$ . Show that, in expectation, this random ordering satisfies at least half of the constraints in $T .$

Hint: What is the probability that a single triplet is satisfied? What is the angle between $v _ { i } - v _ { j }$ and $v _ { k } - v _ { j } ?$

## 26.7 Notes

The results of this chapter are based on the seminal work of Goemans and Williamson [106] that introduced the use of semidefinite programs in approximation algorithms. Experimental results reported in their paper show that Algorithm 26.8 performs much better on typical instances than the worst case guarantee. Mahajan and Ramesh [200] give a derandomization of Algorithm 26.8, as well as the MAX-2SAT algorithm, using the method of conditional expectation. Karlof [161] provides a family of tight examples for Algorithm 26.8, for which the expected weight of the cut produced is arbitrarily close to $\alpha \cdot \mathrm { O P T } _ { v }$ . Feige and Schechtman [84] strengthen this to showing that there are graphs such that even the best hyperplane (rather than a random one, as prescribed in Algorithm 26.8) gives a cut of weight only $\alpha \cdot \mathrm { O P T } _ { v }$ . They also show that the integrality gap of the semidefinite relaxation (26.2) for MAX-CUT is α.

For eficient algorithms, using interior point methods, for approximating semidefinite programs, see Alizadeh [4], Nesterov and Nemirovskii [214] and Overton [215]. For a duality theory for semidefinite programs, see Wolkowitz [260] and Vandeberghe and Boyd [250].

Lov´asz and Schrijver [196] use semidefinite programming to provide an automatic way of strengthening any convex relaxation (having a convex feasible region) of a 0/1 integer program. They also show that if the original relaxation can be optimized in polynomial time, then so can the strengthened relaxation (however, in order to guarantee polynomial running time, this process can be applied only a constant number of times).

Feige and Goemans [81] improve the approximation factor for MAX-2SAT to 0.931. They also give a 0.859 factor for the maximum directed cut problem (see Exercise 26.11). For semidefinite-programming-based algorithms for the MAX k-CUT problem see Frieze and Jerrum [90]. Karger, Motwani, and Sudan [158] use the relaxation in Exercise 26.13 to obtain an $O ( n ^ { 1 - 3 / ( k + 1 ) } \log ^ { 1 / 2 } n )$ coloring for k-colorable graphs.

Part III

Other Topics