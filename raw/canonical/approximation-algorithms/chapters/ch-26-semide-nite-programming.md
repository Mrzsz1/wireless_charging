---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-26"
chapter_number: 26
chapter_title: "Semideﬁnite Programming"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 273
source_page_end: 290
printed_page_start: 255
printed_page_end: 272
part_ids: ["approximation-algorithms-ch-26-part-027"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Semideﬁnite Programming

26 Semideﬁnite Programming




In the previous chapters of Part II of this book we have shown how linear
programs provide a systematic way of placing a good upper bound on OPT
(assuming a minimization problem), for numerous NP-hard problems. As
stated earlier, this is a key step in the design of an approximation algorithm
for an NP-hard problem. It is natural, then, to ask if there are other widely
applicable ways of doing this.
    In this chapter we provide another class of relaxations, called vector pro-
grams. These serve as relaxations for several NP-hard problems, in partic-
ular, for problems that can be expressed as strict quadratic programs (see
Section 26.1 for a deﬁnition). Vector programs are equivalent to a powerful
and well-studied generalization of linear programs, called semideﬁnite pro-
grams. Semideﬁnite programs, and consequently vector programs, can be
solved within an additive error of ε, for any ε > 0, in time polynomial in n
and log(1/ε), using the ellipsoid algorithm (see Section 26.3).
    We will illustrate the use of vector programs by deriving a 0.87856 factor
algorithm for the following problem (see Exercises 2.1 and 16.6 for a factor
1/2 algorithm).
Problem 26.1 (Maximum cut (MAX-CUT)) Given an undirected
graph G = (V, E), with edge weights w : E → Q+ , ﬁnd a partition (S, S)
of V so as to maximize the total weight of edges in this cut, i.e., edges that
have one endpoint in S and one endpoint in S.


26.1 Strict quadratic programs and vector programs

A quadratic program is the problem of optimizing (minimizing or maximiz-
ing) a quadratic function of integer valued variables, subject to quadratic
constraints on these variables. If each monomial in the objective function, as
well as in each of the constraints, is of degree 0 (i.e., is a constant) or 2, then
we will say that this is a strict quadratic program.
   Let us give a strict quadratic program for MAX-CUT. Let yi be an in-
dicator variable for vertex vi which will be constrained to be either +1 or
−1. The partition (S, S) will be deﬁned as follows. S = {vi | yi = 1} and
S = {vi | yi = −1}. If vi and vj are on opposite sides of this partition,
256     26   Semideﬁnite Programming

then yi yj = −1, and edge (vi , vj ) contributes wij to the objective function.
On the other hand, if they are on the same side, then yi yj = 1, and edge
(vi , vj ) makes no contribution. Hence, an optimal solution to this program is
a maximum cut in G.

                      1     
      maximize                      wij (1 − yi yj )                              (26.1)
                      2
                          1≤i<j≤n

      subject to     yi2 = 1,                              vi ∈ V
                     yi ∈ Z,                               vi ∈ V

     We will relax this program to a vector program. A vector program is
deﬁned over n vector variables in Rn , say v 1 , . . . , v n , and is the problem of
optimizing (minimizing or maximizing) a linear function of the inner products
v i · v j , 1 ≤ i ≤ j ≤ n, subject to linear constraints on these inner products.
Thus, a vector program can be thought of as being obtained from a linear
program by replacing each variable with an inner product of a pair of these
vectors.
     A strict quadratic program over n integer variables deﬁnes a vector pro-
gram over n vector variables in Rn as follows. Establish a correspondence
between the n integer variables and the n vector variables, and replace each
degree 2 term with the corresponding inner product. For instance, the term
yi yj in (26.1) is replaced with v i · v j . In this manner, we obtain the following
vector program for MAX-CUT.

                      1     
      maximize                      wij (1 − v i · v j )                          (26.2)
                      2
                          1≤i<j≤n

      subject to     v i · v i = 1,                          vi ∈ V
                              n
                     vi ∈ R ,                                vi ∈ V

    Because of the constraint v i ·v i = 1, the vectors v 1 , . . . , v n are constrained
to lie on the n-dimensional sphere, Sn−1 . Any feasible solution to (26.1) yields
a solution to (26.2) having the same objective function value, by assigning
the vector (yi , 0, . . . , 0) to v i . (Notice that under this assignment, v i · v j is
simply yi yj .) Therefore, the vector program (26.2) is a relaxation of the strict
quadratic program (26.1). Clearly, this holds in general as well; the vector
program corresponding to a strict quadratic program is a relaxation of the
quadratic program.
    Interestingly enough, vector programs are approximable to any desired
degree of accuracy in polynomial time, and thus relaxation (26.2) provides
an upper bound on OPT for MAX-CUT. To show this, we need to recall
some interesting and powerful properties of positive semideﬁnite matrices.
                          26.2   Properties of positive semideﬁnite matrices        257

Remark 26.2 Vector programs do not always come about as relaxations
of strict quadratic programs. Exercise 26.13 gives an NP-hard problem that
has vector program relaxation; however, we do not know of a strict quadratic
program for it.


26.2 Properties of positive semideﬁnite matrices
Let A be a real, symmetric n×n matrix. Then A has real eigenvalues and has
n linearly independent eigenvectors (even if the eigenvalues are not distinct).
We will say that A is positive semideﬁnite if

      ∀x ∈ Rn , xT Ax ≥ 0.

We will use the following two equivalent conditions crucially. We provide a
proof sketch for completeness.

Theorem 26.3 Let A be a real symmetric n × n matrix. Then, the following
are equivalent:
 1. ∀x ∈ Rn , xT Ax ≥ 0.
 2. All eigenvalues of A are nonnegative.
 3. There is an n × n real matrix W , such that A = W T W .

Proof: (1 ⇒ 2): Let λ be an eigenvalue of A, and let v be a corresponding
eigenvector. Therefore, Av = λv. Pre-multiplying by v T we get v T Av =
λv T v. Now, by (1), v T Av ≥ 0. Therefore, λv T v ≥ 0. Since v T v > 0, λ ≥ 0.
    (2 ⇒ 3): Let λ1 , . . . , λn be the n eigenvalues of A, and v 1 , . . . , v n be the
corresponding complete collection of orthonormal eigenvectors. Let Q be the
matrix whose columns are v 1 , . . . , v n , and Λ be the diagonal matrix with
entries λ1 , . . . , λn . Since for each i, Av i = λi v i , we have AQ = QΛ. Since
Q is orthogonal, i.e., QQT = I, we get that QT = Q−1 . Therefore,

      A = QΛQT .

Let D be the diagonal matrix whose diagonal entries are the positive square
roots of λ1 , . . . , λn (by (2), λ1 , . . . , λn are nonnegative, and thus their square
roots are real). Then, Λ = DD T . Substituting, we get

      A = QDD T QT = (QD)(QD)T .

Now, (3) follows by letting W = (QD)T .
  (3 ⇒ 1): For any

      x ∈ Rn , xT Ax = xT W T W x = (W x)T (W x) ≥ 0.                                 ✷
258     26   Semideﬁnite Programming

    Using Cholesky decomposition (see Section 26.7), a real symmetric matrix
can be decomposed, in polynomial time, as A = U ΛU T , where Λ is a diago-
nal matrix whose diagonal entries are the eigenvalues of A. Now A is positive
semideﬁnite iﬀ all the entries of Λ are nonnegative, thus giving a polynomial
time test for positive semideﬁniteness. The decomposition W W T is not poly-
nomial time computable because in general it may contain irrational entries.
However, it can be approximated to any desired degree by approximating the
square roots of the entries of Λ. In the rest of this chapter we will assume
that we have an exact decomposition, since the inaccuracy resulting from an
approximate decomposition can be absorbed into the approximation factor
(see Exercise 26.6).
    It is easy to see that the sum of two n × n positive semideﬁnite matrices is
also positive semideﬁnite (e.g., using characterization (1) of Theorem 26.3).
This is also true of any convex combination of such matrices.


26.3 The semideﬁnite programming problem
Let Y be an n × n matrix of real valued variables whose (i, j)th entry is yij .
The problem of maximizing a linear function of the yij ’s, subject to linear
constraints on them, and the additional constraint that Y be symmetric and
positive semideﬁnite, is called the semideﬁnite programming problem.
    Let us introduce some notation to state this formally. Denote by Rn×n
the space of n × n real matrices. Recall that the trace of a matrix A ∈ Rn×n
is the sum of its diagonal entries and is denoted by tr(A). The Frobenius
inner product of matrices A, B ∈ Rn×n , denoted A • B, is deﬁned to be
                             n 
                              n
      A • B = tr(AT B) =                aij bij ,
                              i=1 j=1


where aij and bij are the (i, j)th entries of A and B, respectively. Let Mn
denote the cone of symmetric n×n real matrices. For A ∈ Mn , A / 0 denotes
the fact that matrix A is positive semideﬁnite.
   Let C, D 1 , . . . , D k ∈ Mn and d1 , . . . dk ∈ R. Following is a statement of
the general semideﬁnite programming problem. Let us denote it by S.

      maximize      C •Y                                                     (26.3)

      subject to    D i • Y = di ,       1≤i≤k
                    Y / 0,
                    Y ∈ Mn .

    Observe that if C, D 1 , . . . , D k are all diagonal matrices, this is simply a
linear programming problem. As in the case of linear programs, it is easy to
                             26.3   The semideﬁnite programming problem          259

see that allowing linear inequalities, in addition to equalities, does not make
the problem more general.
     Let us call a matrix in Rn×n satisfying all the constraints of S a feasible
solution. Since a convex combination of positive semideﬁnite matrices is pos-
itive semideﬁnite, it is easy to see that the set of feasible solutions is convex,
i.e., if A ∈ Rn×n and B ∈ Rn×n are feasible solutions then so is any convex
combination of these solutions.
     Let A ∈ Rn×n be an infeasible point. Let C ∈ Rn×n . A hyperplane
C • Y ≤ b is called a separating hyperplane for A if all feasible points satisfy
it and point A does not satisfy it. In the next theorem we show how to ﬁnd a
separating hyperplane in polynomial time. As a consequence, for any ε > 0,
semideﬁnite programs can be solved within an additive error of ε, in time
polynomial in n and log(1/ε), using the ellipsoid algorithm (see Section 26.7
for more eﬃcient methods).
Theorem 26.4 Let S be a semideﬁnite programming problem, and A be a
point in Rn×n . We can determine, in polynomial time, whether A is feasible
for S and, if it is not, ﬁnd a separating hyperplane.

Proof: Testing for feasibility involves ensuring that A is symmetric and
positive semideﬁnite and that it satisﬁes all the linear constraints. By remarks
made in Section 26.2, this can be done in polynomial time. If A is infeasible,
a separating hyperplane is obtained as follows.
• If A is not symmetric, aij > aji for some i, j. Then yij ≤ yji is a separating
  hyperplane.
• If A is not positive semideﬁnite, then it has a negative eigenvalue, say λ.
  Let v be the corresponding eigenvector. Now (vv T ) • Y = v T Y v ≥ 0 is a
  separating hyperplane.
• If any of the linear constraints is violated, it directly yields a separating
  hyperplane.
                                                                                   ✷
   Next, let us show that vector programs are equivalent to semideﬁnite
programs, thereby showing that the former can be solved eﬃciently to any
desired degree of accuracy. Let V be a vector program on n n-dimensional
vector variables v 1 , . . . , v n . Deﬁne the corresponding semideﬁnite program, S,
over n2 variables yij , 1 ≤ i, j ≤ n, as follows. Replace each inner product v i ·v j
occurring in V by the variable yij . The objective function and constraints are
now linear in the yij ’s. Additionally, require that matrix Y , whose (i, j)th
entry is yij , be symmetric and positive semideﬁnite.
Lemma 26.5 Vector program V is equivalent to semideﬁnite program S.

Proof: We will show that corresponding to each feasible solution to V,
there is a feasible solution to S of the same objective function value, and vice
260     26   Semideﬁnite Programming

versa. Let a1 , . . . , an be a feasible solution to V. Let W be the matrix whose
columns are a1 , . . . , an . Then, it is easy to see that A = W T W is a feasible
solution to S having the same objective function value.
    For the other direction, let A be a feasible solution to S. By Theorem
26.3, there is an n × n matrix W such that A = W T W . Let a1 , . . . , an be
the columns of W . Then, it is easy to see that a1 , . . . , an is a feasible solution
to V having the same objective function value.                                       ✷
   Finally, we give the semideﬁnite programming relaxation to MAX-CUT
that is equivalent to vector program 26.2.

                     1      
      maximize                     wij (1 − yi yj )                            (26.4)
                     2
                         1≤i<j≤n

      subject to     yi2 = 1,                         vi ∈ V
                     Y / 0,
                     Y ∈ Mn .


26.4 Randomized rounding algorithm
We now present the algorithm for MAX-CUT. For convenience, let us assume
that we have an optimal solution to the vector program (26.2). The slight
inaccuracy in solving it can be absorbed into the approximation factor (see
Exercise 26.6). Let a1 , . . . , an be an optimal solution, and let OPTv denote its
objective function value. These vectors lie on the n-dimensional unit sphere
Sn−1 . We need to obtain a cut (S, S) whose weight is a large fraction of
OPTv .
    Let θij denote the angle between vectors ai and aj . The contribution of
this pair of vectors to OPTv is
      wij
          (1 − cos θij ).
       2
Clearly, the closer θij is to π, the larger this contribution will be. In turn,
we would like vertices vi and vj to be separated if θij is large. The following
method accomplishes precisely this. Pick r to be a uniformly distributed
vector on the unit sphere Sn−1 , and let S = {vi | ai · r ≥ 0}.
                                                               θij
Lemma 26.6               Pr[vi and vj are separated ] =            .
                                                                π

Proof: Project r onto the plane containing v i and v j . Now, vertices vi and
vj will be separated iﬀ the projection lies in one of the two arcs of angle θij
shown below.
                                          26.4          Randomized rounding algorithm   261
                                                   vi

                                                                  vj

                                                        θ ij


                                            θ ij




Since r has been picked from a spherically symmetric distribution, its pro-
jection will be a random direction on this plane. The lemma follows.     ✷
    The next lemma shows how to generate vectors that are uniformly dis-
tributed on the unit sphere Sn−1 .
Lemma 26.7 Let x1 , . . . , xn be picked independently from the normal distri-
bution with mean 0 and unit standard deviation. Let d = (x21 + . . . + x2n )1/2 .
Then, (x1 /d, . . . , xn /d) is a random vector on the unit sphere Sn−1 .

Proof: Consider the vector r = (x1 , . . . , xn ). The distribution function for
r has density
                               n
                               +    1             1            2
                                        2                − 12   y
      f (y1 , . . . , yn ) =       √ e−yi /2 =      n/2
                                                        e      i i.

                               i=1
                                    2π         (2π)

Notice that the density function depends only on the distance of the point
from the origin. Therefore, the distribution of r is spherically symmetric.
Hence, dividing by the length of r, i.e., d, we get a random vector on Sn−1 .
                                                                           ✷
   The algorithm is summarized below.


 Algorithm 26.8 (MAX-CUT)
  1. Solve vector program (26.2). Let a1 , . . . , an be an optimal solution.
  2. Pick r to be a uniformly distributed vector on the unit sphere Sn−1 .
  3. Let S = {vi | ai · r ≥ 0}.


   Let W be the random variable denoting the weight of edges in the cut
picked by Algorithm 26.8, and let

            2           θ
      α=       min            .
            π 0≤θ≤π 1 − cos θ
262       26    Semideﬁnite Programming

One can show that α > 0.87856 (see Exercise 26.3).
Lemma 26.9 E[W ] ≥ α · OPTv .

Proof: By the deﬁnition of α we have that for any θ, 0 ≤ θ ≤ π,
                                
        θ            1 − cos θ
          ≥α                         .                                                 (26.5)
        π                2

Using this and Lemma 26.6, we get
                       
        E[W ] =                wij Pr[v i and v j are separated]
                     1≤i<j≤n

                            θij                  1
        =              wij       ≥α·                 wij (1 − cos θij ) = α · OPTv .
                              π                    2
             1≤i<j≤n                     1≤i<j≤n

                                                                                           ✷
      Let us deﬁne the integrality gap for relaxation (26.2) to be

              OPT(I)
        inf            ,
         I    OPTv (I)

where the inﬁmum is over all instances I of MAX-CUT.
Corollary 26.10 The integrality gap for relaxation (26.2) is at least α >
0.87856.
Theorem 26.11 There is a randomized approximation algorithm for MAX-
CUT achieving an approximation factor of 0.87856.

Proof: Let us ﬁrst obtain a “high probability” statement using the bound
on expectation established in Lemma 26.9. Let T denote the sum of weights
of all edges in G, and deﬁne a so that E[W ] = aT . Let

        p = Pr[W < (1 − ε)aT ],

where ε > 0 is a constant. Since the random variable W is always bounded
by T , we get

        aT ≤ p(1 − ε)aT + (1 − p)T.

Therefore,

                 1−a
        p≤                .
               1 − a + aε
                             26.5   Improving the guarantee for MAX-2SAT              263

Now,

                                                         αT
       T ≥ E[W ] = aT ≥ α · OPTv ≥ α · OPT ≥                ,
                                                          2

where the last inequality follows from the fact that OPT ≥ T /2 (see Exercise
2.1). Therefore, 1 ≥ a ≥ α/2. Using this upper and lower bound on a, we get

                   εα/2
       p≤1−                 ≤ 1 − c,
                1 + ε − α/2

where

               εα/2
       c=               .
            1 + ε − α/2

   Run Algorithm 26.8 1/c times, and output the heaviest cut found in these
runs. Let W  be the weight of this cut. Then,

                                                  1
       Pr[W  ≥ (1 − ε)aT ] ≥ 1 − (1 − c)1/c ≥ 1 − .
                                                  e
Since aT ≥ α · OPT > 0.87856 OPT, we can pick a value of ε > 0 so that
(1 − ε)aT ≥ 0.87856 OPT.                                            ✷

Example 26.12 The following example shows that the bound on the inte-
grality gap of relaxation (26.2) given in Corollary 26.10 is almost tight. Con-
sider a graph which is a 5-cycle v1 , v2 , v3 , v4 , v5 , v1 . Then, an optimal solution
to relaxation (26.2) is to place the ﬁve vectors in a 2-dimensional subspace
within which they are given by v i = (cos( 4iπ                  4iπ
                                                     5 ), sin( 5 )), for 1 ≤ i ≤ 5 (see
                                                                                     √
Exercise 26.5). The cost of this solution is OPTv = 52 (1 + cos π5 ) = 25+5        8
                                                                                       5
                                                                                         .
Since OPT = 4 for this graph, the integrality gap for this example is
   32√
25+5 5
         = 0.88445....                                                                ✷



26.5 Improving the guarantee for MAX-2SAT
MAX-2SAT is the restriction of MAX-SAT (Problem 16.1) to formulae in
which each clause contains at most two literals. In Chapter 16 we obtained
a factor 3/4 algorithm for this problem using randomization, followed by the
method of conditional expectation. We will give an improved algorithm using
semideﬁnite programming.
    The key new idea needed is a way of converting the obvious quadratic
program (see Exercise 26.8) for this problem into a strict quadratic program.
We will accomplish this as follows. Corresponding to each Boolean variable
264     26   Semideﬁnite Programming

xi , introduce variable yi which is constrained to be either +1 or −1, for
1 ≤ i ≤ n. In addition, introduce another variable, say y0 , which is also
constrained to be +1 or −1. Let us impose the convention that Boolean
variable xi is true if yi = y0 and false otherwise. Under this convention we
can write the value of a clause in terms of the yi ’s, where the value, v(C), of
clause C is deﬁned to be 1 if C is satisﬁed and 0 otherwise. Thus, for clauses
containing only one literal,

                 1 + y0 yi              1 − y0 yi
      v(xi ) =             and v(xi ) =           .
                    2                      2

Consider a clause containing 2 literals, e.g., (xi ∨ xj ). Its value is

                                             1 − y 0 yi 1 − y0 yj
      v(xi ∨ xj ) = 1 − v(xi )v(xj ) = 1 −
                                                  2         2
                     1                       2
                                                     
                   =    3 + y0 yi + y0 yj − y0 yi yj
                     4
                     1 + y0 y i   1 + y0 yj     1 − yi yj
                   =            +           +             .
                        4            4              4

Observe that in this derivation we have used the fact that y02 = 1. In all
the remaining cases as well, it is easy to check that the value of a 2 literal
clause consists of a linear combination of terms of the form (1 + yi yj ) or
(1 − yi yj ). Therefore, a MAX-2SAT instance can be written as the following
strict quadratic program, where the aij ’s and bij ’s are computed by collecting
terms appropriately.
                        
      maximize                   aij (1 + yi yj ) + bij (1 − yi yj )               (26.6)
                      0≤i<j≤n

      subject to      yi2 = 1,                                             0≤i≤n
                      yi ∈ Z,                                              0≤i≤n

   Following is the vector program relaxation for (26.6), where vector vari-
able v i corresponds to yi .
                        
      maximize                   aij (1 + v i · v j ) + bij (1 − v i · v j )       (26.7)
                      0≤i<j≤n

      subject to      v i · v i = 1,                                           0≤i≤n
                              n+1
                      vi ∈ R        ,                                          0≤i≤n

    The algorithm is similar to that for MAX-CUT. We solve vector program
(26.7). Let a0 , . . . , an be an optimal solution. Pick a vector r uniformly dis-
tributed on the unit sphere in (n + 1) dimensions, Sn , and let yi = 1 iﬀ
                                                               26.6   Exercises      265

r · ai ≥ 0, for 0 ≤ i ≤ n. This gives a truth assignment for the Boolean
variables. Let W be the random variable denoting the weight of this truth
assignment.
Lemma 26.13 E[W ] ≥ α · OPTv .

Proof:
                    
     E[W ] = 2             aij Pr[yi = yj ] + bij Pr[yi = yj ].
                 0≤i<j≤n


Let θij denote the angle between ai and aj . By inequality (26.5),

                       θij  α
     Pr[yi = yj ] =       ≥ (1 − cos θij ).
                        π   2
By Exercise 26.4,

                           θij  α
     Pr[yi = yj ] = 1 −        ≥ (1 + cos θij ).
                            π   2
Therefore,
                       
     E[W ] ≥ α ·              aij (1 + cos θij ) + bij (1 − cos θij ) = α · OPTv .
                    0≤i<j≤n

                                                                                      ✷



26.6 Exercises

26.1 Is matrix W in Theorem 26.3 unique (up to multiplication by −1)?
Hint: Consider the matrix QDQT .

26.2 Let B be obtained from matrix A by throwing away a set of columns
and the corresponding set of rows. We will say that B is a principal submatrix
of A. Show that the following is another equivalent condition for a real sym-
metric matrix to be positive semideﬁnite: that all of its principal submatrices
have nonnegative determinants. (See Theorem 26.3 for other conditions.)

26.3 Show, using elementary calculus, that α > 0.87856.

26.4 Show that for any φ, 0 ≤ φ ≤ π,

          φ  α
     1−     ≥ (1 + cos φ).
          π  2
266    26    Semideﬁnite Programming

Hint: Substitute θ = π − φ in inequality (26.5).

26.5 Show that for a 5-cycle, the solution given in Example 26.12 is indeed
an optimal solution to the vector program relaxation for MAX-CUT.

26.6 Show that the inaccuracies resulting from the fact we do not have
an optimal solution to the vector program (26.2) and that matrix A is not
exactly decomposed as W W T (see end of Section 26.2) can be absorbed into
the approximation factor for MAX-CUT.
Hint: Use the idea behind the proof of Theorem 26.11 and the fact that the
solution to program (26.2) lies in the range [T /2, T ], where T is the sum of
weights of all edges in G.

26.7 Theorem 26.11 shows how to obtain a “high probability” statement
from Lemma 26.9. Obtain a similar statement for MAX-2SAT, using Lemma
26.13, thereby obtaining a 0.87856 factor algorithm for MAX-2SAT.

26.8 Give a quadratic program for MAX-2SAT.

26.9 (Linial, London, and Rabinovich [190]) Let G be the complete undi-
rected graph on n vertices, V , and let w be a function assigning nonnegative
weights to the edges of G. The object is to ﬁnd an optimal distortion 22 -
embedding of the vertices of G. Let vertex i be mapped to v i ∈ Rn by such
an embedding. The embedding should satisfy:
1. no edge is overstretched, i.e., for 1 ≤ i < j ≤ n, ||v i − v j ||2 ≤ w( ij), and
2. the maximum shrinkage is minimized, i.e.,

            maximize      min          (||v i − v j ||2 /wij ).
                       (i,j):wij =0


Give a vector program for ﬁnding such an optimal embedding and give the
equivalent semideﬁnite program.
Hint: The vector program is:

      minimize     c                                                        (26.8)

      subject to   v i · v i + v j · v j − 2v i · v j ≤ wij ,     1≤i<j≤n
                   v i · v i + v j · v j − 2v i · v j ≥ cwij ,    1≤i<j≤n
                              n
                   vi ∈ R ,                                       1≤i≤n


26.10 (Knuth [174]) Give an eﬃcient algorithm for sampling from the nor-
mal distribution with mean 0 and unit standard deviation, given a source of
unbiased random bits.
                                                         26.6   Exercises   267

26.11 Give a strict quadratic program for the MAX k-CUT and maximum
directed cut problems, Problems 2.14 and 2.15 stated in Exercises 2.3 and
2.4. Give a vector program relaxation and an equivalent semideﬁnite program
as well.

26.12 (Goemans and Williamson [106]) Consider MAX-CUT with the ad-
ditional constraint that speciﬁed pairs of vertices be on the same/opposite
sides of the cut. Formally, we are speciﬁed two sets of pairs of vertices, S1
and S2 . The pairs in S1 need to be separated, and those in S2 need to be on
the same side of the cut sought. Under these constraints, the problem is to
ﬁnd a maximum weight cut. Assume that the constraints provided by S1 and
S2 are not inconsistent. Give a strict quadratic program and vector program
relaxation for this problem. Show how Algorithm 26.8 can be adapted to this
problem so as to maintain the same approximation factor.

26.13 (Karger, Motwani, and Sudan [158]) Let G = (V, E) be an undirected
graph. Consider a vector program with n n-dimensional vectors corresponding
to the vertices of G, and constraints that the vectors lie on the unit sphere,
Sn−1 , and that for each edge (i, j) ∈ G,

                     1
      vi · vj ≤ −       .
                    k−1

Show that this vector program is a relaxation of the k-coloring problem, i.e.,
if G is k-colorable, then this vector program has a feasible solution.
Hint: Consider the following .   k vectors in Rn . Each vector has 0 in the last
                                                                  /
n − k positions. Vector i has − k−1  k in the ith position and 1/ k(k − 1) in
the remaining positions.

26.14 (Chor and Sudan [43]) Consider the following problem:
Problem 26.14 (Betweenness) We are given a set S = {x1 , x2 , . . . , xn } of
n items and a set T of m triplets T ⊆ S × S × S. Each triplet consists of three
distinct items. A total ordering (permutation) of S, xπ1 < xπ2 < . . . < xπn
satisﬁes a triplet (xi , xj , xk ) ∈ T if xj occurs between xi and xk in the
ordering, i.e., if either xi < xj < xk holds or xk < xj < xi holds. The
problem is to ﬁnd a total ordering that maximizes the number of satisﬁed
triplets.
 1. Show that a random ordering (i.e., a permutation chosen uniformly at
    random among all possible permutations) will satisfy in expectation one
    third of all triplets in T .
 2. Use the method of conditional expectation to derandomize the above
    algorithm, thereby obtaining a factor 1/3 approximation algorithm. What
    upper bound on OPT is this algorithm using? Give an example showing
    that with this upper bound a better algorithm is not possible.
268    26     Semideﬁnite Programming

 3. The rest of the exercise develops an algorithm based on semideﬁnite pro-
    gramming. The ideas can be illustrated more simply by assuming that the
    instance is satisﬁable, i.e., that all m triplets can be satisﬁed simultane-
    ously. Note that checking for this condition is NP-hard, so the restriction
    of the betweenness problem to such instances is not an NP-optimization
    problem (see Exercise 1.9). Show that an instance is satisﬁable iﬀ the
    following strict quadratic program in variables pi ∈ R, i = 1, . . . , n, has
    a solution:

                    (pi − pj )2 ≥ 1    for all i, j,
            (pi − pj )(pk − pj ) ≤ 0   for all (xi , xj , xk ) ∈ T.

 4. Obtain the vector programming relaxation of this strict quadratic pro-
    gram as well as the equivalent semideﬁnite program.
 5. Give an instance where the above semideﬁnite program is satisﬁable but
    the instance itself is not satisﬁable.
 6. Let us assume that n × n matrix Y is a feasible solution to the above
    semideﬁnite program, and let vi ∈ Rn for i = 1, · · · , n be vectors such that
    Yij = viT vj . Now select r uniformly at random on the unit sphere Sn−1 .
    Consider the random ordering obtained by sorting rT vi . Show that, in
    expectation, this random ordering satisﬁes at least half of the constraints
    in T .
    Hint: What is the probability that a single triplet is satisﬁed? What is
    the angle between vi − vj and vk − vj ?


26.7 Notes
The results of this chapter are based on the seminal work of Goemans and
Williamson [106] that introduced the use of semideﬁnite programs in approx-
imation algorithms. Experimental results reported in their paper show that
Algorithm 26.8 performs much better on typical instances than the worst case
guarantee. Mahajan and Ramesh [200] give a derandomization of Algorithm
26.8, as well as the MAX-2SAT algorithm, using the method of conditional
expectation. Karloﬀ [161] provides a family of tight examples for Algorithm
26.8, for which the expected weight of the cut produced is arbitrarily close to
α · OPTv . Feige and Schechtman [84] strengthen this to showing that there
are graphs such that even the best hyperplane (rather than a random one,
as prescribed in Algorithm 26.8) gives a cut of weight only α · OPTv . They
also show that the integrality gap of the semideﬁnite relaxation (26.2) for
MAX-CUT is α.
    For eﬃcient algorithms, using interior point methods, for approximating
semideﬁnite programs, see Alizadeh [4], Nesterov and Nemirovskii [214] and
Overton [215]. For a duality theory for semideﬁnite programs, see Wolkowitz
[260] and Vandeberghe and Boyd [250].
                                                        26.7   Notes   269

    Lovász and Schrijver [196] use semideﬁnite programming to provide an
automatic way of strengthening any convex relaxation (having a convex fea-
sible region) of a 0/1 integer program. They also show that if the original
relaxation can be optimized in polynomial time, then so can the strength-
ened relaxation (however, in order to guarantee polynomial running time,
this process can be applied only a constant number of times).
    Feige and Goemans [81] improve the approximation factor for MAX-
2SAT to 0.931. They also give a 0.859 factor for the maximum directed
cut problem (see Exercise 26.11). For semideﬁnite-programming-based al-
gorithms for the MAX k-CUT problem see Frieze and Jerrum [90]. Karger,
Motwani, and Sudan [158] use the relaxation in Exercise 26.13 to obtain an
O(n1−3/(k+1) log1/2 n) coloring for k-colorable graphs.
     Part III

Other Topics
