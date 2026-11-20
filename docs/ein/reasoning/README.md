# Reasoning — what Ein computes, and why it is right

These pages state Ein's reasoning as a theory: what a program means once it is read, how its knowledge base grows,
what the answer is, and why every way of finding the answer gives the same one. The theory is stated so that a
second implementation would be held to it. Nothing here is true only of the implementation that ships: that is
`ein.rs`'s perspective. What a program says and what a run prints is the user's perspective.

**The question asked of every paragraph** here is *would this still be true of a second implementation?* A
paragraph that is true only of how one engine computes a thing belongs to that engine's pages. A paragraph that a
second implementation must also make true belongs here, whether it is a definition, an order a reader can observe,
or an argument that two computations agree.

**What a page states.** Each page states **definitions**, **propositions** and **arguments**. A definition fixes
a thing: a set, a relation, a function, an order. A proposition says what holds of everything that satisfies
the definitions, and stands beside its argument; where it needs a condition, the proposition names it. The pages
are written in three languages
([Q-M1f.370](../../decisions/docs/Q-M1f.370-each-perspective-of-the-documentation-is-written.md)): mathematics, which
is the main one; graphs, of facts, of rules and of the lattice of commitments; and words, which say what a
formula, an arrow or a node means. An example program appears as its graph and its formulas, and the page links
the user's page that has its spelling. No page says how the shipped engine computes a thing or which test holds it
to a proposition: that is [reasoning in code](../ein.rs/08-reasoning-in-code/README.md), which follows these pages
one by one. The decisions a page carries out are cited by link to their files under
[the decisions](../../decisions/README.md).

## Notation

The symbols of the first table mean one thing on every page, and each is defined on the page named. A letter a
page needs for itself is defined where it is used and is in the second table, so a letter that is in neither is
bound by the sentence it stands in.

**Letters.** A constant of a program is set upright, $\mathsf{Norwegian}$, and a relation of a program as an
operator, $\operatorname{lives-in}$. Italic letters range over what the table says: $u, v, w$ and $o$ over
objects, $f, g$ over facts, $h$ over candidates, $r$ over relations, $x, y, z$ over variables, $a$ over activators,
and, where no rule is in view, $a, b, c$ over atoms or over candidates, as the page says.

**Words.** An **object** is a node of the graph a knowledge base is, and the word means nothing else on these
pages. An **atom** is a labelled node with no outgoing arrow, and its label is its **name**; a **fact** is a
node with outgoing arrows. Of a relation three things are kept apart: **the relation**, one atom; its
**extent**, every fact whose first arrow points at that atom, held or not; and **a fact of** it. The extent is
the same whatever is held. What a knowledge base holds of it is the **held extent**, and that is what grows,
what a rule reads, what a commitment adds to, and what is empty or complete. A fact that is held is in the
**global context**; a fact that a held fact points at, at any depth, is in that fact's **context**; a fact may be
in either or in both, and it is one node.

