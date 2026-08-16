# `ein.kb` — the knowledge base

The in-memory graph the engine reasons over: a `KnowledgeBase` registry
of entities + reverse indexes, the entity dataclasses, and per-fact
provenance. Source: [`ein.py/src/ein/kb/`](../../ein.py/src/ein/kb/).

> **Audience: embedders.** This is the data you load and the data you read
> back. The abstract (language-agnostic) model is
> [`docs/kernel/ir/02-data-model/`](../kernel/ir/02-data-model/); the
> code-level internals are
> [`02-data-model/03_python_impl.md`](../kernel/ir/02-data-model/03_python_impl.md).

*Verified against commit `60c192b` (2026-06-16).*

## Construction

### `KnowledgeBase.from_file(path) -> KnowledgeBase`

Read a `.ein` file and build the KB, resolving `(import …)` forms
file-relative to the file's directory (and `std.*` against the stdlib).
**The right entry for real puzzles** — anything that imports the stdlib
(e.g. `zebra2.ein`) needs this or `from_ir(..., base_dir=…)`.

### `KnowledgeBase.from_ir(forms, *, base_dir=None) -> KnowledgeBase`

Build from already-parsed [`SForm`](ir.md)s. `base_dir` is the directory
file-relative imports resolve against; `None` resolves only `std.*`.

### `load(forms, *, base_dir=None) -> KnowledgeBase`

The module-level function the two classmethods delegate to
(`from ein.kb import load`). Equivalent to `from_ir`.

### `KBLoadError`

Raised at the end of `load` with the accumulated malformed-IR problems.

```python
from ein.kb.store import KnowledgeBase
kb = KnowledgeBase.from_file("examples/zebra2.ein")
```

## Reading a `KnowledgeBase`

The attributes an embedder reads (all populated by the loader):

| attribute | type | what it holds |
|-----------|------|---------------|
| `facts` | `list[Fact]` | every fact across all three layers. |
| `relations` | `dict[str, Relation]` | declared relations by name. |
| `rules` | `dict[str, Rule]` | rules the saturator fires. |
| `hrules` | `dict[str, Rule]` | hypothesis rules (hypgen only — never fired). |
| `query` | `Query \| None` | the `(query …)` block, if any. |
| `config` | `SolverConfig \| None` | the parsed `(config …)` block (see [`inference.md`](inference.md)). |
| `names` | `dict[str, NameRef]` | the global names index (`object` / `relation` / `rule`). |
| `classes` | `EqClasses` | equality union-find (an F4 placeholder in M1). |

To read the *answer* after solving, prefer
[`goal_bindings(kb)`](inference.md) over walking `facts` by hand.

### `justifications(fact) -> tuple[Provenance, ...]`

