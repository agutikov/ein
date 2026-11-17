# Saturation

Saturation is the one computation every answer is made of. Root is the program's own facts saturated by its
rules, $\mathrm{root} = \operatorname{sat}(K_0)$; every node of the search is root's fixpoint with a
commitment added, saturated again, $\operatorname{st}(C) = \operatorname{sat}(\mathrm{root} \cup C)$; the
candidate set $H$ is read off root's fixpoint, and whether a node is dead and what it owes off its own
([solution semantics](06-solution-semantics.md)). This page states what $\operatorname{sat}$ computes, the order
it derives in and what that order can and cannot decide, what it derives before anything is supposed, and the
invariant that lets $H$ be taken once, at root. The symbols are those of [firing a rule](01-firing.md).

## The fixpoint

**The consequence operator.** One step applies every enabled firing at once:

$$
T_P(K) \;=\; K \;\cup \bigcup_{(\rho, a, \theta) \,\in\, \operatorname{en}(K)} \psi_\rho[a]\,\theta
$$

$T_P$ is **inflationary**, $K \subseteq T_P(K)$: firing adds the literals a conclusion states, stored negations
among them, and never removes one. Within one world the set of facts only grows, and nothing is retracted.

**Where no rule reads an absence, $T_P$ is monotone**, since a match that holds in $K$ holds in every larger
knowledge base:

$$
K \subseteq K' \implies \operatorname{en}(K) \subseteq \operatorname{en}(K') \implies T_P(K) \subseteq T_P(K')
$$

and $\operatorname{sat}(K)$ is the least knowledge base containing $K$ that no firing changes, the limit of the
chain

$$
K \;\subseteq\; T_P(K) \;\subseteq\; T_P^2(K) \;\subseteq\; \cdots,
\qquad
\operatorname{sat}(K) \;=\; \bigcup_{n \geq 0} T_P^{\,n}(K)
\;=\; \min \lbrace\, K' \supseteq K \;:\; T_P(K') = K' \,\rbrace
$$

Three properties of that fixpoint are used on every later page:

$$
K \subseteq \operatorname{sat}(K), \qquad
\operatorname{sat}(\operatorname{sat}(K)) = \operatorname{sat}(K), \qquad
K \subseteq K' \implies \operatorname{sat}(K) \subseteq \operatorname{sat}(K')
$$

It is extensive, idempotent and monotone: a **closure operator**.

