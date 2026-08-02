---
type: "book-chapter"
book_id: "algorithmic-game-theory"
chapter_id: "ch-08"
chapter_number: 8
chapter_title: "Cryptography and Game Theory Yevgeniy Dodis and Tal Rabin"
source_pdf: "raw/inbox/manual-drop/PDF_B.pdf"
source_page_start: 202
source_page_end: 229
printed_page_start: 202
printed_page_end: 227
part_ids: ["algorithmic-game-theory-ch-08-part-009"]
ingest_engine: "mineru-precise-v4"
ingest_status: "pending_quality"
---

# Cryptography and Game Theory Yevgeniy Dodis and Tal Rabin

P1: SBT
9780521872829main       CUNY1061-Nisan        0 521 87282 0     July 5, 2007     14:16




                                                               CHAPTER 8


                           Cryptography and Game Theory

                                               Yevgeniy Dodis and Tal Rabin




                                                                  Abstract

                    The Cryptographic and Game Theory worlds seem to have an intersection in that they both deal with
                    an interaction between mutually distrustful parties which has some end result. In the cryptographic
                    setting the multiparty interaction takes the shape of a set of parties communicating for the purpose
                    of evaluating a function on their inputs, where each party receives at the end some output of the
                    computation. In the game theoretic setting, parties interact in a game that guarantees some payoff for
                    the participants according to the joint actions of all the parties, while the parties wish to maximize
                    their own payoff. In the past few years the relationship between these two areas has been investigated
                    with the hope of having cross fertilization and synergy. In this chapter we describe the two areas, the
                    similarities and differences, and some of the new results stemming from their interaction.
                        The first and second section will describe the cryptographic and the game theory settings (respec-
                    tively). In the third section we contrast the two settings, and in the last sections we detail some of the
                    existing results.




                                          8.1 Cryptographic Notions and Settings

                    Cryptography is a vast subject requiring its own book. Therefore, in the following
                    we will give only a high-level overview of the problem of Multi-Party Computation
                    (MPC), ignoring most of the lower-level details and concentrating only on aspects
                    relevant to Game Theory.
                         MPC deals with the following problem. There are n ≥ 2 parties P1 , . . . , Pn where
                    party Pi holds input ti , 1 ≤ i ≤ n, and they wish to compute together a function
                    s = f (t1 , . . . , tn ) on their inputs. The goal is that each party will learn the output of
                    the function, s, yet with the restriction that Pi will not learn any additional information
                    about the input of the other parties aside from what can be deduced from the pair
                    (ti , s). Clearly it is the secrecy restriction that adds complexity to the problem, as
                    without it each party could announce its input to all other parties, and each party would
                    locally compute the value of the function. Thus, the goal of MPC is to achieve the
                                                                      181
P1: SBT
9780521872829main      CUNY1061-Nisan           0 521 87282 0       July 5, 2007     14:16




                    182                            cryptography and game theory

                    following two properties at the same time: correctness of the computation and privacy
                    preservation of the inputs.
                    Two generalizations. The following two generalizations of the above scenario are often
                    useful.
                     (i) Probabilistic functions. Here the value of the function depends on some random string
                         r chosen according to some distribution: s = f (t1 , . . . , tn ; r). An example of this is
                         the coin-flipping functionality, which takes no inputs, and outputs an unbiased random
                         bit. Notice, it is crucial that the value r is not controlled by any of the parties, but is
                         somehow jointly generated during the computation.
                    (ii) Multioutput functions. It is not mandatory that there be a single output of the function.
                         More generally there could be a unique output for each party, i.e., (s1 , . . . , sn ) =
                         f (t1 , . . . , tn ). In this case, only party Pi learns the output si , and no other party learns
                         any information about the other parties input and outputs aside from what can be
                         derived from its own input and output.

                    The parties. One of the most interesting aspects of MPC is to reach the objective of
                    computing the function value, but under the assumption that some of the parties may
                    deviate from the protocol. In cryptography, the parties are usually divided into two
                    types: honest and faulty. An honest party follows the protocol without any deviation.
                    Otherwise, the party is considered to be faulty. The faulty behavior can exemplify itself
                    in a wide range of possibilities. The most benign faulty behavior is where the parties
                    follow the protocol, yet try to learn as much as possible about the inputs of the other
                    parties. These parties are called honest-but-curious (or semihonest). At the other end
                    of the spectrum, the parties may deviate from the prescribed protocol in any way that
                    they desire, with the goal of either influencing the computed output value in some way,
                    or of learning as much as possible about the inputs of the other parties. These parties
                    are called malicious.
                        We envision an adversary A, who controls all the faulty parties and can coordinate
                    their actions. Thus, in a sense we assume that the faulty parties are working together and
                    can exert the most knowledge and influence over the computation out of this collusion.
                    The adversary can corrupt any number of parties out of the n participating parties. Yet,
                    in order to be able to achieve a solution to the problem, in many cases we would need
                    to limit the number of corrupted parties. We call this limit a threshold k, indicating that
                    the protocol remains secure as long as the number of corrupted parties is at most k.


                                             8.1.1 Security of Multiparty Computations
                    We are ready to formulate the idea of what it means to securely compute a given
                    function f . Assume that there exists a trusted party who privately receives the inputs
                    of all the participating parties, calculates the output value s, and then transmits this
                    value to each one of the parties.1 This process clearly computes the correct output of
                    f , and also does not enable the participating parties to learn any additional information

                    1 Note that in the case of a probabilistic function the trusted party will choose r according to the specified

                      distribution and use it in the computation. Similarly, for multioutput functions the trusted party will only give
                      each party its own output.
