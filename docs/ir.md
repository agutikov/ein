# ein-bot IR — kernel specification

> **This file is a redirect.** The IR documentation has been
> reorganised into a kernel-documentation tree under
> [`docs/kernel/`](kernel/) — graph semantics first, data model
> second, surface language third, inference engine fourth. The new
> layout has the same content plus expanded *graph-first* chapters
> that didn't exist in the flat `ir.md`.

## New layout

| was `ir.md` §                          | now lives in                                                   |
|----------------------------------------|----------------------------------------------------------------|
| **§1 Lexical rules**                   | [`kernel/ir/03-ein-lang/01_grammar.md`](kernel/ir/03-ein-lang/01_grammar.md) |
| **§2 Top-level forms**                 | [`kernel/ir/03-ein-lang/01_grammar.md`](kernel/ir/03-ein-lang/01_grammar.md) |
| **§3 Pattern sub-language**            | [`kernel/ir/03-ein-lang/02_patterns.md`](kernel/ir/03-ein-lang/02_patterns.md) |
| **§4 Inspirations + diversions**       | [`kernel/ir/03-ein-lang/05_inspirations.md`](kernel/ir/03-ein-lang/05_inspirations.md) |
| **§5 Examples**                        | [`kernel/ir/03-ein-lang/03_examples.md`](kernel/ir/03-ein-lang/03_examples.md) |
| **§6 Rendering — IR ↔ DOT**            | [`kernel/ir/03-ein-lang/04_dot_rendering.md`](kernel/ir/03-ein-lang/04_dot_rendering.md) |
| § §6.5 Unified KB graph (planned)      | [`kernel/ir/03-ein-lang/04_dot_rendering.md`](kernel/ir/03-ein-lang/04_dot_rendering.md) + [S1.2.4](../plans/m1_core_graph_reasoning/p1.2_typed_hypergraph/s1.2.4_graph_representation.md) |
| §Provenance (deferred from ir.md)      | [`kernel/ir/02-data-model/02_store.md`](kernel/ir/02-data-model/02_store.md) §provenance |

## New material — graph-first

The reorganisation introduces two *new* chapters that the flat
`ir.md` lacked. These describe ein-bot in **pure graph terms**, no
syntax, no Python — the conceptual core the rest of the kernel
documentation builds on:

- [`kernel/ir/01-ein-graph/01_kb.md`](kernel/ir/01-ein-graph/01_kb.md)
  — Knowledge base. Five node kinds (object, type, relation, rule,
  fact); two equivalent views (compact + Levi-bipartite); three
  layers (ontology / fact / reasoning); provenance, fork, equality
  classes, open-class node kinds.
- [`kernel/ir/01-ein-graph/02_rules.md`](kernel/ir/01-ein-graph/02_rules.md)
  — Graph rewriting rules. Three rule types (T1 first-order, T2
  relation-polymorphic, T3 structural / aggregate); negative
  conclusions; rule rendering modes; saturation.

…plus the Python data-model chapter:

- [`kernel/ir/02-data-model/`](kernel/ir/02-data-model/) — How the
  graph is held in memory. Frozen dataclasses, identity rules,
  the KB store with reverse indexes, layer views, fork semantics,
  encoding-agnostic helpers, provenance and derivation DAG.

…and an inference stub:

- [`kernel/inference/`](kernel/inference/) — Stub before P1.3;
  becomes the matcher + saturation loop + hypothesis branching +
  contradiction analysis + trace generation as those phases ship.

## Why this split

The flat `ir.md` led with surface syntax, but the **graph is the
canonical data model** ([feedback memory `graph-canonical`](../README.md);
see also [F4 Q34](../plans/followups/f4_cross_cutting.md)).
Documentation order now matches conceptual order:

1. **Graph semantics** — what the system reasons about.
2. **Data model** — how the graph is held in memory.
3. **Surface language** — how users author graphs.
4. **Inference engine** — how rules transform graphs.

## Cross-references that previously pointed here

References to `docs/ir.md §<N>` from plan files and code remain
valid as section pointers — they now resolve via this redirect. The
short-form references (`docs/ir.md §Rendering`, etc.) still apply;
they just live in `kernel/ir/03-ein-lang/04_dot_rendering.md` and
similar.

For new cross-references, prefer the direct kernel paths above.

## See also

- [`docs/kernel/README.md`](kernel/README.md) — top-level
  orientation for the kernel documentation tree.
- Grammar source of truth: [`src/ein_bot/ir/grammar.lark`](../src/ein_bot/ir/grammar.lark).
- Code source of truth for the KB: [`src/ein_bot/kb/`](../src/ein_bot/kb/).