**With an absence the third property can fail.** A premise $\operatorname{absent}(G)$ holds in $K$ and may fail in
a larger $K'$, so a larger input can block a firing a smaller one admits, $\operatorname{en}(K) \not\subseteq
\operatorname{en}(K')$. What $\operatorname{sat}$ is on a program that reads absences is fixed by the next section
and by [absence](03-absent-and-naf.md): the least fixpoint above is taken on the positive part of the program,
and a boundary decides the rest. The first two properties hold of every program.

**Termination is assumed, not given.** A rule may build a fact out of a fact, so nesting can grow without bound
and the chain above need not become stationary. Everything stated about an answer is about programs whose
saturation reaches a fixpoint:

$$
\exists n.\; T_P^{\,n+1}(K) = T_P^{\,n}(K)
$$

**A derivation is a graph.** Read with what each firing matched, the chain is a directed acyclic graph: a fact
is a node, a firing is a node, an arrow leads from each premise into the firing and from the firing to what it
adds. Two firings below take three stated facts to a contradiction; the first adds
$\operatorname{near}(\mathsf{Norwegian}, \mathsf{House\text{-}2})$ and the second reads it.

```dot
digraph derivation {
  rankdir=LR;
  node [fontname="Inter"];
  "f_ba78d8e6bc" [label="lives-in(Norwegian, House-1)", shape=box, style=rounded];
  "f_203bb25e0e" [label="next-to(House-1, House-2)", shape=box, style=rounded];
  "f_e42df9f04f" [label="owns(Norwegian, Fox)", shape=box, style=rounded];
  "f_cb1915def2" [label="near(Norwegian, House-2)", shape=box, style=rounded, penwidth=2];
  "f_a1f94f5b63" [label="false", shape=box, style=rounded, penwidth=2];
  "fire0_neighbour" [shape=box, style="rounded,bold", color="#e377c2", fontcolor="#e377c2", label="neighbour"];
  "fire0_fox-keeps-away" [shape=box, style="rounded,bold", color="#17becf", fontcolor="#17becf", label="fox-keeps-away"];
  "f_ba78d8e6bc" -> "fire0_neighbour" [color="#e377c2"];
  "f_203bb25e0e" -> "fire0_neighbour" [color="#e377c2"];
  "fire0_neighbour" -> "f_cb1915def2" [color="#e377c2", style=bold];
  "f_e42df9f04f" -> "fire0_fox-keeps-away" [color="#17becf"];
  "f_cb1915def2" -> "fire0_fox-keeps-away" [color="#17becf"];
  "fire0_fox-keeps-away" -> "f_a1f94f5b63" [color="#17becf", style=bold];
}
```

A thin box is a fact, a heavy box a fact a firing added, a coloured box a firing. The facts with no arrow in are
$K$; each column further right is one more application of $T_P$:

$$
K \;\subset\; T_P(K) = K \cup \lbrace \operatorname{near}(\mathsf{Norwegian}, \mathsf{House\text{-}2}) \rbrace
\;\subset\; T_P^2(K) = T_P(K) \cup \lbrace \bot \rbrace = \operatorname{sat}(K)
$$

## Two phases: the closure and the boundary

Saturation alternates two phases, and that shape is the contract.

**Candidates.** Write a rule's match in disjunctive form, each disjunct a positive part and its guards:

$$
\varphi \;=\; \bigvee_{d} \Bigl( \varphi_d^{+} \;\land\; \bigwedge_{j} \operatorname{absent}(G_{d,j}) \Bigr)
$$

where $\varphi_d^{+}$ holds facts, stored negations and comparisons, and no absence. A **candidate** is
$\kappa = (\rho, a, \theta, d)$ with $K \models \varphi_d^{+}\theta$. It is **positive** when its disjunct has no guard,
and **parked** otherwise. A parked candidate **passes** in $K$ when every guard of its disjunct holds there:

$$
\operatorname{passes}(\kappa, K) \iff \forall j.\;\; K \models \operatorname{absent}(G_{d,j})\,\theta
$$

**The closure** consults no negation. It is the least fixpoint of the positive candidates alone:

$$
T_P^{+}(K) \;=\; K \;\cup \bigcup_{\kappa \text{ positive in } K} \psi_\kappa\,\theta,
\qquad
\operatorname{pos}(K) \;=\; \bigcup_{n \geq 0} \bigl(T_P^{+}\bigr)^{n}(K)
$$

$K$ is **quiescent** when $\operatorname{pos}(K) = K$.

**The boundary** speaks only at a quiescence. It judges the parked candidates against that fixpoint and admits
**at most one**, the first in the agenda's order $\lhd$ (next section) that passes and was not admitted before.
Saturation is the sequence

$$
K_0' = \operatorname{pos}(K), \qquad
K_{i+1}' = \operatorname{pos}\bigl(K_i' \cup \psi_{\kappa_i}\theta_i\bigr), \qquad
\kappa_i = \min_{\lhd} \,\lbrace\, \kappa \text{ parked} : \operatorname{passes}(\kappa, K_i') \,\rbrace \setminus \lbrace \kappa_0, \dots, \kappa_{i-1} \rbrace
$$

and its **fixpoint** is the first quiescence at which the boundary admits nothing:

$$
\operatorname{sat}(K) = K_m', \qquad m = \min \lbrace\, i : \text{no parked } \kappa \notin \lbrace \kappa_0, \dots, \kappa_{i-1} \rbrace \text{ passes in } K_i' \,\rbrace
$$

```dot
digraph two_phases {
  rankdir=LR;
  node [fontname="Inter", shape=box, style=rounded];
  K [label="K"];
  q0 [label="K′₀ = pos(K)\nquiescent"];
  q1 [label="K′₁ = pos(K′₀ ∪ Aκ₀)\nquiescent"];
  qm [label="K′ₘ = sat(K)\nquiescent, nothing passes", penwidth=2];
  K -> q0 [label="closure:\nevery positive candidate"];
  q0 -> q1 [label="boundary admits κ₀,\nthen closure", style=dashed];
  q1 -> qm [label="… one admission a round …", style=dashed];
}
```

A solid arrow is the closure, a least fixpoint in which no absence is read. A dashed arrow is one round of the
boundary: one candidate admitted, against exactly the knowledge base its guards were judged in, and the closure
resumed.

**One admission a round is a soundness requirement.** Let $P$ hold the two rules
$t \land \operatorname{absent}(q) \implies p$ and $t \land \operatorname{absent}(p) \implies q$, and $K = \lbrace
t \rbrace$. Both candidates pass in $K$. A boundary that admitted every passing candidate would reach
$\lbrace t, p, q \rbrace$, in which neither guard holds, a model of $P$ under no reading. Admitting one and judging
the other again in the knowledge base the first made reaches $\lbrace t, p \rbrace$ or $\lbrace t, q \rbrace$:

$$
\lbrace t \rbrace \;\xrightarrow{\ \kappa_0 = p\ }\; \lbrace t, p \rbrace,
\qquad \lbrace t, p \rbrace \not\models \operatorname{absent}(p)
$$

**A parked candidate's fate.** A guard with no absence nested in it, once failed, fails in every larger
knowledge base, so its candidate can never pass and is dropped. A guard with a nested absence, a universal's, can
turn from failing to passing as the knowledge base grows; its candidate stays parked and is judged again at a
later quiescence ([absence](03-absent-and-naf.md)).

## The agenda: the order rules fire in

Every saturation, root's, each node's and that of any reading of a fixpoint, derives in one order, the
**agenda**. It is a total order on the candidates that are enabled and not yet fired, defined by two keys.

**Leads to $\bot$.** A candidate leads to a contradiction when its rule's conclusion can make one:

$$
\operatorname{lead}(\kappa, K) \iff \bot \in \psi_\rho \;\lor\; \exists f.\; \lnot f \in \psi_\rho \;\lor\;
\exists\, r(\bar t) \in \psi_\rho.\; \bigl( \text{some rule of } P \text{ concludes } \lnot r(\cdot) \;\lor\; \exists \bar v.\; \lnot r(\bar v) \in K \bigr)
$$

**Arrival.** $\operatorname{arr}(\kappa) \in \mathbb{N}$ is the candidate's place in the sequence in which
candidates become enabled; candidates that become enabled together are taken in the written order of their rules.

$$
\kappa \lhd \kappa' \iff
\bigl( \lnot\operatorname{lead}(\kappa, K),\ \operatorname{arr}(\kappa) \bigr) <_{\text{lex}}
\bigl( \lnot\operatorname{lead}(\kappa', K),\ \operatorname{arr}(\kappa') \bigr)
$$

with *false* before *true*: first what leads to $\bot$, then the rest, and within each half first in, first out.
The $j$-th firing of a saturation is the $\lhd$-least candidate enabled after the first $j - 1$; the boundary
takes its parked candidates in the same order; a saturation that continues another keeps it
([Q-M1f.313](../../../decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)).

**Proposition (the order decides no fixpoint where $T_P$ is monotone).** If no rule of $P$ reads an absence,
every order of firing that leaves no enabled candidate unfired reaches $\operatorname{sat}(K)$.

*Argument.* Let $K^{\ast}$ be the set an exhaustive order ends on. Each firing adds a subset of
$T_P$ of what is held, so by induction along the order $K^{\ast} \subseteq \operatorname{sat}(K)$. No candidate
is left enabled and unfired, so $T_P(K^{\ast}) = K^{\ast}$, and $\operatorname{sat}(K)$ is the least such set
containing $K$: $\operatorname{sat}(K) \subseteq K^{\ast}$. $\square$

**Corollary (the order activators are written in decides nothing).** Let no rule of $P$ read an absence. Then
$\operatorname{sat}(K_0)$ does not depend on the order the facts of $K_0$ are written in, and writing one more
activator only adds facts to it.

*Argument.* The firings of $\rho$ in $K$ are the triples $(\rho, a, \theta)$ with
$a \in \operatorname{Act}_K(\rho)$ and $K \models \varphi_\rho[a]\theta$. Both conditions are conditions on the
set $K$, not on the order its facts arrived in, and both are preserved when $K$ grows, so by the proposition the
fixpoint is one set whatever the order. A new activator adds triples and, since every argument of an activator
is part of a firing's identity ([firing a rule](01-firing.md#firing-a-rule)), is equal to none that was there;
$\operatorname{sat}$ is monotone, so the fixpoint of a larger set of facts is larger. $\square$

**The example where a narrower identity would show.** One rule with two parameters, $r$ in a relation's place
and $f$ any object:

$$
\operatorname{note}[r, f]:\qquad r(a, b) \;\land\; \operatorname{holds}(f) \;\implies\; \operatorname{noted}(a, f)
$$

```dot
digraph rule_note_lhs_rhs {
  rankdir=TB;
  label="note"; labelloc=t;
  subgraph cluster_lhs { label="match";
    "?a_L" [label="?a", shape=diamond];
    "?b_L" [label="?b", shape=diamond];
    "?f_L" [label="?f", shape=diamond];
    "?a_L" -> "?b_L" [label="?R", color="#1f77b4", fontcolor="#1f77b4"];
    "h1_holds_L" [shape=octagon, label="holds", color="#bcbd22", fontcolor="#bcbd22"];
    "h1_holds_L" -> "?f_L" [label="1", color="#bcbd22"];
  }
  subgraph cluster_rhs { label="assert";
    "?a_R" [label="?a", shape=diamond];
    "?f_R" [label="?f", shape=diamond];
    "?a_R" -> "?f_R" [label="noted", color="#8c564b", fontcolor="#8c564b", style=dashed];
  }
}
```

The upper panel is the match and the lower the conclusion; a diamond is a variable, and the arrow labelled by
the variable in a relation's place stands for a fact of whichever relation the activator names. The program
states four facts and two activators:

$$
\operatorname{edge}(\mathsf{1}, \mathsf{2}),\ \ \operatorname{edge}(\mathsf{2}, \mathsf{3}),\ \ \operatorname{holds}(\mathsf{1}),\ \ \operatorname{holds}(\mathsf{3}),
\qquad
\operatorname{note}(\operatorname{edge}, \mathsf{1}),\ \ \operatorname{note}(\operatorname{edge}, \operatorname{src}(\mathsf{Y}))
$$

The first activator binds $f$ to the name $\mathsf{1}$, which holds, and derives
$\operatorname{noted}(\mathsf{1}, \mathsf{1})$ and $\operatorname{noted}(\mathsf{2}, \mathsf{1})$, one for each
edge. The second binds $f$ to the fact $\operatorname{src}(\mathsf{Y})$, of which nothing says that it holds, so
it derives nothing, and it removes nothing the first derives. In either order of the two activators the fixpoint
is the same:

```dot
digraph kb {
  rankdir=BT;
  "1" -> "2" [label="edge", color="#8c564b", fontcolor="#8c564b", style=solid];
  "2" -> "3" [label="edge", color="#8c564b", fontcolor="#8c564b", style=solid];
  "1" -> "1" [label="holds", color="#bcbd22", fontcolor="#bcbd22", style=solid];
  "3" -> "3" [label="holds", color="#bcbd22", fontcolor="#bcbd22", style=solid];
  f_45dd2ba665 [shape=octagon, label="(written)\\nby #written", color="#d62728", fontcolor="#d62728", style=dashed];
  f_45dd2ba665 -> "(src Y)" [label="#1", color="#d62728", style=dashed];
  "1" -> "1" [label="noted by note", color="#8c564b", fontcolor="#8c564b", style=dashed];
  "2" -> "1" [label="noted by note", color="#8c564b", fontcolor="#8c564b", style=dashed];
}
```

The two dashed arrows labelled with the rule are the two conclusions. The octagon is the fact that reading the
program states of the nested fact the second activator writes
([the knowledge base](../01-static-ir/01-knowledge-base.md#the-knowledge-base-of-a-program)); the activators and
the declarations' facts are left out of the drawing.

So on such a program every question and every claim answers alike under the agenda and under the written order
alone. Taking first what can make a contradiction finds a dead node's contradiction sooner, and that is all the
order is for. What it does decide is *when* a fact is derived:

- where a dead node's derivation stops;
- how much of a derivation a node narrates;
- which of a fact's equally valid derivations its record keeps first, which a core is explained from. A core is
  an explanation, and it may move with the order
  ([Q-M1f.6](../../../decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)).

Where rules read absences, the order of admission at the boundary is part of what $\operatorname{sat}$ is, and
[absence](03-absent-and-naf.md) says when it can decide what is derived. Of a program that may not be monotone no
answer is claimed under any order ([monotonicity](08-monotonicity.md)).

## A node resumes root's saturation

A node's saturation is root's, continued. It starts from root's fixpoint, with what root has fired and what it
has parked, adds the commitment's facts, and saturates to quiescence; it does not derive root's fixpoint again,
and every candidate it inherits parked is judged again in its own world before it can be admitted.

**Proposition (continuing is saturating afresh).** Let $\mathrm{root}$ be quiescent with fired set $\Xi$ and
parked set $Q$. The saturation continued from $(\mathrm{root}, \Xi, Q)$ with $C$ added reaches the fixpoint a
fresh saturation of the set $\mathrm{root} \cup C$ reaches
([Q-M1a.18](../../../decisions/engine/Q-M1a.18-a-node-resumes-roots-saturation.md)).

*Argument.* A fresh saturation of $\mathrm{root} \cup C$ meets three kinds of candidate. One whose match
lies in $\mathrm{root}$ and is positive was fired at root, is in $\Xi$, and adds nothing a second time. One
whose match lies in $\mathrm{root}$ and is parked is in $Q$, and the continued saturation judges it again in
the node's own world. One whose match reads a fact of $C$ or a fact derived from one is new to both, and both
find it, since the continued saturation starts from the facts the node has and root lacked. $\square$

```dot
digraph resume {
  rankdir=LR;
  node [fontname="Inter", shape=box, style=rounded];
  K0 [label="K₀"];
  root [label="root = sat(K₀)\nfired F, parked Q"];
  rc [label="root ∪ C"];
  st [label="st(C)", penwidth=2];
  K0 -> root [label="saturate"];
  root -> rc [label="add C"];
  rc -> st [label="continue from (F, Q)"];
  rc -> st [label="saturate afresh", style=dashed];
}
```

The two arrows into $\operatorname{st}(C)$ end on one set of facts, so every answer and every count is the same
by either. What may differ is how many firings are counted, and which derivation of a fact derived several ways
its record keeps first. Saturating the program's own facts with the commitment, $\operatorname{sat}(K_0 \cup C)$,
is a third construction and is not claimed equal to these where rules read absences
([monotonicity](08-monotonicity.md)).

**Proposition (a dead node may stop at its first contradiction).** Let $K_0'' \subseteq K_1'' \subseteq \cdots$
be the knowledge bases a saturation passes through. Then

$$
\operatorname{dead}(K_j'') \implies \forall j' \geq j.\; \operatorname{dead}(K_{j'}'')
\implies \operatorname{dead}(\operatorname{sat}(K))
$$

since a world never loses a fact. Whether a node is dead is therefore decided at the first $j$ at which a
contradiction is held, and every firing after it adds nothing to that. On a monotone program a computation of
$R$ that needs only *dead* of a node may stop there, and the answer is the one the node run to quiescence gives.
The definition of a state never stops: $\operatorname{st}(C)$ is the fixpoint. Root, a program that may not be
monotone, and any reading that needs the whole state run to quiescence, and a core is always taken over the
whole fixpoint ([finding R](07-finding-r.md)).

## What saturation derives before anything is supposed

Propagation decides everything it can before anything is supposed: $H$ is proposed at root's fixpoint, never
while a saturation runs, and a node is judged as its rules leave it
([Q19](../../../decisions/hypotheses/Q19-hypotheses-are-proposed-at-a-fixpoint-after.md)). So what a solver by
hand concludes without guessing, saturation has to conclude too, and that includes the negatives.

**Negative completion.** A stored negation can be derived from positive evidence and a declared property, with
no search behind it. For a relation $r$ from a domain $X$ to a range $Y$, each a set of names under one label:

$$
\begin{aligned}
\textbf{functional:}\quad & r(a, b) \;\land\; b' \in Y \setminus \lbrace b \rbrace \;\implies\; \lnot r(a, b') \\
\textbf{injective:}\quad & r(a, b) \;\land\; a' \in X \setminus \lbrace a \rbrace \;\implies\; \lnot r(a', b) \\
\textbf{domain elimination:}\quad & \bigl( \forall b' \in Y \setminus \lbrace b \rbrace.\;\; \lnot r(a, b') \bigr) \;\implies\; r(a, b) \\
\textbf{range elimination:}\quad & \bigl( \forall a' \in X \setminus \lbrace a \rbrace.\;\; \lnot r(a', b) \bigr) \;\implies\; r(a, b)
\end{aligned}
$$