P1: SBT
9780521872829main       CUNY1061-Nisan           0 521 87282 0      July 5, 2007      14:16




                                                cryptographic notions and settings                                     183

                    about the inputs of others. We call this model the ideal model. The security of MPC
                    then states that a protocol is secure if its execution satisfies the following: (1) the
                    honest parties compute the same (correct) outputs as they would in the ideal model;
                    and (2) the protocol does not expose more information than a comparable execution
                    with the trusted party, in the ideal model.
                        Intuitively, this is explained in the following way. The adversary’s interaction with
                    the parties (on a vector of inputs) in the protocol generates a transcript. This transcript
                    is a random variable that includes the outputs of all the honest parties, which is needed
                    to ensure correctness as explained below, and the output of the adversary A. The
                    latter output, without loss of generality, includes all the information that the adversary
                    learned, including its inputs, private state, all the messages sent by the honest parties
                    to A, and, depending on the model (see later discussion on the communication model),
                    maybe even include more information, such as public messages that the honest parties
                    exchanged. If we show that exactly the same transcript distribution2 can be generated
                    when interacting with the trusted party in the ideal model, then we are guaranteed that
                    no information is leaked from the computation via the execution of the protocol, as we
                    know that the ideal process does not expose any information about the inputs. More
                    formally,

                       Definition 8.1 Let f be a function on n inputs and let π be a protocol that
                       computes the function f . Given an adversary A, which controls some set of
                       parties, we define REALA,π (t) to be the sequence of outputs of honest parties
                       resulting from the execution of π on input vector t under the attack of A, in
                       addition to the output of A. Similarly, given an adversary A which controls a set
                       of parties, we define IDEALA ,f (t) to be the sequence of outputs of honest parties
                       computed by the trusted party in the ideal model on input vector t, in addition
                       to the output of A . We say that π securely computes f if, for every adversary
                       A as above, there exists an adversary A , which controls the same parties in the
                       ideal model, such that, on any input vector t, we have that the distribution of
                       REALA,π (t) is “indistinguishable” from the distribution of IDEALA ,f (t) (where
                       the term “indistinguishable will be explained later).

                       Intuitively, the task of the ideal adversary A is to generate (almost) the same output
                    as A generates in the real execution (referred to also as the real model). Thus, the
                    attacker A is often called the simulator (of A). Also note that the above definition
                    guarantees correctness of the protocol. Indeed, the transcript value generated in the ideal
                    model, IDEALA ,f (t), also includes the outputs of the honest parties (even though we
                    do not give these outputs to A ), which we know were correctly computed by the trusted
                    party. Thus, the real transcript REALA,π (t) should also include correct outputs of the
                    honest parties in the real model.
                    The inputs of the faulty parties. We assumed that every party Pi has an input ti , which
                    it enters into the computation. However, if Pi is faulty, nothing stops Pi from changing
                    ti into some ti . Thus, the notion of a “correct” input is defined only for honest parties.

                    2 The requirement that the transcript distribution be exactly the same will be relaxed later on.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0     July 5, 2007   14:16




                    184                        cryptography and game theory

                    However, the “effective” input of a faulty party Pi could be defined as the value ti that
                    the simulator A (which we assume exists for any real model A) gives to the trusted
                    party in the ideal model. Indeed, since the outputs of honest parties look the same in
                    both models, for all effective purposes Pi must have “contributed” the same input ti in
                    the real model.
                       Another possible misbehavior of Pi , even in the ideal model, might be a refusal to
                    give any input at all to the trusted party. This can be handled in a variety of ways,
                    ranging from aborting the entire computation to simply assigning ti some “default
                    value.” For concreteness, we assume that the domain of f includes a special symbol
                    ⊥ indicating this refusal to give the input, so that it is well defined how f should be
                    computed on such missing inputs. What this requires is that in any real protocol we
                    detect when a party does not enter its input and deal with it exactly in the same manner
                    as if the party would input ⊥ in the ideal model.
                    Variations on output delivery. In the above definition of security it is implicitly assumed
                    that all honest parties receive the output of the computation. This is achieved by stating
                    that IDEALA ,f (t) includes the outputs of all honest parties. We therefore say that our
                    current definition guarantees output delivery.
                        A more relaxed property than output delivery is fairness. If fairness is achieved, then
                    this means that if at least one (even faulty!) party learns its outputs, then all (honest)
                    parties eventually do too. A bit more formally, we allow the ideal model adversary
                    A to instruct the trusted party not to compute any of the outputs. In this case, in the
                    ideal model either all the parties learn the output, or none do. Since A’s transcript is
                    indistinguishable from A ’s this guarantees that the same fairness guarantee must hold
                    in the real model as well.
                        Yet, a further relaxation of the definition of security is to provide only correct-
                    ness and privacy. This means that faulty parties can learn their outputs, and pre-
                    vent the honest parties from learning theirs. Yet, at the same time the protocol will
                    still guarantee that (1) if an honest party receives an output, then this is the cor-
                    rect value, and (2) the privacy of the inputs and outputs of the honest parties is
                    preserved.
                    Variations on the model. The basic security notions introduced above are universal and
                    model-independent. However, specific implementations crucially depend on spelling
                    out precisely the model where the computation will be carried out. In particular, the
                    following issues must be specified:

                     (i) The parties. As mentioned above, the faulty parties could be honest-but-curious or
                         malicious, and there is usually an upper bound k on the number of parties that the
                         adversary can corrupt.
                    (ii) Computational assumptions. We distinguish between the computational setting and
                         the information theoretic setting. In the information theoretic model we assume that
                         the adversary is unlimited in its computing powers. In this case the term “indistin-
                         guishable” in Definition 8.1 is formalized by requiring the two transcript distributions
                         to be either identical (so-called perfect security) or, at least, statistically close in their
                         variation distance (so-called statistical security). On the other hand, in the compu-
                         tational setting we restrict the power of the adversary (as well as that of the honest
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 5, 2007   14:16




                                            cryptographic notions and settings                                   185

                          parties). A bit more precisely, we assume that the corresponding MPC problem is
                          parameterized by the security parameter λ, in which case (a) all the computation
                          and communication shall be done in time polynomial in λ; and (b) the misbehavior
                          strategies of the faulty parties are also restricted to be run in time polynomial in λ.
                          Furthermore, the term “indistinguishability” in Definition 8.1 is formalized by com-
                          putational indistinguishability: two distribution ensembles {Xλ }λ and {Yλ }λ are said to
                          be computationally indistinguishable, if for any polynomial-time distinguisher D, the
                          quantity , defined as |P r[D(Xλ ) = 1] − P r[D(Yλ ) = 1]|, is a “negligible” function
                          of λ. This means that for any j > 0 and all sufficiently large λ,  eventually becomes
                          smaller than λ−j .
                             This modeling of computationally bounded parties enables us to build secure MPC
                          protocols depending on plausible computational assumptions, such as the hardness of
                          factoring large integers, etc.
                    (iii) Communication assumptions. The two common communication assumptions are the
                          existence of a secure channel and the existence of a broadcast channel. Secure chan-
                          nels assume that every pair of parties Pi and Pj are connected via an authenticated,
                          private channel. A broadcast channel is a channel with the following properties:
                          if a party Pi (honest or faulty) broadcasts a message m, then m is correctly re-
                          ceived by all the parties (who are also sure the message came from Pi ). In partic-
                          ular, if an honest party receives m, then it knows that every other honest party also
                          received m.
                             A different communication assumption is the existence of envelopes. An envelope
                          (in its most general definition) guarantees the following properties: a value m can
                          be stored inside the envelope, it will be held without exposure for a given period of
                          time, and then the value m will be revealed without modification. A ballot box is an
                          enhancement of the envelope setting that also provides a random shuffling mechanism
                          of the envelopes.
                             These are, of course, idealized assumptions that allow for a clean description of
                          a protocol, as they separate the communication issues from the computational ones.
                          These idealized assumptions may be realized by a physical mechanisms, but in some
                          settings such mechanisms may not be available. Then it is important to address the
                          question if and under what circumstances we can remove a given communication
                          assumption. For example, we know that the assumption of a secure channel can be
                          substituted with a protocol, but under the introduction of a computational assumption
                          and a public key infrastructure. In general, the details of these substitutions are delicate
                          and need to be done with care.


                                    8.1.2 Existing Results for Multiparty Computation
                    Since the introduction of the MPC problem in the beginning of the 1980s, the work in
                    this area has been extensive. We will only state, without proofs, a few representative
                    results from the huge literature in this area.


                      Theorem 8.2 Secure MPC protocols withstanding coalitions of up to k mali-
                      cious parties (controlled by an attacker A) exist in the following cases:
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:16




                    186                      cryptography and game theory

                       (i) Assuming that A is computationally bounded, secure channels, and a broadcast
                           channel (and a certain cryptographic assumption, implied for example, by the
                           hardness of factoring, is true), then:
                           (a) for k < n/2 with output delivery.
                           (b) for k < n with correctness and privacy.
                           (c) additionally assuming envelopes, for k < n with fairness.
                      (ii) Assuming that A is computationally unbounded:
                           (a) assuming secure channels, then for k < n/3 with output delivery.
                           (b) assuming secure and broadcast channels, then for k < n/2 with output de-
                               livery (but with an arbitrarily small probability of error).
                           (c) assuming envelopes, ballot-box and a broadcast channel, then for k < n with
                               output delivery.


                    Structure of MPC protocols. A common design structure of many MPC protocols
                    proceeds in three stages: commitment to the inputs, computation of the function on the
                    committed inputs, revealing of the output. Below we describe these stages at a high
                    level, assuming for simplicity that the faulty parties are honest-but-curious.
                        In the first stage the parties commit to their inputs, this is done by utilizing the
                    first phase of a two-phased primitive called secret-sharing. The first phase of a (k, n)-
                    secret-sharing scheme is the sharing phase. A dealer, D, who holds some secret z,
                    computes n shares z1 , . . . , zn of z and gives the share zi to party Pi . The second
                    phase is the reconstruction phase, which we describe here and utilize later. For the
                    reconstruction, the parties broadcast their shares to recover z. Informally, such secret-
                    sharing schemes satisfy the following two properties: (1) k, or fewer, shares do not
                    reveal any information about z; but (2) any k + 1 or more shares enable one to recover
                    z. Thus, up to k colluding parties learn no information about z after the sharing stage,
                    while the presence of at least k + 1 honest parties allows one to recover the secret in
                    the reconstruction phase (assuming, for now, that no incorrect shares are given).
                        The classical secret-sharing scheme satisfying these properties is the Shamir secret-
                    sharing scheme. Here we assume that the value z lies in some finite field F of cardinality
                    greater than n (such as the field of integers modulo a prime p > n). The dealer D
                    chooses a random polynomial g of degree k with the only constraint that the free
                    coefficient of g is z. Thus, z = g(0). Then, if α1 , . . . , αn are arbitrary but agreed in
                    advance nonzero elements of F , the shares of party Pi is computed as zi = g(αi ). It is
                    now easy to observe that any k + 1 shares zi are enough to interpolate the polynomial
                    g and compute g(0) = z. Furthermore, any set of k shares is independent of z. This
                    is easy to see as for any value z ∈ F there exists a (k + 1)st share such that with the
                    given set of k shares they interpolate a polynomial g  , where g  (0) = z , in a sense
                    making any value of the secret equally likely. Thus, properties (1) and (2) stated above
                    are satisfied.
                        To summarize, the first stage of the MPC is achieved by having each party Pi invoke
                    the first part of the secret-sharing process as the dealer D with its input ti as the secret,
                    and distribute the correct shares of ti to each party Pj . If f is probabilistic, the players
                    additionally run a special protocol at the end of which a (k, n)-secret-sharing of a
                    random and secret value r is computed.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 5, 2007   14:16




                                             game theory notions and settings                                   187

                        In the second stage the parties compute the function f . This is done by evaluating
                    the pre–agreed-upon arithmetic circuit representing f over F , which is composed of
                    addition, scalar-multiplication and multiplication gates. The computation proceeds by
                    evaluating the gates one by one. We inductively assume that the inputs to the gates are
                    shared in the manner described above in the secret-sharing scheme, and we guarantee
                    that the output of the gate will preserve the same representation. This step forms the
                    heart of most MPC protocols. The computation of the addition and scalar-multiplication
                    gates are typically pretty straightforward and does not require communication (e.g.,
                    for the Shamir secret-sharing scheme the parties locally simply add or multiply by
                    the scalar their input shares), but is considerably more involved for the multiplication
                    gate and requires communication. For our purposes we will not need the details of the
                    computation mechanism, simply assuming that this computation on shares is possible
                    will suffice. Therefore, we can assume that at the end of the second stage the parties
                    have a valid secret-sharing of the required output(s) of the function f . The most crucial
                    observation is that no additional information is leaked throughout this stage, since all
                    the values are always shared through a (k, n)-secret-sharing scheme.
                        Finally, in the last stage the parties need to compute their individual outputs of the
                    function. As we have inductively maintained the property that the output of each gate
                    is in the secret-sharing representation, then the same it true for the output gate of f .
                    Thus, to let the parties learn the output s, which is the value of the function, the parties
                    simply run the reconstruction phase of the secret-sharing scheme (as described above),
                    by having each party broadcast its share of s.


                                         8.2 Game Theory Notions and Settings

                    Strategic games. We assume that the reader is familiar with the basic concepts
                    of strategic (or “one-shot simultaneous move”) games, including the notions of
                    Nash Equilibrium (NE) and Correlated Equilibrium (CE). In particular, recall from
                    Chapter 1 that the class of NE corresponds to independent strategies of all the parties,
                    while the class of CE – to arbitrary correlated strategies. However, in order to implement
                    a given CE one generally needs a special “correlation device” – so-called mediator M –
                    which will sample the prescribed strategy profile s = (s1 , . . . , sn ) for all the parties, and
                    disclose privately only action si to each player Pi . In particular, it is very important that
                    Pi does not learn anything about the recommended actions of the other parties, beyond
                    what could be implied by its own action si . Finally, recall that one can achieve consider-
                    ably higher payoffs by playing a well-selected CE than what is possible using any given
                    NE, or even what can be achieved by taking any convex combination of NE payoffs.
                    Games with incomplete information. In games with incomplete information, each party
                    has a private type ti ∈ Ti , where the joint vector t = (t1 , . . . , tn ) is assumed to be drawn
                    from some publicly known distribution. The point of such type, ti , is that it affects
                    the utility function of party Pi : namely, the utility ui depends not only on the actions
                    s1 , . . . , sn , but also on the private type ti of party Pi , or, in even more general games,
                    on the entire type vector t of all the parties. With this in mind, generalizing the notion
                    of Nash equilibrium to such games is straightforward. (The resulting Nash equilibrium
                    is also called Bayesian.)
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 5, 2007   14:16




                    188                       cryptography and game theory

                       Mediated games generalize to the typed setting, in which parties have to send their
                    types to the mediator M before receiving the joint recommendation. Depending on
                    the received type vector t, the mediator samples a correlated strategy profile s and
                    gives each party its recommended action si , as before. We remark that the expected
                    canonical strategy of party Pi is to honestly report its type ti to M, and then follow the
                    recommended action si . However, Pi can deviate from the protocol in two ways: (1)
                    send a wrong type ti or not send a type at all to M, as well as (2) decide to change
                    the recommended action from si to some si . As a mediator may receive faulty types, a
                    fully defined sampling strategy for the mediator should specify the joint distribution x
                    for every type t = (t1 , . . . , tn ), even outside the 
                                                                            support of the joint type distribution.
                    Formally, x t should be defined for every t ∈ i (Ti ∪ {⊥}), where ⊥ is a special
                    symbol indicating an invalid type. (In particular, games of complete information can
                    be seen as a special case where all ti = ⊥ and each party “refused” to report its type.)
                    With this in mind, the generalization of CE to games with incomplete information is
                    straightforward.
                    Aborting the game. We assume that the parties will always play the game by choosing an
                    action si ∈ Si and getting an appropriate payoff ui (s). Of course, we can always model
                    refusal to play by introducing a special action ⊥ into the strategy space, and defining
                    the explicit utilities corresponding to such actions. Indeed, many games effectively
                    guarantee participation by assigning very low payoff to actions equivalent to aborting
                    the computation. However, this is not a requirement; in fact, many games do not even
                    have the abort action as parts of their action spaces. To summarize, aborting is not
                    something which is inherent to games, although it could be modeled within the game,
                    if required.
                    Extended games. So far we considered only strategic games, where parties move
                    in “one-shot” (possibly with the help of the mediator). Of course, these games are
                    special cases of much more general extensive form games (with complete or incomplete
                    information), where a party can move in many rounds and whose payoffs depend on
                    the entire run of the game. In our setting we will be interested only in a special class of
                    such extensive form games, which we call (strategic) games extended by cheap-talk,
                    or, in brief, extended games.
                       An extended game G∗ is always induced by a basic strategic game G (of either
                    complete or incomplete information), and has the following form. In the cheap-talk
                    (or preamble) phase, parties follow some protocol by exchanging messages in some
                    appropriate communication model. This communication model can vary depending on
                    the exact setting we consider. But once the setting is agreed upon, the format of the
                    cheap talk phase is well defined. After the preamble, the game phase will start and the
                    parties simply play the original game G. In particular, the payoffs of the extended game
                    are exactly the payoff that the parties get in G (and this explains why the preamble
                    phase is called “cheap talk”).
                       Correspondingly, the strategy xi of party Pi in the extended game consists of its
                    strategy in the cheap talk phase, followed by the choice of an action si that Pi will
                    play in G. Just like in strategic games, we assume that the game phase must always go
                    on. Namely, aborting the game phase will be modeled inside G, but only if necessary.
                    However, the parties can always abort the preamble phase of the extended game, and
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0     July 5, 2007     14:16




                                                  contrasting mpc and games                                   189

                    prematurely decide to move on to the game phase. Thus, a valid strategy profile for the
                    extended game must include instructions of which action to play if some other party
                    refuses to follow its strategy, or, more generally, deviates from the protocol instructions
                    during the cheap talk phase (with abort being a special case of such misbehavior).
                    Nash equilibrium of extended games. With this in mind, (Bayesian) Nash equilibrium
                    for extended games is defined as before. We remark, however, that Nash equilibrium
                    is known to be too liberal for extensive form games, as it allows for “unreasonable”
                    strategy profiles to satisfy the definition of NE. For example, it allows for equilibrium
                    strategies containing so-called “empty threats” and has other subtle deficiencies. Nev-
                    ertheless, in order to keep our presentation simple, we will primarily restrict ourselves
                    to the basic concept of NE when talking about extended games.
                    Collusions. All the discussion so far assumed the traditional noncooperative setting,
                    where agents are assumed not to form collusions. In contrast, cooperative game theory
                    tries to model reasonable equilibrium concepts arising in scenarios where agents are
                    allowed to form collusions. However, traditional game-theoretic treatment of such
                    equilibria are fairly weak. We will come back to this issue in Section 8.4.1, where we
                    provide the definition of an equilibrium that we think is the most appropriate for our
                    setting and has been influenced by the MPC setting.

                                             8.3 Contrasting MPC and Games

                    As we can see, MPC and games share several common characteristics. In both cases
                    an important problem is to compute some function (s1 . . . sn ) = f (t1 , . . . , tn ; r) in a
                    private manner. However, there are some key differences summarized in Figure 8.1,
                    making the translation from MPC to Games and vice versa a promising but nonobvious
                    task.
                    Incentives and rationality. Game theory is critically built on incentives. Although it
                    may not necessarily explain why parties participate in a game, once they do, they have
                    a very well defined incentive. Specifically, players are assumed to be rational and
                    only care about maximizing their utility. Moreover, rationality is common knowledge:
                    parties are not only rational, but know that other parties are rational and utilize this
                    knowledge when making their strategic decisions. In contrast, the incentives in the


                                    Issue                Cryptography                  Game Theory

                                    Incentive            Outside the model             Payoff
                                    Players              Totally honest or malicious   Always rational
                                    Solution drivers     Secure protocol               Equilibrium
                                    Privacy              Goal                          Means
                                    Trusted party        In the ideal model            In the actual game
                                    Punishing cheaters   Outside the model             Central part
                                    Early stopping       Possible                      The game goes on!
                                    Deviations           Usually efficient             Usually unbounded
                                    k-collusions         Tolerate “large” k            Usually only k = 1


                                   Figure 8.1. Differences between Crytography and game theory.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:16




                    190                      cryptography and game theory

                    MPC setting remain external to the computation, and the reason the computation
                    actually ends with a correct and meaningful output comes from the assumption on the
                    parties. Specifically, in the MPC setting one assumes that there exist two diametrically
                    opposite kinds of parties: totally honest and arbitrarily malicious. Thus, the settings are
                    somewhat incomparable in general. On the one hand, the MPC setting may be harder as
                    it has to protect against completely unexplained behavior of the malicious parties (even
                    if such behaviors would be irrational had the parties had the utilities defined). On the
                    other hand, the Game Theory setting could be harder as it does not have the benefit of
                    assuming that some of the parties (i.e., the honest parties) blindly follow the protocol.
                    However, we remark that this latter benefit disappears for the basic notions of Nash
                    and correlated equilibria, since there one always assumes that the other parties follow
                    the protocol when considering whether or not to deviate. For such basic concepts, we
                    will indeed see in Section 8.4.2 that the MPC setting is more powerful.
                    Privacy and solution drivers. In the cryptographic setting the objective is to achieve a
                    secure protocol, as defined in Definition 8.1. In particular, the main task is to eliminate
                    the trusted party in a private and resilient way. While in the game theory setting the goal
                    is to achieve “stability” by means of some appropriate equilibrium. In particular, the
                    existence of the mediator is just another “parameter setting” resulting in a more desir-
                    able, but harder to implement equilibrium concept. Moreover, the privacy constraint on
                    the mediator is merely a technical way to justify a much richer class of “explainable”
                    rational behaviors. Thus, in the MPC setting privacy is the goal while in the game
                    theory setting it is a means to an end.
                    “Crime and punishment”. We also notice that studying deviations from the prescribed
                    strategy is an important part of both the cryptographic and the game-theoretic setting.
                    However, there are several key differences.
                       In cryptography, the goal is to compute the function, while achieving some security
                    guarantees in spite of the deviations of the faulty parties. Most protocols also enable
                    the participating parties to detect which party has deviated from the protocol. Yet, even
                    when exposed, in many instances no action is taken against the faulty party. Yet, when
                    an action, such as removal from the computation, is taken, this is not in an attempt to
                    punish the party, but rather to enable the protocol to reach its final goal of computing
                    the function. In contrast, in the game-theoretic setting it is crucial to specify exactly
                    how the misbehavior will be dealt with by the other parties. In particular, one typical
                    approach is to design reaction strategies that will negatively affect the payoffs of the
                    misbehaving party(s). By rationality, this ensures that it is in no player’s self-interest
                    to deviate from the prescribed strategy.
                       We already commented on a particular misbehavior when a party refuses to partic-
                    ipate in a given protocol/strategy. This is called early stopping. In the MPC setting,
                    there is nothing one can do about this problem, since it is possible in the ideal model
                    as well. In the Game Theory setting, however, we already pointed out that one always
                    assumes that “the game goes on.” That is, if one wishes, it is possible to model stopping
                    by an explicit action with explicit payoffs, but the formal game is always assumed to be
                    played. Thus, if we use MPC inside a game-theoretic protocol, we will have to argue –
                    from the game-theoretic point of view – what should happen when a given party aborts
                    the MPC.
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0    July 5, 2007   14:16




                                       cryptographic influences on game theory                                  191

                    Efficiency. Most game-theoretic literature places no computational limitations on the
                    efficiency of a party when deciding whether or not to deviate. In contrast, a significant
                    part of cryptographic protocol literature is designed to only withstand computationally
                    bounded adversaries.
                    Collusions. Finally, we comment again on the issue of collusions. Most game-theoretic
                    literature considers noncooperative setting, which corresponds to collusions of size
                    k = 1. In contrast, in the MPC setting the case k = 1 is usually straightforward, and
                    a lot of effort is made to make the maximum collusion threshold as high as possible.
                    Indeed, in most MPC settings one can tolerate at least a linear fraction of colluding
                    parties, and sometimes even a collusion of all but one party.

                                    8.4 Cryptographic Influences on Game Theory

                    In this section we discuss how the techniques and notions from MPC and cryptography
                    can be used in Game Theory. We start by presenting the notions of computational
                    and k-resilient equilibria, which were directly influenced by cryptography. We then
                    proceed by describing how to use appropriate MPC protocols and replace the mediator
                    implementing a given CE by a “payoff-equivalent” cheap-talk phase in a variety of
                    contexts.


                                                        8.4.1 New Notions
                    Computational equilibrium. Drawing from the cryptographic world, we consider set-
                    tings where parties participating in the extended game are computationally bounded
                    and we define the notion of computational equilibriums. In this case we only have to
                    protect against efficient misbehavior strategies xi . A bit more precisely, we will assume
                    that the basic game G has constant size. However, when designing the preamble phase
                    of the extended game, we can parameterize it by the security parameter λ, in which
                    case (a) all the computation and communication shall be done in time polynomial in
                    λ; and (b) the misbehavior strategies xi are also restricted to be run in time polynomial
                    in λ.
                         The preamble phase will be designed under the assumption of the existence of a
                    computationally hard problem. However, this introduces a negligible probability (see
                    Section 8.1.1) that within xi the attacker might break (say, by luck) the underlying
                    hard problem, and thus might get considerably higher payoff than by following the
                    equilibrium strategy xi∗ . Of course, this can improve this party’s expected payoff by at
                    most a negligible amount (since the parameters of G, including the highest payoff, are
                    assumed constant with respect to λ), so we must make an assumption that the party will
                    not bother to deviate if its payoffs will increase only by a negligible amount. This gives
                    rise to the notion of computational Nash equilibrium: a tuple of independent strategies
                    x1∗ , . . . , xn∗ where each strategy is efficient in λ such that for every Pi and for every
                                                                                ∗
                    alternative efficient in λ strategy xi , we have ui (xi∗ , x−i               ∗
                                                                                   ) ≥ ui (xi , x−i ) − , where  is
                    a negligible function of λ.
                    k-Resiliency. As we mentioned, the Game Theory world introduced several flavors of
                    cooperative equilibria concepts. Yet, for our purposes here, we define a stronger type
