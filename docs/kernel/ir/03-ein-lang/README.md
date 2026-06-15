# ein-lang — surface S-expression syntax

The textual IR that users author and the engine dumps. A small
S-expression dialect — a flat sequence of forms classified by head
(P1.7c); the surface representation of the graph from
[`../01-ein-graph/`](../01-ein-graph/), held in memory by the data
model in [`../02-data-model/`](../02-data-model/).

> **Source of truth for what parses**:
> [`src/ein/ir/grammar.lark`](../../../../src/ein/ir/grammar.lark).
> This tree explains intent, examples, and rendering — the grammar
> file is canonical.

## Files

- [`01_grammar.md`](01_grammar.md) — lexical rules (terminals,
  comments, naming convention) + the flat top-level form set: the
  declarators (`relation`, `rule`, `hrule`, `query`, `config`), the
  engine-emitted `trace`, and the else-is-a-fact default.
- [`02_patterns.md`](02_patterns.md) — the pattern sub-language used
  inside rule `:match` / `:assert` clauses; predicate registry;
  what's deliberately *not* in the pattern language.
- [`03_examples.md`](03_examples.md) — worked Zebra fragments; the
  classic and unified-is-a encodings side by side.
- [`04_dot_rendering.md`](04_dot_rendering.md) — IR ↔ DOT mapping;
  node-shape legend; Levi-bipartite hyperedge encoding; rule
  rendering modes; trace views; branch rendering.
- [`05_inspirations.md`](05_inspirations.md) — design influences
  (SMT-LIB, miniKanren, AtomSpace, Datalog, DPO graph rewriting) and
  where ein-lang diverges.
- [`06_reserved_names.md`](06_reserved_names.md) — the authoritative
  reserved surface-words reference: declarators, rule-body / ⊥
  primitives, predicates, `open`/`forall` sugar, `closed`,
  `hypothesis-relations`. (Engine-internal strings:
  [`../../inference/reserved_engine_strings.md`](../../inference/reserved_engine_strings.md).)

## Reading order

For a quick orientation: read `01_grammar.md`, then look at
`03_examples.md` to ground the syntax in working puzzles. The
pattern sub-language (`02_patterns.md`) is needed only when writing
rules. The DOT rendering chapter (`04_dot_rendering.md`) and the
inspirations chapter (`05_inspirations.md`) are reference material.

## Cross-cutting questions resolved here

- [Q3](../../../../plans/open_questions.md#q3--surface-ir-syntax) —
  homoiconic S-expressions over a heavier DSL.
- [Q4](../../../../plans/m1_core_graph_reasoning/open_questions.md#q4) —
  pattern-rewrite DSL with `:match` / `:assert` clauses; no Python
  fallback for the M1 rule set.
- [Q17](../../../../plans/m1_core_graph_reasoning/open_questions.md#q17--spatial-relation-formalisation) —
  spatial relations stay IR-native (declarative `square-fwd` /
  `square-bwd` rules + property facts; no integer-arithmetic
  position lattice).
- [Q21](../../../../plans/m1_core_graph_reasoning/open_questions.md#q21) —
  IR ↔ DOT bidirectional, layout-free.

## Conventions

- `;` line comments, `#| block |#` non-nesting block comments
  (SMT-LIB compatible).
- Hyphenated lowercase for relations and rule names (`has-color`,
  `triangle-composition`); PascalCase or `Foo_N` for types and
  instances (`Person`, `House-1`).
- All ein code blocks tagged ```` ```lisp ```` for syntax
  highlighting.
- DOT examples tagged ```` ```dot ````.