and a negative carries across a declared equivalence $e$: $\lnot r(a, b) \land e(b, b') \implies \lnot r(a, b')$.
The standard library states these as rules over a relation's declared properties
([properties and operators](../../user/03-syntax-and-semantics/07-properties-and-operators.md)), so a puzzle
writes one property per constraint and saturation fans it out.

```dot
digraph negative_completion {
  rankdir=LR;
  node [fontname="Inter"];
  subgraph cluster_before { label="one positive fact, r functional";
    a0 [label="a", shape=oval]; b0 [label="b", shape=oval]; c0 [label="b′", shape=oval]; d0 [label="b″", shape=oval];
    a0 -> b0 [label="r"];
    a0 -> c0 [style=invis]; a0 -> d0 [style=invis];
  }
  subgraph cluster_after { label="what saturation adds";
    a1 [label="a", shape=oval]; b1 [label="b", shape=oval]; c1 [label="b′", shape=oval]; d1 [label="b″", shape=oval];
    a1 -> b1 [label="r"];
    a1 -> c1 [label="¬ r", color="#d62728", fontcolor="#d62728", style=dashed];
    a1 -> d1 [label="¬ r", color="#d62728", fontcolor="#d62728", style=dashed];
  }
}
```

A solid arrow is a fact, a red dashed arrow a stored negation saturation derived. Elimination is the same
picture read backwards: when every arrow out of $a$ but one is red, the last one is drawn solid. On the Zebra
puzzle this closes at depth zero every step of the human walkthrough: each *therefore* is reached from the clues
and the ontology with $C = \emptyset$.