P1: SBT
9780521872829main      CUNY1061-Nisan          0 521 87282 0       July 5, 2007     14:16




                    192                            cryptography and game theory

                    of such an equilibrium, called a resilient (Nash or Correlated) equilibrium. Being a
                    very strong notion of an equilibrium, it may not exist in most games. Yet, we choose to
                    present it since it will exist in the “Game Theory-MPC” setting, where we will use MPC
                    protocols in several game-theoretic scenarios. The possibility of realizing such strong
                    equilibria using MPC shows the strength of the cryptographic techniques. Furthermore,
                    with minor modifications, most of the results we present later in the chapter extend to
                    weaker kinds of cooperative equilibria, such as various flavors of a more well known
                    coalition-proof equilibrium.3
                        Informally, resilient equilibrium requires protection against all coalitional deviations
                    that strictly benefit even one of its colluding parties. Thus, no such deviation will be
                    justifiable to any member of the coalition, meaning that the equilibrium strategies
                    are very stable. A bit more formally, an independent strategy profile (x1∗ , . . . , xn∗ ) is a
                    k-resilient Nash Equilibrium of G, if for all coalitions C of cardinality at most k, all
                    correlated deviation strategies xC of the members of C, and all members Pi ∈ C, we
                                    ∗
                    have ui (xC∗ , x−C                ∗
                                       ) ≥ ui (xC , x−C ). Thus, no coalition member benefits by xC .
                        The notion of k-resilient correlated equilibrium is defined similarly, although here
                    we can have two variants. In the ex ante variant, members of C are allowed to collude
                    only before receiving their actions from the mediator: namely, a deviation strategy
                    will tell each member of the coalition how to change its recommended action, but this
                    would be done without knowledge of the recommendations to the other members of
                    the coalition. In the more powerful interim variant, the members of the coalition will
                    see the entire recommended action vector sC∗ and then can attempt to jointly change
                    it to some sC . Clearly, ex ante correlated equilibria are more abundant than interim
                    equilibria. For example, it is easy to construct games where already 2-resilient ex ante
                    CEs achieve higher payoffs than 2-resilient interim equilibria, and even games where
                    the former correlated equilibria exist and the latter do not! This is true because the ex
                    ante setting makes a strong restriction that coalitions cannot form after the mediator
                    gave its recommended actions. Thus, unless stated otherwise, k-resilient CE will refer
                    to the interim scenario.
                        Finally, we mention that one can naturally generalize the above notions to games
                    with incomplete information, and also define (usual or computational) k-resilient Nash
                    equilibria of extended games.


                                  8.4.2 Removing the Mediator in Correlated Equilibrium
                    The natural question that can be asked is whether the mediator can be removed in the
                    game theory setting, by simulating it with a multiparty computation. The motivation
                    for this is clear, as the presence of the mediator significantly expands the number of
                    equilibria in strategic form games; yet, the existence of such a mediator is a very strong
                    and often unrealizable assumption.
                       Recall that in any correlated equilibrium x of a strategic game G (with imperfect
                    information, for the sake of generality), the mediator samples a tuple of recommended
                    action (s1 , . . . , sn ) according to the appropriate distribution based on the types of

                    3 Informally, these equilibria prevent only deviations benefiting all members of the coalition, while resilient

                      equilibria also prevent deviations benefiting even a single member.
