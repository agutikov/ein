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

The symbols below mean one thing on every page. A page that needs a symbol of its own defines it where it is used.

**Letters.** A constant of a program is set upright, $\mathsf{Norwegian}$, and a relation of a program as an
operator, $\operatorname{lives-in}$. Italic letters range over what the table says: $a, b, c$ over names,
$u, v, w$ over values, $f, g, h$ over facts, $r, s, t$ over relation names, $x, y, z$ over variables.

| symbol | what it is | where it is defined |
|---|---|---|
| $\Sigma$, $\Sigma^{+}$ | the name characters, and the names: the non-empty words over them | [names and atoms](01-static-ir/01-names-and-atoms.md) |
| $N$ | the names of the program, a finite subset of $\Sigma^{+}$ | [names and atoms](01-static-ir/01-names-and-atoms.md) |
| $V$ | the values: names, and facts over values | [names and atoms](01-static-ir/01-names-and-atoms.md) |
| $F$ | the facts, $F = V \setminus \Sigma^{+}$ | [names and atoms](01-static-ir/01-names-and-atoms.md) |
| $r(v_1, \dots, v_n)$ | the fact of relation $r$ over the arguments $v_1, \dots, v_n$ | [names and atoms](01-static-ir/01-names-and-atoms.md) |
| $\lnot f$ | the stored negation of the fact $f$, itself a fact | [relations and facts](01-static-ir/02-relations-and-facts.md) |
| $\bot$ | the fact of falsity, of no argument | [relations and facts](01-static-ir/02-relations-and-facts.md) |
| $S$ | a state: a finite set of facts, $S \subseteq F$ | [relations and facts](01-static-ir/02-relations-and-facts.md) |
| $K = (P, S, J)$ | a knowledge base: a program, a state, and the state's provenance | [relations and facts](01-static-ir/02-relations-and-facts.md) |
| $J(f)$ | the derivations recorded for the fact $f$ | [relations and facts](01-static-ir/02-relations-and-facts.md) |
| $\preceq$ | the total order of the values | [values and their order](01-static-ir/03-values-and-order.md) |
| $P$ | the program's rule set | [rules as statements](01-static-ir/04-rules-as-statements.md) |
| $\rho = (\bar{p}, \varphi, \psi)$ | a rule: its parameters, its pattern, its conclusion | [rules as statements](01-static-ir/04-rules-as-statements.md) |
| $\theta$ | a substitution of values for variables | [rules as statements](01-static-ir/04-rules-as-statements.md) |
| $S \models \varphi\theta$ | the pattern $\varphi$ holds in $S$ under $\theta$ | [rules as statements](01-static-ir/04-rules-as-statements.md) |
| $\operatorname{sat}$ | saturation: a state taken to its fixpoint under $P$ | [saturation](02-kb-evolution/02-saturation.md) |
| $\operatorname{root}$ | the program's own facts, saturated | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $H$, $n = \lvert H \rvert$ | the candidate set, and its size | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $C \subseteq H$ | a commitment, a node of the lattice | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\operatorname{sat}(\operatorname{root} + C)$ | the state of the node $C$ | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $\operatorname{cl}(C)$ | the closure of $C$: the candidates its state holds | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $R$ | the region: the commitments whose state is alive and discharged | [solution semantics](02-kb-evolution/06-solution-semantics.md) |
| $G_P$, $\sigma$ | the dependency graph of the rule set, and the stratum of a relation or a rule | [analysis of rules](03-rule-evolution/02-analysis-of-rules.md) |

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
