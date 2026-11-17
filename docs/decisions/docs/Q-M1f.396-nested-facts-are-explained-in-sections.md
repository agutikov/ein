# Q-M1f.396 — Nested facts are explained in sections of the knowledge-base and rules pages

What a fact inside a fact is, and how a rule moves a fact between the global context and a fact's, is stated in
sections of the two pages of `docs/ein/reasoning/01-static-ir/` and has no page of its own: on
[the knowledge base](../../ein/reasoning/01-static-ir/01-knowledge-base.md), § The global context, and the context
of a fact, and § A context gives a fact no truth; on
[the rules](../../ein/reasoning/01-static-ir/02-rewriting-rules.md), § Packing and unpacking. The pages of
`02-kb-evolution/` say in a sentence each what follows there: that a saturation holds a fact only where it was
given or concluded, what a candidate and a state hold, and what a core names.

The cases and the two rewritings are told on one small example, *Alice thinks that five is two plus two*, whose
innermost fact is never held; the page's own drawing of a fact about a fact stays.

**Why.** The representation is the graph's: one node for one list of arrows, held or not. A page of its own would
define again what the knowledge-base page defines, and a rule that packs or unpacks is an ordinary rewriting.

**Held by.** The two pages and their drawings; the notation table of
[the reasoning](../../ein/reasoning/README.md#notation), which has $\operatorname{ctx}_K$.