**The same inference over one generic relation.** A puzzle may instead link every attribute through one
equivalence relation $e$, which is a bijection only between each pair of labels. The same inference is stated for
that encoding, from a property scoped by a family of labels, rule for rule: placement through an index,
exclusivity within a label, the negative across a link, elimination for a value and for a slot, the
unique-neighbour spatial rules. Two things are worth knowing about it. Elimination for a value and fill for a
slot are both needed and are not each other's mirror: the conclusions are the same edge, but they read different
negatives and fire at different times. And a symmetric relation needs its negative mirrored,
$\lnot e(a, b) \implies \lnot e(b, a)$: a negative derived in one argument order reaches no rule that reads the
other. Both encodings of the Zebra puzzle reach the same model by the same walkthrough.

**The closed-world reads are on what saturation determines.** Both encodings read under an absence only the
position structure, the neighbour, pruning and endpoint guards, which the clues fix before any hypothesis; they
never read under one an attribute the search decides. That is the bet a closed-world read makes, and it is
exactly the line the language draws: a refutation may not rest on an absence over a relation a commitment can
still extend ([absence](03-absent-and-naf.md)).

## The alive-set invariant

The candidate set $H$ is taken once, at root's fixpoint, and every computation of $R$ reads subsets of it. That
is sound only if nothing a node derives names what root's fixpoint and the program did not: a name that first
appeared inside a node would be an atom no candidate is about, in a part of the lattice $H$ was not built for,
and a model naming it would be missing from $R$.

