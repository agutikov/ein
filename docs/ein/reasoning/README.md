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
an object: a set, a relation, a function, an order. A proposition says what holds of every object that satisfies
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
operator, $\operatorname{lives-in}$. Italic letters range over what the table says: $u, v, w$ over values,
$f, g$ over facts, $h$ over candidates, $r$ over relation names, $x, y, z$ over variables, $a$ over activators,
and, where no rule is in view, $a, b, c$ over names or over candidates, as the page says.

| symbol | what it is | where it is defined |
|---|---|---|
| $\Sigma$, $\Sigma^{+}$ | the name characters, and the names: the non-empty words over them | [names and atoms](01-static-ir/01-names-and-atoms.md) |
| $V$, $F$, $A$ | the values: names, and facts over values; the facts, $F = V \setminus \Sigma^{+}$; the atoms, the facts that are no stored negation | [names and atoms](01-static-ir/01-names-and-atoms.md) |
| $r(v_1, \dots, v_n)$ | the fact of relation $r$ over the arguments $v_1, \dots, v_n$ | [names and atoms](01-static-ir/01-names-and-atoms.md) |
| $\operatorname{rel}(f)$, $\operatorname{arg}_i(f)$, $\operatorname{ar}(f)$ | the relation of a fact, its $i$-th argument, and how many arguments it has | [names and atoms](01-static-ir/01-names-and-atoms.md) |
| $\operatorname{sub}(f)$ | the facts nested in $f$, at any depth | [names and atoms](01-static-ir/01-names-and-atoms.md) |
| $\operatorname{decl}(r;\ T_1, \dots, T_n)$ | the declaration of the relation $r$ with the labels $T_1, \dots, T_n$ | [names and atoms](01-static-ir/01-names-and-atoms.md) |
| $\operatorname{names}(X)$ | the names that stand in an argument place of a fact of $X$, at any depth | [names and atoms](01-static-ir/01-names-and-atoms.md) |
| $\operatorname{names}(P)$ | the names of the program: those of its written facts and of its rules, a finite subset of $\Sigma^{+}$ | [names and atoms](01-static-ir/01-names-and-atoms.md) |
| $\operatorname{labels}(P)$, $\operatorname{obj}(P)$ | the labels the program's declarations write; the objects, the names less the labels, the reserved words and the names of relations and rules | [names and atoms](01-static-ir/01-names-and-atoms.md), [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\lnot f$ | the stored negation of the fact $f$, itself a fact | [relations and facts](01-static-ir/02-relations-and-facts.md) |
| $\bot$ | the fact of falsity, of no argument | [relations and facts](01-static-ir/02-relations-and-facts.md) |
| $K$, $K_0$ | a knowledge base: a finite set of facts, read with the program and the provenance of its facts; the one a program's text is read into | [relations and facts](01-static-ir/02-relations-and-facts.md) |
| $S$ | a state: the set of facts a saturation ends in, root's or a node's | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\operatorname{dead}(K)$, $\operatorname{alive}(K)$ | $K$ holds a contradiction, or holds none. Of a commitment, $\operatorname{dead}(C)$ is $\operatorname{dead}(\operatorname{st}(C))$, and so for every predicate of a state | [relations and facts](01-static-ir/02-relations-and-facts.md) |
| $J$, $J_K(f)$ | the provenance: the derivations recorded for the fact $f$ in $K$, the primary one first | [relations and facts](01-static-ir/02-relations-and-facts.md) |
| $\delta$; $\operatorname{prem}(\delta)$, $\operatorname{all}(\delta)$, $\operatorname{abs}(\delta)$; $\operatorname{rests}(\delta)$ | a derivation; the premises it matched, the facts a universal premise ranged over, the absences that had to hold; the facts it rests on | [relations and facts](01-static-ir/02-relations-and-facts.md), [cores](02-kb-evolution/09-cores.md) |
| $\preceq$, $\prec$ | the total order of the values | [values and their order](01-static-ir/03-values-and-order.md) |
| $\lt_{H}$ | the order of the candidates, by their text | [values and their order](01-static-ir/03-values-and-order.md) |
| $P$ | the program, and its rule set | [rules as statements](01-static-ir/04-rules-as-statements.md) |
| $\rho = (\bar{p}, \varphi, \psi)$, $\operatorname{name}(\rho)$ | a rule: its parameters, its pattern (its match), its conclusion; and its name. $\varphi_\rho$ and $\psi_\rho$ where the rule has to be named | [rules as statements](01-static-ir/04-rules-as-statements.md) |
| $\theta$ | a substitution of values for variables | [rules as statements](01-static-ir/04-rules-as-statements.md) |
| $K \models \varphi\theta$ | the pattern $\varphi$ holds in $K$ under $\theta$ | [rules as statements](01-static-ir/04-rules-as-statements.md) |
| $a$, $\operatorname{Act}_K(\rho)$, $\rho[a]$ | an activator of $\rho$, a fact of $K$ whose relation is the rule's name; the set of them; the rule applied under $a$, with pattern $\varphi[a]$ and conclusion $\psi[a]$ | [rules as statements](01-static-ir/04-rules-as-statements.md), [values and their order](01-static-ir/03-values-and-order.md) |
| $\xi = (\rho, a, \theta)$, $\operatorname{en}(K)$, $\operatorname{fire}(K, \xi)$ | a firing; the firings enabled in $K$; $K$ with the firing's conclusion added | [firing a rule](02-kb-evolution/01-firing.md) |
| $T_P$ | the consequence operator: every enabled firing at once | [saturation](02-kb-evolution/02-saturation.md) |
| $\operatorname{sat}$, $\operatorname{pos}$ | saturation, a knowledge base taken to its fixpoint under $P$; the closure under the firings that read no absence | [saturation](02-kb-evolution/02-saturation.md) |
| $\kappa$, $\lhd$ | a candidate of a saturation, a firing with the disjunct it matched; the agenda's order of the candidates | [saturation](02-kb-evolution/02-saturation.md) |
| $\operatorname{absent}(G)$ | the pattern that holds where $G$ matches nothing | [rules as statements](01-static-ir/04-rules-as-statements.md), [absence](02-kb-evolution/03-absent-and-naf.md) |
| $\operatorname{rels}(X)$ | the relations of the facts of $X$ | [saturation](02-kb-evolution/02-saturation.md) |
| $D$, $D^{+}(X)$ | the relations a commitment can change; the same closure from the relations $X$, followed through a rule's activator as well | [monotonicity](02-kb-evolution/08-monotonicity.md), [hypothesis generation](02-kb-evolution/04-hypothesis-generation.md) |
| $\mathrm{root}$ | the program's own facts, saturated: $\mathrm{root} = \operatorname{sat}(K_0)$ | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $H$, $n = \lvert H \rvert$, $\operatorname{gen}(P, \mathrm{root})$ | the candidate set, its size, and the ladder that builds it | [solution semantics](02-kb-evolution/06-solution-semantics.md), [hypothesis generation](02-kb-evolution/04-hypothesis-generation.md) |
| $C \subseteq H$ | a commitment, a node of the lattice $(2^{H}, \subseteq)$; $\uparrow C$ and $\downarrow C$ are the commitments above and below it | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\operatorname{st}(C) = \operatorname{sat}(\mathrm{root} \cup C)$ | the state of the node $C$ | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\operatorname{cl}(C)$, $C \sim C'$, $[C]$ | the closure of $C$, the candidates its state holds; two commitments of one state; the class of $C$ | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\operatorname{owes}(o, a, \theta, S)$, $\operatorname{Owed}(S)$, $\operatorname{owes}(S)$, $\operatorname{discharged}(S)$ | the state $S$ owes the instance $(o, a, \theta)$ of the obligation $o$, whose trigger $T_o$ holds with no witness $W_o$; the instances owed; their number; none is owed | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
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
| [names and atoms](01-static-ir/01-names-and-atoms.md) | $X$ the variables; $W$, $W_{\!c}$ the reserved words; $T$ the facts the text writes; $\alpha_{\text{file}}$ a file's short names |
| [relations and facts](01-static-ir/02-relations-and-facts.md) | $G(K)$ the hypergraph of $K$; $D$ the facts declarations are held as; $\Delta$ a set of added facts; $Z$ an explanation; $\operatorname{status}_K$, $\operatorname{supported}$, $\operatorname{front}$ |
| [values and their order](01-static-ir/03-values-and-order.md) | $\iota$, $w$, $O$ a numbering of the names, a count of workers, what a reader observes; $\operatorname{text}$, $q$ a value as text and as quoted in a message |
| [rules as statements](01-static-ir/04-rules-as-statements.md) | $L$, $I$, $L^{+}$ the span of a rewrite; $\ell$ the level of a node; $\gamma$, $\beta$ the domain and the body of a universal |
| [saturation](02-kb-evolution/02-saturation.md) | $\Xi$, $Q$ the firings root made and the candidates it parked; $\operatorname{lead}$, $\operatorname{arr}$, $\operatorname{passes}$; $X$ the kinds of name that are no object |
| [absence](02-kb-evolution/03-absent-and-naf.md) | $G$ the pattern under an absence; $\operatorname{abs}^{\ast}(\rho)$ the absences a rule's conclusion can depend on; $\operatorname{dep}(f)$ |
| [hypothesis generation](02-kb-evolution/04-hypothesis-generation.md) | $\nu(P)$, $\operatorname{Hr}(P)$, $\operatorname{ActOb}(P)$, $\operatorname{declines}(P)$ the four tests of the ladder; $\operatorname{listed}(h)$ the activators a program lists for a hypothesis rule; $\Pi$ and $H$ with a rung's subscript, its proposals and its candidates; $\operatorname{unsettled}$; $T_o$, $\Delta_o$, $w_o$, $W_o$ the parts of an obligation; $I$ an instance; $L^{+}$, $L^{-}$ the two lists; $\operatorname{Closed}$, $\operatorname{inscope}$, $\operatorname{Elig}$, $\operatorname{Uncov}$, $\operatorname{Prop}$, $\operatorname{Ow}$, $\operatorname{stuck}$; $\operatorname{Kw}$, $\operatorname{Res}$, $\operatorname{RelNames}$ |
| [the hypothesis space](02-kb-evolution/05-hypothesis-space.md) | $a \rightarrow b$, $a \leftrightarrow b$, $a \perp b$, $a \bowtie b$ the pair relations; $X$, $X^{\circ}$ the excluding pairs; $H^{\mathrm{o}}$ the open candidates; $K_0^{\circ}$, $\mathrm{root}^{\circ}$ the theory's share; $\mathcal{G}$, $g$ the groups; $Y$ a grid; $Q$, $W$ the questions and the whole node; $\operatorname{Puz}$, $\operatorname{Ties}$ |
| [solution semantics](02-kb-evolution/06-solution-semantics.md) | $\Theta$, $\Omega$, $\Phi$ the context and the problem; $\mathcal{L}$, $L_k$ the lattice and its layers; $[a, b]$ a sub-cube; $\operatorname{verdict}(R)$ |
| [finding R](02-kb-evolution/07-finding-r.md) | $\mathcal{O}$ the oracle's table; $\mathcal{K}$ the sets known dead; $\mathcal{C}$ the alive closed sets; $s$, $T$ closed sets; $M(T)$; $\mathcal{W}$; $\Lambda$; $\operatorname{Leaves}(C)$; L0 to L8 the licences |
| [monotonicity](02-kb-evolution/08-monotonicity.md) | $\operatorname{site}$, $\operatorname{depth}$, $\operatorname{stale}$, $\operatorname{watch}$; $\mathit{Sh}$ the closure over shapes |
| [cores](02-kb-evolution/09-cores.md) | $\mathcal{T}(K)$ the terminals; $Z$ a set of them; $\operatorname{der}_K(Z)$, $\operatorname{contr}(K)$, $\operatorname{by}(f)$ |
| [satisfiability](02-kb-evolution/10-satisfiability.md) | $\Gamma$ a claim; $\operatorname{holds}(\Gamma, \mathcal{S})$; $\operatorname{alt}(I, S)$ the alternatives a state has not denied; $\operatorname{key}$, $\operatorname{rows}_g$ |
| [rules as data](03-rule-evolution/01-rules-as-data.md) | $W_{\!d}$ the declaring words; $\operatorname{ext}_K(r)$ the extent of a relation |
| [analysis of rules](03-rule-evolution/02-analysis-of-rules.md) | $\operatorname{reads}$, $\operatorname{writes}$, $\operatorname{sgn}$; $c$ a strongly connected component; $\chi$ |
| [transformations](03-rule-evolution/03-transformations.md) | $\tau$ a transformation; $\beta$ a renaming; $\alpha$ the map to full names; $\operatorname{resolve}$, $\operatorname{expand}$, $\operatorname{lits}$, $\operatorname{inst}_K(P)$ |

