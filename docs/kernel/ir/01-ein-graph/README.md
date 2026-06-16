# Ein graph — semantic core

The graph the engine reasons over. **No language, no Python here** —
this tree describes Ein in graph-theory terms.

## Files

- [`01_kb.md`](01_kb.md) — Knowledge base. The five node kinds
  (object, type, relation, rule, fact); the two equivalent views
  (compact + Levi-bipartite); the three layers (ontology / fact /
  reasoning); provenance, hypothesis branches, equality classes,
  open-class node kinds.
- [`02_rules.md`](02_rules.md) — Graph rewriting rules. Anatomy of a
  rule (LHS / `:where` / RHS); the three rule types (T1 first-order,
  T2 relation-polymorphic, T3 structural / aggregate); negative
  conclusions; rule rendering modes; saturation.
- [`03_ein_model.md`](03_ein_model.md) — The reflexive ein algebra.
  Instance-of-instance fixed point; the five foundational terms
  (atom / node / arrow / object / relation); two flavours of node;
  disambiguating "relation"; types as common-relation holders;
  reserved relation names; open design seams (empty `()`, declaration
  body form).
- [`04_jack_drinks_coffee.md`](04_jack_drinks_coffee.md) — A small
  worked example illustrating the type-as-relation-holder pattern.
  Stated four ways: natural language, ein-lang, compact graph,
  detailed Levi-bipartite graph.
- [`05_four_level_kb.md`](05_four_level_kb.md) — The four-level KB
  (L0 objects / L1 facts / L2 relations / L3 rules) and the
  *consumes* stack; how the levels differ from the three layers.

## Reading order

Read `01_kb.md` first — it establishes what *kind* of graph this is
and the canonical (Levi-bipartite) form of every assertion. Then
`02_rules.md` builds on that vocabulary to describe rewriting.
`03_ein_model.md` extends `01_kb.md` with the reflexive observations
(instance-of-instance, types-as-relation-holders) — read it when the
self-referential framing matters (e.g., before reading any of the
self-modification followups F2 / F5 / F6).
`04_jack_drinks_coffee.md` is a teaching device; read it whenever
the type-inheritance pattern is unfamiliar.

## Where this maps to code

The graph here is *purely conceptual*. The Python implementation is
in [`../02-data-model/`](../02-data-model/); the surface S-expression
syntax that authors graphs is in [`../03-ein-lang/`](../03-ein-lang/);
the engine that *applies* rules is in
[`../../inference/`](../../inference/).