| symbol | what it is | where it is defined |
|---|---|---|
| $\Sigma$, $\Sigma^{+}$ | the name characters, and the names: the non-empty words over them | [the knowledge base](01-static-ir/01-knowledge-base.md#what-is-where) |
| $O$, $F$ | the objects: atoms, each its name in $\Sigma^{+}$, and facts over objects; the facts, $F = O \setminus \Sigma^{+}$ | [the knowledge base](01-static-ir/01-knowledge-base.md#what-is-where) |
| $\operatorname{out}(o)$ | the list of the objects under the arrows of the object $o$, counted from the first; empty for an atom | [the knowledge base](01-static-ir/01-knowledge-base.md#the-graph) |
| $r(v_1, \dots, v_n)$ | the fact whose first arrow points at the relation $r$ and whose arrows after it at the arguments $v_1, \dots, v_n$ | [the knowledge base](01-static-ir/01-knowledge-base.md#what-is-where) |
| $\operatorname{rel}(f)$, $\operatorname{arg}_i(f)$, $\operatorname{ar}(f)$ | the relation of a fact, its $i$-th argument, and how many arguments it has | [the knowledge base](01-static-ir/01-knowledge-base.md#what-is-where) |
| $\operatorname{ext}(r)$, $\operatorname{ext}_K(r)$ | the extent of the relation $r$: every fact whose first arrow points at $r$, held or not; and the held extent, the facts of $r$ that $K$ holds | [the knowledge base](01-static-ir/01-knowledge-base.md#the-relation-its-extent-a-fact-of-it) |
| $\operatorname{decl}(r;\ T_1, \dots, T_n)$ | the declaration of the relation $r$ with the labels $T_1, \dots, T_n$, a fact of $\mathsf{relation}$ | [the knowledge base](01-static-ir/01-knowledge-base.md#what-is-where) |
| $\lnot f$ | the stored negation of the fact $f$, itself a fact, of the relation $\mathsf{not}$ | [the knowledge base](01-static-ir/01-knowledge-base.md#a-fact-about-a-relation-a-fact-about-a-fact-a-negation-falsity) |
| $\bot$ | the fact of falsity, of the relation $\mathsf{false}$ and no argument | [the knowledge base](01-static-ir/01-knowledge-base.md#a-fact-about-a-relation-a-fact-about-a-fact-a-negation-falsity) |
| $K$, $K_0$ | a knowledge base: a finite set of facts, the ones its graph holds; the one a program's text is read into | [the knowledge base](01-static-ir/01-knowledge-base.md#the-graph), [its program](01-static-ir/01-knowledge-base.md#the-knowledge-base-of-a-program) |
| $\operatorname{sub}(f)$ | the facts nested in $f$, at any depth | [the knowledge base](01-static-ir/01-knowledge-base.md#nesting) |
| $\operatorname{ctx}_K(g)$ | the context of the fact $g$ in $K$: the held facts from which a path of argument arrows leads to $g$. $K$ itself is the global context | [the knowledge base](01-static-ir/01-knowledge-base.md#the-global-context-and-the-context-of-a-fact) |
| $\operatorname{names}(X)$ | the atoms under an argument arrow of a fact of $X$, at any depth | [the knowledge base](01-static-ir/01-knowledge-base.md#nesting) |
| $\preceq$, $\prec$ | the total order of the objects | [the knowledge base](01-static-ir/01-knowledge-base.md#order) |
| $\operatorname{text}(v)$ | the text of an object, which reads back as it | [the knowledge base](01-static-ir/01-knowledge-base.md#the-text-of-an-object) |
| $\operatorname{dead}(K)$, $\operatorname{alive}(K)$ | a rule that states the engine's word has a match in $K$, or none has: $K$ holds a contradiction, or holds none. Of a commitment, $\operatorname{dead}(C)$ is $\operatorname{dead}(\operatorname{st}(C))$, and so for every predicate of a state | [the knowledge base](01-static-ir/01-knowledge-base.md#a-fact-its-negation-both-or-neither), [the rules](01-static-ir/02-rewriting-rules.md#by-its-conclusion) |
| $\theta$ | a substitution of objects for variables | [the rules](01-static-ir/02-rewriting-rules.md#a-match) |
| $K \models \varphi\theta$ | the pattern $\varphi$ holds in $K$ under $\theta$ | [the rules](01-static-ir/02-rewriting-rules.md#the-conditions-on-a-match) |
| $\operatorname{absent}(G)$ | the condition that holds where the pattern $G$ has no match | [the rules](01-static-ir/02-rewriting-rules.md#the-conditions-on-a-match), [absence](02-kb-evolution/03-absent-and-naf.md) |
| $\mathsf{inconsistent}$, $P_{\mathsf{inc}}$ | the engine's word, a conclusion in the place of literals, which is no fact; the rules of $P$ that state it, which end a state | [the rules](01-static-ir/02-rewriting-rules.md#a-conclusion-and-the-result-of-rewriting) |
| $\psi\theta$, $K \cup \psi\theta$ | the instance of a conclusion under a match, a set of facts; the result of rewriting $K$ at that match | [the rules](01-static-ir/02-rewriting-rules.md#a-conclusion-and-the-result-of-rewriting) |
| $\rho = (\bar{p}, \varphi, \psi)$, $\operatorname{name}(\rho)$ | a rule: its parameters, its pattern (its match), its conclusion; and its name. $\varphi_\rho$ and $\psi_\rho$ where the rule has to be named | [the rules](01-static-ir/02-rewriting-rules.md#a-rule-declared) |
| $a$, $\operatorname{Act}_K(\rho)$, $\rho[a]$ | an activator of $\rho$, a fact of $K$ whose relation is the rule's name; the set of them; the rule applied under $a$, with pattern $\varphi[a]$ and conclusion $\psi[a]$ | [the rules](01-static-ir/02-rewriting-rules.md#a-rule-declared) |
| $P$ | the program, and its rule set | [the knowledge base](01-static-ir/01-knowledge-base.md#the-knowledge-base-of-a-program), [the rules](01-static-ir/02-rewriting-rules.md#the-program-and-its-names) |
| $\operatorname{names}(P)$ | the names of the program: those of $K_0$ and of its rules, a finite subset of $\Sigma^{+}$ | [the rules](01-static-ir/02-rewriting-rules.md#the-program-and-its-names) |
| $\operatorname{labels}(P)$, $\operatorname{guessable}(P)$ | the labels the program's declarations write; the guessable atoms, the names less the labels, the reserved words and the names of relations and rules | [the knowledge base](01-static-ir/01-knowledge-base.md#what-is-where), [hypothesis generation](02-kb-evolution/04-hypothesis-generation.md#5-the-blind-enumerator) |
| $S$ | a state: the set of facts a saturation ends in, root's or a node's | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $J$, $J_K(f)$ | the provenance: the derivations recorded for the fact $f$ in $K$, the primary one first | [cores](02-kb-evolution/09-cores.md#a-derivation-record) |
| $\delta$; $\operatorname{prem}(\delta)$, $\operatorname{all}(\delta)$, $\operatorname{abs}(\delta)$; $\operatorname{rests}(\delta)$ | a derivation; the premises it matched, the facts a universal premise ranged over, the absences that had to hold; the facts it rests on | [cores](02-kb-evolution/09-cores.md#a-derivation-record) |
| $\lt_{H}$ | the order of the candidates, by their text | [solution semantics](02-kb-evolution/06-solution-semantics.md#14-candidate-and-universe) |
| $\xi = (\rho, a, \theta)$, $\operatorname{en}(K)$, $\operatorname{fire}(K, \xi)$ | a firing; the firings enabled in $K$; $K$ with the firing's conclusion added | [firing a rule](02-kb-evolution/01-firing.md) |
| $T_P$ | the consequence operator: every enabled firing at once | [saturation](02-kb-evolution/02-saturation.md) |
| $\operatorname{sat}$, $\operatorname{pos}$ | saturation, a knowledge base taken to its fixpoint under $P$; the closure under the firings that read no absence | [saturation](02-kb-evolution/02-saturation.md) |
| $\kappa$, $\lhd$ | a candidate of a saturation, a firing with the disjunct it matched; the agenda's order of the candidates | [saturation](02-kb-evolution/02-saturation.md) |
| $\operatorname{rels}(X)$ | the relations of the facts of $X$ | [saturation](02-kb-evolution/02-saturation.md) |
| $D$, $D^{+}(X)$ | the relations whose held extent a commitment can change; the same closure from the relations $X$, followed through a rule's activator as well, $D^{+}$ where $X$ is the candidates' relations | [monotonicity](02-kb-evolution/08-monotonicity.md), [absence](02-kb-evolution/03-absent-and-naf.md#a-refutation-resting-on-an-absent) |
| $\mathrm{root}$ | the program's own facts, saturated: $\mathrm{root} = \operatorname{sat}(K_0)$ | [firing a rule](02-kb-evolution/01-firing.md#two-layers), [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $H$, $n = \lvert H \rvert$, $\operatorname{gen}(P, \mathrm{root})$ | the candidate set, its size, and the writers that build it | [firing a rule](02-kb-evolution/01-firing.md#two-layers), [hypothesis generation](02-kb-evolution/04-hypothesis-generation.md), [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $C \subseteq H$ | a commitment, a node of the lattice $(2^{H}, \subseteq)$; $\uparrow C$ and $\downarrow C$ are the commitments above and below it | [firing a rule](02-kb-evolution/01-firing.md#two-layers), [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\operatorname{st}(C) = \operatorname{sat}(\mathrm{root} \cup C)$ | the state of the node $C$ | [firing a rule](02-kb-evolution/01-firing.md#two-layers), [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\operatorname{cl}(C)$, $C \sim C'$, $[C]$ | the closure of $C$, the candidates its state holds; two commitments of one state; the class of $C$ | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\operatorname{owes}(o, a, \theta, S)$ | the state $S$ owes the instance $(o, a, \theta)$ of the obligation $o$, whose trigger $T_o$ holds with no witness $W_o$ | [hypothesis generation](02-kb-evolution/04-hypothesis-generation.md#41-an-instance-and-the-facts-that-would-pay-it) |
| $\operatorname{Owed}(S)$, $\operatorname{owes}(S)$, $\operatorname{discharged}(S)$ | the instances a state owes; their number; none is owed | [solution semantics](02-kb-evolution/06-solution-semantics.md#15-commitment-node-state) |
| $\operatorname{pay}(I)$, $\operatorname{alt}(I)$ | the facts that would pay the owed instance $I$; those of them that are candidates, its alternatives | [hypothesis generation](02-kb-evolution/04-hypothesis-generation.md), [finding R](02-kb-evolution/07-finding-r.md) |
| $R$, $k_c$, $k_s$ | the region, the commitments whose state is alive and discharged; its size in commitments, and in states | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\operatorname{Dead}$, $\operatorname{Alive}$, $\operatorname{Owing}$ | the sets of commitments that are dead, alive, alive and owing | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\min R$, $\max R$, $N$, $E$ | the two lines of $R$; the minimal dead sets; the minimal discharging sets | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\operatorname{MinTr}(\mathcal{F})$ | the minimal transversals of a family $\mathcal{F}$ of sets | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\operatorname{model}(C)$, $\pi_g$ | what a solution's state adds to the program's facts; its projection on a goal $g$ | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\operatorname{core}(K)$ | of a dead knowledge base, a smallest set of givens and commitments its contradiction follows from | [cores](02-kb-evolution/09-cores.md) |
| $\mathcal{S}(X)$ | the states of a set $X$ of commitments, $\mathcal{S}(R)$ those of the region | [firing a rule](02-kb-evolution/01-firing.md), [satisfiability](02-kb-evolution/10-satisfiability.md) |
| $G_P = (\mathcal{V}, \mathcal{E})$, $\sigma$ | the dependency graph of the rule set, and the stratum of a relation or a rule | [analysis of rules](03-rule-evolution/02-analysis-of-rules.md) |
| $\operatorname{Ans}(P)$ | everything an answer states of a program | [transformations](03-rule-evolution/03-transformations.md) |

**The letters of one page.**

| page | its own symbols |
|---|---|
| [the knowledge base](01-static-ir/01-knowledge-base.md) | $G$, $O_K$ the graph of a knowledge base and its objects; $\nu$ the reading of written tuples; $W$, $W_{\!c}$ the reserved words; $T$, $D$ the facts the text writes and those its declarations are; $\alpha_{\text{file}}$ a file's short names; $q$, $\lt_{q}$ the quoted form of an object and its order; $\operatorname{status}_K$; $N_K$ the facts whose stored negation $K$ holds; $s$, $e$, $t$ the three facts of the example of a context |
| [the rules](01-static-ir/02-rewriting-rules.md) | $X$ the variables; $t$ a term; $m$ a match as a homomorphism; $\gamma$ a part of a pattern; $N$ the atoms a pattern is matched over; $\ell$ a literal; $L$, $I$, $L^{+}$ the span of a rewrite; $G$, $B$ the domain and the body of a universal |
| [firing a rule](02-kb-evolution/01-firing.md) | $\iota$, $w$, $\operatorname{Obs}$ a numbering of the objects, a count of workers, what a reader observes of a run; $K(t)$ a knowledge base after $t$ additions |
| [saturation](02-kb-evolution/02-saturation.md) | $\Xi$, $Q$ the firings root made and the candidates it parked; $\operatorname{lead}$, $\operatorname{arr}$, $\operatorname{passes}$; $w(r(\bar x))$ the side of a pair that holds the other; $X$ the kinds of name that are not guessable |
| [absence](02-kb-evolution/03-absent-and-naf.md) | $G$ the pattern under an absence; $\operatorname{refutes}(\rho)$ a rule whose firing can make a state dead; $\operatorname{abs}^{\ast}(\rho)$ the absences a rule's conclusion can depend on; $\operatorname{dep}(f)$; $\mathrm{end}$ the node of the rules that end a state |
| [hypothesis generation](02-kb-evolution/04-hypothesis-generation.md) | $\nu(P)$, $\beta(P)$ the two tests of the definition; $\operatorname{hyp}(S)$ the hypotheses a state holds; $\operatorname{Hr}(P)$ the hypothesis rules; $\operatorname{ActOb}(P)$ the obligations activated at root; $\operatorname{listed}(h)$ the activators of a hypothesis rule; $\Pi$ and $H$ with a writer's subscript, its proposals and its candidates, $\Pi_P$ the hypotheses the program states; $\operatorname{unsettled}$, $\operatorname{denies}$; $T_o$, $\Delta_o$, $w_o$, $W_o$ the parts of an obligation; $I$ an instance; $\operatorname{grows}(o)$; $L^{+}$, $L^{-}$ the two lists; $\operatorname{Closed}$, $\operatorname{inscope}$, $\operatorname{Elig}$, $\operatorname{Uncov}$, $\operatorname{Op}$, $\operatorname{stuck}$; $\operatorname{Kw}$, $\operatorname{Res}$, $\operatorname{RelNames}$ the kinds of atom that are not guessable; $\operatorname{reads}$, $\operatorname{concludes}$ |
| [the hypothesis space](02-kb-evolution/05-hypothesis-space.md) | $a \rightarrow b$, $a \leftrightarrow b$, $a \perp b$, $a \bowtie b$ the pair relations; $X$, $X^{\circ}$ the excluding pairs; $H^{\mathrm{o}}$ the open candidates; $K_0^{\circ}$, $\mathrm{root}^{\circ}$ the theory's share; $\mathcal{G}$, $g$ the groups; $Y$ a grid; $Q$, $W$ the questions and the whole node; $\operatorname{Puz}$, $\operatorname{given}$, $\operatorname{clue}$, $\operatorname{Ties}$ |
| [solution semantics](02-kb-evolution/06-solution-semantics.md) | $\Theta$, $\Omega$, $\Phi$ the context and the problem; $\mathcal{L}$, $L_k$ the lattice and its layers; $[a, b]$ a sub-cube; $\operatorname{verdict}(R)$ |
| [finding R](02-kb-evolution/07-finding-r.md) | $\mathcal{O}$ the oracle's table; $\mathcal{K}$ the sets known dead; $\mathcal{C}$ the alive closed sets; $s$, $T$ closed sets; $M(T)$; $\mathcal{W}$; $\Lambda$; $\operatorname{Leaves}(C)$; L0 to L8 the licences |
| [monotonicity](02-kb-evolution/08-monotonicity.md) | $\operatorname{site}$, $\operatorname{depth}$, $\operatorname{stale}$; $\mathit{Sh}$ the closure over shapes |
| [cores](02-kb-evolution/09-cores.md) | $f$, $n$, $q$ the nodes of the contradiction rule's pattern; $\operatorname{clash}_n$ the activator of the rule for a count $n$; $\mathcal{T}(K)$ the terminals; $Z$ a set of them; $\operatorname{der}_K(Z)$, $\operatorname{contr}(K)$, $\operatorname{by}(f)$; $\operatorname{origin}$, $\operatorname{front}$ |
| [satisfiability](02-kb-evolution/10-satisfiability.md) | $\Gamma$ a claim; $\operatorname{holds}(\Gamma, \mathcal{S})$; $\operatorname{alt}(I, S)$ the alternatives a state has not denied; $\operatorname{rows}_g$ |
| [rules as data](03-rule-evolution/01-rules-as-data.md) | $W_{\!d}$ the declaring words |
| [analysis of rules](03-rule-evolution/02-analysis-of-rules.md) | $\operatorname{reads}$, $\operatorname{writes}$, $\operatorname{sgn}$; $c$ a strongly connected component; $\chi$ |
| [transformations](03-rule-evolution/03-transformations.md) | $\tau$ a transformation; $\beta$ a renaming; $\alpha$ the map to full names; $\operatorname{resolve}$, $\operatorname{expand}$, $\operatorname{lits}$, $\operatorname{inst}_K(P)$ |

**Graphs.** A knowledge base is a graph, and the pages draw it as it is: a box is an atom, with its name
in it; an octagon is a fact, with no name in it, in a dashed frame where the knowledge base only points at it and
does not hold it; an arrow carries its place among the arrows of its fact, $1$ on the first, which points at the
relation; a diamond is a variable; a dashed octagon, with its dashed arrows, is a fact that was derived, or, in a
rule, a fact the conclusion adds ([the knowledge base](01-static-ir/01-knowledge-base.md#the-graph)). No relation
is drawn otherwise than another. A drawing may **fold** a fact with a relation and two arguments that no fact
points at into one arrow from the first argument to the second, labelled with the relation's name, and a fact with a
relation and one argument into a loop on its argument, labelled the same way, the way a program is often drawn for
its author ([rendering](../user/04-reference/12-rendering.md)); a drawing that folds says so beside it, and
[the knowledge base](01-static-ir/01-knowledge-base.md#drawn-with-labelled-arrows) says what the fold stands for and
what it loses. Nothing else is folded. A drawing of facts stands in layers, each relation above its facts and each
fact above its arguments, where its layout allows.
Every drawing is an image, and its source is the file of the same name in the `diagrams/` folder beside
its page ([Q-M1f.368](../../decisions/docs/Q-M1f.368-a-diagram-is-a-file-and-a-page-shows-its-picture.md)).

## Reading order

Three parts, in the order a program lives through them.

### The program, up to its loaded state — [`01-static-ir/`](01-static-ir/)

1. [The knowledge base: facts of relations](01-static-ir/01-knowledge-base.md): the graph; what is where in it, objects, atoms and names,
   facts, relations, declarations; and their properties, identity, order and the rest.
2. [The rules: rewriting the graph](01-static-ir/02-rewriting-rules.md): a pattern, a match, the conditions on one, a conclusion and the result
   of rewriting; then how a rule is declared, the kinds of rule, and the four levels a knowledge base describes
   itself in.

### The knowledge base as it grows — [`02-kb-evolution/`](02-kb-evolution/)

1. [Firing a rule](02-kb-evolution/01-firing.md): a firing and its identity; the two layers, with root, a
   candidate, a commitment, a node and its state, and a fork; what Ein answers; and the operations every
   implementation performs.
2. [Saturation](02-kb-evolution/02-saturation.md): the fixpoint, the agenda's order and what the order of the
   written facts cannot decide, and the alive-set invariant.
3. [Negation as failure](02-kb-evolution/03-absent-and-naf.md): what an absence reads, and in which world; a
   refutation that rests on one, the relations a commitment can extend, and the end of a state as the last stratum.
4. [Hypothesis generation](02-kb-evolution/04-hypothesis-generation.md): the writers that build the candidate set
   $H$; hypothesis rules, obligations and what would pay one, the blind enumerator and the guessable atoms.
5. [The hypothesis space and its structure](02-kb-evolution/05-hypothesis-space.md): $H$, and what the theory says of
   its candidates before any search.
6. [Solution semantics: the region R](02-kb-evolution/06-solution-semantics.md): the answer, defined, with what
   a state owes, the identity of a state and the order of the candidates.
7. [Finding R: the oracle and the routes](02-kb-evolution/07-finding-r.md): every way $R$ is computed, and why they
   agree.
8. [Monotone and non-monotone programs](02-kb-evolution/08-monotonicity.md): when the routes' licences hold, and what
   is claimed when they do not.
9. [Cores: what a contradiction rests on](02-kb-evolution/09-cores.md): the contradiction rule and why it is a
   schema, how each fact came to be held, and a dead node's explanation.
10. [Satisfiability: what a program can say of its answer](02-kb-evolution/10-satisfiability.md): where a rule stops
    being a sentence about the world it fires in.

### The rule set as a thing to reason about — [`03-rule-evolution/`](03-rule-evolution/)

1. [Rules as data](03-rule-evolution/01-rules-as-data.md): what of Ein describes itself in Ein, and where that stops.
2. [Analysis of rules](03-rule-evolution/02-analysis-of-rules.md): the dependency graph of a rule set and its strata.
3. [Transformations of a rule set](03-rule-evolution/03-transformations.md): the rewrites of a rule set that leave
   $R$ unchanged.