$\operatorname{names}(X)$ is the names under an argument place of a fact of $X$ at any depth, a nested fact's
arguments included ([the knowledge base](../01-static-ir/01-knowledge-base.md#nesting)); write
$\operatorname{rels}(X)$ for the relations of the facts of $X$. $\operatorname{names}(P)$ is every constant of
the program's facts and of its rules, match and conclusion alike
([the rules](../01-static-ir/02-rewriting-rules.md#the-program-and-its-names),
[Q-M1e2.9](../../../decisions/language/Q-M1e2.9-a-programs-names-are-the-constants-its-facts.md)). The
invariant, *no rule asserts a name the program lacks*, is

$$
\forall C \subseteq H.\quad \operatorname{names}\bigl(\operatorname{st}(C)\bigr) \;\subseteq\; \operatorname{names}(\mathrm{root}) \cup \operatorname{names}(P)
$$

and it has three clauses:

1. **No new atoms.** $\operatorname{names}(\operatorname{fire}(K, \xi)) \subseteq \operatorname{names}(K) \cup
   \operatorname{names}(P)$: a rewriting brings in no name
   ([the rules](../01-static-ir/02-rewriting-rules.md#the-program-and-its-names)). A variable carries only an
   object a match already found, or a name of the program, and nothing else but a constant can bring a name in,
   and every constant is the program's
   ([Q-M1e.21](../../../decisions/hypotheses/Q-M1e.21-a-rule-brings-in-no-name.md)). By induction along a
   saturation, $\operatorname{names}(\operatorname{sat}(K)) \subseteq \operatorname{names}(K) \cup
   \operatorname{names}(P)$. A constant only a query writes is not a name of the program: adding a query never
   changes $R$.
2. **No new relations.** $\operatorname{rels}(\operatorname{st}(C)) \subseteq \operatorname{rels}(P)$, the
   relations fixed when the program is read: the declared ones, the heads of the program's facts, and every
   constant head a rule can conclude. Nothing adds a relation while a saturation runs, and nothing adds a rule
   or a declaration: a conclusion is literals, so along every saturation $P$ and $\operatorname{names}(P)$ are
   constant and a knowledge base grows by facts alone.
3. **Hypotheses connect names.** $h \in H \implies \operatorname{names}(h) \subseteq \operatorname{names}(P)$: a
   candidate's arguments are names of the program or facts built of them. A parameter written in a relation's
   place and bound to an object that names no relation, a nested fact, is refused where it is written or where a
   rule could derive it.

Three readings of the invariant, each stronger than the one before:

| | claim |
|---|---|
| **static** | $\operatorname{names}(P) \setminus X \subseteq \operatorname{guessable}(P)$: every name of the program, less the excluded kinds $X$ below, is an atom candidates are proposed about when nothing narrower is stated |
| **at the fixpoint** | $\operatorname{names}(\operatorname{sat}(K_0)) \subseteq \operatorname{names}(K_0) \cup \operatorname{names}(P)$: nothing a saturation derives names what the program as read, its facts and its rules' constants, did not |
| **in the search** | $\forall C \subseteq H.\; \operatorname{names}(\operatorname{st}(C)) \subseteq \operatorname{names}(\mathrm{root}) \cup \operatorname{names}(P)$: nothing a node reaches names what root's fixpoint and the rules' constants did not |

The third is the one $R$'s soundness rests on, and the first is what makes it hold for a rule's constant: a
constant a rule writes, and that would be reached only inside a commitment, is guessable from root on. The static
reading answers for every saturation the program could have, not only for the one that happened. Together they
make the lattice finite: $H$ is a finite set of facts over $\operatorname{names}(P)$, and
$\lvert 2^H \rvert = 2^{\lvert H \rvert}$.

**The guessable atoms are not the list of names.** Four kinds of name, the set $X$, are not guessable, each left out by
name: the labels a relation's declaration writes for its argument places; the reserved words of the pattern
language, negation, contradiction, conjunction, disjunction, absence, equality and inequality; every name the
program calls a relation or a rule; and the two reserved markers that close a relation and make one symmetric,
which a program may write as an argument and which stay names of the program
([Q-M1f.306](../../../decisions/hypotheses/Q-M1f.306-the-blind-enumerators-objects-are-the-programs.md);
[reserved names](../../user/04-reference/03-reserved-names.md) lists them as written).