P1: SBT
9780521872829main       CUNY1061-Nisan          0 521 87282 0      July 5, 2007     14:16




                                          cryptographic influences on game theory                                              193

                    the parties. This can be considered as the mediator computing some probabilistic
                    function (s1 , . . . , sn ) = f (t1 , . . . , tn ; r). We define the following extended game G∗ of
                    G by substituting the mediator with an MPC and ask whether the extended game is a
                    (potentially computational) Nash equilibrium.

                     (i) In the preamble stage, the parties run an “appropriate” MPC protocol4 to compute the
                         profile (s1 , . . . , sn ). Some additional actions may be needed (see below).
                    (ii) Once the preamble stage is finished, party Pi holds a recommended action si , which it
                         uses in the game G.

                    Meta-Theorem. Under “appropriate” conditions, the above strategies form a (poten-
                    tially computational) Nash equilibrium of the extended game G∗ , which achieves the
                    same expected payoffs for all the parties as the corresponding correlated equilibrium
                    of the original game G.5
                       As we discussed in Section 8.3, there are several differences between the MPC and
                    the game theory settings. Not surprisingly, we will have to resolve these differences
                    before validating the meta-theorem above. To make matters a bit more precise, we
                    assume that
                    r x is an interim k-resilient correlated equilibrium6 of G that we are trying to simulate.
                      k = 1 (i.e., no collusions) will be the main special case.
                    r the MPC protocol computing x is cryptographically secure against coalitions of up to
                      k malicious parties. This means the protocol is at least correct and private, and we will
                      comment about its “output delivery” guarantees later.
                    r The objective is to achieve a (possibly computational) k-resilient Nash equilibrium x ∗
                      of G∗ with the same payoffs as x.

                       Now the only indeterminant in the definition of G∗ is to specify the behavior of the
                    parties in case the MPC computation fails for some reason.
                    Using MPC with guaranteed output delivery. Recall that there exist MPC protocols (in
                    various models) that guarantee output delivery for various resiliencies k. Namely, the
                    malicious parties cannot cause the honest parties not to receive their output. The only
                    thing they can do is to choose their inputs arbitrarily (where a special input ⊥ indicates
                    they refuse to provide the input). But since this is allowed in the mediated game as
                    well, and k-resilient equilibrium ensures the irrationality of such behavior (assuming
                    the remaining (n − k) parties follow the protocol), we know the parties will contribute
                    their proper types and our meta-theorem is validated.

                       Theorem 8.3 If x is a k-resilient CE of G specified by a function f , and π is
                       an MPC protocol (with output delivery) securely computing f against a coalition
                       of up to k computationally unbounded/bounded parties, then running π in the
                       preamble step (and using any strategy to select a move in case some misbehavior

                    4 Where the type of the protocol depends on the particular communication model and the capabilities of the

                      parties.
                    5 Note that the converse (every NE of G∗ can be achieved by a CE of G) is true as well.
                    6 As we already remarked, the techniques presented here easily extend to weaker coalitional equilibria concepts.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 5, 2007   14:16




                    194                       cryptography and game theory

                      occurs) yields a k-resilient regular/computational NE of the extended game G∗ ,
                      achieving the same payoffs as x.


                    Using fair MPC. In some instances (e.g., part i.c of Theorem 8.2) we cannot guarantee
                    output delivery, but can still achieve fairness. Recall, this means that if at least one
                    party Pi obtains its correct output si , then all parties do. However, it might be possible
                    for misbehaving parties to cause everybody to abort or complete the protocol without
                    an output.
                         In the case where the protocol terminates successfully, we are exactly in the same
                    situation as if the protocol had output delivery, and the same analysis applies. In the
                    other case, we assume that the protocol enables detection of faulty behavior and that it
                    is observed that one of the parties (for simplicity, assume that it is Pn ) deviated from
                    the protocol. As the protocol is fair, the aborting deviation must have occurred before
                    any party has any information about their output. The simplest solution is to restart
                    the computation of x from scratch with all parties. The technical problem with this
                    solution is that it effectively allows (a coalition containing) Pn to mount a denial of
                    service attack, by misbehaving in every MPC iteration causing the preamble to run
                    forever.
                         Instead, to make the extended game always finite, we follow a slightly more so-
                    phisticated punishment strategy. We restart the preamble without Pn , and let the
                    (n − 1) remaining parties run a new MPC to compute the (n − 1)-input function f 
                    on the remaining parties’ inputs and a default value ⊥ for Pn : f  (t1 , . . . , tn−1 ; r) =
                    f (t1 , . . . , tn−1 , ⊥; r). Notice that in this new MPC n is replaced by n − 1 and k re-
                    placed by k − 1 (as Pn is faulty), which means that the ratio n−1     k−1
                                                                                              < nk , and, thus, f 
                    can still be securely computed in the same setting as f . Also notice that Pn does not
                    participate in this MPC, and will have to decide by itself (or with the help of other
                    colluding members) which action to play in the actual game phase. In contrast, parties
                    P1 , . . . , Pn−1 are instructed to follow the recommendations they get when computing
                    f  , if f  completes. If not, then another party (say, Pn−1 ) must have aborted this MPC,
                    in which case we reiterate the same process of excluding Pn−1 , and so on. Thus, at
                    some point we have that the process will end, as there is a finite number n of parties
                    and we eliminate (at least) one in each iteration.
                         Next, we argue that the resulting strategy profile x ∗ forms a k-resilient Nash equi-
                    librium of G∗ . To see this, the fairness of the MPC step clearly ensures that the only
                    effective misbehavior of a coalition of size |C| is to declare invalid types ⊥ for some of
                    its members, while changing the real type for others. In this case, their reluctance to do
                    so follows from the fact that such misbehavior is allowed in the mediated game as well.
                    And since we assumed that the strategy profile x is a k-resilient correlated equilibrium
                    of G, it is irrational for the members of the coalition to deviate in this way.
                    Using correct and private MPC: Case k = 1. We can see that the previous argument
                    crucially relied on the fairness of the MPC. In contrast, if the MPC used only provides
                    correctness and privacy, then the members of C might find their vector of outputs
                    sC before the remaining parties, and can choose to abort the computation precisely
                    when one of their expected payoffs pi = Exp(ui (s) | sC = sC ) when playing sC is
                    less than the a priori value pi = Exp(ui (s)). In fact, even for two-players games of