**Graphs.** A drawing of facts follows one vocabulary, the one in which a program is drawn for its author
([rendering](../user/04-reference/12-rendering.md)): a name is a node; a fact of two arguments is an arrow from the
first to the second, labelled by its relation; a fact of any other count is a node of its own with one numbered
arrow to each argument; a variable is a diamond; a dashed arrow is a fact that was derived, or, in a rule, the
fact a conclusion adds.

## Reading order

Three parts, in the order a program lives through them.

### The program, up to its loaded state — [`01-static-ir/`](01-static-ir/)

1. [Names and atoms](01-static-ir/01-names-and-atoms.md): names, values and facts, and identity by name.
2. [Relations and facts](01-static-ir/02-relations-and-facts.md): the knowledge base as a state and its provenance.
3. [Values and their order](01-static-ir/03-values-and-order.md): the orders a reader can observe, and the identity of
   a state and of a rule application.
4. [Rules as statements](01-static-ir/04-rules-as-statements.md): the kinds of rule, a rule as a rewrite, and the four
   levels a knowledge base describes itself in.

### The knowledge base as it grows — [`02-kb-evolution/`](02-kb-evolution/)

1. [Firing a rule](02-kb-evolution/01-firing.md): what Ein solves, and the operations every implementation performs.
2. [Saturation](02-kb-evolution/02-saturation.md): the fixpoint, the agenda's order, and the alive-set invariant.
3. [Negation as failure](02-kb-evolution/03-absent-and-naf.md): what an absence reads, and in which world.
4. [Hypothesis generation](02-kb-evolution/04-hypothesis-generation.md): the ladder that builds the candidate set $H$.
5. [The hypothesis space and its structure](02-kb-evolution/05-hypothesis-space.md): $H$, and what the theory says of
   its candidates before any search.
6. [Solution semantics: the region R](02-kb-evolution/06-solution-semantics.md): the answer, defined.
7. [Finding R: the oracle and the routes](02-kb-evolution/07-finding-r.md): every way $R$ is computed, and why they
   agree.
8. [Monotone and non-monotone programs](02-kb-evolution/08-monotonicity.md): when the routes' licences hold, and what
   is claimed when they do not.
9. [Cores: what a contradiction rests on](02-kb-evolution/09-cores.md): a dead node's explanation.
10. [Satisfiability: what a program can say of its answer](02-kb-evolution/10-satisfiability.md): where a rule stops
    being a sentence about the world it fires in.

### The rule set as an object — [`03-rule-evolution/`](03-rule-evolution/)

1. [Rules as data](03-rule-evolution/01-rules-as-data.md): what of Ein describes itself in Ein, and where that stops.
2. [Analysis of rules](03-rule-evolution/02-analysis-of-rules.md): the dependency graph of a rule set and its strata.
3. [Transformations of a rule set](03-rule-evolution/03-transformations.md): the rewrites of a rule set that leave
   $R$ unchanged.
