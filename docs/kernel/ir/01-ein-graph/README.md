# ein-bot graph — semantic core

The graph the engine reasons over. **No language, no Python here** —
this tree describes ein-bot in graph-theory terms.

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

## Reading order

Read `01_kb.md` first — it establishes what *kind* of graph this is
and the canonical (Levi-bipartite) form of every assertion. Then
`02_rules.md` builds on that vocabulary to describe rewriting.

## Where this maps to code

The graph here is *purely conceptual*. The Python implementation is
in [`../02-data-model/`](../02-data-model/); the surface S-expression
syntax that authors graphs is in [`../03-ein-lang/`](../03-ein-lang/);
the engine that *applies* rules is in
[`../../inference/`](../../inference/).