P1: SBT
9780521872829main       CUNY1061-Nisan          0 521 87282 0       July 5, 2007     14:16




                                           cryptographic influences on game theory                                               195

                    complete information, it is easy to construct a game G (e.g., the “Game of Chicken” in
                    Chapter 1) where the above aborting strategy of the player who learns the output
                    first will be strictly profitable for this player, even if the other player will play its
                    “conditional” strategy suggested in the previous paragraph.
                        Nevertheless, we show that one can still use unfair (yet private and correct) MPC
                    protocols in an important special case of the problem. Specifically, we concentrate
                    on the usual coalition-free case k = 1, and also restrict our attention to games with
                    complete information (i.e., no types). In this case, we show that if some party Pi deviates
                    in the MPC stage (perhaps by aborting the computation based on its recommended
                    action), the remaining parties P−i can sufficiently punish Pi to discourage such an
                    action. Let the min–max value vi for party Pi denote the worst payoff that players P−i
                    can jointly enforce on Pi : namely, vi = minz−i ∈(S−i ) maxsi ∈Si ui (si , z−i ).

                       Claim 8.4 For any correlated equilibrium x of G, any Pi and any action si for
                       Pi in the support of xi , Exp(ui (s) | si = si ) ≥ vi .

                       proof Notice that since x is a CE, si is the best response of Pi to the profile
                       x̄−i defined as x−i conditioned on si = si . Thus, the payoff Pi gets in this case is
                       what others would force on Pi by playing x̄−i , which is at least as large as what
                       others could have selected by choosing the worst profile z−i .

                       Now, in case Pi would (unfairly) abort the MPC step, we will instruct the other
                    parties P−i to punish Pi to its min–max value vi . More specifically, parties P−i should
                    play the correlated strategy z−i , which would force Pi into getting at most vi . Notice,
                    however, since this strategy is correlated, they would need to run another MPC protocol
                    to implement z−i ,7 By the above claim, irrespective of the recommendation si that Pi
                    learned, the corresponding payoff of Pi can only go down by aborting the MPC.
                    Therefore, it is in Pi ’s interests not to abort the computation after learning si .
                       We notice that the above punishment strategy does not straightforwardly generalize
                    to more advanced settings. For example, in case of coalitions it could be that the min–
                    max punishment for P1 tremendously benefits another colluding party P2 (who poses
                    as honest and instructs P1 to abort the computation to get high benefits for itself). Also,
                    in the case of incomplete information, it is not clear how to even define the min–max
                    punishment, since the parties do not even know the precise utility of Pi !


                                                          8.4.3 Stronger Equilibria
                    So far we talked only about plain Nash equilibria of the extended game G∗ . As we
                    already commented briefly, Nash equilibria are usually too weak to capture extensive-
                    form games. Therefore, an interesting (and still developing!) direction in recent research
                    is to ensure much stronger and more stable equilibria that would simulate correlated
                    equilibria of the original game.
                    Eliminating empty threats. One weakness of the Nash equilibrium is that it allows for
                    the so-called empty threats. Consider, for example, the min–max punishment strategy

                    7 Notice that there are no dishonest parties left, so any MPC protocol for the honest-but-curious case would work.
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:16




                    196                      cryptography and game theory

                    used above. In some games, punishing a misbehaving party to its min–max value
                    is actually very damaging for the punishers as well. Thus, the threat to punish the
                    misbehaving party to the min–max value is not credible in such cases, despite being
                    an NE. In this case, eliminating such an empty threat could be done by modifying the
                    punishment strategy to playing the worst Nash equilibrium of G for Pi (in terms of Pi ’s
                    payoff) when Pi is caught cheating. Unlike the min–max punishment, this is no longer
                    an empty threat because it is an equilibrium of G. However, it does limit (although
                    slightly) the class of correlated equilibria one can simulate, as one can achieve only a
                    payoff vector which is at least as large as the worst Nash equilibrium for each player.
                    In addition, formally defining such so-called subgame-perfect or sequential equilibria
                    has not yet been done in the computational setting, where most MPC protocols are
                    analyzed.
                    Ex ante correlated equilibria. So far we only talked about simulating interim corre-
                    lated equilibria, where colluding parties can base their actions after seeing all their
                    recommendations. Another interesting direction is that of simulating ex ante corre-
                    lated equilibria, where colluding parties can only communicate prior to contacting
                    the mediator. To implement this physical restriction in real life, we need to design
                    collusion-free protocols, where one has to ensure that no subliminal communication
                    (a.k.a. steganography) is possible. This is a very difficult problem. Indeed, most cryp-
                    tographic protocols need randomness (or entropy), and it is known that entropy almost
                    always implies steganography. In fact, it turns out that, in order to build such protocols,
                    one needs some physical assumptions in the real model as well. On a positive side, it
                    is known that envelopes (and a broadcast channel) are enough for building a class of
                    collusion-free protocols sufficient to simulate ex ante correlated equilibria without the
                    mediator.
                    Iterated deletion of weakly dominated strategies. In Section 8.5.2 we will study
                    a pretty general class of “function evaluation games,” where the objective is to
                    achieve Nash equilibrium that survives so-called iterated deletion of weakly dominated
                    strategies.
                    Strategic and privacy equivalence. The strongest recent results regarding removing
                    the mediator is to ensure (polynomially efficient) “real-life” simulation that guaran-
                    tees an extremely strong property called strategic and privacy equivalence. Intuitively,
                    it implies that our simulation gives exactly the same power in the real model as in
                    the ideal model. As such, it precisely preserves all different types of equilibria of
                    the original game (e.g., without introducing new, unexpected equilibria in the ex-
                    tended game, which we allowed so far), does not require the knowledge of the utility
                    functions or an a priori-type distribution (which most of the other results above do),
                    does not give any extra power to arbitrary coalitions, preserves privacy of the play-
                    ers types as much as in the ideal model, and has other attractive properties. Not
                    surprisingly, strategic and privacy equivalence is very difficult to achieve, and re-
                    quires some physical assumptions in the real model as well. The best known result
                    is an extension of the MPC result ii.c in Theorem 8.2, and shows how to imple-
                    ment strategic and privacy equivalence assuming a broadcast channel, envelopes and a
                    ballot box.
