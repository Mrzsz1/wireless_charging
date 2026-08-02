---
type: "book-chapter"
book_id: "approximation-algorithms"
chapter_id: "front-matter"
chapter_number: 0
chapter_title: "Front matter"
source_pdf: "raw/inbox/manual-drop/PDF_A.pdf"
source_page_start: 1
source_page_end: 18
printed_page_start: null
printed_page_end: null
part_ids: ["approximation-algorithms-front-matter-part-001"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Front matter

Vijay V. Vazirani


College of Computing
Georgia Institute of Technology
          c 2001
Copyright 




Approximation Algorithms




Springer
Berlin Heidelberg NewYork
Barcelona Hong Kong
London Milan Paris
Singapore Tokyo
To my parents
Preface



                                  Although this may seem a paradox, all exact
                            science is dominated by the idea of approximation.
                                                Bertrand Russell (1872–1970)


Most natural optimization problems, including those arising in important
application areas, are NP-hard. Therefore, under the widely believed con-
jecture that P = NP, their exact solution is prohibitively time consuming.
Charting the landscape of approximability of these problems, via polynomial
time algorithms, therefore becomes a compelling subject of scientiﬁc inquiry
in computer science and mathematics. This book presents the theory of ap-
proximation algorithms as it stands today. It is reasonable to expect the
picture to change with time.
    The book is divided into three parts. In Part I we cover a combinato-
rial algorithms for a number of important problems, using a wide variety
of algorithm design techniques. The latter may give Part I a non-cohesive
appearance. However, this is to be expected – nature is very rich, and we
cannot expect a few tricks to help solve the diverse collection of NP-hard
problems. Indeed, in this part, we have purposely refrained from tightly cat-
egorizing algorithmic techniques so as not to trivialize matters. Instead, we
have attempted to capture, as accurately as possible, the individual character
of each problem, and point out connections between problems and algorithms
for solving them.
    In Part II, we present linear programming based algorithms. These are
categorized under two fundamental techniques: rounding and the primal–
dual schema. But once again, the exact approximation guarantee obtainable
depends on the speciﬁc LP-relaxation used, and there is no ﬁxed recipe for
discovering good relaxations, just as there is no ﬁxed recipe for proving a the-
orem in mathematics (readers familiar with complexity theory will recognize
this as the philosophical point behind the P = NP question).
    Part III covers four important topics. The ﬁrst is the problem of ﬁnding
a shortest vector in a lattice which, for several reasons, deserves individual
treatment (see Chapter 27).
    The second topic is the approximability of counting, as opposed to
optimization, problems (counting the number of solutions to a given in-
stance). The counting versions of all known NP-complete problems are #P-
complete1 . Interestingly enough, other than a handful of exceptions, this is
true of problems in P as well. An impressive theory has been built for ob-
1
    However, there is no theorem to this eﬀect yet.
VIII   Preface

taining eﬃcient approximate counting algorithms for this latter class of prob-
lems. Most of these algorithms are based on the Markov chain Monte Carlo
(MCMC) method, a topic that deserves a book by itself and is therefore not
treated here. In Chapter 28 we present combinatorial algorithms, not using
the MCMC method, for two fundamental counting problems.
    The third topic is centered around recent breakthrough results, estab-
lishing hardness of approximation for many key problems, and giving new
legitimacy to approximation algorithms as a deep theory. An overview of
these results is presented in Chapter 29, assuming the main technical theo-
rem, the PCP Theorem. The latter theorem, unfortunately, does not have a
simple proof at present.
    The fourth topic consists of the numerous open problems of this young
ﬁeld. The list presented should by no means be considered exhaustive, and
is moreover centered around problems and issues currently in vogue. Exact
algorithms have been studied intensively for over four decades, and yet basic
insights are still being obtained. Considering the fact that among natural
computational problems, polynomial time solvability is the exception rather
than the rule, it is only reasonable to expect the theory of approximation
algorithms to grow considerably over the years.
    The set cover problem occupies a special place, not only in the theory of
approximation algorithms, but also in this book. It oﬀers a particularly simple
setting for introducing key concepts as well as some of the basic algorithm
design techniques of Part I and Part II. In order to give a complete treatment
for this central problem, in Part III we give a hardness result for it, even
though the proof is quite elaborate. The hardness result essentially matches
the guarantee of the best algorithm known – this being another reason for
presenting this rather diﬃcult proof.
    Our philosophy on the design and exposition of algorithms is nicely il-
lustrated by the following analogy with an aspect of Michelangelo’s art. A
major part of his eﬀort involved looking for interesting pieces of stone in the
quarry and staring at them for long hours to determine the form they natu-
rally wanted to take. The chisel work exposed, in a minimalistic manner, this
form. By analogy, we would like to start with a clean, simply stated problem
(perhaps a simpliﬁed version of the problem we actually want to solve in
practice). Most of the algorithm design eﬀort actually goes into understand-
ing the algorithmically relevant combinatorial structure of the problem. The
algorithm exploits this structure in a minimalistic manner. The exposition of
algorithms in this book will also follow this analogy, with emphasis on stating
the structure oﬀered by problems, and keeping the algorithms minimalistic.
    An attempt has been made to keep individual chapters short and simple,
often presenting only the key result. Generalizations and related results are
relegated to exercises. The exercises also cover other important results which
could not be covered in detail due to logistic constraints. Hints have been
                                                                 Preface     IX

provided for some of the exercises; however, there is no correlation between
the degree of diﬃculty of an exercise and whether a hint is provided for it.
    This book is suitable for use in advanced undergraduate and graduate level
courses on approximation algorithms. It has more than twice the material
that can be covered in a semester long course, thereby leaving plenty of room
for an instructor to choose topics. An undergraduate course in algorithms
and the theory of NP-completeness should suﬃce as a prerequisite for most
of the chapters. For completeness, we have provided background information
on several topics: complexity theory in Appendix A, probability theory in
Appendix B, linear programming in Chapter 12, semideﬁnite programming in
Chapter 26, and lattices in Chapter 27. (A disproportionate amount of space
has been devoted to the notion of self-reducibility in Appendix A because
this notion has been quite sparsely treated in other sources.) This book can
also be used is as supplementary text in basic undergraduate and graduate
algorithms courses. The ﬁrst few chapters of Part I and Part II are suitable
for this purpose. The ordering of chapters in both these parts is roughly by
increasing diﬃculty.
    In anticipation of this wide audience, we decided not to publish this book
in any of Springer’s series – even its prestigious Yellow Series. (However, we
could not resist spattering a patch of yellow on the cover!) The following
translations are currently planned: French by Claire Kenyon, Japanese by
Takao Asano, and Romanian by Ion Măndoiu. Corrections and comments
from readers are welcome. We have set up a special email address for this
purpose: approx@cc.gatech.edu.
    Finally, a word about practical impact. With practitioners looking for
high performance algorithms having error within 2% or 5% of the optimal,
what good are algorithms that come within a factor of 2, or even worse,
O(log n), of the optimal? Further, by this token, what is the usefulness of
improving the approximation guarantee from, say, factor 2 to 3/2?
    Let us address both issues and point out some fallacies in these assertions.
The approximation guarantee only reﬂects the performance of the algorithm
on the most pathological instances. Perhaps it is more appropriate to view
the approximation guarantee as a measure that forces us to explore deeper
into the combinatorial structure of the problem and discover more powerful
tools for exploiting this structure. It has been observed that the diﬃculty
of constructing tight examples increases considerably as one obtains algo-
rithms with better guarantees. Indeed, for some recent algorithms, obtaining
a tight example has been a paper by itself (e.g., see Section 26.7). Experi-
ments have conﬁrmed that these and other sophisticated algorithms do have
error bounds of the desired magnitude, 2% to 5%, on typical instances, even
though their worst case error bounds are much higher. Additionally, the the-
oretically proven algorithm should be viewed as a core algorithmic idea that
needs to be ﬁne tuned to the types of instances arising in speciﬁc applications.
X      Preface

   We hope that this book will serve as a catalyst in helping this theory grow
and have practical impact.


Acknowledgments
This book is based on courses taught at the Indian Institute of Technology,
Delhi in Spring 1992 and Spring 1993, at Georgia Tech in Spring 1997, Spring
1999, and Spring 2000, and at DIMACS in Fall 1998. The Spring 1992 course
resulted in the ﬁrst set of class notes on this topic. It is interesting to note
that more than half of this book is based on subsequent research results.
    Numerous friends – and family members – have helped make this book a
reality. First, I would like to thank Naveen Garg, Kamal Jain, Ion Măndoiu,
Sridhar Rajagopalan, Huzur Saran, and Mihalis Yannakakis – my extensive
collaborations with them helped shape many of the ideas presented in this
book. I was fortunate to get Ion Măndoiu’s help and advice on numerous
matters – his elegant eye for layout and ﬁgures helped shape the presentation.
A special thanks, Ion!
    I would like to express my gratitude to numerous experts in the ﬁeld for
generous help on tasks ranging all the way from deciding the contents and
its organization, providing feedback on the writeup, ensuring correctness and
completeness of references to designing exercises and helping list open prob-
lems. Thanks to Sanjeev Arora, Alan Frieze, Naveen Garg, Michel Goemans,
Mark Jerrum, Claire Kenyon, Samir Khuller, Daniele Micciancio, Yuval Ra-
bani, Sridhar Rajagopalan, Dana Randall, Tim Roughgarden, Amin Saberi,
Leonard Schulman, Amin Shokrollahi, and Mihalis Yannakakis, with special
thanks to Kamal Jain, Éva Tardos, and Luca Trevisan.
    Numerous other people helped with valuable comments and discussions.
In particular, I would like to thank Sarmad Abbasi, Cristina Bazgan, Rogerio
Brito Gruia Calinescu, Amit Chakrabarti, Mosses Charikar, Joseph Cheriyan,
Vasek Chvátal, Uri Feige, Cristina Fernandes, Ashish Goel, Parikshit Gopalan,
Mike Grigoriadis, Sudipto Guha, Dorit Hochbaum, Howard Karloﬀ, Leonid
Khachian, Stavros Kolliopoulos, Jan van Leeuwen, Nati Lenial, George
Leuker, Vangelis Markakis, Aranyak Mehta, Rajeev Motwani, Prabhakar
Raghavan, Satish Rao, Miklos Santha, Jiri Sgall, David Shmoys, Alistair
Sinclair, Prasad Tetali, Pete Veinott, Ramarathnam Venkatesan, Nisheeth
Vishnoi, and David Williamson. I am sure I am missing several names – my
apologies and thanks to these people as well. A special role was played by
the numerous students who took my courses on this topic and scribed notes.
It will be impossible to individually remember their names. I would like to
express my gratitude collectively to them.
    I would like to thank IIT Delhi – with special thanks to Shachin Mahesh-
wari – Georgia Tech, and DIMACS for providing pleasant, supportive and
academically rich environments. Thanks to NSF for support under grants
CCR-9627308 and CCR-9820896.
                                                             Preface     XI

    It was a pleasure to work with Hans Wössner on editorial matters. The
personal care with which he handled all such matters and his sensitivity to
an author’s unique point of view were especially impressive. Thanks also to
Frank Holzwarth for sharing his expertise with LATEX.
    A project of this magnitude would be hard to pull oﬀ without whole-
hearted support from family members. Fortunately, in my case, some of them
are also fellow researchers – my wife, Milena Mihail, and my brother, Umesh
Vazirani. Little Michel’s arrival, halfway through this project, brought new
joys and energies, though made the end even more challenging! Above all,
I would like to thank my parents for their unwavering support and inspira-
tion – my father, a distinguished author of several Civil Engineering books,
and my mother, with her deep understanding of Indian Classical Music. This
book is dedicated to them.

Atlanta, Georgia, May 2001                                   Vijay Vazirani
Table of Contents




1   Introduction . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 1
    1.1 Lower bounding OPT . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 2
        1.1.1 An approximation algorithm for cardinality vertex cover 3
        1.1.2 Can the approximation guarantee be improved? . . . . . . 3
    1.2 Well-characterized problems and min–max relations . . . . . . . . . 5
    1.3 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 7
    1.4 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 10


Part I. Combinatorial Algorithms

2   Set Cover . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .   15
    2.1 The greedy algorithm . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                16
    2.2 Layering . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .      17
    2.3 Application to shortest superstring . . . . . . . . . . . . . . . . . . . . . . .                         19
    2.4 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .     22
    2.5 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .   26

3   Steiner Tree and TSP . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                  27
    3.1 Metric Steiner tree . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .             27
        3.1.1 MST-based algorithm . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                       28
    3.2 Metric TSP . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .        30
        3.2.1 A simple factor 2 algorithm . . . . . . . . . . . . . . . . . . . . . . . .                         31
        3.2.2 Improving the factor to 3/2 . . . . . . . . . . . . . . . . . . . . . . . .                         32
    3.3 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .     33
    3.4 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .   37

4   Multiway Cut and k-Cut . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                      38
    4.1 The multiway cut problem . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                    38
    4.2 The minimum k-cut problem . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                       40
    4.3 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .     44
    4.4 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .   46
XIV        Table of Contents

5      k-Center . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .   47
       5.1 Parametric pruning applied to metric k-center . . . . . . . . . . . . . .                                  47
       5.2 The weighted version . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                 50
       5.3 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .      52
       5.4 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .    53

6      Feedback Vertex Set . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                54
       6.1 Cyclomatic weighted graphs . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                       54
       6.2 Layering applied to feedback vertex set . . . . . . . . . . . . . . . . . . . .                            57
       6.3 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .      60
       6.4 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .    60

7      Shortest Superstring . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .               61
       7.1 A factor 4 algorithm . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .               61
       7.2 Improving to factor 3 . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                64
           7.2.1 Achieving half the optimal compression . . . . . . . . . . . . .                                     66
       7.3 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .      66
       7.4 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .    67

8      Knapsack . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .     68
       8.1 A pseudo-polynomial time algorithm for knapsack . . . . . . . . . .                                        69
       8.2 An FPTAS for knapsack . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                      69
       8.3 Strong NP-hardness and the existence of FPTAS’s . . . . . . . . .                                          71
           8.3.1 Is an FPTAS the most desirable approximation
                  algorithm? . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .            72
       8.4 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .      72
       8.5 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .    73

9      Bin Packing . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .        74
       9.1 An asymptotic PTAS . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                   74
       9.2 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .      77
       9.3 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .    78

10 Minimum Makespan Scheduling . . . . . . . . . . . . . . . . . . . . . . . . . .                                    79
   10.1 Factor 2 algorithm . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                79
   10.2 A PTAS for minimum makespan . . . . . . . . . . . . . . . . . . . . . . . . .                                 80
        10.2.1 Bin packing with ﬁxed number of object sizes . . . . . . . .                                           81
        10.2.2 Reducing makespan to restricted bin packing . . . . . . . .                                            81
   10.3 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .         83
   10.4 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .       83

11 Euclidean TSP . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                84
   11.1 The algorithm . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .               84
   11.2 Proof of correctness . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .                87
   11.3 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .         89
   11.4 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .       89
                                                                               Table of Contents              XV


Part II. LP-Based Algorithms

12 Introduction to LP-Duality . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 93
   12.1 The LP-duality theorem . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 93
   12.2 Min–max relations and LP-duality . . . . . . . . . . . . . . . . . . . . . . . . 97
   12.3 Two fundamental algorithm design techniques . . . . . . . . . . . . . . 100
        12.3.1 A comparison of the techniques and the notion of
               integrality gap . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 101
   12.4 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 103
   12.5 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 107

13 Set Cover via Dual Fitting . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 108
   13.1 Dual-ﬁtting-based analysis for the greedy set cover algorithm 108
        13.1.1 Can the approximation guarantee be improved? . . . . . . 111
   13.2 Generalizations of set cover . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 112
        13.2.1 Dual ﬁtting applied to constrained set multicover . . . . . 112
   13.3 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 116
   13.4 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 118

14 Rounding Applied to Set Cover . . . . . . . . . . . . . . . . . . . . . . . . . . . 119
   14.1 A simple rounding algorithm . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 119
   14.2 Randomized rounding . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 120
   14.3 Half-integrality of vertex cover . . . . . . . . . . . . . . . . . . . . . . . . . . . 122
   14.4 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 123
   14.5 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 124

15 Set Cover via the Primal–Dual Schema . . . . . . . . . . . . . . . . . . . 125
   15.1 Overview of the schema . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 125
   15.2 Primal–dual schema applied to set cover . . . . . . . . . . . . . . . . . . . 127
   15.3 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 129
   15.4 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 129

16 Maximum Satisﬁability . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 131
   16.1 Dealing with large clauses . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 132
   16.2 Derandomizing via the method of conditional expectation . . . 132
   16.3 Dealing with small clauses via LP-rounding . . . . . . . . . . . . . . . . 134
   16.4 A 3/4 factor algorithm . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 136
   16.5 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 137
   16.6 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 139

17 Scheduling on Unrelated Parallel Machines . . . . . . . . . . . . . . . 140
   17.1 Parametric pruning in an LP setting . . . . . . . . . . . . . . . . . . . . . . 140
   17.2 Properties of extreme point solutions . . . . . . . . . . . . . . . . . . . . . . 141
   17.3 The algorithm . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 142
XVI        Table of Contents

      17.4 Additional properties of extreme point solutions . . . . . . . . . . . . 143
      17.5 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 144
      17.6 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 145

18 Multicut and Integer Multicommodity Flow in Trees . . . . . 146
   18.1 The problems and their LP-relaxations . . . . . . . . . . . . . . . . . . . . 146
   18.2 Primal–dual schema based algorithm . . . . . . . . . . . . . . . . . . . . . . 149
   18.3 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 152
   18.4 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 154

19 Multiway Cut . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 155
   19.1 An interesting LP-relaxation . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 155
   19.2 Randomized rounding algorithm . . . . . . . . . . . . . . . . . . . . . . . . . . 157
   19.3 Half-integrality of node multiway cut . . . . . . . . . . . . . . . . . . . . . 160
   19.4 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 163
   19.5 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 167

20 Multicut in General Graphs . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 168
   20.1 Sum multicommodity ﬂow . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 168
   20.2 LP-rounding-based algorithm . . . . . . . . . . . . . . . . . . . . . . . . . . . . 170
        20.2.1 Growing a region: the continuous process . . . . . . . . . . . . 171
        20.2.2 The discrete process . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 172
        20.2.3 Finding successive regions . . . . . . . . . . . . . . . . . . . . . . . . . 173
   20.3 A tight example . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 175
   20.4 Some applications of multicut . . . . . . . . . . . . . . . . . . . . . . . . . . . . 176
   20.5 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 177
   20.6 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 179

21 Sparsest Cut . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 180
   21.1 Demands multicommodity ﬂow . . . . . . . . . . . . . . . . . . . . . . . . . . . 180
   21.2 Linear programming formulation . . . . . . . . . . . . . . . . . . . . . . . . . 181
   21.3 Metrics, cut packings, and 1 -embeddability . . . . . . . . . . . . . . . . 183
        21.3.1 Cut packings for metrics . . . . . . . . . . . . . . . . . . . . . . . . . . 183
        21.3.2 1 -embeddability of metrics . . . . . . . . . . . . . . . . . . . . . . . . 185
   21.4 Low distortion 1 -embeddings for metrics . . . . . . . . . . . . . . . . . . 186
        21.4.1 Ensuring that a single edge is not overshrunk . . . . . . . . 187
        21.4.2 Ensuring that no edge is overshrunk . . . . . . . . . . . . . . . . 190
   21.5 LP-rounding-based algorithm . . . . . . . . . . . . . . . . . . . . . . . . . . . . 191
   21.6 Applications . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 192
        21.6.1 Edge expansion . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 192
        21.6.2 Conductance . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 192
        21.6.3 Balanced cut . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 193
        21.6.4 Minimum cut linear arrangement . . . . . . . . . . . . . . . . . . . 194
   21.7 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 195
   21.8 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 197
                                                                               Table of Contents           XVII

22 Steiner Forest . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 198
   22.1 LP-relaxation and dual . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 198
   22.2 Primal–dual schema with synchronization . . . . . . . . . . . . . . . . . 199
   22.3 Analysis . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 204
   22.4 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 207
   22.5 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 212

23 Steiner Network . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 213
   23.1 The LP-relaxation and half-integrality . . . . . . . . . . . . . . . . . . . . 213
   23.2 The technique of iterated rounding . . . . . . . . . . . . . . . . . . . . . . . 217
   23.3 Characterizing extreme point solutions . . . . . . . . . . . . . . . . . . . . 219
   23.4 A counting argument . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 221
   23.5 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 224
   23.6 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 231

24 Facility Location . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 232
   24.1 An intuitive understanding of the dual . . . . . . . . . . . . . . . . . . . . 233
   24.2 Relaxing primal complementary slackness conditions . . . . . . . . 234
   24.3 Primal–dual schema based algorithm . . . . . . . . . . . . . . . . . . . . . . 235
   24.4 Analysis . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 236
        24.4.1 Running time . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 238
        24.4.2 Tight example . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 238
   24.5 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 239
   24.6 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 242

25 k-Median . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 243
   25.1 LP-relaxation and dual . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 243
   25.2 The high-level idea . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 244
   25.3 Randomized rounding . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 247
        25.3.1 Derandomization . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 248
        25.3.2 Running time . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 249
        25.3.3 Tight example . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 249
        25.3.4 Integrality gap . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 250
   25.4 A Lagrangian relaxation technique
        for approximation algorithms . . . . . . . . . . . . . . . . . . . . . . . . . . . . 250
   25.5 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 251
   25.6 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 254

26 Semideﬁnite Programming . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 255
   26.1 Strict quadratic programs and vector programs . . . . . . . . . . . . . 255
   26.2 Properties of positive semideﬁnite matrices . . . . . . . . . . . . . . . . 257
   26.3 The semideﬁnite programming problem . . . . . . . . . . . . . . . . . . . 258
   26.4 Randomized rounding algorithm . . . . . . . . . . . . . . . . . . . . . . . . . . 260
   26.5 Improving the guarantee for MAX-2SAT . . . . . . . . . . . . . . . . . . 263
   26.6 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 265
   26.7 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 268
XVIII Table of Contents


Part III. Other Topics

27 Shortest Vector . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 273
   27.1 Bases, determinants, and orthogonality defect . . . . . . . . . . . . . . 274
   27.2 The algorithms of Euclid and Gauss . . . . . . . . . . . . . . . . . . . . . . 276
   27.3 Lower bounding OPT using Gram–Schmidt orthogonalization 278
   27.4 Extension to n dimensions . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 280
   27.5 The dual lattice and its algorithmic use . . . . . . . . . . . . . . . . . . . 284
   27.6 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 288
   27.7 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 292

28 Counting Problems . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 294
   28.1 Counting DNF solutions . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 295
   28.2 Network reliability . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 297
        28.2.1 Upperbounding the number of near-minimum cuts . . . . 298
        28.2.2 Analysis . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 300
   28.3 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 302
   28.4 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 305

29 Hardness of Approximation . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 306
   29.1 Reductions, gaps, and hardness factors . . . . . . . . . . . . . . . . . . . . 306
   29.2 The PCP theorem . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 309
   29.3 Hardness of MAX-3SAT . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 311
   29.4 Hardness of MAX-3SAT with bounded occurrence
        of variables . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 313
   29.5 Hardness of vertex cover and Steiner tree . . . . . . . . . . . . . . . . . . 316
   29.6 Hardness of clique . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 318
   29.7 Hardness of set cover . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 322
        29.7.1 The two-prover one-round characterization of NP . . . . 322
        29.7.2 The gadget . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 324
        29.7.3 Reducing error probability by parallel repetition . . . . . . 325
        29.7.4 The reduction . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 326
   29.8 Exercises . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 329
   29.9 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 332

30 Open Problems . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 334
   30.1 Problems having constant factor algorithms . . . . . . . . . . . . . . . . 334
   30.2 Other optimization problems . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 336
   30.3 Counting problems . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 338
                                                                                 Table of Contents             XIX

Appendix

A      An Overview of Complexity Theory
       for the Algorithm Designer . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 343
       A.1 Certiﬁcates and the class NP . . . . . . . . . . . . . . . . . . . . . . . . . . . . 343
       A.2 Reductions and NP-completeness . . . . . . . . . . . . . . . . . . . . . . . . 344
       A.3 NP-optimization problems and approximation algorithms . . . 345
            A.3.1 Approximation factor preserving reductions . . . . . . . . . . 347
       A.4 Randomized complexity classes . . . . . . . . . . . . . . . . . . . . . . . . . . . 347
       A.5 Self-reducibility . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 348
       A.6 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 351

B      Basic Facts from Probability Theory . . . . . . . . . . . . . . . . . . . . . . 352
       B.1 Expectation and moments . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 352
       B.2 Deviations from the mean . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 353
       B.3 Basic distributions . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 354
       B.4 Notes . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 354

References . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 355

Problem Index . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 371

Subject Index . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 375