Every derivation the engine recorded for `fact`, **primary first**.
[Provenance](#provenance) is per *derivation*, not per fact: the fact is
an OR-node and each entry an AND-node over its `premises_raw`. A fact
with no recorded alternatives yields just its primary (an empty tuple if
it has none), so single-justification callers read it unchanged.

### `derivation_dag(fact, *, all_justifications=False) -> DerivationDAG`

`kb.derivation_dag(fact)` returns the transitive-closure derivation DAG
of a fact (over each fact's `provenance.premises_raw`). The substrate the
[trace renderer](trace.md) reads. `all_justifications=True` expands every
recorded derivation instead of the primary one, giving the AND/OR graph
whose conjunction structure lands in
[`DerivationDAG.and_nodes`](#derivationdag). The default stays
primary-only: the DAG is a *display* object, and one derivation per fact
is what a reader can follow.

### `unsat_core(conflicting, *, all_justifications=False) -> set[Fact]`

The **source frontier** of a set of conflicting facts: walk each one's
derivation closure and union the terminals the engine treats as *given*
(`source`-kind, `hypothesis`-kind, or un-provenanced). Primary
justification only by default, deliberately — unioning over alternative
derivations makes the core monotonically *larger*, the opposite of a
legible explanation, so `all_justifications=True` is a soundness envelope
(every explanation of these conflicts is a subset of it) rather than a
better answer. For a minimum-cardinality answer use
[`ein.inference.frontier.smallest_contradiction_frontier`](inference.md),
which *chooses* one justification per fact instead of unioning them
(searched across every recorded derivation, budgeted; **not** a
subset-minimal MUS); it is what a [`Contradiction`](inference.md) verdict
reports as its `unsat_core`.

> **Loader/engine-internal, not embedding surface:** `add_fact`,
> `add_and_index_fact`, `record_justification`, `rebuild_indexes`,
> `fork`, and the `_…`-prefixed reverse indexes (`_facts_by_relation`,
> `_negated_facts`, `_alt_justifications`, …). These are maintained by
> the loader and the engine; an embedder reads, never writes.

## Entity dataclasses

### `Fact`

A proposition over objects. Identity is `(relation_name, args)`.

| field | meaning |
|-------|---------|
| `relation_name` | `str` — the relation head. |
| `args` | tuple of args (bare strings/ints, or nested `Fact`s for hyperedges). |
| `layer` | a [`Layer`](#layer) — which population the fact belongs to. |
| `provenance` | a [`Provenance`](#provenance) — where the fact came from. |

```python
f = verdict.trace[-1].derived[0]
f.relation_name, f.args, f.layer.name, f.provenance.kind
# 'pet-loc', ('Fox', 'House-1'), 'REASONING', 'rule'
```

### `Relation`, `Rule`

Frozen dataclasses identified by `name`. They expose cross-reference
`@property` accessors (`Relation.rules`, `Rule.relations`, …) that
delegate to the owning KB. Read them via `kb.relations[name]` /
`kb.rules[name]`.

### `Layer`

`Enum` with three members — the populations the KB stratifies facts into:

- `ONTOLOGY` — implicit assumptions (schema, instance enumeration,
  structural facts).
- `FACT` — explicit problem statements (the puzzle's numbered conditions,
  each `:source "(N)"`-annotated).
- `REASONING` — facts the engine derives at runtime (firings, hypotheses).

### `Pattern`, `Query`, `FactView`, `EqClasses`

Also exported from `ein.kb`: `Pattern` (a structural view of a `:match` /
`:assert` clause), `Query` (a `(query …)` block — read its goal via
`ein.inference.verdict.query_value`), `FactView`, `EqClasses`.

## Provenance

### `Provenance`

The per-derivation origin record (`from ein.kb import Provenance`):

| field | meaning |
|-------|---------|
| `kind` | `'source'` / `'rule'` / `'hypothesis'` / `'rejected'`. |
| `source` | for `source`-kind: the sentence id (`"(3)"`) or `None`. |
| `rule` | for `rule`-kind: the firing rule's name. |
| `premises_raw` | tuple of `(relation_name, args)` fact-ids the rule consumed. |
| `bindings` | the `var → name` binding used by the firing. |
| `branch` | for `hypothesis`/`rejected`-kind: the branch id. |

Convenience constructors: `Provenance.from_source`, `.from_rule`,
`.from_hypothesis`, `.rejected` (used by the loader/engine).

Provenance is per **derivation**, not per fact: `Fact.provenance` is the
primary justification and
[`kb.justifications(fact)`](#reading-a-knowledgebase) returns every
recorded one, so a fact is an OR-node over AND-nodes — the proof
structure is an AND/OR graph. Re-derivations of an already-known fact are
appended as alternatives (capped per fact, the shortest kept); terminals
take none — a `source`/`hypothesis` primary *is* the frontier (a given
stays given), and a `rule`-kind primary with empty `premises_raw` is a
synthetic engine writeback whose contract is that provenance walks ground
out on it. Recording is gated by
`SolverConfig.record_alternative_justifications` (default `True` — see
[`inference.md`](inference.md)).

### `DerivationDAG`

The transitive-closure derivation graph of a fact — built by
`kb.derivation_dag(fact)` over the `premises_raw` chains. Read by the
trace generator to produce its "it follows that …" narrative.

| field / property | meaning |
|------------------|---------|
| `root` | `Fact` — the fact the graph explains. |
| `nodes` | `tuple[Fact, …]` — every fact reached (cycles broken at re-visit). |
| `edges` | `tuple[(premise, conclusion), …]` — the flat premise→conclusion edges. |
| `and_nodes` | `tuple[(conclusion, premises), …]` — one entry per *justification*: the conjunction structure a flat edge set cannot express. |
| `is_or_graph` | `bool` — some fact here has more than one recorded derivation. |
| `sources` | `tuple[Fact, …]` — the frontier terminals (`source` / `hypothesis` / un-provenanced). |
| `to_dot()` | Graphviz `digraph` string; an OR graph draws one small diamond per justification, so alternative derivations read as alternatives instead of as one big conjunction. |

## See also

- [`ein.md`](ein.md) — the end-to-end flow.
- [`inference.md`](inference.md) — `solve` / verdicts / `goal_bindings`.
- [`docs/kernel/ir/02-data-model/`](../kernel/ir/02-data-model/) — the
  abstract model (`01_entities`, `02_store`) + the python-impl map.