P1: SBT
9780521872829main       CUNY1061-Nisan       0 521 87282 0   July 5, 2007   14:16




                                      game theoretic influences on cryptography                                  197

                       To summarize, MPC techniques are promising in replacing the mediator by cheap
                    talk in a variety of situations. However, more work has to be done in trying to achieve
                    stronger kinds of equilibria using weaker assumptions.


                                   8.5 Game Theoretic Influences on Cryptography

                    The influence of Game Theory on Multiparty Computation has exemplified itself in
                    modeling multiparty computation with a game-theoretic flavor by introducing rational
                    parties with some natural utility functions into the computation. Once this is done,
                    two main areas of investigation are as follows. First, we try to characterize the class
                    of functions where it is in the parties’ selfish interest to report their true inputs to the
                    computation. We call such functions noncooperatively computable (NCC). Second, we
                    can ask to what extent the existing MPC protocols (used to compute NCC functions)
                    form an appropriate equilibrium for the extended game, where we remove the trusted
                    mediator by cheap talk computing the same function. As we see, the answer will depend
                    on the strength of the equilibrium we desire (and, of course, on the natural utilities we
                    assign to the “function evaluation game” defined below). Furthermore, issues arising
                    in the MPC “honest vs. malicious” setting also hold in the Game Theory “rational”
                    setting, further providing a synergy between these two fields.

                                       8.5.1 Noncooperatively Computable Functions
                    In order to “rationalize” the process of securely evaluating a given function f , we first
                    need to define an appropriate function evaluation game. For concreteness, we concen-
                    trate on single-output functions f (t1 , . . . , tn ), although the results easily generalize to
                    the n-output case. We also assume that each input ti matters (i.e., for some t−i the value
                    of f is not yet determined without ti ).
                    Function evaluation game. We assume that the parties’ types ti are their inputs to f
                    (which are selected according to some probability distribution D having full support).
                    The action of each party Pi is its guess about the output s ∗ of f . The key question,
                    however, is how to define the utilities of the parties. Now, there are several natural
                    cryptographic considerations that might weight into the definition of party Pi ’s utility.
                    r   Correctness. Each Pi wishes to compute f correctly.
                    r   Exclusivity. Each Pi prefers others parties Pj not to learn the value of f correctly.
                    r   Privacy. Each Pi wishes to leak as little as possible about its input ti to the other parties.
                    r   Voyeurism. Each Pi wishes to learn as much as possible about the other parties’ inputs.

                       Not surprisingly, one can have many different definitions for a cryptographically
                    motivated utility function of party Pi . In turn, different definitions would lead to
                    different results. For concreteness, we will restrict ourselves to one of the simplest and,
                    arguably, most natural choices. Specifically, we will consider only correctness and ex-
                    clusivity, and value correctness over exclusivity. However, other choices might also be
                    interesting in various situations, so our choice here is certainly with a loss of generality.
                       A bit more formally, recall that the utility ui of party Pi depends on the true type
                    vector t of all the parties, and the parties’ actions s1 , . . . , sn . Notice that the true type
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0   July 5, 2007   14:16




                    198                       cryptography and game theory

                    vector t determines the correct function value s ∗ = f (t), and parties’ actions determine
                    the boolean vector correct = (correct1 , . . . , correctn ), where correcti = 1 if an only
                    if si = s ∗ . In our specific choice of the utility function, we will assume that the utilities
                    of each party depend only on the boolean vector correct: namely, which of the parties
                    learned the output and which did not. Therefore, we will write ui (correct) to denote
                    the utility of party Pi . Now, rather than assigning somewhat arbitrary numbers to
                    capture correctness and exclusivity, we state only the minimal constraints that imply
                    these properties. Then, the correctness constraint states that ui (correct) > ui (correct ),
                    whenever correcti = 1 and correcti = 0. Similarly, exclusivity constraint states that
                    if (a) correcti = correcti , (b) for all j = i we have correctj ≤ correctj , while (c)
                    for some j actually correctj = 0 and correctj = 1, then ui (correct) > ui (correct ).
                    Namely, provided Pi has the same success in learning the output, it prefers as few
                    parties as possible to be successful.
                    Noncooperatively computable functions. Having defined the function evaluation game,
                    we can now ask what are the equilibria of this game. In this case, Nash equilibria are not
                    very interesting, since parties typically have too little information to be successful with
                    any nontrivial probability. On the other hand, it is very interesting to study correlated
                    equilibria of this game. Namely, parties give their inputs ti to the mediator M, who then
                    recommends an action si∗ for each party. Given that each party is trying to compute
                    the value of the function f , it is natural to consider “canonical” mediator strategy:
                    namely, that of evaluating the function f on the reported type vector t, and simply
                    recommending each party to “guess” the resulting function value s ∗ = f (t). Now, we
                    can ask the question of characterizing the class of functions f for which this canonical
                    strategy is indeed a correlated equilibrium of the function evaluation game. To make
                    this precise, though, we also need to define the actions of the mediator if some party
                    gives a wrong type to the mediator. Although several options are possible, here we
                    will assume that the mediator will send an error message to all the parties and let them
                    decide by themselves what to play.

                      Definition 8.5 We say that a function f is noncooperatively computable (NCC)
                      with respect to utility functions {ui } (and a specific input distribution D) if the
                      above canonical mediated strategy is a correlated equilibrium of the function
                      evaluation game. Namely, it is in the parties’ selfish interest to honestly report
                      their true inputs to the mediator.

                         We illustrate this definition by giving two classes of functions that are never NCC.
                    Let us say that a function f is dominated if there exists an index i and an input
                    ti , which determine the value of f irrespective of the other inputs t−i . Clearly, for
                    such an input ti it is not in the interest of Pi to submit ti to the mediator, as Pi
                    is assured of correcti = 1 even without the help of M, while every other party is
                    not (for at least some of its inputs). Thus, dominated functions cannot be NCC. For
                    another example, a function f is reversible if for some index i and some input ti ,
                    there exists another input ti and a function g, such that (a) for all other parties’ inputs
                    t−i we have g(f (ti , t−i ), ti ) = f (ti , t−i ), and (b) for some other parties’ inputs t−i
                    we have f (ti , t−i ) = f (ti , t−i ). Namely, property (a) states that there is no risk in
                    terms of correctness for Pi to report ti instead of ti , while property (b) states that
P1: SBT
9780521872829main       CUNY1061-Nisan          0 521 87282 0       July 5, 2007      14:16




                                         game theoretic influences on cryptography                                                199

                    at least sometimes Pi will be rewarded by higher exclusivity. A simple example of
                    such (boolean) function is the parity function: negating one’s input always negates the
                    outcome, but still in a manner easily correctable by negating the output back. Clearly,
                    reversible functions are also not NCC.
                       In general, depending on the exact utilities and the input distribution D, other
                    functions might also be non-NCC. However, if we assume that the risk of losing
                    correctness is always too great to be tempted by higher exclusivity, it turns out that these
                    two classes are the only non-NCC functions. (And, thus, most functions, like majority,
                    are NCC.) More precisely, assume that the utilities and the input distribution D are
                    such that for all vectors correct, correct , correct satisfying correcti = correcti = 1,
                    correcti = 0, we have ui (correct) > (1 − )ui (correct ) + ui (correct ), where  is
                    the smallest probability in D. Namely, if by deviating from the canonical strategy
                    there is even a minuscule chance of Pi not learning the value of f correctly, this loss
                    will always exceed any potential gain caused by many other parties not learning the
                    outcome as well. In this case we can show the following:

                       Theorem 8.6 Under the above assumption, a function f is NCC if and only if
                       it is not dominated and not reversible.8

                    Collusions. So far we concentrated on the case of no collusions; i.e., k = 1. However,
                    one can also define (a much smaller class of) k-Non-Cooperatively Computable (k-
                    NCC) functions, for which no coalition of up to k parties has any incentive to deviate
                    from the canonical strategy of reporting their true types. One can also characterize
                    k-NCC functions under appropriate assumptions regarding the utilities and the input
                    distribution D.

                                               8.5.2 Rational Multiparty Computation
                    Assume that a given function f is k-NCC, so it is in the parties’ own interest to
                    contribute their inputs in the ideal model. We now ask the same question as in Section
                    8.4: can we replace the mediator computing f by a corresponding MPC protocol for
                    f ? Notice, by doing so the parties effectively run the cryptographic MPC protocol
                    for computing f . Thus, a positive answer would imply that a given MPC protocol
                    π securely computes f not only from a cryptographic point of view but also from a
                    game-theoretic, rational point of view! Fortunately, since the function evaluation game
                    is just a particular game, Theorem 8.3 immediately implies

                       Theorem 8.7 If f is a k-NCC function (w.r.t. to some utilities and input dis-
                       tribution) and π is an MPC protocol securely computing f against a coalition
                       of up to k computationally unbounded/bounded parties, then π is a k-resilient
                       regular/computational Nash equilibrium for computing f in the corresponding
                       extended game.

                      From a positive perspective, this result shows that for the goal of achieving just a
                    Nash equilibrium, current MPC protocols can be explained in rational terms, as long

                    8 In fact, under our assumption that each party’s input matters in some cases and D has full support, it is easy to

                      see that every dominated function is also reversible.
