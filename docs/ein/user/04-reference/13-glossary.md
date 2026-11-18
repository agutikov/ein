# Glossary

The terms Ein uses with a meaning of its own, each with the page that defines it. Concepts from outside Ein, an LLM, a
CSP, an SMT solver, are catalogued in [`docs/lib/`](../../../lib/README.md).

## The language

### Object
A node of the knowledge-base graph, and nothing else. Every object is an [atom](#atom) or a [fact](#fact). An
object's identity is its name or its list of arrows: one atom for a name, one fact for a list. See
[`../03-syntax-and-semantics/01-objects.md`](../03-syntax-and-semantics/01-objects.md), and
[the knowledge base](../../reasoning/01-static-ir/01-knowledge-base.md) for the graph defined.

### Atom
A labelled node with no outgoing arrow: `Norwegian`, `House-1`, `Red`, `co-located`, `T`. Its label is its
[name](#name), and two occurrences of one name are one atom. What an atom is called, a relation, an argument, a
label, says which arrow of a fact points at it, and nothing about the atom. See
[`../03-syntax-and-semantics/01-objects.md`](../03-syntax-and-semantics/01-objects.md).

### Name
The label of an [atom](#atom), and the word a program writes for it. **A run of digits is a name like any other**:
`1`, `01` and `-7` are three names, and `(p 1)` and `(p 01)` two facts; Ein has no numbers
([Q-M1e2.24](../../../decisions/language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)). **A string is no name**:
it is natural-language text, the value of `:why`, `:source` or `:goal-text`, and `"Red"` where a name goes is a parse
error ([Q-M1e2.23](../../../decisions/language/Q-M1e2.23-a-string-is-natural-language-text-never.md)). A name
is ASCII; a string is any Unicode. See [`02-syntax.md` § 1](02-syntax.md#1-lexical-rules).

### Fact
A node with outgoing arrows, in order: the first points at its [relation](#relation), the others at its
**arguments**, each an [object](#object), an atom or a fact
([Q40](../../../decisions/language/Q40-a-fact-may-hold-a-fact-as-an-argument.md)). A fact has no name and is
identified by that list. It is the unit of statement in a knowledge base, which **holds** some facts and only
points at others, the arguments of the facts it holds. `ein` draws a fact as an octagon with its relation's name in
it, or, for a fact of two arguments in the compact view, as one arrow with the relation's name on it. See
[`../03-syntax-and-semantics/03-facts.md`](../03-syntax-and-semantics/03-facts.md).

### Held, global context, context
A fact is **held** when the knowledge base holds it, and the facts held are the **global context**: where a rule's
premise, a query and a contradiction look. A fact is **in a context** when a held fact points at it, as an argument
or deeper: `(knows Carol Dan)` is in the context of `(claims Carol (knows Carol Dan))`, which speaks about it. A
fact may be held, in a context, or both, and a context gives it no truth: only `(not P)`, which is a context of
`P`, is read by the engine as saying something of `P`
([Q-M1f.394](../../../decisions/language/Q-M1f.394-a-fact-is-in-the-global-context-in-a-facts.md)). See
[`../03-syntax-and-semantics/03-facts.md` § A fact inside a fact](../03-syntax-and-semantics/03-facts.md#a-fact-inside-a-fact).

### Relation
The [atom](#atom) under the first arrow of a fact: one object, however many facts point at it. Three things go by
its name:
1. the **relation**, that atom, `co-located`. A fact that has it as an argument, a property `(symmetric co-located)`
   or a declaration, speaks of the relation as one object;
2. its **extent**, every fact whose first arrow points at it, held or not. What a knowledge base or a state holds
   of it is the **held extent**;
3. **a fact of** the relation, one member of the extent: `(co-located Norwegian Red)`.

See [`../03-syntax-and-semantics/02-relations.md`](../03-syntax-and-semantics/02-relations.md).

### Declared relation
The atom a declaration names: `co-located` in `(relation co-located Attribute Attribute :why "…")`. A declaration
is itself a fact, of the relation `relation`: its second arrow points at the declared relation and its later arrows
at the [labels](#label-signature). *Relation* and *declared relation* are two things an atom can be: a relation
nothing declares is a relation all the same, and a declared relation of which there is no fact is under no first
arrow. See [`../03-syntax-and-semantics/02-relations.md`](../03-syntax-and-semantics/02-relations.md).

### Label, signature
The atoms a declaration lists after the declared relation, one for each argument: `Nationality` and `House` in
`(relation nation-loc Nationality House :why "…")`. Together they are the relation's **signature**. Their number is
the number of arguments every form of the relation has, which the loader holds; a label itself is an ordinary atom,
no argument is checked against it, and a rule reads it through the stored declaration, `(relation ?R ?A ?B)`. The
engine reads a signature only for what may be guessed: whether it is empty, how long it is, and which atoms it
lists, about which the blind enumerator proposes nothing. See
[`02-syntax.md` § What the signature means](02-syntax.md#what-the-signature-means--labels-for-rules-a-shape-for-the-engine).

### Type
**Ein has no types** ([Q23](../../../decisions/language/Q23-a-premise-says-what-a-variable-ranges-over.md),
[Q-M1f.372](../../../decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md)). The
word is a program's, for an atom it groups other atoms under, `Nationality`, `House`, `Color`: the atom its
membership facts point at, `(is-a Norwegian Nationality)`, or a [label](#label-signature) of a declaration. Such an
atom is a node like any other and takes part in facts as any atom does, and it means what the rules that read those
facts make it mean. `ein render kb` draws the atom an `is-a` fact points at as a box, a convention of the drawing
keyed on that relation's name, and `ein` prints the word in a few places, `--no-types` among them, for such an
atom or for a label. See
[`../03-syntax-and-semantics/04-ontology.md`](../03-syntax-and-semantics/04-ontology.md).

### Value
The [object](#object) a variable of a rule is bound to: in a match that binds `?a` to `Alice`, `Alice` is the value
of `?a`; a variable has one value in each match, and over the matches of a pattern one or many. An object is a value only while a variable
is bound to it; a node of the graph is an object. Where `ein` prints *value*, as in the refusal *where a value
goes*, it means an object in an argument's place. The word also keeps its ordinary sense for the value of a keyword
and of a config field.

### Rule
A rewriting rule: a `:match`, what it reads, whose guards, `(neq ?a ?b)`, are premises in it; an `:assert`, what it
derives; a `:why`, why; and no other keyword
([Q4](../../../decisions/language/Q4-a-rule-is-a-pattern-a-conclusion-and-a-reason.md)). See
[`../03-syntax-and-semantics/05-rules.md`](../03-syntax-and-semantics/05-rules.md).

### Hrule
A hypothesis rule: the shape of a rule, and what it concludes is proposed as a hypothesis rather than derived. See
[`../03-syntax-and-semantics/09-hypotheses.md`](../03-syntax-and-semantics/09-hypotheses.md).

### Activator, property fact
A fact whose relation is a rule's name; its arguments bind the rule's parameters. `(symmetric co-located)` activates the
rule `symmetric` on the relation `co-located`. See [`02-syntax.md` § Rules](02-syntax.md#rules).

### `eq`, `neq`
The relations of **equality** and **non-equality**: `(eq x x)` for every object and `(neq x y)` for every two different
objects, **computed, never stored**, so no fact, rule or commitment changes them
([Q-M1e2.26](../../../decisions/language/Q-M1e2.26-eq-and-neq-are-the-relations-of-equality-and-non.md)). In a goal
or a premise each is a guard, which binds a variable nothing else binds, over the program's names where it must; a
conclusion of either is checked, a false one `(false)`; `(not (eq x y))` is `(neq x y)`. See
[`02-syntax.md` § Premise order around a guard](02-syntax.md#premise-order-around-a-guard).

### Absent, negation as failure
`(absent P)` in a `:match`: the question *does this state hold no fact matching P?*, asked once the positive rules
have reached their fixpoint, under the bindings of the premises written before it. Never a stored fact. On a monotone
program its answer does not depend on the order rules fire in. See
[`02-syntax.md` § NAF evaluation timing](02-syntax.md#naf-evaluation-timing--the-closureworld-boundary) and
[`03-absent-and-naf.md`](../../reasoning/02-kb-evolution/03-absent-and-naf.md).

### Stratification
A rule set is stratified when no `(absent …)` reads facts of a relation that depend, through the rules, on the guarded
rule's own conclusion: no recursion through negation. Ein checks it and says so, as a diagnostic and not a refusal: a cycle
through an `absent` is named by `ein solve`'s `strata` row and `ein saturate --dump`'s `;; Strata` line. On a rule set
that is not stratified, a rule that concludes `p` where `q` is absent beside one that concludes `q` where `p` is
absent, the engine reaches one fixpoint, by the order the
boundary admits firings in, and the `strata` row says it may depend on that order. See [`08-read-out.md`](08-read-out.md).

### Macro
A pattern alias, `(macro N (?p…) BODY :why "…")`: a call in a rule's clause or a query's `:goal` is replaced by its
body at load. `forall` and `unknown` are the standard library's. See
[`../03-syntax-and-semantics/11-macros.md`](../03-syntax-and-semantics/11-macros.md).

### Hyperedge, Levi-bipartite
A hyperedge is an edge with any number of participants, and a fact is one, over the objects under its arrows. The
graph that says the same as a hypergraph is its **Levi graph**, after Friedrich Wilhelm Levi: a node for each
participant, a node for each hyperedge, and an arrow from a hyperedge's node to each participant. The knowledge
base is that graph with one change, that a fact may point at a fact. `ein`'s `--levi` drawing is of it: one octagon
per fact and an edge to each argument, numbered from the first argument, with the relation's name written in the
octagon and no node drawn for the relation. See
[`12-rendering.md` § Hyperedge encoding](12-rendering.md#hyperedge-encoding--levi-bipartite).

### Provenance
Where a fact came from: **given** (`:source`), **derived** (`:rule`, `:using`), or **background** (neither), and a
fact a commitment adds is a **hypothesis**. Provenance is kept per derivation: a fact holds every derivation recorded
for it, each its premises, so the proof structure is an AND/OR graph
([Q18](../../../decisions/engine/Q18-provenance-is-per-fact-an-or-of-derivations-over.md)). A fact's origin is its
provenance and nothing else. See [`../03-syntax-and-semantics/03-facts.md`](../03-syntax-and-semantics/03-facts.md).

### Open-world
A relation nothing declares is allowed: a fact or a pattern over it loads, and the relation is one of the program's.
A pattern over a misspelled relation therefore loads and matches nothing. See
[`02-syntax.md` § What a pattern is made of](02-syntax.md#what-a-pattern-is-made-of).

## The answer

Defined in full in [`../03-syntax-and-semantics/13-solutions-and-models.md`](../03-syntax-and-semantics/13-solutions-and-models.md).
The answer to a program is a **region** of its hypothesis lattice
([Q-M1e2.7](../../../decisions/answer/Q-M1e2.7-the-answer-is-a-region-r-of-the-hypothesis.md)), and the entries
below are written for a **monotone** program.

### Root
The program's facts saturated by its rules, nothing hypothesised: the lattice's node $\emptyset$, which the read-out
writes `∅`.

### Hypothesis, candidate
A fact the generator proposes at root. Which ones exist is the **generation ladder**'s answer: the program's own
`(hrule …)`, else the facts that would discharge what root owes and what a rule can bring a commitment to owe, else the
blind enumerator. `:hypothesis-relations`, `:no-hypothesis` and `(__closed__ R)` decide which relations are eligible,
so a fact of an excluded relation is no hypothesis at all. The blind enumerator proposes about the **guessable
atoms**: the program's names, less a signature's labels, the language's own words, every relation and rule, and the
two dunders
([Q-M1f.306](../../../decisions/hypotheses/Q-M1f.306-the-guessable-atoms-are-the-programs-names-never.md)); where
`ein` prints *object* of the enumerator, it means one of them. See
[`../03-syntax-and-semantics/09-hypotheses.md`](../03-syntax-and-semantics/09-hypotheses.md).

### Universe, H
The candidate list, $n = |H|$: **the space every definition quantifies over**. It must hold every fact a model needs; a
shortfall is a program error.

### Commitment
A subset of the universe: a node of the lattice. Without merging, the lattice is the $n$-cube, and its layer $k$ holds
the commitments of $k$ facts.

### State
$\operatorname{sat}(\text{root} \cup C)$ for a commitment $C$: root's facts and $C$'s, saturated. **Dead** when it holds
`(false)`, **alive** otherwise; **owing $n$** with $n$ undischarged obligation instances; **discharged** when it owes
nothing.

### Closure, state class
$\operatorname{cl}(C)$, the candidates $C$'s state holds, so $C \subseteq \operatorname{cl}(C)$. The commitments that reach
one state form its **class**: the closed set on top, its **minimal generators** at the bottom, the least that must be
guessed.

### Region R, solution
**R is the commitments whose state is alive and discharged**: the answer to a program. A **solution** is a node of R.
In general R is its set of nodes; on a monotone program it has no holes, and its **minimal** and **maximal lines**
describe it. $\emptyset$ is a solution exactly when root is alive and discharged.

### k_s, k_c
$|R|$ counted in states and in commitments.

### Line, hole, sub-cube, component
The **minimal** and **maximal line** are R's minimal and maximal nodes, set by obligations and by restrictions. A
**hole** is a node outside R between two members; a monotone program has none. A **sub-cube** $[a, b]$ is every
commitment $C$ with $a \subseteq C \subseteq b$, of dimension $|b| - |a|$. A **component** is a connected part of R under one-fact edges: a
set of models, never a model.

### N
The minimal dead sets: the dead commitments every one of whose proper subsets is alive. `ein solve` prints each with
its core. See [`08-read-out.md`](08-read-out.md).

### Monotone
No read that can take a conclusion away, or add a debt, as a commitment grows. Then dead and discharged are
upward-closed, and R is convex. A static check over the program, its verdict the read-out's `lattice` row.

### Exhausted
That the part of R a run asked is all there: the run did not stop at a budget or a count. See
[`08-read-out.md`](08-read-out.md).

### Model
The **positive** part of a solution's state **less the positive part of the program as loaded**: no `(not …)`,
nothing the file stated and nothing the loader wrote, so it is what the puzzle did not say and the solution
established; `ein solve -P` prints it
([Q-M1e.6](../../../decisions/answer/Q-M1e.6-a-model-is-what-a-solutions-state-adds.md)). Distinct from a listing's
`(model …)`, the claim form, which holds a state's facts over the candidates' relations, root's included
([Q-M1f.18](../../../decisions/read-out/Q-M1f.18-the-read-outs-model-is-the-claim-form-and-p.md)). A query sees a
model's **projection**, and the models with one projection form a **projection class**.

### Obligation
A rule that asserts the verdict `(open ?R)` while a witness is missing
([Q-M1d.2](../../../decisions/hypotheses/Q-M1d.2-an-obligation-is-a-rule-that-concludes-open-r.md)). What a state
**owes** is its undischarged instances, and a state owing none is **discharged**. Obligations set R's minimal line, as
restrictions, rules that assert `(false)`, set its maximal line. See
[`../03-syntax-and-semantics/08-obligations.md`](../03-syntax-and-semantics/08-obligations.md).

### Core, the smallest contradiction frontier
Why a node is dead: the smallest set of given facts, and the facts its commitment adds, from which, with root's rules,
saturation derives the contradiction, searched over every derivation the program records. **A contradiction is
explained by its facts together**: `(false)`, or a fact `X` and its `(not X)` as one pair. Beside it, `refuted by`
names the rules of the last step of its derivation, the first derivation recorded whose premises it derives
([Q-M1e2.21](../../../decisions/read-out/Q-M1e2.21-refuted-by-names-the-first-recorded-derivation.md)). A core rests
on the activator a node derived or committed
([Q-M1e2.12](../../../decisions/answer/Q-M1e2.12-a-dead-nodes-core-follows-an-activator-only.md)) and on the facts a
`forall` read ([Q-M1e2.15](../../../decisions/answer/Q-M1e2.15-a-derivation-through-a-forall-rests-on-each-fact.md)).

- **A core is an explanation**: it may vary with the order rules are written in, and is not a subset-minimal
  unsatisfiable set ([Q-M1f.6](../../../decisions/answer/Q-M1f.6-a-core-is-an-explanation.md)).
- **The search is budgeted**, and a core cut at its budget says so: every JSON writes `core_status`, `within-budget`
  or `cut-` and the caps it stopped at, and every text surface says a cut core was cut and at which cap.
  `within-budget` says the search was not cut, and no more.
- **A node that is not dead has no core**: JSON writes a dead node's core as a list, `[]` being the empty set, and a
  node that is not dead `null` ([Q-M1e2.27](../../../decisions/read-out/Q-M1e2.27-in-eins-json-is-the-empty-set-and-null-what-does.md)).
  A text surface prints the empty core `∅`.
- **A core is taken only where a surface prints one**: a bare `ein solve` takes root's and the minimal holes', and
  `--dump-answer` and `--trace` each member of N's.
- `:record-alternative-justifications false` records each fact's first derivation only, which may name another core
  and with it other rules `refuted by`, and changes nothing else
  ([`06-config.md`](06-config.md)).

See [`08-read-out.md`](08-read-out.md).

### Config principle
No configuration option, lever or flag changes R; one may change only how fast it is found
([Q-M1e2.30](../../../decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)). A budget may
stop a run, and the report then says it is partial. See [`06-config.md`](06-config.md).

### Saturation
The fixpoint of the rules: apply every rule until no new fact is derived. It runs in two phases, repeated: a
*closure*, in which the rules whose premises are all positive fire until nothing is left to derive, and a *boundary*,
in which each firing waiting on an `(absent …)` is judged against that fixpoint and one is admitted, after which the
closure runs again; saturation ends when a boundary admits nothing. The agenda fires first what leads to `(false)`
([Q-M1f.313](../../../decisions/engine/Q-M1f.313-the-agenda-fires-first-what-leads-to-false.md)).

## Properties of relations

Predicates over a relation, each stated as a fact that activates a rule, `(symmetric co-located)`. The standard
library's are [`../03-syntax-and-semantics/07-properties-and-operators.md`](../03-syntax-and-semantics/07-properties-and-operators.md).

### Symmetric
$R(a,b) \implies R(b,a)$. Example: `co-located`, `next-to`.

### Transitive
$R(a,b) \land R(b,c) \implies R(a,c)$. Example: `co-located`, `ancestor-of`.

### Reflexive
$R(x,x)$ for every $x$ in the relation's domain. `co-located` is reflexive mathematically, and Ein does not derive the
self-edges, which add nothing to a Zebra-class puzzle; `std.typing`'s `(reflexive R)` closes a preorder where one is
wanted.

### Asymmetric
$R(a,b) \implies \lnot R(b,a)$ for $a \neq b$. Example: `is-a` (Norwegian is a Nationality, and Nationality is not a Norwegian),
`right-of`.

### Antisymmetric
$R(a,b) \land R(b,a) \implies a = b$. Weaker than asymmetric: the property of a partial order.

### Irreflexive
$\lnot R(x,x)$ for every $x$. Used with strict orders and an explicit `neq`.

## Theory

### Homoiconic
A language whose program text is itself data of the language. Ein is homoiconic: rules, facts and traces share one
S-expression grammar, so the engine reads its own traces
([Q3](../../../decisions/language/Q3-ein-is-an-s-expression-language.md)).

### DPO, double pushout
A categorical formulation of graph rewriting in which a rule is a span $L \leftarrow K \rightarrow R$. Ein's patterns are
positive and conjunctive and delete nothing, so a rule is the case $K = L$.

### ATMS, ATMS label
An assumption-based truth maintenance system keeps, for every fact, its **label**: the subset-minimal sets of
assumptions, *environments*, it follows from. Ein records *justifications*, a fact's derivations, and computes labels
on demand, from the given facts upward over the AND/OR graph of derivations, under a budget; starting from empty
labels that only grow makes a cyclic graph of justifications safe, so a symmetric or transitive closure, where `(R a
b)` and `(R b a)` justify each other, never grounds a fact in itself. A core is read off them.

### E-graph, equality saturation
An e-graph keeps classes of equal terms with shared sub-terms; equality saturation applies every equality rewrite
without committing to a normal form. Ein has neither: a fact of `=` licenses no substitution
([`02-syntax.md` § 3](02-syntax.md#is-a-fact-head-not-a-unifier)). See
[`docs/lib/06-graphs-rewrite-systems.md`](../../../lib/06-graphs-rewrite-systems.md).
