# Q-M1f.420 — `ein render program` draws the program's knowledge base, its rules and its query

`ein render kb` draws the whole knowledge base as one graph, leaving nothing out: a stored negation is a fact of `not`
whose arrow `2` is at the fact it denies, and a fact that applies a rule an octagon whose arrow `1` is at the rule's
name. `ein render program` draws the program as written in
three parts: the digraph `kb`, every fact and declaration the program states, drawn by the KB view's code as one
graph; a digraph per rule, hrule and macro, as `ein render rule` draws one; and the digraph `query`, each query's
`:goal` drawn as a rule's match is, its other clauses in a note beside it, a claim written there as it is written,
and the `(solve …)` clause in a note. A
written `(trace …)` is drawn after them. The knowledge base is not split into an ontology and facts: a
declaration is the fact the loader stores, drawn as any fact. It is drawn from the parsed forms, so imports are not
resolved and a macro's call stands where it is written; a written check, `(eq …)` or `(neq …)`, is no fact and is
not drawn, and an `(import …)` or a `(config …)` draws nothing. A program with no rule, no import and no macro has a `kb` digraph equal to what `ein render kb` draws of
it, but for the fact the loader adds beside each declaration with labels.

**Why.** A program under review is read as what it states, what it derives by and what it asks. A split into
ontology and facts is how a program is written, while it is synthesised, and not what it is: drawn split, a fact is
drawn once in each part that holds it, and a fact one part points at is drawn apart from the part that states it.

**Held by.** `ein-render/tests/every_object_once.rs`: `the_program_views_kb_is_the_kb_views_drawing` and its sweep
over the corpus, `the_program_views_kb_is_the_kb_views_drawing_on_the_corpus`, in both forms;
`ein-render/src/ir_dot.rs`. The page is [rendering](../../ein/user/04-reference/12-rendering.md).