P1: SBT
9780521872829main      CUNY1061-Nisan          0 521 87282 0      July 5, 2007      14:16




                    200                            cryptography and game theory

                    as the parties are willing to compute f in the ideal model. From a negative perspective,
                    the latter constraint nontrivially limits the class of functions f , which can be rationally
                    explained, and it is an interesting open problem how to rationalize MPC even for
                    non-NCC functions, for which the cryptographic definition still makes perfect sense.
                    Stronger equilibria. As another drawback, we already mentioned that the notion of
                    Nash equilibrium is really too weak to capture the rationality of extensive-form pro-
                    cesses, such as multiparty computation protocols. Thus, an important direction is to
                    try achieving stronger kinds of equilibria explaining current MPC protocols, or, alter-
                    natively, design robust enough MPC protocols which would achieve such equilibria.
                    In Section 8.4.3, we briefly touched on several general results in this direction (which
                    clearly still apply to the special case of the function evaluation games). Here we will
                    instead concentrate on the specifics of computing the function under the correctness
                    and exclusivity preferences defined in the previous section, and will study a specific
                    refinement of the Nash equilibrium natural for these utility functions.
                        To motivate our choice, let us see a particular problem with current MPC protocols.
                    Recall, such protocols typically consist of three stages; in the first two stages the parties
                    enter their inputs and compute the secret-sharing of the output of f , while the last stage
                    consists of the opening of the appropriate output shares. Now we claim that the strategy
                    of not sending out the output shares is always at least as good as, and sometimes better
                    than, the strategy of sending the output shares. Indeed, consider any party Pi . The
                    correctness of output recovery for Pi is not affected by whether or not Pi sent his own
                    share, irrespective of the behavior of the other parties. Yet, not sending the share to
                    others might, in some cases, prevent others from reconstructing their outputs, resulting
                    in higher exclusivity for Pi . True, along the Nash equilibrium path of Theorem 8.7,
                    such cases where the share of Pi was critical did not exhibit themselves. Still, in reality
                    it seems that there is no incentive for any party to send out their shares, since this
                    is never better, and sometimes worse than not sending the shares. This motivates the
                    following definition.

                       Definition 8.8 We say that a strategy s ∈ Si is weakly dominated by s  ∈ Si
                       with respect to S−i if (a) there exists s−i ∈ S−i such that ui (s, s−i ) < ui (s  , s−i )
                                                                                                 
                       and (b) for all strategies s−i ∈ S−i we have that ui (s, s−i ) ≤ ui (s  , s−i ). We define
                       iterated deletion of weakly dominated strategies (IDoWDS) as the following
                       process. Let DOMi (S1 , . . . , Sn ) denote the set of strategies in Si that are weakly
                                                                                                 j
                       dominated with respect to S−i . Let Si0 = Si and for j ≥ 1 define Si inductively as
                               j −1            j −1        j −1                
                       Si = Si \DOMi (S1 , . . . , Sn ) and let Si∞ = j ≥1 Si . Finally, we say that
                         j                                                             j

                       a Nash equilibrium (x1 , . . . , xn ) survives IDoWDS, if each xi is fully supported
                       within Si∞ .

                    k-resilient Nash equilibria surviving IDoWDS are defined similarly.9
                       Now, the above discussion implies that the k-resilient Nash equilibrium from Theo-
                    rem 8.7 does not survive IDoWDS. On a positive side, the only reason for that was that

                    9 We notice that, in general, it matters in which order 1 removes the weakly dominated strategies. The specific

                      order chosen above seems natural, however, and will not affect the results we present below.
P1: SBT
9780521872829main      CUNY1061-Nisan        0 521 87282 0       July 5, 2007      14:16




                                      game theoretic influences on cryptography                                  201

                    the basic secret-sharing scheme where the parties are instructed to blindly open their
                    shares does not survive IDoWDS. It turns out that the moment we fix the secret-sharing
                    scheme to survive IDoWDS, the resulting Nash equilibrium for the function evaluation
                    game will survive IDoWDS too, and Theorem 8.7 can be extended to Nash equilibrium
                    surviving IDoWDS. Therefore, we will treat only the latter, more concise problem. We
                    remark, however, that although a Nash equilibrium surviving IDoWDS is better than
                    plain Nash equilibrium, it is still a rather weak concept. For example, it still allows for
                    “empty threats,” and has other undesirable properties. Thus, stronger equilibria are still
                    very desirable to achieve.
                    Rational secret-sharing. Recall, in the (k, n)-secret-sharing problem the parties are
                    given (random valid) shares z1 , . . . , zn of some secret z, such that any k shares leak
                    no information about z, while any k + 1 or more shares reveal z. We can define the
                    secret-sharing game, where the objective of each party is to guess the value of z, and
                    where we assume that parties’ utilities satisfy the correctness and exclusivity constraints
                    defined earlier. In the extended game corresponding to the secret-sharing game, the
                    parties can perform some computation before guessing the value of the secret. For our
                    communication model, we assume that it is strong enough to perform generic multiparty
                    computation, since this will be the case in the application to the function evaluation
                    game. (On the other hand, we will need only MPC with correctness and privacy, and not
                    necessarily fairness.) In addition, if not already present, we also assume the existence of
                    a simultaneous broadcast channel, where at each round all parties can simultaneously
                    announce some message, after which they atomically receive the messages of all the
                    other parties. Our goal is to build a preamble protocol for which the outcome of all
                    the parties learning the secret z will be a k-resilient Nash equilibrium for the extended
                    game that survives IDoWDS.
                        As we observed already, the natural 1-round preamble protocol where each party
                    is supposed to simply broadcast its share does not survive IDoWDS. In fact, a simple
                    backward induction argument shows that any preamble protocol having an a priori fixed
                    number of simultaneous broadcast rounds (and no other physical assumptions, such as
                    envelopes and ballot boxes) cannot enable the parties to rationally learn the secret and
                    survive IDoWDS. Luckily, it turns out that we can have probabilistic protocols with no
                    fixed upper bound on the number of rounds, but which have a constant expected number
                    of rounds until each party learns the secret. We sketch the simplest such protocol below.
                    W.l.o.g. we assume that the domain of the secret-sharing scheme is large enough to
                    deter random guessing of z, and also includes a special value denoted ⊥, such that z is
                    guaranteed to be different from ⊥.
                        Let α ∈ (0, 1) be a number specified shortly. At each iteration r ≥ 1, the parties do
                    the following two steps:

                     (i) Run an MPC protocol on inputs zi which computes the following probabilistic
                         functionality. With probability α, compute fresh and random (k, n)-secret-sharing
                         z1 , . . . , zn of z, where party Pi learns zi . Otherwise, with probability 1 − α compute
                         a random (k, n)-secret-sharing z1 , . . . , zn of ⊥, where party Pi learns zi .10


                10 This protocol is typically pretty efficient for the popular Shamir’s secret-sharing scheme.
P1: SBT
9780521872829main      CUNY1061-Nisan      0 521 87282 0     July 5, 2007   14:16




                    202                       cryptography and game theory

                     (ii) All parties Pi simultaneously broadcast zi to other parties.
                    (iii) If either the MPC protocol fails for even one party, or even one party fails to broadcast
                          the value zi , all parties are instructed to abort.
                    (iv) Each party tries to recover some value z from the shares received from the other
                          parties. If the recovery fails, or at least one share is inconsistent with the final value
                          z , the party aborts the preamble. Otherwise, if z = ⊥ the parties proceed to the next
                          iteration, while in case z = ⊥ the parties stop the preamble and output z as their
                          guess for z.

                       Notice, by the privacy of the MPC step, no coalition C of up to k parties knows if
                    the value z is equal to z or ⊥. Thus, in case this coalition chooses not to broadcast
                    their shares, they will learn only the value z (while punishing all the other parties) with
                    probability α, and not learn the value z forever with probability 1 − α. Thus, if α is
                    small enough (depending on the particular utilities), the risk of not learning the secret
                    will outweigh the gain of achieving higher exclusivity. Also, it is easy to see that no
                    strategy of the above protocol is weakly dominated by another strategy, so the above
                    Nash equilibrium survives IDoWDS.
                       The above protocol works for any k. However, it runs in expected O(1/α) iterations,
                    which is constant, but depends on the specific utilities of the parties (and the value
                    k). Somewhat more sophisticated protocols are known to work for not too large k, but
                    have expected number of iterations which is independent of the utilities. These results
                    are summarized without further details below.

                      Theorem 8.9 Assume that the parties utilities satisfy correctness over exclu-
                      sivity properties for the (k, n)-secret-sharing game. Then there exists k-resilient
                      Nash equilibria for the extended game that survive IDoWDS and run in expected
                      constant number of iterations r, where
                      r k < n, but r depends on the specific utilities.
                       r k < n/2, r is fixed, but the parties still need to know a certain parameter depending
                         on the specific utilities.
                       r k < n/3, r is fixed, and no other information about the utilities is needed.


                                                           8.6 Conclusions

                    As we have seen, the settings of MPC in cryptography and correlated equilibrium
                    in game theory have many similarities, as well as many differences. Existing results
                    so far started to explore these connections, but much work remains to be done. For
                    example, can we use some flavors of MPC to remove the mediator, while achiev-
                    ing very strong types of Nash equilibria, but with more realistic physical and other
                    setup assumptions? Or, can we use game theory to “rationalize” MPC protocols for
                    non-NCC functions (such as parity), or to explain other popular cryptographic tasks
                    such as commitment or zero-knowledge proofs? In addition, so far “rationalizing”
                    MPC using game theory resulted only in more sophisticated protocols. Are there nat-
                    ural instances where assuming rationality will simplify the design of cryptographic
                    tasks?
