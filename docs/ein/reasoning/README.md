# Reasoning — what Ein computes, and why it is right

These pages state Ein's reasoning so that a second implementation would be held to it: what a program means once
it is loaded, how its knowledge base grows, what the answer is, how the answer is found and why each way of
finding it gives the same answer. They name no crate, module or type of the implementation that ships; that is
`ein.rs`'s perspective. What a program says and what `ein solve` prints is the user's perspective.

**The question asked of every paragraph** here is *would this still be true of a second implementation?* A
paragraph that is true only of how `ein.rs` does it belongs to `ein.rs`'s pages. A paragraph that a second
implementation must also make true belongs here, whether it is a definition, an ordering a reader can observe, or
an argument that two computations agree.

**What holds a page.** Each page ends with *What holds it*: the claims it makes about what the engine does, each
beside the test of the shipped engine that fails when the claim stops being true. A claim with no test is stated
as a definition or as an argument, never as a fact about the engine. The decisions a page carries out are cited by
link to their files under [`docs/decisions/`](../../decisions/README.md).

## Reading order

Three parts, in the order a program lives through them.

### The program, up to its loaded state — [`static-ir/`](static-ir/)

1. [Names and atoms](static-ir/names-and-atoms.md): what a name is, and identity by name.
2. [Relations and facts](static-ir/relations-and-facts.md): the knowledge base as facts and stored negations.
3. [Values and their order](static-ir/values-and-order.md): the orders a reader can observe, and the identity of
   a state.
4. [Rules as statements](static-ir/rules-as-statements.md): the kinds of rule, a rule as a rewrite, and the four
   levels a knowledge base describes itself in.

### The knowledge base as it grows — [`kb-evolution/`](kb-evolution/)

5. [Firing a rule](kb-evolution/firing.md): what Ein solves, and the operations every implementation performs.
6. [Saturation](kb-evolution/saturation.md): the fixpoint, the agenda's order, and the alive-set invariant.
7. [`absent`: negation as failure](kb-evolution/absent-and-naf.md): what `absent` reads, and in which world.
8. [Hypothesis generation](kb-evolution/hypothesis-generation.md): the ladder that builds the candidate set H.
9. [The hypothesis space and its structure](kb-evolution/hypothesis-space.md): H, and what the theory says of its
   candidates before any search.
10. [Solution semantics: the region R](kb-evolution/solution-semantics.md): the answer, defined.
11. [Finding R: the oracle and the routes](kb-evolution/finding-r.md): every way R is computed, and why they
    agree.
12. [Monotone and non-monotone programs](kb-evolution/monotonicity.md): when the routes' licences hold, and what
    is claimed when they do not.
13. [Cores: what a contradiction rests on](kb-evolution/cores.md): a dead node's explanation.
14. [Satisfiability: what a program can say of its answer](kb-evolution/satisfiability.md): where a rule stops
    being a sentence about the world it fires in.

### The rule set as an object — [`rule-evolution/`](rule-evolution/)

15. [Rules as data](rule-evolution/rules-as-data.md): what of Ein already describes itself in Ein.
16. [Analysis of rules](rule-evolution/analysis-of-rules.md): the rule graph and its strata.
17. [Transformations of a rule set](rule-evolution/transformations.md): the open question, stated.
