---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "ch-13"
chapter_number: 13
chapter_title: "Set Cover via Dual Fitting"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 126
source_page_end: 136
printed_page_start: 108
printed_page_end: 118
part_ids: ["approximation-algorithms-ch-13-part-014"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Set Cover via Dual Fitting

13 Set Cover via Dual Fitting




In this chapter we will introduce the method of dual ﬁtting, which helps an-
alyze combinatorial algorithms using LP-duality theory. Using this method,
we will present an alternative analysis of the natural greedy algorithm (Al-
gorithm 2.2) for the set cover problem (Problem 2.1). Recall that in Section
2.1 we deferred giving the lower bounding method on which this algorithm
was based. We will provide the answer below. The power of this approach
will become apparent when we show the ease with which it extends to solving
several generalizations of the set cover problem (see Section 13.2).
    The method of dual ﬁtting can be described as follows, assuming a min-
imization problem: The basic algorithm is combinatorial – in the case of set
cover it is in fact the simple greedy algorithm. Using the linear programming
relaxation of the problem and its dual, one shows that the primal integral so-
lution found by the algorithm is fully paid for by the dual computed; however,
the dual is infeasible. By fully paid for we mean that the objective function
value of the primal solution found is at most the objective function value of
the dual computed. The main step in the analysis consists of dividing the
dual by a suitable factor and showing that the shrunk dual is feasible, i.e., it
ﬁts into the given instance. The shrunk dual is then a lower bound on OPT,
and the factor is the approximation guarantee of the algorithm.


13.1 Dual-ﬁtting-based analysis for the greedy set
cover algorithm
To formulate the set cover problem as an integer program, let us assign a
variable xS for each set S ∈ S, which is allowed 0/1 values. This variable will
be set to 1 iﬀ set S is picked in the set cover. Clearly, the constraint is that
for each element e ∈ U we want that at least one of the sets containing it be
picked.
                   
      minimize           c(S)xS                                          (13.1)
                   S∈S
                     
      subject to            xS ≥ 1,   e∈U
                   S: e∈S
      13.1   Dual-ﬁtting-based analysis for the greedy set cover algorithm     109

                    xS ∈ {0, 1},          S∈S

    The LP-relaxation of this integer program is obtained by letting the do-
main of variables xS be 1 ≥ xS ≥ 0. Since the upper bound on xS is re-
dundant, we get the following LP. A solution to this LP can be viewed as a
fractional set cover.
                    
     minimize             c(S)xS                                             (13.2)
                    S∈S
                     
     subject to                xS ≥ 1,    e∈U
                    S: e∈S
                    xS ≥ 0,               S∈S

Example 13.1 Let us give a simple example to show that a fractional set
cover may be cheaper than the optimal integral set cover. Let U = {e, f, g}
and the speciﬁed sets be S1 = {e, f }, S2 = {f, g}, S3 = {e, g}, each of unit
cost. An integral cover must pick two of the sets for a cost of 2. On the other
hand, picking each set to the extent of 1/2 gives a fractional cover of cost
3/2.                                                                         ✷
   Introducing a variable ye corresponding to each element e ∈ U , we obtain
the dual program.
                    
     maximize             ye                                                 (13.3)
                    e∈U
                     
     subject to              ye ≤ c(S),    S∈S
                    e: e∈S
                    ye ≥ 0,                e∈U

    Intuitively, why is LP (13.3) the dual of LP (13.2)? In our experience,
this is not the right question to be asked. As stated in Section 12.1, there
is a purely mechanical procedure for obtaining the dual of a linear program.
Once the dual is obtained, one can devise intuitive, and possibly physically
meaningful, ways of thinking about it. Using this mechanical procedure, one
can obtain the dual of a complex linear program in a fairly straightforward
manner. Indeed, the LP-duality-based approach derives its wide applicability
from this fact.
    An intuitive way of thinking about LP (13.3) is that it is packing “stuﬀ”
into elements, trying to maximize the total amount packed, subject to the
constraint that no set is overpacked. A set is said to be overpacked if the
total amount packed into its elements exceeds the cost of the set. Whenever
the coeﬃcients in the constraint matrix, objective function, and right-hand
side are all nonnegative, the minimization LP is called a covering LP and
110       13   Set Cover via Dual Fitting

the maximization LP is called a packing LP. Thus, (13.2) and (13.3) are a
covering-packing pair of linear programs. Such pairs of programs will arise
frequently in subsequent chapters.

      0                          OPTf                 OPT                   ∞✲

                                                        ✛                     ✲
                                                        primal integral solutions
      ✛                           ✲✛                                          ✲
          dual fractional solutions         primal fractional solutions


     At this point, we can state the lower bounding scheme being used by
Algorithm 2.2. Denote by OPTf the cost of an optimal fractional set cover,
i.e., an optimal solution to LP (13.2). Clearly OPTf ≤ OPT, the cost of
an optimal (integral) set cover. The cost of any feasible solution to the dual
program, LP (13.3), is a lower bound on OPTf , and hence also on OPT.
Algorithm 2.2 uses this as the lower bound.
     Algorithm 2.2 deﬁnes dual variables price(e), for each element, e. Observe
that the cover picked by the algorithm is fully payed for by this dual solution.
However, in general, this dual solution is not feasible (see Exercise 13.2). We
will show below that if this dual is shrunk by a factor of Hn , it ﬁts into the
given set cover instance, i.e., no set is overpacked. For each element e deﬁne,

               price(e)
      ye =              .
                 Hn

Algorithm 2.2 uses the dual feasible solution, y, as the lower bound on OPT.
Lemma 13.2 The vector y deﬁned above is a feasible solution for the dual
program (13.3).

Proof: We need to show that no set is overpacked by the solution y. Consider
a set S ∈ S consisting of k elements. Number the elements in the order
in which they are covered by the algorithm, breaking ties arbitrarily, say
e1 , . . . , ek .
      Consider the iteration in which the algorithm covers element ei . At this
point, S contains at least k −i+1 uncovered elements. Thus, in this iteration,
S itself can cover ei at an average cost of at most c(S)/(k − i + 1). Since
the algorithm chose the most cost-eﬀective set in this iteration, price(ei ) ≤
c(S)/(k − i + 1). Thus,

                1   c(S)
      y ei ≤      ·         .
               Hn k − i + 1

Summing over all elements in S,
      13.1    Dual-ﬁtting-based analysis for the greedy set cover algorithm           111
      k
                                                        
                    c(S)         1   1         1                  Hk
            yei ≤        ·         +   + ··· +                =      · c(S) ≤ c(S).
      i=1
                     Hn          k k−1         1                  Hn

Therefore, S is not overpacked.                                                        ✷

Theorem 13.3 The approximation guarantee of the greedy set cover algo-
rithm is Hn .

Proof: The cost of the set cover picked is
                                             
                                    
            price(e) = Hn                ye       ≤ Hn · OPT,
      e∈U                         e∈U


where OPT denotes the cost of the optimal fractional set cover. The last
inequality follows from the fact that y is dual feasible.             ✷


13.1.1      Can the approximation guarantee be improved?

Consider the three questions raised in Section 1.1.2 regarding improving the
approximation guarantee for vertex cover. Let us ask analogous questions for
set cover. The ﬁrst and third questions are already answered in Section 2.1.
    As a corollary of Theorem 13.3 we get an upper bound of Hn on the
integrality gap of relaxation (13.2). Example 13.4 shows that this bound is
essentially tight. Since the integrality gap of the LP-relaxation used bounds
the best approximation factor one can hope to achieve using this relaxation,
the answer to the second question is also essentially “no”.
Example 13.4 Consider the following set cover instance. Let n = 2k − 1,
where k is a positive integer, and let U = {e1 , e2 , . . . , en }. For 1 ≤ i ≤ n,
consider i written as a k-bit number. We can view this as a k-dimensional
vector over GF [2]. Let i denote this vector. For 1 ≤ i ≤ n deﬁne set Si =
{ej | i · j = 1}, where i · j denotes the inner product of these two vectors.
Finally, let S = {S1 , . . . , Sn }, and deﬁne the cost of each set to be 1.
    It is easy to check that each set contains 2k−1 = (n + 1)/2 elements, and
each element is contained in (n + 1)/2 sets. Thus, xi = 2/(n + 1), 1 ≤ i ≤ n,
is a fractional set cover. Its cost is 2n/(n + 1).
    Next, we will show that any integral set cover must pick at least k of
the sets. Consider the union of any p sets, where p < k. Let i1 , . . . , ip be
the indices of these p sets, and let A be a p × k matrix over GF [2] whose
rows consist of vectors i1 , . . . , ip , respectively. Since the rank of A is < k, the
dimension of its null space is ≥ 1, and so the null space contains a nonzero
vector, say j. Since Aj = 0, the element ej is not in any of the p sets. Hence
the p sets do not form a cover.
112      13   Set Cover via Dual Fitting

   Therefore, any integral set cover has cost at least k = log2 (n + 1). Hence,
the lower bound on the integrality gap established by this example is
               
          n+1                          log2 n
                    · log2 (n + 1) >          .
           2n                            2

                                                                               ✷



13.2 Generalizations of set cover
The greedy algorithm and its analysis using dual ﬁtting extend naturally to
several generalizations of the set cover problem (see Exercise 13.4).
• Set multicover: Each element, e, needs to be covered a speciﬁed integer
  number, re , of times. The objective again is to cover all elements up to their
  coverage requirements at minimum cost. We will assume that the cost of
  picking a set S k times is kcost(S).
• Multiset multicover: We are given a collection of multisets, rather than
  sets, of U . A multiset contains a speciﬁed number of copies of each element.
  Let M (S, e) denote the multiplicity of element e in set S. The instance
  satisﬁes the condition that the multiplicity of an element in a set is at
  most its coverage requirement, i.e., ∀S, e M (S, e) ≤ re . The objective is
  the same as before.
• Covering integer programs: These are integer programs of the form

          minimize       c·x

          subject to     Ax ≥ b,

  where all entries in A, b, c are nonnegative and x is required to be nonneg-
  ative and integral.

13.2.1     Dual ﬁtting applied to constrained set multicover

In this section, we will present an Hn factor approximation algorithm for set
multicover with the additional constraint that each set can be picked at most
once. Let us call this the constrained set multicover problem. One interesting
feature of this problem is that its linear relaxation and dual contain negative
coeﬃcients and thus do not form a covering-packing pair of LP’s.
    Let re ∈ Z+ be the coverage requirement for each element e ∈ U . The
integer programming formulation of constrained set multicover is not very
diﬀerent from that of set cover.
                                             13.2   Generalizations of set cover     113
                   
     minimize            c(S)xS                                                    (13.4)
                   S∈S
                    
     subject to             xS ≥ re ,       e∈U
                   S: e∈S
                   xS ∈ {0, 1},             S∈S

Notice, however, that in the LP-relaxation, the constraints xS ≤ 1 are no
longer redundant. If we drop them, then a set may be picked multiple times
to satisfy the coverage requirement of the elements. Thus, the LP-relaxation
looks diﬀerent from that for set cover. In particular, because of the negative
numbers in the constraint matrix and the right-hand side, it is not even
a covering linear program. The analysis given below deals with this added
complexity.
                    
      minimize          c(S)xS                                          (13.5)
                   S∈S
                    
     subject to             xS ≥ re ,       e∈U
                   S: e∈S
                   −xS ≥ −1,                S∈S
                   xS ≥ 0,                  S∈S

    The additional constraints in the primal lead to new variables, zS , in
the dual. The dual also has negative numbers in the constraint matrix and
is not a packing program. Now, a set S can be overpacked with the ye ’s.
However, this can be done only if we raise zS to ensure feasibility, which in
turn decreases the objective function value. Overall, overpacking may still be
advantageous, since the ye ’s appear with coeﬃcients of re in the objective
function.
                               
      maximize         re y e −   zS                                   (13.6)
                   e∈U             S∈S
                                  
                       
     subject to               ye       − zS ≤ c(S),     S∈S
                     e: e∈S
                     ye ≥ 0,                            e∈U
                     zS ≥ 0,                            S∈S

    The algorithm is again greedy. Let us say that element e is alive if it
occurs in fewer than re of the picked sets. In each iteration, the algorithm
picks, from amongst the currently unpicked sets, the most cost-eﬀective set,
where the cost-eﬀectiveness of a set is deﬁned to be the average cost at which
it covers alive elements. The algorithm halts when there are no more alive
elements, i.e., each element has been covered to the extent of its requirement.
114    13     Set Cover via Dual Fitting

      When a set S is picked, its cost is distributed equally among the alive
elements it covers as follows: if S covers e for the jth time, we set price(e, j)
to the current cost-eﬀectiveness of S. Clearly, the cost-eﬀectiveness of sets
picked is nondecreasing. Hence, for each element e, price(e, 1) ≤ price(e, 2) ≤
. . . ≤ price(e, re ).
      At the end of the algorithm, the dual variables are set as follows: For each
e ∈ U , let αe = (1/Hn ) · price(e, re ). For each S ∈ S that is picked by the
algorithm, let
                                                              
            1         
      βS =    ·                 (price(e, re ) − price(e, je )) ,
           Hn
                 e covered by S

where je is the copy of e that is covered by S. Notice that since price(e, je ) ≤
price(e, re ), βS is nonnegative. If S is not picked by the algorithm, βS is
deﬁned to be 0.

Lemma 13.5 The multicover picked by the algorithm is fully paid for by the
dual solution (α, β).

Proof: Since the cost of the sets picked by the algorithm is distributed
among the covered elements, it follows that the total cost of the multicover
produced by the algorithm is
       re
      
                price(e, j).
      e∈U j=1


The objective function value of the dual solution (α, β) is

                                  re
                                   
            re αe −         βS =             price(e, j).
      e∈U             S∈S          e∈U j=1


The lemma follows.                                                              ✷
   The dual solution deﬁned above is, in general, infeasible. We will show
that when scaled by a factor of Hn , a feasible solution results. Deﬁne for each
element e ∈ U and each set S ∈ S,

             αe          βS
      ye =      and zS =    .
             Hn          Hn

Lemma 13.6 The pair (y, z) is a feasible solution for the dual program
(13.6).
                                                 13.2   Generalizations of set cover     115

Proof: Consider a set S ∈ S consisting of k elements. Number its elements
in the order in which their requirements are fulﬁlled, i.e., the order in which
they stopped being alive. Let the ordered elements be e1 , . . . , ek .
    First, assume that S is not picked by the algorithm. When the algorithm
is about to cover the last copy of ei , S contains at least k−i+1 alive elements,
so

                              c(S)
      price(ei , rei ) ≤           .
                             k−i+1

Since zS is zero, we get
       k                          k
                               1 
              y ei       − zS =        price(ei , rei )
        i=1
                               Hn i=1
                                                               
                               c(S)     1      1              1
                             ≤      ·     +           + ··· +     ≤ c(S) .
                                Hn      k k−1                 1

     Next, assume that S is picked by the algorithm, and before this happens,
k  ≥ 0 elements of S are already completely covered. Then
       k            
       
              yei  − zS
        i=1           $ k                       k
                                                                                           %
                 1                            
              =     ·      price(ei , rei ) −          (price(ei , rei ) − price(ei , ji ))
                Hn
                       
                       i=1                    i=k  +1
                                                                      
                        k                        k
                 1                            
              =     ·      price(ei , rei ) +          price(ei , ji ) ,
                Hn     i=1                        i=k +1


where S covers the ji th copy of ei , for each i ∈ {k  + 1, . . . , k}.
     k
But i=k +1 price(ei , ji ) = cost(S), since the cost of S is equally distributed
among the copies it covers. Finally consider elements ei , i ∈ {1, . . . , k }. When
the last copy of ei is being covered, S is not yet picked and covers at least
k − i + 1 alive elements. Thus, price(ei , rei ) ≤ c(S)/(k − i + 1). Therefore,
       k                                                          
                               c(S)         1             1
              y ei       − zS ≤      ·         + ··· +            + 1 ≤ c(S).
        i=1
                                 Hn          k         k − k + 1

Hence, (y, z) is feasible for the dual program.                                            ✷

Theorem 13.7 The greedy algorithm achieves an approximation guarantee
of Hn for the constrained set multicover problem.
116    13     Set Cover via Dual Fitting

Proof: By Lemmas 13.5 and 13.6, the total cost of the multicover produced
by the algorithm is
                                        $                           %
                                                          
            re αe −         βS = Hn ·             re y e −         zS ≤ Hn · OPT.
      e∈U             S∈S                   e∈U              S∈S

                                                                                    ✷
   Observe that as a corollary of Theorem 13.7 we get that the integrality
gap of LP (13.5) is bounded by Hn . In contrast, the integrality gap of the
corresponding LP for multiset multicover, with the restriction that each set
be picked at most once, is not bounded by any function of n (see Exercise
13.5).


13.3 Exercises

13.1 Show that the dual-ﬁtting-based analysis for the greedy set cover and
constrained set multicover algorithms actually establishes an approximation
guarantee of Hk , where k is size of the largest set in the given instance.
(Notice the ease with which this can be established using the LP-duality
approach; compare with Exercise 2.8.)

13.2 Give an example in which the dual solution, price(e), for each element e,
computed by Algorithm 2.2 overpacks some sets, S, by a factor of essentially
H|S| .

13.3 Give examples to show that the lower bound used by Algorithm 2.2,
y, can be smaller than OPT by a factor of O(log n).

13.4 Give the following approximation algorithms.
1. Hn factor for set multicover.
2. Hm factor for multiset multicover, where m is the size of the largest
   multiset in the given instance (the size of a multiset counts elements
   with multiplicity).
3. O(log n) factor for covering integer programs.
Hint: For Hm factor algorithm for multiset multicover, set the dual variables
according to the average price for covering elements, i.e.,
                 re
              1 
      ye =          price(e, i)/re .
             Hm i=1
                                                           13.3   Exercises     117

Use scaling and rounding to reduce covering integer programs to multiset
multicover, with m polynomially bounded in n, at the expense of a small
error (which goes into the approximation factor).

13.5 Show that the integrality gap of the relaxation for the following two
variants of multiset multicover, based on LP (13.2), is not bounded by any
function of n.
 1. Remove the restriction that M (S, e) ≤ re .
 2. Impose the constraint that each set can be picked at most once.
What is the best approximation guarantee you can establish for the greedy
algorithm for the second variant. Why does the proof of factor Hn given in
Section 13.2 not extend to this case?

13.6 (Mihail [206]) Consider the following variant on the set multicover
problem. Let U be the universal set, |U | = n, and S a collection of subsets
of U . For each S ∈ S, its cost is given as a function of time, t ∈ {1, . . . , T }.
Each of these cost functions is nonincreasing with time. In addition, for each
element in U , a coverage requirement is speciﬁed, again as a function of
time; these functions are nondecreasing with time. The problem is to pick
sets at a minimum total cost so that the coverage requirements are satisﬁed
for each element at each time. A set can be picked any number of times;
the cost of picking a set depends on the time at which it is picked. Once
picked, the set remains in the cover for all future times at no additional cost.
Give an Hn factor algorithm for this problem. (An H(n·T ) factor algorithm
is straightforward.)

13.7 In many realistic situations, the cost of picking an item a multiple
number of times does not grow linearly. Instead it is given by a concave
function. The following variant of the set multicover problem models this
situation. For each set Si we are given a concave function fi specifying the cost
of picking this set multiple times. The problem again is to satisfy all coverage
requirements of elements at minimum cost. Give a factor Hn algorithm for
this problem.
Hint: Reduce the problem to a multiset multicover problem. For each set Si ,
construct sets Sij , j ≥ 1. Set Sij contains each element of Si with multiplicity
j and has a cost of fi (j). The greedy algorithm run on this instance achieves
the required factor. Next show that there is no need to explicitly construct all
the sets Sij . In each iteration of the greedy algorithm, the most cost-eﬀective
set can be computed directly in polynomial time, even if the requirements
are exponentially large.
118    13   Set Cover via Dual Fitting

13.4 Notes
The dual-ﬁtting-based analysis of set cover is due to Lovász [192] and Chvátal
[48]. The analysis of constrained set multicover is due to Rajagopalan and
Vazirani [227]. For algorithms for covering integer programs, see Dobson [61]
and Rajagopalan and Vazirani [227].