P1: SBT
9780521872829main      CUNY1061-Nisan     0 521 87282 0   July 5, 2007   14:16




                                                              notes                                        203

                                                           8.7 Notes

                    The multiparty computation problem (Section 8.1) was introduced in Yao (1982).
                    The basic definitional and construction approaches were introduced by Goldreich
                    et al. (1987), in particular the paradigm of a real/ideal execution. In Section 8.1.1
                    we follow the definitional framework of Canetti (2000), which is based on the works
                    of Goldwasser and Levin (1990), Micali and Rogaway (1991), and Beaver (1991).
                    The results mentioned in Theorem 8.2 are from the following: parts i.a and i.b from
                    Goldreich et al. (1987), part i.c from Lepinski et al. (2004), part ii.a from Ben-Or et al.
                    (1988) and Chaum et al. (1988), part ii.b from Rabin and Ben-Or (1989) and Beaver
                    (1991), part ii.c from Izmalkov et al. (2005). The secret-Sharing protocol presented is
                    Shamir’s Secret-Sharing (1979). The notion of indistinguishability was introduced in
                    Goldwasser and Micali (1984). For a more formal and in-depth discussion on multiparty
                    computations see Goldreich (2004).
                       In Section 8.2 we present the classical results of Nash (1951) and Aumann (1974) for
                    Nash and correlated equilibrium (respectively). The extension of correlated equilibrium
                    to games with incomplete information is due to Forges (1986). The notion of extended
                    games is from Barany (1992). For a broader game theory background, see the book by
                    Osborne and Rubinstein (1999).
                       The comparison discussion between Game Theory and Cryptography, as it appears
                    in Section 8.3, was initiated by Dodis et al. (2000) and later expanded by Feigebaum
                    and Shenker (2002); yet here we further expand on these points. The related discussion
                    was also carried out in many other works (Abraham et al., 2006; Barany, 1992; Lepinski
                    et al., 2004; Izmalkov et al., 2005).
                       The notion of computational equilibrium which appears in Section 8.4.1 was intro-
                    duced in Dodis et al. (2000). The work of Urbano and Vila (2002, 2004) also deals
                    with the computational model, but does not explicitly define this notion. The impor-
                    tance of tolerating collusions was first addressed in our setting by Feigenbaum and
                    Shanker (2002). For the k-resilient equilibrium we chose the formulation of Abraham
                    et al. (2006), as we felt it best suited our presentation. For other related formulations,
                    see the references in Abraham et al. (2006), and also a recent work of Lysyanskaya
                    and Triandopoulos (2006). The results which appear in Section 8.4.2 appear in the
                    following. Theorem 8.3 follows by combining results such as Dodis et al. (2000),
                    Barany (1992), Ben-Porath (1998), Gerardi (2004), Urbano and Vila (2002, 2004) and
                    Abraham et al. (2006). The result for using fair MPC appears in Lepinski et al. (2004).
                    The introduction of a min-max punishment to deal with unfair MPC in the attempt to
                    remove the mediator appears in Dodis et al. (2000). For some efficiency improvements
                    to the protocol of Dodis et al. (2000), see the works of Teague (2004) and Attalah
                    et al. (2006). The results which appear in Section 8.4.2 appear in the following. The
                    worst equilibrium punishment technique was first applied to unmediated games by
                    Ben-Porath (1998). The notion of collusion free protocols which is used to implement
                    ex ante equilibria is from the work of Lepinski et al. (2005). The result of achieving
                    strategic and privacy equivalence under physical assumptions is from Izmalkov et al.
                    (2005).
P1: SBT
9780521872829main      CUNY1061-Nisan       0 521 87282 0     July 5, 2007   14:16




                    204                         cryptography and game theory

                       The noncooperative computation formulation and some discussion used in Section
                    8.5.1 are introduced (for k = 1) by Shoham and Tennenholtz (2005), and expanded
                    by McGrew et al. (2003). Theorem 8.6 is also from Shoham and Tennenholtz (2005),
                    while the formulation of “correctness followed by exclusivity” utilities is from Halpern
                    and Teague (2004). The results in Section 8.5.2 appear as follows: the introduction of
                    rational secret-sharing surviving IDowDS and the impossibility result of reaching it in a
                    fixed number of rounds are from Halpern and Teague (2004). The protocol for rational
                    secret-sharing we present appears in Abraham et al. (2006) and (for k = 1) by Gordon
                    and Katz (2006). Yet, a more complicated and less general solution along these lines
                    appeared first (for k = 1) in Halpern and Teague (2004). Theorem 8.9 is from Abraham
                    et al. (2006). For a different, but related “mixed MPC” model, see Lysyanskaya and
                    Triandopoulos (2006).



                                                         Acknowledgments

                    We thank the following people for extensive discussions, explanations, and general ad-
                    vice: Ittai Abraham, Ran Canetti, Hugo Krawczyk, Matt Lepinski, Anna Lysyanskaya,
                    Silvio Micali, abhi shelat, and Nikos Triandopoulos, and give special thanks to our
                    coauthor Shai Halevi.


                                                             Bibliography
                    I. Abraham, D. Dolev, R. Gonen, and J. Halpern. Distributed computing meets game theory: Ro-
                       bust mechanisms for rational secret-sharing and multiparty computation. In Princ. of Distributed
                       Computing ’06, pp. 53–62. ACM Press, 2006.
                    M. Atallah, M. Blanton, K. Frikken, and J. Li. Efficient Correlated Action Selection. In Financial
                       Crypt., LNCS 4107:296–310. Springer, 2006.
                    R. Aumann. Subjectivity and correlation in randomized strategies. J. Math. Econ., 1:67–96, 1974.
                    I. Barany. Fair Distribution Protocols or How the Players Replace Fortune. Math. Oper. Res.,
                       17(2):327–341, 1992.
                    D. Beaver. Secure multiparty protocols and zero-knowledge proof systems tolerating a faulty minority.
                       J. Cryptology, 4(2):75–122, 1991.
                    M. Ben-Or, S. Goldwasser, and A. Wigderson. Completeness theorems for noncryptographic fault-
                       tolerant distributed Computations. In Proc. 20th Symp. on Theory of Computing 88, pp. 1–10.
                    E. Ben-Porath. Correlation without mediation: Expanding the set of equilibrium outcomes by “cheap”
                       pre-play procedures. J. Econ. Theo., 80(1):108–122, 1998.
                    R. Canetti. Security and composition of multiparty cryptographic protocols. J. Cryptology, 13(1):143–
                       202, 2000. Available at eprint.iacr.org/1998/018.
                    D. Chaum, C. Crepeau, and I. Damgard. Multiparty unconditionally secure protocols. In Proc. 20th
                       Symp. on Theory of Computing 88, pp. 11–19.
                    Y. Dodis, S. Halevi, and T. Rabin. A cryptographic solution to a game theoretic problem. In Crypto
                       2000, pp. 112–130, 2000. LNCS No. 1880.
                    F.M. Forges. An approach to communication equilibria. Econometrica, 54(6):1375–85, 1986.
                    J. Feigenbaum and S. Shenker. Distributed algorithmic mechanism design: Recent results and future
                       directions. In Proc. 6th Intl. Wkshp. Disc. Algo. Meth. Mobile Comp. Comm., pp. 1–13. ACM
                       Press, 2002.
P1: SBT
9780521872829main       CUNY1061-Nisan       0 521 87282 0     July 5, 2007   14:16




                                                             bibliography                                            205

                    D. Gerardi. Unmediated communication in games with complete and incomplete information. J.
                       Econ. Theo., 114:104,131, 2004.
                    O. Goldreich. Foundations of Cryptography: Volume 2. Cambridge University Press, 2004. Prelimi-
                       nary version http://philby.ucsd.edu/cryptolib.html/.
                    O. Goldreich, S. Micali, and A. Wigderson. How to play any mental game. In Proc. 19th STOC, pp.
                       218–229. ACM, 1987.
                    S. Goldwasser and L. Levin. Fair computation of general functions in presence of immoral majority.
                       In Crypto ’90, LNCS 537:77–93.
                    S. Goldwasser and S. Micali. Probabilistic encryption. J. Comp. Syst. Sci., 28(2):270–299, April
                       1984.
                    S.D. Gordon and J. Katz. Rational secret-sharing, revisited. In 5th Conf. Sec. Crypto. Networks, 2006.
                       Updated version available at http://eprint.iacr.org/2006/142.
                    J. Halpern and V. Teague. Rational secret-sharing and multiparty computation. In Proc. of 36th STOC,
                       pp. 623–632. ACM Press, 2004.
                    S. Izmalkov, M. Lepinski, and S. Micali. Rational secure computation and ideal mechanism design.
                       In Proc. of 46th Fdns. of Computer Science, pp. 585–595, 2005.
                    M. Lepinksi, S. Micali, and A. Shelat. Collusion-free protocols. In Proc. 37th Ann. ACM Symp. Theo.
                       Comp., pp. 543–552. ACM Press, 2005.
                    M. Lepinski, S. Micali, C. Peikert, and A. Shelat. Completely fair sfe and coalition-safe cheap talk.
                       In PODC ’04: Proc. 23rd Annual ACM Symp. Princ. Dist. Comp., pp. 1–10. ACM Press, 2004.
                    A. Lysyanskaya and N. Triandopoulos. Rationality and adversarial Behavior in Multi-Party Compu-
                       tation. In Crypto 2006, 2006.
                    R. McGrew, R. Porter, and Y. Shoham. Towards a general theory of non-cooperative computation
                       (extended abstract). In Theo. Aspects of Rationality and Knowledge IX, 2003.
                    S. Micali and P. Rogaway. Secure computation. In Crypto ’91, LNCS 576:392–404, 1991.
                    J. Nash. Non-cooperative games. Annals of Math., 54:286–295, 1951.
                    M.J. Osborne and A. Rubinstein. A Course in Game Theory. MIT Press, 1999.
                    T. Rabin and M. Ben-Or. Verifiable secret-sharing and multiparty protocols with honest majority. In
                       Proc. 21st Symp. on Theory of Computing, pp. 73–85. ACM, 1989.
                    A. Shamir. How to share a secret. Comm. ACM, 22:612–613, 1979.
                    Y. Shoham and M. Tennenholtz. Non-cooperative computation: Boolean functions with correctness
                       and exclusivity. Theor. Comput. Sci., 343(1–2):97–113, 2005.
                    V. Teague. Selecting correlated random actions. In Financial Cryptography, LNCS 3110:181–195.
                       Springer, 2004.
                    A. Urbano and J.E. Vila. Computational complexity and communication: Coordination in two-player
                       games. Econometrica, 70(5):1893–1927, 2002.
                    A. Urbano and J.E. Vila. Computationally restricted unmediated talk under incomplete information.
                       Econ. Theory, 23:283–320, 2004.
                    A.C. Yao. Protocols for secure computations. In Proc. Fdns. of Computer Science 82, pp. 160–164,
                       IEEE, 1982.
P1: SBT
9780521872829main   CUNY1061-Nisan   0 521 87282 0   July 5, 2007   14:16
P1: SBT
9780521872829main   CUNY1061-Nisan   0 521 87282 0   July 5, 2007   14:17




                                                     PART TWO

                        Algorithmic Mechanism
                                Design
P1: SBT
9780521872829main   CUNY1061-Nisan   0 521 87282 0   July 5, 2007   14:17
