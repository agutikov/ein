# IR ↔ DOT rendering

Every kernel IR form has a **fixed DOT shape**. Only **graph
structure** is fixed by this schema; layout (positions, rank,
unspecified style choices) is free — `random_layout` is permitted.

Per [Q21](../../../../plans/m1_core_graph_reasoning/open_questions.md#q21),
render is mandatory (`ein_bot.ir.to_dot`,
[S1.1.4](../../../../plans/m1_core_graph_reasoning/p1.1_ir_language/s1.1.4_ir_to_dot.md));
reverse parse (`ein_bot.ir.from_dot`) is a P1.2 deliverable
alongside the typed-hypergraph data model.

This was [`docs/ir.md` §6](../../../ir.md) before the kernel-
documentation split. The conceptual content overlaps with the
**detailed (Levi-bipartite) view** in
[`../01-ein-graph/01_kb.md` §2.2](../01-ein-graph/01_kb.md) —
this document specifies the *concrete DOT encoding*.

---

## Node-shape legend

| IR element                | DOT shape                          |
|---------------------------|------------------------------------|
| `type` declaration        | `box`                              |
| `instance` declaration    | `oval` (ellipse)                   |
| ground atom               | `rectangle`                        |
| hyperedge (Levi-bipartite) | `octagon`                          |
| equality class            | `doublecircle`                     |
| pattern variable `?x`     | `diamond`                          |
| wildcard `_`              | `diamond` with `style=dashed`      |
| relation schema           | dashed labelled edge               |
| derived edge (fact)       | solid labelled edge                |
| hypothetical edge         | solid edge with `style=dashed`     |

## Hyperedge encoding — Levi-bipartite

DOT has no native hyperedges. Every n-ary relation fact `(name a b c)`
is encoded **Levi-bipartite**: one `octagon` node for the hyperedge
itself, with directed edges to each participant labelled by role index
(or role name when declared). The hyperedge's node identity is what
[Q18](../../../../plans/m1_core_graph_reasoning/open_questions.md#q18)
provenance tuples reference; this anchors
[Q1](../../../../plans/open_questions.md#q1--what-kind-of-graph-is-the-ir)'s
typed-hypergraph + equality-class-ID answer visually.

The Levi-bipartite scheme is the *canonical* form — see
[`../01-ein-graph/01_kb.md` §2.2](../01-ein-graph/01_kb.md). For
binary facts, the compact rendering (collapsed labelled arrow) is
also permitted; see
[`../01-ein-graph/01_kb.md` §2.1](../01-ein-graph/01_kb.md).

## Ontology — UML-ish

```dot
digraph ontology {
  Person     [shape=box];
  House      [shape=box];
  Norwegian  [shape=oval];
  House_1    [shape=oval];
  Norwegian -> Person [style=dashed, arrowhead=empty];   // instance-of
  House_1   -> House  [style=dashed, arrowhead=empty];
  Person -> House [label="lives-in (1..1)", style=dashed]; // relation schema
}
```

## Rule rendering — three modes, configurable

Default: **(a)** for `rules.ein` documentation, **(c)** for trace
output. **(b)** is opt-in.

**(a) Side-by-side LHS | RHS** — explicit; readable for rule libraries.

```dot
digraph rule_triangle_lhs_rhs {
  subgraph cluster_lhs { label="match";
    a1 -> b1 [label="?r"]; b1 -> c1 [label="?r"]; }
  subgraph cluster_rhs { label="assert";
    a2 -> c2 [label="?r"]; }
}
```

**(b) DPO span `L ← K → R`** — categorical reading
([idea 07](../../../ideas/07-categorical-formulation.md)). Three
sub-clusters share the interface graph K (the bindings preserved by
the rule); the left morphism deletes nothing for our pattern
language (positive conjunctive), the right morphism adds the RHS.

**(c) Overlay** — most compact; LHS in solid, RHS additions in dashed.
Default at rule-firing time inside traces:

```dot
digraph rule_triangle_overlay {
  a -> b [label="?r"];               // LHS
  b -> c [label="?r"];               // LHS
  a -> c [label="?r", style=dashed]; // RHS addition
}
```

## Trace rendering — three views, configurable

Default: **(a)** — matches
[M1 acceptance §2](../../../../plans/m1_core_graph_reasoning/README.md)'s
`zebra/` snapshot folder.

**(a) Per-step DOT** — one file per step under
`<trace-name>/sNN.dot`; each shows working memory immediately after
the step's `:derives` is committed.

**(b) Aggregate** — single file, final state, edges coloured by step
number (early = blue, late = red). For overviews and paper figures.

**(c) Derivation DAG** — nodes are derived facts (one per `:derives`),
edges connect each derived fact to its `:using` premises. The natural
"explanation graph" view per
[idea 08](../../../ideas/08-human-style-deductive-trace.md). The
DAG is also what
[`../02-data-model/02_store.md` §provenance](../02-data-model/02_store.md)'s
`kb.derivation_dag(fact).to_dot()` produces directly:

```dot
digraph derivation {
  c10 [shape=rectangle, label="condition (10)"];
  s1  [shape=rectangle, label="lives-in(Norwegian, House_1)"];
  s2  [shape=rectangle, label="¬lives-in(Norwegian, House_2)"];
  c10 -> s1 [label="from-condition"];
  s1  -> s2 [label="exclusivity"];
}
```

## Branch rendering

- **Search-tree view** (P1.5 forward-reference): nodes are states
  bracketed by `branch-open` / `branch-close`; edges are `:choices`.
  Default for the `--search-tree` flag.
- **Per-state snapshots**: each branch becomes a `cluster_branch_<id>`
  sub-graph inside the working-memory DOT. Maps onto
  [`../02-data-model/02_store.md`](../02-data-model/02_store.md)'s
  `kb.fork()` semantics — each fork gets a distinct DOT cluster.

## Unified KB view (S1.2.4)

When the renderer has the full `KnowledgeBase` (not just a
single-form AST), it produces a **unified graph** where node identity
is fused across forms — `Norwegian` (instance) appears once and
participates in its is-a edge, its co-located facts, and any derived
edges. See
[S1.2.4](../../../../plans/m1_core_graph_reasoning/p1.2_typed_hypergraph/s1.2.4_graph_representation.md)
for the design.

## Reverse parse (`from_dot`)

Required by Q21 but not blocking on P1.1. The schema fixed by this
chapter is the contract `from_dot` will follow when implemented in
P1.2. Generic DOT files outside this schema are NOT round-trippable;
the API will reject non-conforming inputs rather than guess.

## See also

- [`../01-ein-graph/01_kb.md` §2](../01-ein-graph/01_kb.md) —
  conceptual compact vs detailed views; this document gives the
  concrete DOT encoding.
- [`../02-data-model/02_store.md`](../02-data-model/02_store.md) —
  `DerivationDAG.to_dot()` and (S1.2.4) `KnowledgeBase.to_dot()`.
- Grammar: [`src/ein_bot/ir/grammar.lark`](../../../../src/ein_bot/ir/grammar.lark).
